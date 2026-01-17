//! Waveform Measurement Calculations
//!
//! Provides automated measurements for waveform analysis including
//! rise/fall time, frequency, duty cycle, and statistical measures.

/// Measurement result with value and units
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
}

impl Measurement {
    pub fn new(name: &'static str, value: f64, unit: &'static str) -> Self {
        Self { name, value, unit }
    }

    /// Format value with SI prefix
    pub fn formatted_value(&self) -> String {
        let (scaled, prefix) = si_prefix(self.value.abs());
        if self.value < 0.0 {
            format!("-{:.3} {}{}", scaled, prefix, self.unit)
        } else {
            format!("{:.3} {}{}", scaled, prefix, self.unit)
        }
    }
}

/// Get SI prefix and scaled value
fn si_prefix(value: f64) -> (f64, &'static str) {
    if value == 0.0 {
        return (0.0, "");
    }
    let abs = value.abs();
    if abs >= 1e9 {
        (value / 1e9, "G")
    } else if abs >= 1e6 {
        (value / 1e6, "M")
    } else if abs >= 1e3 {
        (value / 1e3, "k")
    } else if abs >= 1.0 {
        (value, "")
    } else if abs >= 1e-3 {
        (value * 1e3, "m")
    } else if abs >= 1e-6 {
        (value * 1e6, "µ")
    } else if abs >= 1e-9 {
        (value * 1e9, "n")
    } else if abs >= 1e-12 {
        (value * 1e12, "p")
    } else {
        (value * 1e15, "f")
    }
}

/// Calculate rise time (10% to 90% of swing by default)
///
/// # Arguments
/// * `x` - Time values
/// * `y` - Signal values
/// * `low_pct` - Low threshold (0.1 = 10%)
/// * `high_pct` - High threshold (0.9 = 90%)
pub fn rise_time(x: &[f64], y: &[f64], low_pct: f64, high_pct: f64) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }

    let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let swing = y_max - y_min;

    if swing < 1e-12 {
        return None; // No signal swing
    }

    let low_thresh = y_min + swing * low_pct;
    let high_thresh = y_min + swing * high_pct;

    // Find first rising edge crossing low threshold
    let mut t_low = None;
    for i in 1..y.len() {
        if y[i - 1] <= low_thresh && y[i] > low_thresh {
            // Linear interpolation for exact crossing
            let frac = (low_thresh - y[i - 1]) / (y[i] - y[i - 1]);
            t_low = Some(x[i - 1] + frac * (x[i] - x[i - 1]));
            break;
        }
    }

    let t_low = t_low?;

    // Find crossing of high threshold after t_low
    for i in 1..y.len() {
        if x[i] > t_low && y[i - 1] <= high_thresh && y[i] > high_thresh {
            let frac = (high_thresh - y[i - 1]) / (y[i] - y[i - 1]);
            let t_high = x[i - 1] + frac * (x[i] - x[i - 1]);
            return Some(t_high - t_low);
        }
    }

    None
}

/// Calculate fall time (90% to 10% of swing by default)
pub fn fall_time(x: &[f64], y: &[f64], high_pct: f64, low_pct: f64) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }

    let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let swing = y_max - y_min;

    if swing < 1e-12 {
        return None;
    }

    let high_thresh = y_min + swing * high_pct;
    let low_thresh = y_min + swing * low_pct;

    // Find first falling edge crossing high threshold going down
    let mut t_high = None;
    for i in 1..y.len() {
        if y[i - 1] >= high_thresh && y[i] < high_thresh {
            let frac = (high_thresh - y[i]) / (y[i - 1] - y[i]);
            t_high = Some(x[i] - frac * (x[i] - x[i - 1]));
            break;
        }
    }

    let t_high = t_high?;

    // Find crossing of low threshold after t_high
    for i in 1..y.len() {
        if x[i] > t_high && y[i - 1] >= low_thresh && y[i] < low_thresh {
            let frac = (low_thresh - y[i]) / (y[i - 1] - y[i]);
            let t_low = x[i] - frac * (x[i] - x[i - 1]);
            return Some(t_low - t_high);
        }
    }

    None
}

/// Calculate frequency using zero-crossing detection
///
/// Finds the average period by detecting crossings of the mean value.
pub fn frequency(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() < 4 || x.len() != y.len() {
        return None;
    }

    let mean = y.iter().sum::<f64>() / y.len() as f64;

    // Find all rising zero crossings (through mean)
    let mut crossings = Vec::new();
    for i in 1..y.len() {
        if y[i - 1] <= mean && y[i] > mean {
            // Interpolate exact crossing time
            let frac = (mean - y[i - 1]) / (y[i] - y[i - 1]);
            crossings.push(x[i - 1] + frac * (x[i] - x[i - 1]));
        }
    }

    if crossings.len() < 2 {
        return None;
    }

    // Calculate average period from consecutive crossings
    let mut total_period = 0.0;
    for i in 1..crossings.len() {
        total_period += crossings[i] - crossings[i - 1];
    }
    let avg_period = total_period / (crossings.len() - 1) as f64;

    if avg_period > 1e-15 {
        Some(1.0 / avg_period)
    } else {
        None
    }
}

/// Calculate duty cycle (percentage of time above threshold)
///
/// # Arguments
/// * `x` - Time values
/// * `y` - Signal values
/// * `threshold` - If None, uses midpoint between min and max
pub fn duty_cycle(x: &[f64], y: &[f64], threshold: Option<f64>) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }

    let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let thresh = threshold.unwrap_or((y_min + y_max) / 2.0);

    let total_time = x.last()? - x.first()?;
    if total_time <= 0.0 {
        return None;
    }

    // Integrate time above threshold using trapezoidal
    let mut high_time = 0.0;
    for i in 1..x.len() {
        let dt = x[i] - x[i - 1];
        // Check if both points are above threshold
        let above_prev = if y[i - 1] > thresh { 1.0 } else { 0.0 };
        let above_curr = if y[i] > thresh { 1.0 } else { 0.0 };
        high_time += dt * (above_prev + above_curr) / 2.0;
    }

    Some(100.0 * high_time / total_time)
}

/// Calculate RMS (Root Mean Square) value
pub fn rms(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }

    let total_time = x.last()? - x.first()?;
    if total_time <= 0.0 {
        return None;
    }

    // Trapezoidal integration of y^2
    let mut integral = 0.0;
    for i in 1..x.len() {
        let dt = x[i] - x[i - 1];
        let y_sq_avg = (y[i - 1].powi(2) + y[i].powi(2)) / 2.0;
        integral += dt * y_sq_avg;
    }

    Some((integral / total_time).sqrt())
}

/// Calculate average (mean) value
pub fn average(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }

    let total_time = x.last()? - x.first()?;
    if total_time <= 0.0 {
        return None;
    }

    // Trapezoidal integration
    let mut integral = 0.0;
    for i in 1..x.len() {
        let dt = x[i] - x[i - 1];
        integral += dt * (y[i - 1] + y[i]) / 2.0;
    }

    Some(integral / total_time)
}

/// Calculate peak-to-peak value
pub fn peak_to_peak(y: &[f64]) -> Option<f64> {
    if y.is_empty() {
        return None;
    }

    let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    Some(y_max - y_min)
}

/// Calculate minimum value
pub fn minimum(y: &[f64]) -> Option<f64> {
    if y.is_empty() {
        return None;
    }
    Some(y.iter().cloned().fold(f64::INFINITY, f64::min))
}

/// Calculate maximum value
pub fn maximum(y: &[f64]) -> Option<f64> {
    if y.is_empty() {
        return None;
    }
    Some(y.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
}

/// Calculate all standard measurements for a waveform
pub fn all_measurements(x: &[f64], y: &[f64]) -> Vec<Measurement> {
    let mut results = Vec::new();

    if let Some(v) = peak_to_peak(y) {
        results.push(Measurement::new("Pk-Pk", v, "V"));
    }
    if let Some(v) = maximum(y) {
        results.push(Measurement::new("Max", v, "V"));
    }
    if let Some(v) = minimum(y) {
        results.push(Measurement::new("Min", v, "V"));
    }
    if let Some(v) = average(x, y) {
        results.push(Measurement::new("Avg", v, "V"));
    }
    if let Some(v) = rms(x, y) {
        results.push(Measurement::new("RMS", v, "V"));
    }
    if let Some(v) = frequency(x, y) {
        results.push(Measurement::new("Freq", v, "Hz"));
    }
    if let Some(v) = duty_cycle(x, y, None) {
        results.push(Measurement::new("Duty", v, "%"));
    }
    if let Some(v) = rise_time(x, y, 0.1, 0.9) {
        results.push(Measurement::new("Rise", v, "s"));
    }
    if let Some(v) = fall_time(x, y, 0.9, 0.1) {
        results.push(Measurement::new("Fall", v, "s"));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn generate_sine(n: usize, freq: f64, amplitude: f64, duration: f64) -> (Vec<f64>, Vec<f64>) {
        let x: Vec<f64> = (0..n)
            .map(|i| i as f64 * duration / (n - 1) as f64)
            .collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| amplitude * (2.0 * PI * freq * t).sin())
            .collect();
        (x, y)
    }

    fn generate_square(n: usize, freq: f64, amplitude: f64, duration: f64) -> (Vec<f64>, Vec<f64>) {
        let x: Vec<f64> = (0..n)
            .map(|i| i as f64 * duration / (n - 1) as f64)
            .collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| {
                let phase = (t * freq).fract();
                if phase < 0.5 {
                    amplitude
                } else {
                    -amplitude
                }
            })
            .collect();
        (x, y)
    }

    #[test]
    fn test_frequency_sine() {
        let (x, y) = generate_sine(1000, 1000.0, 1.0, 0.01); // 1kHz for 10ms
        let freq = frequency(&x, &y).unwrap();
        assert!((freq - 1000.0).abs() < 20.0, "Expected ~1kHz, got {}", freq);
    }

    #[test]
    fn test_rms_sine() {
        let (x, y) = generate_sine(1000, 100.0, 1.0, 0.1); // 100Hz sine, amplitude 1
        let rms_val = rms(&x, &y).unwrap();
        let expected = 1.0 / 2.0_f64.sqrt(); // RMS of sine = amplitude / sqrt(2)
        assert!(
            (rms_val - expected).abs() < 0.02,
            "Expected {}, got {}",
            expected,
            rms_val
        );
    }

    #[test]
    fn test_peak_to_peak() {
        let (_, y) = generate_sine(100, 100.0, 2.5, 0.02);
        let pp = peak_to_peak(&y).unwrap();
        assert!((pp - 5.0).abs() < 0.1, "Expected 5V pk-pk, got {}", pp);
    }

    #[test]
    fn test_duty_cycle_square() {
        let (x, y) = generate_square(1000, 100.0, 1.0, 0.05);
        let duty = duty_cycle(&x, &y, Some(0.0)).unwrap();
        assert!((duty - 50.0).abs() < 2.0, "Expected 50%, got {}%", duty);
    }

    #[test]
    fn test_average_dc_offset() {
        let n = 100;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.001).collect();
        let y: Vec<f64> = vec![2.5; n]; // DC 2.5V
        let avg = average(&x, &y).unwrap();
        assert!((avg - 2.5).abs() < 0.01, "Expected 2.5V, got {}", avg);
    }

    #[test]
    fn test_rise_time() {
        // Generate step response with known rise time
        let n = 1000;
        let tau = 1e-6; // 1µs time constant
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 10e-9).collect(); // 10ns steps
        let y: Vec<f64> = x.iter().map(|t| 1.0 - (-t / tau).exp()).collect();

        let rt = rise_time(&x, &y, 0.1, 0.9).unwrap();
        // For RC circuit: t_rise ≈ 2.2 * tau
        let expected = 2.2 * tau;
        assert!(
            (rt - expected).abs() / expected < 0.1,
            "Expected ~{}s, got {}s",
            expected,
            rt
        );
    }
}
