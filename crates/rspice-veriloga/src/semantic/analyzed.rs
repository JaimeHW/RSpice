//! Public analyzed-output data shapes produced by the semantic analyzer.

use super::SymbolTable;
use crate::ast::{Expression, ParamType, PortDirection, SourceFile, VarType};
use crate::source::Span;
use crate::types::{ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Analyzed source file with resolved symbols
#[derive(Debug, Clone)]
pub struct AnalyzedFile {
    pub source: SourceFile,
    pub modules: HashMap<SmolStr, AnalyzedModule>,
}

/// Analyzed module with resolved types
#[derive(Debug, Clone)]
pub struct AnalyzedModule {
    pub name: SmolStr,
    pub ports: Vec<AnalyzedPort>,
    pub parameters: Vec<AnalyzedParameter>,
    /// Parameter aliases (aliasparam): alternate instance-facing names
    /// resolving to entries of `parameters`
    pub param_aliases: Vec<AnalyzedParamAlias>,
    pub variables: Vec<AnalyzedVariable>,
    pub branches: Vec<AnalyzedBranch>,
    pub contributions: Vec<AnalyzedContribution>,
    /// Ordered evaluation statements (assignments and runtime loops),
    /// executed before the contributions on every device evaluation
    pub statements: Vec<AnalyzedStatement>,
    pub internal_nodes: Vec<AnalyzedInternalNode>,
    /// Names of nets declared `ground` (they map to the global reference)
    pub ground_nodes: Vec<SmolStr>,
    /// Array variables: name -> contiguous element storage layout
    pub arrays: HashMap<SmolStr, AnalyzedArray>,
    pub symbol_table: SymbolTable,
}

/// An analyzed array variable: elements occupy contiguous slots in the
/// variable storage starting at `base`
#[derive(Debug, Clone)]
pub struct AnalyzedArray {
    /// First element's index in the variables list
    pub base: usize,
    /// Declared lower bound (x[lo:hi] indexes from lo)
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
    pub param_type: ParamType,
    pub value_type: ValueType,
    /// Constant default value, when the default expression folds to a constant
    pub default: Option<f64>,
    /// Full default expression (may reference previously declared parameters)
    pub default_expr: Option<Expression>,
    pub range: Option<TypedParameterRange>,
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
    /// Type of the expression
    pub expr_type: ValueType,
    /// Source span
    pub span: Span,
    /// Name of the snapshotted, unfiltered `initial_step` guard enclosing
    /// this assignment. This provenance lets canonical lowering remove only
    /// that guard when a single-assignment initialization is proven static.
    pub unfiltered_initial_step_guard: Option<SmolStr>,
}
