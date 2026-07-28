//! Offline Verilog-A-to-Rust backend for the generated built-in devices.
//!
//! Unlike the VM and the JIT, nothing here runs during a simulation. This
//! backend takes canonical IR and emits Rust source, which is checked into
//! `rspice-core/src/device/veriloga_generated/` and compiled as ordinary code
//! by the `veriloga-builtins` feature — a build that uses generated built-ins
//! does not link this crate at all. The [`crate`] README documents the
//! `rspice-veriloga-gen` binary that drives it.
//!
//! ## One emitter
//!
//! There is a single path from canonical IR to Rust: [`canonical`], which
//! lowers the CFG with its control flow intact and packed derivative lanes.
//! There is no tier to select, no fallback to report, and no environment
//! variable that changes which emitter runs — a model either lowers or the
//! generator fails with the construct that stopped it.
//!
//! ## Determinism
//!
//! Generation must be reproducible, because staleness is detected by digest
//! rather than by timestamp: the manifest pairs a digest of the model sources
//! with `RSPICE_VERILOGA_GENERATOR_SOURCE_DIGEST`, computed in `build.rs` over
//! this crate's own source, so editing the generator invalidates its output
//! exactly as editing a model does. Emission is therefore ordered, writes are
//! skipped when content is unchanged, and stale device folders are removed
//! rather than left behind.

mod builtins;
pub mod canonical;
mod discover;
pub mod emit;
mod error;
mod expr;
mod files;
mod manifest;
mod names;
mod noise;
mod registry;
mod state_file;
pub mod stamp_plan;

pub use builtins::{
    BuiltinGenerationReport, BuiltinSubsetGenerationReport, GENERATED_BUILTIN_MANIFEST_FILE_NAME,
    REGENERATE_BUILTINS_COMMAND, generate_generated_builtin_subset_with_progress,
    generate_generated_builtin_subset_with_progress_and_jobs, regenerate_generated_builtins,
    regenerate_generated_builtins_with_progress,
    regenerate_generated_builtins_with_progress_and_jobs, validate_generated_builtins,
};
pub use discover::{
    VERILOGA_COMPILE_PROFILE_FILE_NAME, VERILOGA_DISCOVERY_SKIP_MARKER, VerilogACompileProfile,
    VerilogASourceCandidate, discover_veriloga_sources,
};
pub use error::{RustBackendError, RustBackendErrorKind};
pub use files::{
    cleanup_stale_generated_device_folders, write_generated_device, write_text_file_if_changed,
};
pub use manifest::{
    GENERATED_BUILTIN_MANIFEST_SCHEMA_VERSION, GeneratedBuiltinManifest,
    GeneratedBuiltinManifestDevice, GeneratedBuiltinManifestFile,
    GeneratedBuiltinWorkspaceResources, parse_generated_builtin_manifest,
    render_generated_builtin_manifest,
};
pub use names::{RustDeviceNames, sanitize_identifier};
pub use registry::resolve_generated_registry_model_names;

use crate::canonical_ir::CanonicalIrArtifact;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedRustFile {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedRustDevice {
    pub module_name: String,
    pub public_model_name: String,
    pub folder_name: String,
    pub files: Vec<GeneratedRustFile>,
    pub source_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTranspileOptions {
    pub runtime_path: String,
}

impl Default for RustTranspileOptions {
    fn default() -> Self {
        Self {
            runtime_path: "crate::device::veriloga_generated".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RustTranspiler {
    options: RustTranspileOptions,
}

impl RustTranspiler {
    pub fn new(options: RustTranspileOptions) -> Self {
        Self { options }
    }

    pub fn transpile(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDevice, RustBackendError> {
        let mut device = canonical::generate_device(artifact, &self.options)?;
        state_file::finalize_checkpoint_identity(&mut device)?;
        Ok(device)
    }
}

