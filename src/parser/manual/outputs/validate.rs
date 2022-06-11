use crate::parser::{Description, Error, OutputStream, ParsableFromStream, Step};
use async_trait::async_trait;
use process_stream::ProcessItem;
use std::{fmt::Display, path::PathBuf};
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
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("Validate".into(), "path".into()))?;

        let mut skip = false;
        let description = Description::from_line(line)?;

        while let Some(s) = stream.next().await {
            if let ProcessItem::Output(line) = s {
                if line.contains("-no-validate-extension") {
                    skip = true;
                }
                if line.trim().is_empty() {
                    break;
                }
            } else if let ProcessItem::Error(line) = s {
                steps.push(Step::Error(line))
            } else if s.trim().is_empty() {
                break;
            }
        }

        steps.push(Step::Validate(Self {
            description,
            path,
            skip,
        }));

        steps.pipe(Ok)
    }
}

impl Display for Validate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.skip { "[skipped]" } else { "" };
        write!(
            f,
            "{} Validating `{}` {}",
            self.description,
            self.path.file_name().unwrap().to_string_lossy(),
            status
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        Validate,
       r#"Validate $ROOT/build/Release/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    builtin-validationUtility $ROOT/build/Release/DemoTarget.app -no-validate-extension
"# 
    };

    if let Step::Validate(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(
            PathBuf::from("$ROOT/build/Release/DemoTarget.app"),
            step.path
        );
        assert_eq!(true, step.skip);
        assert_eq!(
            "[DemoTarget] Validating `DemoTarget.app` [skipped]",
            step.to_string()
        )
    } else {
        panic!("No script execution {steps:#?}")
    }
}
