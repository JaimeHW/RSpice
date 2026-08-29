//! Dense linear-system helpers shared by HB linear and Newton solves.

use super::*;
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};

impl HbSolver {
    /// Solve and certify a complex linear system.
    ///
    /// HB, PAC, and PNoise assemble dense operators today, but they must use
    /// the same scale-aware pivoting and componentwise backward-error checks
    /// as the rest of the simulator. In particular, an absolute pivot cutoff
    /// would mistake a valid high-impedance circuit for a singular one, while
    /// skipping a genuinely singular pivot can manufacture a zero-filled
    /// "solution".
    pub(super) fn solve_complex_linear_system(
        &self,
        a: &[Vec<Complex64>],
        b: &[Complex64],
    ) -> Result<Vec<Complex64>, HbError> {
        let n = b.len();
        if n == 0 {
            if a.is_empty() {
                return Ok(Vec::new());
            }
            return Err(HbError::InvalidCircuit(format!(
                "HB linear system has {} matrix rows but an empty RHS",
                a.len()
            )));
        }
        if a.len() != n {
            return Err(HbError::InvalidCircuit(format!(
                "HB linear-system dimension mismatch: matrix has {} rows, RHS has {n}",
                a.len()
            )));
        }
        if b.iter()
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(HbError::InvalidCircuit(
                "HB linear-system RHS contains a non-finite value".to_string(),
            ));
        }

        // Retain the diagonal structurally even when it is zero so an empty
        // row reaches the factorization and is diagnosed as singular. Exact
        // off-diagonal zeros do not need sparse storage.
        let mut structure = Vec::with_capacity(n);
        for (row_index, row) in a.iter().enumerate() {
            if row.len() != n {
                return Err(HbError::InvalidCircuit(format!(
                    "HB linear-system row {row_index} has {} columns; expected {n}",
                    row.len()
                )));
            }
            for (col_index, &value) in row.iter().enumerate() {
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB linear-system coefficient ({row_index}, {col_index}) is non-finite"
                    )));
                }
                if row_index == col_index || value != Complex64::new(0.0, 0.0) {
                    structure.push((row_index, col_index, 0.0));
                }
            }
        }

        let real_structure =
            StaticMatrix::from_triplets(n, n, &structure).map_err(Self::map_linear_solve_error)?;
        let mut matrix = ComplexMatrix::from_real_structure(&real_structure);
        for (row_index, row) in a.iter().enumerate() {
            for (col_index, &value) in row.iter().enumerate() {
                if value != Complex64::new(0.0, 0.0) {
                    matrix
                        .try_add(row_index, col_index, value)
                        .map_err(Self::map_linear_solve_error)?;
                }
            }
        }

        match matrix.solve(b) {
            Ok(solution) => Ok(solution),
            Err(SolverError::InaccurateSolution(_)) if n <= 64 => matrix
                .solve_dense_extended(b)
                .map_err(Self::map_linear_solve_error),
            Err(error) => Err(Self::map_linear_solve_error(error)),
        }
    }

    pub(super) fn map_linear_solve_error(error: SolverError) -> HbError {
        match error {
            SolverError::SingularMatrix | SolverError::PivotGrowth => HbError::SingularMatrix,
            SolverError::InvalidCircuit(message) => {
                HbError::InvalidCircuit(format!("HB linear solve: {message}"))
            }
            error => HbError::InvalidCircuit(format!("HB linear solve failed: {error}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::harmonic_balance::HbConfig;

    fn solver() -> HbSolver {
        HbSolver::new(HbConfig::new(1.0e9), 1)
    }

    #[test]
    fn singular_system_fails_closed() {
        let a = vec![
            vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0)],
            vec![Complex64::new(2.0, 0.0), Complex64::new(4.0, 0.0)],
        ];
        let b = vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0)];

        assert!(matches!(
            solver().solve_complex_linear_system(&a, &b),
            Err(HbError::SingularMatrix)
        ));
    }

    #[test]
    fn zero_row_with_nonzero_rhs_fails_closed() {
        let a = vec![
            vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
        ];
        let b = vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)];

        assert!(matches!(
            solver().solve_complex_linear_system(&a, &b),
            Err(HbError::SingularMatrix)
        ));
    }

    #[test]
    fn tiny_physical_coefficient_is_not_an_absolute_singularity() {
        let a = vec![vec![Complex64::new(1.0e-18, 0.0)]];
        let b = vec![Complex64::new(1.0, 0.0)];

        let solution = solver()
            .solve_complex_linear_system(&a, &b)
            .expect("1e18-ohm scalar system is nonsingular");
        assert!((solution[0].re - 1.0e18).abs() <= 1.0e3);
        assert_eq!(solution[0].im, 0.0);
    }

    #[test]
    fn badly_scaled_nonsingular_system_is_certified() {
        let a = vec![
            vec![Complex64::new(1.0e-18, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(1.0, 1.0), Complex64::new(1.0e18, 0.0)],
        ];
        let expected = [Complex64::new(2.0, -3.0), Complex64::new(-4.0, 5.0)];
        let b = vec![
            a[0][0] * expected[0] + a[0][1] * expected[1],
            a[1][0] * expected[0] + a[1][1] * expected[1],
        ];

        let solution = solver()
            .solve_complex_linear_system(&a, &b)
            .expect("scaled nonsingular system should solve");
        for (actual, expected) in solution.iter().zip(expected) {
            assert!((*actual - expected).norm() <= 1.0e-12 * expected.norm().max(1.0));
        }
    }

    #[test]
    fn malformed_or_nonfinite_system_is_rejected() {
        let rhs = vec![Complex64::new(1.0, 0.0)];
        assert!(matches!(
            solver().solve_complex_linear_system(&[], &rhs),
            Err(HbError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solver().solve_complex_linear_system(&[vec![Complex64::new(f64::NAN, 0.0)]], &rhs,),
            Err(HbError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solver().solve_complex_linear_system(
                &[vec![Complex64::new(1.0, 0.0)]],
                &[Complex64::new(f64::INFINITY, 0.0)],
            ),
            Err(HbError::InvalidCircuit(_))
        ));
    }
}
