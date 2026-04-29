// Integral/Area Measurements
// =============================================================================

/// Calculate integral (area under curve) using trapezoidal rule
pub fn calculate_integral(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return None;
    }

    let n = x_data.len().min(y_data.len());
    let mut integral = 0.0;

    for i in 0..n - 1 {
        let y0 = y_data[i];
        let y1 = y_data[i + 1];
        let dx = x_data[i + 1] - x_data[i];

        if y0.is_finite() && y1.is_finite() && dx.is_finite() {
            // Trapezoidal rule
            integral += 0.5 * (y0 + y1) * dx;
        }
    }

    Some(integral)
}

// =============================================================================
