//! One physical-reference grammar for authored outputs and retained receipts.

use super::SavedOutputKind;
use crate::calculator::{ast::CalculatorExpr, parser::Parser};
use rspice_app_types::hierarchy_path::ProbeTarget;
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

/// Quantity identity for a raw probe, including accepted function whitespace.
/// This is a unit projection; argument validation remains in validate_raw_probe.
pub fn raw_probe_unit(expression: &str) -> Option<&'static str> {
    if device_current_probe(expression).is_some() {
        return Some("A");
    }
    let (function, _) = expression.trim().split_once('(')?;
    match function.trim() {
        value if value.eq_ignore_ascii_case("V") => Some("V"),
        value if value.eq_ignore_ascii_case("I") => Some("A"),
        _ => None,
    }
}

pub fn validate_raw_probe(expression: &str) -> Result<(), String> {
    let expression = expression.trim();
    if expression.starts_with('@') {
        return device_current_probe(expression).map(|_| ()).ok_or_else(|| {
            "device current must use @device[i], @device[id], @device[ig], or another terminal current".to_owned()
        });
    }
    let open = expression
        .find('(')
        .ok_or_else(|| "raw output must use V(node), V(node+, node-), or I(source)".to_owned())?;
    if !expression.ends_with(')') {
        return Err("raw output has an unterminated probe".to_owned());
    }
    let function = expression[..open].trim();
    let arguments = &expression[open + 1..expression.len() - 1];
    let arguments = arguments.split(',').map(str::trim).collect::<Vec<_>>();
    if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2)
        || function.eq_ignore_ascii_case("I") && arguments.len() == 1
    {
        for argument in arguments {
            parse_probe_target(argument).map_err(|error| {
                format!("raw output must use V(node), V(node+, node-), or I(source): {error}")
            })?;
        }
        Ok(())
    } else {
        Err("raw output must use V(node), V(node+, node-), or I(source)".to_owned())
    }
}

/// A terminal-current trace, distinct from a scalar operating-point parameter.
pub fn device_current_probe(expression: &str) -> Option<(&str, &str)> {
    let body = expression.trim().strip_prefix('@')?;
    let (device, quantity) = body.strip_suffix(']')?.split_once('[')?;
    parse_probe_target(device).ok()?;
    if !matches!(
        quantity.to_ascii_lowercase().as_str(),
        "i" | "id" | "ig" | "is" | "ib" | "ic" | "ie" | "isub" | "ik" | "ip" | "in" | "icp" | "icn"
    ) {
        return None;
    }
    Some((device, quantity))
}

/// Resolve one probe token — a node, device, or noise-source name, optionally
/// scoped — against the one instance-path grammar.
///
/// Every spelling the product has written is accepted and resolves to the same
/// target: canonical `/X1/net`, engine `x1.net` and `x1:net`, and the legacy
/// `/top/X1/net` that projects saved before the design root became implicit
/// still carry. Resolution is all this does — the persisted expression stays
/// exactly as the project stored it, because a read is not an edit.
///
/// `/` on its own is the design root, which is why a doubled separator is
/// refused here: `//net` would otherwise resolve to a root probe rather than
/// to the empty instance name it actually spells.
pub fn parse_probe_target(value: &str) -> Result<ProbeTarget, String> {
    ProbeTarget::parse_legacy(value).map_err(|error| error.to_string())
}
