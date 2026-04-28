use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::advanced::pole_zero) fn extract_submatrix(
        &self,
        m: &Matrix,
        rows: &[usize],
        cols: &[usize],
    ) -> Matrix {
        let mut out = Matrix::zeros(rows.len(), cols.len());
        for (ri, &src_r) in rows.iter().enumerate() {
            for (ci, &src_c) in cols.iter().enumerate() {
                out.data[ri][ci] = m.data[src_r][src_c];
            }
        }
        out
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_subtract(
        &self,
        a: &Matrix,
        b: &Matrix,
    ) -> Matrix {
        assert_eq!(a.rows, b.rows);
        assert_eq!(a.cols, b.cols);
        let mut out = Matrix::zeros(a.rows, a.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                out.data[i][j] = a.data[i][j] - b.data[i][j];
            }
        }
        out
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_multiply(
        &self,
        a: &Matrix,
        b: &Matrix,
    ) -> Matrix {
        assert_eq!(a.cols, b.rows);
        let mut out = Matrix::zeros(a.rows, b.cols);
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    sum += a.data[i][k] * b.data[k][j];
                }
                out.data[i][j] = sum;
            }
        }
        out
    }

    pub(in crate::analysis::advanced::pole_zero) fn solve_matrix_columns_regularized(
        &self,
        a: &Matrix,
        b: &Matrix,
    ) -> Option<Matrix> {
        assert_eq!(a.rows, a.cols);
        assert_eq!(a.rows, b.rows);

        let scale = a
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max)
            .max(1.0);

        // Try exact solve first, then progressively stronger diagonal regularization.
        let regularizations = [
            0.0,
            1e-18 * scale,
            1e-15 * scale,
            1e-12 * scale,
            1e-9 * scale,
            1e-6 * scale,
        ];
        for &eps in &regularizations {
            let mut a_reg = a.clone();
            if eps > 0.0 {
                for i in 0..a_reg.rows.min(a_reg.cols) {
                    a_reg.data[i][i] += eps;
                }
            }
            if let Some(x) = self.solve_matrix_columns(&a_reg, b) {
                return Some(x);
            }
        }

        None
    }

    pub(in crate::analysis::advanced::pole_zero) fn solve_matrix_columns(
        &self,
        a: &Matrix,
        b: &Matrix,
    ) -> Option<Matrix> {
        assert_eq!(a.rows, a.cols);
        assert_eq!(a.rows, b.rows);

        let mut out = Matrix::zeros(a.rows, b.cols);
        let triangular = self.triangular_kind(a, self.relative_matrix_tolerance(a, 1e-12));
        for col in 0..b.cols {
            let rhs: Vec<Value> = (0..b.rows).map(|r| b.data[r][col]).collect();
            let x = if let Some(kind) = triangular {
                self.solve_triangular(a, &rhs, kind)?
            } else {
                self.solve_linear(a, &rhs)?
            };
            for (row, value) in x.into_iter().enumerate() {
                out.data[row][col] = value;
            }
        }
        Some(out)
    }

    pub(in crate::analysis::advanced::pole_zero) fn qr_eigenvalues(
        &self,
        matrix: &Matrix,
    ) -> Vec<Complex> {
        let n = matrix.rows;
        if n == 0 {
            return Vec::new();
        }
        let scale = self.matrix_eigen_scale(matrix);
        let scaled = self.scale_matrix(matrix, 1.0 / scale);
        let tol = 1e-10;
        if let Some(diagonal_roots) = self.triangular_diagonal_eigenvalues(&scaled, tol) {
            return diagonal_roots
                .into_iter()
                .map(|root| Complex::new(root.re * scale, root.im * scale))
                .collect();
        }
        if n == 1 {
            return vec![Complex::real(scaled.data[0][0] * scale)];
        }
        if n == 2 {
            return self
                .eigenvalues_2x2(
                    scaled.data[0][0],
                    scaled.data[0][1],
                    scaled.data[1][0],
                    scaled.data[1][1],
                )
                .into_iter()
                .map(|root| Complex::new(root.re * scale, root.im * scale))
                .collect();
        }

        let max_iter = 2000;
        let mut a = scaled.data.clone();

        for _ in 0..max_iter {
            let mut converged = true;
            for i in 1..n {
                if a[i][i - 1].abs() > tol {
                    converged = false;
                    break;
                }
            }
            if converged {
                break;
            }

            // Basic shifted QR iteration.
            let shift = a[n - 1][n - 1];
            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] -= shift;
            }

            let (q, r) = self.qr_decompose(&a);
            a = self.matrix_multiply_raw(&r, &q);

            for (i, row) in a.iter_mut().enumerate().take(n) {
                row[i] += shift;
            }
        }

        let mut eigenvalues = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            if i == n - 1 || a[i + 1][i].abs() < tol {
                eigenvalues.push(Complex::real(a[i][i]));
                i += 1;
            } else {
                eigenvalues.extend(self.eigenvalues_2x2(
                    a[i][i],
                    a[i][i + 1],
                    a[i + 1][i],
                    a[i + 1][i + 1],
                ));
                i += 2;
            }
        }

        eigenvalues
            .into_iter()
            .map(|root| Complex::new(root.re * scale, root.im * scale))
            .collect()
    }

    pub(in crate::analysis::advanced::pole_zero) fn eigenvalues_2x2(
        &self,
        a00: Value,
        a01: Value,
        a10: Value,
        a11: Value,
    ) -> Vec<Complex> {
        let trace = a00 + a11;
        let det = a00 * a11 - a01 * a10;
        let discriminant = trace * trace - 4.0 * det;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            vec![
                Complex::real((trace + sqrt_d) / 2.0),
                Complex::real((trace - sqrt_d) / 2.0),
            ]
        } else {
            let sqrt_d = (-discriminant).sqrt() / 2.0;
            vec![
                Complex::new(trace / 2.0, sqrt_d),
                Complex::new(trace / 2.0, -sqrt_d),
            ]
        }
    }

    pub(in crate::analysis::advanced::pole_zero) fn qr_decompose(
        &self,
        a: &[Vec<Value>],
    ) -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let n = a.len();
        let mut q = vec![vec![0.0; n]; n];
        let mut r = vec![vec![0.0; n]; n];
        let cols: Vec<Vec<Value>> = (0..n).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

        for j in 0..n {
            let mut v = cols[j].clone();
            for i in 0..j {
                let q_col: Vec<Value> = (0..n).map(|k| q[k][i]).collect();
                let dot: Value = v.iter().zip(&q_col).map(|(x, y)| x * y).sum();
                r[i][j] = dot;
                for k in 0..n {
                    v[k] -= dot * q_col[k];
                }
            }

            let norm = v.iter().map(|x| x * x).sum::<Value>().sqrt();
            r[j][j] = norm;
            if norm > 1e-15 {
                for k in 0..n {
                    q[k][j] = v[k] / norm;
                }
            }
        }

        (q, r)
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_multiply_raw(
        &self,
        a: &[Vec<Value>],
        b: &[Vec<Value>],
    ) -> Vec<Vec<Value>> {
        let n = a.len();
        let mut out = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[i][k] * b[k][j];
                }
                out[i][j] = sum;
            }
        }
        out
    }

    /// Fallback pole estimator for highly singular systems.
    pub(in crate::analysis::advanced::pole_zero) fn eigenvalues_diagonal_fallback(
        &self,
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        let n = self.num_nodes;
        let mut poles = Vec::new();
        for i in 0..n {
            let g = self.g_matrix.get(i, i);
            let c = self.c_matrix.get(i, i);
            if c.abs() > 1e-15 && g.abs() > 1e-15 {
                let pole = -g / c;
                if pole.abs() < config.max_pole_freq * 2.0 * PI {
                    poles.push(Complex::real(pole));
                }
            }
        }
        poles.sort_by(|a, b| {
            let a_re = if a.re.is_finite() {
                a.re
            } else {
                f64::INFINITY
            };
            let b_re = if b.re.is_finite() {
                b.re
            } else {
                f64::INFINITY
            };
            a_re.total_cmp(&b_re)
        });
        poles.dedup_by(|a, b| (a.re - b.re).abs() < 1e-6);
        poles
    }
}
