//! The CFG route's model plan, and the plan production compiles.
//!
//! [`build_model_plan_with_canonical_ir`] lowers every value entry from MIR's
//! flat postfix stream. This is the second constructor: the same
//! [`NativeModelPlan`], with `stamp_values`, `jacobians`, `reactive_jacobians`
//! and — when [`CfgNoiseScope::Cfg`] is asked for — `noise_psd` and
//! `noise_exponents` lowered instead from the canonical CFG through
//! [`lower_cfg_function`] and [`scalarize_lanes`], adopted as
//! [`PlanProgram::Blocks`].
//!
//! # The plan has an assignment pass of its own
//!
//! Every `stamp_values`, `jacobians` and `reactive_jacobians` entry used to be
//! its own pruned dependence cone of one function, and the cones overlapped
//! almost completely: Σ cone / union ran from 24 on asmesd to 383 on asmhemt,
//! and the fused stamp kernel inlined every one of them a second time.
//! bsimcmg_va compiled to 417 MB against the postfix route's 4.8 MB — past the
//! image limit, which is raised *after* the whole-module fallback point, so it
//! was a model production could not construct.
//!
//! So the builder gathers every entry of every stamp first, builds one
//! [`CfgPrelude`] over them — one prune, one lowering, one slot per distinct
//! output value — and each entry becomes the single `LoadPreludeSlot` that
//! reads what the prelude published. The plan carries it as
//! [`NativeModelPlan::prelude`] and every backend runs it once per evaluation,
//! after the assignment pass whose variables it reads and before the first
//! entry that reads one of its slots.
//!
//! Two classes keep their own cone. A value that reads a contribution current
//! is ordered against the residuals before it and the prelude runs before all
//! of them ([`LiveCurrentTaint`]); and the noise magnitudes below lower from
//! the *primal* body and from hoisted copies of it, which are not the function
//! the prelude is built over. Noise is tens of entries, not thousands, so it
//! was never part of the explosion.
//!
//! [`build_default_model_plan`] is what x64, AArch64 and the WASM JIT all call,
//! and it builds **this** plan: the CFG form of a module's residual, Jacobian,
//! reactive-Jacobian and noise entries, every other field postfix, and the
//! shipped postfix plan entire for a module the CFG route refuses.
//! [`build_model_plan_with_canonical_ir`] is no longer a route production
//! selects; it is that fallback, and the reference side of
//! [`crate::native::cfg_mir_census`].
//!
//! There is no route switch. It was a constant read at one call site while the
//! evidence was being taken, and once production takes the CFG plan a two-armed
//! selector has one arm nothing constructs. See [`build_default_model_plan`]
//! for the evidence the flip stands on.
//!
//! # What stays postfix, and why that is not a half measure
//!
//! `parameter_defaults`, `static_conditions` and both assignment passes keep
//! their MIR lowering. A parameter default is evaluated once, before any body
//! runs, from a MIR expression the CFG does not carry; the assignment passes
//! write the runtime variable slots the *postfix* value entries read, and the
//! CFG route recomputes those values inline instead. Leaving them alone is
//! therefore what makes the two plans comparable at all: both fill `variables`
//! by the same code, and the value entries then have to agree about what that
//! operating point evaluates to.
//!
//! # Noise comes from the CFG too, and what that costs
//!
//! Production asks for [`CfgNoiseScope::Cfg`], which is the scope the
//! CFG-versus-MIR census measures: a plan production built under
//! [`CfgNoiseScope::Postfix`] would be a third plan neither census walked.
//!
//! It is the expensive half of the flip and the numbers say so.
//! [`crate::native::cfg_cost_census`] times the shipped plan, the CFG route
//! with the noise magnitudes left postfix, and the CFG route entire, at one
//! bias in one process. The route alone is free — 1.00, 1.05, 0.95, 0.97 and
//! 0.89 times the shipped plan's evaluation on `asmesd`, `vbic13_4t`,
//! `hicumL2va`, `bsimcmg_va` and `asmhemt`, and images within a few per cent of
//! [`crate::native::cfg_size_census`]'s. Taking noise from the CFG as well
//! costs 1.14, 1.65, 1.67, 1.30 and 0.95, and takes `hicumL2va`'s image from
//! 1.27 MB to 3.08 and `bsimcmg_va`'s from 5.62 to 9.81.
//!
//! Where it comes from is the paragraph above about the prelude: the noise
//! magnitudes lower from the primal body and from hoisted copies of it, which
//! are not the function the prelude is built over, so every one of them keeps
//! its own cone. "Noise is tens of entries, not thousands, so it was never part
//! of the explosion" was a count of entries; what the cost census measures is
//! that those tens are cones over the deepest part of a compact model's body,
//! and on `hicumL2va` and `bsimcmg_va` they are most of the image.
//!
//! Sharing them with the prelude is the work that would close it, and
//! [`NoiseMagnitude`] names the analysis it needs. Whether the flip should have
//! waited for that is a ruling about noise, not about this route.
//!
//! ## What the gap was, and what it actually is
//!
//! This module used to say the two routes held different quantities because a
//! *grouped* noise process folds its routing amplitude into the injection
//! instead of into the PSD. That is not the difference, and the corpus says so
//! twice over. `noise_process_schema` is written as `1` by every compiler this
//! crate has — `tests/support` states it outright — so "thirty-four of
//! forty-three carry schema 1" was counting the modules that have any noise
//! source at all, not a grouping. And `crate::native::cfg_noise_pins` compiles
//! one module carrying each shape and reads both routes at one bias: a plain
//! white source, one scaled by a factor, a flicker exponent and a process
//! routed into two equations agree **bit for bit**. The amplitude was never the
//! problem.
//!
//! The difference is the **activation guard**. A `CfgNoiseProcess`'s `psd` and
//! `exponent` are read at the exit block, so a source the body did not reach
//! reads back the zero its variables were seeded with; the shipped
//! `psd_program` is the magnitude expression and the runtime evaluates it
//! unconditionally. `angelov`'s `flicker_noise(NoisePwrG, 2, "gate")` sits
//! under `case (Noimod) 1:` with `Noimod` defaulting to `0`, which is why its
//! `noise_exponents[9]` read 2.0 through MIR and 0.0 through the CFG.
//! `bsimsoi_va`'s `noise_psd[1]` is the same shape.
//!
//! ## What closes it, and what is left
//!
//! The lowering now publishes each magnitude twice — merged at the exit, and as
//! the site itself computed it — and this builder takes whichever one the
//! shipped plan holds, read off the shipped program rather than guessed. On
//! `angelov` and `angelov_gan` that makes every noise entry read the same
//! double on both routes, thirty comparisons that were not exact before.
//!
//! What it does not close is the magnitude whose operands reach the shipped
//! program through a *variable* assigned outside the guard the source sits
//! under: `bsimsoi_va`'s `noise_psd[1]` is the case, and [`NoiseMagnitude`]
//! names what closing it needs. The census asserts that class rather than
//! measuring it, which is the part that matters for the flip.
//!
//! # All or nothing, per module
//!
//! Every refusal is per *module*, never per entry. A plan that took the CFG
//! form for the entries it could lower and the postfix form for the rest would
//! be a third thing neither census measured, and its evidence would say nothing
//! about either route. So a construct the CFG route will not lower, a shipped
//! Jacobian column the derivative pass numbers differently, or a shipped noise
//! source with no CFG process all refuse the whole module by name — see
//! [`CfgPlanRefusal`] for the list.
//!
//! What is *not* a refusal is a noise magnitude whose site value could not be
//! hoisted out of its guard. That entry keeps the exit read — the quantity this
//! route has always lowered — and is counted
//! ([`CfgPlanReport::noise_guarded_reads`]) for the census to hold to its
//! weaker criterion. Refusing it instead would take a module's residual and
//! Jacobian entries down over one noise entry nothing has shown to be wrong.
//!
//! # The fallback is loud
//!
//! A refused module still compiles: [`build_default_model_plan`] returns the
//! postfix plan for it, which is the plan every backend shipped before the flip,
//! so a refusal costs accuracy nowhere and coverage nothing. What it must not do
//! is happen quietly — a module that stops taking the CFG route because a pass
//! regressed would otherwise look exactly like one that never took it. So the
//! refusal goes to the same `[JIT]` seam a failed native compile uses, naming
//! the module and the refusal class, and [`build_default_model_plan_reported`]
//! hands it back so a test can pin it. Running the estate therefore censuses the
//! fallback by class, and across the shipped forty-three it names two modules:
//! `mvsg_cmc` and the `BSIM_SOI_100.1.1` `bsimsoi`, the W-D
//! `Ddt`-under-condition class. They are also the only two whose machine code
//! [`crate::native::code_identity`] measures unmoved across the flip, which is
//! the same fact read from the other end.
//!
//! # The one place a program is built rather than lowered
//!
//! The shipped planner decides Jacobian sparsity with a conservative structural
//! reachability mask and then drops whatever its simplifier folds to a literal
//! zero. Liveness on the CFG is exact, so it drops entries the shipped planner
//! keeps: the shipped plan carries a program that always returns zero and the
//! CFG route carries no value at all. The plan shape is the shipped one — a
//! backend indexes `jacobians[stamp][entry]` positionally — so those entries
//! need a program, and the program the CFG route's own analysis implies is the
//! constant zero. They are counted ([`CfgPlanReport::structural_zeros`]) and the
//! census checks each one against what the shipped entry actually evaluates to,
//! because "the shipped program is identically zero" is a claim about the
//! corpus and not a theorem.

use std::collections::{HashMap, HashSet};

use super::cfg_lanes::scalarize_lanes;
use super::cfg_prelude::{CfgPrelude, LiveCurrentTaint};
use super::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use super::expr::NativeOp;
use super::model_plan::{NativeModelPlan, NativePrelude};
use super::plan_builder::{
    build_model_plan_with_canonical_ir, canonical_branch_unknown_runtime_map,
};
use super::plan_program::{BlockProgram, PlanProgram, PlanProgramRef};
use super::ssa::{BlockId, BuilderTerminator, Program, ProgramBuilder, ValueType};
use super::{JitError, JitResult};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::cfg_lower::CfgNoiseProcess;
use crate::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction,
    CfgStateAllocation, CfgTerminator, CfgValueKind, MirModel, ValueId, differentiate,
    prune_cfg_to_outputs,
};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::codegen::{ColumnAxis, CompiledModel};
use crate::rust_backend::canonical::stored_charges;

/// Why the CFG route cannot build a plan for a module.
///
/// Named rather than a string so the census can tabulate refusals by class and
/// a reader can tell "this construct is not lowered yet" from "these two routes
/// disagree about what the model is".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CfgPlanRefusal {
    /// The shipped postfix plan, which this one is built on top of, refused the
    /// module. Nothing about the CFG route is implicated.
    ShippedPlan,
    /// The HIR body does not lower to a CFG at all.
    CfgLowering,
    /// The CFG's analog-operator sites cannot be resolved onto the runtime's
    /// state records. This is where [`crate::canonical_ir::HirExecutedCorrespondence`]
    /// refuses a site it does not cover — a `case` arm's condition, a prologue
    /// statement, a named block's local initializer.
    StateAllocation,
    /// The derivative pass refused the body.
    Differentiate,
    /// The body contains an operation the CFG derivative pass has no rule for.
    ///
    /// Checked *before* differentiating rather than reported by it, because the
    /// pass does not report it: `binary_factor`'s scalar fallthrough in
    /// [`crate::canonical_ir`]'s `ad` module is a `debug_assert!` over
    /// `is_predicate` and a `None` result, so a hole there panics a debug build
    /// and silently yields a zero derivative in a release one. A zero Jacobian
    /// entry that should not be zero is precisely the silently-wrong class this
    /// program refuses, and taking the postfix plan for the module is the only
    /// answer available until the scalar rules exist.
    ///
    /// See [`DERIVATIVE_RULE_HOLES`] for the list and the rules that empty it.
    DerivativeRuleMissing,
    /// The lane scalarizer refused the differentiated body.
    Scalarize,
    /// A value the plan needs has no scalar after lane scalarization.
    NoScalar,
    /// [`lower_cfg_function`] refused an entry: the W-C refusal classes.
    Lowering,
    /// The shipped model and the CFG disagree about how many equations the
    /// module has, so no entry can be paired with the residual it belongs to.
    EquationsUnpaired,
    /// A shipped Jacobian column names a lane outside the derivative pass's
    /// seed list, so the two routes number the unknowns differently.
    LaneUnmapped,
    /// A stamp has reactive Jacobian entries but the CFG route found no stored
    /// charge for its equation.
    ChargeMissing,
    /// A shipped noise source has no CFG process with its id.
    NoiseUnpaired,
    /// A block program addresses a state slot the module's
    /// [`StateSlotMapping`] never allocated.
    SlotUnclaimed,
    /// The prelude's union cone reads a contribution current the evaluation
    /// publishes after the prelude runs.
    ///
    /// The one refusal that does *not* take the module down: the plan is built
    /// without a prelude and every entry keeps the cone it has always had, so
    /// this is a size refusal rather than a correctness one. It reaches the
    /// caller only through [`CfgPrelude::build`]; the plan builder catches it
    /// by class and records it in [`CfgPlanReport::prelude_live_current`].
    ///
    /// A value read through a `ContributedCurrent` leaf is normally kept off the
    /// prelude entry by entry ([`LiveCurrentTaint`]). What this names is the
    /// case that cannot be: a *branch condition* on the union's own path reads
    /// one, and a cone carries every condition of every surviving block.
    PreludeLiveCurrent,
}

impl CfgPlanRefusal {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::ShippedPlan => "shipped-plan",
            Self::CfgLowering => "cfg-lowering",
            Self::StateAllocation => "state-allocation",
            Self::Differentiate => "differentiate",
            Self::DerivativeRuleMissing => "derivative-rule-missing",
            Self::Scalarize => "scalarize",
            Self::NoScalar => "no-scalar",
            Self::Lowering => "lowering",
            Self::EquationsUnpaired => "equations-unpaired",
            Self::LaneUnmapped => "lane-unmapped",
            Self::ChargeMissing => "charge-missing",
            Self::NoiseUnpaired => "noise-unpaired",
            Self::SlotUnclaimed => "slot-unclaimed",
            Self::PreludeLiveCurrent => "prelude-live-current",
        }
    }
}

/// A module the CFG route will not build a plan for, with the detail the
/// refusing pass gave.
#[derive(Debug, Clone)]
pub(crate) struct CfgPlanRefused {
    pub(crate) module: String,
    pub(crate) class: CfgPlanRefusal,
    pub(crate) detail: String,
}

impl std::fmt::Display for CfgPlanRefused {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "the CFG route refuses {}: {}: {}",
            self.module,
            self.class.name(),
            self.detail
        )
    }
}

impl From<CfgPlanRefused> for JitError {
    fn from(refused: CfgPlanRefused) -> Self {
        Self::unsupported_native_coverage(
            refused.module.clone(),
            format!(
                "CFG-built plan, {}: {}",
                refused.class.name(),
                refused.detail
            ),
        )
    }
}

/// What the CFG route did for one module, in the terms the flip is judged on.
#[derive(Debug, Clone, Default)]
pub(crate) struct CfgPlanReport {
    pub(crate) stamp_values: usize,
    pub(crate) jacobians: usize,
    pub(crate) reactive_jacobians: usize,
    pub(crate) noise_values: usize,
    /// Noise magnitudes taken at their own site rather than at the exit block,
    /// so that they hold the unconditional quantity the shipped route holds.
    /// See [`NoiseMagnitude`].
    pub(crate) noise_hoisted: usize,
    /// Noise magnitudes lowered from the exit read while the source is under a
    /// guard — the class where the two routes can still hold different numbers.
    ///
    /// It is not "different by design": where the shipped magnitude is a
    /// `LoadVariable`, the slot the guarded noise assignment pass wrote holds
    /// the same zero the exit read does, and the two agree. What this names is
    /// the entries where that is not established by construction, and it is
    /// what the census asserts the weaker of its two noise criteria on. Keyed
    /// like [`Self::structural_zeros`], so the census names each one.
    pub(crate) noise_guarded_reads: Vec<CfgPlanEntry>,
    /// Shipped Jacobian and reactive-Jacobian entries the CFG route's liveness
    /// found no value for, given the constant zero its analysis implies. Keyed
    /// so the census can name each one it checks.
    pub(crate) structural_zeros: Vec<CfgPlanEntry>,
    /// Noise magnitudes that only lowered after the derivative pass resolved a
    /// `ddx` inside them — the shipped emitter's own two-pass rule, kept.
    pub(crate) noise_from_differentiated: usize,
    /// Instructions in the largest block entry the plan *lowered as a cone*,
    /// with its identity.
    ///
    /// Since the prelude, that is a small population: every entry the prelude
    /// publishes is one instruction, so what this measures is the noise
    /// magnitudes and the entries [`Self::live_current_entries`] counts. `None`
    /// on a module where the prelude covers everything and noise stays postfix.
    pub(crate) largest_entry: Option<(CfgPlanEntry, usize)>,
    /// `f64` slots the plan's prelude publishes, one per distinct entry output.
    pub(crate) prelude_slots: usize,
    /// Instructions in the prelude, which is the numerator the size census
    /// used to spend once per entry.
    pub(crate) prelude_instructions: usize,
    /// Entries kept out of the prelude because their value reads a contribution
    /// current the evaluation publishes after the prelude runs. Each keeps its
    /// own cone; see [`LiveCurrentTaint`].
    pub(crate) live_current_entries: usize,
    /// Whether a branch condition reads one anywhere in the function. On its
    /// own this decides nothing — see [`LiveCurrentTaint`].
    pub(crate) live_current_control_flow: bool,
    /// Why the prelude was dropped, when one could not be built.
    ///
    /// Every entry then keeps its own cone, which is the plan this route built
    /// before there was a prelude — a size regression for that module and
    /// nothing more. `None` means either that a prelude was built or that the
    /// module had no publishable entry to build one from.
    pub(crate) prelude_refused: Option<CfgPlanRefusal>,
}

/// One value entry's position in the plan, for a message that names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CfgPlanEntry {
    StampValue(usize),
    Jacobian(usize, usize),
    ReactiveJacobian(usize, usize),
    NoisePsd(usize),
    NoiseExponent(usize),
}

impl std::fmt::Display for CfgPlanEntry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StampValue(stamp) => write!(formatter, "stamp_values[{stamp}]"),
            Self::Jacobian(stamp, entry) => write!(formatter, "jacobians[{stamp}][{entry}]"),
            Self::ReactiveJacobian(stamp, entry) => {
                write!(formatter, "reactive_jacobians[{stamp}][{entry}]")
            }
            Self::NoisePsd(source) => write!(formatter, "noise_psd[{source}]"),
            Self::NoiseExponent(source) => write!(formatter, "noise_exponents[{source}]"),
        }
    }
}

/// A CFG-built plan and the figures it reports about itself.
#[derive(Debug)]
pub(crate) struct CfgModelPlan {
    pub(crate) plan: NativeModelPlan,
    /// Read by the CFG-versus-MIR census, which is what the figures are for.
    /// [`build_default_model_plan`] takes the plan and drops this.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) report: CfgPlanReport,
}

/// Which derivative lane a shipped Jacobian column stands for.
///
/// A node column is its own lane, which is the inverse of the seed-list
/// construction below and the same arithmetic the generated backend's `emit_row`
/// performs when it splits a lane index back into a node column and a branch
/// column. A *branch* column is not, and that is what this type exists for.
///
/// # Why a branch column is not `node_count + ordinal`
///
/// MIR mints one branch-current unknown per potential or indirect **equation**
/// (`collect_branch_unknowns`), so a branch contributed to more than once
/// carries more than one. Angelov writes its optional gate resistance as five
/// contributions to `V(g,gi)` — the resistance, the inductance, the thermal
/// noise, the inductance again in the `Rg == 0` arm, and the short in the arm
/// where both are zero — and MIR gives that one branch unknowns 5 through 9.
///
/// The solver does not work that way: a branch has one flow, so the shipped
/// route gives the whole branch a single `branch_sources` entry and
/// [`ColumnAxis::Branch`] numbers *those*. The CFG route agrees with the solver
/// — `CfgLowerer::branch_unknown_by_nodes` reads a branch's flow as the first
/// MIR unknown on it — so the derivative row is populated at that leader's lane
/// and nowhere else on the branch.
///
/// Adding `node_count` to a shipped branch ordinal therefore names the right
/// lane only while no branch is contributed to twice, and names *a different
/// branch's* unknown as soon as one is. Angelov measured it: stamp 29 is the
/// first equation of `V(si,sii)`, whose solver column is 6, and lane
/// `16 + 6 = 22` is MIR unknown 6 — one of the five on `V(g,gi)`. The residual
/// has no derivative there, so the entry was taken for a structural zero and
/// lowered to constant zero while the shipped route stamped `Rs_T` = 0.05 into
/// the matrix.
pub(crate) struct ShippedColumnLanes {
    /// Indexed by the shipped route's branch ordinal: the lane of the MIR
    /// branch unknown whose flow that solver column is, or `None` for an
    /// ordinal no MIR unknown claims.
    branch_lanes: Vec<Option<usize>>,
}

impl ShippedColumnLanes {
    /// Built from the same endpoint matching the runtime binding uses, so the
    /// two agree about which solver branch an unknown belongs to by
    /// construction rather than by a second rule that could drift.
    ///
    /// The leader is the first MIR unknown on the branch, in MIR order, which
    /// is the one `branch_unknown_by_nodes` picks. An `inverted` leader would
    /// mean the solver source runs against the equation that minted it; the
    /// derivative would then need a sign the lane cannot carry, so no lane is
    /// recorded and the module is refused rather than lowered with a flipped
    /// column.
    pub(crate) fn build(model: &CompiledModel, mir: &MirModel) -> JitResult<Self> {
        let mut branch_lanes = vec![None; model.branch_sources.len()];
        for (index, mapping) in canonical_branch_unknown_runtime_map(model, mir)?
            .into_iter()
            .enumerate()
        {
            if mapping.inverted {
                continue;
            }
            if let Some(slot) = branch_lanes.get_mut(mapping.runtime_index) {
                slot.get_or_insert(mir.nodes.len() + index);
            }
        }
        Ok(Self { branch_lanes })
    }

    /// `None` for a branch column no MIR unknown leads, which the plan builder
    /// refuses on and the sparsity censuses drop: a column with no lane is a
    /// position neither route can hold.
    pub(crate) fn lane(&self, axis: &ColumnAxis) -> Option<usize> {
        match axis {
            ColumnAxis::Node(node) => Some(*node),
            ColumnAxis::Branch(branch) => self.branch_lanes.get(*branch).copied().flatten(),
        }
    }
}

/// The seed list, in the order every consumer of the derivative pass already
/// uses: node potentials, then branch unknowns, then the limiter correction if
/// the model limits at all.
///
/// Read off the generated-Rust backend rather than invented here, because it is
/// the one shipped consumer of CFG-level AD and the lane index means "unknown
/// number n" only if both agree. Its own words for putting the correction last:
/// "a model without `$limit` carries no lane for it and every other lane index
/// still means 'unknown number n'".
pub(crate) fn derivative_seeds(cfg: &CfgModel, mir: &MirModel) -> (Vec<AdSeed>, Option<usize>) {
    let limits = cfg
        .function
        .values
        .iter()
        .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }));
    let seeds: Vec<AdSeed> = (0..mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain((0..mir.branch_unknowns.len()).map(|index| AdSeed::BranchUnknownFlow(index.into())))
        .chain(limits.then_some(AdSeed::LimiterCorrection))
        .collect();
    let correction = limits.then(|| seeds.len() - 1);
    (seeds, correction)
}

/// The binary operations the CFG derivative pass's *scalar* rules omit.
///
/// The pass has two rule sets. Its lane rules carry both of these already, and
/// carry them correctly:
///
/// ```text
/// d hypot(x, y) = (x·dx + y·dy) / hypot(x, y)
/// d atan2(y, x) = (x·dy − y·dx) / (x² + y²)
/// ```
///
/// Its scalar rules do not, and say so only in a debug build: `binary_factor`'s
/// fallthrough asserts the operation is a predicate and otherwise returns
/// `None`, which a release build reads as "the derivative is zero". The plan
/// route reaches the scalar rules, so a `ddx` over either operation compiles to
/// a Jacobian entry that is wrong rather than absent.
///
/// So this list exists to keep a wrong Jacobian out of a shipped plan, not to
/// describe a limitation anybody intends to keep: the two rules are already
/// written a thousand lines further down the same file, and the scalar cases
/// are the same algebra without the lane plumbing. Adding them empties this
/// list, and [`a_module_the_derivative_pass_has_no_rule_for_falls_back`] is what
/// notices — it fails by *building* the module, which is the day the list and
/// the refusal class both come out.
///
/// No shipped model reaches this — a search of the forty-three-module tree
/// finds neither operation — so the generated-Rust backend, which runs the same
/// pass, emits nothing that depends on the hole today.
const DERIVATIVE_RULE_HOLES: &[CfgBinaryOp] = &[CfgBinaryOp::Hypot, CfgBinaryOp::Atan2];

/// The first operation of `function` the derivative pass has no rule for.
fn derivative_rule_hole(function: &CfgFunction) -> Option<CfgBinaryOp> {
    function.values.iter().find_map(|value| match value.kind {
        CfgValueKind::Binary { op, .. }
        | CfgValueKind::LaneBinary { op, .. }
        | CfgValueKind::LaneScalar { op, .. }
            if DERIVATIVE_RULE_HOLES.contains(&op) =>
        {
            Some(op)
        }
        _ => None,
    })
}

/// Which of a noise process's two magnitudes the plan entry holds.
///
/// # The measurement this exists for
///
/// A `CfgNoiseProcess` publishes each magnitude twice: read at the exit block,
/// where the control flow that reached the source has been merged into it, and
/// as [`CfgNoiseProcessSite`](crate::canonical_ir::cfg_lower::CfgNoiseProcessSite)
/// — the value the site itself computed, with no merge.
///
/// The shipped plan holds neither one consistently, and that is not a reading
/// of the code but of the corpus. `crate::native::cfg_noise_pins` compiles one
/// module carrying each shape and reads both routes at one bias: an unguarded
/// white source, one scaled by a factor, a flicker exponent, and a process
/// routed into two equations all agree *bit for bit* between the two routes
/// today. Only a source under a guard disagrees, and there the shipped
/// behaviour splits:
///
/// * where the front end wrote the magnitude operand inline — a literal, a
///   parameter expression — the shipped `psd_program` is that expression and
///   the runtime evaluates it **unconditionally**. `angelov`'s `flicker_noise(
///   NoisePwrG, 2, "gate")` sits under `case (Noimod) 1:` with `Noimod`
///   defaulting to 0, and its shipped exponent still reads 2.0 while the exit
///   read is the seeded zero. The site value is the quantity to take.
/// * where the operand reaches the program through a variable, the shipped
///   magnitude is that *slot*, filled by the noise assignment pass — and the
///   CFG route recomputes a variable inline from the definition reaching the
///   read, which is a different thing under a guard. Hoisting there would put a
///   recomputed value against a slot read and disagree in the other direction,
///   so the exit read is taken instead.
///
/// So the shipped route is not self-consistent about the guard: whether it
/// applies is decided by whether a variable happened to carry the operand. This
/// reads that decision off the shipped program itself, which is the only place
/// it is recorded, rather than guessing it from the source.
///
/// # What is left, named
///
/// Taking the exit read for the variable-carried case is a *conservative*
/// choice and not a proof that the two agree there. It is exact when the
/// variable is assigned inside the same guard — the slot then holds the zero
/// the exit read holds — and `bsimsoi_va` is the case where it is not:
/// `noise_psd[1]`'s operands are assigned outside the guard the source sits
/// under, so the shipped program evaluates them to `7.19e-28` while the exit
/// read is the seeded zero. Its cfg-mir line says `noise_hoisted=0
/// guarded_reads=10`.
///
/// Closing that needs the CFG's noise cone to read a variable as the runtime
/// *slot* rather than as its reaching definition — the `frozen_event_state`
/// split generalized from event-controlled variables to every variable a noise
/// magnitude reads — which is a new `CfgValueKind` and a rule for it in the
/// derivative pass, the interpreter and the block lowering. Until then the
/// census holds the class to its second criterion, which is what makes the
/// difference harmless rather than merely unmeasured: see `check_guarded_noise`
/// in [`crate::native::cfg_mir_census`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoiseMagnitude {
    /// The value the site computed, evaluated unconditionally.
    Site,
    /// The value read at the exit block, with the activation guard folded in.
    Exit,
}

fn noise_magnitude_source(plan: &NativeModelPlan, entry: CfgPlanEntry) -> NoiseMagnitude {
    let program = match entry {
        CfgPlanEntry::NoisePsd(source) => plan.noise_psd.get(source),
        CfgPlanEntry::NoiseExponent(source) => {
            plan.noise_exponents.get(source).and_then(Option::as_ref)
        }
        _ => None,
    };
    // Anything but a postfix program is a shape this route did not build, so
    // the exit read — what it has always lowered — is the conservative answer.
    let Some(PlanProgram::Postfix(program)) = program else {
        return NoiseMagnitude::Exit;
    };
    if program.ops().iter().any(|op| {
        matches!(
            op,
            NativeOp::LoadVariable(_) | NativeOp::LoadVariableDyn { .. }
        )
    }) {
        NoiseMagnitude::Exit
    } else {
        NoiseMagnitude::Site
    }
}

/// Whether the body reaches this process on every path.
///
/// The lowering writes a process's activation as the constant one at its site
/// and seeds it with zero in the entry block, so a source no control flow
/// guards has the constant one and nothing else does.
fn process_is_unconditional(function: &CfgFunction, active: ValueId) -> bool {
    matches!(
        function.value(active).kind,
        CfgValueKind::RealConstant(value) if value == 1.0
    )
}

/// A straight-line copy of `function` carrying every value `roots` reads.
///
/// A site value is defined in the block the source was written in, which a
/// guard may keep off the path to the exit. [`lower_cfg_function`] returns the
/// value its exit block holds, so it cannot be asked for one defined under a
/// branch. This builds the function that *can* be asked: one block, the cones
/// of `roots` in an order that defines each value before it is read, and a
/// `Return`.
///
/// Copying the whole value table keeps every id valid, so a caller holds on to
/// the roots it already had; the values no root reads are simply pinned to no
/// block and never emitted.
///
/// # What it will not hoist, and why that is not a refusal
///
/// A cone containing a block parameter — a merge of two paths — is exactly the
/// control flow this is trying to leave behind, and a single block cannot carry
/// it. Such a root is left out and its entry falls back to the exit read, which
/// is what this route lowered before the site values existed: a magnitude the
/// two routes may disagree about, never a wrong one. Refusing the module
/// instead would take its residual and Jacobian entries down over a noise entry
/// nothing has yet shown to be wrong.
struct HoistedSites {
    function: CfgFunction,
    pinned: HashSet<ValueId>,
}

impl HoistedSites {
    /// Whether the single block defines `value`, or `value` is a leaf it may
    /// read. Either way the function can be asked for it.
    fn pins(&self, value: ValueId) -> bool {
        self.pinned.contains(&value)
    }
}

fn hoisted_site_function(function: &CfgFunction, roots: &[ValueId]) -> Option<HoistedSites> {
    if roots.is_empty() {
        return None;
    }
    // Which values the original function defines by an instruction. Everything
    // else a cone reaches is a leaf, which the block model materializes in the
    // entry block itself.
    let mut instruction: HashSet<ValueId> = HashSet::new();
    for block in &function.blocks {
        for value in &block.instructions {
            instruction.insert(value.result);
        }
    }

    let mut order: Vec<ValueId> = Vec::new();
    let mut placed: HashSet<ValueId> = HashSet::new();
    let mut pinned: HashSet<ValueId> = HashSet::new();
    for root in roots {
        // Post-order, iteratively: a compact model's cone is deep enough that
        // recursion here is a stack-overflow waiting for one more model.
        let mut stack = vec![(*root, false)];
        let mut cone = Vec::new();
        let mut hoistable = true;
        let mut seen: HashSet<ValueId> = HashSet::new();
        while let Some((value, expanded)) = stack.pop() {
            if expanded {
                if instruction.contains(&value) && !placed.contains(&value) {
                    cone.push(value);
                }
                continue;
            }
            if !seen.insert(value) {
                continue;
            }
            if matches!(function.value(value).kind, CfgValueKind::BlockParameter) {
                hoistable = false;
                break;
            }
            stack.push((value, true));
            for operand in function.value(value).kind.operands() {
                stack.push((operand, false));
            }
        }
        if !hoistable {
            continue;
        }
        for value in cone {
            if placed.insert(value) {
                order.push(value);
            }
        }
        pinned.insert(*root);
    }
    if pinned.is_empty() {
        return None;
    }
    pinned.extend(order.iter().copied());

    let block = CfgBlock {
        id: crate::canonical_ir::BlockId::from(0usize),
        params: Vec::new(),
        instructions: order
            .into_iter()
            .map(|result| CfgInstruction { result })
            .collect(),
        terminator: CfgTerminator::Return,
    };
    Some(HoistedSites {
        function: CfgFunction {
            entry: crate::canonical_ir::BlockId::from(0usize),
            blocks: vec![block],
            values: function.values.clone(),
            shapes: function.shapes.clone(),
        },
        pinned,
    })
}

/// The block program a structurally absent entry gets: one instruction, one
/// block, returning zero.
fn constant_zero_program() -> JitResult<Program> {
    let entry = BlockId::new(0)?;
    let mut builder = ProgramBuilder::new(&[Vec::new()])?;
    builder.begin_block(entry)?;
    let zero = builder.push(NativeOp::Const(0.0), &[], ValueType::F64)?;
    builder.end_block(BuilderTerminator::Return(zero))?;
    builder.finish(entry, entry)
}

/// Where a CFG-built plan's `noise_psd` and `noise_exponents` come from.
///
/// The one field of the plan the two routes are known to disagree about, so it
/// is the one the caller chooses rather than the builder. See the module
/// documentation for the measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CfgNoiseScope {
    /// Lower them from the CFG with everything else. What production asks for,
    /// and what the CFG-versus-MIR census measures.
    Cfg,
    /// Keep the postfix plan's.
    ///
    /// [`CfgPlanRefusal::NoiseUnpaired`] cannot fire under this scope: a
    /// shipped source with no CFG process is not a disagreement about a value
    /// nobody is taking from the CFG.
    ///
    /// Production stopped asking for this at the flip, so outside `cfg(test)`
    /// nothing constructs it and the attribute is the honest statement of that
    /// rather than a silenced warning. What still does construct it is the
    /// measurement that separates the route's cost from the noise scope's —
    /// [`crate::native::cfg_size_census`] and
    /// [`crate::native::cfg_cost_census`] both build this plan to hold the
    /// other one against — and [`crate::native::cfg_noise_pins`], which reads
    /// both scopes at one bias.
    #[cfg_attr(not(test), allow(dead_code))]
    Postfix,
}

/// Build the CFG route's plan for `model`, or say why it cannot be built.
///
/// The postfix plan is built first and kept: it validates the canonical
/// artifact against the compiled model, and its assignment passes, parameter
/// defaults, static conditions and published current pairs are the CFG plan's
/// too. Only the program-bearing value fields `noise` names are replaced.
pub(crate) fn build_model_plan_from_canonical_cfg(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
    noise: CfgNoiseScope,
) -> Result<CfgModelPlan, CfgPlanRefused> {
    let module = model.name.to_string();
    let refuse = |class: CfgPlanRefusal, detail: String| CfgPlanRefused {
        module: module.clone(),
        class,
        detail,
    };

    let mut plan = build_model_plan_with_canonical_ir(model, artifact)
        .map_err(|error| refuse(CfgPlanRefusal::ShippedPlan, error.to_string()))?;

    // The executable lowering, not the generated one: this plan is executed by
    // `VerilogADevice`, which builds instances from whatever terminal list the
    // netlist supplied.
    let mut cfg = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir).map_err(
        |diagnostics| {
            refuse(
                CfgPlanRefusal::CfgLowering,
                diagnostics
                    .first()
                    .map_or_else(|| "unknown".to_string(), |first| first.message.to_string()),
            )
        },
    )?;
    if model.stamp_programs.len() != cfg.residuals.len() {
        return Err(refuse(
            CfgPlanRefusal::EquationsUnpaired,
            format!(
                "{} shipped equations against {} canonical residuals",
                model.stamp_programs.len(),
                cfg.residuals.len()
            ),
        ));
    }

    // Charges first: the extraction *builds* the values a scaled or summed
    // charge needs (`k * ddt(q)` stores `k * q`, which exists nowhere until it
    // is spliced in), so it has to run before the state allocation is read off
    // the function and before anything is differentiated.
    let charges = stored_charges(&mut cfg.function, &cfg.residuals);
    let state = CfgStateAllocation::build(&artifact.hir, &cfg.function).map_err(|errors| {
        refuse(
            CfgPlanRefusal::StateAllocation,
            errors
                .first()
                .map_or_else(|| "unknown".to_string(), ToString::to_string),
        )
    })?;

    if let Some(op) = derivative_rule_hole(&cfg.function) {
        return Err(refuse(
            CfgPlanRefusal::DerivativeRuleMissing,
            format!("the CFG derivative pass has no rule for {op:?}"),
        ));
    }
    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let mut differentiated = differentiate(&cfg.function, &seeds)
        .map_err(|error| refuse(CfgPlanRefusal::Differentiate, format!("{error:?}")))?;
    // Every read-out before anything lowers: taking one appends an instruction,
    // so a function captured earlier would not contain the later ones.
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
    let scalarized = scalarize_lanes(&differentiated.function)
        .map_err(|error| refuse(CfgPlanRefusal::Scalarize, error.to_string()))?;

    let branch_unknowns = canonical_branch_unknown_runtime_map(model, &artifact.mir)
        .map_err(|error| refuse(CfgPlanRefusal::ShippedPlan, error.to_string()))?;
    // The CFG numbers event-controlled state in HIR declaration order; the
    // runtime numbers every variable the model has. Names are what the two
    // spaces share.
    let event_state_variables: Vec<Option<usize>> = artifact
        .hir
        .variables
        .iter()
        .filter(|variable| variable.is_state)
        .map(|variable| {
            model
                .variable_names
                .iter()
                .position(|name| *name == variable.name)
        })
        .collect();
    let bindings = CfgRuntimeBindings::from_mir(
        module.as_str(),
        &artifact.mir,
        branch_unknowns,
        event_state_variables,
    );
    let slots = StateSlotMapping::build(model, &artifact.hir, &artifact.mir);

    let mut report = CfgPlanReport::default();

    // One lowering closure, so every entry reaches the block model by the same
    // three steps: prune to the output, lower, adopt through the module's slot
    // mapping.
    let lower = |function: &CfgFunction,
                 output: ValueId,
                 entry: CfgPlanEntry,
                 report: &mut CfgPlanReport|
     -> Result<PlanProgram, CfgPlanRefused> {
        let (pruned, outputs) = prune_cfg_to_outputs(function, &[output]);
        let program = lower_cfg_function(&pruned, outputs[0], &state, &bindings)
            .map_err(|error| refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}")))?;
        let instructions = program.instructions().len();
        if report
            .largest_entry
            .is_none_or(|(_, largest)| instructions > largest)
        {
            report.largest_entry = Some((entry, instructions));
        }
        let adopted = BlockProgram::adopt(module.as_str(), program, &slots)
            .map_err(|error| refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}")))?;
        Ok(PlanProgram::Blocks(adopted))
    };

    let scalar = |value: ValueId, entry: CfgPlanEntry| -> Result<ValueId, CfgPlanRefused> {
        scalarized
            .scalar(value)
            .ok_or_else(|| refuse(CfgPlanRefusal::NoScalar, entry.to_string()))
    };

    // The lane a shipped column stands for, refusing the module if the two
    // routes have numbered the unknowns differently. The limiter correction is
    // a displacement rather than a coordinate, so a shipped column landing on
    // it would mean the same thing.
    let column_lanes = ShippedColumnLanes::build(model, &artifact.mir)
        .map_err(|error| refuse(CfgPlanRefusal::LaneUnmapped, error.to_string()))?;
    let require_lane = |axis: &ColumnAxis, entry: CfgPlanEntry| -> Result<usize, CfgPlanRefused> {
        let Some(lane) = column_lanes.lane(axis) else {
            return Err(refuse(
                CfgPlanRefusal::LaneUnmapped,
                format!("{entry}: {axis:?} leads no branch unknown"),
            ));
        };
        if lane >= seeds.len() || Some(lane) == correction_lane {
            return Err(refuse(
                CfgPlanRefusal::LaneUnmapped,
                format!("{entry}: lane {lane} of {} seeds", seeds.len()),
            ));
        }
        Ok(lane)
    };

    // ---- stamp values and Jacobians ------------------------------------
    //
    // Both come off the *scalarized differentiated* body rather than the
    // primal, and that is not incidental: a primal CFG containing an
    // undifferentiated `ddx` has no value until the pass that resolves one has
    // run, and ten shipped modules are in that position. Running the pass first
    // is what makes their residuals lowerable at all.
    //
    // Every entry of every stamp is gathered first and lowered *once*, through
    // one [`CfgPrelude`]. Slicing a cone per entry is what made the CFG route's
    // image explode — Σ cone / union ran from 24 on asmesd to 383 on asmhemt,
    // and the fused stamp kernel inlined every one of them a second time — and
    // the cones overlapped because they are all values of one function. The
    // prelude computes each of them once into a slot and each entry becomes the
    // single instruction that reads it.
    let mut entries: Vec<(CfgPlanEntry, ValueId)> = Vec::new();
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let stamp_entry = CfgPlanEntry::StampValue(stamp_index);
        entries.push((
            stamp_entry,
            scalar(cfg.residuals[stamp_index], stamp_entry)?,
        ));
        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            let lane = require_lane(&jacobian.col_axis, entry)?;
            if let Some(value) = jacobian_rows[stamp_index].get(lane).copied().flatten() {
                entries.push((entry, scalar(value, entry)?));
            }
        }
        for (entry_index, reactive) in stamp.reactive_jacobians.iter().enumerate() {
            let entry = CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index);
            let lane = require_lane(&reactive.col_axis, entry)?;
            if let Some(value) = reactive_rows[stamp_index].get(lane).copied().flatten() {
                entries.push((entry, scalar(value, entry)?));
            }
        }
    }
    let entry_values: HashMap<CfgPlanEntry, ValueId> = entries.iter().copied().collect();

    // A value that reads a contribution current is ordered against the
    // residuals before it, and the prelude runs before all of them. Those
    // entries keep the cone they have always had; see [`LiveCurrentTaint`].
    let taint = LiveCurrentTaint::build(&scalarized.function);
    let (published, deferred): (Vec<_>, Vec<_>) = entries
        .iter()
        .copied()
        .partition(|(_, value)| taint.publishable(*value));
    report.live_current_entries = deferred.len();
    report.live_current_control_flow = taint.taints_control_flow();

    let prelude = if published.is_empty() {
        None
    } else {
        match CfgPrelude::build(
            module.as_str(),
            &scalarized.function,
            &published,
            &state,
            &bindings,
            &slots,
        ) {
            Ok(prelude) => Some(prelude),
            // A prelude that will not build costs size and nothing else: every
            // entry goes back to the cone this route lowered before there was
            // one. Refusing the module instead would take a plan that builds
            // down over an optimization, and the union is *harder* to lower
            // than any single cone — it keeps every surviving block's control
            // flow, so a `ddt` under a condition that no one entry's cone
            // reaches is in the union's (mvsg_cmc, measured). The class is
            // recorded rather than swallowed; the size census prints it.
            Err(refused) => {
                report.prelude_refused = Some(refused.class);
                None
            }
        }
    };
    report.prelude_slots = prelude.as_ref().map_or(0, CfgPrelude::slot_count);
    report.prelude_instructions = prelude
        .as_ref()
        .map_or(0, |prelude| prelude.program().ssa().instructions().len());

    // The cones the prelude does not cover, sliced once as a group and then per
    // entry inside that slice — the same two-level prune the whole loop used
    // before, kept because it is what bounds the cost of a module that does
    // have probes.
    let deferred: Vec<(CfgPlanEntry, ValueId)> = if prelude.is_some() {
        deferred
    } else {
        entries.clone()
    };
    let deferred_outputs: Vec<ValueId> = deferred.iter().map(|(_, value)| *value).collect();
    let (deferred_function, deferred_mapped) =
        prune_cfg_to_outputs(&scalarized.function, &deferred_outputs);
    let deferred_values: HashMap<CfgPlanEntry, ValueId> = deferred
        .iter()
        .map(|(entry, _)| *entry)
        .zip(deferred_mapped)
        .collect();

    // One entry's program: a slot read where the prelude publishes it, its own
    // cone where it does not.
    let entry_program = |entry: CfgPlanEntry,
                         report: &mut CfgPlanReport|
     -> Result<PlanProgram, CfgPlanRefused> {
        if let Some(program) = prelude.as_ref().and_then(|p| p.entry_program(entry)) {
            let program = program
                .map_err(|error| refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}")))?;
            let adopted =
                BlockProgram::adopt(module.as_str(), program, &slots).map_err(|error| {
                    refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}"))
                })?;
            return Ok(PlanProgram::Blocks(adopted));
        }
        let output = deferred_values.get(&entry).copied().ok_or_else(|| {
            refuse(
                CfgPlanRefusal::Lowering,
                format!("{entry} has neither a prelude slot nor a cone"),
            )
        })?;
        lower(&deferred_function, output, entry, report)
    };

    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let stamp_entry = CfgPlanEntry::StampValue(stamp_index);
        stamp_values.push(entry_program(stamp_entry, &mut report)?);
        report.stamp_values += 1;

        let mut row = Vec::with_capacity(stamp.jacobian_programs.len());
        for entry_index in 0..stamp.jacobian_programs.len() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            match entry_values.get(&entry).copied() {
                Some(_) => {
                    row.push(entry_program(entry, &mut report)?);
                    report.jacobians += 1;
                }
                None => {
                    report.structural_zeros.push(entry);
                    row.push(PlanProgram::Blocks(
                        BlockProgram::adopt(
                            module.as_str(),
                            constant_zero_program().map_err(|error| {
                                refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}"))
                            })?,
                            &slots,
                        )
                        .map_err(|error| {
                            refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}"))
                        })?,
                    ));
                }
            }
        }
        jacobians.push(row);

        let mut reactive_row = Vec::with_capacity(stamp.reactive_jacobians.len());
        if !stamp.reactive_jacobians.is_empty() && charges[stamp_index].is_none() {
            return Err(refuse(
                CfgPlanRefusal::ChargeMissing,
                format!(
                    "stamp {stamp_index} has {} shipped reactive entries and no stored charge",
                    stamp.reactive_jacobians.len()
                ),
            ));
        }
        for entry_index in 0..stamp.reactive_jacobians.len() {
            let entry = CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index);
            match entry_values.get(&entry).copied() {
                Some(_) => {
                    reactive_row.push(entry_program(entry, &mut report)?);
                    report.reactive_jacobians += 1;
                }
                None => {
                    report.structural_zeros.push(entry);
                    reactive_row.push(PlanProgram::Blocks(
                        BlockProgram::adopt(
                            module.as_str(),
                            constant_zero_program().map_err(|error| {
                                refuse(CfgPlanRefusal::Lowering, format!("{entry}: {error}"))
                            })?,
                            &slots,
                        )
                        .map_err(|error| {
                            refuse(CfgPlanRefusal::SlotUnclaimed, format!("{entry}: {error}"))
                        })?,
                    ));
                }
            }
        }
        reactive_jacobians.push(reactive_row);
    }

    // ---- noise ---------------------------------------------------------
    //
    // A noise magnitude is an ordinary value of the body, so it lowers from the
    // primal; the shipped emitter's two-pass rule is kept because six shipped
    // models read a `ddx` inside a noise power and only the AD pass resolves
    // one.
    //
    // Which *value* is lowered is the whole of the noise slice, and
    // [`noise_magnitude_source`] is where that is decided. See it for the
    // measurement.
    //
    // Skipped entirely under `CfgNoiseScope::Postfix`: nothing here would be
    // kept, and running it would only let a noise-only refusal take a module's
    // residual and Jacobian entries down with it.
    let cfg_noise = match noise {
        CfgNoiseScope::Postfix => None,
        CfgNoiseScope::Cfg => {
            let processes: HashMap<usize, &CfgNoiseProcess> = cfg
                .noise_processes
                .iter()
                .filter_map(|process| {
                    usize::try_from(process.process_id)
                        .ok()
                        .map(|id| (id, process))
                })
                .collect();

            // The magnitudes taken at their own site, gathered before anything
            // is lowered: they are hoisted together, into one straight-line
            // copy of the body per function, so a module with eighteen sources
            // pays for one walk rather than eighteen.
            let mut site_roots = Vec::new();
            for (source_index, source) in model.noise_sources.iter().enumerate() {
                let Some(process) = processes.get(&source.process_id) else {
                    continue;
                };
                let Some(site) = process.site else { continue };
                if noise_magnitude_source(&plan, CfgPlanEntry::NoisePsd(source_index))
                    == NoiseMagnitude::Site
                {
                    site_roots.push(site.psd);
                }
                if let Some(exponent) = site.exponent
                    && noise_magnitude_source(&plan, CfgPlanEntry::NoiseExponent(source_index))
                        == NoiseMagnitude::Site
                {
                    site_roots.push(exponent);
                }
            }
            let primal_sites = hoisted_site_function(&cfg.function, &site_roots);
            let scalarized_sites = hoisted_site_function(
                &scalarized.function,
                &site_roots
                    .iter()
                    .filter_map(|root| scalarized.scalar(*root))
                    .collect::<Vec<_>>(),
            );

            let mut noise_psd = Vec::with_capacity(model.noise_sources.len());
            let mut noise_exponents = Vec::with_capacity(model.noise_sources.len());
            for (source_index, source) in model.noise_sources.iter().enumerate() {
                let process = processes.get(&source.process_id).ok_or_else(|| {
                    refuse(
                        CfgPlanRefusal::NoiseUnpaired,
                        format!(
                            "shipped source {source_index} names process {}",
                            source.process_id
                        ),
                    )
                })?;
                // Site value where the shipped route computes the magnitude
                // unconditionally, exit read where it reads a slot the guarded
                // noise assignment pass wrote. A site value the hoist could not
                // take falls back to the exit read, which is what this route
                // has always lowered.
                let magnitude = |site: Option<ValueId>,
                                 exit: ValueId,
                                 entry: CfgPlanEntry,
                                 report: &mut CfgPlanReport|
                 -> Result<PlanProgram, CfgPlanRefused> {
                    let hoisted = match (site, noise_magnitude_source(&plan, entry)) {
                        (Some(site), NoiseMagnitude::Site) => primal_sites
                            .as_ref()
                            .filter(|function| function.pins(site))
                            .map(|function| (&function.function, site)),
                        _ => None,
                    };
                    if let Some((function, value)) = hoisted {
                        report.noise_hoisted += 1;
                        match lower(function, value, entry, report) {
                            Ok(program) => return Ok(program),
                            Err(primal) if primal.class != CfgPlanRefusal::Lowering => {
                                return Err(primal);
                            }
                            // A `ddx` inside the magnitude, resolved only by the
                            // derivative pass: the same two-pass rule the exit
                            // read follows, over the hoisted copy.
                            Err(primal) => {
                                let resolved = site.and_then(|site| scalarized.scalar(site));
                                if let Some(resolved) = resolved
                                    && let Some(function) = scalarized_sites
                                        .as_ref()
                                        .filter(|function| function.pins(resolved))
                                {
                                    let program =
                                        lower(&function.function, resolved, entry, report)?;
                                    report.noise_from_differentiated += 1;
                                    return Ok(program);
                                }
                                return Err(primal);
                            }
                        }
                    }
                    if !process_is_unconditional(&cfg.function, process.active) {
                        report.noise_guarded_reads.push(entry);
                    }
                    match lower(&cfg.function, exit, entry, report) {
                        Ok(program) => Ok(program),
                        Err(primal) if primal.class == CfgPlanRefusal::Lowering => {
                            let Some(resolved) = scalarized.scalar(exit) else {
                                return Err(primal);
                            };
                            let program = lower(&scalarized.function, resolved, entry, report)?;
                            report.noise_from_differentiated += 1;
                            Ok(program)
                        }
                        Err(other) => Err(other),
                    }
                };
                noise_psd.push(magnitude(
                    process.site.map(|site| site.psd),
                    process.psd,
                    CfgPlanEntry::NoisePsd(source_index),
                    &mut report,
                )?);
                report.noise_values += 1;
                let exponent = match (process.exponent, source.exponent_program.as_ref()) {
                    (Some(exponent), Some(_)) => {
                        report.noise_values += 1;
                        Some(magnitude(
                            process.site.and_then(|site| site.exponent),
                            exponent,
                            CfgPlanEntry::NoiseExponent(source_index),
                            &mut report,
                        )?)
                    }
                    (None, None) => None,
                    (canonical, shipped) => {
                        return Err(refuse(
                            CfgPlanRefusal::NoiseUnpaired,
                            format!(
                                "source {source_index} exponent: canonical={} shipped={}",
                                canonical.is_some(),
                                shipped.is_some()
                            ),
                        ));
                    }
                };
                noise_exponents.push(exponent);
            }
            Some((noise_psd, noise_exponents))
        }
    };

    // ---- the read-sets the runtime validates a dispatch against ---------
    //
    // Recomputed from the block programs rather than carried over: the two
    // routes do not reach the same unknowns, and a borrowed set would describe
    // a program that was never compiled.
    let dependencies = &mut plan.current_dependencies;
    // The prelude's, first: a slot-read entry has no read-set of its own any
    // more, so the reads it used to declare are the prelude's now and the
    // runtime has to be told where they went.
    dependencies.prelude_branch_unknowns = prelude
        .as_ref()
        .map(|prelude| {
            PlanProgramRef::Blocks(prelude.program())
                .branch_unknown_dependencies()
                .to_vec()
        })
        .unwrap_or_default();
    dependencies.stamp_values = read_set(&stamp_values, PlanProgram::current_pair_dependencies);
    dependencies.stamp_value_prior_currents =
        read_set(&stamp_values, PlanProgram::prior_current_dependencies);
    dependencies.stamp_value_branch_unknowns =
        read_set(&stamp_values, PlanProgram::branch_unknown_dependencies);
    dependencies.jacobians = read_set_rows(&jacobians, PlanProgram::current_pair_dependencies);
    dependencies.jacobian_prior_currents =
        read_set_rows(&jacobians, PlanProgram::prior_current_dependencies);
    dependencies.jacobian_branch_unknowns =
        read_set_rows(&jacobians, PlanProgram::branch_unknown_dependencies);
    dependencies.reactive_jacobians =
        read_set_rows(&reactive_jacobians, PlanProgram::current_pair_dependencies);
    dependencies.reactive_jacobian_prior_currents =
        read_set_rows(&reactive_jacobians, PlanProgram::prior_current_dependencies);
    dependencies.reactive_jacobian_branch_unknowns = read_set_rows(
        &reactive_jacobians,
        PlanProgram::branch_unknown_dependencies,
    );
    // The noise read-sets stay the postfix plan's whenever its noise programs
    // do: a recomputed set would then describe programs this plan does not
    // carry, which is the same defect in the other direction.
    if let Some((noise_psd, noise_exponents)) = &cfg_noise {
        dependencies.noise_psd = read_set(noise_psd, PlanProgram::current_pair_dependencies);
        dependencies.noise_psd_prior_currents =
            read_set(noise_psd, PlanProgram::prior_current_dependencies);
        dependencies.noise_psd_branch_unknowns =
            read_set(noise_psd, PlanProgram::branch_unknown_dependencies);
        dependencies.noise_exponents =
            read_set_optional(noise_exponents, PlanProgram::current_pair_dependencies);
        dependencies.noise_exponent_prior_currents =
            read_set_optional(noise_exponents, PlanProgram::prior_current_dependencies);
        dependencies.noise_exponent_branch_unknowns =
            read_set_optional(noise_exponents, PlanProgram::branch_unknown_dependencies);
    }

    plan.prelude = prelude.map(|prelude| NativePrelude {
        slot_count: prelude.slot_count(),
        program: PlanProgram::Blocks(prelude.into_program()),
    });
    plan.stamp_values = stamp_values;
    plan.jacobians = jacobians;
    plan.reactive_jacobians = reactive_jacobians;
    if let Some((noise_psd, noise_exponents)) = cfg_noise {
        plan.noise_psd = noise_psd;
        plan.noise_exponents = noise_exponents;
    }
    plan.validate_shape(model)
        .map_err(|error| refuse(CfgPlanRefusal::EquationsUnpaired, error.to_string()))?;
    Ok(CfgModelPlan { plan, report })
}

/// The plan every backend compiles.
///
/// x64, AArch64 and the WASM JIT all enter here, and what they get is the CFG
/// route's plan, with the postfix plan as the fallback for a module the CFG
/// route refuses. A refusal reaches the `[JIT]` log seam a failed native compile
/// already uses, so a module that quietly stopped taking the CFG route is
/// visible in the same place a module that quietly stopped compiling is.
///
/// # Why this is the CFG route, and this is the evidence
///
/// W-F3c built the CFG route into this constructor and measured it against the
/// estate. Six tests turned red, and not one was rounding: each was a construct
/// the CFG lowering treated differently from MIR, where the CFG plan *builds*
/// and computes another number. `$port_connected` (6.0 against 0.0 on an
/// omitted terminal), `$simparam` (0.0 against a gmin of 1e-12), a
/// contribution-current probe (a Jacobian entry of 4.5 against 2.5), an array
/// variable's declaration initializer, an event-controlled variable across a
/// rejected step (2.0 against 1.0), and `hypot`/`atan2` under a `ddx`, where the
/// scalar derivative rules fall through to a zero.
///
/// Every one of them but the last was *closed* rather than screened, and
/// [`a_closed_divergence_takes_the_cfg_route`] is where they moved to.
/// `hypot`/`atan2` is not on that list because it is not a divergence: neither
/// route computes a wrong number for it, the CFG route simply has no rule and
/// refuses, which is [`DERIVATIVE_RULE_HOLES`]. The pattern each followed is the
/// same one: the divergence was a lowering decision one route had and the other
/// did not, and the fix was to give the CFG route the same decision.
///
/// * `$port_connected` folded to `1.0` because the generated backend builds
///   every instance it evaluates. It is a runtime leaf for a backend that
///   cannot promise that —
///   [`CfgModel::from_hir_for_executable_backend`](crate::canonical_ir::CfgModel::from_hir_for_executable_backend)
///   is where the two consumers part company — and `$simparam` with no source
///   now reads the same `simparam_source_default` table on both routes. (W-F4)
/// * A **prologue-only definition** — a localparam or a declaration initializer
///   the body reads — took Verilog-AMS zero because the CFG had no definition of
///   it at all. The executable lowering now evaluates
///   [`HirModel::prologue_statements`](crate::canonical_ir::HirModel) into the
///   entry block. (W-F4b)
/// * An **event-controlled variable** was applied twice: the assignment pass
///   this plan keeps had already run the guard-folded event body into the
///   variable's slot, and the CFG applied it again on top of the leaf that reads
///   that slot, counting 2.0 where the shipped route counts 1.0. A read of one
///   is now the leaf itself, which is the `LoadVariable` the postfix route's own
///   read lowers to. (W-F4b)
/// * A **contribution-current probe** — `I(p, mid)` read inside another
///   contribution — was frozen by the MIR route and inlined and differentiated
///   through by the CFG route, so the two Jacobians were different matrices:
///   `native_device_stamps_internal_node_current_probe_alias_jacobian_without_fallback`
///   measured 4.5 against 2.5 on one entry.
///   [`CfgValueKind::ContributedCurrent`](crate::canonical_ir::CfgValueKind)
///   names the probe's endpoints canonically and the contribution it was taken
///   after;
///   [`CfgRuntimeBindings`](crate::native::cfg_program::CfgRuntimeBindings)
///   carries the translation to the `LoadCurrent(pair)` or summed
///   `LoadPriorCurrent(index)` that `lower_branch_access` chose — the same op,
///   the same order, the same signs — and a leaf's derivative is zero, which is
///   what frozen means. Pinned by
///   [`a_contribution_current_probe_reads_the_shipped_routes_storage`]. (W-F5)
///
/// # Why six red tests were not enough, and what was
///
/// That screen was empirical, and W-F4 showed the bound on what it was worth by
/// going looking for what the estate did not cover: a module-scope `real g =
/// 4.5;` and a `localparam` read from the body both computed a residual of 0.0
/// on this route, against 9.0 and 6.0 through MIR, and about twelve hundred
/// tests ran green with both silently wrong. Two more, found by asking, on top
/// of the six found by failing.
///
/// So the instrument that bounds the rest is a census, not a test suite, and
/// four of them stand behind this call:
///
/// * [`crate::native::cfg_mir_census`] walks both *built plans* through x64 over
///   the shipped forty-three at three biases, judging each entry against a
///   double-double reference: 43/43 within bound, `over_bound=0`,
///   `walker_disagreements=0`, `lost_entries=0`, and the only two modules it
///   cannot build are this route's two refusals.
/// * [`crate::native::cfg_census`] compares the CFG interpreter against x64 on
///   the CFG plan: 43/43 at `max_relative_deviation=0`.
/// * [`crate::native::code_identity`] measures the shipped corpus's emitted
///   machine code. The flip moved it, which is what a route change is *supposed*
///   to do, and its documentation carries the per-module reading that
///   re-baselined it — every module moved except the two the fallback names.
/// * [`crate::native::cfg_cost_census`] measures what it costs, which is the one
///   thing the other three cannot say. See the module documentation: the route
///   is free and [`CfgNoiseScope::Cfg`] is not.
///
/// # The postfix plan did not change, and that is checkable
///
/// [`build_model_plan_with_canonical_ir`], and every function it reaches, is
/// untouched by this lane — it is the fallback above and the reference side of
/// the CFG-versus-MIR census, so a module that refuses compiles to exactly the
/// machine code it compiled to before. `jit::plan_builder` is not in the diff.
pub(crate) fn build_default_model_plan(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModelPlan> {
    let (plan, refused) = build_default_model_plan_reported(model, artifact)?;
    if let Some(refused) = refused {
        log::warn!(
            "[JIT] Model '{}' takes the postfix plan: {} ({})",
            refused.module,
            refused.class.name(),
            refused.detail
        );
        #[cfg(debug_assertions)]
        eprintln!(
            "[JIT] Model '{}' takes the postfix plan: {} ({})",
            refused.module,
            refused.class.name(),
            refused.detail
        );
    }
    Ok(plan)
}

/// [`build_default_model_plan`], handing the refusal back instead of logging
/// it, so a test can pin which module fell back and on what class.
///
/// The fallback rebuilds the postfix plan rather than recovering the one the CFG
/// builder started from and threw away. That is one extra plan build, on the
/// refusal path only, once per module per process — the native cache in
/// [`crate::device`] keys on the module, so no model pays it twice — against a
/// constructor whose refusals stay a single `?` each.
pub(crate) fn build_default_model_plan_reported(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<(NativeModelPlan, Option<CfgPlanRefused>)> {
    match build_model_plan_from_canonical_cfg(model, artifact, CfgNoiseScope::Cfg) {
        Ok(built) => Ok((built.plan, None)),
        Err(refused) => {
            let plan = build_model_plan_with_canonical_ir(model, artifact)?;
            Ok((plan, Some(refused)))
        }
    }
}

fn read_set(entries: &[PlanProgram], read: fn(&PlanProgram) -> &[usize]) -> Vec<Vec<usize>> {
    entries.iter().map(|entry| read(entry).to_vec()).collect()
}

fn read_set_rows(
    rows: &[Vec<PlanProgram>],
    read: fn(&PlanProgram) -> &[usize],
) -> Vec<Vec<Vec<usize>>> {
    rows.iter().map(|row| read_set(row, read)).collect()
}

fn read_set_optional(
    entries: &[Option<PlanProgram>],
    read: fn(&PlanProgram) -> &[usize],
) -> Vec<Vec<usize>> {
    entries
        .iter()
        .map(|entry| {
            entry
                .as_ref()
                .map_or_else(Vec::new, |entry| read(entry).to_vec())
        })
        .collect()
}

#[cfg(all(test, feature = "native"))]
mod tests {
    use super::{
        CfgNoiseScope, CfgPlanRefusal, ShippedColumnLanes,
        build_default_model_plan_reported, build_model_plan_from_canonical_cfg,
    };
    use crate::canonical_ir::CanonicalIrArtifact;
    use crate::codegen::{ColumnAxis, CompiledModel};
    use crate::jit::model_plan::NativeModelPlan;
    use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
    use crate::{CompilerOptions, VerilogACompiler};

    /// Angelov's optional series resistance, reduced to the two branches that
    /// make the numbering diverge.
    ///
    /// `V(a, b)` is contributed to twice, so MIR mints two branch-current
    /// unknowns for one branch while the solver gives it one column. Every
    /// solver column after it is then a different number from the MIR unknown
    /// that shares its ordinal.
    const REPEATED_POTENTIAL_CONTRIBUTION: &str = r#"
module cfg_repeated_potential_branch(a, b, c);
  inout a, b, c;
  electrical a, b, c;
  parameter real r = 0.05;
  parameter real l = 1.0e-9;
  parameter real rs = 0.25;
  analog begin
    V(a, b) <+ I(a, b) * r;
    V(a, b) <+ ddt(l * I(a, b));
    V(b, c) <+ I(b, c) * rs;
  end
endmodule
"#;

    /// A resistor, a capacitor and a thermal noise source: one module carrying
    /// all four kinds of value entry the flip decides between.
    const RESISTOR_WITH_CHARGE_AND_NOISE: &str = r#"
module cfg_default_plan(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  parameter real c = 1.0e-12;
  // An expression default, so `parameter_defaults` carries a program at all.
  parameter real g = 1.0 / r;
  analog begin
    I(p, n) <+ V(p, n) * g;
    I(p, n) <+ ddt(c * V(p, n));
    I(p, n) <+ white_noise(4.0 * 1.3806505e-23 * $temperature / r, "thermal");
  end
endmodule
"#;

    /// A `ddt` inside a `case` selector, which is one of the sites
    /// `HirExecutedCorrespondence` does not cover: the canonical operator has no
    /// executed counterpart, so [`CfgStateAllocation`](crate::canonical_ir::CfgStateAllocation)
    /// cannot name its state record and the CFG route refuses the module.
    ///
    /// A real refusal from a real source, not an injected one. If a later lane
    /// covers that site the CFG route will start building this module, and this
    /// test will say so rather than going quietly green.
    const OPERATOR_IN_A_CASE_SELECTOR: &str = r#"
module cfg_plan_fallback(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    case (ddt(V(p, n)) > 0.0)
      1: I(p, n) <+ V(p, n);
      default: I(p, n) <+ 2.0 * V(p, n);
    endcase
  end
endmodule
"#;

    fn compile(source: &str) -> (CompiledModel, CanonicalIrArtifact) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        (model, artifact)
    }

    fn forms(plan: &NativeModelPlan) -> Vec<(&'static str, &'static str)> {
        let mut forms = Vec::new();
        for entry in &plan.stamp_values {
            forms.push(("stamp_values", entry.borrow().form_name()));
        }
        for entry in plan.jacobians.iter().flatten() {
            forms.push(("jacobians", entry.borrow().form_name()));
        }
        for entry in plan.reactive_jacobians.iter().flatten() {
            forms.push(("reactive_jacobians", entry.borrow().form_name()));
        }
        for entry in &plan.noise_psd {
            forms.push(("noise_psd", entry.borrow().form_name()));
        }
        for entry in plan.noise_exponents.iter().flatten() {
            forms.push(("noise_exponents", entry.borrow().form_name()));
        }
        for entry in plan.parameter_defaults.iter().flatten() {
            forms.push(("parameter_defaults", entry.borrow().form_name()));
        }
        for entry in plan.static_conditions.iter().flatten() {
            forms.push(("static_conditions", entry.borrow().form_name()));
        }
        forms
    }

    /// A branch written twice must not shift the next branch's column.
    ///
    /// The census measured this on Angelov as `jacobians[29][5]`: the shipped
    /// route stamped `Rs_T` = 0.05 into the matrix and the CFG route held the
    /// entry structurally absent, because solver column 6 was read as MIR
    /// unknown 6 — one of the five on `V(g,gi)` — instead of unknown 10, the
    /// one the equation actually differentiates against.
    #[test]
    fn a_branch_contributed_to_twice_does_not_shift_the_next_branchs_column() {
        let (model, artifact) = compile(REPEATED_POTENTIAL_CONTRIBUTION);
        // Not vacuous: the two numbering spaces really do differ here.
        assert_eq!(
            artifact.mir.branch_unknowns.len(),
            3,
            "MIR mints one branch unknown per potential equation"
        );
        assert_eq!(
            model.branch_sources.len(),
            2,
            "the solver gives each physical branch one column"
        );

        let lanes = ShippedColumnLanes::build(&model, &artifact.mir).expect("shipped column lanes");
        let nodes = artifact.mir.nodes.len();
        assert_eq!(lanes.lane(&ColumnAxis::Branch(0)), Some(nodes));
        assert_eq!(
            lanes.lane(&ColumnAxis::Branch(1)),
            Some(nodes + 2),
            "solver column 1 is the third MIR unknown, not the second"
        );

        let plan = build_model_plan_from_canonical_cfg(&model, &artifact, CfgNoiseScope::Cfg)
            .expect("the CFG plan builds");
        assert!(
            plan.report.structural_zeros.is_empty(),
            "every column of this module is a derivative the CFG route holds: {:?}",
            plan.report.structural_zeros
        );
    }

    /// The flip, stated as the shape of the plan production builds: the CFG
    /// route owns the residual, both Jacobians and the noise magnitudes, and
    /// the MIR route owns everything else.
    #[test]
    fn the_cfg_route_takes_its_values_and_its_noise_from_the_cfg() {
        let (model, artifact) = compile(RESISTOR_WITH_CHARGE_AND_NOISE);
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact)
            .expect("the default plan builds");
        assert!(
            refused.is_none(),
            "a resistor with a charge and a noise source is not a module the CFG route refuses: \
             {refused:?}"
        );

        // Not vacuous: the module really does carry an entry of each kind.
        assert_eq!(plan.stamp_values.len(), 3);
        assert!(plan.jacobians.iter().flatten().count() > 0);
        assert!(plan.reactive_jacobians.iter().flatten().count() > 0);
        assert_eq!(plan.noise_psd.len(), 1);
        assert!(plan.parameter_defaults.iter().flatten().count() > 0);

        for (field, form) in forms(&plan) {
            let expected = match field {
                "stamp_values" | "jacobians" | "reactive_jacobians" | "noise_psd"
                | "noise_exponents" => "block",
                // `parameter_defaults` and `static_conditions`, which the CFG
                // does not carry and this route deliberately leaves alone.
                _ => "postfix",
            };
            assert_eq!(
                form, expected,
                "the default plan's {field} entries are {expected} programs"
            );
        }
    }

    /// A module the CFG route refuses takes the postfix plan whole — every
    /// field, not the fields that happened to lower — and says which class
    /// refused it.
    #[test]
    fn a_module_the_cfg_route_refuses_takes_the_postfix_plan_whole() {
        let (model, artifact) = compile(OPERATOR_IN_A_CASE_SELECTOR);
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact)
            .expect("the default plan builds");
        let refused = refused.expect(
            "a canonical operator in a case selector has no executed counterpart, so the CFG \
             route cannot name its state record",
        );
        assert_eq!(refused.class, CfgPlanRefusal::StateAllocation);
        assert_eq!(refused.module, "cfg_plan_fallback");

        for (field, form) in forms(&plan) {
            assert_eq!(
                form, "postfix",
                "a refused module's {field} entries come from the postfix plan"
            );
        }

        // The fallback is the postfix plan, not a plan that resembles it: the
        // same constructor, over the same artifact, produces the same shape.
        let postfix = build_model_plan_with_canonical_ir(&model, &artifact)
            .expect("the postfix plan builds for a module the CFG route refuses");
        assert_eq!(forms(&plan), forms(&postfix));
        assert_eq!(
            plan.current_dependencies.stamp_values,
            postfix.current_dependencies.stamp_values,
        );
        assert_eq!(
            plan.current_dependencies.jacobians,
            postfix.current_dependencies.jacobians,
        );
    }

    /// `hypot` under a `ddx` is differentiable and the CFG pass has no rule for
    /// it, so the module falls back rather than taking a zero derivative.
    ///
    /// This test fails by *building* the module, which is what it is for: the
    /// day [`DERIVATIVE_RULE_HOLES`] gains the two rules, this says so instead
    /// of quietly continuing to refuse a module it no longer needs to.
    #[test]
    fn a_module_the_derivative_pass_has_no_rule_for_falls_back() {
        let source = r#"
module cfg_plan_no_rule(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(hypot(2.0 * V(p, n), V(p, n) + 3.0), V(p, n));
endmodule
"#;
        let (model, artifact) = compile(source);
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact)
            .expect("the default plan builds");
        let refused = refused.expect(
            "the CFG derivative pass has no rule for hypot, so a module differentiating one \
             cannot take the CFG route",
        );
        assert_eq!(refused.class, CfgPlanRefusal::DerivativeRuleMissing);
        for (field, form) in forms(&plan) {
            assert_eq!(form, "postfix", "a refused module's {field} stay postfix");
        }
    }

    /// A construct that *was* a known divergence and no longer is takes the CFG
    /// route, and takes it for the values the route owns.
    ///
    /// The pin every closed entry moved to as it came off the divergence
    /// screen, which is why there is no screen left. Refusing one of these
    /// again — by reinstating one, or by a lowering change that makes the
    /// construct unbuildable —
    /// fails here rather than passing quietly as a fallback, which is the only
    /// way a closed divergence can regress without being noticed.
    ///
    /// The *numbers* these modules produce are pinned where the divergence was
    /// found, against the executed device: `$port_connected` by
    /// `port_connected_reflects_omitted_trailing_terminal` in `device_eval`,
    /// `$simparam` by
    /// `native_device_with_canonical_ir_executes_simparam_current_without_fallback`
    /// in `native_contract`. This test pins the route; those pin the value.
    #[test]
    fn a_closed_divergence_takes_the_cfg_route() {
        let cases: &[(&str, &str)] = &[
            (
                "port-connected",
                r#"
module cfg_closed_port(p, n, opt);
  inout p, n, opt;
  electrical p, n, opt;
  analog I(p, n) <+ ($port_connected(opt) ? 10.0 : 1.0) * V(p, n);
endmodule
"#,
            ),
            (
                "simparam-with-fallback",
                r#"
module cfg_closed_simparam(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ $simparam("gmin", 1.0e-15) * V(p, n);
endmodule
"#,
            ),
            (
                "simparam-without-fallback",
                r#"
module cfg_closed_simparam_bare(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ $simparam("gmin") * V(p, n);
endmodule
"#,
            ),
            // The three prologue shapes W-F4 measured and W-F4b closed. The
            // numbers they produce are pinned by
            // [`the_prologue_reaches_a_body_read`]; this pins that they build
            // at all, which is what a reinstated screen would break.
            (
                "array-initializer",
                r#"
module cfg_div_array(p, n);
  inout p, n;
  electrical p, n;
  real c[0:1] = '{0.5e-3, 1.5e-3};
  analog I(p, n) <+ (c[0] + c[1]) * V(p, n);
endmodule
"#,
            ),
            (
                "scalar-initializer",
                r#"
module cfg_div_scalar_init(p, n);
  inout p, n;
  electrical p, n;
  real g = 4.5;
  analog I(p, n) <+ g * V(p, n);
endmodule
"#,
            ),
            (
                "localparam",
                r#"
module cfg_div_localparam(p, n);
  inout p, n;
  electrical p, n;
  localparam real k = 3.0;
  real g;
  analog begin
    g = k;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#,
            ),
            // The event-controlled variable W-F4 measured at 2.0 against 1.0.
            // Its number is pinned by
            // `event_semantics::checkpoint_before_acceptance_excludes_step_event_variable_candidates`,
            // which runs the shipped route; this pins that the CFG route builds
            // the module rather than falling it back.
            (
                "event-controlled-variable",
                r#"
module cfg_div_event(p, n);
  inout p, n;
  electrical p, n;
  real count;
  analog begin
    @(initial_step("tran")) count = count + 1.0;
    I(p, n) <+ count;
  end
endmodule
"#,
            ),
            // A guarded contribution, which mints a `__guard1` snapshot. The
            // screen that used to name a prologue-only definition by variable
            // read that snapshot as one and refused sixteen of the estate's
            // modules over it; the prologue is now a list the analyzer records
            // rather than a set inferred from assignment targets, so a
            // synthesized name cannot be mistaken for one.
            (
                "guarded-contribution",
                r#"
module cfg_closed_guard(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    if (V(p, n) > 1.0)
      I(p, n) <+ 2.0 * V(p, n);
    else
      I(p, n) <+ V(p, n);
  end
endmodule
"#,
            ),
            // The contribution-current probe, in each of the four shapes the
            // estate had of it. Their storage is pinned by
            // [`a_contribution_current_probe_reads_the_shipped_route_s_storage`]
            // and their numbers by the `native_contract` device tests, which
            // run the plan production builds.
            (
                "current-probe-terminal-pair",
                r#"
module cfg_closed_current_probe(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ V(p, n);
    I(p, n) <+ I(p, n) * 0.1;
  end
endmodule
"#,
            ),
            (
                "current-probe-internal-node",
                r#"
module cfg_closed_internal_current_probe(p, n);
  inout p, n;
  electrical p, n;
  electrical mid;
  analog begin
    I(p, mid) <+ V(p, mid);
    I(p) <+ I(p, mid) * V(p);
  end
endmodule
"#,
            ),
            (
                "current-probe-named-branch",
                r#"
module cfg_closed_named_current_probe(p, n);
  inout p, n;
  electrical p, n, sense_node;
  branch (sense_node) sense;
  analog begin
    I(sense) <+ V(p, n);
    I(p, n) <+ 2.0 * I(sense);
  end
endmodule
"#,
            ),
            // A probe read back into a variable rather than into a residual.
            // The assignment passes stay postfix on this route, so what this
            // pins is that carrying the probe in the CFG at all no longer
            // refuses the module.
            (
                "current-probe-in-an-assignment",
                r#"
module cfg_closed_assigned_current_probe(p, n);
  inout p, n;
  electrical p, n, x;
  real sensed, reverse, port_n;
  analog begin
    I(x, n) <+ 2.0 * V(p, n);
    I(x, n) <+ 1.0;
    sensed = I(x, n);
    reverse = I(n, x);
    port_n = I(<n>);
  end
endmodule
"#,
            ),
            // A probe read *before* the contribution it names. There is no
            // slot to freeze it against — the shipped route registers a probe
            // only for a contribution it has already lowered, and answers this
            // one by splitting the assignment pass in two — so the CFG keeps
            // the running accumulator, which at that point is the entry block's
            // zero. That is the language's answer and a constant, so freezing
            // it would change nothing and refusing it would drop the module.
            (
                "current-probe-before-its-contribution",
                r#"
module cfg_closed_forward_current_probe(p, n);
  inout p, n;
  electrical p, n;
  real sensed;
  analog begin
    sensed = I(p, n);
    I(p, n) <+ V(p, n);
  end
endmodule
"#,
            ),
        ];
        for (case, source) in cases {
            let (model, artifact) = compile(source);
            let (plan, refused) =
                build_default_model_plan_reported(&model, &artifact)
                    .unwrap_or_else(|error| panic!("{case}: the default plan builds: {error}"));
            assert!(
                refused.is_none(),
                "{case}: this construct is no longer a known divergence, so the CFG route must \
                 build it: {refused:?}"
            );
            for (field, form) in forms(&plan) {
                let expected = match field {
                    "stamp_values" | "jacobians" | "reactive_jacobians" => "block",
                    _ => "postfix",
                };
                assert_eq!(form, expected, "{case}: {field} entries are {expected}");
            }
        }
    }

    /// A contribution-current probe reads the storage the shipped route reads,
    /// and nothing of the equation that filled it.
    ///
    /// The identity half of the closure, and the reason the class is closed
    /// rather than bounded. The two routes are held to two things here.
    ///
    /// *The same storage.* A plan's current read-sets are recomputed from the
    /// programs it actually carries — see the `read_set` calls above — so the
    /// CFG plan's sets are a statement about the block programs and the postfix
    /// plan's are a statement about the streams. Equal sets mean the two routes
    /// load the same slots for the same entries, which is what
    /// [`CfgRuntimeBindings`](crate::native::cfg_program::CfgRuntimeBindings)
    /// is translating for.
    ///
    /// *Frozen, not recomputed.* The read-sets would also be equal if the CFG
    /// program loaded the slot and then went on to inline the equation anyway,
    /// so the second assertion is over the instructions: the entry holds the
    /// load and holds nothing the inlined equation would have brought with it.
    /// That is the assertion the 4.5-against-2.5 Jacobian was a symptom of —
    /// `native_device_stamps_internal_node_current_probe_alias_jacobian_without_fallback`
    /// in `native_contract` measures the matrix itself, on the plan production
    /// builds.
    #[test]
    fn a_contribution_current_probe_reads_the_shipped_routes_storage() {
        use crate::jit::expr::{NativeOp, VoltageNode};
        use crate::jit::plan_program::PlanProgram;

        // A two-terminal probe of a pair of terminals: the shipped route reads
        // the pair's own running total, and `I(p, n)` on terminals 0 and 1 of a
        // two-terminal module is pair `0 * (2 + 1) + 1`.
        let terminal_pair = r#"
module cfg_probe_terminal_pair(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ V(p, n);
    I(p, n) <+ I(p, n) * 0.1;
  end
endmodule
"#;
        // A probe with an internal endpoint, which has no pair total, so the
        // shipped route reads the contributing equation's own slot. The
        // fixture is `native_internal_current_probe_alias_jacobian`, whose
        // Jacobian is where the divergence was measured.
        let internal_node = r#"
module cfg_probe_internal_node(p, n);
  inout p, n;
  electrical p, n;
  electrical mid;
  analog begin
    I(p, mid) <+ V(p, mid);
    I(p) <+ I(p, mid) * V(p);
  end
endmodule
"#;

        for (case, source, expected, forbidden) in [
            (
                "terminal-pair",
                terminal_pair,
                NativeOp::LoadCurrent(1),
                // Inlining `I(p, n)` would bring the first equation's
                // `V(p, n)` with it, and the second equation reads no
                // potential of its own.
                "a node potential" as &str,
            ),
            (
                "internal-node",
                internal_node,
                NativeOp::LoadPriorCurrent(0),
                // This equation does read `V(p)`, so what inlining would add
                // is the internal node the probed branch ends on.
                "the internal node's potential",
            ),
        ] {
            let (model, artifact) = compile(source);
            let (cfg, refused) =
                build_default_model_plan_reported(&model, &artifact)
                    .unwrap_or_else(|error| panic!("{case}: the CFG plan builds: {error}"));
            assert!(
                refused.is_none(),
                "{case}: a contribution-current probe is no longer a divergence: {refused:?}"
            );
            let postfix = build_model_plan_with_canonical_ir(&model, &artifact)
                .unwrap_or_else(|error| panic!("{case}: the postfix plan builds: {error}"));

            let cfg_reads = &cfg.current_dependencies;
            let postfix_reads = &postfix.current_dependencies;
            assert_eq!(
                (
                    &cfg_reads.stamp_values,
                    &cfg_reads.stamp_value_prior_currents
                ),
                (
                    &postfix_reads.stamp_values,
                    &postfix_reads.stamp_value_prior_currents
                ),
                "{case}: the two routes read the same current storage for every residual"
            );
            assert_eq!(
                (&cfg_reads.jacobians, &cfg_reads.jacobian_prior_currents),
                (
                    &postfix_reads.jacobians,
                    &postfix_reads.jacobian_prior_currents
                ),
                "{case}: the two routes read the same current storage for every Jacobian entry"
            );

            // The probing equation is the second one, and it is the entry the
            // structural claim is about.
            let PlanProgram::Blocks(probe) = &cfg.stamp_values[1] else {
                panic!("{case}: the CFG route's residuals are block programs");
            };
            let ops: Vec<NativeOp> = probe
                .ssa()
                .instructions()
                .iter()
                .map(|instruction| instruction.op())
                .collect();
            assert!(
                ops.contains(&expected),
                "{case}: the probing residual loads the frozen current {expected:?}: {ops:?}"
            );
            let inlined = ops.iter().any(|op| match (case, op) {
                ("terminal-pair", NativeOp::LoadVoltage { .. }) => true,
                (
                    "internal-node",
                    NativeOp::LoadVoltage {
                        pos: VoltageNode::Internal(_),
                        ..
                    }
                    | NativeOp::LoadVoltage {
                        neg: VoltageNode::Internal(_),
                        ..
                    },
                ) => true,
                _ => false,
            });
            assert!(
                !inlined,
                "{case}: the probing residual holds the load and not the equation behind it, so \
                 it must not read {forbidden}: {ops:?}"
            );
        }
    }

    /// A body read of a prologue definition sees what the prologue wrote.
    ///
    /// The value half of the prologue class, evaluated on the *executable*
    /// lowering — the one [`build_model_plan_from_canonical_cfg`] builds — at a
    /// bias where the expected residual is a number a reader can check by hand.
    /// Each of these returned `0.0` before W-F4b, on a route that built the
    /// module and computed a wrong number rather than refusing it.
    ///
    /// The reference numbers are the flat route's, which is the language's:
    /// `cinit` is the fixture `array_vars::assignment_pattern_initializer_fills_elements`
    /// stamps at `4.0e-3`, and the scalar and localparam shapes are the two
    /// W-F4 measured at `9.0` and `6.0` against `Postfix`.
    ///
    /// The last case is the one a "run the prologue only where the body never
    /// assigns" fix would get wrong: `g` has a declaration initializer *and* a
    /// conditional body assignment, and on the arm that does not assign it the
    /// initializer's value must survive.
    #[test]
    fn the_prologue_reaches_a_body_read() {
        use crate::canonical_ir::{CfgEvalInputs, CfgModel, evaluate_cfg};
        use std::collections::{HashMap, HashSet};

        // (case, source, V(p, n), residual of `I(p, n)`)
        let cases: &[(&str, &str, f64, f64)] = &[
            (
                "declaration-initializer",
                r#"
module cinit(p, n);
  inout p, n;
  electrical p, n;
  real c[0:2] = '{0.5e-3, 1.5e-3, 2.0e-3};
  analog I(p, n) <+ (c[0] + c[1] + c[2]) * V(p, n);
endmodule
"#,
                1.0,
                4.0e-3,
            ),
            (
                "scalar-initializer",
                r#"
module cfg_prologue_scalar(p, n);
  inout p, n;
  electrical p, n;
  real g = 4.5;
  analog I(p, n) <+ g * V(p, n);
endmodule
"#,
                2.0,
                9.0,
            ),
            (
                "localparam",
                r#"
module cfg_prologue_localparam(p, n);
  inout p, n;
  electrical p, n;
  localparam real k = 3.0;
  real g;
  analog begin
    g = k;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#,
                2.0,
                6.0,
            ),
            (
                "initializer-a-body-arm-overwrites",
                r#"
module cfg_prologue_overwritten(p, n);
  inout p, n;
  electrical p, n;
  real g = 4.5;
  analog begin
    if (V(p, n) > 10.0)
      g = 1.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#,
                2.0,
                9.0,
            ),
        ];

        for (case, source, voltage, expected) in cases {
            let (_, artifact) = compile(source);
            let cfg = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir)
                .unwrap_or_else(|diagnostics| panic!("{case}: lowers: {diagnostics:?}"));
            let mut node_potentials = vec![0.0; artifact.mir.nodes.len()];
            node_potentials[0] = *voltage;
            let inputs = CfgEvalInputs {
                parameters: artifact
                    .mir
                    .parameters
                    .iter()
                    .map(|parameter| parameter.default.unwrap_or(0.0))
                    .collect(),
                parameter_given: vec![false; artifact.mir.parameters.len()],
                port_connected: vec![true; artifact.hir.ports.len()],
                event_state: Vec::new(),
                event_controls: HashMap::new(),
                node_potentials,
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
            };
            let snapshot = evaluate_cfg(&cfg.function, &inputs)
                .unwrap_or_else(|error| panic!("{case}: evaluates: {error:?}"));
            let residual = cfg
                .residuals
                .first()
                .and_then(|value| snapshot.value(*value))
                .unwrap_or_else(|| panic!("{case}: the residual is defined on every path"));
            assert_eq!(
                residual, *expected,
                "{case}: the executable CFG reads the prologue's value"
            );
        }
    }

    /// An event-controlled variable is read from its slot, not recomputed.
    ///
    /// The executable plan keeps the MIR route's assignment pass, which has
    /// already run `count = count + 1.0` into `count`'s runtime slot by the time
    /// a value entry executes. Applying the event body again on top of the leaf
    /// that reads that slot is what counted 2.0 against the shipped route's 1.0.
    ///
    /// Stated structurally, because that is the property rather than one
    /// arithmetic consequence of it: the residual's cone holds the
    /// `EventState` leaf — the same `LoadVariable` the postfix route reads —
    /// and holds nothing from the event body, so there is no second
    /// application left to get wrong. The generated lowering of the same module
    /// keeps the body, because there it *is* the computation.
    #[test]
    fn an_event_controlled_variable_is_read_from_its_slot() {
        use crate::canonical_ir::{CfgModel, CfgValueKind, prune_cfg_to_outputs};

        const SOURCE: &str = r#"
module cfg_event_frozen(p, n);
  inout p, n;
  electrical p, n;
  real count;
  analog begin
    @(initial_step("tran")) count = count + 1.0;
    I(p, n) <+ count;
  end
endmodule
"#;
        let (_, artifact) = compile(SOURCE);

        let residual_cone = |cfg: &CfgModel| {
            let (pruned, _) = prune_cfg_to_outputs(&cfg.function, &cfg.residuals);
            pruned
                .values
                .iter()
                .map(|value| value.kind.clone())
                .collect::<Vec<_>>()
        };

        let executable = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir)
            .expect("lowers");
        let cone = residual_cone(&executable);
        assert!(
            cone.iter()
                .any(|kind| matches!(kind, CfgValueKind::EventState(0))),
            "the residual reads the event-state leaf: {cone:?}"
        );
        assert!(
            !cone
                .iter()
                .any(|kind| matches!(kind, CfgValueKind::RealConstant(one) if *one == 1.0)),
            "the event body's increment is not applied a second time: {cone:?}"
        );

        let generated = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
        let cone = residual_cone(&generated);
        assert!(
            cone.iter()
                .any(|kind| matches!(kind, CfgValueKind::RealConstant(one) if *one == 1.0)),
            "the generated lowering keeps the event body, which is its whole computation: \
             {cone:?}"
        );
    }

    /// A declared branch nothing contributes to — the one construct the CFG
    /// route would have lowered differently and the screen that is gone used to
    /// name — is refused by the *shipped* route, before a CFG plan is asked
    /// for.
    ///
    /// This is why there is no divergence screen any more, and it is the
    /// assertion that keeps that true. `I(sense)` on a branch with no
    /// contribution and no branch unknown is `CfgValueKind::BranchFlow`, a
    /// runtime-supplied flow that only the generated backend can answer; the
    /// shipped route has no probe registered for it and says so. A module that
    /// starts building here would be one carrying a construct nothing has
    /// compared the two routes on, and this test would go red rather than let
    /// it through quietly.
    #[test]
    fn a_branch_nothing_contributes_to_is_refused_before_a_cfg_plan_is_asked_for() {
        let source = r#"
module cfg_div_branch_flow(p, n);
  inout p, n;
  electrical p, n, s;
  branch (s) sense;
  analog I(p, n) <+ I(sense) * 2.0;
endmodule
"#;
        let (model, artifact) = compile(source);
        let error = build_default_model_plan_reported(&model, &artifact)
            .expect_err("the shipped route cannot lower a probe of a branch with no current");
        let message = error.to_string();
        assert!(
            message.contains("named branch current sense"),
            "the refusal names the branch: {message}"
        );
    }
}
