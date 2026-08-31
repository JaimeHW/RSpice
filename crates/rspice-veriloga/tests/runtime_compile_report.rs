use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

use rspice_veriloga::{
    CompileDiagnosticPhase, CompileDiagnosticSeverity, CompileError, CompileSourcePosition,
    CompilerOptions, PhaseTiming, PipelineControl, PipelineMetrics, PipelinePhase,
    RuntimeArtifactIntegrityError, RuntimeQualificationOptions, RuntimeTarget,
    RuntimeTargetMaturity, RuntimeTargetReadiness, VerilogACompiler, compile_diagnostics,
};

const SENSOR_BRIDGE_SOURCE: &str = "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule\n";

const ZI_STATE_SOURCE: &str = r#"
module zi_state(p, n);
  inout p, n;
  electrical p, n;
  real sampled;
  analog begin
    sampled = zi_nd(V(p, n), '{1.0, 0.25}, '{1.0, -0.5}, 1.0e-6, 0.0);
    I(p, n) <+ sampled;
  end
endmodule
"#;

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
    assert_eq!(targets.len(), 6);
    assert_eq!(
        targets
            .iter()
            .map(|entry| entry.target)
            .collect::<HashSet<_>>()
            .len(),
        6
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
    assert_eq!(rust.readiness, RuntimeTargetReadiness::Unavailable);
    assert!(rust.detail.contains("not requested"));
    assert!(report.generated_rust.is_none());

    let native = report.targets.get(RuntimeTarget::NativeJit);
    assert_eq!(native.maturity, RuntimeTargetMaturity::Preview);
    assert_eq!(native.readiness, RuntimeTargetReadiness::Unavailable);
    assert!(native.detail.contains("not requested"));

    let wasm_jit = report.targets.get(RuntimeTarget::WasmJit);
    assert_eq!(wasm_jit.maturity, RuntimeTargetMaturity::QualificationOnly);
    assert_eq!(wasm_jit.readiness, RuntimeTargetReadiness::Unavailable);
    assert!(wasm_jit.detail.contains("not requested"));
}

#[test]
fn optional_backend_qualification_is_explicit() {
    let report = compiler()
        .compile_runtime_with_qualifications(
            SENSOR_BRIDGE_SOURCE,
            None,
            RuntimeQualificationOptions::ALL,
        )
        .expect("compile and qualify workbench model");

    let rust = report.targets.get(RuntimeTarget::GeneratedRust);
    assert_eq!(rust.readiness, RuntimeTargetReadiness::Available);
    let generated = report
        .generated_rust
        .as_ref()
        .expect("requested generated Rust artifact");
    assert_eq!(
        generated.source_digest,
        report.canonical_ir.metadata.source_digest
    );
    assert!(!generated.files.is_empty());

    let native = report.targets.get(RuntimeTarget::NativeJit);
    #[cfg(not(feature = "native"))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Unavailable);
    #[cfg(all(feature = "native", target_arch = "x86_64"))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Available);
    #[cfg(all(
        feature = "native",
        target_arch = "aarch64",
        any(target_os = "macos", target_os = "linux", windows)
    ))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Available);
    #[cfg(all(
        feature = "native",
        not(any(
            target_arch = "x86_64",
            all(
                target_arch = "aarch64",
                any(target_os = "macos", target_os = "linux", windows)
            )
        ))
    ))]
    assert_eq!(native.readiness, RuntimeTargetReadiness::Unavailable);

    report
        .validate_integrity()
        .expect("explicit qualification remains coherent");
}

#[test]
fn required_generated_rust_never_silently_falls_back_to_the_interpreter() {
    let report = compiler()
        .compile_runtime_with_qualifications(
            SENSOR_BRIDGE_SOURCE,
            None,
            RuntimeQualificationOptions::GENERATED_RUST_REQUIRED,
        )
        .expect("the portable direct Rust backend must qualify this model");

    assert!(report.targets.is_available(RuntimeTarget::GeneratedRust));
    assert!(report.generated_rust.is_some());
}

#[test]
fn generated_rust_stateful_operator_qualification_is_explicit_and_fail_closed() {
    let cases = [
        (
            "Laplace",
            r#"
module generated_laplace(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
endmodule
"#,
            "laplace_nd",
        ),
        (
            "Zi",
            r#"
module generated_zi(p, n);
    inout p, n;
    electrical p, n;
    real sampled;
    analog begin
        sampled = zi_nd(V(p, n), '{1.0}, '{1.0}, 1.0e-6, 0.0);
        I(p, n) <+ sampled;
    end
endmodule
"#,
            "zi_nd",
        ),
        (
            "absdelay",
            r#"
module generated_absdelay(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 1.0e-6);
endmodule
"#,
            "absdelay",
        ),
        (
            "transition",
            r#"
module generated_transition(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 1.0e-6, 1.0e-6);
endmodule
"#,
            "transition",
        ),
        (
            "rate-limited slew",
            r#"
module generated_slew_limited(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 1.0, -1.0);
endmodule
"#,
            "rate-limited slew",
        ),
    ];
    let qualification_only = RuntimeQualificationOptions {
        generated_rust: true,
        ..RuntimeQualificationOptions::NONE
    };

    for (operator, source, expected_detail) in cases {
        let report = compiler()
            .compile_runtime_with_qualifications(source, None, qualification_only)
            .unwrap_or_else(|error| panic!("{operator} runtime compilation failed: {error}"));
        let generated = report.targets.get(RuntimeTarget::GeneratedRust);
        assert_eq!(
            generated.readiness,
            RuntimeTargetReadiness::Rejected,
            "{operator}: {generated:?}"
        );
        assert!(
            generated.detail.contains(expected_detail),
            "{operator}: {}",
            generated.detail
        );
        assert!(
            report.generated_rust.is_none(),
            "{operator} must not publish a partial generated artifact"
        );
        assert!(
            report.targets.is_available(RuntimeTarget::BytecodeVm),
            "{operator} remains executable by the production VM"
        );
        report
            .validate_integrity()
            .unwrap_or_else(|error| panic!("{operator} report integrity failed: {error}"));

        let required = report
            .enforce_fallback_policy(RuntimeQualificationOptions::GENERATED_RUST_REQUIRED)
            .expect_err("requiring rejected generated Rust must fail closed");
        assert_eq!(required.target, RuntimeTarget::GeneratedRust, "{operator}");
        assert_eq!(
            required.readiness,
            RuntimeTargetReadiness::Rejected,
            "{operator}"
        );
        assert!(required.detail.contains(expected_detail), "{operator}");
    }

    let passthrough = r#"
module generated_slew_passthrough(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n));
endmodule
"#;
    let report = compiler()
        .compile_runtime_with_qualifications(passthrough, None, qualification_only)
        .expect("one-argument slew must qualify for generated Rust");
    assert!(report.targets.is_available(RuntimeTarget::GeneratedRust));
    assert!(report.generated_rust.is_some());
    report
        .enforce_fallback_policy(RuntimeQualificationOptions::GENERATED_RUST_REQUIRED)
        .expect("required one-argument slew generated Rust remains available");
    report
        .validate_integrity()
        .expect("one-argument slew report remains coherent");
}

#[test]
fn required_native_backend_is_a_typed_fail_closed_contract() {
    let result = compiler().compile_runtime_with_qualifications(
        SENSOR_BRIDGE_SOURCE,
        None,
        RuntimeQualificationOptions::NATIVE_REQUIRED,
    );

    #[cfg(all(
        feature = "native",
        any(
            target_arch = "x86_64",
            all(
                target_arch = "aarch64",
                any(target_os = "macos", target_os = "linux", windows)
            )
        )
    ))]
    assert!(
        result
            .expect("supported native host must qualify the sample")
            .targets
            .is_available(RuntimeTarget::NativeJit)
    );

    #[cfg(not(all(
        feature = "native",
        any(
            target_arch = "x86_64",
            all(
                target_arch = "aarch64",
                any(target_os = "macos", target_os = "linux", windows)
            )
        )
    )))]
    {
        let error = result.expect_err("required native backend must fail closed");
        assert!(
            matches!(
                &error,
                CompileError::BackendQualification(qualification)
                    if qualification.target == RuntimeTarget::NativeJit
                        && qualification.readiness == RuntimeTargetReadiness::Unavailable
            ),
            "{error}"
        );
        assert!(
            error
                .to_string()
                .contains("interpreter fallback is disabled")
        );
        let diagnostics = compile_diagnostics(SENSOR_BRIDGE_SOURCE, &error);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].phase,
            CompileDiagnosticPhase::BackendQualification
        );
    }
}

#[test]
fn older_qualification_options_default_to_allowing_portable_fallback() {
    let options: RuntimeQualificationOptions =
        serde_json::from_str(r#"{"generated_rust":true,"native_x64_jit":false}"#)
            .expect("deserialize the previous options shape");

    assert_eq!(
        options.interpreter_fallback,
        rspice_veriloga::InterpreterFallbackPolicy::Allow
    );
}

#[test]
fn runtime_compile_reports_ordered_frontend_phase_metrics() {
    let report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, Some("sensor_bridge"))
        .expect("compile measured workbench source");

    for phase in [
        PipelinePhase::Preprocess,
        PipelinePhase::Lex,
        PipelinePhase::Parse,
        PipelinePhase::Semantic,
        PipelinePhase::BytecodeGeneration,
        PipelinePhase::HirLowering,
        PipelinePhase::MirLowering,
        PipelinePhase::CanonicalNoisePlanning,
        PipelinePhase::RuntimeQualification,
        PipelinePhase::IntegrityValidation,
    ] {
        assert!(
            report.metrics.has_phase(phase),
            "missing structured metric for {phase}"
        );
    }
    assert!(report.metrics.preprocessed_bytes > 0);
    assert!(report.metrics.token_count > 0);
    assert_eq!(report.metrics.module_count, 1);
    assert_eq!(
        report.metrics.total_elapsed_nanos,
        report
            .metrics
            .phases
            .iter()
            .map(|timing| timing.elapsed_nanos)
            .sum::<u64>()
    );
}

struct CancelAfterSemantic {
    cancelled: AtomicBool,
}

impl PipelineControl for CancelAfterSemantic {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    fn phase_completed(&self, timing: PhaseTiming, _metrics: &PipelineMetrics) {
        if timing.phase == PipelinePhase::Semantic {
            self.cancelled.store(true, Ordering::Relaxed);
        }
    }
}

#[test]
fn runtime_compile_honors_phase_observer_cancellation() {
    let control = CancelAfterSemantic {
        cancelled: AtomicBool::new(false),
    };
    let error = compiler()
        .compile_runtime_with_qualifications_and_control(
            SENSOR_BRIDGE_SOURCE,
            Some("sensor_bridge"),
            RuntimeQualificationOptions::NONE,
            &control,
        )
        .expect_err("observer cancellation must stop before bytecode generation");

    assert!(matches!(
        error,
        CompileError::Cancelled(cancelled)
            if cancelled.phase == PipelinePhase::BytecodeGeneration
    ));
}

#[test]
fn canonical_artifacts_exclude_removed_scalar_graph() {
    let runtime = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, None)
        .expect("compile production runtime artifact");
    let encoded = serde_json::to_value(&runtime.canonical_ir).expect("serialize runtime artifact");
    let fields = encoded.as_object().expect("runtime artifact object");
    assert!(!fields.contains_key("opt"));
    assert!(!fields.contains_key("opt_digest"));

    let canonical = compiler()
        .compile_canonical_ir(SENSOR_BRIDGE_SOURCE)
        .expect("compile canonical artifact");
    let encoded = serde_json::to_value(canonical).expect("serialize canonical artifact");
    let fields = encoded.as_object().expect("canonical artifact object");
    assert!(!fields.contains_key("opt"));
    assert!(!fields.contains_key("opt_digest"));
}

#[test]
fn canonical_artifacts_default_missing_dual_scope_metadata_to_false() {
    let canonical = compiler()
        .compile_canonical_ir(SENSOR_BRIDGE_SOURCE)
        .expect("compile canonical artifact");
    let mut encoded = serde_json::to_value(&canonical).expect("serialize canonical artifact");
    for section in ["hir", "mir"] {
        let parameters = encoded
            .get_mut(section)
            .and_then(|section| section.get_mut("parameters"))
            .and_then(serde_json::Value::as_array_mut)
            .expect("canonical parameter array");
        for parameter in parameters {
            parameter
                .as_object_mut()
                .expect("canonical parameter object")
                .remove("also_model");
        }
    }

    let decoded: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_value(encoded).expect("deserialize pre-dual-scope canonical artifact");
    assert!(
        decoded
            .hir
            .parameters
            .iter()
            .all(|parameter| !parameter.also_model)
    );
    assert!(
        decoded
            .mir
            .parameters
            .iter()
            .all(|parameter| !parameter.also_model)
    );
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
fn runtime_report_deserialization_rejects_malformed_zi_history() {
    let report = compiler()
        .compile_runtime(ZI_STATE_SOURCE, Some("zi_state"))
        .expect("compile Zi runtime report");
    let mut encoded = serde_json::to_value(report).expect("serialize Zi runtime report");
    encoded["model"]["zi_filters"][0]["x_hist"] = serde_json::json!([0.0]);

    let error = serde_json::from_value::<rspice_veriloga::RuntimeCompileReport>(encoded)
        .expect_err("malformed compiled-model Zi history must fail deserialization");
    assert!(
        error.to_string().contains("input history length"),
        "got: {error}"
    );
}

#[test]
fn integrity_validation_reports_the_compiled_zi_filter_index() {
    let mut report = compiler()
        .compile_runtime(ZI_STATE_SOURCE, Some("zi_state"))
        .expect("compile Zi runtime report");
    let filter = report
        .model
        .zi_filters
        .get_mut(0)
        .expect("Zi model owns one filter placeholder");
    filter.eval(1.0, 0.0, true).unwrap();
    filter.commit(0.0).unwrap();

    let error = report
        .validate_integrity()
        .expect_err("dirty state beneath an unfrozen definition must fail integrity");
    assert!(matches!(
        error,
        RuntimeArtifactIntegrityError::InvalidZiFilterState { index: 0, .. }
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
        .compile_runtime_with_qualifications(
            SENSOR_BRIDGE_SOURCE,
            None,
            RuntimeQualificationOptions::ALL,
        )
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
fn discarded_system_tasks_warn_on_a_compile_that_still_succeeds() {
    let source = "module chatty(p, n);\n  inout p, n; electrical p, n;\n  analog begin\n    $display(\"one\");\n    $strobe(\"two\");\n    I(p, n) <+ V(p, n);\n  end\nendmodule\n";
    let report = compiler()
        .compile_runtime(source, Some("chatty"))
        .expect("a discarded system task must not fail the compile");

    report
        .validate_integrity()
        .expect("a report carrying warnings is still a coherent artifact");
    assert_eq!(report.abi.equation_count, 1);

    assert_eq!(report.diagnostics.len(), 2);
    for diagnostic in &report.diagnostics {
        assert_eq!(diagnostic.severity, CompileDiagnosticSeverity::Warning);
        assert_eq!(diagnostic.phase, CompileDiagnosticPhase::Semantic);
        assert_eq!(diagnostic.code, "VA-SEM-NO-EFFECT-SYSTEM-TASK");
    }

    let display = &report.diagnostics[0];
    assert!(display.message.contains("$display"));
    assert!(display.message.contains("parsed and discarded"));
    let span = display
        .span
        .as_ref()
        .expect("a discarded task is pinned to its call");
    assert_eq!(
        span.byte_start,
        u32::try_from(source.find("$display").unwrap()).unwrap()
    );
    assert_eq!(
        span.start,
        Some(CompileSourcePosition { line: 4, column: 5 })
    );
    assert!(report.diagnostics[1].message.contains("$strobe"));
}

#[test]
fn a_source_without_discarded_constructs_carries_no_warnings() {
    let report = compiler()
        .compile_runtime(SENSOR_BRIDGE_SOURCE, Some("sensor_bridge"))
        .expect("the exact workbench source must compile");

    assert!(report.diagnostics.is_empty());
}

#[test]
fn generate_regions_are_refused_by_name_with_their_own_code() {
    for (keyword, source) in [
        (
            "genvar",
            "module tapped(p, n);\n  inout p, n; electrical p, n;\n  genvar k;\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        ),
        (
            "generate",
            "module tapped(p, n);\n  inout p, n; electrical p, n;\n  generate\n  endgenerate\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        ),
        (
            "endgenerate",
            "module tapped(p, n);\n  inout p, n; electrical p, n;\n  endgenerate\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        ),
    ] {
        let error = compiler()
            .compile_runtime(source, Some("tapped"))
            .expect_err("generate regions have no elaborator");
        let diagnostics = compile_diagnostics(source, &error);

        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, CompileDiagnosticSeverity::Error);
        assert_eq!(diagnostic.phase, CompileDiagnosticPhase::Parser);
        assert_eq!(diagnostic.code, "VA-PARSE-UNSUPPORTED-GENERATE");
        assert!(diagnostic.message.contains(keyword));
        assert!(
            diagnostic
                .message
                .contains("generate regions are not supported")
        );
        let span = diagnostic
            .span
            .as_ref()
            .expect("the refusal names the construct's own span");
        let byte_start = u32::try_from(source.find(keyword).unwrap()).unwrap();
        assert_eq!(span.byte_start, byte_start);
        assert_eq!(
            span.byte_end,
            byte_start + u32::try_from(keyword.len()).unwrap()
        );
        assert_eq!(
            span.start,
            Some(CompileSourcePosition { line: 3, column: 3 })
        );
    }
}

#[test]
fn every_compile_failure_carries_a_compiler_owned_code() {
    for source in [
        "module broken(p, n);\n  analog I(p, n) <+ α;\nendmodule\n",
        "module first; endmodule\nmodule second; endmodule\n",
        "module missing(p, n);\n  inout p, n; electrical p, n;\n  analog I(p, n) <+ nowhere;\nendmodule\n",
    ] {
        let error = compiler()
            .compile_runtime(source, None)
            .expect_err("each source is rejected");
        let diagnostics = compile_diagnostics(source, &error);
        assert!(!diagnostics.is_empty());
        for diagnostic in diagnostics {
            assert!(
                diagnostic.code.starts_with("VA-"),
                "diagnostic code '{}' is not compiler-owned",
                diagnostic.code
            );
            assert_eq!(diagnostic.severity, CompileDiagnosticSeverity::Error);
        }
    }
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
