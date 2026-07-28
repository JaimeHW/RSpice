//! Touchstone v1.1 serialization for N-port scattering results.
//!
//! One writer, shared by every front-end. The CLI previously emitted a
//! fixed `# HZ S RI R <z0>` line of its own while the Python bindings carried
//! this richer implementation, so the same result exported through two
//! products produced two different files.
//!
//! A result whose ports do not all share one reference impedance is refused
//! rather than written with a silently wrong `R`. Touchstone v1 has a single
//! global `R` option and no way to express per-port normalization, so any
//! value written for a mixed-impedance network would be a lie that reads as
//! authoritative in whatever tool consumes the file.

use std::fmt::Write as _;

use crate::Complex64;

/// Numeric field width used by every text writer.
///
/// 17 significant digits round-trips an IEEE-754 double exactly, so a written
/// artifact reloads without loss.
fn format_float(value: f64) -> String {
    format!("{value:.17e}")
}

/// Reject an option value with the list of what is accepted.
pub(crate) fn unknown_option(kind: &str, value: &str, accepted: &[&str]) -> String {
    format!(
        "unknown {kind} '{value}'; expected one of: {}",
        accepted.join(", ")
    )
}

/// Complex-value encoding in a Touchstone data line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFormat {
    RealImaginary,
    MagnitudeAngle,
    DecibelAngle,
}

impl TouchstoneFormat {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "ri" | "real_imaginary" => Ok(Self::RealImaginary),
            "ma" | "magnitude_angle" => Ok(Self::MagnitudeAngle),
            "db" | "decibel_angle" => Ok(Self::DecibelAngle),
            other => Err(unknown_option(
                "Touchstone format",
                other,
                &["ri", "ma", "db"],
            )),
        }
    }

    fn keyword(self) -> &'static str {
        match self {
            Self::RealImaginary => "RI",
            Self::MagnitudeAngle => "MA",
            Self::DecibelAngle => "DB",
        }
    }

    /// Encode one S-parameter as the format's two real numbers.
    ///
    /// `DB` uses `20*log10|S|`; a zero magnitude therefore yields `-inf`,
    /// which no Touchstone reader accepts, so it is floored at the smallest
    /// representable magnitude instead of emitting a non-numeric token.
    fn encode(self, value: Complex64) -> (f64, f64) {
        match self {
            Self::RealImaginary => (value.re, value.im),
            Self::MagnitudeAngle => (value.norm(), value.arg().to_degrees()),
            Self::DecibelAngle => {
                let magnitude = value.norm();
                let decibels = if magnitude > 0.0 {
                    20.0 * magnitude.log10()
                } else {
                    -f64::MAX
                };
                (decibels, value.arg().to_degrees())
            }
        }
    }
}

/// Frequency unit written on the option line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchstoneFrequencyUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

impl TouchstoneFrequencyUnit {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "hz" => Ok(Self::Hz),
            "khz" => Ok(Self::KHz),
            "mhz" => Ok(Self::MHz),
            "ghz" => Ok(Self::GHz),
            other => Err(unknown_option(
                "Touchstone frequency unit",
                other,
                &["hz", "khz", "mhz", "ghz"],
            )),
        }
    }

    fn keyword(self) -> &'static str {
        match self {
            Self::Hz => "HZ",
            Self::KHz => "KHZ",
            Self::MHz => "MHZ",
            Self::GHz => "GHZ",
        }
    }

    fn divisor(self) -> f64 {
        match self {
            Self::Hz => 1.0,
            Self::KHz => 1e3,
            Self::MHz => 1e6,
            Self::GHz => 1e9,
        }
    }
}

/// Everything a Touchstone file needs from an S-parameter result.
pub struct TouchstoneInput<'a> {
    pub frequencies: &'a [f64],
    /// `[row][column][frequency_index]`, one-based ports in row/column order.
    pub parameters: &'a [Vec<Vec<Complex64>>],
    pub reference_impedances: &'a [f64],
    pub comments: &'a [String],
}

/// Touchstone v1 permits at most four complex pairs on one physical line.
pub const MAX_PAIRS_PER_LINE: usize = 4;

/// Render a Touchstone v1 document.
pub fn touchstone(
    input: &TouchstoneInput<'_>,
    format: TouchstoneFormat,
    unit: TouchstoneFrequencyUnit,
) -> Result<String, String> {
    let ports = input.parameters.len();
    if ports == 0 {
        return Err("Touchstone export requires at least one port".to_string());
    }
    if input.reference_impedances.len() != ports
        || input.parameters.iter().any(|row| row.len() != ports)
    {
        return Err(
            "malformed S-parameter result: port count does not match the scattering matrix"
                .to_string(),
        );
    }
    for row in input.parameters {
        for series in row {
            if series.len() != input.frequencies.len() {
                return Err(format!(
                    "malformed S-parameter result: a series has {} points for {} frequencies",
                    series.len(),
                    input.frequencies.len()
                ));
            }
        }
    }

    // Touchstone v1 carries a single reference impedance on its option line.
    // Writing one anyway for a mixed-impedance network would produce a file
    // whose S-parameters mean something other than what was simulated.
    let reference = input.reference_impedances[0];
    if let Some((index, mismatched)) = input
        .reference_impedances
        .iter()
        .enumerate()
        .find(|(_, z0)| z0.to_bits() != reference.to_bits())
    {
        return Err(format!(
            "Touchstone v1 supports one reference impedance, but port 1 uses {reference} ohm and \
             port {} uses {mismatched} ohm; renormalize the ports before exporting",
            index + 1
        ));
    }

    let mut output = String::new();
    for comment in input.comments {
        for line in comment.lines() {
            let _ = writeln!(output, "! {line}");
        }
    }
    let _ = writeln!(
        output,
        "# {} S {} R {reference}",
        unit.keyword(),
        format.keyword()
    );

    let divisor = unit.divisor();
    for (frequency_index, frequency) in input.frequencies.iter().enumerate() {
        let pair = |row: usize, column: usize| {
            format.encode(input.parameters[row][column][frequency_index])
        };
        let frequency_field = format_float(frequency / divisor);

        match ports {
            // 1-port: freq S11
            1 => {
                let (first, second) = pair(0, 0);
                let _ = writeln!(
                    output,
                    "{frequency_field}\t{}\t{}",
                    format_float(first),
                    format_float(second)
                );
            }
            // 2-port: the format's historical S11 S21 S12 S22 ordering.
            2 => {
                let mut line = frequency_field;
                for (row, column) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let (first, second) = pair(row, column);
                    let _ = write!(line, "\t{}\t{}", format_float(first), format_float(second));
                }
                let _ = writeln!(output, "{line}");
            }
            // N-port: one matrix row per line block, row-major, wrapped at
            // four pairs. The frequency leads the first line only.
            _ => {
                for row in 0..ports {
                    let mut line = if row == 0 {
                        frequency_field.clone()
                    } else {
                        String::new()
                    };
                    let mut pairs_on_line = 0;
                    for column in 0..ports {
                        if pairs_on_line == MAX_PAIRS_PER_LINE {
                            let _ = writeln!(output, "{line}");
                            line = String::new();
                            pairs_on_line = 0;
                        }
                        let (first, second) = pair(row, column);
                        let _ = write!(
                            line,
                            "{}{}\t{}",
                            if line.is_empty() { "" } else { "\t" },
                            format_float(first),
                            format_float(second)
                        );
                        pairs_on_line += 1;
                    }
                    let _ = writeln!(output, "{line}");
                }
            }
        }
    }

    Ok(output)
}

/// Conventional Touchstone file extension for a port count.
pub fn touchstone_extension(ports: usize) -> String {
    format!("s{ports}p")
}
