use super::*;

impl PoleZeroAnalyzer {
    /// Create analyzer from G and C matrices
    ///
    /// The MNA equation is: (G + s·C)·x = b
    /// Poles are values of s where det(G + s·C) = 0
    pub fn new(g_matrix: Matrix, c_matrix: Matrix) -> Self {
        let num_nodes = g_matrix.dims().0;
        Self {
            g_matrix,
            c_matrix,
            num_nodes,
        }
    }

    /// Find poles using companion matrix method
    ///
    /// Poles are eigenvalues of -C⁻¹·G (if C is invertible)
    /// For singular C, use generalized eigenvalue: G·x = -s·C·x
    pub fn find_poles(&self, config: &PoleZeroConfig) -> Vec<Complex> {
        let n = self.num_nodes;
        if n == 0 {
            return Vec::new();
        }
        let expected_poles = self.finite_pole_count();

        // For single-node RC circuit:
        // G + s·C = 0 → s = -G/C
        if n == 1 {
            let g = self.g_matrix.get(0, 0);
            let c = self.c_matrix.get(0, 0);
            if c.abs() > 1e-15 {
                return vec![Complex::real(-g / c)];
            }
            return Vec::new();
        }

        if let Some(state_space) = self.build_state_space(&vec![0.0; n], &vec![0.0; n])
            && let Some(mut poles) = self.eigenvalues_from_matrix(&state_space.a)
        {
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        if let Some(mut poles) = self.generalized_eigenvalues(&self.g_matrix, &self.c_matrix) {
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        if let Some(state_matrix) = self.build_descriptor_state_matrix() {
            let mut poles = self.qr_eigenvalues(&state_matrix);
            self.canonicalize_real_roots(&mut poles);
            poles.retain(|p| {
                p.re.is_finite()
                    && p.im.is_finite()
                    && p.magnitude() < config.max_pole_freq * 2.0 * PI
            });
            poles.sort_by(|a, b| a.magnitude().total_cmp(&b.magnitude()));
            if expected_poles > 0 && poles.len() > expected_poles {
                poles.truncate(expected_poles);
            }
            if self.has_complete_pole_set(&poles, expected_poles) {
                return poles;
            }
        }

        // Fallback for heavily singular descriptors where state extraction fails.
        self.eigenvalues_diagonal_fallback(config)
    }

    /// Descriptor-system reduction:
    /// Cx' + Gx = 0
    /// Split x into dynamic/algebraic variables and eliminate algebraic states.
    /// This yields x_d' = A x_d where A is used for pole extraction.
    pub(in crate::analysis::advanced::pole_zero) fn build_descriptor_state_matrix(
        &self,
    ) -> Option<Matrix> {
        let partition = self.partition_descriptor()?;
        if !self.partition_is_regular(&partition) {
            return None;
        }

        let g_eff = self.reduced_g_matrix(&partition)?;
        let c_dd_inv_g_eff = self.solve_matrix_columns_regularized(&partition.c_dd, &g_eff)?;
        let mut a = c_dd_inv_g_eff;
        for row in &mut a.data {
            for value in row {
                *value = -*value;
            }
        }

        Some(a)
    }

    pub(in crate::analysis::advanced::pole_zero) fn partition_descriptor(
        &self,
    ) -> Option<DescriptorPartition> {
        let n = self.num_nodes;
        let tol = 1e-15;

        let mut dynamic = Vec::new();
        for i in 0..n {
            let row_nonzero = self.c_matrix.data[i].iter().any(|v| v.abs() > tol);
            let col_nonzero = (0..n).any(|r| self.c_matrix.data[r][i].abs() > tol);
            if row_nonzero || col_nonzero {
                dynamic.push(i);
            }
        }

        if dynamic.is_empty() {
            return None;
        }

        let mut is_dynamic = vec![false; n];
        for &idx in &dynamic {
            is_dynamic[idx] = true;
        }
        let algebraic: Vec<usize> = (0..n).filter(|i| !is_dynamic[*i]).collect();

        Some(DescriptorPartition {
            c_dd: self.extract_submatrix(&self.c_matrix, &dynamic, &dynamic),
            g_dd: self.extract_submatrix(&self.g_matrix, &dynamic, &dynamic),
            g_da: self.extract_submatrix(&self.g_matrix, &dynamic, &algebraic),
            g_ad: self.extract_submatrix(&self.g_matrix, &algebraic, &dynamic),
            g_aa: self.extract_submatrix(&self.g_matrix, &algebraic, &algebraic),
            dynamic,
            algebraic,
        })
    }

    pub(in crate::analysis::advanced::pole_zero) fn reduced_g_matrix(
        &self,
        partition: &DescriptorPartition,
    ) -> Option<Matrix> {
        if partition.algebraic.is_empty() {
            return Some(partition.g_dd.clone());
        }

        let g_aa_inv_g_ad =
            self.solve_matrix_columns_regularized(&partition.g_aa, &partition.g_ad)?;
        let correction = self.matrix_multiply(&partition.g_da, &g_aa_inv_g_ad);
        Some(self.matrix_subtract(&partition.g_dd, &correction))
    }

    pub(in crate::analysis::advanced::pole_zero) fn partition_is_regular(
        &self,
        partition: &DescriptorPartition,
    ) -> bool {
        if !self.matrix_has_stable_inverse(&partition.c_dd) {
            return false;
        }

        if partition.algebraic.is_empty() {
            return true;
        }

        self.matrix_has_stable_inverse(&partition.g_aa)
    }

    pub(in crate::analysis::advanced::pole_zero) fn extract_subvector(
        &self,
        values: &[Value],
        indices: &[usize],
    ) -> Vec<Value> {
        indices.iter().map(|&idx| values[idx]).collect()
    }

    pub(in crate::analysis::advanced::pole_zero) fn vector_to_column_matrix(
        &self,
        values: &[Value],
    ) -> Matrix {
        let mut column = Matrix::zeros(values.len(), 1);
        for (row, value) in values.iter().enumerate() {
            column.data[row][0] = *value;
        }
        column
    }

    pub(in crate::analysis::advanced::pole_zero) fn column_matrix_to_vector(
        &self,
        column: &Matrix,
    ) -> Vec<Value> {
        (0..column.rows).map(|row| column.data[row][0]).collect()
    }

    pub(in crate::analysis::advanced::pole_zero) fn row_vector_times_matrix(
        &self,
        row: &[Value],
        matrix: &Matrix,
    ) -> Vec<Value> {
        assert_eq!(row.len(), matrix.rows);
        let mut out = vec![0.0; matrix.cols];
        for (r, weight) in row.iter().copied().enumerate() {
            if weight == 0.0 {
                continue;
            }
            for (c, target) in out.iter_mut().enumerate().take(matrix.cols) {
                *target += weight * matrix.data[r][c];
            }
        }
        out
    }

    pub(in crate::analysis::advanced::pole_zero) fn build_state_space(
        &self,
        input_vec: &[Value],
        output_vec: &[Value],
    ) -> Option<StateSpaceModel> {
        let partition = self.partition_descriptor()?;
        if !self.partition_is_regular(&partition) {
            return None;
        }
        let g_eff = self.reduced_g_matrix(&partition)?;

        let b_d = self.extract_subvector(input_vec, &partition.dynamic);
        let l_d = self.extract_subvector(output_vec, &partition.dynamic);

        let (b_eff, c_eff, d_eff) = if partition.algebraic.is_empty() {
            (b_d, l_d, 0.0)
        } else {
            let b_a = self.extract_subvector(input_vec, &partition.algebraic);
            let l_a = self.extract_subvector(output_vec, &partition.algebraic);
            let g_aa_inv_g_ad =
                self.solve_matrix_columns_regularized(&partition.g_aa, &partition.g_ad)?;
            let g_aa_inv_ba = self.solve_matrix_columns_regularized(
                &partition.g_aa,
                &self.vector_to_column_matrix(&b_a),
            )?;

            let gda_ginv_ba = self.matrix_multiply(&partition.g_da, &g_aa_inv_ba);
            let mut b_eff = b_d;
            for (target, correction) in b_eff
                .iter_mut()
                .zip(self.column_matrix_to_vector(&gda_ginv_ba))
            {
                *target -= correction;
            }

            let l_a_ginv_gad = self.row_vector_times_matrix(&l_a, &g_aa_inv_g_ad);
            let mut c_eff = l_d;
            for (target, correction) in c_eff.iter_mut().zip(l_a_ginv_gad) {
                *target -= correction;
            }

            let d_eff = self
                .row_vector_times_matrix(&l_a, &g_aa_inv_ba)
                .into_iter()
                .next()
                .unwrap_or(0.0);

            (b_eff, c_eff, d_eff)
        };

        let mut a = self.solve_matrix_columns_regularized(&partition.c_dd, &g_eff)?;
        for row in &mut a.data {
            for value in row {
                *value = -*value;
            }
        }

        let b = self.solve_matrix_columns_regularized(
            &partition.c_dd,
            &self.vector_to_column_matrix(&b_eff),
        )?;

        Some(StateSpaceModel {
            a,
            b: self.column_matrix_to_vector(&b),
            c: c_eff,
            d: d_eff,
        })
    }
}
