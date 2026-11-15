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

    /// Assemble, sparse, the matrix whose entries `visit` streams, or its
    /// transpose: the matrix [`Self::solve_complex_linear_system`] solves
    /// from dense rows, without the dense rows, so a system of any size can
    /// be solved directly. Duplicate entries are summed in the order they
    /// stream, as dense rows sum them, and the diagonal is kept structurally
    /// as there.
    pub(super) fn assemble_complex_entry_system(
        size: usize,
        transpose: bool,
        visit: impl FnOnce(&mut dyn FnMut(usize, usize, Complex64)),
    ) -> Result<ComplexMatrix, HbError> {
        let mut entries: Vec<(usize, usize, Complex64)> = Vec::new();
        let mut failure = None;
        visit(&mut |row, column, value| {
            if failure.is_some() {
                return;
            }
            let (row, column) = if transpose {
                (column, row)
            } else {
                (row, column)
            };
            if row >= size || column >= size {
                failure = Some(HbError::InvalidCircuit(format!(
                    "HB linear-system entry ({row}, {column}) is outside its {size}x{size} matrix"
                )));
            } else if entries.try_reserve(1).is_err() {
                failure = Some(HbError::LinearSolveFailed(
                    "HB linear-system entry allocation failed".to_string(),
                ));
            } else {
                entries.push((row, column, value));
            }
        });
        if let Some(failure) = failure {
            return Err(failure);
        }
        // A stable sort keeps each coordinate's entries in the order they
        // streamed, so each sum rounds as the dense row's does.
        entries.sort_by_key(|&(row, column, _)| (row, column));
        let mut cells: Vec<(usize, usize, Complex64)> = Vec::new();
        cells.try_reserve_exact(entries.len()).map_err(|error| {
            HbError::LinearSolveFailed(format!("HB linear-system allocation failed: {error}"))
        })?;
        for (row, column, value) in entries {
            match cells.last_mut() {
                Some(cell) if cell.0 == row && cell.1 == column => cell.2 += value,
                _ => cells.push((row, column, Complex64::new(0.0, 0.0) + value)),
            }
        }
        if let Some(&(row, column, _)) = cells
            .iter()
            .find(|(_, _, value)| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(HbError::InvalidCircuit(format!(
                "HB linear-system coefficient ({row}, {column}) is non-finite"
            )));
        }
        let mut structure = Vec::new();
        structure
            .try_reserve_exact(cells.len() + size)
            .map_err(|error| {
                HbError::LinearSolveFailed(format!("HB linear-system allocation failed: {error}"))
            })?;
        let mut next = cells.iter().peekable();
        for row in 0..size {
            let mut diagonal = false;
            while let Some(&&(cell_row, column, value)) = next.peek() {
                if cell_row != row {
                    break;
                }
                next.next();
                if column > row && !diagonal {
                    structure.push((row, row, 0.0));
                    diagonal = true;
                }
                if column == row || value != Complex64::new(0.0, 0.0) {
                    diagonal |= column == row;
                    structure.push((row, column, 0.0));
                }
            }
            if !diagonal {
                structure.push((row, row, 0.0));
            }
        }
        let real_structure = StaticMatrix::from_triplets(size, size, &structure)
            .map_err(Self::map_linear_solve_error)?;
        let mut matrix = ComplexMatrix::from_real_structure(&real_structure);
        for (row, column, value) in cells {
            if value != Complex64::new(0.0, 0.0) {
                matrix
                    .try_add(row, column, value)
                    .map_err(Self::map_linear_solve_error)?;
            }
        }
        Ok(matrix)
    }

    /// Solve and certify a system [`Self::assemble_complex_entry_system`]
    /// assembled, as [`Self::solve_complex_linear_system`] solves its own;
    /// the matrix keeps its factorization for the next right-hand side.
    pub(super) fn solve_complex_entry_system(
        matrix: &mut ComplexMatrix,
        b: &[Complex64],
    ) -> Result<Vec<Complex64>, HbError> {
        if b.iter()
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(HbError::InvalidCircuit(
                "HB linear-system RHS contains a non-finite value".to_string(),
            ));
        }
        match matrix.solve(b) {
            Ok(solution) => Ok(solution),
            Err(SolverError::InaccurateSolution(_)) if b.len() <= 64 => matrix
                .solve_dense_extended(b)
                .map_err(Self::map_linear_solve_error),
            Err(error) => Err(Self::map_linear_solve_error(error)),
        }
    }

    pub(super) fn map_linear_solve_error(error: SolverError) -> HbError {
        Self::map_linear_solve_error_with_context(error, "HB linear solve")
    }

    pub(super) fn map_linear_solve_error_with_context(
        error: SolverError,
        context: &str,
    ) -> HbError {
        match error {
            SolverError::SingularMatrix | SolverError::PivotGrowth => HbError::SingularMatrix,
            SolverError::ConvergenceFailed(_) | SolverError::InaccurateSolution(_) => {
                HbError::LinearConvergenceFailed(format!("{context}: {error}"))
            }
            SolverError::InvalidCircuit(message) => {
                HbError::InvalidCircuit(format!("{context}: {message}"))
            }
            SolverError::OutOfMemory | SolverError::Overflow => {
                HbError::LinearSolveFailed(format!("{context}: {error}"))
            }
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
    fn linear_failure_policy_preserves_diagnostics_and_excludes_fatal_errors() {
        for error in [
            SolverError::ConvergenceFailed(7),
            SolverError::InaccurateSolution(1e-8),
        ] {
            let cause = error.to_string();
            let mapped = HbSolver::map_linear_solve_error_with_context(error, "exact 258x258 step");
            assert!(mapped.is_convergence_failure(), "{mapped}");
            assert!(mapped.to_string().contains(&cause));
            assert!(mapped.to_string().contains("exact 258x258 step"));
        }
        for error in [SolverError::OutOfMemory, SolverError::Overflow] {
            let cause = error.to_string();
            let mapped = HbSolver::map_linear_solve_error(error);
            assert!(!mapped.is_convergence_failure(), "{mapped}");
            assert!(mapped.to_string().contains(&cause));
        }
        let malformed = HbSolver::map_linear_solve_error(SolverError::InvalidCircuit(
            "malformed operator".to_owned(),
        ));
        assert!(matches!(malformed, HbError::InvalidCircuit(_)));
        assert!(!malformed.is_convergence_failure());
        assert!(malformed.to_string().contains("malformed operator"));
        assert!(!HbError::Aborted.is_convergence_failure());
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
