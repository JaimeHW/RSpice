//! Shared closed-form device physics for oracle computation.
//!
//! Constants are the ngspice-dialect definitions the adapter pins
//! (CODATA k and q, 27°C nominal = 300.15 K): SPICE model equations are
//! *defined* in terms of these values, so using them is part of the model
//! specification, not an imitation of the engine.

pub const K_BOLTZMANN: f64 = 1.380649e-23;
pub const Q_ELECTRON: f64 = 1.602176634e-19;
pub const KELVIN_AT_ZERO_CELSIUS: f64 = 273.15;
pub const NOMINAL_CELSIUS: f64 = 27.0;

pub fn thermal_voltage(temperature_celsius: f64) -> f64 {
    (temperature_celsius + KELVIN_AT_ZERO_CELSIUS) * K_BOLTZMANN / Q_ELECTRON
}

/// SPICE junction saturation-current temperature scaling with the level-1
/// diode defaults `eg = 1.11`, `xti = 3`:
/// `is(T) = is * (T/Tnom)^(xti/n) * exp(eg/(n*vt) * (T/Tnom - 1))`.
pub fn diode_saturation_current_at(is_nominal: f64, n: f64, temperature_celsius: f64) -> f64 {
    const ENERGY_GAP: f64 = 1.11;
    const XTI: f64 = 3.0;
    let t = temperature_celsius + KELVIN_AT_ZERO_CELSIUS;
    let t_nominal = NOMINAL_CELSIUS + KELVIN_AT_ZERO_CELSIUS;
    let ratio = t / t_nominal;
    let vt = thermal_voltage(temperature_celsius);
    is_nominal * ratio.powf(XTI / n) * (ENERGY_GAP / (n * vt) * (ratio - 1.0)).exp()
}

/// Node voltage of a source-fed diode with series resistance: the root of
/// `(vs - v) / r = is * (exp(v / (n*vt)) - 1)`, bracketed and bisected to
/// the last representable bit. Independent mathematics — the analytic node
/// equation solved directly, not the engine's companion-model iteration.
pub fn diode_node_voltage(vs: f64, r: f64, is: f64, n: f64, temperature_celsius: f64) -> f64 {
    let n_vt = n * thermal_voltage(temperature_celsius);
    let residual = |v: f64| (vs - v) / r - is * ((v / n_vt).exp() - 1.0);
    let (mut low, mut high) = (0.0_f64, vs);
    for _ in 0..200 {
        let middle = 0.5 * (low + high);
        if middle <= low || middle >= high {
            break;
        }
        if residual(middle) > 0.0 {
            low = middle;
        } else {
            high = middle;
        }
    }
    0.5 * (low + high)
}

/// First-order temperature-coefficient resistance:
/// `r(T) = r * (1 + tc1*dT + tc2*dT^2)` around the 27°C nominal.
pub fn resistance_at(r_nominal: f64, tc1: f64, tc2: f64, temperature_celsius: f64) -> f64 {
    let delta = temperature_celsius - NOMINAL_CELSIUS;
    r_nominal * (1.0 + tc1 * delta + tc2 * delta * delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dialect_thermal_voltage_is_the_codata_value() {
        let vt = thermal_voltage(NOMINAL_CELSIUS);
        assert!((vt - 0.02586493).abs() < 1e-8, "vt(27C) = {vt}");
    }

    #[test]
    fn diode_bisection_solves_the_node_equation_to_machine_precision() {
        let v = diode_node_voltage(5.0, 1.0e3, 1.0e-14, 1.0, NOMINAL_CELSIUS);
        let n_vt = thermal_voltage(NOMINAL_CELSIUS);
        let residual = (5.0 - v) / 1.0e3 - 1.0e-14 * ((v / n_vt).exp() - 1.0);
        assert!(residual.abs() < 1e-12, "residual {residual}");
        assert!((0.5..0.8).contains(&v), "forward drop {v}");
    }

    #[test]
    fn saturation_current_scaling_is_monotone_and_nominal_at_nominal() {
        assert_eq!(
            diode_saturation_current_at(1e-14, 1.0, NOMINAL_CELSIUS),
            1e-14
        );
        assert!(diode_saturation_current_at(1e-14, 1.0, 85.0) > 1e-12);
        assert!(diode_saturation_current_at(1e-14, 1.0, -40.0) < 1e-16);
    }

    #[test]
    fn resistance_scaling_matches_the_polynomial() {
        assert_eq!(resistance_at(1000.0, 0.0, 0.0, 85.0), 1000.0);
        let r = resistance_at(1000.0, 2e-3, 1e-5, 85.0);
        assert_eq!(r, 1000.0 * (1.0 + 2e-3 * 58.0 + 1e-5 * 58.0 * 58.0));
    }
}
