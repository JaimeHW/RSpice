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

/// ngspice's integrated noise (spicelib/analysis/noisean.c, each device's
/// INT_NOIZ pass): each source's output density is integrated over every
/// frequency step as a power law between its ends, the input-referred part
/// at the step's upper gain. Returns the output and input totals in RMS
/// (cktnoise.c:121 takes the root unless `sqrnoise` is set).
fn integrated(points: &[NoiseResult]) -> (Value, Value) {
    // noisedef.h:105 N_MINLOG and :106 N_MINGAIN.
    let ln = |density: Value| density.max(1e-38).ln();
    let (mut output, mut input) = (0.0, 0.0);
    for pair in points.windows(2) {
        let [last, point] = pair else { continue };
        let step = Step {
            del_freq: point.frequency - last.frequency,
            ln_freq: point.frequency.max(1e-38).ln(),
            ln_last_freq: last.frequency.max(1e-38).ln(),
        };
        let gain_inverse = 1.0 / point.input_gain_squared.max(1e-20);
        for (index, source) in point.contributions.iter().enumerate() {
            let previous = last
                .contributions
                .get(index)
                .filter(|previous| previous.identity == source.identity)
                .or_else(|| {
                    last.contributions
                        .iter()
                        .find(|previous| previous.identity == source.identity)
                });
            let Some(previous) = previous else { continue };
            let density = source.output_contribution;
            let step_output = step.integral(density, ln(density), ln(previous.output_contribution));
            output += step_output;
            input += gain_inverse * step_output;
        }
    }
    (output.sqrt(), input.sqrt())
}

/// One frequency step of the noise integration (ngspice `Ndata`).
struct Step {
    del_freq: Value,
    ln_freq: Value,
    ln_last_freq: Value,
}

impl Step {
    /// ngspice Nintegrate (spicelib/analysis/ninteg.c:27).
    fn integral(&self, density: Value, ln_density: Value, ln_last: Value) -> Value {
        // ninteg.c:24
        let limexp = |x: Value| {
            if x > 700.0 {
                700.0_f64.exp() * (1.0 + x - 700.0)
            } else {
                x.exp()
            }
        };
        let exponent = (ln_density - ln_last) / (self.ln_freq - self.ln_last_freq);
        // noisedef.h:109 N_INTFTHRESH and :113 N_INTUSELOG.
        if exponent.abs() < 1e-10 {
            return density * self.del_freq;
        }
        let a = limexp(ln_density - exponent * self.ln_freq);
        let exponent = exponent + 1.0;
        if exponent.abs() < 1e-10 {
            a * (self.ln_freq - self.ln_last_freq)
        } else {
            a * ((limexp(exponent * self.ln_freq) - limexp(exponent * self.ln_last_freq))
                / exponent)
        }
    }
}

impl ControlCircuit {
    /// ngspice's noise makes two plots: the spectrum (`noise1`, the dataset)
    /// and the integrated noise (`noise2`, frontend/outitf.c via
    /// noisean.c:285), which becomes the current plot. Its first vector,
    /// `onoise_total`, is its scale.
    pub(in crate::engine::control) fn integrated_noise_plot(
        &mut self,
        line: usize,
    ) -> Result<(), ControlError> {
        let Some(ControlAnalysisResult::Noise(points)) =
            self.datasets.last().map(|dataset| &dataset.result)
        else {
            return Ok(());
        };
        let (output, input) = integrated(points);
        let input_unit = match points.first().and_then(|point| point.input_quantity) {
            Some(NoiseInputQuantity::Current) => SignalUnit::Ampere,
            _ => SignalUnit::Volt,
        };
        let totals = [
            ("onoise_total", output, SignalUnit::Volt),
            ("inoise_total", input, input_unit),
        ];
        let name = self.new_plot_name("noise", line)?;
        self.vectors
            .new_analysis_plot(name, self.datasets.len(), "Integrated Noise");
        for (vector, value, unit) in totals {
            let value = VectorValue {
                samples: vec![value.into()],
                complex: false,
                scale: None,
                unit,
            };
            self.push_vector(vector, value, false);
        }
        Ok(())
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
