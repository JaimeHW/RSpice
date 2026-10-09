//! Analytic real-coordinate Jacobian actions without assembling coupled columns.
use super::evaluation::Evaluation;
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};

impl Workspace<'_> {
    /// Apply d(F + dQ/dt)/dx to an arbitrary real Fourier direction. Device
    /// derivatives multiply phase-domain perturbations before projection, so
    /// couplings outside the retained difference lattice are not discarded.
    pub(super) fn jacobian_action(
        &mut self,
        evaluation: &Evaluation,
        direction: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        check_abort(abort)?;
        let entries = self.grid.len();
        let count = self.grid.sample_count();
        if direction.len() != self.unknowns.saturating_mul(entries)
            || direction.iter().any(|value| !value.is_finite())
        {
            return Err(Error::InvalidConfig(
                "Jacobian direction differs from the finite real MNA tone coordinates".into(),
            ));
        }
        if evaluation.jacobian.len() != count {
            return Err(Error::InvalidConfig(
                "Jacobian samples do not cover the phase grid".into(),
            ));
        }
        let spectra = coordinates::decode(direction, entries);
        // Constant derivatives are diagonal in the tone lattice. Sending them
        // through an inverse/forward transform introduces cross-tone roundoff
        // in otherwise exact constraints, which can stall tight Krylov solves.
        // Match each slot across every sample; changing sparse term order or
        // count simply keeps the affected term on the general sampled path.
        let constant = |charge: bool| -> Result<Vec<bool>, Error> {
            let first = if charge {
                &evaluation.jacobian[0].capacitance
            } else {
                &evaluation.jacobian[0].conductance
            };
            let mut fixed = vec![true; first.len()];
            for (time, sample) in evaluation.jacobian.iter().enumerate() {
                if time.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                let terms = if charge {
                    &sample.capacitance
                } else {
                    &sample.conductance
                };
                for (index, (flag, entry)) in fixed.iter_mut().zip(first).enumerate() {
                    *flag &= terms.get(index) == Some(entry);
                }
            }
            Ok(fixed)
        };
        let constant_g = constant(false)?;
        let constant_c = constant(true)?;
        let mut output = vec![vec![Complex64::ZERO; entries]; self.unknowns];
        for (terms, fixed, charge) in [
            (&evaluation.jacobian[0].conductance, &constant_g, false),
            (&evaluation.jacobian[0].capacitance, &constant_c, true),
        ] {
            for (&(row, col, derivative), &fixed) in terms.iter().zip(fixed) {
                check_abort(abort)?;
                if fixed {
                    for k in 0..entries {
                        let value = if charge {
                            Complex64::new(
                                0.0,
                                std::f64::consts::TAU * self.grid.frequencies_hz()[k] * derivative,
                            )
                        } else {
                            Complex64::new(derivative, 0.0)
                        };
                        output[row][k] += value * spectra[col][k];
                    }
                }
            }
        }
        // A direct-solve column activates only one MNA coordinate; preserve
        // that transform cost while also accepting dense Krylov directions.
        let mut perturbations = Vec::with_capacity(self.unknowns);
        for spectrum in &spectra {
            check_abort(abort)?;
            perturbations.push(if spectrum.iter().any(|value| *value != Complex64::ZERO) {
                Some(self.transform.to_real_samples_with_abort(spectrum, abort)?)
            } else {
                None
            });
        }
        let mut conductance = vec![vec![Complex64::ZERO; count]; self.unknowns];
        let mut capacitance = vec![vec![Complex64::ZERO; count]; self.unknowns];
        for (time, sample) in evaluation.jacobian.iter().enumerate() {
            if time.is_multiple_of(256) {
                check_abort(abort)?;
            }
            for (terms, values, fixed) in [
                (&sample.conductance, &mut conductance, &constant_g),
                (&sample.capacitance, &mut capacitance, &constant_c),
            ] {
                for (index, &(row, col, derivative)) in terms.iter().enumerate() {
                    if fixed.get(index) == Some(&true) {
                        continue;
                    }
                    if let Some(perturbation) = &perturbations[col] {
                        values[row][time].re += derivative * perturbation[time];
                    }
                }
            }
        }
        for row in 0..self.unknowns {
            let g = self
                .transform
                .to_spectrum_with_abort(&conductance[row], abort)?;
            let c = self
                .transform
                .to_spectrum_with_abort(&capacitance[row], abort)?;
            for k in 0..entries {
                output[row][k] += g[k]
                    + Complex64::new(0.0, std::f64::consts::TAU * self.grid.frequencies_hz()[k])
                        * c[k];
            }
        }
        for (k, matrix) in self.linear.iter().enumerate() {
            check_abort(abort)?;
            for &(row, col, value) in matrix {
                output[row][k] += value * spectra[col][k];
            }
        }
        if output.iter().flatten().any(|value| !finite(*value)) {
            return Err(Error::Numerical(
                "quasi-periodic Jacobian projection overflowed".into(),
            ));
        }
        Ok(coordinates::encode(&output))
    }

    /// The direct backend and iterative operators share the exact same
    /// derivative; selecting a linear solver cannot change the physical model.
    pub(super) fn jacobian_column(
        &mut self,
        evaluation: &Evaluation,
        column: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let size = self.unknowns.saturating_mul(self.grid.len());
        if column >= size {
            return Err(Error::InvalidConfig(
                "Jacobian column is out of range".into(),
            ));
        }
        let mut direction = vec![0.0; size];
        direction[column] = 1.0;
        self.jacobian_action(evaluation, &direction, abort)
    }
}
