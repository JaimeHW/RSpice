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
//! appears in the arena twice, under two different `ExprId`s.
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
//! integration operator, and the generator compiles the same source operator
//! more than once in two different ways. A statement is compiled twice when a
//! module's noise replay genuinely differs from its ordinary pass — once as
//! `assignment_steps` and again as `noise_assignment_steps`, the latter from
//! `DeviceIR::noise_assignments` carrying noise shadows — and a contribution's
//! operator is compiled again inside each Jacobian entry that the product rule
//! leaves it in. One canonical `ddt` site therefore owns two or more bytecode
//! slots. (A module whose two passes are the same — every shipped compact
//! model — leaves `noise_assignment_steps` empty and emits its statements
//! once; the numberings then differ only through the contributions.)
//! [`CfgStateAllocation`] carries the measurement over the shipped corpus.
//!
//! So a CFG-sourced backend cannot adopt the bytecode numbering; it allocates
//! from this layout, and the point at which shipped code does that is the point
//! at which the accepted-state arrays are re-indexed and the runtime checkpoint
//! state version has to move. Nothing here changes either yet:
//! [`crate::native::expr`] still reads its slot *numbers* from the program it
//! is lowering, and takes only the identity and the order from this module.

use std::collections::HashMap;

use super::cfg::{CfgFunction, CfgStateSite};
use super::hir::{HirAnalogOperator, HirExprKind, HirExpression, HirModel, HirStatement};
use super::ids::ExprId;
use super::noise::contains_noise;

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

    /// Whether the bytecode generator takes a fresh slot at every *emission* of
    /// an operator in this family, rather than one per source site.
    ///
    /// The distinction is in `codegen::generator` and it is mechanical: three
    /// families are allocated by a bare monotonic counter — `limit_state_count`
    /// for [`Self::Integration`], `cross_detector_count` for
    /// [`Self::CrossDetector`], `timer_state_count` for [`Self::TimerEvent`] —
    /// so compiling one operator twice reserves two records. The rest go
    /// through a site map (`absdelay_sites`, `transition_sites`, `slew_sites`,
    /// `laplace_sites`, `zi_sites`) or are deduplicated by content
    /// (`register_lookup_table`), so a second compilation of the same operator
    /// returns the slot the first one took.
    ///
    /// This is what decides whether a module's per-site numbering can differ
    /// from the generator's at all: see
    /// [`CfgStateAllocation::agrees_with_emission_allocation`].
    pub fn allocates_per_emission(self) -> bool {
        matches!(
            self,
            Self::Integration | Self::CrossDetector | Self::TimerEvent
        )
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
        let mut roots = statement_roots(hir);
        roots.extend(
            hir.contributions
                .iter()
                .map(|contribution| contribution.expression.id),
        );
        Self::for_roots(&hir.expressions, roots)
    }

    /// Number only the sites the module's *statements* own — the prefix of
    /// [`Self::from_hir`]'s walk, before it reaches the contributions.
    ///
    /// The split matters to anyone comparing this numbering against the
    /// bytecode generator's: the generator compiles `ir.assignments` (the
    /// statements) and `ir.equations` (the contributions) in two separate
    /// passes with a replay in between, so where a module's sites fall across
    /// that boundary is what decides whether the two numberings share a prefix.
    /// Exposed rather than reconstructed by the caller so that the boundary is
    /// read off the one walk instead of a second copy of it.
    pub fn statement_prefix(hir: &HirModel) -> Self {
        Self::for_roots(&hir.expressions, statement_roots(hir))
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

/// Every expression the module's statements evaluate, in evaluation order.
fn statement_roots(hir: &HirModel) -> Vec<ExprId> {
    let mut roots = Vec::new();
    for statement in &hir.statements {
        collect_statement_roots(statement, &mut roots);
    }
    roots
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
/// *not* compatible where they do not. Nothing here silently reconciles them:
/// [`Self::agrees_with_emission_allocation`] answers whether a given module is
/// one of the coinciding ones, so a caller decides rather than assumes.
///
/// ## Which numbering the JIT runtime owns (W-D's ruling; W-E's measurement)
///
/// **Per site — this type's numbering (2) — is what the JIT runtime takes when
/// the CFG route becomes the default.** Three reasons, in the order they
/// decide it:
///
/// 1. A state record belongs to an *operator*, not to a copy of the statement
///    it appears in. The conformance checkpoint pins are written in that
///    identity: one `ddt` in the source is one entry in a saved transient
///    state, and a reader matching them up by position gets the same answer
///    whether or not the module also declares noise. Numbering (1) breaks that
///    correspondence for exactly the modules where it is hardest to notice.
/// 2. The CFG route has no per-emission numbering to adopt. Emission order is a
///    property of the bytecode generator's passes over `assignments`,
///    `noise_assignments`, and each equation's derivative programs; a CFG has
///    one body and one traversal, so reproducing (1) would mean re-deriving the
///    generator's replay in a level that does not have it.
/// 3. The disagreement is a double *allocation*. The generator takes a fresh
///    slot at each emission, so a module in the affected set reserves slots no
///    evaluation ever addresses.
///
/// ### What that costs, measured
///
/// `the_two_state_slot_numberings_are_censused_over_the_shipped_corpus`
/// (`native::cfg_census`) is the census. W-E rebuilt it to read the generator's
/// slot counter back off the compiled model *context by context* rather than to
/// take the largest slot addressed, which is what makes the table below able to
/// say where each extra slot came from and whether it displaces a site's own.
///
/// Twelve of the forty-three shipped modules differ, not seven, and only **two**
/// of them append:
///
/// | module | per site | per emission | assign | noise-assign | eq. primal | eq. derivative | shape |
/// | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
/// | `bjt505_va` | 10 | 20 | 0 | 0 | 10 | 10 | append |
/// | `bjtd505_va` | 9 | 18 | 0 | 0 | 9 | 9 | append |
/// | `angelov_gan` | 17 | 18 | 1 | 1 | 16 | 0 | interleave |
/// | `bjt505t_va` | 11 | 23 | 1 | 1 | 10 | 11 | interleave |
/// | `bjtd505t_va` | 10 | 21 | 1 | 1 | 9 | 10 | interleave |
/// | `asmesd` | 13 | 15 | 0 | 0 | 13 | 2 | interleave |
/// | `asmesd_dio` | 6 | 8 | 0 | 0 | 6 | 2 | interleave |
/// | `asmhemt` | 121 | 141 | 0 | 0 | 121 | 20 | interleave |
/// | `hicumL0va` | 9 | 10 | 1 | 1 | 8 | 0 | interleave |
/// | `hicumL2va` | 20 | 38 | 0 | 0 | 20 | 18 | interleave |
/// | `mvsg_cmc` | 146 | 148 | 0 | 0 | 146 | 2 | interleave |
/// | `ekv_va` | 5 | 7 | 2 | 2 | 3 | 0 | interleave |
///
/// The other thirty-one modules emit exactly their site count in exactly the
/// site order. No module emits a state slot from a parameter program, and — the
/// correction that matters most — **no module emits one from a noise PSD
/// program either**: that column is zero on all forty-three.
///
/// ### Two corrections to W-D's note
///
/// 1. **The PSD programs are not a re-emitting context.** W-D's note said a
///    module whose assignments own nothing could still emit a contribution's
///    operator again "while compiling the PSD programs". Measured, that never
///    happens. The context W-D's note omitted is the one doing all the work: an
///    equation's **derivative programs**. `codegen::autodiff`'s product rule is
///    `d(l·r) = dl·r + l·dr`, which keeps `l` and `r`, so a contribution
///    spelled `I(a,b) <+ f(V) * ddt(q)` re-emits its primal `ddt` into the
///    Jacobian entry. Nine of the twelve rows above are that.
/// 2. **"The extra slots are appended, not interleaved" is false for ten of the
///    twelve.** Only `bjt505_va` and `bjtd505_va` append. The reason is
///    structural: `compile_equation` compiles equation *i*'s derivatives before
///    equation *i+1*'s value program, so one re-emission anywhere but in the
///    last state-owning equation displaces every contribution slot after it.
///    W-D's note reasoned that the extra slots must come after everything
///    because the PSD programs are compiled last; with the derivative programs
///    in the picture that argument does not hold. `asmesd`, `asmesd_dio` and
///    `asmhemt` — three of the modules W-D's predicate called `true` and
///    therefore "appending" — all interleave.
///
/// ### What W-F's numbering move is, exactly
///
/// A **renumbering**, not a shrink, and therefore a shipped-behaviour change:
///
/// * for thirty-one modules the two numberings are already identical and
///   nothing moves;
/// * for `bjt505_va` and `bjtd505_va` slots `0..per_site` keep their meaning
///   and the array merely shortens;
/// * for the remaining ten the arrays *permute*, so a checkpoint written by the
///   bytecode runtime cannot be read by a per-site runtime without a mapping.
///
/// So W-F moves `RUNTIME_CHECKPOINT_STATE_VERSION`, with those ten modules
/// named as the reason in place. The generated bundle keeps numbering (3),
/// which is neither of these two spaces, so
/// `contracts_bug325_son::GENERATED_CHECKPOINT_IDENTITY` does not move.
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
        let mut errors = Vec::new();

        for value in &function.values {
            let Some(CfgStateSite(operator, kind)) = value.kind.state_site() else {
                continue;
            };
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
            // `cross` and `above` are one family but two operators, and the
            // layout records the spelling it saw; comparing families rather
            // than operators is what keeps a `cross` reading a detector from
            // being called a mispairing.
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

    /// Whether this module's per-site numbering is also the bytecode
    /// generator's per-emission numbering, so a checkpoint written by one
    /// runtime means the same thing to the other.
    ///
    /// Delegates to [`EmissionCensus`], which counts the state sites of every
    /// context `generate_from_ir` compiles and reports which of them re-emit.
    /// See that type for what each context is and why it counts.
    ///
    /// Still a *sufficient* condition, and still stated as one: a module that
    /// answers `false` may coincide anyway — it is not asserted to differ, only
    /// not proven to agree, which is the reading a caller resuming a foreign
    /// checkpoint needs. What changed in W-E is that a `true` is now sound.
    /// The previous spelling inspected `hir.statements` alone and answered
    /// `true` for five of the seven shipped modules whose two numberings
    /// measurably differ.
    pub fn agrees_with_emission_allocation(&self, hir: &HirModel) -> bool {
        EmissionCensus::of(hir).agrees()
    }
}

/// The state sites of every context the bytecode generator compiles, counted
/// where the generator's numbering can be displaced.
///
/// `codegen::generator::generate_from_ir` compiles, in this order:
///
/// 1. every parameter's `default_expr`, `min_expr`, `max_expr` and each
///    `exclude_exprs` entry;
/// 2. `ir.assignments` — the module's statements;
/// 3. `ir.noise_assignments` — `ir.assignments` carrying noise shadows, built
///    only when the module has noise sources *and* some variable is
///    noise-shadowed; when the replay would mirror the ordinary pass the
///    generator emits nothing here and leaves `noise_assignment_steps` empty;
/// 4. every equation: its value program, its peeled static condition, each
///    resistive `derivatives[i].expr` and each `reactive_derivatives[i].expr`;
/// 5. every noise source: its `psd`, its `exponent`, and each injection's
///    `gain`.
///
/// `allocate_slot` is a bare counter for three families
/// ([`CanonicalStateFamily::allocates_per_emission`]), so an operator compiled
/// in two of those contexts reserves two records; the other families go through
/// a site map and reserve one however often they are compiled.
///
/// ## Why the contribution count is fatal on its own
///
/// W-D's ruling recorded the noise-assignment clone (3) and the PSD programs
/// (5) as the re-emitting contexts. **The derivative programs in (4) are the
/// third, and they are the one that makes a contribution-borne site unsafe by
/// itself.** `codegen::autodiff`'s product rule is
/// `d(l·r) = dl·r + l·dr`, which *keeps* `l` and `r`; so a contribution
/// spelled `I(a,b) <+ f(V) * ddt(q)` differentiates to
/// `df/dV · ddt(q) + f · ddt_companion(dq/dV)`, and the primal `ddt` is
/// emitted again — once per Jacobian axis whose term the simplifier does not
/// fold away. Which terms survive is `autodiff::simplify`'s answer, not a
/// property of the HIR, so this level cannot bound it and does not pretend to:
/// any counter-family site in a contribution answers `false`.
///
/// A contribution-borne site that is only ever *accumulated* — reached from the
/// contribution root through `+`, `-` and unary sign alone — does vanish from
/// its own derivative, because `d(ddt(q))` is a companion and carries no primal
/// copy. Refusing those too is deliberate precision loss rather than an
/// oversight: which terms survive is `autodiff::simplify`'s answer, and the
/// noise PSD and injection-gain programs compile sub-expressions of the same
/// trees under rules of their own, so a predicate that had to be right about
/// all of them would be a second copy of the generator.
///
/// The loss is measured rather than assumed. Over the shipped corpus this
/// answers `false` for thirty of the thirty-one modules whose two numberings do
/// coincide, and `true` only for `r2_cmc`, which owns no state at all. The
/// census `the_two_state_slot_numberings_are_censused_over_the_shipped_corpus`
/// (`native::cfg_census`) reports that figure and asserts the soundness
/// direction — `true` implies the counts coincide — on every module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmissionCensus {
    /// Sites reachable from a parameter's default, bound or exclude
    /// expression, in any family. Compiled first, so one of these displaces
    /// every slot the module allocates afterwards — including the site-mapped
    /// families, whose slot is fixed by first emission.
    pub parameters: usize,
    /// Counter-family sites reachable from the module's statements. Emitted
    /// twice when [`Self::has_noise`] *and* the module's noise replay is not a
    /// mirror of its ordinary pass, once otherwise. The mirror case is the one
    /// this level cannot see — whether a variable is noise-shadowed is
    /// `ir::autodiff`'s answer, not the HIR's — so `has_noise` alone is used,
    /// and a mirrored module is counted as if it re-emitted. That is precision
    /// lost in the safe direction: [`Self::agrees`] then answers `false` for a
    /// module whose numberings do coincide, never `true` for one whose do not.
    pub statements: usize,
    /// Counter-family sites reachable from the module's contributions.
    pub contributions: usize,
    /// Whether the module has a noise expression at all: the generator
    /// considers a second statement pass only then. It does not say the second
    /// pass was *emitted* — a module with noise whose replay mirrors the
    /// ordinary pass emits none — so this over-counts rather than under-counts.
    pub has_noise: bool,
}

impl EmissionCensus {
    /// Count the contexts of one module.
    pub fn of(hir: &HirModel) -> Self {
        let mut parameters = 0usize;
        for parameter in &hir.parameters {
            let mut roots: Vec<ExprId> =
                parameter.default_expr.iter().map(|expr| expr.id).collect();
            if let Some(range) = &parameter.range {
                roots.extend(range.min_expression.iter().map(|expr| expr.id));
                roots.extend(range.max_expression.iter().map(|expr| expr.id));
                roots.extend(range.exclude_expressions.iter().map(|expr| expr.id));
            }
            for root in roots {
                parameters += count_sites(hir, root, |_| true);
            }
        }

        let statement_roots = statement_roots(hir);
        let statements = statement_roots
            .iter()
            .map(|root| count_sites(hir, *root, |kind| kind.family().allocates_per_emission()))
            .sum();
        let contributions = hir
            .contributions
            .iter()
            .map(|contribution| {
                count_sites(hir, contribution.expression.id, |kind| {
                    kind.family().allocates_per_emission()
                })
            })
            .sum();

        let has_noise = hir
            .contributions
            .iter()
            .map(|contribution| contribution.expression.id)
            .chain(statement_roots.iter().copied())
            .any(|root| contains_noise(hir, root));

        Self {
            parameters,
            statements,
            contributions,
            has_noise,
        }
    }

    /// Whether nothing in the module can be emitted into a second record.
    pub fn agrees(&self) -> bool {
        self.parameters == 0 && self.contributions == 0 && (!self.has_noise || self.statements == 0)
    }
}

/// How many state sites `root` reaches whose operator `wanted` accepts.
///
/// Counted per *occurrence* of an operator expression, which is what
/// `allocate_slot` does. A malformed arena is HIR validation's diagnostic, not
/// this walk's panic; whatever the traversal reached before a dangling id is
/// still counted.
fn count_sites(
    hir: &HirModel,
    root: ExprId,
    wanted: impl Fn(CanonicalStateOperator) -> bool,
) -> usize {
    let mut count = 0usize;
    let _ = visit_state_sites(&hir.expressions, root, &mut |_, kind| {
        if wanted(kind) {
            count += 1;
        }
    });
    count
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
