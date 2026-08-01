//! `$limit`: the convention, and the lane that pays for it.
//!
//! A limited evaluation is a function of the bias *and* of the previous
//! iterate, so it cannot be checked the way every other derivative in this
//! crate is — perturbing an unknown and re-running perturbs the limiter too.
//! What can be checked, and is here, is the pair of claims the convention
//! rests on:
//!
//! 1. The Jacobian is the *proposed* value's, `dL/dv := 1`. Chaining through
//!    the limiter body instead is self-consistent and useless to a solver: a
//!    saturating limiter has slope zero wherever its clamp is active, so the
//!    device would linearise as disconnected on exactly the iterations that
//!    asked for help.
//! 2. The correction lane is the directional derivative along the displacement
//!    the limiter introduced — `dI/dL * (L - v)` — which is what the stamp
//!    subtracts from the residual to put the linearisation back at `v`.
//!
//! Both are checked against arithmetic written out by hand rather than against
//! another pass, because the thing under test *is* the convention. A second
//! implementation of the same convention would agree with a wrong one.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg::{CfgFunction, CfgValueKind};
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, differentiate, evaluate_cfg,
};
use std::collections::{HashMap, HashSet};

/// Agreement demanded against hand arithmetic. The rule and the expectation
/// evaluate the same operations in a different order and nothing else should
/// separate them.
const TOLERANCE: f64 = 1.0e-12;

const IS: f64 = 1.0e-14;
const VT: f64 = 0.025;
const VMAX: f64 = 0.4;

/// A diode whose junction voltage is clamped by a user-defined limiter.
///
/// The clamp is a plain `min`, which makes the discriminating case explicit:
/// above `VMAX` the limiter body's own slope is zero, so a rule that chained
/// through it would report no conductance at all.
const CLAMPED_DIODE: &str = r#"
module clamped_diode(p, n);
    inout p, n;
    electrical p, n;
    parameter real is_sat = 1.0e-14;
    parameter real vt = 0.025;
    parameter real vmax = 0.4;

    analog function real clampv;
        input proposed, previous, ceiling;
        real proposed, previous, ceiling;
        begin
            clampv = proposed > ceiling ? ceiling : proposed;
        end
    endfunction

    real vd;
    analog begin
        vd = $limit(V(p, n), "clampv", vmax);
        I(p, n) <+ is_sat * (exp(vd / vt) - 1.0);
    end
endmodule
"#;

#[test]
fn the_jacobian_is_the_proposed_values_even_where_the_clamp_is_active() {
    // Well above the clamp, which is where the two rules disagree most.
    for bias in [0.2, VMAX, 0.7] {
        let row = differentiate_at(bias);
        // `d/dv` of `is*(exp(vd/vt) - 1)` with `d(vd)/dv := 1`, at whatever
        // point `vd` was evaluated. The reference interpreter is the
        // limiting-disabled semantics, so that point is `v`.
        let expected = IS / VT * (bias / VT).exp();
        let stamped = row.node(0);
        assert!(
            (stamped - expected).abs() <= TOLERANCE * expected.abs(),
            "at {bias} V the conductance is {stamped}, expected {expected}"
        );
        // The half that matters. `d(min(v, vmax))/dv` is zero above the clamp,
        // so a rule chaining through the limiter body would report exactly this
        // and the device would look disconnected.
        assert!(
            stamped > 0.0,
            "at {bias} V the device stamps no conductance at all"
        );
    }
}

#[test]
fn the_correction_is_the_displacement_seen_through_the_residual() {
    for bias in [0.2, VMAX, 0.7] {
        let row = differentiate_at(bias);
        // The limiter clamps at `VMAX`, so the displacement is what it removed.
        let displacement = VMAX.min(bias) - bias;
        let expected = IS / VT * (bias / VT).exp() * displacement;
        let correction = row.correction();
        assert!(
            (correction - expected).abs() <= TOLERANCE * expected.abs().max(f64::MIN_POSITIVE),
            "at {bias} V the correction is {correction}, expected {expected}"
        );
    }
}

#[test]
fn below_the_clamp_the_correction_is_exactly_zero() {
    // Not "small": the limiter returned what was proposed, so the displacement
    // is a subtraction of a number from itself. A correction that is merely
    // tiny here would mean the lane had picked up arithmetic from somewhere.
    let row = differentiate_at(0.1);
    assert_eq!(row.correction(), 0.0);
}

#[test]
fn a_model_without_a_limiter_carries_no_correction_lane() {
    // The lane is reserved per model, so a model that does not limit must not
    // pay a slot for one — and every other lane index still has to mean
    // "unknown number n", which is what the stamp plan reads it as.
    let artifact = artifact(
        r#"
module plain_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0e3;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("the resistor lowers");
    assert!(
        !limits(&cfg.function),
        "a resistor should not lower a limiter"
    );
}

/// The Jacobian row of the single current contribution, split the way the stamp
/// plan splits it.
struct Row {
    entries: Vec<f64>,
    correction: usize,
}

impl Row {
    fn node(&self, index: usize) -> f64 {
        self.entries[index]
    }

    fn correction(&self) -> f64 {
        self.entries[self.correction]
    }
}

/// Differentiate the clamped diode at one bias and read its only row.
fn differentiate_at(bias: f64) -> Row {
    let artifact = artifact(CLAMPED_DIODE);
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("the clamped diode lowers: {diagnostics:?}"));
    assert!(
        limits(&cfg.function),
        "the fixture must actually reach $limit, or it tests nothing"
    );

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .chain(std::iter::once(AdSeed::LimiterCorrection))
        .collect();
    let correction = lanes.len() - 1;

    let mut differentiated =
        differentiate(&cfg.function, &lanes).expect("the clamped diode differentiates");
    let residual = cfg.residuals[0];
    let row = differentiated.derivative_row(residual);

    let mut inputs = inputs(&artifact);
    inputs.node_potentials[0] = bias;
    let snapshot = evaluate_cfg(&differentiated.function, &inputs)
        .expect("the differentiated diode evaluates");

    let entries = row
        .iter()
        .map(|entry| entry.and_then(|value| snapshot.value(value)).unwrap_or(0.0))
        .collect();
    Row {
        entries,
        correction,
    }
}

fn limits(function: &CfgFunction) -> bool {
    function
        .values
        .iter()
        .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }))
}

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("the fixture compiles to canonical IR")
}

/// Defaults everywhere, with node zero the one the tests move.
fn inputs(artifact: &CanonicalIrArtifact) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect(),
        parameter_given: vec![false; artifact.mir.parameters.len()],
        node_potentials: vec![0.0; artifact.mir.nodes.len()],
        branch_flows: vec![0.0; artifact.mir.branches.len()],
        branch_unknown_flows: vec![0.0; artifact.mir.branch_unknowns.len()],
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
