//! Shared parsing for analysis setup fields.
//!
//! Expanding a temperature list into its points, and reading the counts and
//! step values a user typed, with the field name carried into the error so a
//! rejection says which input was wrong.

use super::*;

impl SimulationController {
    pub(super) fn expand_temperature_points(
        start: f64,
        stop: f64,
        step: f64,
    ) -> Result<Vec<f64>, String> {
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
            return Err(
                "temperature sweep range requires finite start/stop/step values".to_string(),
            );
        }
        if step == 0.0 {
            return Err("temperature sweep step cannot be zero".to_string());
        }
        if (start < stop && step < 0.0) || (start > stop && step > 0.0) {
            return Err("temperature sweep step direction must match start/stop range".to_string());
        }

        if start == stop {
            return Ok(vec![start]);
        }

        let point_limit = rspice_core::ResourceLimits::default().max_batch_runs;
        let step_magnitude = step.abs();
        let raw_intervals = if (start < 0.0 && stop > 0.0) || (start > 0.0 && stop < 0.0) {
            // Subtracting opposite-sign finite endpoints can overflow even
            // when division by the step would produce a small, valid count.
            start.abs() / step_magnitude + stop.abs() / step_magnitude
        } else {
            (stop - start).abs() / step_magnitude
        };
        let estimated_points = if !raw_intervals.is_finite() || raw_intervals >= usize::MAX as f64 {
            usize::MAX
        } else {
            (raw_intervals.floor() as usize).saturating_add(1)
        };
        if estimated_points > point_limit {
            return Err(format!(
                "temperature sweep requests at least {estimated_points} runs, exceeding the configured limit of {point_limit}"
            ));
        }

        let mut values = Vec::new();
        values
            .try_reserve_exact(estimated_points.saturating_add(1).min(point_limit))
            .map_err(|_| {
                format!(
                    "temperature sweep could not allocate storage for {estimated_points} points"
                )
            })?;
        let mut current = start;
        let tolerance = (step.abs() * 1e-12).max((start.abs().max(stop.abs())) * 1e-12);
        let ascending = step > 0.0;
        let endpoint = if ascending {
            match stop + tolerance {
                value if value.is_finite() => value,
                _ => stop,
            }
        } else {
            match stop - tolerance {
                value if value.is_finite() => value,
                _ => stop,
            }
        };

        while if ascending {
            current <= endpoint
        } else {
            current >= endpoint
        } {
            if values.len() == point_limit {
                return Err(format!(
                    "temperature sweep requests more than {point_limit} runs, exceeding the configured limit of {point_limit}"
                ));
            }
            values.push(current);
            if (ascending && current >= stop) || (!ascending && current <= stop) {
                break;
            }

            let next = current + step;
            if !next.is_finite() {
                return Err(format!(
                    "temperature sweep step {step} produces a non-finite value after {current} before reaching {stop}"
                ));
            }
            if (ascending && next <= current) || (!ascending && next >= current) {
                return Err(format!(
                    "temperature sweep step {step} is too small to advance from {current}"
                ));
            }
            current = next;
        }

        if values.is_empty() {
            return Err("temperature sweep produced no points".to_string());
        }

        Ok(values)
    }

    pub(super) fn parse_positive_points(raw: &str, field_name: &str) -> Result<usize, String> {
        let points = raw
            .trim()
            .parse::<usize>()
            .map_err(|_| format!("{} must be a positive integer", field_name))?;
        if points == 0 {
            return Err(format!("{} must be greater than zero", field_name));
        }
        Ok(points)
    }

    pub(super) fn parse_optional_spice_value(raw: &str) -> Result<Option<f64>, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("auto") {
            return Ok(None);
        }
        parse_spice_value_checked(trimmed).map(Some)
    }

    pub(super) fn map_frequency_sweep(idx: usize) -> FrequencySweep {
        match idx {
            1 => FrequencySweep::Octave,
            2 => FrequencySweep::Linear,
            _ => FrequencySweep::Decade,
        }
    }

    pub(super) fn map_ac_sweep(sweep: FrequencySweep) -> AcSweepType {
        match sweep {
            FrequencySweep::Decade => AcSweepType::Decade,
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimulationController;

    #[test]
    fn temperature_points_expand_in_both_directions() {
        assert_eq!(
            SimulationController::expand_temperature_points(-40.0, 125.0, 55.0).unwrap(),
            [-40.0, 15.0, 70.0, 125.0]
        );
        assert_eq!(
            SimulationController::expand_temperature_points(125.0, -40.0, -55.0).unwrap(),
            [125.0, 70.0, 15.0, -40.0]
        );
    }

    #[test]
    fn temperature_points_reject_non_finite_inputs() {
        for (start, stop, step) in [
            (f64::NAN, 1.0, 1.0),
            (0.0, f64::INFINITY, 1.0),
            (0.0, 1.0, f64::NEG_INFINITY),
        ] {
            assert!(SimulationController::expand_temperature_points(start, stop, step).is_err());
        }
    }

    #[test]
    fn temperature_points_reject_a_grid_above_the_batch_limit() {
        let limit = rspice_core::ResourceLimits::default().max_batch_runs;
        let error = SimulationController::expand_temperature_points(0.0, limit as f64, 1.0)
            .expect_err("one point beyond the run limit must be refused");
        assert!(error.contains(&limit.to_string()), "{error}");
    }

    #[test]
    fn temperature_points_reject_a_step_that_cannot_advance() {
        let start = 1.0e300_f64;
        let stop = f64::from_bits(start.to_bits() + 1);
        let step = (stop - start) / 4.0;
        let error = SimulationController::expand_temperature_points(start, stop, step)
            .expect_err("a sub-ULP step must not loop on one value");
        assert!(error.contains("too small to advance"), "{error}");
    }

    #[test]
    fn temperature_points_keep_extreme_results_finite() {
        let points =
            SimulationController::expand_temperature_points(0.0, f64::MAX, f64::MAX / 2.0).unwrap();
        assert_eq!(points, [0.0, f64::MAX / 2.0, f64::MAX]);
        assert!(points.iter().all(|point| point.is_finite()));

        let ascending =
            SimulationController::expand_temperature_points(-f64::MAX, f64::MAX, f64::MAX).unwrap();
        assert_eq!(ascending, [-f64::MAX, 0.0, f64::MAX]);
        let descending =
            SimulationController::expand_temperature_points(f64::MAX, -f64::MAX, -f64::MAX)
                .unwrap();
        assert_eq!(descending, [f64::MAX, 0.0, -f64::MAX]);
    }

    #[test]
    fn temperature_points_reject_overflow_before_the_endpoint() {
        let error =
            SimulationController::expand_temperature_points(f64::MAX / 2.0, f64::MAX, f64::MAX)
                .expect_err("overflow before the stop must not publish a truncated sweep");
        assert!(error.contains("non-finite value"), "{error}");
    }
}
