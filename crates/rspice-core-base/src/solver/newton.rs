//! Newton-Raphson step limiting for junction voltages

use crate::Value;

/// Limit voltage changes to prevent numerical issues
pub fn limit_voltage_step(v_old: Value, v_new: Value, v_crit: Value) -> Value {
    let v_diff = v_new - v_old;

    if v_diff.abs() > v_crit {
        // Limit the step
        v_old + v_diff.signum() * v_crit
    } else {
        v_new
    }
}

/// Limit junction voltage for PN devices (diodes, BJTs)
pub fn limit_pn_voltage(v_old: Value, v_new: Value, vt: Value) -> Value {
    // Critical voltage is typically 10 * thermal voltage
    let v_crit = 10.0 * vt;

    if v_new > v_crit && (v_new - v_old).abs() > 2.0 * vt {
        // Use logarithmic limiting
        if v_old > 0.0 {
            let arg = 1.0 + (v_new - v_old) / vt;
            if arg > 0.0 {
                return v_old + vt * arg.ln();
            }
        }
        vt * (1.0 + (v_new / vt).ln())
    } else {
        limit_voltage_step(v_old, v_new, v_crit)
    }
}
