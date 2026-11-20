//! The selected trace's statistics, computed once per trace and retained data version.
//!
//! The inspector states them on every frame a trace is selected. An in-memory trace is measured
//! when they are first asked for; a trace whose samples a results store keeps is read and
//! measured on a background thread, so no frame reads its columns.

use std::sync::{Arc, OnceLock};

use rspice_results::measurements::measure_interval_values;

use super::{AnalysisPresentationKey, ResultsReads, frame_work};
use crate::state::RetainedTrace;

/// What the inspector states about one trace.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TraceStatistics {
    /// Samples the trace holds.
    pub(crate) samples: usize,
    /// Samples with a finite value.
    pub(crate) finite: usize,
    /// The least and greatest finite value.
    pub(crate) extremes: Option<(f64, f64)>,
    /// The average over X, as the calculator's `avg()` takes it, or why there is none.
    pub(crate) average: Result<f64, String>,
}

/// Statistics a background thread may still be computing.
pub(super) type PendingStatistics = Arc<OnceLock<TraceStatistics>>;

/// The statistics of waveform `waveform_index` of `analysis`, once they are known. The first
/// ask starts the work; a background thread that finishes it asks `repaint` to draw again.
pub(crate) fn trace_statistics(
    reads: ResultsReads<'_>,
    analysis: AnalysisPresentationKey,
    waveform_index: usize,
    waveform: &RetainedTrace,
    repaint: &egui::Context,
) -> Option<TraceStatistics> {
    reads
        .results
        .trace_statistics
        .get_or_insert_with(&reads.model.simulation, (analysis, waveform_index), || {
            start(waveform, repaint)
        })
        .get()
        .cloned()
}

fn start(waveform: &RetainedTrace, repaint: &egui::Context) -> PendingStatistics {
    frame_work::note(frame_work::DatasetWalk::TraceStatistics);
    let pending = PendingStatistics::default();
    if !waveform.x.is_stored() && !waveform.y.is_stored() {
        let _ = pending.set(measure(&waveform.x, &waveform.y));
        return pending;
    }
    let (x, y) = (waveform.x.clone(), waveform.y.clone());
    let (slot, repaint) = (Arc::clone(&pending), repaint.clone());
    rspice_app_services::workbench::platform::spawn_or_inline(move || {
        let _ = slot.set(measure(&x, &y));
        repaint.request_repaint();
    });
    pending
}

/// The statistics of the samples `y` at `x`, read a block at a time: a stored trace is never
/// held whole.
pub(crate) fn measure(
    x: &rspice_results::waveform::WaveformValues,
    y: &rspice_results::waveform::WaveformValues,
) -> TraceStatistics {
    let samples = x.len().min(y.len());
    let mut finite = 0;
    let mut extremes: Option<(f64, f64)> = None;
    for value in y.samples(..samples).filter(|value| value.is_finite()) {
        finite += 1;
        extremes = Some(extremes.map_or((value, value), |(low, high)| {
            (low.min(value), high.max(value))
        }));
    }
    TraceStatistics {
        samples,
        finite,
        extremes,
        average: measure_interval_values(x, y, 0..samples, None)
            .map(|statistics| statistics.mean)
            .map_err(|error| error.to_string()),
    }
}
