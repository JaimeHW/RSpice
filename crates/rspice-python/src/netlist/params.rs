//! Rewriting `.PARAM` values in deck source.
//!
//! `Netlist.with_parameters()` derives a variant deck by rewriting top-level
//! assignments and re-parsing, rather than mutating a parsed tree. Re-parsing
//! is what keeps a derived netlist identical to one written by hand.
//!
//! Subcircuit-scoped definitions are deliberately left alone: rewriting them
//! would silently change every instance of that subcircuit, which is almost
//! never what a caller sweeping a top-level parameter intends.

use super::*;

/// Rewrite top-level `.PARAM` assignments in a deck.
///
/// Assignments are replaced in place so the result is independent of the
/// parameter-redefinition policy in force, which appending a second
/// definition would not be. Names with no existing top-level definition get
/// a fresh `.param` card immediately after the title, so they are defined
/// before any line that reads them.
///
/// Definitions inside a `.SUBCKT` body are deliberately left alone: those
/// are scoped to the subcircuit, and rewriting them would change a different
/// parameter that merely shares a name.
pub(super) fn override_param_source(
    source: &str,
    overrides: &[(String, f64)],
) -> (String, Vec<String>) {
    let mut applied: Vec<String> = Vec::new();
    let mut depth = 0usize;
    let mut lines: Vec<String> = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim_start();
        let lowered = trimmed.to_ascii_lowercase();
        if lowered.starts_with(".subckt") {
            depth += 1;
        } else if lowered.starts_with(".ends") || lowered.starts_with(".eom") {
            depth = depth.saturating_sub(1);
        }

        let is_top_level_param =
            depth == 0 && (lowered.starts_with(".param") || lowered.starts_with(".csparam"));
        if !is_top_level_param {
            lines.push(line.to_string());
            continue;
        }

        let mut rewritten = String::with_capacity(line.len());
        for (index, token) in split_outside_braces(line).into_iter().enumerate() {
            if index > 0 {
                rewritten.push(' ');
            }
            match token.split_once('=') {
                Some((name, _)) => {
                    let candidate = name.trim();
                    match overrides
                        .iter()
                        .find(|(target, _)| target.eq_ignore_ascii_case(candidate))
                    {
                        Some((target, value)) => {
                            rewritten.push_str(&format!("{candidate}={value:.17e}"));
                            if !applied.iter().any(|seen| seen.eq_ignore_ascii_case(target)) {
                                applied.push(target.clone());
                            }
                        }
                        None => rewritten.push_str(token),
                    }
                }
                None => rewritten.push_str(token),
            }
        }
        lines.push(rewritten);
    }

    let missing: Vec<&(String, f64)> = overrides
        .iter()
        .filter(|(name, _)| !applied.iter().any(|seen| seen.eq_ignore_ascii_case(name)))
        .collect();
    if !missing.is_empty() {
        // The first line is the title; new cards go directly beneath it.
        let insert_at = usize::from(!lines.is_empty());
        let cards: Vec<String> = missing
            .iter()
            .map(|(name, value)| format!(".param {name}={value:.17e}"))
            .collect();
        lines.splice(insert_at..insert_at, cards);
    }

    let mut rendered = lines.join("\n");
    if source.ends_with('\n') {
        rendered.push('\n');
    }
    (rendered, applied)
}

/// Split a line on whitespace that is not inside `{...}`, `'...'`, or `"..."`.
///
/// A `.param` value may be a brace expression containing spaces, so naive
/// whitespace splitting would tear one assignment into several tokens.
pub(super) fn split_outside_braces(line: &str) -> Vec<&str> {
    let bytes = line.as_bytes();
    let mut tokens = Vec::new();
    let (mut start, mut depth) = (None::<usize>, 0usize);
    let mut quote: Option<u8> = None;

    for (index, byte) in bytes.iter().enumerate() {
        if let Some(open) = quote {
            if *byte == open {
                quote = None;
            }
            continue;
        }
        match byte {
            b'\'' | b'"' => quote = Some(*byte),
            b'{' | b'(' => depth += 1,
            b'}' | b')' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if byte.is_ascii_whitespace() && depth == 0 && quote.is_none() {
            if let Some(begin) = start.take() {
                tokens.push(&line[begin..index]);
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }
    if let Some(begin) = start {
        tokens.push(&line[begin..]);
    }
    tokens
}

/// Prepend a synthetic title unless the content already starts with a
/// `*` comment line (which SPICE treats as the title here).
pub(super) fn ensure_statement_content(content: &str) -> Cow<'_, str> {
    let first_meaningful = content.lines().map(str::trim).find(|line| !line.is_empty());
    match first_meaningful {
        Some(line) if line.starts_with('*') => Cow::Borrowed(content),
        Some(_) => Cow::Owned(format!("* Untitled circuit\n{content}")),
        None => Cow::Borrowed(content),
    }
}

pub(super) fn parse_options(
    resource_limits: Option<&PyResourceLimits>,
) -> rspice_core::netlist::NetlistParseOptions {
    rspice_core::netlist::NetlistParseOptions {
        resource_limits: resource_limits
            .map(PyResourceLimits::to_core)
            .unwrap_or_default(),
        ..rspice_core::netlist::NetlistParseOptions::default()
    }
}
