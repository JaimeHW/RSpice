//! The store as an analysis's result sink.
//!
//! [`StoreSink`] implements the engine's `ResultSink` over a [`RunWriter`]: the run's schema and
//! metadata create the segment, each row is staged into the open chunk, and `end` writes `END`.
//! The engine's sink types mirror the store's own one for one; this module maps between them, and
//! maps a [`StoreError`] onto the typed stop the engine reports.

use rspice_core_base::result_sink::{self as sink, ResultSink, SinkReceipt, SinkStop};

use crate::{
    Axis, EventChange, EventValue, Logic, RunIo, RunMeta, RunStatus, RunWriter, Schema, Signal,
    SignalKind, StopReason, StoreError, StoreReceipt,
};

/// Writes a run's rows into one segment on `I`.
///
/// The segment is created when the run begins and closed when it ends; the I/O comes back with
/// [`StoreSink::into_io`]. A sink the run never began writes nothing, and refuses to end.
pub struct StoreSink<I: RunIo> {
    io: Option<I>,
    writer: Option<RunWriter<I>>,
    receipt: Option<StoreReceipt>,
    /// Event changes converted for the writer, reused across calls.
    changes: Vec<EventChange>,
    /// A sink appending to a segment a resumed run continues: the x of the segment's last kept
    /// row, which the run's own rows and changes at or before it repeat.
    seam: Option<Seam>,
}

/// Where a resumed run meets the segment it continues.
struct Seam {
    through: f64,
    /// The segment's rows when it was reopened.
    kept: u64,
    /// The run's rows at or before `through`, which the segment already holds.
    repeated: u64,
}

impl<I: RunIo> StoreSink<I> {
    /// A sink that writes its segment on `io`, which is emptied when the run begins.
    pub fn new(io: I) -> Self {
        Self {
            io: Some(io),
            writer: None,
            receipt: None,
            changes: Vec::new(),
            seam: None,
        }
    }

    /// A sink that appends a resumed run to the segment on `io`, which holds the rows of the run
    /// it continues: the segment keeps its rows and event changes up to its row at `through`, the
    /// resumed run's start, and drops the rest; the run's rows and changes at or before `through`,
    /// which the segment already holds, are not written again. The run must record the same
    /// signals. A segment with no row at `through` is left as it was and refused with
    /// [`StoreError::NoRowAt`].
    pub fn appending(io: I, through: f64) -> Result<Self, StoreError> {
        let writer = RunWriter::reopen(io, through)?;
        Ok(Self {
            io: None,
            seam: Some(Seam {
                through,
                kept: writer.rows(),
                repeated: 0,
            }),
            writer: Some(writer),
            receipt: None,
            changes: Vec::new(),
        })
    }

    /// What `END` recorded, once the run ended.
    pub fn receipt(&self) -> Option<&StoreReceipt> {
        self.receipt.as_ref()
    }

    /// The segment's I/O, once the run ended or if it never began. `None` while the segment is
    /// open, or after the writer failed to close it.
    pub fn into_io(self) -> Option<I> {
        self.io
    }

    fn writer(&mut self) -> Result<&mut RunWriter<I>, SinkStop> {
        self.writer
            .as_mut()
            .ok_or_else(|| SinkStop::Io("the results segment is not open".to_owned()))
    }
}

impl<I: RunIo> ResultSink for StoreSink<I> {
    fn begin(&mut self, schema: &sink::ResultSchema, meta: &sink::RunMeta) -> Result<(), SinkStop> {
        if self.seam.is_some() {
            let mut bytes = Vec::new();
            store_schema(schema).encode(&mut bytes);
            return if self.writer()?.schema_bytes() == bytes.as_slice() {
                Ok(())
            } else {
                Err(SinkStop::Io(
                    "the resumed run does not record the signals of the run it continues"
                        .to_owned(),
                ))
            };
        }
        let io = self
            .io
            .take()
            .ok_or_else(|| SinkStop::Io("the results segment was already begun".to_owned()))?;
        let writer =
            RunWriter::create(io, &store_schema(schema), &store_meta(meta)).map_err(sink_stop)?;
        self.writer = Some(writer);
        Ok(())
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        if let Some(seam) = &mut self.seam
            && x <= seam.through
        {
            seam.repeated += 1;
            return Ok(());
        }
        self.writer()?.push_row(x, values).map_err(sink_stop)
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        let row = match &self.seam {
            Some(seam) => seam
                .kept
                .checked_add(row)
                .and_then(|row| row.checked_sub(seam.repeated))
                .filter(|_| row >= seam.repeated)
                .ok_or_else(|| {
                    SinkStop::Io(format!(
                        "the resumed run amended its row {row}, which the store already holds"
                    ))
                })?,
            None => row,
        };
        self.writer()?
            .amend_open_row(row, column as usize, value)
            .map_err(sink_stop)
    }

    fn events(&mut self, changes: &[sink::EventChange]) -> Result<(), SinkStop> {
        let mut converted = std::mem::take(&mut self.changes);
        converted.clear();
        let seam = self.seam.as_ref();
        converted.extend(
            changes
                .iter()
                .filter(|change| seam.is_none_or(|seam| change.time > seam.through))
                .map(store_change),
        );
        let pushed = self.writer()?.push_events(&converted).map_err(sink_stop);
        self.changes = converted;
        pushed
    }

    fn annotate(&mut self, key: &str, bytes: &[u8]) -> Result<(), SinkStop> {
        let Some(writer) = self.writer.as_mut() else {
            return Err(SinkStop::Io(
                "the results segment was annotated without being begun".to_owned(),
            ));
        };
        writer.annotate(key, bytes.to_vec());
        Ok(())
    }

    fn end(&mut self, status: sink::RunStatus) -> Result<SinkReceipt, SinkStop> {
        let Some(writer) = self.writer.take() else {
            return Err(SinkStop::Io(
                "the results segment ended without being begun".to_owned(),
            ));
        };
        let (receipt, io) = writer.finish(store_status(status)).map_err(sink_stop)?;
        self.io = Some(io);
        self.receipt = Some(receipt.clone());
        Ok(SinkReceipt {
            rows: receipt.rows,
            events: receipt.events,
            bytes: receipt.bytes,
            digest: Some(receipt.digest.0),
        })
    }
}

/// The engine's typed stop for a store error: storage and memory exhaustion stay typed, every
/// other failure carries its message.
pub fn sink_stop(error: StoreError) -> SinkStop {
    match error {
        StoreError::DiskFull => SinkStop::DiskFull {
            needed: None,
            available: None,
        },
        StoreError::MemoryFull => SinkStop::MemoryFull { bytes: None },
        other => SinkStop::Io(other.to_string()),
    }
}

fn store_schema(schema: &sink::ResultSchema) -> Schema {
    Schema {
        x: Axis {
            name: schema.x.name.clone(),
            unit: schema.x.unit.clone(),
        },
        signals: schema
            .signals
            .iter()
            .map(|signal| Signal {
                name: signal.name.clone(),
                unit: signal.unit.clone(),
                kind: match signal.kind {
                    sink::ResultSignalKind::Real => SignalKind::Real,
                    sink::ResultSignalKind::Complex => SignalKind::Complex,
                    sink::ResultSignalKind::Logic => SignalKind::Logic,
                    sink::ResultSignalKind::RealEvent => SignalKind::RealEvent,
                    sink::ResultSignalKind::Bus { width } => SignalKind::Bus { width },
                },
                quantity: signal.quantity.clone(),
                path: signal.path.clone(),
            })
            .collect(),
    }
}

fn store_meta(meta: &sink::RunMeta) -> RunMeta {
    RunMeta {
        analysis: meta.analysis.clone(),
        sweep: meta.sweep.clone(),
        seed: meta.seed,
        options: meta.options.clone(),
    }
}

fn store_change(change: &sink::EventChange) -> EventChange {
    EventChange {
        signal: change.signal,
        time: change.time,
        sequence: change.sequence,
        value: match &change.value {
            sink::EventValue::Logic(logic) => EventValue::Logic(match logic {
                sink::Logic::Zero => Logic::Zero,
                sink::Logic::One => Logic::One,
                sink::Logic::X => Logic::X,
                sink::Logic::Z => Logic::Z,
            }),
            sink::EventValue::Real(value) => EventValue::Real(*value),
            sink::EventValue::Bus { value, x, z } => EventValue::Bus {
                value: value.clone(),
                x: x.clone(),
                z: z.clone(),
            },
        },
        strength: change.strength,
    }
}

fn store_status(status: sink::RunStatus) -> RunStatus {
    match status {
        sink::RunStatus::Completed => RunStatus::Completed,
        sink::RunStatus::Stopped(reason) => RunStatus::Stopped(match reason {
            sink::StopReason::DiskFull => StopReason::DiskFull,
            sink::StopReason::MemoryFull => StopReason::MemoryFull,
            sink::StopReason::Cancelled => StopReason::Cancelled,
            sink::StopReason::Timeout => StopReason::Timeout,
            sink::StopReason::Failed(message) => StopReason::Failed(message),
        }),
    }
}
