//! Offline Verilog-A-to-Rust backend for the generated built-in devices.
//!
//! Unlike the VM and the JIT, nothing here runs during a simulation. This
//! backend takes canonical IR and emits Rust source, which is checked into
//! `rspice-core/src/device/veriloga_generated/` and compiled as ordinary code
//! by the `veriloga-builtins` feature — a build that uses generated built-ins
//! does not link this crate at all. The [`crate`] README documents the
//! `rspice-veriloga-gen` binary that drives it.
//!
//! ## Backend tiers
//!
//! [`RustBackendSelection`] records how a device was emitted. `ScalarOptIr` is
//! the target: OptIR lowered to straight-line scalar Rust, which optimizes
//! best. Models whose derivative structure makes that impractical fall back
//! through sparse-local and structured kernels to the legacy device emitter,
//! and every fallback is reported as a [`BuiltinBackendFallbackReason`] rather
//! than silently taken. Setting `RSPICE_RUST_BACKEND_REQUIRE_SCALAR_BUILTINS`
//! turns any fallback into a hard error.
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
    BuiltinBackendFallbackReason, BuiltinBackendSelectionCounts, BuiltinGenerationReport,
    BuiltinSubsetGenerationReport, GENERATED_BUILTIN_MANIFEST_FILE_NAME,
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

/// Which emitter produced a device.
///
/// One variant, and it stays an enum rather than collapsing to nothing so the
/// manifest keeps a field that says what wrote the tree. Phase 6 deleted the
/// four tiers this used to choose between — `ScalarOptIr`, `SparseLocalKernel`,
/// `StructuredKernel`, `ScalarHybrid` and `LegacyDevice` — after the corpus had
/// been regenerated through the canonical emitter with `legacy-device=0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RustBackendSelection {
    /// The canonical CFG emitter — control flow, packed derivatives, and only
    /// the stamp entries that exist.
    CanonicalCfg,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedRustDeviceReport {
    pub device: GeneratedRustDevice,
    pub backend: RustBackendSelection,
    pub fallback_reason: Option<String>,
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

    /// Retained as a name after Phase 6 collapsed the tiers.
    ///
    /// There is one emitter now, so this and [`Self::new`] are the same thing.
    /// Kept because "canonical" is what callers meant when they asked for it,
    /// and a call site that says so still reads correctly.
    pub fn new_canonical(options: RustTranspileOptions) -> Self {
        Self::new(options)
    }

    pub fn transpile(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDevice, RustBackendError> {
        self.transpile_with_report(artifact)
            .map(|report| report.device)
    }

    pub fn transpile_with_report(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDeviceReport, RustBackendError> {
        let mut report = GeneratedRustDeviceReport {
            device: canonical::generate_device(artifact, &self.options)?,
            backend: RustBackendSelection::CanonicalCfg,
            fallback_reason: None,
        };
        state_file::finalize_checkpoint_identity(&mut report.device)?;
        Ok(report)
    }
}

fn combined_structured_tier_error(
    artifact: &CanonicalIrArtifact,
    local_error: &RustBackendError,
    structured_error: &RustBackendError,
) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        format!(
            "sparse local kernel path: {}; structured kernel path: {}",
            unsupported_detail(local_error),
            unsupported_detail(structured_error)
        ),
    )
}

fn auto_backend_unsupported(
    artifact: &CanonicalIrArtifact,
    scalar_error: &RustBackendError,
    structured_error: &RustBackendError,
    hybrid_error: &RustBackendError,
) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        format!(
            "model cannot be lowered by optimized generated Rust backends; direct scalar path: {}; structured kernel path: {}; hybrid scalar path: {}",
            unsupported_detail(scalar_error),
            unsupported_detail(structured_error),
            unsupported_detail(hybrid_error),
        ),
    )
}

fn unsupported_detail(error: &RustBackendError) -> &str {
    error
        .message
        .strip_prefix("unsupported Verilog-A construct for Rust backend: ")
        .unwrap_or(error.message.as_str())
}

