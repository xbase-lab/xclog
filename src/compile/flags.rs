use super::util;
use super::XCCompilationDatabase;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tap::Pipe;

/// File Compile Arguments
#[derive(Debug, Clone, Serialize, Deserialize, derive_deref_rs::Deref)]
pub struct XCCompileArgs(pub(crate) Vec<String>);

impl XCCompileArgs {
    /// Generate compile flags from [`XCCompileCommand`] command field
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// XCCompileArgs::try_from_str(&compile_command.command);
    /// ```
    ///
    /// [`XCCompileCommand`]: crate::XCCompileCommand
    pub fn try_from_str(command: &str) -> Result<Self> {
        let mut items = shell_words::split(command)?;
        items.remove(0);
        Ok(Self(util::inject_files_list_items(items)?))
    }

    /// This reads compile file into [`XCCompilationDatabase`] then generates file compile
    /// arguments and find the given file_path in that.
    ///
    /// Used as fallback in cases where only the filepath and compile_path is known.
    ///
    /// Please avoid using it directly.
    pub fn try_from_compile_path<P: AsRef<Path>>(
        filepath: P,
        compile_filepath: &PathBuf,
    ) -> Result<Self> {
        let filepath = filepath.as_ref();
        let compile_commands = XCCompilationDatabase::try_from_filepath(compile_filepath)?;
        let mut file_arguments = compile_commands.get_files_compile_args();

        file_arguments
            .remove(filepath)
            .ok_or_else(|| anyhow::anyhow!("No file path found with {filepath:?}"))
    }

    /// Try to get file compile arguments from given filepath.
    pub fn try_from_filepath<P: AsRef<Path>>(path: P) -> Result<Self> {
        let (root, swift_flags, compile_filepath) = util::find_swift_module_root(path.as_ref());
        match root {
            Some(root) => {
                if let Some(compile_filepath) = compile_filepath {
                    Self::try_from_compile_path(path, &compile_filepath)
                } else {
                    util::generate_compile_args_from_root(root, swift_flags)
                }
            }
            None => {
                return path
                    .as_ref()
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Unable to convert filepath to path"))?
                    .pipe(|f| vec![f.into(), "-sdk".into(), util::SDKPATH.into()])
                    .pipe(Self)
                    .pipe(Ok);
            }
        }
    }
}
