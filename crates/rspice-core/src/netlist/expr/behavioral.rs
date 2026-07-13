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

/// Match the evaluator's production recursion guard. Expansion depth counts
/// named `.FUNC` and `.GLOBAL_PARAM` dependencies only; ordinary AST shape is
/// not recursion and must not consume this budget.
const MAX_NAMED_EXPANSION_DEPTH: usize = 4096;
const MAX_EXPANDED_EXPRESSION_NODES: usize = 1_000_000;
const STACK_SAFE_CONSTANT_FOLD_DEPTH: usize = 256;
const LARGE_FUNCTION_GRAPH_THRESHOLD: usize = 64;

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

/// Return whether a prepared behavioral expression depends on a quantity that
/// changes with the active analysis point. This shared classifier keeps
/// flattening and circuit construction from making different static/runtime
/// decisions for nested global-parameter expressions.
pub fn behavioral_expression_references_runtime_quantity(expression: &str) -> bool {
    if let Ok(expression) = crate::expr::parse_expression_strict(expression) {
        return strict_expr_references_runtime_quantity(&expression);
    }
    let parsed = match parse_net_expr(expression) {
        Ok(expression) => expression,
        Err(_) => {
            let upper = expression.to_ascii_uppercase();
            return upper.contains("V(")
                || upper.contains("I(")
                || contains_runtime_identifier(expression);
        }
    };
    net_expr_references_runtime_quantity(&parsed)
}

fn strict_expr_references_runtime_quantity(expression: &crate::expr::Expr) -> bool {
    match expression {
        crate::expr::Expr::NodeVoltage(_)
        | crate::expr::Expr::BranchCurrent(_)
        | crate::expr::Expr::LookupTable(_)
        | crate::expr::Expr::Time
        | crate::expr::Expr::Frequency => true,
        crate::expr::Expr::Unary { operand, .. } => {
            strict_expr_references_runtime_quantity(operand)
        }
        crate::expr::Expr::Binary { left, right, .. } => {
            strict_expr_references_runtime_quantity(left)
                || strict_expr_references_runtime_quantity(right)
        }
        crate::expr::Expr::Function { args, .. } => {
            args.iter().any(strict_expr_references_runtime_quantity)
        }
        crate::expr::Expr::Const(_)
        | crate::expr::Expr::StringLiteral(_)
        | crate::expr::Expr::Temperature => false,
    }
}

/// Validate every retained `.GLOBAL_PARAM` expression after the complete
/// top-level scope has been parsed. This provides declaration-order-
/// independent dependency resolution while rejecting cycles, undefined
/// symbols, and circuit probes that Xyce does not permit in global parameters.
pub fn validate_global_parameter_expressions(params: &ParamContext) -> Result<(), String> {
    let mut definitions = params.all_global_expressions();
    definitions.sort_by(|left, right| left.0.cmp(&right.0));
    for (name, _) in definitions {
        let prepared = prepare_behavioral_expression(&name, params)
            .map_err(|error| format!("Unable to resolve global parameter {name}: {error}"))?;
        let parsed = parse_net_expr(&prepared)
            .map_err(|error| format!("Unable to resolve global parameter {name}: {error}"))?;
        if net_expr_contains_circuit_probe(&parsed) {
            return Err(format!(
                "Global parameter {name} may not reference node voltages or branch currents"
            ));
        }
        if let Some(identifier) = first_unresolved_global_identifier(&parsed) {
            return Err(format!(
                "Unable to resolve global parameter {name}: Undefined parameter: {identifier}"
            ));
        }
        crate::expr::parse_expression_strict(&prepared)
            .map_err(|error| format!("Unable to resolve global parameter {name}: {error}"))?;
    }
    Ok(())
}

fn net_expr_references_runtime_quantity(expression: &NetExpr) -> bool {
    match expression {
        NetExpr::Param(name) => {
            name.eq_ignore_ascii_case("TIME") || name.eq_ignore_ascii_case("FREQ")
        }
        NetExpr::UnaryOp { operand, .. } => net_expr_references_runtime_quantity(operand),
        NetExpr::BinOp { left, right, .. } => {
            net_expr_references_runtime_quantity(left)
                || net_expr_references_runtime_quantity(right)
        }
        NetExpr::FnCall { name, args } => {
            is_circuit_probe(name) || args.iter().any(net_expr_references_runtime_quantity)
        }
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) => false,
    }
}

fn net_expr_contains_circuit_probe(expression: &NetExpr) -> bool {
    match expression {
        NetExpr::UnaryOp { operand, .. } => net_expr_contains_circuit_probe(operand),
        NetExpr::BinOp { left, right, .. } => {
            net_expr_contains_circuit_probe(left) || net_expr_contains_circuit_probe(right)
        }
        NetExpr::FnCall { name, args } => {
            is_circuit_probe(name) || args.iter().any(net_expr_contains_circuit_probe)
        }
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) | NetExpr::Param(_) => false,
    }
}

fn first_unresolved_global_identifier(expression: &NetExpr) -> Option<&str> {
    match expression {
        NetExpr::Param(name) if name.eq_ignore_ascii_case("TIME") => None,
        NetExpr::Param(name) => Some(name.as_str()),
        NetExpr::UnaryOp { operand, .. } => first_unresolved_global_identifier(operand),
        NetExpr::BinOp { left, right, .. } => first_unresolved_global_identifier(left)
            .or_else(|| first_unresolved_global_identifier(right)),
        NetExpr::FnCall { args, .. } => args.iter().find_map(first_unresolved_global_identifier),
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) => None,
    }
}

fn contains_runtime_identifier(expression: &str) -> bool {
    expression
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .any(|token| token.eq_ignore_ascii_case("TIME") || token.eq_ignore_ascii_case("FREQ"))
}

fn contains_statistical_function_call(expression: &str) -> bool {
    let bytes = expression.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            let name = &expression[start..index];
            let mut call_index = index;
            while call_index < bytes.len() && bytes[call_index].is_ascii_whitespace() {
                call_index += 1;
            }
            if call_index < bytes.len()
                && bytes[call_index] == b'('
                && ["GAUSS", "AGAUSS", "UNIF", "AUNIF", "RAND"]
                    .iter()
                    .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return true;
            }
        } else {
            index += 1;
        }
    }
    false
}

struct FunctionExpander<'a, 'p> {
    params: &'a ParamContext,
    probe_protector: &'p mut ProbeProtector,
    body_cache: HashMap<String, NetExpr>,
    call_stack: Vec<String>,
    global_body_cache: HashMap<String, NetExpr>,
    global_stack: Vec<String>,
    fold_static_function_graph: bool,
}

impl<'a, 'p> FunctionExpander<'a, 'p> {
    fn new(params: &'a ParamContext, probe_protector: &'p mut ProbeProtector) -> Self {
        Self {
            params,
            probe_protector,
            body_cache: HashMap::new(),
            call_stack: Vec::new(),
            global_body_cache: HashMap::new(),
            global_stack: Vec::new(),
            fold_static_function_graph: params.function_count() > LARGE_FUNCTION_GRAPH_THRESHOLD,
        }
    }

    fn expand_expr(&mut self, expr: &NetExpr, named_depth: usize) -> Result<NetExpr, String> {
        enum Task {
            Expand(NetExpr, usize),
            FinishUnary(UnaryOpKind, usize),
            FinishBinary(BinOpKind, usize),
            ApplyFunction(String, usize, usize),
            ExitFunction(String),
            ExitGlobal(String),
        }

        let mut tasks = vec![Task::Expand(expr.clone(), named_depth)];
        let mut values = Vec::new();
        let mut expanded_nodes = 0usize;

        while let Some(task) = tasks.pop() {
            match task {
                Task::Expand(expr, depth) => {
                    if depth > MAX_NAMED_EXPANSION_DEPTH {
                        return Err(format!(
                            "Behavioral expression exceeded named dependency expansion depth (>{})",
                            MAX_NAMED_EXPANSION_DEPTH
                        ));
                    }
                    expanded_nodes += 1;
                    if expanded_nodes > MAX_EXPANDED_EXPRESSION_NODES {
                        return Err(format!(
                            "Behavioral expression expansion exceeded {} nodes",
                            MAX_EXPANDED_EXPRESSION_NODES
                        ));
                    }
                    match expr {
                        NetExpr::Number(value) => values.push(NetExpr::Number(value)),
                        NetExpr::ComplexNumber(value) => {
                            values.push(NetExpr::Number(value.real_projection()));
                        }
                        NetExpr::Param(name) if is_behavioral_runtime_symbol(&name) => {
                            values.push(NetExpr::Param(name));
                        }
                        NetExpr::Param(name) => {
                            if let Some(expression) = self.params.get_global_expression(&name) {
                                if contains_statistical_function_call(expression)
                                    && !contains_runtime_identifier(expression)
                                    && let Some(value) = self.params.get_complex(&name)
                                {
                                    values.push(NetExpr::Number(value.real_projection()));
                                    continue;
                                }
                                let key = name.to_ascii_uppercase();
                                if self.global_stack.iter().any(|active| active == &key) {
                                    let mut cycle = self.global_stack.clone();
                                    cycle.push(key);
                                    return Err(format!(
                                        "Detected cyclic .GLOBAL_PARAM dependency: {}",
                                        cycle.join(" -> ")
                                    ));
                                }
                                let parsed = if let Some(cached) = self.global_body_cache.get(&key)
                                {
                                    cached.clone()
                                } else {
                                    let protected = self.probe_protector.protect(expression);
                                    let parsed = parse_net_expr(&protected).map_err(|error| {
                                        format!(
                                            "Failed to parse .GLOBAL_PARAM {} expression '{}': {}",
                                            name, expression, error
                                        )
                                    })?;
                                    self.global_body_cache.insert(key.clone(), parsed.clone());
                                    parsed
                                };
                                self.global_stack.push(key.clone());
                                tasks.push(Task::ExitGlobal(key));
                                tasks.push(Task::Expand(parsed, depth + 1));
                            } else if let Some(value) = self.params.get(&name) {
                                values.push(NetExpr::Number(value));
                            } else {
                                values.push(NetExpr::Param(name));
                            }
                        }
                        NetExpr::UnaryOp { op, operand } => {
                            tasks.push(Task::FinishUnary(op, depth));
                            tasks.push(Task::Expand(*operand, depth));
                        }
                        NetExpr::BinOp { op, left, right } => {
                            tasks.push(Task::FinishBinary(op, depth));
                            tasks.push(Task::Expand(*right, depth));
                            tasks.push(Task::Expand(*left, depth));
                        }
                        NetExpr::FnCall { name, args } if is_circuit_probe(&name) => {
                            values.push(NetExpr::FnCall { name, args });
                        }
                        NetExpr::FnCall { name, args } => {
                            let arg_count = args.len();
                            tasks.push(Task::ApplyFunction(name, arg_count, depth));
                            for arg in args.into_iter().rev() {
                                tasks.push(Task::Expand(arg, depth));
                            }
                        }
                    }
                }
                Task::FinishUnary(op, depth) => {
                    let operand = values.pop().ok_or_else(|| {
                        "Internal behavioral expansion stack underflow".to_string()
                    })?;
                    if let NetExpr::Number(value) = &operand
                        && (self.fold_static_function_graph
                            || depth >= STACK_SAFE_CONSTANT_FOLD_DEPTH
                            || matches!(op, UnaryOpKind::Not))
                    {
                        values.push(NetExpr::Number(match op {
                            UnaryOpKind::Neg => -*value,
                            UnaryOpKind::Pos => *value,
                            UnaryOpKind::Not => f64::from(*value == 0.0),
                        }));
                    } else {
                        values.push(NetExpr::UnaryOp {
                            op,
                            operand: Box::new(operand),
                        });
                    }
                }
                Task::FinishBinary(op, depth) => {
                    let right = values.pop().ok_or_else(|| {
                        "Internal behavioral expansion stack underflow".to_string()
                    })?;
                    let left = values.pop().ok_or_else(|| {
                        "Internal behavioral expansion stack underflow".to_string()
                    })?;
                    if let (NetExpr::Number(left_value), NetExpr::Number(right_value)) =
                        (&left, &right)
                        && let Some(value) = fold_real_binary(
                            op,
                            *left_value,
                            *right_value,
                            self.fold_static_function_graph
                                || depth >= STACK_SAFE_CONSTANT_FOLD_DEPTH,
                        )
                    {
                        values.push(NetExpr::Number(value));
                    } else {
                        values.push(NetExpr::BinOp {
                            op,
                            left: Box::new(left),
                            right: Box::new(right),
                        });
                    }
                }
                Task::ApplyFunction(name, arg_count, depth) => {
                    if values.len() < arg_count {
                        return Err("Internal behavioral argument stack underflow".to_string());
                    }
                    let expanded_args = values.split_off(values.len() - arg_count);
                    let Some(func_def) = self.params.get_function(&name) else {
                        if let Some(value) = fold_condition_function(&name, &expanded_args) {
                            values.push(value);
                            continue;
                        }
                        values.push(NetExpr::FnCall {
                            name,
                            args: expanded_args,
                        });
                        continue;
                    };
                    if expanded_args.len() != func_def.args.len() {
                        return Err(format!(
                            "Function '{}' expects {} args but got {}",
                            name,
                            func_def.args.len(),
                            expanded_args.len()
                        ));
                    }
                    let func_name = func_def.name.clone();
                    if self.call_stack.iter().any(|active| active == &func_name) {
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
                        let parsed_body = parse_net_expr(&protected_body).map_err(|error| {
                            format!(
                                "Failed to parse .FUNC {} body '{}': {}",
                                func_name, func_def.body, error
                            )
                        })?;
                        self.body_cache
                            .insert(func_name.clone(), parsed_body.clone());
                        parsed_body
                    };
                    let mut bindings = HashMap::with_capacity(func_def.args.len());
                    for (arg_name, arg_value) in func_def.args.iter().zip(expanded_args) {
                        bindings.insert(arg_name.to_ascii_uppercase(), arg_value);
                    }
                    let substituted = substitute_function_args(&body_ast, &bindings);
                    self.call_stack.push(func_name.clone());
                    tasks.push(Task::ExitFunction(func_name));
                    tasks.push(Task::Expand(substituted, depth + 1));
                }
                Task::ExitFunction(name) => {
                    debug_assert_eq!(self.call_stack.last(), Some(&name));
                    self.call_stack.pop();
                }
                Task::ExitGlobal(name) => {
                    debug_assert_eq!(self.global_stack.last(), Some(&name));
                    self.global_stack.pop();
                }
            }
        }

        if values.len() != 1 {
            return Err(format!(
                "Internal behavioral expansion produced {} results",
                values.len()
            ));
        }
        values
            .pop()
            .ok_or_else(|| "Internal behavioral expansion produced no result".to_string())
    }
}

fn fold_real_binary(
    op: BinOpKind,
    left: Value,
    right: Value,
    allow_arithmetic: bool,
) -> Option<Value> {
    let value = match op {
        BinOpKind::Add if allow_arithmetic => left + right,
        BinOpKind::Sub if allow_arithmetic => left - right,
        BinOpKind::Mul if allow_arithmetic => left * right,
        BinOpKind::Div if allow_arithmetic && right != 0.0 => left / right,
        BinOpKind::Mod if allow_arithmetic && right != 0.0 => left % right,
        BinOpKind::Pow if allow_arithmetic => left.powf(right),
        BinOpKind::Gt => f64::from(left > right),
        BinOpKind::Lt => f64::from(left < right),
        BinOpKind::Ge => f64::from(left >= right),
        BinOpKind::Le => f64::from(left <= right),
        BinOpKind::Eq => f64::from((left - right).abs() < 1e-12),
        BinOpKind::Ne => f64::from((left - right).abs() >= 1e-12),
        BinOpKind::And => f64::from(left != 0.0 && right != 0.0),
        BinOpKind::Or => f64::from(left != 0.0 || right != 0.0),
        BinOpKind::Add
        | BinOpKind::Sub
        | BinOpKind::Mul
        | BinOpKind::Div
        | BinOpKind::Mod
        | BinOpKind::Pow => return None,
    };
    value.is_finite().then_some(value)
}

fn fold_condition_function(name: &str, args: &[NetExpr]) -> Option<NetExpr> {
    if !(name.eq_ignore_ascii_case("IF") || name.eq_ignore_ascii_case("TERNARY_FCN"))
        || args.len() != 3
    {
        return None;
    }
    let NetExpr::Number(condition) = &args[0] else {
        return None;
    };
    Some(if *condition != 0.0 {
        args[1].clone()
    } else {
        args[2].clone()
    })
}

fn is_behavioral_runtime_symbol(name: &str) -> bool {
    name.eq_ignore_ascii_case("TIME")
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
            if c == '"' {
                let start = i;
                i += 1;
                let mut escaped = false;
                while i < chars.len() {
                    let current = chars[i];
                    i += 1;
                    if escaped {
                        escaped = false;
                    } else if current == '\\' {
                        escaped = true;
                    } else if current == '"' {
                        break;
                    }
                }
                let original: String = chars[start..i].iter().collect();
                out.push_str(&self.placeholder_for_literal(&original));
                continue;
            }
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

    fn placeholder_for_literal(&mut self, original: &str) -> String {
        let placeholder = format!("__RSPICE_STRING_LITERAL_{}__", self.replacements.len());
        self.replacements
            .push((placeholder.clone(), original.to_string()));
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
    c.is_ascii_alphabetic() || matches!(c, '_' | '`' | '@' | '#' | '$')
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '`' | '@' | '#' | '$')
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
        NetExpr::ComplexNumber(v) => NetExpr::Number(v.real_projection()),
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
    enum Task<'a> {
        Expr(&'a NetExpr),
        Static(&'static str),
        Owned(String),
    }

    let mut output = String::new();
    let mut tasks = vec![Task::Expr(expr)];
    while let Some(task) = tasks.pop() {
        match task {
            Task::Static(text) => output.push_str(text),
            Task::Owned(text) => output.push_str(&text),
            Task::Expr(NetExpr::Number(value)) => {
                if *value < 0.0 {
                    output.push('(');
                    output.push_str(&value.to_string());
                    output.push(')');
                } else {
                    output.push_str(&value.to_string());
                }
            }
            Task::Expr(NetExpr::ComplexNumber(value)) => {
                let projection = value.real_projection();
                if projection < 0.0 {
                    output.push('(');
                    output.push_str(&projection.to_string());
                    output.push(')');
                } else {
                    output.push_str(&projection.to_string());
                }
            }
            Task::Expr(NetExpr::Param(name)) => output.push_str(name),
            Task::Expr(NetExpr::UnaryOp { op, operand }) => {
                tasks.push(Task::Static(")"));
                tasks.push(Task::Expr(operand));
                tasks.push(Task::Static(match op {
                    UnaryOpKind::Neg => "(-",
                    UnaryOpKind::Pos => "(",
                    UnaryOpKind::Not => "(!",
                }));
            }
            Task::Expr(NetExpr::BinOp { op, left, right }) => {
                let symbol = match op {
                    BinOpKind::Add => "+",
                    BinOpKind::Sub => "-",
                    BinOpKind::Mul => "*",
                    BinOpKind::Div => "/",
                    BinOpKind::Mod => "%",
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
                tasks.push(Task::Static(")"));
                tasks.push(Task::Expr(right));
                tasks.push(Task::Static(symbol));
                tasks.push(Task::Expr(left));
                tasks.push(Task::Static("("));
            }
            Task::Expr(NetExpr::FnCall { name, args }) => {
                tasks.push(Task::Static(")"));
                for index in (0..args.len()).rev() {
                    tasks.push(Task::Expr(&args[index]));
                    if index > 0 {
                        tasks.push(Task::Static(","));
                    }
                }
                tasks.push(Task::Static("("));
                tasks.push(Task::Owned(name.to_ascii_uppercase()));
            }
        }
    }
    output
}
