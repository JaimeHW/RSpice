//! Semantic Analyzer for Verilog-A/AMS
//!
//! Provides comprehensive semantic analysis including:
//! - Hierarchical symbol table with nested scopes
//! - Type inference and checking
//! - Discipline validation
//! - Expression validation
//! - Parameter range checking

use crate::ast::*;
use crate::disciplines::{Discipline, DisciplineDb, Domain, Nature};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use crate::types::{FunctionRegistry, ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::collections::HashMap;

const RSPICE_LIMITED_EXP_INTRINSIC: &str = "__rspice_limited_exp";

mod analyzed;
mod symbols;

pub use analyzed::*;
pub use symbols::*;

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
    /// Caller-visible assignments produced by analog function output/inout
    /// arguments while expression lowering is in progress.
    function_side_effects: Vec<AssignmentStmt>,
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

/// How an event expression lowers into the dataflow representation
enum EventLowering {
    /// Body executes when this runtime guard is nonzero
    Guard(Expression),
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
            function_side_effects: Vec::new(),
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

        // First pass: register user-defined natures, then disciplines that
        // reference them. Access compatibility validation relies on this DB.
        for item in &source.items {
            if let Item::Nature(nature) = item {
                self.register_nature(nature)?;
            }
        }
        for item in &source.items {
            if let Item::Discipline(discipline) = item {
                self.register_discipline(discipline)?;
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
                self.function_side_effects.clear();
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

    fn register_nature(&mut self, nature: &NatureDef) -> CompileResult<()> {
        let base = nature
            .base
            .as_deref()
            .map(|base| {
                self.disciplines.get_nature(base).ok_or_else(|| {
                    CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "nature '{}' extends unknown base nature '{}'",
                            nature.name, base
                        )),
                        nature.span,
                    ))
                })
            })
            .transpose()?;

        let access = nature
            .access
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.access.clone()))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::MissingAttribute(format!(
                        "access for nature '{}'",
                        nature.name
                    )),
                    nature.span,
                ))
            })?;
        let units = nature
            .units
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.units.clone()))
            .unwrap_or_default();
        let abstol = nature
            .abstol
            .as_ref()
            .and_then(|expr| self.eval_const(expr))
            .or_else(|| base.map(|base| base.abstol))
            .unwrap_or(0.0);

        self.disciplines.add_nature(Nature {
            name: nature.name.to_string(),
            units,
            abstol,
            access,
            idt_nature: nature.idt_nature.as_ref().map(|s| s.to_string()),
            ddt_nature: nature.ddt_nature.as_ref().map(|s| s.to_string()),
            span: Some(nature.span),
        });
        Ok(())
    }

    fn register_discipline(&mut self, discipline: &DisciplineDef) -> CompileResult<()> {
        for nature_name in [discipline.potential.as_ref(), discipline.flow.as_ref()]
            .into_iter()
            .flatten()
        {
            if self.disciplines.get_nature(nature_name).is_none() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(format!(
                        "discipline '{}' references unknown nature '{}'",
                        discipline.name, nature_name
                    )),
                    discipline.span,
                )));
            }
        }

        let domain = match discipline.domain.unwrap_or(DomainKind::Continuous) {
            DomainKind::Continuous => Domain::Continuous,
            DomainKind::Discrete => Domain::Discrete,
        };
        self.disciplines.add_discipline(Discipline {
            name: discipline.name.to_string(),
            domain,
            potential: discipline.potential.as_ref().map(|s| s.to_string()),
            flow: discipline.flow.as_ref().map(|s| s.to_string()),
            span: Some(discipline.span),
        });
        Ok(())
    }

    fn require_discipline(&self, name: &str, span: Span) -> CompileResult<()> {
        if self.disciplines.get_discipline(name).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(name.to_string()),
                span,
            )));
        }
        Ok(())
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
        let mut declared_ports: HashMap<SmolStr, Span> = HashMap::new();
        for decl in &module.port_declarations {
            if let Some(discipline) = &decl.discipline {
                self.require_discipline(discipline, decl.span)?;
            }
            for name in &decl.names {
                if let Some(first_defined) = declared_ports.get(name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::DuplicateSymbol {
                            name: name.clone(),
                            first_defined: *first_defined,
                        },
                        decl.span,
                    )));
                }
                declared_ports.insert(name.clone(), decl.span);
                port_info.insert(name.clone(), (decl.direction, decl.discipline.clone()));
            }
        }

        // Phase 3: Update port disciplines from net declarations
        for net in &module.nets {
            self.require_discipline(&net.discipline, net.span)?;
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

            let disc = self.disciplines.get_discipline(&disc_name).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(disc_name.to_string()),
                    module.span,
                ))
            })?;
            let potential = disc.potential.as_ref().map(|s| SmolStr::from(s.as_str()));
            let flow = disc.flow.as_ref().map(|s| SmolStr::from(s.as_str()));

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
                discipline: discipline.clone(),
            });

            self.define_symbol(Symbol {
                name: branch.name.clone(),
                kind: SymbolKind::Branch,
                value_type: ValueType::NatureAccess,
                span: branch.span,
                attrs: SymbolAttrs {
                    discipline: Some(discipline),
                    ..Default::default()
                },
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

            if param.param_type == ParamType::Integer
                && let Some(default) = default
                && (default.fract() != 0.0
                    || default < f64::from(i32::MIN)
                    || default > f64::from(i32::MAX))
            {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "32-bit integer".into(),
                        found: default.to_string(),
                        context: format!("default of parameter '{}'", param.name),
                    },
                    param.span,
                );
            }

            if let Some(declared_range) = &param.range {
                if declared_range.bounds.len() > 1 {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "multiple parameter 'from' ranges are not yet supported".into(),
                        ),
                        declared_range.span,
                    );
                }
                let dependent_constraint = declared_range.bounds.iter().any(|bound| {
                    bound.lower.as_ref().is_some_and(|expression| {
                        Self::references_identifiers(expression, &param_names)
                    }) || bound.upper.as_ref().is_some_and(|expression| {
                        Self::references_identifiers(expression, &param_names)
                    })
                }) || declared_range
                    .exclude
                    .iter()
                    .any(|expression| Self::references_identifiers(expression, &param_names));
                if dependent_constraint {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "parameter-dependent range constraints are not yet supported".into(),
                        ),
                        declared_range.span,
                    );
                }
            }

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

            let expression =
                self.lower_expression_with_side_effects(default, &mut analyzed, &mut statements)?;
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
                        let expression = self.lower_expression_with_side_effects(
                            element,
                            &mut analyzed,
                            &mut statements,
                        )?;
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
                let expression =
                    self.lower_expression_with_side_effects(init, &mut analyzed, &mut statements)?;
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

    fn value_type_for_var_type(var_type: VarType) -> ValueType {
        match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        }
    }

    fn register_function_temp(
        &mut self,
        module: &mut AnalyzedModule,
        name: SmolStr,
        var_type: VarType,
        span: Span,
    ) -> CompileResult<()> {
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type,
            value_type: Self::value_type_for_var_type(var_type),
            is_state: false,
        });
        self.define_symbol(Symbol {
            name,
            kind: SymbolKind::Variable,
            value_type: Self::value_type_for_var_type(var_type),
            span,
            attrs: Default::default(),
        })
    }

    fn function_needs_materialization(func: &FunctionDef) -> bool {
        func.params
            .iter()
            .any(|param| param.direction != ParamDirection::Input)
    }

    fn is_recognized_limited_exp_function(func: &FunctionDef) -> bool {
        if !func.name.eq_ignore_ascii_case("lexp")
            || func.return_type != VarType::Real
            || func.params.len() != 1
            || func.params[0].direction != ParamDirection::Input
            || func.params[0].param_type != VarType::Real
        {
            return false;
        }

        let input = &func.params[0].name;
        let mut has_exp_return = false;
        let mut has_upper_linear_return = false;
        let mut has_lower_clamp_return = false;
        Self::collect_limited_exp_return_features(
            &func.body.statements,
            &func.name,
            input,
            &mut has_exp_return,
            &mut has_upper_linear_return,
            &mut has_lower_clamp_return,
        );
        has_exp_return && has_upper_linear_return && has_lower_clamp_return
    }

    fn collect_limited_exp_return_features(
        statements: &[AnalogStatement],
        return_name: &SmolStr,
        input_name: &SmolStr,
        has_exp_return: &mut bool,
        has_upper_linear_return: &mut bool,
        has_lower_clamp_return: &mut bool,
    ) {
        for statement in statements {
            match statement {
                AnalogStatement::Assignment(assignment) if matches!(&assignment.target, LValue::Variable { name, .. } if name == return_name) =>
                {
                    let value = &assignment.value;
                    if Self::expr_contains_call(value, "exp")
                        && Self::expr_contains_identifier(value, input_name)
                    {
                        *has_exp_return = true;
                    }
                    if Self::expr_contains_identifier(value, input_name)
                        && Self::expr_contains_number_close(value, 5.540622384e34)
                    {
                        *has_upper_linear_return = true;
                    }
                    if Self::expr_contains_number_close(value, 1.804851387e-35) {
                        *has_lower_clamp_return = true;
                    }
                }
                AnalogStatement::Conditional(conditional) => {
                    Self::collect_limited_exp_return_features(
                        std::slice::from_ref(&conditional.then_branch),
                        return_name,
                        input_name,
                        has_exp_return,
                        has_upper_linear_return,
                        has_lower_clamp_return,
                    );
                    if let Some(else_branch) = &conditional.else_branch {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(else_branch),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                AnalogStatement::Block(block) => Self::collect_limited_exp_return_features(
                    &block.statements,
                    return_name,
                    input_name,
                    has_exp_return,
                    has_upper_linear_return,
                    has_lower_clamp_return,
                ),
                AnalogStatement::Case(case) => {
                    for item in &case.items {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(&item.statement),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                    if let Some(default) = &case.default {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(default),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn expr_contains_identifier(expr: &Expression, expected: &SmolStr) -> bool {
        match expr {
            Expression::Identifier(identifier) => &identifier.name == expected,
            Expression::Binary(binary) => {
                Self::expr_contains_identifier(&binary.left, expected)
                    || Self::expr_contains_identifier(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_identifier(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_identifier(&conditional.condition, expected)
                    || Self::expr_contains_identifier(&conditional.then_expr, expected)
                    || Self::expr_contains_identifier(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_identifier(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::expr_contains_identifier(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn expr_contains_call(expr: &Expression, expected: &str) -> bool {
        match expr {
            Expression::Call(call) => {
                call.name.eq_ignore_ascii_case(expected)
                    || call
                        .args
                        .iter()
                        .any(|arg| Self::expr_contains_call(arg, expected))
            }
            Expression::Binary(binary) => {
                Self::expr_contains_call(&binary.left, expected)
                    || Self::expr_contains_call(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_call(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_call(&conditional.condition, expected)
                    || Self::expr_contains_call(&conditional.then_expr, expected)
                    || Self::expr_contains_call(&conditional.else_expr, expected)
            }
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_call(arg, expected)),
            Expression::ArrayAccess(access) => Self::expr_contains_call(&access.index, expected),
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::expr_contains_call(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn expr_contains_number_close(expr: &Expression, expected: f64) -> bool {
        match expr {
            Expression::Number(number) => {
                let tolerance = expected.abs().max(1.0) * 1.0e-12;
                (number.value - expected).abs() <= tolerance
            }
            Expression::Binary(binary) => {
                Self::expr_contains_number_close(&binary.left, expected)
                    || Self::expr_contains_number_close(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_number_close(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_number_close(&conditional.condition, expected)
                    || Self::expr_contains_number_close(&conditional.then_expr, expected)
                    || Self::expr_contains_number_close(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_number_close(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::expr_contains_number_close(element, expected)),
            Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
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
                self.analyze_contribution(contrib, module, sink)?;
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
                            let expression =
                                self.lower_expression_with_side_effects(init, module, sink)?;
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
                let condition =
                    self.lower_expression_with_side_effects(&cond.condition, module, sink)?;
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
                let selector =
                    self.lower_expression_with_side_effects(&case_stmt.expr, module, sink)?;
                let selector = self.snapshot_guard(selector, case_stmt.span, module, sink)?;

                let mut item_guards: Vec<Option<Expression>> = Vec::new();
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered = self.lower_expression_with_side_effects(m, module, sink)?;
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
                let count_expr =
                    self.lower_expression_with_side_effects(&repeat.count, module, sink)?;
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
                let condition =
                    self.lower_expression_with_side_effects(&while_stmt.condition, module, sink)?;
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
                let EventLowering::Guard(guard) =
                    self.event_guard(&event_ctrl.event, module, sink)?;
                // Snapshot: the body must not perturb its own guard.
                let guard = self.snapshot_guard(guard, event_ctrl.span, module, sink)?;
                self.guard_stack.push(guard);
                self.analyze_statement(&event_ctrl.statement, module, sink)?;
                self.guard_stack.pop();
            }
            AnalogStatement::IndirectContribution(stmt) => {
                self.analyze_indirect_contribution(stmt, module, sink)?;
            }
            // $bound_step and $discontinuity steer the transient stepper
            // through hidden per-evaluation variables; other system tasks
            // ($strobe, $display, ...) have no effect on the device
            // equations
            AnalogStatement::Call(call) => match call.name.as_str() {
                "$bound_step" => {
                    self.validate_system_task_arity(call, 1, Some(1))?;
                    self.analyze_bound_step(call, module, sink)?;
                }
                "$discontinuity" => {
                    self.validate_system_task_arity(call, 0, Some(1))?;
                    self.analyze_discontinuity(call, module, sink)?;
                }
                name if Self::is_no_effect_system_task(name) => {}
                _ => return Err(Self::unknown_system_task_error(call)),
            },
            AnalogStatement::Disable(_) | AnalogStatement::Null(_) => {}
        }
        Ok(())
    }

    const MAX_STATIC_UNROLL_ITERATIONS: usize = 32;
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
        let init = self.lower_expression_without_side_effects(&for_stmt.init, "for-loop init")?;
        let init_value = self.eval_const_invariant(&init);
        let static_unrollable = if let Some(value) = init_value {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));
            let cond_probe = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .ok()
                .and_then(|c| self.eval_const_invariant(&c));
            let update_probe = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .ok()
                .and_then(|u| self.eval_const_invariant(&u));
            self.subst_stack.pop();
            cond_probe.is_some() && update_probe.is_some()
        } else {
            false
        };

        if static_unrollable {
            let init_value = init_value.expect("checked");
            let iteration_count = self.static_for_iteration_count(for_stmt, init_value)?;
            if iteration_count <= Self::MAX_STATIC_UNROLL_ITERATIONS
                || Self::statement_contains_contribution(&for_stmt.body)
            {
                self.unroll_for(for_stmt, init_value, module, sink)
            } else {
                self.lower_runtime_for(for_stmt, module, sink)
            }
        } else {
            self.lower_runtime_for(for_stmt, module, sink)
        }
    }

    fn static_for_iteration_count(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
    ) -> CompileResult<usize> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = match self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop condition stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(condition) => condition,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            if condition == 0.0 {
                self.subst_stack.pop();
                return Ok(iterations);
            }

            let update = match self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop update stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(update) => update,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            self.subst_stack.pop();

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
            value = update;
        }
    }

    fn statement_contains_contribution(stmt: &AnalogStatement) -> bool {
        match stmt {
            AnalogStatement::Contribution(_) | AnalogStatement::IndirectContribution(_) => true,
            AnalogStatement::Block(block) => block
                .statements
                .iter()
                .any(Self::statement_contains_contribution),
            AnalogStatement::Conditional(cond) => {
                Self::statement_contains_contribution(&cond.then_branch)
                    || cond
                        .else_branch
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::Case(case_stmt) => {
                case_stmt
                    .items
                    .iter()
                    .any(|item| Self::statement_contains_contribution(&item.statement))
                    || case_stmt
                        .default
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::For(for_stmt) => Self::statement_contains_contribution(&for_stmt.body),
            AnalogStatement::Repeat(repeat) => Self::statement_contains_contribution(&repeat.body),
            AnalogStatement::While(while_stmt) => {
                Self::statement_contains_contribution(&while_stmt.body)
            }
            AnalogStatement::EventControl(event_ctrl) => {
                Self::statement_contains_contribution(&event_ctrl.statement)
            }
            AnalogStatement::Assignment(_)
            | AnalogStatement::Call(_)
            | AnalogStatement::Disable(_)
            | AnalogStatement::Null(_) => false,
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

            let condition = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
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

            let update = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")?;
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
        let condition =
            self.lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
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
    fn event_guard(
        &mut self,
        event: &EventExpr,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<EventLowering> {
        Ok(match event {
            EventExpr::InitialStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_initial_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::FinalStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_final_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::Cross {
                signal,
                direction,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let dir_value = match direction {
                    Some(CrossDirection::Rising) => 1.0,
                    Some(CrossDirection::Falling) => -1.0,
                    Some(CrossDirection::Both) | None => 0.0,
                };
                let mut args = vec![signal, Self::number_expr(dir_value, *span)];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Posedge { signal, span } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Negedge { signal, span } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(-1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Above {
                signal,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let mut args = vec![signal];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "above".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Timer {
                start,
                period,
                time_tol,
                enable,
                span,
            } => {
                let mut args = vec![self.lower_expression_with_side_effects(start, module, sink)?];
                if let Some(period) = period {
                    args.push(self.lower_expression_with_side_effects(period, module, sink)?);
                } else if time_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "timer".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Or { left, right, .. } => {
                let left = self.event_guard(left, module, sink)?;
                let right = self.event_guard(right, module, sink)?;
                match (left, right) {
                    (EventLowering::Guard(l), EventLowering::Guard(r)) => {
                        EventLowering::Guard(Self::binary_expr(BinaryOp::Or, l, r))
                    }
                }
            }
        })
    }

    fn filter_step_event(phase: Expression, analyses: &[StringLit], span: Span) -> Expression {
        if analyses.is_empty() {
            return phase;
        }

        let analysis_filter = Expression::Call(CallExpr {
            name: "analysis".into(),
            args: analyses
                .iter()
                .cloned()
                .map(Expression::StringLit)
                .collect(),
            span,
        });
        Self::binary_expr(BinaryOp::And, phase, analysis_filter)
    }

    fn analyze_contribution(
        &mut self,
        contrib: &ContributionStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
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

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&contrib.target, module, contrib.span)?;
        self.validate_branch_access_compatible(&contrib.target, contrib.span)?;

        let expression = self.lower_expression_with_side_effects(&contrib.value, module, sink)?;
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
            declared_branch,
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
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                stmt.span,
            )));
        }

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&stmt.branch, module, stmt.span)?;
        self.validate_branch_access_compatible(&stmt.branch, stmt.span)?;

        let lhs = self.lower_expression_with_side_effects(&stmt.lhs, module, sink)?;
        let rhs = self.lower_expression_with_side_effects(&stmt.rhs, module, sink)?;
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
            declared_branch,
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
        let bound = self.lower_expression_with_side_effects(arg, module, sink)?;
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
        if let Some(arg) = call.args.first() {
            let degree = self.lower_expression_with_side_effects(arg, module, sink)?;
            let degree_type = self.infer_type(&degree)?;
            if !degree_type.is_numeric() {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric".to_string(),
                        found: degree_type.to_string(),
                        context: "$discontinuity argument".to_string(),
                    },
                    call.span,
                );
            }
        }

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
    ) -> CompileResult<(SmolStr, bool, Option<SmolStr>)> {
        match target {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                let is_current = self.resolve_access_kind(access, span)?;

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
                    Ok((
                        SmolStr::from(branch_str),
                        is_current,
                        Some(branch.name.clone()),
                    ))
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
                    Ok((branch.into(), is_current, None))
                }
            }
            BranchAccess::Branch { name, access, .. } => {
                let is_current = self.resolve_access_kind(access, span)?;
                match module.branches.iter().find(|b| b.name == *name) {
                    Some(branch) => {
                        let branch_str = if branch.neg_node.is_empty() {
                            branch.pos_node.to_string()
                        } else {
                            format!("{},{}", branch.pos_node, branch.neg_node)
                        };
                        Ok((
                            SmolStr::from(branch_str),
                            is_current,
                            Some(branch.name.clone()),
                        ))
                    }
                    None => {
                        self.validate_node(name, span)?;
                        Ok((name.to_string().into(), is_current, None))
                    }
                }
            }
        }
    }

    /// Resolve and classify an access function. Unknown access names must
    /// not silently become potential contributions.
    fn resolve_access_kind(&self, access: &str, span: Span) -> CompileResult<bool> {
        if self.disciplines.resolve_access(access).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "unknown access function '{access}'"
                )),
                span,
            )));
        }
        Ok(self.is_flow_access(access))
    }

    fn validate_branch_access_compatible(
        &self,
        access_expr: &BranchAccess,
        span: Span,
    ) -> CompileResult<()> {
        match access_expr {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                self.validate_access_compatible_with_symbol(access, pos, span)?;
                if let Some(neg) = neg {
                    self.validate_access_compatible_with_symbol(access, neg, span)?;
                }
            }
            BranchAccess::Branch { access, name, .. } => {
                self.validate_access_compatible_with_symbol(access, name, span)?;
            }
        }
        Ok(())
    }

    fn validate_access_compatible_with_symbol(
        &self,
        access: &str,
        name: &str,
        span: Span,
    ) -> CompileResult<()> {
        let Some(nature) = self.disciplines.resolve_access(access) else {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "unknown access function '{access}'"
                )),
                span,
            )));
        };
        let Some(symbol) = self.symbols.lookup(name) else {
            if is_global_ground_name(name) {
                return Ok(());
            }
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )));
        };
        if !matches!(
            symbol.kind,
            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
        ) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidNodeReference {
                    name: name.into(),
                    kind: format!("{:?}", symbol.kind),
                },
                span,
            )));
        }
        let discipline = symbol.attrs.discipline.as_deref().unwrap_or("electrical");
        let Some(discipline_def) = self.disciplines.get_discipline(discipline) else {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(discipline.to_string()),
                span,
            )));
        };
        if discipline_def.potential.as_deref() == Some(nature.name.as_str())
            || discipline_def.flow.as_deref() == Some(nature.name.as_str())
        {
            return Ok(());
        }

        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidContribution(format!(
                "access function '{access}' is incompatible with discipline '{discipline}'"
            )),
            span,
        )))
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
                let index = self.lower_expression_with_side_effects(index, module, sink)?;
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

        let expression = self.lower_expression_with_side_effects(&assign.value, module, sink)?;
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

    fn materialize_output_function_call(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Expression>> {
        let Expression::Call(call) = expr else {
            return Ok(None);
        };
        let Some(func) = self.user_functions.get(&call.name).cloned() else {
            return Ok(None);
        };
        if !Self::function_needs_materialization(&func) {
            return Ok(None);
        }
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    call.name
                )),
                call.span,
            )));
        }
        if call.args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected: func.params.len().to_string(),
                    got: call.args.len(),
                },
                call.span,
            )));
        }

        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            if param.direction != ParamDirection::Input {
                let target = self.function_output_lvalue(&func.name, param, arg)?;
                output_bindings.push((param.name.clone(), target, param.span));
            }
        }

        self.local_counter += 1;
        let call_id = self.local_counter;
        let prefix = format!("__fn{call_id}_{}", func.name);
        let make_name = |name: &SmolStr| -> SmolStr { SmolStr::from(format!("{prefix}__{name}")) };

        let return_name: SmolStr = format!("{prefix}__return").into();
        self.register_function_temp(module, return_name.clone(), func.return_type, func.span)?;

        let mut frame = HashMap::new();
        frame.insert(
            func.name.clone(),
            Expression::Identifier(Identifier {
                name: return_name.clone(),
                span: func.span,
            }),
        );

        let mut formal_temps = HashMap::new();
        for param in &func.params {
            let temp_name = make_name(&param.name);
            self.register_function_temp(module, temp_name.clone(), param.param_type, param.span)?;
            formal_temps.insert(param.name.clone(), temp_name.clone());
            frame.insert(
                param.name.clone(),
                Expression::Identifier(Identifier {
                    name: temp_name,
                    span: param.span,
                }),
            );
        }

        let zero_return = AssignmentStmt {
            target: LValue::Variable {
                name: return_name.clone(),
                span: func.span,
            },
            value: Self::number_expr(0.0, func.span),
            span: func.span,
        };
        self.analyze_assignment(&zero_return, module, sink)?;

        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            let formal_temp = formal_temps
                .get(&param.name)
                .expect("formal temp registered")
                .clone();
            let value = match param.direction {
                ParamDirection::Input | ParamDirection::Inout => arg.clone(),
                ParamDirection::Output => Self::number_expr(0.0, param.span),
            };
            let assignment = AssignmentStmt {
                target: LValue::Variable {
                    name: formal_temp,
                    span: param.span,
                },
                value,
                span: param.span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|param| param.name == item.name) {
                    continue;
                }
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, func.name
                        )),
                        item.span,
                    )));
                }
                let temp_name = make_name(&item.name);
                self.register_function_temp(
                    module,
                    temp_name.clone(),
                    var_decl.var_type,
                    item.span,
                )?;
                frame.insert(
                    item.name.clone(),
                    Expression::Identifier(Identifier {
                        name: temp_name.clone(),
                        span: item.span,
                    }),
                );
            }
        }

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let body_result = (|| -> CompileResult<()> {
            for var_decl in &func.locals {
                for item in &var_decl.items {
                    if func.params.iter().any(|param| param.name == item.name) {
                        continue;
                    }
                    let target_name = self.resolve_substituted_name(&item.name);
                    let value = item
                        .init
                        .clone()
                        .unwrap_or_else(|| Self::number_expr(0.0, item.span));
                    let assignment = AssignmentStmt {
                        target: LValue::Variable {
                            name: target_name,
                            span: item.span,
                        },
                        value,
                        span: item.span,
                    };
                    self.analyze_assignment(&assignment, module, sink)?;
                }
            }
            for statement in &func.body.statements {
                self.analyze_statement(statement, module, sink)?;
            }
            Ok(())
        })();
        self.inline_depth -= 1;
        self.subst_stack.pop().expect("function frame");
        body_result?;

        for (formal, target, span) in output_bindings {
            let formal_temp = formal_temps.get(&formal).expect("formal temp registered");
            let assignment = AssignmentStmt {
                target,
                value: Expression::Identifier(Identifier {
                    name: formal_temp.clone(),
                    span,
                }),
                span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        Ok(Some(Expression::Identifier(Identifier {
            name: return_name,
            span: call.span,
        })))
    }

    fn lower_expression_with_side_effects(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        let expr = self.materialize_output_function_calls(expr, module, sink)?;
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(&expr)?;
        let side_effects = self.function_side_effects.split_off(side_effect_start);
        for assignment in side_effects {
            self.analyze_assignment(&assignment, module, sink)?;
        }
        Ok(lowered)
    }

    fn materialize_output_function_calls(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => expr.clone(),
            Expression::SystemFunction(function) => Expression::SystemFunction(SystemFunction {
                name: function.name.clone(),
                args: function
                    .args
                    .iter()
                    .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                    .collect::<CompileResult<Vec<_>>>()?,
                span: function.span,
            }),
            Expression::Binary(binary) => Expression::Binary(BinaryExpr {
                op: binary.op,
                left: Box::new(self.materialize_output_function_calls(
                    &binary.left,
                    module,
                    sink,
                )?),
                right: Box::new(self.materialize_output_function_calls(
                    &binary.right,
                    module,
                    sink,
                )?),
                span: binary.span,
            }),
            Expression::Unary(unary) => Expression::Unary(UnaryExpr {
                op: unary.op,
                operand: Box::new(self.materialize_output_function_calls(
                    &unary.operand,
                    module,
                    sink,
                )?),
                span: unary.span,
            }),
            Expression::Conditional(conditional) => {
                if self.expression_contains_output_function_call(expr) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "analog function output/inout arguments are not supported inside conditional expressions".into(),
                        ),
                        conditional.span,
                    )));
                }
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(self.materialize_output_function_calls(
                        &conditional.condition,
                        module,
                        sink,
                    )?),
                    then_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.then_expr,
                        module,
                        sink,
                    )?),
                    else_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.else_expr,
                        module,
                        sink,
                    )?),
                    span: conditional.span,
                })
            }
            Expression::Call(call) => {
                if let Some(func) = self.user_functions.get(&call.name).cloned()
                    && Self::function_needs_materialization(&func)
                {
                    let args = if call.args.len() == func.params.len() {
                        call.args
                            .iter()
                            .zip(func.params.iter())
                            .map(|(arg, param)| match param.direction {
                                ParamDirection::Input => {
                                    self.materialize_output_function_calls(arg, module, sink)
                                }
                                ParamDirection::Output | ParamDirection::Inout => Ok(arg.clone()),
                            })
                            .collect::<CompileResult<Vec<_>>>()?
                    } else {
                        call.args.clone()
                    };
                    let call = Expression::Call(CallExpr {
                        name: call.name.clone(),
                        args,
                        span: call.span,
                    });
                    return self
                        .materialize_output_function_call(&call, module, sink)?
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidAnalogOperator(
                                    "analog function output/inout call could not be materialized"
                                        .into(),
                                ),
                                expr.span(),
                            ))
                        });
                }
                Expression::Call(CallExpr {
                    name: call.name.clone(),
                    args: call
                        .args
                        .iter()
                        .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: call.span,
                })
            }
            Expression::ArrayAccess(access) => Expression::ArrayAccess(ArrayAccessExpr {
                array: access.array.clone(),
                index: Box::new(self.materialize_output_function_calls(
                    &access.index,
                    module,
                    sink,
                )?),
                span: access.span,
            }),
            Expression::ArrayLiteral(array) => Expression::ArrayLiteral(ArrayLiteralExpr {
                elements: array
                    .elements
                    .iter()
                    .map(|element| self.materialize_output_function_calls(element, module, sink))
                    .collect::<CompileResult<Vec<_>>>()?,
                span: array.span,
            }),
            Expression::AnalogOperator(op) => Expression::AnalogOperator(
                self.materialize_output_function_calls_in_analog_operator(op, module, sink)?,
            ),
            Expression::NoiseSource(noise) => Expression::NoiseSource(
                self.materialize_output_function_calls_in_noise_source(noise, module, sink)?,
            ),
        })
    }

    fn materialize_output_function_calls_in_analog_operator(
        &mut self,
        op: &AnalogOperator,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<AnalogOperator> {
        Ok(match op {
            AnalogOperator::Ddt { expr, abstol, span } => AnalogOperator::Ddt {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                span,
            } => AnalogOperator::Idt {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                ic: self.materialize_output_function_calls_opt_box(ic, module, sink)?,
                assert_val: self
                    .materialize_output_function_calls_opt_box(assert_val, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                span,
            } => AnalogOperator::IdtMod {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                ic: self.materialize_output_function_calls_opt_box(ic, module, sink)?,
                modulus: self.materialize_output_function_calls_opt_box(modulus, module, sink)?,
                offset: self.materialize_output_function_calls_opt_box(offset, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::Ddx { expr, probe, span } => AnalogOperator::Ddx {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                probe: probe.clone(),
                span: *span,
            },
            AnalogOperator::Limexp { expr, span } => AnalogOperator::Limexp {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                span: *span,
            },
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                span,
            } => AnalogOperator::Absdelay {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                delay: self.materialize_output_function_calls_box(delay, module, sink)?,
                max_delay: self
                    .materialize_output_function_calls_opt_box(max_delay, module, sink)?,
                span: *span,
            },
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                span,
            } => AnalogOperator::Transition {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                delay: self.materialize_output_function_calls_opt_box(delay, module, sink)?,
                rise: self.materialize_output_function_calls_opt_box(rise, module, sink)?,
                fall: self.materialize_output_function_calls_opt_box(fall, module, sink)?,
                tolerance: self
                    .materialize_output_function_calls_opt_box(tolerance, module, sink)?,
                span: *span,
            },
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                span,
            } => AnalogOperator::Slew {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                max_rise: self.materialize_output_function_calls_opt_box(max_rise, module, sink)?,
                max_fall: self.materialize_output_function_calls_opt_box(max_fall, module, sink)?,
                span: *span,
            },
            AnalogOperator::LastCrossing { expr, edge, span } => AnalogOperator::LastCrossing {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                edge: *edge,
                span: *span,
            },
            AnalogOperator::Laplace { kind, expr, span } => AnalogOperator::Laplace {
                kind: self.materialize_output_function_calls_in_laplace_kind(kind, module, sink)?,
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                span: *span,
            },
            AnalogOperator::Zi { kind, expr, span } => AnalogOperator::Zi {
                kind: self.materialize_output_function_calls_in_zi_kind(kind, module, sink)?,
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_box(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Box<Expression>> {
        Ok(Box::new(
            self.materialize_output_function_calls(expr, module, sink)?,
        ))
    }

    fn materialize_output_function_calls_opt_box(
        &mut self,
        expr: &Option<Box<Expression>>,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Box<Expression>>> {
        expr.as_deref()
            .map(|expr| self.materialize_output_function_calls_box(expr, module, sink))
            .transpose()
    }

    fn materialize_output_function_calls_in_laplace_kind(
        &mut self,
        kind: &LaplaceKind,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<LaplaceKind> {
        Ok(match kind {
            LaplaceKind::ZeroPole { zeros, poles } => LaplaceKind::ZeroPole {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            LaplaceKind::ZeroDenominator { zeros, denominator } => LaplaceKind::ZeroDenominator {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
            LaplaceKind::NumeratorPole { numerator, poles } => LaplaceKind::NumeratorPole {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => LaplaceKind::NumeratorDenominator {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
        })
    }

    fn materialize_output_function_calls_in_zi_kind(
        &mut self,
        kind: &ZiKind,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<ZiKind> {
        Ok(match kind {
            ZiKind::ZeroPole { zeros, poles } => ZiKind::ZeroPole {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            ZiKind::ZeroDenominator { zeros, denominator } => ZiKind::ZeroDenominator {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
            ZiKind::NumeratorPole { numerator, poles } => ZiKind::NumeratorPole {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => ZiKind::NumeratorDenominator {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
        })
    }

    fn materialize_output_function_calls_in_noise_source(
        &mut self,
        noise: &NoiseSource,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<NoiseSource> {
        Ok(match noise {
            NoiseSource::White { power, name, span } => NoiseSource::White {
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Flicker {
                power,
                exponent,
                name,
                span,
            } => NoiseSource::Flicker {
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                exponent: Box::new(self.materialize_output_function_calls(exponent, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Table { data, name, span } => NoiseSource::Table {
                data: self.materialize_output_function_calls_in_expr_list(data, module, sink)?,
                name: name.clone(),
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_in_expr_list(
        &mut self,
        exprs: &[Expression],
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Vec<Expression>> {
        exprs
            .iter()
            .map(|expr| self.materialize_output_function_calls(expr, module, sink))
            .collect()
    }

    fn expression_contains_output_function_call(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Call(call) => {
                self.user_functions
                    .get(&call.name)
                    .is_some_and(Self::function_needs_materialization)
                    || call
                        .args
                        .iter()
                        .any(|arg| self.expression_contains_output_function_call(arg))
            }
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| self.expression_contains_output_function_call(arg)),
            Expression::Binary(binary) => {
                self.expression_contains_output_function_call(&binary.left)
                    || self.expression_contains_output_function_call(&binary.right)
            }
            Expression::Unary(unary) => {
                self.expression_contains_output_function_call(&unary.operand)
            }
            Expression::Conditional(conditional) => {
                self.expression_contains_output_function_call(&conditional.condition)
                    || self.expression_contains_output_function_call(&conditional.then_expr)
                    || self.expression_contains_output_function_call(&conditional.else_expr)
            }
            Expression::ArrayAccess(access) => {
                self.expression_contains_output_function_call(&access.index)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| self.expression_contains_output_function_call(element)),
            Expression::AnalogOperator(op) => {
                self.analog_operator_contains_output_function_call(op)
            }
            Expression::NoiseSource(noise) => {
                self.noise_source_contains_output_function_call(noise)
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => false,
        }
    }

    fn analog_operator_contains_output_function_call(&self, op: &AnalogOperator) -> bool {
        let contains_opt = |expr: &Option<Box<Expression>>| {
            expr.as_ref()
                .is_some_and(|expr| self.expression_contains_output_function_call(expr))
        };
        match op {
            AnalogOperator::Ddt { expr, abstol, .. } => {
                self.expression_contains_output_function_call(expr) || contains_opt(abstol)
            }
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(ic)
                    || contains_opt(assert_val)
                    || contains_opt(abstol)
            }
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(ic)
                    || contains_opt(modulus)
                    || contains_opt(offset)
                    || contains_opt(abstol)
            }
            AnalogOperator::Ddx { expr, .. }
            | AnalogOperator::Limexp { expr, .. }
            | AnalogOperator::LastCrossing { expr, .. } => {
                self.expression_contains_output_function_call(expr)
            }
            AnalogOperator::Laplace { kind, expr, .. } => {
                self.expression_contains_output_function_call(expr)
                    || self.laplace_kind_contains_output_function_call(kind)
            }
            AnalogOperator::Zi { kind, expr, .. } => {
                self.expression_contains_output_function_call(expr)
                    || self.zi_kind_contains_output_function_call(kind)
            }
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || self.expression_contains_output_function_call(delay)
                    || contains_opt(max_delay)
            }
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(delay)
                    || contains_opt(rise)
                    || contains_opt(fall)
                    || contains_opt(tolerance)
            }
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(max_rise)
                    || contains_opt(max_fall)
            }
        }
    }

    fn laplace_kind_contains_output_function_call(&self, kind: &LaplaceKind) -> bool {
        match kind {
            LaplaceKind::ZeroPole { zeros, poles } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(poles)
            }
            LaplaceKind::ZeroDenominator { zeros, denominator } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(denominator)
            }
            LaplaceKind::NumeratorPole { numerator, poles } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(poles)
            }
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(denominator)
            }
        }
    }

    fn zi_kind_contains_output_function_call(&self, kind: &ZiKind) -> bool {
        match kind {
            ZiKind::ZeroPole { zeros, poles } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(poles)
            }
            ZiKind::ZeroDenominator { zeros, denominator } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(denominator)
            }
            ZiKind::NumeratorPole { numerator, poles } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(poles)
            }
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(denominator)
            }
        }
    }

    fn expr_list_contains_output_function_call(&self, exprs: &[Expression]) -> bool {
        exprs
            .iter()
            .any(|expr| self.expression_contains_output_function_call(expr))
    }

    fn noise_source_contains_output_function_call(&self, noise: &NoiseSource) -> bool {
        match noise {
            NoiseSource::White { power, .. } => {
                self.expression_contains_output_function_call(power)
            }
            NoiseSource::Flicker {
                power, exponent, ..
            } => {
                self.expression_contains_output_function_call(power)
                    || self.expression_contains_output_function_call(exponent)
            }
            NoiseSource::Table { data, .. } => data
                .iter()
                .any(|expr| self.expression_contains_output_function_call(expr)),
        }
    }

    fn lower_expression_without_side_effects(
        &mut self,
        expr: &Expression,
        context: &str,
    ) -> CompileResult<Expression> {
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(expr)?;
        if self.function_side_effects.len() != side_effect_start {
            self.function_side_effects.truncate(side_effect_start);
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function output/inout arguments are not supported in {context}"
                )),
                expr.span(),
            )));
        }
        Ok(lowered)
    }

    /// Rewrite an expression: apply substitutions (block locals, loop
    /// variables) and inline calls to user-defined analog functions.
    fn lower_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Identifier(id) => match self.lookup_substitution(&id.name) {
                Some(subst) => subst,
                None => expr.clone(),
            },
            Expression::Number(_) | Expression::StringLit(_) => expr.clone(),
            Expression::BranchAccess(access) => {
                self.validate_branch_access_compatible(access, access.span())?;
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
                self.validate_builtin_call_arity(call)?;

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
                    let access_expr = BranchAccess::Nodes {
                        access: call.name.clone(),
                        pos: nodes.next().unwrap(),
                        neg: nodes.next(),
                        span: call.span,
                    };
                    self.validate_branch_access_compatible(&access_expr, call.span)?;
                    return Ok(Expression::BranchAccess(access_expr));
                }

                if let Some(func) = self.user_functions.get(&call.name) {
                    if Self::is_recognized_limited_exp_function(func) {
                        let args = call
                            .args
                            .iter()
                            .map(|a| self.lower_expression(a))
                            .collect::<CompileResult<Vec<_>>>()?;
                        return Ok(Expression::Call(CallExpr {
                            name: RSPICE_LIMITED_EXP_INTRINSIC.into(),
                            args,
                            span: call.span,
                        }));
                    }
                    if Self::function_needs_materialization(func) {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "analog function '{}': output/inout calls must be lowered at a statement boundary",
                                call.name
                            )),
                            call.span,
                        )));
                    }
                    self.inline_function(&call.name, &call.args, call.span)?
                } else {
                    let args = call
                        .args
                        .iter()
                        .map(|a| self.lower_expression(a))
                        .collect::<CompileResult<Vec<_>>>()?;
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

    fn validate_builtin_call_arity(&self, call: &CallExpr) -> CompileResult<()> {
        let Some(signature) = self.functions.get(&call.name) else {
            return Ok(());
        };

        let min_args = signature.min_args();
        let max_args = signature.max_args();
        let got = call.args.len();
        if got < min_args || got > max_args {
            let expected = if min_args == max_args {
                min_args.to_string()
            } else {
                format!("{min_args}..{max_args}")
            };
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected,
                    got,
                },
                call.span,
            )));
        }

        Ok(())
    }

    const MAX_INLINE_DEPTH: usize = 16;

    /// Inline a call to a user-defined analog function by symbolically
    /// executing its body. The return value is the final expression bound
    /// to the function-name variable.
    fn inline_function(
        &mut self,
        name: &SmolStr,
        args: &[Expression],
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

        if args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: name.to_string(),
                    expected: func.params.len().to_string(),
                    got: args.len(),
                },
                span,
            )));
        }

        // Bind parameters and locals in a fresh substitution frame
        let mut frame = HashMap::new();
        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            match param.direction {
                ParamDirection::Input => {
                    frame.insert(param.name.clone(), self.lower_expression(arg)?);
                }
                ParamDirection::Output => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    frame.insert(param.name.clone(), Self::number_expr(0.0, param.span));
                    output_bindings.push((param.name.clone(), target, param.span));
                }
                ParamDirection::Inout => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    frame.insert(param.name.clone(), self.lower_expression(arg)?);
                    output_bindings.push((param.name.clone(), target, param.span));
                }
            }
        }
        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|p| p.name == item.name) {
                    continue;
                }
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

        for (formal, target, span) in output_bindings {
            let value = frame
                .get(&formal)
                .cloned()
                .unwrap_or_else(|| Self::number_expr(0.0, span));
            self.function_side_effects.push(AssignmentStmt {
                target,
                value,
                span,
            });
        }

        Ok(frame
            .get(&func.name)
            .cloned()
            .unwrap_or_else(|| Self::number_expr(0.0, span)))
    }

    fn function_output_lvalue(
        &mut self,
        function_name: &SmolStr,
        param: &FunctionParam,
        arg: &Expression,
    ) -> CompileResult<LValue> {
        match arg {
            Expression::Identifier(id) => {
                let resolved = self.resolve_substituted_name(&id.name);
                Ok(LValue::Variable {
                    name: resolved,
                    span: id.span,
                })
            }
            Expression::ArrayAccess(access) => {
                let array = self.resolve_substituted_name(&access.array);
                let index = self.lower_expression_without_side_effects(
                    &access.index,
                    "analog function output argument index",
                )?;
                Ok(LValue::ArrayAccess {
                    name: array,
                    index: Box::new(index),
                    span: access.span,
                })
            }
            _ => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function '{}': {} argument '{}' must be an assignable variable",
                    function_name,
                    match param.direction {
                        ParamDirection::Output => "output",
                        ParamDirection::Inout => "inout",
                        ParamDirection::Input => "input",
                    },
                    param.name
                )),
                arg.span(),
            ))),
        }
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
                let value = self
                    .lower_expression_without_side_effects(&assign.value, "analog function body")?;
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
                let condition = self.lower_expression_without_side_effects(
                    &cond.condition,
                    "analog function body",
                )?;
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
                let selector = self.lower_expression_without_side_effects(
                    &case_stmt.expr,
                    "analog function body",
                )?;
                let mut prior_match: Option<Expression> = None;
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered =
                            self.lower_expression_without_side_effects(m, "analog function body")?;
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
                            Some(init) => self.lower_expression_without_side_effects(
                                init,
                                "analog function body",
                            )?,
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
            AnalogStatement::Null(_) => {}
            AnalogStatement::Call(call) => self.validate_no_effect_system_task(call)?,
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

    fn is_no_effect_system_task(name: &str) -> bool {
        matches!(
            name,
            "$display"
                | "$error"
                | "$fatal"
                | "$finish"
                | "$info"
                | "$monitor"
                | "$stop"
                | "$strobe"
                | "$warning"
                | "$write"
        )
    }

    fn validate_no_effect_system_task(&self, call: &CallStmt) -> CompileResult<()> {
        if Self::is_no_effect_system_task(call.name.as_str()) {
            return Ok(());
        }

        if matches!(call.name.as_str(), "$bound_step" | "$discontinuity") {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "system task '{}' is not supported inside analog functions",
                    call.name
                )),
                call.span,
            )));
        }

        Err(Self::unknown_system_task_error(call))
    }

    fn validate_system_task_arity(
        &self,
        call: &CallStmt,
        min_args: usize,
        max_args: Option<usize>,
    ) -> CompileResult<()> {
        let got = call.args.len();
        let too_many = max_args.is_some_and(|max| got > max);
        if got >= min_args && !too_many {
            return Ok(());
        }

        let expected = match max_args {
            Some(max) if min_args == max => min_args.to_string(),
            Some(max) => format!("{min_args}..{max}"),
            None => format!("{min_args}+"),
        };
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::ArgumentCountMismatch {
                name: call.name.to_string(),
                expected,
                got,
            },
            call.span,
        )))
    }

    fn unknown_system_task_error(call: &CallStmt) -> CompileError {
        CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::UnknownFunction(call.name.to_string()),
            call.span,
        ))
    }

    fn validate_node(&self, name: &str, span: Span) -> CompileResult<()> {
        if is_global_ground_name(name) {
            return Ok(());
        }
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

fn is_global_ground_name(name: &str) -> bool {
    name == "0"
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
