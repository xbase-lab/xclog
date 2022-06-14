use crate::compile::XCCompileArgs;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tap::Pipe;

pub const SDKPATH: &str = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/";

/// Get all files in SwiftFileList file.
pub(crate) fn get_files_list<T, P>(file_lists: P) -> Result<Vec<T>>
where
    T: From<String>,
    P: AsRef<Path>,
{
    std::fs::read_to_string(file_lists)?
        .pipe(|s| shell_words::split(&s))?
        .into_iter()
        .map(T::from)
        .collect::<Vec<_>>()
        .pipe(Result::Ok)
}

/// Find Header directory and frameworks from path.
pub(crate) fn find_header_dirs(root: &Path) -> Result<(Vec<String>, Vec<String>)> {
    wax::walk("**/*.h", root)?
        .flatten()
        .enumerate()
        .map(|(i, entry)| {
            entry
                .path()
                .ancestors()
                .find(|p| p.extension().eq(&Some("framework".as_ref())))
                .pipe(|p| {
                    if let Some(path) = p {
                        let framework = path.file_name()?.to_str()?.to_string();
                        tracing::trace!("Framework {i}: {framework}");
                        Some((framework.into(), None))
                    } else {
                        let dir = entry.path().parent()?.file_name()?.to_str()?.to_string();
                        tracing::trace!("Directory {i}: {dir}");
                        Some((None, dir.into()))
                    }
                })
        })
        .flatten()
        .unzip()
        .pipe(|(dirs, frameworks): (Vec<_>, Vec<_>)| {
            let dirs = dirs.into_iter().flatten().collect();
            let frameworks = frameworks.into_iter().flatten().collect();
            Ok((dirs, frameworks))
        })
}

pub fn find_swift_files(project_root: &Path) -> Result<Vec<String>> {
    wax::walk("**/*.swift", project_root)?
        .enumerate()
        .map(|(i, entry)| {
            entry.ok()?.path().to_str()?.to_string().pipe(|path| {
                tracing::trace!("{i}: {path}");
                Some(path)
            })
        })
        .flatten()
        .collect::<Vec<_>>()
        .pipe(Ok)
}

/// Find directory, swiftflags and comple file from a path to file within a project.
pub(crate) fn find_swift_module_root(
    file_path: &Path,
) -> (Option<PathBuf>, Option<PathBuf>, Option<PathBuf>) {
    let mut compile_file = None;
    let mut directory = match file_path.parent() {
        Some(directory) => directory,
        None => return (None, None, None),
    };

    while directory.components().count() > 1 {
        let path = match directory.parent() {
            Some(path) => path,
            None => break,
        };

        let flag_path = path.join(".swiftflags");
        if flag_path.is_file() {
            return (Some(directory.to_path_buf()), Some(flag_path), compile_file);
        };

        if compile_file.is_none() {
            path.join(".compile")
                .pipe(|p| p.is_file().then(|| compile_file = p.into()));
        };

        if is_project_root(directory) {
            return (Some(directory.to_path_buf()), None, compile_file);
        } else {
            directory = path;
        }
    }

    (Some(directory.to_path_buf()), None, compile_file)
}

pub(crate) fn is_project_root(directory: &Path) -> bool {
    if directory.is_dir() {
        directory.join(".git").exists()
    } else {
        tracing::warn!("Not a directory");
        false
    }
}

/// Filter swift compilation arguments and inject files_list content to flags
pub(crate) fn inject_files_list_items(flags: Vec<String>) -> Result<Vec<String>> {
    let mut args = vec![];
    let mut items = flags.into_iter();

    while let Some(arg) = items.next() {
        // SourceKit don't support filelist, unfold it
        if arg == "-filelist" {
            if let Some(arg) = items.next() {
                arg.pipe(get_files_list)?
                    .pipe_as_mut(|paths| args.append(paths));
                continue;
            }
        }

        // swift 5.1 filelist, unfold it
        if arg.starts_with("@") {
            if let Some(arg) = arg.strip_prefix("@") {
                arg.pipe(get_files_list)?
                    .pipe_as_mut(|paths| args.append(paths));
                continue;
            }
        }

        args.push(arg)
    }

    Ok(args)
}

pub(crate) fn generate_compile_args_from_root(
    root: PathBuf,
    swiftflags_filepath: Option<PathBuf>,
) -> Result<XCCompileArgs> {
    let mut flags_collect = Vec::default();
    let (headers, frameworks) = find_header_dirs(&root)?;

    headers
        .into_iter()
        .flat_map(|header| vec!["-Xcc".into(), "-I".into(), header])
        .collect::<Vec<String>>()
        .pipe_ref_mut(|flags| flags_collect.append(flags));

    frameworks
        .into_iter()
        .map(|framework| format!("-F{framework}"))
        .collect::<Vec<String>>()
        .pipe_ref_mut(|flags| flags_collect.append(flags));

    find_swift_files(&root)?.pipe_ref_mut(|flags| flags_collect.append(flags));

    if let Some(ref swiftflags_filepath) = swiftflags_filepath {
        if let Some(ref mut additional_flags) = additional_flags(swiftflags_filepath) {
            flags_collect.append(additional_flags)
        }
    }
    return flags_collect.pipe(XCCompileArgs).pipe(Result::Ok);
}

/// Get Additional flags from an optional flags_path.
fn additional_flags(flags_path: &Path) -> Option<Vec<String>> {
    std::fs::read_to_string(flags_path)
        .ok()?
        .split("\n")
        .filter(|line| line.starts_with("#"))
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>()
        .into()
}
