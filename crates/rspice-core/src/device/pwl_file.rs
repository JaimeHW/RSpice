//! PWL (Piecewise Linear) File Loading
//!
//! Supports loading waveform data from external files for PWL sources:
//! - **CSV format**: Two-column (time, value) text files
//! - **WAV format**: Audio files converted to time-value pairs
//!
//! # SPICE Syntax
//! ```text
//! V1 in 0 PWL FILE="stimulus.csv"
//! V2 in 0 PWL FILE="audio.wav" TSCALE=1e-3 VSCALE=5
//! ```
//!
//! # Performance
//! Uses binary search for O(log n) interpolation on large datasets.

use crate::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

//=============================================================================
// Errors
//=============================================================================

/// Errors that can occur when loading PWL files
#[derive(Debug)]
pub enum PwlFileError {
    /// File not found or inaccessible
    IoError(std::io::Error),
    /// Invalid CSV format
    ParseError(String),
    /// Invalid WAV format
    WavError(String),
    /// Empty file or no data points
    EmptyData,
    /// Time values not monotonically increasing
    NonMonotonic,
    /// Non-finite time/value data encountered
    NonFiniteData,
}

impl std::fmt::Display for PwlFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PwlFileError::IoError(e) => write!(f, "IO error: {}", e),
            PwlFileError::ParseError(s) => write!(f, "Parse error: {}", s),
            PwlFileError::WavError(s) => write!(f, "WAV error: {}", s),
            PwlFileError::EmptyData => write!(f, "No data points found"),
            PwlFileError::NonMonotonic => write!(f, "Time values must be monotonically increasing"),
            PwlFileError::NonFiniteData => write!(f, "Time/value data must be finite"),
        }
    }
}

impl std::error::Error for PwlFileError {}

impl From<std::io::Error> for PwlFileError {
    fn from(e: std::io::Error) -> Self {
        PwlFileError::IoError(e)
    }
}

//=============================================================================
// PWL Waveform Data
//=============================================================================

/// Loaded PWL waveform data with efficient interpolation
#[derive(Debug, Clone)]
pub struct PwlWaveform {
    /// Time points (sorted, monotonically increasing)
    times: Vec<Value>,
    /// Value points (corresponding to times)
    values: Vec<Value>,
    /// Time scaling factor
    pub time_scale: Value,
    /// Value scaling factor
    pub value_scale: Value,
    /// Time offset
    pub time_offset: Value,
    /// Value offset
    pub value_offset: Value,
}

impl PwlWaveform {
    /// Create a new PWL waveform from time-value pairs
    pub fn new(points: Vec<(Value, Value)>) -> Result<Self, PwlFileError> {
        if points.is_empty() {
            return Err(PwlFileError::EmptyData);
        }

        let mut times = Vec::with_capacity(points.len());
        let mut values = Vec::with_capacity(points.len());

        for (t, v) in points {
            if !t.is_finite() || !v.is_finite() {
                return Err(PwlFileError::NonFiniteData);
            }
            if !times.is_empty() && t <= *times.last().unwrap() {
                return Err(PwlFileError::NonMonotonic);
            }
            times.push(t);
            values.push(v);
        }

        Ok(Self {
            times,
            values,
            time_scale: 1.0,
            value_scale: 1.0,
            time_offset: 0.0,
            value_offset: 0.0,
        })
    }

    /// Create with scaling/offset parameters
    pub fn with_scaling(
        mut self,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Self {
        self.time_scale = time_scale;
        self.value_scale = value_scale;
        self.time_offset = time_offset;
        self.value_offset = value_offset;
        self
    }

    /// Get value at specified time with linear interpolation
    ///
    /// Uses binary search for O(log n) performance on large waveforms.
    pub fn value_at(&self, time: Value) -> Value {
        let scaled_start = self.values[0] * self.value_scale + self.value_offset;
        let scaled_end = self.values.last().copied().unwrap_or(self.values[0]) * self.value_scale
            + self.value_offset;

        if !time.is_finite() {
            return if time.is_sign_positive() {
                scaled_end
            } else {
                scaled_start
            };
        }
        if !self.time_scale.is_finite() || self.time_scale.abs() <= Value::EPSILON {
            return scaled_start;
        }

        // Apply time scaling and offset
        let t = (time - self.time_offset) / self.time_scale;
        if !t.is_finite() {
            return if t.is_sign_positive() {
                scaled_end
            } else {
                scaled_start
            };
        }

        // Handle edge cases
        if t <= self.times[0] {
            return scaled_start;
        }
        if t >= *self.times.last().unwrap() {
            return scaled_end;
        }

        // Binary search for the interval
        match self.times.binary_search_by(|probe| probe.total_cmp(&t)) {
            Ok(idx) => {
                // Exact match
                self.values[idx] * self.value_scale + self.value_offset
            }
            Err(idx) => {
                // Interpolate between idx-1 and idx
                let t0 = self.times[idx - 1];
                let t1 = self.times[idx];
                let v0 = self.values[idx - 1];
                let v1 = self.values[idx];

                let dt = t1 - t0;
                if !dt.is_finite() || dt.abs() <= Value::EPSILON {
                    return v0 * self.value_scale + self.value_offset;
                }
                let frac = (t - t0) / dt;
                if !frac.is_finite() {
                    return v0 * self.value_scale + self.value_offset;
                }
                let v = v0 + frac * (v1 - v0);
                v * self.value_scale + self.value_offset
            }
        }
    }

    /// Get number of data points
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Check if waveform is empty
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    /// Get the time range (start, end)
    pub fn time_range(&self) -> (Value, Value) {
        let start = self.times[0] * self.time_scale + self.time_offset;
        let end = self.times.last().unwrap() * self.time_scale + self.time_offset;
        (start, end)
    }

    /// Iterate over scaled knot times used by the waveform interpolation.
    ///
    /// Each returned time already includes `time_scale` and `time_offset`.
    pub fn scaled_knot_times(&self) -> impl Iterator<Item = Value> + '_ {
        self.times
            .iter()
            .copied()
            .map(|t| t * self.time_scale + self.time_offset)
    }
}

//=============================================================================
// File Loading
//=============================================================================

/// Load PWL waveform from a CSV file
///
/// Expected format: two columns (time, value) separated by comma, semicolon,
/// tab, or whitespace. Lines starting with '#' or containing non-numeric
/// data are skipped (header lines).
pub fn load_csv<P: AsRef<Path>>(path: P) -> Result<Vec<(Value, Value)>, PwlFileError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // Try to parse as two numeric values
        let parts: Vec<&str> = trimmed
            .split([',', ';', '\t', ' '])
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() >= 2 {
            match (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                (Ok(time), Ok(value)) => {
                    if !time.is_finite() || !value.is_finite() {
                        return Err(PwlFileError::NonFiniteData);
                    }
                    points.push((time, value));
                }
                _ => {
                    // Skip non-numeric lines (headers)
                    if points.is_empty() {
                        continue;
                    }
                    return Err(PwlFileError::ParseError(format!(
                        "Invalid data at line {}: '{}'",
                        line_num + 1,
                        trimmed
                    )));
                }
            }
        }
    }

    if points.is_empty() {
        return Err(PwlFileError::EmptyData);
    }

    // Verify monotonicity
    for i in 1..points.len() {
        if points[i].0 <= points[i - 1].0 {
            return Err(PwlFileError::NonMonotonic);
        }
    }

    Ok(points)
}

/// Load PWL waveform from a WAV audio file
///
/// Converts audio samples to time-value pairs where:
/// - Time is derived from sample rate
/// - Values are normalized to -1.0 to +1.0 range
pub fn load_wav<P: AsRef<Path>>(path: P) -> Result<Vec<(Value, Value)>, PwlFileError> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 44];

    file.read_exact(&mut header)
        .map_err(|_| PwlFileError::WavError("File too small for WAV header".to_string()))?;

    // Validate RIFF header
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(PwlFileError::WavError("Not a valid WAV file".to_string()));
    }

    // Parse format chunk
    if &header[12..16] != b"fmt " {
        return Err(PwlFileError::WavError("Missing fmt chunk".to_string()));
    }

    let audio_format = u16::from_le_bytes([header[20], header[21]]);
    if audio_format != 1 {
        return Err(PwlFileError::WavError(
            "Only PCM format supported".to_string(),
        ));
    }

    let num_channels = u16::from_le_bytes([header[22], header[23]]) as usize;
    let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]) as f64;
    let bits_per_sample = u16::from_le_bytes([header[34], header[35]]) as usize;

    // Find data chunk (may not be at offset 36)
    let data_size;
    let mut search_buf = [0u8; 8];

    // Read remaining header to find data chunk
    file.read_exact(&mut search_buf[0..8])
        .map_err(|_| PwlFileError::WavError("Cannot find data chunk".to_string()))?;

    if &search_buf[0..4] == b"data" {
        data_size =
            u32::from_le_bytes([search_buf[4], search_buf[5], search_buf[6], search_buf[7]]);
    } else {
        // Skip extra format bytes and find data chunk
        let extra_size =
            u32::from_le_bytes([search_buf[4], search_buf[5], search_buf[6], search_buf[7]]);
        let mut skip = vec![0u8; extra_size as usize];
        file.read_exact(&mut skip)?;

        file.read_exact(&mut search_buf[0..8])?;
        if &search_buf[0..4] == b"data" {
            data_size =
                u32::from_le_bytes([search_buf[4], search_buf[5], search_buf[6], search_buf[7]]);
        } else {
            return Err(PwlFileError::WavError("Cannot find data chunk".to_string()));
        }
    }

    let bytes_per_sample = bits_per_sample / 8;
    let num_samples = data_size as usize / (num_channels * bytes_per_sample);

    // Read audio data
    let mut audio_data = vec![0u8; data_size as usize];
    file.read_exact(&mut audio_data)?;

    // Convert to time-value pairs (use first channel only)
    let mut points = Vec::with_capacity(num_samples);
    let sample_period = 1.0 / sample_rate;
    let max_val = (1 << (bits_per_sample - 1)) as f64;

    for i in 0..num_samples {
        let time = i as f64 * sample_period;
        let offset = i * num_channels * bytes_per_sample;

        let sample_value = match bits_per_sample {
            8 => {
                // 8-bit is unsigned
                (audio_data[offset] as f64 - 128.0) / 128.0
            }
            16 => {
                let val = i16::from_le_bytes([audio_data[offset], audio_data[offset + 1]]);
                val as f64 / max_val
            }
            24 => {
                let val = i32::from_le_bytes([
                    0,
                    audio_data[offset],
                    audio_data[offset + 1],
                    audio_data[offset + 2],
                ]) >> 8;
                val as f64 / max_val
            }
            32 => {
                let val = i32::from_le_bytes([
                    audio_data[offset],
                    audio_data[offset + 1],
                    audio_data[offset + 2],
                    audio_data[offset + 3],
                ]);
                val as f64 / max_val
            }
            _ => {
                return Err(PwlFileError::WavError(format!(
                    "Unsupported bits per sample: {}",
                    bits_per_sample
                )));
            }
        };

        points.push((time, sample_value));
    }

    // Downsample if too many points (> 1M samples)
    if points.len() > 1_000_000 {
        let factor = points.len() / 1_000_000 + 1;
        points = points.into_iter().step_by(factor).collect();
    }

    Ok(points)
}

/// Load PWL waveform from file (auto-detect format by extension)
pub fn load_pwl_file<P: AsRef<Path>>(path: P) -> Result<PwlWaveform, PwlFileError> {
    let path_ref = path.as_ref();
    let extension = path_ref
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    let points = match extension.as_deref() {
        Some("wav") => load_wav(path)?,
        _ => load_csv(path)?, // Default to CSV
    };

    PwlWaveform::new(points)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwl_waveform_creation() {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5)];
        let waveform = PwlWaveform::new(points).unwrap();
        assert_eq!(waveform.len(), 3);
    }

    #[test]
    fn test_pwl_waveform_empty() {
        let result = PwlWaveform::new(vec![]);
        assert!(matches!(result, Err(PwlFileError::EmptyData)));
    }

    #[test]
    fn test_pwl_waveform_non_monotonic() {
        let points = vec![(0.0, 0.0), (2.0, 1.0), (1.0, 0.5)]; // 1.0 < 2.0
        let result = PwlWaveform::new(points);
        assert!(matches!(result, Err(PwlFileError::NonMonotonic)));
    }

    #[test]
    fn test_pwl_waveform_non_finite_data_rejected() {
        let points = vec![(0.0, 0.0), (f64::NAN, 1.0)];
        let result = PwlWaveform::new(points);
        assert!(matches!(result, Err(PwlFileError::NonFiniteData)));
    }

    #[test]
    fn test_pwl_interpolation() {
        let points = vec![(0.0, 0.0), (1.0, 10.0), (2.0, 5.0)];
        let waveform = PwlWaveform::new(points).unwrap();

        // Exact points
        assert!((waveform.value_at(0.0) - 0.0).abs() < 1e-10);
        assert!((waveform.value_at(1.0) - 10.0).abs() < 1e-10);
        assert!((waveform.value_at(2.0) - 5.0).abs() < 1e-10);

        // Interpolated
        assert!((waveform.value_at(0.5) - 5.0).abs() < 1e-10);
        assert!((waveform.value_at(1.5) - 7.5).abs() < 1e-10);
    }

    #[test]
    fn test_pwl_edge_cases() {
        let points = vec![(1.0, 10.0), (2.0, 20.0)];
        let waveform = PwlWaveform::new(points).unwrap();

        // Before first point
        assert!((waveform.value_at(0.0) - 10.0).abs() < 1e-10);
        // After last point
        assert!((waveform.value_at(5.0) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_pwl_scaling() {
        let points = vec![(0.0, 0.0), (1.0, 1.0)];
        let waveform = PwlWaveform::new(points)
            .unwrap()
            .with_scaling(2.0, 5.0, 0.0, 1.0);

        // At t=2.0, scaled time is 1.0, value is 1.0*5.0+1.0 = 6.0
        assert!((waveform.value_at(2.0) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_pwl_value_at_handles_non_finite_query_time() {
        let points = vec![(0.0, 10.0), (1.0, 20.0)];
        let waveform = PwlWaveform::new(points).unwrap();
        assert!((waveform.value_at(f64::NEG_INFINITY) - 10.0).abs() < 1e-10);
        assert!((waveform.value_at(f64::INFINITY) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_pwl_value_at_handles_invalid_time_scale() {
        let points = vec![(0.0, 10.0), (1.0, 20.0)];
        let waveform = PwlWaveform::new(points)
            .unwrap()
            .with_scaling(0.0, 1.0, 0.0, 0.0);
        assert!((waveform.value_at(0.5) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_pwl_time_range() {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5)];
        let waveform = PwlWaveform::new(points).unwrap();
        let (start, end) = waveform.time_range();
        assert!((start - 0.0).abs() < 1e-10);
        assert!((end - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_csv_parse_simple() {
        // Create a temporary CSV-like string for testing
        let csv_content = "# Time, Value\n0.0, 0.0\n1.0, 5.0\n2.0, 2.5\n";

        // Parse manually to test the logic
        let mut points = Vec::new();
        for line in csv_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split(',').collect();
            if parts.len() >= 2 {
                if let (Ok(t), Ok(v)) = (
                    parts[0].trim().parse::<f64>(),
                    parts[1].trim().parse::<f64>(),
                ) {
                    points.push((t, v));
                }
            }
        }

        assert_eq!(points.len(), 3);
        assert!((points[0].0 - 0.0).abs() < 1e-10);
        assert!((points[1].1 - 5.0).abs() < 1e-10);
    }
}
