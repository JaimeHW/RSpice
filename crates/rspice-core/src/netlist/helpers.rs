//! Helper functions for SPICE netlist parsing
//!
//! Common utilities used across parser modules:
//! - Token stream navigation and consumption
//! - Value extraction (numbers, parameters, expressions)
//! - Error conversion

use super::expr::eval_expression;
use super::lexer::{LexError, TokenKind, TokenStream};
use super::{ParamContext, ParseError};
use crate::Value;

/// Expect and consume an identifier token
pub fn expect_ident(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
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

/// Expect and consume a node name (identifier or number)
pub fn expect_node(stream: &mut TokenStream, line_num: usize) -> Result<String, ParseError> {
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

/// Expect and consume a numeric value (number, expression, or parameter reference)
pub fn expect_value(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    skip_commas(stream);

    match &stream.peek().kind {
        TokenKind::Number(v) => {
            let v = *v;
            stream.advance();
            Ok(v)
        }
        TokenKind::Expression(expr) => {
            let expr = expr.clone();
            stream.advance();
            eval_expression(&expr, params).map_err(|e| ParseError::InvalidValue(e.to_string()))
        }
        TokenKind::Ident(s) => {
            // Could be a parameter reference
            if let Some(v) = params.get(s) {
                stream.advance();
                Ok(v)
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

/// Try to consume a value, returning None if not present
pub fn try_value(stream: &mut TokenStream, params: &ParamContext) -> Option<Value> {
    skip_commas(stream);

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
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Try to consume a value, returning a default if not present
#[inline]
pub fn expect_value_default(
    stream: &mut TokenStream,
    params: &ParamContext,
    default: Value,
) -> Value {
    skip_commas(stream);
    try_value(stream, params).unwrap_or(default)
}

/// Try to consume a named parameter (e.g., IC=value)
pub fn try_value_with_param(
    stream: &mut TokenStream,
    params: &ParamContext,
    param_name: &str,
) -> Option<Value> {
    skip_commas(stream);

    // Check if next token is the param name followed by =
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s == param_name {
            stream.advance();
            if stream.consume(&TokenKind::Equals) {
                return try_value(stream, params);
            }
        }
    }

    try_value(stream, params)
}

/// Skip an optional parameter name prefix (e.g., R= before value)
pub fn skip_optional_param_name(stream: &mut TokenStream, param_name: &str) {
    if let TokenKind::Ident(s) = &stream.peek().kind {
        if s == param_name {
            stream.advance();
            stream.consume(&TokenKind::Equals);
        }
    }
}

/// Skip any comma tokens
#[inline]
pub fn skip_commas(stream: &mut TokenStream) {
    while stream.consume(&TokenKind::Comma) {}
}

/// Convert lexer error to parse error
pub fn lex_to_parse_error(e: LexError, line_num: usize) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: e.to_string(),
    }
}
