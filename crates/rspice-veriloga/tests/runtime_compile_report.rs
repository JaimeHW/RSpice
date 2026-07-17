use std::collections::HashSet;

use rspice_veriloga::{
    CompileDiagnosticPhase, CompileError, CompileSourcePosition, CompilerOptions,
    RuntimeArtifactIntegrityError, RuntimeTarget, RuntimeTargetMaturity, RuntimeTargetReadiness,
    VerilogACompiler, compile_diagnostics,
};

const SENSOR_BRIDGE_SOURCE: &str = "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule\n";

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions::default())
}

#[test]
fn exact_workbench_sample_compiles_to_coherent_runtime_artifacts() {
    let report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, Some("sensor_bridge"))
        .expect("the exact workbench source must compile");

    assert_eq!(report.model.name.as_str(), "sensor_bridge");
    assert_eq!(
        report.canonical_ir.hir.module_name.as_str(),
        "sensor_bridge"
    );
    assert_eq!(
        report.model.source_digest,
        report.canonical_ir.metadata.source_digest
    );
    report
        .validate_integrity()
        .expect("artifacts remain coherent");

    assert_eq!(report.abi.module_name.as_str(), "sensor_bridge");
    assert_eq!(report.abi.analog_port_count(), 3);
    assert_eq!(
        report
            .abi
            .analog_ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>(),
        ["out", "inp", "inn"]
    );
    assert!(
        report
            .abi
            .analog_ports
            .iter()
            .all(|port| port.discipline.as_str() == "electrical")
    );
    assert_eq!(report.abi.parameters.len(), 1);
    assert_eq!(report.abi.parameters[0].name.as_str(), "gain");
    assert_eq!(report.abi.parameters[0].default, Some(100.0));
    assert_eq!(report.abi.noise_source_count, 0);
    assert_eq!(report.abi.state_variable_count, 0);
    assert_eq!(report.abi.internal_node_count, 0);
    assert_eq!(report.abi.equation_count, 1);
}

#[test]
fn target_matrix_is_exhaustive_and_truthful() {
    let report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, None)
        .expect("compile workbench model");
    let targets = report.targets.all();
    assert_eq!(targets.len(), 5);
    assert_eq!(
        targets
            .iter()
            .map(|entry| entry.target)
            .collect::<HashSet<_>>()
            .len(),
        5
    );

    for target in [
        RuntimeTarget::SemanticIr,
        RuntimeTarget::BytecodeVm,
        RuntimeTarget::WasmInterpreter,
    ] {
        let qualification = report.targets.get(target);
        assert_eq!(qualification.readiness, RuntimeTargetReadiness::Available);
        assert_eq!(qualification.maturity, RuntimeTargetMaturity::Production);
        assert!(!qualification.detail.is_empty());
    }

    let rust = report.targets.get(RuntimeTarget::GeneratedRust);
    assert_eq!(rust.maturity, RuntimeTargetMaturity::QualificationOnly);
    assert_eq!(rust.is_available(), report.generated_rust.is_some());
    if let Some(generated) = &report.generated_rust {
        assert_eq!(
            generated.device.source_digest,
            report.canonical_ir.metadata.source_digest
        );
        assert!(!generated.device.files.is_empty());
        assert!(
            generated
                .device
                .files
                .iter()
                .all(|file| !file.relative_path.is_empty() && !file.contents.is_empty())
        );
    }

    let native = report.targets.get(RuntimeTarget::NativeX64Jit);
    assert_eq!(native.maturity, RuntimeTargetMaturity::Preview);
    assert!(!native.detail.is_empty());
    #[cfg(not(feature = "native"))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Unavailable);
    #[cfg(all(feature = "native", target_arch = "x86_64"))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Available);
    #[cfg(all(feature = "native", not(target_arch = "x86_64")))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Unavailable);
}

#[test]
fn integrity_validation_rejects_cross_artifact_digest_drift() {
    let mut report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, None)
        .expect("compile workbench model");
    report.model.source_digest = "0000000000000000".into();

    let error = report
        .validate_integrity()
        .expect_err("mutated model digest must fail closed");
    assert!(matches!(
        error,
        RuntimeArtifactIntegrityError::SourceDigestMismatch {
            artifact: "compiled model",
            ..
        }
    ));
}

#[test]
fn integrity_validation_rejects_abi_and_qualification_drift() {
    let mut report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, None)
        .expect("compile workbench model");
    report.abi.equation_count += 1;
    assert_eq!(
        report.validate_integrity(),
        Err(RuntimeArtifactIntegrityError::AbiSurfaceMismatch)
    );

    let mut report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, None)
        .expect("compile workbench model");
    assert!(report.generated_rust.take().is_some());
    assert!(matches!(
        report.validate_integrity(),
        Err(
            RuntimeArtifactIntegrityError::GeneratedRustQualificationMismatch {
                qualified: true,
                artifact_present: false,
            }
        )
    ));
}

#[test]
fn requested_module_is_used_for_both_runtime_artifacts() {
    let source = r#"
module first(p, n);
  analog I(p, n) <+ V(p, n);
endmodule
module second(p, n);
  analog I(p, n) <+ 2.0 * V(p, n);
endmodule
"#;
    let report = compiler()
        .compile_runtime(source, Some("second"))
        .expect("select the second module");

    assert_eq!(report.model.name.as_str(), "second");
    assert_eq!(report.canonical_ir.hir.module_name.as_str(), "second");
    assert_eq!(report.abi.module_name.as_str(), "second");
    report
        .validate_integrity()
        .expect("selected artifacts agree");
}

#[test]
fn utf8_diagnostic_preserves_bytes_and_reports_character_column() {
    let source = "module broken(p, n);\n  analog I(p, n) <+ α;\nendmodule\n";
    let error = compiler()
        .compile_runtime(source, None)
        .expect_err("non-ASCII identifier is outside the supported lexer grammar");
    let diagnostics = compile_diagnostics(source, &error);

    assert_eq!(diagnostics.len(), 1);
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic.phase, CompileDiagnosticPhase::Lexer);
    let span = diagnostic.span.as_ref().expect("lexer errors carry a span");
    let byte_start = u32::try_from(source.find('α').unwrap()).unwrap();
    assert_eq!(span.byte_start, byte_start);
    assert_eq!(span.byte_end, byte_start + 2);
    assert_eq!(
        span.start,
        Some(CompileSourcePosition {
            line: 2,
            column: 21,
        })
    );
    assert_eq!(
        span.end,
        Some(CompileSourcePosition {
            line: 2,
            column: 22,
        })
    );
    assert!(diagnostic.message.contains("Unexpected character"));
}

#[test]
fn spanless_module_selection_errors_still_produce_typed_diagnostics() {
    let source = "module first; endmodule\nmodule second; endmodule\n";
    let error = compiler()
        .compile_runtime(source, None)
        .expect_err("multi-module source requires a selection");
    let diagnostics = compile_diagnostics(source, &error);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].phase,
        CompileDiagnosticPhase::ModuleSelection
    );
    assert!(diagnostics[0].span.is_none());
    assert!(diagnostics[0].message.contains("first, second"));
}

#[test]
fn in_memory_runtime_compile_never_reads_configured_include_paths() {
    let mut options = CompilerOptions::default();
    options
        .include_paths
        .push(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let compiler = VerilogACompiler::new(options);
    let error = compiler
        .compile_runtime("`include \"Cargo.toml\"\nmodule x; endmodule\n", None)
        .expect_err("runtime source compilation must not read disk includes");

    assert!(matches!(
        error,
        CompileError::IoError { message }
            if message.contains("Include file not found: Cargo.toml")
    ));
}
