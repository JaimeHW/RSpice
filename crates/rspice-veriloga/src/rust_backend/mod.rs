mod builtins;
mod compact;
mod device;
mod discover;
mod error;
mod expr;
mod files;
mod manifest;
mod names;
mod registry;
mod scalar;

pub use builtins::{
    BuiltinBackendSelectionCounts, BuiltinGenerationReport, BuiltinSubsetGenerationReport,
    GENERATED_BUILTIN_MANIFEST_FILE_NAME, REGENERATE_BUILTINS_COMMAND,
    generate_generated_builtin_subset_with_progress, regenerate_generated_builtins,
    regenerate_generated_builtins_with_progress, validate_generated_builtins,
};
pub use device::render_runtime_support_module;
pub use discover::{
    VERILOGA_DISCOVERY_SKIP_MARKER, VerilogASourceCandidate, discover_veriloga_sources,
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
    ScalarHybrid,
    LegacyDevice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustDeviceReport {
    pub device: GeneratedRustDevice,
    pub backend: RustBackendSelection,
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

    pub fn options(&self) -> &RustTranspileOptions {
        &self.options
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
            RustBackendKind::Auto => match scalar::generate_device(artifact, &self.options) {
                Ok(device) => Ok(GeneratedRustDeviceReport {
                    device,
                    backend: RustBackendSelection::ScalarOptIr,
                }),
                Err(scalar_error) if scalar_error.is_unsupported() => {
                    match device::generate_hybrid_device(artifact, &self.options) {
                        Ok(device) => Ok(GeneratedRustDeviceReport {
                            device,
                            backend: RustBackendSelection::ScalarHybrid,
                        }),
                        Err(hybrid_error) if hybrid_error.is_unsupported() => Err(
                            auto_backend_unsupported(artifact, &scalar_error, &hybrid_error),
                        ),
                        Err(error) => Err(error),
                    }
                }
                Err(error) => Err(error),
            },
            RustBackendKind::Legacy => Ok(GeneratedRustDeviceReport {
                device: device::generate_device(artifact, &self.options)?,
                backend: RustBackendSelection::LegacyDevice,
            }),
            RustBackendKind::ScalarOptIr => Ok(GeneratedRustDeviceReport {
                device: scalar::generate_device(artifact, &self.options)?,
                backend: RustBackendSelection::ScalarOptIr,
            }),
        }
    }
}

fn auto_backend_unsupported(
    artifact: &CanonicalIrArtifact,
    scalar_error: &RustBackendError,
    hybrid_error: &RustBackendError,
) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        format!(
            "model cannot be lowered by scalar Rust backends; scalar path: {}; hybrid scalar path: {}",
            unsupported_detail(scalar_error),
            unsupported_detail(hybrid_error)
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
                .contains("model cannot be lowered by scalar Rust backends"),
            "{error}"
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
        assert!(stamp.contains("scalar_limexp(v"), "{stamp}");
        assert!(stamp.contains("scalar_limexp_derivative(v"), "{stamp}");
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

        assert_eq!(report.backend, RustBackendSelection::ScalarOptIr);
        assert!(stamp.contains("let mut r0_"), "{stamp}");
        assert!(stamp.contains("let mut r0g=0usize;"), "{stamp}");
        assert!(stamp.contains("while {"), "{stamp}");
        assert!(
            value_locals < 400,
            "runtime loop should not unroll into thousands of scalar locals, saw {value_locals}"
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
        assert!(stamp.contains("{1.0}else{0.0}"), "{stamp}");
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
