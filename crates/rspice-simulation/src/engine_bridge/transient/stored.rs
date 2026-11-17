//! A transient the desktop writes to a results store as it runs.
//!
//! The engine hands every accepted point to a sink over a segment in this process's store
//! folder, so the run's waveforms never sit in memory: the result's time axis and waveforms are
//! windows of the segment's columns, read when something first asks for their values. Before the
//! first row, the store's volume must have room for the rows the run cannot avoid writing, or the
//! run is refused before it solves; a volume that fills while it runs stops it at that accepted
//! point, keeping every row written.
//!
//! A deck whose `.FFT` cards or lead-power outputs read whole waveforms after the run is read
//! back whole, completed by the engine and converted in memory, as before. `.MEAS TRAN` and
//! `.ASSERT` read the rows back for their own evaluation only.

use std::sync::Arc;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{SinkTransient, TransientResult, TransientState};
use rspice_core::result_sink::SinkStop;
use rspice_results::stored_transient::{
    SlotTarget, clear_rows, fill_events, fill_rows, fill_span, instance_parameters, slot_targets,
};
use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
use rspice_results_store::{FileIo, SegmentReader, StoreError, StoreSink, WatchedIo, WatchedSink};
use rspice_simulation_contract::resource_failure::ResourceFailure;
use rspice_simulation_contract::result_storage::ResultStorageFailure;

use super::super::{EngineBridge, ensure_not_aborted};
use crate::error::SimulationError;
use crate::results::{SampledWaveform, TransientStoreReceipt, TransientStoreStop};
use crate::results_store::{StoredColumn, StoredRun};

/// What the store's object is called when it cannot be had.
const STORE_OBJECT: &str = "the transient's results store";

/// A transient run into a store, as it came back.
pub(super) enum StoredOutcome {
    /// Its rows stay in the store.
    Stored(StoredTransient),
    /// It was read back whole for the outputs that read whole waveforms after the run, and its
    /// segment is gone.
    ReadBack(TransientResult),
}

/// A transient whose rows stay in its store.
pub(super) struct StoredTransient {
    /// The run's result with its time and saved columns empty and its events read back.
    pub(super) shell: TransientResult,
    pub(super) run: Arc<StoredRun>,
    pub(super) targets: Vec<SlotTarget>,
    /// `(device, parameter)` of each instance-parameter column.
    pub(super) parameters: Vec<(String, String)>,
    pub(super) receipt: TransientStoreReceipt,
}

/// The stop time, output step and largest step of the run.
pub(super) struct StoreSpan {
    pub(super) tstop: f64,
    pub(super) tstep: f64,
    pub(super) max_step: f64,
}

impl EngineBridge {
    /// Run the transient into a new segment of this process's store folder. `capture_times`
    /// are the times whose states the deck's small-signal cards take, captured as the run
    /// passes them.
    pub(super) fn run_transient_into_store(
        &self,
        engine: &rspice_core::Engine,
        netlist: &rspice_core::Netlist,
        span: StoreSpan,
        startup_mode: rspice_core::engine::TransientStartupMode,
        capture_times: &[f64],
        abort: &dyn AbortSignal,
    ) -> Result<(StoredOutcome, Vec<TransientState>), SimulationError> {
        let (path, io) = crate::results_store::create_segment().map_err(|cause| {
            self.translate_error(rspice_core::SimulationError::ResultSink(SinkStop::Io(
                format!("{STORE_OBJECT} could not be created: {cause}"),
            )))
        })?;
        let dir = path.parent().unwrap_or(&path).to_path_buf();
        let free = crate::results_store::free_space(&path);
        let mut sink = WatchedSink::new(
            StoreSink::new(WatchedIo::new(io, free.clone())),
            free,
            span.tstop / span.max_step,
            span.tstop / span.max_step.min(span.tstep),
        )
        .with_space_warning(Box::new({
            let dir = dir.clone();
            move |likely, free| {
                log::warn!(
                    "This transient likely writes about {likely} bytes of results and the volume \
                     holding {} has {free} bytes free; it stops, keeping what it wrote, if the \
                     volume fills",
                    dir.display()
                );
            }
        }));
        let ran = if capture_times.is_empty() {
            engine
                .run_tran_into_sink_with_abort(netlist, span.tstop, span.max_step, abort, &mut sink)
                .map(|run| (run, Vec::new()))
        } else {
            engine.run_tran_capturing_states_to_stop_into_sink(
                netlist,
                (span.tstop, span.max_step),
                startup_mode,
                capture_times,
                abort,
                &mut sink,
            )
        };
        let ran = match ran {
            Ok((
                SinkTransient {
                    result,
                    receipt,
                    stopped,
                },
                states,
            )) => Ok((Some(result), receipt.digest, stopped, states)),
            Err(source) => Err(source),
        };
        let refused = sink.refused();
        let rows_needed = sink.rows_needed();
        let (inner, free) = sink.into_parts();
        let (shell, digest, stopped, states) = match ran {
            Ok(ran) => ran,
            // The volume filled as the run handed over its events: it stopped there, and its
            // segment keeps every whole chunk it wrote.
            Err(rspice_core::SimulationError::ResultSink(stop @ SinkStop::DiskFull { .. }))
                if refused.is_none() =>
            {
                (None, None, Some(stop), Vec::new())
            }
            Err(source) => {
                // The segment closes before it is deleted: a host may refuse to delete an open
                // file.
                drop(inner);
                let _ = std::fs::remove_file(&path);
                return Err(match refused {
                    Some((needed, available)) => {
                        SimulationError::ResultStorage(ResultStorageFailure::Insufficient {
                            rows: rows_needed.ceil() as u64,
                            needed,
                            available,
                            folder: dir.display().to_string(),
                        })
                    }
                    None => self.translate_error(source),
                });
            }
        };
        let reader = match inner.into_io() {
            Some(io) => SegmentReader::open(io.into_inner()),
            None => FileIo::open(&path)
                .map_err(StoreError::from)
                .and_then(SegmentReader::open),
        };
        let reader = match reader {
            Ok(reader) if shell.is_some() || reader.rows() > 0 => reader,
            // A volume that filled before the segment held a whole chunk kept nothing.
            Ok(_) => {
                let _ = std::fs::remove_file(&path);
                return Err(SimulationError::ResultStorage(
                    ResultStorageFailure::from_sink_stop(
                        stopped.unwrap_or(SinkStop::DiskFull {
                            needed: None,
                            available: free(),
                        }),
                        Some(dir.display().to_string()),
                    ),
                ));
            }
            Err(error) => {
                let _ = std::fs::remove_file(&path);
                return Err(store_failure(&path, error));
            }
        };
        let bytes = std::fs::metadata(&path).map_or(0, |meta| meta.len());
        let run = Arc::new(StoredRun::new(reader, path.clone()));
        let (shell, targets, parameters) = run
            .with_reader(|reader| {
                // A run that ended with an error returned no result: the segment names it.
                let mut shell = match shell {
                    Some(shell) => shell,
                    None => {
                        match reader.annotation(rspice_core::engine::TRANSIENT_SHELL_ANNOTATION) {
                            Some(bytes) => rspice_core::engine::decode_transient_shell(bytes),
                            None => {
                                rspice_results::stored_transient::transient_shell(reader.schema())
                            }
                        }
                        .map_err(StoreError::Schema)?
                    }
                };
                let targets = slot_targets(reader, &shell).map_err(StoreError::Schema)?;
                fill_events(reader, &mut shell)?;
                Ok((shell, targets, instance_parameters(reader)))
            })
            .map_err(|error| store_failure(&path, error))?;
        let stopped = match stopped {
            Some(SinkStop::DiskFull { .. }) => Some(TransientStoreStop {
                at: last_time(&run),
                free: free(),
            }),
            Some(other) => {
                return Err(SimulationError::ResultStorage(
                    ResultStorageFailure::from_sink_stop(other, Some(dir.display().to_string())),
                ));
            }
            None => None,
        };
        let receipt = TransientStoreReceipt {
            path,
            rows: run.rows(),
            bytes,
            digest,
            stopped,
        };
        let mut stored = StoredTransient {
            shell,
            run,
            targets,
            parameters,
            receipt,
        };
        // A run its volume stopped keeps its rows where they are: the outputs that read whole
        // waveforms once the run ends are not taken over the part it kept.
        if stored.receipt.stopped.is_some() || engine.transient_rows_are_whole(netlist) {
            return Ok((StoredOutcome::Stored(stored), states));
        }
        // The deck's `.FFT` spectra and lead powers read the whole waveforms once the run ends.
        stored.read_rows()?;
        engine
            .finish_transient_from_rows(netlist, &mut stored.shell, span.tstop, abort)
            .map_err(|error| self.translate_error(error))?;
        Ok((StoredOutcome::ReadBack(stored.shell), states))
    }
}

impl StoredTransient {
    /// Read every row into the shell, with each instance parameter as the constant trace the
    /// run kept, so it is the result the run returned in memory.
    pub(super) fn read_rows(&mut self) -> Result<(), SimulationError> {
        let rows = self.rows()?;
        let Self {
            shell,
            run,
            targets,
            parameters,
            receipt,
        } = self;
        run.with_reader(|reader| {
            fill_rows(reader, targets, shell, 0, rows)?;
            for (slot, &target) in targets.iter().enumerate() {
                let SlotTarget::Parameter(index) = target else {
                    continue;
                };
                let values = reader.read_slot(slot, 0, rows)?;
                let (device, parameter) = &parameters[index];
                shell.adopt_instance_parameter(device, parameter, values);
            }
            Ok(())
        })
        .map_err(|error| store_failure(&receipt.path, error))
    }

    /// Give the rows [`Self::read_rows`] read back their memory, keeping the shell's
    /// `device_traces` traces of its own.
    pub(super) fn drop_rows(&mut self, device_traces: usize) {
        clear_rows(&self.targets, &mut self.shell);
        self.shell.device_op_traces.truncate(device_traces);
    }

    /// Read the first and last rows into the shell, each instance parameter included: what the
    /// shape checks and the event history read of the run.
    pub(super) fn read_span(&mut self) -> Result<(), SimulationError> {
        let Self {
            shell,
            run,
            targets,
            parameters,
            receipt,
        } = self;
        run.with_reader(|reader| {
            fill_span(reader, targets, shell)?;
            let rows = reader.rows();
            let ends: &[u64] = if rows > 1 { &[0, rows - 1] } else { &[0] };
            for (slot, &target) in targets.iter().enumerate() {
                let SlotTarget::Parameter(index) = target else {
                    continue;
                };
                let mut values = Vec::with_capacity(ends.len());
                for &row in ends {
                    values.extend(reader.read_slot(slot, row, 1)?);
                }
                let (device, parameter) = &parameters[index];
                shell.adopt_instance_parameter(device, parameter, values);
            }
            Ok(())
        })
        .map_err(|error| store_failure(&receipt.path, error))
    }

    fn rows(&self) -> Result<usize, SimulationError> {
        usize::try_from(self.run.rows()).map_err(|_| {
            store_failure(
                &self.receipt.path,
                StoreError::Schema("the segment holds more rows than this host can address".into()),
            )
        })
    }

    /// One pass over every chunk: the checks the in-memory result's shape check makes of its
    /// samples, the output window that starts at `start_time`, and the time of each row in
    /// `forced` (ascending).
    pub(super) fn scan(
        &self,
        start_time: f64,
        forced: &[usize],
        abort: &dyn AbortSignal,
    ) -> Result<RowScan, SimulationError> {
        let shell = &self.shell;
        let voltages = shell.voltages.len();
        let mut scan = RowScan {
            rows: self.run.rows(),
            first: 0.0,
            last: 0.0,
            start_idx: 0,
            lead: None,
            forced_times: Vec::with_capacity(forced.len()),
        };
        let mut axis_invalid = false;
        let mut steps_invalid = false;
        // The first trace (in result order) with a non-finite sample, and the first auxiliary
        // trace with a value its check refuses.
        let mut bad_trace: Option<usize> = None;
        let mut bad_device = false;
        let mut bad_store = false;
        let mut before_start: Option<f64> = None;
        let mut start_found = !(start_time.is_finite() && start_time > 0.0);
        let mut next_forced = 0;
        let mut previous: Option<f64> = None;
        self.run
            .with_reader(|reader| {
                let spans = reader.chunk_spans().collect::<Vec<_>>();
                let mut values = Vec::new();
                for (chunk, &(first_row, rows)) in spans.iter().enumerate() {
                    if abort.is_aborted() {
                        return Ok(());
                    }
                    reader.read_chunk_x(chunk, &mut values)?;
                    for (offset, &x) in values.iter().enumerate().take(rows) {
                        let row = first_row as usize + offset;
                        if !x.is_finite() || previous.is_some_and(|previous| x <= previous) {
                            axis_invalid = true;
                        }
                        if row == 0 {
                            scan.first = x;
                        }
                        if !start_found && x >= start_time {
                            start_found = true;
                            scan.start_idx = row;
                            if row > 0 && x > start_time {
                                scan.lead = before_start.map(|lower| (lower, x));
                            }
                        }
                        before_start = Some(x);
                        while forced.get(next_forced) == Some(&row) {
                            scan.forced_times.push(x);
                            next_forced += 1;
                        }
                        previous = Some(x);
                    }
                    for (slot, &target) in self.targets.iter().enumerate() {
                        let trace = match target {
                            SlotTarget::Voltage(node) => Some(node),
                            SlotTarget::Current(branch) => Some(voltages + branch),
                            _ => None,
                        };
                        let checked = match target {
                            SlotTarget::StepSize => !steps_invalid,
                            SlotTarget::DeviceOp(_) | SlotTarget::Parameter(_) => !bad_device,
                            SlotTarget::Store(_) => !bad_store,
                            _ => trace.is_some_and(|trace| bad_trace.is_none_or(|bad| trace < bad)),
                        };
                        if !checked {
                            continue;
                        }
                        reader.read_chunk_slot(chunk, slot, &mut values)?;
                        let values = &values[..rows.min(values.len())];
                        match target {
                            SlotTarget::StepSize => {
                                steps_invalid |=
                                    values.iter().any(|step| !step.is_finite() || *step < 0.0);
                            }
                            SlotTarget::DeviceOp(_) | SlotTarget::Parameter(_) => {
                                bad_device |= values.iter().any(|value| value.is_infinite());
                            }
                            SlotTarget::Store(_) => {
                                bad_store |= values.iter().any(|value| !value.is_finite());
                            }
                            SlotTarget::Voltage(_) | SlotTarget::Current(_) => {
                                if values.iter().any(|value| !value.is_finite()) {
                                    bad_trace = trace;
                                }
                            }
                        }
                    }
                }
                scan.last = previous.unwrap_or(scan.first);
                Ok(())
            })
            .map_err(|error| store_failure(&self.receipt.path, error))?;
        ensure_not_aborted(abort)?;
        if scan.rows == 0 {
            return Err(SimulationError::SolverError(
                "transient engine returned an empty time axis".to_owned(),
            ));
        }
        if steps_invalid {
            return Err(SimulationError::SolverError(
                "transient engine returned an invalid accepted-step history".to_owned(),
            ));
        }
        if axis_invalid {
            return Err(SimulationError::SolverError(
                "transient engine returned a non-finite or non-increasing time axis".to_owned(),
            ));
        }
        if let Some(trace) = bad_trace {
            return Err(SimulationError::SolverError(format!(
                "transient waveform {} contains a non-finite sample",
                trace + 1
            )));
        }
        if bad_device {
            return Err(SimulationError::SolverError(
                "transient engine returned an invalid device operating-point trace".to_owned(),
            ));
        }
        if bad_store {
            return Err(SimulationError::SolverError(
                "transient engine returned an invalid device store trace".to_owned(),
            ));
        }
        if !start_found {
            scan.start_idx = usize::try_from(scan.rows).unwrap_or(usize::MAX);
        }
        Ok(scan)
    }

    /// The result's waveforms: a window of each saved node's and branch's column over the
    /// output window, every one on one window of the time column.
    pub(super) fn waveforms(
        &self,
        window: &OutputWindow,
        abort: &dyn AbortSignal,
    ) -> Result<
        (
            WaveformValues,
            std::collections::HashMap<String, SampledWaveform>,
        ),
        SimulationError,
    > {
        let stored = |slot: Option<usize>, name: String, lead: WindowLead| {
            let column = StoredColumn::new(Arc::clone(&self.run), slot, name);
            WaveformValues::stored(
                column,
                StoredWindow {
                    first: window.start_idx as u64,
                    rows: window.sample_count as u64,
                    lead,
                },
            )
            .map_err(SimulationError::SolverError)
        };
        let time = stored(
            None,
            "time".to_owned(),
            window
                .lead
                .map_or(WindowLead::None, |_| WindowLead::Value(window.start_time)),
        )?;
        let value_lead = window.lead.map_or(WindowLead::None, |(_, fraction)| {
            WindowLead::Interpolated(fraction)
        });
        let mut waveforms = std::collections::HashMap::new();
        for (slot, &target) in self.targets.iter().enumerate() {
            ensure_not_aborted(abort)?;
            let (name, unit) = match target {
                SlotTarget::Voltage(node) => {
                    let Some((name, unit)) = super::node_waveform(&self.shell, node) else {
                        continue;
                    };
                    (name, unit.to_owned())
                }
                SlotTarget::Current(branch) => (
                    super::branch_waveform_name(&self.shell.branch_names[branch]),
                    "A".to_owned(),
                ),
                _ => continue,
            };
            let values = stored(Some(slot), name.clone(), value_lead)?;
            waveforms.insert(
                name.clone(),
                SampledWaveform::new_time_domain_in_unit(&name, time.clone(), values, unit),
            );
        }
        Ok((time, waveforms))
    }
}

/// What one pass over a stored run's rows found.
pub(super) struct RowScan {
    pub(super) rows: u64,
    /// The first and last rows' times.
    pub(super) first: f64,
    pub(super) last: f64,
    /// The first row at or after the output start.
    pub(super) start_idx: usize,
    /// The times of the rows either side of the output start, when it falls between them.
    pub(super) lead: Option<(f64, f64)>,
    /// The time of each force-accepted row asked for.
    pub(super) forced_times: Vec<f64>,
}

/// The rows a transient's output shows: from `start_idx` on, after the authored start itself
/// when it falls between two rows (`lead`: the row before's index and the fraction of the way
/// from it to `start_idx`).
pub(super) struct OutputWindow {
    pub(super) start_time: f64,
    pub(super) start_idx: usize,
    pub(super) sample_count: usize,
    pub(super) lead: Option<(usize, f64)>,
}

/// The time of a stored run's last row.
fn last_time(run: &StoredRun) -> Option<f64> {
    let last = run.rows().checked_sub(1)?;
    let mut time = [0.0];
    run.read(None, last, &mut time).ok()?;
    Some(time[0])
}

/// A failure to read the run's own segment back, naming the segment.
fn store_failure(path: &std::path::Path, error: StoreError) -> SimulationError {
    match error {
        StoreError::Allocation {
            object,
            bytes,
            source,
        } => SimulationError::ResourceFailure(ResourceFailure::Allocation {
            object: object.to_owned(),
            bytes: bytes as u64,
            detail: source.to_string(),
        }),
        error => SimulationError::ResultStorage(ResultStorageFailure::Io {
            cause: format!(
                "the transient's results segment {} could not be read back: {error}",
                path.display()
            ),
        }),
    }
}

#[cfg(test)]
mod tests;
