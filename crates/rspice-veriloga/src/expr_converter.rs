//! Expression Converter for Verilog-A
//!
//! Converts AST expressions to IR expressions for code generation.
//! This module handles:
//! - Node name to terminal index mapping
//! - Parameter and variable resolution
//! - System function conversion ($vt, $temperature)
//! - Analog operator translation (ddt, idt, limexp)

use crate::ast::AccessKind;
use crate::ast::{
    AnalogOperator, ArrayLiteralElement, BinaryOp, BranchAccess, CallExpr, Expression, Identifier,
    LimiterArgument, NumberLit, SystemFunction,
};
use crate::branch_identity::BranchIdentity;
use crate::error::{CodeGenError, CodeGenErrorKind, CompileResult};
use crate::ir::arena::{ExprArena, Heavy, IndexedRead, Node, ZiPolynomial, pack_index};
use crate::ir::{BranchRef, DdxAxis, IrFunction, NodeId};
use crate::semantic::AnalyzedModule;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::path::Path;

/// Constant-fold an arena expression (used for filter coefficients and
/// constant direction arguments)
fn autodiff_fold(arena: &mut ExprArena, expr: NodeId) -> NodeId {
    crate::ir::autodiff::simplify(arena, expr)
}

/// The value of a node that folded to a constant, if it did.
fn constant(arena: &ExprArena, expr: NodeId) -> Option<f64> {
    match *arena.node(expr) {
        Node::Const(value) => Some(value),
        _ => None,
    }
}

fn zi_polynomial_is_wholly_constant(arena: &ExprArena, definition: &ZiPolynomial) -> bool {
    match definition {
        ZiPolynomial::Coefficients(values) => {
            values.iter().all(|value| constant(arena, *value).is_some())
        }
        ZiPolynomial::Roots(values) => values.iter().all(|(real, imaginary)| {
            constant(arena, *real).is_some() && constant(arena, *imaginary).is_some()
        }),
    }
}

fn zi_polynomial_scalar_count(
    definition: &ZiPolynomial,
) -> Result<usize, crate::zfilter::ZiFilterError> {
    match definition {
        ZiPolynomial::Coefficients(values) => Ok(values.len()),
        ZiPolynomial::Roots(values) => values.len().checked_mul(2).ok_or_else(|| {
            crate::zfilter::ZiFilterError::InvalidDefinition(
                "Zi complex-root scalar count overflows usize".into(),
            )
        }),
    }
}

fn validate_zi_polynomial_budget(
    operator: &str,
    numerator: &ZiPolynomial,
    denominator: &ZiPolynomial,
) -> CompileResult<()> {
    let numerator = zi_polynomial_scalar_count(numerator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string()))
    })?;
    let denominator = zi_polynomial_scalar_count(denominator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string()))
    })?;
    crate::zfilter::validate_zi_runtime_operand_budget(operator, numerator, denominator)
        .map(|_| ())
        .map_err(|error| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string())).into()
        })
}

fn expand_constant_zi_polynomial(
    arena: &ExprArena,
    definition: &ZiPolynomial,
) -> Result<Vec<f64>, String> {
    match definition {
        ZiPolynomial::Coefficients(values) => Ok(values
            .iter()
            .map(|value| {
                constant(arena, *value)
                    .ok_or_else(|| "Zi coefficient unexpectedly remained dynamic".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?),
        ZiPolynomial::Roots(values) => {
            let roots = values
                .iter()
                .map(|(real, imaginary)| {
                    match (constant(arena, *real), constant(arena, *imaginary)) {
                        (Some(real), Some(imaginary)) => Ok((real, imaginary)),
                        _ => Err("Zi root unexpectedly remained dynamic".to_string()),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            crate::zfilter::z_roots_to_coefficients(&roots)
        }
    }
}

/// Eagerly validate a Zi definition only when every frozen argument is known
/// at compile time. Any parameter-, variable-, or circuit-dependent operand
/// remains deferred to the per-instance analysis-start freeze.
fn validate_wholly_constant_zi_definition(
    arena: &ExprArena,
    operator: &str,
    numerator: &ZiPolynomial,
    denominator: &ZiPolynomial,
    period: NodeId,
    first_transition: NodeId,
) -> CompileResult<()> {
    let (Some(period), Some(first_transition)) =
        (constant(arena, period), constant(arena, first_transition))
    else {
        return Ok(());
    };
    if !zi_polynomial_is_wholly_constant(arena, numerator)
        || !zi_polynomial_is_wholly_constant(arena, denominator)
    {
        return Ok(());
    }
    let numerator = expand_constant_zi_polynomial(arena, numerator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
            "{operator} numerator: {error}"
        )))
    })?;
    let denominator = expand_constant_zi_polynomial(arena, denominator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
            "{operator} denominator: {error}"
        )))
    })?;
    crate::zfilter::ZiFilter::new_with_timing(numerator, denominator, period, first_transition)
        .map(|_| ())
        .map_err(|error| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "{operator}: {error}"
            )))
            .into()
        })
}

fn laplace_error(
    operator: &str,
    error: crate::laplace::LaplaceError,
) -> crate::error::CompileError {
    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
        "{operator}: {error}"
    )))
    .into()
}

fn validate_laplace_coefficients(
    operator: &str,
    numerator_ascending: &[f64],
    denominator_ascending: &[f64],
) -> CompileResult<()> {
    let mut numerator = numerator_ascending.to_vec();
    numerator.reverse();
    let mut denominator = denominator_ascending.to_vec();
    denominator.reverse();
    crate::laplace::StateSpaceFilter::from_transfer_function(&numerator, &denominator)
        .map(|_| ())
        .map_err(|error| laplace_error(operator, error))
}

fn validate_arg_range(
    name: &str,
    actual: usize,
    min: usize,
    max: Option<usize>,
) -> CompileResult<()> {
    let too_many = match max {
        Some(max) => actual > max,
        None => false,
    };
    if actual < min || too_many {
        let expected = match max {
            Some(max) if min == max => min.to_string(),
            Some(max) => format!("{min}..{max}"),
            None => format!("{min}+"),
        };
        return Err(
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "{name} expects {expected} argument(s), got {actual}"
            )))
            .into(),
        );
    }
    Ok(())
}

fn validate_event_argument_dependencies(name: &str, args: &[Expression]) -> CompileResult<()> {
    let defined = |index: usize| {
        args.get(index)
            .is_some_and(|argument| !matches!(argument, Expression::NullArgument(_)))
    };
    match name {
        "cross" => {
            if (defined(2) || defined(3)) && !defined(1) {
                return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "cross tolerances require a defined direction".into(),
                ))
                .into());
            }
            if defined(3) && !defined(2) {
                return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "cross expr_tol requires time_tol".into(),
                ))
                .into());
            }
        }
        "above" if defined(2) && !defined(1) => {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "above expr_tol requires time_tol".into(),
            ))
            .into());
        }
        _ => {}
    }
    Ok(())
}

fn normalize_analysis_name(name: &str) -> Option<String> {
    rspice_veriloga_runtime::analysis_query_id(name)
        .and_then(rspice_veriloga_runtime::analysis_query_name)
        .map(str::to_string)
}

fn analysis_expression(
    arena: &mut ExprArena,
    name: &str,
    args: &[Expression],
) -> CompileResult<NodeId> {
    validate_arg_range(name, args.len(), 1, None)?;
    let query = |arena: &mut ExprArena, arg: &Expression| -> CompileResult<NodeId> {
        let Expression::StringLit(value) = arg else {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "analysis() requires string arguments".into(),
            ))
            .into());
        };
        Ok(match normalize_analysis_name(value.value.as_str()) {
            Some(name) => {
                let name = arena.intern(&name);
                arena.push(Node::Analysis(name))
            }
            None => arena.push(Node::Const(0.0)),
        })
    };
    let Some(first) = args.first() else {
        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
            "analysis() requires at least one string argument".into(),
        ))
        .into());
    };
    let mut expression = query(arena, first)?;
    for arg in &args[1..] {
        let next = query(arena, arg)?;
        expression = arena.push(Node::Binary(BinaryOp::Or, expression, next));
    }
    Ok(expression)
}

fn optional_string_arg(
    function_name: &str,
    arg: Option<&Expression>,
    role: &str,
) -> CompileResult<Option<String>> {
    match arg {
        None => Ok(None),
        Some(Expression::StringLit(s)) => Ok(Some(s.value.to_string())),
        Some(_) => Err(
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "{function_name} {role} argument must be a string literal"
            )))
            .into(),
        ),
    }
}

/// Sentinel node index for the global reference (ground) node.
///
/// `V(a)` measures the potential of `a` against this reference, never
/// against terminal 0.
pub const GROUND_NODE: usize = usize::MAX;

/// Context for expression conversion
///
/// Maintains mappings from names to indices for efficient IR generation.
/// Nodes occupy a unified index space: ports first (0..num_terminals),
/// then internal nodes (num_terminals..num_terminals+num_internal).
/// Ground nets map to [`GROUND_NODE`].
#[derive(Debug)]
pub struct ConversionContext {
    /// Map from node name (port or internal) to unified node index
    node_map: HashMap<SmolStr, usize>,
    /// Map from named branch to its (pos, neg) node indices
    branch_map: HashMap<SmolStr, (usize, usize)>,
    /// Normalized branch pairs carrying solver-owned current unknowns, mapped
    /// to (ordinal, authored positive endpoint).
    branch_current_map: HashMap<BranchIdentity<usize>, (usize, usize)>,
    /// Map from parameter name to index
    param_map: HashMap<SmolStr, usize>,
    /// Map from variable name to index
    var_map: HashMap<SmolStr, usize>,
    /// Array layouts: name -> (base, lower, len)
    arrays: HashMap<SmolStr, (usize, i64, usize)>,
    /// Number of external terminals (ports)
    num_terminals: usize,
    /// Number of internal nodes
    num_internal: usize,
    /// Effective file-scoped default transition time in seconds.
    default_transition: f64,
}

impl ConversionContext {
    /// Create a new conversion context from an analyzed module
    pub fn from_module(module: &AnalyzedModule) -> Self {
        let mut node_map = HashMap::new();
        let mut param_map = HashMap::new();
        let mut var_map = HashMap::new();

        // Ports occupy node indices 0..P
        let num_terminals = module.ports.len();
        for (idx, port) in module.ports.iter().enumerate() {
            node_map.insert(port.name.clone(), idx);
        }

        // Internal nodes follow at P..P+N
        let num_internal = module.internal_nodes.len();
        for node in &module.internal_nodes {
            node_map.insert(node.name.clone(), num_terminals + node.index);
        }

        node_map.insert(SmolStr::from("0"), GROUND_NODE);

        // Ground nets reference the global ground sentinel
        for name in &module.ground_nodes {
            node_map.insert(name.clone(), GROUND_NODE);
        }

        // Named branches resolve to node pairs
        let mut branch_map = HashMap::new();
        for branch in &module.branches {
            let pos = node_map.get(&branch.pos_node).copied();
            let neg = if branch.neg_node.is_empty() {
                Some(GROUND_NODE)
            } else {
                node_map.get(&branch.neg_node).copied()
            };
            if let (Some(pos), Some(neg)) = (pos, neg) {
                branch_map.insert(branch.name.clone(), (pos, neg));
            }
        }

        let mut branch_current_map = HashMap::new();
        for contribution in &module.contributions {
            if contribution.is_current && !contribution.indirect {
                continue;
            }
            let mut endpoints = contribution.branch.split(',').map(str::trim);
            let Some(pos) = endpoints
                .next()
                .and_then(|name| node_map.get(name).copied())
            else {
                continue;
            };
            let Some(neg) = endpoints
                .next()
                .map(|name| node_map.get(name).copied())
                .unwrap_or(Some(GROUND_NODE))
            else {
                continue;
            };
            let key = BranchIdentity::new(contribution.declared_branch.as_ref(), pos, neg);
            if !branch_current_map.contains_key(&key) {
                let ordinal = branch_current_map.len();
                branch_current_map.insert(key, (ordinal, pos));
            }
        }

        // Map parameter names to indices
        for (idx, param) in module.parameters.iter().enumerate() {
            param_map.insert(param.name.clone(), idx);
        }

        // Map variable names to indices
        for (idx, var) in module.variables.iter().enumerate() {
            var_map.insert(var.name.clone(), idx);
        }

        // Array layouts (elements occupy contiguous variable slots)
        let mut arrays = HashMap::new();
        for (name, layout) in &module.arrays {
            arrays.insert(name.clone(), (layout.base, layout.lower, layout.len));
        }

        Self {
            node_map,
            branch_map,
            branch_current_map,
            param_map,
            var_map,
            arrays,
            num_terminals,
            num_internal,
            default_transition: module.default_transition,
        }
    }

    /// Get unified node index by name (port, internal node, or ground net)
    pub fn node_index(&self, name: &str) -> Option<usize> {
        self.node_map.get(name).copied()
    }

    /// Resolve a named branch to its (pos, neg) node indices
    pub fn branch_nodes(&self, name: &str) -> Option<(usize, usize)> {
        self.branch_map.get(name).copied()
    }

    fn branch_current_axis(
        &self,
        name: Option<&SmolStr>,
        pos: usize,
        neg: usize,
    ) -> Option<(usize, bool)> {
        let key = BranchIdentity::new(name, pos, neg);
        self.branch_current_map
            .get(&key)
            .map(|(ordinal, authored_pos)| (*ordinal, pos != *authored_pos))
    }

    /// Get parameter index by name
    pub fn param_index(&self, name: &str) -> Option<usize> {
        self.param_map.get(name).copied()
    }

    /// Get variable index by name
    pub fn var_index(&self, name: &str) -> Option<usize> {
        self.var_map.get(name).copied()
    }

    /// Get an array layout (base, lower, len) by name
    pub fn array(&self, name: &str) -> Option<(usize, i64, usize)> {
        self.arrays.get(name).copied()
    }

    /// Global ground (reference) node index
    pub fn ground(&self) -> usize {
        GROUND_NODE
    }

    /// Number of external terminals
    pub fn num_terminals(&self) -> usize {
        self.num_terminals
    }

    /// Total number of nodes carrying unknowns (terminals + internal)
    pub fn num_nodes(&self) -> usize {
        self.num_terminals + self.num_internal
    }

    pub fn default_transition(&self) -> f64 {
        self.default_transition
    }
}

/// Expression converter
///
/// Converts AST expressions to IR expressions using the provided context.
pub struct ExprConverter<'a> {
    ctx: &'a ConversionContext,
    direct_zi_assignment: bool,
    /// Oriented proposals of the named limiters whose bodies are being
    /// converted, innermost last. A limiter body reads its implicit arguments
    /// from the enclosing call, and nested limiters accumulate, so the
    /// conversion carries the enclosing proposals rather than a single one.
    limiters: std::cell::RefCell<Vec<NodeId>>,
}

impl<'a> ExprConverter<'a> {
    /// Create a new expression converter
    pub fn new(ctx: &'a ConversionContext) -> Self {
        Self {
            ctx,
            direct_zi_assignment: false,
            limiters: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Convert the complete right-hand side of an analog contribution.
    /// Every Zi node in this expression tree is subject to the VAMS-2023
    /// section 4.5.12 strictly-positive transition-time rule.
    pub(crate) fn convert_contribution(
        &self,
        arena: &mut ExprArena,
        expr: &Expression,
    ) -> CompileResult<NodeId> {
        Self {
            ctx: self.ctx,
            direct_zi_assignment: true,
            limiters: std::cell::RefCell::new(Vec::new()),
        }
        .convert(arena, expr)
    }

    /// Array layout (base, lower, len) by name
    pub fn array_layout(&self, name: &str) -> Option<(usize, i64, usize)> {
        self.ctx.array(name)
    }

    fn cross_direction_argument(
        &self,
        arena: &mut ExprArena,
        arg: &Expression,
        name: &str,
    ) -> CompileResult<NodeId> {
        let converted = self.convert(arena, arg)?;
        let folded = autodiff_fold(arena, converted);
        match constant(arena, folded) {
            Some(v) if matches!(v, -1.0 | 0.0 | 1.0) => Ok(folded),
            Some(v) => Err(
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "{name} direction must be -1, 0, or 1, got {v}"
                )))
                .into(),
            ),
            None => Ok(converted),
        }
    }

    fn convert_optional_argument(
        &self,
        arena: &mut ExprArena,
        args: &[Expression],
        index: usize,
    ) -> CompileResult<Option<NodeId>> {
        match args.get(index) {
            None | Some(Expression::NullArgument(_)) => Ok(None),
            Some(expression) => self.convert(arena, expression).map(Some),
        }
    }

    /// Resolve the independent solver quantity named by a `ddx` probe. A flow
    /// is legal only when topology gives it a branch-current unknown; inferred
    /// contribution and terminal currents are dependent values, not axes.
    fn convert_ddx_probe(&self, probe: &BranchAccess) -> CompileResult<DdxAxis> {
        let kind = Self::resolved_access_kind(probe)?;
        let unknown_node = |name: &str| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "Unknown node: {name}"
            )))
        };
        let unknown_branch_or_node = |name: &str| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "Unknown branch or node: {name}"
            )))
        };
        match probe {
            BranchAccess::Nodes { pos, neg, .. } => {
                let (pos_node, neg_node) = if neg.is_none()
                    && let Some((pos_node, neg_node)) = self.ctx.branch_nodes(pos)
                {
                    (pos_node, neg_node)
                } else {
                    let pos_node = self.ctx.node_index(pos).ok_or_else(|| unknown_node(pos))?;
                    let neg_node = neg
                        .as_ref()
                        .map(|node| self.ctx.node_index(node).ok_or_else(|| unknown_node(node)))
                        .transpose()?
                        .unwrap_or_else(|| self.ctx.ground());
                    (pos_node, neg_node)
                };
                if kind == AccessKind::Flow {
                    let Some((ordinal, reversed)) = self.ctx.branch_current_axis(
                        (neg.is_none() && self.ctx.branch_nodes(pos).is_some()).then_some(pos),
                        pos_node,
                        neg_node,
                    ) else {
                        return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                            "ddx flow probe requires a solver-owned branch-current unknown from a potential or indirect contribution".into(),
                        ))
                        .into());
                    };
                    Ok(DdxAxis::BranchCurrent { ordinal, reversed })
                } else {
                    Ok(DdxAxis::Potential {
                        pos: (pos_node != self.ctx.ground()).then_some(pos_node),
                        neg: (neg_node != self.ctx.ground()).then_some(neg_node),
                    })
                }
            }
            BranchAccess::Branch { name, .. } => {
                let (pos_node, neg_node) = if let Some(nodes) = self.ctx.branch_nodes(name) {
                    nodes
                } else {
                    (
                        self.ctx
                            .node_index(name)
                            .ok_or_else(|| unknown_branch_or_node(name))?,
                        self.ctx.ground(),
                    )
                };
                if kind == AccessKind::Flow {
                    let Some((ordinal, reversed)) = self.ctx.branch_current_axis(
                        self.ctx.branch_nodes(name).is_some().then_some(name),
                        pos_node,
                        neg_node,
                    ) else {
                        return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                            format!(
                                "ddx flow probe I(<{name}>) requires a solver-owned branch-current unknown from a potential or indirect contribution"
                            ),
                        ))
                        .into());
                    };
                    Ok(DdxAxis::BranchCurrent { ordinal, reversed })
                } else {
                    Ok(DdxAxis::Potential {
                        pos: (pos_node != self.ctx.ground()).then_some(pos_node),
                        neg: (neg_node != self.ctx.ground()).then_some(neg_node),
                    })
                }
            }
        }
    }

    /// Convert an AST expression to an IR expression
    pub fn convert(&self, arena: &mut ExprArena, expr: &Expression) -> CompileResult<NodeId> {
        match expr {
            Expression::Number(num) => self.convert_number(arena, num),
            Expression::StringLit(_) => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature("String literals in expressions".into()),
            )
            .into()),
            // Semantic analysis refuses a discrete-domain expression in a
            // continuous-domain position, so this arm is unreachable through
            // the compiler's own pipeline and exists so that no other route
            // can turn one into a number.
            Expression::Digital(digital) => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature(format!(
                    "a {} has no continuous-domain value",
                    digital.construct()
                )),
            )
            .into()),
            Expression::Identifier(ident) => self.convert_identifier(arena, ident),
            Expression::SystemFunction(func) => self.convert_system_function(arena, func),
            Expression::Binary(binary) => self.convert_binary(arena, binary),
            Expression::Unary(unary) => self.convert_unary(arena, unary),
            Expression::Conditional(cond) => self.convert_conditional(arena, cond),
            Expression::Call(call) => self.convert_call(arena, call),
            Expression::NullArgument(_) => {
                Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "null positional argument is not legal in this expression position".into(),
                ))
                .into())
            }
            Expression::BranchAccess(access) => self.convert_branch_access(arena, access),
            Expression::ArrayAccess(access) => {
                let Some((base, lower, len)) = self.ctx.array(&access.array) else {
                    return Err(
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "'{}' is not a declared array variable",
                            access.array
                        )))
                        .into(),
                    );
                };
                let index = self.convert(arena, &access.index)?;
                let array = arena.intern(&access.array);
                let payload = arena.push_indexed(IndexedRead {
                    array,
                    base,
                    len,
                    lower,
                });
                Ok(arena.push(Node::VarIndexed { payload, index }))
            }
            Expression::ArrayLiteral(_) => {
                Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                    "Array literals are only supported as analog filter coefficient lists".into(),
                ))
                .into())
            }
            Expression::AnalogOperator(op) => self.convert_analog_operator(arena, op),
            Expression::NoiseSource(noise) => self.convert_noise_source(arena, noise),
        }
    }

    /// Convert a number literal
    fn convert_number(&self, arena: &mut ExprArena, num: &NumberLit) -> CompileResult<NodeId> {
        Ok(arena.push(Node::Const(num.value)))
    }

    /// Convert an identifier reference
    fn convert_identifier(
        &self,
        arena: &mut ExprArena,
        ident: &Identifier,
    ) -> CompileResult<NodeId> {
        let name = &ident.name;

        // Check if it's a parameter
        if self.ctx.param_map.contains_key(name) {
            let name = arena.intern(name);
            return Ok(arena.push(Node::Param(name)));
        }

        // Check if it's a variable
        if self.ctx.var_map.contains_key(name) {
            let name = arena.intern(name);
            return Ok(arena.push(Node::Var(name)));
        }

        // Check for built-in constants
        match name.as_str() {
            "M_PI" | "P_PI" => Ok(arena.push(Node::Const(std::f64::consts::PI))),
            "M_E" | "P_E" => Ok(arena.push(Node::Const(std::f64::consts::E))),
            "M_LN2" => Ok(arena.push(Node::Const(std::f64::consts::LN_2))),
            "M_LN10" => Ok(arena.push(Node::Const(std::f64::consts::LN_10))),
            "M_LOG2E" => Ok(arena.push(Node::Const(std::f64::consts::LOG2_E))),
            "M_LOG10E" => Ok(arena.push(Node::Const(std::f64::consts::LOG10_E))),
            "M_SQRT2" => Ok(arena.push(Node::Const(std::f64::consts::SQRT_2))),
            "inf" => Ok(arena.push(Node::Const(f64::INFINITY))),
            _ => Err(
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "Unknown identifier: {}",
                    name
                )))
                .into(),
            ),
        }
    }

    /// Convert a system function call
    fn convert_system_function(
        &self,
        arena: &mut ExprArena,
        func: &SystemFunction,
    ) -> CompileResult<NodeId> {
        match func.name.as_str() {
            "$vt" | "$thermal_vt" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(1))?;
                if func.args.is_empty() {
                    // $vt() = kT/q at nominal temperature
                    Ok(arena.push(Node::Vt))
                } else {
                    // $vt(temp) = k*temp/q
                    let temp_expr = self.convert(arena, &func.args[0])?;
                    // vt = temp * (k/q) where k/q ~ 8.617e-5
                    let scale =
                        arena.push(Node::Const(rspice_veriloga_runtime::THERMAL_VOLTAGE_PER_K));
                    Ok(arena.push(Node::Binary(BinaryOp::Mul, temp_expr, scale)))
                }
            }
            "$temperature" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(arena.push(Node::Temperature))
            }
            "$abstime" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(arena.push(Node::Time))
            }
            "$realtime" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(arena.push(Node::Time))
            }
            "$mfactor" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(arena.push(Node::Mfactor))
            }
            "$simparam" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                let name = match func.args.first() {
                    Some(Expression::StringLit(s)) => s.value.as_str(),
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "$simparam requires a string parameter name".into(),
                        ))
                        .into());
                    }
                };
                let Some(parameter) = rspice_veriloga_runtime::SimulationParameter::from_name(name)
                else {
                    if let Some(fallback) = func.args.get(1) {
                        return self.convert(arena, fallback);
                    }
                    return Err(
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "unsupported simulation parameter '{name}' has no fallback"
                        )))
                        .into(),
                    );
                };
                let value = arena.push(Node::SimParamValue(parameter));
                if let Some(fallback) = func.args.get(1) {
                    let present = arena.push(Node::SimParamPresent(parameter));
                    let fallback = self.convert(arena, fallback)?;
                    Ok(arena.push(Node::Conditional(present, value, fallback)))
                } else {
                    Ok(value)
                }
            }
            "$param_given" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(1))?;
                // $param_given(name) - whether the instance explicitly set
                // the parameter
                match func.args.first() {
                    Some(Expression::Identifier(id))
                        if self.ctx.param_index(&id.name).is_some() =>
                    {
                        let name = arena.intern(&id.name);
                        Ok(arena.push(Node::ParamGiven(name)))
                    }
                    _ => Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "$param_given requires a parameter name argument".into(),
                    ))
                    .into()),
                }
            }
            "$port_connected" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(1))?;
                let port_name = match func.args.first() {
                    Some(Expression::Identifier(id)) => id.name.as_str(),
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "$port_connected requires a terminal name".into(),
                        ))
                        .into());
                    }
                };
                let Some(index) = self.ctx.node_index(port_name) else {
                    return Err(
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "$port_connected unknown terminal: {port_name}"
                        )))
                        .into(),
                    );
                };
                if index >= self.ctx.num_terminals() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "$port_connected requires an external terminal, got internal node: {port_name}"
                    )))
                    .into());
                }
                Ok(arena.push(Node::PortConnected(pack_index(index))))
            }
            "$limit" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                // $limit(expr) or $limit(expr, step)
                // Bounds expression change per Newton iteration for convergence
                let inner = self.convert(arena, &func.args[0])?;
                let step = if func.args.len() > 1 {
                    Some(self.convert(arena, &func.args[1])?)
                } else {
                    None
                };
                Ok(arena.push(Node::Limit(inner, step)))
            }
            "$table_model" => {
                validate_arg_range(&func.name, func.args.len(), 2, None)?;
                // $table_model(input, table_spec, ...)
                // Supports:
                // 1) Inline numeric pairs: $table_model(x, 0,0, 1,2, 2,4)
                // 2) Inline string table: $table_model(x, "0 0; 1 2; 2 4")
                // 3) Table file path:    $table_model(x, "table.dat")
                let input = self.convert(arena, &func.args[0])?;
                let (x_data, y_data) = self.parse_table_model_data(func)?;
                let table = arena.push_table(x_data, y_data);
                Ok(arena.push(Node::TableLookup { input, table }))
            }
            "absdelay" => {
                validate_arg_range(&func.name, func.args.len(), 2, Some(3))?;
                let expr = self.convert(arena, &func.args[0])?;
                let delay_time = self.convert(arena, &func.args[1])?;
                let max_delay = match func.args.get(2) {
                    Some(value) => Some(self.convert(arena, value)?),
                    None => None,
                };
                Ok(arena.push_heavy(Heavy::AbsDelay {
                    site: crate::ir::AbsDelaySiteId::from_span(func.span),
                    expr,
                    delay_time,
                    max_delay,
                }))
            }
            "transition" => {
                // transition(expr, delay, rise_time, fall_time)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "transition requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(arena, &func.args[0])?;
                let delay = if func.args.len() > 1 {
                    Some(self.convert(arena, &func.args[1])?)
                } else {
                    None
                };
                let rise_time = if func.args.len() > 2 {
                    Some(self.convert(arena, &func.args[2])?)
                } else {
                    None
                };
                let fall_time = if func.args.len() > 3 {
                    Some(self.convert(arena, &func.args[3])?)
                } else {
                    None
                };
                Ok(arena.push_heavy(Heavy::Transition {
                    site: crate::ir::TransitionSiteId::from_span(func.span),
                    expr,
                    delay,
                    rise_time,
                    fall_time,
                }))
            }
            "slew" => {
                // slew(expr, max_pos_slew, max_neg_slew)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(arena, &func.args[0])?;
                let max_pos_slew = if func.args.len() > 1 {
                    Some(self.convert(arena, &func.args[1])?)
                } else {
                    None
                };
                let max_neg_slew = if func.args.len() > 2 {
                    Some(self.convert(arena, &func.args[2])?)
                } else {
                    None
                };
                Ok(arena.push_heavy(Heavy::Slew {
                    site: crate::ir::SlewSiteId::from_span(func.span),
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                }))
            }
            "cross" => {
                // cross(expr [, direction [, time_tol [, expr_tol [, enable]]]])
                validate_arg_range(&func.name, func.args.len(), 1, Some(5))?;
                validate_event_argument_dependencies("cross", &func.args)?;
                let expr = self.convert(arena, &func.args[0])?;
                let direction = self.convert_optional_argument(arena, &func.args, 1)?;
                let time_tol = self.convert_optional_argument(arena, &func.args, 2)?;
                let expr_tol = self.convert_optional_argument(arena, &func.args, 3)?;
                let enable = self.convert_optional_argument(arena, &func.args, 4)?;
                Ok(arena.push_heavy(Heavy::Cross {
                    expr,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                }))
            }
            "$white_noise" | "white_noise" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                // $white_noise(power, name)
                let power = self.convert(arena, &func.args[0])?;
                let name = optional_string_arg(&func.name, func.args.get(1), "name")?;
                Ok(arena.push_heavy(Heavy::WhiteNoise {
                    site: crate::ir::NoiseSiteId::from_span(func.span),
                    power,
                    name,
                }))
            }
            "$flicker_noise" | "flicker_noise" => {
                validate_arg_range(&func.name, func.args.len(), 2, Some(3))?;
                // $flicker_noise(power, exponent, name)
                let power = self.convert(arena, &func.args[0])?;
                let exponent = self.convert(arena, &func.args[1])?;
                let name = optional_string_arg(&func.name, func.args.get(2), "name")?;
                Ok(arena.push_heavy(Heavy::FlickerNoise {
                    site: crate::ir::NoiseSiteId::from_span(func.span),
                    power,
                    exponent,
                    name,
                }))
            }
            "$noise_table" | "noise_table" | "$noise_table_log" | "noise_table_log" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                let log_interp = func.name.contains("log");
                let name = optional_string_arg(&func.name, func.args.get(1), "name")?;
                let points = self.noise_table_points(arena, &func.args[0], log_interp)?;
                Ok(arena.push_heavy(Heavy::NoiseTable {
                    site: crate::ir::NoiseSiteId::from_span(func.span),
                    points,
                    log_interp,
                    name,
                }))
            }
            "analysis" => analysis_expression(arena, &func.name, &func.args),
            "above" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(4))?;
                validate_event_argument_dependencies("above", &func.args)?;
                let expr = self.convert(arena, &func.args[0])?;
                let time_tol = self.convert_optional_argument(arena, &func.args, 1)?;
                let expr_tol = self.convert_optional_argument(arena, &func.args, 2)?;
                let enable = self.convert_optional_argument(arena, &func.args, 3)?;
                Ok(arena.push_heavy(Heavy::Above {
                    expr,
                    time_tol,
                    expr_tol,
                    enable,
                }))
            }
            "timer" => {
                // timer(start_time [, period [, time_tol [, enable]]])
                validate_arg_range(&func.name, func.args.len(), 1, Some(4))?;
                let start_time = self.convert(arena, &func.args[0])?;
                let optional = |arena: &mut ExprArena, index: usize| match func.args.get(index) {
                    Some(expr) => self.convert(arena, expr).map(Some),
                    None => Ok(None),
                };
                let period = optional(arena, 1)?;
                let time_tol = optional(arena, 2)?;
                let enable = optional(arena, 3)?;
                Ok(arena.push_heavy(Heavy::Timer {
                    start_time,
                    period,
                    time_tol,
                    enable,
                }))
            }
            // The IR models real zeros/poles and numerator/denominator
            // coefficients, but this converter never parsed the coefficient
            // arrays: it discarded them and emitted a unity passthrough, so a
            // filtered contribution silently evaluated as if unfiltered.
            // Refuse instead, matching the `Node::Ddx` precedent in
            // codegen/generator.rs, until the arrays are actually lowered.
            "laplace_zp" | "laplace_nd" => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature(format!(
                    "{}(): analog filter operators are not lowered; the transfer \
                     function would be silently ignored",
                    func.name
                )),
            )
            .into()),
            _ => Err(
                CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(format!(
                    "System function: {}",
                    func.name
                )))
                .into(),
            ),
        }
    }

    /// Convert a binary expression
    fn convert_binary(
        &self,
        arena: &mut ExprArena,
        binary: &crate::ast::BinaryExpr,
    ) -> CompileResult<NodeId> {
        let left = self.convert(arena, &binary.left)?;
        let right = self.convert(arena, &binary.right)?;

        Ok(arena.push(Node::Binary(binary.op, left, right)))
    }

    /// Convert a unary expression
    fn convert_unary(
        &self,
        arena: &mut ExprArena,
        unary: &crate::ast::UnaryExpr,
    ) -> CompileResult<NodeId> {
        let operand = self.convert(arena, &unary.operand)?;
        if unary.op == crate::ast::UnaryOp::ToInteger {
            // The IR already has a checked integer identity, with a zero
            // tangent and an IntegerCast specialization in the JITs.
            let zero = arena.push(Node::Const(0.0));
            return Ok(arena.push(Node::Binary(BinaryOp::BitOr, operand, zero)));
        }
        Ok(arena.push(Node::Unary(unary.op, operand)))
    }

    /// Convert a conditional expression
    fn convert_conditional(
        &self,
        arena: &mut ExprArena,
        cond: &crate::ast::ConditionalExpr,
    ) -> CompileResult<NodeId> {
        let condition = self.convert(arena, &cond.condition)?;
        let then_expr = self.convert(arena, &cond.then_expr)?;
        let else_expr = self.convert(arena, &cond.else_expr)?;

        Ok(arena.push(Node::Conditional(condition, then_expr, else_expr)))
    }

    /// Convert a function call
    fn convert_call(&self, arena: &mut ExprArena, call: &CallExpr) -> CompileResult<NodeId> {
        let ir_func = match call.name.as_str() {
            "abs" => IrFunction::Abs,
            "sqrt" => IrFunction::Sqrt,
            "exp" => IrFunction::Exp,
            "__rspice_limited_exp" => IrFunction::LimitedExp,
            "ln" => IrFunction::Log,
            "log" => IrFunction::Log,
            "log10" => IrFunction::Log10,
            "sin" => IrFunction::Sin,
            "cos" => IrFunction::Cos,
            "tan" => IrFunction::Tan,
            "sinh" => IrFunction::Sinh,
            "cosh" => IrFunction::Cosh,
            "tanh" => IrFunction::Tanh,
            "asin" => IrFunction::Asin,
            "acos" => IrFunction::Acos,
            "atan" => IrFunction::Atan,
            "asinh" => IrFunction::Asinh,
            "acosh" => IrFunction::Acosh,
            "atanh" => IrFunction::Atanh,
            "atan2" => IrFunction::Atan2,
            "hypot" => IrFunction::Hypot,
            "floor" => IrFunction::Floor,
            "ceil" => IrFunction::Ceil,
            "min" => IrFunction::Min,
            "max" => IrFunction::Max,
            "pow" => IrFunction::Pow,
            "limexp" => {
                // limexp is a special analog operator
                if call.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "limexp requires an argument".into(),
                    ))
                    .into());
                }
                let arg = self.convert(arena, &call.args[0])?;
                return Ok(arena.push(Node::Limexp(arg)));
            }
            // Analog operators, noise sources, filters, and event functions
            // arrive as plain calls; route them to their IR forms.
            _ => return self.convert_analog_call(arena, call),
        };

        let mut args = Vec::with_capacity(call.args.len());
        for argument in &call.args {
            args.push(self.convert(arena, argument)?);
        }

        Ok(arena.push_call(ir_func, &args))
    }

    /// Convert analog operators, filters, noise sources, and event functions
    fn convert_analog_call(&self, arena: &mut ExprArena, call: &CallExpr) -> CompileResult<NodeId> {
        let require_arg = |n: usize| -> CompileResult<&Expression> {
            call.args.get(n).ok_or_else(|| {
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "{} requires at least {} argument(s)",
                    call.name,
                    n + 1
                )))
                .into()
            })
        };

        match call.name.as_str() {
            "ddt" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(1))?;
                let inner = self.convert(arena, require_arg(0)?)?;
                Ok(arena.push(Node::Ddt(inner)))
            }
            "idt" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let inner = self.convert(arena, require_arg(0)?)?;
                let ic = match call.args.get(1) {
                    Some(expr) => Some(self.convert(arena, expr)?),
                    None => None,
                };
                Ok(arena.push(Node::Idt(inner, ic)))
            }
            "idtmod" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                // idtmod(expr [, ic [, modulus [, offset]]]) - without a
                // modulus it degenerates to idt
                let inner = self.convert(arena, require_arg(0)?)?;
                let ic = match call.args.get(1) {
                    Some(expr) => Some(self.convert(arena, expr)?),
                    None => None,
                };
                match call.args.get(2) {
                    Some(modulus) => {
                        let modulus = self.convert(arena, modulus)?;
                        let offset = match call.args.get(3) {
                            Some(expr) => Some(self.convert(arena, expr)?),
                            None => None,
                        };
                        let payload = arena.push_optional_pair((ic, offset));
                        Ok(arena.push(Node::IdtMod {
                            expr: inner,
                            modulus,
                            payload,
                        }))
                    }
                    None => Ok(arena.push(Node::Idt(inner, ic))),
                }
            }
            "ddx" => {
                validate_arg_range(&call.name, call.args.len(), 2, Some(2))?;
                let inner = self.convert(arena, require_arg(0)?)?;
                let probe = require_arg(1)?;
                let axis = match probe {
                    Expression::BranchAccess(probe) => self.convert_ddx_probe(probe)?,
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "ddx probe must be a branch access like V(node), V(a,b), V(<branch>), or I(<solver-owned-branch>)"
                                .into(),
                        ))
                        .into());
                    }
                };
                let axis = arena.push_ddx_axis(axis);
                Ok(arena.push(Node::Ddx { expr: inner, axis }))
            }
            "absdelay" => {
                validate_arg_range(&call.name, call.args.len(), 2, Some(3))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let delay_time = self.convert(arena, require_arg(1)?)?;
                let max_delay = match call.args.get(2) {
                    Some(value) => Some(self.convert(arena, value)?),
                    None => None,
                };
                Ok(arena.push_heavy(Heavy::AbsDelay {
                    site: crate::ir::AbsDelaySiteId::from_span(call.span),
                    expr,
                    delay_time,
                    max_delay,
                }))
            }
            "transition" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(5))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let opt = |arena: &mut ExprArena, n: usize| match call.args.get(n) {
                    Some(e) => self.convert(arena, e).map(Some),
                    None => Ok(None),
                };
                let delay = opt(arena, 1)?;
                let rise_time = opt(arena, 2)?;
                let fall_time = opt(arena, 3)?;
                Ok(arena.push_heavy(Heavy::Transition {
                    site: crate::ir::TransitionSiteId::from_span(call.span),
                    expr,
                    delay,
                    rise_time,
                    fall_time,
                }))
            }
            "slew" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(3))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let opt = |arena: &mut ExprArena, n: usize| match call.args.get(n) {
                    Some(e) => self.convert(arena, e).map(Some),
                    None => Ok(None),
                };
                let max_pos_slew = opt(arena, 1)?;
                let max_neg_slew = opt(arena, 2)?;
                Ok(arena.push_heavy(Heavy::Slew {
                    site: crate::ir::SlewSiteId::from_span(call.span),
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                }))
            }
            "cross" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(5))?;
                validate_event_argument_dependencies("cross", &call.args)?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let direction = self.convert_optional_argument(arena, &call.args, 1)?;
                let time_tol = self.convert_optional_argument(arena, &call.args, 2)?;
                let expr_tol = self.convert_optional_argument(arena, &call.args, 3)?;
                let enable = self.convert_optional_argument(arena, &call.args, 4)?;
                Ok(arena.push_heavy(Heavy::Cross {
                    expr,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                }))
            }
            "above" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                validate_event_argument_dependencies("above", &call.args)?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let time_tol = self.convert_optional_argument(arena, &call.args, 1)?;
                let expr_tol = self.convert_optional_argument(arena, &call.args, 2)?;
                let enable = self.convert_optional_argument(arena, &call.args, 3)?;
                Ok(arena.push_heavy(Heavy::Above {
                    expr,
                    time_tol,
                    expr_tol,
                    enable,
                }))
            }
            "timer" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                let start_time = self.convert(arena, require_arg(0)?)?;
                let optional = |arena: &mut ExprArena, index: usize| match call.args.get(index) {
                    Some(expr) => self.convert(arena, expr).map(Some),
                    None => Ok(None),
                };
                let period = optional(arena, 1)?;
                let time_tol = optional(arena, 2)?;
                let enable = optional(arena, 3)?;
                Ok(arena.push_heavy(Heavy::Timer {
                    start_time,
                    period,
                    time_tol,
                    enable,
                }))
            }
            "last_crossing" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let direction = match call.args.get(1) {
                    Some(arg) => {
                        Some(self.cross_direction_argument(arena, arg, "last_crossing")?)
                    }
                    None => None,
                };
                Ok(arena.push(Node::LastCrossing { expr, direction }))
            }
            "analysis" => analysis_expression(arena, &call.name, &call.args),
            "white_noise" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let power = self.convert(arena, require_arg(0)?)?;
                let name = optional_string_arg(&call.name, call.args.get(1), "name")?;
                Ok(arena.push_heavy(Heavy::WhiteNoise {
                    site: crate::ir::NoiseSiteId::from_span(call.span),
                    power,
                    name,
                }))
            }
            "flicker_noise" => {
                validate_arg_range(&call.name, call.args.len(), 2, Some(3))?;
                let power = self.convert(arena, require_arg(0)?)?;
                let exponent = self.convert(arena, require_arg(1)?)?;
                let name = optional_string_arg(&call.name, call.args.get(2), "name")?;
                Ok(arena.push_heavy(Heavy::FlickerNoise {
                    site: crate::ir::NoiseSiteId::from_span(call.span),
                    power,
                    exponent,
                    name,
                }))
            }
            "noise_table" | "noise_table_log" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let log_interp = call.name.ends_with("log");
                let name = optional_string_arg(&call.name, call.args.get(1), "name")?;
                let points = self.noise_table_points(arena, require_arg(0)?, log_interp)?;
                Ok(arena.push_heavy(Heavy::NoiseTable {
                    site: crate::ir::NoiseSiteId::from_span(call.span),
                    points,
                    log_interp,
                    name,
                }))
            }
            "laplace_nd" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let numerator = self.const_filter_real_array(
                    arena,
                    require_arg(1)?,
                    "laplace_nd",
                    "numerator",
                    false,
                )?;
                let denominator = self.const_filter_real_array(
                    arena,
                    require_arg(2)?,
                    "laplace_nd",
                    "denominator",
                    false,
                )?;
                validate_laplace_coefficients("laplace_nd", &numerator, &denominator)?;
                Ok(arena.push_heavy(Heavy::LaplaceND {
                    site: crate::ir::LaplaceSiteId::from_span(call.span),
                    expr,
                    numerator,
                    denominator,
                }))
            }
            "laplace_zp" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let zeros =
                    self.const_complex_pairs(arena, require_arg(1)?, "laplace_zp", "zeros", true)?;
                let poles =
                    self.const_complex_pairs(arena, require_arg(2)?, "laplace_zp", "poles", false)?;
                let numerator = crate::laplace::laplace_roots_to_polynomial(&zeros)
                    .map_err(|error| laplace_error("laplace_zp zeros", error))?;
                let denominator = crate::laplace::laplace_roots_to_polynomial(&poles)
                    .map_err(|error| laplace_error("laplace_zp poles", error))?;
                validate_laplace_coefficients("laplace_zp", &numerator, &denominator)?;
                Ok(arena.push_heavy(Heavy::LaplaceND {
                    site: crate::ir::LaplaceSiteId::from_span(call.span),
                    expr,
                    numerator,
                    denominator,
                }))
            }
            "laplace_zd" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                // zeros (pairs) + denominator coefficients: expand the
                // zeros into a numerator polynomial
                let expr = self.convert(arena, require_arg(0)?)?;
                let zeros =
                    self.const_complex_pairs(arena, require_arg(1)?, "laplace_zd", "zeros", true)?;
                let denominator = self.const_filter_real_array(
                    arena,
                    require_arg(2)?,
                    "laplace_zd",
                    "denominator",
                    false,
                )?;
                let numerator =
                    crate::laplace::laplace_roots_to_polynomial(&zeros).map_err(|e| {
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "laplace_zd zeros: {}",
                            e
                        )))
                    })?;
                validate_laplace_coefficients("laplace_zd", &numerator, &denominator)?;
                Ok(arena.push_heavy(Heavy::LaplaceND {
                    site: crate::ir::LaplaceSiteId::from_span(call.span),
                    expr,
                    numerator,
                    denominator,
                }))
            }
            "laplace_np" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                // numerator coefficients + poles (pairs)
                let expr = self.convert(arena, require_arg(0)?)?;
                let numerator = self.const_filter_real_array(
                    arena,
                    require_arg(1)?,
                    "laplace_np",
                    "numerator",
                    false,
                )?;
                let poles =
                    self.const_complex_pairs(arena, require_arg(2)?, "laplace_np", "poles", false)?;
                let denominator =
                    crate::laplace::laplace_roots_to_polynomial(&poles).map_err(|e| {
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "laplace_np poles: {}",
                            e
                        )))
                    })?;
                validate_laplace_coefficients("laplace_np", &numerator, &denominator)?;
                Ok(arena.push_heavy(Heavy::LaplaceND {
                    site: crate::ir::LaplaceSiteId::from_span(call.span),
                    expr,
                    numerator,
                    denominator,
                }))
            }
            // Z-domain filters. Constant arguments are retained as programs
            // and frozen per instance at the beginning of each analysis.
            "zi_nd" | "zi_zp" | "zi_zd" | "zi_np" => {
                validate_arg_range(&call.name, call.args.len(), 4, Some(6))?;
                self.validate_raw_zi_operand_budget(&call.name, require_arg(1)?, require_arg(2)?)?;
                let expr = self.convert(arena, require_arg(0)?)?;
                let (numerator, denominator) = match call.name.as_str() {
                    "zi_nd" => (
                        ZiPolynomial::Coefficients(self.zi_real_array(
                            arena,
                            require_arg(1)?,
                            "zi_nd",
                            "numerator",
                            false,
                        )?),
                        ZiPolynomial::Coefficients(self.zi_real_array(
                            arena,
                            require_arg(2)?,
                            "zi_nd",
                            "denominator",
                            false,
                        )?),
                    ),
                    "zi_zp" => (
                        ZiPolynomial::Roots(self.zi_complex_pairs(
                            arena,
                            require_arg(1)?,
                            "zi_zp",
                            "zeros",
                            true,
                        )?),
                        ZiPolynomial::Roots(self.zi_complex_pairs(
                            arena,
                            require_arg(2)?,
                            "zi_zp",
                            "poles",
                            false,
                        )?),
                    ),
                    "zi_zd" => (
                        ZiPolynomial::Roots(self.zi_complex_pairs(
                            arena,
                            require_arg(1)?,
                            "zi_zd",
                            "zeros",
                            true,
                        )?),
                        ZiPolynomial::Coefficients(self.zi_real_array(
                            arena,
                            require_arg(2)?,
                            "zi_zd",
                            "denominator",
                            false,
                        )?),
                    ),
                    _ => (
                        ZiPolynomial::Coefficients(self.zi_real_array(
                            arena,
                            require_arg(1)?,
                            "zi_np",
                            "numerator",
                            false,
                        )?),
                        ZiPolynomial::Roots(self.zi_complex_pairs(
                            arena,
                            require_arg(2)?,
                            "zi_np",
                            "poles",
                            false,
                        )?),
                    ),
                };
                validate_zi_polynomial_budget(&call.name, &numerator, &denominator)?;
                let period =
                    self.zi_definition_arg(arena, require_arg(3)?, &call.name, "sample period")?;
                let transition = match call.args.get(4) {
                    Some(Expression::NullArgument(_)) => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            format!("{} transition-time argument may not be null", call.name),
                        ))
                        .into());
                    }
                    Some(value) => self.convert(arena, value)?,
                    None => arena.push(Node::Const(self.ctx.default_transition())),
                };
                let first_transition = match call.args.get(5) {
                    Some(value) => {
                        self.zi_definition_arg(arena, value, &call.name, "first transition time")?
                    }
                    None => arena.push(Node::Const(0.0)),
                };
                validate_wholly_constant_zi_definition(
                    arena,
                    &call.name,
                    &numerator,
                    &denominator,
                    period,
                    first_transition,
                )?;
                Ok(arena.push_heavy(Heavy::ZiFilter {
                    site: crate::ir::ZiSiteId::from_span(call.span),
                    expr,
                    numerator,
                    denominator,
                    period,
                    transition,
                    first_transition,
                    direct_assignment: self.direct_zi_assignment,
                }))
            }
            _ => Err(
                CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(format!(
                    "Function: {}",
                    call.name
                )))
                .into(),
            ),
        }
    }

    /// Parse a noise_table pair list `{f1, p1, f2, p2, ...}` into sorted,
    /// frequency-unique (frequency, power) points. String (file) input and
    /// non-constant entries are clean unsupported errors.
    fn noise_table_points(
        &self,
        arena: &mut ExprArena,
        arg: &Expression,
        log_interp: bool,
    ) -> CompileResult<Vec<(f64, f64)>> {
        if matches!(arg, Expression::StringLit(_)) {
            return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                "noise_table file input (inline the {f, p, ...} pairs instead)".into(),
            ))
            .into());
        }
        let flat = self.const_real_array(arena, arg)?;
        if flat.is_empty() || flat.len() % 2 != 0 {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table needs a non-empty, even-length {f, p, ...} list".into(),
            ))
            .into());
        }
        let mut points: Vec<(f64, f64)> = flat.chunks_exact(2).map(|c| (c[0], c[1])).collect();
        if points
            .iter()
            .any(|&(frequency, power)| !frequency.is_finite() || !power.is_finite())
        {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table frequencies and powers must be finite".into(),
            ))
            .into());
        }
        if log_interp
            && points
                .iter()
                .any(|&(frequency, power)| frequency <= 0.0 || power <= 0.0)
        {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table_log requires strictly positive frequencies and powers".into(),
            ))
            .into());
        }
        if !log_interp
            && points
                .iter()
                .any(|&(frequency, power)| frequency < 0.0 || power < 0.0)
        {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table requires non-negative frequencies and powers".into(),
            ))
            .into());
        }
        points.sort_by(|left, right| left.0.total_cmp(&right.0));
        if let Some(duplicate) = points
            .windows(2)
            .find(|adjacent| adjacent[0].0 == adjacent[1].0)
        {
            let operator = if log_interp {
                "noise_table_log"
            } else {
                "noise_table"
            };
            return Err(
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} frequency points must be unique; duplicate frequency {}",
                    duplicate[0].0
                )))
                .into(),
            );
        }
        Ok(points)
    }

    /// Evaluate an array-literal argument to constant reals
    fn const_real_array(
        &self,
        arena: &mut ExprArena,
        expr: &Expression,
    ) -> CompileResult<Vec<f64>> {
        let elements: Vec<&Expression> = match expr {
            Expression::ArrayLiteral(arr) => arr
                .elements
                .iter()
                .map(|element| match element {
                    ArrayLiteralElement::Value(expression) => Ok(expression),
                    ArrayLiteralElement::Replication(replication) => Err(CodeGenError::with_span(
                        CodeGenErrorKind::UnsupportedFeature(
                            "replication in analog filter coefficient lists is not yet supported; write coefficients explicitly within the operand limit"
                                .into(),
                        ),
                        replication.span,
                    )
                    .into()),
                })
                .collect::<CompileResult<Vec<_>>>()?,
            other => vec![other],
        };

        elements
            .into_iter()
            .map(|e| {
                let argument = self.convert(arena, e)?;
                let converted = autodiff_fold(arena, argument);
                match constant(arena, converted) {
                    Some(v) => Ok(v),
                    None => Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                        "filter coefficients must be compile-time constants \
                         (parameter-dependent coefficients are not supported yet)"
                            .into(),
                    ))
                    .into()),
                }
            })
            .collect()
    }

    /// Return the flat expressions in a Verilog-AMS filter vector without
    /// treating a concatenation or scalar expression as an unpacked array.
    fn filter_vector_elements<'expr>(
        &self,
        arena: &mut ExprArena,
        expression: &'expr Expression,
        operator: &str,
        role: &str,
        allow_null: bool,
    ) -> CompileResult<Vec<&'expr Expression>> {
        match expression {
            Expression::NullArgument(_) if allow_null => Ok(Vec::new()),
            Expression::NullArgument(span) => Err(CodeGenError::with_span(
                CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} operand may not be null"
                )),
                *span,
            )
            .into()),
            Expression::ArrayLiteral(array) if !array.assignment_pattern => {
                Err(CodeGenError::with_span(
                    CodeGenErrorKind::InvalidExpression(format!(
                        "{operator} {role} vector must be an assignment pattern opened with `'{{` or an array identifier; ordinary concatenation `{{...}}` is not a Verilog-AMS array value"
                    )),
                    array.span,
                )
                .into())
            }
            Expression::ArrayLiteral(array) => {
                let mut materialized = Vec::new();
                self.append_filter_vector_elements(
                    arena,
                    &array.elements,
                    operator,
                    role,
                    &mut materialized,
                    0,
                )?;
                Ok(materialized)
            }
            Expression::Identifier(identifier) if self.ctx.array(&identifier.name).is_some() => {
                Err(CodeGenError::with_span(
                    CodeGenErrorKind::UnsupportedFeature(format!(
                        "{operator} {role} array identifier '{}' is valid Verilog-AMS syntax, but executable filter array operands are not implemented yet",
                        identifier.name
                    )),
                    identifier.span,
                )
                .into())
            }
            other => Err(CodeGenError::with_span(
                CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} operand must be an assignment pattern opened with `'{{` or an array identifier; a scalar expression is not a filter vector"
                )),
                other.span(),
            )
            .into()),
        }
    }

    fn append_filter_vector_elements<'expr>(
        &self,
        arena: &mut ExprArena,
        elements: &'expr [ArrayLiteralElement],
        operator: &str,
        role: &str,
        output: &mut Vec<&'expr Expression>,
        depth: usize,
    ) -> CompileResult<()> {
        const MAX_DEPTH: usize = 128;
        const MAX_ELEMENTS: usize =
            crate::zfilter::MAX_ZI_RUNTIME_OPERANDS - crate::zfilter::ZI_FIXED_RUNTIME_OPERANDS;

        if depth >= MAX_DEPTH {
            let Some(first) = elements.first() else {
                return Ok(());
            };
            return Err(CodeGenError::with_span(
                CodeGenErrorKind::UnsupportedFeature(format!(
                    "{operator} {role} replication nesting exceeds the safety limit of {MAX_DEPTH}"
                )),
                first.span(),
            )
            .into());
        }

        for element in elements {
            match element {
                ArrayLiteralElement::Value(expression) => {
                    if output.len() == MAX_ELEMENTS {
                        return Err(CodeGenError::with_span(
                            CodeGenErrorKind::InvalidExpression(format!(
                                "{operator} {role} vector exceeds the materialization limit of {MAX_ELEMENTS} elements"
                            )),
                            expression.span(),
                        )
                        .into());
                    }
                    output.push(expression);
                }
                ArrayLiteralElement::Replication(replication) => {
                    if replication.elements.is_empty() {
                        return Err(CodeGenError::with_span(
                            CodeGenErrorKind::InvalidExpression(format!(
                                "{operator} {role} replication body must contain at least one element"
                            )),
                            replication.span,
                        )
                        .into());
                    }
                    let count =
                        self.filter_replication_count(arena, replication, operator, role)?;
                    let mut body = Vec::new();
                    self.append_filter_vector_elements(
                        arena,
                        &replication.elements,
                        operator,
                        role,
                        &mut body,
                        depth + 1,
                    )?;
                    let additional = count.checked_mul(body.len()).ok_or_else(|| {
                        CodeGenError::with_span(
                            CodeGenErrorKind::InvalidExpression(format!(
                                "{operator} {role} replication element count overflows usize"
                            )),
                            replication.span,
                        )
                    })?;
                    let projected = output.len().checked_add(additional).ok_or_else(|| {
                        CodeGenError::with_span(
                            CodeGenErrorKind::InvalidExpression(format!(
                                "{operator} {role} vector length overflows usize"
                            )),
                            replication.span,
                        )
                    })?;
                    if projected > MAX_ELEMENTS {
                        return Err(CodeGenError::with_span(
                            CodeGenErrorKind::InvalidExpression(format!(
                                "{operator} {role} replication materializes {projected} elements; the supported limit is {MAX_ELEMENTS}"
                            )),
                            replication.span,
                        )
                        .into());
                    }
                    output.try_reserve(additional).map_err(|_| {
                        CodeGenError::with_span(
                            CodeGenErrorKind::UnsupportedFeature(format!(
                                "{operator} {role} replication could not reserve storage for {additional} elements"
                            )),
                            replication.span,
                        )
                    })?;
                    for _ in 0..count {
                        output.extend(body.iter().copied());
                    }
                }
            }
        }
        Ok(())
    }

    fn filter_replication_count(
        &self,
        arena: &mut ExprArena,
        replication: &crate::ast::ReplicationExpr,
        operator: &str,
        role: &str,
    ) -> CompileResult<usize> {
        let converted = self.convert(arena, &replication.count)?;
        let folded = autodiff_fold(arena, converted);
        let value = match constant(arena, folded) {
            Some(value) => value,
            None => {
                return Err(CodeGenError::with_span(
                    CodeGenErrorKind::InvalidExpression(format!(
                        "{operator} {role} replication count must be an instance-invariant integer constant expression"
                    )),
                    replication.count.span(),
                )
                .into());
            }
        };
        if !value.is_finite() || value.fract() != 0.0 {
            return Err(CodeGenError::with_span(
                CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} replication count must be a finite integer; found {value}"
                )),
                replication.count.span(),
            )
            .into());
        }
        if value < 0.0 {
            return Err(CodeGenError::with_span(
                CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} replication count must be non-negative; found {value}"
                )),
                replication.count.span(),
            )
            .into());
        }
        if value > usize::MAX as f64 {
            return Err(CodeGenError::with_span(
                CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} replication count {value} is not representable on this platform"
                )),
                replication.count.span(),
            )
            .into());
        }
        Ok(value as usize)
    }

    fn const_filter_real_array(
        &self,
        arena: &mut ExprArena,
        expression: &Expression,
        operator: &str,
        role: &str,
        allow_null: bool,
    ) -> CompileResult<Vec<f64>> {
        let elements =
            self.filter_vector_elements(arena, expression, operator, role, allow_null)?;
        let mut values = Vec::with_capacity(elements.len());
        for element in elements {
            let converted = self.convert(arena, element)?;
            let folded = autodiff_fold(arena, converted);
            match constant(arena, folded) {
                Some(value) => values.push(value),
                None => {
                    return Err(CodeGenError::with_span(
                        CodeGenErrorKind::UnsupportedFeature(format!(
                            "{operator} {role} values must be compile-time constants (parameter-dependent values are not supported yet)"
                        )),
                        element.span(),
                    )
                    .into());
                }
            }
        }
        Ok(values)
    }

    fn zi_definition_arg(
        &self,
        arena: &mut ExprArena,
        expression: &Expression,
        operator: &str,
        role: &str,
    ) -> CompileResult<NodeId> {
        if matches!(expression, Expression::NullArgument(_)) {
            return Err(
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "{operator} {role} argument may not be null"
                )))
                .into(),
            );
        }
        // VAMS-2023 §4.5.14 freezes a dynamic expression passed to a
        // constant argument at analysis start. Preserve the complete safe
        // runtime program here; lifecycle initialization evaluates it once.
        let converted = self.convert(arena, expression)?;
        Ok(autodiff_fold(arena, converted))
    }

    fn validate_raw_zi_operand_budget(
        &self,
        operator: &str,
        numerator: &Expression,
        denominator: &Expression,
    ) -> CompileResult<()> {
        let scalar_count = |expression: &Expression| match expression {
            Expression::NullArgument(_) => 0,
            Expression::ArrayLiteral(array) => array.elements.len(),
            _ => 1,
        };
        crate::zfilter::validate_zi_runtime_operand_budget(
            operator,
            scalar_count(numerator),
            scalar_count(denominator),
        )
        .map(|_| ())
        .map_err(|error| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string())).into()
        })
    }

    fn zi_real_array(
        &self,
        arena: &mut ExprArena,
        expression: &Expression,
        operator: &str,
        role: &str,
        allow_null: bool,
    ) -> CompileResult<Vec<NodeId>> {
        let elements =
            self.filter_vector_elements(arena, expression, operator, role, allow_null)?;
        let mut values = Vec::with_capacity(elements.len());
        for element in elements {
            values.push(self.zi_definition_arg(arena, element, operator, role)?);
        }
        Ok(values)
    }

    fn zi_complex_pairs(
        &self,
        arena: &mut ExprArena,
        expression: &Expression,
        operator: &str,
        role: &str,
        allow_null: bool,
    ) -> CompileResult<Vec<(NodeId, NodeId)>> {
        let values = self.zi_real_array(arena, expression, operator, role, allow_null)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "zi pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values
            .chunks_exact(2)
            .map(|pair| (pair[0], pair[1]))
            .collect())
    }

    /// Evaluate an array-literal argument to constant (re, im) pairs
    fn const_complex_pairs(
        &self,
        arena: &mut ExprArena,
        expression: &Expression,
        operator: &str,
        role: &str,
        allow_null: bool,
    ) -> CompileResult<Vec<(f64, f64)>> {
        let values = self.const_filter_real_array(arena, expression, operator, role, allow_null)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values.chunks_exact(2).map(|p| (p[0], p[1])).collect())
    }

    /// Convert a branch access expression
    fn convert_branch_access(
        &self,
        arena: &mut ExprArena,
        access: &BranchAccess,
    ) -> CompileResult<NodeId> {
        let kind = Self::resolved_access_kind(access)?;
        match access {
            BranchAccess::Nodes { pos, neg, .. } => {
                // A single-name access may refer to a declared named branch
                if neg.is_none()
                    && let Some((pos_idx, neg_idx)) = self.ctx.branch_nodes(pos)
                {
                    return self.access_to_ir(arena, kind, Some(pos), pos_idx, neg_idx);
                }

                let pos_idx = self.ctx.node_index(pos).ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "Unknown node: {}",
                        pos
                    )))
                })?;

                let neg_idx = neg
                    .as_ref()
                    .map(|n| {
                        self.ctx.node_index(n).ok_or_else(|| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "Unknown node: {}",
                                n
                            )))
                        })
                    })
                    .transpose()?
                    .unwrap_or(self.ctx.ground());

                self.access_to_ir(arena, kind, None, pos_idx, neg_idx)
            }
            BranchAccess::Branch { name, .. } => {
                let (pos_idx, neg_idx) = if let Some(nodes) = self.ctx.branch_nodes(name) {
                    nodes
                } else {
                    let pos_idx = self.ctx.node_index(name).ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "Unknown branch or node: {}",
                            name
                        )))
                    })?;
                    (pos_idx, self.ctx.ground())
                };
                self.access_to_ir(
                    arena,
                    kind,
                    self.ctx.branch_nodes(name).is_some().then_some(name),
                    pos_idx,
                    neg_idx,
                )
            }
        }
    }

    /// Map an access function and node pair to the IR quantity.
    ///
    /// Potential accesses (V, Temp, Pos, ...) read the node-pair potential;
    /// flow accesses (I, Pwr, ...) read the branch flow.
    fn access_to_ir(
        &self,
        arena: &mut ExprArena,
        kind: AccessKind,
        name: Option<&SmolStr>,
        pos: usize,
        neg: usize,
    ) -> CompileResult<NodeId> {
        match kind {
            AccessKind::Flow => {
                if let Some((ordinal, reversed)) = self.ctx.branch_current_axis(name, pos, neg) {
                    let current = arena.push(Node::BranchCurrent(pack_index(ordinal)));
                    Ok(if reversed {
                        arena.push(Node::Unary(crate::ast::UnaryOp::Neg, current))
                    } else {
                        current
                    })
                } else {
                    Ok(arena.push(Node::Current(pack_index(pos), pack_index(neg))))
                }
            }
            AccessKind::Potential => {
                Ok(arena.push(Node::Voltage(pack_index(pos), pack_index(neg))))
            }
        }
    }

    fn resolved_access_kind(access: &BranchAccess) -> CompileResult<AccessKind> {
        access.kind().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "unresolved branch access role".into(),
            ))
            .into()
        })
    }

    /// Convert an analog operator
    fn convert_analog_operator(
        &self,
        arena: &mut ExprArena,
        op: &AnalogOperator,
    ) -> CompileResult<NodeId> {
        match op {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            } => {
                // The limiter body is the operator's value, so the bytecode
                // carries it: every lowering applies the same limiting
                // function during Newton, and none of them drops it.
                let proposed = self.convert(arena, proposed)?;
                let proposed = if let Some(polarity) = type_metadata {
                    let polarity = self.convert(arena, polarity)?;
                    arena.push(Node::Binary(BinaryOp::Mul, polarity, proposed))
                } else {
                    proposed
                };
                // The body's implicit arguments resolve against this call, so
                // the enclosing operator has to be on the stack while it is
                // converted.
                self.limiters.borrow_mut().push(proposed);
                let candidate = self.convert(arena, candidate);
                self.limiters.borrow_mut().pop();
                let candidate = candidate?;
                Ok(arena.push(Node::NamedLimit {
                    proposed,
                    candidate,
                }))
            }
            AnalogOperator::LimiterArgument { argument, .. } => {
                let Some(proposed) = self.limiters.borrow().last().copied() else {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "named $limit implicit argument escaped its limiter body".into(),
                    ))
                    .into());
                };
                Ok(match argument {
                    LimiterArgument::Proposed => proposed,
                    LimiterArgument::Previous => arena.push(Node::LimiterPrevious(proposed)),
                })
            }
        }
    }

    /// Convert a noise source
    fn convert_noise_source(
        &self,
        arena: &mut ExprArena,
        noise: &crate::ast::NoiseSource,
    ) -> CompileResult<NodeId> {
        use crate::ast::NoiseSource;
        let process_id = match noise {
            NoiseSource::White { process_id, .. }
            | NoiseSource::Flicker { process_id, .. }
            | NoiseSource::Table { process_id, .. } => process_id.ok_or_else(|| {
                CodeGenError::new(CodeGenErrorKind::Internal(
                    "analyzed noise source is missing its semantic process identity".into(),
                ))
            })?,
        };
        match noise {
            NoiseSource::White {
                process_id: _,
                power,
                name,
                span,
            } => {
                let power = self.convert(arena, power)?;
                Ok(arena.push_heavy(Heavy::WhiteNoise {
                    site: crate::ir::NoiseSiteId {
                        ordinal: process_id,
                        ..crate::ir::NoiseSiteId::from_span(*span)
                    },
                    power,
                    name: name.as_ref().map(ToString::to_string),
                }))
            }
            NoiseSource::Flicker {
                process_id: _,
                power,
                exponent,
                name,
                span,
            } => {
                let power = self.convert(arena, power)?;
                let exponent = self.convert(arena, exponent)?;
                Ok(arena.push_heavy(Heavy::FlickerNoise {
                    site: crate::ir::NoiseSiteId {
                        ordinal: process_id,
                        ..crate::ir::NoiseSiteId::from_span(*span)
                    },
                    power,
                    exponent,
                    name: name.as_ref().map(ToString::to_string),
                }))
            }
            NoiseSource::Table {
                process_id: _,
                data,
                log_interp,
                name,
                span,
            } => {
                let mut flat = Vec::with_capacity(data.len());
                for value in data {
                    let converted = self.convert(arena, value)?;
                    let folded = autodiff_fold(arena, converted);
                    match constant(arena, folded) {
                        Some(value) => flat.push(value),
                        None => {
                            return Err(CodeGenError::with_span(
                                CodeGenErrorKind::UnsupportedFeature(
                                    "noise_table entries must be compile-time constants".into(),
                                ),
                                *span,
                            )
                            .into());
                        }
                    }
                }
                if flat.is_empty() || flat.len() % 2 != 0 {
                    return Err(CodeGenError::with_span(
                        CodeGenErrorKind::InvalidExpression(
                            "noise_table needs a non-empty, even-length {f, p, ...} list".into(),
                        ),
                        *span,
                    )
                    .into());
                }
                let mut points = flat
                    .chunks_exact(2)
                    .map(|pair| (pair[0], pair[1]))
                    .collect::<Vec<_>>();
                if points.iter().any(|(frequency, power)| {
                    !frequency.is_finite() || !power.is_finite() || *frequency < 0.0 || *power < 0.0
                }) {
                    return Err(CodeGenError::with_span(
                        CodeGenErrorKind::InvalidExpression(
                            "noise_table frequencies and powers must be finite and non-negative"
                                .into(),
                        ),
                        *span,
                    )
                    .into());
                }
                points.sort_by(|left, right| left.0.total_cmp(&right.0));
                if points.windows(2).any(|pair| pair[0].0 == pair[1].0) {
                    return Err(CodeGenError::with_span(
                        CodeGenErrorKind::InvalidExpression(
                            "noise_table frequency points must be unique".into(),
                        ),
                        *span,
                    )
                    .into());
                }
                Ok(arena.push_heavy(Heavy::NoiseTable {
                    site: crate::ir::NoiseSiteId {
                        ordinal: process_id,
                        ..crate::ir::NoiseSiteId::from_span(*span)
                    },
                    points,
                    log_interp: *log_interp,
                    name: name.as_ref().map(ToString::to_string),
                }))
            }
        }
    }

    /// Convert a branch access to a BranchRef
    pub fn convert_branch_ref(&self, access: &BranchAccess) -> CompileResult<BranchRef> {
        match access {
            BranchAccess::Nodes { pos, neg, .. } => {
                if neg.is_none()
                    && let Some((pos_idx, neg_idx)) = self.ctx.branch_nodes(pos)
                {
                    return Ok(BranchRef {
                        pos_terminal: pos_idx,
                        neg_terminal: neg_idx,
                    });
                }

                let pos_idx = self.ctx.node_index(pos).ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "Unknown node: {}",
                        pos
                    )))
                })?;

                let neg_idx = neg
                    .as_ref()
                    .map(|n| {
                        self.ctx.node_index(n).ok_or_else(|| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "Unknown node: {}",
                                n
                            )))
                        })
                    })
                    .transpose()?
                    .unwrap_or(self.ctx.ground());

                Ok(BranchRef {
                    pos_terminal: pos_idx,
                    neg_terminal: neg_idx,
                })
            }
            BranchAccess::Branch { name, .. } => {
                let (pos_idx, neg_idx) = if let Some(nodes) = self.ctx.branch_nodes(name) {
                    nodes
                } else {
                    let pos_idx = self.ctx.node_index(name).ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "Unknown branch or node: {}",
                            name
                        )))
                    })?;
                    (pos_idx, self.ctx.ground())
                };
                Ok(BranchRef {
                    pos_terminal: pos_idx,
                    neg_terminal: neg_idx,
                })
            }
        }
    }

    fn parse_table_model_data(&self, func: &SystemFunction) -> CompileResult<(Vec<f64>, Vec<f64>)> {
        let data_args = &func.args[1..];

        if data_args.len() >= 2
            && data_args
                .iter()
                .all(|arg| matches!(arg, Expression::Number(_)))
        {
            if !data_args.len().is_multiple_of(2) {
                return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "$table_model numeric table data requires x/y pairs".into(),
                ))
                .into());
            }
            let mut x_data = Vec::with_capacity(data_args.len() / 2);
            let mut y_data = Vec::with_capacity(data_args.len() / 2);
            for pair in data_args.chunks_exact(2) {
                let Expression::Number(x) = &pair[0] else {
                    unreachable!()
                };
                let Expression::Number(y) = &pair[1] else {
                    unreachable!()
                };
                x_data.push(x.value);
                y_data.push(y.value);
            }
            return Self::validate_and_sort_table_data(x_data, y_data);
        }

        let Some(Expression::StringLit(spec)) = data_args.first() else {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model table data must be numeric pairs or a string table/file path".into(),
            ))
            .into());
        };

        let raw = spec.value.as_str();
        if let Ok((x_data, y_data)) = Self::parse_inline_table_spec(raw) {
            return Self::validate_and_sort_table_data(x_data, y_data);
        }

        let path = Path::new(raw);
        let table_text = std::fs::read_to_string(path).map_err(|e| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "$table_model could not parse inline table and failed to read '{}': {}",
                raw, e
            )))
        })?;
        let (x_data, y_data) = Self::parse_inline_table_spec(&table_text).map_err(|e| {
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "$table_model failed to parse table file '{}': {}",
                raw, e
            )))
        })?;
        Self::validate_and_sort_table_data(x_data, y_data)
    }

    fn parse_inline_table_spec(spec: &str) -> Result<(Vec<f64>, Vec<f64>), String> {
        let cleaned_lines: Vec<String> = spec
            .lines()
            .map(|line| line.split('!').next().unwrap_or("").trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        let normalized = cleaned_lines.join(";");
        if normalized.is_empty() {
            return Err("table data is empty".to_string());
        }

        let mut x_data = Vec::new();
        let mut y_data = Vec::new();

        let segments: Vec<&str> = normalized
            .split(';')
            .map(str::trim)
            .filter(|segment| !segment.is_empty())
            .collect();
        if !segments.is_empty() {
            let mut all_pairs = true;
            for segment in &segments {
                let segment_normalized = segment.replace(',', " ");
                let tokens: Vec<&str> = segment_normalized.split_whitespace().collect();
                if tokens.len() != 2 {
                    all_pairs = false;
                    break;
                }
                let x = tokens[0]
                    .parse::<f64>()
                    .map_err(|_| format!("invalid x value '{}'", tokens[0]))?;
                let y = tokens[1]
                    .parse::<f64>()
                    .map_err(|_| format!("invalid y value '{}'", tokens[1]))?;
                x_data.push(x);
                y_data.push(y);
            }
            if all_pairs && !x_data.is_empty() {
                return Ok((x_data, y_data));
            }
            x_data.clear();
            y_data.clear();
        }

        let normalized_commas = normalized.replace(',', " ");
        let tokens: Vec<&str> = normalized_commas.split_whitespace().collect();
        if tokens.len() < 4 || !tokens.len().is_multiple_of(2) {
            return Err("table data must contain at least two x/y pairs".to_string());
        }
        for pair in tokens.chunks_exact(2) {
            let x = pair[0]
                .parse::<f64>()
                .map_err(|_| format!("invalid x value '{}'", pair[0]))?;
            let y = pair[1]
                .parse::<f64>()
                .map_err(|_| format!("invalid y value '{}'", pair[1]))?;
            x_data.push(x);
            y_data.push(y);
        }
        Ok((x_data, y_data))
    }

    fn validate_and_sort_table_data(
        x_data: Vec<f64>,
        y_data: Vec<f64>,
    ) -> CompileResult<(Vec<f64>, Vec<f64>)> {
        if x_data.len() != y_data.len() {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model x/y data length mismatch".into(),
            ))
            .into());
        }
        if x_data.len() < 2 {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model requires at least two table points".into(),
            ))
            .into());
        }
        if x_data.iter().any(|value| !value.is_finite())
            || y_data.iter().any(|value| !value.is_finite())
        {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model table data must be finite".into(),
            ))
            .into());
        }

        let mut pairs: Vec<(f64, f64)> = x_data.into_iter().zip(y_data).collect();
        pairs.sort_by(|(x_a, _), (x_b, _)| {
            x_a.partial_cmp(x_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        for idx in 1..pairs.len() {
            if (pairs[idx].0 - pairs[idx - 1].0).abs() < 1e-30 {
                return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "$table_model x values must be strictly monotonic".into(),
                ))
                .into());
            }
        }

        let (x_sorted, y_sorted): (Vec<f64>, Vec<f64>) = pairs.into_iter().unzip();
        Ok((x_sorted, y_sorted))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        ArrayLiteralExpr, BinaryExpr, CallExpr, ConditionalExpr, NumberLit, ReplicationExpr,
        SystemFunction,
    };
    use crate::source::Span;

    fn number(value: f64) -> Expression {
        Expression::Number(NumberLit {
            value,
            raw: value.to_string().into(),
            span: Span::dummy(),
        })
    }

    fn empty_context() -> ConversionContext {
        ConversionContext {
            node_map: HashMap::new(),
            branch_map: HashMap::new(),
            branch_current_map: HashMap::new(),
            param_map: HashMap::new(),
            var_map: HashMap::new(),
            arrays: HashMap::new(),
            num_terminals: 0,
            num_internal: 0,
            default_transition: 1.0e-9,
        }
    }

    fn vector(values: &[f64], assignment_pattern: bool) -> Expression {
        Expression::ArrayLiteral(ArrayLiteralExpr {
            elements: values
                .iter()
                .copied()
                .map(number)
                .map(ArrayLiteralElement::Value)
                .collect(),
            assignment_pattern,
            span: Span::dummy(),
        })
    }

    fn replicated_vector(count: i64, elements: Vec<ArrayLiteralElement>) -> Expression {
        Expression::ArrayLiteral(ArrayLiteralExpr {
            elements: vec![ArrayLiteralElement::Replication(ReplicationExpr {
                count: Box::new(number(count as f64)),
                elements,
                span: Span::dummy(),
            })],
            assignment_pattern: true,
            span: Span::dummy(),
        })
    }

    fn inline_noise_table(name: &str, values: &[f64]) -> Expression {
        let args = vec![vector(values, true)];
        if name.starts_with('$') {
            Expression::SystemFunction(SystemFunction {
                name: name.into(),
                args,
                span: Span::dummy(),
            })
        } else {
            Expression::Call(CallExpr {
                name: name.into(),
                args,
                span: Span::dummy(),
            })
        }
    }

    /// The axis a `ddx` node names, for a fixture that asserts on one.
    fn ddx_axis_of(arena: &ExprArena, id: NodeId) -> DdxAxis {
        match *arena.node(id) {
            Node::Ddx { axis, .. } => arena.ddx_axis(axis),
            other => panic!("expected a ddx node, found {other:?}"),
        }
    }

    /// The heavy payload a node carries, for a fixture that asserts on one.
    fn heavy_of(arena: &ExprArena, id: NodeId) -> Heavy {
        match *arena.node(id) {
            Node::Heavy(_, heavy) => arena.heavy(heavy).clone(),
            other => panic!("expected a heavy operator, found {other:?}"),
        }
    }

    fn converted_noise_table(
        converter: &ExprConverter<'_>,
        name: &str,
        values: &[f64],
    ) -> CompileResult<(Vec<(f64, f64)>, bool)> {
        let arena = &mut ExprArena::new();
        let converted = converter.convert(arena, &inline_noise_table(name, values))?;
        let Heavy::NoiseTable {
            points, log_interp, ..
        } = heavy_of(arena, converted)
        else {
            panic!("{name} did not lower to a noise table");
        };
        Ok((points, log_interp))
    }

    #[test]
    fn inline_noise_tables_accept_finite_boundaries_and_sort_by_frequency() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);

        for name in ["noise_table", "$noise_table"] {
            let (points, log_interp) =
                converted_noise_table(&converter, name, &[f64::MAX, 0.0, 0.0, f64::MAX, 10.0, 1.0])
                    .unwrap_or_else(|error| panic!("{name} rejected valid linear points: {error}"));
            assert!(!log_interp);
            assert_eq!(points, vec![(0.0, f64::MAX), (10.0, 1.0), (f64::MAX, 0.0)]);
        }

        for name in ["noise_table_log", "$noise_table_log"] {
            let (points, log_interp) = converted_noise_table(
                &converter,
                name,
                &[
                    f64::MAX,
                    f64::MIN_POSITIVE,
                    f64::MIN_POSITIVE,
                    f64::MAX,
                    10.0,
                    1.0,
                ],
            )
            .unwrap_or_else(|error| panic!("{name} rejected valid log points: {error}"));
            assert!(log_interp);
            assert_eq!(
                points,
                vec![
                    (f64::MIN_POSITIVE, f64::MAX),
                    (10.0, 1.0),
                    (f64::MAX, f64::MIN_POSITIVE),
                ]
            );
        }
    }

    #[test]
    fn inline_noise_tables_reject_nonfinite_frequency_or_power() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let invalid = [
            [f64::NAN, 1.0],
            [1.0, f64::NAN],
            [f64::INFINITY, 1.0],
            [1.0, f64::NEG_INFINITY],
        ];

        for name in [
            "noise_table",
            "$noise_table",
            "noise_table_log",
            "$noise_table_log",
        ] {
            for values in invalid {
                let error = converter
                    .convert(arena, &inline_noise_table(name, &values))
                    .expect_err("nonfinite noise-table point must fail")
                    .to_string();
                assert!(
                    error.contains("frequencies and powers must be finite"),
                    "unexpected {name} diagnostic: {error}"
                );
            }
        }
    }

    #[test]
    fn inline_noise_tables_enforce_linear_and_log_domains() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();

        for name in ["noise_table", "$noise_table"] {
            for values in [[-1.0, 1.0], [1.0, -1.0]] {
                let error = converter
                    .convert(arena, &inline_noise_table(name, &values))
                    .expect_err("negative linear noise-table point must fail")
                    .to_string();
                assert!(
                    error.contains("requires non-negative frequencies and powers"),
                    "unexpected {name} diagnostic: {error}"
                );
            }
        }

        for name in ["noise_table_log", "$noise_table_log"] {
            for values in [[0.0, 1.0], [1.0, 0.0], [-1.0, 1.0], [1.0, -1.0]] {
                let error = converter
                    .convert(arena, &inline_noise_table(name, &values))
                    .expect_err("nonpositive log noise-table point must fail")
                    .to_string();
                assert!(
                    error.contains("requires strictly positive frequencies and powers"),
                    "unexpected {name} diagnostic: {error}"
                );
            }
        }
    }

    #[test]
    fn inline_noise_tables_reject_duplicate_frequencies_after_sorting() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();

        for name in [
            "noise_table",
            "$noise_table",
            "noise_table_log",
            "$noise_table_log",
        ] {
            let error = converter
                .convert(
                    arena,
                    &inline_noise_table(name, &[10.0, 1.0, 1.0, 2.0, 10.0, 3.0]),
                )
                .expect_err("duplicate noise-table frequencies must fail")
                .to_string();
            assert!(
                error.contains("frequency points must be unique")
                    && error.contains("duplicate frequency 10"),
                "unexpected {name} diagnostic: {error}"
            );
        }

        for name in ["noise_table", "$noise_table"] {
            let error = converter
                .convert(arena, &inline_noise_table(name, &[-0.0, 1.0, 0.0, 2.0]))
                .expect_err("signed zero frequencies are the same linear knot")
                .to_string();
            assert!(
                error.contains("frequency points must be unique"),
                "unexpected {name} signed-zero diagnostic: {error}"
            );
        }
    }

    #[test]
    fn transition_call_retains_a_site_correlated_derivative_carrier() {
        let mut context = empty_context();
        context.node_map.insert("p".into(), 0);
        context.node_map.insert("n".into(), 1);
        context.num_terminals = 2;
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let expression = Expression::Call(CallExpr {
            name: "transition".into(),
            args: vec![
                Expression::BranchAccess(BranchAccess::Nodes {
                    access: "V".into(),
                    kind: Some(AccessKind::Potential),
                    pos: "p".into(),
                    neg: Some("n".into()),
                    span: Span::dummy(),
                }),
                number(1.0e-9),
                number(2.0e-9),
                number(3.0e-9),
            ],
            span: Span::dummy(),
        });
        let primal = converter
            .convert(arena, &expression)
            .expect("a transition call must convert to its dynamic operator");
        let mut next = 0;
        let primal = crate::ir::autodiff::assign_transition_site_ordinals(arena, primal, &mut next);
        let derivative = crate::ir::autodiff::differentiate(
            arena,
            primal,
            &crate::ir::DerivativeWrt::Voltage(0),
        );

        let Heavy::Transition {
            site: primal_site, ..
        } = heavy_of(arena, primal)
        else {
            panic!("a transition call must remain a primal transition carrier");
        };
        let Heavy::TransitionDerivative {
            site,
            input_derivative,
            delay,
            rise_time,
            fall_time,
            ..
        } = heavy_of(arena, derivative)
        else {
            panic!("transition derivative must remain a runtime carrier");
        };
        assert_eq!(site, primal_site);
        assert_eq!(site.ordinal, 0);
        assert!(matches!(*arena.node(input_derivative), Node::Const(1.0)));
        assert!(delay.is_some() && rise_time.is_some() && fall_time.is_some());
    }

    #[test]
    fn source_filter_calls_do_not_scalarize_concatenations_or_scalars() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let call = |numerator: Expression| {
            Expression::Call(CallExpr {
                name: "laplace_nd".into(),
                args: vec![number(1.0), numerator, vector(&[1.0, 1.0], true)],
                span: Span::dummy(),
            })
        };

        let error = converter
            .convert(arena, &call(vector(&[1.0], false)))
            .expect_err("concatenation is not a coefficient vector")
            .to_string();
        assert!(error.contains("ordinary concatenation"), "got: {error}");
        assert!(error.contains("assignment pattern"), "got: {error}");

        let error = converter
            .convert(arena, &call(number(1.0)))
            .expect_err("scalar is not a coefficient vector")
            .to_string();
        assert!(error.contains("scalar expression"), "got: {error}");

        let id = converter
            .convert(arena, &call(vector(&[1.0], true)))
            .expect("assignment pattern is a coefficient vector");
        assert!(matches!(heavy_of(arena, id), Heavy::LaplaceND { .. }));
    }

    #[test]
    fn source_filter_codegen_materializes_bounded_nested_replication() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let inner = ArrayLiteralElement::Replication(ReplicationExpr {
            count: Box::new(number(2.0)),
            elements: vec![ArrayLiteralElement::Value(number(1.0))],
            span: Span::dummy(),
        });
        let expression = Expression::Call(CallExpr {
            name: "laplace_nd".into(),
            args: vec![
                number(1.0),
                replicated_vector(2, vec![inner]),
                vector(&[1.0, 1.0, 1.0, 1.0], true),
            ],
            span: Span::dummy(),
        });
        let id = converter
            .convert(arena, &expression)
            .expect("nested replication lowers defensively at codegen");
        let Heavy::LaplaceND { numerator, .. } = heavy_of(arena, id) else {
            panic!("expected a Laplace operator");
        };
        assert_eq!(numerator, vec![1.0; 4]);

        let oversized = Expression::Call(CallExpr {
            name: "laplace_nd".into(),
            args: vec![
                number(1.0),
                replicated_vector(1021, vec![ArrayLiteralElement::Value(number(1.0))]),
                vector(&[1.0], true),
            ],
            span: Span::dummy(),
        });
        let error = converter
            .convert(arena, &oversized)
            .expect_err("oversized replication fails before allocation")
            .to_string();
        assert!(error.contains("supported limit is 1020"), "{error}");
    }

    #[test]
    fn idtmod_retains_all_four_wrapping_operands() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let expression = Expression::Call(CallExpr {
            name: "idtmod".into(),
            args: vec![number(1.25), number(2.5), number(3.75), number(-4.0)],
            span: Span::dummy(),
        });

        let id = converter
            .convert(arena, &expression)
            .expect("idtmod with a modulus must retain modulo integration");
        let Node::IdtMod {
            expr,
            modulus,
            payload,
        } = *arena.node(id)
        else {
            panic!("idtmod was not lowered to a wrapping integral");
        };
        let (Some(ic), Some(offset)) = arena.optional_pair(payload) else {
            panic!("idtmod must retain its initial condition and offset");
        };
        let value = |id| match *arena.node(id) {
            Node::Const(value) => value,
            other => panic!("expected a constant, found {other:?}"),
        };
        assert_eq!(value(expr), 1.25);
        assert_eq!(value(ic), 2.5);
        assert_eq!(value(modulus), 3.75);
        assert_eq!(value(offset), -4.0);
    }

    #[test]
    fn ddx_retains_a_symbolic_derivative_instead_of_its_operand() {
        let mut context = empty_context();
        context.node_map.insert("p".into(), 0);
        context.node_map.insert("n".into(), 1);
        context.num_terminals = 2;
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let voltage = || {
            Expression::BranchAccess(BranchAccess::Nodes {
                access: "V".into(),
                kind: Some(AccessKind::Potential),
                pos: "p".into(),
                neg: Some("n".into()),
                span: Span::dummy(),
            })
        };
        let expression = Expression::Call(CallExpr {
            name: "ddx".into(),
            args: vec![
                Expression::Binary(BinaryExpr {
                    op: BinaryOp::Mul,
                    left: Box::new(voltage()),
                    right: Box::new(voltage()),
                    span: Span::dummy(),
                }),
                voltage(),
            ],
            span: Span::dummy(),
        });

        let converted = converter
            .convert(arena, &expression)
            .expect("ddx potential probe must lower symbolically");
        assert!(matches!(
            ddx_axis_of(arena, converted),
            DdxAxis::Potential {
                pos: Some(0),
                neg: Some(1),
            }
        ));
    }

    #[test]
    fn ddx_fails_closed_for_an_unrepresentable_flow_axis() {
        let mut context = empty_context();
        context.node_map.insert("p".into(), 0);
        context.node_map.insert("n".into(), 1);
        context.num_terminals = 2;
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let expression = Expression::Call(CallExpr {
            name: "ddx".into(),
            args: vec![
                number(1.0),
                Expression::BranchAccess(BranchAccess::Nodes {
                    access: "I".into(),
                    kind: Some(AccessKind::Flow),
                    pos: "p".into(),
                    neg: Some("n".into()),
                    span: Span::dummy(),
                }),
            ],
            span: Span::dummy(),
        });

        let error = converter
            .convert(arena, &expression)
            .expect_err("dependent flow must not become a ddx axis")
            .to_string();
        assert!(
            error.contains("requires a solver-owned branch-current unknown"),
            "{error}"
        );
        assert!(!error.contains("Unknown node"), "{error}");
    }

    #[test]
    fn ddx_retains_a_valid_branch_current_axis() {
        let mut context = empty_context();
        context.node_map.insert("p".into(), 0);
        context.node_map.insert("n".into(), 1);
        context
            .branch_current_map
            .insert(BranchIdentity::Nodes(0, 1), (0, 0));
        context.num_terminals = 2;
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let current = || {
            Expression::BranchAccess(BranchAccess::Nodes {
                access: "I".into(),
                kind: Some(AccessKind::Flow),
                pos: "p".into(),
                neg: Some("n".into()),
                span: Span::dummy(),
            })
        };
        let expression = Expression::Call(CallExpr {
            name: "ddx".into(),
            args: vec![current(), current()],
            span: Span::dummy(),
        });

        let id = converter
            .convert(arena, &expression)
            .expect("solver branch flow is a valid ddx axis");
        assert!(matches!(
            ddx_axis_of(arena, id),
            DdxAxis::BranchCurrent {
                ordinal: 0,
                reversed: false,
            }
        ));
    }

    #[test]
    fn ddx_preserves_resolved_magnetic_access_roles() {
        let mut context = empty_context();
        context.node_map.insert("p".into(), 0);
        context.node_map.insert("n".into(), 1);
        context
            .branch_current_map
            .insert(BranchIdentity::Nodes(0, 1), (0, 0));
        context.num_terminals = 2;
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let ddx = |access: &str| {
            Expression::Call(CallExpr {
                name: "ddx".into(),
                args: vec![
                    number(1.0),
                    Expression::BranchAccess(BranchAccess::Nodes {
                        access: access.into(),
                        kind: Some(if access == "Phi" {
                            AccessKind::Flow
                        } else {
                            AccessKind::Potential
                        }),
                        pos: "p".into(),
                        neg: Some("n".into()),
                        span: Span::dummy(),
                    }),
                ],
                span: Span::dummy(),
            })
        };

        let mmf = converter
            .convert(arena, &ddx("MMF"))
            .expect("MMF is a potential");
        assert!(matches!(ddx_axis_of(arena, mmf), DdxAxis::Potential { .. }));
        let phi = converter
            .convert(arena, &ddx("Phi"))
            .expect("Phi is a solver-owned magnetic flow");
        assert!(matches!(
            ddx_axis_of(arena, phi),
            DdxAxis::BranchCurrent { .. }
        ));
    }

    #[test]
    fn laplace_refuses_improper_and_nonconjugate_definitions() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let improper = Expression::Call(CallExpr {
            name: "laplace_nd".into(),
            args: vec![number(1.0), vector(&[1.0, 2.0], true), vector(&[0.5], true)],
            span: Span::dummy(),
        });
        let error = converter
            .convert(arena, &improper)
            .expect_err("laplace_nd must enforce proper transfer shape");
        assert!(error.to_string().contains("improper transfer function"));

        let nonconjugate = Expression::Call(CallExpr {
            name: "laplace_zp".into(),
            args: vec![
                number(1.0),
                vector(&[1.0, 2.0, 3.0, -2.0], true),
                vector(&[-1.0, 0.0, -2.0, 0.0], true),
            ],
            span: Span::dummy(),
        });
        let error = converter
            .convert(arena, &nonconjugate)
            .expect_err("laplace_zp must validate conjugate roots");
        assert!(error.to_string().contains("no conjugate partner"));
    }

    #[test]
    fn zi_lowers_only_a_definition_that_passes_validation() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let valid = Expression::Call(CallExpr {
            name: "zi_nd".into(),
            args: vec![
                number(2.0),
                vector(&[0.25], true),
                vector(&[1.0, -0.75], true),
                number(1.0e-6),
            ],
            span: Span::dummy(),
        });
        let id = converter
            .convert(arena, &valid)
            .expect("valid zi_nd definition");
        assert!(matches!(heavy_of(arena, id), Heavy::ZiFilter { .. }));

        let invalid = Expression::Call(CallExpr {
            name: "zi_nd".into(),
            args: vec![
                number(1.0),
                vector(&[1.0], true),
                vector(&[0.0], true),
                number(1.0e-6),
            ],
            span: Span::dummy(),
        });
        let error = converter
            .convert(arena, &invalid)
            .expect_err("zi_nd must reject zero a0");
        assert!(error.to_string().contains("a0 must be nonzero"));
    }

    #[test]
    fn zi_nodes_with_dummy_spans_receive_distinct_site_ordinals() {
        let unit_zi = |input| {
            Expression::Call(CallExpr {
                name: "zi_nd".into(),
                args: vec![
                    number(input),
                    vector(&[1.0], true),
                    vector(&[1.0], true),
                    number(1.0e-6),
                    number(0.0),
                ],
                span: Span::dummy(),
            })
        };
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let arena = &mut ExprArena::new();
        let assignment = converter
            .convert(arena, &unit_zi(3.0))
            .expect("assignment Zi lowers independently");
        let Heavy::ZiFilter {
            direct_assignment: assignment_direct,
            ..
        } = heavy_of(arena, assignment)
        else {
            panic!("assignment expression must remain Zi");
        };
        assert!(!assignment_direct);

        let wrapped = Expression::Conditional(ConditionalExpr {
            condition: Box::new(number(1.0)),
            then_expr: Box::new(Expression::Binary(BinaryExpr {
                op: BinaryOp::Mul,
                left: Box::new(number(2.0)),
                right: Box::new(unit_zi(4.0)),
                span: Span::dummy(),
            })),
            else_expr: Box::new(number(0.0)),
            span: Span::dummy(),
        });
        let contribution = converter
            .convert_contribution(arena, &wrapped)
            .expect("wrapped contribution Zi lowers independently");
        let Node::Conditional(_, then_expr, _) = *arena.node(contribution) else {
            panic!("contribution wrapper must remain conditional");
        };
        let Node::Binary(_, _, contribution_zi) = *arena.node(then_expr) else {
            panic!("contribution then-arm must remain arithmetic");
        };
        let Heavy::ZiFilter {
            direct_assignment: contribution_direct,
            ..
        } = heavy_of(arena, contribution_zi)
        else {
            panic!("wrapped contribution operand must remain Zi");
        };
        assert!(contribution_direct);

        let expression = Expression::Binary(BinaryExpr {
            op: BinaryOp::Add,
            left: Box::new(unit_zi(1.0)),
            right: Box::new(unit_zi(2.0)),
            span: Span::dummy(),
        });
        let converted = converter
            .convert(arena, &expression)
            .expect("independent zi_nd calls lower");
        let mut next = 0;
        let converted = crate::ir::autodiff::assign_zi_site_ordinals(arena, converted, &mut next);

        let Node::Binary(_, left, right) = *arena.node(converted) else {
            panic!("binary expression must remain binary");
        };
        let Heavy::ZiFilter { site: left, .. } = heavy_of(arena, left) else {
            panic!("left operand must remain Zi");
        };
        let Heavy::ZiFilter { site: right, .. } = heavy_of(arena, right) else {
            panic!("right operand must remain Zi");
        };
        assert_ne!(left, right, "equal/dummy spans must not alias Zi state");
        assert_eq!(left.ordinal, 0);
        assert_eq!(right.ordinal, 1);
    }
}
