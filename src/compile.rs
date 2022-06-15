//! Compiled commands and completion database generated from xcodebuild logs;

mod command;
mod flags;
#[cfg(test)]
mod tests;
mod util;

use crate::parser::XCLOG_MATCHER;
use anyhow::{Context, Result};
use process_stream::{Process, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tap::Pipe;

pub use command::XCCompileCommand;
pub use flags::XCCompileArgs;

/// A clang-compatible compilation database
///
/// See <https://clang.llvm.org/docs/JSONCompilationDatabase.html>
#[derive(Debug, Deserialize, Serialize, derive_deref_rs::Deref, PartialEq, Eq)]
pub struct XCCompilationDatabase(Vec<XCCompileCommand>);

impl IntoIterator for XCCompilationDatabase {
    type Item = XCCompileCommand;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
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
            .pipe(Self)
            .pipe(Ok)
    }

    /// Generate XCCompilationDatabase from a vector build log lines
    ///
    /// Note root is set to default
    pub fn try_from_lines(lines: Vec<String>) -> Self {
        lines
            .iter()
            .filter_map(|line| {
                XCLOG_MATCHER
                    .get_compile_command(line)
                    .and_then(XCCompileCommand::from_compile_command_data)
            })
            .collect::<Vec<_>>()
            .pipe(Self)
    }

    /// Get file compile arguments for all projects files
    pub fn get_files_compile_args(&self) -> HashMap<PathBuf, XCCompileArgs> {
        self.iter()
            .flat_map(XCCompileCommand::compile_flags)
            .flatten()
            .collect::<HashMap<_, _>>()
    }

    /// Write completion database to a file
    pub async fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        serde_json::to_vec_pretty(&self)?
            .pipe_ref(|json| tokio::fs::write(path, json))
            .await
            .context("Unable to write XCCompilationDatabase from the given path.")
    }

    /// Read completion database from file asynchronously
    pub async fn try_from_filepath_async<P: AsRef<Path>>(path: P) -> Result<Self> {
        tokio::fs::read_to_string(path)
            .await?
            .pipe_ref(|s| serde_json::from_str::<Self>(s))?
            .pipe(Ok)
    }

    /// Read completion database from file synchronously
    pub fn try_from_filepath<P: AsRef<Path>>(path: P) -> Result<Self> {
        std::fs::read_to_string(path)?
            .pipe_ref(|s| serde_json::from_str::<Self>(s))?
            .pipe(Ok)
    }
}
