use super::*;
use std::collections::HashMap;

const MAX_EVAL_FUNCTION_CALL_DEPTH: usize = 4096;

/// Evaluate an expression with the given context
pub fn evaluate(expr: &Expr, ctx: &ParamContext) -> Result<Value, ExprError> {
    evaluate_complex(expr, ctx).map(ComplexValue::real_projection)
}

/// Evaluate an expression with the given context, preserving complex values.
pub fn evaluate_complex(expr: &Expr, ctx: &ParamContext) -> Result<ComplexValue, ExprError> {
    ExpressionEvaluator::new(ctx).evaluate(expr)
}

#[derive(Debug, Clone)]
struct EvalScope {
    function_name: Option<String>,
}

impl EvalScope {
    fn global() -> Self {
        Self {
            function_name: None,
        }
    }

    fn function(function_name: String) -> Self {
        Self {
            function_name: Some(function_name),
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionArgExpr {
    id: u64,
    expr: Expr,
    scope: EvalScope,
}

#[derive(Debug, Clone)]
enum FunctionArgBinding {
    Expr(FunctionArgExpr),
    Unset(u64),
}

#[derive(Debug, Default)]
struct FunctionFrame {
    args: HashMap<String, FunctionArgBinding>,
}

struct ExpressionEvaluator<'a> {
    ctx: &'a ParamContext,
    function_frames: HashMap<String, FunctionFrame>,
    body_cache: HashMap<String, Expr>,
    call_depth: usize,
    next_binding_id: u64,
}

enum EvalFrame {
    Eval(Expr, EvalScope),
    ApplyUnary(UnaryOpKind),
    ApplyBinary(BinOpKind),
    ApplyIf {
        then_expr: Expr,
        else_expr: Expr,
        scope: EvalScope,
    },
    ApplyBuiltin {
        name: String,
        argc: usize,
    },
    ApplyFunctionArg {
        function_name: String,
        arg_name: String,
        binding_id: u64,
        param_name: String,
        scope: EvalScope,
    },
    UnsetUserFunction {
        function_name: String,
        arg_names: Vec<String>,
    },
}

impl<'a> ExpressionEvaluator<'a> {
    fn new(ctx: &'a ParamContext) -> Self {
        Self {
            ctx,
            function_frames: HashMap::new(),
            body_cache: HashMap::new(),
            call_depth: 0,
            next_binding_id: 1,
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<ComplexValue, ExprError> {
        let mut frames = vec![EvalFrame::Eval(expr.clone(), EvalScope::global())];
        let mut values = Vec::<ComplexValue>::new();

        while let Some(frame) = frames.pop() {
            match frame {
                EvalFrame::Eval(expr, scope) => match expr {
                    Expr::Number(value) => values.push(ComplexValue::real(value)),
                    Expr::ComplexNumber(value) => values.push(value),
                    Expr::Param(name) => {
                        self.push_param_eval(&mut frames, &mut values, name, scope)?
                    }
                    Expr::UnaryOp { op, operand } => {
                        frames.push(EvalFrame::ApplyUnary(op));
                        frames.push(EvalFrame::Eval(*operand, scope));
                    }
                    Expr::BinOp { op, left, right } => {
                        frames.push(EvalFrame::ApplyBinary(op));
                        frames.push(EvalFrame::Eval(*right, scope.clone()));
                        frames.push(EvalFrame::Eval(*left, scope));
                    }
                    Expr::FnCall { name, args } => {
                        self.push_function_eval(&mut frames, name, args, scope)?;
                    }
                },
                EvalFrame::ApplyUnary(op) => {
                    let value = pop_value(&mut values)?;
                    values.push(apply_unary(op, value));
                }
                EvalFrame::ApplyBinary(op) => {
                    let right = pop_value(&mut values)?;
                    let left = pop_value(&mut values)?;
                    values.push(apply_binary(op, left, right)?);
                }
                EvalFrame::ApplyIf {
                    then_expr,
                    else_expr,
                    scope,
                } => {
                    let cond = pop_value(&mut values)?;
                    frames.push(EvalFrame::Eval(
                        if complex_truth(cond) {
                            then_expr
                        } else {
                            else_expr
                        },
                        scope,
                    ));
                }
                EvalFrame::ApplyBuiltin { name, argc } => {
                    let args = pop_args(&mut values, argc)?;
                    values.push(eval_builtin_function_values(&name, &args, self.ctx)?);
                }
                EvalFrame::ApplyFunctionArg {
                    function_name,
                    arg_name,
                    binding_id,
                    param_name,
                    scope,
                } => {
                    let value = pop_value(&mut values)?;
                    if self
                        .current_function_arg_binding_id(&function_name, &arg_name)
                        .is_some_and(|current_id| current_id == binding_id)
                    {
                        values.push(value);
                    } else {
                        frames.push(EvalFrame::Eval(Expr::Param(param_name), scope));
                    }
                }
                EvalFrame::UnsetUserFunction {
                    function_name,
                    arg_names,
                } => {
                    self.call_depth = self.call_depth.saturating_sub(1);
                    self.unset_function_args(&function_name, &arg_names);
                }
            }
        }

        if values.len() == 1 {
            Ok(values.pop().expect("length checked"))
        } else {
            Err(ExprError::InvalidArgument(format!(
                "expression evaluation produced {} values",
                values.len()
            )))
        }
    }

    fn push_param_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        values: &mut Vec<ComplexValue>,
        name: String,
        scope: EvalScope,
    ) -> Result<(), ExprError> {
        if let Some((function_name, arg_name, binding)) = self.function_arg_binding(&name, &scope) {
            match binding {
                FunctionArgBinding::Expr(arg) => {
                    frames.push(EvalFrame::ApplyFunctionArg {
                        function_name,
                        arg_name,
                        binding_id: arg.id,
                        param_name: name,
                        scope,
                    });
                    frames.push(EvalFrame::Eval(arg.expr, arg.scope));
                }
                FunctionArgBinding::Unset(_) => values.push(ComplexValue::zero()),
            }
            return Ok(());
        }

        values.push(
            self.ctx
                .get_complex(&name)
                .ok_or_else(|| ExprError::UndefinedParam(name.to_string()))?,
        );
        Ok(())
    }

    fn push_function_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        name: String,
        args: Vec<Expr>,
        scope: EvalScope,
    ) -> Result<(), ExprError> {
        let upper = name.to_ascii_uppercase();
        if self.ctx.has_function(&upper) {
            self.push_user_function_eval(frames, &upper, &args, &scope)?;
        } else if upper == "IF" {
            if args.len() != 3 {
                return Err(ExprError::WrongArgCount(upper));
            }
            let mut args = args.into_iter();
            let cond = args.next().expect("IF arg count checked");
            let then_expr = args.next().expect("IF arg count checked");
            let else_expr = args.next().expect("IF arg count checked");
            frames.push(EvalFrame::ApplyIf {
                then_expr,
                else_expr,
                scope: scope.clone(),
            });
            frames.push(EvalFrame::Eval(cond, scope));
        } else {
            frames.push(EvalFrame::ApplyBuiltin {
                name: upper,
                argc: args.len(),
            });
            for arg in args.into_iter().rev() {
                frames.push(EvalFrame::Eval(arg, scope.clone()));
            }
        }
        Ok(())
    }

    fn push_user_function_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        name: &str,
        args: &[Expr],
        caller_scope: &EvalScope,
    ) -> Result<(), ExprError> {
        let func = self
            .ctx
            .get_function(name)
            .cloned()
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;
        if args.len() != func.args.len() {
            return Err(ExprError::WrongArgCount(name.to_string()));
        }
        if self.call_depth >= MAX_EVAL_FUNCTION_CALL_DEPTH {
            return Err(ExprError::InvalidArgument(format!(
                "function nesting exceeds maximum depth of {} while calling {}",
                MAX_EVAL_FUNCTION_CALL_DEPTH, func.name
            )));
        }

        let body = if let Some(body) = self.body_cache.get(&func.name) {
            body.clone()
        } else {
            let body = parse_expression(&func.body)?;
            self.body_cache.insert(func.name.clone(), body.clone());
            body
        };

        let bindings = func
            .args
            .iter()
            .zip(args.iter())
            .map(|(arg_name, arg_expr)| {
                let binding = FunctionArgBinding::Expr(FunctionArgExpr {
                    id: self.allocate_binding_id(),
                    expr: arg_expr.clone(),
                    scope: caller_scope.clone(),
                });
                (arg_name.clone(), binding)
            })
            .collect::<Vec<_>>();

        {
            let frame = self.function_frames.entry(func.name.clone()).or_default();
            for (arg_name, binding) in bindings {
                frame.args.insert(arg_name, binding);
            }
        }

        self.call_depth += 1;
        frames.push(EvalFrame::UnsetUserFunction {
            function_name: func.name.clone(),
            arg_names: func.args.clone(),
        });
        frames.push(EvalFrame::Eval(body, EvalScope::function(func.name)));
        Ok(())
    }

    fn function_arg_binding(
        &self,
        name: &str,
        scope: &EvalScope,
    ) -> Option<(String, String, FunctionArgBinding)> {
        let function_name = scope.function_name.as_ref()?;
        let func = self.ctx.get_function(function_name)?;
        let arg_name = name.to_ascii_uppercase();
        if !func.args.iter().any(|arg| arg == &arg_name) {
            return None;
        }
        let binding = self
            .function_frames
            .get(function_name)
            .and_then(|frame| frame.args.get(&arg_name))
            .cloned()
            .unwrap_or(FunctionArgBinding::Unset(0));
        Some((function_name.clone(), arg_name, binding))
    }

    fn unset_function_args(&mut self, function_name: &str, arg_names: &[String]) {
        for arg_name in arg_names {
            let binding_id = self.allocate_binding_id();
            if let Some(frame) = self.function_frames.get_mut(function_name) {
                frame
                    .args
                    .insert(arg_name.clone(), FunctionArgBinding::Unset(binding_id));
            }
        }
    }

    fn current_function_arg_binding_id(&self, function_name: &str, arg_name: &str) -> Option<u64> {
        match self
            .function_frames
            .get(function_name)?
            .args
            .get(arg_name)?
        {
            FunctionArgBinding::Expr(arg) => Some(arg.id),
            FunctionArgBinding::Unset(id) => Some(*id),
        }
    }

    fn allocate_binding_id(&mut self) -> u64 {
        let id = self.next_binding_id;
        self.next_binding_id = self.next_binding_id.saturating_add(1);
        id
    }
}

fn pop_value(values: &mut Vec<ComplexValue>) -> Result<ComplexValue, ExprError> {
    values.pop().ok_or_else(|| {
        ExprError::InvalidArgument("expression evaluation stack underflow".to_string())
    })
}

fn pop_args(values: &mut Vec<ComplexValue>, argc: usize) -> Result<Vec<ComplexValue>, ExprError> {
    if values.len() < argc {
        return Err(ExprError::InvalidArgument(
            "expression function argument stack underflow".to_string(),
        ));
    }
    Ok(values.split_off(values.len() - argc))
}

fn apply_unary(op: UnaryOpKind, value: ComplexValue) -> ComplexValue {
    match op {
        UnaryOpKind::Neg => ComplexValue::new(-value.re, -value.im),
        UnaryOpKind::Pos => value,
        UnaryOpKind::Not => bool_value(!complex_truth(value)),
    }
}

fn apply_binary(
    op: BinOpKind,
    left: ComplexValue,
    right: ComplexValue,
) -> Result<ComplexValue, ExprError> {
    Ok(match op {
        BinOpKind::Add => complex_add(left, right),
        BinOpKind::Sub => complex_sub(left, right),
        BinOpKind::Mul => complex_mul(left, right),
        BinOpKind::Div => complex_div(left, right)?,
        BinOpKind::Pow => complex_pow(left, right),
        BinOpKind::Gt => bool_value(left.re > right.re),
        BinOpKind::Lt => bool_value(left.re < right.re),
        BinOpKind::Ge => bool_value(left.re >= right.re),
        BinOpKind::Le => bool_value(left.re <= right.re),
        BinOpKind::Eq => {
            bool_value((left.re - right.re).abs() < 1e-12 && (left.im - right.im).abs() < 1e-12)
        }
        BinOpKind::Ne => {
            bool_value((left.re - right.re).abs() >= 1e-12 || (left.im - right.im).abs() >= 1e-12)
        }
        BinOpKind::And => bool_value(complex_truth(left) && complex_truth(right)),
        BinOpKind::Or => bool_value(complex_truth(left) || complex_truth(right)),
    })
}

#[inline]
fn bool_value(value: bool) -> ComplexValue {
    ComplexValue::real(if value { 1.0 } else { 0.0 })
}

#[inline]
fn complex_truth(value: ComplexValue) -> bool {
    value.re != 0.0 || value.im != 0.0
}

#[inline]
fn complex_add(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(left.re + right.re, left.im + right.im)
}

#[inline]
fn complex_sub(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(left.re - right.re, left.im - right.im)
}

#[inline]
fn complex_mul(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        left.re.mul_add(right.re, -(left.im * right.im)),
        left.re.mul_add(right.im, left.im * right.re),
    )
}

#[inline]
fn complex_div(left: ComplexValue, right: ComplexValue) -> Result<ComplexValue, ExprError> {
    let denom = right.re.mul_add(right.re, right.im * right.im);
    if denom < 1e-300 {
        return Err(ExprError::DivisionByZero);
    }
    Ok(ComplexValue::new(
        (left.re * right.re + left.im * right.im) / denom,
        (left.im * right.re - left.re * right.im) / denom,
    ))
}

#[inline]
fn complex_arg(value: ComplexValue) -> Value {
    if value.im == 0.0 && value.re < 0.0 {
        -std::f64::consts::PI
    } else {
        value.im.atan2(value.re)
    }
}

#[inline]
fn complex_ln(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(value.magnitude().ln(), complex_arg(value))
}

#[inline]
fn complex_exp(value: ComplexValue) -> ComplexValue {
    let scale = value.re.exp();
    ComplexValue::new(scale * value.im.cos(), scale * value.im.sin())
}

#[inline]
fn complex_pow(base: ComplexValue, exponent: ComplexValue) -> ComplexValue {
    if base.is_real() && exponent.is_real() {
        let real = base.re.powf(exponent.re);
        if real.is_finite() || base.re >= 0.0 {
            return ComplexValue::real(real);
        }
    }
    complex_exp(complex_mul(exponent, complex_ln(base)))
}

#[inline]
fn complex_sqrt(value: ComplexValue) -> ComplexValue {
    if value.im == 0.0 {
        return if value.re >= 0.0 {
            ComplexValue::real(value.re.sqrt())
        } else {
            ComplexValue::new(0.0, -(-value.re).sqrt())
        };
    }
    complex_pow(value, ComplexValue::real(0.5))
}

#[inline]
fn complex_sin(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        value.re.sin() * value.im.cosh(),
        value.re.cos() * value.im.sinh(),
    )
}

#[inline]
fn complex_cos(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        value.re.cos() * value.im.cosh(),
        -(value.re.sin() * value.im.sinh()),
    )
}

#[inline]
fn complex_tan(value: ComplexValue) -> Result<ComplexValue, ExprError> {
    complex_div(complex_sin(value), complex_cos(value))
}

fn eval_builtin_function_values(
    name: &str,
    args: &[ComplexValue],
    ctx: &ParamContext,
) -> Result<ComplexValue, ExprError> {
    eval_complex_builtin_function(name, args, ctx)
}

fn eval_complex_builtin_function(
    name: &str,
    args: &[ComplexValue],
    ctx: &ParamContext,
) -> Result<ComplexValue, ExprError> {
    match name {
        "SQRT" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_sqrt(args[0]))
        }
        "SIN" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_sin(args[0]))
        }
        "COS" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_cos(args[0]))
        }
        "TAN" => {
            require_arg_count(name, args, 1)?;
            complex_tan(args[0])
        }
        "EXP" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_exp(args[0]))
        }
        "LOG" | "LN" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_ln(args[0]))
        }
        "LOG10" => {
            require_arg_count(name, args, 1)?;
            let value = complex_ln(args[0]);
            Ok(ComplexValue::new(
                value.re / std::f64::consts::LN_10,
                value.im / std::f64::consts::LN_10,
            ))
        }
        "ABS" | "M" | "MAG" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].magnitude()))
        }
        "R" | "RE" | "REAL" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].re))
        }
        "IMG" | "IMAG" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].im))
        }
        "PH" | "PHASE" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(complex_arg(args[0]).to_degrees()))
        }
        "DB" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(20.0 * args[0].magnitude().log10()))
        }
        "ASIN" => real_unary(name, args, |x| x.asin()),
        "ACOS" => real_unary(name, args, |x| x.acos()),
        "ATAN" => real_unary(name, args, |x| x.atan()),
        "ATAN2" => real_binary(name, args, |left, right| left.atan2(right)),
        "FLOOR" => real_unary(name, args, |x| x.floor()),
        "CEIL" => real_unary(name, args, |x| x.ceil()),
        "ROUND" => real_unary(name, args, |x| x.round()),
        "MIN" => real_binary(name, args, |left, right| left.min(right)),
        "MAX" => real_binary(name, args, |left, right| left.max(right)),
        "POW" => {
            require_arg_count(name, args, 2)?;
            Ok(complex_pow(args[0], args[1]))
        }
        "PWR" => Ok(ComplexValue::real(
            checked_real_arg(name, args, 0)?
                .abs()
                .powf(checked_real_arg(name, args, 1)?),
        )),
        "PWRS" => {
            let base = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(
                base.signum() * base.abs().powf(checked_real_arg(name, args, 1)?),
            ))
        }
        "LIMIT" => match args.len() {
            2 => {
                let nom = checked_real_arg(name, args, 0)?;
                let avar = checked_real_arg(name, args, 1)?;
                if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                    return Ok(ComplexValue::real(nom));
                }
                let sign = if ctx.random().next_symmetric() >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                Ok(ComplexValue::real(nom + avar * sign))
            }
            3 => {
                let x = checked_real_arg(name, args, 0)?;
                let min = checked_real_arg(name, args, 1)?;
                let max = checked_real_arg(name, args, 2)?;
                Ok(ComplexValue::real(x.clamp(min, max)))
            }
            _ => Err(ExprError::WrongArgCount("LIMIT".to_string())),
        },
        "GAUSS" | "AGAUSS" => {
            require_arg_count(name, args, 3)?;
            let nom = checked_real_arg(name, args, 0)?;
            let var = checked_real_arg(name, args, 1)?;
            let sigma = checked_real_arg(name, args, 2)?;
            if sigma == 0.0 {
                return Err(ExprError::InvalidArgument(format!(
                    "{name}: sigma must be non-zero"
                )));
            }
            let deviation = if name == "GAUSS" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(ComplexValue::real(nom));
            }
            Ok(ComplexValue::real(
                nom + deviation / sigma * ctx.random().next_standard_normal(),
            ))
        }
        "UNIF" | "AUNIF" => {
            require_arg_count(name, args, 2)?;
            let nom = checked_real_arg(name, args, 0)?;
            let var = checked_real_arg(name, args, 1)?;
            let deviation = if name == "UNIF" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(ComplexValue::real(nom));
            }
            Ok(ComplexValue::real(
                nom + deviation * ctx.random().next_symmetric(),
            ))
        }
        "IF" => {
            require_arg_count(name, args, 3)?;
            if complex_truth(args[0]) {
                Ok(args[1])
            } else {
                Ok(args[2])
            }
        }
        "URAMP" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(if x > 0.0 { x } else { 0.0 }))
        }
        "SGN" | "SIGN" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(if x > 0.0 {
                1.0
            } else if x < 0.0 {
                -1.0
            } else {
                0.0
            }))
        }
        "TABLE" | "PWL" => eval_real_table_function(name, args),
        "SINH" => real_unary(name, args, |x| x.sinh()),
        "COSH" => real_unary(name, args, |x| x.cosh()),
        "TANH" => real_unary(name, args, |x| x.tanh()),
        "ASINH" => real_unary(name, args, |x| x.asinh()),
        "ACOSH" => real_unary(name, args, |x| x.acosh()),
        "ATANH" => real_unary(name, args, |x| x.atanh()),
        "ARCTAN" => real_unary(name, args, |x| x.atan()),
        "INT" | "TRUNC" => real_unary(name, args, |x| x.trunc()),
        "NINT" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(x.round_ties_even()))
        }
        "SQR" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_mul(args[0], args[0]))
        }
        "U" | "USTEP" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(if x > 0.0 {
                1.0
            } else if x == 0.0 {
                0.5
            } else {
                0.0
            }))
        }
        "U2" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(x.clamp(0.0, 1.0)))
        }
        "EQ0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x.abs() < 1e-12))
        }
        "NE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x.abs() >= 1e-12))
        }
        "GT0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x > 0.0))
        }
        "LT0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x < 0.0))
        }
        "GE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x >= 0.0))
        }
        "LE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x <= 0.0))
        }
        "RAND" | "RANDOM" => Ok(ComplexValue::real(0.5)),
        _ => Err(ExprError::UnknownFunction(name.to_string())),
    }
}

fn require_arg_count(name: &str, args: &[ComplexValue], expected: usize) -> Result<(), ExprError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(ExprError::WrongArgCount(name.to_string()))
    }
}

fn checked_arg(name: &str, args: &[ComplexValue], index: usize) -> Result<ComplexValue, ExprError> {
    args.get(index)
        .copied()
        .ok_or_else(|| ExprError::WrongArgCount(name.to_string()))
}

fn checked_real_arg(name: &str, args: &[ComplexValue], index: usize) -> Result<Value, ExprError> {
    let value = checked_arg(name, args, index)?;
    if value.is_real() {
        Ok(value.re)
    } else {
        Err(ExprError::InvalidArgument(format!(
            "{name}: complex argument {} is not valid for a real-valued function",
            index + 1
        )))
    }
}

fn real_unary(
    name: &str,
    args: &[ComplexValue],
    op: impl FnOnce(Value) -> Value,
) -> Result<ComplexValue, ExprError> {
    require_arg_count(name, args, 1)?;
    Ok(ComplexValue::real(op(checked_real_arg(name, args, 0)?)))
}

fn real_binary(
    name: &str,
    args: &[ComplexValue],
    op: impl FnOnce(Value, Value) -> Value,
) -> Result<ComplexValue, ExprError> {
    require_arg_count(name, args, 2)?;
    Ok(ComplexValue::real(op(
        checked_real_arg(name, args, 0)?,
        checked_real_arg(name, args, 1)?,
    )))
}

fn eval_real_table_function(name: &str, args: &[ComplexValue]) -> Result<ComplexValue, ExprError> {
    if args.len() < 3 {
        return Err(ExprError::WrongArgCount(name.to_string()));
    }
    let x = checked_real_arg(name, args, 0)?;
    let mut points = Vec::<(Value, Value)>::new();
    let mut i = 1;
    while i + 1 < args.len() {
        points.push((
            checked_real_arg(name, args, i)?,
            checked_real_arg(name, args, i + 1)?,
        ));
        i += 2;
    }
    Ok(ComplexValue::real(table_interpolate(x, &points)))
}

/// Piecewise linear interpolation for TABLE function
///
/// Linearly interpolates between defined points.
/// For x outside the defined range, extrapolates from the nearest segment.
#[inline]
fn table_interpolate(x: Value, points: &[(Value, Value)]) -> Value {
    if points.is_empty() {
        return 0.0;
    }

    if points.len() == 1 {
        return points[0].1;
    }

    // Sort points by x (should already be sorted in valid input)
    // For performance, we assume input is sorted

    // Handle x below first point - extrapolate
    if x <= points[0].0 {
        if points.len() >= 2 {
            let (x0, y0) = points[0];
            let (x1, y1) = points[1];
            if (x1 - x0).abs() > 1e-18 {
                let slope = (y1 - y0) / (x1 - x0);
                return y0 + slope * (x - x0);
            }
        }
        return points[0].1;
    }

    // Handle x above last point - extrapolate
    let last = points.len() - 1;
    if x >= points[last].0 {
        if points.len() >= 2 {
            let (x0, y0) = points[last - 1];
            let (x1, y1) = points[last];
            if (x1 - x0).abs() > 1e-18 {
                let slope = (y1 - y0) / (x1 - x0);
                return y1 + slope * (x - x1);
            }
        }
        return points[last].1;
    }

    // Find bracketing points and interpolate
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];

        if x >= x0 && x <= x1 {
            if (x1 - x0).abs() < 1e-18 {
                return y0;
            }
            let t = (x - x0) / (x1 - x0);
            return y0 + t * (y1 - y0);
        }
    }

    // Fallback (shouldn't reach here)
    points[last].1
}
