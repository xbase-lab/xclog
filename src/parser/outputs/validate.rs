use crate::{
    parser::{Description, Error, OutputStream, ParsableFromStream},
    runner::ProcessUpdate,
};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;
use tokio_stream::StreamExt;

/// Validate app
#[derive(Debug)]
pub struct Validate {
    pub description: Description,
    pub path: PathBuf,
    pub skip: bool,
}

#[async_trait]
impl ParsableFromStream for Validate {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("Validate".into(), "path".into()))?;

        let mut skip = false;
        let description = Description::from_line(line)?;
        while let Some(ProcessUpdate::Stdout(line)) = stream.next().await {
            if line.contains("-no-validate-extension") {
                skip = true;
            }

            if line.is_empty() {
                break;
            }
        }

        Self {
            description,
            path,
            skip,
        }
        .pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        Validate,
       r#"Validate $ROOT/build/Release/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    builtin-validationUtility $ROOT/build/Release/DemoTarget.app -no-validate-extension
"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/Release/DemoTarget.app"),
        step.path
    );
    assert_eq!(true, step.skip);
}
