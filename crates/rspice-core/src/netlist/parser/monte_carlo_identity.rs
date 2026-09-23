//! Source identity for exact Monte Carlo trial reuse.
//!
//! Only the real command parser identifies report-only fields. A source card
//! must still match its recorded owner, physical line and complete logical text
//! before its literal report fields can be omitted. Unmatched/edited source,
//! included files, disabled branches and control scripts remain identity-bound.

use super::{ExpressionDialect, Netlist, NetlistSourceLocation, TokenKind, TokenStream, tokenize};
use std::ops::Range;

#[derive(Debug, Clone)]
pub(crate) struct SourceCard {
    origin: NetlistSourceLocation,
    logical_line: String,
    identity: [u8; 32],
}

impl SourceCard {
    pub(super) fn new(
        origin: &NetlistSourceLocation,
        line: &str,
        reports: &[Range<usize>],
    ) -> Self {
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice-mc-source-card-v1\0");
        for token in tokenize(line).expect("successfully parsed Monte Carlo card") {
            if !matches!(token.kind, TokenKind::Newline | TokenKind::Eof)
                && !reports
                    .iter()
                    .any(|span| span.start <= token.span.start && token.span.end <= span.end)
            {
                field(&mut hash, token.lexeme.as_bytes());
            }
        }
        Self {
            origin: origin.clone(),
            logical_line: line.into(),
            identity: *hash.finalize().as_bytes(),
        }
    }
}

// Expressions deliberately remain part of source identity. Evaluating a
// statistical expression in a reporting field can consume random draws and
// thereby affect later device/model values, even at the same nominal value.
pub(super) fn literal_span(stream: &TokenStream, start: usize) -> Option<Range<usize>> {
    matches!(stream.peek().kind, TokenKind::Number(_)).then(|| start..stream.peek_n(1).span.start)
}

pub(super) fn retain_literal_span(
    reports: &mut Vec<Range<usize>>,
    span: Option<Range<usize>>,
    stream: &TokenStream,
) {
    if let Some(span) = span
        && span.end == stream.peek().span.start
    {
        reports.push(span);
    }
}

fn field(hash: &mut blake3::Hasher, bytes: &[u8]) {
    hash.update(&(bytes.len() as u64).to_le_bytes());
    hash.update(bytes);
}

/// Authenticate all original source except parser-identified literal batch and
/// confidence fields. This does not rewrite source used by statistical replay.
/// Physical assembly reuses the parser's dialect/comment rules and must match
/// the exact logical card it accepted; otherwise the original bytes are hashed.
pub(crate) fn source_identity(netlist: &Netlist) -> [u8; 32] {
    let source = netlist.source_text.as_deref().unwrap_or("");
    let mut hash = blake3::Hasher::new();
    hash.update(b"rspice-mc-root-source-v1\0");
    let mut cards = netlist
        .monte_carlo_source_cards
        .iter()
        .filter(|card| card.origin.path == netlist.source_path)
        .map(|card| (card.origin.line, card))
        .collect::<std::collections::BTreeMap<_, _>>();
    if cards.is_empty() {
        field(&mut hash, source.as_bytes());
        return *hash.finalize().as_bytes();
    }
    let xyce = netlist.params.expression_dialect() == ExpressionDialect::Xyce;
    let mut lines = source.split_inclusive('\n').enumerate().peekable();
    let mut offset = 0;
    let mut unhashed = 0;
    while let Some((index, physical)) = lines.next() {
        let start = offset;
        offset += physical.len();
        let Some(card) = cards.remove(&(index + 1)) else {
            continue;
        };
        let Some(first) = meaningful(physical, xyce) else {
            continue;
        };
        let mut logical = first.to_owned();
        let mut end = offset;
        // Look ahead without consuming the next independent physical card.
        let following = lines.clone();
        let mut next_offset = offset;
        for (_, line) in following {
            next_offset += line.len();
            let Some(line) = meaningful(line, xyce) else {
                continue;
            };
            let Some(rest) = line.strip_prefix('+') else {
                break;
            };
            logical.push(' ');
            logical.push_str(rest);
            end = next_offset;
        }
        if logical != card.logical_line {
            continue;
        }
        field(&mut hash, &source.as_bytes()[unhashed..start]);
        field(&mut hash, &card.identity);
        unhashed = end;
        while offset < end {
            let (_, line) = lines.next().expect("validated physical continuation");
            offset += line.len();
        }
    }
    field(&mut hash, &source.as_bytes()[unhashed..]);
    *hash.finalize().as_bytes()
}

fn meaningful(line: &str, xyce: bool) -> Option<&str> {
    let line = line.trim_end_matches(['\r', '\n']);
    if xyce && super::xyce_physical_line_is_comment(line) {
        return None;
    }
    let line =
        super::line::strip_inline_semicolon_comment_with_non_semicolon_comments(line, !xyce).trim();
    (!line.is_empty() && !line.starts_with('*')).then_some(line)
}
