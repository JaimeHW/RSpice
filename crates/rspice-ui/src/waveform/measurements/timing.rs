use super::basic::{calculate_max, calculate_mean, calculate_min};

// Timing Measurements
// =============================================================================

/// Threshold crossing detection result
#[derive(Debug, Clone)]
pub struct Crossing {
    /// Index of the first point before crossing
    pub index: usize,
    /// Interpolated X position of crossing
    pub x: f64,
    /// Whether this is a rising or falling edge
    pub rising: bool,
}

/// Find all threshold crossings in the waveform
pub fn find_crossings(x_data: &[f64], y_data: &[f64], threshold: f64) -> Vec<Crossing> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return Vec::new();
    }

    let n = x_data.len().min(y_data.len());
    let mut crossings = Vec::new();

    for i in 0..n - 1 {
        let y0 = y_data[i];
        let y1 = y_data[i + 1];

        if !y0.is_finite() || !y1.is_finite() {
            continue;
        }

        // Check for rising crossing
        if y0 < threshold && y1 >= threshold {
            let x = interpolate_crossing(x_data[i], x_data[i + 1], y0, y1, threshold);
            crossings.push(Crossing {
                index: i,
                x,
                rising: true,
            });
        }
        // Check for falling crossing
        else if y0 >= threshold && y1 < threshold {
            let x = interpolate_crossing(x_data[i], x_data[i + 1], y0, y1, threshold);
            crossings.push(Crossing {
                index: i,
                x,
                rising: false,
            });
        }
    }

    crossings
}

/// Interpolate X position of threshold crossing
fn interpolate_crossing(x0: f64, x1: f64, y0: f64, y1: f64, threshold: f64) -> f64 {
    let dy = y1 - y0;
    if dy.abs() < 1e-15 {
        return (x0 + x1) / 2.0;
    }
    let t = (threshold - y0) / dy;
    x0 + t * (x1 - x0)
}

/// Calculate rise time (10% to 90% of swing)
pub fn calculate_rise_time(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let min = calculate_min(y_data)?;
    let max = calculate_max(y_data)?;
    let range = max - min;

    if range < 1e-15 {
        return None;
    }

    let low_threshold = min + 0.1 * range;
    let high_threshold = min + 0.9 * range;

    // Find first rising edge through both thresholds
    let low_crossings = find_crossings(x_data, y_data, low_threshold);
    let high_crossings = find_crossings(x_data, y_data, high_threshold);

    // Find first rising low crossing
    let first_low_rise = low_crossings.iter().find(|c| c.rising)?;

    // Find first rising high crossing after the low crossing
    let first_high_rise = high_crossings
        .iter()
        .find(|c| c.rising && c.x > first_low_rise.x)?;

    Some(first_high_rise.x - first_low_rise.x)
}

/// Calculate fall time (90% to 10% of swing)
pub fn calculate_fall_time(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let min = calculate_min(y_data)?;
    let max = calculate_max(y_data)?;
    let range = max - min;

    if range < 1e-15 {
        return None;
    }

    let high_threshold = min + 0.9 * range;
    let low_threshold = min + 0.1 * range;

    let high_crossings = find_crossings(x_data, y_data, high_threshold);
    let low_crossings = find_crossings(x_data, y_data, low_threshold);

    // Find first falling high crossing
    let first_high_fall = high_crossings.iter().find(|c| !c.rising)?;

    // Find first falling low crossing after the high crossing
    let first_low_fall = low_crossings
        .iter()
        .find(|c| !c.rising && c.x > first_high_fall.x)?;

    Some(first_low_fall.x - first_high_fall.x)
}

/// Calculate period from rising edge to rising edge
pub fn calculate_period(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    let mean = calculate_mean(y_data)?;
    let crossings = find_crossings(x_data, y_data, mean);

    // Find two consecutive rising crossings
    let rising: Vec<&Crossing> = crossings.iter().filter(|c| c.rising).collect();

    if rising.len() < 2 {
        return None;
    }

    // Average of all periods
    let mut periods = Vec::new();
    for i in 1..rising.len() {
        periods.push(rising[i].x - rising[i - 1].x);
    }

    if periods.is_empty() {
        return None;
    }

    Some(periods.iter().sum::<f64>() / periods.len() as f64)
}

/// Calculate frequency (1/period)
pub fn calculate_frequency(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    calculate_period(x_data, y_data).map(|p| if p > 0.0 { 1.0 / p } else { 0.0 })
}

/// Calculate duty cycle (percentage of time signal is above mean)
pub fn calculate_duty_cycle(x_data: &[f64], y_data: &[f64]) -> Option<f64> {
    if x_data.len() < 2 || y_data.len() < 2 {
        return None;
    }

    let mean = calculate_mean(y_data)?;
    let n = x_data.len().min(y_data.len());

    let mut high_time = 0.0;
    let mut total_time = 0.0;

    for i in 0..n - 1 {
        let dt = x_data[i + 1] - x_data[i];
        if dt > 0.0 && y_data[i].is_finite() {
            total_time += dt;
            if y_data[i] >= mean {
                high_time += dt;
            }
        }
    }

    if total_time > 0.0 {
        Some(100.0 * high_time / total_time)
    } else {
        None
    }
}

// =============================================================================
