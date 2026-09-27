//! Compiler preparation for retained model-source imports.
//!
//! The model library owns portable source data. This service prepares every HDL
//! root before constructing the import; the caller owns live catalog publication.

use rspice_model_library::{
    ModelLibrary,
    source_bundle::{self, HdlSourceInclude, HdlSourceInputs, ImportLimits},
};

/// Prepare the complete selected source closure and build its retained model catalog.
///
/// HDL preparation and include discovery use the same immutable source snapshot
/// consumed by catalog construction. No partial library is returned on failure.
pub fn import_source_bundle(
    display_name: &str,
    root_member: Option<&str>,
    files: Vec<(String, Vec<u8>)>,
    section: Option<&str>,
    limits: ImportLimits,
    veriloga_limits: rspice_veriloga::VirtualCompileLimits,
) -> Result<(String, ModelLibrary), String> {
    let prepared = source_bundle::prepare(display_name, root_member, files, section, limits)?;
    let includes = validate_hdl_sources(prepared.hdl_inputs(), veriloga_limits)?;
    prepared.into_model_library(includes)
}

fn validate_hdl_sources(
    input: HdlSourceInputs<'_>,
    veriloga_limits: rspice_veriloga::VirtualCompileLimits,
) -> Result<Vec<HdlSourceInclude>, String> {
    let mut includes = Vec::new();
    let virtual_files = input
        .sources
        .iter()
        .map(|(path, source)| rspice_veriloga::VirtualSourceFile::new(path, source))
        .collect::<Vec<_>>();

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

#[cfg(test)]
mod tests;
