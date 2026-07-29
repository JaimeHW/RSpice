//! Shared charge companion stamper used by transient state modules.

use super::*;

/// Adapter exposing the transient [`StaticMatrix`](crate::solver::StaticMatrix)
/// + RHS pair as a [`MatrixStamper`](crate::device::MatrixStamper) for devices that stamp through the generic trait (the
/// B3SOIDD charge companion). Maps 1-indexed device NodeIds to the 0-indexed
/// matrix/RHS, matching `CircuitData`'s own stamper convention.
pub(super) struct StaticMatrixChargeStamper<'a> {
    pub(super) matrix: &'a mut crate::solver::StaticMatrix,
    pub(super) rhs: &'a mut [Value],
}

impl crate::device::MatrixStamper for StaticMatrixChargeStamper<'_> {
    #[inline]
    fn stamp(&mut self, row: crate::NodeId, col: crate::NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_rhs(&mut self, index: crate::NodeId, value: Value) {
        if index > 0 && index <= self.rhs.len() {
            self.rhs[index - 1] += value;
        }
    }
}
