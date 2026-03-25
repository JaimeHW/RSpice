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
            && t < x_start {
                continue;
            }
        if let Some(x_end) = options.x_end
            && t > x_end {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::path::PathBuf;

    #[derive(Default)]
    struct MockExportWorkflowIo {
        save_dialog_results: RefCell<VecDeque<Option<PathBuf>>>,
        write_text_results: RefCell<VecDeque<Result<(), String>>>,
        save_dialog_calls: Cell<usize>,
        write_text_calls: Cell<usize>,
        last_save_dialog_config: RefCell<Option<(String, String, String, Vec<String>)>>,
        last_write_path: RefCell<Option<PathBuf>>,
        last_write_contents: RefCell<Option<String>>,
    }

    impl MockExportWorkflowIo {
        fn push_save_dialog_result(&self, result: Option<PathBuf>) {
            self.save_dialog_results.borrow_mut().push_back(result);
        }

        fn push_write_text_result(&self, result: Result<(), String>) {
            self.write_text_results.borrow_mut().push_back(result);
        }
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Option<PathBuf> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
            *self.last_save_dialog_config.borrow_mut() = Some((
                config.title.to_string(),
                config.default_name.to_string(),
                config.filter_name.to_string(),
                config
                    .filter_extensions
                    .iter()
                    .map(|ext| (*ext).to_string())
                    .collect(),
            ));
            self.save_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_save_dialog result")
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.write_text_calls
                .set(self.write_text_calls.get().saturating_add(1));
            *self.last_write_path.borrow_mut() = Some(path.to_path_buf());
            *self.last_write_contents.borrow_mut() = Some(contents.to_string());
            self.write_text_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide write_text_file result")
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Err("unexpected write_waveform_csv call".to_string())
        }
    }

    fn make_test_traces() -> Vec<TraceData> {
        vec![
            TraceData::new(
                "V(out)",
                vec![0.0, 1e-6, 2e-6, 3e-6],
                vec![0.0, 1.0, 2.0, 1.5],
            ),
            TraceData::new(
                "V(in)",
                vec![0.0, 1e-6, 2e-6, 3e-6],
                vec![1.0, 1.0, 1.0, 1.0],
            ),
        ]
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::Tsv.extension(), "tsv");
        assert_eq!(ExportFormat::SpiceRaw.extension(), "raw");
    }

    #[test]
    fn test_export_format_delimiter() {
        assert_eq!(ExportFormat::Csv.delimiter(), ',');
        assert_eq!(ExportFormat::Tsv.delimiter(), '\t');
    }

    #[test]
    fn test_export_options_default() {
        let opts = ExportOptions::default();
        assert_eq!(opts.format, ExportFormat::Csv);
        assert!(opts.include_header);
        assert!(!opts.include_hidden);
    }

    #[test]
    fn test_export_to_csv_basic() {
        let traces = make_test_traces();
        let options = ExportOptions::default();

        let csv = export_to_csv(&traces, &options);

        // Should have header
        assert!(csv.starts_with("Time,"));
        assert!(csv.contains("V(out)"));
        assert!(csv.contains("V(in)"));

        // Should have data rows
        let lines: Vec<&str> = csv.lines().collect();
        assert!(lines.len() > 1); // Header + at least one data row
    }

    #[test]
    fn test_export_to_csv_no_header() {
        let traces = make_test_traces();
        let mut options = ExportOptions::default();
        options.include_header = false;

        let csv = export_to_csv(&traces, &options);

        // Should not start with "Time"
        assert!(!csv.starts_with("Time"));
    }

    #[test]
    fn test_export_to_csv_tsv_format() {
        let traces = make_test_traces();
        let mut options = ExportOptions::default();
        options.format = ExportFormat::Tsv;

        let tsv = export_to_csv(&traces, &options);

        // Should use tabs as delimiter
        assert!(tsv.contains('\t'));
    }

    #[test]
    fn test_export_to_csv_hidden_traces() {
        let mut traces = make_test_traces();
        traces[1].visible = false;

        let options = ExportOptions::default();
        let csv = export_to_csv(&traces, &options);

        // Should not include hidden trace
        assert!(!csv.contains("V(in)"));
    }

    #[test]
    fn test_export_to_csv_include_hidden() {
        let mut traces = make_test_traces();
        traces[1].visible = false;

        let mut options = ExportOptions::default();
        options.include_hidden = true;

        let csv = export_to_csv(&traces, &options);

        // Should include hidden trace
        assert!(csv.contains("V(in)"));
    }

    #[test]
    fn test_export_to_csv_x_range() {
        let traces = make_test_traces();
        let mut options = ExportOptions::default();
        options.include_header = false;
        options.x_start = Some(1e-6);
        options.x_end = Some(2e-6);

        let csv = export_to_csv(&traces, &options);

        // Should only have 2 data rows (1e-6 and 2e-6)
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_format_value_normal() {
        let options = ExportOptions {
            precision: 3,
            scientific_notation: false,
            ..Default::default()
        };

        assert_eq!(format_value(1.5, &options), "1.500");
        assert_eq!(format_value(0.0, &options), "0.000");
    }

    #[test]
    fn test_format_value_scientific() {
        let options = ExportOptions {
            precision: 3,
            scientific_notation: true,
            ..Default::default()
        };

        // Small values should use scientific notation
        let result = format_value(1e-9, &options);
        assert!(result.contains('e'));

        // Normal values should not
        let result2 = format_value(1.5, &options);
        assert!(!result2.contains('e'));
    }

    #[test]
    fn test_format_value_nan() {
        let options = ExportOptions::default();
        assert_eq!(format_value(f64::NAN, &options), "NaN");
        assert_eq!(format_value(f64::INFINITY, &options), "NaN");
    }

    #[test]
    fn test_export_to_spice_raw() {
        let traces = make_test_traces();
        let raw = export_to_spice_raw(&traces, "Test Circuit");

        // Should have header
        assert!(raw.contains("Title: Test Circuit"));
        assert!(raw.contains("Plotname: Transient Analysis"));
        assert!(raw.contains("No. Variables:"));
        assert!(raw.contains("No. Points:"));

        // Should have variable definitions
        assert!(raw.contains("time"));
        assert!(raw.contains("V(out)"));

        // Should have data
        assert!(raw.contains("Values:"));
    }

    #[test]
    fn test_export_to_spice_raw_variable_types() {
        let traces = vec![
            TraceData::new("V(node)", vec![0.0], vec![0.0]),
            TraceData::new("I(r1)", vec![0.0], vec![0.0]),
        ];

        let raw = export_to_spice_raw(&traces, "Test");

        // Voltage signal should be marked as voltage
        assert!(raw.contains("voltage"));
        // Current signal should be marked as current
        assert!(raw.contains("current"));
    }

    #[test]
    fn test_calculate_export_stats() {
        let traces = make_test_traces();
        let options = ExportOptions::default();

        let stats = calculate_export_stats(&traces, &options);

        assert_eq!(stats.num_traces, 2);
        assert_eq!(stats.num_points, 4);
        assert!(stats.estimated_size > 0);
    }

    #[test]
    fn test_calculate_export_stats_with_hidden() {
        let mut traces = make_test_traces();
        traces[1].visible = false;

        let options = ExportOptions::default();
        let stats = calculate_export_stats(&traces, &options);

        assert_eq!(stats.num_traces, 1); // Only visible trace
    }

    #[test]
    fn test_export_empty_traces() {
        let traces: Vec<TraceData> = vec![];
        let options = ExportOptions::default();

        let csv = export_to_csv(&traces, &options);
        assert!(csv.is_empty());
    }

    #[test]
    fn test_export_all_hidden() {
        let mut traces = make_test_traces();
        traces[0].visible = false;
        traces[1].visible = false;

        let options = ExportOptions::default();
        let csv = export_to_csv(&traces, &options);

        assert!(csv.is_empty());
    }

    #[test]
    fn test_build_export_payload_routes_by_format() {
        let traces = vec![TraceData::new("V(out)", vec![0.0, 1e-6], vec![0.0, 1.0])];

        let mut csv_opts = ExportOptions::default();
        csv_opts.format = ExportFormat::Csv;
        let csv = build_export_payload(&traces, &csv_opts);
        assert!(csv.contains("Time,"));

        let mut tsv_opts = ExportOptions::default();
        tsv_opts.format = ExportFormat::Tsv;
        let tsv = build_export_payload(&traces, &tsv_opts);
        assert!(tsv.contains('\t'));

        let mut raw_opts = ExportOptions::default();
        raw_opts.format = ExportFormat::SpiceRaw;
        let raw = build_export_payload(&traces, &raw_opts);
        assert!(raw.contains("Title: RSpice Waveforms"));
        assert!(raw.contains("Values:"));
    }

    #[test]
    fn test_save_export_payload_with_io_uses_shared_dialog_contract() {
        let io = MockExportWorkflowIo::default();
        io.push_save_dialog_result(Some(PathBuf::from("exports/waveform_dump")));
        io.push_write_text_result(Ok(()));

        let path = save_export_payload_with_io("payload", ExportFormat::Csv, &io)
            .expect("save should succeed");

        assert_eq!(path, PathBuf::from("exports/waveform_dump.csv"));
        assert_eq!(io.save_dialog_calls.get(), 1);
        assert_eq!(io.write_text_calls.get(), 1);
        assert_eq!(
            io.last_save_dialog_config.borrow().clone(),
            Some((
                "Export Waveforms".to_string(),
                "waveforms.csv".to_string(),
                "CSV".to_string(),
                vec!["csv".to_string()],
            ))
        );
        assert_eq!(
            io.last_write_path.borrow().clone(),
            Some(PathBuf::from("exports/waveform_dump.csv"))
        );
        assert_eq!(
            io.last_write_contents.borrow().clone(),
            Some("payload".to_string())
        );
    }

    #[test]
    fn test_save_export_payload_with_io_propagates_cancel() {
        let io = MockExportWorkflowIo::default();
        io.push_save_dialog_result(None);

        let err = save_export_payload_with_io("payload", ExportFormat::SpiceRaw, &io)
            .expect_err("cancellation should be returned");

        assert_eq!(err, "Export canceled");
        assert_eq!(io.save_dialog_calls.get(), 1);
        assert_eq!(io.write_text_calls.get(), 0);
    }

    #[test]
    fn test_save_export_payload_with_io_propagates_write_errors() {
        let io = MockExportWorkflowIo::default();
        io.push_save_dialog_result(Some(PathBuf::from("exports/fail.raw")));
        io.push_write_text_result(Err("disk full".to_string()));

        let err = save_export_payload_with_io("payload", ExportFormat::SpiceRaw, &io)
            .expect_err("write errors should surface");

        assert_eq!(err, "disk full");
        assert_eq!(io.write_text_calls.get(), 1);
    }
}
