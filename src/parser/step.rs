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
    BuildFailed,
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
    Exit(String),
    /// Register Launch Services
    RegisterWithLaunchServices(RegisterWithLaunchServices),
}
