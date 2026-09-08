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
//! Port counts are small — a handful, not thousands — but normalization still
//! uses the simulator's scale-aware complex LU and componentwise backward-error
//! certificate. A hand-written absolute pivot cutoff can reject a valid scaled
//! network or accept an inaccurate inverse, either of which silently corrupts
//! every S-parameter derived from it.

use crate::Complex64;
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};

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
    /// A solved port voltage was NaN or infinite.
    NonFinitePortVoltage {
        /// Zero-based port position.
        port: usize,
    },
    /// A network matrix contained a NaN or infinite entry.
    NonFiniteMatrixEntry { row: usize, column: usize },
    /// `(I + ZY)` is singular, so no scattering matrix exists at this point.
    ///
    /// Callers must surface this. Returning a zero-filled matrix instead —
    /// which one front-end used to do — presents a fabricated measurement as a
    /// real one.
    SingularNormalization,
    /// LU completed without a certifiably accurate finite inverse.
    NumericalFailure(String),
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
            Self::NonFinitePortVoltage { port } => write!(
                f,
                "S-parameter port {} has a non-finite solved voltage",
                port + 1
            ),
            Self::NonFiniteMatrixEntry { row, column } => write!(
                f,
                "S-parameter network matrix entry ({}, {}) is non-finite",
                row + 1,
                column + 1
            ),
            Self::SingularNormalization => {
                write!(f, "S-parameter normalization matrix is singular")
            }
            Self::NumericalFailure(message) => {
                write!(f, "S-parameter normalization solve failed: {message}")
            }
            Self::Aborted => write!(f, "S-parameter conversion was cancelled"),
        }
    }
}

impl std::error::Error for NetworkError {}

/// Invert a dense complex matrix through certified complex LU solves.
///
/// Returns `None` when the matrix is not square, is singular, or fails for an
/// input reason that this legacy convenience signature cannot represent. Code
/// that needs a typed diagnostic should call
/// [`invert_complex_matrix_with_abort`].
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
    if abort.is_aborted() {
        return Err(NetworkError::Aborted);
    }
    let mut structure = Vec::with_capacity(size);
    for (row, values) in matrix.iter().enumerate() {
        for (column, &value) in values.iter().enumerate() {
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(NetworkError::NonFiniteMatrixEntry { row, column });
            }
            if row == column || value != Complex64::new(0.0, 0.0) {
                structure.push((row, column, 0.0));
            }
        }
    }

    let real = StaticMatrix::from_triplets(size, size, &structure)
        .map_err(|error| NetworkError::NumericalFailure(error.to_string()))?;
    let mut workspace = ComplexMatrix::from_real_structure(&real);
    for (row, values) in matrix.iter().enumerate() {
        for (column, &value) in values.iter().enumerate() {
            if value != Complex64::new(0.0, 0.0) {
                workspace
                    .try_add(row, column, value)
                    .map_err(|error| NetworkError::NumericalFailure(error.to_string()))?;
            }
        }
    }

    let mut inverse = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    let mut rhs = vec![Complex64::new(0.0, 0.0); size];
    for column in 0..size {
        if column % ABORT_POLL_STRIDE == 0 && abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        rhs.fill(Complex64::new(0.0, 0.0));
        rhs[column] = Complex64::new(1.0, 0.0);
        let solution = match workspace.solve(&rhs) {
            Ok(solution) => solution,
            Err(SolverError::InaccurateSolution(_)) if size <= 64 => {
                match workspace.solve_dense_extended(&rhs) {
                    Ok(solution) => solution,
                    Err(SolverError::SingularMatrix | SolverError::PivotGrowth) => return Ok(None),
                    Err(error) => {
                        return Err(NetworkError::NumericalFailure(error.to_string()));
                    }
                }
            }
            Err(SolverError::SingularMatrix | SolverError::PivotGrowth) => return Ok(None),
            Err(error) => return Err(NetworkError::NumericalFailure(error.to_string())),
        };
        for (row, value) in solution.into_iter().enumerate() {
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(NetworkError::NumericalFailure(
                    "inverse contains a non-finite value".to_string(),
                ));
            }
            inverse[row][column] = value;
        }
    }
    Ok(Some(inverse))
}

/// One column of the scattering matrix, read straight off the port voltages of
/// a driven network.
///
/// With every port terminated in its own reference impedance and port `excited`
/// driven by a 1 V generator behind `Z0`, the wave amplitudes follow from the
/// port voltages alone:
///
/// ```text
/// S[j][k] = 2 * V[j] * sqrt(Z0[k] / Z0[j])     for j != k
/// S[k][k] = 2 * V[k] - 1
/// ```
///
/// The incident wave is fixed at `1 / (2*sqrt(Z0[k]))` by the generator, and
/// every undriven port is matched, so its voltage is purely the wave leaving
/// the network. No branch currents and no matrix inversion are involved, which
/// is why this works on networks that have no admittance representation at all
/// — the ones [`s_from_y`] can only report as a singular matrix.
///
/// `voltages[j]` is the complex voltage across port `j` at one frequency, taken
/// at its reference plane. Ports must be normalized to their Thevenin form
/// first, or an ideal source will hold its node at the generator value and
/// every reflection will read as zero.
pub fn s_column_from_port_voltages(
    voltages: &[Complex64],
    excited: usize,
    reference_impedances: &[Value],
) -> Result<Vec<Complex64>, NetworkError> {
    let size = voltages.len();
    if size == 0 || reference_impedances.len() != size || excited >= size {
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
    for (port, value) in voltages.iter().enumerate() {
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(NetworkError::NonFinitePortVoltage { port });
        }
    }

    let excited_z0 = reference_impedances[excited];
    let column = (0..size)
        .map(|row| {
            if row == excited {
                voltages[row] * 2.0 - Complex64::new(1.0, 0.0)
            } else {
                let source_scale = excited_z0.sqrt();
                let port_scale = reference_impedances[row].sqrt();
                let ratio = source_scale / port_scale;
                let scale = |value: Value| {
                    if value == 0.0 {
                        return value;
                    }
                    let result = value * 2.0 * ratio;
                    if ratio.is_normal() && result.is_normal() {
                        return result;
                    }
                    // Preserve finite waves when an intermediate ratio or
                    // product overflows/underflows. Keep the common path cheap.
                    let (value, value_exp) = libm::frexp(value);
                    let (source, source_exp) = libm::frexp(source_scale);
                    let (port, port_exp) = libm::frexp(port_scale);
                    libm::scalbn(
                        value * (source / port),
                        value_exp + source_exp - port_exp + 1,
                    )
                };
                Complex64::new(scale(voltages[row].re), scale(voltages[row].im))
            }
        })
        .collect::<Vec<_>>();
    if let Some(port) = column
        .iter()
        .position(|value| !value.re.is_finite() || !value.im.is_finite())
    {
        return Err(NetworkError::NumericalFailure(format!(
            "scattering column overflowed at port {}",
            port + 1
        )));
    }
    Ok(column)
}

/// Renormalize real power waves directly, including ideal open/short circuits.
/// With u = sqrt(R/Z), v = sqrt(Z/R), D = (u+v)/2 and C = (u-v)/2:
/// a_new = D a_old + C b_old, b_new = C a_old + D b_old.
/// Thus S_new = (C + D S_old) (D + C S_old)^-1. The diagonal is
/// evaluated through (1+S) and (1-S) to retain extreme open/short references.
pub(super) fn renormalize_with_abort(
    scattering: &mut [Vec<Complex64>],
    from: &[Value],
    to: &[Value],
    abort: &dyn AbortSignal,
) -> Result<(), NetworkError> {
    let size = scattering.len();
    if size == 0 || scattering.iter().any(|row| row.len() != size) {
        return Err(NetworkError::MalformedAdmittance {
            rows: size,
            impedances: to.len(),
        });
    }
    for references in [from, to] {
        if references.len() != size {
            return Err(NetworkError::MalformedAdmittance {
                rows: size,
                impedances: references.len(),
            });
        }
        for (port, &z0) in references.iter().enumerate() {
            if !z0.is_finite() || z0 <= 0.0 {
                return Err(NetworkError::InvalidReferenceImpedance { port, z0 });
            }
        }
    }
    let mut incident = vec![vec![Complex64::ZERO; size]; size];
    let mut reflected = vec![vec![Complex64::ZERO; size]; size];
    for row in 0..size {
        if row % ABORT_POLL_STRIDE == 0 && abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        let u = 0.5 * (from[row].sqrt() / to[row].sqrt());
        let v = 0.5 * (to[row].sqrt() / from[row].sqrt());
        for column in 0..size {
            let s = scattering[row][column];
            if !s.re.is_finite() || !s.im.is_finite() {
                return Err(NetworkError::NonFiniteMatrixEntry { row, column });
            }
            if row == column {
                let voltage = (Complex64::ONE + s) * u;
                let current = (Complex64::ONE - s) * v;
                incident[row][column] = voltage + current;
                reflected[row][column] = voltage - current;
            } else {
                incident[row][column] = s * (u - v);
                reflected[row][column] = s * (u + v);
            }
        }
    }
    let inverse = invert_complex_matrix_with_abort(&incident, abort)?
        .ok_or(NetworkError::SingularNormalization)?;
    for (row, output) in scattering.iter_mut().enumerate() {
        if row % ABORT_POLL_STRIDE == 0 && abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        for (column, entry) in output.iter_mut().enumerate() {
            let mut sum = Complex64::ZERO;
            let mut correction = Complex64::ZERO;
            for (inner, values) in inverse.iter().enumerate() {
                let term = reflected[row][inner] * values[column] - correction;
                let next = sum + term;
                correction = (next - sum) - term;
                sum = next;
            }
            if !sum.re.is_finite() || !sum.im.is_finite() {
                return Err(NetworkError::NumericalFailure(format!(
                    "scattering renormalization overflowed at ({}, {})",
                    row + 1,
                    column + 1
                )));
            }
            *entry = sum;
        }
    }
    Ok(())
}

/// Convert scattering parameters back to an N-port admittance matrix.
///
/// The exact inverse of [`s_from_y`]:
///
/// ```text
/// Y = D^-1 (I + S)^-1 (I - S) D^-1,   D = diag(sqrt(Z0_i))
/// ```
///
/// This exists so a caller that measured `S` need not also measure `Y`. Reading
/// admittance off the port branch currents is only valid when the port source is
/// ideal; behind a Thevenin generator the branch current is the one flowing
/// through the reference resistor, which is a different quantity.
///
/// Returns [`NetworkError::SingularNormalization`] when `I + S` is singular,
/// which is what a network with no admittance representation looks like from
/// here.
pub fn y_from_s(
    scattering: &[Vec<Complex64>],
    reference_impedances: &[Value],
) -> Result<Vec<Vec<Complex64>>, NetworkError> {
    let size = scattering.len();
    if size == 0
        || reference_impedances.len() != size
        || scattering.iter().any(|row| row.len() != size)
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

    let one = Complex64::new(1.0, 0.0);
    let mut sum = scattering.to_vec();
    let mut difference = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    for row in 0..size {
        for column in 0..size {
            difference[row][column] = -scattering[row][column];
        }
        sum[row][row] += one;
        difference[row][row] += one;
    }

    let inverse = invert_complex_matrix_with_abort(&sum, &NoAbort)?
        .ok_or(NetworkError::SingularNormalization)?;
    let inverse_scale = reference_impedances
        .iter()
        .map(|z0| 1.0 / z0.sqrt())
        .collect::<Vec<_>>();

    let mut admittance = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    for row in 0..size {
        for column in 0..size {
            let product: Complex64 = (0..size)
                .map(|k| inverse[row][k] * difference[k][column])
                .sum();
            let value = product * inverse_scale[row] * inverse_scale[column];
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(NetworkError::NumericalFailure(format!(
                    "admittance conversion overflowed at ({}, {})",
                    row + 1,
                    column + 1
                )));
            }
            admittance[row][column] = value;
        }
    }
    Ok(admittance)
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
            if !scattering[row][column].re.is_finite() || !scattering[row][column].im.is_finite() {
                return Err(NetworkError::NumericalFailure(format!(
                    "scattering conversion overflowed at ({}, {})",
                    row + 1,
                    column + 1
                )));
            }
        }
    }
    Ok(scattering)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renormalization_matches_a_complex_series_impedance() {
        let impedance = Complex64::new(30.0, 17.0);
        let series = |references: [Value; 2]| {
            let [z1, z2] = references;
            let total = impedance + z1 + z2;
            let transmission = 2.0 * z1.sqrt() * z2.sqrt() / total;
            vec![
                vec![(impedance + z2 - z1) / total, transmission],
                vec![transmission, (impedance + z1 - z2) / total],
            ]
        };
        let old = [25.0, 150.0];
        let new = [50.0, 75.0];
        let mut actual = series(old);
        renormalize_with_abort(&mut actual, &old, &new, &NoAbort).unwrap();
        let expected = series(new);
        for row in 0..2 {
            for column in 0..2 {
                assert!((actual[row][column] - expected[row][column]).norm() < 1e-14);
            }
        }
    }

    #[test]
    fn renormalization_preserves_ideal_opens_and_shorts_at_extreme_references() {
        for reflection in [-1.0, 1.0] {
            for (from, to) in [(50.0, 75.0), (1e-250, 1e250), (1e250, 1e-250)] {
                let mut matrix = vec![vec![Complex64::new(reflection, 0.0)]];
                renormalize_with_abort(&mut matrix, &[from], &[to], &NoAbort).unwrap();
                assert!((matrix[0][0].re - reflection).abs() < 1e-14);
                assert_eq!(matrix[0][0].im, 0.0);
            }
        }
    }

    #[test]
    fn renormalization_rejects_invalid_inputs_and_preserves_cancellation() {
        use crate::abort_signal::CountingAbort;
        let input = vec![vec![Complex64::new(0.6, 0.1)]];
        for invalid in [0.0, -1.0, Value::NAN, Value::INFINITY] {
            for (old, new) in [(invalid, 50.0), (50.0, invalid)] {
                assert!(matches!(
                    renormalize_with_abort(&mut input.clone(), &[old], &[new], &NoAbort),
                    Err(NetworkError::InvalidReferenceImpedance { .. })
                ));
            }
        }
        assert!(matches!(
            renormalize_with_abort(&mut input.clone(), &[], &[50.0], &NoAbort),
            Err(NetworkError::MalformedAdmittance { .. })
        ));
        assert!(matches!(
            renormalize_with_abort(
                &mut [vec![Complex64::new(Value::NAN, 0.0)]],
                &[25.0],
                &[50.0],
                &NoAbort
            ),
            Err(NetworkError::NonFiniteMatrixEntry { .. })
        ));
        assert!(matches!(
            renormalize_with_abort(
                &mut [vec![Complex64::new(1.25, 0.0)]],
                &[1.0],
                &[9.0],
                &NoAbort
            ),
            Err(NetworkError::SingularNormalization)
        ));
        let count = CountingAbort::new(usize::MAX);
        renormalize_with_abort(&mut input.clone(), &[25.0], &[50.0], &count).unwrap();
        for threshold in 0..count.count() {
            let abort = CountingAbort::new(threshold);
            assert_eq!(
                renormalize_with_abort(&mut input.clone(), &[25.0], &[50.0], &abort),
                Err(NetworkError::Aborted)
            );
            assert_eq!(abort.polls_after_abort(), 0);
        }
    }

    #[test]
    fn wave_extraction_avoids_overflow_in_impedance_ratio() {
        let column = s_column_from_port_voltages(
            &[Complex64::new(0.5, 0.0), Complex64::new(1e-200, 0.0)],
            0,
            &[1e200, 1e-200],
        )
        .unwrap();
        assert_eq!(column[0], Complex64::ZERO);
        assert!((column[1].re - 2.0).abs() < 1e-14);
        for (source, port, voltage, expected) in [
            (
                libm::scalbn(1.0, 1000),
                Value::from_bits(1),
                libm::scalbn(1.0, -1000),
                libm::scalbn(1.0, 38),
            ),
            (
                Value::from_bits(1),
                libm::scalbn(1.0, 1000),
                libm::scalbn(1.0, 1000),
                libm::scalbn(1.0, -36),
            ),
            (1.0, 4.0, libm::scalbn(1.0, 1023), libm::scalbn(1.0, 1023)),
        ] {
            let column = s_column_from_port_voltages(
                &[Complex64::new(0.5, 0.0), Complex64::new(voltage, -voltage)],
                0,
                &[source, port],
            )
            .unwrap();
            assert_eq!(column[1], Complex64::new(expected, -expected));
        }
    }

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

    /// The two extractions must agree wherever both are defined.
    ///
    /// Driving a series resistor between two terminated ports is a circuit with
    /// a closed-form solution, so the port voltages here are exact rather than
    /// simulated: `1 V` behind `Z1`, through `R`, into `Z2`. Feeding those to
    /// the wave formula must reproduce, column for column, what inverting the
    /// admittance matrix produces — otherwise the new path is measuring a
    /// different network from the one the old path measured.
    #[test]
    fn wave_extraction_agrees_with_the_admittance_route() {
        let r = 50.0;
        let (z1, z2) = (75.0, 50.0);
        let y = vec![
            vec![Complex64::new(1.0 / r, 0.0), Complex64::new(-1.0 / r, 0.0)],
            vec![Complex64::new(-1.0 / r, 0.0), Complex64::new(1.0 / r, 0.0)],
        ];
        let expected = s_from_y(&y, &[z1, z2]).expect("conversion succeeds");

        let total = z1 + r + z2;
        // Port voltages with port 1 driven, then with port 2 driven.
        let driven = [
            [(r + z2) / total, z2 / total],
            [z1 / total, (r + z1) / total],
        ];

        for (excited, voltages) in driven.iter().enumerate() {
            let column = s_column_from_port_voltages(
                &voltages.map(|v| Complex64::new(v, 0.0)),
                excited,
                &[z1, z2],
            )
            .expect("wave extraction succeeds");
            for (row, value) in column.iter().enumerate() {
                assert!(
                    (value - expected[row][excited]).norm() < 1e-12,
                    "S[{row}][{excited}]: wave {value} vs admittance {}",
                    expected[row][excited]
                );
            }
        }
    }

    /// A network with no admittance representation still has scattering
    /// parameters, and the wave route is the one that can reach them.
    ///
    /// An ideal series voltage source between the ports forces `V1 - V2` to a
    /// constant whatever current flows, so no finite `Y` exists — the case the
    /// admittance route can only report as a singular matrix.
    #[test]
    fn wave_extraction_handles_a_network_with_no_admittance_matrix() {
        let (z1, z2) = (50.0, 50.0);
        // 1 V behind z1, a 0 V series source, then z2: the loop current is
        // 1/(z1+z2) and both planes sit at the resulting divider voltage.
        let current = 1.0 / (z1 + z2);
        let voltages = [
            Complex64::new(1.0 - current * z1, 0.0),
            Complex64::new(current * z2, 0.0),
        ];

        let column =
            s_column_from_port_voltages(&voltages, 0, &[z1, z2]).expect("wave extraction succeeds");

        // A through-connection with matched terminations: no reflection, full
        // transmission.
        assert!(column[0].norm() < 1e-12, "S11 = {}", column[0]);
        assert!(
            (column[1] - Complex64::new(1.0, 0.0)).norm() < 1e-12,
            "S21 = {}",
            column[1]
        );
    }

    /// `y_from_s` must be the exact inverse of `s_from_y`, including the `D`
    /// scaling that only shows up when the reference impedances differ.
    #[test]
    fn admittance_round_trips_through_scattering() {
        let y = vec![
            vec![Complex64::new(0.02, 0.01), Complex64::new(-0.005, 0.002)],
            vec![Complex64::new(-0.005, 0.002), Complex64::new(0.01, -0.003)],
        ];
        let z0 = [50.0, 75.0];

        let s = s_from_y(&y, &z0).expect("conversion succeeds");
        let recovered = y_from_s(&s, &z0).expect("inverse succeeds");

        for (row, values) in y.iter().enumerate() {
            for (column, value) in values.iter().enumerate() {
                assert!(
                    (recovered[row][column] - value).norm() < 1e-12,
                    "Y[{row}][{column}] = {}, expected {value}",
                    recovered[row][column]
                );
            }
        }
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
    fn nonfinite_network_data_is_rejected() {
        assert_eq!(
            s_column_from_port_voltages(&[Complex64::new(Value::NAN, 0.0)], 0, &[50.0],),
            Err(NetworkError::NonFinitePortVoltage { port: 0 })
        );

        let nonfinite = vec![vec![Complex64::new(Value::NAN, 0.0)]];
        assert_eq!(
            s_from_y(&nonfinite, &[50.0]),
            Err(NetworkError::NonFiniteMatrixEntry { row: 0, column: 0 })
        );
        assert_eq!(
            y_from_s(&nonfinite, &[50.0]),
            Err(NetworkError::NonFiniteMatrixEntry { row: 0, column: 0 })
        );
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
        for (row, m_row) in m.iter().enumerate() {
            for column in 0..2 {
                let mut acc = Complex64::new(0.0, 0.0);
                for (coefficient, inv_row) in m_row.iter().zip(&inv) {
                    acc += coefficient * inv_row[column];
                }
                let expected = if row == column { 1.0 } else { 0.0 };
                assert!((acc - Complex64::new(expected, 0.0)).norm() < 1e-12);
            }
        }
        assert!(invert_complex_matrix(&[]).is_none());
    }

    #[test]
    fn inverse_has_no_absolute_pivot_floor() {
        let tiny = vec![vec![Complex64::new(1.0e-30, 0.0)]];
        let inverse = invert_complex_matrix(&tiny).expect("scaled 1x1 matrix is nonsingular");
        assert!((inverse[0][0].re - 1.0e30).abs() <= 1.0e15);
        assert_eq!(inverse[0][0].im, 0.0);
    }
}
