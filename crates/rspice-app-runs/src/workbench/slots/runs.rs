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
    /// press took it, and the run that waits there for more memory than is
    /// free. Runtime only.
    pub preparing_deck: Option<PreparingDeck>,
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
    pub action: crate::simulation::controller::DeckAction,
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
        Some(deck)
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
