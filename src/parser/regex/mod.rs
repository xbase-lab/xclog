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
                            Ok(Some(lines)) => { for line in lines.into_iter() { yield line } },
                            Err(e) => tracing::error!("ParseError: {e}"),
                            _ => ()
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

async fn parse(line: String, stream: &mut OutputStream) -> Result<Option<Vec<String>>> {
    if line.contains("ONLY_ACTIVE_ARCH=YES") {
        return Ok(None);
    }

    let matcher = match MATCHER.capture(&line) {
        Some(m) => m,
        None => return Ok(None),
    };

    let mut lines = vec![];
    let line = match matcher.output()?.value {
        Some(line) => line,
        None => return Ok(None),
    };

    let (is_compile_warning, is_compile_error) =
        (matcher.is_compile_warning(), matcher.is_compile_error());
    if is_compile_warning || is_compile_error {
        let leading = if is_compile_error {
            "[Error]"
        } else {
            "[Warning]"
        };
        lines.push(leading.to_string());
        lines.push(leading.to_string());
        lines.push(line);
        while let Some(line) = stream.next().await.map(|s| s.to_string()) {
            if line.is_empty() {
                break;
            }
            lines.push(format!("{leading} {line}"));
        }
        lines.push(leading.to_string());
        lines.push(leading.to_string());
    } else {
        lines.push(line);
    }

    Ok(Some(lines))
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
