use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::advanced::pole_zero) fn numerator_roots_raw(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        if self.num_nodes == 0 {
            return Vec::new();
        }
        if self.num_nodes == 1 {
            return Vec::new();
        }
        if self.num_nodes == 2 {
            if let Some(root) = self.numerator_root_2x2(input_vec, output_vec) {
                return vec![root];
            }
            return Vec::new();
        }

        let n = self.num_nodes;
        let mut g_aug = Matrix::zeros(n + 1, n + 1);
        let mut c_aug = Matrix::zeros(n + 1, n + 1);

        for i in 0..n {
            for j in 0..n {
                g_aug.set(i, j, self.g_matrix.get(i, j));
                c_aug.set(i, j, self.c_matrix.get(i, j));
            }
            g_aug.set(i, n, -input_vec[i]);
            g_aug.set(n, i, output_vec[i]);
        }

        self.generalized_eigenvalues(&g_aug, &c_aug)
            .map(|mut roots| {
                roots.retain(|r| {
                    r.re.is_finite()
                        && r.im.is_finite()
                        && r.magnitude() < config.max_pole_freq * 2.0 * PI
                });
                roots
            })
            .unwrap_or_default()
    }

    pub(in crate::analysis::advanced::pole_zero) fn to_faer_matrix(
        &self,
        matrix: &Matrix,
    ) -> Mat<f64> {
        let mut out = Mat::zeros(matrix.rows, matrix.cols);
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                out[(row, col)] = matrix.data[row][col];
            }
        }
        out
    }

    pub(in crate::analysis::advanced::pole_zero) fn eigenvalues_from_matrix(
        &self,
        matrix: &Matrix,
    ) -> Option<Vec<Complex>> {
        if matrix.rows == 0 || matrix.rows != matrix.cols {
            return None;
        }
        let scale = self.matrix_eigen_scale(matrix);
        let scaled = self.scale_matrix(matrix, 1.0 / scale);
        if let Some(diagonal_roots) = self.triangular_diagonal_eigenvalues(&scaled, 1e-10) {
            return Some(
                diagonal_roots
                    .into_iter()
                    .map(|root| Complex::new(root.re * scale, root.im * scale))
                    .collect(),
            );
        }
        let faer_matrix = self.to_faer_matrix(&scaled);
        let eigen =
            faer::linalg::solvers::Eigen::<f64>::new_from_real(faer_matrix.as_ref()).ok()?;
        let spectrum = eigen.S().column_vector();
        let mut eigenvalues = Vec::with_capacity(matrix.rows);
        for idx in 0..matrix.rows {
            let value = *spectrum.get(idx);
            if value.re.is_finite() && value.im.is_finite() {
                eigenvalues.push(Complex::new(value.re * scale, value.im * scale));
            }
        }
        Some(eigenvalues)
    }

    pub(in crate::analysis::advanced::pole_zero) fn generalized_eigenvalues(
        &self,
        g_matrix: &Matrix,
        c_matrix: &Matrix,
    ) -> Option<Vec<Complex>> {
        let n = g_matrix.rows;
        if n == 0 || g_matrix.rows != g_matrix.cols || c_matrix.rows != c_matrix.cols {
            return None;
        }
        if g_matrix.rows != c_matrix.rows {
            return None;
        }

        let scale = self
            .matrix_eigen_scale(g_matrix)
            .max(self.matrix_eigen_scale(c_matrix));
        let g_scaled = self.scale_matrix(g_matrix, 1.0 / scale);
        let c_scaled = self.scale_matrix(c_matrix, 1.0 / scale);

        let mut a = self.to_faer_matrix(&g_scaled);
        for row in 0..n {
            for col in 0..n {
                a[(row, col)] = -a[(row, col)];
            }
        }
        let b = self.to_faer_matrix(&c_scaled);
        let gevd = GeneralizedEigen::<f64>::new_from_real(a.as_ref(), b.as_ref()).ok()?;
        let alpha = gevd.S_a().column_vector();
        let beta = gevd.S_b().column_vector();

        let mut eigenvalues = Vec::with_capacity(n);
        for idx in 0..n {
            let alpha = *alpha.get(idx);
            let beta = *beta.get(idx);
            if !alpha.re.is_finite()
                || !alpha.im.is_finite()
                || !beta.re.is_finite()
                || !beta.im.is_finite()
            {
                continue;
            }

            let beta_norm = beta.norm();
            if beta_norm <= 1e-18 {
                continue;
            }

            let lambda = alpha / beta;
            if lambda.re.is_finite() && lambda.im.is_finite() {
                eigenvalues.push(Complex::new(lambda.re, lambda.im));
            }
        }

        Some(eigenvalues)
    }

    pub(in crate::analysis::advanced::pole_zero) fn zeros_from_state_space(
        &self,
        model: &StateSpaceModel,
        poles: &[Complex],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        let n = model.a.rows;
        if n == 0 {
            return Vec::new();
        }

        let mut g_zero = Matrix::zeros(n + 1, n + 1);
        let mut c_zero = Matrix::zeros(n + 1, n + 1);

        for row in 0..n {
            for col in 0..n {
                g_zero.set(row, col, -model.a.get(row, col));
            }
            g_zero.set(row, n, -model.b[row]);
            c_zero.set(row, row, 1.0);
        }
        for col in 0..n {
            g_zero.set(n, col, model.c[col]);
        }
        g_zero.set(n, n, model.d);

        let zeros = self
            .generalized_eigenvalues(&g_zero, &c_zero)
            .unwrap_or_default();
        self.finalize_zero_roots(zeros, poles, config)
    }

    pub(in crate::analysis::advanced::pole_zero) fn build_voltage_input_transfer_system(
        &self,
        config: &PoleZeroConfig,
        output_vec: &[Value],
    ) -> Option<(PoleZeroAnalyzer, Vec<Value>, Vec<Value>)> {
        if let Some(input_voltage_branch) = config.input_voltage_branch {
            if input_voltage_branch >= self.num_nodes || output_vec.len() != self.num_nodes {
                return None;
            }

            let mut drive_vec = vec![0.0; self.num_nodes];
            drive_vec[input_voltage_branch] = config.input_voltage_gain;
            return Some((
                PoleZeroAnalyzer::new(self.g_matrix.clone(), self.c_matrix.clone()),
                drive_vec,
                output_vec.to_vec(),
            ));
        }

        let n = self.num_nodes;
        if output_vec.len() != n || config.input_pos >= n {
            return None;
        }

        let mut g_ext = Matrix::zeros(n + 1, n + 1);
        let mut c_ext = Matrix::zeros(n + 1, n + 1);

        for i in 0..n {
            for j in 0..n {
                g_ext.set(i, j, self.g_matrix.get(i, j));
                c_ext.set(i, j, self.c_matrix.get(i, j));
            }
        }

        let branch = n;
        g_ext.add(config.input_pos, branch, 1.0);
        g_ext.add(branch, config.input_pos, 1.0);
        if let Some(input_neg) = config.input_neg {
            if input_neg >= n {
                return None;
            }
            g_ext.add(input_neg, branch, -1.0);
            g_ext.add(branch, input_neg, -1.0);
        }

        let mut drive_vec = vec![0.0; n + 1];
        drive_vec[branch] = 1.0;

        let mut output_ext = vec![0.0; n + 1];
        output_ext[..n].copy_from_slice(output_vec);

        Some((PoleZeroAnalyzer::new(g_ext, c_ext), drive_vec, output_ext))
    }

    pub(in crate::analysis::advanced::pole_zero) fn finalize_zero_roots(
        &self,
        mut zeros: Vec<Complex>,
        poles: &[Complex],
        config: &PoleZeroConfig,
    ) -> Vec<Complex> {
        let finite_zero_limit = self.finite_zero_limit(poles, config);
        zeros.retain(|z| z.magnitude() <= finite_zero_limit);
        zeros.retain(|z| !poles.iter().any(|p| Self::is_same_root(z, p, 1e-4)));
        self.sort_roots(&mut zeros);
        self.canonicalize_near_real_zero_pairs(&mut zeros);
        self.sort_roots(&mut zeros);
        zeros
    }

    pub(in crate::analysis::advanced::pole_zero) fn numerator_root_2x2(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
    ) -> Option<Complex> {
        if input_vec.len() != 2 || output_vec.len() != 2 {
            return None;
        }

        let b1 = input_vec[0];
        let b2 = input_vec[1];
        let l1 = output_vec[0];
        let l2 = output_vec[1];

        let g11 = self.g_matrix.get(0, 0);
        let g12 = self.g_matrix.get(0, 1);
        let g21 = self.g_matrix.get(1, 0);
        let g22 = self.g_matrix.get(1, 1);
        let c11 = self.c_matrix.get(0, 0);
        let c12 = self.c_matrix.get(0, 1);
        let c21 = self.c_matrix.get(1, 0);
        let c22 = self.c_matrix.get(1, 1);

        // N(s) = L^T * adj(G + sC) * B = a + b*s for 2x2 systems.
        let a = l1 * (g22 * b1 - g12 * b2) + l2 * (-g21 * b1 + g11 * b2);
        let b = l1 * (c22 * b1 - c12 * b2) + l2 * (-c21 * b1 + c11 * b2);
        if b.abs() < 1e-15 {
            return None;
        }

        let root = -a / b;
        if root.is_finite() {
            Some(Complex::real(root))
        } else {
            None
        }
    }

    pub(in crate::analysis::advanced::pole_zero) fn finite_zero_limit(
        &self,
        poles: &[Complex],
        config: &PoleZeroConfig,
    ) -> Value {
        let pole_scale = poles
            .iter()
            .map(|p| p.magnitude())
            .fold(1.0_f64, |acc, mag| acc.max(mag));
        (pole_scale * 1e6).min(config.max_pole_freq * 2.0 * PI)
    }

    /// Find zeros.
    ///
    /// Uses the Rosenbrock system matrix for SISO transfer numerator extraction:
    ///
    /// det([G + s*C, -B; L^T, 0]) = 0
    ///
    /// where B is the input excitation vector and L selects a measured voltage
    /// (including differential references).
    pub fn find_zeros(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        if self.num_nodes == 0 {
            return Vec::new();
        }
        if self.is_direct_voltage_port_measurement(config) {
            return Vec::new();
        }

        let (input_vec, output_vec) = match self.build_port_vectors(config) {
            Some(v) => v,
            None => return Vec::new(),
        };

        if config.input_is_current {
            if let Some(state_space) = self.build_state_space(&input_vec, &output_vec) {
                let poles = self
                    .eigenvalues_from_matrix(&state_space.a)
                    .unwrap_or_else(|| self.find_poles(config));
                return self.zeros_from_state_space(&state_space, &poles, config);
            }

            let poles = self.find_poles(config);
            let zeros = self.numerator_roots_raw(&input_vec, &output_vec, config);
            return self.finalize_zero_roots(zeros, &poles, config);
        }

        let Some((voltage_analyzer, drive_vec, output_ext)) =
            self.build_voltage_input_transfer_system(config, &output_vec)
        else {
            return Vec::new();
        };

        if let Some(state_space) = voltage_analyzer.build_state_space(&drive_vec, &output_ext) {
            let poles = voltage_analyzer
                .eigenvalues_from_matrix(&state_space.a)
                .unwrap_or_else(|| voltage_analyzer.find_poles(config));
            return voltage_analyzer.zeros_from_state_space(&state_space, &poles, config);
        }

        let poles = self.find_poles(config);
        let zeros = voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config);
        self.finalize_zero_roots(zeros, &poles, config)
    }
}
