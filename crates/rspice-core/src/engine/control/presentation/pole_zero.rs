//! Scalar views of retained roots and transfer limits.
use super::*;
use crate::analysis::PoleZeroResult;
use crate::netlist::expr::ExprError;

#[derive(Clone, Copy)]
pub(super) enum PoleZeroColumn {
    Pole(usize),
    Zero(usize),
    DcGain,
    HighFrequencyGain,
}

impl PoleZeroColumn {
    pub(super) fn sample(self, result: &PoleZeroResult) -> Option<ComplexValue> {
        match self {
            Self::Pole(index) => result.poles.get(index).copied(),
            Self::Zero(index) => result.zeros.get(index).copied(),
            Self::DcGain => result.dc_gain.map(Into::into),
            Self::HighFrequencyGain => result.hf_gain.map(Into::into),
        }
    }
}

pub(super) fn select_gain<'a>(
    dataset: &'a ControlNamedDataset,
    name: &str,
) -> Option<Selected<'a>> {
    let ControlAnalysisResult::PoleZero(result) = &dataset.result else {
        return None;
    };
    let (column, signal) = match name.to_ascii_lowercase().as_str() {
        "dc_gain" => (PoleZeroColumn::DcGain, "dc_gain"),
        "hf_gain" | "high_frequency_gain" => (PoleZeroColumn::HighFrequencyGain, "hf_gain"),
        _ => return None,
    };
    Some(selected(
        dataset,
        column,
        signal.into(),
        result.gain_unit.clone(),
    ))
}

fn selected(
    dataset: &ControlNamedDataset,
    column: PoleZeroColumn,
    signal: String,
    unit: SignalUnit,
) -> Selected<'_> {
    Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal,
        },
        column: Column::PoleZero(column),
        unit,
        noise_probe: None,
    }
}

pub(super) fn select_root<'a>(
    dataset: &'a ControlNamedDataset,
    name: &str,
    args: &[ComplexValue],
) -> Result<Selected<'a>, ExprError> {
    let [index] = args else {
        return Err(ExprError::WrongArgCount(name.into()));
    };
    if !index.re.is_finite()
        || index.im != 0.0
        || index.re < 1.0
        || index.re.fract() != 0.0
        || index.re >= 2.0_f64.powi(usize::BITS as i32)
    {
        return Err(ExprError::InvalidArgument(format!(
            "{name} requires a positive real integer root index"
        )));
    }
    let index = index.re as usize - 1;
    let ControlAnalysisResult::PoleZero(result) = &dataset.result else {
        return Err(ExprError::InvalidArgument(format!(
            "{} is not a pole-zero dataset",
            dataset.name
        )));
    };
    let (column, roots, kind) = match name {
        "POLE" => (PoleZeroColumn::Pole(index), &result.poles, "pole"),
        "ZERO" => (PoleZeroColumn::Zero(index), &result.zeros, "zero"),
        _ => return Err(ExprError::UnknownFunction(name.into())),
    };
    if index >= roots.len() {
        return Err(ExprError::InvalidArgument(format!(
            "{}.{kind}({}) is unavailable in the retained result",
            dataset.name,
            index + 1
        )));
    }
    Ok(selected(
        dataset,
        column,
        format!("{kind}({})", index + 1),
        SignalUnit::RadianPerSecond,
    ))
}

fn finite(selected: Selected<'_>) -> Result<ComplexValue, ExprError> {
    selected
        .sample(0)
        .filter(|value| value.re.is_finite() && value.im.is_finite())
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!(
                "pole-zero scalar '{}.{}' has no finite value",
                selected.dataset.name, selected.id.signal
            ))
        })
}

pub(in crate::engine::control) fn resolve_scalar(
    circuit: &ControlCircuit,
    name: &str,
) -> Result<Option<ComplexValue>, ExprError> {
    let Ok((dataset, raw)) = circuit.qualified(name, 0) else {
        return Ok(None);
    };
    select_gain(dataset, raw).map(finite).transpose()
}

pub(in crate::engine::control) fn resolve_function(
    circuit: &ControlCircuit,
    name: &str,
    args: &[ComplexValue],
) -> Result<Option<ComplexValue>, ExprError> {
    if !matches!(name.rsplit('.').next(), Some("POLE" | "ZERO")) {
        return Ok(None);
    }
    let (dataset, raw) = circuit
        .qualified(name, 0)
        .map_err(|error| ExprError::InvalidArgument(error.message))?;
    select_root(dataset, raw, args).and_then(finite).map(Some)
}
