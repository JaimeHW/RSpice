//! Dense linear-system helpers shared by HB linear and Newton solves.

use super::*;
impl HbSolver {
    /// Solve complex linear system using Gaussian elimination
    pub(super) fn solve_complex_linear_system(
        &self,
        a: &[Vec<Complex64>],
        b: &[Complex64],
    ) -> Result<Vec<Complex64>, HbError> {
        let n = b.len();
        if n == 0 {
            return Ok(vec![]);
        }

        // Augmented matrix
        let mut aug: Vec<Vec<Complex64>> = a
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(bi);
                r
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].norm() > aug[max_row][col].norm() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.norm() < 1e-15 {
                continue; // Near-singular, skip
            }

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    let col_val = aug[col][k];
                    aug[row][k] -= factor * col_val;
                }
            }
        }

        // Back substitution
        let mut x = vec![Complex64::new(0.0, 0.0); n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].norm() > 1e-15 {
                x[i] = sum / aug[i][i];
            }
        }

        Ok(x)
    }
}
