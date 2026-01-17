//! Waveform data interpolation utilities.
//!
//! Functions for interpolating Y values at specific X positions
//! and extracting cursor values from waveform data.

use super::measurements::{CursorValue, CursorValues};

/// Interpolate Y value at a specific X position using linear interpolation.
///
/// Returns None if the data is empty or invalid.
pub fn interpolate_y_at_x(x_data: &[f64], y_data: &[f64], target_x: f64) -> Option<f64> {
    if x_data.len() != y_data.len() || x_data.is_empty() {
        return None;
    }

    // Find the two points bracketing target_x
    let mut left_idx = 0;
    for (i, &x) in x_data.iter().enumerate() {
        if x <= target_x {
            left_idx = i;
        } else {
            break;
        }
    }

    // Handle edge cases
    if left_idx >= x_data.len() - 1 {
        return Some(y_data[y_data.len() - 1]);
    }

    let right_idx = left_idx + 1;
    let x0 = x_data[left_idx];
    let x1 = x_data[right_idx];
    let y0 = y_data[left_idx];
    let y1 = y_data[right_idx];

    // Linear interpolation
    if (x1 - x0).abs() < 1e-15 {
        return Some(y0);
    }

    let t = (target_x - x0) / (x1 - x0);
    Some(y0 + t * (y1 - y0))
}

/// Interpolate Y value at a given X position (alternative implementation).
///
/// Returns None if data is invalid or target_x is outside the data range.
#[allow(dead_code)]
pub fn interpolate_y(x: &[f64], y: &[f64], target_x: f64) -> Option<f64> {
    if x.len() < 2 || y.len() < 2 || x.len() != y.len() {
        return None;
    }

    // Find the two points surrounding target_x
    for i in 0..x.len() - 1 {
        if x[i] <= target_x && target_x <= x[i + 1] {
            // Linear interpolation
            let t = (target_x - x[i]) / (x[i + 1] - x[i]);
            return Some(y[i] + t * (y[i + 1] - y[i]));
        }
    }
    None
}

/// Get cursor values for all visible traces at a given X position.
pub fn get_cursor_values(x_pos: f64, waveforms: &[crate::state::WaveformData]) -> CursorValues {
    let mut values = CursorValues {
        x: x_pos,
        traces: Vec::new(),
    };

    for wf in waveforms.iter().filter(|w| w.visible) {
        if let Some(y_val) = interpolate_y_at_x(&wf.x, &wf.y, x_pos) {
            values.traces.push(CursorValue {
                name: wf.name.clone(),
                color: wf.color.clone(),
                value: y_val,
            });
        }
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_y_at_x() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let y = vec![0.0, 1.0, 2.0, 3.0];

        // Exact point
        assert!((interpolate_y_at_x(&x, &y, 1.0).unwrap() - 1.0).abs() < 1e-10);

        // Interpolated point
        assert!((interpolate_y_at_x(&x, &y, 1.5).unwrap() - 1.5).abs() < 1e-10);

        // Edge case: at end
        assert!((interpolate_y_at_x(&x, &y, 3.0).unwrap() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolate_y() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 2.0, 4.0];

        assert!((interpolate_y(&x, &y, 0.5).unwrap() - 1.0).abs() < 1e-10);
        assert!((interpolate_y(&x, &y, 1.5).unwrap() - 3.0).abs() < 1e-10);
    }
}
