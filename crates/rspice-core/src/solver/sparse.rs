//! High-performance sparse matrix solver using faer
//!
//! Uses faer's sparse LU decomposition for circuit simulation.
//! Provides O(n) scaling for typical circuit matrices.
//!
//! Key optimization: Static structure matrix that caches topology
//! and allows updates to values only, avoiding O(N log N) rebuild.

#![allow(clippy::needless_range_loop)]
use super::SolverError;
use crate::Value;
use faer::dyn_stack::{MemBuffer, MemStack};
use faer::linalg::solvers::Solve;
use faer::sparse::linalg::lu as sparse_lu;
use faer::sparse::linalg::solvers::{Lu, SymbolicLu};
use faer::sparse::{SparseColMat, SparseColMatRef, SymbolicSparseColMat};
use faer::{Conj, Mat, get_global_parallelism};
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// Reusable sparse-LU workspace for the Newton hot path.
///
/// The symbolic analysis is computed once for the frozen sparsity pattern;
/// the numeric factorization, scratch arena, and RHS buffer are reused so
/// repeated solves perform no per-iteration allocations. The symbolic part is
/// `Arc`-shared because LU symbolic analysis depends only on the pattern, not
/// the scalar type — AC's complex matrices reuse it as-is.
struct LuWorkspace {
    symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    numeric: sparse_lu::NumericLu<usize, Value>,
    factor_mem: MemBuffer,
    solve_mem: MemBuffer,
    rhs: Mat<Value>,
    /// Numerically equilibrated CSC values for faer's factorization. Circuit
    /// matrices routinely mix conductances, ideal-source rows, and high-gain
    /// controlled sources across many orders of magnitude; factoring the raw
    /// values can lose the pivot quality required by Newton iteration.
    scaled_values: Vec<Value>,
    scaled_rhs: Vec<Value>,
    row_scale: Vec<Value>,
    col_scale: Vec<Value>,
    max_row_nnz: usize,
}

//=============================================================================
// Static Structure Matrix - The Key Optimization
//=============================================================================

/// Pre-computed stamp location that maps directly to CSC values array
#[derive(Debug, Clone, Copy)]
pub struct CscIndex(pub usize);

#[derive(Debug, Clone, Copy)]
struct MissingMatrixPosition {
    method: &'static str,
    row: usize,
    col: usize,
}

impl MissingMatrixPosition {
    fn into_solver_error(self) -> SolverError {
        SolverError::InvalidCircuit(format!(
            "{} missing matrix position ({}, {})",
            self.method, self.row, self.col
        ))
    }
}

/// Pre-built matrix structure with static topology
///
/// This is the critical optimization: we build the structure once during
/// circuit setup, then only update the values during Newton-Raphson iterations.
/// This avoids the O(N log N) sort and memory allocation on every solve.
pub struct StaticMatrix {
    /// Matrix dimensions
    pub nrows: usize,
    pub ncols: usize,
    /// Frozen CSC sparsity pattern, validated once at construction.
    /// Solves borrow it as a view — no per-iteration structure copies.
    csc: SymbolicSparseColMat<usize>,
    /// CSC values (mutable - updated each iteration)
    values: Vec<Value>,
    /// Mapping from (row, col) to index in values array
    /// This enables O(1) stamping during simulation. FxHash: stamp lookups
    /// are integer pairs on the Newton hot path, and the map is never
    /// iterated, so hasher choice is invisible beyond speed.
    position_map: FxHashMap<(usize, usize), usize>,
    /// Reusable LU workspace (lazily initialized on first solve)
    lu: Option<LuWorkspace>,
    /// Default KLU-class real backend: refactors the frozen pattern with a
    /// stored pivot sequence instead of fully re-pivoting every Newton
    /// iteration. Lazily initialized; any failure falls back to the faer path.
    klu: Option<crate::solver::klu::KluSolver>,
    /// Scratch values + RHS retained between residual probes (see
    /// [`StaticMatrix::with_probe_values`]).
    probe_values: Option<Vec<Value>>,
    probe_rhs: Option<Vec<Value>>,
    /// Scratch for the A*x product inside residual norms.
    residual_scratch: Vec<Value>,
    residual_gross_scratch: Vec<Value>,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MissingMatrixPosition>,
}

#[cold]
#[inline(never)]
fn missing_matrix_position(method: &'static str, row: usize, col: usize) -> MissingMatrixPosition {
    MissingMatrixPosition { method, row, col }
}

/// Whether the KLU-class backend handles real solves for this process.
///
/// Default ON: the full ngspice conformance run under the backend
/// reproduces the faer baseline failure set exactly, and the benchmark
/// scoreboard shows 14-15% end-to-end improvement on solver-bound decks
/// (benchmarks/scoreboards/2026-06-11-faer-vs-klu-*.json).
/// `RSPICE_SOLVER=faer` opts out.
pub(crate) fn klu_backend_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        !std::env::var("RSPICE_SOLVER").is_ok_and(|v| v.eq_ignore_ascii_case("faer"))
    })
}

#[inline]
fn finite_solution_or_singular(solution: Vec<Value>) -> Result<Vec<Value>, SolverError> {
    if solution.iter().all(|value| value.is_finite()) {
        Ok(solution)
    } else {
        Err(SolverError::SingularMatrix)
    }
}

#[inline]
fn finite_reciprocal_scale(max_abs: Value) -> Value {
    debug_assert!(max_abs.is_finite() && max_abs >= 0.0);
    let small_number = Value::MIN_POSITIVE / Value::EPSILON;
    let large_number = 1.0 / small_number;
    (1.0 / max_abs).clamp(small_number, large_number)
}

/// Build `D_r * A * D_c` and the matching `D_r * b` transforms for faer.
///
/// Column max scaling followed by row max scaling mirrors the equilibration
/// used by production SPICE sparse solvers. The original matrix remains
/// untouched for stamping and residual evaluation; after solving for `y`, the
/// caller recovers the original-coordinate solution as `x = D_c * y`.
fn equilibrate_sparse_system(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Value],
    rhs: &[Value],
    scaled_values: &mut Vec<Value>,
    row_scale: &mut Vec<Value>,
    col_scale: &mut Vec<Value>,
) -> Result<(), SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Sparse equilibration dimension mismatch".to_string(),
        ));
    }

    scaled_values.resize(values.len(), 0.0);
    row_scale.resize(nrows, 1.0);
    col_scale.resize(ncols, 1.0);

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let mut max_abs: Value = 0.0;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            if !value.is_finite() {
                return Err(SolverError::Overflow);
            }
            max_abs = max_abs.max(value.abs());
        }
        if max_abs == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        let scale = finite_reciprocal_scale(max_abs);
        col_scale[col] = scale;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] = values[idx] * scale;
        }
    }

    row_scale.fill(0.0);
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let row = row_idx[idx];
            row_scale[row] = row_scale[row].max(scaled_values[idx].abs());
        }
    }
    for row in 0..nrows {
        let rhs_value = rhs[row];
        if !rhs_value.is_finite() {
            return Err(SolverError::Overflow);
        }
        if row_scale[row] == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        row_scale[row] = finite_reciprocal_scale(row_scale[row]);
        if !(rhs_value * row_scale[row]).is_finite() {
            return Err(SolverError::Overflow);
        }
    }
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] *= row_scale[row_idx[idx]];
        }
    }
    if scaled_values.iter().any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow);
    }

    Ok(())
}

#[inline]
fn faer_backward_error_tolerance(max_row_nnz: usize) -> Value {
    64.0 * Value::EPSILON * (max_row_nnz.saturating_add(1) as Value)
}

/// Compute componentwise backward error in the supplied coordinates.
///
/// The denominator `|b| + |A| |x|` makes the check unit- and scale-invariant.
/// Consequently, checking `D_r*A*D_c`, `y`, and `D_r*b` is mathematically
/// identical to checking the original system with `x = D_c*y`, while avoiding
/// overflow when valid original-coordinate terms approach `f64::MAX`.
/// The safe terms follow LAPACK's treatment of rows whose denominator is near
/// underflow. `residual` is retained as `b - A*x` for iterative refinement.
fn componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    max_row_nnz: usize,
) -> Result<Value, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || solution.len() != ncols || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Sparse backward-error dimension mismatch".to_string(),
        ));
    }

    residual.resize(nrows, 0.0);
    denominator.resize(nrows, 0.0);
    for row in 0..nrows {
        if !rhs[row].is_finite() {
            return Err(SolverError::Overflow);
        }
        residual[row] = rhs[row];
        denominator[row] = rhs[row].abs();
    }

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let x = solution[col];
        if !x.is_finite() {
            return Err(SolverError::Overflow);
        }
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            let term = value * x;
            let magnitude = value.abs() * x.abs();
            if !term.is_finite() || !magnitude.is_finite() {
                return Err(SolverError::Overflow);
            }
            let row = row_idx[idx];
            residual[row] -= term;
            denominator[row] = (denominator[row] + magnitude).min(Value::MAX);
        }
    }

    let safe1 = (max_row_nnz.saturating_add(1) as Value) * Value::MIN_POSITIVE;
    let safe2 = safe1 / Value::EPSILON;
    let mut error: Value = 0.0;
    for row in 0..nrows {
        let residual_abs = residual[row].abs();
        let scale = denominator[row];
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let row_error = if residual_abs == 0.0 {
            0.0
        } else if scale > safe2 {
            residual_abs / scale
        } else {
            (residual_abs + safe1) / (scale + safe1)
        };
        error = error.max(row_error);
    }
    Ok(error)
}

impl StaticMatrix {
    /// Create a zero-valued matrix with the same sparsity structure.
    ///
    /// This is used for residual probes that must stamp into an independent
    /// numeric workspace while preserving O(1) matrix-entry lookup.
    pub fn clone_structure(&self) -> Self {
        Self {
            nrows: self.nrows,
            ncols: self.ncols,
            csc: self.csc.clone(),
            values: vec![0.0; self.values.len()],
            position_map: self.position_map.clone(),
            lu: None,
            klu: None,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            stamping_error: None,
        }
    }

    /// Run `f` against this matrix with zeroed scratch values and RHS swapped
    /// in, restoring the live values afterwards.
    ///
    /// Residual probes need to stamp a trial linearization without disturbing
    /// the in-flight Newton system. Since the structure, position map, and LU
    /// workspace are all valid for any values array, swapping the values
    /// buffer gives the probe a free matrix: no structure clones, no
    /// position-map rehash, and probe solves reuse the cached symbolic
    /// analysis and numeric workspace. (A subsequent live solve refactorizes
    /// from the restored values, so workspace sharing is safe.)
    pub fn with_probe_values<R>(&mut self, f: impl FnOnce(&mut Self, &mut [Value]) -> R) -> R {
        let mut scratch = self.probe_values.take().unwrap_or_default();
        scratch.resize(self.values.len(), 0.0);
        scratch.fill(0.0);
        let mut rhs = self.probe_rhs.take().unwrap_or_default();
        rhs.resize(self.nrows, 0.0);
        rhs.fill(0.0);

        std::mem::swap(&mut self.values, &mut scratch);
        let result = f(self, &mut rhs);
        std::mem::swap(&mut self.values, &mut scratch);

        self.probe_values = Some(scratch);
        self.probe_rhs = Some(rhs);
        result
    }

    fn to_dense_real(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx];
            }
        }
        dense
    }

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
        let mut position_map = FxHashMap::default();

        for (r, c, v) in entries {
            if let Some(last) = accumulated.last_mut()
                && last.0 == r
                && last.1 == c
            {
                last.2 += v;
                continue;
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

        // Validate the pattern once here; every solve afterwards borrows it
        // without re-checking.
        let csc = SymbolicSparseColMat::new_checked(nrows, ncols, col_ptrs, None, row_indices);

        Ok(Self {
            nrows,
            ncols,
            csc,
            values,
            position_map,
            lu: None,
            klu: None,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            stamping_error: None,
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
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                self.record_missing_position("StaticMatrix::add", row, col);
                return;
            }
        };
        self.values[idx] += value;
    }

    /// Checked add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add(&mut self, row: usize, col: usize, value: Value) -> Result<(), SolverError> {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                let missing = missing_matrix_position("StaticMatrix::try_add", row, col);
                self.stamping_error.get_or_insert(missing);
                return Err(missing.into_solver_error());
            }
        };
        self.values[idx] += value;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error
            .get_or_insert_with(|| missing_matrix_position(method, row, col));
    }

    #[inline]
    fn check_stamping_error(&self) -> Result<(), SolverError> {
        match self.stamping_error {
            Some(error) => Err(error.into_solver_error()),
            None => Ok(()),
        }
    }

    /// Rows whose entries are all exactly zero (or absent): the immediate
    /// structural suspects when factorization reports a singular system.
    pub fn deficient_rows(&self) -> Vec<usize> {
        let mut row_max = vec![0.0f64; self.nrows];
        for (&(row, _col), &idx) in &self.position_map {
            let magnitude = self.values[idx].abs();
            if magnitude > row_max[row] {
                row_max[row] = magnitude;
            }
        }
        row_max
            .iter()
            .enumerate()
            .filter(|entry| *entry.1 == 0.0)
            .map(|(row, _)| row)
            .collect()
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

    /// Replace one existing row with an identity constraint.
    ///
    /// The sparsity pattern remains frozen; callers use this for temporary
    /// operating-point constraints such as `.NODESET` startup solves.
    pub fn force_identity_row(&mut self, row: usize) -> Result<(), SolverError> {
        if row >= self.nrows || row >= self.ncols {
            return Err(SolverError::InvalidCircuit(format!(
                "identity row {} outside {}x{} matrix",
                row, self.nrows, self.ncols
            )));
        }

        let mut row_entries = Vec::new();
        for (&(entry_row, entry_col), &idx) in &self.position_map {
            if entry_row == row {
                row_entries.push((entry_col, idx));
            }
        }

        let mut diagonal_found = false;
        for (col, idx) in row_entries {
            let value = if col == row {
                diagonal_found = true;
                1.0
            } else {
                0.0
            };
            self.values[idx] = value;
        }

        if diagonal_found {
            Ok(())
        } else {
            self.record_missing_position("StaticMatrix::force_identity_row", row, row);
            Err(SolverError::InvalidCircuit(format!(
                "identity row {} has no diagonal matrix position",
                row
            )))
        }
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
    ///
    /// Takes `&mut self` to reuse the internal A*x scratch buffer; this is
    /// evaluated once or twice per Newton iteration (merit + convergence).
    pub fn scaled_residual_inf_norm(
        &mut self,
        solution: &[Value],
        rhs: &[Value],
        abstol: Value,
        reltol: Value,
    ) -> Result<Value, SolverError> {
        self.scaled_residual_inf_norm_by_row(solution, rhs, reltol, |_| abstol)
    }

    /// Compute infinity norm of the scaled residual `A*x-b` with row-specific
    /// absolute tolerances.
    pub fn scaled_residual_inf_norm_by_row<F>(
        &mut self,
        solution: &[Value],
        rhs: &[Value],
        reltol: Value,
        mut row_abstol: F,
    ) -> Result<Value, SolverError>
    where
        F: FnMut(usize) -> Value,
    {
        self.check_stamping_error()?;
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

        let safe_reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };

        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        self.residual_scratch.resize(self.nrows, 0.0);
        self.residual_scratch.fill(0.0);
        self.residual_gross_scratch.resize(self.nrows, 0.0);
        self.residual_gross_scratch.fill(0.0);
        let (ax, ax_gross) = (&mut self.residual_scratch, &mut self.residual_gross_scratch);
        for col in 0..self.ncols {
            let x = solution[col];
            if !x.is_finite() {
                return Ok(Value::INFINITY);
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                let term = self.values[idx] * x;
                ax[row] += term;
                ax_gross[row] += term.abs();
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
            let abstol = row_abstol(row);
            let safe_abstol = if abstol.is_finite() && abstol > 0.0 {
                abstol
            } else {
                1e-12
            };
            // The row scale is the NET magnitude max(|Σa_ij·x_j|, |b_i|)
            // plus an explicit floating-point cancellation floor on the
            // GROSS term magnitude Σ|a_ij·x_j|. At a converged KCL row the
            // net cancels to ~0 and the residual that remains is the
            // summation noise of the row's own mA-scale currents — bounded
            // by O(n·ε)·gross — so the floor accepts it without bare-abstol
            // rejections (the 4096-MOSFET array artifact). Scaling by the
            // gross magnitude itself (full Oettli–Prager) is too loose as a
            // NONLINEAR acceptance criterion: on high-gain summing rows with
            // large balanced flows it hides genuine disequilibrium that is a
            // tiny fraction of the gross (a feedback amplifier accepts a
            // wrong-basin operating point), so the relative part stays on
            // the net.
            const CANCELLATION_NOISE_TERMS: Value = 256.0;
            let noise_floor = CANCELLATION_NOISE_TERMS * Value::EPSILON * ax_gross[row];
            let scale = safe_abstol + noise_floor + safe_reltol * row_ax.abs().max(row_rhs.abs());
            let normalized = residual / scale.max(safe_abstol);
            residual_inf = residual_inf.max(normalized);
        }

        Ok(residual_inf)
    }

    /// Compute raw residual vector `A*x - b`.
    pub fn residual_vector(
        &self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        if solution.len() != self.ncols || rhs.len() != self.nrows {
            return Err(SolverError::InvalidCircuit(
                "Residual vector size mismatch".to_string(),
            ));
        }

        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut ax = vec![0.0; self.nrows];
        for col in 0..self.ncols {
            let x = solution[col];
            if x == 0.0 {
                continue;
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                ax[row] += self.values[idx] * x;
            }
        }

        for row in 0..self.nrows {
            ax[row] -= rhs[row];
        }

        Ok(ax)
    }

    /// Convert to an owned faer SparseColMat (legacy/test path; copies)
    fn to_sparse_col_mat(&self) -> SparseColMat<usize, Value> {
        SparseColMat::new(self.csc.clone(), self.values.clone())
    }

    /// Initialize the reusable LU workspace if it does not exist yet.
    fn ensure_lu_workspace(&mut self) -> Result<(), SolverError> {
        if self.lu.is_some() {
            return Ok(());
        }
        let par = get_global_parallelism();
        let symbolic = sparse_lu::factorize_symbolic_lu(self.csc.as_ref(), Default::default())
            .map_err(|_| SolverError::SingularMatrix)?;
        let factor_mem = MemBuffer::try_new(
            symbolic.factorize_numeric_lu_scratch::<Value>(par, Default::default()),
        )
        .map_err(|_| SolverError::SingularMatrix)?;
        let solve_mem = MemBuffer::try_new(symbolic.solve_in_place_scratch::<Value>(1, par))
            .map_err(|_| SolverError::SingularMatrix)?;
        let mut row_nnz = vec![0_usize; self.nrows];
        for &row in self.csc.row_idx() {
            row_nnz[row] = row_nnz[row].saturating_add(1);
        }
        let max_row_nnz = row_nnz.into_iter().max().unwrap_or(0);
        self.lu = Some(LuWorkspace {
            symbolic: Arc::new(symbolic),
            numeric: sparse_lu::NumericLu::new(),
            factor_mem,
            solve_mem,
            rhs: Mat::zeros(self.nrows, 1),
            scaled_values: Vec::new(),
            scaled_rhs: Vec::new(),
            row_scale: Vec::new(),
            col_scale: Vec::new(),
            max_row_nnz,
        });
        Ok(())
    }

    /// Solve Ax = b using cached structure.
    ///
    /// Hot path: borrows the frozen pattern and live values as a view and
    /// refactorizes into a persistent numeric workspace — no structure
    /// copies and no allocations after the first call.
    pub fn solve(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;

        if n != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {}x{} doesn't match RHS size {}",
                n,
                self.ncols,
                rhs.len()
            )));
        }

        if klu_backend_enabled()
            && let Some(result) = self.try_solve_klu(rhs)
        {
            return finite_solution_or_singular(result);
        }

        self.solve_faer(rhs)
    }

    /// Solve through faer's sparse LU in equilibrated coordinates.
    ///
    /// Kept separate from backend selection so solver tests can exercise this
    /// path without mutating the process-wide `RSPICE_SOLVER` policy.
    fn solve_faer(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        self.ensure_lu_workspace()?;

        let par = get_global_parallelism();
        let Self {
            csc,
            values,
            lu,
            residual_scratch,
            residual_gross_scratch,
            ..
        } = self;
        let ws = lu.as_mut().expect("LU workspace initialized above");

        equilibrate_sparse_system(
            csc,
            values,
            rhs,
            &mut ws.scaled_values,
            &mut ws.row_scale,
            &mut ws.col_scale,
        )?;

        let mat = SparseColMatRef::new(csc.as_ref(), ws.scaled_values.as_slice());

        let lu_ref = ws
            .symbolic
            .factorize_numeric_lu(
                &mut ws.numeric,
                mat,
                par,
                MemStack::new(&mut ws.factor_mem),
                Default::default(),
            )
            .map_err(|_| SolverError::SingularMatrix)?;

        ws.scaled_rhs.resize(rhs.len(), 0.0);
        for ((scaled_rhs, &rhs_value), &row_scale) in
            ws.scaled_rhs.iter_mut().zip(rhs).zip(&ws.row_scale)
        {
            *scaled_rhs = rhs_value * row_scale;
        }
        ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.scaled_rhs);
        lu_ref.solve_in_place_with_conj(
            Conj::No,
            ws.rhs.as_mut(),
            par,
            MemStack::new(&mut ws.solve_mem),
        );

        let mut scaled_solution = ws.rhs.col_as_slice(0).to_vec();
        if scaled_solution.iter().any(|value| !value.is_finite()) {
            return Err(SolverError::SingularMatrix);
        }

        let target_error = faer_backward_error_tolerance(ws.max_row_nnz);
        let mut backward_error = componentwise_backward_error(
            csc,
            &ws.scaled_values,
            &scaled_solution,
            &ws.scaled_rhs,
            residual_scratch,
            residual_gross_scratch,
            ws.max_row_nnz,
        )?;
        if backward_error <= target_error {
            for (value, &col_scale) in scaled_solution.iter_mut().zip(&ws.col_scale) {
                *value *= col_scale;
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
            }
            return Ok(scaled_solution);
        }

        const MAX_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_REFINEMENTS {
            for (scaled_residual, &residual) in ws
                .rhs
                .col_as_slice_mut(0)
                .iter_mut()
                .zip(residual_scratch.iter())
            {
                *scaled_residual = residual;
            }
            lu_ref.solve_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.solve_mem),
            );

            for (value, &scaled_correction) in
                scaled_solution.iter_mut().zip(ws.rhs.col_as_slice(0))
            {
                let refined = *value + scaled_correction;
                if !scaled_correction.is_finite() || !refined.is_finite() {
                    return Err(SolverError::Overflow);
                }
                *value = refined;
            }

            let refined_error = componentwise_backward_error(
                csc,
                &ws.scaled_values,
                &scaled_solution,
                &ws.scaled_rhs,
                residual_scratch,
                residual_gross_scratch,
                ws.max_row_nnz,
            )?;
            if refined_error <= target_error {
                for (value, &col_scale) in scaled_solution.iter_mut().zip(&ws.col_scale) {
                    *value *= col_scale;
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(scaled_solution);
            }
            if refined_error >= backward_error * MIN_IMPROVEMENT_FACTOR {
                backward_error = refined_error;
                break;
            }
            backward_error = refined_error;
        }

        Err(SolverError::InaccurateSolution(backward_error))
    }

    /// Default KLU-class real solve: values-only
    /// refactorization over the frozen pattern with a stored pivot
    /// sequence; full re-pivoting only on a growth alarm. Returns `None`
    /// on any backend failure so the caller falls through to faer —
    /// backend fallback can degrade performance but never a result.
    fn try_solve_klu(&mut self, rhs: &[Value]) -> Option<Vec<Value>> {
        let Self {
            nrows,
            csc,
            values,
            klu,
            ..
        } = self;
        let n = *nrows;
        let col_ptr = csc.col_ptr();
        let row_idx = csc.row_idx();

        let backend = klu.get_or_insert_with(crate::solver::klu::KluSolver::new);
        if !backend.is_analyzed_for(n) {
            backend.analyze(n, col_ptr, row_idx);
        }
        let factored = match backend.refactor(col_ptr, row_idx, values) {
            Ok(()) => true,
            Err(SolverError::PivotGrowth) => backend.factor(col_ptr, row_idx, values).is_ok(),
            Err(_) => backend.factor(col_ptr, row_idx, values).is_ok(),
        };
        if !factored {
            static FALLBACK_LOGGED: std::sync::Once = std::sync::Once::new();
            FALLBACK_LOGGED.call_once(|| {
                log::warn!("klu backend could not factor this system; using faer fallback");
            });
            return None;
        }
        let mut out = Vec::new();
        backend.solve(rhs, &mut out).ok()?;
        Some(out)
    }

    /// Solve Ax = b via dense Gaussian elimination.
    ///
    /// This is used as a high-stability fallback for small linear systems with
    /// strong transformer/coupling fill-in where sparse LU can become noisy.
    pub fn solve_dense(&self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Dense solve requires a square matrix matching RHS size, got {}x{} with RHS {}",
                self.nrows,
                self.ncols,
                rhs.len()
            )));
        }

        solve_gauss(self.to_dense_real(), rhs.to_vec())
    }
}

//=============================================================================
// Complex Matrix for AC Analysis
//=============================================================================

use num_complex::Complex64;

/// Reusable complex sparse-LU workspace (see [`LuWorkspace`]).
struct ComplexLuWorkspace {
    symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    numeric: sparse_lu::NumericLu<usize, Complex64>,
    factor_mem: MemBuffer,
    solve_mem: MemBuffer,
    rhs: Mat<Complex64>,
}

/// ComplexMatrix for AC small-signal analysis
///
/// Shares the same sparsity structure as a StaticMatrix but uses Complex64 values.
/// This enables AC analysis at different frequencies without rebuilding topology.
pub struct ComplexMatrix {
    /// Matrix dimensions
    pub nrows: usize,
    pub ncols: usize,
    /// Frozen CSC sparsity pattern (cloned from the real matrix once)
    csc: SymbolicSparseColMat<usize>,
    /// Complex values (updated for each frequency)
    values: Vec<Complex64>,
    /// Mapping from (row, col) to index in values array
    position_map: FxHashMap<(usize, usize), usize>,
    /// Reusable LU workspace; the symbolic part is shared with the real
    /// matrix when available (symbolic LU is scalar-type independent).
    lu: Option<ComplexLuWorkspace>,
    /// True while `lu.numeric` holds the factorization of the current
    /// `values`. Consecutive solves against unchanged values (noise analysis
    /// solves one matrix against many excitation vectors) skip refactorizing.
    factorization_valid: bool,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MissingMatrixPosition>,
}

impl ComplexMatrix {
    /// Create a ComplexMatrix with the same structure as a StaticMatrix
    pub fn from_real_structure(real_matrix: &StaticMatrix) -> Self {
        let nnz = real_matrix.values.len();
        let mut this = Self {
            nrows: real_matrix.nrows,
            ncols: real_matrix.ncols,
            csc: real_matrix.csc.clone(),
            values: vec![Complex64::new(0.0, 0.0); nnz],
            position_map: real_matrix.position_map.clone(),
            lu: None,
            factorization_valid: false,
            stamping_error: None,
        };
        // Reuse the real matrix's symbolic analysis when it has already been
        // computed (any DC solve does so); AC sweeps then never repeat it.
        if let Some(symbolic) = real_matrix.lu.as_ref().map(|ws| ws.symbolic.clone()) {
            this.lu = Self::workspace_from_symbolic(this.nrows, symbolic).ok();
        }
        this
    }

    fn workspace_from_symbolic(
        nrows: usize,
        symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    ) -> Result<ComplexLuWorkspace, SolverError> {
        let par = get_global_parallelism();
        let factor_mem = MemBuffer::try_new(
            symbolic.factorize_numeric_lu_scratch::<Complex64>(par, Default::default()),
        )
        .map_err(|_| SolverError::SingularMatrix)?;
        let solve_mem = MemBuffer::try_new(symbolic.solve_in_place_scratch::<Complex64>(1, par))
            .map_err(|_| SolverError::SingularMatrix)?;
        Ok(ComplexLuWorkspace {
            symbolic,
            numeric: sparse_lu::NumericLu::new(),
            factor_mem,
            solve_mem,
            rhs: Mat::zeros(nrows, 1),
        })
    }

    /// Zero all values
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.fill(Complex64::new(0.0, 0.0));
        self.factorization_valid = false;
    }

    /// Add real value at (row, col)
    #[inline]
    pub fn add_real(&mut self, row: usize, col: usize, value: Value) {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add_real", row, col);
                return;
            }
        };
        self.values[idx] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Add complex value at (row, col)
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Complex64) {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add", row, col);
                return;
            }
        };
        self.values[idx] += value;
        self.factorization_valid = false;
    }

    /// Add imaginary value (for frequency-dependent terms like jwC)
    #[inline]
    pub fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add_imag", row, col);
                return;
            }
        };
        self.values[idx] += Complex64::new(0.0, value);
        self.factorization_valid = false;
    }

    /// Direct real add using a precomputed CSC index.
    #[inline]
    pub fn stamp_direct_real(&mut self, idx: CscIndex, value: Value) {
        self.values[idx.0] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Direct imaginary add using a precomputed CSC index.
    #[inline]
    pub fn stamp_direct_imag(&mut self, idx: CscIndex, value: Value) {
        self.values[idx.0] += Complex64::new(0.0, value);
        self.factorization_valid = false;
    }

    /// Checked real add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add_real(
        &mut self,
        row: usize,
        col: usize,
        value: Value,
    ) -> Result<(), SolverError> {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add_real", row, col);
                self.stamping_error.get_or_insert(missing);
                return Err(missing.into_solver_error());
            }
        };
        self.values[idx] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
        Ok(())
    }

    /// Checked complex add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add(&mut self, row: usize, col: usize, value: Complex64) -> Result<(), SolverError> {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add", row, col);
                self.stamping_error.get_or_insert(missing);
                return Err(missing.into_solver_error());
            }
        };
        self.values[idx] += value;
        self.factorization_valid = false;
        Ok(())
    }

    /// Checked imaginary add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add_imag(
        &mut self,
        row: usize,
        col: usize,
        value: Value,
    ) -> Result<(), SolverError> {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add_imag", row, col);
                self.stamping_error.get_or_insert(missing);
                return Err(missing.into_solver_error());
            }
        };
        self.values[idx] += Complex64::new(0.0, value);
        self.factorization_valid = false;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error
            .get_or_insert_with(|| missing_matrix_position(method, row, col));
    }

    #[inline]
    fn check_stamping_error(&self) -> Result<(), SolverError> {
        match self.stamping_error {
            Some(error) => Err(error.into_solver_error()),
            None => Ok(()),
        }
    }

    /// Materialize the real part of the sparse matrix as a dense matrix.
    pub fn to_dense_real(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx].re;
            }
        }
        dense
    }

    /// Materialize the imaginary part of the sparse matrix as a dense matrix.
    pub fn to_dense_imag(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx].im;
            }
        }
        dense
    }

    /// Multiply the current sparse matrix by a complex vector without
    /// factorizing it.
    ///
    /// Distortion and sensitivity analyses use this to contract directional
    /// derivatives of the small-signal MNA operator while retaining the
    /// circuit's sparse structure.
    pub(crate) fn multiply_vector(
        &self,
        vector: &[Complex64],
    ) -> Result<Vec<Complex64>, SolverError> {
        self.check_stamping_error()?;
        if self.ncols != vector.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix column count {} doesn't match vector length {}",
                self.ncols,
                vector.len()
            )));
        }

        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut product = vec![Complex64::new(0.0, 0.0); self.nrows];
        for col in 0..self.ncols {
            let input = vector[col];
            if input == Complex64::new(0.0, 0.0) {
                continue;
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                product[row_idx[idx]] += self.values[idx] * input;
            }
        }
        Ok(product)
    }

    /// Solve Ax = b for complex values.
    ///
    /// The symbolic analysis is computed once per structure (or inherited
    /// from the real matrix) and the numeric factorization is reused across
    /// consecutive solves with unchanged values.
    pub fn solve(&mut self, rhs: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;

        if n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {} doesn't match RHS size {}",
                n,
                rhs.len()
            )));
        }

        if self.lu.is_none() {
            let symbolic = sparse_lu::factorize_symbolic_lu(self.csc.as_ref(), Default::default())
                .map_err(|_| SolverError::SingularMatrix)?;
            self.lu = Some(Self::workspace_from_symbolic(n, Arc::new(symbolic))?);
            self.factorization_valid = false;
        }

        let par = get_global_parallelism();
        let Self {
            csc,
            values,
            lu,
            factorization_valid,
            ..
        } = self;
        let ws = lu.as_mut().expect("LU workspace initialized above");

        if !*factorization_valid {
            let mat = SparseColMatRef::new(csc.as_ref(), values.as_slice());
            ws.symbolic
                .factorize_numeric_lu(
                    &mut ws.numeric,
                    mat,
                    par,
                    MemStack::new(&mut ws.factor_mem),
                    Default::default(),
                )
                .map_err(|_| SolverError::SingularMatrix)?;
            *factorization_valid = true;
        }

        // SAFETY: `ws.numeric` was produced by `ws.symbolic.factorize_numeric_lu`
        // on this matrix's pattern, and `factorization_valid` guarantees the
        // values have not been mutated since (every mutator clears the flag).
        let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };

        ws.rhs.col_as_slice_mut(0).copy_from_slice(rhs);
        lu_ref.solve_in_place_with_conj(
            Conj::No,
            ws.rhs.as_mut(),
            par,
            MemStack::new(&mut ws.solve_mem),
        );

        Ok(ws.rhs.col_as_slice(0).to_vec())
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
        finite_solution_or_singular((0..n).map(|i| b[(i, 0)]).collect())
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

    finite_solution_or_singular(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    fn scaled_tridiagonal_system(
        row_factors: [Value; 5],
        col_factors: [Value; 5],
    ) -> (Vec<(usize, usize, Value)>, Vec<Value>, Vec<Value>) {
        let normalized = [
            [4.0, -1.0, 0.0, 0.0, 0.0],
            [-1.0, 4.0, -1.0, 0.0, 0.0],
            [0.0, -1.0, 4.0, -1.0, 0.0],
            [0.0, 0.0, -1.0, 4.0, -1.0],
            [0.0, 0.0, 0.0, -1.0, 4.0],
        ];
        let solution = col_factors.map(|factor| 1.0 / factor);
        let mut triplets = Vec::new();
        let mut rhs = vec![0.0; 5];
        for row in 0..5 {
            for col in 0..5 {
                if normalized[row][col] == 0.0 {
                    continue;
                }
                let value = row_factors[row] * normalized[row][col] * col_factors[col];
                triplets.push((row, col, value));
                rhs[row] += value * solution[col];
            }
        }
        (triplets, rhs, solution.to_vec())
    }

    fn assert_relative_solution(actual: &[Value], expected: &[Value]) {
        assert_eq!(actual.len(), expected.len());
        for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
            let relative_error =
                (actual - expected).abs() / expected.abs().max(Value::MIN_POSITIVE);
            assert!(
                relative_error <= 1.0e-10,
                "solution[{index}] expected {expected:.17e}, got {actual:.17e} (relative error {relative_error:.3e})"
            );
        }
    }

    #[test]
    fn faer_equilibration_scales_both_axes_and_round_trips() {
        let (triplets, rhs, _) = scaled_tridiagonal_system(
            [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20],
            [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20],
        );
        let matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();
        let mut scaled_values = Vec::new();
        let mut row_scale = Vec::new();
        let mut col_scale = Vec::new();

        equilibrate_sparse_system(
            &matrix.csc,
            &matrix.values,
            &rhs,
            &mut scaled_values,
            &mut row_scale,
            &mut col_scale,
        )
        .unwrap();

        let col_ptr = matrix.csc.col_ptr();
        let row_idx = matrix.csc.row_idx();
        for col in 0..matrix.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let expected = matrix.values[idx] * row_scale[row_idx[idx]] * col_scale[col];
                assert_eq!(scaled_values[idx].to_bits(), expected.to_bits());
            }
        }
        assert!(
            scaled_values
                .iter()
                .all(|value| value.is_finite() && value.abs() <= 1.0)
        );
        assert!(
            rhs.iter()
                .zip(&row_scale)
                .all(|(&value, &scale)| (value * scale).is_finite())
        );
    }

    #[test]
    fn componentwise_backward_error_detects_perturbed_original_solution() {
        let matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)],
        )
        .unwrap();
        let rhs = [6.0, 8.0];
        let mut residual = Vec::new();
        let mut denominator = Vec::new();

        let exact_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &[1.0, 2.0],
            &rhs,
            &mut residual,
            &mut denominator,
            2,
        )
        .unwrap();
        assert_eq!(exact_error.to_bits(), 0.0_f64.to_bits());
        assert_eq!(residual, vec![0.0, 0.0]);

        let perturbed_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &[1.0 + 1.0e-6, 2.0],
            &rhs,
            &mut residual,
            &mut denominator,
            2,
        )
        .unwrap();
        assert!(perturbed_error > faer_backward_error_tolerance(2));

        let zero_row = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let zero_error = componentwise_backward_error(
            &zero_row.csc,
            &zero_row.values,
            &[0.0],
            &[0.0],
            &mut residual,
            &mut denominator,
            1,
        )
        .unwrap();
        assert_eq!(zero_error.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn faer_equilibration_solves_ill_scaled_system_and_recomputes_scales() {
        let row_factors = [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20];
        let col_factors = [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20];
        let (triplets, rhs, expected) = scaled_tridiagonal_system(row_factors, col_factors);
        let mut matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();

        let first = matrix.solve_faer(&rhs).unwrap();
        let repeated = matrix.solve_faer(&rhs).unwrap();
        assert_relative_solution(&first, &expected);
        let mut residual = Vec::new();
        let mut denominator = Vec::new();
        let backward_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &first,
            &rhs,
            &mut residual,
            &mut denominator,
            3,
        )
        .unwrap();
        assert!(backward_error <= faer_backward_error_tolerance(3));
        assert_eq!(
            first
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            repeated
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );

        let (changed_triplets, changed_rhs, changed_expected) =
            scaled_tridiagonal_system(col_factors, row_factors);
        matrix.clear_values();
        for (row, col, value) in changed_triplets {
            matrix.add(row, col, value);
        }
        let changed = matrix.solve_faer(&changed_rhs).unwrap();
        assert_relative_solution(&changed, &changed_expected);
    }

    #[test]
    fn faer_equilibration_rejects_zero_and_nonfinite_numeric_systems() {
        let mut zero_column =
            StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 0.0)]).unwrap();
        assert!(matches!(
            zero_column.solve_faer(&[1.0, 0.0]),
            Err(SolverError::SingularMatrix)
        ));
        let mut zero_row = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 1.0), (1, 1, 1.0)],
        )
        .unwrap();
        assert!(matches!(
            zero_row.solve_faer(&[0.0, 2.0]),
            Err(SolverError::SingularMatrix)
        ));

        let mut nonfinite =
            StaticMatrix::from_triplets(2, 2, &[(0, 0, Value::NAN), (1, 1, 1.0)]).unwrap();
        assert!(matches!(
            nonfinite.solve_faer(&[0.0, 1.0]),
            Err(SolverError::Overflow)
        ));
        let mut finite = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();
        assert!(matches!(
            finite.solve_faer(&[Value::INFINITY, 1.0]),
            Err(SolverError::Overflow)
        ));
    }

    #[test]
    fn faer_equilibration_keeps_extreme_finite_scales_finite() {
        let mut matrix =
            StaticMatrix::from_triplets(2, 2, &[(0, 0, Value::MIN_POSITIVE), (1, 1, Value::MAX)])
                .unwrap();

        let solution = matrix
            .solve_faer(&[Value::MIN_POSITIVE, Value::MAX])
            .unwrap();

        assert_relative_solution(&solution, &[1.0, 1.0]);
    }

    #[test]
    fn static_matrix_missing_stamp_returns_solver_error() {
        let mut matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();

        matrix.add(0, 1, 2.0);
        let message = matrix.solve(&[1.0, 1.0]).unwrap_err().to_string();

        assert!(
            message.contains("missing matrix position")
                && message.contains("StaticMatrix::add")
                && message.contains("(0, 1)"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn static_matrix_singular_solve_rejects_non_finite_solution() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .unwrap();

        matrix.add(0, 0, 1.0);
        matrix.add(0, 1, 1.0);
        matrix.add(1, 0, 2.0);
        matrix.add(1, 1, 2.0);

        let err = matrix.solve(&[1.0, 2.0]).unwrap_err();

        assert!(matches!(err, SolverError::SingularMatrix));
    }

    #[test]
    fn complex_matrix_missing_stamp_returns_solver_error() {
        let real = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);

        matrix.add_real(0, 1, 2.0);
        let message = matrix
            .solve(&[Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)])
            .unwrap_err()
            .to_string();

        assert!(
            message.contains("missing matrix position")
                && message.contains("ComplexMatrix::add_real")
                && message.contains("(0, 1)"),
            "unexpected error: {message}"
        );
    }
}
