//! Physical residuals and analytic F/Q derivatives on independent phases.
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};

pub(super) struct JacobianSample {
    pub conductance: Vec<JacobianEntry>,
    pub capacitance: Vec<JacobianEntry>,
}

pub(super) struct Evaluation {
    pub residual: Vec<Vec<Complex64>>,
    pub jacobian: Vec<JacobianSample>,
    pub merit: Value,
}

type TermWaves = Vec<(usize, Vec<Complex64>)>;

fn record_terms(
    waves: &mut TermWaves,
    terms: &[(usize, Value)],
    time: usize,
    count: usize,
    unknowns: usize,
) -> Result<(), Error> {
    if time == 0 {
        for &(row, _) in terms {
            waves.push((row, super::super::zero_buffer(count)?));
        }
    }
    if terms.len() != waves.len() {
        return Err(Error::InvalidCircuit(
            "physical F/Q term count changes over the phase grid".into(),
        ));
    }
    for ((row, wave), &(actual_row, value)) in waves.iter_mut().zip(terms) {
        if *row != actual_row || *row >= unknowns {
            return Err(Error::InvalidCircuit(
                "physical F/Q term changes its MNA row".into(),
            ));
        }
        if !value.is_finite() {
            return Err(Error::Numerical(
                "device returned a non-finite F/Q term".into(),
            ));
        }
        wave[time] = Complex64::new(value, 0.0);
    }
    Ok(())
}

fn add_term(residual: &mut Complex64, scale: &mut Value, term: Complex64) -> Result<(), Error> {
    *residual += term;
    *scale += term.norm();
    if !finite(*residual) || !scale.is_finite() {
        return Err(Error::Numerical(
            "physical equation or its contribution scale overflowed".into(),
        ));
    }
    Ok(())
}

impl Workspace<'_> {
    pub(super) fn evaluate(
        &mut self,
        circuit: &mut impl Circuit,
        spectra: &[Vec<Complex64>],
        sources: &[Vec<Complex64>],
        with_jacobian: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Evaluation, Error> {
        check_abort(abort)?;
        let entries = self.grid.len();
        let count = self.grid.sample_count();
        let mut residual = vec![vec![Complex64::ZERO; entries]; self.unknowns];
        let mut scales = vec![vec![0.0; entries]; self.unknowns];
        for row in 0..self.unknowns {
            for k in 0..entries {
                add_term(&mut residual[row][k], &mut scales[row][k], sources[row][k])?;
            }
        }
        for (k, matrix) in self.linear.iter().enumerate() {
            check_abort(abort)?;
            for &(row, col, value) in matrix {
                add_term(
                    &mut residual[row][k],
                    &mut scales[row][k],
                    -value * spectra[col][k],
                )?;
            }
        }
        let waves: Vec<_> = spectra
            .iter()
            .map(|s| self.transform.to_real_samples_with_abort(s, abort))
            .collect::<Result<_, _>>()?;
        let mut state = vec![0.0; self.unknowns];
        let mut current = Vec::new();
        let mut charge = Vec::new();
        let mut jacobian = Vec::new();
        let mut retained = 0usize;
        for time in 0..count {
            check_abort(abort)?;
            for (row, value) in state.iter_mut().enumerate() {
                *value = waves[row][time];
            }
            let phases = self.grid.phases(time).expect("bounded collocation index");
            let sample = circuit.sample_at_phases(&state, &phases, with_jacobian)?;
            if time == 0 {
                // The previous Newton evaluation can coexist with a trial
                // evaluation; charge both sets of physical-term waveforms.
                retained = sample
                    .current
                    .len()
                    .saturating_add(sample.charge.len())
                    .saturating_mul(count)
                    .saturating_mul(4);
            }
            retained = retained.saturating_add(
                sample
                    .conductance
                    .len()
                    .saturating_add(sample.capacitance.len())
                    .saturating_mul(6),
            );
            self.budget(retained)?;
            record_terms(&mut current, &sample.current, time, count, self.unknowns)?;
            record_terms(&mut charge, &sample.charge, time, count, self.unknowns)?;
            if with_jacobian {
                for &(row, col, value) in sample.conductance.iter().chain(&sample.capacitance) {
                    if row >= self.unknowns || col >= self.unknowns {
                        return Err(Error::InvalidCircuit(
                            "device Jacobian has an invalid MNA coordinate".into(),
                        ));
                    }
                    if !value.is_finite() {
                        return Err(Error::Numerical(
                            "device returned a non-finite F/Q derivative".into(),
                        ));
                    }
                }
                jacobian.push(JacobianSample {
                    conductance: sample.conductance,
                    capacitance: sample.capacitance,
                });
            }
        }
        for (terms, is_charge) in [(current, false), (charge, true)] {
            for (row, wave) in terms {
                check_abort(abort)?;
                let spectrum = self.transform.to_spectrum_with_abort(&wave, abort)?;
                for (k, mut term) in spectrum.into_iter().enumerate() {
                    if is_charge {
                        term *= Complex64::new(
                            0.0,
                            std::f64::consts::TAU * self.grid.frequencies_hz()[k],
                        );
                    }
                    add_term(&mut residual[row][k], &mut scales[row][k], term)?;
                }
            }
        }
        let mut merit = 0.0_f64;
        for row in 0..self.unknowns {
            let absolute = if self.voltage_rows[row] {
                self.config.voltage_absolute_tolerance
            } else {
                self.config.current_absolute_tolerance
            };
            for k in 0..entries {
                let denominator = absolute + self.config.relative_tolerance * scales[row][k];
                let normalized = residual[row][k].norm() / denominator;
                if !denominator.is_finite() || !normalized.is_finite() {
                    return Err(Error::Numerical(
                        "non-finite equation convergence certificate".into(),
                    ));
                }
                merit = merit.max(normalized);
            }
        }
        Ok(Evaluation {
            residual,
            jacobian,
            merit,
        })
    }
}
