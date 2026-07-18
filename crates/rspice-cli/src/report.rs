//! CI/CD Report Generation
//!
//! Provides machine-readable output formats for automation:
//! - JUnit XML for test framework integration
//! - TAP (Test Anything Protocol) for streaming output
//! - JSON/CSV for measurement results

use crate::cli::CliError;
use std::fmt;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Simulation result for reporting
#[derive(Debug, Clone)]
pub struct SimulationReport {
    /// Test/simulation name
    pub name: String,
    /// Netlist file path
    pub netlist: String,
    /// Whether simulation passed
    pub passed: bool,
    /// Execution time in seconds
    pub duration_secs: f64,
    /// Error message if failed
    pub error: Option<String>,
    /// Stable error metadata if the simulation failed.
    pub error_details: Option<crate::cli::ErrorDetails>,
    /// Measurements (name, value, passed)
    pub measurements: Vec<MeasurementReport>,
}

/// Format like C's `%e` (ngspice's style): six fractional digits and a
/// signed, zero-padded, at-least-two-digit exponent.
///
/// Measurement values must use this rather than fixed-point `{:.6}`, which
/// rounds SPICE-scale magnitudes (e.g. a 219.8ns risetime) to `0.000000`.
pub(crate) fn format_spice_exponent(value: f64) -> String {
    let formatted = format!("{value:.6e}");
    match formatted.split_once('e') {
        Some((mantissa, exponent)) => {
            let (sign, digits) = match exponent.strip_prefix('-') {
                Some(rest) => ('-', rest),
                None => ('+', exponent),
            };
            format!("{mantissa}e{sign}{digits:0>2}")
        }
        None => formatted,
    }
}

/// Measurement result for reporting
#[derive(Debug, Clone)]
pub struct MeasurementReport {
    /// Measurement name
    pub name: String,
    /// Measured value
    pub value: Option<f64>,
    /// Expected value (for comparison)
    pub expected: Option<f64>,
    /// Tolerance for comparison
    pub tolerance: Option<f64>,
    /// Whether measurement passed
    pub passed: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// JUnit XML report writer
pub struct JUnitReporter;

impl JUnitReporter {
    /// Write JUnit XML report to file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
        let mut writer = BufWriter::new(file);
        write_junit_report(&mut writer, path, reports)
    }
}

fn write_junit_report<W: Write>(
    writer: &mut W,
    path: &Path,
    reports: &[SimulationReport],
) -> Result<(), CliError> {
    let total_tests: usize = reports.iter().map(|r| 1 + r.measurements.len()).sum();
    let failures: usize = reports
        .iter()
        .map(|r| {
            usize::from(run_failure_message(r).is_some())
                + r.measurements.iter().filter(|m| !m.passed).count()
        })
        .sum();
    let total_time: f64 = reports.iter().map(|r| r.duration_secs).sum();

    write_line(
        writer,
        path,
        format_args!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"),
    )?;
    write_line(
        writer,
        path,
        format_args!(
            "<testsuites tests=\"{}\" failures=\"{}\" time=\"{:.3}\">",
            total_tests, failures, total_time
        ),
    )?;

    for report in reports {
        let test_count = 1 + report.measurements.len();
        let run_failure = run_failure_message(report);
        let failure_count = usize::from(run_failure.is_some())
            + report.measurements.iter().filter(|m| !m.passed).count();

        write_line(
            writer,
            path,
            format_args!(
                "  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" time=\"{:.3}\">",
                xml_escape(&report.name),
                test_count,
                failure_count,
                report.duration_secs
            ),
        )?;

        // Main simulation test case
        write_line(
            writer,
            path,
            format_args!(
                "    <testcase name=\"simulation\" classname=\"{}\" time=\"{:.3}\">",
                xml_escape(&report.netlist),
                report.duration_secs
            ),
        )?;

        if let Some(ref message) = run_failure {
            write_line(
                writer,
                path,
                format_args!(
                    "      <failure message=\"Run verification failed\">{}</failure>",
                    xml_escape(message)
                ),
            )?;
        }
        write_line(writer, path, format_args!("    </testcase>"))?;

        // Measurement test cases
        for meas in &report.measurements {
            write_line(
                writer,
                path,
                format_args!(
                    "    <testcase name=\"{}\" classname=\"{}\">",
                    xml_escape(&meas.name),
                    xml_escape(&report.netlist)
                ),
            )?;

            if !meas.passed
                && let Some(ref err) = meas.error
            {
                write_line(
                    writer,
                    path,
                    format_args!(
                        "      <failure message=\"Measurement failed\">{}</failure>",
                        xml_escape(err)
                    ),
                )?;
            }
            write_line(writer, path, format_args!("    </testcase>"))?;
        }

        write_line(writer, path, format_args!("  </testsuite>"))?;
    }

    write_line(writer, path, format_args!("</testsuites>"))?;
    Ok(())
}

/// TAP (Test Anything Protocol) report writer
pub struct TapReporter;

impl TapReporter {
    /// Write TAP report to file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
        let mut writer = BufWriter::new(file);
        write_tap_report(&mut writer, path, reports)
    }
}

fn write_tap_report<W: Write>(
    writer: &mut W,
    path: &Path,
    reports: &[SimulationReport],
) -> Result<(), CliError> {
    let total_tests: usize = reports.iter().map(|r| 1 + r.measurements.len()).sum();
    write_line(writer, path, format_args!("TAP version 13"))?;
    write_line(writer, path, format_args!("1..{}", total_tests))?;

    let mut test_num = 0;
    for report in reports {
        test_num += 1;
        if let Some(message) = run_failure_message(report) {
            write_line(
                writer,
                path,
                format_args!("not ok {} - {}", test_num, report.name),
            )?;
            write_line(writer, path, format_args!("  ---"))?;
            write_line(writer, path, format_args!("  message: '{}'", message))?;
            write_line(writer, path, format_args!("  ..."))?;
        } else {
            write_line(
                writer,
                path,
                format_args!("ok {} - {}", test_num, report.name),
            )?;
        }

        for meas in &report.measurements {
            test_num += 1;
            if meas.passed {
                let value_str = meas
                    .value
                    .map(|v| format!(" = {}", format_spice_exponent(v)))
                    .unwrap_or_default();
                write_line(
                    writer,
                    path,
                    format_args!("ok {} - {}{}", test_num, meas.name, value_str),
                )?;
            } else {
                write_line(
                    writer,
                    path,
                    format_args!("not ok {} - {}", test_num, meas.name),
                )?;
                if let Some(ref err) = meas.error {
                    write_line(writer, path, format_args!("  ---"))?;
                    write_line(writer, path, format_args!("  message: '{}'", err))?;
                    write_line(writer, path, format_args!("  ..."))?;
                }
            }
        }
    }

    Ok(())
}

fn run_failure_message(report: &SimulationReport) -> Option<String> {
    if report.passed && report.error.is_none() {
        return None;
    }

    if let Some(err) = &report.error {
        return Some(format!("Simulation failed: {err}"));
    }

    let failed_measurements: Vec<&str> = report
        .measurements
        .iter()
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    if failed_measurements.is_empty() {
        Some("Run verification failed".to_string())
    } else {
        Some(format!(
            "Run verification failed: {} measurement(s) failed: {}",
            failed_measurements.len(),
            failed_measurements.join(", ")
        ))
    }
}

/// JSON report for measurements
pub struct JsonMeasReporter;

impl JsonMeasReporter {
    /// Write measurement results to JSON file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
        let mut writer = BufWriter::new(file);
        write_measurement_json(&mut writer, path, reports)
    }
}

fn write_measurement_json<W: Write>(
    writer: &mut W,
    path: &Path,
    reports: &[SimulationReport],
) -> Result<(), CliError> {
    let mut results = Vec::new();

    for report in reports {
        for meas in &report.measurements {
            results.push(serde_json::json!({
                "netlist": report.netlist,
                "name": meas.name,
                "value": meas.value,
                "expected": meas.expected,
                "tolerance": meas.tolerance,
                "passed": meas.passed,
                "error": meas.error,
            }));
        }
    }

    let json = serde_json::json!({
        "measurements": results,
        "total": results.len(),
        "passed": results.iter().filter(|r| r["passed"] == true).count(),
        "failed": results.iter().filter(|r| r["passed"] == false).count(),
    });

    serde_json::to_writer_pretty(&mut *writer, &json)
        .map_err(|e| CliError::output_json_error(path, e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| CliError::output_error(path, e))?;
    Ok(())
}

/// CSV report for measurements
pub struct CsvMeasReporter;

impl CsvMeasReporter {
    /// Write measurement results to CSV file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
        let mut writer = BufWriter::new(file);

        write_line(
            &mut writer,
            path,
            format_args!("netlist,name,value,expected,tolerance,passed,error"),
        )?;

        for report in reports {
            for meas in &report.measurements {
                write_line(
                    &mut writer,
                    path,
                    format_args!(
                        "{},{},{},{},{},{},{}",
                        csv_escape(&report.netlist),
                        csv_escape(&meas.name),
                        meas.value.map(|v| format!("{:.9e}", v)).unwrap_or_default(),
                        meas.expected
                            .map(|v| format!("{:.9e}", v))
                            .unwrap_or_default(),
                        meas.tolerance
                            .map(|v| format!("{:.9e}", v))
                            .unwrap_or_default(),
                        meas.passed,
                        csv_escape(meas.error.as_deref().unwrap_or("")),
                    ),
                )?;
            }
        }

        Ok(())
    }
}

/// Quote a CSV field if it contains separators, quotes, or newlines
fn csv_escape(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn write_line<W: Write>(
    writer: &mut W,
    path: &Path,
    args: fmt::Arguments<'_>,
) -> Result<(), CliError> {
    writer
        .write_fmt(args)
        .map_err(|e| CliError::output_error(path, e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| CliError::output_error(path, e))
}

/// Escape XML special characters
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spice_exponent_preserves_sub_microsecond_magnitudes() {
        assert_eq!(format_spice_exponent(2.198108e-7), "2.198108e-07");
        assert_eq!(format_spice_exponent(-3.5e-12), "-3.500000e-12");
        assert_eq!(format_spice_exponent(0.0), "0.000000e+00");
        assert_eq!(format_spice_exponent(4.999773), "4.999773e+00");
        assert_eq!(format_spice_exponent(1.5e123), "1.500000e+123");
    }

    fn report_with_measurement(value: Option<f64>) -> SimulationReport {
        SimulationReport {
            name: "rc".into(),
            netlist: "rc.sp".into(),
            passed: true,
            duration_secs: 0.01,
            error: None,
            error_details: None,
            measurements: vec![MeasurementReport {
                name: "risetime".into(),
                value,
                expected: None,
                tolerance: None,
                passed: value.is_some(),
                error: None,
            }],
        }
    }

    #[test]
    fn tap_report_keeps_sub_microsecond_measurement_visible() {
        let reports = [report_with_measurement(Some(2.198108e-7))];
        let mut buf = Vec::new();
        write_tap_report(&mut buf, Path::new("test.tap"), &reports).expect("write tap");
        let tap = String::from_utf8(buf).expect("utf8 tap");
        assert!(
            tap.contains("ok 2 - risetime = 2.198108e-07"),
            "TAP output should carry the scientific-notation value: {tap}"
        );
        assert!(
            !tap.contains("0.000000"),
            "TAP output must not round the measurement to zero: {tap}"
        );
    }
}
