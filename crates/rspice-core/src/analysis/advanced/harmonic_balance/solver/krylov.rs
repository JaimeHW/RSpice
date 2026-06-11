//! Krylov linear solver for the HB Newton step.
//!
//! The HB Jacobian is an (n·h) × (n·h) complex matrix whose dominant
//! structure is block-diagonal per harmonic (the linearized admittance
//! Y(ωₖ) couples nodes within one harmonic; nonlinear devices add Toeplitz
//! coupling between harmonics). Dense Gaussian elimination costs
//! O((n·h)³) per Newton iteration, which caps HB at toy circuits.
//!
//! This module replaces that solve with restarted GMRES, right-
//! preconditioned by the per-harmonic diagonal blocks (block-Jacobi):
//! each n×n block J[·,k][·,k] is LU-factorized once per Newton iteration,
//! and GMRES iterates with O((n·h)²) matrix-vector products. With moderate
//! harmonic coupling the preconditioned iteration converges in a few tens
//! of steps, turning the cubic bottleneck into an effectively quadratic
//! one. When the matrix IS block-diagonal (linear circuit), GMRES
//! converges in a single iteration.
//!
//! Correctness policy: the caller falls back to the exact dense solve
//! whenever GMRES reports stagnation or non-convergence, so enabling the
//! Krylov path can never change whether Newton converges — only how fast
//! the step is computed.

use num_complex::Complex64;

const ZERO: Complex64 = Complex64::new(0.0, 0.0);

/// Relative tolerance for the inner linear solve. Newton needs the step
/// direction accurate well below its own tolerance; 1e-10 keeps Krylov
/// steps numerically indistinguishable from the direct solve.
pub(super) const GMRES_REL_TOL: f64 = 1e-10;

/// Problem size at and above which the Krylov path is tried automatically
/// (the direct solve remains the small-problem default — exact and faster
/// below this size).
pub(super) const KRYLOV_AUTO_THRESHOLD: usize = 256;

//=============================================================================
// Dense complex LU (preconditioner blocks)
//=============================================================================

/// LU factors of one dense complex block with partial pivoting.
///
/// Near-singular pivots are regularized rather than failed: the
/// preconditioner only steers GMRES, so an approximate inverse of a
/// degenerate block is still useful, and the exact-solve fallback path
/// protects correctness.
pub(super) struct LuFactors {
    n: usize,
    /// Combined L (unit lower, below diagonal) and U (upper), row-major.
    lu: Vec<Complex64>,
    /// Row swap applied at elimination step k: rows k <-> pivot[k].
    pivots: Vec<usize>,
    /// True when any pivot needed regularization.
    pub regularized: bool,
}

impl LuFactors {
    pub(super) fn factor(mut a: Vec<Complex64>, n: usize) -> Self {
        debug_assert_eq!(a.len(), n * n);
        let mut pivots = vec![0usize; n];
        let mut regularized = false;

        for k in 0..n {
            // Partial pivoting: largest magnitude in column k at/below row k.
            let mut p = k;
            let mut max_norm = a[k * n + k].norm_sqr();
            for i in (k + 1)..n {
                let cand = a[i * n + k].norm_sqr();
                if cand > max_norm {
                    max_norm = cand;
                    p = i;
                }
            }
            pivots[k] = p;
            if p != k {
                for j in 0..n {
                    a.swap(k * n + j, p * n + j);
                }
            }

            let mut pivot = a[k * n + k];
            if pivot.norm_sqr() < 1e-300 {
                // Regularize: identity-scale placeholder keeps the block
                // invertible as a preconditioner.
                pivot = Complex64::new(1e-12, 0.0);
                a[k * n + k] = pivot;
                regularized = true;
            }

            for i in (k + 1)..n {
                let factor = a[i * n + k] / pivot;
                a[i * n + k] = factor;
                if factor != ZERO {
                    for j in (k + 1)..n {
                        let u_kj = a[k * n + j];
                        a[i * n + j] -= factor * u_kj;
                    }
                }
            }
        }

        Self {
            n,
            lu: a,
            pivots,
            regularized,
        }
    }

    /// Solve `A x = b` in place using the stored factors.
    pub(super) fn solve_in_place(&self, x: &mut [Complex64]) {
        let n = self.n;
        debug_assert_eq!(x.len(), n);

        // Apply row permutation in elimination order.
        for k in 0..n {
            let p = self.pivots[k];
            if p != k {
                x.swap(k, p);
            }
        }

        // Forward substitution: L y = P b (unit diagonal).
        for i in 1..n {
            let mut sum = x[i];
            for j in 0..i {
                sum -= self.lu[i * n + j] * x[j];
            }
            x[i] = sum;
        }

        // Back substitution: U x = y.
        for i in (0..n).rev() {
            let mut sum = x[i];
            for j in (i + 1)..n {
                sum -= self.lu[i * n + j] * x[j];
            }
            x[i] = sum / self.lu[i * n + i];
        }
    }
}

//=============================================================================
// Block-Jacobi preconditioner
//=============================================================================

/// Per-harmonic block-Jacobi preconditioner for the HB Jacobian.
///
/// Unknown layout is node-major (`index = node * h + harmonic`), so the
/// harmonic-k block gathers the strided entries `J[i*h + k][j*h + k]`.
pub(super) struct BlockJacobiPreconditioner {
    num_nodes: usize,
    num_components: usize, // h = harmonics + 1
    factors: Vec<LuFactors>,
}

impl BlockJacobiPreconditioner {
    /// Extract and factorize the per-harmonic diagonal blocks of `jac`.
    pub(super) fn build(jac: &[Vec<Complex64>], num_nodes: usize, num_components: usize) -> Self {
        let factors: Vec<LuFactors> = (0..num_components)
            .map(|k| {
                let mut block = vec![ZERO; num_nodes * num_nodes];
                for i in 0..num_nodes {
                    let row = &jac[i * num_components + k];
                    for j in 0..num_nodes {
                        block[i * num_nodes + j] = row[j * num_components + k];
                    }
                }
                LuFactors::factor(block, num_nodes)
            })
            .collect();

        // Singular harmonic blocks are convergence forensics: the
        // preconditioner stays usable (regularized pivots), but the
        // condition is worth a trace when debugging stagnation.
        let regularized_blocks = factors.iter().filter(|f| f.regularized).count();
        if regularized_blocks > 0 {
            log::debug!(
                "HB block-Jacobi preconditioner: {} of {} harmonic blocks were \
                 near-singular and regularized",
                regularized_blocks,
                num_components
            );
        }

        Self {
            num_nodes,
            num_components,
            factors,
        }
    }

    /// Apply M⁻¹ to a flattened residual vector.
    pub(super) fn apply(&self, r: &[Complex64]) -> Vec<Complex64> {
        let n = self.num_nodes;
        let h = self.num_components;
        let mut out = r.to_vec();
        let mut scratch = vec![ZERO; n];

        for k in 0..h {
            for i in 0..n {
                scratch[i] = r[i * h + k];
            }
            self.factors[k].solve_in_place(&mut scratch);
            for i in 0..n {
                out[i * h + k] = scratch[i];
            }
        }
        out
    }
}

//=============================================================================
// Right-preconditioned restarted GMRES
//=============================================================================

/// Outcome of a GMRES solve.
pub(super) struct GmresOutcome {
    pub solution: Vec<Complex64>,
    pub iterations: usize,
    pub relative_residual: f64,
    pub converged: bool,
}

#[inline]
fn dot(u: &[Complex64], v: &[Complex64]) -> Complex64 {
    u.iter().zip(v).map(|(a, b)| a.conj() * b).sum()
}

#[inline]
fn norm(v: &[Complex64]) -> f64 {
    v.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt()
}

/// Complex Givens rotation zeroing `b` against `a`:
/// `[c, s; -conj(s), c] · [a; b] = [r; 0]` with real `c >= 0`.
#[inline]
fn givens(a: Complex64, b: Complex64) -> (f64, Complex64) {
    if b == ZERO {
        return (1.0, ZERO);
    }
    if a == ZERO {
        return (0.0, b.conj() / b.norm());
    }
    let t = (a.norm_sqr() + b.norm_sqr()).sqrt();
    let c = a.norm() / t;
    let s = (a / a.norm()) * b.conj() / t;
    (c, s)
}

/// Solve `A x = b` with restarted GMRES(m), right-preconditioned by `precond`.
///
/// `matvec` applies A to a flattened vector. `x` starts at zero (the HB
/// Newton step has no better initial guess). Stagnation across a restart
/// cycle terminates early with `converged = false`; the caller is expected
/// to fall back to the exact dense solve.
pub(super) fn gmres(
    matvec: &dyn Fn(&[Complex64]) -> Vec<Complex64>,
    precond: &BlockJacobiPreconditioner,
    b: &[Complex64],
    restart: usize,
    max_outer: usize,
) -> GmresOutcome {
    let size = b.len();
    let b_norm = norm(b);
    if b_norm == 0.0 {
        return GmresOutcome {
            solution: vec![ZERO; size],
            iterations: 0,
            relative_residual: 0.0,
            converged: true,
        };
    }

    let m = restart.clamp(1, size);
    let mut x = vec![ZERO; size];
    let mut r = b.to_vec();
    let mut beta = b_norm;
    let mut total_iterations = 0usize;

    for _outer in 0..max_outer {
        // Arnoldi basis (m+1 vectors) and Hessenberg columns.
        let mut basis: Vec<Vec<Complex64>> = Vec::with_capacity(m + 1);
        basis.push(r.iter().map(|c| c / beta).collect());

        let mut hessenberg: Vec<Vec<Complex64>> = Vec::with_capacity(m);
        let mut cs: Vec<f64> = Vec::with_capacity(m);
        let mut sn: Vec<Complex64> = Vec::with_capacity(m);
        let mut g = vec![ZERO; m + 1];
        g[0] = Complex64::new(beta, 0.0);

        let mut inner_used = 0usize;
        let mut happy_breakdown = false;

        for j in 0..m {
            total_iterations += 1;
            inner_used = j + 1;

            // w = A · M⁻¹ vⱼ
            let z = precond.apply(&basis[j]);
            let mut w = matvec(&z);

            // Modified Gram-Schmidt.
            let mut h_col = vec![ZERO; j + 2];
            for (i, v_i) in basis.iter().enumerate().take(j + 1) {
                let h_ij = dot(v_i, &w);
                h_col[i] = h_ij;
                for (w_k, v_k) in w.iter_mut().zip(v_i) {
                    *w_k -= h_ij * v_k;
                }
            }
            let w_norm = norm(&w);
            h_col[j + 1] = Complex64::new(w_norm, 0.0);

            // Apply accumulated rotations to the new column.
            for i in 0..j {
                let temp = cs[i] * h_col[i] + sn[i] * h_col[i + 1];
                h_col[i + 1] = -sn[i].conj() * h_col[i] + cs[i] * h_col[i + 1];
                h_col[i] = temp;
            }

            // New rotation zeroing the subdiagonal.
            let (c, s) = givens(h_col[j], h_col[j + 1]);
            h_col[j] = c * h_col[j] + s * h_col[j + 1];
            h_col[j + 1] = ZERO;
            let g_j = g[j];
            g[j] = c * g_j + s * g[j + 1];
            g[j + 1] = -s.conj() * g_j + c * g[j + 1];
            cs.push(c);
            sn.push(s);
            hessenberg.push(h_col);

            let residual_estimate = g[j + 1].norm();
            if residual_estimate / b_norm < GMRES_REL_TOL {
                happy_breakdown = true;
                break;
            }
            if w_norm < 1e-300 {
                happy_breakdown = true;
                break;
            }
            basis.push(w.into_iter().map(|c| c / w_norm).collect());
        }

        // Solve the (triangular) least-squares system for y.
        let k = inner_used;
        let mut y = vec![ZERO; k];
        for i in (0..k).rev() {
            let mut sum = g[i];
            for (j, y_j) in y.iter().enumerate().take(k).skip(i + 1) {
                sum -= hessenberg[j][i] * y_j;
            }
            y[i] = sum / hessenberg[i][i];
        }

        // x += M⁻¹ (V y)
        let mut update = vec![ZERO; size];
        for (j, y_j) in y.iter().enumerate() {
            for (u, v) in update.iter_mut().zip(&basis[j]) {
                *u += y_j * v;
            }
        }
        let preconditioned = precond.apply(&update);
        for (x_i, p_i) in x.iter_mut().zip(&preconditioned) {
            *x_i += p_i;
        }

        // True residual for the restart decision.
        let ax = matvec(&x);
        for ((r_i, b_i), ax_i) in r.iter_mut().zip(b).zip(&ax) {
            *r_i = b_i - ax_i;
        }
        let new_beta = norm(&r);
        let relative = new_beta / b_norm;

        if relative < GMRES_REL_TOL {
            return GmresOutcome {
                solution: x,
                iterations: total_iterations,
                relative_residual: relative,
                converged: true,
            };
        }
        if happy_breakdown || new_beta > 0.99 * beta {
            // Lucky breakdown without convergence, or stagnation across a
            // full cycle: stop and let the caller take the exact path.
            return GmresOutcome {
                solution: x,
                iterations: total_iterations,
                relative_residual: relative,
                converged: false,
            };
        }
        beta = new_beta;
    }

    let relative = norm(&r) / b_norm;
    GmresOutcome {
        solution: x,
        iterations: total_iterations,
        relative_residual: relative,
        converged: false,
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pseudo-random stream for reproducible test matrices.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) - 0.5
        }
        fn next_c(&mut self) -> Complex64 {
            Complex64::new(self.next(), self.next())
        }
    }

    /// Reference dense solve (Gaussian elimination with partial pivoting).
    fn dense_solve(a: &[Vec<Complex64>], b: &[Complex64]) -> Vec<Complex64> {
        let n = b.len();
        let mut aug: Vec<Vec<Complex64>> = a
            .iter()
            .zip(b)
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(bi);
                r
            })
            .collect();
        for col in 0..n {
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].norm() > aug[max_row][col].norm() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);
            let pivot = aug[col][col];
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    let v = aug[col][k];
                    aug[row][k] -= factor * v;
                }
            }
        }
        let mut x = vec![ZERO; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }
        x
    }

    /// Build an HB-shaped test matrix: strong per-harmonic diagonal blocks
    /// plus weak Toeplitz-style inter-harmonic coupling.
    fn hb_like_matrix(
        num_nodes: usize,
        num_components: usize,
        coupling: f64,
        seed: u64,
    ) -> Vec<Vec<Complex64>> {
        let size = num_nodes * num_components;
        let mut rng = Lcg(seed);
        let mut a = vec![vec![ZERO; size]; size];

        for k in 0..num_components {
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    let mut v = rng.next_c();
                    if i == j {
                        v += Complex64::new(4.0 + num_nodes as f64, 1.0 + k as f64);
                    }
                    a[i * num_components + k][j * num_components + k] = v;
                }
            }
        }
        if coupling != 0.0 {
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    for k in 0..num_components {
                        for l in 0..num_components {
                            if k != l {
                                a[i * num_components + k][j * num_components + l] +=
                                    rng.next_c() * coupling;
                            }
                        }
                    }
                }
            }
        }
        a
    }

    fn matvec_of(a: &[Vec<Complex64>]) -> impl Fn(&[Complex64]) -> Vec<Complex64> + '_ {
        move |x: &[Complex64]| {
            a.iter()
                .map(|row| row.iter().zip(x).map(|(m, v)| m * v).sum())
                .collect()
        }
    }

    #[test]
    fn lu_solve_roundtrip() {
        let n = 8;
        let mut rng = Lcg(7);
        let mut a = vec![ZERO; n * n];
        for i in 0..n {
            for j in 0..n {
                a[i * n + j] = rng.next_c() + if i == j { Complex64::new(5.0, 0.0) } else { ZERO };
            }
        }
        let x_true: Vec<Complex64> = (0..n).map(|i| Complex64::new(i as f64 + 1.0, -(i as f64))).collect();
        let mut b = vec![ZERO; n];
        for i in 0..n {
            for j in 0..n {
                b[i] += a[i * n + j] * x_true[j];
            }
        }

        let lu = LuFactors::factor(a, n);
        assert!(!lu.regularized);
        let mut x = b;
        lu.solve_in_place(&mut x);
        for (got, want) in x.iter().zip(&x_true) {
            assert!((got - want).norm() < 1e-10, "LU roundtrip: {got} vs {want}");
        }
    }

    #[test]
    fn block_diagonal_systems_converge_in_one_cycle_step() {
        // No inter-harmonic coupling: the preconditioner IS the inverse, so
        // GMRES must hit the tolerance on its first iteration.
        let (n, h) = (6, 5);
        let a = hb_like_matrix(n, h, 0.0, 11);
        let mut rng = Lcg(23);
        let b: Vec<Complex64> = (0..n * h).map(|_| rng.next_c()).collect();

        let precond = BlockJacobiPreconditioner::build(&a, n, h);
        let outcome = gmres(&matvec_of(&a), &precond, &b, 30, 4);
        assert!(outcome.converged, "block-diagonal GMRES must converge");
        assert!(
            outcome.iterations <= 2,
            "exact preconditioner needs one iteration, took {}",
            outcome.iterations
        );
    }

    #[test]
    fn coupled_systems_match_the_direct_solve() {
        let (n, h) = (7, 6);
        let a = hb_like_matrix(n, h, 0.15, 31);
        let mut rng = Lcg(57);
        let b: Vec<Complex64> = (0..n * h).map(|_| rng.next_c()).collect();

        let reference = dense_solve(&a, &b);
        let precond = BlockJacobiPreconditioner::build(&a, n, h);
        let outcome = gmres(&matvec_of(&a), &precond, &b, 30, 4);

        assert!(outcome.converged, "coupled GMRES must converge");
        let scale = reference.iter().map(|c| c.norm()).fold(0.0f64, f64::max);
        for (got, want) in outcome.solution.iter().zip(&reference) {
            assert!(
                (got - want).norm() <= 1e-8 * scale.max(1.0),
                "GMRES vs direct: {got} vs {want}"
            );
        }
    }

    #[test]
    fn stagnation_is_reported_not_silent() {
        // A matrix whose off-block coupling overwhelms the blocks defeats
        // the block-Jacobi preconditioner quickly; the outcome must say so.
        let (n, h) = (4, 4);
        let mut a = hb_like_matrix(n, h, 0.0, 91);
        // Make the true operator wildly different from its diagonal blocks.
        let size = n * h;
        let mut rng = Lcg(101);
        for i in 0..size {
            for j in 0..size {
                if i != j {
                    a[i][j] += rng.next_c() * 25.0;
                }
            }
        }
        let b: Vec<Complex64> = (0..size).map(|_| rng.next_c()).collect();
        let precond = BlockJacobiPreconditioner::build(&a, n, h);
        let outcome = gmres(&matvec_of(&a), &precond, &b, 4, 1);
        // Either it converged anyway (fine) or it must report failure
        // honestly so the caller can take the exact path.
        if !outcome.converged {
            assert!(outcome.relative_residual > GMRES_REL_TOL);
        }
    }
}
