//! Nonlinear operating-point stamping and device-state update helpers.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::convergence) fn stamp_nonlinear_devices_for_dc(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) {
        self.stamp_nonlinear_devices_for_operating_point(
            circuit,
            matrix,
            rhs,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
        );
    }

    #[inline]
    pub(in crate::engine::convergence) fn stamp_nonlinear_devices_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        circuit.update_nonlinear(solution);
        circuit.stamp_nonlinear(matrix, rhs, solution);
        circuit.stamp_behavioral(matrix, rhs, solution, time);
        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
            circuit.stamp_xspice(matrix, rhs);
        }
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_dc(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
    ) {
        self.update_device_states_for_operating_point(
            circuit,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
        );
    }

    #[inline]
    pub(in crate::engine::convergence) fn update_device_states_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
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
        self.update_device_states_for_operating_point(circuit, solution, time, analysis);
    }

    #[inline]
    pub(in crate::engine::convergence) fn stamp_transient_operating_point_linear(
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

        circuit.stamp_transient_operating_point_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
    }
}
