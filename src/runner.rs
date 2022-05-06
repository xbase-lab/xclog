use std::ffi;
use std::path::Path;

use anyhow::Result;
use futures::{stream::Stream, stream::StreamExt};
use tap::Pipe;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{ChildStderr, ChildStdout, Command};
use tokio_stream::wrappers::LinesStream;

#[derive(Clone, Debug)]
pub enum ProcessUpdate {
    Stdout(String),
    Stderr(String),
    Exit(String),
    Error(String),
}

impl std::ops::Deref for ProcessUpdate {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            ProcessUpdate::Stdout(s) => s,
            ProcessUpdate::Stderr(s) => s,
            ProcessUpdate::Exit(s) => s,
            ProcessUpdate::Error(s) => s,
        }
    }
}

#[allow(dead_code)]
async fn spawn_stream<P, I, S>(
    root: P,
    args: I,
) -> Result<(
    LinesStream<BufReader<ChildStdout>>,
    LinesStream<BufReader<ChildStderr>>,
)>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let mut build = Command::new("/usr/bin/xcodebuild")
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(root)
        .spawn()?;

    let readers = (
        build
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("child did not have a handle to stdout"))?
            .pipe(BufReader::new)
            .lines()
            .pipe(LinesStream::new),
        build
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("child did not have a handle to stdout"))?
            .pipe(BufReader::new)
            .lines()
            .pipe(LinesStream::new),
    );

    tokio::spawn(async move {
        let status = build.wait().await.unwrap();

        #[cfg(feature = "tracing")]
        tracing::info!("build status: {status}");
    });

    Ok(readers)
}

pub async fn spawn<P, I, S>(root: P, args: I) -> Result<impl Stream<Item = crate::parser::Step>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
    P: AsRef<Path>,
{
    let mut child = tokio::process::Command::new("/usr/bin/xcodebuild")
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(root)
        .spawn()?;

    let stdout_reader = child
        .stdout
        .take()
        .unwrap()
        .pipe(tokio::io::BufReader::new)
        .lines()
        .pipe(tokio_stream::wrappers::LinesStream::new)
        .map(|line| match line {
            Ok(s) => ProcessUpdate::Stdout(s),
            Err(e) => ProcessUpdate::Error(format!("stderr: {e}")),
        });

    let stderr_reader = child
        .stderr
        .take()
        .unwrap()
        .pipe(tokio::io::BufReader::new)
        .lines()
        .pipe(tokio_stream::wrappers::LinesStream::new)
        .map(|line| match line {
            Ok(s) => ProcessUpdate::Stderr(s),
            Err(e) => ProcessUpdate::Error(format!("stderr: {e}")),
        });

    let exit_status = tokio::spawn(async move { child.wait().await });

    async_stream::stream! {
        let mut reader = tokio_stream::StreamExt::merge(stdout_reader, stderr_reader)
            .chain(futures::stream::once(async {
                match exit_status.await {
                    Ok(x) => match x {
                        Ok(x) => ProcessUpdate::Exit(x.code().unwrap_or(0).to_string()),
                        Err(e) => ProcessUpdate::Error(e.to_string()),
                    },
                    Err(e) => ProcessUpdate::Error(e.to_string()),
                }
            }))
        .boxed();

        while let Some(update) = reader.next().await {
            match update {
                ProcessUpdate::Stdout(line) => {
                    if !line.is_empty() {
                        match crate::parser::parse_step_from_stream(line, &mut reader).await {
                            Ok(v) => if let Some(step) = v { yield step; }
                            Err(e) => yield crate::parser::Step::Error(e.to_string())
                        }
                    }
                }
                ProcessUpdate::Exit(status) => yield crate::parser::Step::Exit(status),
                ProcessUpdate::Error(e) => yield crate::parser::Step::Error(e),
                ProcessUpdate::Stderr(e) => yield crate::parser::Step::Error(e),
            }
        }
    }
    .boxed()
    .pipe(Ok)
}

pub async fn spawn_once<P, I, S>(root: P, args: I) -> Result<()>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    Command::new("/usr/bin/xcodebuild")
        .args(args)
        .current_dir(root)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()?
        .wait()
        .await?;
    Ok(())
}
