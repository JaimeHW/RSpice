//! The runs' state: what watching the simulations in flight and checking a
//! run before it starts keep between frames.

use rspice_app_state::workbench::app_state::model::{
    Model, SchematicReviewMarkerVisibility, SlotFacts,
};

/// What the runs keep between frames.
#[derive(Debug, Clone, Default)]
pub struct RunsState {
    /// The runs seen running and the campaign outcomes not yet announced.
    ///
    /// The shell announces a completion on the *transition* into a terminal
    /// lifecycle rather than on the terminal state itself, so opening a
    /// project whose history already holds a finished run never claims a run
    /// just completed.
    pub run_watch: crate::simulation::controller::prepared_run::RunWatch,
    /// Mockup-specified simulation-preflight workflow. Validation reports are
    /// revision-bound runtime evidence and therefore never serialized.
    pub preflight: crate::workbench::app_state::run_preflight::PreflightDialogState,
    /// The deck a manual-deck run started from and the baseline the last one
    /// left. Runtime only, and boxed so a moved state stays the same size.
    pub manual_run: Box<crate::simulation::manual_run::ManualRunHandoff>,
    /// A manual deck's validation and a campaign's start that wait for a
    /// Verilog-A compile to end. Runtime only.
    pub waiting: crate::workbench::waiting::WaitingPreparations,
    /// The deck whose run is being prepared away from the frame, as the Run
    /// press took it, the press that waits there on the deck's files on
    /// disk, and the run that waits there for more memory than is free.
    /// Runtime only, and boxed so the state stays small to build and clone.
    pub preparing_deck: Option<Box<PreparingDeck>>,
}

/// A deck being prepared: what the press took, which names the deck in what
/// the verdict says, and finds the line a refusal names, and what the press
/// asked of it.
#[derive(Debug, Clone)]
pub struct PreparingDeck {
    pub name: String,
    pub text: rspice_app_state::state::RevisedText,
    /// What the run, prepared, needs beyond the memory free, when it needs
    /// more than is free and fits what is installed: it waits here, with
    /// Run anyway and Stop, and nothing of it dispatches until the reader
    /// runs it anyway. The status line and the band above the deck say it.
    pub memory: Option<rspice_simulation::execution::run_memory::MemoryWarning>,
    /// What the press waits for on the deck's files on disk before the
    /// deck is prepared, when it waits: nothing of it is prepared until
    /// then, and Stop drops it.
    pub disk: Option<DiskWait>,
    pub action: crate::simulation::controller::DeckAction,
}

/// What a press waits for on the files of the deck it took, on disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskWait {
    /// Their read, away from the frame: a file whose length or time moved
    /// since it was read is read to tell whether its text changed.
    Reading(rspice_app_state::state::DeckId),
    /// The reader's answer for a file changed on disk while its tab held
    /// edits: Reload or Keep mine above the deck.
    Answer(rspice_app_state::state::DeckId),
}

impl PreparingDeck {
    /// What Stop says of it.
    #[must_use]
    pub fn stopped(&self) -> String {
        match self.action {
            crate::simulation::controller::DeckAction::Run => {
                format!("Stopped {} before it ran.", self.name)
            }
            crate::simulation::controller::DeckAction::Check => {
                format!("Stopped checking {}.", self.name)
            }
        }
    }
}

impl RunsState {
    /// What model code reads of the features' state: whether the run gate's
    /// report is retained, and the review markers the drawing shows.
    pub fn slot_facts(&self, review_markers: SchematicReviewMarkerVisibility) -> SlotFacts {
        SlotFacts {
            preflight_report_retained: self.preflight.report.is_some(),
            review_markers,
        }
    }

    /// Stop the deck's run being prepared, or prepared and waiting for more
    /// memory than is free: nothing of it runs. The deck's name, for what
    /// Stop says, when one was being prepared.
    pub fn stop_preparing_deck(
        &mut self,
        controller: &mut crate::simulation::controller::SimulationController,
    ) -> Option<PreparingDeck> {
        let deck = self.preparing_deck.take()?;
        controller.cancel_deck_run_preparation();
        if deck.memory.is_some() {
            controller.clear_prepared_run();
        }
        Some(*deck)
    }

    /// What the deck's run, prepared and waiting for Run anyway, needs
    /// beyond the memory free.
    #[must_use]
    pub fn memory_warning(
        &self,
    ) -> Option<rspice_simulation::execution::run_memory::MemoryWarning> {
        self.preparing_deck.as_ref()?.memory
    }
}

/// Request a run of the Simulate workspace's run set.
pub fn request_run_set(model: &mut Model) {
    model.simulation.execution.request_simulate_run_set();
}
