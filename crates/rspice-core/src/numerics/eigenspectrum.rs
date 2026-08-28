//! Strict ordinary eigenspectrum extraction shared by periodic analyses.
//!
//! A successful result is complete and carries a normwise residual
//! certificate.  The faer eigensolve itself is an atomic call, so cooperative
//! cancellation is checked immediately before and after it and throughout the
//! surrounding validation, conversion, and qualification work.

use crate::Value;
use crate::abort_signal::AbortSignal;
use faer::Mat;
use faer::linalg::solvers::Eigen;
use num_complex::Complex64;

const ABORT_POLL_STRIDE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct OrdinarySpectrumCertificate {
    pub(crate) problem_order: usize,
    pub(crate) max_backward_error: Value,
    pub(crate) qualification_tolerance: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct QualifiedOrdinarySpectrum {
    pub(crate) eigenvalues: Vec<Complex64>,
    /// Right eigenvectors stored one vector per eigenvalue.
    pub(crate) right_eigenvectors: Vec<Vec<Complex64>>,
    pub(crate) certificate: OrdinarySpectrumCertificate,
}

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub(crate) enum OrdinarySpectrumError {
    #[error("eigenspectrum computation was aborted")]
    Aborted,
    #[error("eigenproblem matrix must be non-empty")]
    EmptyMatrix,
    #[error(
        "eigenproblem matrix must be square: row {row} has {actual} columns, expected {expected}"
    )]
    NonSquare {
        row: usize,
        expected: usize,
        actual: usize,
    },
    #[error("eigenproblem matrix entry ({row}, {col}) is non-finite")]
    NonFiniteInput { row: usize, col: usize },
    #[error("faer ordinary eigenspectrum extraction failed")]
    SolverFailure,
    #[error(
        "ordinary eigenspectrum is incomplete: expected {expected} eigenpairs, received {eigenvalues} eigenvalues and a {vector_rows}x{vector_cols} eigenvector matrix"
    )]
    IncompleteSpectrum {
        expected: usize,
        eigenvalues: usize,
        vector_rows: usize,
        vector_cols: usize,
    },
    #[error("ordinary eigenvalue {index} is non-finite")]
    NonFiniteEigenvalue { index: usize },
    #[error("ordinary right eigenvector {index} is non-finite or zero")]
    InvalidEigenvector { index: usize },
    #[error(
        "ordinary eigenpair {index} failed residual qualification (backward error {backward_error:.3e}, maximum {maximum:.3e})"
    )]
    NumericalQualification {
        index: usize,
        backward_error: Value,
        maximum: Value,
    },
}

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), OrdinarySpectrumError> {
    if abort.is_aborted() {
        Err(OrdinarySpectrumError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_periodically(
    abort: &dyn AbortSignal,
    operation_index: usize,
) -> Result<(), OrdinarySpectrumError> {
    if operation_index % ABORT_POLL_STRIDE == 0 {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[inline]
pub(crate) fn qualification_tolerance(problem_order: usize) -> Value {
    128.0 * problem_order.max(1) as Value * Value::EPSILON
}

fn eigenpair_backward_error(
    matrix: &[Vec<Value>],
    matrix_norm: Value,
    lambda: Complex64,
    vector: &[Complex64],
    vector_norm: Value,
    abort: &dyn AbortSignal,
    operation_index: &mut usize,
) -> Result<Value, OrdinarySpectrumError> {
    let mut residual_norm = 0.0_f64;
    for row in 0..matrix.len() {
        let mut product = Complex64::new(0.0, 0.0);
        for col in 0..matrix.len() {
            poll_periodically(abort, *operation_index)?;
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
        Value::INFINITY
    })
}

pub(crate) fn qualified_real_eigenspectrum(
    matrix: &[Vec<Value>],
    abort: &dyn AbortSignal,
) -> Result<QualifiedOrdinarySpectrum, OrdinarySpectrumError> {
    ensure_not_aborted(abort)?;
    let n = matrix.len();
    if n == 0 {
        return Err(OrdinarySpectrumError::EmptyMatrix);
    }

    let mut max_abs = 0.0_f64;
    let mut operation_index = 0usize;
    for (row_index, row) in matrix.iter().enumerate() {
        poll_periodically(abort, operation_index)?;
        if row.len() != n {
            return Err(OrdinarySpectrumError::NonSquare {
                row: row_index,
                expected: n,
                actual: row.len(),
            });
        }
        for (col_index, &value) in row.iter().enumerate() {
            poll_periodically(abort, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            if !value.is_finite() {
                return Err(OrdinarySpectrumError::NonFiniteInput {
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
            poll_periodically(abort, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            let value = matrix[row][col] / scale;
            scaled[row][col] = value;
            faer_matrix[(row, col)] = value;
            matrix_norm = matrix_norm.hypot(value);
        }
    }

    ensure_not_aborted(abort)?;
    let eigen = Eigen::<f64>::new_from_real(faer_matrix.as_ref());
    ensure_not_aborted(abort)?;
    let eigen = eigen.map_err(|_| OrdinarySpectrumError::SolverFailure)?;

    let spectrum = eigen.S().column_vector();
    let eigenvectors = eigen.U();
    if spectrum.nrows() != n || eigenvectors.nrows() != n || eigenvectors.ncols() != n {
        return Err(OrdinarySpectrumError::IncompleteSpectrum {
            expected: n,
            eigenvalues: spectrum.nrows(),
            vector_rows: eigenvectors.nrows(),
            vector_cols: eigenvectors.ncols(),
        });
    }

    let tolerance = qualification_tolerance(n);
    let mut eigenvalues = Vec::with_capacity(n);
    let mut right_eigenvectors = Vec::with_capacity(n);
    let mut max_backward_error = 0.0_f64;

    for index in 0..n {
        poll_periodically(abort, operation_index)?;
        let value = *spectrum.get(index);
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(OrdinarySpectrumError::NonFiniteEigenvalue { index });
        }

        let mut vector = Vec::with_capacity(n);
        let mut vector_norm = 0.0_f64;
        for row in 0..n {
            poll_periodically(abort, operation_index)?;
            operation_index = operation_index.wrapping_add(1);
            let component = eigenvectors[(row, index)];
            if !component.re.is_finite() || !component.im.is_finite() {
                return Err(OrdinarySpectrumError::InvalidEigenvector { index });
            }
            vector_norm = vector_norm.hypot(component.norm());
            vector.push(Complex64::new(component.re, component.im));
        }
        if !vector_norm.is_finite() || vector_norm == 0.0 {
            return Err(OrdinarySpectrumError::InvalidEigenvector { index });
        }

        let lambda = Complex64::new(value.re, value.im);
        let backward_error = eigenpair_backward_error(
            &scaled,
            matrix_norm,
            lambda,
            &vector,
            vector_norm,
            abort,
            &mut operation_index,
        )?;
        if !backward_error.is_finite() || backward_error > tolerance {
            return Err(OrdinarySpectrumError::NumericalQualification {
                index,
                backward_error,
                maximum: tolerance,
            });
        }

        let root = Complex64::new(value.re * scale, value.im * scale);
        if !root.re.is_finite() || !root.im.is_finite() {
            return Err(OrdinarySpectrumError::NonFiniteEigenvalue { index });
        }
        max_backward_error = max_backward_error.max(backward_error);
        eigenvalues.push(root);
        right_eigenvectors.push(vector);
    }

    ensure_not_aborted(abort)?;
    Ok(QualifiedOrdinarySpectrum {
        eigenvalues,
        right_eigenvectors,
        certificate: OrdinarySpectrumCertificate {
            problem_order: n,
            max_backward_error,
            qualification_tolerance: tolerance,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{ImmediateAbort, NoAbort};

    #[test]
    fn qualified_spectrum_returns_complete_real_and_complex_roots() {
        let matrix = vec![
            vec![0.0, -1.0, 0.0],
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.5],
        ];
        let spectrum = qualified_real_eigenspectrum(&matrix, &NoAbort).unwrap();

        assert_eq!(spectrum.eigenvalues.len(), 3);
        assert_eq!(spectrum.right_eigenvectors.len(), 3);
        assert_eq!(spectrum.certificate.problem_order, 3);
        assert!(
            spectrum.certificate.max_backward_error <= spectrum.certificate.qualification_tolerance
        );
        assert!(
            spectrum
                .eigenvalues
                .iter()
                .any(|value| (value.re - 0.5).abs() < 1e-12 && value.im.abs() < 1e-12)
        );
        assert!(
            spectrum
                .eigenvalues
                .iter()
                .any(|value| value.re.abs() < 1e-12 && (value.im - 1.0).abs() < 1e-12)
        );
        assert!(
            spectrum
                .eigenvalues
                .iter()
                .any(|value| value.re.abs() < 1e-12 && (value.im + 1.0).abs() < 1e-12)
        );
    }

    #[test]
    fn qualified_spectrum_rejects_invalid_inputs_without_panicking() {
        assert_eq!(
            qualified_real_eigenspectrum(&[], &NoAbort).unwrap_err(),
            OrdinarySpectrumError::EmptyMatrix
        );
        assert!(matches!(
            qualified_real_eigenspectrum(&[vec![1.0, 0.0], vec![0.0]], &NoAbort),
            Err(OrdinarySpectrumError::NonSquare { row: 1, .. })
        ));
        assert_eq!(
            qualified_real_eigenspectrum(&[vec![Value::NAN]], &NoAbort).unwrap_err(),
            OrdinarySpectrumError::NonFiniteInput { row: 0, col: 0 }
        );
    }

    #[test]
    fn qualified_spectrum_honors_entry_abort() {
        assert_eq!(
            qualified_real_eigenspectrum(&[vec![1.0]], &ImmediateAbort).unwrap_err(),
            OrdinarySpectrumError::Aborted
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
            &NoAbort,
            &mut operation_index,
        )
        .unwrap();

        assert!(error > qualification_tolerance(2));
    }
}
