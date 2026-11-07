//! LTspice and QSPICE read a resistor's current, `I(R1)`, in behavioral,
//! switch-control and state-machine expressions. A resistor read that way is
//! built in branch form, so its current is a solution variable the expression
//! binds to. Xyce has no such lead current, and its dialect keeps refusing it.

use super::*;

/// The resistors, by upper-case name, whose current an expression reads.
pub(super) fn read_as_currents(elements: &[Element]) -> HashSet<String> {
    let operands = current_reads(elements);
    if operands.is_empty() {
        return operands;
    }
    elements
        .iter()
        .filter(|element| matches!(element.kind, ElementKind::Resistor { .. }))
        .map(|element| element.name.to_ascii_uppercase())
        .filter(|name| operands.contains(name))
        .collect()
}

/// Every name, upper case, whose current a behavioral, switch-control or
/// state-machine expression reads as `I(name)`.
pub(super) fn current_reads(elements: &[Element]) -> HashSet<String> {
    let mut operands = HashSet::new();
    for element in elements {
        match &element.kind {
            ElementKind::BehavioralCurrent { expression, .. }
            | ElementKind::BehavioralVoltage { expression, .. }
            | ElementKind::GenericSwitch {
                control_expression: expression,
                ..
            } => current_operands(expression, &mut operands),
            ElementKind::StateMachine(machine) => {
                for text in machine
                    .rules
                    .iter()
                    .map(|rule| rule.condition.as_str())
                    .chain(machine.outputs.iter().map(String::as_str))
                {
                    current_operands(text, &mut operands);
                }
            }
            _ => {}
        }
    }
    operands
}

/// Adds the upper-case name of every single-operand `I(name)` in `text`.
fn current_operands(text: &str, operands: &mut HashSet<String>) {
    let continues_name =
        |c: char| c.is_alphanumeric() || matches!(c, '_' | '.' | '#' | ':' | '$' | '§');
    let mut previous = None;
    for (index, c) in text.char_indices() {
        if c.eq_ignore_ascii_case(&'I') && !previous.is_some_and(continues_name) {
            let rest = text[index + 1..].trim_start();
            if let Some(inside) = rest.strip_prefix('(')
                && let Some(end) = inside.find(')')
            {
                let name = inside[..end].trim();
                if !name.is_empty() && !name.contains([',', '(']) {
                    operands.insert(name.to_ascii_uppercase());
                }
            }
        }
        previous = Some(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_operand_current_reads_are_found_outside_longer_names() {
        let mut operands = HashSet::new();
        current_operands(
            "I(r1)*1k + i( X1.RA ) - maxi(R2) + I(a,b) + ii(R3)",
            &mut operands,
        );
        let mut found = operands.into_iter().collect::<Vec<_>>();
        found.sort();
        assert_eq!(found, ["R1", "X1.RA"]);
    }
}
