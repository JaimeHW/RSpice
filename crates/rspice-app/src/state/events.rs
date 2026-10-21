//! What the run tier reports about the results it derives.
//!
//! The run tier says what happened to a result it produced; the workspace
//! that owns the viewers decides what that does to them. A sink applies each
//! event before the report returns, so code that reads a viewer after
//! reporting to it sees the event's effect.

use rspice_results_ui::bode::BodeData;
use rspice_results_ui::eye_diagram::{EyeData, EyeTimebaseProvenance};
use rspice_results_ui::fft::{FftInputError, PreparedFftInput};
use rspice_results_ui::nyquist::NyquistData;

use super::{ActiveViewer, SpecializedViewerCacheProvenance};

/// One thing the run tier reports to the viewers that show its results.
#[derive(Debug)]
pub(crate) enum RunEvent {
    /// A transient result replaced the one the FFT and eye viewers were
    /// derived from, or left them nothing to derive from.
    TransientResultReplaced,
    /// The trace the FFT transforms was chosen.
    FftSourceChosen { source: String },
    /// A transient waveform was folded into an eye. `eye` is `None` when
    /// nothing could be folded; `timebase` still says what was tried.
    EyeFolded {
        owner: Option<SpecializedViewerCacheProvenance>,
        eye: Option<EyeData>,
        timebase: EyeTimebaseProvenance,
    },
    /// The reader's bit period no longer matches the eye that is loaded.
    EyeTimebaseChanged,
    /// The eye the run tier loaded belongs to a result that is no longer
    /// active.
    EyeOutdated,
    /// A transient waveform was prepared for the FFT, or failed to be.
    FftInputPrepared {
        owner: Option<SpecializedViewerCacheProvenance>,
        input: Result<PreparedFftInput, FftInputError>,
    },
    /// The FFT worker ended without answering.
    FftWorkerLost,
    /// The FFT the run tier loaded belongs to a result that is no longer
    /// active.
    FftOutdated,
    /// The FFT the run tier loaded is no longer in the viewer: an in-place
    /// recompute or an explicit clear emptied it.
    FftEmptied,
    /// The data a viewer already holds was derived from `owner`.
    ViewerCacheConfirmed {
        viewer: ActiveViewer,
        owner: SpecializedViewerCacheProvenance,
    },
    /// A frequency-domain result replaced what the Bode, Nyquist and Smith
    /// viewers hold. `loop_gain` is the stability run's loop-gain locus, the
    /// only response the Nyquist viewer is given.
    FrequencyResponseReplaced {
        owner: Option<SpecializedViewerCacheProvenance>,
        bode: BodeData,
        loop_gain: Option<NyquistData>,
    },
    /// The run tier finished collecting this frame's completions, which may
    /// have changed the active result.
    CompletionsCollected,
}

/// Where the run tier reports. The implementor applies each event before
/// returning.
pub(crate) trait RunEventSink {
    fn apply_run_event(&mut self, event: RunEvent);
}
