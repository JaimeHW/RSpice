//! What the CFG route's noise magnitudes are, one representative shape each.
//!
//! [`cfg_mir_census`](super::cfg_mir_census) measures the whole shipped corpus
//! and is `#[ignore]`d release-qualification work. This is the same comparison
//! on a module small enough to read, run in the ordinary test pass, so that a
//! regression in the noise slice is caught by `cargo test` rather than by a
//! quarterly census.
//!
//! # What is pinned, and why each shape is here
//!
//! One module carries every shape the noise slice distinguishes, and each is
//! checked against the shipped postfix plan **by identity** — the same double,
//! not the same number to within a bound. Identity is available because a noise
//! magnitude is one expression lowered twice, and for these shapes the two
//! lowerings associate it the same way; the census carries the reassociation
//! bound for the compact models where they do not.
//!
//! * **plain, scaled, flicker exponent, routed into two equations.** These
//!   agreed before the site values existed, and pinning them is what stops the
//!   hoist from being credited with work it did not do. The scaled source is
//!   the one the module documentation used to blame: its amplitude belongs to
//!   the injection gain on *both* routes, so the two magnitudes were never
//!   different.
//! * **guarded with an inline operand.** The shipped route evaluates the
//!   magnitude unconditionally; the CFG's exit read is the seeded zero. This is
//!   the shape `angelov`'s `noise_exponents[9]` is — a `flicker_noise(...)`
//!   under `case (Noimod) 1:` with `Noimod` defaulting to `0` — and the one the
//!   site values exist for.
//! * **guarded with a variable-carried operand.** The shipped program is a
//!   `LoadVariable`, and the slot was filled by the guarded noise assignment
//!   pass, so the shipped magnitude is the *guarded* one. Hoisting here would
//!   disagree in the other direction, which is why
//!   [`NoiseMagnitude`](crate::jit::cfg_plan_builder) reads the decision off
//!   the shipped program. Its exponent is still a literal, so one source
//!   exercises both answers at once.
//! * **guarded by a bias-dependent condition.** The guard is not
//!   instance-static, so nothing folds it away before the CFG is built, and the
//!   hoist has to leave a real branch behind.

use super::cfg_census::OperatingPoint;
use crate::jit::cfg_plan_builder::{CfgNoiseScope, build_model_plan_from_canonical_cfg};
use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
use crate::{CompilerOptions, VerilogACompiler};

/// One module carrying every shape, with `mode` left at its default of zero so
/// the guarded sources are the ones the body does not reach.
const SHAPES: &str = r#"
module cfg_noise_shapes(p, n);
  inout p, n;
  electrical p, n;
  parameter integer mode = 0;
  parameter real gm = 3.0;
  parameter real pwr = 4.0e-21;
  real routed;
  real carried;
  analog begin
    I(p, n) <+ V(p, n) * gm;
    I(p, n) <+ white_noise(pwr, "plain");
    I(p, n) <+ gm * white_noise(pwr, "scaled");
    I(p, n) <+ flicker_noise(pwr, 2.0, "flicker");
    if (mode == 1) begin
      I(p, n) <+ flicker_noise(pwr * 2.0, 2.0, "guarded-inline");
    end
    if (mode == 1) begin
      carried = pwr * 3.0;
      I(p, n) <+ flicker_noise(carried, 2.0, "guarded-variable");
    end
    if (V(p, n) > 1.0e9) begin
      I(p, n) <+ white_noise(pwr * 5.0, "guarded-bias-condition");
    end
    routed = white_noise(pwr, "routed");
    I(p, n) <+ routed * gm;
    I(p, n) <+ routed * 2.0;
  end
endmodule
"#;

/// One entry read on both routes.
struct Reading {
    what: String,
    mir: f64,
    cfg: f64,
}

/// Every noise entry of `source`, read on both compiled plans at one bias.
///
/// The variable array is filled once, by the shipped plan's assignment pass,
/// and handed to both: the two plans carry the same assignment programs by
/// construction, and comparing value entries against an unfilled array would
/// compare a model to a different model. This is what
/// [`cfg_mir_census`](super::cfg_mir_census) does, on a module small enough to
/// state the expected numbers for.
fn readings(source: &str) -> Vec<Reading> {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");

    let cfg_plan = build_model_plan_from_canonical_cfg(&model, &artifact, CfgNoiseScope::Cfg)
        .unwrap_or_else(|refused| panic!("CFG plan: {refused}"));
    let mir_plan = build_model_plan_with_canonical_ir(&model, &artifact)
        .unwrap_or_else(|error| panic!("shipped plan: {error}"));
    let mir_native = crate::native::x64::compile_model_plan(&model, &mir_plan)
        .unwrap_or_else(|error| panic!("shipped codegen: {error}"));
    let cfg_native = crate::native::x64::compile_model_plan(&model, &cfg_plan.plan)
        .unwrap_or_else(|error| panic!("CFG codegen: {error}"));

    let parameter_defaults: Vec<Option<f64>> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default)
        .collect();
    let branch_unknowns =
        crate::jit::plan_builder::canonical_branch_unknown_runtime_map(&model, &artifact.mir)
            .expect("branch unknown map");
    let mir_storage = mir_native.required_storage();
    let cfg_storage = cfg_native.required_storage();
    let state_len = mir_storage
        .state_values
        .max(cfg_storage.state_values)
        .max(mir_storage.state_initialized)
        .max(cfg_storage.state_initialized)
        .max(mir_storage.state_candidate_valid)
        .max(cfg_storage.state_candidate_valid)
        + 8;
    let mut point = OperatingPoint::new(
        0x0005_EED1,
        0,
        &parameter_defaults,
        model.num_terminals,
        model.internal_nodes,
        &branch_unknowns,
        state_len,
        0,
    )
    .with_prelude_slots(cfg_storage.prelude_slots);
    let mut currents = vec![0.0_f64; model.stamp_programs.len() + 64];
    let terminals = model.num_terminals + 64;
    let mut branch_currents = vec![0.0_f64; terminals * terminals];
    let mut context = point.context();
    context.currents = currents.as_mut_ptr();
    context.currents_len = currents.len();
    context.branch_currents = branch_currents.as_mut_ptr();
    context.branch_currents_len = branch_currents.len();
    let mut variables = vec![0.0_f64; model.num_variables + 64];
    context.clear_runtime_error();
    mir_native.run_assignments(&context, variables.as_mut_ptr());
    // The CFG plan's own assignment pass, in the position the device runs it.
    cfg_native.run_prelude(&context, variables.as_ptr());
    let _ = context.take_runtime_error();

    let mut out = Vec::new();
    let mut read = |what: String, run: &dyn Fn(&crate::native::NativeModel) -> Option<f64>| {
        context.clear_runtime_error();
        let mir = run(&mir_native).unwrap_or_else(|| panic!("{what}: shipped plan has no entry"));
        assert!(
            context.take_runtime_error().is_none(),
            "{what}: the shipped entry raised a runtime error"
        );
        context.clear_runtime_error();
        let cfg = run(&cfg_native).unwrap_or_else(|| panic!("{what}: CFG plan has no entry"));
        assert!(
            context.take_runtime_error().is_none(),
            "{what}: the CFG entry raised a runtime error"
        );
        out.push(Reading { what, mir, cfg });
    };
    for (index, noise) in model.noise_sources.iter().enumerate() {
        let label = noise.name.as_deref().unwrap_or("<unnamed>").to_string();
        read(format!("noise_psd[{index}] {label}"), &|native| {
            native.run_noise_psd(index, &context, variables.as_ptr())
        });
        if noise.exponent_program.is_some() {
            read(format!("noise_exponents[{index}] {label}"), &|native| {
                native.run_noise_exponent(index, &context, variables.as_ptr())
            });
        }
    }
    out
}

/// Every shape, and the exact double both routes must produce.
///
/// Written out rather than compared route-to-route alone: a pin that only said
/// "the two agree" would stay green if both routes started answering zero,
/// which is precisely the failure this slice exists to prevent. `pwr` is
/// `4.0e-21`, so `pwr * 2.0` and `pwr * 5.0` are the guarded magnitudes and
/// the flicker exponents are the literal two.
#[test]
fn every_noise_shape_reads_the_same_double_on_both_routes() {
    const PWR: f64 = 4.0e-21;
    let expected: Vec<(&str, f64)> = vec![
        ("noise_psd[0] plain", PWR),
        ("noise_psd[1] scaled", PWR),
        ("noise_psd[2] flicker", PWR),
        ("noise_exponents[2] flicker", 2.0),
        // The shipped route computes this one unconditionally even though the
        // body never reaches it, so the CFG route takes it at its site.
        ("noise_psd[3] guarded-inline", PWR * 2.0),
        ("noise_exponents[3] guarded-inline", 2.0),
        // The shipped route reads a slot the guarded assignment pass left at
        // zero, so the CFG route takes the exit read, which is that same zero.
        // Its exponent is a literal and is taken at the site.
        ("noise_psd[4] guarded-variable", 0.0),
        ("noise_exponents[4] guarded-variable", 2.0),
        ("noise_psd[5] guarded-bias-condition", PWR * 5.0),
        ("noise_psd[6] routed", PWR),
    ];

    let readings = readings(SHAPES);
    let found: Vec<&str> = readings
        .iter()
        .map(|reading| reading.what.as_str())
        .collect();
    assert_eq!(
        found,
        expected.iter().map(|(what, _)| *what).collect::<Vec<_>>(),
        "the fixture's noise entries are not the ones this test names"
    );
    for (reading, (_, want)) in readings.iter().zip(&expected) {
        assert_eq!(
            reading.mir, *want,
            "{}: the shipped route no longer computes what this test was written against \
             (read {:.17e})",
            reading.what, reading.mir
        );
        assert_eq!(
            reading.cfg, reading.mir,
            "{}: the CFG route reads {:.17e} where the shipped route reads {:.17e}",
            reading.what, reading.cfg, reading.mir
        );
    }
}

/// The plan builder's own account of what it did, so the counts the census
/// prints are pinned against a module whose shapes are known.
///
/// Six magnitudes are prelude slots: the five unguarded ones, where the exit
/// read *is* the site value and so publishing it costs the module nothing, and
/// the guarded-variable source's power, whose shipped magnitude is a variable
/// read and whose quantity is therefore the exit read anyway. It is the one
/// entry the guarded-read count names.
///
/// Four are hoisted site values, which is the class no slot can hold: the two
/// guarded-inline magnitudes, the guarded-variable source's literal exponent
/// and the bias-guarded power. The shipped route evaluates each of those
/// unconditionally on a path the body may not take, so a slot written where the
/// value is computed would not be written at all.
#[test]
fn the_builder_says_which_magnitudes_it_hoisted() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(SHAPES).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(SHAPES)
        .expect("compile canonical IR");
    let built = build_model_plan_from_canonical_cfg(&model, &artifact, CfgNoiseScope::Cfg)
        .unwrap_or_else(|refused| panic!("CFG plan: {refused}"));

    assert_eq!(built.report.noise_values, 10);
    assert_eq!(built.report.noise_prelude_slots, 6);
    assert_eq!(built.report.noise_hoisted, 4);
    assert_eq!(
        built
            .report
            .noise_guarded_reads
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["noise_psd[4]".to_string()],
        "only the guarded source whose shipped magnitude is a variable read \
         keeps the exit value"
    );
}

/// Production takes no noise from the CFG, so asking for the postfix scope must
/// leave the plan's noise entries exactly as the shipped builder produced them
/// — not "equivalent to", the same programs.
#[test]
fn the_postfix_scope_takes_no_noise_from_the_cfg() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(SHAPES).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(SHAPES)
        .expect("compile canonical IR");
    let built = build_model_plan_from_canonical_cfg(&model, &artifact, CfgNoiseScope::Postfix)
        .unwrap_or_else(|refused| panic!("CFG plan: {refused}"));
    let shipped = build_model_plan_with_canonical_ir(&model, &artifact).expect("shipped plan");

    assert!(!built.plan.noise_psd.is_empty());
    for (index, entry) in built.plan.noise_psd.iter().enumerate() {
        assert_eq!(
            entry.borrow().form_name(),
            "postfix",
            "noise_psd[{index}] must stay the shipped program"
        );
    }
    assert_eq!(built.plan.noise_psd.len(), shipped.noise_psd.len());
    assert_eq!(built.report.noise_values, 0);
    assert!(built.report.noise_guarded_reads.is_empty());
    assert_eq!(built.report.noise_hoisted, 0);
    assert_eq!(built.report.noise_prelude_slots, 0);
}
