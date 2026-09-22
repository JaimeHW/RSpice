//! Periodic primitives of prescribed rates, with the SDT zero-time origin.
//!
//! These rates cannot respond to a circuit perturbation. Their primitives
//! therefore supply constraints rather than an unanchored DC rate row.
//! Driven node identities can also anchor the large-signal trajectory; retained
//! consumers restore their original continuous F/Q response to perturbations.

use super::forced::coordinate_group;
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
    fn hb_prescribed_integral_residual_preserves_tight_tolerance_at_high_frequency() {
        let rate = 1e9;
        let mut config = HbConfig::new(rate)
            .with_harmonics(3)
            .with_collocation_points(17);
        config.tolerance = 1e-10;
        config.abstol = 1e-14;
        let mut solver = HbSolver::new(config, 1);
        solver
            .try_add_periodic_constitutive_port_branch(1, 0, 1, "BP")
            .unwrap();
        solver
            .try_add_exact_mna_static_entry(1, 0, 1.0, "BP")
            .unwrap();
        solver.add_resistor(1, 0, 1e3);
        let sources = BehavioralSources {
            voltage_sources: vec![
                BehavioralVoltageSource::new(
                    "BP".into(),
                    1,
                    0,
                    1,
                    &format!("sdt(2*pi*{rate}*sin(2*pi*{rate}*time))"),
                )
                .unwrap(),
            ],
            current_sources: vec![],
        };
        solver
            .set_periodic_behavioral_sources(&sources, false, 1_000_000, false, &NoAbort)
            .unwrap();
        let mut state = HbSolverState::new(1, 3);
        solver
            .solve_newton_with_abort(&mut state, &NoAbort)
            .unwrap();
        assert!((state.x[0][0].re - 1.0).abs() < 1e-10);
        assert!((state.x[0][1] + 0.5).norm() < 1e-10);
        assert!(state.x[0][2..].iter().all(|value| value.norm() < 1e-10));
        // The row retains rate units and both contribution magnitudes. A
        // perturbation at an otherwise absent harmonic must remain visible.
        state.mna_branch_currents[1][2] += 1e-6;
        state.mna_branch_residual[1].fill(Complex64::ZERO);
        state.mna_branch_residual_scale[1].fill(0.0);
        solver.add_prescribed_integral_residual(&mut state).unwrap();
        assert!((state.mna_branch_residual[1][2].re + 1e3).abs() < 1e-7);
        assert!((state.mna_branch_residual_scale[1][2] - 1e3).abs() < 1e-7);
        assert!(!state.rows_converged(1e-10, 1e-14));
    }

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
    pub(in crate::analysis::harmonic_balance::solver) fn add_prescribed_integral_residual(
        &self,
        state: &mut HbSolverState,
    ) -> Result<(), HbError> {
        let physical = self.physical_branch_count();
        let frequency = self.config.fundamental_freq;
        for (index, prescribed) in self.prescribed_integrals.iter().enumerate() {
            let Some(prescribed) = prescribed else {
                continue;
            };
            let row = physical + index;
            let actual = &state.mna_branch_currents[row];
            let target = match prescribed {
                PrescribedIntegral::Primitive(coefficients)
                | PrescribedIntegral::Driven(coefficients) => coefficients,
                PrescribedIntegral::Retained => actual,
            };
            if actual.len() != self.num_harmonics + 1 || target.len() != actual.len() {
                return Err(HbError::InvalidCircuit(
                    "prescribed integral spectral basis is inconsistent".into(),
                ));
            }
            // Both trajectories are already represented in this exact basis.
            // A trigonometric evaluation and FFT roundtrip can leave spurious
            // rate residuals in empty harmonics, especially at high frequency.
            // Keep the original inverse-time row scale and each term's norm.
            for (harmonic, (&target, &actual)) in target.iter().zip(actual).enumerate() {
                let left = frequency * target;
                let right = frequency * actual;
                let residual = &mut state.mna_branch_residual[row][harmonic];
                let scale = &mut state.mna_branch_residual_scale[row][harmonic];
                *residual += left - right;
                *scale += left.norm() + right.norm();
                if !residual.re.is_finite() || !residual.im.is_finite() || !scale.is_finite() {
                    return Err(HbError::InvalidCircuit(
                        "prescribed integral spectral residual is non-finite".into(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub(in crate::analysis::harmonic_balance::solver) fn refresh_driven_integrals(
        &mut self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<bool>, HbError> {
        let Some(max_values) = self.periodic_integral_budget else {
            return Ok(Vec::new());
        };
        if self
            .prescribed_integrals
            .iter()
            .all(|row| row.as_ref().is_some_and(|row| !row.is_circuit_driven()))
        {
            return Ok(Vec::new());
        }
        // Registration may precede nonlinear device stamping. Only inspect
        // linear row closure once the complete device registry is available.
        let sources = std::mem::take(&mut self.behavioral_sources);
        let result =
            self.prepare_integrals_with_inputs(&sources, max_values, false, true, None, abort);
        self.behavioral_sources = sources;
        let (spectra, needed) = result?;
        self.prescribed_integrals = spectra;
        Ok(needed)
    }

    pub(in crate::analysis::harmonic_balance::solver) fn prepare_prescribed_integrals(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        max_values: usize,
        retained: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Option<PrescribedIntegral>>, HbError> {
        self.prepare_integrals_with_inputs(sources, max_values, retained, false, None, abort)
            .map(|(spectra, _)| spectra)
    }

    pub(super) fn prepare_integrals_with_inputs(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        max_values: usize,
        retained: bool,
        linear_coordinates: bool,
        driven: Option<&[Option<Vec<Complex64>>]>,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Option<PrescribedIntegral>>, Vec<bool>), HbError> {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let mut plans = sources
            .prescribed_integral_rates()
            .map_err(HbError::InvalidCircuit)?;
        // Borrow rate plans from an independent snapshot while the solver
        // discovers physical producer components and performs FFTs.
        let capacitor_expressions = self
            .periodic_capacitors
            .iter()
            .filter(|cap| cap.expression.program.sdt_count != 0)
            .map(|cap| cap.expression.clone())
            .collect::<Vec<_>>();
        for expression in &capacitor_expressions {
            expression
                .append_prescribed_integral_rates(&mut plans)
                .map_err(HbError::InvalidCircuit)?;
        }
        let mut spectra: Vec<Option<PrescribedIntegral>> = Vec::new();
        spectra.try_reserve_exact(plans.len()).map_err(|e| {
            HbError::InvalidCircuit(format!("prescribed integral basis allocation failed: {e}"))
        })?;
        let samples = self.fft.size();
        let frequency = self.config.fundamental_freq;
        let n = self.num_nodes + self.exact_mna_branches().len();
        let needed_size = if linear_coordinates && !retained && !plans.is_empty() {
            n
        } else {
            0
        };
        if needed_size > max_values {
            return Err(HbError::InvalidCircuit(
                "driven integral input metadata exceeds the periodic allocation limit".into(),
            ));
        }
        let mut needed = vec![false; needed_size];
        let mut retained_values = needed_size;
        let mut forced: Vec<Option<Vec<Value>>> = Vec::new();
        let mut groups = Vec::new();
        if !retained && plans.iter().any(|plan| plan.dependencies().is_none()) {
            let topology = self
                .num_nodes
                .saturating_add(self.exact_mna_branches().len())
                .saturating_add(1)
                .saturating_mul(12);
            let ensure = |values| {
                if values > max_values {
                    Err(HbError::InvalidCircuit(format!(
                        "driven integral exceeds the {max_values}-value periodic allocation limit"
                    )))
                } else {
                    Ok(())
                }
            };
            ensure(topology.saturating_add(needed_size))?;
            let mut forest = self.forced_voltage_forest(true, abort)?;
            forest.retain_coordinates(plans.iter().flat_map(|plan| plan.coordinates()), abort)?;
            groups = forest.groups;
            retained_values = topology
                .saturating_add(needed_size)
                .saturating_add(forest.nodes.len().saturating_mul(samples));
            ensure(
                retained_values
                    .saturating_add(2 * (self.num_harmonics + 1))
                    .saturating_add(samples),
            )?;
            forced.resize(self.num_nodes, None);
            for node in forest.nodes {
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
                    let parent = node.parent.map_or(0.0, |p| {
                        forced[p].as_ref().map_or(0.0, |values| values[sample])
                    });
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
                plan.dependencies_with_offsets(|i| coordinate_group(&groups, &forced, i))
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
                        coordinate_group(&groups, &forced, row).is_some()
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
            groups.extend((groups.len()..coordinates.len()).map(|i| i + 1));
            for (row, spectrum) in coordinates.iter().enumerate() {
                let Some(spectrum) = spectrum else { continue };
                if coordinate_group(&groups, &forced, row).is_none() {
                    continue;
                }
                if forced[row].is_none() {
                    retained_values = retained_values.saturating_add(samples);
                }
                if retained_values.saturating_add(temporary) > max_values {
                    return Err(HbError::InvalidCircuit(format!(
                        "driven integral exceeds the {max_values}-value periodic allocation limit"
                    )));
                }
                let mut values = forced[row]
                    .take()
                    .unwrap_or_else(|| Vec::with_capacity(samples));
                values.clear();
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
                groups[row] = 0;
            }
        }
        if let Some(driven) = driven {
            if retained || driven.len() != n {
                return Err(HbError::InvalidCircuit(
                    "driven integral spectra do not match the physical circuit".into(),
                ));
            }
            forced.resize(n, None);
            groups.extend((groups.len()..n).map(|i| i + 1));
            for (row, spectrum) in driven.iter().enumerate() {
                let Some(spectrum) = spectrum else { continue };
                if coordinate_group(&groups, &forced, row).is_none() {
                    continue;
                }
                if spectrum.len() != self.num_harmonics + 1
                    || spectrum[0].im != 0.0
                    || spectrum
                        .iter()
                        .any(|v| !v.re.is_finite() || !v.im.is_finite())
                {
                    return Err(HbError::InvalidCircuit(
                        "invalid nonlinear input spectrum".into(),
                    ));
                }
                if forced[row].is_none() {
                    retained_values = retained_values.saturating_add(samples);
                }
                if retained_values > max_values {
                    return Err(HbError::InvalidCircuit(
                        "nonlinear integral input waveforms exceed the periodic allocation limit"
                            .into(),
                    ));
                }
                let mut values = forced[row]
                    .take()
                    .unwrap_or_else(|| Vec::with_capacity(samples));
                values.clear();
                for sample in 0..samples {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    let value = primitive_value(spectrum, sample as Value / samples as Value);
                    if !value.is_finite() {
                        return Err(HbError::InvalidCircuit(
                            "nonlinear integral input projection overflowed".into(),
                        ));
                    }
                    values.push(value);
                }
                forced[row] = Some(values);
                groups[row] = 0;
            }
        }
        for plan in plans {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let dependencies = if retained {
                plan.dependencies()
            } else {
                plan.dependencies_with_offsets(|i| coordinate_group(&groups, &forced, i))
            };
            let Some(dependencies) = dependencies else {
                for row in plan.coordinates() {
                    if coordinate_group(&groups, &forced, row).is_some()
                        && let Some(needed) = needed.get_mut(row)
                    {
                        *needed = true;
                    }
                }
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
                        |index| {
                            forced
                                .get(index)
                                .and_then(Option::as_ref)
                                .map_or(0.0, |values| values[sample])
                        },
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
        Ok((spectra, needed))
    }
}
