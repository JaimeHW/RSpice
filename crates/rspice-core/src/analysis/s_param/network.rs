//! N-port admittance-to-scattering conversion with per-port reference impedances.
//!
//! This is the shared implementation behind every front-end's `.SP` support.
//! It previously existed as three independent copies — one each in the Python
//! bindings, the CLI, and the desktop runner — which had already drifted in
//! their handling of a singular normalization matrix.
//!
//! # Convention
//!
//! Power waves, with real per-port reference impedances:
//!
//! ```text
//! S = D^-1 (I - ZY) (I + ZY)^-1 D,   Z = diag(Z0_i),  D = diag(sqrt(Z0_i))
//! ```
//!
//! which reduces to `S[i][j] = M[i][j] * sqrt(Z0_j / Z0_i)` where
//! `M = (I - ZY)(I + ZY)^-1`. When every port shares one reference impedance
//! the `D` factors cancel and the scaling is invisible, so a test suite built
//! only on 50-ohm ports cannot tell this apart from dropping `D` entirely.
//!
//! Note that `S` is symmetric for a reciprocal network *even when the
//! reference impedances differ*: both `S` and its transpose are functions of
//! `DYD`, which is symmetric whenever `Y` is. Asymmetry is therefore not a
//! valid signal that the normalization was applied.
//!
//! # Numerics
//!
//! The inverse is Gauss-Jordan with partial pivoting on complex magnitude.
//! Port counts are small — a handful, not thousands — so the O(n^3) dense
//! solve is not worth specializing, and the sparse solver's setup cost would
//! dominate. This is deliberately separate from the single-RHS Gaussian solve
//! in the harmonic-balance solver: that one solves `Ax = b` for one `b` and
//! reports through `HbError`, whereas normalization needs a full inverse.

use crate::Complex64;
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};

/// Rows processed between abort polls.
///
/// Port counts are usually small, but package and connector models reach into
/// the hundreds, where an O(n^3) inverse is long enough that a GUI must stay
/// cancellable through it.
const ABORT_POLL_STRIDE: usize = 8;

/// Why an N-port network conversion could not be performed.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkError {
    /// The admittance matrix was not square, or its size did not match the
    /// supplied reference impedances.
    MalformedAdmittance { rows: usize, impedances: usize },
    /// The caller's abort signal fired part-way through.
    Aborted,
    /// A reference impedance was not a positive, finite resistance.
    InvalidReferenceImpedance {
        /// Zero-based port position.
        port: usize,
        z0: Value,
    },
    /// `(I + ZY)` is singular, so no scattering matrix exists at this point.
    ///
    /// Callers must surface this. Returning a zero-filled matrix instead —
    /// which one front-end used to do — presents a fabricated measurement as a
    /// real one.
    SingularNormalization,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedAdmittance { rows, impedances } => write!(
                f,
                "malformed S-parameter admittance matrix: {rows} row(s) against {impedances} \
                 reference impedance(s)"
            ),
            Self::InvalidReferenceImpedance { port, z0 } => write!(
                f,
                "S-parameter port {} has invalid z0 {z0}; expected a positive impedance",
                port + 1
            ),
            Self::SingularNormalization => {
                write!(f, "S-parameter normalization matrix is singular")
            }
            Self::Aborted => write!(f, "S-parameter conversion was cancelled"),
        }
    }
}

impl std::error::Error for NetworkError {}

/// Invert a dense complex matrix by Gauss-Jordan elimination.
///
/// Returns `None` when the matrix is not square or is numerically singular.
/// The pivot floor is absolute rather than scaled because the matrices reaching
/// this function are `I + ZY`, whose entries are already normalized to order
/// unity by construction.
pub fn invert_complex_matrix(matrix: &[Vec<Complex64>]) -> Option<Vec<Vec<Complex64>>> {
    invert_complex_matrix_with_abort(matrix, &NoAbort)
        .ok()
        .flatten()
}

/// [`invert_complex_matrix`], cancellable through `abort`.
///
/// `Ok(None)` means singular or malformed; [`NetworkError::Aborted`] means
/// cancelled. The two are kept distinct because a cancelled run must never be
/// reported to the user as a singular matrix.
pub fn invert_complex_matrix_with_abort(
    matrix: &[Vec<Complex64>],
    abort: &dyn AbortSignal,
) -> Result<Option<Vec<Vec<Complex64>>>, NetworkError> {
    let size = matrix.len();
    if size == 0 || matrix.iter().any(|row| row.len() != size) {
        return Ok(None);
    }
    let zero = Complex64::new(0.0, 0.0);
    let one = Complex64::new(1.0, 0.0);
    let mut augmented = vec![vec![zero; 2 * size]; size];
    for row in 0..size {
        augmented[row][..size].copy_from_slice(&matrix[row]);
        augmented[row][size + row] = one;
    }
    for column in 0..size {
        if column % ABORT_POLL_STRIDE == 0 && abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        let Some(pivot) = (column..size).max_by(|&lhs, &rhs| {
            augmented[lhs][column]
                .norm()
                .total_cmp(&augmented[rhs][column].norm())
        }) else {
            return Ok(None);
        };
        if augmented[pivot][column].norm() <= 1e-24 {
            return Ok(None);
        }
        augmented.swap(pivot, column);
        let pivot_value = augmented[column][column];
        for value in &mut augmented[column] {
            *value /= pivot_value;
        }
        let pivot_row = augmented[column].clone();
        for (row, values) in augmented.iter_mut().enumerate() {
            if row == column {
                continue;
            }
            let factor = values[column];
            for index in 0..2 * size {
                values[index] -= factor * pivot_row[index];
            }
        }
    }
    Ok(Some(
        augmented
            .into_iter()
            .map(|row| row[size..].to_vec())
            .collect(),
    ))
}

/// Convert an N-port admittance matrix to scattering parameters.
///
/// `admittance` is indexed `[row][column]`; `reference_impedances` gives one
/// real, positive reference impedance per port, in port order.
pub fn s_from_y(
    admittance: &[Vec<Complex64>],
    reference_impedances: &[Value],
) -> Result<Vec<Vec<Complex64>>, NetworkError> {
    s_from_y_with_abort(admittance, reference_impedances, &NoAbort)
}

/// [`s_from_y`], cancellable through `abort`.
pub fn s_from_y_with_abort(
    admittance: &[Vec<Complex64>],
    reference_impedances: &[Value],
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<Complex64>>, NetworkError> {
    let size = admittance.len();
    if size == 0
        || reference_impedances.len() != size
        || admittance.iter().any(|row| row.len() != size)
    {
        return Err(NetworkError::MalformedAdmittance {
            rows: size,
            impedances: reference_impedances.len(),
        });
    }
    for (port, &z0) in reference_impedances.iter().enumerate() {
        if !z0.is_finite() || z0 <= 0.0 {
            return Err(NetworkError::InvalidReferenceImpedance { port, z0 });
        }
    }

    let zero = Complex64::new(0.0, 0.0);
    let one = Complex64::new(1.0, 0.0);
    let mut plus = vec![vec![zero; size]; size];
    let mut minus = vec![vec![zero; size]; size];
    for row in 0..size {
        for column in 0..size {
            let identity = if row == column { one } else { zero };
            let normalized = reference_impedances[row] * admittance[row][column];
            plus[row][column] = identity + normalized;
            minus[row][column] = identity - normalized;
        }
    }
    let inverse = invert_complex_matrix_with_abort(&plus, abort)?
        .ok_or(NetworkError::SingularNormalization)?;
    let mut scattering = vec![vec![zero; size]; size];
    for row in 0..size {
        if row % ABORT_POLL_STRIDE == 0 && abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        for column in 0..size {
            for inner in 0..size {
                scattering[row][column] += minus[row][inner] * inverse[inner][column];
            }
            scattering[row][column] *=
                (reference_impedances[column] / reference_impedances[row]).sqrt();
        }
    }
    Ok(scattering)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Closed form for a series resistance bridging two ports.
    ///
    /// With `a = z1/R`, `b = z2/R`, `c = sqrt(z1*z2)/R` and `c^2 = a*b`:
    /// `S11 = (1 + b - a)/(1 + a + b)`, `S22 = (1 + a - b)/(1 + a + b)`,
    /// `S12 = S21 = 2c/(1 + a + b)`. Dropping the `D` scaling would give
    /// `2b/(1 + a + b)` for `S21`, which differs whenever `z1 != z2`.
    #[test]
    fn series_resistor_matches_closed_form_with_unequal_reference_impedances() {
        let r = 50.0;
        let (z1, z2) = (75.0, 50.0);
        let y = vec![
            vec![Complex64::new(1.0 / r, 0.0), Complex64::new(-1.0 / r, 0.0)],
            vec![Complex64::new(-1.0 / r, 0.0), Complex64::new(1.0 / r, 0.0)],
        ];

        let s = s_from_y(&y, &[z1, z2]).expect("conversion succeeds");

        let (a, b) = (z1 / r, z2 / r);
        let c = (z1 * z2).sqrt() / r;
        let total = 1.0 + a + b;
        assert!((s[0][0].re - (1.0 + b - a) / total).abs() < 1e-12);
        assert!((s[1][1].re - (1.0 + a - b) / total).abs() < 1e-12);
        assert!((s[1][0].re - 2.0 * c / total).abs() < 1e-12);
        assert!((s[0][1].re - 2.0 * c / total).abs() < 1e-12);
        // The value an unnormalized implementation would produce.
        assert!((2.0 * c / total - 2.0 * b / total).abs() > 0.1);
    }

    #[test]
    fn reciprocal_network_stays_symmetric_across_unequal_reference_impedances() {
        let y = vec![
            vec![Complex64::new(0.02, 0.01), Complex64::new(-0.005, 0.002)],
            vec![Complex64::new(-0.005, 0.002), Complex64::new(0.01, -0.003)],
        ];
        let s = s_from_y(&y, &[50.0, 75.0]).expect("conversion succeeds");
        assert!((s[0][1] - s[1][0]).norm() < 1e-14);
    }

    #[test]
    fn matched_load_is_reflectionless() {
        let z0 = 50.0;
        let y = vec![vec![Complex64::new(1.0 / z0, 0.0)]];
        let s = s_from_y(&y, &[z0]).expect("conversion succeeds");
        assert!(s[0][0].norm() < 1e-15);
    }

    #[test]
    fn singular_normalization_is_reported_not_zero_filled() {
        // Y = -I/Z0 makes I + ZY exactly zero.
        let z0 = 50.0;
        let y = vec![
            vec![Complex64::new(-1.0 / z0, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(0.0, 0.0), Complex64::new(-1.0 / z0, 0.0)],
        ];
        assert_eq!(
            s_from_y(&y, &[z0, z0]),
            Err(NetworkError::SingularNormalization)
        );
    }

    #[test]
    fn malformed_and_invalid_inputs_are_rejected() {
        let y = vec![vec![Complex64::new(0.02, 0.0)]];
        assert!(matches!(
            s_from_y(&y, &[50.0, 50.0]),
            Err(NetworkError::MalformedAdmittance { .. })
        ));
        assert!(matches!(
            s_from_y(&y, &[0.0]),
            Err(NetworkError::InvalidReferenceImpedance { port: 0, .. })
        ));
        assert!(matches!(
            s_from_y(&y, &[f64::NAN]),
            Err(NetworkError::InvalidReferenceImpedance { port: 0, .. })
        ));
    }

    #[test]
    fn conversion_is_cancellable_part_way_through() {
        use crate::abort_signal::CountingAbort;

        // Large enough that the O(n^3) inverse is worth interrupting; this is
        // the size regime a package or connector model reaches.
        const PORTS: usize = 128;
        let mut y = vec![vec![Complex64::new(0.0, 0.0); PORTS]; PORTS];
        for (index, row) in y.iter_mut().enumerate() {
            row[index] = Complex64::new(1e-3, 0.0);
        }
        let abort = CountingAbort::new(5);

        let result = s_from_y_with_abort(&y, &vec![50.0; PORTS], &abort);

        assert_eq!(result, Err(NetworkError::Aborted));
        assert!(abort.count() > 5);
    }

    #[test]
    fn cancellation_is_distinct_from_singularity() {
        use crate::abort_signal::ImmediateAbort;

        let z0 = 50.0;
        let singular = vec![
            vec![Complex64::new(-1.0 / z0, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(0.0, 0.0), Complex64::new(-1.0 / z0, 0.0)],
        ];
        // Even a genuinely singular matrix reports cancellation when the abort
        // fires first: a cancelled run must never be shown as a bad circuit.
        assert_eq!(
            s_from_y_with_abort(&singular, &[z0, z0], &ImmediateAbort),
            Err(NetworkError::Aborted)
        );
    }

    #[test]
    fn inverse_round_trips_a_well_conditioned_matrix() {
        let m = vec![
            vec![Complex64::new(2.0, 1.0), Complex64::new(0.5, -0.25)],
            vec![Complex64::new(-1.0, 0.75), Complex64::new(3.0, 0.0)],
        ];
        let inv = invert_complex_matrix(&m).expect("invertible");
        for row in 0..2 {
            for column in 0..2 {
                let mut acc = Complex64::new(0.0, 0.0);
                for inner in 0..2 {
                    acc += m[row][inner] * inv[inner][column];
                }
                let expected = if row == column { 1.0 } else { 0.0 };
                assert!((acc - Complex64::new(expected, 0.0)).norm() < 1e-12);
            }
        }
        assert!(invert_complex_matrix(&[]).is_none());
    }
}
