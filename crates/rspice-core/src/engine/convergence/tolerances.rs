//! Convergence tolerances, iteration budgets, and voltage-step checks.

use super::*;
use crate::SpiceDialect;

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
    pub(crate) fn transient_trtol(&self) -> Value {
        match self.config.spice_dialect {
            // Xyce's StepErrorControl accepts normalized DAE-Q error at one;
            // ngspice's CKTterr applies the independent TRTOL multiplier
            // (default seven). RSpice's device charge estimators feed the
            // same accepted-history Q error into both dialect paths, so the
            // multiplier must remain dialect-specific.
            SpiceDialect::Xyce => 1.0,
            SpiceDialect::BestAvailable | SpiceDialect::Ngspice => {
                Self::sanitize_positive_tolerance(
                    self.config.transient_trtol,
                    crate::constants::TRTOL,
                )
            }
        }
    }

    #[inline]
    pub(crate) fn transient_lte_reltol(&self) -> Value {
        self.config
            .transient_lte_reltol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or_else(|| match self.config.spice_dialect {
                SpiceDialect::Xyce => 1.0e-3,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self.voltage_reltol(),
            })
    }

    #[inline]
    pub(crate) fn transient_lte_abstol(&self) -> Value {
        self.config
            .transient_lte_abstol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or_else(|| match self.config.spice_dialect {
                SpiceDialect::Xyce => 1.0e-6,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self.voltage_abstol(),
            })
    }

    #[inline]
    pub(crate) fn transient_nonlinear_reltol(&self) -> Value {
        self.config
            .transient_nonlinear_reltol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or_else(|| match self.config.spice_dialect {
                SpiceDialect::Xyce => 1.0e-2,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self.voltage_reltol(),
            })
    }

    #[inline]
    pub(crate) fn transient_nonlinear_abstol(&self) -> Value {
        self.config
            .transient_nonlinear_abstol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or_else(|| match self.config.spice_dialect {
                SpiceDialect::Xyce => 1.0e-6,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self.voltage_abstol(),
            })
    }

    #[inline]
    pub(crate) fn transient_nonlinear_deltaxtol(&self) -> Value {
        self.config
            .transient_nonlinear_deltaxtol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or(match self.config.spice_dialect {
                SpiceDialect::Xyce => 0.33,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => 1.0,
            })
    }

    #[inline]
    pub(crate) fn transient_nonlinear_rhstol(&self) -> Value {
        self.config
            .transient_nonlinear_rhstol
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or_else(|| match self.config.spice_dialect {
                SpiceDialect::Xyce => 1.0e-2,
                SpiceDialect::BestAvailable | SpiceDialect::Ngspice => self.current_abstol(),
            })
    }

    /// Resolve Xyce's `NONLIN-TRAN ENFORCEDEVICECONV` status-test policy.
    /// Xyce 7.10 deliberately disables the device-local convergence test for
    /// transient NOX solves; its weighted-update and raw-residual tests remain
    /// authoritative. Native and ngspice modes preserve the stricter legacy
    /// RSpice policy unless explicitly overridden.
    #[inline]
    pub(crate) fn transient_enforce_device_convergence(&self) -> bool {
        self.config
            .transient_enforce_device_convergence
            .unwrap_or(self.config.spice_dialect != SpiceDialect::Xyce)
    }

    /// Build Xyce's immutable transient nonlinear-update weights.
    ///
    /// Xyce 7.10 NOX computes these weights once, at nonlinear iteration zero,
    /// from the initial timepoint iterate and the previously accepted solution.
    /// It then reuses them for every subsequent update at that timepoint.
    pub(crate) fn transient_newton_update_weights(
        &self,
        initial: &[Value],
        accepted: &[Value],
    ) -> Option<Vec<Value>> {
        if self.config.spice_dialect != SpiceDialect::Xyce || initial.len() != accepted.len() {
            return None;
        }

        let reltol = self.transient_nonlinear_reltol();
        let abstol = self.transient_nonlinear_abstol();
        Some(
            initial
                .iter()
                .zip(accepted)
                .map(|(&initial, &accepted)| reltol * initial.abs().max(accepted.abs()) + abstol)
                .collect(),
        )
    }

    /// Test a transient Newton correction using the active solver's update
    /// contract. Xyce applies its frozen weighted max norm to every MNA
    /// unknown, including branch currents and private device states, and does
    /// not allow the iteration-zero update test to declare convergence.
    #[inline]
    pub(crate) fn transient_newton_update_convergence_met(
        &self,
        old: &[Value],
        new: &[Value],
        node_count: usize,
        xyce_weights: Option<&[Value]>,
        iteration: usize,
    ) -> bool {
        if old.len() != new.len() || old.iter().chain(new.iter()).any(|v| !v.is_finite()) {
            return false;
        }

        if self.config.spice_dialect == SpiceDialect::Xyce {
            if iteration == 0 {
                return false;
            }
            return self
                .transient_newton_weighted_update_norm(old, new, xyce_weights)
                .is_some_and(|norm| norm < self.transient_nonlinear_deltaxtol());
        }

        let limit = node_count.min(old.len()).min(new.len());
        Self::check_voltage_convergence_with_tolerances(
            &old[..limit],
            &new[..limit],
            self.voltage_abstol(),
            self.voltage_reltol(),
        )
    }

    /// Compute Xyce's frozen weighted maximum norm for one transient Newton
    /// correction. Returning the norm (rather than only a boolean) lets the
    /// ordered NOX status tests apply both DELTAXTOL and SMALLUPDATETOL to the
    /// same canonical quantity.
    pub(crate) fn transient_newton_weighted_update_norm(
        &self,
        old: &[Value],
        new: &[Value],
        xyce_weights: Option<&[Value]>,
    ) -> Option<Value> {
        if self.config.spice_dialect != SpiceDialect::Xyce
            || old.len() != new.len()
            || old.iter().chain(new.iter()).any(|value| !value.is_finite())
        {
            return None;
        }
        let weights = xyce_weights.filter(|weights| weights.len() == old.len())?;
        old.iter()
            .zip(new)
            .zip(weights)
            .try_fold(0.0_f64, |norm, ((&old, &new), &weight)| {
                if !weight.is_finite() || weight <= 0.0 {
                    None
                } else {
                    Some(norm.max((new - old).abs() / weight))
                }
            })
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

    /// Apply the nonlinear update status test for a DC Newton solve.
    ///
    /// Xyce recomputes DC weights at every nonlinear iteration from the new
    /// candidate and the fixed accepted/startup solution in DataStore. It
    /// measures every MNA unknown, including voltage-source branch currents,
    /// and deliberately prevents convergence on nonlinear iteration zero.
    pub(crate) fn dc_newton_update_convergence_met(
        &self,
        old: &[Value],
        new: &[Value],
        accepted: &[Value],
        node_count: usize,
        iteration: usize,
    ) -> bool {
        if self.config.spice_dialect != SpiceDialect::Xyce {
            return self.node_voltage_convergence_met(old, new, node_count);
        }
        if iteration == 0
            || old.len() != new.len()
            || new.len() != accepted.len()
            || old
                .iter()
                .chain(new)
                .chain(accepted)
                .any(|value| !value.is_finite())
        {
            return false;
        }

        const XYCE_DC_RELTOL: Value = 1.0e-3;
        const XYCE_DC_ABSTOL: Value = 1.0e-12;
        const XYCE_DC_DELTAXTOL: Value = 1.0;
        old.iter()
            .zip(new)
            .zip(accepted)
            .all(|((&old_value, &new_value), &accepted_value)| {
                let weight =
                    XYCE_DC_RELTOL * new_value.abs().max(accepted_value.abs()) + XYCE_DC_ABSTOL;
                (new_value - old_value).abs() / weight < XYCE_DC_DELTAXTOL
            })
    }

    /// Check voltage convergence using explicit absolute and relative tolerances.
    ///
    /// Criterion: `|ΔV| <= VABSTOL + RELTOL * max(|Vnew|, |Vold|)`
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

    #[test]
    fn xyce_dc_update_uses_fixed_accepted_reference_and_full_mna_vector() {
        let mut engine = Engine::default();
        engine.config.spice_dialect = SpiceDialect::Xyce;

        assert!(!engine.dc_newton_update_convergence_met(
            &[0.0, 0.0],
            &[0.0, 0.0],
            &[0.0, 0.0],
            1,
            0,
        ));
        assert!(engine.dc_newton_update_convergence_met(
            &[0.0, 0.0],
            &[5.0e-4, 0.0],
            &[1.0, 0.0],
            1,
            1,
        ));
        assert!(!engine.dc_newton_update_convergence_met(
            &[0.0, 0.0],
            &[5.0e-4, 1.0e-6],
            &[1.0, 0.0],
            1,
            1,
        ));
    }

    #[test]
    fn xyce_timeint_defaults_are_independent_from_newton_tolerances() {
        let mut engine = Engine::default();
        engine.config.spice_dialect = SpiceDialect::Xyce;
        engine.config.convergence_config.voltage_reltol = 0.25;
        engine.config.convergence_config.voltage_abstol = 0.5;

        assert_eq!(engine.transient_lte_reltol(), 1.0e-3);
        assert_eq!(engine.transient_lte_abstol(), 1.0e-6);

        engine.config.transient_lte_reltol = Some(2.0e-5);
        engine.config.transient_lte_abstol = Some(3.0e-8);
        assert_eq!(engine.transient_lte_reltol(), 2.0e-5);
        assert_eq!(engine.transient_lte_abstol(), 3.0e-8);
    }

    #[test]
    fn native_timeint_defaults_retain_legacy_newton_tolerance_fallbacks() {
        for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
            let mut engine = Engine::default();
            engine.config.spice_dialect = dialect;
            engine.config.convergence_config.voltage_reltol = 0.25;
            engine.config.convergence_config.voltage_abstol = 0.5;

            assert_eq!(engine.transient_lte_reltol(), 0.25);
            assert_eq!(engine.transient_lte_abstol(), 0.5);
        }
    }

    #[test]
    fn transient_nonlinear_tolerance_fallbacks_are_dialect_specific() {
        let mut engine = Engine::default();
        engine.config.convergence_config.voltage_reltol = 0.25;
        engine.config.convergence_config.voltage_abstol = 0.5;
        engine.config.convergence_config.current_abstol = 0.75;

        for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
            engine.config.spice_dialect = dialect;
            assert_eq!(engine.transient_nonlinear_reltol(), 0.25);
            assert_eq!(engine.transient_nonlinear_abstol(), 0.5);
            assert_eq!(engine.transient_nonlinear_deltaxtol(), 1.0);
            assert_eq!(engine.transient_nonlinear_rhstol(), 0.75);
            assert!(engine.transient_enforce_device_convergence());
            assert_eq!(engine.transient_trtol(), crate::constants::TRTOL);
        }

        engine.config.spice_dialect = SpiceDialect::Xyce;
        assert_eq!(engine.transient_nonlinear_reltol(), 1.0e-2);
        assert_eq!(engine.transient_nonlinear_abstol(), 1.0e-6);
        assert_eq!(engine.transient_nonlinear_deltaxtol(), 0.33);
        assert_eq!(engine.transient_nonlinear_rhstol(), 1.0e-2);
        assert!(!engine.transient_enforce_device_convergence());
        assert_eq!(engine.transient_trtol(), 1.0);

        engine.config.transient_enforce_device_convergence = Some(true);
        assert!(engine.transient_enforce_device_convergence());
    }

    #[test]
    fn xyce_transient_newton_update_honors_nonlin_tran_options() {
        let mut engine = Engine::default();
        engine.config.spice_dialect = SpiceDialect::Xyce;
        engine.config.transient_nonlinear_reltol = Some(0.5);
        engine.config.transient_nonlinear_abstol = Some(0.25);
        engine.config.transient_nonlinear_deltaxtol = Some(0.2);

        let initial = [2.0];
        let accepted = [1.0];
        let weights = engine
            .transient_newton_update_weights(&initial, &accepted)
            .expect("Xyce weights");
        assert_eq!(weights, vec![1.25]);
        assert!(engine.transient_newton_update_convergence_met(
            &initial,
            &[2.24],
            1,
            Some(&weights),
            1,
        ));
        assert!(!engine.transient_newton_update_convergence_met(
            &initial,
            &[2.25],
            1,
            Some(&weights),
            1,
        ));
    }

    #[test]
    fn xyce_transient_newton_update_uses_frozen_full_vector_weights() {
        let mut engine = Engine::default();
        engine.config.spice_dialect = SpiceDialect::Xyce;
        engine.config.convergence_config.voltage_reltol = 1.0e-4;
        engine.config.convergence_config.voltage_abstol = 1.0e-6;

        let initial = [0.5, 1.0e-3];
        let accepted = [0.4, 0.0];
        let weights = engine
            .transient_newton_update_weights(&initial, &accepted)
            .expect("Xyce weights");
        assert_eq!(
            weights,
            vec![1.0e-2 * 0.5 + 1.0e-6, 1.0e-2 * 1.0e-3 + 1.0e-6]
        );
        assert!(!engine.transient_newton_update_convergence_met(
            &initial,
            &[0.5, 1.0e-3],
            1,
            Some(&weights),
            0,
        ));
        assert!(engine.transient_newton_update_convergence_met(
            &initial,
            &[0.500_1, 1.003e-3],
            1,
            Some(&weights),
            1,
        ));
        assert!(!engine.transient_newton_update_convergence_met(
            &initial,
            &[0.500_1, 1.004e-3],
            1,
            Some(&weights),
            1,
        ));

        // The denominator remains tied to the initial timepoint scale even
        // after an intermediate Newton iterate grows by orders of magnitude.
        assert!(!engine.transient_newton_update_convergence_met(
            &[1.0e3, 1.0e-3],
            &[1.0e3 + 1.0, 1.0e-3],
            1,
            Some(&weights),
            2,
        ));
    }
}
