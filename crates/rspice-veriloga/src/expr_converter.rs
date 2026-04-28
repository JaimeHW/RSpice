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

/// Context for expression conversion
///
/// Maintains mappings from names to indices for efficient IR generation.
#[derive(Debug)]
pub struct ConversionContext {
    /// Map from terminal name to index
    terminal_map: HashMap<SmolStr, usize>,
    /// Map from parameter name to index
    param_map: HashMap<SmolStr, usize>,
    /// Map from variable name to index
    var_map: HashMap<SmolStr, usize>,
    /// Ground terminal index (if any)
    ground_index: Option<usize>,
}

impl ConversionContext {
    /// Create a new conversion context from an analyzed module
    pub fn from_module(module: &AnalyzedModule) -> Self {
        let mut terminal_map = HashMap::new();
        let mut param_map = HashMap::new();
        let mut var_map = HashMap::new();

        // Map port names to terminal indices
        for (idx, port) in module.ports.iter().enumerate() {
            terminal_map.insert(port.name.clone(), idx);
        }

        // Map parameter names to indices
        for (idx, param) in module.parameters.iter().enumerate() {
            param_map.insert(param.name.clone(), idx);
        }

        // Map variable names to indices
        for (idx, var) in module.variables.iter().enumerate() {
            var_map.insert(var.name.clone(), idx);
        }

        Self {
            terminal_map,
            param_map,
            var_map,
            ground_index: None,
        }
    }

    /// Set the ground terminal index
    pub fn set_ground(&mut self, idx: usize) {
        self.ground_index = Some(idx);
    }

    /// Get terminal index by name
    pub fn terminal_index(&self, name: &str) -> Option<usize> {
        self.terminal_map.get(name).copied()
    }

    /// Get parameter index by name
    pub fn param_index(&self, name: &str) -> Option<usize> {
        self.param_map.get(name).copied()
    }

    /// Get variable index by name
    pub fn var_index(&self, name: &str) -> Option<usize> {
        self.var_map.get(name).copied()
    }

    /// Ground terminal index (0 for implicit ground)
    pub fn ground(&self) -> usize {
        self.ground_index.unwrap_or(0)
    }

    /// Number of terminals
    pub fn num_terminals(&self) -> usize {
        self.terminal_map.len()
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
            Expression::ArrayAccess(_) => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature("Array access in expressions".into()),
            )
            .into()),
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
            "$temperature" => Ok(IrExpr::Temperature),
            "$abstime" => Ok(IrExpr::Time),
            "$realtime" => Ok(IrExpr::Time),
            "$simparam" => {
                // Return default for now - real implementations would query simulator
                Ok(IrExpr::Const(0.0))
            }
            "$param_given" => {
                // Check if parameter was explicitly set (always true for now)
                Ok(IrExpr::Const(1.0))
            }
            "$port_connected" => {
                // Check if port is connected (always true for now)
                Ok(IrExpr::Const(1.0))
            }
            "$mfactor" => {
                // Multiplicity factor (default 1.0)
                Ok(IrExpr::Const(1.0))
            }
            "$limit" => {
                // $limit(expr) or $limit(expr, step)
                // Bounds expression change per Newton iteration for convergence
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "$limit requires at least one argument".into(),
                    ))
                    .into());
                }
                let inner = self.convert(&func.args[0])?;
                let step = if func.args.len() > 1 {
                    Some(Box::new(self.convert(&func.args[1])?))
                } else {
                    None
                };
                Ok(IrExpr::Limit(Box::new(inner), step))
            }
            "$table_model" => {
                // $table_model(input, table_spec, ...)
                // Supports:
                // 1) Inline numeric pairs: $table_model(x, 0,0, 1,2, 2,4)
                // 2) Inline string table: $table_model(x, "0 0; 1 2; 2 4")
                // 3) Table file path:    $table_model(x, "table.dat")
                if func.args.len() < 2 {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "$table_model requires input expression and table data".into(),
                    ))
                    .into());
                }
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
                // cross(expr, direction, time_tol)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "cross requires at least one argument".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                // Direction: +1=rising, -1=falling, 0=both (default)
                let direction = if func.args.len() > 1 {
                    // Try to extract constant direction
                    if let crate::ast::Expression::Number(n) = &func.args[1] {
                        Some(n.value as i32)
                    } else {
                        Some(0)
                    }
                } else {
                    None
                };
                let time_tol = if func.args.len() > 2 {
                    Some(Box::new(self.convert(&func.args[2])?))
                } else {
                    None
                };
                Ok(IrExpr::Cross {
                    expr: Box::new(expr),
                    direction,
                    time_tol,
                })
            }
            "$white_noise" | "white_noise" => {
                // $white_noise(power, name)
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "$white_noise requires power argument".into(),
                    ))
                    .into());
                }
                let power = self.convert(&func.args[0])?;
                let name = if func.args.len() > 1 {
                    // Extract name if it's a string literal
                    if let crate::ast::Expression::StringLit(s) = &func.args[1] {
                        Some(s.value.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };
                Ok(IrExpr::WhiteNoise {
                    power: Box::new(power),
                    name,
                })
            }
            "$flicker_noise" | "flicker_noise" => {
                // $flicker_noise(power, exponent, name)
                if func.args.len() < 2 {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "$flicker_noise requires power and exponent arguments".into(),
                    ))
                    .into());
                }
                let power = self.convert(&func.args[0])?;
                let exponent = self.convert(&func.args[1])?;
                let name = if func.args.len() > 2 {
                    if let crate::ast::Expression::StringLit(s) = &func.args[2] {
                        Some(s.value.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };
                Ok(IrExpr::FlickerNoise {
                    power: Box::new(power),
                    exponent: Box::new(exponent),
                    name,
                })
            }
            "analysis" => {
                // analysis("dc"), analysis("ac"), analysis("tran")
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "analysis() requires analysis type argument".into(),
                    ))
                    .into());
                }
                // Extract analysis type string
                let analysis_type = if let crate::ast::Expression::StringLit(s) = &func.args[0] {
                    s.value.as_str()
                } else {
                    "dc" // Default to DC if not a string
                };
                Ok(IrExpr::Analysis(analysis_type.to_string()))
            }
            "above" => {
                // above(expr, threshold)
                if func.args.len() < 2 {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "above() requires expr and threshold arguments".into(),
                    ))
                    .into());
                }
                let expr = self.convert(&func.args[0])?;
                let threshold = self.convert(&func.args[1])?;
                let time_tol = if func.args.len() > 2 {
                    Some(Box::new(self.convert(&func.args[2])?))
                } else {
                    None
                };
                Ok(IrExpr::Above {
                    expr: Box::new(expr),
                    threshold: Box::new(threshold),
                    time_tol,
                })
            }
            "timer" => {
                // timer(start_time, period?) - period is optional
                if func.args.is_empty() {
                    return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                        "timer() requires at least start_time argument".into(),
                    ))
                    .into());
                }
                let start_time = self.convert(&func.args[0])?;
                let period = if func.args.len() > 1 {
                    Some(Box::new(self.convert(&func.args[1])?))
                } else {
                    None
                };
                Ok(IrExpr::Timer {
                    start_time: Box::new(start_time),
                    period,
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
            _ => {
                return Err(
                    CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(format!(
                        "Function: {}",
                        call.name
                    )))
                    .into(),
                );
            }
        };

        let args: Vec<IrExpr> = call
            .args
            .iter()
            .map(|a| self.convert(a))
            .collect::<CompileResult<Vec<_>>>()?;

        Ok(IrExpr::Call(ir_func, args))
    }

    /// Convert a branch access expression
    fn convert_branch_access(&self, access: &BranchAccess) -> CompileResult<IrExpr> {
        match access {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                let pos_idx = self.ctx.terminal_index(pos).ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "Unknown terminal: {}",
                        pos
                    )))
                })?;

                let neg_idx = neg
                    .as_ref()
                    .map(|n| {
                        self.ctx.terminal_index(n).ok_or_else(|| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "Unknown terminal: {}",
                                n
                            )))
                        })
                    })
                    .transpose()?
                    .unwrap_or(self.ctx.ground());

                match access.as_str() {
                    "V" => Ok(IrExpr::Voltage(pos_idx, neg_idx)),
                    "I" => Ok(IrExpr::Current(pos_idx, neg_idx)),
                    _ => Err(
                        CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(format!(
                            "Branch access: {}",
                            access
                        )))
                        .into(),
                    ),
                }
            }
            BranchAccess::Branch { access, name, .. } => {
                // Named branch access - would need branch table lookup
                Err(
                    CodeGenError::new(CodeGenErrorKind::UnsupportedFeature(format!(
                        "Named branch access: {}(<{}>)",
                        access, name
                    )))
                    .into(),
                )
            }
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
                let pos_idx = self.ctx.terminal_index(pos).ok_or_else(|| {
                    CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                        "Unknown terminal: {}",
                        pos
                    )))
                })?;

                let neg_idx = neg
                    .as_ref()
                    .map(|n| {
                        self.ctx.terminal_index(n).ok_or_else(|| {
                            CodeGenError::new(CodeGenErrorKind::InvalidExpression(format!(
                                "Unknown terminal: {}",
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
            BranchAccess::Branch { name, .. } => Err(CodeGenError::new(
                CodeGenErrorKind::UnsupportedFeature(format!("Named branch: {}", name)),
            )
            .into()),
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
