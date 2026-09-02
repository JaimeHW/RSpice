//! Token, value, expression, and model-parameter helpers.

use super::*;
use crate::netlist::lexer::{Token, collect_contiguous_expression};

/// Temperature- and thermal-voltage-dependent model expressions must remain
/// symbolic.  Evaluating them against the parser's default 27 C context
/// would freeze the device parameter before an analysis starts.
fn model_expression_references_temperature(expression: &str) -> bool {
    let bytes = expression.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            let identifier = expression[start..index].to_ascii_uppercase();
            if matches!(identifier.as_str(), "TEMP" | "TEMPER" | "VT") {
                return true;
            }
        } else {
            index += 1;
        }
    }
    false
}

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

/// Rejoin `name = value` assignments that whitespace split across fields.
///
/// Every SPICE dialect accepts spaces around `=` in an instance parameter
/// list, and foundry PDKs use the spaced form freely — GF180MCU's device
/// netlists are written `xmn1 d g 0 0 nmos_3p3 W = 10u L = 0.28u`. A parser
/// that scans fields for one containing `=` sees `W`, `=`, `10u` as three
/// fields, finds its first `=` one field too late, and concludes the
/// subcircuit is named `W`.
///
/// Operates on already-split fields rather than raw text so quoting and
/// brace nesting have already been honoured: `{a = b}` arrives as one field
/// and is left alone, where a character-level rewrite would reach inside it.
///
/// Applied by callers that want it rather than folded into
/// [`split_spice_fields`], because a bare `=` is meaningful to some callers
/// and silently absorbing it everywhere would trade this bug for a subtler
/// one.
pub(super) fn coalesce_assignment_fields(fields: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(fields.len());
    let mut pending_value = false;

    for field in fields {
        // `name =` waiting for its value, or a lone `=` already joined onto
        // the name: the next field completes the assignment.
        if pending_value {
            if let Some(last) = out.last_mut() {
                last.push_str(&field);
            }
            pending_value = false;
            continue;
        }

        if field == "=" {
            // A lone `=` with no name before it is not an assignment; leave
            // it for the caller to reject with its own diagnostic.
            if out.is_empty() {
                out.push(field);
                continue;
            }
            if let Some(last) = out.last_mut() {
                last.push('=');
            }
            pending_value = true;
            continue;
        }

        if let Some(rest) = field.strip_prefix('=')
            && !out.is_empty()
        {
            if let Some(last) = out.last_mut() {
                last.push('=');
                last.push_str(rest);
            }
            // `=value` completes the assignment unless it was a bare `=`,
            // which the branch above already handled.
            continue;
        }

        if field.ends_with('=') && field.len() > 1 {
            out.push(field);
            pending_value = true;
            continue;
        }

        out.push(field);
    }

    out
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
    /// Canonical parameter names in the order they appeared on the model
    /// card.  The typed value stores below deliberately optimize later
    /// resolution by representation, so they cannot reproduce Xyce's
    /// user-facing diagnostic order on their own.
    pub(super) authored_parameter_order: Vec<String>,
    pub(super) numeric: Vec<(String, Value)>,
    pub(super) expr: Vec<(String, String)>,
    /// Parameters whose value was a bare identifier this pass could not
    /// resolve, with the line that wrote them.
    ///
    /// Tracked separately from [`expr`](Self::expr) because the two carry
    /// different guarantees. A braced `{missing}` is an explicit expression
    /// whose resolution is somebody else's job — the XSPICE resolver rejects
    /// its own. A bare `noia = nmos_3p3_noia` used to be a parse error, and
    /// still has to become one if nothing in the finished deck defines the
    /// name; otherwise the model quietly takes its default. Keeping the list
    /// means end-of-parse validation can restore that error, with its line,
    /// without guessing from expression text which deferrals were which.
    pub(super) bare_ident_deferrals: Vec<(String, String, usize)>,
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
    model_name: &str,
    origin: &NetlistSourceLocation,
) -> Result<ParsedModelParams, ParseError> {
    let mut numeric_params = Vec::new();
    let mut expr_params = Vec::new();
    let mut string_params = Vec::new();
    let mut string_vector_params = Vec::new();
    let mut real_vector_params = Vec::new();
    let mut real_vector_expr_params = Vec::new();
    let integer_vector_params = Vec::new();
    let mut bare_ident_deferrals: Vec<(String, String, usize)> = Vec::new();
    let mut authored_names = Vec::new();

    let opened_paren = stream.consume(&TokenKind::LParen);
    let allow_missing_close = opened_paren
        && model_type
            .map(crate::codemodels::is_builtin_model_name)
            .unwrap_or(false);

    loop {
        skip_commas(stream);

        match &stream.peek().kind {
            TokenKind::RParen if opened_paren => {
                stream.advance();
                break;
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
                        break;
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
                    break;
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
            let authored_name = stream.peek().lexeme.clone();
            let name = name.clone();
            authored_names.push((authored_name, name.clone()));
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
                    // Numeric tokens are values even when the parameter name
                    // also admits a bare string form (for example XSPICE
                    // `*_PATH` parameters).  A numeric filename is preserved
                    // only for the explicitly string-valued table payload;
                    // ordinary model geometry such as CORE `PATH=8.49` must
                    // remain numeric instead of silently moving to
                    // `string_params`.
                    TokenKind::Number(_)
                        if crate::netlist::xspice_param_preserves_numeric_string(&name) =>
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
                        } else if params.expression_references_spectre_statistics(&value) {
                            stream.advance();
                            expr_params.push((name, value));
                        } else if let Some(value) = try_signed_model_value(stream, params) {
                            numeric_params.push((name, value));
                        } else {
                            // A bare identifier that does not resolve *here*
                            // is a forward parameter reference. It gets the
                            // same deferral a compound expression already
                            // receives one branch below — `noia = 'k*1'`
                            // deferring while `noia = k` hard-errored was an
                            // inconsistency, not a safety property.
                            //
                            // Forward references are ordinary in foundry
                            // corner libraries: GF180MCU's `.LIB typical`
                            // pulls in the MOS model cards before the
                            // `.LIB noise_corner` section defining the
                            // flicker-noise parameters they cite, so a
                            // single forward pass cannot have seen them yet.
                            //
                            // `resolve_static_model_expression_params` folds
                            // these back into numeric parameters once the
                            // whole deck has been read; anything still
                            // unresolved then is either genuinely
                            // temperature-dependent (correctly deferred to
                            // the builder) or a real typo, which the builder
                            // reports naming the model and parameter.
                            let TokenKind::Ident(reference) = &stream.peek().kind else {
                                unreachable!("ident branch matched before fallback")
                            };
                            let reference = reference.clone();
                            stream.advance();
                            bare_ident_deferrals.push((name.clone(), reference.clone(), line_num));
                            expr_params.push((name, reference));
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
                            if defer_expression_params
                                || model_expression_references_temperature(&expr)
                                || params.expression_references_spectre_statistics(&expr)
                            {
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
                        && model_bare_string_token_can_start(kind)
                        && !matches!(kind, TokenKind::Number(_)) =>
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
            } else if let Some(value) = try_signed_model_value(stream, params) {
                // Xyce and SPICE-compatible model cards also permit the
                // positional `NAME VALUE` form (for example, `BF 20`).
                // Consume only a value-like token here so bare model flags
                // retain their established numeric-one representation.
                numeric_params.push((name, value));
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

    let mut unique_names = HashSet::with_capacity(authored_names.len());
    for (authored_name, parsed_name) in &authored_names {
        let canonical_name = parsed_name.to_ascii_uppercase();
        if canonical_name == "LEVEL" {
            continue;
        }
        if !unique_names.insert(canonical_name.clone())
            && params.expression_dialect() == ExpressionDialect::Xyce
        {
            return Err(ParseError::DuplicateModelParameter(Box::new(
                DuplicateModelParameterError {
                    model_name: model_name.to_string(),
                    canonical_model_name: model_name.to_ascii_uppercase(),
                    parameter_name: authored_name.clone(),
                    canonical_parameter_name: canonical_name,
                    model_origin: origin.clone(),
                },
            )));
        }
    }

    Ok(ParsedModelParams {
        authored_parameter_order: authored_names
            .into_iter()
            .map(|(_, parsed_name)| parsed_name.to_ascii_uppercase())
            .collect(),
        numeric: numeric_params,
        expr: expr_params,
        string: string_params,
        string_vector: string_vector_params,
        real_vector: real_vector_params,
        real_vector_expr: real_vector_expr_params,
        integer_vector: integer_vector_params,
        bare_ident_deferrals,
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
        .map(crate::codemodels::is_builtin_model_name)
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
    if defer_expression_params
        || model_expression_references_temperature(&expr)
        || params.expression_references_spectre_statistics(&expr)
    {
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
        || model_param_is_manufacturer_annotation(name)
        || model_param_accepts_contextual_bare_string(name, model_type)
}

/// `MFG=` names the part's manufacturer and carries no electrical meaning.
///
/// PSpice and LTspice both document it, and discrete libraries annotate device
/// cards of every type with it — `MFG=VISHAY`, `MFG=Linear_Systems`. It is kept
/// as a string parameter rather than discarded so the part's origin survives
/// into the model record.
fn model_param_is_manufacturer_annotation(name: &str) -> bool {
    name.eq_ignore_ascii_case("mfg")
}

fn model_param_accepts_contextual_bare_string(name: &str, model_type: Option<&str>) -> bool {
    let Some(model_type) = model_type else {
        return false;
    };
    name.eq_ignore_ascii_case("model") && model_type.eq_ignore_ascii_case("multi_input_pwl")
        || name.eq_ignore_ascii_case("string")
            && model_type.eq_ignore_ascii_case("print_param_types")
        || (name.eq_ignore_ascii_case("fxpdata") || name.eq_ignore_ascii_case("fxmdata"))
            && model_type.eq_ignore_ascii_case("memristor")
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

    let first = stream.peek().clone();
    let TokenKind::Ident(head) = &first.kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", first.kind),
        });
    };

    // Xyce permits punctuation-rich model names (for example
    // `TX39-20-13_3C80_25C`).  The lexer keeps `-` and digit fragments
    // separate because they are operators/numeric tokens elsewhere; when
    // those fragments are contiguous in the source they are one identifier
    // at this grammar boundary.
    let mut name = head.clone();
    let mut end = first.span.end;
    stream.advance();
    while stream.peek().span.start == end {
        let token = stream.peek().clone();
        let Some(fragment) = xyce_dev_name_fragment(&token) else {
            break;
        };
        name.push_str(&fragment);
        end = token.span.end;
        stream.advance();
    }
    if !name.chars().all(is_xyce_dev_name_char) {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!(
                "Invalid identifier '{}': contains unsupported punctuation",
                name
            ),
        });
    }
    Ok(name)
}

/// Consume an element-head name using Xyce's contiguous DEV token grammar.
///
/// The lexer intentionally keeps punctuation as separate tokens because those
/// characters are operators or delimiters in other parser contexts.  Xyce
/// nevertheless permits them inside a device name (for example `R+`), so the
/// element head is the one place where adjacent DEV fragments are reassembled.
/// Whitespace remains a hard boundary: a punctuation token after a separated
/// identifier is the next field, not part of the name.
pub(super) fn expect_element_name(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    skip_commas(stream);

    let first = stream.peek().clone();
    let TokenKind::Ident(head) = &first.kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected element name, found {:?}", first.kind),
        });
    };
    if !head
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic())
        || !head.chars().all(is_xyce_dev_name_char)
    {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Invalid Xyce element name '{}'", first.lexeme),
        });
    }

    let mut name = head.clone();
    let mut end = first.span.end;
    stream.advance();
    while stream.peek().span.start == end {
        let token = stream.peek().clone();
        let Some(fragment) = xyce_dev_name_fragment(&token) else {
            break;
        };
        name.push_str(&fragment);
        end = token.span.end;
        stream.advance();
    }

    Ok(name)
}

fn xyce_dev_name_fragment(token: &Token) -> Option<String> {
    let fragment = match &token.kind {
        TokenKind::Ident(value) => value.clone(),
        TokenKind::Number(_) => token.lexeme.to_ascii_uppercase(),
        TokenKind::Plus => "+".to_string(),
        TokenKind::Minus => "-".to_string(),
        TokenKind::Star => "*".to_string(),
        TokenKind::Slash => "/".to_string(),
        TokenKind::AtSign => "@".to_string(),
        TokenKind::Tilde => "~".to_string(),
        TokenKind::LBracket => "[".to_string(),
        TokenKind::RBracket => "]".to_string(),
        TokenKind::Other(character) => character.to_string(),
        _ => return None,
    };
    fragment
        .chars()
        .all(is_xyce_dev_name_char)
        .then_some(fragment)
}

fn is_xyce_dev_name_char(character: char) -> bool {
    crate::netlist::lexer::is_xyce_device_name_char(character)
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

/// Consume a model name, at a `.model` card or at an instance's model field.
///
/// Model names are far more permissive than identifiers, and real libraries
/// lean on that. Digit-leading names (`1N4148`, `2N2222`, `74HC00`),
/// manufacturer suffixes joined by a hyphen (`2N3819-VSH`) and slash-qualified
/// names (`BC547A/PLP`, `LM741/NS`) are all commonplace, and all are accepted
/// by ngspice. The lexer splits those across several tokens because the same
/// characters are operators elsewhere, so the name is reassembled with the
/// node-label rule: pieces join only where they touch in the source. Whitespace
/// stays a hard boundary, which keeps `.model FOO D` from gluing a name to its
/// type, and the scan still stops cleanly at `(` or `=`.
pub(super) fn expect_model_name(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    skip_commas(stream);

    if let Some(name) = consume_node_label(stream) {
        return Ok(name);
    }

    let other = &stream.peek().kind;
    Err(ParseError::Syntax {
        line: line_num,
        message: format!("Expected model name, found {:?}", other),
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
        TokenKind::Star => "*",
        TokenKind::Slash => "/",
        TokenKind::AtSign => "@",
        TokenKind::Tilde => "~",
        TokenKind::LBracket => "[",
        TokenKind::RBracket => "]",
        TokenKind::Other('#')
        | TokenKind::Other('%')
        | TokenKind::Other('.')
        | TokenKind::Other(':')
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
            } else if let Some(v) = parse_boolean_literal(s) {
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

/// Recombine a `Minus`/`Plus` sign token that the lexer split off from a
/// magnitude written without a leading zero (e.g. `-.14`, `+.5`).
pub(super) fn try_signed_value(stream: &mut TokenStream, params: &ParamContext) -> Option<Value> {
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

/// Signed model-card values use the same token recombination as command-level
/// value lists; retain this name at model call sites to document that context.
pub(super) fn try_signed_model_value(
    stream: &mut TokenStream,
    params: &ParamContext,
) -> Option<Value> {
    try_signed_value(stream, params)
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
    if !defer {
        let expression = match (&stream.peek().kind, &stream.peek_n(1).kind) {
            (TokenKind::Expression(expr), _) => Some(expr.clone()),
            (TokenKind::Plus, TokenKind::Expression(expr)) => Some(expr.clone()),
            (TokenKind::Minus, TokenKind::Expression(expr)) => Some(format!("-({expr})")),
            _ => None,
        };
        if let Some(expression) = expression
            && eval_expression(&expression, params).is_err()
            && let Ok(prepared) =
                super::super::expr::prepare_behavioral_expression(&expression, params)
        {
            let consumed = take_value_expression_string(stream, params)?;
            return Some(match eval_expression(&prepared, params) {
                Ok(value) => DeferrableValue::Resolved(value),
                Err(_) => DeferrableValue::Deferred(consumed),
            });
        }
    }
    if let Some(value) = take_contiguous_instance_expression(stream, params, defer) {
        return Some(value);
    }
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

/// Take an unbraced arithmetic expression from an instance parameter.
///
/// Foundry PDK subcircuits write instance geometry as plain arithmetic over the
/// subcircuit's own parameters — IHP SG13G2's diode subcircuits carry
/// `area=mf*aws pj=mf*pws`, and the same shape appears on MOS `w`/`l` across
/// sky130 and GF180MCU. ngspice evaluates these through numparam without
/// requiring braces, so `{...}` cannot be the only accepted spelling.
///
/// The scan only claims a token run that is genuinely *compound*, reusing the
/// same guards as the `.model` card path. A lone value or a bare parameter
/// reference is left to the caller, which keeps engineering suffixes (`1u`) and
/// deferred single-identifier references on their existing paths.
fn take_contiguous_instance_expression(
    stream: &mut TokenStream,
    params: &ParamContext,
    defer: bool,
) -> Option<DeferrableValue> {
    let first = stream.peek().clone();
    if !model_scalar_expression_token_can_start(&first.kind) {
        return None;
    }

    let mut probe = stream.clone();
    let probed = collect_contiguous_expression(&mut probe)?;
    if !model_scalar_expression_is_compound(&first, &probed) {
        return None;
    }

    let expr = collect_contiguous_expression(stream)?;
    // A signed literal (`ic=-5`) reads as compound but is still just a number;
    // resolving it here keeps it off the deferred path it never used before.
    // This boundary must require complete consumption: the compatibility
    // parser intentionally accepts numeric prefixes, which would turn
    // `pd=2*(length+width)` into the literal `2` and discard the arithmetic.
    if let Ok(value) = crate::netlist::lexer::parse_spice_value_complete(&expr) {
        return Some(DeferrableValue::Resolved(value));
    }
    if defer {
        return Some(DeferrableValue::Deferred(expr));
    }
    Some(match eval_expression(&expr, params) {
        Ok(value) => DeferrableValue::Resolved(value),
        Err(_) => DeferrableValue::Deferred(expr),
    })
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

#[cfg(test)]
mod tests {
    use crate::Netlist;
    use crate::netlist::expr::ParamContext;
    use crate::netlist::lexer::{TokenKind, TokenStream, tokenize};

    fn coalesce(line: &str) -> Vec<String> {
        super::coalesce_assignment_fields(super::split_spice_fields(line))
    }

    #[test]
    fn instance_values_distinguish_complete_numbers_from_numeric_prefixed_expressions() {
        let mut params = ParamContext::new();
        params.set("X", 4.0);
        let cases = [
            ("2*X", 2.0_f64 * 4.0),
            ("2/X", 2.0 / 4.0),
            ("2+X", 2.0 + 4.0),
            ("2-X", 2.0 - 4.0),
            (".5*X", 0.5 * 4.0),
            ("2u*X", 2.0e-6 * 4.0),
            ("2*(X+1)", 2.0 * (4.0 + 1.0)),
            ("2", 2.0),
            ("-5", -5.0),
            ("1u", 1.0e-6),
        ];

        for (source, expected) in cases {
            let tokens = tokenize(&format!("{source} NEXT=99\n")).expect("tokenize value");
            let mut stream = TokenStream::new(tokens);
            let parsed = super::take_deferrable_value(&mut stream, &params, false)
                .unwrap_or_else(|| panic!("{source} did not parse as an instance value"));
            let super::DeferrableValue::Resolved(actual) = parsed else {
                panic!("{source} unexpectedly remained deferred");
            };
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "instance value {source} was truncated or mis-evaluated as {actual}"
            );
            assert_eq!(
                stream.peek().kind,
                TokenKind::Ident("NEXT".to_string()),
                "instance value {source} consumed the following assignment"
            );
        }
    }

    #[test]
    fn assignment_fields_rejoin_across_every_spacing_of_equals() {
        for line in [
            "xmn1 d g 0 0 nmos_3p3 W = 10u L = 0.28u",
            "xmn1 d g 0 0 nmos_3p3 W= 10u L= 0.28u",
            "xmn1 d g 0 0 nmos_3p3 W =10u L =0.28u",
            "xmn1 d g 0 0 nmos_3p3 W=10u L=0.28u",
        ] {
            assert_eq!(
                coalesce(line),
                vec!["xmn1", "d", "g", "0", "0", "nmos_3p3", "W=10u", "L=0.28u"],
                "{line}"
            );
        }
    }

    /// The node list must survive: a rejoin that swallowed a field would move
    /// the subcircuit name and silently reconnect the instance.
    #[test]
    fn assignment_coalescing_leaves_positional_fields_alone() {
        assert_eq!(coalesce("x1 a b c sub"), vec!["x1", "a", "b", "c", "sub"]);
    }

    /// Braced and quoted values arrive pre-grouped from `split_spice_fields`,
    /// so an `=` inside one is not an assignment separator.
    #[test]
    fn assignment_coalescing_does_not_reach_inside_grouped_values() {
        assert_eq!(
            coalesce("x1 a b sub W = {wn = 2} L = 'lmin'"),
            vec!["x1", "a", "b", "sub", "W={wn = 2}", "L='lmin'"]
        );
    }

    /// A leading `=` has no name to attach to; leave it for the caller to
    /// reject rather than inventing an assignment.
    #[test]
    fn leading_equals_is_left_for_the_caller_to_diagnose() {
        assert_eq!(coalesce("= 5"), vec!["=", "5"]);
    }

    #[test]
    fn subcircuit_instance_reads_the_subckt_name_past_spaced_parameters() {
        let netlist = Netlist::parse(
            "spaced instance parameters\n\
             .subckt myamp d g s b\n\
             R1 d s 1k\n\
             .ends\n\
             X1 nd ng 0 0 myamp W = 10u L = 0.28u\n\
             .end\n",
        )
        .expect("spaced instance parameters parse");

        let instance = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("X1"))
            .expect("X1 was parsed");
        let crate::netlist::ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &instance.kind
        else {
            panic!("X1 is a subcircuit instance, got {:?}", instance.kind);
        };

        assert!(subckt_name.eq_ignore_ascii_case("myamp"), "{subckt_name}");
        assert_eq!(instance.nodes, vec!["ND", "NG", "0", "0"]);
        assert_eq!(params.len(), 2, "{params:?}");
        assert!(
            params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("W"))
        );
        assert!(
            params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("L"))
        );
    }

    #[test]
    fn model_accepts_positional_numeric_parameters_and_bare_flags() {
        let netlist = Netlist::parse(
            "positional model parameters\n\
             .model Q1 NPN BF 20 RB 100 TF .1NS CJC 2PF FLAG\n\
             .end\n",
        )
        .expect("positional model values parse");
        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("Q1"))
            .expect("Q1 model was parsed");

        let value = |name: &str| {
            model
                .params
                .iter()
                .find(|(parameter, _)| parameter.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value)
                .unwrap_or_else(|| panic!("missing model parameter {name}"))
        };

        assert_eq!(value("BF"), 20.0);
        assert_eq!(value("RB"), 100.0);
        assert!((value("TF") - 0.1e-9).abs() < 1e-20);
        assert!((value("CJC") - 2e-12).abs() < 1e-20);
        assert_eq!(value("FLAG"), 1.0);
    }
}
