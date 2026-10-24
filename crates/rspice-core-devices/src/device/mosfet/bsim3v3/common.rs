//! Numeric constants shared by the BSIM3v3.3 port.
//!
//! Values are transcribed from ngspice-46 `b3ld.c`/`b3temp.c`/`b3set.c`
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
/// Oxide permittivity used by the Berkeley source (F/m).
pub const EPSOX: Value = 3.453133e-11;
/// Silicon permittivity used by the Berkeley source (F/m).
pub const EPSSI: Value = 1.03594e-10;
/// Electron charge used by the Berkeley source (C).
pub const CHARGE_Q: Value = 1.60219e-19;
/// Boltzmann constant over charge, `Kb / q` (V/K) — ngspice `KboQ`.
pub const KB_OVER_Q: Value = 8.617087e-5;
/// Pi as written in the Berkeley source (`#define PI 3.141592654`).
#[allow(clippy::approx_constant)]
pub const PI: Value = 3.141592654;

// Smoothing deltas from b3ld.c (DELTA_2 exists in the C but is unused).
pub const DELTA_1: Value = 0.02;
pub const DELTA_3: Value = 0.02;
pub const DELTA_4: Value = 0.02;

/// `CONSTvt0 = CONSTboltz * (27 C in K) / CHARGE` (ngspice const.h/main.c);
/// the thermal voltage at 300.15 K used by `DEVpnjlim` and `BSIM3vcrit`.
/// Note these are the modern CODATA-ish constants of const.h, not the
/// truncated Berkeley ones above.
pub const CONST_VT0: Value = 1.38064852e-23 * 300.15 / 1.6021766208e-19;
/// `CONSTroot2` (ngspice `CONSTsqrt2`).
#[allow(clippy::approx_constant)]
pub const CONST_ROOT2: Value = 1.4142135623730950488016887242097;
