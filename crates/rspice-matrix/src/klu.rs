//! KLU-class sparse LU for circuit matrices.
//!
//! Circuit Newton loops factor the *same sparsity pattern* hundreds of
//! thousands of times with changing values. This solver exploits that:
//!
//! * **analyze** — maximum structural matching, block-triangular form (BTF),
//!   then fill-reducing AMD inside each irreducible diagonal block, computed
//!   once per pattern;
//! * **factor** — left-looking Gilbert–Peierls LU with
//!   diagonal-preference threshold pivoting (KLU's default bias keeps
//!   circuit diagonals as pivots whenever they are within `PIVOT_TOL`
//!   of the column maximum, which keeps the pivot sequence reusable);
//! * **refactor** — the hot path: values-only replay over the frozen
//!   L/U pattern with the stored pivots — no symbolic work, no pivot
//!   search, no allocation. A pivot-growth alarm falls back to a fresh
//!   full factorization (and the caller may fall back further).
//!
//! This is the default real-valued backend (`RSPICE_SOLVER=faer` opts
//! out). Kernel conventions, all benchmark-gated (`rspice-bench klu`):
//! u32 row indices (half the index bandwidth), a precomputed pivot-space
//! scatter map for A's values, paired-slice iteration for bounds-check
//! elision, reciprocal pivot scaling (the contiguous multiplies
//! autovectorize — the gather/scatter loops themselves cannot, which is
//! also why KLU-class solvers are famously non-BLAS), and an
//! allocation-free solve path.

#![allow(clippy::needless_range_loop)]

use crate::{SolverError, Value};

#[inline]
fn finite_reciprocal_scale(max_abs: Value) -> Value {
    debug_assert!(max_abs.is_finite() && max_abs > 0.0);
    let small_number = Value::MIN_POSITIVE / Value::EPSILON;
    let large_number = 1.0 / small_number;
    (1.0 / max_abs).clamp(small_number, large_number)
}

#[inline]
fn reserve_for<T>(values: &mut Vec<T>, additional: usize) -> Result<(), SolverError> {
    values
        .try_reserve(additional)
        .map_err(|_| SolverError::OutOfMemory)
}

#[inline]
fn resize_fallible<T: Clone>(
    values: &mut Vec<T>,
    new_len: usize,
    value: T,
) -> Result<(), SolverError> {
    if new_len > values.len() {
        reserve_for(values, new_len - values.len())?;
    }
    values.resize(new_len, value);
    Ok(())
}

/// KLU's default partial-pivoting tolerance: the diagonal entry is kept
/// as the pivot when `|diag| >= PIVOT_TOL * colmax`.
const PIVOT_TOL: Value = 1e-3;

/// Refactor pivot-growth alarm. Reusing a stable pivot sequence is the KLU
/// hot path; the solve-level backward-error check remains the final authority
/// on accuracy and triggers the supernodal fallback when necessary.
const REFACTOR_PIVOT_TOL: Value = 1e-8;

/// Numeric quality indicators for the current factorization.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct KluDiagnostics {
    /// Smallest ratio of the largest scaled input entry to the largest U
    /// entry in the corresponding factor column. Values near zero indicate
    /// damaging element growth.
    pub reciprocal_pivot_growth: Value,
    /// Cheap reciprocal-condition proxy formed from the smallest and largest
    /// absolute U diagonal entries. This is conservative enough for backend
    /// routing but is not a replacement for a full condition estimator.
    pub diagonal_rcond: Value,
    /// Smallest absolute pivot in the row-equilibrated factorization.
    pub min_abs_pivot: Value,
    /// Largest absolute pivot in the row-equilibrated factorization.
    pub max_abs_pivot: Value,
}

#[derive(Debug, Default)]
struct FactorWorkspace {
    pinv: Vec<usize>,
    p_row: Vec<u32>,
    l_ptr: Vec<usize>,
    l_rows: Vec<u32>,
    l_vals: Vec<Value>,
    u_ptr: Vec<usize>,
    u_pos: Vec<u32>,
    u_vals: Vec<Value>,
    u_diag_recip: Vec<Value>,
    x: Vec<Value>,
    flag: Vec<usize>,
    topo: Vec<usize>,
    dfs_stack: Vec<(usize, usize)>,
    nonpivot_rows: Vec<usize>,
}

/// Sparse LU with a reusable pivot sequence.
#[derive(Debug, Default)]
pub struct KluSolver {
    n: usize,
    pivot_tolerance: Value,
    absolute_pivot_tolerance: Value,
    /// Validated CSC column pointers for the analyzed matrix. Keeping these
    /// with the factorization makes a values-only refactor structurally safe:
    /// callers cannot accidentally replay values against another pattern.
    a_col_ptr: Vec<usize>,
    /// Original CSC row indices narrowed after validation. Full re-pivoting
    /// needs the original rows; storing them here also removes a pointer-sized
    /// row-index load from the cold factorization path.
    a_rows: Vec<u32>,
    /// Fill-reducing column order: pivot column k eliminates original
    /// column `col_perm[k]`.
    col_perm: Vec<u32>,
    /// Structural transversal: preferred pivot row for each original column.
    matched_row_for_col: Vec<u32>,
    /// BTF block identifier for every original row and column. Numeric pivots
    /// are restricted to the current diagonal block.
    row_block: Vec<u32>,
    col_block: Vec<u32>,
    /// Number of irreducible blocks. Zero means matching failed and numeric
    /// factorization uses unrestricted pivots before reporting singularity.
    block_count: usize,
    /// Pivot row order: pivot position k sits on original row
    /// `row_perm[k]`.
    row_perm: Vec<u32>,
    /// Unit-lower-triangular L in CSC over *pivot-space* rows, strictly
    /// below the diagonal (the implicit unit diagonal is not stored).
    /// Row indices are `u32` — half the index memory traffic of `usize`
    /// in the gather/scatter inner loops (n is bounded far below 2³²).
    l_col_ptr: Vec<usize>,
    l_rows: Vec<u32>,
    l_vals: Vec<Value>,
    /// Upper-triangular U in CSC over pivot-space rows, diagonal last in
    /// each column; off-diagonal rows in elimination (stored) order.
    u_col_ptr: Vec<usize>,
    u_rows: Vec<u32>,
    u_vals: Vec<Value>,
    /// Reciprocals of U's diagonal, one per pivot column. Sparse back solve
    /// otherwise performs one hardware divide per unknown; ordinary circuit
    /// pivots are normal values, so replacing those divides with multiplies
    /// materially reduces repeated-RHS solve latency.
    u_diag_recip: Vec<Value>,
    /// False only when a finite, nonzero subnormal pivot has an infinite
    /// reciprocal. That rare scale is still solved correctly with division.
    use_diag_recip: bool,
    /// Refactor scatter targets: pivot-space destination for every entry
    /// of A's value array (aligned to the original CSC value index), so
    /// the refactor scatter is one indexed load + store per nonzero with
    /// no row-index load and no permutation lookup.
    a_scatter: Vec<u32>,
    /// Left row equilibration applied to both A and b. The solution is in the
    /// original coordinates because no column scaling is applied.
    row_scale: Vec<Value>,
    /// Factor-time row scale aligned with the original CSC values. Reusing
    /// these scales during refactor removes a random row-scale gather from
    /// every nonzero on the Newton hot path.
    a_entry_scale: Vec<Value>,
    /// Scatter workspace (pivot space).
    work: Vec<Value>,
    /// Retained row-major workspace for multi-right-hand-side solves.
    batch_work: Vec<Value>,
    diagnostics: KluDiagnostics,
    condition_rhs: Vec<Value>,
    condition_solution: Vec<Value>,
    condition_transpose: Vec<Value>,
    factor_workspace: FactorWorkspace,
    /// Whether L/U currently hold a valid factorization.
    factored: bool,
}

impl KluSolver {
    /// Create an empty solver. Call [`Self::analyze`] once for a pattern, then
    /// [`Self::factor`] or [`Self::refactor`] before solving.
    pub fn new() -> Self {
        Self {
            pivot_tolerance: PIVOT_TOL,
            ..Self::default()
        }
    }

    /// Set the relative threshold-pivoting tolerance used by subsequent full
    /// factors and values-only refactors.
    pub fn set_pivot_tolerance(&mut self, tolerance: Value) -> Result<(), SolverError> {
        if !tolerance.is_finite() || tolerance <= 0.0 || tolerance > 1.0 {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU pivot tolerance must be finite and in (0, 1], got {tolerance}"
            )));
        }
        if tolerance.to_bits() != self.pivot_tolerance.to_bits() {
            self.pivot_tolerance = tolerance;
            self.factored = false;
        }
        Ok(())
    }

    /// Set the absolute pivot threshold in original matrix units. Zero
    /// disables the threshold.
    pub fn set_absolute_pivot_tolerance(&mut self, tolerance: Value) -> Result<(), SolverError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU absolute pivot tolerance must be finite and non-negative, got {tolerance}"
            )));
        }
        if tolerance.to_bits() != self.absolute_pivot_tolerance.to_bits() {
            self.absolute_pivot_tolerance = tolerance;
            self.factored = false;
        }
        Ok(())
    }

    #[inline]
    fn effective_pivot_tolerance(&self) -> Value {
        if self.pivot_tolerance > 0.0 {
            self.pivot_tolerance
        } else {
            // Preserve the public `Default` contract even though the derived
            // zero initializes numeric storage fields.
            PIVOT_TOL
        }
    }

    /// Drop every symbolic and numeric association with the prior pattern.
    /// Capacities are retained so a caller that rebuilds a same-sized circuit
    /// does not pay avoidable allocator traffic.
    fn invalidate_analysis(&mut self) {
        self.n = 0;
        self.a_col_ptr.clear();
        self.a_rows.clear();
        self.col_perm.clear();
        self.matched_row_for_col.clear();
        self.row_block.clear();
        self.col_block.clear();
        self.block_count = 0;
        self.row_perm.clear();
        self.l_col_ptr.clear();
        self.l_rows.clear();
        self.l_vals.clear();
        self.u_col_ptr.clear();
        self.u_rows.clear();
        self.u_vals.clear();
        self.u_diag_recip.clear();
        self.a_scatter.clear();
        self.row_scale.clear();
        self.a_entry_scale.clear();
        self.work.clear();
        self.batch_work.clear();
        self.diagnostics = KluDiagnostics::default();
        self.condition_rhs.clear();
        self.condition_solution.clear();
        self.condition_transpose.clear();
        self.use_diag_recip = false;
        self.factored = false;
    }

    /// Whether the symbolic analysis matches this pattern instance.
    pub(crate) fn is_analyzed_for(&self, n: usize) -> bool {
        self.n == n && self.a_col_ptr.len() == n.saturating_add(1)
    }

    /// Release factor-sized numeric storage while preserving the symbolic
    /// ordering. Auto routing uses this after measuring supernodal-friendly
    /// fill so the rejected backend does not permanently double matrix memory.
    pub(crate) fn discard_numeric_factorization(&mut self) {
        self.row_perm = Vec::new();
        self.l_col_ptr = Vec::new();
        self.l_rows = Vec::new();
        self.l_vals = Vec::new();
        self.u_col_ptr = Vec::new();
        self.u_rows = Vec::new();
        self.u_vals = Vec::new();
        self.u_diag_recip = Vec::new();
        self.a_scatter = Vec::new();
        self.row_scale = Vec::new();
        self.a_entry_scale = Vec::new();
        self.work = Vec::new();
        self.batch_work = Vec::new();
        self.condition_rhs = Vec::new();
        self.condition_solution = Vec::new();
        self.condition_transpose = Vec::new();
        self.factor_workspace = FactorWorkspace::default();
        self.diagnostics = KluDiagnostics::default();
        self.use_diag_recip = false;
        self.factored = false;
    }

    /// `(L, U)` stored nonzero counts of the current factorization —
    /// fill diagnostics for ordering quality.
    pub fn factor_nnz(&self) -> (usize, usize) {
        (self.l_vals.len(), self.u_vals.len())
    }

    /// Number of irreducible diagonal BTF blocks found by analysis.
    pub fn block_count(&self) -> usize {
        self.block_count
    }

    /// Numeric quality indicators for the current factorization.
    pub fn diagnostics(&self) -> KluDiagnostics {
        self.diagnostics
    }

    /// Recompute reciprocal pivot growth for the current values on demand.
    ///
    /// This is intentionally separate from [`Self::refactor`]: production
    /// Newton loops should not pay an additional pair of reductions over
    /// every factor column unless diagnostics are actually requested.
    pub fn recompute_reciprocal_pivot_growth(
        &mut self,
        values: &[Value],
    ) -> Result<Value, SolverError> {
        if !self.factored {
            return Err(SolverError::SingularMatrix);
        }
        if values.len() != self.a_rows.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU pivot-growth values length {} does not match analyzed nonzero count {}",
                values.len(),
                self.a_rows.len()
            )));
        }
        let mut growth = Value::INFINITY;
        for j in 0..self.n {
            let original_col = self.col_perm[j] as usize;
            let mut input_max: Value = 0.0;
            for index in self.a_col_ptr[original_col]..self.a_col_ptr[original_col + 1] {
                let scaled = values[index] * self.a_entry_scale[index];
                if !scaled.is_finite() {
                    return Err(SolverError::Overflow);
                }
                input_max = input_max.max(scaled.abs());
            }
            let mut factor_max: Value = 0.0;
            for &value in &self.u_vals[self.u_col_ptr[j]..self.u_col_ptr[j + 1]] {
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                factor_max = factor_max.max(value.abs());
            }
            if factor_max > 0.0 {
                growth = growth.min(input_max / factor_max);
            }
        }
        let growth = if growth.is_finite() { growth } else { 0.0 };
        self.diagnostics.reciprocal_pivot_growth = growth;
        Ok(growth)
    }

    /// Estimate `1 / cond_1(A)` with a bounded Hager iteration using the
    /// existing normal and transpose factors. This is intentionally on demand:
    /// running multiple triangular solves on every Newton refactor would harm
    /// the hot path. Returns zero for a numerically singular estimate.
    pub fn estimate_rcond(&mut self, values: &[Value]) -> Result<Value, SolverError> {
        if !self.factored {
            return Err(SolverError::SingularMatrix);
        }
        if values.len() != self.a_rows.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU condition estimate values length {} does not match analyzed nonzero count {}",
                values.len(),
                self.a_rows.len()
            )));
        }
        if self.n == 0 {
            return Ok(1.0);
        }
        let n = self.n;
        let mut matrix_one_norm: Value = 0.0;
        for col in 0..n {
            let mut column_sum: Value = 0.0;
            for &value in &values[self.a_col_ptr[col]..self.a_col_ptr[col + 1]] {
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                column_sum = (column_sum + value.abs()).min(Value::MAX);
            }
            matrix_one_norm = matrix_one_norm.max(column_sum);
        }
        let mut rhs = std::mem::take(&mut self.condition_rhs);
        let mut solution = std::mem::take(&mut self.condition_solution);
        let mut transpose = std::mem::take(&mut self.condition_transpose);
        if let Err(error) = resize_fallible(&mut rhs, n, 1.0 / n as Value) {
            self.condition_rhs = rhs;
            self.condition_solution = solution;
            self.condition_transpose = transpose;
            return Err(error);
        }
        rhs.fill(1.0 / n as Value);
        let mut inverse_norm: Value = 0.0;
        let mut previous_index = usize::MAX;
        let estimate = (|| {
            for _ in 0..5 {
                self.solve(&rhs, &mut solution)?;
                inverse_norm = solution
                    .iter()
                    .fold(0.0, |sum, value| (sum + value.abs()).min(Value::MAX));
                for (sign, &value) in rhs.iter_mut().zip(&solution) {
                    *sign = if value.is_sign_negative() { -1.0 } else { 1.0 };
                }
                self.solve_transpose(&rhs, &mut transpose)?;
                let Some((index, &maximum)) = transpose
                    .iter()
                    .enumerate()
                    .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()))
                else {
                    return Ok(0.0);
                };
                if index == previous_index || maximum == 0.0 {
                    break;
                }
                previous_index = index;
                rhs.fill(0.0);
                rhs[index] = 1.0;
            }
            let condition = matrix_one_norm * inverse_norm;
            Ok(if condition.is_finite() && condition > 0.0 {
                (1.0 / condition).min(1.0)
            } else {
                0.0
            })
        })();
        self.condition_rhs = rhs;
        self.condition_solution = solution;
        self.condition_transpose = transpose;
        estimate
    }

    /// One-time symbolic phase for a pattern: structural matching, BTF, and
    /// AMD on each symmetrized diagonal block. Structurally singular patterns
    /// fall back to global AMD so [`Self::factor`] can report the numeric
    /// singularity. The L/U pattern is discovered during first factorization.
    pub fn analyze(
        &mut self,
        n: usize,
        col_ptr: &[usize],
        row_idx: &[usize],
    ) -> Result<(), SolverError> {
        // Analysis is fail-closed: after any rejected pattern the old matrix
        // cannot accidentally be factored or solved under the assumption that
        // this call succeeded.
        self.invalidate_analysis();
        validate_csc_pattern(n, col_ptr, row_idx)?;
        self.n = n;
        if let Some(order) = btf_amd_order(n, col_ptr, row_idx)? {
            reserve_for(&mut self.col_perm, order.col_perm.len())?;
            self.col_perm
                .extend(order.col_perm.into_iter().map(|col| col as u32));
            self.matched_row_for_col = order.matched_row_for_col;
            self.row_block = order.row_block;
            self.col_block = order.col_block;
            self.block_count = order.block_count;
        } else {
            let order = amd_order(n, col_ptr, row_idx)?;
            reserve_for(&mut self.col_perm, order.len())?;
            self.col_perm
                .extend(order.into_iter().map(|col| col as u32));
        }
        reserve_for(&mut self.a_col_ptr, col_ptr.len())?;
        self.a_col_ptr.extend_from_slice(col_ptr);
        reserve_for(&mut self.a_rows, row_idx.len())?;
        self.a_rows.extend(row_idx.iter().map(|&row| row as u32));
        resize_fallible(&mut self.work, n, 0.0)?;
        self.work.fill(0.0);
        resize_fallible(&mut self.row_scale, n, 1.0)?;
        resize_fallible(&mut self.a_entry_scale, row_idx.len(), 1.0)?;
        Ok(())
    }

    /// Compute max-row equilibration, matching the robust default used by
    /// KLU-family production solvers. A numerically empty row is singular.
    fn update_row_scaling(&mut self, values: &[Value]) -> Result<(), SolverError> {
        self.row_scale.resize(self.n, 0.0);
        self.row_scale.fill(0.0);
        for col in 0..self.n {
            for idx in self.a_col_ptr[col]..self.a_col_ptr[col + 1] {
                let value = values[idx];
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                let slot = &mut self.row_scale[self.a_rows[idx] as usize];
                *slot = slot.max(value.abs());
            }
        }
        let mut smallest = Value::INFINITY;
        let mut largest: Value = 0.0;
        for &scale in &self.row_scale {
            if scale == 0.0 {
                return Err(SolverError::SingularMatrix);
            }
            smallest = smallest.min(scale);
            largest = largest.max(scale);
        }
        // Equilibration materially improves values near the floating-point
        // range limits, but needlessly rescales ordinary circuit rows and can
        // perturb a nonlinear Newton trajectory for no conditioning benefit.
        // The solve-level backward-error gate still routes other difficult
        // systems to the independently equilibrated supernodal backend.
        const ROW_SCALING_SMALL_LIMIT: Value = 1.0e-150;
        const ROW_SCALING_LARGE_LIMIT: Value = 1.0e150;
        if smallest < ROW_SCALING_SMALL_LIMIT || largest > ROW_SCALING_LARGE_LIMIT {
            for scale in &mut self.row_scale {
                *scale = finite_reciprocal_scale(*scale);
            }
        } else {
            self.row_scale.fill(1.0);
        }
        self.a_entry_scale.resize(values.len(), 1.0);
        for (index, &row) in self.a_rows.iter().enumerate() {
            self.a_entry_scale[index] = self.row_scale[row as usize];
        }
        Ok(())
    }

    /// Full Gilbert–Peierls factorization with fresh pivot selection.
    pub fn factor(&mut self, values: &[Value]) -> Result<(), SolverError> {
        self.factor_with_pivot_tolerance(values, self.effective_pivot_tolerance())?;
        // Preserve the sparse diagonal on the normal path, but rebuild once
        // with maximum partial pivoting when measured element growth shows
        // that threshold pivoting selected a numerically damaging sequence.
        if self.diagnostics.reciprocal_pivot_growth < Value::EPSILON.sqrt() {
            self.factor_with_pivot_tolerance(values, 1.0)?;
        }
        Ok(())
    }

    fn factor_with_pivot_tolerance(
        &mut self,
        values: &[Value],
        pivot_tolerance: Value,
    ) -> Result<(), SolverError> {
        self.factored = false;
        if self.a_col_ptr.len() != self.n.saturating_add(1) {
            return Err(SolverError::InvalidCircuit(
                "KLU factorization requires a successfully analyzed pattern".to_string(),
            ));
        }
        if values.len() != self.a_rows.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU values length {} does not match analyzed nonzero count {}",
                values.len(),
                self.a_rows.len()
            )));
        }
        self.update_row_scaling(values)?;
        let n = self.n;
        let col_ptr = &self.a_col_ptr;
        let row_idx = &self.a_rows;
        let ws = &mut self.factor_workspace;

        // Move the current factor storage into the build workspace so a full
        // repivot reuses its capacity. The workspace receives empty vectors
        // again on success; no second factor-sized allocation is retained.
        if !self.u_col_ptr.is_empty() {
            ws.l_ptr = Vec::new();
            ws.l_vals = Vec::new();
            ws.u_ptr = Vec::new();
            ws.u_vals = Vec::new();
            ws.u_diag_recip = Vec::new();
            ws.p_row = Vec::new();
            std::mem::swap(&mut self.l_col_ptr, &mut ws.l_ptr);
            std::mem::swap(&mut self.l_vals, &mut ws.l_vals);
            std::mem::swap(&mut self.u_col_ptr, &mut ws.u_ptr);
            std::mem::swap(&mut self.u_vals, &mut ws.u_vals);
            std::mem::swap(&mut self.u_diag_recip, &mut ws.u_diag_recip);
            std::mem::swap(&mut self.row_perm, &mut ws.p_row);
        }

        // pinv[orig_row] = pivot position, or usize::MAX while unpivoted.
        resize_fallible(&mut ws.pinv, n, usize::MAX)?;
        ws.pinv.fill(usize::MAX);
        resize_fallible(&mut ws.p_row, n, u32::MAX)?;
        ws.p_row.fill(u32::MAX);

        // Working L/U with ORIGINAL row indices (remapped to pivot space
        // once the full pivot sequence is known).
        ws.l_ptr.clear();
        reserve_for(&mut ws.l_ptr, n.saturating_add(1))?;
        ws.l_rows.clear();
        ws.l_vals.clear();
        ws.u_ptr.clear();
        reserve_for(&mut ws.u_ptr, n.saturating_add(1))?;
        ws.u_pos.clear();
        ws.u_vals.clear();
        ws.u_diag_recip.clear();
        reserve_for(&mut ws.u_diag_recip, n)?;
        let mut use_diag_recip = true;
        let mut reciprocal_pivot_growth = Value::INFINITY;
        let mut min_abs_pivot = Value::INFINITY;
        let mut max_abs_pivot: Value = 0.0;
        ws.l_ptr.push(0);
        ws.u_ptr.push(0);

        // Scatter workspace split in two halves: pivoted rows live at
        // their pivot position `k < n`, unpivoted rows at `n + original`,
        // so a pivot position can never collide with an original index.
        let x_len = n.checked_mul(2).ok_or_else(|| {
            SolverError::InvalidCircuit("KLU factor workspace size overflow".to_string())
        })?;
        resize_fallible(&mut ws.x, x_len, 0.0)?;
        ws.x.fill(0.0);
        // DFS state: flag[row] == j+1 marks visitation in column j.
        resize_fallible(&mut ws.flag, n, 0)?;
        ws.flag.fill(0);
        ws.topo.clear();
        reserve_for(&mut ws.topo, n)?;
        ws.dfs_stack.clear();
        reserve_for(&mut ws.dfs_stack, n)?;
        ws.nonpivot_rows.clear();
        reserve_for(&mut ws.nonpivot_rows, n)?;

        let FactorWorkspace {
            pinv,
            p_row,
            l_ptr,
            l_rows,
            l_vals,
            u_ptr,
            u_pos,
            u_vals,
            u_diag_recip,
            x,
            flag,
            topo,
            dfs_stack,
            nonpivot_rows,
        } = ws;

        for j in 0..n {
            let a_col = self.col_perm[j] as usize;
            let stamp = j + 1;
            topo.clear();
            nonpivot_rows.clear();
            let mut input_col_max: Value = 0.0;

            // Symbolic + numeric scatter: reach of the column's pattern
            // through already-built L columns (depth-first, postorder
            // gives the topological elimination order reversed).
            for idx in col_ptr[a_col]..col_ptr[a_col + 1] {
                let row = row_idx[idx] as usize;
                let scaled_value = values[idx] * self.row_scale[row];
                input_col_max = input_col_max.max(scaled_value.abs());
                if flag[row] == stamp {
                    x[Self::x_slot(n, pinv, row)] += scaled_value;
                    continue;
                }
                Self::dfs_reach(
                    n,
                    row,
                    stamp,
                    flag,
                    pinv,
                    l_ptr,
                    l_rows,
                    dfs_stack,
                    topo,
                    nonpivot_rows,
                    x,
                );
                x[Self::x_slot(n, pinv, row)] += scaled_value;
            }

            // Numeric left-looking elimination in topological order.
            // `topo` holds pivot positions discovered in reverse
            // topological order (postorder), so walk it backwards.
            reserve_for(u_pos, topo.len().saturating_add(1))?;
            reserve_for(u_vals, topo.len().saturating_add(1))?;
            for &k in topo.iter().rev() {
                let alpha = x[k];
                if !alpha.is_finite() {
                    return Err(SolverError::Overflow);
                }
                u_pos.push(k as u32);
                u_vals.push(alpha);
                if alpha != 0.0 {
                    for li in l_ptr[k]..l_ptr[k + 1] {
                        let row = l_rows[li] as usize;
                        let slot = Self::x_slot(n, pinv, row);
                        let updated = x[slot] - alpha * l_vals[li];
                        if !updated.is_finite() {
                            return Err(SolverError::Overflow);
                        }
                        x[slot] = updated;
                    }
                }
                x[k] = 0.0;
            }

            // Pivot selection over the unpivoted rows: column maximum
            // with structural-transversal preference within PIVOT_TOL. When
            // BTF is available, never pivot across a diagonal block boundary.
            let current_block = self.col_block.get(a_col).copied();
            let preferred_row = self
                .matched_row_for_col
                .get(a_col)
                .map_or(a_col, |&row| row as usize);
            let mut max_abs = 0.0_f64;
            let mut max_row = usize::MAX;
            let mut diag_abs = -1.0_f64;
            reserve_for(l_rows, nonpivot_rows.len())?;
            reserve_for(l_vals, nonpivot_rows.len())?;
            for &row in nonpivot_rows.iter() {
                if current_block.is_some_and(|block| self.row_block[row] != block) {
                    continue;
                }
                let v = x[n + row].abs();
                if v > max_abs {
                    max_abs = v;
                    max_row = row;
                }
                if row == preferred_row {
                    diag_abs = v;
                }
            }
            if max_row == usize::MAX || max_abs == 0.0 || !max_abs.is_finite() {
                return Err(SolverError::SingularMatrix);
            }
            let pivot_row = if diag_abs >= pivot_tolerance * max_abs {
                preferred_row
            } else {
                max_row
            };
            let pivot_val = x[n + pivot_row];
            let pivot_abs = pivot_val.abs();
            if self.absolute_pivot_tolerance > 0.0
                && pivot_abs / self.row_scale[pivot_row] < self.absolute_pivot_tolerance
            {
                return Err(SolverError::SingularMatrix);
            }
            min_abs_pivot = min_abs_pivot.min(pivot_abs);
            max_abs_pivot = max_abs_pivot.max(pivot_abs);
            let pivot_recip = 1.0 / pivot_val;
            use_diag_recip &= pivot_recip.is_finite();
            u_diag_recip.push(pivot_recip);

            // Emit U's diagonal last so solves can read it directly.
            u_pos.push(j as u32);
            u_vals.push(pivot_val);

            let u_col_max = u_vals[u_ptr[j]..]
                .iter()
                .fold(0.0_f64, |max, value| max.max(value.abs()));
            if u_col_max > 0.0 {
                reciprocal_pivot_growth = reciprocal_pivot_growth.min(input_col_max / u_col_max);
            }

            // Emit L column (unpivoted rows except the pivot), scaled by
            // the pivot reciprocal (one divide per column; the multiplies
            // autovectorize). Numeric zeros are kept: the pattern is
            // *symbolic* — a value that cancels at this factorization can
            // be nonzero at the next refactor, which replays these slots.
            for &row in nonpivot_rows.iter() {
                let slot = n + row;
                let v = x[slot];
                x[slot] = 0.0;
                if !v.is_finite() {
                    return Err(SolverError::Overflow);
                }
                if row == pivot_row {
                    continue;
                }
                l_rows.push(row as u32);
                let factor_value = if pivot_recip.is_finite() {
                    v * pivot_recip
                } else {
                    v / pivot_val
                };
                if !factor_value.is_finite() {
                    return Err(SolverError::Overflow);
                }
                l_vals.push(factor_value);
            }
            l_ptr.push(l_rows.len());
            u_ptr.push(u_pos.len());

            pinv[pivot_row] = j;
            p_row[j] = pivot_row as u32;
        }

        // Remap L's original row indices into pivot space (after a full
        // factorization every row holds a pivot position) and narrow the
        // index arrays to u32 for the hot loops.
        self.l_rows.clear();
        reserve_for(&mut self.l_rows, l_rows.len())?;
        self.l_rows
            .extend(l_rows.iter().map(|&row| pinv[row as usize] as u32));
        self.u_rows.clear();
        reserve_for(&mut self.u_rows, u_pos.len())?;
        self.u_rows.extend(u_pos.iter().copied());

        // Precompute the refactor scatter: pivot-space target of every
        // entry of A's value array, aligned to the original value index.
        self.a_scatter.clear();
        reserve_for(&mut self.a_scatter, row_idx.len())?;
        self.a_scatter
            .extend(row_idx.iter().map(|&row| pinv[row as usize] as u32));

        std::mem::swap(&mut self.row_perm, p_row);
        std::mem::swap(&mut self.l_col_ptr, l_ptr);
        std::mem::swap(&mut self.l_vals, l_vals);
        std::mem::swap(&mut self.u_col_ptr, u_ptr);
        std::mem::swap(&mut self.u_vals, u_vals);
        std::mem::swap(&mut self.u_diag_recip, u_diag_recip);
        self.use_diag_recip = use_diag_recip;
        self.diagnostics = KluDiagnostics {
            reciprocal_pivot_growth: if reciprocal_pivot_growth.is_finite() {
                reciprocal_pivot_growth
            } else {
                0.0
            },
            diagonal_rcond: if max_abs_pivot > 0.0 {
                min_abs_pivot / max_abs_pivot
            } else {
                0.0
            },
            min_abs_pivot,
            max_abs_pivot,
        };
        self.factored = true;
        Ok(())
    }

    /// Scatter slot for an original row during `factor`: pivoted rows
    /// live at their pivot position (`< n`), unpivoted rows in the upper
    /// half (`n + original`), so the two index spaces cannot collide.
    #[inline]
    fn x_slot(n: usize, pinv: &[usize], row: usize) -> usize {
        let p = pinv[row];
        if p == usize::MAX { n + row } else { p }
    }

    /// Depth-first reach over L's columns, marking visited rows and
    /// recording pivoted reach in postorder.
    #[allow(clippy::too_many_arguments)]
    fn dfs_reach(
        n: usize,
        start_row: usize,
        stamp: usize,
        flag: &mut [usize],
        pinv: &[usize],
        l_ptr: &[usize],
        l_rows: &[u32],
        dfs_stack: &mut Vec<(usize, usize)>,
        topo: &mut Vec<usize>,
        nonpivot_rows: &mut Vec<usize>,
        x: &mut [Value],
    ) {
        debug_assert!(dfs_stack.is_empty());
        flag[start_row] = stamp;
        dfs_stack.push((start_row, 0));

        while let Some(&mut (row, ref mut child)) = dfs_stack.last_mut() {
            let k = pinv[row];
            if k == usize::MAX {
                // Unpivoted row: a leaf of the reach.
                x[n + row] = 0.0;
                nonpivot_rows.push(row);
                dfs_stack.pop();
                continue;
            }
            let begin = l_ptr[k];
            let end = l_ptr[k + 1];
            let mut advanced = false;
            while begin + *child < end {
                let next = l_rows[begin + *child] as usize;
                *child += 1;
                if flag[next] != stamp {
                    flag[next] = stamp;
                    dfs_stack.push((next, 0));
                    advanced = true;
                    break;
                }
            }
            if !advanced {
                x[k] = 0.0;
                topo.push(k);
                dfs_stack.pop();
            }
        }
    }

    /// Values-only refactorization over the frozen pattern and pivots.
    /// Returns `PivotGrowth` when a stored pivot has become numerically
    /// inadequate — the caller refactors fully.
    pub fn refactor(&mut self, values: &[Value]) -> Result<(), SolverError> {
        if self.absolute_pivot_tolerance > 0.0 {
            self.refactor_impl::<true>(values)
        } else {
            self.refactor_impl::<false>(values)
        }
    }

    fn refactor_impl<const CHECK_ABSOLUTE_PIVOT: bool>(
        &mut self,
        values: &[Value],
    ) -> Result<(), SolverError> {
        if !self.factored {
            return self.factor(values);
        }
        if values.len() != self.a_rows.len() {
            self.factored = false;
            return Err(SolverError::InvalidCircuit(format!(
                "KLU values length {} does not match analyzed nonzero count {}",
                values.len(),
                self.a_rows.len()
            )));
        }
        // Keep the factor-time row scaling during values-only refactors. Any
        // nonzero diagonal scaling remains algebraically exact; large numeric
        // drift is caught by the pivot-growth test and triggers a fresh factor
        // (which recomputes max-row scaling). This avoids an extra random row
        // scatter over A on every Newton iteration.
        let n = self.n;
        let absolute_pivot_tolerance = self.absolute_pivot_tolerance;
        let x = &mut self.work;
        let col_ptr = &self.a_col_ptr;
        let mut use_diag_recip = true;
        let reciprocal_pivot_growth = self.diagnostics.reciprocal_pivot_growth;
        let mut min_abs_pivot = Value::INFINITY;
        let mut max_abs_pivot: Value = 0.0;

        for j in 0..n {
            let a_col = self.col_perm[j] as usize;
            // Scatter A's column into pivot space through the precomputed
            // targets — one u32 load + one store per nonzero, no
            // row-index load, no permutation lookup.
            let (a_begin, a_end) = (col_ptr[a_col], col_ptr[a_col + 1]);
            for ((&slot, &scale), &v) in self.a_scatter[a_begin..a_end]
                .iter()
                .zip(&self.a_entry_scale[a_begin..a_end])
                .zip(&values[a_begin..a_end])
            {
                if !v.is_finite() {
                    self.factored = false;
                    return Err(SolverError::Overflow);
                }
                let scaled = v * scale;
                if !scaled.is_finite() || (scaled == 0.0 && v != 0.0) {
                    self.factored = false;
                    return Err(SolverError::PivotGrowth);
                }
                x[slot as usize] = scaled;
            }

            let u_begin = self.u_col_ptr[j];
            let u_end = self.u_col_ptr[j + 1];
            // Off-diagonal U entries replay in stored elimination order;
            // the diagonal is the final slot.
            let mut col_max = 0.0_f64;
            for ui in u_begin..u_end - 1 {
                let k = self.u_rows[ui] as usize;
                let alpha = x[k];
                x[k] = 0.0;
                if !alpha.is_finite() {
                    self.factored = false;
                    return Err(SolverError::Overflow);
                }
                self.u_vals[ui] = alpha;
                col_max = col_max.max(alpha.abs());
                if alpha != 0.0 {
                    let (ls, le) = (self.l_col_ptr[k], self.l_col_ptr[k + 1]);
                    // Paired slice iteration elides the per-element bounds
                    // checks on the index/value arrays. (A 2-way manual
                    // unroll was measured and rejected: −3% on the long
                    // expander columns but +7-9% on circuit-typical 3-5
                    // entry columns, where chunking overhead dominates.)
                    for (&row, &lv) in self.l_rows[ls..le].iter().zip(&self.l_vals[ls..le]) {
                        let updated = x[row as usize] - alpha * lv;
                        if !updated.is_finite() {
                            self.factored = false;
                            return Err(SolverError::Overflow);
                        }
                        x[row as usize] = updated;
                    }
                }
            }

            let pivot = x[j];
            x[j] = 0.0;
            let (ls, le) = (self.l_col_ptr[j], self.l_col_ptr[j + 1]);
            {
                let rows = &self.l_rows[ls..le];
                let vals = &mut self.l_vals[ls..le];
                for (&row, slot) in rows.iter().zip(vals.iter_mut()) {
                    let r = row as usize;
                    let v = x[r];
                    x[r] = 0.0;
                    if !v.is_finite() {
                        self.factored = false;
                        return Err(SolverError::Overflow);
                    }
                    col_max = col_max.max(v.abs());
                    *slot = v; // scaled below once the pivot is validated
                }
            }
            col_max = col_max.max(pivot.abs());

            if !pivot.is_finite() {
                self.factored = false;
                return Err(SolverError::Overflow);
            }
            if pivot == 0.0 || pivot.abs() < REFACTOR_PIVOT_TOL * col_max {
                self.factored = false;
                return Err(SolverError::PivotGrowth);
            }
            let pivot_abs = pivot.abs();
            if CHECK_ABSOLUTE_PIVOT {
                let original_row = self.row_perm[j] as usize;
                if pivot_abs / self.row_scale[original_row] < absolute_pivot_tolerance {
                    self.factored = false;
                    return Err(SolverError::PivotGrowth);
                }
            }
            min_abs_pivot = min_abs_pivot.min(pivot_abs);
            max_abs_pivot = max_abs_pivot.max(pivot_abs);
            self.u_vals[u_end - 1] = pivot;
            // One divide per column; the contiguous multiplies vectorize.
            let pivot_recip = 1.0 / pivot;
            use_diag_recip &= pivot_recip.is_finite();
            self.u_diag_recip[j] = pivot_recip;
            if pivot_recip.is_finite() {
                for value in &mut self.l_vals[ls..le] {
                    *value *= pivot_recip;
                }
            } else {
                for value in &mut self.l_vals[ls..le] {
                    *value /= pivot;
                }
            }
        }
        self.use_diag_recip = use_diag_recip;
        self.diagnostics = KluDiagnostics {
            reciprocal_pivot_growth,
            diagonal_rcond: if max_abs_pivot > 0.0 {
                min_abs_pivot / max_abs_pivot
            } else {
                0.0
            },
            min_abs_pivot,
            max_abs_pivot,
        };
        Ok(())
    }

    /// Solve `A x = b` with the current factorization. Allocation-free
    /// after warmup: the un-permutation stages through the persistent
    /// scratch instead of cloning.
    pub fn solve(&mut self, b: &[Value], out: &mut Vec<Value>) -> Result<(), SolverError> {
        if !self.factored {
            return Err(SolverError::SingularMatrix);
        }
        let n = self.n;
        if b.len() != n {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU right-hand side length {} does not match matrix dimension {}",
                b.len(),
                n
            )));
        }
        let x = &mut self.work;

        // Permute b into pivot space: pivot position k reads original row.
        for (slot, &row) in x[..n].iter_mut().zip(&self.row_perm) {
            let row = row as usize;
            let scaled = b[row] * self.row_scale[row];
            if !scaled.is_finite() {
                return Err(SolverError::Overflow);
            }
            *slot = scaled;
        }
        // Forward solve L y = Pb (unit diagonal).
        for k in 0..n {
            let xk = x[k];
            if xk != 0.0 {
                let (ls, le) = (self.l_col_ptr[k], self.l_col_ptr[k + 1]);
                for (&row, &lv) in self.l_rows[ls..le].iter().zip(&self.l_vals[ls..le]) {
                    x[row as usize] -= xk * lv;
                }
            }
        }
        // Back solve U z = y; columns hold the diagonal last. Ordinary
        // circuit pivots use cached reciprocals, avoiding one serial hardware
        // divide per unknown. Keep an exact-division path for subnormal pivots
        // whose reciprocal overflows even though the original system may be
        // representable.
        if self.use_diag_recip {
            for j in (0..n).rev() {
                let u_begin = self.u_col_ptr[j];
                let u_end = self.u_col_ptr[j + 1];
                let zj = x[j] * self.u_diag_recip[j];
                if !zj.is_finite() {
                    return Err(SolverError::Overflow);
                }
                x[j] = zj;
                if zj != 0.0 {
                    for (&row, &uv) in self.u_rows[u_begin..u_end - 1]
                        .iter()
                        .zip(&self.u_vals[u_begin..u_end - 1])
                    {
                        x[row as usize] -= zj * uv;
                    }
                }
            }
        } else {
            for j in (0..n).rev() {
                let u_begin = self.u_col_ptr[j];
                let u_end = self.u_col_ptr[j + 1];
                let diag = self.u_vals[u_end - 1];
                let zj = x[j] / diag;
                if !zj.is_finite() {
                    return Err(SolverError::Overflow);
                }
                x[j] = zj;
                if zj != 0.0 {
                    for (&row, &uv) in self.u_rows[u_begin..u_end - 1]
                        .iter()
                        .zip(&self.u_vals[u_begin..u_end - 1])
                    {
                        x[row as usize] -= zj * uv;
                    }
                }
            }
        }
        // Un-permute columns into the output: the solution component for
        // original column `col_perm[j]` is z[j].
        resize_fallible(out, n, 0.0)?;
        for (&col, &zj) in self.col_perm.iter().zip(&x[..n]) {
            out[col as usize] = zj;
        }
        Ok(())
    }

    /// Solve `A^T x = b` with the current factorization. The transposed
    /// triangular kernels reuse the same factors and perform no refactor.
    pub fn solve_transpose(
        &mut self,
        b: &[Value],
        out: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_many_impl(b, 1, out, true)
    }

    /// Solve `A X = B` for column-major right-hand sides. `b` and `out` use
    /// dense column-major layout with `n` consecutive values per RHS.
    pub fn solve_many(
        &mut self,
        b: &[Value],
        rhs_count: usize,
        out: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_many_impl(b, rhs_count, out, false)
    }

    /// Solve `A^T X = B` for column-major right-hand sides.
    pub fn solve_many_transpose(
        &mut self,
        b: &[Value],
        rhs_count: usize,
        out: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_many_impl(b, rhs_count, out, true)
    }

    fn solve_many_impl(
        &mut self,
        b: &[Value],
        rhs_count: usize,
        out: &mut Vec<Value>,
        transpose: bool,
    ) -> Result<(), SolverError> {
        if !self.factored {
            return Err(SolverError::SingularMatrix);
        }
        let required = self.n.checked_mul(rhs_count).ok_or_else(|| {
            SolverError::InvalidCircuit("KLU right-hand-side size overflow".to_string())
        })?;
        if b.len() != required {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU batched RHS length {} does not match {}x{}",
                b.len(),
                self.n,
                rhs_count
            )));
        }
        if rhs_count == 0 {
            out.clear();
            return Ok(());
        }

        let n = self.n;
        let work = &mut self.batch_work;
        resize_fallible(work, required, 0.0)?;
        if transpose {
            // P_c^T b. Work is pivot-major so values for every RHS touched by
            // a sparse edge are contiguous and the inner loops vectorize.
            for (j, &col) in self.col_perm.iter().enumerate() {
                let col = col as usize;
                for rhs in 0..rhs_count {
                    let value = b[rhs * n + col];
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    work[j * rhs_count + rhs] = value;
                }
            }

            // U^T y = P_c^T b.
            for j in 0..n {
                let (us, ue) = (self.u_col_ptr[j], self.u_col_ptr[j + 1]);
                let target = j * rhs_count;
                for (&row, &uv) in self.u_rows[us..ue - 1].iter().zip(&self.u_vals[us..ue - 1]) {
                    let source = row as usize * rhs_count;
                    for rhs in 0..rhs_count {
                        work[target + rhs] -= uv * work[source + rhs];
                    }
                }
                for rhs in 0..rhs_count {
                    let solved = if self.use_diag_recip {
                        work[target + rhs] * self.u_diag_recip[j]
                    } else {
                        work[target + rhs] / self.u_vals[ue - 1]
                    };
                    if !solved.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    work[target + rhs] = solved;
                }
            }
            // L^T w = y (unit diagonal).
            for j in (0..n).rev() {
                let (ls, le) = (self.l_col_ptr[j], self.l_col_ptr[j + 1]);
                let target = j * rhs_count;
                for (&row, &lv) in self.l_rows[ls..le].iter().zip(&self.l_vals[ls..le]) {
                    let source = row as usize * rhs_count;
                    for rhs in 0..rhs_count {
                        work[target + rhs] -= lv * work[source + rhs];
                    }
                }
            }

            resize_fallible(out, required, 0.0)?;
            for (j, &row) in self.row_perm.iter().enumerate() {
                let row = row as usize;
                let scale = self.row_scale[row];
                for rhs in 0..rhs_count {
                    let value = work[j * rhs_count + rhs] * scale;
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    out[rhs * n + row] = value;
                }
            }
        } else {
            // P_r D_r B.
            for (j, &row) in self.row_perm.iter().enumerate() {
                let row = row as usize;
                let scale = self.row_scale[row];
                for rhs in 0..rhs_count {
                    let value = b[rhs * n + row] * scale;
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    work[j * rhs_count + rhs] = value;
                }
            }
            // L Y = P_r D_r B.
            for j in 0..n {
                let (ls, le) = (self.l_col_ptr[j], self.l_col_ptr[j + 1]);
                for (&row, &lv) in self.l_rows[ls..le].iter().zip(&self.l_vals[ls..le]) {
                    let target = row as usize * rhs_count;
                    let source = j * rhs_count;
                    for rhs in 0..rhs_count {
                        work[target + rhs] -= work[source + rhs] * lv;
                    }
                }
            }
            // U Z = Y.
            for j in (0..n).rev() {
                let (us, ue) = (self.u_col_ptr[j], self.u_col_ptr[j + 1]);
                let source = j * rhs_count;
                for rhs in 0..rhs_count {
                    let solved = if self.use_diag_recip {
                        work[source + rhs] * self.u_diag_recip[j]
                    } else {
                        work[source + rhs] / self.u_vals[ue - 1]
                    };
                    if !solved.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    work[source + rhs] = solved;
                }
                for (&row, &uv) in self.u_rows[us..ue - 1].iter().zip(&self.u_vals[us..ue - 1]) {
                    let target = row as usize * rhs_count;
                    for rhs in 0..rhs_count {
                        work[target + rhs] -= work[source + rhs] * uv;
                    }
                }
            }

            resize_fallible(out, required, 0.0)?;
            for (j, &col) in self.col_perm.iter().enumerate() {
                let col = col as usize;
                for rhs in 0..rhs_count {
                    out[rhs * n + col] = work[j * rhs_count + rhs];
                }
            }
        }
        Ok(())
    }
}

/// Validate the complete structural contract required by the KLU kernels.
///
/// In particular, rows must be strictly increasing within each column. The
/// refactor scatter uses assignment rather than accumulation, so admitting
/// duplicate coordinates would make the first factorization and later
/// refactorizations represent different matrices.
fn validate_csc_pattern(n: usize, col_ptr: &[usize], row_idx: &[usize]) -> Result<(), SolverError> {
    if n > u32::MAX as usize {
        return Err(SolverError::InvalidCircuit(format!(
            "KLU matrix dimension {n} exceeds the u32 kernel index limit"
        )));
    }
    let expected_col_ptr = n
        .checked_add(1)
        .ok_or_else(|| SolverError::InvalidCircuit("KLU matrix dimension overflow".to_string()))?;
    if col_ptr.len() != expected_col_ptr {
        return Err(SolverError::InvalidCircuit(format!(
            "KLU column-pointer length {} does not match dimension {}",
            col_ptr.len(),
            n
        )));
    }
    if col_ptr.first().copied() != Some(0) {
        return Err(SolverError::InvalidCircuit(
            "KLU column pointers must start at zero".to_string(),
        ));
    }
    if col_ptr.last().copied() != Some(row_idx.len()) {
        return Err(SolverError::InvalidCircuit(format!(
            "KLU final column pointer {:?} does not match row-index length {}",
            col_ptr.last(),
            row_idx.len()
        )));
    }
    for col in 0..n {
        let begin = col_ptr[col];
        let end = col_ptr[col + 1];
        if begin > end || end > row_idx.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "KLU column {col} has invalid range {begin}..{end} for {} row indices",
                row_idx.len()
            )));
        }
        let mut previous = None;
        for &row in &row_idx[begin..end] {
            if row >= n {
                return Err(SolverError::InvalidCircuit(format!(
                    "KLU row index {row} in column {col} exceeds dimension {n}"
                )));
            }
            if previous.is_some_and(|value| row <= value) {
                return Err(SolverError::InvalidCircuit(format!(
                    "KLU row indices in column {col} must be strictly increasing"
                )));
            }
            previous = Some(row);
        }
    }
    Ok(())
}

struct BtfOrder {
    col_perm: Vec<usize>,
    matched_row_for_col: Vec<u32>,
    row_block: Vec<u32>,
    col_block: Vec<u32>,
    block_count: usize,
}

/// Find a structural transversal with a greedy diagonal-first seed followed
/// by explicit-stack augmenting paths. The common MNA case matches almost
/// entirely in the two linear seed passes; the augmenting phase handles ideal
/// source rows and other missing-diagonal structures without recursion.
fn structural_matching(
    n: usize,
    col_ptr: &[usize],
    row_idx: &[usize],
) -> Option<(Vec<usize>, Vec<usize>)> {
    let unmatched = usize::MAX;
    let mut matched_row_for_col = vec![unmatched; n];
    let mut matched_col_for_row = vec![unmatched; n];

    // Preserve ordinary circuit diagonals whenever they are structurally
    // available; this minimizes augmenting work and keeps pivots intuitive.
    for col in 0..n {
        if row_idx[col_ptr[col]..col_ptr[col + 1]]
            .binary_search(&col)
            .is_ok()
            && matched_col_for_row[col] == unmatched
        {
            matched_row_for_col[col] = col;
            matched_col_for_row[col] = col;
        }
    }
    // Greedily consume any free row for columns without a diagonal match.
    for col in 0..n {
        if matched_row_for_col[col] != unmatched {
            continue;
        }
        if let Some(&row) = row_idx[col_ptr[col]..col_ptr[col + 1]]
            .iter()
            .find(|&&row| matched_col_for_row[row] == unmatched)
        {
            matched_row_for_col[col] = row;
            matched_col_for_row[row] = col;
        }
    }

    let mut seen_rows = vec![0usize; n];
    let mut seen_cols = vec![0usize; n];
    let mut parent_col_for_row = vec![unmatched; n];
    let mut stack = Vec::<(usize, usize)>::new();
    let work_limit = row_idx.len().saturating_mul(8).max(n);
    let mut work = 0usize;
    for start_col in 0..n {
        if matched_row_for_col[start_col] != unmatched {
            continue;
        }
        let stamp = start_col.saturating_add(1);
        stack.clear();
        seen_cols[start_col] = stamp;
        stack.push((start_col, col_ptr[start_col]));
        let mut augmented = false;

        while let Some((col, next_entry)) = stack.last_mut() {
            work = work.saturating_add(1);
            if work > work_limit {
                return None;
            }
            let col = *col;
            if *next_entry >= col_ptr[col + 1] {
                stack.pop();
                continue;
            }
            let row = row_idx[*next_entry];
            *next_entry += 1;
            if seen_rows[row] == stamp {
                continue;
            }
            seen_rows[row] = stamp;
            parent_col_for_row[row] = col;

            let next_col = matched_col_for_row[row];
            if next_col == unmatched {
                // Flip the alternating path from the free row back to the
                // unmatched start column.
                let mut free_row = row;
                loop {
                    let path_col = parent_col_for_row[free_row];
                    let previous_row = matched_row_for_col[path_col];
                    matched_row_for_col[path_col] = free_row;
                    matched_col_for_row[free_row] = path_col;
                    if previous_row == unmatched {
                        break;
                    }
                    free_row = previous_row;
                }
                augmented = true;
                break;
            }
            if seen_cols[next_col] != stamp {
                seen_cols[next_col] = stamp;
                stack.push((next_col, col_ptr[next_col]));
            }
        }
        if !augmented {
            return None;
        }
    }

    Some((matched_row_for_col, matched_col_for_row))
}

#[derive(Debug)]
struct FlatGraph {
    ptr: Vec<usize>,
    nodes: Vec<usize>,
}

impl FlatGraph {
    #[inline]
    fn len(&self) -> usize {
        self.ptr.len().saturating_sub(1)
    }

    #[inline]
    fn neighbors(&self, node: usize) -> &[usize] {
        &self.nodes[self.ptr[node]..self.ptr[node + 1]]
    }

    fn transpose(&self) -> Self {
        let n = self.len();
        let mut ptr = vec![0usize; n + 1];
        for &target in &self.nodes {
            ptr[target + 1] += 1;
        }
        for node in 0..n {
            ptr[node + 1] += ptr[node];
        }
        let mut cursor = ptr[..n].to_vec();
        let mut nodes = vec![0usize; self.nodes.len()];
        for source in 0..n {
            for &target in self.neighbors(source) {
                nodes[cursor[target]] = source;
                cursor[target] += 1;
            }
        }
        Self { ptr, nodes }
    }
}

/// Iterative Kosaraju SCC decomposition over flat CSR storage. Avoiding one
/// heap allocation per graph node matters for large reducible MNA systems.
fn strongly_connected_components(adjacency: &FlatGraph) -> (Vec<usize>, usize) {
    let n = adjacency.len();
    let reverse = adjacency.transpose();

    let mut seen = vec![false; n];
    let mut finish = Vec::with_capacity(n);
    let mut dfs = Vec::<(usize, usize)>::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        dfs.push((start, adjacency.ptr[start]));
        while let Some((node, next)) = dfs.last_mut() {
            if *next < adjacency.ptr[*node + 1] {
                let target = adjacency.nodes[*next];
                *next += 1;
                if !seen[target] {
                    seen[target] = true;
                    dfs.push((target, adjacency.ptr[target]));
                }
            } else {
                if let Some((finished, _)) = dfs.pop() {
                    finish.push(finished);
                }
            }
        }
    }

    let mut component_of = vec![usize::MAX; n];
    let mut stack = Vec::new();
    let mut component_count = 0;
    for &start in finish.iter().rev() {
        if component_of[start] != usize::MAX {
            continue;
        }
        let component = component_count;
        component_count += 1;
        component_of[start] = component;
        stack.push(start);
        while let Some(node) = stack.pop() {
            for &target in reverse.neighbors(node) {
                if component_of[target] == usize::MAX {
                    component_of[target] = component;
                    stack.push(target);
                }
            }
        }
    }
    (component_of, component_count)
}

/// KLU symbolic ordering: maximum transversal, BTF, then AMD within blocks.
fn btf_amd_order(
    n: usize,
    col_ptr: &[usize],
    row_idx: &[usize],
) -> Result<Option<BtfOrder>, SolverError> {
    if n == 0 {
        return Ok(Some(BtfOrder {
            col_perm: Vec::new(),
            matched_row_for_col: Vec::new(),
            row_block: Vec::new(),
            col_block: Vec::new(),
            block_count: 0,
        }));
    }
    let Some((matched_row_for_col, matched_col_for_row)) = structural_matching(n, col_ptr, row_idx)
    else {
        return Ok(None);
    };

    let mut adjacency_ptr = vec![0usize; n + 1];
    for col in 0..n {
        for &row in &row_idx[col_ptr[col]..col_ptr[col + 1]] {
            let target = matched_col_for_row[row];
            if target != col {
                adjacency_ptr[col + 1] += 1;
            }
        }
    }
    for col in 0..n {
        adjacency_ptr[col + 1] += adjacency_ptr[col];
    }
    let mut adjacency_nodes = Vec::with_capacity(adjacency_ptr[n]);
    for col in 0..n {
        for &row in &row_idx[col_ptr[col]..col_ptr[col + 1]] {
            let target = matched_col_for_row[row];
            if target != col {
                adjacency_nodes.push(target);
            }
        }
    }
    let adjacency = FlatGraph {
        ptr: adjacency_ptr,
        nodes: adjacency_nodes,
    };
    let (component_of, block_count) = strongly_connected_components(&adjacency);
    let mut component_ptr = vec![0usize; block_count + 1];
    for &component in &component_of {
        component_ptr[component + 1] += 1;
    }
    for component in 0..block_count {
        component_ptr[component + 1] += component_ptr[component];
    }
    let mut component_cursor = component_ptr[..block_count].to_vec();
    let mut component_nodes = vec![0usize; n];
    for (col, &component) in component_of.iter().enumerate() {
        component_nodes[component_cursor[component]] = col;
        component_cursor[component] += 1;
    }
    let mut col_block = vec![0u32; n];
    let mut row_block = vec![0u32; n];
    let mut col_perm = Vec::with_capacity(n);
    let mut local_position = vec![usize::MAX; n];
    let mut local_col_ptr = Vec::new();
    let mut local_rows = Vec::new();

    for block in 0..block_count {
        let nodes = &mut component_nodes[component_ptr[block]..component_ptr[block + 1]];
        nodes.sort_unstable();
        for (local, &col) in nodes.iter().enumerate() {
            local_position[col] = local;
            col_block[col] = block as u32;
            row_block[matched_row_for_col[col]] = block as u32;
        }

        if nodes.len() == 1 {
            col_perm.push(nodes[0]);
        } else {
            local_col_ptr.clear();
            local_rows.clear();
            reserve_for(&mut local_col_ptr, nodes.len().saturating_add(1))?;
            let block_input_nnz = nodes.iter().fold(0usize, |count, &col| {
                count.saturating_add(col_ptr[col + 1].saturating_sub(col_ptr[col]))
            });
            reserve_for(&mut local_rows, block_input_nnz)?;
            local_col_ptr.push(0);
            for &original_col in nodes.iter() {
                let local_begin = local_rows.len();
                for &row in &row_idx[col_ptr[original_col]..col_ptr[original_col + 1]] {
                    let target_col = matched_col_for_row[row];
                    if component_of[target_col] == component_of[original_col] {
                        local_rows.push(local_position[target_col]);
                    }
                }
                local_rows[local_begin..].sort_unstable();
                local_col_ptr.push(local_rows.len());
            }
            let local_order = amd_order(nodes.len(), &local_col_ptr, &local_rows)?;
            col_perm.extend(local_order.into_iter().map(|local| nodes[local]));
        }

        for &col in nodes.iter() {
            local_position[col] = usize::MAX;
        }
    }

    Ok(Some(BtfOrder {
        col_perm,
        matched_row_for_col: matched_row_for_col
            .into_iter()
            .map(|row| row as u32)
            .collect(),
        row_block,
        col_block,
        block_count,
    }))
}

/// AMD ordering on the symmetrized pattern via faer (the fill-reducing
/// ordering KLU applies inside its blocks). Returns the permutation as
/// "step k eliminates original column `perm[k]`", or `None` when the
/// pattern is rejected (caller falls back to natural order).
fn amd_order(n: usize, col_ptr: &[usize], row_idx: &[usize]) -> Result<Vec<usize>, SolverError> {
    use faer::dyn_stack::{MemBuffer, MemStack};
    use faer::sparse::SymbolicSparseColMatRef;
    use faer::sparse::linalg::amd;

    if n == 0 {
        return Ok(Vec::new());
    }
    let csc = SymbolicSparseColMatRef::new_checked(n, n, col_ptr, None, row_idx);

    let mut perm = Vec::new();
    resize_fallible(&mut perm, n, 0usize)?;
    let mut perm_inv = Vec::new();
    resize_fallible(&mut perm_inv, n, 0usize)?;
    let mut mem = MemBuffer::try_new(amd::order_scratch::<usize>(n, row_idx.len()))
        .map_err(|_| SolverError::OutOfMemory)?;
    amd::order(
        &mut perm,
        &mut perm_inv,
        csc,
        amd::Control::default(),
        MemStack::new(&mut mem),
    )
    .map_err(|error| match error {
        faer::sparse::FaerError::OutOfMemory => SolverError::OutOfMemory,
        faer::sparse::FaerError::IndexOverflow => {
            SolverError::InvalidCircuit("AMD ordering exceeded the backend index range".to_string())
        }
        _ => SolverError::InvalidCircuit("AMD ordering failed".to_string()),
    })?;
    Ok(perm)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Dense reference solve (partial pivoting) for validation.
    fn dense_solve(n: usize, a: &[Vec<Value>], b: &[Value]) -> Option<Vec<Value>> {
        let mut m: Vec<Vec<Value>> = a.to_vec();
        let mut x: Vec<Value> = b.to_vec();
        for col in 0..n {
            let piv =
                (col..n).max_by(|&i, &j| m[i][col].abs().partial_cmp(&m[j][col].abs()).unwrap())?;
            if m[piv][col].abs() < 1e-300 {
                return None;
            }
            m.swap(col, piv);
            x.swap(col, piv);
            for row in col + 1..n {
                let f = m[row][col] / m[col][col];
                if f != 0.0 {
                    for k in col..n {
                        m[row][k] -= f * m[col][k];
                    }
                    x[row] -= f * x[col];
                }
            }
        }
        for col in (0..n).rev() {
            x[col] /= m[col][col];
            for row in 0..col {
                let f = m[row][col];
                x[row] -= f * x[col];
                m[row][col] = 0.0;
            }
        }
        Some(x)
    }

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v >> 12;
            v ^= v << 25;
            v ^= v >> 27;
            self.0 = v;
            v.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        fn unit(&mut self) -> Value {
            (self.next() >> 11) as Value / (1u64 << 53) as Value
        }
    }

    /// Random circuit-like sparse matrix: dominant diagonal plus a few
    /// symmetric-ish couplings and one asymmetric entry per column.
    fn random_system(
        rng: &mut Rng,
        n: usize,
    ) -> (Vec<usize>, Vec<usize>, Vec<Value>, Vec<Vec<Value>>) {
        let mut dense = vec![vec![0.0; n]; n];
        for j in 0..n {
            dense[j][j] = 1.0 + 4.0 * rng.unit();
            let couplings = 1 + (rng.next() as usize % 3);
            for _ in 0..couplings {
                let i = rng.next() as usize % n;
                if i != j {
                    let v = (rng.unit() - 0.5) * 1.5;
                    dense[i][j] += v;
                    dense[j][i] += v * (0.5 + rng.unit()); // asymmetric
                }
            }
        }
        // CSC of the dense reference.
        let mut col_ptr = vec![0usize];
        let mut rows = Vec::new();
        let mut vals = Vec::new();
        for j in 0..n {
            for i in 0..n {
                if dense[i][j] != 0.0 {
                    rows.push(i);
                    vals.push(dense[i][j]);
                }
            }
            col_ptr.push(rows.len());
        }
        (col_ptr, rows, vals, dense)
    }

    fn assert_close(a: &[Value], b: &[Value]) {
        for (x, y) in a.iter().zip(b) {
            let scale = x.abs().max(y.abs()).max(1.0);
            assert!(
                (x - y).abs() <= 1e-9 * scale,
                "solutions diverge: {x} vs {y}"
            );
        }
    }

    #[test]
    fn factor_solve_matches_dense_reference() {
        let mut rng = Rng(0xC1AC_0001);
        for trial in 0..60 {
            let n = 2 + (rng.next() as usize % 40);
            let (col_ptr, rows, vals, dense) = random_system(&mut rng, n);
            let b: Vec<Value> = (0..n).map(|_| rng.unit() * 2.0 - 1.0).collect();
            let expected = dense_solve(n, &dense, &b).expect("dense solvable");

            let mut klu = KluSolver::new();
            klu.analyze(n, &col_ptr, &rows).expect("analyze");
            klu.factor(&vals)
                .unwrap_or_else(|e| panic!("factor failed on trial {trial}: {e:?}"));
            let mut out = Vec::new();
            klu.solve(&b, &mut out).expect("solve");
            assert_close(&out, &expected);
        }
    }

    #[test]
    fn transpose_and_batched_solves_match_dense_references() {
        let mut rng = Rng(0xC1AC_0003);
        for _ in 0..30 {
            let n = 2 + (rng.next() as usize % 30);
            let (col_ptr, rows, vals, dense) = random_system(&mut rng, n);
            let transpose = (0..n)
                .map(|row| (0..n).map(|col| dense[col][row]).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let rhs_count = 4;
            let mut rhs = Vec::with_capacity(n * rhs_count);
            let mut normal_expected = Vec::with_capacity(n * rhs_count);
            let mut transpose_expected = Vec::with_capacity(n * rhs_count);
            for _ in 0..rhs_count {
                let one_rhs = (0..n).map(|_| rng.unit() * 2.0 - 1.0).collect::<Vec<_>>();
                normal_expected.extend(dense_solve(n, &dense, &one_rhs).unwrap());
                transpose_expected.extend(dense_solve(n, &transpose, &one_rhs).unwrap());
                rhs.extend(one_rhs);
            }

            let mut klu = KluSolver::new();
            klu.analyze(n, &col_ptr, &rows).unwrap();
            klu.factor(&vals).unwrap();

            let mut actual = Vec::new();
            klu.solve_many(&rhs, rhs_count, &mut actual).unwrap();
            assert_close(&actual, &normal_expected);
            klu.solve_many_transpose(&rhs, rhs_count, &mut actual)
                .unwrap();
            assert_close(&actual, &transpose_expected);
            klu.solve_transpose(&rhs[..n], &mut actual).unwrap();
            assert_close(&actual, &transpose_expected[..n]);
        }
    }

    #[test]
    fn on_demand_condition_estimate_tracks_diagonal_conditioning() {
        let mut klu = KluSolver::new();
        klu.analyze(2, &[0, 1, 2], &[0, 1]).unwrap();
        klu.factor(&[2.0, 4.0]).unwrap();
        let rcond = klu.estimate_rcond(&[2.0, 4.0]).unwrap();
        assert!((rcond - 0.5).abs() <= 16.0 * Value::EPSILON);

        klu.refactor(&[1.0e-12, 1.0]).unwrap();
        let ill_rcond = klu.estimate_rcond(&[1.0e-12, 1.0]).unwrap();
        assert!((ill_rcond - 1.0e-12).abs() <= 1.0e-24);
    }

    #[test]
    fn refactor_tracks_changing_values() {
        let mut rng = Rng(0xC1AC_0002);
        for _ in 0..25 {
            let n = 3 + (rng.next() as usize % 30);
            let (col_ptr, rows, mut vals, mut dense) = random_system(&mut rng, n);
            let mut klu = KluSolver::new();
            klu.analyze(n, &col_ptr, &rows).expect("analyze");
            klu.factor(&vals).expect("factor");

            // Newton-like value drift on the same pattern.
            for _step in 0..6 {
                for (k, v) in vals.iter_mut().enumerate() {
                    *v *= 1.0 + 0.1 * (rng.unit() - 0.5);
                    let _ = k;
                }
                // Rebuild the dense mirror.
                for col in dense.iter_mut() {
                    for x in col.iter_mut() {
                        *x = 0.0;
                    }
                }
                for j in 0..n {
                    for idx in col_ptr[j]..col_ptr[j + 1] {
                        dense[rows[idx]][j] = vals[idx];
                    }
                }
                let b: Vec<Value> = (0..n).map(|_| rng.unit() * 2.0 - 1.0).collect();
                let expected = dense_solve(n, &dense, &b).expect("dense solvable");

                match klu.refactor(&vals) {
                    Ok(()) => {}
                    Err(SolverError::PivotGrowth) => {
                        klu.factor(&vals).expect("re-factor");
                    }
                    Err(e) => panic!("refactor error: {e:?}"),
                }
                let mut out = Vec::new();
                klu.solve(&b, &mut out).expect("solve");
                assert_close(&out, &expected);
            }
        }
    }

    #[test]
    fn singular_matrix_is_reported_not_panicked() {
        // Column of zeros.
        let n = 3;
        let col_ptr = vec![0, 1, 1, 2];
        let rows = vec![0, 2];
        let vals = vec![1.0, 1.0];
        let mut klu = KluSolver::new();
        klu.analyze(n, &col_ptr, &rows).expect("analyze");
        assert!(matches!(
            klu.factor(&vals),
            Err(SolverError::SingularMatrix)
        ));
    }

    #[test]
    fn malformed_patterns_are_rejected_without_panicking() {
        let mut klu = KluSolver::new();
        for (n, col_ptr, rows) in [
            (2, vec![0, 1], vec![0]),
            (2, vec![1, 1, 1], vec![0]),
            (2, vec![0, 2, 1], vec![0]),
            (2, vec![0, 1, 1], vec![2]),
            (2, vec![0, 2, 2], vec![0, 0]),
            (2, vec![0, 2, 2], vec![1, 0]),
        ] {
            assert!(
                matches!(
                    klu.analyze(n, &col_ptr, &rows),
                    Err(SolverError::InvalidCircuit(_))
                ),
                "accepted malformed CSC pattern: n={n}, col_ptr={col_ptr:?}, rows={rows:?}"
            );
        }

        klu.analyze(1, &[0, 1], &[0]).expect("valid analysis");
        klu.factor(&[2.0]).expect("valid factorization");
        assert!(klu.analyze(2, &[0, 1], &[0]).is_err());
        assert!(matches!(
            klu.solve(&[1.0], &mut Vec::new()),
            Err(SolverError::SingularMatrix)
        ));
        assert!(matches!(
            klu.factor(&[]),
            Err(SolverError::InvalidCircuit(_))
        ));
    }

    #[test]
    fn public_kernel_rejects_invalid_numeric_dimensions_and_values() {
        let mut klu = KluSolver::new();
        let col_ptr = [0, 1, 2];
        let rows = [0, 1];
        klu.analyze(2, &col_ptr, &rows).expect("analyze");

        assert!(matches!(
            klu.factor(&[1.0]),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            klu.factor(&[1.0, Value::NAN]),
            Err(SolverError::Overflow)
        ));

        klu.factor(&[2.0, 4.0]).expect("factor");
        assert!(matches!(
            klu.solve(&[1.0], &mut Vec::new()),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            klu.solve(&[1.0, Value::INFINITY], &mut Vec::new()),
            Err(SolverError::Overflow)
        ));
        assert!(matches!(
            klu.refactor(&[2.0]),
            Err(SolverError::InvalidCircuit(_))
        ));

        let mut lower = KluSolver::new();
        lower.analyze(2, &[0, 2, 3], &[0, 1, 1]).unwrap();
        lower.factor(&[1.0, 0.5, 1.0]).unwrap();
        assert!(matches!(
            lower.refactor(&[1.0, Value::NAN, 1.0]),
            Err(SolverError::Overflow)
        ));

        for invalid in [Value::NAN, 0.0, -1.0, 1.0 + Value::EPSILON] {
            assert!(matches!(
                lower.set_pivot_tolerance(invalid),
                Err(SolverError::InvalidCircuit(_))
            ));
        }
        for invalid in [Value::NAN, Value::INFINITY, -Value::EPSILON] {
            assert!(matches!(
                lower.set_absolute_pivot_tolerance(invalid),
                Err(SolverError::InvalidCircuit(_))
            ));
        }

        let mut absolute = KluSolver::new();
        absolute.analyze(1, &[0, 1], &[0]).unwrap();
        absolute.set_absolute_pivot_tolerance(1.0e-3).unwrap();
        assert!(matches!(
            absolute.factor(&[1.0e-4]),
            Err(SolverError::SingularMatrix)
        ));
        absolute.set_absolute_pivot_tolerance(0.0).unwrap();
        absolute.factor(&[1.0e-4]).unwrap();

        let mut default_solver = KluSolver::default();
        default_solver.analyze(1, &[0, 1], &[0]).unwrap();
        default_solver.factor(&[2.0]).unwrap();
        let mut solution = Vec::new();
        default_solver.solve(&[4.0], &mut solution).unwrap();
        assert_eq!(solution, [2.0]);
    }

    #[test]
    fn subnormal_input_is_safely_equilibrated_before_factorization() {
        let tiny = Value::from_bits(1);
        let mut klu = KluSolver::new();
        klu.analyze(1, &[0, 1], &[0]).expect("analyze");
        klu.factor(&[tiny]).expect("factor");
        assert!(klu.use_diag_recip);
        assert!(klu.row_scale[0].is_finite());

        let mut solution = Vec::new();
        klu.solve(&[tiny], &mut solution).expect("solve");
        assert_eq!(solution, [1.0]);
    }

    #[test]
    fn amd_ordering_keeps_banded_fill_near_minimal() {
        // Tridiagonal pattern: a good ordering factors with (near) zero
        // fill — L+U nonzeros stay close to the matrix's own count.
        let n: usize = 64;
        let mut col_ptr = vec![0usize];
        let mut rows = Vec::new();
        let mut vals = Vec::new();
        for j in 0..n {
            for i in j.saturating_sub(1)..=(j + 1).min(n - 1) {
                rows.push(i);
                vals.push(if i == j { 3.0 } else { -1.0 });
            }
            col_ptr.push(rows.len());
        }
        let a_nnz = vals.len();

        let mut klu = KluSolver::new();
        klu.analyze(n, &col_ptr, &rows).expect("analyze");
        klu.factor(&vals).expect("factor");
        let (l_nnz, u_nnz) = klu.factor_nnz();
        assert!(
            l_nnz + u_nnz <= a_nnz + n / 4,
            "banded fill blew up: {} from {}",
            l_nnz + u_nnz,
            a_nnz
        );
    }

    #[test]
    fn btf_separates_reducible_blocks_and_preserves_the_solution() {
        // Two irreducible 2x2 blocks with one-way coupling from the first
        // block into the second. A global ordering is unnecessary and can
        // introduce fill across the boundary; BTF must retain two blocks.
        let n = 4;
        let col_ptr = [0, 3, 6, 8, 10];
        let rows = [0, 1, 2, 0, 1, 3, 2, 3, 2, 3];
        let values = [4.0, -1.0, 0.5, -1.0, 4.0, 0.25, 3.0, -1.0, -1.0, 3.0];
        let dense = vec![
            vec![4.0, -1.0, 0.0, 0.0],
            vec![-1.0, 4.0, 0.0, 0.0],
            vec![0.5, 0.0, 3.0, -1.0],
            vec![0.0, 0.25, -1.0, 3.0],
        ];
        let rhs = [1.0, 2.0, 3.0, 4.0];
        let expected = dense_solve(n, &dense, &rhs).unwrap();

        let mut klu = KluSolver::new();
        klu.analyze(n, &col_ptr, &rows).unwrap();
        assert_eq!(klu.block_count(), 2);
        klu.factor(&values).unwrap();
        assert!(
            klu.factor_nnz().0 + klu.factor_nnz().1 <= values.len() + 1,
            "unexpected fill: {:?}",
            klu.factor_nnz()
        );
        let mut solution = Vec::new();
        klu.solve(&rhs, &mut solution).unwrap();
        assert_close(&solution, &expected);
    }

    #[test]
    fn structural_matching_handles_a_missing_original_diagonal() {
        let mut klu = KluSolver::new();
        klu.analyze(2, &[0, 1, 2], &[1, 0]).unwrap();
        assert_eq!(klu.block_count(), 2);
        klu.factor(&[3.0, 2.0]).unwrap();

        let mut solution = Vec::new();
        klu.solve(&[4.0, 9.0], &mut solution).unwrap();
        assert_close(&solution, &[3.0, 2.0]);
    }
}
