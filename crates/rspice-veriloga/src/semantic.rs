//! Semantic Analyzer for Verilog-A/AMS
//!
//! Provides comprehensive semantic analysis including:
//! - Hierarchical symbol table with nested scopes
//! - Type inference and checking
//! - Discipline validation
//! - Expression validation
//! - Parameter range checking

use crate::ast::*;
use crate::disciplines::DisciplineDb;
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use crate::types::{FunctionRegistry, ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::collections::HashMap;

// ============================================================================
// Symbol Table Infrastructure
// ============================================================================

/// Hierarchical symbol table with nested scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Stack of scopes (index 0 = global/module scope)
    scopes: Vec<Scope>,
    /// Current scope index
    current: usize,
}

/// A single scope containing symbols
#[derive(Debug, Clone)]
struct Scope {
    /// Parent scope index (None for module scope)
    parent: Option<usize>,
    /// Symbols in this scope
    symbols: HashMap<SmolStr, Symbol>,
}

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub kind: SymbolKind,
    pub value_type: ValueType,
    pub span: Span,
    pub attrs: SymbolAttrs,
}

/// Symbol kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Port,
    Parameter,
    /// aliasparam name: reserves the identifier so nothing else may
    /// reuse it; not referenceable in the module body
    ParamAlias,
    Variable,
    Node,
    Branch,
    LoopVar,
}

/// Additional symbol attributes
#[derive(Debug, Clone, Default)]
pub struct SymbolAttrs {
    pub direction: Option<PortDirection>,
    pub discipline: Option<SmolStr>,
    pub range: Option<TypedParameterRange>,
    pub is_state: bool,
    pub used: bool,
    /// Whether this is an internal node (not in port list)
    pub is_internal: bool,
    /// Whether this node is declared ground
    pub is_ground: bool,
    /// Index in internal nodes array (for VM access)
    pub internal_node_index: Option<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                parent: None,
                symbols: HashMap::new(),
            }],
            current: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let parent = self.current;
        self.scopes.push(Scope {
            parent: Some(parent),
            symbols: HashMap::new(),
        });
        self.current = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current].parent {
            self.current = parent;
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), Box<Symbol>> {
        let scope = &mut self.scopes[self.current];
        if scope.symbols.contains_key(&symbol.name) {
            return Err(Box::new(scope.symbols.get(&symbol.name).unwrap().clone()));
        }
        scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut scope_idx = Some(self.current);
        while let Some(idx) = scope_idx {
            let scope = &self.scopes[idx];
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
            scope_idx = scope.parent;
        }
        None
    }

    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes[self.current].symbols.get(name)
    }

    pub fn mark_used(&mut self, name: &str) {
        let mut scope_idx = Some(self.current);
        while let Some(idx) = scope_idx {
            let scope = &mut self.scopes[idx];
            if let Some(sym) = scope.symbols.get_mut(name) {
                sym.attrs.used = true;
                return;
            }
            scope_idx = scope.parent;
        }
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut idx = self.current;
        while let Some(parent) = self.scopes[idx].parent {
            depth += 1;
            idx = parent;
        }
        depth
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Semantic Analyzer
// ============================================================================

/// Semantic analyzer for Verilog-A modules
pub struct SemanticAnalyzer {
    disciplines: DisciplineDb,
    functions: FunctionRegistry,
    symbols: SymbolTable,
    errors: Vec<SemanticError>,
    /// User-defined analog functions of the module under analysis
    user_functions: HashMap<SmolStr, FunctionDef>,
    /// Stack of active guard conditions (innermost last)
    guard_stack: Vec<Expression>,
    /// Stack of identifier substitution frames (innermost last). Used for
    /// hoisted block locals, unrolled loop variables, and inlined function
    /// locals.
    subst_stack: Vec<HashMap<SmolStr, Expression>>,
    /// Counter for generating unique hoisted local names
    local_counter: usize,
    /// Constant parameter default values (compile-time diagnostics only:
    /// instances may override parameters, so these must never influence
    /// generated code)
    param_consts: HashMap<SmolStr, f64>,
    /// Values that cannot vary per instance (localparams derived purely
    /// from literals). Safe for loop unrolling and code folding.
    invariant_consts: HashMap<SmolStr, f64>,
    /// Current function inlining depth (recursion guard)
    inline_depth: usize,
    /// Nesting depth of runtime-bounded loops (contributions inside them
    /// are not representable and must error)
    runtime_loop_depth: usize,
    /// Array variables of the module under analysis (name -> layout)
    arrays: HashMap<SmolStr, AnalyzedArray>,
    /// Hidden system-task variables ($bound_step, $discontinuity)
    /// registered on first use
    task_vars: HashMap<SmolStr, usize>,
}

/// Analyzed source file with resolved symbols
#[derive(Debug, Clone)]
pub struct AnalyzedFile {
    pub source: SourceFile,
    pub modules: HashMap<SmolStr, AnalyzedModule>,
}

/// How an event expression lowers into the dataflow representation
enum EventLowering {
    /// Body executes when this runtime guard is nonzero
    Guard(Expression),
    /// Body executes unconditionally
    Always,
    /// Body never affects the device equations (e.g. final_step)
    Never,
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
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            disciplines: DisciplineDb::with_standard(),
            functions: FunctionRegistry::new(),
            symbols: SymbolTable::new(),
            errors: Vec::new(),
            user_functions: HashMap::new(),
            guard_stack: Vec::new(),
            subst_stack: Vec::new(),
            local_counter: 0,
            param_consts: HashMap::new(),
            invariant_consts: HashMap::new(),
            inline_depth: 0,
            runtime_loop_depth: 0,
            arrays: HashMap::new(),
            task_vars: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, source: &SourceFile) -> CompileResult<AnalyzedFile> {
        let mut modules = HashMap::new();

        // First pass: register user-defined disciplines and natures
        for item in &source.items {
            match item {
                Item::Discipline(_disc) => {
                    // Future: add to discipline DB
                }
                Item::Nature(_nat) => {
                    // Future: add to nature DB
                }
                _ => {}
            }
        }

        // Second pass: analyze modules
        for item in &source.items {
            if let Item::Module(module) = item {
                self.symbols = SymbolTable::new();
                self.errors.clear();
                self.user_functions.clear();
                self.guard_stack.clear();
                self.subst_stack.clear();
                self.local_counter = 0;
                self.param_consts.clear();
                self.invariant_consts.clear();
                self.inline_depth = 0;
                self.runtime_loop_depth = 0;

                match self.analyze_module(module) {
                    Ok(analyzed) => {
                        modules.insert(module.name.clone(), analyzed);
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(AnalyzedFile {
            source: source.clone(),
            modules,
        })
    }

    fn analyze_module(&mut self, module: &Module) -> CompileResult<AnalyzedModule> {
        let mut analyzed = AnalyzedModule {
            name: module.name.clone(),
            ports: Vec::new(),
            parameters: Vec::new(),
            param_aliases: Vec::new(),
            variables: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            statements: Vec::new(),
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
            arrays: HashMap::new(),
            symbol_table: SymbolTable::new(),
        };
        // Evaluation statements accumulate in a local sink so loop bodies
        // can recurse into their own sinks without aliasing the module
        let mut statements: Vec<AnalyzedStatement> = Vec::new();
        self.user_functions = module
            .functions
            .iter()
            .map(|f| (f.name.clone(), f.clone()))
            .collect();
        self.arrays.clear();
        self.task_vars.clear();

        // Phase 1: Collect port names from module header
        let port_names: Vec<SmolStr> = module.ports.iter().map(|p| p.name.clone()).collect();

        // Phase 2: Process port declarations to get direction and discipline
        let mut port_info: HashMap<SmolStr, (PortDirection, Option<SmolStr>)> = HashMap::new();
        for decl in &module.port_declarations {
            for name in &decl.names {
                port_info.insert(name.clone(), (decl.direction, decl.discipline.clone()));
            }
        }

        // Phase 3: Update port disciplines from net declarations
        for net in &module.nets {
            let discipline = net.discipline.clone();
            for name in &net.names {
                // If this is a port, update its discipline
                if let Some((dir, _)) = port_info.get(name) {
                    port_info.insert(name.clone(), (*dir, Some(discipline.clone())));
                }
            }
        }

        // Phase 4: Define ports in symbol table FIRST
        for port_name in &port_names {
            let (direction, discipline) = port_info
                .get(port_name)
                .cloned()
                .unwrap_or((PortDirection::Inout, None));

            let disc_name = discipline.unwrap_or_else(|| "electrical".into());

            // Look up discipline to get natures
            let (potential, flow) = if let Some(disc) = self.disciplines.get_discipline(&disc_name)
            {
                (
                    disc.potential.as_ref().map(|s| SmolStr::from(s.as_str())),
                    disc.flow.as_ref().map(|s| SmolStr::from(s.as_str())),
                )
            } else {
                (Some("Voltage".into()), Some("Current".into()))
            };

            analyzed.ports.push(AnalyzedPort {
                name: port_name.clone(),
                direction,
                discipline: disc_name.clone(),
                nature_potential: potential,
                nature_flow: flow,
            });

            self.define_symbol(Symbol {
                name: port_name.clone(),
                kind: SymbolKind::Port,
                value_type: ValueType::NatureAccess,
                span: Span::new(crate::source::SourceId::new(0), 0, 0),
                attrs: SymbolAttrs {
                    direction: Some(direction),
                    discipline: Some(disc_name),
                    ..Default::default()
                },
            })?;
        }

        // Phase 5: Define internal and ground nodes (nets that aren't ports)
        let mut internal_node_idx = 0usize;
        for net in &module.nets {
            let discipline = net.discipline.clone();
            for name in &net.names {
                // Skip if already defined as a port
                if self.symbols.lookup_local(name).is_some() {
                    continue;
                }

                if net.is_ground {
                    // Ground nets reference the global reference node and
                    // must not consume an internal node slot.
                    self.define_symbol(Symbol {
                        name: name.clone(),
                        kind: SymbolKind::Node,
                        value_type: ValueType::NatureAccess,
                        span: net.span,
                        attrs: SymbolAttrs {
                            discipline: Some(discipline.clone()),
                            is_ground: true,
                            ..Default::default()
                        },
                    })?;
                    analyzed.ground_nodes.push(name.clone());
                    continue;
                }

                // Define as internal node
                self.define_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Node,
                    value_type: ValueType::NatureAccess,
                    span: net.span,
                    attrs: SymbolAttrs {
                        discipline: Some(discipline.clone()),
                        is_internal: true,
                        internal_node_index: Some(internal_node_idx),
                        ..Default::default()
                    },
                })?;

                // Add to analyzed internal nodes
                analyzed.internal_nodes.push(AnalyzedInternalNode {
                    name: name.clone(),
                    discipline: discipline.clone(),
                    index: internal_node_idx,
                });
                internal_node_idx += 1;
            }
        }

        // Phase 6: Named branch declarations
        for branch in &module.branches {
            self.validate_node(&branch.pos, branch.span)?;
            if !branch.neg.is_empty() {
                self.validate_node(&branch.neg, branch.span)?;
            }
            let discipline = self
                .symbols
                .lookup(&branch.pos)
                .and_then(|s| s.attrs.discipline.clone())
                .unwrap_or_else(|| "electrical".into());

            analyzed.branches.push(AnalyzedBranch {
                name: branch.name.clone(),
                pos_node: branch.pos.clone(),
                neg_node: branch.neg.clone(),
                discipline,
            });

            self.define_symbol(Symbol {
                name: branch.name.clone(),
                kind: SymbolKind::Branch,
                value_type: ValueType::NatureAccess,
                span: branch.span,
                attrs: Default::default(),
            })?;
        }

        // Phase 7: Analyze parameters (defaults may reference earlier ones)
        let param_names: std::collections::HashSet<SmolStr> =
            module.parameters.iter().map(|p| p.name.clone()).collect();
        for param in &module.parameters {
            let value_type = match param.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            // A default that references other parameters must stay
            // symbolic: instance overrides of those parameters change it,
            // so it is evaluated per instance at setup time.
            let default = if param
                .default
                .as_ref()
                .is_some_and(|e| Self::references_identifiers(e, &param_names))
            {
                None
            } else {
                param.default.as_ref().and_then(|e| self.eval_const(e))
            };

            // Parse parameter range if present
            let range = param
                .range
                .as_ref()
                .map(|r| self.parse_range(r, &param_names));

            // Validate default against range
            if let (Some(default_val), Some(range_constraint)) = (default, &range)
                && !range_constraint.contains(default_val)
            {
                self.record_error_at(
                    SemanticErrorKind::ParameterOutOfRange {
                        name: param.name.clone(),
                        value: default_val,
                        range: format!("{}", range_constraint),
                    },
                    param.span,
                );
            }

            if let Some(value) = default {
                self.param_consts.insert(param.name.clone(), value);
            }

            analyzed.parameters.push(AnalyzedParameter {
                name: param.name.clone(),
                param_type: param.param_type,
                value_type,
                default,
                default_expr: param.default.clone(),
                range: range.clone(),
            });

            self.define_symbol(Symbol {
                name: param.name.clone(),
                kind: SymbolKind::Parameter,
                value_type,
                span: param.span,
                attrs: SymbolAttrs {
                    range,
                    ..Default::default()
                },
            })?;
        }

        // Phase 7b: Parameter aliases (aliasparam). The target must be a
        // declared parameter; the alias name must not collide with any
        // other declaration. The alias enters the symbol table only to
        // reserve its name - it is not a parameter, so the module body
        // cannot reference it and it gets no storage or default.
        for decl in &module.aliasparams {
            let Some(target) = analyzed
                .parameters
                .iter()
                .position(|p| p.name == decl.target)
            else {
                self.record_error_at(
                    SemanticErrorKind::UndeclaredSymbol {
                        name: decl.target.clone(),
                    },
                    decl.span,
                );
                continue;
            };

            self.define_symbol(Symbol {
                name: decl.alias.clone(),
                kind: SymbolKind::ParamAlias,
                value_type: analyzed.parameters[target].value_type,
                span: decl.span,
                attrs: Default::default(),
            })?;

            analyzed.param_aliases.push(AnalyzedParamAlias {
                alias: decl.alias.clone(),
                target,
            });
        }

        // Pre-pass for Phase 8: seed the constant environments with
        // localparam values so array bounds may reference them (their full
        // lowering to computed variables happens in Phase 9)
        for localparam in &module.localparams {
            if let Some(default) = &localparam.default {
                if let Some(value) = self.eval_const(default) {
                    self.param_consts.insert(localparam.name.clone(), value);
                }
                if let Some(value) = self.eval_const_invariant(default) {
                    self.invariant_consts.insert(localparam.name.clone(), value);
                }
            }
        }

        // Phase 8: Analyze variables
        for var_decl in &module.variables {
            let value_type = match var_decl.var_type {
                VarType::Real => ValueType::Real,
                VarType::Integer => ValueType::Integer,
                VarType::String => ValueType::String,
            };

            for item in &var_decl.items {
                if !item.dimensions.is_empty() {
                    let name = item.name.clone();
                    if let Some(layout) =
                        self.register_array_variable(item, var_decl.var_type, &name, &mut analyzed)
                    {
                        analyzed.arrays.insert(name.clone(), layout.clone());
                        self.arrays.insert(name.clone(), layout);
                        self.define_symbol(Symbol {
                            name,
                            kind: SymbolKind::Variable,
                            value_type,
                            span: var_decl.span,
                            attrs: Default::default(),
                        })?;
                    }
                    continue;
                }

                analyzed.variables.push(AnalyzedVariable {
                    name: item.name.clone(),
                    var_type: var_decl.var_type,
                    value_type,
                    is_state: false,
                });

                self.define_symbol(Symbol {
                    name: item.name.clone(),
                    kind: SymbolKind::Variable,
                    value_type,
                    span: var_decl.span,
                    attrs: Default::default(),
                })?;
            }
        }

        // Phase 9: Lower localparams to computed variables. Their values may
        // depend on parameters, so they are evaluated at runtime before any
        // analog-block assignment, in declaration order.
        for localparam in &module.localparams {
            let value_type = match localparam.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            let Some(default) = &localparam.default else {
                self.record_error_at(
                    SemanticErrorKind::MissingAttribute(format!(
                        "localparam '{}' requires a value",
                        localparam.name
                    )),
                    localparam.span,
                );
                continue;
            };

            if let Some(value) = self.eval_const(default) {
                self.param_consts.insert(localparam.name.clone(), value);
            }
            // A localparam derived purely from literals (and other
            // invariant localparams) cannot vary per instance, so it may
            // participate in loop unrolling and other code folding
            if let Some(value) = self.eval_const_invariant(default) {
                self.invariant_consts.insert(localparam.name.clone(), value);
            }

            let var_index = analyzed.variables.len();
            analyzed.variables.push(AnalyzedVariable {
                name: localparam.name.clone(),
                var_type: match localparam.param_type {
                    ParamType::Real => VarType::Real,
                    ParamType::Integer => VarType::Integer,
                    ParamType::String => VarType::String,
                },
                value_type,
                is_state: false,
            });

            self.define_symbol(Symbol {
                name: localparam.name.clone(),
                kind: SymbolKind::Parameter,
                value_type,
                span: localparam.span,
                attrs: Default::default(),
            })?;

            let expression = self.lower_expression(default)?;
            let expr_type = self.infer_type(&expression)?;
            statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                target: localparam.name.clone(),
                var_index,
                index: None,
                expression,
                expr_type,
                span: localparam.span,
            }));
        }

        // Phase 10: Module-level variable initializers run before the
        // analog block, in declaration order.
        for var_decl in &module.variables {
            for item in &var_decl.items {
                let Some(init) = &item.init else { continue };

                if let Some(layout) = self.arrays.get(&item.name).cloned() {
                    // Array initializer: '{e0, e1, ...} fills the elements
                    // in declaration order
                    let Expression::ArrayLiteral(lit) = init else {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: "array literal".to_string(),
                                found: "scalar expression".to_string(),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    };
                    if lit.elements.len() != layout.len {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: format!("{} elements", layout.len),
                                found: format!("{} elements", lit.elements.len()),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    }
                    for (offset, element) in lit.elements.iter().enumerate() {
                        let var_index = layout.base + offset;
                        let expression = self.lower_expression(element)?;
                        let expr_type = self.infer_type(&expression)?;
                        statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                            target: analyzed.variables[var_index].name.clone(),
                            var_index,
                            index: None,
                            expression,
                            expr_type,
                            span: item.span,
                        }));
                    }
                    continue;
                }

                let var_index = analyzed
                    .variables
                    .iter()
                    .position(|v| v.name == item.name)
                    .expect("variable registered above");
                let expression = self.lower_expression(init)?;
                let expr_type = self.infer_type(&expression)?;
                statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                    target: item.name.clone(),
                    var_index,
                    index: None,
                    expression,
                    expr_type,
                    span: item.span,
                }));
            }
        }

        // Phase 11: analog initial runs before the main analog block
        if let Some(block) = &module.analog_initial {
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut statements)?;
            }
        }

        // Phase 12: Analyze analog block
        if let Some(block) = &module.analog_block {
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut statements)?;
            }
        }

        analyzed.statements = statements;

        // Surface every recorded diagnostic instead of silently succeeding
        if !self.errors.is_empty() {
            let errors = std::mem::take(&mut self.errors)
                .into_iter()
                .map(CompileError::Semantic)
                .collect();
            return Err(CompileError::multiple(errors));
        }

        analyzed.symbol_table = self.symbols.clone();
        Ok(analyzed)
    }

    fn define_symbol(&mut self, symbol: Symbol) -> CompileResult<()> {
        if let Err(existing) = self.symbols.define(symbol.clone()) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::DuplicateSymbol {
                    name: symbol.name,
                    first_defined: existing.span,
                },
                symbol.span,
            )));
        }
        Ok(())
    }

    /// Hard cap on array storage so a typo in a bound cannot silently
    /// allocate gigabytes of per-instance state
    const MAX_ARRAY_ELEMENTS: usize = 65_536;

    /// Register a 1-D array variable: its elements become contiguous
    /// `name[k]` slots in the variable storage (named after `storage_name`,
    /// which differs from the declared name for hoisted block locals).
    /// Bounds must fold to instance-invariant constants
    /// (parameter-dependent shapes would make the storage layout vary per
    /// instance).
    fn register_array_variable(
        &mut self,
        item: &VariableItem,
        var_type: VarType,
        storage_name: &SmolStr,
        analyzed: &mut AnalyzedModule,
    ) -> Option<AnalyzedArray> {
        if item.dimensions.len() != 1 {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "multi-dimensional array '{}' is not supported",
                    item.name
                )),
                item.span,
            );
            return None;
        }
        let dim = &item.dimensions[0];
        let (Some(start), Some(end)) = (
            self.eval_const_invariant(&dim.start),
            self.eval_const_invariant(&dim.end),
        ) else {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' bounds must be compile-time constants",
                    item.name
                )),
                dim.span,
            );
            return None;
        };
        let (start, end) = (start.round() as i64, end.round() as i64);
        // The LRM writes ranges [lo:hi]; accept either order
        let (lower, upper) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        let len = (upper - lower + 1) as usize;
        if len > Self::MAX_ARRAY_ELEMENTS {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' has {len} elements (limit {})",
                    item.name,
                    Self::MAX_ARRAY_ELEMENTS
                )),
                dim.span,
            );
            return None;
        }
        let value_type = match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        };
        let base = analyzed.variables.len();
        for k in lower..=upper {
            analyzed.variables.push(AnalyzedVariable {
                name: SmolStr::from(format!("{storage_name}[{k}]")),
                var_type,
                value_type,
                is_state: false,
            });
        }
        Some(AnalyzedArray { base, lower, len })
    }

    /// Fold the active guard stack into a single condition expression
    fn current_guard(&self) -> Option<Expression> {
        let mut guards = self.guard_stack.iter();
        let first = guards.next()?.clone();
        Some(guards.fold(first, |acc, g| {
            let span = g.span();
            Expression::Binary(BinaryExpr {
                op: BinaryOp::And,
                left: Box::new(acc),
                right: Box::new(g.clone()),
                span,
            })
        }))
    }

    /// Build `guard ? value : fallback` under the active guard (or `value`
    /// when unguarded)
    fn apply_guard(&self, value: Expression, fallback: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => {
                let span = value.span();
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(guard),
                    then_expr: Box::new(value),
                    else_expr: Box::new(fallback),
                    span,
                })
            }
            None => value,
        }
    }

    fn not_expr(expr: Expression) -> Expression {
        let span = expr.span();
        Expression::Unary(UnaryExpr {
            op: UnaryOp::Not,
            operand: Box::new(expr),
            span,
        })
    }

    fn binary_expr(op: BinaryOp, left: Expression, right: Expression) -> Expression {
        let span = left.span();
        Expression::Binary(BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span,
        })
    }

    fn number_expr(value: f64, span: Span) -> Expression {
        Expression::Number(NumberLit {
            value,
            raw: SmolStr::default(),
            span,
        })
    }

    /// Analyze a statement, lowering control flow into guarded dataflow.
    ///
    /// Assignments and contributions inside conditionals become conditional
    /// expressions (`guard ? value : previous`), so the recorded flat lists
    /// preserve branch semantics exactly. Loops whose bounds do not fold to
    /// compile-time constants lower to runtime loop statements.
    fn analyze_statement(
        &mut self,
        stmt: &AnalogStatement,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Contribution(contrib) => {
                self.analyze_contribution(contrib, module)?;
            }
            AnalogStatement::Assignment(assign) => {
                self.analyze_assignment(assign, module, sink)?;
            }
            AnalogStatement::Block(block) => {
                self.symbols.enter_scope();
                self.subst_stack.push(HashMap::new());

                // Hoist block-local variables to module scope under unique
                // names; expressions are rewritten through the subst frame.
                for var_decl in &block.variables {
                    let value_type = match var_decl.var_type {
                        VarType::Real => ValueType::Real,
                        VarType::Integer => ValueType::Integer,
                        VarType::String => ValueType::String,
                    };
                    for item in &var_decl.items {
                        self.local_counter += 1;
                        let hoisted: SmolStr =
                            if module.variables.iter().any(|v| v.name == item.name)
                                || self.arrays.contains_key(&item.name)
                            {
                                format!("{}__blk{}", item.name, self.local_counter).into()
                            } else {
                                item.name.clone()
                            };

                        if !item.dimensions.is_empty() {
                            if let Some(layout) = self.register_array_variable(
                                item,
                                var_decl.var_type,
                                &hoisted,
                                module,
                            ) {
                                module.arrays.insert(hoisted.clone(), layout.clone());
                                self.arrays.insert(hoisted.clone(), layout);
                                self.define_symbol(Symbol {
                                    name: hoisted.clone(),
                                    kind: SymbolKind::Variable,
                                    value_type,
                                    span: item.span,
                                    attrs: Default::default(),
                                })?;
                                self.subst_stack.last_mut().unwrap().insert(
                                    item.name.clone(),
                                    Expression::Identifier(Identifier {
                                        name: hoisted.clone(),
                                        span: item.span,
                                    }),
                                );
                                if item.init.is_some() {
                                    self.record_error_at(
                                        SemanticErrorKind::UnsupportedFeature(format!(
                                            "initializer on block-local array '{}'",
                                            item.name
                                        )),
                                        item.span,
                                    );
                                }
                            }
                            continue;
                        }

                        module.variables.push(AnalyzedVariable {
                            name: hoisted.clone(),
                            var_type: var_decl.var_type,
                            value_type,
                            is_state: false,
                        });
                        self.define_symbol(Symbol {
                            name: hoisted.clone(),
                            kind: SymbolKind::Variable,
                            value_type,
                            span: item.span,
                            attrs: Default::default(),
                        })?;
                        self.subst_stack.last_mut().unwrap().insert(
                            item.name.clone(),
                            Expression::Identifier(Identifier {
                                name: hoisted.clone(),
                                span: item.span,
                            }),
                        );

                        if let Some(init) = &item.init {
                            let expression = self.lower_expression(init)?;
                            let expression =
                                self.apply_guard(expression, Self::number_expr(0.0, item.span));
                            let expr_type = self.infer_type(&expression)?;
                            let var_index = module
                                .variables
                                .iter()
                                .position(|v| v.name == hoisted)
                                .expect("just registered");
                            sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                                target: hoisted.clone(),
                                var_index,
                                index: None,
                                expression,
                                expr_type,
                                span: item.span,
                            }));
                        }
                    }
                }

                for s in &block.statements {
                    self.analyze_statement(s, module, sink)?;
                }

                self.subst_stack.pop();
                self.symbols.exit_scope();
            }
            AnalogStatement::Conditional(cond) => {
                let condition = self.lower_expression(&cond.condition)?;
                let cond_type = self.infer_type(&condition)?;
                if !cond_type.is_condition() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidCondition {
                            found: cond_type.to_string(),
                        },
                        cond.span,
                    );
                }

                // Snapshot the condition into a temporary BEFORE either
                // branch runs: the then-branch may assign variables the
                // condition reads, and a re-evaluated else-guard would then
                // see the mutated state and fire as well.
                let condition = self.snapshot_guard(condition, cond.span, module, sink)?;

                self.guard_stack.push(condition.clone());
                self.analyze_statement(&cond.then_branch, module, sink)?;
                self.guard_stack.pop();

                if let Some(else_branch) = &cond.else_branch {
                    self.guard_stack.push(Self::not_expr(condition));
                    self.analyze_statement(else_branch, module, sink)?;
                    self.guard_stack.pop();
                }
            }
            AnalogStatement::Case(case_stmt) => {
                // The selector and ALL match comparisons are evaluated
                // before any arm executes (LRM case semantics); snapshot
                // them so arm bodies cannot perturb later guards.
                let selector = self.lower_expression(&case_stmt.expr)?;
                let selector = self.snapshot_guard(selector, case_stmt.span, module, sink)?;

                let mut item_guards: Vec<Option<Expression>> = Vec::new();
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered = self.lower_expression(m)?;
                        let eq = Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered);
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                    }
                    let snapshotted = match item_match {
                        Some(expr) => {
                            Some(self.snapshot_guard(expr, case_stmt.span, module, sink)?)
                        }
                        None => None,
                    };
                    item_guards.push(snapshotted);
                }

                // OR of all guards matched so far (case items are priority
                // ordered: the first matching item wins)
                let mut prior_match: Option<Expression> = None;

                for (item, item_match) in case_stmt.items.iter().zip(item_guards) {
                    let Some(item_match) = item_match else {
                        continue;
                    };

                    let guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };

                    self.guard_stack.push(guard);
                    self.analyze_statement(&item.statement, module, sink)?;
                    self.guard_stack.pop();

                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }

                if let Some(default) = &case_stmt.default {
                    match prior_match {
                        Some(prior) => {
                            self.guard_stack.push(Self::not_expr(prior));
                            self.analyze_statement(default, module, sink)?;
                            self.guard_stack.pop();
                        }
                        None => self.analyze_statement(default, module, sink)?,
                    }
                }
            }
            AnalogStatement::For(for_stmt) => {
                self.analyze_for(for_stmt, module, sink)?;
            }
            AnalogStatement::Repeat(repeat) => {
                let count_expr = self.lower_expression(&repeat.count)?;
                match self.eval_const_invariant(&count_expr) {
                    Some(count) if (count as usize) <= Self::MAX_UNROLL_ITERATIONS => {
                        for _ in 0..(count as usize) {
                            self.analyze_statement(&repeat.body, module, sink)?;
                        }
                    }
                    Some(count) => {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "repeat count {count} exceeds the unroll limit"
                            )),
                            repeat.span,
                        )));
                    }
                    // Runtime-dependent count: lower to a runtime loop with
                    // a synthesized counter
                    None => self.lower_runtime_repeat(repeat, count_expr, module, sink)?,
                }
            }
            AnalogStatement::While(while_stmt) => {
                let condition = self.lower_expression(&while_stmt.condition)?;
                match self.eval_const_invariant(&condition) {
                    Some(0.0) => {} // statically dead loop
                    Some(_) => {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "while loop condition is constant-true (infinite loop)".into(),
                            ),
                            while_stmt.span,
                        )));
                    }
                    None => {
                        // Runtime condition: lower to a runtime loop
                        let cond_type = self.infer_type(&condition)?;
                        if !cond_type.is_condition() {
                            self.record_error_at(
                                SemanticErrorKind::InvalidCondition {
                                    found: cond_type.to_string(),
                                },
                                while_stmt.span,
                            );
                        }
                        let condition = self.fold_guard_into_condition(condition);
                        let body = self.analyze_loop_body(&while_stmt.body, None, module)?;
                        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
                            condition,
                            body,
                            span: while_stmt.span,
                        }));
                    }
                }
            }
            AnalogStatement::EventControl(event_ctrl) => {
                match self.event_guard(&event_ctrl.event)? {
                    EventLowering::Guard(guard) => {
                        // Snapshot: the body must not perturb its own guard
                        let guard = self.snapshot_guard(guard, event_ctrl.span, module, sink)?;
                        self.guard_stack.push(guard);
                        self.analyze_statement(&event_ctrl.statement, module, sink)?;
                        self.guard_stack.pop();
                    }
                    EventLowering::Always => {
                        self.analyze_statement(&event_ctrl.statement, module, sink)?;
                    }
                    EventLowering::Never => {} // e.g. final_step bodies
                }
            }
            AnalogStatement::IndirectContribution(stmt) => {
                self.analyze_indirect_contribution(stmt, module)?;
            }
            // $bound_step and $discontinuity steer the transient stepper
            // through hidden per-evaluation variables; other system tasks
            // ($strobe, $display, ...) have no effect on the device
            // equations
            AnalogStatement::Call(call) => match call.name.as_str() {
                "$bound_step" => self.analyze_bound_step(call, module, sink)?,
                "$discontinuity" => self.analyze_discontinuity(call, module, sink)?,
                _ => {}
            },
            AnalogStatement::Disable(_) | AnalogStatement::Null(_) => {}
        }
        Ok(())
    }

    const MAX_UNROLL_ITERATIONS: usize = 65536;

    /// Analyze a for loop: statically unroll when the bounds fold to
    /// compile-time constants, otherwise lower to a runtime loop
    fn analyze_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let loop_var = self.resolve_substituted_name(&for_stmt.var);
        if self.symbols.lookup(&loop_var).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol {
                    name: loop_var.clone(),
                },
                for_stmt.span,
            )));
        }

        if *for_stmt.update.target_name() != for_stmt.var {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(
                    "for-loop update must assign the loop variable".into(),
                ),
                for_stmt.span,
            )));
        }

        // Probe whether init, condition, and update fold to constants;
        // only then is static unrolling sound.
        let init = self.lower_expression(&for_stmt.init)?;
        let init_value = self.eval_const_invariant(&init);
        let static_unrollable = if let Some(value) = init_value {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));
            let cond_probe = self
                .lower_expression(&for_stmt.condition)
                .ok()
                .and_then(|c| self.eval_const_invariant(&c));
            let update_probe = self
                .lower_expression(&for_stmt.update.value)
                .ok()
                .and_then(|u| self.eval_const_invariant(&u));
            self.subst_stack.pop();
            cond_probe.is_some() && update_probe.is_some()
        } else {
            false
        };

        if static_unrollable {
            self.unroll_for(for_stmt, init_value.expect("checked"), module, sink)
        } else {
            self.lower_runtime_for(for_stmt, module, sink)
        }
    }

    /// Statically unroll a for loop with compile-time-constant bounds
    fn unroll_for(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            // Bind the loop variable to its current constant value
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = self.lower_expression(&for_stmt.condition)?;
            let Some(cond_value) = self.eval_const_invariant(&condition) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop condition stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            if cond_value == 0.0 {
                self.subst_stack.pop();
                break;
            }

            self.analyze_statement(&for_stmt.body, module, sink)?;

            let update = self.lower_expression(&for_stmt.update.value)?;
            let Some(next_value) = self.eval_const_invariant(&update) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop update stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            self.subst_stack.pop();

            value = next_value;
            iterations += 1;
            if iterations > Self::MAX_UNROLL_ITERATIONS {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "for-loop exceeds the unroll limit of {} iterations",
                        Self::MAX_UNROLL_ITERATIONS
                    )),
                    for_stmt.span,
                )));
            }
        }

        Ok(())
    }

    /// Materialize a guard expression into a synthesized variable assigned
    /// once at this point in the statement stream.
    ///
    /// Guards must capture the state at evaluation time: branch bodies may
    /// assign variables their own guard reads, and re-evaluating the raw
    /// expression inside each guarded assignment would observe the
    /// mutation (e.g. `if (x == UNSET) x = a; else x = x + b;` must never
    /// run both arms). Trivial guards (literals, identifiers) pass through.
    fn snapshot_guard(
        &mut self,
        condition: Expression,
        span: Span,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        // Identifiers and literals are stable by construction
        if matches!(condition, Expression::Identifier(_) | Expression::Number(_)) {
            return Ok(condition);
        }

        self.local_counter += 1;
        let name: SmolStr = format!("__guard{}", self.local_counter).into();

        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.define_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Variable,
            value_type: ValueType::Real,
            span,
            attrs: Default::default(),
        })?;

        // The snapshot assignment itself is unconditional: guard
        // expressions are pure, and enclosing guards already gate every
        // consumer of this variable.
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: name.clone(),
            var_index,
            index: None,
            expression: condition,
            expr_type: ValueType::Real,
            span,
        }));

        Ok(Expression::Identifier(Identifier { name, span }))
    }

    /// AND the enclosing guard into a runtime loop condition so a guarded
    /// loop runs zero iterations when its guard is inactive
    fn fold_guard_into_condition(&self, condition: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => Self::binary_expr(BinaryOp::And, guard, condition),
            None => condition,
        }
    }

    /// Analyze loop-body statements into a fresh sink, tracking the
    /// runtime-loop nesting depth (contributions inside are rejected).
    /// An optional trailing statement (the for-loop update) is analyzed
    /// after the body.
    fn analyze_loop_body(
        &mut self,
        body: &AnalogStatement,
        trailing: Option<&AnalogStatement>,
        module: &mut AnalyzedModule,
    ) -> CompileResult<Vec<AnalyzedStatement>> {
        let mut statements = Vec::new();
        self.runtime_loop_depth += 1;
        let result = self
            .analyze_statement(body, module, &mut statements)
            .and_then(|()| match trailing {
                Some(stmt) => self.analyze_statement(stmt, module, &mut statements),
                None => Ok(()),
            });
        self.runtime_loop_depth -= 1;
        result?;
        Ok(statements)
    }

    /// Lower a for loop with runtime-dependent bounds (e.g. iterating to a
    /// parameter like BSIM4's nf finger count) into a runtime loop
    fn lower_runtime_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // Loop variable initialization through the normal guarded path
        let init_stmt = AssignmentStmt {
            target: LValue::Variable {
                name: for_stmt.var.clone(),
                span: for_stmt.span,
            },
            value: for_stmt.init.clone(),
            span: for_stmt.span,
        };
        self.analyze_assignment(&init_stmt, module, sink)?;

        // Condition re-evaluated each iteration, with the enclosing guard
        // folded in
        let condition = self.lower_expression(&for_stmt.condition)?;
        let cond_type = self.infer_type(&condition)?;
        if !cond_type.is_condition() {
            self.record_error_at(
                SemanticErrorKind::InvalidCondition {
                    found: cond_type.to_string(),
                },
                for_stmt.span,
            );
        }
        let condition = self.fold_guard_into_condition(condition);

        // Body, then the update assignment, inside the loop sink
        let update_stmt = AnalogStatement::Assignment((*for_stmt.update).clone());
        let body = self.analyze_loop_body(&for_stmt.body, Some(&update_stmt), module)?;

        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            body,
            span: for_stmt.span,
        }));
        Ok(())
    }

    /// Lower a repeat loop with a runtime-dependent count into a runtime
    /// loop over a synthesized counter
    fn lower_runtime_repeat(
        &mut self,
        repeat: &RepeatStmt,
        count_expr: Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let span = repeat.span;

        // Synthesize counter and count-snapshot variables (the LRM
        // evaluates the repeat count once, before iterating)
        self.local_counter += 1;
        let idx_name: SmolStr = format!("__repeat_i{}", self.local_counter).into();
        let cnt_name: SmolStr = format!("__repeat_n{}", self.local_counter).into();
        let mut register = |this: &mut Self, name: &SmolStr| -> CompileResult<usize> {
            let var_index = module.variables.len();
            module.variables.push(AnalyzedVariable {
                name: name.clone(),
                var_type: VarType::Real,
                value_type: ValueType::Real,
                is_state: false,
            });
            this.define_symbol(Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                value_type: ValueType::Real,
                span,
                attrs: Default::default(),
            })?;
            Ok(var_index)
        };
        let idx_index = register(self, &idx_name)?;
        let cnt_index = register(self, &cnt_name)?;

        let ident = |name: &SmolStr| {
            Expression::Identifier(Identifier {
                name: name.clone(),
                span,
            })
        };

        // cnt = <count>; idx = 0;
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: cnt_name.clone(),
            var_index: cnt_index,
            index: None,
            expression: count_expr,
            expr_type: ValueType::Real,
            span,
        }));
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::number_expr(0.0, span),
            expr_type: ValueType::Real,
            span,
        }));

        // while (guard && idx < cnt) { body; idx = idx + 1; }
        let condition = self.fold_guard_into_condition(Self::binary_expr(
            BinaryOp::Lt,
            ident(&idx_name),
            ident(&cnt_name),
        ));

        let mut body = self.analyze_loop_body(&repeat.body, None, module)?;
        body.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::binary_expr(
                BinaryOp::Add,
                ident(&idx_name),
                Self::number_expr(1.0, span),
            ),
            expr_type: ValueType::Real,
            span,
        }));

        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            body,
            span,
        }));
        Ok(())
    }

    /// Lower an event expression into a runtime guard
    fn event_guard(&mut self, event: &EventExpr) -> CompileResult<EventLowering> {
        Ok(match event {
            EventExpr::InitialStep { span } => {
                // Approximation: initial_step is active during static
                // (DC / IC) analyses. Assignments latched there persist
                // into the following transient.
                let dc = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "static".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(dc)
            }
            EventExpr::FinalStep { .. } => EventLowering::Never,
            EventExpr::Cross {
                signal,
                direction,
                span,
                ..
            } => {
                let signal = self.lower_expression(signal)?;
                let dir_value = match direction {
                    Some(CrossDirection::Rising) => 1.0,
                    Some(CrossDirection::Falling) => -1.0,
                    Some(CrossDirection::Both) | None => 0.0,
                };
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(dir_value, *span)],
                    span: *span,
                }))
            }
            EventExpr::Posedge { signal, span } => {
                let signal = self.lower_expression(signal)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Negedge { signal, span } => {
                let signal = self.lower_expression(signal)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(-1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Above { signal, span } => {
                let signal = self.lower_expression(signal)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "above".into(),
                    args: vec![signal, Self::number_expr(0.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Timer {
                start,
                period,
                span,
            } => {
                let mut args = vec![self.lower_expression(start)?];
                if let Some(period) = period {
                    args.push(self.lower_expression(period)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "timer".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Or { left, right, .. } => {
                let left = self.event_guard(left)?;
                let right = self.event_guard(right)?;
                match (left, right) {
                    (EventLowering::Guard(l), EventLowering::Guard(r)) => {
                        EventLowering::Guard(Self::binary_expr(BinaryOp::Or, l, r))
                    }
                    (EventLowering::Always, _) | (_, EventLowering::Always) => {
                        EventLowering::Always
                    }
                    (EventLowering::Never, other) | (other, EventLowering::Never) => other,
                }
            }
        })
    }

    fn analyze_contribution(
        &mut self,
        contrib: &ContributionStmt,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        // Contributions accumulate into fixed stamp programs; a contribution
        // executed a runtime-dependent number of times is not representable
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                contrib.span,
            )));
        }

        let (branch_name, is_current) =
            self.resolve_contribution_target(&contrib.target, module, contrib.span)?;

        let expression = self.lower_expression(&contrib.value)?;
        let expr_type = self.infer_type(&expression)?;
        if !expr_type.is_numeric() && expr_type != ValueType::Unknown {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "numeric".to_string(),
                    found: expr_type.to_string(),
                    context: "contribution expression".to_string(),
                },
                contrib.span,
            );
        }

        // A guarded contribution contributes zero when inactive
        let expression = self.apply_guard(expression, Self::number_expr(0.0, contrib.span));

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            is_current,
            indirect: false,
            expression,
            expr_type,
            span: contrib.span,
        });

        Ok(())
    }

    /// Analyze an indirect contribution `V(x): lhs == rhs`: the target
    /// branch carries an unknown source whose value the solver picks so
    /// the constraint holds. The recorded expression is the residual
    /// `lhs - rhs`; under an inactive guard it degrades to `I(branch)`,
    /// pinning the unknown to zero so the branch opens.
    fn analyze_indirect_contribution(
        &mut self,
        stmt: &IndirectContributionStmt,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                stmt.span,
            )));
        }

        let (branch_name, is_current) =
            self.resolve_contribution_target(&stmt.branch, module, stmt.span)?;

        let lhs = self.lower_expression(&stmt.lhs)?;
        let rhs = self.lower_expression(&stmt.rhs)?;
        for (side, expr) in [("left", &lhs), ("right", &rhs)] {
            let ty = self.infer_type(expr)?;
            if !ty.is_numeric() && ty != ValueType::Unknown && ty != ValueType::NatureAccess {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric".to_string(),
                        found: ty.to_string(),
                        context: format!("{side} side of indirect contribution"),
                    },
                    stmt.span,
                );
            }
        }

        let residual = Self::binary_expr(BinaryOp::Sub, lhs, rhs);

        // Guard fallback: the constraint is replaced by I(branch) = 0
        let fallback = Expression::BranchAccess(match &stmt.branch {
            BranchAccess::Nodes { pos, neg, span, .. } => BranchAccess::Nodes {
                access: "I".into(),
                pos: pos.clone(),
                neg: neg.clone(),
                span: *span,
            },
            BranchAccess::Branch { name, span, .. } => BranchAccess::Branch {
                access: "I".into(),
                name: name.clone(),
                span: *span,
            },
        });
        let expression = self.apply_guard(residual, fallback);

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            is_current,
            indirect: true,
            expression,
            expr_type: ValueType::Real,
            span: stmt.span,
        });

        Ok(())
    }

    /// `$bound_step(max_dt)`: cap the next transient step while the call
    /// is active. Lowers to `$bound_step = min($bound_step, max_dt)` on a
    /// hidden variable reset to +inf at the top of every evaluation, so
    /// multiple calls and guards compose naturally.
    fn analyze_bound_step(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let Some(arg) = call.args.first() else {
            self.record_error_at(
                SemanticErrorKind::MissingAttribute(
                    "$bound_step requires a maximum-step argument".into(),
                ),
                call.span,
            );
            return Ok(());
        };
        let var_index =
            self.ensure_task_variable("$bound_step", f64::INFINITY, module, sink, call.span);
        let bound = self.lower_expression(arg)?;
        let current = Expression::Identifier(Identifier {
            name: "$bound_step".into(),
            span: call.span,
        });
        let min = Expression::Call(CallExpr {
            name: "min".into(),
            args: vec![current.clone(), bound],
            span: call.span,
        });
        let expression = self.apply_guard(min, current);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: "$bound_step".into(),
            var_index,
            index: None,
            expression,
            expr_type: ValueType::Real,
            span: call.span,
        }));
        Ok(())
    }

    /// `$discontinuity(degree)`: flag a topology/regime change so the
    /// transient stepper places a breakpoint. Lowers to a hidden flag
    /// reset to 0 every evaluation and set to 1 while the call is active.
    fn analyze_discontinuity(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let var_index = self.ensure_task_variable("$discontinuity", 0.0, module, sink, call.span);
        let current = Expression::Identifier(Identifier {
            name: "$discontinuity".into(),
            span: call.span,
        });
        let expression = self.apply_guard(Self::number_expr(1.0, call.span), current);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: "$discontinuity".into(),
            var_index,
            index: None,
            expression,
            expr_type: ValueType::Real,
            span: call.span,
        }));
        Ok(())
    }

    /// Register a hidden system-task variable on first use and emit its
    /// unguarded per-evaluation reset (the `$`-prefixed name cannot
    /// collide with user identifiers)
    fn ensure_task_variable(
        &mut self,
        name: &str,
        reset: f64,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
        span: Span,
    ) -> usize {
        if let Some(&idx) = self.task_vars.get(name) {
            return idx;
        }
        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.into(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.task_vars.insert(name.into(), var_index);
        // The reset runs unconditionally: every evaluation starts neutral
        // and only active calls move the value
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: name.into(),
            var_index,
            index: None,
            expression: Self::number_expr(reset, span),
            expr_type: ValueType::Real,
            span,
        }));
        var_index
    }

    /// Resolve a contribution target (node pair or named branch) to the
    /// IR branch string and its flow/potential kind
    fn resolve_contribution_target(
        &mut self,
        target: &BranchAccess,
        module: &AnalyzedModule,
        span: Span,
    ) -> CompileResult<(SmolStr, bool)> {
        match target {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                let is_current = self.is_flow_access(access);

                // V(name)/I(name) where `name` is a declared branch resolves
                // through the branch table
                if neg.is_none()
                    && let Some(branch) = module.branches.iter().find(|b| b.name == *pos)
                {
                    let branch_str = if branch.neg_node.is_empty() {
                        branch.pos_node.to_string()
                    } else {
                        format!("{},{}", branch.pos_node, branch.neg_node)
                    };
                    Ok((SmolStr::from(branch_str), is_current))
                } else {
                    self.validate_node(pos, span)?;
                    if let Some(n) = neg {
                        self.validate_node(n, span)?;
                    }
                    // Format as "pos,neg" for IR parser compatibility
                    let branch = if neg.is_some() {
                        format!("{},{}", pos, neg.as_deref().unwrap())
                    } else {
                        pos.to_string()
                    };
                    Ok((branch.into(), is_current))
                }
            }
            BranchAccess::Branch { name, access, .. } => {
                let is_current = self.is_flow_access(access);
                match module.branches.iter().find(|b| b.name == *name) {
                    Some(branch) => {
                        let branch_str = if branch.neg_node.is_empty() {
                            branch.pos_node.to_string()
                        } else {
                            format!("{},{}", branch.pos_node, branch.neg_node)
                        };
                        Ok((SmolStr::from(branch_str), is_current))
                    }
                    None => Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidBranch(format!("undeclared branch '{}'", name)),
                        span,
                    ))),
                }
            }
        }
    }

    /// Whether the access function refers to a flow (current-like) quantity
    fn is_flow_access(&self, access: &str) -> bool {
        if access == "I" {
            return true;
        }
        self.disciplines.is_flow_access(access)
    }

    fn analyze_assignment(
        &mut self,
        assign: &AssignmentStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // `symbol_name` is the declared symbol checked for kind/type;
        // `target_name` is the storage slot (for const-index array elements
        // they differ: symbol `arr`, storage `arr[k]`)
        let (symbol_name, target_name, span, dyn_index) = match &assign.target {
            LValue::Variable { name, span } => {
                let resolved = self.resolve_substituted_name(name);
                if self.symbols.lookup(&resolved).is_none() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        *span,
                    )));
                }
                self.symbols.mark_used(&resolved);
                (resolved.clone(), resolved, *span, None)
            }
            LValue::ArrayAccess { name, index, span } => {
                let array_name = self.resolve_substituted_name(name);
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            name
                        )),
                        *span,
                    );
                    return Ok(());
                };
                self.symbols.mark_used(&array_name);
                let index = self.lower_expression(index)?;
                if let Some(k) = self.eval_const_invariant(&index) {
                    // Compile-time index: target the element slot directly
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, *span)?;
                    let elem = SmolStr::from(format!("{array_name}[{k}]"));
                    (array_name, elem, *span, None)
                } else {
                    (array_name.clone(), array_name, *span, Some(index))
                }
            }
        };

        // Assignments may only target variables
        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !matches!(sym.kind, SymbolKind::Variable | SymbolKind::LoopVar)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "variable".to_string(),
                    found: format!("{:?}", sym.kind).to_lowercase(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
            return Ok(());
        }

        let expression = self.lower_expression(&assign.value)?;
        let value_type = self.infer_type(&expression)?;

        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !value_type.can_coerce_to(&sym.value_type)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: sym.value_type.to_string(),
                    found: value_type.to_string(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
        }

        if let Some(index) = dyn_index {
            return self.push_indexed_assignment(
                target_name,
                index,
                expression,
                value_type,
                span,
                sink,
            );
        }

        // Find variable index; assignments to unknown storage are an error
        let Some(var_index) = module.variables.iter().position(|v| v.name == target_name) else {
            self.record_error_at(
                SemanticErrorKind::UndeclaredSymbol {
                    name: target_name.clone(),
                },
                span,
            );
            return Ok(());
        };

        // Under a guard, the variable keeps its previous value when the
        // guard is inactive
        let fallback = Expression::Identifier(Identifier {
            name: target_name.clone(),
            span,
        });
        let expression = self.apply_guard(expression, fallback);

        // Record the assignment for code generation
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: target_name,
            var_index,
            index: None,
            expression,
            expr_type: value_type,
            span,
        }));

        Ok(())
    }

    /// Record an array element assignment whose index is only known at
    /// runtime. Guards fall back to re-reading the same element, so an
    /// inactive guard leaves the array untouched.
    fn push_indexed_assignment(
        &mut self,
        array_name: SmolStr,
        index: Expression,
        expression: Expression,
        value_type: ValueType,
        span: Span,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let layout = self.arrays.get(&array_name).cloned().expect("checked");
        let fallback = Expression::ArrayAccess(ArrayAccessExpr {
            array: array_name.clone(),
            index: Box::new(index.clone()),
            span,
        });
        let expression = self.apply_guard(expression, fallback);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: array_name,
            var_index: layout.base,
            index: Some(index),
            expression,
            expr_type: value_type,
            span,
        }));
        Ok(())
    }

    /// Validate a compile-time array index against the declared bounds
    fn check_array_bounds(
        &self,
        name: &SmolStr,
        layout: &AnalyzedArray,
        k: i64,
        span: Span,
    ) -> CompileResult<()> {
        if k < layout.lower || k >= layout.lower + layout.len as i64 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::IndexOutOfBounds(format!(
                    "index {k} outside '{name}[{}:{}]'",
                    layout.lower,
                    layout.lower + layout.len as i64 - 1
                )),
                span,
            )));
        }
        Ok(())
    }

    /// Resolve a name through the substitution stack (innermost first).
    /// Only identity renames (identifier-to-identifier) are returned; other
    /// substitutions keep the original name.
    fn resolve_substituted_name(&self, name: &SmolStr) -> SmolStr {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                if let Expression::Identifier(id) = expr {
                    return id.name.clone();
                }
                return name.clone();
            }
        }
        name.clone()
    }

    /// Look up a substitution for an identifier
    fn lookup_substitution(&self, name: &SmolStr) -> Option<Expression> {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                return Some(expr.clone());
            }
        }
        None
    }

    /// Rewrite an expression: apply substitutions (block locals, loop
    /// variables) and inline calls to user-defined analog functions.
    fn lower_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Identifier(id) => match self.lookup_substitution(&id.name) {
                Some(subst) => subst,
                None => expr.clone(),
            },
            Expression::Number(_) | Expression::StringLit(_) | Expression::BranchAccess(_) => {
                expr.clone()
            }
            Expression::Binary(b) => Expression::Binary(BinaryExpr {
                op: b.op,
                left: Box::new(self.lower_expression(&b.left)?),
                right: Box::new(self.lower_expression(&b.right)?),
                span: b.span,
            }),
            Expression::Unary(u) => Expression::Unary(UnaryExpr {
                op: u.op,
                operand: Box::new(self.lower_expression(&u.operand)?),
                span: u.span,
            }),
            Expression::Conditional(c) => Expression::Conditional(ConditionalExpr {
                condition: Box::new(self.lower_expression(&c.condition)?),
                then_expr: Box::new(self.lower_expression(&c.then_expr)?),
                else_expr: Box::new(self.lower_expression(&c.else_expr)?),
                span: c.span,
            }),
            Expression::SystemFunction(f) => {
                let args = f
                    .args
                    .iter()
                    .map(|a| self.lower_expression(a))
                    .collect::<CompileResult<Vec<_>>>()?;
                Expression::SystemFunction(SystemFunction {
                    name: f.name.clone(),
                    args,
                    span: f.span,
                })
            }
            Expression::Call(call) => {
                // Nature access functions other than V/I (Pwr, Temp, ...)
                // parse as calls; rewrite them into branch accesses.
                if !self.user_functions.contains_key(&call.name)
                    && self.disciplines.resolve_access(&call.name).is_some()
                    && matches!(call.args.len(), 1 | 2)
                    && call.args.iter().all(|a| {
                        matches!(a, Expression::Identifier(id)
                        if self.symbols.lookup(&id.name).is_some_and(|s| matches!(
                            s.kind,
                            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
                        )))
                    })
                {
                    let mut nodes = call.args.iter().map(|a| match a {
                        Expression::Identifier(id) => id.name.clone(),
                        _ => unreachable!(),
                    });
                    return Ok(Expression::BranchAccess(BranchAccess::Nodes {
                        access: call.name.clone(),
                        pos: nodes.next().unwrap(),
                        neg: nodes.next(),
                        span: call.span,
                    }));
                }

                let args = call
                    .args
                    .iter()
                    .map(|a| self.lower_expression(a))
                    .collect::<CompileResult<Vec<_>>>()?;
                if self.user_functions.contains_key(&call.name) {
                    self.inline_function(&call.name, args, call.span)?
                } else {
                    Expression::Call(CallExpr {
                        name: call.name.clone(),
                        args,
                        span: call.span,
                    })
                }
            }
            Expression::ArrayAccess(a) => {
                let index = self.lower_expression(&a.index)?;
                let array_name = self.resolve_substituted_name(&a.array);
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            a.array
                        )),
                        a.span,
                    )));
                };
                // Indexes that fold to instance-invariant constants (literals,
                // unrolled loop variables) resolve straight to the element
                // variable; everything else stays a runtime indexed access
                if let Some(k) = self.eval_const_invariant(&index) {
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, a.span)?;
                    Expression::Identifier(Identifier {
                        name: SmolStr::from(format!("{array_name}[{k}]")),
                        span: a.span,
                    })
                } else {
                    Expression::ArrayAccess(ArrayAccessExpr {
                        array: array_name,
                        index: Box::new(index),
                        span: a.span,
                    })
                }
            }
            Expression::ArrayLiteral(a) => {
                let elements = a
                    .elements
                    .iter()
                    .map(|e| self.lower_expression(e))
                    .collect::<CompileResult<Vec<_>>>()?;
                Expression::ArrayLiteral(ArrayLiteralExpr {
                    elements,
                    span: a.span,
                })
            }
            Expression::AnalogOperator(_) | Expression::NoiseSource(_) => expr.clone(),
        })
    }

    const MAX_INLINE_DEPTH: usize = 16;

    /// Inline a call to a user-defined analog function by symbolically
    /// executing its body. The return value is the final expression bound
    /// to the function-name variable.
    fn inline_function(
        &mut self,
        name: &SmolStr,
        args: Vec<Expression>,
        span: Span,
    ) -> CompileResult<Expression> {
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    name
                )),
                span,
            )));
        }

        let func = self
            .user_functions
            .get(name)
            .cloned()
            .expect("checked by caller");

        let inputs: Vec<_> = func
            .params
            .iter()
            .filter(|p| p.direction == ParamDirection::Input)
            .collect();
        if func.params.len() != inputs.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function '{}': output/inout arguments are not supported yet",
                    name
                )),
                span,
            )));
        }
        if args.len() != inputs.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: name.to_string(),
                    expected: inputs.len().to_string(),
                    got: args.len(),
                },
                span,
            )));
        }

        // Bind parameters and locals in a fresh substitution frame
        let mut frame = HashMap::new();
        for (param, arg) in inputs.iter().zip(args) {
            frame.insert(param.name.clone(), arg);
        }
        for var_decl in &func.locals {
            for item in &var_decl.items {
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, name
                        )),
                        item.span,
                    )));
                }
                let init = match &item.init {
                    Some(init) => init.clone(),
                    None => Self::number_expr(0.0, item.span),
                };
                frame.insert(item.name.clone(), init);
            }
        }
        // The return value accumulates in a variable named after the function
        frame.insert(func.name.clone(), Self::number_expr(0.0, span));

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let result = self.exec_function_body(&func.body.statements, None);
        self.inline_depth -= 1;
        let frame = self.subst_stack.pop().expect("pushed above");
        result?;

        Ok(frame
            .get(&func.name)
            .cloned()
            .unwrap_or_else(|| Self::number_expr(0.0, span)))
    }

    /// Symbolically execute function-body statements, updating the topmost
    /// substitution frame.
    fn exec_function_body(
        &mut self,
        statements: &[AnalogStatement],
        guard: Option<&Expression>,
    ) -> CompileResult<()> {
        for stmt in statements {
            self.exec_function_statement(stmt, guard)?;
        }
        Ok(())
    }

    fn exec_function_statement(
        &mut self,
        stmt: &AnalogStatement,
        guard: Option<&Expression>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Assignment(assign) => {
                let LValue::Variable { name, span } = &assign.target else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "array assignment inside analog functions is not supported".into(),
                        ),
                        assign.span,
                    )));
                };
                let value = self.lower_expression(&assign.value)?;
                let prev = self
                    .lookup_substitution(name)
                    .unwrap_or_else(|| Self::number_expr(0.0, *span));
                let new_value = match guard {
                    Some(g) => Expression::Conditional(ConditionalExpr {
                        condition: Box::new(g.clone()),
                        then_expr: Box::new(value),
                        else_expr: Box::new(prev),
                        span: *span,
                    }),
                    None => value,
                };
                self.subst_stack
                    .last_mut()
                    .expect("function frame")
                    .insert(name.clone(), new_value);
            }
            AnalogStatement::Conditional(cond) => {
                let condition = self.lower_expression(&cond.condition)?;
                let then_guard = match guard {
                    Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), condition.clone()),
                    None => condition.clone(),
                };
                self.exec_function_statement(&cond.then_branch, Some(&then_guard))?;
                if let Some(else_branch) = &cond.else_branch {
                    let not_cond = Self::not_expr(condition);
                    let else_guard = match guard {
                        Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), not_cond),
                        None => not_cond,
                    };
                    self.exec_function_statement(else_branch, Some(&else_guard))?;
                }
            }
            AnalogStatement::Case(case_stmt) => {
                let selector = self.lower_expression(&case_stmt.expr)?;
                let mut prior_match: Option<Expression> = None;
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered = self.lower_expression(m)?;
                        let eq = Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered);
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                    }
                    let Some(item_match) = item_match else {
                        continue;
                    };
                    let mut item_guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };
                    if let Some(g) = guard {
                        item_guard = Self::binary_expr(BinaryOp::And, g.clone(), item_guard);
                    }
                    self.exec_function_statement(&item.statement, Some(&item_guard))?;
                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }
                if let Some(default) = &case_stmt.default {
                    let default_guard = match (guard, prior_match) {
                        (Some(g), Some(prior)) => Some(Self::binary_expr(
                            BinaryOp::And,
                            g.clone(),
                            Self::not_expr(prior),
                        )),
                        (None, Some(prior)) => Some(Self::not_expr(prior)),
                        (Some(g), None) => Some(g.clone()),
                        (None, None) => None,
                    };
                    self.exec_function_statement(default, default_guard.as_ref())?;
                }
            }
            AnalogStatement::Block(block) => {
                // Function-internal blocks share the function frame; local
                // declarations bind into it
                for var_decl in &block.variables {
                    for item in &var_decl.items {
                        if !item.dimensions.is_empty() {
                            return Err(CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::UnsupportedFeature(format!(
                                    "array local '{}' in analog function",
                                    item.name
                                )),
                                item.span,
                            )));
                        }
                        let init = match &item.init {
                            Some(init) => self.lower_expression(init)?,
                            None => Self::number_expr(0.0, item.span),
                        };
                        self.subst_stack
                            .last_mut()
                            .expect("function frame")
                            .insert(item.name.clone(), init);
                    }
                }
                self.exec_function_body(&block.statements, guard)?;
            }
            AnalogStatement::Null(_) | AnalogStatement::Call(_) => {}
            other => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "statement not supported inside analog functions: {:?}",
                        std::mem::discriminant(other)
                    )),
                    Span::dummy(),
                )));
            }
        }
        Ok(())
    }

    fn validate_node(&self, name: &str, span: Span) -> CompileResult<()> {
        if let Some(sym) = self.symbols.lookup(name) {
            match sym.kind {
                SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch => Ok(()),
                _ => Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidNodeReference {
                        name: name.into(),
                        kind: format!("{:?}", sym.kind),
                    },
                    span,
                ))),
            }
        } else {
            Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )))
        }
    }

    fn infer_type(&self, expr: &Expression) -> CompileResult<ValueType> {
        match expr {
            Expression::Number(_) => Ok(ValueType::Real),
            Expression::StringLit(_) => Ok(ValueType::String),
            Expression::Identifier(ident) => {
                if let Some(sym) = self.symbols.lookup(&ident.name) {
                    Ok(sym.value_type)
                } else if let Some((base, _)) = ident.name.split_once('[')
                    && self.arrays.contains_key(base)
                    && let Some(sym) = self.symbols.lookup(base)
                {
                    // Array element slot (`arr[k]`): typed like the array
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::BranchAccess(_) => Ok(ValueType::NatureAccess),
            Expression::SystemFunction(_) => Ok(ValueType::Real),
            Expression::Call(call) => {
                if let Some(sig) = self.functions.get(&call.name) {
                    Ok(sig.return_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::Unary(unary) => {
                let operand_type = self.infer_type(&unary.operand)?;
                match unary.op {
                    UnaryOp::Pos | UnaryOp::Neg => Ok(operand_type),
                    UnaryOp::Not => Ok(ValueType::Boolean),
                    UnaryOp::BitNot => Ok(ValueType::Integer),
                }
            }
            Expression::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                match binary.op {
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => Ok(ValueType::Boolean),
                    BinaryOp::And | BinaryOp::Or => Ok(ValueType::Boolean),
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Pow
                    | BinaryOp::Mod => Ok(left.common_type(right)),
                    BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => Ok(ValueType::Integer),
                }
            }
            Expression::Conditional(cond) => {
                let then_type = self.infer_type(&cond.then_expr)?;
                let else_type = self.infer_type(&cond.else_expr)?;
                Ok(then_type.common_type(else_type))
            }
            Expression::ArrayAccess(a) => {
                if let Some(sym) = self.symbols.lookup(&a.array) {
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::ArrayLiteral(_) => Ok(ValueType::Unknown),
            Expression::AnalogOperator(_) => Ok(ValueType::Real),
            Expression::NoiseSource(_) => Ok(ValueType::Real),
        }
    }

    /// Constant evaluation against parameter defaults. Suitable only for
    /// compile-time diagnostics (range checks on declared defaults):
    /// instances may override parameters.
    fn eval_const(&self, expr: &Expression) -> Option<f64> {
        Self::eval_const_with(expr, &self.param_consts)
    }

    /// Constant evaluation that only resolves instance-invariant values.
    /// Anything that shapes generated code (loop unrolling, repeat counts)
    /// must use this: folding a parameter's *default* would bake it in and
    /// break per-instance overrides.
    fn eval_const_invariant(&self, expr: &Expression) -> Option<f64> {
        Self::eval_const_with(expr, &self.invariant_consts)
    }

    fn eval_const_with(expr: &Expression, env: &HashMap<SmolStr, f64>) -> Option<f64> {
        let eval = |e: &Expression| Self::eval_const_with(e, env);
        match expr {
            Expression::Number(n) => Some(n.value),
            Expression::Unary(u) => {
                let v = eval(&u.operand)?;
                Some(match u.op {
                    UnaryOp::Neg => -v,
                    UnaryOp::Pos => v,
                    UnaryOp::Not => {
                        if v == 0.0 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    UnaryOp::BitNot => !(v as i64) as f64,
                })
            }
            Expression::Binary(b) => {
                let l = eval(&b.left)?;
                let r = eval(&b.right)?;
                Some(match b.op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Mod => l % r,
                    BinaryOp::Pow => l.powf(r),
                    BinaryOp::Eq => f64::from(l == r),
                    BinaryOp::Ne => f64::from(l != r),
                    BinaryOp::Lt => f64::from(l < r),
                    BinaryOp::Le => f64::from(l <= r),
                    BinaryOp::Gt => f64::from(l > r),
                    BinaryOp::Ge => f64::from(l >= r),
                    BinaryOp::And => f64::from(l != 0.0 && r != 0.0),
                    BinaryOp::Or => f64::from(l != 0.0 || r != 0.0),
                    BinaryOp::Shl => ((l as i64) << (r as i64)) as f64,
                    BinaryOp::Shr => ((l as i64) >> (r as i64)) as f64,
                    BinaryOp::BitAnd => ((l as i64) & (r as i64)) as f64,
                    BinaryOp::BitOr => ((l as i64) | (r as i64)) as f64,
                    BinaryOp::BitXor => ((l as i64) ^ (r as i64)) as f64,
                })
            }
            Expression::Conditional(c) => {
                let cond = eval(&c.condition)?;
                if cond != 0.0 {
                    eval(&c.then_expr)
                } else {
                    eval(&c.else_expr)
                }
            }
            Expression::Call(call) => {
                let args: Option<Vec<f64>> = call.args.iter().map(eval).collect();
                let args = args?;
                match (call.name.as_str(), args.as_slice()) {
                    ("abs", [x]) => Some(x.abs()),
                    ("sqrt", [x]) => Some(x.sqrt()),
                    ("exp", [x]) => Some(x.exp()),
                    ("ln" | "log", [x]) => Some(x.ln()),
                    ("log10", [x]) => Some(x.log10()),
                    ("floor", [x]) => Some(x.floor()),
                    ("ceil", [x]) => Some(x.ceil()),
                    ("min", [a, b]) => Some(a.min(*b)),
                    ("max", [a, b]) => Some(a.max(*b)),
                    ("pow", [a, b]) => Some(a.powf(*b)),
                    _ => None,
                }
            }
            Expression::Identifier(ident) => match ident.name.as_str() {
                "inf" => Some(f64::INFINITY),
                name => env.get(name).copied(),
            },
            _ => None,
        }
    }

    fn parse_range(
        &self,
        range: &ParameterRange,
        param_names: &std::collections::HashSet<SmolStr>,
    ) -> TypedParameterRange {
        // A bound that references another parameter must not fold against
        // that parameter's default: the instance may override it, and a
        // baked-in bound would clamp against the wrong limit. Such bounds
        // stay unchecked (None).
        let fold = |e: &Expression| -> Option<f64> {
            if Self::references_identifiers(e, param_names) {
                None
            } else {
                self.eval_const(e)
            }
        };

        // Extract bounds from first range bound if present
        if let Some(bound) = range.bounds.first() {
            let min = bound.lower.as_ref().and_then(fold);
            let max = bound.upper.as_ref().and_then(fold);
            let exclude: Vec<f64> = range.exclude.iter().filter_map(fold).collect();

            TypedParameterRange {
                min,
                max,
                min_exclusive: !bound.lower_inclusive,
                max_exclusive: !bound.upper_inclusive,
                exclude,
            }
        } else {
            TypedParameterRange::unrestricted()
        }
    }

    fn record_error_at(&mut self, kind: SemanticErrorKind, span: Span) {
        self.errors.push(SemanticError::new(kind, span));
    }

    /// Whether an expression references any identifier from the given set
    fn references_identifiers(
        expr: &Expression,
        names: &std::collections::HashSet<SmolStr>,
    ) -> bool {
        match expr {
            Expression::Identifier(id) => names.contains(&id.name),
            Expression::Number(_) | Expression::StringLit(_) => false,
            Expression::Binary(b) => {
                Self::references_identifiers(&b.left, names)
                    || Self::references_identifiers(&b.right, names)
            }
            Expression::Unary(u) => Self::references_identifiers(&u.operand, names),
            Expression::Conditional(c) => {
                Self::references_identifiers(&c.condition, names)
                    || Self::references_identifiers(&c.then_expr, names)
                    || Self::references_identifiers(&c.else_expr, names)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::SystemFunction(f) => f
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::ArrayAccess(a) => Self::references_identifiers(&a.index, names),
            Expression::ArrayLiteral(a) => a
                .elements
                .iter()
                .any(|e| Self::references_identifiers(e, names)),
            Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::source::SourceId;

    fn analyze(source: &str) -> CompileResult<AnalyzedFile> {
        let tokens = Lexer::new(source, SourceId::new(0))
            .collect_tokens()
            .expect("lex failed");
        let file = Parser::new(&tokens).parse().expect("parse failed");
        SemanticAnalyzer::new().analyze(&file)
    }

    fn analyze_one(source: &str) -> AnalyzedModule {
        let analyzed = analyze(source).expect("semantic analysis failed");
        analyzed.modules.into_values().next().expect("one module")
    }

    const PREAMBLE: &str = r#"
        module dut(p, n);
        inout p, n;
        electrical p, n;
    "#;

    fn module_src(body: &str) -> String {
        format!("{PREAMBLE}{body}\nendmodule")
    }

    /// Flatten the statement tree into the assignments it contains
    /// (loop bodies included), preserving order
    fn flat_assignments(m: &AnalyzedModule) -> Vec<&AnalyzedAssignment> {
        fn walk<'a>(stmts: &'a [AnalyzedStatement], out: &mut Vec<&'a AnalyzedAssignment>) {
            for stmt in stmts {
                match stmt {
                    AnalyzedStatement::Assignment(a) => out.push(a),
                    AnalyzedStatement::Loop(l) => walk(&l.body, out),
                }
            }
        }
        let mut out = Vec::new();
        walk(&m.statements, &mut out);
        out
    }

    #[test]
    fn conditional_assignment_lowered_to_guarded_expression() {
        let m = analyze_one(&module_src(
            r#"
            parameter integer mode = 0;
            real x;
            analog begin
                if (mode > 0)
                    x = 1.0;
                else
                    x = 2.0;
                I(p, n) <+ x * V(p, n);
            end
            "#,
        ));
        // Guard snapshot + the two guarded branch assignments
        assert_eq!(flat_assignments(&m).len(), 3);
        // The condition is snapshotted once so branch bodies cannot
        // perturb it
        assert!(flat_assignments(&m)[0].target.starts_with("__guard"));
        // Both branch assignments must be guarded conditionals, not raw values
        assert!(matches!(
            flat_assignments(&m)[1].expression,
            Expression::Conditional(_)
        ));
        assert!(matches!(
            flat_assignments(&m)[2].expression,
            Expression::Conditional(_)
        ));
        // The else-branch guard preserves the previous value via the variable
        let Expression::Conditional(c) = &flat_assignments(&m)[2].expression else {
            unreachable!()
        };
        assert!(matches!(*c.else_expr, Expression::Identifier(_)));
    }

    #[test]
    fn branch_body_cannot_perturb_its_own_guard() {
        // The classic NOT_GIVEN defaulting idiom: only ONE arm may run.
        // Without condition snapshotting the then-branch assignment makes
        // the re-evaluated else-guard true as well.
        let m = analyze_one(&module_src(
            r#"
            real t;
            analog begin
                t = -1.0;
                if (t < 0.0)
                    t = 25.0;
                else
                    t = t + 273.15;
                I(p, n) <+ t * 1e-6 * V(p, n);
            end
            "#,
        ));

        // Emulate the VM: execute assignments in order over a variable map
        let mut vars: std::collections::HashMap<&str, f64> = std::collections::HashMap::new();
        fn eval(expr: &Expression, vars: &std::collections::HashMap<&str, f64>) -> f64 {
            match expr {
                Expression::Number(n) => n.value,
                Expression::Identifier(id) => vars.get(id.name.as_str()).copied().unwrap_or(0.0),
                Expression::Unary(u) => {
                    let v = eval(&u.operand, vars);
                    match u.op {
                        UnaryOp::Neg => -v,
                        UnaryOp::Pos => v,
                        UnaryOp::Not => f64::from(v == 0.0),
                        UnaryOp::BitNot => !(v as i64) as f64,
                    }
                }
                Expression::Binary(b) => {
                    let l = eval(&b.left, vars);
                    let r = eval(&b.right, vars);
                    match b.op {
                        BinaryOp::Add => l + r,
                        BinaryOp::Lt => f64::from(l < r),
                        BinaryOp::And => f64::from(l != 0.0 && r != 0.0),
                        _ => f64::NAN,
                    }
                }
                Expression::Conditional(c) => {
                    if eval(&c.condition, vars) != 0.0 {
                        eval(&c.then_expr, vars)
                    } else {
                        eval(&c.else_expr, vars)
                    }
                }
                _ => f64::NAN,
            }
        }
        for assign in flat_assignments(&m) {
            let value = eval(&assign.expression, &vars);
            // Keys live as long as the module borrow
            let key: &str = Box::leak(assign.target.to_string().into_boxed_str());
            vars.insert(key, value);
        }
        assert_eq!(
            vars.get("t").copied(),
            Some(25.0),
            "only the then-arm may execute; got {vars:?}"
        );
    }

    #[test]
    fn conditional_contribution_contributes_zero_when_inactive() {
        let m = analyze_one(&module_src(
            r#"
            parameter integer on = 1;
            analog begin
                if (on)
                    I(p, n) <+ V(p, n);
            end
            "#,
        ));
        assert_eq!(m.contributions.len(), 1);
        let Expression::Conditional(c) = &m.contributions[0].expression else {
            panic!("expected guarded contribution");
        };
        let Expression::Number(zero) = &*c.else_expr else {
            panic!("expected zero fallback");
        };
        assert_eq!(zero.value, 0.0);
    }

    #[test]
    fn for_loop_unrolls_statically() {
        let m = analyze_one(&module_src(
            r#"
            integer i;
            real x;
            analog begin
                for (i = 0; i < 3; i = i + 1)
                    x = x + 1.0;
            end
            "#,
        ));
        assert_eq!(flat_assignments(&m).len(), 3);
    }

    #[test]
    fn localparam_becomes_computed_variable() {
        let m = analyze_one(&module_src(
            r#"
            parameter real w = 2.0;
            localparam real area = w * 3.0;
            analog I(p, n) <+ area * V(p, n);
            "#,
        ));
        assert!(m.variables.iter().any(|v| v.name == "area"));
        assert_eq!(flat_assignments(&m).len(), 1);
        assert_eq!(flat_assignments(&m)[0].target.as_str(), "area");
    }

    #[test]
    fn variable_initializer_recorded() {
        let m = analyze_one(&module_src(
            r#"
            real x = 4.5;
            analog I(p, n) <+ x * V(p, n);
            "#,
        ));
        assert_eq!(flat_assignments(&m).len(), 1);
        assert_eq!(flat_assignments(&m)[0].target.as_str(), "x");
    }

    #[test]
    fn named_branch_contribution_resolves_nodes() {
        let m = analyze_one(&module_src(
            r#"
            branch (p, n) res;
            analog I(res) <+ V(res) / 2.0;
            "#,
        ));
        assert_eq!(m.contributions.len(), 1);
        assert_eq!(m.contributions[0].branch.as_str(), "p,n");
        assert!(m.contributions[0].is_current);
    }

    #[test]
    fn parameter_default_out_of_range_is_an_error() {
        let result = analyze(&module_src(
            r#"
            parameter real r = -1.0 from (0:inf);
            analog I(p, n) <+ V(p, n) / r;
            "#,
        ));
        assert!(result.is_err(), "out-of-range default must fail");
    }

    #[test]
    fn assignment_to_parameter_is_an_error() {
        let result = analyze(&module_src(
            r#"
            parameter real r = 1.0;
            analog begin
                r = 2.0;
                I(p, n) <+ V(p, n) / r;
            end
            "#,
        ));
        assert!(result.is_err(), "assigning a parameter must fail");
    }

    #[test]
    fn undeclared_node_in_contribution_is_an_error() {
        let result = analyze(&module_src(
            r#"
            analog I(p, ghost) <+ V(p, n);
            "#,
        ));
        assert!(result.is_err());
    }

    #[test]
    fn block_local_variable_shadowing_is_hoisted() {
        let m = analyze_one(&module_src(
            r#"
            real tmp;
            analog begin : outer
                real tmp;
                tmp = V(p, n);
                I(p, n) <+ tmp;
            end
            "#,
        ));
        // Module-level tmp plus a hoisted (renamed) block-local tmp
        assert_eq!(
            m.variables
                .iter()
                .filter(|v| v.name.starts_with("tmp"))
                .count(),
            2
        );
        // The block assignment targets the hoisted name, not the outer tmp
        assert_ne!(flat_assignments(&m)[0].target.as_str(), "tmp");
    }

    #[test]
    fn user_function_is_inlined() {
        let m = analyze_one(&module_src(
            r#"
            parameter real gain = 2.0;
            analog function real double_it;
                input v;
                real scratch;
                begin
                    scratch = 2.0 * v;
                    double_it = scratch;
                end
            endfunction
            analog I(p, n) <+ double_it(V(p, n)) * gain;
            "#,
        ));
        assert_eq!(m.contributions.len(), 1);
        // No residual user-function calls may remain after inlining
        fn has_user_call(expr: &Expression) -> bool {
            match expr {
                Expression::Call(c) => c.name == "double_it" || c.args.iter().any(has_user_call),
                Expression::Binary(b) => has_user_call(&b.left) || has_user_call(&b.right),
                Expression::Unary(u) => has_user_call(&u.operand),
                Expression::Conditional(c) => {
                    has_user_call(&c.condition)
                        || has_user_call(&c.then_expr)
                        || has_user_call(&c.else_expr)
                }
                _ => false,
            }
        }
        assert!(!has_user_call(&m.contributions[0].expression));
    }

    #[test]
    fn function_with_conditional_inlines_to_ternary() {
        let m = analyze_one(&module_src(
            r#"
            analog function real clip;
                input v;
                begin
                    if (v > 1.0)
                        clip = 1.0;
                    else
                        clip = v;
                end
            endfunction
            analog I(p, n) <+ clip(V(p, n));
            "#,
        ));
        assert!(matches!(
            m.contributions[0].expression,
            Expression::Conditional(_)
        ));
    }

    #[test]
    fn thermal_contribution_is_flow() {
        let m = analyze_one(
            r#"
            module heater(p, n, t);
            inout p, n, t;
            electrical p, n;
            thermal t;
            analog Pwr(t) <+ V(p, n) * V(p, n) / 10.0;
            endmodule
            "#,
        );
        assert_eq!(m.contributions.len(), 1);
        assert!(
            m.contributions[0].is_current,
            "power into a thermal node is a flow contribution"
        );
    }

    #[test]
    fn ground_net_does_not_allocate_internal_node() {
        let m = analyze_one(&module_src(
            r#"
            ground gnd;
            electrical mid;
            analog begin
                I(p, mid) <+ V(p, mid);
                I(mid, gnd) <+ V(mid, gnd);
            end
            "#,
        ));
        assert_eq!(m.internal_nodes.len(), 1);
        assert_eq!(m.internal_nodes[0].name.as_str(), "mid");
        assert_eq!(m.ground_nodes, vec![SmolStr::from("gnd")]);
    }

    #[test]
    fn initial_step_lowered_to_static_analysis_guard() {
        let m = analyze_one(&module_src(
            r#"
            real seed;
            analog begin
                @(initial_step) seed = 1.0;
                I(p, n) <+ seed * V(p, n);
            end
            "#,
        ));
        // [0] is the snapshotted analysis() guard, [1] the guarded seed
        let snapshot = flat_assignments(&m)[0];
        assert!(snapshot.target.starts_with("__guard"));
        let Expression::Call(call) = &snapshot.expression else {
            panic!(
                "expected analysis() snapshot, got {:?}",
                snapshot.expression
            );
        };
        assert_eq!(call.name.as_str(), "analysis");
        let Expression::Conditional(c) = &flat_assignments(&m)[1].expression else {
            panic!("expected guarded assignment");
        };
        assert!(matches!(*c.condition, Expression::Identifier(_)));
    }

    #[test]
    fn while_loop_with_runtime_condition_lowers_to_runtime_loop() {
        let m = analyze_one(&module_src(
            r#"
            real x;
            analog begin
                while (x < 10.0) x = x + 1.0;
            end
            "#,
        ));
        assert!(
            m.statements
                .iter()
                .any(|s| matches!(s, AnalyzedStatement::Loop(_))),
            "runtime while must lower to a loop statement"
        );
    }

    #[test]
    fn parameter_bounded_for_loop_lowers_to_runtime_loop() {
        let m = analyze_one(&module_src(
            r#"
            parameter integer nf = 4;
            integer i;
            real acc;
            analog begin
                acc = 0.0;
                for (i = 0; i < nf; i = i + 1)
                    acc = acc + 2.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
        ));
        let loops: Vec<_> = m
            .statements
            .iter()
            .filter(|s| matches!(s, AnalyzedStatement::Loop(_)))
            .collect();
        assert_eq!(loops.len(), 1, "parameter-bounded loop stays a loop");
        let AnalyzedStatement::Loop(l) = loops[0] else {
            unreachable!()
        };
        // Body: accumulator update + loop variable update
        assert_eq!(l.body.len(), 2);
    }

    #[test]
    fn guarded_runtime_loop_condition_includes_guard() {
        let m = analyze_one(&module_src(
            r#"
            parameter integer nf = 4;
            parameter integer en = 1;
            integer i;
            real acc;
            analog begin
                if (en > 0)
                    for (i = 0; i < nf; i = i + 1)
                        acc = acc + 1.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
        ));
        let AnalyzedStatement::Loop(l) = m
            .statements
            .iter()
            .find(|s| matches!(s, AnalyzedStatement::Loop(_)))
            .expect("loop present")
        else {
            unreachable!()
        };
        // The enclosing guard is ANDed into the loop condition
        assert!(
            matches!(&l.condition, Expression::Binary(b) if b.op == BinaryOp::And),
            "guard must be folded into the loop condition, got {:?}",
            l.condition
        );
    }

    #[test]
    fn contribution_inside_runtime_loop_is_an_error() {
        let result = analyze(&module_src(
            r#"
            parameter integer nf = 4;
            integer i;
            analog begin
                for (i = 0; i < nf; i = i + 1)
                    I(p, n) <+ V(p, n);
            end
            "#,
        ));
        assert!(
            result.is_err(),
            "contributions need compile-time-constant loop bounds"
        );
    }

    #[test]
    fn runtime_repeat_synthesizes_counter() {
        let m = analyze_one(&module_src(
            r#"
            parameter integer nf = 3;
            real acc;
            analog begin
                repeat (nf) acc = acc + 1.0;
                I(p, n) <+ acc * V(p, n);
            end
            "#,
        ));
        assert!(
            m.statements
                .iter()
                .any(|s| matches!(s, AnalyzedStatement::Loop(_))),
            "runtime repeat lowers to a loop"
        );
        assert!(
            m.variables.iter().any(|v| v.name.starts_with("__repeat")),
            "synthesized counter variables registered"
        );
    }
}
