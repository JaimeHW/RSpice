//! Scanning primitives shared by every keyword-form analysis card.
//!
//! One rule per field shape, parameterized by the [`AnalysisCard`] that owns
//! it, so `.PSS`, `.PNOISE` and `.DCMATCH` refuse a malformed field with the
//! same source-located [`AnalysisCardError`] and the same wording. A card
//! module contributes the field shapes only it defines.

use super::*;

/// Build the typed card failure for a card, line and issue.
pub(super) fn card_error(card: AnalysisCard, line: usize, issue: AnalysisCardIssue) -> ParseError {
    ParseError::AnalysisCard(Box::new(AnalysisCardError::new(card, line, issue)))
}

/// Whether the stream is positioned at the end of the card's logical line.
pub(super) fn at_card_end(stream: &TokenStream) -> bool {
    matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof)
}

/// Whether the next tokens spell `IDENT =`.
///
/// This single rule keeps the positional and keyword forms disjoint: a
/// positional field is never followed by `=`, and a keyword always is.
pub(super) fn at_keyword(stream: &TokenStream) -> bool {
    matches!(stream.peek().kind, TokenKind::Ident(_))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
}

/// Consume `KEYWORD =` and return the upper-cased keyword, or leave the
/// stream untouched when it is not positioned on a keyword pair.
pub(super) fn take_keyword(stream: &mut TokenStream) -> Option<String> {
    if !at_keyword(stream) {
        return None;
    }
    let TokenKind::Ident(name) = &stream.peek().kind else {
        return None;
    };
    let keyword = name.to_ascii_uppercase();
    stream.advance();
    stream.advance();
    Some(keyword)
}

/// Reject a keyword the card has already bound.
pub(super) fn bind_once<T>(
    slot: &mut Option<T>,
    value: T,
    card: AnalysisCard,
    line: usize,
    keyword: &'static str,
) -> Result<(), ParseError> {
    if slot.is_some() {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::DuplicateKeyword { keyword },
        ));
    }
    *slot = Some(value);
    Ok(())
}

/// Read a numeric field and check it against a predicate.
pub(super) fn card_number(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    expected: &'static str,
    admissible: impl Fn(Value) -> bool,
) -> Result<Value, ParseError> {
    let value = expect_value(stream, line, params)
        .map_err(|_| card_error(card, line, AnalysisCardIssue::MissingField { field }))?;
    if !value.is_finite() || !admissible(value) {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(value)
}

/// Read a non-negative integer-valued field.
pub(super) fn card_count(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    minimum: usize,
) -> Result<usize, ParseError> {
    let expected: &'static str = match minimum {
        1 => "a whole number >= 1",
        2 => "a whole number >= 2",
        16 => "a whole number >= 16",
        _ => "a whole number",
    };
    let value = card_number(stream, line, params, card, field, expected, |value| {
        value >= 0.0 && value.fract() == 0.0 && value <= usize::MAX as Value
    })?;
    let count = value as usize;
    if count < minimum {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(count)
}

/// Read a signed integer-valued field.
pub(super) fn card_signed(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
    minimum: i32,
) -> Result<i32, ParseError> {
    let expected: &'static str = if minimum >= 1 {
        "a whole number >= 1"
    } else {
        "a whole number"
    };
    let value = card_number(stream, line, params, card, field, expected, |value| {
        value.fract() == 0.0 && value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)
    })?;
    let signed = value as i32;
    if signed < minimum {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidNumber {
                field,
                value,
                expected,
            },
        ));
    }
    Ok(signed)
}

/// Read a boolean-valued field written as `TRUE|FALSE|YES|NO|1|0`.
pub(super) fn card_bool(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
    field: &'static str,
) -> Result<bool, ParseError> {
    let token = stream.peek().clone();
    let spelling = match &token.kind {
        TokenKind::Ident(name) => name.clone(),
        TokenKind::Number(value) if *value == 0.0 || *value == 1.0 => {
            stream.advance();
            return Ok(*value == 1.0);
        }
        _ => token.lexeme.clone(),
    };
    let decoded = match spelling.to_ascii_uppercase().as_str() {
        "TRUE" | "YES" | "ON" => Some(true),
        "FALSE" | "NO" | "OFF" => Some(false),
        _ => None,
    };
    match decoded {
        Some(value) => {
            stream.advance();
            Ok(value)
        }
        None => Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidChoice {
                field,
                value: spelling,
                expected: "TRUE, FALSE, YES, NO, 1 or 0",
            },
        )),
    }
}

/// Read a bare name field (node, source or integration method).
pub(super) fn card_name(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
    field: &'static str,
) -> Result<String, ParseError> {
    let name = expect_node(stream, line)
        .map_err(|_| card_error(card, line, AnalysisCardIssue::MissingField { field }))?;
    if name.trim().is_empty() {
        return Err(card_error(
            card,
            line,
            AnalysisCardIssue::InvalidName { field, value: name },
        ));
    }
    Ok(name)
}

/// Read an output probe written `V(node[,ref])` or as a bare node name.
pub(super) fn card_output_probe(
    stream: &mut TokenStream,
    line: usize,
    card: AnalysisCard,
) -> Result<(String, Option<String>), ParseError> {
    parse_voltage_output_reference(stream, line).map_err(|_| {
        card_error(
            card,
            line,
            AnalysisCardIssue::InvalidChoice {
                field: "OUT",
                value: stream.peek().lexeme.clone(),
                expected: "V(node), V(node,ref) or a bare node name",
            },
        )
    })
}

/// Read `n` or `n,n,...`, parenthesized or bare, each at least one.
///
/// A bare list only continues past a comma when a number follows it, so a
/// deck that separates its keywords with commas — which SPICE treats as
/// whitespace everywhere else — is read the way it was written.
pub(super) fn card_count_list(
    stream: &mut TokenStream,
    line: usize,
    params: &ParamContext,
    card: AnalysisCard,
    field: &'static str,
) -> Result<Vec<usize>, ParseError> {
    let mut counts = Vec::new();
    if stream.consume(&TokenKind::LParen) {
        loop {
            counts.push(card_count(stream, line, params, card, field, 1)?);
            if stream.consume(&TokenKind::RParen) {
                break;
            }
            if !stream.consume(&TokenKind::Comma) {
                return Err(card_error(
                    card,
                    line,
                    AnalysisCardIssue::TrailingToken {
                        token: stream.peek().lexeme.clone(),
                    },
                ));
            }
        }
    } else {
        counts.push(card_count(stream, line, params, card, field, 1)?);
        while matches!(stream.peek().kind, TokenKind::Comma)
            && matches!(
                stream.peek_n(1).kind,
                TokenKind::Number(_) | TokenKind::Expression(_)
            )
        {
            stream.advance();
            counts.push(card_count(stream, line, params, card, field, 1)?);
        }
    }
    Ok(counts)
}
