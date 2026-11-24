//! `.ASSERT` over a stored transient's whole run, a chunk of rows at a time.
//!
//! The core's streamed checks walk every accepted row from the first in one pass over the store's
//! chunks, so a run larger than memory is checked without being read back whole. Assertions
//! observe the whole accepted trajectory, as the in-memory route checks it, not the output window.

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::{AssertionReport, TranAssertStream};
use rspice_core::engine::TransientResult;
use rspice_results::stored_transient::{SlotTarget, fill_event_traces, fill_rows};
use rspice_results_store::{SegmentIo, SegmentReader, StoreError};

use super::measure::stream_column;
use super::{StoredTransient, store_failure};
use crate::error::SimulationError;

impl StoredTransient {
    /// The deck's transient `.ASSERT` reports, checked from the store; `None` when a card needs
    /// the run in memory. The shell's time, saved columns and event histories hold whatever the
    /// pass left in them: the caller reads them back as it needs them.
    pub(in super::super) fn assert(
        &mut self,
        netlist: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
        translate: &dyn Fn(rspice_core::SimulationError) -> SimulationError,
    ) -> Result<Option<Vec<AssertionReport>>, SimulationError> {
        let Self {
            shell,
            run,
            targets,
            receipt,
            ..
        } = self;
        let checked = run
            .with_reader(|reader| check_run(reader, targets, shell, netlist, abort))
            .map_err(|error| store_failure(&receipt.path, error))?;
        checked.map_err(translate)
    }
}

fn check_run<I: SegmentIo>(
    reader: &SegmentReader<I>,
    targets: &[SlotTarget],
    shell: &mut TransientResult,
    netlist: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> Result<Result<Option<Vec<AssertionReport>>, rspice_core::SimulationError>, StoreError> {
    let aborted = || Ok(Err(rspice_core::SimulationError::Aborted));
    // A check on a digital or real event node reads its history as the in-memory run holds it:
    // the histories are the run's sparse record, read back whole, never its rows.
    fill_event_traces(reader, shell)?;
    let spans: Vec<(u64, usize)> = reader.chunk_spans().collect();
    let rows = reader.rows();
    // Planned against the run's first chunk of rows, its event operands over the run's span.
    let (Some(&(first, count)), Some(last)) = (spans.first(), rows.checked_sub(1)) else {
        return Ok(Ok(None));
    };
    let Some(stop) = reader.read_x(last, 1)?.first().copied() else {
        return Ok(Ok(None));
    };
    fill_rows(reader, targets, shell, first, count)?;
    let columns = targets
        .iter()
        .map(|&target| stream_column(shell, target))
        .collect::<Vec<_>>();
    let mut stream = match TranAssertStream::plan(netlist, shell, stop, &columns, abort) {
        Ok(Some(stream)) => stream,
        Ok(None) => return Ok(Ok(None)),
        Err(error) => return Ok(Err(error)),
    };
    let slots = stream.columns().to_vec();
    let mut x = Vec::new();
    let mut values = vec![Vec::new(); slots.len()];
    for index in 0..spans.len() {
        if abort.is_aborted() {
            return aborted();
        }
        reader.read_chunk_x(index, &mut x)?;
        for (&slot, values) in slots.iter().zip(&mut values) {
            reader.read_chunk_slot(index, slot, values)?;
        }
        let rows: Vec<&[f64]> = values.iter().map(Vec::as_slice).collect();
        stream.rows(&x, &rows);
    }
    Ok(Ok(stream.finish()))
}
