use super::*;
mod fmt;

#[derive(Debug)]
pub enum Step {
    /// Command invocated
    Invocation(Invocation),
    /// Swift file compilation step
    CompileSwift(CompileSwift),
    /// Aggregated swift files compilation
    CompileSwiftSources(CompileSwiftSources),
    /// Clang compilation step
    CompileC(CompileC),
    /// Code Sign Step
    CodeSign(CodeSign),
    /// Asset's catalog compilation
    CompileAssetCatalog(CompileAssetCatalog),
    /// Storyboard compilation
    CompileStoryboard(CompileStoryboard),
    /// Xib file compilation
    CompileXIB(CompileXIB),
    /// Linking of a library
    Ld(Ld),
    /// Swift Runtime was copied
    CopySwiftLibs(CopySwiftLibs),
    /// Resource file was copied
    CopyResource(CopyResource),
    /// Build Directory Creation Step
    CreateBuildDirectory(CreateBuildDirectory),
    /// DSM File Generation Step
    GenerateDSYMFile(GenerateDSYMFile),
    /// Storyboard linked
    LinkStoryboards(LinkStoryboards),
    /// Build phase shell script execution
    ScriptExecution(ScriptExecution),
    /// Precompile Bridging header
    PrecompileSwiftBridgingHeader(PrecompileSwiftBridgingHeader),
    /// Info plist process step
    ProcessInfoPlistFile(ProcessInfoPlistFile),
    /// Packaging step
    ProcessProductPackaging(ProcessProductPackaging),
    /// Packaging step
    ResolvedSourcePackages(ResolvedSourcePackages),
    /// Validate app
    Validate(Validate),
    /// TODO: Libtool was used to create a static library
    LibTool,
    /// General Note emitted
    Note(String),
    /// Planning phase
    Planning,
    /// Swift Module was merged
    MergeSwiftModule(MergeSwiftModule),
    /// Swift Module was emitted
    EmitSwiftModule(EmitSwiftModule),
    /// Build Succeeded
    BuildSucceed,
    /// Build Failed
    BuildFailed(BuildFailed),
    /// Clean Succeeded
    CleanSucceed,
    /// Test Succeeded
    TestSucceed,
    /// Test Failed
    TestFailed,
    /// Error from stderr or error:
    Error(String),
    /// Error from warning:
    Warning(String),
    /// Exit Code
    Exit(i32),
    /// Register Launch Services
    RegisterWithLaunchServices(RegisterWithLaunchServices),
}

impl Step {
    pub fn as_invocation(&self) -> Option<&Invocation> {
        if let Self::Invocation(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Invocation`].
    ///
    /// [`Invocation`]: Step::Invocation
    #[must_use]
    pub fn is_invocation(&self) -> bool {
        matches!(self, Self::Invocation(..))
    }

    pub fn as_compile_swift(&self) -> Option<&CompileSwift> {
        if let Self::CompileSwift(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CompileSwift`].
    ///
    /// [`CompileSwift`]: Step::CompileSwift
    #[must_use]
    pub fn is_compile_swift(&self) -> bool {
        matches!(self, Self::CompileSwift(..))
    }

    pub fn as_compile_swift_sources(&self) -> Option<&CompileSwiftSources> {
        if let Self::CompileSwiftSources(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CompileSwiftSources`].
    ///
    /// [`CompileSwiftSources`]: Step::CompileSwiftSources
    #[must_use]
    pub fn is_compile_swift_sources(&self) -> bool {
        matches!(self, Self::CompileSwiftSources(..))
    }

    pub fn as_compile_c(&self) -> Option<&CompileC> {
        if let Self::CompileC(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CompileC`].
    ///
    /// [`CompileC`]: Step::CompileC
    #[must_use]
    pub fn is_compile_c(&self) -> bool {
        matches!(self, Self::CompileC(..))
    }

    pub fn as_code_sign(&self) -> Option<&CodeSign> {
        if let Self::CodeSign(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CodeSign`].
    ///
    /// [`CodeSign`]: Step::CodeSign
    #[must_use]
    pub fn is_code_sign(&self) -> bool {
        matches!(self, Self::CodeSign(..))
    }

    /// Returns `true` if the step is [`CompileStoryboard`].
    ///
    /// [`CompileStoryboard`]: Step::CompileStoryboard
    #[must_use]
    pub fn is_compile_storyboard(&self) -> bool {
        matches!(self, Self::CompileStoryboard(..))
    }

    pub fn as_compile_storyboard(&self) -> Option<&CompileStoryboard> {
        if let Self::CompileStoryboard(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CompileXIB`].
    ///
    /// [`CompileXIB`]: Step::CompileXIB
    #[must_use]
    pub fn is_compile_xib(&self) -> bool {
        matches!(self, Self::CompileXIB(..))
    }

    pub fn as_compile_xib(&self) -> Option<&CompileXIB> {
        if let Self::CompileXIB(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Ld`].
    ///
    /// [`Ld`]: Step::Ld
    #[must_use]
    pub fn is_ld(&self) -> bool {
        matches!(self, Self::Ld(..))
    }

    pub fn as_ld(&self) -> Option<&Ld> {
        if let Self::Ld(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CopySwiftLibs`].
    ///
    /// [`CopySwiftLibs`]: Step::CopySwiftLibs
    #[must_use]
    pub fn is_copy_swift_libs(&self) -> bool {
        matches!(self, Self::CopySwiftLibs(..))
    }

    pub fn as_copy_swift_libs(&self) -> Option<&CopySwiftLibs> {
        if let Self::CopySwiftLibs(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CopyResource`].
    ///
    /// [`CopyResource`]: Step::CopyResource
    #[must_use]
    pub fn is_copy_resource(&self) -> bool {
        matches!(self, Self::CopyResource(..))
    }

    pub fn as_copy_resource(&self) -> Option<&CopyResource> {
        if let Self::CopyResource(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`CreateBuildDirectory`].
    ///
    /// [`CreateBuildDirectory`]: Step::CreateBuildDirectory
    #[must_use]
    pub fn is_create_build_directory(&self) -> bool {
        matches!(self, Self::CreateBuildDirectory(..))
    }

    pub fn as_create_build_directory(&self) -> Option<&CreateBuildDirectory> {
        if let Self::CreateBuildDirectory(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`GenerateDSYMFile`].
    ///
    /// [`GenerateDSYMFile`]: Step::GenerateDSYMFile
    #[must_use]
    pub fn is_generate_dsymfile(&self) -> bool {
        matches!(self, Self::GenerateDSYMFile(..))
    }

    pub fn as_link_storyboards(&self) -> Option<&LinkStoryboards> {
        if let Self::LinkStoryboards(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`LinkStoryboards`].
    ///
    /// [`LinkStoryboards`]: Step::LinkStoryboards
    #[must_use]
    pub fn is_link_storyboards(&self) -> bool {
        matches!(self, Self::LinkStoryboards(..))
    }

    pub fn as_script_execution(&self) -> Option<&ScriptExecution> {
        if let Self::ScriptExecution(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`ScriptExecution`].
    ///
    /// [`ScriptExecution`]: Step::ScriptExecution
    #[must_use]
    pub fn is_script_execution(&self) -> bool {
        matches!(self, Self::ScriptExecution(..))
    }

    /// Returns `true` if the step is [`PrecompileSwiftBridgingHeader`].
    ///
    /// [`PrecompileSwiftBridgingHeader`]: Step::PrecompileSwiftBridgingHeader
    #[must_use]
    pub fn is_precompile_swift_bridging_header(&self) -> bool {
        matches!(self, Self::PrecompileSwiftBridgingHeader(..))
    }

    pub fn as_precompile_swift_bridging_header(&self) -> Option<&PrecompileSwiftBridgingHeader> {
        if let Self::PrecompileSwiftBridgingHeader(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`ProcessInfoPlistFile`].
    ///
    /// [`ProcessInfoPlistFile`]: Step::ProcessInfoPlistFile
    #[must_use]
    pub fn is_process_info_plist_file(&self) -> bool {
        matches!(self, Self::ProcessInfoPlistFile(..))
    }

    pub fn as_process_info_plist_file(&self) -> Option<&ProcessInfoPlistFile> {
        if let Self::ProcessInfoPlistFile(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`ProcessProductPackaging`].
    ///
    /// [`ProcessProductPackaging`]: Step::ProcessProductPackaging
    #[must_use]
    pub fn is_process_product_packaging(&self) -> bool {
        matches!(self, Self::ProcessProductPackaging(..))
    }

    pub fn as_process_product_packaging(&self) -> Option<&ProcessProductPackaging> {
        if let Self::ProcessProductPackaging(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`ResolvedSourcePackages`].
    ///
    /// [`ResolvedSourcePackages`]: Step::ResolvedSourcePackages
    #[must_use]
    pub fn is_resolved_source_packages(&self) -> bool {
        matches!(self, Self::ResolvedSourcePackages(..))
    }

    pub fn as_resolved_source_packages(&self) -> Option<&ResolvedSourcePackages> {
        if let Self::ResolvedSourcePackages(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Validate`].
    ///
    /// [`Validate`]: Step::Validate
    #[must_use]
    pub fn is_validate(&self) -> bool {
        matches!(self, Self::Validate(..))
    }

    pub fn as_validate(&self) -> Option<&Validate> {
        if let Self::Validate(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`LibTool`].
    ///
    /// [`LibTool`]: Step::LibTool
    #[must_use]
    pub fn is_lib_tool(&self) -> bool {
        matches!(self, Self::LibTool)
    }

    pub fn as_note(&self) -> Option<&String> {
        if let Self::Note(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Note`].
    ///
    /// [`Note`]: Step::Note
    #[must_use]
    pub fn is_note(&self) -> bool {
        matches!(self, Self::Note(..))
    }

    /// Returns `true` if the step is [`MergeSwiftModule`].
    ///
    /// [`MergeSwiftModule`]: Step::MergeSwiftModule
    #[must_use]
    pub fn is_merge_swift_module(&self) -> bool {
        matches!(self, Self::MergeSwiftModule(..))
    }

    pub fn as_merge_swift_module(&self) -> Option<&MergeSwiftModule> {
        if let Self::MergeSwiftModule(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`EmitSwiftModule`].
    ///
    /// [`EmitSwiftModule`]: Step::EmitSwiftModule
    #[must_use]
    pub fn is_emit_swift_module(&self) -> bool {
        matches!(self, Self::EmitSwiftModule(..))
    }

    pub fn as_emit_swift_module(&self) -> Option<&EmitSwiftModule> {
        if let Self::EmitSwiftModule(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is build successed
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::BuildSucceed)
            | matches!(self, Self::CleanSucceed)
            | matches!(self, Self::TestSucceed)
    }

    /// Returns `true` if the step is failed
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::TestFailed)
            | matches!(self, Self::TestSucceed)
            | matches!(self, Self::BuildFailed(_))
    }

    /// Returns `true` if the step is [`Exit`].
    ///
    /// [`Exit`]: Step::Exit
    #[must_use]
    pub fn is_exit(&self) -> bool {
        matches!(self, Self::Exit(..))
    }

    pub fn as_exit(&self) -> Option<&i32> {
        if let Self::Exit(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Warning`].
    ///
    /// [`Warning`]: Step::Warning
    #[must_use]
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning(..))
    }

    pub fn as_warning(&self) -> Option<&String> {
        if let Self::Warning(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the step is [`Error`].
    ///
    /// [`Error`]: Step::Error
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(..))
    }

    pub fn as_error(&self) -> Option<&String> {
        if let Self::Error(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
