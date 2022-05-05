use std::ffi;

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
async fn spawn_stream<I, S>(
    root: &str,
    args: I,
) -> Result<(
    LinesStream<BufReader<ChildStdout>>,
    LinesStream<BufReader<ChildStderr>>,
)>
where
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

    // // Make child produce exit event
    // let exit_status_stream = Box::pin(async move { build.wait().await })
    //     .into_stream()
    //     .map(|maybe_exit_status| maybe_exit_status.map(Event::ExitStatus).map_err(Error::Io));

    tokio::spawn(async move {
        let status = build.wait().await.unwrap();

        #[cfg(feature = "tracing")]
        tracing::info!("build status: {status}");
    });

    Ok(readers)
}

pub async fn spawn<I, S>(root: &str, args: I) -> Result<impl Stream<Item = crate::parser::Step>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
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

pub async fn spawn_once<I, S>(root: &str, args: I) -> Result<()>
where
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

// https://github.com/ThatAnnoyingKid/pikadick-rs/blob/cecd1a88882fe3c07a9f8c52e81a97ca6e5f013e/lib/tokio-ffmpeg-cli-rs/src/lib.rs
// https://github.com/zhaofengli/colmena/blob/09a8a72b0c5113aa40648949986278040487c9bd/src/nix/evaluator/nix_eval_jobs.rs
// https://github.com/ezclap-tv/shit-chat-says/blob/c34be8edd12ade50c04ea879403a1a5d8db745d4/scs-manage-api/src/v1.rs
// https://github.com/MrRobu/concurrent-and-distributed-computing/blob/7292cc1188b3a66cf26f756d40b47894fc1c631a/homework1/src/bin/rce-agent.rs
