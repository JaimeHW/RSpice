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
    BuiltinBackendSelectionCounts, BuiltinGenerationReport, GENERATED_BUILTIN_MANIFEST_FILE_NAME,
    REGENERATE_BUILTINS_COMMAND, regenerate_generated_builtins,
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
}
