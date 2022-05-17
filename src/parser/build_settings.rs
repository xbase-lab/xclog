use std::path::PathBuf;

/// Subset of build settings
#[derive(Debug)]
pub struct BuildSettings {
    pub action: String,
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
}

impl BuildSettings {
    pub fn new(mut lines: std::str::Split<'_, &str>) -> anyhow::Result<BuildSettings> {
        let mut action: Option<String> = None;
        let mut ad_hoc_code_signing_allowed: Option<bool> = None;
        let mut allow_target_platform_specialization: Option<bool> = None;
        let mut alternate_owner: Option<bool> = None;
        let mut always_search_user_paths: Option<bool> = None;
        let mut always_use_separate_headermaps: Option<bool> = None;
        let mut application_extension_api_only: Option<bool> = None;
        let mut apply_rules_in_copy_files: Option<bool> = None;
        let mut apply_rules_in_copy_headers: Option<bool> = None;
        let mut archs: Option<Vec<String>> = None;
        let mut build_active_resources_only: Option<bool> = None;
        let mut build_dir: Option<PathBuf> = None;
        let mut build_library_for_distribution: Option<bool> = None;
        let mut build_root: Option<PathBuf> = None;
        let mut built_products_dir: Option<PathBuf> = None;
        let mut cache_root: Option<PathBuf> = None;
        let mut class_file_dir: Option<PathBuf> = None;
        let mut clean_precomps: Option<bool> = None;
        let mut clone_headers: Option<bool> = None;
        let mut codesigning_folder_path: Option<PathBuf> = None;
        let mut code_signing_allowed: Option<bool> = None;
        let mut code_signing_required: Option<bool> = None;
        let mut code_sign_identity: Option<String> = None;
        let mut code_sign_inject_base_entitlements: Option<bool> = None;
        let mut color_diagnostics: Option<bool> = None;
        let mut combine_hidpi_images: Option<bool> = None;
        let mut composite_sdk_dirs: Option<PathBuf> = None;
        let mut compress_png_files: Option<bool> = None;
        let mut configuration: Option<String> = None;
        let mut configuration_build_dir: Option<PathBuf> = None;
        let mut configuration_temp_dir: Option<PathBuf> = None;
        let mut contents_folder_path: Option<String> = None;
        let mut contents_folder_path_shallow_bundle_no: Option<String> = None;
        let mut contents_folder_path_shallow_bundle_yes: Option<String> = None;
        let mut copying_preserves_hfs_data: Option<bool> = None;
        let mut copy_headers_run_unifdef: Option<bool> = None;
        let mut copy_phase_strip: Option<bool> = None;
        let mut copy_resources_from_static_frameworks: Option<bool> = None;
        let mut corresponding_device_platform_dir: Option<PathBuf> = None;
        let mut corresponding_device_platform_name: Option<String> = None;
        let mut corresponding_device_sdk_dir: Option<PathBuf> = None;
        let mut corresponding_device_sdk_name: Option<String> = None;
        let mut create_infoplist_section_in_binary: Option<bool> = None;
        let mut debug_information_format: Option<String> = None;
        let mut defines_module: Option<bool> = None;
        let mut deployment_location: Option<bool> = None;
        let mut derived_files_dir: Option<PathBuf> = None;
        let mut derived_file_dir: Option<PathBuf> = None;
        let mut derived_sources_dir: Option<PathBuf> = None;
        let mut developer_applications_dir: Option<PathBuf> = None;
        let mut developer_bin_dir: Option<PathBuf> = None;
        let mut developer_dir: Option<PathBuf> = None;
        let mut development_team: Option<String> = None;
        let mut disable_manual_target_order_build_warning: Option<bool> = None;
        let mut documentation_folder_path: Option<String> = None;
        let mut dstroot: Option<PathBuf> = None;
        let mut dwarf_dsym_file_name: Option<String> = None;
        let mut dwarf_dsym_file_should_accompany_product: Option<bool> = None;
        let mut dwarf_dsym_folder_path: Option<PathBuf> = None;
        let mut effective_platform_name: Option<String> = None;
        let mut enable_app_sandbox: Option<bool> = None;
        let mut enable_bitcode: Option<bool> = None;
        let mut enable_default_header_search_paths: Option<bool> = None;
        let mut enable_default_search_paths: Option<bool> = None;
        let mut enable_hardened_runtime: Option<bool> = None;
        let mut enable_header_dependencies: Option<bool> = None;
        let mut enable_on_demand_resources: Option<bool> = None;
        let mut enable_strict_objc_msgsend: Option<bool> = None;
        let mut enable_testability: Option<bool> = None;
        let mut enable_testing_search_paths: Option<bool> = None;
        let mut entitlements_required: Option<bool> = None;
        let mut executable_folder_path: Option<String> = None;
        let mut executable_folder_path_shallow_bundle_no: Option<String> = None;
        let mut executable_folder_path_shallow_bundle_yes: Option<String> = None;
        let mut executable_name: Option<String> = None;
        let mut executable_path: Option<String> = None;
        let mut file_list: Option<PathBuf> = None;
        let mut full_product_name: Option<String> = None;
        let mut generated_modulemap_dir: Option<PathBuf> = None;
        let mut generate_infoplist_file: Option<bool> = None;
        let mut generate_master_object_file: Option<bool> = None;
        let mut generate_pkginfo_file: Option<bool> = None;
        let mut generate_profiling_code: Option<bool> = None;
        let mut generate_text_based_stubs: Option<bool> = None;
        let mut infoplist_file: Option<String> = None;
        let mut infoplist_path: Option<String> = None;
        let mut infoplist_preprocess: Option<bool> = None;
        let mut install_owner: Option<String> = None;
        let mut install_path: Option<PathBuf> = None;
        let mut install_root: Option<PathBuf> = None;
        let mut ld_dependency_info_file: Option<PathBuf> = None;
        let mut library_dext_install_path: Option<PathBuf> = None;
        let mut library_kext_install_path: Option<PathBuf> = None;
        let mut localization_export_supported: Option<bool> = None;
        let mut metal_library_output_dir: Option<PathBuf> = None;
        let mut no_common: Option<bool> = None;
        let mut product_bundle_identifier: Option<String> = None;
        let mut product_module_name: Option<String> = None;
        let mut product_name: Option<String> = None;
        let mut product_settings_path: Option<PathBuf> = None;
        let mut product_type: Option<String> = None;
        let mut project: Option<String> = None;
        let mut project_dir: Option<PathBuf> = None;
        let mut project_file_path: Option<PathBuf> = None;
        let mut sdkroot: Option<PathBuf> = None;
        let mut sdk_dir: Option<PathBuf> = None;
        let mut sdk_version: Option<String> = None;
        let mut shallow_bundle: Option<bool> = None;
        let mut shallow_bundle_triple: Option<String> = None;
        let mut symroot: Option<PathBuf> = None;
        let mut version_info_builder: Option<String> = None;
        let mut wrapper_name: Option<String> = None;

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
                "ACTION" => action = Some(value.to_string()),
                "AD_HOC_CODE_SIGNING_ALLOWED" => {
                    ad_hoc_code_signing_allowed = Some(yes_no_bool(value));
                }
                "ALLOW_TARGET_PLATFORM_SPECIALIZATION" => {
                    allow_target_platform_specialization = Some(yes_no_bool(value))
                }
                "ALTERNATE_OWNER" => alternate_owner = Some(yes_no_bool(value)),
                "ALWAYS_SEARCH_USER_PATHS" => always_search_user_paths = Some(yes_no_bool(value)),
                "ALWAYS_USE_SEPARATE_HEADERMAPS" => {
                    always_use_separate_headermaps = Some(yes_no_bool(value))
                }
                "APPLICATION_EXTENSION_API_ONLY" => {
                    application_extension_api_only = Some(yes_no_bool(value))
                }
                "APPLY_RULES_IN_COPY_FILES" => apply_rules_in_copy_files = Some(yes_no_bool(value)),
                "APPLY_RULES_IN_COPY_HEADERS" => {
                    apply_rules_in_copy_headers = Some(yes_no_bool(value))
                }
                "ARCHS" => {
                    archs = Some(value.split_whitespace().map(ToString::to_string).collect())
                }
                "BUILD_ACTIVE_RESOURCES_ONLY" => {
                    build_active_resources_only = Some(yes_no_bool(value))
                }
                "BUILD_DIR" => build_dir = Some(PathBuf::from(value)),
                "BUILD_LIBRARY_FOR_DISTRIBUTION" => {
                    build_library_for_distribution = Some(yes_no_bool(value))
                }
                "BUILD_ROOT" => build_root = Some(PathBuf::from(value)),
                "BUILT_PRODUCTS_DIR" => built_products_dir = Some(PathBuf::from(value)),
                "CACHE_ROOT" => cache_root = Some(PathBuf::from(value)),
                "CLASS_FILE_DIR" => class_file_dir = Some(PathBuf::from(value)),
                "CLEAN_PRECOMPS" => clean_precomps = Some(yes_no_bool(value)),
                "CLONE_HEADERS" => clone_headers = Some(yes_no_bool(value)),
                "CODESIGNING_FOLDER_PATH" => codesigning_folder_path = Some(PathBuf::from(value)),
                "CODE_SIGNING_ALLOWED" => code_signing_allowed = Some(yes_no_bool(value)),
                "CODE_SIGNING_REQUIRED" => code_signing_required = Some(yes_no_bool(value)),
                "CODE_SIGN_IDENTITY" => code_sign_identity = Some(value.to_string()),
                "CODE_SIGN_INJECT_BASE_ENTITLEMENTS" => {
                    code_sign_inject_base_entitlements = Some(yes_no_bool(value))
                }
                "COLOR_DIAGNOSTICS" => color_diagnostics = Some(yes_no_bool(value)),
                "COMBINE_HIDPI_IMAGES" => combine_hidpi_images = Some(yes_no_bool(value)),
                "COMPOSITE_SDK_DIRS" => composite_sdk_dirs = Some(PathBuf::from(value)),
                "COMPRESS_PNG_FILES" => compress_png_files = Some(yes_no_bool(value)),
                "CONFIGURATION" => configuration = Some(value.to_string()),
                "CONFIGURATION_BUILD_DIR" => configuration_build_dir = Some(PathBuf::from(value)),
                "CONFIGURATION_TEMP_DIR" => configuration_temp_dir = Some(PathBuf::from(value)),
                "CONTENTS_FOLDER_PATH" => contents_folder_path = Some(value.into()),
                "CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_NO" => {
                    contents_folder_path_shallow_bundle_no = Some(value.to_string())
                }
                "CONTENTS_FOLDER_PATH_SHALLOW_BUNDLE_YES" => {
                    contents_folder_path_shallow_bundle_yes = Some(value.to_string())
                }
                "COPYING_PRESERVES_HFS_DATA" => {
                    copying_preserves_hfs_data = Some(yes_no_bool(value))
                }
                "COPY_HEADERS_RUN_UNIFDEF" => copy_headers_run_unifdef = Some(yes_no_bool(value)),
                "COPY_PHASE_STRIP" => copy_phase_strip = Some(yes_no_bool(value)),
                "COPY_RESOURCES_FROM_STATIC_FRAMEWORKS" => {
                    copy_resources_from_static_frameworks = Some(yes_no_bool(value))
                }
                "CORRESPONDING_DEVICE_PLATFORM_DIR" => {
                    corresponding_device_platform_dir = Some(PathBuf::from(value))
                }
                "CORRESPONDING_DEVICE_PLATFORM_NAME" => {
                    corresponding_device_platform_name = Some(value.to_string())
                }
                "CORRESPONDING_DEVICE_SDK_DIR" => {
                    corresponding_device_sdk_dir = Some(PathBuf::from(value))
                }
                "CORRESPONDING_DEVICE_SDK_NAME" => {
                    corresponding_device_sdk_name = Some(value.to_string())
                }
                "CREATE_INFOPLIST_SECTION_IN_BINARY" => {
                    create_infoplist_section_in_binary = Some(yes_no_bool(value))
                }
                "DEBUG_INFORMATION_FORMAT" => debug_information_format = Some(value.to_string()),
                "DEFINES_MODULE" => defines_module = Some(yes_no_bool(value)),
                "DEPLOYMENT_LOCATION" => deployment_location = Some(yes_no_bool(value)),
                "DERIVED_FILES_DIR" => derived_files_dir = Some(PathBuf::from(value)),
                "DERIVED_FILE_DIR" => derived_file_dir = Some(PathBuf::from(value)),
                "DERIVED_SOURCES_DIR" => derived_sources_dir = Some(PathBuf::from(value)),
                "DEVELOPER_APPLICATIONS_DIR" => {
                    developer_applications_dir = Some(PathBuf::from(value))
                }
                "DEVELOPER_BIN_DIR" => developer_bin_dir = Some(PathBuf::from(value)),
                "DEVELOPER_DIR" => developer_dir = Some(PathBuf::from(value)),
                "DEVELOPMENT_TEAM" => development_team = Some(value.to_string()),
                "DISABLE_MANUAL_TARGET_ORDER_BUILD_WARNING" => {
                    disable_manual_target_order_build_warning = Some(yes_no_bool(value))
                }
                "DOCUMENTATION_FOLDER_PATH" => documentation_folder_path = Some(value.to_string()),
                "DSTROOT" => dstroot = Some(PathBuf::from(value)),
                "DWARF_DSYM_FILE_NAME" => dwarf_dsym_file_name = Some(value.to_string()),
                "DWARF_DSYM_FILE_SHOULD_ACCOMPANY_PRODUCT" => {
                    dwarf_dsym_file_should_accompany_product = Some(yes_no_bool(value))
                }
                "DWARF_DSYM_FOLDER_PATH" => dwarf_dsym_folder_path = Some(PathBuf::from(value)),
                "EFFECTIVE_PLATFORM_NAME" => effective_platform_name = Some(value.to_string()),
                "ENABLE_APP_SANDBOX" => enable_app_sandbox = Some(yes_no_bool(value)),
                "ENABLE_BITCODE" => enable_bitcode = Some(yes_no_bool(value)),
                "ENABLE_DEFAULT_HEADER_SEARCH_PATHS" => {
                    enable_default_header_search_paths = Some(yes_no_bool(value))
                }
                "ENABLE_DEFAULT_SEARCH_PATHS" => {
                    enable_default_search_paths = Some(yes_no_bool(value))
                }
                "ENABLE_HARDENED_RUNTIME" => enable_hardened_runtime = Some(yes_no_bool(value)),
                "ENABLE_HEADER_DEPENDENCIES" => {
                    enable_header_dependencies = Some(yes_no_bool(value))
                }
                "ENABLE_ON_DEMAND_RESOURCES" => {
                    enable_on_demand_resources = Some(yes_no_bool(value))
                }
                "ENABLE_STRICT_OBJC_MSGSEND" => {
                    enable_strict_objc_msgsend = Some(yes_no_bool(value))
                }
                "ENABLE_TESTABILITY" => enable_testability = Some(yes_no_bool(value)),
                "ENABLE_TESTING_SEARCH_PATHS" => {
                    enable_testing_search_paths = Some(yes_no_bool(value))
                }
                "ENTITLEMENTS_REQUIRED" => entitlements_required = Some(yes_no_bool(value)),
                "EXECUTABLE_FOLDER_PATH" => executable_folder_path = Some(value.to_string()),
                "EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_NO" => {
                    executable_folder_path_shallow_bundle_no = Some(value.to_string())
                }
                "EXECUTABLE_FOLDER_PATH_SHALLOW_BUNDLE_YES" => {
                    executable_folder_path_shallow_bundle_yes = Some(value.to_string())
                }
                "EXECUTABLE_NAME" => executable_name = Some(value.to_string()),
                "EXECUTABLE_PATH" => executable_path = Some(value.to_string()),
                "FILE_LIST" => file_list = Some(PathBuf::from(value)),
                "FULL_PRODUCT_NAME" => full_product_name = Some(value.to_string()),
                "GENERATED_MODULEMAP_DIR" => generated_modulemap_dir = Some(PathBuf::from(value)),
                "GENERATE_INFOPLIST_FILE" => generate_infoplist_file = Some(yes_no_bool(value)),
                "GENERATE_MASTER_OBJECT_FILE" => {
                    generate_master_object_file = Some(yes_no_bool(value))
                }
                "GENERATE_PKGINFO_FILE" => generate_pkginfo_file = Some(yes_no_bool(value)),
                "GENERATE_PROFILING_CODE" => generate_profiling_code = Some(yes_no_bool(value)),
                "GENERATE_TEXT_BASED_STUBS" => generate_text_based_stubs = Some(yes_no_bool(value)),
                "INFOPLIST_FILE" => infoplist_file = Some(value.to_string()),
                "INFOPLIST_PATH" => infoplist_path = Some(value.to_string()),
                "INFOPLIST_PREPROCESS" => infoplist_preprocess = Some(yes_no_bool(value)),
                "INSTALL_OWNER" => install_owner = Some(value.to_string()),
                "INSTALL_PATH" => install_path = Some(PathBuf::from(value)),
                "INSTALL_ROOT" => install_root = Some(PathBuf::from(value)),
                "LD_DEPENDENCY_INFO_FILE" => ld_dependency_info_file = Some(PathBuf::from(value)),
                "LIBRARY_DEXT_INSTALL_PATH" => {
                    library_dext_install_path = Some(PathBuf::from(value))
                }
                "LIBRARY_KEXT_INSTALL_PATH" => {
                    library_kext_install_path = Some(PathBuf::from(value))
                }
                "LOCALIZATION_EXPORT_SUPPORTED" => {
                    localization_export_supported = Some(yes_no_bool(value))
                }
                "METAL_LIBRARY_OUTPUT_DIR" => metal_library_output_dir = Some(PathBuf::from(value)),
                "NO_COMMON" => no_common = Some(yes_no_bool(value)),
                "PRODUCT_BUNDLE_IDENTIFIER" => product_bundle_identifier = Some(value.to_string()),
                "PRODUCT_MODULE_NAME" => product_module_name = Some(value.to_string()),
                "PRODUCT_NAME" => product_name = Some(value.to_string()),
                "PRODUCT_SETTINGS_PATH" => product_settings_path = Some(PathBuf::from(value)),
                "PRODUCT_TYPE" => product_type = Some(value.to_string()),
                "PROJECT" => project = Some(value.to_string()),
                "PROJECT_DIR" => project_dir = Some(PathBuf::from(value)),
                "PROJECT_FILE_PATH" => project_file_path = Some(PathBuf::from(value)),
                "SDKROOT" => sdkroot = Some(PathBuf::from(value)),
                "SDK_DIR" => sdk_dir = Some(PathBuf::from(value)),
                "SDK_VERSION" => sdk_version = Some(value.to_string()),
                "SHALLOW_BUNDLE" => shallow_bundle = Some(yes_no_bool(value)),
                "SHALLOW_BUNDLE_TRIPLE" => shallow_bundle_triple = Some(value.to_string()),
                "SYMROOT" => symroot = Some(PathBuf::from(value)),
                "VERSION_INFO_BUILDER" => version_info_builder = Some(value.to_string()),
                "WRAPPER_NAME" => wrapper_name = Some(value.to_string()),
                _ => continue,
            }
        }

        let action = action.ok_or_else(|| anyhow::anyhow!("missing action:"))?;
        let ad_hoc_code_signing_allowed = ad_hoc_code_signing_allowed
            .ok_or_else(|| anyhow::anyhow!("missing ad_hoc_code_signing_allowed:"))?;
        let allow_target_platform_specialization = allow_target_platform_specialization
            .ok_or_else(|| anyhow::anyhow!("missing allow_target_platform_specialization:"))?;
        let alternate_owner =
            alternate_owner.ok_or_else(|| anyhow::anyhow!("missing alternate_owner:"))?;
        let always_search_user_paths = always_search_user_paths
            .ok_or_else(|| anyhow::anyhow!("missing always_search_user_paths:"))?;
        let always_use_separate_headermaps = always_use_separate_headermaps
            .ok_or_else(|| anyhow::anyhow!("missing always_use_separate_headermaps:"))?;
        let application_extension_api_only = application_extension_api_only
            .ok_or_else(|| anyhow::anyhow!("missing application_extension_api_only:"))?;
        let apply_rules_in_copy_files = apply_rules_in_copy_files
            .ok_or_else(|| anyhow::anyhow!("missing apply_rules_in_copy_files:"))?;
        let apply_rules_in_copy_headers = apply_rules_in_copy_headers
            .ok_or_else(|| anyhow::anyhow!("missing apply_rules_in_copy_headers:"))?;
        let archs = archs.ok_or_else(|| anyhow::anyhow!("missing archs:"))?;
        let build_active_resources_only = build_active_resources_only
            .ok_or_else(|| anyhow::anyhow!("missing build_active_resources_only:"))?;
        let build_dir = build_dir.ok_or_else(|| anyhow::anyhow!("missing build_dir:"))?;
        let build_library_for_distribution = build_library_for_distribution
            .ok_or_else(|| anyhow::anyhow!("missing build_library_for_distribution:"))?;
        let build_root = build_root.ok_or_else(|| anyhow::anyhow!("missing build_root:"))?;
        let built_products_dir =
            built_products_dir.ok_or_else(|| anyhow::anyhow!("missing built_products_dir:"))?;
        let cache_root = cache_root.ok_or_else(|| anyhow::anyhow!("missing cache_root:"))?;
        let class_file_dir =
            class_file_dir.ok_or_else(|| anyhow::anyhow!("missing class_file_dir:"))?;
        let clean_precomps =
            clean_precomps.ok_or_else(|| anyhow::anyhow!("missing clean_precomps:"))?;
        let clone_headers =
            clone_headers.ok_or_else(|| anyhow::anyhow!("missing clone_headers:"))?;
        let codesigning_folder_path = codesigning_folder_path
            .ok_or_else(|| anyhow::anyhow!("missing codesigning_folder_path:"))?;
        let code_signing_allowed =
            code_signing_allowed.ok_or_else(|| anyhow::anyhow!("missing code_signing_allowed:"))?;
        let code_signing_required = code_signing_required
            .ok_or_else(|| anyhow::anyhow!("missing code_signing_required:"))?;
        let code_sign_identity =
            code_sign_identity.ok_or_else(|| anyhow::anyhow!("missing code_sign_identity:"))?;
        let code_sign_inject_base_entitlements = code_sign_inject_base_entitlements
            .ok_or_else(|| anyhow::anyhow!("missing code_sign_inject_base_entitlements:"))?;
        let color_diagnostics =
            color_diagnostics.ok_or_else(|| anyhow::anyhow!("missing color_diagnostics:"))?;
        let combine_hidpi_images =
            combine_hidpi_images.ok_or_else(|| anyhow::anyhow!("missing combine_hidpi_images:"))?;
        let composite_sdk_dirs =
            composite_sdk_dirs.ok_or_else(|| anyhow::anyhow!("missing composite_sdk_dirs:"))?;
        let compress_png_files =
            compress_png_files.ok_or_else(|| anyhow::anyhow!("missing compress_png_files:"))?;
        let configuration =
            configuration.ok_or_else(|| anyhow::anyhow!("missing configuration:"))?;
        let configuration_build_dir = configuration_build_dir
            .ok_or_else(|| anyhow::anyhow!("missing configuration_build_dir:"))?;
        let configuration_temp_dir = configuration_temp_dir
            .ok_or_else(|| anyhow::anyhow!("missing configuration_temp_dir:"))?;
        let contents_folder_path =
            contents_folder_path.ok_or_else(|| anyhow::anyhow!("missing contents_folder_path:"))?;
        let contents_folder_path_shallow_bundle_no = contents_folder_path_shallow_bundle_no
            .ok_or_else(|| anyhow::anyhow!("missing contents_folder_path_shallow_bundle_no:"))?;
        let contents_folder_path_shallow_bundle_yes = contents_folder_path_shallow_bundle_yes
            .ok_or_else(|| anyhow::anyhow!("missing contents_folder_path_shallow_bundle_yes:"))?;
        let copying_preserves_hfs_data = copying_preserves_hfs_data
            .ok_or_else(|| anyhow::anyhow!("missing copying_preserves_hfs_data:"))?;
        let copy_headers_run_unifdef = copy_headers_run_unifdef
            .ok_or_else(|| anyhow::anyhow!("missing copy_headers_run_unifdef:"))?;
        let copy_phase_strip =
            copy_phase_strip.ok_or_else(|| anyhow::anyhow!("missing copy_phase_strip:"))?;
        let copy_resources_from_static_frameworks = copy_resources_from_static_frameworks
            .ok_or_else(|| anyhow::anyhow!("missing copy_resources_from_static_frameworks:"))?;
        let corresponding_device_platform_dir = corresponding_device_platform_dir
            .ok_or_else(|| anyhow::anyhow!("missing corresponding_device_platform_dir:"))?;
        let corresponding_device_platform_name = corresponding_device_platform_name
            .ok_or_else(|| anyhow::anyhow!("missing corresponding_device_platform_name:"))?;
        let corresponding_device_sdk_dir = corresponding_device_sdk_dir
            .ok_or_else(|| anyhow::anyhow!("missing corresponding_device_sdk_dir:"))?;
        let corresponding_device_sdk_name = corresponding_device_sdk_name
            .ok_or_else(|| anyhow::anyhow!("missing corresponding_device_sdk_name:"))?;
        let create_infoplist_section_in_binary = create_infoplist_section_in_binary
            .ok_or_else(|| anyhow::anyhow!("missing create_infoplist_section_in_binary:"))?;
        let debug_information_format = debug_information_format
            .ok_or_else(|| anyhow::anyhow!("missing debug_information_format:"))?;
        let defines_module =
            defines_module.ok_or_else(|| anyhow::anyhow!("missing defines_module:"))?;
        let deployment_location =
            deployment_location.ok_or_else(|| anyhow::anyhow!("missing deployment_location:"))?;
        let derived_files_dir =
            derived_files_dir.ok_or_else(|| anyhow::anyhow!("missing derived_files_dir:"))?;
        let derived_file_dir =
            derived_file_dir.ok_or_else(|| anyhow::anyhow!("missing derived_file_dir:"))?;
        let derived_sources_dir =
            derived_sources_dir.ok_or_else(|| anyhow::anyhow!("missing derived_sources_dir:"))?;
        let developer_applications_dir = developer_applications_dir
            .ok_or_else(|| anyhow::anyhow!("missing developer_applications_dir:"))?;
        let developer_bin_dir =
            developer_bin_dir.ok_or_else(|| anyhow::anyhow!("missing developer_bin_dir:"))?;
        let developer_dir =
            developer_dir.ok_or_else(|| anyhow::anyhow!("missing developer_dir:"))?;
        let development_team =
            development_team.ok_or_else(|| anyhow::anyhow!("missing development_team:"))?;
        let disable_manual_target_order_build_warning = disable_manual_target_order_build_warning
            .ok_or_else(|| {
            anyhow::anyhow!("missing disable_manual_target_order_build_warning:")
        })?;
        let documentation_folder_path = documentation_folder_path
            .ok_or_else(|| anyhow::anyhow!("missing documentation_folder_path:"))?;
        let dstroot = dstroot.ok_or_else(|| anyhow::anyhow!("missing dstroot:"))?;
        let dwarf_dsym_file_name =
            dwarf_dsym_file_name.ok_or_else(|| anyhow::anyhow!("missing dwarf_dsym_file_name:"))?;
        let dwarf_dsym_file_should_accompany_product = dwarf_dsym_file_should_accompany_product
            .ok_or_else(|| anyhow::anyhow!("missing dwarf_dsym_file_should_accompany_product:"))?;
        let dwarf_dsym_folder_path = dwarf_dsym_folder_path
            .ok_or_else(|| anyhow::anyhow!("missing dwarf_dsym_folder_path:"))?;
        let effective_platform_name = effective_platform_name
            .ok_or_else(|| anyhow::anyhow!("missing effective_platform_name:"))?;
        let enable_app_sandbox =
            enable_app_sandbox.ok_or_else(|| anyhow::anyhow!("missing enable_app_sandbox:"))?;
        let enable_bitcode =
            enable_bitcode.ok_or_else(|| anyhow::anyhow!("missing enable_bitcode:"))?;
        let enable_default_header_search_paths = enable_default_header_search_paths
            .ok_or_else(|| anyhow::anyhow!("missing enable_default_header_search_paths:"))?;
        let enable_default_search_paths = enable_default_search_paths
            .ok_or_else(|| anyhow::anyhow!("missing enable_default_search_paths:"))?;
        let enable_hardened_runtime = enable_hardened_runtime
            .ok_or_else(|| anyhow::anyhow!("missing enable_hardened_runtime:"))?;
        let enable_header_dependencies = enable_header_dependencies
            .ok_or_else(|| anyhow::anyhow!("missing enable_header_dependencies:"))?;
        let enable_on_demand_resources = enable_on_demand_resources
            .ok_or_else(|| anyhow::anyhow!("missing enable_on_demand_resources:"))?;
        let enable_strict_objc_msgsend = enable_strict_objc_msgsend
            .ok_or_else(|| anyhow::anyhow!("missing enable_strict_objc_msgsend:"))?;
        let enable_testability =
            enable_testability.ok_or_else(|| anyhow::anyhow!("missing enable_testability:"))?;
        let enable_testing_search_paths = enable_testing_search_paths
            .ok_or_else(|| anyhow::anyhow!("missing enable_testing_search_paths:"))?;
        let entitlements_required = entitlements_required
            .ok_or_else(|| anyhow::anyhow!("missing entitlements_required:"))?;
        let executable_folder_path = executable_folder_path
            .ok_or_else(|| anyhow::anyhow!("missing executable_folder_path:"))?;
        let executable_folder_path_shallow_bundle_no = executable_folder_path_shallow_bundle_no
            .ok_or_else(|| anyhow::anyhow!("missing executable_folder_path_shallow_bundle_no:"))?;
        let executable_folder_path_shallow_bundle_yes = executable_folder_path_shallow_bundle_yes
            .ok_or_else(|| {
            anyhow::anyhow!("missing executable_folder_path_shallow_bundle_yes:")
        })?;
        let executable_name =
            executable_name.ok_or_else(|| anyhow::anyhow!("missing executable_name:"))?;
        let executable_path =
            executable_path.ok_or_else(|| anyhow::anyhow!("missing executable_path:"))?;
        let file_list = file_list.ok_or_else(|| anyhow::anyhow!("missing file_list:"))?;
        let full_product_name =
            full_product_name.ok_or_else(|| anyhow::anyhow!("missing full_product_name:"))?;
        let generated_modulemap_dir = generated_modulemap_dir
            .ok_or_else(|| anyhow::anyhow!("missing generated_modulemap_dir:"))?;
        let generate_infoplist_file = generate_infoplist_file
            .ok_or_else(|| anyhow::anyhow!("missing generate_infoplist_file:"))?;
        let generate_master_object_file = generate_master_object_file
            .ok_or_else(|| anyhow::anyhow!("missing generate_master_object_file:"))?;
        let generate_pkginfo_file = generate_pkginfo_file
            .ok_or_else(|| anyhow::anyhow!("missing generate_pkginfo_file:"))?;
        let generate_profiling_code = generate_profiling_code
            .ok_or_else(|| anyhow::anyhow!("missing generate_profiling_code:"))?;
        let generate_text_based_stubs = generate_text_based_stubs
            .ok_or_else(|| anyhow::anyhow!("missing generate_text_based_stubs:"))?;
        let infoplist_file =
            infoplist_file.ok_or_else(|| anyhow::anyhow!("missing infoplist_file:"))?;
        let infoplist_path =
            infoplist_path.ok_or_else(|| anyhow::anyhow!("missing infoplist_path:"))?;
        let infoplist_preprocess =
            infoplist_preprocess.ok_or_else(|| anyhow::anyhow!("missing infoplist_preprocess:"))?;
        let install_owner =
            install_owner.ok_or_else(|| anyhow::anyhow!("missing install_owner:"))?;
        let install_path = install_path.ok_or_else(|| anyhow::anyhow!("missing install_path:"))?;
        let install_root = install_root.ok_or_else(|| anyhow::anyhow!("missing install_root:"))?;
        let ld_dependency_info_file = ld_dependency_info_file
            .ok_or_else(|| anyhow::anyhow!("missing ld_dependency_info_file:"))?;
        let library_dext_install_path = library_dext_install_path
            .ok_or_else(|| anyhow::anyhow!("missing library_dext_install_path:"))?;
        let library_kext_install_path = library_kext_install_path
            .ok_or_else(|| anyhow::anyhow!("missing library_kext_install_path:"))?;
        let localization_export_supported = localization_export_supported
            .ok_or_else(|| anyhow::anyhow!("missing localization_export_supported:"))?;
        let metal_library_output_dir = metal_library_output_dir
            .ok_or_else(|| anyhow::anyhow!("missing metal_library_output_dir:"))?;
        let no_common = no_common.ok_or_else(|| anyhow::anyhow!("missing no_common:"))?;
        let product_bundle_identifier = product_bundle_identifier
            .ok_or_else(|| anyhow::anyhow!("missing product_bundle_identifier:"))?;
        let product_module_name =
            product_module_name.ok_or_else(|| anyhow::anyhow!("missing product_module_name:"))?;
        let product_name = product_name.ok_or_else(|| anyhow::anyhow!("missing product_name:"))?;
        let product_settings_path = product_settings_path
            .ok_or_else(|| anyhow::anyhow!("missing product_settings_path:"))?;
        let product_type = product_type.ok_or_else(|| anyhow::anyhow!("missing product_type:"))?;
        let project = project.ok_or_else(|| anyhow::anyhow!("missing project:"))?;
        let project_dir = project_dir.ok_or_else(|| anyhow::anyhow!("missing project_dir:"))?;
        let project_file_path =
            project_file_path.ok_or_else(|| anyhow::anyhow!("missing project_file_path:"))?;
        let sdkroot = sdkroot.ok_or_else(|| anyhow::anyhow!("missing sdkroot:"))?;
        let sdk_dir = sdk_dir.ok_or_else(|| anyhow::anyhow!("missing sdk_dir:"))?;
        let sdk_version = sdk_version.ok_or_else(|| anyhow::anyhow!("missing sdk_version:"))?;
        let shallow_bundle =
            shallow_bundle.ok_or_else(|| anyhow::anyhow!("missing shallow_bundle:"))?;
        let shallow_bundle_triple = shallow_bundle_triple
            .ok_or_else(|| anyhow::anyhow!("missing shallow_bundle_triple:"))?;
        let symroot = symroot.ok_or_else(|| anyhow::anyhow!("missing symroot:"))?;
        let version_info_builder =
            version_info_builder.ok_or_else(|| anyhow::anyhow!("missing version_info_builder:"))?;
        let wrapper_name = wrapper_name.ok_or_else(|| anyhow::anyhow!("missing wrapper_name:"))?;
        Ok(Self {
            action,
            ad_hoc_code_signing_allowed,
            allow_target_platform_specialization,
            alternate_owner,
            always_search_user_paths,
            always_use_separate_headermaps,
            application_extension_api_only,
            apply_rules_in_copy_files,
            apply_rules_in_copy_headers,
            archs,
            build_active_resources_only,
            build_dir,
            build_library_for_distribution,
            build_root,
            built_products_dir,
            cache_root,
            class_file_dir,
            clean_precomps,
            clone_headers,
            codesigning_folder_path,
            code_signing_allowed,
            code_signing_required,
            code_sign_identity,
            code_sign_inject_base_entitlements,
            color_diagnostics,
            combine_hidpi_images,
            composite_sdk_dirs,
            compress_png_files,
            configuration,
            configuration_build_dir,
            configuration_temp_dir,
            contents_folder_path,
            contents_folder_path_shallow_bundle_no,
            contents_folder_path_shallow_bundle_yes,
            copying_preserves_hfs_data,
            copy_headers_run_unifdef,
            copy_phase_strip,
            copy_resources_from_static_frameworks,
            corresponding_device_platform_dir,
            corresponding_device_platform_name,
            corresponding_device_sdk_dir,
            corresponding_device_sdk_name,
            create_infoplist_section_in_binary,
            debug_information_format,
            defines_module,
            deployment_location,
            derived_files_dir,
            derived_file_dir,
            derived_sources_dir,
            developer_applications_dir,
            developer_bin_dir,
            developer_dir,
            development_team,
            disable_manual_target_order_build_warning,
            documentation_folder_path,
            dstroot,
            dwarf_dsym_file_name,
            dwarf_dsym_file_should_accompany_product,
            dwarf_dsym_folder_path,
            effective_platform_name,
            enable_app_sandbox,
            enable_bitcode,
            enable_default_header_search_paths,
            enable_default_search_paths,
            enable_hardened_runtime,
            enable_header_dependencies,
            enable_on_demand_resources,
            enable_strict_objc_msgsend,
            enable_testability,
            enable_testing_search_paths,
            entitlements_required,
            executable_folder_path,
            executable_folder_path_shallow_bundle_no,
            executable_folder_path_shallow_bundle_yes,
            executable_name,
            executable_path,
            file_list,
            full_product_name,
            generated_modulemap_dir,
            generate_infoplist_file,
            generate_master_object_file,
            generate_pkginfo_file,
            generate_profiling_code,
            generate_text_based_stubs,
            infoplist_file,
            infoplist_path,
            infoplist_preprocess,
            install_owner,
            install_path,
            install_root,
            ld_dependency_info_file,
            library_dext_install_path,
            library_kext_install_path,
            localization_export_supported,
            metal_library_output_dir,
            no_common,
            product_bundle_identifier,
            product_module_name,
            product_name,
            product_settings_path,
            product_type,
            project,
            project_dir,
            project_file_path,
            sdkroot,
            sdk_dir,
            sdk_version,
            shallow_bundle,
            shallow_bundle_triple,
            symroot,
            version_info_builder,
            wrapper_name,
        })
    }
}
fn yes_no_bool(value: &str) -> bool {
    match value {
        "No" => false,
        "Yes" => true,
        _ => false,
    }
}
