//! Intermediate Representation for Verilog-A
//!
//! The IR represents device equations in a form suitable for:
//! 1. Automatic differentiation (Jacobian generation)
//! 2. Code generation for MNA matrix stamping

use crate::ast::{BinaryOp, UnaryOp};
use crate::error::CompileResult;
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

#[derive(Debug, Clone)]
pub enum ZiPolynomialDefinition {
    Coefficients(Vec<IrExpr>),
    Roots(Vec<(IrExpr, IrExpr)>),
}

/// Compiled device model in IR form
#[derive(Debug, Clone)]
pub struct DeviceIR {
    /// Module name
    pub name: SmolStr,
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
    /// Variable assignments and runtime loops (in execution order)
    pub assignments: Vec<IrAssignmentItem>,
    /// Frequency-domain assignment replay including noise-process shadows.
    /// Kept separate so DC/transient evaluation pays no process-AD overhead.
    pub noise_assignments: Vec<IrAssignmentItem>,
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
    pub default_expr: Option<IrExpr>,
    pub is_integer: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_parameter: Option<SmolStr>,
    pub max_parameter: Option<SmolStr>,
    pub min_expr: Option<IrExpr>,
    pub max_expr: Option<IrExpr>,
    pub min_exclusive: bool,
    pub max_exclusive: bool,
    pub exclude: Vec<f64>,
    pub exclude_parameters: Vec<SmolStr>,
    pub exclude_exprs: Vec<IrExpr>,
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
    /// The expression to assign
    pub expr: IrExpr,
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
    pub index: IrExpr,
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
        condition: IrExpr,
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
    pub static_condition: Option<IrExpr>,
    /// The expression tree
    pub expr: IrExpr,
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
    /// Positive node (unified index)
    pub pos: usize,
    /// Negative node (unified index)
    pub neg: usize,
    /// Driven by an indirect contribution: the branch row holds the
    /// constraint equation, so the structural V(p)-V(n) row entries must
    /// not be stamped
    pub indirect: bool,
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
    /// The derivative expression
    pub expr: IrExpr,
}

/// What a derivative is with respect to
#[derive(Debug, Clone)]
pub enum DerivativeWrt {
    /// Voltage at a unified node index
    Voltage(usize),
    /// Branch-current unknown (by ordinal)
    BranchCurrent(usize),
    /// Unit-amplitude realization of one syntactic noise process.
    Noise(usize),
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

/// IR Expression tree
#[derive(Debug, Clone)]
pub enum IrExpr {
    /// Constant value
    Const(f64),
    /// Parameter reference
    Param(SmolStr),
    /// Whether a parameter was explicitly set on the instance
    /// ($param_given)
    ParamGiven(SmolStr),
    /// Variable reference
    Var(SmolStr),
    /// Runtime-indexed array element read: element `index - lower` of the
    /// contiguous variable run starting at `base`
    VarIndexed {
        /// Array name (for shadow naming)
        array: SmolStr,
        /// First element's variable index
        base: usize,
        /// Number of elements
        len: usize,
        /// Declared lower bound
        lower: i64,
        /// Element index expression
        index: Box<IrExpr>,
    },
    /// Voltage at terminal pair
    Voltage(usize, usize),
    /// Current through branch
    Current(usize, usize),
    /// Branch-current unknown of a potential contribution (by ordinal)
    BranchCurrent(usize),
    /// Time variable
    Time,
    /// Temperature ($temperature)
    Temperature,
    /// Thermal voltage ($vt)
    Vt,
    /// Instance multiplicity ($mfactor): the number of parallel copies
    /// this instance represents. The simulator scales flow contributions
    /// automatically; reading it supports models that need fine control.
    Mfactor,
    /// Whether an external terminal was connected on this instance.
    PortConnected(usize),
    /// Binary operation
    Binary(BinaryOp, Box<IrExpr>, Box<IrExpr>),
    /// Unary operation
    Unary(UnaryOp, Box<IrExpr>),
    /// Function call
    Call(IrFunction, Vec<IrExpr>),
    /// Time derivative (ddt)
    Ddt(Box<IrExpr>),
    /// Time integral (idt)
    Idt(Box<IrExpr>, Option<Box<IrExpr>>),
    /// Wrapped time integral (idtmod): the integral folds into
    /// [offset, offset + modulus)
    IdtMod {
        expr: Box<IrExpr>,
        ic: Option<Box<IrExpr>>,
        modulus: Box<IrExpr>,
        offset: Option<Box<IrExpr>>,
    },
    /// Limited exponential
    Limexp(Box<IrExpr>),
    /// $limit function for convergence control
    /// Bounds the expression change per Newton iteration
    /// Args: (expression, step_limit)
    Limit(Box<IrExpr>, Option<Box<IrExpr>>),
    /// Non-executable legacy-IR carrier used only to allocate and correlate a
    /// named limiter's state slot for canonical native compilation.
    CanonicalLimit(Box<IrExpr>),
    /// $table_model lookup table interpolation
    /// Args: (input_expr, table_data) where table_data is (x_values, y_values)
    TableLookup {
        input: Box<IrExpr>,
        x_data: Vec<f64>,
        y_data: Vec<f64>,
    },
    /// absdelay - absolute transport delay
    /// Returns the value of expr delayed by delay_time seconds
    /// Uses a circular buffer for transient analysis
    AbsDelay {
        site: AbsDelaySiteId,
        expr: Box<IrExpr>,
        delay_time: Box<IrExpr>,
        max_delay: Option<Box<IrExpr>>,
    },
    /// Exact local first-derivative action of one `absdelay` candidate.
    /// Higher derivative orders are retained only so bytecode lowering can
    /// reject them explicitly instead of emitting a silently wrong Hessian.
    AbsDelayDerivative {
        site: AbsDelaySiteId,
        input: Box<IrExpr>,
        input_derivative: Box<IrExpr>,
        delay_time: Box<IrExpr>,
        delay_derivative: Box<IrExpr>,
        max_delay: Option<Box<IrExpr>>,
        derivative_order: u8,
    },
    /// transition - piecewise-linear signal smoothing
    /// Args: (expr, delay, rise_time, fall_time)
    /// Smoothly transitions between values over rise/fall times
    Transition {
        site: TransitionSiteId,
        expr: Box<IrExpr>,
        delay: Option<Box<IrExpr>>,
        rise_time: Option<Box<IrExpr>>,
        fall_time: Option<Box<IrExpr>>,
    },
    /// Exact read-only local derivative action of one `transition` candidate.
    /// Timing operands are primal-only because the LRM requires the Jacobian
    /// action here to be the candidate's input coefficient times `d(input)`.
    TransitionDerivative {
        site: TransitionSiteId,
        input: Box<IrExpr>,
        input_derivative: Box<IrExpr>,
        delay: Option<Box<IrExpr>>,
        rise_time: Option<Box<IrExpr>>,
        fall_time: Option<Box<IrExpr>>,
    },
    /// slew - slew rate limiting
    /// Args: (expr, max_pos_slew, max_neg_slew)
    /// Limits the rate of change of the signal
    Slew {
        site: SlewSiteId,
        expr: Box<IrExpr>,
        max_pos_slew: Option<Box<IrExpr>>,
        max_neg_slew: Option<Box<IrExpr>>,
    },
    /// Exact local derivative action of one `slew` candidate.
    SlewDerivative {
        site: SlewSiteId,
        input: Box<IrExpr>,
        input_derivative: Box<IrExpr>,
        max_pos_slew: Option<Box<IrExpr>>,
        max_pos_slew_derivative: Option<Box<IrExpr>>,
        max_neg_slew: Option<Box<IrExpr>>,
        max_neg_slew_derivative: Option<Box<IrExpr>>,
    },
    /// cross - threshold crossing detection
    /// Args: (expr, direction, time_tol, expr_tol)
    /// Returns 1 when expr crosses zero, else 0
    Cross {
        expr: Box<IrExpr>,
        direction: Option<Box<IrExpr>>, // runtime integer: +1=rising, -1=falling, 0=both
        time_tol: Option<Box<IrExpr>>,
        expr_tol: Option<Box<IrExpr>>,
        enable: Option<Box<IrExpr>>,
    },
    /// Time of the most recent zero crossing, or -1 before any crossing.
    LastCrossing {
        expr: Box<IrExpr>,
        direction: Option<i32>,
    },
    /// white_noise - white noise source for AC noise analysis
    /// Args: (power, name)
    WhiteNoise {
        site: NoiseSiteId,
        power: Box<IrExpr>,
        name: Option<String>,
    },
    /// flicker_noise - 1/f flicker noise source
    /// Args: (power, exponent, name)
    FlickerNoise {
        site: NoiseSiteId,
        power: Box<IrExpr>,
        exponent: Box<IrExpr>,
        name: Option<String>,
    },
    /// noise_table / noise_table_log - interpolated PSD over frequency.
    /// Points are (frequency, power) pairs sorted by frequency;
    /// `log_interp` selects log-log interpolation.
    NoiseTable {
        site: NoiseSiteId,
        points: Vec<(f64, f64)>,
        log_interp: bool,
        name: Option<String>,
    },
    /// analysis(name) - check current analysis type
    /// Returns 1.0 if running specified analysis, else 0.0
    Analysis(String),
    /// above(expr, time_tol, expr_tol, enable) - rising zero-crossing event
    /// Returns 1 initially when positive and on each subsequent rising event.
    Above {
        expr: Box<IrExpr>,
        time_tol: Option<Box<IrExpr>>,
        expr_tol: Option<Box<IrExpr>>,
        enable: Option<Box<IrExpr>>,
    },
    /// timer(start, period, time_tol, enable) - time event
    /// Returns 1 at time=start and every positive period thereafter.
    Timer {
        start_time: Box<IrExpr>,
        period: Option<Box<IrExpr>>,
        time_tol: Option<Box<IrExpr>>,
        enable: Option<Box<IrExpr>>,
    },
    /// laplace_zp - s-domain filter with poles and zeros
    /// Args: (expr, zeros, poles, k_factor)
    LaplaceZP {
        site: LaplaceSiteId,
        expr: Box<IrExpr>,
        zeros: Vec<(f64, f64)>, // (real, imag) pairs
        poles: Vec<(f64, f64)>,
        gain: f64,
    },
    /// laplace_nd - s-domain filter with num/den coefficients
    /// Args: (expr, numerator_coeffs, denominator_coeffs)
    LaplaceND {
        site: LaplaceSiteId,
        expr: Box<IrExpr>,
        numerator: Vec<f64>, // ascending powers of s
        denominator: Vec<f64>,
    },
    /// Exact Jacobian action of a coefficient-form Laplace filter. It shares
    /// the primal site's state and applies the DC gain outside active
    /// transient integration or the active companion-rule input gain during
    /// it.
    LaplaceNDDerivative {
        site: LaplaceSiteId,
        expr: Box<IrExpr>,
        numerator: Vec<f64>,
        denominator: Vec<f64>,
    },
    /// Exact Jacobian action of a pole-zero Laplace filter. It has the same
    /// analysis-dependent behavior and state identity as
    /// [`Self::LaplaceNDDerivative`].
    LaplaceZPDerivative {
        site: LaplaceSiteId,
        expr: Box<IrExpr>,
        zeros: Vec<(f64, f64)>,
        poles: Vec<(f64, f64)>,
        gain: f64,
    },
    /// zi_* - z-domain (sampled-data) filter: the input samples every
    /// `period` seconds and the difference equation output holds between
    /// samples. Coefficients ascend in z⁻¹.
    ZiFilter {
        site: ZiSiteId,
        expr: Box<IrExpr>,
        numerator: ZiPolynomialDefinition,
        denominator: ZiPolynomialDefinition,
        period: Box<IrExpr>,
        transition: Box<IrExpr>,
        first_transition: Box<IrExpr>,
        direct_assignment: bool,
    },
    /// Exact Jacobian action of a zi filter. It uses the same schedule as the
    /// value filter but applies H(1), b0/a0, or zero according to analysis and
    /// whether the current point is a sample edge.
    ZiFilterDerivative {
        site: ZiSiteId,
        expr: Box<IrExpr>,
        numerator: ZiPolynomialDefinition,
        denominator: ZiPolynomialDefinition,
        period: Box<IrExpr>,
        transition: Box<IrExpr>,
        first_transition: Box<IrExpr>,
        direct_assignment: bool,
    },
    /// Symbolic partial derivative with respect to a branch potential or a
    /// solver-owned branch flow. Resolved to an explicit derivative expression
    /// during device IR construction (where assignment chains are known).
    Ddx { expr: Box<IrExpr>, axis: DdxAxis },
    /// Companion-model Jacobian factor for ddt: operand / dt in transient,
    /// zero at DC (backward Euler)
    DdtCompanion(Box<IrExpr>),
    /// Companion-model Jacobian factor for idt: operand * dt in transient,
    /// zero at DC
    IdtCompanion(Box<IrExpr>),
    /// Slope of a lookup table evaluated at the input point
    TableDerivative {
        input: Box<IrExpr>,
        x_data: Vec<f64>,
        y_data: Vec<f64>,
    },
    /// Conditional
    Conditional(Box<IrExpr>, Box<IrExpr>, Box<IrExpr>),
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
    pub gain: IrExpr,
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
    pub psd: IrExpr,
    /// Flicker frequency exponent (None = white): S(f) = psd / f^exp
    pub exponent: Option<IrExpr>,
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
            terminals: Vec::new(),
            internal_nodes: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
            event_state_variables: module.event_state_variables.clone(),
            assignments: Vec::new(),
            noise_assignments: Vec::new(),
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

        // Array layouts (element slots are already in `variables`)
        for (name, layout) in &module.arrays {
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
                let converted = converter.convert(default_expr)?;
                if !Self::is_static_param_expr(&converted) {
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
                let convert_range_expr =
                    |expression: &crate::ast::Expression, label: &str| -> CompileResult<IrExpr> {
                        let converted = converter.convert(expression)?;
                        if !Self::is_range_parameter_expr(&converted) {
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
                ir.parameters[idx].min_expr = range
                    .min_expression
                    .as_ref()
                    .map(|expression| convert_range_expr(expression, "lower range bound"))
                    .transpose()?;
                ir.parameters[idx].max_expr = range
                    .max_expression
                    .as_ref()
                    .map(|expression| convert_range_expr(expression, "upper range bound"))
                    .transpose()?;
                ir.parameters[idx].exclude_exprs = range
                    .exclude_expressions
                    .iter()
                    .map(|expression| convert_range_expr(expression, "excluded range value"))
                    .collect::<CompileResult<Vec<_>>>()?;
            }
        }

        // Convert evaluation statements (assignments and runtime loops) to
        // IR, in order
        let span = crate::metrics::FineSpan::new("ir.statements");
        let mut items = Vec::with_capacity(module.statements.len());
        Self::convert_statements(&module.statements, &converter, &mut items)?;
        let mut zi_site_ordinal = 0_u32;
        let mut laplace_site_ordinal = 0_u32;
        let mut slew_site_ordinal = 0_u32;
        let mut transition_site_ordinal = 0_u32;
        let mut absdelay_site_ordinal = 0_u32;
        autodiff::assign_zi_site_ordinals_in_items(&mut items, &mut zi_site_ordinal);
        autodiff::assign_laplace_site_ordinals_in_items(&mut items, &mut laplace_site_ordinal);
        autodiff::assign_slew_site_ordinals_in_items(&mut items, &mut slew_site_ordinal);
        autodiff::assign_transition_site_ordinals_in_items(
            &mut items,
            &mut transition_site_ordinal,
        );
        autodiff::assign_absdelay_site_ordinals_in_items(&mut items, &mut absdelay_site_ordinal);
        ir.assignments = items;
        span.finish(&format!(
            "module={} statements={}",
            module.name,
            module.statements.len()
        ));

        // Pre-pass over contributions: parse branch refs and register a
        // branch-current unknown per node pair receiving a potential
        // contribution. Pairs are normalized so V(a,b) and V(b,a) share
        // one unknown (the reversed orientation flips the sign).
        let mut parsed_contribs: Vec<BranchRef> = Vec::with_capacity(module.contributions.len());
        // (min,max) node pair -> (ordinal, oriented positive node)
        let mut branch_table: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for contrib in &module.contributions {
            let branch_ref = Self::parse_branch_name(&contrib.branch, &ctx).ok_or_else(|| {
                crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::InvalidExpression(
                    format!("Unknown contribution branch '{}'", contrib.branch),
                ))
            })?;

            // Potential contributions and indirect contributions (either
            // target kind) introduce a branch-current unknown
            if !contrib.is_current || contrib.indirect {
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let ordinal = match branch_table.get(&key) {
                    Some(&(ordinal, _)) => ordinal,
                    None => {
                        let ordinal = ir.branch_unknowns.len();
                        ir.branch_unknowns.push(BranchUnknownDef {
                            pos: branch_ref.pos_terminal,
                            neg: branch_ref.neg_terminal,
                            indirect: contrib.indirect,
                        });
                        branch_table.insert(key, (ordinal, branch_ref.pos_terminal));
                        ordinal
                    }
                };
                // A branch is either constrained by one indirect equation
                // or driven by (summed) direct potential contributions;
                // mixing them would over-determine the unknown
                let registered_indirect = ir.branch_unknowns[ordinal].indirect;
                if registered_indirect != contrib.indirect
                    || (contrib.indirect && registered_indirect && {
                        // Second indirect contribution on the same pair
                        parsed_contribs.iter().zip(module.contributions.iter()).any(
                            |(prev_ref, prev)| {
                                prev.indirect
                                    && (
                                        prev_ref.pos_terminal.min(prev_ref.neg_terminal),
                                        prev_ref.pos_terminal.max(prev_ref.neg_terminal),
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
        let mut converted_contribs = Vec::with_capacity(module.contributions.len());
        for contrib in &module.contributions {
            let mut expr = converter.convert_contribution(&contrib.expression)?;
            autodiff::assign_zi_site_ordinals(&mut expr, &mut zi_site_ordinal);
            autodiff::assign_laplace_site_ordinals(&mut expr, &mut laplace_site_ordinal);
            autodiff::assign_slew_site_ordinals(&mut expr, &mut slew_site_ordinal);
            autodiff::assign_transition_site_ordinals(&mut expr, &mut transition_site_ordinal);
            autodiff::assign_absdelay_site_ordinals(&mut expr, &mut absdelay_site_ordinal);
            converted_contribs.push(autodiff::rewrite_branch_probes(&expr, &branch_table));
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
                crate::semantic::AnalyzedStatement::Loop(loop_statement) => loop_statement.site,
            })
            .collect::<Vec<_>>();
        let equation_sites = module
            .contributions
            .iter()
            .map(|contribution| contribution.site)
            .collect::<Vec<_>>();
        ir.reaching_snapshots = crate::reaching_definition::insert_equation_snapshots(
            &mut ir.assignments,
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
        Self::collect_noise_processes_in_items(&ir.assignments, &mut ir.noise_sources)?;
        for expr in &converted_contribs {
            Self::collect_noise_processes(expr, &mut ir.noise_sources)?;
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

        // Current probes I(a,b) of a branch that carries a potential
        // contribution read the branch unknown (exact), not the inferred
        // contribution cache.
        if !branch_table.is_empty() {
            autodiff::rewrite_branch_probes_in_items(&mut ir.assignments, &branch_table);
        }

        // Variables that are fixed per instance (computed purely from
        // parameters) may participate in topology guards
        let span = crate::metrics::FineSpan::new("ir.static_vars");
        let static_vars = Self::compute_instance_static_vars(&ir.assignments, &ir.variables);
        span.finish(&format!(
            "module={} static={}",
            module.name,
            static_vars.len()
        ));

        // Shadow liveness roots: only variables that contribution
        // expressions (the equation Jacobians chain through them) or
        // ddx() operands read need derivative shadows. Everything else —
        // operating-point reporting variables above all — keeps its
        // primal value but never costs shadow slots or updates.
        let mut shadow_roots: HashSet<SmolStr> = HashSet::new();
        let mut second_shadow_roots: HashSet<SmolStr> = HashSet::new();
        for expr in &converted_contribs {
            autodiff::collect_var_names(&expr, &mut shadow_roots);
            autodiff::collect_ddx_operand_names_in_expr(&expr, &mut second_shadow_roots);
        }
        autodiff::collect_ddx_operand_names(&ir.assignments, &mut second_shadow_roots);
        shadow_roots.extend(second_shadow_roots.iter().cloned());

        // Forward-mode AD over the assignment sequence: build shadow
        // assignments holding each variable's partial derivative w.r.t.
        // every node voltage and branch-current unknown, so equation
        // Jacobians chain through intermediate variables. Shadow updates
        // recurse into loop bodies so loop-carried dependencies
        // differentiate correctly.
        let span = crate::metrics::FineSpan::new("ir.shadow_assignments");
        let mut shadows = autodiff::build_shadow_assignments(
            &mut ir,
            num_nodes,
            num_branches,
            &shadow_roots,
            &second_shadow_roots,
        );
        span.finish(&format!(
            "module={} shadow_variables={}",
            module.name,
            ir.variables.len()
        ));
        let span = crate::metrics::FineSpan::new("ir.noise_shadow_assignments");
        if !ir.noise_sources.is_empty() {
            let noise_process_count = ir.noise_sources.len();
            let ordinary_assignments = ir.assignments.clone();
            autodiff::build_noise_shadow_assignments(
                &mut ir,
                noise_process_count,
                &shadow_roots,
                &mut shadows,
            );
            ir.noise_assignments = std::mem::replace(&mut ir.assignments, ordinary_assignments);
        }
        span.finish(&format!(
            "module={} processes={}",
            module.name,
            ir.noise_sources.len()
        ));

        // Resolve ddx() operators now that the shadow context exists
        let span = crate::metrics::FineSpan::new("ir.resolve_ddx");
        autodiff::resolve_ddx_in_items(&mut ir.assignments, &shadows);
        span.finish(&format!(
            "module={} assignments={}",
            module.name,
            ir.assignments.len()
        ));
        let span = crate::metrics::FineSpan::new("ir.resolve_ddx_noise");
        autodiff::resolve_ddx_in_items(&mut ir.noise_assignments, &shadows);
        span.finish(&format!(
            "module={} assignments={}",
            module.name,
            ir.noise_assignments.len()
        ));

        // Convert contributions to equations
        let equation_span = crate::metrics::FineSpan::new("ir.equations");
        let mut derivative_elapsed = std::time::Duration::ZERO;
        let mut reactive_elapsed = std::time::Duration::ZERO;
        let mut noise_gain_elapsed = std::time::Duration::ZERO;
        for ((contrib, branch_ref), expr) in module
            .contributions
            .iter()
            .zip(parsed_contribs)
            .zip(converted_contribs)
        {
            let expr = autodiff::resolve_ddx(&expr, &shadows);

            // Peel instance-static guards (parameter expressions or
            // variables derived purely from parameters): a potential
            // contribution that is mode-disabled must leave the branch
            // open, not short it to zero volts.
            let (static_condition, expr) = Self::peel_static_condition(expr, &static_vars);

            let (branch_ref, expr, branch_ordinal) = if contrib.indirect {
                // Constraint equations are orientation-free (f == g holds
                // whichever way the target was written); the KCL couplings
                // use the unknown's registered orientation
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let (ordinal, _) = branch_table[&key];
                let unknown = &ir.branch_unknowns[ordinal];
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
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let (ordinal, oriented_pos) = branch_table[&key];
                if branch_ref.pos_terminal == oriented_pos {
                    (branch_ref, expr, Some(ordinal))
                } else {
                    // Reversed orientation: V(b,a) <+ E is V(a,b) <+ -E
                    let unknown = &ir.branch_unknowns[ordinal];
                    (
                        BranchRef {
                            pos_terminal: unknown.pos,
                            neg_terminal: unknown.neg,
                        },
                        IrExpr::Unary(UnaryOp::Neg, Box::new(expr)),
                        Some(ordinal),
                    )
                }
            };

            // Generate derivatives for Jacobian (over node voltages and
            // branch-current unknowns)
            let span = crate::metrics::FineSpan::new("ir.equation_derivatives");
            let derivatives = Self::generate_derivatives(&expr, num_nodes, num_branches, &shadows);
            derivative_elapsed += span.elapsed();

            // Reactive (charge/flux) derivatives for AC analysis: extract
            // the ddt() operand and differentiate it
            let span = crate::metrics::FineSpan::new("ir.equation_reactive");
            let reactive_derivatives = match Self::extract_charge(&expr) {
                Some(charge) => {
                    Self::generate_derivatives(&charge, num_nodes, num_branches, &shadows)
                }
                None => Vec::new(),
            };
            reactive_elapsed += span.elapsed();

            // Extract small-signal noise sources (white_noise /
            // flicker_noise terms) for noise analysis; they evaluate to
            // zero in the large-signal programs
            let equation_index = ir.equations.len();
            let span = crate::metrics::FineSpan::new("ir.equation_noise_gains");
            for process in &mut ir.noise_sources {
                let gain = autodiff::simplify(autodiff::differentiate_with_shadows(
                    &expr,
                    &DerivativeWrt::Noise(process.process_id),
                    &shadows,
                ));
                if Self::is_zero(&gain) {
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

            ir.equations.push(BranchEquation {
                branch: branch_ref,
                is_current: contrib.is_current,
                indirect: contrib.indirect,
                branch_ordinal,
                static_condition,
                expr,
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
        out: &mut Vec<NoiseSourceDef>,
    ) -> CompileResult<()> {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    Self::collect_noise_processes(&assignment.expr, out)?;
                }
                IrAssignmentItem::Loop { condition, body } => {
                    Self::collect_noise_processes(condition, out)?;
                    Self::collect_noise_processes_in_items(body, out)?;
                }
            }
        }
        Ok(())
    }

    fn collect_noise_processes(expr: &IrExpr, out: &mut Vec<NoiseSourceDef>) -> CompileResult<()> {
        let mut definitions = Vec::new();
        autodiff::collect_noise_definitions(expr, &mut definitions);
        for (site, psd, exponent, table, name) in definitions {
            let process_id = site.ordinal as usize;
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
    fn extract_charge(expr: &IrExpr) -> Option<IrExpr> {
        fn contains_ddt(e: &IrExpr) -> bool {
            match e {
                IrExpr::Ddt(_) => true,
                IrExpr::Binary(_, l, r) => contains_ddt(l) || contains_ddt(r),
                IrExpr::Unary(_, inner)
                | IrExpr::Limexp(inner)
                | IrExpr::DdtCompanion(inner)
                | IrExpr::IdtCompanion(inner) => contains_ddt(inner),
                IrExpr::Idt(inner, ic) => {
                    contains_ddt(inner) || ic.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::IdtMod {
                    expr,
                    ic,
                    modulus,
                    offset,
                } => {
                    contains_ddt(expr)
                        || ic.as_deref().is_some_and(contains_ddt)
                        || contains_ddt(modulus)
                        || offset.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Limit(inner, step) => {
                    contains_ddt(inner) || step.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::CanonicalLimit(inner) => contains_ddt(inner),
                IrExpr::Call(_, args) => args.iter().any(contains_ddt),
                IrExpr::Conditional(c, t, e) => {
                    contains_ddt(c) || contains_ddt(t) || contains_ddt(e)
                }
                IrExpr::TableLookup { input, .. } | IrExpr::TableDerivative { input, .. } => {
                    contains_ddt(input)
                }
                IrExpr::AbsDelay {
                    expr,
                    delay_time,
                    max_delay,
                    ..
                } => {
                    contains_ddt(expr)
                        || contains_ddt(delay_time)
                        || max_delay.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::AbsDelayDerivative {
                    input,
                    input_derivative,
                    delay_time,
                    delay_derivative,
                    max_delay,
                    ..
                } => {
                    contains_ddt(input)
                        || contains_ddt(input_derivative)
                        || contains_ddt(delay_time)
                        || contains_ddt(delay_derivative)
                        || max_delay.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Transition { expr, .. }
                | IrExpr::LaplaceZP { expr, .. }
                | IrExpr::LaplaceND { expr, .. }
                | IrExpr::LaplaceZPDerivative { expr, .. }
                | IrExpr::LaplaceNDDerivative { expr, .. }
                | IrExpr::ZiFilter { expr, .. }
                | IrExpr::ZiFilterDerivative { expr, .. }
                | IrExpr::Ddx { expr, .. } => contains_ddt(expr),
                IrExpr::TransitionDerivative {
                    input,
                    input_derivative,
                    delay,
                    rise_time,
                    fall_time,
                    ..
                } => {
                    contains_ddt(input)
                        || contains_ddt(input_derivative)
                        || delay.as_deref().is_some_and(contains_ddt)
                        || rise_time.as_deref().is_some_and(contains_ddt)
                        || fall_time.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Slew {
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                    ..
                } => {
                    contains_ddt(expr)
                        || max_pos_slew.as_deref().is_some_and(contains_ddt)
                        || max_neg_slew.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::SlewDerivative {
                    input,
                    input_derivative,
                    max_pos_slew,
                    max_pos_slew_derivative,
                    max_neg_slew,
                    max_neg_slew_derivative,
                    ..
                } => {
                    contains_ddt(input)
                        || contains_ddt(input_derivative)
                        || max_pos_slew.as_deref().is_some_and(contains_ddt)
                        || max_pos_slew_derivative.as_deref().is_some_and(contains_ddt)
                        || max_neg_slew.as_deref().is_some_and(contains_ddt)
                        || max_neg_slew_derivative.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Cross {
                    expr,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                } => {
                    contains_ddt(expr)
                        || direction.as_deref().is_some_and(contains_ddt)
                        || time_tol.as_deref().is_some_and(contains_ddt)
                        || expr_tol.as_deref().is_some_and(contains_ddt)
                        || enable.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::LastCrossing { expr, .. } => contains_ddt(expr),
                IrExpr::WhiteNoise { power, .. } => contains_ddt(power),
                IrExpr::FlickerNoise {
                    power, exponent, ..
                } => contains_ddt(power) || contains_ddt(exponent),
                IrExpr::NoiseTable { .. } => false,
                IrExpr::Above {
                    expr,
                    time_tol,
                    expr_tol,
                    enable,
                } => {
                    contains_ddt(expr)
                        || time_tol.as_deref().is_some_and(contains_ddt)
                        || expr_tol.as_deref().is_some_and(contains_ddt)
                        || enable.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Timer {
                    start_time,
                    period,
                    time_tol,
                    enable,
                } => {
                    contains_ddt(start_time)
                        || period.as_deref().is_some_and(contains_ddt)
                        || time_tol.as_deref().is_some_and(contains_ddt)
                        || enable.as_deref().is_some_and(contains_ddt)
                }
                // ddt() cannot appear in an element index (assignments
                // reject it upstream), so an indexed read is resistive
                IrExpr::VarIndexed { index, .. } => contains_ddt(index),
                IrExpr::Const(_)
                | IrExpr::Param(_)
                | IrExpr::ParamGiven(_)
                | IrExpr::Var(_)
                | IrExpr::Voltage(..)
                | IrExpr::Current(..)
                | IrExpr::BranchCurrent(_)
                | IrExpr::Time
                | IrExpr::Temperature
                | IrExpr::Vt
                | IrExpr::Mfactor
                | IrExpr::PortConnected(_)
                | IrExpr::Analysis(_) => false,
            }
        }

        match expr {
            IrExpr::Ddt(q) => Some((**q).clone()),
            IrExpr::Binary(op @ (BinaryOp::Add | BinaryOp::Sub), l, r) => {
                let ql = Self::extract_charge(l);
                let qr = Self::extract_charge(r);
                if ql.is_none() && qr.is_none() {
                    return None;
                }
                Some(IrExpr::Binary(
                    *op,
                    Box::new(ql.unwrap_or(IrExpr::Const(0.0))),
                    Box::new(qr.unwrap_or(IrExpr::Const(0.0))),
                ))
            }
            IrExpr::Binary(BinaryOp::Mul, l, r) => match (contains_ddt(l), contains_ddt(r)) {
                (false, false) => None,
                (false, true) => Self::extract_charge(r)
                    .map(|q| IrExpr::Binary(BinaryOp::Mul, l.clone(), Box::new(q))),
                (true, false) => Self::extract_charge(l)
                    .map(|q| IrExpr::Binary(BinaryOp::Mul, Box::new(q), r.clone())),
                (true, true) => {
                    log::warn!(
                        "ddt() on both sides of a product; reactive AC \
                             contribution omitted"
                    );
                    None
                }
            },
            IrExpr::Binary(BinaryOp::Div, l, r) if !contains_ddt(r) => Self::extract_charge(l)
                .map(|q| IrExpr::Binary(BinaryOp::Div, Box::new(q), r.clone())),
            IrExpr::Unary(op @ (UnaryOp::Neg | UnaryOp::Pos), e) => {
                Self::extract_charge(e).map(|q| IrExpr::Unary(*op, Box::new(q)))
            }
            IrExpr::Conditional(c, t, e) => {
                let qt = Self::extract_charge(t);
                let qe = Self::extract_charge(e);
                if qt.is_none() && qe.is_none() {
                    return None;
                }
                Some(IrExpr::Conditional(
                    c.clone(),
                    Box::new(qt.unwrap_or(IrExpr::Const(0.0))),
                    Box::new(qe.unwrap_or(IrExpr::Const(0.0))),
                ))
            }
            other => {
                if contains_ddt(other) {
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
        expr: IrExpr,
        static_vars: &HashSet<SmolStr>,
    ) -> (Option<IrExpr>, IrExpr) {
        let mut condition: Option<IrExpr> = None;
        let mut current = expr;
        loop {
            match current {
                IrExpr::Conditional(cond, then_expr, else_expr)
                    if Self::is_instance_static_expr(&cond, static_vars)
                        && matches!(*else_expr, IrExpr::Const(v) if v == 0.0) =>
                {
                    condition = Some(match condition {
                        Some(prev) => IrExpr::Binary(BinaryOp::And, Box::new(prev), cond),
                        None => *cond,
                    });
                    current = *then_expr;
                }
                other => return (condition, other),
            }
        }
    }

    /// Convert analyzed statements (assignments and runtime loops) to IR
    fn convert_statements(
        statements: &[crate::semantic::AnalyzedStatement],
        converter: &crate::expr_converter::ExprConverter,
        out: &mut Vec<IrAssignmentItem>,
    ) -> crate::error::CompileResult<()> {
        use crate::semantic::AnalyzedStatement;
        for stmt in statements {
            match stmt {
                AnalyzedStatement::Assignment(assign) => {
                    let expr = converter.convert(&assign.expression)?;
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
                                index: converter.convert(index_expr)?,
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
                    let condition = converter.convert(&loop_stmt.condition)?;
                    let mut body = Vec::with_capacity(loop_stmt.body.len());
                    Self::convert_statements(&loop_stmt.body, converter, &mut body)?;
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
        expr: &IrExpr,
        num_nodes: usize,
        num_branches: usize,
        shadows: &autodiff::ShadowContext,
    ) -> Vec<Derivative> {
        let mut derivatives = Vec::new();
        let active_axes = autodiff::expression_axes(expr, shadows, num_nodes);
        if active_axes == 0 {
            return derivatives;
        }

        for wrt in autodiff::axes(num_nodes, num_branches) {
            if !autodiff::mask_contains_axis(active_axes, &wrt, num_nodes) {
                continue;
            }
            let deriv_expr = autodiff::differentiate_with_shadows(expr, &wrt, shadows);
            let simplified = autodiff::simplify(deriv_expr);

            // Only add non-zero derivatives
            if !Self::is_zero(&simplified) {
                derivatives.push(Derivative {
                    wrt,
                    expr: simplified,
                });
            }
        }

        derivatives
    }

    /// Check if an expression is zero (constant 0.0)
    fn is_zero(expr: &IrExpr) -> bool {
        matches!(expr, IrExpr::Const(v) if v.abs() < 1e-30)
    }

    /// Check whether an expression depends only on parameters and constants
    /// (valid for instance-time parameter default evaluation)
    fn is_static_param_expr(expr: &IrExpr) -> bool {
        Self::is_instance_static_expr_with_options(expr, &HashSet::new(), false)
    }

    /// Range constraints are evaluated during instance setup and therefore
    /// may only read final parameter values and pure numeric expressions.
    fn is_range_parameter_expr(expr: &IrExpr) -> bool {
        let recurse = Self::is_range_parameter_expr;
        match expr {
            IrExpr::Const(_) | IrExpr::Param(_) => true,
            IrExpr::Binary(_, left, right) => recurse(left) && recurse(right),
            IrExpr::Unary(_, operand) | IrExpr::Limexp(operand) => recurse(operand),
            IrExpr::Call(_, arguments) => arguments.iter().all(recurse),
            IrExpr::Conditional(condition, then_expr, else_expr) => {
                recurse(condition) && recurse(then_expr) && recurse(else_expr)
            }
            _ => false,
        }
    }

    /// Check whether an expression is fixed per instance: it depends only
    /// on parameters, constants, temperature, analysis type, and variables
    /// proven instance-static. Such expressions may gate device topology.
    fn is_instance_static_expr(expr: &IrExpr, static_vars: &HashSet<SmolStr>) -> bool {
        Self::is_instance_static_expr_with_options(expr, static_vars, true)
    }

    fn is_instance_static_expr_with_options(
        expr: &IrExpr,
        static_vars: &HashSet<SmolStr>,
        allow_analysis: bool,
    ) -> bool {
        let recurse =
            |e: &IrExpr| Self::is_instance_static_expr_with_options(e, static_vars, allow_analysis);
        match expr {
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_) => true,
            IrExpr::Var(name) => static_vars.contains(name),
            // An indexed read is static when the index is static and every
            // element it could select is static
            IrExpr::VarIndexed {
                array,
                len,
                lower,
                index,
                ..
            } => {
                recurse(index)
                    && (*lower..*lower + *len as i64)
                        .all(|k| static_vars.contains(format!("{array}[{k}]").as_str()))
            }
            IrExpr::Binary(_, l, r) => recurse(l) && recurse(r),
            IrExpr::Unary(_, e) | IrExpr::Limexp(e) => recurse(e),
            IrExpr::Call(_, args) => args.iter().all(recurse),
            IrExpr::Conditional(c, t, e) => recurse(c) && recurse(t) && recurse(e),
            IrExpr::Analysis(_) => allow_analysis,
            _ => false,
        }
    }

    /// Fixpoint over the assignment tree: a variable is instance-static if
    /// every assignment to it uses only parameters, constants, and other
    /// instance-static variables. These variables hold the same value for
    /// every evaluation of a given instance (BSIM4's mode selectors like
    /// BSIM4rdsMod), so guards built from them may gate topology.
    fn compute_instance_static_vars(
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

        loop {
            let mut changed = false;
            fn prune(
                items: &[IrAssignmentItem],
                variables: &[VarDef],
                static_vars: &mut HashSet<SmolStr>,
                changed: &mut bool,
                enclosing_static: bool,
            ) {
                for item in items {
                    match item {
                        IrAssignmentItem::Assign(a) => {
                            if let Some(target) = &a.index {
                                // A runtime-indexed write may land in any
                                // element; a non-static one evicts them all
                                let write_static = enclosing_static
                                    && DeviceIR::is_instance_static_expr(&a.expr, static_vars)
                                    && DeviceIR::is_instance_static_expr(
                                        &target.index,
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
                                    || !DeviceIR::is_instance_static_expr(&a.expr, static_vars))
                            {
                                static_vars.remove(name);
                                *changed = true;
                            }
                        }
                        IrAssignmentItem::Loop { condition, body } => {
                            let loop_static = enclosing_static
                                && DeviceIR::is_instance_static_expr(condition, static_vars);
                            prune(body, variables, static_vars, changed, loop_static);
                        }
                    }
                }
            }
            prune(items, variables, &mut static_vars, &mut changed, true);
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
            DerivativeWrt::Noise(_) => return ALL_AXES,
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
        /// Independent syntactic noise processes carried by each variable.
        noise_shadowed: HashMap<SmolStr, BTreeSet<usize>>,
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
            }
        }

        pub fn is_shadowed(&self, name: &str) -> bool {
            self.shadowed.get(name).is_some_and(|mask| *mask != 0)
        }

        /// Whether `name` carries a shadow along the given axis
        pub fn is_shadowed_on(&self, name: &str, wrt: &DerivativeWrt) -> bool {
            if let DerivativeWrt::Noise(process) = wrt {
                return self
                    .noise_shadowed
                    .get(name)
                    .is_some_and(|axes| axes.contains(process));
            }
            self.shadowed
                .get(name)
                .is_some_and(|mask| mask & axis_bit(wrt, self.num_nodes) != 0)
        }

        /// Dependency mask of a variable (0 when not shadowed)
        fn axes_of(&self, name: &str) -> AxisMask {
            self.shadowed.get(name).copied().unwrap_or(0)
        }

        fn noise_axes_of(&self, name: &str) -> Option<&BTreeSet<usize>> {
            self.noise_shadowed.get(name)
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
    pub(crate) fn collect_var_names(expr: &IrExpr, out: &mut HashSet<SmolStr>) {
        visit_expr(expr, &mut |e| match e {
            IrExpr::Var(name) => {
                out.insert(name.clone());
            }
            IrExpr::VarIndexed { array, .. } => {
                out.insert(array.clone());
            }
            _ => {}
        });
    }

    pub(crate) fn collect_noise_definitions(
        expr: &IrExpr,
        out: &mut Vec<(
            NoiseSiteId,
            IrExpr,
            Option<IrExpr>,
            Option<NoiseTableData>,
            Option<SmolStr>,
        )>,
    ) {
        visit_expr(expr, &mut |node| match node {
            IrExpr::WhiteNoise { site, power, name } => out.push((
                *site,
                power.as_ref().clone(),
                None,
                None,
                name.as_deref().map(SmolStr::from),
            )),
            IrExpr::FlickerNoise {
                site,
                power,
                exponent,
                name,
            } => out.push((
                *site,
                power.as_ref().clone(),
                Some(exponent.as_ref().clone()),
                None,
                name.as_deref().map(SmolStr::from),
            )),
            IrExpr::NoiseTable {
                site,
                points,
                log_interp,
                name,
            } => out.push((
                *site,
                IrExpr::Const(1.0),
                None,
                Some(NoiseTableData {
                    points: points.clone(),
                    log_interp: *log_interp,
                }),
                name.as_deref().map(SmolStr::from),
            )),
            _ => {}
        });
    }

    /// Collect variable names appearing inside ddx() operands across an
    /// assignment tree (their derivative resolution reads shadows)
    pub(crate) fn collect_ddx_operand_names_in_expr(expr: &IrExpr, out: &mut HashSet<SmolStr>) {
        visit_expr(expr, &mut |e| {
            if let IrExpr::Ddx { expr, .. } = e {
                collect_var_names(expr, out);
            }
        });
    }

    pub(crate) fn collect_ddx_operand_names(
        items: &[IrAssignmentItem],
        out: &mut HashSet<SmolStr>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    collect_ddx_operand_names_in_expr(&assign.expr, out);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    collect_ddx_operand_names_in_expr(condition, out);
                    collect_ddx_operand_names(body, out);
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
        expr: &IrExpr,
        deps: &HashMap<SmolStr, AxisMask>,
        num_nodes: usize,
    ) -> AxisMask {
        let recurse = |e: &IrExpr| derivative_axes(e, deps, num_nodes);
        match expr {
            IrExpr::Voltage(p, n) => node_bit(*p) | node_bit(*n),
            IrExpr::BranchCurrent(k) => axis_bit(&DerivativeWrt::BranchCurrent(*k), num_nodes),
            // Current probes differentiate to zero in the DC Jacobian
            IrExpr::Current(..) => 0,
            IrExpr::Var(name) => deps.get(name).copied().unwrap_or(0),
            // The index only selects; the elements carry the slope
            IrExpr::VarIndexed { array, .. } => deps.get(array).copied().unwrap_or(0),
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_)
            | IrExpr::Analysis(_) => 0,
            IrExpr::Binary(op, l, r) => match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    recurse(l) | recurse(r)
                }
                // Piecewise-constant results: derivative identically zero
                BinaryOp::Mod
                | BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Shl
                | BinaryOp::Shr => 0,
            },
            IrExpr::Unary(UnaryOp::Neg | UnaryOp::Pos, e) => recurse(e),
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => 0,
            IrExpr::Limexp(e) | IrExpr::Ddt(e) => recurse(e),
            IrExpr::Idt(e, _) => recurse(e),
            IrExpr::IdtMod { expr, .. } => recurse(expr),
            IrExpr::Limit(e, _) | IrExpr::CanonicalLimit(e) => recurse(e),
            IrExpr::Call(func, args) => match func {
                IrFunction::Floor | IrFunction::Ceil => 0,
                _ => args.iter().map(recurse).fold(0, |acc, m| acc | m),
            },
            // The condition only selects; the branches carry the slope
            IrExpr::Conditional(_, t, e) => recurse(t) | recurse(e),
            IrExpr::TableLookup { input, .. } => recurse(input),
            IrExpr::AbsDelay {
                expr,
                delay_time,
                max_delay,
                ..
            } => recurse(expr) | recurse(delay_time) | max_delay.as_deref().map_or(0, recurse),
            IrExpr::AbsDelayDerivative {
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                ..
            } => {
                recurse(input)
                    | recurse(input_derivative)
                    | recurse(delay_time)
                    | recurse(delay_derivative)
                    | max_delay.as_deref().map_or(0, recurse)
            }
            IrExpr::Transition { expr, .. }
            | IrExpr::LaplaceZP { expr, .. }
            | IrExpr::LaplaceND { expr, .. }
            | IrExpr::LaplaceZPDerivative { expr, .. }
            | IrExpr::LaplaceNDDerivative { expr, .. }
            | IrExpr::ZiFilter { expr, .. }
            | IrExpr::ZiFilterDerivative { expr, .. }
            | IrExpr::Ddx { expr, .. } => recurse(expr),
            IrExpr::TransitionDerivative {
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
                ..
            } => {
                recurse(input)
                    | recurse(input_derivative)
                    | delay.as_deref().map_or(0, recurse)
                    | rise_time.as_deref().map_or(0, recurse)
                    | fall_time.as_deref().map_or(0, recurse)
            }
            IrExpr::Slew {
                expr,
                max_pos_slew,
                max_neg_slew,
                ..
            } => {
                recurse(expr)
                    | max_pos_slew.as_deref().map_or(0, recurse)
                    | max_neg_slew.as_deref().map_or(0, recurse)
            }
            IrExpr::SlewDerivative {
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
                ..
            } => {
                recurse(input)
                    | recurse(input_derivative)
                    | max_pos_slew.as_deref().map_or(0, recurse)
                    | max_pos_slew_derivative.as_deref().map_or(0, recurse)
                    | max_neg_slew.as_deref().map_or(0, recurse)
                    | max_neg_slew_derivative.as_deref().map_or(0, recurse)
            }
            IrExpr::DdtCompanion(e) | IrExpr::IdtCompanion(e) => recurse(e),
            IrExpr::TableDerivative { input, .. } => recurse(input),
            // Event detectors and noise sources are piecewise constant
            // (or zero) in the DC Jacobian
            IrExpr::Cross { .. }
            | IrExpr::LastCrossing { .. }
            | IrExpr::Above { .. }
            | IrExpr::Timer { .. }
            | IrExpr::WhiteNoise { .. }
            | IrExpr::FlickerNoise { .. }
            | IrExpr::NoiseTable { .. } => 0,
        }
    }

    pub(crate) fn expression_axes(
        expr: &IrExpr,
        shadows: &ShadowContext,
        num_nodes: usize,
    ) -> AxisMask {
        derivative_axes(expr, &shadows.shadowed, num_nodes)
    }

    /// Accumulate per-variable dependency axes over an item tree
    /// (fixpoint helper for [`build_shadow_assignments`]).
    ///
    /// A voltage-dependent write into any array element shadows the whole
    /// array: a runtime index may route the value to any slot, so every
    /// element (and the array name itself, checked by indexed reads)
    /// shares one mask.
    fn scan_shadowed(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        arrays: &[ArrayDef],
        num_nodes: usize,
        deps: &mut HashMap<SmolStr, AxisMask>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let mask = derivative_axes(&assign.expr, deps, num_nodes);
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
                    scan_shadowed(body, variables, arrays, num_nodes, deps, changed);
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
    /// second call site — inside `build_noise_shadow_assignments`, where the
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
        fn build(items: &[IrAssignmentItem], variables: &[VarDef], arrays: &[ArrayDef]) -> Self {
            let mut reads_by_target: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
            collect_liveness_edges(items, variables, &mut reads_by_target);
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
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        out: &mut HashMap<SmolStr, Vec<SmolStr>>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let target = match &assign.index {
                        Some(target) => target.array.clone(),
                        None => variables[assign.var_index].name.clone(),
                    };
                    let mut reads = HashSet::new();
                    collect_var_names(&assign.expr, &mut reads);
                    if let Some(target) = &assign.index {
                        collect_var_names(&target.index, &mut reads);
                    }
                    out.entry(target).or_default().extend(reads);
                }
                IrAssignmentItem::Loop { body, .. } => {
                    collect_liveness_edges(body, variables, out);
                }
            }
        }
    }

    /// Interleave shadow derivative updates before each original
    /// assignment, recursing into loop bodies so loop-carried voltage
    /// dependencies accumulate their derivatives per iteration
    fn interleave_shadows(
        items: Vec<IrAssignmentItem>,
        variables: &[VarDef],
        shadow_index: &HashMap<SmolStr, usize>,
        ctx: &ShadowContext,
        second_deps: &HashMap<SmolStr, AxisMask>,
        num_nodes: usize,
        num_branches: usize,
    ) -> Vec<IrAssignmentItem> {
        let mut rewritten = Vec::with_capacity(items.len() * 2);
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    if let Some(target) = &assign.index {
                        // Indexed write: the shadow run receives an indexed
                        // write of the value's derivative at the same slot,
                        // along the array's live axes only
                        let second_mask = second_deps.get(&target.array).copied().unwrap_or(0);
                        if second_mask != 0 {
                            for first in axes(num_nodes, num_branches) {
                                if second_mask & axis_bit(&first, num_nodes) == 0 {
                                    continue;
                                }
                                let first_deriv =
                                    simplify(differentiate_with_shadows(&assign.expr, &first, ctx));
                                let first_shadow_array =
                                    ShadowContext::shadow_name(&target.array, &first);
                                for second in axes(num_nodes, num_branches) {
                                    if second_mask & axis_bit(&second, num_nodes) == 0 {
                                        continue;
                                    }
                                    let second_deriv = simplify(differentiate_with_shadows(
                                        &first_deriv,
                                        &second,
                                        ctx,
                                    ));
                                    let second_shadow_array =
                                        ShadowContext::shadow_name(&first_shadow_array, &second);
                                    let shadow_base = ctx
                                        .array_shadow_base(&first_shadow_array, &second)
                                        .expect("second-order shadowed array has a shadow run");
                                    rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                        var_index: shadow_base,
                                        index: Some(IndexedTarget {
                                            array: second_shadow_array,
                                            len: target.len,
                                            lower: target.lower,
                                            index: target.index.clone(),
                                        }),
                                        expr: second_deriv,
                                    }));
                                }
                            }
                        }
                        let mask = ctx.axes_of(&target.array);
                        if mask != 0 {
                            for wrt in axes(num_nodes, num_branches) {
                                if mask & axis_bit(&wrt, num_nodes) == 0 {
                                    continue;
                                }
                                let deriv =
                                    simplify(differentiate_with_shadows(&assign.expr, &wrt, ctx));
                                let shadow_array = ShadowContext::shadow_name(&target.array, &wrt);
                                let shadow_base = ctx
                                    .array_shadow_base(&target.array, &wrt)
                                    .expect("shadowed array has a shadow run");
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_base,
                                    index: Some(IndexedTarget {
                                        array: shadow_array,
                                        len: target.len,
                                        lower: target.lower,
                                        index: target.index.clone(),
                                    }),
                                    expr: deriv,
                                }));
                            }
                        }
                        rewritten.push(IrAssignmentItem::Assign(assign));
                        continue;
                    }
                    let target = variables[assign.var_index].name.clone();
                    let second_mask = second_deps.get(&target).copied().unwrap_or(0);
                    if second_mask != 0 {
                        for first in axes(num_nodes, num_branches) {
                            if second_mask & axis_bit(&first, num_nodes) == 0 {
                                continue;
                            }
                            let first_deriv =
                                simplify(differentiate_with_shadows(&assign.expr, &first, ctx));
                            let first_shadow = ShadowContext::shadow_name(&target, &first);
                            for second in axes(num_nodes, num_branches) {
                                if second_mask & axis_bit(&second, num_nodes) == 0 {
                                    continue;
                                }
                                let second_deriv = simplify(differentiate_with_shadows(
                                    &first_deriv,
                                    &second,
                                    ctx,
                                ));
                                let second_shadow =
                                    ShadowContext::shadow_name(&first_shadow, &second);
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_index[&second_shadow],
                                    index: None,
                                    expr: second_deriv,
                                }));
                            }
                        }
                    }
                    let mask = ctx.axes_of(&target);
                    if mask != 0 {
                        for wrt in axes(num_nodes, num_branches) {
                            if mask & axis_bit(&wrt, num_nodes) == 0 {
                                continue;
                            }
                            let deriv =
                                simplify(differentiate_with_shadows(&assign.expr, &wrt, ctx));
                            let shadow = ShadowContext::shadow_name(&target, &wrt);
                            rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                var_index: shadow_index[&shadow],
                                index: None,
                                expr: deriv,
                            }));
                        }
                    }
                    rewritten.push(IrAssignmentItem::Assign(assign));
                }
                IrAssignmentItem::Loop { condition, body } => {
                    let body = interleave_shadows(
                        body,
                        variables,
                        shadow_index,
                        ctx,
                        second_deps,
                        num_nodes,
                        num_branches,
                    );
                    rewritten.push(IrAssignmentItem::Loop { condition, body });
                }
            }
        }
        rewritten
    }

    /// Build shadow derivative assignments for voltage-dependent variables.
    ///
    /// Rewrites `ir.assignments` so that each assignment to a
    /// voltage-dependent variable is preceded by assignments computing the
    /// variable's partial derivative w.r.t. every node voltage and
    /// branch-current unknown. Shadow variables are appended to
    /// `ir.variables`.
    pub fn build_shadow_assignments(
        ir: &mut DeviceIR,
        num_nodes: usize,
        num_branches: usize,
        shadow_roots: &HashSet<SmolStr>,
        second_shadow_roots: &HashSet<SmolStr>,
    ) -> ShadowContext {
        // Fixpoint: a variable depends on an axis if any assignment to it
        // reads a probe of that axis or another variable depending on it.
        let span = crate::metrics::FineSpan::new("ir.shadow_axis_fixpoint");
        let mut deps: HashMap<SmolStr, AxisMask> = HashMap::new();
        let mut axis_passes = 0_usize;
        loop {
            let mut changed = false;
            axis_passes += 1;
            scan_shadowed(
                &ir.assignments,
                &ir.variables,
                &ir.arrays,
                num_nodes,
                &mut deps,
                &mut changed,
            );
            if !changed {
                break;
            }
        }
        span.finish(&format!("passes={axis_passes} shadowed={}", deps.len()));

        // Backward liveness: a shadow matters only when the equation
        // Jacobians can reach it — the variable feeds a contribution (or
        // ddx operand) directly, or feeds an assignment to a live
        // variable. Dead shadows (operating-point reporting chains) are
        // dropped before any slot is allocated.
        let span = crate::metrics::FineSpan::new("ir.shadow_liveness");
        let liveness = LivenessGraph::build(&ir.assignments, &ir.variables, &ir.arrays);
        let live = liveness.live_from(shadow_roots);
        let second_live = liveness.live_from(second_shadow_roots);
        span.finish(&format!(
            "live={} second_live={}",
            live.len(),
            second_live.len()
        ));
        deps.retain(|name, _| live.contains(name) || second_live.contains(name));
        let second_deps: HashMap<SmolStr, AxisMask> = deps
            .iter()
            .filter(|(name, _)| second_live.contains(*name))
            .map(|(name, mask)| (name.clone(), *mask))
            .collect();

        if deps.is_empty() {
            return ShadowContext::default();
        }

        let span = crate::metrics::FineSpan::new("ir.shadow_layout");

        // Register shadow variables along each variable's live axes only:
        // a value computed from V(g) and V(s) never varies with the drain
        // or any branch unknown, so those slots (and their update
        // assignments downstream) never exist. Array elements get their
        // slots in contiguous runs (allocated below) so runtime-indexed
        // reads and writes can address d(arr[i]) as
        // shadow_base + (i - lower); the scalar loop must skip them.
        let array_member: HashSet<SmolStr> = ir
            .arrays
            .iter()
            .filter(|a| deps.get(&a.name).copied().unwrap_or(0) != 0)
            .flat_map(|a| {
                std::iter::once(a.name.clone()).chain(
                    ir.variables[a.base..a.base + a.len]
                        .iter()
                        .map(|v| v.name.clone()),
                )
            })
            .collect();
        let mut shadow_index: HashMap<SmolStr, usize> = HashMap::new();
        let mut scalar_shadow_layout = deps
            .iter()
            .filter(|(name, _)| !array_member.contains(*name))
            .map(|(name, mask)| (name.clone(), *mask))
            .collect::<Vec<_>>();
        scalar_shadow_layout.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        for (name, mask) in &scalar_shadow_layout {
            for wrt in axes(num_nodes, num_branches) {
                if mask & axis_bit(&wrt, num_nodes) == 0 {
                    continue;
                }
                let shadow = ShadowContext::shadow_name(name, &wrt);
                shadow_index.insert(shadow.clone(), ir.variables.len());
                ir.variables.push(VarDef {
                    name: shadow,
                    is_state: false,
                });
            }
        }
        for (name, _) in scalar_shadow_layout
            .iter()
            .filter(|(name, _)| second_deps.contains_key(name))
        {
            let mask = second_deps
                .get(name)
                .copied()
                .expect("filtered scalar second-derivative layout has a dependency mask");
            for first in axes(num_nodes, num_branches) {
                if mask & axis_bit(&first, num_nodes) == 0 {
                    continue;
                }
                let first_shadow = ShadowContext::shadow_name(name, &first);
                for second in axes(num_nodes, num_branches) {
                    if mask & axis_bit(&second, num_nodes) == 0 {
                        continue;
                    }
                    let second_shadow = ShadowContext::shadow_name(&first_shadow, &second);
                    shadow_index.insert(second_shadow.clone(), ir.variables.len());
                    ir.variables.push(VarDef {
                        name: second_shadow,
                        is_state: false,
                    });
                }
            }
        }

        // Contiguous shadow runs per (array, live axis)
        let mut array_shadow_base: HashMap<SmolStr, usize> = HashMap::new();
        let mut shadow_runs: Vec<VarDef> = Vec::new();
        for array in ir.arrays.iter() {
            let mask = deps.get(&array.name).copied().unwrap_or(0);
            if mask == 0 {
                continue;
            }
            for wrt in axes(num_nodes, num_branches) {
                if mask & axis_bit(&wrt, num_nodes) == 0 {
                    continue;
                }
                let run_base = ir.variables.len() + shadow_runs.len();
                array_shadow_base.insert(ShadowContext::shadow_name(&array.name, &wrt), run_base);
                for k in array.lower..array.lower + array.len as i64 {
                    let element = format!("{}[{k}]", array.name);
                    let shadow = ShadowContext::shadow_name(&element, &wrt);
                    shadow_index.insert(shadow.clone(), run_base + (k - array.lower) as usize);
                    shadow_runs.push(VarDef {
                        name: shadow,
                        is_state: false,
                    });
                }
            }
        }
        for array in ir.arrays.iter() {
            let mask = second_deps.get(&array.name).copied().unwrap_or(0);
            if mask == 0 {
                continue;
            }
            for first in axes(num_nodes, num_branches) {
                if mask & axis_bit(&first, num_nodes) == 0 {
                    continue;
                }
                let first_shadow_array = ShadowContext::shadow_name(&array.name, &first);
                for second in axes(num_nodes, num_branches) {
                    if mask & axis_bit(&second, num_nodes) == 0 {
                        continue;
                    }
                    let second_shadow_array =
                        ShadowContext::shadow_name(&first_shadow_array, &second);
                    let run_base = ir.variables.len() + shadow_runs.len();
                    array_shadow_base.insert(second_shadow_array, run_base);
                    for k in array.lower..array.lower + array.len as i64 {
                        let element = format!("{}[{k}]", array.name);
                        let first_shadow_element = ShadowContext::shadow_name(&element, &first);
                        let second_shadow_element =
                            ShadowContext::shadow_name(&first_shadow_element, &second);
                        shadow_index.insert(
                            second_shadow_element.clone(),
                            run_base + (k - array.lower) as usize,
                        );
                        shadow_runs.push(VarDef {
                            name: second_shadow_element,
                            is_state: false,
                        });
                    }
                }
            }
        }
        ir.variables.extend(shadow_runs);

        let mut shadowed = deps;
        for (name, mask) in &second_deps {
            if array_member.contains(name) {
                continue;
            }
            for first in axes(num_nodes, num_branches) {
                if mask & axis_bit(&first, num_nodes) == 0 {
                    continue;
                }
                shadowed.insert(ShadowContext::shadow_name(name, &first), *mask);
            }
        }
        for array in ir.arrays.iter() {
            let mask = second_deps.get(&array.name).copied().unwrap_or(0);
            if mask == 0 {
                continue;
            }
            for first in axes(num_nodes, num_branches) {
                if mask & axis_bit(&first, num_nodes) == 0 {
                    continue;
                }
                shadowed.insert(ShadowContext::shadow_name(&array.name, &first), mask);
                for k in array.lower..array.lower + array.len as i64 {
                    let element = format!("{}[{k}]", array.name);
                    shadowed.insert(ShadowContext::shadow_name(&element, &first), mask);
                }
            }
        }

        let ctx = ShadowContext {
            shadowed,
            array_shadow_base,
            num_nodes,
            noise_shadowed: HashMap::new(),
        };
        span.finish(&format!("shadow_slots={}", shadow_index.len()));

        // Interleave shadow updates before each original assignment.
        // Both the derivative and the original expression read the
        // pre-assignment values, so the shadows must be written first.
        let span = crate::metrics::FineSpan::new("ir.shadow_interleave");
        let originals = std::mem::take(&mut ir.assignments);
        ir.assignments = interleave_shadows(
            originals,
            &ir.variables,
            &shadow_index,
            &ctx,
            &second_deps,
            num_nodes,
            num_branches,
        );
        span.finish(&format!("assignments={}", ir.assignments.len()));

        ctx
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
    /// memoized by expression identity so zero-factor and constant-condition
    /// checks stay linear even for deeply skewed expression trees.
    fn simplified_constant(
        expr: &IrExpr,
        constants: &mut HashMap<*const IrExpr, SimplifiedConstant>,
    ) -> SimplifiedConstant {
        let key = std::ptr::from_ref(expr);
        if let Some(value) = constants.get(&key) {
            return *value;
        }
        let value = match expr {
            IrExpr::Const(value) => SimplifiedConstant::Value(*value),
            IrExpr::Binary(op, left, right) => {
                let left = simplified_constant(left, constants);
                let right = simplified_constant(right, constants);
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
            IrExpr::Unary(op, inner) => {
                let inner = simplified_constant(inner, constants);
                match (op, inner) {
                    (UnaryOp::Neg, SimplifiedConstant::Value(value)) => {
                        SimplifiedConstant::Value(-value)
                    }
                    (UnaryOp::Pos, value) => value,
                    _ => SimplifiedConstant::Other,
                }
            }
            IrExpr::Conditional(condition, then_expr, else_expr) => {
                let condition = simplified_constant(condition, constants);
                let then_expr = simplified_constant(then_expr, constants);
                let else_expr = simplified_constant(else_expr, constants);
                match condition {
                    SimplifiedConstant::Value(value) if value != 0.0 => then_expr,
                    SimplifiedConstant::Value(_) => else_expr,
                    SimplifiedConstant::Other => SimplifiedConstant::Other,
                }
            }
            IrExpr::Call(_, arguments) => {
                for argument in arguments {
                    simplified_constant(argument, constants);
                }
                SimplifiedConstant::Other
            }
            IrExpr::DdtCompanion(inner) | IrExpr::IdtCompanion(inner) => {
                if simplified_constant(inner, constants).is_zero() {
                    SimplifiedConstant::Value(0.0)
                } else {
                    SimplifiedConstant::Other
                }
            }
            _ => SimplifiedConstant::Other,
        };
        constants.insert(key, value);
        value
    }

    /// Collect the independent noise processes that can affect `expr`.
    ///
    /// This is the set-valued counterpart of [`differentiate_with_shadows`]
    /// for [`DerivativeWrt::Noise`].  Keeping the traversal here in lock-step
    /// with that routine is important: operands which are metadata or merely
    /// select a value (noise PSDs, array indices, conditional predicates, and
    /// stateful-operator timing arguments) are deliberately not traversed.
    /// Actual derivative expressions are still built later, after liveness
    /// has removed processes which cannot reach a contribution.
    fn collect_expression_noise_axes(
        expr: &IrExpr,
        deps: &HashMap<SmolStr, BTreeSet<usize>>,
        num_processes: usize,
        constants: &mut HashMap<*const IrExpr, SimplifiedConstant>,
        axes: &mut BTreeSet<usize>,
    ) {
        macro_rules! collect {
            ($value:expr) => {
                collect_expression_noise_axes($value, deps, num_processes, constants, axes)
            };
        }
        match expr {
            IrExpr::Var(name) => {
                if let Some(processes) = deps.get(name) {
                    axes.extend(processes.iter().copied());
                }
            }
            // A runtime index selects an element; it is not part of the
            // differentiable value path.
            IrExpr::VarIndexed { array, .. } => {
                if let Some(processes) = deps.get(array) {
                    axes.extend(processes.iter().copied());
                }
            }
            IrExpr::WhiteNoise { site, .. }
            | IrExpr::FlickerNoise { site, .. }
            | IrExpr::NoiseTable { site, .. } => {
                let process = site.ordinal as usize;
                if process < num_processes {
                    axes.insert(process);
                }
            }
            IrExpr::Binary(op, left, right) => match op {
                BinaryOp::Add | BinaryOp::Sub => {
                    collect!(left);
                    collect!(right);
                }
                BinaryOp::Mul => {
                    // `simplify` removes either product-rule term when its
                    // primal multiplier is identically zero.
                    if !simplified_constant(right, constants).is_zero() {
                        collect!(left);
                    }
                    if !simplified_constant(left, constants).is_zero() {
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
                    if !simplified_constant(left, constants).is_zero() {
                        collect!(right);
                    }
                }
                BinaryOp::Pow => {
                    if let IrExpr::Const(exponent) = right.as_ref() {
                        if *exponent != 0.0 {
                            collect!(left);
                        }
                    } else {
                        // d(u^v) contains v' and, unless v is identically
                        // zero, u'.
                        collect!(right);
                        if !simplified_constant(right, constants).is_zero() {
                            collect!(left);
                        }
                    }
                }
                // These operators are piecewise constant under the Jacobian
                // convention used by `differentiate_with_shadows`.
                BinaryOp::Mod
                | BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Shl
                | BinaryOp::Shr => {}
            },
            IrExpr::Unary(UnaryOp::Neg | UnaryOp::Pos, inner) => collect!(inner),
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => {}
            IrExpr::Conditional(condition, then_expr, else_expr) => {
                // The predicate selects a derivative branch but is not itself
                // differentiated. Match `simplify`'s constant-branch fold.
                match simplified_constant(condition, constants) {
                    SimplifiedConstant::Value(value) if value != 0.0 => collect!(then_expr),
                    SimplifiedConstant::Value(_) => collect!(else_expr),
                    SimplifiedConstant::Other => {
                        collect!(then_expr);
                        collect!(else_expr);
                    }
                }
            }
            IrExpr::Call(function, arguments) => match (function, arguments.as_slice()) {
                (IrFunction::Floor | IrFunction::Ceil, [_]) => {}
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
                    [inner],
                ) => collect!(inner),
                (IrFunction::Atan2, [ordinate, abscissa]) => {
                    if !simplified_constant(abscissa, constants).is_zero() {
                        collect!(ordinate);
                    }
                    if !simplified_constant(ordinate, constants).is_zero() {
                        collect!(abscissa);
                    }
                }
                (IrFunction::Min | IrFunction::Max, [left, right]) => {
                    collect!(left);
                    collect!(right);
                }
                (IrFunction::Pow, [base, exponent]) => {
                    if let IrExpr::Const(exponent) = exponent {
                        if *exponent != 0.0 {
                            collect!(base);
                        }
                    } else {
                        collect!(exponent);
                        if !simplified_constant(exponent, constants).is_zero() {
                            collect!(base);
                        }
                    }
                }
                // Malformed calls differentiate to zero and are diagnosed by
                // their construction path.
                _ => {}
            },
            IrExpr::Limexp(inner)
            | IrExpr::Ddt(inner)
            | IrExpr::CanonicalLimit(inner)
            | IrExpr::LaplaceND { expr: inner, .. }
            | IrExpr::LaplaceZP { expr: inner, .. }
            | IrExpr::LaplaceNDDerivative { expr: inner, .. }
            | IrExpr::LaplaceZPDerivative { expr: inner, .. }
            | IrExpr::ZiFilter { expr: inner, .. }
            | IrExpr::ZiFilterDerivative { expr: inner, .. } => collect!(inner),
            IrExpr::Idt(inner, _) | IrExpr::Limit(inner, _) => collect!(inner),
            IrExpr::IdtMod { expr: inner, .. } => collect!(inner),
            IrExpr::TableLookup { input, .. } => collect!(input),
            IrExpr::AbsDelay {
                expr: input,
                delay_time,
                ..
            } => {
                collect!(input);
                collect!(delay_time);
            }
            IrExpr::AbsDelayDerivative {
                input_derivative,
                delay_derivative,
                ..
            } => {
                collect!(input_derivative);
                collect!(delay_derivative);
            }
            IrExpr::Transition { expr: input, .. } => collect!(input),
            IrExpr::TransitionDerivative {
                input_derivative, ..
            } => collect!(input_derivative),
            IrExpr::Slew {
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
            IrExpr::SlewDerivative {
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
            IrExpr::Ddx { .. } => {
                // ddx is resolved along its solver axis before the outer noise
                // derivative. Preserve that ordering; walking the raw operand
                // would incorrectly retain noise which its ddx eliminates.
                let shadows = ShadowContext {
                    noise_shadowed: deps.clone(),
                    ..ShadowContext::default()
                };
                let resolved = resolve_ddx(expr, &shadows);
                axes.extend(expression_noise_axes(&resolved, deps, num_processes));
            }
            // Solver probes, parameters, analysis/event queries, companion
            // derivative carriers, and table slopes are constant on a noise
            // realization axis.
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Voltage(_, _)
            | IrExpr::Current(_, _)
            | IrExpr::BranchCurrent(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_)
            | IrExpr::Cross { .. }
            | IrExpr::LastCrossing { .. }
            | IrExpr::Analysis(_)
            | IrExpr::Above { .. }
            | IrExpr::Timer { .. }
            | IrExpr::DdtCompanion(_)
            | IrExpr::IdtCompanion(_)
            | IrExpr::TableDerivative { .. } => {}
        }
    }

    fn expression_noise_axes(
        expr: &IrExpr,
        deps: &HashMap<SmolStr, BTreeSet<usize>>,
        num_processes: usize,
    ) -> BTreeSet<usize> {
        let mut constants = HashMap::new();
        let mut axes = BTreeSet::new();
        collect_expression_noise_axes(expr, deps, num_processes, &mut constants, &mut axes);
        axes
    }

    fn scan_noise_shadowed(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        arrays: &[ArrayDef],
        num_processes: usize,
        deps: &mut HashMap<SmolStr, BTreeSet<usize>>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let axes = expression_noise_axes(&assign.expr, deps, num_processes);
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
                    scan_noise_shadowed(body, variables, arrays, num_processes, deps, changed)
                }
            }
        }
    }

    fn interleave_noise_shadows(
        items: Vec<IrAssignmentItem>,
        variables: &[VarDef],
        shadow_index: &HashMap<SmolStr, usize>,
        ctx: &ShadowContext,
    ) -> Vec<IrAssignmentItem> {
        let mut rewritten = Vec::with_capacity(items.len().saturating_mul(2));
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let target_name = assign
                        .index
                        .as_ref()
                        .map(|target| target.array.clone())
                        .or_else(|| variables.get(assign.var_index).map(|var| var.name.clone()));
                    if let Some(target_name) = target_name {
                        let processes = ctx
                            .noise_axes_of(&target_name)
                            .into_iter()
                            .flatten()
                            .copied()
                            .collect::<Vec<_>>();
                        for process in processes {
                            let axis = DerivativeWrt::Noise(process);
                            let derivative =
                                simplify(differentiate_with_shadows(&assign.expr, &axis, ctx));
                            let shadow_name = ShadowContext::shadow_name(&target_name, &axis);
                            if let Some(target) = &assign.index {
                                let shadow_base = ctx
                                    .array_shadow_base(&target.array, &axis)
                                    .expect("noise-shadowed array has a contiguous run");
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_base,
                                    index: Some(IndexedTarget {
                                        array: shadow_name,
                                        len: target.len,
                                        lower: target.lower,
                                        index: target.index.clone(),
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
                        body: interleave_noise_shadows(body, variables, shadow_index, ctx),
                    });
                }
            }
        }
        rewritten
    }

    /// Add assignment shadows for syntactic noise processes.  These are
    /// separate from solver-axis shadows so process count never enlarges the
    /// nonlinear Jacobian or its second-derivative layout.
    pub fn build_noise_shadow_assignments(
        ir: &mut DeviceIR,
        num_processes: usize,
        shadow_roots: &HashSet<SmolStr>,
        ctx: &mut ShadowContext,
    ) {
        if num_processes == 0 {
            return;
        }
        let span = crate::metrics::FineSpan::new("ir.noise_axis_fixpoint");
        let mut deps = HashMap::new();
        let mut passes = 0_usize;
        loop {
            let mut changed = false;
            passes += 1;
            scan_noise_shadowed(
                &ir.assignments,
                &ir.variables,
                &ir.arrays,
                num_processes,
                &mut deps,
                &mut changed,
            );
            if !changed {
                break;
            }
        }
        span.finish(&format!("passes={passes} shadowed={}", deps.len()));
        if deps.is_empty() {
            return;
        }
        let span = crate::metrics::FineSpan::new("ir.noise_liveness");
        let liveness = LivenessGraph::build(&ir.assignments, &ir.variables, &ir.arrays);
        let live = liveness.live_from(shadow_roots);
        span.finish(&format!(
            "assignments={} live={}",
            ir.assignments.len(),
            live.len()
        ));
        deps.retain(|name, _| live.contains(name));
        if deps.is_empty() {
            return;
        }

        let span = crate::metrics::FineSpan::new("ir.noise_shadow_layout");
        let array_members = ir
            .arrays
            .iter()
            .filter(|array| deps.get(&array.name).is_some_and(|axes| !axes.is_empty()))
            .flat_map(|array| {
                std::iter::once(array.name.clone()).chain(
                    ir.variables[array.base..array.base + array.len]
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
            for process in axes {
                let shadow =
                    ShadowContext::shadow_name(name.as_str(), &DerivativeWrt::Noise(process));
                shadow_index.insert(shadow.clone(), ir.variables.len());
                ir.variables.push(VarDef {
                    name: shadow,
                    is_state: false,
                });
            }
        }
        for array in &ir.arrays {
            let processes = deps.get(&array.name).cloned().unwrap_or_default();
            for process in processes {
                let axis = DerivativeWrt::Noise(process);
                let run_name = ShadowContext::shadow_name(&array.name, &axis);
                let run_base = ir.variables.len();
                ctx.array_shadow_base.insert(run_name, run_base);
                for index in array.lower..array.lower + array.len as i64 {
                    let element = format!("{}[{index}]", array.name);
                    let shadow = ShadowContext::shadow_name(&element, &axis);
                    shadow_index.insert(shadow.clone(), ir.variables.len());
                    ir.variables.push(VarDef {
                        name: shadow,
                        is_state: false,
                    });
                }
            }
        }
        ctx.noise_shadowed = deps;
        span.finish(&format!("shadow_slots={}", shadow_index.len()));
        let span = crate::metrics::FineSpan::new("ir.noise_shadow_interleave");
        let originals = std::mem::take(&mut ir.assignments);
        ir.assignments = interleave_noise_shadows(originals, &ir.variables, &shadow_index, ctx);
        span.finish(&format!("assignments={}", ir.assignments.len()));
    }

    /// Rewrite I(a,b) probes of branches carrying potential contributions
    /// into branch-current unknown references. The table maps a normalized
    /// (min,max) node pair to (ordinal, oriented positive node); a probe
    /// against the orientation negates.
    pub fn rewrite_branch_probes(
        expr: &IrExpr,
        table: &HashMap<(usize, usize), (usize, usize)>,
    ) -> IrExpr {
        map_expr(expr, &mut |e| {
            if let IrExpr::Current(p, n) = e {
                let key = (*p.min(n), *p.max(n));
                if let Some(&(ordinal, oriented_pos)) = table.get(&key) {
                    let unknown = IrExpr::BranchCurrent(ordinal);
                    return Some(if *p == oriented_pos {
                        unknown
                    } else {
                        IrExpr::Unary(UnaryOp::Neg, Box::new(unknown))
                    });
                }
            }
            None
        })
    }

    /// Apply [`rewrite_branch_probes`] across an assignment-item tree
    pub fn rewrite_branch_probes_in_items(
        items: &mut [IrAssignmentItem],
        table: &HashMap<(usize, usize), (usize, usize)>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    assign.expr = rewrite_branch_probes(&assign.expr, table);
                    if let Some(target) = &mut assign.index {
                        target.index = rewrite_branch_probes(&target.index, table);
                    }
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = rewrite_branch_probes(condition, table);
                    rewrite_branch_probes_in_items(body, table);
                }
            }
        }
    }

    /// Resolve ddx() operators into explicit derivative expressions
    pub fn resolve_ddx(expr: &IrExpr, shadows: &ShadowContext) -> IrExpr {
        map_expr(expr, &mut |e| {
            if let IrExpr::Ddx { expr, axis } = e {
                let inner = resolve_ddx(expr, shadows);
                Some(match axis {
                    DdxAxis::Potential {
                        pos: Some(pos),
                        neg: None,
                    } => simplify(differentiate_with_shadows(
                        &inner,
                        &DerivativeWrt::Voltage(*pos),
                        shadows,
                    )),
                    DdxAxis::Potential {
                        pos: None,
                        neg: Some(neg),
                    } => simplify(IrExpr::Unary(
                        UnaryOp::Neg,
                        Box::new(differentiate_with_shadows(
                            &inner,
                            &DerivativeWrt::Voltage(*neg),
                            shadows,
                        )),
                    )),
                    // ddx(f, V(a,b)): when f depends on the pair only
                    // through V(a)-V(b), (df/dVa - df/dVb)/2 is exactly
                    // df/d(Va-Vb).
                    DdxAxis::Potential {
                        pos: Some(pos),
                        neg: Some(neg),
                    } => {
                        let d_pos = simplify(differentiate_with_shadows(
                            &inner,
                            &DerivativeWrt::Voltage(*pos),
                            shadows,
                        ));
                        let d_neg = simplify(differentiate_with_shadows(
                            &inner,
                            &DerivativeWrt::Voltage(*neg),
                            shadows,
                        ));
                        simplify(IrExpr::Binary(
                            BinaryOp::Mul,
                            Box::new(IrExpr::Const(0.5)),
                            Box::new(IrExpr::Binary(
                                BinaryOp::Sub,
                                Box::new(d_pos),
                                Box::new(d_neg),
                            )),
                        ))
                    }
                    DdxAxis::Potential {
                        pos: None,
                        neg: None,
                    } => IrExpr::Const(0.0),
                    DdxAxis::BranchCurrent { ordinal, reversed } => {
                        let derivative = simplify(differentiate_with_shadows(
                            &inner,
                            &DerivativeWrt::BranchCurrent(*ordinal),
                            shadows,
                        ));
                        if *reversed {
                            simplify(IrExpr::Unary(UnaryOp::Neg, Box::new(derivative)))
                        } else {
                            derivative
                        }
                    }
                })
            } else {
                None
            }
        })
    }

    /// Resolve ddx() operators across an assignment-item tree
    /// Resolve every `ddx` in an assignment tree.
    ///
    /// Each rewrite goes through [`map_expr`], which rebuilds the expression
    /// whether or not it changed. Almost none of these expressions hold a
    /// `ddx` — after shadow interleaving the tree is a million derivative
    /// assignments and the operators are the couple of dozen sites the author
    /// wrote — so the untouched ones are copied for nothing. Asking
    /// [`contains_ddx`] first replaces that copy with a read.
    pub fn resolve_ddx_in_items(items: &mut [IrAssignmentItem], shadows: &ShadowContext) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    if contains_ddx(&assign.expr) {
                        assign.expr = resolve_ddx(&assign.expr, shadows);
                    }
                    if let Some(target) = &mut assign.index
                        && contains_ddx(&target.index)
                    {
                        target.index = resolve_ddx(&target.index, shadows);
                    }
                }
                IrAssignmentItem::Loop { condition, body } => {
                    if contains_ddx(condition) {
                        *condition = resolve_ddx(condition, shadows);
                    }
                    resolve_ddx_in_items(body, shadows);
                }
            }
        }
    }

    /// Read-only sibling of [`map_expr`]: the same nodes, in the same
    /// pre-order, without rebuilding the tree.
    ///
    /// [`map_expr`] reconstructs every node it walks, so a caller that only
    /// wanted to *look* at an expression paid for a full copy of it and then
    /// dropped the copy. On a compact model whose analog block has been
    /// expanded into a million shadow assignments that copy is the whole
    /// program, and the passes that only inspect — collecting the names a
    /// write reads, finding noise definitions, asking whether a subtree holds
    /// a `ddx` at all — were each paying it.
    ///
    /// The match below mirrors [`map_expr`]'s child slots exactly, including
    /// where it does *not* descend: the coefficient vectors of a Laplace or
    /// Zi filter, and every event, noise, and companion node, are leaves in
    /// both. `visit_expr_walks_the_same_child_slots_as_map_expr` pins that
    /// agreement per variant, and the arms are written out rather than left
    /// to a wildcard so a new [`IrExpr`] variant has to be classified here.
    pub(crate) fn visit_expr(expr: &IrExpr, f: &mut impl FnMut(&IrExpr)) {
        f(expr);
        let mut recurse = |e: &IrExpr| visit_expr(e, f);
        match expr {
            IrExpr::Binary(_, left, right) => {
                recurse(left);
                recurse(right);
            }
            IrExpr::Unary(_, inner) => recurse(inner),
            IrExpr::Call(_, args) => args.iter().for_each(recurse),
            IrExpr::Conditional(condition, then_expr, else_expr) => {
                recurse(condition);
                recurse(then_expr);
                recurse(else_expr);
            }
            IrExpr::Ddt(inner)
            | IrExpr::Limexp(inner)
            | IrExpr::CanonicalLimit(inner)
            | IrExpr::Ddx { expr: inner, .. }
            | IrExpr::LaplaceND { expr: inner, .. }
            | IrExpr::LaplaceNDDerivative { expr: inner, .. }
            | IrExpr::LaplaceZP { expr: inner, .. }
            | IrExpr::LaplaceZPDerivative { expr: inner, .. }
            | IrExpr::TableLookup { input: inner, .. }
            | IrExpr::VarIndexed { index: inner, .. } => recurse(inner),
            IrExpr::Idt(inner, second) | IrExpr::Limit(inner, second) => {
                recurse(inner);
                second.iter().for_each(|e| recurse(e));
            }
            IrExpr::IdtMod {
                expr,
                ic,
                modulus,
                offset,
            } => {
                recurse(expr);
                ic.iter().for_each(|e| recurse(e));
                recurse(modulus);
                offset.iter().for_each(|e| recurse(e));
            }
            IrExpr::AbsDelay {
                expr,
                delay_time,
                max_delay,
                ..
            } => {
                recurse(expr);
                recurse(delay_time);
                max_delay.iter().for_each(|e| recurse(e));
            }
            IrExpr::AbsDelayDerivative {
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                ..
            } => {
                recurse(input);
                recurse(input_derivative);
                recurse(delay_time);
                recurse(delay_derivative);
                max_delay.iter().for_each(|e| recurse(e));
            }
            IrExpr::Transition {
                expr,
                delay,
                rise_time,
                fall_time,
                ..
            } => {
                recurse(expr);
                delay.iter().for_each(|e| recurse(e));
                rise_time.iter().for_each(|e| recurse(e));
                fall_time.iter().for_each(|e| recurse(e));
            }
            IrExpr::TransitionDerivative {
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
                ..
            } => {
                recurse(input);
                recurse(input_derivative);
                delay.iter().for_each(|e| recurse(e));
                rise_time.iter().for_each(|e| recurse(e));
                fall_time.iter().for_each(|e| recurse(e));
            }
            IrExpr::Slew {
                expr,
                max_pos_slew,
                max_neg_slew,
                ..
            } => {
                recurse(expr);
                max_pos_slew.iter().for_each(|e| recurse(e));
                max_neg_slew.iter().for_each(|e| recurse(e));
            }
            IrExpr::SlewDerivative {
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
                ..
            } => {
                recurse(input);
                recurse(input_derivative);
                max_pos_slew.iter().for_each(|e| recurse(e));
                max_pos_slew_derivative.iter().for_each(|e| recurse(e));
                max_neg_slew.iter().for_each(|e| recurse(e));
                max_neg_slew_derivative.iter().for_each(|e| recurse(e));
            }
            IrExpr::ZiFilter {
                expr,
                period,
                transition,
                first_transition,
                ..
            }
            | IrExpr::ZiFilterDerivative {
                expr,
                period,
                transition,
                first_transition,
                ..
            } => {
                recurse(expr);
                recurse(period);
                recurse(transition);
                recurse(first_transition);
            }
            // Leaves for `map_expr`, so leaves here. The event, noise and
            // companion nodes carry operands that it never descends into,
            // and rewriting that would change what every existing caller
            // sees, not just what this one costs.
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Var(_)
            | IrExpr::Voltage(..)
            | IrExpr::Current(..)
            | IrExpr::BranchCurrent(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_)
            | IrExpr::Analysis(_)
            | IrExpr::Cross { .. }
            | IrExpr::LastCrossing { .. }
            | IrExpr::WhiteNoise { .. }
            | IrExpr::FlickerNoise { .. }
            | IrExpr::NoiseTable { .. }
            | IrExpr::Above { .. }
            | IrExpr::Timer { .. }
            | IrExpr::DdtCompanion(_)
            | IrExpr::IdtCompanion(_)
            | IrExpr::TableDerivative { .. } => {}
        }
    }

    /// Whether a subtree holds a `ddx` operator.
    ///
    /// [`resolve_ddx`] rewrites through [`map_expr`], which copies whatever it
    /// walks whether or not anything changed. A module's `ddx` operators are
    /// a couple of dozen sites in a program of a million assignments, so
    /// asking first turns a whole-program copy into a whole-program read.
    pub(crate) fn contains_ddx(expr: &IrExpr) -> bool {
        let mut found = false;
        visit_expr(expr, &mut |node| {
            if matches!(node, IrExpr::Ddx { .. }) {
                found = true;
            }
        });
        found
    }

    /// Structurally map an IR expression bottom-up. The closure may replace
    /// a node entirely (returning Some) before its children are visited.
    pub(crate) fn map_expr(expr: &IrExpr, f: &mut impl FnMut(&IrExpr) -> Option<IrExpr>) -> IrExpr {
        if let Some(replacement) = f(expr) {
            return replacement;
        }
        match expr {
            IrExpr::Binary(op, l, r) => {
                IrExpr::Binary(*op, Box::new(map_expr(l, f)), Box::new(map_expr(r, f)))
            }
            IrExpr::Unary(op, e) => IrExpr::Unary(*op, Box::new(map_expr(e, f))),
            IrExpr::Call(func, args) => {
                IrExpr::Call(*func, args.iter().map(|a| map_expr(a, f)).collect())
            }
            IrExpr::Conditional(c, t, e) => IrExpr::Conditional(
                Box::new(map_expr(c, f)),
                Box::new(map_expr(t, f)),
                Box::new(map_expr(e, f)),
            ),
            IrExpr::Ddt(e) => IrExpr::Ddt(Box::new(map_expr(e, f))),
            IrExpr::Idt(e, ic) => IrExpr::Idt(
                Box::new(map_expr(e, f)),
                ic.as_ref().map(|e| Box::new(map_expr(e, f))),
            ),
            IrExpr::IdtMod {
                expr,
                ic,
                modulus,
                offset,
            } => IrExpr::IdtMod {
                expr: Box::new(map_expr(expr, f)),
                ic: ic.as_ref().map(|e| Box::new(map_expr(e, f))),
                modulus: Box::new(map_expr(modulus, f)),
                offset: offset.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Limexp(e) => IrExpr::Limexp(Box::new(map_expr(e, f))),
            IrExpr::Limit(e, step) => IrExpr::Limit(
                Box::new(map_expr(e, f)),
                step.as_ref().map(|e| Box::new(map_expr(e, f))),
            ),
            IrExpr::CanonicalLimit(e) => IrExpr::CanonicalLimit(Box::new(map_expr(e, f))),
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => IrExpr::TableLookup {
                input: Box::new(map_expr(input, f)),
                x_data: x_data.clone(),
                y_data: y_data.clone(),
            },
            IrExpr::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => IrExpr::AbsDelay {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                delay_time: Box::new(map_expr(delay_time, f)),
                max_delay: max_delay.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::AbsDelayDerivative {
                site,
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                derivative_order,
            } => IrExpr::AbsDelayDerivative {
                site: *site,
                input: Box::new(map_expr(input, f)),
                input_derivative: Box::new(map_expr(input_derivative, f)),
                delay_time: Box::new(map_expr(delay_time, f)),
                delay_derivative: Box::new(map_expr(delay_derivative, f)),
                max_delay: max_delay.as_ref().map(|e| Box::new(map_expr(e, f))),
                derivative_order: *derivative_order,
            },
            IrExpr::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::Transition {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                delay: delay.as_ref().map(|e| Box::new(map_expr(e, f))),
                rise_time: rise_time.as_ref().map(|e| Box::new(map_expr(e, f))),
                fall_time: fall_time.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::TransitionDerivative {
                site: *site,
                input: Box::new(map_expr(input, f)),
                input_derivative: Box::new(map_expr(input_derivative, f)),
                delay: delay.as_ref().map(|e| Box::new(map_expr(e, f))),
                rise_time: rise_time.as_ref().map(|e| Box::new(map_expr(e, f))),
                fall_time: fall_time.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => IrExpr::Slew {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                max_pos_slew: max_pos_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
                max_neg_slew: max_neg_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => IrExpr::SlewDerivative {
                site: *site,
                input: Box::new(map_expr(input, f)),
                input_derivative: Box::new(map_expr(input_derivative, f)),
                max_pos_slew: max_pos_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
                max_pos_slew_derivative: max_pos_slew_derivative
                    .as_ref()
                    .map(|e| Box::new(map_expr(e, f))),
                max_neg_slew: max_neg_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
                max_neg_slew_derivative: max_neg_slew_derivative
                    .as_ref()
                    .map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Ddx { expr, axis } => IrExpr::Ddx {
                expr: Box::new(map_expr(expr, f)),
                axis: *axis,
            },
            IrExpr::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceND {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
            IrExpr::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceNDDerivative {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
            IrExpr::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZP {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },
            IrExpr::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZPDerivative {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },
            IrExpr::ZiFilter {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => IrExpr::ZiFilter {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: Box::new(map_expr(period, f)),
                transition: Box::new(map_expr(transition, f)),
                first_transition: Box::new(map_expr(first_transition, f)),
                direct_assignment: *direct_assignment,
            },
            IrExpr::ZiFilterDerivative {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => IrExpr::ZiFilterDerivative {
                site: *site,
                expr: Box::new(map_expr(expr, f)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: Box::new(map_expr(period, f)),
                transition: Box::new(map_expr(transition, f)),
                first_transition: Box::new(map_expr(first_transition, f)),
                direct_assignment: *direct_assignment,
            },
            IrExpr::VarIndexed {
                array,
                base,
                len,
                lower,
                index,
            } => IrExpr::VarIndexed {
                array: array.clone(),
                base: *base,
                len: *len,
                lower: *lower,
                index: Box::new(map_expr(index, f)),
            },
            other => other.clone(),
        }
    }

    pub(crate) fn assign_zi_site_ordinals(expr: &mut IrExpr, next: &mut u32) {
        *expr = map_expr(expr, &mut |node| match node {
            IrExpr::ZiFilter {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next.checked_add(1).expect("Zi site ordinal overflow");
                Some(IrExpr::ZiFilter {
                    site: assigned,
                    expr: expr.clone(),
                    numerator: numerator.clone(),
                    denominator: denominator.clone(),
                    period: period.clone(),
                    transition: transition.clone(),
                    first_transition: first_transition.clone(),
                    direct_assignment: *direct_assignment,
                })
            }
            _ => None,
        });
    }

    pub(crate) fn assign_zi_site_ordinals_in_items(items: &mut [IrAssignmentItem], next: &mut u32) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    assign_zi_site_ordinals(&mut assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    assign_zi_site_ordinals(condition, next);
                    assign_zi_site_ordinals_in_items(body, next);
                }
            }
        }
    }

    pub(crate) fn assign_laplace_site_ordinals(expr: &mut IrExpr, next: &mut u32) {
        *expr = map_expr(expr, &mut |node| match node {
            IrExpr::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next.checked_add(1).expect("Laplace site ordinal overflow");
                let mut inner = expr.as_ref().clone();
                assign_laplace_site_ordinals(&mut inner, next);
                Some(IrExpr::LaplaceND {
                    site: assigned,
                    expr: Box::new(inner),
                    numerator: numerator.clone(),
                    denominator: denominator.clone(),
                })
            }
            IrExpr::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next.checked_add(1).expect("Laplace site ordinal overflow");
                let mut inner = expr.as_ref().clone();
                assign_laplace_site_ordinals(&mut inner, next);
                Some(IrExpr::LaplaceZP {
                    site: assigned,
                    expr: Box::new(inner),
                    zeros: zeros.clone(),
                    poles: poles.clone(),
                    gain: *gain,
                })
            }
            _ => None,
        });
    }

    pub(crate) fn assign_laplace_site_ordinals_in_items(
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    assign_laplace_site_ordinals(&mut assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    assign_laplace_site_ordinals(condition, next);
                    assign_laplace_site_ordinals_in_items(body, next);
                }
            }
        }
    }

    pub(crate) fn assign_slew_site_ordinals(expr: &mut IrExpr, next: &mut u32) {
        *expr = map_expr(expr, &mut |node| match node {
            IrExpr::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next.checked_add(1).expect("slew site ordinal overflow");
                Some(IrExpr::Slew {
                    site: assigned,
                    expr: expr.clone(),
                    max_pos_slew: max_pos_slew.clone(),
                    max_neg_slew: max_neg_slew.clone(),
                })
            }
            _ => None,
        });
    }

    pub(crate) fn assign_slew_site_ordinals_in_items(
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    assign_slew_site_ordinals(&mut assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    assign_slew_site_ordinals(condition, next);
                    assign_slew_site_ordinals_in_items(body, next);
                }
            }
        }
    }

    pub(crate) fn assign_transition_site_ordinals(expr: &mut IrExpr, next: &mut u32) {
        *expr = map_expr(expr, &mut |node| match node {
            IrExpr::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next
                    .checked_add(1)
                    .expect("transition site ordinal overflow");
                Some(IrExpr::Transition {
                    site: assigned,
                    expr: expr.clone(),
                    delay: delay.clone(),
                    rise_time: rise_time.clone(),
                    fall_time: fall_time.clone(),
                })
            }
            _ => None,
        });
    }

    pub(crate) fn assign_transition_site_ordinals_in_items(
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    assign_transition_site_ordinals(&mut assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    assign_transition_site_ordinals(condition, next);
                    assign_transition_site_ordinals_in_items(body, next);
                }
            }
        }
    }

    pub(crate) fn assign_absdelay_site_ordinals(expr: &mut IrExpr, next: &mut u32) {
        *expr = map_expr(expr, &mut |node| match node {
            IrExpr::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => {
                let mut assigned = *site;
                assigned.ordinal = *next;
                *next = next.checked_add(1).expect("absdelay site ordinal overflow");
                Some(IrExpr::AbsDelay {
                    site: assigned,
                    expr: expr.clone(),
                    delay_time: delay_time.clone(),
                    max_delay: max_delay.clone(),
                })
            }
            _ => None,
        });
    }

    pub(crate) fn assign_absdelay_site_ordinals_in_items(
        items: &mut [IrAssignmentItem],
        next: &mut u32,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assignment) => {
                    assign_absdelay_site_ordinals(&mut assignment.expr, next);
                }
                IrAssignmentItem::Loop { condition, body } => {
                    assign_absdelay_site_ordinals(condition, next);
                    assign_absdelay_site_ordinals_in_items(body, next);
                }
            }
        }
    }

    /// Differentiate an expression with respect to a variable
    /// (without assignment-chain shadows; prefer
    /// [`differentiate_with_shadows`] when a chain context exists)
    pub fn differentiate(expr: &IrExpr, wrt: &DerivativeWrt) -> IrExpr {
        differentiate_with_shadows(expr, wrt, &ShadowContext::default())
    }

    /// Differentiate an expression, chaining through shadowed variables
    pub fn differentiate_with_shadows(
        expr: &IrExpr,
        wrt: &DerivativeWrt,
        shadows: &ShadowContext,
    ) -> IrExpr {
        let differentiate = |e: &IrExpr| differentiate_with_shadows(e, wrt, shadows);
        match expr {
            IrExpr::Const(_) => IrExpr::Const(0.0),

            IrExpr::Voltage(p, n) => {
                if let DerivativeWrt::Voltage(v) = wrt {
                    if *v == *p {
                        IrExpr::Const(1.0)
                    } else if *v == *n {
                        IrExpr::Const(-1.0)
                    } else {
                        IrExpr::Const(0.0)
                    }
                } else {
                    IrExpr::Const(0.0)
                }
            }

            // Chain rule through intermediate variables: the shadow
            // variable carries the derivative along the active axis. A
            // variable that cannot vary along this axis differentiates to
            // zero without a shadow slot ever existing.
            IrExpr::Var(name) => {
                if shadows.is_shadowed_on(name, wrt) {
                    IrExpr::Var(ShadowContext::shadow_name(name, wrt))
                } else {
                    IrExpr::Const(0.0)
                }
            }

            // Runtime-indexed reads chain through the array's shadow run
            // at the same element; the index itself only selects
            IrExpr::VarIndexed {
                array,
                base: _,
                len,
                lower,
                index,
            } => match shadows.array_shadow_base(array, wrt) {
                Some(shadow_base) => IrExpr::VarIndexed {
                    array: ShadowContext::shadow_name(array, wrt),
                    base: shadow_base,
                    len: *len,
                    lower: *lower,
                    index: index.clone(),
                },
                None => IrExpr::Const(0.0),
            },

            // Branch-current unknowns differentiate to 1 along their own
            // axis and 0 along every other
            IrExpr::BranchCurrent(k) => match wrt {
                DerivativeWrt::BranchCurrent(j) if j == k => IrExpr::Const(1.0),
                _ => IrExpr::Const(0.0),
            },

            IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Time
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_) => IrExpr::Const(0.0),

            IrExpr::Binary(op, left, right) => {
                let dl = differentiate(left);
                let dr = differentiate(right);

                match op {
                    BinaryOp::Add => IrExpr::Binary(BinaryOp::Add, Box::new(dl), Box::new(dr)),
                    BinaryOp::Sub => IrExpr::Binary(BinaryOp::Sub, Box::new(dl), Box::new(dr)),
                    BinaryOp::Mul => {
                        // Product rule: d(f*g) = f'*g + f*g'
                        IrExpr::Binary(
                            BinaryOp::Add,
                            Box::new(IrExpr::Binary(BinaryOp::Mul, Box::new(dl), right.clone())),
                            Box::new(IrExpr::Binary(BinaryOp::Mul, left.clone(), Box::new(dr))),
                        )
                    }
                    BinaryOp::Div => {
                        // Quotient rule: d(f/g) = (f'*g - f*g') / g^2
                        let num = IrExpr::Binary(
                            BinaryOp::Sub,
                            Box::new(IrExpr::Binary(BinaryOp::Mul, Box::new(dl), right.clone())),
                            Box::new(IrExpr::Binary(BinaryOp::Mul, left.clone(), Box::new(dr))),
                        );
                        let den = IrExpr::Binary(BinaryOp::Mul, right.clone(), right.clone());
                        IrExpr::Binary(BinaryOp::Div, Box::new(num), Box::new(den))
                    }
                    BinaryOp::Pow => {
                        // d(u^v) =
                        //   if v is const c: c*u^(c-1)*u'
                        //   else: u^v * (v' * ln(u) + v * u'/u)
                        match right.as_ref() {
                            IrExpr::Const(c) => {
                                let u_pow = IrExpr::Binary(
                                    BinaryOp::Pow,
                                    left.clone(),
                                    Box::new(IrExpr::Const(*c - 1.0)),
                                );
                                IrExpr::Binary(
                                    BinaryOp::Mul,
                                    Box::new(IrExpr::Const(*c)),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Mul,
                                        Box::new(u_pow),
                                        Box::new(dl),
                                    )),
                                )
                            }
                            _ => {
                                let u_pow_v =
                                    IrExpr::Binary(BinaryOp::Pow, left.clone(), right.clone());
                                let vprime_ln_u = IrExpr::Binary(
                                    BinaryOp::Mul,
                                    Box::new(dr),
                                    Box::new(IrExpr::Call(
                                        IrFunction::Log,
                                        vec![left.as_ref().clone()],
                                    )),
                                );
                                let v_uprime_over_u = IrExpr::Binary(
                                    BinaryOp::Mul,
                                    right.clone(),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Div,
                                        Box::new(dl),
                                        left.clone(),
                                    )),
                                );
                                let term = IrExpr::Binary(
                                    BinaryOp::Add,
                                    Box::new(vprime_ln_u),
                                    Box::new(v_uprime_over_u),
                                );
                                IrExpr::Binary(BinaryOp::Mul, Box::new(u_pow_v), Box::new(term))
                            }
                        }
                    }
                    // Piecewise-constant or discontinuous operators are treated
                    // as zero derivative in the DC Jacobian.
                    BinaryOp::Mod
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => IrExpr::Const(0.0),
                }
            }

            IrExpr::Unary(UnaryOp::Neg, inner) => {
                IrExpr::Unary(UnaryOp::Neg, Box::new(differentiate(inner)))
            }
            // Unary plus is the identity
            IrExpr::Unary(UnaryOp::Pos, inner) => differentiate(inner),
            // Logical/bitwise negation is piecewise constant
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => IrExpr::Const(0.0),

            // d(c ? a : b) = c ? da : db
            IrExpr::Conditional(cond, then_expr, else_expr) => IrExpr::Conditional(
                cond.clone(),
                Box::new(differentiate(then_expr)),
                Box::new(differentiate(else_expr)),
            ),

            IrExpr::Call(func, args) if args.len() == 1 => {
                let inner = &args[0];
                let di = differentiate(inner);

                // Chain rule: d(f(g)) = f'(g) * g'
                let outer_deriv = match func {
                    IrFunction::Abs => IrExpr::Conditional(
                        Box::new(IrExpr::Binary(
                            BinaryOp::Ge,
                            Box::new(inner.clone()),
                            Box::new(IrExpr::Const(0.0)),
                        )),
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Const(-1.0)),
                    ),
                    IrFunction::Exp => IrExpr::Call(IrFunction::Exp, vec![inner.clone()]),
                    IrFunction::LimitedExp => limited_exp_derivative_scale(inner.clone()),
                    IrFunction::Log => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(inner.clone()),
                    ),
                    IrFunction::Log10 => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Mul,
                            Box::new(inner.clone()),
                            Box::new(IrExpr::Const(std::f64::consts::LN_10)),
                        )),
                    ),
                    IrFunction::Sqrt => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(0.5)),
                        Box::new(IrExpr::Call(IrFunction::Sqrt, vec![inner.clone()])),
                    ),
                    IrFunction::Sin => IrExpr::Call(IrFunction::Cos, vec![inner.clone()]),
                    IrFunction::Cos => IrExpr::Unary(
                        UnaryOp::Neg,
                        Box::new(IrExpr::Call(IrFunction::Sin, vec![inner.clone()])),
                    ),
                    IrFunction::Tan => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Pow,
                            Box::new(IrExpr::Call(IrFunction::Cos, vec![inner.clone()])),
                            Box::new(IrExpr::Const(2.0)),
                        )),
                    ),
                    IrFunction::Sinh => IrExpr::Call(IrFunction::Cosh, vec![inner.clone()]),
                    IrFunction::Cosh => IrExpr::Call(IrFunction::Sinh, vec![inner.clone()]),
                    IrFunction::Tanh => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Pow,
                            Box::new(IrExpr::Call(IrFunction::Cosh, vec![inner.clone()])),
                            Box::new(IrExpr::Const(2.0)),
                        )),
                    ),
                    IrFunction::Asin => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Call(
                            IrFunction::Sqrt,
                            vec![IrExpr::Binary(
                                BinaryOp::Sub,
                                Box::new(IrExpr::Const(1.0)),
                                Box::new(IrExpr::Binary(
                                    BinaryOp::Pow,
                                    Box::new(inner.clone()),
                                    Box::new(IrExpr::Const(2.0)),
                                )),
                            )],
                        )),
                    ),
                    IrFunction::Acos => IrExpr::Unary(
                        UnaryOp::Neg,
                        Box::new(IrExpr::Binary(
                            BinaryOp::Div,
                            Box::new(IrExpr::Const(1.0)),
                            Box::new(IrExpr::Call(
                                IrFunction::Sqrt,
                                vec![IrExpr::Binary(
                                    BinaryOp::Sub,
                                    Box::new(IrExpr::Const(1.0)),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Pow,
                                        Box::new(inner.clone()),
                                        Box::new(IrExpr::Const(2.0)),
                                    )),
                                )],
                            )),
                        )),
                    ),
                    IrFunction::Atan => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Add,
                            Box::new(IrExpr::Const(1.0)),
                            Box::new(IrExpr::Binary(
                                BinaryOp::Pow,
                                Box::new(inner.clone()),
                                Box::new(IrExpr::Const(2.0)),
                            )),
                        )),
                    ),
                    IrFunction::Asinh => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Call(
                            IrFunction::Sqrt,
                            vec![IrExpr::Binary(
                                BinaryOp::Add,
                                Box::new(IrExpr::Const(1.0)),
                                Box::new(IrExpr::Binary(
                                    BinaryOp::Pow,
                                    Box::new(inner.clone()),
                                    Box::new(IrExpr::Const(2.0)),
                                )),
                            )],
                        )),
                    ),
                    IrFunction::Acosh => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Mul,
                            Box::new(IrExpr::Call(
                                IrFunction::Sqrt,
                                vec![IrExpr::Binary(
                                    BinaryOp::Sub,
                                    Box::new(inner.clone()),
                                    Box::new(IrExpr::Const(1.0)),
                                )],
                            )),
                            Box::new(IrExpr::Call(
                                IrFunction::Sqrt,
                                vec![IrExpr::Binary(
                                    BinaryOp::Add,
                                    Box::new(inner.clone()),
                                    Box::new(IrExpr::Const(1.0)),
                                )],
                            )),
                        )),
                    ),
                    IrFunction::Atanh => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Sub,
                            Box::new(IrExpr::Const(1.0)),
                            Box::new(IrExpr::Binary(
                                BinaryOp::Pow,
                                Box::new(inner.clone()),
                                Box::new(IrExpr::Const(2.0)),
                            )),
                        )),
                    ),
                    IrFunction::Floor | IrFunction::Ceil => IrExpr::Const(0.0),
                    _ => return IrExpr::Const(0.0),
                };

                IrExpr::Binary(BinaryOp::Mul, Box::new(outer_deriv), Box::new(di))
            }
            IrExpr::Call(IrFunction::Atan2, args) if args.len() == 2 => {
                // atan2(y, x): d = (x*dy - y*dx)/(x^2 + y^2)
                let y = args[0].clone();
                let x = args[1].clone();
                let dy = differentiate(&y);
                let dx = differentiate(&x);
                let num = IrExpr::Binary(
                    BinaryOp::Sub,
                    Box::new(IrExpr::Binary(
                        BinaryOp::Mul,
                        Box::new(x.clone()),
                        Box::new(dy),
                    )),
                    Box::new(IrExpr::Binary(
                        BinaryOp::Mul,
                        Box::new(y.clone()),
                        Box::new(dx),
                    )),
                );
                let den = IrExpr::Binary(
                    BinaryOp::Add,
                    Box::new(IrExpr::Binary(
                        BinaryOp::Pow,
                        Box::new(x),
                        Box::new(IrExpr::Const(2.0)),
                    )),
                    Box::new(IrExpr::Binary(
                        BinaryOp::Pow,
                        Box::new(y),
                        Box::new(IrExpr::Const(2.0)),
                    )),
                );
                IrExpr::Binary(BinaryOp::Div, Box::new(num), Box::new(den))
            }
            IrExpr::Call(IrFunction::Pow, args) if args.len() == 2 => {
                let as_binary = IrExpr::Binary(
                    BinaryOp::Pow,
                    Box::new(args[0].clone()),
                    Box::new(args[1].clone()),
                );
                differentiate(&as_binary)
            }
            IrExpr::Call(IrFunction::Min, args) if args.len() == 2 => {
                let left = args[0].clone();
                let right = args[1].clone();
                IrExpr::Conditional(
                    Box::new(IrExpr::Binary(
                        BinaryOp::Le,
                        Box::new(left.clone()),
                        Box::new(right.clone()),
                    )),
                    Box::new(differentiate(&left)),
                    Box::new(differentiate(&right)),
                )
            }
            IrExpr::Call(IrFunction::Max, args) if args.len() == 2 => {
                let left = args[0].clone();
                let right = args[1].clone();
                IrExpr::Conditional(
                    Box::new(IrExpr::Binary(
                        BinaryOp::Ge,
                        Box::new(left.clone()),
                        Box::new(right.clone()),
                    )),
                    Box::new(differentiate(&left)),
                    Box::new(differentiate(&right)),
                )
            }

            IrExpr::Limexp(inner) => {
                // d(limexp(x)) = limexp(x) * x' (same as exp, but clamped)
                let di = differentiate(inner);
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Limexp(inner.clone())),
                    Box::new(di),
                )
            }

            // ddt companion: d(ddt(q))/dV = (dq/dV) / dt under backward
            // Euler (zero at DC). The DdtCompanion wrapper multiplies its
            // operand by the integration coefficient at runtime.
            IrExpr::Ddt(inner) => IrExpr::DdtCompanion(Box::new(differentiate(inner))),

            // idt companion: d(idt(x))/dV = dt * dx/dV (zero at DC)
            IrExpr::Idt(inner, _) => IrExpr::IdtCompanion(Box::new(differentiate(inner))),

            // idtmod: the wrap is the identity almost everywhere, so the
            // small-signal derivative matches idt
            IrExpr::IdtMod { expr, .. } => IrExpr::IdtCompanion(Box::new(differentiate(expr))),

            // $limit passes its value through at convergence
            IrExpr::Limit(inner, _) | IrExpr::CanonicalLimit(inner) => differentiate(inner),

            // Table lookup: slope of the active segment times the inner
            // derivative
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => {
                let slope = IrExpr::TableDerivative {
                    input: input.clone(),
                    x_data: x_data.clone(),
                    y_data: y_data.clone(),
                };
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(slope),
                    Box::new(differentiate(input)),
                )
            }

            // Transport delay passes the DC small-signal through. Transition
            // instead needs the exact accepted-state-dependent transient
            // coefficient: zero on delayed/history-driven ramps and one only
            // on an instantaneous direct candidate. Keep the primal operands
            // and site correlated so the runtime can compute that coefficient
            // read-only even if the derivative executes before the primal.
            IrExpr::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => IrExpr::AbsDelayDerivative {
                site: *site,
                input: expr.clone(),
                input_derivative: Box::new(differentiate(expr)),
                delay_time: delay_time.clone(),
                delay_derivative: Box::new(differentiate(delay_time)),
                max_delay: max_delay.clone(),
                derivative_order: 1,
            },
            IrExpr::AbsDelayDerivative {
                site,
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                derivative_order,
            } => IrExpr::AbsDelayDerivative {
                site: *site,
                input: input.clone(),
                input_derivative: Box::new(differentiate(input_derivative)),
                delay_time: delay_time.clone(),
                delay_derivative: Box::new(differentiate(delay_derivative)),
                max_delay: max_delay.clone(),
                derivative_order: derivative_order.saturating_add(1),
            },
            IrExpr::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::TransitionDerivative {
                site: *site,
                input: expr.clone(),
                input_derivative: Box::new(differentiate(expr)),
                delay: delay.clone(),
                rise_time: rise_time.clone(),
                fall_time: fall_time.clone(),
            },
            IrExpr::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::TransitionDerivative {
                site: *site,
                input: input.clone(),
                input_derivative: Box::new(differentiate(input_derivative)),
                delay: delay.clone(),
                rise_time: rise_time.clone(),
                fall_time: fall_time.clone(),
            },

            // `slew` has a branch-exact transient derivative: the first
            // argument tracks directly when unsaturated, while a saturated
            // candidate depends on the active rate operand and elapsed time.
            IrExpr::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => IrExpr::SlewDerivative {
                site: *site,
                input: expr.clone(),
                input_derivative: Box::new(differentiate(expr)),
                max_pos_slew: max_pos_slew.clone(),
                max_pos_slew_derivative: max_pos_slew
                    .as_deref()
                    .map(|rate| Box::new(differentiate(rate))),
                max_neg_slew: max_neg_slew.clone(),
                max_neg_slew_derivative: max_neg_slew
                    .as_deref()
                    .map(|rate| Box::new(differentiate(rate))),
            },
            // The same read-only branch action also represents higher fixed-
            // branch derivatives. Preserve the primal branch operands and
            // differentiate only the derivative payloads. This avoids the
            // incorrect assumption that a derivative of a slew Jacobian is
            // always zero when a dynamic rate is nonlinear.
            IrExpr::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => IrExpr::SlewDerivative {
                site: *site,
                input: input.clone(),
                input_derivative: Box::new(differentiate(input_derivative)),
                max_pos_slew: max_pos_slew.clone(),
                max_pos_slew_derivative: max_pos_slew_derivative
                    .as_deref()
                    .map(|derivative| Box::new(differentiate(derivative))),
                max_neg_slew: max_neg_slew.clone(),
                max_neg_slew_derivative: max_neg_slew_derivative
                    .as_deref()
                    .map(|derivative| Box::new(differentiate(derivative))),
            },

            // Sampled-data filters have a time-dependent exact Jacobian:
            // H(1) in equilibrium, b0/a0 on an edge, and zero while holding.
            IrExpr::ZiFilter {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => IrExpr::ZiFilterDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: period.clone(),
                transition: transition.clone(),
                first_transition: first_transition.clone(),
                direct_assignment: *direct_assignment,
            },
            // Differentiation is only run once per Jacobian axis in normal
            // construction. Retain the schedule action if a transformed IR
            // is differentiated again.
            IrExpr::ZiFilterDerivative {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => IrExpr::ZiFilterDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: period.clone(),
                transition: transition.clone(),
                first_transition: first_transition.clone(),
                direct_assignment: *direct_assignment,
            },

            // Laplace derivatives retain the primal site's state action.
            // Runtime selects DC gain or the active companion-rule input gain.
            IrExpr::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceNDDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
            IrExpr::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZPDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },
            IrExpr::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceNDDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
            IrExpr::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZPDerivative {
                site: *site,
                expr: Box::new(differentiate(expr)),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },

            // Unresolved ddx: expand, then differentiate the expansion
            IrExpr::Ddx { .. } => {
                let resolved = resolve_ddx(expr, shadows);
                differentiate(&resolved)
            }

            // A syntactic noise call is the unit realization of exactly one
            // independent process.  Its PSD operands are metadata, not part
            // of the realization gain.
            IrExpr::WhiteNoise { site, .. }
            | IrExpr::FlickerNoise { site, .. }
            | IrExpr::NoiseTable { site, .. } => match wrt {
                DerivativeWrt::Noise(process) if *process == site.ordinal as usize => {
                    IrExpr::Const(1.0)
                }
                _ => IrExpr::Const(0.0),
            },

            // Event detectors, noise sources, analysis queries, and current
            // probes are treated as constants in the DC Jacobian
            _ => IrExpr::Const(0.0),
        }
    }

    fn limited_exp_derivative_scale(inner: IrExpr) -> IrExpr {
        const LIMIT: f64 = 80.0;
        let high = IrExpr::Binary(
            BinaryOp::Gt,
            Box::new(inner.clone()),
            Box::new(IrExpr::Const(LIMIT)),
        );
        let low = IrExpr::Binary(
            BinaryOp::Lt,
            Box::new(inner.clone()),
            Box::new(IrExpr::Const(-LIMIT)),
        );

        IrExpr::Conditional(
            Box::new(high),
            Box::new(IrExpr::Const(LIMIT.exp())),
            Box::new(IrExpr::Conditional(
                Box::new(low),
                Box::new(IrExpr::Const(0.0)),
                Box::new(IrExpr::Call(IrFunction::Exp, vec![inner])),
            )),
        )
    }

    /// Simplify an IR expression (constant folding, identity removal)
    pub fn simplify(expr: IrExpr) -> IrExpr {
        match expr {
            IrExpr::Binary(op, left, right) => {
                let left = simplify(*left);
                let right = simplify(*right);

                // Constant folding
                if let (IrExpr::Const(l), IrExpr::Const(r)) = (&left, &right) {
                    return IrExpr::Const(match op {
                        BinaryOp::Add => l + r,
                        BinaryOp::Sub => l - r,
                        BinaryOp::Mul => l * r,
                        BinaryOp::Div => l / r,
                        BinaryOp::Pow => l.powf(*r),
                        _ => return IrExpr::Binary(op, Box::new(left), Box::new(right)),
                    });
                }

                // Identity rules
                match op {
                    BinaryOp::Add => {
                        if let IrExpr::Const(0.0) = left {
                            return right;
                        }
                        if let IrExpr::Const(0.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Sub => {
                        if let IrExpr::Const(0.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Mul => {
                        if let IrExpr::Const(0.0) = left {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(0.0) = right {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(1.0) = left {
                            return right;
                        }
                        if let IrExpr::Const(1.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Div => {
                        if let IrExpr::Const(0.0) = left {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(1.0) = right {
                            return left;
                        }
                    }
                    _ => {}
                }

                IrExpr::Binary(op, Box::new(left), Box::new(right))
            }
            IrExpr::Unary(op, inner) => {
                let inner = simplify(*inner);
                if let (UnaryOp::Neg, IrExpr::Const(v)) = (op, &inner) {
                    return IrExpr::Const(-v);
                }
                if let UnaryOp::Pos = op {
                    return inner;
                }
                IrExpr::Unary(op, Box::new(inner))
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                let cond = simplify(*cond);
                let then_expr = simplify(*then_expr);
                let else_expr = simplify(*else_expr);
                if let IrExpr::Const(c) = cond {
                    return if c != 0.0 { then_expr } else { else_expr };
                }
                IrExpr::Conditional(Box::new(cond), Box::new(then_expr), Box::new(else_expr))
            }
            IrExpr::Call(func, args) => {
                IrExpr::Call(func, args.into_iter().map(simplify).collect())
            }
            // Companion factors of a zero derivative vanish
            IrExpr::DdtCompanion(inner) => {
                let inner = simplify(*inner);
                if matches!(inner, IrExpr::Const(v) if v == 0.0) {
                    return IrExpr::Const(0.0);
                }
                IrExpr::DdtCompanion(Box::new(inner))
            }
            IrExpr::IdtCompanion(inner) => {
                let inner = simplify(*inner);
                if matches!(inner, IrExpr::Const(v) if v == 0.0) {
                    return IrExpr::Const(0.0);
                }
                IrExpr::IdtCompanion(Box::new(inner))
            }
            other => other,
        }
    }

    #[cfg(test)]
    mod visit_expr_parity_tests {
        use super::*;

        fn marker(index: usize) -> Box<IrExpr> {
            Box::new(IrExpr::Var(SmolStr::new(format!("m{index}"))))
        }

        fn opt(index: usize) -> Option<Box<IrExpr>> {
            Some(marker(index))
        }

        /// Names reached through [`map_expr`] — the traversal
        /// [`visit_expr`] has to reproduce.
        fn names_via_map(expr: &IrExpr) -> Vec<SmolStr> {
            let mut names = Vec::new();
            map_expr(expr, &mut |node| {
                if let IrExpr::Var(name) = node {
                    names.push(name.clone());
                }
                None
            });
            names
        }

        fn names_via_visit(expr: &IrExpr) -> Vec<SmolStr> {
            let mut names = Vec::new();
            visit_expr(expr, &mut |node| {
                if let IrExpr::Var(name) = node {
                    names.push(name.clone());
                }
            });
            names
        }

        /// One value per [`IrExpr`] variant, with a distinctly named marker
        /// in every operand slot the variant has — including the slots
        /// `map_expr` deliberately does not descend into, so a visitor that
        /// descended too far would be caught as surely as one that stopped
        /// short.
        fn one_of_every_variant() -> Vec<IrExpr> {
            let site = ZiSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            let laplace = LaplaceSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            let slew = SlewSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            let transition = TransitionSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            let absdelay = AbsDelaySiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            let noise_site = NoiseSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            };
            vec![
                IrExpr::Const(1.0),
                IrExpr::Param(SmolStr::new("p")),
                IrExpr::ParamGiven(SmolStr::new("p")),
                IrExpr::Var(SmolStr::new("m0")),
                IrExpr::VarIndexed {
                    array: SmolStr::new("a"),
                    base: 0,
                    len: 2,
                    lower: 0,
                    index: marker(1),
                },
                IrExpr::Voltage(0, 1),
                IrExpr::Current(0, 1),
                IrExpr::BranchCurrent(0),
                IrExpr::Time,
                IrExpr::Temperature,
                IrExpr::Vt,
                IrExpr::Mfactor,
                IrExpr::PortConnected(0),
                IrExpr::Binary(BinaryOp::Add, marker(2), marker(3)),
                IrExpr::Unary(UnaryOp::Neg, marker(4)),
                IrExpr::Call(IrFunction::Sqrt, vec![*marker(5), *marker(6)]),
                IrExpr::Ddt(marker(7)),
                IrExpr::Idt(marker(8), opt(9)),
                IrExpr::IdtMod {
                    expr: marker(10),
                    ic: opt(11),
                    modulus: marker(12),
                    offset: opt(13),
                },
                IrExpr::Limexp(marker(14)),
                IrExpr::Limit(marker(15), opt(16)),
                IrExpr::CanonicalLimit(marker(17)),
                IrExpr::TableLookup {
                    input: marker(18),
                    x_data: vec![0.0],
                    y_data: vec![0.0],
                },
                IrExpr::AbsDelay {
                    site: absdelay,
                    expr: marker(19),
                    delay_time: marker(20),
                    max_delay: opt(21),
                },
                IrExpr::AbsDelayDerivative {
                    site: absdelay,
                    input: marker(22),
                    input_derivative: marker(23),
                    delay_time: marker(24),
                    delay_derivative: marker(25),
                    max_delay: opt(26),
                    derivative_order: 1,
                },
                IrExpr::Transition {
                    site: transition,
                    expr: marker(27),
                    delay: opt(28),
                    rise_time: opt(29),
                    fall_time: opt(30),
                },
                IrExpr::TransitionDerivative {
                    site: transition,
                    input: marker(31),
                    input_derivative: marker(32),
                    delay: opt(33),
                    rise_time: opt(34),
                    fall_time: opt(35),
                },
                IrExpr::Slew {
                    site: slew,
                    expr: marker(36),
                    max_pos_slew: opt(37),
                    max_neg_slew: opt(38),
                },
                IrExpr::SlewDerivative {
                    site: slew,
                    input: marker(39),
                    input_derivative: marker(40),
                    max_pos_slew: opt(41),
                    max_pos_slew_derivative: opt(42),
                    max_neg_slew: opt(43),
                    max_neg_slew_derivative: opt(44),
                },
                IrExpr::Cross {
                    expr: marker(45),
                    direction: opt(46),
                    time_tol: opt(47),
                    expr_tol: opt(48),
                    enable: opt(49),
                },
                IrExpr::LastCrossing {
                    expr: marker(50),
                    direction: Some(1),
                },
                IrExpr::WhiteNoise {
                    site: noise_site,
                    power: marker(51),
                    name: None,
                },
                IrExpr::FlickerNoise {
                    site: noise_site,
                    power: marker(52),
                    exponent: marker(53),
                    name: None,
                },
                IrExpr::NoiseTable {
                    site: noise_site,
                    points: vec![(1.0, 1.0)],
                    log_interp: false,
                    name: None,
                },
                IrExpr::Analysis("dc".to_string()),
                IrExpr::Above {
                    expr: marker(54),
                    time_tol: opt(55),
                    expr_tol: opt(56),
                    enable: opt(57),
                },
                IrExpr::Timer {
                    start_time: marker(58),
                    period: opt(59),
                    time_tol: opt(60),
                    enable: opt(61),
                },
                IrExpr::LaplaceZP {
                    site: laplace,
                    expr: marker(62),
                    zeros: Vec::new(),
                    poles: Vec::new(),
                    gain: 1.0,
                },
                IrExpr::LaplaceND {
                    site: laplace,
                    expr: marker(63),
                    numerator: vec![1.0],
                    denominator: vec![1.0],
                },
                IrExpr::LaplaceNDDerivative {
                    site: laplace,
                    expr: marker(64),
                    numerator: vec![1.0],
                    denominator: vec![1.0],
                },
                IrExpr::LaplaceZPDerivative {
                    site: laplace,
                    expr: marker(65),
                    zeros: Vec::new(),
                    poles: Vec::new(),
                    gain: 1.0,
                },
                IrExpr::ZiFilter {
                    site,
                    expr: marker(66),
                    numerator: ZiPolynomialDefinition::Coefficients(vec![*marker(67)]),
                    denominator: ZiPolynomialDefinition::Roots(vec![(*marker(68), *marker(69))]),
                    period: marker(70),
                    transition: marker(71),
                    first_transition: marker(72),
                    direct_assignment: false,
                },
                IrExpr::ZiFilterDerivative {
                    site,
                    expr: marker(73),
                    numerator: ZiPolynomialDefinition::Coefficients(vec![*marker(74)]),
                    denominator: ZiPolynomialDefinition::Roots(vec![(*marker(75), *marker(76))]),
                    period: marker(77),
                    transition: marker(78),
                    first_transition: marker(79),
                    direct_assignment: false,
                },
                IrExpr::Ddx {
                    expr: marker(80),
                    axis: DdxAxis::Potential {
                        pos: Some(0),
                        neg: None,
                    },
                },
                IrExpr::DdtCompanion(marker(81)),
                IrExpr::IdtCompanion(marker(82)),
                IrExpr::TableDerivative {
                    input: marker(83),
                    x_data: vec![0.0],
                    y_data: vec![0.0],
                },
                IrExpr::Conditional(marker(84), marker(85), marker(86)),
            ]
        }

        #[test]
        fn visit_expr_walks_the_same_child_slots_as_map_expr() {
            for expr in one_of_every_variant() {
                assert_eq!(
                    names_via_visit(&expr),
                    names_via_map(&expr),
                    "visit_expr and map_expr disagree about {expr:?}"
                );
            }
        }

        #[test]
        fn contains_ddx_finds_an_operator_under_every_descended_slot() {
            let ddx = IrExpr::Ddx {
                expr: Box::new(IrExpr::Var(SmolStr::new("v"))),
                axis: DdxAxis::BranchCurrent {
                    ordinal: 0,
                    reversed: false,
                },
            };
            assert!(contains_ddx(&ddx));
            assert!(contains_ddx(&IrExpr::Binary(
                BinaryOp::Mul,
                Box::new(IrExpr::Const(2.0)),
                Box::new(IrExpr::Conditional(
                    Box::new(IrExpr::Const(1.0)),
                    Box::new(ddx.clone()),
                    Box::new(IrExpr::Const(0.0)),
                )),
            )));
            assert!(!contains_ddx(&IrExpr::Binary(
                BinaryOp::Mul,
                Box::new(IrExpr::Const(2.0)),
                Box::new(IrExpr::Var(SmolStr::new("v"))),
            )));
            // A slot `map_expr` does not descend into is a slot
            // `resolve_ddx` cannot rewrite, so reporting it would promise a
            // resolution that never happens.
            assert!(!contains_ddx(&IrExpr::WhiteNoise {
                site: NoiseSiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                power: Box::new(ddx),
                name: None,
            }));
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn noise(process: u32) -> IrExpr {
            IrExpr::WhiteNoise {
                site: NoiseSiteId {
                    source: 0,
                    start: process,
                    end: process + 1,
                    ordinal: process,
                },
                power: Box::new(IrExpr::Const(1.0)),
                name: None,
            }
        }

        fn add(left: IrExpr, right: IrExpr) -> IrExpr {
            IrExpr::Binary(BinaryOp::Add, Box::new(left), Box::new(right))
        }

        fn mul(left: IrExpr, right: IrExpr) -> IrExpr {
            IrExpr::Binary(BinaryOp::Mul, Box::new(left), Box::new(right))
        }

        fn old_ad_axes(
            expr: &IrExpr,
            deps: &HashMap<SmolStr, BTreeSet<usize>>,
            num_processes: usize,
        ) -> BTreeSet<usize> {
            let shadows = ShadowContext {
                noise_shadowed: deps.clone(),
                ..ShadowContext::default()
            };
            (0..num_processes)
                .filter(|process| {
                    let derivative = simplify(differentiate_with_shadows(
                        expr,
                        &DerivativeWrt::Noise(*process),
                        &shadows,
                    ));
                    !matches!(derivative, IrExpr::Const(value) if value == 0.0)
                })
                .collect()
        }

        #[test]
        fn structural_noise_axes_match_symbolic_ad_for_value_expressions() {
            let deps = HashMap::from([
                (SmolStr::new("assigned"), BTreeSet::from([2usize])),
                (SmolStr::new("samples"), BTreeSet::from([3usize])),
            ]);
            let corpus = [
                (
                    "noise metadata is not a realization operand",
                    IrExpr::WhiteNoise {
                        site: NoiseSiteId {
                            source: 0,
                            start: 0,
                            end: 1,
                            ordinal: 0,
                        },
                        power: Box::new(noise(1)),
                        name: None,
                    },
                ),
                (
                    "assigned variable and direct process",
                    add(IrExpr::Var("assigned".into()), noise(0)),
                ),
                (
                    "zero multiplier removes derivative",
                    mul(IrExpr::Const(0.0), noise(0)),
                ),
                (
                    "zero power removes derivative",
                    IrExpr::Binary(
                        BinaryOp::Pow,
                        Box::new(noise(0)),
                        Box::new(IrExpr::Const(0.0)),
                    ),
                ),
                (
                    "discrete operator has zero derivative",
                    IrExpr::Binary(BinaryOp::Gt, Box::new(noise(0)), Box::new(noise(1))),
                ),
                (
                    "constant conditional selects one derivative branch",
                    IrExpr::Conditional(
                        Box::new(IrExpr::Const(0.0)),
                        Box::new(noise(0)),
                        Box::new(noise(1)),
                    ),
                ),
                (
                    "continuous function follows its argument",
                    IrExpr::Call(IrFunction::Exp, vec![noise(0)]),
                ),
                (
                    "pow function follows binary-pow zero-exponent semantics",
                    IrExpr::Call(IrFunction::Pow, vec![noise(0), IrExpr::Const(0.0)]),
                ),
                (
                    "atan2 removes a derivative multiplied by zero",
                    IrExpr::Call(IrFunction::Atan2, vec![noise(0), IrExpr::Const(0.0)]),
                ),
                (
                    "piecewise-constant function has zero derivative",
                    IrExpr::Call(IrFunction::Floor, vec![noise(0)]),
                ),
                (
                    "ddx is resolved before noise provenance",
                    IrExpr::Ddx {
                        expr: Box::new(mul(noise(1), IrExpr::Voltage(0, usize::MAX))),
                        axis: DdxAxis::Potential {
                            pos: Some(0),
                            neg: None,
                        },
                    },
                ),
                (
                    "ddx can eliminate a noise-only value",
                    IrExpr::Ddx {
                        expr: Box::new(noise(1)),
                        axis: DdxAxis::Potential {
                            pos: Some(0),
                            neg: None,
                        },
                    },
                ),
            ];

            for (description, expr) in corpus {
                assert_eq!(
                    expression_noise_axes(&expr, &deps, 4),
                    old_ad_axes(&expr, &deps, 4),
                    "{description}"
                );
            }

            let singular_division = IrExpr::Binary(
                BinaryOp::Div,
                Box::new(noise(0)),
                Box::new(IrExpr::Const(0.0)),
            );
            assert_eq!(
                expression_noise_axes(&singular_division, &deps, 1),
                old_ad_axes(&singular_division, &deps, 1),
                "a zero denominator must not prune numerator noise before runtime validation"
            );
        }

        #[test]
        fn structural_noise_axes_follow_only_stateful_derivative_operands() {
            let deps = HashMap::from([(SmolStr::new("samples"), BTreeSet::from([3usize]))]);
            let indexed = IrExpr::VarIndexed {
                array: "samples".into(),
                base: 0,
                len: 4,
                lower: 0,
                index: Box::new(noise(1)),
            };
            assert_eq!(
                expression_noise_axes(&indexed, &deps, 4),
                BTreeSet::from([3]),
                "runtime array reads follow the selected value, not the index"
            );

            let conditional = IrExpr::Conditional(
                Box::new(noise(0)),
                Box::new(noise(1)),
                Box::new(IrExpr::Var("samples".into())),
            );
            assert_eq!(
                expression_noise_axes(&conditional, &deps, 4),
                BTreeSet::from([1, 3]),
                "conditional predicates select a derivative branch but are not differentiated"
            );

            let absdelay = IrExpr::AbsDelay {
                site: AbsDelaySiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr: Box::new(noise(0)),
                delay_time: Box::new(noise(1)),
                max_delay: Some(Box::new(noise(2))),
            };
            assert_eq!(
                expression_noise_axes(&absdelay, &deps, 4),
                BTreeSet::from([0, 1]),
                "absdelay differentiates its value and delay, not max-delay metadata"
            );

            let transition = IrExpr::Transition {
                site: TransitionSiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr: Box::new(noise(0)),
                delay: Some(Box::new(noise(1))),
                rise_time: Some(Box::new(noise(2))),
                fall_time: Some(Box::new(noise(3))),
            };
            assert_eq!(
                expression_noise_axes(&transition, &deps, 4),
                BTreeSet::from([0]),
                "transition timing operands are primal-only"
            );

            let slew = IrExpr::Slew {
                site: SlewSiteId {
                    source: 0,
                    start: 0,
                    end: 1,
                    ordinal: 0,
                },
                expr: Box::new(noise(0)),
                max_pos_slew: Some(Box::new(noise(1))),
                max_neg_slew: Some(Box::new(noise(2))),
            };
            assert_eq!(
                expression_noise_axes(&slew, &deps, 4),
                BTreeSet::from([0, 1, 2]),
                "slew has derivative action through its value and rate operands"
            );
        }
    }
}
