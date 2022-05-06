use crate::parser::{consume_till_empty_line, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Build Directory Creation Step
#[derive(Debug)]
pub struct CreateBuildDirectory {
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for CreateBuildDirectory {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        consume_till_empty_line(stream).await;

        Self {
            path: PathBuf::from(line),
        }
        .pipe(Ok)
    }
}

impl Display for CreateBuildDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Mkdir] {}", self.path.display())
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        CreateBuildDirectory,
       r#"CreateBuildDirectory $ROOT/build/Release
    cd $ROOT/DemoTarget.xcodeproj
    builtin-create-build-directory $ROOT/build/Release

"# 
    };
    assert_eq!(PathBuf::from("$ROOT/build/Release"), step.path);

    assert_eq!(
        step.to_string(),
        "[Create Build Directory] `$ROOT/build/Release`"
    )
}
