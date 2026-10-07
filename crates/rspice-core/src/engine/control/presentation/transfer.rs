//! Transfer scalars retain physical units and exact unbounded determinations.
use super::*;
use crate::analysis::TransferFunctionResult;
use crate::execution::result_document::{ResultScalar, ScalarUnavailability, ScalarValue};
use crate::netlist::expr::ExprError;

pub(in crate::engine::control) fn resolve_scalar(
    circuit: &ControlCircuit,
    name: &str,
) -> Result<Option<ComplexValue>, ExprError> {
    let Ok((dataset, raw)) = circuit.qualified(name, 0) else {
        return Ok(None);
    };
    let Some(selected) = select(dataset, raw) else {
        return Ok(None);
    };
    let value = selected.sample(0).ok_or_else(|| {
        ExprError::InvalidArgument(format!("transfer-function scalar '{name}' is unavailable"))
    })?;
    if !value.re.is_finite() || !value.im.is_finite() {
        return Err(ExprError::InvalidArgument(format!(
            "transfer-function scalar '{name}' is unbounded"
        )));
    }
    Ok(Some(value))
}

#[derive(Clone, Copy)]
pub(super) enum TransferColumn {
    Gain,
    InputImpedance,
    OutputImpedance,
}

impl TransferColumn {
    pub(super) fn sample(self, result: &TransferFunctionResult) -> Value {
        match self {
            Self::Gain => result.gain,
            Self::InputImpedance => result.input_impedance,
            Self::OutputImpedance => result.output_impedance,
        }
    }
}

pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    let ControlAnalysisResult::TransferFunction(result) = &dataset.result else {
        return None;
    };
    let lower = name.to_ascii_lowercase();
    let (column, signal, unit) = if matches!(lower.as_str(), "transfer_function" | "transfer_gain")
    {
        (
            TransferColumn::Gain,
            "transfer_gain",
            result.gain_unit.clone(),
        )
    } else if lower == "input_impedance"
        || lower == format!("{}#input_impedance", result.input.to_ascii_lowercase())
    {
        (
            TransferColumn::InputImpedance,
            "input_impedance",
            SignalUnit::Ohm,
        )
    } else if lower == "output_impedance"
        || lower == format!("output_impedance_at_{}", result.output.to_ascii_lowercase())
        || result
            .output
            .strip_prefix("I(")
            .and_then(|name| name.strip_suffix(')'))
            .is_some_and(|name| lower == format!("{}#output_impedance", name.to_ascii_lowercase()))
    {
        (
            TransferColumn::OutputImpedance,
            "output_impedance",
            SignalUnit::Ohm,
        )
    } else {
        return None;
    };
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: signal.into(),
        },
        column: Column::Transfer(column),
        unit,
        noise_probe: None,
    })
}

pub(super) fn output_probe<'a>(
    dataset: &'a ControlNamedDataset,
    args: &[Expr],
    line: usize,
) -> Result<Selected<'a>, ControlError> {
    if !(1..=2).contains(&args.len()) {
        return Err(command_error(
            line,
            "output impedance requires one or two probe nodes",
        ));
    }
    let mut nodes = Vec::with_capacity(args.len());
    for arg in args {
        nodes.push(match arg {
            Expr::Param(name) | Expr::StringLiteral(name) => name.clone(),
            Expr::Number(value) if *value == 0.0 => "0".into(),
            _ => {
                return Err(command_error(
                    line,
                    "output impedance requires the retained voltage probe",
                ));
            }
        });
    }
    let name = format!("output_impedance_at_v({})", nodes.join(","));
    select(dataset, &name).ok_or_else(|| unavailable(line, dataset, &name))
}

pub(super) fn unbounded(
    selected: &Selected<'_>,
    label: &str,
    position: usize,
    line: usize,
) -> Result<Option<ControlScalar>, ControlError> {
    if !matches!(selected.column, Column::Transfer(_)) {
        return Ok(None);
    }
    let Some(value) = selected.sample(0) else {
        return Ok(None);
    };
    if !value.re.is_infinite() || value.im != 0.0 {
        return Ok(None);
    }
    let reason = if value.re.is_sign_positive() {
        ScalarUnavailability::PositiveInfinity
    } else {
        ScalarUnavailability::NegativeInfinity
    };
    let scalar = ResultScalar::new(
        &selected.id.signal,
        label,
        Some(selected.unit.clone()),
        ScalarValue::Unavailable { reason },
    )
    .map_err(|error| command_error(line, error.to_string()))?;
    Ok(Some(ControlScalar {
        position,
        dataset: selected.dataset.name.clone(),
        scalar,
    }))
}
