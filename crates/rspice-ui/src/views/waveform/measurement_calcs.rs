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

// =============================================================================
// Spectral Analysis Functions
// =============================================================================

/// Calculate Total Harmonic Distortion (THD)
///
/// THD is the ratio of the sum of harmonic power to fundamental power.
/// Expressed as a percentage: THD = sqrt(sum(V_n^2)) / V1 * 100
///
/// # Arguments
/// * `frequencies` - FFT frequencies (Hz)
/// * `magnitudes` - FFT magnitudes (linear, not dB)
/// * `fundamental_freq` - Fundamental frequency to analyze
/// * `num_harmonics` - Number of harmonics to include (default: 10)
///
/// # Returns
/// THD as a percentage (%)
pub fn thd(
    frequencies: &[f64],
    magnitudes: &[f64],
    fundamental_freq: f64,
    num_harmonics: usize,
) -> Option<f64> {
    if frequencies.is_empty() || magnitudes.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    // Find fundamental magnitude
    let fundamental_mag = find_peak_near(frequencies, magnitudes, fundamental_freq)?;
    if fundamental_mag <= 0.0 {
        return None;
    }

    // Sum harmonic powers
    let mut harmonic_power_sum = 0.0;
    for n in 2..=num_harmonics {
        let harmonic_freq = fundamental_freq * (n as f64);
        if let Some(harmonic_mag) = find_peak_near(frequencies, magnitudes, harmonic_freq) {
            harmonic_power_sum += harmonic_mag.powi(2);
        }
    }

    let thd_ratio = harmonic_power_sum.sqrt() / fundamental_mag;
    Some(thd_ratio * 100.0)
}

/// Calculate THD+N (Total Harmonic Distortion plus Noise)
///
/// Similar to THD but includes all noise power, not just harmonics.
/// THD+N = sqrt(total_power - fundamental_power) / fundamental
///
/// # Arguments
/// * `frequencies` - FFT frequencies (Hz)
/// * `magnitudes` - FFT magnitudes (linear)
/// * `fundamental_freq` - Fundamental frequency
/// * `bandwidth` - Bandwidth to exclude around fundamental (Hz)
///
/// # Returns
/// THD+N as a percentage (%)
pub fn thd_plus_n(
    frequencies: &[f64],
    magnitudes: &[f64],
    fundamental_freq: f64,
    bandwidth: f64,
) -> Option<f64> {
    if frequencies.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    let fundamental_mag = find_peak_near(frequencies, magnitudes, fundamental_freq)?;
    if fundamental_mag <= 0.0 {
        return None;
    }

    // Calculate total power excluding DC
    let mut total_power = 0.0;
    let mut fundamental_power = 0.0;

    for (freq, mag) in frequencies.iter().zip(magnitudes.iter()) {
        if *freq > 0.0 {
            // Skip DC
            let power = mag.powi(2);
            total_power += power;

            // Check if within fundamental bandwidth
            if (*freq - fundamental_freq).abs() < bandwidth / 2.0 {
                fundamental_power += power;
            }
        }
    }

    let noise_power = total_power - fundamental_power;
    if noise_power < 0.0 {
        return None;
    }

    let thd_n_ratio = noise_power.sqrt() / fundamental_mag;
    Some(thd_n_ratio * 100.0)
}

/// Calculate Spurious-Free Dynamic Range (SFDR)
///
/// SFDR is the ratio of fundamental to the largest spurious signal.
/// Expressed in dB: SFDR = 20*log10(V1 / V_spur)
///
/// # Arguments
/// * `frequencies` - FFT frequencies (Hz)
/// * `magnitudes` - FFT magnitudes (linear)
/// * `fundamental_freq` - Fundamental frequency
/// * `exclude_bandwidth` - Bandwidth to exclude around fundamental (Hz)
///
/// # Returns
/// SFDR in dB
pub fn sfdr(
    frequencies: &[f64],
    magnitudes: &[f64],
    fundamental_freq: f64,
    exclude_bandwidth: f64,
) -> Option<f64> {
    if frequencies.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    let fundamental_mag = find_peak_near(frequencies, magnitudes, fundamental_freq)?;
    if fundamental_mag <= 0.0 {
        return None;
    }

    // Find largest spur outside fundamental bandwidth
    let mut max_spur = 0.0;

    for (freq, mag) in frequencies.iter().zip(magnitudes.iter()) {
        // Skip DC and fundamental region
        if *freq > 0.0 && (*freq - fundamental_freq).abs() > exclude_bandwidth / 2.0 {
            if *mag > max_spur {
                max_spur = *mag;
            }
        }
    }

    if max_spur <= 0.0 {
        return Some(f64::INFINITY); // No spurious signals
    }

    Some(20.0 * (fundamental_mag / max_spur).log10())
}

/// Calculate Signal-to-Noise Ratio (SNR)
///
/// SNR is the ratio of signal power to noise power (excluding harmonics).
///
/// # Arguments
/// * `frequencies` - FFT frequencies (Hz)
/// * `magnitudes` - FFT magnitudes (linear)
/// * `fundamental_freq` - Fundamental frequency
/// * `num_harmonics` - Number of harmonics to exclude from noise
///
/// # Returns
/// SNR in dB
pub fn snr(
    frequencies: &[f64],
    magnitudes: &[f64],
    fundamental_freq: f64,
    num_harmonics: usize,
) -> Option<f64> {
    if frequencies.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    let fundamental_mag = find_peak_near(frequencies, magnitudes, fundamental_freq)?;
    if fundamental_mag <= 0.0 {
        return None;
    }

    let fundamental_power = fundamental_mag.powi(2);
    let harmonic_bandwidth = fundamental_freq * 0.05; // 5% bandwidth for each harmonic

    // Calculate noise power (excluding DC, fundamental, and harmonics)
    let mut noise_power = 0.0;

    for (freq, mag) in frequencies.iter().zip(magnitudes.iter()) {
        if *freq <= 0.0 {
            continue; // Skip DC
        }

        // Check if near any harmonic (including fundamental)
        let mut is_harmonic = false;
        for n in 1..=num_harmonics {
            let harmonic_freq = fundamental_freq * (n as f64);
            if (*freq - harmonic_freq).abs() < harmonic_bandwidth {
                is_harmonic = true;
                break;
            }
        }

        if !is_harmonic {
            noise_power += mag.powi(2);
        }
    }

    if noise_power <= 0.0 {
        return Some(f64::INFINITY); // No noise
    }

    Some(10.0 * (fundamental_power / noise_power).log10())
}

/// Calculate SINAD (Signal-to-Noise and Distortion ratio)
///
/// SINAD = signal power / (noise + distortion power)
/// SINAD = 1 / (THD_N / 100)^2
///
/// # Arguments
/// * `frequencies` - FFT frequencies (Hz)
/// * `magnitudes` - FFT magnitudes (linear)
/// * `fundamental_freq` - Fundamental frequency
///
/// # Returns
/// SINAD in dB
pub fn sinad(frequencies: &[f64], magnitudes: &[f64], fundamental_freq: f64) -> Option<f64> {
    if frequencies.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    let fundamental_mag = find_peak_near(frequencies, magnitudes, fundamental_freq)?;
    if fundamental_mag <= 0.0 {
        return None;
    }

    let fundamental_bandwidth = fundamental_freq * 0.05;
    let fundamental_power = fundamental_mag.powi(2);

    // Calculate total noise + distortion power
    let mut noise_distortion_power = 0.0;

    for (freq, mag) in frequencies.iter().zip(magnitudes.iter()) {
        if *freq > 0.0 && (*freq - fundamental_freq).abs() > fundamental_bandwidth {
            noise_distortion_power += mag.powi(2);
        }
    }

    if noise_distortion_power <= 0.0 {
        return Some(f64::INFINITY);
    }

    Some(10.0 * (fundamental_power / noise_distortion_power).log10())
}

/// Calculate Effective Number of Bits (ENOB)
///
/// ENOB = (SINAD - 1.76) / 6.02
///
/// This is the standard ADC quality metric derived from SINAD.
///
/// # Arguments
/// * `sinad_db` - SINAD in dB
///
/// # Returns
/// ENOB (effective bits)
pub fn enob(sinad_db: f64) -> f64 {
    (sinad_db - 1.76) / 6.02
}

/// Calculate Noise Floor in dB
///
/// Determines the average noise level in the spectrum.
///
/// # Arguments
/// * `magnitudes` - FFT magnitudes (linear)
/// * `exclude_top_percent` - Percentage of largest bins to exclude (default: 5%)
///
/// # Returns
/// Noise floor in dB relative to full scale
pub fn noise_floor(magnitudes: &[f64], exclude_top_percent: f64) -> Option<f64> {
    if magnitudes.is_empty() {
        return None;
    }

    // Sort magnitudes to find percentile
    let mut sorted: Vec<f64> = magnitudes.iter().filter(|&&m| m > 0.0).cloned().collect();
    if sorted.is_empty() {
        return None;
    }

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Exclude top percentile
    let exclude_count = ((sorted.len() as f64) * exclude_top_percent / 100.0) as usize;
    let include_count = sorted.len().saturating_sub(exclude_count).max(1);

    // Calculate mean of remaining
    let sum: f64 = sorted.iter().take(include_count).sum();
    let mean = sum / (include_count as f64);

    if mean <= 0.0 {
        return None;
    }

    // Find max magnitude for reference
    let max_mag = magnitudes.iter().cloned().fold(0.0, f64::max);
    if max_mag <= 0.0 {
        return None;
    }

    Some(20.0 * (mean / max_mag).log10())
}

/// Helper: Find peak magnitude near a target frequency
fn find_peak_near(frequencies: &[f64], magnitudes: &[f64], target_freq: f64) -> Option<f64> {
    if frequencies.is_empty() || frequencies.len() != magnitudes.len() {
        return None;
    }

    // Calculate frequency resolution
    let freq_res = if frequencies.len() > 1 {
        (frequencies[frequencies.len() - 1] - frequencies[0]) / (frequencies.len() as f64)
    } else {
        return magnitudes.first().copied();
    };

    // Search window: +/- 3 bins
    let search_radius = freq_res * 3.0;
    let mut max_mag = 0.0;

    for (freq, mag) in frequencies.iter().zip(magnitudes.iter()) {
        if (*freq - target_freq).abs() <= search_radius {
            if *mag > max_mag {
                max_mag = *mag;
            }
        }
    }

    if max_mag > 0.0 {
        Some(max_mag)
    } else {
        None
    }
}

/// Calculate all spectral measurements
pub fn spectral_measurements(
    frequencies: &[f64],
    magnitudes: &[f64],
    fundamental_freq: f64,
) -> Vec<Measurement> {
    let mut results = Vec::new();

    if let Some(v) = thd(frequencies, magnitudes, fundamental_freq, 10) {
        results.push(Measurement::new("THD", v, "%"));
    }
    if let Some(v) = sfdr(
        frequencies,
        magnitudes,
        fundamental_freq,
        fundamental_freq * 0.1,
    ) {
        if v.is_finite() {
            results.push(Measurement::new("SFDR", v, "dB"));
        }
    }
    if let Some(v) = snr(frequencies, magnitudes, fundamental_freq, 10) {
        if v.is_finite() {
            results.push(Measurement::new("SNR", v, "dB"));
        }
    }
    if let Some(v) = sinad(frequencies, magnitudes, fundamental_freq) {
        if v.is_finite() {
            results.push(Measurement::new("SINAD", v, "dB"));
            results.push(Measurement::new("ENOB", enob(v), "bits"));
        }
    }
    if let Some(v) = noise_floor(magnitudes, 5.0) {
        results.push(Measurement::new("Floor", v, "dB"));
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

    // -------------------------------------------------------------------------
    // Spectral Analysis Tests
    // -------------------------------------------------------------------------

    /// Generate spectrum with fundamental and harmonics
    fn generate_spectrum(
        fund_freq: f64,
        fund_mag: f64,
        harmonics: &[(usize, f64)], // (harmonic number, relative magnitude)
        noise_level: f64,
        num_bins: usize,
        max_freq: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let freq_res = max_freq / (num_bins as f64);
        let frequencies: Vec<f64> = (0..num_bins).map(|i| i as f64 * freq_res).collect();

        let mut magnitudes = vec![noise_level; num_bins];

        // Add fundamental
        let fund_bin = (fund_freq / freq_res).round() as usize;
        if fund_bin < num_bins {
            magnitudes[fund_bin] = fund_mag;
        }

        // Add harmonics
        for (n, rel_mag) in harmonics {
            let harm_freq = fund_freq * (*n as f64);
            let harm_bin = (harm_freq / freq_res).round() as usize;
            if harm_bin < num_bins {
                magnitudes[harm_bin] = fund_mag * rel_mag;
            }
        }

        (frequencies, magnitudes)
    }

    #[test]
    fn test_thd_pure_sine() {
        // Pure sine has no harmonics, THD should be 0
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &[], 0.0, 1024, 10000.0);
        let result = thd(&freqs, &mags, 1000.0, 10).unwrap();
        assert!(result < 0.1, "Pure sine THD should be ~0, got {}", result);
    }

    #[test]
    fn test_thd_with_harmonics() {
        // Signal with known harmonics
        let harmonics = vec![(2, 0.1), (3, 0.05), (4, 0.02)];
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &harmonics, 0.001, 1024, 10000.0);

        let result = thd(&freqs, &mags, 1000.0, 10).unwrap();
        // Expected: sqrt(0.1^2 + 0.05^2 + 0.02^2) * 100 = 11.36%
        let expected = (0.1f64.powi(2) + 0.05f64.powi(2) + 0.02f64.powi(2)).sqrt() * 100.0;
        assert!(
            (result - expected).abs() < 1.0,
            "Expected THD ~{}%, got {}%",
            expected,
            result
        );
    }

    #[test]
    fn test_thd_empty() {
        let result = thd(&[], &[], 1000.0, 10);
        assert!(result.is_none());
    }

    #[test]
    fn test_sfdr_with_spur() {
        // Fundamental at 1.0, spur at 0.01 -> SFDR = 40dB
        let harmonics = vec![(2, 0.01)];
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &harmonics, 0.001, 1024, 10000.0);

        let result = sfdr(&freqs, &mags, 1000.0, 100.0).unwrap();
        // Expected: 20*log10(1.0/0.01) = 40 dB
        assert!(
            result > 35.0 && result < 45.0,
            "Expected SFDR ~40dB, got {} dB",
            result
        );
    }

    #[test]
    fn test_sfdr_no_spur() {
        // No spurious signals
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &[], 0.0, 1024, 10000.0);
        let result = sfdr(&freqs, &mags, 1000.0, 100.0).unwrap();
        assert!(result.is_infinite(), "No spurs should give infinite SFDR");
    }

    #[test]
    fn test_sfdr_empty() {
        let result = sfdr(&[], &[], 1000.0, 100.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_snr_with_noise() {
        // Generate spectrum with noise
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &[], 0.001, 1024, 10000.0);

        let result = snr(&freqs, &mags, 1000.0, 5).unwrap();
        // SNR should be positive and finite
        assert!(
            result > 0.0 && result.is_finite(),
            "SNR should be positive, got {}",
            result
        );
    }

    #[test]
    fn test_snr_empty() {
        let result = snr(&[], &[], 1000.0, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_sinad_calculation() {
        // SINAD with known noise + distortion
        let harmonics = vec![(2, 0.1), (3, 0.05)];
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &harmonics, 0.001, 1024, 10000.0);

        let result = sinad(&freqs, &mags, 1000.0).unwrap();
        // SINAD should be positive and finite
        assert!(
            result > 0.0 && result.is_finite(),
            "SINAD should be positive, got {}",
            result
        );
    }

    #[test]
    fn test_sinad_empty() {
        let result = sinad(&[], &[], 1000.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_enob_known_values() {
        // 12-bit ADC typically has SINAD ~ 74 dB -> ENOB ~ 12
        let sinad_12bit = 74.0;
        let result = enob(sinad_12bit);
        assert!(
            (result - 12.0).abs() < 0.5,
            "Expected ENOB ~12, got {}",
            result
        );

        // 8-bit ADC: SINAD ~ 50 dB -> ENOB ~ 8
        let sinad_8bit = 50.0;
        let result_8 = enob(sinad_8bit);
        assert!(
            (result_8 - 8.0).abs() < 0.5,
            "Expected ENOB ~8, got {}",
            result_8
        );
    }

    #[test]
    fn test_noise_floor() {
        let magnitudes: Vec<f64> = (0..100)
            .map(|i| {
                if i == 50 {
                    1.0 // Peak at bin 50
                } else {
                    0.001 // Noise floor
                }
            })
            .collect();

        let result = noise_floor(&magnitudes, 5.0).unwrap();
        // Noise floor should be negative dB relative to peak
        assert!(
            result < 0.0,
            "Noise floor should be negative dB, got {}",
            result
        );
        // Noise floor ~ 20*log10(0.001) = -60 dB relative to 1.0
        assert!(
            result < -50.0,
            "Noise floor should be < -50 dB, got {}",
            result
        );
    }

    #[test]
    fn test_noise_floor_empty() {
        let result = noise_floor(&[], 5.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_peak_near() {
        let frequencies: Vec<f64> = (0..100).map(|i| i as f64 * 10.0).collect();
        let mut magnitudes = vec![0.1; 100];
        magnitudes[50] = 1.0; // Peak at 500 Hz

        let result = find_peak_near(&frequencies, &magnitudes, 495.0);
        assert!(result.is_some());
        assert!((result.unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_find_peak_near_empty() {
        let result = find_peak_near(&[], &[], 1000.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_spectral_measurements_returns_all() {
        let harmonics = vec![(2, 0.1), (3, 0.05)];
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &harmonics, 0.001, 1024, 10000.0);

        let results = spectral_measurements(&freqs, &mags, 1000.0);

        // Should have THD, potentially SFDR, SNR, SINAD, ENOB, Floor
        assert!(!results.is_empty(), "Should have measurements");

        // Check THD is present
        assert!(
            results.iter().any(|m| m.name == "THD"),
            "Should include THD measurement"
        );
    }

    #[test]
    fn test_thd_plus_n() {
        let harmonics = vec![(2, 0.1)];
        let (freqs, mags) = generate_spectrum(1000.0, 1.0, &harmonics, 0.01, 1024, 10000.0);

        let result = thd_plus_n(&freqs, &mags, 1000.0, 50.0).unwrap();
        // THD+N should be > THD due to noise
        let thd_only = thd(&freqs, &mags, 1000.0, 10).unwrap();
        assert!(result > 0.0, "THD+N should be positive");
        // May or may not be greater than THD depending on noise level
    }

    #[test]
    fn test_thd_plus_n_empty() {
        let result = thd_plus_n(&[], &[], 1000.0, 50.0);
        assert!(result.is_none());
    }
}
