//! `.if` / `.elseif` / `.else` / `.endif` netlist conditionals.
//!
//! Conditions are evaluated against the parameters known at the point the
//! directive is reached (ngspice numparam semantics), in the active scope
//! (subcircuit-local parameters when inside a `.subckt` body). Lines inside
//! a false branch are skipped entirely — their `.param`, element, and model
//! definitions never take effect — while nesting is still tracked so block
//! structure is validated everywhere.
//!
//! Conditions in regions that are themselves suppressed are *not* evaluated
//! (they may reference parameters that were never defined); their branches
//! are unconditionally inactive.
//!
//! An expression failure leaves the decision unresolved for the current pass.
//! No branch of that chain emits cards, but its structure is still validated.
//! Independent later cards can select TEMP/TNOM; the retained error prevents
//! publication unless a fresh source pass evaluates the decision successfully.

use super::*;

/// One open `.if`/`.elseif`/`.else` block.
#[derive(Debug, Clone)]
pub(super) struct ConditionalFrame {
    /// Whether the currently selected branch emits lines.
    active: bool,
    /// Whether any branch of this block has already been taken (or the
    /// whole block is inside a suppressed region, in which case no branch
    /// may ever activate).
    branch_taken: bool,
    /// A failed decision cannot select this or any later branch in this pass.
    unresolved: bool,
    /// Whether `.else` has been seen (no further branches are legal).
    else_seen: bool,
    /// Line number of the opening `.if`, for unbalanced-block diagnostics.
    pub(super) opened_at_line: usize,
}

/// Conditional directive recognized on a logical line.
pub(super) enum ConditionalDirective<'a> {
    If(&'a str),
    ElseIf(&'a str),
    Else,
    EndIf,
}

/// Recognize a conditional directive, returning the condition text for
/// `.if`/`.elseif`. Requires a clean token boundary so commands such as
/// `.ifx` are left to the normal (unknown-command) path.
pub(super) fn parse_conditional_directive(line: &str) -> Option<ConditionalDirective<'_>> {
    let trimmed = line.trim();

    let take = |keyword: &str| -> Option<&str> {
        let rest = trimmed.get(..keyword.len())?;
        if !rest.eq_ignore_ascii_case(keyword) {
            return None;
        }
        let tail = &trimmed[keyword.len()..];
        match tail.chars().next() {
            None => Some(""),
            Some(c) if c.is_whitespace() || c == '(' => Some(tail.trim()),
            _ => None,
        }
    };

    // Longest keywords first so `.elseif` is not consumed as `.else`.
    if let Some(cond) = take(".elseif") {
        return Some(ConditionalDirective::ElseIf(cond));
    }
    if let Some(rest) = take(".endif") {
        // Tolerate trailing text after `.endif` (some decks annotate the
        // closer, e.g. `.endif ; corners`); the block structure is what
        // matters here.
        let _ = rest;
        return Some(ConditionalDirective::EndIf);
    }
    if let Some(rest) = take(".else") {
        // `.else` takes no condition; tolerate trailing comment-free text
        // only if empty (anything else is likelier a typo than intent).
        return rest.is_empty().then_some(ConditionalDirective::Else);
    }
    if let Some(cond) = take(".if") {
        return Some(ConditionalDirective::If(cond));
    }
    None
}

impl ParseState {
    /// Whether the current position is inside a false conditional branch.
    pub(super) fn conditionals_suppress(&self) -> bool {
        self.conditional_stack.iter().any(|frame| !frame.active)
    }

    /// Parameter scope for evaluating a condition: subcircuit-local when
    /// inside a `.subckt` body, global otherwise.
    pub(super) fn condition_scope(&self) -> &ParamContext {
        self.subckt_stack
            .last()
            .map(|frame| &frame.local_params)
            .unwrap_or(&self.params)
    }

    /// Apply one conditional directive to the stack.
    pub(super) fn apply_conditional_directive(
        &mut self,
        directive: ConditionalDirective<'_>,
        line_num: usize,
        origin: &NetlistSourceLocation,
    ) -> Result<(), ParseError> {
        match directive {
            ConditionalDirective::If(condition) => {
                let parent_active = !self.conditionals_suppress();
                let decision = if parent_active {
                    self.condition_decision(condition, line_num, origin)?
                } else {
                    Some(false)
                };
                self.conditional_stack.push(ConditionalFrame {
                    active: parent_active && decision == Some(true),
                    // Inside a suppressed region no branch may ever fire.
                    branch_taken: decision == Some(true) || !parent_active,
                    unresolved: decision.is_none(),
                    else_seen: false,
                    opened_at_line: line_num,
                });
            }
            ConditionalDirective::ElseIf(condition) => {
                let parent_active = self
                    .conditional_stack
                    .split_last()
                    .map(|(_, below)| below.iter().all(|frame| frame.active))
                    .unwrap_or(true);
                let frame =
                    self.conditional_stack
                        .last_mut()
                        .ok_or_else(|| ParseError::Syntax {
                            line: line_num,
                            message: ".elseif without a matching .if".to_string(),
                        })?;
                if frame.else_seen {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".elseif after .else".to_string(),
                    });
                }
                if frame.branch_taken || frame.unresolved || !parent_active {
                    frame.active = false;
                } else {
                    let decision = self.condition_decision(condition, line_num, origin)?;
                    let frame = self
                        .conditional_stack
                        .last_mut()
                        .expect("frame checked above");
                    frame.active = decision == Some(true);
                    frame.branch_taken = decision == Some(true);
                    frame.unresolved = decision.is_none();
                }
            }
            ConditionalDirective::Else => {
                let parent_active = self
                    .conditional_stack
                    .split_last()
                    .map(|(_, below)| below.iter().all(|frame| frame.active))
                    .unwrap_or(true);
                let frame =
                    self.conditional_stack
                        .last_mut()
                        .ok_or_else(|| ParseError::Syntax {
                            line: line_num,
                            message: ".else without a matching .if".to_string(),
                        })?;
                if frame.else_seen {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: "duplicate .else".to_string(),
                    });
                }
                frame.else_seen = true;
                frame.active = parent_active && !frame.branch_taken && !frame.unresolved;
                frame.branch_taken = true;
            }
            ConditionalDirective::EndIf => {
                self.conditional_stack
                    .pop()
                    .ok_or_else(|| ParseError::Syntax {
                        line: line_num,
                        message: ".endif without a matching .if".to_string(),
                    })?;
            }
        }
        Ok(())
    }

    fn condition_decision(
        &mut self,
        condition: &str,
        line_num: usize,
        origin: &NetlistSourceLocation,
    ) -> Result<Option<bool>, ParseError> {
        let text = condition.trim();
        if text.is_empty() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".if/.elseif requires a condition expression".to_string(),
            });
        }
        match eval_expression(text, self.condition_scope()) {
            Ok(value) => Ok(Some(value != 0.0)),
            Err(error) => {
                self.temperature_options.retain_card_error(
                    ParseError::Syntax {
                        line: line_num,
                        message: format!("invalid conditional expression `{text}`: {error}"),
                    },
                    line_num,
                    origin,
                );
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_chains_emit_nothing_and_skip_later_condition_samples() {
        let mut state = ParseState::new();
        state.params.set_random_seed(37);
        let reference = state.params.isolated_random_clone();
        for (index, line) in [
            ".if (1/(TEMP-27))",
            ".param first=1",
            ".if missing",
            ".param nested=1",
            ".endif",
            ".elseif aunif(0,1)",
            ".param second=1",
            ".else",
            ".param third=1",
            ".endif",
        ]
        .into_iter()
        .enumerate()
        {
            process_line_gated(
                line,
                index + 2,
                &NetlistSourceLocation::in_memory(index + 2),
                &mut state,
                false,
                &NoAbort,
            )
            .unwrap();
        }
        assert!(state.conditional_stack.is_empty());
        for name in ["first", "nested", "second", "third"] {
            assert_eq!(state.params.get(name), None, "{name}");
        }
        let error = state.temperature_options.take_error().unwrap();
        assert!(error.to_string().contains("Division by zero"), "{error}");
        assert_eq!(
            eval_expression("aunif(0,1)", &state.params).unwrap(),
            eval_expression("aunif(0,1)", &reference).unwrap()
        );
    }
}
