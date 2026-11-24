//! A transient's results store read as a result file: the table a run of its deck writes with no
//! output request of its own (every saved column, and every event trace sampled on the rows), and
//! its event histories.
//!
//! A conversion to a rawfile, a delimited table, JSON or HDF5 reads the store a chunk at a time,
//! as the run that wrote it publishes its own outputs, decoding only the chunks a range reaches
//! and the columns it keeps; one the table would refuse, or that keeps a column that is not one
//! of the store's, reads the whole table and converts it as any result file.
//!
//! Every conversion checks the store first ([`store_notes`]): a segment that is not the run its
//! manifest names is refused, and a store that cannot show its run is whole converts with a note
//! saying why.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rspice_core::analysis::transient::TransientResult;
use rspice_core::execution::HeldEvents;
use rspice_results::stored_transient::{StoreEvents, StoredTransientRun, fill_rows};
use rspice_results_store::{
    FileIo, RunStatus, SegmentIo, SegmentReader, SegmentState, StopReason, StoreDir, StoreError,
};

use crate::cli::{CliError, ConvertArgs, InputFormat, NumericBound, OutputFormat};
use crate::commands::export_table::{ExportTable, JsonArray, scalar_table_taking_samples};
use crate::commands::run::{Column, ColumnStop, RowCursor, plan_columns};
use crate::commands::run_signals::ScalarSignal;
use crate::commands::waveform_io::{MIN_RESULT_SAMPLES, conversion_error};

/// The segment a store path names: a store directory's segment, or a segment file itself.
fn segment(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.join(StoreDir::segment_name(0))
    } else {
        path.to_path_buf()
    }
}

fn read_error(path: &Path, error: StoreError) -> CliError {
    conversion_error(
        path,
        format_args!("the results store cannot be read: {error}"),
    )
}

fn open_io(path: &Path) -> Result<FileIo, CliError> {
    let segment = segment(path);
    FileIo::open(&segment).map_err(|source| CliError::InputReadError {
        path: segment,
        source,
    })
}

/// What a conversion must say about the store at `path` before it reads it, or why it must
/// refuse it. A segment whose `END` names other results than its manifest records (a sync or a
/// copy that brought back an older segment) is refused. A store that cannot show its run is whole
/// converts with a note for each reason: its run stopped early, its segment has no sound `END` (a
/// run that did not finish, or an `END` damaged since), or it has no readable manifest to check
/// its segment against.
pub(crate) fn store_notes(path: &Path) -> Result<Vec<String>, CliError> {
    let reader = SegmentReader::open(open_io(path)?).map_err(|error| read_error(path, error))?;
    let rows = reader.rows();
    let mut notes = Vec::new();
    let held = match reader.state() {
        SegmentState::Complete { status, digest } => {
            if let RunStatus::Stopped(reason) = status {
                notes.push(format!(
                    "was written by a run that stopped early ({}): it holds the {rows} rows \
                     computed before the stop",
                    stopped_because(reason)
                ));
            }
            Some(*digest)
        }
        SegmentState::Recovered => {
            notes.push(match reader.end_damage() {
                Some(damage) => format!(
                    "has no sound END, so it was cut short or damaged since its run finished \
                     ({damage}): its {rows} rows are read back without the digest that checks \
                     them"
                ),
                None => format!(
                    "was written by a run that did not finish: the {rows} rows it made durable \
                     are read back without a digest to check them"
                ),
            });
            None
        }
    };
    if !path.is_dir() {
        return Ok(notes);
    }
    let unchecked = "so its segment is read unchecked against the run it was written for";
    match StoreDir::new(path).load_manifest() {
        Ok(Some(manifest)) => {
            let named = manifest.members().iter().find(|member| member.index == 0);
            match (named.and_then(|member| member.digest), held) {
                (Some(named), Some(held)) if named != held => {
                    return Err(conversion_error(
                        path,
                        format_args!(
                            "the results store's segment is not the run its manifest names: the \
                             manifest records results {named} and the segment holds {held}, as \
                             when a sync or a copy brings back an older segment"
                        ),
                    ));
                }
                _ if named.is_none() => {
                    notes.push(format!(
                        "has a manifest that does not name its segment, {unchecked}"
                    ));
                }
                _ => {}
            }
        }
        Ok(None) => notes.push(format!("has no manifest, {unchecked}")),
        Err(error) => notes.push(format!(
            "has a manifest that cannot be read ({error}), {unchecked}"
        )),
    }
    Ok(notes)
}

fn stopped_because(reason: &StopReason) -> String {
    match reason {
        StopReason::DiskFull => "the disk filled".to_owned(),
        StopReason::StorageBudget => "it reached its storage budget".to_owned(),
        StopReason::MemoryFull => "memory ran out".to_owned(),
        StopReason::Cancelled => "it was cancelled".to_owned(),
        StopReason::Timeout => "it timed out".to_owned(),
        StopReason::Failed(message) => format!("it failed: {message}"),
    }
}

/// Check the store at `path` before a conversion reads it: refuse it as [`store_notes`] does,
/// and say each of its notes once however many readers the conversion opens.
fn check_store(path: &Path) -> Result<(), CliError> {
    static NOTED: Mutex<BTreeSet<PathBuf>> = Mutex::new(BTreeSet::new());
    let notes = store_notes(path)?;
    let first = NOTED
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(path.to_path_buf());
    if first {
        for note in notes {
            crate::observability::diagnostic(
                "results_store_unverified",
                None,
                format_args!("Note: the results store {} {note}", path.display()),
            );
        }
    }
    Ok(())
}

/// The transient a store holds, with its event histories read back.
pub(crate) fn open(path: &Path) -> Result<StoredTransientRun<FileIo>, CliError> {
    check_store(path)?;
    StoredTransientRun::open(open_io(path)?).map_err(|error| read_error(path, error))
}

/// The signals of a table over the rows `shell` holds, as a run with no output request names
/// them.
fn signals(
    path: &Path,
    shell: &TransientResult,
    limits: rspice_core::ResourceLimits,
) -> Result<Vec<ScalarSignal>, CliError> {
    crate::commands::run_signals::transient_export_signals(
        &rspice_core::netlist::Netlist::default(),
        shell,
        limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| conversion_error(path, error))
}

/// The table of `signals` over `time`, taking the signals' samples.
fn table(signals: &mut [ScalarSignal], time: Vec<f64>) -> ExportTable {
    scalar_table_taking_samples(
        "transient",
        "Transient Analysis",
        "time",
        "time",
        time,
        signals,
    )
}

/// The store's whole transient as a table. The run read back is dropped before the table is
/// built, and the table takes the signals' samples, so at most two copies of the values are held
/// at once.
pub(crate) fn load_table(
    path: &Path,
    limits: rspice_core::ResourceLimits,
) -> Result<ExportTable, CliError> {
    let mut run = open(path)?;
    run.read_all().map_err(|error| read_error(path, error))?;
    let mut signals = signals(path, &run.shell, limits)?;
    let time = std::mem::take(&mut run.shell.time);
    drop(run);
    Ok(table(&mut signals, time))
}

/// Convert a store as [`load_table`]'s table converts, reading it a chunk at a time, to a
/// rawfile, a delimited table, JSON or HDF5. `Ok(false)` when it cannot, and the whole table
/// converts instead: another target, a column that is neither one of the store's nor sampled from
/// an event trace, a selection the table refuses, a time in the range or a kept value there the
/// table refuses as not finite, fewer rows than a table holds, or a range that keeps more than
/// one run of rows.
pub(crate) fn convert_streamed(
    args: &ConvertArgs,
    limits: rspice_core::ResourceLimits,
) -> Result<bool, CliError> {
    if !matches!(
        args.to,
        OutputFormat::Raw
            | OutputFormat::RawAscii
            | OutputFormat::Csv
            | OutputFormat::Tsv
            | OutputFormat::Json
            | OutputFormat::Hdf5
    ) {
        return Ok(false);
    }
    let path = args.input.as_path();
    crate::commands::waveform_io::validate_section_selector(
        path,
        InputFormat::Rsr,
        args.section.as_deref(),
    )?;
    check_store(path)?;
    let StoredTransientRun {
        mut shell,
        mut reader,
        targets,
        ..
    } = StoredTransientRun::open(open_io(path)?).map_err(|error| read_error(path, error))?;
    let rows = reader.rows();
    if rows < MIN_RESULT_SAMPLES as u64 {
        return Ok(false);
    }
    let probe = reader.chunk_spans().next().map_or(0, |span| span.1);
    fill_rows(&mut reader, &targets, &mut shell, 0, probe)
        .map_err(|error| read_error(path, error))?;
    let Ok(mut signals) = signals(path, &shell, limits) else {
        return Ok(false);
    };
    // A mixed run's `D(node)` and `E(node)` columns are sampled from its event histories.
    let events = HeldEvents {
        digital: &shell.digital_traces,
        real: &shell.real_traces,
    };
    let Some(columns) = plan_columns(&shell, &targets, &signals, Some(&events), false) else {
        return Ok(false);
    };
    for signal in &mut signals {
        signal.values = Vec::new();
    }
    let mut table = table(&mut signals, Vec::new());
    let Ok(keep) = table.selected_columns(&args.variables) else {
        return Ok(false);
    };
    let columns: Vec<Column> = match &keep {
        Some(keep) => columns
            .iter()
            .zip(keep)
            .filter_map(|(&column, &kept)| kept.then_some(column))
            .collect(),
        None => columns,
    };
    let lo = args
        .start
        .as_ref()
        .map_or(f64::NEG_INFINITY, NumericBound::value);
    let hi = args
        .stop
        .as_ref()
        .map_or(f64::INFINITY, NumericBound::value);
    // The table refuses a value it would write that is not finite; only the range's chunks and
    // the kept columns are read to find one.
    let slots = columns
        .iter()
        .filter_map(|column| match column {
            Column::Slot(slot) => Some(*slot),
            Column::Digital(_) | Column::Real(_) | Column::Derived(_) => None,
        })
        .collect::<Vec<_>>();
    let Some((first, points)) = kept_rows(path, &mut reader, &slots, lo, hi)? else {
        return Ok(false);
    };
    if let Some(keep) = keep {
        let mut keep = keep.into_iter();
        table.columns.retain(|_| keep.next().unwrap_or(false));
    }
    if points < MIN_RESULT_SAMPLES as u64 {
        return Ok(false);
    }
    let points = usize::try_from(points).map_err(|_| CliError::InternalError {
        message: "a results store holds more rows than this host can address".to_owned(),
    })?;
    let output = args.output.as_path();
    let dir = path.to_path_buf();
    crate::commands::publish::artifact(output, |writer| {
        let mut cursor = RowCursor::over(&dir, &mut reader, &columns, Some(&events), None, first);
        if args.to == OutputFormat::Hdf5 {
            write_hdf5(writer, output, &table, points, &mut cursor)?;
        } else if args.to == OutputFormat::Json {
            write_json(writer, output, &table, points, &mut cursor)?;
        } else {
            table.write_streamed_to(writer, output, args.to, points, |row, variable| {
                cursor.value(row, variable)
            })?;
        }
        cursor.finish(output)
    })
    .map_err(|error| crate::cli::map_atomic_output_error(output, error))?;
    Ok(true)
}

/// Write `table`'s HDF5 document, the one [`ExportTable::write`] writes, with each of its
/// `points` rows' values read from `cursor` a dataset at a time.
fn write_hdf5(
    writer: &mut dyn std::io::Write,
    output: &Path,
    table: &ExportTable,
    points: usize,
    cursor: &mut RowCursor<'_>,
) -> Result<(), CliError> {
    let data = crate::hdf5::table_data(table, None);
    crate::hdf5::write_streamed_to_writer(writer, &data, "table", points, |variable, out| {
        cursor
            .write_column(variable, points, out, output)
            .map_err(|stop| match stop {
                ColumnStop::NonFinite(_) => crate::commands::export_table::invalid_samples(output),
                ColumnStop::Failed(error) => error,
            })
    })
    .map_err(|failure| match failure {
        crate::hdf5::Hdf5StreamFailure::Hdf5(error) => crate::hdf5::map_output_error(output, error),
        crate::hdf5::Hdf5StreamFailure::Source(error) => error,
    })
}

/// Write `table`'s JSON document, the one [`ExportTable::write`] writes, with each of its
/// `points` rows' values read from `cursor` an array at a time.
fn write_json(
    writer: &mut dyn std::io::Write,
    output: &Path,
    table: &ExportTable,
    points: usize,
    cursor: &mut RowCursor<'_>,
) -> Result<(), CliError> {
    table.write_json_from(writer, output, points, &mut |array, row| {
        let variable = match array {
            JsonArray::Scale => 0,
            JsonArray::Values(column) => column + 1,
            JsonArray::Real(_) | JsonArray::Imag(_) => {
                return Err(CliError::InternalError {
                    message: "a streamed table has a column that is not dense real".to_owned(),
                });
            }
        };
        let value = cursor.value(row, variable);
        if value.is_finite() {
            Ok(Some(value))
        } else {
            Err(cursor
                .read_failure(output)
                .unwrap_or_else(|| crate::commands::export_table::invalid_samples(output)))
        }
    })
}

/// The rows `[lo, hi]` keeps, as `(first row, rows)`, having read the time of every row of each
/// chunk the range reaches, and the values of `slots` in the rows it keeps, and found them
/// finite; `None` when one is not, or when the rows kept are not one run of rows. A chunk whose
/// times all lie outside the range is not read.
fn kept_rows<I: SegmentIo>(
    path: &Path,
    reader: &SegmentReader<I>,
    slots: &[usize],
    lo: f64,
    hi: f64,
) -> Result<Option<(u64, u64)>, CliError> {
    let chunks = reader
        .chunk_spans()
        .zip(reader.chunk_x_ranges())
        .collect::<Vec<_>>();
    let (mut x, mut values) = (Vec::new(), Vec::new());
    let mut kept: Option<(u64, u64)> = None;
    let mut ended = false;
    for (chunk, &((start, _), (x_min, x_max))) in chunks.iter().enumerate() {
        // Both NaN (every time NaN) reaches the range, so the chunk is read and refused.
        if x_max < lo || x_min > hi {
            ended |= kept.is_some();
            continue;
        }
        reader
            .read_chunk_x(chunk, &mut x)
            .map_err(|error| read_error(path, error))?;
        if x.iter().any(|time| !time.is_finite()) {
            return Ok(None);
        }
        let mut window: Option<std::ops::Range<usize>> = None;
        for (offset, &time) in x.iter().enumerate() {
            let row = start + offset as u64;
            if time >= lo && time <= hi {
                match &mut kept {
                    _ if ended => return Ok(None),
                    Some((_, count)) => *count += 1,
                    None => kept = Some((row, 1)),
                }
                window.get_or_insert(offset..offset).end = offset + 1;
            } else if kept.is_some() {
                ended = true;
            }
        }
        let Some(window) = window else {
            continue;
        };
        for &slot in slots {
            reader
                .read_chunk_slot(chunk, slot, &mut values)
                .map_err(|error| read_error(path, error))?;
            if values[window.clone()]
                .iter()
                .any(|value| !value.is_finite())
            {
                return Ok(None);
            }
        }
    }
    Ok(Some(kept.unwrap_or((0, 0))))
}

/// Write a store's event dump reading its event histories from the store, as a dump of them read
/// back writes it. `Ok(false)` when it cannot, and the dump is built whole instead: a selection,
/// a range or expanded buses, or a run with no event history, whose dump comes from its table.
pub(crate) fn convert_vcd_streamed(args: &ConvertArgs) -> Result<bool, CliError> {
    if !args.variables.is_empty()
        || args.start.is_some()
        || args.stop.is_some()
        || args.expand_buses
    {
        return Ok(false);
    }
    let path = args.input.as_path();
    crate::commands::waveform_io::validate_section_selector(
        path,
        InputFormat::Rsr,
        args.section.as_deref(),
    )?;
    check_store(path)?;
    let run = StoredTransientRun::open_leaving_events(open_io(path)?)
        .map_err(|error| read_error(path, error))?;
    let shell = &run.shell;
    if shell.digital_traces.is_empty() && shell.real_traces.is_empty() {
        return Ok(false);
    }
    let reader = SegmentReader::open(open_io(path)?).map_err(|error| read_error(path, error))?;
    let stream = crate::commands::vcd_io::event_stream(
        path,
        StoreEvents::new(reader, shell),
        &shell.digital_traces,
        &shell.real_traces,
        &shell.digital_buses,
    )?;
    let output = args.output.as_path();
    crate::commands::publish::artifact(output, |writer| {
        crate::commands::vcd_io::write_event_stream(writer, output, &stream)
    })
    .map_err(|error| crate::cli::map_atomic_output_error(output, error))?;
    Ok(true)
}

/// A store's event histories as a dump's traces, or `None` when it has none.
pub(crate) fn event_traces(
    path: &Path,
) -> Result<Option<rspice_core::execution::RawEventTraces>, CliError> {
    let run = open(path)?;
    let shell = run.shell;
    if shell.digital_traces.is_empty() && shell.real_traces.is_empty() {
        return Ok(None);
    }
    Ok(Some(rspice_core::execution::RawEventTraces {
        digital_traces: shell.digital_traces,
        digital_buses: shell.digital_buses,
        real_traces: shell.real_traces,
    }))
}

#[cfg(test)]
mod tests;
