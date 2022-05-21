use crate::parser::{parse_step_from_stream, Step};
use anyhow::Result;
use futures::{stream::Stream, stream::StreamExt};
use std::ffi;
use std::path::Path;
use std::process::Stdio;
use tap::Pipe;
use tokio::process::Command;

use crate::parser::BuildSettings;
use process_stream::{Process, ProcessItem};

pub async fn spawn<P, I, S>(root: P, args: I) -> Result<impl Stream<Item = crate::parser::Step>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
    P: AsRef<Path>,
{
    let mut xcodebuild = Process::new("/usr/bin/xcodebuild");
    xcodebuild.args(args);
    xcodebuild.current_dir(root);

    let mut reader = xcodebuild.stream()?;

    async_stream::stream! {
        use ProcessItem::*;
        while let Some(update) = reader.next().await {
            match update {
                Output(line) => {
                    if !line.is_empty() {
                        match parse_step_from_stream(line, &mut reader).await {
                            Ok(v) => if let Some(step) = v { yield step; }
                            Err(e) => yield Step::Error(e.to_string())
                        }
                    }
                }
                Exit(status) => match status.parse::<i32>() {
                    Ok(code) => yield Step::Exit(code),
                    Err(err) => yield Step::Error(format!("fail to parse exit code as i32 {err}")),
                },
                Error(e) => yield crate::parser::Step::Error(e),
            }
        }
    }
    .boxed()
    .pipe(Ok)
}

pub async fn build_settings<P, I, S>(root: P, args: I) -> Result<BuildSettings>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let output = Command::new("/usr/bin/xcodebuild")
        .args(args)
        .arg("-showBuildSettings")
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    if output.status.success() {
        BuildSettings::new(String::from_utf8(output.stdout)?.split("\n"))
    } else {
        anyhow::bail!(String::from_utf8(output.stderr)?)
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_build_settings() {
    let root = "/Users/tami5/repos/swift/wordle";

    // spawn_once(root, &["clean"]).await.unwrap();

    let data = build_settings(
        root,
        &[
            "build",
            "-configuration",
            "Debug",
            "-target",
            "Wordle",
            "-showBuildSettings",
            "-sdk",
            "iphonesimulator",
        ],
    )
    .await
    .unwrap();

    tracing::info!("{:#?}", data);
}

// https://github.com/ThatAnnoyingKid/pikadick-rs/blob/cecd1a88882fe3c07a9f8c52e81a97ca6e5f013e/lib/tokio-ffmpeg-cli-rs/src/lib.rs
// https://github.com/zhaofengli/colmena/blob/09a8a72b0c5113aa40648949986278040487c9bd/src/nix/evaluator/nix_eval_jobs.rs
// https://github.com/ezclap-tv/shit-chat-says/blob/c34be8edd12ade50c04ea879403a1a5d8db745d4/scs-manage-api/src/v1.rs
// https://github.com/MrRobu/concurrent-and-distributed-computing/blob/7292cc1188b3a66cf26f756d40b47894fc1c631a/homework1/src/bin/rce-agent.rs
