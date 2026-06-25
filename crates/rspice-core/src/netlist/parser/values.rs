//! Token, value, expression, and model-parameter helpers.

use super::*;

pub(super) fn split_spice_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut single_quote = false;
    let mut double_quote = false;
    let mut brace_depth = 0usize;

    for ch in line.chars() {
        match ch {
            '\'' if !double_quote => {
                single_quote = !single_quote;
                current.push(ch);
            }
            '"' if !single_quote => {
                double_quote = !double_quote;
                current.push(ch);
            }
            '{' if !single_quote && !double_quote => {
                brace_depth += 1;
                current.push(ch);
            }
            '}' if !single_quote && !double_quote => {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(ch);
            }
            ',' | ' ' | '\t' if !single_quote && !double_quote && brace_depth == 0 => {
                if !current.is_empty() {
                    fields.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        fields.push(current);
    }

    fields
}

pub(super) fn strip_wrapping_expression_delimiters(raw: &str) -> &str {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0] as char;
        let last = trimmed.as_bytes()[trimmed.len() - 1] as char;
        if (first == '\'' && last == '\'')
            || (first == '"' && last == '"')
            || (first == '{' && last == '}')
        {
            return &trimmed[1..trimmed.len() - 1];
        }
    }
    trimmed
}

pub(super) fn parse_numeric_field_value(
    raw_value: &str,
    params: &ParamContext,
    line_num: usize,
) -> Result<Value, ParseError> {
    let expr = strip_wrapping_expression_delimiters(raw_value);
    if !looks_like_expression(expr)
        && let Ok(value) = crate::netlist::lexer::parse_spice_value(expr)
    {
        return Ok(value);
    }
    if let Some(value) = params.get(expr) {
        return Ok(value);
    }
    eval_expression(expr, params)
        .map_err(|e| ParseError::InvalidValue(format!("line {}: {}", line_num, e)))
}

pub(super) fn parse_parametric_field_value(
    raw_value: &str,
    params: &ParamContext,
) -> ParametricValue {
    let expr = strip_wrapping_expression_delimiters(raw_value);
    if !looks_like_expression(expr)
        && let Ok(value) = crate::netlist::lexer::parse_spice_value(expr)
    {
        return ParametricValue::Resolved(value);
    }
    if params.get(expr).is_some() || expr.chars().any(|ch| "+-*/()".contains(ch)) {
        return ParametricValue::Expression(expr.to_string());
    }
    if let Ok(value) = eval_expression(expr, params) {
        return ParametricValue::Resolved(value);
    }
    ParametricValue::Expression(expr.to_string())
}

pub(super) fn looks_like_expression(expr: &str) -> bool {
    let trimmed = expr.trim();
    for (idx, ch) in trimmed.char_indices() {
        match ch {
            '*' | '/' | '(' | ')' => return true,
            '+' | '-' if idx > 0 => {
                let prev = trimmed.as_bytes()[idx - 1] as char;
                if prev != 'e' && prev != 'E' {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

pub(super) struct ParsedModelParams {
    pub(super) numeric: Vec<(String, Value)>,
    pub(super) expr: Vec<(String, String)>,
    pub(super) string: Vec<(String, String)>,
}

pub(super) fn parse_model_params(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Result<ParsedModelParams, ParseError> {
    let mut numeric_params = Vec::new();
    let mut expr_params = Vec::new();
    let mut string_params = Vec::new();

    // Skip optional opening paren
    stream.consume(&TokenKind::LParen);

    while !stream.is_eof() {
        skip_commas(stream);

        if matches!(
            stream.peek().kind,
            TokenKind::RParen | TokenKind::Newline | TokenKind::Eof
        ) {
            break;
        }

        // Look for NAME=VALUE
        if let TokenKind::Ident(name) = &stream.peek().kind {
            let name = name.clone();
            stream.advance();

            if stream.consume(&TokenKind::Equals) {
                match &stream.peek().kind {
                    TokenKind::StringLit(value) => {
                        let value = value.clone();
                        stream.advance();
                        string_params.push((name, value));
                    }
                    TokenKind::Expression(expr) => {
                        let expr = expr.clone();
                        stream.advance();
                        if let Ok(value) = eval_expression(&expr, params) {
                            numeric_params.push((name, value));
                        } else {
                            expr_params.push((name, expr));
                        }
                    }
                    _ => {
                        if stream.consume(&TokenKind::LParen) {
                            if let Some(value) = try_signed_model_value(stream, params) {
                                numeric_params.push((name, value));
                            }
                            if !stream.consume(&TokenKind::RParen) {
                                return Err(ParseError::Syntax {
                                    line: stream.line(),
                                    message:
                                        "Expected ')' after parenthesized model parameter value"
                                            .to_string(),
                                });
                            }
                        } else if let Some(value) = try_signed_model_value(stream, params) {
                            numeric_params.push((name, value));
                        }
                    }
                }
            } else {
                numeric_params.push((name, 1.0));
            }
        } else {
            stream.advance(); // Skip unknown token
        }
    }

    // Skip optional closing paren
    stream.consume(&TokenKind::RParen);

    Ok(ParsedModelParams {
        numeric: numeric_params,
        expr: expr_params,
        string: string_params,
    })
}

pub(super) fn expect_ident(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let s = s.clone();
            stream.advance();
            Ok(s)
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", other),
        }),
    }
}

pub(super) fn expect_node(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Ident(s) => {
            let s = s.clone();
            stream.advance();
            Ok(s)
        }
        TokenKind::Number(n) => {
            // Numeric node name (e.g., "0", "1")
            let s = format!("{}", *n as i64);
            stream.advance();
            Ok(s)
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected node name, found {:?}", other),
        }),
    }
}

pub(super) fn expect_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    skip_commas(stream);

    // Handle optional sign prefix (+15 or -15)
    let sign = match &stream.peek().kind {
        TokenKind::Plus => {
            stream.advance();
            1.0
        }
        TokenKind::Minus => {
            stream.advance();
            -1.0
        }
        _ => 1.0,
    };

    match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v * sign;
            stream.advance();
            Ok(v)
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params)
                .map(|v| v * sign)
                .map_err(|e| ParseError::InvalidValue(e.to_string()))
        }
        TokenKind::Ident(s) => {
            // Could be a parameter reference
            if let Some(v) = params.get(s) {
                stream.advance();
                Ok(v * sign)
            } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(s) {
                stream.advance();
                Ok(v * sign)
            } else {
                Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Expected value, found identifier '{}'", s),
                })
            }
        }
        other => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected value, found {:?}", other),
        }),
    }
}

pub(super) fn try_value(stream: &mut TokenStream, params: &ParamContext) -> Option<Value> {
    skip_commas(stream);
    try_value_unsigned(stream, params)
}

/// Like [`try_value`] but also recombines a `Minus`/`Plus` sign token that the
/// lexer split off from a magnitude written without a leading zero (e.g.
/// `-.14`, `+.5`). Model-card parameters such as `VOFF=-.14` depend on this.
///
/// Restricted to model-parameter parsing: element/source/command parsers handle
/// signs in their own layers, so applying it there could double-consume tokens.
pub(super) fn try_signed_model_value(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Option<Value> {
    skip_commas(stream);
    let sign = match &stream.peek().kind {
        TokenKind::Plus => Some(1.0),
        TokenKind::Minus => Some(-1.0),
        _ => None,
    };
    if let Some(sign) = sign {
        if token_is_value_like(&stream.peek_n(1).kind, params) {
            stream.advance();
            if let Some(magnitude) = try_value_unsigned(stream, params) {
                return Some(sign * magnitude);
            }
        }
    }
    try_value_unsigned(stream, params)
}

#[inline]
fn token_is_value_like(kind: &TokenKind, params: &ParamContext) -> bool {
    match kind {
        TokenKind::Number(_) | TokenKind::Expression(_) => true,
        TokenKind::Ident(s) => {
            params.get(s).is_some() || crate::netlist::lexer::parse_spice_value(s).is_ok()
        }
        _ => false,
    }
}

fn try_value_unsigned(stream: &mut TokenStream, params: &ParamContext) -> Option<Value> {
    match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v;
            stream.advance();
            Some(v)
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params).ok()
        }
        TokenKind::Ident(s) => {
            if let Some(v) = params.get(s) {
                stream.advance();
                Some(v)
            } else if let Ok(v) = crate::netlist::lexer::parse_spice_value(s) {
                stream.advance();
                Some(v)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(super) fn take_value_expression_string(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Option<String> {
    skip_commas(stream);

    let sign = match &stream.peek().kind {
        TokenKind::Plus => {
            stream.advance();
            ""
        }
        TokenKind::Minus => {
            stream.advance();
            "-"
        }
        _ => "",
    };

    match &stream.peek().kind {
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            if sign.is_empty() {
                Some(expr)
            } else {
                Some(format!("-({expr})"))
            }
        }
        TokenKind::Ident(name) if params.get(name).is_some() => {
            let ident = name.clone();
            stream.advance();
            if sign.is_empty() {
                Some(ident)
            } else {
                Some(format!("-({ident})"))
            }
        }
        _ => None,
    }
}

pub(super) fn expect_value_default(
    stream: &mut TokenStream,
    params: &ParamContext,
    default: Value,
) -> Value {
    skip_commas(stream);
    try_value(stream, params).unwrap_or(default)
}

/// An instance-parameter value: resolved at parse time, or captured as
/// expression text for per-instance evaluation during flattening.
pub(super) enum DeferrableValue {
    Resolved(Value),
    Deferred(String),
}

/// Take one instance-parameter value. With `defer` set (subcircuit bodies),
/// brace expressions and parameter references are captured as expression
/// text instead of being evaluated against the definition-time scope, whose
/// defaults would otherwise shadow per-instance overrides. Plain numerics
/// resolve immediately in both modes.
pub(super) fn take_deferrable_value(
    stream: &mut TokenStream,
    params: &ParamContext,
    defer: bool,
) -> Option<DeferrableValue> {
    skip_commas(stream);
    if defer {
        match &stream.peek().kind {
            TokenKind::Expression(_) => {
                return take_value_expression_string(stream, params).map(DeferrableValue::Deferred);
            }
            TokenKind::Minus | TokenKind::Plus
                if matches!(stream.peek_n(1).kind, TokenKind::Expression(_)) =>
            {
                return take_value_expression_string(stream, params).map(DeferrableValue::Deferred);
            }
            TokenKind::Ident(name) => {
                // Engineering-suffixed numerics ("1u", "2.5k") resolve below;
                // anything non-numeric here is a parameter reference that may
                // be overridden (or only provided) at the instance, so it has
                // to survive to flattening as text.
                if crate::netlist::lexer::parse_spice_value(name).is_err() {
                    let expr = name.clone();
                    stream.advance();
                    return Some(DeferrableValue::Deferred(expr));
                }
            }
            _ => {}
        }
    }
    try_value(stream, params).map(DeferrableValue::Resolved)
}

pub(super) fn skip_optional_param_name(stream: &mut TokenStream, param_name: &str) {
    if let TokenKind::Ident(s) = &stream.peek().kind
        && s == param_name
    {
        stream.advance();
        stream.consume(&TokenKind::Equals);
    }
}

pub(super) fn skip_commas(stream: &mut TokenStream) {
    while stream.consume(&TokenKind::Comma) {}
}

pub(super) fn lex_to_parse_error(e: LexError, line_num: usize) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: e.to_string(),
    }
}
