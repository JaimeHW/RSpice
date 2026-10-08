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

#[test]
fn packed_parameter_given_preserves_explicit_defaults_and_aliases() {
    assert_packed_parameter_given("[128:0] CODE", "129'h1_00000000_00000000_00000000_000000xz");
    assert_packed_parameter_given("CODE", "64'h20000000000001");
}

fn assert_packed_parameter_given(declaration: &str, literal: &str) {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            &format!(
                r#"
module packed_given(p);
 inout p; electrical p;
 parameter {declaration}={literal};
 aliasparam PATTERN=CODE;
 parameter real GAIN=1;
 parameter real LEVEL=$param_given(PATTERN)?10.0:1.0;
 localparam real EXTRA=$param_given(CODE)?2.0:0.0;
 analog I(p)<+LEVEL+EXTRA+3*$param_given(PATTERN)+$param_given(GAIN);
endmodule
"#
            ),
            None,
        )
        .unwrap();
    for supplied in [None, Some("CODE"), Some("PATTERN")] {
        let report = if let Some(name) = supplied {
            compiler
                .specialize_mixed_runtime_typed(
                    &original.canonical_ir,
                    &[(
                        name,
                        ScalarParameterValue::Bits {
                            value: bits(literal),
                            signed: false,
                        },
                    )],
                    &NoPipelineControl,
                )
                .unwrap()
        } else {
            original.clone()
        };
        let encoded = serde_json::to_vec(&report).unwrap();
        let decoded: rspice_veriloga::RuntimeCompileReport =
            serde_json::from_slice(&encoded).unwrap();
        decoded.validate_integrity().unwrap();
        assert_eq!(
            decoded.abi.elaboration_parameters[0].is_given,
            supplied.is_some()
        );
        assert_eq!(decoded.abi.elaboration_parameters[0].aliases, ["PATTERN"]);
        let mut damaged = decoded.clone();
        damaged.canonical_ir.digital.elaboration_parameters[0].is_given = supplied.is_none();
        assert!(damaged.validate_integrity().is_err());
        if supplied.is_some() {
            assert_ne!(
                decoded.canonical_ir.digital.content_identity,
                original.canonical_ir.digital.content_identity
            );
        }
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "given",
            decoded.model,
            &decoded.canonical_ir,
            &[1],
        )
        .unwrap();
        let expected = if supplied.is_some() { 15.0 } else { 1.0 };
        assert_eq!(device.try_evaluate().unwrap()[0], expected, "{supplied:?}");
        assert!(device.try_set_parameter("GAIN", 2.0).unwrap());
        assert_eq!(device.try_evaluate().unwrap()[0], expected + 1.0);
    }
}

#[test]
fn constant_given_dependencies_require_source_even_for_same_value_assignments() {
    let compiler = compiler();
    for declaration in ["parameter [7:0]", "parameter integer"] {
        let source = format!(
            r#"
module constant_given(p,q);
 inout p; electrical p;
 parameter real INPUT=5.0;
 aliasparam USER=INPUT;
 {declaration} CODE=$param_given(USER)?9:3;
 parameter real LEVEL=CODE+0.0;
 output reg [7:0] q=CODE;
 analog I(p)<+LEVEL;
endmodule
"#
        );
        let original = compiler.compile_runtime(&source, None).unwrap();
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "omitted",
            original.model.clone(),
            &original.canonical_ir,
            &[1, 2],
        )
        .unwrap();
        assert_eq!(device.try_evaluate().unwrap()[0], 3.0);
        let error = device
            .try_set_parameter("USER", 5.0)
            .unwrap_err()
            .to_string();
        assert!(error.contains("specialize the source"), "{error}");
        assert_eq!(device.try_evaluate().unwrap()[0], 3.0);
        let supplied = compiler
            .specialize_mixed_runtime_typed(
                &original.canonical_ir,
                &[("USER", ScalarParameterValue::Real(5.0))],
                &NoPipelineControl,
            )
            .unwrap();
        let encoded = serde_json::to_vec(&supplied).unwrap();
        let supplied: rspice_veriloga::RuntimeCompileReport =
            serde_json::from_slice(&encoded).unwrap();
        supplied.validate_integrity().unwrap();
        assert_eq!(supplied.abi.parameters[0].elaboration_given, Some(true));
        assert_eq!(original.abi.parameters[0].elaboration_given, Some(false));
        let mut damaged = supplied.clone();
        damaged.canonical_ir.hir.parameters[0].elaboration_given = Some(false);
        assert!(damaged.validate_integrity().is_err());
        assert_eq!(
            initial(&supplied, "q"),
            DigitalInitialValue::FourState(bits("8'h09"))
        );
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "supplied",
            supplied.model.clone(),
            &supplied.canonical_ir,
            &[1, 2],
        )
        .unwrap();
        assert_eq!(device.try_evaluate().unwrap()[0], 9.0);
        // Only presence controls CODE: numeric changes remain legal after INPUT
        // is supplied, without inventing a value dependency.
        assert!(device.try_set_parameter("INPUT", 8.0).unwrap());
        device.try_resolve_parameter_defaults().unwrap();
        assert_eq!(device.try_evaluate().unwrap()[0], 9.0);
    }
}

#[test]
fn constant_given_generate_changes_execution_without_freezing_numeric_values() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module generated_given(p);
 inout p; electrical p;
 parameter real INPUT=5.0;
 aliasparam USER=INPUT;
 integer q=0;
 generate if($param_given(USER)) begin:enabled
  initial q=1;
 end endgenerate
 analog I(p)<+INPUT+q;
endmodule
"#,
            None,
        )
        .unwrap();
    assert!(!original.canonical_ir.digital.has_executable_content());
    assert_eq!(original.model.parameters[0].elaboration_given, Some(false));
    let supplied = compiler
        .specialize_mixed_runtime_typed(
            &original.canonical_ir,
            &[("USER", ScalarParameterValue::Real(5.0))],
            &NoPipelineControl,
        )
        .unwrap();
    assert!(supplied.canonical_ir.digital.has_executable_content());
    assert_eq!(supplied.model.parameters[0].elaboration_given, Some(true));
    assert_eq!(supplied.model.parameters[0].elaboration_value, None);
    assert!(compiler.compile_runtime(
        "module illegal(q); parameter integer P=1; output reg q; initial q=$param_given(P); endmodule", None,
    ).is_err(), "procedural digital queries remain outside the supported contexts");
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

#[test]
fn implicit_native_parameter_types_agree_across_domains_and_specializations() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module inferred(p);
 inout p; electrical p;
 parameter P=5;
 parameter Q=P/2;
 localparam L=P/2;
 parameter RESULT=Q+L;
 real sample=RESULT;
 initial sample=RESULT;
 analog I(p)<+RESULT+(L-Q);
endmodule
"#,
            None,
        )
        .unwrap();
    let verify = |report: &rspice_veriloga::RuntimeCompileReport, expected: f64, integer: bool| {
        assert_eq!(
            initial(report, "sample"),
            DigitalInitialValue::Real(expected)
        );
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "inferred",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap();
        assert_eq!(device.try_evaluate().unwrap()[0], expected);
        report.validate_integrity().unwrap();
        assert!(
            report
                .model
                .parameters
                .iter()
                .all(|parameter| parameter.is_integer == integer)
        );
        device
    };
    let mut device = verify(&original, 4.0, true);
    device.try_set_parameter("P", 7.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 6.0);
    for (override_value, expected, integer) in [
        (ScalarParameterValue::Real(5.0), 5.0, false),
        (ScalarParameterValue::Integer(7), 6.0, true),
        (ScalarParameterValue::Integer(-5), -4.0, true),
    ] {
        let specialized = compiler
            .specialize_mixed_runtime_typed(
                &original.canonical_ir,
                &[("P", override_value)],
                &NoPipelineControl,
            )
            .unwrap();
        verify(&specialized, expected, integer);
        let restored = compiler
            .specialize_mixed_runtime_typed(
                &specialized.canonical_ir,
                &[("P", ScalarParameterValue::Integer(5))],
                &NoPipelineControl,
            )
            .unwrap();
        verify(&restored, 4.0, true);
    }
}

#[test]
fn declared_packed_parameters_keep_assignment_context_ranges_and_overrides() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module declared(p);
 inout p; electrical p;
 parameter integer W=8;
 parameter [W+15:16] U=4'shf;
 aliasparam PATTERN=U;
 parameter signed [0:W-1] S=4'hf;
 parameter signed SIGNED=8'hf0;
 localparam [4:7] N=8'hab;
 parameter [15:8] SUM=8'hff+8'h2;
 parameter signed [15:0] WIDE=8'hff+8'h2;
 parameter [0:7] R=7.5;
 parameter real LEVEL=U+0.0+S+N;
 reg [15:0] u=U, s=S, sign=SIGNED, n=N, sum=SUM, wide=WIDE, rounded=R;
 reg [7:0] slices={U[W+15:W+12],N[4:7]};
 initial begin u=U; s=S; sign=SIGNED; n=N; sum=SUM; wide=WIDE; rounded=R; end
 analog I(p)<+LEVEL;
endmodule
"#,
            None,
        )
        .unwrap();
    for (name, raw) in [
        ("u", "16'hff"),
        ("s", "16'hf"),
        ("sign", "16'hfff0"),
        ("n", "16'hb"),
        ("sum", "16'h1"),
        ("wide", "16'h101"),
        ("rounded", "16'h8"),
        ("slices", "8'hfb"),
    ] {
        assert_eq!(
            initial(&original, name),
            DigitalInitialValue::FourState(bits(raw)),
            "{name}"
        );
    }
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "declared",
        original.model.clone(),
        &original.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 281.0);
    let error = device.try_set_parameter("W", 4.0).unwrap_err().to_string();
    assert!(error.contains("specialize the source"), "{error}");
    assert_eq!(device.try_evaluate().unwrap()[0], 281.0);
    assert!(!device.try_set_parameter("U", 4.0).unwrap());
    for overrides in [
        vec![
            ("PATTERN", ScalarParameterValue::Integer(0x1234)),
            ("W", ScalarParameterValue::Integer(4)),
        ],
        vec![
            ("W", ScalarParameterValue::Integer(4)),
            ("PATTERN", ScalarParameterValue::Integer(0x1234)),
        ],
    ] {
        let report = compiler
            .specialize_mixed_runtime_typed(&original.canonical_ir, &overrides, &NoPipelineControl)
            .unwrap();
        assert_eq!(
            initial(&report, "u"),
            DigitalInitialValue::FourState(bits("16'h4"))
        );
        assert_eq!(
            initial(&report, "s"),
            DigitalInitialValue::FourState(bits("16'hffff"))
        );
        let u = report
            .abi
            .elaboration_parameters
            .iter()
            .find(|value| value.name == "U")
            .unwrap();
        assert_eq!(u.value.width(), 4);
        assert_eq!(
            u.bounds.unwrap(),
            rspice_veriloga::semantic::VectorBounds { msb: 19, lsb: 16 }
        );
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "override",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap();
        assert_eq!(device.try_evaluate().unwrap()[0], 14.0);
        let encoded = serde_json::to_vec(&report).unwrap();
        let decoded: rspice_veriloga::RuntimeCompileReport =
            serde_json::from_slice(&encoded).unwrap();
        decoded.validate_integrity().unwrap();
        let mut damaged = decoded.clone();
        damaged
            .model
            .parameters
            .iter_mut()
            .find(|value| value.name == "W")
            .unwrap()
            .elaboration_value = None;
        assert!(damaged.validate_integrity().is_err());
        let mut damaged = decoded;
        damaged
            .canonical_ir
            .digital
            .elaboration_parameters
            .iter_mut()
            .find(|value| value.name == "U")
            .unwrap()
            .bounds = None;
        assert!(damaged.validate_integrity().is_err());
    }
}

#[test]
fn declared_packed_parameters_guard_transitive_dependencies_and_child_scopes() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module locked(p);
 inout p; electrical p;
 parameter A=254;
 parameter BASE=A+1;
 aliasparam SOURCE=A;
 parameter [7:0] P=BASE+1;
 parameter real LEVEL=P+0.0;
 analog I(p)<+LEVEL;
endmodule
"#,
            None,
        )
        .unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "locked",
        original.model.clone(),
        &original.canonical_ir,
        &[1],
    )
    .unwrap();
    for (name, value) in [("A", 255.0), ("SOURCE", 255.0), ("BASE", 256.0)] {
        assert!(
            device
                .try_set_parameter(name, value)
                .unwrap_err()
                .to_string()
                .contains("specialize the source")
        );
        assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
    }
    let report = compiler
        .specialize_mixed_runtime_typed(
            &original.canonical_ir,
            &[("SOURCE", ScalarParameterValue::Integer(255))],
            &NoPipelineControl,
        )
        .unwrap();
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "changed",
        report.model,
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    let hierarchy = compiler
        .compile_runtime(
            r#"
module leaf(q);
 parameter W=8;
 parameter [W+3:4] P=0;
 output reg [15:0] q=P;
endmodule
module parent(a,b);
 output wire [15:0] a,b;
 parameter W=1;
 leaf #(.P(16'h1234),.W(4)) small(a);
 leaf #(.W(8),.P(16'h1234)) large(b);
endmodule
"#,
            Some("parent"),
        )
        .unwrap();
    assert_eq!(
        initial(&hierarchy, "small.q"),
        DigitalInitialValue::FourState(bits("16'h4"))
    );
    assert_eq!(
        initial(&hierarchy, "large.q"),
        DigitalInitialValue::FourState(bits("16'h34"))
    );
    let wide = compiler
        .compile_runtime(
            r#"
module wide(q,x,z,n);
 parameter [W+3:4] P=16'h1234;
 parameter integer W=8;
 parameter BASE=129'h1_00000000_00000000_00000000_00000003;
 parameter [7:0] N=BASE;
 parameter signed [0:128] X=4'shx, Z=4'shz;
 output reg [15:0] q=P;
 output reg [128:0] x=X,z=Z;
 output reg [7:0] n=N;
endmodule
"#,
            None,
        )
        .unwrap();
    for (name, raw) in [
        ("q", "16'h34"),
        ("x", "129'bx"),
        ("z", "129'bz"),
        ("n", "8'h03"),
    ] {
        assert_eq!(
            initial(&wide, name),
            DigitalInitialValue::FourState(bits(raw)),
            "{name}"
        );
    }
    wide.validate_integrity().unwrap();
    for (source, expected) in [
        (
            r#"module leaf(p); inout p; electrical p;
 parameter real A[1:0]='{1,2}; parameter [7:0] P=1;
 parameter real LEVEL=P+0.0; analog I(p)<+LEVEL; endmodule
module parent(p); inout p; electrical p; leaf #(.P(2)) child(p); endmodule"#,
            "requires combined packed and array source specialization",
        ),
        (
            "module bad(q); parameter [1.5:0] P=0; output reg q=0; endmodule",
            "packed bounds require known integer",
        ),
        (
            "module bad(q); parameter [65536:0] P=0; output reg q=0; endmodule",
            "supported packed width",
        ),
        (
            "module bad(q); parameter [W:0] P=0; parameter W=P; output reg q=0; endmodule",
            "cyclic",
        ),
    ] {
        let error = compiler
            .compile_runtime(source, source.contains("module parent").then_some("parent"))
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn analog_children_specialize_packed_parameters_and_preserve_live_numeric_inputs() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module packed_leaf(p);
 inout p; electrical p;
 parameter integer W=8 from [1:32];
 parameter [W+15:16] CODE=16'h1234;
 aliasparam PATTERN=CODE;
 parameter real GAIN=1;
 parameter real LEVEL=CODE+0.0;
 analog I(p)<+GAIN*(LEVEL+($param_given(CODE)?1000:0));
endmodule
module middle(p);
 inout p; electrical p;
 parameter integer WIDTH=4;
 parameter real GAIN=2;
 packed_leaf #(.W(WIDTH),.PATTERN(16'h1234),.GAIN(GAIN)) stage(p);
endmodule
module top(p);
 inout p; electrical p;
 parameter integer N=4;
 parameter real GAIN=2;
 localparam WIDTH=N;
 middle #(.WIDTH(WIDTH),.GAIN(GAIN)) nested(p);
 packed_leaf #(.PATTERN(16'h12ff),.W(8)) direct(p);
endmodule
"#,
            Some("top"),
        )
        .unwrap();
    let current = |report: &rspice_veriloga::RuntimeCompileReport,
                   device: &mut rspice_veriloga::device::VerilogADevice| {
        report
            .model
            .stamp_programs
            .iter()
            .zip(device.try_evaluate().unwrap())
            .map(|(program, value)| {
                program
                    .stamp_locations
                    .iter()
                    .filter(|stamp| {
                        matches!(stamp.row, rspice_veriloga::codegen::StampIndex::Terminal(0))
                    })
                    .map(|stamp| -stamp.sign * value)
                    .sum::<f64>()
            })
            .sum::<f64>()
    };
    let make_device = |report: &rspice_veriloga::RuntimeCompileReport| {
        rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "hierarchy",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap()
    };
    let mut device = make_device(&original);
    assert_eq!(current(&original, &mut device), 3263.0);
    assert!(
        device
            .try_set_parameter("N", 8.0)
            .unwrap_err()
            .to_string()
            .contains("specialize the source")
    );
    assert_eq!(current(&original, &mut device), 3263.0);
    assert!(device.try_set_parameter("GAIN", 3.0).unwrap());
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(current(&original, &mut device), 4267.0);
    let specialized = compiler
        .specialize_mixed_runtime_typed(
            &original.canonical_ir,
            &[("N", ScalarParameterValue::Integer(8))],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(
        current(&specialized, &mut make_device(&specialized)),
        3359.0
    );
    let encoded = serde_json::to_vec(&specialized).unwrap();
    let restored: rspice_veriloga::RuntimeCompileReport = serde_json::from_slice(&encoded).unwrap();
    restored.validate_integrity().unwrap();
    assert_eq!(current(&restored, &mut make_device(&restored)), 3359.0);
    assert_eq!(
        restored.canonical_ir.digital.elaboration_parameters.len(),
        2
    );
    assert!(
        restored
            .canonical_ir
            .digital
            .elaboration_parameters
            .iter()
            .all(|value| value.is_given)
    );
    assert!(!restored.canonical_ir.digital.has_executable_content());
}

#[test]
fn analog_generated_children_keep_wide_values_and_structural_dependencies() {
    let compiler = compiler();
    let original = compiler
        .compile_runtime(
            r#"
module cell(p);
 inout p; electrical p;
 parameter signed [0:128] WORD=0;
 parameter real GAIN=1;
 parameter real LEVEL=WORD[113:120]+0.0;
 analog I(p)<+GAIN*LEVEL;
endmodule
module bank(p);
 inout p; electrical p;
 parameter integer COUNT=1;
 parameter [128:0] WORD=0;
 parameter real GAIN=1;
 genvar i;
 generate for(i=0;i<COUNT;i=i+1) begin:cells
   cell #(.WORD(WORD),.GAIN(GAIN)) item(p);
 end endgenerate
endmodule
module top(p);
 inout p; electrical p;
 parameter integer COUNT=2;
 parameter [128:0] WORD=129'h1_00000000_00000000_00000000_000001xz;
 parameter real GAIN=2;
 bank #(.COUNT(COUNT),.WORD(WORD),.GAIN(GAIN)) stage(p);
endmodule
"#,
            Some("top"),
        )
        .unwrap();
    let make_device = |report: &rspice_veriloga::RuntimeCompileReport| {
        rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "wide",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap()
    };
    let current = |report: &rspice_veriloga::RuntimeCompileReport,
                   device: &mut rspice_veriloga::device::VerilogADevice| {
        report
            .model
            .stamp_programs
            .iter()
            .zip(device.try_evaluate().unwrap())
            .map(|(program, value)| {
                program
                    .stamp_locations
                    .iter()
                    .filter(|stamp| {
                        matches!(stamp.row, rspice_veriloga::codegen::StampIndex::Terminal(0))
                    })
                    .map(|stamp| -stamp.sign * value)
                    .sum::<f64>()
            })
            .sum::<f64>()
    };
    let mut device = make_device(&original);
    assert_eq!(current(&original, &mut device), 4.0);
    assert!(
        device
            .try_set_parameter("COUNT", 3.0)
            .unwrap_err()
            .to_string()
            .contains("specialize the source")
    );
    assert!(device.try_set_parameter("GAIN", 3.0).unwrap());
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(current(&original, &mut device), 6.0);
    let expanded = compiler
        .specialize_mixed_runtime_typed(
            &original.canonical_ir,
            &[("COUNT", ScalarParameterValue::Integer(3))],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(current(&expanded, &mut make_device(&expanded)), 6.0);
    assert_eq!(
        expanded.canonical_ir.digital.elaboration_parameters.len(),
        5
    );
    for value in &expanded.canonical_ir.digital.elaboration_parameters {
        assert_eq!(
            value.value,
            bits("129'h1_00000000_00000000_00000000_000001xz")
        );
    }
    expanded.validate_integrity().unwrap();
    let empty = compiler
        .compile_runtime(
            r#"
module unit(p); inout p; electrical p; analog I(p)<+1; endmodule
module empty(p);
 inout p; electrical p; parameter integer COUNT=0;
 generate if(COUNT) begin:g unit child(p); end endgenerate
endmodule
"#,
            Some("empty"),
        )
        .unwrap();
    let mut damaged = empty.canonical_ir.clone();
    damaged.parameter_source = None;
    assert!(damaged.validate().is_err());
    let mut empty_device = make_device(&empty);
    assert!(
        empty_device
            .try_set_parameter("COUNT", 1.0)
            .unwrap_err()
            .to_string()
            .contains("specialize the source")
    );
    let populated = compiler
        .specialize_mixed_runtime_typed(
            &empty.canonical_ir,
            &[("COUNT", ScalarParameterValue::Integer(1))],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(current(&populated, &mut make_device(&populated)), 1.0);
}

#[test]
fn constant_given_hierarchy_protects_parent_presence_used_by_child_width() {
    let compiler = compiler();
    let current = |report: &rspice_veriloga::RuntimeCompileReport,
                   device: &mut rspice_veriloga::device::VerilogADevice| {
        report
            .model
            .stamp_programs
            .iter()
            .zip(device.try_evaluate().unwrap())
            .flat_map(|(program, value)| {
                program
                    .stamp_locations
                    .iter()
                    .filter(|stamp| {
                        matches!(stamp.row, rspice_veriloga::codegen::StampIndex::Terminal(0))
                    })
                    .map(move |stamp| -stamp.sign * value)
            })
            .sum::<f64>()
    };
    let original = compiler
        .compile_runtime(
            r#"
module given_width_child(p);
 inout p; electrical p;
 parameter integer W=4;
 parameter [W-1:0] CODE=16'h1234;
 parameter real LEVEL=CODE+0.0;
 analog I(p)<+LEVEL;
endmodule
module given_width_parent(p);
 inout p; electrical p;
 parameter real INPUT=5;
 aliasparam USER=INPUT;
 given_width_child #(.W($param_given(USER)?8:4)) child(p);
endmodule
"#,
            Some("given_width_parent"),
        )
        .unwrap();
    assert_eq!(original.model.parameters[0].elaboration_given, Some(false));
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "omitted",
        original.model.clone(),
        &original.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(current(&original, &mut device), 4.0);
    assert!(device.try_set_parameter("USER", 5.0).is_err());
    let supplied = compiler
        .specialize_mixed_runtime_typed(
            &original.canonical_ir,
            &[("USER", ScalarParameterValue::Real(5.0))],
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(supplied.model.parameters[0].elaboration_given, Some(true));
    assert_eq!(supplied.model.parameters[0].elaboration_value, None);
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "supplied",
        supplied.model.clone(),
        &supplied.canonical_ir,
        &[1],
    )
    .unwrap();
    assert_eq!(current(&supplied, &mut device), 52.0);
    assert!(device.try_set_parameter("INPUT", 8.0).unwrap());
    device.try_resolve_parameter_defaults().unwrap();
    assert_eq!(current(&supplied, &mut device), 52.0);
}
