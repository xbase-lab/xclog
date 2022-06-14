use crate::parser::{parse, XCOutput, XCOutputTask};
use anyhow::Result;
use process_stream::{Process, ProcessItem, Stream, StreamExt};
use std::ffi;
use std::path::PathBuf;
use std::{path::Path, pin::Pin};

/// XCLogger struct
pub struct XCLogger {
    #[allow(dead_code)]
    root: PathBuf,
    /// ..
    pub stream: Pin<Box<dyn Stream<Item = XCOutput> + Send>>,
}

impl XCLogger {
    /// Create new XCLogger instance via running xcodebuild with a in given root with given build arguments.
    pub fn new<P, I, S>(root: P, args: I) -> Result<Self>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<ffi::OsStr> + Send,
    {
        let mut process = Process::new("/usr/bin/xcodebuild");

        process.current_dir(&root);
        process.args(args);

        let output_stream = process.spawn_and_stream()?;

        Ok(Self {
            root: root.as_ref().to_path_buf(),
            stream: output_stream_to_xclogger_stream(output_stream.boxed()),
        })
    }

    /// Create new XCLogger instance from log lines
    pub fn new_from_lines<P: AsRef<Path> + Send>(root: P, lines: Vec<String>) -> Result<Self> {
        let mut lines = lines.into_iter();
        let output_stream = async_stream::stream! {
            while let Some(line) = lines.next() {
                yield ProcessItem::Output(line)
            }
        }
        .boxed();
        Ok(Self {
            root: root.as_ref().to_path_buf(),
            stream: output_stream_to_xclogger_stream(output_stream.boxed()),
        })
    }
}

/// TODO: return MatchOutput or XCLoggerOutput
fn output_stream_to_xclogger_stream(
    mut output_stream: Pin<Box<dyn Stream<Item = process_stream::ProcessItem> + Send>>,
) -> Pin<Box<dyn Stream<Item = XCOutput> + Send>> {
    async_stream::stream! {
        while let Some(output) = output_stream.next().await {
            match output {
                ProcessItem::Output(line) | ProcessItem::Error(line) => {
                    match parse(line, &mut output_stream).await {
                        Ok(Some(lines)) => { for line in lines.into_iter() { yield line } },
                        Err(e) => tracing::error!("ParseError: {e}"),
                        _ => ()
                    }
                },
                ProcessItem::Exit(status) => yield XCOutput {
                    value: format!("[Exit] {status}"), kind: XCOutputTask::Result
                }
            }
        }
    }
    .boxed()
}
