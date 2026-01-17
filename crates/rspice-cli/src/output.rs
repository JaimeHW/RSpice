//! Output formatters for simulation results
//!
//! These functions are kept for future CLI output format support.

#![allow(dead_code)]

use rspice_core::solver::SimulationResult;
use std::io::Write;
use std::path::Path;

/// Output format types
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum OutputFormat {
    /// SPICE raw format (binary)
    Raw,
    /// CSV text format
    Csv,
    /// JSON format
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "raw" => Some(OutputFormat::Raw),
            "csv" => Some(OutputFormat::Csv),
            "json" => Some(OutputFormat::Json),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Raw => "raw",
            OutputFormat::Csv => "csv",
            OutputFormat::Json => "json",
        }
    }
}

/// Write simulation results to a file
pub fn write_results(
    result: &SimulationResult,
    path: &Path,
    format: OutputFormat,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;

    match format {
        OutputFormat::Csv => write_csv(&mut file, result),
        OutputFormat::Json => write_json(&mut file, result),
        OutputFormat::Raw => write_raw(&mut file, result),
    }
}

fn write_csv<W: Write>(writer: &mut W, result: &SimulationResult) -> std::io::Result<()> {
    // Header
    if !result.time_points.is_empty() {
        write!(writer, "time")?;
        for i in 1..result.voltage_waveforms.len() {
            write!(writer, ",V({})", i)?;
        }
        writeln!(writer)?;

        // Data
        for (t_idx, &time) in result.time_points.iter().enumerate() {
            write!(writer, "{:.9e}", time)?;
            for waveform in result.voltage_waveforms.iter().skip(1) {
                if let Some(&v) = waveform.get(t_idx) {
                    write!(writer, ",{:.9e}", v)?;
                }
            }
            writeln!(writer)?;
        }
    } else {
        // DC operating point
        writeln!(writer, "node,voltage")?;
        for (i, &v) in result.node_voltages.iter().enumerate() {
            writeln!(writer, "{},{:.9e}", i, v)?;
        }
    }

    Ok(())
}

fn write_json<W: Write>(writer: &mut W, result: &SimulationResult) -> std::io::Result<()> {
    writeln!(writer, "{{")?;

    // Node voltages
    writeln!(writer, "  \"node_voltages\": [")?;
    for (i, &v) in result.node_voltages.iter().enumerate() {
        let comma = if i < result.node_voltages.len() - 1 {
            ","
        } else {
            ""
        };
        writeln!(writer, "    {:.9e}{}", v, comma)?;
    }
    writeln!(writer, "  ],")?;

    // Time points (if any)
    if !result.time_points.is_empty() {
        writeln!(writer, "  \"time_points\": [")?;
        for (i, &t) in result.time_points.iter().enumerate() {
            let comma = if i < result.time_points.len() - 1 {
                ","
            } else {
                ""
            };
            writeln!(writer, "    {:.9e}{}", t, comma)?;
        }
        writeln!(writer, "  ]")?;
    } else {
        writeln!(writer, "  \"time_points\": []")?;
    }

    writeln!(writer, "}}")?;

    Ok(())
}

fn write_raw<W: Write>(writer: &mut W, result: &SimulationResult) -> std::io::Result<()> {
    // SPICE raw file header
    writeln!(writer, "Title: RSpice Simulation")?;
    writeln!(writer, "Date: {}", chrono_lite())?;
    writeln!(writer, "Plotname: Transient Analysis")?;
    writeln!(writer, "Flags: real")?;
    writeln!(writer, "No. Variables: {}", result.node_voltages.len())?;
    writeln!(writer, "No. Points: {}", result.time_points.len().max(1))?;
    writeln!(writer, "Variables:")?;

    writeln!(writer, "\t0\ttime\ttime")?;
    for i in 1..result.node_voltages.len() {
        writeln!(writer, "\t{}\tV({})\tvoltage", i, i)?;
    }

    writeln!(writer, "Values:")?;

    if result.time_points.is_empty() {
        // DC operating point
        writeln!(writer, "0")?;
        writeln!(writer, "\t0.0")?;
        for &v in result.node_voltages.iter().skip(1) {
            writeln!(writer, "\t{:.9e}", v)?;
        }
    } else {
        // Transient data
        for (idx, &time) in result.time_points.iter().enumerate() {
            writeln!(writer, "{}", idx)?;
            writeln!(writer, "\t{:.9e}", time)?;
            for waveform in result.voltage_waveforms.iter().skip(1) {
                if let Some(&v) = waveform.get(idx) {
                    writeln!(writer, "\t{:.9e}", v)?;
                }
            }
        }
    }

    Ok(())
}

/// Simple date/time without external dependencies
fn chrono_lite() -> String {
    // Would use chrono in production
    "2024-01-01 00:00:00".to_string()
}
