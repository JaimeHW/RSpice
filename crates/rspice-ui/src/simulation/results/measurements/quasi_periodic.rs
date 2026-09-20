//! Stable lattice selection and native integrated-noise observations for studies.
use super::*;

pub(crate) fn parse_study_tuple(key: &str) -> Result<(Vec<i32>, &str, &str), String> {
    let mut parts = key.splitn(3, ':');
    let tuple = parts
        .next()
        .unwrap_or_default()
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Lattice coordinates must be comma-separated signed integers")?;
    let quantity = parts.next().unwrap_or_default();
    if !["real", "imag", "magnitude", "phase"]
        .iter()
        .any(|name| quantity.eq_ignore_ascii_case(name))
    {
        return Err("Lattice quantity must be real, imag, magnitude, or phase (degrees)".into());
    }
    let signal = parts
        .next()
        .filter(|signal| !signal.trim().is_empty())
        .ok_or("Lattice observations require an explicit signal")?;
    Ok((tuple, quantity, signal))
}

impl SimulationResult {
    pub(super) fn qpss_tuple_measurement(&self, key: &str) -> Option<f64> {
        let Self::Qpss {
            waveforms, tuples, ..
        } = self
        else {
            return None;
        };
        let (tuple, quantity, signal) = parse_study_tuple(key).ok()?;
        // Display spectra retain the nonnegative half of a real waveform's
        // conjugate-symmetric spectrum. A negative tuple selects its conjugate.
        let (index, sign) =
            if let Some(index) = tuples.iter().position(|candidate| *candidate == tuple) {
                (index, 1.0)
            } else {
                let opposite = tuple
                    .iter()
                    .map(|index| index.checked_neg())
                    .collect::<Option<Vec<_>>>()?;
                (
                    tuples.iter().position(|candidate| *candidate == opposite)?,
                    -1.0,
                )
            };
        let waveform = named_value(waveforms, signal)?;
        let real = *waveform.y_values.get(index)?;
        let imaginary = *waveform.y_imag.as_ref()?.get(index)? * sign;
        Some(match quantity.to_ascii_lowercase().as_str() {
            "real" => real,
            "imag" => imaginary,
            "magnitude" => real.hypot(imaginary),
            "phase" => imaginary.atan2(real).to_degrees(),
            _ => unreachable!(),
        })
    }

    pub(super) fn qpnoise_study_scalar(&self, key: &str) -> Option<f64> {
        use rspice_core::engine::QpnoiseValue;
        let Self::Qpnoise { response, .. } = self else {
            return None;
        };
        let (name, arguments) = key.split_once('(')?;
        let arguments = arguments.strip_suffix(')')?;
        let (output, source) = arguments
            .split_once(',')
            .map_or((arguments, None), |(output, source)| {
                (output, Some(source.trim()))
            });
        let output = output.trim().parse::<usize>().ok()?.checked_sub(1)?;
        let spectrum = response.outputs.get(output)?;
        let value = match name.to_ascii_lowercase().as_str() {
            "qpnoise.output_rms" if source.is_none() => spectrum.integrated.as_ref()?.output_rms,
            "qpnoise.input_rms" if source.is_none() => spectrum.integrated.as_ref()?.input_rms?,
            "qpnoise.contributor_rms" | "qpnoise.contributor_share_percent" => {
                let source = source?;
                let index = response
                    .sources
                    .iter()
                    .position(|candidate| candidate.name.eq_ignore_ascii_case(source))?;
                if name.eq_ignore_ascii_case("qpnoise.contributor_rms") {
                    *spectrum.integrated.as_ref()?.contributor_rms.get(index)?
                } else {
                    QpnoiseValue::Finite(
                        spectrum
                            .ranking
                            .as_ref()?
                            .iter()
                            .find(|rank| rank.source_index == index)?
                            .percentage,
                    )
                }
            }
            _ => return None,
        };
        match value {
            QpnoiseValue::Finite(value) if value.is_finite() => Some(value),
            _ => None,
        }
    }
}
