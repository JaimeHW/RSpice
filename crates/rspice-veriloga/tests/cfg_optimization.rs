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
fn homogeneous_math_derivatives_preserve_extreme_finite_scales() {
    for op in ["hypot", "atan2"] {
        for derivative in 0..3 {
            let expression = format!("{op}(V(p),V(q))");
            let expression = match derivative {
                1 => format!("ddx({expression},V(p))"),
                2 => format!("ddx({expression},V(q))"),
                _ => expression,
            };
            let artifact = artifact(&format!(
                "module planar(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
            ));
            let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
            let seeds = [
                AdSeed::NodePotential(0usize.into()),
                AdSeed::NodePotential(1usize.into()),
            ];
            let mut ad = differentiate(&cfg.function, &seeds).unwrap();
            let mut outputs = vec![cfg.residuals[0]];
            if derivative == 0 {
                outputs.extend(
                    ad.derivative_row(cfg.residuals[0])
                        .into_iter()
                        .map(Option::unwrap),
                );
            }
            let (optimized, moved) = optimize_cfg(&ad.function, &outputs);
            for scale in [1e-200, 1.0, 1e200, 8e307] {
                for (a, b) in [(-1.0_f64, 2.0_f64), (0.0, -2.0), (1.0, -1.0), (1.0, 1.0)] {
                    let (p, q) = (a * scale, b * scale);
                    let expected = if op == "hypot" {
                        let radius = a.hypot(b);
                        [p.hypot(q), a / radius, b / radius]
                    } else {
                        let square = a * a + b * b;
                        [p.atan2(q), (b / square) / scale, (-a / square) / scale]
                    };
                    let expected = if derivative == 0 {
                        &expected[..]
                    } else {
                        &expected[derivative..=derivative]
                    };
                    let mut inputs = inputs(&artifact);
                    inputs.node_potentials[..2].copy_from_slice(&[p, q]);
                    for (function, outputs) in [
                        (&ad.function, outputs.as_slice()),
                        (&optimized, moved.as_slice()),
                    ] {
                        let result = evaluate_cfg(function, &inputs).unwrap();
                        for (&output, &expected) in outputs.iter().zip(expected) {
                            let actual = result.value(output).unwrap();
                            if expected == 0.0 {
                                assert_eq!(actual, expected, "{expression} at {p},{q}");
                            } else {
                                assert!(
                                    (actual / expected - 1.0).abs() < 1e-12,
                                    "{expression} at {p},{q}: expected {expected}, got {actual}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn extrema_preserve_numeric_operands_nan_and_signed_zero_order() {
    for op in ["min", "max"] {
        for expression in [
            format!("{op}(V(p),sqrt(V(q)))"),
            format!("{op}(sqrt(V(q)),V(p))"),
            format!("atan2({op}(V(p),V(q)),-1.0)-atan2({op}(V(q),V(p)),-1.0)"),
        ] {
            let artifact = artifact(&format!(
                "module extrema(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
            ));
            let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
            let (optimized, outputs) = optimize_cfg(&cfg.function, &cfg.residuals);
            let mut inputs = inputs(&artifact);
            let expected = if expression.starts_with("atan2") {
                inputs.node_potentials[..2].copy_from_slice(&[-0.0, 0.0]);
                -2.0 * std::f64::consts::PI
            } else {
                inputs.node_potentials[..2].copy_from_slice(&[1.0, -1.0]);
                1.0
            };
            for (function, outputs) in [(&cfg.function, &cfg.residuals), (&optimized, &outputs)] {
                let actual = evaluate_cfg(function, &inputs)
                    .unwrap()
                    .value(outputs[0])
                    .unwrap();
                assert_eq!(actual, expected, "{expression}");
            }
        }
    }
}

#[test]
fn extrema_derivative_only_optimization_retains_checked_predicates() {
    let artifact = artifact(
        "module checked_extrema(p,q); inout p,q; electrical p,q; analog I(p)<+min(ddx(0.0/V(q),V(p))+V(p),V(p)); endmodule",
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
    let mut ad = differentiate(&cfg.function, &[AdSeed::NodePotential(0usize.into())]).unwrap();
    let output = ad.derivative(cfg.residuals[0], 0).unwrap();
    let (optimized, outputs) = optimize_cfg(&ad.function, &[output]);
    for q in [0.0, 1.0] {
        let mut inputs = inputs(&artifact);
        inputs.node_potentials[..2].copy_from_slice(&[1.0, q]);
        for (function, output) in [(&ad.function, output), (&optimized, outputs[0])] {
            let result = evaluate_cfg(function, &inputs);
            if q == 0.0 {
                assert!(
                    result.is_err(),
                    "derivative-only extrema erased checked primal: {result:?}"
                );
            } else {
                assert_eq!(result.unwrap().value(output).unwrap(), 1.0);
            }
        }
    }
}

#[test]
fn extrema_select_the_left_tangent_on_finite_ties() {
    for op in ["min", "max"] {
        for (left, right, bias, expected) in [
            ("V(p)", "2.0*V(q)", [2.0, 1.0], [1.0_f64, 0.0]),
            ("-0.0*V(p)", "0.0*V(q)", [1.0, 1.0], [-0.0, 0.0]),
        ] {
            let artifact = artifact(&format!(
                "module tie(p,q); inout p,q; electrical p,q; analog I(p)<+{op}({left},{right}); endmodule"
            ));
            let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
            let seeds = [
                AdSeed::NodePotential(0usize.into()),
                AdSeed::NodePotential(1usize.into()),
            ];
            let mut ad = differentiate(&cfg.function, &seeds).unwrap();
            let outputs = [
                ad.derivative(cfg.residuals[0], 0).unwrap(),
                ad.derivative(cfg.residuals[0], 1).unwrap(),
            ];
            for operand in 0..3 {
                let mut invalid = ad.function.clone();
                let selection = invalid
                    .values
                    .iter_mut()
                    .find(|value| matches!(value.kind, CfgValueKind::Select { .. }))
                    .unwrap();
                let id = selection.id;
                if let CfgValueKind::Select {
                    condition,
                    then_value,
                    else_value,
                } = &mut selection.kind
                {
                    *[condition, then_value, else_value][operand] = cfg.residuals[0];
                }
                assert_eq!(invalid.validate(),Err(rspice_veriloga::canonical_ir::cfg::CfgValidationError::SelectionTypeMismatch(id)));
            }
            let (optimized, moved) = optimize_cfg(&ad.function, &outputs);
            let mut inputs = inputs(&artifact);
            inputs.node_potentials[..2].copy_from_slice(&bias);
            for (function, outputs) in [
                (&ad.function, outputs.as_slice()),
                (&optimized, moved.as_slice()),
            ] {
                let values = evaluate_cfg(function, &inputs).unwrap();
                for (&output, expected) in outputs.iter().zip(expected) {
                    assert_eq!(
                        values.value(output).unwrap().to_bits(),
                        expected.to_bits(),
                        "{op}({left},{right}), value={output}"
                    );
                }
            }
        }
    }
}

#[test]
fn extrema_select_derivatives_without_inactive_singular_arithmetic() {
    for (expression, p) in [
        ("max(V(p),sqrt(V(q)))", 1.0),
        ("max(sqrt(V(q)),V(p))", 1.0),
        ("min(V(p),sqrt(V(q)))", -1.0),
        ("min(sqrt(V(q)),V(p))", -1.0),
    ] {
        let artifact = artifact(&format!(
            "module extrema_derivative(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
        ));
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
        let seeds = [
            AdSeed::NodePotential(0usize.into()),
            AdSeed::NodePotential(1usize.into()),
        ];
        let mut ad = differentiate(&cfg.function, &seeds).unwrap();
        let outputs = [
            cfg.residuals[0],
            ad.derivative(cfg.residuals[0], 0).unwrap(),
            ad.derivative(cfg.residuals[0], 1).unwrap(),
        ];
        let (optimized, moved) = optimize_cfg(&ad.function, &outputs);
        for q in [-1.0, 0.0] {
            let mut inputs = inputs(&artifact);
            inputs.node_potentials[..2].copy_from_slice(&[p, q]);
            for (function, outputs) in [
                (&ad.function, outputs.as_slice()),
                (&optimized, moved.as_slice()),
            ] {
                function.validate().unwrap();
                let result = evaluate_cfg(function, &inputs).unwrap();
                for (&output, expected) in outputs.iter().zip([p, 1.0, 0.0]) {
                    assert_eq!(
                        result.value(output).unwrap(),
                        expected,
                        "{expression}, q={q}, value={output}"
                    );
                }
            }
        }
    }
}

#[test]
fn fractional_power_rewrites_preserve_branch_cuts_and_infinite_limits() {
    for exponent in [0.5_f64, 1.5] {
        for (base, apply) in [
            ("V(p)", (|x: f64| x) as fn(f64) -> f64),
            ("sqrt(V(p))", |x| x.sqrt()),
            ("abs(V(p))", |x| x.abs()),
            ("max(sqrt(-1.0),V(p))", |x| x),
            ("1.0/sqrt(V(p))", |x| 1.0 / x.sqrt()),
            ("sqrt(V(p))/2.0", |x| x.sqrt() / 2.0),
            ("sqrt(V(p))+sqrt(V(p))", |x| x.sqrt() + x.sqrt()),
            ("sqrt(V(p))*sqrt(V(p))", |x| x.sqrt() * x.sqrt()),
            ("V(p)*V(p)", |x| x * x),
            ("exp(V(p))", |x| x.exp()),
            ("max(V(p),0.0)", |x| if x >= 0.0 { x } else { 0.0 }),
        ] {
            let artifact = artifact(&format!(
                "module fractional_power(p); inout p; electrical p;
                 analog I(p)<+atan2(pow({base},{exponent}),-1.0); endmodule"
            ));
            let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
            let (optimized, residuals) = optimize_cfg(&cfg.function, &cfg.residuals);
            for voltage in [f64::NEG_INFINITY, -1.0, -0.0, 0.0, 1.0, f64::INFINITY] {
                let mut inputs = inputs(&artifact);
                inputs.node_potentials[0] = voltage;
                let expected = apply(voltage).powf(exponent).atan2(-1.0);
                for (function, residuals) in
                    [(&cfg.function, &cfg.residuals), (&optimized, &residuals)]
                {
                    let actual = evaluate_cfg(function, &inputs)
                        .unwrap()
                        .value(residuals[0])
                        .unwrap();
                    assert!(
                        (expected.is_nan() && actual.is_nan())
                            || expected.to_bits() == actual.to_bits(),
                        "pow({base},{exponent}) at {voltage:?} before atan2: expected {expected:?}, got {actual:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn simplification_preserves_signed_zero_before_a_branch_cut() {
    for (expression, apply) in [
        ("0.0+V(p)", (|x: f64| 0.0 + x) as fn(f64) -> f64),
        ("V(p)+0.0", |x| x + 0.0),
        ("-0.0+V(p)", |x| -0.0 + x),
        ("V(p)+(-0.0)", |x| x + (-0.0)),
        ("V(p)-0.0", |x| x - 0.0),
        ("V(p)-(-0.0)", |x| x - (-0.0)),
    ] {
        let artifact = artifact(&format!(
            "module branch_cut(p); inout p; electrical p;
             analog I(p)<+atan2({expression},-1.0); endmodule"
        ));
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
        let (optimized, residuals) = optimize_cfg(&cfg.function, &cfg.residuals);
        for voltage in [-0.0, 0.0, -1.0, 1.0] {
            let mut inputs = inputs(&artifact);
            inputs.node_potentials[0] = voltage;
            let expected = apply(voltage).atan2(-1.0);
            for (function, residuals) in [(&cfg.function, &cfg.residuals), (&optimized, &residuals)]
            {
                let result = evaluate_cfg(function, &inputs).unwrap();
                assert_eq!(
                    result.value(residuals[0]).unwrap(),
                    expected,
                    "{expression} at {voltage:?}"
                );
            }
        }
    }
}

#[test]
fn packed_simplification_preserves_both_zero_signs() {
    use rspice_veriloga::canonical_ir::cfg::{
        CfgBlock, CfgInstruction, CfgTerminator, CfgValue, CfgValueType,
    };
    let artifact = artifact("module empty(p); inout p; electrical p; endmodule");
    let inputs = inputs(&artifact);
    for op in [CfgBinaryOp::Add, CfgBinaryOp::Sub] {
        for left in [-0.0_f64, 0.0] {
            for right in [-0.0_f64, 0.0] {
                let kinds = [
                    CfgValueKind::LaneSplat(left),
                    CfgValueKind::LaneSplat(right),
                    CfgValueKind::LaneBinary {
                        op,
                        left: 0usize.into(),
                        right: 1usize.into(),
                    },
                ];
                let function = CfgFunction {
                    entry: 0usize.into(),
                    blocks: vec![CfgBlock {
                        id: 0usize.into(),
                        params: vec![],
                        instructions: vec![CfgInstruction {
                            result: 2usize.into(),
                        }],
                        terminator: CfgTerminator::Return,
                    }],
                    values: kinds
                        .into_iter()
                        .enumerate()
                        .map(|(id, kind)| CfgValue {
                            id: id.into(),
                            kind,
                            value_type: CfgValueType::Lanes(0usize.into()),
                        })
                        .collect(),
                    shapes: vec![vec![0]],
                };
                function.validate().unwrap();
                let (optimized, outputs) = optimize_cfg(&function, &[2usize.into()]);
                optimized.validate().unwrap();
                let expected = if op == CfgBinaryOp::Add {
                    left + right
                } else {
                    left - right
                };
                let actual = evaluate_cfg(&optimized, &inputs).unwrap();
                assert_eq!(
                    actual.lanes(outputs[0]).unwrap()[0].to_bits(),
                    expected.to_bits(),
                    "{left:?} {op:?} {right:?}"
                );
            }
        }
    }
}

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

/// A merge nothing reads goes, and takes the edges into it with it.
///
/// Keeping it is what this pass used to do, because dropping one means
/// rewriting every edge into the block. The cost of not doing that is paid by
/// every slice: a guarded contribution's residual merges at the join, so asking
/// for anything else in the same function still dragged the residual — and
/// everything feeding it — along. Here that is a `ddt`, which is the case that
/// made it matter: a slice that keeps one has to call it.
#[test]
fn a_merge_nothing_reads_is_dropped_along_with_its_edges() {
    let artifact = artifact(
        r#"
module guarded_charge(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    parameter real g = 1.0e-3;
    parameter real store = 1.0;
    analog begin
        if (store > 0.5) begin
            I(p, n) <+ ddt(c * V(p, n));
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("fixture must lower");

    // The conduction residual only. The charge is the other contribution's, and
    // it is the one that merges at the join.
    let conduction = *cfg.residuals.last().expect("two contributions");
    let (optimized, _) = optimize_cfg(&cfg.function, &[conduction]);
    optimized
        .validate()
        .unwrap_or_else(|error| panic!("simplification produced {error}"));

    assert!(
        !optimized
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::Ddt { .. })),
        "the charge is read by nothing that was asked for, so its ddt must not survive"
    );
    let merges: usize = optimized
        .blocks
        .iter()
        .map(|block| block.params.len())
        .sum();
    assert_eq!(
        merges, 0,
        "no value asked for is defined on only one arm, so no merge is needed"
    );
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
        // Every declared port connected, which is what the CFG level itself
        // says: `$port_connected` folds to a constant one there. A shorter
        // vector would read as unconnected.
        port_connected: vec![true; artifact.hir.ports.len()],
        event_state: Vec::new(),
        event_controls: HashMap::new(),
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
        simparams: Default::default(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        integral_derivatives: Default::default(),
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
