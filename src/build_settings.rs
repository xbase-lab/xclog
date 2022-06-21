use process_stream::{Process, StreamExt};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::Stdio,
};

use anyhow::Result;
use tap::Pipe;

/// Subset of build settings
#[derive(Debug, Default)]
pub struct XCBuildSettings {
    /// Whether adhoc Code Signing Allowed
    pub ad_hoc_code_signing_allowed: bool,
    /// Whether target platform is specified
    pub allow_target_platform_specialization: bool,
    /// Whether there is alternate owner
    pub alternate_owner: bool,
    /// Whether to always search user paths
    pub always_search_user_paths: bool,
    /// Whether to always use seprate headermaps
    pub always_use_separate_headermaps: bool,
    /// Whether the application extension is api only
    pub application_extension_api_only: bool,
    /// Whether to apply rules in copy files
    pub apply_rules_in_copy_files: bool,
    /// Whether to apply rules in header files
    pub apply_rules_in_copy_headers: bool,
    /// build architecture
    pub archs: Vec<String>,
    /// Whether to build active resources only
    pub build_active_resources_only: bool,
    /// Path to build directory
    pub build_dir: PathBuf,
    /// Whether it is a build library for distribution
    pub build_library_for_distribution: bool,
    /// Path to build root
    pub build_root: PathBuf,
    /// Path to build product directory
    pub built_products_dir: PathBuf,
    /// Path to cache root
    pub cache_root: PathBuf,
    /// Path to class files directory
    pub class_file_dir: PathBuf,
    /// Whether to clean
    pub clean_precomps: bool,
    /// Whether to colone headers
    pub clone_headers: bool,
    /// Path to codesigning folder
    pub codesigning_folder_path: PathBuf,
    /// Whether code signing is allowed
    pub code_signing_allowed: bool,
    /// Whether code signing is required
    pub code_signing_required: bool,
    /// Whether code signing identiry
    pub code_sign_identity: String,
    /// Whether to inject entitlements with code signing
    pub code_sign_inject_base_entitlements: bool,
    /// Whether to use colors in diagnostics
    pub color_diagnostics: bool,
    /// Path to composite sdk directory
    pub composite_sdk_dirs: PathBuf,
    /// Whether to compress png files
    pub compress_png_files: bool,
    /// Name of Build configuration
    pub configuration: String,
    /// Path to build configuration directory
    pub configuration_build_dir: PathBuf,
    /// Path to tmp configuration directory
    pub configuration_temp_dir: PathBuf,
    /// Path to tmp configuration directory
    pub copying_preserves_hfs_data: bool,
    /// Whether to strip in copy phase
    pub copy_phase_strip: bool,
    /// Whether to copy resources from static frameworks
    pub copy_resources_from_static_frameworks: bool,
    /// Path to device platform directory
    pub corresponding_device_platform_dir: PathBuf,
    /// Name of device platform
    pub corresponding_device_platform_name: String,
    /// Path to device platform sdk directory
    pub corresponding_device_sdk_dir: PathBuf,
    /// Name of device sdk
    pub corresponding_device_sdk_name: String,
    /// Whether to include info plist in binary
    pub create_infoplist_section_in_binary: bool,
    /// Debug information format
    pub debug_information_format: String,
    /// Whether deployment_ ocation is specified
    pub deployment_location: bool,
    /// Path to derived directory
    pub derived_files_dir: PathBuf,
    /// Path to derived directory
    pub derived_file_dir: PathBuf,
    /// Path to derived sources directory
    pub derived_sources_dir: PathBuf,
    /// Path to developer applications directory
    pub developer_applications_dir: PathBuf,
    /// Path to developer bind directory
    pub developer_bin_dir: PathBuf,
    /// Path to developer directory
    pub developer_dir: PathBuf,
    /// Name of development team
    pub development_team: String,
    /// Whether manual target order build warning is disabled
    pub disable_manual_target_order_build_warning: bool,
    /// Path to documentation directory
    pub documentation_folder_path: String,
    /// Path to destination root
    pub dstroot: PathBuf,
    /// DWARF dysm file name
    pub dwarf_dsym_file_name: String,
    /// Whether DWARF should include product
    pub dwarf_dsym_file_should_accompany_product: bool,
    /// DWARF dysm folder path
    pub dwarf_dsym_folder_path: PathBuf,
    /// Platform name
    pub effective_platform_name: String,
    /// Whether sandbox is enabled
    pub enable_app_sandbox: bool,
    /// Whether bitcode is enabled
    pub enable_bitcode: bool,
    /// Whether testability is enabled
    pub enable_testability: bool,
    /// Whether searching path of tests is enabled
    pub enable_testing_search_paths: bool,
    /// Whether entitlements is required
    pub entitlements_required: bool,
    /// Path to executable folder
    pub executable_folder_path: String,
    /// Name of executable
    pub executable_name: String,
    /// Path to executable file
    pub executable_path: String,
    /// Path to file list
    pub file_list: PathBuf,
    /// Full product name
    pub full_product_name: String,
    /// Path to modulemap directory
    pub generated_modulemap_dir: PathBuf,
    /// info list file
    pub infoplist_file: String,
    /// Path to infolist directory
    pub infoplist_path: PathBuf,
    /// Installation owner
    pub install_owner: String,
    /// Path to install file
    pub install_path: PathBuf,
    /// Path to install root
    pub install_root: PathBuf,
    /// Path to file specifying ld linking dependency
    pub ld_dependency_info_file: PathBuf,
    /// Path to metal output directory
    pub metal_library_output_dir: PathBuf,
    /// Product handle identifier
    pub product_bundle_identifier: String,
    /// Product module name
    pub product_module_name: String,
    /// Product name
    pub product_name: String,
    /// Path to Product settings file
    pub product_settings_path: PathBuf,
    /// Product type
    pub product_type: String,
    /// Project Name
    pub project: String,
    /// Project directory
    pub project_dir: PathBuf,
    /// Project file path
    pub project_file_path: PathBuf,
    /// Platform display name
    pub platform_display_name: String,
    /// Path to SDK
    pub sdkroot: PathBuf,
    /// Path to directory
    pub sdk_dir: PathBuf,
    /// SDK version
    pub sdk_version: String,
    /// Path to `symroot`
    pub symroot: PathBuf,
    /// Wrapper name
    pub wrapper_name: String,
}

impl XCBuildSettings {
    /// Generate Build Settings from given root and build arguments
    pub async fn new<P, I, S>(root: P, args: I) -> Result<XCBuildSettings>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<std::ffi::OsStr> + Send,
    {
        let mut process = Process::new("/usr/bin/xcodebuild");

        process.current_dir(root);

        process.args(args);
        process.arg("-showBuildSettings");

        let output = process
            .spawn_and_stream()?
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>();

        Self::generate_from_lines(output)
        // if output.status.success() {
        //     #[allow(clippy::single_char_pattern)]
        //     Self::generate_from_lines(String::from_utf8(output.stdout)?.split("\n"))
        // } else {
        //     anyhow::bail!(String::from_utf8(output.stderr)?)
        // }
    }

    /// Generate Build Settings from given root and build arguments
    pub fn new_sync<P, I, S>(root: P, args: I) -> Result<XCBuildSettings>
    where
        P: AsRef<Path> + Send,
        I: IntoIterator<Item = S> + Send,
        S: AsRef<std::ffi::OsStr> + Send,
    {
        let mut process = std::process::Command::new("/usr/bin/xcodebuild");

        process.current_dir(root);

        process.args(args);
        process.arg("-showBuildSettings");

        process.stdin(Stdio::null());
        process.stdout(Stdio::piped());
        process.stderr(Stdio::null());
        let output = &mut Default::default();

        process.spawn()?.stdout.unwrap().read_to_string(output)?;

        Self::generate_from_lines(output.split("\n").map(ToString::to_string).collect())
    }

    /// Get path to output directory
    pub fn path_to_output_folder(&self) -> Result<&Path> {
        let Self {
            codesigning_folder_path: codesign_folder,
            metal_library_output_dir: library_output,
            ..
        } = self;

        if codesign_folder.exists() {
            codesign_folder
        } else if let Some(contents) = library_output.parent() {
            if let Some(folder_path) = contents.parent() {
                folder_path
            } else {
                anyhow::bail!(
                    "Unable to get folder_path from {codesign_folder:?} or {library_output:?}"
                );
            }
        } else {
            anyhow::bail!(
                "Unable to get folder_path from {codesign_folder:?} or {library_output:?}"
            );
        }
        .pipe(Ok)
    }

    /// Get path to output binaray
    pub fn path_to_output_binary(&self) -> Result<PathBuf> {
        let mut app_folder = self.path_to_output_folder()?.to_path_buf();
        app_folder.extend(self.executable_path.split('/').skip(1));
        app_folder.pipe(Ok)
    }

    fn generate_from_lines(lines: Vec<String>) -> Result<XCBuildSettings> {
        let mut data = Self::default();

        for line in lines {
            if line.contains("Build settings for action build and target") {
                continue;
            }
            let line = line.trim();
            let mut parts = line.split(" = ");
            let key = parts.next();
            let value = parts.next();
            if key.is_none() || value.is_none() {
                continue;
            }
            let (key, value) = (key.unwrap().trim(), value.unwrap().trim());
            match key {
                "AD_HOC_CODE_SIGNING_ALLOWED" => {
                    data.ad_hoc_code_signing_allowed = yes_no_bool(value);
                }
                "ALLOW_TARGET_PLATFORM_SPECIALIZATION" => {
                    data.allow_target_platform_specialization = yes_no_bool(value)
                }
                "ALTERNATE_OWNER" => data.alternate_owner = yes_no_bool(value),
                "ALWAYS_SEARCH_USER_PATHS" => data.always_search_user_paths = yes_no_bool(value),
                "ALWAYS_USE_SEPARATE_HEADERMAPS" => {
                    data.always_use_separate_headermaps = yes_no_bool(value)
                }
                "APPLICATION_EXTENSION_API_ONLY" => {
                    data.application_extension_api_only = yes_no_bool(value)
                }
                "APPLY_RULES_IN_COPY_FILES" => data.apply_rules_in_copy_files = yes_no_bool(value),
                "APPLY_RULES_IN_COPY_HEADERS" => {
                    data.apply_rules_in_copy_headers = yes_no_bool(value)
                }
                "ARCHS" => data.archs = value.split_whitespace().map(ToString::to_string).collect(),
                "BUILD_ACTIVE_RESOURCES_ONLY" => {
                    data.build_active_resources_only = yes_no_bool(value)
                }
                "BUILD_DIR" => data.build_dir = value.into(),
                "BUILD_LIBRARY_FOR_DISTRIBUTION" => {
                    data.build_library_for_distribution = yes_no_bool(value)
                }
                "BUILD_ROOT" => data.build_root = value.into(),
                "PLATFORM_DISPLAY_NAME" => data.platform_display_name = value.into(),
                "BUILT_PRODUCTS_DIR" => data.built_products_dir = value.into(),
                "CACHE_ROOT" => data.cache_root = value.into(),
                "CLASS_FILE_DIR" => data.class_file_dir = value.into(),
                "CLEAN_PRECOMPS" => data.clean_precomps = yes_no_bool(value),
                "CLONE_HEADERS" => data.clone_headers = yes_no_bool(value),
                "CODESIGNING_FOLDER_PATH" => data.codesigning_folder_path = value.into(),
                "CODE_SIGNING_ALLOWED" => data.code_signing_allowed = yes_no_bool(value),
                "CODE_SIGNING_REQUIRED" => data.code_signing_required = yes_no_bool(value),
                "CODE_SIGN_IDENTITY" => data.code_sign_identity = value.to_string(),
                "CODE_SIGN_INJECT_BASE_ENTITLEMENTS" => {
                    data.code_sign_inject_base_entitlements = yes_no_bool(value)
                }
                "COLOR_DIAGNOSTICS" => data.color_diagnostics = yes_no_bool(value),
                "COMPOSITE_SDK_DIRS" => data.composite_sdk_dirs = value.into(),
                "COMPRESS_PNG_FILES" => data.compress_png_files = yes_no_bool(value),
                "CONFIGURATION" => data.configuration = value.to_string(),
                "CONFIGURATION_BUILD_DIR" => data.configuration_build_dir = value.into(),
                "CONFIGURATION_TEMP_DIR" => data.configuration_temp_dir = value.into(),
                "COPYING_PRESERVES_HFS_DATA" => {
                    data.copying_preserves_hfs_data = yes_no_bool(value)
                }
                "COPY_PHASE_STRIP" => data.copy_phase_strip = yes_no_bool(value),
                "COPY_RESOURCES_FROM_STATIC_FRAMEWORKS" => {
                    data.copy_resources_from_static_frameworks = yes_no_bool(value)
                }
                "CORRESPONDING_DEVICE_PLATFORM_DIR" => {
                    data.corresponding_device_platform_dir = value.into()
                }
                "CORRESPONDING_DEVICE_PLATFORM_NAME" => {
                    data.corresponding_device_platform_name = value.to_string()
                }
                "CORRESPONDING_DEVICE_SDK_DIR" => data.corresponding_device_sdk_dir = value.into(),
                "CORRESPONDING_DEVICE_SDK_NAME" => {
                    data.corresponding_device_sdk_name = value.to_string()
                }
                "CREATE_INFOPLIST_SECTION_IN_BINARY" => {
                    data.create_infoplist_section_in_binary = yes_no_bool(value)
                }
                "DEBUG_INFORMATION_FORMAT" => data.debug_information_format = value.to_string(),
                "DEPLOYMENT_LOCATION" => data.deployment_location = yes_no_bool(value),
                "DERIVED_FILES_DIR" => data.derived_files_dir = value.into(),
                "DERIVED_FILE_DIR" => data.derived_file_dir = value.into(),
                "DERIVED_SOURCES_DIR" => data.derived_sources_dir = value.into(),
                "DEVELOPER_APPLICATIONS_DIR" => data.developer_applications_dir = value.into(),
                "DEVELOPER_BIN_DIR" => data.developer_bin_dir = value.into(),
                "DEVELOPER_DIR" => data.developer_dir = value.into(),
                "DEVELOPMENT_TEAM" => data.development_team = value.to_string(),
                "DISABLE_MANUAL_TARGET_ORDER_BUILD_WARNING" => {
                    data.disable_manual_target_order_build_warning = yes_no_bool(value)
                }
                "DOCUMENTATION_FOLDER_PATH" => data.documentation_folder_path = value.to_string(),
                "DSTROOT" => data.dstroot = value.into(),
                "DWARF_DSYM_FILE_NAME" => data.dwarf_dsym_file_name = value.to_string(),
                "DWARF_DSYM_FILE_SHOULD_ACCOMPANY_PRODUCT" => {
                    data.dwarf_dsym_file_should_accompany_product = yes_no_bool(value)
                }
                "DWARF_DSYM_FOLDER_PATH" => data.dwarf_dsym_folder_path = value.into(),
                "EFFECTIVE_PLATFORM_NAME" => data.effective_platform_name = value.to_string(),
                "ENABLE_APP_SANDBOX" => data.enable_app_sandbox = yes_no_bool(value),
                "ENABLE_BITCODE" => data.enable_bitcode = yes_no_bool(value),
                "ENABLE_TESTABILITY" => data.enable_testability = yes_no_bool(value),
                "ENABLE_TESTING_SEARCH_PATHS" => {
                    data.enable_testing_search_paths = yes_no_bool(value)
                }
                "ENTITLEMENTS_REQUIRED" => data.entitlements_required = yes_no_bool(value),
                "EXECUTABLE_FOLDER_PATH" => data.executable_folder_path = value.to_string(),
                "EXECUTABLE_NAME" => data.executable_name = value.to_string(),
                "EXECUTABLE_PATH" => data.executable_path = value.to_string(),
                "FILE_LIST" => data.file_list = value.into(),
                "FULL_PRODUCT_NAME" => data.full_product_name = value.to_string(),
                "GENERATED_MODULEMAP_DIR" => data.generated_modulemap_dir = value.into(),
                "INFOPLIST_FILE" => data.infoplist_file = value.to_string(),
                "INFOPLIST_PATH" => data.infoplist_path = value.into(),
                "INSTALL_OWNER" => data.install_owner = value.to_string(),
                "INSTALL_PATH" => data.install_path = value.into(),
                "INSTALL_ROOT" => data.install_root = value.into(),
                "LD_DEPENDENCY_INFO_FILE" => data.ld_dependency_info_file = value.into(),
                "METAL_LIBRARY_OUTPUT_DIR" => data.metal_library_output_dir = value.into(),
                "PRODUCT_BUNDLE_IDENTIFIER" => data.product_bundle_identifier = value.to_string(),
                "PRODUCT_MODULE_NAME" => data.product_module_name = value.to_string(),
                "PRODUCT_NAME" => data.product_name = value.to_string(),
                "PRODUCT_SETTINGS_PATH" => data.product_settings_path = value.into(),
                "PRODUCT_TYPE" => data.product_type = value.to_string(),
                "PROJECT" => data.project = value.to_string(),
                "PROJECT_DIR" => data.project_dir = value.into(),
                "PROJECT_FILE_PATH" => data.project_file_path = value.into(),
                "SDKROOT" => data.sdkroot = value.into(),
                "SDK_DIR" => data.sdk_dir = value.into(),
                "SDK_VERSION" => data.sdk_version = value.to_string(),
                "SYMROOT" => data.symroot = value.into(),
                "WRAPPER_NAME" => data.wrapper_name = value.to_string(),
                _ => continue,
            }
        }

        Ok(data)
    }
}

fn yes_no_bool(value: &str) -> bool {
    match value {
        "No" => false,
        "Yes" => true,
        _ => false,
    }
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = "local"]
async fn does() {
    let root = "/Users/tami5/repos/swift/yabaimaster";
    let args = &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
        ];
    let build_settings = XCBuildSettings::new(root, args).await.unwrap();
    println!("{:#?}", build_settings);
}
