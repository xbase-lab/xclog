use std::path::PathBuf;

/// Subset of build settings
#[derive(Debug, Default)]
pub struct BuildSettings {
    pub ad_hoc_code_signing_allowed: bool,
    pub allow_target_platform_specialization: bool,
    pub alternate_owner: bool,
    pub always_search_user_paths: bool,
    pub always_use_separate_headermaps: bool,
    pub application_extension_api_only: bool,
    pub apply_rules_in_copy_files: bool,
    pub apply_rules_in_copy_headers: bool,
    pub archs: Vec<String>,
    pub build_active_resources_only: bool,
    pub build_dir: PathBuf,
    pub build_library_for_distribution: bool,
    pub build_root: PathBuf,
    pub built_products_dir: PathBuf,
    pub cache_root: PathBuf,
    pub class_file_dir: PathBuf,
    pub clean_precomps: bool,
    pub clone_headers: bool,
    pub codesigning_folder_path: PathBuf,
    pub code_signing_allowed: bool,
    pub code_signing_required: bool,
    pub code_sign_identity: String,
    pub code_sign_inject_base_entitlements: bool,
    pub color_diagnostics: bool,
    pub combine_hidpi_images: bool,
    pub composite_sdk_dirs: PathBuf,
    pub compress_png_files: bool,
    pub configuration: String,
    pub configuration_build_dir: PathBuf,
    pub configuration_temp_dir: PathBuf,
    pub contents_folder_path: String,
    pub contents_folder_path_shallow_bundle_no: String,
    pub contents_folder_path_shallow_bundle_yes: String,
    pub copying_preserves_hfs_data: bool,
    pub copy_headers_run_unifdef: bool,
    pub copy_phase_strip: bool,
    pub copy_resources_from_static_frameworks: bool,
    pub corresponding_device_platform_dir: PathBuf,
    pub corresponding_device_platform_name: String,
    pub corresponding_device_sdk_dir: PathBuf,
    pub corresponding_device_sdk_name: String,
    pub create_infoplist_section_in_binary: bool,
    pub debug_information_format: String,
    pub defines_module: bool,
    pub deployment_location: bool,
    pub derived_files_dir: PathBuf,
    pub derived_file_dir: PathBuf,
    pub derived_sources_dir: PathBuf,
    pub developer_applications_dir: PathBuf,
    pub developer_bin_dir: PathBuf,
    pub developer_dir: PathBuf,
    pub development_team: String,
    pub disable_manual_target_order_build_warning: bool,
    pub documentation_folder_path: String,
    pub dstroot: PathBuf,
    pub dwarf_dsym_file_name: String,
    pub dwarf_dsym_file_should_accompany_product: bool,
    pub dwarf_dsym_folder_path: PathBuf,
    pub effective_platform_name: String,
    pub enable_app_sandbox: bool,
    pub enable_bitcode: bool,
    pub enable_default_header_search_paths: bool,
    pub enable_default_search_paths: bool,
    pub enable_hardened_runtime: bool,
    pub enable_header_dependencies: bool,
    pub enable_on_demand_resources: bool,
    pub enable_strict_objc_msgsend: bool,
    pub enable_testability: bool,
    pub enable_testing_search_paths: bool,
    pub entitlements_required: bool,
    pub executable_folder_path: String,
    pub executable_folder_path_shallow_bundle_no: String,
    pub executable_folder_path_shallow_bundle_yes: String,
    pub executable_name: String,
    pub executable_path: String,
    pub file_list: PathBuf,
    pub full_product_name: String,
    pub generated_modulemap_dir: PathBuf,
    pub generate_infoplist_file: bool,
    pub generate_master_object_file: bool,
    pub generate_pkginfo_file: bool,
    pub generate_profiling_code: bool,
    pub generate_text_based_stubs: bool,
    pub infoplist_file: String,
    pub infoplist_path: String,
    pub infoplist_preprocess: bool,
    pub install_owner: String,
    pub install_path: PathBuf,
    pub install_root: PathBuf,
    pub ld_dependency_info_file: PathBuf,
    pub library_dext_install_path: PathBuf,
    pub library_kext_install_path: PathBuf,
    pub localization_export_supported: bool,
    pub metal_library_output_dir: PathBuf,
    pub no_common: bool,
    pub product_bundle_identifier: String,
    pub product_module_name: String,
    pub product_name: String,
    pub product_settings_path: PathBuf,
    pub product_type: String,
    pub project: String,
    pub project_dir: PathBuf,
    pub project_file_path: PathBuf,
    pub sdkroot: PathBuf,
    pub sdk_dir: PathBuf,
    pub sdk_version: String,
    pub shallow_bundle: bool,
    pub shallow_bundle_triple: String,
    pub symroot: PathBuf,
    pub version_info_builder: String,
    pub wrapper_name: String,
    pub platform_display_name: String,
}

impl BuildSettings {
    pub fn new(mut lines: std::str::Split<'_, &str>) -> anyhow::Result<BuildSettings> {
        let mut data = Self::default();
        while let Some(line) = lines.next() {
            if line.contains("Build settings for action build and target") {
                continue;
            } else {
                break;
            }
        }

        while let Some(line) = lines.next() {
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
                "COMBINE_HIDPI_IMAGES" => data.combine_hidpi_images = yes_no_bool(value),
                "COMPOSITE_SDK_DIRS" => data.composite_sdk_dirs = value.into(),
                "COMPRESS_PNG_FILES" => data.compress_png_files = yes_no_bool(value),
                "CONFIGURATION" => data.configuration = value.to_string(),
                "CONFIGURATION_BUILD_DIR" => data.configuration_build_dir = value.into(),
                "CONFIGURATION_TEMP_DIR" => data.configuration_temp_dir = value.into(),
                "CONTENTS_FOLDER_PATH" => data.contents_folder_path = value.into(),
                "CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_NO" => {
                    data.contents_folder_path_shallow_bundle_no = value.to_string()
                }
                "CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_YES" => {
                    data.contents_folder_path_shallow_bundle_yes = value.to_string()
                }
                "COPYING_PRESERVES_HFS_DATA" => {
                    data.copying_preserves_hfs_data = yes_no_bool(value)
                }
                "COPY_HEADERS_RUN_UNIFDEF" => data.copy_headers_run_unifdef = yes_no_bool(value),
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
                "DEFINES_MODULE" => data.defines_module = yes_no_bool(value),
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
                "ENABLE_DEFAULT_HEADER_SEARCH_PATHS" => {
                    data.enable_default_header_search_paths = yes_no_bool(value)
                }
                "ENABLE_DEFAULT_SEARCH_PATHS" => {
                    data.enable_default_search_paths = yes_no_bool(value)
                }
                "ENABLE_HARDENED_RUNTIME" => data.enable_hardened_runtime = yes_no_bool(value),
                "ENABLE_HEADER_DEPENDENCIES" => {
                    data.enable_header_dependencies = yes_no_bool(value)
                }
                "ENABLE_ON_DEMAND_RESOURCES" => {
                    data.enable_on_demand_resources = yes_no_bool(value)
                }
                "ENABLE_STRICT_OBJC_MSGSEND" => {
                    data.enable_strict_objc_msgsend = yes_no_bool(value)
                }
                "ENABLE_TESTABILITY" => data.enable_testability = yes_no_bool(value),
                "ENABLE_TESTING_SEARCH_PATHS" => {
                    data.enable_testing_search_paths = yes_no_bool(value)
                }
                "ENTITLEMENTS_REQUIRED" => data.entitlements_required = yes_no_bool(value),
                "EXECUTABLE_FOLDER_PATH" => data.executable_folder_path = value.to_string(),
                "EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_NO" => {
                    data.executable_folder_path_shallow_bundle_no = value.to_string()
                }
                "EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_YES" => {
                    data.executable_folder_path_shallow_bundle_yes = value.to_string()
                }
                "EXECUTABLE_NAME" => data.executable_name = value.to_string(),
                "EXECUTABLE_PATH" => data.executable_path = value.to_string(),
                "FILE_LIST" => data.file_list = value.into(),
                "FULL_PRODUCT_NAME" => data.full_product_name = value.to_string(),
                "GENERATED_MODULEMAP_DIR" => data.generated_modulemap_dir = value.into(),
                "GENERATE_INFOPLIST_FILE" => data.generate_infoplist_file = yes_no_bool(value),
                "GENERATE_MASTER_OBJECT_FILE" => {
                    data.generate_master_object_file = yes_no_bool(value)
                }
                "GENERATE_PKGINFO_FILE" => data.generate_pkginfo_file = yes_no_bool(value),
                "GENERATE_PROFILING_CODE" => data.generate_profiling_code = yes_no_bool(value),
                "GENERATE_TEXT_BASED_STUBS" => data.generate_text_based_stubs = yes_no_bool(value),
                "INFOPLIST_FILE" => data.infoplist_file = value.to_string(),
                "INFOPLIST_PATH" => data.infoplist_path = value.to_string(),
                "INFOPLIST_PREPROCESS" => data.infoplist_preprocess = yes_no_bool(value),
                "INSTALL_OWNER" => data.install_owner = value.to_string(),
                "INSTALL_PATH" => data.install_path = value.into(),
                "INSTALL_ROOT" => data.install_root = value.into(),
                "LD_DEPENDENCY_INFO_FILE" => data.ld_dependency_info_file = value.into(),
                "LIBRARY_DEXT_INSTALL_PATH" => data.library_dext_install_path = value.into(),
                "LIBRARY_KEXT_INSTALL_PATH" => data.library_kext_install_path = value.into(),
                "LOCALIZATION_EXPORT_SUPPORTED" => {
                    data.localization_export_supported = yes_no_bool(value)
                }
                "METAL_LIBRARY_OUTPUT_DIR" => data.metal_library_output_dir = value.into(),
                "NO_COMMON" => data.no_common = yes_no_bool(value),
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
                "SHALLOW_BUNDLE" => data.shallow_bundle = yes_no_bool(value),
                "SHALLOW_BUNDLE_TRIPLE" => data.shallow_bundle_triple = value.to_string(),
                "SYMROOT" => data.symroot = value.into(),
                "VERSION_INFO_BUILDER" => data.version_info_builder = value.to_string(),
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
