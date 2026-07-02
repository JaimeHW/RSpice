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
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
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
        self.value_at_raw_time(time)
    }

    /// Get value at specified time, repeating from `repeat_from` after the
    /// final source-time knot when requested.
    pub fn value_at_repeating(&self, time: Value, repeat_from: Option<Value>) -> Value {
        let time = self.repeated_time(time, repeat_from);
        self.value_at_raw_time(time)
    }

    fn repeated_time(&self, time: Value, repeat_from: Option<Value>) -> Value {
        let Some(repeat_from) = repeat_from else {
            return time;
        };
        if !repeat_from.is_finite()
            || !self.time_scale.is_finite()
            || self.time_scale.abs() <= Value::EPSILON
        {
            return time;
        }
        let Some(&last) = self.times.last() else {
            return time;
        };
        let t = (time - self.time_offset) / self.time_scale;
        if !t.is_finite() || t <= last {
            return time;
        }
        let first = self.times[0];
        let repeat_start = repeat_from.max(first);
        if repeat_start >= last {
            return time;
        }
        let period = last - repeat_start;
        if !period.is_finite() || period <= Value::EPSILON {
            return time;
        }
        let elapsed = t - repeat_start;
        let remainder = elapsed.rem_euclid(period);
        let boundary_tolerance = Value::EPSILON * elapsed.abs().max(period).max(1.0);
        if remainder <= boundary_tolerance {
            return self.time_offset + last * self.time_scale;
        }
        let repeated = repeat_start + remainder;
        self.time_offset + repeated * self.time_scale
    }

    fn value_at_raw_time(&self, time: Value) -> Value {
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

    /// Last source-time knot before scaling and offset.
    pub fn last_source_time(&self) -> Value {
        self.times.last().copied().unwrap_or(0.0)
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

        if parts.len() < 2 {
            if points.is_empty() {
                continue;
            }
            return Err(PwlFileError::ParseError(format!(
                "Invalid data at line {}: expected 2 columns, got {}: '{}'",
                line_num + 1,
                parts.len(),
                trimmed
            )));
        }

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
    let mut riff_header = [0u8; 12];

    file.read_exact(&mut riff_header)
        .map_err(|_| PwlFileError::WavError("File too small for WAV header".to_string()))?;

    // Validate RIFF header
    if &riff_header[0..4] != b"RIFF" || &riff_header[8..12] != b"WAVE" {
        return Err(PwlFileError::WavError("Not a valid WAV file".to_string()));
    }

    let mut format = None;
    let audio_data = loop {
        let mut chunk_header = [0u8; 8];
        match file.read_exact(&mut chunk_header) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err(PwlFileError::WavError("Cannot find data chunk".to_string()));
            }
            Err(e) => return Err(PwlFileError::IoError(e)),
        }

        let chunk_id = &chunk_header[0..4];
        let chunk_size = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]) as u64;

        match chunk_id {
            b"fmt " => {
                if chunk_size < 16 {
                    return Err(PwlFileError::WavError("fmt chunk is too small".to_string()));
                }
                let mut fmt = vec![0u8; chunk_size as usize];
                file.read_exact(&mut fmt)?;
                if !chunk_size.is_multiple_of(2) {
                    file.seek(SeekFrom::Current(1))?;
                }
                format = Some(WavPcmFormat::parse(&fmt)?);
            }
            b"data" => {
                let format = format.ok_or_else(|| {
                    PwlFileError::WavError("data chunk appeared before fmt chunk".to_string())
                })?;
                break read_wav_data_chunk(&mut file, chunk_size, format)?;
            }
            _ => {
                let skip = chunk_size + (chunk_size % 2);
                file.seek(SeekFrom::Current(skip as i64))?;
            }
        }
    };

    let WavAudioData {
        data: audio_data,
        data_size,
        format,
    } = audio_data;

    let frame_bytes = format.frame_bytes;
    if data_size == 0 {
        return Err(PwlFileError::EmptyData);
    }
    if data_size % frame_bytes != 0 {
        return Err(PwlFileError::WavError(format!(
            "data chunk size {data_size} is not aligned to {frame_bytes}-byte sample frames"
        )));
    }

    let num_samples = (data_size / frame_bytes) as usize;
    if num_samples == 0 {
        return Err(PwlFileError::EmptyData);
    }

    let max_val = format.normalization_scale;

    // Convert to time-value pairs (use first channel only)
    let mut points = Vec::with_capacity(num_samples);
    let sample_period = 1.0 / format.sample_rate as f64;

    for i in 0..num_samples {
        let time = i as f64 * sample_period;
        let offset = i * frame_bytes as usize;

        let sample_value = match format.bits_per_sample {
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
                    format.bits_per_sample
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

#[derive(Debug, Clone, Copy)]
struct WavPcmFormat {
    sample_rate: u32,
    bits_per_sample: usize,
    frame_bytes: u64,
    normalization_scale: f64,
}

impl WavPcmFormat {
    fn parse(fmt: &[u8]) -> Result<Self, PwlFileError> {
        let audio_format = u16::from_le_bytes([fmt[0], fmt[1]]);
        if audio_format != 1 {
            return Err(PwlFileError::WavError(
                "Only PCM format supported".to_string(),
            ));
        }

        let num_channels = u16::from_le_bytes([fmt[2], fmt[3]]);
        if num_channels == 0 {
            return Err(PwlFileError::WavError(
                "WAV channel count must be greater than zero".to_string(),
            ));
        }

        let sample_rate = u32::from_le_bytes([fmt[4], fmt[5], fmt[6], fmt[7]]);
        if sample_rate == 0 {
            return Err(PwlFileError::WavError(
                "WAV sample rate must be greater than zero".to_string(),
            ));
        }

        let block_align = u16::from_le_bytes([fmt[12], fmt[13]]) as u64;
        let bits_per_sample = u16::from_le_bytes([fmt[14], fmt[15]]) as usize;
        let bytes_per_sample = match bits_per_sample {
            8 | 16 | 24 | 32 => (bits_per_sample / 8) as u64,
            other => {
                return Err(PwlFileError::WavError(format!(
                    "Unsupported bits per sample: {other}"
                )));
            }
        };

        let frame_bytes = u64::from(num_channels) * bytes_per_sample;
        if block_align == 0 || block_align != frame_bytes {
            return Err(PwlFileError::WavError(format!(
                "WAV block alignment {block_align} does not match {frame_bytes}-byte sample frames"
            )));
        }

        let normalization_scale = match bits_per_sample {
            8 => 128.0,
            16 => 32768.0,
            24 => 8_388_608.0,
            32 => 2_147_483_648.0,
            _ => unreachable!("bits_per_sample was validated above"),
        };

        Ok(Self {
            sample_rate,
            bits_per_sample,
            frame_bytes,
            normalization_scale,
        })
    }
}

struct WavAudioData {
    data: Vec<u8>,
    data_size: u64,
    format: WavPcmFormat,
}

fn read_wav_data_chunk(
    file: &mut File,
    data_size: u64,
    format: WavPcmFormat,
) -> Result<WavAudioData, PwlFileError> {
    let data_start = file.stream_position()?;
    let file_len = file.metadata()?.len();
    if data_size > file_len.saturating_sub(data_start) {
        return Err(PwlFileError::WavError(format!(
            "data chunk declares {data_size} bytes but file has only {} bytes remaining",
            file_len.saturating_sub(data_start)
        )));
    }

    let mut data = vec![0u8; data_size as usize];
    file.read_exact(&mut data)?;
    if !data_size.is_multiple_of(2) {
        file.seek(SeekFrom::Current(1))?;
    }

    Ok(WavAudioData {
        data,
        data_size,
        format,
    })
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
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is available")
            .as_nanos();
        path.push(format!("rspice-pwl-file-{name}-{nonce}.wav"));
        path
    }

    fn write_temp_wav(name: &str, bytes: &[u8]) -> PathBuf {
        let path = temp_path(name);
        fs::write(&path, bytes).expect("write temporary WAV fixture");
        path
    }

    fn write_temp_csv(name: &str, contents: &str) -> PathBuf {
        let path = temp_path(name).with_extension("csv");
        fs::write(&path, contents).expect("write temporary CSV fixture");
        path
    }

    fn pcm_wav_bytes(
        num_channels: u16,
        sample_rate: u32,
        bits_per_sample: u16,
        sample_bytes: &[u8],
    ) -> Vec<u8> {
        let bytes_per_sample = u32::from(bits_per_sample).div_ceil(8);
        let block_align = u32::from(num_channels).saturating_mul(bytes_per_sample) as u16;
        let byte_rate = sample_rate.saturating_mul(u32::from(block_align));
        let data_size = sample_bytes.len() as u32;
        let riff_size = 4 + (8 + 16) + (8 + data_size);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&riff_size.to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&num_channels.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&byte_rate.to_le_bytes());
        bytes.extend_from_slice(&block_align.to_le_bytes());
        bytes.extend_from_slice(&bits_per_sample.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_size.to_le_bytes());
        bytes.extend_from_slice(sample_bytes);
        bytes
    }

    fn assert_wav_error_contains(bytes: Vec<u8>, needle: &str) {
        let path = write_temp_wav("bad-metadata", &bytes);
        let err = load_wav(&path).expect_err("malformed WAV metadata must be rejected");
        let _ = fs::remove_file(&path);
        match err {
            PwlFileError::WavError(message) => {
                assert!(
                    message.contains(needle),
                    "expected WAV error containing `{needle}`, got `{message}`"
                );
            }
            other => panic!("expected WAV metadata error, got {other:?}"),
        }
    }

    #[test]
    fn pwl_waveform_repeats_from_requested_source_time() {
        let waveform = PwlWaveform::new(vec![(0.0, 1.0), (2.0, 5.0), (4.0, 3.0)]).unwrap();

        assert!((waveform.value_at_repeating(5.0, Some(0.0)) - 3.0).abs() < 1e-15);
        assert!((waveform.value_at_repeating(4.5, Some(2.0)) - 4.5).abs() < 1e-15);
        assert_eq!(waveform.value_at_repeating(8.0, Some(0.0)), 3.0);
        assert_eq!(waveform.value_at_repeating(4.5, None), 3.0);
    }

    #[test]
    fn standard_pcm_wav_loads_samples() {
        let path = write_temp_wav("standard", &pcm_wav_bytes(1, 8_000, 8, &[0, 128, 255]));

        let points = load_wav(&path).expect("standard PCM WAV should load");
        let _ = fs::remove_file(&path);

        assert_eq!(points.len(), 3);
        assert_eq!(points[0], (0.0, -1.0));
        assert!((points[1].0 - 1.0 / 8_000.0).abs() < 1e-15);
        assert_eq!(points[1].1, 0.0);
        assert!((points[2].0 - 2.0 / 8_000.0).abs() < 1e-15);
        assert!((points[2].1 - 127.0 / 128.0).abs() < 1e-15);
    }

    #[test]
    fn wav_loader_rejects_invalid_pcm_metadata_before_sample_math() {
        assert_wav_error_contains(pcm_wav_bytes(0, 8_000, 8, &[128]), "channel");
        assert_wav_error_contains(pcm_wav_bytes(1, 0, 8, &[128]), "sample rate");
        assert_wav_error_contains(pcm_wav_bytes(1, 8_000, 7, &[128]), "bits per sample");
        assert_wav_error_contains(pcm_wav_bytes(1, 8_000, 16, &[0]), "data chunk size");
    }

    #[test]
    fn csv_loader_rejects_short_rows_after_data_starts() {
        let path = write_temp_csv("short-row", "time,value\n0,0\n1e-6\n2e-6,1\n");

        let err = load_csv(&path).expect_err("short CSV rows after data starts must reject");
        let _ = fs::remove_file(&path);

        match err {
            PwlFileError::ParseError(message) => {
                assert!(
                    message.contains("line 3") && message.contains("expected 2 columns"),
                    "unexpected parse message: {message}"
                );
            }
            other => panic!("expected CSV parse error, got {other:?}"),
        }
    }
}
