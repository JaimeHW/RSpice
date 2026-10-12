//! C4 — 8-bit SAR ADC (design §3 C4).
//!
//! The staircase converts the mid-code inputs `(S_k + 0.5)/256 V` (Vref 1 V),
//! so each code is exactly `S_k`. The comparator is `gm = 1 mS`,
//! `ro = 100 kΩ`, `co = 50 fF` (gain 100, pole 31.83 MHz); at the held
//! D-A state the top plate is grounded through the top switch's 50 Ω.

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[("codes", codes), ("acc", acc), ("nz", nz)];

/// The staircase's codes, in conversion order (`net/vin_codes.cir`).
pub const STAIRCASE: [u8; 32] = [
    0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 127, 128, 129, 144, 170, 191, 200, 233, 250, 254, 255,
    64, 63, 32, 31, 16, 15, 7, 6, 4, 192,
];

const GM: f64 = 1e-3;
const RO: f64 = 100e3;
const CO: f64 = 50e-15;
const RON_TOP: f64 = 50.0;
/// Boltzmann's constant (exact, SI 2019) and the corpus temperature.
const BOLTZMANN: f64 = 1.380_649e-23;
const TEMPERATURE: f64 = 300.15;

fn pole() -> f64 {
    1.0 / (2.0 * std::f64::consts::PI * RO * CO)
}

/// Every mid-code input converts to its own code.
fn codes(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(
        STAIRCASE.iter().map(|code| f64::from(*code)).collect(),
    ))
}

/// `v(xcmp.o1)/v(cref) = gm·ro/(1 + jf/fp)`: cref drives inn; the top plate
/// (inp) is grounded and draws nothing from the comparator.
fn acc(context: &Context<'_>) -> Result<Expected, String> {
    let gain = GM * RO;
    Ok(complex(
        points(context.quantity)?
            .into_iter()
            .map(|frequency| Complex64::new(gain, 0.0) / Complex64::new(1.0, frequency / pole()))
            .collect(),
    ))
}

/// `S(f) = 4kT·(ro + (gm·ro)²·Ron_top)/(1 + (f/fp)²)`: ro's own noise and the
/// top switch's thermal noise amplified by the comparator, both through its
/// pole.
fn nz(context: &Context<'_>) -> Result<Expected, String> {
    let four_kt = 4.0 * BOLTZMANN * TEMPERATURE;
    let white = four_kt * (RO + (GM * RO).powi(2) * RON_TOP);
    Ok(values(
        points(context.quantity)?
            .into_iter()
            .map(|frequency| white / (1.0 + (frequency / pole()).powi(2)))
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparator_constants_match_the_design() {
        assert!((pole() - 31.83e6).abs() < 0.01e6, "{}", pole());
        let four_kt = 4.0 * BOLTZMANN * TEMPERATURE;
        let white = four_kt * (RO + (GM * RO).powi(2) * RON_TOP);
        assert!((white - 9.944e-15).abs() < 0.002e-15, "{white}");
        // Every staircase code is distinct from its neighbours and in range.
        assert!(STAIRCASE.windows(2).all(|pair| pair[0] != pair[1]));
        assert_eq!(STAIRCASE.len(), 32);
    }
}
