//! Numerical limiting laws shared by generated Verilog-A devices.
//!
//! These helpers implement simulator iteration policy, rather than device
//! equations, and therefore belong to the handwritten runtime ABI instead of
//! any generated device module.

use crate::Value;

/// Xyce `pnjlim_new`: limit one Newton-iteration update of a PN-junction
/// voltage.
///
/// The return value is the voltage at which the device must be evaluated and
/// a flag indicating whether the candidate was changed. `thermal_voltage`
/// must be finite and strictly positive; generated callers obtain it from
/// `$vt`. `critical_voltage` is the device's forward-bias limiting threshold.
///
/// This is intentionally distinct from classic SPICE `pnjlim`. In addition to
/// forward-bias limiting, Xyce's `pnjlim_new` bounds large negative updates so
/// reverse-biased junctions cannot jump arbitrarily far in one iteration.
#[inline]
#[must_use]
pub(crate) fn pnjlim_new(
    candidate: Value,
    previous: Value,
    thermal_voltage: Value,
    critical_voltage: Value,
) -> (Value, bool) {
    if candidate > critical_voltage
        && (candidate - previous).abs() > thermal_voltage + thermal_voltage
    {
        let limited = if previous > 0.0 {
            let normalized_step = (candidate - previous) / thermal_voltage;
            if normalized_step > 0.0 {
                previous + thermal_voltage * (2.0 + (normalized_step - 2.0).ln())
            } else {
                previous - thermal_voltage * (2.0 + (2.0 - normalized_step).ln())
            }
        } else {
            thermal_voltage * (candidate / thermal_voltage).ln()
        };
        return (limited, true);
    }

    if candidate < 0.0 {
        let negative_limit = if previous > 0.0 {
            -previous - 1.0
        } else {
            2.0 * previous - 1.0
        };
        if candidate < negative_limit {
            return (negative_limit, true);
        }
    }

    (candidate, false)
}

#[cfg(test)]
mod tests {
    use super::pnjlim_new;

    // Xyce 7.10 uses its historical physical constants in adms_vt(), so use
    // the corresponding 300.15 K voltage for bit-for-formula oracle vectors.
    const XYCE_THERMAL_VOLTAGE_300_15_K: f64 = 300.15 * 1.380_622_6e-23 / 1.602_191_8e-19;
    const VBIC_CRITICAL_VOLTAGE: f64 = 1.279_470_603_277_974_6;

    fn assert_matches_xyce(candidate: f64, previous: f64, expected: f64, limited: bool) {
        let (actual, actual_limited) = pnjlim_new(
            candidate,
            previous,
            XYCE_THERMAL_VOLTAGE_300_15_K,
            VBIC_CRITICAL_VOLTAGE,
        );
        let tolerance = 4.0 * f64::EPSILON * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "candidate={candidate}, previous={previous}: expected {expected:.17e}, got {actual:.17e}"
        );
        assert_eq!(actual_limited, limited);
    }

    #[test]
    fn pnjlim_new_matches_xyce_forward_bias_oracle() {
        assert_matches_xyce(5.0, 0.0, 0.136_157_714_335_926_8, true);
        assert_matches_xyce(5.0, 0.7, 0.883_672_146_739_666, true);
    }

    #[test]
    fn pnjlim_new_matches_xyce_reverse_bias_oracle() {
        assert_matches_xyce(-2.0, 0.0, -1.0, true);
        assert_matches_xyce(-2.0, 0.7, -1.7, true);
        assert_matches_xyce(-2.0, -0.2, -1.4, true);
        assert_matches_xyce(-0.9, 0.0, -0.9, false);
    }
}
