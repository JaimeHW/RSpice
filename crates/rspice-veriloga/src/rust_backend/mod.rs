mod builtins;
mod compact;
mod device;
mod discover;
mod error;
mod expr;
mod files;
mod kernel_ir;
mod manifest;
mod names;
mod noise;
mod registry;
mod scalar;

pub use builtins::{
    BuiltinBackendFallbackReason, BuiltinBackendSelectionCounts, BuiltinGenerationReport,
    BuiltinSubsetGenerationReport, GENERATED_BUILTIN_MANIFEST_FILE_NAME,
    REGENERATE_BUILTINS_COMMAND, generate_generated_builtin_subset_with_progress,
    generate_generated_builtin_subset_with_progress_and_jobs, regenerate_generated_builtins,
    regenerate_generated_builtins_with_progress,
    regenerate_generated_builtins_with_progress_and_jobs, validate_generated_builtins,
};
pub use device::render_runtime_support_module;
pub use discover::{
    VERILOGA_COMPILE_PROFILE_FILE_NAME, VERILOGA_DISCOVERY_SKIP_MARKER, VerilogACompileProfile,
    VerilogASourceCandidate, discover_veriloga_sources,
};
pub use error::RustBackendError;
pub use files::{
    cleanup_stale_generated_device_folders, write_generated_device, write_text_file_if_changed,
};
pub use manifest::{
    GeneratedBuiltinManifest, parse_generated_builtin_manifest, render_generated_builtin_manifest,
};
pub use names::{RustDeviceNames, sanitize_identifier};
pub use registry::resolve_generated_registry_model_names;

use crate::canonical_ir::CanonicalIrArtifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustFile {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustDevice {
    pub module_name: String,
    pub public_model_name: String,
    pub folder_name: String,
    pub files: Vec<GeneratedRustFile>,
    pub source_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustBackendSelection {
    ScalarOptIr,
    SparseLocalKernel,
    StructuredKernel,
    ScalarHybrid,
    LegacyDevice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustDeviceReport {
    pub device: GeneratedRustDevice,
    pub backend: RustBackendSelection,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustKernelTier {
    DirectScalar,
    SparseLocal,
    Structured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustDerivativeStorage {
    Scalar,
    Sparse,
    Dense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RustKernelPlan {
    pub preferred_tier: RustKernelTier,
    pub derivative_storage: RustDerivativeStorage,
    pub scalar_values: usize,
    pub scalar_derivative_entries: usize,
    pub scalar_optimizer_nodes: usize,
    pub structured_expressions: usize,
    pub structured_operations: usize,
    pub structured_control_regions: usize,
    pub runtime_loop_operations: usize,
    pub scheduled_opt_operations: usize,
    pub derivative_lanes: usize,
    pub maximum_value_derivative_lanes: usize,
    pub scalar_expansion_ratio: usize,
    pub statement_regions: usize,
    pub equation_regions: usize,
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
    backend: RustBackendKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum RustBackendKind {
    #[default]
    Auto,
    Legacy,
    ScalarOptIr,
    SparseLocalKernel,
    StructuredKernel,
}

impl RustTranspiler {
    pub fn new(options: RustTranspileOptions) -> Self {
        Self::new_auto(options)
    }

    pub fn new_legacy(options: RustTranspileOptions) -> Self {
        Self {
            options,
            backend: RustBackendKind::Legacy,
        }
    }

    pub fn new_auto(options: RustTranspileOptions) -> Self {
        Self {
            options,
            backend: RustBackendKind::Auto,
        }
    }

    pub fn new_scalar(options: RustTranspileOptions) -> Self {
        Self {
            options,
            backend: RustBackendKind::ScalarOptIr,
        }
    }

    pub fn new_structured(options: RustTranspileOptions) -> Self {
        Self {
            options,
            backend: RustBackendKind::StructuredKernel,
        }
    }

    pub fn new_sparse_local(options: RustTranspileOptions) -> Self {
        Self {
            options,
            backend: RustBackendKind::SparseLocalKernel,
        }
    }

    pub fn options(&self) -> &RustTranspileOptions {
        &self.options
    }

    pub fn plan(&self, artifact: &CanonicalIrArtifact) -> Result<RustKernelPlan, RustBackendError> {
        kernel_ir::KernelPlan::analyze(artifact).map(|plan| plan.summary(artifact))
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
        match self.backend {
            RustBackendKind::Auto => self.transpile_auto(artifact),
            RustBackendKind::Legacy => Ok(GeneratedRustDeviceReport {
                device: device::generate_device(artifact, &self.options)?,
                backend: RustBackendSelection::LegacyDevice,
                fallback_reason: None,
            }),
            RustBackendKind::ScalarOptIr => Ok(GeneratedRustDeviceReport {
                device: scalar::generate_device(artifact, &self.options)?,
                backend: RustBackendSelection::ScalarOptIr,
                fallback_reason: None,
            }),
            RustBackendKind::SparseLocalKernel => Ok(GeneratedRustDeviceReport {
                device: device::generate_sparse_local_kernel_device(artifact, &self.options)?,
                backend: RustBackendSelection::SparseLocalKernel,
                fallback_reason: None,
            }),
            RustBackendKind::StructuredKernel => Ok(GeneratedRustDeviceReport {
                device: device::generate_structured_kernel_device(artifact, &self.options)?,
                backend: RustBackendSelection::StructuredKernel,
                fallback_reason: None,
            }),
        }
    }

    fn transpile_auto(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDeviceReport, RustBackendError> {
        let plan = kernel_ir::KernelPlan::analyze(artifact)?;
        match plan.preferred_tier() {
            kernel_ir::PreferredKernelTier::DirectScalar => {
                match scalar::generate_device(artifact, &self.options) {
                    Ok(device) => Ok(GeneratedRustDeviceReport {
                        device,
                        backend: RustBackendSelection::ScalarOptIr,
                        fallback_reason: None,
                    }),
                    Err(scalar_error) if scalar_error.is_unsupported() => {
                        self.transpile_structured_after_scalar_failure(artifact, scalar_error)
                    }
                    Err(error) => Err(error),
                }
            }
            kernel_ir::PreferredKernelTier::SparseLocal => self.transpile_sparse_local(artifact),
            kernel_ir::PreferredKernelTier::Structured => {
                match device::generate_structured_kernel_device(artifact, &self.options) {
                    Ok(device) => Ok(GeneratedRustDeviceReport {
                        device,
                        backend: RustBackendSelection::StructuredKernel,
                        fallback_reason: None,
                    }),
                    Err(structured_error) if structured_error.is_unsupported() => {
                        match scalar::generate_device(artifact, &self.options) {
                            Ok(device) => Ok(GeneratedRustDeviceReport {
                                device,
                                backend: RustBackendSelection::ScalarOptIr,
                                fallback_reason: Some(format!(
                                    "structured kernel path: {}",
                                    unsupported_detail(&structured_error)
                                )),
                            }),
                            Err(scalar_error) if scalar_error.is_unsupported() => self
                                .transpile_hybrid_after_tier_failures(
                                    artifact,
                                    scalar_error,
                                    structured_error,
                                ),
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                }
            }
        }
    }

    fn transpile_sparse_local(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDeviceReport, RustBackendError> {
        match device::generate_sparse_local_kernel_device(artifact, &self.options) {
            Ok(device) => Ok(GeneratedRustDeviceReport {
                device,
                backend: RustBackendSelection::SparseLocalKernel,
                fallback_reason: None,
            }),
            Err(local_error) if local_error.is_unsupported() => {
                match device::generate_structured_kernel_device(artifact, &self.options) {
                    Ok(device) => Ok(GeneratedRustDeviceReport {
                        device,
                        backend: RustBackendSelection::StructuredKernel,
                        fallback_reason: Some(format!(
                            "sparse local kernel path: {}",
                            unsupported_detail(&local_error)
                        )),
                    }),
                    Err(structured_error) if structured_error.is_unsupported() => {
                        match scalar::generate_device(artifact, &self.options) {
                            Ok(device) => Ok(GeneratedRustDeviceReport {
                                device,
                                backend: RustBackendSelection::ScalarOptIr,
                                fallback_reason: Some(format!(
                                    "sparse local kernel path: {}; structured kernel path: {}",
                                    unsupported_detail(&local_error),
                                    unsupported_detail(&structured_error)
                                )),
                            }),
                            Err(scalar_error) if scalar_error.is_unsupported() => {
                                let combined_structured_error = combined_structured_tier_error(
                                    artifact,
                                    &local_error,
                                    &structured_error,
                                );
                                self.transpile_hybrid_after_tier_failures(
                                    artifact,
                                    scalar_error,
                                    combined_structured_error,
                                )
                            }
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }

    fn transpile_structured_after_scalar_failure(
        &self,
        artifact: &CanonicalIrArtifact,
        scalar_error: RustBackendError,
    ) -> Result<GeneratedRustDeviceReport, RustBackendError> {
        match device::generate_structured_kernel_device(artifact, &self.options) {
            Ok(device) => Ok(GeneratedRustDeviceReport {
                device,
                backend: RustBackendSelection::StructuredKernel,
                fallback_reason: Some(format!(
                    "direct scalar path: {}",
                    unsupported_detail(&scalar_error)
                )),
            }),
            Err(structured_error) if structured_error.is_unsupported() => {
                self.transpile_hybrid_after_tier_failures(artifact, scalar_error, structured_error)
            }
            Err(error) => Err(error),
        }
    }

    fn transpile_hybrid_after_tier_failures(
        &self,
        artifact: &CanonicalIrArtifact,
        scalar_error: RustBackendError,
        structured_error: RustBackendError,
    ) -> Result<GeneratedRustDeviceReport, RustBackendError> {
        match device::generate_hybrid_device(artifact, &self.options) {
            Ok(device) => Ok(GeneratedRustDeviceReport {
                device,
                backend: RustBackendSelection::ScalarHybrid,
                fallback_reason: Some(format!(
                    "direct scalar path: {}; structured kernel path: {}",
                    unsupported_detail(&scalar_error),
                    unsupported_detail(&structured_error)
                )),
            }),
            Err(hybrid_error) if hybrid_error.is_unsupported() => Err(auto_backend_unsupported(
                artifact,
                &scalar_error,
                &structured_error,
                &hybrid_error,
            )),
            Err(error) => Err(error),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpile_with_report_records_selected_backend() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module tiny_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_auto(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("transpile tiny resistor");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert_eq!(report.device.public_model_name, "tiny_res");
    }

    #[test]
    fn auto_backend_does_not_select_explicit_legacy_generator() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module ideal_opamp(out, inp, inn);
    inout out, inp, inn;
    electrical out, inp, inn;
    analog V(out): V(inp, inn) == 0.0;
endmodule
"#,
            )
            .expect("canonical IR");

        let error = RustTranspiler::new_auto(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect_err("auto backend must not select the explicit legacy generator");

        assert!(
            error
                .message
                .contains("model cannot be lowered by optimized generated Rust backends"),
            "{error}"
        );
        assert!(
            error
                .message
                .contains("hybrid scalar path: indirect contributions")
        );
    }

    #[test]
    fn scalar_backend_emits_shared_limexp_helpers() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module shared_limexp_helpers(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ limexp(V(p, n));
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("limexp should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("fn scalar_limexp("), "{stamp}");
        assert!(stamp.contains("fn scalar_limexp_derivative("), "{stamp}");
        assert!(stamp.matches("scalar_limexp(").count() >= 2, "{stamp}");
        assert!(
            stamp.matches("scalar_limexp_derivative(").count() >= 2,
            "{stamp}"
        );
        assert!(!stamp.contains("let limexp_arg"), "{stamp}");
    }

    #[test]
    fn scalar_backend_prunes_zero_derivative_lanes() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module zero_derivative_pow(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ pow(V(p, n), 0.0);
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("zero derivative lanes should not block scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("stamp_current_const_local"), "{stamp}");
        assert!(!stamp.contains("stamp_current_node"), "{stamp}");
        assert!(!stamp.contains("let d"), "{stamp}");
    }

    #[test]
    fn scalar_backend_specializes_constant_integer_powers() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module integer_power_specializations(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ pow(V(p, n) + 1.0, 2.0)
                 + pow(V(p, n) + 2.0, 3.0)
                 + pow(V(p, n) + 3.0, 4.0);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("constant integer powers should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("{let pb="), "{stamp}");
        assert!(stamp.contains("let ps=pb*pb;ps*ps"), "{stamp}");
        assert!(!stamp.contains(".powi(4)"), "{stamp}");
        assert!(!stamp.contains("f64::powf("), "{stamp}");
        assert!(!stamp.contains(".powf("), "{stamp}");
    }

    #[test]
    fn scalar_backend_inlines_stamp_derivative_values() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module inline_stamp_derivatives(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("resistor contribution should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("stamp_current_node2_local"), "{stamp}");
        assert!(!stamp.contains("let d"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_noise_sources_as_zero_stamps() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module noisy_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog begin
        I(p, n) <+ V(p, n) / r;
        I(p, n) <+ white_noise(4.0e-21, "thermal");
        I(p, n) <+ flicker_noise(1.0e-18, 1.0, "flicker");
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("noise-only contributions should lower to scalar zero stamps");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();
        assert!(stamp.contains("stamp_current_node2_local"), "{stamp}");
        assert!(!stamp.contains("white_noise"), "{stamp}");
        assert!(!stamp.contains("flicker_noise"), "{stamp}");

        let noise = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "noise.rs")
            .expect("noise ABI file")
            .contents
            .as_str();
        assert!(noise.starts_with(
            "#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]"
        ));
        assert!(
            noise.contains("pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2]"),
            "{noise}"
        );
        assert!(noise.contains("WHITE_P_N_THERMAL"), "{noise}");
        assert!(noise.contains("FLICKER_P_N_FLICKER"), "{noise}");
        assert!(noise.contains("GeneratedNoiseKind::White"), "{noise}");
        assert!(noise.contains("GeneratedNoiseKind::Flicker"), "{noise}");
        assert!(
            noise.contains("const LIMEXP_MAX: f64 = 5.54062238439351e34;"),
            "{noise}"
        );
        assert!(
            noise.contains("const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;"),
            "{noise}"
        );
        assert!(
            noise.contains("Result<GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError>"),
            "{noise}"
        );
        assert!(noise.contains("let table_operands = vec![]"), "{noise}");
        assert!(!noise.contains("evaluate_noise_table_operand"), "{noise}");
        assert!(!noise.contains("pub struct GeneratedNoise"), "{noise}");
        assert!(!noise.contains("pub enum GeneratedNoise"), "{noise}");
    }

    #[test]
    fn noise_abi_replays_conditional_loop_dependencies_in_order() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module noisy_loop(p, n);
    inout p, n;
    electrical p, n;
    parameter real density = 2.0 from [0:inf);
    parameter real enabled = 1.0;
    real accumulated;
    integer iteration;
    analog begin
        accumulated = 1.0;
        iteration = 0;
        while (iteration < 2) begin
            accumulated = accumulated + density;
            iteration = iteration + 1;
        end
        I(p, n) <+ white_noise(enabled ? accumulated : density, "looped");
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("conditional loop noise dependencies should lower");
        let noise = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "noise.rs")
            .expect("noise ABI file")
            .contents
            .as_str();

        let initial = noise
            .find("noise_variable_0 = 1.0")
            .expect("accumulator initialization must be retained");
        let loop_start = noise
            .find("loop {")
            .expect("dependency loop must be retained");
        let update = noise[loop_start..]
            .find("noise_variable_0 =")
            .map(|update| loop_start + update)
            .expect("loop-carried accumulator update must be retained");
        assert!(initial < loop_start && loop_start < update, "{noise}");
        assert!(noise.contains("noise_variable_1 = 0.0"), "{noise}");
        assert!(noise.contains("noise_variable_1 ="), "{noise}");
        assert!(noise.contains("WHITE_P_N_LOOPED"), "{noise}");
    }

    #[test]
    fn noise_abi_loop_liveness_converges_for_two_variable_cycle() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module noisy_cycle(p, n);
    inout p, n;
    electrical p, n;
    real left;
    real right;
    integer iteration;
    analog begin
        left = 1.0;
        right = 2.0;
        iteration = 0;
        while (iteration < 2) begin
            left = right;
            right = left;
            iteration = iteration + 1;
        end
        I(p, n) <+ white_noise(left, "cycle");
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let generated = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile(&artifact)
            .expect("cyclic loop liveness must converge");
        let noise = generated
            .files
            .iter()
            .find(|file| file.relative_path == "noise.rs")
            .expect("noise ABI file")
            .contents
            .as_str();
        assert!(noise.contains("loop {"), "{noise}");
        assert!(noise.matches("noise_variable_0 =").count() >= 3, "{noise}");
        assert!(noise.matches("noise_variable_1 =").count() >= 3, "{noise}");
    }

    #[test]
    fn noise_abi_resets_shared_loop_state_between_activation_and_metadata() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_loop_noise(p, n);
    inout p, n;
    electrical p, n;
    real accumulated;
    integer iteration;
    analog begin
        accumulated = 0.0;
        iteration = 0;
        while (iteration < 2) begin
            accumulated = accumulated + 1.0;
            iteration = iteration + 1;
        end
        I(p, n) <+ (accumulated > 0.0)
            ? white_noise(accumulated, "shared_loop")
            : 0.0;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let generated = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile(&artifact)
            .expect("shared activation/metadata loop should lower");
        let noise = generated
            .files
            .iter()
            .find(|file| file.relative_path == "noise.rs")
            .expect("noise ABI file")
            .contents
            .as_str();
        let first_loop = noise.find("loop {").expect("activation loop");
        let reset = noise[first_loop..]
            .find("noise_variable_0 = 0.0;")
            .map(|position| first_loop + position)
            .expect("metadata phase reset");
        let second_loop = noise[reset..]
            .find("loop {")
            .map(|position| reset + position)
            .expect("metadata loop");
        assert!(first_loop < reset && reset < second_loop, "{noise}");
    }

    #[test]
    fn noise_abi_rejects_dynamic_initial_step_dependency_without_persisted_state() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module dynamic_initial_noise(p, n);
    inout p, n;
    electrical p, n;
    real captured;
    analog begin
        @(initial_step) captured = V(p, n);
        I(p, n) <+ white_noise(captured, "captured");
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let error = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile(&artifact)
            .expect_err("dynamic initial-step state must not be recomputed at noise time");
        assert!(
            error
                .to_string()
                .contains("requires persisted generated state"),
            "{error}"
        );
    }

    #[test]
    fn noise_abi_short_circuits_inactive_dynamic_source_before_invalid_psd() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_noise(p, n);
    inout p, n;
    electrical p, n;
    real invalid_density;
    real independent_density;
    analog begin
        invalid_density = -1.0;
        independent_density = 2.0;
        I(p, n) <+ (V(p, n) > 0.0)
        ? white_noise(invalid_density, "invalid_when_inactive")
        : 0.0;
        I(p, n) <+ white_noise(independent_density, "independent");
    end
endmodule
"#,
            )
            .expect("canonical IR");
        assert!(artifact.noise_sources.sources[0].activation.is_some());

        let generated = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile(&artifact)
            .expect("guarded noise should lower");
        let noise = generated
            .files
            .iter()
            .find(|file| file.relative_path == "noise.rs")
            .expect("noise ABI file")
            .contents
            .as_str();
        let inactive = noise
            .find("active: false")
            .expect("inactive evaluation return");
        let invalid_assignment = noise[inactive..]
            .find("noise_variable_0 =")
            .map(|position| inactive + position)
            .expect("invalid PSD dependency assignment");
        assert!(inactive < invalid_assignment, "{noise}");
        assert!(noise.contains("matches!(source_index, 0)"), "{noise}");
        assert!(noise.contains("matches!(source_index, 1)"), "{noise}");
        assert!(!noise.contains("matches!(source_index, 0 | 1)"), "{noise}");
        assert!(noise.contains("ctx.node_voltage"), "{noise}");
        assert!(
            super::noise::noise_liveness_expression_walks(&artifact)
                <= artifact.hir.expressions.len(),
            "cached liveness must walk each expression root at most once"
        );
    }

    #[test]
    fn scalar_backend_lowers_constant_mod_assignment_chain() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module static_mod_chain(p, n);
    inout p, n;
    electrical p, n;
    integer subversion;
    real gain;
    analog begin
        subversion = (15 * 10) % 10;
        gain = (subversion < 1) ? 0.0 : 2.0;
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("constant modulo assignment chain should lower to scalar OptIR");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_lowers_parameter_modulo() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module parameter_modulo(p, n);
    inout p, n;
    electrical p, n;
    parameter real version = 151.0;
    real gain;
    analog begin
        gain = ((version % 10.0) < 5.0) ? 2.0 : 3.0;
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("parameter modulo should lower through scalar OptIR");

        let generated = report
            .device
            .files
            .iter()
            .map(|file| file.contents.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(generated.contains(".trunc()%"), "{generated}");
        assert!(!generated.contains("AdValue"), "{generated}");
    }

    #[test]
    fn generated_state_compacts_parameter_defaults() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module compact_parameter_defaults(p, n);
    inout p, n;
    electrical p, n;
    parameter real a = 1.0;
    parameter real b = 2.0;
    parameter real c = a from (0:inf);
    parameter real d = 3.0;
    aliasparam aa = a;
    analog begin
        I(p, n) <+ (a + b + c + d) * V(p, n);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("parameter defaults should emit compact state");

        let state = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "state.rs")
            .expect("state file")
            .contents
            .as_str();

        assert!(state.contains("#[repr(C)]"), "{state}");
        assert!(state.contains("#[derive(Copy, Clone)]"), "{state}");
        assert!(state.contains("const DEFAULTS_0: [f64; 2]"), "{state}");
        assert!(state.contains("std::ptr::copy_nonoverlapping"), "{state}");
        assert!(state.contains("let params = &mut *ptr;"), "{state}");
        assert!(state.contains("params.p2 = params.p0"), "{state}");
        assert!(
            state.contains("fn finish_set_parameter(&mut self, index: usize)"),
            "{state}"
        );
        assert!(state.contains("const PARAMETER_NAME_LOOKUP"), "{state}");
        assert!(state.contains("(\"aa\", 0)"), "{state}");
        assert!(
            state.contains("validate_parameter_scalar_metadata(index, value)?"),
            "{state}"
        );
        assert!(
            state.contains("self.write_parameter_slot(index, value);"),
            "{state}"
        );
        assert!(
            state.contains("self.finish_set_parameter(index);"),
            "{state}"
        );
        assert!(
            !state.contains("self.mark_param_given(0); Ok(())"),
            "{state}"
        );
        assert!(!state.contains("match name.to_ascii_lowercase"), "{state}");
        assert!(!state.contains("self.params.p0 = value"), "{state}");
        assert!(!state.contains("impl Copy for Parameters"), "{state}");
    }

    #[test]
    fn scalar_backend_lowers_nested_ddt_conditionals() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module nested_ddt(p, n, th);
    inout p, n, th;
    electrical p, n, th;
    parameter integer enable = 1 from [0:1];
    analog begin
        I(p, n) <+ V(p, n);
        I(th) <+ (enable != 0) ? (V(th) + ddt(2.0 * V(th))) : 0.0;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("nested ddt contribution should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("eval_ddt("), "{stamp}");
        assert!(stamp.contains("stamp_current_reactive"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_guarded_internal_node_nqs_ddt() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_internal_node_nqs_ddt(p, n);
    inout p, n;
    electrical p, n;
    electrical depl_A;
    parameter real corecovery = 1.0 from [0:1];
    parameter real depnqs = 1.0 from (0:inf);
    real w_dep_a, w_qs_a, w_nqs_a, iw_nqs_a;
    analog begin
        w_dep_a = V(p, n);
        w_qs_a = 0.0;
        w_nqs_a = 0.0;
        iw_nqs_a = 0.0;
        if (corecovery > 0.0) begin
            if (depnqs > 0.0) begin
                w_qs_a = w_dep_a;
                w_nqs_a = 2.0 * V(depl_A);
                iw_nqs_a = (w_nqs_a - w_qs_a) / depnqs;
            end else begin
                w_qs_a = w_dep_a;
                w_nqs_a = w_nqs_a;
                iw_nqs_a = iw_nqs_a;
            end
        end
        if (corecovery > 0.0 && depnqs > 0.0) begin
            I(depl_A) <+ iw_nqs_a + ddt(w_nqs_a);
        end else begin
            V(depl_A) <+ 0.0;
        end
        I(p, n) <+ w_qs_a;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("guarded internal-node NQS DDT should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("eval_ddt("), "{stamp}");
        assert!(stamp.contains("stamp_current_reactive"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_direct_branch_flow_ddt_potential() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module direct_flow_inductor(p, n);
    inout p, n;
    electrical p, n;
    parameter real l = 1.0e-9 from (0:inf);
    parameter real r = 1.0 from (0:inf);
    real l_term;
    analog begin
        l_term = ddt(l * I(p, n));
        V(p, n) <+ r * I(p, n);
        V(p, n) <+ l_term;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("direct branch-flow ddt potential should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("eval_ddt("), "{stamp}");
        assert!(stamp.contains("stamp_potential_branch1_local"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_nested_idt_potential_equation() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module nested_idt_potential(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = 2.0 from (0:inf);
    analog begin
        V(p, n) <+ scale * idt(-V(p, n), 0.0);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("nested idt potential contribution should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_guarded_series_resistance_state() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_series_resistance(d, di, p, n);
    inout d, di, p, n;
    electrical d, di, p, n;
    parameter integer rds_mod = 1 from [0:2];
    parameter real drain_resistance = 2.0 from [0:inf);
    real rd, rd0, rdwmin, t0, t1;
    analog begin
        if (rds_mod != 1) begin
            rd0 = drain_resistance;
            rdwmin = 0.0;
        end else begin
            t0 = V(p, n);
            t1 = sqrt(t0 * t0 + 1.0e-4);
            rd0 = t1 + 1.0;
            rdwmin = rd0 * 0.5;
        end

        if (rds_mod == 1) begin
            t0 = V(p, n) + 0.25;
            t1 = sqrt(t0 * t0 + 1.0e-4);
            rd = rdwmin + rd0 * t1 + drain_resistance;
        end else begin
            rd = drain_resistance;
        end

        if (rds_mod == 2)
            rd = 0.0;

        if (rds_mod != 2)
            I(d, di) <+ V(d, di) / rd;
        else
            V(d, di) <+ 0.0;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("guarded series resistance state should lower to scalar OptIR");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_lowers_sibling_guard_alias_temp() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module sibling_guard_alias_temp(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    parameter integer mode = 1 from [0:1];
    real tmp;
    analog begin
        tmp = 0.0;
        if (enable == 1) begin
            if (mode == 1) begin
                tmp = V(p, n);
            end
        end
        if ((enable == 1) && (mode == 1)) begin
            I(p, n) <+ tmp;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("sibling guard alias temp should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_inactive_guarded_assignment_fallback() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module inactive_guarded_assignment_fallback(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    real tmp;
    analog begin
        tmp = 0.0;
        if (enable == 1) begin
            tmp = V(p, n);
        end
        if (enable != 1) begin
            tmp = $simparam("unsupported_scalar_probe");
        end
        if (enable == 1) begin
            I(p, n) <+ tmp;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("inactive guarded assignment fallback should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_ignores_dead_runtime_indexed_array_work() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module dead_runtime_indexed_array_work(p, n);
    inout p, n;
    electrical p, n;
    integer idx;
    real scratch[0:1];
    analog begin
        idx = 0;
        scratch[idx] = V(p, n);
        I(p, n) <+ 2.0 * V(p, n);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("dead indexed array work should not force scalar-hybrid fallback");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reconstructs_history_for_guard_alias_condition() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module history_guard_alias_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter integer polarity = 1 from [-1:1];
    real vdsu, sig;
    analog begin
        if (polarity == 1) begin
            vdsu = V(p, n);
        end else begin
            vdsu = -V(p, n);
        end

        if (vdsu < 0.0) begin
            sig = -1.0;
        end else begin
            sig = 1.0;
        end

        if (sig < 0.0) begin
            I(p, n) <+ sig * V(p, n);
        end else begin
            I(n, p) <+ sig * V(p, n);
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history guard alias condition should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reconstructs_complementary_guard_overwrite_history() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module complementary_guard_overwrite_history(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real tmp, out;
    analog begin
        tmp = $simparam("unsupported_scalar_probe");

        if (mode == 1) begin
            tmp = V(p, n);
        end else begin
            tmp = 2.0 * V(p, n);
        end

        out = tmp;
        I(p, n) <+ out;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("complete guarded overwrite should ignore unreachable stale history");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_additive_assignment_history() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module additive_assignment_history(p, n);
    inout p, n;
    electrical p, n;
    real acc;
    analog begin
        acc = 0.0;
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        acc = acc + V(p, n);
        I(p, n) <+ acc;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("additive assignment history should replay with previous-value substitution");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reconstructs_current_path_complementary_overwrites() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module current_path_complementary_overwrites(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real tmp;
    analog begin
        tmp = $simparam("unsupported_scalar_probe");

        if (mode == 1) begin
            if (V(p, n) > 0.0) begin
                tmp = sqrt(V(p, n));
            end else begin
                tmp = 0.0;
            end
            I(p, n) <+ tmp * V(p, n);
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("current-path complementary overwrites should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reconstructs_current_path_self_update_cascade() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module current_path_self_update_cascade(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    parameter integer region = 0;
    parameter integer side = 0;
    real state;
    analog begin
        state = $simparam("unsupported_stale_state");

        if (mode == 1) begin
            state = (region == 0) ? V(p, n) : state;
            state = ((region != 0) && (side == 0)) ? (2.0 * V(p, n)) : state;
            state = ((region != 0) && (side != 0)) ? (3.0 * V(p, n)) : state;
            I(p, n) <+ state;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("current-path self-update cascade should cover the stale base");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("unsupported_stale_state"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_unknown_current_path_history_before_stale_base() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module unknown_current_path_history_before_stale_base(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    real state;
    analog begin
        state = 0.0;

        if (enable == 1) begin
            state = V(p, n);
        end

        if (V(p, n) > 0.0) begin
            state = 7.0 * V(p, n);
        end

        if (enable == 0) begin
            state = $simparam("inactive_scalar_probe");
        end

        if (enable == 1) begin
            I(p, n) <+ state;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("current-path history should replay through unknown guarded assignments");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("7.0"), "{stamp}");
        assert!(!stamp.contains("inactive_scalar_probe"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_guarded_loop_self_update_on_current_path() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_loop_self_update_current_path(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    integer iter;
    real tmp;
    analog begin
        iter = 0;
        tmp = $simparam("unsupported_scalar_probe");

        while ((iter < 1) && (enable == 1)) begin
            if (enable == 1) begin
                tmp = V(p, n);
            end
            iter = iter + 1;
        end

        if (enable == 1) begin
            I(p, n) <+ tmp;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("guarded loop self-update should replay active current-path branch");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reconstructs_indirect_previous_value_alias_history() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module indirect_previous_value_alias_history(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    parameter integer mode = 1 from [0:1];
    real state, alias;
    analog begin
        state = $simparam("unsupported_scalar_probe");
        if (enable == 1) begin
            state = V(p, n);
        end

        if (mode == 1) begin
            alias = state + V(p, n);
        end else begin
            alias = state - V(p, n);
        end

        state = alias;
        if (enable == 1) begin
            I(p, n) <+ state;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history replay should resolve indirect references to the previous value");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_guarded_alias_self_history_with_branch_path() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_alias_self_history_branch_path(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    real state, alias;
    analog begin
        state = 0.0;
        alias = $simparam("unsupported_scalar_probe");

        if (enable == 1) begin
            alias = V(p, n);
            state = alias;
        end

        I(p, n) <+ state;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history replay should lower guarded aliases under the selected branch path");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_reuses_known_current_path_conditions_without_depth_growth() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module known_current_path_condition_depth(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real state;
    analog begin
        state = $simparam("unsupported_initial_state");

        if (mode == 1) begin
            if (mode == 1) begin
                if (mode == 1) begin
                    if (mode == 1) begin
                        if (mode == 1) begin
                            state = sqrt(1.0 + V(p, n) * V(p, n));
                        end
                    end
                end
            end
        end

        if (mode == 1) begin
            I(p, n) <+ state;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("known current-path conditions should not consume replay depth");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("unsupported_initial_state"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_snapshots_intrinsic_call_operands_for_history_replay() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module intrinsic_call_operand_history_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    real state, factor, beta;
    analog begin
        state = 0.0;
        factor = 0.0;
        beta = V(p, n);

        if (enable == 1) begin
            factor = exp(beta);
        end

        beta = $simparam("unused_later_beta");

        if (enable == 1) begin
            state = factor;
        end

        I(p, n) <+ state;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history replay should snapshot operands inside pure intrinsic calls");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_multiple_history_backed_intrinsic_operands() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module multiple_history_backed_intrinsic_operands(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    real state, factor, a, b;
    analog begin
        state = 0.0;
        factor = 0.0;
        a = $simparam("unsupported_initial_a");
        b = $simparam("unsupported_initial_b");

        if (enable == 1) begin
            a = V(p, n);
            b = 2.0 * V(p, n);
            factor = exp(a + b);
            state = factor;
        end

        I(p, n) <+ state;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history replay should snapshot multiple replayable intrinsic operands");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_snapshots_guard_condition_operands_for_history_replay() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guard_condition_operand_history_snapshot(p, n);
    inout p, n;
    electrical p, n;
    real state, limit;
    analog begin
        state = 0.0;
        limit = V(p, n);

        if (limit > 0.0) begin
            state = exp(V(p, n));
        end

        limit = $simparam("unused_later_limit");

        I(p, n) <+ state;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("history replay should snapshot operands used by guarded conditions");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_snapshots_concrete_alias_operands_for_history_replay() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module concrete_alias_operand_history_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real a, b, state;
    analog begin
        a = V(p, n);
        b = 2.0 * V(p, n);
        state = a + b;

        a = $simparam("unused_later_a");
        b = $simparam("unused_later_b");

        if (mode == 0) begin
            state = $simparam("inactive_state");
        end

        if (mode == 1) begin
            I(p, n) <+ state;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("alias history should replay from assignment-time operand snapshots");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_snapshots_non_target_operands_for_self_alias_history_replay() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module self_alias_operand_history_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real a, b, c, d, e, total;
    analog begin
        a = V(p, n);
        b = 2.0 * V(p, n);
        c = 3.0 * V(p, n);
        d = 4.0 * V(p, n);
        e = 5.0 * V(p, n);
        total = 1.0;
        total = total + a + b + c + d + e;

        a = $simparam("unused_later_a");
        b = $simparam("unused_later_b");
        c = $simparam("unused_later_c");
        d = $simparam("unused_later_d");
        e = $simparam("unused_later_e");

        if (mode == 0) begin
            total = $simparam("inactive_total");
        end

        if (mode == 1) begin
            I(p, n) <+ total;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("self alias history should replay from assignment-time operand snapshots");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_selective_mid_size_self_update_history() {
        let mut source = String::from(
            r#"
module selective_mid_size_history(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real pad, state;
    analog begin
        state = $simparam("unsupported_initial_state");
        state = (mode == 1) ? V(p, n) : state;
"#,
        );
        for _ in 0..3500 {
            source.push_str("        pad = (((pad + 1.0) + (2.0 * 3.0)) + (4.0 / 5.0));\n");
        }
        source.push_str(
            r#"
        I(p, n) <+ ((mode == 1) ? state : 0.0);
    end
endmodule
"#,
        );

        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .expect("canonical IR");
        assert!(
            artifact.hir.expressions.len() > 20_000,
            "fixture must exceed complete expanded-history threshold"
        );
        assert!(
            artifact.hir.statements.len() <= 5_000,
            "fixture must stay within selective history statement gate"
        );

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("selective current-path history should lower to scalar OptIR");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_replays_selective_mid_size_guarded_overwrites() {
        let mut source = String::from(
            r#"
module selective_mid_size_guarded_overwrites(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real pad, state;
    analog begin
        state = $simparam("unsupported_initial_state");
        if (mode == 1) begin
            if (V(p, n) > 0.0) begin
                state = sqrt(V(p, n));
            end else begin
                state = 0.0;
            end
        end
"#,
        );
        for _ in 0..3500 {
            source.push_str("        pad = (((pad + 1.0) + (2.0 * 3.0)) + (4.0 / 5.0));\n");
        }
        source.push_str(
            r#"
        if (mode == 1) begin
            I(p, n) <+ state * V(p, n);
        end
    end
endmodule
"#,
        );

        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .expect("canonical IR");
        assert!(
            artifact.hir.expressions.len() > 20_000,
            "fixture must exceed complete expanded-history expression threshold"
        );
        assert!(
            artifact.hir.statements.len() <= 5_000,
            "fixture must stay within selective history statement gate"
        );

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("selective guarded overwrites should lower to scalar OptIR");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_snapshots_selective_conditional_alias_history() {
        let mut source = String::from(
            r#"
module selective_conditional_alias_history(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real pad, left, right, alias, out;
    analog begin
        left = $simparam("unsupported_initial_left");
        right = $simparam("unsupported_initial_right");
        left = V(p, n);
        right = 2.0 * V(p, n);
        if (mode == 0) begin
            left = $simparam("inactive_left");
            right = $simparam("inactive_right");
        end
        alias = (mode == 1) ? left : right;
        out = alias;
"#,
        );
        for _ in 0..3500 {
            source.push_str("        pad = (((pad + 1.0) + (2.0 * 3.0)) + (4.0 / 5.0));\n");
        }
        source.push_str(
            r#"
        I(p, n) <+ ((mode == 1) ? out : 0.0);
    end
endmodule
"#,
        );

        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .expect("canonical IR");
        assert!(
            artifact.hir.expressions.len() > 20_000,
            "fixture must exceed complete expanded-history expression threshold"
        );
        assert!(
            artifact.hir.statements.len() <= 5_000,
            "fixture must stay within selective history statement gate"
        );

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("selective conditional aliases should snapshot replay operands");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_replays_selective_self_update_before_alias_snapshot() {
        let mut source = String::from(
            r#"
module selective_self_update_before_alias_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real pad, state, delta, saved;
    analog begin
        state = $simparam("unsupported_initial_state");
        state = (mode == 1) ? V(p, n) : state;
        delta = 2.0 * V(p, n);
        state = state + delta;
        saved = state;
        if (mode == 1) begin
            state = saved;
        end else begin
            state = -saved;
        end
        saved = $simparam("unused_later_saved");
"#,
        );
        for _ in 0..3500 {
            source.push_str("        pad = (((pad + 1.0) + (2.0 * 3.0)) + (4.0 / 5.0));\n");
        }
        source.push_str(
            r#"
        I(p, n) <+ ((mode == 1) ? state : 0.0);
    end
endmodule
"#,
        );

        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .expect("canonical IR");
        assert!(
            artifact.hir.expressions.len() > 20_000,
            "fixture must exceed complete expanded-history expression threshold"
        );
        assert!(
            artifact.hir.statements.len() <= 5_000,
            "fixture must stay within selective history statement gate"
        );

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("selective self-update history should replay through alias snapshots");

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
    }

    #[test]
    fn scalar_backend_coerces_boolean_assignment_history_to_real() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module boolean_real_assignment_history(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real state;
    analog begin
        state = $simparam("unsupported_initial_state");
        state = (mode == 1) ? (V(p, n) > 0.0) : state;
        I(p, n) <+ ((mode == 1) ? state : 0.0);
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("boolean history assigned to a real variable should be scalar-coerced");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_uses_binary_switch_range_for_guard_complements() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module binary_switch_guard_complements(p, n);
    inout p, n;
    electrical p, n;
    parameter integer flag = 1 from [0:1];
    real tmp;
    analog begin
        tmp = $simparam("unsupported_scalar_probe");
        if (flag == 0) begin
            tmp = $simparam("inactive_scalar_probe");
        end else begin
            tmp = V(p, n);
        end

        if (flag == 1) begin
            I(p, n) <+ tmp;
        end
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("binary switch complements should make the active guarded value visible");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_branch_local_reassigned_temp() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module branch_local_reassigned_temp(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 1 from [0:1];
    real tmp, out;
    analog begin
        tmp = $simparam("unsupported_scalar_probe");

        if (mode == 1) begin
            tmp = 2.0;
            out = tmp * V(p, n);
        end else begin
            out = 3.0;
        end

        I(p, n) <+ out;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("branch-local reassigned temp should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_inlines_single_use_pure_temporaries() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module inline_scalar_chain(p, n);
    inout p, n;
    electrical p, n;
    real a, b, c, d, e;
    analog begin
        a = V(p, n) + 1.0;
        b = a * 2.0;
        c = b - 3.0;
        d = c / 4.0;
        e = d + 5.0;
        I(p, n) <+ e;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("pure single-use arithmetic chain should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();
        let value_locals = stamp
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                line.starts_with("let v")
                    && line
                        .get("let v".len()..)
                        .is_some_and(|tail| tail.starts_with(|c: char| c.is_ascii_digit()))
            })
            .count();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(
            value_locals <= 3,
            "expected scalar inlining to keep temporary locals low, saw {value_locals}\n{stamp}"
        );
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_small_bounded_runtime_while() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module bounded_runtime_while(p, n);
    inout p, n;
    electrical p, n;
    real niter, x, xprev, term, out;
    analog begin
        niter = 0.0;
        x = 1.0;
        xprev = 1.0e6;
        while ((niter <= 4.0) && (abs(x - xprev) > 1.0e-12)) begin
            xprev = x;
            term = V(p, n) + niter;
            x = 0.5 * (xprev + term);
            niter = niter + 1.0;
        end
        out = x;
        I(p, n) <+ out;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("small bounded runtime while should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_hundred_step_guarded_runtime_while() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module hundred_step_runtime_while(p, n);
    inout p, n;
    electrical p, n;
    integer iter;
    real x, prev, delta;
    analog begin
        iter = 0;
        x = 1.0;
        delta = 1.0;
        while ((delta > 1.0e-12) && (iter <= 100)) begin
            prev = x;
            x = 0.5 * (x + V(p, n));
            delta = abs(x - prev);
            iter = iter + 1;
        end
        I(p, n) <+ x;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("hundred-step guarded while should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_emits_runtime_loop_for_large_guarded_while() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module large_guarded_runtime_while(p, n);
    inout p, n;
    electrical p, n;
    integer iter;
    real x, prev, delta;
    real t0, t1, t2, t3, t4, t5, t6, t7, t8, t9;
    real t10, t11, t12, t13, t14, t15, t16, t17, t18, t19;
    real t20, t21, t22, t23, t24;
    analog begin
        iter = 0;
        x = 1.0;
        delta = 1.0;
        while ((delta > 1.0e-12) && (iter <= 100)) begin
            prev = x;
            t0 = x + V(p, n);
            t1 = 0.5 * (t0 + x);
            t2 = 0.5 * (t1 + x);
            t3 = 0.5 * (t2 + x);
            t4 = 0.5 * (t3 + x);
            t5 = 0.5 * (t4 + x);
            t6 = 0.5 * (t5 + x);
            t7 = 0.5 * (t6 + x);
            t8 = 0.5 * (t7 + x);
            t9 = 0.5 * (t8 + x);
            t10 = 0.5 * (t9 + x);
            t11 = 0.5 * (t10 + x);
            t12 = 0.5 * (t11 + x);
            t13 = 0.5 * (t12 + x);
            t14 = 0.5 * (t13 + x);
            t15 = 0.5 * (t14 + x);
            t16 = 0.5 * (t15 + x);
            t17 = 0.5 * (t16 + x);
            t18 = 0.5 * (t17 + x);
            t19 = 0.5 * (t18 + x);
            t20 = 0.5 * (t19 + x);
            t21 = 0.5 * (t20 + x);
            t22 = 0.5 * (t21 + x);
            t23 = 0.5 * (t22 + x);
            t24 = 0.5 * (t23 + x);
            x = t24;
            delta = abs(x - prev);
            iter = iter + 1;
        end
        I(p, n) <+ x;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("large guarded while should lower to scalar runtime loop");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();
        let value_locals = stamp
            .lines()
            .filter(|line| line.trim_start().starts_with("let v"))
            .count();
        let runtime_loop_initializer_lines = stamp
            .lines()
            .filter(|line| line.trim_start().starts_with("let mut r0_"))
            .count();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(
            stamp.contains(";let mut r0_"),
            "runtime loop mutable locals should be packed onto shared source lines\n{stamp}"
        );
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(stamp.contains("while {"), "{stamp}");
        assert!(
            value_locals < 400,
            "runtime loop should not unroll into thousands of scalar locals, saw {value_locals}"
        );
        assert!(
            runtime_loop_initializer_lines < 16,
            "runtime loop initializers should stay compact, saw {runtime_loop_initializer_lines}\n{stamp}"
        );
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_counter_dependent_accumulator_loop() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module counted_accumulator(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc, term;
    analog begin
        acc = 0.0;
        for (i = 0; i < nf; i = i + 1) begin
            term = 1.0 / (10.0 + V(p, n) + i);
            acc = acc + term;
        end
        I(p, n) <+ acc;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("counter-dependent accumulator loop should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("while (counted_sum_"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_loop_invariant_product_accumulator() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module counted_product_accumulator(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real product, factor;
    analog begin
        product = 2.0;
        factor = 1.0 + 0.1 * V(p, n);
        for (i = 0; i < nf; i = i + 1) begin
            product = product * factor;
        end
        I(p, n) <+ product;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("loop-invariant product accumulator should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains(".powi("), "{stamp}");
        assert!(!stamp.contains("f64::powf("), "{stamp}");
        assert!(!stamp.contains("while {"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_guarded_loop_invariant_product_accumulator() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_counted_product_accumulator(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    parameter integer enable = 1 from [0:1];
    integer i;
    real product, factor;
    analog begin
        product = 2.0;
        factor = 1.0 + 0.1 * V(p, n);
        for (i = 0; (i < nf) && (enable != 0); i = i + 1) begin
            product = product * factor;
        end
        I(p, n) <+ product;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("guarded loop-invariant product accumulator should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains(".powi("), "{stamp}");
        assert!(!stamp.contains("f64::powf("), "{stamp}");
        assert!(!stamp.contains("while {"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_guarded_counter_dependent_accumulator_loop() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_counted_accumulator(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    parameter integer enable = 1 from [0:1];
    integer i;
    real acc, out, term;
    analog begin
        if (enable != 0) begin
            acc = 0.0;
            for (i = 0; i < nf; i = i + 1) begin
                term = 1.0 / (10.0 + V(p, n) + i);
                acc = acc + term;
            end
            out = acc;
        end else begin
            out = 0.0;
        end
        I(p, n) <+ out;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("guarded counter-dependent accumulator loop should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("while (counted_sum_"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_parameter_bounded_runtime_for_loop() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module parameter_bounded_runtime_for_loop(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc, filtered, term;
    analog begin
        acc = 0.0;
        filtered = 1.0;
        for (i = 0; i < nf; i = i + 1) begin
            term = V(p, n) + i;
            filtered = 0.5 * (filtered + term);
            acc = acc + filtered;
        end
        I(p, n) <+ acc;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("parameter-bounded recurrence loop should lower to a scalar runtime loop");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_defaults_overwritten_runtime_loop_locals() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module overwritten_runtime_loop_local(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc, filtered, term;
    analog begin
        acc = 0.0;
        filtered = 1.0;
        term = $simparam("unsupported_scalar_probe");
        for (i = 0; i < nf; i = i + 1) begin
            term = V(p, n) + i;
            filtered = 0.5 * (filtered + term);
            acc = acc + filtered;
        end
        I(p, n) <+ acc;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("overwritten loop locals should not require stale pre-loop scalar values");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_replays_guarded_runtime_loop_initializer() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module guarded_runtime_loop_initializer(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enable = 1 from [0:1];
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc, out, term;
    analog begin
        acc = $simparam("unsupported_initial_acc");
        term = $simparam("unsupported_initial_term");
        if (enable == 1) begin
            acc = 0.0;
            for (i = 0; i < nf; i = i + 1) begin
                term = V(p, n) + i;
                acc = acc + term;
            end
            out = acc;
        end else begin
            out = 0.0;
        end
        I(p, n) <+ out;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("runtime loop should replay the active guarded initializer");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_coerces_bool_runtime_loop_assignment_to_real() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module bool_to_real_runtime_loop_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc, enabled, term;
    analog begin
        acc = 0.0;
        enabled = 0.0;
        for (i = 0; i < nf; i = i + 1) begin
            enabled = (i == 0);
            term = enabled * V(p, n);
            acc = acc + term;
        end
        I(p, n) <+ acc;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("bool-to-real runtime loop assignment should lower to scalar OptIR");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(stamp.contains("}else{"), "{stamp}");
        assert!(!stamp.contains("{true}else{false}"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_parameter_bounded_inclusive_runtime_for_loop() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module parameter_bounded_inclusive_runtime_for_loop(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [1:inf);
    integer i;
    real filtered, term;
    analog begin
        filtered = 1.0;
        for (i = 1; i <= nf; i = i + 1) begin
            term = V(p, n) / (10.0 + i);
            filtered = 0.5 * (filtered + term);
        end
        I(p, n) <+ filtered;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("inclusive parameter-bounded loop should lower to a scalar runtime loop");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }

    #[test]
    fn scalar_backend_lowers_scalar_bounded_runtime_for_loop() {
        let artifact = crate::VerilogACompiler::default()
            .compile_canonical_ir(
                r#"
module scalar_bounded_runtime_for_loop(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 3 from [0:inf);
    integer i, limit;
    real filtered, term;
    analog begin
        filtered = 1.0;
        limit = (V(p, n) > 0.0) ? nf : nf + 1;
        for (i = 0; i < limit; i = i + 1) begin
            term = V(p, n) / (2.0 + i);
            filtered = 0.5 * (filtered + term);
        end
        I(p, n) <+ filtered;
    end
endmodule
"#,
            )
            .expect("canonical IR");

        let report = RustTranspiler::new_scalar(RustTranspileOptions::default())
            .transpile_with_report(&artifact)
            .expect("scalar-bounded loop should lower to a guarded runtime loop");

        let stamp = report
            .device
            .files
            .iter()
            .find(|file| file.relative_path == "stamp.rs")
            .expect("stamp file")
            .contents
            .as_str();

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(!stamp.contains("AdValue"), "{stamp}");
    }
}
