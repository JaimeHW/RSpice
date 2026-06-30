//! DC stamping functions
//!
//! Functions for stamping DC values into the matrix and RHS vector.

use super::Engine;
use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

impl Engine {
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
        let node_count = circuit.num_nodes();

        // Add GMIN only to node-voltage equations (not branch-current equations).
        for i in 0..node_count {
            matrix.add(i, i, gmin);
        }

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
        let node_count = circuit.num_nodes();

        // Add GMIN only to node-voltage equations (not branch-current equations).
        for i in 0..node_count {
            matrix.add(i, i, gmin);
        }

        circuit.stamp_dc_direct_scaled(matrix, rhs, scale);
    }
}
