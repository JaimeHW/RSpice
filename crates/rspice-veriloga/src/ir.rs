//! Intermediate Representation for Verilog-A
//!
//! The IR represents device equations in a form suitable for:
//! 1. Automatic differentiation (Jacobian generation)
//! 2. Code generation for MNA matrix stamping

pub mod arena;

use crate::ast::{BinaryOp, UnaryOp};
use crate::branch_identity::BranchIdentity;
use crate::error::CompileResult;
pub use crate::ir::arena::NodeId;
use crate::ir::arena::{ExprArena, Heavy, HeavyKind, IndexedRead, Node, unpack_index};
use crate::semantic::AnalyzedModule;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// Where the reaching-definition snapshots belong on a route that replays the
/// module's *statements* rather than the compiled step list.
///
/// [`crate::reaching_definition`] splices each snapshot copy into
/// [`DeviceIR::assignments`], so the VM — and every other route that executes
/// [`crate::codegen::CompiledModel::assignment_steps`] in order — needs nothing
/// here: the copy is already in the sequence, in place. The canonical route
/// does need it. It walks the canonical HIR's statements and pulls each one's
/// compiled program by variable slot, and a spliced copy has no statement to be
/// pulled by; its equation entries are lowered from the canonical IR, which
/// names the variable the author wrote rather than the snapshot. This plan says
/// which statement each copy runs after, and which read of which equation the
/// snapshot answers, which is all that route is missing.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReachingSnapshotPlan {
    /// One entry per snapshot slot, in the order the slots were allocated.
    pub copies: Vec<ReachingSnapshotCopy>,
    /// The redirected reads, one entry per equation that has any.
    pub reads: Vec<EquationSnapshotReads>,
}

impl ReachingSnapshotPlan {
    /// Whether this module allocated no snapshot — the state every module
    /// without the construct is in, and the one that leaves every route's
    /// output exactly as it was.
    pub fn is_empty(&self) -> bool {
        self.copies.is_empty() && self.reads.is_empty()
    }
}

/// One copy of a definition into the slot the equations reading it were
/// redirected to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReachingSnapshotCopy {
    /// Index, in the module's top-level statement sequence, of the definition
    /// this copy captures. The copy runs immediately after that statement, so
    /// no write to the captured slot separates the definition from the copy.
    /// `None` when no statement precedes the equations reading it.
    pub definition_statement: Option<usize>,
    /// The variable slot the copy writes.
    pub slot: usize,
}

/// The reads one equation had redirected to a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquationSnapshotReads {
    /// The equation's index among the module's contributions, which is also
    /// its stamp index and its canonical equation id.
    pub equation: usize,
    /// The name the equation was written with, and the snapshot holding the
    /// definition that reaches it. A route resolving the equation's reads by
    /// name substitutes the second for the first; a derivative shadow's name is
    /// built by appending axes to the value's, so one substitution carries the
    /// whole family.
    pub reads: Vec<(SmolStr, SmolStr)>,
}

/// Stable identity of one logical Zi operator in the source tree. The same
/// identity is retained by value and every generated Jacobian expression so
/// they share one history, candidate, clock, and breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZiSiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Deterministic preorder ordinal assigned during executable-IR
    /// construction. This disambiguates independently authored public-AST
    /// nodes that carry the same (often dummy) span.
    pub ordinal: u32,
}

impl ZiSiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Stable identity of one logical Laplace operator in the source tree. The
/// primal expression and every generated Jacobian action retain this identity
/// so bytecode lowering assigns them one shared state-space slot regardless of
/// compilation order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LaplaceSiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Deterministic preorder ordinal assigned during executable-IR
    /// construction. This disambiguates independently authored public-AST
    /// nodes that carry the same (often dummy) span.
    pub ordinal: u32,
}

impl LaplaceSiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Stable identity of one logical `slew` operator in the source tree. The
/// primal expression and its generated Jacobian action retain this identity
/// so both programs address one transactional filter candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlewSiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Deterministic preorder ordinal used to disambiguate equal spans.
    pub ordinal: u32,
}

impl SlewSiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Stable identity of one logical `transition` operator in the source tree.
/// The primal expression and its generated Jacobian action retain this
/// identity so both programs inspect the same transactional filter candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransitionSiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Deterministic preorder ordinal used to disambiguate equal spans.
    pub ordinal: u32,
}

impl TransitionSiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Stable identity of one logical `absdelay` operator in the source tree.
/// The primal expression and its generated Jacobian action retain this
/// identity so both programs address the same transactional delay buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsDelaySiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Deterministic preorder ordinal used to disambiguate equal spans.
    pub ordinal: u32,
}

impl AbsDelaySiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Stable identity of one syntactic Verilog-A noise process.  A process may
/// reach several contribution branches (for example through an assigned
/// intermediate variable); every such injection remains perfectly
/// correlated because generated derivatives retain this identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoiseSiteId {
    pub source: u32,
    pub start: u32,
    pub end: u32,
    /// Dense deterministic preorder id used by the executable noise plan.
    pub ordinal: u32,
}

impl NoiseSiteId {
    pub fn from_span(span: crate::source::Span) -> Self {
        Self {
            source: span.source.raw(),
            start: span.start,
            end: span.end,
            ordinal: 0,
        }
    }
}

/// Compiled device model in IR form
///
/// # Every expression lives in `exprs`
///
/// The assignment forest, the branch equations, their Jacobians, the noise
/// programs and the parameter programs are [`NodeId`]s into [`Self::exprs`].
/// Nothing in the front end is a boxed tree any more — the converter writes
/// into this arena directly. That is what keeps the shadow-expanded forest —
/// eighteen million nodes on `bsimcmg`, fifty-three on `psp104_nqs` — at
/// sixteen bytes a node instead of a hundred and twenty-eight, and it is what
/// makes the derivative rules' primal copies free: a rule that used to
/// `.clone()` a subtree now names it, so one arena node serves every path
/// through it.
///
/// Sharing is invisible to everything downstream because every consumer walks
/// the forest by *unfolding* it — a shared subtree is visited once per path,
/// and the emitter's per-emission state slots are allocated at the visit — so
/// the emitted program is byte for byte the one a fully copied forest emits.
/// Any pass added here inherits that obligation; `ir::arena`'s module rustdoc
/// states it in full.
#[derive(Debug, Clone)]
pub struct DeviceIR {
    /// Module name
    pub name: SmolStr,
    /// Every expression this module owns.
    ///
    /// Dropped with the IR, after [`crate::codegen::CodeGenerator`] has
    /// emitted from it.
    pub exprs: ExprArena,
    /// Terminal/port definitions
    pub terminals: Vec<Terminal>,
    /// Internal node definitions (not in port list)
    pub internal_nodes: Vec<InternalNodeDef>,
    /// Parameter definitions
    pub parameters: Vec<ParamDef>,
    /// Internal variables (state)
    pub variables: Vec<VarDef>,
    /// Sorted, duplicate-free variable slots written from event-controlled
    /// procedural bodies.
    pub event_state_variables: Vec<usize>,
    /// Sorted event-state slots holding the retained kind of each switch branch.
    /// A change from the accepted kind implies an order-zero discontinuity.
    pub switch_branch_variables: Vec<usize>,
    /// Pure localparam slots evaluated before either initialization phase.
    pub initialization_prologue_variables: Vec<usize>,
    /// Variable assignments and runtime loops (in execution order)
    pub assignments: Vec<IrAssignmentItem>,
    /// Frequency-domain assignment replay including noise-process shadows.
    /// Kept separate so DC/transient evaluation pays no process-AD overhead.
    pub noise_assignments: Vec<IrAssignmentItem>,
    /// `noise_assignments` is the ordinary pass and was not materialised.
    ///
    /// Set when the module has noise sources but no variable is noise-shadowed,
    /// which is every shipped compact model: neither the whole shadow-expanded
    /// forest — 18 M nodes and 2.1 GB on bsimcmg, held only to be handed back
    /// unchanged — nor a second emission of it is then materialised, and
    /// [`crate::codegen::CodeGenerator`] leaves the compiled noise pass empty
    /// to say so. `noise_assignments` is empty while this is set; it is
    /// non-empty only when the two passes really differ.
    pub noise_assignments_mirror_ordinary: bool,
    /// Array variables (elements are contiguous slots in `variables`)
    pub arrays: Vec<ArrayDef>,
    /// Branch equations
    pub equations: Vec<BranchEquation>,
    /// Branch-current unknowns introduced by potential contributions
    pub branch_unknowns: Vec<BranchUnknownDef>,
    /// Noise sources
    pub noise_sources: Vec<NoiseSourceDef>,
    /// Where the spliced reaching-definition copies belong for a route that
    /// replays statements rather than steps. Empty for a module that reads no
    /// reassigned variable.
    pub reaching_snapshots: ReachingSnapshotPlan,
}

/// Terminal (port) definition
#[derive(Debug, Clone)]
pub struct Terminal {
    pub name: SmolStr,
    pub index: usize,
}

/// Internal node definition (not in port list)
#[derive(Debug, Clone)]
pub struct InternalNodeDef {
    pub is_state: bool,
    pub name: SmolStr,
    pub index: usize,
}

/// Parameter definition
#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: SmolStr,
    /// Exposed through the selected module's instance-facing ABI.
    pub is_public: bool,
    /// Alternate instance-facing names (aliasparam); setting an alias
    /// writes this parameter
    pub aliases: Vec<SmolStr>,
    pub default: f64,
    /// Default expression when it does not fold to a constant (may
    /// reference previously declared parameters)
    pub default_expr: Option<NodeId>,
    pub is_integer: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_parameter: Option<SmolStr>,
    pub max_parameter: Option<SmolStr>,
    pub min_expr: Option<NodeId>,
    pub max_expr: Option<NodeId>,
    pub min_exclusive: bool,
    pub max_exclusive: bool,
    pub exclude: Vec<f64>,
    pub exclude_parameters: Vec<SmolStr>,
    pub exclude_exprs: Vec<NodeId>,
}

/// Variable definition  
#[derive(Debug, Clone)]
pub struct VarDef {
    pub name: SmolStr,
    pub is_state: bool,
}

/// Variable assignment in IR form
#[derive(Debug, Clone)]
pub struct VarAssignment {
    /// Index of variable being assigned (for indexed writes: the array's
    /// first element)
    pub var_index: usize,
    /// Runtime-indexed array element write (None for scalar targets)
    pub index: Option<IndexedTarget>,
    /// The expression to assign, in [`DeviceIR::exprs`]
    pub expr: NodeId,
}

/// Runtime-indexed array write target: the element `index - lower` of the
/// contiguous run starting at the assignment's `var_index`
#[derive(Debug, Clone)]
pub struct IndexedTarget {
    /// Array name (for diagnostics and shadow naming)
    pub array: SmolStr,
    /// Number of elements
    pub len: usize,
    /// Declared lower bound
    pub lower: i64,
    /// Element index expression (evaluated against declared bounds)
    pub index: NodeId,
}

/// Array variable layout: elements occupy contiguous variable slots
#[derive(Debug, Clone)]
pub struct ArrayDef {
    pub name: SmolStr,
    /// First element's variable index
    pub base: usize,
    /// Declared lower bound
    pub lower: i64,
    /// Number of elements
    pub len: usize,
}

/// An ordered evaluation step: a plain assignment or a runtime-bounded loop
#[derive(Debug, Clone)]
pub enum IrAssignmentItem {
    /// Single variable assignment
    Assign(VarAssignment),
    /// Loop executing its body while the condition evaluates nonzero
    Loop {
        condition: NodeId,
        body: Vec<IrAssignmentItem>,
    },
    Task(crate::analog_tasks::AnalogTaskCall<NodeId, crate::canonical_ir::SourceSpanRef>),
    Initialization {
        phase: rspice_veriloga_runtime::AnalogEvaluationPhase,
        body: Vec<IrAssignmentItem>,
    },
}

/// Branch equation: represents I(p,n) <+ f(...) or V(p,n) <+ f(...)
#[derive(Debug, Clone)]
pub struct BranchEquation {
    /// Branch identifier
    pub branch: BranchRef,
    /// Whether this contributes current (true) or voltage (false)
    pub is_current: bool,
    /// Indirect contribution: `expr` is a constraint residual driven to
    /// zero by the branch unknown; the branch row carries f(x) = 0
    /// instead of the V(p)-V(n)-E source relation
    pub indirect: bool,
    /// Potential contributions reference a branch-current unknown
    pub branch_ordinal: Option<usize>,
    /// Instance-static activation condition (parameter-only guard peeled
    /// from the contribution). None = always active.
    pub static_condition: Option<NodeId>,
    /// The expression tree, in [`DeviceIR::exprs`]
    pub expr: NodeId,
    /// Affine residual correction at the limited Newton operating point.
    pub limiter_correction: Option<NodeId>,
    /// Partial derivatives (Jacobian entries)
    pub derivatives: Vec<Derivative>,
    /// Derivatives of the reactive operand Q (where expr ~ resistive +
    /// ddt(Q)): the small-signal capacitance/inductance entries stamped
    /// as jw * dQ/dx in AC analysis
    pub reactive_derivatives: Vec<Derivative>,
}

/// A branch-current unknown introduced by potential contributions
#[derive(Debug, Clone)]
pub struct BranchUnknownDef {
    /// Scoped declared branch, or an unnamed endpoint pair.
    pub declared_name: Option<SmolStr>,
    /// Positive node (unified index)
    pub pos: usize,
    /// Negative node (unified index)
    pub neg: usize,
    /// Driven by an indirect contribution: the branch row holds the
    /// constraint equation, so the structural V(p)-V(n) row entries must
    /// not be stamped
    pub indirect: bool,
    pub equation_abstol: Option<NodeId>,
}

/// Branch reference
#[derive(Debug, Clone)]
pub struct BranchRef {
    pub pos_terminal: usize,
    pub neg_terminal: usize,
}

/// Derivative of an expression w.r.t. a variable
#[derive(Debug, Clone)]
pub struct Derivative {
    /// What we're differentiating with respect to
    pub wrt: DerivativeWrt,
    /// The derivative expression, in [`DeviceIR::exprs`]
    pub expr: NodeId,
}

/// What a derivative is with respect to
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivativeWrt {
    /// Voltage at a unified node index
    Voltage(usize),
    /// Branch-current unknown (by ordinal)
    BranchCurrent(usize),
    /// Unit-amplitude realization of one syntactic noise process.
    Noise(usize),
    /// Affine residual displacement introduced by Newton limiting.
    LimiterCorrection,
}

/// Independent solver quantity selected by a symbolic `ddx` expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdxAxis {
    Potential {
        pos: Option<usize>,
        neg: Option<usize>,
    },
    BranchCurrent {
        ordinal: usize,
        reversed: bool,
    },
}

/// Built-in functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrFunction {
    Abs,
    Sqrt,
    Exp,
    LimitedExp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Asinh,
    Acosh,
    Atanh,
    Atan2,
    Hypot,
    /// Internal derivative primitive with pairs followed by one divisor.
    SumProductsDiv,
    Floor,
    Ceil,
    Min,
    Max,
    Pow,
}

/// Frequency-interpolated PSD table (noise_table / noise_table_log)
#[derive(Debug, Clone)]
pub struct NoiseTableData {
    /// (frequency, power) points sorted by frequency
    pub points: Vec<(f64, f64)>,
    /// Interpolate in log-log coordinates
    pub log_interp: bool,
}

/// One coherent injection of a syntactic noise process into an equation.
#[derive(Debug, Clone)]
pub struct NoiseInjectionDef {
    pub branch: BranchRef,
    pub is_current: bool,
    pub branch_ordinal: Option<usize>,
    pub equation_index: usize,
    /// Complex small-signal gain from the unit process to this contribution.
    pub gain: NodeId,
}

/// One independent syntactic noise process.  Reusing its assigned value or
/// routing it through several equations adds injections here instead of
/// creating independent sources.
#[derive(Debug, Clone)]
pub struct NoiseSourceDef {
    pub site: NoiseSiteId,
    pub process_id: usize,
    /// First legacy injection fields are retained while canonical generated
    /// backends migrate to the grouped process representation.
    pub branch: BranchRef,
    pub is_current: bool,
    pub branch_ordinal: Option<usize>,
    pub equation_index: usize,
    /// Raw process power spectral density at the operating point.
    pub psd: NodeId,
    /// Flicker frequency exponent (None = white): S(f) = psd / f^exp
    pub exponent: Option<NodeId>,
    /// Frequency-interpolated PSD table; when present, `psd` carries only
    /// the amplitude-squared scale applied to the interpolated value
    pub table: Option<NoiseTableData>,
    /// Source label from the noise function's name argument
    pub name: Option<SmolStr>,
    /// All coherent circuit injections of this process.
    pub injections: Vec<NoiseInjectionDef>,
}

impl DeviceIR {
    /// Create a new device IR from an analyzed module
    ///
    /// Converts contributions to branch equations and generates
    /// Jacobian derivatives using automatic differentiation.
    /// Conversion failures are hard errors: silently dropping an equation
    /// would produce a wrong (but plausible-looking) device.
    pub fn from_analyzed(module: &AnalyzedModule) -> crate::error::CompileResult<Self> {
        Self::from_analyzed_inner(module, false)
    }

    /// Create the continuous-domain IR for a module whose discrete-domain
    /// half is executed by an external mixed-signal host.
    ///
    /// This is intentionally not the default bytecode boundary: callers must
    /// already own the canonical digital plan and its scheduler.  Dropping the
    /// digital half anywhere else would still be a silent miscompile.
    pub(crate) fn from_analyzed_mixed_analog_half(
        module: &AnalyzedModule,
    ) -> crate::error::CompileResult<Self> {
        Self::from_analyzed_inner(module, true)
    }

    fn from_analyzed_inner(
        module: &AnalyzedModule,
        mixed_host_owns_digital: bool,
    ) -> crate::error::CompileResult<Self> {
        use crate::expr_converter::{ConversionContext, ExprConverter};

        // The bytecode backend's fail-closed boundary for digital content. It
        // has no representation for a process, a net, or a continuous
        // assignment, and building an IR from the analog half alone would
        // produce a device missing the behavior the author wrote.
        if !mixed_host_owns_digital {
            crate::semantic::reject_digital_content(module)?;
        }

        if module
            .event_state_variables
            .iter()
            .any(|&slot| slot >= module.variables.len())
            || module
                .event_state_variables
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            || module
                .switch_branch_variables
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
            || module
                .switch_branch_variables
                .iter()
                .any(|slot| module.event_state_variables.binary_search(slot).is_err())
        {
            return Err(crate::error::CompileError::CodeGen(
                crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::Internal(
                    "event-state variable metadata must be sorted, unique, and within the module variable layout"
                        .into(),
                )),
            ));
        }

        if let Some(parameter) = module
            .parameters
            .iter()
            .find(|parameter| !parameter.dimensions.is_empty())
        {
            return Err(crate::error::CompileError::Semantic(
                crate::error::SemanticError::new(
                    crate::error::SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter array '{}' is represented in canonical HIR/MIR, but executable array storage and atomic instance overrides are not implemented",
                        parameter.name
                    )),
                    parameter.dimensions[0].span,
                ),
            ));
        }

        let mut ir = DeviceIR {
            name: module.name.clone(),
            exprs: ExprArena::new(),
            terminals: Vec::new(),
            internal_nodes: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
            event_state_variables: module.event_state_variables.clone(),
            switch_branch_variables: module.switch_branch_variables.clone(),
            initialization_prologue_variables: module
                .prologue_statements
                .iter()
                .filter_map(|&index| {
                    if let crate::semantic::AnalyzedStatement::Assignment(assignment) =
                        &module.statements[index]
                    {
                        Some(assignment.var_index)
                    } else {
                        None
                    }
                })
                .collect(),
            assignments: Vec::new(),
            noise_assignments: Vec::new(),
            noise_assignments_mirror_ordinary: false,
            arrays: Vec::new(),
            equations: Vec::new(),
            branch_unknowns: Vec::new(),
            noise_sources: Vec::new(),
            reaching_snapshots: ReachingSnapshotPlan::default(),
        };

        // Build terminals from ports
        for (idx, port) in module.ports.iter().enumerate() {
            ir.terminals.push(Terminal {
                name: port.name.clone(),
                index: idx,
            });
        }

        // Build internal nodes from analyzed module
        for node in &module.internal_nodes {
            ir.internal_nodes.push(InternalNodeDef {
                is_state: node.is_state,
                name: node.name.clone(),
                index: node.index,
            });
        }

        // Build parameters
        for param in &module.parameters {
            let range = param
                .range
                .clone()
                .unwrap_or_else(crate::types::ParameterRange::unrestricted);

            ir.parameters.push(ParamDef {
                name: param.name.clone(),
                is_public: param.is_public,
                aliases: Vec::new(),
                default: param.default.unwrap_or(0.0),
                default_expr: None,
                is_integer: param.param_type == crate::ast::ParamType::Integer,
                min: range.min,
                max: range.max,
                min_parameter: range.min_parameter,
                max_parameter: range.max_parameter,
                min_expr: None,
                max_expr: None,
                min_exclusive: range.min_exclusive,
                max_exclusive: range.max_exclusive,
                exclude: range.exclude,
                exclude_parameters: range.exclude_parameters,
                exclude_exprs: Vec::new(),
            });
        }

        // Attach aliasparam names to their target parameters
        for alias in &module.param_aliases {
            ir.parameters[alias.target]
                .aliases
                .push(alias.alias.clone());
        }

        // Build variables
        for var in &module.variables {
            ir.variables.push(VarDef {
                name: var.name.clone(),
                is_state: var.is_state,
            });
        }

        // Array layouts (element slots are already in `variables`).
        //
        // Ordered by name, exactly as `HirModule::from_analyzed` orders its
        // own array list. The analyzed module holds these in a `HashMap`, and
        // this list is not a lookup table: derivative shadow runs are handed
        // contiguous variable slots one array at a time in this order, so a
        // walk of the map would number the slots of one module differently on
        // every process — and hand the bytecode a different program each time.
        let mut layouts: Vec<_> = module.arrays.iter().collect();
        layouts.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        for (name, layout) in layouts {
            ir.arrays.push(ArrayDef {
                name: name.clone(),
                base: layout.base,
                lower: layout.lower,
                len: layout.len,
            });
        }

        // Create conversion context
        let ctx = ConversionContext::from_module(module);
        let converter = ExprConverter::new(&ctx);
        let num_nodes = ctx.num_nodes();

        // Compile non-constant parameter defaults. They may reference
        // previously declared parameters and are evaluated per instance,
        // in declaration order, for parameters not explicitly given.
        for (idx, param) in module.parameters.iter().enumerate() {
            if param.default.is_none()
                && let Some(default_expr) = &param.default_expr
            {
                let converted = converter.convert(&mut ir.exprs, default_expr)?;
                if !Self::is_static_param_expr(&ir.exprs, converted) {
                    return Err(crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "default of parameter '{}' must depend only on parameters",
                            param.name
                        )),
                    )
                    .into());
                }
                ir.parameters[idx].default_expr = Some(converted);
            }

            if let Some(range) = &param.range {
                let convert_range_expr = |arena: &mut ExprArena,
                                          expression: &crate::ast::Expression,
                                          label: &str|
                 -> CompileResult<NodeId> {
                    let converted = converter.convert(arena, expression)?;
                    if !Self::is_range_parameter_expr(arena, converted) {
                        return Err(crate::error::CodeGenError::new(
                            crate::error::CodeGenErrorKind::InvalidExpression(format!(
                                "{label} of parameter '{}' must depend only on parameters",
                                param.name
                            )),
                        )
                        .into());
                    }
                    Ok(converted)
                };
                let min_expr = match &range.min_expression {
                    Some(expression) => Some(convert_range_expr(
                        &mut ir.exprs,
                        expression,
                        "lower range bound",
                    )?),
                    None => None,
                };
                let max_expr = match &range.max_expression {
                    Some(expression) => Some(convert_range_expr(
                        &mut ir.exprs,
                        expression,
                        "upper range bound",
                    )?),
                    None => None,
                };
                let mut exclude_exprs = Vec::with_capacity(range.exclude_expressions.len());
                for expression in &range.exclude_expressions {
                    exclude_exprs.push(convert_range_expr(
                        &mut ir.exprs,
                        expression,
                        "excluded range value",
                    )?);
                }
                ir.parameters[idx].min_expr = min_expr;
                ir.parameters[idx].max_expr = max_expr;
                ir.parameters[idx].exclude_exprs = exclude_exprs;
            }
        }

        // Convert evaluation statements (assignments and runtime loops) to
        // IR, in order
        let span = crate::metrics::FineSpan::new("ir.statements");
        let mut items = Vec::with_capacity(module.statements.len());
        Self::convert_statements(&module.statements, &converter, &mut ir.exprs, &mut items)?;
        let mut zi_site_ordinal = 0_u32;
        let mut laplace_site_ordinal = 0_u32;
        let mut slew_site_ordinal = 0_u32;
        let mut transition_site_ordinal = 0_u32;
        let mut absdelay_site_ordinal = 0_u32;
        autodiff::assign_zi_site_ordinals_in_items(&mut ir.exprs, &mut items, &mut zi_site_ordinal);
        autodiff::assign_laplace_site_ordinals_in_items(
            &mut ir.exprs,
            &mut items,
            &mut laplace_site_ordinal,
        );
        autodiff::assign_slew_site_ordinals_in_items(
            &mut ir.exprs,
            &mut items,
            &mut slew_site_ordinal,
        );
        autodiff::assign_transition_site_ordinals_in_items(
            &mut ir.exprs,
            &mut items,
            &mut transition_site_ordinal,
        );
        autodiff::assign_absdelay_site_ordinals_in_items(
            &mut ir.exprs,
            &mut items,
            &mut absdelay_site_ordinal,
        );
        span.finish(&format!(
            "module={} statements={}",
            module.name,
            module.statements.len()
        ));

        // Pre-pass over contributions: parse branch refs and register a
        // branch-current unknown per named or unnamed branch receiving a
        // potential contribution. Repeated contributions share an unknown;
        // distinct declared branches retain independent flows.
        let mut parsed_contribs: Vec<BranchRef> = Vec::with_capacity(module.contributions.len());
        let mut branch_table: HashMap<BranchIdentity<usize>, (usize, usize)> = HashMap::new();
        for contrib in &module.contributions {
            let branch_ref = Self::parse_branch_name(&contrib.branch, &ctx).ok_or_else(|| {
                crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::InvalidExpression(
                    format!("Unknown contribution branch '{}'", contrib.branch),
                ))
            })?;

            // Potential contributions and indirect contributions (either
            // target kind) introduce a branch-current unknown
            if !contrib.is_current || contrib.indirect {
                let key = BranchIdentity::new(
                    contrib.declared_branch.as_ref(),
                    branch_ref.pos_terminal,
                    branch_ref.neg_terminal,
                );
                let ordinal = match branch_table.get(&key) {
                    Some(&(ordinal, _)) => ordinal,
                    None => {
                        let ordinal = ir.branch_unknowns.len();
                        ir.branch_unknowns.push(BranchUnknownDef {
                            declared_name: contrib.declared_branch.clone(),
                            pos: branch_ref.pos_terminal,
                            neg: branch_ref.neg_terminal,
                            indirect: contrib.indirect,
                            equation_abstol: contrib
                                .equation_abstol
                                .as_ref()
                                .map(|expression| -> CompileResult<NodeId> {
                                    let converted = converter.convert(&mut ir.exprs, expression)?;
                                    if !Self::is_range_parameter_expr(&ir.exprs, converted) {
                                        return Err(crate::error::CodeGenError::new(
                                            crate::error::CodeGenErrorKind::InvalidExpression(
                                                "equation tolerance must depend only on parameters"
                                                    .into(),
                                            ),
                                        )
                                        .into());
                                    }
                                    Ok(converted)
                                })
                                .transpose()?,
                        });
                        branch_table.insert(key.clone(), (ordinal, branch_ref.pos_terminal));
                        ordinal
                    }
                };
                // A branch is either constrained by one indirect equation
                // or driven by (summed) direct potential contributions;
                // mixing them would over-determine the unknown
                let registered_indirect = ir.branch_unknowns[ordinal].indirect;
                if registered_indirect != contrib.indirect
                    || (contrib.indirect && registered_indirect && {
                        // Second indirect contribution on the same branch
                        parsed_contribs.iter().zip(module.contributions.iter()).any(
                            |(prev_ref, prev)| {
                                prev.indirect
                                    && BranchIdentity::new(
                                        prev.declared_branch.as_ref(),
                                        prev_ref.pos_terminal,
                                        prev_ref.neg_terminal,
                                    ) == key
                            },
                        )
                    })
                {
                    return Err(crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "branch '{}' is over-determined: a branch carries either one \
                             indirect constraint or direct potential contributions, not both",
                            contrib.branch
                        )),
                    )
                    .into());
                }
            }

            parsed_contribs.push(branch_ref);
        }
        let num_branches = ir.branch_unknowns.len();

        // Convert contribution expressions exactly once.  Besides avoiding
        // duplicate stateful-operator identities, this makes the preorder
        // noise process ids assigned below identical for metadata,
        // assignment shadows, and final equation gains.
        let span = crate::metrics::FineSpan::new("ir.contributions");
        let mut converted_contribs: Vec<NodeId> = Vec::with_capacity(module.contributions.len());
        for contrib in &module.contributions {
            let arena = &mut ir.exprs;
            let mut expr = converter.convert_contribution(arena, &contrib.expression)?;
            expr = autodiff::assign_zi_site_ordinals(arena, expr, &mut zi_site_ordinal);
            expr = autodiff::assign_laplace_site_ordinals(arena, expr, &mut laplace_site_ordinal);
            expr = autodiff::assign_slew_site_ordinals(arena, expr, &mut slew_site_ordinal);
            expr = autodiff::assign_transition_site_ordinals(
                arena,
                expr,
                &mut transition_site_ordinal,
            );
            expr = autodiff::assign_absdelay_site_ordinals(arena, expr, &mut absdelay_site_ordinal);
            converted_contribs.push(expr);
        }
        span.finish(&format!(
            "module={} contributions={}",
            module.name,
            module.contributions.len()
        ));

        // Give every equation the definition that reaches the point it was
        // written at, before anything reads a contribution expression again.
        // Noise process metadata is extracted from these expressions below and
        // keeps a clone of the magnitude, so a rewrite after that point would
        // leave the PSD reading the slot the assignment pass finishes with
        // while the residual read the snapshot. The pass also splices copies
        // into `assignments`, which has to precede the static classification (a
        // snapshot of a static variable is itself static) and the shadow build
        // (which then differentiates the copies and captures the derivative
        // shadows at the same point). Its spliced items are plain variable
        // reads: no noise site, no branch probe, nothing the passes between
        // here and there look for.
        let span = crate::metrics::FineSpan::new("ir.reaching_snapshots");
        let statement_sites = module
            .statements
            .iter()
            .map(|statement| match statement {
                crate::semantic::AnalyzedStatement::Assignment(assignment) => assignment.site,
                crate::semantic::AnalyzedStatement::Task(task) => {
                    crate::semantic::AnalogSiteId(task.site)
                }
                crate::semantic::AnalyzedStatement::Loop(loop_statement) => loop_statement.site,
                crate::semantic::AnalyzedStatement::Initialization { site, .. } => *site,
            })
            .collect::<Vec<_>>();
        let equation_sites = module
            .contributions
            .iter()
            .map(|contribution| contribution.site)
            .collect::<Vec<_>>();
        ir.reaching_snapshots = crate::reaching_definition::insert_equation_snapshots(
            &mut ir.exprs,
            &mut items,
            &mut ir.variables,
            &ir.arrays,
            &statement_sites,
            &mut converted_contribs,
            &equation_sites,
        )?;
        span.finish(&format!(
            "module={} reaching_snapshots={}",
            module.name,
            ir.reaching_snapshots.copies.len()
        ));

        let span = crate::metrics::FineSpan::new("ir.noise_collect");
        Self::collect_noise_processes_in_items(&items, &mut ir.exprs, &mut ir.noise_sources)?;
        for expr in &converted_contribs {
            Self::collect_noise_processes(*expr, &mut ir.exprs, &mut ir.noise_sources)?;
        }
        ir.noise_sources.sort_by_key(|source| source.process_id);
        for (expected, source) in ir.noise_sources.iter().enumerate() {
            if source.process_id != expected {
                return Err(crate::error::CodeGenError::new(
                    crate::error::CodeGenErrorKind::Internal(format!(
                        "noise process source-order IDs are not dense: expected {expected}, found {}",
                        source.process_id
                    )),
                )
                .into());
            }
        }
        span.finish(&format!(
            "module={} processes={}",
            module.name,
            ir.noise_sources.len()
        ));

        // There is no seam any more: the converter wrote these nodes into
        // `ir.exprs` directly, so the statement list is already the arena's and
        // the primal forest was never boxed. `ir.exprs.len()` here is the same
        // count the import span used to report, and it is still worth a line —
        // it is the primal forest against which the shadow build's expansion is
        // read.
        let span = crate::metrics::FineSpan::new("ir.primal_forest");
        ir.assignments = items;
        span.finish(&format!("module={} nodes={}", module.name, ir.exprs.len()));

        // Variables that are fixed per instance (computed purely from
        // parameters) may participate in topology guards
        let span = crate::metrics::FineSpan::new("ir.static_vars");
        let static_vars =
            Self::compute_instance_static_vars(&ir.exprs, &ir.assignments, &ir.variables);
        span.finish(&format!(
            "module={} static={}",
            module.name,
            static_vars.len()
        ));

        // Forward AD carries precisely the derivative orders read by the
        // equations and by ddx, including reads through mutable assignments.
        // Check the small primal arena before physical shadows expand it.
        let has_limiters = ir.exprs.has_limiters();
        let mut shadow_roots = HashSet::new();
        for &expr in &converted_contribs {
            autodiff::collect_var_names(&ir.exprs, expr, &mut shadow_roots);
        }
        autodiff::collect_ddx_operand_names(&ir.exprs, &ir.assignments, &mut shadow_roots);
        let span = crate::metrics::FineSpan::new("ir.shadow_assignments");
        let mut shadows = autodiff::build_shadow_assignments(
            &mut ir,
            num_nodes,
            num_branches,
            &converted_contribs,
        )?;
        span.finish(&format!(
            "module={} shadow_variables={}",
            module.name,
            ir.variables.len()
        ));
        let span = crate::metrics::FineSpan::new("ir.noise_shadow_assignments");
        if !ir.noise_sources.is_empty() {
            let noise_process_count = ir.noise_sources.len();
            // Decide before cloning anything: the clone of the shadow-expanded
            // forest is the single largest allocation of the whole compile, and
            // on a module with nothing noise-shadowed it was made only to be
            // handed back untouched.
            let shadowed = autodiff::auxiliary_shadowed_dependencies(
                &mut ir,
                autodiff::AuxiliaryAxes::Noise(noise_process_count),
                &shadow_roots,
            );
            if shadowed.is_empty() {
                ir.noise_assignments_mirror_ordinary = true;
            } else {
                let ordinary_assignments = ir.assignments.clone();
                autodiff::build_auxiliary_shadow_assignments(
                    &mut ir,
                    shadowed,
                    &mut shadows,
                    autodiff::AuxiliaryAxes::Noise(noise_process_count),
                );
                ir.noise_assignments = std::mem::replace(&mut ir.assignments, ordinary_assignments);
            }
        }
        span.finish(&format!(
            "module={} processes={}",
            module.name,
            ir.noise_sources.len()
        ));

        // Resolve ddx() operators now that the shadow context exists
        let span = crate::metrics::FineSpan::new("ir.resolve_ddx");
        let mut assignments = std::mem::take(&mut ir.assignments);
        autodiff::resolve_ddx_in_items(&mut ir.exprs, &mut assignments, &shadows);
        ir.assignments = assignments;
        span.finish(&format!(
            "module={} assignments={}",
            module.name,
            ir.assignments.len()
        ));
        let span = crate::metrics::FineSpan::new("ir.resolve_ddx_noise");
        let mut noise_assignments = std::mem::take(&mut ir.noise_assignments);
        autodiff::resolve_ddx_in_items(&mut ir.exprs, &mut noise_assignments, &shadows);
        ir.noise_assignments = noise_assignments;
        span.finish(&format!(
            "module={} assignments={}",
            module.name,
            ir.noise_assignments.len()
        ));

        for expr in &mut converted_contribs {
            *expr = autodiff::resolve_ddx(&mut ir.exprs, *expr, &shadows);
        }
        // A ddx result can read physical derivative shadows. Resolve it first
        // so the displacement pass roots those variables as well as primals.
        if has_limiters {
            let mut roots = HashSet::new();
            for &expr in &converted_contribs {
                autodiff::collect_var_names(&ir.exprs, expr, &mut roots);
            }
            let family = autodiff::AuxiliaryAxes::LimiterCorrection;
            let shadowed = autodiff::auxiliary_shadowed_dependencies(&mut ir, family, &roots);
            autodiff::build_auxiliary_shadow_assignments(&mut ir, shadowed, &mut shadows, family);
        }

        // Convert contributions to equations
        let equation_span = crate::metrics::FineSpan::new("ir.equations");
        let mut derivative_elapsed = std::time::Duration::ZERO;
        let mut reactive_elapsed = std::time::Duration::ZERO;
        let mut noise_gain_elapsed = std::time::Duration::ZERO;
        // The arena and the three lists this loop writes are borrowed apart:
        // a noise gain is differentiated into `exprs` while `noise_sources` is
        // being extended, which one borrow of the whole IR cannot express.
        let DeviceIR {
            exprs,
            equations: out_equations,
            noise_sources,
            branch_unknowns,
            ..
        } = &mut ir;
        for ((contrib, branch_ref), expr) in module
            .contributions
            .iter()
            .zip(parsed_contribs)
            .zip(converted_contribs)
        {
            // Peel instance-static guards (parameter expressions or
            // variables derived purely from parameters): a potential
            // contribution that is mode-disabled must leave the branch
            // open, not short it to zero volts.
            let (static_condition, expr) = Self::peel_static_condition(exprs, expr, &static_vars);

            let (branch_ref, expr, branch_ordinal) = if contrib.indirect {
                // Constraint equations are orientation-free (f == g holds
                // whichever way the target was written); the KCL couplings
                // use the unknown's registered orientation
                let key = BranchIdentity::new(
                    contrib.declared_branch.as_ref(),
                    branch_ref.pos_terminal,
                    branch_ref.neg_terminal,
                );
                let (ordinal, _) = branch_table[&key];
                let unknown = &branch_unknowns[ordinal];
                (
                    BranchRef {
                        pos_terminal: unknown.pos,
                        neg_terminal: unknown.neg,
                    },
                    expr,
                    Some(ordinal),
                )
            } else if contrib.is_current {
                (branch_ref, expr, None)
            } else {
                let key = BranchIdentity::new(
                    contrib.declared_branch.as_ref(),
                    branch_ref.pos_terminal,
                    branch_ref.neg_terminal,
                );
                let (ordinal, oriented_pos) = branch_table[&key];
                if branch_ref.pos_terminal == oriented_pos {
                    (branch_ref, expr, Some(ordinal))
                } else {
                    // Reversed orientation: V(b,a) <+ E is V(a,b) <+ -E
                    let unknown = &branch_unknowns[ordinal];
                    let branch = BranchRef {
                        pos_terminal: unknown.pos,
                        neg_terminal: unknown.neg,
                    };
                    (
                        branch,
                        exprs.push(Node::Unary(UnaryOp::Neg, expr)),
                        Some(ordinal),
                    )
                }
            };

            // Generate derivatives for Jacobian (over node voltages and
            // branch-current unknowns)
            let span = crate::metrics::FineSpan::new("ir.equation_derivatives");
            let derivatives =
                Self::generate_derivatives(exprs, expr, num_nodes, num_branches, &shadows);
            derivative_elapsed += span.elapsed();

            let limiter_correction = has_limiters
                .then(|| {
                    let correction = autodiff::differentiate_with_shadows(
                        exprs,
                        expr,
                        &DerivativeWrt::LimiterCorrection,
                        &shadows,
                    );
                    autodiff::simplify(exprs, correction)
                })
                .filter(|&correction| !Self::is_zero(exprs, correction));

            // Reactive (charge/flux) derivatives for AC analysis: extract
            // the ddt() operand and differentiate it
            let span = crate::metrics::FineSpan::new("ir.equation_reactive");
            let reactive_derivatives = match Self::extract_charge(exprs, expr) {
                Some(charge) => {
                    Self::generate_derivatives(exprs, charge, num_nodes, num_branches, &shadows)
                }
                None => Vec::new(),
            };
            reactive_elapsed += span.elapsed();

            // Extract small-signal noise sources (white_noise /
            // flicker_noise terms) for noise analysis; they evaluate to
            // zero in the large-signal programs
            let equation_index = out_equations.len();
            let span = crate::metrics::FineSpan::new("ir.equation_noise_gains");
            for process in noise_sources.iter_mut() {
                let gain = autodiff::differentiate_with_shadows(
                    exprs,
                    expr,
                    &DerivativeWrt::Noise(process.process_id),
                    &shadows,
                );
                if Self::is_zero(exprs, gain) {
                    continue;
                }
                let injection = NoiseInjectionDef {
                    branch: branch_ref.clone(),
                    is_current: contrib.is_current,
                    branch_ordinal,
                    equation_index,
                    gain,
                };
                if process.injections.is_empty() {
                    process.branch = injection.branch.clone();
                    process.is_current = injection.is_current;
                    process.branch_ordinal = injection.branch_ordinal;
                    process.equation_index = injection.equation_index;
                }
                process.injections.push(injection);
            }
            noise_gain_elapsed += span.elapsed();

            out_equations.push(BranchEquation {
                branch: branch_ref,
                is_current: contrib.is_current,
                indirect: contrib.indirect,
                branch_ordinal,
                static_condition,
                expr,
                limiter_correction,
                derivatives,
                reactive_derivatives,
            });
        }
        equation_span.finish(&format!(
            "module={} equations={}",
            module.name,
            ir.equations.len()
        ));
        let detail = format!("module={}", module.name);
        crate::metrics::report_fine_span("ir.equation_derivatives", derivative_elapsed, &detail);
        crate::metrics::report_fine_span("ir.equation_reactive", reactive_elapsed, &detail);
        crate::metrics::report_fine_span("ir.equation_noise_gains", noise_gain_elapsed, &detail);

        Ok(ir)
    }

    fn collect_noise_processes_in_items(
        items: &[IrAssignmentItem],
        arena: &mut ExprArena,
        out: &mut Vec<NoiseSourceDef>,
    ) -> CompileResult<()> {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { .. } => {}
                IrAssignmentItem::Task(task) => {
                    for &expression in task.expressions() {
                        Self::collect_noise_processes(expression, arena, out)?;
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    Self::collect_noise_processes(assignment.expr, arena, out)?;
                }
                IrAssignmentItem::Loop { condition, body } => {
                    Self::collect_noise_processes(*condition, arena, out)?;
                    Self::collect_noise_processes_in_items(body, arena, out)?;
                }
            }
        }
        Ok(())
    }

    fn collect_noise_processes(
        expr: NodeId,
        arena: &mut ExprArena,
        out: &mut Vec<NoiseSourceDef>,
    ) -> CompileResult<()> {
        let mut definitions = Vec::new();
        autodiff::collect_noise_definitions(arena, expr, &mut definitions);
        for (site, psd, exponent, table, name) in definitions {
            let process_id = site.ordinal as usize;
            // A `noise_table` publishes the constant one; every other process
            // publishes the magnitude the converter built.
            let psd = match psd {
                Some(psd) => psd,
                None => arena.push(Node::Const(1.0)),
            };
            out.push(NoiseSourceDef {
                site,
                process_id,
                branch: BranchRef {
                    pos_terminal: usize::MAX,
                    neg_terminal: usize::MAX,
                },
                is_current: true,
                branch_ordinal: None,
                equation_index: 0,
                psd,
                exponent,
                table,
                name,
                injections: Vec::new(),
            });
        }
        Ok(())
    }

    /// Extract the reactive operand of a contribution: for
    /// expr ~ resistive + ddt(Q), returns Q. Returns None when no ddt()
    /// is present.
    ///
    /// ddt() results must combine linearly per the LRM; sums, differences,
    /// negation, guards, and ddt-free multiplicative factors fold into Q
    /// (a bias-dependent factor f folds as f*Q, the quasi-static
    /// approximation: at the operating point dq/dt = 0, so the factor's
    /// own derivative carries no small-signal current).
    fn extract_charge(arena: &mut ExprArena, expr: NodeId) -> Option<NodeId> {
        fn contains_ddt_opt(arena: &ExprArena, slot: Option<NodeId>) -> bool {
            slot.is_some_and(|child| contains_ddt(arena, child))
        }

        /// The heavy operators' slots, exactly as the boxed walk read them —
        /// including where it is asymmetric (a primal `transition` looks only
        /// at its input, its derivative at every rate operand).
        fn contains_ddt_heavy(arena: &ExprArena, heavy: &Heavy) -> bool {
            match heavy {
                Heavy::IntegralDerivative {
                    primal,
                    input_derivative,
                    ic_derivative,
                    modulus_derivative,
                } => {
                    contains_ddt(arena, *primal)
                        || contains_ddt(arena, *input_derivative)
                        || contains_ddt(arena, *ic_derivative)
                        || contains_ddt_opt(arena, *modulus_derivative)
                }
                Heavy::AbsDelay {
                    expr,
                    delay_time,
                    max_delay,
                    ..
                } => {
                    contains_ddt(arena, *expr)
                        || contains_ddt(arena, *delay_time)
                        || contains_ddt_opt(arena, *max_delay)
                }
                Heavy::AbsDelayDerivative {
                    input,
                    input_derivative,
                    delay_time,
                    delay_derivative,
                    max_delay,
                    ..
                } => {
                    contains_ddt(arena, *input)
                        || contains_ddt(arena, *input_derivative)
                        || contains_ddt(arena, *delay_time)
                        || contains_ddt(arena, *delay_derivative)
                        || contains_ddt_opt(arena, *max_delay)
                }
                Heavy::Transition { expr, .. }
                | Heavy::LaplaceZP { expr, .. }
                | Heavy::LaplaceND { expr, .. }
                | Heavy::LaplaceZPDerivative { expr, .. }
                | Heavy::LaplaceNDDerivative { expr, .. }
                | Heavy::ZiFilter { expr, .. }
                | Heavy::ZiFilterDerivative { expr, .. } => contains_ddt(arena, *expr),
                Heavy::TransitionDerivative {
                    input,
                    input_derivative,
                    delay,
                    rise_time,
                    fall_time,
                    ..
                } => {
                    contains_ddt(arena, *input)
                        || contains_ddt(arena, *input_derivative)
                        || contains_ddt_opt(arena, *delay)
                        || contains_ddt_opt(arena, *rise_time)
                        || contains_ddt_opt(arena, *fall_time)
                }
                Heavy::Slew {
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                    ..
                } => {
                    contains_ddt(arena, *expr)
                        || contains_ddt_opt(arena, *max_pos_slew)
                        || contains_ddt_opt(arena, *max_neg_slew)
                }
                Heavy::SlewDerivative {
                    input,
                    input_derivative,
                    max_pos_slew,
                    max_pos_slew_derivative,
                    max_neg_slew,
                    max_neg_slew_derivative,
                    ..
                } => {
                    contains_ddt(arena, *input)
                        || contains_ddt(arena, *input_derivative)
                        || contains_ddt_opt(arena, *max_pos_slew)
                        || contains_ddt_opt(arena, *max_pos_slew_derivative)
                        || contains_ddt_opt(arena, *max_neg_slew)
                        || contains_ddt_opt(arena, *max_neg_slew_derivative)
                }
                Heavy::Cross {
                    expr,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                } => {
                    contains_ddt(arena, *expr)
                        || contains_ddt_opt(arena, *direction)
                        || contains_ddt_opt(arena, *time_tol)
                        || contains_ddt_opt(arena, *expr_tol)
                        || contains_ddt_opt(arena, *enable)
                }
                Heavy::Above {
                    expr,
                    time_tol,
                    expr_tol,
                    enable,
                } => {
                    contains_ddt(arena, *expr)
                        || contains_ddt_opt(arena, *time_tol)
                        || contains_ddt_opt(arena, *expr_tol)
                        || contains_ddt_opt(arena, *enable)
                }
                Heavy::Timer {
                    start_time,
                    period,
                    time_tol,
                    enable,
                } => {
                    contains_ddt(arena, *start_time)
                        || contains_ddt_opt(arena, *period)
                        || contains_ddt_opt(arena, *time_tol)
                        || contains_ddt_opt(arena, *enable)
                }
                Heavy::WhiteNoise { power, .. } => contains_ddt(arena, *power),
                Heavy::FlickerNoise {
                    power, exponent, ..
                } => contains_ddt(arena, *power) || contains_ddt(arena, *exponent),
                Heavy::NoiseTable { .. } => false,
            }
        }

        fn contains_ddt(arena: &ExprArena, id: NodeId) -> bool {
            match *arena.node(id) {
                Node::FreezeDerivative(inner) => contains_ddt(arena, inner),
                Node::Ddt(_) | Node::DdtDerivative { .. } => true,
                Node::Binary(_, left, right) => {
                    contains_ddt(arena, left) || contains_ddt(arena, right)
                }
                Node::Unary(_, inner)
                | Node::Limexp(inner)
                | Node::IdtCompanion(inner)
                | Node::LimiterPrevious(inner) => contains_ddt(arena, inner),
                Node::NamedLimit {
                    proposed,
                    candidate,
                } => contains_ddt(arena, proposed) || contains_ddt(arena, candidate),
                Node::Idt(inner, second) | Node::Limit(inner, second) => {
                    contains_ddt(arena, inner) || contains_ddt_opt(arena, second)
                }
                Node::IdtMod {
                    expr,
                    modulus,
                    payload,
                } => {
                    let (ic, offset) = arena.optional_pair(payload);
                    contains_ddt(arena, expr)
                        || contains_ddt_opt(arena, ic)
                        || contains_ddt(arena, modulus)
                        || contains_ddt_opt(arena, offset)
                }
                Node::Call { a, b, .. } => contains_ddt_opt(arena, a) || contains_ddt_opt(arena, b),
                Node::CallSpilled { args, .. } => arena
                    .call_args(args)
                    .iter()
                    .any(|arg| contains_ddt(arena, *arg)),
                Node::Conditional(condition, then_expr, else_expr) => {
                    contains_ddt(arena, condition)
                        || contains_ddt(arena, then_expr)
                        || contains_ddt(arena, else_expr)
                }
                Node::TableLookup { input, .. } | Node::TableDerivative { input, .. } => {
                    contains_ddt(arena, input)
                }
                Node::Ddx { expr, .. } => contains_ddt(arena, expr),
                Node::LastCrossing { expr, direction } => {
                    contains_ddt(arena, expr) || contains_ddt_opt(arena, direction)
                }
                // ddt() cannot appear in an element index (assignments
                // reject it upstream), so an indexed read is resistive
                Node::VarIndexed { index, .. } => contains_ddt(arena, index),
                Node::Heavy(_, heavy) => contains_ddt_heavy(arena, arena.heavy(heavy)),
                Node::Const(_)
                | Node::Param(_)
                | Node::ParamGiven(_)
                | Node::Var(_)
                | Node::Voltage(..)
                | Node::Current(..)
                | Node::BranchCurrent(_)
                | Node::Time
                | Node::Temperature
                | Node::Vt
                | Node::SimParamValue(_)
                | Node::SimParamPresent(_)
                | Node::Mfactor
                | Node::PortConnected(_)
                | Node::Analysis(_) => false,
            }
        }

        match *arena.node(expr) {
            Node::Ddt(charge) => Some(charge),
            Node::Binary(op @ (BinaryOp::Add | BinaryOp::Sub), left, right) => {
                let left_charge = Self::extract_charge(arena, left);
                let right_charge = Self::extract_charge(arena, right);
                if left_charge.is_none() && right_charge.is_none() {
                    return None;
                }
                let left_charge = match left_charge {
                    Some(charge) => charge,
                    None => arena.push(Node::Const(0.0)),
                };
                let right_charge = match right_charge {
                    Some(charge) => charge,
                    None => arena.push(Node::Const(0.0)),
                };
                Some(arena.push(Node::Binary(op, left_charge, right_charge)))
            }
            Node::Binary(BinaryOp::Mul, left, right) => {
                match (contains_ddt(arena, left), contains_ddt(arena, right)) {
                    (false, false) => None,
                    (false, true) => {
                        let charge = Self::extract_charge(arena, right)?;
                        let left = arena.push(Node::FreezeDerivative(left));
                        Some(arena.push(Node::Binary(BinaryOp::Mul, left, charge)))
                    }
                    (true, false) => {
                        let charge = Self::extract_charge(arena, left)?;
                        let right = arena.push(Node::FreezeDerivative(right));
                        Some(arena.push(Node::Binary(BinaryOp::Mul, charge, right)))
                    }
                    (true, true) => {
                        log::warn!(
                            "ddt() on both sides of a product; reactive AC \
                             contribution omitted"
                        );
                        None
                    }
                }
            }
            Node::Binary(BinaryOp::Div, left, right) if !contains_ddt(arena, right) => {
                let charge = Self::extract_charge(arena, left)?;
                let right = arena.push(Node::FreezeDerivative(right));
                Some(arena.push(Node::Binary(BinaryOp::Div, charge, right)))
            }
            Node::Unary(op @ (UnaryOp::Neg | UnaryOp::Pos), inner) => {
                let charge = Self::extract_charge(arena, inner)?;
                Some(arena.push(Node::Unary(op, charge)))
            }
            Node::Conditional(condition, then_expr, else_expr) => {
                let then_charge = Self::extract_charge(arena, then_expr);
                let else_charge = Self::extract_charge(arena, else_expr);
                if then_charge.is_none() && else_charge.is_none() {
                    return None;
                }
                let then_charge = match then_charge {
                    Some(charge) => charge,
                    None => arena.push(Node::Const(0.0)),
                };
                let else_charge = match else_charge {
                    Some(charge) => charge,
                    None => arena.push(Node::Const(0.0)),
                };
                Some(arena.push(Node::Conditional(condition, then_charge, else_charge)))
            }
            _ => {
                if contains_ddt(arena, expr) {
                    log::warn!(
                        "ddt() inside an unsupported expression shape; its \
                         reactive contribution is omitted from AC analysis"
                    );
                }
                None
            }
        }
    }

    /// Peel leading instance-static guards (`cond ? inner : 0` where cond
    /// is fixed per instance) into a separate activation condition
    fn peel_static_condition(
        arena: &mut ExprArena,
        expr: NodeId,
        static_vars: &HashSet<SmolStr>,
    ) -> (Option<NodeId>, NodeId) {
        let mut condition: Option<NodeId> = None;
        let mut current = expr;
        loop {
            match *arena.node(current) {
                Node::Conditional(cond, then_expr, else_expr)
                    if Self::is_instance_static_expr(arena, cond, static_vars)
                        && matches!(*arena.node(else_expr), Node::Const(v) if v == 0.0) =>
                {
                    condition = Some(match condition {
                        Some(prev) => arena.push(Node::Binary(BinaryOp::And, prev, cond)),
                        None => cond,
                    });
                    current = then_expr;
                }
                _ => return (condition, current),
            }
        }
    }

    /// Convert analyzed statements (assignments and runtime loops) to IR
    fn convert_statements(
        statements: &[crate::semantic::AnalyzedStatement],
        converter: &crate::expr_converter::ExprConverter,
        arena: &mut ExprArena,
        out: &mut Vec<IrAssignmentItem>,
    ) -> crate::error::CompileResult<()> {
        use crate::semantic::AnalyzedStatement;
        for stmt in statements {
            match stmt {
                AnalyzedStatement::Initialization { phase, body, .. } => {
                    let mut converted = Vec::new();
                    Self::convert_statements(body, converter, arena, &mut converted)?;
                    out.push(IrAssignmentItem::Initialization {
                        phase: *phase,
                        body: converted,
                    });
                }
                AnalyzedStatement::Task(task) => out.push(IrAssignmentItem::Task(task.try_map(
                    crate::canonical_ir::SourceSpanRef::from(task.span),
                    |expression| converter.convert(arena, expression),
                )?)),
                AnalyzedStatement::Assignment(assign) => {
                    let expr = converter.convert(arena, &assign.expression)?;
                    let index = match &assign.index {
                        Some(index_expr) => {
                            let (_base, lower, len) =
                                converter.array_layout(&assign.target).ok_or_else(|| {
                                    crate::error::CodeGenError::new(
                                        crate::error::CodeGenErrorKind::Internal(format!(
                                            "indexed assignment to unknown array '{}'",
                                            assign.target
                                        )),
                                    )
                                })?;
                            Some(IndexedTarget {
                                array: assign.target.clone(),
                                len,
                                lower,
                                index: converter.convert(arena, index_expr)?,
                            })
                        }
                        None => None,
                    };
                    out.push(IrAssignmentItem::Assign(VarAssignment {
                        var_index: assign.var_index,
                        index,
                        expr,
                    }));
                }
                AnalyzedStatement::Loop(loop_stmt) => {
                    let condition = converter.convert(arena, &loop_stmt.condition)?;
                    let mut body = Vec::with_capacity(loop_stmt.body.len());
                    Self::convert_statements(&loop_stmt.body, converter, arena, &mut body)?;
                    out.push(IrAssignmentItem::Loop { condition, body });
                }
            }
        }
        Ok(())
    }

    /// Parse a branch name string like "p,n" or "p" to node indices
    fn parse_branch_name(
        branch_name: &str,
        ctx: &crate::expr_converter::ConversionContext,
    ) -> Option<BranchRef> {
        let parts: Vec<&str> = branch_name.split(',').collect();

        let pos_name = parts.first()?.trim();
        let pos_idx = ctx.node_index(pos_name)?;

        let neg_idx = if parts.len() > 1 {
            // An unknown negative node is an error, not silently ground
            ctx.node_index(parts[1].trim())?
        } else {
            ctx.ground()
        };

        Some(BranchRef {
            pos_terminal: pos_idx,
            neg_terminal: neg_idx,
        })
    }

    /// Generate derivatives for Jacobian entries over the unified node
    /// space (terminals, internal nodes) and the branch-current unknowns
    fn generate_derivatives(
        arena: &mut ExprArena,
        expr: NodeId,
        num_nodes: usize,
        num_branches: usize,
        shadows: &autodiff::ShadowContext,
    ) -> Vec<Derivative> {
        let mut derivatives = Vec::new();
        let active_axes = autodiff::expression_axes(arena, expr, shadows, num_nodes);
        if active_axes == 0 {
            return derivatives;
        }

        for wrt in autodiff::axes(num_nodes, num_branches) {
            if !autodiff::mask_contains_axis(active_axes, &wrt, num_nodes) {
                continue;
            }
            let simplified = autodiff::differentiate_with_shadows(arena, expr, &wrt, shadows);

            // Only add non-zero derivatives
            if !Self::is_zero(arena, simplified) {
                derivatives.push(Derivative {
                    wrt,
                    expr: simplified,
                });
            }
        }

        derivatives
    }

    /// Check if an expression is zero (constant 0.0)
    fn is_zero(arena: &ExprArena, expr: NodeId) -> bool {
        matches!(*arena.node(expr), Node::Const(value) if value == 0.0)
    }

    /// Check whether an expression depends only on parameters and constants
    /// (valid for instance-time parameter default evaluation)
    ///
    /// It is [`Self::is_instance_static_expr_with_options`] with no static
    /// variables and no `$analysis`, written out because that is the only shape
    /// it is ever called in.
    fn is_static_param_expr(arena: &ExprArena, expr: NodeId) -> bool {
        let recurse = |child| Self::is_static_param_expr(arena, child);
        match *arena.node(expr) {
            Node::Const(_)
            | Node::Param(_)
            | Node::ParamGiven(_)
            | Node::Temperature
            | Node::Vt
            | Node::SimParamValue(_)
            | Node::SimParamPresent(_)
            | Node::Mfactor
            | Node::PortConnected(_) => true,
            // No variable is instance-static here, so an element read is
            // static only where there is no element to read.
            Node::VarIndexed { payload, index } => {
                arena.indexed(payload).len == 0 && recurse(index)
            }
            Node::Binary(_, left, right) => recurse(left) && recurse(right),
            Node::Unary(_, operand) | Node::Limexp(operand) => recurse(operand),
            Node::Call { a, b, .. } => a.is_none_or(&recurse) && b.is_none_or(&recurse),
            Node::CallSpilled { args, .. } => arena.call_args(args).iter().all(|arg| recurse(*arg)),
            Node::Conditional(condition, then_expr, else_expr) => {
                recurse(condition) && recurse(then_expr) && recurse(else_expr)
            }
            _ => false,
        }
    }

    /// Range constraints are evaluated during instance setup and therefore
    /// may only read final parameter values and pure numeric expressions.
    fn is_range_parameter_expr(arena: &ExprArena, expr: NodeId) -> bool {
        let recurse = |child| Self::is_range_parameter_expr(arena, child);
        match *arena.node(expr) {
            Node::Const(_) | Node::Param(_) => true,
            Node::Binary(_, left, right) => recurse(left) && recurse(right),
            Node::Unary(_, operand) | Node::Limexp(operand) => recurse(operand),
            Node::Call { a, b, .. } => a.is_none_or(&recurse) && b.is_none_or(&recurse),
            Node::CallSpilled { args, .. } => arena.call_args(args).iter().all(|arg| recurse(*arg)),
            Node::Conditional(condition, then_expr, else_expr) => {
                recurse(condition) && recurse(then_expr) && recurse(else_expr)
            }
            _ => false,
        }
    }

    /// Check whether an expression is fixed per instance: it depends only
    /// on parameters, constants, temperature, analysis type, and variables
    /// proven instance-static. Such expressions may gate device topology.
    fn is_instance_static_expr(
        arena: &ExprArena,
        expr: NodeId,
        static_vars: &HashSet<SmolStr>,
    ) -> bool {
        Self::is_instance_static_expr_with_options(arena, expr, static_vars, true)
    }

    fn is_instance_static_expr_with_options(
        arena: &ExprArena,
        expr: NodeId,
        static_vars: &HashSet<SmolStr>,
        allow_analysis: bool,
    ) -> bool {
        let recurse = |e: NodeId| {
            Self::is_instance_static_expr_with_options(arena, e, static_vars, allow_analysis)
        };
        match *arena.node(expr) {
            Node::Const(_)
            | Node::Param(_)
            | Node::ParamGiven(_)
            | Node::Temperature
            | Node::Vt
            | Node::Mfactor
            | Node::PortConnected(_) => true,
            Node::Var(name) => static_vars.contains(arena.name(name).as_str()),
            // An indexed read is static when the index is static and every
            // element it could select is static
            Node::VarIndexed { payload, index } => {
                let read = *arena.indexed(payload);
                let array = arena.name(read.array);
                recurse(index)
                    && (read.lower..read.lower + read.len as i64)
                        .all(|k| static_vars.contains(format!("{array}[{k}]").as_str()))
            }
            Node::Binary(_, left, right) => recurse(left) && recurse(right),
            Node::Unary(_, operand) | Node::Limexp(operand) => recurse(operand),
            Node::Call { a, b, .. } => a.is_none_or(&recurse) && b.is_none_or(&recurse),
            Node::CallSpilled { args, .. } => arena.call_args(args).iter().all(|arg| recurse(*arg)),
            Node::Conditional(condition, then_expr, else_expr) => {
                recurse(condition) && recurse(then_expr) && recurse(else_expr)
            }
            Node::Analysis(_) => allow_analysis,
            _ => false,
        }
    }

    /// Fixpoint over the assignment tree: a variable is instance-static if
    /// every assignment to it uses only parameters, constants, and other
    /// instance-static variables. These variables hold the same value for
    /// every evaluation of a given instance (BSIM4's mode selectors like
    /// BSIM4rdsMod), so guards built from them may gate topology.
    fn compute_instance_static_vars(
        arena: &ExprArena,
        items: &[IrAssignmentItem],
        variables: &[VarDef],
    ) -> HashSet<SmolStr> {
        // Start from "all assigned variables are static" and remove any
        // with a non-static assignment until stable. Variables assigned
        // inside runtime loops stay eligible only if the loop condition is
        // also static (the iteration count must not vary per evaluation).
        let mut static_vars: HashSet<SmolStr> = HashSet::new();
        fn collect_targets(
            items: &[IrAssignmentItem],
            variables: &[VarDef],
            out: &mut HashSet<SmolStr>,
        ) {
            for item in items {
                match item {
                    IrAssignmentItem::Initialization { body, .. } => {
                        collect_targets(body, variables, out)
                    }
                    IrAssignmentItem::Task(_) => {}
                    IrAssignmentItem::Assign(a) => {
                        out.insert(variables[a.var_index].name.clone());
                    }
                    IrAssignmentItem::Loop { body, .. } => {
                        collect_targets(body, variables, out);
                    }
                }
            }
        }
        collect_targets(items, variables, &mut static_vars);
        fn remove_mutable_state(
            items: &[IrAssignmentItem],
            variables: &[VarDef],
            out: &mut HashSet<SmolStr>,
        ) {
            for item in items {
                match item {
                    IrAssignmentItem::Assign(assignment) => {
                        let len = assignment.index.as_ref().map_or(1, |target| target.len);
                        for variable in variables.iter().skip(assignment.var_index).take(len) {
                            if variable.is_state {
                                out.remove(&variable.name);
                            }
                        }
                    }
                    IrAssignmentItem::Loop { body, .. } => {
                        remove_mutable_state(body, variables, out)
                    }
                    IrAssignmentItem::Initialization { .. } | IrAssignmentItem::Task(_) => {}
                }
            }
        }
        remove_mutable_state(items, variables, &mut static_vars);

        loop {
            let mut changed = false;
            fn prune(
                arena: &ExprArena,
                items: &[IrAssignmentItem],
                variables: &[VarDef],
                static_vars: &mut HashSet<SmolStr>,
                changed: &mut bool,
                enclosing_static: bool,
            ) {
                for item in items {
                    match item {
                        IrAssignmentItem::Initialization { body, .. } => prune(
                            arena,
                            body,
                            variables,
                            static_vars,
                            changed,
                            enclosing_static,
                        ),
                        IrAssignmentItem::Task(_) => {}
                        IrAssignmentItem::Assign(a) => {
                            if let Some(target) = &a.index {
                                // A runtime-indexed write may land in any
                                // element; a non-static one evicts them all
                                let write_static = enclosing_static
                                    && DeviceIR::is_instance_static_expr(
                                        arena,
                                        a.expr,
                                        static_vars,
                                    )
                                    && DeviceIR::is_instance_static_expr(
                                        arena,
                                        target.index,
                                        static_vars,
                                    );
                                if !write_static {
                                    for k in target.lower..target.lower + target.len as i64 {
                                        let elem: SmolStr = format!("{}[{k}]", target.array).into();
                                        if static_vars.remove(&elem) {
                                            *changed = true;
                                        }
                                    }
                                }
                                continue;
                            }
                            let name = &variables[a.var_index].name;
                            if static_vars.contains(name)
                                && (!enclosing_static
                                    || !DeviceIR::is_instance_static_expr(
                                        arena,
                                        a.expr,
                                        static_vars,
                                    ))
                            {
                                static_vars.remove(name);
                                *changed = true;
                            }
                        }
                        IrAssignmentItem::Loop { condition, body } => {
                            let loop_static = enclosing_static
                                && DeviceIR::is_instance_static_expr(
                                    arena,
                                    *condition,
                                    static_vars,
                                );
                            prune(arena, body, variables, static_vars, changed, loop_static);
                        }
                    }
                }
            }
            prune(
                arena,
                items,
                variables,
                &mut static_vars,
                &mut changed,
                true,
            );
            if !changed {
                break;
            }
        }

        static_vars
    }
}

/// Automatic differentiation for Jacobian generation
pub mod autodiff {
    use super::*;
    use crate::ir::arena::{rewrite, visit};
    use std::collections::{BTreeSet, HashMap, HashSet};

    /// Bitmask over differentiation axes (node voltages first, then
    /// branch-current unknowns). Devices with more than 128 axes saturate
    /// to "all axes" — dense but always correct.
    pub(crate) type AxisMask = u128;

    /// All-axes mask (saturation value)
    const ALL_AXES: AxisMask = !0;

    /// Bit for one differentiation axis
    fn axis_bit(wrt: &DerivativeWrt, num_nodes: usize) -> AxisMask {
        let ordinal = match wrt {
            DerivativeWrt::Voltage(node) => *node,
            DerivativeWrt::BranchCurrent(k) => num_nodes + k,
            DerivativeWrt::Noise(_) | DerivativeWrt::LimiterCorrection => return ALL_AXES,
        };
        if ordinal >= 128 {
            ALL_AXES
        } else {
            1 << ordinal
        }
    }

    pub(crate) fn mask_contains_axis(
        mask: AxisMask,
        wrt: &DerivativeWrt,
        num_nodes: usize,
    ) -> bool {
        mask & axis_bit(wrt, num_nodes) != 0
    }

    /// Bit for a unified node index appearing in a probe (the ground
    /// sentinel is not an axis)
    fn node_bit(node: usize) -> AxisMask {
        if node == usize::MAX {
            0
        } else if node >= 128 {
            ALL_AXES
        } else {
            1 << node
        }
    }

    /// Shadow-variable context for forward-mode AD through assignment
    /// sequences.
    ///
    /// For every variable whose value depends (transitively) on node
    /// voltages, a shadow variable holds d(var)/d(axis) — but only along
    /// the axes the variable can actually vary with (its dependency mask):
    /// a variable computed from V(g) and V(s) never carries shadows along
    /// the drain or any branch-current axis. The shadows are updated by
    /// generated assignments placed immediately before each original
    /// assignment.
    #[derive(Debug, Default)]
    pub struct ShadowContext {
        /// Dependency axes per voltage-dependent variable. For arrays, the
        /// array name and every element name share one mask: a runtime
        /// index may select any slot.
        shadowed: HashMap<SmolStr, AxisMask>,
        /// First slot of the contiguous shadow run per shadow-array name
        /// (`shadow_name(array, wrt)` -> variable index of element `lower`)
        array_shadow_base: HashMap<SmolStr, usize>,
        /// Node-axis count (axis ordinals of branch unknowns start here)
        num_nodes: usize,
        /// Non-matrix derivative directions carried by each variable.
        auxiliary_shadowed: HashMap<SmolStr, BTreeSet<DerivativeWrt>>,
    }

    impl ShadowContext {
        pub fn empty() -> Self {
            Self::default()
        }

        /// Name of the shadow variable holding the derivative of `name`
        /// along the given axis (node voltage or branch current)
        pub fn shadow_name(name: &str, wrt: &DerivativeWrt) -> SmolStr {
            match wrt {
                DerivativeWrt::Voltage(node) => format!("{name}@d{node}").into(),
                DerivativeWrt::BranchCurrent(k) => format!("{name}@dI{k}").into(),
                DerivativeWrt::Noise(k) => format!("{name}@dN{k}").into(),
                DerivativeWrt::LimiterCorrection => format!("{name}@dL").into(),
            }
        }

        pub fn is_shadowed(&self, name: &str) -> bool {
            self.shadowed.get(name).is_some_and(|mask| *mask != 0)
        }

        /// Whether `name` carries a shadow along the given axis
        pub fn is_shadowed_on(&self, name: &str, wrt: &DerivativeWrt) -> bool {
            if matches!(
                wrt,
                DerivativeWrt::Noise(_) | DerivativeWrt::LimiterCorrection
            ) {
                return self
                    .auxiliary_shadowed
                    .get(name)
                    .is_some_and(|axes| axes.contains(wrt));
            }
            self.shadowed
                .get(name)
                .is_some_and(|mask| mask & axis_bit(wrt, self.num_nodes) != 0)
        }

        fn auxiliary_axes_of(&self, name: &str) -> Option<&BTreeSet<DerivativeWrt>> {
            self.auxiliary_shadowed.get(name)
        }

        /// First variable slot of an array's shadow run along an axis
        pub fn array_shadow_base(&self, array: &str, wrt: &DerivativeWrt) -> Option<usize> {
            self.array_shadow_base
                .get(&Self::shadow_name(array, wrt))
                .copied()
        }
    }

    /// All differentiation axes of a device: node voltages first, then
    /// branch-current unknowns
    pub(crate) fn axes(
        num_nodes: usize,
        num_branches: usize,
    ) -> impl Iterator<Item = DerivativeWrt> {
        (0..num_nodes)
            .map(DerivativeWrt::Voltage)
            .chain((0..num_branches).map(DerivativeWrt::BranchCurrent))
    }

    /// Collect every variable (and array) name an expression reads
    pub(crate) fn collect_var_names(arena: &ExprArena, expr: NodeId, out: &mut HashSet<SmolStr>) {
        visit(arena, expr, &mut |node| match *node {
            Node::Var(name) => {
                out.insert(arena.name(name).clone());
            }
            Node::VarIndexed { payload, .. } => {
                let array = arena.indexed(payload).array;
                out.insert(arena.name(array).clone());
            }
            _ => {}
        });
    }

    /// One noise process as the walk finds it.
    ///
    /// `psd` is `None` for a `noise_table`, whose magnitude is the constant
    /// one; the caller pushes that node, because this walk borrows the arena
    /// immutably and a shipped model has thousands of processes and one place
    /// that needs to write.
    pub(crate) type NoiseDefinition = (
        NoiseSiteId,
        Option<NodeId>,
        Option<NodeId>,
        Option<NoiseTableData>,
        Option<SmolStr>,
    );

    pub(crate) fn collect_noise_definitions(
        arena: &ExprArena,
        expr: NodeId,
        out: &mut Vec<NoiseDefinition>,
    ) {
        visit(arena, expr, &mut |node| {
            let Node::Heavy(_, heavy) = *node else {
                return;
            };
            match arena.heavy(heavy) {
                Heavy::WhiteNoise { site, power, name } => out.push((
                    *site,
                    Some(*power),
                    None,
                    None,
                    name.as_deref().map(SmolStr::from),
                )),
                Heavy::FlickerNoise {
                    site,
                    power,
                    exponent,
                    name,
                } => out.push((
                    *site,
                    Some(*power),
                    Some(*exponent),
                    None,
                    name.as_deref().map(SmolStr::from),
                )),
                Heavy::NoiseTable {
                    site,
                    points,
                    log_interp,
                    name,
                } => out.push((
                    *site,
                    None,
                    None,
                    Some(NoiseTableData {
                        points: points.clone(),
                        log_interp: *log_interp,
                    }),
                    name.as_deref().map(SmolStr::from),
                )),
                _ => {}
            }
        });
    }

    /// Collect variable names appearing inside ddx() operands across an
    /// assignment tree (their derivative resolution reads shadows)
    pub(crate) fn collect_ddx_operand_names_in_expr(
        arena: &ExprArena,
        expr: NodeId,
        out: &mut HashSet<SmolStr>,
    ) {
        visit(arena, expr, &mut |node| {
            if let Node::Ddx { expr, .. } = *node {
                collect_var_names(arena, expr, out);
            }
        });
    }

    pub(crate) fn collect_ddx_operand_names(
        arena: &ExprArena,
        items: &[IrAssignmentItem],
        out: &mut HashSet<SmolStr>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { .. } => {}
                IrAssignmentItem::Task(task) => {
                    for &expression in task.expressions() {
                        collect_ddx_operand_names_in_expr(arena, expression, out);
                    }
                }
                IrAssignmentItem::Assign(assign) => {
                    collect_ddx_operand_names_in_expr(arena, assign.expr, out);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    collect_ddx_operand_names_in_expr(arena, *condition, out);
                    collect_ddx_operand_names(arena, body, out);
                }
            }
        }
    }

    /// Check whether an expression can have a nonzero derivative along any
    /// node/branch axis, directly or through already-shadowed variables.
    ///
    /// Axes along which an expression can have a nonzero derivative,
    /// directly (probes) or through already-shadowed variables.
    ///
    /// Comparisons, logical operations, and event detectors differentiate
    /// to exactly zero regardless of their operands, so variables holding
    /// only such results (e.g. snapshotted branch guards) never need
    /// shadow slots; current probes are treated as constants in the DC
    /// Jacobian (matching [`differentiate_with_shadows`]).
    fn derivative_axes(
        arena: &ExprArena,
        expr: NodeId,
        deps: &HashMap<SmolStr, AxisMask>,
        num_nodes: usize,
    ) -> AxisMask {
        let recurse = |e: NodeId| derivative_axes(arena, e, deps, num_nodes);
        let optional = |slot: Option<NodeId>| slot.map_or(0, &recurse);
        match *arena.node(expr) {
            Node::FreezeDerivative(_) => 0,
            Node::Voltage(pos, neg) => node_bit(unpack_index(pos)) | node_bit(unpack_index(neg)),
            Node::BranchCurrent(ordinal) => axis_bit(
                &DerivativeWrt::BranchCurrent(unpack_index(ordinal)),
                num_nodes,
            ),
            // Current probes differentiate to zero in the DC Jacobian
            Node::Current(..) => 0,
            Node::Var(name) => deps.get(arena.name(name).as_str()).copied().unwrap_or(0),
            // The index only selects; the elements carry the slope
            Node::VarIndexed { payload, .. } => {
                let array = arena.indexed(payload).array;
                deps.get(arena.name(array).as_str()).copied().unwrap_or(0)
            }
            Node::Const(_)
            | Node::Param(_)
            | Node::ParamGiven(_)
            | Node::Time
            | Node::Temperature
            | Node::Vt
            | Node::SimParamValue(_)
            | Node::SimParamPresent(_)
            | Node::Mfactor
            | Node::PortConnected(_)
            | Node::Analysis(_) => 0,
            Node::Binary(op, left, right) => match op {
                BinaryOp::CheckedValue => recurse(right),
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Pow
                | BinaryOp::Mod => recurse(left) | recurse(right),
                // Piecewise-constant results: derivative identically zero
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::IntAdd
                | BinaryOp::IntSub
                | BinaryOp::IntMul
                | BinaryOp::IntDiv
                | BinaryOp::IntMod
                | BinaryOp::IntPow
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Shl
                | BinaryOp::Shr => 0,
            },
            Node::Unary(UnaryOp::Neg | UnaryOp::Pos, inner) => recurse(inner),
            Node::Unary(UnaryOp::Not | UnaryOp::BitNot | UnaryOp::ToInteger, _) => 0,
            Node::Limexp(inner) | Node::Ddt(inner) => recurse(inner),
            Node::Idt(inner, ic) => recurse(inner) | optional(ic),
            Node::IdtMod {
                expr: inner,
                modulus,
                payload,
            } => {
                let (ic, _) = arena.optional_pair(payload);
                recurse(inner) | recurse(modulus) | optional(ic)
            }
            // The slope chains through the proposal, not the limiter body: an
            // axis only the body can reach is a structural zero.
            Node::Limit(inner, _)
            | Node::NamedLimit {
                proposed: inner, ..
            } => recurse(inner),
            // The previous Newton iterate is frozen history.
            Node::LimiterPrevious(_) => 0,
            Node::Call { func, a, b, .. } => match func {
                IrFunction::Floor | IrFunction::Ceil => 0,
                _ => optional(a) | optional(b),
            },
            Node::CallSpilled { func, args } => match func {
                IrFunction::Floor | IrFunction::Ceil => 0,
                _ => arena
                    .call_args(args)
                    .iter()
                    .map(|arg| recurse(*arg))
                    .fold(0, |acc, mask| acc | mask),
            },
            // The condition only selects; the branches carry the slope
            Node::Conditional(_, then_expr, else_expr) => recurse(then_expr) | recurse(else_expr),
            Node::TableLookup { input, .. } | Node::TableDerivative { input, .. } => recurse(input),
            Node::Ddx { expr: inner, .. } => recurse(inner),
            Node::IdtCompanion(inner)
            | Node::DdtDerivative {
                input_derivative: inner,
                ..
            } => recurse(inner),
            // Event detectors are piecewise constant (or zero) in the DC
            // Jacobian
            Node::LastCrossing { .. } => 0,
            Node::Heavy(_, payload) => match arena.heavy(payload) {
                Heavy::IntegralDerivative {
                    input_derivative,
                    ic_derivative,
                    modulus_derivative,
                    ..
                } => {
                    recurse(*input_derivative)
                        | recurse(*ic_derivative)
                        | optional(*modulus_derivative)
                }
                Heavy::AbsDelay {
                    expr,
                    delay_time,
                    max_delay,
                    ..
                } => recurse(*expr) | recurse(*delay_time) | optional(*max_delay),
                Heavy::AbsDelayDerivative {
                    input,
                    input_derivative,
                    delay_time,
                    delay_derivative,
                    max_delay,
                    ..
                } => {
                    recurse(*input)
                        | recurse(*input_derivative)
                        | recurse(*delay_time)
                        | recurse(*delay_derivative)
                        | optional(*max_delay)
                }
                Heavy::Transition { expr, .. }
                | Heavy::LaplaceZP { expr, .. }
                | Heavy::LaplaceND { expr, .. }
                | Heavy::LaplaceZPDerivative { expr, .. }
                | Heavy::LaplaceNDDerivative { expr, .. }
                | Heavy::ZiFilter { expr, .. }
                | Heavy::ZiFilterDerivative { expr, .. } => recurse(*expr),
                Heavy::TransitionDerivative {
                    input,
                    input_derivative,
                    delay,
                    rise_time,
                    fall_time,
                    ..
                } => {
                    recurse(*input)
                        | recurse(*input_derivative)
                        | optional(*delay)
                        | optional(*rise_time)
                        | optional(*fall_time)
                }
                Heavy::Slew {
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                    ..
                } => recurse(*expr) | optional(*max_pos_slew) | optional(*max_neg_slew),
                Heavy::SlewDerivative {
                    input,
                    input_derivative,
                    max_pos_slew,
                    max_pos_slew_derivative,
                    max_neg_slew,
                    max_neg_slew_derivative,
                    ..
                } => {
                    recurse(*input)
                        | recurse(*input_derivative)
                        | optional(*max_pos_slew)
                        | optional(*max_pos_slew_derivative)
                        | optional(*max_neg_slew)
                        | optional(*max_neg_slew_derivative)
                }
                // Event detectors and noise sources are piecewise constant
                // (or zero) in the DC Jacobian
                Heavy::Cross { .. }
                | Heavy::Above { .. }
                | Heavy::Timer { .. }
                | Heavy::WhiteNoise { .. }
                | Heavy::FlickerNoise { .. }
                | Heavy::NoiseTable { .. } => 0,
            },
        }
    }

    pub(crate) fn expression_axes(
        arena: &ExprArena,
        expr: NodeId,
        shadows: &ShadowContext,
        num_nodes: usize,
    ) -> AxisMask {
        derivative_axes(arena, expr, &shadows.shadowed, num_nodes)
    }

    /// Accumulate per-variable dependency axes over an item tree
    /// (fixpoint helper for [`build_shadow_assignments`]).
    ///
    /// A voltage-dependent write into any array element shadows the whole
    /// array: a runtime index may route the value to any slot, so every
    /// element (and the array name itself, checked by indexed reads)
    /// shares one mask.
    fn scan_shadowed(
        arena: &ExprArena,
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        arrays: &[ArrayDef],
        num_nodes: usize,
        deps: &mut HashMap<SmolStr, AxisMask>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { .. } => {}
                IrAssignmentItem::Task(_) => {}
                IrAssignmentItem::Assign(assign) => {
                    let mask = derivative_axes(arena, assign.expr, deps, num_nodes);
                    if mask == 0 {
                        continue;
                    }
                    let enclosing = arrays
                        .iter()
                        .find(|a| assign.var_index >= a.base && assign.var_index < a.base + a.len);
                    if let Some(array) = enclosing {
                        let current = deps.get(&array.name).copied().unwrap_or(0);
                        if current | mask != current {
                            let merged = current | mask;
                            deps.insert(array.name.clone(), merged);
                            for k in array.lower..array.lower + array.len as i64 {
                                deps.insert(format!("{}[{k}]", array.name).into(), merged);
                            }
                            *changed = true;
                        }
                    } else {
                        let name = &variables[assign.var_index].name;
                        let current = deps.get(name).copied().unwrap_or(0);
                        if current | mask != current {
                            deps.insert(name.clone(), current | mask);
                            *changed = true;
                        }
                    }
                }
                IrAssignmentItem::Loop { body, .. } => {
                    scan_shadowed(arena, body, variables, arrays, num_nodes, deps, changed);
                }
            }
        }
    }

    /// The read edges shadow liveness closes over, extracted once.
    ///
    /// Liveness here is a reachability question — a variable is live when
    /// some chain of writes carries it into a contribution — and reachability
    /// wants a graph, not a rescan. Reading the edges out of the assignment
    /// trees once and then closing over them keeps the two things that used to
    /// be entangled apart: the expression walk happens a fixed number of
    /// times, and the fixpoint iterates over names.
    ///
    /// The distinction matters because the walk was the expensive half.
    /// Liveness flows *backward* through a forward-ordered assignment list, so
    /// a straight-line chain of `n` writes advanced the answer by exactly one
    /// write per sweep and re-walked every live expression on each of the `n`
    /// sweeps — and the walk went through `map_expr`, which rebuilds the tree,
    /// so each sweep also copied and dropped the live half of the program.
    /// That is quadratic in the assignment count times the expression size.
    ///
    /// What this buys on the shipped corpus is small and should be stated as
    /// such: the old sweep cost 1.31s across all 43 models against 0.12s here,
    /// because a shipped model's live set converges in a few sweeps, and the
    /// second call site — inside `build_auxiliary_shadow_assignments`, where the
    /// list has already been expanded by shadow interleaving — is never
    /// reached, since no shipped model has a noise process feeding a variable.
    /// The bound is the point. A user model that does reach it would sweep a
    /// million-assignment list once per link of its dependency chain.
    struct LivenessGraph {
        /// Names read by the writes to each target, deduplicated. Sorted so
        /// the structure is a function of the module and not of hash order.
        reads_by_target: HashMap<SmolStr, Vec<SmolStr>>,
        /// Array element families: one live member makes the family live.
        families: Vec<Vec<SmolStr>>,
        /// Family index of an array name or one of its element names.
        family_of: HashMap<SmolStr, usize>,
    }

    impl LivenessGraph {
        fn build(
            arena: &ExprArena,
            items: &[IrAssignmentItem],
            variables: &[VarDef],
            arrays: &[ArrayDef],
        ) -> Self {
            let mut reads_by_target: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
            collect_liveness_edges(arena, items, variables, &mut reads_by_target);
            for reads in reads_by_target.values_mut() {
                reads.sort_unstable();
                reads.dedup();
            }

            let mut families = Vec::with_capacity(arrays.len());
            let mut family_of = HashMap::new();
            for array in arrays {
                let index = families.len();
                let mut members = Vec::with_capacity(array.len + 1);
                members.push(array.name.clone());
                for k in array.lower..array.lower + array.len as i64 {
                    members.push(format!("{}[{k}]", array.name).into());
                }
                for member in &members {
                    family_of.insert(member.clone(), index);
                }
                families.push(members);
            }

            Self {
                reads_by_target,
                families,
                family_of,
            }
        }

        /// Least set containing `roots` and closed under both relations: a
        /// live target makes everything its writes read live, and a live
        /// family member makes the whole family live.
        fn live_from(&self, roots: &HashSet<SmolStr>) -> HashSet<SmolStr> {
            let mut live = HashSet::with_capacity(roots.len());
            let mut pending: Vec<SmolStr> = Vec::with_capacity(roots.len());
            for root in roots {
                if live.insert(root.clone()) {
                    pending.push(root.clone());
                }
            }
            while let Some(name) = pending.pop() {
                if let Some(&family) = self.family_of.get(&name) {
                    for member in &self.families[family] {
                        if live.insert(member.clone()) {
                            pending.push(member.clone());
                        }
                    }
                }
                if let Some(reads) = self.reads_by_target.get(&name) {
                    for read in reads {
                        if live.insert(read.clone()) {
                            pending.push(read.clone());
                        }
                    }
                }
            }
            live
        }
    }

    /// Record, per write target, every name the write reads. An indexed write
    /// is attributed to the array name, since a runtime index may land in any
    /// element; a loop's condition is not a read here, exactly as before.
    fn collect_liveness_edges(
        arena: &ExprArena,
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        out: &mut HashMap<SmolStr, Vec<SmolStr>>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { .. } => {}
                IrAssignmentItem::Task(_) => {}
                IrAssignmentItem::Assign(assign) => {
                    let target = match &assign.index {
                        Some(target) => target.array.clone(),
                        None => variables[assign.var_index].name.clone(),
                    };
                    let mut reads = HashSet::new();
                    collect_var_names(arena, assign.expr, &mut reads);
                    if let Some(target) = &assign.index {
                        collect_var_names(arena, target.index, &mut reads);
                    }
                    out.entry(target).or_default().extend(reads);
                }
                IrAssignmentItem::Loop { body, .. } => {
                    collect_liveness_edges(arena, body, variables, out);
                }
            }
        }
    }

    /// Orders needed at each write, rather than a module-wide fixed jet.
    /// A later reassignment can require fewer derivatives than an earlier one.
    #[derive(Default)]
    struct ShadowPlan {
        order: usize,
        body: Vec<ShadowPlan>,
    }

    struct ShadowDemand<'a> {
        arena: &'a ExprArena,
        variables: &'a [VarDef],
        arrays: &'a [ArrayDef],
        families: HashMap<SmolStr, SmolStr>,
        deps: &'a HashMap<SmolStr, AxisMask>,
        orders: HashMap<SmolStr, usize>,
    }

    impl ShadowDemand<'_> {
        fn name(&self, name: &SmolStr) -> SmolStr {
            self.families.get(name).unwrap_or(name).clone()
        }

        fn require(&mut self, name: SmolStr, order: usize, pending: &mut HashMap<SmolStr, usize>) {
            let family = self.name(&name);
            if order == 0 || self.deps.get(&family).copied().unwrap_or(0) == 0 {
                return;
            }
            // Indexed reads may select any element. Keep their demands separate
            // so a definite write kills only the overwritten element's demand.
            if let Some(array) = self.arrays.iter().find(|array| array.name == name) {
                let variables = self.variables;
                for variable in &variables[array.base..array.base + array.len] {
                    self.require(variable.name.clone(), order, pending);
                }
                return;
            }
            pending
                .entry(name)
                .and_modify(|old| *old = (*old).max(order))
                .or_insert(order);
            self.orders
                .entry(family)
                .and_modify(|old| *old = (*old).max(order))
                .or_insert(order);
        }

        fn expression(
            &mut self,
            expr: NodeId,
            order: usize,
            pending: &mut HashMap<SmolStr, usize>,
        ) {
            let node = *self.arena.node(expr);
            match node {
                Node::Var(name) => self.require(self.arena.name(name).clone(), order, pending),
                Node::VarIndexed { payload, index } => {
                    self.require(
                        self.arena.name(self.arena.indexed(payload).array).clone(),
                        order,
                        pending,
                    );
                    self.expression(index, 0, pending);
                }
                Node::Ddx { expr, .. } => self.expression(expr, order + 1, pending),
                Node::FreezeDerivative(expr) => self.expression(expr, 0, pending),
                Node::Conditional(condition, left, right) => {
                    self.expression(condition, 0, pending);
                    self.expression(left, order, pending);
                    self.expression(right, order, pending);
                }
                _ => {
                    let arena = self.arena;
                    arena::for_each_child(arena, &node, &mut |child| {
                        self.expression(child, order, pending);
                    });
                    for child in arena::operator_operands(arena, &node) {
                        self.expression(child, order, pending);
                    }
                }
            }
        }

        fn assignments(
            &mut self,
            items: &[IrAssignmentItem],
            pending: &mut HashMap<SmolStr, usize>,
            plans: &mut Vec<ShadowPlan>,
        ) -> CompileResult<()> {
            plans.resize_with(items.len(), ShadowPlan::default);
            for (item, plan) in items.iter().zip(plans).rev() {
                match item {
                    IrAssignmentItem::Initialization { .. } => {}
                    IrAssignmentItem::Task(task) => {
                        for &expr in task.expressions() {
                            self.expression(expr, 0, pending);
                        }
                    }
                    IrAssignmentItem::Assign(assign) => {
                        let target = &self.variables[assign.var_index].name;
                        let order = if let Some(indexed) = &assign.index {
                            let members =
                                &self.variables[assign.var_index..assign.var_index + indexed.len];
                            let order = members
                                .iter()
                                .filter_map(|member| pending.get(&member.name))
                                .copied()
                                .max()
                                .unwrap_or(0);
                            if indexed.len == 1 {
                                pending.remove(target);
                            }
                            order
                        } else {
                            pending.remove(target).unwrap_or(0)
                        };
                        plan.order = plan.order.max(order);
                        self.expression(assign.expr, order, pending);
                        if let Some(target) = &assign.index {
                            self.expression(target.index, 0, pending);
                        }
                    }
                    IrAssignmentItem::Loop { condition, body } => {
                        self.expression(*condition, 0, pending);
                        let mut converged = false;
                        // A finite dependency path crosses at most one edge per
                        // variable before repeating. Further growth requires a
                        // positive-order cycle (ddx of a loop-carried readback).
                        for _ in 0..=self.variables.len() {
                            let previous = pending.clone();
                            self.assignments(body, pending, &mut plan.body)?;
                            self.expression(*condition, 0, pending);
                            for (name, order) in &previous {
                                pending
                                    .entry(name.clone())
                                    .and_modify(|old| *old = (*old).max(*order))
                                    .or_insert(*order);
                            }
                            if *pending == previous {
                                converged = true;
                                break;
                            }
                        }
                        if !converged {
                            return Err(crate::error::CodeGenError::new(
                                crate::error::CodeGenErrorKind::InvalidExpression(
                                    "ddx through a recursive loop dependency requires an unbounded derivative order".into(),
                                ),
                            ).into());
                        }
                    }
                }
            }
            Ok(())
        }
    }

    fn ddx_reads_target(
        arena: &ExprArena,
        expr: NodeId,
        target: &SmolStr,
        families: &HashMap<SmolStr, SmolStr>,
    ) -> bool {
        if !contains_ddx(arena, expr) {
            return false;
        }
        let mut reads = HashSet::new();
        collect_var_names(arena, expr, &mut reads);
        if reads.contains(target) {
            return true;
        }
        let Some(family) = families.get(target) else {
            return false;
        };
        if family == target {
            // An indexed write may alias any read from this array.
            reads.iter().any(|name| families.get(name) == Some(family))
        } else {
            // A fixed-element write may alias a runtime-indexed read.
            reads.contains(family)
        }
    }

    fn stage_shadow_expression(
        arena: &mut ExprArena,
        variables: &mut Vec<VarDef>,
        assignments: &mut Vec<IrAssignmentItem>,
        expr: NodeId,
    ) -> NodeId {
        let slot = variables.len();
        let mut name: SmolStr = format!("@ddx_update{slot}").into();
        while variables.iter().any(|variable| variable.name == name) {
            name = format!("{name}_").into();
        }
        variables.push(VarDef {
            name: name.clone(),
            is_state: false,
        });
        assignments.push(IrAssignmentItem::Assign(VarAssignment {
            var_index: slot,
            index: None,
            expr,
        }));
        let name = arena.intern(&name);
        arena.push(Node::Var(name))
    }

    /// Emit higher orders first so ordinary self-updates read the previous jet.
    /// A ddx self-update also reads higher orders: stage that small group before
    /// writing any shadow, preserving simultaneous-assignment semantics.
    fn interleave_shadows(
        arena: &mut ExprArena,
        items: Vec<IrAssignmentItem>,
        plans: Vec<ShadowPlan>,
        variables: &mut Vec<VarDef>,
        shadow_index: &HashMap<SmolStr, usize>,
        ctx: &ShadowContext,
        families: &HashMap<SmolStr, SmolStr>,
        num_nodes: usize,
        num_branches: usize,
    ) -> Vec<IrAssignmentItem> {
        let mut rewritten = Vec::with_capacity(items.len() * 2);
        for (item, plan) in items.into_iter().zip(plans) {
            match item {
                IrAssignmentItem::Assign(mut assign) => {
                    let target = assign
                        .index
                        .as_ref()
                        .map(|target| target.array.clone())
                        .unwrap_or_else(|| variables[assign.var_index].name.clone());
                    let stage = ddx_reads_target(arena, assign.expr, &target, families);
                    if let Some(indexed) = &mut assign.index
                        && ddx_reads_target(arena, indexed.index, &target, families)
                    {
                        indexed.index = stage_shadow_expression(
                            arena,
                            variables,
                            &mut rewritten,
                            indexed.index,
                        );
                    }
                    let mut layer = vec![(target.clone(), assign.expr)];
                    let mut updates = Vec::new();
                    for _ in 0..plan.order {
                        let mut next = Vec::new();
                        let mut group = Vec::new();
                        for (name, expr) in layer {
                            for wrt in axes(num_nodes, num_branches) {
                                if !ctx.is_shadowed_on(&name, &wrt) {
                                    continue;
                                }
                                let derivative = differentiate_with_shadows(arena, expr, &wrt, ctx);
                                let shadow = ShadowContext::shadow_name(&name, &wrt);
                                let (var_index, index) = match &assign.index {
                                    Some(target) => (
                                        ctx.array_shadow_base(&name, &wrt)
                                            .expect("shadow array has a contiguous run"),
                                        Some(IndexedTarget {
                                            array: shadow.clone(),
                                            ..target.clone()
                                        }),
                                    ),
                                    None => (shadow_index[&shadow], None),
                                };
                                group.push(VarAssignment {
                                    var_index,
                                    index,
                                    expr: derivative,
                                });
                                next.push((shadow, derivative));
                            }
                        }
                        updates.push(group);
                        layer = next;
                    }
                    if stage {
                        // The primal itself reads the old derivative shadows.
                        assign.expr =
                            stage_shadow_expression(arena, variables, &mut rewritten, assign.expr);
                    }
                    let mut writes = Vec::new();
                    for mut update in updates.into_iter().rev().flatten() {
                        if stage {
                            update.expr = stage_shadow_expression(
                                arena,
                                variables,
                                &mut rewritten,
                                update.expr,
                            );
                            writes.push(IrAssignmentItem::Assign(update));
                        } else {
                            rewritten.push(IrAssignmentItem::Assign(update));
                        }
                    }
                    rewritten.extend(writes);
                    rewritten.push(IrAssignmentItem::Assign(assign));
                }
                IrAssignmentItem::Loop { condition, body } => {
                    let body = interleave_shadows(
                        arena,
                        body,
                        plan.body,
                        variables,
                        shadow_index,
                        ctx,
                        families,
                        num_nodes,
                        num_branches,
                    );
                    rewritten.push(IrAssignmentItem::Loop { condition, body });
                }
                other => rewritten.push(other),
            }
        }
        rewritten
    }

    /// Allocate only the orders actually consumed at each reaching write.
    pub fn build_shadow_assignments(
        ir: &mut DeviceIR,
        num_nodes: usize,
        num_branches: usize,
        contributions: &[NodeId],
    ) -> CompileResult<ShadowContext> {
        let DeviceIR {
            exprs,
            assignments,
            variables,
            arrays,
            ..
        } = ir;
        let span = crate::metrics::FineSpan::new("ir.shadow_axis_fixpoint");
        let mut deps = HashMap::new();
        let mut axis_passes = 0;
        loop {
            let mut changed = false;
            axis_passes += 1;
            scan_shadowed(
                exprs,
                assignments,
                variables,
                arrays,
                num_nodes,
                &mut deps,
                &mut changed,
            );
            if !changed {
                break;
            }
        }
        span.finish(&format!("passes={axis_passes} shadowed={}", deps.len()));
        let span = crate::metrics::FineSpan::new("ir.shadow_liveness");
        let families: HashMap<_, _> = arrays
            .iter()
            .flat_map(|array| {
                std::iter::once((array.name.clone(), array.name.clone())).chain(
                    variables[array.base..array.base + array.len]
                        .iter()
                        .map(|var| (var.name.clone(), array.name.clone())),
                )
            })
            .collect();
        let mut demand = ShadowDemand {
            arena: exprs,
            variables,
            arrays,
            families,
            deps: &deps,
            orders: HashMap::new(),
        };
        let mut pending = HashMap::new();
        for &expr in contributions {
            demand.expression(expr, 1, &mut pending);
        }
        let mut plans = Vec::new();
        demand.assignments(assignments, &mut pending, &mut plans)?;
        let orders = demand.orders;
        let families = demand.families;
        span.finish(&format!(
            "live={} max_order={}",
            orders.len(),
            orders.values().copied().max().unwrap_or(0)
        ));
        if orders.is_empty() {
            return Ok(ShadowContext::default());
        }

        let span = crate::metrics::FineSpan::new("ir.shadow_layout");
        let mut ctx = ShadowContext {
            num_nodes,
            ..ShadowContext::default()
        };
        let mut shadow_index = HashMap::new();
        let mut layout: Vec<_> = orders.into_iter().collect();
        layout.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        for (name, order) in layout {
            let mask = deps[&name];
            let array = arrays.iter().find(|array| array.name == name);
            let mut layer = vec![(
                name.clone(),
                array.map(|array| {
                    variables[array.base..array.base + array.len]
                        .iter()
                        .map(|var| var.name.clone())
                        .collect::<Vec<_>>()
                }),
            )];
            for _ in 0..order {
                let mut next = Vec::new();
                for (name, members) in layer {
                    ctx.shadowed.insert(name.clone(), mask);
                    if let Some(members) = &members {
                        for member in members {
                            ctx.shadowed.insert(member.clone(), mask);
                        }
                    }
                    for wrt in axes(num_nodes, num_branches) {
                        if mask & axis_bit(&wrt, num_nodes) == 0 {
                            continue;
                        }
                        let shadow = ShadowContext::shadow_name(&name, &wrt);
                        let shadow_members = members.as_ref().map(|members| {
                            ctx.array_shadow_base
                                .insert(shadow.clone(), variables.len());
                            members
                                .iter()
                                .map(|member| ShadowContext::shadow_name(member, &wrt))
                                .collect::<Vec<_>>()
                        });
                        for slot_name in shadow_members
                            .as_deref()
                            .unwrap_or(std::slice::from_ref(&shadow))
                        {
                            shadow_index.insert(slot_name.clone(), variables.len());
                            variables.push(VarDef {
                                name: slot_name.clone(),
                                is_state: false,
                            });
                        }
                        next.push((shadow, shadow_members));
                    }
                }
                layer = next;
            }
        }
        span.finish(&format!("shadow_slots={}", shadow_index.len()));
        let span = crate::metrics::FineSpan::new("ir.shadow_interleave");
        let originals = std::mem::take(assignments);
        *assignments = interleave_shadows(
            exprs,
            originals,
            plans,
            variables,
            &shadow_index,
            &ctx,
            &families,
            num_nodes,
            num_branches,
        );
        span.finish(&format!(
            "assignments={} nodes={}",
            assignments.len(),
            exprs.len()
        ));
        Ok(ctx)
    }

    #[derive(Clone, Copy)]
    enum SimplifiedConstant {
        Value(f64),
        Other,
    }

    impl SimplifiedConstant {
        fn is_zero(self) -> bool {
            matches!(self, Self::Value(value) if value == 0.0)
        }
    }

    /// Return the constant produced by [`simplify`], if any. Results are
    /// memoized by node id so zero-factor and constant-condition checks stay
    /// linear even for deeply skewed expression trees — and, now that a
    /// derivative names its primal operands instead of copying them, one entry
    /// answers for every path through a shared subtree.
    fn simplified_constant(
        arena: &ExprArena,
        expr: NodeId,
        constants: &mut HashMap<NodeId, SimplifiedConstant>,
    ) -> SimplifiedConstant {
        if let Some(value) = constants.get(&expr) {
            return *value;
        }
        let value = match *arena.node(expr) {
            Node::Const(value) => SimplifiedConstant::Value(value),
            Node::Binary(op, left, right) => {
                let left = simplified_constant(arena, left, constants);
                let right = simplified_constant(arena, right, constants);
                if let (SimplifiedConstant::Value(left), SimplifiedConstant::Value(right)) =
                    (left, right)
                {
                    match op {
                        BinaryOp::Add => SimplifiedConstant::Value(left + right),
                        BinaryOp::Sub => SimplifiedConstant::Value(left - right),
                        BinaryOp::Mul => SimplifiedConstant::Value(left * right),
                        BinaryOp::Div => SimplifiedConstant::Value(left / right),
                        BinaryOp::Pow => SimplifiedConstant::Value(left.powf(right)),
                        _ => SimplifiedConstant::Other,
                    }
                } else {
                    match op {
                        BinaryOp::Add if left.is_zero() => right,
                        BinaryOp::Add if right.is_zero() => left,
                        BinaryOp::Sub if right.is_zero() => left,
                        BinaryOp::Mul if left.is_zero() || right.is_zero() => {
                            SimplifiedConstant::Value(0.0)
                        }
                        BinaryOp::Mul if matches!(left, SimplifiedConstant::Value(1.0)) => right,
                        BinaryOp::Mul if matches!(right, SimplifiedConstant::Value(1.0)) => left,
                        BinaryOp::Div if left.is_zero() => SimplifiedConstant::Value(0.0),
                        BinaryOp::Div if matches!(right, SimplifiedConstant::Value(1.0)) => left,
                        _ => SimplifiedConstant::Other,
                    }
                }
            }
            Node::Unary(op, inner) => {
                let inner = simplified_constant(arena, inner, constants);
                match (op, inner) {
                    (UnaryOp::Neg, SimplifiedConstant::Value(value)) => {
                        SimplifiedConstant::Value(-value)
                    }
                    (UnaryOp::Pos, value) => value,
                    _ => SimplifiedConstant::Other,
                }
            }
            Node::Conditional(condition, then_expr, else_expr) => {
                let condition = simplified_constant(arena, condition, constants);
                let then_expr = simplified_constant(arena, then_expr, constants);
                let else_expr = simplified_constant(arena, else_expr, constants);
                match condition {
                    SimplifiedConstant::Value(value) if value != 0.0 => then_expr,
                    SimplifiedConstant::Value(_) => else_expr,
                    SimplifiedConstant::Other => SimplifiedConstant::Other,
                }
            }
            Node::Call { a, b, .. } => {
                if let Some(argument) = a {
                    simplified_constant(arena, argument, constants);
                }
                if let Some(argument) = b {
                    simplified_constant(arena, argument, constants);
                }
                SimplifiedConstant::Other
            }
            Node::CallSpilled { args, .. } => {
                for argument in arena.call_args(args) {
                    simplified_constant(arena, *argument, constants);
                }
                SimplifiedConstant::Other
            }
            Node::IdtCompanion(inner) => {
                if simplified_constant(arena, inner, constants).is_zero() {
                    SimplifiedConstant::Value(0.0)
                } else {
                    SimplifiedConstant::Other
                }
            }
            _ => SimplifiedConstant::Other,
        };
        constants.insert(expr, value);
        value
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum AuxiliaryAxes {
        Noise(usize),
        LimiterCorrection,
    }

    impl AuxiliaryAxes {
        fn contains(self, axis: DerivativeWrt) -> bool {
            match (self, axis) {
                (Self::Noise(count), DerivativeWrt::Noise(process)) => process < count,
                (Self::LimiterCorrection, DerivativeWrt::LimiterCorrection) => true,
                _ => false,
            }
        }
    }

    /// Collect the auxiliary directions that can affect `expr`.
    ///
    /// This is the set-valued counterpart of [`differentiate_with_shadows`]
    /// for noise realizations and limiter displacement. Keep this traversal in lock-step
    /// with that routine is important: operands which are metadata or merely
    /// select a value (noise PSDs, array indices, conditional predicates, and
    /// stateful-operator timing arguments) are deliberately not traversed.
    /// Actual derivative expressions are built after liveness removes
    /// directions which cannot reach a contribution.
    fn collect_expression_auxiliary_axes(
        arena: &mut ExprArena,
        expr: NodeId,
        deps: &HashMap<SmolStr, BTreeSet<DerivativeWrt>>,
        family: AuxiliaryAxes,
        constants: &mut HashMap<NodeId, SimplifiedConstant>,
        axes: &mut BTreeSet<DerivativeWrt>,
    ) {
        macro_rules! collect {
            ($value:expr) => {
                collect_expression_auxiliary_axes(arena, $value, deps, family, constants, axes)
            };
        }
        macro_rules! is_zero {
            ($value:expr) => {
                simplified_constant(arena, $value, constants).is_zero()
            };
        }
        match *arena.node(expr) {
            Node::FreezeDerivative(_) => {}
            Node::Var(name) => {
                if let Some(processes) = deps.get(arena.name(name).as_str()) {
                    axes.extend(processes.iter().copied());
                }
            }
            // A runtime index selects an element; it is not part of the
            // differentiable value path.
            Node::VarIndexed { payload, .. } => {
                let array = arena.indexed(payload).array;
                if let Some(processes) = deps.get(arena.name(array).as_str()) {
                    axes.extend(processes.iter().copied());
                }
            }
            Node::Binary(op, left, right) => match op {
                BinaryOp::CheckedValue => collect!(right),
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mod => {
                    collect!(left);
                    collect!(right);
                }
                BinaryOp::Mul => {
                    // `simplify` removes either product-rule term when its
                    // primal multiplier is identically zero.
                    if !is_zero!(right) {
                        collect!(left);
                    }
                    if !is_zero!(left) {
                        collect!(right);
                    }
                }
                BinaryOp::Div => {
                    // The quotient numerator is dl*right - left*dr.
                    // Keep numerator provenance even for an identically zero
                    // denominator: symbolic AD produces 0/0 (NaN), not a zero
                    // derivative, and the runtime must diagnose that singular
                    // expression instead of pruning its process as dead.
                    collect!(left);
                    if !is_zero!(left) {
                        collect!(right);
                    }
                }
                BinaryOp::Pow => {
                    if let Node::Const(exponent) = *arena.node(right) {
                        if exponent != 0.0 {
                            collect!(left);
                        }
                    } else {
                        // d(u^v) contains v' and, unless v is identically
                        // zero, u'.
                        collect!(right);
                        if !is_zero!(right) {
                            collect!(left);
                        }
                    }
                }
                // These operators are piecewise constant under the Jacobian
                // convention used by `differentiate_with_shadows`.
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::IntAdd
                | BinaryOp::IntSub
                | BinaryOp::IntMul
                | BinaryOp::IntDiv
                | BinaryOp::IntMod
                | BinaryOp::IntPow
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Shl
                | BinaryOp::Shr => {}
            },
            Node::Unary(UnaryOp::Neg | UnaryOp::Pos, inner) => collect!(inner),
            Node::Unary(UnaryOp::Not | UnaryOp::BitNot | UnaryOp::ToInteger, _) => {}
            Node::Conditional(condition, then_expr, else_expr) => {
                // The predicate selects a derivative branch but is not itself
                // differentiated. Match `simplify`'s constant-branch fold.
                match simplified_constant(arena, condition, constants) {
                    SimplifiedConstant::Value(value) if value != 0.0 => collect!(then_expr),
                    SimplifiedConstant::Value(_) => collect!(else_expr),
                    SimplifiedConstant::Other => {
                        collect!(then_expr);
                        collect!(else_expr);
                    }
                }
            }
            Node::Call { func, argc, a, b } => match (func, argc, a, b) {
                (IrFunction::Floor | IrFunction::Ceil, 1, Some(_), _) => {}
                (
                    IrFunction::Abs
                    | IrFunction::Sqrt
                    | IrFunction::Exp
                    | IrFunction::LimitedExp
                    | IrFunction::Log
                    | IrFunction::Log10
                    | IrFunction::Sin
                    | IrFunction::Cos
                    | IrFunction::Tan
                    | IrFunction::Sinh
                    | IrFunction::Cosh
                    | IrFunction::Tanh
                    | IrFunction::Asin
                    | IrFunction::Acos
                    | IrFunction::Atan
                    | IrFunction::Asinh
                    | IrFunction::Acosh
                    | IrFunction::Atanh,
                    1,
                    Some(inner),
                    _,
                ) => collect!(inner),
                (
                    IrFunction::Min | IrFunction::Max | IrFunction::Hypot | IrFunction::Atan2,
                    2,
                    Some(left),
                    Some(right),
                ) => {
                    collect!(left);
                    collect!(right);
                }
                (IrFunction::Pow, 2, Some(base), Some(exponent)) => {
                    if let Node::Const(value) = *arena.node(exponent) {
                        if value != 0.0 {
                            collect!(base);
                        }
                    } else {
                        collect!(exponent);
                        if !is_zero!(exponent) {
                            collect!(base);
                        }
                    }
                }
                // Malformed calls differentiate to zero and are diagnosed by
                // their construction path.
                _ => {}
            },
            Node::CallSpilled {
                func: IrFunction::SumProductsDiv,
                args,
            } => {
                for argument in arena.call_args(args).to_vec() {
                    collect!(argument);
                }
            }
            Node::CallSpilled { .. } => {}
            Node::Limexp(inner) | Node::Ddt(inner) => collect!(inner),
            Node::Idt(inner, ic) => {
                collect!(inner);
                if let Some(ic) = ic {
                    collect!(ic);
                }
            }
            Node::Limit(inner, _)
            | Node::NamedLimit {
                proposed: inner, ..
            } => {
                collect!(inner);
                if family == AuxiliaryAxes::LimiterCorrection {
                    axes.insert(DerivativeWrt::LimiterCorrection);
                }
            }
            // Frozen history: no auxiliary axis reaches through it.
            Node::LimiterPrevious(_) => {}
            Node::IdtMod {
                expr: inner,
                modulus,
                payload,
            } => {
                collect!(inner);
                collect!(modulus);
                if let Some(ic) = arena.optional_pair(payload).0 {
                    collect!(ic);
                }
            }
            Node::DdtDerivative {
                input_derivative, ..
            } => collect!(input_derivative),
            Node::TableLookup { input, .. } => collect!(input),
            Node::Ddx { .. } => {
                // ddx is resolved along its solver axis before the outer noise
                // derivative. Preserve that ordering; walking the raw operand
                // would incorrectly retain noise which its ddx eliminates.
                let shadows = ShadowContext {
                    auxiliary_shadowed: deps.clone(),
                    ..ShadowContext::default()
                };
                let resolved = resolve_ddx(arena, expr, &shadows);
                axes.extend(expression_auxiliary_axes(arena, resolved, deps, family));
            }
            Node::Heavy(_, payload) => match arena.heavy(payload).clone() {
                Heavy::IntegralDerivative {
                    input_derivative,
                    ic_derivative,
                    modulus_derivative,
                    ..
                } => {
                    collect!(input_derivative);
                    collect!(ic_derivative);
                    if let Some(derivative) = modulus_derivative {
                        collect!(derivative);
                    }
                }
                Heavy::WhiteNoise { site, .. }
                | Heavy::FlickerNoise { site, .. }
                | Heavy::NoiseTable { site, .. } => {
                    let process = site.ordinal as usize;
                    if let AuxiliaryAxes::Noise(count) = family
                        && process < count
                    {
                        axes.insert(DerivativeWrt::Noise(process));
                    }
                }
                Heavy::LaplaceND { expr: inner, .. }
                | Heavy::LaplaceZP { expr: inner, .. }
                | Heavy::LaplaceNDDerivative { expr: inner, .. }
                | Heavy::LaplaceZPDerivative { expr: inner, .. }
                | Heavy::ZiFilter { expr: inner, .. }
                | Heavy::ZiFilterDerivative { expr: inner, .. }
                | Heavy::Transition { expr: inner, .. } => collect!(inner),
                Heavy::AbsDelay {
                    expr: input,
                    delay_time,
                    ..
                } => {
                    collect!(input);
                    collect!(delay_time);
                }
                Heavy::AbsDelayDerivative {
                    input_derivative,
                    delay_derivative,
                    ..
                } => {
                    collect!(input_derivative);
                    collect!(delay_derivative);
                }
                Heavy::TransitionDerivative {
                    input_derivative, ..
                } => collect!(input_derivative),
                Heavy::Slew {
                    expr: input,
                    max_pos_slew,
                    max_neg_slew,
                    ..
                } => {
                    collect!(input);
                    if let Some(rate) = max_pos_slew {
                        collect!(rate);
                    }
                    if let Some(rate) = max_neg_slew {
                        collect!(rate);
                    }
                }
                Heavy::SlewDerivative {
                    input_derivative,
                    max_pos_slew_derivative,
                    max_neg_slew_derivative,
                    ..
                } => {
                    collect!(input_derivative);
                    if let Some(rate) = max_pos_slew_derivative {
                        collect!(rate);
                    }
                    if let Some(rate) = max_neg_slew_derivative {
                        collect!(rate);
                    }
                }
                // Event detectors are constant on a noise realization axis.
                Heavy::Cross { .. } | Heavy::Above { .. } | Heavy::Timer { .. } => {}
            },
            // Solver probes, parameters, analysis/event queries, companion
            // derivative carriers, and table slopes are constant on a noise
            // realization axis.
            Node::Const(_)
            | Node::Param(_)
            | Node::ParamGiven(_)
            | Node::Voltage(_, _)
            | Node::Current(_, _)
            | Node::BranchCurrent(_)
            | Node::Time
            | Node::Temperature
            | Node::Vt
            | Node::SimParamValue(_)
            | Node::SimParamPresent(_)
            | Node::Mfactor
            | Node::PortConnected(_)
            | Node::LastCrossing { .. }
            | Node::Analysis(_)
            | Node::IdtCompanion(_)
            | Node::TableDerivative { .. } => {}
        }
    }

    fn expression_auxiliary_axes(
        arena: &mut ExprArena,
        expr: NodeId,
        deps: &HashMap<SmolStr, BTreeSet<DerivativeWrt>>,
        family: AuxiliaryAxes,
    ) -> BTreeSet<DerivativeWrt> {
        let mut constants = HashMap::new();
        let mut axes = BTreeSet::new();
        collect_expression_auxiliary_axes(arena, expr, deps, family, &mut constants, &mut axes);
        axes
    }

    fn scan_auxiliary_shadowed(
        arena: &mut ExprArena,
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        arrays: &[ArrayDef],
        family: AuxiliaryAxes,
        deps: &mut HashMap<SmolStr, BTreeSet<DerivativeWrt>>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { .. } => {}
                IrAssignmentItem::Task(_) => {}
                IrAssignmentItem::Assign(assign) => {
                    let axes = expression_auxiliary_axes(arena, assign.expr, deps, family);
                    if axes.is_empty() {
                        continue;
                    }
                    if let Some(array) = arrays.iter().find(|array| {
                        assign.var_index >= array.base && assign.var_index < array.base + array.len
                    }) {
                        let current = deps.get(&array.name).cloned().unwrap_or_default();
                        let mut merged = current.clone();
                        merged.extend(axes.iter().copied());
                        if merged != current {
                            deps.insert(array.name.clone(), merged.clone());
                            for index in array.lower..array.lower + array.len as i64 {
                                deps.insert(
                                    format!("{}[{index}]", array.name).into(),
                                    merged.clone(),
                                );
                            }
                            *changed = true;
                        }
                    } else if let Some(variable) = variables.get(assign.var_index) {
                        let current = deps.get(&variable.name).cloned().unwrap_or_default();
                        let mut merged = current.clone();
                        merged.extend(axes.iter().copied());
                        if merged != current {
                            deps.insert(variable.name.clone(), merged);
                            *changed = true;
                        }
                    }
                }
                IrAssignmentItem::Loop { body, .. } => {
                    scan_auxiliary_shadowed(arena, body, variables, arrays, family, deps, changed)
                }
            }
        }
    }

    fn interleave_auxiliary_shadows(
        arena: &mut ExprArena,
        items: Vec<IrAssignmentItem>,
        variables: &[VarDef],
        shadow_index: &HashMap<SmolStr, usize>,
        ctx: &ShadowContext,
        family: AuxiliaryAxes,
    ) -> Vec<IrAssignmentItem> {
        let mut rewritten = Vec::with_capacity(items.len().saturating_mul(2));
        for item in items {
            match item {
                initialization @ IrAssignmentItem::Initialization { .. } => {
                    rewritten.push(initialization)
                }
                IrAssignmentItem::Task(task) => rewritten.push(IrAssignmentItem::Task(task)),
                IrAssignmentItem::Assign(assign) => {
                    let target_name = assign
                        .index
                        .as_ref()
                        .map(|target| target.array.clone())
                        .or_else(|| variables.get(assign.var_index).map(|var| var.name.clone()));
                    if let Some(target_name) = target_name {
                        let processes = ctx
                            .auxiliary_axes_of(&target_name)
                            .into_iter()
                            .flatten()
                            .copied()
                            .filter(|axis| family.contains(*axis));
                        for axis in processes {
                            let derivative =
                                differentiate_with_shadows(arena, assign.expr, &axis, ctx);
                            let shadow_name = ShadowContext::shadow_name(&target_name, &axis);
                            if let Some(target) = &assign.index {
                                let shadow_base = ctx
                                    .array_shadow_base(&target.array, &axis)
                                    .expect("auxiliary-shadowed array has a contiguous run");
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_base,
                                    index: Some(IndexedTarget {
                                        array: shadow_name,
                                        len: target.len,
                                        lower: target.lower,
                                        index: target.index,
                                    }),
                                    expr: derivative,
                                }));
                            } else {
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_index[&shadow_name],
                                    index: None,
                                    expr: derivative,
                                }));
                            }
                        }
                    }
                    rewritten.push(IrAssignmentItem::Assign(assign));
                }
                IrAssignmentItem::Loop { condition, body } => {
                    rewritten.push(IrAssignmentItem::Loop {
                        condition,
                        body: interleave_auxiliary_shadows(
                            arena,
                            body,
                            variables,
                            shadow_index,
                            ctx,
                            family,
                        ),
                    });
                }
            }
        }
        rewritten
    }

    /// The auxiliary-shadowed variables of `ir`, each with the directions it
    /// carries, after the fixpoint over the assignments and the liveness cut
    /// from `shadow_roots`.
    ///
    /// Empty requires no assignment rewrite. For noise this also means the
    /// ordinary assignment pass can serve the noise evaluator without a copy.
    pub(crate) fn auxiliary_shadowed_dependencies(
        ir: &mut DeviceIR,
        family: AuxiliaryAxes,
        shadow_roots: &HashSet<SmolStr>,
    ) -> HashMap<SmolStr, BTreeSet<DerivativeWrt>> {
        let mut deps = HashMap::new();
        if family == AuxiliaryAxes::Noise(0) {
            return deps;
        }
        let DeviceIR {
            exprs,
            assignments,
            variables,
            arrays,
            ..
        } = ir;
        let span = crate::metrics::FineSpan::new("ir.auxiliary_axis_fixpoint");
        let mut passes = 0_usize;
        loop {
            let mut changed = false;
            passes += 1;
            scan_auxiliary_shadowed(
                exprs,
                assignments,
                variables,
                arrays,
                family,
                &mut deps,
                &mut changed,
            );
            if !changed {
                break;
            }
        }
        span.finish(&format!("passes={passes} shadowed={}", deps.len()));
        if deps.is_empty() {
            return deps;
        }
        let span = crate::metrics::FineSpan::new("ir.auxiliary_liveness");
        let liveness = LivenessGraph::build(exprs, assignments, variables, arrays);
        let live = liveness.live_from(shadow_roots);
        span.finish(&format!(
            "assignments={} live={}",
            assignments.len(),
            live.len()
        ));
        deps.retain(|name, _| live.contains(name));
        deps
    }

    /// Add the selected auxiliary direction shadows in `deps`. They have no
    /// physical matrix columns and do not enlarge the solver-axis layout.
    /// A later auxiliary pass retains earlier shadows but rewrites only its
    /// own family, preserving the separate numerical and noise schedules.
    pub(crate) fn build_auxiliary_shadow_assignments(
        ir: &mut DeviceIR,
        deps: HashMap<SmolStr, BTreeSet<DerivativeWrt>>,
        ctx: &mut ShadowContext,
        family: AuxiliaryAxes,
    ) {
        if deps.is_empty() {
            return;
        }
        let DeviceIR {
            exprs,
            assignments,
            variables,
            arrays,
            ..
        } = ir;

        let span = crate::metrics::FineSpan::new("ir.auxiliary_shadow_layout");
        let array_members = arrays
            .iter()
            .filter(|array| deps.get(&array.name).is_some_and(|axes| !axes.is_empty()))
            .flat_map(|array| {
                std::iter::once(array.name.clone()).chain(
                    variables[array.base..array.base + array.len]
                        .iter()
                        .map(|var| var.name.clone()),
                )
            })
            .collect::<HashSet<_>>();
        let mut shadow_index = HashMap::new();
        let mut scalar_layout = deps
            .iter()
            .filter(|(name, _)| !array_members.contains(*name))
            .map(|(name, axes)| (name.clone(), axes.clone()))
            .collect::<Vec<_>>();
        scalar_layout.sort_unstable_by(|left, right| left.0.cmp(&right.0));
        for (name, axes) in scalar_layout {
            for axis in axes {
                let shadow = ShadowContext::shadow_name(name.as_str(), &axis);
                shadow_index.insert(shadow.clone(), variables.len());
                variables.push(VarDef {
                    name: shadow,
                    is_state: false,
                });
            }
        }
        for array in arrays.iter() {
            let processes = deps.get(&array.name).cloned().unwrap_or_default();
            for axis in processes {
                let run_name = ShadowContext::shadow_name(&array.name, &axis);
                let run_base = variables.len();
                ctx.array_shadow_base.insert(run_name, run_base);
                for index in array.lower..array.lower + array.len as i64 {
                    let element = format!("{}[{index}]", array.name);
                    let shadow = ShadowContext::shadow_name(&element, &axis);
                    shadow_index.insert(shadow.clone(), variables.len());
                    variables.push(VarDef {
                        name: shadow,
                        is_state: false,
                    });
                }
            }
        }
        for (name, axes) in deps {
            ctx.auxiliary_shadowed.entry(name).or_default().extend(axes);
        }
        span.finish(&format!("shadow_slots={}", shadow_index.len()));
        let span = crate::metrics::FineSpan::new("ir.auxiliary_shadow_interleave");
        let originals = std::mem::take(assignments);
        *assignments =
            interleave_auxiliary_shadows(exprs, originals, variables, &shadow_index, ctx, family);
        span.finish(&format!("assignments={}", assignments.len()));
    }

    /// Resolve ddx() operators into explicit derivative expressions
    ///
    /// The replacement is handed back to [`rewrite`] as a node rather than as
    /// an id, so the resolved root is appended a second time — one extra node
    /// per `ddx` site, of which a module has a couple of dozen — and every
    /// subtree the walk did not touch keeps the id it had.
    pub fn resolve_ddx(arena: &mut ExprArena, expr: NodeId, shadows: &ShadowContext) -> NodeId {
        rewrite(arena, expr, &mut |arena, node| {
            let Node::Ddx { expr, axis } = node else {
                return None;
            };
            let axis = arena.ddx_axis(axis);
            let inner = resolve_ddx(arena, expr, shadows);
            let resolved = match axis {
                DdxAxis::Potential {
                    pos: Some(pos),
                    neg: None,
                } => {
                    differentiate_with_shadows(arena, inner, &DerivativeWrt::Voltage(pos), shadows)
                }
                DdxAxis::Potential {
                    pos: None,
                    neg: Some(neg),
                } => {
                    let derivative = differentiate_with_shadows(
                        arena,
                        inner,
                        &DerivativeWrt::Voltage(neg),
                        shadows,
                    );
                    let negated = arena.push(Node::Unary(UnaryOp::Neg, derivative));
                    simplify(arena, negated)
                }
                // ddx(f, V(a,b)): when f depends on the pair only
                // through V(a)-V(b), (df/dVa - df/dVb)/2 is exactly
                // df/d(Va-Vb).
                DdxAxis::Potential {
                    pos: Some(pos),
                    neg: Some(neg),
                } => {
                    let d_pos = differentiate_with_shadows(
                        arena,
                        inner,
                        &DerivativeWrt::Voltage(pos),
                        shadows,
                    );
                    let d_neg = differentiate_with_shadows(
                        arena,
                        inner,
                        &DerivativeWrt::Voltage(neg),
                        shadows,
                    );
                    let half = arena.push(Node::Const(0.5));
                    let difference = arena.push(Node::Binary(BinaryOp::Sub, d_pos, d_neg));
                    let scaled = arena.push(Node::Binary(BinaryOp::Mul, half, difference));
                    simplify(arena, scaled)
                }
                DdxAxis::Potential {
                    pos: None,
                    neg: None,
                } => arena.push(Node::Const(0.0)),
                DdxAxis::BranchCurrent { ordinal, reversed } => {
                    let derivative = differentiate_with_shadows(
                        arena,
                        inner,
                        &DerivativeWrt::BranchCurrent(ordinal),
                        shadows,
                    );
                    if reversed {
                        let negated = arena.push(Node::Unary(UnaryOp::Neg, derivative));
                        simplify(arena, negated)
                    } else {
                        derivative
                    }
                }
            };
            Some(Node::Binary(BinaryOp::CheckedValue, inner, resolved))
        })
    }

    /// Resolve every `ddx` in an assignment tree.
    ///
    /// Each rewrite goes through [`rewrite`], which rebuilds every node on a
    /// path that changed. Almost none of these expressions hold a `ddx` —
    /// after shadow interleaving the tree is a million derivative assignments
    /// and the operators are the couple of dozen sites the author wrote — so
    /// asking [`contains_ddx`] first replaces that walk with a read.
    pub fn resolve_ddx_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        shadows: &ShadowContext,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    resolve_ddx_in_items(arena, body, shadows)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        if contains_ddx(arena, *expression) {
                            *expression = resolve_ddx(arena, *expression, shadows);
                        }
                    }
                }
                IrAssignmentItem::Assign(assign) => {
                    if contains_ddx(arena, assign.expr) {
                        assign.expr = resolve_ddx(arena, assign.expr, shadows);
                    }
                    if let Some(target) = &mut assign.index
                        && contains_ddx(arena, target.index)
                    {
                        target.index = resolve_ddx(arena, target.index, shadows);
                    }
                }
                IrAssignmentItem::Loop { condition, body } => {
                    if contains_ddx(arena, *condition) {
                        *condition = resolve_ddx(arena, *condition, shadows);
                    }
                    resolve_ddx_in_items(arena, body, shadows);
                }
            }
        }
    }

    /// Whether a subtree holds a `ddx` operator.
    ///
    /// [`resolve_ddx`] rewrites through [`rewrite`], which rebuilds every node
    /// on a path it changes. A module's `ddx` operators are a couple of dozen
    /// sites in a program of a million assignments, so asking first turns a
    /// whole-program rebuild into a whole-program read.
    pub(crate) fn contains_ddx(arena: &ExprArena, expr: NodeId) -> bool {
        let mut found = false;
        visit(arena, expr, &mut |node| {
            if matches!(node, Node::Ddx { .. }) {
                found = true;
            }
        });
        found
    }

    /// Number the `zi_filter` sites of one tree in the order they are walked.
    ///
    /// # These five walks unfold, and that is the whole of their correctness
    ///
    /// A site's ordinal is its position in a **path** walk, not in the DAG:
    /// where the producer names one arena node from two places, each path
    /// through it is its own site and takes its own ordinal, exactly as the two
    /// `Box` trees the boxed converter cloned did. That is why these recurse
    /// through [`arena::rebuild_children`] instead of memoizing by [`NodeId`] —
    /// a memo would hand a shared `transition` one ordinal where the emitted
    /// program needs two, and would then renumber every site after it.
    ///
    /// The descent stops where the boxed `map_expr` stopped: a matched node's
    /// children are **not** walked, so a `zi_filter` nested in another one's
    /// operand takes no ordinal. [`assign_laplace_site_ordinals`] is the one
    /// exception and says so.
    pub(crate) fn assign_zi_site_ordinals(
        arena: &mut ExprArena,
        id: NodeId,
        next: &mut u32,
    ) -> NodeId {
        let node = *arena.node(id);
        if let Node::Heavy(HeavyKind::ZiFilter, heavy) = node {
            let mut updated = arena.heavy(heavy).clone();
            if let Heavy::ZiFilter { site, .. } = &mut updated {
                site.ordinal = *next;
                *next = next.checked_add(1).expect("Zi site ordinal overflow");
            }
            return arena.push_heavy(updated);
        }
        arena::rebuild_children(arena, id, node, &mut |arena, child| {
            assign_zi_site_ordinals(arena, child, next)
        })
    }

    pub(crate) fn assign_zi_site_ordinals_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    assign_zi_site_ordinals_in_items(arena, body, next)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        *expression = assign_zi_site_ordinals(arena, *expression, next);
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    assignment.expr = assign_zi_site_ordinals(arena, assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = assign_zi_site_ordinals(arena, *condition, next);
                    assign_zi_site_ordinals_in_items(arena, body, next);
                }
            }
        }
    }

    /// Number the Laplace sites of one tree, descending into a numbered one.
    ///
    /// The one walk of the five that enters a matched node: the boxed closure
    /// recursed into the filtered expression by hand, so a `laplace_*` nested
    /// in another one's operand *does* take an ordinal, and takes it
    /// immediately after its parent. The coefficient and root lists are numbers
    /// and are not walked at all.
    pub(crate) fn assign_laplace_site_ordinals(
        arena: &mut ExprArena,
        id: NodeId,
        next: &mut u32,
    ) -> NodeId {
        let node = *arena.node(id);
        if let Node::Heavy(HeavyKind::LaplaceND | HeavyKind::LaplaceZP, heavy) = node {
            let mut updated = arena.heavy(heavy).clone();
            match &mut updated {
                Heavy::LaplaceND { site, expr, .. } | Heavy::LaplaceZP { site, expr, .. } => {
                    site.ordinal = *next;
                    *next = next.checked_add(1).expect("Laplace site ordinal overflow");
                    *expr = assign_laplace_site_ordinals(arena, *expr, next);
                }
                _ => unreachable!("a Laplace kind carries a Laplace payload"),
            }
            return arena.push_heavy(updated);
        }
        arena::rebuild_children(arena, id, node, &mut |arena, child| {
            assign_laplace_site_ordinals(arena, child, next)
        })
    }

    pub(crate) fn assign_laplace_site_ordinals_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    assign_laplace_site_ordinals_in_items(arena, body, next)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        *expression = assign_laplace_site_ordinals(arena, *expression, next);
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    assignment.expr = assign_laplace_site_ordinals(arena, assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = assign_laplace_site_ordinals(arena, *condition, next);
                    assign_laplace_site_ordinals_in_items(arena, body, next);
                }
            }
        }
    }

    /// Number the `slew` sites of one tree; see [`assign_zi_site_ordinals`].
    pub(crate) fn assign_slew_site_ordinals(
        arena: &mut ExprArena,
        id: NodeId,
        next: &mut u32,
    ) -> NodeId {
        let node = *arena.node(id);
        if let Node::Heavy(HeavyKind::Slew, heavy) = node {
            let mut updated = arena.heavy(heavy).clone();
            if let Heavy::Slew { site, .. } = &mut updated {
                site.ordinal = *next;
                *next = next.checked_add(1).expect("slew site ordinal overflow");
            }
            return arena.push_heavy(updated);
        }
        arena::rebuild_children(arena, id, node, &mut |arena, child| {
            assign_slew_site_ordinals(arena, child, next)
        })
    }

    pub(crate) fn assign_slew_site_ordinals_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    assign_slew_site_ordinals_in_items(arena, body, next)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        *expression = assign_slew_site_ordinals(arena, *expression, next);
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    assignment.expr = assign_slew_site_ordinals(arena, assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = assign_slew_site_ordinals(arena, *condition, next);
                    assign_slew_site_ordinals_in_items(arena, body, next);
                }
            }
        }
    }

    /// Number the `transition` sites of one tree; see
    /// [`assign_zi_site_ordinals`].
    pub(crate) fn assign_transition_site_ordinals(
        arena: &mut ExprArena,
        id: NodeId,
        next: &mut u32,
    ) -> NodeId {
        let node = *arena.node(id);
        if let Node::Heavy(HeavyKind::Transition, heavy) = node {
            let mut updated = arena.heavy(heavy).clone();
            if let Heavy::Transition { site, .. } = &mut updated {
                site.ordinal = *next;
                *next = next
                    .checked_add(1)
                    .expect("transition site ordinal overflow");
            }
            return arena.push_heavy(updated);
        }
        arena::rebuild_children(arena, id, node, &mut |arena, child| {
            assign_transition_site_ordinals(arena, child, next)
        })
    }

    pub(crate) fn assign_transition_site_ordinals_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    assign_transition_site_ordinals_in_items(arena, body, next)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        *expression = assign_transition_site_ordinals(arena, *expression, next);
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    assignment.expr = assign_transition_site_ordinals(arena, assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = assign_transition_site_ordinals(arena, *condition, next);
                    assign_transition_site_ordinals_in_items(arena, body, next);
                }
            }
        }
    }

    /// Number the `absdelay` sites of one tree; see
    /// [`assign_zi_site_ordinals`].
    pub(crate) fn assign_absdelay_site_ordinals(
        arena: &mut ExprArena,
        id: NodeId,
        next: &mut u32,
    ) -> NodeId {
        let node = *arena.node(id);
        if let Node::Heavy(HeavyKind::AbsDelay, heavy) = node {
            let mut updated = arena.heavy(heavy).clone();
            if let Heavy::AbsDelay { site, .. } = &mut updated {
                site.ordinal = *next;
                *next = next.checked_add(1).expect("absdelay site ordinal overflow");
            }
            return arena.push_heavy(updated);
        }
        arena::rebuild_children(arena, id, node, &mut |arena, child| {
            assign_absdelay_site_ordinals(arena, child, next)
        })
    }

    pub(crate) fn assign_absdelay_site_ordinals_in_items(
        arena: &mut ExprArena,
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Initialization { body, .. } => {
                    assign_absdelay_site_ordinals_in_items(arena, body, next)
                }
                IrAssignmentItem::Task(task) => {
                    for expression in task.expressions_mut() {
                        *expression = assign_absdelay_site_ordinals(arena, *expression, next);
                    }
                }
                IrAssignmentItem::Assign(assignment) => {
                    assignment.expr = assign_absdelay_site_ordinals(arena, assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = assign_absdelay_site_ordinals(arena, *condition, next);
                    assign_absdelay_site_ordinals_in_items(arena, body, next);
                }
            }
        }
    }

    /// Differentiate an expression with respect to a variable
    /// (without assignment-chain shadows; prefer
    /// [`differentiate_with_shadows`] when a chain context exists)
    pub fn differentiate(arena: &mut ExprArena, expr: NodeId, wrt: &DerivativeWrt) -> NodeId {
        differentiate_with_shadows(arena, expr, wrt, &ShadowContext::default())
    }

    /// Differentiate an expression, chaining through shadowed variables
    ///
    /// # The primal operands are named, not copied
    ///
    /// Every rule that used to write `left.clone()` writes `left`: the
    /// derivative and the primal share one arena node. That is what makes this
    /// the memory step — more than half of every shipped model's shadow forest
    /// was a verbatim copy of a primal subtree — and it changes no emitted
    /// program, because every consumer *unfolds*: a shared subtree is walked
    /// once per path and the emitter allocates its per-emission state slot at
    /// the visit, so a shared `ddt` still takes one slot per occurrence in the
    /// program exactly as a copied one did.
    ///
    /// Nothing here may memoize by [`NodeId`] for the same reason a site walk
    /// may not: the result is stored once but read along every path, and the
    /// identity that matters is the unfolded one.
    pub fn differentiate_with_shadows(
        arena: &mut ExprArena,
        expr: NodeId,
        wrt: &DerivativeWrt,
        shadows: &ShadowContext,
    ) -> NodeId {
        let primal_len = arena.len();
        let derivative = differentiate_raw(arena, expr, wrt, shadows);
        simplify_from(arena, derivative, Some(primal_len))
    }

    fn differentiate_raw(
        arena: &mut ExprArena,
        expr: NodeId,
        wrt: &DerivativeWrt,
        shadows: &ShadowContext,
    ) -> NodeId {
        let primal_len = arena.len();
        macro_rules! constant {
            ($value:expr) => {
                arena.push(Node::Const($value))
            };
        }
        macro_rules! binary {
            ($op:expr, $left:expr, $right:expr) => {
                arena.push(Node::Binary($op, $left, $right))
            };
        }
        macro_rules! differentiate {
            ($child:expr) => {
                differentiate_raw(arena, $child, wrt, shadows)
            };
        }

        match *arena.node(expr) {
            Node::Const(_) => constant!(0.0),
            Node::FreezeDerivative(_) => constant!(0.0),

            Node::Voltage(pos, neg) => {
                let pos = unpack_index(pos);
                let neg = unpack_index(neg);
                if let DerivativeWrt::Voltage(v) = wrt {
                    constant!(f64::from(i8::from(*v == pos) - i8::from(*v == neg)))
                } else {
                    constant!(0.0)
                }
            }

            // Chain rule through intermediate variables: the shadow
            // variable carries the derivative along the active axis. A
            // variable that cannot vary along this axis differentiates to
            // zero without a shadow slot ever existing.
            Node::Var(name) => {
                let name = arena.name(name).clone();
                if shadows.is_shadowed_on(&name, wrt) {
                    let shadow = ShadowContext::shadow_name(&name, wrt);
                    let interned = arena.intern(&shadow);
                    arena.push(Node::Var(interned))
                } else {
                    constant!(0.0)
                }
            }

            // Runtime-indexed reads chain through the array's shadow run
            // at the same element; the index itself only selects
            Node::VarIndexed { payload, index } => {
                let read = *arena.indexed(payload);
                let array = arena.name(read.array).clone();
                match shadows.array_shadow_base(&array, wrt) {
                    Some(shadow_base) => {
                        let shadow = ShadowContext::shadow_name(&array, wrt);
                        let interned = arena.intern(&shadow);
                        let payload = arena.push_indexed(IndexedRead {
                            array: interned,
                            base: shadow_base,
                            len: read.len,
                            lower: read.lower,
                        });
                        arena.push(Node::VarIndexed { payload, index })
                    }
                    None => constant!(0.0),
                }
            }

            // Branch-current unknowns differentiate to 1 along their own
            // axis and 0 along every other
            Node::BranchCurrent(ordinal) => {
                let ordinal = unpack_index(ordinal);
                match wrt {
                    DerivativeWrt::BranchCurrent(k) if *k == ordinal => constant!(1.0),
                    _ => constant!(0.0),
                }
            }

            Node::Param(_)
            | Node::ParamGiven(_)
            | Node::Temperature
            | Node::Vt
            | Node::Time
            | Node::SimParamValue(_)
            | Node::SimParamPresent(_)
            | Node::Mfactor
            | Node::PortConnected(_) => constant!(0.0),

            Node::Binary(op, left, right) => {
                let dl = differentiate!(left);
                let dr = differentiate!(right);

                match op {
                    BinaryOp::Add => binary!(BinaryOp::Add, dl, dr),
                    BinaryOp::Sub => binary!(BinaryOp::Sub, dl, dr),
                    BinaryOp::Mul => {
                        // Product rule: d(f*g) = f'*g + f*g'
                        let from_left = binary!(BinaryOp::Mul, dl, right);
                        let from_right = binary!(BinaryOp::Mul, left, dr);
                        binary!(BinaryOp::Add, from_left, from_right)
                    }
                    BinaryOp::Div => {
                        let quotient = binary!(BinaryOp::Div, left, right);
                        let negative = arena.push(Node::Unary(UnaryOp::Neg, quotient));
                        let one = constant!(1.0);
                        arena.push_call(IrFunction::SumProductsDiv, &[dl, one, negative, dr, right])
                    }
                    BinaryOp::Pow => {
                        // d(u^v) =
                        //   if v is const c: c*u^(c-1)*u'
                        //   else: v*u^(v-1)*u' + u^v*ln(u)*v'
                        //
                        // The two terms are written separately rather than as
                        // the factored `u^v * (v'*ln u + v*u'/u)`, which is
                        // singular at `u = 0` on EVERY axis: the quotient
                        // divides by zero for the base term as well, so a model
                        // evaluated at zero bias got NaN derivatives along
                        // axes whose `u'` is zero. The separated form is the
                        // one the canonical CFG pass builds
                        // (`canonical_ir/ad.rs`), so the two routes now compute
                        // the same expression and the finite oracle compares
                        // them instead of skipping a non-finite reference.
                        match *arena.node(right) {
                            Node::Const(c) => {
                                let base = power_rule_base_term_base(arena, left, Some(c));
                                let reduced_exponent = constant!(c - 1.0);
                                let u_pow = binary!(BinaryOp::Pow, base, reduced_exponent);
                                let scale = constant!(c);
                                let term = binary!(BinaryOp::Mul, u_pow, dl);
                                binary!(BinaryOp::Mul, scale, term)
                            }
                            _ => {
                                let base = power_rule_base_term_base(arena, left, None);
                                let one = constant!(1.0);
                                let reduced_exponent = binary!(BinaryOp::Sub, right, one);
                                let reduced = binary!(BinaryOp::Pow, base, reduced_exponent);
                                let scaled = binary!(BinaryOp::Mul, right, reduced);
                                let from_base = binary!(BinaryOp::Mul, dl, scaled);
                                let power = binary!(BinaryOp::Pow, left, right);
                                let log = power_rule_guarded_log(arena, left);
                                let logged = binary!(BinaryOp::Mul, power, log);
                                let from_exponent = binary!(BinaryOp::Mul, dr, logged);
                                binary!(BinaryOp::Add, from_base, from_exponent)
                            }
                        }
                    }
                    BinaryOp::CheckedValue => dr,
                    BinaryOp::Mod => {
                        // The integer quotient is locally constant even when
                        // both real operands vary: d(l % r) = dl - trunc(l/r)*dr.
                        let dr = simplify_from(arena, dr, Some(primal_len));
                        if matches!(arena.node(dr), Node::Const(value) if *value == 0.0) {
                            return dl;
                        }
                        let quotient = binary!(BinaryOp::Div, left, right);
                        let zero = constant!(0.0);
                        let negative = binary!(BinaryOp::Lt, quotient, zero);
                        let ceil = arena.push_call(IrFunction::Ceil, &[quotient]);
                        let floor = arena.push_call(IrFunction::Floor, &[quotient]);
                        let whole = arena.push(Node::Conditional(negative, ceil, floor));
                        let scaled = binary!(BinaryOp::Mul, whole, dr);
                        binary!(BinaryOp::Sub, dl, scaled)
                    }
                    // Piecewise-constant or discontinuous operators are treated
                    // as zero derivative in the DC Jacobian.
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::IntAdd
                    | BinaryOp::IntSub
                    | BinaryOp::IntMul
                    | BinaryOp::IntDiv
                    | BinaryOp::IntMod
                    | BinaryOp::IntPow
                    | BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => constant!(0.0),
                }
            }

            Node::Unary(UnaryOp::Neg, inner) => {
                let di = differentiate!(inner);
                arena.push(Node::Unary(UnaryOp::Neg, di))
            }
            // Unary plus is the identity
            Node::Unary(UnaryOp::Pos, inner) => differentiate!(inner),
            // Logical/bitwise negation is piecewise constant
            Node::Unary(UnaryOp::Not | UnaryOp::BitNot | UnaryOp::ToInteger, _) => constant!(0.0),

            // d(c ? a : b) = c ? da : db
            Node::Conditional(condition, then_expr, else_expr) => {
                let dt = differentiate!(then_expr);
                let de = differentiate!(else_expr);
                arena.push(Node::Conditional(condition, dt, de))
            }

            Node::Call {
                func,
                argc: 1,
                a: Some(inner),
                ..
            } => {
                let di = differentiate!(inner);

                // Chain rule: d(f(g)) = f'(g) * g'
                let outer_deriv = match func {
                    IrFunction::Abs => {
                        let zero = constant!(0.0);
                        let nonneg = binary!(BinaryOp::Ge, inner, zero);
                        let one = constant!(1.0);
                        let minus_one = constant!(-1.0);
                        arena.push(Node::Conditional(nonneg, one, minus_one))
                    }
                    IrFunction::Exp => arena.push_call(IrFunction::Exp, &[inner]),
                    IrFunction::LimitedExp => limited_exp_derivative_scale(arena, inner),
                    IrFunction::Log => {
                        let one = constant!(1.0);
                        binary!(BinaryOp::Div, one, inner)
                    }
                    IrFunction::Log10 => {
                        let one = constant!(1.0);
                        let ln10 = constant!(std::f64::consts::LN_10);
                        let scaled = binary!(BinaryOp::Mul, inner, ln10);
                        binary!(BinaryOp::Div, one, scaled)
                    }
                    IrFunction::Sqrt => {
                        let half = constant!(0.5);
                        let root = arena.push_call(IrFunction::Sqrt, &[inner]);
                        binary!(BinaryOp::Div, half, root)
                    }
                    IrFunction::Sin => arena.push_call(IrFunction::Cos, &[inner]),
                    IrFunction::Cos => {
                        let sin = arena.push_call(IrFunction::Sin, &[inner]);
                        arena.push(Node::Unary(UnaryOp::Neg, sin))
                    }
                    IrFunction::Tan => {
                        let one = constant!(1.0);
                        let cos = arena.push_call(IrFunction::Cos, &[inner]);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, cos, two);
                        binary!(BinaryOp::Div, one, squared)
                    }
                    IrFunction::Sinh => arena.push_call(IrFunction::Cosh, &[inner]),
                    IrFunction::Cosh => arena.push_call(IrFunction::Sinh, &[inner]),
                    IrFunction::Tanh => {
                        let one = constant!(1.0);
                        let cosh = arena.push_call(IrFunction::Cosh, &[inner]);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, cosh, two);
                        binary!(BinaryOp::Div, one, squared)
                    }
                    IrFunction::Asin => {
                        let one = constant!(1.0);
                        let outer_one = constant!(1.0);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, inner, two);
                        let complement = binary!(BinaryOp::Sub, outer_one, squared);
                        let root = arena.push_call(IrFunction::Sqrt, &[complement]);
                        binary!(BinaryOp::Div, one, root)
                    }
                    IrFunction::Acos => {
                        let one = constant!(1.0);
                        let outer_one = constant!(1.0);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, inner, two);
                        let complement = binary!(BinaryOp::Sub, outer_one, squared);
                        let root = arena.push_call(IrFunction::Sqrt, &[complement]);
                        let quotient = binary!(BinaryOp::Div, one, root);
                        arena.push(Node::Unary(UnaryOp::Neg, quotient))
                    }
                    IrFunction::Atan => {
                        let one = constant!(1.0);
                        let outer_one = constant!(1.0);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, inner, two);
                        let sum = binary!(BinaryOp::Add, outer_one, squared);
                        binary!(BinaryOp::Div, one, sum)
                    }
                    IrFunction::Asinh => {
                        let one = constant!(1.0);
                        let outer_one = constant!(1.0);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, inner, two);
                        let sum = binary!(BinaryOp::Add, outer_one, squared);
                        let root = arena.push_call(IrFunction::Sqrt, &[sum]);
                        binary!(BinaryOp::Div, one, root)
                    }
                    IrFunction::Acosh => {
                        let one = constant!(1.0);
                        let lower_one = constant!(1.0);
                        let below = binary!(BinaryOp::Sub, inner, lower_one);
                        let below_root = arena.push_call(IrFunction::Sqrt, &[below]);
                        let upper_one = constant!(1.0);
                        let above = binary!(BinaryOp::Add, inner, upper_one);
                        let above_root = arena.push_call(IrFunction::Sqrt, &[above]);
                        let product = binary!(BinaryOp::Mul, below_root, above_root);
                        binary!(BinaryOp::Div, one, product)
                    }
                    IrFunction::Atanh => {
                        let one = constant!(1.0);
                        let outer_one = constant!(1.0);
                        let two = constant!(2.0);
                        let squared = binary!(BinaryOp::Pow, inner, two);
                        let complement = binary!(BinaryOp::Sub, outer_one, squared);
                        binary!(BinaryOp::Div, one, complement)
                    }
                    IrFunction::Floor | IrFunction::Ceil => constant!(0.0),
                    _ => return constant!(0.0),
                };

                binary!(BinaryOp::Mul, outer_deriv, di)
            }
            Node::Call {
                func: IrFunction::Atan2,
                argc: 2,
                a: Some(y),
                b: Some(x),
            } => {
                let dy = differentiate!(y);
                let dx = differentiate!(x);
                let (scale, y, x) = normalized_coordinates(arena, y, x);
                let from_ordinate = binary!(BinaryOp::Mul, x, dy);
                let from_abscissa = binary!(BinaryOp::Mul, y, dx);
                let num = binary!(BinaryOp::Sub, from_ordinate, from_abscissa);
                let x_squared = binary!(BinaryOp::Mul, x, x);
                let y_squared = binary!(BinaryOp::Mul, y, y);
                let den = binary!(BinaryOp::Add, x_squared, y_squared);
                let normalized = binary!(BinaryOp::Div, num, den);
                binary!(BinaryOp::Div, normalized, scale)
            }
            Node::Call {
                func: IrFunction::Hypot,
                argc: 2,
                a: Some(left),
                b: Some(right),
            } => {
                let dl = differentiate!(left);
                let dr = differentiate!(right);
                let (_, left, right) = normalized_coordinates(arena, left, right);
                let magnitude = arena.push_call(IrFunction::Hypot, &[left, right]);
                let left_factor = binary!(BinaryOp::Div, left, magnitude);
                let right_factor = binary!(BinaryOp::Div, right, magnitude);
                let from_left = binary!(BinaryOp::Mul, dl, left_factor);
                let from_right = binary!(BinaryOp::Mul, dr, right_factor);
                binary!(BinaryOp::Add, from_left, from_right)
            }
            Node::Call {
                func: IrFunction::Pow,
                argc: 2,
                a: Some(base),
                b: Some(exponent),
            } => {
                let as_binary = binary!(BinaryOp::Pow, base, exponent);
                differentiate!(as_binary)
            }
            Node::Call {
                func: IrFunction::Min | IrFunction::Max,
                argc: 2,
                a: Some(left),
                b: Some(right),
            } => {
                // The selected primal identifies its tangent even when the
                // other operand is NaN. Equal finite operands take the left.
                let condition = binary!(BinaryOp::Eq, expr, left);
                let dl = differentiate!(left);
                let dr = differentiate!(right);
                arena.push(Node::Conditional(condition, dl, dr))
            }

            Node::CallSpilled {
                func: IrFunction::SumProductsDiv,
                args,
            } => {
                let arguments = arena.call_args(args).to_vec();
                let terms = (arguments.len() - 1) / 2;
                let divisor = arguments[2 * terms];
                let mut derivatives = Vec::with_capacity(4 * terms + 3);
                for pair in arguments[..2 * terms].chunks_exact(2) {
                    let da = differentiate!(pair[0]);
                    let db = differentiate!(pair[1]);
                    derivatives.extend([da, pair[1], pair[0], db]);
                }
                let negative = arena.push(Node::Unary(UnaryOp::Neg, expr));
                let dd = differentiate!(divisor);
                derivatives.extend([negative, dd, divisor]);
                arena.push_call(IrFunction::SumProductsDiv, &derivatives)
            }

            // d(limexp(x)) = limexp(x) * x' (same as exp, but clamped). The
            // primal node itself is the factor the rule names.
            Node::Limexp(inner) => {
                let di = differentiate!(inner);
                binary!(BinaryOp::Mul, expr, di)
            }

            // The first candidate's synthetic history follows its input.
            // Preserve the primal site so the tangent uses the same history.
            Node::Ddt(inner) => {
                let input_derivative = differentiate!(inner);
                arena.push(Node::DdtDerivative {
                    primal: expr,
                    input_derivative,
                })
            }
            Node::DdtDerivative {
                primal,
                input_derivative,
            } => {
                let input_derivative = differentiate!(input_derivative);
                arena.push(Node::DdtDerivative {
                    primal,
                    input_derivative,
                })
            }

            // Retain the primal site: initialization and the local wrap count
            // are properties of this candidate, not global timestep factors.
            Node::Idt(inner, ic) => {
                let input_derivative = differentiate!(inner);
                let ic_derivative = ic
                    .map(|ic| differentiate!(ic))
                    .unwrap_or_else(|| arena.push(Node::Const(0.0)));
                arena.push_heavy(Heavy::IntegralDerivative {
                    primal: expr,
                    input_derivative,
                    ic_derivative,
                    modulus_derivative: None,
                })
            }
            Node::IdtMod {
                expr: inner,
                modulus,
                payload,
            } => {
                let (ic, _) = arena.optional_pair(payload);
                let input_derivative = differentiate!(inner);
                let ic_derivative = ic
                    .map(|ic| differentiate!(ic))
                    .unwrap_or_else(|| arena.push(Node::Const(0.0)));
                let modulus_derivative = Some(differentiate!(modulus));
                arena.push_heavy(Heavy::IntegralDerivative {
                    primal: expr,
                    input_derivative,
                    ic_derivative,
                    modulus_derivative,
                })
            }

            // Physical/noise tangents pass through the proposal. The separate
            // affine direction also carries the local limiter displacement.
            Node::Limit(inner, _)
            | Node::NamedLimit {
                proposed: inner, ..
            } => {
                let base = differentiate!(inner);
                if *wrt == DerivativeWrt::LimiterCorrection {
                    let displacement = binary!(BinaryOp::Sub, expr, inner);
                    binary!(BinaryOp::Add, base, displacement)
                } else {
                    base
                }
            }

            // The previous Newton iterate is history the solve cannot move.
            Node::LimiterPrevious(_) => arena.push(Node::Const(0.0)),

            // Table lookup: slope of the active segment times the inner
            // derivative
            Node::TableLookup { input, table } => {
                let slope = arena.push(Node::TableDerivative { input, table });
                let di = differentiate!(input);
                binary!(BinaryOp::Mul, slope, di)
            }

            Node::Heavy(_, payload) => {
                let heavy = arena.heavy(payload).clone();
                match heavy {
                    Heavy::IntegralDerivative {
                        primal,
                        input_derivative,
                        ic_derivative,
                        modulus_derivative,
                    } => {
                        let input_derivative = differentiate!(input_derivative);
                        let ic_derivative = differentiate!(ic_derivative);
                        let modulus_derivative =
                            modulus_derivative.map(|derivative| differentiate!(derivative));
                        arena.push_heavy(Heavy::IntegralDerivative {
                            primal,
                            input_derivative,
                            ic_derivative,
                            modulus_derivative,
                        })
                    }
                    // Transport delay passes the DC small-signal through.
                    // Transition instead needs the exact
                    // accepted-state-dependent transient coefficient: zero on
                    // delayed/history-driven ramps and one only on an
                    // instantaneous direct candidate. Keep the primal operands
                    // and site correlated so the runtime can compute that
                    // coefficient read-only even if the derivative executes
                    // before the primal.
                    Heavy::AbsDelay {
                        site,
                        expr,
                        delay_time,
                        max_delay,
                    } => {
                        let input_derivative = differentiate!(expr);
                        let delay_derivative = differentiate!(delay_time);
                        arena.push_heavy(Heavy::AbsDelayDerivative {
                            site,
                            input: expr,
                            input_derivative,
                            delay_time,
                            delay_derivative,
                            max_delay,
                            derivative_order: 1,
                        })
                    }
                    Heavy::AbsDelayDerivative {
                        site,
                        input,
                        input_derivative,
                        delay_time,
                        delay_derivative,
                        max_delay,
                        derivative_order,
                    } => {
                        let second_input = differentiate!(input_derivative);
                        let second_delay = differentiate!(delay_derivative);
                        arena.push_heavy(Heavy::AbsDelayDerivative {
                            site,
                            input,
                            input_derivative: second_input,
                            delay_time,
                            delay_derivative: second_delay,
                            max_delay,
                            derivative_order: derivative_order.saturating_add(1),
                        })
                    }
                    Heavy::Transition {
                        site,
                        expr,
                        delay,
                        rise_time,
                        fall_time,
                    } => {
                        let input_derivative = differentiate!(expr);
                        arena.push_heavy(Heavy::TransitionDerivative {
                            site,
                            input: expr,
                            input_derivative,
                            delay,
                            rise_time,
                            fall_time,
                        })
                    }
                    Heavy::TransitionDerivative {
                        site,
                        input,
                        input_derivative,
                        delay,
                        rise_time,
                        fall_time,
                    } => {
                        let second = differentiate!(input_derivative);
                        arena.push_heavy(Heavy::TransitionDerivative {
                            site,
                            input,
                            input_derivative: second,
                            delay,
                            rise_time,
                            fall_time,
                        })
                    }

                    // `slew` has a branch-exact transient derivative: the first
                    // argument tracks directly when unsaturated, while a
                    // saturated candidate depends on the active rate operand
                    // and elapsed time.
                    Heavy::Slew {
                        site,
                        expr,
                        max_pos_slew,
                        max_neg_slew,
                    } => {
                        let input_derivative = differentiate!(expr);
                        let max_pos_slew_derivative = max_pos_slew.map(|rate| differentiate!(rate));
                        let max_neg_slew_derivative = max_neg_slew.map(|rate| differentiate!(rate));
                        arena.push_heavy(Heavy::SlewDerivative {
                            site,
                            input: expr,
                            input_derivative,
                            max_pos_slew,
                            max_pos_slew_derivative,
                            max_neg_slew,
                            max_neg_slew_derivative,
                        })
                    }
                    // The same read-only branch action also represents higher
                    // fixed-branch derivatives. Preserve the primal branch
                    // operands and differentiate only the derivative payloads.
                    // This avoids the incorrect assumption that a derivative of
                    // a slew Jacobian is always zero when a dynamic rate is
                    // nonlinear.
                    Heavy::SlewDerivative {
                        site,
                        input,
                        input_derivative,
                        max_pos_slew,
                        max_pos_slew_derivative,
                        max_neg_slew,
                        max_neg_slew_derivative,
                    } => {
                        let second_input = differentiate!(input_derivative);
                        let second_pos =
                            max_pos_slew_derivative.map(|derivative| differentiate!(derivative));
                        let second_neg =
                            max_neg_slew_derivative.map(|derivative| differentiate!(derivative));
                        arena.push_heavy(Heavy::SlewDerivative {
                            site,
                            input,
                            input_derivative: second_input,
                            max_pos_slew,
                            max_pos_slew_derivative: second_pos,
                            max_neg_slew,
                            max_neg_slew_derivative: second_neg,
                        })
                    }

                    // Sampled-data filters have a time-dependent exact
                    // Jacobian: H(1) in equilibrium, b0/a0 on an edge, and zero
                    // while holding. Differentiation is only run once per
                    // Jacobian axis in normal construction; the derivative form
                    // retains the schedule action if a transformed IR is
                    // differentiated again.
                    Heavy::ZiFilter {
                        site,
                        expr,
                        numerator,
                        denominator,
                        period,
                        transition,
                        first_transition,
                        direct_assignment,
                    }
                    | Heavy::ZiFilterDerivative {
                        site,
                        expr,
                        numerator,
                        denominator,
                        period,
                        transition,
                        first_transition,
                        direct_assignment,
                    } => {
                        let derivative = differentiate!(expr);
                        arena.push_heavy(Heavy::ZiFilterDerivative {
                            site,
                            expr: derivative,
                            numerator,
                            denominator,
                            period,
                            transition,
                            first_transition,
                            direct_assignment,
                        })
                    }

                    // Laplace derivatives retain the primal site's state
                    // action. Runtime selects DC gain or the active
                    // companion-rule input gain.
                    Heavy::LaplaceND {
                        site,
                        expr,
                        numerator,
                        denominator,
                    }
                    | Heavy::LaplaceNDDerivative {
                        site,
                        expr,
                        numerator,
                        denominator,
                    } => {
                        let derivative = differentiate!(expr);
                        arena.push_heavy(Heavy::LaplaceNDDerivative {
                            site,
                            expr: derivative,
                            numerator,
                            denominator,
                        })
                    }
                    Heavy::LaplaceZP {
                        site,
                        expr,
                        zeros,
                        poles,
                        gain,
                    }
                    | Heavy::LaplaceZPDerivative {
                        site,
                        expr,
                        zeros,
                        poles,
                        gain,
                    } => {
                        let derivative = differentiate!(expr);
                        arena.push_heavy(Heavy::LaplaceZPDerivative {
                            site,
                            expr: derivative,
                            zeros,
                            poles,
                            gain,
                        })
                    }

                    // A syntactic noise call is the unit realization of exactly
                    // one independent process. Its PSD operands are metadata,
                    // not part of the realization gain.
                    Heavy::WhiteNoise { site, .. }
                    | Heavy::FlickerNoise { site, .. }
                    | Heavy::NoiseTable { site, .. } => match wrt {
                        DerivativeWrt::Noise(process) if *process == site.ordinal as usize => {
                            constant!(1.0)
                        }
                        _ => constant!(0.0),
                    },

                    // Event detectors are treated as constants in the DC
                    // Jacobian
                    Heavy::Cross { .. } | Heavy::Above { .. } | Heavy::Timer { .. } => {
                        constant!(0.0)
                    }
                }
            }

            // Unresolved ddx: expand, then differentiate the expansion
            Node::Ddx { .. } => {
                let resolved = resolve_ddx(arena, expr, shadows);
                differentiate!(resolved)
            }

            // Analysis queries, current probes, last-crossing times, companion
            // carriers, table slopes and malformed calls are treated as
            // constants in the DC Jacobian
            _ => constant!(0.0),
        }
    }

    /// A positive auxiliary scale is held fixed under AD. Homogeneous norm
    /// and angle derivatives are independent of its value, at every order.
    fn normalized_coordinates(
        arena: &mut ExprArena,
        left: NodeId,
        right: NodeId,
    ) -> (NodeId, NodeId, NodeId) {
        let left_abs = arena.push_call(IrFunction::Abs, &[left]);
        let right_abs = arena.push_call(IrFunction::Abs, &[right]);
        let scale = arena.push_call(IrFunction::Max, &[left_abs, right_abs]);
        let scale = arena.push(Node::FreezeDerivative(scale));
        let left = arena.push(Node::Binary(BinaryOp::Div, left, scale));
        let right = arena.push(Node::Binary(BinaryOp::Div, right, scale));
        (scale, left, right)
    }

    /// The base the power rule's `u^(v−1)` factor is raised from: `u` itself
    /// wherever that factor is finite, and `u` nudged off exactly zero
    /// wherever it is not.
    ///
    /// `v · u^(v−1) · u'` is `∞ · 0 = NaN` at `u = 0` for every `v < 1`, and
    /// `u'` is numerically 0 exactly where this matters — an axis a merge
    /// keeps live whose taken arm does not carry it. Adding
    /// `(u == 0) · MIN_POSITIVE` adds nothing at all to any other `u`, so the
    /// term stays bit-exact — including for a negative base under an integral
    /// exponent, where clamping with `max` would silently return the wrong
    /// derivative — and the factor becomes a large finite number at `u = 0`,
    /// so a zero derivative multiplies to exactly zero.
    ///
    /// `exponent` is the constant exponent when the rule knows one: a
    /// constant of 1 or more has no singularity to guard, and those are
    /// almost all of them, so their derivative programs are left exactly as
    /// they were. The canonical CFG pass reads the same constant and makes
    /// the same choice (`canonical_ir/ad.rs`).
    fn power_rule_base_term_base(
        arena: &mut ExprArena,
        base: NodeId,
        exponent: Option<f64>,
    ) -> NodeId {
        if exponent.is_some_and(|value| value >= 1.0) {
            return base;
        }
        let zero = arena.push(Node::Const(0.0));
        let is_zero = arena.push(Node::Binary(BinaryOp::Eq, base, zero));
        let smallest = arena.push(Node::Const(f64::MIN_POSITIVE));
        let nudge = arena.push(Node::Binary(BinaryOp::Mul, is_zero, smallest));
        arena.push(Node::Binary(BinaryOp::Add, base, nudge))
    }

    /// `ln(u)` as the power rule's exponent term needs it: the logarithm where
    /// it exists, and exactly zero where it does not.
    ///
    /// `u^v · ln(u) · v'` is defined only for `u > 0`. At `u = 0` the power is
    /// 0 and the logarithm −∞, and IEEE makes their product NaN even when `v'`
    /// is 0 — which it numerically is wherever the exponent's dependence on the
    /// differentiation axis is only structural. Clamping the logarithm's
    /// argument to the smallest positive normal and masking the result by
    /// `u > 0` leaves the term exactly the textbook one wherever it exists and
    /// makes it exactly 0 at the boundary. This is the same guard, in the same
    /// shape, that the canonical CFG pass emits (`canonical_ir/ad.rs`), so the
    /// bytecode and native routes agree term by term.
    fn power_rule_guarded_log(arena: &mut ExprArena, base: NodeId) -> NodeId {
        let smallest = arena.push(Node::Const(f64::MIN_POSITIVE));
        let clamped = arena.push_call(IrFunction::Max, &[base, smallest]);
        let zero = arena.push(Node::Const(0.0));
        let positive = arena.push(Node::Binary(BinaryOp::Gt, base, zero));
        let log = arena.push_call(IrFunction::Log, &[clamped]);
        arena.push(Node::Binary(BinaryOp::Mul, log, positive))
    }

    fn limited_exp_derivative_scale(arena: &mut ExprArena, inner: NodeId) -> NodeId {
        const LIMIT: f64 = 80.0;
        let upper = arena.push(Node::Const(LIMIT));
        let high = arena.push(Node::Binary(BinaryOp::Gt, inner, upper));
        let lower = arena.push(Node::Const(-LIMIT));
        let low = arena.push(Node::Binary(BinaryOp::Lt, inner, lower));

        let saturated = arena.push(Node::Const(LIMIT.exp()));
        let zero = arena.push(Node::Const(0.0));
        let exp = arena.push_call(IrFunction::Exp, &[inner]);
        let below = arena.push(Node::Conditional(low, zero, exp));
        arena.push(Node::Conditional(high, saturated, below))
    }

    /// Rebuild a binary node only if an operand moved.
    ///
    /// Returning the original id when nothing changed is what keeps a rewrite
    /// from copying the parts of a forest it did not touch — the arena's
    /// spelling of leaving a `Box` alone.
    fn rebuilt_binary(
        arena: &mut ExprArena,
        original: NodeId,
        op: BinaryOp,
        left: NodeId,
        right: NodeId,
    ) -> NodeId {
        if let Node::Binary(_, old_left, old_right) = *arena.node(original)
            && old_left == left
            && old_right == right
        {
            return original;
        }
        arena.push(Node::Binary(op, left, right))
    }

    /// Simplify an IR expression (constant folding, identity removal)
    pub fn simplify(arena: &mut ExprArena, expr: NodeId) -> NodeId {
        simplify_from(arena, expr, None)
    }

    /// Sparse derivative zeros can erase inactive chain-rule terms, but that
    /// algebra must never rewrite an authored primal factor. Arena indices
    /// identify the original nodes without copying or annotating their trees.
    fn simplify_from(arena: &mut ExprArena, expr: NodeId, primal_len: Option<u32>) -> NodeId {
        if primal_len.is_some_and(|len| expr.index() < len) {
            return expr;
        }
        match *arena.node(expr) {
            Node::FreezeDerivative(input) => {
                let simplified = simplify_from(arena, input, primal_len);
                if matches!(arena.node(simplified), Node::Const(_)) {
                    simplified
                } else if simplified == input {
                    expr
                } else {
                    arena.push(Node::FreezeDerivative(simplified))
                }
            }
            Node::Binary(op, left, right) => {
                let simplified_left = simplify_from(arena, left, primal_len);
                let simplified_right = simplify_from(arena, right, primal_len);

                // Constant folding
                if let (Node::Const(l), Node::Const(r)) =
                    (*arena.node(simplified_left), *arena.node(simplified_right))
                {
                    let folded = match op {
                        BinaryOp::Add => Some(l + r),
                        BinaryOp::Sub => Some(l - r),
                        BinaryOp::Mul => Some(l * r),
                        BinaryOp::Div => Some(l / r),
                        BinaryOp::Pow => Some(l.powf(r)),
                        _ => None,
                    };
                    return match folded {
                        Some(value) => arena.push(Node::Const(value)),
                        None => rebuilt_binary(arena, expr, op, simplified_left, simplified_right),
                    };
                }

                // Fold explicit constant arithmetic first: a singular 0/0 is
                // still invalid when its numerator came from differentiation.
                let left_zero = primal_len.is_some_and(|len| simplified_left.index() >= len)
                    && matches!(*arena.node(simplified_left), Node::Const(value) if value == 0.0);
                let right_zero = primal_len.is_some_and(|len| simplified_right.index() >= len)
                    && matches!(*arena.node(simplified_right), Node::Const(value) if value == 0.0);
                match op {
                    BinaryOp::Mul if left_zero || right_zero => {
                        return arena.push(Node::Const(0.0));
                    }
                    BinaryOp::Div if left_zero => return arena.push(Node::Const(0.0)),
                    BinaryOp::Add if left_zero => return simplified_right,
                    BinaryOp::Add | BinaryOp::Sub if right_zero => return simplified_left,
                    _ => {}
                }

                // Identity rules
                match op {
                    BinaryOp::Add => {
                        if matches!(*arena.node(simplified_left), Node::Const(v) if v.to_bits() == (-0.0_f64).to_bits())
                        {
                            return simplified_right;
                        }
                        if matches!(*arena.node(simplified_right), Node::Const(v) if v.to_bits() == (-0.0_f64).to_bits())
                        {
                            return simplified_left;
                        }
                    }
                    BinaryOp::Sub => {
                        if matches!(*arena.node(simplified_right), Node::Const(v) if v.to_bits() == 0)
                        {
                            return simplified_left;
                        }
                    }
                    BinaryOp::Mul => {
                        if matches!(*arena.node(simplified_left), Node::Const(v) if v == 1.0) {
                            return simplified_right;
                        }
                        if matches!(*arena.node(simplified_right), Node::Const(v) if v == 1.0) {
                            return simplified_left;
                        }
                    }
                    BinaryOp::Div => {
                        if matches!(*arena.node(simplified_right), Node::Const(v) if v == 1.0) {
                            return simplified_left;
                        }
                    }
                    _ => {}
                }

                rebuilt_binary(arena, expr, op, simplified_left, simplified_right)
            }
            Node::Unary(op, operand) => {
                let simplified = simplify_from(arena, operand, primal_len);
                if let (UnaryOp::Neg, Node::Const(value)) = (op, *arena.node(simplified)) {
                    return arena.push(Node::Const(-value));
                }
                if let UnaryOp::Pos = op {
                    return simplified;
                }
                if simplified == operand {
                    return expr;
                }
                arena.push(Node::Unary(op, simplified))
            }
            Node::Conditional(condition, then_expr, else_expr) => {
                let simplified_condition = simplify_from(arena, condition, primal_len);
                let simplified_then = simplify_from(arena, then_expr, primal_len);
                let simplified_else = simplify_from(arena, else_expr, primal_len);
                if let Node::Const(value) = *arena.node(simplified_condition) {
                    return if value != 0.0 {
                        simplified_then
                    } else {
                        simplified_else
                    };
                }
                if simplified_condition == condition
                    && simplified_then == then_expr
                    && simplified_else == else_expr
                {
                    return expr;
                }
                arena.push(Node::Conditional(
                    simplified_condition,
                    simplified_then,
                    simplified_else,
                ))
            }
            Node::Call { func, argc, a, b } => {
                let simplified_a = a.map(|argument| simplify_from(arena, argument, primal_len));
                let simplified_b = b.map(|argument| simplify_from(arena, argument, primal_len));
                if simplified_a == a && simplified_b == b {
                    return expr;
                }
                arena.push(Node::Call {
                    func,
                    argc,
                    a: simplified_a,
                    b: simplified_b,
                })
            }
            Node::CallSpilled { func, args } => {
                let arguments = arena.call_args(args).to_vec();
                let mut simplified = Vec::with_capacity(arguments.len());
                for argument in &arguments {
                    simplified.push(simplify_from(arena, *argument, primal_len));
                }
                if let IrFunction::SumProductsDiv = func {
                    let terms = (simplified.len() - 1) / 2;
                    let divisor = simplified[2 * terms];
                    let mut live = Vec::with_capacity(simplified.len());
                    for pair in simplified[..2 * terms].chunks_exact(2) {
                        if pair
                            .iter()
                            .any(|&id| matches!(*arena.node(id), Node::Const(x) if x == 0.0))
                        {
                            continue;
                        }
                        live.extend(pair);
                    }
                    if live.is_empty() {
                        return arena.push(Node::Const(0.0));
                    }
                    if live.len() == 2 {
                        let factor = if matches!(*arena.node(live[0]), Node::Const(1.0)) {
                            Some(live[1])
                        } else if matches!(*arena.node(live[1]), Node::Const(1.0)) {
                            Some(live[0])
                        } else {
                            None
                        };
                        if let Some(factor) = factor {
                            return arena.push(Node::Binary(BinaryOp::Div, factor, divisor));
                        }
                    }
                    live.push(divisor);
                    if live != arguments {
                        return arena.push_call(IrFunction::SumProductsDiv, &live);
                    }
                }
                if simplified == arguments {
                    return expr;
                }
                arena.push_call(func, &simplified)
            }
            // Companion factors of a zero derivative vanish
            Node::IdtCompanion(operand) => {
                let simplified = simplify_from(arena, operand, primal_len);
                if matches!(*arena.node(simplified), Node::Const(value) if value == 0.0) {
                    return arena.push(Node::Const(0.0));
                }
                if simplified == operand {
                    return expr;
                }
                arena.push(Node::IdtCompanion(simplified))
            }
            _ => expr,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A `white_noise` of one process, its magnitude the constant one.
        fn noise(arena: &mut ExprArena, process: u32) -> NodeId {
            let power = arena.push(Node::Const(1.0));
            arena.push_heavy(Heavy::WhiteNoise {
                site: NoiseSiteId {
                    source: 0,
                    start: process,
                    end: process + 1,
                    ordinal: process,
                },
                power,
                name: None,
            })
        }

        fn add(arena: &mut ExprArena, left: NodeId, right: NodeId) -> NodeId {
            arena.push(Node::Binary(BinaryOp::Add, left, right))
        }

        fn mul(arena: &mut ExprArena, left: NodeId, right: NodeId) -> NodeId {
            arena.push(Node::Binary(BinaryOp::Mul, left, right))
        }

        fn var(arena: &mut ExprArena, name: &str) -> NodeId {
            let name = arena.intern(name);
            arena.push(Node::Var(name))
        }

        fn constant(arena: &mut ExprArena, value: f64) -> NodeId {
            arena.push(Node::Const(value))
        }

        /// The axes symbolic differentiation finds, which the structural walk
        /// has to agree with.
        fn noise_dependencies(
            deps: &HashMap<SmolStr, BTreeSet<usize>>,
        ) -> HashMap<SmolStr, BTreeSet<DerivativeWrt>> {
            deps.iter()
                .map(|(name, axes)| {
                    (
                        name.clone(),
                        axes.iter().copied().map(DerivativeWrt::Noise).collect(),
                    )
                })
                .collect()
        }

        fn expression_noise_axes(
            arena: &mut ExprArena,
            expr: NodeId,
            deps: &HashMap<SmolStr, BTreeSet<usize>>,
            count: usize,
        ) -> BTreeSet<usize> {
            expression_auxiliary_axes(
                arena,
                expr,
                &noise_dependencies(deps),
                AuxiliaryAxes::Noise(count),
            )
            .into_iter()
            .map(|axis| {
                let DerivativeWrt::Noise(process) = axis else {
                    panic!("unexpected non-noise direction")
                };
                process
            })
            .collect()
        }

        fn old_ad_axes(
            arena: &mut ExprArena,
            expr: NodeId,
            deps: &HashMap<SmolStr, BTreeSet<usize>>,
            num_processes: usize,
        ) -> BTreeSet<usize> {
            let shadows = ShadowContext {
                auxiliary_shadowed: noise_dependencies(deps),
                ..ShadowContext::default()
            };
            (0..num_processes)
                .filter(|process| {
                    let raw = differentiate_with_shadows(
                        arena,
                        expr,
                        &DerivativeWrt::Noise(*process),
                        &shadows,
                    );
                    let derivative = simplify(arena, raw);
                    !matches!(*arena.node(derivative), Node::Const(value) if value == 0.0)
                })
                .collect()
        }

        #[test]
        fn structural_noise_axes_match_symbolic_ad_for_value_expressions() {
            let deps = HashMap::from([
                (SmolStr::new("assigned"), BTreeSet::from([2usize])),
                (SmolStr::new("samples"), BTreeSet::from([3usize])),
            ]);

            // Each fixture builds into its own arena so the two walks below
            // read the same nodes and nothing a previous case appended is
            // reachable from this one.
            type Build = fn(&mut ExprArena) -> NodeId;
            let corpus: [(&str, Build); 14] = [
                ("noise metadata is not a realization operand", |arena| {
                    let power = noise(arena, 1);
                    arena.push_heavy(Heavy::WhiteNoise {
                        site: NoiseSiteId {
                            source: 0,
                            start: 0,
                            end: 1,
                            ordinal: 0,
                        },
                        power,
                        name: None,
                    })
                }),
                ("assigned variable and direct process", |arena| {
                    let left = var(arena, "assigned");
                    let right = noise(arena, 0);
                    add(arena, left, right)
                }),
                ("zero multiplier removes derivative", |arena| {
                    let left = constant(arena, 0.0);
                    let right = noise(arena, 0);
                    mul(arena, left, right)
                }),
                ("zero power removes derivative", |arena| {
                    let base = noise(arena, 0);
                    let exponent = constant(arena, 0.0);
                    arena.push(Node::Binary(BinaryOp::Pow, base, exponent))
                }),
                ("discrete operator has zero derivative", |arena| {
                    let left = noise(arena, 0);
                    let right = noise(arena, 1);
                    arena.push(Node::Binary(BinaryOp::Gt, left, right))
                }),
                (
                    "constant conditional selects one derivative branch",
                    |arena| {
                        let condition = constant(arena, 0.0);
                        let then_expr = noise(arena, 0);
                        let else_expr = noise(arena, 1);
                        arena.push(Node::Conditional(condition, then_expr, else_expr))
                    },
                ),
                ("continuous function follows its argument", |arena| {
                    let argument = noise(arena, 0);
                    arena.push_call(IrFunction::Exp, &[argument])
                }),
                (
                    "pow function follows binary-pow zero-exponent semantics",
                    |arena| {
                        let base = noise(arena, 0);
                        let exponent = constant(arena, 0.0);
                        arena.push_call(IrFunction::Pow, &[base, exponent])
                    },
                ),
                ("atan2 retains its normalized derivative domain", |arena| {
                    let left = noise(arena, 0);
                    let right = constant(arena, 0.0);
                    arena.push_call(IrFunction::Atan2, &[left, right])
                }),
                ("hypot retains both noise operands", |arena| {
                    let left = noise(arena, 0);
                    let right = noise(arena, 1);
                    arena.push_call(IrFunction::Hypot, &[left, right])
                }),
                ("piecewise-constant function has zero derivative", |arena| {
                    let argument = noise(arena, 0);
                    arena.push_call(IrFunction::Floor, &[argument])
                }),
                ("ddx is resolved before noise provenance", |arena| {
                    let left = noise(arena, 1);
                    let right = arena.push(Node::Voltage(0, u32::MAX));
                    let expr = mul(arena, left, right);
                    let axis = arena.push_ddx_axis(DdxAxis::Potential {
                        pos: Some(0),
                        neg: None,
                    });
                    arena.push(Node::Ddx { expr, axis })
                }),
                ("ddx can eliminate a noise-only value", |arena| {
                    let expr = noise(arena, 1);
                    let axis = arena.push_ddx_axis(DdxAxis::Potential {
                        pos: Some(0),
                        neg: None,
                    });
                    arena.push(Node::Ddx { expr, axis })
                }),
                (
                    "a zero denominator must not prune numerator noise before runtime validation",
                    |arena| {
                        let numerator = noise(arena, 0);
                        let denominator = constant(arena, 0.0);
                        arena.push(Node::Binary(BinaryOp::Div, numerator, denominator))
                    },
                ),
            ];

            for (description, build) in corpus {
                // The singular division is asserted over one process, as it
                // was before; every other case over four.
                let processes = if description.starts_with("a zero denominator") {
                    1
                } else {
                    4
                };
                let mut structural = ExprArena::new();
                let id = build(&mut structural);
                let structural = expression_noise_axes(&mut structural, id, &deps, processes);

                let mut symbolic = ExprArena::new();
                let id = build(&mut symbolic);
                let symbolic = old_ad_axes(&mut symbolic, id, &deps, processes);

                assert_eq!(structural, symbolic, "{description}");
            }
        }

        #[test]
        fn structural_noise_axes_follow_only_stateful_derivative_operands() {
            let deps = HashMap::from([(SmolStr::new("samples"), BTreeSet::from([3usize]))]);

            let arena = &mut ExprArena::new();
            let index = noise(arena, 1);
            let array = arena.intern("samples");
            let payload = arena.push_indexed(arena::IndexedRead {
                array,
                base: 0,
                len: 4,
                lower: 0,
            });
            let indexed = arena.push(Node::VarIndexed { payload, index });
            assert_eq!(
                expression_noise_axes(arena, indexed, &deps, 4),
                BTreeSet::from([3]),
                "runtime array reads follow the selected value, not the index"
            );

            let arena = &mut ExprArena::new();
            let condition = noise(arena, 0);
            let then_expr = noise(arena, 1);
            let else_expr = var(arena, "samples");
            let conditional = arena.push(Node::Conditional(condition, then_expr, else_expr));
            assert_eq!(
                expression_noise_axes(arena, conditional, &deps, 4),
                BTreeSet::from([1, 3]),
                "conditional predicates select a derivative branch but are not differentiated"
            );

            let arena = &mut ExprArena::new();
            let expr = noise(arena, 0);
            let delay_time = noise(arena, 1);
            let max_delay = noise(arena, 2);
            let absdelay = arena.push_heavy(Heavy::AbsDelay {
                site: AbsDelaySiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr,
                delay_time,
                max_delay: Some(max_delay),
            });
            assert_eq!(
                expression_noise_axes(arena, absdelay, &deps, 4),
                BTreeSet::from([0, 1]),
                "absdelay differentiates its value and delay, not max-delay metadata"
            );

            let arena = &mut ExprArena::new();
            let expr = noise(arena, 0);
            let delay = noise(arena, 1);
            let rise_time = noise(arena, 2);
            let fall_time = noise(arena, 3);
            let transition = arena.push_heavy(Heavy::Transition {
                site: TransitionSiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr,
                delay: Some(delay),
                rise_time: Some(rise_time),
                fall_time: Some(fall_time),
            });
            assert_eq!(
                expression_noise_axes(arena, transition, &deps, 4),
                BTreeSet::from([0]),
                "transition timing operands are primal-only"
            );

            let arena = &mut ExprArena::new();
            let expr = noise(arena, 0);
            let max_pos_slew = noise(arena, 1);
            let max_neg_slew = noise(arena, 2);
            let slew = arena.push_heavy(Heavy::Slew {
                site: SlewSiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr,
                max_pos_slew: Some(max_pos_slew),
                max_neg_slew: Some(max_neg_slew),
            });
            assert_eq!(
                expression_noise_axes(arena, slew, &deps, 4),
                BTreeSet::from([0, 1, 2]),
                "slew has derivative action through its value and rate operands"
            );
        }
    }
}
