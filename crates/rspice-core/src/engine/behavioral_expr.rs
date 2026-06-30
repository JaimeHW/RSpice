//! Behavioral expression preprocessing for circuit build.
//!
//! This module expands user-defined `.FUNC` calls into plain expressions before
//! strict behavioral parsing/compilation.

use crate::netlist::ParamContext;
use crate::netlist::expr::{
    BinOpKind, Expr as NetExpr, UnaryOpKind, parse_expression as parse_net_expr,
};
use std::collections::HashMap;

const MAX_FUNCTION_EXPANSION_DEPTH: usize = 64;

/// Prepare a behavioral source expression for strict compilation.
///
/// - Expands user-defined `.FUNC` calls recursively.
/// - Substitutes globally-defined `.PARAM` identifiers when available.
/// - Leaves non-user function calls (e.g., `V(...)`, `SIN(...)`) untouched.
pub fn prepare_behavioral_expression(
    expression: &str,
    params: &ParamContext,
) -> Result<String, String> {
    // Keep legacy behavior when the permissive parser cannot parse a
    // non-standard expression; strict parser will still validate later.
    let parsed = match parse_net_expr(expression) {
        Ok(expr) => expr,
        Err(_) => return Ok(expression.to_string()),
    };

    let mut expander = FunctionExpander::new(params);
    let expanded = expander.expand_expr(&parsed, 0)?;
    Ok(serialize_expr(&expanded))
}

struct FunctionExpander<'a> {
    params: &'a ParamContext,
    body_cache: HashMap<String, NetExpr>,
    call_stack: Vec<String>,
}

impl<'a> FunctionExpander<'a> {
    fn new(params: &'a ParamContext) -> Self {
        Self {
            params,
            body_cache: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    fn expand_expr(&mut self, expr: &NetExpr, depth: usize) -> Result<NetExpr, String> {
        if depth > MAX_FUNCTION_EXPANSION_DEPTH {
            return Err(format!(
                "Behavioral expression exceeded function expansion depth (>{})",
                MAX_FUNCTION_EXPANSION_DEPTH
            ));
        }

        match expr {
            NetExpr::Number(v) => Ok(NetExpr::Number(*v)),
            NetExpr::Param(name) => {
                if is_behavioral_runtime_symbol(name) {
                    Ok(NetExpr::Param(name.clone()))
                } else if let Some(v) = self.params.get(name) {
                    Ok(NetExpr::Number(v))
                } else {
                    Ok(NetExpr::Param(name.clone()))
                }
            }
            NetExpr::UnaryOp { op, operand } => Ok(NetExpr::UnaryOp {
                op: *op,
                operand: Box::new(self.expand_expr(operand, depth + 1)?),
            }),
            NetExpr::BinOp { op, left, right } => Ok(NetExpr::BinOp {
                op: *op,
                left: Box::new(self.expand_expr(left, depth + 1)?),
                right: Box::new(self.expand_expr(right, depth + 1)?),
            }),
            NetExpr::FnCall { name, args } => {
                let expanded_args = args
                    .iter()
                    .map(|arg| self.expand_expr(arg, depth + 1))
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(func_def) = self.params.get_function(name) {
                    if expanded_args.len() != func_def.args.len() {
                        return Err(format!(
                            "Function '{}' expects {} args but got {}",
                            name,
                            func_def.args.len(),
                            expanded_args.len()
                        ));
                    }

                    let func_name = func_def.name.clone();
                    if self.call_stack.iter().any(|s| s == &func_name) {
                        return Err(format!(
                            "Detected recursive .FUNC expansion for '{}'",
                            func_name
                        ));
                    }

                    let body_ast = if let Some(cached) = self.body_cache.get(&func_name) {
                        cached.clone()
                    } else {
                        let parsed_body = parse_net_expr(&func_def.body).map_err(|e| {
                            format!(
                                "Failed to parse .FUNC {} body '{}': {}",
                                func_name, func_def.body, e
                            )
                        })?;
                        self.body_cache
                            .insert(func_name.clone(), parsed_body.clone());
                        parsed_body
                    };

                    let mut arg_bindings: HashMap<String, NetExpr> =
                        HashMap::with_capacity(func_def.args.len());
                    for (arg_name, arg_value) in func_def.args.iter().zip(expanded_args.iter()) {
                        arg_bindings.insert(arg_name.to_ascii_uppercase(), arg_value.clone());
                    }

                    self.call_stack.push(func_name.clone());
                    let substituted = substitute_function_args(&body_ast, &arg_bindings);
                    let result = self.expand_expr(&substituted, depth + 1);
                    self.call_stack.pop();
                    result
                } else {
                    Ok(NetExpr::FnCall {
                        name: name.clone(),
                        args: expanded_args,
                    })
                }
            }
        }
    }
}

fn is_behavioral_runtime_symbol(name: &str) -> bool {
    name.eq_ignore_ascii_case("TEMPER")
}

fn substitute_function_args(expr: &NetExpr, args: &HashMap<String, NetExpr>) -> NetExpr {
    match expr {
        NetExpr::Number(v) => NetExpr::Number(*v),
        NetExpr::Param(name) => args
            .get(&name.to_ascii_uppercase())
            .cloned()
            .unwrap_or_else(|| NetExpr::Param(name.clone())),
        NetExpr::UnaryOp { op, operand } => NetExpr::UnaryOp {
            op: *op,
            operand: Box::new(substitute_function_args(operand, args)),
        },
        NetExpr::BinOp { op, left, right } => NetExpr::BinOp {
            op: *op,
            left: Box::new(substitute_function_args(left, args)),
            right: Box::new(substitute_function_args(right, args)),
        },
        NetExpr::FnCall {
            name,
            args: fn_args,
        } => NetExpr::FnCall {
            name: name.clone(),
            args: if name.eq_ignore_ascii_case("V") || name.eq_ignore_ascii_case("I") {
                // Preserve symbolic probe references in V(...) / I(...). ngspice
                // resolves these against circuit node/branch namespaces, even
                // when formal .FUNC argument names collide with probe symbols.
                fn_args.clone()
            } else {
                fn_args
                    .iter()
                    .map(|a| substitute_function_args(a, args))
                    .collect()
            },
        },
    }
}

fn serialize_expr(expr: &NetExpr) -> String {
    match expr {
        NetExpr::Number(v) => {
            if *v < 0.0 {
                format!("({})", v)
            } else {
                format!("{}", v)
            }
        }
        NetExpr::Param(name) => name.clone(),
        NetExpr::UnaryOp { op, operand } => match op {
            UnaryOpKind::Neg => format!("(-{})", serialize_expr(operand)),
            UnaryOpKind::Pos => format!("({})", serialize_expr(operand)),
            UnaryOpKind::Not => format!("(!{})", serialize_expr(operand)),
        },
        NetExpr::BinOp { op, left, right } => {
            let symbol = match op {
                BinOpKind::Add => "+",
                BinOpKind::Sub => "-",
                BinOpKind::Mul => "*",
                BinOpKind::Div => "/",
                BinOpKind::Pow => "^",
                BinOpKind::Gt => ">",
                BinOpKind::Lt => "<",
                BinOpKind::Ge => ">=",
                BinOpKind::Le => "<=",
                BinOpKind::Eq => "==",
                BinOpKind::Ne => "!=",
                BinOpKind::And => "&&",
                BinOpKind::Or => "||",
            };
            format!(
                "({}{}{})",
                serialize_expr(left),
                symbol,
                serialize_expr(right)
            )
        }
        NetExpr::FnCall { name, args } => {
            let args_joined = args
                .iter()
                .map(serialize_expr)
                .collect::<Vec<_>>()
                .join(",");
            format!("{}({})", name.to_ascii_uppercase(), args_joined)
        }
    }
}
