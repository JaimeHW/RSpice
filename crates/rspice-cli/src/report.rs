//! CI/CD Report Generation
//!
//! Provides machine-readable output formats for automation:
//! - JUnit XML for test framework integration
//! - TAP (Test Anything Protocol) for streaming output
//! - JSON/CSV for measurement results

use crate::cli::CliError;
use std::io::Write;
use std::path::Path;

/// Report format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// JUnit XML format (compatible with CI systems)
    JUnit,
    /// TAP (Test Anything Protocol)
    Tap,
    /// JSON format
    Json,
}

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
    /// Measurements (name, value, passed)
    pub measurements: Vec<MeasurementReport>,
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

impl MeasurementReport {
    /// Create a passed measurement
    pub fn passed(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: Some(value),
            expected: None,
            tolerance: None,
            passed: true,
            error: None,
        }
    }

    /// Create a failed measurement
    pub fn failed(name: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
            expected: None,
            tolerance: None,
            passed: false,
            error: Some(error.into()),
        }
    }

    /// Create a measurement with expected value comparison
    pub fn with_expected(
        name: impl Into<String>,
        value: f64,
        expected: f64,
        tolerance: f64,
    ) -> Self {
        let passed = (value - expected).abs() <= tolerance;
        Self {
            name: name.into(),
            value: Some(value),
            expected: Some(expected),
            tolerance: Some(tolerance),
            passed,
            error: if passed {
                None
            } else {
                Some(format!(
                    "Value {} differs from expected {} by more than tolerance {}",
                    value, expected, tolerance
                ))
            },
        }
    }
}

/// JUnit XML report writer
pub struct JUnitReporter;

impl JUnitReporter {
    /// Write JUnit XML report to file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let total_tests: usize = reports.iter().map(|r| 1 + r.measurements.len()).sum();
        let failures: usize = reports
            .iter()
            .map(|r| {
                (if r.passed { 0 } else { 1 }) + r.measurements.iter().filter(|m| !m.passed).count()
            })
            .sum();
        let total_time: f64 = reports.iter().map(|r| r.duration_secs).sum();

        writeln!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>").unwrap();
        writeln!(
            file,
            "<testsuites tests=\"{}\" failures=\"{}\" time=\"{:.3}\">",
            total_tests, failures, total_time
        )
        .unwrap();

        for report in reports {
            let test_count = 1 + report.measurements.len();
            let failure_count = (if report.passed { 0 } else { 1 })
                + report.measurements.iter().filter(|m| !m.passed).count();

            writeln!(
                file,
                "  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" time=\"{:.3}\">",
                xml_escape(&report.name),
                test_count,
                failure_count,
                report.duration_secs
            )
            .unwrap();

            // Main simulation test case
            writeln!(
                file,
                "    <testcase name=\"simulation\" classname=\"{}\" time=\"{:.3}\">",
                xml_escape(&report.netlist),
                report.duration_secs
            )
            .unwrap();

            if !report.passed {
                if let Some(ref err) = report.error {
                    writeln!(
                        file,
                        "      <failure message=\"Simulation failed\">{}</failure>",
                        xml_escape(err)
                    )
                    .unwrap();
                }
            }
            writeln!(file, "    </testcase>").unwrap();

            // Measurement test cases
            for meas in &report.measurements {
                writeln!(
                    file,
                    "    <testcase name=\"{}\" classname=\"{}\">",
                    xml_escape(&meas.name),
                    xml_escape(&report.netlist)
                )
                .unwrap();

                if !meas.passed {
                    if let Some(ref err) = meas.error {
                        writeln!(
                            file,
                            "      <failure message=\"Measurement failed\">{}</failure>",
                            xml_escape(err)
                        )
                        .unwrap();
                    }
                }
                writeln!(file, "    </testcase>").unwrap();
            }

            writeln!(file, "  </testsuite>").unwrap();
        }

        writeln!(file, "</testsuites>").unwrap();
        Ok(())
    }
}

/// TAP (Test Anything Protocol) report writer
pub struct TapReporter;

impl TapReporter {
    /// Write TAP report to file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
        let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let total_tests: usize = reports.iter().map(|r| 1 + r.measurements.len()).sum();
        writeln!(file, "TAP version 13").unwrap();
        writeln!(file, "1..{}", total_tests).unwrap();

        let mut test_num = 0;
        for report in reports {
            test_num += 1;
            if report.passed {
                writeln!(file, "ok {} - {}", test_num, report.name).unwrap();
            } else {
                writeln!(file, "not ok {} - {}", test_num, report.name).unwrap();
                if let Some(ref err) = report.error {
                    writeln!(file, "  ---").unwrap();
                    writeln!(file, "  message: '{}'", err).unwrap();
                    writeln!(file, "  ...").unwrap();
                }
            }

            for meas in &report.measurements {
                test_num += 1;
                if meas.passed {
                    let value_str = meas
                        .value
                        .map(|v| format!(" = {:.6}", v))
                        .unwrap_or_default();
                    writeln!(file, "ok {} - {}{}", test_num, meas.name, value_str).unwrap();
                } else {
                    writeln!(file, "not ok {} - {}", test_num, meas.name).unwrap();
                    if let Some(ref err) = meas.error {
                        writeln!(file, "  ---").unwrap();
                        writeln!(file, "  message: '{}'", err).unwrap();
                        writeln!(file, "  ...").unwrap();
                    }
                }
            }
        }

        Ok(())
    }
}

/// JSON report for measurements
pub struct JsonMeasReporter;

impl JsonMeasReporter {
    /// Write measurement results to JSON file
    pub fn write(reports: &[SimulationReport], path: &Path) -> Result<(), CliError> {
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

        let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
            path: path.to_path_buf(),
            source: e,
        })?;
        writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).unwrap();

        Ok(())
    }
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
    use tempfile::tempdir;

    #[test]
    fn test_measurement_passed() {
        let m = MeasurementReport::passed("VDD", 1.8);
        assert!(m.passed);
        assert_eq!(m.value, Some(1.8));
    }

    #[test]
    fn test_measurement_with_expected() {
        let m = MeasurementReport::with_expected("VDD", 1.79, 1.8, 0.05);
        assert!(m.passed);

        let m2 = MeasurementReport::with_expected("VDD", 1.5, 1.8, 0.05);
        assert!(!m2.passed);
    }

    #[test]
    fn test_junit_report() {
        let reports = vec![SimulationReport {
            name: "test_sim".to_string(),
            netlist: "test.sp".to_string(),
            passed: true,
            duration_secs: 0.5,
            error: None,
            measurements: vec![MeasurementReport::passed("VDD", 1.8)],
        }];

        let dir = tempdir().unwrap();
        let path = dir.path().join("report.xml");
        JUnitReporter::write(&reports, &path).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("testsuites"));
        assert!(content.contains("test_sim"));
    }

    #[test]
    fn test_tap_report() {
        let reports = vec![SimulationReport {
            name: "test_sim".to_string(),
            netlist: "test.sp".to_string(),
            passed: true,
            duration_secs: 0.5,
            error: None,
            measurements: vec![MeasurementReport::passed("VDD", 1.8)],
        }];

        let dir = tempdir().unwrap();
        let path = dir.path().join("report.tap");
        TapReporter::write(&reports, &path).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("TAP version 13"));
        assert!(content.contains("ok 1"));
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("<test>"), "&lt;test&gt;");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
    }
}
