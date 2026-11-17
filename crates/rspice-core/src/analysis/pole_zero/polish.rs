//! Newton refinement of a pencil's computed roots against the pencil itself.
//!
//! The generalized eigensolve is backward stable in norm: it returns the
//! exact roots of a pencil within a rounding of `‖G‖` and `‖C‖`. An MNA
//! pencil spans many decades (a 1e-15 F capacitance beside a 1 S
//! conductance), so a perturbation that small relative to the norm is large
//! relative to the entries a root depends on, and a root can come back
//! wrong in its fifth digit. On C5's LDO the right-half-plane zero pair moved
//! 1.8e-5 of its magnitude when the operating point moved 1e-13 V, while
//! the AC response it shapes moved 3e-9.
//!
//! Newton's method on `det(G + sC)` reads the pencil through an LU
//! factorization of `G + sC` itself, whose rounding is relative to the
//! entries it combines, so a refined root is as accurate as the pencil's
//! own entries allow. `d/ds ln det(G + sC) = tr((G + sC)⁻¹ C)`, so each step
//! is one factorization and one solve per row of `C` that holds a value.
//!
//! A refinement is kept only when its steps contracted and it stayed nearer
//! its starting root than any other root; otherwise the computed root
//! stands. Comparisons use squared magnitudes, which are exact arithmetic,
//! so every target computes the same roots from the same pencil.

use super::{Matrix, PoleZeroAnalyzer};
use crate::Complex64;

/// Newton steps a refinement may take. From a root the eigensolve placed to
/// within its backward error, Newton converges quadratically and reaches
/// the rounding floor in two or three.
const MAX_NEWTON_STEPS: usize = 8;

impl PoleZeroAnalyzer {
    /// Refine each root of `det(G + sC)` in `roots` in place. A conjugate
    /// pair is refined once and stays an exact conjugate pair; a real root
    /// stays real.
    pub(in crate::analysis::pole_zero) fn polish_pencil_roots(
        g: &Matrix,
        c: &Matrix,
        roots: &mut [Complex64],
    ) {
        let n = g.rows;
        if n == 0 || roots.is_empty() || g.dims() != c.dims() || c.rows != n {
            return;
        }
        // The rows of C that hold a value, with their entries: the trace of
        // M⁻¹C needs the columns of M⁻¹ at those rows only.
        let c_rows: Vec<(usize, Vec<(usize, f64)>)> = (0..n)
            .filter_map(|row| {
                let entries: Vec<(usize, f64)> = (0..n)
                    .filter_map(|col| {
                        let value = c.get(row, col);
                        (value != 0.0).then_some((col, value))
                    })
                    .collect();
                (!entries.is_empty()).then_some((row, entries))
            })
            .collect();
        if c_rows.is_empty() {
            return;
        }
        let starts = roots.to_vec();
        let mut pencil = PencilLu::new(n);
        for index in 0..starts.len() {
            let start = starts[index];
            if !start.re.is_finite() || !start.im.is_finite() {
                continue;
            }
            let partner = if start.im == 0.0 {
                None
            } else {
                starts
                    .iter()
                    .enumerate()
                    .position(|(other, root)| other != index && *root == start.conj())
            };
            // A pair is refined from its upper member.
            if start.im < 0.0 && partner.is_some() {
                continue;
            }
            let separation_sqr = starts
                .iter()
                .enumerate()
                .filter(|(other, _)| *other != index)
                .map(|(_, root)| (root - start).norm_sqr())
                .fold(f64::INFINITY, f64::min);
            let Some(refined) = pencil.newton(g, c, &c_rows, start, separation_sqr) else {
                continue;
            };
            roots[index] = refined;
            if let Some(partner) = partner {
                roots[partner] = refined.conj();
            }
        }
    }
}

/// `G + sC` and its LU factors with partial pivoting, row-major.
struct PencilLu {
    n: usize,
    lu: Vec<Complex64>,
    pivots: Vec<usize>,
    column: Vec<Complex64>,
}

impl PencilLu {
    fn new(n: usize) -> Self {
        Self {
            n,
            lu: vec![Complex64::new(0.0, 0.0); n * n],
            pivots: vec![0; n],
            column: vec![Complex64::new(0.0, 0.0); n],
        }
    }

    /// Newton's method on `det(G + sC)` from `start`. `None` keeps the
    /// computed root: the steps did not contract, or they carried the
    /// iterate a quarter of the way to the nearest other root.
    fn newton(
        &mut self,
        g: &Matrix,
        c: &Matrix,
        c_rows: &[(usize, Vec<(usize, f64)>)],
        start: Complex64,
        separation_sqr: f64,
    ) -> Option<Complex64> {
        let real = start.im == 0.0;
        let mut root = start;
        let mut previous_step_sqr = f64::INFINITY;
        let mut contracted = false;
        for _ in 0..MAX_NEWTON_STEPS {
            if !self.factor(g, c, root) {
                // An exactly singular pencil: the iterate is a root to the
                // last bit.
                return Some(root);
            }
            let trace = self.trace_inverse_times(c_rows);
            if trace.norm_sqr() == 0.0 || !trace.re.is_finite() || !trace.im.is_finite() {
                return None;
            }
            let mut step = -trace.inv();
            if real {
                step.im = 0.0;
            }
            let step_sqr = step.norm_sqr();
            if !step_sqr.is_finite() {
                return None;
            }
            if step_sqr >= 0.25 * previous_step_sqr {
                // The steps stopped contracting: the iterate sits at the
                // rounding floor of the determinant.
                return contracted.then_some(root);
            }
            contracted |= previous_step_sqr.is_finite();
            root += step;
            if 16.0 * (root - start).norm_sqr() > separation_sqr {
                return None;
            }
            if step_sqr <= (4.0 * f64::EPSILON).powi(2) * root.norm_sqr() {
                return Some(root);
            }
            previous_step_sqr = step_sqr;
        }
        None
    }

    /// Factor `G + sC`; false when a pivot column is exactly zero.
    fn factor(&mut self, g: &Matrix, c: &Matrix, s: Complex64) -> bool {
        let n = self.n;
        for row in 0..n {
            for col in 0..n {
                self.lu[row * n + col] = Complex64::new(g.get(row, col), 0.0) + s * c.get(row, col);
            }
        }
        for (index, pivot) in self.pivots.iter_mut().enumerate() {
            *pivot = index;
        }
        for k in 0..n {
            let mut best = k;
            let mut best_sqr = self.lu[k * n + k].norm_sqr();
            for row in k + 1..n {
                let candidate = self.lu[row * n + k].norm_sqr();
                if candidate > best_sqr {
                    best = row;
                    best_sqr = candidate;
                }
            }
            if best_sqr == 0.0 {
                return false;
            }
            if best != k {
                for col in 0..n {
                    self.lu.swap(k * n + col, best * n + col);
                }
                self.pivots.swap(k, best);
            }
            let inverse_pivot = self.lu[k * n + k].inv();
            for row in k + 1..n {
                let factor = self.lu[row * n + k] * inverse_pivot;
                if factor.norm_sqr() == 0.0 {
                    continue;
                }
                let (upper_rows, lower_rows) = self.lu.split_at_mut(row * n);
                lower_rows[k] = factor;
                let pivot_row = &upper_rows[k * n + k + 1..(k + 1) * n];
                for (target, upper) in lower_rows[k + 1..n].iter_mut().zip(pivot_row) {
                    if upper.norm_sqr() != 0.0 {
                        *target -= factor * upper;
                    }
                }
            }
        }
        true
    }

    /// `tr((G + sC)⁻¹ C)` from the factors: `Σ_j Σ_i (M⁻¹)_ij C_ji`, one
    /// solve for each row `j` of C that holds a value.
    fn trace_inverse_times(&mut self, c_rows: &[(usize, Vec<(usize, f64)>)]) -> Complex64 {
        let n = self.n;
        let mut trace = Complex64::new(0.0, 0.0);
        for (row, entries) in c_rows {
            // Column `row` of M⁻¹: solve M x = e_row through P, L and U.
            for (index, value) in self.column.iter_mut().enumerate() {
                *value = Complex64::new(if self.pivots[index] == *row { 1.0 } else { 0.0 }, 0.0);
            }
            for i in 0..n {
                let mut sum = self.column[i];
                for (lower, solved) in self.lu[i * n..i * n + i].iter().zip(&self.column[..i]) {
                    if lower.norm_sqr() != 0.0 {
                        sum -= lower * solved;
                    }
                }
                self.column[i] = sum;
            }
            for i in (0..n).rev() {
                let mut sum = self.column[i];
                let upper_row = &self.lu[i * n + i + 1..(i + 1) * n];
                for (upper, solved) in upper_row.iter().zip(&self.column[i + 1..]) {
                    if upper.norm_sqr() != 0.0 {
                        sum -= upper * solved;
                    }
                }
                self.column[i] = sum / self.lu[i * n + i];
            }
            for (col, value) in entries {
                trace += self.column[*col] * *value;
            }
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-section RC ladder driven from a source resistor, scaled so its
    /// node conductances span ten decades, with known poles.
    fn ladder(scale: f64) -> (Matrix, Matrix, [Complex64; 2]) {
        // Node 1: g1 to ground, g2 to node 2, c1 to ground; node 2: g2 to
        // node 1, c2 to ground. Row 2 is scaled by `scale`, as a large stamp
        // on one row of an MNA system scales it.
        let (g1, g2, c1, c2) = (1.0, 1e-3, 1e-12, 1e-9);
        let g = Matrix::from_dense(vec![vec![g1 + g2, -g2], vec![-g2 * scale, g2 * scale]]);
        let c = Matrix::from_dense(vec![vec![c1, 0.0], vec![0.0, c2 * scale]]);
        // det(G + sC) = c1 c2 s² + (c1 g2 + c2 (g1 + g2)) s + g1 g2, whose
        // roots are q/a and k/q with q = -(b + √(b² - 4ak))/2, free of the
        // cancellation the textbook form suffers on a root this small.
        let (a, b, k) = (c1 * c2, c1 * g2 + c2 * (g1 + g2), g1 * g2);
        let q = -0.5 * (b + (b * b - 4.0 * a * k).sqrt());
        let poles = [Complex64::new(k / q, 0.0), Complex64::new(q / a, 0.0)];
        (g, c, poles)
    }

    #[test]
    fn a_perturbed_root_is_refined_to_the_pencils_own() {
        let (g, c, exact) = ladder(1e8);
        let mut roots = exact.map(|root| root * (1.0 + 3e-6));
        PoleZeroAnalyzer::polish_pencil_roots(&g, &c, &mut roots);
        for (refined, exact) in roots.iter().zip(exact) {
            assert!(
                (refined - exact).norm() <= 1e-12 * exact.norm(),
                "refined {refined} against {exact}"
            );
            assert_eq!(refined.im, 0.0, "a real root stays real");
        }
    }

    #[test]
    fn a_conjugate_pair_is_refined_once_and_stays_conjugate() {
        // s² + 2ζω s + ω²: G + sC with C = I and G the companion matrix.
        let (omega, zeta) = (5.9e5_f64, 0.11);
        let g = Matrix::from_dense(vec![
            vec![0.0, -1.0],
            vec![omega * omega, 2.0 * zeta * omega],
        ]);
        let c = Matrix::from_dense(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        let exact = Complex64::new(-zeta * omega, omega * (1.0 - zeta * zeta).sqrt());
        let mut roots = [exact * (1.0 + 2e-5), (exact * (1.0 + 2e-5)).conj()];
        PoleZeroAnalyzer::polish_pencil_roots(&g, &c, &mut roots);
        assert!(
            (roots[0] - exact).norm() <= 1e-12 * exact.norm(),
            "{} against {exact}",
            roots[0]
        );
        assert_eq!(roots[1], roots[0].conj());
    }

    #[test]
    fn a_root_that_would_reach_another_is_left_as_computed() {
        let (g, c, exact) = ladder(1.0);
        // Start the slow pole most of the way to the fast one: Newton would
        // leave its basin, so the computed value stands.
        let far = exact[0] + (exact[1] - exact[0]) * 0.6;
        let mut roots = [far, exact[1]];
        PoleZeroAnalyzer::polish_pencil_roots(&g, &c, &mut roots);
        assert_eq!(roots[0], far);
    }
}
