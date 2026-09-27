//! One physical-reference grammar for authored outputs and retained receipts.

use super::SavedOutputKind;
use crate::calculator::{ast::CalculatorExpr, parser::Parser};
use rspice_app_types::raw_probe::{device_current_probe, validate_raw_probe};
use std::collections::BTreeSet;

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
