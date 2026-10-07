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
    let pattern = bits("129'h10000000000000000000000000000000xz");
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
        "parameter real OTHER=P; analog I(p)<+OTHER;",
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
