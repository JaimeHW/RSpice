//! Convergence tolerances, iteration budgets, and voltage-step checks.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn should_abort_iteration(
        abort: &dyn AbortSignal,
        iteration: usize,
    ) -> bool {
        (iteration & Self::ABORT_POLL_MASK) == 0 && abort.is_aborted()
    }

    #[inline]
    pub(in crate::engine::convergence) fn nonlinear_iteration_budget(
        &self,
        multiplier: usize,
    ) -> usize {
        self.config.max_iterations.saturating_mul(multiplier).max(1)
    }

    #[inline]
    pub(in crate::engine::convergence) fn continuation_iteration_budget(
        &self,
        multiplier: usize,
        minimum: usize,
    ) -> usize {
        self.nonlinear_iteration_budget(multiplier)
            .max(minimum)
            .min(Self::MAX_CONTINUATION_CORRECTOR_ITERS)
    }

    #[inline]
    pub(in crate::engine::convergence) fn sanitize_positive_tolerance(
        value: Value,
        fallback: Value,
    ) -> Value {
        if value.is_finite() && value > 0.0 {
            value
        } else {
            fallback
        }
    }

    #[inline]
    pub(crate) fn voltage_reltol(&self) -> Value {
        Self::sanitize_positive_tolerance(self.config.convergence_config.voltage_reltol, 1e-3)
    }

    #[inline]
    pub(crate) fn voltage_abstol(&self) -> Value {
        let configured = self.config.convergence_config.voltage_abstol;
        if configured.is_finite() && configured > 0.0 {
            configured
        } else {
            Self::sanitize_positive_tolerance(self.config.tolerance, 1e-6)
        }
    }

    #[inline]
    pub(crate) fn current_abstol(&self) -> Value {
        Self::sanitize_positive_tolerance(self.config.convergence_config.current_abstol, 1e-12)
    }

    #[inline]
    pub(crate) fn charge_abstol(&self) -> Value {
        Self::sanitize_positive_tolerance(
            self.config.convergence_config.charge_abstol,
            crate::constants::CHGTOL,
        )
    }

    #[inline]
    pub(crate) fn residual_reltol(&self) -> Value {
        let configured = self.config.convergence_config.residual_reltol;
        if configured.is_finite() && configured > 0.0 {
            configured
        } else {
            self.voltage_reltol()
        }
    }

    #[inline]
    pub(crate) fn device_convergence_criteria(&self) -> NonlinearConvergenceCriteria {
        NonlinearConvergenceCriteria::new(
            self.voltage_abstol(),
            self.current_abstol(),
            self.voltage_reltol(),
        )
    }

    #[inline]
    pub(crate) fn node_voltage_convergence_met(
        &self,
        old: &[Value],
        new: &[Value],
        node_count: usize,
    ) -> bool {
        if old.len() != new.len() || old.iter().chain(new.iter()).any(|v| !v.is_finite()) {
            return false;
        }

        let limit = node_count.min(old.len()).min(new.len());
        Self::check_voltage_convergence_with_tolerances(
            &old[..limit],
            &new[..limit],
            self.voltage_abstol(),
            self.voltage_reltol(),
        )
    }

    /// Check if voltage solution has converged using legacy signature.
    ///
    /// Uses `tolerance` as an absolute voltage tolerance with default SPICE-like
    /// relative tolerance of 1e-3.
    #[allow(dead_code)]
    pub(crate) fn check_voltage_convergence(
        old: &[Value],
        new: &[Value],
        tolerance: Value,
    ) -> bool {
        Self::check_voltage_convergence_with_tolerances(old, new, tolerance, 1e-3)
    }

    /// Check voltage convergence using explicit absolute and relative tolerances.
    ///
    /// Criterion: `|Î”V| <= VABSTOL + RELTOL * max(|Vnew|, |Vold|)`
    pub(crate) fn check_voltage_convergence_with_tolerances(
        old: &[Value],
        new: &[Value],
        voltage_abstol: Value,
        voltage_reltol: Value,
    ) -> bool {
        if old.len() != new.len() {
            return false;
        }
        let abstol = Self::sanitize_positive_tolerance(voltage_abstol, 1e-12);
        let reltol = Self::sanitize_positive_tolerance(voltage_reltol, 1e-3);

        for (&v_old, &v_new) in old.iter().zip(new.iter()) {
            if !v_old.is_finite() || !v_new.is_finite() {
                return false;
            }

            let delta = (v_new - v_old).abs();
            let limit = abstol + reltol * v_new.abs().max(v_old.abs());
            if delta > limit {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_voltage_convergence_ignores_branch_current_unknowns() {
        let engine = Engine::default();
        let previous = [1.0, -2.0, 0.0, 0.0];
        let next = [1.0 + 1e-8, -2.0 - 1e-8, 1.0e3, -1.0e3];

        assert!(engine.node_voltage_convergence_met(&previous, &next, 2));
    }

    #[test]
    fn node_voltage_convergence_rejects_node_voltage_motion() {
        let engine = Engine::default();
        let previous = [1.0, -2.0, 0.0, 0.0];
        let next = [1.0 + 1e-1, -2.0, 0.0, 0.0];

        assert!(!engine.node_voltage_convergence_met(&previous, &next, 2));
    }

    #[test]
    fn node_voltage_convergence_rejects_non_finite_solution_entries() {
        let engine = Engine::default();
        let previous = [1.0, -2.0, 0.0];
        let next = [1.0, -2.0, Value::NAN];

        assert!(!engine.node_voltage_convergence_met(&previous, &next, 2));
    }
}
