use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::pole_zero) fn extract_submatrix(
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

    pub(in crate::analysis::pole_zero) fn matrix_subtract(&self, a: &Matrix, b: &Matrix) -> Matrix {
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

    pub(in crate::analysis::pole_zero) fn matrix_multiply(&self, a: &Matrix, b: &Matrix) -> Matrix {
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

    pub(in crate::analysis::pole_zero) fn solve_matrix_columns(
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
}
