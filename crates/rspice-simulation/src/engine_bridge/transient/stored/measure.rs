//! `.MEASURE TRAN` over a stored transient's output window, a chunk of rows at a time.
//!
//! The core's streamed evaluator reads the window in passes over the store's chunks, as the
//! command line measures a stored run, so a run larger than memory is measured without being read
//! back whole. The window is the one the in-memory route measures: from the output start, its
//! first row interpolated there when the start falls between two stored rows.

use std::borrow::Cow;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::{MeasureResult, TranMeasureStream};
use rspice_core::engine::TransientResult;
use rspice_results::stored_transient::{SlotTarget, fill_rows};
use rspice_results_store::{SegmentIo, SegmentReader, StoreError};

use super::{OutputWindow, StoredTransient, store_failure};
use crate::error::SimulationError;

impl StoredTransient {
    /// The deck's `.MEASURE TRAN` results over `window`, measured from the store; `None` when a
    /// statement needs the run in memory. The shell's time and saved columns hold whatever the
    /// passes left in them: the caller reads them back as it needs them.
    pub(in super::super) fn measure(
        &mut self,
        netlist: &rspice_core::Netlist,
        window: &OutputWindow,
        abort: &dyn AbortSignal,
        translate: &dyn Fn(rspice_core::SimulationError) -> SimulationError,
    ) -> Result<Option<Vec<MeasureResult>>, SimulationError> {
        let Self {
            shell,
            run,
            targets,
            receipt,
            ..
        } = self;
        let measured = run
            .with_reader(|reader| measure_window(reader, targets, shell, netlist, window, abort))
            .map_err(|error| store_failure(&receipt.path, error))?;
        measured.map_err(translate)
    }
}

/// The output window's first row when it falls between two stored rows: each saved column
/// interpolated at the window's start, as the in-memory window takes it.
struct LeadRow {
    time: f64,
    /// By slot; `None` for a slot the result keeps no column of.
    values: Vec<Option<f64>>,
}

impl LeadRow {
    fn read<I: SegmentIo>(
        reader: &mut SegmentReader<I>,
        targets: &[SlotTarget],
        shell: &mut TransientResult,
        window: &OutputWindow,
    ) -> Result<Option<Self>, StoreError> {
        let Some((lower, fraction)) = window.lead else {
            return Ok(None);
        };
        fill_rows(reader, targets, shell, lower as u64, 2)?;
        let values = targets
            .iter()
            .map(|&target| {
                target
                    .column(shell)
                    .map(|column| column[0] + fraction * (column[1] - column[0]))
            })
            .collect();
        Ok(Some(Self {
            time: window.start_time,
            values,
        }))
    }
}

/// The rows of one chunk the window holds, as offsets into the chunk, and whether the window's
/// lead row comes before them.
#[derive(Clone, Copy)]
struct ChunkWindow {
    first_row: u64,
    start: usize,
    end: usize,
    lead: bool,
}

impl ChunkWindow {
    fn of(first: u64, count: usize, window: &OutputWindow, lead: bool) -> Option<Self> {
        let (first, count) = (usize::try_from(first).ok()?, count);
        let window_end = window.start_idx + window.sample_count;
        let start = window.start_idx.max(first);
        let end = window_end.min(first + count);
        (start < end).then_some(Self {
            first_row: start as u64,
            start: start - first,
            end: end - first,
            lead: lead && (first..first + count).contains(&window.start_idx),
        })
    }

    /// `chunk`'s rows in the window, after `lead` when the window's lead row comes first.
    fn rows<'a>(self, chunk: &'a [f64], lead: impl FnOnce() -> f64) -> Cow<'a, [f64]> {
        let rows = &chunk[self.start..self.end];
        if self.lead {
            let mut held = Vec::with_capacity(rows.len() + 1);
            held.push(lead());
            held.extend_from_slice(rows);
            Cow::Owned(held)
        } else {
            Cow::Borrowed(rows)
        }
    }
}

/// The window's rows `chunk` holds into `shell`'s time and saved columns, after its lead row
/// when that comes first.
fn fill_chunk<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    targets: &[SlotTarget],
    shell: &mut TransientResult,
    chunk: ChunkWindow,
    lead: Option<&LeadRow>,
) -> Result<(), StoreError> {
    fill_rows(
        reader,
        targets,
        shell,
        chunk.first_row,
        chunk.end - chunk.start,
    )?;
    if let Some(lead) = lead.filter(|_| chunk.lead) {
        shell.time.insert(0, lead.time);
        for (&target, value) in targets.iter().zip(&lead.values) {
            if let (Some(column), Some(value)) = (target.column(shell), value) {
                column.insert(0, *value);
            }
        }
    }
    Ok(())
}

/// The column of `shell` a stored slot fills, which the evaluator reads from the rows; `None` for
/// one it derives.
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

fn measure_window<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    targets: &[SlotTarget],
    shell: &mut TransientResult,
    netlist: &rspice_core::Netlist,
    window: &OutputWindow,
    abort: &dyn AbortSignal,
) -> Result<Result<Option<Vec<MeasureResult>>, rspice_core::SimulationError>, StoreError> {
    let aborted = || Ok(Err(rspice_core::SimulationError::Aborted));
    let lead = LeadRow::read(reader, targets, shell, window)?;
    let spans: Vec<(u64, usize)> = reader.chunk_spans().collect();
    let chunks: Vec<Option<ChunkWindow>> = spans
        .iter()
        .map(|&(first, count)| ChunkWindow::of(first, count, window, lead.is_some()))
        .collect();
    // Planned against the window's first chunk of rows.
    let Some(&probe) = chunks.iter().flatten().next() else {
        return Ok(Ok(None));
    };
    fill_chunk(reader, targets, shell, probe, lead.as_ref())?;
    let columns = targets
        .iter()
        .map(|&target| stream_column(shell, target))
        .collect::<Vec<_>>();
    let mut stream = match TranMeasureStream::plan(netlist, shell, &columns, abort) {
        Ok(Some(stream)) => stream,
        Ok(None) => return Ok(Ok(None)),
        Err(error) => return Ok(Err(error)),
    };
    let lead_time = || lead.as_ref().map_or(window.start_time, |lead| lead.time);
    let lead_value = |slot: usize| {
        lead.as_ref()
            .and_then(|lead| lead.values[slot])
            .unwrap_or(f64::NAN)
    };
    let mut x = Vec::new();
    for (index, chunk) in chunks.iter().enumerate() {
        let Some(chunk) = *chunk else { continue };
        if abort.is_aborted() {
            return aborted();
        }
        reader.read_chunk_x(index, &mut x)?;
        stream.axis(&chunk.rows(&x, lead_time));
    }
    let slots = stream.columns().to_vec();
    let mut values = vec![Vec::new(); slots.len()];
    loop {
        for (index, chunk) in chunks.iter().enumerate() {
            let Some(chunk) = *chunk else { continue };
            if abort.is_aborted() {
                return aborted();
            }
            reader.read_chunk_x(index, &mut x)?;
            for (&slot, values) in slots.iter().zip(&mut values) {
                reader.read_chunk_slot(index, slot, values)?;
            }
            // An expression, a differential voltage or an alias is derived from the chunk's
            // rows of every column.
            if stream.derives() {
                fill_chunk(reader, targets, shell, chunk, lead.as_ref())?;
                match stream.derive(netlist, shell, abort) {
                    Ok(true) => {}
                    Ok(false) => return Ok(Ok(None)),
                    Err(error) => return Ok(Err(error)),
                }
            }
            let rows: Vec<Cow<'_, [f64]>> = slots
                .iter()
                .zip(&values)
                .map(|(&slot, values)| chunk.rows(values, || lead_value(slot)))
                .collect();
            let rows: Vec<&[f64]> = rows.iter().map(AsRef::as_ref).collect();
            stream.rows(&chunk.rows(&x, lead_time), &rows);
        }
        if !stream.next_pass() {
            break;
        }
    }
    // The window's first and last rows, for the statements read on its span.
    let last = (window.start_idx + window.sample_count - 1) as u64;
    fill_chunk(
        reader,
        targets,
        shell,
        ChunkWindow {
            first_row: window.start_idx as u64,
            start: 0,
            end: 1,
            lead: lead.is_some(),
        },
        lead.as_ref(),
    )?;
    if lead.is_some() {
        shell.time.truncate(1);
        for &target in targets {
            if let Some(column) = target.column(shell) {
                column.truncate(1);
            }
        }
    }
    if window.sample_count > 1 || lead.is_some() {
        shell.time.extend(reader.read_x(last, 1)?);
        for (slot, &target) in targets.iter().enumerate() {
            if target.column(shell).is_none() {
                continue;
            }
            let value = reader.read_slot(slot, last, 1)?;
            if let Some(column) = target.column(shell) {
                column.extend(value);
            }
        }
    }
    Ok(stream.finish(netlist, shell, abort))
}
