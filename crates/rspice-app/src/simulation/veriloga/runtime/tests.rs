//! Artifact precision and sealing regression coverage.

use super::PreparedVerilogARuntime;

/// Constants whose shortest decimal form `serde_json` does *not* parse back
/// exactly unless the `float_roundtrip` feature is on. `1.3806505e-23` is
/// Boltzmann's constant verbatim from shipped Verilog-A models; without the
/// feature it reads back as `1.3806504999999999e-23`.
const LOSSY_MODEL_CONSTANTS: [(&str, f64); 3] = [
    ("kb", 1.380_650_5e-23),
    ("q", 1.602_176_634e-19),
    ("scale", 6.25e41),
];

const EXACT_CONSTANT_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_float_roundtrip_probe(p, n);
  inout p, n;
  electrical p, n;
  parameter real kb = 1.3806505e-23;
  parameter real q = 1.602176634e-19;
  parameter real scale = 6.25e41;
  analog I(p, n) <+ (kb + q + scale) * V(p, n);
endmodule
"#;

/// Compile one fixture source and seal it exactly as a project source is
/// sealed. `try_from_virtual_compilation` is the constructor both the
/// project and the signed-PDK paths reach, so a fixture that seals here
/// seals in production.
fn seal_runtime(file: &str, module: &str, source: &str) -> PreparedVerilogARuntime {
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        file,
        [rspice_veriloga::VirtualSourceFile::new(file, source)],
    )
    .expect("fixture bundle is well formed");
    let compilation = rspice_veriloga::VerilogACompiler::default()
        .compile_virtual_runtime(
            &bundle,
            module,
            rspice_veriloga::VirtualCompileLimits::default(),
        )
        .expect("fixture module compiles");
    PreparedVerilogARuntime::try_from_virtual_compilation(
        format!("__rspice_project__/fixture/{file}"),
        crate::product::ContentDigest::from_bytes([0x5a; 32]),
        module.to_owned(),
        &compilation,
    )
    .expect("fixture compilation seals into a prepared runtime")
}

fn seal_probe_runtime() -> PreparedVerilogARuntime {
    seal_runtime(
        "probe.va",
        "rspice_float_roundtrip_probe",
        EXACT_CONSTANT_SOURCE,
    )
}

/// A sealed runtime stores its `CompiledModel` as JSON and every consumer —
/// `registration`, `terminal_names`, `validate`, and the browser worker's
/// `compile_wasm_jit_artifact` — parses that string back. The parse has to
/// be bit exact, or a device built through the browser WASM JIT is built
/// from constants one unit in the last place away from the ones the native
/// path uses, for the same source and the same `artifact_digest`. The
/// digest cannot catch that: the JSON text is intact and only the parse is
/// lossy. Workspace `serde_json` carries `float_roundtrip` for this reason;
/// removing it fails here rather than silently perturbing device physics.
#[test]
fn sealed_model_constants_survive_the_json_the_runtime_ships() {
    let runtime = seal_probe_runtime();
    let decoded: rspice_veriloga::CompiledModel =
        serde_json::from_str(runtime.model_json.as_str()).expect("sealed model payload parses");

    for (name, expected) in LOSSY_MODEL_CONSTANTS {
        let parameter = decoded
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == name)
            .unwrap_or_else(|| panic!("probe model declares parameter '{name}'"));
        assert_eq!(
            parameter.default.to_bits(),
            expected.to_bits(),
            "parameter '{name}' came back as {:e} instead of {expected:e}",
            parameter.default
        );
    }
}

/// The same guarantee for the canonical IR, which is what the browser JIT
/// lowers. Both payloads travel together, but they are separate strings
/// with separate parses.
#[test]
fn sealed_canonical_ir_constants_survive_the_json_the_runtime_ships() {
    let runtime = seal_probe_runtime();
    let decoded: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(runtime.canonical_ir_json.as_str())
            .expect("sealed canonical IR payload parses");

    for (name, expected) in LOSSY_MODEL_CONSTANTS {
        let default = decoded
            .hir
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == name)
            .unwrap_or_else(|| panic!("probe IR declares parameter '{name}'"))
            .default
            .expect("probe parameters declare a literal default");
        assert_eq!(
            default.to_bits(),
            expected.to_bits(),
            "IR parameter '{name}' came back as {default:e} instead of {expected:e}"
        );
    }
}

/// `$bound_step` caps the next transient step. It lowers to a hidden task
/// variable reset to `+inf` at the top of every evaluation, so *every*
/// module that calls it carries an infinity in its compiled bytecode.
/// JSON has no spelling for one, so before `rspice_veriloga::json_float`
/// this module could not be sealed at all — `serde_json` wrote the reset
/// as `null` and the sealed payload then refused to decode, which took the
/// whole browser path away from every `$bound_step` model.
const BOUND_STEP_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rspice-simulation/tests/fixtures/bound_step.va"
));

/// The declaration-side spellings, and the silent ones. Both are folded
/// constants, so neither is announced by anything in the source text.
///
/// `folded` divides by a literal zero. The fold yields `+inf`, which lands
/// in `CompiledParameter.default` — a bare `f64` the sealed payload then
/// refused to decode — and in `HirParameter.default`, an `Option<f64>` that
/// decoded `null` back as `None`, the default simply gone with the artifact
/// digest over the JSON text agreeing nothing had happened.
///
/// `unbounded` reaches a *range*. The grammar's `inf` shortcut is not the
/// only way `inf` can appear in a bound: written as part of an expression it
/// is the ordinary constant again, folds to `+inf`, and lands in
/// `CompiledParameter.max` and `HirParamRange.max` — `Option<f64>` again.
///
/// `excluded` reaches the *third* range field, and it is the one place
/// where a bare `inf` really is an infinity: `exclude` takes a plain
/// expression, with none of a bound's absence rule, so `exclude inf` folds
/// to `+inf` and lands in `HirParamRange.exclude` and
/// `CompiledParameter.exclude` — a `Vec<f64>`, filled by the very same
/// closure that fills `min` and `max` (`SemanticAnalyzer::parse_range`).
///
/// `conductance` is the control, and it is here to keep a wrong premise
/// from coming back: `from (0:inf)` does *not* put an infinity anywhere.
/// The parser reads `inf` in a range bound as the absence of a bound
/// (`RangeBound.upper` is an `Option<Expression>` whose `None` means
/// `+inf`), so an open range never becomes a float at all — which is why
/// the shipped corpus, full of open ranges, carries no non-finite float.
const INFINITE_CONSTANT_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rspice-simulation/tests/fixtures/infinite_constant.va"
));

/// Every non-finite `PushConst` in a compiled program, however nested,
/// with its assigned variable when the expression writes a variable.
fn non_finite_constants(
    steps: &[rspice_veriloga::codegen::AssignmentStep],
) -> Vec<(Option<usize>, f64)> {
    fn constants(program: &rspice_veriloga::codegen::BytecodeProgram) -> Vec<f64> {
        program
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                rspice_veriloga::codegen::Instruction::PushConst(value) if !value.is_finite() => {
                    Some(*value)
                }
                _ => None,
            })
            .collect()
    }

    let mut found = Vec::new();
    for step in steps {
        match step {
            rspice_veriloga::codegen::AssignmentStep::Assign(assignment) => found.extend(
                constants(&assignment.program)
                    .into_iter()
                    .map(|value| (Some(assignment.var_index), value)),
            ),
            rspice_veriloga::codegen::AssignmentStep::AssignIndexed {
                base, index, value, ..
            } => {
                found.extend(constants(index).into_iter().map(|value| (None, value)));
                found.extend(
                    constants(value)
                        .into_iter()
                        .map(|value| (Some(*base), value)),
                );
            }
            rspice_veriloga::codegen::AssignmentStep::Loop { condition, body } => {
                found.extend(constants(condition).into_iter().map(|value| (None, value)));
                found.extend(non_finite_constants(body));
            }
            rspice_veriloga::codegen::AssignmentStep::Initialization { body, .. } => {
                found.extend(non_finite_constants(body));
            }
            rspice_veriloga::codegen::AssignmentStep::Task(task) => found.extend(
                task.expressions()
                    .flat_map(constants)
                    .map(|value| (None, value)),
            ),
        }
    }
    found
}

/// The seal is `serde_json::to_string`; decoding it is `from_str`. A
/// payload that is a fixed point of that pair lost nothing at all — not
/// only the non-finite values this change is about.
fn assert_payload_is_its_own_fixed_point<T>(what: &str, sealed: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let decoded: T = serde_json::from_str(sealed)
        .unwrap_or_else(|error| panic!("sealed {what} payload decodes: {error}"));
    let resealed = serde_json::to_string(&decoded).expect("decoded payload re-seals");
    assert_eq!(
        resealed, sealed,
        "the sealed {what} payload is not a fixed point of its own encoding"
    );
}

#[test]
fn a_bound_step_module_seals_with_its_infinite_step_sentinel_intact() {
    let runtime = seal_runtime(
        "bound-step.va",
        "rspice_bound_step_probe",
        BOUND_STEP_SOURCE,
    );
    assert_payload_is_its_own_fixed_point::<rspice_veriloga::CompiledModel>(
        "model",
        &runtime.model_json,
    );
    assert_payload_is_its_own_fixed_point::<rspice_veriloga::canonical_ir::CanonicalIrArtifact>(
        "canonical IR",
        &runtime.canonical_ir_json,
    );

    let decoded: rspice_veriloga::CompiledModel =
        serde_json::from_str(&runtime.model_json).expect("sealed model payload parses");
    let found = non_finite_constants(&decoded.assignment_steps);
    assert_eq!(
        found.len(),
        1,
        "the module resets exactly one task variable to a non-finite sentinel, found {found:?}"
    );
    let (var_index, value) = found[0];
    let var_index = var_index.expect("the step sentinel is assigned to a variable");
    assert_eq!(
        decoded.variable_names[var_index].as_str(),
        "$bound_step",
        "the surviving sentinel belongs to the step bound"
    );
    assert_eq!(
        value.to_bits(),
        f64::INFINITY.to_bits(),
        "the step bound came back as {value} instead of +inf"
    );

    // The canonical IR carries the same sentinel as an expression literal,
    // and it is the payload the browser JIT lowers. Asserted separately
    // because the two payloads are separate strings with separate parses.
    let ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(&runtime.canonical_ir_json).expect("sealed IR payload parses");
    let sentinels: Vec<f64> = ir
        .hir
        .expressions
        .iter()
        .filter_map(|expression| match &expression.kind {
            rspice_veriloga::canonical_ir::hir::HirExprKind::Number { value, .. }
                if !value.is_finite() =>
            {
                Some(*value)
            }
            _ => None,
        })
        .collect();
    // Lowering may retain separate literals for initialization and the
    // per-evaluation reset. Their count is a compiler detail; this seal
    // must retain every sentinel's exact sign and non-finite value.
    assert!(!sentinels.is_empty(), "the IR lost its step-bound sentinel");
    assert!(
        sentinels
            .iter()
            .all(|value| value.to_bits() == f64::INFINITY.to_bits()),
        "the IR step-bound sentinels arrived as {sentinels:?}"
    );
}

#[test]
fn an_infinite_folded_constant_survives_the_seal_in_both_payloads() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );
    assert_payload_is_its_own_fixed_point::<rspice_veriloga::CompiledModel>(
        "model",
        &runtime.model_json,
    );
    assert_payload_is_its_own_fixed_point::<rspice_veriloga::canonical_ir::CanonicalIrArtifact>(
        "canonical IR",
        &runtime.canonical_ir_json,
    );

    let decoded: rspice_veriloga::CompiledModel =
        serde_json::from_str(&runtime.model_json).expect("sealed model payload parses");
    let folded = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "folded")
        .expect("probe model declares the infinite parameter");
    assert_eq!(
        folded.default.to_bits(),
        f64::INFINITY.to_bits(),
        "the default came back as {} instead of +inf",
        folded.default
    );

    let ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(&runtime.canonical_ir_json).expect("sealed IR payload parses");
    let folded = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "folded")
        .expect("probe IR declares the infinite parameter");
    assert_eq!(
        folded.default.map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "the IR default arrived as {:?}, which is the silent loss",
        folded.default
    );

    let unbounded = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "unbounded")
        .expect("probe IR declares the expression-bounded range")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(
        unbounded.max.map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "an infinite bound expression arrived as {:?}",
        unbounded.max
    );
    assert_eq!(
        decoded
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == "unbounded")
            .and_then(|parameter| parameter.max)
            .map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "the bound the device range-checks against must be the declared one"
    );
}

/// The third range field, and the one whose loss was *loud*: `exclude` is
/// a `Vec<f64>` of bare elements, so before it was annotated `serde_json`
/// wrote the infinity as `null` and the whole payload then refused to
/// decode. The seal caught that and refused a legal declaration by field
/// path; now it encodes it instead.
#[test]
fn an_excluded_value_that_folds_to_an_infinity_survives_the_seal() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );

    let decoded: rspice_veriloga::CompiledModel =
        serde_json::from_str(&runtime.model_json).expect("sealed model payload parses");
    let excluded = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "excluded")
        .expect("probe model declares the excluding parameter");
    assert_eq!(
        excluded
            .exclude
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        vec![f64::INFINITY.to_bits()],
        "the excluded value arrived as {:?}",
        excluded.exclude
    );

    let ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(&runtime.canonical_ir_json).expect("sealed IR payload parses");
    let range = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "excluded")
        .expect("probe IR declares the excluding parameter")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(
        range
            .exclude
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        vec![f64::INFINITY.to_bits()],
        "the IR excluded value arrived as {:?}",
        range.exclude
    );

    // The seal already ran its guard — `seal_runtime` would have panicked
    // otherwise — but assert the guard's own answer too, so a regression
    // that puts `exclude` back in a refusal is named here rather than
    // showing up as an opaque panic.
    assert_eq!(
        rspice_veriloga::json_float::non_finite_floats("model", &decoded).unwrap(),
        Vec::new(),
        "no bare non-finite float may remain in the compiled model"
    );
    assert_eq!(
        rspice_veriloga::json_float::non_finite_floats("canonical_ir", &ir).unwrap(),
        Vec::new(),
        "no bare non-finite float may remain in the canonical IR"
    );
}

/// The control, and the correction of a premise worth not rediscovering:
/// an open range bound is an *absence*, not an infinity. Nothing about the
/// non-finite encoding applies to it, and the shipped corpus — hundreds of
/// `from (0:inf)` declarations across 43 modules — carries no non-finite
/// float because of this.
#[test]
fn an_open_range_bound_is_an_absence_rather_than_an_infinity() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );
    let decoded: rspice_veriloga::CompiledModel =
        serde_json::from_str(&runtime.model_json).expect("sealed model payload parses");
    let conductance = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "conductance")
        .expect("probe model declares the ranged parameter");
    assert_eq!(conductance.min.map(f64::to_bits), Some(0.0_f64.to_bits()));
    assert_eq!(conductance.max, None, "`inf` in a range bound is no bound");

    let ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(&runtime.canonical_ir_json).expect("sealed IR payload parses");
    let range = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "conductance")
        .expect("probe IR declares the ranged parameter")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(range.min.map(f64::to_bits), Some(0.0_f64.to_bits()));
    assert_eq!(range.max, None);
}

/// The guard the seal runs before it writes. Nothing in a compiled
/// artifact can reach it today — every field that carries a non-finite
/// float is annotated — so it is exercised on a value that stands in for
/// a field that acquired one without being annotated, which is the
/// regression it exists to catch.
///
/// The refusal is a variant a caller can match on, and it still renders
/// the exact sentence it rendered when it was only a `String`.
#[test]
fn the_seal_refuses_a_payload_carrying_a_float_json_cannot_encode() {
    let error = super::seal_payload_json(
        "model",
        "compiled Verilog-A model",
        &vec![1.0_f64, f64::INFINITY],
    )
    .expect_err("a bare non-finite float must stop the seal");
    let super::PreparedRuntimeError::NonFinite {
        description,
        source,
    } = &error
    else {
        panic!("an unencodable float must not be flattened into a sentence: {error:?}");
    };
    assert_eq!(*description, "compiled Verilog-A model");
    assert_eq!(
        *source,
        rspice_veriloga::json_float::NonFiniteFloatError::Unencodable(vec![
            rspice_veriloga::json_float::NonFiniteFloat {
                path: "model[1]".to_owned(),
                value: f64::INFINITY,
            }
        ])
    );
    assert_eq!(
        error.to_string(),
        "Could not seal compiled Verilog-A model: 1 non-finite float JSON cannot encode: \
             model[1] = inf"
    );
    assert_eq!(
        String::from(error),
        "Could not seal compiled Verilog-A model: 1 non-finite float JSON cannot encode: \
             model[1] = inf",
        "the `String` conversion every existing caller relies on is the same sentence"
    );
}
