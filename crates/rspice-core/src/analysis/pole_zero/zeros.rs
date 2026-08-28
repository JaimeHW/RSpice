use super::*;

impl PoleZeroAnalyzer {
    pub(in crate::analysis::pole_zero) fn numerator_roots_raw(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
        config: &PoleZeroConfig,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        if self.num_nodes == 0 {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "zero extraction requires a non-empty descriptor".to_string(),
            ));
        }
        if self.num_nodes == 1 {
            return Ok(Vec::new());
        }
        if self.num_nodes == 2 {
            return Ok(self
                .numerator_root_2x2(input_vec, output_vec)?
                .into_iter()
                .collect());
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

        let spectrum = match self.generalized_eigenvalues(&g_aug, &c_aug) {
            Err(PoleZeroAnalysisError::IrregularDescriptor { .. }) => {
                return Err(PoleZeroAnalysisError::TransferExtraction(
                    "transfer numerator is identically zero",
                ));
            }
            result => result?,
        };
        debug_assert_eq!(spectrum.finite.len() + spectrum.infinite, n + 1);
        let mut zeros = spectrum.finite;
        self.certify_numerically_real_zero_pairs(&mut zeros, &g_aug, &c_aug);
        self.ensure_roots_within_frequency_limit(&zeros, config, "zero")?;
        Ok(zeros)
    }

    pub(in crate::analysis::pole_zero) fn to_faer_matrix(&self, matrix: &Matrix) -> Mat<f64> {
        let mut out = Mat::zeros(matrix.rows, matrix.cols);
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                out[(row, col)] = matrix.data[row][col];
            }
        }
        out
    }

    pub(in crate::analysis::pole_zero) fn eigenvalues_from_matrix(
        &self,
        matrix: &Matrix,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        if matrix.rows == 0 || matrix.rows != matrix.cols {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "state matrix must be non-empty and square".to_string(),
            ));
        }
        let scale = self.matrix_eigen_scale(matrix);
        let scaled = self.scale_matrix(matrix, 1.0 / scale);
        if let Some(diagonal_roots) = self.triangular_diagonal_eigenvalues(&scaled, 0.0) {
            return Ok(diagonal_roots
                .into_iter()
                .map(|root| Complex64::new(root.re * scale, root.im * scale))
                .collect());
        }
        let faer_matrix = self.to_faer_matrix(&scaled);
        let eigen = faer::linalg::solvers::Eigen::<f64>::new_from_real(faer_matrix.as_ref())
            .map_err(|_| PoleZeroAnalysisError::EigenvalueFailure {
                problem: "state-space",
            })?;
        let spectrum = eigen.S().column_vector();
        let mut eigenvalues = Vec::with_capacity(matrix.rows);
        for idx in 0..matrix.rows {
            let value = *spectrum.get(idx);
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(PoleZeroAnalysisError::NonFiniteEigenvalue {
                    problem: "state-space",
                    index: idx,
                });
            }
            eigenvalues.push(Complex64::new(value.re * scale, value.im * scale));
        }
        if eigenvalues.len() != matrix.rows {
            return Err(PoleZeroAnalysisError::IncompleteSpectrum {
                problem: "state-space",
                expected: matrix.rows,
                actual: eigenvalues.len(),
            });
        }
        Ok(eigenvalues)
    }

    pub(in crate::analysis::pole_zero) fn generalized_eigenvalues(
        &self,
        g_matrix: &Matrix,
        c_matrix: &Matrix,
    ) -> Result<GeneralizedSpectrum, PoleZeroAnalysisError> {
        let n = g_matrix.rows;
        if n == 0 || g_matrix.rows != g_matrix.cols || c_matrix.rows != c_matrix.cols {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "generalized eigenvalue matrices must be non-empty and square".to_string(),
            ));
        }
        if g_matrix.rows != c_matrix.rows {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "generalized eigenvalue matrices must have equal dimensions".to_string(),
            ));
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
        let gevd =
            GeneralizedEigen::<f64>::new_from_real(a.as_ref(), b.as_ref()).map_err(|_| {
                PoleZeroAnalysisError::EigenvalueFailure {
                    problem: "generalized descriptor",
                }
            })?;
        let alpha = gevd.S_a().column_vector();
        let beta = gevd.S_b().column_vector();

        let mut eigenvalues = Vec::with_capacity(n);
        let mut infinite = 0;
        for idx in 0..n {
            let alpha = *alpha.get(idx);
            let beta = *beta.get(idx);
            if !alpha.re.is_finite()
                || !alpha.im.is_finite()
                || !beta.re.is_finite()
                || !beta.im.is_finite()
            {
                return Err(PoleZeroAnalysisError::NonFiniteEigenvalue {
                    problem: "generalized descriptor",
                    index: idx,
                });
            }

            let alpha_norm = alpha.norm();
            let beta_norm = beta.norm();
            if alpha_norm == 0.0 && beta_norm == 0.0 {
                return Err(PoleZeroAnalysisError::IrregularDescriptor {
                    index: idx,
                    alpha_norm,
                    beta_norm,
                });
            }
            // QZ represents an eigenvalue at infinity with beta == 0. A
            // relative small-beta cutoff is not valid here: it converts
            // perfectly finite, high-magnitude roots into infinite roots.
            if beta_norm == 0.0 {
                infinite += 1;
                continue;
            }

            let lambda = alpha / beta;
            if !lambda.re.is_finite() || !lambda.im.is_finite() {
                return Err(PoleZeroAnalysisError::NonFiniteEigenvalue {
                    problem: "generalized descriptor",
                    index: idx,
                });
            }
            eigenvalues.push(Complex64::new(lambda.re, lambda.im));
        }

        if eigenvalues.len() + infinite != n {
            return Err(PoleZeroAnalysisError::IncompleteSpectrum {
                problem: "generalized descriptor",
                expected: n,
                actual: eigenvalues.len() + infinite,
            });
        }

        Ok(GeneralizedSpectrum {
            finite: eigenvalues,
            infinite,
        })
    }

    pub(in crate::analysis::pole_zero) fn zeros_from_state_space(
        &self,
        model: &StateSpaceModel,
        config: &PoleZeroConfig,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        let n = model.a.rows;
        if n == 0 {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "state-space zero extraction requires at least one state".to_string(),
            ));
        }
        if n == 2 {
            let zeros = self.state_space_zeros_2x2(model)?;
            return self.finalize_zero_roots(zeros, config);
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

        let spectrum = match self.generalized_eigenvalues(&g_zero, &c_zero) {
            Err(PoleZeroAnalysisError::IrregularDescriptor { .. }) => {
                return Err(PoleZeroAnalysisError::TransferExtraction(
                    "transfer numerator is identically zero",
                ));
            }
            result => result?,
        };
        debug_assert_eq!(spectrum.finite.len() + spectrum.infinite, n + 1);
        let mut zeros = spectrum.finite;
        self.certify_numerically_real_zero_pairs(&mut zeros, &g_zero, &c_zero);
        self.finalize_zero_roots(zeros, config)
    }

    /// Collapse a tiny conjugate split only when the real midpoint makes the
    /// original real Rosenbrock pencil numerically rank deficient. Repeated
    /// real roots are ill-conditioned and a generic QZ solve may split them;
    /// the singular-value certificate avoids distance-only root snapping.
    fn certify_numerically_real_zero_pairs(
        &self,
        roots: &mut [Complex64],
        g_matrix: &Matrix,
        c_matrix: &Matrix,
    ) {
        let order = g_matrix.rows.max(1);
        let rank_tolerance = 1024.0 * order as Value * Value::EPSILON;
        let conjugacy_tolerance = 256.0 * Value::EPSILON;

        for left in 0..roots.len() {
            if roots[left].im == 0.0 || !roots[left].re.is_finite() || !roots[left].im.is_finite() {
                continue;
            }
            for right in (left + 1)..roots.len() {
                if !roots[right].re.is_finite() || !roots[right].im.is_finite() {
                    continue;
                }
                let real_scale = 1.0 + roots[left].re.abs().max(roots[right].re.abs());
                let imaginary_scale = 1.0 + roots[left].im.abs().max(roots[right].im.abs());
                if (roots[left].re - roots[right].re).abs() > conjugacy_tolerance * real_scale
                    || (roots[left].im + roots[right].im).abs()
                        > conjugacy_tolerance * imaginary_scale
                {
                    continue;
                }

                let real = 0.5 * (roots[left].re + roots[right].re);
                let imaginary = roots[left].im.abs().max(roots[right].im.abs());
                if imaginary > rank_tolerance.sqrt() * (1.0 + real.abs()) {
                    continue;
                }

                let mut candidate = Matrix::zeros(g_matrix.rows, g_matrix.cols);
                for row in 0..g_matrix.rows {
                    for col in 0..g_matrix.cols {
                        candidate.set(
                            row,
                            col,
                            g_matrix.get(row, col) + real * c_matrix.get(row, col),
                        );
                    }
                }
                let scale = self.matrix_eigen_scale(&candidate);
                let scaled = self.scale_matrix(&candidate, 1.0 / scale);
                let matrix = self.to_faer_matrix(&scaled);
                let Ok(svd) = faer::linalg::solvers::Svd::<f64>::new(matrix.as_ref()) else {
                    continue;
                };
                let singular_values = svd.S().column_vector();
                let mut largest = 0.0_f64;
                let mut smallest = Value::INFINITY;
                for index in 0..singular_values.nrows() {
                    let value = singular_values.get(index).abs();
                    if !value.is_finite() {
                        smallest = Value::INFINITY;
                        break;
                    }
                    largest = largest.max(value);
                    smallest = smallest.min(value);
                }
                if largest > 0.0 && smallest <= rank_tolerance * largest {
                    roots[left] = Complex64::new(real, 0.0);
                    roots[right] = Complex64::new(real, 0.0);
                    break;
                }
            }
        }
    }

    /// Form the exact real numerator polynomial for a second-order SISO
    /// state-space model instead of recovering it through a larger Rosenbrock
    /// pencil. Besides being cheaper, this preserves repeated real zeros: a
    /// generic QZ solve can split a double root into a small conjugate pair.
    fn state_space_zeros_2x2(
        &self,
        model: &StateSpaceModel,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        let a00 = model.a.get(0, 0);
        let a01 = model.a.get(0, 1);
        let a10 = model.a.get(1, 0);
        let a11 = model.a.get(1, 1);
        let b0 = model.b[0];
        let b1 = model.b[1];
        let c0 = model.c[0];
        let c1 = model.c[1];
        let d = model.d;

        // det(sI-A) H(s) = q2*s^2 + q1*s + q0.
        let q2_terms = [d];
        let q1_terms = [c0 * b0, c1 * b1, -d * a00, -d * a11];
        let q0_terms = [
            -c0 * a11 * b0,
            c0 * a01 * b1,
            c1 * a10 * b0,
            -c1 * a00 * b1,
            d * a00 * a11,
            -d * a01 * a10,
        ];
        let coefficient = |terms: &[Value]| -> Result<(Value, Value), PoleZeroAnalysisError> {
            if terms.iter().any(|term| !term.is_finite()) {
                return Err(PoleZeroAnalysisError::TransferExtraction(
                    "second-order numerator coefficients are non-finite",
                ));
            }
            Ok((
                terms.iter().sum(),
                terms.iter().map(|term| term.abs()).sum(),
            ))
        };
        let (q2, q2_scale) = coefficient(&q2_terms)?;
        let (q1, q1_scale) = coefficient(&q1_terms)?;
        let (q0, q0_scale) = coefficient(&q0_terms)?;
        let coefficient_is_zero = |value: Value, scale: Value| {
            scale == 0.0 || value.abs() <= 64.0 * Value::EPSILON * scale
        };
        let q2_is_zero = coefficient_is_zero(q2, q2_scale);
        let q1_is_zero = coefficient_is_zero(q1, q1_scale);
        let q0_is_zero = coefficient_is_zero(q0, q0_scale);

        if q2_is_zero && q1_is_zero && q0_is_zero {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "transfer numerator is identically zero",
            ));
        }
        if q2_is_zero && q1_is_zero {
            return Ok(Vec::new());
        }
        if q2_is_zero {
            let root = -q0 / q1;
            return root
                .is_finite()
                .then(|| vec![Complex64::new(root, 0.0)])
                .ok_or(PoleZeroAnalysisError::TransferExtraction(
                    "second-order numerator root is non-finite",
                ));
        }

        let polynomial_scale = q2.abs().max(q1.abs()).max(q0.abs());
        let a = q2 / polynomial_scale;
        let b = q1 / polynomial_scale;
        let c = q0 / polynomial_scale;
        let b_squared = b * b;
        let four_ac = 4.0 * a * c;
        let mut discriminant = b_squared - four_ac;
        let discriminant_error = 128.0 * Value::EPSILON * (b_squared.abs() + four_ac.abs());
        if discriminant.abs() <= discriminant_error {
            discriminant = 0.0;
        }

        let roots = if discriminant == 0.0 {
            let root = -b / (2.0 * a);
            vec![Complex64::new(root, 0.0), Complex64::new(root, 0.0)]
        } else if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let q = -0.5 * (b + sqrt_discriminant.copysign(b));
            let first = q / a;
            let second = c / q;
            vec![Complex64::new(first, 0.0), Complex64::new(second, 0.0)]
        } else {
            let real = -b / (2.0 * a);
            let imaginary = (-discriminant).sqrt() / (2.0 * a.abs());
            vec![
                Complex64::new(real, imaginary),
                Complex64::new(real, -imaginary),
            ]
        };
        if roots
            .iter()
            .any(|root| !root.re.is_finite() || !root.im.is_finite())
        {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "second-order numerator roots are non-finite",
            ));
        }
        Ok(roots)
    }

    pub(in crate::analysis::pole_zero) fn build_voltage_input_transfer_system(
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

    pub(in crate::analysis::pole_zero) fn finalize_zero_roots(
        &self,
        mut zeros: Vec<Complex64>,
        config: &PoleZeroConfig,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        self.ensure_roots_within_frequency_limit(&zeros, config, "zero")?;
        // Do not cancel nearby pole/zero pairs solely by geometric distance.
        // Legitimate near-cancellations are important conditioning evidence;
        // cancellation requires a certified common factor, which the current
        // numerical root lists do not provide.
        self.sort_roots(&mut zeros);
        Ok(zeros)
    }

    pub(in crate::analysis::pole_zero) fn numerator_root_2x2(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
    ) -> Result<Option<Complex64>, PoleZeroAnalysisError> {
        if input_vec.len() != 2 || output_vec.len() != 2 {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 numerator vectors have inconsistent dimensions",
            ));
        }
        if input_vec
            .iter()
            .chain(output_vec)
            .any(|value| !value.is_finite())
        {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 numerator vectors contain a non-finite value",
            ));
        }

        // Normalize by factors common to both numerator coefficients. This
        // prevents overflow/underflow and makes polynomial-degree detection
        // invariant to common descriptor and port-vector scaling.
        let descriptor_scale = self
            .g_matrix
            .data
            .iter()
            .chain(&self.c_matrix.data)
            .flatten()
            .map(|value| value.abs())
            .fold(0.0_f64, Value::max);
        let input_scale = input_vec
            .iter()
            .map(|value| value.abs())
            .fold(0.0_f64, Value::max);
        let output_scale = output_vec
            .iter()
            .map(|value| value.abs())
            .fold(0.0_f64, Value::max);
        if descriptor_scale == 0.0 || input_scale == 0.0 || output_scale == 0.0 {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 transfer numerator is identically zero",
            ));
        }

        let b1 = input_vec[0] / input_scale;
        let b2 = input_vec[1] / input_scale;
        let l1 = output_vec[0] / output_scale;
        let l2 = output_vec[1] / output_scale;

        let g11 = self.g_matrix.get(0, 0) / descriptor_scale;
        let g12 = self.g_matrix.get(0, 1) / descriptor_scale;
        let g21 = self.g_matrix.get(1, 0) / descriptor_scale;
        let g22 = self.g_matrix.get(1, 1) / descriptor_scale;
        let c11 = self.c_matrix.get(0, 0) / descriptor_scale;
        let c12 = self.c_matrix.get(0, 1) / descriptor_scale;
        let c21 = self.c_matrix.get(1, 0) / descriptor_scale;
        let c22 = self.c_matrix.get(1, 1) / descriptor_scale;

        // N(s) = L^T * adj(G + sC) * B = a + b*s for 2x2 systems.
        let a_terms = [l1 * g22 * b1, -l1 * g12 * b2, -l2 * g21 * b1, l2 * g11 * b2];
        let b_terms = [l1 * c22 * b1, -l1 * c12 * b2, -l2 * c21 * b1, l2 * c11 * b2];
        let a = a_terms.iter().sum::<Value>();
        let b = b_terms.iter().sum::<Value>();
        let a_scale = a_terms.iter().map(|term| term.abs()).sum::<Value>();
        let b_scale = b_terms.iter().map(|term| term.abs()).sum::<Value>();
        if !a.is_finite() || !b.is_finite() || !a_scale.is_finite() || !b_scale.is_finite() {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 numerator coefficients are non-finite",
            ));
        }

        let coefficient_is_zero = |value: Value, scale: Value| {
            scale == 0.0 || value.abs() <= 64.0 * Value::EPSILON * scale
        };
        let a_is_zero = coefficient_is_zero(a, a_scale);
        let b_is_zero = coefficient_is_zero(b, b_scale);
        if a_is_zero && b_is_zero {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 transfer numerator is identically zero",
            ));
        }
        if b_is_zero {
            return Ok(None);
        }

        let root = if a_is_zero { 0.0 } else { -a / b };
        if root.is_finite() {
            Ok(Some(Complex64::new(root, 0.0)))
        } else {
            Err(PoleZeroAnalysisError::TransferExtraction(
                "2x2 numerator root is non-finite",
            ))
        }
    }

    /// Find zeros.
    ///
    /// Uses the Rosenbrock system matrix for SISO transfer numerator extraction:
    ///
    /// det([G + s*C, -B; L^T, 0]) = 0
    ///
    /// where B is the input excitation vector and L selects a measured voltage
    /// (including differential references).
    pub(crate) fn find_zeros(
        &self,
        config: &PoleZeroConfig,
    ) -> Result<Vec<Complex64>, PoleZeroAnalysisError> {
        if self.num_nodes == 0 {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "zero extraction requires a non-empty descriptor".to_string(),
            ));
        }
        if self.is_direct_voltage_port_measurement(config) {
            return Ok(Vec::new());
        }

        let (input_vec, output_vec) =
            self.build_port_vectors(config)
                .ok_or(PoleZeroAnalysisError::TransferExtraction(
                    "input or output port is invalid",
                ))?;

        if config.input_is_current {
            if let Some(state_space) = self.build_state_space(&input_vec, &output_vec) {
                return self.zeros_from_state_space(&state_space, config);
            }

            let zeros = self.numerator_roots_raw(&input_vec, &output_vec, config)?;
            return self.finalize_zero_roots(zeros, config);
        }

        let Some((voltage_analyzer, drive_vec, output_ext)) =
            self.build_voltage_input_transfer_system(config, &output_vec)
        else {
            return Err(PoleZeroAnalysisError::TransferExtraction(
                "voltage input source could not be constructed",
            ));
        };

        if let Some(state_space) = voltage_analyzer.build_state_space(&drive_vec, &output_ext) {
            return voltage_analyzer.zeros_from_state_space(&state_space, config);
        }

        let zeros = voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config)?;
        self.finalize_zero_roots(zeros, config)
    }
}
