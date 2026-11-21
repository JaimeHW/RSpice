//! A network's scattering matrix read as another parameter set: admittance,
//! impedance, hybrid and chain (ABCD) parameters.
//!
//! Each wave channel has its own real reference resistance R, and its power
//! waves are a = (V + R·I)/(2√R) and b = (V − R·I)/(2√R). With D = diag(√R)
//! that gives Z = D(I + S)(I − S)⁻¹D and Y = D⁻¹(I − S)(I + S)⁻¹D⁻¹. The hybrid
//! and chain forms of a two-port are read straight from S (Frickey, IEEE
//! MTT-42, 1994), so a series element, which has no impedance matrix, still
//! has its chain matrix.
//!
//! A form the network does not have is `None`, never a matrix of huge
//! numbers: a shunt element has no admittance matrix, a series element no
//! impedance matrix, and a two-port with no forward transmission no chain
//! matrix.

use num_complex::Complex64;

/// The parameter set a network matrix is read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NetworkForm {
    /// Scattering parameters: the retained power-wave matrix itself.
    #[default]
    S,
    /// Admittance parameters, I = Y·V.
    Y,
    /// Impedance parameters, V = Z·I.
    Z,
    /// Hybrid parameters of a two-port: (V1, I2) = H·(I1, V2).
    H,
    /// Chain parameters of a two-port: (V1, I1) = ABCD·(V2, −I2).
    Abcd,
}

impl NetworkForm {
    /// Every form, in the order a reader is offered them.
    pub const ALL: [Self; 5] = [Self::S, Self::Y, Self::Z, Self::H, Self::Abcd];

    /// The form's name as engineers write it.
    pub const fn label(self) -> &'static str {
        match self {
            Self::S => "S",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::H => "H",
            Self::Abcd => "ABCD",
        }
    }

    /// Whether a network of `channels` waves has this form: S, Y and Z have
    /// any number of ports, H and ABCD only two.
    pub const fn applies_to(self, channels: usize) -> bool {
        match self {
            Self::S | Self::Y | Self::Z => channels > 0,
            Self::H | Self::Abcd => channels == 2,
        }
    }

    /// The unit the coefficient at `(row, column)` reads in; empty for a
    /// ratio.
    pub const fn unit(self, row: usize, column: usize) -> &'static str {
        match (self, row, column) {
            (Self::S, ..) => "",
            (Self::Y, ..) => "S",
            (Self::Z, ..) => "Ω",
            (Self::H, 0, 0) | (Self::Abcd, 0, 1) => "Ω",
            (Self::H, 1, 1) | (Self::Abcd, 1, 0) => "S",
            (Self::H | Self::Abcd, ..) => "",
        }
    }

    /// The coefficient's name: `Z21`, `H12`, or `A` to `D` for the chain
    /// matrix. `port` names a channel by its index.
    pub fn coefficient(self, row: usize, column: usize, port: impl Fn(usize) -> String) -> String {
        match self {
            Self::Abcd => ["A", "B", "C", "D"][(row * 2 + column).min(3)].to_owned(),
            _ => {
                let (output, input) = (port(row), port(column));
                let separator = if output.len() > 1 || input.len() > 1 {
                    ","
                } else {
                    ""
                };
                format!("{}{output}{separator}{input}", self.label())
            }
        }
    }
}

/// The smallest pivot or denominator, relative to the matrix's scale, that a
/// form is still read through. Below it the form does not exist.
const SINGULAR: f64 = 1.0e-12;

/// `s`, an n×n scattering matrix in row-major order whose channel `i` is
/// referred to `references[i]` ohms, read in `form`. `None` when the network
/// has no such form, or the references are not finite positive resistances.
pub fn convert(s: &[Complex64], references: &[f64], form: NetworkForm) -> Option<Vec<Complex64>> {
    let n = references.len();
    if s.len() != n * n
        || !form.applies_to(n)
        || references.iter().any(|r| !r.is_finite() || *r <= 0.0)
    {
        return None;
    }
    match form {
        NetworkForm::S => Some(s.to_vec()),
        NetworkForm::Z => {
            // (I − S)·X = (I + S), then Z = D·X·D.
            let x = solve(&identity_minus(s, n), &identity_plus(s, n), n)?;
            Some(scaled(&x, references, |r| r.sqrt(), |r| r.sqrt()))
        }
        NetworkForm::Y => {
            // (I + S)·X = (I − S), then Y = D⁻¹·X·D⁻¹.
            let x = solve(&identity_plus(s, n), &identity_minus(s, n), n)?;
            Some(scaled(
                &x,
                references,
                |r| 1.0 / r.sqrt(),
                |r| 1.0 / r.sqrt(),
            ))
        }
        NetworkForm::H => hybrid(s, references),
        NetworkForm::Abcd => chain(s, references),
    }
}

/// The scattering matrix of the impedance matrix `z` (n×n, row-major) with
/// channel references `references`: S = (z + I)⁻¹(z − I), z = D⁻¹·Z·D⁻¹.
pub fn scattering_from_impedance(z: &[Complex64], references: &[f64]) -> Option<Vec<Complex64>> {
    let n = references.len();
    if z.len() != n * n || references.iter().any(|r| !r.is_finite() || *r <= 0.0) {
        return None;
    }
    let normalized = scaled(z, references, |r| 1.0 / r.sqrt(), |r| 1.0 / r.sqrt());
    let one = Complex64::new(1.0, 0.0);
    let shifted = |sign: f64| {
        let mut m = normalized.clone();
        for i in 0..n {
            m[i * n + i] += sign * one;
        }
        m
    };
    solve(&shifted(1.0), &shifted(-1.0), n)
}

/// Frickey's hybrid parameters from S on unit references, scaled to each
/// port's resistance: H11 = R1·h11, H12 = √(R1/R2)·h12, H21 = √(R1/R2)·h21,
/// H22 = h22/R2.
fn hybrid(s: &[Complex64], references: &[f64]) -> Option<Vec<Complex64>> {
    let (s11, s12, s21, s22) = (s[0], s[1], s[2], s[3]);
    let one = Complex64::new(1.0, 0.0);
    let denominator = (one - s11) * (one + s22) + s12 * s21;
    if denominator.norm() <= SINGULAR {
        return None;
    }
    let (r1, r2) = (references[0], references[1]);
    let ratio = (r1 / r2).sqrt();
    let h = [
        ((one + s11) * (one + s22) - s12 * s21) / denominator * r1,
        2.0 * s12 / denominator * ratio,
        -2.0 * s21 / denominator * ratio,
        ((one - s11) * (one - s22) - s12 * s21) / denominator / r2,
    ];
    finite(h.to_vec())
}

/// Frickey's chain parameters from S on unit references, scaled to each
/// port's resistance: A·√(R1/R2), B·√(R1R2), C/√(R1R2), D·√(R2/R1).
fn chain(s: &[Complex64], references: &[f64]) -> Option<Vec<Complex64>> {
    let (s11, s12, s21, s22) = (s[0], s[1], s[2], s[3]);
    let one = Complex64::new(1.0, 0.0);
    if s21.norm() <= SINGULAR {
        return None;
    }
    let twice = 2.0 * s21;
    let (r1, r2) = (references[0], references[1]);
    let abcd = [
        ((one + s11) * (one - s22) + s12 * s21) / twice * (r1 / r2).sqrt(),
        ((one + s11) * (one + s22) - s12 * s21) / twice * (r1 * r2).sqrt(),
        ((one - s11) * (one - s22) - s12 * s21) / twice / (r1 * r2).sqrt(),
        ((one - s11) * (one + s22) + s12 * s21) / twice * (r2 / r1).sqrt(),
    ];
    finite(abcd.to_vec())
}

fn finite(values: Vec<Complex64>) -> Option<Vec<Complex64>> {
    values
        .iter()
        .all(|value| value.re.is_finite() && value.im.is_finite())
        .then_some(values)
}

fn identity_plus(s: &[Complex64], n: usize) -> Vec<Complex64> {
    let mut m = s.to_vec();
    for i in 0..n {
        m[i * n + i] += 1.0;
    }
    m
}

fn identity_minus(s: &[Complex64], n: usize) -> Vec<Complex64> {
    let mut m = s.iter().map(|value| -value).collect::<Vec<_>>();
    for i in 0..n {
        m[i * n + i] += 1.0;
    }
    m
}

/// `m` with row `i` multiplied by `row(references[i])` and column `j` by
/// `column(references[j])`.
fn scaled(
    m: &[Complex64],
    references: &[f64],
    row: impl Fn(f64) -> f64,
    column: impl Fn(f64) -> f64,
) -> Vec<Complex64> {
    let n = references.len();
    (0..n * n)
        .map(|index| m[index] * row(references[index / n]) * column(references[index % n]))
        .collect()
}

/// X with A·X = B, both n×n row-major, by elimination with partial
/// pivoting. `None` when A is singular to within [`SINGULAR`] of its scale.
fn solve(a: &[Complex64], b: &[Complex64], n: usize) -> Option<Vec<Complex64>> {
    let scale = a.iter().map(|value| value.norm()).fold(0.0, f64::max);
    if !scale.is_finite() || scale == 0.0 {
        return None;
    }
    let mut a = a.to_vec();
    let mut x = b.to_vec();
    for column in 0..n {
        let pivot = (column..n).max_by(|&left, &right| {
            a[left * n + column]
                .norm()
                .total_cmp(&a[right * n + column].norm())
        })?;
        if a[pivot * n + column].norm() <= SINGULAR * scale {
            return None;
        }
        if pivot != column {
            for k in 0..n {
                a.swap(pivot * n + k, column * n + k);
                x.swap(pivot * n + k, column * n + k);
            }
        }
        let diagonal = a[column * n + column];
        for row in column + 1..n {
            let factor = a[row * n + column] / diagonal;
            if factor == Complex64::new(0.0, 0.0) {
                continue;
            }
            for k in column..n {
                let value = a[column * n + k];
                a[row * n + k] -= factor * value;
            }
            for k in 0..n {
                let value = x[column * n + k];
                x[row * n + k] -= factor * value;
            }
        }
    }
    for column in (0..n).rev() {
        let diagonal = a[column * n + column];
        for k in 0..n {
            let mut value = x[column * n + k];
            for j in column + 1..n {
                value -= a[column * n + j] * x[j * n + k];
            }
            x[column * n + k] = value / diagonal;
        }
    }
    finite(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(re: f64, im: f64) -> Complex64 {
        Complex64::new(re, im)
    }

    fn close(left: &[Complex64], right: &[Complex64], tolerance: f64) {
        assert_eq!(left.len(), right.len());
        for (index, (l, r)) in left.iter().zip(right).enumerate() {
            assert!(
                (l - r).norm() <= tolerance * (1.0 + r.norm()),
                "cell {index}: {l} against {r}"
            );
        }
    }

    /// A series impedance between two ports referred to 50 Ω:
    /// S11 = S22 = Z/(Z + 2R), S21 = S12 = 2R/(Z + 2R).
    fn series(z: Complex64) -> Vec<Complex64> {
        let r = 50.0;
        let (reflection, transmission) = (z / (z + 2.0 * r), 2.0 * r / (z + 2.0 * r));
        vec![reflection, transmission, transmission, reflection]
    }

    /// A shunt impedance across two ports referred to 50 Ω:
    /// S11 = S22 = −R/(2Z + R), S21 = S12 = 2Z/(2Z + R).
    fn shunt(z: Complex64) -> Vec<Complex64> {
        let r = 50.0;
        let (reflection, transmission) = (-r / (2.0 * z + r), 2.0 * z / (2.0 * z + r));
        vec![reflection, transmission, transmission, reflection]
    }

    #[test]
    fn a_series_element_has_a_chain_and_admittance_matrix_but_no_impedance_matrix() {
        let z = c(100.0, 25.0);
        let s = series(z);
        let references = [50.0, 50.0];
        assert_eq!(convert(&s, &references, NetworkForm::Z), None);
        let y = 1.0 / z;
        close(
            &convert(&s, &references, NetworkForm::Y).unwrap(),
            &[y, -y, -y, y],
            1e-12,
        );
        close(
            &convert(&s, &references, NetworkForm::Abcd).unwrap(),
            &[c(1.0, 0.0), z, c(0.0, 0.0), c(1.0, 0.0)],
            1e-12,
        );
        // Port 2 shorted, the input sees the element; open at port 1, port 2
        // sees nothing.
        close(
            &convert(&s, &references, NetworkForm::H).unwrap(),
            &[z, c(1.0, 0.0), c(-1.0, 0.0), c(0.0, 0.0)],
            1e-12,
        );
    }

    #[test]
    fn a_shunt_element_has_an_impedance_matrix_but_no_admittance_matrix() {
        let z = c(25.0, -10.0);
        let s = shunt(z);
        let references = [50.0, 50.0];
        assert_eq!(convert(&s, &references, NetworkForm::Y), None);
        close(
            &convert(&s, &references, NetworkForm::Z).unwrap(),
            &[z, z, z, z],
            1e-12,
        );
        close(
            &convert(&s, &references, NetworkForm::Abcd).unwrap(),
            &[c(1.0, 0.0), c(0.0, 0.0), 1.0 / z, c(1.0, 0.0)],
            1e-12,
        );
        close(
            &convert(&s, &references, NetworkForm::H).unwrap(),
            &[c(0.0, 0.0), c(1.0, 0.0), c(-1.0, 0.0), 1.0 / z],
            1e-12,
        );
    }

    #[test]
    fn a_tee_on_unequal_references_reads_its_impedances_back_from_s() {
        // A tee of Za, Zb in the arms and Zc in the leg: Z = [[Za+Zc, Zc],
        // [Zc, Zb+Zc]], between a 50 Ω and a 75 Ω port.
        let (za, zb, zc) = (c(10.0, 5.0), c(20.0, -15.0), c(40.0, 30.0));
        let z = vec![za + zc, zc, zc, zb + zc];
        let references = [50.0, 75.0];
        let s = scattering_from_impedance(&z, &references).unwrap();
        close(
            &convert(&s, &references, NetworkForm::Z).unwrap(),
            &z,
            1e-12,
        );
        // Y is the inverse of Z; the chain and hybrid forms follow from Z by
        // their textbook definitions.
        let determinant = z[0] * z[3] - z[1] * z[2];
        close(
            &convert(&s, &references, NetworkForm::Y).unwrap(),
            &[
                z[3] / determinant,
                -z[1] / determinant,
                -z[2] / determinant,
                z[0] / determinant,
            ],
            1e-12,
        );
        close(
            &convert(&s, &references, NetworkForm::Abcd).unwrap(),
            &[z[0] / z[2], determinant / z[2], 1.0 / z[2], z[3] / z[2]],
            1e-12,
        );
        close(
            &convert(&s, &references, NetworkForm::H).unwrap(),
            &[determinant / z[3], z[1] / z[3], -z[2] / z[3], 1.0 / z[3]],
            1e-12,
        );
    }

    #[test]
    fn a_three_port_round_trips_from_s_to_z_and_back() {
        let s = vec![
            c(0.2, -0.1),
            c(0.3, 0.05),
            c(-0.1, 0.2),
            c(0.3, 0.05),
            c(-0.25, 0.1),
            c(0.15, -0.05),
            c(-0.1, 0.2),
            c(0.15, -0.05),
            c(0.05, 0.3),
        ];
        let references = [50.0, 25.0, 100.0];
        let z = convert(&s, &references, NetworkForm::Z).unwrap();
        close(
            &scattering_from_impedance(&z, &references).unwrap(),
            &s,
            1e-12,
        );
        let y = convert(&s, &references, NetworkForm::Y).unwrap();
        // Y·Z is the identity.
        let product = (0..9)
            .map(|index| {
                let (row, column) = (index / 3, index % 3);
                (0..3).map(|k| y[row * 3 + k] * z[k * 3 + column]).sum()
            })
            .collect::<Vec<Complex64>>();
        close(
            &product,
            &[
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(0.0, 0.0),
                c(1.0, 0.0),
            ],
            1e-12,
        );
    }

    #[test]
    fn hybrid_and_chain_forms_are_two_port_only_and_references_must_be_resistances() {
        let s = vec![c(0.1, 0.0); 9];
        assert_eq!(convert(&s, &[50.0; 3], NetworkForm::H), None);
        assert_eq!(convert(&s, &[50.0; 3], NetworkForm::Abcd), None);
        assert!(convert(&s, &[50.0; 3], NetworkForm::Z).is_some());
        assert_eq!(
            convert(&series(c(50.0, 0.0)), &[50.0, 0.0], NetworkForm::Z),
            None
        );
        assert_eq!(
            convert(
                &[c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)],
                &[50.0, 50.0],
                NetworkForm::Abcd
            ),
            None,
            "a two-port with no forward transmission has no chain matrix"
        );
    }

    #[test]
    fn coefficients_are_named_by_form_and_port() {
        let port = |index: usize| (index + 1).to_string();
        assert_eq!(NetworkForm::Z.coefficient(1, 0, port), "Z21");
        assert_eq!(NetworkForm::Abcd.coefficient(0, 1, port), "B");
        assert_eq!(
            NetworkForm::Y.coefficient(10, 1, |index| (index + 1).to_string()),
            "Y11,2"
        );
        assert_eq!(NetworkForm::H.unit(0, 0), "Ω");
        assert_eq!(NetworkForm::H.unit(1, 1), "S");
        assert_eq!(NetworkForm::Abcd.unit(0, 1), "Ω");
        assert_eq!(NetworkForm::Abcd.unit(1, 0), "S");
        assert_eq!(NetworkForm::Abcd.unit(0, 0), "");
    }
}
