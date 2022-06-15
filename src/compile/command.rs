use super::{util, XCCompileArgs};
use crate::parser::XCCompileCommandData;
use anyhow::Result;
use lazy_regex::regex_captures as cap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
        tracing::info!("{}", data.name);
        if !(data.name.contains("swiftc") || data.name.contains("clang")) {
            return None;
        }
        let is_clang = data.name.contains("clang");
        let args = &data.arguments;
        let mut command = Self::default();

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

        command.index_store_path =
            cap!(r"-index-store-path\s(/[^\s]+)", args).map(|(_, c)| c.into());
        command.directory = cap!(r"-working-directory\s(.*)\s*", args)
            .map(|(_, c)| c.to_string())
            .unwrap_or_else(|| "/".to_string());

        // NOTE: command.files and command.output are skipped

        Some(command)
    }

    /// Generate a map of filespaths in workspaces and their compilation flags
    pub fn compile_flags<'a>(&'a self) -> Result<HashMap<PathBuf, XCCompileArgs>> {
        let mut info = HashMap::default();
        let flags = XCCompileArgs::try_from_str(&self.command)?;

        // Swift File Lists
        self.file_lists.iter().for_each(|path| {
            match util::get_files_list(&path) {
                Ok(file_list) => {
                    file_list.into_iter().for_each(|file_path: PathBuf| {
                        info.insert(file_path, flags.clone());
                    });
                }
                Err(e) => tracing::error!("Fail to get file lists {e}"),
            };
        });

        // Swift Module Files
        self.files.as_ref().map(|files| {
            files.iter().for_each(|file| {
                info.insert(file.clone(), flags.clone());
            })
        });

        // Single File Command
        self.file
            .as_ref()
            .map(|file| info.insert(file.clone(), flags.clone()));

        Ok(info)
    }
}
