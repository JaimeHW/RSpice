//! C2 — charge-pump PLL (design §3 C2).
//!
//! The VCO is ngspice's `d_osc` PWL table (control → frequency), the divider
//! is 40, the reference 10 MHz; the loop filter is `loopf` (R1 300, R2 200,
//! C1 10 nF, C2 5 nF, R3 50, C3 0.5 nF).

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("vlock", vlock),
    ("fdiv", fdiv),
    ("zlf", zlf),
    ("pnoise_lf", pnoise_lf),
];

const CONTROL: [f64; 5] = [0.5, 1.0, 1.5, 2.0, 2.5];
const FREQUENCY: [f64; 5] = [879.0820e6, 747.2197e6, 579.9500e6, 377.2727e6, 161.1650e6];
const DIVISOR: f64 = 40.0;
const REFERENCE: f64 = 10e6;
const R2: f64 = 200.0;
const R3: f64 = 50.0;
const C1: f64 = 10e-9;
const C2: f64 = 5e-9;
const C3: f64 = 0.5e-9;

/// The control voltage at which the PWL table gives `frequency`.
fn control_for(frequency: f64) -> Option<f64> {
    CONTROL
        .windows(2)
        .zip(FREQUENCY.windows(2))
        .find(|(_, f)| (f[0] - frequency) * (f[1] - frequency) <= 0.0)
        .map(|(v, f)| v[0] + (v[1] - v[0]) * (f[0] - frequency) / (f[0] - f[1]))
}

/// `V_lock = 1.5 + 0.5·(579.95 − 400)/(579.95 − 377.2727) = 1.943932 V`
/// (the design writes 1.94394).
fn vlock(_: &Context<'_>) -> Result<Expected, String> {
    let lock = control_for(DIVISOR * REFERENCE).ok_or("400 MHz is outside the VCO table")?;
    Ok(values(vec![lock]))
}

/// Locked divider period: one reference period.
fn fdiv(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![1.0 / REFERENCE]))
}

/// The loop-filter impedance seen by the pump, at the deck's injection.
///
/// `Z(jω) = Z1/(1 + jωR3C3)`, `Z1 = [jωC2 + 1/(R2 + 1/jωC1) + 1/(R3 + 1/jωC3)]⁻¹`,
/// valid for `mx`, whose switches are both off at the D-A state. The deck's
/// `iinj xlf.dra 0 ac 1` draws its unit current *out of* `dra` (SPICE: current
/// flows from n+ through the source to n−), so `v(cont) = −Z(jω)`.
fn zlf(context: &Context<'_>) -> Result<Expected, String> {
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| -impedance(frequency))
        .collect();
    Ok(complex(expected))
}

fn impedance(frequency: f64) -> Complex64 {
    let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
    let one = Complex64::new(1.0, 0.0);
    let admittance = s * C2 + one / (one * R2 + one / (s * C1)) + one / (one * R3 + one / (s * C3));
    (one / admittance) / (one + s * R3 * C3)
}

/// L(f) needs the VCO's frequency-noise source (`white_noise(sf, "fm")`),
/// which the design adds at P7, and the pump current measured from the tran
/// run; before P7 there is nothing to evaluate.
fn pnoise_lf(_: &Context<'_>) -> Result<Expected, String> {
    Err("C2 pnoise L(f) is defined at the P7 exit (VCO noise source and measured Icp); not evaluable before P7".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_voltage_and_loop_filter_limits() {
        // 1.5 + 0.5·179.95/202.6773 = 1.943932 (the design writes 1.94394).
        let lock = control_for(400e6).unwrap();
        assert!((lock - 1.943932).abs() < 1e-6, "{lock}");
        // Low frequency: C1 + C2 + C3 dominate; high frequency: C2 ∥ C3 path.
        let low = impedance(1.0);
        let expected = 1.0 / (2.0 * std::f64::consts::PI * (C1 + C2 + C3));
        assert!((low.norm() - expected).abs() / expected < 1e-3, "{low}");
        assert!(impedance(1e9).norm() < 1.0);
    }
}
