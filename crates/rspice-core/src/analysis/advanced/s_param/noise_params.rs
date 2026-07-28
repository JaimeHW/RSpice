//! Standard two-port noise parameters from an admittance matrix and its
//! Norton current-noise correlation matrix.
//!
//! These are the Maas/ngspice `.SP donoise` equations, normalized by `4*k*T`
//! at the actual circuit temperature. The quantities produced are the ones an
//! RF designer expects on a datasheet:
//!
//! - `Rn`   — equivalent noise resistance
//! - `F`    — noise factor at the port's own reference impedance
//! - `Fmin` — minimum achievable noise factor over source admittance
//! - `Sopt` — source reflection coefficient that achieves `Fmin`
//!
//! Every guard here fails to [`TwoPortNoise::undefined`] rather than to a
//! plausible-looking number. A noise figure that is quietly wrong is worse
//! than one that is visibly absent, because it will be believed.

use crate::Complex64;
use crate::Value;

/// Derived two-port noise parameters at one frequency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoPortNoise {
    /// Equivalent noise resistance, ohms.
    pub noise_resistance: Value,
    /// Noise factor at the input reference impedance (linear, not dB).
    pub noise_factor: Value,
    /// Minimum noise factor over all source admittances (linear, not dB).
    pub minimum_noise_factor: Value,
    /// Source reflection coefficient achieving `minimum_noise_factor`.
    pub optimum_source_reflection: Complex64,
    /// False when the inputs did not support a physical solution; every other
    /// field is NaN in that case.
    pub valid: bool,
}

impl TwoPortNoise {
    /// The all-NaN, `valid = false` result.
    pub fn undefined() -> Self {
        Self {
            noise_resistance: Value::NAN,
            noise_factor: Value::NAN,
            minimum_noise_factor: Value::NAN,
            optimum_source_reflection: Complex64::new(Value::NAN, Value::NAN),
            valid: false,
        }
    }
}

/// Derive the standard two-port noise parameters from `Y` and its Norton
/// current-noise covariance `Cy`.
///
/// `input_reference_impedance` is port 1's reference impedance and
/// `temperature` the circuit temperature in kelvin. Both matrices are 2x2,
/// indexed `[row][column]`.
pub fn derive_two_port_noise(
    admittance: &[Vec<Complex64>],
    cy: &[Vec<Complex64>],
    input_reference_impedance: Value,
    temperature: Value,
) -> TwoPortNoise {
    if admittance.len() != 2
        || cy.len() != 2
        || admittance.iter().any(|row| row.len() != 2)
        || cy.iter().any(|row| row.len() != 2)
        || !input_reference_impedance.is_finite()
        || input_reference_impedance <= 0.0
        || !temperature.is_finite()
        || temperature <= 0.0
    {
        return TwoPortNoise::undefined();
    }

    let finite_complex = |value: Complex64| value.re.is_finite() && value.im.is_finite();
    if admittance
        .iter()
        .flatten()
        .any(|value| !finite_complex(*value))
        || cy.iter().flatten().any(|value| !finite_complex(*value))
    {
        return TwoPortNoise::undefined();
    }

    let y21_power = admittance[1][0].norm_sqr();
    let y_scale = admittance
        .iter()
        .flatten()
        .map(|value| value.norm())
        .fold(0.0_f64, f64::max);
    let transmission_floor = (y_scale * f64::EPSILON * 64.0).powi(2);
    if !y21_power.is_finite() || y21_power <= transmission_floor.max(f64::MIN_POSITIVE) {
        return TwoPortNoise::undefined();
    }

    let knorm = 4.0 * crate::analysis::noise::K_BOLTZMANN * temperature;
    let c11 = cy[0][0] / knorm;
    let c12 = cy[0][1] / knorm;
    let c22 = cy[1][1] / knorm;
    let covariance_scale = c11.norm().max(c12.norm()).max(c22.norm());

    // A noiseless two-port has F=Fmin=1 and Rn=0. Sopt is non-unique;
    // zero is the deterministic matched-source convention used here.
    if covariance_scale <= f64::MIN_POSITIVE {
        return TwoPortNoise {
            noise_resistance: 0.0,
            noise_factor: 1.0,
            minimum_noise_factor: 1.0,
            optimum_source_reflection: Complex64::new(0.0, 0.0),
            valid: true,
        };
    }

    let covariance_tolerance = covariance_scale * f64::EPSILON * 256.0;
    if c11.re < -covariance_tolerance
        || c22.re <= covariance_tolerance
        || c11.im.abs() > covariance_tolerance
        || c22.im.abs() > covariance_tolerance
    {
        return TwoPortNoise::undefined();
    }

    let noise_resistance = c22.re.max(0.0) / y21_power;
    if !noise_resistance.is_finite() || noise_resistance <= 0.0 {
        return TwoPortNoise::undefined();
    }
    let ycor = admittance[0][0] - (c12 / c22.re) * admittance[1][0];
    let gu = c11.re - noise_resistance * (admittance[0][0] - ycor).norm_sqr();
    let raw_radicand = ycor.re * ycor.re + gu / noise_resistance;
    let radicand_scale = ycor.norm_sqr().max((gu / noise_resistance).abs());
    let radicand_tolerance = radicand_scale * f64::EPSILON * 1024.0;
    if !raw_radicand.is_finite() || raw_radicand < -radicand_tolerance {
        return TwoPortNoise::undefined();
    }
    let ysopt = Complex64::new(raw_radicand.max(0.0).sqrt(), -ycor.im);
    let y0 = Complex64::new(1.0 / input_reference_impedance, 0.0);
    let reflection_denominator = y0 + ysopt;
    if reflection_denominator.norm_sqr() <= f64::MIN_POSITIVE {
        return TwoPortNoise::undefined();
    }
    let optimum_source_reflection = (y0 - ysopt) / reflection_denominator;
    let mut minimum_noise_factor = 1.0 + 2.0 * noise_resistance * (ycor.re + ysopt.re);
    let mut noise_factor =
        minimum_noise_factor + (noise_resistance / y0.re) * (y0 - ysopt).norm_sqr();

    let factor_tolerance = 4096.0 * f64::EPSILON;
    if minimum_noise_factor >= 1.0 - factor_tolerance && minimum_noise_factor < 1.0 {
        minimum_noise_factor = 1.0;
    }
    if noise_factor >= 1.0 - factor_tolerance && noise_factor < 1.0 {
        noise_factor = 1.0;
    }
    if !minimum_noise_factor.is_finite()
        || !noise_factor.is_finite()
        || minimum_noise_factor < 1.0
        || noise_factor < minimum_noise_factor - factor_tolerance
        || !finite_complex(optimum_source_reflection)
    {
        return TwoPortNoise::undefined();
    }

    TwoPortNoise {
        noise_resistance,
        noise_factor,
        minimum_noise_factor,
        optimum_source_reflection,
        valid: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A series resistor R between two ports: `Y = [[1,-1],[-1,1]]/R`, and the
    /// Norton noise covariance of a single conductance `G` is `4kTG` on the
    /// diagonal with `-4kTG` off-diagonal, since the same current noise leaves
    /// one port and enters the other.
    fn series_resistor(r: Value, temperature: Value) -> (Vec<Vec<Complex64>>, Vec<Vec<Complex64>>) {
        let g = 1.0 / r;
        let y = vec![
            vec![Complex64::new(g, 0.0), Complex64::new(-g, 0.0)],
            vec![Complex64::new(-g, 0.0), Complex64::new(g, 0.0)],
        ];
        let power = 4.0 * crate::analysis::noise::K_BOLTZMANN * temperature * g;
        let cy = vec![
            vec![Complex64::new(power, 0.0), Complex64::new(-power, 0.0)],
            vec![Complex64::new(-power, 0.0), Complex64::new(power, 0.0)],
        ];
        (y, cy)
    }

    #[test]
    fn series_resistor_has_unit_rn_ratio_and_3db_noise_figure() {
        let (r, temperature) = (50.0, 300.15);
        let (y, cy) = series_resistor(r, temperature);

        let noise = derive_two_port_noise(&y, &cy, r, temperature);

        assert!(noise.valid);
        // Rn of a series resistor equals the resistance itself.
        assert!((noise.noise_resistance - r).abs() < r * 1e-11);
        // Source and device contribute equally: F = 2, i.e. NF = 3.01 dB.
        assert!((noise.noise_factor - 2.0).abs() < 1e-11);
        // A passive reciprocal two-port is already matched for minimum noise.
        assert!((noise.minimum_noise_factor - 1.0).abs() < 1e-11);
        assert!((noise.optimum_source_reflection - Complex64::new(1.0, 0.0)).norm() < 1e-11);
    }

    #[test]
    fn noiseless_two_port_reports_unity_factors() {
        let y = vec![
            vec![Complex64::new(0.02, 0.0), Complex64::new(-0.02, 0.0)],
            vec![Complex64::new(-0.02, 0.0), Complex64::new(0.02, 0.0)],
        ];
        let zero = vec![vec![Complex64::new(0.0, 0.0); 2]; 2];

        let noise = derive_two_port_noise(&y, &zero, 50.0, 300.15);

        assert!(noise.valid);
        assert_eq!(noise.noise_resistance, 0.0);
        assert_eq!(noise.noise_factor, 1.0);
        assert_eq!(noise.minimum_noise_factor, 1.0);
        assert_eq!(noise.optimum_source_reflection, Complex64::new(0.0, 0.0));
    }

    #[test]
    fn degenerate_inputs_are_undefined_rather_than_guessed() {
        let (y, cy) = series_resistor(50.0, 300.15);

        // No forward transmission: Rn is not derivable.
        let isolated = vec![
            vec![Complex64::new(0.02, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(0.0, 0.0), Complex64::new(0.02, 0.0)],
        ];
        assert!(!derive_two_port_noise(&isolated, &cy, 50.0, 300.15).valid);

        // Non-physical temperature and reference impedance.
        assert!(!derive_two_port_noise(&y, &cy, 50.0, 0.0).valid);
        assert!(!derive_two_port_noise(&y, &cy, 0.0, 300.15).valid);
        assert!(!derive_two_port_noise(&y, &cy, -50.0, 300.15).valid);

        // Wrong shape.
        let one_port = vec![vec![Complex64::new(0.02, 0.0)]];
        assert!(!derive_two_port_noise(&one_port, &cy, 50.0, 300.15).valid);

        // Non-finite entries.
        let nan = vec![vec![Complex64::new(f64::NAN, 0.0); 2]; 2];
        assert!(!derive_two_port_noise(&nan, &cy, 50.0, 300.15).valid);
        assert!(!derive_two_port_noise(&y, &nan, 50.0, 300.15).valid);
    }

    #[test]
    fn undefined_is_all_nan_and_not_equal_to_itself_by_value() {
        let undefined = TwoPortNoise::undefined();
        assert!(!undefined.valid);
        assert!(undefined.noise_resistance.is_nan());
        assert!(undefined.noise_factor.is_nan());
        assert!(undefined.minimum_noise_factor.is_nan());
        assert!(undefined.optimum_source_reflection.re.is_nan());
    }
}
