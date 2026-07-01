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
    LegacyNativeLocalFallback,
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
                        Err(hybrid_error) if hybrid_error.is_unsupported() => {
                            match device::generate_auto_device(artifact, &self.options) {
                                Ok(device) => {
                                    let device = reject_legacy_ad_device(artifact, device)?;
                                    Ok(GeneratedRustDeviceReport {
                                        device,
                                        backend: RustBackendSelection::LegacyNativeLocalFallback,
                                    })
                                }
                                Err(legacy_error) if legacy_error.is_unsupported() => {
                                    Err(auto_backend_unsupported(
                                        artifact,
                                        &scalar_error,
                                        &hybrid_error,
                                    ))
                                }
                                Err(error) => Err(error),
                            }
                        }
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

fn reject_legacy_ad_device(
    artifact: &CanonicalIrArtifact,
    device: GeneratedRustDevice,
) -> Result<GeneratedRustDevice, RustBackendError> {
    if let Some((relative_path, marker)) = generated_legacy_ad_marker(&device) {
        return Err(RustBackendError::unsupported(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            format!(
                "model requires legacy AD backend; generated {relative_path} contains {marker}"
            ),
        ));
    }
    Ok(device)
}

fn generated_legacy_ad_marker(device: &GeneratedRustDevice) -> Option<(&str, &'static str)> {
    const MARKERS: &[&str] = &[
        "GenericAdValue",
        "AdValue",
        "GenericScratch",
        "GenericReactiveScratch",
        "scratch:",
        "reactive_scratch:",
        "scratch.",
        "reactive_scratch.",
        "::support::",
    ];

    device.files.iter().find_map(|file| {
        MARKERS
            .iter()
            .copied()
            .find(|marker| file.contents.contains(marker))
            .map(|marker| (file.relative_path.as_str(), marker))
    })
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
            "model cannot be lowered without the legacy AD backend; scalar path: {}; hybrid scalar path: {}",
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
    fn generated_legacy_ad_marker_detects_ad_runtime_usage() {
        let device = generated_device("let eq0 = AdValue::log10(input);");

        assert_eq!(
            generated_legacy_ad_marker(&device),
            Some(("stamp.rs", "AdValue"))
        );
    }

    #[test]
    fn generated_legacy_ad_marker_allows_native_local_output() {
        let device = generated_device(
            "let v0: f64 = ctx.node_voltage(nodes[0]);\nstamper.stamp_current_node2_local();",
        );

        assert_eq!(generated_legacy_ad_marker(&device), None);
    }

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

    fn generated_device(stamp: &str) -> GeneratedRustDevice {
        GeneratedRustDevice {
            module_name: "fixture".to_string(),
            public_model_name: "fixture".to_string(),
            folder_name: "fixture__fixture__00000000".to_string(),
            source_digest: "0000000000000000".to_string(),
            files: vec![GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: stamp.to_string(),
            }],
        }
    }
}
