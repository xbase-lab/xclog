use std::fmt::Display;

use super::Step;

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Invocation(v) => Display::fmt(v, f),
            Step::CompileSwift(v) => Display::fmt(v, f),
            Step::CompileSwiftSources(v) => Display::fmt(v, f),
            Step::CompileC(v) => Display::fmt(v, f),
            Step::CodeSign(v) => Display::fmt(v, f),
            Step::CompileAssetCatalog(v) => Display::fmt(v, f),
            Step::CompileStoryboard(v) => Display::fmt(v, f),
            Step::CompileXIB(v) => Display::fmt(v, f),
            Step::Ld(v) => Display::fmt(v, f),
            Step::CopySwiftLibs(v) => Display::fmt(v, f),
            Step::CopyResource(v) => std::fmt::Display::fmt(v, f),
            Step::CreateBuildDirectory(v) => Display::fmt(v, f),
            Step::GenerateDSYMFile(v) => Display::fmt(v, f),
            Step::LinkStoryboards(v) => Display::fmt(v, f),
            Step::ScriptExecution(v) => Display::fmt(v, f),
            Step::PrecompileSwiftBridgingHeader(v) => Display::fmt(v, f),
            Step::ProcessInfoPlistFile(v) => Display::fmt(v, f),
            Step::ProcessProductPackaging(v) => Display::fmt(v, f),
            Step::Validate(v) => Display::fmt(v, f),
            Step::Note(v) => write!(f, "[Note]  {v}"),
            Step::MergeSwiftModule(v) => Display::fmt(v, f),
            Step::EmitSwiftModule(v) => Display::fmt(v, f),
            Step::ResolvedSourcePackages(v) => Display::fmt(v, f),
            Step::Error(v) => {
                if !v.contains("plug-in") {
                    write!(f, "[Error]  {v}")
                } else {
                    Ok(())
                }
            }
            Step::Exit(_) => write!(
                f,
                "-------------------------------------------------------------------"
            ),
            Step::LibTool => write!(f, "LibTool"),
            Step::Planning => write!(f, "[Planning]"),
            Step::NewBuildSystem => write!(f, "[New Build System]"),
            Step::BuildSucceed => write!(f, "[Succeed]"),
            Step::BuildFailed => write!(f, "[Failed]"),
            Step::CleanSucceed => write!(f, "[Succeed]"),
            Step::TestSucceed => write!(f, "[Succeed]"),
            Step::TestFailed => write!(f, "[Failed]"),
        }
    }
}
