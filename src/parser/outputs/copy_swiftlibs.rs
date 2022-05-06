use crate::parser::util::consume_till_empty_line;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::fmt::Display;
use std::path::PathBuf;
use tap::Pipe;

#[derive(Debug)]
/// Swift Runtime was copied
pub struct CopySwiftLibs {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for CopySwiftLibs {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CopySwiftLibs".into(), "path".into()))?;

        let description = Description::from_line(line)?;

        consume_till_empty_line(stream).await;

        Self { description, path }.pipe(Ok)
    }
}

impl Display for CopySwiftLibs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Copying     Swift Libraries", self.description,)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        CopySwiftLibs,
       r#"CopySwiftLibs $ROOT/build/Release/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    export CODESIGN_ALLOCATE\=$TOOLCHAIN_BIN/codesign_allocate
    export DEVELOPER_DIR\=/Applications/Xcode.app/Contents/Developer
    export SDKROOT\=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk
    builtin-swiftStdLibTool --copy --verbose --sign - --scan-executable $ROOT/build/Release/DemoTarget.app/Contents/MacOS/DemoTarget --scan-folder $ROOT/build/Release/DemoTarget.app/Contents/Frameworks --scan-folder $ROOT/build/Release/DemoTarget.app/Contents/PlugIns --scan-folder $ROOT/build/Release/DemoTarget.app/Contents/Library/SystemExtensions --platform macosx --toolchain /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain --destination $ROOT/build/Release/DemoTarget.app/Contents/Frameworks --strip-bitcode --strip-bitcode-tool $TOOLCHAIN_BIN/bitcode_strip --emit-dependency-info $ROOT/build/DemoTarget.build/Release/DemoTarget.build/SwiftStdLibToolInputDependencies.dep --filter-for-swift-os

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/Release/DemoTarget.app"),
        step.path
    );
    assert_eq!(
        step.to_string(),
        "[DemoProject.DemoTarget] Copying swift libraries ..."
    )
}
