//! Deterministic source projection onto signed independent-tone tuples.
mod waveforms;

use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicTransform;
use crate::config::SpiceDialect;
use std::collections::BTreeMap;

pub(super) struct Projector<'a> {
    grid: Arc<QuasiPeriodicGrid>,
    transform: QuasiPeriodicTransform,
    dialect: SpiceDialect,
    abort: &'a dyn AbortSignal,
}

impl Projector<'_> {
    fn zero(&self) -> Vec<Complex64> {
        vec![Complex64::ZERO; self.grid.len()]
    }

    fn constant(&self, value: Value) -> Result<Vec<Complex64>, SimulationError> {
        if !value.is_finite() {
            return Err(invalid("source bias is not finite"));
        }
        let mut result = self.zero();
        result[self.grid.dc_index()] = Complex64::new(value, 0.0);
        Ok(result)
    }

    fn tuple(&self, frequency: Value) -> Result<Vec<i32>, SimulationError> {
        self.grid.clock_tuple(frequency).map_err(numerical_error)
    }

    fn add_cosine(
        &self,
        spectrum: &mut [Complex64],
        tuple: &[i32],
        amplitude: Value,
        phase: Value,
    ) -> Result<(), SimulationError> {
        if !amplitude.is_finite() || !phase.is_finite() {
            return Err(invalid("source amplitude or phase is not finite"));
        }
        if amplitude != 0.0 && amplitude * 0.5 == 0.0 {
            return Err(invalid(
                "source Fourier coefficient is below the representable range",
            ));
        }
        let index = self
            .grid
            .index_of(tuple)
            .ok_or_else(|| invalid("source tone tuple is outside the retained basis"))?;
        let value = Complex64::from_polar(amplitude * 0.5, phase);
        spectrum[index] += value;
        spectrum[self.grid.len() - 1 - index] += value.conj();
        Ok(())
    }

    fn ac(
        &self,
        dc: Value,
        magnitude: Value,
        phase: Value,
        tones: &[usize],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let mut result = self.constant(dc)?;
        for &tone in tones {
            let mut tuple = vec![0; self.grid.dimensions().len()];
            tuple[tone] = 1;
            self.add_cosine(&mut result, &tuple, magnitude, phase)?;
        }
        Ok(result)
    }

    fn project(
        &mut self,
        spec: &SourceSpec,
        tones: &[usize],
    ) -> Result<Vec<Complex64>, SimulationError> {
        check_abort(self.abort)?;
        let mut result = match spec {
            SourceSpec::Distortion { inner, .. } => self.project(inner, tones)?,
            SourceSpec::RfPort { inner, port } => {
                if let Some((amplitude, frequency, phase)) = port.drive_tone() {
                    if !tones.is_empty() {
                        return Err(invalid(
                            "an RF drive uses its authored clock; AC tone bindings would be ignored",
                        ));
                    }
                    let mut spectrum = self.project(inner, &[])?;
                    if amplitude != 0.0 {
                        self.add_cosine(&mut spectrum, &self.tuple(frequency)?, amplitude, phase)?;
                    }
                    spectrum
                } else {
                    self.project(inner, tones)?
                }
            }
            SourceSpec::Ac { magnitude, phase } => self.ac(0.0, *magnitude, *phase, tones)?,
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } => self.ac(*dc_value, *ac_magnitude, *ac_phase, tones)?,
            _ if !tones.is_empty() => {
                return Err(invalid(
                    "tone bindings require an AC-only source; a time waveform supplies its own clocks",
                ));
            }
            SourceSpec::Dc(value) => self.constant(*value)?,
            SourceSpec::AcTransient { transient, .. }
            | SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => self.project(transient, &[])?,
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                let mut spectrum = self.constant(*offset)?;
                if *amplitude != 0.0 {
                    if *damping != 0.0 {
                        return Err(invalid(
                            "a damped SIN has no stationary quasiperiodic drive",
                        ));
                    }
                    let phase = phase
                        - std::f64::consts::FRAC_PI_2
                        - std::f64::consts::TAU * frequency * delay;
                    self.add_cosine(&mut spectrum, &self.tuple(*frequency)?, *amplitude, phase)?;
                }
                spectrum
            }
            SourceSpec::Pulse { .. } => self.pulse(spec)?,
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => self.pwl(points, *delay, *repeat_from)?,
            SourceSpec::Sffm { .. } | SourceSpec::Am { .. } => self.modulated(spec)?,
            SourceSpec::PwlFile { .. } | SourceSpec::Pat { .. } => {
                return Err(SimulationError::unsupported_capability(
                    "analysis.qpss.source",
                    "QPSS periodic FILE/PAT source projection is not connected",
                ));
            }
            SourceSpec::Exp { .. } | SourceSpec::TrNoise { .. } | SourceSpec::TrRandom { .. } => {
                return Err(invalid(
                    "QPSS requires stationary deterministic source waveforms",
                ));
            }
        };
        // A real sampled source's FFT is conjugate symmetric up to roundoff.
        // Canonicalize each pair locally; no large DC coefficient can hide a
        // malformed AC value, and serialized spectra retain exact symmetry.
        for value in &result {
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(invalid("source spectrum overflowed"));
            }
        }
        let dc = self.grid.dc_index();
        result[dc].im = 0.0;
        for index in 0..dc {
            let other = result.len() - 1 - index;
            let conjugate = result[other].conj();
            let value = if result[index] == conjugate {
                result[index]
            } else {
                result[index] * 0.5 + conjugate * 0.5
            };
            result[index] = value;
            result[other] = value.conj();
        }
        Ok(result)
    }

    fn max_order(&self, tuple: &[i32]) -> usize {
        tuple
            .iter()
            .zip(&self.grid.config().harmonics)
            .filter_map(|(k, h)| (*k != 0).then(|| h / k.unsigned_abs() as usize))
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpss_source_projection_preserves_representable_subnormal_coefficients() {
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(
                QuasiPeriodicGridConfig::new(vec![1.0, std::f64::consts::SQRT_2], vec![1, 1]),
                &crate::ResourceLimits::default(),
                &NoAbort,
            )
            .unwrap(),
        );
        let mut projector = Projector {
            transform: QuasiPeriodicTransform::new_with_abort(grid.clone(), &NoAbort).unwrap(),
            grid: grid.clone(),
            dialect: SpiceDialect::BestAvailable,
            abort: &NoAbort,
        };
        let spectrum = projector
            .project(
                &SourceSpec::Ac {
                    magnitude: Value::from_bits(2),
                    phase: 0.0,
                },
                &[0],
            )
            .unwrap();
        assert_eq!(spectrum[grid.index_of(&[1, 0]).unwrap()].re.to_bits(), 1);
        assert_eq!(spectrum[grid.index_of(&[-1, 0]).unwrap()].re.to_bits(), 1);
        assert!(
            projector
                .project(
                    &SourceSpec::Ac {
                        magnitude: Value::from_bits(1),
                        phase: 0.0
                    },
                    &[0]
                )
                .is_err()
        );
    }
}

pub(super) fn build(
    engine: &Engine,
    circuit: &CircuitData,
    config: &QpssConfig,
    grid: Arc<QuasiPeriodicGrid>,
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<Complex64>>, SimulationError> {
    let mut bindings: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for binding in &config.source_tones {
        check_abort(abort)?;
        if binding.source.trim() != binding.source
            || binding.source.is_empty()
            || binding.tone >= grid.dimensions().len()
        {
            return Err(invalid(
                "source tone binding needs an exact source name and a valid zero-based tone index",
            ));
        }
        if !circuit
            .voltage_sources
            .names
            .iter()
            .chain(&circuit.current_sources.names)
            .any(|name| name.eq_ignore_ascii_case(&binding.source))
        {
            return Err(invalid(format!(
                "tone source '{}' does not exist",
                binding.source
            )));
        }
        let tones = bindings
            .entry(binding.source.to_ascii_lowercase())
            .or_default();
        if tones.contains(&binding.tone) {
            return Err(invalid("duplicate source/tone binding"));
        }
        tones.push(binding.tone);
    }
    let unknowns = circuit.num_nodes()
        + circuit.num_branches()
        + Engine::hb_periodic_extra_branch_count(circuit)?
        + circuit.behavioral_sources.integral_count()
        + circuit
            .capacitors
            .value_expressions
            .iter()
            .flatten()
            .count();
    let mut result = vec![vec![Complex64::ZERO; grid.len()]; unknowns];
    let mut projector = Projector {
        transform: QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)
            .map_err(numerical_error)?,
        grid,
        dialect: engine.config.spice_dialect,
        abort,
    };
    let voltage = &circuit.voltage_sources;
    let current = &circuit.current_sources;
    for (names, biases, amplitudes, phases, specs, pos, neg, is_voltage) in [
        (
            &voltage.names,
            &voltage.dc_values,
            &voltage.ac_magnitudes,
            &voltage.ac_phases,
            &voltage.source_specs,
            &voltage.node_pos,
            &voltage.node_neg,
            true,
        ),
        (
            &current.names,
            &current.dc_values,
            &current.ac_magnitudes,
            &current.ac_phases,
            &current.source_specs,
            &current.node_pos,
            &current.node_neg,
            false,
        ),
    ] {
        for (index, name) in names.iter().enumerate() {
            check_abort(abort)?;
            let tones = bindings
                .get(&name.to_ascii_lowercase())
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let fallback = SourceSpec::DcAc {
                dc_value: biases[index],
                ac_magnitude: amplitudes.get(index).copied().unwrap_or(0.0),
                ac_phase: phases.get(index).copied().unwrap_or(0.0),
            };
            let spec = specs
                .get(index)
                .and_then(Option::as_ref)
                .unwrap_or(&fallback);
            let spectrum = projector
                .project(spec, tones)
                .map_err(|error| match error {
                    SimulationError::Aborted => SimulationError::Aborted,
                    other => invalid(format!("source '{name}': {other}")),
                })?;
            let rows = if is_voltage {
                vec![(circuit.num_nodes() + voltage.branch_indices[index] - 1, 1.0)]
            } else {
                [(pos[index], -1.0), (neg[index], 1.0)]
                    .into_iter()
                    .filter_map(|(node, sign)| (node > 0).then(|| (node - 1, sign)))
                    .collect()
            };
            for (row, sign) in rows {
                for (value, coefficient) in result[row].iter_mut().zip(&spectrum) {
                    *value += *coefficient * sign;
                    if !value.re.is_finite() || !value.im.is_finite() {
                        return Err(invalid("source accumulation overflowed"));
                    }
                }
            }
        }
    }
    Ok(result)
}
