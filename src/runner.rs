use crate::parser::BuildSettings;
use crate::parser::{parse_step_from_stream, Step};
use anyhow::Result;
use process_stream::{Process, ProcessItem, Stream, StreamExt};
use std::ffi;
use std::path::Path;
use std::process::Stdio;
use tap::Pipe;
use tokio::process::Command;

pub async fn spawn<P, I, S>(root: P, args: I) -> Result<impl Stream<Item = crate::parser::Step>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
    P: AsRef<Path>,
{
    let mut xcodebuild = Process::new("/usr/bin/xcodebuild");
    xcodebuild.args(args);
    xcodebuild.current_dir(root);

    let mut reader = xcodebuild.spawn_and_stream()?;

    async_stream::stream! {
        use ProcessItem::*;
        while let Some(update) = reader.next().await {
            // tracing::warn!("{update:?}");
            match update {
                Output(line) | Error(line) => {
                    if !line.is_empty() {
                        match parse_step_from_stream(line, &mut reader).await {
                            Ok(v) => if let Some(steps) = v {
                                for step in steps.into_iter() {
                                    yield step

                                }
                            }
                            Err(e) => {
                                tracing::error!("Fail to parse step {e}");
                                yield Step::Error(e.to_string())
                            }
                        }
                    }
                }
                Exit(status) => match status.parse::<i32>() {
                    Ok(code) => yield Step::Exit(code),
                    Err(err) => yield Step::Error(format!("fail to parse exit code as i32 {err}")),
                },
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
