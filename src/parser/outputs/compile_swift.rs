use crate::parser::{get_commands_and_compile_errors, Step};

use super::super::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Swift file compilation step
#[derive(Debug)]
pub struct CompileSwift {
    pub arch: String,
    pub description: Description,
    pub path: Option<PathBuf>,
    pub command: String,
}

#[async_trait]
impl ParsableFromStream for CompileSwift {
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();

        chunks.next();
        let arch = chunks
            .next()
            .ok_or_else(|| Error::EOF("CompileSwift".into(), "arch".into()))?
            .to_string();

        let path = match chunks.next() {
            Some(path) => path.to_string(),
            None => return Err(Error::EOF("CompileSwift".into(), "path".into())),
        };

        let description = Description::from_line(line)?;

        let (command, errors) = get_commands_and_compile_errors(stream).await;
        steps.extend(errors);

        let mut result = vec![Step::CompileSwift(Self {
            arch,
            description,
            path: if path.eq("(in") {
                // TODO: Parse compile commands for CompileSwift that doesn't contains path
                None
            } else {
                Some(PathBuf::from(path))
            },
            command,
        })];
        result.extend(steps);
        result.pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;
    let steps = to_stream_test! {
        CompileSwift,
        r#"CompileSwift normal arm64 /path/to/file.swift (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift-frontend -frontend -c ...

    "#
    };

    if let Step::CompileSwift(step) = steps.first().unwrap() {
        assert_eq!("arm64", step.arch);
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(Some(PathBuf::from("/path/to/file.swift")), step.path);
        assert_eq!(
                "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift-frontend -frontend -c ...",
                &step.command
                );
    }
}

impl Display for CompileSwift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Compiling ", self.description,)?;
        if let Some(path) = &self.path {
            write!(f, "`{}`", path.file_name().unwrap().to_str().unwrap())?;
        } else {
            write!(f, "`Swift Files`")?;
        }
        Ok(())
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn fmt() {
    let data = CompileSwift {
        arch: "x86".into(),
        description: Description {
            project: "DAB".into(),
            target: "iOS".into(),
        },
        path: Some(PathBuf::from("/path/to/file.swift")),
        command: "".into(),
    };

    assert_eq!("[DAB.iOS] Compiling `file.swift`", &format!("{}", data),);
}
