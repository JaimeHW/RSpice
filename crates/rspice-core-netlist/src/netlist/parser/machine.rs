//! LTspice `.machine` state machines.
//!
//! ```text
//! .mach[ine] [[tripdt=]<value>]
//! .state <name> <value>
//! .rule <from>|* <to> <condition>
//! .output (<node>[, <node>]) <expression>
//! .endmach[ine]
//! ```
//!
//! The cards of a block may come in any order: the first `.state` is the
//! initial state and the rules keep their order. Any other line inside the
//! block is an error, as it is in LTspice, and so is a block card outside
//! one.

use super::*;
use crate::netlist::{MachineRule, StateMachine};

/// A `.machine` block being read.
#[derive(Debug)]
pub(super) struct OpenMachine {
    opened_at: usize,
    tripdt: Option<String>,
    states: Vec<(String, String)>,
    rules: Vec<PendingRule>,
    /// Each output's positive and negative node and its expression.
    outputs: Vec<(String, String, String)>,
}

impl OpenMachine {
    /// The line of the `.machine` card.
    pub(super) fn opened_at(&self) -> usize {
        self.opened_at
    }
}

/// A rule whose state names are bound when the block closes, since a rule
/// may name a state declared after it.
#[derive(Debug)]
struct PendingRule {
    line: usize,
    from: String,
    to: String,
    condition: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Machine,
    EndMachine,
    State,
    Rule,
    Output,
}

/// The block card `head` names. `.mach` and `.endmach` may be spelled out to
/// any length up to `.machine` and `.endmachine`.
fn card(head: &str) -> Option<Card> {
    let lower = head.to_ascii_lowercase();
    let spells = |full: &str, shortest: usize| lower.len() >= shortest && full.starts_with(&lower);
    if spells(".machine", ".mach".len()) {
        Some(Card::Machine)
    } else if spells(".endmachine", ".endmach".len()) {
        Some(Card::EndMachine)
    } else {
        match lower.as_str() {
            ".state" => Some(Card::State),
            ".rule" => Some(Card::Rule),
            ".output" => Some(Card::Output),
            _ => None,
        }
    }
}

fn syntax(line: usize, message: String) -> ParseError {
    ParseError::Syntax { line, message }
}

/// Split off the first whitespace-delimited field of `text`.
fn split_field(text: &str) -> Option<(&str, &str)> {
    let text = text.trim_start();
    if text.is_empty() {
        return None;
    }
    let end = text.find(char::is_whitespace).unwrap_or(text.len());
    Some((&text[..end], text[end..].trim()))
}

fn is_state_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Read `line` as part of a `.machine` block when it is one, and report
/// whether it was. Every line inside an open block belongs to it.
pub(super) fn process_machine_line(
    line: &str,
    line_num: usize,
    origin: &NetlistSourceLocation,
    state: &mut ParseState,
) -> Result<bool, ParseError> {
    let Some((head, rest)) = split_field(line) else {
        return Ok(false);
    };
    let card = card(head);
    let Some(open) = state.open_machine.as_mut() else {
        return match card {
            None => Ok(false),
            Some(Card::Machine) => {
                state.open_machine = Some(open_machine(rest, line_num)?);
                Ok(true)
            }
            Some(_) => Err(syntax(
                line_num,
                format!("{head} must be inside a .machine ... .endmachine block"),
            )),
        };
    };
    match card {
        Some(Card::State) => read_state(open, rest, line_num)?,
        Some(Card::Rule) => read_rule(open, rest, line_num)?,
        Some(Card::Output) => read_output(open, rest, line_num)?,
        Some(Card::EndMachine) => {
            if !rest.is_empty() {
                return Err(syntax(
                    line_num,
                    format!("unexpected '{rest}' after {head}"),
                ));
            }
            let open = state
                .open_machine
                .take()
                .expect("an open block was matched above");
            close_machine(open, origin, state)?;
        }
        Some(Card::Machine) | None => {
            return Err(syntax(
                line_num,
                format!(
                    "expected .state, .rule, .output or .endmachine in the .machine block \
                     opened at line {}, found '{head}'",
                    open.opened_at
                ),
            ));
        }
    }
    Ok(true)
}

fn open_machine(rest: &str, line_num: usize) -> Result<OpenMachine, ParseError> {
    let tripdt = if rest.is_empty() {
        None
    } else {
        let value = match rest.get(..6) {
            Some(keyword) if keyword.eq_ignore_ascii_case("tripdt") => {
                let after = rest[6..].trim_start();
                after.strip_prefix('=').map(str::trim).ok_or_else(|| {
                    syntax(
                        line_num,
                        format!("expected 'tripdt=<value>', found '{rest}'"),
                    )
                })?
            }
            _ => rest,
        };
        if value.is_empty() {
            return Err(syntax(line_num, "tripdt needs a value".to_string()));
        }
        Some(value.to_string())
    };
    Ok(OpenMachine {
        opened_at: line_num,
        tripdt,
        states: Vec::new(),
        rules: Vec::new(),
        outputs: Vec::new(),
    })
}

fn read_state(open: &mut OpenMachine, rest: &str, line_num: usize) -> Result<(), ParseError> {
    let (name, value) = split_field(rest)
        .filter(|(_, value)| !value.is_empty())
        .ok_or_else(|| syntax(line_num, ".state needs a name and a value".to_string()))?;
    if !is_state_name(name) {
        return Err(syntax(
            line_num,
            format!("'{name}' is not a state name: use letters, digits and '_'"),
        ));
    }
    if name.eq_ignore_ascii_case("state") {
        return Err(syntax(
            line_num,
            "'state' is the current state's value in a .machine expression and cannot name a \
             state"
                .to_string(),
        ));
    }
    if open
        .states
        .iter()
        .any(|(existing, _)| existing.eq_ignore_ascii_case(name))
    {
        return Err(syntax(
            line_num,
            format!("state '{name}' is already defined in this .machine block"),
        ));
    }
    open.states.push((name.to_string(), value.to_string()));
    Ok(())
}

fn read_rule(open: &mut OpenMachine, rest: &str, line_num: usize) -> Result<(), ParseError> {
    let missing = || {
        syntax(
            line_num,
            ".rule needs a state to leave (or '*'), a state to enter and a condition".to_string(),
        )
    };
    let (from, rest) = split_field(rest).ok_or_else(missing)?;
    let (to, condition) = split_field(rest).ok_or_else(missing)?;
    if condition.is_empty() {
        return Err(missing());
    }
    open.rules.push(PendingRule {
        line: line_num,
        from: from.to_string(),
        to: to.to_string(),
        condition: condition.to_string(),
    });
    Ok(())
}

fn read_output(open: &mut OpenMachine, rest: &str, line_num: usize) -> Result<(), ParseError> {
    let inner_and_expression = rest
        .strip_prefix('(')
        .and_then(|after| after.split_once(')'));
    let Some((inner, expression)) = inner_and_expression else {
        return Err(syntax(
            line_num,
            format!("expected .output (<node>[, <node>]) <expression>, found '.output {rest}'"),
        ));
    };
    let nodes: Vec<&str> = inner
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|node| !node.is_empty())
        .collect();
    let (pos, neg) = match nodes.as_slice() {
        [pos] => (*pos, "0"),
        [pos, neg] => (*pos, *neg),
        _ => {
            return Err(syntax(
                line_num,
                format!(".output names one or two nodes, found '({inner})'"),
            ));
        }
    };
    let expression = expression.trim();
    if expression.is_empty() {
        return Err(syntax(
            line_num,
            format!(".output ({inner}) needs an expression"),
        ));
    }
    // Node names fold to upper case, as every node table matches them.
    open.outputs.push((
        pos.to_ascii_uppercase(),
        neg.to_ascii_uppercase(),
        expression.to_string(),
    ));
    Ok(())
}

/// Bind the rules' state names and add the machine to the open subcircuit or
/// the top level.
fn close_machine(
    open: OpenMachine,
    origin: &NetlistSourceLocation,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    let index_of = |name: &str| {
        open.states
            .iter()
            .position(|(state, _)| state.eq_ignore_ascii_case(name))
    };
    let mut rules = Vec::with_capacity(open.rules.len());
    for rule in &open.rules {
        let unknown = |name: &str| {
            syntax(
                rule.line,
                format!("unknown state '{name}' in .rule; declare it with .state"),
            )
        };
        let from = if rule.from == "*" {
            None
        } else {
            Some(index_of(&rule.from).ok_or_else(|| unknown(&rule.from))?)
        };
        let to = index_of(&rule.to).ok_or_else(|| unknown(&rule.to))?;
        rules.push(MachineRule {
            from,
            to,
            condition: rule.condition.clone(),
        });
    }
    let mut nodes = Vec::with_capacity(2 * open.outputs.len());
    let mut outputs = Vec::with_capacity(open.outputs.len());
    for (pos, neg, expression) in open.outputs {
        nodes.push(pos);
        nodes.push(neg);
        outputs.push(expression);
    }
    state.machine_count += 1;
    let element = Element {
        name: format!("MACHINE#{}", state.machine_count),
        kind: ElementKind::StateMachine(Box::new(StateMachine {
            tripdt: open.tripdt,
            states: open.states,
            rules,
            outputs,
        })),
        nodes,
        provenance: crate::netlist::ElementProvenance::Authored,
    };
    let scope = state
        .subckt_stack
        .last()
        .map(|frame| frame.qualified_name.clone());
    state.element_origins.insert(
        crate::netlist::element_origin_key(scope.as_deref(), &element.name),
        origin.clone(),
    );
    match state.subckt_stack.last_mut() {
        Some(frame) => frame.def.elements.push(element),
        None => state.elements.push(element),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_cards_take_ltspice_abbreviations() {
        for head in [".mach", ".MACHI", ".machine"] {
            assert_eq!(card(head), Some(Card::Machine), "{head}");
        }
        for head in [".endmach", ".EndMachine"] {
            assert_eq!(card(head), Some(Card::EndMachine), "{head}");
        }
        for head in [".mac", ".machines", ".endma", ".states", ".out"] {
            assert_eq!(card(head), None, "{head}");
        }
    }

    #[test]
    fn tripdt_is_positional_or_named() {
        assert_eq!(open_machine("", 1).unwrap().tripdt, None);
        assert_eq!(open_machine("1u", 1).unwrap().tripdt.as_deref(), Some("1u"));
        assert_eq!(
            open_machine("TripDt = {t}", 1).unwrap().tripdt.as_deref(),
            Some("{t}")
        );
        assert!(open_machine("tripdt=", 1).is_err());
    }

    #[test]
    fn outputs_take_one_or_two_nodes() {
        let mut open = open_machine("", 1).unwrap();
        read_output(&mut open, "(a) state", 2).unwrap();
        read_output(&mut open, "(b, c) V(x) < .5", 3).unwrap();
        read_output(&mut open, "(d e) 1", 4).unwrap();
        assert_eq!(
            open.outputs,
            [
                ("A".into(), "0".into(), "state".into()),
                ("B".into(), "C".into(), "V(x) < .5".into()),
                ("D".into(), "E".into(), "1".into()),
            ]
        );
        for malformed in ["2 state", "(a) ", "(a, b, c) 1", "() 1"] {
            assert!(read_output(&mut open, malformed, 5).is_err(), "{malformed}");
        }
    }
}
