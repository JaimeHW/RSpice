//! Operating-point linearization of the Xyce LEVEL=2 constitutive law.

use super::JilesAthertonInductor;
use crate::Value;

impl JilesAthertonInductor {
    /// At DC the winding voltage and its smoothed direction are zero. The
    /// branch law `V / mid = d(LO I)/dt` therefore linearizes to
    /// `v = mid * LO * di/dt`: voltage times a derivative of `mid` vanishes.
    /// Xyce advances a provisional M update during DC evaluation, even though
    /// it does not accept magnetic history there. Converge that update locally
    /// so the AC tangent cannot depend on unrelated electrical Newton loads.
    /// This is the converged constitutive initialization, not a reproduction
    /// of Xyce 7.10's incidental DC evaluation count. Accepted history and
    /// transient candidate caches are never modified.
    pub(crate) fn xyce_core_level2_dc_mid(&self, happ: Value) -> Option<Value> {
        if !self.is_xyce_core_level2() || !happ.is_finite() {
            return None;
        }
        let previous_happ = self.xyce_old_branch_current_sum / self.params.length;
        let delta = happ - previous_happ;
        let evaluate = |update: Value| {
            let susceptibility =
                self.xyce_core_level2_dc_susceptibility(happ, self.state.m + update)?;
            let target = susceptibility * delta;
            let residual = update - target;
            let scale = update.abs().max(target.abs()).max(Value::MIN_POSITIVE);
            (target.is_finite() && residual.is_finite()).then_some((
                susceptibility,
                residual,
                32.0 * Value::EPSILON * scale,
            ))
        };
        let (p, initial_residual, tolerance) = evaluate(0.0)?;
        if initial_residual.abs() <= tolerance {
            return self.xyce_core_level2_checked_mid(p);
        }

        // Bracket the initialization branch reached from the previous M.
        // Accept only a small constitutive residual, never interval width:
        // a susceptibility pole can otherwise masquerade as a root.
        let mut left = 0.0;
        let mut left_residual = initial_residual;
        let mut right = -initial_residual;
        let mut bracketed = false;
        for _ in 0..64 {
            let (p, residual, tolerance) = evaluate(right)?;
            if residual.abs() <= tolerance {
                return self.xyce_core_level2_checked_mid(p);
            }
            if residual.is_sign_positive() != left_residual.is_sign_positive() {
                bracketed = true;
                break;
            }
            right *= 2.0;
        }
        if !bracketed {
            return None;
        }
        for _ in 0..128 {
            let middle = left * 0.5 + right * 0.5;
            let (p, residual, tolerance) = evaluate(middle)?;
            if residual.abs() <= tolerance {
                return self.xyce_core_level2_checked_mid(p);
            }
            if middle == left || middle == right {
                return None;
            }
            if residual.is_sign_positive() == left_residual.is_sign_positive() {
                left = middle;
                left_residual = residual;
            } else {
                right = middle;
            }
        }
        None
    }

    fn xyce_core_level2_checked_mid(&self, susceptibility: Value) -> Option<Value> {
        let mid = 1.0 + (1.0 - self.params.gap / self.params.length) * susceptibility;
        (mid.is_finite() && mid != 0.0).then_some(mid)
    }

    fn xyce_core_level2_dc_susceptibility(&self, happ: Value, m: Value) -> Option<Value> {
        let p = &self.params;
        let gap = p.gap / p.length;
        let he = happ + (p.alpha - gap) * m;
        let smoothing = p.beta_h * p.a;
        let root = he.hypot(smoothing);
        let denominator = p.a + root;
        let man = p.ms * (he / denominator);
        let man_prime =
            (p.ms / denominator) * ((p.a + smoothing * (smoothing / root)) / denominator);
        let departure = (man - m).hypot(p.beta_m * p.ms);
        let pinning = 2.0 * (p.k - p.alpha * departure);
        let irreversible = departure / pinning;
        let susceptibility_denominator =
            1.0 + (gap - p.alpha) * p.c * man_prime + gap * (1.0 - p.c) * irreversible;
        let susceptibility =
            (p.c * man_prime + (1.0 - p.c) * irreversible) / susceptibility_denominator;
        // FACTORMS and hidden-variable/equation scales cancel in this
        // physical susceptibility; LEVEL=2 has no solved M or R coordinate.
        // An undefined tangent must not silently become a vacuum inductor.
        (root > 0.0
            && denominator.is_finite()
            && pinning != 0.0
            && susceptibility_denominator != 0.0
            && [
                gap,
                m,
                he,
                man,
                man_prime,
                departure,
                pinning,
                irreversible,
                susceptibility_denominator,
                susceptibility,
            ]
            .iter()
            .all(|value| value.is_finite()))
        .then_some(susceptibility)
    }
}
