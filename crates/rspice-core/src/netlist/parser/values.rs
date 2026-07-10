//! Token, value, expression, and model-parameter helpers.

use super::*;
use crate::netlist::lexer::{Token, collect_contiguous_expression};

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
    if let Some(value) = strip_wrapping_double_quoted_string_literal(raw_value) {
        return ParametricValue::String(value.to_string());
    }
    let expr = strip_wrapping_expression_delimiters(raw_value);
    if !looks_like_expression(expr)
        && let Ok(value) = crate::netlist::lexer::parse_spice_value(expr)
    {
        return ParametricValue::Resolved(value);
    }
    if params.get_string(expr).is_some() {
        return ParametricValue::StringExpression(expr.to_string());
    }
    if params.get(expr).is_some() || expr.chars().any(|ch| "+-*/()".contains(ch)) {
        return ParametricValue::Expression(expr.to_string());
    }
    if let Ok(value) = eval_expression(expr, params) {
        return ParametricValue::Resolved(value);
    }
    ParametricValue::Expression(expr.to_string())
}

pub(super) fn strip_wrapping_double_quoted_string_literal(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if trimmed.len() >= 2
        && trimmed.as_bytes()[0] as char == '"'
        && trimmed.as_bytes()[trimmed.len() - 1] as char == '"'
    {
        Some(&trimmed[1..trimmed.len() - 1])
    } else {
        None
    }
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
    pub(super) string_vector: Vec<(String, Vec<String>)>,
    pub(super) real_vector: Vec<(String, Vec<Value>)>,
    pub(super) real_vector_expr: Vec<(String, Vec<String>)>,
    pub(super) integer_vector: Vec<(String, Vec<i64>)>,
}

pub(super) fn strip_wrapping_string_literal(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0] as char;
        let last = trimmed.as_bytes()[trimmed.len() - 1] as char;
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return Some(&trimmed[1..trimmed.len() - 1]);
        }
    }
    None
}

pub(super) fn parse_model_params(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    defer_expression_params: bool,
    model_type: Option<&str>,
) -> Result<ParsedModelParams, ParseError> {
    let mut numeric_params = Vec::new();
    let mut expr_params = Vec::new();
    let mut string_params = Vec::new();
    let mut string_vector_params = Vec::new();
    let mut real_vector_params = Vec::new();
    let mut real_vector_expr_params = Vec::new();
    let integer_vector_params = Vec::new();

    let opened_paren = stream.consume(&TokenKind::LParen);
    let allow_missing_close = opened_paren
        && model_type
            .map(crate::xspice::CodeModelRegistry::is_builtin_model_name)
            .unwrap_or(false);

    loop {
        skip_commas(stream);

        match &stream.peek().kind {
            TokenKind::RParen if opened_paren => {
                stream.advance();
                return Ok(ParsedModelParams {
                    numeric: numeric_params,
                    expr: expr_params,
                    string: string_params,
                    string_vector: string_vector_params,
                    real_vector: real_vector_params,
                    real_vector_expr: real_vector_expr_params,
                    integer_vector: integer_vector_params,
                });
            }
            TokenKind::RParen => {
                let has_params = !numeric_params.is_empty()
                    || !expr_params.is_empty()
                    || !string_params.is_empty()
                    || !string_vector_params.is_empty()
                    || !real_vector_params.is_empty()
                    || !real_vector_expr_params.is_empty()
                    || !integer_vector_params.is_empty();
                if has_params {
                    stream.advance();
                    skip_commas(stream);
                    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                        return Ok(ParsedModelParams {
                            numeric: numeric_params,
                            expr: expr_params,
                            string: string_params,
                            string_vector: string_vector_params,
                            real_vector: real_vector_params,
                            real_vector_expr: real_vector_expr_params,
                            integer_vector: integer_vector_params,
                        });
                    }
                }
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Unexpected ')' in .MODEL parameter list".to_string(),
                });
            }
            TokenKind::Newline | TokenKind::Eof if opened_paren => {
                let has_params = !numeric_params.is_empty()
                    || !expr_params.is_empty()
                    || !string_params.is_empty()
                    || !string_vector_params.is_empty()
                    || !real_vector_params.is_empty()
                    || !real_vector_expr_params.is_empty()
                    || !integer_vector_params.is_empty();
                if allow_missing_close && has_params {
                    return Ok(ParsedModelParams {
                        numeric: numeric_params,
                        expr: expr_params,
                        string: string_params,
                        string_vector: string_vector_params,
                        real_vector: real_vector_params,
                        real_vector_expr: real_vector_expr_params,
                        integer_vector: integer_vector_params,
                    });
                }
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: "Parenthesized .MODEL parameter list is missing ')'".to_string(),
                });
            }
            TokenKind::Newline | TokenKind::Eof => break,
            _ => {}
        }

        // Look for NAME=VALUE
        if let TokenKind::Ident(name) = &stream.peek().kind {
            let name = name.clone();
            stream.advance();

            if stream.consume(&TokenKind::Equals) {
                if name.eq_ignore_ascii_case("VERSION")
                    && let Some(version) = try_dotted_model_version(stream)
                {
                    string_params.push((name, version));
                    continue;
                }
                match &stream.peek().kind {
                    TokenKind::StringLit(value) => {
                        let value = value.clone();
                        stream.advance();
                        string_params.push((name, value));
                    }
                    TokenKind::Other('<') => {
                        match parse_model_complex_literal(
                            stream,
                            line_num,
                            &name,
                            params,
                            defer_expression_params,
                        )? {
                            ParsedModelComplexLiteral::Resolved(value) => {
                                string_params.push((name, value));
                            }
                            ParsedModelComplexLiteral::Deferred { real, imag } => {
                                expr_params.push((
                                    name,
                                    crate::netlist::encode_deferred_xspice_complex(&real, &imag),
                                ));
                            }
                        }
                    }
                    TokenKind::Number(_)
                        if crate::netlist::xspice_param_preserves_numeric_string(&name)
                            || model_param_accepts_bare_string(&name, model_type) =>
                    {
                        let value = parse_model_bare_string_value(stream, line_num, &name)?;
                        push_model_string_value(
                            &mut string_params,
                            &mut string_vector_params,
                            &mut real_vector_params,
                            line_num,
                            &name,
                            &value,
                        )?;
                    }
                    TokenKind::Ident(value) => {
                        let value = value.clone();
                        if let Some(value) = bare_model_ident_string_value(
                            &name,
                            &value,
                            params,
                            defer_expression_params,
                            model_type,
                        ) {
                            stream.advance();
                            match value {
                                BareModelIdentString::Literal(value) => push_model_string_value(
                                    &mut string_params,
                                    &mut string_vector_params,
                                    &mut real_vector_params,
                                    line_num,
                                    &name,
                                    &value,
                                )?,
                                BareModelIdentString::Deferred(expr) => {
                                    expr_params.push((name, expr));
                                }
                            }
                        } else if model_param_accepts_bare_string(&name, model_type) {
                            let value = parse_model_bare_string_value(stream, line_num, &name)?;
                            push_model_string_value(
                                &mut string_params,
                                &mut string_vector_params,
                                &mut real_vector_params,
                                line_num,
                                &name,
                                &value,
                            )?;
                        } else if xspice_model_type_accepts_contiguous_expressions(model_type)
                            && let Some(value) = try_xspice_model_scalar_expression(
                                stream,
                                params,
                                defer_expression_params,
                            )
                        {
                            push_model_scalar_expression_param(
                                &mut numeric_params,
                                &mut expr_params,
                                name,
                                value,
                            );
                        } else if let Some(value) = try_signed_model_value(stream, params) {
                            numeric_params.push((name, value));
                        } else {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for model parameter '{}', found {}",
                                    name,
                                    stream.peek().kind
                                ),
                            });
                        }
                    }
                    TokenKind::Expression(_) => {
                        if xspice_model_type_accepts_contiguous_expressions(model_type)
                            && let Some(value) = try_xspice_model_scalar_expression(
                                stream,
                                params,
                                defer_expression_params,
                            )
                        {
                            push_model_scalar_expression_param(
                                &mut numeric_params,
                                &mut expr_params,
                                name,
                                value,
                            );
                        } else {
                            let TokenKind::Expression(expr) = &stream.peek().kind else {
                                unreachable!("expression branch matched before fallback")
                            };
                            let expr = expr.clone();
                            stream.advance();
                            if defer_expression_params {
                                expr_params.push((name, expr));
                            } else if let Ok(value) = eval_expression(&expr, params) {
                                numeric_params.push((name, value));
                            } else if let Some(value) = params.get_string(&expr) {
                                push_model_string_value(
                                    &mut string_params,
                                    &mut string_vector_params,
                                    &mut real_vector_params,
                                    line_num,
                                    &name,
                                    value,
                                )?;
                            } else {
                                expr_params.push((name, expr));
                            }
                        }
                    }
                    TokenKind::LBracket => {
                        if crate::netlist::xspice_param_prefers_string_vector(&name)
                            || model_vector_starts_with_complex(stream)
                            || model_vector_starts_with_string(
                                stream,
                                params,
                                defer_expression_params,
                            )
                        {
                            match parse_model_string_vector(
                                stream,
                                line_num,
                                &name,
                                params,
                                defer_expression_params,
                            )? {
                                ParsedModelStringVector::Resolved(values) => {
                                    string_vector_params.push((name, values));
                                }
                                ParsedModelStringVector::Deferred(expr) => {
                                    expr_params.push((name, expr));
                                }
                            }
                        } else {
                            match parse_model_real_vector(
                                stream,
                                line_num,
                                &name,
                                params,
                                defer_expression_params,
                            )? {
                                ParsedModelRealVector::Resolved(values) => {
                                    real_vector_params.push((name, values));
                                }
                                ParsedModelRealVector::Deferred(exprs) => {
                                    real_vector_expr_params.push((name, exprs));
                                }
                            }
                        }
                    }
                    kind if model_param_accepts_bare_string(&name, model_type)
                        && model_bare_string_token_can_start(kind) =>
                    {
                        let value = parse_model_bare_string_value(stream, line_num, &name)?;
                        push_model_string_value(
                            &mut string_params,
                            &mut string_vector_params,
                            &mut real_vector_params,
                            line_num,
                            &name,
                            &value,
                        )?;
                    }
                    _ => {
                        if xspice_model_type_accepts_contiguous_expressions(model_type)
                            && let Some(value) = try_xspice_model_scalar_expression(
                                stream,
                                params,
                                defer_expression_params,
                            )
                        {
                            push_model_scalar_expression_param(
                                &mut numeric_params,
                                &mut expr_params,
                                name,
                                value,
                            );
                        } else if stream.consume(&TokenKind::LParen) {
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
                        } else {
                            return Err(ParseError::Syntax {
                                line: line_num,
                                message: format!(
                                    "Expected value for model parameter '{}', found {}",
                                    name,
                                    stream.peek().kind
                                ),
                            });
                        }
                    }
                }
            } else if model_param_accepts_bare_string(&name, model_type) {
                match &stream.peek().kind {
                    TokenKind::StringLit(value) => {
                        let value = value.clone();
                        stream.advance();
                        push_model_string_value(
                            &mut string_params,
                            &mut string_vector_params,
                            &mut real_vector_params,
                            line_num,
                            &name,
                            &value,
                        )?;
                    }
                    kind if model_bare_string_token_can_start(kind) => {
                        let value = parse_model_bare_string_value(stream, line_num, &name)?;
                        push_model_string_value(
                            &mut string_params,
                            &mut string_vector_params,
                            &mut real_vector_params,
                            line_num,
                            &name,
                            &value,
                        )?;
                    }
                    _ => numeric_params.push((name, 1.0)),
                }
            } else {
                numeric_params.push((name, 1.0));
            }
        } else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "Unexpected token in .MODEL parameter list: {}",
                    stream.peek().kind
                ),
            });
        }
    }

    Ok(ParsedModelParams {
        numeric: numeric_params,
        expr: expr_params,
        string: string_params,
        string_vector: string_vector_params,
        real_vector: real_vector_params,
        real_vector_expr: real_vector_expr_params,
        integer_vector: integer_vector_params,
    })
}

enum BareModelIdentString {
    Literal(String),
    Deferred(String),
}

enum ParsedModelScalarExpression {
    Resolved(Value),
    Deferred(String),
}

enum ParsedModelComplexComponent {
    Resolved(Value),
    Deferred(String),
}

enum ParsedModelComplexLiteral {
    Resolved(String),
    Deferred { real: String, imag: String },
}

enum ParsedModelStringVector {
    Resolved(Vec<String>),
    Deferred(String),
}

fn push_model_scalar_expression_param(
    numeric_params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
    name: String,
    value: ParsedModelScalarExpression,
) {
    match value {
        ParsedModelScalarExpression::Resolved(value) => {
            numeric_params.push((name, value));
        }
        ParsedModelScalarExpression::Deferred(expr) => {
            expr_params.push((name, expr));
        }
    }
}

fn xspice_model_type_accepts_contiguous_expressions(model_type: Option<&str>) -> bool {
    model_type
        .map(crate::xspice::CodeModelRegistry::is_builtin_model_name)
        .unwrap_or(false)
}

fn try_xspice_model_scalar_expression(
    stream: &mut TokenStream,
    params: &ParamContext,
    defer_expression_params: bool,
) -> Option<ParsedModelScalarExpression> {
    let first = stream.peek().clone();
    if !model_scalar_expression_token_can_start(&first.kind) {
        return None;
    }

    let mut probe = stream.clone();
    let expr = collect_contiguous_expression(&mut probe)?;
    if !model_scalar_expression_is_compound(&first, &expr) {
        return None;
    }

    let expr = collect_contiguous_expression(stream)?;
    if let Ok(value) = crate::netlist::lexer::parse_spice_value(&expr) {
        return Some(ParsedModelScalarExpression::Resolved(value));
    }
    if let Some(value) = parse_boolean_literal(&expr) {
        return Some(ParsedModelScalarExpression::Resolved(value));
    }
    if defer_expression_params {
        return Some(ParsedModelScalarExpression::Deferred(expr));
    }

    match eval_expression(&expr, params) {
        Ok(value) => Some(ParsedModelScalarExpression::Resolved(value)),
        Err(_) => Some(ParsedModelScalarExpression::Deferred(expr)),
    }
}

fn model_scalar_expression_token_can_start(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Expression(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::LParen
    )
}

fn model_scalar_expression_is_compound(first: &Token, expr: &str) -> bool {
    model_scalar_expression_first_piece(first)
        .map(|piece| piece != expr)
        .unwrap_or(false)
}

fn model_scalar_expression_first_piece(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(value) => Some(value.clone()),
        TokenKind::Expression(expr) => Some(format!("({expr})")),
        TokenKind::Number(value) if token.lexeme.is_empty() => Some(format_compact_number(*value)),
        TokenKind::Number(_) | TokenKind::Plus | TokenKind::Minus | TokenKind::LParen => {
            Some(token.lexeme.clone())
        }
        _ => None,
    }
}

fn bare_model_ident_string_value(
    name: &str,
    value: &str,
    params: &ParamContext,
    defer_expression_params: bool,
    model_type: Option<&str>,
) -> Option<BareModelIdentString> {
    if model_param_accepts_bare_string(name, model_type) {
        if defer_expression_params && params.get_string(value).is_some() {
            return Some(BareModelIdentString::Deferred(value.to_string()));
        }
        if let Some(value) = params.get_string(value) {
            return Some(BareModelIdentString::Literal(value.to_string()));
        }
        return None;
    }

    if params.get(value).is_some()
        || parse_boolean_literal(value).is_some()
        || crate::netlist::lexer::parse_spice_value(value).is_ok()
    {
        return None;
    }

    if defer_expression_params && params.get_string(value).is_some() {
        return Some(BareModelIdentString::Deferred(value.to_string()));
    }

    if let Some(value) = params.get_string(value) {
        return Some(BareModelIdentString::Literal(value.to_string()));
    }

    None
}

fn parse_model_bare_string_value(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
) -> Result<String, ParseError> {
    let mut value = String::new();
    let mut previous_end = None;

    loop {
        let token = stream.peek().clone();
        match token.kind {
            TokenKind::Comma
            | TokenKind::RParen
            | TokenKind::RBracket
            | TokenKind::Newline
            | TokenKind::Eof => break,
            TokenKind::StringLit(_) if value.is_empty() => break,
            _ => {}
        }

        if let Some(end) = previous_end
            && token.span.start != end
        {
            break;
        }

        let Some(piece) = model_bare_string_piece_from_token(&token) else {
            break;
        };
        if piece.is_empty() {
            break;
        }

        previous_end = Some(token.span.end);
        value.push_str(&piece);
        stream.advance();
    }

    if value.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected string value for model parameter '{}', found {}",
                name,
                stream.peek().kind
            ),
        });
    }

    Ok(value)
}

fn model_bare_string_token_can_start(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Equals
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::AtSign
            | TokenKind::Tilde
            | TokenKind::Other(_)
    )
}

fn model_bare_string_piece_from_token(token: &Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(_)
        | TokenKind::Number(_)
        | TokenKind::Equals
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::AtSign
        | TokenKind::Tilde
        | TokenKind::Other(_) => Some(if token.lexeme.is_empty() {
            token.kind.to_string()
        } else {
            token.lexeme.clone()
        }),
        _ => None,
    }
}

fn model_param_accepts_bare_string(name: &str, model_type: Option<&str>) -> bool {
    crate::netlist::xspice_model_param_accepts_bare_string(name)
        || model_param_accepts_contextual_bare_string(name, model_type)
}

fn model_param_accepts_contextual_bare_string(name: &str, model_type: Option<&str>) -> bool {
    let Some(model_type) = model_type else {
        return false;
    };
    name.eq_ignore_ascii_case("model") && model_type.eq_ignore_ascii_case("multi_input_pwl")
        || name.eq_ignore_ascii_case("string")
            && model_type.eq_ignore_ascii_case("print_param_types")
}

fn push_model_string_value(
    string_params: &mut Vec<(String, String)>,
    string_vector_params: &mut Vec<(String, Vec<String>)>,
    real_vector_params: &mut Vec<(String, Vec<Value>)>,
    line_num: usize,
    name: &str,
    value: &str,
) -> Result<(), ParseError> {
    if value.trim_start().starts_with('[') {
        match parse_model_vector_string_literal(value, line_num, name)? {
            ModelVectorLiteral::Real(values) => real_vector_params.push((name.to_string(), values)),
            ModelVectorLiteral::String(values) => {
                string_vector_params.push((name.to_string(), values))
            }
        }
    } else {
        string_params.push((name.to_string(), value.to_string()));
    }
    Ok(())
}

pub(super) enum ModelVectorLiteral {
    Real(Vec<Value>),
    String(Vec<String>),
}

pub(super) fn parse_model_vector_string_literal(
    value: &str,
    line_num: usize,
    name: &str,
) -> Result<ModelVectorLiteral, ParseError> {
    let trimmed = value.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Model parameter vector '{}' from string parameter must be enclosed in '[' and ']'",
                name
            ),
        });
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let fields = split_spice_fields(inner);
    if fields.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Model parameter vector '{}' cannot be empty", name),
        });
    }

    let mut real_values = Vec::with_capacity(fields.len());
    let mut all_numeric = true;
    for field in &fields {
        match crate::netlist::lexer::parse_spice_value(field) {
            Ok(value) if value.is_finite() => real_values.push(value),
            _ => {
                all_numeric = false;
                break;
            }
        }
    }

    if all_numeric {
        return Ok(ModelVectorLiteral::Real(real_values));
    }

    Ok(ModelVectorLiteral::String(
        fields
            .into_iter()
            .map(|field| {
                strip_wrapping_string_literal(&field)
                    .unwrap_or(field.as_str())
                    .to_string()
            })
            .collect(),
    ))
}

fn model_vector_starts_with_string(
    stream: &TokenStream,
    params: &ParamContext,
    defer_expression_params: bool,
) -> bool {
    let mut offset = 1usize;
    loop {
        match &stream.peek_n(offset).kind {
            TokenKind::Comma => offset += 1,
            TokenKind::StringLit(_) => return true,
            TokenKind::Ident(_) => {
                return !token_can_start_model_real_vector_value(
                    &stream.peek_n(offset).kind,
                    params,
                    defer_expression_params,
                );
            }
            TokenKind::Plus | TokenKind::Minus => {
                return !token_can_start_model_real_vector_value(
                    &stream.peek_n(offset + 1).kind,
                    params,
                    defer_expression_params,
                );
            }
            _ => return false,
        }
    }
}

fn model_vector_starts_with_complex(stream: &TokenStream) -> bool {
    let mut offset = 1usize;
    loop {
        match &stream.peek_n(offset).kind {
            TokenKind::Comma => offset += 1,
            TokenKind::Other('<') => return true,
            _ => return false,
        }
    }
}

fn token_can_start_model_real_vector_value(
    kind: &TokenKind,
    params: &ParamContext,
    defer_expression_params: bool,
) -> bool {
    match kind {
        TokenKind::Number(_) | TokenKind::Expression(_) => true,
        TokenKind::Ident(s) => {
            defer_expression_params
                || params.get(s).is_some()
                || crate::netlist::lexer::parse_spice_value(s).is_ok()
        }
        _ => false,
    }
}

fn parse_model_string_vector(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
    params: &ParamContext,
    defer_expression_params: bool,
) -> Result<ParsedModelStringVector, ParseError> {
    if !stream.consume(&TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected '[' for model parameter vector '{}'", name),
        });
    }

    let mut values = Vec::new();
    let mut deferred_entries = Vec::new();
    loop {
        skip_commas(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance();
                if deferred_entries.is_empty() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Model parameter vector '{}' cannot be empty", name),
                    });
                }
                if deferred_entries.iter().any(|entry| {
                    matches!(
                        entry,
                        crate::netlist::DeferredXspiceStringVectorEntry::Complex { .. }
                    )
                }) {
                    return Ok(ParsedModelStringVector::Deferred(
                        crate::netlist::encode_deferred_xspice_complex_vector(&deferred_entries),
                    ));
                }
                return Ok(ParsedModelStringVector::Resolved(values));
            }
            TokenKind::StringLit(value) => {
                let value = value.clone();
                stream.advance();
                deferred_entries.push(crate::netlist::DeferredXspiceStringVectorEntry::Resolved(
                    value.clone(),
                ));
                values.push(value);
            }
            TokenKind::Other('<') => {
                match parse_model_complex_literal(
                    stream,
                    line_num,
                    name,
                    params,
                    defer_expression_params,
                )? {
                    ParsedModelComplexLiteral::Resolved(value) => {
                        deferred_entries.push(
                            crate::netlist::DeferredXspiceStringVectorEntry::Resolved(
                                value.clone(),
                            ),
                        );
                        values.push(value);
                    }
                    ParsedModelComplexLiteral::Deferred { real, imag } => {
                        deferred_entries.push(
                            crate::netlist::DeferredXspiceStringVectorEntry::Complex { real, imag },
                        );
                    }
                }
            }
            TokenKind::RParen | TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Model parameter vector '{}' is missing ']'", name),
                });
            }
            _ => {
                let value = parse_model_string_vector_bare_value(stream, line_num, name)?;
                deferred_entries.push(crate::netlist::DeferredXspiceStringVectorEntry::Resolved(
                    value.clone(),
                ));
                values.push(value);
            }
        }
    }
}

fn parse_model_string_vector_bare_value(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
) -> Result<String, ParseError> {
    let mut value = String::new();
    let mut previous_end = None;

    loop {
        let token = stream.peek().clone();
        match token.kind {
            TokenKind::Comma | TokenKind::RBracket | TokenKind::Newline | TokenKind::Eof => break,
            TokenKind::StringLit(_) if value.is_empty() => break,
            _ => {}
        }

        if let Some(end) = previous_end
            && token.span.start != end
        {
            break;
        }

        let piece = if token.lexeme.is_empty() {
            token.kind.to_string()
        } else {
            token.lexeme
        };
        if piece.is_empty() {
            break;
        }

        previous_end = Some(token.span.end);
        value.push_str(&piece);
        stream.advance();
    }

    if value.is_empty() {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected string value in model parameter vector '{}', found {}",
                name,
                stream.peek().kind
            ),
        });
    }

    Ok(value)
}

fn parse_model_complex_literal(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
    params: &ParamContext,
    defer_expression_params: bool,
) -> Result<ParsedModelComplexLiteral, ParseError> {
    if !matches!(stream.peek().kind, TokenKind::Other('<')) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected complex value for model parameter '{}', found {}",
                name,
                stream.peek().kind
            ),
        });
    }
    stream.advance();

    let real = parse_model_complex_component(
        stream,
        line_num,
        name,
        params,
        "real",
        defer_expression_params,
    )?;
    let imag = parse_model_complex_component(
        stream,
        line_num,
        name,
        params,
        "imaginary",
        defer_expression_params,
    )?;

    skip_commas(stream);
    if !stream.consume(&TokenKind::Other('>')) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected '>' after complex model parameter '{}', found {}",
                name,
                stream.peek().kind
            ),
        });
    }

    match (real, imag) {
        (
            ParsedModelComplexComponent::Resolved(real),
            ParsedModelComplexComponent::Resolved(imag),
        ) => Ok(ParsedModelComplexLiteral::Resolved(format!(
            "<{} {}>",
            format_compact_number(real),
            format_compact_number(imag)
        ))),
        (real, imag) => Ok(ParsedModelComplexLiteral::Deferred {
            real: model_complex_component_expr(real),
            imag: model_complex_component_expr(imag),
        }),
    }
}

fn parse_model_complex_component(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
    params: &ParamContext,
    component: &str,
    defer_expression_params: bool,
) -> Result<ParsedModelComplexComponent, ParseError> {
    skip_commas(stream);

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

    let expr =
        collect_model_complex_component_expression(stream).ok_or_else(|| ParseError::Syntax {
            line: line_num,
            message: format!(
                "Expected {component} part in complex model parameter '{}', found {}",
                name,
                stream.peek().kind
            ),
        })?;

    if let Ok(value) = crate::netlist::lexer::parse_spice_value(&expr) {
        return Ok(ParsedModelComplexComponent::Resolved(sign * value));
    }
    if let Some(value) = parse_boolean_literal(&expr) {
        return Ok(ParsedModelComplexComponent::Resolved(sign * value));
    }
    if defer_expression_params {
        return Ok(ParsedModelComplexComponent::Deferred(signed_model_expr(
            sign, expr,
        )));
    }

    let value = eval_expression(&expr, params).map_err(|err| {
        ParseError::InvalidValue(format!(
            "line {}: complex model parameter '{}' {} expression '{}' could not be resolved: {}",
            line_num, name, component, expr, err
        ))
    })?;
    Ok(ParsedModelComplexComponent::Resolved(sign * value))
}

fn model_complex_component_expr(component: ParsedModelComplexComponent) -> String {
    match component {
        ParsedModelComplexComponent::Resolved(value) => format_compact_number(value),
        ParsedModelComplexComponent::Deferred(expr) => expr,
    }
}

fn signed_model_expr(sign: Value, expr: String) -> String {
    if sign < 0.0 {
        format!("-({expr})")
    } else {
        expr
    }
}

fn collect_model_complex_component_expression(stream: &mut TokenStream) -> Option<String> {
    let mut pieces = Vec::new();
    let mut previous_end = None;
    let mut paren_depth = 0usize;
    let mut offset = 0usize;

    loop {
        let token = stream.peek_n(offset);
        if let Some(end) = previous_end
            && token.span.start != end
        {
            break;
        }

        let piece = match &token.kind {
            TokenKind::Comma | TokenKind::Newline | TokenKind::Eof if paren_depth == 0 => break,
            TokenKind::Other('>') if paren_depth == 0 => break,
            TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Expression(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Other(_) => model_complex_component_piece(token),
            TokenKind::LParen => {
                paren_depth += 1;
                model_complex_component_piece(token)
            }
            TokenKind::RParen if paren_depth > 0 => {
                paren_depth -= 1;
                model_complex_component_piece(token)
            }
            _ => break,
        };

        if piece.is_empty() {
            break;
        }
        previous_end = Some(token.span.end);
        pieces.push(piece);
        offset += 1;
    }

    if pieces.is_empty() || paren_depth != 0 {
        return None;
    }

    for _ in 0..offset {
        stream.advance();
    }
    Some(pieces.join(""))
}

fn model_complex_component_piece(token: &Token) -> String {
    match &token.kind {
        TokenKind::Expression(expr) => format!("({expr})"),
        TokenKind::Number(value) if token.lexeme.is_empty() => format_compact_number(*value),
        _ if !token.lexeme.is_empty() => token.lexeme.clone(),
        _ => token.kind.to_string(),
    }
}

enum ParsedModelRealVector {
    Resolved(Vec<Value>),
    Deferred(Vec<String>),
}

enum ParsedModelRealVectorEntry {
    Resolved(Value),
    Deferred(String),
}

fn parse_model_real_vector(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
    params: &ParamContext,
    defer_expression_params: bool,
) -> Result<ParsedModelRealVector, ParseError> {
    if !stream.consume(&TokenKind::LBracket) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected '[' for model parameter vector '{}'", name),
        });
    }

    let mut entries = Vec::new();
    loop {
        skip_commas(stream);

        match &stream.peek().kind {
            TokenKind::RBracket => {
                stream.advance();
                if entries.is_empty() {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: format!("Model parameter vector '{}' cannot be empty", name),
                    });
                }
                if entries
                    .iter()
                    .any(|entry| matches!(entry, ParsedModelRealVectorEntry::Deferred(_)))
                {
                    let exprs = entries
                        .into_iter()
                        .map(|entry| match entry {
                            ParsedModelRealVectorEntry::Resolved(value) => {
                                format_compact_number(value)
                            }
                            ParsedModelRealVectorEntry::Deferred(expr) => expr,
                        })
                        .collect();
                    return Ok(ParsedModelRealVector::Deferred(exprs));
                }
                let values = entries
                    .into_iter()
                    .map(|entry| match entry {
                        ParsedModelRealVectorEntry::Resolved(value) => value,
                        ParsedModelRealVectorEntry::Deferred(_) => unreachable!(),
                    })
                    .collect();
                return Ok(ParsedModelRealVector::Resolved(values));
            }
            TokenKind::RParen | TokenKind::Newline | TokenKind::Eof => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!("Model parameter vector '{}' is missing ']'", name),
                });
            }
            _ => {}
        }

        entries.push(parse_model_real_vector_entry(
            stream,
            line_num,
            name,
            params,
            defer_expression_params,
        )?);
    }
}

fn parse_model_real_vector_entry(
    stream: &mut TokenStream,
    line_num: usize,
    name: &str,
    params: &ParamContext,
    defer_expression_params: bool,
) -> Result<ParsedModelRealVectorEntry, ParseError> {
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

    let signed_expr = |expr: String| {
        if sign < 0.0 {
            format!("-({expr})")
        } else {
            expr
        }
    };

    let expr = collect_contiguous_expression(stream).ok_or_else(|| ParseError::Syntax {
        line: line_num,
        message: format!(
            "Expected numeric value in model parameter vector '{}', found {}",
            name,
            stream.peek().kind
        ),
    })?;

    let entry = if let Ok(value) = crate::netlist::lexer::parse_spice_value(&expr) {
        ParsedModelRealVectorEntry::Resolved(sign * value)
    } else if let Some(value) = parse_boolean_literal(&expr) {
        ParsedModelRealVectorEntry::Resolved(sign * value)
    } else if defer_expression_params {
        ParsedModelRealVectorEntry::Deferred(signed_expr(expr))
    } else {
        let value = eval_expression(&expr, params).map_err(|err| {
            ParseError::InvalidValue(format!(
                "line {}: model parameter vector '{}' expression '{}' could not be resolved: {}",
                line_num, name, expr, err
            ))
        })?;
        ParsedModelRealVectorEntry::Resolved(sign * value)
    };

    if let ParsedModelRealVectorEntry::Resolved(value) = &entry
        && !value.is_finite()
    {
        return Err(ParseError::InvalidValue(format!(
            "line {}: model parameter vector '{}' contains non-finite value {}",
            stream.line(),
            name,
            value
        )));
    }

    Ok(entry)
}

fn try_dotted_model_version(stream: &mut TokenStream) -> Option<String> {
    skip_commas(stream);

    let mut parts = Vec::new();
    match &stream.peek().kind {
        TokenKind::Number(value) if value.is_finite() => {
            parts.push(format_compact_number(*value));
        }
        _ => return None,
    }

    let mut token_count = 1usize;
    loop {
        let previous_span = stream.peek_n(token_count - 1).span;
        let token = stream.peek_n(token_count);
        if token.span.start != previous_span.end {
            break;
        }

        let TokenKind::Number(value) = &token.kind else {
            break;
        };
        let component =
            dotted_version_tail_component(*value, token.span.end.saturating_sub(token.span.start))?;
        parts.push(component);
        token_count += 1;
    }

    if parts.len() < 2 {
        return None;
    }

    for _ in 0..token_count {
        stream.advance();
    }
    Some(parts.join("."))
}

fn format_compact_number(value: f64) -> String {
    format!("{value}")
}

fn dotted_version_tail_component(value: f64, span_len: usize) -> Option<String> {
    if !value.is_finite() || !(0.0..1.0).contains(&value) || span_len < 2 {
        return None;
    }
    let digits = span_len - 1;
    let scale = 10_f64.powi(digits as i32);
    let scaled = (value * scale).round();
    if (value * scale - scaled).abs() > 1e-9 {
        return None;
    }
    Some(format!("{:0width$}", scaled as u64, width = digits))
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

    if let Some(node) = consume_node_label(stream) {
        return Ok(node);
    }

    let other = &stream.peek().kind;
    Err(ParseError::Syntax {
        line: line_num,
        message: format!("Expected node name, found {:?}", other),
    })
}

/// Consume one node inside a bracket-delimited field without treating the
/// closing bracket as part of an adjacent punctuation-rich node name.
pub(super) fn expect_node_before_rbracket(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    skip_commas(stream);

    if let Some(node) = consume_node_label_before_rbracket(stream) {
        return Ok(node);
    }

    let other = &stream.peek().kind;
    Err(ParseError::Syntax {
        line: line_num,
        message: format!("Expected node name before ']', found {:?}", other),
    })
}

fn consume_node_label(stream: &mut TokenStream) -> Option<String> {
    let mut node = node_label_piece(stream.peek())?;
    let mut end = stream.peek().span.end;
    stream.advance();

    while stream.peek().span.start == end {
        let Some(piece) = node_label_piece(stream.peek()) else {
            break;
        };
        node.push_str(&piece);
        end = stream.peek().span.end;
        stream.advance();
    }

    Some(node)
}

fn consume_node_label_before_rbracket(stream: &mut TokenStream) -> Option<String> {
    if matches!(stream.peek().kind, TokenKind::RBracket) {
        return None;
    }

    let mut node = node_label_piece(stream.peek())?;
    let mut end = stream.peek().span.end;
    stream.advance();

    while stream.peek().span.start == end && !matches!(stream.peek().kind, TokenKind::RBracket) {
        let Some(piece) = node_label_piece(stream.peek()) else {
            break;
        };
        node.push_str(&piece);
        end = stream.peek().span.end;
        stream.advance();
    }

    Some(node)
}

fn node_label_piece(token: &crate::netlist::lexer::Token) -> Option<String> {
    match &token.kind {
        TokenKind::Ident(s) => Some(s.clone()),
        TokenKind::Number(value) => Some(if token.lexeme.is_empty() {
            format_compact_number(*value)
        } else {
            token.lexeme.clone()
        }),
        _ => punctuation_node_name(token),
    }
}

fn punctuation_node_name(token: &crate::netlist::lexer::Token) -> Option<String> {
    let name = match token.kind {
        TokenKind::Plus => "+",
        TokenKind::Minus => "-",
        TokenKind::Slash => "/",
        TokenKind::AtSign => "@",
        TokenKind::Tilde => "~",
        TokenKind::LBracket => "[",
        TokenKind::RBracket => "]",
        TokenKind::Other(':')
        | TokenKind::Other('`')
        | TokenKind::Other('!')
        | TokenKind::Other('$')
        | TokenKind::Other('^')
        | TokenKind::Other('&')
        | TokenKind::Other('|')
        | TokenKind::Other('\\')
        | TokenKind::Other('<')
        | TokenKind::Other('>')
        | TokenKind::Other('?') => token.lexeme.as_str(),
        _ => return None,
    };
    Some(name.to_string())
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
    if let Some(sign) = sign
        && token_is_value_like(&stream.peek_n(1).kind, params)
    {
        stream.advance();
        if let Some(magnitude) = try_value_unsigned(stream, params) {
            return Some(sign * magnitude);
        }
    }
    try_value_unsigned(stream, params)
}

#[inline]
fn token_is_value_like(kind: &TokenKind, params: &ParamContext) -> bool {
    match kind {
        TokenKind::Number(_) | TokenKind::Expression(_) => true,
        TokenKind::Ident(s) => {
            params.get(s).is_some()
                || parse_boolean_literal(s).is_some()
                || crate::netlist::lexer::parse_spice_value(s).is_ok()
        }
        _ => false,
    }
}

fn parse_boolean_literal(raw: &str) -> Option<Value> {
    if raw.eq_ignore_ascii_case("true") {
        Some(1.0)
    } else if raw.eq_ignore_ascii_case("false") {
        Some(0.0)
    } else {
        None
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
            } else if let Some(v) = parse_boolean_literal(s) {
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
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
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
