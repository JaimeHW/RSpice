//! Expression Converter for Verilog-A
//!
//! Converts AST expressions to IR expressions for code generation.
//! This module handles:
//! - Node name to terminal index mapping
//! - Parameter and variable resolution
//! - System function conversion ($vt, $temperature)
//! - Analog operator translation (ddt, idt, limexp)

use crate::ast::{
    AnalogOperator, BinaryOp, BranchAccess, CallExpr, Expression, Identifier, NumberLit,
    SystemFunction,
};
use crate::error::{CodeGenError, CodeGenErrorKind, CompileResult};
use crate::ir::{BranchRef, IrExpr, IrFunction};
use crate::semantic::AnalyzedModule;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::path::Path;

/// Constant-fold an IR expression (used for filter coefficients and
/// constant direction arguments)
fn autodiff_fold(expr: IrExpr) -> IrExpr {
    crate::ir::autodiff::simplify(expr)
}

/// Map a z-root expansion failure into a compile error
fn zi_root_error(message: String) -> crate::error::CompileError {
    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
        "zi filter roots: {message}"
    )))
    .into()
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

fn validate_analysis_name(name: &str) -> CompileResult<String> {
    let normalized = name.to_ascii_lowercase();
    match normalized.as_str() {
        "dc" | "op" => Ok("dc".to_string()),
        "ac" => Ok("ac".to_string()),
        "tran" | "transient" => Ok("tran".to_string()),
        "noise" => Ok("noise".to_string()),
        "ic" => Ok("ic".to_string()),
        "static" => Ok("static".to_string()),
        "smallsig" | "smallsignal" | "small_signal" => Ok("smallsig".to_string()),
        _ => Err(
            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                "analysis() unknown analysis name '{name}'"
            )))
            .into(),
        ),
    }
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
}

/// Expression converter
///
/// Converts AST expressions to IR expressions using the provided context.
pub struct ExprConverter<'a> {
    ctx: &'a ConversionContext,
}

impl<'a> ExprConverter<'a> {
    /// Create a new expression converter
    pub fn new(ctx: &'a ConversionContext) -> Self {
        Self { ctx }
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
            "analysis" => {
                // analysis("dc"), analysis("ac"), analysis("tran")
                validate_arg_range(&func.name, func.args.len(), 1, Some(1))?;
                let analysis_type = match func.args.first() {
                    Some(crate::ast::Expression::StringLit(s)) => {
                        validate_analysis_name(s.value.as_str())?
                    }
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "analysis() requires a string argument".into(),
                        ))
                        .into());
                    }
                };
                Ok(IrExpr::Analysis(analysis_type))
            }
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
            "laplace_zp" => {
                // laplace_zp(expr, zeros, poles)
                // For now, simplified: just takes the input expression
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "laplace_zp() requires input expression".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                // zeros and poles would be parsed from array arguments
                // For now, treat as empty (passthrough)
                Ok(IrExpr::LaplaceZP {
                    expr: Box::new(expr),
                    zeros: Vec::new(),
                    poles: Vec::new(),
                    gain: 1.0, // Unity gain for passthrough
                })
            }
            "laplace_nd" => {
                // laplace_nd(expr, num_coeffs, den_coeffs)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "laplace_nd() requires input expression".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                // num/den coefficients parsed from arrays
                // For now, unity transfer function [1], [1]
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator: vec![1.0],
                    denominator: vec![1.0],
                })
            }
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
            "asinh" => {
                // asinh(x) = ln(x + sqrt(x^2 + 1))
                if call.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "asinh requires an argument".into(),
                    ))
                    .into());
                }
                let x = self.convert(&call.args[0])?;
                let x_sq = IrExpr::Binary(BinaryOp::Mul, Box::new(x.clone()), Box::new(x.clone()));
                let sum =
                    IrExpr::Binary(BinaryOp::Add, Box::new(x_sq), Box::new(IrExpr::Const(1.0)));
                let sqrt_part = IrExpr::Call(IrFunction::Sqrt, vec![sum]);
                let arg = IrExpr::Binary(BinaryOp::Add, Box::new(x), Box::new(sqrt_part));
                return Ok(IrExpr::Call(IrFunction::Log, vec![arg]));
            }
            "acosh" => {
                // acosh(x) = ln(x + sqrt(x^2 - 1))
                if call.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "acosh requires an argument".into(),
                    ))
                    .into());
                }
                let x = self.convert(&call.args[0])?;
                let x_sq = IrExpr::Binary(BinaryOp::Mul, Box::new(x.clone()), Box::new(x.clone()));
                let diff =
                    IrExpr::Binary(BinaryOp::Sub, Box::new(x_sq), Box::new(IrExpr::Const(1.0)));
                let sqrt_part = IrExpr::Call(IrFunction::Sqrt, vec![diff]);
                let arg = IrExpr::Binary(BinaryOp::Add, Box::new(x), Box::new(sqrt_part));
                return Ok(IrExpr::Call(IrFunction::Log, vec![arg]));
            }
            "atanh" => {
                // atanh(x) = 0.5 * ln((1+x)/(1-x))
                if call.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "atanh requires an argument".into(),
                    ))
                    .into());
                }
                let x = self.convert(&call.args[0])?;
                let one_plus = IrExpr::Binary(
                    BinaryOp::Add,
                    Box::new(IrExpr::Const(1.0)),
                    Box::new(x.clone()),
                );
                let one_minus =
                    IrExpr::Binary(BinaryOp::Sub, Box::new(IrExpr::Const(1.0)), Box::new(x));
                let ratio = IrExpr::Binary(BinaryOp::Div, Box::new(one_plus), Box::new(one_minus));
                let ln_ratio = IrExpr::Call(IrFunction::Log, vec![ratio]);
                return Ok(IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Const(0.5)),
                    Box::new(ln_ratio),
                ));
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
            "last_crossing" => Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                "last_crossing() requires crossing-time history; it is not implemented yet".into(),
            ))
            .into()),
            "analysis" => {
                validate_arg_range(&call.name, call.args.len(), 1, Some(1))?;
                let analysis_type = match call.args.first() {
                    Some(Expression::StringLit(s)) => validate_analysis_name(s.value.as_str())?,
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                            "analysis() requires a string argument".into(),
                        ))
                        .into());
                    }
                };
                Ok(IrExpr::Analysis(analysis_type))
            }
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
                if denominator.iter().all(|c| *c == 0.0) {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "laplace_nd denominator must be nonzero".into(),
                    ))
                    .into());
                }
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                })
            }
            "laplace_zp" => {
                validate_arg_range(&call.name, call.args.len(), 3, Some(3))?;
                let expr = self.convert(require_arg(0)?)?;
                let zeros = self.const_complex_pairs(require_arg(1)?)?;
                let poles = self.const_complex_pairs(require_arg(2)?)?;
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
                let zeros = self.const_complex_pairs(require_arg(1)?)?;
                let denominator = self.const_real_array(require_arg(2)?)?;
                let numerator = crate::laplace::roots_to_polynomial(&zeros).map_err(|e| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "laplace_zd zeros: {}",
                        e
                    )))
                })?;
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
                Ok(IrExpr::LaplaceND {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                })
            }
            // Z-domain filters: zi_xx(expr, num, den, T). Coefficient
            // arrays ascend in z^-1; zero/pole pair lists expand into
            // polynomials. The sample period must fold to a constant
            // (per-instance periods would reshape the filter state).
            "zi_nd" | "zi_zp" | "zi_zd" | "zi_np" => {
                validate_arg_range(&call.name, call.args.len(), 4, Some(4))?;
                let expr = self.convert(require_arg(0)?)?;
                let (numerator, denominator) = match call.name.as_str() {
                    "zi_nd" => (
                        self.const_real_array(require_arg(1)?)?,
                        self.const_real_array(require_arg(2)?)?,
                    ),
                    "zi_zp" => (
                        crate::zfilter::z_roots_to_coefficients(
                            &self.const_complex_pairs(require_arg(1)?)?,
                        )
                        .map_err(zi_root_error)?,
                        crate::zfilter::z_roots_to_coefficients(
                            &self.const_complex_pairs(require_arg(2)?)?,
                        )
                        .map_err(zi_root_error)?,
                    ),
                    "zi_zd" => (
                        crate::zfilter::z_roots_to_coefficients(
                            &self.const_complex_pairs(require_arg(1)?)?,
                        )
                        .map_err(zi_root_error)?,
                        self.const_real_array(require_arg(2)?)?,
                    ),
                    _ => (
                        self.const_real_array(require_arg(1)?)?,
                        crate::zfilter::z_roots_to_coefficients(
                            &self.const_complex_pairs(require_arg(2)?)?,
                        )
                        .map_err(zi_root_error)?,
                    ),
                };
                if denominator.first().copied().unwrap_or(0.0) == 0.0 {
                    return Err(
                        CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                            "{}: leading denominator coefficient must be nonzero",
                            call.name
                        )))
                        .into(),
                    );
                }
                let period = match autodiff_fold(self.convert(require_arg(3)?)?) {
                    IrExpr::Const(t) if t > 0.0 && t.is_finite() => t,
                    _ => {
                        return Err(CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(
                            format!(
                                "{}: the sample period must be a positive compile-time constant",
                                call.name
                            ),
                        ))
                        .into());
                    }
                };
                Ok(IrExpr::ZiFilter {
                    expr: Box::new(expr),
                    numerator,
                    denominator,
                    period,
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
            Expression::ArrayLiteral(arr) => arr.elements.iter().collect(),
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
            AnalogOperator::Ddx { expr, probe, .. } => {
                // ddx(expr, V(a,b)) - partial derivative w.r.t. voltage
                // For now, just return the expression - proper implementation
                // would compute symbolic derivative
                let inner = self.convert(expr)?;
                let _ = probe; // Would use for derivative target
                Ok(inner)
            }
            AnalogOperator::Absdelay { expr, delay, .. } => {
                // For static analysis, absdelay is just the expression
                // Real implementation would add delay handling
                let _ = delay;
                self.convert(expr)
            }
            AnalogOperator::Transition { expr, .. } => {
                // Transition is a smoothing operator - return expression for static
                self.convert(expr)
            }
            AnalogOperator::Slew { expr, .. } => {
                // Slew rate limiter - return expression for static
                self.convert(expr)
            }
            AnalogOperator::LastCrossing { .. } => {
                // Returns time of last crossing - not meaningful for DC
                Ok(IrExpr::Const(0.0))
            }
            AnalogOperator::Laplace { expr, .. } | AnalogOperator::Zi { expr, .. } => {
                // Laplace/Z-transform filters - simplified to just expression
                self.convert(expr)
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
