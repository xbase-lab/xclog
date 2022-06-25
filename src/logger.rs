use crate::parser::{parse, XCLOG_MATCHER};
use crate::XCCompileCommand;
use anyhow::Result;
use async_stream::stream;
use process_stream::{ProcessExt, ProcessItem, Stream, StreamExt};
use std::ffi;
use std::path::PathBuf;
use std::sync::Arc;
use std::{path::Path, pin::Pin};
use tokio::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

/// XCLogger struct
#[derive(derive_deref_rs::Deref)]
pub struct XCLogger {
    #[allow(dead_code)]
    root: PathBuf,
    #[deref]
    inner: tokio::process::Command,
    kill_send: Option<Sender<()>>,
    /// Arc Reference to compile_commands
    pub compile_commands: Arc<Mutex<Vec<XCCompileCommand>>>,
}

impl ProcessExt for XCLogger {
    fn get_command(&mut self) -> &mut tokio::process::Command {
        &mut self.inner
    }

    fn killer(&self) -> Option<Sender<()>> {
        self.kill_send.clone()
    }

    fn set_killer(&mut self, killer: Sender<()>) {
        self.kill_send = killer.into()
    }

    fn spawn_and_stream(
        &mut self,
    ) -> std::io::Result<Pin<Box<dyn Stream<Item = ProcessItem> + Send>>> {
        let mut output_stream = self._spawn_and_stream()?;
        let compile_commands = self.compile_commands.clone();

        Ok(stream! {
            let mut compile_commands = compile_commands.lock().await;
            while let Some(output) = output_stream.next().await {
                match output {
                    ProcessItem::Error(line) => {
                        match parse(line, &mut output_stream).await {
                            Ok(Some(lines)) => {
                                for line in lines.into_iter() {
                                        yield ProcessItem::Error(line.to_string())
                                } },
                            Err(e) => tracing::error!("ParseError: {e}"),
                            _ => ()
                        }
                    },
                    ProcessItem::Output(line) => {
                        if let Some(cmd) = XCLOG_MATCHER.get_compile_command(line.as_str()).and_then(XCCompileCommand::from_compile_command_data) {
                            compile_commands.push(cmd);
                        } else {
                            match parse(line, &mut output_stream).await {
                                Ok(Some(lines)) => {
                                    for line in lines.into_iter() {
                                            yield ProcessItem::Output(line.to_string())
                                    } },
                                Err(e) => tracing::error!("ParseError: {e}"),
                                _ => ()
                            }
                        }

                    },
                    output => yield output
                }
            }
        }
        .boxed())
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
            kill_send: None,
            compile_commands: Default::default(),
        })
    }
}
