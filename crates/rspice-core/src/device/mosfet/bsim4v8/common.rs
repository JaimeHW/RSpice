//! Numeric constants shared by the BSIM4 v4.8 port.
//!
//! Values are transcribed from ngspice-46 `b4ld.c`/`b4temp.c`/`b4set.c`
//! `#define`s and `src/include/ngspice/const.h`. The Berkeley sources use
//! their own truncated physical constants; matching them bit-for-bit is what
//! keeps this port pinned to the reference output, so do not "fix" them.

use crate::Value;

/// `exp(34.0)` guard value (ngspice `MAX_EXP`).
pub const MAX_EXP: Value = 5.834617425e14;
/// `exp(-34.0)` guard value (ngspice `MIN_EXP`).
pub const MIN_EXP: Value = 1.713908431e-15;
/// Exponent magnitude threshold (ngspice `EXP_THRESHOLD`).
pub const EXP_THRESHOLD: Value = 34.0;
/// `exp(100.0)` guard value for the v4.7 GIDL/GISL model (`MAX_EXPL`).
pub const MAX_EXPL: Value = 2.688117142e+43;
/// `exp(-100.0)` guard value (`MIN_EXPL`).
pub const MIN_EXPL: Value = 3.720075976e-44;
/// Long-range exponent threshold (`EXPL_THRESHOLD`).
pub const EXPL_THRESHOLD: Value = 100.0;
/// Vacuum permittivity as written in the BSIM4 sources (F/m).
pub const EPS0: Value = 8.85418e-12;
/// Silicon permittivity used by the Berkeley source (F/m).
pub const EPSSI: Value = 1.03594e-10;
/// Electron charge used by the Berkeley source (C) — b4ld.c `Charge_q`.
pub const CHARGE_Q: Value = 1.60219e-19;
/// Modern electron charge of ngspice const.h (`CHARGE`); only
/// `BSIM4polyDepletion` reads this one, not `Charge_q`.
pub const CONST_CHARGE: Value = 1.6021766208e-19;
/// Boltzmann constant over charge, `Kb / q` (V/K) — b4temp.c `KboQ`.
pub const KB_OVER_Q: Value = 8.617087e-5;
/// Pi as written in the Berkeley source (`#define PI 3.141592654`).
#[allow(clippy::approx_constant)]
pub const PI: Value = 3.141592654;

// Smoothing deltas from b4ld.c (DELTA_2 exists in the C but is unused).
pub const DELTA_1: Value = 0.02;
pub const DELTA_3: Value = 0.02;
pub const DELTA_4: Value = 0.02;
/// Source-end velocity-limit smoothing order (b4ld.c `MM`).
pub const MM: Value = 3.0;

/// `CONSTvt0 = CONSTboltz * (27 C in K) / CHARGE` (ngspice const.h/main.c);
/// the thermal voltage at 300.15 K used by `DEVpnjlim` and `BSIM4vcrit`.
/// Note these are the modern CODATA-ish constants of const.h, not the
/// truncated Berkeley ones above.
pub const CONST_VT0: Value = 1.38064852e-23 * 300.15 / 1.6021766208e-19;
/// `CONSTroot2` (ngspice `CONSTsqrt2`).
#[allow(clippy::approx_constant)]
pub const CONST_ROOT2: Value = 1.4142135623730950488016887242097;

/// b4ld.c `DEXP(A, B, C)`: clamped exponential with derivative.
/// Returns `(B, C)` where `B ~ exp(a)` and `C = dB/da`.
#[inline]
pub fn dexp(a: Value) -> (Value, Value) {
    if a > EXP_THRESHOLD {
        (MAX_EXP * (1.0 + a - EXP_THRESHOLD), MAX_EXP)
    } else if a < -EXP_THRESHOLD {
        (MIN_EXP, 0.0)
    } else {
        let b = a.exp();
        (b, b)
    }
}

/// b4temp.c `DEXP(A, B)`: clamped exponential without derivative (the value
/// clamp is identical to the load-side macro above).
#[inline]
pub fn dexp_temp(a: Value) -> Value {
    dexp(a).0
}
