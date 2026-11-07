//! SPICE parameters are deck-global wherever their `.param` card sits: an
//! element may name a root parameter that a later card, or a later include,
//! defines. Such an element is read once the whole root has been read, with
//! every root parameter bound, and keeps its authored place in the deck.

use super::*;

/// Root `.param` names the deck declares anywhere, and the element cards
/// that named one of them before it was bound.
#[derive(Debug, Default)]
pub(super) struct ForwardParameters {
    declared: HashSet<String>,
    postponed: Vec<PostponedElement>,
    replaying: bool,
}

#[derive(Debug)]
struct PostponedElement {
    line: String,
    line_num: usize,
    origin: NetlistSourceLocation,
    source_is_included: bool,
    position: usize,
}

impl ForwardParameters {
    /// Collects the names every root `.param` card declares (subcircuit
    /// bodies keep their own scope and are left out).
    pub(super) fn scan(
        lines: &[&str],
        body_start: usize,
        allow_non_semicolon_comments: bool,
        xyce_syntax: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let mut declared = HashSet::new();
        let mut depth = 0usize;
        let mut card = String::new();
        let mut finish = |card: &mut String, depth: &mut usize| {
            let head = card.split_whitespace().next().unwrap_or("");
            if head.eq_ignore_ascii_case(".subckt") || head.eq_ignore_ascii_case(".macro") {
                *depth += 1;
            } else if head.eq_ignore_ascii_case(".ends") || head.eq_ignore_ascii_case(".eom") {
                *depth = depth.saturating_sub(1);
            } else if *depth == 0
                && [".param", ".params", ".global_param"]
                    .iter()
                    .any(|command| head.eq_ignore_ascii_case(command))
            {
                declared.extend(declared_parameter_names(&card[head.len()..]));
            }
            card.clear();
        };
        for (index, line) in lines.iter().enumerate().skip(body_start) {
            poll_parse_abort(abort, index)?;
            if xyce_syntax && xyce_physical_line_is_comment(line) {
                continue;
            }
            let stripped = strip_inline_semicolon_comment_with_non_semicolon_comments(
                line,
                allow_non_semicolon_comments,
            );
            let trimmed = stripped.trim();
            if trimmed.is_empty() || trimmed.starts_with('*') {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix('+') {
                card.push(' ');
                card.push_str(rest);
                continue;
            }
            finish(&mut card, &mut depth);
            card.push_str(trimmed);
        }
        finish(&mut card, &mut depth);
        Ok(Self {
            declared,
            postponed: Vec::new(),
            replaying: false,
        })
    }
}

/// Postpones a root element card that names a root parameter no card has
/// bound yet. Returns whether the card was postponed.
pub(super) fn postpone_forward_reference(
    state: &mut ParseState,
    line: &str,
    line_num: usize,
    origin: &NetlistSourceLocation,
    source_is_included: bool,
) -> bool {
    let plan = &state.forward_parameters;
    if plan.replaying
        || plan.declared.is_empty()
        || !state.subckt_stack.is_empty()
        || state.open_machine.is_some()
        || !line.trim_start().starts_with(|c: char| c.is_alphabetic())
    {
        return false;
    }
    let mut words = line.split_whitespace();
    words.next();
    let names_unbound = words.any(|word| {
        word.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .filter(|name| name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_'))
            .any(|name| {
                plan.declared.contains(&name.to_ascii_uppercase())
                    && !state.params.has_any_parameter_binding(name)
            })
    });
    if names_unbound {
        let position = state.elements.len();
        state.forward_parameters.postponed.push(PostponedElement {
            line: line.to_owned(),
            line_num,
            origin: origin.clone(),
            source_is_included,
            position,
        });
    }
    names_unbound
}

/// Reads the postponed element cards now that the root is complete, placing
/// each card's elements where the card stood.
pub(super) fn read_postponed_elements(
    state: &mut ParseState,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let postponed = std::mem::take(&mut state.forward_parameters.postponed);
    state.forward_parameters.replaying = true;
    let mut inserted = 0usize;
    for (index, card) in postponed.into_iter().enumerate() {
        poll_parse_abort(abort, index)?;
        let first_new = state.elements.len();
        process_line_gated(
            &card.line,
            card.line_num,
            &card.origin,
            state,
            card.source_is_included,
            abort,
        )?;
        let parsed = state.elements.drain(first_new..).collect::<Vec<_>>();
        let at = card.position + inserted;
        inserted += parsed.len();
        state.elements.splice(at..at, parsed);
    }
    state.forward_parameters.replaying = false;
    Ok(())
}

/// Names a `.param` card's text declares: every name followed by `=` (or by
/// a parenthesised argument list and `=`) outside braces and parentheses.
fn declared_parameter_names(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut names = Vec::new();
    let mut depth = 0usize;
    let mut quoted = false;
    for (index, &byte) in bytes.iter().enumerate() {
        match byte {
            b'"' | b'\'' => quoted = !quoted,
            b'{' | b'(' if !quoted => depth += 1,
            b'}' | b')' if !quoted => depth = depth.saturating_sub(1),
            b'=' if !quoted
                && depth == 0
                && !matches!(bytes.get(index + 1), Some(b'='))
                && !matches!(
                    index.checked_sub(1).map(|before| bytes[before]),
                    Some(b'=' | b'<' | b'>' | b'!')
                ) =>
            {
                if let Some(name) = name_before_assignment(&text[..index]) {
                    names.push(name.to_ascii_uppercase());
                }
            }
            _ => {}
        }
    }
    names
}

fn name_before_assignment(text: &str) -> Option<&str> {
    let mut text = text.trim_end();
    if text.ends_with(')') {
        text = &text[..text.rfind('(')?];
        text = text.trim_end();
    }
    let start = text
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .map_or(0, |index| index + 1);
    let name = &text[start..];
    name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        .then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_names_cover_spaced_function_and_multi_assignment_forms() {
        assert_eq!(
            declared_parameter_names(" a=1 b = {c*2} f(x, y)=x*y g={a==b ? 1 : 2}"),
            ["A", "B", "F", "G"]
        );
    }

    #[test]
    fn an_element_reads_parameters_declared_after_it() {
        let netlist = parse_netlist(
            "* forward\nV1 a 0 1\nR1 a b {R}\nC1 b 0 {Cx}\nL1 b c {Lx}\nE1 e 0 a 0 {gain}\n\
             G1 0 g a 0 {gm}\nR2 g 0 1k\n.param R=10k\n.param Cx=.1u Lx={2*half}\n\
             .param half=0.5m gain=3 gm=2m\n.op\n.end\n",
        )
        .unwrap();
        let names = netlist
            .elements
            .iter()
            .map(|element| element.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, ["V1", "R1", "C1", "L1", "E1", "G1", "R2"]);
        let capacitor = netlist.elements.iter().find(|e| e.name == "C1").unwrap();
        let ElementKind::Capacitor {
            value, value_expr, ..
        } = &capacitor.kind
        else {
            panic!("C1 is a capacitor");
        };
        assert!(value_expr.is_none());
        assert!((value - 0.1e-6).abs() < 1e-18);
        let inductor = netlist.elements.iter().find(|e| e.name == "L1").unwrap();
        let ElementKind::Inductor { value, .. } = &inductor.kind else {
            panic!("L1 is an inductor");
        };
        assert!((value - 1e-3).abs() < 1e-15);
    }
}
