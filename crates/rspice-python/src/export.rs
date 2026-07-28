//! Result serialization: Touchstone, SPICE raw, and CSV.
//!
//! Touchstone and CSV output is a pure function of the result, so those
//! artifacts can be diffed in a regression suite directly. A raw file carries
//! an ngspice-style `Date:` header, so callers who need byte-reproducible raw
//! output pin it through the `timestamp` argument.
//! Each format is offered both as an in-memory string (or byte vector) and as
//! a file write, because CI jobs frequently need the text without touching a
//! filesystem.
//!
//! # Touchstone
//!
//! Re-exported from `rspice_core`, which owns the single writer every
//! front-end shares. It stays visible through this module so the result types
//! reach all three formats through one path.
//!
//! # SPICE raw
//!
//! ngspice-compatible ASCII and binary raw files, including `Flags: complex`
//! for frequency-domain data, which the shared core exporter does not model.
//!
//! Nothing here touches the Python interpreter: every entry point returns a
//! plain `Result<_, String>` that the binding layer maps to an exception. That
//! keeps the format logic unit-testable without an embedded CPython.

use std::fmt::Write as _;

use rspice_core::Complex64;

// The Touchstone writer lives in core so the CLI and the desktop runner emit
// byte-identical files; the result types keep reaching it through this module.
pub(crate) use rspice_core::analysis::advanced::s_param::{
    TouchstoneFormat, TouchstoneFrequencyUnit, TouchstoneInput, touchstone, touchstone_extension,
};

/// Render a float the way every serializer here needs it: shortest round-trip
/// form, with a `.0` kept on integral values so column types stay obvious.
fn format_float(value: f64) -> String {
    if value == value.trunc() && value.is_finite() {
        format!("{value:.1}")
    } else {
        format!("{value}")
    }
}

/// Uniform message for a rejected string-valued option.
fn unknown_option(kind: &str, value: &str, accepted: &[&str]) -> String {
    format!(
        "unknown {kind} '{value}'; expected one of {}",
        accepted.join(", ")
    )
}

//=============================================================================
// Shared helpers
//=============================================================================

/// asctime-style UTC timestamp, matching the header ngspice writes.
///
/// Civil-date conversion uses Hinnant's days-from-epoch algorithm;
/// 1970-01-01 was a Thursday.
fn asctime_utc_now() -> String {
    const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let days = (seconds / 86_400) as i64;
    let time_of_day = seconds % 86_400;
    let (hour, minute, second) = (
        time_of_day / 3600,
        (time_of_day % 3600) / 60,
        time_of_day % 60,
    );

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_position + 2) / 5 + 1;
    let month = if month_position < 10 {
        month_position + 3
    } else {
        month_position - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    format!(
        "{} {} {day} {hour:02}:{minute:02}:{second:02} {year}",
        WEEKDAYS[days.rem_euclid(7) as usize],
        MONTHS[(month - 1) as usize],
    )
}

//=============================================================================
// SPICE raw
//=============================================================================

/// Physical quantity of one raw-file column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVariableKind {
    Time,
    Frequency,
    Voltage,
    Current,
}

impl RawVariableKind {
    fn keyword(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Frequency => "frequency",
            Self::Voltage => "voltage",
            Self::Current => "current",
        }
    }
}

/// One raw-file column.
pub(crate) struct RawVariable {
    pub name: String,
    pub kind: RawVariableKind,
}

/// A complete raw plot: header metadata plus column-major data.
pub(crate) struct RawPlot {
    pub title: String,
    pub plot_name: String,
    pub variables: Vec<RawVariable>,
    /// One series per variable, all the same length.
    pub series: Vec<Vec<Complex64>>,
    /// False writes `Flags: real` and one number per value.
    pub complex: bool,
    /// `Date:` header text. `None` stamps the current UTC time; a fixed value
    /// makes the whole artifact a pure function of the result.
    pub timestamp: Option<String>,
}

impl RawPlot {
    fn validate(&self) -> Result<usize, String> {
        if self.variables.is_empty() {
            return Err("raw export requires at least one variable".to_string());
        }
        if self.series.len() != self.variables.len() {
            return Err(format!(
                "raw export has {} series for {} variables",
                self.series.len(),
                self.variables.len()
            ));
        }
        let points = self.series[0].len();
        for (variable, series) in self.variables.iter().zip(&self.series) {
            if series.len() != points {
                return Err(format!(
                    "raw export variable '{}' has {} points but the sweep axis has {points}",
                    variable.name,
                    series.len()
                ));
            }
        }
        Ok(points)
    }

    fn header(&self, points: usize) -> String {
        let mut header = String::new();
        let _ = writeln!(header, "Title: {}", self.title);
        let _ = writeln!(
            header,
            "Date: {}",
            self.timestamp.clone().unwrap_or_else(asctime_utc_now)
        );
        let _ = writeln!(header, "Plotname: {}", self.plot_name);
        let _ = writeln!(
            header,
            "Flags: {}",
            if self.complex { "complex" } else { "real" }
        );
        let _ = writeln!(header, "No. Variables: {}", self.variables.len());
        let _ = writeln!(header, "No. Points: {points}");
        let _ = writeln!(header, "Variables:");
        for (index, variable) in self.variables.iter().enumerate() {
            let _ = writeln!(
                header,
                "\t{index}\t{}\t{}",
                variable.name,
                variable.kind.keyword()
            );
        }
        header
    }

    /// ASCII raw text. Complex values are written `re,im`, as ngspice does.
    pub(crate) fn to_ascii(&self) -> Result<String, String> {
        let points = self.validate()?;
        let mut output = self.header(points);
        let _ = writeln!(output, "Values:");
        for point in 0..points {
            for (column, series) in self.series.iter().enumerate() {
                let value = series[point];
                let rendered = if self.complex {
                    format!("{},{}", format_float(value.re), format_float(value.im))
                } else {
                    format_float(value.re)
                };
                if column == 0 {
                    let _ = write!(output, "{point}\t{rendered}");
                } else {
                    let _ = write!(output, "\t{rendered}");
                }
            }
            output.push('\n');
        }
        Ok(output)
    }

    /// Binary raw bytes: the same header, then little-endian f64 values.
    pub(crate) fn to_binary(&self) -> Result<Vec<u8>, String> {
        let points = self.validate()?;
        let mut output = self.header(points).into_bytes();
        output.extend_from_slice(b"Binary:\n");
        for point in 0..points {
            for series in &self.series {
                let value = series[point];
                output.extend_from_slice(&value.re.to_le_bytes());
                if self.complex {
                    output.extend_from_slice(&value.im.to_le_bytes());
                }
            }
        }
        Ok(output)
    }
}

/// Raw-file encoding selected by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawFormat {
    Ascii,
    Binary,
}

impl RawFormat {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "ascii" | "text" => Ok(Self::Ascii),
            "binary" | "bin" => Ok(Self::Binary),
            other => Err(unknown_option("raw format", other, &["ascii", "binary"])),
        }
    }
}

/// Serialize a plot in the requested encoding.
pub(crate) fn raw_bytes(plot: &RawPlot, format: RawFormat) -> Result<Vec<u8>, String> {
    match format {
        RawFormat::Ascii => plot.to_ascii().map(String::into_bytes),
        RawFormat::Binary => plot.to_binary(),
    }
}

//=============================================================================
// CSV
//=============================================================================

/// Quote a CSV field only when the content requires it (RFC 4180).
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Render a table as RFC 4180 CSV with CRLF-free `\n` line endings.
///
/// Rows shorter or longer than the header are a programming error in the
/// caller, so the mismatch is reported rather than padded.
pub(crate) fn csv(headers: &[String], rows: &[Vec<f64>]) -> Result<String, String> {
    let mut output = String::new();
    let _ = writeln!(
        output,
        "{}",
        headers
            .iter()
            .map(|header| csv_field(header))
            .collect::<Vec<_>>()
            .join(",")
    );
    for (index, row) in rows.iter().enumerate() {
        if row.len() != headers.len() {
            return Err(format!(
                "CSV row {index} has {} values for {} columns",
                row.len(),
                headers.len()
            ));
        }
        let _ = writeln!(
            output,
            "{}",
            row.iter()
                .map(|value| format_float(*value))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
    Ok(output)
}

//=============================================================================
// File writing
//=============================================================================

/// Write bytes to a path.
pub(crate) fn write_bytes(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(values: &[(f64, f64)]) -> Vec<Complex64> {
        values
            .iter()
            .map(|(re, im)| Complex64::new(*re, *im))
            .collect()
    }

    #[test]
    fn touchstone_two_port_uses_the_s11_s21_s12_s22_ordering() {
        let parameters = vec![
            vec![series(&[(0.1, 0.0)]), series(&[(0.2, 0.0)])],
            vec![series(&[(0.3, 0.0)]), series(&[(0.4, 0.0)])],
        ];
        let text = touchstone(
            &TouchstoneInput {
                frequencies: &[1e9],
                parameters: &parameters,
                reference_impedances: &[50.0, 50.0],
                comments: &["two port".to_string()],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect("two-port export");

        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "! two port");
        assert_eq!(lines[1], "# GHZ S RI R 50");
        let fields: Vec<f64> = lines[2]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        // frequency in GHz, then S11, S21, S12, S22 as real/imaginary pairs.
        assert_eq!(fields[0], 1.0);
        assert_eq!(fields[1], 0.1);
        assert_eq!(fields[3], 0.3);
        assert_eq!(fields[5], 0.2);
        assert_eq!(fields[7], 0.4);
    }

    #[test]
    fn touchstone_rejects_mixed_reference_impedances() {
        let parameters = vec![
            vec![series(&[(0.0, 0.0)]), series(&[(0.0, 0.0)])],
            vec![series(&[(0.0, 0.0)]), series(&[(0.0, 0.0)])],
        ];
        let error = touchstone(
            &TouchstoneInput {
                frequencies: &[1e9],
                parameters: &parameters,
                reference_impedances: &[50.0, 75.0],
                comments: &[],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect_err("mixed impedances must be rejected");
        assert!(error.to_string().contains("one reference impedance"));
    }

    #[test]
    fn touchstone_three_port_writes_one_line_per_matrix_row() {
        let entry = |value: f64| series(&[(value, 0.0)]);
        let parameters = vec![
            vec![entry(11.0), entry(12.0), entry(13.0)],
            vec![entry(21.0), entry(22.0), entry(23.0)],
            vec![entry(31.0), entry(32.0), entry(33.0)],
        ];
        let text = touchstone(
            &TouchstoneInput {
                frequencies: &[2e9],
                parameters: &parameters,
                reference_impedances: &[50.0; 3],
                comments: &[],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect("three-port export");

        let data: Vec<&str> = text.lines().skip(1).collect();
        assert_eq!(data.len(), 3, "one line per matrix row");
        let first: Vec<f64> = data[0]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        assert_eq!(first[0], 2.0);
        assert_eq!(first[1], 11.0);
        assert_eq!(first[3], 12.0);
        assert_eq!(first[5], 13.0);
        let second: Vec<f64> = data[1]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        assert_eq!(second[0], 21.0);
    }

    #[test]
    fn raw_ascii_and_binary_agree_on_header_and_values() {
        let plot = RawPlot {
            title: "probe".to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![
                RawVariable {
                    name: "time".to_string(),
                    kind: RawVariableKind::Time,
                },
                RawVariable {
                    name: "V(out)".to_string(),
                    kind: RawVariableKind::Voltage,
                },
            ],
            series: vec![
                series(&[(0.0, 0.0), (1e-6, 0.0)]),
                series(&[(0.0, 0.0), (2.5, 0.0)]),
            ],
            complex: false,
            timestamp: None,
        };

        let ascii = plot.to_ascii().expect("ascii raw");
        assert!(ascii.contains("Flags: real"));
        assert!(ascii.contains("No. Variables: 2"));
        assert!(ascii.contains("No. Points: 2"));

        let binary = plot.to_binary().expect("binary raw");
        let marker = b"Binary:\n";
        let start = binary
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("binary marker")
            + marker.len();
        // Two points x two real variables x 8 bytes.
        assert_eq!(binary.len() - start, 2 * 2 * 8);
        let last = f64::from_le_bytes(binary[binary.len() - 8..].try_into().expect("f64"));
        assert_eq!(last, 2.5);
    }

    #[test]
    fn raw_complex_writes_two_numbers_per_value() {
        let plot = RawPlot {
            title: "ac".to_string(),
            plot_name: "AC Analysis".to_string(),
            variables: vec![RawVariable {
                name: "frequency".to_string(),
                kind: RawVariableKind::Frequency,
            }],
            series: vec![series(&[(1e3, 0.0)])],
            complex: true,
            timestamp: Some("Thu Jan 1 00:00:00 1970".to_string()),
        };
        let ascii = plot.to_ascii().expect("ascii raw");
        assert!(ascii.contains("Flags: complex"));
        assert!(ascii.lines().last().expect("value line").contains(','));

        let binary = plot.to_binary().expect("binary raw");
        let marker = b"Binary:
";
        let start = binary
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("binary marker")
            + marker.len();
        // One point x one complex variable x two f64.
        assert_eq!(binary.len() - start, 16);
    }

    #[test]
    fn raw_rejects_ragged_series() {
        let plot = RawPlot {
            title: "bad".to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![
                RawVariable {
                    name: "time".to_string(),
                    kind: RawVariableKind::Time,
                },
                RawVariable {
                    name: "V(out)".to_string(),
                    kind: RawVariableKind::Voltage,
                },
            ],
            series: vec![series(&[(0.0, 0.0), (1.0, 0.0)]), series(&[(0.0, 0.0)])],
            complex: false,
            timestamp: None,
        };
        assert!(plot.to_ascii().is_err());
    }

    #[test]
    fn csv_quotes_only_fields_that_need_it() {
        let text = csv(
            &["time".to_string(), "V(a,b)".to_string()],
            &[vec![0.0, 1.0]],
        )
        .expect("csv");
        let header = text.lines().next().expect("header");
        assert_eq!(header, "time,\"V(a,b)\"");
    }

    #[test]
    fn csv_rejects_rows_that_do_not_match_the_header() {
        assert!(csv(&["a".to_string(), "b".to_string()], &[vec![1.0]]).is_err());
    }
}
