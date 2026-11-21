//! Global differential-algebraic equation vectors.
//!
//! Xyce-compatible transient assembly must aggregate every device's charge
//! (`Q`), static contribution (`F`), and independent-source contribution (`B`)
//! into global vectors before the integration method forms a residual.  This
//! type belongs to the circuit layer so device loaders do not depend on an
//! analysis driver.

use crate::Value;

/// Reusable global `Q`, `F`, and `B` vectors for DAE assembly.
///
/// [`resize_and_clear`](Self::resize_and_clear) starts a new load without
/// releasing allocation, retaining existing capacity whenever it can satisfy
/// the requested dimension. Loaders always know the dimension, so that is the
/// only entry point production code needs.
#[derive(Clone, Debug)]
pub(crate) struct XyceDaeVectors {
    q: Vec<Value>,
    f: Vec<Value>,
    b: Vec<Value>,
}

impl XyceDaeVectors {
    /// Allocates zero-filled vectors for a circuit with `dimension` equations.
    pub(crate) fn new(dimension: usize) -> Self {
        Self {
            q: vec![0.0; dimension],
            f: vec![0.0; dimension],
            b: vec![0.0; dimension],
        }
    }

    /// Returns the common number of equations in all three vectors.
    #[inline]
    pub(crate) fn dimension(&self) -> usize {
        debug_assert_eq!(self.q.len(), self.f.len());
        debug_assert_eq!(self.q.len(), self.b.len());
        self.q.len()
    }

    /// Changes the equation count and zeros all current contributions.
    ///
    /// Shrinking does not reduce capacity. Growing reuses each allocation when
    /// its retained capacity is sufficient.
    pub(crate) fn resize_and_clear(&mut self, dimension: usize) {
        resize_and_zero(&mut self.q, dimension);
        resize_and_zero(&mut self.f, dimension);
        resize_and_zero(&mut self.b, dimension);
    }

    /// Returns the aggregated charge vector.
    #[inline]
    pub(crate) fn q(&self) -> &[Value] {
        &self.q
    }

    /// Returns the aggregated static-contribution vector.
    #[inline]
    pub(crate) fn f(&self) -> &[Value] {
        &self.f
    }

    /// Returns the aggregated independent-source vector.
    #[inline]
    pub(crate) fn b(&self) -> &[Value] {
        &self.b
    }

    /// Returns all load vectors mutably for allocation-free device stamping.
    #[inline]
    pub(crate) fn q_f_b_mut(&mut self) -> (&mut [Value], &mut [Value], &mut [Value]) {
        (&mut self.q, &mut self.f, &mut self.b)
    }
}

fn resize_and_zero(vector: &mut Vec<Value>, dimension: usize) {
    vector.resize(dimension, 0.0);
    vector.fill(0.0);
}
