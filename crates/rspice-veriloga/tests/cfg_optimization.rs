//! The CFG simplification passes: they must shrink the graph and change nothing.
//!
//! Both halves matter and they pull against each other. A pass that shrinks
//! nothing is dead weight in the pipeline; a pass that shrinks by changing an
//! answer is worse than no pass at all. Every fixture here is checked for both,
//! and the residual is compared before and after at the same bias.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg::{CfgBinaryOp, CfgFunction, CfgUnaryOp, CfgValueKind};
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, differentiate, evaluate_cfg, optimize_cfg,
};
use std::collections::{HashMap, HashSet};

#[test]
fn simplification_preserves_every_residual() {
    for (name, source) in fixtures() {
        let artifact = artifact(source);
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

        let (optimized, residuals) = optimize_cfg(&cfg.function, &cfg.residuals);
        optimized
            .validate()
            .unwrap_or_else(|error| panic!("{name}: simplification produced {error}"));

        let inputs = inputs(&artifact);
        let before = evaluate_cfg(&cfg.function, &inputs)
            .unwrap_or_else(|error| panic!("{name}: original failed: {error}"));
        let after = evaluate_cfg(&optimized, &inputs)
            .unwrap_or_else(|error| panic!("{name}: simplified failed: {error}"));

        for (equation, (original, simplified)) in
            cfg.residuals.iter().zip(residuals.iter()).enumerate()
        {
            let expected = before.value(*original).expect("residual is defined");
            let actual = after.value(*simplified).expect("residual is defined");
            assert!(
                (expected - actual).abs() <= 1.0e-12 * expected.abs().max(1.0),
                "{name}: equation {equation} became {actual}, was {expected}"
            );
        }
    }
}

/// Simplifying after differentiating must not disturb the Jacobian either.
///
/// This is where a wrong dominance test shows up: merging two expressions from
/// sibling arms is invisible in the primal of a fixture whose arms agree, and
/// immediately wrong in the derivative.
#[test]
fn simplification_preserves_the_jacobian() {
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
        let mut differentiated =
            differentiate(&cfg.function, &lanes).unwrap_or_else(|error| panic!("{name}: {error}"));

        let outputs: Vec<_> = cfg
            .residuals
            .iter()
            .flat_map(|residual| (0..lanes.len()).map(move |lane| (*residual, lane)))
            .filter_map(|(residual, lane)| differentiated.derivative(residual, lane))
            .collect();

        let (optimized, moved) = optimize_cfg(&differentiated.function, &outputs);
        optimized
            .validate()
            .unwrap_or_else(|error| panic!("{name}: simplification produced {error}"));

        let inputs = inputs(&artifact);
        let before = evaluate_cfg(&differentiated.function, &inputs).expect("evaluates");
        let after = evaluate_cfg(&optimized, &inputs).expect("evaluates");
        for (original, simplified) in outputs.iter().zip(moved.iter()) {
            let expected = before.value(*original).expect("derivative is defined");
            let actual = after.value(*simplified).expect("derivative is defined");
            assert!(
                (expected - actual).abs() <= 1.0e-12 * expected.abs().max(1.0),
                "{name}: a Jacobian entry became {actual}, was {expected}"
            );
        }
    }
}

/// The passes have to earn their place: a differentiated model is where the
/// duplication actually is, since every lane repeats the primal's shape.
#[test]
fn simplification_shrinks_a_differentiated_model() {
    let artifact = artifact(
        r#"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 100.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r + 1.0e-14 * (exp(V(p, mid) / 0.02585) - 1.0);
        I(mid, n) <+ V(mid, n) / r + 1.0e-14 * (exp(V(mid, n) / 0.02585) - 1.0);
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .collect();
    let differentiated = differentiate(&cfg.function, &lanes).expect("differentiates");

    let outputs: Vec<_> = cfg.residuals.clone();
    let before = differentiated.function.values.len();
    let (optimized, _) = optimize_cfg(&differentiated.function, &outputs);
    let after = optimized.values.len();

    eprintln!("differentiated divider: {before} values before, {after} after");
    assert!(
        after * 2 < before,
        "simplification left {after} values of {before}; it is not paying for itself"
    );
}

#[test]
fn a_squared_power_becomes_a_multiply() {
    let artifact = artifact(
        r#"
module squarer(p, n);
    inout p, n;
    electrical p, n;
    parameter real k = 1.0e-3;
    analog I(p, n) <+ k * pow(V(p, n), 2.0);
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let (optimized, _) = optimize_cfg(&cfg.function, &cfg.residuals);
    assert!(
        !uses(&optimized, |kind| matches!(
            kind,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Pow,
                ..
            }
        )),
        "a constant square must not survive as a call to pow"
    );
}

#[test]
fn a_half_power_becomes_a_square_root() {
    let artifact = artifact(
        r#"
module rooted(p, n);
    inout p, n;
    electrical p, n;
    parameter real k = 1.0e-3;
    analog I(p, n) <+ k * pow(abs(V(p, n)) + 1.0e-9, 0.5);
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let (optimized, _) = optimize_cfg(&cfg.function, &cfg.residuals);
    assert!(
        uses(&optimized, |kind| matches!(
            kind,
            CfgValueKind::Unary {
                op: CfgUnaryOp::Sqrt,
                ..
            }
        )),
        "pow(x, 0.5) must become a square root"
    );
}

fn uses(function: &CfgFunction, predicate: impl Fn(&CfgValueKind) -> bool) -> bool {
    function.values.iter().any(|value| predicate(&value.kind))
}

fn inputs(artifact: &CanonicalIrArtifact) -> CfgEvalInputs<f64> {
    let parameters: Vec<f64> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default.unwrap_or(0.0))
        .collect();
    let parameter_given = vec![false; parameters.len()];
    CfgEvalInputs {
        parameters,
        parameter_given,
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|index| 0.37 - 0.12 * index as f64)
            .collect(),
        branch_flows: (0..artifact.mir.branches.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        temperature: 300.15,
        thermal_voltage: 300.15 * 8.617_333_262e-5,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: 0.0,
        ddt_scale: 0.0,
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
            "constants throughout",
            r#"
module folded(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = (2.0 * 3.0 + 4.0) / 1000.0;
        I(p, n) <+ g * V(p, n) * 1.0 + 0.0;
    end
endmodule
"#,
        ),
        (
            "arms that differ",
            r#"
module branching(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    real g;
    analog begin
        if (sel > 0.5) begin
            g = 1.0e-3 * exp(V(p, n));
        end else begin
            g = 2.0e-3 * exp(V(p, n));
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
        ),
        (
            "repeated subexpression",
            r#"
module repeated(p, n);
    inout p, n;
    electrical p, n;
    parameter real is = 1.0e-14;
    analog begin
        I(p, n) <+ is * (exp(V(p, n) / 0.02585) - 1.0)
                 + is * (exp(V(p, n) / 0.02585) - 1.0);
    end
endmodule
"#,
        ),
        (
            "powers",
            r#"
module powered(p, n);
    inout p, n;
    electrical p, n;
    parameter real k = 1.0e-3;
    analog begin
        I(p, n) <+ k * pow(V(p, n), 2.0)
                 + k * pow(V(p, n), 3.0)
                 + k * pow(abs(V(p, n)) + 1.0e-9, 1.5);
    end
endmodule
"#,
        ),
        (
            "loop",
            r#"
module summed(p, n);
    inout p, n;
    electrical p, n;
    parameter integer steps = 3;
    parameter real k = 1.0e-3;
    real total;
    integer i;
    analog begin
        total = 0.0;
        i = 0;
        while (i < steps) begin
            total = total + k * V(p, n);
            i = i + 1;
        end
        I(p, n) <+ total;
    end
endmodule
"#,
        ),
    ]
}
