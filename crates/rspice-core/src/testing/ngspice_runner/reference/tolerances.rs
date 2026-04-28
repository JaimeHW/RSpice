use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn interpolate_series(
        x: &[f64],
        y: &[f64],
        x_query: f64,
    ) -> Option<f64> {
        if x.len() != y.len() || x.is_empty() {
            return None;
        }
        if x.len() == 1 {
            return Some(y[0]);
        }

        let ascending = x[0] <= x[x.len() - 1];
        let axis_scale = x[0].abs().max(x[x.len() - 1].abs()).max(x_query.abs());
        let range_eps = (8e-15 * axis_scale).max(1e-18);
        let in_range = if ascending {
            x_query >= x[0] - range_eps && x_query <= x[x.len() - 1] + range_eps
        } else {
            x_query <= x[0] + range_eps && x_query >= x[x.len() - 1] - range_eps
        };
        if !in_range {
            return None;
        }

        let mut lo = 0usize;
        let mut hi = x.len() - 1;
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let go_right = if ascending {
                x[mid] < x_query
            } else {
                x[mid] > x_query
            };
            if go_right {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let x0 = x[lo];
        let x1 = x[hi];
        let y0 = y[lo];
        let y1 = y[hi];
        let local_scale = x0.abs().max(x1.abs()).max(x_query.abs());
        let snap_eps = (8e-15 * local_scale).max(1e-18);
        if (x_query - x0).abs() <= snap_eps {
            return Some(y0);
        }
        if (x_query - x1).abs() <= snap_eps {
            return Some(y1);
        }
        if (x1 - x0).abs() <= f64::EPSILON {
            return Some(y0);
        }
        let t = (x_query - x0) / (x1 - x0);
        Some(y0 + t * (y1 - y0))
    }

    pub(in crate::testing::ngspice_runner) fn series_absolute_tolerance_floor(
        &self,
        var: &str,
        expected_series: &ReferenceSeries,
        actual_series: &[f64],
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        let normalized = Self::normalize_variable_name(var);
        let expected_scale = expected_series
            .y
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let actual_scale = actual_series
            .iter()
            .copied()
            .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
        let series_scale = expected_scale.max(actual_scale);
        if normalized.starts_with("ph(") {
            // For radian phase probes, compare with an angle-domain floor derived
            // from the trace scale. This avoids over-penalizing tiny imaginary
            // residue on near-zero phase currents while still capping tolerance.
            floor = floor.max((series_scale * 0.7).clamp(2e-3, 5e-2));
        } else if normalized.starts_with("db(") || normalized.starts_with("vdb(") {
            // dB-domain probes are already logarithmic. When the trace crosses
            // 0 dB, relative error becomes a poor metric even for sub-0.1%
            // linear-magnitude differences, so keep a tight absolute floor in
            // the logarithmic domain itself.
            floor = floor.max((series_scale * 2e-4).clamp(5e-3, 2e-1));
        } else if normalized.starts_with("vp(") || normalized.starts_with("ip(") {
            floor = floor.max((series_scale * 0.7).clamp(0.12, 3.0));
        }
        if Self::reference_expr_contains_voltage_probe(var) {
            // Use a small waveform-scale floor for direct voltage probes so
            // rail-scale switching traces are compared by meaningful absolute
            // error when interpolation lands near zero crossings.
            floor = floor.max(series_scale * 1e-4);
        } else if Self::reference_expr_contains_current_probe(var) {
            // Keep direct current probes strict by default, but when the trace
            // genuinely spans both polarities, use a modest full-scale floor so
            // sweep points around the sign-change boundary are not dominated by
            // relative error on effectively zero current.
            let spans_zero = Self::series_spans_zero(&expected_series.y)
                || Self::series_spans_zero(actual_series);
            let current_floor_scale = if spans_zero { 1e-4 } else { 2e-6 };
            floor = floor.max(series_scale * current_floor_scale);
        }
        floor
    }

    pub(in crate::testing::ngspice_runner) fn series_spans_zero(values: &[f64]) -> bool {
        let has_positive = values.iter().any(|&value| value > 0.0);
        let has_negative = values.iter().any(|&value| value < 0.0);
        has_positive && has_negative
    }

    pub(in crate::testing::ngspice_runner) fn dc_op_absolute_tolerance_floor(
        &self,
        probe: &str,
        reference: &OpReference,
        result: &crate::SimulationResult,
    ) -> f64 {
        let mut floor = self.config.absolute_tolerance;
        if Self::parse_voltage_probe(probe).is_some() {
            let expected_scale = reference
                .node_voltages
                .iter()
                .filter_map(|(name, value)| Self::parse_voltage_probe(name).map(|_| value.abs()))
                .fold(0.0_f64, f64::max);
            let actual_scale = result
                .node_voltages
                .iter()
                .copied()
                .fold(0.0_f64, |max_v, value| max_v.max(value.abs()));
            let circuit_scale = expected_scale.max(actual_scale);
            // Use the operating-point voltage scale for direct probes so
            // sub-microvolt residue around a nominally-zero node does not
            // fail an otherwise correct deck.
            floor = floor.max(circuit_scale * 1e-4);
        }
        floor
    }

    pub(in crate::testing::ngspice_runner) fn compare_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
    ) -> Option<f64> {
        let abs_diff = (expected - actual).abs();

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;

        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    pub(in crate::testing::ngspice_runner) fn wrap_phase_delta(delta: f64, period: f64) -> f64 {
        let half_period = 0.5 * period;
        (delta + half_period).rem_euclid(period) - half_period
    }

    pub(in crate::testing::ngspice_runner) fn compare_phase_values_with_abs_tol(
        &self,
        expected: f64,
        actual: f64,
        absolute_tolerance: f64,
        allow_orientation_flip: bool,
        degrees: bool,
    ) -> Option<f64> {
        let period = if degrees {
            360.0
        } else {
            2.0 * std::f64::consts::PI
        };
        let half_turn = 0.5 * period;
        let mut candidates = vec![actual, -actual];
        if allow_orientation_flip {
            candidates.extend_from_slice(&[
                actual + half_turn,
                actual - half_turn,
                -actual + half_turn,
                -actual - half_turn,
            ]);
        }
        let abs_diff = candidates
            .into_iter()
            .map(|candidate| Self::wrap_phase_delta(candidate - expected, period).abs())
            .fold(f64::INFINITY, f64::min);

        if abs_diff < absolute_tolerance {
            return None;
        }

        let rel_scale = expected.abs().max(actual.abs()).max(absolute_tolerance);
        let rel_error = abs_diff / rel_scale;
        if rel_error > self.config.relative_tolerance {
            Some(rel_error)
        } else {
            None
        }
    }

    pub(in crate::testing::ngspice_runner) fn compare_values(
        &self,
        expected: f64,
        actual: f64,
    ) -> Option<f64> {
        self.compare_values_with_abs_tol(expected, actual, self.config.absolute_tolerance)
    }
}
