//! Nonlinear operating-point stamping and device-state update helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine) fn effective_device_junction_gmin(&self, gmin: Value) -> Value {
        let floor = self.config.convergence_config.junction_gmin_target.max(0.0);
        if gmin.is_finite() && gmin > floor {
            gmin
        } else {
            floor
        }
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_static_probe_nonlinear_devices_for_dc(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        self.try_stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
            circuit,
            matrix,
            rhs,
            solution,
            junction_gmin,
        )
    }

    /// Refresh nonlinear operating-point state at the exact DC solution that
    /// will be returned to the caller.
    ///
    /// Newton convergence and the optional fixed-point polish can accept
    /// different vectors. Device reports must therefore be observed only
    /// after the final vector has been selected. The observation is
    /// transactional so a generated-model or behavioral evaluation failure
    /// cannot leave a partially refreshed operating-point cache behind.
    pub(in crate::engine) fn try_observe_dc_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        let snapshot = circuit.nonlinear_state_snapshot();
        let result = matrix.with_probe_values(|probe, rhs| {
            self.try_stamp_static_probe_nonlinear_devices_for_dc(circuit, probe, rhs, solution)
        });
        if let Err(error) = result {
            circuit.restore_nonlinear_state(snapshot);
            return Err(error);
        }
        Ok(())
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        junction_gmin: Value,
    ) -> Result<(), SimulationError> {
        self.try_stamp_static_probe_nonlinear_devices_for_operating_point(
            circuit,
            matrix,
            rhs,
            OperatingPointProbe {
                solution,
                time: 0.0,
                analysis: crate::xspice::AnalysisType::DcOp,
                junction_gmin,
            },
        )
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_static_probe_nonlinear_devices_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        probe: OperatingPointProbe<'_>,
    ) -> Result<(), SimulationError> {
        self.try_stamp_operating_point_devices(circuit, matrix, rhs, probe, true, false)
    }

    /// Complete a partially assembled operating-point system in correction
    /// coordinates. VBIC contributes its physical currents after the other
    /// devices' companions have been converted, avoiding large J*x-I sources.
    pub(in crate::engine::convergence) fn try_stamp_operating_point_correction(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        probe: OperatingPointProbe<'_>,
        static_probe: bool,
        correction_rhs: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        let solution = probe.solution;
        self.try_stamp_operating_point_devices(circuit, matrix, rhs, probe, static_probe, true)?;
        matrix.correction_rhs_into(rhs, solution, correction_rhs)?;
        circuit.stamp_promoted_vbic_correction(matrix, correction_rhs, solution);
        Ok(())
    }

    /// Assemble an operating-point Newton system using direct correction
    /// coordinates when promoted VBIC devices need them. The caller must
    /// apply constraints and interpret the solve in the same coordinates.
    pub(in crate::engine::convergence) fn try_stamp_operating_point_newton_system(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        probe: OperatingPointProbe<'_>,
        static_probe: bool,
        correction_rhs: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        if Self::requires_vbic_correction_form(circuit) {
            self.try_stamp_operating_point_correction(
                circuit,
                matrix,
                rhs,
                probe,
                static_probe,
                correction_rhs,
            )?;
            rhs.copy_from_slice(correction_rhs);
            Ok(())
        } else {
            self.try_stamp_operating_point_devices(circuit, matrix, rhs, probe, static_probe, false)
        }
    }

    fn try_stamp_operating_point_devices(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        probe: OperatingPointProbe<'_>,
        static_probe: bool,
        defer_vbic: bool,
    ) -> Result<(), SimulationError> {
        let OperatingPointProbe {
            solution,
            time,
            analysis,
            junction_gmin,
        } = probe;
        // One call assembles one complete nonlinear Newton Jacobian. Keep the
        // public convergence metric at this common assembly boundary so DC,
        // startup, and continuation paths cannot silently omit successful
        // iterations or double-count residual-only probes.
        if !static_probe {
            self.record_convergence(|quality| {
                quality.total_iterations = quality.total_iterations.saturating_add(1);
            });
        }
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_xyce_memristor_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.update_nonlinear(solution);
        if static_probe {
            circuit.update_bjt_static_linearizations(solution);
            circuit.update_b3soi_static_linearizations(solution);
            circuit.update_jfet_static_linearizations(solution);
        }
        circuit.stamp_generic_switches_with_solution(matrix, rhs, solution, time);
        if defer_vbic {
            circuit.try_stamp_nonlinear_deferred_vbic(matrix, rhs, solution, static_probe)
        } else if static_probe {
            circuit.try_stamp_static_probe_nonlinear(matrix, rhs, solution)
        } else {
            circuit.try_stamp_nonlinear(matrix, rhs, solution)
        }
        .map_err(SimulationError::from)?;
        if static_probe {
            circuit.stamp_behavioral_static_probe(matrix, rhs, solution, time, analysis)
        } else {
            circuit.stamp_behavioral(matrix, rhs, solution, time, analysis)
        }
        .map_err(SimulationError::from)?;
        if circuit.has_independent_xspice_evaluation() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
            circuit
                .stamp_xspice(matrix, rhs)
                .map_err(SimulationError::from)?;
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_mixed_signal_hosts() {
            circuit.stamp_mixed_operating_point(matrix, rhs, solution, time)?;
        }
        Ok(())
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_dc(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
    ) {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        self.update_device_states_for_dc_with_junction_gmin(circuit, solution, junction_gmin);
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_dc_with_junction_gmin(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        junction_gmin: Value,
    ) {
        self.update_device_states_for_operating_point(
            circuit,
            OperatingPointProbe {
                solution,
                time: 0.0,
                analysis: crate::xspice::AnalysisType::DcOp,
                junction_gmin,
            },
        );
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        probe: OperatingPointProbe<'_>,
    ) {
        let OperatingPointProbe {
            solution,
            time,
            analysis,
            junction_gmin,
        } = probe;
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_xyce_memristor_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.update_nonlinear(solution);
        if circuit.has_independent_xspice_evaluation() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
        }
    }

    #[inline]
    pub(in crate::engine::convergence) fn prime_operating_point_seed(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        circuit.begin_switch_initial_junction_load();
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_xyce_memristor_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.prime_nonlinear_operating_point(solution);
        if circuit.has_independent_xspice_evaluation() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
        }
    }

    #[inline]
    pub(in crate::engine::convergence) fn stamp_transient_operating_point_linear(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        gmin: Value,
        stamp_generic_switches: bool,
    ) {
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        circuit.stamp_transient_operating_point_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.stamp_transient_rhs(rhs, time);
        if stamp_generic_switches {
            circuit.stamp_generic_switches(matrix, rhs, time);
        }
    }

    #[inline]
    pub(in crate::engine::convergence) fn stamp_transient_current_seed_linear(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        gmin: Value,
        stamp_generic_switches: bool,
    ) {
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        circuit.stamp_transient_current_seed_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.stamp_transient_rhs(rhs, time);
        if stamp_generic_switches {
            circuit.stamp_generic_switches(matrix, rhs, time);
        }
    }

    /// Assemble the complete linear transient-startup system identified by an
    /// accepted operating-point contract. The solver and post-solve physical
    /// audit share this routine so behavioral and XSPICE contributions cannot
    /// silently drift between acceptance and verification.
    pub(in crate::engine::convergence) fn stamp_linear_transient_operating_point_system(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        gmin: Value,
        linear_system: TransientOperatingPointLinearSystem,
    ) -> Result<(), SimulationError> {
        match linear_system {
            TransientOperatingPointLinearSystem::IdealInductorShorts => {
                Self::stamp_transient_operating_point_linear(
                    circuit, matrix, rhs, time, gmin, true,
                );
            }
            TransientOperatingPointLinearSystem::CurrentSeededInductors => {
                Self::stamp_transient_current_seed_linear(circuit, matrix, rhs, time, gmin, true);
            }
        }

        if !circuit.behavioral_sources.is_empty()
            && !circuit.behavioral_sources.has_solution_dependent_sources()
        {
            let zero_solution = vec![0.0; rhs.len()];
            circuit
                .stamp_behavioral_sources(matrix, rhs, &zero_solution, time)
                .map_err(SimulationError::from)?;
        }
        if circuit.has_xspice_devices() {
            let zero_solution = vec![0.0; rhs.len()];
            circuit
                .stamp_xspice_transient_trial(matrix, rhs, time, 0.0, &zero_solution)
                .map_err(SimulationError::from)?;
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_mixed_signal_hosts() {
            let zero_solution = vec![0.0; rhs.len()];
            circuit.stamp_mixed_operating_point(matrix, rhs, &zero_solution, time)?;
        }
        Ok(())
    }
}
