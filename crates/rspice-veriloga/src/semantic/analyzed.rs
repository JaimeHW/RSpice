//! Public analyzed-output data shapes produced by the semantic analyzer.

use super::{AnalyzedDigital, SymbolTable};
use crate::ast::{Expression, ParamType, PortDirection, SourceFile, VarType};
use crate::source::Span;
use crate::types::{ParameterRange as TypedParameterRange, ValueType};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Analyzed source file with resolved symbols
#[derive(Debug, Clone)]
pub struct AnalyzedFile {
    pub source: SourceFile,
    pub modules: HashMap<SmolStr, AnalyzedModule>,
    /// Non-fatal findings raised while analyzing this file, in source order.
    pub warnings: Vec<SemanticWarning>,
    /// The file's `connectmodule` declarations and `connectrules` blocks,
    /// validated against one another and against the discipline database.
    ///
    /// Empty for the overwhelming majority of files, which declare neither.
    /// It is built here rather than at the point of use because the checks it
    /// performs — that a `connect` statement names a declared connect module,
    /// that the module bridges one continuous and one discrete discipline,
    /// that an overriding discipline is compatible with the one it overrides —
    /// are the author's to see at compile time.
    pub connect_rules: crate::connect::ConnectRuleTable,
}

/// One non-fatal finding raised by semantic analysis.
///
/// An error stops compilation and travels as a [`crate::error::CompileError`].
/// A warning does not: the analyzed module is exactly what it would have been
/// without the finding, and the finding is published to the caller on the
/// runtime compile report so an editor can pin it to its line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticWarning {
    /// Stable compiler-owned diagnostic code, for example
    /// `VA-SEM-NO-EFFECT-SYSTEM-TASK`.
    pub code: &'static str,
    /// One sentence in the compiler's voice, naming the exact construct.
    pub message: String,
    /// The construct this finding is about.
    pub span: Span,
}

/// Analyzed module with resolved types
#[derive(Debug, Clone)]
pub struct AnalyzedModule {
    pub name: SmolStr,
    pub default_transition: f64,
    /// The `` `default_discipline `` in effect where this module was declared
    /// (Verilog-AMS LRM 2.4 section 10.2), which is the default Annex F.2.1
    /// step 4b applies to a discrete net of this module that resolution leaves
    /// with no discipline of its own.
    ///
    /// `None` is both "no directive was written" and section 10.2's reset
    /// form, which have the same effect.
    pub default_discipline: Option<SmolStr>,
    /// Number of semantic noise processes allocated in this module. Process
    /// IDs are dense from zero and are remapped per instance during hierarchy
    /// elaboration so repeated children never alias one another.
    pub noise_process_count: u32,
    pub ports: Vec<AnalyzedPort>,
    pub parameters: Vec<AnalyzedParameter>,
    /// Parameter aliases (aliasparam): alternate instance-facing names
    /// resolving to entries of `parameters`
    pub param_aliases: Vec<AnalyzedParamAlias>,
    pub variables: Vec<AnalyzedVariable>,
    /// Sorted, duplicate-free variable slots written by an event-controlled
    /// procedural body. These slots require accepted/candidate lifecycle
    /// handling at runtime; ordinary procedural variables deliberately remain
    /// outside that transaction set.
    pub event_state_variables: Vec<usize>,
    pub branches: Vec<AnalyzedBranch>,
    pub contributions: Vec<AnalyzedContribution>,
    /// Ordered evaluation statements (assignments and runtime loops),
    /// executed before the contributions on every device evaluation
    pub statements: Vec<AnalyzedStatement>,
    /// Indices into [`Self::statements`] of the module *prologue*: the
    /// localparam and module-scope variable initializers that run before the
    /// analog block.
    ///
    /// These are the statements [`Self::body`] has no counterpart for at all.
    /// The analog block's own steps are recorded twice — once here with the
    /// enclosing guard folded in, once in `body` with the control flow intact —
    /// but a prologue statement runs *before* the `analog` keyword, so the
    /// region tree has no position to hold it. A consumer that reads only
    /// `body` therefore has no definition of a prologue variable, and a read of
    /// one falls through to Verilog-AMS zero initialisation. This list is what
    /// such a consumer evaluates first; see
    /// [`crate::canonical_ir::HirModel::prologue_statements`].
    ///
    /// Recorded rather than inferred, for the same reason
    /// [`AnalyzedAssignment::site`] is: the analyzer is the only level that
    /// knows which phase emitted a statement. It is a prefix of `statements` as
    /// one module is analyzed, and stops being one as soon as hierarchy
    /// elaboration appends an instance's statements after the parent's, which
    /// is why it is a list of indices and not a count.
    pub prologue_statements: Vec<usize>,
    /// The same analog block with its control flow intact.
    ///
    /// [`Self::statements`] is the historical form: conditionals dissolved into
    /// `guard ? value : previous` so the list is flat. That representation
    /// cannot be recovered from, which is what the backend rebuild
    /// (`design/VERILOGA_BACKEND_PLAN.md`) is about, so the analyzer now also
    /// records the shape it saw.
    ///
    /// Both are produced by one walk and describe the same module. The flat
    /// list is what every current consumer reads and is unchanged; this is what
    /// the CFG level consumes. The flat list goes away with the last of those
    /// consumers.
    pub body: Vec<AnalyzedRegion>,
    /// How many [`AnalogSiteId`]s this module has minted.
    ///
    /// Every id in the module is below it, which is what lets hierarchy
    /// elaboration rebase a child's sites onto the flattened parent the same
    /// way it rebases variable indices.
    pub analog_site_count: u32,
    pub internal_nodes: Vec<AnalyzedInternalNode>,
    /// Names of nets declared `ground` (they map to the global reference)
    pub ground_nodes: Vec<SmolStr>,
    /// Array variables: name -> contiguous element storage layout
    pub arrays: HashMap<SmolStr, AnalyzedArray>,
    pub symbol_table: SymbolTable,
    /// Resolved discrete-domain (IEEE 1364) content.
    ///
    /// Empty for every continuous-domain model. When it is not, the module has
    /// passed parse and semantic analysis and is refused at every executable
    /// backend boundary, by name — see
    /// [`crate::semantic::AnalyzedDigital::first_construct`].
    pub digital: AnalyzedDigital,
}

/// Identity of one analog-block site that *both* lowerings of the module
/// describe.
///
/// The analyzer emits every guarded step twice: once into [`AnalyzedModule::body`]
/// as the author wrote it, and once into [`AnalyzedModule::statements`] /
/// [`AnalyzedModule::contributions`] with the enclosing guard folded in. The two
/// copies are literally clones of one expression at the moment the guard is
/// applied, so the analyzer is the only level that can state their
/// correspondence without reconstructing it — and it states it by stamping one
/// id on both copies as they are built.
///
/// Canonical lowering turns that into
/// [`crate::canonical_ir::HirExecutedCorrespondence`], which is what lets a
/// CFG-sourced backend name the state record an operator owns: the CFG carries
/// body-copy expression ids, and the runtime's state arrays are numbered in the
/// executed copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnalogSiteId(pub u32);

impl std::fmt::Display for AnalogSiteId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "AnalogSiteId({})", self.0)
    }
}

/// How the executed copy of a site wraps the copy the author wrote.
///
/// The analyzer applies exactly one of these shapes when it folds a guard, so
/// naming the shape is enough for canonical lowering to walk from the executed
/// root back down to the expression the structured body holds. Recording it is
/// not an optimisation over inspecting the lowered tree: an *unguarded* `a ? b
/// : c` assignment and a *guarded* one are the same shape, and only the
/// analyzer knows which it built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalogSiteGuard {
    /// No guard was active: the executed copy is the authored expression.
    None,
    /// `guard ? authored : fallback` — the authored copy is the then-arm.
    Select,
    /// `guard && authored` — the authored copy is the right operand.
    Conjunction,
}

/// One step of the analog block, with control flow intact.
///
/// The expressions here are *unguarded*: an assignment inside an `if` carries
/// what the source wrote, not `cond ? written : previous`. Reconstructing
/// "previous" is precisely the bounded-search problem this representation
/// exists to delete, and it is only recoverable while the walk still knows
/// which branch it is in.
#[derive(Debug, Clone)]
pub enum AnalyzedRegion {
    Assignment(AnalyzedAssignment),
    Contribution(AnalyzedContribution),
    Conditional {
        condition: Expression,
        /// The site of the `__guardN` snapshot assignment holding this
        /// condition's executed copy, when one was materialised.
        ///
        /// `None` when [`super::SemanticAnalyzer::snapshot_guard`] passed the
        /// condition through — which it does exactly for an identifier or a
        /// numeric literal, neither of which can own a state record. So an
        /// unpaired condition is not a hole in the correspondence: it is a leaf.
        condition_site: Option<AnalogSiteId>,
        then_body: Vec<AnalyzedRegion>,
        else_body: Vec<AnalyzedRegion>,
        span: Span,
    },
    /// Loop whose trip count is not known until run time.
    ///
    /// Compile-time-bounded loops are unrolled before they reach here, as they
    /// are for the flat list.
    Loop {
        condition: Expression,
        /// The flat [`AnalyzedLoop`] this region was recorded beside.
        site: AnalogSiteId,
        body: Vec<AnalyzedRegion>,
        span: Span,
    },
}

/// An analyzed array variable: elements occupy contiguous slots in the
/// variable storage starting at `base`
#[derive(Debug, Clone)]
pub struct AnalyzedArray {
    /// First element's index in the variables list
    pub base: usize,
    /// Declared lower bound (`x[lo:hi]` indexes from `lo`)
    pub lower: i64,
    /// Number of elements
    pub len: usize,
}

/// An ordered evaluation step of the analog block
#[derive(Debug, Clone)]
pub enum AnalyzedStatement {
    /// Variable assignment
    Assignment(AnalyzedAssignment),
    /// Loop whose bounds are only known at runtime (e.g. parameter
    /// dependent). The condition is re-evaluated before every iteration.
    Loop(AnalyzedLoop),
}

/// Runtime-bounded loop over assignment statements
#[derive(Debug, Clone)]
pub struct AnalyzedLoop {
    /// Loop continues while this evaluates nonzero (any enclosing guard
    /// is folded in, so a guarded loop runs zero iterations when inactive)
    pub condition: Expression,
    /// Identity shared with the [`AnalyzedRegion::Loop`] recorded for the same
    /// source loop, and the shape by which `condition` wraps that region's.
    pub site: AnalogSiteId,
    pub condition_guard: AnalogSiteGuard,
    /// Loop body (assignments and nested loops)
    pub body: Vec<AnalyzedStatement>,
    /// Source span
    pub span: Span,
}

/// Analyzed port
#[derive(Debug, Clone)]
pub struct AnalyzedPort {
    pub name: SmolStr,
    pub direction: PortDirection,
    pub discipline: SmolStr,
    pub nature_potential: Option<SmolStr>,
    pub nature_flow: Option<SmolStr>,
}

/// Analyzed parameter
#[derive(Debug, Clone)]
pub struct AnalyzedParameter {
    pub name: SmolStr,
    /// Whether this parameter belongs to the selected module's public ABI.
    /// Parameters of flattened child instances remain runtime slots so their
    /// defaults and ranges retain exact semantics, but are not externally
    /// settable or reported as top-level model parameters.
    pub is_public: bool,
    /// Whether the source declares shared model-card or per-device storage.
    pub scope: ParameterScope,
    /// Xyce/CMC dual-scope convention: an instance parameter may take a
    /// model-card fallback while retaining independent instance-given state.
    pub also_model: bool,
    pub param_type: ParamType,
    pub value_type: ValueType,
    /// Ordered declaration dimensions. Bounds remain symbolic because an
    /// instance may override an earlier scalar parameter that shapes this
    /// array. No compile-time flat storage base is implied here.
    pub dimensions: Vec<AnalyzedParameterDimension>,
    /// Constant default value, when the default expression folds to a constant
    pub default: Option<f64>,
    /// Full default expression (may reference previously declared parameters)
    pub default_expr: Option<Expression>,
    pub range: Option<TypedParameterRange>,
}

/// One parameter-array declaration dimension, preserving the source's left to
/// right bound order exactly (including descending ranges).
#[derive(Debug, Clone)]
pub struct AnalyzedParameterDimension {
    pub left: Expression,
    pub right: Expression,
    pub span: Span,
}

/// Storage and preprocessing scope requested by a Verilog-A parameter.
///
/// CMC compact models mark geometry and per-device switches with
/// `(* type="instance" *)`. Unmarked parameters are model-card parameters,
/// matching the convention used by the published CMC model corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterScope {
    #[default]
    Model,
    Instance,
}

impl ParameterScope {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Instance => "instance",
        }
    }
}

/// Analyzed parameter alias (aliasparam): an alternate instance-facing
/// name for an existing parameter. Setting the alias on an instance
/// writes the target; the alias itself is not a parameter and the module
/// body may not reference it.
#[derive(Debug, Clone)]
pub struct AnalyzedParamAlias {
    /// Alias name
    pub alias: SmolStr,
    /// Index of the target in the parameters list
    pub target: usize,
}

/// Analyzed variable
#[derive(Debug, Clone)]
pub struct AnalyzedVariable {
    pub name: SmolStr,
    pub var_type: VarType,
    pub value_type: ValueType,
    pub is_state: bool,
}

/// Analyzed internal node (not connected to external ports)
#[derive(Debug, Clone)]
pub struct AnalyzedInternalNode {
    pub name: SmolStr,
    pub discipline: SmolStr,
    pub index: usize, // Index within internal nodes array
}

/// Analyzed branch
#[derive(Debug, Clone)]
pub struct AnalyzedBranch {
    pub name: SmolStr,
    pub pos_node: SmolStr,
    pub neg_node: SmolStr,
    pub discipline: SmolStr,
}

/// Analyzed contribution
#[derive(Debug, Clone)]
pub struct AnalyzedContribution {
    pub branch: SmolStr,
    pub declared_branch: Option<SmolStr>,
    pub is_current: bool,
    /// Indirect (implicit-equation) contribution: `expression` holds the
    /// constraint residual `lhs - rhs` that the unknown source drives to
    /// zero
    pub indirect: bool,
    pub expression: Expression,
    /// Identity shared with the [`AnalyzedRegion::Contribution`] recorded for
    /// the same `<+`, and the shape by which this copy's `expression` wraps
    /// that region's.
    pub site: AnalogSiteId,
    pub expression_guard: AnalogSiteGuard,
    pub expr_type: ValueType,
    pub span: Span,
}

/// Analyzed variable assignment
#[derive(Debug, Clone)]
pub struct AnalyzedAssignment {
    /// Variable name being assigned
    pub target: SmolStr,
    /// Index of variable in variables list (for array targets: the base
    /// element)
    pub var_index: usize,
    /// Runtime element index for array targets whose index does not fold
    /// at compile time (relative to the array's declared lower bound
    /// after evaluation)
    pub index: Option<Expression>,
    /// The expression being assigned
    pub expression: Expression,
    /// Identity shared with the [`AnalyzedRegion::Assignment`] recorded for the
    /// same source assignment, and the shape by which this copy's `expression`
    /// wraps that region's.
    ///
    /// A statement with no region counterpart — a `__guard` snapshot, a
    /// synthesized `repeat` counter — still carries a site, because it is the
    /// executed half of a pair whose other half may be a region *condition*
    /// rather than a region assignment.
    pub site: AnalogSiteId,
    pub expression_guard: AnalogSiteGuard,
    /// Type of the expression
    pub expr_type: ValueType,
    /// Source span
    pub span: Span,
    /// Name of the snapshotted, unfiltered `initial_step` guard enclosing
    /// this assignment. This provenance lets canonical lowering remove only
    /// that guard when a single-assignment initialization is proven static.
    pub unfiltered_initial_step_guard: Option<SmolStr>,
}
