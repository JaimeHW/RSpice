//! Periodic primitives of prescribed rates, with the SDT zero-time origin.
//!
//! These rates cannot respond to a circuit perturbation. Their primitives
//! therefore supply constraints rather than an unanchored DC rate row.
//! Driven node identities can also anchor the large-signal trajectory; retained
//! consumers restore their original continuous F/Q response to perturbations.

use super::*;

#[derive(Debug)]
pub(in crate::analysis::harmonic_balance::solver) enum PrescribedIntegral {
    Primitive(Vec<Complex64>),
    /// Fixed by independent circuit equations only for the carrier solve.
    Driven(Vec<Complex64>),
    /// The authenticated producer supplied the complete trajectory. Its
    /// constant must not be reset or its orbit re-solved by a linear consumer.
    Retained,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::behavioral::{BehavioralSources, BehavioralVoltageSource};

    #[test]
    fn prescribed_integral_preparation_cancels_during_collocation() {
        let sources = BehavioralSources {
            voltage_sources: vec![
                BehavioralVoltageSource::new("B1".into(), 1, 0, 1, "sdt(cos(2*pi*1k*time))")
                    .unwrap(),
            ],
            current_sources: vec![],
        };
        let mut solver = HbSolver::try_new(
            HbConfig::new(1e3)
                .with_harmonics(3)
                .with_collocation_points(129),
            1,
        )
        .unwrap();
        assert!(matches!(
            solver.prepare_prescribed_integrals(
                &sources,
                4096,
                false,
                &crate::abort_signal::CountingAbort::new(16)
            ),
            Err(HbError::Aborted)
        ));
        assert!(
            solver.prescribed_integrals.is_empty(),
            "cancelled preparation cannot publish a partial basis"
        );
    }
}

impl PrescribedIntegral {
    pub(in crate::analysis::harmonic_balance::solver) fn is_circuit_driven(&self) -> bool {
        matches!(self, Self::Driven(_))
    }
    pub(in crate::analysis::harmonic_balance::solver) fn value(
        &self,
        cycles: Value,
        retained: Value,
    ) -> Value {
        match self {
            Self::Primitive(spectrum) | Self::Driven(spectrum) => primitive_value(spectrum, cycles),
            Self::Retained => retained,
        }
    }
}

pub(in crate::analysis::harmonic_balance::solver) fn primitive_value(
    coefficients: &[Complex64],
    cycles: Value,
) -> Value {
    coefficients.iter().enumerate().skip(1).fold(
        coefficients[0].re,
        |value, (harmonic, coefficient)| {
            let (sin, cos) = (std::f64::consts::TAU * harmonic as Value * cycles).sin_cos();
            value + 2.0 * (coefficient.re * cos - coefficient.im * sin)
        },
    )
}

impl HbSolver {
    pub(in crate::analysis::harmonic_balance::solver) fn refresh_driven_integrals(
        &mut self,
        abort: &dyn AbortSignal,
    ) -> Result<(), HbError> {
        let Some(max_values) = self.periodic_integral_budget else {
            return Ok(());
        };
        if self
            .prescribed_integrals
            .iter()
            .all(|row| row.as_ref().is_some_and(|row| !row.is_circuit_driven()))
        {
            return Ok(());
        }
        // Registration may precede nonlinear device stamping. Only inspect
        // linear row closure once the complete device registry is available.
        let sources = std::mem::take(&mut self.behavioral_sources);
        let result = self
            .prepare_integrals_with_linear_coordinates(&sources, max_values, false, true, abort);
        self.behavioral_sources = sources;
        self.prescribed_integrals = result?;
        Ok(())
    }

    pub(in crate::analysis::harmonic_balance::solver) fn prepare_prescribed_integrals(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        max_values: usize,
        retained: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Option<PrescribedIntegral>>, HbError> {
        self.prepare_integrals_with_linear_coordinates(sources, max_values, retained, false, abort)
    }

    fn prepare_integrals_with_linear_coordinates(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        max_values: usize,
        retained: bool,
        linear_coordinates: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Option<PrescribedIntegral>>, HbError> {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let plans = sources
            .prescribed_integral_rates()
            .map_err(HbError::InvalidCircuit)?;
        let mut spectra: Vec<Option<PrescribedIntegral>> = Vec::new();
        spectra.try_reserve_exact(plans.len()).map_err(|e| {
            HbError::InvalidCircuit(format!("prescribed integral basis allocation failed: {e}"))
        })?;
        let samples = self.fft.size();
        let frequency = self.config.fundamental_freq;
        let mut retained_values = 0usize;
        let mut forced: Vec<Option<Vec<Value>>> = Vec::new();
        if !retained && plans.iter().any(|plan| plan.dependencies().is_none()) {
            let topology = self
                .num_nodes
                .saturating_add(self.exact_mna_branches().len())
                .saturating_add(1)
                .saturating_mul(8);
            let ensure = |values| {
                if values > max_values {
                    Err(HbError::InvalidCircuit(format!(
                        "driven integral exceeds the {max_values}-value periodic allocation limit"
                    )))
                } else {
                    Ok(())
                }
            };
            ensure(topology)?;
            let tree = self.forced_voltage_tree(true, abort)?;
            retained_values = topology.saturating_add(tree.len().saturating_mul(samples));
            ensure(
                retained_values
                    .saturating_add(2 * (self.num_harmonics + 1))
                    .saturating_add(samples),
            )?;
            forced.resize(self.num_nodes, None);
            for node in tree {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                let ExactMnaBranch::VoltageSource {
                    source: Some(source),
                    ..
                } = &self.exact_mna_branches()[node.branch]
                else {
                    unreachable!("authored voltage tree");
                };
                let spectrum: Vec<_> = (0..=self.num_harmonics)
                    .map(|k| Self::voltage_source_value_at_harmonic(source, k))
                    .collect();
                let mut values = Vec::with_capacity(samples);
                for sample in 0..samples {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    let parent = node
                        .parent
                        .map_or(0.0, |p| forced[p].as_ref().expect("tree order")[sample]);
                    let value = parent
                        + node.sign
                            * primitive_value(&spectrum, sample as Value / samples as Value);
                    if !value.is_finite() {
                        return Err(HbError::InvalidCircuit(
                            "driven node voltage overflowed during integral preparation".into(),
                        ));
                    }
                    values.push(value);
                }
                forced[node.node] = Some(values);
            }
        }
        if linear_coordinates
            && !retained
            && plans.iter().any(|plan| {
                plan.dependencies_with_coordinates(|i| forced.get(i).is_some_and(Option::is_some))
                    .is_none()
            })
        {
            let frequencies: Vec<_> = (0..=self.num_harmonics)
                .map(|k| frequency * k as Value)
                .collect();
            let coordinates = self
                .linear_driven_spectra(
                    sources,
                    &frequencies,
                    |row, k| {
                        if row < self.num_nodes {
                            self.source_spectra[row][k]
                        } else if let ExactMnaBranch::VoltageSource {
                            source: Some(source),
                            ..
                        } = &self.exact_mna_branches()[row - self.num_nodes]
                        {
                            Self::voltage_source_value_at_harmonic(source, k)
                        } else {
                            Complex64::ZERO
                        }
                    },
                    |row| {
                        forced.get(row).is_none_or(Option::is_none)
                            && plans
                                .iter()
                                .any(|plan| plan.coordinates().any(|index| index == row))
                    },
                    max_values
                        .saturating_sub(retained_values)
                        .saturating_sub(frequencies.len()),
                    abort,
                )
                .map_err(|error| match error {
                    crate::analysis::quasi_periodic::QuasiPeriodicError::Aborted => {
                        HbError::Aborted
                    }
                    other => {
                        HbError::InvalidCircuit(format!("driven integral preparation: {other}"))
                    }
                })?;
            let temporary = coordinates
                .len()
                .saturating_mul(4)
                .saturating_add(
                    coordinates
                        .iter()
                        .flatten()
                        .map(|row| row.len().saturating_mul(2))
                        .sum::<usize>(),
                )
                .saturating_add(frequencies.len());
            forced.resize(coordinates.len(), None);
            for (row, spectrum) in coordinates.iter().enumerate() {
                let Some(spectrum) = spectrum else { continue };
                if forced[row].is_some() {
                    continue;
                }
                retained_values = retained_values.saturating_add(samples);
                if retained_values.saturating_add(temporary) > max_values {
                    return Err(HbError::InvalidCircuit(format!(
                        "driven integral exceeds the {max_values}-value periodic allocation limit"
                    )));
                }
                let mut values = Vec::with_capacity(samples);
                for sample in 0..samples {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    let value = primitive_value(spectrum, sample as Value / samples as Value);
                    if !value.is_finite() {
                        return Err(HbError::InvalidCircuit(
                            "driven coordinate projection overflowed".into(),
                        ));
                    }
                    values.push(value);
                }
                forced[row] = Some(values);
            }
        }
        for plan in plans {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let Some(dependencies) =
                plan.dependencies_with_coordinates(|i| forced.get(i).is_some_and(Option::is_some))
            else {
                spectra.push(None);
                continue;
            };
            if dependencies
                .iter()
                .any(|&i| spectra.get(i).is_none_or(Option::is_none))
            {
                spectra.push(None);
                continue;
            }
            if retained {
                spectra.push(Some(PrescribedIntegral::Retained));
                continue;
            }
            let circuit_driven = plan.dependencies().is_none()
                || dependencies.iter().any(|&i| {
                    spectra[i]
                        .as_ref()
                        .is_some_and(PrescribedIntegral::is_circuit_driven)
                });
            retained_values = retained_values
                .checked_add(2 * (self.num_harmonics + 1))
                .ok_or_else(|| {
                    HbError::InvalidCircuit(
                        "prescribed integral spectrum count overflows this platform".into(),
                    )
                })?;
            let working_values = samples
                .checked_mul(3)
                .and_then(|count| count.checked_add(retained_values));
            if working_values.is_none_or(|count| count > max_values) {
                return Err(HbError::InvalidCircuit(format!(
                    "prescribed integral '{}' exceeds the {max_values}-value periodic allocation limit",
                    plan.name
                )));
            }
            let mut rates = Vec::new();
            rates.try_reserve_exact(samples).map_err(|e| {
                HbError::InvalidCircuit(format!("prescribed integral rate allocation failed: {e}"))
            })?;
            for sample in 0..samples {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                let cycles = sample as Value / samples as Value;
                let rate = plan
                    .sample_with_coordinates(
                        cycles / frequency,
                        &[],
                        |index| {
                            spectra[index]
                                .as_ref()
                                .expect("dependency qualified above")
                                .value(cycles, Value::NAN)
                        },
                        |index| forced[index].as_ref().expect("qualified coordinate")[sample],
                    )
                    .map_err(HbError::InvalidCircuit)?;
                rates.push(rate);
            }
            let mut coefficients = self.checked_periodic_spectrum(
                &rates,
                self.num_harmonics,
                "prescribed integral rate",
            )?;
            // The shared projection represents an identically zero waveform
            // with an empty vector. A primitive still owns its DC coordinate.
            if coefficients.is_empty() {
                coefficients.resize(self.num_harmonics + 1, Complex64::ZERO);
            }
            let scale = rates
                .iter()
                .fold(0.0_f64, |scale, value| scale.max(value.abs()));
            // Only FFT/evaluation roundoff may be discarded from the DC rate.
            // A nonlinear solver tolerance cannot authorize a secular drift.
            let roundoff = 64.0 * Value::EPSILON * (samples as Value).log2().max(1.0);
            if scale > 0.0 && coefficients[0].norm() / scale > roundoff {
                return Err(HbError::InvalidCircuit(format!(
                    "prescribed integral '{}' has nonzero mean input {:e}; its zero-origin primitive cannot be periodic",
                    plan.name, coefficients[0].re,
                )));
            }
            for (harmonic, coefficient) in coefficients.iter_mut().enumerate().skip(1) {
                let omega = std::f64::consts::TAU * frequency * harmonic as Value;
                if !omega.is_finite() || omega <= 0.0 {
                    return Err(HbError::InvalidCircuit(format!(
                        "prescribed integral '{}' has an unrepresentable angular frequency",
                        plan.name
                    )));
                }
                // Avoid squaring omega in generic complex division; valid
                // primitives can span much more than half the exponent range.
                *coefficient = Complex64::new(coefficient.im / omega, -coefficient.re / omega);
            }
            // SDT starts at zero. A sine rate consequently needs a nonzero
            // primitive mean; blindly setting the DC coefficient to zero is wrong.
            coefficients[0] = Complex64::new(
                -2.0 * coefficients[1..]
                    .iter()
                    .map(|value| value.re)
                    .sum::<Value>(),
                0.0,
            );
            if coefficients
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "prescribed integral '{}' has unrepresentable spectral values",
                    plan.name
                )));
            }
            spectra.push(Some(if circuit_driven {
                PrescribedIntegral::Driven(coefficients)
            } else {
                PrescribedIntegral::Primitive(coefficients)
            }));
        }
        Ok(spectra)
    }
}
