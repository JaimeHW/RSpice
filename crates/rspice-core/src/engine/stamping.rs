//! DC stamping functions
//!
//! Functions for stamping DC values into the matrix and RHS vector.

use super::Engine;
use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

impl Engine {
    /// Add a numerical conditioning term to every MNA equation except private
    /// non-electrical DAE state rows. Unlike circuit GMIN, this intentionally
    /// covers branch-current equations so inductive and controlled-source
    /// systems retain the conditioning contract used by continuation/startup.
    pub(in crate::engine) fn stamp_matrix_conditioning_diagonal(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        matrix_size: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let node_count = circuit.num_nodes().min(matrix_size);
        for index in 0..matrix_size {
            if index >= node_count || !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, value);
            }
        }
    }

    /// Add a diagonal term to electrical node-voltage equations only.
    /// Private DAE state rows share the matrix's nodal prefix but are not
    /// voltages and therefore must not receive circuit GMIN.
    pub(in crate::engine) fn stamp_nodal_gmin(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) {
        if gmin == 0.0 {
            return;
        }
        for index in 0..circuit.num_nodes() {
            if !circuit.is_non_electrical_state_matrix_index(index) {
                matrix.add(index, index, gmin);
            }
        }
    }

    /// Final DC nodal gmin floor.
    ///
    /// B3SOI floating-body currents can be in the e-18 A range at a valid DC
    /// operating point. Xyce/ngspice apply their BSIMSOI GMIN inside the device
    /// load; adding a second simulator-level final shunt visibly deforms those
    /// roots. Keep explicit homotopy GMIN steps unchanged, but solve the final
    /// native BSIM3-SOI DC system without an extra nodal floor.
    pub(in crate::engine) fn dc_nodal_gmin_floor(&self, circuit: &CircuitData) -> Value {
        let gmin = self.config.convergence_config.gmin_target.max(0.0);
        if circuit.has_b3soi_devices() {
            0.0
        } else {
            gmin
        }
    }

    /// Stamp all DC values into matrix using O(1) direct stamping
    pub(crate) fn stamp_dc_direct(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        // Use the optimized direct stamping from CircuitData
        circuit.stamp_dc_direct(matrix, rhs);
    }

    /// Stamp with scaled sources for source stepping
    pub(crate) fn stamp_dc_scaled(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        gmin: Value,
        scale: Value,
    ) {
        // Add GMIN only to node-voltage equations (not branch-current equations).
        Self::stamp_nodal_gmin(circuit, matrix, gmin);

        circuit.stamp_dc_direct_scaled(matrix, rhs, scale);
    }
}
