//! Exact source identities captured before authored waveforms are adopted.
//! Missing references stay missing even if an output later uses that label.

use std::collections::{BTreeMap, BTreeSet};

use super::*;
use crate::analysis::calculator::{ast::CalculatorExpr, parser::Parser};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutputSourceBindings {
    pub axis: SavedOutputAxis,
    pub references: BTreeMap<String, SavedOutputBoundSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SavedOutputAxis {
    OperatingPoint,
    Waveform { name: String },
    DcFamily,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SavedOutputBoundSource {
    Waveform { name: String },
    DcQuantity { quantity: usize },
    Ground,
    Missing,
}

impl SavedOutputSourceBindings {
    pub(super) fn validate(
        &self,
        receipt: &SavedOutputReceipt,
        analysis: &crate::state::AnalysisResult,
    ) -> Result<(), String> {
        use crate::state::{AnalysisResultPayload, DcSweepFamily};
        let expected = saved_output_references(receipt.output_kind, &receipt.source_expression)?
            .ok_or_else(|| "this saved-output kind cannot carry source bindings".to_owned())?;
        if expected.iter().ne(self.references.keys()) {
            return Err("saved-output bindings do not match the expression references".to_owned());
        }
        let family = match &analysis.result_payload {
            Some(AnalysisResultPayload::DcSweep { evidence })
                if !matches!(evidence.family, DcSweepFamily::Single) =>
            {
                Some(evidence)
            }
            _ => None,
        };
        let deferred = receipt.status == SavedOutputMaterializationStatus::Deferred;
        let retained = |name: &str| {
            analysis.dc_op.as_ref().map_or_else(
                || analysis.waveforms.iter().any(|wave| wave.name == name),
                |op| {
                    op.node_voltages
                        .iter()
                        .chain(&op.branch_currents)
                        .any(|value| value.name == name)
                },
            )
        };
        match &self.axis {
            SavedOutputAxis::OperatingPoint if analysis.dc_op.is_none() => {
                return Err(
                    "saved-output OP axis requires solved operating-point tables".to_owned(),
                );
            }
            SavedOutputAxis::DcFamily if family.is_none() => {
                return Err("saved-output DC axis requires exact family evidence".to_owned());
            }
            SavedOutputAxis::Waveform { name }
                if name.is_empty()
                    || family.is_some()
                    || analysis.dc_op.is_some()
                    || deferred && !retained(name) =>
            {
                return Err("saved-output axis does not identify retained analysis data".to_owned());
            }
            _ => {}
        }
        for (reference, source) in &self.references {
            let current = reference.starts_with("i(");
            match source {
                SavedOutputBoundSource::Ground if current => {
                    return Err("a current source cannot bind to the ground voltage".to_owned());
                }
                SavedOutputBoundSource::Waveform { name }
                    if name.is_empty() || family.is_some() || deferred && !retained(name) =>
                {
                    return Err(
                        "saved-output source does not identify retained analysis data".to_owned(),
                    );
                }
                SavedOutputBoundSource::DcQuantity { quantity } => {
                    let evidence = family.ok_or_else(|| {
                        "saved-output DC source requires exact family evidence".to_owned()
                    })?;
                    let physical = evidence.quantities.get(*quantity).ok_or_else(|| {
                        "saved-output DC quantity index is out of bounds".to_owned()
                    })?;
                    if (physical.unit() == "A") != current {
                        return Err(
                            "saved-output DC binding changes the voltage/current namespace"
                                .to_owned(),
                        );
                    }
                    if deferred
                        && !matches!(evidence.selection, crate::state::DcCurveSelection::All)
                        && evidence
                            .curve_indices()
                            .filter(|curve| curve.quantity == *quantity)
                            .map(|curve| curve.member)
                            .collect::<std::collections::HashSet<_>>()
                            .len()
                            != evidence.member_count()
                    {
                        return Err(
                            "deferred saved-output source has missing DC members".to_owned()
                        );
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Only absence means legacy unknown. Explicit null is malformed evidence.
pub(super) fn deserialize_bindings<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<SavedOutputSourceBindings>, D::Error> {
    serde::Deserialize::deserialize(deserializer).map(Some)
}

/// A shared reference grammar for preparation and persisted receipt validation.
/// Keys preserve literal node punctuation and numeric spellings.
pub(crate) fn saved_output_references(
    kind: SavedOutputKind,
    expression: &str,
) -> Result<Option<BTreeSet<String>>, String> {
    let mut references = BTreeSet::new();
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => {
            let expression = expression.trim();
            crate::state::workspace::validate_raw_probe(expression)?;
            let (function, arguments) = expression.split_once('(').expect("validated probe");
            for argument in arguments[..arguments.len() - 1].split(',') {
                references.insert(
                    format!("{}({})", function.trim(), argument.trim()).to_ascii_lowercase(),
                );
            }
        }
        SavedOutputKind::DerivedExpression => {
            let parsed = Parser::new(expression)
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
