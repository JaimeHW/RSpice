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

/// Where a promoted BJT's companions write: `newton` takes the Jacobian and
/// the right-hand side about `anchor`, and the direct correction, when there
/// is one, takes only its own right-hand side, about the voltages it carries.
pub(super) struct BjtLoadTargets<'s, N, C> {
    pub newton: &'s mut N,
    pub anchor: Option<&'s [Value]>,
    pub correction: Option<(&'s mut C, &'s [Value])>,
}

impl<'s, N> BjtLoadTargets<'s, N, N> {
    /// One stamper taking the Jacobian and the right-hand side about `anchor`.
    pub(super) fn single(newton: &'s mut N, anchor: Option<&'s [Value]>) -> Self {
        Self {
            newton,
            anchor,
            correction: None,
        }
    }
}

impl<N: crate::device::MatrixStamper, C: crate::device::MatrixStamper> BjtLoadTargets<'_, N, C> {
    /// `source(anchor)` for each right-hand side, about its own anchor.
    pub(super) fn sources(
        &self,
        mut source: impl FnMut(Option<&[Value]>) -> Value,
    ) -> BjtLoadSources {
        BjtLoadSources {
            newton: source(self.anchor),
            correction: self
                .correction
                .as_ref()
                .map(|(_, anchor)| source(Some(anchor))),
        }
    }

    pub(super) fn stamp(&mut self, row: crate::NodeId, col: crate::NodeId, value: Value) {
        self.newton.stamp(row, col, value);
    }

    /// Add each right-hand side's entry of `sources` at `row`.
    pub(super) fn stamp_rhs(&mut self, row: crate::NodeId, sources: BjtLoadSources) {
        self.newton.stamp_rhs(row, sources.newton);
        if let (Some((stamper, _)), Some(value)) = (self.correction.as_mut(), sources.correction) {
            stamper.stamp_rhs(row, value);
        }
    }
}

/// One value for each right-hand side a promoted BJT's companions write.
#[derive(Clone, Copy)]
pub(super) struct BjtLoadSources {
    newton: Value,
    correction: Option<Value>,
}

impl BjtLoadSources {
    pub(super) fn map(self, f: impl Fn(Value) -> Value) -> Self {
        Self {
            newton: f(self.newton),
            correction: self.correction.map(f),
        }
    }

    pub(super) fn all(self, f: impl Fn(Value) -> bool) -> bool {
        f(self.newton) && self.correction.is_none_or(f)
    }
}

/// The right-hand side additions of a stamp, without its Jacobian.
pub(super) struct RhsOnly<'s, S>(pub &'s mut S);

impl<S: crate::device::MatrixStamper> crate::device::MatrixStamper for RhsOnly<'_, S> {
    fn stamp(&mut self, _row: crate::NodeId, _col: crate::NodeId, _value: Value) {}

    fn stamp_rhs(&mut self, row: crate::NodeId, value: Value) {
        self.0.stamp_rhs(row, value);
    }
}
