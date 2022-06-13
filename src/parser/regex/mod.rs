#[macro_use]
mod define;
mod generate;
mod output;

use crate::parser::OutputStream;
use anyhow::Result;
use process_stream::{Process, ProcessItem, Stream, StreamExt};
use std::pin::Pin;
use tap::Pipe;

pub use generate::*;
pub use output::*;

#[async_trait::async_trait]
pub trait XcodeLogger {
    /// Generate logs from [`Process`]
    ///
    /// [`Process`]: process_stream::Process
    async fn stream_logs(&mut self) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl XcodeLogger for Process {
    async fn stream_logs(&mut self) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>>
    where
        Self: Sized,
    {
        let mut stream = self.spawn_and_stream()?;

        async_stream::stream! {
            while let Some(output) = stream.next().await {
                match output {
                    ProcessItem::Output(line) | ProcessItem::Error(line) => {
                        match parse(line, &mut stream).await {
                            Ok(lines) => { for line in lines.into_iter() { yield line } },
                            Err(e) => tracing::error!("ParseError: {e}"),
                        }
                    },
                    ProcessItem::Exit(status) => yield format!("[Exit] {status}")
                }
            }
        }
        .boxed()
        .pipe(Ok)
    }
}

async fn parse(line: String, stream: &mut OutputStream) -> Result<Vec<String>> {
    let mut lines = vec![];

    let matcher = match MATCHER.capture(&line) {
        Some(m) => m,
        None => return Ok(lines),
    };

    let output = matcher.output()?;
    if let Some(line) = output.value {
        lines.push(line);
    }

    if matcher.is_warning() || matcher.is_error() {
        if let Some(issue) = stream.next().await {
            let output_line = issue.to_string();
            if let Some(line) = MATCHER
                .capture(&output_line)
                .map(|m| m.output().ok())
                .flatten()
                .map(|o| o.value)
                .flatten()
            {
                lines.push(line)
            } else {
                lines.push(format!("[Warning] {output_line}"))
            }
        }
    }

    Ok(lines)
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_case_2() {
    use process_stream::StreamExt;

    let mut process: Process = Process::new("/usr/bin/xcodebuild");

    process.current_dir("/Users/tami5/repos/swift/yabaimaster");

    process.args(&[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
    ]);

    let mut stream = process.stream_logs().await.unwrap();

    while let Some(line) = StreamExt::next(&mut stream).await {
        println!("{}", line)
    }
}
