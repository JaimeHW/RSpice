//! Native BJT noise mechanisms sampled on the authenticated physical orbit.

use super::*;
use crate::analysis::harmonic_balance::normalize_scaled_noise_waveform;
use crate::analysis::noise::{NoiseSource, NoiseSourceIdentity, NoiseSourceType};
use std::collections::HashMap;

struct NativeNoiseWaveform {
    name: String,
    nodes: [usize; 2],
    frequency_exponent: Option<Value>,
    samples: Vec<ScaledNonnegative>,
}

/// The native stationary source law at 1 Hz, without materializing a density
/// that could overflow/underflow before it is combined with circuit gain.
fn scaled_native_density(
    source: &NoiseSource,
    temperature: Value,
    name: &str,
) -> Result<ScaledNonnegative, SimulationError> {
    match source.noise_type {
        NoiseSourceType::Thermal => {
            if !temperature.is_finite() || temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise source '{name}' has invalid absolute temperature {temperature} K"
                )));
            }
            checked_scaled_positive_ratio(
                &[4.0, source.physical_constants.boltzmann, temperature],
                source.parameter,
                name,
            )
        }
        NoiseSourceType::Shot => checked_scaled_positive_product(
            &[
                2.0,
                source.physical_constants.electron_charge,
                source.parameter,
            ],
            name,
        ),
        NoiseSourceType::Flicker => {
            if !source.af.is_finite() || !source.ef.is_finite() || !source.current.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "pnoise source '{name}' has invalid flicker parameters"
                )));
            }
            let current = source.current.abs();
            let power = current.powf(source.af);
            let mut density = if power.is_normal() || (power.is_finite() && current == 0.0) {
                checked_scaled_positive_product(&[source.parameter, power], name)?
            } else {
                let logarithm = source.af * current.log2();
                let exponent = logarithm.floor();
                if !exponent.is_finite()
                    || exponent < i32::MIN as Value
                    || exponent > i32::MAX as Value
                {
                    return Err(SimulationError::Circuit(format!(
                        "pnoise source '{name}' current power exceeds the supported binary range"
                    )));
                }
                let mut scaled = checked_scaled_positive_product(
                    &[source.parameter, (logarithm - exponent).exp2()],
                    name,
                )?;
                scaled.exponent =
                    scaled
                        .exponent
                        .checked_add(exponent as i32)
                        .ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "pnoise source '{name}' current exponent overflows"
                            ))
                        })?;
                scaled
            };
            density.exponent = density
                .exponent
                .checked_add(source.parameter_exponent)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise source '{name}' coefficient exponent overflows"
                    ))
                })?;
            Ok(density)
        }
        _ => Err(SimulationError::Circuit(format!(
            "pnoise native BJT source '{name}' has an unsupported noise law"
        ))),
    }
}

impl Engine {
    pub(in crate::engine::hb) fn native_bjt_periodic_noise_sources(
        &self,
        solver: &mut HbSolver,
        state: &HbSolverState,
        ambient: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PeriodicNoiseSource>, SimulationError> {
        let mut indices: HashMap<(NoiseSourceIdentity, usize, usize), usize> = HashMap::new();
        let mut waveforms: Vec<NativeNoiseWaveform> = Vec::new();
        let mut elementary = Vec::new();
        let mut temperatures = HashMap::new();
        solver.visit_native_bjt_samples(state, abort, |time, count, bjts, solution| {
            let mut sample = || -> Result<(), SimulationError> {
                elementary.clear();
                temperatures.clear();
                for bjt in bjts {
                    if Self::append_bjt_noise_sources(
                        bjt,
                        solution,
                        None,
                        solution.len(),
                        &mut elementary,
                        &mut temperatures,
                    )? {
                        return Err(SimulationError::Circuit(format!(
                            "BJT '{}' periodic noise requires unbound private coordinates",
                            bjt.name
                        )));
                    }
                }
                Self::configure_noise_physical_constants(
                    &mut elementary,
                    &mut [],
                    self.config.spice_dialect,
                );
                for source in &elementary {
                    let name = Self::noise_source_label(&source.identity);
                    let frequency_exponent =
                        (source.noise_type == NoiseSourceType::Flicker).then_some(source.ef);
                    let temperature = temperatures
                        .get(&source.identity)
                        .copied()
                        .unwrap_or(ambient + source.temperature_offset);
                    let density = scaled_native_density(source, temperature, &name)?;
                    let key = (source.identity.clone(), source.node_pos, source.node_neg);
                    let index = if let Some(&index) = indices.get(&key) {
                        index
                    } else {
                        let values = waveforms
                            .len()
                            .checked_add(1)
                            .and_then(|n| n.checked_mul(count))
                            .ok_or_else(|| {
                                SimulationError::Circuit(
                                    "native BJT noise waveform size overflows".into(),
                                )
                            })?;
                        self.ensure_result_values(values)?;
                        let mut samples = Vec::new();
                        samples.try_reserve_exact(count).map_err(|error| {
                            SimulationError::Circuit(format!(
                                "pnoise source '{name}' waveform allocation failed: {error}"
                            ))
                        })?;
                        samples.resize(count, ScaledNonnegative::ZERO);
                        let index = waveforms.len();
                        waveforms.push(NativeNoiseWaveform {
                            name,
                            nodes: [source.node_pos, source.node_neg],
                            frequency_exponent,
                            samples,
                        });
                        indices.insert(key, index);
                        index
                    };
                    let waveform = &mut waveforms[index];
                    if waveform.nodes != [source.node_pos, source.node_neg]
                        || waveform.frequency_exponent != frequency_exponent
                    {
                        return Err(SimulationError::Circuit(format!(
                            "pnoise source '{}' changes its terminals or frequency law over the orbit",
                            waveform.name
                        )));
                    }
                    waveform.samples[time] = density;
                }
                Ok(())
            };
            sample().map_err(|error| crate::analysis::HbError::InvalidCircuit(error.to_string()))
        }).map_err(|error| match error {
            crate::analysis::HbError::Aborted => SimulationError::Aborted,
            error => SimulationError::Circuit(format!("native BJT periodic-noise sampling failed: {error}")),
        })?;

        let mut sources = Vec::new();
        for mut waveform in waveforms {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if waveform.samples.iter().all(|sample| sample.mantissa == 0.0) {
                continue;
            }
            if waveform.frequency_exponent.is_some() {
                // Native junction flicker_noise is a positive noise-power law.
                // Its square root modulates one stationary colored process;
                // Fourier-transforming the power instead loses correlations.
                for sample in &mut waveform.samples {
                    sample.mantissa = (sample.mantissa
                        * if sample.exponent.rem_euclid(2) == 0 {
                            1.0
                        } else {
                            2.0
                        })
                    .sqrt();
                    sample.exponent = sample.exponent.div_euclid(2);
                }
            }
            let (samples, exponent) =
                normalize_scaled_noise_waveform(&waveform.samples).map_err(|error| {
                    SimulationError::Circuit(format!("pnoise source '{}': {error}", waveform.name))
                })?;
            // Native noise laws can generate harmonics above the voltage
            // basis. Keep the complete usable collocation spectrum, including
            // colored modulation beyond the requested conversion window.
            let spectrum = solver
                .checked_periodic_spectrum(&samples, (samples.len() - 1) / 2, &waveform.name)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            let mut source = PeriodicNoiseSource {
                name: waveform.name,
                node_pos: waveform.nodes[0].checked_sub(1).unwrap_or(usize::MAX),
                node_neg: waveform.nodes[1].checked_sub(1).unwrap_or(usize::MAX),
                psd: spectrum,
                binary_scale_exponent: exponent,
                flicker: None,
            };
            if let Some(frequency_exponent) = waveform.frequency_exponent {
                source.binary_scale_exponent = exponent.checked_mul(2).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise source '{}' amplitude exponent overflows",
                        source.name
                    ))
                })?;
                source.flicker = Some(PeriodicFlickerNoise {
                    coefficient: 1.0,
                    exponent: frequency_exponent,
                    modulation: std::mem::replace(&mut source.psd, vec![Complex64::new(0.0, 0.0)]),
                });
            }
            sources.push(source);
        }
        Ok(sources)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_flicker_power_retains_range_before_coefficient_and_gain() {
        // The current square itself overflows or underflows binary64. Both
        // complete source densities are nevertheless exactly representable.
        for sign in [-1.0, 1.0] {
            for exponent in [-600, 600] {
                let mut source = NoiseSource::flicker_with_frequency_exponent(
                    "Q1".into(),
                    1,
                    0,
                    2.0_f64.powi(-exponent),
                    2.0,
                    0.7,
                    sign * 2.0_f64.powi(exponent),
                );
                source.parameter_exponent = -exponent / 3 * 2;
                let density = scaled_native_density(&source, 300.15, "Q1:FN").unwrap();
                assert_eq!(density.mantissa, 1.0);
                assert_eq!(density.exponent, exponent / 3);
                source.parameter = 1.0;
                source.parameter_exponent = 0;
                let density = scaled_native_density(&source, 300.15, "Q1:FN").unwrap();
                assert_eq!(density.mantissa, 1.0);
                assert_eq!(density.exponent, 2 * exponent);
            }
        }
    }
}
