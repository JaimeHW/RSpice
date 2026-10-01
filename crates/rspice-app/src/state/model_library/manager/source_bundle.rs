//! Host acquisition of external model-source files.

pub(super) use rspice_simulation::model_import::{
    import_project_source_bundle as build, project_import_limits as limits,
};

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
