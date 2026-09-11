//! Opt-in executable compilation of both halves of one mixed module.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const MIXED: &str = r#"
module mixed(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  initial q = 1'b0;
  always @(posedge clk) q <= ~q;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn connection_closure_is_retained_validated_and_reused_for_specialization() {
    use rspice_veriloga::canonical_ir::{CanonicalConnectionContext, CanonicalIrArtifact};

    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = format!(
        "nature ProbeQuantity; units=\"V\"; access=Probe; abstol=1e-7; endnature\ndiscipline sensing; potential ProbeQuantity; enddiscipline\nmodule sized(q); parameter integer WIDTH=2; output [WIDTH-1:0] q; reg [WIDTH-1:0] q; initial q=1; endmodule\n{}",
        rspice_veriloga::connect::library::builtin_connect_library_source()
    );
    let runtime = compiler.compile_runtime(&source, None).unwrap();
    assert!(
        runtime.canonical_ir.parameter_source.is_none(),
        "the closure is stored once"
    );
    let encoded = serde_json::to_value(&runtime.canonical_ir).unwrap();
    let artifact: CanonicalIrArtifact = serde_json::from_value(encoded.clone()).unwrap();
    artifact.validate().unwrap();
    assert!(
        artifact
            .connections
            .source()
            .unwrap()
            .contains("connectrules")
    );
    let specification = compiler
        .connect_specification_from_preprocessed(artifact.connections.source().unwrap())
        .unwrap();
    assert_eq!(
        specification.disciplines.natures["ProbeQuantity"].abstol,
        1e-7
    );
    assert_eq!(
        specification.disciplines.disciplines["sensing"]
            .potential
            .as_deref(),
        Some("ProbeQuantity")
    );
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact,
            &[("WIDTH", 4.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    assert_eq!(specialized.canonical_ir.digital.signals[0].width, 4);
    assert_eq!(specialized.canonical_ir.connections, artifact.connections);

    let mut missing = encoded;
    missing.as_object_mut().unwrap().remove("connections");
    assert!(serde_json::from_value::<CanonicalIrArtifact>(missing).is_err());
    let mut lost = artifact.clone();
    lost.connections = CanonicalConnectionContext::None;
    assert!(
        lost.validate()
            .unwrap_err()
            .iter()
            .any(|error| { error.to_string().contains("stored connection identity") })
    );
    let mut altered = artifact;
    altered.connections = CanonicalConnectionContext::Source(
        altered
            .connections
            .source()
            .unwrap()
            .replace("WIDTH=2", "WIDTH=3")
            .into(),
    );
    assert!(
        altered
            .validate()
            .unwrap_err()
            .iter()
            .any(|error| { error.to_string().contains("connection elaboration source") })
    );
}

#[test]
fn mixed_parameter_specialization_survives_serialization_and_changes_port_shape() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = "module sized(q); parameter integer WIDTH=2; output [WIDTH-1:0] q; reg [WIDTH-1:0] q; initial q=1; endmodule";
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let bytes = serde_json::to_vec(&runtime.canonical_ir).unwrap();
    let artifact = serde_json::from_slice(&bytes).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact,
            &[("WIDTH", 4.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    specialized.validate_integrity().unwrap();
    assert_eq!(specialized.canonical_ir.digital.signals[0].width, 4);
    assert_eq!(runtime.canonical_ir.digital.signals[0].width, 2);
    assert_ne!(
        runtime.canonical_ir.digital_identity,
        specialized.canonical_ir.digital_identity
    );
    for parameters in [
        vec![("missing", 1.0)],
        vec![("WIDTH", f64::NAN)],
        vec![("WIDTH", 2.0), ("WIDTH", 3.0)],
    ] {
        assert!(
            compiler
                .specialize_mixed_runtime(
                    &artifact,
                    &parameters,
                    &rspice_veriloga::NoPipelineControl
                )
                .is_err()
        );
    }
    let mut corrupted = runtime.canonical_ir;
    corrupted.parameter_source = Some(source.replace("WIDTH=2", "WIDTH=3").into());
    assert!(corrupted.validate().is_err());
}

#[test]
fn shared_discrete_inputs_reject_dual_writers_and_unrepresentable_widths() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    for (source, expected) in [
        (
            "module dual(p); inout p; electrical p; reg state; initial state=0; analog begin state=1; I(p)<+state; end endmodule",
            "cannot be written by the analog body",
        ),
        (
            "module wide(p); inout p; electrical p; reg [63:0] state; initial state=0; analog I(p)<+state; endmodule",
            "32-bit signed integer range",
        ),
    ] {
        let error = compiler
            .compile_runtime(source, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn mixed_runtime_compilation_is_explicit_and_retains_both_domains() {
    let refused = VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime(MIXED, None)
        .expect_err("ordinary analog runtime compilation must remain fail-closed");
    assert_eq!(
        refused.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );

    let runtime = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
    .compile_runtime(MIXED, None)
    .expect("an opted-in mixed host owns digital execution");
    assert_eq!(runtime.model.stamp_programs.len(), 1);
    assert_eq!(runtime.canonical_ir.mir.equations.len(), 1);
    assert_eq!(runtime.canonical_ir.digital.processes.len(), 2);
}

#[test]
fn malformed_or_unsupported_mixed_content_still_fails_closed() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    });
    assert!(
        compiler
            .compile_runtime("module broken(p); always @( endmodule", None)
            .is_err()
    );

    let shared_crossing = r#"
module unsupported(p, n, q);
  inout p, n; electrical p, n;
  output q; reg q; real shared;
  always @(q) q = shared;
  analog begin shared = V(p, n); I(p, n) <+ shared; end
endmodule
"#;
    let error = compiler
        .compile_runtime(shared_crossing, None)
        .expect_err("implicit real-to-four-state crossing has no bridge contract");
    assert!(
        error.to_string().contains("analog") || error.to_string().contains("continuous-domain"),
        "{error}"
    );
}
