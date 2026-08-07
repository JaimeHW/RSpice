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
/// [`clear`](Self::clear) starts a new load without releasing allocation.
/// [`resize_and_clear`](Self::resize_and_clear) likewise retains existing
/// capacity whenever it can satisfy the requested dimension.
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

    /// Zeros all current contributions while retaining their allocations.
    pub(crate) fn clear(&mut self) {
        self.q.fill(0.0);
        self.f.fill(0.0);
        self.b.fill(0.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_preserves_allocations_and_vector_separation() {
        let mut vectors = XyceDaeVectors::new(3);
        let q_ptr = vectors.q.as_ptr();
        let f_ptr = vectors.f.as_ptr();
        let b_ptr = vectors.b.as_ptr();

        let (q, f, b) = vectors.q_f_b_mut();
        q.copy_from_slice(&[1.0, 2.0, 3.0]);
        f.copy_from_slice(&[4.0, 5.0, 6.0]);
        b.copy_from_slice(&[7.0, 8.0, 9.0]);
        vectors.clear();

        assert_eq!(vectors.q(), &[0.0; 3]);
        assert_eq!(vectors.f(), &[0.0; 3]);
        assert_eq!(vectors.b(), &[0.0; 3]);
        assert_eq!(vectors.q.as_ptr(), q_ptr);
        assert_eq!(vectors.f.as_ptr(), f_ptr);
        assert_eq!(vectors.b.as_ptr(), b_ptr);
    }

    #[test]
    fn resize_retains_capacity_and_clears_existing_entries() {
        let mut vectors = XyceDaeVectors::new(8);
        let capacities = (
            vectors.q.capacity(),
            vectors.f.capacity(),
            vectors.b.capacity(),
        );
        let pointers = (vectors.q.as_ptr(), vectors.f.as_ptr(), vectors.b.as_ptr());
        let (q, f, b) = vectors.q_f_b_mut();
        q.fill(1.0);
        f.fill(2.0);
        b.fill(3.0);

        vectors.resize_and_clear(3);
        assert_eq!(vectors.dimension(), 3);
        assert_eq!(vectors.q(), &[0.0; 3]);
        assert_eq!(vectors.f(), &[0.0; 3]);
        assert_eq!(vectors.b(), &[0.0; 3]);
        assert_eq!(
            (
                vectors.q.capacity(),
                vectors.f.capacity(),
                vectors.b.capacity(),
            ),
            capacities
        );
        assert_eq!(
            (vectors.q.as_ptr(), vectors.f.as_ptr(), vectors.b.as_ptr(),),
            pointers
        );

        vectors.resize_and_clear(8);
        assert_eq!(vectors.dimension(), 8);
        assert_eq!(vectors.q(), &[0.0; 8]);
        assert_eq!(vectors.f(), &[0.0; 8]);
        assert_eq!(vectors.b(), &[0.0; 8]);
        assert_eq!(
            (vectors.q.as_ptr(), vectors.f.as_ptr(), vectors.b.as_ptr(),),
            pointers
        );
    }
}
