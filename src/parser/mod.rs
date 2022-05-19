mod build_settings;
mod description;
mod error;
mod outputs;
mod step;
mod util;

use async_trait::async_trait;
use tap::Pipe;

pub use build_settings::*;
pub use description::Description;
pub use error::Error;
pub use outputs::*;
pub use step::Step;
pub use util::*;

use crate::runner::ProcessUpdate;

pub type OutputStream = dyn tokio_stream::Stream<Item = ProcessUpdate> + Unpin + Send;

#[async_trait]
pub trait ParsableFromStream {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error>
    where
        Self: Sized + Send;
}

pub async fn parse_step_from_stream(
    line: String,
    stream: &mut OutputStream,
) -> Result<Option<Step>, Error> {
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
            (cmd.to_string(), chunks.as_str().to_string())
        }
        None => return Err(Error::Failure("Empty Line, couldn't identity step".into())),
    };

    match cmd.as_str() {
        "Command" => Invocation::parse_from_stream(line, stream)
            .await
            .map(Step::Invocation),
        "RegisterWithLaunchServices" => RegisterWithLaunchServices::parse_from_stream(line, stream)
            .await
            .map(Step::RegisterWithLaunchServices),
        "Resolved" if line.contains("source packages") => {
            ResolvedSourcePackages::parse_from_stream(line, stream)
                .await
                .map(Step::ResolvedSourcePackages)
        }
        "CompileSwift" => CompileSwift::parse_from_stream(line, stream)
            .await
            .map(Step::CompileSwift),
        "CompileSwiftSources" => CompileSwiftSources::parse_from_stream(line, stream)
            .await
            .map(Step::CompileSwiftSources),
        "CompileC" => CompileC::parse_from_stream(line, stream)
            .await
            .map(Step::CompileC),
        "CodeSign" => CodeSign::parse_from_stream(line, stream)
            .await
            .map(Step::CodeSign),
        "CompileAssetCatalog" => CompileAssetCatalog::parse_from_stream(line, stream)
            .await
            .map(Step::CompileAssetCatalog),
        "CompileStoryboard" => CompileStoryboard::parse_from_stream(line, stream)
            .await
            .map(Step::CompileStoryboard),
        "CompileXIB" => CompileXIB::parse_from_stream(line, stream)
            .await
            .map(Step::CompileXIB),
        "PrecompileSwiftBridgingHeader" => {
            PrecompileSwiftBridgingHeader::parse_from_stream(line, stream)
                .await
                .map(Step::PrecompileSwiftBridgingHeader)
        }
        "CopySwiftLibs" => CopySwiftLibs::parse_from_stream(line, stream)
            .await
            .map(Step::CopySwiftLibs),
        "Ld" => Ld::parse_from_stream(line, stream).await.map(Step::Ld),
        "CpResource" => CopyResource::parse_from_stream(line, stream)
            .await
            .map(Step::CopyResource),
        "CreateBuildDirectory" => CreateBuildDirectory::parse_from_stream(line, stream)
            .await
            .map(Step::CreateBuildDirectory),
        "GenerateDSYMFile" => GenerateDSYMFile::parse_from_stream(line, stream)
            .await
            .map(Step::GenerateDSYMFile),
        "LinkStoryboards" => LinkStoryboards::new(line).map(Step::LinkStoryboards),
        "MergeSwiftModule" => MergeSwiftModule::parse_from_stream(line, stream)
            .await
            .map(Step::MergeSwiftModule),
        "EmitSwiftModule" => EmitSwiftModule::parse_from_stream(line, stream)
            .await
            .map(Step::EmitSwiftModule),
        "PhaseScriptExecution" => ScriptExecution::parse_from_stream(line, stream)
            .await
            .map(Step::ScriptExecution),
        "ProcessInfoPlistFile" => ProcessInfoPlistFile::parse_from_stream(line, stream)
            .await
            .map(Step::ProcessInfoPlistFile),
        "ProcessProductPackaging" => {
            if !line.contains("mobileprovision") {
                ProcessProductPackaging::parse_from_stream(line, stream)
                    .await
                    .map(Step::ProcessProductPackaging)
            } else {
                return Ok(None);
            }
        }
        "Validate" => Validate::parse_from_stream(line, stream)
            .await
            .map(Step::Validate),
        "**" if line.contains("BUILD SUCCEEDED") => Step::BuildSucceed.pipe(Ok),
        "**" if line.contains("BUILD FAILED") => Step::BuildFailed.pipe(Ok),
        "**" if line.contains("CLEAN SUCCEEDED") => Step::CleanSucceed.pipe(Ok),
        "**" if line.contains("TEST SUCCEEDED") => Step::TestSucceed.pipe(Ok),
        "**" if line.contains("TEST FAILED") => Step::TestFailed.pipe(Ok),
        "Note" | "note:" => {
            if line.eq("Planning") {
                Step::Planning.pipe(Ok)
            } else if line.eq("Using new build system") {
                return Ok(None);
            } else if line.contains("suppress this warning") {
                return Ok(None);
            } else if line.ne("Build preparation complete") {
                Step::Note(line).pipe(Ok)
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
            let warn = Step::Warning(line).pipe(Ok);
            consume_till_empty_line(stream).await;
            warn
        }
        "error:" => {
            let warn = Step::Error(line).pipe(Ok);
            consume_till_empty_line(stream).await;
            warn
        }
        cmd => {
            #[cfg(feature = "tracing")]
            tracing::error!("Skipping: {cmd}");
            consume_till_empty_line(stream).await;
            return Ok(None);
        }
    }
    .map(Some)
}

#[tokio::test]
#[tracing_test::traced_test]
async fn spawn_and_parse() {
    let root = "/Users/tami5/repos/swift/wordle";
    use crate::runner::spawn;
    use futures::StreamExt;

    // spawn_once(root, &["clean"]).await.unwrap();

    let mut stream = spawn(
        root,
        &[
            "clean",
            "build",
            "-configuration",
            "Debug",
            "-target",
            "Wordle",
            "-sdk",
            "iphonesimulator",
        ],
    )
    .await
    .unwrap();

    while let Some(step) = StreamExt::next(&mut stream).await {
        println!("{:#?}", step)
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn libc() {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    use crate::runner::{spawn, spawn_once};
    use futures::StreamExt;

    spawn_once(root, &["clean"]).await.unwrap();

    let mut stream = spawn(
        root,
        &[
            "-configuration",
            "Release",
            "-arch",
            "arm64",
            r#"SYMROOT=/Users/tami5/repos/swift/yabaimaster/build"#,
        ],
    )
    .await
    .unwrap();

    while let Some(step) = StreamExt::next(&mut stream).await {
        println!("{}", step)
    }
}
