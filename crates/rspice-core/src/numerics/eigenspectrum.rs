//! Strict ordinary eigenspectrum extraction shared by periodic analyses.
//!
//! A successful result is complete and carries a normwise residual
//! certificate. The eigensolve and its O(n³) residual qualification run in
//! [`rspice_matrix::qualified_dense_eigenspectrum`], which the development and
//! test profiles optimize; faer's eigen code is generic and would otherwise be
//! compiled unoptimized here. Cooperative cancellation is polled before and
//! after the atomic eigensolve and throughout the surrounding validation,
//! extraction and qualification work.

use crate::Value;
use crate::abort_signal::AbortSignal;
use num_complex::Complex64;
use rspice_matrix::{DenseSchurForm, qualified_dense_eigenspectrum};

pub(crate) use rspice_matrix::DenseEigenspectrumError as OrdinarySpectrumError;

/// Residual certificate for one complete ordinary Floquet eigenspectrum.
///
/// Unlike a generalized pole-zero certificate, a Floquet certificate has no
/// infinite-root accounting: a qualified result contains exactly
/// `problem_order` finite multipliers.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct FloquetSpectrumCertificate {
    /// Order of the monodromy eigenproblem.
    pub problem_order: usize,
    /// Largest normwise backward error among all returned eigenpairs.
    pub max_backward_error: Value,
    /// Canonical strict qualification threshold.
    pub qualification_tolerance: Value,
}

impl FloquetSpectrumCertificate {
    /// Canonical qualification threshold for an ordinary spectrum.
    #[inline]
    pub fn canonical_qualification_tolerance(problem_order: usize) -> Value {
        128.0 * problem_order.max(1) as Value * Value::EPSILON
    }

    /// Construct a strictly qualified certificate.
    ///
    /// The tolerance must match the canonical `128 * max(n, 1) * EPSILON`
    /// value exactly; callers cannot inflate it to authenticate a weaker
    /// eigenspectrum. Zero-order systems use the analysis layer's explicit
    /// no-dynamic-modes evidence instead of a certificate.
    pub fn new(
        problem_order: usize,
        max_backward_error: Value,
        qualification_tolerance: Value,
    ) -> Option<Self> {
        let certificate = Self {
            problem_order,
            max_backward_error,
            qualification_tolerance,
        };
        certificate.is_valid().then_some(certificate)
    }

    /// Whether this is a canonical strict certificate.
    pub fn is_valid(self) -> bool {
        self.problem_order > 0
            && self.max_backward_error.is_finite()
            && self.max_backward_error >= 0.0
            && self.qualification_tolerance
                == Self::canonical_qualification_tolerance(self.problem_order)
            && self.max_backward_error <= self.qualification_tolerance
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct QualifiedOrdinarySpectrum {
    pub(crate) eigenvalues: Vec<Complex64>,
    /// Right eigenvectors stored one vector per eigenvalue.
    pub(crate) right_eigenvectors: Vec<Vec<Complex64>>,
    pub(crate) certificate: FloquetSpectrumCertificate,
}

#[inline]
pub(crate) fn qualification_tolerance(problem_order: usize) -> Value {
    FloquetSpectrumCertificate::canonical_qualification_tolerance(problem_order)
}

pub(crate) fn qualified_real_eigenspectrum(
    matrix: &[Vec<Value>],
    abort: &dyn AbortSignal,
) -> Result<QualifiedOrdinarySpectrum, OrdinarySpectrumError> {
    match qualified_eigenspectrum_impl(matrix, abort, DenseSchurForm::Real) {
        // faer 0.24's real-Schur eigenvectors are not backward stable inside
        // a cluster of complex multipliers of radius about sqrt(eps * s),
        // s being the entrywise 1-norm of the Schur form T, such as the
        // rounding images of a defective zero multiplier. Its back
        // substitution solves each 2x2 block of T - lambda*I by Cramer's
        // rule and, when the determinant falls below eps * s, replaces the
        // determinant but keeps the adjugate. That is no perturbation of the
        // block, so the block's residual stays the size of its right-hand
        // side. The complex Schur back substitution clamps a scalar pivot
        // instead, which is a perturbation within the bound. A defective
        // block can also yield a zero or nonfinite vector. Retry a complex
        // Schur solve, then apply the identical residual certificate in the
        // original coordinates. Never relax the qualification bound.
        Err(
            OrdinarySpectrumError::NumericalQualification { .. }
            | OrdinarySpectrumError::InvalidEigenvector { .. },
        ) => qualified_eigenspectrum_impl(matrix, abort, DenseSchurForm::Complex),
        result => result,
    }
}

fn qualified_eigenspectrum_impl(
    matrix: &[Vec<Value>],
    abort: &dyn AbortSignal,
    form: DenseSchurForm,
) -> Result<QualifiedOrdinarySpectrum, OrdinarySpectrumError> {
    let n = matrix.len();
    let tolerance = qualification_tolerance(n);
    let spectrum = qualified_dense_eigenspectrum(matrix, form, tolerance, &|| abort.is_aborted())?;
    let certificate = FloquetSpectrumCertificate::new(n, spectrum.max_backward_error, tolerance)
        .ok_or(OrdinarySpectrumError::NumericalQualification {
            index: n.saturating_sub(1),
            backward_error: spectrum.max_backward_error,
            maximum: tolerance,
        })?;
    Ok(QualifiedOrdinarySpectrum {
        eigenvalues: spectrum.eigenvalues,
        right_eigenvectors: spectrum.right_eigenvectors,
        certificate,
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
}
