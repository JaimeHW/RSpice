//! Continuous Fourier pulse/PWL coefficients and independent modulation phases.
use super::*;

fn angle(tuple: &[i32], phases: &[Value]) -> Value {
    tuple
        .iter()
        .zip(phases)
        .map(|(k, phase)| *k as Value * phase)
        .sum()
}

impl Projector<'_> {
    pub(super) fn pulse(&mut self, spec: &SourceSpec) -> Result<Vec<Complex64>, SimulationError> {
        let SourceSpec::Pulse { v1, v2, period, .. } = spec else {
            unreachable!()
        };
        if v1 == v2 {
            return self.constant(*v1);
        }
        let frequency = period.recip();
        let tuple = self.tuple(frequency)?;
        let config = HbConfig::new(frequency).with_harmonics(self.max_order(&tuple));
        // QPSS uses continuous Fourier coefficients for piecewise-linear
        // drives. It is not Xyce's common-time APFT transform, even when the
        // engine selects Xyce device models.
        let projected = Engine::hb_source_spectrum(
            0.0,
            0.0,
            0.0,
            Some(spec),
            &config,
            &[1],
            SpiceDialect::BestAvailable,
        )?;
        let mut result = self.constant(projected.dc)?;
        for (order, amplitude, phase) in projected.harmonics {
            let harmonic: Vec<_> = tuple.iter().map(|k| k * order as i32).collect();
            if self.grid.index_of(&harmonic).is_some() {
                self.add_cosine(&mut result, &harmonic, amplitude, phase)?;
            }
        }
        Ok(result)
    }

    pub(super) fn pwl(
        &self,
        points: &[(Value, Value)],
        delay: Value,
        repeat_from: Option<Value>,
    ) -> Result<Vec<Complex64>, SimulationError> {
        if points.is_empty()
            || !delay.is_finite()
            || points.iter().any(|(t, v)| !t.is_finite() || !v.is_finite())
            || points.windows(2).any(|pair| pair[0].0 > pair[1].0)
        {
            return Err(invalid(
                "PWL requires finite values and nondecreasing knot times",
            ));
        }
        if points.iter().all(|(_, value)| *value == points[0].1) {
            return self.constant(points[0].1);
        }
        let start = repeat_from
            .ok_or_else(|| invalid("nonconstant PWL needs an explicit repeat interval for QPSS"))?;
        let end = points.last().unwrap().0;
        if !start.is_finite() || start < points[0].0 || start >= end {
            return Err(invalid(
                "PWL repeat origin must lie between its first and last knots",
            ));
        }
        let period = end - start;
        let tuple = self.tuple(period.recip())?;
        let left = points.iter().rposition(|(t, _)| *t <= start).unwrap();
        let initial = if points[left].0 == start {
            points[left].1
        } else {
            let (t0, v0) = points[left];
            let (t1, v1) = points[left + 1];
            v0 + (v1 - v0) * ((start - t0) / (t1 - t0))
        };
        let mut cycle = vec![(0.0, initial)];
        cycle.extend(
            points
                .iter()
                .filter(|(t, _)| *t > start)
                .map(|(t, v)| (t - start, *v)),
        );
        let dc = cycle
            .windows(2)
            .map(|pair| (pair[0].1 * 0.5 + pair[1].1 * 0.5) * ((pair[1].0 - pair[0].0) / period))
            .sum();
        let mut result = self.constant(dc)?;
        for order in 1..=self.max_order(&tuple) {
            check_abort(self.abort)?;
            let harmonic: Vec<_> = tuple.iter().map(|k| k * order as i32).collect();
            let Some(index) = self.grid.index_of(&harmonic) else {
                continue;
            };
            let omega = std::f64::consts::TAU * order as Value / period;
            let mut derivative = Complex64::new(initial - cycle.last().unwrap().1, 0.0);
            for pair in cycle.windows(2) {
                let (time, value) = pair[0];
                let (next, next_value) = pair[1];
                let half_angle = omega * (next - time) * 0.5;
                let sinc = if half_angle == 0.0 {
                    1.0
                } else {
                    half_angle.sin() / half_angle
                };
                derivative += Complex64::from_polar(
                    (next_value - value) * sinc,
                    -omega * (time * 0.5 + next * 0.5),
                );
            }
            let phase = -omega * (delay + start).rem_euclid(period);
            let coefficient = derivative
                / Complex64::new(0.0, std::f64::consts::TAU * order as Value)
                * Complex64::from_polar(1.0, phase);
            result[index] = coefficient;
            let conjugate = result.len() - 1 - index;
            result[conjugate] = coefficient.conj();
        }
        Ok(result)
    }

    pub(super) fn modulated(
        &mut self,
        spec: &SourceSpec,
    ) -> Result<Vec<Complex64>, SimulationError> {
        let (offset, amplitude, envelope_offset, fc, fm, phase_c, phase_m, modulation_index) =
            match spec {
                SourceSpec::Am {
                    offset,
                    modulation_offset,
                    modulation_amplitude,
                    carrier_freq,
                    modulating_freq,
                    delay,
                    phase_modulation,
                    phase_carrier,
                } => (
                    *offset,
                    *modulation_amplitude,
                    *modulation_offset,
                    *carrier_freq,
                    *modulating_freq,
                    phase_carrier.to_radians() - std::f64::consts::TAU * carrier_freq * delay,
                    phase_modulation.to_radians() - std::f64::consts::TAU * modulating_freq * delay,
                    None,
                ),
                SourceSpec::Sffm {
                    offset,
                    amplitude,
                    carrier_freq,
                    signal_freq,
                    delay,
                    phase_modulation,
                    phase_carrier,
                    modulation_index,
                } => {
                    let xyce = self.dialect == SpiceDialect::Xyce;
                    let ratio = carrier_freq / signal_freq;
                    let mdi = if xyce {
                        if modulation_index.is_nan() {
                            0.0
                        } else {
                            *modulation_index
                        }
                    } else if modulation_index.is_nan() {
                        90.0_f64.min(ratio)
                    } else if *modulation_index > ratio {
                        ratio
                    } else {
                        modulation_index.max(0.0)
                    };
                    (
                        *offset,
                        *amplitude,
                        0.0,
                        *carrier_freq,
                        *signal_freq,
                        if xyce {
                            0.0
                        } else {
                            phase_carrier.to_radians()
                                - std::f64::consts::TAU * carrier_freq * delay
                        },
                        if xyce {
                            0.0
                        } else {
                            phase_modulation.to_radians()
                                - std::f64::consts::TAU * signal_freq * delay
                        },
                        Some(mdi),
                    )
                }
                _ => unreachable!(),
            };
        if [offset, amplitude, envelope_offset, fc, fm, phase_c, phase_m]
            .iter()
            .any(|v| !v.is_finite())
            || modulation_index.is_some_and(|value| !value.is_finite())
        {
            return Err(invalid(
                "modulated QPSS sources require explicit finite frequencies, levels, delay and phases",
            ));
        }
        let carrier = self.tuple(fc)?;
        let modulation = self.tuple(fm)?;
        let mut samples = Vec::with_capacity(self.grid.sample_count());
        for index in 0..self.grid.sample_count() {
            if index.is_multiple_of(64) {
                check_abort(self.abort)?;
            }
            let phases = self.grid.phases(index).unwrap();
            let ac = angle(&carrier, &phases) + phase_c;
            let am = angle(&modulation, &phases) + phase_m;
            let value = if let Some(mdi) = modulation_index {
                offset + amplitude * (ac + mdi * am.sin()).sin()
            } else {
                offset + (envelope_offset + amplitude * am.sin()) * ac.sin()
            };
            samples.push(Complex64::new(value, 0.0));
        }
        self.transform
            .to_spectrum_with_abort(&samples, self.abort)
            .map_err(numerical_error)
    }
}
