//! Compilation policy shared by project, model-library and signed-PDK sources.
//!
//! Source owners validate executable closures before a simulation is prepared.
//! Validation and execution share this policy and retain the same digital plan.

/// Retain the canonical digital plan for installation through the unified
/// engine's mixed host when the source requires it.
pub fn unified_runtime_compiler_options() -> rspice_veriloga::CompilerOptions {
    rspice_veriloga::CompilerOptions {
        enable_ams: true,
        ..Default::default()
    }
}
