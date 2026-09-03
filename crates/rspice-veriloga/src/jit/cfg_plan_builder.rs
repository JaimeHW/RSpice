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
//! Not conservatism: the two routes hold *different quantities* there, and the
//! difference is measured rather than argued. `noise_process_schema >= 1` on
//! thirty-four of the forty-three shipped modules, and under that schema the
//! routing amplitude is folded into the grouped complex injection instead of
//! into the PSD, so the shipped `psd_program` and the CFG process's `psd` stop
//! being the same number — `angelov`'s `noise_exponents[9]` reads 2.0 through
//! MIR and 0.0 through the CFG, `bsimsoi_va`'s `noise_psd[1]` reads 7.19e-28
//! against 0.0. Taking the CFG's noise today would change what the runtime
//! injects across most of the corpus, which is exactly the silently-wrong class
//! this program refuses. Closing the CFG noise slice is its own lane; until it
//! lands, production asks for [`CfgNoiseScope::Postfix`] and the CFG-versus-MIR
//! census asks for [`CfgNoiseScope::Cfg`] so the gap stays measured.
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
//! What is *not* a refusal is a difference the two routes are entitled to:
//! grouped noise, where the shipped PSD program and the CFG process hold
//! different quantities on purpose. That is recorded on the report
//! ([`CfgPlanReport::grouped_noise`]) for the census to classify, because
//! refusing it would refuse thirty-four of the forty-three shipped modules and
//! take every entry they *do* agree on down with them.
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
    AdSeed, CanonicalIrArtifact, CfgBinaryOp, CfgFunction, CfgStateAllocation, CfgValueKind,
    MirModel, ValueId, differentiate, prune_cfg_to_outputs,
};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::codegen::{ColumnAxis, CompiledModel};
use crate::rust_backend::canonical::stored_charges;
use smol_str::SmolStr;

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
    /// The module contains a construct the two routes are known to disagree
    /// about, so the CFG plan would build and be wrong.
    ///
    /// See [`route_divergence`] for the list, each entry with the measurement
    /// that put it there.
    KnownDivergence,
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
            Self::KnownDivergence => "known-divergence",
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
    /// Whether this module's noise reaches the runtime as a grouped complex
    /// injection.
    ///
    /// It changes what the shipped `psd_program` *is*. Under the flat schema it
    /// is the syntactic noise power, which is also what the CFG process's `psd`
    /// value holds; under the grouped schema the routing amplitude is folded
    /// into the injection instead, and the two are no longer the same quantity.
    /// Recorded rather than refused, because refusing here would refuse almost
    /// the whole shipped corpus — measured: thirty-four of the forty-three
    /// modules carry schema 1 — and the entries that *are* the same quantity on
    /// those modules would go unmeasured with them.
    ///
    /// Written on every build and read only by the census, because production
    /// asks for [`CfgNoiseScope::Postfix`] and so takes no noise from the CFG
    /// to classify. The lane that closes the CFG noise slice is what gives this
    /// a production reader.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) grouped_noise: bool,
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

/// The construct in this module the two routes are known to disagree about, if
/// there is one.
///
/// # Why a list rather than an argument
///
/// The CFG is a second lowering of the same source, and the two lowerings do
/// not agree everywhere. Where they disagree the CFG plan still *builds* — it
/// simply computes a different number — so nothing below this function can
/// catch it. Each entry here was found by an estate test that the flip turned
/// red, and each one is a wrong Jacobian or a wrong residual, not a rounding
/// difference.
///
/// Two entries have come off the list, and how they came off is the pattern the
/// rest are meant to follow — the divergence was a lowering decision one route
/// had and the other did not, and the fix was to give the CFG route the same
/// decision rather than a better screen. `$port_connected` folded to `1.0`
/// because the generated backend builds every instance it evaluates;
/// [`CfgModel::from_hir_for_executable_backend`](crate::canonical_ir::CfgModel::from_hir_for_executable_backend)
/// is where a backend that cannot promise that says so, and the fold stays
/// where it is earned. `$simparam` with no source fallback took zero where the
/// bytecode route folded the language's default; both now read
/// `simparam_source_default`. What remains:
///
/// * **A contribution-current probe.** `I(p, mid)` read inside another
///   contribution: the MIR route reads the current the other equation wrote and
///   treats it as frozen, and the CFG route inlines that equation and
///   differentiates through it, so the two Jacobians are different matrices —
///   `native_device_stamps_internal_node_current_probe_alias_jacobian_without_fallback`
///   measured 4.5 against 2.5 on one entry. Detected as a shipped value entry
///   with a non-empty contribution-current read set, which is what "this entry
///   probes a current" means in the plan.
/// * **A prologue-only definition.** A localparam, or a module-scope variable
///   with a declaration initializer: the MIR route's assignment pass runs it and
///   its value entries read the slot it wrote, and the CFG, built from the
///   analog body alone, has no definition of the variable at all. See
///   [`prologue_only_definition`], which also records the two shapes of this the
///   estate did not cover.
/// * **An event-controlled variable.** The CFG recomputes a variable inline
///   where the postfix plan reads the slot the assignment pass wrote, and for a
///   variable whose value is an accepted event state those are different
///   quantities across a rejected step —
///   `checkpoint_before_acceptance_excludes_step_event_variable_candidates`
///   measured 2.0 against 1.0.
///
/// # This list is empirical, and that is a bound on what it is worth
///
/// It names what the estate's tests exposed, not what a proof of the CFG
/// route's soundness would name. The instrument that would bound the rest is
/// the forty-three-module CFG-versus-MIR census, which has never run past nine
/// modules. Until it does, a construct no test covers can still diverge, and
/// this function will not say so.
fn route_divergence(
    hir: &crate::canonical_ir::HirModel,
    function: &CfgFunction,
    postfix: &NativeModelPlan,
) -> Option<String> {
    if let Some(name) = prologue_only_definition(hir) {
        return Some(format!(
            "'{name}' is defined only by a module prologue statement, which the MIR route's \
             assignment pass runs and the CFG has no definition of at all"
        ));
    }
    let dependencies = &postfix.current_dependencies;
    let flat = |rows: &[Vec<usize>]| rows.iter().any(|set| !set.is_empty());
    let nested = |rows: &[Vec<Vec<usize>>]| {
        rows.iter()
            .flatten()
            .any(|set: &Vec<usize>| !set.is_empty())
    };
    let probes_a_current = !dependencies.assignment_current_pairs.is_empty()
        || !dependencies.assignment_prior_currents.is_empty()
        || !dependencies.post_assignment_current_pairs.is_empty()
        || !dependencies.post_assignment_prior_currents.is_empty()
        || flat(&dependencies.stamp_values)
        || flat(&dependencies.stamp_value_prior_currents)
        || nested(&dependencies.jacobians)
        || nested(&dependencies.jacobian_prior_currents)
        || nested(&dependencies.reactive_jacobians)
        || nested(&dependencies.reactive_jacobian_prior_currents);
    if probes_a_current {
        return Some(
            "a shipped value entry probes a contribution current, which the MIR route freezes \
             and the CFG route differentiates through"
                .to_string(),
        );
    }
    function.values.iter().find_map(|value| match value.kind {
        CfgValueKind::BranchFlow(_) => Some(
            "a branch-flow probe is frozen by the MIR route and differentiated through by the \
             CFG route"
                .to_string(),
        ),
        CfgValueKind::EventState(_) => Some(
            "an event-controlled variable is read from its runtime slot by the MIR route and \
             recomputed inline by the CFG route"
                .to_string(),
        ),
        _ => None,
    })
}

/// A variable this module defines only in its prologue, if the body reads one.
///
/// The general form of what was recorded here as "an array variable", and the
/// array was only the case an estate test happened to cover.
/// [`crate::canonical_ir::HirExecutedCorrespondence`] states the shape:
/// "module prologue statements (localparam and module-scope variable
/// initializers, `$bound_step` resets) exist only in the executed copy and pair
/// with nothing in the body". The MIR route's assignment pass runs them and its
/// value entries read the slots they wrote. The CFG is built from the body
/// alone, so it has no definition of such a variable and a read of one falls
/// through to Verilog-AMS zero initialisation — a wrong number, not a refusal.
///
/// W-F4 measured three shapes of it on the `Cfg` route, each against the same
/// module on `Postfix`:
///
/// * an array declaration initializer, `real c[0:2] = '{...}` — the entry this
///   screen used to name, found by `assignment_pattern_initializer_fills_elements`;
/// * a scalar declaration initializer, `real g = 4.5;` — a residual of 0.0
///   against 9.0 at `V(p, n) = 2`;
/// * a `localparam` read from the body, `localparam real K = 3.0; g = K;` — a
///   residual of 0.0 against 6.0.
///
/// The last two were not screened and no estate test covered them, which is the
/// same lesson [`route_divergence`] records about its own list: it names what
/// the estate exposed. See `a_prologue_only_definition_falls_the_module_back`.
///
/// # The read test is by name, over both copies
///
/// [`crate::canonical_ir::HirModel`]'s expression arena holds the executed copy
/// and the structured body together, so a name found in it may have been read
/// by another prologue statement rather than by the body. That over-refuses a
/// module whose prologue variable the body never mentions — `lpsize`, whose
/// `localparam integer SIZE` survives only in an array bound the analyzer has
/// already folded — and it does not under-refuse, which is the direction a
/// screen has to be wrong in.
/// Whether `name` is a variable the analyzer minted for the executed copy
/// rather than one the source declared.
///
/// Both spellings are reserved: `$bound_step` is a task variable, and no source
/// identifier begins with `$`; `__guardN` is the snapshot
/// `SemanticAnalyzer::stabilize_condition` takes of a guard expression, minted
/// under a counter.
///
/// Neither is a divergence, and that is a statement about what they are for. A
/// guard snapshot exists because the executed copy has no control flow to carry
/// a condition in — the structured body does, so the CFG derives the same
/// condition from the branch it is a condition *of*, which is
/// [`crate::canonical_ir::HirExecutedCorrespondence`]'s "`selector == match`
/// against `__guardN == match`" in the other direction. `$bound_step` bounds the
/// next timestep and appears in no residual. Without this, sixteen of the
/// estate's modules refused on a variable the CFG is right not to have.
fn analyzer_synthesized(name: &str) -> bool {
    name.starts_with('$') || name.starts_with("__guard")
}

fn prologue_only_definition(hir: &crate::canonical_ir::HirModel) -> Option<SmolStr> {
    fn assigned(statements: &[crate::canonical_ir::hir::HirStatement], into: &mut HashSet<u32>) {
        for statement in statements {
            match statement {
                crate::canonical_ir::hir::HirStatement::Assignment(assignment) => {
                    into.insert(assignment.target.index());
                }
                crate::canonical_ir::hir::HirStatement::Loop(body) => assigned(&body.body, into),
            }
        }
    }
    fn assigned_in_body(regions: &[crate::canonical_ir::hir::HirRegion], into: &mut HashSet<u32>) {
        for region in regions {
            match region {
                crate::canonical_ir::hir::HirRegion::Assignment(assignment) => {
                    into.insert(assignment.target.index());
                }
                crate::canonical_ir::hir::HirRegion::Conditional {
                    then_body,
                    else_body,
                    ..
                } => {
                    assigned_in_body(then_body, into);
                    assigned_in_body(else_body, into);
                }
                crate::canonical_ir::hir::HirRegion::Loop { body, .. } => {
                    assigned_in_body(body, into)
                }
                crate::canonical_ir::hir::HirRegion::Contribution(_) => {}
            }
        }
    }

    let mut prologue = HashSet::new();
    assigned(&hir.statements, &mut prologue);
    let mut body = HashSet::new();
    assigned_in_body(&hir.body, &mut body);
    prologue.retain(|target| !body.contains(target));
    if prologue.is_empty() {
        return None;
    }

    // Both spellings of a read: an identifier, and an array access whose index
    // the analyzer did not fold to an element variable.
    let read: HashSet<&SmolStr> = hir
        .expressions
        .iter()
        .filter_map(|expression| match &expression.kind {
            crate::canonical_ir::hir::HirExprKind::Identifier { name } => Some(name),
            crate::canonical_ir::hir::HirExprKind::ArrayAccess { array, .. } => Some(array),
            _ => None,
        })
        .collect();
    // An array element is a variable of its own, named `c[0]`, so a read of one
    // is found either under that name or under the array's.
    let array_of = |target: u32| {
        hir.arrays.iter().find(|array| {
            let base = array.base.index();
            target >= base && target < base + array.len
        })
    };
    hir.variables
        .iter()
        .filter(|variable| prologue.contains(&variable.id.index()))
        .filter(|variable| !analyzer_synthesized(&variable.name))
        .find_map(|variable| {
            if read.contains(&variable.name) {
                return Some(variable.name.clone());
            }
            array_of(variable.id.index())
                .filter(|array| read.contains(&array.name))
                .map(|array| array.name.clone())
        })
}

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
    if let Some(divergence) = route_divergence(&artifact.hir, &cfg.function, &plan) {
        return Err(refuse(CfgPlanRefusal::KnownDivergence, divergence));
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

    let mut report = CfgPlanReport {
        grouped_noise: model.noise_process_schema >= 1 && !model.noise_sources.is_empty(),
        ..CfgPlanReport::default()
    };
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
                let lower_noise = |value: ValueId,
                                   entry: CfgPlanEntry,
                                   report: &mut CfgPlanReport|
                 -> Result<PlanProgram, CfgPlanRefused> {
                    match lower(&cfg.function, value, entry, report) {
                        Ok(program) => Ok(program),
                        Err(primal) if primal.class == CfgPlanRefusal::Lowering => {
                            let Some(resolved) = scalarized.scalar(value) else {
                                return Err(primal);
                            };
                            let program = lower(&scalarized.function, resolved, entry, report)?;
                            report.noise_from_differentiated += 1;
                            Ok(program)
                        }
                        Err(other) => Err(other),
                    }
                };
                noise_psd.push(lower_noise(
                    process.psd,
                    CfgPlanEntry::NoisePsd(source_index),
                    &mut report,
                )?);
                report.noise_values += 1;
                let exponent = match (process.exponent, source.exponent_program.as_ref()) {
                    (Some(exponent), Some(_)) => {
                        report.noise_values += 1;
                        Some(lower_noise(
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
/// W-F4 *closed* the first two rather than screening them, and
/// [`a_closed_divergence_takes_the_cfg_route`] is where they moved to:
/// `$port_connected` is a runtime leaf for a backend that evaluates instances
/// it did not build, and `$simparam` reads the same default table on both
/// routes. [`route_divergence`] and [`DERIVATIVE_RULE_HOLES`] screen the other
/// four, and with them the estate is green — 2080 tests, `--test-threads=1`,
/// `--features native`, exit 0 with this constant reading `Cfg`.
///
/// The fallback census taken on that run, against W-F3c's:
///
/// ```text
///                          W-F3c   W-F4
/// lowering                    63     63
/// cfg-lowering                24     24
/// known-divergence            28     15
/// derivative-rule-missing      7      7
///                            ---    ---
///                            122    109
/// ```
///
/// The thirteen are the two closed constructs and the array screen's
/// over-refusal: [`prologue_only_definition`] names a variable rather than
/// counting a module's arrays, so `polysum`, `gsel`, `desc` and `lpsize` — which
/// declare arrays and fill every element inside the analog block — take the CFG
/// route now. What is left under `known-divergence` is eleven
/// contribution-current probes, three event-controlled variables, and `cinit`,
/// whose array really is filled by a declaration initializer.
///
/// # The screen is empirical, and W-F4 measured what that costs
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
/// So the switch sits here at `Postfix` until that census runs 43/43 and the
/// remaining classes are closed rather than screened. Flipping it is a one-line
/// change and everything behind [`PlanRoute::Cfg`] is live, tested and pinned.
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
    /// The pin the closed entries move to when they come off
    /// [`route_divergence`]. Refusing one of these again — by reinstating a
    /// screen, or by a lowering change that makes the construct unbuildable —
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

    /// Each construct in [`route_divergence`] falls the module back, and says
    /// which construct did it.
    ///
    /// One source per entry rather than one source carrying all of them, so a
    /// screen that stopped working is attributed rather than covered for by the
    /// next one in the list.
    #[test]
    fn every_known_divergence_falls_the_module_back() {
        let cases: &[(&str, &str, &str)] = &[
            (
                "array-initializer",
                "'c[0]' is defined only by a module prologue statement",
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
                "'g' is defined only by a module prologue statement",
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
                "'k' is defined only by a module prologue statement",
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
            (
                "current-probe",
                "contribution current",
                r#"
module cfg_div_probe(p, n);
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
        ];
        for (case, expected, source) in cases {
            let (model, artifact) = compile(source);
            let (plan, refused) =
                build_default_model_plan_reported(&model, &artifact, PlanRoute::Cfg)
                    .unwrap_or_else(|error| panic!("{case}: the default plan builds: {error}"));
            let refused =
                refused.unwrap_or_else(|| panic!("{case}: the CFG route must refuse this module"));
            assert_eq!(refused.class, CfgPlanRefusal::KnownDivergence, "{case}");
            assert!(
                refused.detail.contains(expected),
                "{case}: {} names the construct",
                refused.detail
            );
            for (field, form) in forms(&plan) {
                assert_eq!(form, "postfix", "{case}: {field} stay postfix");
            }
        }
    }
}
