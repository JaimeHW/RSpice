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
        let size = circuit.matrix_size();

        // Add GMIN to diagonal for numerical stability
        for i in 0..size {
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
        let size = circuit.matrix_size();

        // Add GMIN to diagonal
        for i in 0..size {
            matrix.add(i, i, gmin);
        }

        circuit.stamp_dc_direct_scaled(matrix, rhs, scale);
    }
}
