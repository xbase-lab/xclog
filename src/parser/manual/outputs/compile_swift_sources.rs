use crate::parser::{consume_errors, Step};

use super::super::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use process_stream::ProcessItem;
use std::{fmt::Display, path::PathBuf};
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
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
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

        while let Some(s) = stream.next().await {
            if let ProcessItem::Output(line) = s {
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
                if line.contains("error:") {
                    steps.extend(consume_errors(line, stream).await);
                }

                if line.starts_with("/") {
                    cmd = line.to_string().into();
                }
            }
        }

        let mut results = vec![Step::CompileSwiftSources(Self {
            arch,
            compiler,
            root: root.ok_or_else(|| Error::Failure("root not found".into()))?,
            description,
            command: cmd.ok_or_else(|| {
                Error::Failure("command for CompileSwiftSources not found".into())
            })?,
        })];
        results.extend(steps);
        results.pipe(Ok)
    }
}

impl Display for CompileSwiftSources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Compiling `Swift Sources`", self.description,)
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        CompileSwiftSources,
        r#"CompileSwiftSources normal arm64 com.apple.xcode.tools.swift.compiler (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    export DEVELOPER_DIR\=/Applications/Xcode.app/Contents/Developer
    export SDKROOT\=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc ...
remark: Incremental compilation has been disabled: it is not compatible with whole module optimization

    "#
    };

    if let Step::CompileSwiftSources(step) = steps.first().unwrap() {
        assert_eq!("arm64", step.arch);
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!("com.apple.xcode.tools.swift.compiler", &step.compiler);
        assert_eq!(PathBuf::from("/path/to/project"), step.root);
        assert_eq!(
        "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swiftc ...",
        &step.command
    );
    } else {
        panic!("{steps:#?}")
    }
}
