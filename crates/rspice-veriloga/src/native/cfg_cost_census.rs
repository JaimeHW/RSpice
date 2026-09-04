//! What one evaluation costs on each plan route, measured side by side.
//!
//! [`super::cfg_size_census`] answers what the CFG route costs in image bytes
//! and [`super::cfg_mir_census`] answers whether it computes the same numbers.
//! Neither answers the question a user judges the flip by: how long a device
//! takes to evaluate. The prelude was introduced to stop the union cone from
//! being inlined once per entry, so it is a *size* argument that has to be
//! checked for a *time* regression — one prelude that computes every distinct
//! value once should cost less than a few thousand overlapping cones, but
//! "should" is not a measurement.
//!
//! # Three plans, one process, interleaved samples
//!
//! `postfix` is the shipped plan. `cfg` is the plan production builds after the
//! flip — the CFG route with [`CfgNoiseScope::Cfg`]. `cfg-mir-noise` is the CFG
//! route with the noise magnitudes left postfix, which is the plan
//! [`super::cfg_size_census`] measures. The third exists because the two
//! switches are separable in principle even though the flip moves them
//! together: with all three timed, a regression can be attributed to the route
//! or to the noise scope instead of only observed.
//!
//! All three are built from the same front-end artifact, compiled by the same
//! x64 backend, and run against the same operating point, the same variable
//! array and the same storage, with each timed sample taken from one plan and
//! then the next. A before-and-after pair of runs across a code change would
//! have measured the box's thermal state and its peer builds as much as the
//! plans; this measures one difference.
//!
//! # What an evaluation is here
//!
//! The assignment pass, the prelude when the plan has one, and every residual,
//! Jacobian, reactive-Jacobian and noise entry the model declares — read
//! through the per-entry entry points rather than through the fused stamp
//! kernel. Production calls the fused kernel where the image is eligible for
//! one, and that kernel inlines exactly these entries, so the sum over entries
//! is the same arithmetic plus one indirect call each. That per-entry overhead
//! is paid identically by every plan and is a few nanoseconds against
//! evaluations measured in microseconds; what it cannot do is favour one plan,
//! which is what the ratios are read for.
//!
//! `prelude=` and `prelude_slots=` are printed per plan so a ratio can be read
//! against whether the plan under it published a prelude at all: a CFG plan
//! that dropped to per-entry cones is a different measurement from one that did
//! not, and the number alone cannot tell them apart.
//!
//! Narrow it with `RSPICE_CFG_CENSUS_FILTER`, as every other census here.

use std::time::Instant;

use super::census_models::shipped_census_models_matching;
use super::cfg_census::OperatingPoint;
use super::cfg_mir_census::{Storage, entry_positions, run_entry};
use crate::jit::cfg_plan_builder::{CfgNoiseScope, build_model_plan_from_canonical_cfg};
use crate::jit::model_plan::NativeModelPlan;
use crate::jit::plan_builder::{
    build_model_plan_with_canonical_ir, canonical_branch_unknown_runtime_map,
};
use crate::native::NativeModel;
use crate::native::abi::EvalContext;

/// The five modules the cost of the flip is reported on.
///
/// One of each shape the estate has: a small ESD clamp, a bipolar model, a
/// HICUM with deep charge storage, and the two whose images the size census
/// measured growing the most (`bsimcmg_va` 4.80 MB to 5.62, `asmhemt` 16.66 to
/// 17.29). If the prelude costs time anywhere it is on the last two.
const COST_MODELS: [&str; 5] = ["asmesd", "vbic13_4t", "hicumL2va", "bsimcmg_va", "asmhemt"];

/// Timed samples per plan. The reported figure is the median of these.
const SAMPLES: usize = 7;

/// Evaluations per timed sample, so one sample is long enough that the clock's
/// resolution is not what is being measured.
const EVALUATIONS_PER_SAMPLE: usize = 8;

/// The operating point every model is measured at: the first of the agreement
/// census's three, so a cost reading and an accuracy reading stand at the same
/// bias.
const COST_POINT: (u64, u8) = (0x0005_EED1, 0);

/// One compiled plan under measurement.
struct Measured {
    label: &'static str,
    native: NativeModel,
    samples: Vec<f64>,
    /// Summed over every *finite* entry reading of every evaluation, printed so
    /// a run where one plan evaluated something else says so. Non-finite
    /// readings are counted instead of summed: a compact model at a synthetic
    /// bias overflows entries on both routes, and one `inf` would otherwise
    /// swallow the whole witness.
    checksum: f64,
    nonfinite: usize,
    /// Whether the image published a prelude, and how many slots it publishes
    /// into.
    prelude: bool,
    prelude_slots: usize,
}

fn median(samples: &[f64]) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}

#[test]
#[ignore = "measurement; run with --release --features native --lib cfg_cost -- --ignored --nocapture"]
fn the_plan_routes_cost_the_same_per_evaluation() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let mut measured = 0_usize;
    for name in COST_MODELS {
        if filter
            .as_deref()
            .is_some_and(|filter| !name.contains(filter))
        {
            continue;
        }
        let shipped = shipped_census_models_matching(Some(name))
            .find(|shipped| shipped.name == name)
            .unwrap_or_else(|| panic!("the shipped model tree no longer declares {name}"));
        let module = &shipped.name;
        let model = &shipped.model;
        let artifact = &shipped.canonical_ir;

        let plans: [(&'static str, NativeModelPlan); 3] = [
            (
                "postfix",
                build_model_plan_with_canonical_ir(model, artifact)
                    .unwrap_or_else(|error| panic!("{module}: shipped plan: {error}")),
            ),
            (
                "cfg-mir-noise",
                build_model_plan_from_canonical_cfg(model, artifact, CfgNoiseScope::Postfix)
                    .unwrap_or_else(|refused| panic!("{module}: CFG plan, MIR noise: {refused}"))
                    .plan,
            ),
            (
                "cfg",
                build_model_plan_from_canonical_cfg(model, artifact, CfgNoiseScope::Cfg)
                    .unwrap_or_else(|refused| panic!("{module}: CFG plan: {refused}"))
                    .plan,
            ),
        ];

        let mut plans: Vec<Measured> = plans
            .into_iter()
            .map(|(label, plan)| {
                let native = crate::native::x64::compile_model_plan(model, &plan)
                    .unwrap_or_else(|error| panic!("{module}: {label} plan codegen: {error}"));
                let prelude_slots = native.required_storage().prelude_slots;
                Measured {
                    label,
                    prelude: plan.prelude.is_some(),
                    prelude_slots,
                    native,
                    samples: Vec::with_capacity(SAMPLES),
                    checksum: 0.0,
                    nonfinite: 0,
                }
            })
            .collect();

        let state_len = plans
            .iter()
            .map(|plan| {
                let storage = plan.native.required_storage();
                storage
                    .state_values
                    .max(storage.state_initialized)
                    .max(storage.state_candidate_valid)
            })
            .max()
            .unwrap_or(0);
        let prelude_slots = plans
            .iter()
            .map(|plan| plan.prelude_slots)
            .max()
            .unwrap_or(0);
        let parameter_defaults: Vec<Option<f64>> = artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default)
            .collect();
        let branch_unknowns = canonical_branch_unknown_runtime_map(model, &artifact.mir)
            .unwrap_or_else(|error| panic!("{module}: branch unknown map: {error}"));
        let mut storage = Storage::for_model(model);
        let mut point = OperatingPoint::new(
            COST_POINT.0,
            COST_POINT.1,
            &parameter_defaults,
            model.num_terminals,
            model.internal_nodes,
            &branch_unknowns,
            state_len,
            0,
        )
        .with_initial_step()
        // Sized for the widest plan and handed to all of them: the shipped plan
        // reads no slot, and a plan that ran without them would write through a
        // null pointer rather than get a slow number.
        .with_prelude_slots(prelude_slots);
        let mut context = point.context();
        context.currents = storage.currents.as_mut_ptr();
        context.currents_len = storage.currents.len();
        context.branch_currents = storage.branch_currents.as_mut_ptr();
        context.branch_currents_len = storage.branch_currents.len();
        let mut variables = vec![0.0_f64; model.num_variables + 64];
        let positions = entry_positions(model);

        // One untimed evaluation of each first: the first call through a freshly
        // published image pays its instruction-cache misses, which is a setup
        // cost and not what a Newton iteration pays.
        for plan in &mut plans {
            evaluate(plan, &context, &mut variables, &positions);
        }
        for plan in &mut plans {
            plan.checksum = 0.0;
            plan.nonfinite = 0;
        }
        for _ in 0..SAMPLES {
            for plan in &mut plans {
                let started = Instant::now();
                for _ in 0..EVALUATIONS_PER_SAMPLE {
                    evaluate(plan, &context, &mut variables, &positions);
                }
                let sample = started.elapsed().as_nanos() as f64 / EVALUATIONS_PER_SAMPLE as f64;
                plan.samples.push(sample);
            }
        }

        let reference = median(&plans[0].samples);
        for plan in &plans {
            let ns = median(&plan.samples);
            let best = plan.samples.iter().copied().fold(f64::INFINITY, f64::min);
            println!(
                "cfg-cost model={module} plan={} entries={} ns_per_evaluation={ns:.0} \
                 best_ns={best:.0} ratio_to_postfix={:.3} bytes={} prelude={} \
                 prelude_slots={} checksum={:.6e} nonfinite={}",
                plan.label,
                positions.len(),
                ns / reference,
                plan.native.image_bytes().len(),
                plan.prelude,
                plan.prelude_slots,
                plan.checksum,
                plan.nonfinite,
            );
        }
        measured += 1;
    }
    println!("cfg-cost models={measured}");
    assert!(
        measured > 0,
        "no model was measured; RSPICE_CFG_CENSUS_FILTER matched none of {COST_MODELS:?}"
    );
}

/// One evaluation of one plan: the assignment pass, the prelude if the plan has
/// one, then every entry.
fn evaluate(
    plan: &mut Measured,
    context: &EvalContext,
    variables: &mut [f64],
    positions: &[crate::jit::cfg_plan_builder::CfgPlanEntry],
) {
    context.clear_runtime_error();
    plan.native.run_assignments(context, variables.as_mut_ptr());
    plan.native.run_prelude(context, variables.as_ptr());
    let _ = context.take_runtime_error();
    for entry in positions {
        match run_entry(&plan.native, *entry, context, variables.as_ptr()) {
            Some(value) if value.is_finite() => plan.checksum += value,
            _ => plan.nonfinite += 1,
        }
    }
}
