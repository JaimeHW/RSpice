//! A transient the command line writes to an on-disk results store.
//!
//! The run hands every accepted point to a [`StoreSink`] that writes one segment of a store
//! directory, so its waveforms never sit in memory: the engine returns everything else, and the
//! outputs read the rows back from the segment chunk by chunk. A run published to `out.raw` leaves
//! its store beside it as `out.rsr/`, as Spectre leaves its PSF directory; `--no-store` deletes it
//! once the output is written, and a run with no output deletes it when it ends.
//!
//! The rows stream straight from the store to a rawfile or a delimited table when the deck's
//! outputs are the store's own columns over every accepted step. Any other output reads the whole
//! run back into memory first and goes through the in-memory path unchanged.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::AbortSignal;
use rspice_core::analysis::{
    FourierAnalysis, FourierConfig, FourierError, FourierMethod, FourierRecord, MeasureResult,
    TranMeasureStream,
};
use rspice_core::engine::{
    AcceptedTransientRows, ScheduledTransientCheckpoint, SinkTransient, TransientCheckpoint,
    TransientEnd, TransientFourierResult, TransientOutputRow, TransientOutputWalk,
    TransientPostResults, TransientResult, TransientStartupMode, TransientState,
};
use rspice_core::execution::{EventPoints, EventProjectionError, EventSource, HeldEvents};
use rspice_core::netlist::{
    AnalysisCommand, OutputAnalysisKind, OutputDirectiveKind, OutputScheduleError,
    XyceOutputIntervalSchedule,
};
use rspice_core::result_sink::{SinkReceipt, SinkStop};
use rspice_core::xspice::DigitalValue;
use rspice_results::stored_transient::{
    SlotTarget, StoreEvents, clear_rows, fill_event_traces, fill_impulses, fill_rows, fill_span,
    first_non_finite, instance_parameters, slot_name, slot_targets,
};
use rspice_results_store::{
    FileIo, FreeSpace, Manifest, Member, SegmentReader, SegmentState, StoreDir, StoreError,
    StoreSink, WatchedIo, WatchedSink,
};

use super::context::RunContext;
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::ScalarSignal;

/// Store directories this process has opened, so concurrent runs never share one.
static NEXT_STORE: AtomicU64 = AtomicU64::new(0);

/// A transient whose rows are in a store: the result holds everything else.
pub(super) struct StoredTransient {
    /// The run's result with its time and waveform columns empty.
    pub(super) shell: TransientResult,
    pub(super) rows: StoredRows,
}

/// The rows of one transient in its store.
pub(super) struct StoredRows {
    store: RunStore,
    reader: SegmentReader<FileIo>,
    targets: Vec<SlotTarget>,
    /// `(device, parameter)` of each instance-parameter column.
    parameters: Vec<(String, String)>,
    receipt: SinkReceipt,
    /// The storage filled and the run ended at its last whole chunk.
    stopped: Option<SinkStop>,
    /// The digital and real event histories are in the shell's traces, not only in the store.
    events_held: bool,
}

/// A store directory, under a private name while its run is open.
struct RunStore {
    dir: PathBuf,
    /// The store beside the output that a resumed run appended to where it was: it is never
    /// moved, and never deleted, since it held the earlier segments' rows before the run began.
    in_place: bool,
    /// The netlist fingerprint of the checkpoints the store's run wrote, which its manifest names.
    fingerprint: Option<u64>,
}

/// One segment of a checkpointed transient.
#[derive(Clone, Copy)]
pub(super) enum Segment<'c> {
    /// The run to its stop, ending with its checkpoint there.
    Whole,
    /// The deck's run over its own window, to `tstop` with `max_step`, interrupted at the stop
    /// the command line gave it.
    Interrupted { tstop: f64, max_step: f64 },
    /// The run continued from a checkpoint.
    Resumed(&'c TransientCheckpoint),
}

/// A store open for a run: its directory, its segment file and the sink writing it.
struct OpenStore {
    store: RunStore,
    segment: PathBuf,
    sink: StoreSink<WatchedIo<FileIo>>,
    free: FreeSpace,
}

/// Run the transient into a new store.
///
/// Before the first row, the store's volume must have room for at least the rows the run cannot
/// avoid writing (`tstop` over the largest step, `max_step`), or the run is refused before it
/// solves; a run whose likely size (`tstop` over the smaller of `max_step` and `tstep`) exceeds
/// the free space only warns. While it runs, a write that would leave less free space than the
/// segment needs to close stops it at that accepted point: the segment closes over every chunk
/// written, and the run's outputs are written for what it kept before it ends as stopped.
pub(super) fn run(
    ctx: &RunContext<'_>,
    tstop: f64,
    tstep: f64,
    max_step: f64,
    startup_mode: TransientStartupMode,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<StoredTransient, CliError> {
    let span = StoreSpan {
        start: 0.0,
        tstop,
        tstep,
        max_step,
    };
    run_with(ctx, span, None, |sink| {
        ctx.engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                ctx.netlist,
                tstop,
                max_step,
                startup_mode,
                abort,
                sink,
            )
            .map(|run| (run, ()))
    })
    .map(|(stored, ())| stored)
}

/// Run one segment of a checkpointed transient into a store, as [`run`] does, and return the
/// checkpoint it ends with. A resumed segment appends to the store of the run it continues, as
/// [`run_with`] says.
pub(super) fn run_segment(
    ctx: &RunContext<'_>,
    segment: Segment<'_>,
    (tstop, tstep, max_step): (f64, f64, f64),
    startup_mode: TransientStartupMode,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<(StoredTransient, TransientEnd), CliError> {
    let (netlist, engine) = (ctx.netlist, ctx.engine);
    let span = |start, max_step| StoreSpan {
        start,
        tstop,
        tstep,
        max_step,
    };
    match segment {
        Segment::Whole => run_with(ctx, span(0.0, max_step), None, |sink| {
            engine.run_tran_checkpointed_into_sink_with_startup_mode_and_abort(
                netlist,
                (tstop, max_step),
                startup_mode,
                abort,
                sink,
            )
        }),
        Segment::Interrupted {
            tstop: deck_stop,
            max_step,
        } => run_with(ctx, span(0.0, max_step), None, |sink| {
            engine.run_tran_checkpointed_at_into_sink_with_startup_mode_and_abort(
                netlist,
                (deck_stop, max_step),
                tstop,
                startup_mode,
                abort,
                sink,
            )
        }),
        Segment::Resumed(checkpoint) => run_with(
            ctx,
            span(checkpoint.time, max_step),
            Some(checkpoint),
            |sink| {
                engine.run_tran_resume_into_sink_with_abort(
                    netlist,
                    checkpoint,
                    (tstop, max_step),
                    abort,
                    sink,
                )
            },
        ),
    }
}

/// What a run writes to its store, for the disk preflight: rows from `start` to `tstop`, none
/// further apart than `max_step` and most no further apart than `tstep`.
#[derive(Debug, Clone, Copy)]
pub(super) struct StoreSpan {
    pub(super) start: f64,
    pub(super) tstop: f64,
    pub(super) tstep: f64,
    pub(super) max_step: f64,
}

impl StoreSpan {
    /// The fewest rows the run writes, none further apart than `max_step`, and the rows it likely
    /// writes. A print step shortens the likely step only when the deck names one: `.TRAN 0 1`
    /// names none, and leaves `max_step` the only estimate.
    fn rows(&self) -> (f64, f64) {
        let rows = self.tstop - self.start;
        let likely_step = if self.tstep > 0.0 {
            self.max_step.min(self.tstep)
        } else {
            self.max_step
        };
        (rows / self.max_step, rows / likely_step)
    }
}

/// The warning for a run that likely writes `likely` bytes to a volume with `free` bytes free.
/// [`WatchedSink::bytes`] saturates at `u64::MAX` past any size it can count, and a count it
/// could not make is no estimate: the run then says nothing.
fn space_warning(likely: u64, free: u64) -> Option<String> {
    (likely < u64::MAX).then(|| {
        format!(
            "Warning: this transient likely writes about {likely} bytes of results and their \
             volume has {free} bytes free; it stops, keeping what it wrote, if the volume fills"
        )
    })
}

/// What a run into a store returns besides its rows: the checkpoints it wrote name the run, so
/// that a run resumed from one of them can find its store.
pub(super) trait StoreRunEnd {
    /// The netlist fingerprint of the checkpoints the run wrote, if it wrote any.
    fn checkpoint_fingerprint(&self) -> Option<u64>;
}

impl StoreRunEnd for () {
    fn checkpoint_fingerprint(&self) -> Option<u64> {
        None
    }
}

impl StoreRunEnd for TransientEnd {
    fn checkpoint_fingerprint(&self) -> Option<u64> {
        match self {
            TransientEnd::Checkpoint(checkpoint) => Some(checkpoint.netlist_fingerprint),
            TransientEnd::Finished(_) => None,
        }
    }
}

impl StoreRunEnd for Vec<ScheduledTransientCheckpoint> {
    fn checkpoint_fingerprint(&self) -> Option<u64> {
        self.first()
            .map(|scheduled| scheduled.checkpoint.netlist_fingerprint)
    }
}

impl StoreRunEnd for Vec<TransientState> {
    fn checkpoint_fingerprint(&self) -> Option<u64> {
        None
    }
}

/// Run a transient into a store through `run`, which hands the engine the store's sink.
///
/// A run resumed from `resume` appends to the store beside its output when that store holds the
/// run the checkpoint continues: its manifest names the checkpoint's netlist fingerprint and its
/// segment has a row at the checkpoint's time. The rows after that row came from a run that went
/// on past it, and are dropped. The store then holds the whole run from its start, and so do the
/// run's outputs. With `--no-store`, beside no such store, or beside another (a note says why), a
/// resumed run writes a store of its own from the checkpoint on.
pub(super) fn run_with<T: StoreRunEnd>(
    ctx: &RunContext<'_>,
    span: StoreSpan,
    resume: Option<&TransientCheckpoint>,
    run: impl FnOnce(
        &mut dyn rspice_core::result_sink::ResultSink,
    ) -> Result<(SinkTransient, T), rspice_core::SimulationError>,
) -> Result<(StoredTransient, T), CliError> {
    let appended = match resume {
        Some(checkpoint) if !ctx.args.no_store => reopen_beside_output(ctx, checkpoint),
        _ => None,
    };
    let OpenStore {
        mut store,
        segment,
        sink: inner,
        free,
    } = match appended {
        Some(open) => open,
        None => create_store(ctx)?,
    };
    let (rows_needed, rows_likely) = span.rows();
    let mut sink = WatchedSink::new(inner, free, rows_needed, rows_likely).with_space_warning(
        Box::new(|likely, free| {
            if let Some(warning) = space_warning(likely, free) {
                crate::observability::diagnostic("results_store_space", None, warning);
            }
        }),
    );
    let (
        SinkTransient {
            result: shell,
            receipt,
            stopped,
        },
        end,
    ) = match run(&mut sink) {
        Ok(done) => done,
        Err(source) => return Err(stopped_run(ctx, store, &segment, sink, source)),
    };
    if !store.in_place {
        store.fingerprint = end.checkpoint_fingerprint();
    }
    let opened = sink
        .into_parts()
        .0
        .into_io()
        .map(WatchedIo::into_inner)
        .ok_or_else(|| StoreError::Io {
            kind: std::io::ErrorKind::Other,
            message: "the results segment was not closed".to_owned(),
        })
        .and_then(SegmentReader::open)
        .and_then(|reader| {
            store.save_manifest(&reader)?;
            Ok(reader)
        });
    let mut reader = match opened {
        Ok(reader) => reader,
        Err(error) => {
            let failure = store_error(&store.dir, error);
            store.discard();
            return Err(failure);
        }
    };
    let targets = match slot_targets(&reader, &shell) {
        Ok(targets) => targets,
        Err(message) => {
            drop(reader);
            store.discard();
            return Err(CliError::InternalError { message });
        }
    };
    let mut shell = shell;
    // The digital and real event histories stay in the store until an output reads them there
    // or asks for them in memory ([`StoredTransient::hold_events`]).
    if let Err(error) = fill_impulses(&mut reader, &mut shell) {
        let failure = store_error(&store.dir, error);
        drop(reader);
        store.discard();
        return Err(failure);
    }
    let stored = StoredTransient {
        shell,
        rows: StoredRows {
            parameters: instance_parameters(&reader),
            store,
            reader,
            targets,
            receipt,
            stopped,
            events_held: false,
        },
    };
    Ok((stored, end))
}

/// A new store for a run, under a private name beside its output.
fn create_store(ctx: &RunContext<'_>) -> Result<OpenStore, CliError> {
    let store = RunStore::create(ctx.output.as_deref())?;
    let segment = store.dir.join(StoreDir::segment_name(0));
    let io = match FileIo::create(&segment) {
        Ok(io) => io,
        Err(error) => {
            store.discard();
            return Err(CliError::output_error(&segment, error));
        }
    };
    let free = free_space(&store.dir);
    Ok(OpenStore {
        store,
        segment,
        sink: StoreSink::new(WatchedIo::new(io, free.clone())),
        free,
    })
}

/// The store beside a resumed run's output, reopened to take the run's rows after the
/// checkpoint's, when it holds the run `checkpoint` continues. Any other store there is left for
/// the run's own store to replace, with a note.
fn reopen_beside_output(
    ctx: &RunContext<'_>,
    checkpoint: &TransientCheckpoint,
) -> Option<OpenStore> {
    let output = ctx.next_output_path("tran")?;
    let dir = store_path_beside(&output);
    let segment = dir.join(StoreDir::segment_name(0));
    let manifest = StoreDir::new(&dir).load_manifest();
    if matches!(manifest, Ok(None)) || !segment.is_file() {
        return None;
    }
    let note = |why: &dyn std::fmt::Display| {
        crate::observability::diagnostic(
            "results_store_not_resumed",
            None,
            format_args!(
                "Note: the resumed transient writes a new results store in place of {}, which \
                 {why}",
                dir.display()
            ),
        );
    };
    let fingerprint = Some(checkpoint.netlist_fingerprint);
    match manifest {
        Ok(Some(manifest))
            if manifest.members().iter().any(|member| {
                member.index == 0 && member.checkpoint_fingerprint == fingerprint
            }) => {}
        Ok(_) => {
            note(&"is not the store of the run the checkpoint continues");
            return None;
        }
        Err(error) => {
            note(&format_args!("cannot be read: {error}"));
            return None;
        }
    }
    let free = free_space(&dir);
    let reopened = FileIo::open_writable(&segment)
        .map_err(StoreError::from)
        .and_then(|io| StoreSink::appending(WatchedIo::new(io, free.clone()), checkpoint.time));
    match reopened {
        Ok(sink) => Some(OpenStore {
            store: RunStore {
                dir,
                in_place: true,
                fingerprint,
            },
            segment,
            sink,
            free,
        }),
        Err(StoreError::NoRowAt { .. }) => {
            note(&"has no row at the checkpoint's time");
            None
        }
        Err(error) => {
            note(&format_args!("cannot be reopened: {error}"));
            None
        }
    }
}

/// What each `.FOUR` card on a transient reads of its whole run besides the rows of its window:
/// the card's index among the deck's `.FOUR` cards, and a record per operand in the order the
/// operands resolve.
pub(super) type FourierRecords = Vec<(usize, Vec<FourierRecord>)>;

impl StoredTransient {
    /// Read the run's digital and real event histories from the store into the shell's traces,
    /// once, for what reads them in memory: a read-back, a measurement, a `.FOUR` card, and the
    /// caller of a run it hands back.
    pub(super) fn hold_events(&mut self) -> Result<(), CliError> {
        if !self.rows.events_held {
            fill_event_traces(&mut self.rows.reader, &mut self.shell)
                .map_err(|error| store_error(&self.rows.store.dir, error))?;
            self.rows.events_held = true;
        }
        Ok(())
    }

    /// Read every row back into the result and complete the outputs that read whole waveforms
    /// (`.FFT`, lead power), after which it equals the in-memory run's.
    pub(super) fn read_back(&mut self, ctx: &RunContext<'_>, tstop: f64) -> Result<(), CliError> {
        self.hold_events()?;
        let rows = self.rows.rows();
        self.rows.fill(&mut self.shell, 0, rows)?;
        let parameters = std::mem::take(&mut self.rows.parameters);
        for (slot, &target) in self.rows.targets.iter().enumerate() {
            let SlotTarget::Parameter(index) = target else {
                continue;
            };
            let values = usize::try_from(rows)
                .map_err(|_| CliError::InternalError {
                    message: "a transient store holds more rows than this host can address"
                        .to_owned(),
                })
                .and_then(|count| {
                    self.rows
                        .reader
                        .read_slot(slot, 0, count)
                        .map_err(|error| store_error(&self.rows.store.dir, error))
                })?;
            let (device, parameter) = &parameters[index];
            self.shell
                .adopt_instance_parameter(device, parameter, values);
        }
        self.rows.parameters = parameters;
        ctx.engine
            .finish_transient_from_rows(
                ctx.netlist,
                &mut self.shell,
                tstop,
                &crate::abort::ProcessAbort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some("Transient".to_string()),
            })
    }

    /// How the outputs can read this run straight from its store, or `None` when they need it in
    /// memory.
    ///
    /// They can when the deck asserts nothing, checks no `.VEC` file's expected outputs, and
    /// asks for a rawfile, a delimited table, HDF5, an event dump or nothing (a `.PRINT TRAN`
    /// card's Xyce `.prn` table is laid out from the store too, and an output schedule's rows
    /// are projected from the store's as they are read); when
    /// every output signal is one of the store's columns, value for value, or a column sampled
    /// from an event trace (`D(node)`, `E(node)`); and when its `.MEASURE TRAN` statements, if
    /// any, read only the store's columns in forms measured in one pass over the rows. The
    /// signals are checked by projecting the first chunk exactly as the whole run would be,
    /// and the statements by planning them against the same chunk; they are then measured from
    /// the store.
    pub(super) fn stream_plan(
        &mut self,
        ctx: &RunContext<'_>,
        tstart: f64,
    ) -> Result<Option<StreamPlan>, CliError> {
        let netlist = ctx.netlist;
        let format_streams = ctx.output.is_none()
            || matches!(
                ctx.format,
                OutputFormat::Raw
                    | OutputFormat::RawAscii
                    | OutputFormat::Csv
                    | OutputFormat::Tsv
                    | OutputFormat::Hdf5
                    | OutputFormat::Vcd
            );
        let rows = self.rows.rows();
        let parameters = !self.rows.parameters.is_empty();
        let whole = if !format_streams {
            Some("its output format is written whole")
        } else if parameters {
            Some("it saves instance parameters")
        } else if !ctx.engine.transient_rows_are_whole(netlist) {
            Some("its rows are not the run's whole trajectory")
        } else if netlist
            .measurements
            .iter()
            .any(|statement| statement.analysis.eq_ignore_ascii_case("TRAN_CONT"))
        {
            Some("it has TRAN_CONT measures")
        } else if !netlist.assertions.is_empty() {
            Some("it has .ASSERT cards")
        } else if !netlist.vector_files.is_empty() {
            Some("it has .VEC cards")
        } else if rows == 0 {
            Some("it has no rows")
        } else if !(tstart.is_finite() && tstart >= 0.0) {
            Some("its start time is not a finite non-negative time")
        } else {
            None
        };
        if let Some(why) = whole {
            log::debug!("the transient is read back whole: {why}");
            return Ok(None);
        }
        let probe_rows = self
            .rows
            .reader
            .chunk_spans()
            .next()
            .map_or(0, |span| span.1);
        self.rows.fill(&mut self.shell, 0, probe_rows as u64)?;
        // A column sampled from an event trace reads the run's event histories; an event dump
        // writes no table, so it reads them alone.
        let event_dump = ctx.output.is_some() && matches!(ctx.format, OutputFormat::Vcd);
        let events = if event_dump
            || !self.shell.digital_traces.is_empty()
            || !self.shell.real_traces.is_empty()
        {
            Some(self.rows.events(&self.shell)?)
        } else {
            None
        };
        let plan = probe_plan(ctx, &self.shell, &self.rows.targets, events.as_ref());
        drop(events);
        let plan = plan.and_then(|plan| {
            let Some(mut plan) = plan else {
                return Ok(None);
            };
            if plan
                .columns
                .iter()
                .any(|column| matches!(column, Column::Derived(_)))
            {
                // An event dump writes no rows; a table or a `.prn` file reads them staged.
                let stage = !event_dump
                    && (ctx.output.is_some()
                        || super::xyce_files::prints(ctx, OutputAnalysisKind::Tran));
                let Some(derived) = self.derive(ctx, &plan, stage)? else {
                    return Ok(None);
                };
                plan.derived = Some(derived);
            }
            if event_dump {
                plan.columns.clear();
            }
            let measures = netlist
                .measurements
                .iter()
                .any(|statement| statement.analysis.eq_ignore_ascii_case("TRAN"));
            if measures {
                self.hold_events()?;
                let Some(measurements) = self.measure(ctx)? else {
                    log::debug!("the transient's .MEASURE TRAN statements read it back whole");
                    return Ok(None);
                };
                log::debug!(
                    "the transient's {} .MEASURE TRAN statements were measured from its store",
                    measurements.len()
                );
                plan.measurements = Some(measurements);
            }
            Ok(Some(plan))
        });
        self.rows.clear(&mut self.shell);
        plan
    }

    /// Evaluate `plan`'s derived columns over every chunk of the store exactly as the whole
    /// run's projection evaluates them, the chunk's rows in the shell, and stage their values
    /// in a file for the outputs that write them when `stage` is set. `None` when the run is
    /// read back whole for them instead, which then reports or writes them as it does: a
    /// chunk's projection fails or names other columns than the first chunk's, a derived value
    /// is not finite, or the staged file cannot be written.
    fn derive(
        &mut self,
        ctx: &RunContext<'_>,
        plan: &StreamPlan,
        stage: bool,
    ) -> Result<Option<DerivedColumns>, CliError> {
        let abort = &crate::abort::ProcessAbort;
        let signals = plan
            .columns
            .iter()
            .enumerate()
            .filter(|(_, column)| matches!(column, Column::Derived(_)))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let whole = |why: &dyn std::fmt::Display| {
            log::debug!("the transient is read back whole: {why}");
            Ok(None)
        };
        let mut derived = DerivedColumns {
            path: stage.then(|| {
                std::env::temp_dir().join(format!(
                    ".rspice-derived-{}-{}",
                    std::process::id(),
                    NEXT_STORE.fetch_add(1, Ordering::Relaxed)
                ))
            }),
            offsets: Vec::new(),
        };
        let mut out = match &derived.path {
            Some(path) => match std::fs::File::create(path) {
                Ok(file) => Some(std::io::BufWriter::new(file)),
                Err(error) => return whole(&format_args!("its derived columns: {error}")),
            },
            None => None,
        };
        let spans = self.rows.reader.chunk_spans().collect::<Vec<_>>();
        let mut offset = 0_u64;
        let mut bytes = Vec::new();
        for &(first, count) in &spans {
            self.rows.fill(&mut self.shell, first, count as u64)?;
            let chunk = match crate::commands::run_signals::transient_export_signals(
                ctx.netlist,
                &self.shell,
                ctx.transient_limits(),
                abort,
            ) {
                Ok(chunk) => chunk,
                Err(rspice_core::SimulationError::Aborted) => {
                    return Err(super::cancellation_cli_error(ctx.args.timeout));
                }
                Err(_) => return whole(&"a chunk of its signals does not project"),
            };
            if chunk.len() != plan.signals.len()
                || chunk
                    .iter()
                    .zip(&plan.signals)
                    .any(|(chunk, signal)| chunk.display_name != signal.display_name)
            {
                return whole(&"a chunk of its signals projects other columns");
            }
            derived.offsets.push(offset);
            for &index in &signals {
                let values = &chunk[index].values;
                if values.len() != count {
                    return whole(&"a chunk of its signals projects other rows");
                }
                if values.iter().any(|value| !value.is_finite()) {
                    return whole(&format_args!(
                        "{} has a value that is not finite",
                        chunk[index].display_name
                    ));
                }
                if let Some(out) = &mut out {
                    bytes.clear();
                    for value in values {
                        bytes.extend_from_slice(&value.to_le_bytes());
                    }
                    if let Err(error) = std::io::Write::write_all(out, &bytes) {
                        return whole(&format_args!("its derived columns: {error}"));
                    }
                }
                offset += (count * std::mem::size_of::<f64>()) as u64;
            }
        }
        if let Some(Err(error)) = out.map(|mut out| std::io::Write::flush(&mut out)) {
            return whole(&format_args!("its derived columns: {error}"));
        }
        Ok(Some(derived))
    }

    /// The deck's `.MEASURE TRAN` results, measured in passes over the store's rows (after one
    /// over their axis), planned with the first chunk's rows in the shell; `None` when a
    /// statement needs the run in memory.
    fn measure(&mut self, ctx: &RunContext<'_>) -> Result<Option<Vec<MeasureResult>>, CliError> {
        let failed = |source| CliError::CoreSimulationError {
            source,
            analysis: Some("Transient measurement projection".to_string()),
        };
        let abort = &crate::abort::ProcessAbort;
        let columns = self
            .rows
            .targets
            .iter()
            .map(|&target| stream_column(&self.shell, target))
            .collect::<Vec<_>>();
        let Some(mut stream) =
            TranMeasureStream::plan(ctx.netlist, &self.shell, &columns, abort).map_err(failed)?
        else {
            return Ok(None);
        };
        let rows = &mut self.rows;
        let spans = rows.reader.chunk_spans().collect::<Vec<_>>();
        let chunks = spans.len();
        let dir = rows.store.dir.clone();
        let read = |error| store_error(&dir, error);
        let mut x = Vec::new();
        for chunk in 0..chunks {
            if abort.is_aborted() {
                return Err(failed(rspice_core::SimulationError::Aborted));
            }
            rows.reader.read_chunk_x(chunk, &mut x).map_err(read)?;
            stream.axis(&x);
        }
        let slots = stream.columns().to_vec();
        let mut values = vec![Vec::new(); slots.len()];
        loop {
            for (chunk, &(first, count)) in spans.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(failed(rspice_core::SimulationError::Aborted));
                }
                rows.reader.read_chunk_x(chunk, &mut x).map_err(read)?;
                for (&slot, values) in slots.iter().zip(&mut values) {
                    rows.reader
                        .read_chunk_slot(chunk, slot, values)
                        .map_err(read)?;
                }
                // An expression, a differential voltage or an alias is derived from the
                // chunk's rows of every column.
                if stream.derives() {
                    rows.fill(&mut self.shell, first, count as u64)?;
                    if !stream
                        .derive(ctx.netlist, &self.shell, abort)
                        .map_err(failed)?
                    {
                        return Ok(None);
                    }
                }
                let columns = values.iter().map(Vec::as_slice).collect::<Vec<_>>();
                stream.rows(&x, &columns);
            }
            if !stream.next_pass() {
                break;
            }
        }
        self.rows.fill_span(&mut self.shell)?;
        stream
            .finish(ctx.netlist, &self.shell, abort)
            .map_err(failed)
    }

    /// Leave in the shell only the rows this transient's `.FOUR` cards read, the ones around the
    /// union of their windows, and return what their operands read of the rest. One pass over
    /// the store evaluates every operand chunk by chunk into its record; a second fills the
    /// rows. With no record that has a window, the shell holds the first two rows, enough for
    /// each card to report its refusal as the whole run would.
    ///
    /// Reads every row back instead, as [`Self::read_back`] does, and returns `None`, when an
    /// operand's chunk is not its share of the whole run: the run has impulses a spectrum counts,
    /// an operand reads other rows (`sdt`), or a chunk's evaluation fails, which the whole run
    /// then reports.
    pub(super) fn fourier_window(
        &mut self,
        ctx: &RunContext<'_>,
        tstop: f64,
    ) -> Result<Option<FourierRecords>, CliError> {
        let netlist = ctx.netlist;
        let abort = &crate::abort::ProcessAbort;
        self.hold_events()?;
        let indices = ctx.fourier_cards_for_current_transient()?;
        let mut whole = self.shell.voltage_impulses.is_some()
            || self.shell.current_impulses.is_some()
            || indices.iter().any(|&index| {
                rspice_core::analysis::tran_four_operands_read_other_rows(netlist, index)
            });
        let mut cards: FourierRecords = indices
            .into_iter()
            .map(|index| (index, Vec::new()))
            .collect();
        let spans = self.rows.reader.chunk_spans().collect::<Vec<_>>();
        'chunks: for &(first, count) in spans.iter().take_while(|_| !whole) {
            self.rows.fill(&mut self.shell, first, count as u64)?;
            for (index, records) in &mut cards {
                let columns =
                    match rspice_core::analysis::evaluate_tran_four_output_requests_with_abort(
                        netlist,
                        &self.shell,
                        *index,
                        ctx.transient_limits(),
                        abort,
                    ) {
                        Ok(columns) => columns,
                        Err(rspice_core::SimulationError::Aborted) => {
                            return Err(super::cancellation_cli_error(ctx.args.timeout));
                        }
                        Err(_) => {
                            whole = true;
                            break 'chunks;
                        }
                    };
                if records.is_empty() {
                    records.resize_with(columns.len(), FourierRecord::new);
                }
                if records.len() != columns.len() {
                    whole = true;
                    break 'chunks;
                }
                for (record, (_, _, values)) in records.iter_mut().zip(&columns) {
                    record.push(&self.shell.time, values);
                }
            }
        }
        if whole {
            self.read_back(ctx, tstop)?;
            return Ok(None);
        }
        let method = FourierMethod::for_dialect(netlist.parse_dialect());
        let fourier_cards = netlist
            .analyses
            .iter()
            .filter(|card| matches!(card, AnalysisCommand::Four { .. }))
            .collect::<Vec<_>>();
        let mut window: Option<(f64, f64)> = None;
        for (index, records) in &cards {
            let Some(config) = fourier_cards
                .get(*index)
                .and_then(|card| FourierConfig::try_from(*card).ok())
            else {
                continue;
            };
            let analysis = FourierAnalysis::new(config.with_method(method));
            for record in records {
                if let Ok((start, stop)) = analysis.record_window(record) {
                    window = Some(window.map_or((start, stop), |(low, high)| {
                        (low.min(start), high.max(stop))
                    }));
                }
            }
        }
        let rows = self.rows.rows();
        let (first, count) = match window {
            // A record with a window has finite, increasing times, so each chunk's share of
            // the rows before the window's start, and through its stop, adds up to the run's.
            Some((start, stop)) => {
                let (mut before, mut through) = (0_u64, 0_u64);
                let mut x = Vec::new();
                for chunk in 0..spans.len() {
                    self.rows
                        .reader
                        .read_chunk_x(chunk, &mut x)
                        .map_err(|error| store_error(&self.rows.store.dir, error))?;
                    before += x.partition_point(|&time| time < start) as u64;
                    through += x.partition_point(|&time| time <= stop) as u64;
                }
                let first = before.saturating_sub(1);
                let last = through.min(rows.saturating_sub(1));
                (first, last.saturating_sub(first) + 1)
            }
            None => (0, rows.min(2)),
        };
        self.rows.fill(&mut self.shell, first, count)?;
        Ok(Some(cards))
    }

    /// The deck's post-processing products measured from the store, equal to the ones
    /// [`rspice_core::engine::evaluate_transient_post_results`] evaluates on the run read back:
    /// its `.FOUR` spectra from the rows around their windows, then its `.MEASURE TRAN`
    /// statements in passes over the rows.
    ///
    /// Reads every row back instead, as [`Self::read_back`] does, and returns `None`, when a
    /// product needs the run in memory: a `.FFT` card, lead power, an instance parameter, a
    /// continuous measurement, an assertion, a `.VEC` check, a statement measured only whole, or
    /// a `.FOUR` card that only the whole run evaluates or that another transient owns.
    pub(super) fn post_results(
        &mut self,
        ctx: &RunContext<'_>,
        tstop: f64,
    ) -> Result<Option<TransientPostResults>, CliError> {
        let netlist = ctx.netlist;
        let abort = &crate::abort::ProcessAbort;
        let failed = |source| CliError::CoreSimulationError {
            source,
            analysis: Some("Transient".to_string()),
        };
        let in_passes = self.rows.rows() > 0
            && self.rows.parameters.is_empty()
            && ctx.engine.transient_rows_are_whole(netlist)
            && !netlist
                .measurements
                .iter()
                .any(|statement| statement.analysis.eq_ignore_ascii_case("TRAN_CONT"))
            && netlist.assertions.is_empty()
            && netlist.vector_files.is_empty();
        if !in_passes {
            self.read_back(ctx, tstop)?;
            return Ok(None);
        }
        self.hold_events()?;
        let cards = netlist
            .analyses
            .iter()
            .filter(|card| matches!(card, AnalysisCommand::Four { .. }))
            .collect::<Vec<_>>();
        let fourier = if cards.is_empty() {
            Vec::new()
        } else {
            let every_card =
                ctx.fourier_cards_for_current_transient()? == (0..cards.len()).collect::<Vec<_>>();
            let records = if every_card {
                self.fourier_window(ctx, tstop)?
            } else {
                self.read_back(ctx, tstop)?;
                None
            };
            let Some(records) = records else {
                return Ok(None);
            };
            match self.fourier_spectra(ctx, &cards, &records)? {
                Some(fourier) => fourier,
                None => {
                    self.read_back(ctx, tstop)?;
                    return Ok(None);
                }
            }
        };
        let measurements = if netlist
            .measurements
            .iter()
            .any(|statement| statement.analysis.eq_ignore_ascii_case("TRAN"))
        {
            let probe_rows = self
                .rows
                .reader
                .chunk_spans()
                .next()
                .map_or(0, |span| span.1);
            self.rows.fill(&mut self.shell, 0, probe_rows as u64)?;
            let measured = self.measure(ctx);
            self.rows.clear(&mut self.shell);
            match measured? {
                Some(measurements) => measurements,
                None => {
                    self.read_back(ctx, tstop)?;
                    return Ok(None);
                }
            }
        } else {
            Vec::new()
        };
        // With neither continuous measurements nor assertions, these read no rows.
        let limits = ctx.engine.config().resource_limits;
        let continuous_measurements =
            rspice_core::analysis::evaluate_tran_continuous_measurements_with_limits_and_abort(
                netlist,
                &self.shell,
                &limits,
                abort,
            )
            .map_err(failed)?;
        let assertions =
            rspice_core::analysis::evaluate_tran_assertions(netlist, &self.shell, abort)
                .map_err(failed)?;
        Ok(Some(TransientPostResults {
            fft: self.shell.fft_results.clone(),
            fourier,
            measurements,
            continuous_measurements,
            assertions,
        }))
    }

    /// Each `.FOUR` card's spectra, analysed from the records [`Self::fourier_window`] took and
    /// the rows it left in the shell, as
    /// [`rspice_core::engine::evaluate_transient_fourier_results`] analyses the run read back;
    /// `None` when a card or an operand fails, which the run read back then reports.
    fn fourier_spectra(
        &self,
        ctx: &RunContext<'_>,
        cards: &[&AnalysisCommand],
        records: &FourierRecords,
    ) -> Result<Option<Vec<TransientFourierResult>>, CliError> {
        let netlist = ctx.netlist;
        let abort = &crate::abort::ProcessAbort;
        let limits = ctx.engine.config().resource_limits;
        let method = FourierMethod::for_dialect(netlist.parse_dialect());
        let mut spectra = Vec::new();
        for (card_index, &card) in cards.iter().enumerate() {
            let &AnalysisCommand::Four {
                fundamental,
                num_harmonics,
                ..
            } = card
            else {
                return Ok(None);
            };
            let Some(config) = (fundamental.is_finite() && fundamental > 0.0 && num_harmonics > 0)
                .then(|| FourierConfig::try_from(card).ok())
                .flatten()
            else {
                return Ok(None);
            };
            let columns = match rspice_core::analysis::evaluate_tran_four_output_requests_with_abort(
                netlist,
                &self.shell,
                card_index,
                limits,
                abort,
            ) {
                Ok(columns) => columns,
                Err(rspice_core::SimulationError::Aborted) => {
                    return Err(super::cancellation_cli_error(ctx.args.timeout));
                }
                Err(_) => return Ok(None),
            };
            let Some(card_records) = records
                .iter()
                .find(|(index, _)| *index == card_index)
                .map(|(_, records)| records)
                .filter(|records| records.len() == columns.len())
            else {
                return Ok(None);
            };
            let analysis = FourierAnalysis::new(config.with_method(method));
            for ((output, physical_type, waveform), record) in columns.into_iter().zip(card_records)
            {
                let spectrum = match analysis.analyze_transient_record_with_abort(
                    record,
                    &self.shell,
                    &waveform,
                    abort,
                ) {
                    Ok(spectrum) => spectrum,
                    Err(FourierError::Aborted) => {
                        return Err(super::cancellation_cli_error(ctx.args.timeout));
                    }
                    Err(_) => return Ok(None),
                };
                spectra.push(TransientFourierResult {
                    card_index,
                    output,
                    physical_type,
                    fundamental,
                    harmonic_count: num_harmonics,
                    spectrum,
                });
            }
        }
        Ok(Some(spectra))
    }
}

/// The result column a slot's values fill, as an output reads it; `None` for one an output
/// reads only from a run read back.
fn stream_column(shell: &TransientResult, target: SlotTarget) -> Option<&[f64]> {
    match target {
        SlotTarget::Voltage(node) => shell.voltages.get(node).map(Vec::as_slice),
        SlotTarget::Current(branch) => shell.branch_currents.get(branch).map(Vec::as_slice),
        SlotTarget::DeviceOp(index) => shell
            .device_op_traces
            .get(index)
            .map(|trace| trace.values.as_slice()),
        SlotTarget::Parameter(_) | SlotTarget::StepSize | SlotTarget::Store(_) => None,
    }
}

/// A stored run on its way to its outputs.
pub(super) struct StoredOutput<'s> {
    pub(super) rows: &'s mut StoredRows,
    /// How the outputs read the rows from the store; `None` when they were read back.
    pub(super) plan: Option<&'s StreamPlan>,
    /// The output the run was published to, which its store is left beside.
    pub(super) published: Option<PathBuf>,
}

/// A results store a run left, for the run summary.
#[derive(Debug, Clone)]
pub(crate) struct StoreRecord {
    pub(crate) path: PathBuf,
    pub(crate) rows: u64,
    pub(crate) events: u64,
    pub(crate) bytes: u64,
    pub(crate) digest: Option<String>,
    /// Why the run stopped before completing, for a store a stopped run left.
    pub(crate) stopped: Option<StoppedRecord>,
}

/// Where a run stopped and what it kept.
#[derive(Debug, Clone)]
pub(crate) struct StoppedRecord {
    pub(crate) reason: &'static str,
    pub(crate) at_x: Option<f64>,
    pub(crate) rows_kept: u64,
    pub(crate) bytes_kept: u64,
}

fn remember(record: StoreRecord) {
    STORES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(record);
}

/// Every store the runs of this process left, in the order they finished.
static STORES: std::sync::Mutex<Vec<StoreRecord>> = std::sync::Mutex::new(Vec::new());

/// Leave `rows`' store beside `output` when `keep`, else delete it, and remember a store left.
/// A run its storage stopped ends here as stopped, its outputs written for what it kept.
pub(super) fn record_store(
    mut rows: StoredRows,
    output: Option<&Path>,
    keep: bool,
) -> Result<(), CliError> {
    let receipt = rows.receipt();
    if rows.stopped.is_some() {
        let at = rows.last_time()?;
        let free = free_space(&rows.store.dir)();
        // A store that kept no row is not left behind.
        let kept_at = rows.publish(output, keep && receipt.rows > 0)?;
        return Err(storage_full(at, receipt, kept_at, free));
    }
    if let Some(path) = rows.publish(output, keep)? {
        let record = StoreRecord {
            path,
            rows: receipt.rows,
            events: receipt.events,
            bytes: receipt.bytes,
            digest: receipt.digest.map(hex),
            stopped: None,
        };
        remember(record);
    }
    Ok(())
}

/// The stop of a run that completed but whose outputs filled the disk as they were written: its
/// store, which holds every row, is left beside `output`.
pub(super) fn output_filled(
    mut rows: StoredRows,
    output: Option<&Path>,
    error: &CliError,
) -> CliError {
    let receipt = rows.receipt();
    let at = match rows.last_time() {
        Ok(at) => at,
        Err(failure) => return failure,
    };
    let free = free_space(&rows.store.dir)();
    let kept_at = match rows.publish(output, true) {
        Ok(path) => path,
        Err(failure) => return failure,
    };
    let message = format!(
        "the transient completed to t = {}, but writing its output filled the disk ({error}); \
         all {} points are kept ({} bytes){}{}",
        at.map_or_else(|| "0 s".to_owned(), crate::console::si_seconds),
        receipt.rows,
        receipt.bytes,
        kept_at
            .as_ref()
            .map_or_else(String::new, |path| format!(" in {}", path.display())),
        free.map_or_else(String::new, |free| format!("; {free} bytes free")),
    );
    if let Some(path) = &kept_at {
        remember(StoreRecord {
            path: path.clone(),
            rows: receipt.rows,
            events: receipt.events,
            bytes: receipt.bytes,
            digest: receipt.digest.map(hex),
            stopped: Some(StoppedRecord {
                reason: "output_disk_full",
                at_x: at,
                rows_kept: receipt.rows,
                bytes_kept: receipt.bytes,
            }),
        });
    }
    CliError::StorageFull {
        message,
        store: kept_at,
    }
}

/// A store digest as the run summary writes it.
fn hex(digest: [u8; 32]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The stores this process's runs left, by path. A store an abandoned axis set took with it is
/// not one of them.
pub(crate) fn stores_left() -> Vec<StoreRecord> {
    let mut stores: Vec<StoreRecord> = STORES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .filter(|record| record.path.is_dir())
        .cloned()
        .collect();
    stores.sort_by(|a, b| a.path.cmp(&b.path));
    stores
}

/// The output signals of a streamed run and the store slot each one reads.
/// Where a transient's outputs read its event histories.
pub(super) enum RunEvents<'r> {
    /// The traces' own points.
    Held(HeldEvents<'r>),
    /// The run's store, a frame at a time.
    Stored(Box<StoreEvents<FileIo>>),
}

impl EventSource for RunEvents<'_> {
    fn digital(&self, index: usize) -> EventPoints<'_, DigitalValue> {
        match self {
            Self::Held(events) => events.digital(index),
            Self::Stored(events) => events.digital(index),
        }
    }

    fn real(&self, index: usize) -> EventPoints<'_, f64> {
        match self {
            Self::Held(events) => events.real(index),
            Self::Stored(events) => events.real(index),
        }
    }

    fn digital_count(&self, index: usize) -> usize {
        match self {
            Self::Held(events) => events.digital_count(index),
            Self::Stored(events) => events.digital_count(index),
        }
    }

    fn real_count(&self, index: usize) -> usize {
        match self {
            Self::Held(events) => events.real_count(index),
            Self::Stored(events) => events.real_count(index),
        }
    }
}

pub(super) struct StreamPlan {
    /// Named and typed as the in-memory projection names them, without samples.
    pub(super) signals: Vec<ScalarSignal>,
    /// Where each signal's values come from; none for an event dump, which reads no rows.
    columns: Vec<Column>,
    /// The deck's `.MEASURE TRAN` results, measured from the store; `None` when it has none.
    pub(super) measurements: Option<Vec<MeasureResult>>,
    /// The derived columns' values, when a column is one.
    derived: Option<DerivedColumns>,
}

/// The values of a plan's derived columns, evaluated chunk by chunk from the store's columns
/// and staged, when an output writes them, in a file in the system's temporary directory:
/// chunk after chunk, each chunk's values one derived column after another. The file goes when
/// this does.
pub(crate) struct DerivedColumns {
    path: Option<PathBuf>,
    /// Where each chunk's values start in the file.
    offsets: Vec<u64>,
}

impl Drop for DerivedColumns {
    fn drop(&mut self) {
        if let Some(path) = &self.path {
            let _ = std::fs::remove_file(path);
        }
    }
}

impl StreamPlan {
    /// Whether a column is sampled from an event trace, so that reading the rows reads the
    /// run's event histories too.
    pub(super) fn reads_events(&self) -> bool {
        self.columns
            .iter()
            .any(|column| matches!(column, Column::Digital(_) | Column::Real(_)))
    }
}

/// Where one output column's values come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Column {
    /// A value slot of the store.
    Slot(usize),
    /// The digital event trace at this index among the result's, sampled onto the rows as its
    /// `D(node)` column.
    Digital(usize),
    /// The real event trace at this index among the result's, sampled as its `E(node)` column.
    Real(usize),
    /// The derived column at this index among the plan's, which the projection evaluates from
    /// the store's columns row by row: a printed expression, or a voltage between two nodes.
    Derived(usize),
}

/// Project the probe rows in `shell` and keep the plan only when [`plan_columns`] finds every
/// projected signal's values. A deck's `.PRINT TRAN` columns are derived from the store's when
/// none reads rows other than its own.
fn probe_plan(
    ctx: &RunContext<'_>,
    shell: &TransientResult,
    targets: &[SlotTarget],
    events: Option<&RunEvents<'_>>,
) -> Result<Option<StreamPlan>, CliError> {
    let Ok(mut signals) = crate::commands::run_signals::transient_export_signals(
        ctx.netlist,
        shell,
        ctx.transient_limits(),
        &crate::abort::ProcessAbort,
    ) else {
        // The run read back whole reports the same failure the in-memory run would.
        log::debug!("the transient is read back whole: its signals do not project");
        return Ok(None);
    };
    let events = events.map(|events| events as &dyn EventSource);
    let prints = ctx.netlist.output_requests.iter().any(|request| {
        matches!(
            request.directive,
            OutputDirectiveKind::Print | OutputDirectiveKind::Plot
        ) && request
            .analysis
            .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
    });
    let derived =
        prints && !rspice_core::analysis::tran_print_operands_read_other_rows(ctx.netlist);
    let Some(columns) = plan_columns(shell, targets, &signals, events, derived) else {
        return Ok(None);
    };
    for signal in &mut signals {
        signal.values = Vec::new();
    }
    Ok(Some(StreamPlan {
        signals,
        columns,
        measurements: None,
        derived: None,
    }))
}

/// Where each of `signals`, projected from the rows `shell` holds, reads its values over the
/// whole run: the store column the projection names it after, holding its values bit for bit
/// on those rows, or, given the run's `events`, the event trace its `D(node)` or `E(node)`
/// column is sampled from, as [`EventCursor`] samples it, or, when `derived` is set, the
/// projection's own evaluation of the store's columns. `None` when a signal is none of these.
///
/// A digital level is always finite, and a real event value is wherever every one of them is,
/// which this checks, as the in-memory route checks every column; `None` when one is not.
pub(crate) fn plan_columns(
    shell: &TransientResult,
    targets: &[SlotTarget],
    signals: &[ScalarSignal],
    events: Option<&dyn EventSource>,
    derived: bool,
) -> Option<Vec<Column>> {
    let slots = signal_slots(shell, targets, signals)?;
    let mut real_checked = false;
    let mut columns = Vec::with_capacity(signals.len());
    let mut derived_columns = 0;
    for (signal, slot) in signals.iter().zip(slots) {
        if let Some(slot) = slot {
            columns.push(Column::Slot(slot));
            continue;
        }
        let unplanned = |why: &str| {
            log::debug!("{} is read back whole: {why}", signal.display_name);
            None
        };
        let event = events.and_then(|_| event_column(shell, &signal.display_name));
        let Some(column) = event else {
            if derived {
                columns.push(Column::Derived(derived_columns));
                derived_columns += 1;
                continue;
            }
            return unplanned(if events.is_some() {
                "it is neither a store column nor an event trace's"
            } else {
                "it is no store column"
            });
        };
        let Some(events) = events else {
            return unplanned("it is no store column");
        };
        if matches!(column, Column::Real(_)) && !real_checked {
            if !real_events_finite(events, shell.real_traces.len()) {
                return unplanned("a real event value is not finite");
            }
            real_checked = true;
        }
        columns.push(column);
    }
    Some(columns)
}

/// The event trace a signal of `shell`'s projection is sampled from, by the name the
/// projection gives its column: `D(node)` for a digital trace, `E(node)` for a real one. `None`
/// for any other name, or one two traces give.
fn event_column(shell: &TransientResult, name: &str) -> Option<Column> {
    fn node<'n>(name: &'n str, prefix: &str) -> Option<&'n str> {
        name.strip_prefix(prefix)?.strip_suffix(')')
    }
    fn only(mut found: impl Iterator<Item = usize>) -> Option<usize> {
        let index = found.next()?;
        found.next().is_none().then_some(index)
    }
    if let Some(node) = node(name, "D(") {
        return only(
            shell
                .digital_traces
                .iter()
                .enumerate()
                .filter(|(_, trace)| trace.node_name == node)
                .map(|(index, _)| index),
        )
        .map(Column::Digital);
    }
    let node = node(name, "E(")?;
    only(
        shell
            .real_traces
            .iter()
            .enumerate()
            .filter(|(_, trace)| trace.node_name == node)
            .map(|(index, _)| index),
    )
    .map(Column::Real)
}

/// The projection's allowance for an event that lands on a row's time but for its last bit.
const EVENT_TIME_EPSILON: f64 = 1.0e-18;

/// A digital level as its `D(node)` column reads it.
fn digital_level(value: DigitalValue) -> f64 {
    match value.to_bool() {
        Some(false) => 0.0,
        Some(true) => 1.0,
        None => 0.5,
    }
}

/// One event trace sampled onto rows of ascending time, as the result's projection samples it:
/// a row reads the last change at or before its time, within the projection's last-bit
/// allowance; a digital level reads 0, 1 or 0.5 (its unknown level before the first change),
/// and a real value itself (0.0 before the first change).
struct EventCursor<'e> {
    points: EventCursorPoints<'e>,
    value: f64,
    /// The time of the last row sampled.
    time: f64,
}

enum EventCursorPoints<'e> {
    Digital(std::iter::Peekable<EventPoints<'e, DigitalValue>>),
    Real(std::iter::Peekable<EventPoints<'e, f64>>),
}

impl<'e> EventCursor<'e> {
    /// A cursor at the start of `column`'s trace in `events`; `None` for a column that is no
    /// event trace's.
    fn open(events: &'e dyn EventSource, column: Column) -> Option<Self> {
        let (points, value) = match column {
            Column::Digital(index) => (
                EventCursorPoints::Digital(events.digital(index).peekable()),
                digital_level(DigitalValue::default()),
            ),
            Column::Real(index) => (EventCursorPoints::Real(events.real(index).peekable()), 0.0),
            Column::Slot(_) | Column::Derived(_) => return None,
        };
        Some(Self {
            points,
            value,
            time: f64::NEG_INFINITY,
        })
    }

    /// The value at a row at `time`, no earlier than the last row sampled.
    fn at(&mut self, time: f64) -> Result<f64, EventProjectionError> {
        self.time = time;
        let limit = time + EVENT_TIME_EPSILON;
        match &mut self.points {
            EventCursorPoints::Digital(points) => {
                while let Some(point) = points.next_if(|point| due(point, limit)) {
                    self.value = digital_level(point?.1);
                }
            }
            EventCursorPoints::Real(points) => {
                while let Some(point) = points.next_if(|point| due(point, limit)) {
                    self.value = point?.1;
                }
            }
        }
        Ok(self.value)
    }
}

/// Whether a row whose time `limit` allows reads `point`: a change no later, or a failure.
fn due<T>(point: &Result<(f64, T), EventProjectionError>, limit: f64) -> bool {
    match point {
        Ok((change, _)) => *change <= limit,
        Err(_) => true,
    }
}

/// Each of `signals`' value slot in the store: the store column `shell`'s projection names it
/// after, holding the signal's own values, bit for bit, on the rows `shell` holds; `None` for a
/// signal that is no store column. `None` overall when `shell` has no projection.
fn signal_slots(
    shell: &TransientResult,
    targets: &[SlotTarget],
    signals: &[ScalarSignal],
) -> Option<Vec<Option<usize>>> {
    let column = |target: SlotTarget| stream_column(shell, target);
    let inventory = rspice_core::execution::transient_projection_signals(shell).ok()?;
    // A `.PRINT` operand names its column as the card spells it (`v(out)` for `V(out)`), so a
    // name is matched without case, and one two columns share is no column's.
    let mut by_name: std::collections::HashMap<String, Option<usize>> =
        std::collections::HashMap::new();
    for signal in &inventory {
        let rspice_core::execution::ProjectionValues::Real(values) = signal.values() else {
            continue;
        };
        let slot = targets
            .iter()
            .position(|&target| column(target).map(|c| c.as_ptr()) == Some(values.as_ptr()));
        by_name
            .entry(signal.descriptor().display_name().to_ascii_uppercase())
            .and_modify(|seen| *seen = None)
            .or_insert(slot);
    }
    Some(
        signals
            .iter()
            .map(|signal| {
                let slot = (*by_name.get(&signal.display_name.to_ascii_uppercase())?)?;
                let values = column(*targets.get(slot)?)?;
                let same = signal.values.len() == values.len()
                    && signal
                        .values
                        .iter()
                        .zip(values)
                        .all(|(a, b)| a.to_bits() == b.to_bits());
                same.then_some(slot)
            })
            .collect(),
    )
}

/// Whether every value of the `traces` real event traces in `events` reads as finite.
fn real_events_finite(events: &dyn EventSource, traces: usize) -> bool {
    (0..traces).all(|trace| {
        events
            .real(trace)
            .all(|point| point.is_ok_and(|(_, value)| value.is_finite()))
    })
}

impl StoredRows {
    pub(super) fn rows(&self) -> u64 {
        self.reader.rows()
    }

    /// The storage filled and the run ended at its last whole chunk, before its stop time.
    pub(super) fn stopped(&self) -> bool {
        self.stopped.is_some()
    }

    /// The time of the last row.
    pub(super) fn last_time(&mut self) -> Result<Option<f64>, CliError> {
        let rows = self.rows();
        if rows == 0 {
            return Ok(None);
        }
        let dir = self.store.dir.clone();
        self.reader
            .read_x(rows - 1, 1)
            .map(|x| x.first().copied())
            .map_err(|error| store_error(&dir, error))
    }

    /// Where an output reads `shell`'s event histories: the shell's traces once they are held,
    /// and until then the store, through a reader of its own so that the rows' reader stays free.
    pub(super) fn events<'r>(&self, shell: &'r TransientResult) -> Result<RunEvents<'r>, CliError> {
        if self.events_held {
            return Ok(RunEvents::Held(HeldEvents {
                digital: &shell.digital_traces,
                real: &shell.real_traces,
            }));
        }
        let segment = self.store.dir.join(StoreDir::segment_name(0));
        let reader = FileIo::open(&segment)
            .map_err(StoreError::from)
            .and_then(SegmentReader::open)
            .map_err(|error| store_error(&self.store.dir, error))?;
        Ok(RunEvents::Stored(Box::new(StoreEvents::new(reader, shell))))
    }

    /// Move rows `first .. first + count` into `result`'s time and saved columns.
    fn fill(
        &mut self,
        result: &mut TransientResult,
        first: u64,
        count: u64,
    ) -> Result<(), CliError> {
        let count = usize::try_from(count).map_err(|_| CliError::InternalError {
            message: "a transient store holds more rows than this host can address".to_owned(),
        })?;
        fill_rows(&mut self.reader, &self.targets, result, first, count)
            .map_err(|error| store_error(&self.store.dir, error))
    }

    /// Move the first and last rows into `result`'s time and saved columns: what a statement
    /// that reads only event operands reads of the run, and the row a PARAM reads.
    fn fill_span(&mut self, result: &mut TransientResult) -> Result<(), CliError> {
        fill_span(&mut self.reader, &self.targets, result)
            .map_err(|error| store_error(&self.store.dir, error))
    }

    fn clear(&self, result: &mut TransientResult) {
        clear_rows(&self.targets, result);
    }

    /// Refuse a non-finite saved value as the in-memory check does: the first column in result
    /// order that has one, at its first such point.
    pub(super) fn ensure_finite(
        &mut self,
        allow: bool,
        shell: &TransientResult,
    ) -> Result<(), CliError> {
        if allow {
            return Ok(());
        }
        let rows = self.rows();
        let Some((slot, point, value)) = first_non_finite(&mut self.reader, &self.targets)
            .map_err(|error| store_error(&self.store.dir, error))?
        else {
            return Ok(());
        };
        let name = slot_name(self.targets[slot], shell, &self.parameters);
        Err(super::shared::non_finite_error(
            "Transient",
            name,
            value,
            point,
            rows,
        ))
    }

    /// The rows an output carries: every accepted step from `tstart` to `tstop`, or the times
    /// the deck's `.OPTIONS OUTPUT` schedule selects, projected from the accepted steps as the
    /// in-memory output projection projects them, from the same time grid, which is the one
    /// column read whole for it.
    pub(super) fn output_view(
        &mut self,
        netlist: &rspice_core::Netlist,
        tstart: f64,
        tstop: f64,
    ) -> Result<OutputView, CliError> {
        let options = &netlist.options;
        if options.output_time_points.is_empty() && options.output_interval_schedule.is_none() {
            let (first, points) = self.output_rows(tstart, tstop)?;
            let points = usize::try_from(points).map_err(|_| CliError::InternalError {
                message: "a transient output has more rows than this host can address".to_owned(),
            })?;
            return Ok(OutputView {
                first,
                points,
                schedule: None,
            });
        }
        let schedule_error = |error| CliError::from(rspice_core::SimulationError::from(error));
        let source_len = usize::try_from(self.rows()).map_err(|_| CliError::InternalError {
            message: "a transient store holds more rows than this host can address".to_owned(),
        })?;
        let chunks = self.reader.chunk_spans().count();
        let mut times = Vec::new();
        if chunks > 0 {
            self.reader
                .read_chunk_x(0, &mut times)
                .map_err(|error| store_error(&self.store.dir, error))?;
        }
        let schedule = OutputSchedule {
            time_points: options.output_time_points.clone(),
            interval: options.output_interval_schedule.clone(),
            start: times.first().map_or(tstart, |&first| tstart.max(first)),
            stop: tstop,
            source_len,
        };
        // The schedule is walked once here to check it and count its rows, and again by
        // each cursor that reads them.
        let mut walk = schedule.walk().map_err(schedule_error)?;
        for chunk in 0..chunks {
            if chunk > 0 {
                self.reader
                    .read_chunk_x(chunk, &mut times)
                    .map_err(|error| store_error(&self.store.dir, error))?;
            }
            for &time in &times {
                walk.push(time, &mut |_| {}).map_err(schedule_error)?;
            }
        }
        let points = walk.finish().map_err(schedule_error)?;
        Ok(OutputView {
            first: 0,
            points,
            schedule: Some(schedule),
        })
    }

    /// The rows an output carries over every accepted step from `tstart` to `tstop`, checked
    /// as the in-memory projection checks the time grid: `(first row, row count)`.
    fn output_rows(&mut self, tstart: f64, tstop: f64) -> Result<(u64, u64), CliError> {
        let invalid = |message: String| {
            CliError::from(rspice_core::SimulationError::from(
                rspice_core::netlist::OutputScheduleError::Invalid(message),
            ))
        };
        let rows = self.rows();
        let dir = self.store.dir.clone();
        let spans = self.reader.chunk_spans().collect::<Vec<_>>();
        let mut x = Vec::new();
        let mut previous: Option<f64> = None;
        let (mut output_start, mut first, mut last) = (tstart, 0u64, 0u64);
        if !tstop.is_finite() || tstop < tstart {
            return Err(invalid(format!(
                "invalid transient output window [{tstart}, {tstop}]"
            )));
        }
        if rows == 0 {
            return Err(invalid(
                "transient result has no accepted samples".to_owned(),
            ));
        }
        for (chunk, &(start, _)) in spans.iter().enumerate() {
            self.reader
                .read_chunk_x(chunk, &mut x)
                .map_err(|error| store_error(&dir, error))?;
            for (offset, &time) in x.iter().enumerate() {
                let index = start + offset as u64;
                if !time.is_finite() {
                    return Err(invalid(format!(
                        "transient result time[{index}] is not finite"
                    )));
                }
                match previous {
                    None => output_start = tstart.max(time),
                    Some(previous) if time <= previous => {
                        return Err(invalid(format!(
                            "transient result time grid is not strictly increasing at index {index}"
                        )));
                    }
                    Some(_) => {}
                }
                previous = Some(time);
                if time < output_start {
                    first = index + 1;
                }
                if time <= tstop {
                    last = index + 1;
                }
            }
        }
        if last <= first {
            return Err(invalid(
                "transient output projection selected no samples".to_owned(),
            ));
        }
        Ok((first, last - first))
    }

    /// Where each channel of `shell`'s compression inventory is in the store; `None` when a
    /// device or store trace is no store column, the store keeps no step sizes, or a column
    /// with no slot is not empty, which the run read back then reports as it is.
    pub(super) fn compression_slots(&self, shell: &TransientResult) -> Option<CompressionSlots> {
        let mut nodes = vec![None; shell.voltages.len()];
        let mut branches = vec![None; shell.branch_currents.len()];
        let mut devices = vec![None; shell.device_op_traces.len()];
        let mut stores = vec![None; shell.store_traces.len()];
        let mut step_size = None;
        for (slot, &target) in self.targets.iter().enumerate() {
            let entry = match target {
                SlotTarget::Voltage(node) => nodes.get_mut(node),
                SlotTarget::Current(branch) => branches.get_mut(branch),
                SlotTarget::DeviceOp(index) => devices.get_mut(index),
                SlotTarget::Store(index) => stores.get_mut(index),
                SlotTarget::StepSize => Some(&mut step_size),
                SlotTarget::Parameter(_) => None,
            };
            if let Some(entry) = entry {
                *entry = Some(slot);
            }
        }
        let unsaved_empty = nodes
            .iter()
            .zip(&shell.voltages)
            .chain(branches.iter().zip(&shell.branch_currents))
            .all(|(slot, column)| slot.is_some() || column.is_empty());
        if !unsaved_empty || devices.iter().chain(&stores).any(Option::is_none) {
            return None;
        }
        Some(CompressionSlots {
            channels: nodes
                .into_iter()
                .chain(branches)
                .chain(devices)
                .chain(stores)
                .collect(),
            step_size: step_size?,
        })
    }

    /// The run's accepted time grid and step sizes, read whole, and a reader of the rest of its
    /// rows at `slots`, a point at a time.
    pub(super) fn compression_rows(
        &mut self,
        slots: CompressionSlots,
    ) -> Result<(Vec<f64>, Vec<f64>, CompressionRows<'_>), CliError> {
        let rows = usize::try_from(self.rows()).map_err(|_| CliError::InternalError {
            message: "a transient store holds more rows than this host can address".to_owned(),
        })?;
        let spans = self.reader.chunk_spans().collect::<Vec<_>>();
        let dir = self.store.dir.as_path();
        let read = |error| store_error(dir, error);
        let (mut time, mut step_sizes) = (Vec::with_capacity(rows), Vec::with_capacity(rows));
        let mut column = Vec::new();
        for chunk in 0..spans.len() {
            self.reader.read_chunk_x(chunk, &mut column).map_err(read)?;
            time.extend_from_slice(&column);
            self.reader
                .read_chunk_slot(chunk, slots.step_size, &mut column)
                .map_err(read)?;
            step_sizes.extend_from_slice(&column);
        }
        let chunk_values = spans
            .iter()
            .map(|&(_, rows)| rows)
            .max()
            .unwrap_or(1)
            .saturating_mul(slots.channels.len())
            .max(1);
        let rows = CompressionRows {
            dir,
            reader: &mut self.reader,
            spans,
            capacity: (COMPRESSION_CACHE_VALUES / chunk_values).max(2),
            slots: slots.channels,
            cache: Vec::new(),
            column,
            error: None,
        };
        Ok((time, step_sizes, rows))
    }

    /// A reader of rows `first ..` for `plan`'s signals, chunk by chunk, with the run's event
    /// histories in `events` when the plan reads an event column.
    pub(super) fn cursor<'r>(
        &'r mut self,
        plan: &'r StreamPlan,
        view: &'r OutputView,
        events: Option<&'r dyn EventSource>,
    ) -> RowCursor<'r> {
        let mut cursor = RowCursor::over(
            &self.store.dir,
            &mut self.reader,
            &plan.columns,
            events,
            plan.derived.as_ref(),
            view.first,
        );
        if let Some(schedule) = &view.schedule {
            cursor.scheduled = Some(ScheduledRows::new(schedule, plan.columns.len() + 1));
        }
        cursor
    }

    /// Leave the store beside `output` as `<stem>.rsr/`, replacing an earlier run's store there,
    /// or delete it when there is no output or `keep` is false. Returns where it was left.
    pub(super) fn publish(
        self,
        output: Option<&Path>,
        keep: bool,
    ) -> Result<Option<PathBuf>, CliError> {
        let Self { store, reader, .. } = self;
        // The segment closes before its store moves or goes: Windows refuses to rename or
        // delete a directory with a file open in it.
        drop(reader);
        store.publish(output, keep)
    }

    /// What the store kept and its digest, for the run summary.
    pub(super) fn receipt(&self) -> SinkReceipt {
        self.receipt
    }
}

/// The rows an output carries, as [`StoredRows::output_view`] selects them: rows `first ..
/// first + points` of the store, or the `points` rows `schedule` selects from all of them.
pub(super) struct OutputView {
    first: u64,
    points: usize,
    schedule: Option<OutputSchedule>,
}

impl OutputView {
    pub(super) fn points(&self) -> usize {
        self.points
    }
}

/// A deck's `.OPTIONS OUTPUT` schedule over a store's rows, walked afresh by each pass over
/// them.
struct OutputSchedule {
    time_points: Vec<f64>,
    interval: Option<XyceOutputIntervalSchedule>,
    start: f64,
    stop: f64,
    source_len: usize,
}

impl OutputSchedule {
    fn walk(&self) -> Result<TransientOutputWalk<'_>, OutputScheduleError> {
        TransientOutputWalk::new(
            self.source_len,
            &self.time_points,
            self.interval.as_ref(),
            self.start,
            self.stop,
            usize::MAX,
        )
    }
}

/// An output schedule's rows as a cursor reads them: the schedule walked over the store's
/// time column a chunk at a time, and the rows it has made due that the cursor has not
/// passed. Its row times are the schedule's, and each row's values are sampled from the
/// store's rows it is read from.
struct ScheduledRows<'r> {
    schedule: &'r OutputSchedule,
    walk: Option<TransientOutputWalk<'r>>,
    /// The time column's next chunk to read, and the times of the one read, fed up to
    /// `offset`.
    chunk: usize,
    times: Vec<f64>,
    offset: usize,
    due: VecDeque<TransientOutputRow>,
    /// The output row `due`'s first row is.
    row: usize,
    /// Each variable's last two values read from the store, by store row: an interpolated row
    /// reads two adjacent store rows, which the next row reads again or moves past by one.
    memo: Vec<[Option<(u64, f64)>; 2]>,
}

impl<'r> ScheduledRows<'r> {
    fn new(schedule: &'r OutputSchedule, variables: usize) -> Self {
        Self {
            schedule,
            walk: None,
            chunk: 0,
            times: Vec::new(),
            offset: 0,
            due: VecDeque::new(),
            row: 0,
            memo: vec![[None; 2]; variables],
        }
    }

    /// Walk the schedule again from its first row.
    fn restart(&mut self) -> Result<(), OutputScheduleError> {
        self.walk = Some(self.schedule.walk()?);
        self.chunk = 0;
        self.times.clear();
        self.offset = 0;
        self.due.clear();
        self.row = 0;
        Ok(())
    }
}

/// Reads the values of an output's rows a chunk at a time: a store column from the store, each
/// column of a chunk decoded once, when it is first asked for, and an event trace's column
/// sampled onto the rows' times as they are read. An error or a non-finite value is recorded
/// rather than returned, since a table writer asks for numbers, and checked once the table is
/// written.
pub(crate) struct RowCursor<'r> {
    dir: &'r Path,
    reader: &'r mut SegmentReader<FileIo>,
    columns: &'r [Column],
    events: Option<&'r dyn EventSource>,
    spans: Vec<(u64, usize)>,
    first: u64,
    chunk: Option<usize>,
    x: Option<Vec<f64>>,
    values: Vec<Option<Vec<f64>>>,
    /// Each event column's place in its trace, where it has one.
    cursors: Vec<Option<EventCursor<'r>>>,
    derived: Option<&'r DerivedColumns>,
    /// The output schedule's rows, when the output has one.
    scheduled: Option<ScheduledRows<'r>>,
    schedule_error: Option<OutputScheduleError>,
    /// The derived columns' staged file, once a row reads one.
    derived_file: Option<std::fs::File>,
    error: Option<StoreError>,
    event_error: Option<EventProjectionError>,
    derived_error: Option<std::io::Error>,
    non_finite: bool,
}

/// Why [`RowCursor::write_column`] stopped.
pub(crate) enum ColumnStop {
    /// The value at this output row is not finite.
    NonFinite(usize),
    /// A read or the write failed.
    Failed(CliError),
}

impl<'r> RowCursor<'r> {
    /// A cursor over rows `first..` of the store in `dir` that `reader` reads, whose output
    /// signal `n` (variable `n + 1`) is `columns[n]`, an event trace's read from `events` and a
    /// derived column's from `derived`.
    pub(crate) fn over(
        dir: &'r Path,
        reader: &'r mut SegmentReader<FileIo>,
        columns: &'r [Column],
        events: Option<&'r dyn EventSource>,
        derived: Option<&'r DerivedColumns>,
        first: u64,
    ) -> Self {
        let spans = reader.chunk_spans().collect();
        Self {
            dir,
            reader,
            columns,
            events,
            spans,
            first,
            chunk: None,
            x: None,
            values: vec![None; columns.len()],
            cursors: columns.iter().map(|_| None).collect(),
            derived,
            scheduled: None,
            schedule_error: None,
            derived_file: None,
            error: None,
            event_error: None,
            derived_error: None,
            non_finite: false,
        }
    }

    /// Variable `variable` (0 is time, then each output signal) of output row `row`.
    pub(crate) fn value(&mut self, row: usize, variable: usize) -> f64 {
        let value = match self.scheduled {
            None => self.stored(row, variable),
            Some(_) => self.scheduled_row(row).map(|output| match variable {
                0 => output.time(),
                _ => output.sample(|index| self.sampled(index as u64, variable)),
            }),
        };
        let value = value.unwrap_or(f64::NAN);
        if !value.is_finite() {
            self.non_finite = true;
        }
        value
    }

    /// Output row `row` of the output schedule, walking it on from the last row read, or from
    /// its start when `row` is earlier.
    fn scheduled_row(&mut self, row: usize) -> Option<TransientOutputRow> {
        let scheduled = self.scheduled.as_mut()?;
        if (scheduled.walk.is_none() || row < scheduled.row)
            && let Err(error) = scheduled.restart()
        {
            self.schedule_error.get_or_insert(error);
            return None;
        }
        loop {
            if row < scheduled.row + scheduled.due.len() {
                scheduled.due.drain(..row - scheduled.row);
                scheduled.row = row;
                return scheduled.due.front().copied();
            }
            scheduled.row += scheduled.due.len();
            scheduled.due.clear();
            if scheduled.offset == scheduled.times.len() {
                if scheduled.chunk >= self.spans.len() {
                    return None;
                }
                if let Err(error) = self
                    .reader
                    .read_chunk_x(scheduled.chunk, &mut scheduled.times)
                {
                    self.error.get_or_insert(error);
                    return None;
                }
                scheduled.chunk += 1;
                scheduled.offset = 0;
                continue;
            }
            let time = scheduled.times[scheduled.offset];
            scheduled.offset += 1;
            let due = &mut scheduled.due;
            let walk = scheduled.walk.as_mut()?;
            if let Err(error) = walk.push(time, &mut |output| due.push_back(output)) {
                self.schedule_error.get_or_insert(error);
                return None;
            }
        }
    }

    /// Variable `variable` of store row `row`, which a scheduled row is sampled from.
    fn sampled(&mut self, row: u64, variable: usize) -> f64 {
        let memo = self
            .scheduled
            .as_ref()
            .and_then(|scheduled| scheduled.memo.get(variable));
        if let Some(&(_, value)) = memo
            .into_iter()
            .flatten()
            .flatten()
            .find(|(read, _)| *read == row)
        {
            return value;
        }
        let value = usize::try_from(row)
            .ok()
            .and_then(|row| self.stored(row, variable))
            .unwrap_or(f64::NAN);
        if let Some(memo) = self
            .scheduled
            .as_mut()
            .and_then(|scheduled| scheduled.memo.get_mut(variable))
        {
            let older = match memo {
                [None, _] => 0,
                [_, None] => 1,
                [Some((first, _)), Some((second, _))] => usize::from(second < first),
            };
            memo[older] = Some((row, value));
        }
        value
    }

    /// Variable `variable` of row `first + row` of the store.
    fn stored(&mut self, row: usize, variable: usize) -> Option<f64> {
        let row = self.first + row as u64;
        let chunk = match self.chunk {
            Some(chunk) if self.spans[chunk].0 <= row => {
                let (start, rows) = self.spans[chunk];
                if row < start + rows as u64 {
                    chunk
                } else {
                    self.load(row)
                }
            }
            _ => self.load(row),
        };
        let offset = (row - self.spans[chunk].0) as usize;
        match variable.checked_sub(1) {
            None => self.time(chunk, offset),
            Some(column) => match self.columns.get(column).copied() {
                Some(Column::Slot(slot)) => {
                    if self.values[column].is_none() {
                        let mut values = Vec::new();
                        if let Err(error) = self.reader.read_chunk_slot(chunk, slot, &mut values) {
                            self.error.get_or_insert(error);
                        }
                        self.values[column] = Some(values);
                    }
                    self.values[column]
                        .as_ref()
                        .and_then(|values| values.get(offset))
                        .copied()
                }
                Some(Column::Derived(derived)) => {
                    if self.values[column].is_none() {
                        let values = self.read_derived(chunk, derived);
                        self.values[column] = Some(values);
                    }
                    self.values[column]
                        .as_ref()
                        .and_then(|values| values.get(offset))
                        .copied()
                }
                Some(event) => self
                    .time(chunk, offset)
                    .and_then(|time| self.event_value(column, event, time)),
                None => None,
            },
        }
    }

    /// The time of row `offset` of chunk `chunk`, the chunk the cursor holds.
    fn time(&mut self, chunk: usize, offset: usize) -> Option<f64> {
        if self.x.is_none() {
            let mut x = Vec::new();
            if let Err(error) = self.reader.read_chunk_x(chunk, &mut x) {
                self.error.get_or_insert(error);
            }
            self.x = Some(x);
        }
        self.x.as_ref().and_then(|x| x.get(offset)).copied()
    }

    /// Derived column `derived`'s values over chunk `chunk`, from the staged file; none when
    /// the read fails, which is kept for [`Self::read_failure`].
    fn read_derived(&mut self, chunk: usize, derived: usize) -> Vec<f64> {
        use std::io::{Read as _, Seek as _};
        let rows = self.spans[chunk].1;
        let width = rows * std::mem::size_of::<f64>();
        let read = |cursor: &mut Self| -> std::io::Result<Vec<f64>> {
            let staged = cursor
                .derived
                .ok_or_else(|| std::io::Error::other("a derived column has no staged values"))?;
            let start = staged
                .offsets
                .get(chunk)
                .copied()
                .ok_or_else(|| std::io::Error::other("a chunk has no staged values"))?;
            if cursor.derived_file.is_none() {
                let path = staged
                    .path
                    .as_deref()
                    .ok_or_else(|| std::io::Error::other("the derived columns were not staged"))?;
                cursor.derived_file = Some(std::fs::File::open(path)?);
            }
            let file = cursor
                .derived_file
                .as_mut()
                .ok_or_else(|| std::io::Error::other("the staged file is not open"))?;
            file.seek(std::io::SeekFrom::Start(start + (derived * width) as u64))?;
            let mut bytes = vec![0_u8; width];
            file.read_exact(&mut bytes)?;
            Ok(bytes
                .chunks_exact(std::mem::size_of::<f64>())
                .map(|value| f64::from_le_bytes(value.try_into().unwrap_or_default()))
                .collect())
        };
        read(self).unwrap_or_else(|error| {
            self.derived_error.get_or_insert(error);
            Vec::new()
        })
    }

    /// Event column `column`, `event`'s trace, at a row at `time`: read on from the last row
    /// it read, or from the trace's start when this row is earlier.
    fn event_value(&mut self, column: usize, event: Column, time: f64) -> Option<f64> {
        let events = self.events?;
        let cursor = self.cursors.get_mut(column)?;
        if cursor.as_ref().is_none_or(|cursor| time < cursor.time) {
            *cursor = EventCursor::open(events, event);
        }
        match cursor.as_mut()?.at(time) {
            Ok(value) => Some(value),
            Err(error) => {
                self.event_error.get_or_insert(error);
                None
            }
        }
    }

    /// Write variable `variable` (as [`Self::value`] numbers it) of output rows `0..rows` to
    /// `out` as the little-endian bytes of its values, a chunk's worth at a time.
    pub(crate) fn write_column(
        &mut self,
        variable: usize,
        rows: usize,
        out: &mut dyn std::io::Write,
        path: &Path,
    ) -> Result<(), ColumnStop> {
        let mut bytes = Vec::with_capacity(1 << 16);
        for row in 0..rows {
            let value = self.value(row, variable);
            if !value.is_finite() {
                return Err(match self.read_failure(path) {
                    Some(error) => ColumnStop::Failed(error),
                    None => ColumnStop::NonFinite(row),
                });
            }
            bytes.extend_from_slice(&value.to_le_bytes());
            if bytes.len() >= 1 << 16 || row + 1 == rows {
                out.write_all(&bytes)
                    .map_err(|error| ColumnStop::Failed(CliError::output_error(path, error)))?;
                bytes.clear();
            }
        }
        Ok(())
    }

    fn load(&mut self, row: u64) -> usize {
        let chunk = self
            .spans
            .partition_point(|&(start, rows)| start + rows as u64 <= row)
            .min(self.spans.len().saturating_sub(1));
        self.chunk = Some(chunk);
        self.x = None;
        self.values.iter_mut().for_each(|column| *column = None);
        chunk
    }

    /// The read that failed, if one did: of the store, or of an event history, which names
    /// `path`, the output it was being written to.
    pub(crate) fn read_failure(&mut self, path: &Path) -> Option<CliError> {
        if let Some(error) = self.error.take() {
            return Some(store_error(self.dir, error));
        }
        if let Some(error) = self.schedule_error.take() {
            return Some(CliError::from(rspice_core::SimulationError::from(error)));
        }
        if let Some(error) = self.derived_error.take() {
            return Some(CliError::output_error(
                self.derived
                    .and_then(|derived| derived.path.as_deref())
                    .unwrap_or(self.dir),
                error,
            ));
        }
        self.event_error
            .take()
            .map(|error| CliError::ConversionError {
                message: format!("{}: {error}", path.display()),
            })
    }

    /// Fail when a read failed or a value was not finite, as a table that held them would.
    pub(crate) fn finish(mut self, path: &Path) -> Result<(), CliError> {
        if let Some(error) = self.read_failure(path) {
            return Err(error);
        }
        if self.non_finite {
            return Err(crate::commands::export_table::invalid_samples(path));
        }
        Ok(())
    }
}

/// Decoded values a compression's row cache keeps at most: chunks of a wide run hold up to 1 Mi
/// values each, so this keeps a few of them, and more of a narrow run's.
const COMPRESSION_CACHE_VALUES: usize = 8 << 20;

/// Where each channel of a compression's inventory is in a store: its value slot, none for a
/// node or branch the run did not save; and the step sizes' slot.
pub(super) struct CompressionSlots {
    channels: Vec<Option<usize>>,
    step_size: usize,
}

/// A transient's rows in its store as a compression reads them, a point at a time: every
/// channel's values at one accepted point, from the chunks a cache keeps decoded, the most
/// recently read last. A read that fails is kept for [`Self::finish`] to report.
pub(super) struct CompressionRows<'r> {
    dir: &'r Path,
    reader: &'r mut SegmentReader<FileIo>,
    spans: Vec<(u64, usize)>,
    slots: Vec<Option<usize>>,
    cache: Vec<CachedChunk>,
    capacity: usize,
    column: Vec<f64>,
    error: Option<StoreError>,
}

/// One decoded chunk: its rows' channel values, row after row.
struct CachedChunk {
    chunk: usize,
    first: u64,
    rows: usize,
    values: Vec<f64>,
}

impl CompressionRows<'_> {
    /// Make the cache's last chunk the one holding row `row`, decoding it when it holds none.
    fn load(&mut self, row: u64) -> Result<(), StoreError> {
        let chunk = self
            .spans
            .partition_point(|&(start, rows)| start + rows as u64 <= row);
        let Some(&(first, rows)) = self.spans.get(chunk) else {
            return Err(StoreError::OutOfRange(format!(
                "row {row} of a segment with {} rows",
                self.reader.rows()
            )));
        };
        if let Some(cached) = self.cache.iter().position(|cached| cached.chunk == chunk) {
            let cached = self.cache.remove(cached);
            self.cache.push(cached);
            return Ok(());
        }
        let mut values = if self.cache.len() >= self.capacity {
            self.cache.remove(0).values
        } else {
            Vec::new()
        };
        let width = self.slots.len();
        values.clear();
        values.resize(rows * width, f64::NAN);
        for (channel, &slot) in self.slots.iter().enumerate() {
            let Some(slot) = slot else {
                continue;
            };
            self.reader.read_chunk_slot(chunk, slot, &mut self.column)?;
            for (row, &value) in self.column.iter().enumerate() {
                values[row * width + channel] = value;
            }
        }
        self.cache.push(CachedChunk {
            chunk,
            first,
            rows,
            values,
        });
        Ok(())
    }

    /// `compressed`, unless a read of the store failed, which is then what failed.
    pub(super) fn finish<T>(self, compressed: Result<T, CliError>) -> Result<T, CliError> {
        match self.error {
            Some(error) => Err(store_error(self.dir, error)),
            None => compressed,
        }
    }
}

impl AcceptedTransientRows for CompressionRows<'_> {
    fn saved(&self, channel: usize) -> bool {
        self.slots.get(channel).is_some_and(Option::is_some)
    }

    fn row(&mut self, index: usize, row: &mut [f64]) -> Result<(), rspice_core::SimulationError> {
        let index = index as u64;
        let cached = |cached: &CachedChunk| {
            cached.first <= index && index < cached.first + cached.rows as u64
        };
        if !self.cache.last().is_some_and(cached)
            && let Err(error) = self.load(index)
        {
            let failed = rspice_core::SimulationError::Circuit(format!(
                "the transient's results store could not be read: {error}"
            ));
            self.error.get_or_insert(error);
            return Err(failed);
        }
        let width = self.slots.len();
        let values = self
            .cache
            .last()
            .and_then(|cached| {
                let offset = (index - cached.first) as usize * width;
                cached.values.get(offset..offset + width)
            })
            .unwrap_or_default();
        for (value, &cached) in row.iter_mut().zip(values) {
            *value = cached;
        }
        Ok(())
    }
}

impl RunStore {
    /// A new store under a private name: beside `output` when there is one, else in the system's
    /// temporary directory.
    fn create(output: Option<&Path>) -> Result<Self, CliError> {
        let ordinal = NEXT_STORE.fetch_add(1, Ordering::Relaxed);
        let name = format!(".rspice-store-{}-{ordinal}.rsr", std::process::id());
        let dir = match output {
            Some(output) => output
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or(Path::new("."))
                .join(name),
            None => std::env::temp_dir().join(name),
        };
        let store = StoreDir::new(&dir);
        store.create().map_err(|error| store_error(&dir, error))?;
        Ok(Self {
            dir,
            in_place: false,
            fingerprint: None,
        })
    }

    fn save_manifest(&self, reader: &SegmentReader<FileIo>) -> Result<(), StoreError> {
        let (status, digest) = match reader.state() {
            SegmentState::Complete { status, digest } => (Some(status.clone()), Some(*digest)),
            SegmentState::Recovered => (None, None),
        };
        let mut manifest = Manifest::default();
        manifest.upsert(Member {
            index: 0,
            segment: StoreDir::segment_name(0),
            analysis: reader.run_meta().analysis.clone(),
            status,
            digest,
            checkpoint_fingerprint: self.fingerprint,
            ..Member::default()
        });
        StoreDir::new(&self.dir).save_manifest(&manifest)
    }

    fn publish(self, output: Option<&Path>, keep: bool) -> Result<Option<PathBuf>, CliError> {
        if self.in_place {
            // The store a resumed run appended to is already beside its output.
            return Ok(keep.then_some(self.dir));
        }
        let Some(output) = output.filter(|_| keep) else {
            self.discard();
            return Ok(None);
        };
        // A store joins the publication its output joins: an axis set's stores are placed with
        // the set's artifacts, or not at all.
        let target = store_path_beside(output);
        crate::commands::publish::directory(&self.dir, &target, place_store)?;
        Ok(Some(target))
    }

    fn discard(self) {
        // A store a resumed run appended to held the earlier segments' rows before the run began,
        // so it stays, closed over whatever the run wrote. A store that cannot be removed is left
        // under its private name; the run's own outcome is what the caller reports.
        if self.in_place {
            return;
        }
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Move the store at `staged` to `target`, replacing an earlier results store there but nothing
/// else. A store that cannot be placed is deleted.
fn place_store(staged: &Path, target: &Path) -> Result<(), CliError> {
    let discard = || {
        let _ = std::fs::remove_dir_all(staged);
    };
    if target.exists() {
        let earlier_store = target.is_dir() && target.join(StoreDir::MANIFEST).is_file();
        if !earlier_store {
            discard();
            return Err(CliError::output_error(
                target,
                std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "the results store would replace something that is not a results store",
                ),
            ));
        }
        if let Err(error) = std::fs::remove_dir_all(target) {
            discard();
            return Err(CliError::output_error(target, error));
        }
    }
    if let Err(error) = std::fs::rename(staged, target) {
        discard();
        return Err(CliError::output_error(target, error));
    }
    Ok(())
}

/// The store a run of this process left beside `output`, if one did.
pub(crate) fn store_left_beside(output: &Path) -> Option<PathBuf> {
    let target = store_path_beside(output);
    STORES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .any(|record| record.path == target)
        .then_some(target)
}

/// `out.raw` keeps its store in `out.rsr/`.
pub(super) fn store_path_beside(output: &Path) -> PathBuf {
    let target = output.with_extension("rsr");
    if target == output {
        let mut name = output.as_os_str().to_owned();
        name.push(".rsr");
        PathBuf::from(name)
    } else {
        target
    }
}

fn store_error(dir: &Path, error: StoreError) -> CliError {
    match error {
        // The host refused the memory a read asked for: the run's typed allocation failure, with
        // the bytes it needed, as a refusal while solving is.
        StoreError::Allocation {
            object,
            bytes,
            source,
        } => CliError::CoreSimulationError {
            source: rspice_core::SimulationError::Allocation {
                object,
                bytes,
                source,
            },
            analysis: Some("Transient".to_string()),
        },
        error => CliError::output_error(
            dir,
            std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string()),
        ),
    }
}

/// The error a run that did not complete ends with. A run refused before solving, or stopped for
/// any reason but full storage, leaves nothing; one its storage stopped keeps its store, readable
/// to its last whole chunk, beside where its output would have gone.
fn stopped_run(
    ctx: &RunContext<'_>,
    store: RunStore,
    segment: &Path,
    sink: WatchedSink<FileIo>,
    source: rspice_core::SimulationError,
) -> CliError {
    use rspice_core::result_sink::SinkStop;
    let (refused, rows_needed) = (sink.refused(), sink.rows_needed());
    let (mut inner, free) = sink.into_parts();
    if store.in_place {
        // A resumed run that failed before it began leaves the store it reopened closed over the
        // rows it held; one that began was closed with how it stopped, and refuses a second end.
        let _ = rspice_core::result_sink::ResultSink::end(
            &mut inner,
            rspice_core::result_sink::RunStatus::Stopped(
                rspice_core::result_sink::StopReason::Failed(source.to_string()),
            ),
        );
    }
    // The segment closes before its store is read, moved or deleted: Windows refuses to rename
    // or delete a directory with a file open in it.
    drop(inner);
    if let Some((needed, available)) = refused {
        store.discard();
        return CliError::CoreSimulationError {
            source: rspice_core::SimulationError::ResultSink(SinkStop::DiskFull {
                needed: Some(needed),
                available: Some(available),
            }),
            analysis: Some(format!(
                "Transient, refused before solving: its results store needs at least {needed} \
                 bytes for {:.0} rows on a volume with {available} bytes free",
                rows_needed.ceil()
            )),
        };
    }
    let filled = matches!(
        source,
        rspice_core::SimulationError::ResultSink(SinkStop::DiskFull { .. })
    );
    if !filled {
        store.discard();
        return CliError::CoreSimulationError {
            source,
            analysis: Some("Transient".to_string()),
        };
    }
    let kept = FileIo::open(segment)
        .map_err(StoreError::from)
        .and_then(SegmentReader::open)
        .and_then(|reader| {
            store.save_manifest(&reader)?;
            Ok(reader)
        });
    // A disk that filled before the segment held a readable chunk kept nothing.
    let Ok(mut reader) = kept else {
        let free = free();
        store.discard();
        return storage_full(None, SinkReceipt::default(), None, free);
    };
    let rows = reader.rows();
    let at = rows
        .checked_sub(1)
        .and_then(|last| reader.read_x(last, 1).ok())
        .and_then(|x| x.first().copied());
    drop(reader);
    let bytes = std::fs::metadata(segment).map_or(0, |meta| meta.len());
    let output = ctx.resolve_output("tran").map(|resolved| resolved.path);
    let kept_at = match store.publish(output.as_deref(), true) {
        Ok(path) => path,
        Err(error) => return error,
    };
    let receipt = SinkReceipt {
        rows,
        bytes,
        ..SinkReceipt::default()
    };
    storage_full(at, receipt, kept_at, free())
}

/// The stop of a run whose storage filled at `at`, with what `receipt` says it kept in the store
/// at `kept_at`, remembered for the run summary.
fn storage_full(
    at: Option<f64>,
    receipt: SinkReceipt,
    kept_at: Option<PathBuf>,
    free: Option<u64>,
) -> CliError {
    let (rows, bytes) = (receipt.rows, receipt.bytes);
    let message = format!(
        "the transient stopped at t = {} with {rows} points kept ({bytes} bytes){}{}",
        at.map_or_else(|| "0 s".to_owned(), crate::console::si_seconds),
        kept_at
            .as_ref()
            .map_or_else(String::new, |path| format!(" in {}", path.display())),
        free.map_or_else(String::new, |free| format!("; {free} bytes free")),
    );
    if let Some(path) = &kept_at {
        remember(StoreRecord {
            path: path.clone(),
            rows,
            events: receipt.events,
            bytes,
            digest: receipt.digest.map(hex),
            stopped: Some(StoppedRecord {
                reason: "disk_full",
                at_x: at,
                rows_kept: rows,
                bytes_kept: bytes,
            }),
        });
    }
    CliError::StorageFull {
        message,
        store: kept_at,
    }
}

fn free_space(dir: &Path) -> FreeSpace {
    #[cfg(test)]
    if let Some(quota) = tests::QUOTA.with(std::cell::Cell::get) {
        return tests::quota(&dir.join(StoreDir::segment_name(0)), quota);
    }
    rspice_results_store::volume_free_space(dir)
}

#[cfg(test)]
mod tests;
