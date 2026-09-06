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
//! Two directions live here:
//!
//! - **Into a dump** ([`load_vcd_document`]). A rawfile carrying event plots
//!   and a typed JSON result document both hold the timelines themselves and
//!   convert exactly. Every other source has only the grid `D(node)` /
//!   `E(node)` columns the tabular projection already flattened, and converts
//!   from those — lossily, as [`grid_event_traces`] states.
//! - **Out of a dump** ([`load_vcd_table`]). The distinct ticks become the
//!   rows, each signal holds its last value between them, and the result is an
//!   ordinary [`ExportTable`] — so `convert` can write a dump in any other
//!   format and `compare` can read one on either side without knowing it is a
//!   dump.

use std::collections::BTreeSet;
use std::path::Path;

use rspice_core::ResourceLimits;
use rspice_core::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
use rspice_core::execution::{
    EventProjectionError, RawEventTraces, ResultPayload, event_vcd_document,
};
use rspice_core::io::{VcdBit, VcdDocument, VcdSignal, VcdSignalKind, VcdValue, VcdVariable};
use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

use crate::cli::{CliError, OutputFormat};
use crate::commands::export_table::{ColumnData, ExportColumn, ExportTable};
use crate::commands::publish;
use crate::commands::waveform_io::{conversion_error, enforce_resource_limit, load_table};

/// The `$scope module` every dump RSpice writes declares its nodes under.
///
/// One constant rather than the analysis identity, so that `run -f vcd` and
/// `convert <result> --to vcd` describe the same run with the same file: a
/// rawfile carries no analysis instance to name a scope after.
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
    event_vcd_document(EVENT_SCOPE, digital_traces, real_traces, &[])
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

/// Publish a dump atomically, exactly as every other result artifact is.
pub(crate) fn write_vcd_artifact(path: &Path, document: &VcdDocument) -> Result<(), CliError> {
    publish::artifact(path, |writer| {
        rspice_core::io::write_vcd(&mut *writer, document).map_err(|error| write_error(path, error))
    })
    .map_err(|error| crate::cli::map_atomic_output_error(path, error))
}

/// Parse a dump, naming the file in any failure.
fn parse_vcd(path: &Path, resource_limits: ResourceLimits) -> Result<VcdDocument, CliError> {
    rspice_core::io::parse_vcd_file_with_limits(path, resource_limits)
        .map_err(|error| conversion_error(path, error))
}

// -------------------------------------------------------------------------
// Into a dump

/// Read any supported result file as a VCD document.
///
/// The event timelines are used when the source carries them; otherwise the
/// grid columns are, and the conversion is lossy in the way
/// [`grid_event_traces`] documents.
pub(crate) fn load_vcd_document(
    path: &Path,
    format: OutputFormat,
    resource_limits: ResourceLimits,
) -> Result<VcdDocument, CliError> {
    if format == OutputFormat::Vcd {
        // Reading and rewriting normalises the file: canonical identifier
        // codes, one declaration order, the writer's layout.
        return parse_vcd(path, resource_limits);
    }

    if let Some(document) = event_traces_of(path, format, resource_limits)? {
        return Ok(document);
    }

    let table = load_table(path, format, resource_limits)?;
    let traces = grid_event_traces(path, &table)?;
    event_document(path, &traces.digital_traces, &traces.real_traces)
}

/// The event timelines a source carries in full, when it carries any.
///
/// `Ok(None)` means the source has no event section at all, not that it failed
/// to read: the caller then falls back to the grid columns.
fn event_traces_of(
    path: &Path,
    format: OutputFormat,
    resource_limits: ResourceLimits,
) -> Result<Option<VcdDocument>, CliError> {
    let traces = match format {
        OutputFormat::Raw | OutputFormat::RawAscii => {
            let file = rspice_core::io::parse_raw_plots_file_with_limits(path, resource_limits)
                .map_err(|error| conversion_error(path, error))?;
            rspice_core::execution::decode_event_plots(&file)
                .map_err(|error| conversion_error(path, error))?
        }
        OutputFormat::Json => match typed_transient_traces(path, resource_limits)? {
            Some(traces) => traces,
            None => return Ok(None),
        },
        OutputFormat::Csv | OutputFormat::Tsv | OutputFormat::Hdf5 | OutputFormat::Vcd => {
            return Ok(None);
        }
    };

    if traces.digital_traces.is_empty() && traces.real_traces.is_empty() {
        return Ok(None);
    }
    event_document(path, &traces.digital_traces, &traces.real_traces).map(Some)
}

/// The event timelines a typed result document carries, when it is a transient.
fn typed_transient_traces(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<Option<RawEventTraces>, CliError> {
    let content = crate::commands::waveform_io::read_utf8_input_limited(
        path,
        resource_limits.max_external_data_bytes,
    )?;
    let value: serde_json::Value =
        serde_json::from_str(&content).map_err(|error| conversion_error(path, error))?;
    if value.get("schema").and_then(serde_json::Value::as_str)
        != Some(rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA)
    {
        return Ok(None);
    }
    let document = rspice_core::execution::AnalysisResultDocument::from_json(&content)
        .map_err(|error| conversion_error(path, error))?;
    let ResultPayload::Tran(payload) = document.payload() else {
        return Ok(None);
    };

    let digital_traces = payload
        .digital_traces
        .iter()
        .map(|trace| DigitalTrace {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| DigitalTracePoint {
                    time: point.time,
                    value: DigitalValue::new(
                        DigitalState::from(point.state),
                        DigitalStrength::from(point.strength),
                    ),
                })
                .collect(),
        })
        .collect();
    let real_traces = payload
        .real_traces
        .iter()
        .map(|trace| RealTrace {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| RealTracePoint {
                    time: point.time,
                    value: point.value,
                })
                .collect(),
        })
        .collect();
    Ok(Some(RawEventTraces {
        digital_traces,
        real_traces,
    }))
}

/// Event timelines recovered from a table's grid columns.
///
/// This is the lossy direction. A `D(node)` column is what the tabular
/// projection left of a twelve-state event history: one level per analysis
/// time point, `0`, `1` or `0.5`, with the XSPICE drive strength already
/// dropped and `z` already indistinguishable from `x`. It comes back as `0`,
/// `1` and `x` at strong drive, one change per distinct value — a level that
/// holds across ten grid points is one change, not ten, because that is what
/// the same waveform is in an event dump. An `E(node)` column comes back as
/// itself.
///
/// A table with neither kind of column has no dump to write and is refused.
fn grid_event_traces(path: &Path, table: &ExportTable) -> Result<RawEventTraces, CliError> {
    let mut digital_traces = Vec::new();
    let mut real_traces = Vec::new();

    for column in &table.columns {
        let ColumnData::Real(values) = &column.data else {
            continue;
        };
        if let Some(node) = inner_name(&column.name, DIGITAL_COLUMN_PREFIX) {
            let mut points: Vec<DigitalTracePoint> = Vec::new();
            for (time, value) in table.scale.iter().zip(values) {
                let state = grid_digital_state(path, &column.name, *value)?;
                let value = DigitalValue::new(state, DigitalStrength::Strong);
                if points.last().is_some_and(|last| last.value == value) {
                    continue;
                }
                points.push(DigitalTracePoint { time: *time, value });
            }
            digital_traces.push(DigitalTrace {
                node_name: node.to_string(),
                points,
            });
        } else if let Some(node) = inner_name(&column.name, REAL_COLUMN_PREFIX) {
            let mut points: Vec<RealTracePoint> = Vec::new();
            for (time, value) in table.scale.iter().zip(values) {
                if points.last().is_some_and(|last| last.value == *value) {
                    continue;
                }
                points.push(RealTracePoint {
                    time: *time,
                    value: *value,
                });
            }
            real_traces.push(RealTrace {
                node_name: node.to_string(),
                points,
            });
        }
    }

    if digital_traces.is_empty() && real_traces.is_empty() {
        return Err(conversion_error(
            path,
            "no digital or real event data to write as VCD: the source carries neither event \
             timelines nor D(node)/E(node) columns",
        ));
    }
    Ok(RawEventTraces {
        digital_traces,
        real_traces,
    })
}

/// The logic level one grid sample names.
fn grid_digital_state(path: &Path, column: &str, value: f64) -> Result<DigitalState, CliError> {
    match value {
        0.0 => Ok(DigitalState::Zero),
        1.0 => Ok(DigitalState::One),
        0.5 => Ok(DigitalState::Unknown),
        other => Err(conversion_error(
            path,
            format!(
                "column '{column}' holds {other}, which is not a digital level; a grid digital \
                 column carries 0, 1 or 0.5"
            ),
        )),
    }
}

// -------------------------------------------------------------------------
// Out of a dump

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

// -------------------------------------------------------------------------
// Selection

/// Apply `convert`'s `--variables`, `--start` and `--stop` to a dump.
///
/// A variable is named the way any of the CLI's spellings of it reads: the
/// column spelling `D(node)`, the scoped name a viewer shows, or the bare node
/// name. The range keeps the changes inside it, exactly as clipping a table
/// keeps the rows inside it.
pub(crate) fn select_and_clip(
    document: &mut VcdDocument,
    requested: &[String],
    start: Option<f64>,
    stop: Option<f64>,
) -> Result<(), CliError> {
    if !requested.is_empty() {
        let names = column_names(document);
        for want in requested {
            if !document
                .signals
                .iter()
                .zip(&names)
                .any(|(signal, name)| signal_matches(signal, name, want))
            {
                return Err(CliError::InvalidArgument {
                    message: format!("variable '{want}' not found in input"),
                    suggestion: Some(format!("available variables: {}", names.join(", "))),
                });
            }
        }
        let keep: Vec<bool> = document
            .signals
            .iter()
            .zip(&names)
            .map(|(signal, name)| {
                requested
                    .iter()
                    .any(|want| signal_matches(signal, name, want))
            })
            .collect();
        let mut keep = keep.into_iter();
        document.signals.retain(|_| keep.next().unwrap_or(false));
    }

    if start.is_some() || stop.is_some() {
        let low = start.unwrap_or(f64::NEG_INFINITY);
        let high = stop.unwrap_or(f64::INFINITY);
        let period = document.timescale.seconds();
        for signal in &mut document.signals {
            signal.changes.retain(|change| {
                let time = change.tick as f64 * period;
                time >= low && time <= high
            });
        }
    }

    Ok(())
}

/// Whether one `--variables` name selects this signal.
fn signal_matches(signal: &VcdSignal, column: &str, want: &str) -> bool {
    if column.eq_ignore_ascii_case(want) {
        return true;
    }
    if inner_name(column, DIGITAL_COLUMN_PREFIX)
        .or_else(|| inner_name(column, REAL_COLUMN_PREFIX))
        .is_some_and(|inner| inner.eq_ignore_ascii_case(want))
    {
        return true;
    }
    signal.variables.iter().any(|variable| {
        variable.scoped_name().eq_ignore_ascii_case(want)
            || variable.name.eq_ignore_ascii_case(want)
    })
}

/// The `x` of `D(x)`, when `name` is spelled that way.
fn inner_name<'a>(name: &'a str, prefix: &str) -> Option<&'a str> {
    let trimmed = name.trim();
    if !trimmed
        .get(..prefix.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
    {
        return None;
    }
    trimmed
        .get(prefix.len()..)
        .and_then(|rest| rest.strip_suffix(')'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::io::{VcdChange, VcdTimescale};

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

    fn grid_table(name: &str, times: &[f64], values: &[f64]) -> ExportTable {
        ExportTable {
            analysis: "transient".to_string(),
            plot_name: "Transient Analysis".to_string(),
            scale_name: "time".to_string(),
            scale_type: "time".to_string(),
            scale: times.to_vec(),
            columns: vec![ExportColumn {
                name: name.to_string(),
                var_type: DIGITAL_VARIABLE_TYPE.to_string(),
                data: ColumnData::Real(values.to_vec()),
            }],
        }
    }

    #[test]
    fn a_grid_column_becomes_one_change_per_level_it_holds() {
        let table = grid_table(
            "D(clk)",
            &[0.0, 1e-9, 2e-9, 3e-9, 4e-9],
            &[0.0, 0.0, 1.0, 1.0, 0.5],
        );
        let traces =
            grid_event_traces(Path::new("grid.csv"), &table).expect("a digital grid column");
        assert!(traces.real_traces.is_empty());
        let [trace] = traces.digital_traces.as_slice() else {
            panic!(
                "one digital column is one timeline: {:?}",
                traces.digital_traces
            );
        };
        assert_eq!(trace.node_name, "clk");
        let history: Vec<(f64, DigitalState)> = trace
            .points
            .iter()
            .map(|point| (point.time, point.value.state))
            .collect();
        assert_eq!(
            history,
            vec![
                (0.0, DigitalState::Zero),
                (2e-9, DigitalState::One),
                (4e-9, DigitalState::Unknown),
            ]
        );
        assert!(
            trace
                .points
                .iter()
                .all(|point| point.value.strength == DigitalStrength::Strong),
            "a grid column dropped the drive band; it comes back strong: {trace:?}"
        );
    }

    #[test]
    fn a_table_with_no_event_column_has_no_dump_to_write() {
        let mut table = grid_table("V(out)", &[0.0, 1e-9], &[0.0, 1.0]);
        table.columns[0].var_type = "voltage".to_string();
        let error = grid_event_traces(Path::new("analog.csv"), &table)
            .expect_err("an analog table carries no event timeline");
        assert!(
            error.to_string().contains("D(node)/E(node)"),
            "the refusal must say what a dump needs: {error}"
        );
    }

    #[test]
    fn a_grid_column_carries_only_the_three_levels_the_projection_writes() {
        let path = Path::new("grid.csv");
        assert_eq!(
            grid_digital_state(path, "D(d)", 0.0).expect("a driven low"),
            DigitalState::Zero
        );
        assert_eq!(
            grid_digital_state(path, "D(d)", 1.0).expect("a driven high"),
            DigitalState::One
        );
        assert_eq!(
            grid_digital_state(path, "D(d)", 0.5).expect("an unknown level"),
            DigitalState::Unknown
        );
        let error = grid_digital_state(path, "D(d)", 0.25)
            .expect_err("an analog sample is not a digital level");
        assert!(
            error.to_string().contains("not a digital level"),
            "unexpected refusal: {error}"
        );
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
