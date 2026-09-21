//! Strict direct and bounded-memory complex Krylov solves.
use super::*;
use crate::numerics::krylov::{GmresError, try_gmres_with_abort};
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};
use std::collections::BTreeMap;

fn matrix(n: usize, entries: &[LinearEntry]) -> Result<ComplexMatrix, Error> {
    let mut pattern: Vec<_> = (0..n).map(|i| (i, i, 0.0)).collect();
    pattern.extend(entries.iter().map(|&(r, c, _)| (r, c, 0.0)));
    let structure = StaticMatrix::from_triplets(n, n, &pattern)?;
    let mut matrix = ComplexMatrix::from_real_structure(&structure);
    for &(r, c, v) in entries {
        matrix.add(r, c, v);
    }
    Ok(matrix)
}

struct Blocks {
    matrices: Vec<Option<ComplexMatrix>>,
    unknowns: usize,
    exact: bool,
}

impl Blocks {
    fn build(
        work: &Linearization,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        let first = &work.derivatives[0];
        let exact = work
            .derivatives
            .iter()
            .all(|s| s.conductance == first.conductance && s.capacitance == first.capacitance);
        let samples = if exact {
            &work.derivatives[..1]
        } else {
            &work.derivatives[..]
        };
        let mut means: BTreeMap<(usize, usize), (Value, Value)> = BTreeMap::new();
        for (charge, terms) in [
            (false, &work.stationary.conductance),
            (true, &work.stationary.capacitance),
        ] {
            for &(r, c, v) in terms {
                let pair = means.entry((r, c)).or_default();
                if charge {
                    pair.1 += v;
                } else {
                    pair.0 += v;
                }
            }
        }
        for sample in samples {
            check_abort(abort)?;
            for (charge, terms) in [(false, &sample.conductance), (true, &sample.capacitance)] {
                for &(r, c, v) in terms {
                    let pair = means.entry((r, c)).or_default();
                    if charge {
                        pair.1 += v / samples.len() as Value;
                    } else {
                        pair.0 += v / samples.len() as Value;
                    }
                }
            }
        }
        let mut matrices = Vec::with_capacity(frequencies.len());
        // Offset translation breaks conjugacy: factor EVERY signed block.
        for (k, &frequency) in frequencies.iter().enumerate() {
            check_abort(abort)?;
            let jw = Complex64::new(0.0, std::f64::consts::TAU * frequency);
            let mut entries = linear[k].clone();
            for (&(r, c), &(g, q)) in &means {
                let value = Complex64::new(g, 0.0) + jw * q;
                if !finite(value) {
                    return Err(Error::Numerical("QPAC mean derivative overflowed".into()));
                }
                entries.push((r, c, value));
            }
            if work.orientation == Orientation::Adjoint {
                for (row, col, value) in &mut entries {
                    std::mem::swap(row, col);
                    *value = value.conj();
                }
            }
            let mut block = matrix(work.unknowns, &entries)?;
            match block.solve(&vec![Complex64::ZERO; work.unknowns]) {
                Ok(_) => matrices.push(Some(block)),
                Err(SolverError::SingularMatrix | SolverError::InaccurateSolution(_)) if !exact => {
                    matrices.push(None)
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(Self {
            matrices,
            unknowns: work.unknowns,
            exact,
        })
    }

    fn apply(
        &mut self,
        direction: &[Complex64],
        divisors: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let entries = self.matrices.len();
        let mut output = vec![Complex64::ZERO; direction.len()];
        let mut rhs = vec![Complex64::ZERO; self.unknowns];
        for (k, matrix) in self.matrices.iter_mut().enumerate() {
            check_abort(abort)?;
            for row in 0..self.unknowns {
                rhs[row] = direction[row * entries + k] * divisors[row];
            }
            let solved = match matrix {
                Some(matrix) => {
                    let mut candidate = vec![Complex64::ZERO; self.unknowns];
                    match matrix.solve_into(&rhs, &mut candidate) {
                        Ok(()) => candidate,
                        Err(SolverError::InaccurateSolution(_))
                            if candidate.iter().all(|v| finite(*v)) =>
                        {
                            // This block is only a preconditioner. Its finite
                            // approximate inverse is qualified by the complete
                            // operator below, never published as a response.
                            if self.unknowns <= 64 {
                                let mut extended = Vec::new();
                                match matrix.solve_dense_extended_into(&rhs, &mut extended) {
                                    Ok(()) | Err(SolverError::InaccurateSolution(_)) => extended,
                                    Err(error) => return Err(error.into()),
                                }
                            } else {
                                candidate
                            }
                        }
                        Err(error) => return Err(error.into()),
                    }
                }
                None => rhs.clone(),
            };
            for (row, value) in solved.into_iter().enumerate() {
                output[row * entries + k] = value;
            }
        }
        Ok(output)
    }
}

impl Linearization {
    pub(super) fn direct(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        rhs: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let size = rhs.len();
        let mut entries = Vec::new();
        let mut unit = vec![Complex64::ZERO; size];
        for column in 0..size {
            unit[column] = Complex64::ONE;
            let values = self.apply(frequencies, linear, &unit, abort)?;
            unit[column] = Complex64::ZERO;
            for (row, value) in values.into_iter().enumerate() {
                if value != Complex64::ZERO {
                    entries.push((row, column, value));
                }
            }
        }
        check_abort(abort)?;
        let mut matrix = matrix(size, &entries)?;
        drop(entries);
        let solution = match matrix.solve(rhs) {
            Ok(solution) => solution,
            // Homogeneous constraints can lose accuracy during sparse
            // elimination. The caller already bounds direct systems to 512
            // coordinates; account for the extended real-block workspace and
            // retain the same strict certificate on the rounded solution.
            Err(SolverError::InaccurateSolution(_)) => {
                ResourceLimitError::ensure(
                    ResourceKind::ResultValues,
                    self.base_values
                        .saturating_add(size.saturating_mul(size).saturating_mul(24)),
                    self.value_limit,
                )?;
                matrix.solve_dense_extended(rhs)?
            }
            Err(error) => return Err(error.into()),
        };
        check_abort(abort)?;
        Ok(solution)
    }

    fn krylov(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        divisors: &[Value],
        rhs: &[Complex64],
        blocks: &mut Blocks,
        config: &QuasiPeriodicAcConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let outcome = try_gmres_with_abort(
            &mut |v| self.apply_equilibrated(frequencies, linear, divisors, v, abort),
            &mut |v| blocks.apply(v, divisors, abort),
            rhs,
            config.linear.restart,
            config.linear.max_cycles,
            config.linear.relative_tolerance,
            &|| abort.is_aborted(),
        )
        .map_err(|error| match error {
            GmresError::Aborted => Error::Aborted,
            GmresError::Operator(error) => error,
            GmresError::InvalidData(reason) => Error::Numerical(reason.into()),
        })?;
        if !outcome.converged {
            return Err(Error::Numerical(format!(
                "QPAC Krylov solve did not converge after {} steps (relative residual {:e})",
                outcome.iterations, outcome.relative_residual
            )));
        }
        Ok(outcome.solution)
    }

    pub(super) fn iterative(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        divisors: &[Value],
        rhs: &[Complex64],
        config: &QuasiPeriodicAcConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let mut blocks = Blocks::build(self, frequencies, linear, abort)?;
        // A compatible RHS, including zero, does not prove the operator is
        // invertible. Require ||D A B D^-1 - I||_infinity < 1/2 using inverse columns
        // retained one at a time. No regularized factor is a certificate.
        if !blocks.exact {
            let mut certified = false;
            for use_krylov in [false, true] {
                let mut row_bounds = vec![0.0; rhs.len()];
                let mut unit = vec![Complex64::ZERO; rhs.len()];
                let mut qualified = true;
                for column in 0..rhs.len() {
                    check_abort(abort)?;
                    unit[column] = Complex64::ONE;
                    let inverse = if use_krylov {
                        self.krylov(
                            frequencies,
                            linear,
                            divisors,
                            &unit,
                            &mut blocks,
                            config,
                            abort,
                        )?
                    } else {
                        blocks.apply(&unit, divisors, abort)?
                    };
                    let applied =
                        self.apply_equilibrated(frequencies, linear, divisors, &inverse, abort)?;
                    unit[column] = Complex64::ZERO;
                    for (row, (bound, value)) in row_bounds.iter_mut().zip(applied).enumerate() {
                        let target = if row == column {
                            Complex64::ONE
                        } else {
                            Complex64::ZERO
                        };
                        *bound += (value - target).norm()
                            + 1024.0 * Value::EPSILON * (1.0 + value.norm());
                        if !bound.is_finite() || *bound >= 0.5 {
                            qualified = false;
                        }
                    }
                    if !qualified {
                        break;
                    }
                }
                if qualified {
                    certified = true;
                    break;
                }
            }
            if !certified {
                return Err(Error::Numerical(
                    "QPAC could not certify a nonsingular translated operator".into(),
                ));
            }
        }
        let rhs: Vec<_> = rhs
            .iter()
            .enumerate()
            .map(|(i, value)| *value / divisors[i / self.grid.len()])
            .collect();
        self.krylov(
            frequencies,
            linear,
            divisors,
            &rhs,
            &mut blocks,
            config,
            abort,
        )
    }
}
