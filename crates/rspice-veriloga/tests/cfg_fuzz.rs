//! Randomly generated modules, through the whole front end and the AD pass.
//!
//! The hand-written fixtures elsewhere test the constructs someone thought to
//! write down. This generates them instead — nested guards, loops, ternaries and
//! read-backs in combinations nobody chose — and asserts the two properties that
//! have to hold for every one of them:
//!
//! 1. A module that compiles must lower to a CFG, and that CFG must
//!    differentiate. A panic, or a lowering that quietly refuses, is a finding.
//! 2. Every lane of the differentiated function must agree with complex step.
//!
//! Complex step is what makes the second assertion worth making at machine
//! precision: it perturbs along the imaginary axis, so nothing is subtracted and
//! there is no step size to argue about. The rule and the oracle share only the
//! value enum.
//!
//! Seeded and deterministic. A failure prints the seed and the source, and
//! re-running reproduces it exactly — a fuzzer whose failures cannot be replayed
//! is a random number generator with extra steps.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg::CfgFunction;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, CfgScalar, ComplexStep, ValueId, differentiate,
    evaluate_cfg,
};
use std::collections::{HashMap, HashSet};

/// How many modules each test draws.
const CASES: usize = 400;

/// Relative agreement demanded between the rule and complex step.
///
/// Generated bodies are small — tens of operations — so the only thing that can
/// separate the two is the order the arithmetic happens in.
const TOLERANCE: f64 = 1.0e-10;

/// Entries below this share of the largest in the block are not compared.
///
/// Same reason as everywhere else in this crate: a partial thirty orders below
/// the block carries no significant figures and demanding agreement on it
/// manufactures failures.
const SIGNIFICANCE: f64 = 1.0e-9;

/// Bias magnitude, in volts. Wide enough to put a generated `exp` on both sides
/// of unity without reaching the range where it overflows.
const BIAS: f64 = 0.35;

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed ^ 0x9e37_79b9_7f4a_7c15)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// An expression over the two node voltages, a parameter, and the running
/// variables.
///
/// Everything here is chosen to be finite and differentiable on the bias range:
/// no division except by a strictly positive quantity, no `ln` or `sqrt` of
/// anything that can reach zero, no `pow` with a variable exponent. The point of
/// this fuzzer is the *shape* of the control flow and the dataflow through it,
/// not rediscovering that `ln(0)` is undefined.
fn expression(rng: &mut Rng, depth: usize, variables: &[&str]) -> String {
    if depth == 0 {
        return match rng.below(5) {
            0 => "V(a, b)".to_string(),
            1 => "V(b, c)".to_string(),
            2 => "scale".to_string(),
            3 => format!("{:.4}", 0.25 + rng.unit()),
            _ => variables[rng.below(variables.len())].to_string(),
        };
    }

    let left = expression(rng, depth - 1, variables);
    let right = expression(rng, depth - 1, variables);
    match rng.below(10) {
        0 => format!("({left} + {right})"),
        1 => format!("({left} - {right})"),
        2 => format!("({left} * {right})"),
        // Guarded: the denominator is an offset square, so it cannot reach zero
        // however the operands are drawn.
        3 => format!("({left} / (1.0 + {right} * {right}))"),
        4 => format!("exp(-({left} * {left}))"),
        5 => format!("ln(1.0 + {left} * {left})"),
        6 => format!("sqrt(1.0 + {left} * {left})"),
        7 => format!("tanh({left})"),
        8 => format!("(({left} > {right}) ? {left} : {right})"),
        _ => format!("abs({left})"),
    }
}

/// A statement: an assignment, a guard, or a bounded loop.
fn statement(rng: &mut Rng, depth: usize, variables: &[&str]) -> String {
    let target = variables[rng.below(variables.len())];
    match rng.below(6) {
        0..=2 => format!("{target} = {};\n", expression(rng, 2, variables)),
        3 => format!(
            "if ({} > {}) {target} = {}; else {target} = {};\n",
            expression(rng, 1, variables),
            expression(rng, 1, variables),
            expression(rng, 2, variables),
            expression(rng, 2, variables),
        ),
        4 if depth > 0 => {
            let inner = statement(rng, depth - 1, variables);
            format!(
                "if ({} > 0.0) begin\n    {inner}end\n",
                expression(rng, 1, variables)
            )
        }
        // A loop with a constant trip count: the body runs, carries values
        // across iterations, and terminates regardless of the bias.
        _ => {
            let inner = statement(rng, 0, variables);
            format!(
                "begin\n    idx = 0;\n    while (idx < 2) begin\n        {inner}        idx = idx + 1;\n    end\nend\n"
            )
        }
    }
}

fn module(seed: u64) -> String {
    let mut rng = Rng::new(seed);
    let variables = ["u", "v", "w"];
    let body: String = (0..2 + rng.below(3))
        .map(|_| statement(&mut rng, 1, &variables))
        .collect();

    format!(
        r#"
module fuzzed(a, b, c);
    inout a, b, c;
    electrical a, b, c;
    parameter real scale = 0.75;
    real u, v, w;
    integer idx;
    analog begin
        u = 0.5;
        v = 1.0;
        w = V(a, b);
        idx = 0;
{body}
        I(a, b) <+ {};
        I(b, c) <+ {};
    end
endmodule
"#,
        expression(&mut rng, 2, &variables),
        expression(&mut rng, 2, &variables),
    )
}

#[derive(Clone)]
struct Bias {
    parameters: Vec<f64>,
    /// One flag per declared port, all connected — the CFG level's own
    /// convention, where `$port_connected` folds to a constant one. A shorter
    /// vector would read as *unconnected*, which no drawn module means.
    port_connected: Vec<bool>,
    node_potentials: Vec<f64>,
    branch_unknown_flows: Vec<f64>,
}

fn bias(artifact: &CanonicalIrArtifact, rng: &mut Rng) -> Bias {
    Bias {
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect(),
        port_connected: vec![true; artifact.hir.ports.len()],
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|_| BIAS * (2.0 * rng.unit() - 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|_| 1.0e-3 * (2.0 * rng.unit() - 1.0))
            .collect(),
    }
}

fn inputs<S: CfgScalar>(bias: &Bias, promote: impl Fn(f64) -> S) -> CfgEvalInputs<S> {
    CfgEvalInputs {
        parameters: bias.parameters.iter().copied().map(&promote).collect(),
        parameter_given: vec![false; bias.parameters.len()],
        port_connected: bias.port_connected.clone(),
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: bias.node_potentials.iter().copied().map(&promote).collect(),
        branch_flows: Vec::new(),
        branch_unknown_flows: bias
            .branch_unknown_flows
            .iter()
            .copied()
            .map(&promote)
            .collect(),
        temperature: promote(300.15),
        thermal_voltage: promote(300.15 * 8.617_333_262e-5),
        multiplicity: promote(1.0),
        time: promote(0.0),
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: promote(0.0),
        ddt_scale: promote(0.0),
        idt: promote(0.0),
        idt_scale: promote(0.0),
        staged: Vec::new(),
    }
}

/// The derivative of `residual` along `seed`, by an imaginary perturbation.
fn complex_step(function: &CfgFunction, bias: &Bias, seed: AdSeed, residual: ValueId) -> f64 {
    const STEP: f64 = 1.0e-30;
    let mut evaluated = inputs(bias, |value| ComplexStep::new(value, 0.0));
    match seed {
        AdSeed::NodePotential(node) => {
            evaluated.node_potentials[usize::from(node)] =
                ComplexStep::new(bias.node_potentials[usize::from(node)], STEP);
        }
        AdSeed::BranchUnknownFlow(unknown) => {
            evaluated.branch_unknown_flows[usize::from(unknown)] =
                ComplexStep::new(bias.branch_unknown_flows[usize::from(unknown)], STEP);
        }
        // Neither is an input this interpreter exposes. The limiter correction
        // is a displacement rather than an unknown, and a noise process
        // evaluates as a constant zero — it is the fluctuation about the
        // operating point, not part of it — so no imaginary step can move
        // either. The lanes drawn above hold node potentials only, so neither
        // arm is reached; keeping the helper total is what a fuzz loop wants.
        AdSeed::LimiterCorrection | AdSeed::NoiseProcess(_) => return 0.0,
    }
    evaluate_cfg(function, &evaluated)
        .ok()
        .and_then(|snapshot| snapshot.value(residual))
        .map_or(0.0, |value| value.im / STEP)
}

#[test]
fn drawn_modules_lower_and_differentiate() {
    let (mut compiled, mut lowered, mut differentiated) = (0usize, 0usize, 0usize);
    let mut refusals = Vec::new();

    for seed in 0..CASES as u64 {
        let source = module(seed);
        // A generator that emits something the front end rejects is a bug in the
        // generator, not a finding about the compiler, so this is counted rather
        // than asserted — but a *drop* in the count is visible below.
        let Ok(artifact) = VerilogACompiler::default().compile_canonical_ir(&source) else {
            continue;
        };
        compiled += 1;

        let cfg = match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
            Ok(cfg) => cfg,
            Err(diagnostics) => {
                refusals.push(format!(
                    "seed {seed}: lowering refused: {diagnostics:?}\n{source}"
                ));
                continue;
            }
        };
        lowered += 1;

        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .collect();
        match differentiate(&cfg.function, &lanes) {
            Ok(_) => differentiated += 1,
            Err(error) => {
                refusals.push(format!(
                    "seed {seed}: differentiation refused: {error}\n{source}"
                ));
            }
        }
    }

    eprintln!("compiled {compiled}, lowered {lowered}, differentiated {differentiated}");
    assert!(
        compiled > CASES / 2,
        "the generator emitted mostly invalid modules: only {compiled} of {CASES} compiled"
    );
    assert!(
        refusals.is_empty(),
        "{} drawn modules were refused after compiling:\n{}",
        refusals.len(),
        refusals.join("\n---\n")
    );
}

#[test]
fn drawn_modules_agree_with_complex_step() {
    let mut compared = 0usize;
    let mut violations = Vec::new();

    for seed in 0..CASES as u64 {
        let source = module(seed);
        let Ok(artifact) = VerilogACompiler::default().compile_canonical_ir(&source) else {
            continue;
        };
        let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            continue;
        };
        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .collect();
        let Ok(mut function) = differentiate(&cfg.function, &lanes) else {
            continue;
        };
        let rows: Vec<Vec<Option<ValueId>>> = cfg
            .residuals
            .iter()
            .map(|residual| function.derivative_row(*residual))
            .collect();

        let mut rng = Rng::new(seed ^ 0xa5a5_a5a5);
        for point in 0..2 {
            let bias = bias(&artifact, &mut rng);
            let Ok(snapshot) = evaluate_cfg(&function.function, &inputs(&bias, |value| value))
            else {
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
                        .map(|seed| complex_step(&function.function, &bias, *seed, *residual))
                        .collect();
                    (stamped, exact)
                })
                .collect();

            let scale = block
                .iter()
                .flat_map(|(stamped, _)| stamped.iter())
                .filter(|value| value.is_finite())
                .fold(0.0_f64, |worst, value| worst.max(value.abs()));
            let floor = scale * SIGNIFICANCE;

            for (equation, (stamped_row, exact_row)) in block.iter().enumerate() {
                for (lane, lane_seed) in lanes.iter().enumerate() {
                    let (stamped, exact) = (stamped_row[lane], exact_row[lane]);
                    if !stamped.is_finite() || !exact.is_finite() {
                        continue;
                    }
                    let magnitude = stamped.abs().max(exact.abs());
                    if magnitude <= floor {
                        continue;
                    }
                    compared += 1;
                    let relative = (stamped - exact).abs() / magnitude;
                    if relative > TOLERANCE {
                        violations.push(format!(
                            "seed {seed} point {point}: d(equation {equation})/d({lane_seed:?}) \
                             is {stamped} by rule and {exact} by complex step ({relative:.3e})\n\
                             {source}"
                        ));
                    }
                }
            }
        }
    }

    eprintln!("compared {compared} entries over {CASES} drawn modules");
    assert!(compared > 0, "no entry was compared");
    assert!(
        violations.is_empty(),
        "{} entries disagree with complex step:\n{}",
        violations.len(),
        violations.join("\n---\n")
    );
}
