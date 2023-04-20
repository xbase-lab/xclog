use crate::parser::{parse, XCOutput, XCOutputTask, XCLOG_MATCHER};
use crate::XCCompileCommand;
use anyhow::Result;
use async_stream::stream;
use process_stream::{ProcessExt, ProcessItem, Stream, StreamExt};
use std::ffi;
use std::path::PathBuf;
use std::sync::Arc;
use std::{path::Path, pin::Pin};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::sync::Notify;

/// XCLogger struct
#[derive(derive_deref_rs::Deref)]
pub struct XCLogger {
    #[allow(dead_code)]
    root: PathBuf,
    #[deref]
    inner: tokio::process::Command,
    abort: Option<Arc<Notify>>,
    /// Arc Reference to compile_commands
    pub compile_commands: Arc<Mutex<Vec<XCCompileCommand>>>,
}

impl ProcessExt for XCLogger {
    fn get_command(&mut self) -> &mut tokio::process::Command {
        &mut self.inner
    }

    fn spawn_and_stream(
        &mut self,
    ) -> std::io::Result<Pin<Box<dyn Stream<Item = ProcessItem> + Send>>> {
        let process_stream = self._spawn_and_stream()?;
        let mut output_stream = self.process_stream(process_stream);

        Ok(stream! {
            while let Some(output) = output_stream.next().await {
                match output.kind {
                    XCOutputTask::Task | XCOutputTask::Test | XCOutputTask::Warning => {
                        yield ProcessItem::Output(output.to_string())
                    }
                    XCOutputTask::Error => yield ProcessItem::Error(output.to_string()),
                    XCOutputTask::Exit => yield ProcessItem::Exit(output.value),
                    XCOutputTask::Result => {
                        yield ProcessItem::Output(output.to_string())
                    }
                };
            };
        }
        .boxed())
    }

    fn aborter(&self) -> Option<Arc<Notify>> {
        self.abort.clone()
    }

    fn set_aborter(&mut self, aborter: Option<Arc<Notify>>) {
        self.abort = aborter
    }
}

impl XCLogger {
    /// Create new XCLogger instance via running xcodebuild with a in given root with given build arguments.
    pub fn new<P, I, S>(root: P, args: I) -> Result<Self>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<ffi::OsStr> + Send,
    {
        let mut inner = Command::new("/usr/bin/xcodebuild");

        inner.current_dir(&root);
        inner.args(args);

        Ok(Self {
            root: root.as_ref().to_path_buf(),
            inner,
            abort: None,
            compile_commands: Default::default(),
        })
    }

    pub(crate) fn process_stream(
        &self,
        mut output_stream: Pin<Box<dyn Stream<Item = ProcessItem> + Send>>,
    ) -> Pin<Box<dyn Stream<Item = XCOutput> + Send>> {
        let compile_commands = self.compile_commands.clone();

        stream! {
            let mut compile_commands = compile_commands.lock().await;
            while let Some(output) = output_stream.next().await {

                // Try to process compile command first
                if let ProcessItem::Output(line) = &output {
                    if let Some(cmd) = XCLOG_MATCHER
                        .get_compile_command(line.as_str())
                        .and_then(XCCompileCommand::from_compile_command_data)
                    {
                        compile_commands.push(cmd);
                        continue;
                    };
                };

                match output {
                    ProcessItem::Error(line) => {
                        match parse(line, &mut output_stream).await {
                            Ok(Some(lines)) => {
                                for output in lines.into_iter() {
                                    yield output
                                } },
                            Err(e) => tracing::error!("ParseError: {e}"),
                            _ => ()
                        }
                    },
                    ProcessItem::Output(line) => {
                        match parse(line, &mut output_stream).await {
                            Ok(Some(outputs)) => {
                                for output in outputs.into_iter() {
                                    yield output
                                } },
                            Err(e) => tracing::error!("ParseError: {e}"),
                            _ => ()
                        }

                    },
                    ProcessItem::Exit(exit) => {
                        let value = exit.trim();
                        yield XCOutput {
                            kind: XCOutputTask::Exit,
                            value: value.into()
                        };
                    }
                }
            }
        }
        .boxed()
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn case_d() {
    let logger = XCLogger::new("", [""]).expect("Create logger");
    let stream = stream! {
        let content = include_str!("../tests/case_d.log");
        let lines = content.split("\n");
        for line in lines {
            yield ProcessItem::Output(line.to_string())
        }
    }
    .boxed();
    let error_outputs = logger
        .process_stream(stream)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter(|t| t.is_error())
        .collect::<Vec<_>>();
    assert_eq!(error_outputs.len(), 0, "{error_outputs:#?}")
}
