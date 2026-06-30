//! Behavioral expression preprocessing shared by flattening and circuit build.
//!
//! User-defined `.FUNC` calls and scalar `.PARAM` references are expanded here,
//! while circuit-state probes such as `V(...)` and `I(...)` remain symbolic for
//! the behavioral compiler and final circuit binding pass.

use super::{
    BinOpKind, Expr as NetExpr, ParamContext, UnaryOpKind, parse_expression as parse_net_expr,
};
use crate::Value;
use std::collections::HashMap;

const MAX_FUNCTION_EXPANSION_DEPTH: usize = 64;

/// Prepare a behavioral source expression for strict compilation.
///
/// - Expands user-defined `.FUNC` calls recursively.
/// - Substitutes scoped `.PARAM` identifiers when available.
/// - Leaves non-user function calls and circuit probes untouched.
pub fn prepare_behavioral_expression(
    expression: &str,
    params: &ParamContext,
) -> Result<String, String> {
    let expression = expand_spice_poly_expression(expression)?;
    let mut probe_protector = ProbeProtector::default();
    let protected_expression = probe_protector.protect(&expression);

    // Keep legacy behavior when the permissive parser cannot parse a
    // non-standard expression; strict parser will still validate later.
    let parsed = match parse_net_expr(&protected_expression) {
        Ok(expr) => expr,
        Err(_) => return Ok(expression),
    };

    let expanded = {
        let mut expander = FunctionExpander::new(params, &mut probe_protector);
        expander.expand_expr(&parsed, 0)?
    };
    Ok(probe_protector.restore(serialize_expr(&expanded)))
}

struct FunctionExpander<'a, 'p> {
    params: &'a ParamContext,
    probe_protector: &'p mut ProbeProtector,
    body_cache: HashMap<String, NetExpr>,
    call_stack: Vec<String>,
}

impl<'a, 'p> FunctionExpander<'a, 'p> {
    fn new(params: &'a ParamContext, probe_protector: &'p mut ProbeProtector) -> Self {
        Self {
            params,
            probe_protector,
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
            NetExpr::FnCall { name, args } if is_circuit_probe(name) => Ok(NetExpr::FnCall {
                name: name.clone(),
                args: args.clone(),
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
                        let expanded_body = expand_spice_poly_expression(&func_def.body)?;
                        let protected_body = self.probe_protector.protect(&expanded_body);
                        let parsed_body = parse_net_expr(&protected_body).map_err(|e| {
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

fn is_circuit_probe(name: &str) -> bool {
    name.eq_ignore_ascii_case("V") || name.eq_ignore_ascii_case("I")
}

#[derive(Debug, Default)]
struct ProbeProtector {
    replacements: Vec<(String, String)>,
}

impl ProbeProtector {
    fn protect(&mut self, expression: &str) -> String {
        let chars: Vec<char> = expression.chars().collect();
        let mut out = String::with_capacity(expression.len());
        let mut i = 0usize;

        while i < chars.len() {
            let c = chars[i];
            if is_ident_start(c) {
                let ident_start = i;
                i += 1;
                while i < chars.len() && is_ident_continue(chars[i]) {
                    i += 1;
                }
                let ident: String = chars[ident_start..i].iter().collect();

                let mut ws_idx = i;
                while ws_idx < chars.len() && chars[ws_idx].is_whitespace() {
                    ws_idx += 1;
                }

                if is_circuit_probe(&ident)
                    && ws_idx < chars.len()
                    && chars[ws_idx] == '('
                    && let Some((inner, end_idx)) = extract_parenthesized(&chars, ws_idx)
                    && let Some(protected_inner) = self.protect_probe_inner(&inner)
                {
                    out.push_str(&ident);
                    out.push('(');
                    out.push_str(&protected_inner);
                    out.push(')');
                    i = end_idx + 1;
                    continue;
                }

                out.push_str(&ident);
                continue;
            }

            out.push(c);
            i += 1;
        }

        out
    }

    fn protect_probe_inner(&mut self, inner: &str) -> Option<String> {
        let parts = split_probe_args(inner)?;
        if parts.is_empty() || parts.len() > 2 || parts.iter().any(|part| part.is_empty()) {
            return None;
        }

        let mut protected = Vec::with_capacity(parts.len());
        for part in parts {
            if !is_simple_probe_reference(part) {
                return None;
            }
            protected.push(self.placeholder_for(part));
        }
        Some(protected.join(","))
    }

    fn placeholder_for(&mut self, original: &str) -> String {
        let placeholder = format!("__RSPICE_PROBE_REF_{}__", self.replacements.len());
        self.replacements
            .push((placeholder.clone(), original.trim().to_string()));
        placeholder
    }

    fn restore(&self, mut expression: String) -> String {
        for (placeholder, original) in &self.replacements {
            expression = expression.replace(placeholder, original);
        }
        expression
    }
}

fn split_probe_args(inner: &str) -> Option<Vec<&str>> {
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut depth = 0usize;

    for (idx, c) in inner.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                args.push(inner[start..idx].trim());
                start = idx + c.len_utf8();
            }
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }
    args.push(inner[start..].trim());
    Some(args)
}

fn is_simple_probe_reference(raw: &str) -> bool {
    !raw.is_empty()
        && raw.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(
                    c,
                    '_' | '.' | '#' | ':' | '$' | '@' | '/' | '\\' | '|' | '+' | '-' | '!'
                )
        })
}

fn expand_spice_poly_expression(expression: &str) -> Result<String, String> {
    let Some((dimension, tail)) = parse_spice_poly_header(expression)? else {
        return Ok(expression.to_string());
    };

    let items = split_spice_poly_tail(tail);
    if items.len() < dimension + 1 {
        return Err(format!(
            "POLY({dimension}) requires {dimension} controlling expression(s) and at least one coefficient"
        ));
    }

    let vars = items[..dimension].to_vec();
    let coeffs = items[dimension..].to_vec();
    ordered_spice_poly_expression(&vars, &coeffs)
}

fn parse_spice_poly_header(expression: &str) -> Result<Option<(usize, &str)>, String> {
    let trimmed = expression.trim();
    if trimmed.len() < 4 || !trimmed[..4].eq_ignore_ascii_case("POLY") {
        return Ok(None);
    }

    let after_keyword = trimmed[4..].trim_start();
    if !after_keyword.starts_with('(') {
        return Ok(None);
    }

    let Some(close_idx) = after_keyword.find(')') else {
        return Err("POLY expression is missing ')' after dimension".to_string());
    };
    let dimension = after_keyword[1..close_idx]
        .trim()
        .parse::<usize>()
        .map_err(|_| "POLY dimension must be a positive integer".to_string())?;
    if dimension == 0 {
        return Err("POLY dimension must be greater than zero".to_string());
    }

    Ok(Some((
        dimension,
        after_keyword[close_idx + 1..].trim_start(),
    )))
}

fn split_spice_poly_tail(tail: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;

    for c in tail.chars() {
        match c {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                current.push(c);
            }
            ',' if depth == 0 => {
                push_poly_item(&mut items, &mut current);
            }
            c if c.is_whitespace() && depth == 0 => {
                push_poly_item(&mut items, &mut current);
            }
            _ => current.push(c),
        }
    }
    push_poly_item(&mut items, &mut current);

    merge_sign_tokens(items)
}

fn push_poly_item(items: &mut Vec<String>, current: &mut String) {
    let item = current.trim();
    if !item.is_empty() {
        items.push(item.to_string());
    }
    current.clear();
}

fn merge_sign_tokens(items: Vec<String>) -> Vec<String> {
    let mut merged = Vec::with_capacity(items.len());
    let mut iter = items.into_iter().peekable();
    while let Some(item) = iter.next() {
        if (item == "+" || item == "-") && iter.peek().is_some() {
            let next = iter.next().expect("peeked item exists");
            if item == "-" {
                merged.push(format!("-{next}"));
            } else {
                merged.push(next);
            }
        } else {
            merged.push(item);
        }
    }
    merged
}

fn ordered_spice_poly_expression(vars: &[String], coeffs: &[String]) -> Result<String, String> {
    if coeffs.is_empty() {
        return Ok("0".to_string());
    }

    let mut terms = vec![format!("({})", coeffs[0])];
    let mut coeff_idx = 1usize;
    let mut degree = 1usize;

    while coeff_idx < coeffs.len() {
        let term_count = vars
            .len()
            .checked_pow(degree as u32)
            .ok_or_else(|| format!("POLY degree {degree} overflows term count"))?;
        for ordinal in 0..term_count {
            if coeff_idx >= coeffs.len() {
                break;
            }
            let coeff = &coeffs[coeff_idx];
            coeff_idx += 1;
            if coefficient_is_numeric_zero(coeff) {
                continue;
            }

            let mut factors = Vec::with_capacity(degree + 1);
            factors.push(format!("({coeff})"));
            for var_index in ordered_poly_indices(vars.len(), degree, ordinal) {
                factors.push(format!("({})", vars[var_index]));
            }
            terms.push(factors.join("*"));
        }
        degree += 1;
    }

    Ok(if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    })
}

fn ordered_poly_indices(var_count: usize, degree: usize, mut ordinal: usize) -> Vec<usize> {
    let mut indices = vec![0; degree];
    for slot in (0..degree).rev() {
        indices[slot] = ordinal % var_count;
        ordinal /= var_count;
    }
    indices
}

fn coefficient_is_numeric_zero(coeff: &str) -> bool {
    coeff.parse::<Value>().is_ok_and(|value| value == 0.0)
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':'
}

fn extract_parenthesized(chars: &[char], lparen_idx: usize) -> Option<(String, usize)> {
    if chars.get(lparen_idx).copied() != Some('(') {
        return None;
    }

    let mut depth = 0usize;
    let mut i = lparen_idx;
    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let inner: String = chars[lparen_idx + 1..i].iter().collect();
                    return Some((inner, i));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
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
            args: if is_circuit_probe(name) {
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
