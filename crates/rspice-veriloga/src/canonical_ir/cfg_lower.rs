//! Lowering the structured HIR body into a [`CfgFunction`].
//!
//! This is the pass that preserves structured guards. Where the former scalar lowering
//! reconstructs "the value a variable had before this `if`" by searching an
//! assignment history, this one asks the SSA builder — which knows, exactly,
//! because it never threw the block structure away.
//!
//! ## Contributions are accumulators
//!
//! `I(a,b) <+ x` becomes `residual += x` against a per-contribution SSA
//! variable, so a contribution inside an `if` is an update on one edge of a
//! diamond and the join merges it with the untouched running total. That is
//! both what a hand-written model does and what makes the "contributes zero
//! when the branch is not taken" rule fall out of ordinary SSA rather than
//! needing a rule of its own.
//!
//! ## Conditional expressions become diamonds
//!
//! `c ? a : b` lowers to a branch, because [`CfgValueKind`] has no select and
//! because evaluating only the taken side is the entire point. `min`/`max` are
//! the deliberate exception: they stay single operations, since a diamond per
//! `min` in a BSIM-class model would swamp the block count for no gain.
//!
//! ## Reporting rather than guessing
//!
//! A construct this pass cannot yet lower produces a diagnostic and a zero
//! placeholder, and the walk continues. That yields the complete inventory of
//! what a model still needs in one run instead of one construct per run, and it
//! is the reason [`CfgModel::from_hir`] returns every diagnostic it found rather
//! than the first.

use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use crate::disciplines::is_standard_flow_access;

use super::cfg::{
    CfgBinaryOp, CfgDdxAxis, CfgFunction, CfgIntegerBitwiseOp, CfgLaplaceTransfer, CfgTerminator,
    CfgUnaryOp, CfgValueKind, CfgValueType, CfgVariable, CfgZiPolynomial, SsaBuilder,
};
use super::hir::{
    HirAnalogOperator, HirAssignment, HirContribution, HirContributionKind, HirExprKind,
    HirExpression, HirLimiterArgument, HirModel, HirRegion, HirStatement,
};
use super::mir::{MirEquationKind, MirModel};
use super::noise::{contains_noise, is_noise_call, string_literal};
use super::{
    BlockId, BranchId, BranchUnknownId, CanonicalNoiseSourceKind, CompilerPhase, ContributionId,
    DiagnosticSeverity, ExprId, IrDiagnostic, NodeId, ParamId, SourceSpanRef, ValueId, VariableId,
};

/// A module lowered to a single control-flow graph.
#[derive(Debug, Clone, PartialEq)]
pub struct CfgModel {
    pub module_name: SmolStr,
    pub function: CfgFunction,
    /// The accumulated residual of each contribution at the exit block, indexed
    /// by [`ContributionId`]. Parallel to `MirModel::equations`.
    pub residuals: Vec<ValueId>,
    /// Topology activation after projecting each potential contribution's
    /// leading instance-static guard prefix. Parallel to [`Self::residuals`]
    /// and the HIR contribution list.
    ///
    /// This is deliberately independent of both the residual value and whether
    /// runtime control reached the statement. In particular,
    /// `if (mode) V(a, b) <+ 0` distinguishes an instance-static false/open
    /// branch from an active ideal zero-volt source, while a contribution below
    /// a bias-, time-, or state-dependent guard remains topology-active even on
    /// an untaken path.
    pub activations: Vec<Option<ValueId>>,
    /// Final values of event-controlled procedural variables at function exit,
    /// in dense accepted-state slot order.
    pub event_state_candidates: Vec<ValueId>,
    /// Every noise source the body writes, in the order the body writes them.
    ///
    /// Kept apart from `residuals` because noise contributes nothing to the
    /// time-domain equations: these are the small-signal powers, evaluated at
    /// whatever operating point the same body just computed.
    pub noise: Vec<CfgNoiseSource>,
    /// Raw syntactic noise processes from every source-order body position,
    /// including assignment origins. Unlike `noise`, routing amplitude is not
    /// folded into these values; grouped complex injections carry it.
    pub noise_processes: Vec<CfgNoiseProcess>,
    /// Everything the lowering wanted to say that did not stop it. Carried on
    /// the model rather than dropped, because a model that lowered *and* warned
    /// is exactly the case worth surfacing.
    pub warnings: Vec<IrDiagnostic>,
}

/// One noise source, as values the body already computes.
///
/// Every field is read at the exit block, so each is the merge of what the
/// source's own site assigned with the zero it started at. That is what makes a
/// source under an untaken `if` inactive, and it costs no separate activation
/// expression: the guard is the control flow the source was written in, and SSA
/// construction already knows it.
#[derive(Debug, Clone, PartialEq)]
pub struct CfgNoiseSource {
    /// The contribution this source was written in, and which of that
    /// contribution's sources it is, counting in source order. Together they
    /// name the [`super::CanonicalNoiseSource`] this corresponds to, which
    /// nothing else can do: the plan is extracted from a second lowering of the
    /// same expressions, so it shares no expression ids with the body.
    pub contribution: ContributionId,
    pub ordinal: usize,
    pub kind: CanonicalNoiseSourceKind,
    pub log_interp: bool,
    pub label: Option<SmolStr>,
    /// Nonzero exactly when control reached the source.
    pub active: ValueId,
    /// Already multiplied by the square of the amplitude the source was scaled
    /// by, matching what the plan folds into its own `psd`.
    pub psd: ValueId,
    pub exponent: Option<ValueId>,
    pub table: Vec<ValueId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgNoiseProcess {
    pub process_id: u32,
    pub kind: CanonicalNoiseSourceKind,
    pub log_interp: bool,
    pub label: Option<SmolStr>,
    pub active: ValueId,
    pub psd: ValueId,
    pub exponent: Option<ValueId>,
    pub table: Vec<ValueId>,
    /// The same magnitudes as the *site* computed them, with no exit merge over
    /// the control flow that reached it.
    ///
    /// `psd` and `exponent` above are read at the exit block, so a process the
    /// body did not reach reads back the zero its variables were seeded with —
    /// the activation guard is folded into the magnitude. That is the right
    /// quantity for a consumer that reads [`Self::active`] beside it, which the
    /// generated backend does.
    ///
    /// An executable plan has no such field: its runtime decides a source's
    /// activity from the injection gains, and its shipped `psd_program` is the
    /// magnitude expression evaluated *unconditionally*. Reproducing that
    /// number is what these are for. See
    /// [`crate::jit::cfg_plan_builder`]'s noise slice.
    ///
    /// `None` for every lowering but the executable one. They are outputs, so
    /// they are liveness roots, and a root the generated backend does not read
    /// would still change which statements its noise schedule selects — the
    /// forty-three checked-in devices are frozen under a `bundle_digest`.
    pub site: Option<CfgNoiseProcessSite>,
}

/// [`CfgNoiseProcess`]'s magnitudes before the exit merge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CfgNoiseProcessSite {
    pub psd: ValueId,
    pub exponent: Option<ValueId>,
}

impl CfgModel {
    /// Lower `hir`'s structured body, using `mir` for name-to-id resolution.
    ///
    /// Both are needed: the body lives in HIR because that is where control
    /// flow survives, and node, branch, and branch-unknown identity is settled
    /// in MIR. Nothing is recomputed here that MIR already decided.
    pub fn from_hir(hir: &HirModel, mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        Self::from_hir_with_mode(hir, mir, CfgLowerMode::GENERATED)
    }

    /// Lower `hir` for a backend that evaluates instances it did not build.
    ///
    /// The one thing this changes is `$port_connected`, and it changes it
    /// because the guarantee behind the constant fold is not a property of the
    /// language: a generated device is instantiated with exactly the terminals
    /// its descriptor declares — `veriloga_builtins::instantiate` refuses any
    /// other count — so every port it can be asked about is connected, and
    /// folding the query to `1.0` deletes every guard in the shipped compact
    /// models. `VerilogADevice` has no such guarantee. It marks terminal `i`
    /// connected only while `i < supplied_terminals`, so an instance that omits
    /// a trailing terminal takes the other arm of every one of those guards.
    ///
    /// So the fold is an optimization one consumer has earned and the other has
    /// not, and this is where the two part company rather than a mode the CFG
    /// level is confused about.
    ///
    /// Gated with its only caller, [`crate::jit`]: a build with neither JIT
    /// compiles no executable backend, so this entry point has no consumer to
    /// be alive for.
    #[cfg(any(feature = "native", feature = "wasm-jit"))]
    pub(crate) fn from_hir_for_executable_backend(
        hir: &HirModel,
        mir: &MirModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        Self::from_hir_with_mode(hir, mir, CfgLowerMode::EXECUTABLE)
    }

    /// Lower only the control-flow and reaching definitions needed to
    /// evaluate raw grouped-noise metadata. Contribution values are traversed
    /// structurally for noise sites, so a supported routing operator such as
    /// `absdelay(noise, ...)` does not require the ordinary residual CFG to
    /// implement that operator. Ordinary [`Self::from_hir`] remains strict.
    pub(crate) fn noise_metadata_from_hir(
        hir: &HirModel,
        mir: &MirModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        Self::from_hir_with_mode(hir, mir, CfgLowerMode::NOISE_METADATA)
    }

    fn from_hir_with_mode(
        hir: &HirModel,
        mir: &MirModel,
        mode: CfgLowerMode,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        let mut lowerer = CfgLowerer::new(hir, mir, mode);
        let (function, residuals, activations, event_state_candidates, noise, noise_processes) =
            lowerer.lower()?;
        // Errors only. A warning that failed the lowering would be an error
        // wearing a different word.
        if lowerer
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return Err(lowerer.diagnostics);
        }
        Ok(Self {
            module_name: hir.module_name.clone(),
            function,
            residuals,
            activations,
            event_state_candidates,
            noise,
            noise_processes,
            warnings: lowerer.diagnostics,
        })
    }
}

/// Re-attach each source to its values once finishing has renumbered them.
///
/// `outputs` is the flattening of every source's variables in the order
/// [`PendingNoise::variables`] yields them, which is the order they were pushed
/// in, so the two walks stay in step by construction.
fn resolve_noise(pending: Vec<PendingNoise>, outputs: &[ValueId]) -> Vec<CfgNoiseSource> {
    let mut next = outputs.iter().copied();
    let mut sources = Vec::with_capacity(pending.len());
    for source in pending {
        let mut take = || next.next().expect("an output for every noise variable");
        let active = take();
        let psd = take();
        let exponent = source.exponent.map(|_| take());
        let table = source.table.iter().map(|_| take()).collect();
        sources.push(CfgNoiseSource {
            contribution: source.contribution,
            ordinal: source.ordinal,
            kind: source.kind,
            log_interp: source.log_interp,
            label: source.label,
            active,
            psd,
            exponent,
            table,
        });
    }
    sources
}

/// Re-attach each process to its values once finishing has renumbered them.
///
/// `outputs` is every process's variables in `variables()` order, then every
/// process's site values in `site_values()` order — two runs rather than one
/// interleaving, because the site values are only present for one lowering
/// mode and splicing them in per process would make the two walks disagree the
/// moment one of them forgot the flag.
fn resolve_noise_processes(
    pending: Vec<PendingNoiseProcess>,
    outputs: &[ValueId],
    site_outputs: &[ValueId],
) -> Vec<CfgNoiseProcess> {
    let mut next = outputs.iter().copied();
    let mut next_site = site_outputs.iter().copied();
    pending
        .into_iter()
        .map(|process| {
            let mut take = || {
                next.next()
                    .expect("an output for every noise-process variable")
            };
            let mut take_site = || {
                next_site
                    .next()
                    .expect("an output for every noise-process site value")
            };
            CfgNoiseProcess {
                process_id: process.process_id,
                kind: process.kind,
                log_interp: process.log_interp,
                label: process.label,
                active: take(),
                psd: take(),
                exponent: process.exponent.map(|_| take()),
                table: process.table.iter().map(|_| take()).collect(),
                site: process.site.map(|site| CfgNoiseProcessSite {
                    psd: take_site(),
                    exponent: site.exponent.map(|_| take_site()),
                }),
            }
        })
        .collect()
}

/// Which halves of a filter's transfer function are named by their roots.
///
/// Two independent facts rather than a four-way enum, because that is what the
/// spellings are: the `z`/`n` and the `p`/`d` in `laplace_zd` decide the two
/// halves separately, and the same pair of letters means the same thing in the
/// `zi_*` family. Saying it this way is what lets both lowerings ask their one
/// real question - is this half a root list - once per half.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FilterForm {
    numerator_is_roots: bool,
    denominator_is_roots: bool,
}

impl FilterForm {
    /// The form an operator's name spells, if it names one.
    fn from_operator(name: &str) -> Option<Self> {
        let (numerator, denominator) = match name.rsplit_once('_')?.1 {
            "zp" => (true, true),
            "zd" => (true, false),
            "np" => (false, true),
            "nd" => (false, false),
            _ => return None,
        };
        Some(Self::new(numerator, denominator))
    }

    fn new(numerator_is_roots: bool, denominator_is_roots: bool) -> Self {
        Self {
            numerator_is_roots,
            denominator_is_roots,
        }
    }
}

impl From<(bool, bool)> for FilterForm {
    fn from((numerator_is_roots, denominator_is_roots): (bool, bool)) -> Self {
        Self::new(numerator_is_roots, denominator_is_roots)
    }
}

/// Whether replacing every noise source by its large-signal value zero makes
/// this expression identically zero. Used only to prove that substituting a
/// routing/filter result with zero in the metadata-only assignment stream is
/// exact; an unproved mixed deterministic/noise value fails closed.
fn noise_substitution_is_zero(hir: &HirModel, id: ExprId) -> bool {
    let Some(expression) = hir.expressions.get(usize::from(id)) else {
        return false;
    };
    let zero = |child| noise_substitution_is_zero(hir, child);
    match &expression.kind {
        HirExprKind::NoiseSource { .. } => true,
        HirExprKind::Unary { op, operand } if matches!(op.as_str(), "Neg" | "Pos") => {
            zero(*operand)
        }
        HirExprKind::Binary { op, left, right } => match op.as_str() {
            "Add" | "Sub" => zero(*left) && zero(*right),
            "Mul" => zero(*left) || zero(*right),
            "Div" => zero(*left) && !contains_noise(hir, *right),
            _ => false,
        },
        HirExprKind::Conditional {
            then_expr,
            else_expr,
            ..
        } => zero(*then_expr) && zero(*else_expr),
        HirExprKind::AnalogOperator { op } => match op {
            HirAnalogOperator::Limit { .. } | HirAnalogOperator::LimiterArgument { .. } => false,
        },
        _ => false,
    }
}

/// A leaf value's identity, for interning.
///
/// Constants and unknowns are read thousands of times in a BSIM-class model and
/// there is no reason for each read to define a new value; the derivative pass
/// would then carry thousands of identical lanes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LeafKey {
    RealConstant(u64),
    Parameter(ParamId),
    ParameterGiven(ParamId),
    PortConnected(u32),
    EventState(u32),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis(SmolStr),
    NodePotential(NodeId),
    BranchFlow(BranchId),
    BranchUnknownFlow(BranchUnknownId),
    ContributedCurrent {
        pos: Option<NodeId>,
        neg: Option<NodeId>,
        through: ContributionId,
    },
    NoiseProcess(u32),
}

struct CfgLowerer<'a> {
    hir: &'a HirModel,
    mir: &'a MirModel,
    builder: SsaBuilder,
    /// Where instructions are appended right now. Moves as blocks are created,
    /// which is why nothing may assume it still equals the block it started in.
    block: BlockId,
    variables_by_name: HashMap<SmolStr, VariableId>,
    parameters_by_name: HashMap<SmolStr, ParamId>,
    /// Guard expressions whose reaching local definitions are instance-static
    /// at that exact program point. These may participate in the leading guard
    /// prefix that controls potential-branch topology.
    static_guard_conditions: HashSet<ExprId>,
    nodes_by_name: HashMap<SmolStr, NodeId>,
    ground_names: HashSet<SmolStr>,
    leaves: HashMap<LeafKey, ValueId>,
    /// How many conditional-expression results have been given SSA keys.
    temporary_count: usize,
    /// `$limit` calls whose inlined body is currently being walked.
    limiters: Vec<Limiter>,
    /// Noise sources found so far, each still holding the SSA variables its
    /// values are carried in rather than the values themselves.
    noise: Vec<PendingNoise>,
    noise_processes: Vec<PendingNoiseProcess>,
    /// Scoped mode used only by the grouped-noise metadata slicer. It never
    /// changes ordinary canonical residual lowering or its diagnostics.
    noise_metadata_only: bool,
    /// Whether each noise process publishes its site magnitudes as well as its
    /// exit-merged ones. See [`CfgNoiseProcess::site`].
    noise_site_values: bool,
    /// Whether `$port_connected` is a runtime leaf rather than the constant
    /// `1.0`. See [`CfgModel::from_hir_for_executable_backend`].
    per_instance_ports: bool,
    /// Whether the module prologue is evaluated into the entry block. See
    /// [`Self::prologue`].
    lower_prologue: bool,
    /// See [`CfgLowerMode::frozen_event_state`].
    frozen_event_state: bool,
    /// The entry block's `EventState` leaf for each event-controlled variable,
    /// when that leaf is what a read of the variable means. Empty unless
    /// [`Self::frozen_event_state`].
    frozen_event_states: HashMap<VariableId, ValueId>,
    /// See [`CfgLowerMode::frozen_contribution_current`].
    frozen_contribution_current: bool,
    /// The current contributions the walk has already completed, in walk order.
    ///
    /// Walk order, not reachability: a contribution under an untaken guard has
    /// still *happened* as far as a probe below it is concerned, because the
    /// storage the probe reads holds that contribution's slot either way — zero
    /// when the guard did not take. That is what the shipped route does, whose
    /// prior-current probe set grows by equation index and knows nothing about
    /// guards, and matching it here is the whole point.
    completed_current_contributions: Vec<ContributionId>,
    /// Whether the walk is inside a noise process's magnitude operands, where a
    /// flow probe reads the *settled* branch current rather than the running
    /// sum at this point in the body.
    ///
    /// A noise magnitude is not evaluated where it is written. The consumer
    /// this matters to runs the whole large-signal body first, caches each
    /// contribution's current — `populate_noise_current_probe_cache` in
    /// [`crate::device`] — and only then evaluates the magnitudes, so `I(br)`
    /// inside one means the operating point's branch current and nothing else.
    /// Lowering it as the running sum reads the *entry block's seeded zero*
    /// whenever the source sits in the contribution to its own branch, which is
    /// the `white_noise(2 * `q` * abs(I(p, n)))` shot-noise idiom every compact
    /// model writes: the magnitude then evaluates to zero and the analysis
    /// drops the source before it is seen.
    ///
    /// So a probe here names the branch's *last* current contribution rather
    /// than the last one the walk has completed. Both are
    /// [`CfgValueKind::ContributedCurrent`] and both translate through
    /// [`CfgRuntimeBindings`](crate::native::cfg_program::CfgRuntimeBindings) to
    /// the same `LoadCurrent`/`LoadPriorCurrent` the shipped route chose; what
    /// changes is only how much of the branch the sum covers, and after the
    /// body has run the answer is all of it.
    ///
    /// Only [`Self::frozen_contribution_current`] consumers see this, because
    /// only they have per-contribution storage to read. A generated device
    /// inlines the contribution instead, and inlining a contribution that has
    /// not happened yet is exactly the forward read the running sum gives.
    noise_magnitude: bool,
    metadata_assignment_value: bool,
    /// Whether the expression being lowered is a contribution's right-hand
    /// side. Verilog-AMS 2023 section 4.5.12 requires a strictly positive
    /// transition time of a `zi_*` filter written there, so the distinction
    /// reaches the node rather than being rediscovered by a backend.
    zi_direct_assignment: bool,
    diagnostics: Vec<IrDiagnostic>,
}

/// A noise source between its site and the exit block.
struct PendingNoise {
    contribution: ContributionId,
    ordinal: usize,
    kind: CanonicalNoiseSourceKind,
    log_interp: bool,
    label: Option<SmolStr>,
    active: CfgVariable,
    psd: CfgVariable,
    exponent: Option<CfgVariable>,
    table: Vec<CfgVariable>,
}

struct PendingNoiseProcess {
    process_id: u32,
    kind: CanonicalNoiseSourceKind,
    log_interp: bool,
    label: Option<SmolStr>,
    active: CfgVariable,
    psd: CfgVariable,
    exponent: Option<CfgVariable>,
    table: Vec<CfgVariable>,
    /// The magnitudes as the site itself computed them, before the exit merge
    /// folded the control flow that reached the site into them.
    ///
    /// `None` unless [`CfgLowerMode::noise_site_values`] asked for them, which
    /// only the executable plan does. Producing them adds liveness roots, and a
    /// root the generated backend does not read would still change what its
    /// schedule selection emits — see [`CfgNoiseProcess::psd_at_site`].
    site: Option<PendingNoiseProcessSite>,
}

/// The site-block values of one process's magnitudes.
#[derive(Debug, Clone, Copy)]
struct PendingNoiseProcessSite {
    psd: ValueId,
    exponent: Option<ValueId>,
}

impl PendingNoise {
    /// Every variable this source carries, in the order the outputs are laid
    /// out. Both the zero-initialization and the read-back walk this, so they
    /// cannot disagree about which variables a source owns.
    fn variables(&self) -> impl Iterator<Item = CfgVariable> + '_ {
        [self.active, self.psd]
            .into_iter()
            .chain(self.exponent)
            .chain(self.table.iter().copied())
    }
}

impl PendingNoiseProcess {
    fn variables(&self) -> impl Iterator<Item = CfgVariable> + '_ {
        [self.active, self.psd]
            .into_iter()
            .chain(self.exponent)
            .chain(self.table.iter().copied())
    }

    /// The site values this process contributes to the output list, in the
    /// order [`resolve_noise_processes`] reads them back.
    fn site_values(&self) -> impl Iterator<Item = ValueId> + '_ {
        self.site
            .into_iter()
            .flat_map(|site| std::iter::once(site.psd).chain(site.exponent))
    }
}

/// One `$limit` call, while its body is in scope.
#[derive(Debug, Clone, Copy)]
struct Limiter {
    operator: ExprId,
    proposed: ValueId,
}

fn assignment_targets(hir: &HirModel, assignment: &super::hir::HirAssignment) -> Vec<VariableId> {
    if assignment.index.is_some()
        && let Some(array) = hir
            .arrays
            .iter()
            .find(|array| array.name == assignment.target_name)
    {
        let base = usize::from(array.base);
        return (0..array.len as usize)
            .map(|offset| VariableId::from(base + offset))
            .collect();
    }
    vec![assignment.target]
}

/// Canonical-HIR counterpart of the legacy backend's
/// `is_instance_static_expr`: only values fixed for an instance/temperature/
/// analysis configuration may gate solver topology.
fn hir_expr_is_instance_static(
    hir: &HirModel,
    expression: ExprId,
    static_variables: &HashSet<VariableId>,
) -> bool {
    let Some(expression) = hir.expressions.get(usize::from(expression)) else {
        return false;
    };
    let recurse = |id| hir_expr_is_instance_static(hir, id, static_variables);
    match &expression.kind {
        HirExprKind::NullArgument => false,
        HirExprKind::Number { .. } | HirExprKind::StringLiteral { .. } => true,
        HirExprKind::Identifier { name } => {
            hir.parameters
                .iter()
                .any(|parameter| parameter.name == *name)
                || hir
                    .variables
                    .iter()
                    .find(|variable| variable.name == *name)
                    .is_some_and(|variable| static_variables.contains(&variable.id))
        }
        HirExprKind::SystemFunction { name, args } => {
            match (name.to_ascii_lowercase().as_str(), args.len()) {
                ("$temperature" | "$mfactor", 0) => true,
                ("$param_given" | "$port_connected", 1) => true,
                ("$vt" | "$thermal_vt", 0) => true,
                ("$vt" | "$thermal_vt", 1) => args.iter().copied().all(recurse),
                // `$abstime`, `$realtime`, `$simparam`, `$frequency`, noise,
                // limiting, and every unknown system function fail closed.
                _ => false,
            }
        }
        HirExprKind::Call { name, args } => {
            let name = name.to_ascii_lowercase();
            match (name.as_str(), args.len()) {
                // Legacy lowering represents a validated analysis query as
                // `IrExpr::Analysis`, which is instance-static. Require the
                // literal query here rather than blessing arbitrary call
                // arguments as topology controls.
                ("analysis", count) if count > 0 => args.iter().copied().all(|argument| {
                    matches!(
                        hir.expressions
                            .get(usize::from(argument))
                            .map(|argument| &argument.kind),
                        Some(HirExprKind::StringLiteral { value })
                            if !matches!(
                                value.to_ascii_lowercase().as_str(),
                                "__rspice_initial_step" | "__rspice_final_step"
                            )
                    )
                }),
                // These are the calls legacy lowering converts to pure
                // `IrExpr::Call` (or `Limexp`) values. Calls are not pure by
                // default in Verilog-A: ddt/idt/ddx, delays, event operators,
                // filters, noise sources, and unknown functions all remain
                // runtime-dependent even when their operands are parameters.
                (
                    "abs"
                    | "fabs"
                    | "sqrt"
                    | "exp"
                    | "expm1"
                    | "limexp"
                    | "__rspice_limited_exp"
                    | "ln"
                    | "log"
                    | "log10"
                    | "log1p"
                    | "sin"
                    | "cos"
                    | "tan"
                    | "asin"
                    | "acos"
                    | "atan"
                    | "sinh"
                    | "cosh"
                    | "tanh"
                    | "asinh"
                    | "acosh"
                    | "atanh"
                    | "floor"
                    | "ceil",
                    1,
                ) => args.iter().copied().all(recurse),
                ("min" | "max" | "pow" | "fpow" | "hypot" | "atan2", 2) => {
                    args.iter().copied().all(recurse)
                }
                _ => false,
            }
        }
        HirExprKind::Binary { left, right, .. } => recurse(*left) && recurse(*right),
        HirExprKind::Unary { operand, .. } => recurse(*operand),
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => recurse(*condition) && recurse(*then_expr) && recurse(*else_expr),
        HirExprKind::ArrayAccess { array, index } => {
            recurse(*index)
                && hir
                    .arrays
                    .iter()
                    .find(|candidate| candidate.name == *array)
                    .is_some_and(|array| {
                        let base = usize::from(array.base);
                        (0..array.len as usize).all(|offset| {
                            static_variables.contains(&VariableId::from(base + offset))
                        })
                    })
        }
        HirExprKind::ArrayLiteral { elements, .. } => elements.iter().copied().all(recurse),
        HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. }
        | HirExprKind::AnalogOperator { .. }
        | HirExprKind::NoiseSource { .. } => false,
    }
}

fn compute_instance_static_guard_conditions(hir: &HirModel) -> HashSet<ExprId> {
    fn analyze(
        hir: &HirModel,
        regions: &[HirRegion],
        static_variables: &mut HashSet<VariableId>,
        static_control: bool,
        mut static_conditions: Option<&mut HashSet<ExprId>>,
    ) {
        for region in regions {
            match region {
                HirRegion::Assignment(assignment) => {
                    let write_static = static_control
                        && hir_expr_is_instance_static(hir, assignment.expr.id, static_variables)
                        && assignment.index.as_ref().is_none_or(|index| {
                            hir_expr_is_instance_static(hir, index.id, static_variables)
                        });
                    for target in assignment_targets(hir, assignment) {
                        if write_static {
                            static_variables.insert(target);
                        } else {
                            static_variables.remove(&target);
                        }
                    }
                }
                HirRegion::Conditional {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    let condition_static =
                        hir_expr_is_instance_static(hir, condition.id, static_variables);
                    if condition_static && let Some(conditions) = static_conditions.as_deref_mut() {
                        conditions.insert(condition.id);
                    }

                    let incoming = static_variables.clone();
                    let branch_control_static = static_control && condition_static;
                    let mut then_variables = incoming.clone();
                    analyze(
                        hir,
                        then_body,
                        &mut then_variables,
                        branch_control_static,
                        static_conditions.as_deref_mut(),
                    );
                    let mut else_variables = incoming;
                    analyze(
                        hir,
                        else_body,
                        &mut else_variables,
                        branch_control_static,
                        static_conditions.as_deref_mut(),
                    );
                    then_variables.retain(|variable| else_variables.contains(variable));
                    *static_variables = then_variables;
                }
                HirRegion::Loop {
                    condition, body, ..
                } => {
                    // The loop header is reached both from the entry edge and
                    // every back edge. Start from the entry state and descend
                    // to the greatest fixed point so a value is considered
                    // static only when it is static on every possible visit.
                    let entry = static_variables.clone();
                    let mut header = entry.clone();
                    loop {
                        let condition_static =
                            hir_expr_is_instance_static(hir, condition.id, &header);
                        let mut body_variables = header.clone();
                        analyze(
                            hir,
                            body,
                            &mut body_variables,
                            static_control && condition_static,
                            None,
                        );
                        body_variables.retain(|variable| entry.contains(variable));
                        if body_variables == header {
                            break;
                        }
                        header = body_variables;
                    }

                    let condition_static = hir_expr_is_instance_static(hir, condition.id, &header);
                    if condition_static && let Some(conditions) = static_conditions.as_deref_mut() {
                        conditions.insert(condition.id);
                    }
                    let mut body_variables = header;
                    analyze(
                        hir,
                        body,
                        &mut body_variables,
                        static_control && condition_static,
                        static_conditions.as_deref_mut(),
                    );
                    body_variables.retain(|variable| entry.contains(variable));
                    *static_variables = body_variables;
                }
                HirRegion::Contribution(_) => {}
            }
        }
    }

    // Verilog-AMS initializes every analog local to zero. That reaching
    // definition is instance-static until a runtime-dependent assignment
    // replaces it.
    let mut static_variables = hir.variables.iter().map(|variable| variable.id).collect();
    let mut static_conditions = HashSet::new();
    analyze(
        hir,
        &hir.body,
        &mut static_variables,
        true,
        Some(&mut static_conditions),
    );
    static_conditions
}

/// The value `$simparam("name")` means when the source names no fallback and
/// the simulator offers no value for `name`.
///
/// One table rather than one per lowering: `$simparam` is answered at compile
/// time by the bytecode route ([`crate::native`]'s `lower_simparam_intrinsic`),
/// as a fallback under a runtime query by the generated-Rust backend, and by
/// the closed defaults above under metadata-only noise lowering. The three
/// answer the same question, so they read the same table — a second copy is
/// how `$simparam("gmin")` came to mean 1e-12 on one route and 0.0 on another.
///
/// Names are matched as the source spells them, which is how the language
/// spells them: `simulatorVersion` is not `simulatorversion`.
pub(crate) fn simparam_source_default(name: &str) -> f64 {
    match name {
        "gmin" => 1.0e-12,
        "tnom" => 300.15,
        "simulatorVersion" => 1.0,
        _ => 0.0,
    }
}

/// What the consumer of a lowered CFG is, in the two respects the lowering has
/// to know about.
///
/// Two independent facts rather than one three-valued mode, because they are
/// independent: noise metadata is lowered for the generated backend and still
/// needs the runtime `$port_connected` leaf, since the metadata it produces is
/// read per instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CfgLowerMode {
    /// Lower only what raw grouped-noise metadata needs.
    noise_metadata_only: bool,
    /// Publish each noise process's magnitudes as its own *site* computed them,
    /// beside the exit-merged ones. See [`CfgNoiseProcess::site`].
    noise_site_values: bool,
    /// The consumer may evaluate an instance that omits a trailing terminal,
    /// so `$port_connected` is a runtime leaf rather than the constant `1.0`.
    /// See [`CfgModel::from_hir_for_executable_backend`].
    per_instance_ports: bool,
    /// Evaluate [`HirModel::prologue_statements`] into the entry block before
    /// the body, so a body read of a localparam or a declaration initializer
    /// sees what the initializer wrote. See [`CfgLowerer::prologue`].
    lower_prologue: bool,
    /// Whether a read of an event-controlled variable is the entry block's
    /// `EventState` leaf rather than the body's reaching definition.
    ///
    /// # Which consumer owns the event body
    ///
    /// The consumer decides, because the consumer decides whether the event
    /// body has already run. A generated device computes everything from this
    /// CFG: the leaf is the *accepted* value its runtime restored, the body
    /// applies the event on top of it, and what comes out is the candidate the
    /// device stores. That is one application of the event body and it is
    /// correct.
    ///
    /// An executable plan does not. It keeps the MIR route's assignment pass —
    /// see [`crate::jit::cfg_plan_builder`] — and that pass has already run the
    /// guard-folded event body into the variable's runtime slot before any
    /// value entry executes. The leaf reads that slot, so by the time the CFG's
    /// body applies the event again it is applying it a *second* time:
    /// `checkpoint_before_acceptance_excludes_step_event_variable_candidates`
    /// counted 2.0 where the shipped route counts 1.0.
    ///
    /// So the residual reads the slot, which is exactly the
    /// `NativeOp::LoadVariable` the postfix route's own read lowers to — the
    /// two routes agree by identity of leaf and op, the standard
    /// `$port_connected` set. The body's writes to the variable are then dead
    /// on this route and the pruner drops them; they are still what
    /// [`CfgModel::event_state_candidates`] reads, which only the generated
    /// backend consumes.
    frozen_event_state: bool,
    /// Whether a probe of a branch nothing solves for is the frozen
    /// [`CfgValueKind::ContributedCurrent`] leaf rather than the running sum of
    /// the contributions themselves.
    ///
    /// # Which consumer owns the contributed current
    ///
    /// The same question `frozen_event_state` answers, with the same answer: it
    /// depends on whether the consumer computes the other equation.
    ///
    /// A generated device does. Its residual for one branch and its probe of
    /// another are two expressions in one emitted function, so inlining the
    /// contribution is not just correct, it is the only thing that *is* — the
    /// device has no per-contribution current storage to read instead.
    ///
    /// An executable plan does not. It keeps the shipped route's equation
    /// order and its `currents` storage, so by the time a later entry runs, the
    /// earlier contribution's current is a number the runtime is holding, and
    /// `crate::native`'s `lower_branch_access` reads it — never recomputes it.
    /// The difference is a whole matrix and not a rounding: inlining makes the
    /// probe differentiable, so
    /// `native_device_stamps_internal_node_current_probe_alias_jacobian_without_fallback`
    /// counted 4.5 where the shipped route counts 2.5.
    ///
    /// So the executable lowering emits the leaf, whose derivative is zero, and
    /// [`crate::native::cfg_program`] translates it to the same
    /// `LoadCurrent`/`LoadPriorCurrent` the shipped route chose. The two routes
    /// then agree by identity of leaf and op, which is the standard the
    /// `$port_connected` and event-state splits were held to.
    frozen_contribution_current: bool,
}

impl CfgLowerMode {
    /// # Why the generated backend does not lower the prologue *yet*
    ///
    /// It has the same hole, and for the same reason — it is built from the
    /// body too — so this is not a difference the two consumers have earned,
    /// the way `per_instance_ports` is. It is a frozen artifact: the forty-three
    /// shipped devices are checked in as generated Rust under a `bundle_digest`,
    /// and `hisimsotb`'s body reads three localparams (`TN`, `QN`, `QB` —
    /// `hisimsotb.va:657`), so turning this on here changes that device's
    /// emitted code. Flipping it is a one-word change plus a regeneration, and
    /// it is a *fix*: today those reads compile to zero.
    const GENERATED: Self = Self {
        noise_metadata_only: false,
        noise_site_values: false,
        per_instance_ports: false,
        lower_prologue: false,
        frozen_event_state: false,
        frozen_contribution_current: false,
    };
    #[cfg(any(feature = "native", feature = "wasm-jit"))]
    const EXECUTABLE: Self = Self {
        noise_metadata_only: false,
        noise_site_values: true,
        per_instance_ports: true,
        lower_prologue: true,
        frozen_event_state: true,
        frozen_contribution_current: true,
    };
    /// Raw grouped-noise metadata is lowered for the generated backend and is
    /// part of the same frozen output, so it stays with `GENERATED` here.
    const NOISE_METADATA: Self = Self {
        noise_metadata_only: true,
        noise_site_values: false,
        per_instance_ports: true,
        lower_prologue: false,
        frozen_event_state: false,
        frozen_contribution_current: false,
    };
}

impl<'a> CfgLowerer<'a> {
    fn new(hir: &'a HirModel, mir: &'a MirModel, mode: CfgLowerMode) -> Self {
        let mut ground_names: HashSet<SmolStr> = mir.ground_nodes.iter().cloned().collect();
        ground_names.insert(SmolStr::new("0"));
        let static_guard_conditions = compute_instance_static_guard_conditions(hir);

        Self {
            hir,
            mir,
            builder: SsaBuilder::new(),
            block: BlockId::from(0usize),
            variables_by_name: hir
                .variables
                .iter()
                .map(|variable| (variable.name.clone(), variable.id))
                .collect(),
            parameters_by_name: hir
                .parameters
                .iter()
                .map(|parameter| (parameter.name.clone(), parameter.id))
                .collect(),
            static_guard_conditions,
            nodes_by_name: mir
                .nodes
                .iter()
                .map(|node| (node.name.clone(), node.id))
                .collect(),
            ground_names,
            leaves: HashMap::new(),
            temporary_count: 0,
            limiters: Vec::new(),
            noise: Vec::new(),
            noise_processes: Vec::new(),
            noise_metadata_only: mode.noise_metadata_only,
            noise_site_values: mode.noise_site_values,
            per_instance_ports: mode.per_instance_ports,
            lower_prologue: mode.lower_prologue,
            frozen_event_state: mode.frozen_event_state,
            frozen_event_states: HashMap::new(),
            frozen_contribution_current: mode.frozen_contribution_current,
            completed_current_contributions: Vec::new(),
            noise_magnitude: false,
            metadata_assignment_value: false,
            zi_direct_assignment: false,
            diagnostics: Vec::new(),
        }
    }

    #[allow(clippy::type_complexity)]
    fn lower(
        &mut self,
    ) -> Result<
        (
            CfgFunction,
            Vec<ValueId>,
            Vec<Option<ValueId>>,
            Vec<ValueId>,
            Vec<CfgNoiseSource>,
            Vec<CfgNoiseProcess>,
        ),
        Vec<IrDiagnostic>,
    > {
        let entry = self.builder.create_block();
        self.builder.seal_block(entry);
        self.block = entry;

        // Every residual starts at zero so an untaken branch needs no special
        // case: the join simply merges the value that was never updated.
        let zero = self.real_constant(0.0);
        let event_state_variables = self
            .hir
            .variables
            .iter()
            .filter(|variable| variable.is_state)
            .map(|variable| variable.id)
            .collect::<Vec<_>>();
        for (slot, variable) in event_state_variables.iter().copied().enumerate() {
            let slot = u32::try_from(slot).expect("event-state slot count fits u32");
            let accepted = self.leaf(
                LeafKey::EventState(slot),
                CfgValueType::Real,
                CfgValueKind::EventState(slot),
            );
            self.builder
                .write_variable(CfgVariable::Local(variable), entry, accepted);
            if self.frozen_event_state {
                self.frozen_event_states.insert(variable, accepted);
            }
        }
        for index in 0..self.hir.contributions.len() {
            let contribution = ContributionId::from(index);
            self.builder
                .write_variable(CfgVariable::Residual(contribution), entry, zero);
            if self.hir.contributions[index].kind == HirContributionKind::Potential {
                self.builder
                    .write_variable(CfgVariable::Activation(contribution), entry, zero);
            }
        }

        if self.lower_prologue {
            self.prologue();
        }

        let body = self.hir.body.clone();
        self.regions(&body, false);

        let exit = self.block;
        let residuals: Vec<_> = (0..self.hir.contributions.len())
            .map(|index| {
                let variable = CfgVariable::Residual(ContributionId::from(index));
                self.builder.read_variable(variable, exit).unwrap_or(zero)
            })
            .collect();
        let activations: Vec<Option<ValueId>> = (0..self.hir.contributions.len())
            .map(|index| {
                if self.hir.contributions[index].kind != HirContributionKind::Potential {
                    return None;
                }
                let variable = CfgVariable::Activation(ContributionId::from(index));
                Some(self.builder.read_variable(variable, exit).unwrap_or(zero))
            })
            .collect();
        let event_state_candidates = event_state_variables
            .iter()
            .map(|variable| {
                self.builder
                    .read_variable(CfgVariable::Local(*variable), exit)
                    .unwrap_or(zero)
            })
            .collect::<Vec<_>>();

        // A noise source's variables are given their zero here, once the body
        // has been walked and it is known which exist. `write_variable` records
        // a definition rather than emitting an instruction, and nothing has read
        // one of these yet, so seeding the entry block after the fact defines
        // them on every path that does not pass the source — which is what makes
        // an unreached source read back inactive instead of undefined.
        //
        // Only where the entry block has no definition already, because a body
        // with no control flow around its sources writes them *in* the entry
        // block, and seeding over that would zero the source that is always on.
        let pending = std::mem::take(&mut self.noise);
        for source in &pending {
            for variable in source.variables() {
                if self.builder.read_variable(variable, entry).is_none() {
                    self.builder.write_variable(variable, entry, zero);
                }
            }
        }
        let mut pending_processes = std::mem::take(&mut self.noise_processes);
        pending_processes.sort_by_key(|process| process.process_id);
        for (expected, process) in pending_processes.iter().enumerate() {
            if process.process_id as usize != expected {
                self.diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::CfgLowering,
                    format!(
                        "structural noise process IDs must be dense: expected {expected}, found {}",
                        process.process_id
                    ),
                ));
            }
        }
        for process in &pending_processes {
            for variable in process.variables() {
                if self.builder.read_variable(variable, entry).is_none() {
                    self.builder.write_variable(variable, entry, zero);
                }
            }
        }
        let mut outputs = residuals.clone();
        outputs.extend(activations.iter().flatten().copied());
        outputs.extend(event_state_candidates.iter().copied());
        for source in &pending {
            for variable in source.variables() {
                outputs.push(self.builder.read_variable(variable, exit).unwrap_or(zero));
            }
        }
        for process in &pending_processes {
            for variable in process.variables() {
                outputs.push(self.builder.read_variable(variable, exit).unwrap_or(zero));
            }
        }
        // After every variable read-back, so the split below stays a suffix
        // whatever the mode decided. A site value is already a value of the
        // function, so listing it here only makes it a liveness root.
        for process in &pending_processes {
            outputs.extend(process.site_values());
        }
        self.builder.set_terminator(exit, CfgTerminator::Return);

        // Through `finish_with_outputs`, because finishing renumbers values and
        // a residual id read out beforehand names something else afterwards.
        let builder = std::mem::take(&mut self.builder);
        match builder.finish_with_outputs(entry, &outputs) {
            Ok((function, outputs)) => {
                let contribution_count = self.hir.contributions.len();
                let (residuals, remaining) = outputs.split_at(contribution_count);
                let activation_count = activations.iter().flatten().count();
                let (mapped_activations, remaining) = remaining.split_at(activation_count);
                let mut mapped_activations = mapped_activations.iter().copied();
                let activations = activations
                    .iter()
                    .map(|activation| {
                        activation.map(|_| mapped_activations.next().expect("activation output"))
                    })
                    .collect();
                debug_assert!(mapped_activations.next().is_none());
                let (event_state_candidates, remaining) =
                    remaining.split_at(event_state_candidates.len());
                let legacy_noise_width = pending
                    .iter()
                    .map(|source| source.variables().count())
                    .sum::<usize>();
                let (noise, remaining) = remaining.split_at(legacy_noise_width);
                let process_width = pending_processes
                    .iter()
                    .map(|process| process.variables().count())
                    .sum::<usize>();
                let (noise_processes, process_sites) = remaining.split_at(process_width);
                Ok((
                    function,
                    residuals.to_vec(),
                    activations,
                    event_state_candidates.to_vec(),
                    resolve_noise(pending, noise),
                    resolve_noise_processes(pending_processes, noise_processes, process_sites),
                ))
            }
            Err(error) => Err(vec![IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("CFG construction produced an invalid function: {error}"),
            )]),
        }
    }

    fn regions(&mut self, regions: &[HirRegion], dynamic_topology_ancestor: bool) {
        for region in regions {
            self.region(region, dynamic_topology_ancestor);
        }
    }

    fn region(&mut self, region: &HirRegion, dynamic_topology_ancestor: bool) {
        match region {
            HirRegion::Assignment(assignment) => self.assignment(assignment),
            HirRegion::Contribution(contribution) => {
                if self.noise_metadata_only {
                    self.metadata_noise_expr(contribution.expression.id);
                } else {
                    self.contribution(contribution, dynamic_topology_ancestor)
                }
            }
            HirRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let condition_static = self.condition_is_instance_static(condition.id);
                if !condition_static && !dynamic_topology_ancestor {
                    self.activate_potential_descendants(then_body);
                    self.activate_potential_descendants(else_body);
                }
                self.conditional(
                    condition.id,
                    then_body,
                    else_body,
                    dynamic_topology_ancestor || !condition_static,
                );
            }
            HirRegion::Loop {
                condition, body, ..
            } => {
                let condition_static = self.condition_is_instance_static(condition.id);
                if !condition_static && !dynamic_topology_ancestor {
                    self.activate_potential_descendants(body);
                }
                self.runtime_loop(
                    condition.id,
                    body,
                    dynamic_topology_ancestor || !condition_static,
                );
            }
        }
    }

    /// Evaluate one assignment into its variable's reaching definition.
    ///
    /// Shared by the structured body and by the module prologue
    /// ([`Self::prologue`]), which is the same construct at a different program
    /// point: `HirStatement::Assignment` and `HirRegion::Assignment` are the
    /// same [`HirAssignment`], so there is one lowering of it rather than two
    /// that could drift.
    fn assignment(&mut self, assignment: &HirAssignment) {
        if assignment.index.is_some() {
            self.unsupported(
                assignment.span,
                format!(
                    "assignment to '{}' at a run-time array index",
                    assignment.target_name
                ),
            );
            return;
        }
        let was_assignment = self.metadata_assignment_value;
        self.metadata_assignment_value = self.noise_metadata_only;
        let value = self.expr(assignment.expr.id);
        self.metadata_assignment_value = was_assignment;
        self.builder
            .write_variable(CfgVariable::Local(assignment.target), self.block, value);
    }

    /// Evaluate the module prologue into the entry block.
    ///
    /// The localparam and module-scope variable initializers run before the
    /// `analog` keyword, so [`HirModel::body`] has no position for them and a
    /// CFG built from the body alone has no definition of what they write. The
    /// flat route runs them by simply executing its statement list in order;
    /// this is the same thing at the one program point the body has for
    /// "before anything else". A body assignment to the same variable then
    /// overwrites the reaching definition exactly as a later statement
    /// overwrites the slot, so a conditionally-reassigned initializer keeps its
    /// declared value on the arm that does not assign it.
    ///
    /// Only the initializers themselves are here. A prologue initializer whose
    /// right-hand side hoists a side effect — an analog function call with an
    /// output argument — pushes that side effect into the flat list *and*
    /// records it as a body region, because the analyzer's region stack is
    /// already open when the prologue runs. Lowering it here as well would
    /// duplicate every operator in it, and duplicating a `ddt` allocates a
    /// second state record for one source operator. So the side effect stays
    /// where the body has it, which leaves the initializer that reads it
    /// reading a definition that does not reach the entry block:
    /// [`Self::identifier`] warns and takes the language's zero, which is what
    /// it did for the whole prologue before this existed.
    fn prologue(&mut self) {
        let statements = self.hir.statements.clone();
        for index in &self.hir.prologue_statements {
            let Some(HirStatement::Assignment(assignment)) =
                statements.get(usize::try_from(*index).unwrap_or(usize::MAX))
            else {
                // A prologue statement is an initializer, never a loop, and the
                // index came from the analyzer. Nothing to lower either way.
                continue;
            };
            self.assignment(assignment);
        }
    }

    fn contribution(&mut self, contribution: &HirContribution, dynamic_topology_ancestor: bool) {
        self.zi_direct_assignment = true;
        let value = self.expr(contribution.expression.id);
        self.zi_direct_assignment = false;
        let variable = CfgVariable::Residual(contribution.id);
        let accumulated = match self.builder.read_variable(variable, self.block) {
            Some(accumulated) => accumulated,
            None => self.real_constant(0.0),
        };
        let sum = self.binary(CfgBinaryOp::Add, accumulated, value);
        self.builder.write_variable(variable, self.block, sum);
        if contribution.kind == HirContributionKind::Potential && !dynamic_topology_ancestor {
            self.activate_contribution(contribution.id);
        }

        // After the residual, not before: the two walks read the same variables
        // in the same block, so which runs first cannot change what either sees,
        // and this order keeps the contribution itself the first thing here.
        self.noise_sources(contribution.id, contribution.expression.id);

        // After the expression, so a flow probe *inside* this contribution
        // reads what came before it and not itself — which is the read the
        // shipped route allows and the one every model that senses its own
        // terminal current writes.
        if contribution.kind == HirContributionKind::Current {
            self.completed_current_contributions.push(contribution.id);
        }
    }

    fn activate_contribution(&mut self, contribution: ContributionId) {
        let active = self.real_constant(1.0);
        self.builder
            .write_variable(CfgVariable::Activation(contribution), self.block, active);
    }

    /// Once a dynamic guard is encountered, legacy topology semantics stop
    /// peeling guards: every potential statement below it owns active topology,
    /// while its residual remains governed by the full control flow. Mark those
    /// statements before branching so an untaken runtime path still closes the
    /// physical branch.
    fn activate_potential_descendants(&mut self, regions: &[HirRegion]) {
        fn collect(regions: &[HirRegion], out: &mut Vec<ContributionId>) {
            for region in regions {
                match region {
                    HirRegion::Contribution(contribution)
                        if contribution.kind == HirContributionKind::Potential =>
                    {
                        out.push(contribution.id);
                    }
                    HirRegion::Conditional {
                        then_body,
                        else_body,
                        ..
                    } => {
                        collect(then_body, out);
                        collect(else_body, out);
                    }
                    HirRegion::Loop { body, .. } => collect(body, out),
                    HirRegion::Assignment(_) | HirRegion::Contribution(_) => {}
                }
            }
        }

        let mut contributions = Vec::new();
        collect(regions, &mut contributions);
        contributions.sort_unstable_by_key(|contribution| usize::from(*contribution));
        contributions.dedup();
        for contribution in contributions {
            self.activate_contribution(contribution);
        }
    }

    fn condition_is_instance_static(&self, expression: ExprId) -> bool {
        self.static_guard_conditions.contains(&expression)
    }

    /// Lower whatever noise the contribution's expression carries.
    ///
    /// The walk is the one `noise::extract_expression` performs on the folded
    /// statements, and it has to stay the one: the amplitude a source is scaled
    /// by is folded into its power there, so descending differently would give
    /// a source the wrong magnitude, and visiting in a different order would
    /// give it the wrong identity.
    fn noise_sources(&mut self, contribution: ContributionId, expression: ExprId) {
        if !contains_noise(self.hir, expression) {
            return;
        }
        let unit = self.real_constant(1.0);
        self.noise_term(contribution, expression, unit);
    }

    /// Walk a contribution value only far enough to execute its syntactic
    /// noise sites. This is deliberately not an alternate primal evaluator:
    /// assignments and conditions still use ordinary CFG semantics so raw PSD
    /// operands retain exact reaching definitions and lazy branch behavior.
    fn metadata_noise_expr(&mut self, id: ExprId) {
        if !contains_noise(self.hir, id) {
            return;
        }
        let Some(expression) = self.hir.expressions.get(usize::from(id)).cloned() else {
            self.diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("noise-metadata lowering found expression id {id} outside the arena"),
            ));
            return;
        };
        match expression.kind {
            HirExprKind::NoiseSource { .. } => {
                let _ = self.expr(id);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.metadata_noise_conditional(condition, then_expr, else_expr),
            HirExprKind::Binary { left, right, .. } => {
                self.metadata_noise_expr(left);
                self.metadata_noise_expr(right);
            }
            HirExprKind::Unary { operand, .. } => self.metadata_noise_expr(operand),
            HirExprKind::Call { args, .. } | HirExprKind::SystemFunction { args, .. } => {
                for argument in args {
                    self.metadata_noise_expr(argument);
                }
            }
            HirExprKind::ArrayAccess { index, .. } => self.metadata_noise_expr(index),
            HirExprKind::ArrayLiteral { elements, .. } => {
                for element in elements {
                    self.metadata_noise_expr(element);
                }
            }
            HirExprKind::AnalogOperator { op } => {
                let mut children = Vec::new();
                match op {
                    HirAnalogOperator::Limit {
                        proposed,
                        candidate,
                        type_metadata,
                        ..
                    } => {
                        children.extend([proposed, candidate]);
                        children.extend(type_metadata);
                    }
                    HirAnalogOperator::LimiterArgument { .. } => {}
                }
                for child in children {
                    self.metadata_noise_expr(child);
                }
            }
            HirExprKind::NullArgument
            | HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
        }
    }

    fn metadata_noise_conditional(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) {
        if contains_noise(self.hir, condition) {
            let span = self.hir.expressions[usize::from(condition)].span;
            self.unsupported_noise(span, "condition");
            return;
        }
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        self.block = then_block;
        self.metadata_noise_expr(then_expr);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        self.block = else_block;
        self.metadata_noise_expr(else_expr);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(join);
        self.block = join;
    }

    fn noise_term(&mut self, contribution: ContributionId, id: ExprId, amplitude: ValueId) {
        if !contains_noise(self.hir, id) {
            return;
        }
        let Some(expression) = self.hir.expressions.get(usize::from(id)).cloned() else {
            self.diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("CFG lowering found expression id {id} outside the arena"),
            ));
            return;
        };
        let span = expression.span;
        match &expression.kind {
            HirExprKind::Call { name, args } | HirExprKind::SystemFunction { name, args }
                if is_noise_call(name) =>
            {
                let label = args.last().and_then(|id| string_literal(self.hir, *id));
                self.noise_call(
                    contribution,
                    name.trim_start_matches('$'),
                    args,
                    label,
                    amplitude,
                    span,
                );
            }
            HirExprKind::NoiseSource {
                source,
                operands,
                name,
                ..
            } => {
                let (kind, log_interp) = match source.as_str() {
                    "White" if operands.len() == 1 => (CanonicalNoiseSourceKind::White, false),
                    "Flicker" if operands.len() == 2 => (CanonicalNoiseSourceKind::Flicker, false),
                    "Table" if !operands.is_empty() => (CanonicalNoiseSourceKind::Table, false),
                    "TableLog" if !operands.is_empty() => (CanonicalNoiseSourceKind::Table, true),
                    _ => {
                        self.unsupported(
                            span,
                            format!("noise source '{source}' with {} operands", operands.len()),
                        );
                        return;
                    }
                };
                let operands = operands.clone();
                let name = name.clone();
                self.noise_source(contribution, kind, log_interp, name, &operands, amplitude);
            }
            HirExprKind::Binary { op, left, right } if matches!(op.as_str(), "Add" | "Sub") => {
                let (left, right) = (*left, *right);
                self.noise_term(contribution, left, amplitude);
                self.noise_term(contribution, right, amplitude);
            }
            HirExprKind::Binary { op, left, right } if op == "Mul" => {
                let (left, right) = (*left, *right);
                match (
                    contains_noise(self.hir, left),
                    contains_noise(self.hir, right),
                ) {
                    (true, true) => self.unsupported_noise(span, "product of noise terms"),
                    (true, false) => {
                        let scale = self.expr(right);
                        let scaled = self.binary(CfgBinaryOp::Mul, amplitude, scale);
                        self.noise_term(contribution, left, scaled);
                    }
                    (false, true) => {
                        let scale = self.expr(left);
                        let scaled = self.binary(CfgBinaryOp::Mul, amplitude, scale);
                        self.noise_term(contribution, right, scaled);
                    }
                    (false, false) => {}
                }
            }
            HirExprKind::Binary { op, left, right } if op == "Div" => {
                let (left, right) = (*left, *right);
                if contains_noise(self.hir, right) {
                    self.unsupported_noise(span, "divisor");
                    return;
                }
                let divisor = self.expr(right);
                let scaled = self.binary(CfgBinaryOp::Div, amplitude, divisor);
                self.noise_term(contribution, left, scaled);
            }
            HirExprKind::Unary { op, operand } if matches!(op.as_str(), "Neg" | "Pos") => {
                let operand = *operand;
                self.noise_term(contribution, operand, amplitude);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let (condition, then_expr, else_expr) = (*condition, *then_expr, *else_expr);
                if contains_noise(self.hir, condition) {
                    self.unsupported_noise(span, "condition");
                    return;
                }
                self.noise_conditional(contribution, condition, then_expr, else_expr, amplitude);
            }
            _ => self.unsupported_noise(span, "nonlinear or dynamic position"),
        }
    }

    /// A ternary with noise in an arm, lowered as the diamond it is.
    ///
    /// The arms become real blocks, so a source in the untaken one neither
    /// evaluates its operands nor writes its variables — it reads back inactive
    /// through the join, on the same mechanism a source under a statement `if`
    /// already uses. That is stricter than gating the result by a numeric zero,
    /// which would still have evaluated an operand that may not be finite here.
    fn noise_conditional(
        &mut self,
        contribution: ContributionId,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
        amplitude: ValueId,
    ) {
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();

        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        self.block = then_block;
        self.noise_term(contribution, then_expr, amplitude);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.block = else_block;
        self.noise_term(contribution, else_expr, amplitude);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(join);
        self.block = join;
    }

    fn noise_call(
        &mut self,
        contribution: ContributionId,
        name: &str,
        args: &[ExprId],
        label: Option<SmolStr>,
        amplitude: ValueId,
        span: SourceSpanRef,
    ) {
        match name {
            "white_noise" if !args.is_empty() => {
                let operands = [args[0]];
                self.noise_source(
                    contribution,
                    CanonicalNoiseSourceKind::White,
                    false,
                    label,
                    &operands,
                    amplitude,
                );
            }
            "flicker_noise" if args.len() >= 2 => {
                let operands = [args[0], args[1]];
                self.noise_source(
                    contribution,
                    CanonicalNoiseSourceKind::Flicker,
                    false,
                    label,
                    &operands,
                    amplitude,
                );
            }
            "noise_table" | "noise_table_log" if !args.is_empty() => {
                // A table given as one array literal is its elements, matching
                // how the plan counts operands; anything else is one operand.
                let operands = match &self.hir.expressions[usize::from(args[0])].kind {
                    HirExprKind::ArrayLiteral { elements, .. } => elements.clone(),
                    _ => vec![args[0]],
                };
                self.noise_source(
                    contribution,
                    CanonicalNoiseSourceKind::Table,
                    name == "noise_table_log",
                    label,
                    &operands,
                    amplitude,
                );
            }
            _ => self.unsupported(
                span,
                format!("noise function '{name}' with {} operands", args.len()),
            ),
        }
    }

    fn noise_source(
        &mut self,
        contribution: ContributionId,
        kind: CanonicalNoiseSourceKind,
        log_interp: bool,
        label: Option<SmolStr>,
        operands: &[ExprId],
        amplitude: ValueId,
    ) {
        let squared = self.binary(CfgBinaryOp::Mul, amplitude, amplitude);
        let (psd, exponent, table) = match kind {
            CanonicalNoiseSourceKind::White => {
                let power = self.expr(operands[0]);
                (
                    self.binary(CfgBinaryOp::Mul, squared, power),
                    None,
                    Vec::new(),
                )
            }
            CanonicalNoiseSourceKind::Flicker => {
                let power = self.expr(operands[0]);
                let psd = self.binary(CfgBinaryOp::Mul, squared, power);
                let exponent = self.expr(operands[1]);
                (psd, Some(exponent), Vec::new())
            }
            // A table's power is the amplitude alone; the tabulated magnitudes
            // are the operands, exactly as the plan splits them.
            CanonicalNoiseSourceKind::Table => {
                let table = operands
                    .iter()
                    .map(|operand| self.expr(*operand))
                    .collect::<Vec<_>>();
                (squared, None, table)
            }
        };

        let ordinal = self
            .noise
            .iter()
            .filter(|source| source.contribution == contribution)
            .count();
        let pending = PendingNoise {
            contribution,
            ordinal,
            kind,
            log_interp,
            label,
            active: CfgVariable::Local(self.result_variable()),
            psd: CfgVariable::Local(self.result_variable()),
            exponent: exponent.map(|_| CfgVariable::Local(self.result_variable())),
            table: table
                .iter()
                .map(|_| CfgVariable::Local(self.result_variable()))
                .collect(),
        };

        // The block is read now, not before the operands were lowered: a table
        // magnitude may itself have been a ternary, and the writes belong
        // wherever lowering the last of them ended up.
        let block = self.block;
        let one = self.real_constant(1.0);
        self.builder.write_variable(pending.active, block, one);
        self.builder.write_variable(pending.psd, block, psd);
        if let (Some(variable), Some(value)) = (pending.exponent, exponent) {
            self.builder.write_variable(variable, block, value);
        }
        for (variable, value) in pending.table.iter().zip(table) {
            self.builder.write_variable(*variable, block, value);
        }
        self.noise.push(pending);
    }

    fn unsupported_noise(&mut self, span: SourceSpanRef, placement: &str) {
        self.unsupported(
            span,
            format!(
                "a noise function in a {placement} (noise terms must enter contributions additively, optionally scaled)"
            ),
        );
    }

    fn conditional(
        &mut self,
        condition: ExprId,
        then_body: &[HirRegion],
        else_body: &[HirRegion],
        dynamic_topology_ancestor: bool,
    ) {
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();

        // The condition may itself have been a diamond, so the branch belongs
        // on wherever lowering it ended up, not on the block it started in.
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        self.block = then_block;
        self.regions(then_body, dynamic_topology_ancestor);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.block = else_block;
        self.regions(else_body, dynamic_topology_ancestor);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(join);
        self.block = join;
    }

    /// `while (condition) body`, as a header that both the entry edge and the
    /// back edge reach.
    ///
    /// The header stays unsealed until the back edge exists. That is the one
    /// place incremental SSA construction needs the delay, and getting it wrong
    /// shows up as a loop-carried variable reading its initial value forever.
    fn runtime_loop(
        &mut self,
        condition: ExprId,
        body: &[HirRegion],
        dynamic_topology_ancestor: bool,
    ) {
        let header = self.builder.create_block();
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        self.block = header;
        let condition = self.expr(condition);
        let test = self.block;

        let body_block = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.set_terminator(
            test,
            CfgTerminator::Branch {
                condition,
                then_target: body_block,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(body_block);

        self.block = body_block;
        self.regions(body, dynamic_topology_ancestor);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.block = exit;
    }

    fn expr(&mut self, id: ExprId) -> ValueId {
        let Some(expression) = self.hir.expressions.get(usize::from(id)).cloned() else {
            self.diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("CFG lowering found expression id {id} outside the arena"),
            ));
            return self.real_constant(0.0);
        };
        self.expr_kind(&expression)
    }

    fn expr_kind(&mut self, expression: &HirExpression) -> ValueId {
        let span = expression.span;
        match &expression.kind {
            HirExprKind::Number { value, .. } => self.real_constant(*value),
            HirExprKind::Identifier { name } => self.identifier(name, span),
            HirExprKind::Binary { op, left, right } => self.binary_expr(op, *left, *right, span),
            HirExprKind::Unary { op, operand } => self.unary_expr(op, *operand, span),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.conditional_expr(*condition, *then_expr, *else_expr),
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.branch_access(access, pos, neg.as_deref(), span)
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.named_branch_access(access, name, span)
            }
            HirExprKind::SystemFunction { name, args } => self.system_function(name, args, span),
            // A dynamic operator's only spelling. `absdelay(x, d)` reaches the
            // HIR as a `Call`, and this mode substitutes the operator's zero
            // primal and records the noise site, which is what makes
            // `noise_metadata_from_hir` able to slice a model whose routing
            // operator the residual CFG need not implement.
            HirExprKind::Call { name, .. }
                if self.noise_metadata_only
                    && is_dynamic_operator_call(name)
                    && contains_noise(self.hir, expression.id) =>
            {
                if self.metadata_assignment_value
                    && !noise_substitution_is_zero(self.hir, expression.id)
                {
                    self.unsupported(
                        span,
                        "a noise-bearing dynamic operator assignment whose deterministic primal can reach later noise metadata"
                            .to_string(),
                    );
                }
                self.metadata_noise_expr(expression.id);
                self.real_constant(0.0)
            }
            HirExprKind::Call { name, args } => self.call(expression.id, name, args, span),
            HirExprKind::AnalogOperator { .. }
                if self.noise_metadata_only && contains_noise(self.hir, expression.id) =>
            {
                if self.metadata_assignment_value
                    && !noise_substitution_is_zero(self.hir, expression.id)
                {
                    self.unsupported(
                        span,
                        "a noise-bearing dynamic operator assignment whose deterministic primal can reach later noise metadata"
                            .to_string(),
                    );
                }
                self.metadata_noise_expr(expression.id);
                self.real_constant(0.0)
            }
            HirExprKind::AnalogOperator { op } => self.analog_operator(op, expression, span),
            // Record raw metadata at the exact executed CFG site, then return
            // the required zero primal. Routing/filter amplitude is handled by
            // grouped complex injection derivatives, not folded into PSD here.
            HirExprKind::NoiseSource {
                process_id,
                source,
                operands,
                name,
            } => self.noise_process(*process_id, source, operands, name.clone(), span),
            other => {
                self.unsupported(span, format!("{} expression", kind_label(other)));
                self.real_constant(0.0)
            }
        }
    }

    /// `laplace_zp`, `laplace_zd`, `laplace_np` or `laplace_nd`.
    ///
    /// The transfer function is folded here rather than carried as operands;
    /// [`CfgValueKind::Laplace`] documents why, and the front end's own refusal
    /// of a non-constant coefficient list is what makes the fold total. When it
    /// is not - a coefficient this level can reach but cannot fold - the answer
    /// is a diagnostic, not a filter with a coefficient guessed at.
    fn laplace(
        &mut self,
        operator: ExprId,
        input: ExprId,
        form: FilterForm,
        numerator: &[ExprId],
        denominator: &[ExprId],
        span: SourceSpanRef,
    ) -> ValueId {
        let Some(transfer) = self.laplace_transfer(form, numerator, denominator, span) else {
            return self.real_constant(0.0);
        };
        let input = self.expr(input);
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::Laplace {
                operator,
                input,
                transfer,
            },
        )
    }

    fn laplace_transfer(
        &mut self,
        form: FilterForm,
        numerator: &[ExprId],
        denominator: &[ExprId],
        span: SourceSpanRef,
    ) -> Option<CfgLaplaceTransfer> {
        // `laplace_zd` and `laplace_np` name one half of the transfer function
        // by its roots and the other by its coefficients. Expanding the root
        // half is exact and is what the executable IR does, which is why there
        // are two forms here and four spellings in the language.
        let expand = |lowerer: &mut Self, roots: &[ExprId]| -> Option<Vec<f64>> {
            let roots = lowerer.laplace_roots(roots, span)?;
            match crate::laplace::roots_to_polynomial(&roots) {
                Ok(polynomial) => Some(polynomial),
                Err(error) => {
                    lowerer.unsupported(span, format!("a filter root list that {error}"));
                    None
                }
            }
        };
        // Pole-zero form is the one that stays unexpanded, because the runtime
        // realization is built from the roots and expanding first would throw
        // that away. A form that names only *one* half by roots has no such
        // realization to preserve, so it expands into the coefficient form.
        if form.numerator_is_roots && form.denominator_is_roots {
            return Some(CfgLaplaceTransfer::ZeroPole {
                zeros: self.laplace_roots(numerator, span)?,
                poles: self.laplace_roots(denominator, span)?,
            });
        }
        let coefficients = |lowerer: &mut Self, list: &[ExprId], is_roots: bool| {
            if is_roots {
                expand(lowerer, list)
            } else {
                lowerer.laplace_coefficients(list, span)
            }
        };
        Some(CfgLaplaceTransfer::Coefficients {
            numerator: coefficients(self, numerator, form.numerator_is_roots)?,
            denominator: coefficients(self, denominator, form.denominator_is_roots)?,
        })
    }

    fn laplace_coefficients(
        &mut self,
        coefficients: &[ExprId],
        span: SourceSpanRef,
    ) -> Option<Vec<f64>> {
        coefficients
            .iter()
            .map(|coefficient| self.filter_constant(*coefficient, span))
            .collect()
    }

    /// A root list, read as the `(real, imaginary)` pairs the operator's
    /// argument is. An odd length is not a pair list, and the runtime would
    /// read the trailing element as a real part with an invented imaginary one.
    fn laplace_roots(&mut self, roots: &[ExprId], span: SourceSpanRef) -> Option<Vec<(f64, f64)>> {
        if !roots.len().is_multiple_of(2) {
            self.unsupported(
                span,
                "a filter pole/zero list whose length is not a whole number of (real, imaginary) pairs"
                    .to_string(),
            );
            return None;
        }
        let values = self.laplace_coefficients(roots, span)?;
        Some(
            values
                .chunks_exact(2)
                .map(|pair| (pair[0], pair[1]))
                .collect(),
        )
    }

    /// One Laplace coefficient, as the constant it has to be.
    fn filter_constant(&mut self, expr: ExprId, span: SourceSpanRef) -> Option<f64> {
        let value = self.expr(expr);
        match self.folded_constant(value) {
            Some(constant) if constant.is_finite() => Some(constant),
            Some(_) => {
                self.unsupported(span, "a non-finite laplace coefficient".to_string());
                None
            }
            None => {
                self.unsupported(
                    span,
                    "a laplace coefficient that is not a compile-time constant".to_string(),
                );
                None
            }
        }
    }

    /// The constant a value folds to, if it is one.
    ///
    /// A local fold rather than a call into the optimizer: the coefficients
    /// have to be numbers before the node can be built, and the optimizer runs
    /// over a finished function.
    fn folded_constant(&self, value: ValueId) -> Option<f64> {
        match self.builder.kind_of(value)? {
            CfgValueKind::RealConstant(constant) => Some(*constant),
            CfgValueKind::BooleanConstant(constant) => Some(f64::from(u8::from(*constant))),
            CfgValueKind::Unary { op, input } => Some(super::cfg_eval::apply_unary(
                *op,
                self.folded_constant(*input)?,
            )),
            CfgValueKind::Binary { op, left, right } => Some(super::cfg_eval::apply_binary(
                *op,
                self.folded_constant(*left)?,
                self.folded_constant(*right)?,
            )),
            _ => None,
        }
    }

    /// `zi_zp`, `zi_zd`, `zi_np` or `zi_nd`.
    ///
    /// The mirror image of [`Self::laplace`]: the coefficients stay operands,
    /// because the language lets them depend on parameters and the runtime
    /// installs them per instance. See [`CfgValueKind::Zi`].
    #[allow(clippy::too_many_arguments)]
    fn zi(
        &mut self,
        operator: ExprId,
        input: ExprId,
        form: FilterForm,
        numerator: &[ExprId],
        denominator: &[ExprId],
        period: ExprId,
        transition: Option<ExprId>,
        first_transition: Option<ExprId>,
        span: SourceSpanRef,
    ) -> ValueId {
        let Some((numerator, denominator)) =
            self.zi_polynomials(form, numerator, denominator, span)
        else {
            return self.real_constant(0.0);
        };
        let input = self.expr(input);
        let period = self.expr(period);
        // An omitted transition time is the module's `default_transition`, and
        // an omitted first-transition time is zero: the same defaults the
        // executable IR applies, applied where the node is built rather than
        // left for a consumer to know.
        let transition = match transition {
            Some(transition) => self.expr(transition),
            None => self.real_constant(self.hir.default_transition),
        };
        let first_transition = match first_transition {
            Some(first_transition) => self.expr(first_transition),
            None => self.real_constant(0.0),
        };
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::Zi {
                operator,
                input,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment: self.zi_direct_assignment,
            },
        )
    }

    fn zi_polynomials(
        &mut self,
        form: FilterForm,
        numerator: &[ExprId],
        denominator: &[ExprId],
        span: SourceSpanRef,
    ) -> Option<(CfgZiPolynomial, CfgZiPolynomial)> {
        // No expansion in either half: a `zi_*` root only has a value at
        // runtime, so the runtime is what expands it.
        Some((
            self.zi_polynomial(numerator, form.numerator_is_roots, span)?,
            self.zi_polynomial(denominator, form.denominator_is_roots, span)?,
        ))
    }

    fn zi_polynomial(
        &mut self,
        polynomial: &[ExprId],
        is_roots: bool,
        span: SourceSpanRef,
    ) -> Option<CfgZiPolynomial> {
        if is_roots {
            self.zi_roots(polynomial, span)
        } else {
            Some(CfgZiPolynomial::Coefficients(
                self.zi_coefficients(polynomial),
            ))
        }
    }

    fn zi_coefficients(&mut self, coefficients: &[ExprId]) -> Vec<ValueId> {
        coefficients
            .iter()
            .map(|coefficient| self.expr(*coefficient))
            .collect()
    }

    fn zi_roots(&mut self, roots: &[ExprId], span: SourceSpanRef) -> Option<CfgZiPolynomial> {
        if !roots.len().is_multiple_of(2) {
            self.unsupported(
                span,
                "a zi pole/zero list whose length is not a whole number of (real, imaginary) pairs"
                    .to_string(),
            );
            return None;
        }
        let values = self.zi_coefficients(roots);
        Some(CfgZiPolynomial::Roots(
            values
                .chunks_exact(2)
                .map(|pair| (pair[0], pair[1]))
                .collect(),
        ))
    }

    /// The elements of a filter's polynomial argument.
    ///
    /// The argument is an array literal in every model that writes more than
    /// one coefficient, and a bare expression in the ones that write exactly
    /// one; an explicitly null argument is the empty root list the LRM allows
    /// for a filter with no zeros. Matching [`crate::expr_converter`]'s reading
    /// of the same argument is what keeps the two routes describing one filter.
    fn filter_operand_list(&self, argument: ExprId) -> Vec<ExprId> {
        match self
            .hir
            .expressions
            .get(usize::from(argument))
            .map(|expression| &expression.kind)
        {
            Some(HirExprKind::ArrayLiteral { elements, .. }) => elements.clone(),
            Some(HirExprKind::NullArgument) => Vec::new(),
            _ => vec![argument],
        }
    }

    fn noise_process(
        &mut self,
        process_id: u32,
        source: &SmolStr,
        operands: &[ExprId],
        label: Option<SmolStr>,
        span: SourceSpanRef,
    ) -> ValueId {
        let (kind, log_interp) = match source.as_str() {
            "White" if operands.len() == 1 => (CanonicalNoiseSourceKind::White, false),
            "Flicker" if operands.len() == 2 => (CanonicalNoiseSourceKind::Flicker, false),
            "Table" if !operands.is_empty() => (CanonicalNoiseSourceKind::Table, false),
            "TableLog" if !operands.is_empty() => (CanonicalNoiseSourceKind::Table, true),
            _ => {
                self.unsupported(
                    span,
                    format!("noise process '{source}' with {} operands", operands.len()),
                );
                return self.real_constant(0.0);
            }
        };
        if self
            .noise_processes
            .iter()
            .any(|process| process.process_id == process_id)
        {
            self.diagnostics.push(IrDiagnostic::error(
                CompilerPhase::CfgLowering,
                format!("duplicate structural noise process id {process_id}"),
                span,
            ));
            return self.real_constant(0.0);
        }

        // Operand lowering occurs only after control reaches this expression;
        // untaken branches therefore cannot evaluate an invalid PSD/exponent.
        //
        // A flow probe inside these operands is the one place in the body that
        // reads the *settled* branch current rather than the running sum — see
        // [`Self::noise_magnitude`] — so the flag is raised around the operand
        // walk and nothing else.
        let outer_noise_magnitude = std::mem::replace(&mut self.noise_magnitude, true);
        let (psd, exponent, table) = match kind {
            CanonicalNoiseSourceKind::White => (self.expr(operands[0]), None, Vec::new()),
            CanonicalNoiseSourceKind::Flicker => (
                self.expr(operands[0]),
                Some(self.expr(operands[1])),
                Vec::new(),
            ),
            CanonicalNoiseSourceKind::Table => (
                self.real_constant(1.0),
                None,
                operands.iter().map(|operand| self.expr(*operand)).collect(),
            ),
        };
        self.noise_magnitude = outer_noise_magnitude;
        let pending = PendingNoiseProcess {
            process_id,
            kind,
            log_interp,
            label,
            active: CfgVariable::Local(self.result_variable()),
            psd: CfgVariable::Local(self.result_variable()),
            exponent: exponent.map(|_| CfgVariable::Local(self.result_variable())),
            table: table
                .iter()
                .map(|_| CfgVariable::Local(self.result_variable()))
                .collect(),
            site: self
                .noise_site_values
                .then_some(PendingNoiseProcessSite { psd, exponent }),
        };
        let block = self.block;
        let one = self.real_constant(1.0);
        self.builder.write_variable(pending.active, block, one);
        self.builder.write_variable(pending.psd, block, psd);
        if let (Some(variable), Some(value)) = (pending.exponent, exponent) {
            self.builder.write_variable(variable, block, value);
        }
        for (variable, value) in pending.table.iter().zip(table) {
            self.builder.write_variable(*variable, block, value);
        }
        self.noise_processes.push(pending);
        self.leaf(
            LeafKey::NoiseProcess(process_id),
            CfgValueType::Real,
            CfgValueKind::NoiseProcess(process_id),
        )
    }

    fn identifier(&mut self, name: &SmolStr, span: SourceSpanRef) -> ValueId {
        if let Some(variable) = self.variables_by_name.get(name).copied() {
            // An event-controlled variable, for a consumer whose assignment
            // pass has already run the body's event bodies into this
            // variable's slot. See [`CfgLowerMode::frozen_event_state`].
            if let Some(frozen) = self.frozen_event_states.get(&variable).copied() {
                return frozen;
            }
            // The whole reason this level exists: no history search, no
            // heuristic. The builder either has a reaching definition or the
            // variable is genuinely read before assignment.
            if let Some(value) = self
                .builder
                .read_variable(CfgVariable::Local(variable), self.block)
            {
                return value;
            }
            // Verilog-AMS initialises an analog variable to zero, and released
            // compact models rely on it: `r3_cmc` reads one before assigning it
            // on any path, and every production compiler runs `r3_cmc`. So this
            // is a warning and a zero, not a refusal.
            //
            // It stays a warning rather than becoming silent because the same
            // shape is also how a name-resolution bug shows up — a block local
            // shadowing a parameter of the same name reads as undefined, and
            // zero is the wrong answer to that one.
            self.warn(
                span,
                format!(
                    "'{name}' is read before it is assigned on any path; \
                     Verilog-AMS initialises it to zero"
                ),
            );
            return self.real_constant(0.0);
        }

        if let Some(parameter) = self.parameters_by_name.get(name).copied() {
            return self.leaf(
                LeafKey::Parameter(parameter),
                CfgValueType::Real,
                CfgValueKind::Parameter(parameter),
            );
        }

        self.unsupported(span, format!("identifier '{name}'"));
        self.real_constant(0.0)
    }

    fn binary_expr(
        &mut self,
        op: &SmolStr,
        left: ExprId,
        right: ExprId,
        span: SourceSpanRef,
    ) -> ValueId {
        if let Some(op) = integer_bitwise_op(op.as_str()) {
            let left = self.expr(left);
            let right = self.expr(right);
            return self.builder.push(
                self.block,
                CfgValueType::Real,
                CfgValueKind::IntegerBitwise { op, left, right },
            );
        }
        let Some(op) = binary_op(op.as_str()) else {
            self.unsupported(span, format!("binary operator '{op}'"));
            return self.real_constant(0.0);
        };
        let left = self.expr(left);
        let right = self.expr(right);
        self.binary(op, left, right)
    }

    fn unary_expr(&mut self, op: &SmolStr, operand: ExprId, span: SourceSpanRef) -> ValueId {
        match op.as_str() {
            "Pos" => self.expr(operand),
            "Neg" => {
                let input = self.expr(operand);
                self.unary(CfgUnaryOp::Neg, input)
            }
            "Not" => {
                let input = self.expr(operand);
                self.unary_typed(CfgUnaryOp::Not, input, CfgValueType::Boolean)
            }
            "BitNot" => {
                let input = self.expr(operand);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::IntegerBitwiseNot { input },
                )
            }
            _ => {
                self.unsupported(span, format!("unary operator '{op}'"));
                self.real_constant(0.0)
            }
        }
    }

    /// `c ? a : b` as a diamond, so the untaken side costs nothing.
    fn conditional_expr(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> ValueId {
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();

        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        // The result is carried through an SSA variable of its own, which is
        // what turns the two arms into one block parameter at the join — and
        // into no parameter at all when both arms produce the same value.
        let result = CfgVariable::Local(self.result_variable());

        self.block = then_block;
        let then_value = self.expr(then_expr);
        self.builder.write_variable(result, self.block, then_value);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.block = else_block;
        let else_value = self.expr(else_expr);
        self.builder.write_variable(result, self.block, else_value);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(join);
        self.block = join;
        self.builder
            .read_variable(result, join)
            .unwrap_or(then_value)
    }

    /// A fresh SSA key for one conditional expression's result.
    ///
    /// Numbered above every declared variable so it cannot collide with one;
    /// nothing else ever reads it, so it needs no name.
    fn result_variable(&mut self) -> VariableId {
        let id = VariableId::from(self.hir.variables.len() + self.temporary_count);
        self.temporary_count += 1;
        id
    }

    fn branch_access(
        &mut self,
        access: &SmolStr,
        pos: &SmolStr,
        neg: Option<&str>,
        span: SourceSpanRef,
    ) -> ValueId {
        let pos_node = self.endpoint(pos, span);
        let neg_node = match neg {
            Some(neg) => self.endpoint(&SmolStr::new(neg), span),
            None => Ok(None),
        };
        let (Ok(pos_node), Ok(neg_node)) = (pos_node, neg_node) else {
            return self.real_constant(0.0);
        };

        if is_flow_access(access.as_str()) {
            let Some((unknown, reversed)) = self.branch_unknown_by_nodes(pos_node, neg_node) else {
                if let Some(contributed) = self.contributed_flow(pos_node, neg_node) {
                    return contributed;
                }
                // `I(<p>)` reaches here as a single-ended access on a port,
                // because a port branch is not a branch between two nets.
                if let (Some(node), None) = (pos_node, neg_node)
                    && let Some(terminal) = self.port_flow(node)
                {
                    return terminal;
                }
                self.unsupported(
                    span,
                    format!("flow access '{access}' on a branch with nothing driving it"),
                );
                return self.real_constant(0.0);
            };
            let flow = self.leaf(
                LeafKey::BranchUnknownFlow(unknown),
                CfgValueType::Real,
                CfgValueKind::BranchUnknownFlow(unknown),
            );
            return if reversed {
                self.unary(CfgUnaryOp::Neg, flow)
            } else {
                flow
            };
        }

        self.potential_difference(pos_node, neg_node)
    }

    /// The current already contributed to a branch, at this point in the walk.
    ///
    /// A branch with no unknown is one nothing solves for: it is driven by flow
    /// contributions, so its current is not a variable but the running sum of
    /// what the model has put into it. That sum is already in hand — it is the
    /// residual accumulator each `<+` writes — so a probe reads the accumulator
    /// rather than needing a mechanism of its own, and it reads it *here*, which
    /// is what makes `I(di, si)` after the contributions see them and the same
    /// probe before them see zero. Both are what the sequential semantics of an
    /// analog block say.
    ///
    /// This is how the models that read back their own terminal currents work:
    /// ASM-HEMT's `idisi = mult_i*I(di,si)` under "output info variables",
    /// HICUM's and BJT505's operating-point sections. Seven of the nine models
    /// that did not lower were this one construct.
    fn contributed_flow(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> Option<ValueId> {
        let contributions = self.branch_current_contributions(pos, neg);
        if contributions.is_empty() {
            return None;
        }

        if self.frozen_contribution_current
            && let Some(frozen) = self.frozen_contributed_flow(pos, neg, &contributions)
        {
            return Some(frozen);
        }

        let mut total: Option<ValueId> = None;
        for (contribution, reversed) in contributions {
            let variable = CfgVariable::Residual(contribution);
            let accumulated = self.builder.read_variable(variable, self.block)?;
            let term = if reversed {
                self.unary(CfgUnaryOp::Neg, accumulated)
            } else {
                accumulated
            };
            total = Some(match total {
                Some(total) => self.binary(CfgBinaryOp::Add, total, term),
                None => term,
            });
        }
        total
    }

    /// Every current contribution to one branch, with whether it drives the
    /// branch the way the probe reads it.
    fn branch_current_contributions(
        &self,
        pos: Option<NodeId>,
        neg: Option<NodeId>,
    ) -> Vec<(ContributionId, bool)> {
        self.mir
            .equations
            .iter()
            .filter(|equation| equation.kind == MirEquationKind::Current)
            .filter_map(|equation| {
                let branch = &equation.branch;
                if branch.pos_node == pos && branch.neg_node == neg {
                    return Some((equation.contribution, false));
                }
                if branch.pos_node == neg && branch.neg_node == pos {
                    return Some((equation.contribution, true));
                }
                None
            })
            .collect()
    }

    /// The same probe, for a consumer that reads the contribution's current out
    /// of storage instead of recomputing it.
    ///
    /// See [`CfgLowerMode::frozen_contribution_current`] for why the two
    /// consumers differ.
    ///
    /// `None` when the walk has completed no matching contribution yet, and the
    /// caller then takes the accumulator sum it always did. That is not a
    /// second answer to the same question: every accumulator in that sum still
    /// holds the entry block's zero, so the value and its derivative are the
    /// zero the language gives a forward read, and there is no storage slot to
    /// freeze it against — the shipped route registers a probe only for a
    /// contribution it has already lowered. Refusing instead would drop a
    /// module for an expression that is constant.
    ///
    /// Inside a noise magnitude there is no "yet". The magnitudes are evaluated
    /// after the whole body, against storage the consumer has already filled
    /// for every contribution, so the probe names the branch's last current
    /// contribution whether or not the walk has reached it. See
    /// [`Self::noise_magnitude`], which is the only thing that answers "would
    /// this read be the seeded zero?" with "and that is wrong".
    fn frozen_contributed_flow(
        &mut self,
        pos: Option<NodeId>,
        neg: Option<NodeId>,
        contributions: &[(ContributionId, bool)],
    ) -> Option<ValueId> {
        let through = if self.noise_magnitude {
            contributions.last()?.0
        } else {
            *self
                .completed_current_contributions
                .iter()
                .rev()
                .find(|completed| {
                    contributions
                        .iter()
                        .any(|(contribution, _)| contribution == *completed)
                })?
        };
        Some(self.leaf(
            LeafKey::ContributedCurrent { pos, neg, through },
            CfgValueType::Real,
            CfgValueKind::ContributedCurrent { pos, neg, through },
        ))
    }

    /// The current entering the module at a terminal — `I(<p>)`.
    ///
    /// A port branch is not a branch between two nets, so nothing above finds
    /// it. What it names is Kirchhoff's law at the terminal: whatever comes in
    /// from outside is what the model's own network takes away, so the answer is
    /// the signed sum of everything incident on that node — flow contributions
    /// through their accumulators, and potential contributions through the
    /// unknown that carries their current.
    ///
    /// Restricted to external nodes. `I(<n>)` for an internal net is not a port
    /// access, and answering it with a KCL sum would turn a mistyped net into a
    /// plausible number.
    ///
    /// A branch driven by potential contributions from both arms of an `if` has
    /// two unknowns for one physical current — see [`Self::branch_unknown_by_nodes`]
    /// — so the unknowns are taken one per distinct branch, or the terminal
    /// current would double-count.
    fn port_flow(&mut self, node: NodeId) -> Option<ValueId> {
        if !self
            .mir
            .nodes
            .get(usize::from(node))
            .is_some_and(|entry| entry.is_external)
        {
            return None;
        }

        // A local copy of the shared reference, so walking MIR does not borrow
        // `self` while the builder is being written.
        let mir = self.mir;
        let mut terms: Vec<(ValueId, bool)> = Vec::new();
        // Which branches this sum has already taken whole, so a branch carrying
        // two contributions contributes its total once. Only the frozen route
        // groups: the accumulator route sums the contributions themselves, and
        // each of those is one term.
        let mut frozen_branches: HashSet<(Option<NodeId>, Option<NodeId>)> = HashSet::new();
        for equation in &mir.equations {
            if equation.kind != MirEquationKind::Current {
                continue;
            }
            let reversed = if equation.branch.pos_node == Some(node) {
                false
            } else if equation.branch.neg_node == Some(node) {
                true
            } else {
                continue;
            };
            let branch = (equation.branch.pos_node, equation.branch.neg_node);
            if self.frozen_contribution_current {
                if frozen_branches.contains(&branch) {
                    continue;
                }
                // The whole branch, through everything contributed to it so
                // far, is one leaf — the same one a direct probe of that branch
                // would read. `None` means nothing has been contributed to it
                // yet, and then the accumulators below are still the entry
                // block's zero, which is the same number with no storage to
                // freeze it against.
                let contributions = self.branch_current_contributions(branch.0, branch.1);
                if let Some(frozen) =
                    self.frozen_contributed_flow(branch.0, branch.1, &contributions)
                {
                    frozen_branches.insert(branch);
                    terms.push((frozen, reversed));
                    continue;
                }
            }
            let variable = CfgVariable::Residual(equation.contribution);
            let accumulated = self.builder.read_variable(variable, self.block)?;
            terms.push((accumulated, reversed));
        }

        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for unknown in &mir.branch_unknowns {
            let reversed = if unknown.pos_node == Some(node) {
                false
            } else if unknown.neg_node == Some(node) {
                true
            } else {
                continue;
            };
            let pos_key = unknown.pos_node.map(usize::from).unwrap_or(usize::MAX);
            let neg_key = unknown.neg_node.map(usize::from).unwrap_or(usize::MAX);
            let physical_key = if pos_key <= neg_key {
                (pos_key, neg_key)
            } else {
                (neg_key, pos_key)
            };
            if !seen.insert(physical_key) {
                continue;
            }
            let flow = self.leaf(
                LeafKey::BranchUnknownFlow(unknown.id),
                CfgValueType::Real,
                CfgValueKind::BranchUnknownFlow(unknown.id),
            );
            terms.push((flow, reversed));
        }

        let mut total: Option<ValueId> = None;
        for (term, reversed) in terms {
            let term = if reversed {
                self.unary(CfgUnaryOp::Neg, term)
            } else {
                term
            };
            total = Some(match total {
                Some(total) => self.binary(CfgBinaryOp::Add, total, term),
                None => term,
            });
        }
        total
    }

    fn named_branch_access(
        &mut self,
        access: &SmolStr,
        name: &SmolStr,
        span: SourceSpanRef,
    ) -> ValueId {
        let Some(branch) = self
            .mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name.as_str())
        else {
            self.unsupported(span, format!("access to undeclared branch '{name}'"));
            return self.real_constant(0.0);
        };
        let (pos_node, neg_node, id) = (branch.pos_node, branch.neg_node, branch.id);

        if is_flow_access(access.as_str()) {
            if let Some((unknown, reversed)) = self.branch_unknown_by_nodes(pos_node, neg_node) {
                let flow = self.leaf(
                    LeafKey::BranchUnknownFlow(unknown),
                    CfgValueType::Real,
                    CfgValueKind::BranchUnknownFlow(unknown),
                );
                return if reversed {
                    self.unary(CfgUnaryOp::Neg, flow)
                } else {
                    flow
                };
            }
            // Same situation as the unnamed form, and the same answer: what the
            // model has contributed. The runtime-supplied flow stays as the
            // fallback for a declared branch nothing contributes to.
            if let Some(contributed) = self.contributed_flow(pos_node, neg_node) {
                return contributed;
            }
            return self.leaf(
                LeafKey::BranchFlow(id),
                CfgValueType::Real,
                CfgValueKind::BranchFlow(id),
            );
        }

        self.potential_difference(pos_node, neg_node)
    }

    fn potential_difference(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> ValueId {
        let pos = pos.map(|node| {
            self.leaf(
                LeafKey::NodePotential(node),
                CfgValueType::Real,
                CfgValueKind::NodePotential(node),
            )
        });
        let neg = neg.map(|node| {
            self.leaf(
                LeafKey::NodePotential(node),
                CfgValueType::Real,
                CfgValueKind::NodePotential(node),
            )
        });
        match (pos, neg) {
            (Some(pos), Some(neg)) => self.binary(CfgBinaryOp::Sub, pos, neg),
            (Some(pos), None) => pos,
            (None, Some(neg)) => self.unary(CfgUnaryOp::Neg, neg),
            (None, None) => self.real_constant(0.0),
        }
    }

    /// `Ok(None)` is ground; `Err(())` is an endpoint that does not resolve.
    fn endpoint(&mut self, name: &SmolStr, span: SourceSpanRef) -> Result<Option<NodeId>, ()> {
        if self.ground_names.contains(name) {
            return Ok(None);
        }
        match self.nodes_by_name.get(name).copied() {
            Some(node) => Ok(Some(node)),
            None => {
                self.unsupported(span, format!("net '{name}'"));
                Err(())
            }
        }
    }

    /// The branch unknown carrying the flow between two nets, and whether the
    /// probe reads it backwards.
    ///
    /// MIR mints one unknown per potential or indirect *contribution statement*,
    /// so a branch driven from both arms of an `if` — `V(rf,si) <+ ...` in one
    /// and `V(rf,si) <+ 0.0` in the other, which is how Angelov writes an
    /// optional series resistance — produces two unknowns for one branch. A
    /// branch has one flow, so the first is the answer; the duplication is
    /// MIR's modelling of contributions, not two physical quantities.
    fn branch_unknown_by_nodes(
        &self,
        pos: Option<NodeId>,
        neg: Option<NodeId>,
    ) -> Option<(BranchUnknownId, bool)> {
        // Preserve MIR/source order across both orientations. The canonical
        // generated backend uses the first potential contribution on a physical
        // branch as its leader unknown; preferring a later exact orientation
        // over an earlier reversed one would make I(a,b) read an unused duplicate
        // that the topology layer correctly pins to zero.
        self.mir.branch_unknowns.iter().find_map(|unknown| {
            if unknown.pos_node == pos && unknown.neg_node == neg {
                Some((unknown.id, false))
            } else if unknown.pos_node == neg && unknown.neg_node == pos {
                Some((unknown.id, true))
            } else {
                None
            }
        })
    }

    fn system_function(&mut self, name: &SmolStr, args: &[ExprId], span: SourceSpanRef) -> ValueId {
        match (name.to_ascii_lowercase().as_str(), args.len()) {
            ("$temperature", 0) => self.leaf(
                LeafKey::Temperature,
                CfgValueType::Real,
                CfgValueKind::Temperature,
            ),
            ("$abstime" | "$realtime", 0) => {
                self.leaf(LeafKey::Time, CfgValueType::Real, CfgValueKind::Time)
            }
            ("$mfactor", 0) => self.leaf(
                LeafKey::Multiplicity,
                CfgValueType::Real,
                CfgValueKind::Multiplicity,
            ),
            ("$vt" | "$thermal_vt", 0) => self.leaf(
                LeafKey::ThermalVoltage,
                CfgValueType::Real,
                CfgValueKind::ThermalVoltage,
            ),
            ("$vt" | "$thermal_vt", 1) => {
                let temperature = self.expr(args[0]);
                let scale = self.real_constant(THERMAL_VOLTAGE_PER_KELVIN);
                self.binary(CfgBinaryOp::Mul, temperature, scale)
            }
            ("$simparam", 1 | 2) => self.simparam(args, span),
            ("$param_given", 1) => match self.parameter_argument(args[0]) {
                Some(parameter) => self.leaf(
                    LeafKey::ParameterGiven(parameter),
                    CfgValueType::Real,
                    CfgValueKind::ParameterGiven(parameter),
                ),
                None => {
                    self.unsupported(span, "$param_given of a non-parameter".to_string());
                    self.real_constant(0.0)
                }
            },
            // The runtime flag for a consumer that evaluates instances it did
            // not build, and the constant `1.0` for one that did.
            //
            // A generated device is instantiated with exactly the terminals its
            // descriptor declares — `veriloga_builtins::instantiate` refuses any
            // other count — so no port it can be asked about is unconnected, and
            // folding the query lets the optimizer delete every
            // `$port_connected` guard in the shipped compact models. The
            // executable backends have no such guarantee: `VerilogADevice` marks
            // terminal `i` connected only while `i < supplied_terminals`, and the
            // JIT reads that flag with `NativeOp::LoadPortConnected`. See
            // [`CfgModel::from_hir_for_executable_backend`] for which consumer
            // asks for which.
            ("$port_connected", 1) if self.per_instance_ports => {
                match self.port_argument(args[0]) {
                    Some(port) => self.leaf(
                        LeafKey::PortConnected(port),
                        CfgValueType::Real,
                        CfgValueKind::PortConnected(port),
                    ),
                    None => {
                        self.unsupported(span, "$port_connected of a non-port".to_string());
                        self.real_constant(0.0)
                    }
                }
            }
            ("$port_connected", 1) => self.real_constant(1.0),
            _ => {
                self.unsupported(span, format!("system function '{name}'"));
                self.real_constant(0.0)
            }
        }
    }

    fn simparam(&mut self, args: &[ExprId], span: SourceSpanRef) -> ValueId {
        let Some(HirExprKind::StringLiteral { value }) = self
            .hir
            .expressions
            .get(usize::from(args[0]))
            .map(|expression| expression.kind.clone())
        else {
            self.unsupported(span, "$simparam with a non-literal name".to_string());
            return self.real_constant(0.0);
        };
        if self.noise_metadata_only {
            if let Some(fallback) = args.get(1) {
                return self.expr(*fallback);
            }
            return self.real_constant(simparam_source_default(value.as_str()));
        }

        // Ordinary generated devices receive simulator-owned values (most
        // importantly the gmin continuation value) on every Newton call. Keep
        // the source fallback in the CFG, but do not replace the runtime leaf
        // with it. Metadata-only noise evaluation has no such runtime input,
        // so the closed defaults above are deliberately confined to that mode.
        //
        // A call that names no fallback still needs one, and it is not zero:
        // the source says nothing, so the value the language defines for the
        // parameter is what the call means. `simparam_source_default` is the
        // same table the bytecode route folds at compile time, so a runtime
        // with no value for the name and the compiled route answer alike.
        let fallback = match args.get(1) {
            Some(fallback) => self.expr(*fallback),
            None => self.real_constant(simparam_source_default(value.as_str())),
        };
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::SimParam {
                name: SmolStr::new(value.to_ascii_lowercase()),
                fallback,
            },
        )
    }

    fn parameter_argument(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.hir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.parameters_by_name.get(name).copied()
    }

    fn port_argument(&self, expr: ExprId) -> Option<u32> {
        let expression = self.hir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.hir
            .ports
            .iter()
            .position(|port| port.name == *name)
            .and_then(|index| u32::try_from(index).ok())
    }

    fn call(
        &mut self,
        expression: ExprId,
        name: &SmolStr,
        args: &[ExprId],
        span: SourceSpanRef,
    ) -> ValueId {
        let lowered = name.to_ascii_lowercase();
        if is_noise_name(lowered.as_str()) {
            return self.real_constant(0.0);
        }
        match (lowered.as_str(), args.len()) {
            // With both optional rates omitted the LRM defines `slew` as an
            // exact stateless passthrough, so the CFG can lower it directly.
            ("slew", 1) => self.expr(args[0]),
            ("slew", 2 | 3) => {
                let input = self.expr(args[0]);
                let max_rise = self.expr(args[1]);
                let max_fall = args.get(2).map(|value| self.expr(*value));
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Slew {
                        operator: expression,
                        input,
                        max_rise,
                        max_fall,
                    },
                )
            }
            // The route the compiler's own parser takes: these are calls in the
            // AST, and the four spellings of each family differ only in which
            // half of the transfer function is named by roots.
            ("laplace_zp" | "laplace_zd" | "laplace_np" | "laplace_nd", 3) => {
                let Some(form) = FilterForm::from_operator(lowered.as_str()) else {
                    self.unsupported(span, format!("function '{name}'"));
                    return self.real_constant(0.0);
                };
                let numerator = self.filter_operand_list(args[1]);
                let denominator = self.filter_operand_list(args[2]);
                self.laplace(expression, args[0], form, &numerator, &denominator, span)
            }
            ("zi_zp" | "zi_zd" | "zi_np" | "zi_nd", 4..=6) => {
                let Some(form) = FilterForm::from_operator(lowered.as_str()) else {
                    self.unsupported(span, format!("function '{name}'"));
                    return self.real_constant(0.0);
                };
                let numerator = self.filter_operand_list(args[1]);
                let denominator = self.filter_operand_list(args[2]);
                let transition = args.get(4).copied().filter(|argument| {
                    !matches!(
                        self.hir
                            .expressions
                            .get(usize::from(*argument))
                            .map(|expression| &expression.kind),
                        Some(HirExprKind::NullArgument)
                    )
                });
                let first_transition = args.get(5).copied();
                self.zi(
                    expression,
                    args[0],
                    form,
                    &numerator,
                    &denominator,
                    args[3],
                    transition,
                    first_transition,
                    span,
                )
            }
            // Keyed by the call, like `ddt` and `idt` above: the transport
            // buffer, the wrapped total and the detector are per-instance slots
            // named by the operator, and keying one by its argument would name
            // a slot no backend allocated.
            ("absdelay", 2 | 3) => {
                let input = self.expr(args[0]);
                let delay = self.expr(args[1]);
                let max_delay = args.get(2).map(|value| self.expr(*value));
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::AbsDelay {
                        operator: expression,
                        input,
                        delay,
                        max_delay,
                    },
                )
            }
            ("idtmod", 1 | 2) => self.idt(expression, args[0], args.get(1).copied()),
            ("idtmod", 3 | 4) => {
                let input = self.expr(args[0]);
                let ic = self.expr(args[1]);
                let modulus = self.expr(args[2]);
                let offset = self.optional_argument(args, 3, 0.0);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::IdtMod {
                        operator: expression,
                        input,
                        ic,
                        modulus,
                        offset,
                    },
                )
            }
            ("last_crossing", 1 | 2) => {
                let input = self.expr(args[0]);
                let direction = self.optional_argument(args, 1, 0.0);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::LastCrossing {
                        operator: expression,
                        input,
                        direction,
                    },
                )
            }
            // Keyed by the *call*, not by its argument. The state-slot
            // allocation every backend reads keys on the call, so keying this
            // one by the operand named a slot that does not exist.
            ("ddt", 1) => {
                let input = self.expr(args[0]);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Ddt {
                        operator: expression,
                        input,
                    },
                )
            }
            // Keyed by the call for the same reason. One argument means no
            // initial condition, which the LRM says is zero; three or four add
            // `assert` and `abstol`, which the operator path refuses too.
            ("idt", 1 | 2) => {
                let input = self.expr(args[0]);
                let ic = match args.get(1) {
                    Some(ic) => self.expr(*ic),
                    None => self.real_constant(0.0),
                };
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Idt {
                        operator: expression,
                        input,
                        ic,
                    },
                )
            }
            ("cross", 1..=5) => {
                let input = self.expr(args[0]);
                let direction = self.optional_argument(args, 1, 0.0);
                let time_tol = self.optional_argument(args, 2, 0.0);
                let expr_tol = self.optional_argument(args, 3, 0.0);
                let enable = self.optional_argument(args, 4, 1.0);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Cross {
                        operator: expression,
                        input,
                        direction,
                        time_tol,
                        expr_tol,
                        enable,
                    },
                )
            }
            ("above", 1..=4) => {
                let input = self.expr(args[0]);
                let time_tol = self.optional_argument(args, 1, 0.0);
                let expr_tol = self.optional_argument(args, 2, 0.0);
                let enable = self.optional_argument(args, 3, 1.0);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Above {
                        operator: expression,
                        input,
                        time_tol,
                        expr_tol,
                        enable,
                    },
                )
            }
            ("timer", 1..=4) => {
                let start = self.expr(args[0]);
                let period = self.optional_argument(args, 1, 0.0);
                let time_tol = self.optional_argument(args, 2, 0.0);
                let enable = self.optional_argument(args, 3, 1.0);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Timer {
                        operator: expression,
                        start,
                        period,
                        time_tol,
                        enable,
                    },
                )
            }
            ("analysis", count) if count > 0 => {
                let mut arguments = args.iter();
                let first = self.analysis_call(
                    *arguments.next().expect("nonempty analysis argument list"),
                    span,
                );
                let mut combined = first;
                for argument in arguments {
                    let right = self.analysis_call(*argument, span);
                    combined = self.binary(CfgBinaryOp::Or, combined, right);
                }
                combined
            }
            ("ddx", 2) => self.ddx(args[0], args[1], span),
            ("expm1", 1) => {
                let input = self.expr(args[0]);
                let exp = self.unary(CfgUnaryOp::Exp, input);
                let one = self.real_constant(1.0);
                self.binary(CfgBinaryOp::Sub, exp, one)
            }
            ("log1p", 1) => {
                let input = self.expr(args[0]);
                let one = self.real_constant(1.0);
                let sum = self.binary(CfgBinaryOp::Add, one, input);
                self.unary(CfgUnaryOp::Ln, sum)
            }
            ("min" | "max" | "pow" | "fpow" | "hypot" | "atan2", 2) => {
                let op = match lowered.as_str() {
                    "min" => CfgBinaryOp::Min,
                    "max" => CfgBinaryOp::Max,
                    "pow" | "fpow" => CfgBinaryOp::Pow,
                    "hypot" => CfgBinaryOp::Hypot,
                    _ => CfgBinaryOp::Atan2,
                };
                let left = self.expr(args[0]);
                let right = self.expr(args[1]);
                self.binary(op, left, right)
            }
            (_, 1) => {
                let Some(op) = unary_intrinsic(lowered.as_str()) else {
                    self.unsupported(span, format!("function '{name}'"));
                    return self.real_constant(0.0);
                };
                let input = self.expr(args[0]);
                self.unary(op, input)
            }
            _ => {
                self.unsupported(span, format!("function '{name}'"));
                self.real_constant(0.0)
            }
        }
    }

    /// `ddx(expr, probe)`, left symbolic for the derivative pass.
    fn ddx(&mut self, expr: ExprId, probe: ExprId, span: SourceSpanRef) -> ValueId {
        let Some(axis) = self.ddx_probe(probe, span) else {
            return self.real_constant(0.0);
        };
        let value = self.expr(expr);
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::Ddx { value, axis },
        )
    }

    /// The solver-owned axis a `ddx` probe names. A flow is a valid axis only
    /// when topology has introduced a branch-current unknown for it; currents
    /// synthesized from flow contributions and terminal currents are dependent
    /// values, not independent Newton coordinates.
    fn ddx_probe(&mut self, probe: ExprId, span: SourceSpanRef) -> Option<CfgDdxAxis> {
        let kind = self
            .hir
            .expressions
            .get(usize::from(probe))
            .map(|expression| expression.kind.clone());
        match kind {
            Some(HirExprKind::BranchAccess { access, pos, neg }) => {
                let pos_node = self.endpoint(&pos, span).ok()?;
                let neg_node = match neg {
                    Some(neg) => self.endpoint(&neg, span).ok()?,
                    None => None,
                };
                if !is_flow_access(&access) {
                    return Some(CfgDdxAxis::Potential { pos_node, neg_node });
                }
                match self.branch_unknown_by_nodes(pos_node, neg_node) {
                    Some((unknown, reversed)) => Some(CfgDdxAxis::BranchFlow { unknown, reversed }),
                    None => {
                        self.unsupported(
                            span,
                            "ddx flow probe requires a solver-owned branch-current unknown from a potential or indirect contribution".to_string(),
                        );
                        None
                    }
                }
            }
            Some(HirExprKind::NamedBranchAccess { access, name }) => {
                let branch = self.mir.branches.iter().find(|branch| branch.name == name);
                let Some(branch) = branch else {
                    self.unsupported(span, format!("unknown ddx probe branch '{name}'"));
                    return None;
                };
                if !is_flow_access(&access) {
                    return Some(CfgDdxAxis::Potential {
                        pos_node: branch.pos_node,
                        neg_node: branch.neg_node,
                    });
                }
                match self.branch_unknown_by_nodes(branch.pos_node, branch.neg_node) {
                    Some((unknown, reversed)) => Some(CfgDdxAxis::BranchFlow { unknown, reversed }),
                    None => {
                        self.unsupported(
                            span,
                            format!(
                                "ddx flow probe I(<{name}>) requires a solver-owned branch-current unknown from a potential or indirect contribution"
                            ),
                        );
                        None
                    }
                }
            }
            _ => {
                self.unsupported(
                    span,
                    "a ddx probe that is not a branch potential or solver-owned branch flow"
                        .to_string(),
                );
                None
            }
        }
    }

    fn analysis_call(&mut self, argument: ExprId, span: SourceSpanRef) -> ValueId {
        let Some(HirExprKind::StringLiteral { value }) = self
            .hir
            .expressions
            .get(usize::from(argument))
            .map(|expression| expression.kind.clone())
        else {
            self.unsupported(span, "analysis() with a non-literal name".to_string());
            return self.real_constant(0.0);
        };
        let name = SmolStr::new(value.to_ascii_lowercase());
        self.leaf(
            LeafKey::Analysis(name.clone()),
            CfgValueType::Boolean,
            CfgValueKind::Analysis(name),
        )
    }

    fn analog_operator(
        &mut self,
        op: &HirAnalogOperator,
        expression: &HirExpression,
        span: SourceSpanRef,
    ) -> ValueId {
        match op {
            HirAnalogOperator::Limit {
                proposed,
                candidate,
                selector,
                ..
            } => {
                let proposed = self.expr(*proposed);
                // The candidate is the limiter's inlined body; its implicit
                // arguments are resolved against this call, so the enclosing
                // operator has to be on the stack while it is walked.
                self.limiters.push(Limiter {
                    operator: expression.id,
                    proposed,
                });
                let candidate = self.expr(*candidate);
                self.limiters.pop();
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Limit {
                        operator: expression.id,
                        proposed,
                        candidate,
                        selector: selector.clone(),
                    },
                )
            }
            HirAnalogOperator::LimiterArgument { argument } => {
                let Some(limiter) = self.limiters.last().copied() else {
                    self.unsupported(span, "a limiter argument outside a $limit".to_string());
                    return self.real_constant(0.0);
                };
                match argument {
                    HirLimiterArgument::Proposed => limiter.proposed,
                    HirLimiterArgument::Previous => self.builder.push(
                        self.block,
                        CfgValueType::Real,
                        CfgValueKind::LimitPrevious {
                            operator: limiter.operator,
                            proposed: limiter.proposed,
                        },
                    ),
                }
            }
        }
    }

    /// `idt(x, ic)`, keyed by the call.
    ///
    /// Shared by `idt` and by `idtmod` written without a modulus, which the LRM
    /// makes the same integral: keying both here is what keeps the two source
    /// forms on one state slot instead of two.
    fn idt(&mut self, operator: ExprId, expr: ExprId, ic: Option<ExprId>) -> ValueId {
        let input = self.expr(expr);
        // Absent means zero, which is what the LRM says an unstated initial
        // condition is.
        let ic = self.optional_expr(ic, 0.0);
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::Idt {
                operator,
                input,
                ic,
            },
        )
    }

    /// An optional operand, or the constant the LRM gives an omitted one.
    fn optional_expr(&mut self, expr: Option<ExprId>, absent: f64) -> ValueId {
        match expr {
            Some(expr) => self.expr(expr),
            None => self.real_constant(absent),
        }
    }

    fn real_constant(&mut self, value: f64) -> ValueId {
        self.leaf(
            LeafKey::RealConstant(value.to_bits()),
            CfgValueType::Real,
            CfgValueKind::RealConstant(value),
        )
    }

    fn leaf(&mut self, key: LeafKey, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        if let Some(value) = self.leaves.get(&key) {
            return *value;
        }
        let value = self.builder.push_leaf(value_type, kind);
        self.leaves.insert(key, value);
        value
    }

    /// One optional operand of an event operator, or `fallback` where the
    /// source left it out.
    ///
    /// A trailing operand is simply absent, but an operand skipped in the
    /// middle — `cross(v, , 1e-9)` — arrives as an explicit null, which is a
    /// perfectly ordinary way to reach a later default and not a value the
    /// walk can lower. Both spellings mean the same thing and both resolve
    /// here, so the level agrees with the executable backends on what a
    /// defaulted tolerance is.
    fn optional_argument(&mut self, args: &[ExprId], index: usize, fallback: f64) -> ValueId {
        match args.get(index) {
            Some(argument)
                if !matches!(
                    self.hir
                        .expressions
                        .get(usize::from(*argument))
                        .map(|expression| &expression.kind),
                    Some(HirExprKind::NullArgument)
                ) =>
            {
                self.expr(*argument)
            }
            _ => self.real_constant(fallback),
        }
    }

    fn unary(&mut self, op: CfgUnaryOp, input: ValueId) -> ValueId {
        self.unary_typed(op, input, CfgValueType::Real)
    }

    fn unary_typed(&mut self, op: CfgUnaryOp, input: ValueId, value_type: CfgValueType) -> ValueId {
        self.builder
            .push(self.block, value_type, CfgValueKind::Unary { op, input })
    }

    fn binary(&mut self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        let value_type = if is_predicate(op) {
            CfgValueType::Boolean
        } else {
            CfgValueType::Real
        };
        self.builder.push(
            self.block,
            value_type,
            CfgValueKind::Binary { op, left, right },
        )
    }

    fn unsupported(&mut self, span: SourceSpanRef, what: String) {
        self.diagnostics.push(IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!("CFG lowering does not support {what}"),
            span,
        ));
    }

    fn warn(&mut self, span: SourceSpanRef, what: String) {
        self.diagnostics.push(IrDiagnostic::warning(
            CompilerPhase::CfgLowering,
            what,
            span,
        ));
    }
}

const THERMAL_VOLTAGE_PER_KELVIN: f64 = 8.617_333_262e-5;

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.trim_start_matches('$'),
        "white_noise" | "flicker_noise" | "noise_table" | "noise_table_log"
    )
}

fn is_flow_access(access: &str) -> bool {
    is_standard_flow_access(access)
}

fn is_predicate(op: CfgBinaryOp) -> bool {
    matches!(
        op,
        CfgBinaryOp::Eq
            | CfgBinaryOp::Ne
            | CfgBinaryOp::Lt
            | CfgBinaryOp::Le
            | CfgBinaryOp::Gt
            | CfgBinaryOp::Ge
            | CfgBinaryOp::And
            | CfgBinaryOp::Or
    )
}

/// HIR spells operators with the AST enum's variant name, not the source token.
fn binary_op(op: &str) -> Option<CfgBinaryOp> {
    Some(match op {
        "Add" => CfgBinaryOp::Add,
        "Sub" => CfgBinaryOp::Sub,
        "Mul" => CfgBinaryOp::Mul,
        "Div" => CfgBinaryOp::Div,
        "Mod" => CfgBinaryOp::Mod,
        "Pow" => CfgBinaryOp::Pow,
        "Eq" => CfgBinaryOp::Eq,
        "Ne" => CfgBinaryOp::Ne,
        "Lt" => CfgBinaryOp::Lt,
        "Le" => CfgBinaryOp::Le,
        "Gt" => CfgBinaryOp::Gt,
        "Ge" => CfgBinaryOp::Ge,
        "And" => CfgBinaryOp::And,
        "Or" => CfgBinaryOp::Or,
        _ => return None,
    })
}

/// Whether a call spells one of the dynamic operators this mode substitutes
/// with a zero primal instead of implementing.
///
/// Deliberately only the operators this level newly represents. `ddt`, `idt`,
/// `cross`, `above` and `timer` have always lowered here, so their metadata
/// behaviour is settled and is not this list's to change.
fn is_dynamic_operator_call(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "absdelay" | "slew" | "idtmod" | "last_crossing"
    )
}

/// The analog `integer` operators, kept out of [`binary_op`] because their
/// result type is not a real function of two reals.
fn integer_bitwise_op(op: &str) -> Option<CfgIntegerBitwiseOp> {
    Some(match op {
        "BitAnd" => CfgIntegerBitwiseOp::And,
        "BitOr" => CfgIntegerBitwiseOp::Or,
        "BitXor" => CfgIntegerBitwiseOp::Xor,
        "Shl" => CfgIntegerBitwiseOp::Shl,
        "Shr" => CfgIntegerBitwiseOp::Shr,
        _ => return None,
    })
}

fn unary_intrinsic(name: &str) -> Option<CfgUnaryOp> {
    Some(match name {
        "exp" => CfgUnaryOp::Exp,
        "limexp" => CfgUnaryOp::LimExp,
        "__rspice_limited_exp" => CfgUnaryOp::LimitedExp,
        "ln" | "log" => CfgUnaryOp::Ln,
        "log10" => CfgUnaryOp::Log10,
        "sqrt" => CfgUnaryOp::Sqrt,
        "abs" | "fabs" => CfgUnaryOp::Abs,
        "sin" => CfgUnaryOp::Sin,
        "cos" => CfgUnaryOp::Cos,
        "tan" => CfgUnaryOp::Tan,
        "sinh" => CfgUnaryOp::Sinh,
        "cosh" => CfgUnaryOp::Cosh,
        "tanh" => CfgUnaryOp::Tanh,
        "asin" => CfgUnaryOp::Asin,
        "acos" => CfgUnaryOp::Acos,
        "atan" => CfgUnaryOp::Atan,
        "asinh" => CfgUnaryOp::Asinh,
        "acosh" => CfgUnaryOp::Acosh,
        "atanh" => CfgUnaryOp::Atanh,
        "floor" => CfgUnaryOp::Floor,
        "ceil" => CfgUnaryOp::Ceil,
        _ => return None,
    })
}

fn kind_label(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::NullArgument => "null argument",
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string literal",
        HirExprKind::Identifier { .. } => "identifier",
        HirExprKind::SystemFunction { .. } => "system function",
        HirExprKind::Binary { .. } => "binary",
        HirExprKind::Unary { .. } => "unary",
        HirExprKind::Conditional { .. } => "conditional",
        HirExprKind::Call { .. } => "call",
        HirExprKind::BranchAccess { .. } => "branch access",
        HirExprKind::NamedBranchAccess { .. } => "named branch access",
        HirExprKind::ArrayAccess { .. } => "array access",
        HirExprKind::ArrayLiteral { .. } => "array literal",
        HirExprKind::AnalogOperator { .. } => "analog operator",
        HirExprKind::NoiseSource { .. } => "noise source",
    }
}
