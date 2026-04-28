use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::advanced::pole_zero) fn build_port_vectors(
        &self,
        config: &PoleZeroConfig,
    ) -> Option<(Vec<Value>, Vec<Value>)> {
        let n = self.num_nodes;
        let mut input_vec: Vec<Value> = vec![0.0; n];
        let mut output_vec: Vec<Value> = vec![0.0; n];

        if config.input_pos >= n || config.output_pos >= n {
            return None;
        }

        input_vec[config.input_pos] += 1.0;
        if let Some(input_neg) = config.input_neg {
            if input_neg >= n {
                return None;
            }
            input_vec[input_neg] -= 1.0;
        }

        output_vec[config.output_pos] += 1.0;
        if let Some(output_neg) = config.output_neg {
            if output_neg >= n {
                return None;
            }
            output_vec[output_neg] -= 1.0;
        }

        let input_norm = input_vec.iter().map(|v| v.abs()).sum::<Value>();
        let output_norm = output_vec.iter().map(|v| v.abs()).sum::<Value>();
        if input_norm < 1e-15 || output_norm < 1e-15 {
            return None;
        }

        Some((input_vec, output_vec))
    }

    pub(in crate::analysis::advanced::pole_zero) fn is_direct_voltage_port_measurement(
        &self,
        config: &PoleZeroConfig,
    ) -> bool {
        if config.input_is_current {
            return false;
        }

        (config.input_pos == config.output_pos && config.input_neg == config.output_neg)
            || (config.input_pos == config.output_neg.unwrap_or(usize::MAX)
                && config.output_pos == config.input_neg.unwrap_or(usize::MAX))
    }

    pub(in crate::analysis::advanced::pole_zero) fn is_same_root(
        a: &Complex,
        b: &Complex,
        tol: Value,
    ) -> bool {
        let re_scale = 1.0 + a.re.abs().max(b.re.abs());
        let im_scale = 1.0 + a.im.abs().max(b.im.abs());
        (a.re - b.re).abs() <= tol * re_scale && (a.im - b.im).abs() <= tol * im_scale
    }

    pub(in crate::analysis::advanced::pole_zero) fn sort_roots(&self, roots: &mut [Complex]) {
        roots.sort_by(|a, b| {
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
            let a_im = if a.im.is_finite() {
                a.im
            } else {
                f64::INFINITY
            };
            let b_im = if b.im.is_finite() {
                b.im
            } else {
                f64::INFINITY
            };
            a_re.total_cmp(&b_re).then_with(|| a_im.total_cmp(&b_im))
        });
    }

    pub(in crate::analysis::advanced::pole_zero) fn round_to_significant_digits(
        &self,
        value: Value,
        digits: i32,
    ) -> Value {
        if !value.is_finite() || value == 0.0 {
            return value;
        }

        let exponent = value.abs().log10().floor() as i32;
        let scale = 10.0_f64.powi(digits - exponent - 1);
        (value * scale).round() / scale
    }

    pub(in crate::analysis::advanced::pole_zero) fn canonicalize_real_roots(
        &self,
        roots: &mut [Complex],
    ) {
        for root in roots {
            if !root.re.is_finite() || !root.im.is_finite() {
                continue;
            }
            if root.im.abs() <= (1.0 + root.re.abs()) * 1e-12 {
                root.im = 0.0;
            }
            if root.im == 0.0 {
                let rounded = self.round_to_significant_digits(root.re, 8);
                let tolerance = (1.0 + root.re.abs()) * 1e-6;
                if (rounded - root.re).abs() <= tolerance {
                    root.re = rounded;
                }
            }
        }
    }

    pub(in crate::analysis::advanced::pole_zero) fn canonicalize_near_real_zero_pairs(
        &self,
        zeros: &mut [Complex],
    ) {
        let snap_ratio = 1e-6;
        let real_tolerance = 1e-9;

        for idx in 0..zeros.len().saturating_sub(1) {
            let (left, right) = zeros.split_at_mut(idx + 1);
            let a = &mut left[idx];
            let b = &mut right[0];

            if !a.re.is_finite() || !a.im.is_finite() || !b.re.is_finite() || !b.im.is_finite() {
                continue;
            }
            if (a.re - b.re).abs() > (1.0 + a.re.abs().max(b.re.abs())) * real_tolerance {
                continue;
            }
            if (a.im + b.im).abs() > (1.0 + a.im.abs().max(b.im.abs())) * real_tolerance {
                continue;
            }

            let imag_scale = a.im.abs().max(b.im.abs());
            let root_scale = 1.0 + a.re.abs().max(b.re.abs());
            if imag_scale <= root_scale * snap_ratio {
                a.re = (a.re + b.re) * 0.5;
                b.re = a.re;
                a.im = 0.0;
                b.im = 0.0;
            }
        }

        self.canonicalize_real_roots(zeros);
    }

    pub(in crate::analysis::advanced::pole_zero) fn finite_pole_count(&self) -> usize {
        self.matrix_rank(
            &self.c_matrix,
            self.relative_matrix_tolerance(&self.c_matrix, 1e-9),
        )
    }

    pub(in crate::analysis::advanced::pole_zero) fn has_complete_pole_set(
        &self,
        poles: &[Complex],
        expected: usize,
    ) -> bool {
        if expected == 0 {
            return true;
        }
        poles.len() == expected
    }

    pub(in crate::analysis::advanced::pole_zero) fn relative_matrix_tolerance(
        &self,
        matrix: &Matrix,
        relative_tolerance: Value,
    ) -> Value {
        let max_abs = matrix
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max);
        if max_abs > 0.0 {
            relative_tolerance * max_abs
        } else {
            relative_tolerance
        }
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_eigen_scale(
        &self,
        matrix: &Matrix,
    ) -> Value {
        matrix
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max)
            .max(1.0)
    }

    pub(in crate::analysis::advanced::pole_zero) fn scale_matrix(
        &self,
        matrix: &Matrix,
        factor: Value,
    ) -> Matrix {
        let mut scaled = matrix.clone();
        for row in &mut scaled.data {
            for value in row {
                *value *= factor;
            }
        }
        scaled
    }

    pub(in crate::analysis::advanced::pole_zero) fn triangular_diagonal_eigenvalues(
        &self,
        matrix: &Matrix,
        tolerance: Value,
    ) -> Option<Vec<Complex>> {
        let _ = self.triangular_kind(matrix, tolerance)?;

        Some(
            (0..matrix.rows)
                .map(|idx| Complex::real(matrix.data[idx][idx]))
                .collect(),
        )
    }

    pub(in crate::analysis::advanced::pole_zero) fn triangular_kind(
        &self,
        matrix: &Matrix,
        tolerance: Value,
    ) -> Option<TriangularKind> {
        if matrix.rows != matrix.cols {
            return None;
        }

        let mut lower = true;
        let mut upper = true;
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                let value = matrix.data[row][col].abs();
                if row > col && value > tolerance {
                    upper = false;
                }
                if col > row && value > tolerance {
                    lower = false;
                }
            }
        }

        if lower {
            Some(TriangularKind::Lower)
        } else if upper {
            Some(TriangularKind::Upper)
        } else {
            None
        }
    }

    pub(in crate::analysis::advanced::pole_zero) fn solve_triangular(
        &self,
        a: &Matrix,
        b: &[Value],
        kind: TriangularKind,
    ) -> Option<Vec<Value>> {
        let n = a.rows;
        let pivot_tolerance = self.relative_matrix_tolerance(a, 1e-15);
        let mut x = vec![0.0; n];

        match kind {
            TriangularKind::Lower => {
                for row in 0..n {
                    let pivot = a.data[row][row];
                    if pivot.abs() <= pivot_tolerance {
                        return None;
                    }
                    let mut sum = b[row];
                    for (col, value) in x.iter().enumerate().take(row) {
                        sum -= a.data[row][col] * *value;
                    }
                    x[row] = sum / pivot;
                }
            }
            TriangularKind::Upper => {
                for row in (0..n).rev() {
                    let pivot = a.data[row][row];
                    if pivot.abs() <= pivot_tolerance {
                        return None;
                    }
                    let mut sum = b[row];
                    for (col, value) in x.iter().enumerate().skip(row + 1) {
                        sum -= a.data[row][col] * *value;
                    }
                    x[row] = sum / pivot;
                }
            }
        }

        Some(x)
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_has_stable_inverse(
        &self,
        matrix: &Matrix,
    ) -> bool {
        if matrix.rows != matrix.cols {
            return false;
        }
        if matrix.rows == 0 {
            return true;
        }
        if self
            .triangular_kind(matrix, self.relative_matrix_tolerance(matrix, 1e-12))
            .is_some()
        {
            let pivot_tolerance = self.relative_matrix_tolerance(matrix, 1e-15);
            return (0..matrix.rows).all(|idx| matrix.data[idx][idx].abs() > pivot_tolerance);
        }

        let identity = Matrix::identity(matrix.rows);
        let Some(inverse) = self.solve_matrix_columns_regularized(matrix, &identity) else {
            return false;
        };
        let product = self.matrix_multiply(matrix, &inverse);
        let mut max_residual = 0.0_f64;
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                let expected = if row == col { 1.0 } else { 0.0 };
                max_residual = max_residual.max((product.data[row][col] - expected).abs());
            }
        }

        max_residual <= 1e-6
    }

    pub(in crate::analysis::advanced::pole_zero) fn matrix_rank(
        &self,
        matrix: &Matrix,
        tolerance: Value,
    ) -> usize {
        let (rows, cols) = matrix.dims();
        if rows == 0 || cols == 0 {
            return 0;
        }

        let mut data = matrix.data.clone();
        let mut rank = 0usize;
        let mut pivot_row = 0usize;

        for pivot_col in 0..cols {
            if pivot_row >= rows {
                break;
            }

            let mut best_row = pivot_row;
            let mut best_value = data[pivot_row][pivot_col].abs();
            for (row_idx, row) in data.iter().enumerate().skip(pivot_row + 1) {
                let candidate = row[pivot_col].abs();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row_idx;
                }
            }

            if best_value <= tolerance {
                continue;
            }

            data.swap(pivot_row, best_row);
            let pivot = data[pivot_row][pivot_col];
            for row_idx in (pivot_row + 1)..rows {
                let factor = data[row_idx][pivot_col] / pivot;
                if factor.abs() <= tolerance {
                    continue;
                }
                for col_idx in pivot_col..cols {
                    data[row_idx][col_idx] -= factor * data[pivot_row][col_idx];
                }
            }

            rank += 1;
            pivot_row += 1;
        }

        rank
    }
}
