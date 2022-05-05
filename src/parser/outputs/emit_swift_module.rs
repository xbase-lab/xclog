use crate::{
    parser::{Description, Error, OutputStream, ParsableFromStream},
    runner::ProcessUpdate,
};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;
use tokio_stream::StreamExt;

/// Resource file was copied
#[derive(Debug)]
pub struct EmitSwiftModule {
    pub arch: String,
    pub description: Description,
    pub output_path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for EmitSwiftModule {
    async fn parse_from_stream(line: String, _stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        chunks.next();
        let mut output_path = None;
        let arch = chunks
            .next()
            .map(ToString::to_string)
            .ok_or_else(|| Error::EOF("EmitSwiftModule".into(), "arch".into()))?;

        while let Some(ProcessUpdate::Stdout(line)) = _stream.next().await {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            if line.starts_with("cd") {
                continue;
            }

            let mut chunks = line.split_whitespace();
            while let Some(chunk) = chunks.next() {
                if chunk.eq("-o") {
                    output_path = chunks.next().map(PathBuf::from);
                    break;
                }
            }
        }

        Self {
            arch,
            output_path: output_path
                .ok_or_else(|| Error::EOF("EmitSwiftModule".into(), "arch".into()))?,
            description: Description::from_line(line)?,
        }
        .pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        EmitSwiftModule,
       r#"EmitSwiftModule normal arm64 (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    $TOOLCHAIN_BIN/swift-frontend -frontend -emit-module -experimental-skip-non-inlinable-function-bodies-without-types -emit-module-source-info-path $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.swiftsourceinfo -emit-objc-header-path $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget-Swift.h -o $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.swiftmodule -emit-abi-descriptor-path $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.abi.json

"# 
    };

    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.swiftmodule"),
        step.output_path
    );
}
