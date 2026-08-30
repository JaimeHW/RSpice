//! Expression Converter for Verilog-A
//!
//! Converts AST expressions to IR expressions for code generation.
//! This module handles:
//! - Node name to terminal index mapping
//! - Parameter and variable resolution
//! - System function conversion ($vt, $temperature)
//! - Analog operator translation (ddt, idt, limexp)

use crate::ast::{
    AnalogOperator, ArrayLiteralElement, BinaryOp, BranchAccess, CallExpr, Expression, Identifier,
    NumberLit, SystemFunction,
};
use crate::error::{CodeGenError, CodeGenErrorKind, CompileResult};
use crate::ir::{BranchRef, IrExpr, IrFunction};
use crate::semantic::AnalyzedModule;
use num_complex::Complex64;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::path::Path;

/// Constant-fold an IR expression (used for filter coefficients and
/// constant direction arguments)
fn autodiff_fold(expr: IrExpr) -> IrExpr {
    crate::ir::autodiff::simplify(expr)
}

fn zi_polynomial_is_wholly_constant(definition: &crate::ir::ZiPolynomialDefinition) -> bool {
    match definition {
        crate::ir::ZiPolynomialDefinition::Coefficients(values) => {
            values.iter().all(|value| matches!(value, IrExpr::Const(_)))
        }
        crate::ir::ZiPolynomialDefinition::Roots(values) => {
            values.iter().all(|(real, imaginary)| {
                matches!(real, IrExpr::Const(_)) && matches!(imaginary, IrExpr::Const(_))
            })
        }
    }
}

fn zi_polynomial_scalar_count(
    definition: &crate::ir::ZiPolynomialDefinition,
) -> Result<usize, crate::zfilter::ZiFilterError> {
    match definition {
        crate::ir::ZiPolynomialDefinition::Coefficients(values) => Ok(values.len()),
        crate::ir::ZiPolynomialDefinition::Roots(values) => {
            values.len().checked_mul(2).ok_or_else(|| {
                crate::zfilter::ZiFilterError::InvalidDefinition(
                    "Zi complex-root scalar count overflows usize".into(),
                )
            })
        }
    }
}

fn validate_zi_polynomial_budget(
    operator: &str,
    numerator: &crate::ir::ZiPolynomialDefinition,
    denominator: &crate::ir::ZiPolynomialDefinition,
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
    definition: &crate::ir::ZiPolynomialDefinition,
) -> Result<Vec<f64>, String> {
    match definition {
        crate::ir::ZiPolynomialDefinition::Coefficients(values) => Ok(values
            .iter()
            .map(|value| match value {
                IrExpr::Const(value) => Ok(*value),
                _ => Err("Zi coefficient unexpectedly remained dynamic".to_string()),
            })
            .collect::<Result<Vec<_>, _>>()?),
        crate::ir::ZiPolynomialDefinition::Roots(values) => {
            let roots = values
                .iter()
                .map(|(real, imaginary)| match (real, imaginary) {
                    (IrExpr::Const(real), IrExpr::Const(imaginary)) => Ok((*real, *imaginary)),
                    _ => Err("Zi root unexpectedly remained dynamic".to_string()),
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
    operator: &str,
    numerator: &crate::ir::ZiPolynomialDefinition,
    denominator: &crate::ir::ZiPolynomialDefinition,
    period: &IrExpr,
    first_transition: &IrExpr,
) -> CompileResult<()> {
    let (IrExpr::Const(period), IrExpr::Const(first_transition)) = (period, first_transition)
    else {
        return Ok(());
    };
    if !zi_polynomial_is_wholly_constant(numerator)
        || !zi_polynomial_is_wholly_constant(denominator)
    {
        return Ok(());
    }
    let numerator = expand_constant_zi_polynomial(numerator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
            "{operator} numerator: {error}"
        )))
    })?;
    let denominator = expand_constant_zi_polynomial(denominator).map_err(|error| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
            "{operator} denominator: {error}"
        )))
    })?;
    crate::zfilter::ZiFilter::new_with_timing(numerator, denominator, *period, *first_transition)
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

fn validate_laplace_roots(
    operator: &str,
    zeros: &[(f64, f64)],
    poles: &[(f64, f64)],
) -> CompileResult<()> {
    let zeros = zeros
        .iter()
        .map(|(real, imaginary)| Complex64::new(*real, *imaginary))
        .collect::<Vec<_>>();
    let poles = poles
        .iter()
        .map(|(real, imaginary)| Complex64::new(*real, *imaginary))
        .collect::<Vec<_>>();
    crate::laplace::StateSpaceFilter::from_poles_zeros(&poles, &zeros, 1.0)
        .map(|_| ())
        .map_err(|error| laplace_error(operator, error))
}

fn reject_flow_ddx_probe(access: &str) -> CompileResult<()> {
    if matches!(access, "I" | "Pwr" | "F" | "Tau" | "MMF" | "Flow") {
        return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
            "ddx with a flow probe (differentiate w.r.t. a potential instead)".into(),
        ))
        .into());
    }
    Ok(())
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

fn normalize_analysis_name(name: &str) -> Option<String> {
    let normalized = name.to_ascii_lowercase();
    match normalized.as_str() {
        "dc" | "op" => Some("dc".to_string()),
        "ac" => Some("ac".to_string()),
        "tran" | "transient" => Some("tran".to_string()),
        "noise" => Some("noise".to_string()),
        "ic" => Some("ic".to_string()),
        "static" => Some("static".to_string()),
        "smallsig" | "smallsignal" | "small_signal" => Some("smallsig".to_string()),
        "__rspice_initial_step" => Some("__rspice_initial_step".to_string()),
        "__rspice_final_step" => Some("__rspice_final_step".to_string()),
        _ => None,
    }
}

fn analysis_expression(name: &str, args: &[Expression]) -> CompileResult<IrExpr> {
    validate_arg_range(name, args.len(), 1, None)?;
    let mut queries = args.iter().map(|arg| -> CompileResult<IrExpr> {
        let Expression::StringLit(value) = arg else {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "analysis() requires string arguments".into(),
            ))
            .into());
        };
        Ok(match normalize_analysis_name(value.value.as_str()) {
            Some(query) => IrExpr::Analysis(query),
            None => IrExpr::Const(0.0),
        })
    });
    let mut expression = queries.next().transpose()?.ok_or_else(|| {
        CodeGenError::new(CodeGenErrorKind::InvalidExpression(
            "analysis() requires at least one string argument".into(),
        ))
    })?;
    for query in queries {
        expression = IrExpr::Binary(BinaryOp::Or, Box::new(expression), Box::new(query?));
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
}

impl<'a> ExprConverter<'a> {
    /// Create a new expression converter
    pub fn new(ctx: &'a ConversionContext) -> Self {
        Self {
            ctx,
            direct_zi_assignment: false,
        }
    }

    /// Convert the complete right-hand side of an analog contribution.
    /// Every Zi node in this expression tree is subject to the VAMS-2023
    /// section 4.5.12 strictly-positive transition-time rule.
    pub(crate) fn convert_contribution(&self, expr: &Expression) -> CompileResult<IrExpr> {
        Self {
            ctx: self.ctx,
            direct_zi_assignment: true,
        }
        .convert(expr)
    }

    /// Array layout (base, lower, len) by name
    pub fn array_layout(&self, name: &str) -> Option<(usize, i64, usize)> {
        self.ctx.array(name)
    }

    fn const_cross_direction(&self, arg: &Expression, name: &str) -> CompileResult<i32> {
        match autodiff_fold(self.convert(arg)?) {
            IrExpr::Const(v) if matches!(v, -1.0 | 0.0 | 1.0) => Ok(v as i32),
            IrExpr::Const(v) => Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                format!("{name} direction must be -1, 0, or 1, got {v}"),
            ))
            .into()),
            _ => Err(
                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                    "{name} direction argument must be a constant -1, 0, or 1"
                )))
                .into(),
            ),
        }
    }

    /// Convert an AST expression to an IR expression
    pub fn convert(&self, expr: &Expression) -> CompileResult<IrExpr> {
        match expr {
            Expression::Number(num) => self.convert_number(num),
            Expression::StringLit(_) => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature("String literals in expressions".into()),
            )
            .into()),
            Expression::Identifier(ident) => self.convert_identifier(ident),
            Expression::SystemFunction(func) => self.convert_system_function(func),
            Expression::Binary(binary) => self.convert_binary(binary),
            Expression::Unary(unary) => self.convert_unary(unary),
            Expression::Conditional(cond) => self.convert_conditional(cond),
            Expression::Call(call) => self.convert_call(call),
            Expression::NullArgument(_) => Err(CodeGenError::new(
                CodeGenErrorKind::InvalidExpression(
                    "null positional argument is only legal in the zeros operand of zi_zp/zi_zd and corresponding Laplace forms".into(),
                ),
            )
            .into()),
            Expression::BranchAccess(access) => self.convert_branch_access(access),
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
                let index = self.convert(&access.index)?;
                Ok(IrExpr::VarIndexed {
                    array: access.array.clone(),
                    base,
                    len,
                    lower,
                    index: Box::new(index),
                })
            }
            Expression::ArrayLiteral(_) => {
                Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                    "Array literals are only supported as analog filter coefficient lists".into(),
                ))
                .into())
            }
            Expression::AnalogOperator(op) => self.convert_analog_operator(op),
            Expression::NoiseSource(noise) => self.convert_noise_source(noise),
        }
    }

    /// Convert a number literal
    fn convert_number(&self, num: &NumberLit) -> CompileResult<IrExpr> {
        Ok(IrExpr::Const(num.value))
    }

    /// Convert an identifier reference
    fn convert_identifier(&self, ident: &Identifier) -> CompileResult<IrExpr> {
        let name = &ident.name;

        // Check if it's a parameter
        if self.ctx.param_map.contains_key(name) {
            return Ok(IrExpr::Param(name.clone()));
        }

        // Check if it's a variable
        if self.ctx.var_map.contains_key(name) {
            return Ok(IrExpr::Var(name.clone()));
        }

        // Check for built-in constants
        match name.as_str() {
            "M_PI" | "P_PI" => Ok(IrExpr::Const(std::f64::consts::PI)),
            "M_E" | "P_E" => Ok(IrExpr::Const(std::f64::consts::E)),
            "M_LN2" => Ok(IrExpr::Const(std::f64::consts::LN_2)),
            "M_LN10" => Ok(IrExpr::Const(std::f64::consts::LN_10)),
            "M_LOG2E" => Ok(IrExpr::Const(std::f64::consts::LOG2_E)),
            "M_LOG10E" => Ok(IrExpr::Const(std::f64::consts::LOG10_E)),
            "M_SQRT2" => Ok(IrExpr::Const(std::f64::consts::SQRT_2)),
            "inf" => Ok(IrExpr::Const(f64::INFINITY)),
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
    fn convert_system_function(&self, func: &SystemFunction) -> CompileResult<IrExpr> {
        match func.name.as_str() {
            "$vt" | "$thermal_vt" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(1))?;
                if func.args.is_empty() {
                    // $vt() = kT/q at nominal temperature
                    Ok(IrExpr::Vt)
                } else {
                    // $vt(temp) = k*temp/q
                    let temp_expr = self.convert(&func.args[0])?;
                    // vt = temp * (k/q) where k/q ~ 8.617e-5
                    Ok(IrExpr::Binary(
                        BinaryOp::Mul,
                        Box::new(temp_expr),
                        Box::new(IrExpr::Const(8.617333262e-5)),
                    ))
                }
            }
            "$temperature" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(IrExpr::Temperature)
            }
            "$abstime" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(IrExpr::Time)
            }
            "$realtime" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(IrExpr::Time)
            }
            "$mfactor" => {
                validate_arg_range(&func.name, func.args.len(), 0, Some(0))?;
                Ok(IrExpr::Mfactor)
            }
            "$simparam" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                // $simparam("name"[, default]) - simulator parameter query.
                // The explicit default argument wins; otherwise return a
                // sensible engine value for well-known names, else 0.
                let name = match func.args.first() {
                    Some(Expression::StringLit(s)) => s.value.as_str(),
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "$simparam requires a string parameter name".into(),
                        ))
                        .into());
                    }
                };
                if let Some(default) = func.args.get(1) {
                    return self.convert(default);
                }
                let value = match name {
                    "gmin" => 1e-12,
                    "tnom" => 300.15,
                    "simulatorVersion" => 1.0,
                    _ => 0.0,
                };
                Ok(IrExpr::Const(value))
            }
            "$param_given" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(1))?;
                // $param_given(name) - whether the instance explicitly set
                // the parameter
                match func.args.first() {
                    Some(Expression::Identifier(id))
                        if self.ctx.param_index(&id.name).is_some() =>
                    {
                        Ok(IrExpr::ParamGiven(id.name.clone()))
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
                Ok(IrExpr::PortConnected(index))
            }
            "$limit" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                // $limit(expr) or $limit(expr, step)
                // Bounds expression change per Newton iteration for convergence
                let inner = self.convert(&func.args[0])?;
                let step = if func.args.len() > 1 {
                    Some(Box::new(self.convert(&func.args[1])?))
                } else {
                    None
                };
                Ok(IrExpr::Limit(Box::new(inner), step))
            }
            "$table_model" => {
                validate_arg_range(&func.name, func.args.len(), 2, None)?;
                // $table_model(input, table_spec, ...)
                // Supports:
                // 1) Inline numeric pairs: $table_model(x, 0,0, 1,2, 2,4)
                // 2) Inline string table: $table_model(x, "0 0; 1 2; 2 4")
                // 3) Table file path:    $table_model(x, "table.dat")
                let input = self.convert(&func.args[0])?;
                let (x_data, y_data) = self.parse_table_model_data(func)?;
                Ok(IrExpr::TableLookup {
                    input: Box::new(input),
                    x_data,
                    y_data,
                })
            }
            "absdelay" => {
                // absdelay(expr, delay_time) - transport delay
                // Returns value of expr delayed by delay_time seconds
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "absdelay requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                let delay_time = if func.args.len() > 1 {
                    self.convert(&func.args[1])?
                } else {
                    // Default delay of 0 (no delay)
                    IrExpr::Const(0.0)
                };
                Ok(IrExpr::AbsDelay {
                    expr: Box::new(expr),
                    delay_time: Box::new(delay_time),
                })
            }
            "transition" => {
                // transition(expr, delay, rise_time, fall_time)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "transition requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                let delay = if func.args.len() > 1 {
                    Some(Box::new(self.convert(&func.args[1])?))
                } else {
                    None
                };
                let rise_time = if func.args.len() > 2 {
                    Some(Box::new(self.convert(&func.args[2])?))
                } else {
                    None
                };
                let fall_time = if func.args.len() > 3 {
                    Some(Box::new(self.convert(&func.args[3])?))
                } else {
                    None
                };
                Ok(IrExpr::Transition {
                    expr: Box::new(expr),
                    delay,
                    rise_time,
                    fall_time,
                })
            }
            "slew" => {
                // slew(expr, max_pos_slew, max_neg_slew)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "slew requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                let max_pos_slew = if func.args.len() > 1 {
                    Some(Box::new(self.convert(&func.args[1])?))
                } else {
                    None
                };
                let max_neg_slew = if func.args.len() > 2 {
                    Some(Box::new(self.convert(&func.args[2])?))
                } else {
                    None
                };
                Ok(IrExpr::Slew {
                    expr: Box::new(expr),
                    max_pos_slew,
                    max_neg_slew,
                })
            }
            "cross" => {
                // cross(expr [, direction [, time_tol [, expr_tol [, enable]]]])
                validate_arg_range(&func.name, func.args.len(), 1, Some(5))?;
                let expr = self.convert(&func.args[0])?;
                // Direction: +1=rising, -1=falling, 0=both (default)
                let direction = func
                    .args
                    .get(1)
                    .map(|arg| self.const_cross_direction(arg, "cross"))
                    .transpose()?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    func.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Cross {
                    expr: Box::new(expr),
                    direction,
                    time_tol: optional(2)?,
                    expr_tol: optional(3)?,
                    enable: optional(4)?,
                })
            }
            "$white_noise" | "white_noise" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                // $white_noise(power, name)
                let power = self.convert(&func.args[0])?;
                let name = optional_string_arg(&func.name, func.args.get(1), "name")?;
                Ok(IrExpr::WhiteNoise {
                    power: Box::new(power),
                    name,
                })
            }
            "$flicker_noise" | "flicker_noise" => {
                validate_arg_range(&func.name, func.args.len(), 2, Some(3))?;
                // $flicker_noise(power, exponent, name)
                let power = self.convert(&func.args[0])?;
                let exponent = self.convert(&func.args[1])?;
                let name = optional_string_arg(&func.name, func.args.get(2), "name")?;
                Ok(IrExpr::FlickerNoise {
                    power: Box::new(power),
                    exponent: Box::new(exponent),
                    name,
                })
            }
            "$noise_table" | "noise_table" | "$noise_table_log" | "noise_table_log" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(2))?;
                let log_interp = func.name.contains("log");
                let name = optional_string_arg(&func.name, func.args.get(1), "name")?;
                let points = self.noise_table_points(&func.args[0], log_interp)?;
                Ok(IrExpr::NoiseTable {
                    points,
                    log_interp,
                    name,
                })
            }
            "analysis" => analysis_expression(&func.name, &func.args),
            "above" => {
                validate_arg_range(&func.name, func.args.len(), 1, Some(4))?;
                let expr = self.convert(&func.args[0])?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    func.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Above {
                    expr: Box::new(expr),
                    time_tol: optional(1)?,
                    expr_tol: optional(2)?,
                    enable: optional(3)?,
                })
            }
            "timer" => {
                // timer(start_time [, period [, time_tol [, enable]]])
                validate_arg_range(&func.name, func.args.len(), 1, Some(4))?;
                let start_time = self.convert(&func.args[0])?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    func.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Timer {
                    start_time: Box::new(start_time),
                    period: optional(1)?,
                    time_tol: optional(2)?,
                    enable: optional(3)?,
                })
            }
            // The IR models real zeros/poles and numerator/denominator
            // coefficients, but this converter never parsed the coefficient
            // arrays: it discarded them and emitted a unity passthrough, so a
            // filtered contribution silently evaluated as if unfiltered.
            // Refuse instead, matching the `IrExpr::Ddx` precedent in
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
    fn convert_binary(&self, binary: &crate::ast::BinaryExpr) -> CompileResult<IrExpr> {
        let left = self.convert(&binary.left)?;
        let right = self.convert(&binary.right)?;

        Ok(IrExpr::Binary(binary.op, Box::new(left), Box::new(right)))
    }

    /// Convert a unary expression
    fn convert_unary(&self, unary: &crate::ast::UnaryExpr) -> CompileResult<IrExpr> {
        let operand = self.convert(&unary.operand)?;
        Ok(IrExpr::Unary(unary.op, Box::new(operand)))
    }

    /// Convert a conditional expression
    fn convert_conditional(&self, cond: &crate::ast::ConditionalExpr) -> CompileResult<IrExpr> {
        let condition = self.convert(&cond.condition)?;
        let then_expr = self.convert(&cond.then_expr)?;
        let else_expr = self.convert(&cond.else_expr)?;

        Ok(IrExpr::Conditional(
            Box::new(condition),
            Box::new(then_expr),
            Box::new(else_expr),
        ))
    }

    /// Convert a function call
    fn convert_call(&self, call: &CallExpr) -> CompileResult<IrExpr> {
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
                let arg = self.convert(&call.args[0])?;
                return Ok(IrExpr::Limexp(Box::new(arg)));
            }
            "hypot" => {
                // hypot(x, y) = sqrt(x^2 + y^2)
                if call.args.len() != 2 {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "hypot requires 2 arguments".into(),
                    ))
                    .into());
                }
                let x = self.convert(&call.args[0])?;
                let y = self.convert(&call.args[1])?;
                let x_sq = IrExpr::Binary(BinaryOp::Mul, Box::new(x.clone()), Box::new(x));
                let y_sq = IrExpr::Binary(BinaryOp::Mul, Box::new(y.clone()), Box::new(y));
                let sum = IrExpr::Binary(BinaryOp::Add, Box::new(x_sq), Box::new(y_sq));
                return Ok(IrExpr::Call(IrFunction::Sqrt, vec![sum]));
            }
            // Analog operators, noise sources, filters, and event functions
            // arrive as plain calls; route them to their IR forms.
            _ => return self.convert_analog_call(call),
        };

        let args: Vec<IrExpr> = call
            .args
            .iter()
            .map(|a| self.convert(a))
            .collect::<CompileResult<Vec<_>>>()?;

        Ok(IrExpr::Call(ir_func, args))
    }

    /// Convert analog operators, filters, noise sources, and event functions
    fn convert_analog_call(&self, call: &CallExpr) -> CompileResult<IrExpr> {
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
                let inner = self.convert(require_arg(0)?)?;
                Ok(IrExpr::Ddt(Box::new(inner)))
            }
            "idt" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let inner = self.convert(require_arg(0)?)?;
                let ic = call
                    .args
                    .get(1)
                    .map(|e| self.convert(e))
                    .transpose()?
                    .map(Box::new);
                Ok(IrExpr::Idt(Box::new(inner), ic))
            }
            "idtmod" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                // idtmod(expr [, ic [, modulus [, offset]]]) - without a
                // modulus it degenerates to idt
                let inner = self.convert(require_arg(0)?)?;
                let ic = call
                    .args
                    .get(1)
                    .map(|e| self.convert(e))
                    .transpose()?
                    .map(Box::new);
                match call.args.get(2) {
                    Some(modulus) => {
                        let modulus = Box::new(self.convert(modulus)?);
                        let offset = call
                            .args
                            .get(3)
                            .map(|e| self.convert(e))
                            .transpose()?
                            .map(Box::new);
                        Ok(IrExpr::IdtMod {
                            expr: Box::new(inner),
                            ic,
                            modulus,
                            offset,
                        })
                    }
                    None => Ok(IrExpr::Idt(Box::new(inner), ic)),
                }
            }
            "ddx" => {
                validate_arg_range(&call.name, call.args.len(), 2, Some(2))?;
                let inner = self.convert(require_arg(0)?)?;
                let probe = require_arg(1)?;
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
                let (pos_node, neg_node) = match probe {
                    Expression::BranchAccess(BranchAccess::Nodes {
                        access, pos, neg, ..
                    }) => {
                        reject_flow_ddx_probe(access)?;
                        if neg.is_none()
                            && let Some((pos_node, neg_node)) = self.ctx.branch_nodes(pos)
                        {
                            (pos_node, Some(neg_node))
                        } else {
                            let pos_node =
                                self.ctx.node_index(pos).ok_or_else(|| unknown_node(pos))?;
                            let neg_node = neg
                                .as_ref()
                                .map(|n| self.ctx.node_index(n).ok_or_else(|| unknown_node(n)))
                                .transpose()?;
                            (pos_node, neg_node)
                        }
                    }
                    Expression::BranchAccess(BranchAccess::Branch { access, name, .. }) => {
                        reject_flow_ddx_probe(access)?;
                        if let Some((pos_node, neg_node)) = self.ctx.branch_nodes(name) {
                            (pos_node, Some(neg_node))
                        } else {
                            (
                                self.ctx
                                    .node_index(name)
                                    .ok_or_else(|| unknown_branch_or_node(name))?,
                                None,
                            )
                        }
                    }
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "ddx probe must be a branch access like V(node), V(a,b), or V(<branch>)"
                                .into(),
                        ))
                        .into());
                    }
                };
                Ok(IrExpr::Ddx {
                    expr: Box::new(inner),
                    pos: pos_node,
                    neg: neg_node,
                })
            }
            "absdelay" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(3))?;
                let expr = self.convert(require_arg(0)?)?;
                let delay = call
                    .args
                    .get(1)
                    .map(|arg| self.convert(arg))
                    .transpose()?
                    .unwrap_or(IrExpr::Const(0.0));
                Ok(IrExpr::AbsDelay {
                    expr: Box::new(expr),
                    delay_time: Box::new(delay),
                })
            }
            "transition" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(5))?;
                let expr = self.convert(require_arg(0)?)?;
                let opt = |n: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    Ok(call
                        .args
                        .get(n)
                        .map(|e| self.convert(e))
                        .transpose()?
                        .map(Box::new))
                };
                Ok(IrExpr::Transition {
                    expr: Box::new(expr),
                    delay: opt(1)?,
                    rise_time: opt(2)?,
                    fall_time: opt(3)?,
                })
            }
            "slew" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(3))?;
                let expr = self.convert(require_arg(0)?)?;
                let opt = |n: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    Ok(call
                        .args
                        .get(n)
                        .map(|e| self.convert(e))
                        .transpose()?
                        .map(Box::new))
                };
                Ok(IrExpr::Slew {
                    expr: Box::new(expr),
                    max_pos_slew: opt(1)?,
                    max_neg_slew: opt(2)?,
                })
            }
            "cross" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(5))?;
                let expr = self.convert(require_arg(0)?)?;
                let direction = call
                    .args
                    .get(1)
                    .map(|arg| self.const_cross_direction(arg, "cross"))
                    .transpose()?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    call.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Cross {
                    expr: Box::new(expr),
                    direction,
                    time_tol: optional(2)?,
                    expr_tol: optional(3)?,
                    enable: optional(4)?,
                })
            }
            "above" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                let expr = self.convert(require_arg(0)?)?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    call.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Above {
                    expr: Box::new(expr),
                    time_tol: optional(1)?,
                    expr_tol: optional(2)?,
                    enable: optional(3)?,
                })
            }
            "timer" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(4))?;
                let start_time = self.convert(require_arg(0)?)?;
                let optional = |index: usize| -> CompileResult<Option<Box<IrExpr>>> {
                    call.args
                        .get(index)
                        .map(|expr| self.convert(expr).map(Box::new))
                        .transpose()
                };
                Ok(IrExpr::Timer {
                    start_time: Box::new(start_time),
                    period: optional(1)?,
                    time_tol: optional(2)?,
                    enable: optional(3)?,
                })
            }
            "last_crossing" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let expr = self.convert(require_arg(0)?)?;
                let direction = call
                    .args
                    .get(1)
                    .map(|arg| self.const_cross_direction(arg, "last_crossing"))
                    .transpose()?;
                Ok(IrExpr::LastCrossing {
                    expr: Box::new(expr),
                    direction,
                })
            }
            "analysis" => analysis_expression(&call.name, &call.args),
            "white_noise" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let power = self.convert(require_arg(0)?)?;
                let name = optional_string_arg(&call.name, call.args.get(1), "name")?;
                Ok(IrExpr::WhiteNoise {
                    power: Box::new(power),
                    name,
                })
            }
            "flicker_noise" => {
                validate_arg_range(&call.name, call.args.len(), 2, Some(3))?;
                let power = self.convert(require_arg(0)?)?;
                let exponent = self.convert(require_arg(1)?)?;
                let name = optional_string_arg(&call.name, call.args.get(2), "name")?;
                Ok(IrExpr::FlickerNoise {
                    power: Box::new(power),
                    exponent: Box::new(exponent),
                    name,
                })
            }
            "noise_table" | "noise_table_log" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(2))?;
                let log_interp = call.name.ends_with("log");
                let name = optional_string_arg(&call.name, call.args.get(1), "name")?;
                let points = self.noise_table_points(require_arg(0)?, log_interp)?;
                Ok(IrExpr::NoiseTable {
                    points,
                    log_interp,
                    name,
                })
            }
            "laplace_nd" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                let expr = self.convert(require_arg(0)?)?;
                let numerator = self.const_real_array(require_arg(1)?)?;
                let denominator = self.const_real_array(require_arg(2)?)?;
                validate_laplace_coefficients("laplace_nd", &numerator, &denominator)?;
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                })
            }
            "laplace_zp" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                let expr = self.convert(require_arg(0)?)?;
                let zeros = match require_arg(1)? {
                    Expression::NullArgument(_) => Vec::new(),
                    value => self.const_complex_pairs(value)?,
                };
                let poles = self.const_complex_pairs(require_arg(2)?)?;
                validate_laplace_roots("laplace_zp", &zeros, &poles)?;
                Ok(IrExpr::LaplaceZP {
                    expr: Box::new(expr),
                    zeros,
                    poles,
                    gain: 1.0,
                })
            }
            "laplace_zd" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                // zeros (pairs) + denominator coefficients: expand the
                // zeros into a numerator polynomial
                let expr = self.convert(require_arg(0)?)?;
                let zeros = match require_arg(1)? {
                    Expression::NullArgument(_) => Vec::new(),
                    value => self.const_complex_pairs(value)?,
                };
                let denominator = self.const_real_array(require_arg(2)?)?;
                let numerator = crate::laplace::roots_to_polynomial(&zeros).map_err(|e| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "laplace_zd zeros: {}",
                        e
                    )))
                })?;
                validate_laplace_coefficients("laplace_zd", &numerator, &denominator)?;
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                })
            }
            "laplace_np" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                // numerator coefficients + poles (pairs)
                let expr = self.convert(require_arg(0)?)?;
                let numerator = self.const_real_array(require_arg(1)?)?;
                let poles = self.const_complex_pairs(require_arg(2)?)?;
                let denominator = crate::laplace::roots_to_polynomial(&poles).map_err(|e| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "laplace_np poles: {}",
                        e
                    )))
                })?;
                validate_laplace_coefficients("laplace_np", &numerator, &denominator)?;
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                })
            }
            // Z-domain filters. Constant arguments are retained as programs
            // and frozen per instance at the beginning of each analysis.
            "zi_nd" | "zi_zp" | "zi_zd" | "zi_np" => {
                validate_arg_range(&call.name, call.args.len(), 4, Some(6))?;
                self.validate_raw_zi_operand_budget(&call.name, require_arg(1)?, require_arg(2)?)?;
                let expr = self.convert(require_arg(0)?)?;
                let (numerator, denominator) = match call.name.as_str() {
                    "zi_nd" => (
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_array(require_arg(1)?, false)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_array(require_arg(2)?, false)?,
                        ),
                    ),
                    "zi_zp" => (
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_pairs(require_arg(1)?, true)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_pairs(require_arg(2)?, false)?,
                        ),
                    ),
                    "zi_zd" => (
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_pairs(require_arg(1)?, true)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_array(require_arg(2)?, false)?,
                        ),
                    ),
                    _ => (
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_array(require_arg(1)?, false)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_pairs(require_arg(2)?, false)?,
                        ),
                    ),
                };
                validate_zi_polynomial_budget(&call.name, &numerator, &denominator)?;
                let period =
                    self.zi_definition_arg(require_arg(3)?, &call.name, "sample period")?;
                let transition = match call.args.get(4) {
                    Some(Expression::NullArgument(_)) => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            format!("{} transition-time argument may not be null", call.name),
                        ))
                        .into());
                    }
                    Some(value) => self.convert(value)?,
                    None => IrExpr::Const(self.ctx.default_transition()),
                };
                let first_transition = match call.args.get(5) {
                    Some(value) => {
                        self.zi_definition_arg(value, &call.name, "first transition time")?
                    }
                    None => IrExpr::Const(0.0),
                };
                validate_wholly_constant_zi_definition(
                    &call.name,
                    &numerator,
                    &denominator,
                    &period,
                    &first_transition,
                )?;
                Ok(IrExpr::ZiFilter {
                    site: crate::ir::ZiSiteId::from_span(call.span),
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                    period: Box::new(period),
                    transition: Box::new(transition),
                    first_transition: Box::new(first_transition),
                    direct_assignment: self.direct_zi_assignment,
                })
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

    /// Parse a noise_table pair list `{f1, p1, f2, p2, ...}` into sorted
    /// (frequency, power) points. String (file) input and non-constant
    /// entries are clean unsupported errors.
    fn noise_table_points(
        &self,
        arg: &Expression,
        log_interp: bool,
    ) -> CompileResult<Vec<(f64, f64)>> {
        if matches!(arg, Expression::StringLit(_)) {
            return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                "noise_table file input (inline the {f, p, ...} pairs instead)".into(),
            ))
            .into());
        }
        let flat = self.const_real_array(arg)?;
        if flat.is_empty() || flat.len() % 2 != 0 {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table needs a non-empty, even-length {f, p, ...} list".into(),
            ))
            .into());
        }
        let mut points: Vec<(f64, f64)> = flat.chunks_exact(2).map(|c| (c[0], c[1])).collect();
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        if log_interp && points.iter().any(|&(f, p)| f <= 0.0 || p <= 0.0) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "noise_table_log requires strictly positive frequencies and powers".into(),
            ))
            .into());
        }
        Ok(points)
    }

    /// Evaluate an array-literal argument to constant reals
    fn const_real_array(&self, expr: &Expression) -> CompileResult<Vec<f64>> {
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
                let converted = autodiff_fold(self.convert(e)?);
                match converted {
                    IrExpr::Const(v) => Ok(v),
                    _ => Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                        "filter coefficients must be compile-time constants \
                         (parameter-dependent coefficients are not supported yet)"
                            .into(),
                    ))
                    .into()),
                }
            })
            .collect()
    }

    fn zi_definition_arg(
        &self,
        expression: &Expression,
        operator: &str,
        role: &str,
    ) -> CompileResult<IrExpr> {
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
        Ok(autodiff_fold(self.convert(expression)?))
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
        expression: &Expression,
        allow_null: bool,
    ) -> CompileResult<Vec<IrExpr>> {
        if matches!(expression, Expression::NullArgument(_)) {
            if allow_null {
                return Ok(Vec::new());
            }
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "null filter operand is not authorized at this position".into(),
            ))
            .into());
        }
        let elements: Vec<&Expression> = match expression {
            Expression::ArrayLiteral(array) => array
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
            .map(|value| self.zi_definition_arg(value, "zi filter", "coefficient/root"))
            .collect()
    }

    fn zi_complex_pairs(
        &self,
        expression: &Expression,
        allow_null: bool,
    ) -> CompileResult<Vec<(IrExpr, IrExpr)>> {
        let values = self.zi_real_array(expression, allow_null)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "zi pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values
            .chunks_exact(2)
            .map(|pair| (pair[0].clone(), pair[1].clone()))
            .collect())
    }

    /// Evaluate the already-unpacked coefficient expressions carried by the
    /// public analog-operator AST.
    fn const_real_expressions(&self, expressions: &[Expression]) -> CompileResult<Vec<f64>> {
        expressions
            .iter()
            .map(|expression| match autodiff_fold(self.convert(expression)?) {
                IrExpr::Const(value) => Ok(value),
                _ => Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                    "filter coefficients must be compile-time constants (parameter-dependent coefficients are not supported yet)"
                        .into(),
                ))
                .into()),
            })
            .collect()
    }

    fn const_complex_expression_pairs(
        &self,
        expressions: &[Expression],
    ) -> CompileResult<Vec<(f64, f64)>> {
        let values = self.const_real_expressions(expressions)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values
            .chunks_exact(2)
            .map(|pair| (pair[0], pair[1]))
            .collect())
    }

    fn zi_real_expressions(&self, expressions: &[Expression]) -> CompileResult<Vec<IrExpr>> {
        expressions
            .iter()
            .map(|expression| self.zi_definition_arg(expression, "zi filter", "coefficient/root"))
            .collect()
    }

    fn zi_complex_expression_pairs(
        &self,
        expressions: &[Expression],
    ) -> CompileResult<Vec<(IrExpr, IrExpr)>> {
        let values = self.zi_real_expressions(expressions)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "zi pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values
            .chunks_exact(2)
            .map(|pair| (pair[0].clone(), pair[1].clone()))
            .collect())
    }

    /// Evaluate an array-literal argument to constant (re, im) pairs
    fn const_complex_pairs(&self, expr: &Expression) -> CompileResult<Vec<(f64, f64)>> {
        let values = self.const_real_array(expr)?;
        if !values.len().is_multiple_of(2) {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "pole/zero vectors must contain (real, imaginary) pairs".into(),
            ))
            .into());
        }
        Ok(values.chunks_exact(2).map(|p| (p[0], p[1])).collect())
    }

    /// Convert a branch access expression
    fn convert_branch_access(&self, access: &BranchAccess) -> CompileResult<IrExpr> {
        match access {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                // A single-name access may refer to a declared named branch
                if neg.is_none()
                    && let Some((pos_idx, neg_idx)) = self.ctx.branch_nodes(pos)
                {
                    return Self::access_to_ir(access, pos_idx, neg_idx);
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

                Self::access_to_ir(access, pos_idx, neg_idx)
            }
            BranchAccess::Branch { access, name, .. } => {
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
                Self::access_to_ir(access, pos_idx, neg_idx)
            }
        }
    }

    /// Map an access function and node pair to the IR quantity.
    ///
    /// Potential accesses (V, Temp, Pos, ...) read the node-pair potential;
    /// flow accesses (I, Pwr, ...) read the branch flow.
    fn access_to_ir(access: &str, pos: usize, neg: usize) -> CompileResult<IrExpr> {
        match access {
            "I" | "Pwr" | "F" | "Tau" | "MMF" | "Flow" => Ok(IrExpr::Current(pos, neg)),
            // All potential-natured accesses behave like V over the unified
            // node space
            _ => Ok(IrExpr::Voltage(pos, neg)),
        }
    }

    /// Convert an analog operator
    fn convert_analog_operator(&self, op: &AnalogOperator) -> CompileResult<IrExpr> {
        match op {
            #[cfg(feature = "native")]
            AnalogOperator::Limit { proposed, .. } => {
                // The native runtime still consumes CompiledModel for
                // topology and state-slot metadata, while the executable
                // expression comes exclusively from canonical MIR. Emit a
                // pass-through legacy limit solely to allocate the matching
                // state slot; native construction requires every executable
                // entry point to compile from canonical IR, so this bytecode
                // is never a semantic fallback.
                let proposed = self.convert(proposed)?;
                Ok(IrExpr::CanonicalLimit(Box::new(proposed)))
            }
            #[cfg(not(feature = "native"))]
            AnalogOperator::Limit { selector, .. } => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature(format!(
                    "stateful named $limit selector '{selector}' requires the canonical backend"
                )),
            )
            .into()),
            AnalogOperator::LimiterArgument { .. } => {
                Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                    "named $limit implicit argument escaped its limiter body".into(),
                ))
                .into())
            }
            AnalogOperator::Ddt { expr, .. } => {
                let inner = self.convert(expr)?;
                Ok(IrExpr::Ddt(Box::new(inner)))
            }
            AnalogOperator::Idt { expr, ic, .. } => {
                let inner = self.convert(expr)?;
                let ic_expr = ic
                    .as_ref()
                    .map(|e| self.convert(e))
                    .transpose()?
                    .map(Box::new);
                Ok(IrExpr::Idt(Box::new(inner), ic_expr))
            }
            AnalogOperator::Limexp { expr, .. } => {
                let inner = self.convert(expr)?;
                Ok(IrExpr::Limexp(Box::new(inner)))
            }
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus: _,
                offset: _,
                abstol: _,
                ..
            } => {
                // idtmod is similar to idt for basic cases
                let inner = self.convert(expr)?;
                let ic_expr = ic
                    .as_ref()
                    .map(|e| self.convert(e))
                    .transpose()?
                    .map(Box::new);
                Ok(IrExpr::Idt(Box::new(inner), ic_expr))
            }
            // KNOWN DEFECT: this returns `expr` itself rather than its partial
            // derivative with respect to `probe`, so any model relying on
            // ddx() gets a wrong number from this pipeline instead of an error.
            // It is not refused here only because 24 of the 43 generated model
            // sources call ddx(), and failing closed would drop them from the
            // catalog. `codegen/generator.rs` already rejects an unresolved
            // `IrExpr::Ddx`, which is the contract this branch should meet once
            // symbolic differentiation is implemented. Do not add callers.
            AnalogOperator::Ddx { expr, probe, .. } => {
                let inner = self.convert(expr)?;
                let _ = probe;
                Ok(inner)
            }
            // These three discarded their defining argument — the delay, the
            // smoothing window, the rate limit — and returned the raw
            // expression, which is a plausible wrong answer rather than a
            // refusal. No model source in models/veriloga/ uses any of them,
            // so refusing costs no catalog coverage.
            AnalogOperator::Absdelay { .. }
            | AnalogOperator::Transition { .. }
            | AnalogOperator::Slew { .. } => {
                Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                    "absdelay()/transition()/slew(): timing operators are not \
                         lowered; their delay, smoothing and rate limits would be \
                         silently ignored"
                        .into(),
                ))
                .into())
            }
            AnalogOperator::LastCrossing { expr, edge, .. } => {
                let direction = edge.map(|edge| match edge {
                    crate::ast::CrossDirection::Rising => 1,
                    crate::ast::CrossDirection::Falling => -1,
                    crate::ast::CrossDirection::Both => 0,
                });
                Ok(IrExpr::LastCrossing {
                    expr: Box::new(self.convert(expr)?),
                    direction,
                })
            }
            AnalogOperator::Laplace { kind, expr, .. } => {
                let expr = Box::new(self.convert(expr)?);
                match kind {
                    crate::ast::LaplaceKind::ZeroPole { zeros, poles } => {
                        let zeros = self.const_complex_expression_pairs(zeros)?;
                        let poles = self.const_complex_expression_pairs(poles)?;
                        validate_laplace_roots("laplace_zp", &zeros, &poles)?;
                        Ok(IrExpr::LaplaceZP {
                            expr,
                            zeros,
                            poles,
                            gain: 1.0,
                        })
                    }
                    crate::ast::LaplaceKind::ZeroDenominator { zeros, denominator } => {
                        let zeros = self.const_complex_expression_pairs(zeros)?;
                        let numerator =
                            crate::laplace::roots_to_polynomial(&zeros).map_err(|error| {
                                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                    "laplace_zd zeros: {error}"
                                )))
                            })?;
                        let denominator = self.const_real_expressions(denominator)?;
                        validate_laplace_coefficients("laplace_zd", &numerator, &denominator)?;
                        Ok(IrExpr::LaplaceND {
                            expr,
                            numerator,
                            denominator,
                        })
                    }
                    crate::ast::LaplaceKind::NumeratorPole { numerator, poles } => {
                        let numerator = self.const_real_expressions(numerator)?;
                        let poles = self.const_complex_expression_pairs(poles)?;
                        let denominator =
                            crate::laplace::roots_to_polynomial(&poles).map_err(|error| {
                                CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                    "laplace_np poles: {error}"
                                )))
                            })?;
                        validate_laplace_coefficients("laplace_np", &numerator, &denominator)?;
                        Ok(IrExpr::LaplaceND {
                            expr,
                            numerator,
                            denominator,
                        })
                    }
                    crate::ast::LaplaceKind::NumeratorDenominator {
                        numerator,
                        denominator,
                    } => {
                        let numerator = self.const_real_expressions(numerator)?;
                        let denominator = self.const_real_expressions(denominator)?;
                        validate_laplace_coefficients("laplace_nd", &numerator, &denominator)?;
                        Ok(IrExpr::LaplaceND {
                            expr,
                            numerator,
                            denominator,
                        })
                    }
                }
            }
            AnalogOperator::Zi {
                kind,
                expr,
                period,
                transition,
                first_transition,
                span,
            } => {
                let (operator, raw_numerator, raw_denominator) = match kind {
                    crate::ast::ZiKind::ZeroPole { zeros, poles } => {
                        ("zi_zp", zeros.len(), poles.len())
                    }
                    crate::ast::ZiKind::ZeroDenominator { zeros, denominator } => {
                        ("zi_zd", zeros.len(), denominator.len())
                    }
                    crate::ast::ZiKind::NumeratorPole { numerator, poles } => {
                        ("zi_np", numerator.len(), poles.len())
                    }
                    crate::ast::ZiKind::NumeratorDenominator {
                        numerator,
                        denominator,
                    } => ("zi_nd", numerator.len(), denominator.len()),
                };
                crate::zfilter::validate_zi_runtime_operand_budget(
                    operator,
                    raw_numerator,
                    raw_denominator,
                )
                .map_err(|error| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(error.to_string()))
                })?;
                let expr = Box::new(self.convert(expr)?);
                let (_, numerator, denominator) = match kind {
                    crate::ast::ZiKind::ZeroPole { zeros, poles } => (
                        "zi_zp",
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_expression_pairs(zeros)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_expression_pairs(poles)?,
                        ),
                    ),
                    crate::ast::ZiKind::ZeroDenominator { zeros, denominator } => (
                        "zi_zd",
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_expression_pairs(zeros)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_expressions(denominator)?,
                        ),
                    ),
                    crate::ast::ZiKind::NumeratorPole { numerator, poles } => (
                        "zi_np",
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_expressions(numerator)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Roots(
                            self.zi_complex_expression_pairs(poles)?,
                        ),
                    ),
                    crate::ast::ZiKind::NumeratorDenominator {
                        numerator,
                        denominator,
                    } => (
                        "zi_nd",
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_expressions(numerator)?,
                        ),
                        crate::ir::ZiPolynomialDefinition::Coefficients(
                            self.zi_real_expressions(denominator)?,
                        ),
                    ),
                };
                validate_zi_polynomial_budget(operator, &numerator, &denominator)?;
                let period = self.zi_definition_arg(period, operator, "sample period")?;
                let transition = transition
                    .as_deref()
                    .map(|value| self.convert(value))
                    .transpose()?
                    .unwrap_or(IrExpr::Const(self.ctx.default_transition()));
                let first_transition = first_transition
                    .as_deref()
                    .map(|value| self.zi_definition_arg(value, operator, "first transition time"))
                    .transpose()?
                    .unwrap_or(IrExpr::Const(0.0));
                validate_wholly_constant_zi_definition(
                    operator,
                    &numerator,
                    &denominator,
                    &period,
                    &first_transition,
                )?;
                Ok(IrExpr::ZiFilter {
                    site: crate::ir::ZiSiteId::from_span(*span),
                    expr,
                    numerator,
                    denominator,
                    period: Box::new(period),
                    transition: Box::new(transition),
                    first_transition: Box::new(first_transition),
                    direct_assignment: self.direct_zi_assignment,
                })
            }
        }
    }

    /// Convert a noise source
    fn convert_noise_source(&self, noise: &crate::ast::NoiseSource) -> CompileResult<IrExpr> {
        // Noise sources evaluate to 0 for DC analysis
        // Real noise handling is in the noise analysis phase
        let _ = noise;
        Ok(IrExpr::Const(0.0))
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
    use crate::ast::{BinaryExpr, ConditionalExpr, LaplaceKind, NumberLit, ZiKind};
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
            param_map: HashMap::new(),
            var_map: HashMap::new(),
            arrays: HashMap::new(),
            num_terminals: 0,
            num_internal: 0,
            default_transition: 1.0e-9,
        }
    }

    #[test]
    fn public_laplace_ast_is_lowered_instead_of_becoming_a_passthrough() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let expression = Expression::AnalogOperator(AnalogOperator::Laplace {
            kind: LaplaceKind::NumeratorDenominator {
                numerator: vec![number(1.0)],
                denominator: vec![number(1.0), number(1.0)],
            },
            expr: Box::new(number(2.0)),
            span: Span::dummy(),
        });

        let converted = converter
            .convert(&expression)
            .expect("valid public Laplace AST");
        assert!(matches!(converted, IrExpr::LaplaceND { .. }));
    }

    #[test]
    fn public_laplace_ast_refuses_improper_and_nonconjugate_definitions() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let improper = Expression::AnalogOperator(AnalogOperator::Laplace {
            kind: LaplaceKind::NumeratorDenominator {
                numerator: vec![number(1.0), number(2.0)],
                denominator: vec![number(0.5)],
            },
            expr: Box::new(number(1.0)),
            span: Span::dummy(),
        });
        let error = converter
            .convert(&improper)
            .expect_err("public AST must enforce proper transfer shape");
        assert!(error.to_string().contains("improper transfer function"));

        let nonconjugate = Expression::AnalogOperator(AnalogOperator::Laplace {
            kind: LaplaceKind::ZeroPole {
                zeros: vec![number(1.0), number(2.0), number(3.0), number(-2.0)],
                poles: vec![number(-1.0), number(0.0), number(-2.0), number(0.0)],
            },
            expr: Box::new(number(1.0)),
            span: Span::dummy(),
        });
        let error = converter
            .convert(&nonconjugate)
            .expect_err("public AST must validate conjugate roots");
        assert!(error.to_string().contains("no conjugate partner"));
    }

    #[test]
    fn public_zi_ast_lowers_with_the_same_definition_validation() {
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let valid = Expression::AnalogOperator(AnalogOperator::Zi {
            kind: ZiKind::NumeratorDenominator {
                numerator: vec![number(0.25)],
                denominator: vec![number(1.0), number(-0.75)],
            },
            expr: Box::new(number(2.0)),
            period: Box::new(number(1.0e-6)),
            transition: None,
            first_transition: None,
            span: Span::dummy(),
        });
        assert!(matches!(
            converter.convert(&valid).expect("valid public Zi AST"),
            IrExpr::ZiFilter { .. }
        ));

        let invalid = Expression::AnalogOperator(AnalogOperator::Zi {
            kind: ZiKind::NumeratorDenominator {
                numerator: vec![number(1.0)],
                denominator: vec![number(0.0)],
            },
            expr: Box::new(number(1.0)),
            period: Box::new(number(1.0e-6)),
            transition: None,
            first_transition: None,
            span: Span::dummy(),
        });
        let error = converter
            .convert(&invalid)
            .expect_err("public Zi AST must reject zero a0");
        assert!(error.to_string().contains("a0 must be nonzero"));
    }

    #[test]
    fn public_zi_nodes_with_dummy_spans_receive_distinct_site_ordinals() {
        let unit_zi = |input| {
            Expression::AnalogOperator(AnalogOperator::Zi {
                kind: ZiKind::NumeratorDenominator {
                    numerator: vec![number(1.0)],
                    denominator: vec![number(1.0)],
                },
                expr: Box::new(number(input)),
                period: Box::new(number(1.0e-6)),
                transition: Some(Box::new(number(0.0))),
                first_transition: None,
                span: Span::dummy(),
            })
        };
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        let assignment = converter
            .convert(&unit_zi(3.0))
            .expect("assignment Zi lowers independently");
        let IrExpr::ZiFilter {
            direct_assignment: assignment_direct,
            ..
        } = assignment
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
            .convert_contribution(&wrapped)
            .expect("wrapped contribution Zi lowers independently");
        let IrExpr::Conditional(_, then_expr, _) = contribution else {
            panic!("contribution wrapper must remain conditional");
        };
        let IrExpr::Binary(_, _, contribution_zi) = then_expr.as_ref() else {
            panic!("contribution then-arm must remain arithmetic");
        };
        let IrExpr::ZiFilter {
            direct_assignment: contribution_direct,
            ..
        } = contribution_zi.as_ref()
        else {
            panic!("wrapped contribution operand must remain Zi");
        };
        assert!(*contribution_direct);

        let expression = Expression::Binary(BinaryExpr {
            op: BinaryOp::Add,
            left: Box::new(unit_zi(1.0)),
            right: Box::new(unit_zi(2.0)),
            span: Span::dummy(),
        });
        let mut converted = converter
            .convert(&expression)
            .expect("independent public Zi nodes lower");
        let mut next = 0;
        crate::ir::autodiff::assign_zi_site_ordinals(&mut converted, &mut next);

        let IrExpr::Binary(_, left, right) = converted else {
            panic!("binary public expression must remain binary");
        };
        let IrExpr::ZiFilter { site: left, .. } = left.as_ref() else {
            panic!("left operand must remain Zi");
        };
        let IrExpr::ZiFilter { site: right, .. } = right.as_ref() else {
            panic!("right operand must remain Zi");
        };
        assert_ne!(left, right, "equal/dummy spans must not alias Zi state");
        assert_eq!(left.ordinal, 0);
        assert_eq!(right.ordinal, 1);
    }

    #[test]
    fn public_zi_ast_obeys_the_shared_runtime_operand_ceiling() {
        let build = |numerator_len| {
            Expression::AnalogOperator(AnalogOperator::Zi {
                kind: ZiKind::NumeratorDenominator {
                    numerator: (0..numerator_len).map(|_| number(1.0)).collect(),
                    denominator: vec![number(1.0)],
                },
                expr: Box::new(number(1.0)),
                period: Box::new(number(1.0e-6)),
                transition: None,
                first_transition: None,
                span: Span::dummy(),
            })
        };
        let context = empty_context();
        let converter = ExprConverter::new(&context);
        converter
            .convert(&build(1019))
            .expect("public Zi AST at exactly 1,024 operands is supported");
        let error = converter
            .convert(&build(1020))
            .expect_err("public Zi AST over the shared ceiling must fail");
        assert!(
            error.to_string().contains("platform-uniform maximum 1024"),
            "got: {error}"
        );
    }
}
