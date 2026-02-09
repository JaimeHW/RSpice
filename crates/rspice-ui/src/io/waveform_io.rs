//! Waveform I/O
//!
//! Read and write waveform data in various formats.
//! Supports interchange formats used by commercial simulators.
//!
//! # Supported Formats
//!
//! - NUTMEG (SPICE3/ngspice raw format) for import
//! - CSV and TSV for import/export
//!
//! # Planned Formats
//!
//! - PSF (Parameter Storage Format) - Cadence
//! - Touchstone S-parameter format

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
        match ext.to_lowercase().as_str() {
            "psf" => Some(WaveformFormat::Psf),
            "raw" | "tr0" | "ac0" => Some(WaveformFormat::Nutmeg),
            "csv" => Some(WaveformFormat::Csv),
            "tsv" => Some(WaveformFormat::Tsv),
            "s1p" | "s2p" | "snp" => Some(WaveformFormat::Touchstone),
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
            WaveformFormat::Csv
                | WaveformFormat::Tsv
                | WaveformFormat::Nutmeg
                | WaveformFormat::AsciiRaw
        )
    }

    /// Whether this format currently supports writing.
    pub fn can_write(&self) -> bool {
        matches!(self, WaveformFormat::Csv | WaveformFormat::Tsv)
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
            WaveformFormat::Csv => self.read_csv(path),
            WaveformFormat::Tsv => self.read_tsv(path),
            WaveformFormat::Nutmeg | WaveformFormat::AsciiRaw => self.read_nutmeg(path),
            _ => Err(format!(
                "Format {:?} read is not implemented (supported: Csv, Tsv, Nutmeg/AsciiRaw)",
                self.format
            )),
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
            _ => Err(format!(
                "Format {:?} write is not implemented (supported: Csv, Tsv)",
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
    use std::io::Write;
    use std::path::Path;
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
        assert_eq!(WaveformFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_format_capabilities() {
        assert!(WaveformFormat::Csv.can_read());
        assert!(WaveformFormat::Csv.can_write());
        assert!(WaveformFormat::Nutmeg.can_read());
        assert!(!WaveformFormat::Nutmeg.can_write());
        assert!(!WaveformFormat::Psf.can_read());
        assert!(!WaveformFormat::Touchstone.can_write());
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
    fn test_reader_reports_unsupported_format() {
        let reader = WaveformReader::new(WaveformFormat::Psf);
        let err = reader
            .read(Path::new("dummy.psf"))
            .expect_err("PSF read should be unsupported");
        assert!(err.contains("not implemented"));
        assert!(err.contains("supported"));
    }
}
