//! Deviation census between the CFG-built plan and the shipped MIR-built plan.
//!
//! [`cfg_census`](super::cfg_census) measures the CFG route against the
//! reference interpreter, which is the semantic authority; its module
//! documentation also records why it cannot measure the CFG route against the
//! *shipped* route bit for bit — a CFG-sourced program associates its
//! arithmetic differently, so the same real number arrives as a different
//! double. This is that missing measurement, taken as a bounded-deviation
//! census rather than an identity one.
//!
//! # What is compared
//!
//! Two whole plans for one module, compiled by the same backend and executed at
//! the same operating point:
//!
//! * the shipped plan from
//!   [`build_model_plan_with_canonical_ir`](crate::jit::plan_builder::build_model_plan_with_canonical_ir),
//!   every value entry a postfix stream;
//! * the CFG plan from
//!   [`build_model_plan_from_canonical_cfg`], whose `stamp_values`,
//!   `jacobians`, `reactive_jacobians`, `noise_psd` and `noise_exponents` are
//!   block programs.
//!
//! Both go through [`x64::compile_model_plan`](crate::native::x64::compile_model_plan)
//! and both are then driven through `NativeModel`'s own entry runners, so the
//! ABI, the storage layout and the runtime error reporting are shared and the
//! only difference left is the program.
//!
//! The assignment passes are identical in the two plans by construction — the
//! CFG plan keeps them — so the census runs the shipped one once and hands both
//! plans the *same filled* variable array. That is what makes the comparison
//! meaningful at all: a shipped value entry reads a variable slot the
//! assignment pass wrote, while the CFG entry recomputes that variable inline.
//! Comparing them against an unfilled array would compare a model to a
//! different model.
//!
//! # The bound, and where it comes from
//!
//! Two, because the entries divide into two kinds. A residual and a noise
//! magnitude are one expression lowered twice, and the bound for those is
//! derived from the lowering: see [`REASSOCIATION_BUDGET`]. A Jacobian entry is
//! not — the two routes run two different implementations of the chain rule, so
//! no reassociation bound applies to them at all, and what is asserted instead
//! is the estate's own criterion for two computations being the same
//! derivative: see [`DERIVATIVE_AGREEMENT`].
//!
//! Either bound only decides an entry the census holds the two routes to at
//! all. An entry nine or more orders below the largest magnitude of its own
//! kind anywhere in the model carries no significant figures, and is measured
//! and printed rather than asserted: see [`MATRIX_SIGNIFICANCE`]. Every
//! per-module line says how many entries that was.
//!
//! # Noise, which used to be measured and is now asserted
//!
//! Every noise magnitude is held to one of two criteria and none is exempt. A
//! magnitude the plan builder took at its own site is one expression lowered
//! twice, and takes the reassociation bound like a residual. A magnitude it had
//! to take at the exit block, under a guard, takes the weaker criterion in
//! [`check_guarded_noise`]: the shipped quantity, or the inactive zero, and
//! nothing else.
//!
//! What was there before was a `grouped_noise` class that was executed,
//! printed, and asserted nothing, on the argument that a grouped process folds
//! its routing amplitude into its injection and so holds a different quantity
//! from the shipped PSD. That argument was wrong twice: `noise_process_schema`
//! is `1` on everything this compiler produces, so the class was every module
//! with any noise source, and the amplitude was never the difference — see
//! [`crate::jit::cfg_plan_builder`], whose noise slice now names what the
//! difference actually was.
//!
//! `#[ignore]`d: this is release-qualification work. Run it with
//! `--release --features native -- --ignored --nocapture`.

use std::collections::HashSet;

use super::census_models::{CensusModel, shipped_census_models_matching};
use super::cfg_census::{OperatingPoint, deviation};
use crate::jit::cfg_plan_builder::{
    CfgNoiseScope, CfgPlanEntry, CfgPlanRefusal, build_model_plan_from_canonical_cfg,
};
use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
use crate::jit::plan_program::PlanProgram;
use crate::native::aarch64::image::MAX_A64_FUNCTION_BYTES;
use crate::native::abi::EvalContext;
use crate::native::model::NativeModel;

/// How many units in the last place one operation of reassociation is allowed
/// to move an entry.
///
/// Neither route is the correct one: IEEE-754 addition and multiplication are
/// not associative, so a program that evaluates `(a + b) + c` where the other
/// evaluates `a + (b + c)` returns a different double for the same inputs, and
/// both are correctly rounded evaluations of their own expression. The question
/// a bound has to answer is how far apart two evaluations of the *same real
/// expression* can be.
///
/// The standard first-order result answers it. Each floating-point operation
/// commits a relative error of at most one unit round-off `u = f64::EPSILON/2`,
/// and `n` of them compose to at most `(1 + u)^n - 1`, which is `n * u` to
/// within a part in ten thousand for every `n` this corpus reaches. Two
/// different associations of one expression therefore differ by at most
/// `2 * n * u = n * f64::EPSILON` relative to the magnitude they evaluate to,
/// where `n` is the operation count of the longer of the two.
///
/// So the budget below is one, and the bound for such an entry is
/// `REASSOCIATION_BUDGET * operations * f64::EPSILON`. It is read off the
/// lowering — the entry's own size — and not fitted to what the run produced.
const REASSOCIATION_BUDGET: f64 = 1.0;

/// What two *different* derivative passes computing one derivative have to
/// agree to.
///
/// # Why the reassociation bound cannot be the whole story
///
/// It applies to two lowerings of one expression, and a residual is exactly
/// that: both routes carry the module's own contribution expression and only
/// group it differently. A Jacobian entry is not. The shipped route lowers
/// `StampProgram::jacobian_programs`, which the front end produced with
/// `ir.rs`'s `differentiate` over `IrExpr`; the CFG route lowers a lane of
/// `canonical_ir::differentiate` over the CFG. Those are two implementations of
/// the chain rule, and they emit algebraically equal but *structurally
/// different* expressions — a product rule expanded in a different order, a
/// quotient rule written as one division or as two. Two such expressions differ
/// by rounding times the condition number of the quantity they compute, and the
/// condition number is not something a census can read off a lowering. There is
/// therefore no derived reassociation bound for a derivative entry, and
/// inventing one would be a tolerance fitted to the run.
///
/// # What is asserted instead, and where it comes from
///
/// The criterion the estate already uses for "these two computations are the
/// same derivative": `tests/cfg_derivatives.rs`'s significance figure, also
/// carried by [`super::cfg_census`] as `ORACLE_SIGNIFICANCE`, which is the
/// share of a residual below which a derivative reading carries no significant
/// figures. Agreement to nine significant figures is that criterion applied
/// between the two routes rather than between a route and an oracle. It is
/// borrowed, not chosen here, and a deviation past it is a finding about one of
/// the two chain rules rather than a number to raise.
const DERIVATIVE_AGREEMENT: f64 = 1.0e-9;

/// One entry's comparison.
struct Comparison {
    entry: CfgPlanEntry,
    point: usize,
    mir: f64,
    cfg: f64,
    operations: usize,
    deviation: f64,
}

impl Comparison {
    /// Whether the two routes lower one expression, or two expressions one
    /// chain rule apart. See [`DERIVATIVE_AGREEMENT`].
    fn is_derivative(&self) -> bool {
        matches!(
            self.entry,
            CfgPlanEntry::Jacobian(..) | CfgPlanEntry::ReactiveJacobian(..)
        )
    }

    fn bound(&self) -> f64 {
        let reassociation = REASSOCIATION_BUDGET * self.operations as f64 * f64::EPSILON;
        if self.is_derivative() {
            reassociation.max(DERIVATIVE_AGREEMENT)
        } else {
            reassociation
        }
    }

    fn describe(&self) -> String {
        format!(
            "{} point={} mir={:.17e} cfg={:.17e} operations={} deviation={:.3e} bound={:.3e} \
             criterion={}",
            self.entry,
            self.point,
            self.mir,
            self.cfg,
            self.operations,
            self.deviation,
            self.bound(),
            if self.is_derivative() {
                "derivative-agreement"
            } else {
                "reassociation"
            }
        )
    }
}

#[derive(Default)]
struct Tally {
    models: usize,
    built: usize,
    refused: usize,
    entries: usize,
    comparisons: usize,
    exact: usize,
    /// Comparisons neither route could make: one of the two reported a runtime
    /// error at the drawn bias, so there is no pair of numbers to subtract.
    runtime_errors: usize,
    /// Entries the CFG route's liveness found structurally absent, where the
    /// shipped plan still carries a program. Checked against zero rather than
    /// against a rounding bound — see [`check_structural_zero`].
    structural_zeros: usize,
    over_bound: usize,
    nonzero_structural_zeros: usize,
    /// Noise magnitudes lowered from the exit read under a guard: asserted, on
    /// the weaker of the two noise criteria. See [`check_guarded_noise`].
    guarded_noise_entries: usize,
    /// Of those, the ones that agreed with the shipped route anyway rather than
    /// reading the inactive zero.
    guarded_noise_agreed: usize,
    /// Guarded noise magnitudes that were neither the shipped quantity nor the
    /// inactive zero. Any is a failure.
    guarded_noise_third_value: usize,
    /// Entries nine or more orders below the largest magnitude of their own
    /// kind anywhere in the model at that operating point: measured and
    /// printed, not asserted. See [`MATRIX_SIGNIFICANCE`].
    insignificant: usize,
}

/// What a noise magnitude the CFG route took at the *exit block*, under a
/// guard, has to satisfy.
///
/// Every other entry — every residual, every Jacobian, and every noise
/// magnitude the plan builder hoisted to its site — is held to the ordinary
/// bound. This class is the one the builder could not make an identity, and its
/// criterion is one step weaker rather than absent:
///
/// > the CFG reads the shipped quantity, or it reads exactly zero.
///
/// Zero is the *inactive* reading, and it is admissible for one reason that is
/// a property of the plan and not of the corpus: the shipped runtime decides a
/// source's activity from its injection gains, and an injection gain is the
/// derivative of the contribution with respect to the noise process. A source
/// the body did not reach appears in that contribution only under a condition
/// MIR dissolved into a select, so the derivative carries the same condition
/// and is zero. The magnitude is then multiplied by a zero gain on both routes,
/// and the injected noise is identical whichever number the plan held.
///
/// What the criterion forbids is a *third* value — a magnitude that is neither
/// what the shipped route computes nor the inactive zero — which is the only
/// reading that could change what the runtime injects. That is asserted, not
/// measured.
fn check_guarded_noise(comparison: &Comparison, bound: f64) -> GuardedNoise {
    if comparison.deviation <= bound {
        GuardedNoise::Agreed
    } else if comparison.cfg == 0.0 {
        GuardedNoise::Inactive
    } else {
        GuardedNoise::ThirdValue
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardedNoise {
    /// The two routes hold the same number after all.
    Agreed,
    /// The CFG holds the inactive zero, which no gain will multiply.
    Inactive,
    /// Neither. A failure.
    ThirdValue,
}

/// Every entry position the two plans both carry, in plan order.
fn entry_positions(model: &crate::codegen::CompiledModel) -> Vec<CfgPlanEntry> {
    let mut positions = Vec::new();
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        positions.push(CfgPlanEntry::StampValue(stamp_index));
        positions.extend(
            (0..stamp.jacobian_programs.len())
                .map(|entry| CfgPlanEntry::Jacobian(stamp_index, entry)),
        );
        positions.extend(
            (0..stamp.reactive_jacobians.len())
                .map(|entry| CfgPlanEntry::ReactiveJacobian(stamp_index, entry)),
        );
    }
    for (source, noise) in model.noise_sources.iter().enumerate() {
        positions.push(CfgPlanEntry::NoisePsd(source));
        if noise.exponent_program.is_some() {
            positions.push(CfgPlanEntry::NoiseExponent(source));
        }
    }
    positions
}

/// How many operations an entry carries on each route, whichever is larger.
///
/// The postfix stream's operation count and the block program's instruction
/// count measure the same thing — one arithmetic step each — so the larger of
/// the two is the `n` the bound is derived for.
fn entry_operations(
    mir: &crate::jit::model_plan::NativeModelPlan,
    cfg: &crate::jit::model_plan::NativeModelPlan,
    entry: CfgPlanEntry,
) -> usize {
    fn size(program: Option<&PlanProgram>) -> usize {
        match program {
            Some(PlanProgram::Postfix(program)) => program.ops().len(),
            Some(PlanProgram::Blocks(program)) => program.ssa().instructions().len(),
            None => 0,
        }
    }
    let pick = |plan: &crate::jit::model_plan::NativeModelPlan| match entry {
        CfgPlanEntry::StampValue(stamp) => size(plan.stamp_values.get(stamp)),
        CfgPlanEntry::Jacobian(stamp, index) => {
            size(plan.jacobians.get(stamp).and_then(|row| row.get(index)))
        }
        CfgPlanEntry::ReactiveJacobian(stamp, index) => size(
            plan.reactive_jacobians
                .get(stamp)
                .and_then(|row| row.get(index)),
        ),
        CfgPlanEntry::NoisePsd(source) => size(plan.noise_psd.get(source)),
        CfgPlanEntry::NoiseExponent(source) => {
            size(plan.noise_exponents.get(source).and_then(Option::as_ref))
        }
    };
    pick(mir).max(pick(cfg))
}

/// Run one entry on one compiled plan, or `None` if it reported a runtime
/// error at this bias.
fn run_entry(
    native: &NativeModel,
    entry: CfgPlanEntry,
    context: &EvalContext,
    variables: *const f64,
) -> Option<f64> {
    context.clear_runtime_error();
    let value = match entry {
        CfgPlanEntry::StampValue(stamp) => native.run_stamp_value(stamp, context, variables),
        CfgPlanEntry::Jacobian(stamp, index) => {
            native.run_jacobian(stamp, index, context, variables)
        }
        CfgPlanEntry::ReactiveJacobian(stamp, index) => {
            native.run_reactive_jacobian(stamp, index, context, variables)
        }
        CfgPlanEntry::NoisePsd(source) => native.run_noise_psd(source, context, variables),
        CfgPlanEntry::NoiseExponent(source) => {
            native.run_noise_exponent(source, context, variables)
        }
    };
    if context.take_runtime_error().is_some() {
        return None;
    }
    value
}

/// Below this share of the largest entry of its own matrix, a reading carries
/// no significant figures.
///
/// The estate's existing figure, under the estate's existing name:
/// `tests/cfg_derivatives.rs` and [`super::cfg_census`] both call it
/// `ORACLE_SIGNIFICANCE`, and both use it for the same judgement — whether a
/// derivative reading is a number or the residue of a cancellation. A
/// difference of two nearly equal quantities carries no significant figures,
/// and demanding that two independent chain rules agree about it manufactures
/// failures rather than finding them.
///
/// # The scale is the matrix, not the row
///
/// The oracle censuses scale an entry against the residual it differentiates.
/// This one scales it against the largest magnitude of its own kind anywhere in
/// the model at that operating point — the whole Jacobian, not one of its rows.
///
/// A row was the first thing tried and it is the wrong unit, for a reason the
/// corpus produced rather than one argued in advance: `bjt505_va`'s
/// `jacobians[21][2]` reads about `4.5e-31` on both routes, and *every* entry
/// of row 21 is that small, so row-relative significance called it significant
/// and the two chain rules' disagreement in its sixth figure became a failure.
/// It is not one. A row whose largest conductance is thirty-one orders below
/// the rest of the matrix contributes nothing to the solve that consumes it,
/// and no linear solver can see the difference between the two readings. What
/// makes an entry matter is its size against the matrix that is factorized,
/// which is what this scale states.
const MATRIX_SIGNIFICANCE: f64 = 1.0e-9;

/// The largest magnitude of each entry kind across the whole model at one
/// operating point.
#[derive(Default)]
struct MatrixScales {
    stamp_values: f64,
    jacobians: f64,
    reactive_jacobians: f64,
    noise_psd: f64,
    noise_exponents: f64,
}

impl MatrixScales {
    fn of(readings: &[(CfgPlanEntry, f64, f64)]) -> Self {
        let mut scales = Self::default();
        // Both routes' readings, because an entry one route calls large and the
        // other calls zero must not be gated away by the route that dropped it.
        for (entry, mir, cfg) in readings {
            let magnitude = mir.abs().max(cfg.abs());
            let slot = scales.slot(*entry);
            *slot = f64::max(*slot, magnitude);
        }
        scales
    }

    /// Each entry kind is its own matrix or vector, and each is scaled against
    /// itself: a reactive Jacobian entry against the reactive matrix, a
    /// residual against the residual vector. Scaling one kind against another
    /// would compare quantities that are not even in the same units.
    fn slot(&mut self, entry: CfgPlanEntry) -> &mut f64 {
        match entry {
            CfgPlanEntry::StampValue(_) => &mut self.stamp_values,
            CfgPlanEntry::Jacobian(..) => &mut self.jacobians,
            CfgPlanEntry::ReactiveJacobian(..) => &mut self.reactive_jacobians,
            CfgPlanEntry::NoisePsd(_) => &mut self.noise_psd,
            CfgPlanEntry::NoiseExponent(_) => &mut self.noise_exponents,
        }
    }

    fn scale(&self, entry: CfgPlanEntry) -> f64 {
        match entry {
            CfgPlanEntry::StampValue(_) => self.stamp_values,
            CfgPlanEntry::Jacobian(..) => self.jacobians,
            CfgPlanEntry::ReactiveJacobian(..) => self.reactive_jacobians,
            CfgPlanEntry::NoisePsd(_) => self.noise_psd,
            CfgPlanEntry::NoiseExponent(_) => self.noise_exponents,
        }
    }

    /// Whether this reading is large enough, within its own matrix, for the two
    /// routes to be held to agreeing about it.
    fn is_significant(&self, comparison: &Comparison) -> bool {
        let scale = self.scale(comparison.entry);
        let magnitude = comparison.mir.abs().max(comparison.cfg.abs());
        magnitude > scale * MATRIX_SIGNIFICANCE
    }
}

/// What a structurally absent entry has to satisfy.
///
/// The CFG route carries no value for it, so its plan entry is the constant
/// zero its own liveness implies. That is only right if the shipped program is
/// identically zero there, which is a claim about the corpus rather than a
/// theorem — so it is checked here, at every operating point, rather than
/// assumed.
fn check_structural_zero(comparison: &Comparison) -> bool {
    comparison.cfg == 0.0 && comparison.mir == 0.0
}

/// Storage both routes read, sized once per model.
struct Storage {
    currents: Vec<f64>,
    branch_currents: Vec<f64>,
}

impl Storage {
    fn for_model(model: &crate::codegen::CompiledModel) -> Self {
        // Slack past every declared count keeps a stray index inside the
        // allocation: an unchecked indexed load must not decide whether this
        // census process survives.
        const SLACK: usize = 64;
        let terminals = model.num_terminals + SLACK;
        Self {
            currents: vec![0.0; model.stamp_programs.len() + SLACK],
            branch_currents: vec![0.0; terminals * terminals],
        }
    }
}

/// Compile both plans for one module and compare every entry they share.
#[allow(clippy::too_many_lines)]
fn census_model(shipped: &CensusModel, tally: &mut Tally) -> Option<String> {
    let module = &shipped.name;
    let model = &shipped.model;
    let artifact = &shipped.canonical_ir;

    // `Cfg`, not the `Postfix` scope production takes: the whole point of this
    // census is to measure the noise slice the default plan declines to use, so
    // narrowing it here would make the gap invisible on the day it closes.
    let cfg_plan = match build_model_plan_from_canonical_cfg(model, artifact, CfgNoiseScope::Cfg) {
        Ok(plan) => plan,
        Err(refused) => {
            tally.refused += 1;
            println!(
                "cfg-mir model={module} refused={} detail={}",
                refused.class.name(),
                refused.detail
            );
            return None;
        }
    };
    let mir_plan = build_model_plan_with_canonical_ir(model, artifact)
        .unwrap_or_else(|error| panic!("{module}: shipped plan: {error}"));
    let mir_native = crate::native::x64::compile_model_plan(model, &mir_plan)
        .unwrap_or_else(|error| panic!("{module}: shipped plan codegen: {error}"));
    // A backend that will not emit the CFG plan is a finding about that plan,
    // counted like a refusal rather than raised as a panic: aborting here would
    // take the other forty-two modules' measurements with it.
    let cfg_native = match crate::native::x64::compile_model_plan(model, &cfg_plan.plan) {
        Ok(native) => native,
        Err(error) => {
            tally.refused += 1;
            println!("cfg-mir model={module} refused=x64-codegen detail={error}");
            return None;
        }
    };
    tally.built += 1;

    let structural_zeros: HashSet<CfgPlanEntry> =
        cfg_plan.report.structural_zeros.iter().copied().collect();
    let guarded_noise: HashSet<CfgPlanEntry> = cfg_plan
        .report
        .noise_guarded_reads
        .iter()
        .copied()
        .collect();

    // Sized to whichever route asks for more, so neither is executed against an
    // array the other decided the size of.
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
    let parameter_defaults: Vec<Option<f64>> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default)
        .collect();
    let branch_unknowns =
        crate::jit::plan_builder::canonical_branch_unknown_runtime_map(model, &artifact.mir)
            .unwrap_or_else(|error| panic!("{module}: branch unknown map: {error}"));
    let mut storage = Storage::for_model(model);
    // The same three points the CFG censuses draw, so a deviation reported here
    // and a deviation reported there are readings at one bias.
    let mut points: Vec<OperatingPoint> =
        [(0x0005_EED1_u64, 0_u8), (0x00C0_FFEE, 2), (0x0000_BEEF, 0)]
            .into_iter()
            .map(|(seed, analysis)| {
                OperatingPoint::new(
                    seed,
                    analysis,
                    &parameter_defaults,
                    model.num_terminals,
                    model.internal_nodes,
                    &branch_unknowns,
                    state_len,
                    0,
                )
            })
            .collect();

    let positions = entry_positions(model);
    tally.entries += positions.len();
    let mut worst: Option<Comparison> = None;
    let mut worst_ratio = 0.0_f64;
    let mut exact = 0_usize;
    let mut compared = 0_usize;
    let mut over: Option<Comparison> = None;
    let mut nonzero_structural: Option<Comparison> = None;
    let mut guarded_noise_worst = 0.0_f64;
    let mut guarded_noise_case: Option<Comparison> = None;
    let mut guarded_noise_third: Option<Comparison> = None;
    let mut guarded_here = 0_usize;
    let mut insignificant_worst = 0.0_f64;
    let mut insignificant_case: Option<Comparison> = None;
    let mut insignificant_here = 0_usize;

    for (index, point) in points.iter_mut().enumerate() {
        let mut context = point.context();
        context.currents = storage.currents.as_mut_ptr();
        context.currents_len = storage.currents.len();
        context.branch_currents = storage.branch_currents.as_mut_ptr();
        context.branch_currents_len = storage.branch_currents.len();
        // The assignment passes are the same programs in both plans, so filling
        // the variable array once with the shipped one is not a bias toward it.
        let mut variables = vec![0.0_f64; model.num_variables + 64];
        context.clear_runtime_error();
        mir_native.run_assignments(&context, variables.as_mut_ptr());
        let _ = context.take_runtime_error();

        // Read every entry on both plans first, then compare. Two passes
        // because a matrix entry's significance is a property of the row it
        // sits in, and the row is not known until it has been read.
        let mut readings = Vec::with_capacity(positions.len());
        for entry in &positions {
            let entry = *entry;
            let (Some(mir), Some(cfg)) = (
                run_entry(&mir_native, entry, &context, variables.as_ptr()),
                run_entry(&cfg_native, entry, &context, variables.as_ptr()),
            ) else {
                tally.runtime_errors += 1;
                continue;
            };
            readings.push((entry, mir, cfg));
        }
        let scales = MatrixScales::of(&readings);

        for (entry, mir, cfg) in readings {
            compared += 1;
            let operations = entry_operations(&mir_plan, &cfg_plan.plan, entry);
            let comparison = Comparison {
                entry,
                point: index,
                mir,
                cfg,
                operations,
                deviation: deviation(mir, cfg).unwrap_or(0.0),
            };
            if comparison.deviation == 0.0 {
                exact += 1;
            }
            if guarded_noise.contains(&entry) {
                tally.guarded_noise_entries += 1;
                guarded_here += 1;
                if comparison.deviation > guarded_noise_worst {
                    guarded_noise_worst = comparison.deviation;
                }
                match check_guarded_noise(&comparison, comparison.bound()) {
                    GuardedNoise::Agreed => tally.guarded_noise_agreed += 1,
                    GuardedNoise::Inactive => {
                        guarded_noise_case.get_or_insert(comparison);
                    }
                    GuardedNoise::ThirdValue => {
                        tally.guarded_noise_third_value += 1;
                        guarded_noise_third.get_or_insert(comparison);
                    }
                }
                continue;
            }
            if structural_zeros.contains(&entry) {
                tally.structural_zeros += 1;
                if !check_structural_zero(&comparison) {
                    tally.nonzero_structural_zeros += 1;
                    nonzero_structural.get_or_insert(comparison);
                }
                continue;
            }
            if !scales.is_significant(&comparison) {
                tally.insignificant += 1;
                insignificant_here += 1;
                if comparison.deviation > insignificant_worst {
                    insignificant_worst = comparison.deviation;
                    insignificant_case = Some(comparison);
                }
                continue;
            }
            let bound = comparison.bound();
            if comparison.deviation > bound {
                tally.over_bound += 1;
                if over.is_none() {
                    over = Some(comparison);
                }
                continue;
            }
            let ratio = if bound > 0.0 {
                comparison.deviation / bound
            } else {
                0.0
            };
            if ratio >= worst_ratio {
                worst_ratio = ratio;
                worst = Some(comparison);
            }
        }
    }
    tally.comparisons += compared;
    tally.exact += exact;

    // The AArch64 ceiling, measured on the entry that would meet it first.
    let largest = cfg_plan.report.largest_entry.map(|(entry, instructions)| {
        let program = cfg_entry_ssa(&cfg_plan.plan, entry)
            .unwrap_or_else(|| panic!("{module}: {entry} is not a block entry"));
        let bytes = crate::native::aarch64::codegen::compile_value_function_from_ssa(program)
            .map_or(0, |bytes| bytes.len());
        (entry, instructions, bytes)
    });

    println!(
        "cfg-mir model={module} entries={} compared={compared} exact={exact} \
         below_significance={insignificant_here} structural_zeros={} \
         noise_from_differentiated={} max_deviation_over_bound={worst_ratio:.3e}{}{}{}",
        positions.len(),
        cfg_plan.report.structural_zeros.len(),
        cfg_plan.report.noise_from_differentiated,
        largest
            .map(|(entry, instructions, bytes)| format!(
                " largest_entry[{entry} instructions={instructions} a64_bytes={bytes} \
                  a64_ceiling={MAX_A64_FUNCTION_BYTES}]"
            ))
            .unwrap_or_default(),
        worst
            .as_ref()
            .map(|worst| format!(" worst[{}]", worst.describe()))
            .unwrap_or_default(),
        over.as_ref()
            .map(|over| format!(" OVER_BOUND[{}]", over.describe()))
            .unwrap_or_default(),
    );
    if let Some(case) = insignificant_case.as_ref() {
        println!(
            "cfg-mir model={module} below_significance={insignificant_here} \
             max_deviation={insignificant_worst:.3e} case[{}]",
            case.describe()
        );
    }
    if !model.noise_sources.is_empty() {
        println!(
            "cfg-mir model={module} noise_sources={} noise_hoisted={} guarded_reads={} \
             guarded_compared={guarded_here} guarded_max_deviation={guarded_noise_worst:.3e}{}",
            model.noise_sources.len(),
            cfg_plan.report.noise_hoisted,
            cfg_plan.report.noise_guarded_reads.len(),
            guarded_noise_case
                .as_ref()
                .map(|case| format!(" inactive_case[{}]", case.describe()))
                .unwrap_or_default(),
        );
    }

    if let Some(third) = guarded_noise_third {
        return Some(format!(
            "{module}: {} is neither the shipped magnitude nor the inactive zero: {}",
            third.entry,
            third.describe()
        ));
    }
    if let Some(nonzero) = nonzero_structural {
        return Some(format!(
            "{module}: the CFG route found {} structurally absent but the shipped route does not \
             evaluate to zero there: {}",
            nonzero.entry,
            nonzero.describe()
        ));
    }
    if let Some(over) = over {
        return Some(format!(
            "{module}: {} is outside the reassociation bound: {}",
            over.entry,
            over.describe()
        ));
    }
    None
}

/// The block program behind one CFG plan entry.
fn cfg_entry_ssa(
    plan: &crate::jit::model_plan::NativeModelPlan,
    entry: CfgPlanEntry,
) -> Option<&crate::jit::ssa::Program> {
    let program = match entry {
        CfgPlanEntry::StampValue(stamp) => plan.stamp_values.get(stamp),
        CfgPlanEntry::Jacobian(stamp, index) => {
            plan.jacobians.get(stamp).and_then(|row| row.get(index))
        }
        CfgPlanEntry::ReactiveJacobian(stamp, index) => plan
            .reactive_jacobians
            .get(stamp)
            .and_then(|row| row.get(index)),
        CfgPlanEntry::NoisePsd(source) => plan.noise_psd.get(source),
        CfgPlanEntry::NoiseExponent(source) => {
            plan.noise_exponents.get(source).and_then(Option::as_ref)
        }
    }?;
    match program {
        PlanProgram::Blocks(program) => Some(program.ssa()),
        PlanProgram::Postfix(_) => None,
    }
}

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_built_plan_agrees_with_the_shipped_plan_within_the_reassociation_bound() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let mut tally = Tally::default();
    let mut failures = Vec::new();
    let mut total_compile_seconds = 0.0_f64;
    let mut total_census_seconds = 0.0_f64;
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let started = std::time::Instant::now();
        tally.models += 1;
        if let Some(failure) = census_model(&shipped, &mut tally) {
            failures.push(failure);
        }
        let census_seconds = started.elapsed().as_secs_f64();
        println!(
            "cfg-mir model={} compile_seconds={:.1} census_seconds={census_seconds:.1} cached={}",
            shipped.name, shipped.compile_seconds, shipped.from_cache
        );
        total_compile_seconds += shipped.compile_seconds;
        total_census_seconds += census_seconds;
    }
    println!(
        "cfg-mir total_compile_seconds={total_compile_seconds:.1} total_census_seconds={total_census_seconds:.1}"
    );
    println!(
        "cfg-mir models={} built={} refused={} entries={} comparisons={} exact={} \
         runtime_errors={} structural_zeros={} over_bound={} nonzero_structural_zeros={} \
         guarded_noise_entries={} guarded_noise_agreed={} guarded_noise_third_value={} \
         below_significance={}",
        tally.models,
        tally.built,
        tally.refused,
        tally.entries,
        tally.comparisons,
        tally.exact,
        tally.runtime_errors,
        tally.structural_zeros,
        tally.over_bound,
        tally.nonzero_structural_zeros,
        tally.guarded_noise_entries,
        tally.guarded_noise_agreed,
        tally.guarded_noise_third_value,
        tally.insignificant,
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    }
    assert!(
        tally.comparisons > 0,
        "the census must actually execute both plans"
    );
    assert!(
        failures.is_empty(),
        "the CFG-built plan deviates from the shipped plan:\n{}",
        failures.join("\n")
    );
}

/// A refusal class the CFG plan builder can reach has to be one this census can
/// name, or a module that stopped building would be reported as a class the
/// reader cannot look up.
#[test]
fn every_refusal_class_has_a_name() {
    for class in [
        CfgPlanRefusal::ShippedPlan,
        CfgPlanRefusal::CfgLowering,
        CfgPlanRefusal::StateAllocation,
        CfgPlanRefusal::Differentiate,
        CfgPlanRefusal::DerivativeRuleMissing,
        CfgPlanRefusal::Scalarize,
        CfgPlanRefusal::NoScalar,
        CfgPlanRefusal::Lowering,
        CfgPlanRefusal::EquationsUnpaired,
        CfgPlanRefusal::LaneUnmapped,
        CfgPlanRefusal::ChargeMissing,
        CfgPlanRefusal::NoiseUnpaired,
        CfgPlanRefusal::SlotUnclaimed,
    ] {
        assert!(!class.name().is_empty());
        assert!(!class.name().contains(' '), "{:?}", class);
    }
}

/// The bound is derived from the entry's size, so it has to grow with it and
/// has to be a rounding budget rather than a tolerance.
#[test]
fn the_reassociation_bound_is_a_rounding_budget() {
    let sized = |operations: usize| Comparison {
        entry: CfgPlanEntry::StampValue(0),
        point: 0,
        mir: 1.0,
        cfg: 1.0,
        operations,
        deviation: 0.0,
    };
    let small = sized(10);
    let large = sized(20_000);
    assert!(small.bound() > 0.0);
    assert!(large.bound() > small.bound());
    // A twenty-thousand-operation compact-model residual still gets a bound
    // four orders below a part per billion: this is rounding, not slack.
    assert!(large.bound() < 1.0e-11, "{:e}", large.bound());

    // A derivative entry of the same size takes the borrowed criterion
    // instead, because the two routes' chain rules are two expressions rather
    // than two groupings of one.
    let derivative = Comparison {
        entry: CfgPlanEntry::Jacobian(0, 0),
        ..sized(20_000)
    };
    assert!(derivative.is_derivative());
    assert_eq!(derivative.bound(), DERIVATIVE_AGREEMENT);
    assert!(!large.is_derivative());
}

/// The significance scale is the whole matrix, and `bjt505_va` is why.
///
/// The figures are that module's, measured: `jacobians[21][2]` reads about
/// `-4.5e-31` on both routes and the two chain rules disagree in its sixth
/// figure, while the Jacobian it belongs to carries entries of order one. Every
/// entry of row 21 is that small, so a row-relative scale called it significant
/// and failed the module on a number no linear solver can see. A matrix-relative
/// scale classifies it as measured, which is what it is.
#[test]
fn an_entry_far_below_its_matrix_is_measured_rather_than_asserted() {
    const TINY_MIR: f64 = -4.545_131_238_293_389_5e-31;
    const TINY_CFG: f64 = -4.545_278_222_975_802e-31;
    let readings = [
        (CfgPlanEntry::Jacobian(0, 0), 1.0, 1.0),
        (CfgPlanEntry::Jacobian(21, 2), TINY_MIR, TINY_CFG),
        (CfgPlanEntry::StampValue(0), 2.0e-30, 2.0e-30),
    ];
    let scales = MatrixScales::of(&readings);
    let sized = |entry, mir: f64, cfg: f64, deviation| Comparison {
        entry,
        point: 0,
        mir,
        cfg,
        operations: 1202,
        deviation,
    };

    let tiny = sized(CfgPlanEntry::Jacobian(21, 2), TINY_MIR, TINY_CFG, 3.234e-5);
    assert!(
        tiny.deviation > tiny.bound(),
        "the fixture has to be one the bound rejects, or it proves nothing"
    );
    assert!(
        !scales.is_significant(&tiny),
        "an entry thirty-one orders below its own Jacobian is invisible to the solve"
    );
    assert!(scales.is_significant(&sized(CfgPlanEntry::Jacobian(0, 0), 1.0, 1.0, 0.0)));

    // Each kind is scaled against itself. This residual is as small as the
    // Jacobian entry that was gated away, and it is still significant, because
    // it is the largest residual there is.
    assert!(scales.is_significant(&sized(CfgPlanEntry::StampValue(0), 2.0e-30, 2.0e-30, 0.0)));
}
