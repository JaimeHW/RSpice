//! Waveform Export Functionality
//!
//! Commercial-grade export capabilities for waveform data including
//! CSV data export and image export.

#![allow(clippy::type_complexity)]

use std::io::Write;
use std::path::Path;

use crate::common::export_workflow::{ExportWorkflowIo, NativeExportWorkflowIo, SaveDialogConfig};

use super::state::TraceData;

// =============================================================================
// Export Configuration
// =============================================================================

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    /// Comma-separated values
    #[default]
    Csv,
    /// Tab-separated values
    Tsv,
    /// SPICE-compatible SXDAT format
    SpiceRaw,
}

impl ExportFormat {
    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Csv => "csv",
            ExportFormat::Tsv => "tsv",
            ExportFormat::SpiceRaw => "raw",
        }
    }

    /// Get delimiter for format
    pub fn delimiter(&self) -> char {
        match self {
            ExportFormat::Csv => ',',
            ExportFormat::Tsv => '\t',
            ExportFormat::SpiceRaw => ' ',
        }
    }
}

/// User-facing display name for an export format.
pub fn export_format_display_name(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Csv => "CSV",
        ExportFormat::Tsv => "TSV",
        ExportFormat::SpiceRaw => "SPICE RAW",
    }
}

/// Export options
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// Export format
    pub format: ExportFormat,
    /// Include header row
    pub include_header: bool,
    /// Decimal precision for values
    pub precision: usize,
    /// Scientific notation for small values
    pub scientific_notation: bool,
    /// Export hidden traces
    pub include_hidden: bool,
    /// X-axis start (None = from data start)
    pub x_start: Option<f64>,
    /// X-axis end (None = to data end)
    pub x_end: Option<f64>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Csv,
            include_header: true,
            precision: 9,
            scientific_notation: true,
            include_hidden: false,
            x_start: None,
            x_end: None,
        }
    }
}

// =============================================================================
// CSV Export
// =============================================================================

/// Export waveform data to CSV format
///
/// Returns the CSV content as a string.
pub fn export_to_csv(traces: &[TraceData], options: &ExportOptions) -> String {
    let delimiter = options.format.delimiter();
    let mut output = String::new();

    // Filter traces
    let visible_traces: Vec<&TraceData> = traces
        .iter()
        .filter(|t| options.include_hidden || t.visible)
        .collect();

    if visible_traces.is_empty() {
        return output;
    }

    // Header
    if options.include_header {
        output.push_str("Time");
        for trace in &visible_traces {
            output.push(delimiter);
            output.push_str(&trace.name);
        }
        output.push('\n');
    }

    // Find the trace with the most points for x-axis
    let _max_len = visible_traces.iter().map(|t| t.len()).max().unwrap_or(0);

    // Use the first visible trace as the time reference
    let Some(time_trace) = visible_traces.first() else {
        return output;
    };

    // Data rows
    for i in 0..time_trace.len() {
        let t = time_trace.x[i];

        // Apply X range filter
        if let Some(x_start) = options.x_start
            && t < x_start
        {
            continue;
        }
        if let Some(x_end) = options.x_end
            && t > x_end
        {
            continue;
        }

        // Time column
        output.push_str(&format_value(t, options));

        // Trace values
        for trace in &visible_traces {
            output.push(delimiter);
            if i < trace.len() {
                // Interpolate if this trace has the same time base
                let value = if (trace.x[i] - t).abs() < 1e-15 {
                    trace.y[i]
                } else {
                    trace.interpolate_at(t).unwrap_or(f64::NAN)
                };
                output.push_str(&format_value(value, options));
            } else {
                output.push_str("NaN");
            }
        }
        output.push('\n');
    }

    output
}

/// Format a value according to export options
fn format_value(value: f64, options: &ExportOptions) -> String {
    if !value.is_finite() {
        return "NaN".to_string();
    }

    if options.scientific_notation
        && value.abs() != 0.0
        && (value.abs() < 1e-3 || value.abs() >= 1e6)
    {
        format!("{:.prec$e}", value, prec = options.precision)
    } else {
        format!("{:.prec$}", value, prec = options.precision)
    }
}

/// Write CSV to file
pub fn write_csv_to_file(
    traces: &[TraceData],
    options: &ExportOptions,
    path: &Path,
) -> std::io::Result<()> {
    let content = export_to_csv(traces, options);
    let mut file = std::fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// =============================================================================
// SPICE Raw Format Export
// =============================================================================

/// Export to SPICE raw format (simplified version)
pub fn export_to_spice_raw(traces: &[TraceData], title: &str) -> String {
    let visible_traces: Vec<&TraceData> = traces.iter().filter(|t| t.visible).collect();

    if visible_traces.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let num_points = visible_traces.first().map(|t| t.len()).unwrap_or(0);
    let num_vars = visible_traces.len() + 1; // +1 for time

    // Header
    output.push_str(&format!("Title: {}\n", title));
    output.push_str("Date: \n");
    output.push_str("Plotname: Transient Analysis\n");
    output.push_str("Flags: real\n");
    output.push_str(&format!("No. Variables: {}\n", num_vars));
    output.push_str(&format!("No. Points: {}\n", num_points));

    // Variable definitions
    output.push_str("Variables:\n");
    output.push_str("\t0\ttime\ttime\n");
    for (i, trace) in visible_traces.iter().enumerate() {
        let var_type = if trace.name.starts_with("I(") {
            "current"
        } else {
            "voltage"
        };
        output.push_str(&format!("\t{}\t{}\t{}\n", i + 1, trace.name, var_type));
    }

    // Binary data marker (we use ASCII for simplicity)
    output.push_str("Values:\n");

    // Data
    let Some(time_trace) = visible_traces.first() else {
        return String::new();
    };
    for i in 0..time_trace.len() {
        output.push_str(&format!("{}\t{:.12e}", i, time_trace.x[i]));
        for trace in &visible_traces {
            if i < trace.len() {
                output.push_str(&format!("\t{:.12e}", trace.y[i]));
            } else {
                output.push_str("\t0.0");
            }
        }
        output.push('\n');
    }

    output
}

/// Build the export payload for the requested format.
pub fn build_export_payload(traces: &[TraceData], options: &ExportOptions) -> String {
    match options.format {
        ExportFormat::SpiceRaw => export_to_spice_raw(traces, "RSpice Waveforms"),
        ExportFormat::Csv | ExportFormat::Tsv => export_to_csv(traces, options),
    }
}

/// Save an export payload through the shared export workflow abstraction.
pub(crate) fn save_export_payload_with_io(
    payload: &str,
    format: ExportFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> Result<std::path::PathBuf, String> {
    let extension = format.extension();
    let Some(mut path) = io.show_save_dialog(SaveDialogConfig {
        title: "Export Waveforms",
        default_name: &format!("waveforms.{}", extension),
        filter_name: export_format_display_name(format),
        filter_extensions: &[extension],
    }) else {
        return Err("Export canceled".to_string());
    };
    crate::common::file_actions::ensure_file_extension(&mut path, extension);
    io.write_text_file(&path, payload)?;
    Ok(path)
}

/// Save an export payload using the production export workflow backend.
pub fn save_export_payload_with_native_dialog(
    payload: &str,
    format: ExportFormat,
) -> Result<std::path::PathBuf, String> {
    let io = NativeExportWorkflowIo;
    save_export_payload_with_io(payload, format, &io)
}

// =============================================================================
// Export Statistics
// =============================================================================

/// Statistics about an export
#[derive(Debug, Clone)]
pub struct ExportStats {
    /// Number of traces exported
    pub num_traces: usize,
    /// Number of data points per trace
    pub num_points: usize,
    /// Approximate file size in bytes
    pub estimated_size: usize,
}

/// Calculate export statistics
pub fn calculate_export_stats(traces: &[TraceData], options: &ExportOptions) -> ExportStats {
    let visible_traces: Vec<&TraceData> = traces
        .iter()
        .filter(|t| options.include_hidden || t.visible)
        .collect();

    let num_traces = visible_traces.len();
    let num_points = visible_traces.first().map(|t| t.len()).unwrap_or(0);

    // Estimate size: ~15 characters per value + delimiter + newline
    let values_per_row = num_traces + 1; // +1 for time
    let chars_per_row = values_per_row * (options.precision + 8) + num_traces + 1;
    let estimated_size = chars_per_row * num_points;

    ExportStats {
        num_traces,
        num_points,
        estimated_size,
    }
}

// =============================================================================
// Tests
// =============================================================================
