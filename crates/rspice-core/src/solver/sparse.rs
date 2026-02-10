//! High-performance sparse matrix solver using faer
//!
//! Uses faer's sparse LU decomposition for circuit simulation.
//! Provides O(n) scaling for typical circuit matrices.
//!
//! Key optimization: Static structure matrix that caches topology
//! and allows updates to values only, avoiding O(N log N) rebuild.

use super::SolverError;
use crate::Value;
use faer::Mat;
use faer::linalg::solvers::Solve;
use faer::sparse::linalg::solvers::{Lu, SymbolicLu};
use faer::sparse::{SparseColMat, SymbolicSparseColMat};

//=============================================================================
// Static Structure Matrix - The Key Optimization
//=============================================================================

/// Pre-computed stamp location that maps directly to CSC values array
#[derive(Debug, Clone, Copy)]
pub struct CscIndex(pub usize);

/// Pre-built matrix structure with static topology
///
/// This is the critical optimization: we build the structure once during
/// circuit setup, then only update the values during Newton-Raphson iterations.
/// This avoids the O(N log N) sort and memory allocation on every solve.
pub struct StaticMatrix {
    /// Matrix dimensions
    pub nrows: usize,
    pub ncols: usize,
    /// CSC column pointers (frozen after setup)
    col_ptrs: Vec<usize>,
    /// CSC row indices (frozen after setup)
    row_indices: Vec<usize>,
    /// CSC values (mutable - updated each iteration)
    values: Vec<Value>,
    /// Mapping from (row, col) to index in values array
    /// This enables O(1) stamping during simulation
    position_map: std::collections::HashMap<(usize, usize), usize>,
    /// Cached symbolic LU factorization
    symbolic_lu: Option<SymbolicLu<usize>>,
}

impl StaticMatrix {
    /// Build static structure from triplets (called once during setup)
    pub fn from_triplets(
        nrows: usize,
        ncols: usize,
        triplets: &[(usize, usize, Value)],
    ) -> Result<Self, SolverError> {
        if nrows == 0 || ncols == 0 {
            return Err(SolverError::InvalidCircuit("Empty matrix".to_string()));
        }
        for (idx, &(row, col, _)) in triplets.iter().enumerate() {
            if row >= nrows || col >= ncols {
                return Err(SolverError::InvalidCircuit(format!(
                    "Triplet {} index out of bounds: ({}, {}) for matrix {}x{}",
                    idx, row, col, nrows, ncols
                )));
            }
        }

        // Sort by (col, row) for CSC format
        let mut entries: Vec<(usize, usize, Value)> = triplets.to_vec();
        entries.sort_by_key(|&(r, c, _)| (c, r));

        // Accumulate duplicates and build position map
        let mut accumulated: Vec<(usize, usize, Value)> = Vec::with_capacity(entries.len());
        let mut position_map = std::collections::HashMap::new();

        for (r, c, v) in entries {
            if let Some(last) = accumulated.last_mut() {
                if last.0 == r && last.1 == c {
                    last.2 += v;
                    continue;
                }
            }
            let idx = accumulated.len();
            position_map.insert((r, c), idx);
            accumulated.push((r, c, v));
        }

        // Build CSC arrays
        let mut col_ptrs = vec![0usize; ncols + 1];
        let mut row_indices = Vec::with_capacity(accumulated.len());
        let mut values = Vec::with_capacity(accumulated.len());

        for &(r, c, v) in &accumulated {
            col_ptrs[c + 1] += 1;
            row_indices.push(r);
            values.push(v);
        }

        // Cumulative sum for col_ptrs
        for i in 1..=ncols {
            col_ptrs[i] += col_ptrs[i - 1];
        }

        Ok(Self {
            nrows,
            ncols,
            col_ptrs,
            row_indices,
            values,
            position_map,
            symbolic_lu: None,
        })
    }

    /// Zero all values (call before each Newton iteration)
    ///
    /// When compiled with the `simd` feature, uses SIMD instructions
    /// for 2-4x faster clearing on large matrices.
    #[inline]
    pub fn clear_values(&mut self) {
        #[cfg(feature = "simd")]
        {
            crate::simd::fill_zero(&mut self.values);
        }
        #[cfg(not(feature = "simd"))]
        {
            self.values.fill(0.0);
        }
    }

    /// Add value at (row, col) - O(1) using position map
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Value) {
        if let Some(&idx) = self.position_map.get(&(row, col)) {
            self.values[idx] += value;
        } else {
            #[cfg(debug_assertions)]
            panic!(
                "StaticMatrix::add called with unknown position ({}, {})",
                row, col
            );
        }
    }

    /// Get CSC index for (row, col) - for pre-indexed stamping
    #[inline]
    pub fn get_index(&self, row: usize, col: usize) -> Option<CscIndex> {
        self.position_map.get(&(row, col)).map(|&i| CscIndex(i))
    }

    /// Direct write to values array using pre-computed index
    #[inline]
    pub fn stamp_direct(&mut self, idx: CscIndex, value: Value) {
        self.values[idx.0] += value;
    }

    /// Get the number of values in the matrix (for parallel matrix sizing)
    #[inline]
    pub fn values_len(&self) -> usize {
        self.values.len()
    }

    /// Copy values from an atomic matrix (after parallel stamping)
    #[cfg(feature = "parallel")]
    pub fn copy_values_from_atomic(&mut self, atomic: &super::parallel::AtomicMatrix) {
        atomic.copy_to(&mut self.values);
    }

    /// Get mutable access to values slice (for advanced use)
    #[inline]
    pub fn values_mut(&mut self) -> &mut [Value] {
        &mut self.values
    }

    /// Compute infinity norm of the scaled residual `A*x-b`.
    ///
    /// Each row is normalized with a SPICE-like tolerance scale:
    /// `abstol + reltol * max(|A*x|, |b|)`.
    /// Returns `inf` when input vectors contain non-finite values.
    pub fn scaled_residual_inf_norm(
        &self,
        solution: &[Value],
        rhs: &[Value],
        abstol: Value,
        reltol: Value,
    ) -> Result<Value, SolverError> {
        if self.nrows != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix rows {} don't match RHS size {}",
                self.nrows,
                rhs.len()
            )));
        }
        if self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix cols {} don't match solution size {}",
                self.ncols,
                solution.len()
            )));
        }

        let safe_abstol = if abstol.is_finite() && abstol > 0.0 {
            abstol
        } else {
            1e-12
        };
        let safe_reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };

        let mut ax = vec![0.0; self.nrows];
        for col in 0..self.ncols {
            let x = solution[col];
            if !x.is_finite() {
                return Ok(Value::INFINITY);
            }
            for idx in self.col_ptrs[col]..self.col_ptrs[col + 1] {
                let row = self.row_indices[idx];
                ax[row] += self.values[idx] * x;
            }
        }

        let mut residual_inf: Value = 0.0;
        for row in 0..self.nrows {
            let row_rhs = rhs[row];
            let row_ax = ax[row];
            if !row_rhs.is_finite() || !row_ax.is_finite() {
                return Ok(Value::INFINITY);
            }
            let residual = (row_ax - row_rhs).abs();
            let scale = safe_abstol + safe_reltol * row_ax.abs().max(row_rhs.abs());
            let normalized = residual / scale.max(safe_abstol);
            residual_inf = residual_inf.max(normalized);
        }

        Ok(residual_inf)
    }

    /// Convert to faer SparseColMat (borrows values, no allocation)
    fn to_sparse_col_mat(&self) -> SparseColMat<usize, Value> {
        let symbolic = SymbolicSparseColMat::new_checked(
            self.nrows,
            self.ncols,
            self.col_ptrs.clone(),
            None,
            self.row_indices.clone(),
        );
        SparseColMat::new(symbolic, self.values.clone())
    }

    /// Solve Ax = b using cached structure
    pub fn solve(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let n = self.nrows;

        if n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {} doesn't match RHS size {}",
                n,
                rhs.len()
            )));
        }

        let sparse_mat = self.to_sparse_col_mat();

        // Create or reuse symbolic factorization
        let symbolic = match &self.symbolic_lu {
            Some(s) => s.clone(),
            None => {
                let s = SymbolicLu::try_new(sparse_mat.symbolic())
                    .map_err(|_| SolverError::SingularMatrix)?;
                self.symbolic_lu = Some(s.clone());
                s
            }
        };

        // Factorize with current values
        let lu = Lu::try_new_with_symbolic(symbolic, sparse_mat.as_ref())
            .map_err(|_| SolverError::SingularMatrix)?;

        // Create RHS as column vector
        let mut b = Mat::<Value>::zeros(n, 1);
        for (i, &val) in rhs.iter().enumerate() {
            b[(i, 0)] = val;
        }

        // Solve in-place
        lu.solve_in_place(b.as_mut());

        // Extract solution
        Ok((0..n).map(|i| b[(i, 0)]).collect())
    }
}

//=============================================================================
// Complex Matrix for AC Analysis
//=============================================================================

use num_complex::Complex64;

/// ComplexMatrix for AC small-signal analysis
///
/// Shares the same sparsity structure as a StaticMatrix but uses Complex64 values.
/// This enables AC analysis at different frequencies without rebuilding topology.
pub struct ComplexMatrix {
    /// Matrix dimensions
    pub nrows: usize,
    pub ncols: usize,
    /// CSC column pointers (cloned from real matrix)
    col_ptrs: Vec<usize>,
    /// CSC row indices (cloned from real matrix)
    row_indices: Vec<usize>,
    /// Complex values (updated for each frequency)
    values: Vec<Complex64>,
    /// Mapping from (row, col) to index in values array
    position_map: std::collections::HashMap<(usize, usize), usize>,
}

impl ComplexMatrix {
    /// Create a ComplexMatrix with the same structure as a StaticMatrix
    pub fn from_real_structure(real_matrix: &StaticMatrix) -> Self {
        let nnz = real_matrix.values.len();
        Self {
            nrows: real_matrix.nrows,
            ncols: real_matrix.ncols,
            col_ptrs: real_matrix.col_ptrs.clone(),
            row_indices: real_matrix.row_indices.clone(),
            values: vec![Complex64::new(0.0, 0.0); nnz],
            position_map: real_matrix.position_map.clone(),
        }
    }

    /// Zero all values
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.fill(Complex64::new(0.0, 0.0));
    }

    /// Add real value at (row, col)
    #[inline]
    pub fn add_real(&mut self, row: usize, col: usize, value: Value) {
        if let Some(&idx) = self.position_map.get(&(row, col)) {
            self.values[idx] += Complex64::new(value, 0.0);
        }
    }

    /// Add complex value at (row, col)
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Complex64) {
        if let Some(&idx) = self.position_map.get(&(row, col)) {
            self.values[idx] += value;
        }
    }

    /// Add imaginary value (for frequency-dependent terms like jwC)
    #[inline]
    pub fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        if let Some(&idx) = self.position_map.get(&(row, col)) {
            self.values[idx] += Complex64::new(0.0, value);
        }
    }

    /// Solve Ax = b for complex values
    pub fn solve(&self, rhs: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
        let n = self.nrows;

        if n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {} doesn't match RHS size {}",
                n,
                rhs.len()
            )));
        }

        // Build faer complex sparse matrix using Complex64 directly
        let symbolic = SymbolicSparseColMat::new_checked(
            self.nrows,
            self.ncols,
            self.col_ptrs.clone(),
            None,
            self.row_indices.clone(),
        );

        let sparse_mat = SparseColMat::new(symbolic, self.values.clone());

        // Factorize
        let symbolic_lu =
            SymbolicLu::try_new(sparse_mat.symbolic()).map_err(|_| SolverError::SingularMatrix)?;
        let lu = Lu::try_new_with_symbolic(symbolic_lu, sparse_mat.as_ref())
            .map_err(|_| SolverError::SingularMatrix)?;

        // Create RHS as column vector
        let mut b = Mat::<Complex64>::zeros(n, 1);
        for (i, &val) in rhs.iter().enumerate() {
            b[(i, 0)] = val;
        }

        // Solve in-place
        lu.solve_in_place(b.as_mut());

        // Extract solution
        Ok((0..n).map(|i| b[(i, 0)]).collect())
    }
}

//=============================================================================
// Legacy TripletMatrix (for initial structure building)
//=============================================================================

/// Sparse matrix in triplet (COO) format for accumulating stamps
#[derive(Debug, Clone)]
pub struct TripletMatrix {
    pub nrows: usize,
    pub ncols: usize,
    pub row_indices: Vec<usize>,
    pub col_indices: Vec<usize>,
    pub values: Vec<Value>,
}

impl TripletMatrix {
    /// Create a new triplet matrix
    pub fn new(size: usize) -> Self {
        Self {
            nrows: size,
            ncols: size,
            row_indices: Vec::with_capacity(size * 6),
            col_indices: Vec::with_capacity(size * 6),
            values: Vec::with_capacity(size * 6),
        }
    }

    /// Add a triplet (row, col, value)
    #[inline]
    pub fn push(&mut self, row: usize, col: usize, value: Value) {
        self.row_indices.push(row);
        self.col_indices.push(col);
        self.values.push(value);
    }

    /// Clear all entries for reuse
    pub fn clear(&mut self) {
        self.row_indices.clear();
        self.col_indices.clear();
        self.values.clear();
    }

    /// Number of entries
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Convert to StaticMatrix (freezes structure)
    pub fn to_static(&self) -> Result<StaticMatrix, SolverError> {
        let triplets: Vec<_> = self
            .row_indices
            .iter()
            .zip(self.col_indices.iter())
            .zip(self.values.iter())
            .map(|((&r, &c), &v)| (r, c, v))
            .collect();

        StaticMatrix::from_triplets(self.nrows, self.ncols, &triplets)
    }

    /// Convert to faer sparse column matrix (legacy path)
    pub fn to_sparse_col_mat(&self) -> Result<SparseColMat<usize, Value>, SolverError> {
        self.to_static().map(|s| s.to_sparse_col_mat())
    }
}

//=============================================================================
// Sparse LU Solver (high-level API)
//=============================================================================

/// High-performance sparse LU solver using faer
pub struct SparseLuSolver {
    /// Cached symbolic LU factorization for reuse when structure doesn't change
    symbolic_lu: Option<SymbolicLu<usize>>,
}

impl SparseLuSolver {
    pub fn new() -> Self {
        Self { symbolic_lu: None }
    }

    /// Solve Ax = b using sparse LU decomposition
    pub fn solve(
        &mut self,
        matrix: &SparseColMat<usize, Value>,
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        let n = matrix.nrows();

        if n == 0 {
            return Ok(Vec::new());
        }

        if n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {} doesn't match RHS size {}",
                n,
                rhs.len()
            )));
        }

        // Create symbolic factorization if not cached
        let symbolic = match &self.symbolic_lu {
            Some(s) => s.clone(),
            None => {
                let s = SymbolicLu::try_new(matrix.symbolic())
                    .map_err(|_| SolverError::SingularMatrix)?;
                self.symbolic_lu = Some(s.clone());
                s
            }
        };

        // Perform LU decomposition with the symbolic structure
        let lu = Lu::try_new_with_symbolic(symbolic, matrix.as_ref())
            .map_err(|_| SolverError::SingularMatrix)?;

        // Create RHS as column vector
        let mut b = Mat::<Value>::zeros(n, 1);
        for (i, &val) in rhs.iter().enumerate() {
            b[(i, 0)] = val;
        }

        // Solve in-place
        lu.solve_in_place(b.as_mut());

        // Extract solution
        Ok((0..n).map(|i| b[(i, 0)]).collect())
    }

    /// Clear cached symbolic factorization (call when matrix structure changes)
    pub fn clear_cache(&mut self) {
        self.symbolic_lu = None;
    }
}

impl Default for SparseLuSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Solve a sparse system Ax = b (convenience function)
pub fn solve_sparse(triplets: &TripletMatrix, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
    let sparse_mat = triplets.to_sparse_col_mat()?;
    let mut solver = SparseLuSolver::new();
    solver.solve(&sparse_mat, rhs)
}

/// Simple Gaussian elimination for small systems or fallback
pub fn solve_gauss(mut a: Vec<Vec<Value>>, mut b: Vec<Value>) -> Result<Vec<Value>, SolverError> {
    let n = b.len();

    if n == 0 {
        return Ok(Vec::new());
    }

    // Forward elimination with partial pivoting
    for k in 0..n {
        // Find pivot
        let mut max_row = k;
        let mut max_val = a[k][k].abs();

        for i in (k + 1)..n {
            if a[i][k].abs() > max_val {
                max_val = a[i][k].abs();
                max_row = i;
            }
        }

        if max_val < 1e-15 {
            return Err(SolverError::SingularMatrix);
        }

        // Swap rows
        if max_row != k {
            a.swap(k, max_row);
            b.swap(k, max_row);
        }

        // Eliminate column
        for i in (k + 1)..n {
            let factor = a[i][k] / a[k][k];
            for j in k..n {
                a[i][j] -= factor * a[k][j];
            }
            b[i] -= factor * b[k];
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplet_matrix() {
        let mut m = TripletMatrix::new(2);
        m.push(0, 0, 2.0);
        m.push(0, 1, 1.0);
        m.push(1, 0, 1.0);
        m.push(1, 1, 3.0);

        assert_eq!(m.nnz(), 4);
    }

    #[test]
    fn test_sparse_solve() {
        // 2x + y = 5
        // x + 3y = 7
        // Solution: x = 1.6, y = 1.8
        let mut m = TripletMatrix::new(2);
        m.nrows = 2;
        m.ncols = 2;
        m.push(0, 0, 2.0);
        m.push(0, 1, 1.0);
        m.push(1, 0, 1.0);
        m.push(1, 1, 3.0);

        let b = vec![5.0, 7.0];
        let x = solve_sparse(&m, &b).unwrap();

        assert!((x[0] - 1.6).abs() < 1e-10);
        assert!((x[1] - 1.8).abs() < 1e-10);
    }

    #[test]
    fn test_duplicate_accumulation() {
        // Add same entry multiple times
        let mut m = TripletMatrix::new(2);
        m.nrows = 2;
        m.ncols = 2;
        m.push(0, 0, 1.0);
        m.push(0, 0, 1.0); // Duplicate - should sum to 2.0
        m.push(0, 1, 1.0);
        m.push(1, 0, 1.0);
        m.push(1, 1, 3.0);

        let b = vec![5.0, 7.0];
        let x = solve_sparse(&m, &b).unwrap();

        // 2x + y = 5, x + 3y = 7 -> x = 1.6, y = 1.8
        assert!((x[0] - 1.6).abs() < 1e-10);
        assert!((x[1] - 1.8).abs() < 1e-10);
    }

    #[test]
    fn test_static_matrix() {
        // Build static matrix once
        let triplets = vec![(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let mut matrix = StaticMatrix::from_triplets(2, 2, &triplets).unwrap();

        // Solve first time
        let x1 = matrix.solve(&[5.0, 7.0]).unwrap();
        assert!((x1[0] - 1.6).abs() < 1e-10);

        // Clear and re-stamp (simulating Newton iteration)
        matrix.clear_values();
        matrix.add(0, 0, 2.0);
        matrix.add(0, 1, 1.0);
        matrix.add(1, 0, 1.0);
        matrix.add(1, 1, 3.0);

        // Solve again - should use cached structure
        let x2 = matrix.solve(&[5.0, 7.0]).unwrap();
        assert!((x2[0] - 1.6).abs() < 1e-10);
    }

    #[test]
    fn test_static_matrix_from_triplets_rejects_out_of_bounds_row() {
        let triplets = vec![(2, 0, 1.0)];
        match StaticMatrix::from_triplets(2, 2, &triplets) {
            Err(SolverError::InvalidCircuit(msg)) => {
                assert!(msg.contains("out of bounds"));
                assert!(msg.contains("(2, 0)"));
            }
            Ok(_) => panic!("expected invalid-circuit error for out-of-bounds row"),
            Err(other) => panic!("unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn test_static_matrix_from_triplets_rejects_out_of_bounds_col() {
        let triplets = vec![(0, 2, 1.0)];
        match StaticMatrix::from_triplets(2, 2, &triplets) {
            Err(SolverError::InvalidCircuit(msg)) => {
                assert!(msg.contains("out of bounds"));
                assert!(msg.contains("(0, 2)"));
            }
            Ok(_) => panic!("expected invalid-circuit error for out-of-bounds col"),
            Err(other) => panic!("unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn test_scaled_residual_inf_norm_exact_solution_is_small() {
        let triplets = vec![(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let matrix = StaticMatrix::from_triplets(2, 2, &triplets).unwrap();
        let rhs = vec![5.0, 7.0];
        let x = vec![1.6, 1.8];
        let norm = matrix
            .scaled_residual_inf_norm(&x, &rhs, 1e-9, 1e-3)
            .unwrap();
        assert!(norm < 1e-10);
    }

    #[test]
    fn test_scaled_residual_inf_norm_detects_off_solution() {
        let triplets = vec![(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let matrix = StaticMatrix::from_triplets(2, 2, &triplets).unwrap();
        let rhs = vec![5.0, 7.0];
        let x = vec![1.2, 2.4];
        let norm = matrix
            .scaled_residual_inf_norm(&x, &rhs, 1e-9, 1e-3)
            .unwrap();
        assert!(norm > 1.0);
    }

    #[test]
    fn test_scaled_residual_inf_norm_rejects_size_mismatch() {
        let triplets = vec![(0, 0, 1.0)];
        let matrix = StaticMatrix::from_triplets(1, 1, &triplets).unwrap();
        let err = matrix
            .scaled_residual_inf_norm(&[1.0, 2.0], &[1.0], 1e-9, 1e-3)
            .unwrap_err();
        match err {
            SolverError::InvalidCircuit(msg) => {
                assert!(msg.contains("solution"));
            }
            other => panic!("unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn test_solve_gauss() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 7.0];

        let x = solve_gauss(a, b).unwrap();

        assert!((x[0] - 1.6).abs() < 1e-10);
        assert!((x[1] - 1.8).abs() < 1e-10);
    }
}
