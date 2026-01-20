//! Axis calculation utilities.
//!
//! Functions for computing grid step sizes and generating axis labels
//! with appropriate SI prefixes.

/// Calculate a nice step size for grid lines.
///
/// Targets approximately 6 divisions and rounds to "nice" values
/// (1, 2, 5 multiples of powers of 10).
pub fn calculate_nice_grid_step(range: f64) -> f64 {
    calculate_nice_grid_step_adaptive(range, None)
}

/// Calculate a nice step size for grid lines with adaptive division count.
///
/// When `container_size_px` is provided, the number of divisions scales with
/// container size to maintain a consistent visual density (target ~80px between
/// gridlines). Minimum 4 divisions, maximum ~15 divisions.
///
/// Uses "nice" step values (1, 2, 5 multiples of powers of 10) for readability.
pub fn calculate_nice_grid_step_adaptive(range: f64, container_size_px: Option<f64>) -> f64 {
    // Calculate target divisions based on container size
    // Aim for ~80px between gridlines, with min 4 and max ~15 divisions
    let target_divisions = if let Some(size) = container_size_px {
        let raw_divisions = size / 80.0;
        raw_divisions.max(4.0).min(15.0)
    } else {
        6.0 // Default when no container size provided
    };

    let raw_step = range / target_divisions;
    if raw_step <= 0.0 || !raw_step.is_finite() {
        return 1.0;
    }

    let magnitude = 10f64.powf(raw_step.log10().floor());
    let normalized = raw_step / magnitude;

    let nice = if normalized < 1.5 {
        1.0
    } else if normalized < 3.5 {
        2.0
    } else if normalized < 7.5 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

/// Calculate step size with a fixed number of divisions.
///
/// This is the professional simulator approach (LTspice, Cadence Spectre):
/// Always use a fixed number of divisions regardless of container size.
/// Uses "nice" step values (1, 2, 5 multiples of powers of 10) for readability.
pub fn calculate_nice_step_fixed_divisions(range: f64, target_divisions: usize) -> f64 {
    let raw_step = range / target_divisions.max(1) as f64;
    if raw_step <= 0.0 || !raw_step.is_finite() {
        return 1.0;
    }

    let magnitude = 10f64.powf(raw_step.log10().floor());
    let normalized = raw_step / magnitude;

    let nice = if normalized < 1.5 {
        1.0
    } else if normalized < 3.5 {
        2.0
    } else if normalized < 7.5 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

/// Generate axis labels at evenly spaced positions (legacy, kept for compatibility).
#[allow(dead_code)]
pub fn generate_axis_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let step = (max - min) / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = min + i as f64 * step;
            format!("{:.2}V", val)
        })
        .collect()
}

/// Generate time labels with appropriate SI prefix (legacy, kept for compatibility).
#[allow(dead_code)]
pub fn generate_time_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let range = max - min;
    let (scale, suffix) = if range < 1e-6 {
        (1e9, "ns")
    } else if range < 1e-3 {
        (1e6, "µs")
    } else if range < 1.0 {
        (1e3, "ms")
    } else {
        (1.0, "s")
    };

    let step = range / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = (min + i as f64 * step) * scale;
            format!("{:.1}{}", val, suffix)
        })
        .collect()
}

/// Get the SI prefix scale and suffix for a time range.
pub fn time_scale_for_range(range: f64) -> (f64, &'static str) {
    if range < 1e-6 {
        (1e9, "ns")
    } else if range < 1e-3 {
        (1e6, "µs")
    } else if range < 1.0 {
        (1e3, "ms")
    } else {
        (1.0, "s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_grid_step() {
        // Should give nice round numbers
        let step = calculate_nice_grid_step(10.0);
        assert!(step == 1.0 || step == 2.0 || step == 5.0);

        let step = calculate_nice_grid_step(100.0);
        assert!(step == 10.0 || step == 20.0 || step == 50.0);
    }

    #[test]
    fn test_time_scale() {
        assert_eq!(time_scale_for_range(0.5).1, "ms");
        assert_eq!(time_scale_for_range(0.0001).1, "µs");
        assert_eq!(time_scale_for_range(2.0).1, "s");
    }
}
