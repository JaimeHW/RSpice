//! Compilation policy shared by project, model-library and signed-PDK sources.
//!
//! Source owners validate executable closures before a simulation is prepared.
//! Keeping this policy with those owners lets validation and execution retain
//! the same digital plan without source validation depending on the runner.

/// Retain the canonical digital plan for installation through the unified
/// engine's mixed host when the source requires it.
pub(crate) fn unified_runtime_compiler_options() -> rspice_veriloga::CompilerOptions {
    rspice_veriloga::CompilerOptions {
        enable_ams: true,
        ..Default::default()
    }
}
