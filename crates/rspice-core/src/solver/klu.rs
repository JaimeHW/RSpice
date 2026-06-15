//! KLU-class sparse LU for circuit matrices (roadmap M3.1, phase 1).
//!
//! Circuit Newton loops factor the *same sparsity pattern* hundreds of
//! thousands of times with changing values. This solver exploits that:
//!
//! * **analyze** — fill-reducing column ordering via AMD on the
//!   symmetrized pattern (faer's Amestoy–Davis–Duff implementation; the
//!   ordering KLU applies inside BTF blocks), computed once per pattern;
//! * **factor** — left-looking Gilbert–Peierls LU with
//!   diagonal-preference threshold pivoting (KLU's default bias keeps
//!   circuit diagonals as pivots whenever they are within `PIVOT_TOL`
//!   of the column maximum, which keeps the pivot sequence reusable);
//! * **refactor** — the hot path: values-only replay over the frozen
//!   L/U pattern with the stored pivots — no symbolic work, no pivot
//!   search, no allocation. A pivot-growth alarm falls back to a fresh
//!   full factorization (and the caller may fall back further).
//!
//! Phase 2 (per the roadmap) adds BTF permutation to factor independent
//! blocks; phase 1 treats the matrix as one block.
//!
//! This is the default real-valued backend (`RSPICE_SOLVER=faer` opts
//! out). Kernel conventions, all benchmark-gated (`examples/klu_bench`):
//! u32 row indices (half the index bandwidth), a precomputed pivot-space
//! scatter map for A's values, paired-slice iteration for bounds-check
//! elision, reciprocal pivot scaling (the contiguous multiplies
//! autovectorize — the gather/scatter loops themselves cannot, which is
//! also why KLU-class solvers are famously non-BLAS), and an
//! allocation-free solve path.

use super::SolverError;
use crate::Value;

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
    /// Fill-reducing column order: pivot column k eliminates original
    /// column `col_perm[k]`.
    col_perm: Vec<usize>,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the symbolic analysis matches this pattern instance.
    pub fn is_analyzed_for(&self, n: usize) -> bool {
        self.n == n && !self.col_perm.is_empty()
    }

    /// `(L, U)` stored nonzero counts of the current factorization —
    /// fill diagnostics for ordering quality.
    pub fn factor_nnz(&self) -> (usize, usize) {
        (self.l_vals.len(), self.u_vals.len())
    }

    /// One-time symbolic phase for a pattern: fill-reducing ordering via
    /// AMD on `A + Aᵀ` (faer's Amestoy–Davis–Duff implementation — the
    /// same ordering KLU applies inside its blocks). Falls back to the
    /// natural order if AMD declines the pattern. The L/U pattern itself
    /// is discovered during the first `factor`.
    pub fn analyze(&mut self, n: usize, col_ptr: &[usize], row_idx: &[usize]) {
        self.n = n;
        self.col_perm = amd_order(n, col_ptr, row_idx).unwrap_or_else(|| (0..n).collect());
        self.row_perm.clear();
        self.row_perm_inv.clear();
        self.factored = false;
        self.work = vec![0.0; n];
    }

    /// Full Gilbert–Peierls factorization with fresh pivot selection.
    pub fn factor(
        &mut self,
        col_ptr: &[usize],
        row_idx: &[usize],
        values: &[Value],
    ) -> Result<(), SolverError> {
        let n = self.n;
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
                let row = row_idx[idx];
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
            // with diagonal preference within PIVOT_TOL.
            let mut max_abs = 0.0_f64;
            let mut max_row = usize::MAX;
            let mut diag_abs = -1.0_f64;
            for &row in &nonpivot_rows {
                let v = x[n + row].abs();
                if v > max_abs {
                    max_abs = v;
                    max_row = row;
                }
                if row == a_col {
                    diag_abs = v;
                }
            }
            if max_row == usize::MAX || max_abs == 0.0 || !max_abs.is_finite() {
                return Err(SolverError::SingularMatrix);
            }
            let pivot_row = if diag_abs >= PIVOT_TOL * max_abs {
                a_col
            } else {
                max_row
            };
            let pivot_val = x[n + pivot_row];

            // Emit U's diagonal last so solves can read it directly.
            u_pos.push(j);
            u_vals.push(pivot_val);

            // Emit L column (unpivoted rows except the pivot), scaled by
            // the pivot reciprocal (one divide per column; the multiplies
            // autovectorize). Numeric zeros are kept: the pattern is
            // *symbolic* — a value that cancels at this factorization can
            // be nonzero at the next refactor, which replays these slots.
            let pivot_recip = 1.0 / pivot_val;
            for &row in &nonpivot_rows {
                let slot = n + row;
                let v = x[slot];
                x[slot] = 0.0;
                if row == pivot_row {
                    continue;
                }
                l_rows.push(row);
                l_vals.push(v * pivot_recip);
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
        self.a_scatter = row_idx.iter().map(|&row| pinv[row] as u32).collect();

        self.row_perm = p_row;
        self.row_perm_inv = pinv;
        self.l_col_ptr = l_ptr;
        self.l_vals = l_vals;
        self.u_col_ptr = u_ptr;
        self.u_vals = u_vals;
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
    pub fn refactor(
        &mut self,
        col_ptr: &[usize],
        row_idx: &[usize],
        values: &[Value],
    ) -> Result<(), SolverError> {
        if !self.factored {
            return self.factor(col_ptr, row_idx, values);
        }
        let n = self.n;
        let x = &mut self.work;
        let _ = row_idx; // pattern is frozen; the precomputed scatter stands in

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

            if pivot == 0.0 || !pivot.is_finite() || pivot.abs() < REFACTOR_GROWTH_TOL * col_max {
                self.factored = false;
                return Err(SolverError::PivotGrowth);
            }
            self.u_vals[u_end - 1] = pivot;
            // One divide per column; the contiguous multiplies vectorize.
            let pivot_recip = 1.0 / pivot;
            for value in &mut self.l_vals[ls..le] {
                *value *= pivot_recip;
            }
        }
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
        // Back solve U z = y; columns hold the diagonal last.
        for j in (0..n).rev() {
            let u_begin = self.u_col_ptr[j];
            let u_end = self.u_col_ptr[j + 1];
            let diag = self.u_vals[u_end - 1];
            let zj = x[j] / diag;
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
        // Un-permute columns into the output: the solution component for
        // original column `col_perm[j]` is z[j].
        out.resize(n, 0.0);
        for (&col, &zj) in self.col_perm.iter().zip(&x[..n]) {
            out[col] = zj;
        }
        Ok(())
    }
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
            klu.analyze(n, &col_ptr, &rows);
            klu.factor(&col_ptr, &rows, &vals)
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
            klu.analyze(n, &col_ptr, &rows);
            klu.factor(&col_ptr, &rows, &vals).expect("factor");

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

                match klu.refactor(&col_ptr, &rows, &vals) {
                    Ok(()) => {}
                    Err(SolverError::PivotGrowth) => {
                        klu.factor(&col_ptr, &rows, &vals).expect("re-factor");
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
        klu.analyze(n, &col_ptr, &rows);
        assert!(matches!(
            klu.factor(&col_ptr, &rows, &vals),
            Err(SolverError::SingularMatrix)
        ));
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
        klu.analyze(n, &col_ptr, &rows);
        klu.factor(&col_ptr, &rows, &vals).expect("factor");
        let (l_nnz, u_nnz) = klu.factor_nnz();
        assert!(
            l_nnz + u_nnz <= a_nnz + n / 4,
            "banded fill blew up: {} from {}",
            l_nnz + u_nnz,
            a_nnz
        );
    }
}
