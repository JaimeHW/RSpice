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
    /// press took it. Runtime only.
    pub preparing_deck: Option<PreparingDeck>,
}

/// A deck whose run is being prepared: what the press took, which names the
/// deck in what the run's verdict says, and finds the line a refusal names.
#[derive(Debug, Clone)]
pub struct PreparingDeck {
    pub name: String,
    pub text: rspice_app_state::state::RevisedText,
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

    /// Stop the deck's run being prepared: nothing of it runs. The deck's
    /// name, for what Stop says, when one was being prepared.
    pub fn stop_preparing_deck(
        &mut self,
        controller: &mut crate::simulation::controller::SimulationController,
    ) -> Option<String> {
        let deck = self.preparing_deck.take()?;
        controller.cancel_deck_run_preparation();
        Some(deck.name)
    }
}

/// Request a run of the Simulate workspace's run set.
pub fn request_run_set(model: &mut Model) {
    model.simulation.execution.request_simulate_run_set();
}
