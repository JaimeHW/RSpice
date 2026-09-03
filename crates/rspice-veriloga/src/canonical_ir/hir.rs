//! HIR: the first canonical level, still shaped like the source module.
//!
//! [`HirModel::from_analyzed_module`] takes the semantic analyzer's output and
//! assigns stable ids to every port, node, branch, parameter, variable, array,
//! and contribution.
//!
//! The module body arrives in two forms. [`HirModel::body`] is the analog block
//! as written: `if`/`case`/loops are nested [`HirRegion`]s, so a construct can
//! still be reported — and compiled — against the shape the author wrote.
//! [`HirModel::statements`] is the same block with conditionals already folded
//! into `guard ? value : previous`; it is what the backends being replaced read,
//! and it goes away with the last of them. See
//! `design/VERILOGA_BACKEND_PLAN.md`.

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use crate::ast::{
    AnalogOperator, ArrayLiteralElement, BinaryOp, BranchAccess, CrossDirection, Expression,
    LaplaceKind, LimiterArgument, NoiseSource, PortDirection, UnaryOp, ZiKind,
};
use crate::ir::TransitionSiteId;
use crate::numeric_literal::parse_integer_literal;
use crate::semantic::{
    AnalogSiteGuard, AnalogSiteId, AnalyzedModule, AnalyzedRegion, AnalyzedStatement,
    ParameterScope,
};
use crate::types::{ParameterRange, ValueType};

use super::{
    ArrayId, BranchId, CANONICAL_IR_SCHEMA_VERSION, CanonicalMetadata, CompilerPhase,
    ContributionId, ExprId, IrDiagnostic, IrValidationResult, ModuleId, NodeId, ParamId, PortId,
    SourceSpanRef, VariableId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalValueType {
    Real,
    Integer,
    String,
    Boolean,
    NatureAccess,
    Void,
    Unknown,
    Error,
}

impl From<ValueType> for CanonicalValueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Real => Self::Real,
            ValueType::Integer => Self::Integer,
            ValueType::String => Self::String,
            ValueType::Boolean => Self::Boolean,
            ValueType::NatureAccess => Self::NatureAccess,
            ValueType::Void => Self::Void,
            ValueType::Unknown => Self::Unknown,
            ValueType::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParamRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
    #[serde(default)]
    pub min_parameter: Option<SmolStr>,
    #[serde(default)]
    pub max_parameter: Option<SmolStr>,
    #[serde(default)]
    pub min_expression: Option<HirExprRef>,
    #[serde(default)]
    pub max_expression: Option<HirExprRef>,
    pub min_exclusive: bool,
    pub max_exclusive: bool,
    pub exclude: Vec<f64>,
    #[serde(default)]
    pub exclude_parameters: Vec<SmolStr>,
    #[serde(default)]
    pub exclude_expressions: Vec<HirExprRef>,
}

impl HirParamRange {
    pub fn from_range(range: &ParameterRange) -> Self {
        Self {
            min: range.min,
            max: range.max,
            min_parameter: range.min_parameter.clone(),
            max_parameter: range.max_parameter.clone(),
            min_expression: None,
            max_expression: None,
            min_exclusive: range.min_exclusive,
            max_exclusive: range.max_exclusive,
            exclude: range.exclude.clone(),
            exclude_parameters: range.exclude_parameters.clone(),
            exclude_expressions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirExprRef {
    pub id: ExprId,
    pub kind: SmolStr,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirExpression {
    pub id: ExprId,
    pub kind: HirExprKind,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirLaplaceKind {
    ZeroPole {
        zeros: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    ZeroDenominator {
        zeros: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
    NumeratorPole {
        numerator: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    NumeratorDenominator {
        numerator: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirZiKind {
    ZeroPole {
        zeros: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    ZeroDenominator {
        zeros: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
    NumeratorPole {
        numerator: Vec<ExprId>,
        poles: Vec<ExprId>,
    },
    NumeratorDenominator {
        numerator: Vec<ExprId>,
        denominator: Vec<ExprId>,
    },
}

impl HirLaplaceKind {
    /// The numerator-then-denominator operand lists, whichever of the four
    /// spellings wrote them.
    ///
    /// Every consumer of a filter kind wants the same two lists and differs
    /// only in what it does with them, so the four-way match lives here rather
    /// than once per consumer. Which of the two is a root list is
    /// [`Self::names_roots`]'s question.
    pub fn polynomials(&self) -> (&[ExprId], &[ExprId]) {
        match self {
            Self::ZeroPole { zeros, poles } => (zeros, poles),
            Self::ZeroDenominator { zeros, denominator } => (zeros, denominator),
            Self::NumeratorPole { numerator, poles } => (numerator, poles),
            Self::NumeratorDenominator {
                numerator,
                denominator,
            } => (numerator, denominator),
        }
    }

    /// Whether the numerator and the denominator are named by their roots.
    pub fn names_roots(&self) -> (bool, bool) {
        match self {
            Self::ZeroPole { .. } => (true, true),
            Self::ZeroDenominator { .. } => (true, false),
            Self::NumeratorPole { .. } => (false, true),
            Self::NumeratorDenominator { .. } => (false, false),
        }
    }
}

impl HirZiKind {
    /// The sampled-filter counterpart of [`HirLaplaceKind::polynomials`].
    pub fn polynomials(&self) -> (&[ExprId], &[ExprId]) {
        match self {
            Self::ZeroPole { zeros, poles } => (zeros, poles),
            Self::ZeroDenominator { zeros, denominator } => (zeros, denominator),
            Self::NumeratorPole { numerator, poles } => (numerator, poles),
            Self::NumeratorDenominator {
                numerator,
                denominator,
            } => (numerator, denominator),
        }
    }

    /// The sampled-filter counterpart of [`HirLaplaceKind::names_roots`].
    pub fn names_roots(&self) -> (bool, bool) {
        match self {
            Self::ZeroPole { .. } => (true, true),
            Self::ZeroDenominator { .. } => (true, false),
            Self::NumeratorPole { .. } => (false, true),
            Self::NumeratorDenominator { .. } => (false, false),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirCrossDirection {
    Rising,
    Falling,
    Both,
}

impl From<CrossDirection> for HirCrossDirection {
    fn from(direction: CrossDirection) -> Self {
        match direction {
            CrossDirection::Rising => Self::Rising,
            CrossDirection::Falling => Self::Falling,
            CrossDirection::Both => Self::Both,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirAnalogOperator {
    Limit {
        proposed: ExprId,
        candidate: ExprId,
        type_metadata: Option<ExprId>,
        selector: SmolStr,
    },
    LimiterArgument {
        argument: HirLimiterArgument,
    },
    Ddt {
        expr: ExprId,
        abstol: Option<ExprId>,
    },
    Idt {
        expr: ExprId,
        ic: Option<ExprId>,
        assert: Option<ExprId>,
        abstol: Option<ExprId>,
    },
    IdtMod {
        expr: ExprId,
        ic: Option<ExprId>,
        modulus: Option<ExprId>,
        offset: Option<ExprId>,
        abstol: Option<ExprId>,
    },
    Ddx {
        expr: ExprId,
        probe: ExprId,
    },
    Limexp {
        expr: ExprId,
    },
    Absdelay {
        expr: ExprId,
        delay: ExprId,
        max_delay: Option<ExprId>,
    },
    Transition {
        site: TransitionSiteId,
        expr: ExprId,
        delay: Option<ExprId>,
        rise: Option<ExprId>,
        fall: Option<ExprId>,
        tolerance: Option<ExprId>,
    },
    /// Read-only local Jacobian action for the correlated transition site.
    TransitionDerivative {
        site: TransitionSiteId,
        input: ExprId,
        input_derivative: ExprId,
        delay: Option<ExprId>,
        rise: Option<ExprId>,
        fall: Option<ExprId>,
    },
    Slew {
        expr: ExprId,
        max_rise: Option<ExprId>,
        max_fall: Option<ExprId>,
    },
    LastCrossing {
        expr: ExprId,
        edge: Option<HirCrossDirection>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirLimiterArgument {
    Proposed,
    Previous,
}

impl From<LimiterArgument> for HirLimiterArgument {
    fn from(value: LimiterArgument) -> Self {
        match value {
            LimiterArgument::Proposed => Self::Proposed,
            LimiterArgument::Previous => Self::Previous,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HirExprKind {
    /// An explicitly omitted positional argument (`,,`). Only operators whose
    /// LRM grammar assigns meaning to a null argument may retain this node.
    NullArgument,
    Number {
        value: f64,
        raw: SmolStr,
    },
    StringLiteral {
        value: SmolStr,
    },
    Identifier {
        name: SmolStr,
    },
    SystemFunction {
        name: SmolStr,
        args: Vec<ExprId>,
    },
    Binary {
        op: SmolStr,
        left: ExprId,
        right: ExprId,
    },
    Unary {
        op: SmolStr,
        operand: ExprId,
    },
    Conditional {
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    },
    Call {
        name: SmolStr,
        args: Vec<ExprId>,
    },
    BranchAccess {
        access: SmolStr,
        pos: SmolStr,
        neg: Option<SmolStr>,
    },
    NamedBranchAccess {
        access: SmolStr,
        name: SmolStr,
    },
    ArrayAccess {
        array: SmolStr,
        index: ExprId,
    },
    ArrayLiteral {
        elements: Vec<ExprId>,
        #[serde(default)]
        assignment_pattern: bool,
    },
    AnalogOperator {
        op: HirAnalogOperator,
    },
    Laplace {
        expr: ExprId,
        kind: HirLaplaceKind,
    },
    Zi {
        expr: ExprId,
        kind: HirZiKind,
        period: ExprId,
        transition: Option<ExprId>,
        first_transition: Option<ExprId>,
    },
    NoiseSource {
        /// Dense structural identity assigned while lowering the source-order
        /// analog body. Labels do not define process correlation.
        #[serde(default)]
        process_id: u32,
        source: SmolStr,
        operands: Vec<ExprId>,
        name: Option<SmolStr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirPort {
    pub id: PortId,
    pub name: SmolStr,
    pub direction: SmolStr,
    pub discipline: SmolStr,
    pub nature_potential: Option<SmolStr>,
    pub nature_flow: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParameter {
    pub id: ParamId,
    pub name: SmolStr,
    #[serde(default = "default_true")]
    pub is_public: bool,
    pub scope: ParameterScope,
    #[serde(default)]
    pub also_model: bool,
    pub value_type: CanonicalValueType,
    /// Ordered unpacked declaration dimensions. Empty means scalar. Bounds
    /// remain expressions because an earlier scalar parameter override may
    /// change an array's shape during instance elaboration.
    #[serde(default)]
    pub dimensions: Vec<HirParameterDimension>,
    pub default: Option<f64>,
    pub default_expr: Option<HirExprRef>,
    pub range: Option<HirParamRange>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirParameterDimension {
    /// Declared left bound; direction is intentionally not normalized.
    pub left: HirExprRef,
    /// Declared right bound; direction is intentionally not normalized.
    pub right: HirExprRef,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirVariable {
    pub id: VariableId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub is_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirArray {
    pub id: ArrayId,
    pub name: SmolStr,
    pub base: VariableId,
    pub lower: i64,
    pub len: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirBranch {
    pub id: BranchId,
    pub name: SmolStr,
    pub pos_node: SmolStr,
    pub neg_node: SmolStr,
    pub discipline: SmolStr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirInternalNode {
    pub id: NodeId,
    pub name: SmolStr,
    pub discipline: SmolStr,
    pub index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirContributionKind {
    Current,
    Potential,
    Indirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirContribution {
    pub id: ContributionId,
    pub branch: SmolStr,
    pub declared_branch: Option<SmolStr>,
    pub kind: HirContributionKind,
    pub expression: HirExprRef,
    pub expr_type: CanonicalValueType,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirAssignment {
    pub target: VariableId,
    pub target_name: SmolStr,
    pub index: Option<HirExprRef>,
    pub expr: HirExprRef,
    pub expr_type: CanonicalValueType,
    pub span: SourceSpanRef,
    pub unfiltered_initial_step_guard: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirLoop {
    pub condition: HirExprRef,
    pub body: Vec<HirStatement>,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirStatement {
    Assignment(HirAssignment),
    Loop(HirLoop),
}

/// One step of the analog block, with control flow intact.
///
/// The counterpart of [`HirStatement`] for consumers that need the shape rather
/// than the guard-folded flattening. Expressions here are as the source wrote
/// them: an assignment inside a conditional holds its own right-hand side, not
/// `cond ? written : previous`.
///
/// [`HirModel::statements`] and [`HirModel::body`] describe the same module by
/// two routes. The flat list is what the existing backends read; this is what
/// the CFG level consumes, and the flat list goes away with the last of those
/// backends. See `design/VERILOGA_BACKEND_PLAN.md`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirRegion {
    Assignment(HirAssignment),
    Contribution(HirContribution),
    Conditional {
        condition: HirExprRef,
        then_body: Vec<HirRegion>,
        else_body: Vec<HirRegion>,
        span: SourceSpanRef,
    },
    Loop {
        condition: HirExprRef,
        body: Vec<HirRegion>,
        span: SourceSpanRef,
    },
}

/// One run of body expression ids and the run of executed ids it names.
///
/// Runs rather than pairs, because a correspondence between two lowerings of
/// one source expression is an *offset*, not a table.
/// [`HirLowerer::lower_expr`] appends children before their parent and mints
/// each id from the arena's length, so lowering a subtree occupies a contiguous
/// block of ids ending at the root. Two lowerings of the same
/// [`crate::ast::Expression`] therefore walk identically and mint identically
/// many ids in identically the same order, which makes the whole subtree's
/// correspondence `executed_start + (body - body_start)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirCorrespondenceSpan {
    /// First body-copy id in the run.
    pub body_start: u32,
    /// The executed-copy id that `body_start` names.
    pub executed_start: u32,
    /// How many ids the run covers. Never zero.
    pub len: u32,
}

/// Which executed expression each structured-body expression is a second
/// lowering of.
///
/// [`HirModel`] lowers a module twice into one arena: once as the guard-folded
/// `contributions`/`statements` the runtimes execute and checkpoint, and again
/// as the structured `body` the CFG consumes. The two copies share no ids, so a
/// CFG value naming its `ddt` by [`ExprId`] names an expression that
/// [`crate::canonical_ir::state::CanonicalStateLayout`] — which numbers the
/// executed copy, because that is the copy whose records the runtime allocates —
/// has never seen. This is the map across.
///
/// ## Built, not matched
///
/// The analyzer stamps one [`crate::semantic::AnalogSiteId`] on both recordings
/// of every analog-block step at the moment it clones one expression into two
/// ([`crate::semantic::AnalyzedRegion`] and the flat sink), and names the guard
/// shape it is about to fold. Lowering reads that stamp: it knows which executed
/// root a region's expression is the authored half of, walks off the guard
/// wrapper by the recorded shape, and pairs the two subtrees by their lengths.
/// Nothing here compares expression *content* to decide what pairs with what.
///
/// Congruence is nevertheless checked while the span is built — kind
/// discriminants must agree position for position — because one thing can make
/// two lowerings of one expression differ: `HirLowerer`'s array-replication work
/// budget is per module, so a replication expanded in the first copy can be
/// refused in the second. A run that fails congruence is not recorded, and the
/// operator inside it is then simply unmapped, which every consumer must refuse
/// by name rather than guess at.
///
/// ## What is deliberately not covered
///
/// A `case` arm's structured condition is `selector == match` while its executed
/// counterpart is `__guardN == match`: two different expressions, not two copies
/// of one, so no site pairs them. Module prologue statements (localparam and
/// module-scope variable initializers, `$bound_step` resets) exist only in the
/// executed copy and pair with nothing in the body — they run before the analog
/// block rather than inside it, so the region tree has no position to hold them.
/// Neither gap is silent: [`Self::executed`] returns `None`, and
/// [`crate::canonical_ir::state::CfgStateAllocation`] refuses the model by name.
///
/// A named block's local initializer is *not* in that list. It is a step of the
/// block, so the analyzer records it in the region tree as well as the flat
/// sink, under one site, and it pairs here like any other assignment. It did
/// not always: recording it only into the sink left the CFG with no definition
/// of the local at all, and the read of it fell through to Verilog-AMS
/// zero-initialisation — a wrong number rather than a refusal, which is why the
/// pairing is asserted in `tests/cfg_lowering.rs` rather than assumed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirExecutedCorrespondence {
    /// Sorted by `body_start` and pairwise disjoint, so a lookup is a binary
    /// search and a builder that overlapped two sites is a detectable bug
    /// rather than a silent last-writer-wins.
    spans: Vec<HirCorrespondenceSpan>,
}

impl HirExecutedCorrespondence {
    /// The executed-copy expression `body` is a second lowering of.
    pub fn executed(&self, body: ExprId) -> Option<ExprId> {
        let body = body.index();
        let index = match self
            .spans
            .binary_search_by_key(&body, |span| span.body_start)
        {
            Ok(index) => index,
            Err(0) => return None,
            Err(index) => index - 1,
        };
        let span = self.spans.get(index)?;
        let offset = body.checked_sub(span.body_start)?;
        if offset >= span.len {
            return None;
        }
        Some(ExprId::from(
            usize::try_from(span.executed_start.checked_add(offset)?).ok()?,
        ))
    }

    /// Every run, in body-id order.
    pub fn spans(&self) -> &[HirCorrespondenceSpan] {
        &self.spans
    }
}

/// Accumulates [`HirExecutedCorrespondence`] runs while the body is lowered.
#[derive(Debug, Default)]
struct CorrespondenceBuilder {
    spans: Vec<HirCorrespondenceSpan>,
}

impl CorrespondenceBuilder {
    /// Pair the body subtree ending at `body_root` with the executed subtree
    /// ending at `executed_root`.
    ///
    /// `body_start` is the arena length captured before the body subtree was
    /// lowered, so the run is `body_start ..= body_root`. The executed run has
    /// the same length by construction and ends at `executed_root`; both are
    /// re-checked against the arena before anything is recorded.
    fn pair(
        &mut self,
        expressions: &[HirExpression],
        body_start: usize,
        body_root: ExprId,
        executed_root: ExprId,
    ) {
        let body_root = body_root.index() as usize;
        let executed_root = executed_root.index() as usize;
        let Some(len) = body_root.checked_sub(body_start).map(|len| len + 1) else {
            return;
        };
        let Some(executed_start) = executed_root.checked_sub(len - 1) else {
            return;
        };
        // The executed copy is lowered first, so it must end before the body
        // copy begins; a run that straddles the two halves is a bug in the
        // caller's unwrapping, not a correspondence.
        if executed_root >= body_start {
            return;
        }
        for offset in 0..len {
            let (Some(body), Some(executed)) = (
                expressions.get(body_start + offset),
                expressions.get(executed_start + offset),
            ) else {
                return;
            };
            if !same_expression_kind(&body.kind, &executed.kind) {
                return;
            }
        }
        let (Ok(body_start), Ok(executed_start), Ok(len)) = (
            u32::try_from(body_start),
            u32::try_from(executed_start),
            u32::try_from(len),
        ) else {
            return;
        };
        self.spans.push(HirCorrespondenceSpan {
            body_start,
            executed_start,
            len,
        });
    }

    fn finish(mut self) -> HirExecutedCorrespondence {
        self.spans.sort_unstable_by_key(|span| span.body_start);
        // Nested regions pair their own subtrees, so an outer run can cover an
        // inner one. Keep the first (outermost) and drop anything it already
        // covers: both answer the same question, and a disjoint list is what
        // makes the lookup a binary search.
        let mut spans: Vec<HirCorrespondenceSpan> = Vec::with_capacity(self.spans.len());
        for span in self.spans {
            if let Some(previous) = spans.last()
                && span.body_start < previous.body_start + previous.len
            {
                continue;
            }
            spans.push(span);
        }
        HirExecutedCorrespondence { spans }
    }
}

/// Whether two lowered expressions are the same construct.
///
/// Discriminant equality plus the operand counts a walk depends on. It is not
/// structural equality — the ids differ, which is the whole point — but it is
/// enough that a positional pairing between the two runs cannot land an
/// operator on something that is not the same operator.
fn same_expression_kind(left: &HirExprKind, right: &HirExprKind) -> bool {
    match (left, right) {
        (HirExprKind::NullArgument, HirExprKind::NullArgument) => true,
        (HirExprKind::Number { value: left, .. }, HirExprKind::Number { value: right, .. }) => {
            left.to_bits() == right.to_bits()
        }
        (
            HirExprKind::StringLiteral { value: left },
            HirExprKind::StringLiteral { value: right },
        ) => left == right,
        (HirExprKind::Identifier { name: left }, HirExprKind::Identifier { name: right }) => {
            left == right
        }
        (
            HirExprKind::BranchAccess {
                access: left_access,
                pos: left_pos,
                neg: left_neg,
            },
            HirExprKind::BranchAccess {
                access: right_access,
                pos: right_pos,
                neg: right_neg,
            },
        ) => left_access == right_access && left_pos == right_pos && left_neg == right_neg,
        (
            HirExprKind::NamedBranchAccess {
                access: left_access,
                name: left_name,
            },
            HirExprKind::NamedBranchAccess {
                access: right_access,
                name: right_name,
            },
        ) => left_access == right_access && left_name == right_name,
        (
            HirExprKind::SystemFunction {
                name: left_name,
                args: left_args,
            },
            HirExprKind::SystemFunction {
                name: right_name,
                args: right_args,
            },
        )
        | (
            HirExprKind::Call {
                name: left_name,
                args: left_args,
            },
            HirExprKind::Call {
                name: right_name,
                args: right_args,
            },
        ) => left_name == right_name && left_args.len() == right_args.len(),
        (
            HirExprKind::ArrayLiteral {
                elements: left,
                assignment_pattern: left_pattern,
            },
            HirExprKind::ArrayLiteral {
                elements: right,
                assignment_pattern: right_pattern,
            },
        ) => left.len() == right.len() && left_pattern == right_pattern,
        (
            HirExprKind::NoiseSource {
                source: left_source,
                operands: left_operands,
                ..
            },
            HirExprKind::NoiseSource {
                source: right_source,
                operands: right_operands,
                ..
            },
        ) => left_source == right_source && left_operands.len() == right_operands.len(),
        (HirExprKind::Unary { op: left, .. }, HirExprKind::Unary { op: right, .. }) => {
            left == right
        }
        (
            HirExprKind::ArrayAccess { array: left, .. },
            HirExprKind::ArrayAccess { array: right, .. },
        ) => left == right,
        (HirExprKind::Binary { op: left, .. }, HirExprKind::Binary { op: right, .. }) => {
            left == right
        }
        (HirExprKind::Conditional { .. }, HirExprKind::Conditional { .. }) => true,
        (HirExprKind::AnalogOperator { op: left }, HirExprKind::AnalogOperator { op: right }) => {
            same_analog_operator(left, right)
        }
        (HirExprKind::Laplace { kind: left, .. }, HirExprKind::Laplace { kind: right, .. }) => {
            std::mem::discriminant(left) == std::mem::discriminant(right)
        }
        (HirExprKind::Zi { kind: left, .. }, HirExprKind::Zi { kind: right, .. }) => {
            std::mem::discriminant(left) == std::mem::discriminant(right)
        }
        _ => false,
    }
}

/// Whether two analog operators are the same operator.
///
/// Deliberately blind to [`TransitionSiteId`]: the ordinal counter runs across
/// both copies, so a `transition`'s two lowerings carry *different* ordinals by
/// construction. That difference is exactly what the correspondence exists to
/// bridge, so demanding equality here would refuse every module that transitions.
fn same_analog_operator(left: &HirAnalogOperator, right: &HirAnalogOperator) -> bool {
    match (left, right) {
        (
            HirAnalogOperator::Limit { selector: left, .. },
            HirAnalogOperator::Limit {
                selector: right, ..
            },
        ) => left == right,
        (
            HirAnalogOperator::LimiterArgument { argument: left },
            HirAnalogOperator::LimiterArgument { argument: right },
        ) => left == right,
        (
            HirAnalogOperator::LastCrossing { edge: left, .. },
            HirAnalogOperator::LastCrossing { edge: right, .. },
        ) => left == right,
        _ => std::mem::discriminant(left) == std::mem::discriminant(right),
    }
}

/// The executed root one analog site produced, and how it wraps the authored
/// expression the structured body holds.
#[derive(Debug, Clone, Copy)]
struct ExecutedSite {
    value: ExprId,
    value_guard: AnalogSiteGuard,
    index: Option<ExprId>,
}

impl ExecutedSite {
    /// Walk off the guard wrapper to the executed copy of the authored
    /// expression.
    ///
    /// `None` when the lowered tree does not have the shape the analyzer said
    /// it folded — which cannot happen through the compiler's own pipeline, and
    /// is a refusal rather than an assertion because a hand-built
    /// `AnalyzedModule` can reach here.
    fn authored(&self, expressions: &[HirExpression]) -> Option<ExprId> {
        match self.value_guard {
            AnalogSiteGuard::None => Some(self.value),
            AnalogSiteGuard::Select => match &expressions.get(self.value.index() as usize)?.kind {
                HirExprKind::Conditional { then_expr, .. } => Some(*then_expr),
                _ => None,
            },
            AnalogSiteGuard::Conjunction => {
                match &expressions.get(self.value.index() as usize)?.kind {
                    // The lowering spells a binary operator with `{:?}`, so the
                    // label is asked for the same way rather than written out.
                    HirExprKind::Binary { op, right, .. }
                        if *op == format!("{:?}", BinaryOp::And) =>
                    {
                        Some(*right)
                    }
                    _ => None,
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirModel {
    pub module_id: ModuleId,
    pub module_name: SmolStr,
    pub schema_version: u32,
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    /// Full BLAKE3 identity of the exact preprocessed source closure.
    pub source_identity: SmolStr,
    pub compiler_version: SmolStr,
    pub feature_flags: Vec<SmolStr>,
    /// Effective module-scoped `default_transition` value in seconds.
    /// Zero is legal and denotes an abrupt transition.
    #[serde(default = "canonical_default_transition")]
    pub default_transition: f64,
    pub ports: Vec<HirPort>,
    pub parameters: Vec<HirParameter>,
    pub variables: Vec<HirVariable>,
    pub arrays: Vec<HirArray>,
    pub branches: Vec<HirBranch>,
    pub contributions: Vec<HirContribution>,
    pub statements: Vec<HirStatement>,
    /// The analog block with its control flow intact; see [`HirRegion`].
    pub body: Vec<HirRegion>,
    /// Which executed expression each [`Self::body`] expression is a second
    /// lowering of. See [`HirExecutedCorrespondence`].
    #[serde(default)]
    pub executed_correspondence: HirExecutedCorrespondence,
    pub expressions: Vec<HirExpression>,
    pub internal_nodes: Vec<HirInternalNode>,
    pub ground_nodes: Vec<SmolStr>,
}

impl HirModel {
    pub fn from_analyzed_module(metadata: &CanonicalMetadata, module: &AnalyzedModule) -> Self {
        let branch_names = module
            .branches
            .iter()
            .map(|branch| branch.name.clone())
            .collect();
        let mut lowerer = HirLowerer::new(branch_names);
        let mut parameters: Vec<_> = module
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                let dimensions = parameter
                    .dimensions
                    .iter()
                    .map(|dimension| HirParameterDimension {
                        left: lowerer.lower_expr(&dimension.left),
                        right: lowerer.lower_expr(&dimension.right),
                        span: SourceSpanRef::from(dimension.span),
                    })
                    .collect();
                let default_expr = parameter
                    .default_expr
                    .as_ref()
                    .map(|expr| lowerer.lower_expr(expr));
                let range = parameter.range.as_ref().map(|range| {
                    let mut lowered = HirParamRange::from_range(range);
                    lowered.min_expression = range
                        .min_expression
                        .as_ref()
                        .map(|expression| lowerer.lower_expr(expression));
                    lowered.max_expression = range
                        .max_expression
                        .as_ref()
                        .map(|expression| lowerer.lower_expr(expression));
                    lowered.exclude_expressions = range
                        .exclude_expressions
                        .iter()
                        .map(|expression| lowerer.lower_expr(expression))
                        .collect();
                    lowered
                });
                HirParameter {
                    id: ParamId::from(index),
                    name: parameter.name.clone(),
                    is_public: parameter.is_public,
                    scope: parameter.scope,
                    also_model: parameter.also_model,
                    value_type: CanonicalValueType::from(parameter.value_type),
                    dimensions,
                    default: parameter.default,
                    default_expr,
                    range,
                    aliases: Vec::new(),
                }
            })
            .collect();

        for alias in &module.param_aliases {
            if let Some(parameter) = parameters.get_mut(alias.target) {
                parameter.aliases.push(alias.alias.clone());
            }
        }

        let mut arrays: Vec<_> = module.arrays.iter().collect();
        arrays.sort_by(|(left_name, _), (right_name, _)| left_name.cmp(right_name));
        let arrays = arrays
            .into_iter()
            .enumerate()
            .map(|(index, (name, array))| HirArray {
                id: ArrayId::from(index),
                name: name.clone(),
                base: VariableId::from(array.base),
                lower: array.lower,
                len: u32::try_from(array.len).expect("array length exceeds u32::MAX"),
            })
            .collect();

        // The executed copy is lowered first and records, per analog site, the
        // root it produced and the guard shape wrapped around it. The body
        // lowering below reads that back to pair its own ids with these.
        let mut executed_sites: HashMap<AnalogSiteId, ExecutedSite> = HashMap::new();

        let span = crate::metrics::FineSpan::new("hir.executed_copy");
        let contributions = module
            .contributions
            .iter()
            .enumerate()
            .map(|(index, contribution)| {
                let expression = lowerer.lower_expr(&contribution.expression);
                executed_sites.insert(
                    contribution.site,
                    ExecutedSite {
                        value: expression.id,
                        value_guard: contribution.expression_guard,
                        index: None,
                    },
                );
                HirContribution {
                    id: ContributionId::from(index),
                    branch: contribution.branch.clone(),
                    declared_branch: contribution.declared_branch.clone(),
                    kind: contribution_kind(contribution.indirect, contribution.is_current),
                    expression,
                    expr_type: CanonicalValueType::from(contribution.expr_type),
                    span: SourceSpanRef::from(contribution.span),
                }
            })
            .collect();

        let statements = module
            .statements
            .iter()
            .map(|statement| lower_statement(&mut lowerer, statement, &mut executed_sites))
            .collect();
        span.finish(&format!(
            "module={} expressions={} sites={}",
            module.name,
            lowerer.expressions.len(),
            executed_sites.len()
        ));

        // The analyzer records a region and pushes the flat contribution at the
        // same program point, so a pre-order walk of the body meets them in the
        // order they were assigned ids. `validate_body` re-checks that against
        // the flat list rather than trusting it.
        let span = crate::metrics::FineSpan::new("hir.body_copy");
        let executed_expression_count = lowerer.expressions.len();
        let mut next_contribution = 0usize;
        let mut correspondence = CorrespondenceBuilder::default();
        let body = module
            .body
            .iter()
            .map(|region| {
                lower_region(
                    &mut lowerer,
                    &mut next_contribution,
                    region,
                    &executed_sites,
                    &mut correspondence,
                )
            })
            .collect();

        let executed_correspondence = correspondence.finish();
        let expressions = lowerer.expressions;
        span.finish(&format!(
            "module={} executed_expressions={executed_expression_count} total_expressions={}",
            module.name,
            expressions.len()
        ));

        Self {
            module_id: ModuleId::new(0),
            module_name: module.name.clone(),
            schema_version: metadata.schema_version,
            source_package: metadata.source_package.clone(),
            source_digest: metadata.source_digest.clone(),
            source_identity: metadata.source_identity.clone(),
            compiler_version: metadata.compiler_version.clone(),
            feature_flags: metadata.feature_flags.clone(),
            default_transition: module.default_transition,
            ports: module
                .ports
                .iter()
                .enumerate()
                .map(|(index, port)| HirPort {
                    id: PortId::from(index),
                    name: port.name.clone(),
                    direction: port_direction_label(port.direction),
                    discipline: port.discipline.clone(),
                    nature_potential: port.nature_potential.clone(),
                    nature_flow: port.nature_flow.clone(),
                })
                .collect(),
            parameters,
            variables: module
                .variables
                .iter()
                .enumerate()
                .map(|(index, variable)| HirVariable {
                    id: VariableId::from(index),
                    name: variable.name.clone(),
                    value_type: CanonicalValueType::from(variable.value_type),
                    is_state: variable.is_state,
                })
                .collect(),
            arrays,
            branches: module
                .branches
                .iter()
                .enumerate()
                .map(|(index, branch)| HirBranch {
                    id: BranchId::from(index),
                    name: branch.name.clone(),
                    pos_node: branch.pos_node.clone(),
                    neg_node: branch.neg_node.clone(),
                    discipline: branch.discipline.clone(),
                })
                .collect(),
            contributions,
            statements,
            body,
            executed_correspondence,
            expressions,
            internal_nodes: module
                .internal_nodes
                .iter()
                .map(|node| HirInternalNode {
                    id: NodeId::from(node.index),
                    name: node.name.clone(),
                    discipline: node.discipline.clone(),
                    index: u32::try_from(node.index).expect("internal node index exceeds u32::MAX"),
                })
                .collect(),
            ground_nodes: module.ground_nodes.clone(),
        }
    }

    pub fn validate(&self) -> IrValidationResult {
        if self.schema_version != CANONICAL_IR_SCHEMA_VERSION {
            return Err(vec![IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "unsupported HIR schema version {}; expected {}",
                    self.schema_version, CANONICAL_IR_SCHEMA_VERSION
                ),
            )]);
        }

        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                "HIR module name must not be empty",
            ));
        }

        if self.ports.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                "HIR module must have at least one port",
            ));
        }

        if !self.default_transition.is_finite() || self.default_transition < 0.0 {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR default_transition must be finite and non-negative, got {}",
                    self.default_transition
                ),
            ));
        }

        validate_dense_port_ids(&mut diagnostics, &self.ports);
        validate_dense_parameter_ids(&mut diagnostics, &self.parameters);
        validate_dense_variable_ids(&mut diagnostics, &self.variables);
        validate_dense_array_ids(&mut diagnostics, &self.arrays);
        validate_dense_branch_ids(&mut diagnostics, &self.branches);
        validate_dense_contribution_ids(&mut diagnostics, &self.contributions);
        validate_dense_internal_node_ids(&mut diagnostics, &self.internal_nodes);
        self.validate_expressions(&mut diagnostics);
        self.validate_arrays(&mut diagnostics);
        self.validate_parameter_aliases(&mut diagnostics);
        self.validate_parameter_expression_refs(&mut diagnostics);
        diagnostics.extend(super::parameter_array::validate_parameter_array_contract(
            CompilerPhase::HirValidation,
            &self.parameters,
            &self.expressions,
        ));
        self.validate_branches(&mut diagnostics);
        self.validate_contributions(&mut diagnostics);
        self.validate_statements(&mut diagnostics, &self.statements);
        self.validate_body(&mut diagnostics);

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }

    fn validate_expressions(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let known_nodes = self.known_node_names();
        let declared_branches = self.declared_branch_names();
        let value_symbols = self.known_value_symbol_names();

        for (expected, expression) in self.expressions.iter().enumerate() {
            let expected = u32::try_from(expected).expect("HIR expression count exceeds u32::MAX");
            if expression.id.index() != expected {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR expression IDs must be dense: expected ExprId({}) at index {}, found {}",
                        expected, expected, expression.id
                    ),
                ));
            }

            self.validate_expression_children(
                diagnostics,
                expression,
                &known_nodes,
                &declared_branches,
                &value_symbols,
            );
        }
    }

    fn validate_expression_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        known_nodes: &HashSet<SmolStr>,
        declared_branches: &HashSet<SmolStr>,
        value_symbols: &HashSet<SmolStr>,
    ) {
        match &expression.kind {
            HirExprKind::NullArgument
            | HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. } => {}
            HirExprKind::Identifier { name } => {
                self.validate_identifier(diagnostics, expression, name, value_symbols);
            }
            HirExprKind::BranchAccess { pos, neg, .. } => {
                self.validate_branch_access_node(diagnostics, expression, pos, known_nodes);
                if let Some(neg) = neg {
                    self.validate_branch_access_node(diagnostics, expression, neg, known_nodes);
                }
            }
            HirExprKind::NamedBranchAccess { name, .. } => {
                if !declared_branches.contains(name) {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        format!("HIR unknown named branch access '{}'", name),
                        expression.span,
                    ));
                }
            }
            HirExprKind::SystemFunction { args, .. } => {
                self.validate_expression_child_list(diagnostics, expression, "arg", args);
            }
            HirExprKind::Call { name, args } => {
                self.validate_expression_child_list(diagnostics, expression, "arg", args);
                self.validate_filter_call_vectors(diagnostics, name, args);
                self.validate_zi_call_budget(diagnostics, expression, name, args);
            }
            HirExprKind::Binary { left, right, .. } => {
                self.validate_expression_child(diagnostics, expression, "left", *left);
                self.validate_expression_child(diagnostics, expression, "right", *right);
            }
            HirExprKind::Unary { operand, .. } => {
                self.validate_expression_child(diagnostics, expression, "operand", *operand);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.validate_expression_child(diagnostics, expression, "condition", *condition);
                self.validate_expression_child(diagnostics, expression, "then_expr", *then_expr);
                self.validate_expression_child(diagnostics, expression, "else_expr", *else_expr);
            }
            HirExprKind::ArrayAccess { array, index } => {
                self.validate_expression_child(diagnostics, expression, "index", *index);
                self.validate_array_access_target(diagnostics, expression, array, value_symbols);
            }
            HirExprKind::ArrayLiteral { elements, .. } => {
                self.validate_expression_child_list(diagnostics, expression, "element", elements);
            }
            HirExprKind::AnalogOperator { op } => {
                self.validate_analog_operator_children(diagnostics, expression, op);
            }
            HirExprKind::Laplace { expr, kind } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_laplace_children(diagnostics, expression, kind);
            }
            HirExprKind::Zi {
                expr,
                kind,
                period,
                transition,
                first_transition,
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_expression_child(diagnostics, expression, "period", *period);
                if let Some(child) = transition {
                    self.validate_expression_child(diagnostics, expression, "transition", *child);
                }
                if let Some(child) = first_transition {
                    self.validate_expression_child(
                        diagnostics,
                        expression,
                        "first_transition",
                        *child,
                    );
                }
                self.validate_zi_children(diagnostics, expression, kind);
            }
            HirExprKind::NoiseSource {
                process_id,
                operands,
                ..
            } => {
                if *process_id == u32::MAX {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        "HIR noise source is missing its semantic process identity",
                        expression.span,
                    ));
                }
                self.validate_expression_child_list(diagnostics, expression, "operand", operands);
            }
        }
    }

    fn validate_identifier(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        name: &SmolStr,
        value_symbols: &HashSet<SmolStr>,
    ) {
        if self.arrays.iter().any(|array| array.name == *name)
            || self
                .parameters
                .iter()
                .any(|parameter| parameter.name == *name && !parameter.dimensions.is_empty())
        {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR array identifier '{}' requires an index", name),
                expression.span,
            ));
            return;
        }

        if !value_symbols.contains(name) {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR unknown identifier '{}'", name),
                expression.span,
            ));
        }
    }

    fn validate_array_access_target(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        name: &SmolStr,
        value_symbols: &HashSet<SmolStr>,
    ) {
        let local_array_count = self
            .arrays
            .iter()
            .filter(|array| array.name == *name)
            .count();
        let parameter = self
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name);
        let parameter_rank = parameter.map_or(0, |parameter| parameter.dimensions.len());

        if local_array_count > 1 || (local_array_count != 0 && parameter_rank != 0) {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR array access target '{}' is ambiguous", name),
                expression.span,
            ));
            return;
        }

        if local_array_count == 1 {
            return;
        }

        if parameter_rank == 1 {
            return;
        }

        if parameter_rank > 1 {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR parameter array access '{}' supplies one index for declared rank {}",
                    name, parameter_rank
                ),
                expression.span,
            ));
        } else if value_symbols.contains(name) {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR scalar symbol '{}' must not be indexed", name),
                expression.span,
            ));
        } else {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR unknown array access target '{}'", name),
                expression.span,
            ));
        }
    }

    fn validate_branch_access_node(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        node: &SmolStr,
        known_nodes: &HashSet<SmolStr>,
    ) {
        if !known_nodes.contains(node) {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!("HIR unknown branch access node '{}'", node),
                expression.span,
            ));
        }
    }

    fn validate_analog_operator_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        op: &HirAnalogOperator,
    ) {
        match op {
            HirAnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            } => {
                self.validate_expression_child(diagnostics, expression, "proposed", *proposed);
                self.validate_expression_child(diagnostics, expression, "candidate", *candidate);
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "type_metadata",
                    *type_metadata,
                );
            }
            HirAnalogOperator::LimiterArgument { .. } => {}
            HirAnalogOperator::Ddt { expr, abstol } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_optional_expression_child(diagnostics, expression, "abstol", *abstol);
            }
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_optional_expression_child(diagnostics, expression, "ic", *ic);
                self.validate_optional_expression_child(diagnostics, expression, "assert", *assert);
                self.validate_optional_expression_child(diagnostics, expression, "abstol", *abstol);
            }
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_optional_expression_child(diagnostics, expression, "ic", *ic);
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "modulus",
                    *modulus,
                );
                self.validate_optional_expression_child(diagnostics, expression, "offset", *offset);
                self.validate_optional_expression_child(diagnostics, expression, "abstol", *abstol);
            }
            HirAnalogOperator::Ddx { expr, probe } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_expression_child(diagnostics, expression, "probe", *probe);
            }
            HirAnalogOperator::Limexp { expr } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
            }
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_expression_child(diagnostics, expression, "delay", *delay);
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "max_delay",
                    *max_delay,
                );
            }
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                ..
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_optional_expression_child(diagnostics, expression, "delay", *delay);
                self.validate_optional_expression_child(diagnostics, expression, "rise", *rise);
                self.validate_optional_expression_child(diagnostics, expression, "fall", *fall);
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "tolerance",
                    *tolerance,
                );
            }
            HirAnalogOperator::TransitionDerivative {
                input,
                input_derivative,
                delay,
                rise,
                fall,
                ..
            } => {
                self.validate_expression_child(diagnostics, expression, "input", *input);
                self.validate_expression_child(
                    diagnostics,
                    expression,
                    "input_derivative",
                    *input_derivative,
                );
                self.validate_optional_expression_child(diagnostics, expression, "delay", *delay);
                self.validate_optional_expression_child(diagnostics, expression, "rise", *rise);
                self.validate_optional_expression_child(diagnostics, expression, "fall", *fall);
            }
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "max_rise",
                    *max_rise,
                );
                self.validate_optional_expression_child(
                    diagnostics,
                    expression,
                    "max_fall",
                    *max_fall,
                );
            }
            HirAnalogOperator::LastCrossing { expr, .. } => {
                self.validate_expression_child(diagnostics, expression, "expr", *expr);
            }
        }
    }

    fn validate_laplace_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        kind: &HirLaplaceKind,
    ) {
        match kind {
            HirLaplaceKind::ZeroPole { zeros, poles } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirLaplaceKind::ZeroDenominator { zeros, denominator } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
            HirLaplaceKind::NumeratorPole { numerator, poles } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
            }
            HirLaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
            }
        }
    }

    fn validate_zi_children(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        kind: &HirZiKind,
    ) {
        let (operator, numerator_scalars, denominator_scalars) = match kind {
            HirZiKind::ZeroPole { zeros, poles } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
                ("zi_zp", zeros.len(), poles.len())
            }
            HirZiKind::ZeroDenominator { zeros, denominator } => {
                self.validate_expression_child_list(diagnostics, expression, "zeros", zeros);
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
                ("zi_zd", zeros.len(), denominator.len())
            }
            HirZiKind::NumeratorPole { numerator, poles } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(diagnostics, expression, "poles", poles);
                ("zi_np", numerator.len(), poles.len())
            }
            HirZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "numerator",
                    numerator,
                );
                self.validate_expression_child_list(
                    diagnostics,
                    expression,
                    "denominator",
                    denominator,
                );
                ("zi_nd", numerator.len(), denominator.len())
            }
        };
        if let Err(error) = crate::zfilter::validate_zi_runtime_operand_budget(
            operator,
            numerator_scalars,
            denominator_scalars,
        ) {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                error.to_string(),
                expression.span,
            ));
        }
    }

    fn validate_zi_call_budget(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        name: &SmolStr,
        args: &[ExprId],
    ) {
        if !matches!(
            name.to_ascii_lowercase().as_str(),
            "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"
        ) || args.len() < 3
        {
            return;
        }
        let scalar_count = |id: ExprId| {
            self.expressions
                .get(usize::from(id))
                .map(|child| match &child.kind {
                    HirExprKind::NullArgument => 0,
                    HirExprKind::ArrayLiteral { elements, .. } => elements.len(),
                    _ => 1,
                })
        };
        let (Some(numerator), Some(denominator)) = (scalar_count(args[1]), scalar_count(args[2]))
        else {
            return;
        };
        if let Err(error) =
            crate::zfilter::validate_zi_runtime_operand_budget(name, numerator, denominator)
        {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                error.to_string(),
                expression.span,
            ));
        }
    }

    fn validate_filter_call_vectors(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        name: &SmolStr,
        args: &[ExprId],
    ) {
        let normalized = name.to_ascii_lowercase();
        let roles = match normalized.as_str() {
            "laplace_zp" | "zi_zp" => ("zeros", "poles", true),
            "laplace_zd" | "zi_zd" => ("zeros", "denominator", true),
            "laplace_np" | "zi_np" => ("numerator", "poles", false),
            "laplace_nd" | "zi_nd" => ("numerator", "denominator", false),
            _ => return,
        };

        for (index, role, allow_null) in [(1, roles.0, roles.2), (2, roles.1, false)] {
            let Some(id) = args.get(index) else {
                return;
            };
            let Some(argument) = self.expressions.get(usize::from(*id)) else {
                continue;
            };
            let detail = match &argument.kind {
                HirExprKind::NullArgument if allow_null => continue,
                HirExprKind::NullArgument => format!("{name} {role} operand may not be null"),
                HirExprKind::ArrayLiteral {
                    assignment_pattern: true,
                    ..
                } => continue,
                HirExprKind::ArrayLiteral { .. } => format!(
                    "{name} {role} vector must be an assignment pattern opened with `'{{` or an array identifier; ordinary concatenation `{{...}}` is not a Verilog-AMS array value"
                ),
                _ => format!(
                    "{name} {role} operand must be an assignment pattern opened with `'{{` or an array identifier; a scalar expression is not a filter vector"
                ),
            };
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                detail,
                argument.span,
            ));
        }
    }

    fn validate_optional_expression_child(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        label: &str,
        child: Option<ExprId>,
    ) {
        if let Some(child) = child {
            self.validate_expression_child(diagnostics, expression, label, child);
        }
    }

    fn validate_expression_child_list(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        label: &str,
        children: &[ExprId],
    ) {
        for (index, child) in children.iter().copied().enumerate() {
            self.validate_expression_child(
                diagnostics,
                expression,
                &format!("{label}[{index}]"),
                child,
            );
        }
    }

    fn validate_expression_child(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        expression: &HirExpression,
        label: &str,
        child: ExprId,
    ) {
        if usize::from(child) >= self.expressions.len() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression {} child {} {} is outside expression arena length {}",
                    expression.id,
                    label,
                    child,
                    self.expressions.len()
                ),
                expression.span,
            ));
            return;
        }

        if child.index() >= expression.id.index() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression {} child {} {} violates expression postorder",
                    expression.id, label, child
                ),
                expression.span,
            ));
        }
    }

    fn validate_arrays(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let variable_count = self.variables.len();
        let mut names = HashSet::new();
        let mut valid_ranges = Vec::<(usize, usize, &SmolStr)>::new();

        for array in &self.arrays {
            if array.name.is_empty() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    "HIR array name must not be empty",
                ));
            } else if !names.insert(array.name.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR duplicate array name '{}'", array.name),
                ));
            }

            let base = usize::from(array.base);
            let Ok(len) = usize::try_from(array.len) else {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR array '{}' length cannot fit this target", array.name),
                ));
                continue;
            };
            if len == 0 {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR array '{}' must not have zero length", array.name),
                ));
            }
            let Some(end) = base.checked_add(len) else {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR array '{}' base overflows variable index space",
                        array.name
                    ),
                ));
                continue;
            };

            if base >= variable_count || end > variable_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR array '{}' base {} with len {} exceeds variable count {}",
                        array.name, array.base, len, variable_count
                    ),
                ));
                continue;
            }

            if len != 0 {
                let element_type = self.variables[base].value_type;
                if let Some(variable) = self.variables[base..end]
                    .iter()
                    .find(|variable| variable.value_type != element_type)
                {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR array '{}' mixes element type {:?} with variable '{}' type {:?}",
                            array.name, element_type, variable.name, variable.value_type
                        ),
                    ));
                }
                valid_ranges.push((base, end, &array.name));
            }
        }

        valid_ranges.sort_by(|left, right| {
            (left.0, left.1, left.2.as_str()).cmp(&(right.0, right.1, right.2.as_str()))
        });
        let mut active: Option<(usize, &SmolStr)> = None;
        for (base, end, name) in valid_ranges {
            if let Some((active_end, active_name)) = active
                && base < active_end
            {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR array '{}' storage range [{base}:{end}) overlaps array '{}' ending at {}",
                        name, active_name, active_end
                    ),
                ));
            }
            if active.is_none_or(|(active_end, _)| end > active_end) {
                active = Some((end, name));
            }
        }
    }

    fn validate_parameter_aliases(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let parameter_names: HashSet<_> = self
            .parameters
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect();
        let mut observed_parameter_names = HashSet::new();
        let mut aliases = HashSet::new();

        for parameter in &self.parameters {
            if !observed_parameter_names.insert(parameter.name.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR duplicate parameter name '{}'", parameter.name),
                ));
            }

            for alias in &parameter.aliases {
                if alias.is_empty() {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR parameter alias for '{}' must not be empty",
                            parameter.name
                        ),
                    ));
                }

                if parameter_names.contains(alias) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR parameter alias '{}' collides with parameter name",
                            alias
                        ),
                    ));
                }

                if !aliases.insert(alias.clone()) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::HirValidation,
                        format!("HIR duplicate parameter alias '{}'", alias),
                    ));
                }
            }
        }
    }

    fn validate_parameter_expression_refs(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        for parameter in &self.parameters {
            for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
                self.validate_expr_ref(
                    diagnostics,
                    &format!(
                        "parameter '{}' dimension {} left bound",
                        parameter.name, dimension_index
                    ),
                    &dimension.left,
                );
                self.validate_expr_ref(
                    diagnostics,
                    &format!(
                        "parameter '{}' dimension {} right bound",
                        parameter.name, dimension_index
                    ),
                    &dimension.right,
                );
            }
            if let Some(default_expr) = &parameter.default_expr {
                self.validate_expr_ref(
                    diagnostics,
                    &format!("parameter '{}' default", parameter.name),
                    default_expr,
                );
            }
            if let Some(range) = &parameter.range {
                if let Some(expression) = &range.min_expression {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("parameter '{}' lower range bound", parameter.name),
                        expression,
                    );
                }
                if let Some(expression) = &range.max_expression {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("parameter '{}' upper range bound", parameter.name),
                        expression,
                    );
                }
                for expression in &range.exclude_expressions {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("parameter '{}' excluded range value", parameter.name),
                        expression,
                    );
                }
            }
        }
    }

    fn validate_branches(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let known_nodes = self.known_node_names();
        let mut branch_names = HashSet::new();

        for branch in &self.branches {
            if branch.name.is_empty() {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    "HIR branch name must not be empty",
                ));
            } else if !branch_names.insert(branch.name.clone()) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!("HIR duplicate branch name '{}'", branch.name),
                ));
            }

            if !known_nodes.contains(&branch.pos_node) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR branch '{}' pos_node '{}' is unknown",
                        branch.name, branch.pos_node
                    ),
                ));
            }

            if !branch.neg_node.is_empty() && !known_nodes.contains(&branch.neg_node) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR branch '{}' neg_node '{}' is unknown",
                        branch.name, branch.neg_node
                    ),
                ));
            }
        }
    }

    fn validate_contributions(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let known_nodes = self.known_node_names();
        let declared_branches = self.declared_branch_names();

        for contribution in &self.contributions {
            self.validate_expr_ref(
                diagnostics,
                &format!("contribution {} expression", contribution.id.index()),
                &contribution.expression,
            );

            if contribution.branch.is_empty() {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    "HIR contribution branch name must not be empty",
                    contribution.span,
                ));
            } else if !is_valid_contribution_branch(
                contribution.branch.as_str(),
                &known_nodes,
                &declared_branches,
            ) {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!("HIR unknown contribution branch '{}'", contribution.branch),
                    contribution.span,
                ));
            }
        }
    }

    fn validate_statements(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        statements: &[HirStatement],
    ) {
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("assignment '{}' expression", assignment.target_name),
                        &assignment.expr,
                    );
                    if let Some(index) = &assignment.index {
                        self.validate_expr_ref(
                            diagnostics,
                            &format!("assignment '{}' index", assignment.target_name),
                            index,
                        );
                    }

                    let target_index = usize::from(assignment.target);
                    if target_index >= self.variables.len() {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR assignment target {} is outside variable count {}",
                                assignment.target,
                                self.variables.len()
                            ),
                            assignment.span,
                        ));
                        continue;
                    }

                    self.validate_assignment_shape(diagnostics, assignment);
                }
                HirStatement::Loop(loop_statement) => {
                    self.validate_expr_ref(
                        diagnostics,
                        "loop condition",
                        &loop_statement.condition,
                    );
                    self.validate_statements(diagnostics, &loop_statement.body);
                }
            }
        }
    }

    /// Check the structured body against the flat lists it duplicates.
    ///
    /// The two are produced by one walk, so they must agree: every region
    /// contribution carries the id of the flat contribution recorded at the
    /// same program point, and the body must account for all of them. If that
    /// correspondence ever slips, a CFG built from the body would stamp the
    /// wrong branch — so it is checked, not assumed.
    fn validate_body(&self, diagnostics: &mut Vec<IrDiagnostic>) {
        let mut expected_contribution = 0usize;
        self.validate_regions(diagnostics, &self.body, &mut expected_contribution);

        if expected_contribution != self.contributions.len() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR body holds {} contributions but the module records {}",
                    expected_contribution,
                    self.contributions.len()
                ),
            ));
        }
    }

    fn validate_regions(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        regions: &[HirRegion],
        next_contribution: &mut usize,
    ) {
        for region in regions {
            match region {
                HirRegion::Assignment(assignment) => {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("region assignment '{}' expression", assignment.target_name),
                        &assignment.expr,
                    );
                    if let Some(index) = &assignment.index {
                        self.validate_expr_ref(
                            diagnostics,
                            &format!("region assignment '{}' index", assignment.target_name),
                            index,
                        );
                    }

                    let target_index = usize::from(assignment.target);
                    if target_index >= self.variables.len() {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR region assignment target {} is outside variable count {}",
                                assignment.target,
                                self.variables.len()
                            ),
                            assignment.span,
                        ));
                        continue;
                    }

                    self.validate_assignment_shape(diagnostics, assignment);
                }
                HirRegion::Contribution(contribution) => {
                    self.validate_expr_ref(
                        diagnostics,
                        &format!("region contribution '{}' expression", contribution.branch),
                        &contribution.expression,
                    );

                    let index = usize::from(contribution.id);
                    if index != *next_contribution {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR region contribution id {} is out of walk order; expected {}",
                                contribution.id, next_contribution
                            ),
                            contribution.span,
                        ));
                    } else if let Some(flat) = self.contributions.get(index) {
                        if flat.branch != contribution.branch || flat.kind != contribution.kind {
                            diagnostics.push(IrDiagnostic::error(
                                CompilerPhase::HirValidation,
                                format!(
                                    "HIR region contribution {} targets '{}' but the flat form targets '{}'",
                                    contribution.id, contribution.branch, flat.branch
                                ),
                                contribution.span,
                            ));
                        }
                    } else {
                        diagnostics.push(IrDiagnostic::error(
                            CompilerPhase::HirValidation,
                            format!(
                                "HIR region contribution id {} is outside contribution count {}",
                                contribution.id,
                                self.contributions.len()
                            ),
                            contribution.span,
                        ));
                    }

                    *next_contribution += 1;
                }
                HirRegion::Conditional {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    self.validate_expr_ref(diagnostics, "region condition", condition);
                    self.validate_regions(diagnostics, then_body, next_contribution);
                    self.validate_regions(diagnostics, else_body, next_contribution);
                }
                HirRegion::Loop {
                    condition, body, ..
                } => {
                    self.validate_expr_ref(diagnostics, "region loop condition", condition);
                    self.validate_regions(diagnostics, body, next_contribution);
                }
            }
        }
    }

    fn validate_expr_ref(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        owner: &str,
        expr_ref: &HirExprRef,
    ) {
        let index = usize::from(expr_ref.id);
        let Some(expression) = self.expressions.get(index) else {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression ref {} id {} is outside expression arena length {}",
                    owner,
                    expr_ref.id,
                    self.expressions.len()
                ),
                expr_ref.span,
            ));
            return;
        };

        let actual_kind = hir_expr_kind_label(&expression.kind);
        if expr_ref.kind.as_str() != actual_kind {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR expression ref {} kind '{}' does not match '{}'",
                    owner, expr_ref.kind, actual_kind
                ),
                expr_ref.span,
            ));
        }
    }

    fn validate_assignment_shape(
        &self,
        diagnostics: &mut Vec<IrDiagnostic>,
        assignment: &HirAssignment,
    ) {
        let target_variable = &self.variables[usize::from(assignment.target)];
        let array_by_name = self
            .arrays
            .iter()
            .find(|array| array.name == assignment.target_name);
        let array_by_base = self
            .arrays
            .iter()
            .find(|array| array.base == assignment.target);

        if assignment.index.is_some() {
            if let Some(array) = array_by_name {
                if assignment.target != array.base {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR indexed assignment '{}' target {} must match array base {}",
                            assignment.target_name, assignment.target, array.base
                        ),
                        assignment.span,
                    ));
                }
            } else if let Some(array) = array_by_base {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR indexed assignment target name '{}' does not match array '{}'",
                        assignment.target_name, array.name
                    ),
                    assignment.span,
                ));
            } else {
                if assignment.target_name != target_variable.name {
                    diagnostics.push(IrDiagnostic::error(
                        CompilerPhase::HirValidation,
                        format!(
                            "HIR assignment target name '{}' does not match variable '{}'",
                            assignment.target_name, target_variable.name
                        ),
                        assignment.span,
                    ));
                }

                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::HirValidation,
                    format!(
                        "HIR scalar assignment '{}' must not have an index",
                        assignment.target_name
                    ),
                    assignment.span,
                ));
            }
        } else if let Some(array) = array_by_name {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR array assignment '{}' must include an index",
                    array.name
                ),
                assignment.span,
            ));
        } else if assignment.target_name != target_variable.name {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR assignment target name '{}' does not match variable '{}'",
                    assignment.target_name, target_variable.name
                ),
                assignment.span,
            ));
        }
    }

    fn known_node_names(&self) -> HashSet<SmolStr> {
        let mut known = HashSet::new();
        known.insert("0".into());
        known.extend(self.ports.iter().map(|port| port.name.clone()));
        known.extend(self.internal_nodes.iter().map(|node| node.name.clone()));
        known.extend(self.ground_nodes.iter().cloned());
        known
    }

    pub(crate) fn known_value_symbol_names(&self) -> HashSet<SmolStr> {
        let mut known = HashSet::new();
        for parameter in &self.parameters {
            known.insert(parameter.name.clone());
        }
        known.extend(self.variables.iter().map(|variable| variable.name.clone()));
        known.extend(self.arrays.iter().map(|array| array.name.clone()));
        known.extend(self.ports.iter().map(|port| port.name.clone()));
        known.extend(self.internal_nodes.iter().map(|node| node.name.clone()));
        known.extend(self.ground_nodes.iter().cloned());
        known
    }

    fn declared_branch_names(&self) -> HashSet<SmolStr> {
        self.branches
            .iter()
            .filter(|branch| !branch.name.is_empty())
            .map(|branch| branch.name.clone())
            .collect()
    }
}

/// Call name given to a discrete-domain expression that reached HIR lowering.
///
/// No function registry defines it, so CFG lowering refuses it by name. It
/// exists only so that this structurally unreachable case fails closed instead
/// of silently becoming a real constant.
const UNLOWERABLE_DIGITAL_CALL: &str = "__rspice_unlowerable_digital";

const fn default_true() -> bool {
    true
}

fn validate_dense_port_ids(diagnostics: &mut Vec<IrDiagnostic>, ports: &[HirPort]) {
    for (expected, port) in ports.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR port count exceeds u32::MAX");
        if port.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR port IDs must be dense: expected PortId({}) at index {}, found {}",
                    expected, expected, port.id
                ),
            ));
        }
    }
}

fn validate_dense_parameter_ids(diagnostics: &mut Vec<IrDiagnostic>, parameters: &[HirParameter]) {
    for (expected, parameter) in parameters.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR parameter count exceeds u32::MAX");
        if parameter.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR parameter IDs must be dense: expected ParamId({}) at index {}, found {}",
                    expected, expected, parameter.id
                ),
            ));
        }
    }
}

fn validate_dense_variable_ids(diagnostics: &mut Vec<IrDiagnostic>, variables: &[HirVariable]) {
    for (expected, variable) in variables.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR variable count exceeds u32::MAX");
        if variable.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR variable IDs must be dense: expected VariableId({}) at index {}, found {}",
                    expected, expected, variable.id
                ),
            ));
        }
    }
}

fn validate_dense_array_ids(diagnostics: &mut Vec<IrDiagnostic>, arrays: &[HirArray]) {
    for (expected, array) in arrays.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR array count exceeds u32::MAX");
        if array.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR array IDs must be dense: expected ArrayId({}) at index {}, found {}",
                    expected, expected, array.id
                ),
            ));
        }
    }
}

fn validate_dense_branch_ids(diagnostics: &mut Vec<IrDiagnostic>, branches: &[HirBranch]) {
    for (expected, branch) in branches.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR branch count exceeds u32::MAX");
        if branch.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR branch IDs must be dense: expected BranchId({}) at index {}, found {}",
                    expected, expected, branch.id
                ),
            ));
        }
    }
}

fn validate_dense_contribution_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    contributions: &[HirContribution],
) {
    for (expected, contribution) in contributions.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR contribution count exceeds u32::MAX");
        if contribution.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR contribution IDs must be dense: expected ContributionId({}) at index {}, found {}",
                    expected, expected, contribution.id
                ),
            ));
        }
    }
}

fn validate_dense_internal_node_ids(
    diagnostics: &mut Vec<IrDiagnostic>,
    internal_nodes: &[HirInternalNode],
) {
    for (expected, internal_node) in internal_nodes.iter().enumerate() {
        let expected = u32::try_from(expected).expect("HIR internal node count exceeds u32::MAX");
        if internal_node.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR internal node IDs must be dense: expected NodeId({}) at index {}, found {}",
                    expected, expected, internal_node.id
                ),
            ));
        }

        if internal_node.index != internal_node.id.index() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::HirValidation,
                format!(
                    "HIR internal node index {} does not match id {}",
                    internal_node.index, internal_node.id
                ),
            ));
        }
    }
}

fn is_valid_contribution_branch(
    branch: &str,
    known_nodes: &HashSet<SmolStr>,
    declared_branches: &HashSet<SmolStr>,
) -> bool {
    if declared_branches.contains(branch) {
        return true;
    }

    if let Some((pos, neg)) = branch.split_once(',') {
        return known_nodes.contains(pos) && known_nodes.contains(neg);
    }

    known_nodes.contains(branch)
}

/// Lower one executed statement, recording the root each analog site produced.
///
/// The record is what the body lowering pairs against; it is written here
/// rather than derived later because here is the only place the site stamp and
/// the freshly minted id are both in hand.
fn lower_statement(
    lowerer: &mut HirLowerer,
    statement: &AnalyzedStatement,
    executed_sites: &mut HashMap<AnalogSiteId, ExecutedSite>,
) -> HirStatement {
    match statement {
        AnalyzedStatement::Assignment(assignment) => {
            let index = assignment
                .index
                .as_ref()
                .map(|expr| lowerer.lower_expr(expr));
            let expr = lowerer.lower_expr(&assignment.expression);
            executed_sites.insert(
                assignment.site,
                ExecutedSite {
                    value: expr.id,
                    value_guard: assignment.expression_guard,
                    index: index.as_ref().map(|index| index.id),
                },
            );
            HirStatement::Assignment(HirAssignment {
                target: VariableId::from(assignment.var_index),
                target_name: assignment.target.clone(),
                index,
                expr,
                expr_type: CanonicalValueType::from(assignment.expr_type),
                span: SourceSpanRef::from(assignment.span),
                unfiltered_initial_step_guard: assignment.unfiltered_initial_step_guard.clone(),
            })
        }
        AnalyzedStatement::Loop(loop_statement) => {
            let condition = lowerer.lower_expr(&loop_statement.condition);
            executed_sites.insert(
                loop_statement.site,
                ExecutedSite {
                    value: condition.id,
                    value_guard: loop_statement.condition_guard,
                    index: None,
                },
            );
            HirStatement::Loop(HirLoop {
                condition,
                body: loop_statement
                    .body
                    .iter()
                    .map(|statement| lower_statement(lowerer, statement, executed_sites))
                    .collect(),
                span: SourceSpanRef::from(loop_statement.span),
            })
        }
    }
}

/// Lower one region, numbering contributions in the order the analyzer met them
/// and pairing each authored expression with its executed twin.
fn lower_region(
    lowerer: &mut HirLowerer,
    next_contribution: &mut usize,
    region: &AnalyzedRegion,
    executed_sites: &HashMap<AnalogSiteId, ExecutedSite>,
    correspondence: &mut CorrespondenceBuilder,
) -> HirRegion {
    match region {
        AnalyzedRegion::Assignment(assignment) => {
            let executed = executed_sites.get(&assignment.site).copied();
            let index = assignment.index.as_ref().map(|expr| {
                let start = lowerer.expressions.len();
                let lowered = lowerer.lower_expr(expr);
                // An array index is never guarded: the fallback re-reads the
                // same element, so only the value is wrapped.
                if let Some(executed_index) = executed.and_then(|site| site.index) {
                    correspondence.pair(&lowerer.expressions, start, lowered.id, executed_index);
                }
                lowered
            });
            let start = lowerer.expressions.len();
            let expr = lowerer.lower_expr(&assignment.expression);
            pair_authored(lowerer, correspondence, executed, start, expr.id);
            HirRegion::Assignment(HirAssignment {
                target: VariableId::from(assignment.var_index),
                target_name: assignment.target.clone(),
                index,
                expr,
                expr_type: CanonicalValueType::from(assignment.expr_type),
                span: SourceSpanRef::from(assignment.span),
                unfiltered_initial_step_guard: assignment.unfiltered_initial_step_guard.clone(),
            })
        }
        AnalyzedRegion::Contribution(contribution) => {
            let id = ContributionId::from(*next_contribution);
            *next_contribution += 1;
            let executed = executed_sites.get(&contribution.site).copied();
            let start = lowerer.expressions.len();
            let expression = lowerer.lower_expr(&contribution.expression);
            pair_authored(lowerer, correspondence, executed, start, expression.id);
            HirRegion::Contribution(HirContribution {
                id,
                branch: contribution.branch.clone(),
                declared_branch: contribution.declared_branch.clone(),
                kind: contribution_kind(contribution.indirect, contribution.is_current),
                expression,
                expr_type: CanonicalValueType::from(contribution.expr_type),
                span: SourceSpanRef::from(contribution.span),
            })
        }
        AnalyzedRegion::Conditional {
            condition,
            condition_site,
            then_body,
            else_body,
            span,
        } => {
            let executed = condition_site.and_then(|site| executed_sites.get(&site).copied());
            let start = lowerer.expressions.len();
            let condition = lowerer.lower_expr(condition);
            pair_authored(lowerer, correspondence, executed, start, condition.id);
            let then_body = lower_regions(
                lowerer,
                next_contribution,
                then_body,
                executed_sites,
                correspondence,
            );
            let else_body = lower_regions(
                lowerer,
                next_contribution,
                else_body,
                executed_sites,
                correspondence,
            );
            HirRegion::Conditional {
                condition,
                then_body,
                else_body,
                span: SourceSpanRef::from(*span),
            }
        }
        AnalyzedRegion::Loop {
            condition,
            site,
            body,
            span,
        } => {
            let executed = executed_sites.get(site).copied();
            let start = lowerer.expressions.len();
            let condition = lowerer.lower_expr(condition);
            pair_authored(lowerer, correspondence, executed, start, condition.id);
            let body = lower_regions(
                lowerer,
                next_contribution,
                body,
                executed_sites,
                correspondence,
            );
            HirRegion::Loop {
                condition,
                body,
                span: SourceSpanRef::from(*span),
            }
        }
    }
}

/// Record the run pairing a just-lowered authored subtree with its executed twin.
fn pair_authored(
    lowerer: &HirLowerer,
    correspondence: &mut CorrespondenceBuilder,
    executed: Option<ExecutedSite>,
    start: usize,
    root: ExprId,
) {
    let Some(executed) = executed else { return };
    let Some(authored) = executed.authored(&lowerer.expressions) else {
        return;
    };
    correspondence.pair(&lowerer.expressions, start, root, authored);
}

fn lower_regions(
    lowerer: &mut HirLowerer,
    next_contribution: &mut usize,
    regions: &[AnalyzedRegion],
    executed_sites: &HashMap<AnalogSiteId, ExecutedSite>,
    correspondence: &mut CorrespondenceBuilder,
) -> Vec<HirRegion> {
    regions
        .iter()
        .map(|region| {
            lower_region(
                lowerer,
                next_contribution,
                region,
                executed_sites,
                correspondence,
            )
        })
        .collect()
}

#[derive(Debug, Default)]
struct HirLowerer {
    expressions: Vec<HirExpression>,
    declared_branches: HashSet<SmolStr>,
    replication_work: usize,
    next_transition_site: u32,
}

impl HirLowerer {
    fn new(declared_branches: HashSet<SmolStr>) -> Self {
        Self {
            expressions: Vec::new(),
            declared_branches,
            replication_work: 0,
            next_transition_site: 0,
        }
    }

    fn lower_expr(&mut self, expr: &Expression) -> HirExprRef {
        let kind_label = expression_kind(expr);
        let span = SourceSpanRef::from(expr.span());
        let kind = self.lower_expr_kind(expr);
        let id = ExprId::from(self.expressions.len());

        self.expressions.push(HirExpression { id, kind, span });

        HirExprRef {
            id,
            kind: kind_label,
            span,
        }
    }

    fn lower_expr_kind(&mut self, expr: &Expression) -> HirExprKind {
        match expr {
            // A discrete-domain expression has no continuous-domain lowering.
            // Semantic analysis refuses one before any module reaches HIR, so
            // this arm is unreachable through the compiler's own pipeline; it
            // lowers to a call no registry defines so that a module arriving
            // by some other route fails closed in CFG lowering rather than
            // acquiring a number it does not have.
            Expression::Digital(digital) => HirExprKind::Call {
                name: SmolStr::new(format!(
                    "{UNLOWERABLE_DIGITAL_CALL}_{}",
                    digital.construct().replace(['-', ' '], "_")
                )),
                args: Vec::new(),
            },
            Expression::NullArgument(_) => HirExprKind::NullArgument,
            Expression::Number(number) => HirExprKind::Number {
                value: number.value,
                raw: number.raw.clone(),
            },
            Expression::StringLit(string) => HirExprKind::StringLiteral {
                value: string.value.clone(),
            },
            Expression::Identifier(identifier) => {
                if let Some(value) = builtin_constant_value(identifier.name.as_str()) {
                    HirExprKind::Number {
                        value,
                        raw: identifier.name.clone(),
                    }
                } else {
                    HirExprKind::Identifier {
                        name: identifier.name.clone(),
                    }
                }
            }
            Expression::SystemFunction(function) => self
                .lower_noise_call(&function.name, &function.args)
                .unwrap_or_else(|| HirExprKind::SystemFunction {
                    name: function.name.clone(),
                    args: self.lower_expr_ids(&function.args),
                }),
            Expression::Binary(binary) => HirExprKind::Binary {
                op: format!("{:?}", binary.op).into(),
                left: self.lower_expr(&binary.left).id,
                right: self.lower_expr(&binary.right).id,
            },
            Expression::Unary(unary) => HirExprKind::Unary {
                op: format!("{:?}", unary.op).into(),
                operand: self.lower_expr(&unary.operand).id,
            },
            Expression::Conditional(conditional) => HirExprKind::Conditional {
                condition: self.lower_expr(&conditional.condition).id,
                then_expr: self.lower_expr(&conditional.then_expr).id,
                else_expr: self.lower_expr(&conditional.else_expr).id,
            },
            Expression::Call(call) => self
                .lower_noise_call(&call.name, &call.args)
                .unwrap_or_else(|| HirExprKind::Call {
                    name: call.name.clone(),
                    args: self.lower_expr_ids(&call.args),
                }),
            Expression::BranchAccess(access) => self.lower_branch_access_kind(access),
            Expression::ArrayAccess(array) => HirExprKind::ArrayAccess {
                array: array.array.clone(),
                index: self.lower_expr(&array.index).id,
            },
            Expression::ArrayLiteral(array) => HirExprKind::ArrayLiteral {
                elements: self.lower_array_literal_elements(&array.elements),
                assignment_pattern: array.assignment_pattern,
            },
            Expression::AnalogOperator(operator) => self.lower_analog_operator(operator),
            Expression::NoiseSource(source) => self.lower_noise_source(source),
        }
    }

    fn lower_expr_ids(&mut self, expressions: &[Expression]) -> Vec<ExprId> {
        expressions
            .iter()
            .map(|expr| self.lower_expr(expr).id)
            .collect()
    }

    /// Public source compilation materializes supported assignment-pattern
    /// replication in semantic analysis. This defensive path also handles a
    /// handcrafted `AnalyzedModule`: valid closed integer replication is
    /// lowered with shared child IDs, while malformed or unsafe retained
    /// replication becomes an unknown sentinel that canonical validation
    /// rejects. No untrusted AST can panic the lowering boundary.
    fn lower_array_literal_elements(&mut self, elements: &[ArrayLiteralElement]) -> Vec<ExprId> {
        self.lower_array_literal_elements_at_depth(elements, 0)
    }

    fn lower_array_literal_elements_at_depth(
        &mut self,
        elements: &[ArrayLiteralElement],
        depth: usize,
    ) -> Vec<ExprId> {
        const MAX_ELEMENTS: usize = 1_048_576;
        const MAX_WORK: usize = 4_194_304;
        const MAX_DEPTH: usize = 128;

        if depth >= MAX_DEPTH {
            return elements
                .first()
                .map(|element| vec![self.lower_invalid_replication(element.span())])
                .unwrap_or_default();
        }

        let mut lowered = Vec::new();
        for element in elements {
            match element {
                ArrayLiteralElement::Value(expression) => {
                    lowered.push(self.lower_expr(expression).id);
                }
                ArrayLiteralElement::Replication(replication) => {
                    let Some(count) = exact_retained_replication_count(&replication.count)
                        .and_then(|count| usize::try_from(count).ok())
                    else {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    };
                    if replication.elements.is_empty() {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    }
                    let body = self
                        .lower_array_literal_elements_at_depth(&replication.elements, depth + 1);
                    let Some(additional) = count.checked_mul(body.len()) else {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    };
                    let Some(projected) = lowered.len().checked_add(additional) else {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    };
                    let Some(projected_work) = self.replication_work.checked_add(additional) else {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    };
                    if projected > MAX_ELEMENTS || projected_work > MAX_WORK {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    }
                    if lowered.try_reserve(additional).is_err() {
                        lowered.push(self.lower_invalid_replication(replication.span));
                        continue;
                    }
                    self.replication_work = projected_work;
                    for _ in 0..count {
                        lowered.extend(body.iter().copied());
                    }
                }
            }
        }
        lowered
    }

    fn lower_invalid_replication(&mut self, span: crate::source::Span) -> ExprId {
        let id = ExprId::from(self.expressions.len());
        self.expressions.push(HirExpression {
            id,
            kind: HirExprKind::Identifier {
                name: "__rspice_invalid_retained_replication".into(),
            },
            span: SourceSpanRef::from(span),
        });
        id
    }

    fn lower_branch_access_expr(&mut self, access: &BranchAccess) -> ExprId {
        let id = ExprId::from(self.expressions.len());
        let span = SourceSpanRef::from(access.span());
        let kind = self.lower_branch_access_kind(access);

        self.expressions.push(HirExpression { id, kind, span });
        id
    }

    fn lower_branch_access_kind(&self, access: &BranchAccess) -> HirExprKind {
        match access {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } if neg.is_none() && self.declared_branches.contains(pos) => {
                HirExprKind::NamedBranchAccess {
                    access: access.clone(),
                    name: pos.clone(),
                }
            }
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => HirExprKind::BranchAccess {
                access: access.clone(),
                pos: pos.clone(),
                neg: neg.clone(),
            },
            BranchAccess::Branch { access, name, .. } if self.declared_branches.contains(name) => {
                HirExprKind::NamedBranchAccess {
                    access: access.clone(),
                    name: name.clone(),
                }
            }
            BranchAccess::Branch { access, name, .. } => HirExprKind::BranchAccess {
                access: access.clone(),
                pos: name.clone(),
                neg: None,
            },
        }
    }

    fn lower_optional_expr_id(&mut self, expression: &Option<Box<Expression>>) -> Option<ExprId> {
        expression.as_ref().map(|expr| self.lower_expr(expr).id)
    }

    fn lower_analog_operator(&mut self, operator: &AnalogOperator) -> HirExprKind {
        let op = match operator {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                selector,
                ..
            } => HirAnalogOperator::Limit {
                proposed: self.lower_expr(proposed).id,
                candidate: self.lower_expr(candidate).id,
                type_metadata: self.lower_optional_expr_id(type_metadata),
                selector: selector.clone(),
            },
            AnalogOperator::LimiterArgument { argument, .. } => {
                HirAnalogOperator::LimiterArgument {
                    argument: (*argument).into(),
                }
            }
            AnalogOperator::Ddt { expr, abstol, .. } => HirAnalogOperator::Ddt {
                expr: self.lower_expr(expr).id,
                abstol: self.lower_optional_expr_id(abstol),
            },
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                ..
            } => HirAnalogOperator::Idt {
                expr: self.lower_expr(expr).id,
                ic: self.lower_optional_expr_id(ic),
                assert: self.lower_optional_expr_id(assert_val),
                abstol: self.lower_optional_expr_id(abstol),
            },
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                ..
            } => HirAnalogOperator::IdtMod {
                expr: self.lower_expr(expr).id,
                ic: self.lower_optional_expr_id(ic),
                modulus: self.lower_optional_expr_id(modulus),
                offset: self.lower_optional_expr_id(offset),
                abstol: self.lower_optional_expr_id(abstol),
            },
            AnalogOperator::Ddx { expr, probe, .. } => HirAnalogOperator::Ddx {
                expr: self.lower_expr(expr).id,
                probe: self.lower_branch_access_expr(probe),
            },
            AnalogOperator::Limexp { expr, .. } => HirAnalogOperator::Limexp {
                expr: self.lower_expr(expr).id,
            },
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                ..
            } => HirAnalogOperator::Absdelay {
                expr: self.lower_expr(expr).id,
                delay: self.lower_expr(delay).id,
                max_delay: self.lower_optional_expr_id(max_delay),
            },
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                span,
            } => {
                let mut site = TransitionSiteId::from_span(*span);
                site.ordinal = self.next_transition_site;
                self.next_transition_site = self
                    .next_transition_site
                    .checked_add(1)
                    .expect("canonical transition site ordinal overflow");
                HirAnalogOperator::Transition {
                    site,
                    expr: self.lower_expr(expr).id,
                    delay: self.lower_optional_expr_id(delay),
                    rise: self.lower_optional_expr_id(rise),
                    fall: self.lower_optional_expr_id(fall),
                    tolerance: self.lower_optional_expr_id(tolerance),
                }
            }
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                ..
            } => HirAnalogOperator::Slew {
                expr: self.lower_expr(expr).id,
                max_rise: self.lower_optional_expr_id(max_rise),
                max_fall: self.lower_optional_expr_id(max_fall),
            },
            AnalogOperator::LastCrossing { expr, edge, .. } => HirAnalogOperator::LastCrossing {
                expr: self.lower_expr(expr).id,
                edge: edge.map(HirCrossDirection::from),
            },
            AnalogOperator::Laplace { kind, expr, .. } => {
                return HirExprKind::Laplace {
                    expr: self.lower_expr(expr).id,
                    kind: self.lower_laplace_kind(kind),
                };
            }
            AnalogOperator::Zi {
                kind,
                expr,
                period,
                transition,
                first_transition,
                ..
            } => {
                return HirExprKind::Zi {
                    expr: self.lower_expr(expr).id,
                    kind: self.lower_zi_kind(kind),
                    period: self.lower_expr(period).id,
                    transition: self.lower_optional_expr_id(transition),
                    first_transition: self.lower_optional_expr_id(first_transition),
                };
            }
        };

        HirExprKind::AnalogOperator { op }
    }

    fn lower_laplace_kind(&mut self, kind: &LaplaceKind) -> HirLaplaceKind {
        match kind {
            LaplaceKind::ZeroPole { zeros, poles } => HirLaplaceKind::ZeroPole {
                zeros: self.lower_expr_ids(zeros),
                poles: self.lower_expr_ids(poles),
            },
            LaplaceKind::ZeroDenominator { zeros, denominator } => {
                HirLaplaceKind::ZeroDenominator {
                    zeros: self.lower_expr_ids(zeros),
                    denominator: self.lower_expr_ids(denominator),
                }
            }
            LaplaceKind::NumeratorPole { numerator, poles } => HirLaplaceKind::NumeratorPole {
                numerator: self.lower_expr_ids(numerator),
                poles: self.lower_expr_ids(poles),
            },
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => HirLaplaceKind::NumeratorDenominator {
                numerator: self.lower_expr_ids(numerator),
                denominator: self.lower_expr_ids(denominator),
            },
        }
    }

    fn lower_zi_kind(&mut self, kind: &ZiKind) -> HirZiKind {
        match kind {
            ZiKind::ZeroPole { zeros, poles } => HirZiKind::ZeroPole {
                zeros: self.lower_expr_ids(zeros),
                poles: self.lower_expr_ids(poles),
            },
            ZiKind::ZeroDenominator { zeros, denominator } => HirZiKind::ZeroDenominator {
                zeros: self.lower_expr_ids(zeros),
                denominator: self.lower_expr_ids(denominator),
            },
            ZiKind::NumeratorPole { numerator, poles } => HirZiKind::NumeratorPole {
                numerator: self.lower_expr_ids(numerator),
                poles: self.lower_expr_ids(poles),
            },
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => HirZiKind::NumeratorDenominator {
                numerator: self.lower_expr_ids(numerator),
                denominator: self.lower_expr_ids(denominator),
            },
        }
    }

    fn lower_noise_source(&mut self, source: &NoiseSource) -> HirExprKind {
        let assigned = match source {
            NoiseSource::White { process_id, .. }
            | NoiseSource::Flicker { process_id, .. }
            | NoiseSource::Table { process_id, .. } => *process_id,
        };
        // Missing identity is malformed analyzed input. Preserve a sentinel so
        // HIR/CFG validation fails closed instead of correlating unrelated
        // sites through an independently restarted counter.
        let process_id = assigned.unwrap_or(u32::MAX);
        match source {
            NoiseSource::White { power, name, .. } => HirExprKind::NoiseSource {
                process_id,
                source: "White".into(),
                operands: vec![self.lower_expr(power).id],
                name: name.clone(),
            },
            NoiseSource::Flicker {
                power,
                exponent,
                name,
                ..
            } => HirExprKind::NoiseSource {
                process_id,
                source: "Flicker".into(),
                operands: vec![self.lower_expr(power).id, self.lower_expr(exponent).id],
                name: name.clone(),
            },
            NoiseSource::Table {
                data,
                log_interp,
                name,
                ..
            } => HirExprKind::NoiseSource {
                process_id,
                source: if *log_interp { "TableLog" } else { "Table" }.into(),
                operands: self.lower_expr_ids(data),
                name: name.clone(),
            },
        }
    }

    fn lower_noise_call(&mut self, name: &SmolStr, args: &[Expression]) -> Option<HirExprKind> {
        let normalized = name.trim_start_matches('$').to_ascii_lowercase();
        let (source, operand_count) = match normalized.as_str() {
            "white_noise" if !args.is_empty() => ("White", 1),
            "flicker_noise" if args.len() >= 2 => ("Flicker", 2),
            "noise_table" if !args.is_empty() => ("Table", 1),
            "noise_table_log" if !args.is_empty() => ("TableLog", 1),
            _ => return None,
        };
        // Executable semantic lowering canonicalizes noise calls into the
        // typed NoiseSource form. A retained raw call is malformed input and
        // must not acquire a second, independently numbered identity stream.
        let process_id = u32::MAX;
        let label = args
            .get(operand_count)
            .and_then(|expression| match expression {
                Expression::StringLit(string) => Some(string.value.clone()),
                _ => None,
            });
        let operands = if matches!(source, "Table" | "TableLog") {
            match &args[0] {
                Expression::ArrayLiteral(array) => {
                    self.lower_array_literal_elements(&array.elements)
                }
                expression => vec![self.lower_expr(expression).id],
            }
        } else {
            self.lower_expr_ids(&args[..operand_count])
        };
        Some(HirExprKind::NoiseSource {
            process_id,
            source: source.into(),
            operands,
            name: label,
        })
    }
}

fn exact_retained_replication_count(expression: &Expression) -> Option<i64> {
    let evaluate = |expression| exact_retained_replication_count(expression);
    match expression {
        Expression::Number(number) => parse_integer_literal(number.raw.as_str()).ok().flatten(),
        Expression::Unary(unary) => {
            let value = evaluate(&unary.operand)?;
            match unary.op {
                UnaryOp::Pos => Some(value),
                UnaryOp::Neg => value.checked_neg(),
                UnaryOp::Not => Some(i64::from(value == 0)),
                UnaryOp::BitNot => Some(!value),
            }
        }
        Expression::Binary(binary) => {
            let left = evaluate(&binary.left)?;
            let right = evaluate(&binary.right)?;
            match binary.op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Sub => left.checked_sub(right),
                BinaryOp::Mul => left.checked_mul(right),
                BinaryOp::Div => left.checked_div(right),
                BinaryOp::Mod => left.checked_rem(right),
                BinaryOp::Pow => left.checked_pow(u32::try_from(right).ok()?),
                BinaryOp::Eq => Some(i64::from(left == right)),
                BinaryOp::Ne => Some(i64::from(left != right)),
                BinaryOp::Lt => Some(i64::from(left < right)),
                BinaryOp::Le => Some(i64::from(left <= right)),
                BinaryOp::Gt => Some(i64::from(left > right)),
                BinaryOp::Ge => Some(i64::from(left >= right)),
                BinaryOp::And => Some(i64::from(left != 0 && right != 0)),
                BinaryOp::Or => Some(i64::from(left != 0 || right != 0)),
                BinaryOp::Shl => left.checked_shl(u32::try_from(right).ok()?),
                BinaryOp::Shr => left.checked_shr(u32::try_from(right).ok()?),
                BinaryOp::BitAnd => Some(left & right),
                BinaryOp::BitOr => Some(left | right),
                BinaryOp::BitXor => Some(left ^ right),
            }
        }
        Expression::Conditional(conditional) => {
            if evaluate(&conditional.condition)? != 0 {
                evaluate(&conditional.then_expr)
            } else {
                evaluate(&conditional.else_expr)
            }
        }
        _ => None,
    }
}

fn contribution_kind(indirect: bool, is_current: bool) -> HirContributionKind {
    if indirect {
        HirContributionKind::Indirect
    } else if is_current {
        HirContributionKind::Current
    } else {
        HirContributionKind::Potential
    }
}

fn hir_expr_kind_label(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::NullArgument => "null_argument",
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string",
        HirExprKind::Identifier { .. } => "identifier",
        HirExprKind::SystemFunction { .. } => "system_function",
        HirExprKind::Binary { .. } => "binary",
        HirExprKind::Unary { .. } => "unary",
        HirExprKind::Conditional { .. } => "conditional",
        HirExprKind::Call { .. } => "call",
        HirExprKind::BranchAccess { .. } | HirExprKind::NamedBranchAccess { .. } => "branch_access",
        HirExprKind::ArrayAccess { .. } => "array_access",
        HirExprKind::ArrayLiteral { .. } => "array_literal",
        HirExprKind::AnalogOperator { .. }
        | HirExprKind::Laplace { .. }
        | HirExprKind::Zi { .. } => "analog_operator",
        HirExprKind::NoiseSource { .. } => "noise_source",
    }
}

fn expression_kind(expr: &Expression) -> SmolStr {
    match expr {
        Expression::NullArgument(_) => "null_argument",
        Expression::Number(_) => "number",
        Expression::StringLit(_) => "string",
        Expression::Identifier(_) => "identifier",
        Expression::SystemFunction(_) => "system_function",
        Expression::Binary(_) => "binary",
        Expression::Unary(_) => "unary",
        Expression::Conditional(_) => "conditional",
        Expression::Call(_) => "call",
        Expression::BranchAccess(_) => "branch_access",
        Expression::ArrayAccess(_) => "array_access",
        Expression::ArrayLiteral(_) => "array_literal",
        Expression::AnalogOperator(_) => "analog_operator",
        Expression::NoiseSource(_) => "noise_source",
        Expression::Digital(_) => "digital",
    }
    .into()
}

const fn canonical_default_transition() -> f64 {
    1.0e-9
}

fn builtin_constant_value(name: &str) -> Option<f64> {
    match name {
        "M_PI" | "P_PI" => Some(std::f64::consts::PI),
        "M_E" | "P_E" => Some(std::f64::consts::E),
        "M_LN2" => Some(std::f64::consts::LN_2),
        "M_LN10" => Some(std::f64::consts::LN_10),
        "M_LOG2E" => Some(std::f64::consts::LOG2_E),
        "M_LOG10E" => Some(std::f64::consts::LOG10_E),
        "M_SQRT2" => Some(std::f64::consts::SQRT_2),
        "inf" => Some(f64::INFINITY),
        _ => None,
    }
}

fn port_direction_label(direction: PortDirection) -> SmolStr {
    match direction {
        PortDirection::Input => "input",
        PortDirection::Output => "output",
        PortDirection::Inout => "inout",
    }
    .into()
}
