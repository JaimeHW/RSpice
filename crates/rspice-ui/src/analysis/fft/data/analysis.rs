use super::FftData;
use crate::analysis::fft::window::WindowFunction;
// =============================================================================
// Spectrum Analysis
// =============================================================================

/// Analysis results from FFT spectrum
#[derive(Debug, Clone, Default)]
pub struct SpectrumAnalysis {
    /// Fundamental frequency (Hz)
    pub fundamental_frequency: Option<f64>,
    /// Fundamental magnitude (dB)
    pub fundamental_db: Option<f64>,
    /// Total Harmonic Distortion (%)
    pub thd_percent: Option<f64>,
    /// THD in dB
    pub thd_db: Option<f64>,
    /// Spurious-Free Dynamic Range (dB)
    pub sfdr_db: Option<f64>,
    /// Signal-to-Noise Ratio (dB)
    pub snr_db: Option<f64>,
    /// Signal-to-Noise-and-Distortion Ratio (dB)
    pub sinad_db: Option<f64>,
    /// Noise floor (dB)
    pub noise_floor_db: Option<f64>,
    /// Harmonic frequencies and magnitudes
    pub harmonics: Vec<(f64, f64)>,
}

impl SpectrumAnalysis {
    /// Analyze FFT data
    pub fn analyze(fft: &FftData, num_harmonics: usize) -> Self {
        let mut analysis = Self::default();
        let Some((fund_idx, _fund_peak)) = fft.find_peak() else {
            return analysis;
        };

        let fundamental = Self::interpolate_peak(fft, fund_idx).unwrap_or_else(|| {
            let p = fft.points[fund_idx];
            (fund_idx, p.frequency, p.magnitude, p.magnitude_db())
        });
        let (fund_bin, fund_freq, fund_mag, fund_db) = fundamental;
        let fund_power = fund_mag * fund_mag;
        if !(fund_power.is_finite() && fund_power > 0.0) {
            return analysis;
        }

        analysis.fundamental_frequency = Some(fund_freq);
        analysis.fundamental_db = Some(fund_db);

        let guard_bins = Self::guard_bins(fft.window);
        let mut excluded_for_noise = vec![false; fft.points.len()];
        let mut excluded_for_spur = vec![false; fft.points.len()];
        Self::exclude_bin_region(&mut excluded_for_noise, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_noise, fund_bin, guard_bins);
        Self::exclude_bin_region(&mut excluded_for_spur, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_spur, fund_bin, guard_bins);

        // Harmonic extraction
        let mut harmonic_power_sum = 0.0;
        let harmonic_count = num_harmonics.max(1);
        for h in 2..=harmonic_count {
            let target = fund_freq * h as f64;
            if target > fft.nyquist() {
                break;
            }
            let Some((harm_idx, harm_freq, harm_mag, harm_db)) =
                Self::find_harmonic_peak(fft, target)
            else {
                continue;
            };
            if !harm_mag.is_finite() || harm_mag <= 0.0 {
                continue;
            }

            analysis.harmonics.push((harm_freq, harm_db));
            harmonic_power_sum += harm_mag * harm_mag;
            Self::exclude_bin_region(&mut excluded_for_noise, harm_idx, guard_bins);
        }

        let thd_ratio = (harmonic_power_sum / fund_power).sqrt();
        if thd_ratio.is_finite() {
            analysis.thd_percent = Some(thd_ratio * 100.0);
            if thd_ratio > 0.0 {
                analysis.thd_db = Some(20.0 * thd_ratio.log10());
            }
        }

        // Largest spur outside guarded signal/harmonic bins.
        let mut largest_spur_db = f64::NEG_INFINITY;
        let mut noise_power_sum = 0.0;
        let mut noise_db_bins = Vec::new();
        for (idx, point) in fft.points.iter().enumerate().skip(1) {
            let mag = point.magnitude;
            if !mag.is_finite() || mag <= 0.0 {
                continue;
            }
            let db = point.magnitude_db();
            if !db.is_finite() {
                continue;
            }

            if !excluded_for_spur.get(idx).copied().unwrap_or(false) {
                largest_spur_db = largest_spur_db.max(db);
            }

            if excluded_for_noise.get(idx).copied().unwrap_or(false) {
                continue;
            }
            noise_power_sum += mag * mag;
            noise_db_bins.push(db);
        }

        if largest_spur_db.is_finite() {
            analysis.sfdr_db = Some(fund_db - largest_spur_db);
        }

        if !noise_db_bins.is_empty() {
            noise_db_bins.sort_by(|a, b| a.total_cmp(b));
            let mid = noise_db_bins.len() / 2;
            let noise_floor = if noise_db_bins.len() % 2 == 0 {
                0.5 * (noise_db_bins[mid - 1] + noise_db_bins[mid])
            } else {
                noise_db_bins[mid]
            };
            analysis.noise_floor_db = Some(noise_floor);
        }

        if noise_power_sum > 0.0 {
            let snr = 10.0 * (fund_power / noise_power_sum).log10();
            if snr.is_finite() {
                analysis.snr_db = Some(snr);
            }
        }

        let noise_and_distortion = noise_power_sum + harmonic_power_sum;
        if noise_and_distortion > 0.0 {
            let sinad = 10.0 * (fund_power / noise_and_distortion).log10();
            if sinad.is_finite() {
                analysis.sinad_db = Some(sinad);
            }
        }

        analysis
    }

    fn interpolate_peak(fft: &FftData, idx: usize) -> Option<(usize, f64, f64, f64)> {
        let point = fft.points.get(idx)?;
        let mut freq = point.frequency;
        let mut db = point.magnitude_db();
        if !db.is_finite() {
            return None;
        }

        if idx > 0 && idx + 1 < fft.points.len() {
            let left = fft.points[idx - 1].magnitude_db();
            let center = fft.points[idx].magnitude_db();
            let right = fft.points[idx + 1].magnitude_db();
            if left.is_finite() && center.is_finite() && right.is_finite() {
                let denom = left - 2.0 * center + right;
                if denom.abs() > 1e-12 {
                    let delta = (0.5 * (left - right) / denom).clamp(-0.5, 0.5);
                    let df = fft.frequency_resolution();
                    freq = point.frequency + delta * df;
                    db = center - 0.25 * (left - right) * delta;
                }
            }
        }

        let mag = if db.is_finite() {
            10.0_f64.powf(db / 20.0)
        } else {
            point.magnitude
        };
        if !mag.is_finite() || mag <= 0.0 {
            return None;
        }

        Some((idx, freq, mag, db))
    }

    fn find_harmonic_peak(fft: &FftData, target_freq: f64) -> Option<(usize, f64, f64, f64)> {
        if fft.points.len() < 3 {
            return None;
        }
        let df = fft.frequency_resolution();
        if df <= 0.0 || !df.is_finite() {
            return None;
        }

        let center = (target_freq / df).round() as isize;
        let search = 2isize;
        let min_idx = (center - search).max(1) as usize;
        let max_idx = (center + search).min((fft.points.len() - 1) as isize) as usize;
        if min_idx > max_idx {
            return None;
        }

        let mut best_idx = None;
        let mut best_mag = f64::NEG_INFINITY;
        for idx in min_idx..=max_idx {
            let mag = fft.points[idx].magnitude;
            if mag.is_finite() && mag > best_mag {
                best_mag = mag;
                best_idx = Some(idx);
            }
        }
        let idx = best_idx?;
        Self::interpolate_peak(fft, idx)
    }

    fn guard_bins(window: WindowFunction) -> usize {
        match window {
            WindowFunction::Rectangular => 1,
            WindowFunction::Hanning | WindowFunction::Hamming => 2,
            WindowFunction::Blackman | WindowFunction::Kaiser | WindowFunction::Gaussian => 3,
            WindowFunction::BlackmanHarris | WindowFunction::FlatTop => 4,
        }
    }

    fn exclude_bin_region(mask: &mut [bool], center: usize, radius: usize) {
        if mask.is_empty() {
            return;
        }
        let start = center.saturating_sub(radius);
        let end = (center + radius).min(mask.len() - 1);
        for value in mask.iter_mut().take(end + 1).skip(start) {
            *value = true;
        }
    }

    /// Format THD for display
    pub fn format_thd(&self) -> String {
        match self.thd_percent {
            Some(thd) if thd.is_finite() => format!("{:.3}%", thd),
            _ => "N/A".to_string(),
        }
    }

    /// Format SFDR for display
    pub fn format_sfdr(&self) -> String {
        match self.sfdr_db {
            Some(sfdr) if sfdr.is_finite() => format!("{:.1} dB", sfdr),
            _ => "N/A".to_string(),
        }
    }
}
