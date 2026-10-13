//! Complete dense eigenspectra with a per-eigenpair residual qualification.
//!
//! faer's eigensolvers are generic, so they are compiled, and optimized or
//! not, in whichever crate names the scalar type. This module names it behind
//! one concrete entry point, so the Hessenberg reduction, the Schur iteration,
//! the eigenvector back substitution and the O(n³) residual qualification are
//! all compiled here. The workspace builds this package at `opt-level = 2` in
//! the development and test profiles, which an unoptimized simulator crate
//! calling faer directly would not get.
//!
//! Cancellation is cooperative: the eigensolve itself is one atomic faer call,
//! so `is_aborted` is polled immediately before and after it and periodically
//! throughout validation, extraction and qualification.

use faer::Mat;
use faer::linalg::solvers::Eigen;
use num_complex::Complex64;

const ABORT_POLL_STRIDE: usize = 64;

/// Schur form used to extract a complete dense eigenspectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenseSchurForm {
    /// Real quasi-triangular Schur form. Real eigenvalues come back real and
    /// complex ones as exact conjugate pairs.
    Real,
    /// Complex triangular Schur form of the matrix taken as complex.
    Complex,
}

/// A complete dense eigenspectrum whose every eigenpair met the residual bound.
#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedDenseEigenspectrum {
    /// Eigenvalues of the input matrix, in the eigensolver's order.
    pub eigenvalues: Vec<Complex64>,
    /// Right eigenvectors, one vector per eigenvalue, unnormalized.
    pub right_eigenvectors: Vec<Vec<Complex64>>,
    /// Largest normwise backward error among the returned eigenpairs.
    pub max_backward_error: f64,
}

/// Why a complete, residual-qualified dense eigenspectrum was not produced.
#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum DenseEigenspectrumError {
    /// Cooperative cancellation was requested.
    #[error("eigenspectrum computation was aborted")]
    Aborted,
    /// The matrix has no rows.
    #[error("eigenproblem matrix must be non-empty")]
    EmptyMatrix,
    /// A row's length differs from the number of rows.
    #[error(
        "eigenproblem matrix must be square: row {row} has {actual} columns, expected {expected}"
    )]
    NonSquare {
        /// Offending row.
        row: usize,
        /// Number of rows.
        expected: usize,
        /// Length of the offending row.
        actual: usize,
    },
    /// An input entry is NaN or infinite.
    #[error("eigenproblem matrix entry ({row}, {col}) is non-finite")]
    NonFiniteInput {
        /// Row of the entry.
        row: usize,
        /// Column of the entry.
        col: usize,
    },
    /// faer reported that the Schur iteration did not converge.
    #[error("faer ordinary eigenspectrum extraction failed")]
    SolverFailure,
    /// faer returned fewer eigenpairs than the matrix order.
    #[error(
        "ordinary eigenspectrum is incomplete: expected {expected} eigenpairs, received {eigenvalues} eigenvalues and a {vector_rows}x{vector_cols} eigenvector matrix"
    )]
    IncompleteSpectrum {
        /// Matrix order.
        expected: usize,
        /// Number of eigenvalues returned.
        eigenvalues: usize,
        /// Rows of the returned eigenvector matrix.
        vector_rows: usize,
        /// Columns of the returned eigenvector matrix.
        vector_cols: usize,
    },
    /// An eigenvalue is NaN or infinite.
    #[error("ordinary eigenvalue {index} is non-finite")]
    NonFiniteEigenvalue {
        /// Eigenpair index.
        index: usize,
    },
    /// An eigenvector is NaN, infinite or zero.
    #[error("ordinary right eigenvector {index} is non-finite or zero")]
    InvalidEigenvector {
        /// Eigenpair index.
        index: usize,
    },
    /// An eigenpair's normwise backward error exceeds the bound.
    #[error(
        "ordinary eigenpair {index} failed residual qualification (backward error {backward_error:.3e}, maximum {maximum:.3e})"
    )]
    NumericalQualification {
        /// First eigenpair, in eigensolver order, over the bound.
        index: usize,
        /// Its normwise backward error.
        backward_error: f64,
        /// The bound it exceeded.
        maximum: f64,
    },
}

#[inline]
fn ensure_not_aborted(is_aborted: &dyn Fn() -> bool) -> Result<(), DenseEigenspectrumError> {
    if is_aborted() {
        Err(DenseEigenspectrumError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_periodically(
    is_aborted: &dyn Fn() -> bool,
    operation_index: usize,
) -> Result<(), DenseEigenspectrumError> {
    if operation_index.is_multiple_of(ABORT_POLL_STRIDE) {
        ensure_not_aborted(is_aborted)?;
    }
    Ok(())
}

/// `‖A v − λ v‖₂ / (‖A‖_F ‖v‖₂)` for one eigenpair of the scaled matrix.
fn eigenpair_backward_error(
    matrix: &[Vec<f64>],
    matrix_norm: f64,
    lambda: Complex64,
    vector: &[Complex64],
    vector_norm: f64,
    is_aborted: &dyn Fn() -> bool,
    operation_index: &mut usize,
) -> Result<f64, DenseEigenspectrumError> {
    let mut residual_norm = 0.0_f64;
    for row in 0..matrix.len() {
        let mut product = Complex64::new(0.0, 0.0);
        for col in 0..matrix.len() {
            poll_periodically(is_aborted, *operation_index)?;
            *operation_index = operation_index.wrapping_add(1);
            product += matrix[row][col] * vector[col];
        }
        residual_norm = residual_norm.hypot((product - lambda * vector[row]).norm());
    }

    let denominator = matrix_norm * vector_norm;
    Ok(if denominator > 0.0 {
        residual_norm / denominator
    } else if residual_norm == 0.0 {
        0.0
    } else {
        f64::INFINITY
    })
}

/// Extract the complete eigenspectrum of a square real matrix and qualify
/// every eigenpair against `tolerance`.
///
/// The matrix is scaled by its largest entry magnitude before the eigensolve.
/// Each eigenpair's normwise backward error `‖A v − λ v‖₂ / (‖A‖_F ‖v‖₂)` is
/// measured on that scaled matrix in its original coordinates, and the first
/// eigenpair, in eigensolver order, over `tolerance` fails the whole spectrum.
/// The returned eigenvalues are scaled back to the input matrix.
///
/// `is_aborted` is polled throughout; a `true` returns
/// [`DenseEigenspectrumError::Aborted`].
pub fn qualified_dense_eigenspectrum(
    matrix: &[Vec<f64>],
    form: DenseSchurForm,
    tolerance: f64,
    is_aborted: &dyn Fn() -> bool,
) -> Result<QualifiedDenseEigenspectrum, DenseEigenspectrumError> {
    ensure_not_aborted(is_aborted)?;
    let n = matrix.len();
    if n == 0 {
        return Err(DenseEigenspectrumError::EmptyMatrix);
    }

    let mut max_abs = 0.0_f64;
    let mut operation_index = 0usize;
    for (row_index, row) in matrix.iter().enumerate() {
        poll_periodically(is_aborted, operation_index)?;
        if row.len() != n {
            return Err(DenseEigenspectrumError::NonSquare {
                row: row_index,
                expected: n,
                actual: row.len(),
            });
        }
        for (col_index, &value) in row.iter().enumerate() {
            poll_periodically(is_aborted, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            if !value.is_finite() {
                return Err(DenseEigenspectrumError::NonFiniteInput {
                    row: row_index,
                    col: col_index,
                });
            }
            max_abs = max_abs.max(value.abs());
        }
    }

    let scale = if max_abs > 0.0 { max_abs } else { 1.0 };
    let mut scaled = vec![vec![0.0; n]; n];
    let mut faer_matrix = Mat::zeros(n, n);
    let mut matrix_norm = 0.0_f64;
    for row in 0..n {
        for col in 0..n {
            poll_periodically(is_aborted, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            let value = matrix[row][col] / scale;
            scaled[row][col] = value;
            faer_matrix[(row, col)] = value;
            matrix_norm = matrix_norm.hypot(value);
        }
    }

    ensure_not_aborted(is_aborted)?;
    let eigen = match form {
        DenseSchurForm::Real => Eigen::<f64>::new_from_real(faer_matrix.as_ref()),
        DenseSchurForm::Complex => {
            let complex = Mat::from_fn(n, n, |row, col| {
                Complex64::new(faer_matrix[(row, col)], 0.0)
            });
            Eigen::<f64>::new(complex.as_ref())
        }
    };
    ensure_not_aborted(is_aborted)?;
    let eigen = eigen.map_err(|_| DenseEigenspectrumError::SolverFailure)?;

    let spectrum = eigen.S().column_vector();
    let eigenvectors = eigen.U();
    if spectrum.nrows() != n || eigenvectors.nrows() != n || eigenvectors.ncols() != n {
        return Err(DenseEigenspectrumError::IncompleteSpectrum {
            expected: n,
            eigenvalues: spectrum.nrows(),
            vector_rows: eigenvectors.nrows(),
            vector_cols: eigenvectors.ncols(),
        });
    }

    let mut eigenvalues = Vec::with_capacity(n);
    let mut right_eigenvectors = Vec::with_capacity(n);
    let mut max_backward_error = 0.0_f64;

    for index in 0..n {
        poll_periodically(is_aborted, operation_index)?;
        let value = *spectrum.get(index);
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(DenseEigenspectrumError::NonFiniteEigenvalue { index });
        }

        let mut vector = Vec::with_capacity(n);
        let mut vector_norm = 0.0_f64;
        for row in 0..n {
            poll_periodically(is_aborted, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            let component = eigenvectors[(row, index)];
            if !component.re.is_finite() || !component.im.is_finite() {
                return Err(DenseEigenspectrumError::InvalidEigenvector { index });
            }
            vector_norm = vector_norm.hypot(component.norm());
            vector.push(Complex64::new(component.re, component.im));
        }
        if !vector_norm.is_finite() || vector_norm == 0.0 {
            return Err(DenseEigenspectrumError::InvalidEigenvector { index });
        }

        let lambda = Complex64::new(value.re, value.im);
        let backward_error = eigenpair_backward_error(
            &scaled,
            matrix_norm,
            lambda,
            &vector,
            vector_norm,
            is_aborted,
            &mut operation_index,
        )?;
        if !backward_error.is_finite() || backward_error > tolerance {
            return Err(DenseEigenspectrumError::NumericalQualification {
                index,
                backward_error,
                maximum: tolerance,
            });
        }

        let root = Complex64::new(value.re * scale, value.im * scale);
        if !root.re.is_finite() || !root.im.is_finite() {
            return Err(DenseEigenspectrumError::NonFiniteEigenvalue { index });
        }
        max_backward_error = max_backward_error.max(backward_error);
        eigenvalues.push(root);
        right_eigenvectors.push(vector);
    }

    ensure_not_aborted(is_aborted)?;
    Ok(QualifiedDenseEigenspectrum {
        eigenvalues,
        right_eigenvectors,
        max_backward_error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEVER: &dyn Fn() -> bool = &|| false;

    fn tolerance(order: usize) -> f64 {
        128.0 * order.max(1) as f64 * f64::EPSILON
    }

    #[test]
    fn both_schur_forms_return_the_complete_real_and_complex_roots() {
        let matrix = vec![
            vec![0.0, -2.0, 0.0],
            vec![2.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        for form in [DenseSchurForm::Real, DenseSchurForm::Complex] {
            let spectrum =
                qualified_dense_eigenspectrum(&matrix, form, tolerance(3), NEVER).unwrap();
            assert_eq!(spectrum.eigenvalues.len(), 3);
            assert_eq!(spectrum.right_eigenvectors.len(), 3);
            assert!(spectrum.max_backward_error <= tolerance(3));
            for (re, im) in [(1.0, 0.0), (0.0, 2.0), (0.0, -2.0)] {
                assert!(
                    spectrum
                        .eigenvalues
                        .iter()
                        .any(|value| (value.re - re).abs() < 1e-12 && (value.im - im).abs() < 1e-12),
                    "{form:?} misses {re}{im:+}i in {:?}",
                    spectrum.eigenvalues
                );
            }
        }
    }

    #[test]
    fn invalid_inputs_are_rejected_before_the_eigensolve() {
        assert_eq!(
            qualified_dense_eigenspectrum(&[], DenseSchurForm::Real, 1.0, NEVER).unwrap_err(),
            DenseEigenspectrumError::EmptyMatrix
        );
        assert_eq!(
            qualified_dense_eigenspectrum(
                &[vec![1.0, 0.0], vec![0.0]],
                DenseSchurForm::Real,
                1.0,
                NEVER
            )
            .unwrap_err(),
            DenseEigenspectrumError::NonSquare {
                row: 1,
                expected: 2,
                actual: 1
            }
        );
        assert_eq!(
            qualified_dense_eigenspectrum(&[vec![f64::NAN]], DenseSchurForm::Real, 1.0, NEVER)
                .unwrap_err(),
            DenseEigenspectrumError::NonFiniteInput { row: 0, col: 0 }
        );
    }

    #[test]
    fn cancellation_is_polled_before_any_work() {
        assert_eq!(
            qualified_dense_eigenspectrum(&[vec![1.0]], DenseSchurForm::Complex, 1.0, &|| true)
                .unwrap_err(),
            DenseEigenspectrumError::Aborted
        );
    }

    #[test]
    fn residual_qualification_rejects_a_bad_non_normal_eigenpair() {
        // A wrong vector for this strongly non-normal Jordan block can have a
        // plausible eigenvalue while its normwise residual remains O(1).
        let matrix = vec![vec![1.0, 1.0e12], vec![0.0, 1.0]];
        let matrix_norm = matrix
            .iter()
            .flatten()
            .fold(0.0_f64, |norm, value| norm.hypot(*value));
        let vector = vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)];
        let mut operation_index = 0;
        let error = eigenpair_backward_error(
            &matrix,
            matrix_norm,
            Complex64::new(1.0, 0.0),
            &vector,
            1.0,
            NEVER,
            &mut operation_index,
        )
        .unwrap();

        assert!(error > tolerance(2));
    }
}
