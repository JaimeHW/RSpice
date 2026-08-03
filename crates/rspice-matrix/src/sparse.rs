//! High-performance sparse matrix solver using faer
//!
//! Uses faer's sparse LU decomposition for circuit simulation.
//! Provides O(n) scaling for typical circuit matrices.
//!
//! Key optimization: Static structure matrix that caches topology
//! and allows updates to values only, avoiding O(N log N) rebuild.

#![allow(clippy::needless_range_loop)]
use crate::{RealSolverBackend, SolverError, SolverOptions, Value};
use faer::dyn_stack::{MemBuffer, MemStack};
use faer::linalg::solvers::Solve;
use faer::sparse::linalg::lu as sparse_lu;
use faer::sparse::linalg::solvers::{Lu, SymbolicLu};
use faer::sparse::{SparseColMat, SparseColMatRef, SymbolicSparseColMat};
use faer::{Conj, Mat, get_global_parallelism};
use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

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
pub struct CscIndex {
    offset: usize,
    pattern_id: u64,
}

impl CscIndex {
    /// Numeric offset in the CSC value array.
    ///
    /// Exposed for read-only diagnostics. Stamping should pass the complete
    /// token back to [`StaticMatrix::stamp_direct`] or the corresponding
    /// complex method so the originating pattern can be validated.
    #[inline]
    pub const fn offset(self) -> usize {
        self.offset
    }
}

static NEXT_PATTERN_ID: AtomicU64 = AtomicU64::new(1);

#[inline]
fn next_pattern_id() -> Result<u64, SolverError> {
    NEXT_PATTERN_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .map_err(|_| {
            SolverError::InvalidCircuit("matrix pattern identifier space exhausted".to_string())
        })
}

#[derive(Debug, Clone, Copy)]
struct MissingMatrixPosition {
    method: &'static str,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
enum MatrixStampError {
    MissingPosition(MissingMatrixPosition),
    InvalidIndex {
        method: &'static str,
        offset: usize,
        index_pattern: u64,
        matrix_pattern: u64,
    },
}

impl MissingMatrixPosition {
    fn into_solver_error(self) -> SolverError {
        SolverError::InvalidCircuit(format!(
            "{} missing matrix position ({}, {})",
            self.method, self.row, self.col
        ))
    }
}

impl MatrixStampError {
    fn into_solver_error(self) -> SolverError {
        match self {
            Self::MissingPosition(missing) => missing.into_solver_error(),
            Self::InvalidIndex {
                method,
                offset,
                index_pattern,
                matrix_pattern,
            } => SolverError::InvalidCircuit(format!(
                "{method} received CSC offset {offset} for pattern {index_pattern}, but the matrix uses pattern {matrix_pattern}"
            )),
        }
    }
}

/// Pre-built matrix structure with static topology
///
/// This is the critical optimization: we build the structure once during
/// circuit setup, then only update the values during Newton-Raphson iterations.
/// This avoids the O(N log N) sort and memory allocation on every solve.
pub struct StaticMatrix {
    /// Matrix row count.
    pub nrows: usize,
    /// Matrix column count.
    pub ncols: usize,
    /// Frozen CSC sparsity pattern, validated once at construction.
    /// Solves borrow it as a view — no per-iteration structure copies.
    csc: SymbolicSparseColMat<usize>,
    /// CSC values (mutable - updated each iteration)
    values: Vec<Value>,
    /// Identity shared by matrices cloned from this exact sparsity pattern.
    /// Precomputed stamp tokens carry the same identity, preventing a valid
    /// offset from one topology from silently corrupting another topology.
    pattern_id: u64,
    /// Maximum structural nonzeros in any row, retained for scale-invariant
    /// backward-error acceptance of every numeric backend.
    max_row_nnz: usize,
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
    klu: Option<crate::KluSolver>,
    /// Explicit per-matrix backend policy. This avoids process-global solver
    /// state for commercial embedding while preserving an environment-aware
    /// compatibility constructor.
    solver_options: SolverOptions,
    /// Scratch values + RHS retained between residual probes (see
    /// [`StaticMatrix::with_probe_values`]).
    probe_values: Option<Vec<Value>>,
    probe_rhs: Option<Vec<Value>>,
    /// Scratch for the A*x product inside residual norms.
    residual_scratch: Vec<Value>,
    residual_gross_scratch: Vec<Value>,
    /// Retained KLU iterative-refinement correction. This path is uncommon,
    /// but keeping its buffer makes even ill-scaled repeated solves allocation
    /// free after the first accepted system.
    klu_correction_scratch: Vec<Value>,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MatrixStampError>,
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
    SolverOptions::from_env().real_backend == RealSolverBackend::Klu
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
/// Rows whose entire signal is near underflow use a safe denominator floor.
/// Unlike LAPACK's forward-error estimator, this solve-acceptance check does
/// not add the floor to the numerator: doing so turns an unavoidable single
/// subnormal rounding bit into an order-one error and rejects otherwise exact
/// sparse solves (notably the inactive tail of a large RC ladder).
/// `residual` is retained as `b - A*x` for iterative refinement.
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
    let mut error: Value = 0.0;
    for row in 0..nrows {
        let residual_abs = residual[row].abs();
        let scale = denominator[row];
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let row_error = residual_abs / scale.max(safe1);
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
            pattern_id: self.pattern_id,
            max_row_nnz: self.max_row_nnz,
            position_map: self.position_map.clone(),
            lu: None,
            klu: None,
            solver_options: self.solver_options,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            klu_correction_scratch: Vec::new(),
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
        Self::from_triplets_with_options(nrows, ncols, triplets, SolverOptions::from_env())
    }

    /// Build a static matrix with an explicit backend policy.
    pub fn from_triplets_with_options(
        nrows: usize,
        ncols: usize,
        triplets: &[(usize, usize, Value)],
        solver_options: SolverOptions,
    ) -> Result<Self, SolverError> {
        if nrows == 0 || ncols == 0 {
            return Err(SolverError::InvalidCircuit("Empty matrix".to_string()));
        }
        for (idx, &(row, col, value)) in triplets.iter().enumerate() {
            if row >= nrows || col >= ncols {
                return Err(SolverError::InvalidCircuit(format!(
                    "Triplet {} index out of bounds: ({}, {}) for matrix {}x{}",
                    idx, row, col, nrows, ncols
                )));
            }
            if !value.is_finite() {
                return Err(SolverError::Overflow);
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
                let combined = last.2 + v;
                if !combined.is_finite() {
                    return Err(SolverError::Overflow);
                }
                last.2 = combined;
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

        let mut row_nnz = vec![0_usize; nrows];
        for &row in &row_indices {
            row_nnz[row] = row_nnz[row].saturating_add(1);
        }
        let max_row_nnz = row_nnz.into_iter().max().unwrap_or(0);

        // Validate the pattern once here; every solve afterwards borrows it
        // without re-checking.
        let csc = SymbolicSparseColMat::new_checked(nrows, ncols, col_ptrs, None, row_indices);

        Ok(Self {
            nrows,
            ncols,
            csc,
            values,
            pattern_id: next_pattern_id()?,
            max_row_nnz,
            position_map,
            lu: None,
            klu: None,
            solver_options,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            klu_correction_scratch: Vec::new(),
            stamping_error: None,
        })
    }

    /// Current per-matrix solver policy.
    #[inline]
    pub const fn solver_options(&self) -> SolverOptions {
        self.solver_options
    }

    /// Change the backend used by subsequent solves. Existing workspaces are
    /// retained so switching back does not repeat symbolic setup.
    #[inline]
    pub fn set_solver_options(&mut self, solver_options: SolverOptions) {
        self.solver_options = solver_options;
    }

    /// Zero all values (call before each Newton iteration)
    ///
    /// Slice fill lowers to an optimized bulk clear without coupling this
    /// dependency-neutral matrix crate to the simulator's optional SIMD layer.
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.fill(0.0);
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
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx] + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx] = combined;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error.get_or_insert_with(|| {
            MatrixStampError::MissingPosition(missing_matrix_position(method, row, col))
        });
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
        self.position_map.get(&(row, col)).map(|&offset| CscIndex {
            offset,
            pattern_id: self.pattern_id,
        })
    }

    /// Direct write to values array using pre-computed index
    #[inline]
    pub fn stamp_direct(&mut self, idx: CscIndex, value: Value) {
        if idx.pattern_id != self.pattern_id || idx.offset >= self.values.len() {
            self.stamping_error
                .get_or_insert(MatrixStampError::InvalidIndex {
                    method: "StaticMatrix::stamp_direct",
                    offset: idx.offset,
                    index_pattern: idx.pattern_id,
                    matrix_pattern: self.pattern_id,
                });
            return;
        }
        self.values[idx.offset] += value;
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

    /// Compute the unscaled infinity norm of `A*x-b` without allocating.
    pub fn raw_residual_inf_norm(
        &mut self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<Value, SolverError> {
        self.raw_residual_norms(solution, rhs)
            .map(|(inf_norm, _)| inf_norm)
    }

    /// Compute the unscaled infinity and Euclidean norms of `A*x-b` in one
    /// matrix-vector product without allocating.
    ///
    /// The Euclidean norm uses the LAPACK scaled sum-of-squares recurrence so
    /// large and small residual components cannot overflow or underflow an
    /// otherwise finite norm. Xyce's transient NOX status tests use both norms
    /// at every nonlinear iterate.
    pub fn raw_residual_norms(
        &mut self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<(Value, Value), SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Residual vector size mismatch: matrix is {}x{}, solution has {}, RHS has {}",
                self.nrows,
                self.ncols,
                solution.len(),
                rhs.len()
            )));
        }

        self.residual_scratch.resize(self.nrows, 0.0);
        self.residual_scratch.fill(0.0);
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        for col in 0..self.ncols {
            let x = solution[col];
            if !x.is_finite() {
                return Ok((Value::INFINITY, Value::INFINITY));
            }
            if x == 0.0 {
                continue;
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                self.residual_scratch[row_idx[idx]] += self.values[idx] * x;
            }
        }

        let mut inf_norm = 0.0_f64;
        let mut l2_scale = 0.0_f64;
        let mut l2_sum_squares = 1.0_f64;
        for (row_ax, row_rhs) in self.residual_scratch.iter().zip(rhs) {
            let residual = row_ax - row_rhs;
            if !residual.is_finite() {
                return Ok((Value::INFINITY, Value::INFINITY));
            }
            let magnitude = residual.abs();
            inf_norm = inf_norm.max(magnitude);
            if magnitude != 0.0 {
                if l2_scale < magnitude {
                    let ratio = l2_scale / magnitude;
                    l2_sum_squares = 1.0 + l2_sum_squares * ratio * ratio;
                    l2_scale = magnitude;
                } else {
                    let ratio = magnitude / l2_scale;
                    l2_sum_squares += ratio * ratio;
                }
            }
        }
        let l2_norm = if l2_scale == 0.0 {
            0.0
        } else {
            l2_scale * l2_sum_squares.sqrt()
        };
        Ok((inf_norm, l2_norm))
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
            max_row_nnz: self.max_row_nnz,
        });
        Ok(())
    }

    /// Solve Ax = b using cached structure.
    ///
    /// Convenience ownership API over [`Self::solve_into`]. Internal symbolic
    /// and numeric workspaces are reused; returning an owned vector requires
    /// one output allocation per call.
    pub fn solve(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_into(rhs, &mut solution)?;
        Ok(solution)
    }

    /// Solve `A*x=b` into a caller-owned buffer.
    ///
    /// Reusing `solution` across Newton iterations removes the final allocation
    /// that the convenience [`Self::solve`] API necessarily performs when it
    /// transfers ownership of a fresh vector to its caller.
    pub fn solve_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
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

        if self.solver_options.real_backend == RealSolverBackend::Klu
            && self.try_solve_klu_into(rhs, solution)
        {
            return if solution.iter().all(|value| value.is_finite()) {
                Ok(())
            } else {
                Err(SolverError::SingularMatrix)
            };
        }

        self.solve_faer_into(rhs, solution)
    }

    /// Form the Newton correction right-hand side `b - A*x` without changing
    /// the stamped absolute system.
    ///
    /// Solving for a correction around the current iterate is algebraically
    /// equivalent to solving `A*x=b` directly, but it preserves substantially
    /// more forward accuracy when the iterate is already close to the answer.
    /// Dynamic-device callers may subsequently replace their own rows with a
    /// constitutive residual evaluated from state differences, avoiding loss
    /// that already occurred while forming a large absolute history source.
    pub fn correction_rhs(
        &self,
        rhs: &[Value],
        iterate: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        if rhs.len() != self.nrows || iterate.len() != self.ncols {
            return Err(SolverError::InvalidCircuit(format!(
                "Correction system requires RHS/iterate dimensions {} and {}, got {} and {}",
                self.nrows,
                self.ncols,
                rhs.len(),
                iterate.len()
            )));
        }
        if rhs.iter().chain(iterate).any(|value| !value.is_finite()) {
            return Err(SolverError::Overflow);
        }

        // Accumulate every row as a double-double expansion. Newton
        // corrections are most valuable precisely when large KCL terms nearly
        // cancel; ordinary f64 accumulation would round that small residual to
        // the same scale as the forward error we are trying to remove.
        let mut correction_hi = rhs.to_vec();
        let mut correction_lo = vec![0.0; self.nrows];
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        for col in 0..self.ncols {
            let x = iterate[col];
            for index in col_ptr[col]..col_ptr[col + 1] {
                let value = self.values[index];
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                let row = row_idx[index];
                let product_hi = (-value) * x;
                let product_lo = (-value).mul_add(x, -product_hi);
                let sum = correction_hi[row] + product_hi;
                let virtual_addend = sum - correction_hi[row];
                let sum_error =
                    (correction_hi[row] - (sum - virtual_addend)) + (product_hi - virtual_addend);
                let tail = correction_lo[row] + product_lo + sum_error;
                let refined = sum + tail;
                correction_lo[row] = tail - (refined - sum);
                correction_hi[row] = refined;
            }
        }
        let correction = correction_hi
            .into_iter()
            .zip(correction_lo)
            .map(|(hi, lo)| hi + lo)
            .collect::<Vec<_>>();
        if correction.iter().any(|value| !value.is_finite()) {
            return Err(SolverError::Overflow);
        }
        Ok(correction)
    }

    /// Solve through faer's sparse LU in equilibrated coordinates.
    ///
    /// Kept separate from backend selection so solver tests can exercise this
    /// path without mutating the process-wide `RSPICE_SOLVER` policy.
    #[cfg(test)]
    fn solve_faer(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_faer_into(rhs, &mut solution)?;
        Ok(solution)
    }

    fn solve_faer_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
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
        let Some(ws) = lu.as_mut() else {
            return Err(SolverError::SingularMatrix);
        };

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

        solution.clear();
        solution.extend_from_slice(ws.rhs.col_as_slice(0));
        if solution.iter().any(|value| !value.is_finite()) {
            return Err(SolverError::SingularMatrix);
        }

        let target_error = faer_backward_error_tolerance(ws.max_row_nnz);
        let mut backward_error = componentwise_backward_error(
            csc,
            &ws.scaled_values,
            solution,
            &ws.scaled_rhs,
            residual_scratch,
            residual_gross_scratch,
            ws.max_row_nnz,
        )?;
        if backward_error <= target_error {
            for (value, &col_scale) in solution.iter_mut().zip(&ws.col_scale) {
                *value *= col_scale;
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
            }
            return Ok(());
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

            for (value, &scaled_correction) in solution.iter_mut().zip(ws.rhs.col_as_slice(0)) {
                let refined = *value + scaled_correction;
                if !scaled_correction.is_finite() || !refined.is_finite() {
                    return Err(SolverError::Overflow);
                }
                *value = refined;
            }

            let refined_error = componentwise_backward_error(
                csc,
                &ws.scaled_values,
                solution,
                &ws.scaled_rhs,
                residual_scratch,
                residual_gross_scratch,
                ws.max_row_nnz,
            )?;
            if refined_error <= target_error {
                for (value, &col_scale) in solution.iter_mut().zip(&ws.col_scale) {
                    *value *= col_scale;
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
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
    /// sequence; full re-pivoting only on a growth alarm. Returns `false`
    /// on any backend failure so the caller falls through to faer —
    /// backend fallback can degrade performance but never a result.
    fn try_solve_klu_into(&mut self, rhs: &[Value], solution: &mut Vec<Value>) -> bool {
        let Self {
            nrows,
            csc,
            values,
            klu,
            max_row_nnz,
            residual_scratch,
            residual_gross_scratch,
            klu_correction_scratch,
            ..
        } = self;
        let n = *nrows;
        let col_ptr = csc.col_ptr();
        let row_idx = csc.row_idx();

        let backend = klu.get_or_insert_with(crate::KluSolver::new);
        if !backend.is_analyzed_for(n) && backend.analyze(n, col_ptr, row_idx).is_err() {
            return false;
        }
        let factored = match backend.refactor(values) {
            Ok(()) => true,
            Err(SolverError::PivotGrowth) => backend.factor(values).is_ok(),
            Err(_) => backend.factor(values).is_ok(),
        };
        if !factored {
            static FALLBACK_LOGGED: std::sync::Once = std::sync::Once::new();
            FALLBACK_LOGGED.call_once(|| {
                log::warn!("klu backend could not factor this system; using faer fallback");
            });
            return false;
        }
        if backend.solve(rhs, solution).is_err() {
            return false;
        }
        let target_error = faer_backward_error_tolerance(*max_row_nnz);
        let mut backward_error = match componentwise_backward_error(
            csc,
            values,
            solution,
            rhs,
            residual_scratch,
            residual_gross_scratch,
            *max_row_nnz,
        ) {
            Ok(error) => error,
            Err(_) => return false,
        };
        if backward_error <= target_error {
            return true;
        }

        // KLU's unscaled factors are substantially faster on the ordinary
        // Newton hot path, but ill-scaled MNA rows (for example a 1e12-ohm
        // bias path beside ideal source constraints) can require refinement.
        // Reuse the same factors to solve A*delta = b-A*x, and accept only a
        // componentwise backward error comparable to the equilibrated faer
        // backend. Failure falls through to faer instead of exposing a
        // finite-but-inaccurate circuit state.
        const MAX_KLU_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_KLU_REFINEMENTS {
            if backend
                .solve(residual_scratch, klu_correction_scratch)
                .is_err()
            {
                return false;
            }
            for (value, &delta) in solution.iter_mut().zip(klu_correction_scratch.iter()) {
                let refined = *value + delta;
                if !delta.is_finite() || !refined.is_finite() {
                    return false;
                }
                *value = refined;
            }
            let refined_error = match componentwise_backward_error(
                csc,
                values,
                solution,
                rhs,
                residual_scratch,
                residual_gross_scratch,
                *max_row_nnz,
            ) {
                Ok(error) => error,
                Err(_) => return false,
            };
            if refined_error <= target_error {
                return true;
            }
            if refined_error >= backward_error * MIN_IMPROVEMENT_FACTOR {
                break;
            }
            backward_error = refined_error;
        }
        false
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

        let solution = solve_gauss(self.to_dense_real(), rhs.to_vec())?;
        let backward_error = componentwise_backward_error(
            &self.csc,
            &self.values,
            &solution,
            rhs,
            &mut Vec::new(),
            &mut Vec::new(),
            self.max_row_nnz,
        )?;
        if backward_error <= faer_backward_error_tolerance(self.max_row_nnz) {
            Ok(solution)
        } else {
            Err(SolverError::InaccurateSolution(backward_error))
        }
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
    scaled_values: Vec<Complex64>,
    scaled_rhs: Vec<Complex64>,
    row_scale: Vec<Value>,
    col_scale: Vec<Value>,
    residual: Vec<Complex64>,
    denominator: Vec<Value>,
}

#[inline]
fn complex_is_finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

/// Build `D_r * A * D_c` for a complex matrix. Positive real diagonal
/// scalings preserve phase while bringing both axes into a numerically safe
/// range for sparse pivoting.
fn equilibrate_complex_matrix(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    scaled_values: &mut Vec<Complex64>,
    row_scale: &mut Vec<Value>,
    col_scale: &mut Vec<Value>,
) -> Result<(), SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() {
        return Err(SolverError::InvalidCircuit(
            "Complex sparse equilibration dimension mismatch".to_string(),
        ));
    }

    scaled_values.resize(values.len(), Complex64::new(0.0, 0.0));
    row_scale.resize(nrows, 1.0);
    col_scale.resize(ncols, 1.0);

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let mut max_abs: Value = 0.0;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            if !complex_is_finite(value) {
                return Err(SolverError::Overflow);
            }
            max_abs = max_abs.max(value.norm());
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
            row_scale[row] = row_scale[row].max(scaled_values[idx].norm());
        }
    }
    for scale in row_scale.iter_mut() {
        if *scale == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        *scale = finite_reciprocal_scale(*scale);
    }
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] *= row_scale[row_idx[idx]];
        }
    }
    if scaled_values
        .iter()
        .copied()
        .any(|value| !complex_is_finite(value))
    {
        return Err(SolverError::Overflow);
    }
    Ok(())
}

fn scale_complex_rhs(
    rhs: &[Complex64],
    row_scale: &[Value],
    scaled_rhs: &mut Vec<Complex64>,
) -> Result<(), SolverError> {
    if rhs.len() != row_scale.len() {
        return Err(SolverError::InvalidCircuit(
            "Complex RHS scaling dimension mismatch".to_string(),
        ));
    }
    scaled_rhs.resize(rhs.len(), Complex64::new(0.0, 0.0));
    for ((scaled, &value), &scale) in scaled_rhs.iter_mut().zip(rhs).zip(row_scale) {
        if !complex_is_finite(value) {
            return Err(SolverError::Overflow);
        }
        *scaled = value * scale;
        if !complex_is_finite(*scaled) {
            return Err(SolverError::Overflow);
        }
    }
    Ok(())
}

fn complex_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    solution: &[Complex64],
    rhs: &[Complex64],
    residual: &mut Vec<Complex64>,
    denominator: &mut Vec<Value>,
    max_row_nnz: usize,
) -> Result<Value, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || solution.len() != ncols || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Complex sparse backward-error dimension mismatch".to_string(),
        ));
    }

    residual.resize(nrows, Complex64::new(0.0, 0.0));
    denominator.resize(nrows, 0.0);
    for row in 0..nrows {
        if !complex_is_finite(rhs[row]) {
            return Err(SolverError::Overflow);
        }
        residual[row] = rhs[row];
        denominator[row] = rhs[row].norm();
    }

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let x = solution[col];
        if !complex_is_finite(x) {
            return Err(SolverError::Overflow);
        }
        let x_abs = x.norm();
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            let term = value * x;
            let magnitude = value.norm() * x_abs;
            if !complex_is_finite(value) || !complex_is_finite(term) || !magnitude.is_finite() {
                return Err(SolverError::Overflow);
            }
            let row = row_idx[idx];
            residual[row] -= term;
            denominator[row] = (denominator[row] + magnitude).min(Value::MAX);
        }
    }

    let safe1 = (max_row_nnz.saturating_add(1) as Value) * Value::MIN_POSITIVE;
    let mut error: Value = 0.0;
    for row in 0..nrows {
        let residual_abs = residual[row].norm();
        let scale = denominator[row];
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        error = error.max(residual_abs / scale.max(safe1));
    }
    Ok(error)
}

/// ComplexMatrix for AC small-signal analysis
///
/// Shares the same sparsity structure as a StaticMatrix but uses Complex64 values.
/// This enables AC analysis at different frequencies without rebuilding topology.
pub struct ComplexMatrix {
    /// Matrix row count.
    pub nrows: usize,
    /// Matrix column count.
    pub ncols: usize,
    /// Frozen CSC sparsity pattern (cloned from the real matrix once)
    csc: SymbolicSparseColMat<usize>,
    /// Complex values (updated for each frequency)
    values: Vec<Complex64>,
    /// Identity of the real sparsity pattern from which this matrix was made.
    pattern_id: u64,
    /// Mapping from (row, col) to index in values array
    position_map: FxHashMap<(usize, usize), usize>,
    /// Maximum structural nonzeros in a row for the shared backward-error
    /// tolerance used by real and complex solves.
    max_row_nnz: usize,
    /// Reusable LU workspace; the symbolic part is shared with the real
    /// matrix when available (symbolic LU is scalar-type independent).
    lu: Option<ComplexLuWorkspace>,
    /// True while `lu.numeric` holds the factorization of the current
    /// `values`. Consecutive solves against unchanged values (noise analysis
    /// solves one matrix against many excitation vectors) skip refactorizing.
    factorization_valid: bool,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MatrixStampError>,
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
            pattern_id: real_matrix.pattern_id,
            position_map: real_matrix.position_map.clone(),
            max_row_nnz: real_matrix.max_row_nnz,
            lu: None,
            factorization_valid: false,
            stamping_error: None,
        };
        // Reuse the real matrix's symbolic analysis when faer has already been
        // selected or used as a fallback. A successful default KLU-only DC
        // solve deliberately does not pay this independent symbolic cost.
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
            scaled_values: Vec::new(),
            scaled_rhs: Vec::new(),
            row_scale: Vec::new(),
            col_scale: Vec::new(),
            residual: Vec::new(),
            denominator: Vec::new(),
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
        if !self.validate_direct_index("ComplexMatrix::stamp_direct_real", idx) {
            return;
        }
        self.values[idx.offset] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Direct imaginary add using a precomputed CSC index.
    #[inline]
    pub fn stamp_direct_imag(&mut self, idx: CscIndex, value: Value) {
        if !self.validate_direct_index("ComplexMatrix::stamp_direct_imag", idx) {
            return;
        }
        self.values[idx.offset] += Complex64::new(0.0, value);
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
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx].re + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx].re = combined;
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
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx] + value;
        if !complex_is_finite(value) || !complex_is_finite(combined) {
            return Err(SolverError::Overflow);
        }
        self.values[idx] = combined;
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
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx].im + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx].im = combined;
        self.factorization_valid = false;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error.get_or_insert_with(|| {
            MatrixStampError::MissingPosition(missing_matrix_position(method, row, col))
        });
    }

    #[inline]
    fn validate_direct_index(&mut self, method: &'static str, idx: CscIndex) -> bool {
        if idx.pattern_id == self.pattern_id && idx.offset < self.values.len() {
            return true;
        }
        self.stamping_error
            .get_or_insert(MatrixStampError::InvalidIndex {
                method,
                offset: idx.offset,
                index_pattern: idx.pattern_id,
                matrix_pattern: self.pattern_id,
            });
        false
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
    pub fn multiply_vector(&self, vector: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
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
            if !complex_is_finite(input) {
                return Err(SolverError::Overflow);
            }
            if input == Complex64::new(0.0, 0.0) {
                continue;
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let term = self.values[idx] * input;
                if !complex_is_finite(self.values[idx]) || !complex_is_finite(term) {
                    return Err(SolverError::Overflow);
                }
                product[row_idx[idx]] += term;
                if !complex_is_finite(product[row_idx[idx]]) {
                    return Err(SolverError::Overflow);
                }
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
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_into(rhs, &mut solution)?;
        Ok(solution)
    }

    /// Solve into a caller-owned complex output buffer, reusing its allocation
    /// across frequency points or multiple noise excitations.
    pub fn solve_into(
        &mut self,
        rhs: &[Complex64],
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;

        if n != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Complex solve requires a square matrix matching RHS size, got {}x{} with RHS {}",
                n,
                self.ncols,
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
            max_row_nnz,
            ..
        } = self;
        let Some(ws) = lu.as_mut() else {
            return Err(SolverError::SingularMatrix);
        };

        if !*factorization_valid {
            equilibrate_complex_matrix(
                csc,
                values,
                &mut ws.scaled_values,
                &mut ws.row_scale,
                &mut ws.col_scale,
            )?;
            let mat = SparseColMatRef::new(csc.as_ref(), ws.scaled_values.as_slice());
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

        scale_complex_rhs(rhs, &ws.row_scale, &mut ws.scaled_rhs)?;

        // SAFETY: `ws.numeric` was produced by `ws.symbolic.factorize_numeric_lu`
        // on this matrix's pattern, and `factorization_valid` guarantees the
        // values have not been mutated since (every mutator clears the flag).
        let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };

        ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.scaled_rhs);
        lu_ref.solve_in_place_with_conj(
            Conj::No,
            ws.rhs.as_mut(),
            par,
            MemStack::new(&mut ws.solve_mem),
        );

        solution.clear();
        solution.extend_from_slice(ws.rhs.col_as_slice(0));
        if solution
            .iter()
            .copied()
            .any(|value| !complex_is_finite(value))
        {
            return Err(SolverError::SingularMatrix);
        }

        let target_error = faer_backward_error_tolerance(*max_row_nnz);
        let mut backward_error = complex_componentwise_backward_error(
            csc,
            &ws.scaled_values,
            solution,
            &ws.scaled_rhs,
            &mut ws.residual,
            &mut ws.denominator,
            *max_row_nnz,
        )?;

        const MAX_COMPLEX_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_COMPLEX_REFINEMENTS {
            if backward_error <= target_error {
                for (value, &col_scale) in solution.iter_mut().zip(&ws.col_scale) {
                    *value *= col_scale;
                    if !complex_is_finite(*value) {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
            }

            ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.residual);
            lu_ref.solve_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.solve_mem),
            );
            for (value, &correction) in solution.iter_mut().zip(ws.rhs.col_as_slice(0)) {
                let refined = *value + correction;
                if !complex_is_finite(correction) || !complex_is_finite(refined) {
                    return Err(SolverError::Overflow);
                }
                *value = refined;
            }
            let refined_error = complex_componentwise_backward_error(
                csc,
                &ws.scaled_values,
                solution,
                &ws.scaled_rhs,
                &mut ws.residual,
                &mut ws.denominator,
                *max_row_nnz,
            )?;
            if refined_error <= target_error {
                for (value, &col_scale) in solution.iter_mut().zip(&ws.col_scale) {
                    *value *= col_scale;
                    if !complex_is_finite(*value) {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
            }
            if refined_error >= backward_error * MIN_IMPROVEMENT_FACTOR {
                backward_error = refined_error;
                break;
            }
            backward_error = refined_error;
        }

        Err(SolverError::InaccurateSolution(backward_error))
    }
}

//=============================================================================
// Legacy TripletMatrix (for initial structure building)
//=============================================================================

/// Sparse matrix in triplet (COO) format for accumulating stamps
#[derive(Debug, Clone)]
pub struct TripletMatrix {
    nrows: usize,
    ncols: usize,
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    values: Vec<Value>,
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

    /// Add a checked triplet, returning immediately when its coordinates are
    /// outside the matrix. [`Self::push`] retains deferred validation for the
    /// simulator's topology-building hot path.
    #[inline]
    pub fn try_push(&mut self, row: usize, col: usize, value: Value) -> Result<(), SolverError> {
        if row >= self.nrows || col >= self.ncols {
            return Err(SolverError::InvalidCircuit(format!(
                "Triplet index ({row}, {col}) is outside {}x{} matrix",
                self.nrows, self.ncols
            )));
        }
        if !value.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.push(row, col, value);
        Ok(())
    }

    /// Matrix dimensions.
    #[inline]
    pub const fn shape(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Matrix row count.
    #[inline]
    pub const fn nrows(&self) -> usize {
        self.nrows
    }

    /// Matrix column count.
    #[inline]
    pub const fn ncols(&self) -> usize {
        self.ncols
    }

    /// Iterate over stored coordinates and values in insertion order.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = (usize, usize, Value)> + '_ {
        debug_assert_eq!(self.row_indices.len(), self.col_indices.len());
        debug_assert_eq!(self.row_indices.len(), self.values.len());
        self.row_indices
            .iter()
            .copied()
            .zip(self.col_indices.iter().copied())
            .zip(self.values.iter().copied())
            .map(|((row, col), value)| (row, col, value))
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
        let triplets: Vec<_> = self.entries().collect();

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
    /// Create a sparse LU facade with no cached symbolic pattern.
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
pub(crate) fn solve_gauss(
    mut a: Vec<Vec<Value>>,
    mut b: Vec<Value>,
) -> Result<Vec<Value>, SolverError> {
    let n = b.len();

    if n == 0 {
        return Ok(Vec::new());
    }
    if a.len() != n || a.iter().any(|row| row.len() != n) {
        return Err(SolverError::InvalidCircuit(format!(
            "Dense solve requires an {n}x{n} coefficient matrix"
        )));
    }
    if b.iter().any(|value| !value.is_finite())
        || a.iter().flatten().any(|value| !value.is_finite())
    {
        return Err(SolverError::Overflow);
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

        if max_val == 0.0 || !max_val.is_finite() {
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
            if !factor.is_finite() {
                return Err(SolverError::Overflow);
            }
            for j in k..n {
                a[i][j] -= factor * a[k][j];
                if !a[i][j].is_finite() {
                    return Err(SolverError::Overflow);
                }
            }
            b[i] -= factor * b[k];
            if !b[i].is_finite() {
                return Err(SolverError::Overflow);
            }
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

    /// The triplet entries, the right-hand side, and the expected solution.
    type ScaledTridiagonalSystem = (Vec<(usize, usize, Value)>, Vec<Value>, Vec<Value>);

    fn scaled_tridiagonal_system(
        row_factors: [Value; 5],
        col_factors: [Value; 5],
    ) -> ScaledTridiagonalSystem {
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

        // A one-ulp subnormal tail is below the arithmetic noise floor. It
        // must not turn into an order-one backward error merely because both
        // the exact row signal and its residual are below MIN_POSITIVE.
        let subnormal_error = componentwise_backward_error(
            &StaticMatrix::from_triplets(1, 1, &[(0, 0, 1.0)])
                .unwrap()
                .csc,
            &[1.0],
            &[Value::from_bits(1)],
            &[0.0],
            &mut residual,
            &mut denominator,
            1,
        )
        .unwrap();
        assert!(subnormal_error <= faer_backward_error_tolerance(1));

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
    fn correction_rhs_solves_for_an_update_around_the_iterate() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)],
        )
        .unwrap();
        let rhs = [6.0, 8.0];
        let iterate = [0.75, 2.25];

        let correction_rhs = matrix.correction_rhs(&rhs, &iterate).unwrap();
        assert_eq!(correction_rhs, vec![0.75, -0.25]);
        let delta = matrix.solve(&correction_rhs).unwrap();
        let corrected = iterate
            .iter()
            .zip(delta)
            .map(|(value, update)| value + update)
            .collect::<Vec<_>>();

        assert!((corrected[0] - 1.0).abs() <= 8.0 * Value::EPSILON);
        assert!((corrected[1] - 2.0).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn correction_rhs_retains_a_small_kcl_residual_between_large_terms() {
        let matrix =
            StaticMatrix::from_triplets(1, 3, &[(0, 0, -1.0e16), (0, 1, 1.0), (0, 2, 1.0e16)])
                .unwrap();

        let correction = matrix.correction_rhs(&[0.0], &[1.0, 1.0, 1.0]).unwrap();

        assert_eq!(correction, vec![-1.0]);
    }

    #[test]
    fn raw_residual_inf_norm_is_exact_and_reuses_workspace() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 2.0), (0, 1, -1.0), (1, 0, 1.0), (1, 1, 3.0)],
        )
        .unwrap();

        assert_eq!(
            matrix
                .raw_residual_inf_norm(&[2.0, 1.0], &[3.0, 5.0])
                .unwrap(),
            0.0
        );
        assert_eq!(
            matrix
                .raw_residual_inf_norm(&[2.0, 1.0], &[2.75, 5.5])
                .unwrap(),
            0.5
        );
        let (inf_norm, l2_norm) = matrix
            .raw_residual_norms(&[2.0, 1.0], &[2.75, 5.5])
            .unwrap();
        assert_eq!(inf_norm.to_bits(), 0.5_f64.to_bits());
        assert_eq!(l2_norm.to_bits(), 0.3125_f64.sqrt().to_bits());
        assert!(
            matrix
                .raw_residual_inf_norm(&[Value::NAN, 1.0], &[3.0, 5.0])
                .unwrap()
                .is_infinite()
        );
    }

    #[test]
    fn raw_residual_l2_norm_is_stable_across_extreme_magnitudes() {
        let mut matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();

        let (inf_norm, l2_norm) = matrix
            .raw_residual_norms(&[1.0e300, 1.0e-300], &[0.0, 0.0])
            .unwrap();

        assert_eq!(inf_norm.to_bits(), 1.0e300_f64.to_bits());
        assert!(l2_norm.is_finite());
        assert_eq!(l2_norm.to_bits(), 1.0e300_f64.to_bits());
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
    fn klu_accepts_only_backward_stable_ill_scaled_solutions() {
        let (triplets, rhs, expected) = scaled_tridiagonal_system(
            [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20],
            [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20],
        );
        let mut matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();
        let mut solution = Vec::new();
        assert!(
            matrix.try_solve_klu_into(&rhs, &mut solution),
            "KLU refinement must recover this finite ill-scaled system"
        );
        assert_relative_solution(&solution, &expected);

        let backward_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &solution,
            &rhs,
            &mut Vec::new(),
            &mut Vec::new(),
            matrix.max_row_nnz,
        )
        .unwrap();
        assert!(backward_error <= faer_backward_error_tolerance(matrix.max_row_nnz));
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

        let mut nonfinite = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 1.0)]).unwrap();
        nonfinite.add(0, 0, Value::NAN);
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

    #[test]
    fn explicit_real_backend_policy_produces_accepted_solutions() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        let rhs = [6.0, 8.0];
        for real_backend in [RealSolverBackend::Klu, RealSolverBackend::Faer] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions { real_backend },
            )
            .unwrap();
            assert_eq!(matrix.solver_options().real_backend, real_backend);
            assert_relative_solution(&matrix.solve(&rhs).unwrap(), &[1.0, 2.0]);
        }
    }

    #[test]
    fn solve_into_reuses_the_callers_output_allocation() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        for real_backend in [RealSolverBackend::Klu, RealSolverBackend::Faer] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions { real_backend },
            )
            .unwrap();
            let mut solution = Vec::new();
            matrix.solve_into(&[6.0, 8.0], &mut solution).unwrap();
            let allocation = solution.as_ptr();
            let capacity = solution.capacity();
            matrix.solve_into(&[12.0, 16.0], &mut solution).unwrap();
            assert_eq!(solution.as_ptr(), allocation);
            assert_eq!(solution.capacity(), capacity);
            assert_relative_solution(&solution, &[2.0, 4.0]);
        }
    }

    #[test]
    fn direct_stamp_tokens_are_bound_to_their_originating_pattern() {
        let first = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        let index = first.get_index(0, 0).unwrap();
        let second_index = first.get_index(1, 1).unwrap();

        let mut unrelated = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        unrelated.stamp_direct(index, 1.0);
        let message = unrelated.solve(&[1.0, 1.0]).unwrap_err().to_string();
        assert!(message.contains("StaticMatrix::stamp_direct"));
        assert!(message.contains("pattern"));

        let mut clone = first.clone_structure();
        clone.stamp_direct(index, 2.0);
        clone.stamp_direct(clone.get_index(1, 1).unwrap(), 4.0);
        assert_relative_solution(&clone.solve(&[2.0, 4.0]).unwrap(), &[1.0, 1.0]);

        let mut complex = ComplexMatrix::from_real_structure(&first);
        complex.stamp_direct_real(index, 2.0);
        complex.stamp_direct_real(second_index, 4.0);
        let solution = complex
            .solve(&[Complex64::new(2.0, 0.0), Complex64::new(4.0, 0.0)])
            .unwrap();
        assert!((solution[0] - Complex64::new(1.0, 0.0)).norm() <= 8.0 * Value::EPSILON);
        assert!((solution[1] - Complex64::new(1.0, 0.0)).norm() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn complex_solve_equilibrates_refines_and_reuses_factorization() {
        let real = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        let a = [
            [Complex64::new(4.0, 1.0), Complex64::new(-1.0e-40, 0.5e-40)],
            [Complex64::new(-1.0e40, -0.25e40), Complex64::new(3.0, -0.5)],
        ];
        for (row, row_values) in a.iter().enumerate() {
            for (col, &value) in row_values.iter().enumerate() {
                matrix.add(row, col, value);
            }
        }

        let mut actual = Vec::new();
        let mut allocation = None;
        for expected in [
            [
                Complex64::new(1.0e-20, 0.5e-20),
                Complex64::new(2.0e20, -1.0e20),
            ],
            [
                Complex64::new(-2.0e-20, 1.0e-20),
                Complex64::new(0.5e20, 0.75e20),
            ],
        ] {
            let rhs = [
                a[0][0] * expected[0] + a[0][1] * expected[1],
                a[1][0] * expected[0] + a[1][1] * expected[1],
            ];
            matrix.solve_into(&rhs, &mut actual).unwrap();
            if let Some(previous) = allocation {
                assert_eq!(actual.as_ptr(), previous);
            } else {
                allocation = Some(actual.as_ptr());
            }
            for (index, (&actual, &expected)) in actual.iter().zip(&expected).enumerate() {
                let relative_error = (actual - expected).norm() / expected.norm();
                assert!(
                    relative_error <= 1.0e-10,
                    "complex solution[{index}] relative error {relative_error:.3e}"
                );
            }
        }
    }

    #[test]
    fn complex_operations_reject_nonfinite_and_rectangular_systems() {
        let real = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        matrix.add(0, 0, Complex64::new(Value::NAN, 0.0));
        assert!(matches!(
            matrix.solve(&[Complex64::new(1.0, 0.0)]),
            Err(SolverError::Overflow)
        ));

        matrix.clear_values();
        matrix.add_real(0, 0, 1.0);
        assert!(matches!(
            matrix.solve(&[Complex64::new(Value::INFINITY, 0.0)]),
            Err(SolverError::Overflow)
        ));
        assert!(matches!(
            matrix.multiply_vector(&[Complex64::new(0.0, Value::NAN)]),
            Err(SolverError::Overflow)
        ));

        let rectangular = StaticMatrix::from_triplets(1, 2, &[(0, 0, 1.0), (0, 1, 1.0)]).unwrap();
        let mut rectangular = ComplexMatrix::from_real_structure(&rectangular);
        assert!(matches!(
            rectangular.solve(&[Complex64::new(1.0, 0.0)]),
            Err(SolverError::InvalidCircuit(_))
        ));

        let mut checked = ComplexMatrix::from_real_structure(&real);
        checked.try_add_real(0, 0, 2.0).unwrap();
        assert!(matches!(
            checked.try_add(0, 0, Complex64::new(Value::NAN, 0.0)),
            Err(SolverError::Overflow)
        ));
        assert_eq!(
            checked.solve(&[Complex64::new(4.0, 0.0)]).unwrap()[0],
            Complex64::new(2.0, 0.0)
        );
    }

    #[test]
    fn checked_triplet_api_preserves_storage_invariants() {
        let mut matrix = TripletMatrix::new(2);
        matrix.try_push(0, 0, 1.0).unwrap();
        matrix.try_push(1, 1, 2.0).unwrap();
        assert!(matches!(
            matrix.try_push(2, 0, 3.0),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            matrix.try_push(0, 1, Value::NAN),
            Err(SolverError::Overflow)
        ));
        assert_eq!(matrix.shape(), (2, 2));
        assert_eq!(
            matrix.entries().collect::<Vec<_>>(),
            vec![(0, 0, 1.0), (1, 1, 2.0)]
        );
        assert_relative_solution(&solve_sparse(&matrix, &[1.0, 2.0]).unwrap(), &[1.0, 1.0]);

        assert!(matches!(
            StaticMatrix::from_triplets(1, 1, &[(0, 0, Value::NAN)]),
            Err(SolverError::Overflow)
        ));
        let mut checked = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        checked.try_add(0, 0, 2.0).unwrap();
        assert!(matches!(
            checked.try_add(0, 0, Value::INFINITY),
            Err(SolverError::Overflow)
        ));
        assert_relative_solution(&checked.solve(&[4.0]).unwrap(), &[2.0]);
    }

    #[test]
    fn dense_fallback_is_scale_invariant_and_validates_shape() {
        let matrix = StaticMatrix::from_triplets(1, 1, &[(0, 0, 1.0e-300)]).unwrap();
        assert_relative_solution(&matrix.solve_dense(&[2.0e-300]).unwrap(), &[2.0]);

        assert!(matches!(
            solve_gauss(vec![vec![1.0, 0.0]], vec![1.0]),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solve_gauss(vec![vec![Value::NAN]], vec![1.0]),
            Err(SolverError::Overflow)
        ));
    }
}
