//! Exercise the same report contract with and without the optional emitter.

use rspice_veriloga::generated_source::{GeneratedRustDevice, GeneratedRustFile};
use rspice_veriloga::{
    CompilerOptions, RuntimeCompileReport, RuntimeQualificationOptions, RuntimeTarget,
    RuntimeTargetReadiness, VerilogACompiler,
};

const SOURCE: &str =
    "module resistor(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n)/1000; endmodule";

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions::default())
}

#[test]
fn ordinary_runtime_report_round_trips_without_emission() {
    let report = compiler().compile_runtime(SOURCE, None).unwrap();
    assert!(report.generated_rust.is_none());
    assert!(report.targets.is_available(RuntimeTarget::BytecodeVm));
    let encoded = serde_json::to_vec(&report).unwrap();
    let decoded: RuntimeCompileReport = serde_json::from_slice(&encoded).unwrap();
    decoded.validate_integrity().unwrap();
    assert_eq!(encoded, serde_json::to_vec(&decoded).unwrap());
}

#[test]
fn optional_generation_reports_the_compiled_capability() {
    let options = RuntimeQualificationOptions {
        generated_rust: true,
        ..RuntimeQualificationOptions::NONE
    };
    let report = compiler()
        .compile_runtime_with_qualifications(SOURCE, None, options)
        .unwrap();
    report.validate_integrity().unwrap();
    let target = report.targets.get(RuntimeTarget::GeneratedRust);
    if cfg!(feature = "rust-codegen") {
        assert_eq!(target.readiness, RuntimeTargetReadiness::Available);
        assert!(report.generated_rust.is_some());
    } else {
        assert_eq!(target.readiness, RuntimeTargetReadiness::Unavailable);
        assert!(target.detail.contains("rust-codegen"));
        assert!(report.generated_rust.is_none());
    }
    let encoded = serde_json::to_vec(&report).unwrap();
    let decoded: RuntimeCompileReport = serde_json::from_slice(&encoded).unwrap();
    decoded.validate_integrity().unwrap();
}

#[test]
fn required_generation_cannot_silently_fall_back() {
    let result = compiler().compile_runtime_with_qualifications(
        SOURCE,
        None,
        RuntimeQualificationOptions::GENERATED_RUST_REQUIRED,
    );
    if cfg!(feature = "rust-codegen") {
        assert!(result.unwrap().generated_rust.is_some());
    } else {
        assert!(result.unwrap_err().to_string().contains("rust-codegen"));
    }
}

#[test]
fn generated_artifact_format_is_available_without_the_emitter() {
    let artifact = GeneratedRustDevice {
        module_name: "resistor".into(),
        public_model_name: "Resistor".into(),
        folder_name: "resistor".into(),
        files: vec![GeneratedRustFile {
            relative_path: "mod.rs".into(),
            contents: "// generated source".into(),
        }],
        source_digest: "source-digest".into(),
        source_identity: "source-identity".into(),
        accepted_state_shape_identity: [42; 32],
    };
    let encoded = serde_json::to_vec(&artifact).unwrap();
    let decoded: GeneratedRustDevice = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(artifact, decoded);
    // Existing tooling paths remain aliases of the same data-only types.
    #[cfg(feature = "rust-codegen")]
    let _: rspice_veriloga::rust_backend::GeneratedRustDevice = decoded;
}

#[test]
fn optimized_runtime_backends_do_not_require_rust_source_generation() {
    let options = RuntimeQualificationOptions {
        native_x64_jit: cfg!(all(
            feature = "native",
            any(target_arch = "x86_64", target_arch = "aarch64"),
            any(target_os = "macos", target_os = "linux", windows)
        )),
        wasm_jit: cfg!(feature = "wasm-jit"),
        ..RuntimeQualificationOptions::NONE
    };
    let report = compiler()
        .compile_runtime_with_qualifications(SOURCE, None, options)
        .unwrap();
    assert!(report.generated_rust.is_none());
    if options.native_x64_jit {
        assert!(report.targets.is_available(RuntimeTarget::NativeJit));
    }
    #[cfg(feature = "wasm-jit")]
    {
        // A host-side compile does not qualify browser startup. Verify the
        // emitter directly without weakening that existing readiness policy.
        let artifact = rspice_veriloga::wasm_jit::compile_model_value_module(
            &report.model,
            &report.canonical_ir,
        )
        .unwrap();
        assert!(!artifact.module().bytes().is_empty());
        assert!(
            report
                .targets
                .get(RuntimeTarget::WasmJit)
                .detail
                .contains("browser startup")
        );
    }
    report.validate_integrity().unwrap();
}
