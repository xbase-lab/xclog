//! Compiled commands and complation database generated from xcodebuild logs;

use anyhow::Result;
use lazy_regex::regex_captures as cap;
use process_stream::{Process, StreamExt};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tap::Pipe;

use crate::parser::CompileCommandData;

/// A clang-compatible compilation database
///
/// It depends on build logs generated from xcode
///
/// `xcodebuild clean build -verbose`
///
/// See <https://clang.llvm.org/docs/JSONCompilationDatabase.html>
#[derive(Debug, Deserialize, Serialize, derive_deref_rs::Deref)]
pub struct XCCompilationDatabase(pub(crate) Vec<XCCompileCommand>);

impl XCCompilationDatabase {
    /// Generate complation database from running xcodebuild arguments in a given root.
    pub async fn generate<P, I, S>(root: P, args: I) -> Result<Self>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<std::ffi::OsStr> + Send,
    {
        let mut process = Process::new("/usr/bin/xcodebuild");
        process.current_dir(root);
        process.arg("clean");
        process.args(args);

        process
            .spawn_and_stream()?
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|o| {
                crate::parser::MATCHER
                    .get_compile_command(o.to_string().as_str())
                    .map(XCCompileCommand::from_compile_command_data)
                    .flatten()
            })
            .flatten()
            .collect::<Vec<_>>()
            .pipe(|vec| XCCompilationDatabase(vec))
            .pipe(Ok)
    }
}

/// Single Compilation Database Command Representation
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
    /// Convert [`CompileCommandData`] to [`XCCompileCommand`].
    pub fn from_compile_command_data(data: CompileCommandData) -> Option<Self> {
        let is_clang = data.name == "clang";
        let ref args = data.arguments;
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
use crate::parser::MATCHER;

#[cfg(test)]
async fn test(lines: Vec<String>) {
    for line in lines {
        if let Some(command) = MATCHER.get_compile_command(&line) {
            XCCompileCommand::from_compile_command_data(command);
        }
    }
}

#[cfg(test)]
fn get_case_lines(content: &str) -> Vec<String> {
    content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = ".."]
async fn case_a() {
    let lines = get_case_lines(include_str!("../tests/case_a.log"));
    test(lines).await;
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = ".."]
async fn case_b() {
    let lines = get_case_lines(include_str!("../tests/case_b.log"));
    test(lines).await;
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = ".."]
async fn case_c() {
    let lines = get_case_lines(include_str!("../tests/case_b.log"));
    test(lines).await;
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
}
