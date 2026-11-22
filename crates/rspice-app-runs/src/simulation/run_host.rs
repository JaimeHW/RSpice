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
pub use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

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
    /// How the FFT prepares the input it is given, as the reader
    /// set it.
    fn fft_input_options(&self) -> FftInputOptions;
    /// The bit period the reader chose for this result's eye.
    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase;
    /// What folds this result's eye: its signal, its clock and the clock's
    /// period. A host that keeps only a period folds the spectrum's signal
    /// on an ideal clock at it.
    fn eye_fold_for(
        &self,
        owner: SpecializedViewerCacheProvenance,
    ) -> rspice_results_ui::eye_diagram::EyeFold {
        rspice_results_ui::eye_diagram::EyeFold {
            timebase: self.eye_timebase_for(owner),
            ..rspice_results_ui::eye_diagram::EyeFold::default()
        }
    }
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
    /// The deck in front of the netlist editor, which a manual run reads.
    fn manual_deck(&self) -> Option<ManualDeck<'_>>;
    /// A run gate filed findings that stop the run: bring the reader to them.
    fn show_problem_findings(&mut self);
    /// The run the gate authorized waits for the reader to run it anyway:
    /// bring the reader to where it asks.
    fn show_held_run(&mut self);
    /// The run gate authorized the retained run set: queue it. The reader
    /// stays on the page they are on.
    fn queue_run_set(&mut self);
    /// The run gate reads the design as the open schematic edits it.
    fn sync_active_schematic_to_workspace(&mut self);
    /// Check `subject` as Check Design does, unless a current check of it
    /// already describes it.
    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String>;
    /// A run's refusal for going past one of the engine's limits, in the
    /// reader's words; `device` names the device that went past it, where
    /// one did.
    fn resource_refusal(
        &self,
        refusal: &rspice_core::ResourceLimitError,
        device: Option<&str>,
    ) -> String;
}

/// A deck a manual run reads: its name, its text as the editor holds it,
/// and where it is read from. The text is read in one piece only by what
/// reads the deck, a run or its check, not by naming it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManualDeck<'a> {
    pub name: &'a str,
    pub text: &'a crate::state::RevisedText,
    pub origin: DeckOrigin<'a>,
}

/// Where a deck is read from, which its relative includes resolve against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckOrigin<'a> {
    /// A file on disk.
    File(&'a std::path::Path),
    /// Text the project keeps, with the files its include lines read.
    Stored {
        files: &'a [rspice_design::deck::StoredDeckFile],
    },
}

/// A [`ManualDeck`] that owns what it names, for a host that keeps a copy;
/// the text is shared with the deck it copies, not copied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualDeckCopy {
    pub name: String,
    pub text: crate::state::RevisedText,
    pub origin: DeckOriginCopy,
}

/// A [`DeckOrigin`] that owns what it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckOriginCopy {
    File(std::path::PathBuf),
    Stored(Vec<rspice_design::deck::StoredDeckFile>),
}

impl ManualDeckCopy {
    /// The deck `text` read from the file at `path`, named by its file.
    #[must_use]
    pub fn new(path: std::path::PathBuf, text: String) -> Self {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        Self {
            name,
            text: text.into(),
            origin: DeckOriginCopy::File(path),
        }
    }

    /// A copy of `deck`.
    #[must_use]
    pub fn of(deck: ManualDeck<'_>) -> Self {
        Self {
            name: deck.name.to_owned(),
            text: deck.text.clone(),
            origin: match deck.origin {
                DeckOrigin::File(path) => DeckOriginCopy::File(path.to_path_buf()),
                DeckOrigin::Stored { files } => DeckOriginCopy::Stored(files.to_vec()),
            },
        }
    }

    /// The deck this copy holds.
    #[must_use]
    pub fn deck(&self) -> ManualDeck<'_> {
        ManualDeck {
            name: &self.name,
            text: &self.text,
            origin: match &self.origin {
                DeckOriginCopy::File(path) => DeckOrigin::File(path),
                DeckOriginCopy::Stored(files) => DeckOrigin::Stored { files },
            },
        }
    }

    /// Where the run's findings say their lines are: the deck's file, or the
    /// name of a deck the project keeps as text.
    #[must_use]
    pub fn findings_path(self) -> std::path::PathBuf {
        match self.origin {
            DeckOriginCopy::File(path) => path,
            DeckOriginCopy::Stored(_) => std::path::PathBuf::from(self.name),
        }
    }
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
