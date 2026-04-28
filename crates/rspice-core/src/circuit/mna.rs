//! Modified Nodal Analysis (MNA) matrix builder

use nalgebra_sparse::{CooMatrix, CsrMatrix};
use crate::{Value, device::MatrixStamper};
use super::NodeId;

/// MNA matrix builder using coordinate (triplet) format
/// Efficiently accumulates stamps before converting to CSR for solving
pub struct MnaBuilder {
    size: usize,
    /// Triplets: (row, col, value)
    triplets: Vec<(usize, usize, Value)>,
    /// Right-hand side vector
    rhs: Vec<Value>,
}

impl MnaBuilder {
    /// Create a new MNA builder with given matrix size
    pub fn new(size: usize) -> Self {
        Self {
            size,
            triplets: Vec::with_capacity(size * 4), // Estimate
            rhs: vec![0.0; size],
        }
    }

    /// Reset the matrix for a new iteration
    pub fn reset(&mut self) {
        self.triplets.clear();
        self.rhs.fill(0.0);
    }

    /// Convert to CSR matrix for efficient solving
    pub fn to_csr(&self) -> CsrMatrix<Value> {
        let mut coo = CooMatrix::new(self.size, self.size);
        
        // Accumulate duplicate entries
        for &(row, col, value) in &self.triplets {
            if row > 0 && col > 0 && row <= self.size && col <= self.size {
                coo.push(row - 1, col - 1, value);
            }
        }
        
        CsrMatrix::from(&coo)
    }

    /// Get the RHS vector (excluding ground node)
    pub fn rhs(&self) -> &[Value] {
        // Skip ground node (index 0)
        if self.rhs.len() > 1 {
            &self.rhs[1..]
        } else {
            &[]
        }
    }

    /// Get mutable RHS vector
    pub fn rhs_mut(&mut self) -> &mut [Value] {
        if self.rhs.len() > 1 {
            &mut self.rhs[1..]
        } else {
            &mut []
        }
    }

    /// Get the full size
    pub fn size(&self) -> usize {
        self.size
    }
}

impl MatrixStamper for MnaBuilder {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        // Ground node (0) stamps are ignored
        if row != 0 && col != 0 {
            self.triplets.push((row, col, value));
        }
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        if index != 0 && index < self.rhs.len() {
            self.rhs[index] += value;
        }
    }
}

/// Dense MNA matrix for small circuits or debugging
pub struct DenseMna {
    size: usize,
    /// Conductance matrix
    g: Vec<Value>,
    /// Right-hand side
    rhs: Vec<Value>,
}

impl DenseMna {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            g: vec![0.0; size * size],
            rhs: vec![0.0; size],
        }
    }

    pub fn reset(&mut self) {
        self.g.fill(0.0);
        self.rhs.fill(0.0);
    }

    /// Get element at (row, col)
    pub fn get(&self, row: usize, col: usize) -> Value {
        self.g[row * self.size + col]
    }

    /// Set element at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: Value) {
        self.g[row * self.size + col] = value;
    }

    /// Add to element at (row, col)
    pub fn add(&mut self, row: usize, col: usize, value: Value) {
        self.g[row * self.size + col] += value;
    }

    /// Get RHS vector
    pub fn rhs(&self) -> &[Value] {
        &self.rhs[1..] // Skip ground
    }

    /// Get the dense matrix as a slice (row-major)
    pub fn matrix_data(&self) -> &[Value] {
        &self.g
    }
}

impl MatrixStamper for DenseMna {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row != 0 && col != 0 && row <= self.size && col <= self.size {
            self.add(row - 1, col - 1, value);
        }
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        if index != 0 && index <= self.size {
            self.rhs[index] += value;
        }
    }
}

