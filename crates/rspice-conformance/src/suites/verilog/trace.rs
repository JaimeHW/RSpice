//! Parsing and comparing the observation traces two simulators produce.
//!
//! The grammar is a header followed by one row per vector:
//!
//! ```text
//! # RSPICE-VERILOG-TRACE 1
//! @0 N22=1 N23=1
//! @1 N22=1 N23=0
//! ```
//!
//! # Strict where it counts, tolerant only before the header
//!
//! Anything a tool prints before the header is preamble and is kept for
//! diagnostics but not compared: banners are not something two different
//! simulators can be asked to agree on. After the header the parser is exact.
//! A non-empty line that is not a row is an error, not something to skip — the
//! skipping version of this parser silently swallows a simulator's runtime
//! warning about the very case under test, and a warning printed between two
//! rows is precisely the interesting kind.
//!
//! Comparison is textual on `%b` values, so `x` and `z` compare as themselves
//! rather than being coerced to a number first. There is no tolerance and no
//! normalisation: these are logic values, and two simulators either produced
//! the same ones or they did not.

use std::collections::BTreeSet;
use std::fmt;

use super::testbench::TRACE_HEADER;

/// One sampled observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceRow {
    pub step: u64,
    /// Port name and its `%b` value, in the order the bench wrote them.
    pub values: Vec<(String, String)>,
}

impl TraceRow {
    pub fn value(&self, port: &str) -> Option<&str> {
        self.values
            .iter()
            .find(|(name, _)| name == port)
            .map(|(_, value)| value.as_str())
    }

    fn ports(&self) -> BTreeSet<&str> {
        self.values.iter().map(|(name, _)| name.as_str()).collect()
    }
}

/// A parsed trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    pub rows: Vec<TraceRow>,
    /// Whatever the tool printed before the header. Never compared; reported
    /// when something else fails, because it usually explains why.
    pub preamble: String,
}

/// Why a trace could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceError {
    /// The header never appeared — usually the simulator failed before the
    /// first `$write` and its output is entirely diagnostics.
    MissingHeader { output: String },
    /// A line after the header that is not a row.
    UnexpectedLine { line: usize, text: String },
    MalformedRow {
        line: usize,
        text: String,
        detail: String,
    },
    /// Rows must be numbered 0, 1, 2, ... in order. Out-of-order rows mean the
    /// bench's own step counter is wrong, which invalidates every comparison
    /// built on it.
    OutOfOrder {
        line: usize,
        expected: u64,
        found: u64,
    },
    /// A trace with no rows would compare equal to any other empty trace.
    Empty,
}

impl fmt::Display for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeader { output } => write!(
                f,
                "no '{TRACE_HEADER}' line in the output; the run produced:\n{}",
                tail(output)
            ),
            Self::UnexpectedLine { line, text } => write!(
                f,
                "line {line}: expected a trace row or nothing, found '{text}'"
            ),
            Self::MalformedRow { line, text, detail } => {
                write!(f, "line {line}: malformed row '{text}': {detail}")
            }
            Self::OutOfOrder {
                line,
                expected,
                found,
            } => write!(f, "line {line}: expected step {expected}, found {found}"),
            Self::Empty => write!(f, "the trace has a header but no rows"),
        }
    }
}

impl std::error::Error for TraceError {}

/// Parse the stdout of one simulation run.
pub fn parse_trace(output: &str) -> Result<Trace, TraceError> {
    let lines = output.lines().collect::<Vec<_>>();
    let header_index = lines
        .iter()
        .position(|line| line.trim() == TRACE_HEADER)
        .ok_or_else(|| TraceError::MissingHeader {
            output: output.to_string(),
        })?;

    let preamble = lines[..header_index].join("\n");
    let mut rows = Vec::new();
    for (offset, raw) in lines[header_index + 1..].iter().enumerate() {
        let number = header_index + offset + 2;
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with('@') {
            return Err(TraceError::UnexpectedLine {
                line: number,
                text: line.to_string(),
            });
        }
        let mut fields = line.split_whitespace();
        let step_token = fields.next().unwrap_or_default();
        let step = step_token[1..]
            .parse::<u64>()
            .map_err(|err| TraceError::MalformedRow {
                line: number,
                text: line.to_string(),
                detail: format!("step '{}' is not a number: {err}", &step_token[1..]),
            })?;
        let expected = rows.len() as u64;
        if step != expected {
            return Err(TraceError::OutOfOrder {
                line: number,
                expected,
                found: step,
            });
        }

        let mut values = Vec::new();
        for field in fields {
            let (name, value) = field
                .split_once('=')
                .ok_or_else(|| TraceError::MalformedRow {
                    line: number,
                    text: line.to_string(),
                    detail: format!("'{field}' is not <port>=<value>"),
                })?;
            if name.is_empty() || value.is_empty() {
                return Err(TraceError::MalformedRow {
                    line: number,
                    text: line.to_string(),
                    detail: format!("'{field}' has an empty port or value"),
                });
            }
            values.push((name.to_string(), value.to_string()));
        }
        if values.is_empty() {
            return Err(TraceError::MalformedRow {
                line: number,
                text: line.to_string(),
                detail: "no observations; the row proves nothing".to_string(),
            });
        }
        rows.push(TraceRow { step, values });
    }

    if rows.is_empty() {
        return Err(TraceError::Empty);
    }
    Ok(Trace { rows, preamble })
}

/// One way in which two traces differ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Divergence {
    /// Different numbers of observations. Reported on its own because every
    /// per-row difference after the shorter trace ends would be noise.
    RowCount { left: usize, right: usize },
    /// The two runs observed different sets of ports.
    PortSet {
        step: u64,
        only_left: Vec<String>,
        only_right: Vec<String>,
    },
    /// The interesting one: same port, same step, different logic value.
    Value {
        step: u64,
        port: String,
        left: String,
        right: String,
    },
}

impl fmt::Display for Divergence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RowCount { left, right } => {
                write!(f, "row count: {left} versus {right}")
            }
            Self::PortSet {
                step,
                only_left,
                only_right,
            } => write!(
                f,
                "step {step}: ports observed only on the left [{}], only on the right [{}]",
                only_left.join(", "),
                only_right.join(", ")
            ),
            Self::Value {
                step,
                port,
                left,
                right,
            } => write!(f, "step {step} {port}: {left} versus {right}"),
        }
    }
}

/// Largest number of divergences reported before the rest are elided.
///
/// A wrong carry chain diverges on every row after the first; the twentieth
/// report adds nothing the first did not, and a truncated list stays readable
/// in a CI log.
pub const MAX_REPORTED_DIVERGENCES: usize = 10;

/// Compare two traces of the same case.
///
/// Returns every difference found, capped at [`MAX_REPORTED_DIVERGENCES`]. An
/// empty result means the two runs agree exactly.
pub fn compare_traces(left: &Trace, right: &Trace) -> Vec<Divergence> {
    let mut divergences = Vec::new();
    if left.rows.len() != right.rows.len() {
        divergences.push(Divergence::RowCount {
            left: left.rows.len(),
            right: right.rows.len(),
        });
    }

    for (left_row, right_row) in left.rows.iter().zip(&right.rows) {
        if divergences.len() >= MAX_REPORTED_DIVERGENCES {
            break;
        }
        let left_ports = left_row.ports();
        let right_ports = right_row.ports();
        if left_ports != right_ports {
            divergences.push(Divergence::PortSet {
                step: left_row.step,
                only_left: left_ports
                    .difference(&right_ports)
                    .map(|port| (*port).to_string())
                    .collect(),
                only_right: right_ports
                    .difference(&left_ports)
                    .map(|port| (*port).to_string())
                    .collect(),
            });
            continue;
        }
        for (port, left_value) in &left_row.values {
            if divergences.len() >= MAX_REPORTED_DIVERGENCES {
                break;
            }
            let right_value = right_row.value(port).unwrap_or_default();
            if left_value != right_value {
                divergences.push(Divergence::Value {
                    step: left_row.step,
                    port: port.clone(),
                    left: left_value.clone(),
                    right: right_value.to_string(),
                });
            }
        }
    }
    divergences
}

/// Render a divergence list for a failure message.
pub fn describe(left_label: &str, right_label: &str, divergences: &[Divergence]) -> String {
    if divergences.is_empty() {
        return format!("{left_label} and {right_label} agree");
    }
    let mut out = format!(
        "{left_label} and {right_label} disagree ({} difference(s), left = {left_label}):",
        divergences.len()
    );
    for divergence in divergences {
        out.push_str("\n  ");
        out.push_str(&divergence.to_string());
    }
    if divergences.len() >= MAX_REPORTED_DIVERGENCES {
        out.push_str("\n  (further differences elided)");
    }
    out
}

fn tail(value: &str) -> String {
    const LIMIT: usize = 800;
    let value = value.trim();
    if value.len() <= LIMIT {
        return value.to_string();
    }
    let start = value.len() - LIMIT;
    let start = value
        .char_indices()
        .map(|(index, _)| index)
        .find(|index| *index >= start)
        .unwrap_or(value.len());
    format!("...{}", &value[start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = "# RSPICE-VERILOG-TRACE 1\n@0 y=1 w=1010\n@1 y=0 w=0101\n";

    #[test]
    fn a_well_formed_trace_parses_into_ordered_rows() {
        let trace = parse_trace(GOOD).expect("parses");

        assert_eq!(trace.rows.len(), 2);
        assert_eq!(trace.rows[0].step, 0);
        assert_eq!(trace.rows[0].value("y"), Some("1"));
        assert_eq!(trace.rows[1].value("w"), Some("0101"));
        assert!(trace.preamble.is_empty());
    }

    #[test]
    fn tool_preamble_before_the_header_is_kept_but_not_compared() {
        let trace = parse_trace(&format!("Verilator 5.020\nbuilding...\n{GOOD}")).expect("parses");

        assert_eq!(trace.rows.len(), 2);
        assert!(trace.preamble.contains("Verilator 5.020"));
        assert_eq!(trace, {
            let mut plain = parse_trace(GOOD).expect("parses");
            plain.preamble = trace.preamble.clone();
            plain
        });
    }

    #[test]
    fn a_warning_printed_between_rows_is_an_error_rather_than_skipped() {
        let output = "# RSPICE-VERILOG-TRACE 1\n@0 y=1\nWARNING: something\n@1 y=0\n";

        let error = parse_trace(output).expect_err("mid-trace output must not be swallowed");

        assert!(
            matches!(&error, TraceError::UnexpectedLine { text, .. } if text.contains("WARNING")),
            "{error:?}"
        );
    }

    #[test]
    fn missing_header_reports_what_the_run_actually_printed() {
        let error = parse_trace("iverilog: command not found\n").expect_err("no header");

        let TraceError::MissingHeader { output } = &error else {
            panic!("expected a missing header, got {error:?}");
        };
        assert!(output.contains("command not found"));
    }

    #[test]
    fn out_of_order_steps_are_rejected() {
        let error = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=1\n@2 y=0\n")
            .expect_err("step numbering is part of the contract");

        assert_eq!(
            error,
            TraceError::OutOfOrder {
                line: 3,
                expected: 1,
                found: 2
            }
        );
    }

    #[test]
    fn a_header_without_rows_is_rejected_rather_than_agreeing_with_everything() {
        assert_eq!(parse_trace(TRACE_HEADER), Err(TraceError::Empty));
    }

    #[test]
    fn identical_traces_agree() {
        let left = parse_trace(GOOD).expect("parses");
        let right = parse_trace(GOOD).expect("parses");

        assert!(compare_traces(&left, &right).is_empty());
        assert!(describe("a", "b", &[]).contains("agree"));
    }

    #[test]
    fn a_single_differing_bit_is_located_by_step_and_port() {
        let left = parse_trace(GOOD).expect("parses");
        let right = parse_trace(GOOD.replace("w=0101", "w=0111").as_str()).expect("parses");

        let divergences = compare_traces(&left, &right);

        assert_eq!(
            divergences,
            vec![Divergence::Value {
                step: 1,
                port: "w".to_string(),
                left: "0101".to_string(),
                right: "0111".to_string(),
            }]
        );
        let described = describe("icarus", "verilator", &divergences);
        assert!(
            described.contains("step 1 w: 0101 versus 0111"),
            "{described}"
        );
    }

    #[test]
    fn four_state_values_compare_as_themselves() {
        let left = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=x\n").expect("parses");
        let right = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=0\n").expect("parses");

        // The whole point of the four-state corpus entry: x is not 0.
        assert_eq!(compare_traces(&left, &right).len(), 1);

        let same = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=x\n").expect("parses");
        assert!(compare_traces(&left, &same).is_empty());
    }

    #[test]
    fn a_truncated_run_is_reported_as_a_row_count_difference() {
        let left = parse_trace(GOOD).expect("parses");
        let right = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=1 w=1010\n").expect("parses");

        let divergences = compare_traces(&left, &right);

        assert_eq!(
            divergences.first(),
            Some(&Divergence::RowCount { left: 2, right: 1 })
        );
    }

    #[test]
    fn differing_port_sets_are_reported_once_per_row_not_per_port() {
        let left = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=1 w=1010\n").expect("parses");
        let right = parse_trace("# RSPICE-VERILOG-TRACE 1\n@0 y=1\n").expect("parses");

        let divergences = compare_traces(&left, &right);

        assert_eq!(divergences.len(), 1);
        assert!(
            matches!(&divergences[0], Divergence::PortSet { only_left, .. } if only_left == &vec!["w".to_string()]),
            "{divergences:?}"
        );
    }

    #[test]
    fn divergence_reporting_is_capped() {
        let mut left_text = String::from("# RSPICE-VERILOG-TRACE 1\n");
        let mut right_text = left_text.clone();
        for step in 0..50 {
            left_text.push_str(&format!("@{step} y=1\n"));
            right_text.push_str(&format!("@{step} y=0\n"));
        }
        let left = parse_trace(&left_text).expect("parses");
        let right = parse_trace(&right_text).expect("parses");

        let divergences = compare_traces(&left, &right);

        assert_eq!(divergences.len(), MAX_REPORTED_DIVERGENCES);
        assert!(describe("a", "b", &divergences).contains("elided"));
    }
}
