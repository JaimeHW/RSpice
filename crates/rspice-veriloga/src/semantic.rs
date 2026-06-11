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
}

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
    pub variables: Vec<AnalyzedVariable>,
    pub branches: Vec<AnalyzedBranch>,
    pub contributions: Vec<AnalyzedContribution>,
    pub assignments: Vec<AnalyzedAssignment>,
    pub internal_nodes: Vec<AnalyzedInternalNode>,
    pub symbol_table: SymbolTable,
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
    pub default: Option<f64>,
    pub range: Option<TypedParameterRange>,
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
    pub expression: Expression,
    pub expr_type: ValueType,
    pub span: Span,
}

/// Analyzed variable assignment
#[derive(Debug, Clone)]
pub struct AnalyzedAssignment {
    /// Variable name being assigned
    pub target: SmolStr,
    /// Index of variable in variables list
    pub var_index: usize,
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
            variables: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            assignments: Vec::new(),
            internal_nodes: Vec::new(),
            symbol_table: SymbolTable::new(),
        };

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

        // Phase 5: Define internal nodes (nets that aren't ports)
        let mut internal_node_idx = 0usize;
        for net in &module.nets {
            let discipline = net.discipline.clone();
            for name in &net.names {
                // Skip if already defined as a port
                if self.symbols.lookup_local(name).is_some() {
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

        // Phase 5: Analyze parameters
        for param in &module.parameters {
            let value_type = match param.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            let default = param.default.as_ref().and_then(|e| self.eval_const(e));

            // Parse parameter range if present
            let range = param.range.as_ref().map(|r| self.parse_range(r));

            // Validate default against range
            if let (Some(default_val), Some(range_constraint)) = (default, &range)
                && !range_constraint.contains(default_val)
            {
                self.record_error(SemanticErrorKind::ParameterOutOfRange {
                    name: param.name.clone(),
                    value: default_val,
                    range: format!("{}", range_constraint),
                });
            }

            analyzed.parameters.push(AnalyzedParameter {
                name: param.name.clone(),
                param_type: param.param_type,
                value_type,
                default,
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

        // Phase 6: Analyze variables
        for var_decl in &module.variables {
            let value_type = match var_decl.var_type {
                VarType::Real => ValueType::Real,
                VarType::Integer => ValueType::Integer,
                VarType::String => ValueType::String,
            };

            for item in &var_decl.items {
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

        // Phase 7: Analyze analog block
        if let Some(block) = &module.analog_block {
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed)?;
            }
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

    fn analyze_statement(
        &mut self,
        stmt: &AnalogStatement,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Contribution(contrib) => {
                self.analyze_contribution(contrib, module)?;
            }
            AnalogStatement::Assignment(assign) => {
                self.analyze_assignment(assign, module)?;
            }
            AnalogStatement::Block(block) => {
                self.symbols.enter_scope();
                for s in &block.statements {
                    self.analyze_statement(s, module)?;
                }
                self.symbols.exit_scope();
            }
            AnalogStatement::Conditional(cond) => {
                let cond_type = self.infer_type(&cond.condition)?;
                if !cond_type.is_condition() {
                    self.record_error(SemanticErrorKind::InvalidCondition {
                        found: cond_type.to_string(),
                    });
                }
                self.analyze_statement(&cond.then_branch, module)?;
                if let Some(else_branch) = &cond.else_branch {
                    self.analyze_statement(else_branch, module)?;
                }
            }
            AnalogStatement::For(for_stmt) => {
                self.symbols.enter_scope();

                // Check condition type
                let cond_type = self.infer_type(&for_stmt.condition)?;
                if !cond_type.is_condition() {
                    self.record_error(SemanticErrorKind::InvalidCondition {
                        found: cond_type.to_string(),
                    });
                }

                self.analyze_statement(&for_stmt.body, module)?;
                self.symbols.exit_scope();
            }
            AnalogStatement::While(while_stmt) => {
                self.symbols.enter_scope();

                let cond_type = self.infer_type(&while_stmt.condition)?;
                if !cond_type.is_condition() {
                    self.record_error(SemanticErrorKind::InvalidCondition {
                        found: cond_type.to_string(),
                    });
                }

                self.analyze_statement(&while_stmt.body, module)?;
                self.symbols.exit_scope();
            }
            AnalogStatement::Case(case_stmt) => {
                let _selector_type = self.infer_type(&case_stmt.expr)?;
                for item in &case_stmt.items {
                    self.analyze_statement(&item.statement, module)?;
                }
                if let Some(default) = &case_stmt.default {
                    self.analyze_statement(default, module)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn analyze_contribution(
        &mut self,
        contrib: &ContributionStmt,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        let (branch_name, is_current) = match &contrib.target {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                let is_current = access.as_str() == "I";
                self.validate_node(pos, contrib.span)?;
                if let Some(n) = neg {
                    self.validate_node(n, contrib.span)?;
                }
                // Format as "pos,neg" for IR parser compatibility
                let branch = if neg.is_some() {
                    format!("{},{}", pos, neg.as_deref().unwrap())
                } else {
                    pos.to_string()
                };
                (branch.into(), is_current)
            }
            BranchAccess::Branch { name, access, .. } => (name.clone(), access.as_str() == "I"),
        };

        let expr_type = self.infer_type(&contrib.value)?;
        if !expr_type.is_numeric() {
            self.record_error(SemanticErrorKind::TypeMismatch {
                expected: "numeric".to_string(),
                found: expr_type.to_string(),
                context: "contribution expression".to_string(),
            });
        }

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            is_current,
            expression: contrib.value.clone(),
            expr_type,
            span: contrib.span,
        });

        Ok(())
    }

    fn analyze_assignment(
        &mut self,
        assign: &AssignmentStmt,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        let (target_name, span) = match &assign.target {
            LValue::Variable { name, span } => {
                if self.symbols.lookup(name).is_none() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        *span,
                    )));
                }
                self.symbols.mark_used(name);
                (name.clone(), *span)
            }
            LValue::ArrayAccess { name, span, .. } => {
                if self.symbols.lookup(name).is_none() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        *span,
                    )));
                }
                self.symbols.mark_used(name);
                (name.clone(), *span)
            }
        };

        let value_type = self.infer_type(&assign.value)?;

        if let Some(sym) = self.symbols.lookup(&target_name)
            && !value_type.can_coerce_to(&sym.value_type)
        {
            self.record_error(SemanticErrorKind::TypeMismatch {
                expected: sym.value_type.to_string(),
                found: value_type.to_string(),
                context: format!("assignment to '{}'", target_name),
            });
        }

        // Find variable index
        let var_index = module
            .variables
            .iter()
            .position(|v| v.name == target_name)
            .unwrap_or(0);

        // Record the assignment for code generation
        module.assignments.push(AnalyzedAssignment {
            target: target_name,
            var_index,
            expression: assign.value.clone(),
            expr_type: value_type,
            span,
        });

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
            Expression::ArrayAccess(_) => Ok(ValueType::Unknown),
            Expression::ArrayLiteral(_) => Ok(ValueType::Unknown),
            Expression::AnalogOperator(_) => Ok(ValueType::Real),
            Expression::NoiseSource(_) => Ok(ValueType::Real),
        }
    }

    fn eval_const(&self, expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(n) => Some(n.value),
            Expression::Unary(u) if u.op == UnaryOp::Neg => self.eval_const(&u.operand).map(|v| -v),
            Expression::Binary(b) => {
                let l = self.eval_const(&b.left)?;
                let r = self.eval_const(&b.right)?;
                Some(match b.op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Pow => l.powf(r),
                    _ => return None,
                })
            }
            Expression::Identifier(ident) => match ident.name.as_str() {
                "inf" => Some(f64::INFINITY),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_range(&self, range: &ParameterRange) -> TypedParameterRange {
        // Extract bounds from first range bound if present
        if let Some(bound) = range.bounds.first() {
            let min = bound.lower.as_ref().and_then(|e| self.eval_const(e));
            let max = bound.upper.as_ref().and_then(|e| self.eval_const(e));
            let exclude: Vec<f64> = range
                .exclude
                .iter()
                .filter_map(|e| self.eval_const(e))
                .collect();

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

    fn record_error(&mut self, kind: SemanticErrorKind) {
        self.errors.push(SemanticError::new(
            kind,
            Span::new(crate::source::SourceId::new(0), 0, 0),
        ));
    }
}

// ============================================================================
// Tests
// ============================================================================
