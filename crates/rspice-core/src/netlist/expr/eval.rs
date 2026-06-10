use super::*;

/// Evaluate an expression with the given context
pub fn evaluate(expr: &Expr, ctx: &ParamContext) -> Result<Value, ExprError> {
    match expr {
        Expr::Number(v) => Ok(*v),

        Expr::Param(name) => ctx
            .get(name)
            .ok_or_else(|| ExprError::UndefinedParam(name.clone())),

        Expr::BinOp { op, left, right } => {
            let l = evaluate(left, ctx)?;
            let r = evaluate(right, ctx)?;

            Ok(match op {
                // Arithmetic
                BinOpKind::Add => l + r,
                BinOpKind::Sub => l - r,
                BinOpKind::Mul => l * r,
                BinOpKind::Div => {
                    if r.abs() < 1e-300 {
                        return Err(ExprError::DivisionByZero);
                    }
                    l / r
                }
                BinOpKind::Pow => l.powf(r),
                // Comparison (return 1.0 for true, 0.0 for false)
                BinOpKind::Gt => {
                    if l > r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Lt => {
                    if l < r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Ge => {
                    if l >= r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Le => {
                    if l <= r {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Eq => {
                    if (l - r).abs() < 1e-12 {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Ne => {
                    if (l - r).abs() >= 1e-12 {
                        1.0
                    } else {
                        0.0
                    }
                }
                // Boolean (0.0 is false, anything else is true)
                BinOpKind::And => {
                    if l != 0.0 && r != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                BinOpKind::Or => {
                    if l != 0.0 || r != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            })
        }

        Expr::UnaryOp { op, operand } => {
            let v = evaluate(operand, ctx)?;
            Ok(match op {
                UnaryOpKind::Neg => -v,
                UnaryOpKind::Pos => v,
                UnaryOpKind::Not => {
                    if v == 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
            })
        }

        Expr::FnCall { name, args } => eval_function(name, args, ctx),
    }
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
        "ABS" => Ok(get_arg(0)?.abs()),
        "FLOOR" => Ok(get_arg(0)?.floor()),
        "CEIL" => Ok(get_arg(0)?.ceil()),
        "ROUND" => Ok(get_arg(0)?.round()),
        "MIN" => Ok(get_arg(0)?.min(get_arg(1)?)),
        "MAX" => Ok(get_arg(0)?.max(get_arg(1)?)),
        "POW" | "PWR" => Ok(get_arg(0)?.powf(get_arg(1)?)),
        "LIMIT" => match args.len() {
            // LIMIT(nom, avar): worst-case two-point draw, nom ± avar with a
            // random sign (ngspice/HSPICE .param semantics).
            2 => {
                let nom = get_arg(0)?;
                let avar = get_arg(1)?;
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
