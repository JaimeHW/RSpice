//! Splitting a body by invalidation class, checked by running the pieces.
//!
//! The split is a program transformation, and the only property that matters is
//! that it does not change the answer: running the stages in order, threading
//! each one's cached values into the next, must reproduce what the whole
//! function computes in one go. Bit-identically — the stages evaluate the same
//! operations on the same values, so anything less is a real difference.
//!
//! Checking that the split *happened* matters too, and separately. A split that
//! quietly put everything in the last stage would satisfy the equivalence test
//! perfectly and be worth nothing.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::schedule::{InvalidationClass, split, worth_splitting};
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, ValueId, differentiate, evaluate_cfg, optimize_cfg,
    schedule_cfg,
};
use std::collections::{HashMap, HashSet};

#[test]
fn the_stages_together_compute_what_the_whole_body_computes() {
    for (name, source) in fixtures() {
        let (function, wanted) = pipeline(source, name);

        let whole = evaluate_cfg(&function, &inputs(&[]))
            .unwrap_or_else(|error| panic!("{name}: whole body: {error}"));
        let expected: Vec<f64> = wanted
            .iter()
            .map(|value| whole.value(*value).expect("defined on every path"))
            .collect();

        let schedule = schedule_cfg(&function);
        let stages = split(&function, &schedule, &wanted)
            .unwrap_or_else(|error| panic!("{name}: split: {error}"));

        // Run them in order, each one seeing what the coarser ones cached.
        let slots = stages
            .iter()
            .flat_map(|stage| stage.exports.iter().map(|(slot, _)| *slot + 1))
            .max()
            .unwrap_or(0) as usize;
        let mut staged = vec![0.0f64; slots];
        let mut actual: Vec<Option<f64>> = vec![None; wanted.len()];
        for stage in &stages {
            let snapshot = evaluate_cfg(&stage.function, &inputs(&staged))
                .unwrap_or_else(|error| panic!("{name}: {} stage: {error}", stage.class.name()));
            for (slot, value) in &stage.exports {
                // A value defined only inside a conditional the run did not
                // take has nothing to cache — and nothing reads it either, for
                // the same reason.
                if let Some(held) = snapshot.value(*value) {
                    staged[*slot as usize] = held;
                }
            }
            for (index, output) in stage.outputs.iter().enumerate() {
                if let Some(output) = output {
                    actual[index] = snapshot.value(*output);
                }
            }
        }

        for (index, (produced, expected)) in actual.iter().zip(&expected).enumerate() {
            let produced =
                produced.unwrap_or_else(|| panic!("{name}: output {index} was left unproduced"));
            assert!(
                produced.to_bits() == expected.to_bits()
                    || (produced.is_nan() && expected.is_nan()),
                "{name}: output {index} is {produced} from the stages and {expected} whole"
            );
        }
    }
}

/// The split has to actually move work, and this is where that is pinned.
///
/// The fixture bins a parameter, folds in the temperature, and only then
/// touches the bias — the shape of every real compact model's prologue. If the
/// classification collapsed, everything would land in the Newton stage and the
/// equivalence test above would still pass.
#[test]
fn work_that_does_not_depend_on_the_bias_leaves_the_newton_stage() {
    let source = r#"
module staged(d, s);
    inout d, s;
    electrical d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, ids;
    analog begin
        geometry = width * width * 1.0e12;
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        ids = geometry * (V(d, s) - vth) * (V(d, s) - vth);
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#;
    let (function, wanted) = pipeline(source, "staged");
    let schedule = schedule_cfg(&function);
    let census = schedule.census();

    assert!(
        census[InvalidationClass::Instance as usize] > 0,
        "the geometry term reads only parameters, so something must be instance-static; \
         census was {census:?}"
    );
    assert!(
        census[InvalidationClass::Temperature as usize] > 0,
        "the threshold shift reads $temperature and no bias, so something must be \
         temperature-static; census was {census:?}"
    );

    let stages = split(&function, &schedule, &wanted).expect("splits");
    let classes: Vec<InvalidationClass> = stages.iter().map(|stage| stage.class).collect();
    assert!(
        classes.contains(&InvalidationClass::Instance)
            && classes.contains(&InvalidationClass::Temperature)
            && classes.contains(&InvalidationClass::Newton),
        "expected a stage per class in use, got {classes:?}"
    );
    assert!(
        stages.iter().any(|stage| !stage.exports.is_empty()),
        "a split that caches nothing has not split anything"
    );
}

/// Splitting is a caching decision, and it has to be declined when there is
/// nothing to cache.
///
/// Measured on real models: a body that is ~90% bias-dependent runs *slower*
/// split than whole, because every value a coarser stage computed then arrives
/// through a slot instead of a register and no saved work pays for those loads.
/// So the two fixtures here are the two answers — one with a real prologue and
/// one with almost none — and the rule has to tell them apart.
#[test]
fn a_body_with_nothing_to_cache_is_not_split() {
    let staged = r#"
module staged(d, s);
    inout d, s;
    electrical d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, corner, ids;
    analog begin
        geometry = width * width * 1.0e12;
        corner = geometry * geometry + geometry * 3.0 - 1.0e-3;
        vth = vth0 - 1.0e-3 * ($temperature - tnom) + corner * 1.0e-9;
        ids = (V(d, s) - vth) * (V(d, s) - vth);
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#;
    // Every value here traces back to the bias, so there is no prologue to
    // hoist and nothing a slot could usefully hold.
    let all_bias = r#"
module allbias(d, s);
    inout d, s;
    electrical d, s;
    analog begin
        I(d, s) <+ 1.0e-6 * V(d, s) * V(d, s) * exp(V(d, s) / 0.02585);
    end
endmodule
"#;

    for (name, source, expected) in [
        ("staged", staged, true),
        ("all bias-dependent", all_bias, false),
    ] {
        let (function, wanted) = pipeline(source, name);
        let schedule = schedule_cfg(&function);
        let stages = split(&function, &schedule, &wanted).expect("splits");
        assert_eq!(
            worth_splitting(&function, &stages),
            expected,
            "{name}: {} values whole against {} in the newton stage",
            function.values.len(),
            stages
                .last()
                .map(|stage| stage.function.values.len())
                .unwrap_or(0),
        );
    }
}

/// The invariant the whole split rests on: a value is at least as volatile as
/// everything it reads.
///
/// Break it and the failure lands somewhere else entirely — a stage keeps a
/// value whose defining block it dropped, and the stage arrives at validation
/// with a value nothing defines. That is what BSIM-CMG reported, and it took a
/// while to trace back to here, so it is worth checking where it is true rather
/// than where it shows.
///
/// The `raise_loops` half is what makes this non-trivial: lifting the values
/// inside a loop says nothing about the consumers outside it, so the two halves
/// have to alternate rather than run once each.
#[test]
fn a_value_is_at_least_as_volatile_as_everything_it_reads() {
    for (name, source) in fixtures() {
        let (function, _) = pipeline(source, name);
        let schedule = schedule_cfg(&function);
        for value in &function.values {
            for operand in value.kind.operands() {
                assert!(
                    schedule.class(operand) <= schedule.class(value.id),
                    "{name}: {} is {:?} but reads {operand}, which is {:?}",
                    value.id,
                    schedule.class(value.id),
                    schedule.class(operand),
                );
            }
        }
    }
}

/// Every output has to survive to the caller, and a stage's locals do not.
///
/// The interpreter hides this: it keeps each stage's whole snapshot, so an
/// output owned by the instance stage can simply be read out of it afterwards.
/// The emitter cannot — a stage is a Rust function and its bindings are gone
/// when it returns — so an output a coarse stage owns has to reach the caller
/// through the slot cache like anything else that crosses a boundary. Nothing
/// demanded it before, because a slot was only assigned when a *later stage*
/// read the value, and the caller is not a stage.
///
/// The failure this prevents is a silent zero rather than a crash, which is why
/// it is worth a test of its own.
#[test]
fn every_output_is_readable_once_the_stages_have_run() {
    for (name, source) in fixtures() {
        let (function, wanted) = pipeline(source, name);
        let schedule = schedule_cfg(&function);
        let stages = split(&function, &schedule, &wanted)
            .unwrap_or_else(|error| panic!("{name}: split: {error}"));
        let Some(last) = stages.last().map(|stage| stage.class) else {
            continue;
        };

        for (index, output) in wanted.iter().enumerate() {
            let owner = stages
                .iter()
                .find(|stage| stage.outputs[index].is_some())
                .unwrap_or_else(|| panic!("{name}: output {index} ({output}) is owned by no stage"));
            if owner.class == last {
                continue;
            }
            let held = owner.outputs[index].expect("just matched");
            assert!(
                owner.slot_of(held).is_some(),
                "{name}: output {index} ({output}) is owned by the {} stage, which returns before \
                 the caller reads it, and no slot carries it",
                owner.class.name(),
            );
        }
    }
}

/// Everything the emitter would be handed: differentiated, simplified, and with
/// the Jacobian read-outs already taken.
fn pipeline(source: &str, name: &str) -> (rspice_veriloga::canonical_ir::CfgFunction, Vec<ValueId>) {
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

    let mut wanted = cfg.residuals.clone();
    for residual in &cfg.residuals.clone() {
        wanted.extend(differentiated.derivative_row(*residual).into_iter().flatten());
    }
    optimize_cfg(&differentiated.function, &wanted)
}

fn inputs(staged: &[f64]) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameters: vec![1.0e-6, 0.4, 300.15, 250.0, 1.0e-14, 2.0],
        parameter_given: vec![false; 6],
        node_potentials: vec![0.41, 0.28, 0.15, 0.07],
        branch_flows: vec![1.0e-4; 4],
        branch_unknown_flows: vec![1.0e-4; 4],
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
        staged: staged.to_vec(),
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
        // A parameter-processing prologue, a temperature fold, and a bias-
        // dependent core — the three classes in one body.
        (
            "staged transistor",
            r#"
module staged(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#,
        ),
        // Contributions that read no unknown at all, alongside one that does.
        // Their residuals are instance-class, so the first stage computes them
        // and the caller reads them after the last — the only shape in which an
        // *output* crosses a stage boundary. Both forms are here on purpose:
        // `bias` simplifies to a bare leaf, which every stage rebuilds and none
        // computes, and `bias * bias` is an instruction that needs a slot.
        (
            "constant contribution",
            r#"
module leak(a, c);
    inout a, c;
    electrical a, c;
    parameter real g = 1.0e-3;
    parameter real bias = 5.0e-6;
    analog begin
        I(a, c) <+ g * V(a, c);
        I(a, c) <+ bias;
        I(a, c) <+ bias * bias;
    end
endmodule
"#,
        ),
        // A guard whose test is bias-dependent wrapping arithmetic that reads
        // only parameters: the case the classification must not hoist.
        (
            "parameter work under a bias guard",
            r#"
module guarded(a, c);
    inout a, c;
    electrical a, c;
    parameter real scale = 2.0;
    real term;
    analog begin
        term = 0.0;
        if (V(a, c) > 0.1) begin
            term = scale * scale * 1.0e-3;
        end
        I(a, c) <+ term * V(a, c);
    end
endmodule
"#,
        ),
        (
            "internal node",
            r#"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 250.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r;
        I(mid, n) <+ V(mid, n) / r;
    end
endmodule
"#,
        ),
        (
            "runtime loop",
            r#"
module iterated(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    integer step;
    real total;
    analog begin
        total = 0.0;
        for (step = 0; step < 3; step = step + 1) begin
            total = total + V(p, n) / r;
        end
        I(p, n) <+ total;
    end
endmodule
"#,
        ),
    ]
}
