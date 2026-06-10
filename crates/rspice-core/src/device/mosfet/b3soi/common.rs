//! Constants and helpers shared by the BSIM3SOI family (FD/DD/PD).
//!
//! Values are transcribed from ngspice-46 `b3soiddld.c` / `b3soiddset.c` /
//! `b3soiddtemp.c` (the FD and PD sources use the same numeric constants).

use crate::Value;

/// `exp(34.0)` guard value (ngspice `MAX_EXP`).
pub const MAX_EXP: Value = 5.834617425e14;
/// `exp(-34.0)` guard value (ngspice `MIN_EXP`).
pub const MIN_EXP: Value = 1.713908431e-15;
/// Exponent magnitude threshold (ngspice `EXP_THRESHOLD`).
pub const EXP_THRESHOLD: Value = 34.0;
/// `exp(100.0)` guard value (ngspice `MAX_EXPL`, used by the PD threshold chain).
pub const MAX_EXPL: Value = 2.688117142e43;
/// `exp(-100.0)` guard value (ngspice `MIN_EXPL`).
pub const MIN_EXPL: Value = 3.720075976e-44;
/// PD exponent magnitude threshold (ngspice `EXPL_THRESHOLD`).
pub const EXPL_THRESHOLD: Value = 100.0;
/// Oxide permittivity used by the Berkeley source (F/m).
pub const EPSOX: Value = 3.453133e-11;
/// Silicon permittivity used by the Berkeley source (F/m).
pub const EPSSI: Value = 1.03594e-10;
/// Electron charge used by the Berkeley source (C).
pub const CHARGE_Q: Value = 1.60219e-19;
/// Boltzmann constant over charge, `Kb / q` (V/K).
pub const KB_OVER_Q: Value = 8.617087e-5;
/// Energy gap at 300K used by the diode/BJT blocks (eV).
pub const EG300: Value = 1.115;
/// Pi as written in the Berkeley source.
pub const PI: Value = 3.141592654;

// Smoothing deltas from b3soiddld.c (kept with their original names).
pub const DELTA_1: Value = 0.02;
pub const DELTA_3: Value = 0.02;
pub const DELTA_4: Value = 0.02;
pub const DELT_VBS0EFF: Value = 0.02;
pub const DELT_VBSMOS: Value = 0.005;
pub const DELT_VBSEFF: Value = 0.005;
pub const DELT_XCSAT: Value = 0.2;
pub const DELT_VBS0DIO: Value = 1e-7;
pub const DELTA_VCSCV: Value = 0.0004;
pub const DELT_VBSDIO: Value = 0.01;
pub const CONST_2OV3: Value = 0.6666666666;
pub const OFF_VBSDIO: Value = 2e-2;
pub const QEX_FACT: Value = 20.0;

/// Per-iteration absolute voltage limiter (`B3SOIDDlimit`, b3soiddld.c:76).
///
/// Limits the change of an absolute node voltage between Newton iterations.
/// Sets `*check = true` when limiting kicks in (ngspice increments
/// `CKTnoncon`).
#[inline]
pub fn soi_limit(vnew: Value, vold: Value, limit: Value, check: &mut bool) -> Value {
    let mut vnew = vnew;
    if vnew.is_nan() || vold.is_nan() {
        // ngspice resets the prediction to 0.0 and flags non-convergence.
        vnew = 0.0;
        *check = true;
        return vnew;
    }
    let t0 = vnew - vold;
    if t0.abs() > limit {
        vnew = if t0 > 0.0 { vold + limit } else { vold - limit };
        *check = true;
    }
    vnew
}
