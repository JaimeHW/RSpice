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

/// KLU's default partial-pivoting tolerance: the diagonal entry is kept
/// as the pivot when `|diag| >= PIVOT_TOL * colmax`.
const PIVOT_TOL: Value = 1e-3;

/// Refactor pivot-growth alarm: a stored pivot whose magnitude falls
/// below this fraction of its column maximum invalidates the pivot
/// sequence (re-pivoting required).
const REFACTOR_GROWTH_TOL: Value = 1e-8;

/// Sparse LU with a reusable pivot sequence.
#[derive(Debug, Default)]
pub struct KluSolver {
    n: usize,
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
    col_perm: Vec<usize>,
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
    row_perm: Vec<usize>,
    /// Inverse of `row_perm` (original row -> pivot position).
    row_perm_inv: Vec<usize>,
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
    /// Scatter workspace (pivot space).
    work: Vec<Value>,
    /// Whether L/U currently hold a valid factorization.
    factored: bool,
}

impl KluSolver {
    /// Create an empty solver. Call [`Self::analyze`] once for a pattern, then
    /// [`Self::factor`] or [`Self::refactor`] before solving.
    pub fn new() -> Self {
        Self::default()
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
        self.row_perm_inv.clear();
        self.l_col_ptr.clear();
        self.l_rows.clear();
        self.l_vals.clear();
        self.u_col_ptr.clear();
        self.u_rows.clear();
        self.u_vals.clear();
        self.u_diag_recip.clear();
        self.a_scatter.clear();
        self.work.clear();
        self.use_diag_recip = false;
        self.factored = false;
    }

    /// Whether the symbolic analysis matches this pattern instance.
    pub(crate) fn is_analyzed_for(&self, n: usize) -> bool {
        self.n == n && self.a_col_ptr.len() == n.saturating_add(1)
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
        if let Some(order) = btf_amd_order(n, col_ptr, row_idx) {
            self.col_perm = order.col_perm;
            self.matched_row_for_col = order.matched_row_for_col;
            self.row_block = order.row_block;
            self.col_block = order.col_block;
            self.block_count = order.block_count;
        } else {
            self.col_perm = amd_order(n, col_ptr, row_idx).unwrap_or_else(|| (0..n).collect());
        }
        self.a_col_ptr.extend_from_slice(col_ptr);
        self.a_rows.extend(row_idx.iter().map(|&row| row as u32));
        self.work.resize(n, 0.0);
        self.work.fill(0.0);
        Ok(())
    }

    /// Full Gilbert–Peierls factorization with fresh pivot selection.
    pub fn factor(&mut self, values: &[Value]) -> Result<(), SolverError> {
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
        let n = self.n;
        let col_ptr = &self.a_col_ptr;
        let row_idx = &self.a_rows;
        // pinv[orig_row] = pivot position, or usize::MAX while unpivoted.
        let mut pinv = vec![usize::MAX; n];
        let mut p_row = vec![usize::MAX; n];

        // Working L/U with ORIGINAL row indices (remapped to pivot space
        // once the full pivot sequence is known).
        let mut l_ptr = Vec::with_capacity(n + 1);
        let mut l_rows: Vec<usize> = Vec::new();
        let mut l_vals: Vec<Value> = Vec::new();
        let mut u_ptr = Vec::with_capacity(n + 1);
        let mut u_pos: Vec<usize> = Vec::new(); // pivot positions (already final)
        let mut u_vals: Vec<Value> = Vec::new();
        let mut u_diag_recip = Vec::with_capacity(n);
        let mut use_diag_recip = true;
        l_ptr.push(0);
        u_ptr.push(0);

        // Scatter workspace split in two halves: pivoted rows live at
        // their pivot position `k < n`, unpivoted rows at `n + original`,
        // so a pivot position can never collide with an original index.
        let mut x = vec![0.0; 2 * n];
        // DFS state: flag[row] == j+1 marks visitation in column j.
        let mut flag = vec![0_usize; n];
        let mut topo: Vec<usize> = Vec::with_capacity(n); // pivot positions, topo order
        let mut dfs_stack: Vec<(usize, usize)> = Vec::new();
        let mut nonpivot_rows: Vec<usize> = Vec::with_capacity(16);

        for j in 0..n {
            let a_col = self.col_perm[j];
            let stamp = j + 1;
            topo.clear();
            nonpivot_rows.clear();

            // Symbolic + numeric scatter: reach of the column's pattern
            // through already-built L columns (depth-first, postorder
            // gives the topological elimination order reversed).
            for idx in col_ptr[a_col]..col_ptr[a_col + 1] {
                let row = row_idx[idx] as usize;
                if !values[idx].is_finite() {
                    return Err(SolverError::Overflow);
                }
                if flag[row] == stamp {
                    x[Self::x_slot(n, &pinv, row)] += values[idx];
                    continue;
                }
                Self::dfs_reach(
                    n,
                    row,
                    stamp,
                    &mut flag,
                    &pinv,
                    &l_ptr,
                    &l_rows,
                    &mut dfs_stack,
                    &mut topo,
                    &mut nonpivot_rows,
                    &mut x,
                );
                x[Self::x_slot(n, &pinv, row)] += values[idx];
            }

            // Numeric left-looking elimination in topological order.
            // `topo` holds pivot positions discovered in reverse
            // topological order (postorder), so walk it backwards.
            for &k in topo.iter().rev() {
                let alpha = x[k];
                u_pos.push(k);
                u_vals.push(alpha);
                if alpha != 0.0 {
                    for li in l_ptr[k]..l_ptr[k + 1] {
                        let row = l_rows[li];
                        x[Self::x_slot(n, &pinv, row)] -= alpha * l_vals[li];
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
            for &row in &nonpivot_rows {
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
            let pivot_row = if diag_abs >= PIVOT_TOL * max_abs {
                preferred_row
            } else {
                max_row
            };
            let pivot_val = x[n + pivot_row];
            let pivot_recip = 1.0 / pivot_val;
            use_diag_recip &= pivot_recip.is_finite();
            u_diag_recip.push(pivot_recip);

            // Emit U's diagonal last so solves can read it directly.
            u_pos.push(j);
            u_vals.push(pivot_val);

            // Emit L column (unpivoted rows except the pivot), scaled by
            // the pivot reciprocal (one divide per column; the multiplies
            // autovectorize). Numeric zeros are kept: the pattern is
            // *symbolic* — a value that cancels at this factorization can
            // be nonzero at the next refactor, which replays these slots.
            for &row in &nonpivot_rows {
                let slot = n + row;
                let v = x[slot];
                x[slot] = 0.0;
                if row == pivot_row {
                    continue;
                }
                l_rows.push(row);
                l_vals.push(if pivot_recip.is_finite() {
                    v * pivot_recip
                } else {
                    v / pivot_val
                });
            }
            l_ptr.push(l_rows.len());
            u_ptr.push(u_pos.len());

            pinv[pivot_row] = j;
            p_row[j] = pivot_row;
        }

        // Remap L's original row indices into pivot space (after a full
        // factorization every row holds a pivot position) and narrow the
        // index arrays to u32 for the hot loops.
        self.l_rows = l_rows.iter().map(|&row| pinv[row] as u32).collect();
        self.u_rows = u_pos.iter().map(|&k| k as u32).collect();

        // Precompute the refactor scatter: pivot-space target of every
        // entry of A's value array, aligned to the original value index.
        self.a_scatter = row_idx
            .iter()
            .map(|&row| pinv[row as usize] as u32)
            .collect();

        self.row_perm = p_row;
        self.row_perm_inv = pinv;
        self.l_col_ptr = l_ptr;
        self.l_vals = l_vals;
        self.u_col_ptr = u_ptr;
        self.u_vals = u_vals;
        self.u_diag_recip = u_diag_recip;
        self.use_diag_recip = use_diag_recip;
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
        l_rows: &[usize],
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
                let next = l_rows[begin + *child];
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
        // A non-short-circuit fold lets LLVM vectorize this contiguous scan;
        // keeping validation out of the pivot-space scatter avoids a branch
        // between each random index load and store.
        if !values
            .iter()
            .fold(true, |all_finite, value| all_finite & value.is_finite())
        {
            self.factored = false;
            return Err(SolverError::Overflow);
        }
        let n = self.n;
        let x = &mut self.work;
        let col_ptr = &self.a_col_ptr;
        let mut use_diag_recip = true;

        for j in 0..n {
            let a_col = self.col_perm[j];
            // Scatter A's column into pivot space through the precomputed
            // targets — one u32 load + one store per nonzero, no
            // row-index load, no permutation lookup.
            let (a_begin, a_end) = (col_ptr[a_col], col_ptr[a_col + 1]);
            for (&slot, &v) in self.a_scatter[a_begin..a_end]
                .iter()
                .zip(&values[a_begin..a_end])
            {
                x[slot as usize] = v;
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
                        x[row as usize] -= alpha * lv;
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
                    col_max = col_max.max(v.abs());
                    *slot = v; // scaled below once the pivot is validated
                }
            }
            col_max = col_max.max(pivot.abs());

            if !pivot.is_finite() {
                self.factored = false;
                return Err(SolverError::Overflow);
            }
            if pivot == 0.0 || pivot.abs() < REFACTOR_GROWTH_TOL * col_max {
                self.factored = false;
                return Err(SolverError::PivotGrowth);
            }
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
            *slot = b[row];
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
        out.resize(n, 0.0);
        for (&col, &zj) in self.col_perm.iter().zip(&x[..n]) {
            out[col] = zj;
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

/// Iterative Kosaraju SCC decomposition. Edges are columns to the columns
/// whose matched rows they touch. Components are emitted in topological order,
/// making the structurally permuted matrix block lower triangular.
fn strongly_connected_components(adjacency: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<usize>) {
    let n = adjacency.len();
    let mut reverse = vec![Vec::new(); n];
    for (source, targets) in adjacency.iter().enumerate() {
        for &target in targets {
            reverse[target].push(source);
        }
    }

    let mut seen = vec![false; n];
    let mut finish = Vec::with_capacity(n);
    let mut dfs = Vec::<(usize, usize)>::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        dfs.push((start, 0));
        while let Some((node, next)) = dfs.last_mut() {
            if *next < adjacency[*node].len() {
                let target = adjacency[*node][*next];
                *next += 1;
                if !seen[target] {
                    seen[target] = true;
                    dfs.push((target, 0));
                }
            } else {
                if let Some((finished, _)) = dfs.pop() {
                    finish.push(finished);
                }
            }
        }
    }

    let mut component_of = vec![usize::MAX; n];
    let mut components = Vec::new();
    let mut stack = Vec::new();
    for &start in finish.iter().rev() {
        if component_of[start] != usize::MAX {
            continue;
        }
        let component = components.len();
        component_of[start] = component;
        stack.push(start);
        let mut nodes = Vec::new();
        while let Some(node) = stack.pop() {
            nodes.push(node);
            for &target in &reverse[node] {
                if component_of[target] == usize::MAX {
                    component_of[target] = component;
                    stack.push(target);
                }
            }
        }
        components.push(nodes);
    }
    (components, component_of)
}

/// KLU symbolic ordering: maximum transversal, BTF, then AMD within blocks.
fn btf_amd_order(n: usize, col_ptr: &[usize], row_idx: &[usize]) -> Option<BtfOrder> {
    if n == 0 {
        return Some(BtfOrder {
            col_perm: Vec::new(),
            matched_row_for_col: Vec::new(),
            row_block: Vec::new(),
            col_block: Vec::new(),
            block_count: 0,
        });
    }
    let (matched_row_for_col, matched_col_for_row) = structural_matching(n, col_ptr, row_idx)?;

    let mut adjacency = vec![Vec::new(); n];
    for col in 0..n {
        for &row in &row_idx[col_ptr[col]..col_ptr[col + 1]] {
            let target = matched_col_for_row[row];
            if target != col {
                adjacency[col].push(target);
            }
        }
    }
    let (mut components, component_of) = strongly_connected_components(&adjacency);
    let block_count = components.len();
    let mut col_block = vec![0u32; n];
    let mut row_block = vec![0u32; n];
    let mut col_perm = Vec::with_capacity(n);
    let mut local_position = vec![usize::MAX; n];

    for (block, nodes) in components.iter_mut().enumerate() {
        nodes.sort_unstable();
        for (local, &col) in nodes.iter().enumerate() {
            local_position[col] = local;
            col_block[col] = block as u32;
            row_block[matched_row_for_col[col]] = block as u32;
        }

        if nodes.len() == 1 {
            col_perm.push(nodes[0]);
        } else {
            let mut local_col_ptr = Vec::with_capacity(nodes.len() + 1);
            let mut local_rows = Vec::new();
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
            let local_order = amd_order(nodes.len(), &local_col_ptr, &local_rows)
                .unwrap_or_else(|| (0..nodes.len()).collect());
            col_perm.extend(local_order.into_iter().map(|local| nodes[local]));
        }

        for &col in nodes.iter() {
            local_position[col] = usize::MAX;
        }
    }

    Some(BtfOrder {
        col_perm,
        matched_row_for_col: matched_row_for_col
            .into_iter()
            .map(|row| row as u32)
            .collect(),
        row_block,
        col_block,
        block_count,
    })
}

/// AMD ordering on the symmetrized pattern via faer (the fill-reducing
/// ordering KLU applies inside its blocks). Returns the permutation as
/// "step k eliminates original column `perm[k]`", or `None` when the
/// pattern is rejected (caller falls back to natural order).
fn amd_order(n: usize, col_ptr: &[usize], row_idx: &[usize]) -> Option<Vec<usize>> {
    use faer::dyn_stack::{MemBuffer, MemStack};
    use faer::sparse::SymbolicSparseColMatRef;
    use faer::sparse::linalg::amd;

    if n == 0 {
        return Some(Vec::new());
    }
    let csc = SymbolicSparseColMatRef::new_checked(n, n, col_ptr, None, row_idx);

    let mut perm = vec![0usize; n];
    let mut perm_inv = vec![0usize; n];
    let mut mem = MemBuffer::try_new(amd::order_scratch::<usize>(n, row_idx.len())).ok()?;
    amd::order(
        &mut perm,
        &mut perm_inv,
        csc,
        amd::Control::default(),
        MemStack::new(&mut mem),
    )
    .ok()?;
    Some(perm)
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
    }

    #[test]
    fn subnormal_pivot_uses_the_exact_division_solve_path() {
        let tiny = Value::from_bits(1);
        let mut klu = KluSolver::new();
        klu.analyze(1, &[0, 1], &[0]).expect("analyze");
        klu.factor(&[tiny]).expect("factor");
        assert!(!klu.use_diag_recip);

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
