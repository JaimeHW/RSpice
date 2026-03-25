//! Waveform I/O
//!
//! Read and write waveform data in various formats.
//! Supports interchange formats used by commercial simulators.
//!
//! # Supported Formats
//!
//! - NUTMEG (SPICE3/ngspice raw format) for import
//! - CSV and TSV for import/export
//! - PSF-Lite binary waveform format (`PSFL`) for import
//! - PSF ASCII waveform exports (`psfascii`) for import
//! - Cadence PSF native binary waveform databases for import

#![allow(clippy::needless_range_loop, clippy::type_complexity)]
//! - Touchstone S-parameter format (`.sNp`) for import/export

use super::binary_io::PsfReader;
use super::cadence_psf::{ParsedCadencePsfBinary, parse_cadence_psf_binary};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// =============================================================================
// Waveform Format
// =============================================================================

/// Supported waveform file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaveformFormat {
    /// Cadence PSF (Parameter Storage Format)
    Psf,
    /// SPICE3/ngspice raw file (NUTMEG)
    Nutmeg,
    /// ASCII raw format
    AsciiRaw,
    /// Binary raw format
    BinaryRaw,
    /// CSV (comma-separated values)
    Csv,
    /// Tab-separated values
    Tsv,
    /// Touchstone S-parameter format
    Touchstone,
}

impl WaveformFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "psf" => Some(WaveformFormat::Psf),
            "raw" | "tr0" | "ac0" => Some(WaveformFormat::Nutmeg),
            "csv" => Some(WaveformFormat::Csv),
            "tsv" => Some(WaveformFormat::Tsv),
            "s1p" | "s2p" | "snp" => Some(WaveformFormat::Touchstone),
            _ if ext.len() >= 3
                && ext.starts_with('s')
                && ext.ends_with('p')
                && ext[1..ext.len() - 1].chars().all(|ch| ch.is_ascii_digit()) =>
            {
                Some(WaveformFormat::Touchstone)
            }
            _ => None,
        }
    }

    /// Default file extension
    pub fn extension(&self) -> &'static str {
        match self {
            WaveformFormat::Psf => "psf",
            WaveformFormat::Nutmeg => "raw",
            WaveformFormat::AsciiRaw => "raw",
            WaveformFormat::BinaryRaw => "raw",
            WaveformFormat::Csv => "csv",
            WaveformFormat::Tsv => "tsv",
            WaveformFormat::Touchstone => "s2p",
        }
    }

    /// Whether this format currently supports reading.
    pub fn can_read(&self) -> bool {
        matches!(
            self,
            WaveformFormat::Psf
                | WaveformFormat::Csv
                | WaveformFormat::Tsv
                | WaveformFormat::Nutmeg
                | WaveformFormat::AsciiRaw
                | WaveformFormat::Touchstone
        )
    }

    /// Whether this format currently supports writing.
    pub fn can_write(&self) -> bool {
        matches!(
            self,
            WaveformFormat::Csv | WaveformFormat::Tsv | WaveformFormat::Touchstone
        )
    }
}

// =============================================================================
// Waveform Signal
// =============================================================================

/// A single signal/variable in waveform data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformSignal {
    /// Signal name
    pub name: String,
    /// Signal type (voltage, current, time, etc.)
    pub signal_type: SignalType,
    /// Unit (V, A, s, Hz, etc.)
    pub unit: String,
    /// Data values
    pub data: Vec<f64>,
}

/// Type of signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SignalType {
    /// Time axis
    Time,
    /// Frequency axis
    Frequency,
    /// Voltage signal
    #[default]
    Voltage,
    /// Current signal
    Current,
    /// Power signal
    Power,
    /// Complex voltage (real part)
    VoltageReal,
    /// Complex voltage (imag part)
    VoltageImag,
    /// Complex current
    CurrentReal,
    /// Complex current (imag part)
    CurrentImag,
    /// S-parameter
    SParameter,
    /// Unknown
    Unknown,
}

impl WaveformSignal {
    /// Create a new signal
    pub fn new(name: impl Into<String>, signal_type: SignalType) -> Self {
        Self {
            name: name.into(),
            signal_type,
            unit: signal_type.default_unit().to_string(),
            data: Vec::new(),
        }
    }

    /// Add data point
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Get number of points
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get min value
    pub fn min(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::min)
    }

    /// Get max value
    pub fn max(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::max)
    }

    /// Get value at index
    pub fn get(&self, idx: usize) -> Option<f64> {
        self.data.get(idx).copied()
    }
}

impl SignalType {
    /// Default unit for signal type
    pub fn default_unit(&self) -> &'static str {
        match self {
            SignalType::Time => "s",
            SignalType::Frequency => "Hz",
            SignalType::Voltage | SignalType::VoltageReal | SignalType::VoltageImag => "V",
            SignalType::Current | SignalType::CurrentReal | SignalType::CurrentImag => "A",
            SignalType::Power => "W",
            SignalType::SParameter => "",
            SignalType::Unknown => "",
        }
    }

    fn from_label(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "time" => SignalType::Time,
            "frequency" | "freq" => SignalType::Frequency,
            "voltage" | "v" => SignalType::Voltage,
            "current" | "i" => SignalType::Current,
            "power" | "p" => SignalType::Power,
            _ => SignalType::Unknown,
        }
    }
}

impl std::str::FromStr for SignalType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_label(s))
    }
}

impl From<&str> for SignalType {
    fn from(s: &str) -> Self {
        Self::from_label(s)
    }
}

// =============================================================================
// Waveform Dataset
// =============================================================================

/// A complete waveform dataset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaveformDataset {
    /// Dataset title
    pub title: String,
    /// Analysis type (tran, ac, dc, etc.)
    pub analysis: String,
    /// Independent variable (usually time or frequency)
    pub x_signal: Option<WaveformSignal>,
    /// Dependent signals
    pub signals: Vec<WaveformSignal>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl WaveformDataset {
    /// Create a new dataset
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set X axis signal
    pub fn set_x(&mut self, signal: WaveformSignal) {
        self.x_signal = Some(signal);
    }

    /// Add a Y signal
    pub fn add_signal(&mut self, signal: WaveformSignal) {
        self.signals.push(signal);
    }

    /// Get signal by name
    pub fn get_signal(&self, name: &str) -> Option<&WaveformSignal> {
        self.signals.iter().find(|s| s.name == name)
    }

    /// Signal count
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Point count (from X signal)
    pub fn point_count(&self) -> usize {
        self.x_signal.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    /// Get signal names
    pub fn signal_names(&self) -> Vec<&str> {
        self.signals.iter().map(|s| s.name.as_str()).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TouchstoneDataFormat {
    Ri,
    Ma,
    Db,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TouchstoneMatrixFormat {
    Full,
    Lower,
    Upper,
}

#[derive(Debug, Clone, Copy)]
struct TouchstoneOptions {
    freq_scale_hz: f64,
    data_format: TouchstoneDataFormat,
    reference_ohms: f64,
}

// =============================================================================
// Waveform Reader
// =============================================================================

/// Waveform file reader
pub struct WaveformReader {
    format: WaveformFormat,
}

impl WaveformReader {
    /// Create reader for format
    pub fn new(format: WaveformFormat) -> Self {
        Self { format }
    }

    /// Read from file
    pub fn read(&self, path: &Path) -> Result<WaveformDataset, String> {
        match self.format {
            WaveformFormat::Psf => self.read_psf(path),
            WaveformFormat::Csv => self.read_csv(path),
            WaveformFormat::Tsv => self.read_tsv(path),
            WaveformFormat::Nutmeg | WaveformFormat::AsciiRaw => self.read_nutmeg(path),
            WaveformFormat::Touchstone => self.read_touchstone(path),
            _ => Err(format!(
                "Format {:?} read is not implemented (supported: PSF-Lite, Csv, Tsv, Nutmeg/AsciiRaw, Touchstone)",
                self.format
            )),
        }
    }

    /// Read PSF-Lite binary waveform format (`PSFL`).
    ///
    /// Supports both:
    /// - `PSFL` binary files emitted by rspice (`psf-lite`)
    /// - Cadence native PSF binary files
    /// - Cadence-style PSF ASCII exports (`psfascii`) from file or directory targets
    fn read_psf(&self, path: &Path) -> Result<WaveformDataset, String> {
        if path.is_dir() {
            return self.read_psf_directory(path);
        }
        if !path.is_file() {
            return Err(format!(
                "PSF path '{}' is neither a file nor a directory",
                path.display()
            ));
        }

        // First try rspice PSF-Lite binary.
        match self.read_psf_lite_file(path) {
            Ok(dataset) => Ok(dataset),
            Err(psf_lite_err) => {
                // Then try Cadence native binary PSF.
                match self.read_cadence_psf_binary_file(path) {
                    Ok(dataset) => Ok(dataset),
                    Err(cadence_bin_err) => {
                        // Finally fall back to PSF ASCII text parsing.
                        match self.read_psf_ascii_file(path) {
                            Ok(dataset) => Ok(dataset),
                            Err(psf_ascii_err) => Err(format!(
                                "Failed to read PSF '{}': {}; Cadence PSF binary parse failed: {}; PSF ASCII parse failed: {}",
                                path.display(),
                                psf_lite_err,
                                cadence_bin_err,
                                psf_ascii_err
                            )),
                        }
                    }
                }
            }
        }
    }

    fn read_psf_lite_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let mut reader = PsfReader::open(path)
            .map_err(|e| format!("Failed to open PSF-Lite file '{}': {}", path.display(), e))?;

        let header = reader.header().clone();
        if header.num_traces == 0 {
            return Err("PSF-Lite file contains zero traces".to_string());
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = "PSF-Lite".to_string();
        dataset
            .metadata
            .insert("format".to_string(), "psf-lite".to_string());
        dataset
            .metadata
            .insert("num_traces".to_string(), header.num_traces.to_string());
        dataset
            .metadata
            .insert("num_points".to_string(), header.num_points.to_string());
        dataset
            .metadata
            .insert("timestamp".to_string(), header.timestamp.to_string());

        let mut x = WaveformSignal::new("time", SignalType::Time);
        x.data = reader
            .read_trace(0)
            .map_err(|e| format!("Failed to read PSF-Lite trace 0: {}", e))?;
        dataset.set_x(x);

        for trace_idx in 1..header.num_traces {
            let mut signal =
                WaveformSignal::new(format!("trace{}", trace_idx), SignalType::Unknown);
            signal.data = reader
                .read_trace(trace_idx)
                .map_err(|e| format!("Failed to read PSF-Lite trace {}: {}", trace_idx, e))?;
            dataset.add_signal(signal);
        }

        Ok(dataset)
    }

    fn read_cadence_psf_binary_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Failed to read PSF file '{}': {}", path.display(), e))?;

        let parsed = std::panic::catch_unwind(|| parse_cadence_psf_binary(&bytes));
        let parsed = match parsed {
            Ok(Ok(parsed)) => parsed,
            Ok(Err(e)) => {
                return Err(format!(
                    "Cadence PSF binary parser error for '{}': {}",
                    path.display(),
                    e
                ));
            }
            Err(_) => {
                return Err(format!(
                    "Cadence PSF binary parser panicked for '{}'",
                    path.display()
                ));
            }
        };

        self.cadence_psf_binary_to_dataset(path, parsed)
    }

    fn cadence_psf_binary_to_dataset(
        &self,
        path: &Path,
        parsed: ParsedCadencePsfBinary,
    ) -> Result<WaveformDataset, String> {
        let has_declared_sweeps = !parsed.sweeps.is_empty();
        let mut sweep_traces: Vec<(String, Vec<f64>)> = parsed
            .sweeps
            .into_iter()
            .filter_map(|sweep| (!sweep.values.is_empty()).then_some((sweep.name, sweep.values)))
            .collect();
        let mut real_traces: HashMap<String, Vec<f64>> = parsed
            .real_signals
            .into_iter()
            .map(|signal| (signal.name, signal.values))
            .collect();
        let complex_traces: HashMap<String, Vec<(f64, f64)>> = parsed
            .complex_signals
            .into_iter()
            .map(|signal| (signal.name, signal.values))
            .collect();

        if real_traces.is_empty() && complex_traces.is_empty() && sweep_traces.is_empty() {
            return Err(format!(
                "Cadence PSF binary file '{}' contained no traces",
                path.display()
            ));
        }

        let mut x_candidate = sweep_traces
            .iter()
            .position(|(name, _)| name.eq_ignore_ascii_case("time"))
            .or_else(|| {
                sweep_traces.iter().position(|(name, _)| {
                    name.eq_ignore_ascii_case("freq") || name.eq_ignore_ascii_case("frequency")
                })
            })
            .or_else(|| (!sweep_traces.is_empty()).then_some(0))
            .map(|idx| sweep_traces.swap_remove(idx));

        if x_candidate.is_none() {
            let x_from_real = real_traces
                .iter()
                .find(|(name, values)| name.eq_ignore_ascii_case("time") && !values.is_empty())
                .map(|(name, _)| name.clone())
                .or_else(|| {
                    real_traces
                        .iter()
                        .find(|(name, values)| {
                            (name.eq_ignore_ascii_case("freq")
                                || name.eq_ignore_ascii_case("frequency"))
                                && !values.is_empty()
                        })
                        .map(|(name, _)| name.clone())
                });
            if let Some(name) = x_from_real
                && let Some(values) = real_traces.remove(&name) {
                    x_candidate = Some((name, values));
                }
        }

        let (x_name, x_values) = if let Some((name, values)) = x_candidate {
            (name, values)
        } else {
            let max_len = real_traces
                .values()
                .map(Vec::len)
                .chain(complex_traces.values().map(Vec::len))
                .max()
                .unwrap_or(0);
            if max_len == 0 {
                return Err(format!(
                    "Cadence PSF binary file '{}' has no usable sample vectors",
                    path.display()
                ));
            }
            (
                "index".to_string(),
                (0..max_len).map(|i| i as f64).collect(),
            )
        };

        let x_len = x_values.len();
        if x_len == 0 {
            return Err(format!(
                "Cadence PSF binary file '{}' has empty independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = if x_name.eq_ignore_ascii_case("time") {
            "Transient".to_string()
        } else if x_name.eq_ignore_ascii_case("freq") || x_name.eq_ignore_ascii_case("frequency") {
            "AC".to_string()
        } else if !has_declared_sweeps && x_name == "index" && x_len <= 1 {
            "DC OP".to_string()
        } else if has_declared_sweeps {
            "DC Sweep".to_string()
        } else {
            "PSF-Binary".to_string()
        };
        dataset
            .metadata
            .insert("format".to_string(), "psf-binary-cadence".to_string());
        dataset
            .metadata
            .insert("source_path".to_string(), path.display().to_string());

        let x_signal_type = if x_name.eq_ignore_ascii_case("time") {
            SignalType::Time
        } else if x_name.eq_ignore_ascii_case("freq") || x_name.eq_ignore_ascii_case("frequency") {
            SignalType::Frequency
        } else {
            SignalType::Unknown
        };
        let mut x_signal = WaveformSignal::new(x_name.clone(), x_signal_type);
        x_signal.data = x_values;
        dataset.set_x(x_signal);

        let mut real_names: Vec<_> = real_traces.keys().cloned().collect();
        real_names.sort();
        for signal_name in real_names {
            if signal_name == x_name {
                continue;
            }
            let Some(values) = real_traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                continue;
            }
            let mut signal =
                WaveformSignal::new(signal_name.clone(), Self::infer_signal_type(&signal_name));
            signal.data = values.clone();
            dataset.add_signal(signal);
        }

        let mut complex_names: Vec<_> = complex_traces.keys().cloned().collect();
        complex_names.sort();
        for signal_name in complex_names {
            let Some(values) = complex_traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                continue;
            }

            let mut real = WaveformSignal::new(
                format!("{}_RE", signal_name),
                Self::infer_complex_signal_type(&signal_name, false),
            );
            let mut imag = WaveformSignal::new(
                format!("{}_IM", signal_name),
                Self::infer_complex_signal_type(&signal_name, true),
            );
            real.data = values.iter().map(|(re, _)| *re).collect();
            imag.data = values.iter().map(|(_, im)| *im).collect();
            dataset.add_signal(real);
            dataset.add_signal(imag);
        }

        if dataset.signals.is_empty() {
            return Err(format!(
                "Cadence PSF binary file '{}' had no traces aligned to '{}'",
                path.display(),
                x_name
            ));
        }

        Ok(dataset)
    }

    fn read_psf_directory(&self, path: &Path) -> Result<WaveformDataset, String> {
        let mut candidates = Vec::new();

        // Prefer explicitly referenced run objects from logFile when available.
        let log_file = path.join("logFile");
        if log_file.is_file() {
            let file = File::open(&log_file).map_err(|e| {
                format!("Failed to open PSF logFile '{}': {}", log_file.display(), e)
            })?;
            for line in BufReader::new(file).lines() {
                let line =
                    line.map_err(|e| format!("Failed to read '{}': {}", log_file.display(), e))?;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                for token in trimmed.split_whitespace() {
                    let token = token.trim_matches(|c| c == '"' || c == '\'');
                    if token.ends_with(".psfascii")
                        || token.ends_with(".ascii")
                        || token.ends_with(".txt")
                        || token.ends_with(".psf")
                    {
                        let candidate = path.join(token);
                        if candidate.is_file() {
                            candidates.push(candidate);
                        }
                    }
                }
            }
        }

        // Also scan direct children for common waveform payload files.
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to scan PSF directory '{}': {}", path.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let child_path = entry.path();
            if !child_path.is_file() {
                continue;
            }
            let Some(name) = child_path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let lower = name.to_ascii_lowercase();
            let likely_payload = lower.ends_with(".psfascii")
                || lower.ends_with(".ascii")
                || lower.ends_with(".psf")
                || lower.contains("tran")
                || lower.contains("ac")
                || lower.contains("dc");
            if likely_payload {
                candidates.push(child_path);
            }
        }

        candidates.sort();
        candidates.dedup();

        let mut errors = Vec::new();
        for candidate in candidates {
            match self.read_psf_lite_file(&candidate) {
                Ok(dataset) => return Ok(dataset),
                Err(psf_lite_err) => match self.read_cadence_psf_binary_file(&candidate) {
                    Ok(dataset) => return Ok(dataset),
                    Err(cadence_bin_err) => match self.read_psf_ascii_file(&candidate) {
                        Ok(dataset) => return Ok(dataset),
                        Err(psf_ascii_err) => errors.push(format!(
                            "{}: {}; {}; {}",
                            candidate.display(),
                            psf_lite_err,
                            cadence_bin_err,
                            psf_ascii_err
                        )),
                    },
                },
            }
        }

        Err(format!(
            "No readable PSF waveform payload found in '{}'. Tried: {}",
            path.display(),
            if errors.is_empty() {
                "none".to_string()
            } else {
                errors.join(" | ")
            }
        ))
    }

    fn read_psf_ascii_file(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path)
            .map_err(|e| format!("Failed to open PSF ASCII file '{}': {}", path.display(), e))?;
        let reader = BufReader::new(file);

        let mut traces: HashMap<String, Vec<f64>> = HashMap::new();
        let mut vector_name: Option<String> = None;
        let mut vector_values: Vec<f64> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Read error in '{}': {}", path.display(), e))?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }

            if let Some(active_name) = vector_name.as_deref() {
                if trimmed.starts_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, active_name, &vector_values);
                    vector_name = None;
                    vector_values.clear();
                    continue;
                }
                vector_values.extend(Self::parse_psf_ascii_numbers(trimmed));
                if trimmed.ends_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, active_name, &vector_values);
                    vector_name = None;
                    vector_values.clear();
                }
                continue;
            }

            let Some((name, rhs)) = Self::parse_psf_ascii_assignment(trimmed) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }

            if rhs.starts_with('(') {
                let mut values = Self::parse_psf_ascii_numbers(rhs);
                if rhs.ends_with(')') {
                    Self::commit_psf_ascii_sample(&mut traces, &name, &values);
                } else {
                    vector_name = Some(name);
                    vector_values.append(&mut values);
                }
            } else {
                let values = Self::parse_psf_ascii_numbers(rhs);
                Self::commit_psf_ascii_sample(&mut traces, &name, &values);
            }
        }

        if let Some(active_name) = vector_name
            && !vector_values.is_empty() {
                Self::commit_psf_ascii_sample(&mut traces, &active_name, &vector_values);
            }

        if traces.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' contained no waveform assignments",
                path.display()
            ));
        }

        let mut x_name = None;
        for candidate in ["time", "freq", "frequency", "sweep", "sweepparam"] {
            if let Some((name, values)) = traces
                .iter()
                .find(|(name, values)| name.eq_ignore_ascii_case(candidate) && values.len() >= 2)
            {
                x_name = Some((name.clone(), values.clone()));
                break;
            }
        }
        if x_name.is_none() {
            x_name = traces
                .iter()
                .max_by_key(|(_, values)| values.len())
                .map(|(name, values)| (name.clone(), values.clone()));
        }

        let (x_name, x_values) = x_name.ok_or_else(|| {
            format!(
                "PSF ASCII file '{}' did not expose a usable independent variable",
                path.display()
            )
        })?;
        let x_len = x_values.len();
        if x_len == 0 {
            return Err(format!(
                "PSF ASCII file '{}' has empty independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("psf"));
        dataset.analysis = if x_name.eq_ignore_ascii_case("time") {
            "Transient".to_string()
        } else {
            "PSF-ASCII".to_string()
        };
        dataset
            .metadata
            .insert("format".to_string(), "psf-ascii".to_string());
        dataset
            .metadata
            .insert("source_path".to_string(), path.display().to_string());

        let mut x_signal = WaveformSignal::new(
            x_name.clone(),
            if x_name.eq_ignore_ascii_case("time") {
                SignalType::Time
            } else {
                SignalType::Frequency
            },
        );
        x_signal.data = x_values;
        dataset.set_x(x_signal);

        let mut signal_names: Vec<_> = traces.keys().cloned().collect();
        signal_names.sort();

        for signal_name in signal_names {
            if signal_name == x_name {
                continue;
            }
            let Some(values) = traces.get(&signal_name) else {
                continue;
            };
            if values.len() != x_len {
                // Keep strict x/y alignment for plotting correctness.
                continue;
            }
            let signal_type = if signal_name.to_ascii_lowercase().starts_with("v(") {
                SignalType::Voltage
            } else if signal_name.to_ascii_lowercase().starts_with("i(") {
                SignalType::Current
            } else {
                SignalType::Unknown
            };
            let mut signal = WaveformSignal::new(signal_name, signal_type);
            signal.data = values.clone();
            dataset.add_signal(signal);
        }

        if dataset.signals.is_empty() {
            return Err(format!(
                "PSF ASCII file '{}' had no signals aligned to independent variable '{}'",
                path.display(),
                x_name
            ));
        }

        Ok(dataset)
    }

    fn parse_psf_ascii_assignment(line: &str) -> Option<(String, &str)> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('"') {
            return None;
        }
        let mut chars = trimmed.char_indices();
        chars.next()?; // opening quote
        let end_quote = chars.find_map(|(idx, ch)| (ch == '"').then_some(idx))?;
        let name = trimmed[1..end_quote].trim().to_string();
        let rhs = trimmed[end_quote + 1..].trim_start();
        let rhs = rhs.strip_prefix('=').map(str::trim_start).unwrap_or(rhs);
        Some((name, rhs))
    }

    fn parse_psf_ascii_numbers(text: &str) -> Vec<f64> {
        text.split(|c: char| c.is_ascii_whitespace() || matches!(c, ',' | '(' | ')' | ';'))
            .filter_map(|token| {
                let token = token.trim();
                if token.is_empty() {
                    return None;
                }
                token.parse::<f64>().ok()
            })
            .collect()
    }

    fn commit_psf_ascii_sample(traces: &mut HashMap<String, Vec<f64>>, name: &str, values: &[f64]) {
        match values {
            [] => {}
            [value] => {
                traces.entry(name.to_string()).or_default().push(*value);
            }
            [re, im] => {
                traces.entry(format!("{}_RE", name)).or_default().push(*re);
                traces.entry(format!("{}_IM", name)).or_default().push(*im);
            }
            _ => {
                traces
                    .entry(name.to_string())
                    .or_default()
                    .extend_from_slice(values);
            }
        }
    }

    fn infer_signal_type(name: &str) -> SignalType {
        let lower = name.to_ascii_lowercase();
        if lower.starts_with("v(") {
            SignalType::Voltage
        } else if lower.starts_with("i(") {
            SignalType::Current
        } else if lower == "time" {
            SignalType::Time
        } else if lower == "freq" || lower == "frequency" {
            SignalType::Frequency
        } else {
            SignalType::Unknown
        }
    }

    fn infer_complex_signal_type(name: &str, imag: bool) -> SignalType {
        let lower = name.to_ascii_lowercase();
        if lower.starts_with("v(") {
            if imag {
                SignalType::VoltageImag
            } else {
                SignalType::VoltageReal
            }
        } else if lower.starts_with("i(") {
            if imag {
                SignalType::CurrentImag
            } else {
                SignalType::CurrentReal
            }
        } else {
            SignalType::Unknown
        }
    }

    /// Read CSV file
    fn read_csv(&self, path: &Path) -> Result<WaveformDataset, String> {
        self.read_delimited(path, ',')
    }

    /// Read TSV file
    fn read_tsv(&self, path: &Path) -> Result<WaveformDataset, String> {
        self.read_delimited(path, '\t')
    }

    /// Read delimited file
    fn read_delimited(&self, path: &Path, delimiter: char) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        let mut dataset =
            WaveformDataset::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or(""));
        let mut lines = reader.lines();

        // Header line
        let header = lines
            .next()
            .ok_or("Empty file")?
            .map_err(|e| format!("Read error: {}", e))?;

        let columns: Vec<&str> = header.split(delimiter).collect();
        if columns.is_empty() {
            return Err("No columns found".to_string());
        }

        // Create signals
        let mut signals: Vec<WaveformSignal> = columns
            .iter()
            .map(|name| {
                let name = name.trim();
                let sig_type = if name.to_lowercase() == "time" {
                    SignalType::Time
                } else if name.to_lowercase().starts_with("v(") {
                    SignalType::Voltage
                } else if name.to_lowercase().starts_with("i(") {
                    SignalType::Current
                } else {
                    SignalType::Unknown
                };
                WaveformSignal::new(name, sig_type)
            })
            .collect();

        // Data lines
        for line in lines {
            let line = line.map_err(|e| format!("Read error: {}", e))?;
            let values: Vec<&str> = line.split(delimiter).collect();

            for (i, val) in values.iter().enumerate() {
                if i < signals.len()
                    && let Ok(v) = val.trim().parse::<f64>() {
                        signals[i].push(v);
                    }
            }
        }

        // First column is typically X axis
        if !signals.is_empty() {
            dataset.x_signal = Some(signals.remove(0));
            dataset.signals = signals;
        }

        Ok(dataset)
    }

    /// Read NUTMEG/raw format
    fn read_nutmeg(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        let mut dataset = WaveformDataset::new("");
        let mut variables: Vec<(String, SignalType)> = Vec::new();
        let mut in_header = true;
        let mut num_points = 0;
        let mut values_buffer: Vec<Vec<f64>> = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Read error: {}", e))?;
            let trimmed = line.trim();

            if in_header {
                if let Some(stripped) = trimmed.strip_prefix("Title:") {
                    dataset.title = stripped.trim().to_string();
                } else if let Some(stripped) = trimmed.strip_prefix("Plotname:") {
                    dataset.analysis = stripped.trim().to_string();
                } else if trimmed.starts_with("No. Variables:") {
                    // Parse number of variables
                } else if let Some(stripped) = trimmed.strip_prefix("No. Points:") {
                    num_points = stripped.trim().parse().unwrap_or(0);
                } else if trimmed.starts_with("Variables:") {
                    // Next lines are variable definitions
                } else if trimmed.starts_with("Values:") {
                    in_header = false;
                    // Initialize buffers
                    values_buffer = vec![Vec::with_capacity(num_points); variables.len()];
                } else if trimmed.contains('\t') && !trimmed.is_empty() {
                    // Variable definition line: index\tname\ttype
                    let parts: Vec<&str> = trimmed.split('\t').collect();
                    if parts.len() >= 3 {
                        let name = parts[1].trim().to_string();
                        let sig_type = SignalType::from(parts[2].trim());
                        variables.push((name, sig_type));
                    }
                }
            } else {
                // Data section
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for (i, val) in parts.iter().enumerate() {
                    if i < values_buffer.len()
                        && let Ok(v) = val.parse::<f64>() {
                            values_buffer[i].push(v);
                        }
                }
            }
        }

        // Create signals from buffers
        for (i, (name, sig_type)) in variables.into_iter().enumerate() {
            let mut signal = WaveformSignal::new(name, sig_type);
            if i < values_buffer.len() {
                signal.data = values_buffer[i].clone();
            }

            if i == 0 {
                dataset.x_signal = Some(signal);
            } else {
                dataset.signals.push(signal);
            }
        }

        Ok(dataset)
    }

    /// Read Touchstone (`.sNp`) S-parameter file.
    ///
    /// Supports Touchstone v1 and v2 with S-parameter data in `RI`, `MA`, and `DB` formats.
    fn read_touchstone(&self, path: &Path) -> Result<WaveformDataset, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open: {}", e))?;
        let reader = BufReader::new(file);

        // Touchstone v1 defaults when no option line is provided.
        let mut options = TouchstoneOptions {
            freq_scale_hz: 1.0e9,
            data_format: TouchstoneDataFormat::Ma,
            reference_ohms: 50.0,
        };
        let mut version = 1u32;
        let mut matrix_format = TouchstoneMatrixFormat::Full;
        let mut declared_ports = Self::touchstone_ports_from_extension(path);
        let mut declared_freqs: Option<usize> = None;
        let mut reference_values: Option<Vec<f64>> = None;
        let mut numeric_tokens: Vec<f64> = Vec::new();

        for (line_idx, line_result) in reader.lines().enumerate() {
            let line_no = line_idx + 1;
            let mut line = line_result.map_err(|e| format!("Read error: {}", e))?;
            if let Some((before_comment, _)) = line.split_once('!') {
                line = before_comment.to_string();
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('#') {
                options = Self::parse_touchstone_option_line(trimmed, line_no)?;
                continue;
            }

            if trimmed.starts_with('[') {
                let (section, value) = Self::parse_touchstone_section_line(trimmed, line_no)?;
                match section.as_str() {
                    "version" => {
                        let parsed = value.parse::<f64>().map_err(|_| {
                            format!("Touchstone line {}: invalid [Version] '{}'", line_no, value)
                        })?;
                        if !parsed.is_finite() || parsed < 1.0 {
                            return Err(format!(
                                "Touchstone line {}: invalid [Version] '{}'",
                                line_no, value
                            ));
                        }
                        version = parsed.floor() as u32;
                    }
                    "number of ports" => {
                        declared_ports = Some(value.parse::<usize>().map_err(|_| {
                            format!(
                                "Touchstone line {}: invalid [Number of Ports] '{}'",
                                line_no, value
                            )
                        })?);
                    }
                    "number of frequencies" => {
                        declared_freqs = Some(value.parse::<usize>().map_err(|_| {
                            format!(
                                "Touchstone line {}: invalid [Number of Frequencies] '{}'",
                                line_no, value
                            )
                        })?);
                    }
                    "matrix format" => {
                        matrix_format = match value.to_ascii_lowercase().as_str() {
                            "full" => TouchstoneMatrixFormat::Full,
                            "lower" => TouchstoneMatrixFormat::Lower,
                            "upper" => TouchstoneMatrixFormat::Upper,
                            _ => {
                                return Err(format!(
                                    "Touchstone line {}: unsupported [Matrix Format] '{}'",
                                    line_no, value
                                ));
                            }
                        };
                    }
                    "reference" => {
                        reference_values =
                            Some(Self::parse_touchstone_numeric_values(value, line_no)?);
                    }
                    // Section present for clarity in v2 files; data parser is token-stream based.
                    "network data" | "end" => {}
                    _ => {}
                }
                continue;
            }

            for token in trimmed.split_whitespace() {
                if token == "+" {
                    continue;
                }
                let value = Self::parse_touchstone_numeric_token(token).ok_or_else(|| {
                    format!(
                        "Touchstone line {}: expected numeric token, got '{}'",
                        line_no, token
                    )
                })?;
                numeric_tokens.push(value);
            }
        }

        let num_ports = declared_ports
            .or_else(|| Self::infer_touchstone_ports_from_tokens(&numeric_tokens, matrix_format))
            .ok_or_else(|| "Unable to determine Touchstone port count".to_string())?;
        if num_ports == 0 {
            return Err("Touchstone [Number of Ports] must be >= 1".to_string());
        }

        let values_per_freq = Self::touchstone_values_per_frequency(num_ports, matrix_format)
            .ok_or_else(|| "Touchstone matrix dimensions overflow".to_string())?;
        if !crate::utils::numeric::is_multiple_of(numeric_tokens.len(), values_per_freq) {
            return Err(format!(
                "Touchstone numeric data length {} is not divisible by record width {}",
                numeric_tokens.len(),
                values_per_freq
            ));
        }
        let num_freqs = numeric_tokens.len() / values_per_freq;
        if num_freqs == 0 {
            return Err("Touchstone file contains no network data points".to_string());
        }
        if let Some(expected_freqs) = declared_freqs
            && expected_freqs != num_freqs {
                return Err(format!(
                    "Touchstone [Number of Frequencies]={} but parsed {} records",
                    expected_freqs, num_freqs
                ));
            }

        let z0_by_port = Self::resolve_touchstone_reference_values(
            num_ports,
            options.reference_ohms,
            reference_values.as_deref(),
        )?;

        let mut frequencies = Vec::with_capacity(num_freqs);
        let mut matrix_re = vec![vec![vec![0.0; num_freqs]; num_ports]; num_ports];
        let mut matrix_im = vec![vec![vec![0.0; num_freqs]; num_ports]; num_ports];

        let mut offset = 0usize;
        for freq_idx in 0..num_freqs {
            let freq_hz = numeric_tokens[offset] * options.freq_scale_hz;
            offset += 1;
            if !freq_hz.is_finite() || freq_hz <= 0.0 {
                return Err(format!(
                    "Touchstone frequency point {} is invalid ({})",
                    freq_idx, freq_hz
                ));
            }
            frequencies.push(freq_hz);

            // Touchstone matrix order: S11 S21 ... SN1 S12 S22 ... SN2 ... SNN.
            for col in 0..num_ports {
                match matrix_format {
                    TouchstoneMatrixFormat::Full => {
                        for row in 0..num_ports {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                    TouchstoneMatrixFormat::Lower => {
                        for row in col..num_ports {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                    TouchstoneMatrixFormat::Upper => {
                        for row in 0..=col {
                            let first = numeric_tokens[offset];
                            let second = numeric_tokens[offset + 1];
                            offset += 2;
                            let (re, im) = Self::touchstone_pair_to_complex(
                                first,
                                second,
                                options.data_format,
                            );
                            matrix_re[row][col][freq_idx] = re;
                            matrix_im[row][col][freq_idx] = im;
                        }
                    }
                }
            }

            match matrix_format {
                TouchstoneMatrixFormat::Full => {}
                TouchstoneMatrixFormat::Lower => {
                    for col in 0..num_ports {
                        for row in 0..col {
                            matrix_re[row][col][freq_idx] = matrix_re[col][row][freq_idx];
                            matrix_im[row][col][freq_idx] = matrix_im[col][row][freq_idx];
                        }
                    }
                }
                TouchstoneMatrixFormat::Upper => {
                    for col in 0..num_ports {
                        for row in (col + 1)..num_ports {
                            matrix_re[row][col][freq_idx] = matrix_re[col][row][freq_idx];
                            matrix_im[row][col][freq_idx] = matrix_im[col][row][freq_idx];
                        }
                    }
                }
            }
        }

        let mut dataset = WaveformDataset::new(
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("touchstone"),
        );
        dataset.analysis = "S-Parameter".to_string();
        dataset
            .metadata
            .insert("format".to_string(), "touchstone".to_string());
        dataset
            .metadata
            .insert("touchstone_version".to_string(), version.to_string());
        dataset
            .metadata
            .insert("num_ports".to_string(), num_ports.to_string());
        dataset.metadata.insert(
            "touchstone_matrix_format".to_string(),
            match matrix_format {
                TouchstoneMatrixFormat::Full => "full",
                TouchstoneMatrixFormat::Lower => "lower",
                TouchstoneMatrixFormat::Upper => "upper",
            }
            .to_string(),
        );
        dataset
            .metadata
            .insert("z0".to_string(), z0_by_port[0].to_string());
        dataset.metadata.insert(
            "z0_ports".to_string(),
            z0_by_port
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );

        let mut x_signal = WaveformSignal::new("frequency", SignalType::Frequency);
        x_signal.data = frequencies;
        dataset.set_x(x_signal);

        for row in 1..=num_ports {
            for col in 1..=num_ports {
                let base = if num_ports <= 9 {
                    format!("S{}{}", row, col)
                } else {
                    format!("S{}_{}", row, col)
                };

                let mut re_signal =
                    WaveformSignal::new(format!("{}_RE", base), SignalType::SParameter);
                re_signal.data = matrix_re[row - 1][col - 1].clone();
                dataset.add_signal(re_signal);

                let mut im_signal =
                    WaveformSignal::new(format!("{}_IM", base), SignalType::SParameter);
                im_signal.data = matrix_im[row - 1][col - 1].clone();
                dataset.add_signal(im_signal);
            }
        }

        Ok(dataset)
    }

    fn touchstone_ports_from_extension(path: &Path) -> Option<usize> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        if ext.len() >= 3
            && ext.starts_with('s')
            && ext.ends_with('p')
            && ext[1..ext.len() - 1].chars().all(|ch| ch.is_ascii_digit())
        {
            return ext[1..ext.len() - 1].parse::<usize>().ok();
        }
        None
    }

    fn touchstone_values_per_frequency(
        num_ports: usize,
        matrix_format: TouchstoneMatrixFormat,
    ) -> Option<usize> {
        let matrix_points = match matrix_format {
            TouchstoneMatrixFormat::Full => num_ports.checked_mul(num_ports)?,
            TouchstoneMatrixFormat::Lower | TouchstoneMatrixFormat::Upper => num_ports
                .checked_mul(num_ports.checked_add(1)?)?
                .checked_div(2)?,
        };
        matrix_points.checked_mul(2)?.checked_add(1)
    }

    fn infer_touchstone_ports_from_tokens(
        tokens: &[f64],
        matrix_format: TouchstoneMatrixFormat,
    ) -> Option<usize> {
        // Guard against unrealistic matrices while still supporting large N.
        const MAX_PORTS_TO_INFER: usize = 64;
        let mut best_match = None;
        for ports in 1..=MAX_PORTS_TO_INFER {
            let Some(record_width) = Self::touchstone_values_per_frequency(ports, matrix_format)
            else {
                continue;
            };
            if tokens.len() < record_width
                || !crate::utils::numeric::is_multiple_of(tokens.len(), record_width)
            {
                continue;
            }
            let num_freqs = tokens.len() / record_width;
            let freqs_valid = (0..num_freqs).all(|idx| {
                let freq = tokens[idx * record_width];
                freq.is_finite() && freq > 0.0
            });
            if freqs_valid {
                best_match = Some(ports);
            }
        }
        best_match
    }

    fn parse_touchstone_option_line(
        line: &str,
        line_no: usize,
    ) -> Result<TouchstoneOptions, String> {
        let fields: Vec<&str> = line[1..].split_whitespace().collect();
        if fields.is_empty() {
            return Err(format!("Touchstone line {}: empty option line", line_no));
        }

        let mut idx = 0usize;

        let freq_scale_hz = match fields[idx].to_ascii_lowercase().as_str() {
            "hz" => 1.0,
            "khz" => 1.0e3,
            "mhz" => 1.0e6,
            "ghz" => 1.0e9,
            other => {
                return Err(format!(
                    "Touchstone line {}: unsupported frequency unit '{}'",
                    line_no, other
                ));
            }
        };
        idx += 1;

        if idx >= fields.len() {
            return Err(format!(
                "Touchstone line {}: option line missing parameter type",
                line_no
            ));
        }
        if !fields[idx].eq_ignore_ascii_case("s") {
            return Err(format!(
                "Touchstone line {}: only S-parameter files are supported (found '{}')",
                line_no, fields[idx]
            ));
        }
        idx += 1;

        if idx >= fields.len() {
            return Err(format!(
                "Touchstone line {}: option line missing data format",
                line_no
            ));
        }
        let data_format = match fields[idx].to_ascii_lowercase().as_str() {
            "ri" => TouchstoneDataFormat::Ri,
            "ma" => TouchstoneDataFormat::Ma,
            "db" => TouchstoneDataFormat::Db,
            other => {
                return Err(format!(
                    "Touchstone line {}: unsupported data format '{}'",
                    line_no, other
                ));
            }
        };
        idx += 1;

        let mut reference_ohms = 50.0;
        if idx < fields.len() {
            if !fields[idx].eq_ignore_ascii_case("r") {
                return Err(format!(
                    "Touchstone line {}: expected 'R <reference>', found '{}'",
                    line_no, fields[idx]
                ));
            }
            idx += 1;
            if idx >= fields.len() {
                return Err(format!(
                    "Touchstone line {}: missing numeric value after 'R'",
                    line_no
                ));
            }
            reference_ohms =
                Self::parse_touchstone_numeric_token(fields[idx]).ok_or_else(|| {
                    format!(
                        "Touchstone line {}: invalid reference impedance '{}'",
                        line_no, fields[idx]
                    )
                })?;
            idx += 1;
        }

        if idx != fields.len() {
            return Err(format!(
                "Touchstone line {}: unexpected tokens in option line",
                line_no
            ));
        }
        if !reference_ohms.is_finite() || reference_ohms <= 0.0 {
            return Err(format!(
                "Touchstone line {}: reference impedance must be positive",
                line_no
            ));
        }

        Ok(TouchstoneOptions {
            freq_scale_hz,
            data_format,
            reference_ohms,
        })
    }

    fn parse_touchstone_section_line(line: &str, line_no: usize) -> Result<(String, &str), String> {
        let Some(end_bracket) = line.find(']') else {
            return Err(format!(
                "Touchstone line {}: malformed section header '{}'",
                line_no, line
            ));
        };
        if !line.starts_with('[') {
            return Err(format!(
                "Touchstone line {}: malformed section header '{}'",
                line_no, line
            ));
        }
        let section = line[1..end_bracket].trim().to_ascii_lowercase();
        if section.is_empty() {
            return Err(format!("Touchstone line {}: empty section header", line_no));
        }
        Ok((section, line[end_bracket + 1..].trim()))
    }

    fn parse_touchstone_numeric_values(value: &str, line_no: usize) -> Result<Vec<f64>, String> {
        let mut out = Vec::new();
        for token in value.split(|ch: char| ch.is_whitespace() || ch == ',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let parsed = Self::parse_touchstone_numeric_token(token).ok_or_else(|| {
                format!(
                    "Touchstone line {}: invalid numeric value '{}'",
                    line_no, token
                )
            })?;
            out.push(parsed);
        }
        if out.is_empty() {
            return Err(format!(
                "Touchstone line {}: section requires at least one numeric value",
                line_no
            ));
        }
        Ok(out)
    }

    fn parse_touchstone_numeric_token(token: &str) -> Option<f64> {
        token.replace(['D', 'd'], "e").parse::<f64>().ok()
    }

    fn resolve_touchstone_reference_values(
        num_ports: usize,
        default_reference: f64,
        override_values: Option<&[f64]>,
    ) -> Result<Vec<f64>, String> {
        let values: Vec<f64> = match override_values {
            Some(values) if values.len() == 1 => vec![values[0]; num_ports],
            Some(values) if values.len() == num_ports => values.to_vec(),
            Some(values) => {
                return Err(format!(
                    "Touchstone [Reference] count {} does not match port count {}",
                    values.len(),
                    num_ports
                ));
            }
            None => vec![default_reference; num_ports],
        };
        for (idx, value) in values.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 {
                return Err(format!(
                    "Touchstone reference impedance for port {} must be positive",
                    idx + 1
                ));
            }
        }
        Ok(values)
    }

    fn touchstone_pair_to_complex(
        first: f64,
        second: f64,
        format: TouchstoneDataFormat,
    ) -> (f64, f64) {
        match format {
            TouchstoneDataFormat::Ri => (first, second),
            TouchstoneDataFormat::Ma => {
                let angle = second.to_radians();
                (first * angle.cos(), first * angle.sin())
            }
            TouchstoneDataFormat::Db => {
                let magnitude = 10.0_f64.powf(first / 20.0);
                let angle = second.to_radians();
                (magnitude * angle.cos(), magnitude * angle.sin())
            }
        }
    }
}

// =============================================================================
// Waveform Writer
// =============================================================================

/// Waveform file writer
pub struct WaveformWriter {
    format: WaveformFormat,
}

impl WaveformWriter {
    /// Create writer for format
    pub fn new(format: WaveformFormat) -> Self {
        Self { format }
    }

    /// Write dataset to file
    pub fn write(&self, dataset: &WaveformDataset, path: &Path) -> Result<(), String> {
        match self.format {
            WaveformFormat::Csv => self.write_csv(dataset, path),
            WaveformFormat::Tsv => self.write_tsv(dataset, path),
            WaveformFormat::Touchstone => self.write_touchstone(dataset, path),
            _ => Err(format!(
                "Format {:?} write is not implemented (supported: Csv, Tsv, Touchstone)",
                self.format
            )),
        }
    }

    /// Write CSV
    fn write_csv(&self, dataset: &WaveformDataset, path: &Path) -> Result<(), String> {
        self.write_delimited(dataset, path, ',')
    }

    /// Write TSV
    fn write_tsv(&self, dataset: &WaveformDataset, path: &Path) -> Result<(), String> {
        self.write_delimited(dataset, path, '\t')
    }

    /// Write Touchstone N-port S-parameter data (RI format).
    fn write_touchstone(&self, dataset: &WaveformDataset, path: &Path) -> Result<(), String> {
        let (frequencies, matrix) = Self::extract_touchstone_matrix(dataset)?;
        let num_ports = matrix.len();
        let z0_by_port = Self::touchstone_reference_values_for_write(dataset, num_ports)?;
        let z0 = z0_by_port[0];
        let uniform_reference = z0_by_port.iter().all(|value| (*value - z0).abs() <= 1e-18);
        let version = dataset
            .metadata
            .get("touchstone_version")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(1);
        if version < 2 && !uniform_reference {
            return Err(
                "Touchstone v1 does not support per-port reference impedance; use version 2"
                    .to_string(),
            );
        }

        let file = File::create(path).map_err(|e| format!("Failed to create: {}", e))?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "! Generated by rspice-ui").map_err(|e| format!("Write error: {}", e))?;
        if version >= 2 {
            writeln!(writer, "[Version] 2.0").map_err(|e| format!("Write error: {}", e))?;
            writeln!(writer, "[Number of Ports] {}", num_ports)
                .map_err(|e| format!("Write error: {}", e))?;
            writeln!(writer, "[Number of Frequencies] {}", frequencies.len())
                .map_err(|e| format!("Write error: {}", e))?;
            writeln!(writer, "# Hz S RI R {}", z0).map_err(|e| format!("Write error: {}", e))?;
            if !uniform_reference {
                writeln!(
                    writer,
                    "[Reference] {}",
                    z0_by_port
                        .iter()
                        .map(|value| format!("{:.12e}", value))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
                .map_err(|e| format!("Write error: {}", e))?;
            }
            writeln!(writer, "[Network Data]").map_err(|e| format!("Write error: {}", e))?;
        } else {
            writeln!(writer, "# Hz S RI R {}", z0).map_err(|e| format!("Write error: {}", e))?;
        }

        for idx in 0..frequencies.len() {
            write!(writer, "{:.12e}", frequencies[idx])
                .map_err(|e| format!("Write error: {}", e))?;
            // Touchstone matrix order: S11 S21 ... SN1 S12 S22 ... SN2 ... SNN.
            for col in 0..num_ports {
                for row in 0..num_ports {
                    let (re, im) = matrix[row][col][idx];
                    write!(writer, " {:.12e} {:.12e}", re, im)
                        .map_err(|e| format!("Write error: {}", e))?;
                }
            }
            writeln!(writer).map_err(|e| format!("Write error: {}", e))?;
        }

        if version >= 2 {
            writeln!(writer, "[End]").map_err(|e| format!("Write error: {}", e))?;
        }

        writer.flush().map_err(|e| format!("Flush error: {}", e))
    }

    fn touchstone_reference_values_for_write(
        dataset: &WaveformDataset,
        num_ports: usize,
    ) -> Result<Vec<f64>, String> {
        let default_z0 = dataset
            .metadata
            .get("z0")
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(50.0);

        let values = match dataset.metadata.get("z0_ports") {
            Some(raw) => {
                let parsed: Vec<f64> = raw
                    .split(|ch: char| ch == ',' || ch.is_whitespace())
                    .filter(|token| !token.trim().is_empty())
                    .map(|token| {
                        token.trim().parse::<f64>().map_err(|_| {
                            format!("Invalid Touchstone z0_ports entry '{}'", token.trim())
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if parsed.is_empty() {
                    vec![default_z0; num_ports]
                } else if parsed.len() == 1 {
                    vec![parsed[0]; num_ports]
                } else if parsed.len() == num_ports {
                    parsed
                } else {
                    return Err(format!(
                        "Touchstone z0_ports count {} does not match {} ports",
                        parsed.len(),
                        num_ports
                    ));
                }
            }
            None => vec![default_z0; num_ports],
        };

        for (idx, value) in values.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 {
                return Err(format!(
                    "Touchstone reference impedance for port {} must be positive",
                    idx + 1
                ));
            }
        }

        Ok(values)
    }

    fn extract_touchstone_matrix(
        dataset: &WaveformDataset,
    ) -> Result<(Vec<f64>, Vec<Vec<Vec<(f64, f64)>>>), String> {
        let x = dataset
            .x_signal
            .as_ref()
            .ok_or_else(|| "Touchstone export requires an X-axis frequency signal".to_string())?;
        if x.data.is_empty() {
            return Err("Touchstone export requires at least one frequency point".to_string());
        }
        let expected_len = x.data.len();

        let mut real_signals: HashMap<(usize, usize), &WaveformSignal> = HashMap::new();
        let mut imag_signals: HashMap<(usize, usize), &WaveformSignal> = HashMap::new();
        let mut max_port = 0usize;

        for signal in &dataset.signals {
            let Some((row, col, is_imag)) = Self::parse_touchstone_signal_name(&signal.name) else {
                continue;
            };
            max_port = max_port.max(row).max(col);
            let key = (row, col);
            if is_imag {
                if imag_signals.insert(key, signal).is_some() {
                    return Err(format!(
                        "Duplicate Touchstone imag component for S{}{}",
                        row, col
                    ));
                }
            } else if real_signals.insert(key, signal).is_some() {
                return Err(format!(
                    "Duplicate Touchstone real component for S{}{}",
                    row, col
                ));
            }
        }

        if max_port < 2 {
            return Err(
                "Touchstone export requires at least a 2-port S-parameter matrix".to_string(),
            );
        }

        let mut matrix = vec![vec![vec![(0.0, 0.0); expected_len]; max_port]; max_port];
        for row in 1..=max_port {
            for col in 1..=max_port {
                let key = (row, col);
                let real = real_signals.get(&key).ok_or_else(|| {
                    format!("Missing Touchstone real component for S{}{}", row, col)
                })?;
                let imag = imag_signals.get(&key).ok_or_else(|| {
                    format!("Missing Touchstone imag component for S{}{}", row, col)
                })?;
                if real.data.len() != expected_len || imag.data.len() != expected_len {
                    return Err(format!(
                        "Touchstone signal length mismatch for S{}{} (freq={}, real={}, imag={})",
                        row,
                        col,
                        expected_len,
                        real.data.len(),
                        imag.data.len()
                    ));
                }
                matrix[row - 1][col - 1] = real
                    .data
                    .iter()
                    .zip(imag.data.iter())
                    .map(|(re, im)| (*re, *im))
                    .collect();
            }
        }

        Ok((x.data.clone(), matrix))
    }

    fn parse_touchstone_signal_name(name: &str) -> Option<(usize, usize, bool)> {
        let normalized = name.trim().to_ascii_uppercase().replace(' ', "");
        // Prefer longest suffix matches first.
        let suffixes = [
            ("_IMAG", true),
            ("IMAG", true),
            ("_REAL", false),
            ("REAL", false),
            ("_IM", true),
            ("IM", true),
            ("_RE", false),
            ("RE", false),
            ("_I", true),
            ("I", true),
            ("_R", false),
            ("R", false),
        ];
        for (suffix, is_imag) in suffixes {
            if let Some(base) = normalized.strip_suffix(suffix)
                && let Some((row, col)) = Self::parse_touchstone_base_name(base) {
                    return Some((row, col, is_imag));
                }
        }
        None
    }

    fn parse_touchstone_base_name(name: &str) -> Option<(usize, usize)> {
        let rest = name.strip_prefix('S')?;
        if let Some(inner) = rest.strip_prefix('(').and_then(|v| v.strip_suffix(')')) {
            let (row, col) = inner.split_once(',')?;
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if let Some((row, col)) = rest.split_once('_') {
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if rest.len() == 2 && rest.chars().all(|ch| ch.is_ascii_digit()) {
            let row = rest[0..1].parse::<usize>().ok()?;
            let col = rest[1..2].parse::<usize>().ok()?;
            return Some((row, col));
        }
        None
    }

    /// Write delimited file
    fn write_delimited(
        &self,
        dataset: &WaveformDataset,
        path: &Path,
        delimiter: char,
    ) -> Result<(), String> {
        let file = File::create(path).map_err(|e| format!("Failed to create: {}", e))?;
        let mut writer = BufWriter::new(file);

        // Header
        let mut headers = Vec::new();
        if let Some(ref x) = dataset.x_signal {
            headers.push(x.name.as_str());
        }
        for sig in &dataset.signals {
            headers.push(sig.name.as_str());
        }
        writeln!(writer, "{}", headers.join(&delimiter.to_string()))
            .map_err(|e| format!("Write error: {}", e))?;

        // Data
        let num_points = dataset.point_count();
        for i in 0..num_points {
            let mut values = Vec::new();

            if let Some(ref x) = dataset.x_signal {
                values.push(x.get(i).map(|v| v.to_string()).unwrap_or_default());
            }
            for sig in &dataset.signals {
                values.push(sig.get(i).map(|v| v.to_string()).unwrap_or_default());
            }

            writeln!(writer, "{}", values.join(&delimiter.to_string()))
                .map_err(|e| format!("Write error: {}", e))?;
        }

        writer.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests;
