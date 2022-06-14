#![deny(missing_docs)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod build_settings;
pub mod compile;
pub mod parser;

use anyhow::Result;
use build_settings::BuildSettings;
use compile::{CompilationDatabase, CompileCommand};
use parser::{MatchOutput, MATCHER};
use process_stream::{Process, ProcessItem, Stream, StreamExt};
use std::ffi;
use std::{path::Path, pin::Pin};
use tap::Pipe;

async fn get_output_stream<P, I, S>(
    root: P,
    args: I,
) -> Result<Pin<Box<dyn Stream<Item = MatchOutput> + Send>>>
where
    P: AsRef<Path> + Send,
    I: IntoIterator<Item = S> + Send,
    S: AsRef<ffi::OsStr> + Send,
{
    let mut process = Process::new("/usr/bin/xcodebuild");
    process.current_dir(root);
    process.args(args);

    let mut stream = process.spawn_and_stream()?;

    async_stream::stream! {
        while let Some(output) = stream.next().await {
            match output {
                ProcessItem::Output(line) | ProcessItem::Error(line) => {
                    if let Some(output) = parser::MATCHER.capture(&line).map(|m| m.output().ok()).flatten() {
                        yield output
                    }
                },
                _ => {}

            }
        }
    }
    .boxed()
    .pipe(Ok)
}

/// Generate logs through running a process
pub async fn get_log_stream<P, I, S>(
    root: P,
    args: I,
) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>>
where
    P: AsRef<Path> + Send,
    I: IntoIterator<Item = S> + Send,
    S: AsRef<ffi::OsStr> + Send,
{
    let mut process = Process::new("/usr/bin/xcodebuild");
    process.current_dir(root);
    process.args(args);

    let mut stream = process.spawn_and_stream()?;

    async_stream::stream! {
        while let Some(output) = stream.next().await {
            match output {
                ProcessItem::Output(line) | ProcessItem::Error(line) => {
                    match parser::parse(line, &mut stream).await {
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

/// Generate logs via a vector of iles
pub async fn get_log_stream_from_lines(
    lines: Vec<String>,
) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
    let mut lines = lines.into_iter();
    let mut stream = async_stream::stream! {
        while let Some(line) = lines.next() {
            yield ProcessItem::Output(line)
        }
    }
    .boxed();

    async_stream::stream! {
        while let Some(line) = stream.next().await {
            match parser::parse(line.to_string(), &mut stream).await {
                Ok(Some(lines)) => {
                    for line in lines.into_iter() { yield line }
                },
                Err(e) => tracing::error!("ParseError: {e}"),
                _ => {}
            }
        }
    }
    .boxed()
    .pipe(Ok)
}

/// Get Build Settings
pub async fn get_build_settings<P, I, S>(root: P, args: I) -> Result<BuildSettings>
where
    P: AsRef<Path> + Send,
    I: IntoIterator<Item = S> + Send,
    S: AsRef<ffi::OsStr> + Send,
{
    let mut process = Process::new("/usr/bin/xcodebuild");

    process.current_dir(root);
    process.args(args);

    let output = process.spawn()?.wait_with_output().await?;

    if output.status.success() {
        BuildSettings::new(String::from_utf8(output.stdout)?.split("\n"))
    } else {
        anyhow::bail!(String::from_utf8(output.stderr)?)
    }
}

/// Get Compile commands from running args on given root directory
pub async fn get_compile_commands<P, I, S>(root: P, args: I) -> Result<CompilationDatabase>
where
    P: AsRef<Path> + Send,
    I: IntoIterator<Item = S> + Send,
    S: AsRef<ffi::OsStr> + Send,
{
    let mut process = Process::new("/usr/bin/xcodebuild");
    process.current_dir(root);
    process.args(args);

    process
        .spawn_and_stream()?
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|o| {
            MATCHER
                .get_compile_command(o.to_string().as_str())
                .map(CompileCommand::from_compile_command_data)
                .flatten()
        })
        .flatten()
        .collect::<Vec<_>>()
        .pipe(|vec| CompilationDatabase(vec))
        .pipe(Ok)
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = "Local tests"]
async fn test_get_compile_commands() {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    let compile_commands = get_compile_commands(root, &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
    ]).await.unwrap();

    println!("{:#?}", compile_commands.len());
    for command in compile_commands.iter() {
        if let Some(ref command) = command.name {
            println!("{:?}", command);
        } else if let Some(ref file) = command.file {
            println!("{:?}", file);
        } else {
            println!("{:?}", command);
        }
    }
}
