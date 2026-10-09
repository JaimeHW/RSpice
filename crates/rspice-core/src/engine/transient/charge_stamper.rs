//! Shared charge companion stamper used by transient state modules.

use super::*;

/// Adapter exposing the transient
/// [`StaticMatrix`](crate::solver::StaticMatrix) + RHS pair as a
/// [`MatrixStamper`](crate::device::MatrixStamper) for devices that stamp
/// through the generic trait (the B3SOIDD charge companion). Maps 1-indexed
/// device NodeIds to the 0-indexed matrix/RHS, matching `CircuitData`'s own
/// stamper convention.
pub(in crate::engine) struct StaticMatrixChargeStamper<'a> {
    pub(in crate::engine) matrix: &'a mut crate::solver::StaticMatrix,
    pub(in crate::engine) rhs: &'a mut [Value],
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

/// BJT assembly can retain a direct correction RHS alongside the affine
/// system. The RHS-only pass uses the same device equations and Jacobian,
/// without adding the Jacobian a second time. Static OneStep terms alone
/// receive their half weight.
pub(super) struct BjtTransientStamper<'a> {
    pub matrix: Option<&'a mut crate::solver::StaticMatrix>,
    pub rhs: &'a mut [Value],
    pub weight: Value,
}

impl crate::device::MatrixStamper for BjtTransientStamper<'_> {
    fn stamp(&mut self, row: crate::NodeId, col: crate::NodeId, value: Value) {
        if row > 0
            && col > 0
            && let Some(matrix) = self.matrix.as_deref_mut()
        {
            matrix.add(row - 1, col - 1, self.weight * value);
        }
    }

    fn stamp_rhs(&mut self, row: crate::NodeId, value: Value) {
        if row > 0 {
            self.rhs[row - 1] += self.weight * value;
        }
    }
}
