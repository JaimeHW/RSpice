//! Exact scalar source specialization and its numeric ABI boundary.
use rspice_veriloga::canonical_ir::digital::DigitalInitialValue;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::{CompilerOptions, NoPipelineControl, ScalarParameterValue, VerilogACompiler};

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
}
fn bits(raw: &str) -> FourStateValue {
    FourStateValue::from_literal(&rspice_veriloga::four_state::decode(raw).unwrap())
}
fn initial(report: &rspice_veriloga::RuntimeCompileReport, name: &str) -> DigitalInitialValue {
    report
        .canonical_ir
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap()
        .initial_value
        .clone()
        .unwrap()
}

#[test]
fn wide_values_survive_specialization_serialization_and_linking() {
    use rspice_veriloga::canonical_ir::digital_link::{DigitalLinkInstance, link_digital_plans};
    let compiler = compiler();
    let runtime = compiler.compile_runtime(
        "module packed(q); parameter PATTERN=1'b0; parameter real GAIN=2; \
         aliasparam BITS=PATTERN; aliasparam STRENGTH=GAIN; output reg [128:0] q=PATTERN; endmodule", None
    ).unwrap();
    let pattern = bits("129'h1_00000000_00000000_00000000_000000xz");
    let specialized = compiler
        .specialize_mixed_runtime_typed(
            &runtime.canonical_ir,
            &[(
                "BITS",
                ScalarParameterValue::Bits {
                    value: pattern.clone(),
                    signed: false,
                },
            )],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(
        initial(&specialized, "q"),
        DigitalInitialValue::FourState(pattern.clone())
    );
    assert_eq!(specialized.model.parameters.len(), 1);
    assert_eq!(specialized.model.parameters[0].name, "GAIN");
    assert_eq!(specialized.model.parameters[0].aliases, ["STRENGTH"]);
    let exact = &specialized.abi.elaboration_parameters[0];
    assert_eq!(exact.name, "PATTERN");
    assert_eq!(exact.aliases, ["BITS"]);
    assert_eq!(exact.value, pattern);
    assert!(!exact.signed);
    let encoded = serde_json::to_vec(&specialized).unwrap();
    let decoded: rspice_veriloga::RuntimeCompileReport = serde_json::from_slice(&encoded).unwrap();
    decoded.validate_integrity().unwrap();
    assert_eq!(decoded.abi, specialized.abi);
    let linked = link_digital_plans(
        &[
            DigitalLinkInstance {
                name: "a",
                plan: &decoded.canonical_ir.digital,
                ports: &[],
            },
            DigitalLinkInstance {
                name: "b",
                plan: &decoded.canonical_ir.digital,
                ports: &[],
            },
        ],
        &[],
        &NoPipelineControl,
    )
    .unwrap();
    linked.plan.validate().unwrap();
    let exact = &linked.plan.elaboration_parameters;
    assert_eq!(exact.len(), 2);
    assert_eq!(exact[0].name, "a.PATTERN");
    assert_eq!(exact[0].aliases, ["a.BITS"]);
    assert_eq!(exact[1].name, "b.PATTERN");
    assert_eq!(exact[1].value, pattern);
    assert!(!exact[0].is_public);

    let mut damaged = decoded.clone();
    damaged.canonical_ir.digital.elaboration_parameters[0].signed = true;
    assert!(damaged.validate_integrity().is_err());
    let mut damaged = decoded.clone();
    damaged.abi.elaboration_parameters[0].value = FourStateValue::zero(129);
    assert!(damaged.validate_integrity().is_err());
    // Only exact parameters remain in this source: there is no numeric slot
    // whose presence could accidentally keep the source closure alive.
    let only = compiler
        .compile_runtime(
            "module exact(q); parameter P=64'h20000000000001; output reg [63:0] q=P; endmodule",
            None,
        )
        .unwrap();
    assert!(only.model.parameters.is_empty());
    assert!(only.canonical_ir.parameter_source.is_some());
    let again = compiler
        .specialize_mixed_runtime_typed(
            &only.canonical_ir,
            &[("P", ScalarParameterValue::Integer(i64::MAX))],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(
        initial(&again, "q"),
        DigitalInitialValue::FourState(FourStateValue::from_integer(64, i64::MAX.into()))
    );
}

#[test]
fn typed_assignments_keep_integer_precision_signedness_and_real_zero() {
    let compiler = compiler();
    let runtime = compiler
        .compile_runtime(
            "module exact(q); parameter P=0; output reg [63:0] q=P; endmodule",
            None,
        )
        .unwrap();
    for (value, expected) in [
        (
            ScalarParameterValue::Integer(9_007_199_254_740_993),
            FourStateValue::from_integer(64, 9_007_199_254_740_993),
        ),
        (
            ScalarParameterValue::Integer(i64::MIN),
            FourStateValue::from_integer(64, i64::MIN.into()),
        ),
        (
            ScalarParameterValue::Bits {
                value: bits("8'hff"),
                signed: true,
            },
            bits("64'hffffffffffffffff"),
        ),
        (
            ScalarParameterValue::Bits {
                value: bits("8'hff"),
                signed: false,
            },
            bits("64'hff"),
        ),
    ] {
        let specialized = compiler
            .specialize_mixed_runtime_typed(
                &runtime.canonical_ir,
                &[("P", value)],
                &NoPipelineControl,
            )
            .unwrap();
        assert_eq!(
            initial(&specialized, "q"),
            DigitalInitialValue::FourState(expected)
        );
    }
    let runtime = compiler.compile_runtime(
        "module converted(q); parameter integer COUNT=1; parameter real LEVEL=0; parameter WIDTH=2; \
         output reg [WIDTH-1:0] q=COUNT; real sample=LEVEL; initial sample=LEVEL; endmodule", None
    ).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime_typed(
            &runtime.canonical_ir,
            &[
                ("COUNT", ScalarParameterValue::Real(7.5)),
                ("LEVEL", ScalarParameterValue::Real(-0.0)),
                ("WIDTH", ScalarParameterValue::Integer(8)),
            ],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(
        initial(&specialized, "q"),
        DigitalInitialValue::FourState(bits("8'd8"))
    );
    let DigitalInitialValue::Real(value) = initial(&specialized, "sample") else {
        panic!("real")
    };
    assert_eq!(value.to_bits(), (-0.0f64).to_bits());
    specialized.validate_integrity().unwrap();
}

#[test]
fn typed_overrides_refuse_invalid_inputs_and_ambiguous_aliases() {
    let compiler = compiler();
    let runtime = compiler
        .compile_runtime(
            "module exact(q); parameter P=0; aliasparam BITS=P; output reg q=P; endmodule",
            None,
        )
        .unwrap();
    for parameters in [
        vec![("P", ScalarParameterValue::Real(f64::NAN))],
        vec![("P", ScalarParameterValue::Real(f64::INFINITY))],
        vec![("missing", ScalarParameterValue::Integer(1))],
        vec![
            ("P", ScalarParameterValue::Integer(1)),
            ("BITS", ScalarParameterValue::Integer(2)),
        ],
        vec![(
            "P",
            ScalarParameterValue::Bits {
                value: FourStateValue::zero(0),
                signed: false,
            },
        )],
        vec![(
            "P",
            ScalarParameterValue::Bits {
                value: FourStateValue::zero(65_537),
                signed: false,
            },
        )],
    ] {
        assert!(
            compiler
                .specialize_mixed_runtime_typed(
                    &runtime.canonical_ir,
                    &parameters,
                    &NoPipelineControl
                )
                .is_err()
        );
    }
}

#[test]
fn packed_values_never_fall_through_to_numeric_slots_or_builtins() {
    let compiler = compiler();
    for body in [
        "analog I(p)<+P;",
        "analog I(p)<+$param_given(P);",
        "parameter real OTHER=1 from [P:inf]; analog I(p)<+OTHER;",
    ] {
        let source = format!(
            "module rejected(p,q); inout p; electrical p; parameter P=64'h20000000000001; output reg q=0; {body} endmodule"
        );
        let error = compiler
            .compile_runtime(&source, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("packed parameter 'P'"), "{error}");
    }
    let source = "module builtin(p,q); inout p; electrical p; parameter M_PI=64'h20000000000001; \
                  output reg q=0; analog I(p)<+M_PI; endmodule";
    let error = compiler
        .compile_runtime(source, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("packed parameter 'M_PI'"), "{error}");
    let source = "module constrained(q); parameter P=64'h20000000000001 from [0:inf]; output reg q=0; endmodule";
    let error = compiler
        .compile_runtime(source, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("typed range validation"), "{error}");
}

#[test]
fn dependent_exact_parameters_preserve_chains_selections_and_numeric_updates() {
    let compiler = compiler();
    let source = r#"
module dependent(p,q);
 inout p; electrical p;
 parameter PATTERN=129'h1000000000000000000000000000101xz;
 parameter COPY=PATTERN;
 parameter MASKED=COPY & 129'h1_ffffffff_ffffffff_ffffffff_ffffff00;
 parameter LOW=COPY[15:8];
 parameter real NEXT=LOW+1;
 parameter real BASE=2.5;
 parameter real RESULT=BASE+(COPY[15:8]+8'd255);
 output reg [128:0] q=MASKED;
 analog I(p)<+RESULT+NEXT;
endmodule
"#;
    let runtime = compiler.compile_runtime(source, None).unwrap();
    runtime.validate_integrity().unwrap();
    assert_eq!(
        initial(&runtime, "q"),
        DigitalInitialValue::FourState(bits("129'h100000000000000000000000000010100"))
    );
    assert_eq!(
        runtime
            .abi
            .elaboration_parameters
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["PATTERN", "COPY", "MASKED"]
    );
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "x",
        runtime.model.clone(),
        &runtime.canonical_ir,
        &[1],
    )
    .unwrap();
    // The packed sum wraps at 8 bits before BASE's real addition.
    assert_eq!(device.try_evaluate().unwrap()[0], 4.5);
    device.try_set_parameter("BASE", 10.5).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 12.5);
    device.try_set_parameter("LOW", 4.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 15.5);
    let specialized = compiler
        .specialize_mixed_runtime_typed(
            &runtime.canonical_ir,
            &[(
                "PATTERN",
                ScalarParameterValue::Bits {
                    value: bits("129'h1000000000000000000000000000102xz"),
                    signed: false,
                },
            )],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(
        initial(&specialized, "q"),
        DigitalInitialValue::FourState(bits("129'h100000000000000000000000000010200"))
    );
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "y",
        specialized.model,
        &specialized.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 6.5);
}

#[test]
fn exact_parameter_conversion_is_explicit_and_scoped_to_one_module() {
    let compiler = compiler();
    let report = compiler
        .compile_runtime(
            "module rounded(p,q); inout p; electrical p; parameter P=64'h20000000000001; \
         parameter real R=P; output reg [63:0] q=P; analog I(p)<+R; endmodule",
            None,
        )
        .unwrap();
    assert_eq!(
        initial(&report, "q"),
        DigitalInitialValue::FourState(bits("64'h20000000000001"))
    );
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "rounded",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 9_007_199_254_740_992.0);
    device.try_set_parameter("R", 1.5).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 1.5);
    let report = compiler
        .compile_runtime(
            "module first(q); parameter P=64'h20000000000001; output reg [63:0] q=P; endmodule \
         module second(p,q); inout p; electrical p; parameter real P=1.0; \
         parameter real R=P+1.0; output reg q=0; analog I(p)<+R; endmodule",
            Some("second"),
        )
        .unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "second",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 2.0);
    device.try_set_parameter("P", 4.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 5.0);
}

#[test]
fn exact_localparams_keep_private_values_and_numeric_prologues() {
    let compiler = compiler();
    let source = r#"
module local_exact(p,q);
 inout p; electrical p;
 parameter real BASE=2.5;
 localparam WORD=129'h1_00000000_00000000_00000000_000101xz;
 localparam COPY=WORD;
 localparam MASKED=COPY & 129'h1_ffffffff_ffffffff_ffffffff_ffffff00;
 localparam integer UNKNOWN=32'bx;
 localparam integer LOW=COPY[15:8];
 localparam real RESULT=BASE+(COPY[15:8]+8'd255);
 output reg [128:0] q=MASKED;
 reg [63:0] extended=UNKNOWN;
 analog I(p)<+RESULT+LOW;
endmodule
"#;
    let report = compiler.compile_runtime(source, None).unwrap();
    report.validate_integrity().unwrap();
    assert_eq!(
        initial(&report, "q"),
        DigitalInitialValue::FourState(bits("129'h1_00000000_00000000_00000000_00010100"))
    );
    assert_eq!(
        initial(&report, "extended"),
        DigitalInitialValue::FourState(bits("64'bx"))
    );
    assert!(report.abi.elaboration_parameters.is_empty());
    assert_eq!(
        report
            .abi
            .parameters
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["BASE"]
    );
    let exact = &report.canonical_ir.digital.elaboration_parameters;
    assert_eq!(
        exact
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["WORD", "COPY", "MASKED", "UNKNOWN"]
    );
    assert!(
        exact
            .iter()
            .all(|parameter| !parameter.is_public && parameter.aliases.is_empty())
    );
    assert!(exact[3].signed);
    for parameter in exact {
        assert!(
            !report
                .canonical_ir
                .hir
                .variables
                .iter()
                .any(|variable| variable.name == parameter.name)
        );
    }
    let serialized = serde_json::to_vec(&report).unwrap();
    let decoded: rspice_veriloga::RuntimeCompileReport =
        serde_json::from_slice(&serialized).unwrap();
    decoded.validate_integrity().unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "local",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 3.5);
    device.try_set_parameter("BASE", 10.5).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 11.5);
    assert!(!device.try_set_parameter("COPY", 0.0).unwrap());
    assert!(
        compiler
            .specialize_mixed_runtime_typed(
                &report.canonical_ir,
                &[("COPY", ScalarParameterValue::Integer(0))],
                &NoPipelineControl
            )
            .is_err()
    );
}

#[test]
fn exact_localparams_keep_each_child_specialization_and_reject_numeric_reads() {
    let compiler = compiler();
    let report = compiler
        .compile_runtime(
            r#"
module local_leaf(q);
 parameter WORD=129'h1_00000000_00000000_00000000_000001xz;
 localparam MASKED=WORD & 129'h1_ffffffff_ffffffff_ffffffff_ffffff00;
 output reg [128:0] q=MASKED;
endmodule
module local_top(a,b);
 output wire [128:0] a,b;
 local_leaf first(a);
 local_leaf #(.WORD(129'h1_00000000_00000000_00000000_000002xz)) second(b);
endmodule
"#,
            Some("local_top"),
        )
        .unwrap();
    let exact = &report.canonical_ir.digital.elaboration_parameters;
    let first = exact
        .iter()
        .find(|parameter| parameter.name == "first.MASKED")
        .unwrap();
    let second = exact
        .iter()
        .find(|parameter| parameter.name == "second.MASKED")
        .unwrap();
    assert_eq!(
        first.value,
        bits("129'h1_00000000_00000000_00000000_00000100")
    );
    assert_eq!(
        second.value,
        bits("129'h1_00000000_00000000_00000000_00000200")
    );
    assert!(!first.is_public && !second.is_public);
    assert!(report.abi.elaboration_parameters.is_empty());
    assert_eq!(
        initial(&report, "first.q"),
        DigitalInitialValue::FourState(first.value.clone())
    );
    assert_eq!(
        initial(&report, "second.q"),
        DigitalInitialValue::FourState(second.value.clone())
    );
    report.validate_integrity().unwrap();
    for name in ["LOCAL", "M_PI"] {
        let source = format!(
            "module bad(p,q); inout p; electrical p; localparam {name}=129'h1_00000000_00000000_00000000_000001xz; output reg q=0; analog I(p)<+{name}; endmodule"
        );
        let error = compiler
            .compile_runtime(&source, None)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(&format!("packed parameter '{name}'")),
            "{error}"
        );
    }
}

#[test]
fn exact_localparams_in_analog_hierarchy_need_no_digital_execution() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let report = compiler
        .compile_runtime(
            r#"
module analog_leaf(p);
 inout p; electrical p;
 localparam WORD=129'h1_00000000_00000000_00000000_000001xz;
 localparam real LEVEL=WORD[15:8]+0.0;
 analog I(p)<+LEVEL;
endmodule
module analog_top(p);
 inout p; electrical p;
 localparam WORD=129'h1_00000000_00000000_00000000_000002xz;
 localparam real LEVEL=WORD[15:8]+0.0;
 analog I(p)<+LEVEL;
 analog_leaf first(p), second(p);
endmodule
"#,
            Some("analog_top"),
        )
        .unwrap();
    let plan = &report.canonical_ir.digital;
    assert!(
        !plan.is_empty(),
        "exact metadata must survive serialization"
    );
    assert!(!plan.has_executable_content());
    assert_eq!(plan.elaboration_parameters.len(), 3);
    assert!(
        plan.elaboration_parameters
            .iter()
            .all(|parameter| !parameter.is_public)
    );
    let names: std::collections::HashSet<_> = plan
        .elaboration_parameters
        .iter()
        .map(|parameter| &parameter.name)
        .collect();
    assert_eq!(
        names.len(),
        3,
        "each analog instance has its own constant scope"
    );
    assert_eq!(
        plan.elaboration_parameters
            .iter()
            .filter(
                |parameter| parameter.value == bits("129'h1_00000000_00000000_00000000_000001xz")
            )
            .count(),
        2
    );
    let serialized = serde_json::to_vec(&report).unwrap();
    let decoded: rspice_veriloga::RuntimeCompileReport =
        serde_json::from_slice(&serialized).unwrap();
    decoded.validate_integrity().unwrap();
    assert_eq!(decoded.canonical_ir.digital, *plan);
    // Flattened branch orientations can differ. Compare current entering p,
    // using the residual stamp sign rather than summing raw branch values.
    let weights: Vec<f64> = report
        .model
        .stamp_programs
        .iter()
        .map(|program| {
            program
                .stamp_locations
                .iter()
                .filter(|location| {
                    matches!(
                        location.row,
                        rspice_veriloga::codegen::StampIndex::Terminal(0)
                    )
                })
                .map(|location| -location.sign)
                .sum()
        })
        .collect();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "analog",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(
        device
            .try_evaluate()
            .unwrap()
            .iter()
            .zip(weights)
            .map(|(value, sign)| value * sign)
            .sum::<f64>(),
        4.0
    );

    // A public packed parameter also remains metadata without adding a runtime.
    let report = compiler
        .compile_runtime(
            "module analog_exact(p); inout p; electrical p; \
         parameter P=64'h20000000000001; localparam real R=P; analog I(p)<+R; endmodule",
            None,
        )
        .unwrap();
    assert!(!report.canonical_ir.digital.has_executable_content());
    assert_eq!(
        report.abi.elaboration_parameters[0].value,
        bits("64'h20000000000001")
    );
    report.validate_integrity().unwrap();

    // Flattening must keep the name of an unsupported analog read bound to the
    // private constant; a parent symbol or built-in M_PI is never a substitute.
    let error = compiler
        .compile_runtime(
            "module bad_leaf(p); inout p; electrical p; localparam M_PI=32'bx; \
         analog I(p)<+M_PI; endmodule \
         module bad_top(p); inout p; electrical p; bad_leaf child(p); endmodule",
            Some("bad_top"),
        )
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("packed parameter") && error.contains("M_PI"),
        "{error}"
    );
}

#[test]
fn interleaved_parameters_keep_exact_locals_and_numeric_assignment_boundaries() {
    let compiler = compiler();
    let report = compiler
        .compile_runtime(
            r#"
module ordered(p,q);
 inout p; electrical p;
 parameter integer BASE=5;
 localparam real AS_REAL=BASE;
 parameter real HALF=AS_REAL/2;
 localparam integer ROUNDED=HALF;
 parameter real RESULT=ROUNDED+HALF;
 localparam integer NIBBLE=16'h10ff;
 parameter SELECTED=NIBBLE[7:0];
 localparam WORD=129'h1_00000000_00000000_00000000_000001xz;
 parameter COPY=WORD;
 localparam MASKED=COPY & 129'h1_ffffffff_ffffffff_ffffffff_ffffff00;
 parameter FINAL=MASKED;
 output reg [128:0] q=FINAL;
 analog I(p)<+RESULT+(SELECTED-255);
endmodule
"#,
            None,
        )
        .unwrap();
    report.validate_integrity().unwrap();
    assert_eq!(
        initial(&report, "q"),
        DigitalInitialValue::FourState(bits("129'h1_00000000_00000000_00000000_00000100"))
    );
    assert_eq!(
        report
            .abi
            .parameters
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>(),
        ["BASE", "HALF", "RESULT", "SELECTED"]
    );
    assert_eq!(
        report
            .abi
            .elaboration_parameters
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>(),
        ["COPY", "FINAL"]
    );
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "ordered",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 5.5);
    device.try_set_parameter("BASE", 7.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 7.5);
    device.try_set_parameter("HALF", 1.25).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 2.25);
    assert!(!device.try_set_parameter("AS_REAL", 10.0).unwrap());
    device.try_set_parameter("SELECTED", 256.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 3.25);
}

#[test]
fn interleaved_parameter_defaults_reject_forward_self_and_excessive_expansion() {
    let compiler = compiler();
    for (declarations, diagnostic) in [
        (
            "localparam A=B+1; parameter B=2;",
            "references later parameter 'B'",
        ),
        (
            "localparam A=B; localparam B=32'bx;",
            "references later parameter 'B'",
        ),
        (
            "parameter A=B; localparam B=32'bx;",
            "references later parameter 'B'",
        ),
        (
            "parameter A=B[7:0]; parameter B=32'bx;",
            "references later parameter 'B'",
        ),
        ("localparam A=A+1;", "references itself"),
        (
            "localparam string S=65; parameter P=S;",
            "default depends on string localparam 'S'",
        ),
        (
            r#"localparam string S="A"; localparam T=S; parameter P=T;"#,
            "default depends on string localparam 'S'",
        ),
    ] {
        let source = format!("module bad(q); {declarations} output reg q=0; endmodule");
        let error = compiler
            .compile_runtime(&source, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains(diagnostic), "{source}: {error}");
    }
    let mut source = "module huge(q); parameter real BASE=1.0; localparam real L0=BASE;".to_owned();
    for index in 1..22 {
        source.push_str(&format!(
            "localparam real L{index}=L{}+L{};",
            index - 1,
            index - 1
        ));
    }
    source.push_str("parameter real RESULT=L21; output reg q=0; endmodule");
    let error = compiler
        .compile_runtime(&source, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("localparam dependency expansion"), "{error}");

    // The argument names the public l21 parameter using external lookup rules;
    // it does not expand the enormous, case-distinct local L21 expression.
    let query = source
        .replace(
            "module huge(q);",
            "module huge(p,q); inout p; electrical p;",
        )
        .replace(
            "parameter real BASE=1.0;",
            "parameter real l21=0; parameter real BASE=1.0;",
        )
        .replace(
            "RESULT=L21",
            "RESULT=$param_given(L21)+$param_given(selector)",
        )
        .replace(
            "endmodule",
            "aliasparam SELECTOR=l21; analog I(p)<+RESULT; endmodule",
        );
    let report = compiler.compile_runtime(&query, None).unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "query",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
    device.try_set_parameter("SELECTOR", 2.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 2.0);
}

#[test]
fn interleaved_numeric_local_defaults_keep_ranges_and_scope_dependencies_live() {
    let compiler = compiler();
    let report = compiler
        .compile_runtime(
            r#"
module ordered_range(p,q);
 inout p; electrical p; output reg q=0;
 parameter real BASE=2.0;
 localparam real LIMIT=BASE+1.0;
 parameter real VALUE=1.0 from [0:LIMIT];
 localparam integer COUNT=VALUE;
 parameter real RESULT=COUNT+VALUE;
 analog I(p)<+RESULT;
endmodule
"#,
            None,
        )
        .unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "range",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    device.try_set_parameter("VALUE", 2.5).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 5.5);
    device.try_set_parameter("BASE", 1.0).unwrap();
    assert!(
        device.try_resolve_parameter_defaults().is_err(),
        "LIMIT must follow BASE"
    );

    let error = compiler
        .compile_runtime(
            r#"
module scope(p,q);
 inout p; electrical p; output reg q=0;
 (* type="instance" *) parameter real BASE=2;
 localparam real PRIVATE=BASE+1;
 parameter real MODEL=PRIVATE;
 analog I(p)<+MODEL;
endmodule
"#,
            None,
        )
        .unwrap_err()
        .to_string();
    assert!(error.contains("instance parameter"), "{error}");
}
