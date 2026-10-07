//! The selected nominal probe and SPICE parameter-derivative vectors.
use super::*;
use crate::netlist::expr::ExprError;

#[derive(Clone, Copy)]
pub(super) enum SensitivityColumn {
    Output,
    Derivative(usize),
}

impl SensitivityColumn {
    pub(super) fn sample(self, result: &SensitivityCardResult, row: usize) -> Option<ComplexValue> {
        match result {
            SensitivityCardResult::Dc(result) if row == 0 => match self {
                Self::Output => Some(result.output_value.into()),
                Self::Derivative(index) => result
                    .sensitivities
                    .get(index)
                    .map(|value| value.absolute.into()),
            },
            SensitivityCardResult::Dc(_) => None,
            SensitivityCardResult::Ac(result) => match self {
                Self::Output => result.output_values.get(row).copied(),
                Self::Derivative(index) => {
                    result.sensitivities.get(index)?.absolute.get(row).copied()
                }
            },
        }
    }
}

pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    let ControlAnalysisResult::Sensitivity(result) = &dataset.result else {
        return None;
    };
    let (index, output_unit) = match result.as_ref() {
        SensitivityCardResult::Dc(result) => (
            result
                .sensitivities
                .iter()
                .position(|trace| trace.vector_name.eq_ignore_ascii_case(name)),
            &result.output_unit,
        ),
        SensitivityCardResult::Ac(result) => (
            result
                .sensitivities
                .iter()
                .position(|trace| trace.vector_name.eq_ignore_ascii_case(name)),
            &result.output_unit,
        ),
    };
    let (column, signal, unit) = if let Some(index) = index {
        // Native parameter units are not inferred from device/vector names.
        (
            SensitivityColumn::Derivative(index),
            name.to_ascii_lowercase(),
            SignalUnit::Unspecified,
        )
    } else if name.eq_ignore_ascii_case("output") || name.eq_ignore_ascii_case("output_value") {
        (
            SensitivityColumn::Output,
            "output".into(),
            output_unit.clone(),
        )
    } else {
        return None;
    };
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal,
        },
        column: Column::Sensitivity(column),
        unit,
        noise_probe: None,
    })
}

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
    if dataset.length() != 1 {
        return Err(ExprError::InvalidArgument(format!(
            "sensitivity vector '{name}' has multiple samples; a scalar requires one sample"
        )));
    }
    selected
        .sample(0)
        .filter(|value| value.re.is_finite() && value.im.is_finite())
        .map(Some)
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!("sensitivity scalar '{name}' is unavailable"))
        })
}
