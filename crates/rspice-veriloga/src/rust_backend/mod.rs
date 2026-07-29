//! Offline Verilog-A-to-Rust backend for the generated built-in devices.
//!
//! Unlike the VM and the JIT, nothing here runs during a simulation. This
//! backend takes canonical IR and emits Rust source, which is checked into
//! feature-selectable crates under `rspice-veriloga-models/` and compiled
//! against the stable generated-model runtime ABI by the
//! `veriloga-builtins` feature — a build that uses generated built-ins
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
pub mod stamp_plan;
mod state_file;

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
use crate::metrics::{
    Measured, MetricsRecorder, NoPipelineControl, PerformanceBudget, PipelineControl,
    PipelinePhase, usize_to_u64,
};

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
    /// Optional wall-clock limits for offline lowering and emission.
    pub performance_budget: PerformanceBudget,
}

impl Default for RustTranspileOptions {
    fn default() -> Self {
        Self {
            runtime_path: "crate::device::veriloga_generated".to_string(),
            performance_budget: PerformanceBudget::default(),
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
        self.transpile_measured(artifact)
            .map(|generated| generated.output)
    }

    /// Transpile one canonical artifact and retain structured phase metrics.
    pub fn transpile_measured(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<Measured<GeneratedRustDevice>, RustBackendError> {
        self.transpile_measured_with_control(artifact, &NoPipelineControl)
    }

    /// Cancellable, progress-observable form of [`Self::transpile_measured`].
    pub fn transpile_measured_with_control(
        &self,
        artifact: &CanonicalIrArtifact,
        control: &dyn PipelineControl,
    ) -> Result<Measured<GeneratedRustDevice>, RustBackendError> {
        let mut measurements =
            MetricsRecorder::with_control(0, self.options.performance_budget.clone(), control);
        let mut device =
            canonical::generate_device_measured(artifact, &self.options, &mut measurements)?;
        measurements
            .checkpoint(PipelinePhase::CheckpointFinalization)
            .map_err(|error| {
                RustBackendError::cancelled(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    error,
                )
            })?;
        let phase_started = web_time::Instant::now();
        state_file::finalize_checkpoint_identity(&mut device)?;
        measurements
            .record(
                PipelinePhase::CheckpointFinalization,
                phase_started.elapsed(),
            )
            .map_err(|error| {
                RustBackendError::performance_budget(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    error,
                )
            })?;
        let generated_bytes = device.files.iter().fold(0_usize, |total, file| {
            total.saturating_add(file.contents.len())
        });
        let generated_lines = device.files.iter().fold(0_usize, |total, file| {
            total.saturating_add(file.contents.lines().count())
        });
        measurements.metrics_mut().generated_rust_bytes = usize_to_u64(generated_bytes);
        measurements.metrics_mut().generated_rust_lines = usize_to_u64(generated_lines);
        Ok(Measured {
            output: device,
            metrics: measurements.finish(),
        })
    }
}
