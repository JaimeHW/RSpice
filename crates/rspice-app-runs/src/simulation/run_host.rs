//! What the run tier reaches of the application that hosts it.
//!
//! The run tier prepares, starts, watches and collects simulations. It works
//! on the shared model and keeps its own slot; everything else it touches
//! belongs to a feature above it, which it never names. It reaches them
//! through this port instead: what it reads of them, and what it tells them.
//! The host applies each operation before returning, so code that reads after
//! reporting sees the effect, in the order the run tier asked.

use rspice_results_ui::eye_diagram::{EyeTimebase, EyeTimebaseProvenance};
use rspice_results_ui::fft::FftInputOptions;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

use crate::state::{CellViewRef, RunEventSink, SpecializedViewerCacheProvenance};
use crate::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::{Model, SlotFacts};

/// What the transient viewers' derived data is made from and reported to:
/// the shared model, what the viewers hold and how the reader set them up,
/// and the viewers themselves, which take each report.
pub trait ViewerHost: RunEventSink {
    /// The shared model.
    fn model(&self) -> &Model;
    /// What the transient viewers hold now.
    fn viewer_reading(&self) -> ViewerReading;
    /// How the FFT prepares the input it is given over `time`, as the reader
    /// set it.
    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions;
    /// The bit period the reader chose for this result's eye.
    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase;
}

/// The application, as the run tier sees it.
pub trait RunHost: ViewerHost {
    /// The host [`Self::freeze`] returns, which owns everything it reads.
    type Frozen: RunHost;

    /// A copy of what a campaign reads, taken now. A campaign prepares every
    /// member from this copy before any engine work begins, so a later
    /// member never observes edits made while an earlier one runs.
    fn freeze(&self) -> Self::Frozen;
    /// The shared model, to change.
    fn model_mut(&mut self) -> &mut Model;
    /// The runs' own slot.
    fn runs(&self) -> &RunsState;
    /// The runs' own slot, to change.
    fn runs_mut(&mut self) -> &mut RunsState;

    /// Answer the gesture just made, on the status bar.
    fn prompt(&mut self, text: impl Into<String>);
    /// What model operations read of the features' state, as it stands now.
    fn slot_facts(&self) -> SlotFacts;
    /// The receipt of the project's Verilog-A compilation, when one is current.
    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt>;
    /// Which netlist document the editor shows.
    fn netlist_shown(&self) -> NetlistShown;
    /// A run gate filed findings that stop the run: bring the reader to them.
    fn show_problem_findings(&mut self);
    /// The run gate authorized the retained run set: queue it, and show the
    /// workspace that runs it.
    fn queue_run_set(&mut self);
    /// The run gate reads the design as the open schematic edits it.
    fn sync_active_schematic_to_workspace(&mut self);
    /// Check `subject` as Check Design does, unless a current check of it
    /// already describes it.
    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String>;
}

/// Which netlist document the editor shows, as a manual-deck run reads it.
#[derive(Debug, Clone, Copy)]
pub struct NetlistShown {
    /// Whether the editor has settled which document a just-opened project
    /// shows.
    pub settled: bool,
    /// The generated revision comparison, which is not a deck.
    pub comparison: bool,
    /// The project's own source.
    pub owned_source: bool,
}

/// What the transient viewers hold, read when the run tier asks.
#[derive(Debug, Clone)]
pub struct ViewerReading {
    /// Whether the eye holds folded traces.
    pub eye_has_traces: bool,
    /// The bit period the loaded eye was folded at, when it records one.
    pub eye_timebase: Option<EyeTimebaseProvenance>,
    /// Whether the FFT holds a spectrum.
    pub fft_has_data: bool,
    /// Whether the FFT holds an error.
    pub fft_failed: bool,
    /// The trace the reader chose for the FFT.
    pub fft_source: Option<String>,
}
