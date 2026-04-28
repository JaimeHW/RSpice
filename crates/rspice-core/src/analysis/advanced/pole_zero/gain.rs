use super::*;

impl PoleZeroAnalyzer {
    /// Compute DC gain H(0)
    pub fn dc_gain(&self, input_node: usize, output_node: usize) -> Option<Value> {
        // At DC (s=0), Y = G
        // Solve GÂ·V = I where I is unit current at input
        let n = self.num_nodes;
        if input_node >= n || output_node >= n {
            return None;
        }

        // Create excitation vector
        let mut b = vec![0.0; n];
        b[input_node] = 1.0;

        // Solve GÂ·x = b using Gaussian elimination
        let x = self.solve_linear(&self.g_matrix, &b)?;

        Some(x[output_node])
    }

    pub(in crate::analysis::advanced::pole_zero) fn dc_gain_from_config(
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
            return Some(vout);
        }

        let vin = input_vec
            .iter()
            .zip(x.iter())
            .map(|(m, v)| m * v)
            .sum::<Value>();
        if vin.abs() < 1e-15 {
            return None;
        }

        Some(vout / vin)
    }

    /// Solve linear system using Gaussian elimination
    pub(in crate::analysis::advanced::pole_zero) fn solve_linear(
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
            for i in (k + 1)..n {
                if aug[i][k].abs() > max_val {
                    max_val = aug[i][k].abs();
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
                let factor = aug[i][k] / pivot;
                aug[i][k] = 0.0;
                for j in (k + 1)..=n {
                    aug[i][j] -= factor * aug[k][j];
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
    pub fn analyze(&self, config: &PoleZeroConfig) -> PoleZeroResult {
        let mut result = PoleZeroResult::new(
            &format!("node{}", config.input_pos),
            &format!("node{}", config.output_pos),
        );

        if !config.input_is_current
            && let Some((_, output_vec)) = self.build_port_vectors(config)
            && let Some((voltage_analyzer, drive_vec, output_ext)) =
                self.build_voltage_input_transfer_system(config, &output_vec)
        {
            if config.compute_poles {
                result.poles = voltage_analyzer.find_poles(config);
            }

            if config.compute_zeros {
                if self.is_direct_voltage_port_measurement(config) {
                    result.zeros.clear();
                } else if let Some(state_space) =
                    voltage_analyzer.build_state_space(&drive_vec, &output_ext)
                {
                    let poles = if config.compute_poles {
                        result.poles.clone()
                    } else {
                        voltage_analyzer
                            .eigenvalues_from_matrix(&state_space.a)
                            .unwrap_or_else(|| voltage_analyzer.find_poles(config))
                    };
                    result.zeros =
                        voltage_analyzer.zeros_from_state_space(&state_space, &poles, config);
                } else {
                    let poles = if config.compute_poles {
                        result.poles.clone()
                    } else {
                        voltage_analyzer.find_poles(config)
                    };
                    let zeros =
                        voltage_analyzer.numerator_roots_raw(&drive_vec, &output_ext, config);
                    result.zeros = voltage_analyzer.finalize_zero_roots(zeros, &poles, config);
                }
            }

            if let Some(gain) = self.dc_gain_from_config(config) {
                result.dc_gain = gain;
            }

            result.sort_poles_by_magnitude();
            result.sort_zeros_by_magnitude();
            return result;
        }

        // Find poles
        if config.compute_poles {
            result.poles = self.find_poles(config);
        }

        // Find zeros
        if config.compute_zeros {
            result.zeros = self.find_zeros(config);
        }

        // Compute DC gain
        if let Some(gain) = self.dc_gain_from_config(config) {
            result.dc_gain = gain;
        }

        result.sort_poles_by_magnitude();
        result.sort_zeros_by_magnitude();

        result
    }
}
