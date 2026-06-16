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
}

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
    /// Experimental KLU-class backend (`RSPICE_SOLVER=klu`): refactors
    /// the frozen pattern with a stored pivot sequence instead of fully
    /// re-pivoting every Newton iteration. Lazily initialized; any
    /// failure falls back to the faer path.
    klu: Option<crate::solver::klu::KluSolver>,
    /// Scratch values + RHS retained between residual probes (see
    /// [`StaticMatrix::with_probe_values`]).
    probe_values: Option<Vec<Value>>,
    probe_rhs: Option<Vec<Value>>,
    /// Scratch for the A*x product inside residual norms.
    residual_scratch: Vec<Value>,
    residual_gross_scratch: Vec<Value>,
}

#[cold]
#[inline(never)]
fn panic_missing_matrix_position(method: &'static str, row: usize, col: usize) -> ! {
    panic!("{method} missing matrix position ({row}, {col})");
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
            None => panic_missing_matrix_position("StaticMatrix::add", row, col),
        };
        self.values[idx] += value;
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

        if n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {} doesn't match RHS size {}",
                n,
                rhs.len()
            )));
        }

        if klu_backend_enabled()
            && let Some(result) = self.try_solve_klu(rhs)
        {
            return Ok(result);
        }

        self.ensure_lu_workspace()?;

        let par = get_global_parallelism();
        let Self {
            csc, values, lu, ..
        } = self;
        let ws = lu.as_mut().expect("LU workspace initialized above");

        let mat = SparseColMatRef::new(csc.as_ref(), values.as_slice());

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

        ws.rhs.col_as_slice_mut(0).copy_from_slice(rhs);
        lu_ref.solve_in_place_with_conj(
            Conj::No,
            ws.rhs.as_mut(),
            par,
            MemStack::new(&mut ws.solve_mem),
        );

        Ok(ws.rhs.col_as_slice(0).to_vec())
    }

    /// Experimental KLU-class solve (`RSPICE_SOLVER=klu`): values-only
    /// refactorization over the frozen pattern with a stored pivot
    /// sequence; full re-pivoting only on a growth alarm. Returns `None`
    /// on any backend failure so the caller falls through to faer —
    /// the experiment can degrade performance but never a result.
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
            None => panic_missing_matrix_position("ComplexMatrix::add_real", row, col),
        };
        self.values[idx] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Add complex value at (row, col)
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Complex64) {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => panic_missing_matrix_position("ComplexMatrix::add", row, col),
        };
        self.values[idx] += value;
        self.factorization_valid = false;
    }

    /// Add imaginary value (for frequency-dependent terms like jwC)
    #[inline]
    pub fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        let idx = match self.position_map.get(&(row, col)) {
            Some(&idx) => idx,
            None => panic_missing_matrix_position("ComplexMatrix::add_imag", row, col),
        };
        self.values[idx] += Complex64::new(0.0, value);
        self.factorization_valid = false;
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

    /// Solve Ax = b for complex values.
    ///
    /// The symbolic analysis is computed once per structure (or inherited
    /// from the real matrix) and the numeric factorization is reused across
    /// consecutive solves with unchanged values.
    pub fn solve(&mut self, rhs: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
        let n = self.nrows;

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
