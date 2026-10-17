//! One physical-reference grammar for authored outputs and retained receipts.

use super::SavedOutputKind;
use crate::calculator::{ast::CalculatorExpr, parser::Parser};
use rspice_app_types::raw_probe::{device_current_probe, validate_raw_probe};
use std::collections::BTreeSet;
use std::ops::Range;

/// A shared reference grammar for preparation and persisted receipt validation.
/// Keys preserve literal node punctuation and numeric spellings.
pub fn saved_output_references(
    kind: SavedOutputKind,
    expression: &str,
) -> Result<Option<BTreeSet<String>>, String> {
    let mut references = BTreeSet::new();
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => {
            let expression = expression.trim();
            validate_raw_probe(expression)?;
            if device_current_probe(expression).is_some() {
                references.insert(expression.to_ascii_lowercase());
                return Ok(Some(references));
            }
            let (function, arguments) = expression.split_once('(').expect("validated probe");
            for argument in arguments[..arguments.len() - 1].split(',') {
                references.insert(
                    format!("{}({})", function.trim(), argument.trim()).to_ascii_lowercase(),
                );
            }
        }
        SavedOutputKind::DerivedExpression => {
            let parsed = Parser::new(expression, crate::spice_value::parse_spice_value_checked)
                .try_parse()
                .map_err(|error| error.to_string())?;
            let mut pending = vec![&parsed];
            while let Some(expr) = pending.pop() {
                match expr {
                    CalculatorExpr::WaveformRef { signal, dataset } => {
                        if dataset.is_some() {
                            return Err("saved outputs require sources from their owning analysis"
                                .to_owned());
                        }
                        references.insert(signal.to_ascii_lowercase());
                    }
                    CalculatorExpr::BinaryOp { left, right, .. } => {
                        pending.extend([left.as_ref(), right.as_ref()])
                    }
                    CalculatorExpr::UnaryOp { operand, .. } => pending.push(operand),
                    CalculatorExpr::FunctionCall { args, .. } => pending.extend(args),
                    CalculatorExpr::Number(_) | CalculatorExpr::Constant(_) => {}
                }
            }
        }
        _ => return Ok(None),
    }
    Ok(Some(references))
}

/// `expression` with each node it names spelled as `spell` says, read by the
/// grammar [`saved_output_references`] reads it with. `None` when `spell`
/// renames none of them.
///
/// Only a voltage probe's arguments are nodes. A current names a device or a
/// branch, a device quantity names a device, and a bare calculator name is a
/// trace, so a device or trace called `gnd` keeps its name. Every byte outside
/// a renamed node stays as written.
pub fn respell_saved_output_nodes(
    kind: SavedOutputKind,
    expression: &str,
    mut spell: impl FnMut(&str) -> Option<String>,
) -> Result<Option<String>, String> {
    let nodes = match kind {
        SavedOutputKind::RawVoltageOrCurrent => raw_probe_nodes(expression)?,
        SavedOutputKind::DerivedExpression => {
            Parser::new(expression, crate::spice_value::parse_spice_value_checked)
                .try_parse_probe_arguments()
                .map_err(|error| error.to_string())?
                .into_iter()
                .filter(|argument| argument.voltage)
                .map(|argument| (argument.span, argument.name))
                .collect()
        }
        _ => return Ok(None),
    };
    let mut respelled = String::with_capacity(expression.len());
    let mut copied = 0;
    let mut renamed = false;
    for (span, name) in nodes {
        if let Some(spelling) = spell(&name) {
            respelled.push_str(&expression[copied..span.start]);
            respelled.push_str(&spelling);
            copied = span.end;
            renamed = true;
        }
    }
    if !renamed {
        return Ok(None);
    }
    respelled.push_str(&expression[copied..]);
    Ok(Some(respelled))
}

/// Where each node of a raw probe is spelled: the arguments of `V(...)`,
/// split as [`saved_output_references`] splits them.
fn raw_probe_nodes(expression: &str) -> Result<Vec<(Range<usize>, String)>, String> {
    validate_raw_probe(expression)?;
    let trimmed = expression.trim();
    if device_current_probe(trimmed).is_some() {
        return Ok(Vec::new());
    }
    let open = trimmed.find('(').expect("validated probe");
    if !trimmed[..open].trim().eq_ignore_ascii_case("V") {
        return Ok(Vec::new());
    }
    let mut start = expression.len() - expression.trim_start().len() + open + 1;
    let mut nodes = Vec::new();
    for argument in trimmed[open + 1..trimmed.len() - 1].split(',') {
        let leading = argument.len() - argument.trim_start().len();
        let name = argument.trim();
        nodes.push((
            start + leading..start + leading + name.len(),
            name.to_owned(),
        ));
        start += argument.len() + 1;
    }
    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ground_as_zero(kind: SavedOutputKind, expression: &str) -> Option<String> {
        respell_saved_output_nodes(kind, expression, |node| {
            ["gnd", "gnd!"]
                .iter()
                .any(|ground| node.eq_ignore_ascii_case(ground))
                .then(|| "0".to_owned())
        })
        .expect("the expression parses")
    }

    #[test]
    fn a_raw_probe_respells_its_nodes_and_nothing_else() {
        let raw = SavedOutputKind::RawVoltageOrCurrent;
        assert_eq!(
            ground_as_zero(raw, "V(out,GND!)").as_deref(),
            Some("V(out,0)")
        );
        assert_eq!(
            ground_as_zero(raw, " v ( GND! , out ) ").as_deref(),
            Some(" v ( 0 , out ) ")
        );
        assert_eq!(ground_as_zero(raw, "V(gnd)").as_deref(), Some("V(0)"));
        // A current names a device, however it is called.
        for untouched in ["V(out)", "V(out,ref)", "I(gnd)", "@gnd[i]"] {
            assert_eq!(ground_as_zero(raw, untouched), None, "{untouched}");
        }
    }

    #[test]
    fn a_derived_expression_respells_voltage_arguments_where_the_parser_reads_them() {
        let derived = SavedOutputKind::DerivedExpression;
        for (authored, spelled) in [
            ("V(out)-V(GND!)", "V(out)-V(0)"),
            ("I(gnd)*V(GND)", "I(gnd)*V(0)"),
            ("1k * v( gnd! ) / 2", "1k * v( 0 ) / 2"),
            (r#"avg(V("GND!"))"#, "avg(V(0))"),
            ("V(/X1/out)/V(gnd)", "V(/X1/out)/V(0)"),
        ] {
            assert_eq!(
                ground_as_zero(derived, authored).as_deref(),
                Some(spelled),
                "{authored}"
            );
        }
        // A bare name is a trace and a current names a device: neither is a
        // node, so neither is respelled.
        for untouched in ["gnd * 2", "I(gnd)", "V(out) - V(gnd2)"] {
            assert_eq!(ground_as_zero(derived, untouched), None, "{untouched}");
        }
    }

    #[test]
    fn an_expression_the_grammar_refuses_is_refused_rather_than_respelled() {
        for (kind, expression) in [
            (SavedOutputKind::RawVoltageOrCurrent, "V(out,GND!"),
            (SavedOutputKind::DerivedExpression, "V(out) - "),
        ] {
            assert!(
                respell_saved_output_nodes(kind, expression, |_| Some("0".to_owned())).is_err(),
                "{expression}"
            );
        }
        assert_eq!(
            respell_saved_output_nodes(SavedOutputKind::RfPortQuantity, "S(1,2)", |_| Some(
                "0".to_owned()
            )),
            Ok(None)
        );
    }
}
