mod build_settings;
mod description;
mod error;
mod outputs;
mod step;
pub mod util;

use async_trait::async_trait;
use process_stream::ProcessItem;
use tap::Pipe;

pub use build_settings::*;
pub use description::Description;
pub use error::Error;
pub use outputs::*;
pub use step::Step;
pub use util::*;

pub type OutputStream = dyn tokio_stream::Stream<Item = ProcessItem> + Unpin + Send;

#[async_trait]
pub trait ParsableFromStream {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Vec<Step>, Error>
    where
        Self: Sized + Send;
}

pub async fn parse_step_from_stream(
    line: String,
    stream: &mut OutputStream,
) -> Result<Option<Vec<Step>>, Error> {
    let mut chunks = line.trim().split_whitespace();

    let (cmd, line) = match chunks.next() {
        Some(cmd) => {
            if cmd == "Create"
                || cmd == "User"
                || cmd == "Touch"
                || cmd == "MkDir"
                || cmd == "Copy"
                || cmd == "WriteAuxiliaryFile"
                || cmd == "Build"
                || cmd == "Analyze"
                || cmd == "cd"
                || cmd == "RegisterExecutionPolicyException"
                || cmd == "Resolve"
            {
                consume_till_empty_line(stream).await;
                return Ok(None);
            }
            (cmd.to_string(), chunks.collect::<String>())
        }
        None => return Err(Error::Failure("Empty Line, couldn't identity step".into())),
    };

    match cmd.as_str() {
        "Command" => Invocation::parse_from_stream(line, stream).await?,
        "RegisterWithLaunchServices" => {
            RegisterWithLaunchServices::parse_from_stream(line, stream).await?
        }
        "Resolved" if line.contains("source packages") => {
            ResolvedSourcePackages::parse_from_stream(line, stream).await?
        }
        "CompileSwift" => CompileSwift::parse_from_stream(line, stream).await?,
        "CompileSwiftSources" => CompileSwiftSources::parse_from_stream(line, stream).await?,
        "CompileC" => CompileC::parse_from_stream(line, stream).await?,
        "CodeSign" => CodeSign::parse_from_stream(line, stream).await?,
        "CompileAssetCatalog" => CompileAssetCatalog::parse_from_stream(line, stream).await?,
        "CompileStoryboard" => CompileStoryboard::parse_from_stream(line, stream).await?,
        "CompileXIB" => CompileXIB::parse_from_stream(line, stream).await?,
        "PrecompileSwiftBridgingHeader" => {
            PrecompileSwiftBridgingHeader::parse_from_stream(line, stream).await?
        }
        "CopySwiftLibs" => CopySwiftLibs::parse_from_stream(line, stream).await?,
        "Ld" => Ld::parse_from_stream(line, stream).await?,
        "CpResource" => CopyResource::parse_from_stream(line, stream).await?,
        "CreateBuildDirectory" => CreateBuildDirectory::parse_from_stream(line, stream).await?,
        "GenerateDSYMFile" => GenerateDSYMFile::parse_from_stream(line, stream).await?,
        "LinkStoryboards" => vec![Step::LinkStoryboards(LinkStoryboards::new(line)?)],
        "MergeSwiftModule" => MergeSwiftModule::parse_from_stream(line, stream).await?,
        "EmitSwiftModule" => EmitSwiftModule::parse_from_stream(line, stream).await?,
        "PhaseScriptExecution" => ScriptExecution::parse_from_stream(line, stream).await?,
        "ProcessInfoPlistFile" => ProcessInfoPlistFile::parse_from_stream(line, stream).await?,
        "ProcessProductPackaging" => {
            if !line.contains("mobileprovision") {
                ProcessProductPackaging::parse_from_stream(line, stream).await?
            } else {
                return Ok(None);
            }
        }
        "Validate" => Validate::parse_from_stream(line, stream).await?,
        "**" if line.contains("BUILD SUCCEEDED") => vec![Step::BuildSucceed],
        "**" if line.contains("CLEAN SUCCEEDED") => vec![Step::CleanSucceed],
        "**" if line.contains("TEST SUCCEEDED") => vec![Step::TestSucceed],
        "**" if line.contains("TEST FAILED") => vec![Step::TestFailed],
        "**" if line.contains("BUILD FAILED") => {
            BuildFailed::parse_from_stream(line, stream).await?
        }
        "Note" | "note:" => {
            if line.eq("Using new build system") {
                return Ok(None);
            } else if line.contains("suppress this warning") {
                return Ok(None);
            } else if line.eq("Planning") {
                return Ok(None);
            } else if line.ne("Build preparation complete") {
                vec![Step::Note(line)]
            } else {
                return Ok(None);
            }
        }
        "warning:" => {
            if line.contains("ONLY_ACTIVE_ARCH=YES")
                || line.contains("Building targets in manual order is deprecated")
            {
                return Ok(None);
            }
            let warn = Step::Warning(line);
            consume_till_empty_line(stream).await;
            vec![warn]
        }
        "error:" => {
            if !line.is_empty() {
                let error = Step::Error(line);
                consume_till_empty_line(stream).await;
                vec![error]
            } else {
                return Ok(None);
            }
        }
        _ => {
            #[cfg(feature = "with_tracing")]
            tracing::error!("Skipping: {cmd}");
            consume_till_empty_line(stream).await;
            return Ok(None);
        }
    }
    .pipe(Some)
    .pipe(Ok)
}

pub fn is_failure(steps: &Vec<Step>) -> anyhow::Result<bool> {
    let exit = steps
        .iter()
        .find(|&m| m.is_exit())
        .ok_or_else(|| anyhow::anyhow!("Exit not found!"))?
        .as_exit()
        .unwrap();
    Ok(exit != &0)
}

pub fn is_success(steps: &Vec<Step>) -> anyhow::Result<bool> {
    let exit = steps
        .iter()
        .find(|&m| m.is_exit())
        .ok_or_else(|| anyhow::anyhow!("Exit not found!"))?
        .as_exit()
        .unwrap();
    Ok(exit == &0)
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_case_1() {
    let root = "/Users/tami5/repos/swift/wordle";
    use crate::runner::spawn;
    use process_stream::StreamExt;

    let mut stream = spawn(
        root,
        &[
            "build",
            "-configuration",
            "Debug",
            "-target",
            "Wordle",
            "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_wordle/Wordle_Debug",
            "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_wordle/Wordle_Debug",
            "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_wordle/Wordle_Debug",
        ],
    )
    .await
    .unwrap();

    while let Some(step) = StreamExt::next(&mut stream).await {
        println!("{}", step)
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_case_2() {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    use crate::runner::spawn;
    use process_stream::StreamExt;

    let mut stream = spawn(
        root,
        &["build", "-configuration", "Debug", "-target", "YabaiMaster", "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug", "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug", "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
        ],
    )
    .await
    .unwrap();

    while let Some(step) = StreamExt::next(&mut stream).await {
        println!("{}", step)
    }
}

// [CompileError]
// [CompileError]  Cannot find 'flashcardRotation' in scope (/Users/tami5/repos/swift/wordle/Source/Animations/Flipcard.swift:29:34)
// [CompileError]        .rotation3DEffect(.degrees(flashcardRotation), axis: (x: 1, y: 0, z: 0))
// [CompileError]                                   ^~~~~~~~~~~~~~~~~
// [CompileError]
// [CompileError]  Cannot find 'flashcardRotation' in scope (/Users/tami5/repos/swift/wordle/Source/Animations/Flipcard.swift:35:7)
// [CompileError]        flashcardRotation += -180
// [CompileError]        ^~~~~~~~~~~~~~~~~
// [CompileError]
