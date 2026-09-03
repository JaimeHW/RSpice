use super::*;

impl PoleZeroAnalyzer {
    /// Analyze an already reduced continuous-time SISO state-space model.
    ///
    /// Engine-level sparse descriptor reduction uses this entry point after
    /// eliminating algebraic MNA variables with sparse LU. Dense eigen work is
    /// then proportional only to the number of dynamic states, while root
    /// filtering, canonical ordering, gain, and zero extraction remain shared
    /// with the ordinary descriptor analyzer. Dense eigensolver calls are
    /// coarse cancellation regions; the signal is checked immediately before
    /// and after each one.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn analyze_state_space_with_abort(
        a: Matrix,
        b: Vec<Value>,
        c: Vec<Value>,
        d: Value,
        config: &PoleZeroConfig,
        input_label: &str,
        output_label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<PoleZeroResult, PoleZeroAnalysisError> {
        ensure_pole_zero_not_aborted(abort)?;
        let n = a.dims().0;
        if n == 0
            || a.dims().1 != n
            || a.data.iter().any(|row| row.len() != n)
            || a.data.iter().flatten().any(|value| !value.is_finite())
            || b.len() != n
            || b.iter().any(|value| !value.is_finite())
            || c.len() != n
            || c.iter().any(|value| !value.is_finite())
            || !d.is_finite()
        {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "reduced state-space matrices and vectors must be finite and dimensionally consistent"
                    .to_string(),
            ));
        }
        let helper = Self::new(Matrix::identity(n), Matrix::identity(n));
        let model = StateSpaceModel { a, b, c, d };
        let mut result = PoleZeroResult::new(input_label, output_label);
        if config.compute_poles {
            ensure_pole_zero_not_aborted(abort)?;
            let mut spectrum = helper.eigenvalues_from_matrix(&model.a)?;
            ensure_pole_zero_not_aborted(abort)?;
            helper.ensure_roots_within_frequency_limit(&spectrum.finite, config, "pole")?;
            spectrum
                .finite
                .sort_by(|left, right| left.norm().total_cmp(&right.norm()));
            result.set_poles(spectrum);
        }
        if config.compute_zeros {
            ensure_pole_zero_not_aborted(abort)?;
            let spectrum = helper.zeros_from_state_space(&model, config)?;
            ensure_pole_zero_not_aborted(abort)?;
            result.set_zeros(spectrum);
        }
        ensure_pole_zero_not_aborted(abort)?;
        if let Some(a_inv_b) = helper.solve_linear(&model.a, &model.b) {
            let correction = model
                .c
                .iter()
                .zip(a_inv_b)
                .map(|(weight, state)| weight * state)
                .sum::<Value>();
            let gain = model.d - correction;
            if gain.is_finite() {
                result.dc_gain = Some(gain);
            }
        }
        result.hf_gain = model.d.is_finite().then_some(model.d);
        result.sort_poles_by_magnitude();
        result.sort_zeros_by_magnitude();
        ensure_pole_zero_not_aborted(abort)?;
        Ok(result)
    }

    /// Compute DC gain H(0)
    pub fn dc_gain(&self, input_node: usize, output_node: usize) -> Option<Value> {
        // At DC (s=0), Y = G
        // Solve G·V = I where I is unit current at input
        let n = self.num_nodes;
        let (rows, cols) = self.g_matrix.dims();
        if n == 0
            || rows != cols
            || rows != n
            || self.g_matrix.data.iter().any(|row| row.len() != n)
            || self
                .g_matrix
                .data
                .iter()
                .flatten()
                .any(|value| !value.is_finite())
            || input_node >= n
            || output_node >= n
        {
            return None;
        }

        // Create excitation vector
        let mut b = vec![0.0; n];
        b[input_node] = 1.0;

        // Solve G·x = b using Gaussian elimination
        let x = self.solve_linear(&self.g_matrix, &b)?;

        x[output_node].is_finite().then_some(x[output_node])
    }

    pub(in crate::analysis::pole_zero) fn dc_gain_from_config(
        &self,
        config: &PoleZeroConfig,
    ) -> Option<Value> {
        let (input_vec, output_vec) = self.build_port_vectors(config)?;
        let x = self.solve_linear(&self.g_matrix, &input_vec)?;
        let vout = output_vec
            .iter()
            .zip(x.iter())
            .map(|(l, v)| l * v)
            .sum::<Value>();

        if config.input_is_current {
            return vout.is_finite().then_some(vout);
        }

        let vin = input_vec
            .iter()
            .zip(x.iter())
            .map(|(m, v)| m * v)
            .sum::<Value>();
        if !vout.is_finite() || !vin.is_finite() || vin == 0.0 {
            return None;
        }

        let gain = vout / vin;
        gain.is_finite().then_some(gain)
    }

    /// Solve linear system using Gaussian elimination
    pub(in crate::analysis::pole_zero) fn solve_linear(
        &self,
        a: &Matrix,
        b: &[Value],
    ) -> Option<Vec<Value>> {
        let n = a.dims().0;
        let pivot_tolerance = self.relative_matrix_tolerance(a, 1e-12);

        // Augmented matrix
        let mut aug: Vec<Vec<Value>> = (0..n)
            .map(|i| {
                let mut row = a.data[i].clone();
                row.push(b[i]);
                row
            })
            .collect();

        // Forward elimination
        for k in 0..n {
            // Partial pivoting
            let mut max_row = k;
            let mut max_val = aug[k][k].abs();
            for (i, entries) in aug.iter().enumerate().take(n).skip(k + 1) {
                if entries[k].abs() > max_val {
                    max_val = entries[k].abs();
                    max_row = i;
                }
            }

            if max_val <= pivot_tolerance {
                return None;
            }

            if max_row != k {
                aug.swap(k, max_row);
            }

            let pivot = aug[k][k];
            for i in (k + 1)..n {
                // `i > k`, so the pivot row stays in `above`.
                let (above, below) = aug.split_at_mut(i);
                let pivot_row = &above[k];
                let target_row = &mut below[0];
                let factor = target_row[k] / pivot;
                target_row[k] = 0.0;
                for (target, &value) in target_row[(k + 1)..=n]
                    .iter_mut()
                    .zip(&pivot_row[(k + 1)..=n])
                {
                    *target -= factor * value;
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Some(x)
    }

    /// Run complete pole-zero analysis
    pub fn analyze(
        &self,
        config: &PoleZeroConfig,
    ) -> Result<PoleZeroResult, PoleZeroAnalysisError> {
        self.analyze_with_abort(config, &NoAbort)
    }

    /// Run complete pole-zero analysis with cooperative cancellation around
    /// each dense factorization/eigensolver phase and every major projection.
    pub fn analyze_with_abort(
        &self,
        config: &PoleZeroConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PoleZeroResult, PoleZeroAnalysisError> {
        ensure_pole_zero_not_aborted(abort)?;
        let (g_rows, g_cols) = self.g_matrix.dims();
        let (c_rows, c_cols) = self.c_matrix.dims();
        if g_rows == 0
            || g_rows != g_cols
            || c_rows != c_cols
            || g_rows != c_rows
            || self.g_matrix.data.iter().any(|row| row.len() != g_cols)
            || self.c_matrix.data.iter().any(|row| row.len() != c_cols)
            || self
                .g_matrix
                .data
                .iter()
                .flatten()
                .any(|value| !value.is_finite())
            || self
                .c_matrix
                .data
                .iter()
                .flatten()
                .any(|value| !value.is_finite())
        {
            return Err(PoleZeroAnalysisError::InvalidSystem(
                "G and C must be finite square matrices with equal dimensions".to_string(),
            ));
        }
        let mut result = PoleZeroResult::new(
            &format!("node{}", config.input_pos),
            &format!("node{}", config.output_pos),
        );
        ensure_pole_zero_not_aborted(abort)?;

        if !config.input_is_current {
            let (_, output_vec) = self.build_port_vectors(config).ok_or(
                PoleZeroAnalysisError::TransferExtraction("input or output port is invalid"),
            )?;
            let (voltage_analyzer, drive_vec, output_ext) = self
                .build_voltage_input_transfer_system(config, &output_vec)
                .ok_or(PoleZeroAnalysisError::TransferExtraction(
                    "voltage input source could not be constructed",
                ))?;
            if config.compute_poles {
                ensure_pole_zero_not_aborted(abort)?;
                let spectrum = voltage_analyzer.find_poles(config)?;
                ensure_pole_zero_not_aborted(abort)?;
                result.set_poles(spectrum);
            }

            if config.compute_zeros {
                ensure_pole_zero_not_aborted(abort)?;
                if self.is_direct_voltage_port_measurement(config) {
                    result.set_zeros(ComputedSpectrum::exact(Vec::new(), 0, 0)?);
                } else if let Some(state_space) =
                    voltage_analyzer.build_state_space(&drive_vec, &output_ext)
                {
                    let spectrum = voltage_analyzer.zeros_from_state_space(&state_space, config)?;
                    result.set_zeros(spectrum);
                } else {
                    let spectrum =
                        voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config)?;
                    let spectrum = voltage_analyzer.finalize_zero_roots(spectrum, config)?;
                    result.set_zeros(spectrum);
                }
                ensure_pole_zero_not_aborted(abort)?;
            }

            ensure_pole_zero_not_aborted(abort)?;
            if let Some(gain) = self.dc_gain_from_config(config) {
                result.dc_gain = Some(gain);
            }

            result.sort_poles_by_magnitude();
            result.sort_zeros_by_magnitude();
            ensure_pole_zero_not_aborted(abort)?;
            return Ok(result);
        }

        // Find poles
        if config.compute_poles {
            ensure_pole_zero_not_aborted(abort)?;
            let spectrum = self.find_poles(config)?;
            ensure_pole_zero_not_aborted(abort)?;
            result.set_poles(spectrum);
        }

        // Find zeros
        if config.compute_zeros {
            ensure_pole_zero_not_aborted(abort)?;
            let spectrum = self.find_zeros(config)?;
            ensure_pole_zero_not_aborted(abort)?;
            result.set_zeros(spectrum);
        }

        // Compute DC gain
        ensure_pole_zero_not_aborted(abort)?;
        if let Some(gain) = self.dc_gain_from_config(config) {
            result.dc_gain = Some(gain);
        }

        result.sort_poles_by_magnitude();
        result.sort_zeros_by_magnitude();

        ensure_pole_zero_not_aborted(abort)?;
        Ok(result)
    }
}
