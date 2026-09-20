//! Bounded Newton updates, backtracking and componentwise equation certificates.
use super::*;
use crate::analysis::quasi_periodic::check_abort;
use crate::solver::{SolverError, StaticMatrix};

impl Workspace<'_> {
    fn correction(
        &mut self,
        evaluation: &evaluation::Evaluation,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let size = self.unknowns * self.grid.len();
        let mut triplets = Vec::new();
        for column in 0..size {
            let values = self.jacobian_column(evaluation, column, abort)?;
            for (row, value) in values.into_iter().enumerate() {
                if row == column || value != 0.0 {
                    triplets.push((row, column, value));
                }
            }
        }
        check_abort(abort)?;
        let mut matrix = StaticMatrix::from_triplets(size, size, &triplets)?;
        drop(triplets);
        let rhs = coordinates::encode(&evaluation.residual);
        let correction = match matrix.solve(&rhs) {
            Err(SolverError::InaccurateSolution(_)) if size <= 64 => {
                matrix.solve_dense_extended(&rhs)
            }
            result => result,
        }?;
        check_abort(abort)?;
        Ok(correction)
    }

    pub(super) fn newton(
        &mut self,
        circuit: &mut impl Circuit,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicSolution, Error> {
        let mut spectra = seed
            .map(<[_]>::to_vec)
            .unwrap_or_else(|| vec![vec![Complex64::ZERO; self.grid.len()]; self.unknowns]);
        for iterations in 0..=self.config.max_iterations {
            check_abort(abort)?;
            let evaluation = self.evaluate(circuit, &spectra, sources, true, abort)?;
            // Certify a nonsingular Jacobian even for a zero-residual seed:
            // an unexcited floating circuit is not a qualified solution.
            let correction = self.correction(&evaluation, abort)?;
            if evaluation.merit <= 1.0 {
                check_abort(abort)?;
                return Ok(QuasiPeriodicSolution {
                    grid: self.grid.clone(),
                    spectra,
                    iterations,
                    normalized_residual: evaluation.merit,
                });
            }
            if iterations == self.config.max_iterations {
                return Err(Error::ConvergenceFailed {
                    iterations,
                    merit: evaluation.merit,
                });
            }
            let state = coordinates::encode(&spectra);
            let mut accepted = None;
            let mut damping = 1.0;
            for _ in 0..=self.config.max_backtracks {
                check_abort(abort)?;
                let trial: Vec<_> = state
                    .iter()
                    .zip(&correction)
                    .map(|(x, dx)| x + damping * dx)
                    .collect();
                if trial.iter().all(|value| value.is_finite()) {
                    let candidate = coordinates::decode(&trial, self.grid.len());
                    match self.evaluate(circuit, &candidate, sources, false, abort) {
                        Ok(value) if value.merit <= 1.0 || value.merit < evaluation.merit => {
                            accepted = Some(candidate);
                            break;
                        }
                        Ok(_) | Err(Error::Numerical(_)) => {}
                        Err(error) => return Err(error),
                    }
                }
                damping *= 0.5;
            }
            spectra = accepted.ok_or(Error::ConvergenceFailed {
                iterations,
                merit: evaluation.merit,
            })?;
        }
        unreachable!("every bounded Newton loop exits with a certificate or a failure")
    }
}
