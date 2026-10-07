//! Noise vectors keep power density distinct from amplitude density.
use super::*;
use crate::analysis::noise::{
    NoiseContributionKind, NoiseContributionProbe, NoiseInputQuantity, NoiseResult,
};

#[derive(Clone, Copy)]
pub(super) enum NoiseColumn {
    OutputAmplitude,
    InputAmplitude,
    OutputDensity,
    InputDensity,
    GainSquared,
}

impl NoiseColumn {
    pub(super) fn sample(self, point: &NoiseResult) -> Value {
        match self {
            Self::OutputAmplitude => point.output_noise_rms(),
            Self::InputAmplitude => point.input_referred_rms(),
            Self::OutputDensity => point.output_noise_density,
            Self::InputDensity => point.input_referred_density,
            Self::GainSquared => point.input_gain_squared,
        }
    }

    fn unit(self, quantity: Option<NoiseInputQuantity>) -> SignalUnit {
        let current = quantity == Some(NoiseInputQuantity::Current);
        match self {
            Self::OutputAmplitude => {
                SignalUnit::Custom(NoiseInputQuantity::Voltage.amplitude_density_unit().into())
            }
            Self::InputAmplitude => SignalUnit::Custom(
                quantity
                    .unwrap_or(NoiseInputQuantity::Voltage)
                    .amplitude_density_unit()
                    .into(),
            ),
            Self::OutputDensity => SignalUnit::Custom("V^2/Hz".into()),
            Self::InputDensity => {
                SignalUnit::Custom(if current { "A^2/Hz" } else { "V^2/Hz" }.into())
            }
            Self::GainSquared if current => SignalUnit::Custom("ohm^2".into()),
            Self::GainSquared => SignalUnit::Dimensionless,
        }
    }
}

pub(super) fn select<'a>(
    dataset: &'a ControlNamedDataset,
    name: &str,
    line: usize,
) -> Result<Option<Selected<'a>>, ControlError> {
    let (ControlAnalysisResult::Noise(points)
    | ControlAnalysisResult::NoiseTable(FrequencyDataResult { points, .. })) = &dataset.result
    else {
        return Ok(None);
    };
    let (column, signal) = match name.to_ascii_lowercase().as_str() {
        "onoise_spectrum" => (NoiseColumn::OutputAmplitude, "onoise_spectrum"),
        "inoise_spectrum" => (NoiseColumn::InputAmplitude, "inoise_spectrum"),
        "onoise" | "output_noise_density" => (NoiseColumn::OutputDensity, "output_noise_density"),
        "inoise" | "input_referred_density" => {
            (NoiseColumn::InputDensity, "input_referred_density")
        }
        "input_gain_squared" => (NoiseColumn::GainSquared, "input_gain_squared"),
        _ => return Ok(None),
    };
    let first = points
        .first()
        .ok_or_else(|| unavailable(line, dataset, name))?;
    Ok(Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: signal.into(),
        },
        column: Column::Noise(column),
        unit: column.unit(first.input_quantity),
        noise_probe: None,
    }))
}

pub(super) fn contribution<'a>(
    dataset: &'a ControlNamedDataset,
    name: &str,
    args: &[Expr],
    line: usize,
) -> Result<Selected<'a>, ControlError> {
    let (ControlAnalysisResult::Noise(points)
    | ControlAnalysisResult::NoiseTable(FrequencyDataResult { points, .. })) = &dataset.result
    else {
        return Err(unavailable(line, dataset, name));
    };
    if !(1..=2).contains(&args.len()) {
        return Err(command_error(
            line,
            "DNO/DNI requires a device and an optional mechanism",
        ));
    }
    let raw = args
        .iter()
        .map(|arg| match arg {
            Expr::Param(value) | Expr::StringLiteral(value) => Ok(value.to_ascii_lowercase()),
            _ => Err(command_error(
                line,
                "DNO/DNI operands must name a device or mechanism",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let probe = NoiseContributionProbe {
        kind: if name == "DNI" {
            NoiseContributionKind::Input
        } else {
            NoiseContributionKind::Output
        },
        device: raw
            .first()
            .cloned()
            .ok_or_else(|| unavailable(line, dataset, name))?,
        mechanism: raw.get(1).cloned(),
    };
    let first = points
        .first()
        .ok_or_else(|| unavailable(line, dataset, name))?;
    first
        .contribution(&probe)
        .map_err(|error| command_error(line, error.to_string()))?;
    let unit = if probe.kind == NoiseContributionKind::Input {
        NoiseColumn::InputDensity
    } else {
        NoiseColumn::OutputDensity
    }
    .unit(first.input_quantity);
    Ok(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: format!("{}({})", name.to_ascii_lowercase(), raw.join(",")),
        },
        column: Column::NoiseContribution,
        unit,
        noise_probe: Some(probe),
    })
}
