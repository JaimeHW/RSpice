//! Value Change Dump as a CLI result format.
//!
//! A VCD is the one output the CLI writes that is not a table. It carries the
//! irregular event timelines a transient captures — four-state digital nodes
//! and real event nodes — at their own times, which is what
//! [`rspice_core::execution::event_vcd_document`] projects and what a logic
//! viewer reads. Everything else the CLI writes is a table on the analysis
//! grid, so every other analysis refuses `--format vcd` through
//! [`unsupported_analysis`].
//!
//! [`load_vcd_table`] is the way back: the distinct ticks become the rows,
//! each signal holds its last value between them, and the result is an
//! ordinary [`ExportTable`] — so `convert` can write a dump in any other
//! format and `compare` can read one on either side without knowing it is a
//! dump.

use std::collections::BTreeSet;
use std::path::Path;

use rspice_core::ResourceLimits;
use rspice_core::engine::{DigitalTrace, RealTrace};
use rspice_core::execution::{EventProjectionError, event_vcd_document};
use rspice_core::io::{VcdBit, VcdDocument, VcdSignalKind, VcdValue, VcdVariable};

use crate::cli::CliError;
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::commands::waveform_io::{conversion_error, enforce_resource_limit};

/// The `$scope module` every dump RSpice writes declares its nodes under.
///
/// One constant rather than the analysis identity, so that every artifact of
/// one run describes it with the same file: a rawfile carries no analysis
/// instance to name a scope after.
const EVENT_SCOPE: &str = "events";

/// Column spelling of a digital event timeline, as every other RSpice surface
/// spells it.
const DIGITAL_COLUMN_PREFIX: &str = "D(";

/// Column spelling of a real event timeline.
const REAL_COLUMN_PREFIX: &str = "E(";

/// Rawfile variable type of a logic column.
const DIGITAL_VARIABLE_TYPE: &str = "digital";

/// Rawfile variable type of a real event column.
const REAL_VARIABLE_TYPE: &str = "real";

/// The widest logic signal whose unsigned value an `f64` column still holds
/// exactly. A wider one would be written rounded, silently.
const MAX_EXACT_LOGIC_WIDTH: u32 = 53;

/// The level an unknown or high-impedance logic value reads as, matching the
/// tabular projection's own `D(node)` column.
const UNKNOWN_LEVEL: f64 = 0.5;

/// `--format vcd` asked of a result that has no event timeline.
///
/// Mirrors the refusal the report-shaped analyses give HDF5: name the format,
/// name the result, and say which formats do carry it.
pub(crate) fn unsupported_analysis(what: &str) -> CliError {
    CliError::InvalidArgument {
        message: format!("VCD output is not supported for {what} results"),
        suggestion: Some(
            "VCD carries the digital and real event timelines only a transient analysis \
             captures; use --format raw, ascii, csv, tsv, json, or hdf5"
                .to_string(),
        ),
    }
}

/// An event history that cannot be written as VCD, named with the file it was
/// being written to.
fn projection_error(path: &Path, error: &EventProjectionError) -> CliError {
    CliError::ConversionError {
        message: format!("{}: {error}", path.display()),
    }
}

/// Project event histories onto a dump under [`EVENT_SCOPE`].
pub(crate) fn event_document(
    path: &Path,
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
) -> Result<VcdDocument, CliError> {
    event_vcd_document(EVENT_SCOPE, digital_traces, real_traces)
        .map_err(|error| projection_error(path, &error))
}

/// A failure while writing a dump, categorised by what actually failed: the
/// stream, or a document VCD cannot express.
pub(crate) fn write_error(path: &Path, error: rspice_core::io::VcdError) -> CliError {
    match error {
        rspice_core::io::VcdError::Io(source) => CliError::OutputError {
            path: path.to_path_buf(),
            source,
        },
        other => conversion_error(path, other),
    }
}

/// Parse a dump, naming the file in any failure.
fn parse_vcd(path: &Path, resource_limits: ResourceLimits) -> Result<VcdDocument, CliError> {
    rspice_core::io::parse_vcd_file_with_limits(path, resource_limits)
        .map_err(|error| conversion_error(path, error))
}

/// Read a dump as a table on the union of its ticks.
///
/// Every distinct tick in the document becomes one row, at `tick` times the
/// `$timescale` period, and each signal holds the last value it changed to.
/// A logic signal becomes a `digital` column: its unsigned integer value when
/// every bit is `0` or `1`, and `0.5` when any bit is `x` or `z` — the same
/// unknown marker the tabular projection writes, which is also why the two
/// cannot be told apart on the way back. A real signal becomes a `real`
/// column.
///
/// Before its first change a logic signal reads `0.5`, because unknown is
/// exactly what it is. A real signal has no unknown to read, so it holds its
/// first value backwards; that is the one value in this direction the dump did
/// not state.
pub(crate) fn load_vcd_table(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<ExportTable, CliError> {
    let document = parse_vcd(path, resource_limits)?;
    let names = column_names(&document);

    let mut ticks: BTreeSet<u64> = BTreeSet::new();
    for signal in &document.signals {
        for change in &signal.changes {
            ticks.insert(change.tick);
        }
    }
    if ticks.is_empty() {
        return Err(conversion_error(
            path,
            "the VCD declares no value change, so it carries no table row",
        ));
    }
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        ticks.len().saturating_mul(document.signals.len() + 1),
        resource_limits.max_external_data_values,
    )?;
    let ticks: Vec<u64> = ticks.into_iter().collect();

    let period = document.timescale.seconds();
    let scale: Vec<f64> = ticks.iter().map(|tick| *tick as f64 * period).collect();

    let mut columns = Vec::with_capacity(document.signals.len());
    for (signal, name) in document.signals.iter().zip(names) {
        if signal.kind == VcdSignalKind::Logic && signal.width > MAX_EXACT_LOGIC_WIDTH {
            return Err(conversion_error(
                path,
                format!(
                    "signal '{name}' is {} bits wide; a table column holds at most {} bits \
                     exactly",
                    signal.width, MAX_EXACT_LOGIC_WIDTH
                ),
            ));
        }
        let mut values = Vec::with_capacity(ticks.len());
        let mut change = signal.changes.iter().peekable();
        let mut held = leading_value(signal.changes.first().map(|change| &change.value));
        for tick in &ticks {
            while change.peek().is_some_and(|next| next.tick <= *tick) {
                if let Some(next) = change.next() {
                    held = sample_value(&next.value);
                }
            }
            values.push(held);
        }
        columns.push(ExportColumn {
            var_type: match signal.kind {
                VcdSignalKind::Logic => DIGITAL_VARIABLE_TYPE.to_string(),
                VcdSignalKind::Real => REAL_VARIABLE_TYPE.to_string(),
            },
            name,
            data: ColumnData::Real(values),
        });
    }

    Ok(ExportTable {
        analysis: "converted".to_string(),
        plot_name: "Converted Data".to_string(),
        scale_name: "time".to_string(),
        scale_type: "time".to_string(),
        scale,
        columns,
    })
}

/// What a signal reads as before its first change.
fn leading_value(first: Option<&VcdValue>) -> f64 {
    match first {
        // A real net has no unknown value to show, so it holds its first one
        // backwards; a logic net's value before it is driven is `x`.
        Some(VcdValue::Real(value)) => *value,
        Some(VcdValue::Logic(_)) | None => UNKNOWN_LEVEL,
    }
}

/// One value change, as a table cell.
fn sample_value(value: &VcdValue) -> f64 {
    match value {
        VcdValue::Real(real) => *real,
        VcdValue::Logic(bits) => {
            let mut number = 0_u64;
            for bit in bits {
                match bit {
                    VcdBit::Zero => number <<= 1,
                    VcdBit::One => number = (number << 1) | 1,
                    VcdBit::Unknown | VcdBit::HighImpedance => return UNKNOWN_LEVEL,
                }
            }
            number as f64
        }
    }
}

/// Column name per signal, in declaration order.
///
/// The scope levels every signal shares are dropped: a dump RSpice wrote
/// declares one scope, so its columns come back `D(node)` and `E(node)` — the
/// spelling every other RSpice surface uses for the same timeline. A foreign
/// dump keeps whatever path still tells its signals apart. An alias shows
/// under the first name it was declared with; a second column would be the
/// same timeline twice.
fn column_names(document: &VcdDocument) -> Vec<String> {
    let declared: Vec<Option<&VcdVariable>> = document
        .signals
        .iter()
        .map(|signal| signal.variables.first())
        .collect();
    let shared = shared_scope_depth(declared.iter().flatten().copied());

    document
        .signals
        .iter()
        .zip(&declared)
        .map(|(signal, variable)| {
            let inner = match variable {
                Some(variable) => {
                    let mut inner = String::new();
                    for level in variable.scope.iter().skip(shared) {
                        inner.push_str(level);
                        inner.push('.');
                    }
                    inner.push_str(&variable.name);
                    inner
                }
                // A signal always declares at least one name, so this is the
                // same last resort the core writer's own labelling uses.
                None => signal.identifier.clone(),
            };
            match signal.kind {
                VcdSignalKind::Logic => format!("{DIGITAL_COLUMN_PREFIX}{inner})"),
                VcdSignalKind::Real => format!("{REAL_COLUMN_PREFIX}{inner})"),
            }
        })
        .collect()
}

/// How many leading scope levels every variable has in common.
fn shared_scope_depth<'a>(variables: impl IntoIterator<Item = &'a VcdVariable>) -> usize {
    let mut variables = variables.into_iter();
    let Some(first) = variables.next() else {
        return 0;
    };
    let mut depth = first.scope.len();
    for variable in variables {
        depth = depth.min(
            first
                .scope
                .iter()
                .zip(&variable.scope)
                .take_while(|(left, right)| left == right)
                .count(),
        );
    }
    depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::engine::DigitalTracePoint;
    use rspice_core::io::{VcdChange, VcdSignal, VcdTimescale};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    fn logic(name: &str, scope: &[&str], changes: &[(u64, VcdBit)]) -> VcdSignal {
        VcdSignal {
            identifier: String::new(),
            variables: vec![VcdVariable {
                scope: scope.iter().map(|level| (*level).to_string()).collect(),
                name: name.to_string(),
            }],
            width: 1,
            kind: VcdSignalKind::Logic,
            changes: changes
                .iter()
                .map(|(tick, bit)| VcdChange {
                    tick: *tick,
                    value: VcdValue::Logic(vec![*bit]),
                })
                .collect(),
        }
    }

    #[test]
    fn a_dump_written_under_one_scope_reads_back_with_the_bare_node_names() {
        let mut document = VcdDocument::new(VcdTimescale::ALL[11]);
        document.signals = vec![
            logic("d", &[EVENT_SCOPE], &[(0, VcdBit::Zero)]),
            logic("clk", &[EVENT_SCOPE], &[(0, VcdBit::One)]),
        ];
        assert_eq!(column_names(&document), vec!["D(d)", "D(clk)"]);
    }

    #[test]
    fn a_foreign_dump_keeps_the_path_that_still_tells_its_signals_apart() {
        let mut document = VcdDocument::new(VcdTimescale::ALL[11]);
        document.signals = vec![
            logic("clk", &["top", "a"], &[(0, VcdBit::Zero)]),
            logic("clk", &["top", "b"], &[(0, VcdBit::One)]),
        ];
        assert_eq!(column_names(&document), vec!["D(a.clk)", "D(b.clk)"]);
    }

    #[test]
    fn a_vector_reads_as_its_unsigned_value_until_a_bit_is_not_driven() {
        use VcdBit::{HighImpedance, One, Unknown, Zero};
        assert_eq!(sample_value(&VcdValue::Logic(vec![One, Zero, One])), 5.0);
        assert_eq!(sample_value(&VcdValue::Logic(vec![Zero, Zero])), 0.0);
        assert_eq!(
            sample_value(&VcdValue::Logic(vec![One, Unknown])),
            UNKNOWN_LEVEL
        );
        assert_eq!(
            sample_value(&VcdValue::Logic(vec![HighImpedance])),
            UNKNOWN_LEVEL
        );
        assert_eq!(sample_value(&VcdValue::Real(1.25)), 1.25);
    }

    #[test]
    fn a_signal_reads_unknown_before_it_is_driven_and_a_real_holds_its_first_value_back() {
        assert_eq!(leading_value(None), UNKNOWN_LEVEL);
        assert_eq!(
            leading_value(Some(&VcdValue::Logic(vec![VcdBit::One]))),
            UNKNOWN_LEVEL
        );
        assert_eq!(leading_value(Some(&VcdValue::Real(-2.5))), -2.5);
    }

    #[test]
    fn an_event_time_no_timescale_carries_names_the_file_and_the_node() {
        let trace = DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![DigitalTracePoint {
                time: 1.5e-15,
                value: DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
            }],
        };
        let error = event_document(Path::new("run.vcd"), &[trace], &[])
            .expect_err("half a femtosecond has no exact tick at any timescale");
        let message = error.to_string();
        assert!(
            message.contains("run.vcd") && message.contains("clk"),
            "the refusal must name the file and the node: {message}"
        );
    }
}
