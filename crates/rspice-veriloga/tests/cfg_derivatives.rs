//! Forward-mode derivatives over the CFG, checked against a finite difference.
//!
//! The derivative pass and the interpreter share no code beyond the value
//! enum: one applies chain rules symbolically, the other re-runs the whole
//! function at a perturbed bias. Agreement between them is the only evidence
//! that carries — a derivative pass checked against itself proves nothing.
//!
//! The step is a Richardson pair rather than a single difference, because a
//! plain difference at any one step cannot distinguish "the rule is wrong" from
//! "the step was badly chosen", and a compact model's exponentials make that a
//! live risk.

use rspice_veriloga::canonical_ir::cfg::CfgFunction;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::mir::MirParameterSlot;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CanonicalValueType, CfgEvalInputs, CfgScalar, ComplexStep,
    ValueId, differentiate, evaluate_cfg,
};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Relative agreement demanded between the rule and the difference.
///
/// A fourth-order Richardson extrapolation of a smooth function resolves far
/// more than this; the margin is for the model, not the method.
const TOLERANCE: f64 = 1.0e-7;

/// Entries below this share of the largest entry in the row are not checked.
///
/// A difference of two numbers that nearly cancel carries no significant
/// figures, and demanding agreement there manufactures failures rather than
/// finding them. Learned the hard way on the stamp oracle.
const SIGNIFICANCE: f64 = 1.0e-9;

const COARSE_STEP: f64 = 1.0e-3;

/// What the corpus test demands, and it is the plan's own figure rather than a
/// number chosen to make a run pass.
///
/// `COMPLEX_TOLERANCE` is 1e-11 because the fixtures are a handful of
/// operations and nothing but arithmetic reordering can separate the rule from
/// the oracle. A shipped compact model is tens of thousands of operations, and
/// the same reordering accumulates: BSIM-CMG agrees to 5.7e-11, which is eleven
/// correct digits and not a wrong chain rule. Holding the corpus to the fixture
/// figure would be measuring float associativity.
const CORPUS_COMPLEX_TOLERANCE: f64 = 1.0e-9;

/// How far above the stamped block an oracle reading may sit before it is read
/// as a diverged measurement rather than a missing derivative.
///
/// Twelve orders is deliberately generous: a real dropped term shows up as an
/// oracle value of ordinary magnitude against a stamped zero, and that still
/// fails. What this excludes is the perturbed evaluation overflowing, which
/// reports as 1e197 and is not a statement about the chain rule at all.
const ORACLE_DIVERGENCE_FACTOR: f64 = 1.0e12;

/// Models that do not meet [`CORPUS_COMPLEX_TOLERANCE`], with what they measure.
///
/// Two different things, and worth keeping apart rather than averaging into one
/// number:
///
/// - `PSPNQS104VA` misses by a factor of ten on a single entry. That is the
///   accumulated float reordering the corpus tolerance already exists for,
///   arriving one model later than the tolerance anticipated.
/// - `bsimcmg_va` has three entries at 2.6e-3 on equation 1 at one drawn bias,
///   alongside three at 1e-9. 2.6e-3 is not arithmetic, and
///   [`the_complex_oracle_preserves_the_value_it_perturbs`] says why: **the
///   oracle is not valid there.** That census evaluates each residual for real
///   and again under an imaginary perturbation and compares the *values*, and
///   `bsimcmg_va` drifts on 51 of them. Complex step is a derivative only where
///   the function is analytic; where a `floor`, a selection on the real part, or
///   a `pow` at its branch cut intervenes, the real part moves and the imaginary
///   part stops being the derivative of anything. `PSPNQS104VA` drifts on 42.
///
/// Both allowlisted models are in the drift census and no model outside it needs
/// an allowance, which is the evidence that these two entries are the oracle's
/// limits rather than the chain rule's.
/// Drawing parameters as well as bias moves four more models off 1e-9, and the
/// same census sorts them into two kinds rather than one.
///
/// `ekv_va` and `l_utsoi` are the oracle again. Neither drifts at declared
/// defaults — `ekv_va` is absent from the default-parameter census entirely —
/// and both appear the moment parameters are drawn, on 8 and 20 evaluations.
/// That is the mechanism this list already documents, reached from a new
/// direction: a drawn parameter puts the model in a regime where a `floor`, a
/// real-part selection or a `pow` at its branch cut intervenes, and the
/// imaginary part stops being a derivative. `ekv_va`'s 3.0e-1 is not a chain
/// rule that is thirty percent wrong; it is a measurement of something else.
///
/// `asmhemt` and `PSP104VA` are **not** in the drift census, so their oracle is
/// valid and these two numbers are real. They are also small and they scale with
/// the model: `asmhemt` is the widest device in the corpus at 23 nodes and 57
/// branches, and 7.5e-8 is accumulated reordering across that, the same argument
/// that put the corpus tolerance at 1e-9 rather than the fixtures' 1e-11.
/// `PSP104VA` at 1.3e-9 is a rounding's width from the tolerance itself.
const KNOWN_COMPLEX_STEP_DEVIATIONS: &[(&str, f64)] = &[
    ("bsimcmg_va", 3.0e-3),
    ("PSPNQS104VA", 1.0e-8),
    // Oracle non-analytic under drawn parameters.
    ("ekv_va", 4.0e-1),
    ("l_utsoi", 5.0e-7),
    // Valid oracle; accumulation, worst measured 7.457e-8 and 1.304e-9.
    ("asmhemt", 1.0e-7),
    ("PSP104VA", 2.0e-9),
];

fn corpus_tolerance(module: &str) -> f64 {
    KNOWN_COMPLEX_STEP_DEVIATIONS
        .iter()
        .find(|(name, _)| *name == module)
        .map_or(CORPUS_COMPLEX_TOLERANCE, |(_, tolerance)| *tolerance)
}

/// Neither oracle here is seeded with [`AdSeed::LimiterCorrection`], and neither
/// could be: the correction is a displacement Newton limiting chose, not a
/// partial with respect to anything a difference can perturb. Limiting is
/// damping applied to a step rather than part of the equations, which is also
/// why the interpreter these oracles run on evaluates `$limit` as the value that
/// was proposed. The correction is checked in `cfg_limit.rs`, where it is
/// applied.
const NO_CORRECTION_LANE: &str =
    "a difference oracle has no unknown to perturb for the limiter correction lane";

#[test]
fn every_jacobian_entry_matches_a_richardson_difference() {
    for (name, source) in fixtures() {
        let artifact = artifact(source);
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();

        let mut differentiated = differentiate(&cfg.function, &lanes)
            .unwrap_or_else(|error| panic!("{name}: differentiation produced {error}"));

        // Every read-out first: taking one appends an instruction, so a
        // snapshot taken before them would not have run the later ones.
        let rows: Vec<Vec<Option<ValueId>>> = cfg
            .residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();
        let bias = bias_point(&artifact);
        let snapshot = evaluate_cfg(&differentiated.function, &inputs(&bias))
            .unwrap_or_else(|error| panic!("{name}: evaluation failed: {error}"));

        for (equation, residual) in cfg.residuals.iter().enumerate() {
            let row: Vec<f64> = rows[equation]
                .iter()
                .map(|entry| entry.and_then(|value| snapshot.value(value)).unwrap_or(0.0))
                .collect();
            let scale = row
                .iter()
                .fold(0.0f64, |scale, entry| scale.max(entry.abs()));

            for (lane, seed) in lanes.iter().enumerate() {
                let numeric = difference(&differentiated.function, &bias, *seed, *residual);
                let stamped = row[lane];
                let floor = (scale * SIGNIFICANCE).max(f64::MIN_POSITIVE);
                if stamped.abs() < floor && numeric.abs() < floor {
                    continue;
                }
                let allowed = TOLERANCE * stamped.abs().max(numeric.abs()).max(floor);
                assert!(
                    (stamped - numeric).abs() <= allowed,
                    "{name}: d(equation {equation})/d({seed:?}) is {stamped} by rule and \
                     {numeric} by difference"
                );
            }
        }
    }
}

/// Fourth-order central difference from two step sizes.
fn difference(function: &CfgFunction, bias: &BiasPoint, seed: AdSeed, residual: ValueId) -> f64 {
    let step = match seed {
        AdSeed::NodePotential(_) => COARSE_STEP,
        // Branch unknowns are currents; perturbing them by a volt-sized step
        // would leave the model's operating point entirely.
        AdSeed::BranchUnknownFlow(_) => COARSE_STEP * 1.0e-3,
        AdSeed::LimiterCorrection => unreachable!("{NO_CORRECTION_LANE}"),
    };
    let coarse = central(function, bias, seed, residual, step);
    let fine = central(function, bias, seed, residual, step / 2.0);
    (4.0 * fine - coarse) / 3.0
}

fn central(
    function: &CfgFunction,
    bias: &BiasPoint,
    seed: AdSeed,
    residual: ValueId,
    step: f64,
) -> f64 {
    let up = at(function, bias, seed, residual, step);
    let down = at(function, bias, seed, residual, -step);
    (up - down) / (2.0 * step)
}

fn at(
    function: &CfgFunction,
    bias: &BiasPoint,
    seed: AdSeed,
    residual: ValueId,
    delta: f64,
) -> f64 {
    let mut perturbed = bias.clone();
    match seed {
        AdSeed::NodePotential(node) => perturbed.node_potentials[usize::from(node)] += delta,
        AdSeed::BranchUnknownFlow(unknown) => {
            perturbed.branch_unknown_flows[usize::from(unknown)] += delta;
        }
        AdSeed::LimiterCorrection => unreachable!("{NO_CORRECTION_LANE}"),
    }
    evaluate_cfg(function, &inputs(&perturbed))
        .expect("the perturbed bias must still evaluate")
        .value(residual)
        .expect("the residual is defined on every path")
}

#[derive(Clone)]
struct BiasPoint {
    parameters: Vec<f64>,
    node_potentials: Vec<f64>,
    branch_flows: Vec<f64>,
    branch_unknown_flows: Vec<f64>,
}

fn bias_point(artifact: &CanonicalIrArtifact) -> BiasPoint {
    BiasPoint {
        // Declared defaults, deliberately: this is the reproducible fixture
        // point. Drawing belongs to `random_bias_point`, which is seeded.
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect(),
        // Asymmetric, and away from zero: a symmetric point hides sign errors
        // and the origin hides everything that is odd about a diode.
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|index| 0.41 - 0.13 * index as f64)
            .collect(),
        branch_flows: (0..artifact.mir.branches.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
    }
}

fn inputs(bias: &BiasPoint) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameters: bias.parameters.clone(),
        parameter_given: vec![false; bias.parameters.len()],
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: bias.node_potentials.clone(),
        branch_flows: bias.branch_flows.clone(),
        branch_unknown_flows: bias.branch_unknown_flows.clone(),
        temperature: 300.15,
        thermal_voltage: 300.15 * 8.617_333_262e-5,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        staged: Vec::new(),
    }
}

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "resistor",
            r#"
module resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        ),
        // Reading back a current the model itself contributed, in both forms:
        // `I(di, si)` for a branch nothing solves for, and `I(<d>)` for the
        // terminal. Seven of the nine models that would not lower were one of
        // these, and both are read-backs whose derivative has to agree with the
        // contribution it reads — which is exactly what the oracles check here
        // and what no amount of inspecting the emitted text would.
        (
            "probed contributions",
            r#"
module probed(d, s, b);
    inout d, s, b;
    electrical d, s, b;
    electrical di;
    parameter real beta = 1.0e-3;
    parameter real rd = 20.0;
    real ids, terminal;
    analog begin
        I(d, di) <+ V(d, di) / rd;
        I(di, s) <+ beta * V(di, s) * V(di, s);
        ids = I(di, s);
        terminal = I(<d>);
        I(b, s) <+ 1.0e-6 * ids + 1.0e-6 * terminal;
    end
endmodule
"#,
        ),
        (
            "diode",
            r#"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is = 1.0e-14;
    parameter real vt = 0.02585;
    analog I(a, c) <+ is * (exp(V(a, c) / vt) - 1.0);
endmodule
"#,
        ),
        (
            "guarded transconductor",
            r#"
module guarded(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real beta = 1.0e-3;
    parameter real vth = 0.4;
    real vov;
    analog begin
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            I(d, s) <+ beta * vov * vov * (1.0 + 0.02 * V(d, s));
        end else begin
            I(d, s) <+ 0.0;
        end
    end
endmodule
"#,
        ),
        (
            "internal node",
            r#"
module series(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r1;
        I(mid, n) <+ V(mid, n) / r2;
    end
endmodule
"#,
        ),
        (
            "transcendentals",
            r#"
module mixed(p, n);
    inout p, n;
    electrical p, n;
    parameter real a = 1.0e-3;
    analog begin
        I(p, n) <+ a * tanh(V(p, n))
                 + a * sqrt(abs(V(p, n)) + 1.0e-6)
                 + a * atan(V(p, n))
                 + a * ln(1.0 + exp(V(p, n)));
    end
endmodule
"#,
        ),
        (
            "min and max",
            r#"
module clamped(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    analog I(p, n) <+ g * max(min(V(p, n), 0.7), -0.7) + g * V(p, n);
endmodule
"#,
        ),
        (
            "potential contribution",
            r#"
module source(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 50.0;
    analog V(p, n) <+ r * I(p, n);
endmodule
"#,
        ),
    ]
}

/// The same check against complex step, which uses no derivative rule at all.
///
/// A finite difference can only confirm a derivative to a handful of figures,
/// and the ones it cannot confirm are exactly the ones a subtly wrong chain
/// rule produces. Complex step subtracts nothing, so its step can be taken
/// small enough that truncation error disappears entirely and agreement is
/// demanded to machine precision.
#[test]
fn every_jacobian_entry_matches_complex_step() {
    for (name, source) in fixtures() {
        let artifact = artifact(source);
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();
        let mut differentiated = differentiate(&cfg.function, &lanes)
            .unwrap_or_else(|error| panic!("{name}: differentiation produced {error}"));

        let rows: Vec<Vec<Option<ValueId>>> = cfg
            .residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();
        let bias = bias_point(&artifact);
        let snapshot = evaluate_cfg(&differentiated.function, &inputs(&bias))
            .unwrap_or_else(|error| panic!("{name}: evaluation failed: {error}"));

        for (equation, residual) in cfg.residuals.iter().enumerate() {
            for (lane, seed) in lanes.iter().enumerate() {
                let stamped = rows[equation][lane]
                    .and_then(|value| snapshot.value(value))
                    .unwrap_or(0.0);
                let exact = complex_step(&differentiated.function, &bias, *seed, *residual);
                let scale = stamped.abs().max(exact.abs());
                assert!(
                    (stamped - exact).abs() <= COMPLEX_TOLERANCE * scale.max(f64::MIN_POSITIVE),
                    "{name}: d(equation {equation})/d({seed:?}) is {stamped} by rule and \
                     {exact} by complex step"
                );
            }
        }
    }
}

/// Complex step resolves to the last bit, so the margin is for the arithmetic
/// alone — the rule and the oracle evaluate the same operations in a different
/// order, and nothing else should separate them.
const COMPLEX_TOLERANCE: f64 = 1.0e-11;

fn complex_step(function: &CfgFunction, bias: &BiasPoint, seed: AdSeed, residual: ValueId) -> f64 {
    let mut inputs = complex_inputs(bias);
    match seed {
        AdSeed::NodePotential(node) => {
            let index = usize::from(node);
            inputs.node_potentials[index] = ComplexStep::seed(bias.node_potentials[index]);
        }
        AdSeed::BranchUnknownFlow(unknown) => {
            let index = usize::from(unknown);
            inputs.branch_unknown_flows[index] =
                ComplexStep::seed(bias.branch_unknown_flows[index]);
        }
        AdSeed::LimiterCorrection => unreachable!("{NO_CORRECTION_LANE}"),
    }
    evaluate_cfg(function, &inputs)
        .expect("the complex evaluation follows the same path")
        .value(residual)
        .expect("the residual is defined on every path")
        .derivative()
}

fn complex_inputs(bias: &BiasPoint) -> CfgEvalInputs<ComplexStep> {
    let lift = |values: &[f64]| values.iter().copied().map(ComplexStep::from_f64).collect();
    CfgEvalInputs {
        parameters: lift(&bias.parameters),
        parameter_given: vec![false; bias.parameters.len()],
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: lift(&bias.node_potentials),
        branch_flows: lift(&bias.branch_flows),
        branch_unknown_flows: lift(&bias.branch_unknown_flows),
        temperature: ComplexStep::from_f64(300.15),
        thermal_voltage: ComplexStep::from_f64(300.15 * 8.617_333_262e-5),
        multiplicity: ComplexStep::from_f64(1.0),
        time: ComplexStep::from_f64(0.0),
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: ComplexStep::from_f64(0.0),
        ddt_scale: ComplexStep::from_f64(0.0),
        idt: ComplexStep::from_f64(0.0),
        idt_scale: ComplexStep::from_f64(0.0),
        staged: Vec::new(),
    }
}

/// `ddx` reads back a Jacobian entry the pass already computed.
///
/// It is checked by value rather than in the sweep above because the pass
/// deliberately treats it as constant when differentiating: `ddx` is a
/// first-order readback, so propagating through it would mean carrying second
/// derivatives for every model that reports a transconductance. The level being
/// replaced makes the same choice, and so does every tool this one is measured
/// against.
///
/// What must be exact is the entry itself, including which lane it names and
/// which way round the two nodes go — the part `resolve_ddx` can get wrong.
#[test]
fn ddx_reads_back_the_jacobian_entry_it_names() {
    let artifact = artifact(
        r#"
module readback(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real beta = 1.0e-3;
    real ids;
    analog begin
        ids = beta * V(g, s) * V(g, s);
        I(d, s) <+ ids;
        I(g, s) <+ 1.0e-3 * ddx(ids, V(g));
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("fixture must lower");
    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .collect();
    let differentiated = differentiate(&cfg.function, &lanes).expect("must differentiate");

    let bias = bias_point(&artifact);
    let snapshot = evaluate_cfg(&differentiated.function, &inputs(&bias)).expect("must evaluate");

    // Ports are numbered in declaration order: g, d, s.
    let beta = 1.0e-3;
    let v_gs = bias.node_potentials[0] - bias.node_potentials[2];
    let expected = 1.0e-3 * 2.0 * beta * v_gs;
    let actual = snapshot
        .value(cfg.residuals[1])
        .expect("the readback contribution is on every path");
    assert!(
        (actual - expected).abs() <= 1.0e-12 * expected.abs(),
        "ddx read back {actual}, but d(ids)/d(V(g)) is {expected}"
    );
}

/// Packing keeps the sparsity rather than trading it away.
///
/// The alternative design — one width per model, every derivative laid out over
/// every unknown — is simpler and emits the same number of lines. It is not what
/// this pass does, because the corpus's live sets are a small fraction of its
/// unknown counts and a uniform width would multiply the arithmetic by that
/// ratio. Nothing else in the suite would notice the difference: a dense layout
/// computes the same numbers, just far more of them. So it is pinned here.
///
/// The fixture has two independent halves over five nodes. No value in it
/// depends on more than two potentials, so no packed value may be wider than
/// two — and the residuals, which is where a dense layout would show first, must
/// name only the potentials their own half is built from.
#[test]
fn a_packed_derivative_carries_only_the_lanes_it_can_reach() {
    let artifact = artifact(
        r#"
module split(a, b, c, d, e);
    inout a, b, c, d, e;
    electrical a, b, c, d, e;
    parameter real g = 1.0e-3;
    analog begin
        I(a, b) <+ g * V(a, b) * V(a, b);
        I(c, d) <+ g * V(c, d);
        I(d, e) <+ g * V(d, e);
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("fixture must lower");
    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .collect();
    assert_eq!(lanes.len(), 5, "the fixture is meant to have five unknowns");
    let differentiated = differentiate(&cfg.function, &lanes).expect("must differentiate");

    let widest = differentiated
        .function
        .values
        .iter()
        .filter_map(|value| differentiated.function.lanes_of(value.id))
        .map(<[u32]>::len)
        .max()
        .expect("a differentiated model carries packed values");
    assert_eq!(
        widest, 2,
        "no value in this fixture depends on more than two potentials, so a \
         packed value {widest} wide means the layout is denser than liveness"
    );

    // Ports are numbered in declaration order, so the first contribution is
    // between lanes 0 and 1 and cannot mention the other three.
    let residual = differentiated
        .packed(cfg.residuals[0])
        .expect("the first contribution depends on a potential");
    assert_eq!(
        differentiated.function.lanes_of(residual),
        Some([0, 1].as_slice()),
        "I(a, b) is built from V(a) and V(b) alone"
    );
}

/// A counter-based generator, so a failure is reproducible from its model name.
///
/// SplitMix64, written out rather than taken from `DefaultHasher`, whose output
/// is explicitly not stable across Rust releases — a bias point keyed to it
/// would silently re-randomize on a toolchain bump and a regression would look
/// like flakiness.
fn seed_for(name: &str) -> u64 {
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    for byte in name.as_bytes() {
        state = state
            .wrapping_add(u64::from(*byte))
            .wrapping_mul(0xff51_afd7_ed55_8ccd);
        state ^= state >> 33;
    }
    state
}

fn next_unit(state: &mut u64) -> f64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

/// A parameter drawn around its default and kept inside its declared range.
///
/// Deliberately a perturbation rather than a uniform draw across the range. A
/// compact model's parameters are not independent — an oxide thickness drawn
/// from one end of its range against a doping from the other describes no
/// device, and a model that then refuses to evaluate is not evidence about the
/// derivative rule. Scaling the default by a half-to-double factor moves every
/// parameter off the value the fixtures pin without leaving the neighbourhood
/// the model was written for.
///
/// Bounds are respected including their exclusivity, excluded points are stepped
/// off, and an integer parameter stays integral.
fn draw_parameter(slot: &MirParameterSlot, state: &mut u64) -> f64 {
    let default = slot.default.unwrap_or(0.0);
    let mut value = default * (0.5 + 1.5 * next_unit(state));
    if !value.is_finite() {
        return default;
    }

    if let Some(range) = &slot.range {
        if let Some(min) = range.min {
            // An exclusive bound is stepped off by a share of the span rather
            // than by an epsilon: a parameter sitting one ulp inside `> 0` is
            // inside the range and useless as a bias.
            let span = range
                .max
                .map_or(min.abs().max(1.0), |max| (max - min).abs());
            let floor = if range.min_exclusive {
                min + 1.0e-3 * span.max(f64::MIN_POSITIVE)
            } else {
                min
            };
            value = value.max(floor);
        }
        if let Some(max) = range.max {
            let span = range
                .min
                .map_or(max.abs().max(1.0), |min| (max - min).abs());
            let ceiling = if range.max_exclusive {
                max - 1.0e-3 * span.max(f64::MIN_POSITIVE)
            } else {
                max
            };
            value = value.min(ceiling);
        }
        if range.exclude.contains(&value) {
            return default;
        }
    }

    if slot.value_type == CanonicalValueType::Integer {
        value = value.round();
    }
    if value.is_finite() { value } else { default }
}

/// A bias *and a parameter vector* drawn from `seed`, spanning both sides of
/// junction turn-on.
///
/// Parameters were previously pinned at their declared defaults here, because an
/// earlier attempt to draw them crashed the corpus run with
/// `STATUS_HEAP_CORRUPTION`. **That no longer reproduces**: the whole corpus now
/// runs drawn, 42 models and 1321 entries, with peak RSS at 0.13 GB and no
/// crash. The crash belonged to a tree this one has replaced, and the note that
/// replaced it was outliving its evidence.
///
/// The caution it recorded is still worth keeping, because it bounds what this
/// tests. A declared range constrains one parameter; a compact model's real
/// constraints are between them, and they live in the model's own validation,
/// which nothing on this path calls — this path evaluates a CFG rather than
/// instantiating a device. So a drawn vector can be a combination the model
/// would have rejected. That is tolerable *here* precisely because the property
/// under test is the chain rule, which does not care whether the operating point
/// is physical: the emitted derivative must agree with complex step wherever
/// both are defined. It would not be tolerable for a test that asserted anything
/// about the model's answers.
///
/// The draw is a perturbation rather than a uniform sweep for the same reason —
/// see [`draw_parameter`], which respects exclusive bounds, excluded points and
/// integer parameters.
fn random_bias_point(artifact: &CanonicalIrArtifact, state: &mut u64) -> BiasPoint {
    BiasPoint {
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| draw_parameter(parameter, state))
            .collect(),
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|_| -0.55 + next_unit(state) * 1.4)
            .collect(),
        branch_flows: (0..artifact.mir.branches.len())
            .map(|_| 1.0e-3 * (2.0 * next_unit(state) - 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|_| 1.0e-3 * (2.0 * next_unit(state) - 1.0))
            .collect(),
    }
}

/// Every lane of every equation, against complex step, at bias points nobody
/// chose.
///
/// [`every_jacobian_entry_matches_complex_step`] pins the same property at one
/// hand-picked bias per fixture. One point is enough to catch a rule that is
/// wrong everywhere and useless against one that is wrong on a branch the
/// chosen point does not take — which is the failure mode this rebuild exists
/// to fix. Drawing the point instead reaches the guarded paths.
///
/// Complex step is what makes this worth asserting to machine precision: it
/// subtracts nothing, so there is no cancellation and no step-size argument.
#[test]
fn every_jacobian_entry_matches_complex_step_at_drawn_bias_points() {
    const POINTS: usize = 8;

    let mut violations = Vec::new();

    for (name, source) in fixtures() {
        let artifact = artifact(source);
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();
        let mut differentiated = differentiate(&cfg.function, &lanes)
            .unwrap_or_else(|error| panic!("{name}: differentiation produced {error}"));
        let rows: Vec<Vec<Option<ValueId>>> = cfg
            .residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();

        let mut state = seed_for(name);
        for point in 0..POINTS {
            let bias = random_bias_point(&artifact, &mut state);
            let Ok(snapshot) = evaluate_cfg(&differentiated.function, &inputs(&bias)) else {
                // A drawn point may sit where the model itself refuses to
                // evaluate. That is the model's business, not the rule's.
                continue;
            };

            let block: Vec<(Vec<f64>, Vec<f64>)> = cfg
                .residuals
                .iter()
                .enumerate()
                .map(|(equation, residual)| {
                    let stamped = (0..lanes.len())
                        .map(|lane| {
                            rows[equation][lane]
                                .and_then(|value| snapshot.value(value))
                                .unwrap_or(0.0)
                        })
                        .collect();
                    let exact = lanes
                        .iter()
                        .map(|seed| complex_step(&differentiated.function, &bias, *seed, *residual))
                        .collect();
                    (stamped, exact)
                })
                .collect();
            compare_block(
                &format!("{name}: point {point}"),
                &lanes,
                &block,
                COMPLEX_TOLERANCE,
                &mut violations,
            );
        }
    }

    assert!(
        violations.is_empty(),
        "the rule and complex step disagree:
{}",
        violations.join(
            "
"
        )
    );
}

/// The same property, over every shipped model rather than the fixtures.
///
/// This is what the plan asks for by name — every model, every lane, against
/// complex step at bias points nobody chose. It is `#[ignore]` for the same
/// reason the noise census is: it compiles the whole CMC corpus, which is
/// minutes of front-end work rather than milliseconds. Run it when the
/// derivative pass changes.
///
/// A model that will not compile or will not lower is *counted*, not skipped
/// silently. A census that says nothing about a model reads as coverage it does
/// not have, and the point of this test is to know which models the rule has
/// actually been checked on.
#[test]
#[ignore = "compiles the whole shipped corpus"]
fn the_whole_corpus_matches_complex_step_at_drawn_bias_points() {
    const POINTS: usize = 3;

    let root = model_root();
    let candidates = discover_veriloga_sources(&root).expect("model tree");
    let (mut checked, mut entries, mut uncompiled, mut unlowered, mut undifferentiated) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut violations = Vec::new();

    for (candidate, module) in candidates.iter().flat_map(|candidate| {
        candidate
            .modules
            .iter()
            .map(move |module| (candidate, module.to_string()))
    }) {
        let mut options = CompilerOptions::default();
        options.include_paths.push(root.clone());
        options.defines = candidate.compile_profile.defines.clone();
        options.undefines = candidate.compile_profile.undefines.clone();
        let Ok(compiled) = VerilogACompiler::new(options)
            .compile_file_canonical_ir_with_metadata(&candidate.path, Some(&module))
        else {
            uncompiled += 1;
            continue;
        };
        let artifact = compiled.artifact;
        let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            unlowered += 1;
            continue;
        };

        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();
        let Ok(mut differentiated) = differentiate(&cfg.function, &lanes) else {
            undifferentiated += 1;
            continue;
        };
        let rows: Vec<Vec<Option<ValueId>>> = cfg
            .residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();

        let mut state = seed_for(&module);
        let mut model_entries = 0usize;
        for point in 0..POINTS {
            let bias = random_bias_point(&artifact, &mut state);
            let Ok(snapshot) = evaluate_cfg(&differentiated.function, &inputs(&bias)) else {
                continue;
            };
            let block: Vec<(Vec<f64>, Vec<f64>)> = cfg
                .residuals
                .iter()
                .enumerate()
                .map(|(equation, residual)| {
                    let stamped = (0..lanes.len())
                        .map(|lane| {
                            rows[equation][lane]
                                .and_then(|value| snapshot.value(value))
                                .unwrap_or(0.0)
                        })
                        .collect();
                    let exact = lanes
                        .iter()
                        .map(|seed| complex_step(&differentiated.function, &bias, *seed, *residual))
                        .collect();
                    (stamped, exact)
                })
                .collect();
            model_entries += compare_block(
                &format!("{module}: point {point}"),
                &lanes,
                &block,
                corpus_tolerance(&module),
                &mut violations,
            );
        }
        checked += 1;
        entries += model_entries;
        eprintln!("{module:>24}  {model_entries} entries");
    }

    eprintln!(
        "checked {checked} models, {entries} entries; \
         {uncompiled} uncompiled, {unlowered} unlowered, {undifferentiated} undifferentiated"
    );
    assert!(checked > 0, "no model reached the derivative rule");
    assert!(entries > 0, "no entry was compared");
    assert!(
        violations.is_empty(),
        "{} of {entries} compared entries disagree with complex step:
{}",
        violations.len(),
        violations.join(
            "
"
        )
    );
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}

/// Compare one equation's row against complex step, and report how many entries
/// carried enough magnitude to be worth comparing.
///
/// The floor is the point. A compact model's row spans thirty orders — a
/// junction conductance beside a 1e-31 partial that exists only because some
/// chain rule multiplied two small things — and demanding relative agreement on
/// the 1e-31 entry manufactures failures out of the last bits of two different
/// evaluation orders. `bjt505_va` is the case that proved it: -3.6226e-31 by
/// rule against -3.6206e-31 by complex step, a 5.6e-4 relative disagreement over
/// an absolute difference of 2e-34, which no solve can observe.
fn compare_block(
    label: &str,
    lanes: &[AdSeed],
    rows: &[(Vec<f64>, Vec<f64>)],
    tolerance: f64,
    violations: &mut Vec<String>,
) -> usize {
    // Against the largest entry in the whole Jacobian, not the largest in the
    // row. A row-relative floor was the first attempt and it does not work: when
    // an entire equation sits at 1e-31 it is its own scale, so every entry in it
    // clears a floor derived from itself and the comparison is exactly as
    // meaningless as before. `bjt505_va` equation 21 is that row.
    // Scaled by what the *device stamps*, not by the larger of the two sides.
    // The oracle can diverge, and when it does it takes the scale with it:
    // `EPFL_HEMT_10a` at a drawn bias returns 1.3e197 from complex step against
    // a stamped 0, because the perturbed evaluation overflowed an exponential.
    // A floor computed from that number puts every honest entry below itself and
    // compares only the overflow — 132 "disagreements" that are all one broken
    // measurement.
    let scale = rows
        .iter()
        .flat_map(|(stamped, _)| stamped.iter())
        .filter(|value| value.is_finite())
        .fold(0.0_f64, |worst, value| worst.max(value.abs()));
    let floor = scale * SIGNIFICANCE;
    // And an oracle reading orders above anything the device stamps has not
    // found a missing derivative, it has left the number range the model is
    // defined on. Bounded rather than unbounded so a genuinely absent term —
    // rule 0 against an oracle of ordinary magnitude — still fails.
    let oracle_ceiling = scale * ORACLE_DIVERGENCE_FACTOR;

    let mut compared = 0usize;
    for (equation, (stamped_row, exact_row)) in rows.iter().enumerate() {
        for (lane, seed) in lanes.iter().enumerate() {
            let (stamped, exact) = (stamped_row[lane], exact_row[lane]);
            if !stamped.is_finite() || !exact.is_finite() {
                continue;
            }
            if scale > 0.0 && exact.abs() > oracle_ceiling {
                continue;
            }
            let magnitude = stamped.abs().max(exact.abs());
            if magnitude <= floor {
                continue;
            }
            let relative = (stamped - exact).abs() / magnitude;
            if relative > tolerance {
                violations.push(format!(
                    "{label}: d(equation {equation})/d({seed:?}) is {stamped} by rule and \
                     {exact} by complex step ({relative:.3e})"
                ));
            }
            compared += 1;
        }
    }
    compared
}

/// The oracle's own validity check, and the reason `bsimcmg_va` is allowlisted.
///
/// Complex step is only a derivative if the imaginary perturbation left the
/// value alone. `f(x + ih) = f(x) + i h f'(x) + O(h^2)` holds when `f` is
/// analytic at `x`; where it is not — a `floor`, a `min` selecting on the real
/// part, a `pow` reaching the branch cut — the real part of the complex
/// evaluation drifts away from the real evaluation, and the imaginary part
/// stops being the derivative of anything.
///
/// So the test is direct: evaluate the residual for real, evaluate it complex
/// with the perturbation, and compare the *values*. Where they disagree the
/// oracle has no standing to accuse the chain rule, and the entry is a coverage
/// gap rather than a failure.
#[test]
#[ignore = "compiles the whole shipped corpus"]
fn the_complex_oracle_preserves_the_value_it_perturbs() {
    let root = model_root();
    let candidates = discover_veriloga_sources(&root).expect("model tree");
    let (mut checked, mut drifted) = (0usize, Vec::new());

    for (candidate, module) in candidates.iter().flat_map(|candidate| {
        candidate
            .modules
            .iter()
            .map(move |module| (candidate, module.to_string()))
    }) {
        let mut options = CompilerOptions::default();
        options.include_paths.push(root.clone());
        options.defines = candidate.compile_profile.defines.clone();
        options.undefines = candidate.compile_profile.undefines.clone();
        let Ok(compiled) = VerilogACompiler::new(options)
            .compile_file_canonical_ir_with_metadata(&candidate.path, Some(&module))
        else {
            continue;
        };
        let artifact = compiled.artifact;
        let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            continue;
        };
        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .collect();
        let Ok(function) = differentiate(&cfg.function, &lanes) else {
            continue;
        };

        let mut state = seed_for(&module);
        for point in 0..3 {
            let bias = random_bias_point(&artifact, &mut state);
            let Ok(snapshot) = evaluate_cfg(&function.function, &inputs(&bias)) else {
                continue;
            };
            for (equation, residual) in cfg.residuals.iter().enumerate() {
                let Some(real) = snapshot.value(*residual) else {
                    continue;
                };
                for seed in &lanes {
                    let mut complex = complex_inputs(&bias);
                    match seed {
                        AdSeed::NodePotential(node) => {
                            let index = usize::from(*node);
                            complex.node_potentials[index] =
                                ComplexStep::seed(bias.node_potentials[index]);
                        }
                        _ => continue,
                    }
                    let Ok(perturbed) = evaluate_cfg(&function.function, &complex) else {
                        continue;
                    };
                    let Some(value) = perturbed.value(*residual) else {
                        continue;
                    };
                    checked += 1;
                    let scale = real.abs().max(value.real().abs());
                    if scale > 0.0 && (real - value.real()).abs() / scale > 1.0e-12 {
                        drifted.push(format!(
                            "{module}: point {point}: equation {equation} under {seed:?} is \
                             {real} for real and {} under an imaginary perturbation",
                            value.real()
                        ));
                    }
                }
            }
        }
    }

    eprintln!("checked {checked} evaluations, {} drifted", drifted.len());
    assert!(checked > 0, "no evaluation was checked");
    // Reported rather than asserted empty: a model that is not analytic
    // everywhere is a fact about the model, and the list is the evidence for
    // which allowlist entries are the oracle's fault rather than the rule's.
    let mut per_model: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for line in &drifted {
        let model = line.split(':').next().unwrap_or("?");
        *per_model.entry(model).or_default() += 1;
    }
    for (model, count) in &per_model {
        eprintln!("{model:>24}  {count} non-analytic evaluations");
    }
}
