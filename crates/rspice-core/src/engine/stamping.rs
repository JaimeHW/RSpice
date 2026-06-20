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
    /// operating point. A global e-15 shunt visibly deforms those roots, while
    /// ngspice's SOI body-row floor is effectively e-18 (`CKTgmin * 1e-6`).
    /// Keep explicit homotopy gmin steps unchanged; only reduce the physical
    /// final floor for circuits containing native BSIM3-SOI devices.
    pub(in crate::engine) fn dc_nodal_gmin_floor(&self, circuit: &CircuitData) -> Value {
        let gmin = self.config.convergence_config.gmin_target.max(0.0);
        if circuit.has_b3soi_devices() {
            gmin.min(1.0e-18)
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
