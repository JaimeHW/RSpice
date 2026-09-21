//! Typed non-node integral coordinates and their convergence contract.

use super::*;

pub(super) mod forced;
pub(super) mod prescribed;

impl HbSolver {
    pub(crate) fn physical_branch_count(&self) -> usize {
        self.exact_mna_branches()
            .iter()
            .position(ExactMnaBranch::is_integral)
            .unwrap_or(self.exact_mna_branches().len())
    }

    pub(super) fn register_integral_coordinates(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
    ) -> Result<(), HbError> {
        let physical = self.physical_branch_count();
        let names = sources.integral_names().collect::<Vec<_>>();
        if physical != self.exact_mna_branches().len() {
            if self.exact_mna_branch_names()[physical..] != names
                || self.exact_mna_branches()[physical..]
                    .iter()
                    .any(|row| !row.is_integral())
            {
                return Err(HbError::InvalidCircuit(
                    "periodic integral basis changed after registration".into(),
                ));
            }
            return Ok(());
        }
        let total = physical.checked_add(names.len()).ok_or_else(|| {
            HbError::InvalidCircuit(
                "periodic integral coordinate count exceeds this platform".into(),
            )
        })?;
        self.num_nodes.checked_add(total).ok_or_else(|| {
            HbError::InvalidCircuit("periodic integral MNA dimension exceeds this platform".into())
        })?;
        for name in &names {
            if self
                .exact_mna_branch_names()
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(name))
            {
                return Err(HbError::InvalidCircuit(format!(
                    "periodic integral identity '{name}' duplicates an existing coordinate"
                )));
            }
        }
        for (index, name) in names.iter().enumerate() {
            self.try_push_periodic_mna_branch(
                ExactMnaBranch::IntegralState {
                    branch_ordinal: physical + index + 1,
                },
                name,
            )?;
        }
        Ok(())
    }

    pub(super) fn bind_integral_tolerances(&self, state: &mut HbSolverState) {
        let physical = self.physical_branch_count();
        state.integral_branch_start =
            (physical != self.exact_mna_branches().len()).then_some(physical);
    }
}

impl HbSolverState {
    pub(super) fn non_node_rows_converged(
        &self,
        reltol: Value,
        abstol: Value,
        voltage_abstol: Value,
        dc_only: bool,
    ) -> bool {
        let count = self.mna_branch_currents.len();
        let start = self.integral_branch_start.unwrap_or(count);
        if start > count
            || self.mna_branch_residual.len() != count
            || self.mna_branch_residual_scale.len() != count
        {
            return false;
        }
        [(0..start, voltage_abstol), (start..count, abstol)]
            .into_iter()
            .all(|(rows, tolerance)| {
                residual_rows_converged(
                    &self.mna_branch_currents[rows.clone()],
                    &self.mna_branch_residual[rows.clone()],
                    &self.mna_branch_residual_scale[rows],
                    reltol,
                    tolerance,
                    dc_only,
                )
            })
    }

    pub(super) fn non_node_merit(
        &self,
        reltol: Value,
        abstol: Value,
        voltage_abstol: Value,
        dc_only: bool,
    ) -> Result<Value, HbError> {
        let count = self.mna_branch_currents.len();
        let start = self.integral_branch_start.unwrap_or(count);
        if start > count
            || self.mna_branch_residual.len() != count
            || self.mna_branch_residual_scale.len() != count
        {
            return Err(HbError::InvalidCircuit(
                "HB non-node residual basis is inconsistent".into(),
            ));
        }
        let mut merit: Value = 0.0;
        for (rows, tolerance, label) in [
            (0..start, voltage_abstol, "KVL-voltage"),
            (start..count, abstol, "integral-rate"),
        ] {
            merit = merit.max(residual_rows_merit(
                label,
                &self.mna_branch_currents[rows.clone()],
                &self.mna_branch_residual[rows.clone()],
                &self.mna_branch_residual_scale[rows],
                reltol,
                tolerance,
                dc_only,
            )?);
        }
        Ok(merit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::behavioral::{
        BehavioralBranchResolution, BehavioralCurrentSource, BehavioralSources,
        BehavioralVoltageSource,
    };

    #[test]
    fn hb_integral_coordinates_solve_dc_constants_and_frequency_scaled_transfer() {
        for (rate, krylov) in [(1e3, false), (1e3, true), (1e9, false), (1e9, true)] {
            let frequency = rate / 10.0;
            let mut config = HbConfig::new(frequency)
                .with_harmonics(2)
                .with_tolerance(1e-10);
            config.use_krylov = krylov;
            let mut solver = HbSolver::new(config, 3);
            solver.set_node_names(vec!["in".into(), "out".into(), "sink".into()]);
            let input = solver
                .try_add_named_voltage_source_branch_harmonics(1, 0, 0.7, &[(1, 1.0, 0.0)], "v1")
                .unwrap();
            solver
                .try_add_periodic_voltage_source_branch(1, 0, input, 1, "v1")
                .unwrap();
            solver
                .try_add_periodic_constitutive_port_branch(2, 0, 2, "bv")
                .unwrap();
            solver
                .try_add_exact_mna_static_entry(4, 1, 1.0, "bv")
                .unwrap();
            solver.add_resistor(1, 3, 1e3);
            solver.add_resistor(2, 3, 1e3);
            let mut source = BehavioralVoltageSource::new(
                "bv".into(),
                2,
                0,
                2,
                &format!(".2+{rate}*sdt(v(in)-v(out))"),
            )
            .unwrap();
            source
                .bind_references(
                    |name| Some(if name == "in" { 1 } else { 2 }),
                    |_| BehavioralBranchResolution::MissingDevice,
                )
                .unwrap();
            let mut current = BehavioralCurrentSource::new(
                "bi".into(),
                3,
                0,
                &format!("-0.001*{rate}*sdt(v(out)-v(sink))"),
            )
            .unwrap();
            current
                .bind_references(
                    |name| Some(if name == "out" { 2 } else { 3 }),
                    |_| BehavioralBranchResolution::MissingDevice,
                )
                .unwrap();
            solver
                .set_periodic_behavioral_sources(
                    &BehavioralSources {
                        voltage_sources: vec![source],
                        current_sources: vec![current],
                    },
                    false,
                    usize::MAX,
                    false,
                    &crate::abort_signal::NoAbort,
                )
                .unwrap();
            let mut state = HbSolverState::new(3, 2);
            solver
                .solve_newton_with_abort(&mut state, &crate::abort_signal::NoAbort)
                .unwrap_or_else(|error| panic!("rate={rate}, krylov={krylov}: {error}"));
            assert!(state.converged);
            assert_eq!(state.integral_branch_start, Some(2));
            assert!((state.x[1][0].re - 0.7).abs() < 1e-9);
            assert!((state.mna_branch_currents[2][0].re * rate - 0.5).abs() < 1e-9);
            let transfer = rate / Complex64::new(rate, std::f64::consts::TAU * frequency);
            assert!(
                (state.x[1][1] * 2.0 - transfer).norm() < 1e-8,
                "rate={rate}, krylov={krylov}"
            );
            assert!((state.mna_branch_currents[2][1] * 2.0 * rate - transfer).norm() < 1e-8);
            assert!((state.x[2][0].re - 0.7).abs() < 1e-9);
            assert!((state.x[2][1] * 2.0 - transfer * transfer).norm() < 1e-8);
            assert!((state.mna_branch_currents[3][0].re * rate - 0.7).abs() < 1e-9);
            let leads = solver
                .nonlinear_lead_spectra(&state, &crate::abort_signal::NoAbort)
                .unwrap();
            let current = &leads.iter().find(|row| row.name == "bi").unwrap().currents[0];
            assert!((current[0].re + 0.7e-3).abs() < 1e-12);
            assert!((current[1] + transfer * transfer * 1e-3).norm() < 1e-11);
            for offset in [0.0, rate * 0.13] {
                let small_signal = solver
                    .solve_periodic_ac_with_branch_voltages(
                        &state,
                        PeriodicSidebandWindow {
                            offset_hz: offset,
                            sideband_min: 0,
                            sideband_max: 0,
                        },
                        &[PeriodicAcExcitation {
                            sideband: 0,
                            injections: Vec::new(),
                        }],
                        &[&[(0, Complex64::new(1.0, 0.0))]],
                    )
                    .unwrap();
                let h = rate / Complex64::new(rate, std::f64::consts::TAU * offset);
                assert!((small_signal[0][1][0] - h).norm() < 1e-8);
                assert!((small_signal[0][2][0] - h * h).norm() < 1e-8);
            }
            let result = solver.build_result(&state).unwrap();
            assert_eq!(result.mna_branch_currents.len(), 2);
            assert!(
                result
                    .mna_branch_currents
                    .iter()
                    .all(|row| !row.device_name.contains("sdt:"))
            );
        }
    }
}
