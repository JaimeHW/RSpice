//! The run tier's port, implemented by the whole application state.
//!
//! The run tier never names the application. Each operation here is what run
//! code did through the application state before, unchanged, so it happens
//! at the same point and in the same order.

use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::FftInputOptions;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

use super::design_checks::CheckScope;
use super::{AppState, Model, RunsState, SlotFacts};
use crate::simulation::{NetlistShown, RunHost, ViewerHost, ViewerReading};
use crate::state::{CellViewRef, SpecializedViewerCacheProvenance};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::state::Workspace;

impl AppState {
    /// [`Model::active_plan_revision`].
    pub(crate) fn active_plan_revision(
        &self,
    ) -> Option<(
        crate::product::SimulationPlanId,
        crate::product::ObjectRevision,
    )> {
        self.model.active_plan_revision()
    }

    /// [`Model::configured_topology_revision`].
    pub(crate) fn configured_topology_revision(&self) -> (String, u64, Vec<(String, u64)>) {
        self.model.configured_topology_revision()
    }
}

impl ViewerHost for AppState {
    fn model(&self) -> &Model {
        &self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        let analysis = &self.results.analysis;
        ViewerReading {
            eye_has_traces: analysis.eye_diagram_state.trace_count() > 0,
            eye_timebase: analysis.eye_diagram_state.timebase_provenance().cloned(),
            fft_has_data: analysis.fft_state.has_data(),
            fft_failed: analysis.fft_state.last_error.is_some(),
            fft_source: analysis.fft_state.selected_source.clone(),
        }
    }

    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions {
        self.results
            .analysis
            .fft_state
            .input_options_for_waveform(time)
    }

    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results.results.eye_timebase_for(owner)
    }
}

impl RunHost for AppState {
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
        self.code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation)
    }

    fn netlist_shown(&self) -> NetlistShown {
        let netlist = &self.code.netlist;
        NetlistShown {
            settled: netlist.active_document_initialized,
            comparison: netlist.active_document == ActiveNetlistDocument::GeneratedDiff,
            owned_source: netlist.active_document == ActiveNetlistDocument::OwnedSource,
        }
    }

    fn show_problem_findings(&mut self) {
        AppState::show_problem_findings(self);
    }

    fn queue_run_set(&mut self) {
        self.request_run_set_simulation();
        self.workbench.activate(Workspace::Simulate);
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
