//! Verilog-A/AMS Abstract Syntax Tree
//!
//! This module defines the complete AST for Verilog-A LRM 2.4 with
//! Verilog-AMS extensions. The AST is designed for:
//!
//! 1. **Completeness** - Represents all language constructs
//! 2. **Fidelity** - Preserves source locations for error reporting
//! 3. **Traversability** - Easy to walk and transform

use crate::four_state::FourStateLiteral;
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
    /// Global Verilog-AMS default-transition compiler directive. The
    /// preprocessor retains it as a typed top-level item so
    /// declaration-order scope is not lost before semantic analysis.
    DefaultTransition(DefaultTransitionDirective),
    /// Global Verilog-AMS default-discipline compiler directive, retained the
    /// same way and for the same reason.
    DefaultDiscipline(DefaultDisciplineDirective),
    /// Module definition
    Module(Module),
    /// Discipline definition
    Discipline(DisciplineDef),
    /// Nature definition
    Nature(NatureDef),
    /// Connectmodule definition (Verilog-AMS)
    ConnectModule(Module),
    /// Connect specification block (Verilog-AMS LRM 2.4 section 7.7).
    ConnectRules(ConnectRulesDecl),
    /// Paramset definition
    ParamSet(ParamSetDef),
}

/// One `connectrules` … `endconnectrules` block.
///
/// Verilog-AMS LRM 2.4 Syntax 7-5 (`connectrules_declaration`, from A.1.8).
/// The block is a *specification*: it names which of the declared connect
/// modules the elaborator may auto-insert, and how otherwise-compatible
/// disciplines resolve against one another. It declares no behaviour of its
/// own.
#[derive(Debug, Clone)]
pub struct ConnectRulesDecl {
    /// The `connectrules_identifier` the block is named by.
    pub name: SmolStr,
    /// Items in source order, which is the order section 7.7.2.1's
    /// "first match wins" rule reads them in.
    pub items: Vec<ConnectRulesItem>,
    pub span: Span,
}

/// Syntax 7-5's `connectrules_item`.
#[derive(Debug, Clone)]
pub enum ConnectRulesItem {
    /// Syntax 7-6's `connect_insertion`.
    Insertion(ConnectInsertion),
    /// Syntax 7-7's `connect_resolution`.
    Resolution(ConnectResolution),
}

/// `connect <cm> [merged|split] [#(...)] [<port overrides>] ;`
///
/// Verilog-AMS LRM 2.4 Syntax 7-6.
#[derive(Debug, Clone)]
pub struct ConnectInsertion {
    /// The `connectmodule_identifier` this statement designates.
    pub connect_module: SmolStr,
    /// `None` is section 7.8.3's default, which is [`ConnectMode::Merged`].
    /// Retained as written so a reader of the AST can tell a default from a
    /// spelled-out `merged`.
    pub mode: Option<ConnectMode>,
    /// Section 7.7.3's parameter passing attribute.
    pub parameters: Vec<ParameterOverride>,
    /// Section 7.7.1's `connect_port_overrides`, when written.
    pub port_overrides: Option<ConnectPortOverrides>,
    pub span: Span,
}

/// Syntax 7-6's `connect_mode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectMode {
    /// One connect module for all the ports a rule applies to (section 7.8.3.1).
    Merged,
    /// One connect module per port (section 7.8.3.2).
    Split,
}

/// Syntax 7-6's `connect_port_overrides`.
///
/// The grammar admits the bare `discipline , discipline` form as well as the
/// three directed forms; both are held here, with `direction: None` for the
/// bare form so the connect module's own port directions stand.
#[derive(Debug, Clone)]
pub struct ConnectPortOverrides {
    pub first: ConnectPortOverride,
    pub second: ConnectPortOverride,
}

/// One side of a [`ConnectPortOverrides`].
#[derive(Debug, Clone)]
pub struct ConnectPortOverride {
    /// `None` is the undirected form.
    pub direction: Option<PortDirection>,
    pub discipline: SmolStr,
    pub span: Span,
}

/// `connect <d> {, <d>} resolveto <d>|exclude ;`
///
/// Verilog-AMS LRM 2.4 Syntax 7-7.
#[derive(Debug, Clone)]
pub struct ConnectResolution {
    /// The `discipline_list` before `resolveto`.
    pub disciplines: Vec<SmolStr>,
    pub target: ConnectResolveTarget,
    pub span: Span,
}

/// Syntax 7-7's `discipline_identifier_or_exclude`.
#[derive(Debug, Clone)]
pub enum ConnectResolveTarget {
    /// The discipline the list resolves to. Section 7.7.2.1: it "need not be
    /// one of the disciplines specified in the discipline list".
    Discipline(SmolStr),
    /// `exclude`: the listed disciplines are declared incompatible, and
    /// finding them on one net is an error.
    Exclude,
}

/// One global default-transition directive.
#[derive(Debug, Clone)]
pub struct DefaultTransitionDirective {
    pub value: Expression,
    pub span: Span,
}

/// One `` `default_discipline `` directive (Verilog-AMS LRM 2.4 section 10.2).
#[derive(Debug, Clone)]
pub struct DefaultDisciplineDirective {
    /// `None` is the reset form: "if this directive is used without a
    /// discipline name, discipline resolution will not use a default
    /// discipline for nets declared after this directive is encountered in
    /// the text stream".
    pub discipline: Option<SmolStr>,
    pub span: Span,
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
    /// Discrete-domain net declarations (`wire`), in declaration order
    pub digital_nets: Vec<DigitalNetDecl>,
    /// Discrete-domain variable declarations (`reg`), in declaration order
    pub digital_variables: Vec<DigitalVariableDecl>,
    /// Continuous assignments (`assign`), in declaration order
    pub continuous_assigns: Vec<ContinuousAssign>,
    /// Procedural processes (`always`, `initial`), in declaration order.
    ///
    /// Each carries the [`DigitalProcessId`] it was given here, so a later
    /// pass can name a process without depending on its index in this vector.
    pub digital_processes: Vec<DigitalProcess>,
    /// `genvar` declarations (IEEE 1364-2005 section 12.1.3.2).
    ///
    /// Retained after the generate regions are unrolled, because the names are
    /// what a diagnostic about a misused one has to quote.
    pub genvars: Vec<GenvarDecl>,
    /// Generate regions **as written**, before elaboration-time unrolling.
    ///
    /// Empty on every module the rest of the compiler ever sees: the parser
    /// unrolls each region at `endmodule` and appends the result to the item
    /// lists above, so nothing downstream has a second shape of module item to
    /// understand. A region that survives here is one that could not be
    /// unrolled, and the parser has already refused it by then.
    pub generates: Vec<GenerateConstruct>,
    /// Module attributes
    pub attributes: Vec<Attribute>,
    /// Source span
    pub span: Span,
}

impl Module {
    /// Whether the module declares anything from the discrete half of
    /// Verilog-AMS.
    pub fn has_digital_content(&self) -> bool {
        !self.digital_nets.is_empty()
            || !self.digital_variables.is_empty()
            || !self.continuous_assigns.is_empty()
            || !self.digital_processes.is_empty()
    }

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
            digital_nets: Vec::new(),
            digital_variables: Vec::new(),
            continuous_assigns: Vec::new(),
            digital_processes: Vec::new(),
            genvars: Vec::new(),
            generates: Vec::new(),
            attributes: Vec::new(),
            span,
        }
    }
}

/// `genvar i, j;` — IEEE 1364-2005 section 12.1.3.2.
#[derive(Debug, Clone)]
pub struct GenvarDecl {
    pub names: Vec<SmolStr>,
    pub span: Span,
}

/// One construct of a generate region, IEEE 1364-2005 section 12.4.
#[derive(Debug, Clone)]
pub enum GenerateConstruct {
    /// `for (i = 0; i < N; i = i + 1) begin : name ... end` (section 12.4.1).
    Loop(Box<GenerateLoop>),
    /// `if (constant) ... else ...` (section 12.4.2).
    Conditional(Box<GenerateConditional>),
    /// `case (constant) ... endcase` (section 12.4.2).
    Case(Box<GenerateCase>),
    /// A block, or a bare module item, written directly in the region.
    Block(GenerateBlock),
}

impl GenerateConstruct {
    pub fn span(&self) -> Span {
        match self {
            Self::Loop(loop_) => loop_.span,
            Self::Conditional(conditional) => conditional.span,
            Self::Case(case) => case.span,
            Self::Block(block) => block.span,
        }
    }

    /// The keyword that opened it, for a diagnostic that has to name it.
    pub const fn keyword(&self) -> &'static str {
        match self {
            Self::Loop(_) => "for",
            Self::Conditional(_) => "if",
            Self::Case(_) => "case",
            Self::Block(_) => "begin",
        }
    }
}

/// The body of one generate construct.
///
/// The module items are parsed into an ordinary [`Module`] used purely as an
/// item bucket, so that one production reads every module item form and the
/// unroller has one shape to copy out of. Nothing about that inner module is a
/// module: it has no name of its own, no ports, and never reaches semantic
/// analysis.
#[derive(Debug, Clone)]
pub struct GenerateBlock {
    /// `begin : name`. Section 12.4.1 requires one on a generate loop, because
    /// the loop's instances are named by the block and the index.
    pub name: Option<SmolStr>,
    pub items: Box<Module>,
    /// Generate constructs written inside this block, in declaration order.
    pub nested: Vec<GenerateConstruct>,
    pub span: Span,
}

/// `for (genvar = init; condition; genvar = update) body`
#[derive(Debug, Clone)]
pub struct GenerateLoop {
    pub genvar: SmolStr,
    pub init: Expression,
    pub condition: Expression,
    /// The genvar's next value, as the header's assignment writes it.
    pub update: Expression,
    pub body: GenerateBlock,
    pub span: Span,
}

/// `if (condition) then_block [else else_block]`
#[derive(Debug, Clone)]
pub struct GenerateConditional {
    pub condition: Expression,
    pub then_block: GenerateBlock,
    pub else_block: Option<GenerateBlock>,
    pub span: Span,
}

/// `case (selector) labels: block ... [default: block] endcase`
#[derive(Debug, Clone)]
pub struct GenerateCase {
    pub selector: Expression,
    pub items: Vec<GenerateCaseItem>,
    pub default: Option<GenerateBlock>,
    pub span: Span,
}

/// One arm of a generate case.
#[derive(Debug, Clone)]
pub struct GenerateCaseItem {
    pub labels: Vec<Expression>,
    pub block: GenerateBlock,
    pub span: Span,
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
    /// Packed vector range written on the port: `input [7:0] bus;`.
    ///
    /// `None` is a scalar port, which is every port a continuous-domain
    /// module declares.
    pub range: Option<VectorRange>,
    /// Whether the port was declared `signed`. Continuous-domain ports never
    /// are.
    pub signedness: Signedness,
    /// The net or variable type written on the port itself.
    ///
    /// IEEE 1364-2005 section 12.3.4 lets a port declaration carry its own
    /// type — `output reg [3:0] q;` — as an alternative to the two-declaration
    /// form `output [3:0] q; reg [3:0] q;`. The two mean the same thing, and
    /// the parser records which spelling was used so the module item can
    /// synthesize the second declaration the compact form stands for.
    ///
    /// `None` is a port with no type of its own, which section 12.3.3 makes an
    /// implicit net.
    pub net_type: Option<PortNetType>,
    /// Port names
    pub names: Vec<SmolStr>,
    /// Source span
    pub span: Span,
}

/// A net or variable type written on a port declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortNetType {
    /// `output wire y;`
    Wire,
    /// `output reg [3:0] q;`
    Reg,
    /// `input wreal in;` — Verilog-AMS LRM 2.4 section 6.5.3's real-valued
    /// port, whose syntax (section 6.5.2) admits `wreal` wherever a net type
    /// may be written.
    Wreal(WrealResolution),
    /// `output real vout;` — a real-valued *variable* port.
    ///
    /// The other half of Verilog-AMS LRM 2.4 section 6.5.2's port grammar: a
    /// port may carry a net type, of which `wreal` is one, or a variable type,
    /// of which `real` is one. IEEE 1364-2005 section 12.3.4 already reads the
    /// variable form for `output reg q;` — a port declaration carrying its own
    /// type, standing for the two-declaration form — and this is exactly that
    /// form with section 3.9's `real` as the type.
    ///
    /// The difference from [`Self::Wreal`] is the difference between a net and
    /// a variable, and it is the whole point of admitting it: section 6.2 lets
    /// a procedural assignment write a variable and not a net, so `output real`
    /// is the port a process can write and `output wreal` is not.
    Real,
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
    /// Whether the source wrote an explicit parameter type. Scalar parameters
    /// retain the language default when this is false; parameter arrays require
    /// an explicit element type so later storage lowering cannot guess it.
    pub type_is_explicit: bool,
    /// Parameter name
    pub name: SmolStr,
    /// Fixed declaration dimensions. These are retained even on compiler
    /// configurations whose external parameter ABI does not yet implement
    /// array-valued overrides, so they can fail closed with a precise
    /// diagnostic instead of being silently treated as scalars.
    pub dimensions: Vec<ArrayDimension>,
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

/// Branch access: `V(a)`, `V(a, b)`, `I(a)`, `I(a, b)`, `I(<branch>)`
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
        direction: Option<Box<Expression>>,
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
    /// Explicitly omitted positional argument (for example the null zeros
    /// operand in `zi_zp(x,,p,T)`). Legality is operator-specific and is
    /// checked by semantic analysis.
    NullArgument(Span),
    /// Branch access: V(a, b), I(a, b)
    BranchAccess(BranchAccess),
    /// Array access: `arr[i]`
    ArrayAccess(ArrayAccessExpr),
    /// Array/concatenation literal: {a, b, c}
    ArrayLiteral(ArrayLiteralExpr),
    /// Analog operator (ddt, idt, ddx, etc.)
    AnalogOperator(AnalogOperator),
    /// Noise source
    NoiseSource(NoiseSource),
    /// An expression form that exists only in the discrete (IEEE 1364) half
    /// of Verilog-AMS.
    ///
    /// One variant, rather than one per form, so every continuous-domain
    /// consumer has exactly one arm to refuse. Semantic analysis stops these
    /// before any executable lowering runs, so nothing downstream of the
    /// analyzer ever observes one.
    Digital(DigitalExpr),
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
            Expression::NullArgument(span) => *span,
            Expression::BranchAccess(b) => b.span(),
            Expression::ArrayAccess(a) => a.span,
            Expression::ArrayLiteral(a) => a.span,
            Expression::AnalogOperator(o) => o.span(),
            Expression::NoiseSource(n) => n.span(),
            Expression::Digital(d) => d.span(),
        }
    }
}

/// Expression forms belonging to the discrete half of Verilog-AMS.
#[derive(Debug, Clone)]
pub enum DigitalExpr {
    /// Four-state literal: `4'b10x1`, `8'hzF`, `'bx`.
    FourState(FourStateLit),
    /// Constant part-select of a vector: `bus[7:4]`.
    ///
    /// The indexed forms `bus[base +: width]` and `bus[base -: width]` are not
    /// part of this wave and are refused by name.
    PartSelect(PartSelectExpr),
    /// Bitwise XNOR: `a ~^ b`, `a ^~ b` (IEEE 1364-2005 section 4.1.9).
    ///
    /// Here rather than as a [`BinaryOp`] because the continuous half of the
    /// language has no XNOR: its bitwise operators run on machine integers and
    /// reach a bytecode instruction, a native x86-64 encoding, and a WebAssembly
    /// opcode, none of which has one. A [`BinaryOp`] variant would therefore
    /// have to be given a continuous-domain meaning in four backends to gain a
    /// discrete-domain one here, and the standard's own answer — `~(a ^ b)` — is
    /// already spellable there.
    Xnor(XnorExpr),
    /// Case equality: `a === b`, `a !== b` (section 4.1.8).
    CaseEquality(CaseEqualityExpr),
    /// A reduction operator: `&a`, `~&a`, `|a`, `~|a`, `^a`, `~^a` (section
    /// 4.1.10).
    Reduction(ReductionExpr),
    /// Arithmetic right shift: `a >>> b` (section 4.1.12).
    ///
    /// Here rather than as a [`BinaryOp`] for the reason [`Self::Xnor`] gives.
    /// The continuous half of the language shifts machine integers through a
    /// bytecode instruction, a native encoding and a WebAssembly opcode; a
    /// [`BinaryOp`] variant would have to be given a meaning in each of those
    /// to gain a discrete-domain one here. There is no `<<<` variant, because
    /// section 4.1.12 makes `<<<` the same operation as `<<` and the lexer
    /// spells them one token.
    ArithmeticShiftRight(ArithmeticShiftExpr),
}

impl DigitalExpr {
    pub fn span(&self) -> Span {
        match self {
            Self::FourState(literal) => literal.span,
            Self::PartSelect(select) => select.span,
            Self::Xnor(xnor) => xnor.span,
            Self::CaseEquality(equality) => equality.span,
            Self::Reduction(reduction) => reduction.span,
            Self::ArithmeticShiftRight(shift) => shift.span,
        }
    }

    /// Name of the construct, for diagnostics that must say what they refused.
    pub const fn construct(&self) -> &'static str {
        match self {
            Self::FourState(_) => "four-state literal",
            Self::PartSelect(_) => "part-select",
            Self::Xnor(_) => "bitwise XNOR operator",
            Self::CaseEquality(_) => "case equality operator",
            Self::Reduction(_) => "reduction operator",
            Self::ArithmeticShiftRight(_) => "arithmetic right shift operator",
        }
    }

    /// Sub-expressions, so a generic expression walk can descend without
    /// knowing the discrete forms.
    pub fn children(&self) -> Vec<&Expression> {
        match self {
            Self::FourState(_) => Vec::new(),
            Self::PartSelect(select) => vec![&select.msb, &select.lsb],
            Self::Xnor(xnor) => vec![&xnor.left, &xnor.right],
            Self::CaseEquality(equality) => vec![&equality.left, &equality.right],
            Self::Reduction(reduction) => vec![&reduction.operand],
            Self::ArithmeticShiftRight(shift) => vec![&shift.left, &shift.right],
        }
    }

    /// The signal this expression reads, when it names one.
    ///
    /// Only a part-select does. The operators name no signal of their own —
    /// whatever they read is in [`Self::children`], which is where a walk that
    /// wants every read has to look.
    pub fn base_name(&self) -> Option<&SmolStr> {
        match self {
            Self::FourState(_)
            | Self::Xnor(_)
            | Self::CaseEquality(_)
            | Self::Reduction(_)
            | Self::ArithmeticShiftRight(_) => None,
            Self::PartSelect(select) => Some(&select.name),
        }
    }
}

/// Bitwise XNOR: `a ~^ b` or `a ^~ b`.
#[derive(Debug, Clone)]
pub struct XnorExpr {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Arithmetic right shift: `a >>> b`.
///
/// The left operand is context-determined and carries the result size; the
/// right is a number of positions and is self-determined, exactly as for `>>`
/// (IEEE 1364-2005 table 5-22). What differs is only the fill, and only when
/// the shift's own expression is signed.
#[derive(Debug, Clone)]
pub struct ArithmeticShiftExpr {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

/// Case equality: `a === b`, or its negation `a !== b`.
#[derive(Debug, Clone)]
pub struct CaseEqualityExpr {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    /// `true` for `!==`. Carried as a flag rather than as two variants because
    /// section 4.1.8 defines `!==` as the complement of `===` and nothing but
    /// the sense differs.
    pub negate: bool,
    pub span: Span,
}

/// A reduction operator applied to one operand.
#[derive(Debug, Clone)]
pub struct ReductionExpr {
    pub op: ReductionOp,
    pub operand: Box<Expression>,
    pub span: Span,
}

/// Which reduction operator, IEEE 1364-2005 section 4.1.10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionOp {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

impl ReductionOp {
    /// The operator as the source spells it.
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::And => "&",
            Self::Nand => "~&",
            Self::Or => "|",
            Self::Nor => "~|",
            Self::Xor => "^",
            Self::Xnor => "~^",
        }
    }

    /// Whether the folded result is inverted on the way out.
    ///
    /// Section 4.1.10: "The reduction nand, reduction nor, and reduction xnor
    /// operators shall be computed as the reduction and, reduction or, and
    /// reduction xor operators, respectively, followed by inverting the
    /// single-bit result."
    pub const fn inverts(self) -> bool {
        matches!(self, Self::Nand | Self::Nor | Self::Xnor)
    }
}

/// A decoded four-state literal at a source position.
#[derive(Debug, Clone)]
pub struct FourStateLit {
    pub value: FourStateLiteral,
    pub span: Span,
}

/// Constant part-select: `name[msb:lsb]`.
#[derive(Debug, Clone)]
pub struct PartSelectExpr {
    pub name: SmolStr,
    pub msb: Box<Expression>,
    pub lsb: Box<Expression>,
    pub span: Span,
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

/// One item inside a concatenation or assignment pattern.
///
/// Replication remains recursive in the syntax tree.  The parser must never
/// expand the repeat count because it is an expression and because even a
/// constant count can exceed every practical compiler work budget.
#[derive(Debug, Clone)]
pub enum ArrayLiteralElement {
    /// An ordinary expression item.
    Value(Expression),
    /// A Verilog replication item such as `4{value}`.
    Replication(ReplicationExpr),
}

impl ArrayLiteralElement {
    pub fn span(&self) -> Span {
        match self {
            Self::Value(expression) => expression.span(),
            Self::Replication(replication) => replication.span,
        }
    }

    /// Return the first retained replication at or below this item.
    pub fn first_replication(&self) -> Option<&ReplicationExpr> {
        match self {
            Self::Value(Expression::ArrayLiteral(array)) => array.first_replication(),
            Self::Value(_) => None,
            Self::Replication(replication) => Some(replication),
        }
    }
}

/// Recursive replication item: `count{element, ...}`.
#[derive(Debug, Clone)]
pub struct ReplicationExpr {
    pub count: Box<Expression>,
    pub elements: Vec<ArrayLiteralElement>,
    pub span: Span,
}

/// Array/concatenation literal expression: `{a, b, c}` or `'{a, b, c}`.
#[derive(Debug, Clone)]
pub struct ArrayLiteralExpr {
    pub elements: Vec<ArrayLiteralElement>,
    /// True for the Verilog-AMS assignment-pattern opener `'{`; false for an
    /// ordinary concatenation `{`. These constructs are not interchangeable.
    pub assignment_pattern: bool,
    pub span: Span,
}

impl ArrayLiteralExpr {
    /// Return the first recursive replication without expanding it.
    pub fn first_replication(&self) -> Option<&ReplicationExpr> {
        self.elements
            .iter()
            .find_map(ArrayLiteralElement::first_replication)
    }
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
    /// Slew rate limiter: slew(expr, max_pos_rate, max_neg_rate)
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
        /// Sample-period constant argument, evaluated and frozen at analysis
        /// start. Its frozen value must be finite and greater than zero.
        period: Box<Expression>,
        /// Dynamic output transition time. Omission uses the effective
        /// global `default_transition` setting.
        transition: Option<Box<Expression>>,
        /// First-transition constant argument, evaluated and frozen at
        /// analysis start. Omission means zero.
        first_transition: Option<Box<Expression>>,
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
        /// Dense identity assigned by semantic lowering. Parsed ASTs carry
        /// `None`; every executable analyzed module carries `Some`.
        process_id: Option<u32>,
        power: Box<Expression>,
        name: Option<SmolStr>,
        span: Span,
    },
    /// Flicker noise: flicker_noise(pwr, exp, name)
    Flicker {
        process_id: Option<u32>,
        power: Box<Expression>,
        exponent: Box<Expression>,
        name: Option<SmolStr>,
        span: Span,
    },
    /// Noise table: noise_table(vector, name)
    Table {
        process_id: Option<u32>,
        data: Vec<Expression>,
        /// `true` for `noise_table_log`, `false` for `noise_table`.
        log_interp: bool,
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
    pub parameters: Vec<ParameterOverride>,
    pub span: Span,
}

/// Parameter override on a module instance. `name == None` is an ordered
/// override; named and ordered overrides are retained distinctly so later
/// elaboration never has to reconstruct source intent from a synthetic name.
#[derive(Debug, Clone)]
pub struct ParameterOverride {
    pub name: Option<SmolStr>,
    pub value: Expression,
    pub span: Span,
}

/// Port connection
#[derive(Debug, Clone)]
pub enum Connection {
    /// Ordered connection. `None` retains an explicitly unconnected port.
    Ordered {
        signal: Option<Expression>,
        span: Span,
    },
    /// Named connection: .port(signal)
    Named {
        port: SmolStr,
        /// `None` retains `.port()` rather than inventing a signal.
        signal: Option<Expression>,
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

// ============================================================================
// Discrete-domain (IEEE 1364-2005) syntax tree
// ============================================================================
//
// Verilog-AMS is a superset of Verilog, so these nodes live in the same tree
// as the analog ones and are produced by the same parser. Three properties are
// deliberate, because the process IR that consumes them must not have to
// re-parse or re-infer anything:
//
//  1. **Source-faithful.** `always @(posedge clk) q <= d;` is a process whose
//     body is a timing-controlled statement — not a desugared analog event,
//     not a hoisted sensitivity list with the control removed. The suspension
//     points a process IR needs are exactly the `@`/`#` nodes the author
//     wrote, in the positions the author wrote them.
//  2. **Identified.** Every process carries a [`DigitalProcessId`], assigned
//     in module declaration order at parse time and stable for the life of the
//     module. A later pass names a process by its id rather than by its index
//     in a vector it does not own.
//  3. **Derivable, not duplicated.** Sensitivity is not stored twice.
//     [`DigitalProcess::event_control`] reads it back off the body when the
//     process opens with one, which is the only shape from which a static
//     sensitivity list is meaningful.
//
// Nothing here shares a node with the analog event grammar
// ([`EventExpr`]). An analog `@(cross(...))` lowers to a continuous-time
// guard that is evaluated on every Newton iteration; a digital `@(posedge c)`
// suspends a process until an edge occurs. Giving them one node would put two
// unrelated execution models behind one `match`.

/// Signedness qualifier on a net, variable, or port declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Signedness {
    #[default]
    Unsigned,
    Signed,
}

impl Signedness {
    pub const fn is_signed(self) -> bool {
        matches!(self, Self::Signed)
    }
}

/// Packed vector range `[msb:lsb]`.
///
/// Both bounds are retained as written. IEEE 1364-2005 section 4.2.1 permits
/// either direction (`[7:0]` and `[0:7]` are both legal and are not the same
/// declaration), so normalizing to an ascending pair here would discard the
/// author's bit ordering.
#[derive(Debug, Clone)]
pub struct VectorRange {
    pub msb: Expression,
    pub lsb: Expression,
    pub span: Span,
}

/// One name in a discrete-domain declaration.
#[derive(Debug, Clone)]
pub struct DigitalDeclItem {
    pub name: SmolStr,
    /// Unpacked (memory) dimensions: `reg [7:0] mem [0:255];`
    pub dimensions: Vec<ArrayDimension>,
    /// Declaration-site initializer, as in `wire w = a & b;`.
    pub init: Option<Expression>,
    pub span: Span,
}

/// How a net with more than one driver combines their contributions.
///
/// Verilog-AMS LRM 2.4 section 6.5.3 says of a real-valued net that "there can
/// be a maximum of one driver of a real-valued net", and section 3.7 gives no
/// resolution function for one — the LRM has none to give. So [`Self::Single`]
/// is the standard's own reading and the only one a `wreal` declaration
/// selects: a second driver on such a net is refused rather than combined.
///
/// The other four spellings are **not** Accellera's. `wrealsum`, `wrealavg`,
/// `wrealmin` and `wrealmax` are the de facto real-number-modelling extension
/// that Cadence AMS Designer introduced and that RNM libraries are written
/// against; RSpice implements them under those names, opt-in at the
/// declaration, so that a design which asks for a resolution gets the one it
/// named and a design which does not gets the LRM's refusal. Nothing here is
/// reached by writing `wreal`.
///
/// The spelling mechanism is the **net type keyword**, and only that. The
/// alternative — selecting a resolution through a discipline attribute — is
/// refused by name at the declaration, because a design that could say the same
/// thing two ways can say two different things in one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrealResolution {
    /// `wreal`: one driver, per LRM 2.4 section 6.5.3.
    Single,
    /// `wrealsum`: the sum of every driver's contribution.
    Sum,
    /// `wrealavg`: the sum divided by the number of drivers the net has.
    Average,
    /// `wrealmin`: the least contribution.
    Minimum,
    /// `wrealmax`: the greatest contribution.
    Maximum,
}

impl WrealResolution {
    /// The net-type keyword that selects this resolution.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Single => "wreal",
            Self::Sum => "wrealsum",
            Self::Average => "wrealavg",
            Self::Minimum => "wrealmin",
            Self::Maximum => "wrealmax",
        }
    }

    /// The resolution a net-type keyword names, if it names one.
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Some(match keyword {
            "wreal" => Self::Single,
            "wrealsum" => Self::Sum,
            "wrealavg" => Self::Average,
            "wrealmin" => Self::Minimum,
            "wrealmax" => Self::Maximum,
            _ => return None,
        })
    }

    /// Whether more than one driver on such a net is well formed.
    ///
    /// False for `wreal` alone: LRM 2.4 section 6.5.3 permits one driver, and
    /// combining two would be inventing a rule the standard declined to state.
    pub const fn admits_multiple_drivers(self) -> bool {
        !matches!(self, Self::Single)
    }
}

/// Net type keyword.
///
/// `wire` is IEEE 1364-2005's; [`Self::Wreal`] is Verilog-AMS LRM 2.4 section
/// 3.7's real net. The remaining IEEE 1364 net types are still refused by name
/// at the keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalNetKind {
    Wire,
    /// A real-valued net: `wreal`, and the four resolved spellings.
    Wreal(WrealResolution),
}

impl DigitalNetKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Wire => "wire",
            Self::Wreal(resolution) => resolution.keyword(),
        }
    }

    /// Whether the net carries a real value rather than four-state bits.
    pub const fn is_real(self) -> bool {
        matches!(self, Self::Wreal(_))
    }

    /// How the net combines its drivers, for a real net.
    pub const fn resolution(self) -> Option<WrealResolution> {
        match self {
            Self::Wire => None,
            Self::Wreal(resolution) => Some(resolution),
        }
    }
}

/// `wire [msb:lsb] a, b;`
#[derive(Debug, Clone)]
pub struct DigitalNetDecl {
    pub kind: DigitalNetKind,
    pub signedness: Signedness,
    pub range: Option<VectorRange>,
    pub items: Vec<DigitalDeclItem>,
    pub span: Span,
}

/// Variable type keyword for a discrete-domain variable declaration.
///
/// `integer` keeps its existing continuous-domain [`VariableDecl`] node: IEEE
/// 1364-2005 section 3.9 gives `integer` no range and no signedness qualifier,
/// so the digital grammar needs nothing new to declare one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalVariableKind {
    Reg,
    /// `real`, as a variable of the *discrete* domain.
    ///
    /// The same keyword the continuous domain declares its variables with, and
    /// deliberately the same keyword: IEEE 1364-2005 section 3.9 defines one
    /// `real` variable type, and Verilog-AMS does not fork it. What differs is
    /// which half of the language owns the storage, which is a question about
    /// the module rather than about the declaration — see the ownership rule in
    /// [`crate::canonical_ir::digital_lower`].
    ///
    /// A declaration reaches this kind two ways: an `output real` port, where
    /// the discrete domain is what the port grammar was asking for, and a
    /// module-level `real` promoted out of the continuous domain because a
    /// process writes it and nothing analog can.
    Real,
}

impl DigitalVariableKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Reg => "reg",
            Self::Real => "real",
        }
    }

    /// Whether the variable carries a real rather than four-state bits.
    pub const fn is_real(self) -> bool {
        matches!(self, Self::Real)
    }
}

/// `reg [msb:lsb] q;`
#[derive(Debug, Clone)]
pub struct DigitalVariableDecl {
    pub kind: DigitalVariableKind,
    pub signedness: Signedness,
    pub range: Option<VectorRange>,
    pub items: Vec<DigitalDeclItem>,
    pub span: Span,
}

/// `assign [#delay] lvalue = expression;`
#[derive(Debug, Clone)]
pub struct ContinuousAssign {
    pub target: DigitalLValue,
    pub value: Expression,
    /// Delay written between `assign` and the target.
    pub delay: Option<Expression>,
    pub span: Span,
}

/// Stable identity of a process within its module.
///
/// Assigned by the parser in declaration order, starting at zero. It survives
/// every later pass, so a process IR, a diagnostic, and a runtime schedule can
/// all name the same process without agreeing on a container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigitalProcessId(pub u32);

impl std::fmt::Display for DigitalProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Which procedural process this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalProcessKind {
    /// `always`: restarts as soon as it finishes (IEEE 1364-2005 section 9.9.2).
    Always,
    /// `initial`: runs once (IEEE 1364-2005 section 9.9.1).
    Initial,
}

impl DigitalProcessKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Initial => "initial",
        }
    }
}

/// One `always` or `initial` process.
#[derive(Debug, Clone)]
pub struct DigitalProcess {
    pub id: DigitalProcessId,
    pub kind: DigitalProcessKind,
    /// Statement the process executes, exactly as written. A process opening
    /// with a timing control has that control as its outermost node.
    pub body: DigitalStatement,
    pub span: Span,
}

impl DigitalProcess {
    /// The event control this process opens with, if it opens with one.
    ///
    /// This is the static sensitivity list: a process whose body begins with
    /// `@(...)` suspends there on every pass, which is what makes the list
    /// meaningful. A process that reaches a timing control later, or has
    /// several, has no single static list and reports `None`.
    pub fn event_control(&self) -> Option<&EventControl> {
        match &self.body {
            DigitalStatement::Timing(timing) => match &timing.control {
                TimingControl::Event(event) => Some(event),
                TimingControl::Delay(_) => None,
            },
            _ => None,
        }
    }

    /// Whether the process contains any timing control at all.
    ///
    /// An `always` process with none never suspends, so it cannot advance
    /// simulation time (IEEE 1364-2005 section 9.9.2).
    pub fn has_timing_control(&self) -> bool {
        self.body.contains_timing_control()
    }
}

/// A statement inside a procedural process.
#[derive(Debug, Clone)]
pub enum DigitalStatement {
    /// `begin ... end`, optionally named, optionally with local declarations.
    Block(DigitalBlock),
    /// `lvalue = [timing] expression;`
    BlockingAssign(DigitalAssign),
    /// `lvalue <= [timing] expression;`
    NonblockingAssign(DigitalAssign),
    /// `if (cond) stmt [else stmt]`
    Conditional(DigitalConditional),
    /// `case`, `casez`, or `casex`.
    Case(DigitalCase),
    /// `for (init; cond; update) stmt`
    For(DigitalFor),
    /// `while (cond) stmt`
    While(DigitalWhile),
    /// `repeat (count) stmt`
    Repeat(DigitalRepeat),
    /// `forever stmt`
    Forever(DigitalForever),
    /// `@(...) stmt` or `#delay stmt`, the process's suspension points.
    Timing(DigitalTiming),
    /// A lone `;`.
    Null(Span),
}

impl DigitalStatement {
    pub fn span(&self) -> Span {
        match self {
            Self::Block(block) => block.span,
            Self::BlockingAssign(assign) | Self::NonblockingAssign(assign) => assign.span,
            Self::Conditional(conditional) => conditional.span,
            Self::Case(case) => case.span,
            Self::For(statement) => statement.span,
            Self::While(statement) => statement.span,
            Self::Repeat(statement) => statement.span,
            Self::Forever(statement) => statement.span,
            Self::Timing(timing) => timing.span,
            Self::Null(span) => *span,
        }
    }

    /// Whether this statement, or anything under it, suspends.
    pub fn contains_timing_control(&self) -> bool {
        match self {
            Self::Timing(_) => true,
            Self::Block(block) => block.statements.iter().any(Self::contains_timing_control),
            Self::Conditional(conditional) => {
                conditional.then_branch.contains_timing_control()
                    || conditional
                        .else_branch
                        .as_ref()
                        .is_some_and(|branch| branch.contains_timing_control())
            }
            Self::Case(case) => {
                case.items
                    .iter()
                    .any(|item| item.statement.contains_timing_control())
                    || case
                        .default
                        .as_ref()
                        .is_some_and(|statement| statement.contains_timing_control())
            }
            Self::For(statement) => statement.body.contains_timing_control(),
            Self::While(statement) => statement.body.contains_timing_control(),
            Self::Repeat(statement) => statement.body.contains_timing_control(),
            Self::Forever(statement) => statement.body.contains_timing_control(),
            Self::BlockingAssign(assign) | Self::NonblockingAssign(assign) => {
                assign.timing.is_some()
            }
            Self::Null(_) => false,
        }
    }
}

/// `begin [: name] ... end`
#[derive(Debug, Clone)]
pub struct DigitalBlock {
    pub name: Option<SmolStr>,
    /// Block-local continuous-domain declarations (`integer`, `real`).
    pub variables: Vec<VariableDecl>,
    /// Block-local `reg` declarations.
    pub digital_variables: Vec<DigitalVariableDecl>,
    pub statements: Vec<DigitalStatement>,
    pub span: Span,
}

/// A procedural assignment, blocking or nonblocking.
#[derive(Debug, Clone)]
pub struct DigitalAssign {
    pub target: DigitalLValue,
    /// Intra-assignment timing control: `q <= #5 d;` or `q = @(posedge c) d;`
    /// (IEEE 1364-2005 section 9.2.2).
    pub timing: Option<TimingControl>,
    pub value: Expression,
    pub span: Span,
}

/// Assignment target inside a process or a continuous assignment.
#[derive(Debug, Clone)]
pub enum DigitalLValue {
    /// A whole signal: `q`
    Identifier { name: SmolStr, span: Span },
    /// A single bit or memory word: `q[3]`
    BitSelect {
        name: SmolStr,
        index: Box<Expression>,
        span: Span,
    },
    /// A constant part-select: `bus[7:4]`
    PartSelect {
        name: SmolStr,
        msb: Box<Expression>,
        lsb: Box<Expression>,
        span: Span,
    },
    /// A concatenation of targets: `{carry, sum}`
    Concat {
        elements: Vec<DigitalLValue>,
        span: Span,
    },
}

impl DigitalLValue {
    pub fn span(&self) -> Span {
        match self {
            Self::Identifier { span, .. }
            | Self::BitSelect { span, .. }
            | Self::PartSelect { span, .. }
            | Self::Concat { span, .. } => *span,
        }
    }

    /// Every signal name this target writes, in source order.
    pub fn written_names(&self) -> Vec<(&SmolStr, Span)> {
        let mut names = Vec::new();
        self.collect_written_names(&mut names);
        names
    }

    fn collect_written_names<'a>(&'a self, names: &mut Vec<(&'a SmolStr, Span)>) {
        match self {
            Self::Identifier { name, span }
            | Self::BitSelect { name, span, .. }
            | Self::PartSelect { name, span, .. } => names.push((name, *span)),
            Self::Concat { elements, .. } => {
                for element in elements {
                    element.collect_written_names(names);
                }
            }
        }
    }
}

/// `if (condition) then_branch [else else_branch]`
#[derive(Debug, Clone)]
pub struct DigitalConditional {
    pub condition: Expression,
    pub then_branch: Box<DigitalStatement>,
    pub else_branch: Option<Box<DigitalStatement>>,
    pub span: Span,
}

/// Which of the three case forms was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseKind {
    /// `case`: `x` and `z` in a label match only themselves.
    Exact,
    /// `casez`: `z` and `?` in either operand are don't-care.
    WildcardZ,
    /// `casex`: `x`, `z`, and `?` in either operand are don't-care.
    WildcardXZ,
}

impl CaseKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Exact => "case",
            Self::WildcardZ => "casez",
            Self::WildcardXZ => "casex",
        }
    }
}

/// `case (selector) item... [default: stmt] endcase`
#[derive(Debug, Clone)]
pub struct DigitalCase {
    pub kind: CaseKind,
    pub selector: Expression,
    pub items: Vec<DigitalCaseItem>,
    pub default: Option<Box<DigitalStatement>>,
    pub span: Span,
}

/// One `label[, label]*: statement` arm of a case statement.
#[derive(Debug, Clone)]
pub struct DigitalCaseItem {
    pub labels: Vec<Expression>,
    pub statement: Box<DigitalStatement>,
    pub span: Span,
}

/// `for (init; condition; update) body`
#[derive(Debug, Clone)]
pub struct DigitalFor {
    pub init: Box<DigitalAssign>,
    pub condition: Expression,
    pub update: Box<DigitalAssign>,
    pub body: Box<DigitalStatement>,
    pub span: Span,
}

/// `while (condition) body`
#[derive(Debug, Clone)]
pub struct DigitalWhile {
    pub condition: Expression,
    pub body: Box<DigitalStatement>,
    pub span: Span,
}

/// `repeat (count) body`
#[derive(Debug, Clone)]
pub struct DigitalRepeat {
    pub count: Expression,
    pub body: Box<DigitalStatement>,
    pub span: Span,
}

/// `forever body`
#[derive(Debug, Clone)]
pub struct DigitalForever {
    pub body: Box<DigitalStatement>,
    pub span: Span,
}

/// A timing control and the statement it guards.
#[derive(Debug, Clone)]
pub struct DigitalTiming {
    pub control: TimingControl,
    /// `None` for a bare `@(posedge clk);`, which suspends and does nothing.
    pub statement: Option<Box<DigitalStatement>>,
    pub span: Span,
}

/// `@(...)` or `#delay`.
#[derive(Debug, Clone)]
pub enum TimingControl {
    Event(EventControl),
    Delay(DelayControl),
}

impl TimingControl {
    pub fn span(&self) -> Span {
        match self {
            Self::Event(event) => event.span,
            Self::Delay(delay) => delay.span,
        }
    }
}

/// `#expression`
#[derive(Debug, Clone)]
pub struct DelayControl {
    pub value: Expression,
    pub span: Span,
}

/// A digital event control: the sensitivity list of a suspension point.
#[derive(Debug, Clone)]
pub struct EventControl {
    pub sensitivity: Sensitivity,
    pub span: Span,
}

/// How the event control names the signals it waits on.
#[derive(Debug, Clone)]
pub enum Sensitivity {
    /// `@*` or `@(*)`: every signal read by the guarded statement
    /// (IEEE 1364-2005 section 9.7.5). The list is *not* materialized here —
    /// computing it requires reading the statement, which is a later pass's
    /// job, and storing a stale copy would be worse than storing none.
    Implicit,
    /// An explicit list. Terms are separated by `or` or `,`, which
    /// IEEE 1364-2005 section 9.7.4 makes synonyms.
    Explicit(Vec<EventTerm>),
}

/// One term of an explicit sensitivity list.
#[derive(Debug, Clone)]
pub struct EventTerm {
    /// `None` is a level-sensitive term: any value change triggers it.
    pub edge: Option<EdgeKind>,
    pub signal: Expression,
    pub span: Span,
}

/// `posedge` or `negedge`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Posedge,
    Negedge,
}

impl EdgeKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Posedge => "posedge",
            Self::Negedge => "negedge",
        }
    }
}
