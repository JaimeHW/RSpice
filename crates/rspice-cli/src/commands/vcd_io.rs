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
use rspice_core::engine::{
    DigitalBusDeclaration, DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint,
};
use rspice_core::execution::{
    EventProjectionError, RawEventTraces, ResultPayload, event_vcd_document, split_bus_notation,
};
use rspice_core::io::{
    VcdBit, VcdChange, VcdDocument, VcdSignal, VcdSignalKind, VcdValue, VcdVariable,
};
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

/// `--expand-buses` asked of an output that is not a dump.
///
/// The flag names one thing about the VCD grammar — that a bus can be written
/// as N one-bit `$var`s instead of one N-bit one — and no other format has a
/// vector to expand. A table already writes each member as its own `D(node)`
/// column, so accepting the flag there would have meant accepting it as a
/// no-op, which is how a caller comes to believe a flag did something.
pub(crate) fn expand_buses_needs_vcd(flag: &str, format: OutputFormat) -> Result<(), CliError> {
    if format == OutputFormat::Vcd {
        return Ok(());
    }
    Err(CliError::InvalidArgument {
        message: format!(
            "{flag} applies to VCD output only, not {}",
            format!("{format:?}").to_lowercase()
        ),
        suggestion: Some(
            "a bus is one vector variable only in a dump; every table format already writes each \
             member as its own D(node) column"
                .to_string(),
        ),
    })
}

/// An event history that cannot be written as VCD, named with the file it was
/// being written to.
fn projection_error(path: &Path, error: &EventProjectionError) -> CliError {
    CliError::ConversionError {
        message: format!("{}: {error}", path.display()),
    }
}

/// Project event histories onto a dump under [`EVENT_SCOPE`].
///
/// A declared bus becomes one vector `$var` of its own width and its member
/// scalars are not written beside it, which is the shape the Python and
/// browser bindings' dumps have. Passing an empty table is not the same
/// request: it writes every member as its own one-bit wire, which is what a
/// source that never said which conductors were one word has to say.
pub(crate) fn event_document(
    path: &Path,
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
    digital_buses: &[DigitalBusDeclaration],
) -> Result<VcdDocument, CliError> {
    event_vcd_document(EVENT_SCOPE, digital_traces, real_traces, digital_buses)
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
///
/// `expand_buses` writes every member of a declared bus as its own one-bit
/// `$var` and no vector beside them — the shape a dump had before buses
/// existed, for a reader that cannot take a vector. Nothing is lost by it: a
/// scalar `$var` carries no strength either, so the members say exactly what
/// the vector said. What it costs is the declaration itself, which no scalar
/// dump has a place for.
///
/// Expanding a dump keeps the dump's own scopes. A source that is not already
/// a dump has no scope tree to keep, so its variables are declared under
/// [`EVENT_SCOPE`] whether they are expanded or not.
pub(crate) fn load_vcd_document(
    path: &Path,
    format: OutputFormat,
    resource_limits: ResourceLimits,
    expand_buses: bool,
) -> Result<VcdDocument, CliError> {
    if format == OutputFormat::Vcd {
        // Reading and rewriting normalises the file: canonical identifier
        // codes, one declaration order, the writer's layout.
        let mut document = parse_vcd(path, resource_limits)?;
        if expand_buses {
            expand_vector_variables(path, &mut document)?;
        }
        return Ok(document);
    }

    if let Some(document) = event_traces_of(path, format, resource_limits, expand_buses)? {
        return Ok(document);
    }

    let table = load_table(path, format, resource_limits)?;
    let traces = grid_event_traces(path, &table)?;
    event_document(
        path,
        &traces.digital_traces,
        &traces.real_traces,
        &traces.digital_buses,
    )
}

/// Replace every vector variable in a parsed dump with its member bits.
///
/// Each member is declared in the scope the vector was declared in, so a
/// foreign dump keeps its own hierarchy. That is the whole reason this is a
/// document rewrite rather than a trip through the event histories: histories
/// carry a flat node name, and reprojecting them re-declares every variable —
/// scalars included — under [`EVENT_SCOPE`], which silently renamed the
/// scopes of a dump that had its own.
///
/// A member is named `bus[index]`, walking the declared range from its most
/// significant end, because a dump does not carry the engine's member node
/// names: what a vector `$var` declares is the range, and its bits are the
/// bit-selects of that range.
///
/// The declaration order is the source's: a vector's members stand where the
/// vector stood, so a dump of scalars and vectors keeps its column order.
fn expand_vector_variables(path: &Path, document: &mut VcdDocument) -> Result<(), CliError> {
    if !document
        .signals
        .iter()
        .any(|signal| signal.kind == VcdSignalKind::Logic && signal.width > 1)
    {
        return Ok(());
    }

    let mut expanded: Vec<VcdSignal> = Vec::new();
    for signal in std::mem::take(&mut document.signals) {
        if signal.kind != VcdSignalKind::Logic || signal.width <= 1 {
            expanded.push(signal);
            continue;
        }
        expanded.extend(expand_one_vector(path, &signal)?);
    }
    document.signals = expanded;
    Ok(())
}

/// One vector variable's bits, in declared order, most significant first.
fn expand_one_vector(path: &Path, signal: &VcdSignal) -> Result<Vec<VcdSignal>, CliError> {
    let width = signal.width as usize;
    // Expanding costs one change per member per recorded change before the
    // per-member deduplication can shrink it, which is the same product
    // `rspice_core::execution::MAX_BUS_EVENT_CELLS` bounds on the way in.
    let cells = signal.changes.len().saturating_mul(width);
    if cells > rspice_core::execution::MAX_BUS_EVENT_CELLS {
        return Err(conversion_error(
            path,
            format!(
                "expanding '{}' would materialize {cells} member values, past the {} this build \
                 holds at once",
                variable_reference(signal),
                rspice_core::execution::MAX_BUS_EVENT_CELLS
            ),
        ));
    }

    // Every declared name for the timeline is expanded, so an aliased vector
    // keeps both of its names on each bit. The bit positions come from each
    // name's own declared range: an alias may spell the same wire's range
    // differently, and a name that declares none is read as `width-1 .. 0`.
    let mut member_names: Vec<Vec<VcdVariable>> = vec![Vec::new(); width];
    for variable in &signal.variables {
        for (position, index) in declared_indices(path, signal, &variable.name)?
            .into_iter()
            .enumerate()
        {
            member_names[position].push(VcdVariable {
                scope: variable.scope.clone(),
                name: format!("{}[{index}]", split_bus_notation(&variable.name).0),
            });
        }
    }

    let mut members: Vec<VcdSignal> = member_names
        .into_iter()
        .map(|variables| VcdSignal {
            identifier: String::new(),
            variables,
            width: 1,
            kind: VcdSignalKind::Logic,
            changes: Vec::new(),
        })
        .collect();

    for change in &signal.changes {
        let VcdValue::Logic(bits) = &change.value else {
            return Err(conversion_error(
                path,
                format!(
                    "vector '{}' carries a real value at tick {}",
                    variable_reference(signal),
                    change.tick
                ),
            ));
        };
        if bits.len() != width {
            return Err(conversion_error(
                path,
                format!(
                    "vector '{}' is {width} bits wide but carries {} at tick {}",
                    variable_reference(signal),
                    bits.len(),
                    change.tick
                ),
            ));
        }
        for (member, bit) in members.iter_mut().zip(bits) {
            // A dump records changes, not samples: a bit that did not move
            // when its neighbour did has nothing to say at that tick.
            if member.changes.last().is_some_and(
                |last| matches!(&last.value, VcdValue::Logic(held) if held.as_slice() == [*bit]),
            ) {
                continue;
            }
            member.changes.push(VcdChange {
                tick: change.tick,
                value: VcdValue::Logic(vec![*bit]),
            });
        }
    }

    Ok(members)
}

/// The bit indices one vector name declares, most significant first.
///
/// A name with no range (`data`) is read as `width-1 .. 0`; a range whose span
/// disagrees with the declared width is refused rather than truncated, because
/// either the range or the width is wrong and nothing here can tell which.
fn declared_indices(path: &Path, signal: &VcdSignal, name: &str) -> Result<Vec<i64>, CliError> {
    let width = i64::from(signal.width);
    let (msb, lsb) = match split_bus_notation(name).1 {
        Some((msb, lsb)) => {
            let span = msb.abs_diff(lsb).saturating_add(1);
            if span != u64::from(signal.width) {
                return Err(conversion_error(
                    path,
                    format!(
                        "vector '{name}' is {} bits wide but declares [{msb}:{lsb}], a range of \
                         {span}",
                        signal.width
                    ),
                ));
            }
            (msb, lsb)
        }
        None => (width - 1, 0),
    };
    let step = if msb >= lsb { -1 } else { 1 };
    Ok((0..width).map(|offset| msb + step * offset).collect())
}

/// A signal's first declared name, for a message about it.
fn variable_reference(signal: &VcdSignal) -> String {
    signal
        .variables
        .first()
        .map_or_else(|| signal.identifier.clone(), VcdVariable::scoped_name)
}

/// The event timelines a source carries in full, when it carries any.
///
/// `Ok(None)` means the source has no event section at all, not that it failed
/// to read: the caller then falls back to the grid columns.
fn event_traces_of(
    path: &Path,
    format: OutputFormat,
    resource_limits: ResourceLimits,
    expand_buses: bool,
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
    let declared: &[DigitalBusDeclaration] = if expand_buses {
        &[]
    } else {
        &traces.digital_buses
    };
    event_document(path, &traces.digital_traces, &traces.real_traces, declared).map(Some)
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
        // A version-2 document says which of those conductors are one word,
        // and a version-1 document -- which had nowhere to say it -- carries
        // an empty table, so this is the declaration the run published or
        // nothing.
        digital_buses: payload
            .digital_buses
            .iter()
            .map(DigitalBusDeclaration::from)
            .collect(),
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
        // A grid column is what a table kept of an event history; it never
        // said which columns were one word, so this direction declares none.
        digital_buses: Vec::new(),
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
/// A logic signal becomes a `digital` column holding `0`, `1`, or `0.5` when
/// the bit is `x` or `z` — the same unknown marker the tabular projection
/// writes, which is also why the two cannot be told apart on the way back. A
/// real signal becomes a `real` column.
///
/// **A vector variable becomes one column per bit**, named `D(bus[k])` and
/// walking the declared range from its most significant end, exactly as
/// [`expand_vector_variables`] declares them in a dump. A table has no place
/// for a declaration, so it has no way to say that N of its columns are one
/// word; writing the word instead would make this the only route in the
/// product where a bus reaches a table as a number, and the same run's
/// rawfile and its dump would convert to two different CSVs.
///
/// Before its first change a logic signal reads `0.5`, because unknown is
/// exactly what it is. A real signal has no unknown to read, so it holds its
/// first value backwards; that is the one value in this direction the dump did
/// not state.
pub(crate) fn load_vcd_table(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<ExportTable, CliError> {
    let mut document = parse_vcd(path, resource_limits)?;
    expand_vector_variables(path, &mut document)?;
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
///
/// Every logic change reaching here carries exactly one bit:
/// [`load_vcd_table`] expands vector variables into member columns before it
/// builds any. Anything wider reads unknown rather than being packed into a
/// number, because a packed word is not something this route writes.
fn sample_value(value: &VcdValue) -> f64 {
    match value {
        VcdValue::Real(real) => *real,
        VcdValue::Logic(bits) => match bits.as_slice() {
            [VcdBit::Zero] => 0.0,
            [VcdBit::One] => 1.0,
            _ => UNKNOWN_LEVEL,
        },
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
///
/// # Buses
///
/// A vector variable answers to its bus name (`data`), to its declared range
/// in either spelling (`data[7:0]`, `data [7:0]`), and to any one of its bits
/// (`data[3]`). The last of those selects the *whole* vector, because a VCD
/// vector is all-or-nothing — a `$var` is as wide as it is declared, and there
/// is no dump that carries one bit of one. That widening is returned as a note
/// rather than done silently, since the caller asked for less than it got.
///
/// The returned notes are informational; an unknown name is still refused.
#[must_use = "the notes say where a selection was widened past what was asked"]
pub(crate) fn select_and_clip(
    document: &mut VcdDocument,
    requested: &[String],
    start: Option<f64>,
    stop: Option<f64>,
) -> Result<Vec<String>, CliError> {
    let mut notes = Vec::new();
    if !requested.is_empty() {
        let names = column_names(document);
        for want in requested {
            let mut found = false;
            for (signal, name) in document.signals.iter().zip(&names) {
                match signal_selection(signal, name, want) {
                    Selection::No => continue,
                    Selection::Whole => found = true,
                    Selection::WholeBusForOneBit { bus } => {
                        found = true;
                        notes.push(format!(
                            "'{want}' names one bit of digital bus '{bus}', and a VCD vector is \
                             written whole or not at all, so the whole bus is kept; convert to a \
                             table format to select one member column"
                        ));
                    }
                }
            }
            if !found {
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
                    .any(|want| signal_selection(signal, name, want) != Selection::No)
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

    Ok(notes)
}

/// What one `--variables` name asks of one signal.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Selection {
    /// The name does not reach this signal.
    No,
    /// The name reaches this signal, and asks for exactly it.
    Whole,
    /// The name reaches one bit of this vector, which is kept whole.
    WholeBusForOneBit {
        /// The bus name, without its range, for the note.
        bus: String,
    },
}

/// Whether one `--variables` name selects this signal, and what it cost.
fn signal_selection(signal: &VcdSignal, column: &str, want: &str) -> Selection {
    if signal_matches(signal, column, want) {
        return Selection::Whole;
    }
    bus_selection(signal, want)
}

/// Whether one `--variables` name selects this signal by its exact spelling.
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

/// Whether one `--variables` name reaches this signal as a bus.
///
/// A vector variable is declared `name [msb:lsb]`, so its bare name is not one
/// of its spellings and neither is the closed-up `name[msb:lsb]` a user is at
/// least as likely to type. Both are resolved here through the same
/// [`split_bus_notation`] grammar core writes the reference with, and so is a
/// bit-select of an index the range actually covers.
fn bus_selection(signal: &VcdSignal, want: &str) -> Selection {
    if signal.kind != VcdSignalKind::Logic || signal.width <= 1 {
        return Selection::No;
    }
    for variable in &signal.variables {
        let (base, Some((msb, lsb))) = split_bus_notation(&variable.name) else {
            continue;
        };
        // The bus by name, or by its range in either spelling.
        let (wanted_base, wanted_range) = split_bus_notation(want);
        if wanted_base.eq_ignore_ascii_case(base)
            && (wanted_range.is_none() || wanted_range == Some((msb, lsb)))
        {
            return Selection::Whole;
        }
        // One of its bits. `split_bus_notation` deliberately leaves a
        // bit-select whole — it is the name of a conductor, not a range — so
        // the index is read here.
        if let Some(index) = bit_select_index(want, base)
            && (msb.min(lsb)..=msb.max(lsb)).contains(&index)
        {
            return Selection::WholeBusForOneBit {
                bus: base.to_string(),
            };
        }
    }
    Selection::No
}

/// The `k` of `base[k]`, when `want` is spelled that way for this base.
fn bit_select_index(want: &str, base: &str) -> Option<i64> {
    let trimmed = want.trim_end();
    let open = trimmed.rfind('[')?;
    if !trimmed[..open].trim_end().eq_ignore_ascii_case(base) {
        return None;
    }
    trimmed[open + 1..].strip_suffix(']')?.trim().parse().ok()
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
    use rspice_core::io::VcdTimescale;

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

    fn vector(name: &str, width: u32) -> VcdSignal {
        VcdSignal {
            identifier: String::new(),
            variables: vec![VcdVariable {
                scope: vec![EVENT_SCOPE.to_string()],
                name: name.to_string(),
            }],
            width,
            kind: VcdSignalKind::Logic,
            changes: Vec::new(),
        }
    }

    #[test]
    fn a_vector_answers_to_its_bus_name_and_to_its_range_in_either_spelling() {
        let bus = vector("x1.count [1:0]", 2);
        let column = "D(x1.count [1:0])";
        for want in [
            "x1.count",
            "X1.COUNT",
            "x1.count[1:0]",
            "x1.count [1:0]",
            "D(x1.count [1:0])",
        ] {
            assert_eq!(
                signal_selection(&bus, column, want),
                Selection::Whole,
                "'{want}' names this vector"
            );
        }

        // A different bus, and a range the variable does not declare, do not.
        for want in ["x1.other", "x1.count[3:0]", "x2.count"] {
            assert_eq!(
                signal_selection(&bus, column, want),
                Selection::No,
                "'{want}' does not name this vector"
            );
        }
    }

    #[test]
    fn naming_one_bit_of_a_vector_keeps_the_whole_vector_and_says_so() {
        let bus = vector("data [7:4]", 4);
        let column = "D(data [7:4])";
        for index in ["data[7]", "DATA[4]", "data[5]"] {
            assert_eq!(
                signal_selection(&bus, column, index),
                Selection::WholeBusForOneBit {
                    bus: "data".to_string()
                },
                "'{index}' is a bit of this vector"
            );
        }
        // An index outside the declared range is not one of its bits.
        for index in ["data[3]", "data[8]", "other[5]"] {
            assert_eq!(signal_selection(&bus, column, index), Selection::No);
        }

        let mut document = VcdDocument::new(VcdTimescale::ALL[11]);
        document.signals = vec![
            logic("clk", &[EVENT_SCOPE], &[(0, VcdBit::Zero)]),
            vector("data [7:4]", 4),
        ];
        let notes = select_and_clip(&mut document, &["data[5]".to_string()], None, None)
            .expect("one bit of a declared bus is a variable this dump has");
        assert_eq!(document.signals.len(), 1, "only the vector is kept");
        assert_eq!(document.signals[0].width, 4, "and it is kept whole");
        let [note] = notes.as_slice() else {
            panic!("widening the selection is stated once: {notes:?}");
        };
        assert!(
            note.contains("data[5]") && note.contains("data") && note.contains("whole"),
            "the note must name what was asked and what was kept: {note}"
        );
    }

    #[test]
    fn a_selection_that_asks_for_exactly_what_it_gets_says_nothing() {
        let mut document = VcdDocument::new(VcdTimescale::ALL[11]);
        document.signals = vec![
            logic("clk", &[EVENT_SCOPE], &[(0, VcdBit::Zero)]),
            vector("data [7:4]", 4),
        ];
        let notes = select_and_clip(&mut document, &["data".to_string()], None, None)
            .expect("the bus by name");
        assert_eq!(document.signals.len(), 1);
        assert!(notes.is_empty(), "nothing was widened: {notes:?}");
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

    /// A table cell is one bit's level, and nothing wider is packed into one.
    ///
    /// `load_vcd_table` expands vectors before it builds columns, so the wide
    /// cases below cannot arise from a parsed dump; they read unknown rather
    /// than as a number, so that a route that ever skipped the expansion
    /// writes an honest `x` instead of a plausible word.
    #[test]
    fn a_table_cell_is_one_bit_and_a_wider_change_is_not_packed() {
        use VcdBit::{HighImpedance, One, Unknown, Zero};
        assert_eq!(sample_value(&VcdValue::Logic(vec![Zero])), 0.0);
        assert_eq!(sample_value(&VcdValue::Logic(vec![One])), 1.0);
        assert_eq!(sample_value(&VcdValue::Logic(vec![Unknown])), UNKNOWN_LEVEL);
        assert_eq!(
            sample_value(&VcdValue::Logic(vec![HighImpedance])),
            UNKNOWN_LEVEL
        );
        assert_eq!(
            sample_value(&VcdValue::Logic(vec![One, Zero, One])),
            UNKNOWN_LEVEL
        );
        assert_eq!(sample_value(&VcdValue::Real(1.25)), 1.25);
    }

    /// Expanding is one function, so the dump route and the table route name
    /// and order a bus's bits identically.
    #[test]
    fn a_vector_expands_to_the_same_members_for_a_dump_and_for_a_table() {
        let mut document = VcdDocument::new(VcdTimescale::ALL[11]);
        document.signals = vec![
            logic("rst", &["top"], &[(0, VcdBit::Zero)]),
            VcdSignal {
                identifier: "#".to_string(),
                variables: vec![VcdVariable {
                    scope: vec!["top".to_string(), "core".to_string()],
                    name: "data [1:0]".to_string(),
                }],
                width: 2,
                kind: VcdSignalKind::Logic,
                changes: vec![
                    VcdChange {
                        tick: 0,
                        value: VcdValue::Logic(vec![VcdBit::Zero, VcdBit::Zero]),
                    },
                    VcdChange {
                        tick: 1,
                        value: VcdValue::Logic(vec![VcdBit::One, VcdBit::Zero]),
                    },
                ],
            },
        ];

        expand_vector_variables(Path::new("fixture.vcd"), &mut document)
            .expect("a well-formed vector expands");

        assert_eq!(
            document
                .signals
                .iter()
                .map(|signal| signal.variables[0].scoped_name())
                .collect::<Vec<_>>(),
            vec!["top.rst", "top.core.data[1]", "top.core.data[0]"],
            "the members stand where the vector stood, in its own scope"
        );
        assert_eq!(
            column_names(&document),
            vec!["D(rst)", "D(core.data[1])", "D(core.data[0])"],
            "and the table names them the same way it names any other signal"
        );
        // Only the bit that moved records a change at tick 1.
        assert_eq!(document.signals[1].changes.len(), 2);
        assert_eq!(document.signals[2].changes.len(), 1);
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
        let error = event_document(Path::new("run.vcd"), &[trace], &[], &[])
            .expect_err("half a femtosecond has no exact tick at any timescale");
        let message = error.to_string();
        assert!(
            message.contains("run.vcd") && message.contains("clk"),
            "the refusal must name the file and the node: {message}"
        );
    }
}
