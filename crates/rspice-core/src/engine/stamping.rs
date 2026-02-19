//! DC stamping functions
//!
//! Functions for stamping DC values into the matrix and RHS vector.

use super::Engine;
use crate::solver::StaticMatrix;
use crate::{CircuitData, Value};

impl Engine {
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
