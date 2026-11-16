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

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::engine::{SinkTransient, TransientResult, TransientStartupMode};
use rspice_core::netlist::OutputDirectiveKind;
use rspice_core::result_sink::{SinkReceipt, SinkStop};
use rspice_results_store::{
    FileIo, Manifest, Member, SegmentReader, SegmentState, StoreDir, StoreError, StoreSink,
};

use super::context::RunContext;
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::ScalarSignal;

/// Store directories this process has opened, so concurrent runs never share one.
static NEXT_STORE: AtomicU64 = AtomicU64::new(0);

/// Where one value slot of a row belongs in the in-memory result.
#[derive(Debug, Clone, Copy)]
enum SlotTarget {
    Voltage(usize),
    Current(usize),
    /// An instance parameter the run kept as a constant column: its index in
    /// [`StoredRows::parameters`].
    Parameter(usize),
    /// The step that reached each point.
    StepSize,
    /// A store trace (a memristor's resistance): its index in the result's store traces.
    Store(usize),
    /// A device operating-point value: its index in the result's device traces.
    DeviceOp(usize),
}

impl SlotTarget {
    /// The result column this slot's values fill; an instance parameter's is adopted whole.
    fn column(self, result: &mut TransientResult) -> Option<&mut Vec<f64>> {
        match self {
            SlotTarget::Voltage(node) => result.voltages.get_mut(node),
            SlotTarget::Current(branch) => result.branch_currents.get_mut(branch),
            SlotTarget::StepSize => Some(&mut result.step_sizes),
            SlotTarget::Store(index) => result
                .store_traces
                .get_mut(index)
                .map(|trace| &mut trace.values),
            SlotTarget::DeviceOp(index) => result
                .device_op_traces
                .get_mut(index)
                .map(|trace| &mut trace.values),
            SlotTarget::Parameter(_) => None,
        }
    }
}

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
}

/// A store directory, under a private name while its run is open.
struct RunStore {
    dir: PathBuf,
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
    let mut sink = WatchedSink {
        inner: StoreSink::new(WatchedIo {
            io,
            free: free.clone(),
            full: false,
        }),
        free,
        rows_needed: tstop / max_step,
        rows_likely: tstop / max_step.min(tstep),
        refused: None,
    };
    let run = ctx.engine.run_tran_into_sink_with_startup_mode_and_abort(
        ctx.netlist,
        tstop,
        max_step,
        startup_mode,
        abort,
        &mut sink,
    );
    let SinkTransient {
        result: shell,
        receipt,
        stopped,
    } = match run {
        Ok(done) => done,
        Err(source) => return Err(stopped_run(ctx, store, &segment, &sink, source)),
    };
    let opened = sink
        .inner
        .into_io()
        .map(|io| io.io)
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
            store.discard();
            return Err(CliError::InternalError { message });
        }
    };
    let mut shell = shell;
    if let Err(error) = fill_events(&mut reader, &mut shell) {
        let failure = store_error(&store.dir, error);
        store.discard();
        return Err(failure);
    }
    Ok(StoredTransient {
        shell,
        rows: StoredRows {
            parameters: reader
                .schema()
                .signals
                .iter()
                .filter(|signal| signal.quantity == "instance_parameter")
                .map(|signal| (signal.path.clone(), signal.name.clone()))
                .collect(),
            store,
            reader,
            targets,
            receipt,
            stopped,
        },
    })
}

/// Read the run's event signals back into the traces the result names: its digital and real
/// event traces and its impulse traces, each of which the run moved to the store as it went.
fn fill_events(
    reader: &mut SegmentReader<FileIo>,
    shell: &mut TransientResult,
) -> Result<(), StoreError> {
    use rspice_core::analysis::transient::{DigitalTracePoint, RealTracePoint};
    use rspice_core::xspice::DigitalValue;
    use rspice_core::{
        CurrentImpulseOwner, CurrentImpulsePoint, ImpulseDerivative, VoltageImpulsePoint,
    };
    use rspice_results_store::EventValue;
    let events: Vec<(usize, String, String, String)> = reader
        .schema()
        .signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| signal.kind.slots() == 0)
        .map(|(id, signal)| {
            (
                id,
                signal.quantity.clone(),
                signal.name.clone(),
                signal.path.clone(),
            )
        })
        .collect();
    let malformed = |what: &str, name: &str| StoreError::Io {
        kind: std::io::ErrorKind::InvalidData,
        message: format!("the transient store's {what} signal '{name}' holds a change it cannot"),
    };
    for (id, quantity, name, path) in events {
        let changes = reader.events(id, f64::NEG_INFINITY, f64::INFINITY)?;
        let real = |value: &EventValue| match value {
            EventValue::Real(value) => Some(*value),
            _ => None,
        };
        let current = |owner: &CurrentImpulseOwner| match owner {
            CurrentImpulseOwner::Branch { branch_name } => path.is_empty() && *branch_name == name,
            CurrentImpulseOwner::DeviceLead {
                device_name,
                parameter,
            } => *device_name == path && *parameter == name,
        };
        match quantity.as_str() {
            "digital" => {
                let Some(trace) = shell
                    .digital_traces
                    .iter_mut()
                    .find(|t| t.node_name == name)
                else {
                    continue;
                };
                trace.points = changes
                    .iter()
                    .map(|change| {
                        DigitalValue::from_store_strength(change.strength)
                            .map(|value| DigitalTracePoint {
                                time: change.time,
                                value,
                            })
                            .ok_or_else(|| malformed("digital", &name))
                    })
                    .collect::<Result<_, _>>()?;
            }
            "real_event" => {
                let Some(trace) = shell.real_traces.iter_mut().find(|t| t.node_name == name) else {
                    continue;
                };
                trace.points = changes
                    .iter()
                    .map(|change| {
                        real(&change.value)
                            .map(|value| RealTracePoint {
                                time: change.time,
                                value,
                            })
                            .ok_or_else(|| malformed("real event", &name))
                    })
                    .collect::<Result<_, _>>()?;
            }
            "current_impulse" | "current_impulse_derivative" => {
                let Some(trace) = shell
                    .current_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| current(&trace.owner))
                else {
                    continue;
                };
                if quantity == "current_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| {
                            real(&change.value)
                                .map(|charge_coulombs| CurrentImpulsePoint {
                                    time: change.time,
                                    charge_coulombs,
                                })
                                .ok_or_else(|| malformed("impulse", &name))
                        })
                        .collect::<Result<_, _>>()?;
                } else {
                    trace.derivatives =
                        derivatives(&changes).ok_or_else(|| malformed("impulse", &name))?;
                }
            }
            "voltage_impulse" | "voltage_impulse_derivative" => {
                let Some(trace) = shell
                    .voltage_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| trace.node_name == name)
                else {
                    continue;
                };
                if quantity == "voltage_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| {
                            real(&change.value)
                                .map(|volt_seconds| VoltageImpulsePoint {
                                    time: change.time,
                                    volt_seconds,
                                })
                                .ok_or_else(|| malformed("impulse", &name))
                        })
                        .collect::<Result<_, _>>()?;
                } else {
                    trace.derivatives =
                        derivatives(&changes).ok_or_else(|| malformed("impulse", &name))?;
                }
            }
            _ => {}
        }
    }
    /// An impulse derivative's order is its change's sequence.
    fn derivatives(
        changes: &[rspice_results_store::EventChange],
    ) -> Option<Vec<ImpulseDerivative>> {
        changes
            .iter()
            .map(|change| match change.value {
                EventValue::Real(coefficient) => Some(ImpulseDerivative {
                    time: change.time,
                    order: change.sequence,
                    coefficient,
                }),
                _ => None,
            })
            .collect()
    }
    Ok(())
}

/// Each slot's place in the result: the store's voltages are the saved nodes in node order, then
/// its currents the saved branches in branch order, so each name is the next one of its kind.
fn slot_targets(
    reader: &SegmentReader<FileIo>,
    shell: &TransientResult,
) -> Result<Vec<SlotTarget>, String> {
    let (mut next_node, mut next_branch, mut parameters) = (0usize, 0usize, 0usize);
    let mut targets = Vec::with_capacity(reader.slots());
    for signal in &reader.schema().signals {
        // An event signal takes no slot; its changes are read on their own.
        if signal.kind.slots() == 0 {
            continue;
        }
        if signal.quantity == "instance_parameter" {
            targets.push(SlotTarget::Parameter(parameters));
            parameters += 1;
            continue;
        }
        if signal.quantity == "step_size" {
            targets.push(SlotTarget::StepSize);
            continue;
        }
        if signal.quantity == "device_op" {
            let index = shell
                .device_op_traces
                .iter()
                .position(|trace| {
                    trace.device_name == signal.path && trace.parameter == signal.name
                })
                .ok_or_else(|| {
                    format!(
                        "the transient store's device value '{}' of '{}' is not a result device \
                         trace",
                        signal.name, signal.path
                    )
                })?;
            targets.push(SlotTarget::DeviceOp(index));
            continue;
        }
        if signal.quantity == "store" {
            let index = shell
                .store_traces
                .iter()
                .position(|trace| trace.name == signal.name)
                .ok_or_else(|| {
                    format!(
                        "the transient store's trace '{}' is not a result store trace",
                        signal.name
                    )
                })?;
            targets.push(SlotTarget::Store(index));
            continue;
        }
        let (names, next, voltage) = match signal.quantity.as_str() {
            "voltage" => (&shell.node_names, &mut next_node, true),
            "current" => (&shell.branch_names, &mut next_branch, false),
            other => {
                return Err(format!(
                    "the transient store holds a '{other}' signal '{}' the result has no column for",
                    signal.name
                ));
            }
        };
        let found = names[(*next).min(names.len())..]
            .iter()
            .position(|name| *name == signal.name)
            .map(|offset| *next + offset)
            .ok_or_else(|| {
                format!(
                    "the transient store's signal '{}' is not a result column",
                    signal.name
                )
            })?;
        *next = found + 1;
        targets.push(if voltage {
            SlotTarget::Voltage(found)
        } else {
            SlotTarget::Current(found)
        });
    }
    Ok(targets)
}

impl StoredTransient {
    /// Read every row back into the result and complete the outputs that read whole waveforms
    /// (`.FFT`, lead power), after which it equals the in-memory run's.
    pub(super) fn read_back(&mut self, ctx: &RunContext<'_>, tstop: f64) -> Result<(), CliError> {
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
    /// They can when the deck measures and asserts nothing, checks no `.VEC` file's expected
    /// outputs, prints no table of its own, keeps every accepted step, and asks for a rawfile, a
    /// delimited table, an event dump or nothing; and when every output signal is one of the
    /// store's columns, value for value. That last is checked by projecting the first chunk
    /// exactly as the whole run would be.
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
                    | OutputFormat::Vcd
            );
        let prints = netlist.output_requests.iter().any(|request| {
            matches!(
                request.directive,
                OutputDirectiveKind::Print | OutputDirectiveKind::Plot
            )
        });
        let every_step = netlist.options.output_time_points.is_empty()
            && netlist.options.output_interval_schedule.is_none();
        let rows = self.rows.rows();
        let parameters = !self.rows.parameters.is_empty();
        if !format_streams
            || parameters
            || !netlist.fft_analyses.is_empty()
            || prints
            || !every_step
            || !netlist.measurements.is_empty()
            || !netlist.assertions.is_empty()
            || !netlist.vector_files.is_empty()
            || rows == 0
            || !(tstart.is_finite() && tstart >= 0.0)
        {
            return Ok(None);
        }
        let probe_rows = self
            .rows
            .reader
            .chunk_spans()
            .next()
            .map_or(0, |span| span.1);
        self.rows.fill(&mut self.shell, 0, probe_rows as u64)?;
        let plan = probe_plan(ctx, &self.shell, &self.rows.targets);
        self.rows.clear(&mut self.shell);
        plan
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
pub(super) struct StreamPlan {
    /// Named and typed as the in-memory projection names them, without samples.
    pub(super) signals: Vec<ScalarSignal>,
    slots: Vec<usize>,
}

/// Project the probe rows in `shell` and keep the plan only when every projected signal is a
/// store column under its own name with its own values.
fn probe_plan(
    ctx: &RunContext<'_>,
    shell: &TransientResult,
    targets: &[SlotTarget],
) -> Result<Option<StreamPlan>, CliError> {
    let column = |target: SlotTarget| match target {
        SlotTarget::Voltage(node) => shell.voltages.get(node),
        SlotTarget::Current(branch) => shell.branch_currents.get(branch),
        SlotTarget::DeviceOp(index) => shell.device_op_traces.get(index).map(|trace| &trace.values),
        // An output that reads these reads the run back.
        SlotTarget::Parameter(_) | SlotTarget::StepSize | SlotTarget::Store(_) => None,
    };
    let inventory = match rspice_core::execution::transient_projection_signals(shell) {
        Ok(inventory) => inventory,
        Err(_) => return Ok(None),
    };
    let mut by_name: std::collections::HashMap<&str, Option<usize>> =
        std::collections::HashMap::new();
    for signal in &inventory {
        let rspice_core::execution::ProjectionValues::Real(values) = signal.values() else {
            continue;
        };
        let slot = targets
            .iter()
            .position(|&target| column(target).map(|c| c.as_ptr()) == Some(values.as_ptr()));
        by_name
            .entry(signal.descriptor().display_name())
            .and_modify(|seen| *seen = None)
            .or_insert(slot);
    }
    let Ok(mut signals) = crate::commands::run_signals::transient_export_signals(
        ctx.netlist,
        shell,
        ctx.transient_limits(),
        &crate::abort::ProcessAbort,
    ) else {
        // The run read back whole reports the same failure the in-memory run would.
        return Ok(None);
    };
    let mut slots = Vec::with_capacity(signals.len());
    for signal in &mut signals {
        let Some(&Some(slot)) = by_name.get(signal.display_name.as_str()) else {
            return Ok(None);
        };
        let Some(values) = column(targets[slot]) else {
            return Ok(None);
        };
        let same = signal.values.len() == values.len()
            && signal
                .values
                .iter()
                .zip(values)
                .all(|(a, b)| a.to_bits() == b.to_bits());
        if !same {
            return Ok(None);
        }
        signal.values = Vec::new();
        slots.push(slot);
    }
    Ok(Some(StreamPlan { signals, slots }))
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
        let dir = self.store.dir.clone();
        let failed = |error| store_error(&dir, error);
        result.time = self.reader.read_x(first, count).map_err(failed)?;
        for (slot, &target) in self.targets.iter().enumerate() {
            let Some(column) = target.column(result) else {
                continue;
            };
            *column = self.reader.read_slot(slot, first, count).map_err(failed)?;
        }
        Ok(())
    }

    fn clear(&self, result: &mut TransientResult) {
        result.time = Vec::new();
        for &target in &self.targets {
            if let Some(column) = target.column(result) {
                *column = Vec::new();
            }
        }
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
        let mut first: Vec<Option<(u64, f64)>> = vec![None; self.targets.len()];
        let dir = self.store.dir.clone();
        let spans = self.reader.chunk_spans().collect::<Vec<_>>();
        let mut values = Vec::new();
        for (chunk, &(start, _)) in spans.iter().enumerate() {
            for (slot, found) in first.iter_mut().enumerate() {
                // The in-memory check reads the saved waveforms and parameters only.
                if found.is_some()
                    || matches!(
                        self.targets[slot],
                        SlotTarget::StepSize | SlotTarget::Store(_) | SlotTarget::DeviceOp(_)
                    )
                {
                    continue;
                }
                self.reader
                    .read_chunk_slot(chunk, slot, &mut values)
                    .map_err(|error| store_error(&dir, error))?;
                if let Some(at) = values.iter().position(|value| !value.is_finite()) {
                    *found = Some((start + at as u64, values[at]));
                }
            }
        }
        let Some((slot, (point, value))) = first
            .iter()
            .enumerate()
            .find_map(|(slot, found)| found.map(|found| (slot, found)))
        else {
            return Ok(());
        };
        let name = match self.targets[slot] {
            SlotTarget::Voltage(node) => shell.node_names.get(node).map_or("node", String::as_str),
            SlotTarget::Current(branch) => shell
                .branch_names
                .get(branch)
                .map_or("branch", String::as_str),
            SlotTarget::Parameter(index) => self
                .parameters
                .get(index)
                .map_or("parameter", |(_, parameter)| parameter.as_str()),
            SlotTarget::StepSize => "step",
            SlotTarget::Store(index) => shell
                .store_traces
                .get(index)
                .map_or("store", |trace| trace.name.as_str()),
            SlotTarget::DeviceOp(index) => shell
                .device_op_traces
                .get(index)
                .map_or("device", |trace| trace.parameter.as_str()),
        };
        Err(super::shared::non_finite_error(
            "Transient",
            name,
            value,
            point,
            rows,
        ))
    }

    /// The rows an output carries over every accepted step from `tstart` to `tstop`, checked
    /// as the in-memory projection checks the time grid: `(first row, row count)`.
    pub(super) fn output_rows(&mut self, tstart: f64, tstop: f64) -> Result<(u64, u64), CliError> {
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

    /// A reader of rows `first ..` for `plan`'s signals, chunk by chunk.
    pub(super) fn cursor<'r>(&'r mut self, plan: &'r StreamPlan, first: u64) -> RowCursor<'r> {
        let spans = self.reader.chunk_spans().collect();
        RowCursor {
            dir: &self.store.dir,
            reader: &mut self.reader,
            slots: &plan.slots,
            spans,
            first,
            chunk: None,
            x: None,
            columns: vec![None; plan.slots.len()],
            error: None,
            non_finite: false,
        }
    }

    /// Leave the store beside `output` as `<stem>.rsr/`, replacing an earlier run's store there,
    /// or delete it when there is no output or `keep` is false. Returns where it was left.
    pub(super) fn publish(
        self,
        output: Option<&Path>,
        keep: bool,
    ) -> Result<Option<PathBuf>, CliError> {
        self.store.publish(output, keep)
    }

    /// What the store kept and its digest, for the run summary.
    pub(super) fn receipt(&self) -> SinkReceipt {
        self.receipt
    }
}

/// Reads the values of an output's rows from the store a chunk at a time, decoding each column of
/// a chunk once, when it is first asked for. An error or a non-finite value is recorded rather
/// than returned, since a table writer asks for numbers, and checked once the table is written.
pub(super) struct RowCursor<'r> {
    dir: &'r Path,
    reader: &'r mut SegmentReader<FileIo>,
    slots: &'r [usize],
    spans: Vec<(u64, usize)>,
    first: u64,
    chunk: Option<usize>,
    x: Option<Vec<f64>>,
    columns: Vec<Option<Vec<f64>>>,
    error: Option<StoreError>,
    non_finite: bool,
}

impl RowCursor<'_> {
    /// Variable `variable` (0 is time, then each output signal) of output row `row`.
    pub(super) fn value(&mut self, row: usize, variable: usize) -> f64 {
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
        let value = if variable == 0 {
            if self.x.is_none() {
                let mut x = Vec::new();
                if let Err(error) = self.reader.read_chunk_x(chunk, &mut x) {
                    self.error.get_or_insert(error);
                }
                self.x = Some(x);
            }
            self.x.as_ref().and_then(|x| x.get(offset)).copied()
        } else {
            let column = variable - 1;
            if self.columns[column].is_none() {
                let mut values = Vec::new();
                if let Err(error) =
                    self.reader
                        .read_chunk_slot(chunk, self.slots[column], &mut values)
                {
                    self.error.get_or_insert(error);
                }
                self.columns[column] = Some(values);
            }
            self.columns[column]
                .as_ref()
                .and_then(|values| values.get(offset))
                .copied()
        };
        let value = value.unwrap_or(f64::NAN);
        if !value.is_finite() {
            self.non_finite = true;
        }
        value
    }

    fn load(&mut self, row: u64) -> usize {
        let chunk = self
            .spans
            .partition_point(|&(start, rows)| start + rows as u64 <= row)
            .min(self.spans.len().saturating_sub(1));
        self.chunk = Some(chunk);
        self.x = None;
        self.columns.iter_mut().for_each(|column| *column = None);
        chunk
    }

    /// Fail when a read failed or a value was not finite, as a table that held them would.
    pub(super) fn finish(self, path: &Path) -> Result<(), CliError> {
        if let Some(error) = self.error {
            return Err(store_error(self.dir, error));
        }
        if self.non_finite {
            return Err(crate::commands::export_table::invalid_samples(path));
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
        Ok(Self { dir })
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
            ..Member::default()
        });
        StoreDir::new(&self.dir).save_manifest(&manifest)
    }

    fn publish(self, output: Option<&Path>, keep: bool) -> Result<Option<PathBuf>, CliError> {
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
        // A store that cannot be removed is left under its private name; the run's own outcome
        // is what the caller reports.
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
    CliError::output_error(
        dir,
        std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string()),
    )
}

/// The error a run that did not complete ends with. A run refused before solving, or stopped for
/// any reason but full storage, leaves nothing; one its storage stopped keeps its store, readable
/// to its last whole chunk, beside where its output would have gone.
fn stopped_run(
    ctx: &RunContext<'_>,
    store: RunStore,
    segment: &Path,
    sink: &WatchedSink,
    source: rspice_core::SimulationError,
) -> CliError {
    use rspice_core::result_sink::SinkStop;
    if let Some((needed, available)) = sink.refused {
        store.discard();
        return CliError::CoreSimulationError {
            source: rspice_core::SimulationError::ResultSink(SinkStop::DiskFull {
                needed: Some(needed),
                available: Some(available),
            }),
            analysis: Some(format!(
                "Transient, refused before solving: its results store needs at least {needed} \
                 bytes for {:.0} rows on a volume with {available} bytes free",
                sink.rows_needed.ceil()
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
        let free = (sink.free)();
        store.discard();
        return storage_full(None, SinkReceipt::default(), None, free);
    };
    let rows = reader.rows();
    let at = rows
        .checked_sub(1)
        .and_then(|last| reader.read_x(last, 1).ok())
        .and_then(|x| x.first().copied());
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
    storage_full(at, receipt, kept_at, (sink.free)())
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

/// Free bytes on the store's volume, asked when the run begins and before each write that grows
/// its segment.
type FreeSpace = std::sync::Arc<dyn Fn() -> Option<u64> + Send + Sync>;

fn free_space(dir: &Path) -> FreeSpace {
    #[cfg(test)]
    if let Some(quota) = tests::QUOTA.with(std::cell::Cell::get) {
        return tests::quota(&dir.join(StoreDir::segment_name(0)), quota);
    }
    let dir = dir.to_path_buf();
    std::sync::Arc::new(move || available_bytes(&dir))
}

/// Bytes an unprivileged writer may still use on the volume holding `path`.
#[cfg(unix)]
fn available_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    // SAFETY: `path` is NUL-terminated and outlives the call, and `stat` is a writable `statvfs`
    // that the call fills when it returns 0, the only case in which it is read.
    let stat = unsafe {
        if libc::statvfs(path.as_ptr(), stat.as_mut_ptr()) != 0 {
            return None;
        }
        stat.assume_init()
    };
    #[allow(clippy::unnecessary_cast)]
    Some((stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64))
}

/// Bytes the calling user may still use on the volume holding `path`, quotas included.
#[cfg(windows)]
fn available_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    // A share's root is named with its trailing separator.
    if wide.last() != Some(&u16::from(b'\\')) {
        wide.push(u16::from(b'\\'));
    }
    wide.push(0);
    let mut available = 0u64;
    // SAFETY: `wide` is NUL-terminated and outlives the call, `available` is a writable `u64` the
    // call fills when it succeeds, and the two totals it is not asked for are null.
    let succeeded = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut available,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    (succeeded != 0).then_some(available)
}

/// Without a free-space query, the operating system's own refusal of a write is the stop.
#[cfg(not(any(unix, windows)))]
fn available_bytes(_path: &Path) -> Option<u64> {
    None
}

/// Free space a segment keeps for closing once a write was refused: its last index and `END`.
const CLOSING_RESERVE: u64 = 1 << 20;

/// A segment file that keeps room on its volume for the segment to close: a write that grows it
/// past the free space less [`CLOSING_RESERVE`] fails as storage full. Every write after that is
/// the writer closing the segment, which the reserve is for.
struct WatchedIo {
    io: FileIo,
    free: FreeSpace,
    full: bool,
}

impl rspice_results_store::SegmentIo for WatchedIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.io.read_at(offset, buf)
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        if !self.full {
            let grows = (offset + data.len() as u64).saturating_sub(self.io.size()?);
            if grows > 0
                && (self.free)().is_some_and(|free| free < grows.saturating_add(CLOSING_RESERVE))
            {
                self.full = true;
                return Err(std::io::ErrorKind::StorageFull.into());
            }
        }
        let written = self.io.write_at(offset, data);
        // The volume filled before the watch saw it: the writes closing the segment still try.
        if written
            .as_ref()
            .is_err_and(|error| error.kind() == std::io::ErrorKind::StorageFull)
        {
            self.full = true;
        }
        written
    }

    fn truncate(&mut self, len: u64) -> std::io::Result<()> {
        self.io.truncate(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }

    fn size(&self) -> std::io::Result<u64> {
        self.io.size()
    }
}

/// The store's sink with the disk preflight: when the run begins, the rows it cannot avoid
/// writing must fit in the free space.
struct WatchedSink {
    inner: StoreSink<WatchedIo>,
    free: FreeSpace,
    /// The fewest rows the run can write: no step is longer than its largest.
    rows_needed: f64,
    /// The rows it likely writes.
    rows_likely: f64,
    /// `(bytes needed, bytes free)` when the preflight refused the run.
    refused: Option<(u64, u64)>,
}

impl WatchedSink {
    /// Bytes `rows` rows of `slots` values take in a segment at most: every value uncompressed,
    /// plus 3.5 % for frames, indexes and summaries.
    fn bytes(rows: f64, slots: usize) -> u64 {
        let bytes = (rows.max(1.0) * (slots as f64 + 1.0) * 8.0 * 1.035).ceil();
        if bytes.is_finite() && bytes < u64::MAX as f64 {
            bytes as u64
        } else {
            u64::MAX
        }
    }
}

impl rspice_core::result_sink::ResultSink for WatchedSink {
    fn begin(
        &mut self,
        schema: &rspice_core::result_sink::ResultSchema,
        meta: &rspice_core::result_sink::RunMeta,
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        if let Some(free) = (self.free)() {
            let needed = Self::bytes(self.rows_needed, schema.slots());
            if needed > free {
                self.refused = Some((needed, free));
                return Err(rspice_core::result_sink::SinkStop::DiskFull {
                    needed: Some(needed),
                    available: Some(free),
                });
            }
            let likely = Self::bytes(self.rows_likely, schema.slots());
            if likely > free {
                crate::observability::diagnostic(
                    "results_store_space",
                    None,
                    format_args!(
                        "Warning: this transient likely writes about {likely} bytes of results \
                         and their volume has {free} bytes free; it stops, keeping what it \
                         wrote, if the volume fills"
                    ),
                );
            }
        }
        self.inner.begin(schema, meta)
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.inner.row(x, values)
    }

    fn amend_open_row(
        &mut self,
        row: u64,
        column: u32,
        value: f64,
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.inner.amend_open_row(row, column, value)
    }

    fn events(
        &mut self,
        changes: &[rspice_core::result_sink::EventChange],
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.inner.events(changes)
    }

    fn end(
        &mut self,
        status: rspice_core::result_sink::RunStatus,
    ) -> Result<SinkReceipt, rspice_core::result_sink::SinkStop> {
        self.inner.end(status)
    }
}

#[cfg(test)]
mod tests;
