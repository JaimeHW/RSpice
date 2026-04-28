//! Raw Waveform Export
//!
//! Provides standard .raw file export for simulation results.
//! Supports both binary (fast, compact) and ASCII (human-readable) formats.
//!
//! # Format Specification
//! .raw files have a header followed by data:
//! ```text
//! Title: <simulation name>
//! Date: <date>
//! Plotname: Transient Analysis
//! Flags: real
//! No. Variables: <N>
//! No. Points: <M>
//! Variables:
//!     0   time    time
//!     1   V(1)    voltage
//!     ...
//! Binary: / Values:
//! <data>
//! ```

use crate::Value;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

/// Export format for raw files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawFormat {
    /// Binary format (compact, fast to load)
    Binary,
    /// ASCII format (human-readable)
    Ascii,
}

/// Variable type in raw file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Time,
    Frequency,
    Voltage,
    Current,
}

impl VariableType {
    fn as_str(&self) -> &'static str {
        match self {
            VariableType::Time => "time",
            VariableType::Frequency => "frequency",
            VariableType::Voltage => "voltage",
            VariableType::Current => "current",
        }
    }
}

/// A variable (signal) in the raw file
#[derive(Debug, Clone)]
pub struct RawVariable {
    /// Variable name (e.g., "V(1)", "I(R1)")
    pub name: String,
    /// Variable type
    pub var_type: VariableType,
}

impl RawVariable {
    /// Create a time variable
    pub fn time() -> Self {
        Self {
            name: "time".to_string(),
            var_type: VariableType::Time,
        }
    }

    /// Create a frequency variable
    pub fn frequency() -> Self {
        Self {
            name: "frequency".to_string(),
            var_type: VariableType::Frequency,
        }
    }

    /// Create a voltage variable
    pub fn voltage(node: &str) -> Self {
        Self {
            name: format!("V({})", node),
            var_type: VariableType::Voltage,
        }
    }

    /// Create a current variable
    pub fn current(element: &str) -> Self {
        Self {
            name: format!("I({})", element),
            var_type: VariableType::Current,
        }
    }
}

/// Raw file writer for exporting simulation results
pub struct RawExporter {
    /// Title of the simulation
    title: String,
    /// Plot name (e.g., "Transient Analysis")
    plot_name: String,
    /// Variables in the file
    variables: Vec<RawVariable>,
    /// Data points (time/freq values in column 0, then signals)
    data: Vec<Vec<Value>>,
}

impl RawExporter {
    /// Create a new raw exporter for transient analysis
    pub fn new_transient(title: &str) -> Self {
        Self {
            title: title.to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![RawVariable::time()],
            data: Vec::new(),
        }
    }

    /// Create a new raw exporter for AC analysis
    pub fn new_ac(title: &str) -> Self {
        Self {
            title: title.to_string(),
            plot_name: "AC Analysis".to_string(),
            variables: vec![RawVariable::frequency()],
            data: Vec::new(),
        }
    }

    /// Add a voltage variable
    pub fn add_voltage(&mut self, node: &str) {
        self.variables.push(RawVariable::voltage(node));
    }

    /// Add a current variable
    pub fn add_current(&mut self, element: &str) {
        self.variables.push(RawVariable::current(element));
    }

    /// Add a data point (all values for one time/frequency point)
    /// First value should be time/frequency, rest are signal values
    pub fn add_point(&mut self, values: Vec<Value>) {
        if values.len() == self.variables.len() {
            self.data.push(values);
        }
    }

    /// Add data from transient result
    pub fn add_transient_data(&mut self, times: &[Value], waveforms: &[Vec<Value>]) {
        for (i, &t) in times.iter().enumerate() {
            let mut point = vec![t];
            for waveform in waveforms {
                point.push(waveform.get(i).copied().unwrap_or(0.0));
            }
            self.data.push(point);
        }
    }

    /// Number of data points
    pub fn num_points(&self) -> usize {
        self.data.len()
    }

    /// Number of variables
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }

    /// Write to file in specified format
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, format: RawFormat) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.write(&mut writer, format)
    }

    /// Write to any writer
    pub fn write<W: Write>(&self, writer: &mut W, format: RawFormat) -> io::Result<()> {
        // Write header
        writeln!(writer, "Title: {}", self.title)?;
        writeln!(writer, "Date: {}", chrono_date())?;
        writeln!(writer, "Plotname: {}", self.plot_name)?;
        writeln!(writer, "Flags: real")?;
        writeln!(writer, "No. Variables: {}", self.variables.len())?;
        writeln!(writer, "No. Points: {}", self.data.len())?;
        writeln!(writer, "Variables:")?;

        for (i, var) in self.variables.iter().enumerate() {
            writeln!(writer, "\t{}\t{}\t{}", i, var.name, var.var_type.as_str())?;
        }

        match format {
            RawFormat::Binary => {
                writeln!(writer, "Binary:")?;
                for point in &self.data {
                    for &value in point {
                        writer.write_all(&value.to_le_bytes())?;
                    }
                }
            }
            RawFormat::Ascii => {
                writeln!(writer, "Values:")?;
                for (i, point) in self.data.iter().enumerate() {
                    write!(writer, "{}", i)?;
                    for &value in point {
                        write!(writer, "\t{:.15e}", value)?;
                    }
                    writeln!(writer)?;
                }
            }
        }

        Ok(())
    }

    /// Write to string in ASCII format
    pub fn to_ascii_string(&self) -> String {
        let mut buffer = Vec::new();
        self.write(&mut buffer, RawFormat::Ascii).unwrap();
        String::from_utf8_lossy(&buffer).into_owned()
    }
}

/// Get current date/time string (simplified, no chrono dependency)
fn chrono_date() -> String {
    // Simple timestamp without external dependency
    "Generated by RSpice".to_string()
}

//=============================================================================
// Convenience functions for engine integration
//=============================================================================

/// Export transient results to raw file
pub fn export_transient<P: AsRef<Path>>(
    path: P,
    times: &[Value],
    node_names: &[String],
    waveforms: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    let mut exporter = RawExporter::new_transient("Transient Analysis");

    for name in node_names {
        exporter.add_voltage(name);
    }

    exporter.add_transient_data(times, waveforms);
    exporter.write_to_file(path, format)
}

/// Export DC sweep results to raw file  
pub fn export_dc_sweep<P: AsRef<Path>>(
    path: P,
    sweep_values: &[Value],
    sweep_name: &str,
    node_names: &[String],
    results: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    let mut exporter = RawExporter {
        title: "DC Sweep".to_string(),
        plot_name: "DC transfer characteristic".to_string(),
        variables: vec![RawVariable {
            name: sweep_name.to_string(),
            var_type: VariableType::Voltage,
        }],
        data: Vec::new(),
    };

    for name in node_names {
        exporter.add_voltage(name);
    }

    for (i, &sweep_val) in sweep_values.iter().enumerate() {
        let mut point = vec![sweep_val];
        for result in results {
            point.push(result.get(i).copied().unwrap_or(0.0));
        }
        exporter.data.push(point);
    }

    exporter.write_to_file(path, format)
}

//=============================================================================
// Tests
//=============================================================================
