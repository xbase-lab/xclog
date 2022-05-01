use crate::parser::util::consume_empty_lines;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;

/// Linking of a library
#[derive(Debug)]
pub struct Ld {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for Ld {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("Ld".into(), "path".into()))?;

        let description = Description::from_line(line)?;

        consume_empty_lines(stream).await;

        Self { description, path }.pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        Ld,
       r#"Ld $ROOT/build/Debug-iphoneos/DemoTarget.app/DemoTarget normal (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    $TOOLCHAIN_BIN/clang -target arm64-apple-ios15.0 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS15.4.sdk -L$ROOT/build/Debug-iphoneos -F$ROOT/build/Debug-iphoneos -filelist $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.LinkFileList -Xlinker -rpath -Xlinker @executable_path/Frameworks -dead_strip -Xlinker -object_path_lto -Xlinker $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget_lto.o -Xlinker -export_dynamic -Xlinker -no_deduplicate -fembed-bitcode-marker -fobjc-link-runtime -L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/iphoneos -L/usr/lib/swift -Xlinker -add_ast_path -Xlinker $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget.swiftmodule -Xlinker -no_adhoc_codesign -Xlinker -dependency_info -Xlinker $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Objects-normal/arm64/DemoTarget_dependency_info.dat -o $ROOT/build/Debug-iphoneos/DemoTarget.app/DemoTarget

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/Debug-iphoneos/DemoTarget.app/DemoTarget"),
        step.path
    );
}
