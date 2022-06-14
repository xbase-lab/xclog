//! Compiled commands and completion database generated from xcodebuild logs;

use anyhow::{Context, Result};
use lazy_regex::regex_captures as cap;
use process_stream::{Process, StreamExt};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tap::Pipe;

use crate::parser::{XCCompileCommandData, XCLOG_MATCHER};

/// A clang-compatible compilation database
///
/// See <https://clang.llvm.org/docs/JSONCompilationDatabase.html>
#[derive(Debug, Deserialize, Serialize, derive_deref_rs::Deref, PartialEq, Eq)]
pub struct XCCompilationDatabase {
    root: PathBuf,
    #[deref]
    inner: Vec<XCCompileCommand>,
}

impl XCCompilationDatabase {
    /// Generate completion database from running xcodebuild arguments in a given root.
    pub async fn generate<P, I, S>(root: P, args: I) -> Result<Self>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<std::ffi::OsStr> + Send,
    {
        let mut process = Process::new("/usr/bin/xcodebuild");
        process.current_dir(&root);
        process.arg("clean");
        process.args(args);
        process
            .spawn_and_stream()?
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|o| {
                XCLOG_MATCHER
                    .get_compile_command(o.to_string().as_str())
                    .and_then(XCCompileCommand::from_compile_command_data)
            })
            .collect::<Vec<_>>()
            .pipe(|inner| Self {
                root: root.as_ref().to_path_buf(),
                inner,
            })
            .pipe(Ok)
    }

    /// Generate XCCompilationDatabase from a vector build log lines
    ///
    /// Note root is set to default
    pub fn from_lines(lines: Vec<String>) -> Self {
        lines
            .iter()
            .filter_map(|line| {
                XCLOG_MATCHER
                    .get_compile_command(line)
                    .and_then(XCCompileCommand::from_compile_command_data)
            })
            .collect::<Vec<_>>()
            .pipe(|inner| Self {
                root: Default::default(),
                inner,
            })
    }

    /// Write completion database to a file
    pub async fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        serde_json::to_vec_pretty(&self)?
            .pipe_ref(|json| tokio::fs::write(path, json))
            .await
            .context("Unable to write XCCompilationDatabase from the given path.")
    }

    /// Read completion database from file asynchronously
    pub async fn from_filepath_async<P: AsRef<Path>>(path: P) -> Result<Self> {
        tokio::fs::read_to_string(path)
            .await?
            .pipe_ref(|s| serde_json::from_str::<Self>(s))?
            .pipe(Ok)
    }

    /// Read completion database from file synchronously
    pub fn from_filepath<P: AsRef<Path>>(path: P) -> Result<Self> {
        std::fs::read_to_string(path)?
            .pipe_ref(|s| serde_json::from_str::<Self>(s))?
            .pipe(Ok)
    }

    /// Set the xccompilation database's root.
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
    }

    /// Get a reference to the xccompilation database's root.
    #[must_use]
    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}

/// Single Compilation Database Command Representation
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct XCCompileCommand {
    /// Module name. NOTE: not sure if this required
    #[serde(rename(serialize = "module_name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The path of the main file for the compilation, which may be relative to `directory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,
    /// The working directory for the compilation
    pub directory: String,
    /// The compile command, this is alias with commandLine or split form of command
    pub command: String,
    /// Source code files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PathBuf>>,
    /// For SwiftFileList
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_lists: Vec<PathBuf>,
    /// The name of the build output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Index store path. Kept for the caller to further process.
    #[serde(skip)]
    pub index_store_path: Option<PathBuf>,
}

// TODO: Remove duplication and keep current architecture?
impl XCCompileCommand {
    /// Convert [`XCCompileCommandData`] to [`XCCompileCommand`].
    pub fn from_compile_command_data(data: XCCompileCommandData) -> Option<Self> {
        let is_clang = data.name.contains("clang");
        let args = &data.arguments;
        let mut command = Self::default();

        // TODO: join with args
        command.command = format!("{} {args}", data.command);

        // If clang and no file captured then return none
        // TODO: make sure this is always the case!
        if is_clang {
            if let Some(file) = cap!(r"-c\s(.*)-o", args).map(|(_, c)| c.trim()) {
                command.file = Some(file.into());
            } else {
                return None;
            }
        } else {
            // Module name is required for swiftc, return early?
            command.name = cap!(r"-module-name\s(\w+)\s", args).map(|(_, c)| c.into());

            if let Some(file_lists) = cap!(r"@(/.*.SwiftFileList)", args).map(|(_, c)| c) {
                command.file_lists = vec![file_lists.into()];
            };
        }

        if let Some(directory) = cap!(r"-working-directory\s(.*)\s*", args).map(|(_, c)| c) {
            command.directory = directory.into();
        } else {
            // NOTE: This may not work
            command.directory = "/".into();
        }

        if let Some(index_store_path) = cap!(r"-index-store-path\s(.*)\s", args).map(|(_, c)| c) {
            command.index_store_path = Some(index_store_path.into());
        };
        // NOTE: command.files and command.output are skipped

        Some(command)
    }
}

#[cfg(test)]
fn get_case_lines(content: &str) -> Vec<String> {
    content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
macro_rules! test_compile_commands_output {
    ($cmd:ident, $($idx:literal: $key:ident, $value:expr),*) => {
        $(
            assert_eq!($cmd[$idx].$key, $value);
        )*
    };
}

#[tokio::test]
#[tracing_test::traced_test]
async fn case_a() {
    let lines = get_case_lines(include_str!("../tests/case_a.log"));
    let compile_commands = XCCompilationDatabase::from_lines(lines);

    assert_eq!(compile_commands.len(), 3);
    test_compile_commands_output! { compile_commands,
        0: name, Some("Logging".to_string()),
        0: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-log"),
        0: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/armv7/Logging.SwiftFileList")],
        1: name, Some("Logging".to_string()),
        1: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-log"),
        1: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-log.build/Debug-iphoneos/Logging.build/Objects-normal/arm64/Logging.SwiftFileList")],
        2: name, Some("Example".to_string()),
        2: directory, String::from("/PROJECT_ROOT"),
        2: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug-iphoneos/Example.build/Objects-normal/arm64/Example.SwiftFileList")]
    };

    let file_path = PathBuf::from("/tmp/case_a_compile_commands.json");
    compile_commands.write(&file_path).await.unwrap();
    assert!(file_path.exists());
    let from_filepath = XCCompilationDatabase::from_filepath(&file_path).unwrap();
    assert_eq!(compile_commands.len(), from_filepath.len());
    std::fs::remove_file(file_path).unwrap();
}

#[tokio::test]
#[tracing_test::traced_test]
async fn case_b() {
    let lines = get_case_lines(include_str!("../tests/case_b.log"));
    let compile_commands = XCCompilationDatabase::from_lines(lines);

    assert_eq!(compile_commands.len(), 12);
    let file_path = PathBuf::from("/tmp/case_b_compile_commands.json");
    compile_commands.write(&file_path).await.unwrap();
    assert!(file_path.exists());
    let from_filepath = XCCompilationDatabase::from_filepath(&file_path).unwrap();
    assert_eq!(compile_commands.len(), from_filepath.len());
    std::fs::remove_file(file_path).unwrap();

    test_compile_commands_output! {compile_commands,
    0: name, Some("ArgumentParserToolInfo".to_string()),
    0: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-argument-parser"),
    0: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParserToolInfo.build/Objects-normal/x86_64/ArgumentParserToolInfo.SwiftFileList")],
    1: name, Some("ArgumentParserToolInfo".to_string()),
    1: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-argument-parser"),
    1: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParserToolInfo.build/Objects-normal/arm64/ArgumentParserToolInfo.SwiftFileList")],
    2: name, Some("ArgumentParser".to_string()),
    2: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-argument-parser"),
    2: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParser.build/Objects-normal/x86_64/ArgumentParser.SwiftFileList")],
    3: name, Some("ArgumentParser".to_string()),
    3: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/swift-argument-parser"),
    3: file_lists, vec![PathBuf::from("/BUILD_ROOT/swift-argument-parser.build/Debug/ArgumentParser.build/Objects-normal/arm64/ArgumentParser.SwiftFileList")],
    4: name, Some("Socket".to_string()),
    4: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/BlueSocket"),
    4: file_lists, vec![PathBuf::from("/BUILD_ROOT/Socket.build/Debug/Socket.build/Objects-normal/arm64/Socket.SwiftFileList")],
    5: name, Some("Socket".to_string()),
    5: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/BlueSocket"),
    5: file_lists, vec![PathBuf::from("/BUILD_ROOT/Socket.build/Debug/Socket.build/Objects-normal/x86_64/Socket.SwiftFileList")],
    6: name, Some("SwiftyBeaver".to_string()),
    6: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/SwiftyBeaver"),
    6: file_lists, vec![PathBuf::from("/BUILD_ROOT/SwiftyBeaver.build/Debug/SwiftyBeaver.build/Objects-normal/x86_64/SwiftyBeaver.SwiftFileList")],
    7: name, Some("SwiftyBeaver".to_string()),
    7: directory, String::from("/DERIVED_DATA_ROOT/SourcePackages/checkouts/SwiftyBeaver"),
    7: file_lists, vec![PathBuf::from("/BUILD_ROOT/SwiftyBeaver.build/Debug/SwiftyBeaver.build/Objects-normal/arm64/SwiftyBeaver.SwiftFileList")],
    8: name, Some("Example".to_string()),
    8: directory, String::from("/PROJECT_ROOT"),
    8: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug/Example.build/Objects-normal/x86_64/Example.SwiftFileList")],
    9: name, Some("Example".to_string()),
    9: directory, String::from("/PROJECT_ROOT"),
    9: file_lists, vec![PathBuf::from("/BUILD_ROOT/Example.build/Debug/Example.build/Objects-normal/arm64/Example.SwiftFileList")],
    10: name, None,
    10: directory, String::from("/"),
    10: file, Some(PathBuf::from("/PROJECT_ROOT/src/client/bridge.c")),
    11: name, None,
    11: directory, String::from("/"),
    11: file, Some(PathBuf::from("/PROJECT_ROOT/src/client/bridge.c"))
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn case_c() {
    let lines = get_case_lines(include_str!("../tests/case_c.log"));
    let compile_commands = XCCompilationDatabase::from_lines(lines);
    // THIS DOESN'T FEEL CORRECT
    assert_eq!(compile_commands.len(), 104)
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = "Local tests"]
async fn test_get_compile_commands() {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    let compile_commands = XCCompilationDatabase::generate(root, &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
    ]).await.unwrap();

    println!("{:#?}", compile_commands.len());
    for command in compile_commands.iter() {
        if let Some(ref command) = command.name {
            println!("{:?}", command);
        } else if let Some(ref file) = command.file {
            println!("{:?}", file);
        } else {
            println!("{:?}", command);
        }
    }
    // In the case above the compile commands is indeed 12
    assert_eq!(compile_commands.len(), 12);
}
