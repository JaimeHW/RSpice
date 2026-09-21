//! Periodic primitives of prescribed rates, with the SDT zero-time origin.
//!
//! These rates cannot respond to a circuit perturbation. Their primitives
//! therefore supply constraints rather than an unanchored DC rate row.
//! Rates reading circuit coordinates retain the ordinary continuous F/Q model.

use super::*;

#[derive(Debug)]
pub(in crate::analysis::harmonic_balance::solver) enum PrescribedIntegral {
    Primitive(Vec<Complex64>),
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
    pub(in crate::analysis::harmonic_balance::solver) fn value(
        &self,
        cycles: Value,
        retained: Value,
    ) -> Value {
        match self {
            Self::Primitive(spectrum) => primitive_value(spectrum, cycles),
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
    pub(in crate::analysis::harmonic_balance::solver) fn prepare_prescribed_integrals(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        max_values: usize,
        retained: bool,
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
        for plan in plans {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let Some(dependencies) = plan.dependencies() else {
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
                    .sample(cycles / frequency, |index| {
                        spectra[index]
                            .as_ref()
                            .expect("dependency qualified above")
                            .value(cycles, Value::NAN)
                    })
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
            spectra.push(Some(PrescribedIntegral::Primitive(coefficients)));
        }
        Ok(spectra)
    }
}
