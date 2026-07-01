use super::*;
use std::collections::HashMap;

const MAX_EVAL_FUNCTION_CALL_DEPTH: usize = 4096;

/// Evaluate an expression with the given context
pub fn evaluate(expr: &Expr, ctx: &ParamContext) -> Result<Value, ExprError> {
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

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, ExprError> {
        let mut frames = vec![EvalFrame::Eval(expr.clone(), EvalScope::global())];
        let mut values = Vec::<Value>::new();

        while let Some(frame) = frames.pop() {
            match frame {
                EvalFrame::Eval(expr, scope) => match expr {
                    Expr::Number(value) => values.push(value),
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
                        if cond != 0.0 { then_expr } else { else_expr },
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
        values: &mut Vec<Value>,
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
                FunctionArgBinding::Unset(_) => values.push(0.0),
            }
            return Ok(());
        }

        values.push(
            self.ctx
                .get(&name)
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

fn pop_value(values: &mut Vec<Value>) -> Result<Value, ExprError> {
    values.pop().ok_or_else(|| {
        ExprError::InvalidArgument("expression evaluation stack underflow".to_string())
    })
}

fn pop_args(values: &mut Vec<Value>, argc: usize) -> Result<Vec<Value>, ExprError> {
    if values.len() < argc {
        return Err(ExprError::InvalidArgument(
            "expression function argument stack underflow".to_string(),
        ));
    }
    Ok(values.split_off(values.len() - argc))
}

fn apply_unary(op: UnaryOpKind, value: Value) -> Value {
    match op {
        UnaryOpKind::Neg => -value,
        UnaryOpKind::Pos => value,
        UnaryOpKind::Not => {
            if value == 0.0 {
                1.0
            } else {
                0.0
            }
        }
    }
}

fn apply_binary(op: BinOpKind, left: Value, right: Value) -> Result<Value, ExprError> {
    Ok(match op {
        BinOpKind::Add => left + right,
        BinOpKind::Sub => left - right,
        BinOpKind::Mul => left * right,
        BinOpKind::Div => {
            if right.abs() < 1e-300 {
                return Err(ExprError::DivisionByZero);
            }
            left / right
        }
        BinOpKind::Pow => left.powf(right),
        BinOpKind::Gt => {
            if left > right {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Lt => {
            if left < right {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Ge => {
            if left >= right {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Le => {
            if left <= right {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Eq => {
            if (left - right).abs() < 1e-12 {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Ne => {
            if (left - right).abs() >= 1e-12 {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::And => {
            if left != 0.0 && right != 0.0 {
                1.0
            } else {
                0.0
            }
        }
        BinOpKind::Or => {
            if left != 0.0 || right != 0.0 {
                1.0
            } else {
                0.0
            }
        }
    })
}

fn eval_builtin_function_values(
    name: &str,
    args: &[Value],
    ctx: &ParamContext,
) -> Result<Value, ExprError> {
    let expr_args = args.iter().copied().map(Expr::Number).collect::<Vec<_>>();
    eval_function(name, &expr_args, ctx)
}

/// Evaluate a built-in or user-defined function
fn eval_function(name: &str, args: &[Expr], ctx: &ParamContext) -> Result<Value, ExprError> {
    // First check for user-defined functions
    if ctx.has_function(name) {
        let arg_values: Vec<Value> = args
            .iter()
            .map(|e| evaluate(e, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        return ctx.call_function(name, &arg_values);
    }

    let get_arg = |idx: usize| -> Result<Value, ExprError> {
        args.get(idx)
            .ok_or_else(|| ExprError::WrongArgCount(name.to_string()))
            .and_then(|e| evaluate(e, ctx))
    };

    match name {
        "SQRT" => Ok(get_arg(0)?.sqrt()),
        "SIN" => Ok(get_arg(0)?.sin()),
        "COS" => Ok(get_arg(0)?.cos()),
        "TAN" => Ok(get_arg(0)?.tan()),
        "ASIN" => Ok(get_arg(0)?.asin()),
        "ACOS" => Ok(get_arg(0)?.acos()),
        "ATAN" => Ok(get_arg(0)?.atan()),
        "ATAN2" => Ok(get_arg(0)?.atan2(get_arg(1)?)),
        "EXP" => Ok(get_arg(0)?.exp()),
        "LOG" | "LN" => Ok(get_arg(0)?.ln()),
        "LOG10" => Ok(get_arg(0)?.log10()),
        "ABS" | "M" => Ok(get_arg(0)?.abs()),
        "FLOOR" => Ok(get_arg(0)?.floor()),
        "CEIL" => Ok(get_arg(0)?.ceil()),
        "ROUND" => Ok(get_arg(0)?.round()),
        "MIN" => Ok(get_arg(0)?.min(get_arg(1)?)),
        "MAX" => Ok(get_arg(0)?.max(get_arg(1)?)),
        "POW" => Ok(get_arg(0)?.powf(get_arg(1)?)),
        "PWR" => Ok(get_arg(0)?.abs().powf(get_arg(1)?)),
        "PWRS" => {
            let base = get_arg(0)?;
            Ok(base.signum() * base.abs().powf(get_arg(1)?))
        }
        "LIMIT" => match args.len() {
            // LIMIT(nom, avar): worst-case two-point draw, nom ± avar with a
            // random sign (ngspice/HSPICE .param semantics).
            2 => {
                let nom = get_arg(0)?;
                let avar = get_arg(1)?;
                if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                    return Ok(nom);
                }
                let sign = if ctx.random().next_symmetric() >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                Ok(nom + avar * sign)
            }
            // LIMIT(x, min, max): clamp.
            3 => {
                let x = get_arg(0)?;
                let min = get_arg(1)?;
                let max = get_arg(2)?;
                Ok(x.clamp(min, max))
            }
            _ => Err(ExprError::WrongArgCount("LIMIT".to_string())),
        },
        // Statistical distribution functions (ngspice/HSPICE .param
        // semantics). Draws come from the context's seeded deterministic
        // stream, so identical seeds reproduce identical values.
        "GAUSS" | "AGAUSS" => {
            if args.len() != 3 {
                return Err(ExprError::WrongArgCount(name.to_string()));
            }
            let nom = get_arg(0)?;
            let var = get_arg(1)?;
            let sigma = get_arg(2)?;
            if sigma == 0.0 {
                return Err(ExprError::InvalidArgument(format!(
                    "{name}: sigma must be non-zero"
                )));
            }
            // gauss: variation is relative to nom; agauss: absolute.
            let deviation = if name == "GAUSS" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(nom);
            }
            Ok(nom + deviation / sigma * ctx.random().next_standard_normal())
        }
        "UNIF" | "AUNIF" => {
            if args.len() != 2 {
                return Err(ExprError::WrongArgCount(name.to_string()));
            }
            let nom = get_arg(0)?;
            let var = get_arg(1)?;
            // unif: variation is relative to nom; aunif: absolute.
            let deviation = if name == "UNIF" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(nom);
            }
            Ok(nom + deviation * ctx.random().next_symmetric())
        }
        "IF" => {
            // IF(cond, then, else)
            let cond = get_arg(0)?;
            if cond != 0.0 { get_arg(1) } else { get_arg(2) }
        }
        "URAMP" => {
            // URAMP(x) = x if x > 0, else 0
            let x = get_arg(0)?;
            Ok(if x > 0.0 { x } else { 0.0 })
        }
        "SGN" | "SIGN" => {
            let x = get_arg(0)?;
            Ok(if x > 0.0 {
                1.0
            } else if x < 0.0 {
                -1.0
            } else {
                0.0
            })
        }
        "TABLE" => {
            // TABLE(x, x1, y1, x2, y2, ...) - piecewise linear interpolation
            // Used extensively in vendor models for nonlinear behavior
            if args.len() < 3 {
                return Err(ExprError::WrongArgCount("TABLE".to_string()));
            }

            let x = get_arg(0)?;

            // Extract (x, y) pairs from remaining arguments
            let mut points: Vec<(Value, Value)> = Vec::new();
            let mut i = 1;
            while i + 1 < args.len() {
                let xi = get_arg(i)?;
                let yi = get_arg(i + 1)?;
                points.push((xi, yi));
                i += 2;
            }

            Ok(table_interpolate(x, &points))
        }
        "PWL" => {
            // PWL is an alias for TABLE (both do piecewise linear)
            if args.len() < 3 {
                return Err(ExprError::WrongArgCount("PWL".to_string()));
            }

            let x = get_arg(0)?;
            let mut points: Vec<(Value, Value)> = Vec::new();
            let mut i = 1;
            while i + 1 < args.len() {
                let xi = get_arg(i)?;
                let yi = get_arg(i + 1)?;
                points.push((xi, yi));
                i += 2;
            }

            Ok(table_interpolate(x, &points))
        }
        // Hyperbolic trigonometric functions
        "SINH" => Ok(get_arg(0)?.sinh()),
        "COSH" => Ok(get_arg(0)?.cosh()),
        "TANH" => Ok(get_arg(0)?.tanh()),
        "ASINH" => Ok(get_arg(0)?.asinh()),
        "ACOSH" => Ok(get_arg(0)?.acosh()),
        "ATANH" => Ok(get_arg(0)?.atanh()),
        "ARCTAN" => Ok(get_arg(0)?.atan()), // Alias for atan
        // Integer/rounding functions
        "INT" | "TRUNC" => Ok(get_arg(0)?.trunc()),
        "NINT" => {
            // Nearest integer (round half towards even - banker's rounding)
            let x = get_arg(0)?;
            Ok(x.round_ties_even())
        }
        "SQR" => {
            // Square of x
            let x = get_arg(0)?;
            Ok(x * x)
        }
        // Step functions for behavioral modeling
        "U" | "USTEP" => {
            // Unit step: 1 if x > 0, 0.5 if x == 0, 0 if x < 0
            let x = get_arg(0)?;
            Ok(if x > 0.0 {
                1.0
            } else if x == 0.0 {
                0.5
            } else {
                0.0
            })
        }
        "U2" => {
            // Smooth step: 0 for x<=0, x for 0<x<1, 1 for x>=1
            let x = get_arg(0)?;
            Ok(x.clamp(0.0, 1.0))
        }
        // Comparison functions (return 0 or 1)
        "EQ0" => {
            let x = get_arg(0)?;
            Ok(if x.abs() < 1e-12 { 1.0 } else { 0.0 })
        }
        "NE0" => {
            let x = get_arg(0)?;
            Ok(if x.abs() >= 1e-12 { 1.0 } else { 0.0 })
        }
        "GT0" => {
            let x = get_arg(0)?;
            Ok(if x > 0.0 { 1.0 } else { 0.0 })
        }
        "LT0" => {
            let x = get_arg(0)?;
            Ok(if x < 0.0 { 1.0 } else { 0.0 })
        }
        "GE0" => {
            let x = get_arg(0)?;
            Ok(if x >= 0.0 { 1.0 } else { 0.0 })
        }
        "LE0" => {
            let x = get_arg(0)?;
            Ok(if x <= 0.0 { 1.0 } else { 0.0 })
        }
        // Pseudo-random (constant for DC, deterministic)
        "RAND" | "RANDOM" => {
            // Return a pseudo-random value (for testing, use a hash)
            Ok(0.5) // Constant for DC evaluation
        }
        _ => Err(ExprError::UnknownFunction(name.to_string())),
    }
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
