//! The state layout: which operator site owns which piece of runtime state.
//!
//! A Verilog-A module's dynamic operators do not only compute — they *remember*.
//! `ddt` keeps the previous accepted operand, `absdelay` keeps a transport
//! queue, `zi_nd` keeps sampled input and output history. Every backend has to
//! agree on which of those records belongs to which operator, or a resumed
//! checkpoint integrates one operator's history into another's.
//!
//! Until this module existed, that agreement was reconstructed rather than
//! recorded. [`crate::native::expr`] walked the canonical expression tree
//! collecting dynamic operators, walked the *bytecode program* collecting state
//! instructions, and paired the two lists positionally — thirteen times per
//! program, once per operator family. The identity of a state record was
//! therefore a bytecode position, which is exactly the property a CFG-sourced
//! backend cannot reproduce: there is no bytecode to count.
//!
//! [`CanonicalStateLayout`] is that agreement written down instead. It is built
//! from the canonical IR alone and answers, for every state-bearing operator in
//! a module, which family its record lives in, which dense slot inside that
//! family it owns, and what the record's shape is.
//!
//! ## The key
//!
//! A site is keyed by the [`ExprId`] of the operator expression, and numbered by
//! its rank within its family along one walk of the module's executed roots —
//! [`HirModel::statements`] in order, then [`HirModel::contributions`] in order,
//! post-order within each. That is the order the bytecode generator emits in,
//! which is why the numbering agrees with the allocation the shipped runtimes,
//! their checkpoints, and the conformance identity pins are written in.
//!
//! Keying by the operator expression rather than by a position is the whole
//! point: [`crate::canonical_ir::cfg`]'s stateful value kinds carry the same
//! `ExprId` in their `operator` field, so a value in a CFG can name the record
//! it reads without anything having to count instructions.
//!
//! ## Two copies of every expression
//!
//! The numbering walks roots rather than the arena, and it has to. [`HirModel`]
//! lowers each module **twice** into one expression arena: once as the flat
//! `contributions` and `statements` the existing backends execute, and again as
//! the structured `body` the CFG level consumes. Every source operator therefore
//! appears in the arena twice, under two different `ExprId`s — and, for
//! `transition`, under two different [`crate::ir::TransitionSiteId`] ordinals,
//! because the preorder counter that mints them runs across both copies.
//!
//! This layout numbers the executed copy, because that is the copy whose records
//! the runtime allocates and the checkpoint serializes. The consequence for a
//! CFG-sourced backend is the concrete shape of the migration blocker: a CFG
//! value's `operator` id belongs to the *body* copy, so it does not appear in
//! this layout at all. Resolving that is not a numbering problem to be papered
//! over with a positional pairing — the two root lists are ordered differently
//! (`statements` then `contributions` here, interleaved in source order in the
//! body), so a per-family positional pairing between them is wrong for any
//! module that contributes before it assigns. It is a canonical-IR problem: the
//! two routes have to become one, or `HirModel` has to carry the correspondence,
//! before compilation can move onto the CFG.
//!
//! ## What this numbering is *not*
//!
//! It is not the legacy bytecode slot. The two spaces are genuinely different
//! sizes, and the difference is structural rather than incidental: the bytecode
//! generator allocates a fresh scalar-state slot at each *emission* of an
//! integration operator, and a module with noise in an assignment is emitted
//! twice — once as `assignment_steps` and again as `noise_assignment_steps`
//! (`DeviceIR::noise_assignments` is a clone of `assignments` carrying noise
//! shadows). One canonical `ddt` site therefore owns two bytecode slots.
//!
//! So a CFG-sourced backend cannot adopt the bytecode numbering; it allocates
//! from this layout, and the point at which shipped code does that is the point
//! at which the accepted-state arrays are re-indexed and the runtime checkpoint
//! state version has to move. Nothing here changes either yet:
//! [`crate::native::expr`] still reads its slot *numbers* from the program it
//! is lowering, and takes only the identity and the order from this module.

use std::collections::{HashMap, HashSet};

use super::cfg::{CfgFunction, CfgStateSite};
use super::hir::{HirAnalogOperator, HirExprKind, HirExpression, HirModel, HirStatement};
use super::ids::ExprId;
use crate::ir::TransitionSiteId;

/// Which runtime array a site's record lives in.
///
/// One variant per accepted-state field of the runtime's checkpoint, plus the
/// compiled lookup tables, which are addressed by the same slot mechanism
/// without being accepted state at all. Grouping by *storage* rather than by
/// spelling is what makes this a layout: `cross` and `last_crossing` are two
/// operators reading one detector, and `ddt`, `idt`, `idtmod` and `$limit` all
/// draw from the module's scalar state lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalStateFamily {
    /// The module's parallel scalar lanes: `state_values_prev`,
    /// `state_values_older`, `state_derivatives_prev` and `state_initialized`,
    /// all indexed by the same slot.
    Integration,
    /// `delay_buffers` — one transport queue and its frozen configuration.
    DelayBuffer,
    /// `transition_filters` — one accepted segment plus its pending queue.
    TransitionFilter,
    /// `slew_filters` — one committed output, its time, and the next corner.
    SlewFilter,
    /// `cross_detectors` — one committed `(value, time)` pair and the last
    /// event and crossing times. `above` draws from this array too: it is a
    /// level detector rather than an edge one, but the record it needs is the
    /// same and the bytecode allocator has always given it one from here.
    CrossDetector,
    /// `laplace_filters` — the state-space realization's state vector.
    LaplaceFilter,
    /// `zi_filters` — the frozen definition plus sampled input/output history.
    ZiFilter,
    /// The module's single `timer_event_bound`. Timer sites are numbered so
    /// that the operators can be told apart and so that a lowering can address
    /// one, but there is no per-slot record: every `timer` in a module
    /// contributes to one earliest-event bound, and the runtime's `TimerState`
    /// instruction ignores the id it carries.
    TimerEvent,
    /// `CompiledModel::lookup_tables` — compiled read-only data, not accepted
    /// state, and never checkpointed. Here because `$table_model` is addressed
    /// by the same per-model dense slot and the same correlation machinery.
    LookupTable,
}

impl CanonicalStateFamily {
    /// The accepted-state field this family occupies, or `None` when the family
    /// owns no checkpointed record.
    ///
    /// Named rather than described so that a consumer can check its own
    /// serialization against the layout by field name instead of by comment.
    pub fn checkpoint_field(self) -> Option<&'static str> {
        match self {
            Self::Integration => Some("state_values_prev"),
            Self::DelayBuffer => Some("delay_buffers"),
            Self::TransitionFilter => Some("transition_filters"),
            Self::SlewFilter => Some("slew_filters"),
            Self::CrossDetector => Some("cross_detectors"),
            Self::LaplaceFilter => Some("laplace_filters"),
            Self::ZiFilter => Some("zi_filters"),
            Self::TimerEvent => Some("timer_event_bound"),
            Self::LookupTable => None,
        }
    }

    /// Whether the family stores one record per slot.
    ///
    /// False only for [`Self::TimerEvent`], where every site in the module
    /// folds into one earliest-event bound. A slot allocator has to ask before
    /// reserving storage, or it reserves an array for a scalar.
    pub fn has_per_slot_record(self) -> bool {
        !matches!(self, Self::TimerEvent)
    }

    /// Whether the family's per-slot record has a length fixed at compile time.
    ///
    /// The distinction a slot allocator needs: a fixed record can be laid out
    /// as a stride, a variable one needs indirection. `absdelay` and
    /// `transition` keep queues whose length is a function of the accepted
    /// trajectory; `laplace` and `zi` keep vectors whose length is fixed by the
    /// transfer function's degree, which is syntactic — see the coefficient
    /// contract on [`super::cfg::CfgValueKind::Zi`].
    pub fn has_fixed_record(self) -> bool {
        match self {
            Self::Integration
            | Self::SlewFilter
            | Self::CrossDetector
            | Self::LaplaceFilter
            | Self::ZiFilter
            | Self::TimerEvent
            | Self::LookupTable => true,
            Self::DelayBuffer | Self::TransitionFilter => false,
        }
    }
}

/// A state-bearing operator, classified by the record it addresses.
///
/// Deliberately coarser than the source spelling. `cross` and `last_crossing`
/// are one variant because they share a detector; `idtmod` with no modulus is
/// [`Self::Idt`] because that is the record the front end gives it; `slew` with
/// no rate limit appears nowhere, because the LRM makes it an exact passthrough
/// that owns no state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalStateOperator {
    Ddt,
    Idt,
    IdtMod,
    Transition,
    Slew,
    Absdelay,
    Laplace,
    Zi,
    Cross,
    Above,
    Timer,
    Limit,
    TableLookup,
}

impl CanonicalStateOperator {
    /// Every operator, in the order a layout reports its families.
    pub const ALL: [Self; 13] = [
        Self::Ddt,
        Self::Idt,
        Self::IdtMod,
        Self::Transition,
        Self::Slew,
        Self::Absdelay,
        Self::Laplace,
        Self::Zi,
        Self::Cross,
        Self::Above,
        Self::Timer,
        Self::Limit,
        Self::TableLookup,
    ];

    /// Position in [`Self::ALL`], for indexing a per-operator table.
    pub fn index(self) -> usize {
        match self {
            Self::Ddt => 0,
            Self::Idt => 1,
            Self::IdtMod => 2,
            Self::Transition => 3,
            Self::Slew => 4,
            Self::Absdelay => 5,
            Self::Laplace => 6,
            Self::Zi => 7,
            Self::Cross => 8,
            Self::Above => 9,
            Self::Timer => 10,
            Self::Limit => 11,
            Self::TableLookup => 12,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Ddt => "ddt",
            Self::Idt => "idt",
            Self::IdtMod => "idtmod",
            Self::Transition => "transition",
            Self::Slew => "slew",
            Self::Absdelay => "absdelay",
            Self::Laplace => "laplace",
            Self::Zi => "zi",
            Self::Cross => "cross",
            Self::Above => "above",
            Self::Timer => "timer",
            Self::Limit => "limit",
            Self::TableLookup => "table_model",
        }
    }

    /// The storage this operator addresses.
    pub fn family(self) -> CanonicalStateFamily {
        match self {
            Self::Ddt | Self::Idt | Self::IdtMod | Self::Limit => CanonicalStateFamily::Integration,
            Self::Transition => CanonicalStateFamily::TransitionFilter,
            Self::Slew => CanonicalStateFamily::SlewFilter,
            Self::Absdelay => CanonicalStateFamily::DelayBuffer,
            Self::Laplace => CanonicalStateFamily::LaplaceFilter,
            Self::Zi => CanonicalStateFamily::ZiFilter,
            Self::Cross | Self::Above => CanonicalStateFamily::CrossDetector,
            Self::Timer => CanonicalStateFamily::TimerEvent,
            Self::TableLookup => CanonicalStateFamily::LookupTable,
        }
    }

    /// Whether a call or system function of this name and arity owns a record
    /// of this operator's kind.
    pub fn matches_call(self, name: &str, arg_count: usize) -> bool {
        let normalized = normalized_intrinsic_name(name);
        // `slew(expr)` is specified to be an exact passthrough. It has no
        // dynamic state and therefore must not consume (or try to correlate)
        // a bytecode filter slot.
        if matches!(self, Self::Slew) && normalized == "slew" && arg_count == 1 {
            return false;
        }
        if normalized == "idtmod" {
            return match self {
                Self::Idt => arg_count <= 2,
                Self::IdtMod => arg_count >= 3,
                _ => false,
            };
        }
        match self {
            Self::Cross => matches!(normalized.as_str(), "cross" | "last_crossing"),
            Self::Laplace => matches!(
                normalized.as_str(),
                "laplace_zp" | "laplace_zd" | "laplace_np" | "laplace_nd"
            ),
            Self::Zi => matches!(normalized.as_str(), "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"),
            _ => normalized == self.name(),
        }
    }

    /// Whether a resolved analog operator owns a record of this kind.
    pub fn matches_operator(self, op: &HirAnalogOperator) -> bool {
        match (self, op) {
            (Self::Limit, HirAnalogOperator::Limit { .. }) => true,
            (Self::Ddt, HirAnalogOperator::Ddt { .. }) => true,
            (Self::Idt, HirAnalogOperator::Idt { .. }) => true,
            (Self::Idt, HirAnalogOperator::IdtMod { modulus: None, .. }) => true,
            (
                Self::IdtMod,
                HirAnalogOperator::IdtMod {
                    modulus: Some(_), ..
                },
            ) => true,
            (
                Self::Transition,
                HirAnalogOperator::Transition { .. }
                | HirAnalogOperator::TransitionDerivative { .. },
            ) => true,
            (
                Self::Slew,
                HirAnalogOperator::Slew {
                    max_rise: Some(_), ..
                },
            ) => true,
            (Self::Absdelay, HirAnalogOperator::Absdelay { .. }) => true,
            (Self::Cross, HirAnalogOperator::LastCrossing { .. }) => true,
            _ => false,
        }
    }
}

fn normalized_intrinsic_name(name: &str) -> String {
    name.strip_prefix('$').unwrap_or(name).to_ascii_lowercase()
}

/// Which operator kind, if any, a canonical expression owns a record for.
///
/// At most one: the predicates above partition the state-bearing expressions,
/// which is what lets one walk serve every family instead of one walk each.
pub fn classify(kind: &HirExprKind) -> Option<CanonicalStateOperator> {
    match kind {
        HirExprKind::Laplace { .. } => Some(CanonicalStateOperator::Laplace),
        HirExprKind::Zi { .. } => Some(CanonicalStateOperator::Zi),
        HirExprKind::SystemFunction { name, args } | HirExprKind::Call { name, args } => {
            CanonicalStateOperator::ALL
                .into_iter()
                .find(|operator| operator.matches_call(name, args.len()))
        }
        HirExprKind::AnalogOperator { op } => CanonicalStateOperator::ALL
            .into_iter()
            .find(|operator| operator.matches_operator(op)),
        _ => None,
    }
}

/// An expression id that is not in the arena it was read against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingExpression(pub ExprId);

impl std::fmt::Display for MissingExpression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "canonical expression {} is outside the expression arena",
            self.0
        )
    }
}

/// Visit every state-bearing operator under `root`, in the order the backends
/// lower it: operands before the operator that owns them.
///
/// The one traversal. Both the layout's numbering and the native JIT's
/// correlation against a bytecode program are built on it, so a new operator
/// becomes visible to both by being classified once rather than by being added
/// to two walks that could drift apart.
pub fn visit_state_sites(
    expressions: &[HirExpression],
    root: ExprId,
    visit: &mut impl FnMut(ExprId, CanonicalStateOperator),
) -> Result<(), MissingExpression> {
    let expression = expressions
        .get(usize::from(root))
        .ok_or(MissingExpression(root))?;

    match &expression.kind {
        HirExprKind::NullArgument
        | HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => {}
        HirExprKind::SystemFunction { args, .. }
        | HirExprKind::Call { args, .. }
        | HirExprKind::ArrayLiteral { elements: args, .. }
        | HirExprKind::NoiseSource { operands: args, .. } => {
            visit_list(expressions, args, visit)?;
        }
        HirExprKind::Unary { operand, .. } | HirExprKind::ArrayAccess { index: operand, .. } => {
            visit_state_sites(expressions, *operand, visit)?;
        }
        HirExprKind::Binary { left, right, .. } => {
            visit_state_sites(expressions, *left, visit)?;
            visit_state_sites(expressions, *right, visit)?;
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            visit_state_sites(expressions, *condition, visit)?;
            visit_state_sites(expressions, *then_expr, visit)?;
            visit_state_sites(expressions, *else_expr, visit)?;
        }
        HirExprKind::AnalogOperator { op } => {
            visit_analog_operator_children(expressions, op, visit)?;
        }
        HirExprKind::Laplace { expr, kind } => {
            visit_state_sites(expressions, *expr, visit)?;
            let (numerator, denominator) = kind.polynomials();
            visit_list(expressions, numerator, visit)?;
            visit_list(expressions, denominator, visit)?;
        }
        HirExprKind::Zi {
            expr,
            kind,
            period,
            transition,
            first_transition,
        } => {
            // This walk order predates the layout and is NOT the generator's
            // emission order (numerator, denominator, period, first_transition,
            // expr, transition). A state-bearing operator nested inside a zi
            // operand would be numbered differently by the two; no shipped or
            // corpus module nests one there, and reordering would move slot
            // numbers, so the divergence stands until the layout allocates.
            visit_state_sites(expressions, *expr, visit)?;
            visit_state_sites(expressions, *period, visit)?;
            for child in [*transition, *first_transition].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
            let (numerator, denominator) = kind.polynomials();
            visit_list(expressions, numerator, visit)?;
            visit_list(expressions, denominator, visit)?;
        }
    }

    if let Some(operator) = classify(&expression.kind) {
        visit(root, operator);
    }

    Ok(())
}

fn visit_list(
    expressions: &[HirExpression],
    roots: &[ExprId],
    visit: &mut impl FnMut(ExprId, CanonicalStateOperator),
) -> Result<(), MissingExpression> {
    for root in roots {
        visit_state_sites(expressions, *root, visit)?;
    }
    Ok(())
}

fn visit_analog_operator_children(
    expressions: &[HirExpression],
    op: &HirAnalogOperator,
    visit: &mut impl FnMut(ExprId, CanonicalStateOperator),
) -> Result<(), MissingExpression> {
    match op {
        HirAnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        } => {
            visit_state_sites(expressions, *proposed, visit)?;
            visit_state_sites(expressions, *candidate, visit)?;
            for child in [*type_metadata].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::LimiterArgument { .. } => {}
        HirAnalogOperator::Ddt { expr, abstol } => {
            visit_state_sites(expressions, *expr, visit)?;
            for child in [*abstol].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            visit_state_sites(expressions, *expr, visit)?;
            for child in [*ic, *assert, *abstol].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            visit_state_sites(expressions, *expr, visit)?;
            for child in [*ic, *modulus, *offset, *abstol].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::Ddx { expr, probe } => {
            visit_state_sites(expressions, *expr, visit)?;
            visit_state_sites(expressions, *probe, visit)?;
        }
        HirAnalogOperator::Limexp { expr } | HirAnalogOperator::LastCrossing { expr, .. } => {
            visit_state_sites(expressions, *expr, visit)?;
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            visit_state_sites(expressions, *expr, visit)?;
            visit_state_sites(expressions, *delay, visit)?;
            for child in [*max_delay].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
            ..
        } => {
            visit_state_sites(expressions, *expr, visit)?;
            for child in [*delay, *rise, *fall, *tolerance].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::TransitionDerivative {
            input,
            input_derivative,
            delay,
            rise,
            fall,
            ..
        } => {
            for child in [Some(*input), Some(*input_derivative), *delay, *rise, *fall]
                .into_iter()
                .flatten()
            {
                visit_state_sites(expressions, child, visit)?;
            }
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            visit_state_sites(expressions, *expr, visit)?;
            for child in [*max_rise, *max_fall].into_iter().flatten() {
                visit_state_sites(expressions, child, visit)?;
            }
        }
    }
    Ok(())
}

/// One state-bearing operator and the record it owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalStateSite {
    /// The canonical expression that owns the record. This is the key: the CFG
    /// carries it in the `operator` field of every stateful value kind, and the
    /// MIR carries it as an expression id, so both address one record.
    pub operator: ExprId,
    pub kind: CanonicalStateOperator,
    /// Dense index within [`CanonicalStateOperator::family`], in the module's
    /// executed-root order.
    pub slot: u32,
}

impl CanonicalStateSite {
    pub fn family(&self) -> CanonicalStateFamily {
        self.kind.family()
    }
}

/// Every state record one module owns, keyed by operator site.
///
/// Built with [`Self::from_hir`], which is the only level carrying the roots the
/// numbering walks: `MirModel` holds the contributions but not the module's
/// assignments, so a MIR-only layout would number a module's contributions as
/// though its assignments owned nothing.
///
/// Derived rather than stored. It carries no wire format and no schema version
/// because nothing serializes it: it is recomputed from the HIR, whose own
/// digest is what an artifact's identity already covers. The first consumer to
/// persist a layout — the CFG-sourced backend, when it allocates accepted state
/// from this numbering — introduces a version with the change that makes one
/// mean something, rather than inheriting one that has never been checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalStateLayout {
    sites: Vec<CanonicalStateSite>,
    by_operator: HashMap<ExprId, usize>,
}

impl CanonicalStateLayout {
    /// Number every state-bearing operator the module executes.
    ///
    /// One walk over the executed roots — assignments in order, then
    /// contributions in order, operands before their operator within each — and
    /// a site's slot is its rank among the sites sharing its family. A root
    /// naming an expression outside the arena is skipped rather than refused:
    /// the arena's integrity is [`HirModel::validate`]'s subject, and a layout
    /// that panicked on a malformed module would turn a diagnostic into a crash.
    pub fn from_hir(hir: &HirModel) -> Self {
        let mut roots = Vec::new();
        for statement in &hir.statements {
            collect_statement_roots(statement, &mut roots);
        }
        roots.extend(
            hir.contributions
                .iter()
                .map(|contribution| contribution.expression.id),
        );
        Self::for_roots(&hir.expressions, roots)
    }

    /// Number the state-bearing operators reachable from `roots`, in the order
    /// the roots are given.
    ///
    /// [`Self::from_hir`] is the spelling that knows which roots a module
    /// executes; this one exists for a consumer holding its own root list, and
    /// is what makes the numbering testable without a whole compiled module.
    pub fn for_roots(
        expressions: &[HirExpression],
        roots: impl IntoIterator<Item = ExprId>,
    ) -> Self {
        let mut builder = LayoutBuilder::default();
        for root in roots {
            builder.root(expressions, root);
        }
        builder.finish()
    }

    /// Every site in the module, in executed-root order.
    pub fn sites(&self) -> &[CanonicalStateSite] {
        &self.sites
    }

    /// The record one operator expression owns, if it owns one.
    pub fn site(&self, operator: ExprId) -> Option<&CanonicalStateSite> {
        self.by_operator
            .get(&operator)
            .and_then(|index| self.sites.get(*index))
    }

    /// How many records one family holds.
    pub fn family_len(&self, family: CanonicalStateFamily) -> usize {
        self.sites
            .iter()
            .filter(|site| site.family() == family)
            .count()
    }
}

/// The expressions one statement evaluates, in evaluation order.
fn collect_statement_roots(statement: &HirStatement, roots: &mut Vec<ExprId>) {
    match statement {
        HirStatement::Assignment(assignment) => {
            if let Some(index) = &assignment.index {
                roots.push(index.id);
            }
            roots.push(assignment.expr.id);
        }
        HirStatement::Loop(loop_statement) => {
            roots.push(loop_statement.condition.id);
            for statement in &loop_statement.body {
                collect_statement_roots(statement, roots);
            }
        }
    }
}

#[derive(Debug, Default)]
struct LayoutBuilder {
    sites: Vec<CanonicalStateSite>,
    by_operator: HashMap<ExprId, usize>,
    next: HashMap<CanonicalStateFamily, u32>,
}

impl LayoutBuilder {
    fn root(&mut self, expressions: &[HirExpression], root: ExprId) {
        let mut sites = Vec::new();
        // A malformed arena is HIR validation's diagnostic, not this walk's
        // panic; whatever the traversal reached before the dangling id is still
        // correctly numbered, and the module will be refused before it runs.
        let _ = visit_state_sites(expressions, root, &mut |operator, kind| {
            sites.push((operator, kind));
        });
        for (operator, kind) in sites {
            let counter = self.next.entry(kind.family()).or_default();
            let slot = *counter;
            *counter += 1;
            self.by_operator.insert(operator, self.sites.len());
            self.sites.push(CanonicalStateSite {
                operator,
                kind,
                slot,
            });
        }
    }

    fn finish(self) -> CanonicalStateLayout {
        CanonicalStateLayout {
            sites: self.sites,
            by_operator: self.by_operator,
        }
    }
}

/// Why a CFG could not be given state records from the layout.
///
/// Every variant names the operator it is about. A CFG-sourced backend that met
/// one of these and carried on would allocate a record for an operator that is
/// not the one the runtime resumes, which is the failure this whole module
/// exists to make impossible — so they are refusals, never fallbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgStateAllocationError {
    /// A CFG operator whose body expression has no executed counterpart.
    ///
    /// The correspondence covers assignment values and indices, contribution
    /// values, `if` and loop conditions. It does not cover a `case` arm's
    /// condition, whose two forms are different expressions rather than two
    /// copies of one. A state operator written there reaches here.
    Unmapped {
        operator: ExprId,
        kind: CanonicalStateOperator,
    },
    /// The executed expression the correspondence names owns no record, or owns
    /// one of another kind.
    ///
    /// Unreachable through the compiler's own pipeline — the congruence check
    /// on each run refuses a pairing whose kinds disagree — and here because a
    /// deserialized artifact can carry a correspondence this build did not make.
    Mispaired {
        operator: ExprId,
        executed: ExprId,
        kind: CanonicalStateOperator,
        found: Option<CanonicalStateOperator>,
    },
    /// A `transition` filter, which this allocation cannot name.
    ///
    /// `transition` is the one operator the CFG names by its own
    /// [`TransitionSiteId`] rather than by the expression that owns it, and the
    /// two lowerings mint different ordinals for one source site because the
    /// preorder counter runs across both copies. The correspondence is a map
    /// over expressions and does not carry that pairing.
    ///
    /// It costs nothing today and is a refusal rather than a gap on purpose.
    /// `transition` reaches the canonical IR as `HirExprKind::Call`, never as
    /// the typed `HirAnalogOperator::Transition` — the parser produces no such
    /// node and the semantic analyzer's arms for one are unreachable through
    /// the compiler's own pipeline — so [`super::cfg_lower`] refuses it by name
    /// and no CFG carries this kind. When the CFG level gains `transition`, the
    /// pairing has to be added deliberately, and this refusal is what makes
    /// that a compile error in a model rather than a wrong slot.
    UnsupportedTransition { site: TransitionSiteId },
    /// Two CFG operators resolved onto one executed record.
    ///
    /// Two distinct operators sharing a record would integrate one's history
    /// into the other's. Refused rather than reported per operator, because the
    /// pair is the finding.
    Aliased {
        first: ExprId,
        second: ExprId,
        executed: ExprId,
    },
}

impl std::fmt::Display for CfgStateAllocationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unmapped { operator, kind } => write!(
                formatter,
                "canonical {} at expression {operator} has no executed counterpart, so its state record cannot be named",
                kind.name()
            ),
            Self::Mispaired {
                operator,
                executed,
                kind,
                found,
            } => write!(
                formatter,
                "canonical {} at expression {operator} pairs with executed expression {executed}, which owns {}",
                kind.name(),
                match found {
                    Some(found) => format!("a {} record", found.name()),
                    None => "no state record".to_string(),
                }
            ),
            Self::UnsupportedTransition { site } => write!(
                formatter,
                "transition site {}:{}..{}#{} is named by site identity rather than by expression, and the two lowerings mint different ordinals for one source site, so its filter record cannot be named from the executed correspondence",
                site.source, site.start, site.end, site.ordinal
            ),
            Self::Aliased {
                first,
                second,
                executed,
            } => write!(
                formatter,
                "canonical expressions {first} and {second} both resolve to executed expression {executed}, which owns one record"
            ),
        }
    }
}

/// Runtime state records addressed by the names a CFG carries.
///
/// [`CanonicalStateLayout`] numbers the *executed* copy of a module, because
/// that is the copy whose records the runtime allocates and the checkpoint
/// serializes. A CFG carries *body*-copy names. This is the composition of the
/// two: resolve each CFG name through
/// [`super::hir::HirExecutedCorrespondence`], then read the layout.
///
/// ## Why this is not simply "the layout"
///
/// Three numberings exist in the tree and all three are correct for their
/// runtime:
///
/// 1. The **bytecode generator** allocates a fresh slot per *emission*, module
///    global, in executed-root order. A module with noise in an assignment is
///    emitted twice, so one canonical `ddt` owns two slots.
/// 2. [`CanonicalStateLayout`] allocates per *site*, per family, in the same
///    order. This is the VM and JIT runtime's shape: `ddt`, `idt`, `idtmod` and
///    `$limit` all draw from one `state_values_prev` array.
/// 3. The **generated-Rust backend** allocates per site in *CFG value* order,
///    with a separate counter per operator, because
///    `GeneratedVerilogAPersistentState` gives `ddt`, `idt` and the limiter
///    anchor their own arrays.
///
/// (2) and (3) are different shapes, not different orders of one shape, so this
/// type serves (2) — the runtime the JIT feeds — and the generated bundle keeps
/// (3). Making one layout serve both would change the generated runtime's
/// struct and renumber a shipped checkpoint, which is a decision about the
/// generated device contract rather than about this map.
///
/// ## Checkpoint compatibility
///
/// Compatible with (1) by construction wherever the two spaces coincide, and
/// *not* compatible where they do not — a module with noise in an assignment
/// carrying an integration operator. Nothing here silently reconciles them:
/// [`Self::agrees_with_emission_allocation`] answers whether a given module is
/// one of the coinciding ones, so a caller decides rather than assumes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgStateAllocation {
    layout: CanonicalStateLayout,
    /// Body-copy operator expression to the executed site it names.
    by_operator: HashMap<ExprId, CanonicalStateSite>,
}

impl CfgStateAllocation {
    /// Give every state-bearing value in `function` the record the layout
    /// numbers for it, or refuse by name.
    ///
    /// Every refusal is collected rather than the first returned: a model that
    /// cannot be allocated should say everything that is wrong with it in one
    /// run, the same contract [`super::cfg_lower::CfgModel::from_hir`] keeps.
    pub fn build(
        hir: &HirModel,
        function: &CfgFunction,
    ) -> Result<Self, Vec<CfgStateAllocationError>> {
        let layout = CanonicalStateLayout::from_hir(hir);
        let correspondence = &hir.executed_correspondence;

        let mut by_operator: HashMap<ExprId, CanonicalStateSite> = HashMap::new();
        let mut executed_owners: HashMap<ExprId, ExprId> = HashMap::new();
        let mut refused_transitions: HashSet<TransitionSiteId> = HashSet::new();
        let mut errors = Vec::new();

        for value in &function.values {
            match value.kind.state_site() {
                Some(CfgStateSite::Operator(operator, kind)) => {
                    if by_operator.contains_key(&operator) {
                        continue;
                    }
                    let Some(executed) = correspondence.executed(operator) else {
                        errors.push(CfgStateAllocationError::Unmapped { operator, kind });
                        continue;
                    };
                    let Some(site) = layout.site(executed) else {
                        errors.push(CfgStateAllocationError::Mispaired {
                            operator,
                            executed,
                            kind,
                            found: None,
                        });
                        continue;
                    };
                    // `cross` and `above` are one family but two operators, and
                    // the layout records the spelling it saw; comparing families
                    // rather than operators is what keeps a `cross` reading a
                    // detector from being called a mispairing.
                    if site.kind.family() != kind.family() {
                        errors.push(CfgStateAllocationError::Mispaired {
                            operator,
                            executed,
                            kind,
                            found: Some(site.kind),
                        });
                        continue;
                    }
                    if let Some(first) = executed_owners.insert(executed, operator) {
                        errors.push(CfgStateAllocationError::Aliased {
                            first,
                            second: operator,
                            executed,
                        });
                        continue;
                    }
                    by_operator.insert(operator, *site);
                }
                Some(CfgStateSite::Transition(site)) => {
                    if refused_transitions.insert(site) {
                        errors.push(CfgStateAllocationError::UnsupportedTransition { site });
                    }
                }
                None => {}
            }
        }

        if errors.is_empty() {
            Ok(Self {
                layout,
                by_operator,
            })
        } else {
            Err(errors)
        }
    }

    /// The record one CFG operator expression owns.
    pub fn site(&self, operator: ExprId) -> Option<&CanonicalStateSite> {
        self.by_operator.get(&operator)
    }

    /// The dense slot one CFG operator expression owns within its family.
    pub fn slot(&self, operator: ExprId) -> Option<u32> {
        self.by_operator.get(&operator).map(|site| site.slot)
    }

    /// How many records the module's runtime has to reserve for one family.
    ///
    /// From the layout, not from what the CFG happened to reference: a family's
    /// array is sized by the module, and an operator whose value was folded away
    /// still owns its slot in the checkpoint.
    pub fn family_len(&self, family: CanonicalStateFamily) -> usize {
        self.layout.family_len(family)
    }

    /// The layout this allocation reads.
    pub fn layout(&self) -> &CanonicalStateLayout {
        &self.layout
    }

    /// Whether this module's per-site numbering is also the bytecode
    /// generator's per-emission numbering, so a checkpoint written by one
    /// runtime means the same thing to the other.
    ///
    /// A *sufficient* condition, deliberately, and stated as one: true when no
    /// assignment in the module owns any state record at all. The replay that
    /// separates the two spaces is `DeviceIR::noise_assignments`, a clone of
    /// `assignments` carrying noise shadows, and the generator allocates a fresh
    /// slot at each emission — so a module whose assignments own nothing has
    /// nothing to double-allocate, whatever its contributions do. A module that
    /// answers `false` may still coincide; it is not asserted to differ, only
    /// not proven to agree, which is the reading a caller resuming a foreign
    /// checkpoint needs. A caller that allocates its own state does not need to
    /// ask at all.
    pub fn agrees_with_emission_allocation(&self, hir: &HirModel) -> bool {
        let mut roots = Vec::new();
        for statement in &hir.statements {
            collect_statement_roots(statement, &mut roots);
        }
        let mut owns_state = false;
        for root in roots {
            let _ = visit_state_sites(&hir.expressions, root, &mut |_, _| owns_state = true);
        }
        !owns_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::diagnostic::SourceSpanRef;
    use crate::canonical_ir::hir::HirExpression;

    fn expression(id: u32, kind: HirExprKind) -> HirExpression {
        HirExpression {
            id: ExprId::from(id as usize),
            kind,
            span: SourceSpanRef {
                source_file_id: 0,
                start: 0,
                end: 0,
            },
        }
    }

    fn call(name: &str, args: usize) -> HirExprKind {
        HirExprKind::Call {
            name: name.into(),
            args: vec![ExprId::from(0usize); args],
        }
    }

    #[test]
    fn operator_indices_address_their_own_position() {
        for (position, operator) in CanonicalStateOperator::ALL.into_iter().enumerate() {
            assert_eq!(
                operator.index(),
                position,
                "{} indexes a table entry belonging to another operator",
                operator.name()
            );
        }
    }

    #[test]
    fn timer_is_the_only_family_without_a_per_slot_record() {
        for operator in CanonicalStateOperator::ALL {
            assert_eq!(
                operator.family().has_per_slot_record(),
                operator != CanonicalStateOperator::Timer,
                "{} disagrees with the runtime about owning per-slot storage",
                operator.name()
            );
        }
    }

    #[test]
    fn classification_partitions_the_state_bearing_calls() {
        // `slew` with one argument is a passthrough and owns nothing; with a
        // rate it owns a filter.
        assert_eq!(classify(&call("slew", 1)), None);
        assert_eq!(
            classify(&call("slew", 2)),
            Some(CanonicalStateOperator::Slew)
        );
        // `idtmod` splits on arity between the unwrapped integral's record and
        // its own.
        assert_eq!(
            classify(&call("idtmod", 2)),
            Some(CanonicalStateOperator::Idt)
        );
        assert_eq!(
            classify(&call("idtmod", 4)),
            Some(CanonicalStateOperator::IdtMod)
        );
        // Two spellings, one detector.
        assert_eq!(
            classify(&call("cross", 2)),
            Some(CanonicalStateOperator::Cross)
        );
        assert_eq!(
            classify(&call("last_crossing", 2)),
            Some(CanonicalStateOperator::Cross)
        );
        assert_eq!(
            classify(&call("$last_crossing", 2)),
            Some(CanonicalStateOperator::Cross)
        );
        assert_eq!(classify(&call("sin", 1)), None);
    }

    /// `leaf`, then `ddt(leaf)`, `cross(leaf, leaf)`, `idt(leaf)`,
    /// `last_crossing(leaf)`, `sin(leaf)` and a second `ddt` — a flat arena of
    /// operators over one shared operand, so the walk's order is the arena's.
    fn arena() -> Vec<HirExpression> {
        let leaf = ExprId::from(0usize);
        let unary = |name: &str| HirExprKind::Call {
            name: name.into(),
            args: vec![leaf],
        };
        vec![
            expression(
                0,
                HirExprKind::Number {
                    value: 1.0,
                    raw: "1.0".into(),
                },
            ),
            expression(1, unary("ddt")),
            expression(
                2,
                HirExprKind::Call {
                    name: "cross".into(),
                    args: vec![leaf, leaf],
                },
            ),
            expression(3, unary("idt")),
            expression(4, unary("last_crossing")),
            expression(5, unary("sin")),
            expression(6, unary("ddt")),
        ]
    }

    #[test]
    fn slots_are_dense_per_family_in_root_order() {
        let expressions = arena();
        let roots = [1u32, 2, 3, 4, 5, 6].map(|id| ExprId::from(id as usize));
        let layout = CanonicalStateLayout::for_roots(&expressions, roots);

        // Integration and the crossing detectors number independently, each
        // dense from zero, in the order the roots were walked.
        let slot = |id: u32| layout.site(ExprId::from(id as usize)).map(|site| site.slot);
        assert_eq!(slot(1), Some(0));
        assert_eq!(slot(3), Some(1));
        assert_eq!(slot(6), Some(2));
        assert_eq!(slot(2), Some(0));
        assert_eq!(slot(4), Some(1));
        assert_eq!(slot(5), None);

        assert_eq!(layout.family_len(CanonicalStateFamily::Integration), 3);
        assert_eq!(layout.family_len(CanonicalStateFamily::CrossDetector), 2);
        assert_eq!(layout.family_len(CanonicalStateFamily::ZiFilter), 0);
    }

    /// A root reached twice numbers its operators twice, because two roots that
    /// share a subtree are two evaluations of it. Nothing in the executed HIR
    /// shares subtrees between roots — the arena is a forest — so this is a
    /// statement about the constructor's contract rather than about a module.
    #[test]
    fn a_dangling_root_does_not_panic() {
        let expressions = arena();
        let layout = CanonicalStateLayout::for_roots(
            &expressions,
            [ExprId::from(1usize), ExprId::from(99usize)],
        );
        assert_eq!(layout.family_len(CanonicalStateFamily::Integration), 1);
    }

    /// Each family names the accepted-state field it occupies, and the two
    /// answers a slot allocator reads before reserving anything.
    ///
    /// Written out against `crate::vm::VmAcceptedCheckpoint`'s own field names
    /// rather than described in prose: a family added without a storage decision
    /// would compile, and this is what refuses it. The lookup tables are the one
    /// entry with no checkpoint field, because compiled table data is read-only
    /// and no accepted state at all.
    #[test]
    fn every_family_names_the_runtime_record_it_occupies() {
        let expected = [
            (
                CanonicalStateFamily::Integration,
                Some("state_values_prev"),
                true,
                true,
            ),
            (
                CanonicalStateFamily::DelayBuffer,
                Some("delay_buffers"),
                true,
                false,
            ),
            (
                CanonicalStateFamily::TransitionFilter,
                Some("transition_filters"),
                true,
                false,
            ),
            (
                CanonicalStateFamily::SlewFilter,
                Some("slew_filters"),
                true,
                true,
            ),
            (
                CanonicalStateFamily::CrossDetector,
                Some("cross_detectors"),
                true,
                true,
            ),
            (
                CanonicalStateFamily::LaplaceFilter,
                Some("laplace_filters"),
                true,
                true,
            ),
            (
                CanonicalStateFamily::ZiFilter,
                Some("zi_filters"),
                true,
                true,
            ),
            (
                CanonicalStateFamily::TimerEvent,
                Some("timer_event_bound"),
                false,
                true,
            ),
            (CanonicalStateFamily::LookupTable, None, true, true),
        ];
        for (family, field, per_slot, fixed) in expected {
            assert_eq!(family.checkpoint_field(), field, "{family:?}");
            assert_eq!(family.has_per_slot_record(), per_slot, "{family:?}");
            assert_eq!(family.has_fixed_record(), fixed, "{family:?}");
        }
        // Every operator's family is one of the nine above, so a new family
        // cannot slip in without an entry here.
        for operator in CanonicalStateOperator::ALL {
            assert!(
                expected
                    .iter()
                    .any(|(family, ..)| *family == operator.family()),
                "{}'s family is not described",
                operator.name()
            );
        }
    }
}
