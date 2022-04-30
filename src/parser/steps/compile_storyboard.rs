use crate::parser::util::consume_empty_lines;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;

/// Storyboard compilation Step
#[derive(Debug)]
pub struct CompileStoryboard {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for CompileStoryboard {
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
        CompileStoryboard,
       r#"CompileStoryboard /path/to/a.storyboard (in target 'DemoTarget' from project 'DemoProject')
    cd /Users/tami5/repos/swift/wordle
    export XCODE_DEVELOPER_USR_PATH\=/Applications/Xcode.app/Contents/Developer/usr/bin/..
    /Applications/Xcode.app/Contents/Developer/usr/bin/ibtool ...

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(PathBuf::from("/path/to/a.storyboard"), step.path);
}
