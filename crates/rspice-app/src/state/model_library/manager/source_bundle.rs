//! Compiler adapters and project limits for model-source imports.
//!
//! Portable source authentication and catalog construction belong to the model
//! library. This adapter validates reachable HDL with the application's compiler.

use rspice_model_library::{
    ModelLibrary,
    source_bundle::{self, HdlSourceInclude, HdlSourceInputs, ImportLimits},
};

pub(super) fn build(
    display_name: &str,
    root_member: Option<&str>,
    files: Vec<(String, Vec<u8>)>,
    section: Option<&str>,
) -> Result<(String, ModelLibrary), String> {
    source_bundle::import(
        display_name,
        root_member,
        files,
        section,
        limits(),
        validate_hdl_sources,
    )
}

fn validate_hdl_sources(input: HdlSourceInputs<'_>) -> Result<Vec<HdlSourceInclude>, String> {
    let mut includes = Vec::new();
    let virtual_files = input
        .sources
        .iter()
        .map(|(path, source)| rspice_veriloga::VirtualSourceFile::new(path, source))
        .collect::<Vec<_>>();
    let veriloga_limits = rspice_veriloga::VirtualCompileLimits {
        max_files: crate::state::MAX_PROJECT_SOURCE_FILES,
        max_path_bytes: crate::state::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
        max_file_bytes: crate::state::MAX_PROJECT_CODE_SOURCE_BYTES,
        max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
        max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
        max_module_name_bytes: 128,
    };
    for veriloga_root in input.roots {
        let bundle =
            rspice_veriloga::VirtualSourceBundle::new(veriloga_root, virtual_files.iter().cloned())
                .map_err(|error| {
                    format!(
                        "Uploaded Verilog-A bundle rooted at '{veriloga_root}' is invalid: {error}"
                    )
                })?;
        let prepared = rspice_veriloga::VerilogACompiler::default()
            .prepare_virtual_runtime_source(&bundle, veriloga_limits)
            .map_err(|error| {
                format!(
                    "Uploaded Verilog-A bundle rooted at '{veriloga_root}' cannot be compiled: {error}"
                )
            })?;
        if prepared.module_names().next().is_none() && !prepared.is_connect_library() {
            return Err(format!(
                "Uploaded Verilog-A root '{veriloga_root}' declares no device modules or connection library"
            ));
        }
        includes.extend(
            prepared
                .include_graph()
                .iter()
                .map(|include| HdlSourceInclude {
                    including_path: include.including_path.clone(),
                    requested_path: include.requested_path.clone(),
                    included_path: include.included_path.clone(),
                }),
        );
    }
    Ok(includes)
}

pub(super) fn limits() -> ImportLimits {
    ImportLimits {
        max_files: crate::state::MAX_PROJECT_SOURCE_FILES,
        max_total_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn capture_external_hdl_sources(
    root: &std::path::Path,
) -> Result<rspice_model_library::CapturedHdlSources, String> {
    let import_limits = limits();
    let provider_limits = rspice_veriloga::SourceProviderLimits {
        max_dependencies: import_limits.max_files.saturating_add(64),
        max_total_source_bytes: import_limits.max_total_bytes.saturating_mul(2),
        max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
        max_expanded_bytes: import_limits.max_total_bytes.saturating_mul(2),
    };
    let mut preprocessor = rspice_veriloga::Preprocessor::new();
    preprocessor
        .preprocess_file_with_limits(root, provider_limits)
        .map_err(|error| {
            format!(
                "Could not authenticate Verilog-A closure rooted at '{}': {error}",
                root.display()
            )
        })?;
    let sources = preprocessor
        .take_dependency_documents()
        .into_iter()
        .filter(|document| document.origin == rspice_veriloga::SourceDocumentOrigin::Provider)
        .map(|document| (document.logical_path, document.source))
        .collect();
    let dependencies = preprocessor
        .take_include_graph()
        .into_iter()
        .map(|include| rspice_core::library::ResolvedLibDependency {
            owner: include.including_path,
            requested_path: include.requested_path,
            target: include.included_path,
        })
        .collect();
    Ok(rspice_model_library::CapturedHdlSources {
        sources,
        dependencies,
    })
}

#[cfg(target_arch = "wasm32")]
pub(super) fn capture_external_hdl_sources(
    root: &std::path::Path,
) -> Result<rspice_model_library::CapturedHdlSources, String> {
    Err(format!(
        "Browser cannot acquire external Verilog-A source '{}'",
        root.display()
    ))
}
