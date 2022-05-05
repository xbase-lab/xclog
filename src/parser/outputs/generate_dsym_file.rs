use crate::parser::Description;
use crate::parser::{Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;

/// DSM File Generation Step
#[derive(Debug)]
pub struct GenerateDSYMFile {
    pub description: Description,
    pub output_path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for GenerateDSYMFile {
    async fn parse_from_stream(line: String, _stream: &mut OutputStream) -> Result<Self, Error> {
        Self {
            output_path: line
                .split_whitespace()
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| Error::EOF("GenerateDSYMFile".into(), "output_path".into()))?,
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
        GenerateDSYMFile,
       r#"GenerateDSYMFile $ROOT/build/Release/DemoTarget.app.dSYM $ROOT/build/Release/DemoTarget.app/Contents/MacOS/DemoTarget (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    $TOOLCHAIN_BIN/dsymutil $ROOT/build/Release/DemoTarget.app/Contents/MacOS/DemoTarget -o $ROOT/build/Release/DemoTarget.app.dSYM

"# 
    };

    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/Release/DemoTarget.app.dSYM"),
        step.output_path
    );
}
