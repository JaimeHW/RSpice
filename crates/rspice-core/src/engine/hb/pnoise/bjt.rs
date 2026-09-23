//! Native device noise mechanisms sampled on the authenticated physical orbit.

use super::*;
use crate::analysis::harmonic_balance::normalize_scaled_noise_waveform;
use crate::analysis::noise::{NoiseSource, NoiseSourceIdentity, NoiseSourceType};
use std::collections::HashMap;

pub(in crate::engine::hb) struct NativeNoiseWaveform {
    pub(in crate::engine::hb) name: String,
    pub(in crate::engine::hb) nodes: [usize; 2],
    pub(in crate::engine::hb) frequency_exponent: Option<Value>,
    pub(in crate::engine::hb) samples: Vec<ScaledNonnegative>,
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
        NoiseSourceType::Bsim3Flicker | NoiseSourceType::Bsim4Flicker => {
            if !source.ef.is_finite() || !temperature.is_finite() || temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise source '{name}' has invalid native MOS flicker parameters"
                )));
            }
            // Both strong/weak-inversion terms in b3noi.c/b4noi.c have the same
            // f^-EF dependence. Their harmonic mean therefore separates into
            // one bias-dependent amplitude and one stationary power law.
            let density = Engine::evaluated_noise_density(source, 1.0, temperature)?;
            checked_scaled_positive_product(&[density], name)
        }
        _ => Err(SimulationError::Circuit(format!(
            "pnoise native source '{name}' has an unsupported noise law"
        ))),
    }
}

/// Physical native mechanisms independent of the orbit's sampling coordinates.
pub(in crate::engine::hb) struct NativeNoiseWaveforms {
    indices: HashMap<(NoiseSourceIdentity, usize, usize), usize>,
    pub(in crate::engine::hb) waveforms: Vec<NativeNoiseWaveform>,
    elementary: Vec<NoiseSource>,
    temperatures: HashMap<NoiseSourceIdentity, Value>,
    ambient: Value,
    value_limit: usize,
}
impl NativeNoiseWaveforms {
    pub(in crate::engine::hb) fn new(ambient: Value, value_limit: usize) -> Self {
        Self {
            indices: HashMap::new(),
            waveforms: Vec::new(),
            elementary: Vec::new(),
            temperatures: HashMap::new(),
            ambient,
            value_limit,
        }
    }
    pub(in crate::engine::hb) fn sample(
        &mut self,
        engine: &Engine,
        time: usize,
        count: usize,
        devices: (
            &[crate::device::Bjt],
            &[crate::device::Bsim3v3Device],
            &[crate::device::Bsim4v8Device],
            &[crate::device::Mosfet],
        ),
        solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.elementary.clear();
        self.temperatures.clear();
        let (bjts, bsim3, bsim4, mosfets) = devices;
        for bjt in bjts {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if Engine::append_bjt_noise_sources(
                bjt,
                solution,
                None,
                solution.len(),
                &mut self.elementary,
                &mut self.temperatures,
            )? {
                return Err(SimulationError::Circuit(format!(
                    "BJT '{}' periodic noise requires unbound private coordinates",
                    bjt.name
                )));
            }
        }
        for device in bsim3 {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let sources = Engine::collect_bsim3v3_noise_sources(device)?;
            for source in &sources {
                self.temperatures
                    .insert(source.identity.clone(), device.core.model_temp.temp);
            }
            self.elementary.extend(sources);
        }
        for device in bsim4 {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let sources = Engine::collect_bsim4_periodic_noise_sources(device, solution)?;
            for source in &sources {
                self.temperatures
                    .insert(source.identity.clone(), device.core.model_temp.temp);
            }
            self.elementary.extend(sources);
        }
        for device in mosfets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            Engine::append_classic_mos_noise_sources(
                device,
                engine.config.spice_dialect,
                &mut self.elementary,
                &mut self.temperatures,
            )?;
        }
        Engine::configure_noise_physical_constants(
            &mut self.elementary,
            &mut [],
            engine.config.spice_dialect,
        );
        for source in &self.elementary {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let name = Engine::noise_source_label(&source.identity);
            let frequency_exponent = matches!(
                source.noise_type,
                NoiseSourceType::Flicker
                    | NoiseSourceType::Bsim3Flicker
                    | NoiseSourceType::Bsim4Flicker
            )
            .then_some(source.ef);
            let temperature = self
                .temperatures
                .get(&source.identity)
                .copied()
                .unwrap_or(self.ambient + source.temperature_offset);
            let density = scaled_native_density(source, temperature, &name)?;
            let key = (source.identity.clone(), source.node_pos, source.node_neg);
            let index = if let Some(&index) = self.indices.get(&key) {
                index
            } else {
                let values = self
                    .waveforms
                    .len()
                    .checked_add(1)
                    .and_then(|n| n.checked_mul(count))
                    .ok_or_else(|| {
                        SimulationError::Circuit("native noise waveform size overflows".into())
                    })?;
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::ResultValues,
                    values.saturating_mul(2),
                    self.value_limit,
                )?;
                let mut samples = Vec::new();
                samples.try_reserve_exact(count).map_err(|error| {
                    SimulationError::Circuit(format!(
                        "pnoise source '{name}' waveform allocation failed: {error}"
                    ))
                })?;
                samples.resize(count, ScaledNonnegative::ZERO);
                let index = self.waveforms.len();
                self.waveforms.push(NativeNoiseWaveform {
                    name,
                    nodes: [source.node_pos, source.node_neg],
                    frequency_exponent,
                    samples,
                });
                self.indices.insert(key, index);
                index
            };
            let waveform = &mut self.waveforms[index];
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
    }
}

impl Engine {
    pub(in crate::engine::hb) fn native_periodic_noise_sources(
        &self,
        solver: &mut HbSolver,
        state: &HbSolverState,
        ambient: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PeriodicNoiseSource>, SimulationError> {
        let mut frames =
            NativeNoiseWaveforms::new(ambient, self.config.resource_limits.max_result_values);
        solver
            .visit_native_noise_samples(state, abort, |time, count, devices, solution| {
                frames
                    .sample(self, time, count, devices, solution, abort)
                    .map_err(|error| match error {
                        SimulationError::Aborted => crate::analysis::HbError::Aborted,
                        error => crate::analysis::HbError::InvalidCircuit(error.to_string()),
                    })
            })
            .map_err(|error| match error {
                crate::analysis::HbError::Aborted => SimulationError::Aborted,
                error => SimulationError::Circuit(format!(
                    "native periodic-noise sampling failed: {error}"
                )),
            })?;

        let mut sources = Vec::new();
        for mut waveform in frames.waveforms {
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
