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
    pub(in crate::engine::convergence) fn try_stamp_nonlinear_devices_for_dc(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        self.try_stamp_nonlinear_devices_for_dc_with_junction_gmin(
            circuit,
            matrix,
            rhs,
            solution,
            junction_gmin,
        )
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_nonlinear_devices_for_dc_with_junction_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        junction_gmin: Value,
    ) -> Result<(), SimulationError> {
        self.try_stamp_nonlinear_devices_for_operating_point(
            circuit,
            matrix,
            rhs,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
            junction_gmin,
        )
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
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
            junction_gmin,
        )
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_static_probe_nonlinear_devices_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
        junction_gmin: Value,
    ) -> Result<(), SimulationError> {
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.update_nonlinear(solution);
        circuit.update_bjt_static_linearizations(solution);
        circuit.update_b3soi_static_linearizations(solution);
        circuit.update_jfet_static_linearizations(solution);
        circuit.stamp_generic_switches(matrix, rhs, time);
        circuit
            .try_stamp_static_probe_nonlinear(matrix, rhs, solution)
            .map_err(SimulationError::Circuit)?;
        circuit.stamp_behavioral_static_probe(matrix, rhs, solution, time, analysis);
        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
            circuit.stamp_xspice(matrix, rhs);
        }
        Ok(())
    }

    #[inline]
    pub(in crate::engine::convergence) fn try_stamp_nonlinear_devices_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
        junction_gmin: Value,
    ) -> Result<(), SimulationError> {
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.update_nonlinear(solution);
        circuit.stamp_generic_switches(matrix, rhs, time);
        circuit
            .try_stamp_nonlinear(matrix, rhs, solution)
            .map_err(SimulationError::Circuit)?;
        circuit.stamp_behavioral(matrix, rhs, solution, time, analysis);
        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
            circuit.stamp_xspice(matrix, rhs);
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
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
            junction_gmin,
        );
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
        junction_gmin: Value,
    ) {
        circuit.set_b3soi_operating_point_mode(true);
        circuit.set_semiconductor_junction_gmin(junction_gmin);
        circuit.update_nonlinear(solution);
        if circuit.has_xspice_devices() {
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
        self.update_device_states_for_operating_point(
            circuit,
            solution,
            time,
            analysis,
            junction_gmin,
        );
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
        let node_count = circuit.num_nodes().min(rhs.len());
        for i in 0..node_count {
            matrix.add(i, i, gmin);
        }

        circuit.stamp_transient_operating_point_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
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
    ) {
        let node_count = circuit.num_nodes().min(rhs.len());
        for i in 0..node_count {
            matrix.add(i, i, gmin);
        }

        circuit.stamp_transient_current_seed_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
        circuit.stamp_generic_switches(matrix, rhs, time);
    }
}
