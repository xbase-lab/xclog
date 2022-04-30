use super::super::{consume_empty_lines, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use tap::Pipe;
use tokio_stream::StreamExt;

/// Command invocated
#[derive(Debug)]
pub struct Invocation {
    pub command: String,
    pub arguments: Vec<String>,
}

#[async_trait]
impl ParsableFromStream for Invocation {
    async fn parse_from_stream(_: String, stream: &mut OutputStream) -> Result<Self, Error> {
        match stream.try_next().await {
            Ok(Some(args)) => {
                consume_empty_lines(stream).await;
                let mut chunks = args.trim().split_whitespace();
                let command = chunks
                    .next()
                    .ok_or_else(|| Error::EOF("Invocation".into(), "command".into()))?
                    .to_string();

                let arguments = chunks.map(|s| s.to_string()).collect();
                Self { command, arguments }.pipe(Ok)
            }
            _ => Err(Error::Failure("Invocation".into())),
        }
    }
}

#[tokio::test]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let step = to_stream_test! {
        Invocation,
        r#"Command line invocation:
    /Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild build

    "#
    };

    assert_eq!(
        "/Applications/Xcode.app/Contents/Developer/usr/bin/xcodebuild",
        &step.command
    );
    assert_eq!(vec!["build".to_string()], step.arguments);
}
