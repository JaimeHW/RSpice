//! Matrix-free Newton corrections with a bounded-memory inverse certificate.
use super::evaluation::Evaluation;
use super::preconditioner::FrequencyBlocks;
use super::*;
use crate::analysis::quasi_periodic::check_abort;
use crate::numerics::krylov::{GmresError, try_gmres_with_abort};

fn real(values: &[Complex64]) -> Result<Vec<Value>, Error> {
    if values
        .iter()
        .any(|value| !value.re.is_finite() || value.im != 0.0)
    {
        return Err(Error::Numerical(
            "QPSS real-coordinate Krylov vector became complex or non-finite".into(),
        ));
    }
    Ok(values.iter().map(|value| value.re).collect())
}
fn complex(values: Vec<Value>) -> Vec<Complex64> {
    values
        .into_iter()
        .map(|value| Complex64::new(value, 0.0))
        .collect()
}

impl Workspace<'_> {
    fn iterative_solve(
        &mut self,
        evaluation: &Evaluation,
        blocks: &mut FrequencyBlocks,
        rhs: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let settings = self.config.linear.clone();
        let rhs = complex(rhs.to_vec());
        let outcome = try_gmres_with_abort(
            &mut |value| {
                self.jacobian_action(evaluation, &real(value)?, abort)
                    .map(complex)
            },
            &mut |value| blocks.apply(&real(value)?, abort).map(complex),
            &rhs,
            settings.restart,
            settings.max_cycles,
            settings.relative_tolerance,
            &|| abort.is_aborted(),
        )
        .map_err(|error| match error {
            GmresError::Aborted => Error::Aborted,
            GmresError::Operator(error) => error,
            GmresError::InvalidData(reason) => Error::Numerical(reason.into()),
        })?;
        if !outcome.converged {
            return Err(Error::Numerical(format!(
                "QPSS Krylov solve did not converge after {} steps (relative residual {:e}); increase restart vectors or cycles, or use the direct solver for small systems",
                outcome.iterations, outcome.relative_residual
            )));
        }
        real(&outcome.solution)
    }

    /// Certify ||J B - I||_infinity < 1/2. Columns of B are generated and
    /// discarded one at a time, so neither J nor B is retained. This excludes
    /// a singular operator even when the physical residual is exactly zero.
    /// The first attempt uses the cheap block inverse; the second constructs
    /// each inverse column with the authored Krylov settings.
    fn certify_iterative_inverse(
        &mut self,
        evaluation: &Evaluation,
        blocks: &mut FrequencyBlocks,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        if blocks.exact {
            return Ok(());
        }
        let size = self.unknowns * self.grid.len();
        for use_krylov in [false, true] {
            let mut row_bounds = vec![0.0; size];
            let mut rhs = vec![0.0; size];
            let mut qualified = true;
            for column in 0..size {
                check_abort(abort)?;
                rhs[column] = 1.0;
                let inverse_column = if use_krylov {
                    self.iterative_solve(evaluation, blocks, &rhs, abort)?
                } else {
                    blocks.apply(&rhs, abort)?
                };
                let applied = self.jacobian_action(evaluation, &inverse_column, abort)?;
                rhs[column] = 0.0;
                for (row, (bound, value)) in row_bounds.iter_mut().zip(applied).enumerate() {
                    let target = if row == column { 1.0 } else { 0.0 };
                    *bound +=
                        (value - target).abs() + 1024.0 * Value::EPSILON * (1.0 + value.abs());
                    if !bound.is_finite() || *bound >= 0.5 {
                        qualified = false;
                    }
                }
                if !qualified {
                    break;
                }
            }
            if qualified {
                return Ok(());
            }
        }
        Err(Error::Numerical("QPSS could not certify a nonsingular coupled Jacobian; tighten the linear tolerance or revise the circuit".into()))
    }

    pub(super) fn iterative_correction(
        &mut self,
        evaluation: &Evaluation,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let mut blocks = FrequencyBlocks::build(self, evaluation, abort)?;
        if evaluation.merit <= 1.0 {
            self.certify_iterative_inverse(evaluation, &mut blocks, abort)?;
            return Ok(vec![0.0; self.unknowns * self.grid.len()]);
        }
        self.iterative_solve(
            evaluation,
            &mut blocks,
            &coordinates::encode(&evaluation.residual),
            abort,
        )
    }
}
