use crate::parser::{consume_empty_lines, Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;

/// Info plist process step
#[derive(Debug)]
pub struct ProcessInfoPlistFile {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for ProcessInfoPlistFile {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        chunks.next();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("ProcessInfoPlistFile".into(), "path".into()))?;

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
        ProcessInfoPlistFile,
       r#"ProcessInfoPlistFile $ROOT/build/Debug-iphoneos/DemoTarget.app/Info.plist $ROOT/Resources/Info.plist (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    builtin-infoPlistUtility $ROOT/Resources/Info.plist -producttype com.apple.product-type.application -genpkginfo $ROOT/build/Debug-iphoneos/DemoTarget.app/PkgInfo -expandbuildsettings -format binary -platform iphoneos -additionalcontentfile $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/LaunchScreen-SBPartialInfo.plist -additionalcontentfile $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/assetcatalog_generated_info.plist -requiredArchitecture arm64 -o $ROOT/build/Debug-iphoneos/DemoTarget.app/Info.plist

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(PathBuf::from("$ROOT/Resources/Info.plist"), step.path);
}
