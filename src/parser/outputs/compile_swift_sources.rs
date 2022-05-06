use std::{fmt::Display, path::PathBuf};

use crate::runner::ProcessUpdate;

use super::super::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use tap::Pipe;
use tokio_stream::StreamExt;

#[derive(Debug)]
/// Aggregated swift files compilation
pub struct CompileSwiftSources {
    pub compiler: String,
    pub arch: String,
    pub root: PathBuf,
    pub description: Description,
    pub command: String,
}

#[async_trait]
impl ParsableFromStream for CompileSwiftSources {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        chunks.next();

        let arch = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("CompileSwiftSources".into(), "arch".into()))?;

        let compiler = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("CompileSwiftSources".into(), "compiler".into()))?;

        let description = Description::from_line(line)?;
        let (mut cmd, mut root) = (None, None);

        while let Some(ProcessUpdate::Stdout(line)) = stream.next().await {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            if line.starts_with("cd") {
                root = line.strip_prefix("cd ").map(PathBuf::from);
            }

            if line.starts_with("export") {
                continue;
            }

            if line.starts_with("/") {
                cmd = line.to_string().into();
            }
        }

        Self {
            arch,
            compiler,
            root: root.ok_or_else(|| Error::Failure("root not found".into()))?,
            description,
            command: cmd.ok_or_else(|| {
                Error::Failure("command for CompileSwiftSources not found".into())
            })?,
        }
        .pipe(Ok)
    }
}

impl Display for CompileSwiftSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Compiling `Swift Sources`", self.description,)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        CompileSwiftSources,
        r#"CompileSwiftSources normal arm64 com.apple.xcode.tools.swift.compiler (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    export DEVELOPER_DIR\=/Applications/Xcode.app/Contents/Developer
    export SDKROOT\=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc ...
remark: Incremental compilation has been disabled: it is not compatible with whole module optimization

    "#
    };

    assert_eq!("arm64", step.arch);
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!("com.apple.xcode.tools.swift.compiler", &step.compiler);
    assert_eq!(PathBuf::from("/path/to/project"), step.root);
    assert_eq!(
        "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc ...",
        &step.command
    );
}
