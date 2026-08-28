use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::pole_zero) fn build_port_vectors(
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

    pub(in crate::analysis::pole_zero) fn is_direct_voltage_port_measurement(
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

    pub(in crate::analysis::pole_zero) fn sort_roots(&self, roots: &mut [Complex64]) {
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

    /// Reject a configured root cutoff that would otherwise turn a complete
    /// eigenspectrum into a silently truncated result.
    pub(in crate::analysis::pole_zero) fn ensure_roots_within_frequency_limit(
        &self,
        roots: &[Complex64],
        config: &PoleZeroConfig,
        quantity: &'static str,
    ) -> Result<(), PoleZeroAnalysisError> {
        if !config.max_pole_freq.is_finite() || config.max_pole_freq <= 0.0 {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "max_pole_freq must be finite and positive".to_string(),
            ));
        }
        let limit = config.max_pole_freq * (2.0 * PI);
        if !limit.is_finite() {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "max_pole_freq overflows when converted to angular frequency".to_string(),
            ));
        }
        let omitted = roots.iter().filter(|root| root.norm() >= limit).count();
        if omitted > 0 {
            return Err(PoleZeroAnalysisError::FrequencyLimitExceeded {
                quantity,
                omitted,
                limit,
            });
        }
        Ok(())
    }

    pub(in crate::analysis::pole_zero) fn relative_matrix_tolerance(
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

    pub(in crate::analysis::pole_zero) fn matrix_eigen_scale(&self, matrix: &Matrix) -> Value {
        let max_abs = matrix
            .data
            .iter()
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, f64::max);
        if max_abs > 0.0 && max_abs.is_finite() {
            max_abs
        } else {
            1.0
        }
    }

    pub(in crate::analysis::pole_zero) fn scale_matrix(
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

    pub(in crate::analysis::pole_zero) fn triangular_diagonal_eigenvalues(
        &self,
        matrix: &Matrix,
        tolerance: Value,
    ) -> Option<Vec<Complex64>> {
        let _ = self.triangular_kind(matrix, tolerance)?;

        Some(
            (0..matrix.rows)
                .map(|idx| Complex64::new(matrix.data[idx][idx], 0.0))
                .collect(),
        )
    }

    pub(in crate::analysis::pole_zero) fn triangular_kind(
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

    pub(in crate::analysis::pole_zero) fn solve_triangular(
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

    pub(in crate::analysis::pole_zero) fn matrix_has_stable_inverse(
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
        let Some(inverse) = self.solve_matrix_columns(matrix, &identity) else {
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
}
