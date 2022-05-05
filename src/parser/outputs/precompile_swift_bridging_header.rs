use crate::parser::util::consume_till_empty_line;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use crate::runner::ProcessUpdate;
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;
use tokio_stream::StreamExt;

#[derive(Debug)]
/// Precompile Bridging header
pub struct PrecompileSwiftBridgingHeader {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for PrecompileSwiftBridgingHeader {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut path = None;

        while let Some(ProcessUpdate::Stdout(line)) = stream.next().await {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            if line.starts_with("cd") || line.starts_with("export") {
                continue;
            }

            let mut chunks = line.split_whitespace();
            while let Some(chunk) = chunks.next() {
                if chunk.eq("-import-objc-header") {
                    if path.is_some() {
                        return Err(Error::Failure(format!(
                            "Trying to overwrite set path with {chunk}"
                        )));
                    }
                    path = chunks.next().map(PathBuf::from);
                }
            }
        }

        let description = Description::from_line(line)?;

        consume_till_empty_line(stream).await;

        Self {
            description,
            path: path
                .ok_or_else(|| Error::Failure("Fail to extract SwiftBridgingHeader path".into()))?,
        }
        .pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        PrecompileSwiftBridgingHeader,
       r#"PrecompileSwiftBridgingHeader normal arm64 (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    $TOOLCHAIN_BIN/swift-frontend -frontend -target arm64-apple-macos12.0 -Xllvm -aarch64-use-tbi -enable-objc-interop -stack-check -sdk /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -I $ROOT/build/Release -F $ROOT/build/Release/PackageFrameworks -F $ROOT/build/Release/PackageFrameworks -F $ROOT/build/Release/PackageFrameworks -F $ROOT/build/Release/PackageFrameworks -F $ROOT/build/Release -g -swift-version 5 -enforce-exclusivity\=checked -O -new-driver-path $TOOLCHAIN_BIN/swift-driver -serialize-debugging-options -Xcc -working-directory -Xcc $ROOT -resource-dir /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift -Xcc -iquote -Xcc -ivfsoverlay -Xcc $ROOT/build/YabaiMaster.build/Release/YabaiMaster.build/all-product-headers.yaml -Xcc -iquote-Xcc -I$ROOT/build/Release/include -Xcc -Isrc -Xcc -I$ROOT/build/YabaiMaster.build/Release/YabaiMaster.build/DerivedSources-normal/arm64 -Xcc -I$ROOT/build/YabaiMaster.build/Release/YabaiMaster.build/DerivedSources/arm64 -Xcc -I$ROOT/build/YabaiMaster.build/Release/YabaiMaster.build/DerivedSources -import-objc-header $ROOT/src/bridge.h -module-name YabaiMaster -target-sdk-version 12.3 -serialize-diagnostics-path $ROOT/build/SharedPrecompiledHeaders/bridge-dcrwnehznx5i.dia $ROOT/src/bridge.h -emit-pch -pch-output-dir $ROOT/build/SharedPrecompiledHeaders
"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(PathBuf::from("$ROOT/src/bridge.h"), step.path);
}
