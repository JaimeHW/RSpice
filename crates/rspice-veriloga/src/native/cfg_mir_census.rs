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
//! # Where the derivative criterion is asked
//!
//! Not in `f64`. Both cones are walked a second time in
//! [`DoubleDouble`](super::double_double::DoubleDouble) — the CFG one by the
//! reference interpreter, the shipped one by
//! [`PostfixWalk`](super::mir_postfix::PostfixWalk) — and the agreement
//! criterion is asked of *those*, where a condition number of `5e7` still
//! leaves twenty-four digits and the arithmetic is no longer part of the
//! answer. Each route's `f64` rounding error is then a *measurement*,
//! `|value_f64 − value_dd|`, reported per entry and per model. See
//! [`Comparison`].
//!
//! Two verdicts come out of that and they are different findings. The agreement
//! criterion asks whether the two chain rules compute one real derivative;
//! [`Comparison::lost_significance`] asks whether either route's `f64`
//! evaluation of it has a correct digit left. A route can fail the second while
//! passing the first — the `max` defect this estate actually had did exactly
//! that — so both are asserted and the message says which.
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

use std::collections::{BTreeSet, HashMap, HashSet};

use super::census_models::{CensusModel, shipped_census_models_matching};
use super::cfg_census::{OperatingPoint, deviation};
use super::double_double::{DoubleDouble, lift_inputs};
use super::mir_postfix::PostfixWalk;
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{CfgScalar, ValueId, differentiate, evaluate_cfg, prune_cfg_to_outputs};
use crate::jit::cfg_lanes::scalarize_lanes;
use crate::jit::cfg_plan_builder::{
    CfgNoiseScope, CfgPlanEntry, CfgPlanRefusal, ShippedColumnLanes,
    build_model_plan_from_canonical_cfg, derivative_seeds,
};
use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
use crate::jit::plan_program::PlanProgram;
use crate::native::aarch64::image::MAX_A64_FUNCTION_BYTES;
use crate::native::abi::EvalContext;
use crate::native::model::NativeModel;
use crate::rust_backend::canonical::stored_charges;

/// The three operating points, drawn once and used by both passes.
///
/// The error lane and the two compiled plans have to stand at the *same* bias
/// or the bound measures a different computation from the one it is a floor
/// under. `OperatingPoint::new` is a pure function of these two numbers and the
/// module's shape, so two constructions from the same pair are the same point;
/// naming the pairs once is what makes that checkable rather than coincidental.
/// They are also the points `cfg_census` draws, so a deviation reported here
/// and one reported there are readings at one bias.
const CENSUS_POINTS: [(u64, u8); 3] = [(0x0005_EED1, 0), (0x00C0_FFEE, 2), (0x0000_BEEF, 0)];

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

/// The unit round-off, `u = f64::EPSILON / 2`.
///
/// Used only as the denominator's floor in the noise ratio the census reports,
/// so an entry both routes evaluate exactly does not divide by zero.
const UNIT_ROUNDOFF: f64 = f64::EPSILON / 2.0;

/// Where the criterion is asked, and why `f64` is the wrong place to ask it.
///
/// [`DERIVATIVE_AGREEMENT`] is a figure about *significance*, and it says
/// nothing about whether `f64` can reach it. A cone that forms a factor at
/// `3e7` and lands it at `5e-9` has a condition number near `5e7`; one unit
/// round-off amplified by that is `5.5e-9`, so no `f64` evaluation of that cone
/// — neither route's, nor a third one — carries nine figures, and two correct
/// evaluations of it differ in the eighth. `l_utsoi_102`'s `jacobians[36][6]`
/// is exactly that entry, and W-F10c established that neither route is wrong
/// there: the complex-step oracle sits *outside* both, collinear with them.
///
/// Holding such an entry to `1e-9` in `f64` fails a correct computation for
/// being evaluated in floating point, and widening the constant to admit it
/// would be a tolerance fitted to that run. W-F10c-2 tried a third way — a
/// first-order forward rounding *bound*, used as a floor under the criterion —
/// and it failed for a reason worth keeping written down: a scalar bound adds
/// the magnitudes of two operands' errors, while the subtraction that follows
/// *cancels* them, so on the one cone that needed it the bound came out at
/// four times the entry itself and decided nothing.
///
/// # What is asked instead
///
/// Both cones are evaluated a second time in [`DoubleDouble`], and the
/// criterion is asked there:
///
/// > `|mir_dd − cfg_dd| ≤ DERIVATIVE_AGREEMENT · |entry|`
///
/// which is [`DERIVATIVE_AGREEMENT`] with nothing added to it. That is not a
/// weaker question than the `f64` one but a *different* one, and the right one:
/// two implementations of the chain rule either compute the same real quantity
/// or they do not, and at `u_dd ≈ 6e-33` a condition number of `5e7` still
/// leaves twenty-four digits, so the arithmetic is no longer part of the
/// answer. A mis-differentiated entry is orders outside this criterion exactly
/// as it was outside the `f64` one — see
/// [`the_criterion_rejects_a_mis_differentiated_max`].
///
/// The `f64` figures do not disappear; they stop being the criterion and become
/// the *measurement*. Each entry reports `E_mir = |mir_f64 − mir_dd|` and
/// `E_cfg = |cfg_f64 − cfg_dd|`, each route's actual rounding error at that
/// entry, and their ratio. Those are reported per entry and per model and
/// asserted nowhere: a CFG route noisier than the shipped route at some entry
/// is a finding for the lane that flips the switch, not a failure of this
/// census.
///
/// # Where the reference is unavailable
///
/// A `DoubleDouble` reference needs *both* walks. The CFG side needs a cone the
/// reference interpreter can walk (see [`entry_reference`]) and the shipped
/// side needs a postfix stream [`PostfixWalk`] implements every operation of.
/// Where either is missing the entry falls back to the `f64` deviation against
/// [`DERIVATIVE_AGREEMENT`] — the criterion this census carried before any of
/// this — and is counted as [`Tally::reference_missing`] so the class cannot
/// grow unseen.
#[derive(Clone, Copy)]
struct Comparison {
    entry: CfgPlanEntry,
    point: usize,
    mir: f64,
    cfg: f64,
    operations: usize,
    deviation: f64,
    /// The shipped route's cone re-walked in double-double.
    mir_dd: Option<DoubleDouble>,
    /// The CFG route's cone re-walked in double-double.
    cfg_dd: Option<DoubleDouble>,
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

    /// Both routes' cones re-walked in double-double, where both walks exist
    /// *and both reached the reals*.
    ///
    /// # Why a non-finite reference is no reference
    ///
    /// A wider precision resolves a cancellation the narrow one rounded away,
    /// and where the cancelled quantity feeds a square root or a logarithm that
    /// changes the answer from a number to a NaN: `sqrt(a − b)` with `a` and `b`
    /// equal to the last bit of `f64` is `sqrt(+0)` there and `sqrt(−1e-40)`
    /// here. `vbic13_3t_et` does that at one of the three biases, in 186 of its
    /// entries.
    ///
    /// Neither walk is wrong. What is true is that the two are no longer
    /// evaluating the same branch of the same function, so the reference does
    /// not answer the question it was computed for, and the entry falls back to
    /// the `f64` criterion. It is counted as [`Tally::reference_not_finite`]
    /// rather than folded into the missing ones, because the two have different
    /// causes and only this one is a statement about the model.
    fn references(&self) -> Option<(DoubleDouble, DoubleDouble)> {
        let (mir, cfg) = (self.mir_dd?, self.cfg_dd?);
        (mir.to_f64().is_finite() && cfg.to_f64().is_finite()).then_some((mir, cfg))
    }

    /// Whether a reference was taken on both sides and one of them left the
    /// reals. See [`Self::references`].
    fn reference_left_the_reals(&self) -> bool {
        self.mir_dd.is_some() && self.cfg_dd.is_some() && self.references().is_none()
    }

    /// `|mir_dd − cfg_dd|` as a share of the entry, taken *in* double-double so
    /// a difference below the `f64` resolution is still a number.
    ///
    /// This is what the criterion is asked of for a derivative entry. It is not
    /// an estimate of the `f64` deviation and is usually orders below it: the
    /// `f64` deviation is dominated by the two routes' rounding, and this one
    /// has none of it left.
    fn reference_deviation(&self) -> Option<f64> {
        let (mir, cfg) = self.references()?;
        let scale = mir.to_f64().abs().max(cfg.to_f64().abs());
        let difference = mir.absolute_distance_to(cfg.to_f64());
        if difference == 0.0 {
            return Some(0.0);
        }
        if scale == 0.0 {
            return Some(f64::INFINITY);
        }
        Some(difference / scale)
    }

    /// `E_mir = |mir_f64 − mir_dd|`, relative: the shipped route's own rounding
    /// error at this entry, measured rather than bounded.
    fn mir_error(&self) -> Option<f64> {
        Some(self.references()?.0.relative_distance_to(self.mir))
    }

    /// `E_cfg`, the same for the CFG route.
    fn cfg_error(&self) -> Option<f64> {
        Some(self.references()?.1.relative_distance_to(self.cfg))
    }

    /// `E_cfg / (E_mir + u)`: how much noisier the CFG route's arithmetic is
    /// than the shipped route's at this entry.
    ///
    /// The `u` in the denominator is not slack. Two routes that both evaluate
    /// an entry exactly give `0/0`, and an entry where only the shipped route
    /// is exact would otherwise report an infinite ratio for a difference of
    /// one unit in the last place. Charging the denominator one unit round-off
    /// makes the ratio read "in units of the smallest error there is".
    fn noise_ratio(&self) -> Option<f64> {
        let mir = self.mir_error()?;
        let cfg = self.cfg_error()?;
        Some(cfg / (mir + UNIT_ROUNDOFF))
    }

    /// Whether either route's `f64` evaluation of this entry has no correct
    /// digit left in it.
    ///
    /// # Why this is a separate verdict from the agreement one
    ///
    /// Asking the agreement question in double-double answers whether the two
    /// chain rules compute the same *real* derivative, and that is the question
    /// worth asking — but it is not the only way a route can be wrong, and the
    /// defect this estate actually had is the other way. The pre-`31be31f82`
    /// `max` rule `db + (da − db)·c` is algebraically `da`, and it is `da` in
    /// double-double too, because `da − db` is exact there. What it is not is
    /// `da` in `f64`: at `da = 1e-9` and `db = 1e9` the subtraction returns
    /// `−db` and the addition returns zero. The algebra agrees; the arithmetic
    /// lost everything.
    ///
    /// A measurement sees that where a bound could not and where the agreement
    /// criterion does not look: `E = |value_f64 − value_dd| / |value_dd|` comes
    /// out at exactly one, which is the statement "this evaluation differs from
    /// the quantity it computes by the whole of that quantity". A value with no
    /// significant figure is not a reading of the entry, whichever route
    /// produced it, and no comparison against it means anything.
    ///
    /// One is the threshold and it is a definition rather than a fit: below it
    /// a value carries some fraction of a digit, at it the value carries none.
    /// Every entry is reported with its `E` either way, so an entry drifting
    /// toward it is visible long before it arrives.
    fn lost_significance(&self) -> bool {
        [self.mir_error(), self.cfg_error()]
            .into_iter()
            .flatten()
            // `>=` rather than `< 1.0` negated, so a NaN — which is what a
            // reference that left the reals produces — is *not* read as a loss
            // here. That case is `Self::reference_left_the_reals`, and the two
            // are different findings.
            .any(|error| error >= 1.0)
    }

    /// The deviation the criterion is applied to: the double-double one for a
    /// derivative entry that has both references, and the `f64` one otherwise.
    fn judged_deviation(&self) -> f64 {
        if self.is_derivative() {
            if let Some(deviation) = self.reference_deviation() {
                return deviation;
            }
        }
        self.deviation
    }

    fn bound(&self) -> f64 {
        let reassociation = REASSOCIATION_BUDGET * self.operations as f64 * f64::EPSILON;
        if self.is_derivative() {
            reassociation.max(DERIVATIVE_AGREEMENT)
        } else {
            reassociation
        }
    }

    /// Which question was asked, which is the reading a per-module line has to
    /// carry: an entry judged in `f64` is one whose reference is missing.
    fn criterion(&self) -> &'static str {
        if !self.is_derivative() {
            return "reassociation";
        }
        if self.references().is_some() {
            "double-double"
        } else {
            "derivative-agreement"
        }
    }

    fn describe(&self) -> String {
        let optional = |value: Option<f64>| value.unwrap_or(f64::NAN);
        format!(
            "{} point={} mir={:.17e} cfg={:.17e} operations={} deviation={:.3e} \
             reference_deviation={:.3e} bound={:.3e} criterion={} mir_dd={:.17e} \
             cfg_dd={:.17e} e_mir={:.3e} e_cfg={:.3e} noise_ratio={:.3e}",
            self.entry,
            self.point,
            self.mir,
            self.cfg,
            self.operations,
            self.deviation,
            optional(self.reference_deviation()),
            self.bound(),
            self.criterion(),
            optional(self.mir_dd.map(DoubleDouble::to_f64)),
            optional(self.cfg_dd.map(DoubleDouble::to_f64)),
            optional(self.mir_error()),
            optional(self.cfg_error()),
            optional(self.noise_ratio()),
        )
    }
}

#[derive(Default)]
struct Tally {
    models: usize,
    built: usize,
    /// Modules the plan builder would not build a CFG plan for. Production
    /// falls those back whole to the postfix plan, so a refusal costs coverage
    /// and nothing else.
    refused: usize,
    /// Modules whose CFG plan *built* and then would not compile. Production
    /// raises that error after the fallback point, so the model cannot be
    /// constructed at all: any is a failure of this census.
    codegen_failed: usize,
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
    /// Structural zeros the shipped route read as non-finite at a point where
    /// its own assignment pass produced no model. See [`check_structural_zero`].
    structural_zeros_not_evaluable: usize,
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
    /// Derivative comparisons judged in double-double, which is every one that
    /// has both routes' references.
    referenced: usize,
    /// Derivative comparisons with no double-double reference on one side or
    /// the other, held to the `f64` criterion instead.
    reference_missing: usize,
    /// Models whose CFG this census could not rebuild for the reference walk,
    /// so every derivative comparison in them is judged in `f64`.
    models_without_reference: usize,
    /// Entries whose shipped postfix stream this census's walker could not
    /// evaluate, by operation name.
    walker_refusals: BTreeSet<&'static str>,
    /// Entries where the walker's own `f64` walk disagreed with the machine
    /// code it is modelling. Any is a finding about the walker.
    walker_disagreements: usize,
    /// Significant entries one route evaluated with no correct digit in it.
    /// See `Comparison::lost_significance`.
    lost_entries: usize,
    /// Derivative comparisons where both references were taken and one left the
    /// reals. See `Comparison::references`.
    reference_not_finite: usize,
    /// Derivative comparisons whose CFG reference walk did not reproduce the
    /// compiled CFG plan's own `f64` reading, so it is a reference for a
    /// different computation and was dropped.
    reference_diverged: usize,
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
#[derive(Clone, Copy)]
enum StructuralZero {
    /// Both routes read exactly zero. The claim holds here.
    Absent,
    /// The shipped route read something else. A finding.
    Present,
    /// The shipped route read no number at all. See [`check_structural_zero`].
    NotEvaluable,
}

/// # Why a non-finite shipped reading is not a counterexample
///
/// "Structurally absent" is a claim about *dependence*: the residual does not
/// depend on this unknown, so the derivative is zero. What decides it is
/// whether the dependence exists, and the shipped route's reading is evidence
/// about that only when it is a number.
///
/// It reaches the same entry by a different road — it carries every column the
/// front end's chain rule emitted, and a column the residual does not depend on
/// is that rule run with a zero seed. `0 * NaN` and `0 / 0` are NaN, so a dead
/// column reads NaN wherever the factor it is multiplied by is not a number:
/// at a bias the model's own domain rejects, and at a kink where the derivative
/// of the primal is `0/0`. Both are in the corpus. BSIMSOI and HICUM read NaN
/// at one of the three points from a node potential drawn from `(-1, 1)` — 130
/// and 4 entries — and HICUM's `jacobians[12][2]` reads NaN out of an
/// eleven-operation program whose inputs are all finite, which is the kink.
///
/// Neither says the residual depends on the unknown. Failing a module on them
/// would be failing it on the arithmetic of a column that contributes nothing.
///
/// So a non-finite shipped reading is excused — but never silently, and never
/// on its own:
///
/// * the CFG's own reading must still be exactly zero, so a CFG entry that
///   stopped being the constant zero is a finding at every point;
/// * every excused reading is counted as
///   [`Tally::structural_zeros_not_evaluable`] and printed per module and in
///   the totals, so the class cannot grow unseen;
/// * the entry is still asserted at every operating point where the shipped
///   route does read a number, so a dependence the CFG really dropped is caught
///   wherever the shipped route can say so.
///
/// A shipped route that stamps NaN into a matrix column is a finding of its
/// own. It is a finding about the shipped route, not about this comparison,
/// which is why it is counted here and raised there.
fn check_structural_zero(comparison: &Comparison) -> StructuralZero {
    if comparison.cfg != 0.0 {
        return StructuralZero::Present;
    }
    if comparison.mir == 0.0 {
        return StructuralZero::Absent;
    }
    if comparison.mir.is_finite() {
        return StructuralZero::Present;
    }
    StructuralZero::NotEvaluable
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

/// Every derivative entry's double-double value, one map per operating point.
type ReferenceValues = Vec<HashMap<CfgPlanEntry, DoubleDouble>>;

/// Fill `bounds` for the named equations off one lowering, and say which of
/// them the interpreter refused.
///
/// # One pass per row, not per entry and not per model
///
/// A row's Jacobian entries share nearly all of their cone with each other, so
/// the outputs are pruned to the row's *union* and interpreted once per
/// operating point: every entry's bound comes off one snapshot. Pruning to the
/// whole model's union instead would be one pass fewer, and would lose a whole
/// module to a single unevaluable value. The row is where the plan builder cuts
/// for the same reason, and it is the granularity at which a refusal is worth
/// reporting.
fn row_reference(
    shipped: &CensusModel,
    mut cfg: CfgModel,
    wanted: &[usize],
    bounds: &mut ReferenceValues,
) -> Vec<usize> {
    let model = &shipped.model;
    let artifact = &shipped.canonical_ir;
    let mut refused = Vec::new();
    if model.stamp_programs.len() != cfg.residuals.len() {
        return wanted.to_vec();
    }

    // Charges first, for the reason the plan builder gives: the extraction
    // *builds* the values a scaled or summed charge needs, so it has to run
    // before anything is differentiated.
    let charges = stored_charges(&mut cfg.function, &cfg.residuals);
    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let Ok(mut differentiated) = differentiate(&cfg.function, &seeds) else {
        return wanted.to_vec();
    };
    // Every read-out before anything else: taking one appends an instruction.
    let jacobian_rows: Vec<Vec<Option<ValueId>>> = cfg
        .residuals
        .iter()
        .map(|residual| differentiated.derivative_row(*residual))
        .collect();
    let reactive_rows: Vec<Vec<Option<ValueId>>> = charges
        .iter()
        .map(|charge| match charge {
            Some(charge) => differentiated.derivative_row(*charge),
            None => Vec::new(),
        })
        .collect();
    let Ok(scalarized) = scalarize_lanes(&differentiated.function) else {
        return wanted.to_vec();
    };
    let Ok(column_lanes) = ShippedColumnLanes::build(model, &artifact.mir) else {
        return wanted.to_vec();
    };
    let Ok(branch_unknowns) =
        crate::jit::plan_builder::canonical_branch_unknown_runtime_map(model, &artifact.mir)
    else {
        return wanted.to_vec();
    };
    let parameter_defaults: Vec<Option<f64>> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default)
        .collect();
    // The state array is sized zero: this is the interpreter's static
    // evaluation, where `ddt` and `idt` answer zero and no slot is read.
    // Everything the two routes share — the parameters, the potentials, the
    // branch flows — is drawn from the same seed in the same order, so these are
    // the points the compiled plans stand at.
    let inputs: Vec<_> = CENSUS_POINTS
        .iter()
        .map(|(seed, analysis)| {
            let mut point = OperatingPoint::new(
                *seed,
                *analysis,
                &parameter_defaults,
                model.num_terminals,
                model.internal_nodes,
                &branch_unknowns,
                0,
                0,
            )
            .with_initial_step();
            point.set_event_state_slots(cfg.event_state_candidates.len());
            lift_inputs(
                &point.interpreter_inputs(artifact.mir.nodes.len(), artifact.mir.branches.len()),
            )
        })
        .collect();

    let lane_of = |axis: &_| -> Option<usize> {
        let lane = column_lanes.lane(axis)?;
        (lane < seeds.len() && Some(lane) != correction_lane).then_some(lane)
    };
    for stamp_index in wanted.iter().copied() {
        let stamp = &model.stamp_programs[stamp_index];
        let mut entries: Vec<(CfgPlanEntry, ValueId)> = Vec::new();
        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let Some(lane) = lane_of(&jacobian.col_axis) else {
                continue;
            };
            let Some(value) = jacobian_rows[stamp_index].get(lane).copied().flatten() else {
                continue;
            };
            if let Some(scalar) = scalarized.scalar(value) {
                entries.push((CfgPlanEntry::Jacobian(stamp_index, entry_index), scalar));
            }
        }
        for (entry_index, reactive) in stamp.reactive_jacobians.iter().enumerate() {
            let Some(lane) = lane_of(&reactive.col_axis) else {
                continue;
            };
            let Some(value) = reactive_rows[stamp_index].get(lane).copied().flatten() else {
                continue;
            };
            if let Some(scalar) = scalarized.scalar(value) {
                entries.push((
                    CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index),
                    scalar,
                ));
            }
        }
        if entries.is_empty() {
            continue;
        }
        let outputs: Vec<ValueId> = entries.iter().map(|(_, value)| *value).collect();
        let (pruned, mapped) = prune_cfg_to_outputs(&scalarized.function, &outputs);
        let mut walked = Vec::with_capacity(inputs.len());
        for point in &inputs {
            match evaluate_cfg(&pruned, point) {
                Ok(snapshot) => walked.push(snapshot),
                Err(error) => {
                    println!(
                        "cfg-mir model={} reference=refused stamp={stamp_index} detail={error}",
                        shipped.name
                    );
                    break;
                }
            }
        }
        if walked.len() != inputs.len() {
            refused.push(stamp_index);
            continue;
        }
        for (index, snapshot) in walked.into_iter().enumerate() {
            for ((entry, _), output) in entries.iter().zip(&mapped) {
                if let Some(value) = snapshot.value(*output) {
                    bounds[index].insert(*entry, value);
                }
            }
        }
    }
    refused
}

/// Every derivative entry's rounding bound, taken off the plan's own lowering
/// where the interpreter can walk it.
///
/// # Two lowerings, chosen per equation
///
/// `from_hir_for_executable_backend` is what the CFG plan is built from, so it
/// is the cone whose bound is wanted. It freezes a branch-current probe into a
/// `ContributedCurrent`: storage an executable plan keeps and
/// the reference interpreter, which evaluates one body to a number, has none
/// of. It refuses the kind by name rather than answering it with the
/// contribution's own expression, which would undo exactly the freezing the
/// kind exists to state.
///
/// So the refusal is taken per equation rather than per module. Rows reaching
/// no frozen current are bounded on the plan's own cone; the rest are retried
/// on `from_hir`, the lowering [`super::cfg_census`] and
/// `tests/cfg_derivatives.rs` already hold the interpreter to, where the same
/// probed current is computed from the contribution it was frozen from rather
/// than read back. Every retried equation is named in the output, so the class
/// cannot grow unseen, and an equation neither lowering can walk is named too.
///
/// The two artifacts are built one after the other and the first is released
/// before the second, so the peak working set is one of them rather than both —
/// and the whole thing runs and is released before either plan is compiled.
/// `asmhemt` has taken this pipeline past twenty gigabytes on its own.
///
/// # Rebuilt rather than borrowed
///
/// The plan builder makes the same function on its way to machine code but
/// keeps no map from a plan entry back to the CFG value it lowered, and it is
/// another lane's file. So the steps are repeated here — lower, store charges,
/// differentiate, scalarize — from the same artifact by the same deterministic
/// passes, which is what makes the entry-to-value map the same one.
fn entry_reference(shipped: &CensusModel) -> Option<ReferenceValues> {
    let artifact = &shipped.canonical_ir;
    let mut bounds: ReferenceValues = vec![HashMap::new(); CENSUS_POINTS.len()];
    let wanted: Vec<usize> = (0..shipped.model.stamp_programs.len()).collect();

    let executable =
        CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir).ok()?;
    let refused = row_reference(shipped, executable, &wanted, &mut bounds);
    if !refused.is_empty() {
        println!(
            "cfg-mir model={} reference=interpretable equations={}",
            shipped.name,
            refused.len()
        );
        if let Ok(interpretable) = CfgModel::from_hir(&artifact.hir, &artifact.mir) {
            let still = row_reference(shipped, interpretable, &refused, &mut bounds);
            if !still.is_empty() {
                println!(
                    "cfg-mir model={} reference=unavailable equations={}",
                    shipped.name,
                    still.len()
                );
            }
        }
    }
    Some(bounds)
}

/// Re-walk the shipped plan's postfix streams in double-double at one point.
///
/// # Why the walker's own `f64` walk runs alongside
///
/// The reference is only a reference if the thing it is a reference *for* is
/// the same computation. [`PostfixWalk`] reads the operation semantics off
/// [`x64::codegen`](crate::native::x64), but reading them off and reproducing
/// them are different claims, so the same streams are walked twice: once in
/// `f64`, where the answer has to be the machine code's own, and once in
/// double-double, which is the value that is kept. An entry whose `f64` walk
/// disagrees with the compiled reading gets *no* reference — a walker that does
/// not model an entry cannot be its reference — and the disagreement is
/// counted.
///
/// # Only the first assignment pass
///
/// The census fills its `f64` variable array with `run_assignments` alone, so
/// that is what this mirrors. Adding `post_assignments` here would give the
/// double-double walk variables the `f64` walk it is measuring never had, and
/// `E_mir` would then be the distance between two different computations rather
/// than one computation's rounding.
fn mir_reference(
    plan: &crate::jit::model_plan::NativeModelPlan,
    point: &super::mir_postfix::MirPoint<'_>,
    variables: usize,
    prelude_slots: usize,
    wanted: &[(CfgPlanEntry, f64)],
    tally: &mut Tally,
) -> HashMap<CfgPlanEntry, DoubleDouble> {
    let mut narrow: PostfixWalk<'_, f64> = PostfixWalk::new(point, variables, prelude_slots);
    let mut wide: PostfixWalk<'_, DoubleDouble> = PostfixWalk::new(point, variables, prelude_slots);
    narrow.fill_variables(&plan.assignments);
    wide.fill_variables(&plan.assignments);

    let mut values = HashMap::new();
    for (entry, compiled) in wanted {
        let Some(PlanProgram::Postfix(program)) = entry_program(plan, *entry) else {
            continue;
        };
        let (Ok(walked), Ok(reference)) = (narrow.run(program), wide.run(program)) else {
            continue;
        };
        // The same reading on both counts, or the walker is not modelling this
        // entry: the `f64` walk against the machine code, and the `f64` walk
        // the double-double scalar carries alongside itself against that.
        let agrees = same_reading(walked, *compiled) && same_reading(reference.narrow(), *compiled);
        if !agrees {
            tally.walker_disagreements += 1;
            println!(
                "cfg-mir walker_disagreement entry={entry} walked={walked:.17e} \
                 carried={:.17e} compiled={compiled:.17e}",
                reference.narrow()
            );
            continue;
        }
        values.insert(*entry, reference);
    }
    tally.walker_refusals.extend(narrow.refusals());
    tally.walker_refusals.extend(wide.refusals());
    values
}

/// Whether two evaluations of one entry are the same reading.
///
/// Not `total_cmp`, which was the first thing written here and is wrong for
/// this question twice over: it is a total order over *bit patterns*, so it
/// separates `+0.0` from `−0.0` and one NaN encoding from another. A route that
/// stamps a NaN and a walker that reproduces it agree about the entry whatever
/// payloads the two NaNs carry — `hisimhv_n5_va`'s `reactive_jacobians[30][0]`
/// is that case, and `total_cmp` reported it as the walker failing to model the
/// backend. Ordinary equality answers both, with the NaN case named.
fn same_reading(left: f64, right: f64) -> bool {
    left == right || (left.is_nan() && right.is_nan())
}

/// One plan entry's program, whichever form its route produced.
fn entry_program(
    plan: &crate::jit::model_plan::NativeModelPlan,
    entry: CfgPlanEntry,
) -> Option<&PlanProgram> {
    match entry {
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
    }
}

/// The distribution of one route's measured rounding error over a model's
/// derivative entries, and of the two routes' ratio.
///
/// The reading the flip's pre-flight wants: a CFG route whose arithmetic is
/// systematically noisier than the shipped route's is a finding even where both
/// are far inside the criterion.
#[derive(Default)]
struct ReferenceSpread {
    mir: Vec<f64>,
    cfg: Vec<f64>,
    ratios: Vec<f64>,
}

impl ReferenceSpread {
    fn push(&mut self, comparison: &Comparison) {
        for (values, reading) in [
            (&mut self.mir, comparison.mir_error()),
            (&mut self.cfg, comparison.cfg_error()),
            (&mut self.ratios, comparison.noise_ratio()),
        ] {
            if let Some(reading) = reading
                && reading.is_finite()
            {
                values.push(reading);
            }
        }
    }

    fn quantiles(values: &mut Vec<f64>) -> (f64, f64, f64) {
        if values.is_empty() {
            return (f64::NAN, f64::NAN, f64::NAN);
        }
        values.sort_unstable_by(f64::total_cmp);
        (
            values[0],
            values[values.len() / 2],
            values[values.len() - 1],
        )
    }

    fn describe(&mut self) -> String {
        if self.mir.is_empty() && self.cfg.is_empty() {
            return "reference[none]".to_string();
        }
        let count = self.mir.len().max(self.cfg.len());
        let (_, mir_median, mir_max) = Self::quantiles(&mut self.mir);
        let (_, cfg_median, cfg_max) = Self::quantiles(&mut self.cfg);
        let (_, ratio_median, ratio_max) = Self::quantiles(&mut self.ratios);
        format!(
            "reference[n={count} e_mir_median={mir_median:.3e} e_mir_max={mir_max:.3e} \
             e_mir_max_ulp={:.1} e_cfg_median={cfg_median:.3e} e_cfg_max={cfg_max:.3e} \
             e_cfg_max_ulp={:.1} ratio_median={ratio_median:.3e} ratio_max={ratio_max:.3e}]",
            mir_max / f64::EPSILON,
            cfg_max / f64::EPSILON,
        )
    }
}

/// Compile both plans for one module and compare every entry they share.
#[allow(clippy::too_many_lines)]
fn census_model(shipped: &CensusModel, tally: &mut Tally) -> Option<String> {
    let module = &shipped.name;
    let model = &shipped.model;
    let artifact = &shipped.canonical_ir;

    // The CFG reference walk first, and released before either plan is
    // compiled: both passes build the same differentiated body, and holding two
    // of them is what puts this census into paging on the largest modules.
    let cfg_reference = entry_reference(shipped);
    if cfg_reference.is_none() {
        tally.models_without_reference += 1;
        println!(
            "cfg-mir model={module} reference=unavailable; every derivative entry is judged in \
             f64"
        );
    }
    let cfg_reference = cfg_reference.unwrap_or_else(|| vec![HashMap::new(); CENSUS_POINTS.len()]);

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
    // A backend that will not emit a plan the builder *built* is a different
    // finding from a builder that refused to build one, and counting it as a
    // refusal hid the difference: `build_model_plan_from_canonical_cfg` failing
    // is a module production falls back whole through the `[JIT]` seam, which
    // costs coverage and nothing else, while `compile_model_plan` failing on a
    // built plan is a module production cannot construct at all. bsimcmg_va's
    // 417 MB image was the second and this census read it as the first.
    //
    // Still not a panic — aborting here would take the other forty-two modules'
    // measurements with it — but the run fails at the end.
    let cfg_native = match crate::native::x64::compile_model_plan(model, &cfg_plan.plan) {
        Ok(native) => native,
        Err(error) => {
            tally.codegen_failed += 1;
            println!("cfg-mir model={module} codegen_failed=x64 detail={error}");
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
    let mut points: Vec<OperatingPoint> = CENSUS_POINTS
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
            .with_initial_step()
            // The CFG plan's prelude publishes into these, and every value
            // entry it covers returns one of them. A census that ran that
            // plan without them would write through a null pointer.
            .with_prelude_slots(cfg_storage.prelude_slots)
        })
        .collect();

    let positions = entry_positions(model);
    tally.entries += positions.len();
    let mut worst: Option<Comparison> = None;
    let mut worst_ratio = 0.0_f64;
    let mut exact = 0_usize;
    let mut compared = 0_usize;
    // Every one of them, not the first. A module can be over the bound at
    // several entries and at several points, and reporting one made the totals
    // and the message disagree: `over_bound=4` under a single named entry.
    let mut over: Vec<String> = Vec::new();
    // Entries a route evaluated with no significant figure left. A second
    // failure class rather than a second reason for the first one: they are
    // different findings and the message has to say which.
    let mut lost: Vec<String> = Vec::new();
    let mut referenced_here = 0_usize;
    let mut missing_here = 0_usize;
    let mut not_finite_here = 0_usize;
    let mut diverged_here = 0_usize;
    // The three entries where the CFG route's measured rounding is furthest
    // above the shipped route's. Named rather than counted, because the reading
    // the flip's pre-flight wants is *which* entry, not how many.
    let mut noisiest: Vec<Comparison> = Vec::new();
    let mut noisiest_floor = 0.0_f64;
    let mut f64_worst: Option<Comparison> = None;
    let mut f64_worst_deviation = 0.0_f64;
    let mut spread = ReferenceSpread::default();
    let mut nonzero_structural: Option<Comparison> = None;
    let mut guarded_noise_worst = 0.0_f64;
    let mut guarded_noise_case: Option<Comparison> = None;
    let mut guarded_noise_third: Option<Comparison> = None;
    let mut guarded_here = 0_usize;
    let mut insignificant_worst = 0.0_f64;
    let mut insignificant_case: Option<Comparison> = None;
    let mut insignificant_here = 0_usize;
    let mut not_evaluable_here = 0_usize;

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
        // The CFG plan's own assignment pass, in the position the device runs
        // it: after the variables are filled and before any entry is read. The
        // shipped plan has none, and its entries do not read a slot.
        cfg_native.run_prelude(&context, variables.as_ptr());
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

        // The shipped route's reference, taken once for this point: the
        // assignment pass is most of the work and every entry shares it. Only
        // the entries the CFG side has a reference for are asked, because a
        // reference on one side alone answers nothing.
        let cfg_here = cfg_reference.get(index);
        let wanted: Vec<(CfgPlanEntry, f64)> = readings
            .iter()
            .filter(|(entry, ..)| cfg_here.is_some_and(|values| values.contains_key(entry)))
            .map(|(entry, mir, _)| (*entry, *mir))
            .collect();
        let mir_here = if wanted.is_empty() {
            HashMap::new()
        } else {
            let walked = point.mir_point(&storage.currents, &storage.branch_currents);
            mir_reference(
                &mir_plan,
                &walked,
                variables.len(),
                mir_storage.prelude_slots,
                &wanted,
                tally,
            )
        };

        for (entry, mir, cfg) in readings {
            compared += 1;
            let operations = entry_operations(&mir_plan, &cfg_plan.plan, entry);
            // The interpreted cone's own `f64` walk has to be the compiled
            // plan's reading, or the reference is a reference for some other
            // computation. `super::cfg_census` pins the block program
            // bit-identical to the interpreter on every shipped module, so a
            // disagreement here is about this census's rebuild of the lowering
            // — it repeats the lower/differentiate/scalarize passes rather than
            // borrowing the plan builder's — and not about the route.
            let cfg_candidate = cfg_here.and_then(|values| values.get(&entry)).copied();
            let cfg_dd = cfg_candidate
                .filter(|value| same_reading(value.narrow(), cfg));
            let diverged = cfg_candidate.is_some() && cfg_dd.is_none();
            let comparison = Comparison {
                entry,
                point: index,
                mir,
                cfg,
                operations,
                deviation: deviation(mir, cfg).unwrap_or(0.0),
                mir_dd: mir_here.get(&entry).copied(),
                cfg_dd,
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
                match check_structural_zero(&comparison) {
                    StructuralZero::Absent => {}
                    StructuralZero::NotEvaluable => {
                        tally.structural_zeros_not_evaluable += 1;
                        not_evaluable_here += 1;
                    }
                    StructuralZero::Present => {
                        tally.nonzero_structural_zeros += 1;
                        nonzero_structural.get_or_insert(comparison);
                    }
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
            // Accounted for here rather than at the top of the loop: an entry
            // the significance gate, the structural-zero check or the noise
            // criterion decided is not one this criterion judges, so counting
            // its reference would make the distribution a reading over entries
            // nothing was asked of. `l_utsoi`'s twenty-six-orders-down reactive
            // entries are the case — they carry no correct digit and they are
            // gated away for exactly that reason.
            if comparison.is_derivative() {
                if comparison.references().is_some() {
                    tally.referenced += 1;
                    referenced_here += 1;
                    spread.push(&comparison);
                } else if diverged {
                    tally.reference_diverged += 1;
                    diverged_here += 1;
                } else if comparison.reference_left_the_reals() {
                    tally.reference_not_finite += 1;
                    not_finite_here += 1;
                } else {
                    tally.reference_missing += 1;
                    missing_here += 1;
                }
                // The entry where the two routes' *`f64`* readings are furthest
                // apart is the one the wider precision changed the verdict on,
                // and naming it is how a reader checks that it did.
                if comparison.deviation >= f64_worst_deviation {
                    f64_worst_deviation = comparison.deviation;
                    f64_worst = Some(comparison);
                }
            }
            let bound = comparison.bound();
            let judged = comparison.judged_deviation();
            if judged > bound {
                tally.over_bound += 1;
                over.push(comparison.describe());
                continue;
            }
            // A separate verdict, and a separate failure: see
            // `Comparison::lost_significance`.
            if comparison.lost_significance() {
                tally.lost_entries += 1;
                lost.push(comparison.describe());
                continue;
            }
            let ratio = if bound > 0.0 { judged / bound } else { 0.0 };
            if ratio >= worst_ratio {
                worst_ratio = ratio;
                worst = Some(comparison);
            }
            // The entries where the CFG route's measured rounding is furthest
            // above the shipped route's are the census's own finding, and they
            // are kept by name rather than only counted: three of them, because
            // one is a coincidence and the distribution says the rest.
            if let Some(noise) = comparison.noise_ratio()
                && noise.is_finite()
                && noise > 1.0
                && (noisiest.len() < 3 || noise > noisiest_floor)
            {
                noisiest.push(comparison);
                noisiest.sort_by(|left, right| {
                    right
                        .noise_ratio()
                        .unwrap_or(0.0)
                        .total_cmp(&left.noise_ratio().unwrap_or(0.0))
                });
                noisiest.truncate(3);
                noisiest_floor = noisiest
                    .last()
                    .and_then(Comparison::noise_ratio)
                    .unwrap_or(0.0);
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
         not_evaluable={not_evaluable_here} \
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
        if over.is_empty() {
            String::new()
        } else {
            format!(" OVER_BOUND={}", over.len())
        },
    );
    println!(
        "cfg-mir model={module} referenced={referenced_here} \
         reference_missing={missing_here} reference_not_finite={not_finite_here} \
         reference_diverged={diverged_here} {}{}",
        spread.describe(),
        f64_worst
            .as_ref()
            .map(|case| format!(" f64_worst[{}]", case.describe()))
            .unwrap_or_default(),
    );
    for case in &noisiest {
        println!("cfg-mir model={module} noisiest[{}]", case.describe());
    }
    for case in &over {
        println!("cfg-mir model={module} OVER_BOUND[{case}]");
    }
    for case in &lost {
        println!("cfg-mir model={module} LOST_SIGNIFICANCE[{case}]");
    }
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
    if !over.is_empty() {
        return Some(format!(
            "{module}: {} comparison(s) outside the bound:\n  {}",
            over.len(),
            over.join("\n  ")
        ));
    }
    if !lost.is_empty() {
        return Some(format!(
            "{module}: {} significant entr(ies) a route evaluated with no correct digit:\n  {}",
            lost.len(),
            lost.join("\n  ")
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
        "cfg-mir models={} built={} refused={} codegen_failed={} entries={} comparisons={} exact={} \
         runtime_errors={} structural_zeros={} over_bound={} nonzero_structural_zeros={} \
         structural_zeros_not_evaluable={} \
         guarded_noise_entries={} guarded_noise_agreed={} guarded_noise_third_value={} \
         below_significance={} referenced={} reference_missing={} \
         reference_not_finite={} reference_diverged={} models_without_reference={} \
         walker_disagreements={} lost_entries={} walker_refusals={:?}",
        tally.models,
        tally.built,
        tally.refused,
        tally.codegen_failed,
        tally.entries,
        tally.comparisons,
        tally.exact,
        tally.runtime_errors,
        tally.structural_zeros,
        tally.over_bound,
        tally.nonzero_structural_zeros,
        tally.structural_zeros_not_evaluable,
        tally.guarded_noise_entries,
        tally.guarded_noise_agreed,
        tally.guarded_noise_third_value,
        tally.insignificant,
        tally.referenced,
        tally.reference_missing,
        tally.reference_not_finite,
        tally.reference_diverged,
        tally.models_without_reference,
        tally.walker_disagreements,
        tally.lost_entries,
        tally.walker_refusals,
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    }
    // The walker is the reference's other half, and a reference that does not
    // reproduce the machine code is not one. This is asserted rather than
    // reported: an entry whose `f64` walk disagrees is already excluded from
    // the measurement above, so a nonzero count means the walker has stopped
    // modelling the backend and every figure it produced is suspect.
    assert_eq!(
        tally.walker_disagreements, 0,
        "the postfix walker must reproduce the machine code it is the reference for"
    );
    assert!(
        tally.comparisons > 0,
        "the census must actually execute both plans"
    );
    // A plan the builder built and the backend would not emit is a module
    // production would fail to construct: `compile_model_plan` raises its error
    // after `build_default_model_plan` has already decided not to fall back.
    // Naming it separately from a refusal is the whole point of the counter.
    assert_eq!(
        tally.codegen_failed, 0,
        "a CFG plan this census built would not compile; production has no fallback left at that \
         point"
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
        CfgPlanRefusal::PreludeLiveCurrent,
    ] {
        assert!(!class.name().is_empty());
        assert!(!class.name().contains(' '), "{:?}", class);
    }
}

/// VBIC's temperature mapping, reduced to the two statements that make an
/// operating point without `@(initial_step)` meaningless.
///
/// `tiniK` is written by the initial block and divided by in the body, and the
/// exponential's coefficient is the zero VBIC's `dear` defaults to. Skip the
/// block and `tiniK` is its slot's zero, `rT` is infinite, and `-0.0 * -inf` is
/// a NaN that everything downstream inherits.
const TEMPERATURE_MAPPED_BY_AN_INITIAL_STEP: &str = r#"
module cfg_initial_step_temperature(p, n);
  inout p, n;
  electrical p, n;
  parameter real tnom = 27.0;
  parameter real dear = 0.0;
  parameter real g0 = 1.0e-3;
  real tiniK;
  real rT;
  real g;
  analog begin
    @(initial_step) begin
      tiniK = 273.15 + tnom;
    end
    rT = $temperature / tiniK;
    g = g0 * exp(-dear * (1.0 - rT));
    I(p, n) <+ V(p, n) * g;
  end
endmodule
"#;

/// The census's operating point has to run the model's initial step.
///
/// Without it a third of VBIC's variables are NaN before either plan is asked
/// for anything — measured at 347 of 1052 — so every entry that loads one
/// compares two readings of the same NaN, and the structural-zero check reads
/// the shipped route's NaN as a nonzero it was supposed to evaluate to zero.
/// This pins the reason rather than the count: the same module, at the same
/// point, with and without the initial block.
#[test]
fn an_operating_point_without_the_initial_step_poisons_the_variable_array() {
    let compiler = crate::VerilogACompiler::new(crate::CompilerOptions::default());
    let model = compiler
        .compile(TEMPERATURE_MAPPED_BY_AN_INITIAL_STEP)
        .expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(TEMPERATURE_MAPPED_BY_AN_INITIAL_STEP)
        .expect("compile canonical IR");
    let plan = build_model_plan_with_canonical_ir(&model, &artifact).expect("shipped plan");
    let native = crate::native::x64::compile_model_plan(&model, &plan).expect("shipped codegen");

    let parameter_defaults: Vec<Option<f64>> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default)
        .collect();
    let branch_unknowns =
        crate::jit::plan_builder::canonical_branch_unknown_runtime_map(&model, &artifact.mir)
            .expect("branch unknown map");
    let storage = native.required_storage();
    let state_len = storage
        .state_values
        .max(storage.state_initialized)
        .max(storage.state_candidate_valid)
        + 8;

    let nan_count = |initial_step: bool| {
        let mut point = OperatingPoint::new(
            0x0005_EED1,
            0,
            &parameter_defaults,
            model.num_terminals,
            model.internal_nodes,
            &branch_unknowns,
            state_len,
            0,
        );
        if initial_step {
            point = point.with_initial_step();
        }
        let context = point.context();
        let mut variables = vec![0.0_f64; model.num_variables + 64];
        context.clear_runtime_error();
        native.run_assignments(&context, variables.as_mut_ptr());
        let _ = context.take_runtime_error();
        variables
            .iter()
            .take(model.num_variables)
            .filter(|value| value.is_nan())
            .count()
    };

    assert!(
        nan_count(false) > 0,
        "without the initial step the body divides by an unwritten slot, so this module is not \
         evaluable and the census would be comparing NaNs"
    );
    assert_eq!(
        nan_count(true),
        0,
        "with the initial step every variable is a number"
    );
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
        mir_dd: None,
        cfg_dd: None,
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

/// A `max` whose losing arm's derivative is eighteen orders above the winner's.
///
/// At `V(p,n) = v0` the second arm is exactly zero and the first is `ga * v0`,
/// so `max` takes the first: `d/dV(p)` is `ga`. The *other* arm's derivative is
/// `gb`, which is what makes this the shape a mis-differentiated `max` fails
/// on. It is the shape a compact model's floor guard has — `max(x, xmin)` with
/// a steep `x` — rather than an invented one.
const MAX_AGAINST_A_STEEP_LOSER: &str = r#"
module cfg_masked_max(p, n);
  inout p, n;
  electrical p, n;
  parameter real ga = 1.0e-9;
  parameter real gb = 1.0e9;
  parameter real v0 = 0.5;
  analog begin
    I(p, n) <+ max(ga * V(p, n), gb * (V(p, n) - v0));
  end
endmodule
"#;

/// The derivative rule this estate carried before `31be31f82`.
///
/// Algebraically `da` when `takes_left` is one, and zero in `f64` whenever
/// `|db|` is large enough that `fl(da - db) == -db`. Reproduced literally so the
/// fixture below tests the criterion against a defect that was real rather than
/// against a number chosen to fail.
fn blended_max_derivative(da: f64, db: f64, takes_left: f64) -> f64 {
    db + (da - db) * takes_left
}

/// The double-double criterion must still reject a wrong derivative.
///
/// # Why this is the test that keeps the criterion honest
///
/// Asking the question in a wider precision is only the right move if it is the
/// *same* question. A criterion that admitted more would be a fitted tolerance
/// wearing a derivation, so the thing to demonstrate is that the one defect
/// this estate actually had is still orders outside it.
///
/// The defect is the pre-`31be31f82` `max` rule, `db + (da − db)·c`, which is
/// algebraically `da` when the mask is one and *exactly zero* in `f64` whenever
/// `|db|` is large enough that `fl(da − db) == −db`.
///
/// # What the wider precision changes about this fixture, and why
///
/// It changes which of the two criteria catches it, and that is worth stating
/// rather than hiding, because it says what each criterion is for. The blend is
/// `da` in double-double — `da − db` is exact there, so the addition returns
/// what the subtraction took away — so the *agreement* criterion passes it, and
/// passes it correctly: the two chain rules do compute the same real
/// derivative. What the blend does not do is evaluate that derivative in `f64`,
/// and the measurement says so exactly: `E_mir = |0 − da| / |da| = 1`, an
/// evaluation that differs from the quantity it computes by the whole of it.
///
/// That is [`Comparison::lost_significance`], and it is the sharper statement
/// of the two. The old `f64` criterion failed this fixture for "the two chain
/// rules disagree", which was not true; this one fails it for "the shipped
/// route's evaluation of this entry has no correct digit", which is what
/// happened.
#[test]
fn the_criterion_rejects_a_mis_differentiated_max() {
    use crate::canonical_ir::{AdSeed, CfgEvalInputs};

    const GA: f64 = 1.0e-9;
    const GB: f64 = 1.0e9;
    const V0: f64 = 0.5;

    let compiler = crate::VerilogACompiler::new(crate::CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir(MAX_AGAINST_A_STEEP_LOSER)
        .expect("compile canonical IR");

    let mut cfg = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir)
        .expect("lower the CFG");
    let _ = stored_charges(&mut cfg.function, &cfg.residuals);
    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let mut differentiated = differentiate(&cfg.function, &seeds).expect("differentiate");
    let row = differentiated.derivative_row(cfg.residuals[0]);
    let scalarized = scalarize_lanes(&differentiated.function).expect("scalarize");

    let lane = seeds
        .iter()
        .position(|seed| matches!(seed, AdSeed::NodePotential(node) if usize::from(*node) == 0))
        .expect("a lane for the p terminal");
    assert_ne!(Some(lane), correction_lane);
    let output = row[lane].expect("d(residual)/d(V(p)) exists");
    let scalar = scalarized.scalar(output).expect("a scalar lane");
    let (pruned, mapped) = prune_cfg_to_outputs(&scalarized.function, &[scalar]);

    let mut node_potentials = vec![V0, 0.0];
    node_potentials.resize(artifact.mir.nodes.len(), 0.0);
    let parameters: Vec<f64> = artifact
        .mir
        .parameters
        .iter()
        .map(|parameter| parameter.default.unwrap_or(0.0))
        .collect();
    let inputs = CfgEvalInputs::<f64> {
        parameter_given: vec![true; parameters.len()],
        parameters,
        port_connected: vec![true; 2],
        node_potentials,
        branch_flows: vec![0.0; artifact.mir.branches.len()],
        temperature: 300.15,
        thermal_voltage: 1.380_649e-23 / 1.602_176_634e-19 * 300.15,
        multiplicity: 1.0,
        ..Default::default()
    };

    let snapshot = evaluate_cfg(&pruned, &lift_inputs(&inputs))
        .expect("the reference walks the cone in double-double");
    let reference = snapshot.value(mapped[0]).expect("the entry has a value");

    // The interpreter takes the winning arm, so the derivative is `ga`.
    assert!(
        (reference.to_f64() / GA - 1.0).abs() < 1.0e-12,
        "the fixture has to select the shallow arm: {:e}",
        reference.to_f64()
    );
    // And the reference is `ga` *exactly*: every step of a masked selection is
    // exact, so the double-double walk and the `f64` walk agree to the last
    // bit however large the losing arm's derivative is.
    assert_eq!(
        reference.relative_distance_to(GA),
        0.0,
        "a masked selection commits no rounding: {:.17e}",
        reference.to_f64()
    );

    // The defect, in the arithmetic it was written in.
    let defect = blended_max_derivative(GA, GB, 1.0);
    assert_eq!(
        defect, 0.0,
        "the blend has to actually lose the winner's derivative, or this proves nothing"
    );

    // The blend's own reference, in the arithmetic the blend is written in.
    // It is `da`, not zero: the subtraction that loses everything in `f64` is
    // exact in double-double, so the addition gives back what it took.
    let defect_reference =
        DoubleDouble::from_f64(GB).add(DoubleDouble::from_f64(GA).sub(DoubleDouble::from_f64(GB)));
    assert_eq!(
        defect_reference.relative_distance_to(GA),
        0.0,
        "the blend is algebraically right, which is what makes this the sharper test"
    );

    let comparison = Comparison {
        entry: CfgPlanEntry::Jacobian(0, 0),
        point: 0,
        mir: defect,
        cfg: reference.to_f64(),
        operations: pruned.values.len(),
        deviation: deviation(defect, reference.to_f64()).unwrap_or(0.0),
        mir_dd: Some(defect_reference),
        cfg_dd: Some(reference),
    };
    println!("cfg-mir masked-max fixture: {}", comparison.describe());
    assert_eq!(
        comparison.criterion(),
        "double-double",
        "the fixture has to be judged in the wider precision: {}",
        comparison.describe()
    );
    // The agreement criterion passes, and correctly: the two rules compute one
    // derivative.
    assert_eq!(comparison.reference_deviation(), Some(0.0));
    assert!(comparison.judged_deviation() <= comparison.bound());
    // The measurement is what rejects it, and it rejects it at the definition
    // of "no significant figure" rather than at a number.
    assert_eq!(comparison.mir_error(), Some(1.0));
    assert_eq!(comparison.cfg_error(), Some(0.0));
    assert!(
        comparison.lost_significance(),
        "the criterion has to reject a zeroed derivative: {}",
        comparison.describe()
    );
    // And the `f64` deviation the census used to judge on is the same one:
    // nine orders, entirely explained by the shipped route's own rounding.
    assert_eq!(comparison.deviation, 1.0);

    // The same fixture with the correct rule agrees with itself and loses
    // nothing, so what the assertions above reject is the defect and not the
    // fixture.
    let agreeing = Comparison {
        mir: reference.to_f64(),
        mir_dd: Some(reference),
        deviation: 0.0,
        ..comparison
    };
    assert_eq!(agreeing.judged_deviation(), 0.0);
    assert!(agreeing.judged_deviation() <= agreeing.bound());
    assert!(!agreeing.lost_significance());
    assert_eq!(agreeing.mir_error(), Some(0.0));
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
        mir_dd: None,
        cfg_dd: None,
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
