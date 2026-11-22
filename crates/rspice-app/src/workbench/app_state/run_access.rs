//! The run tier's port, implemented by the whole application state.
//!
//! The run tier never names the application. Each operation here is what run
//! code did through the application state before, unchanged, so it happens
//! at the same point and in the same order.

use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::FftInputOptions;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

use super::{AppState, RunsState};
use rspice_app_runs::simulation::{ManualDeck, RunHost, ViewerHost, ViewerReading};
use rspice_app_state::state::{CellViewRef, SpecializedViewerCacheProvenance};
use rspice_app_state::workbench::app_state::design_checks::CheckScope;
use rspice_app_state::workbench::app_state::model::{Model, SlotFacts};

impl ViewerHost for AppState {
    fn model(&self) -> &Model {
        &self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        self.results_reads().viewer_reading()
    }

    fn fft_input_options(&self) -> FftInputOptions {
        self.results_reads().fft_input_options()
    }

    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results_reads().eye_timebase_for(owner)
    }

    fn eye_fold_for(
        &self,
        owner: SpecializedViewerCacheProvenance,
    ) -> rspice_results_ui::eye_diagram::EyeFold {
        self.results_reads().eye_fold_for(owner)
    }
}

impl RunHost for AppState {
    type Frozen = AppState;

    fn resource_refusal(
        &self,
        refusal: &rspice_core::ResourceLimitError,
        device: Option<&str>,
    ) -> String {
        let messages = self.workbench.messages();
        match device {
            Some(device) => messages.device_resource_refusal(device, refusal),
            None => messages.resource_refusal(refusal),
        }
    }

    /// The whole state, as a campaign has always frozen it.
    fn freeze(&self) -> AppState {
        self.clone()
    }

    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    fn runs(&self) -> &RunsState {
        &self.runs
    }

    fn runs_mut(&mut self) -> &mut RunsState {
        &mut self.runs
    }

    fn prompt(&mut self, text: impl Into<String>) {
        AppState::prompt(self, text);
    }

    fn slot_facts(&self) -> SlotFacts {
        AppState::slot_facts(self)
    }

    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        self.code.veriloga_compilation()
    }

    fn manual_deck(&self) -> Option<ManualDeck<'_>> {
        self.code.manual_deck()
    }

    fn show_problem_findings(&mut self) {
        AppState::show_problem_findings(self);
    }

    fn show_held_run(&mut self) {
        self.workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Simulate);
    }

    fn queue_run_set(&mut self) {
        self.request_run_set_simulation();
    }

    fn sync_active_schematic_to_workspace(&mut self) {
        if let Some(prompt) = self.model.sync_active_schematic_to_workspace() {
            self.prompt(prompt);
        }
    }

    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String> {
        AppState::ensure_design_check(self, subject, CheckScope::Design)
    }
}
