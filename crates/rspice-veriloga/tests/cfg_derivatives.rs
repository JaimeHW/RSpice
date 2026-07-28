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

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg::CfgFunction;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, CfgScalar, ComplexStep, ValueId, differentiate,
    evaluate_cfg,
};
use std::collections::{HashMap, HashSet};

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
                .map(|entry| {
                    entry
                        .and_then(|value| snapshot.value(value))
                        .unwrap_or(0.0)
                })
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
