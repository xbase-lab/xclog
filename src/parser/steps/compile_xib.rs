use crate::parser::util::consume_empty_lines;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;

/// XIB compilation Step
#[derive(Debug)]
pub struct CompileXIB {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for CompileXIB {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CompileStoryboard".into(), "path".into()))?;

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
        CompileXIB,
       r#"CompileXIB CocoaChip/en.lproj/MainMenu.xib (in target 'DemoTarget' from project 'DemoProject')
    cd /Users/dustin/Source/CocoaChip
    setenv XCODE_DEVELOPER_USR_PATH /Applications/Xcode.app/Contents/Developer/usr/bin/..
    /Applications/Xcode.app/Contents/Developer/usr/bin/ibtool --errors ..

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(PathBuf::from("CocoaChip/en.lproj/MainMenu.xib"), step.path);
}
