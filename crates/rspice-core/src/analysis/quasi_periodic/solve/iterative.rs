//! Matrix-free Newton corrections with a bounded-memory inverse certificate.
use super::evaluation::Evaluation;
use super::preconditioner::FrequencyBlocks;
use super::*;
use crate::analysis::quasi_periodic::check_abort;
use crate::numerics::krylov::{GmresError, GmresOutcome, try_gmres_with_abort};

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
    /// Fixed equation scaling keeps inverse-column qualification independent
    /// of the units of a behavioral integral. Physical Newton acceptance is
    /// still measured by evaluate(), with the user's original tolerances.
    pub(super) fn equation_divisors(
        &self,
        evaluation: &Evaluation,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let mut divisors = vec![1.0_f64; self.unknowns];
        let omega = self
            .grid
            .frequencies_hz()
            .iter()
            .map(|f| f.abs())
            .fold(0.0, Value::max)
            * std::f64::consts::TAU;
        for sample in &evaluation.jacobian {
            check_abort(abort)?;
            for (terms, weight) in [(&sample.conductance, 1.0), (&sample.capacitance, omega)] {
                for &(row, _, value) in terms {
                    let magnitude = value.abs() * weight;
                    if !magnitude.is_finite() {
                        return Err(Error::Numerical("QPSS equation scale overflowed".into()));
                    }
                    divisors[row] = divisors[row].max(magnitude);
                }
            }
        }
        for entries in &self.linear {
            check_abort(abort)?;
            for &(row, _, value) in entries {
                let magnitude = value.norm();
                if !magnitude.is_finite() {
                    return Err(Error::Numerical("QPSS equation scale overflowed".into()));
                }
                divisors[row] = divisors[row].max(magnitude);
            }
        }
        Ok(divisors)
    }

    fn equilibrated_action(
        &mut self,
        evaluation: &Evaluation,
        direction: &[Value],
        divisors: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let mut result = self.jacobian_action(evaluation, direction, abort)?;
        for (row, values) in result.chunks_exact_mut(self.grid.len()).enumerate() {
            for value in values {
                *value /= divisors[row];
            }
        }
        Ok(result)
    }

    fn iterative_candidate(
        &mut self,
        evaluation: &Evaluation,
        blocks: &mut FrequencyBlocks,
        divisors: &[Value],
        rhs: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<GmresOutcome, Error> {
        let settings = self.config.linear.clone();
        let rhs = complex(rhs.to_vec());
        try_gmres_with_abort(
            &mut |value| {
                self.equilibrated_action(evaluation, &real(value)?, divisors, abort)
                    .map(complex)
            },
            &mut |value| blocks.apply(&real(value)?, divisors, abort).map(complex),
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
        })
    }

    /// Certify ||D J B D^-1 - I||_infinity < 1/2. Columns are generated and
    /// discarded one at a time, so neither J nor B is retained. This excludes
    /// a singular operator even when the physical residual is exactly zero.
    /// The first attempt uses the cheap block inverse; the second constructs
    /// each inverse column with the authored Krylov settings.
    fn certify_iterative_inverse(
        &mut self,
        evaluation: &Evaluation,
        blocks: &mut FrequencyBlocks,
        divisors: &[Value],
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
                    let candidate =
                        self.iterative_candidate(evaluation, blocks, divisors, &rhs, abort)?;
                    // These columns prove invertibility; they are not Newton
                    // corrections. Even a finite unconverged candidate can
                    // prove ||D J B D^-1-I|| < 1/2. Certify that bound below
                    // instead of demanding needless inverse-column precision.
                    real(&candidate.solution)?
                } else {
                    blocks.apply(&rhs, divisors, abort)?
                };
                let applied =
                    self.equilibrated_action(evaluation, &inverse_column, divisors, abort)?;
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
        let divisors = self.equation_divisors(evaluation, abort)?;
        if evaluation.merit <= 1.0 {
            self.certify_iterative_inverse(evaluation, &mut blocks, &divisors, abort)?;
            return Ok(vec![0.0; self.unknowns * self.grid.len()]);
        }
        let mut rhs = coordinates::encode(&evaluation.residual);
        for (row, values) in rhs.chunks_exact_mut(self.grid.len()).enumerate() {
            for value in values {
                *value /= divisors[row];
            }
        }
        let outcome = self.iterative_candidate(evaluation, &mut blocks, &divisors, &rhs, abort)?;
        if !outcome.converged {
            return Err(Error::Numerical(format!(
                "QPSS Krylov correction did not converge after {} steps (relative residual {:e}); increase restart vectors or cycles, or use the direct solver for small systems",
                outcome.iterations, outcome.relative_residual
            )));
        }
        real(&outcome.solution)
    }
}
