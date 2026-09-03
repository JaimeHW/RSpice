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
//! [`build_default_model_plan`] is what x64, AArch64 and the WASM JIT now all
//! call, and [`DEFAULT_PLAN_ROUTE`] is the one thing that decides what it
//! builds. On [`PlanRoute::Cfg`] it takes the CFG form of a module's residual,
//! Jacobian and reactive-Jacobian entries, keeps every other field postfix, and
//! falls the whole module back to the postfix plan when the CFG route refuses
//! it. On [`PlanRoute::Postfix`] it calls [`build_model_plan_with_canonical_ir`]
//! and nothing else.
//!
//! **The constant reads `Postfix`.** Everything below is built, tested and
//! pinned; what it is waiting on is the evidence, and [`DEFAULT_PLAN_ROUTE`]
//! carries that argument in full.
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
//! # Why noise stays postfix in the default plan
//!
//! Because the flip that takes it has not run its censuses yet, and for no
//! other reason. Production asks for [`CfgNoiseScope::Postfix`] and the
//! CFG-versus-MIR census asks for [`CfgNoiseScope::Cfg`]; the two switches move
//! together or not at all.
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
//! On [`PlanRoute::Cfg`], a refused module still compiles:
//! [`build_default_model_plan`] returns the postfix plan for it, which is the
//! plan every backend ships today, so a refusal costs accuracy nowhere and
//! coverage nothing. What it must not do is happen quietly — a module that
//! stops taking the CFG route because a pass regressed would otherwise look
//! exactly like one that never took it. So the refusal goes to the same `[JIT]`
//! seam a failed native compile uses, naming the module and the refusal class,
//! and [`build_default_model_plan_reported`] hands it back so a test can pin
//! it. Running the estate with the constant set to `Cfg` therefore censuses the
//! fallback by class, which is how the counts in that lane's report were taken.
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
use super::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use super::expr::NativeOp;
use super::model_plan::NativeModelPlan;
use super::plan_builder::{
    build_model_plan_with_canonical_ir, canonical_branch_unknown_runtime_map,
};
use super::plan_program::{BlockProgram, PlanProgram};
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
    /// Instructions in the largest block entry, with its identity.
    pub(crate) largest_entry: Option<(CfgPlanEntry, usize)>,
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

/// Which lane a shipped Jacobian column stands for.
///
/// The inverse of the seed-list construction below, and the same arithmetic the
/// generated backend's `emit_row` performs when it splits a lane index back
/// into a node column and a branch column.
pub(crate) fn shipped_entry_lane(axis: &ColumnAxis, node_count: usize) -> usize {
    match axis {
        ColumnAxis::Node(node) => *node,
        ColumnAxis::Branch(branch) => node_count + *branch,
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
    /// Lower them from the CFG with everything else. What the CFG-versus-MIR
    /// census asks for, because measuring the gap is how it closes.
    ///
    /// Constructed only there until that lane lands; the attribute is the
    /// honest statement of it rather than a silenced warning.
    #[cfg_attr(not(test), allow(dead_code))]
    Cfg,
    /// Keep the postfix plan's. What production asks for.
    ///
    /// [`CfgPlanRefusal::NoiseUnpaired`] cannot fire under this scope: a
    /// shipped source with no CFG process is not a disagreement about a value
    /// nobody is taking from the CFG.
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
    let node_count = artifact.mir.nodes.len();

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
    let require_lane = |axis: &ColumnAxis,
                        node_count: usize,
                        entry: CfgPlanEntry|
     -> Result<usize, CfgPlanRefused> {
        let lane = shipped_entry_lane(axis, node_count);
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
    let mut stamp_values = Vec::with_capacity(model.stamp_programs.len());
    let mut jacobians = Vec::with_capacity(model.stamp_programs.len());
    let mut reactive_jacobians = Vec::with_capacity(model.stamp_programs.len());
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        // Slice the whole equation out of the body once, then each of its
        // entries out of that slice. Pruning is linear in the function it is
        // given, so slicing per entry from the *model* would cost the entry
        // count times a compact model's whole body — quadratic, and measured:
        // it took asmhemt past twenty gigabytes of working set and into paging
        // before this loop was written this way. Slicing the equation first
        // makes every inner pass linear in the equation instead. The result is
        // the same function either way: pruning is idempotent, and composing
        // two slices keeps exactly what the inner one asked for.
        let residual = cfg.residuals[stamp_index];
        let stamp_entry = CfgPlanEntry::StampValue(stamp_index);
        let mut row_entries: Vec<(CfgPlanEntry, ValueId)> =
            vec![(stamp_entry, scalar(residual, stamp_entry)?)];
        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            let lane = require_lane(&jacobian.col_axis, node_count, entry)?;
            if let Some(value) = jacobian_rows[stamp_index].get(lane).copied().flatten() {
                row_entries.push((entry, scalar(value, entry)?));
            }
        }
        for (entry_index, reactive) in stamp.reactive_jacobians.iter().enumerate() {
            let entry = CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index);
            let lane = require_lane(&reactive.col_axis, node_count, entry)?;
            if let Some(value) = reactive_rows[stamp_index].get(lane).copied().flatten() {
                row_entries.push((entry, scalar(value, entry)?));
            }
        }
        let row_outputs: Vec<ValueId> = row_entries.iter().map(|(_, value)| *value).collect();
        let (row_function, row_mapped) = prune_cfg_to_outputs(&scalarized.function, &row_outputs);
        let row_values: HashMap<CfgPlanEntry, ValueId> = row_entries
            .iter()
            .map(|(entry, _)| *entry)
            .zip(row_mapped)
            .collect();

        stamp_values.push(lower(
            &row_function,
            row_values[&stamp_entry],
            stamp_entry,
            &mut report,
        )?);
        report.stamp_values += 1;

        let mut row = Vec::with_capacity(stamp.jacobian_programs.len());
        for entry_index in 0..stamp.jacobian_programs.len() {
            let entry = CfgPlanEntry::Jacobian(stamp_index, entry_index);
            match row_values.get(&entry).copied() {
                Some(output) => {
                    row.push(lower(&row_function, output, entry, &mut report)?);
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
            match row_values.get(&entry).copied() {
                Some(output) => {
                    reactive_row.push(lower(&row_function, output, entry, &mut report)?);
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

/// Which route builds a plan's `stamp_values`, `jacobians` and
/// `reactive_jacobians`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRoute {
    /// MIR's flat postfix stream, for every field.
    Postfix,
    /// The canonical CFG for those three fields, the postfix stream for the
    /// rest, and the postfix plan entire for a module the CFG route refuses.
    ///
    /// Constructed by the pins below and by anyone who sets
    /// [`DEFAULT_PLAN_ROUTE`] to it — which nothing does yet, so outside
    /// `cfg(test)` this variant has no constructor and the attribute is the
    /// honest statement of that rather than a silenced warning. It comes off
    /// with the commit that flips the constant. Note that only the *variant* is
    /// unconstructed: everything the `Cfg` arm reaches is compiled, warned
    /// about normally, and exercised by the pins.
    #[cfg_attr(not(test), allow(dead_code))]
    Cfg,
}

/// The route production compiles, and the one thing that decides it.
///
/// # It is `Postfix`, and this is the evidence
///
/// W-F3c built the `Cfg` route into the default constructor and then measured
/// it against the estate. Six tests turned red, and not one was rounding: each
/// was a construct the CFG lowering treats differently from MIR, where the CFG
/// plan *builds* and computes another number. `$port_connected` (6.0 against
/// 0.0 on an omitted terminal), `$simparam` (0.0 against a gmin of 1e-12), a
/// contribution-current probe (a Jacobian entry of 4.5 against 2.5), an array
/// variable's declaration initializer, an event-controlled variable across a
/// rejected step (2.0 against 1.0), and `hypot`/`atan2` under a `ddx`, where
/// the scalar derivative rules fall through to a zero.
///
/// Every one of them but the last has since been *closed* rather than
/// screened, and [`a_closed_divergence_takes_the_cfg_route`] is where they
/// moved to. `hypot`/`atan2` is not on that list because it is not a
/// divergence: neither route computes a wrong number for it, the CFG route
/// simply has no rule and refuses, which is [`DERIVATIVE_RULE_HOLES`]. The
/// pattern each followed is the same one: the divergence was a lowering
/// decision one route had and the other did not, and the fix was to give the
/// CFG route the same decision.
///
/// * `$port_connected` folded to `1.0` because the generated backend builds
///   every instance it evaluates. It is a runtime leaf for a backend that
///   cannot promise that —
///   [`CfgModel::from_hir_for_executable_backend`](crate::canonical_ir::CfgModel::from_hir_for_executable_backend)
///   is where the two consumers part company — and `$simparam` with no source
///   now reads the same `simparam_source_default` table on both routes. (W-F4)
/// * A **prologue-only definition** — a localparam or a declaration
///   initializer the body reads — took Verilog-AMS zero because the CFG had no
///   definition of it at all. The executable lowering now evaluates
///   [`HirModel::prologue_statements`](crate::canonical_ir::HirModel) into the
///   entry block. (W-F4b)
/// * An **event-controlled variable** was applied twice: the assignment pass
///   this plan keeps had already run the guard-folded event body into the
///   variable's slot, and the CFG applied it again on top of the leaf that
///   reads that slot, counting 2.0 where the shipped route counts 1.0. A read
///   of one is now the leaf itself, which is the `LoadVariable` the postfix
///   route's own read lowers to. (W-F4b)
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
/// So there is no divergence screen any more, and no refusal class for one.
/// [`DERIVATIVE_RULE_HOLES`] screens what is left, which is a construct one
/// route cannot lower rather than one they lower differently.
///
/// The fallback census, W-F3c through W-F5:
///
/// ```text
///                          W-F3c   W-F4  W-F4b   W-F5
/// lowering                    63     63     63     63
/// cfg-lowering                24     24     24     24
/// known-divergence            28     15     11      0
/// derivative-rule-missing      7      7      7      7
///                            ---    ---    ---    ---
///                            122    109    105     94
/// ```
///
/// W-F4's thirteen were its two closed constructs and the array screen's
/// over-refusal: the prologue screen named a variable rather than counting a
/// module's arrays, so `polysum`, `gsel`, `desc` and `lpsize` — which declare
/// arrays and fill every element inside the analog block — took the CFG route
/// from W-F4 on. W-F4b's four are `cinit`, whose array really is filled by a
/// declaration initializer, and the three event-controlled variables. W-F5's
/// eleven were the whole contribution-current class.
///
/// # The screen was empirical, and that is a bound on what it is worth
///
/// It names what about twelve hundred tests happened to expose, not what a
/// proof would name. W-F4 went looking for what they did not: a module-scope
/// `real g = 4.5;` and a `localparam` read from the body both computed a
/// residual of 0.0 on this route, against 9.0 and 6.0 on `Postfix`, and the
/// whole estate ran green with both silently wrong. Two more, found by asking,
/// on top of the six the flip found by failing. That is the argument for the
/// constant, not the six.
///
/// The instrument that would bound the rest is the forty-three-module
/// CFG-versus-MIR census ([`crate::native::cfg_mir_census`]), which has never
/// run past nine modules.
///
/// That census is now the only thing this constant is waiting on. Every
/// divergence the estate found is closed, the screen and its refusal class are
/// gone, and the estate runs green with the constant reading `Cfg` —
/// `--test-threads=1`, `--features native`, `--no-fail-fast`, fifty-seven test
/// binaries, one target failing and that one the generated-bundle digest stamp,
/// which this lane restamped. What is missing is not a construct; it is the
/// measurement that would bound the constructs no test covers, and W-F5 could
/// not take it: the box had 19.0 GB free against the 24 GB a release corpus
/// census needs, with peer builds running.
///
/// So the switch sits here at `Postfix` until [`crate::native::cfg_mir_census`]
/// runs 43/43 within bound, [`crate::native::cfg_census`]'s zero-deviation pin
/// holds, and [`crate::native::code_identity`]'s digest reproduces. Flipping it
/// is a one-line change and everything behind [`PlanRoute::Cfg`] is live,
/// tested and pinned.
///
/// # What `Postfix` reaches, and why the shipped plan is byte-identical
///
/// The `Postfix` arm of [`build_default_model_plan_reported`] calls
/// [`build_model_plan_with_canonical_ir`] and nothing else — no wrapper, no
/// post-pass, no field replaced afterwards. That constructor, and every
/// function it reaches, is unchanged: the diff touches
/// [`build_model_plan_from_canonical_cfg`] and the items below it in this
/// module, [`crate::jit::plan_program`]'s documentation and one attribute,
/// [`crate::native::ssa`]'s attributes, and the four call sites' choice of
/// entry point. `jit::plan_builder` is untouched. A model therefore compiles to
/// the same plan, and so to the same machine code, as it did before the lane —
/// which is what keeps [`crate::native::code_identity`]'s digest valid.
///
/// W-F4 kept that true through a refactor that could have broken it: the
/// `$simparam` table `crate::native`'s `lower_simparam_intrinsic` folds is now
/// read from `canonical_ir`'s `simparam_source_default` rather than written out
/// twice. The postfix route emits the same `NativeOp::Const` it always did,
/// which is why the digest still reads what it read.
pub(crate) const DEFAULT_PLAN_ROUTE: PlanRoute = PlanRoute::Postfix;

/// The plan every backend compiles: [`DEFAULT_PLAN_ROUTE`]'s.
///
/// A refusal on the [`PlanRoute::Cfg`] route reaches the `[JIT]` log seam a
/// failed native compile already uses, so a module that quietly stopped taking
/// the CFG route would be visible in the same place a module that quietly
/// stopped compiling is.
pub(crate) fn build_default_model_plan(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModelPlan> {
    let (plan, refused) = build_default_model_plan_reported(model, artifact, DEFAULT_PLAN_ROUTE)?;
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

/// [`build_default_model_plan`] for a named route, handing the refusal back
/// instead of logging it.
///
/// The `route` is a parameter rather than a read of [`DEFAULT_PLAN_ROUTE`] for
/// one reason: the tests below pin [`PlanRoute::Cfg`]'s behaviour, and a pin
/// that only exercised whatever the constant happens to say would go vacuously
/// green the moment the constant said `Postfix`. Production passes the
/// constant; the pins pass `Cfg`.
///
/// On the `Cfg` route the fallback rebuilds the postfix plan rather than
/// recovering the one the CFG builder started from and threw away. That is one
/// extra plan build, on the refusal path only, once per module per process —
/// the native cache in [`crate::device`] keys on the module, so no model pays
/// it twice — against a constructor whose refusals stay a single `?` each.
pub(crate) fn build_default_model_plan_reported(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
    route: PlanRoute,
) -> JitResult<(NativeModelPlan, Option<CfgPlanRefused>)> {
    match route {
        // Nothing wraps this call. See [`DEFAULT_PLAN_ROUTE`] on why that is
        // load-bearing rather than incidental.
        PlanRoute::Postfix => Ok((build_model_plan_with_canonical_ir(model, artifact)?, None)),
        PlanRoute::Cfg => {
            match build_model_plan_from_canonical_cfg(model, artifact, CfgNoiseScope::Postfix) {
                Ok(built) => Ok((built.plan, None)),
                Err(refused) => {
                    let plan = build_model_plan_with_canonical_ir(model, artifact)?;
                    Ok((plan, Some(refused)))
                }
            }
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
    use super::{CfgPlanRefusal, PlanRoute, build_default_model_plan_reported};
    use crate::canonical_ir::CanonicalIrArtifact;
    use crate::codegen::CompiledModel;
    use crate::jit::model_plan::NativeModelPlan;
    use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
    use crate::{CompilerOptions, VerilogACompiler};

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

    /// The flip, stated as the shape of one plan built on [`PlanRoute::Cfg`]:
    /// that route owns the residual and both Jacobians, the MIR route owns
    /// everything else.
    #[test]
    fn the_cfg_route_takes_its_values_from_the_cfg_and_its_noise_from_mir() {
        let (model, artifact) = compile(RESISTOR_WITH_CHARGE_AND_NOISE);
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
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
                "stamp_values" | "jacobians" | "reactive_jacobians" => "block",
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
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
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
        let (plan, refused) = build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
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
            // run whatever [`DEFAULT_PLAN_ROUTE`] says.
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
                build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
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
    /// in `native_contract` measures the matrix itself, on whichever route
    /// [`DEFAULT_PLAN_ROUTE`] names.
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
                build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
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
        let error = build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
            .expect_err("the shipped route cannot lower a probe of a branch with no current");
        let message = error.to_string();
        assert!(
            message.contains("named branch current sense"),
            "the refusal names the branch: {message}"
        );
    }
}
