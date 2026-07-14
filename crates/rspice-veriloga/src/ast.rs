//! Verilog-A/AMS Abstract Syntax Tree
//!
//! This module defines the complete AST for Verilog-A LRM 2.4 with
//! Verilog-AMS extensions. The AST is designed for:
//!
//! 1. **Completeness** - Represents all language constructs
//! 2. **Fidelity** - Preserves source locations for error reporting
//! 3. **Traversability** - Easy to walk and transform

use crate::source::Span;
use smol_str::SmolStr;

/// A complete Verilog-A source file
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// All items in the file (modules, disciplines, natures)
    pub items: Vec<Item>,
    /// Span of the entire file
    pub span: Span,
}

/// Top-level item in a source file
#[derive(Debug, Clone)]
pub enum Item {
    /// Module definition
    Module(Module),
    /// Discipline definition
    Discipline(DisciplineDef),
    /// Nature definition
    Nature(NatureDef),
    /// Connectmodule definition (Verilog-AMS)
    ConnectModule(Module),
    /// Paramset definition
    ParamSet(ParamSetDef),
}

/// Module definition
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: SmolStr,
    /// Module ports in declaration order
    pub ports: Vec<Port>,
    /// Port declarations with types
    pub port_declarations: Vec<PortDeclaration>,
    /// Parameter declarations
    pub parameters: Vec<ParameterDecl>,
    /// Local parameter declarations
    pub localparams: Vec<ParameterDecl>,
    /// Parameter alias declarations (aliasparam)
    pub aliasparams: Vec<AliasParamDecl>,
    /// Variable declarations
    pub variables: Vec<VariableDecl>,
    /// Net declarations (electrical nodes, etc.)
    pub nets: Vec<NetDecl>,
    /// Branch declarations
    pub branches: Vec<BranchDecl>,
    /// Analog block (main behavioral code)
    pub analog_block: Option<AnalogBlock>,
    /// Analog initial block
    pub analog_initial: Option<AnalogBlock>,
    /// Analog final block (for cleanup)
    pub analog_final: Option<AnalogBlock>,
    /// Module instances
    pub instances: Vec<ModuleInstance>,
    /// Function definitions
    pub functions: Vec<FunctionDef>,
    /// Module attributes
    pub attributes: Vec<Attribute>,
    /// Source span
    pub span: Span,
}

impl Module {
    pub fn new(name: impl Into<SmolStr>, span: Span) -> Self {
        Self {
            name: name.into(),
            ports: Vec::new(),
            port_declarations: Vec::new(),
            parameters: Vec::new(),
            localparams: Vec::new(),
            aliasparams: Vec::new(),
            variables: Vec::new(),
            nets: Vec::new(),
            branches: Vec::new(),
            analog_block: None,
            analog_initial: None,
            analog_final: None,
            instances: Vec::new(),
            functions: Vec::new(),
            attributes: Vec::new(),
            span,
        }
    }
}

/// Port in module port list
#[derive(Debug, Clone)]
pub struct Port {
    pub name: SmolStr,
    pub span: Span,
}

/// Port declaration with direction and discipline
#[derive(Debug, Clone)]
pub struct PortDeclaration {
    /// Port direction
    pub direction: PortDirection,
    /// Discipline (electrical, thermal, etc.)
    pub discipline: Option<SmolStr>,
    /// Port names
    pub names: Vec<SmolStr>,
    /// Source span
    pub span: Span,
}

/// Port direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
}

/// Parameter declaration
#[derive(Debug, Clone)]
pub struct ParameterDecl {
    /// Parameter type (real, integer, string)
    pub param_type: ParamType,
    /// Parameter name
    pub name: SmolStr,
    /// Default value
    pub default: Option<Expression>,
    /// Range constraint
    pub range: Option<ParameterRange>,
    /// Units string
    pub units: Option<SmolStr>,
    /// Description
    pub description: Option<SmolStr>,
    /// Attributes
    pub attributes: Vec<Attribute>,
    /// Source span
    pub span: Span,
}

/// Parameter alias declaration: `aliasparam alias = target;`
///
/// The alias is an alternate instance-facing name for an existing
/// parameter. Per the LRM it is not a parameter itself - it carries no
/// default and no storage - and the module body may not reference it.
#[derive(Debug, Clone)]
pub struct AliasParamDecl {
    /// Alias name
    pub alias: SmolStr,
    /// Name of the target parameter
    pub target: SmolStr,
    /// Source span
    pub span: Span,
}

/// Parameter type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParamType {
    #[default]
    Real,
    Integer,
    String,
}

/// Parameter range constraint
#[derive(Debug, Clone)]
pub struct ParameterRange {
    /// Range bounds
    pub bounds: Vec<RangeBound>,
    /// Excluded values
    pub exclude: Vec<Expression>,
    /// Source span
    pub span: Span,
}

/// Single range bound
#[derive(Debug, Clone)]
pub struct RangeBound {
    /// Lower bound (None = -inf)
    pub lower: Option<Expression>,
    /// Lower bound inclusive
    pub lower_inclusive: bool,
    /// Upper bound (None = +inf)
    pub upper: Option<Expression>,
    /// Upper bound inclusive
    pub upper_inclusive: bool,
    /// Source span
    pub span: Span,
}

/// Variable declaration
#[derive(Debug, Clone)]
pub struct VariableDecl {
    /// Variable type
    pub var_type: VarType,
    /// Variable names and optional array dimensions
    pub items: Vec<VariableItem>,
    /// Source span
    pub span: Span,
}

/// Single variable in a declaration
#[derive(Debug, Clone)]
pub struct VariableItem {
    pub name: SmolStr,
    /// Array dimensions (empty for scalar)
    pub dimensions: Vec<ArrayDimension>,
    /// Initial value
    pub init: Option<Expression>,
    /// Source span
    pub span: Span,
}

/// Variable type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarType {
    Real,
    Integer,
    String,
}

/// Array dimension
#[derive(Debug, Clone)]
pub struct ArrayDimension {
    pub start: Expression,
    pub end: Expression,
    pub span: Span,
}

/// Net declaration (nodes, wires, etc.)
#[derive(Debug, Clone)]
pub struct NetDecl {
    /// Discipline (electrical, thermal, etc.)
    pub discipline: SmolStr,
    /// Net names
    pub names: Vec<SmolStr>,
    /// Whether this is a ground node
    pub is_ground: bool,
    /// Whether this is an internal node (not in port list)
    pub is_internal: bool,
    /// Source span
    pub span: Span,
}

/// Branch declaration
#[derive(Debug, Clone)]
pub struct BranchDecl {
    /// Branch name
    pub name: SmolStr,
    /// Positive terminal
    pub pos: SmolStr,
    /// Negative terminal
    pub neg: SmolStr,
    /// Source span
    pub span: Span,
}

/// Analog block
#[derive(Debug, Clone)]
pub struct AnalogBlock {
    /// Statements in the analog block
    pub statements: Vec<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// Analog statement
#[derive(Debug, Clone)]
pub enum AnalogStatement {
    /// Contribution statement: I(p, n) <+ expr
    Contribution(ContributionStmt),
    /// Indirect contribution: V(p, n) : expr == target
    IndirectContribution(IndirectContributionStmt),
    /// Conditional: if (cond) stmt else stmt
    Conditional(ConditionalStmt),
    /// Case statement
    Case(CaseStmt),
    /// For loop
    For(ForStmt),
    /// While loop
    While(WhileStmt),
    /// Repeat loop
    Repeat(RepeatStmt),
    /// Block: begin ... end
    Block(BlockStmt),
    /// Assignment: var = expr
    Assignment(AssignmentStmt),
    /// Event control: @(event) stmt
    EventControl(EventControlStmt),
    /// Function/task call
    Call(CallStmt),
    /// Disable statement
    Disable(DisableStmt),
    /// Null statement (just semicolon)
    Null(Span),
}

/// Contribution statement: I(a, b) <+ expression
#[derive(Debug, Clone)]
pub struct ContributionStmt {
    /// Branch access being contributed to
    pub target: BranchAccess,
    /// Contributed expression
    pub value: Expression,
    /// Source span
    pub span: Span,
}

/// Indirect contribution (implicit equations)
#[derive(Debug, Clone)]
pub struct IndirectContributionStmt {
    /// Branch access
    pub branch: BranchAccess,
    /// Left side of equation
    pub lhs: Expression,
    /// Right side of equation (target value)
    pub rhs: Expression,
    /// Source span
    pub span: Span,
}

/// Branch access: V(a), V(a, b), I(a), I(a, b), I(<branch>)
#[derive(Debug, Clone)]
pub enum BranchAccess {
    /// Access by node(s): V(a) or V(a, b)
    Nodes {
        /// Access function (V, I, etc.)
        access: SmolStr,
        /// Positive node
        pos: SmolStr,
        /// Negative node (None for single-ended)
        neg: Option<SmolStr>,
        /// Source span
        span: Span,
    },
    /// Access by named branch: I(<branch_name>)
    Branch {
        /// Access function
        access: SmolStr,
        /// Branch name
        name: SmolStr,
        /// Source span
        span: Span,
    },
}

impl BranchAccess {
    pub fn span(&self) -> Span {
        match self {
            BranchAccess::Nodes { span, .. } | BranchAccess::Branch { span, .. } => *span,
        }
    }
}

/// Conditional statement
#[derive(Debug, Clone)]
pub struct ConditionalStmt {
    /// Condition expression
    pub condition: Expression,
    /// Then branch
    pub then_branch: Box<AnalogStatement>,
    /// Optional else branch
    pub else_branch: Option<Box<AnalogStatement>>,
    /// Source span
    pub span: Span,
}

/// Case statement
#[derive(Debug, Clone)]
pub struct CaseStmt {
    /// Expression to match
    pub expr: Expression,
    /// Case items
    pub items: Vec<CaseItem>,
    /// Default case
    pub default: Option<Box<AnalogStatement>>,
    /// Source span
    pub span: Span,
}

/// Case item
#[derive(Debug, Clone)]
pub struct CaseItem {
    /// Match expressions
    pub matches: Vec<Expression>,
    /// Statement to execute
    pub statement: Box<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// For loop
#[derive(Debug, Clone)]
pub struct ForStmt {
    /// Loop variable
    pub var: SmolStr,
    /// Initial value
    pub init: Expression,
    /// Condition
    pub condition: Expression,
    /// Update assignment
    pub update: Box<AssignmentStmt>,
    /// Loop body
    pub body: Box<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// While loop
#[derive(Debug, Clone)]
pub struct WhileStmt {
    /// Condition
    pub condition: Expression,
    /// Loop body
    pub body: Box<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// Repeat loop
#[derive(Debug, Clone)]
pub struct RepeatStmt {
    /// Count expression
    pub count: Expression,
    /// Loop body
    pub body: Box<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// Block statement
#[derive(Debug, Clone)]
pub struct BlockStmt {
    /// Optional block name
    pub name: Option<SmolStr>,
    /// Block statements
    pub statements: Vec<AnalogStatement>,
    /// Local variables
    pub variables: Vec<VariableDecl>,
    /// Source span
    pub span: Span,
}

/// Assignment statement
#[derive(Debug, Clone)]
pub struct AssignmentStmt {
    /// Target variable
    pub target: LValue,
    /// Value expression
    pub value: Expression,
    /// Source span
    pub span: Span,
}

impl AssignmentStmt {
    /// Name of the assignment target (array element targets return the
    /// array name)
    pub fn target_name(&self) -> &SmolStr {
        match &self.target {
            LValue::Variable { name, .. } | LValue::ArrayAccess { name, .. } => name,
        }
    }
}

/// Left-hand side of assignment
#[derive(Debug, Clone)]
pub enum LValue {
    /// Simple variable
    Variable { name: SmolStr, span: Span },
    /// Array element
    ArrayAccess {
        name: SmolStr,
        index: Box<Expression>,
        span: Span,
    },
}

/// Event control statement
#[derive(Debug, Clone)]
pub struct EventControlStmt {
    /// Event expression
    pub event: EventExpr,
    /// Controlled statement
    pub statement: Box<AnalogStatement>,
    /// Source span
    pub span: Span,
}

/// Event expression
#[derive(Debug, Clone)]
pub enum EventExpr {
    /// Posedge of a signal
    Posedge { signal: Expression, span: Span },
    /// Negedge of a signal
    Negedge { signal: Expression, span: Span },
    /// Cross event
    Cross {
        signal: Expression,
        direction: Option<CrossDirection>,
        time_tol: Option<Box<Expression>>,
        expr_tol: Option<Box<Expression>>,
        enable: Option<Box<Expression>>,
        span: Span,
    },
    /// Above threshold event
    Above {
        signal: Expression,
        time_tol: Option<Box<Expression>>,
        expr_tol: Option<Box<Expression>>,
        enable: Option<Box<Expression>>,
        span: Span,
    },
    /// Timer event: timer(start [, period [, time_tol [, enable]]])
    Timer {
        start: Expression,
        period: Option<Box<Expression>>,
        time_tol: Option<Box<Expression>>,
        enable: Option<Box<Expression>>,
        span: Span,
    },
    /// First accepted point of an analysis, optionally filtered by analysis name.
    InitialStep {
        analyses: Vec<StringLit>,
        span: Span,
    },
    /// Last accepted point of an analysis, optionally filtered by analysis name.
    FinalStep {
        analyses: Vec<StringLit>,
        span: Span,
    },
    /// Or of events
    Or {
        left: Box<EventExpr>,
        right: Box<EventExpr>,
        span: Span,
    },
}

/// Cross event direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossDirection {
    Rising,  // +1
    Falling, // -1
    Both,    // 0
}

/// Function/task call statement
#[derive(Debug, Clone)]
pub struct CallStmt {
    /// Function name
    pub name: SmolStr,
    /// Arguments
    pub args: Vec<Expression>,
    /// Source span
    pub span: Span,
}

/// Disable statement
#[derive(Debug, Clone)]
pub struct DisableStmt {
    /// Block name to disable
    pub name: SmolStr,
    /// Source span
    pub span: Span,
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expression {
    /// Numeric literal
    Number(NumberLit),
    /// String literal
    StringLit(StringLit),
    /// Identifier reference
    Identifier(Identifier),
    /// System function ($temperature, $vt, etc.)
    SystemFunction(SystemFunction),
    /// Binary operation
    Binary(BinaryExpr),
    /// Unary operation
    Unary(UnaryExpr),
    /// Conditional expression: cond ? then : else
    Conditional(ConditionalExpr),
    /// Function call
    Call(CallExpr),
    /// Branch access: V(a, b), I(a, b)
    BranchAccess(BranchAccess),
    /// Array access: arr[i]
    ArrayAccess(ArrayAccessExpr),
    /// Array/concatenation literal: {a, b, c}
    ArrayLiteral(ArrayLiteralExpr),
    /// Analog operator (ddt, idt, ddx, etc.)
    AnalogOperator(AnalogOperator),
    /// Noise source
    NoiseSource(NoiseSource),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Number(n) => n.span,
            Expression::StringLit(s) => s.span,
            Expression::Identifier(i) => i.span,
            Expression::SystemFunction(s) => s.span,
            Expression::Binary(b) => b.span,
            Expression::Unary(u) => u.span,
            Expression::Conditional(c) => c.span,
            Expression::Call(c) => c.span,
            Expression::BranchAccess(b) => b.span(),
            Expression::ArrayAccess(a) => a.span,
            Expression::ArrayLiteral(a) => a.span,
            Expression::AnalogOperator(o) => o.span(),
            Expression::NoiseSource(n) => n.span(),
        }
    }
}

/// Number literal
#[derive(Debug, Clone)]
pub struct NumberLit {
    /// The numeric value
    pub value: f64,
    /// Raw text representation
    pub raw: SmolStr,
    /// Source span
    pub span: Span,
}

/// String literal
#[derive(Debug, Clone)]
pub struct StringLit {
    pub value: SmolStr,
    pub span: Span,
}

/// Identifier
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: SmolStr,
    pub span: Span,
}

/// System function call
#[derive(Debug, Clone)]
pub struct SystemFunction {
    /// Function name (with $ prefix)
    pub name: SmolStr,
    /// Arguments
    pub args: Vec<Expression>,
    /// Source span
    pub span: Span,
}

/// Binary expression
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<Expression>,
    pub span: Span,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    // -
    Pos,    // +
    Not,    // !
    BitNot, // ~
}

/// Conditional expression
#[derive(Debug, Clone)]
pub struct ConditionalExpr {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Box<Expression>,
    pub span: Span,
}

/// Function call expression
#[derive(Debug, Clone)]
pub struct CallExpr {
    pub name: SmolStr,
    pub args: Vec<Expression>,
    pub span: Span,
}

/// Array access expression
#[derive(Debug, Clone)]
pub struct ArrayAccessExpr {
    pub array: SmolStr,
    pub index: Box<Expression>,
    pub span: Span,
}

/// Array/concatenation literal expression: {a, b, c}
#[derive(Debug, Clone)]
pub struct ArrayLiteralExpr {
    pub elements: Vec<Expression>,
    pub span: Span,
}

/// Analog operator
#[derive(Debug, Clone)]
pub enum AnalogOperator {
    /// Stateful Newton limiter. This internal form is produced by semantic
    /// lowering for named `$limit` calls; the parser never constructs it.
    Limit {
        /// Raw proposed value at the call site.
        proposed: Box<Expression>,
        /// Inlined limiter function body. Its implicit arguments are carried
        /// by `LimiterArgument` nodes so they cannot collide with source names.
        candidate: Box<Expression>,
        /// Xyce typed-limiter polarity metadata. `None` is the untyped ABI.
        type_metadata: Option<Box<Expression>>,
        /// Canonical selector retained for diagnostics and artifacts.
        selector: SmolStr,
        span: Span,
    },
    /// Implicit argument inside the inlined body of a named limiter.
    LimiterArgument {
        argument: LimiterArgument,
        span: Span,
    },
    /// Time derivative: ddt(expr)
    Ddt {
        expr: Box<Expression>,
        abstol: Option<Box<Expression>>,
        span: Span,
    },
    /// Time integral: idt(expr, ic, assert, abstol)
    Idt {
        expr: Box<Expression>,
        ic: Option<Box<Expression>>,
        assert_val: Option<Box<Expression>>,
        abstol: Option<Box<Expression>>,
        span: Span,
    },
    /// Modulo integral: idtmod(expr, ic, modulus, offset, abstol)
    IdtMod {
        expr: Box<Expression>,
        ic: Option<Box<Expression>>,
        modulus: Option<Box<Expression>>,
        offset: Option<Box<Expression>>,
        abstol: Option<Box<Expression>>,
        span: Span,
    },
    /// Partial derivative: ddx(expr, signal)
    Ddx {
        expr: Box<Expression>,
        probe: BranchAccess,
        span: Span,
    },
    /// Limited exponential: limexp(expr)
    Limexp { expr: Box<Expression>, span: Span },
    /// Absolute delay: absdelay(expr, delay, max_delay)
    Absdelay {
        expr: Box<Expression>,
        delay: Box<Expression>,
        max_delay: Option<Box<Expression>>,
        span: Span,
    },
    /// Transition filter: transition(expr, delay, rise, fall, tol)
    Transition {
        expr: Box<Expression>,
        delay: Option<Box<Expression>>,
        rise: Option<Box<Expression>>,
        fall: Option<Box<Expression>>,
        tolerance: Option<Box<Expression>>,
        span: Span,
    },
    /// Slew rate limiter: slew(expr, max_rise, max_fall)
    Slew {
        expr: Box<Expression>,
        max_rise: Option<Box<Expression>>,
        max_fall: Option<Box<Expression>>,
        span: Span,
    },
    /// Last crossing: last_crossing(expr, edge)
    LastCrossing {
        expr: Box<Expression>,
        edge: Option<CrossDirection>,
        span: Span,
    },
    /// Laplace transfer function
    Laplace {
        kind: LaplaceKind,
        expr: Box<Expression>,
        span: Span,
    },
    /// Z-transform filter
    Zi {
        kind: ZiKind,
        expr: Box<Expression>,
        span: Span,
    },
}

impl AnalogOperator {
    pub fn span(&self) -> Span {
        match self {
            AnalogOperator::Limit { span, .. }
            | AnalogOperator::LimiterArgument { span, .. }
            | AnalogOperator::Ddt { span, .. }
            | AnalogOperator::Idt { span, .. }
            | AnalogOperator::IdtMod { span, .. }
            | AnalogOperator::Ddx { span, .. }
            | AnalogOperator::Limexp { span, .. }
            | AnalogOperator::Absdelay { span, .. }
            | AnalogOperator::Transition { span, .. }
            | AnalogOperator::Slew { span, .. }
            | AnalogOperator::LastCrossing { span, .. }
            | AnalogOperator::Laplace { span, .. }
            | AnalogOperator::Zi { span, .. } => *span,
        }
    }
}

/// Implicit named-`$limit` function inputs supplied by the simulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimiterArgument {
    Proposed,
    Previous,
}

/// Laplace transform kinds
#[derive(Debug, Clone)]
pub enum LaplaceKind {
    /// laplace_zp(expr, zeros, poles)
    ZeroPole {
        zeros: Vec<Expression>,
        poles: Vec<Expression>,
    },
    /// laplace_zd(expr, zeros, poles)
    ZeroDenominator {
        zeros: Vec<Expression>,
        denominator: Vec<Expression>,
    },
    /// laplace_np(expr, numerator, poles)
    NumeratorPole {
        numerator: Vec<Expression>,
        poles: Vec<Expression>,
    },
    /// laplace_nd(expr, numerator, denominator)
    NumeratorDenominator {
        numerator: Vec<Expression>,
        denominator: Vec<Expression>,
    },
}

/// Z-transform kinds
#[derive(Debug, Clone)]
pub enum ZiKind {
    /// zi_zp
    ZeroPole {
        zeros: Vec<Expression>,
        poles: Vec<Expression>,
    },
    /// zi_zd
    ZeroDenominator {
        zeros: Vec<Expression>,
        denominator: Vec<Expression>,
    },
    /// zi_np
    NumeratorPole {
        numerator: Vec<Expression>,
        poles: Vec<Expression>,
    },
    /// zi_nd
    NumeratorDenominator {
        numerator: Vec<Expression>,
        denominator: Vec<Expression>,
    },
}

/// Noise source
#[derive(Debug, Clone)]
pub enum NoiseSource {
    /// White noise: white_noise(pwr, name)
    White {
        power: Box<Expression>,
        name: Option<SmolStr>,
        span: Span,
    },
    /// Flicker noise: flicker_noise(pwr, exp, name)
    Flicker {
        power: Box<Expression>,
        exponent: Box<Expression>,
        name: Option<SmolStr>,
        span: Span,
    },
    /// Noise table: noise_table(vector, name)
    Table {
        data: Vec<Expression>,
        name: Option<SmolStr>,
        span: Span,
    },
}

impl NoiseSource {
    pub fn span(&self) -> Span {
        match self {
            NoiseSource::White { span, .. }
            | NoiseSource::Flicker { span, .. }
            | NoiseSource::Table { span, .. } => *span,
        }
    }
}

/// Discipline definition
#[derive(Debug, Clone)]
pub struct DisciplineDef {
    pub name: SmolStr,
    pub potential: Option<SmolStr>,
    pub flow: Option<SmolStr>,
    pub domain: Option<DomainKind>,
    pub span: Span,
}

/// Domain kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainKind {
    Continuous,
    Discrete,
}

/// Nature definition
#[derive(Debug, Clone)]
pub struct NatureDef {
    pub name: SmolStr,
    pub base: Option<SmolStr>,
    pub access: Option<SmolStr>,
    pub units: Option<SmolStr>,
    pub abstol: Option<Expression>,
    pub idt_nature: Option<SmolStr>,
    pub ddt_nature: Option<SmolStr>,
    pub span: Span,
}

/// Paramset definition
#[derive(Debug, Clone)]
pub struct ParamSetDef {
    pub name: SmolStr,
    pub module: SmolStr,
    pub parameters: Vec<ParameterDecl>,
    pub span: Span,
}

/// Module instance
#[derive(Debug, Clone)]
pub struct ModuleInstance {
    pub module: SmolStr,
    pub name: SmolStr,
    pub connections: Vec<Connection>,
    pub parameters: Vec<(SmolStr, Expression)>,
    pub span: Span,
}

/// Port connection
#[derive(Debug, Clone)]
pub enum Connection {
    /// Ordered connection
    Ordered(Expression),
    /// Named connection: .port(signal)
    Named {
        port: SmolStr,
        signal: Expression,
        span: Span,
    },
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: SmolStr,
    pub return_type: VarType,
    pub params: Vec<FunctionParam>,
    /// Function-local variable declarations
    pub locals: Vec<VariableDecl>,
    pub body: AnalogBlock,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: SmolStr,
    pub param_type: VarType,
    pub direction: ParamDirection,
    pub span: Span,
}

/// Parameter direction for functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamDirection {
    Input,
    Output,
    Inout,
}

/// Attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: SmolStr,
    pub value: Option<Expression>,
    pub span: Span,
}
