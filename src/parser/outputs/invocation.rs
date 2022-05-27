use super::super::{consume_till_empty_line, Error, OutputStream, ParsableFromStream};
use crate::parser::Step;
use async_trait::async_trait;
use process_stream::ProcessItem;
use std::{collections::HashMap, fmt::Display};
use tap::Pipe;
use tokio_stream::StreamExt;

/// Command invocated
#[derive(Debug)]
pub struct Invocation {
    pub command: String,
    pub arguments: Vec<String>,
    pub env_vars: HashMap<String, String>,
}

#[async_trait]
impl ParsableFromStream for Invocation {
    async fn parse_from_stream(_: String, stream: &mut OutputStream) -> Result<Vec<Step>, Error> {
        match stream.next().await {
            Some(ProcessItem::Output(args)) => {
                consume_till_empty_line(stream).await;
                let mut chunks = args.trim().split_whitespace();
                let command = chunks
                    .next()
                    .ok_or_else(|| Error::EOF("Invocation".into(), "command".into()))?
                    .to_string();

                let mut arguments = vec![];
                let mut env_vars = HashMap::default();

                for value in chunks {
                    if value.contains("=") {
                        let parts = value.split('=').collect::<Vec<&str>>();
                        if let (Some(key), Some(value)) = (parts.get(0), parts.get(1)) {
                            env_vars.insert(key.to_lowercase().to_string(), value.to_string());
                        }
                    } else {
                        arguments.push(value.to_string());
                    }
                }

                vec![Step::Invocation(Self {
                    command,
                    arguments,
                    env_vars,
                })]
                .pipe(Ok)
            }
            _ => Err(Error::Failure("Invocation".into())),
        }
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        Invocation,
        r#"Command line invocation:
    /Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild build SYMROOT=/path/to/symroot

    "#
    };
    if let Step::Invocation(step) = steps.first().unwrap() {
        assert_eq!(
            "/Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild",
            &step.command
        );
        assert_eq!(
            HashMap::from([("symroot".into(), "/path/to/symroot".into())]),
            step.env_vars
        );
        assert_eq!(vec!["build".to_string()], step.arguments);
    } else {
        panic!("No script execution {steps:#?}")
    }
}

impl Display for Invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Running] {} {}",
            self.command.split("/").last().unwrap(),
            self.arguments.join(" "),
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn fmt() {
    let data = Invocation {
        command: "/Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild".into(),
        arguments: vec!["build".into()],
        env_vars: HashMap::from([("symroot".into(), "/path/to/symroot".into())]),
    };

    assert_eq!("[Running] xcodebuild build", &format!("{}", data),);
}
