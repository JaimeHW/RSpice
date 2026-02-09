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
//!
//! # Planned Formats
//!
//! - Cadence PSF native database import
//! - Touchstone S-parameter format

use super::binary_io::PsfReader;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
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

    /// Parse from string
    pub fn from_str(s: &str) -> Self {
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
            _ => Err(format!(
                "Format {:?} read is not implemented (supported: PSF-Lite, Csv, Tsv, Nutmeg/AsciiRaw)",
                self.format
            )),
        }
    }

    /// Read PSF-Lite binary waveform format (`PSFL`).
    ///
    /// This currently supports rspice's PSF-Lite container and does not yet parse
    /// Cadence native PSF directory databases.
    fn read_psf(&self, path: &Path) -> Result<WaveformDataset, String> {
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
                if i < signals.len() {
                    if let Ok(v) = val.trim().parse::<f64>() {
                        signals[i].push(v);
                    }
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
                if trimmed.starts_with("Title:") {
                    dataset.title = trimmed[6..].trim().to_string();
                } else if trimmed.starts_with("Plotname:") {
                    dataset.analysis = trimmed[9..].trim().to_string();
                } else if trimmed.starts_with("No. Variables:") {
                    // Parse number of variables
                } else if trimmed.starts_with("No. Points:") {
                    num_points = trimmed[11..].trim().parse().unwrap_or(0);
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
                        let sig_type = SignalType::from_str(parts[2].trim());
                        variables.push((name, sig_type));
                    }
                }
            } else {
                // Data section
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                for (i, val) in parts.iter().enumerate() {
                    if i < values_buffer.len() {
                        if let Ok(v) = val.parse::<f64>() {
                            values_buffer[i].push(v);
                        }
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
        let z0 = dataset
            .metadata
            .get("z0")
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|v| *v > 0.0)
            .unwrap_or(50.0);
        let version = dataset
            .metadata
            .get("touchstone_version")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(1);

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
            if let Some(base) = normalized.strip_suffix(suffix) {
                if let Some((row, col)) = Self::parse_touchstone_base_name(base) {
                    return Some((row, col, is_imag));
                }
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
mod tests {
    use super::*;
    use crate::io::binary_io::{PsfHeader, PsfWriter};
    use std::io::Write;
    use tempfile::NamedTempFile;

    // =========================================================================
    // WaveformFormat Tests
    // =========================================================================

    #[test]
    fn test_format_from_extension() {
        assert_eq!(
            WaveformFormat::from_extension("csv"),
            Some(WaveformFormat::Csv)
        );
        assert_eq!(
            WaveformFormat::from_extension("raw"),
            Some(WaveformFormat::Nutmeg)
        );
        assert_eq!(
            WaveformFormat::from_extension("psf"),
            Some(WaveformFormat::Psf)
        );
        assert_eq!(
            WaveformFormat::from_extension("s3p"),
            Some(WaveformFormat::Touchstone)
        );
        assert_eq!(WaveformFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_format_capabilities() {
        assert!(WaveformFormat::Csv.can_read());
        assert!(WaveformFormat::Csv.can_write());
        assert!(WaveformFormat::Nutmeg.can_read());
        assert!(!WaveformFormat::Nutmeg.can_write());
        assert!(WaveformFormat::Psf.can_read());
        assert!(WaveformFormat::Touchstone.can_write());
    }

    // =========================================================================
    // SignalType Tests
    // =========================================================================

    #[test]
    fn test_signal_type_from_str() {
        assert_eq!(SignalType::from_str("time"), SignalType::Time);
        assert_eq!(SignalType::from_str("voltage"), SignalType::Voltage);
        assert_eq!(SignalType::from_str("unknown_type"), SignalType::Unknown);
    }

    #[test]
    fn test_signal_type_unit() {
        assert_eq!(SignalType::Time.default_unit(), "s");
        assert_eq!(SignalType::Voltage.default_unit(), "V");
    }

    // =========================================================================
    // WaveformSignal Tests
    // =========================================================================

    #[test]
    fn test_signal_creation() {
        let sig = WaveformSignal::new("v(out)", SignalType::Voltage);
        assert_eq!(sig.name, "v(out)");
        assert_eq!(sig.unit, "V");
    }

    #[test]
    fn test_signal_push() {
        let mut sig = WaveformSignal::new("test", SignalType::Voltage);
        sig.push(1.0);
        sig.push(2.0);
        sig.push(3.0);

        assert_eq!(sig.len(), 3);
        assert_eq!(sig.get(1), Some(2.0));
    }

    #[test]
    fn test_signal_min_max() {
        let mut sig = WaveformSignal::new("test", SignalType::Voltage);
        sig.push(1.0);
        sig.push(5.0);
        sig.push(3.0);

        assert_eq!(sig.min(), Some(1.0));
        assert_eq!(sig.max(), Some(5.0));
    }

    // =========================================================================
    // WaveformDataset Tests
    // =========================================================================

    #[test]
    fn test_dataset_creation() {
        let dataset = WaveformDataset::new("Test Simulation");
        assert_eq!(dataset.title, "Test Simulation");
        assert!(dataset.signals.is_empty());
    }

    #[test]
    fn test_dataset_add_signal() {
        let mut dataset = WaveformDataset::new("Test");
        dataset.add_signal(WaveformSignal::new("v(out)", SignalType::Voltage));
        dataset.add_signal(WaveformSignal::new("i(vdd)", SignalType::Current));

        assert_eq!(dataset.signal_count(), 2);
        assert!(dataset.get_signal("v(out)").is_some());
    }

    #[test]
    fn test_dataset_signal_names() {
        let mut dataset = WaveformDataset::new("Test");
        dataset.add_signal(WaveformSignal::new("sig1", SignalType::Voltage));
        dataset.add_signal(WaveformSignal::new("sig2", SignalType::Current));

        let names = dataset.signal_names();
        assert!(names.contains(&"sig1"));
        assert!(names.contains(&"sig2"));
    }

    // =========================================================================
    // CSV I/O Tests
    // =========================================================================

    #[test]
    fn test_csv_roundtrip() {
        // Create dataset
        let mut dataset = WaveformDataset::new("Test");

        let mut time = WaveformSignal::new("time", SignalType::Time);
        time.data = vec![0.0, 1e-9, 2e-9, 3e-9];
        dataset.set_x(time);

        let mut vout = WaveformSignal::new("v(out)", SignalType::Voltage);
        vout.data = vec![0.0, 0.5, 1.0, 0.8];
        dataset.add_signal(vout);

        // Write
        let temp = NamedTempFile::new().unwrap();
        let writer = WaveformWriter::new(WaveformFormat::Csv);
        writer.write(&dataset, temp.path()).unwrap();

        // Read back
        let reader = WaveformReader::new(WaveformFormat::Csv);
        let loaded = reader.read(temp.path()).unwrap();

        assert_eq!(loaded.signal_count(), 1);
        assert_eq!(loaded.point_count(), 4);
    }

    #[test]
    fn test_read_csv_with_header() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "time,v(out),i(vdd)").unwrap();
        writeln!(temp, "0,0.0,1e-3").unwrap();
        writeln!(temp, "1e-9,0.5,0.5e-3").unwrap();
        writeln!(temp, "2e-9,1.0,0.2e-3").unwrap();

        let reader = WaveformReader::new(WaveformFormat::Csv);
        let dataset = reader.read(temp.path()).unwrap();

        assert_eq!(dataset.signal_count(), 2);
        assert_eq!(dataset.point_count(), 3);
    }

    // =========================================================================
    // NUTMEG Format Tests
    // =========================================================================

    #[test]
    fn test_read_nutmeg_basic() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "Title: Test Simulation").unwrap();
        writeln!(temp, "Plotname: Transient Analysis").unwrap();
        writeln!(temp, "No. Variables: 2").unwrap();
        writeln!(temp, "No. Points: 3").unwrap();
        writeln!(temp, "Variables:").unwrap();
        writeln!(temp, "\t0\ttime\ttime").unwrap();
        writeln!(temp, "\t1\tv(out)\tvoltage").unwrap();
        writeln!(temp, "Values:").unwrap();
        writeln!(temp, "0 0.0").unwrap();
        writeln!(temp, "1e-9 0.5").unwrap();
        writeln!(temp, "2e-9 1.0").unwrap();

        let reader = WaveformReader::new(WaveformFormat::Nutmeg);
        let dataset = reader.read(temp.path()).unwrap();

        assert_eq!(dataset.title, "Test Simulation");
        assert_eq!(dataset.analysis, "Transient Analysis");
    }

    #[test]
    fn test_write_touchstone_v1_two_port() {
        let mut dataset = WaveformDataset::new("sp");
        dataset.metadata.insert("z0".to_string(), "75".to_string());
        dataset
            .metadata
            .insert("touchstone_version".to_string(), "1".to_string());

        let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
        freq.data = vec![1.0e6, 2.0e6];
        dataset.set_x(freq);

        let mut s11_re = WaveformSignal::new("S11_RE", SignalType::SParameter);
        s11_re.data = vec![0.1, 0.2];
        let mut s11_im = WaveformSignal::new("S11_IM", SignalType::SParameter);
        s11_im.data = vec![0.01, 0.02];
        let mut s21_re = WaveformSignal::new("S21_RE", SignalType::SParameter);
        s21_re.data = vec![0.9, 0.8];
        let mut s21_im = WaveformSignal::new("S21_IM", SignalType::SParameter);
        s21_im.data = vec![0.0, -0.1];
        let mut s12_re = WaveformSignal::new("S12_RE", SignalType::SParameter);
        s12_re.data = vec![0.01, 0.02];
        let mut s12_im = WaveformSignal::new("S12_IM", SignalType::SParameter);
        s12_im.data = vec![0.0, 0.0];
        let mut s22_re = WaveformSignal::new("S22_RE", SignalType::SParameter);
        s22_re.data = vec![0.2, 0.3];
        let mut s22_im = WaveformSignal::new("S22_IM", SignalType::SParameter);
        s22_im.data = vec![-0.01, -0.02];

        dataset.add_signal(s11_re);
        dataset.add_signal(s11_im);
        dataset.add_signal(s21_re);
        dataset.add_signal(s21_im);
        dataset.add_signal(s12_re);
        dataset.add_signal(s12_im);
        dataset.add_signal(s22_re);
        dataset.add_signal(s22_im);

        let temp = NamedTempFile::new().unwrap();
        let writer = WaveformWriter::new(WaveformFormat::Touchstone);
        writer
            .write(&dataset, temp.path())
            .expect("touchstone write");

        let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
        assert!(content.contains("# Hz S RI R 75"));
        assert!(
            content
                .lines()
                .any(|line| line.contains("1.000000000000e6"))
        );
        assert!(
            content
                .lines()
                .any(|line| line.contains("2.000000000000e6"))
        );
        assert!(!content.contains("[Version] 2.0"));
    }

    #[test]
    fn test_write_touchstone_v2_two_port() {
        let mut dataset = WaveformDataset::new("spv2");
        dataset
            .metadata
            .insert("touchstone_version".to_string(), "2".to_string());

        let mut freq = WaveformSignal::new("freq", SignalType::Frequency);
        freq.data = vec![1.0e9];
        dataset.set_x(freq);

        for (name, value) in [
            ("S11_RE", 0.1),
            ("S11_IM", 0.0),
            ("S21_RE", 0.8),
            ("S21_IM", -0.1),
            ("S12_RE", 0.02),
            ("S12_IM", 0.0),
            ("S22_RE", 0.2),
            ("S22_IM", 0.01),
        ] {
            let mut signal = WaveformSignal::new(name, SignalType::SParameter);
            signal.data = vec![value];
            dataset.add_signal(signal);
        }

        let temp = NamedTempFile::new().unwrap();
        WaveformWriter::new(WaveformFormat::Touchstone)
            .write(&dataset, temp.path())
            .expect("touchstone v2 write");

        let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
        assert!(content.contains("[Version] 2.0"));
        assert!(content.contains("[Number of Ports] 2"));
        assert!(content.contains("[Network Data]"));
        assert!(content.contains("[End]"));
    }

    #[test]
    fn test_write_touchstone_v2_three_port() {
        let mut dataset = WaveformDataset::new("sp3");
        dataset
            .metadata
            .insert("touchstone_version".to_string(), "2".to_string());

        let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
        freq.data = vec![1.0e6, 2.0e6];
        dataset.set_x(freq);

        for row in 1..=3 {
            for col in 1..=3 {
                let base = format!("S{}_{}", row, col);
                let mut re = WaveformSignal::new(format!("{}_RE", base), SignalType::SParameter);
                let mut im = WaveformSignal::new(format!("{}_IM", base), SignalType::SParameter);
                re.data = vec![0.1 * row as f64, 0.2 * col as f64];
                im.data = vec![0.01 * col as f64, -0.02 * row as f64];
                dataset.add_signal(re);
                dataset.add_signal(im);
            }
        }

        let temp = NamedTempFile::new().unwrap();
        WaveformWriter::new(WaveformFormat::Touchstone)
            .write(&dataset, temp.path())
            .expect("touchstone 3-port write");

        let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
        assert!(content.contains("[Number of Ports] 3"));
        assert!(
            content
                .lines()
                .any(|line| line.starts_with("1.000000000000e6 "))
        );
    }

    #[test]
    fn test_write_touchstone_requires_complete_matrix_components() {
        let mut dataset = WaveformDataset::new("sp_missing");
        let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
        freq.data = vec![1.0e6];
        dataset.set_x(freq);

        // Intentionally missing S11_IM
        let mut s11_re = WaveformSignal::new("S11_RE", SignalType::SParameter);
        s11_re.data = vec![0.1];
        dataset.add_signal(s11_re);
        for (name, value) in [
            ("S21_RE", 0.8),
            ("S21_IM", -0.1),
            ("S12_RE", 0.02),
            ("S12_IM", 0.0),
            ("S22_RE", 0.2),
            ("S22_IM", 0.01),
        ] {
            let mut signal = WaveformSignal::new(name, SignalType::SParameter);
            signal.data = vec![value];
            dataset.add_signal(signal);
        }

        let writer = WaveformWriter::new(WaveformFormat::Touchstone);
        let err = writer
            .write(&dataset, Path::new("dummy.s2p"))
            .expect_err("missing matrix components should fail");
        assert!(err.contains("Missing Touchstone imag component for S11"));
    }

    #[test]
    fn test_read_psf_lite_roundtrip() {
        let temp = NamedTempFile::new().expect("temp file");
        let header = PsfHeader::new(3, 4);

        {
            let mut writer = PsfWriter::create(temp.path()).expect("psf writer");
            writer.write_header(&header).expect("header");
            writer.write_trace(&[0.0, 1.0, 2.0, 3.0]).expect("time");
            writer.write_trace(&[0.1, 0.2, 0.3, 0.4]).expect("trace 1");
            writer.write_trace(&[1.1, 1.2, 1.3, 1.4]).expect("trace 2");
        }

        let reader = WaveformReader::new(WaveformFormat::Psf);
        let dataset = reader.read(temp.path()).expect("PSF-Lite read should work");

        assert_eq!(dataset.analysis, "PSF-Lite");
        assert_eq!(dataset.point_count(), 4);
        assert_eq!(dataset.signal_count(), 2);
        assert_eq!(
            dataset
                .x_signal
                .as_ref()
                .and_then(|x| x.get(2))
                .unwrap_or(0.0),
            2.0
        );
        assert_eq!(dataset.signals[0].data[0], 0.1);
    }
}
