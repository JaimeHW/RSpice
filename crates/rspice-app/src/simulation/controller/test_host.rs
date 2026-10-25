//! What the run tier's own tests run against: the shared model and the runs'
//! slot.
//!
//! The fields carry the names `AppState` gives the same values, so a test
//! body reads the same against either and only its constructor differs. What
//! the run tier asks of a feature above it is answered as a new session
//! answers it, and what it tells one is dropped, never applied: a test that
//! needs a feature to have acted on a report runs against the whole
//! application in the shell's feature tests.

use rspice_app_state::workbench::workspace::restore_model_library_manager;
use rspice_design::drc::DrcResult;
use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::{FftInputOptions, FftState};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

use crate::simulation::{NetlistShown, RunHost, ViewerReading};
use crate::state::pdk_config::PdkConfig;
use crate::state::{CellViewRef, RunEvent, RunEventSink, SpecializedViewerCacheProvenance};
use crate::workbench::{
    app_state::{Model, SlotFacts, design_checks::CheckScope},
    slots::runs::RunsState,
};

/// The shared model and the runs' slot, under the names `AppState` gives
/// them.
#[derive(Clone)]
pub(crate) struct RunTestHost {
    pub(crate) model: Model,
    pub(crate) runs: Box<RunsState>,
}

impl Default for RunTestHost {
    /// The model and runs' slot a new session starts with, over the empty PDK
    /// configuration a test session reads.
    fn default() -> Self {
        Self {
            model: Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
            runs: Box::default(),
        }
    }
}

impl RunTestHost {
    /// [`Model::sync_active_schematic_to_workspace`]. What it has to say goes
    /// to the status bar, which is the workbench's.
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        let _prompt = self.model.sync_active_schematic_to_workspace();
    }

    /// Check the open cellview and every cellview its walk reaches, as the
    /// application's Check Hierarchy does.
    pub(crate) fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        let subject = self.model.workspace.content.active_schematic_reference();
        self.run_design_check(subject, CheckScope::Hierarchy)
    }

    /// [`Model::provision_test_project_technology_contract`].
    pub(crate) fn provision_test_project_technology_contract(&mut self) {
        self.model.provision_test_project_technology_contract();
    }

    /// [`Model::provision_test_project_veriloga_technology_contract`].
    pub(crate) fn provision_test_project_veriloga_technology_contract(&mut self) {
        self.model
            .provision_test_project_veriloga_technology_contract();
    }

    /// Whether the project's technology governs its runs, as the model's
    /// technology inputs answer it.
    pub(crate) fn project_technology_in_effect(&self) -> bool {
        self.model
            .technology_inputs()
            .project_technology_in_effect()
    }

    /// The application's half of a design check: the drawing's annotation is
    /// brought into the workspace first where restoring it failed.
    fn run_design_check(
        &mut self,
        subject: CellViewRef,
        scope: CheckScope,
    ) -> Result<DrcResult, String> {
        if self
            .model
            .workspace
            .annotation_restoration_error()
            .is_some()
        {
            self.sync_active_schematic_to_workspace();
        }
        let facts = self.slot_facts();
        self.model.run_design_check(subject, scope, facts)
    }
}

impl RunEventSink for RunTestHost {
    /// The results viewer is a feature's: a test of what it makes of a
    /// report runs against the whole application.
    fn apply_run_event(&mut self, _event: RunEvent) {}
}

impl RunHost for RunTestHost {
    fn model(&self) -> &Model {
        &self.model
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

    /// The status bar is the workbench's.
    fn prompt(&mut self, _text: impl Into<String>) {}

    /// The drawing shows the review markers a new session shows.
    fn slot_facts(&self) -> SlotFacts {
        SlotFacts {
            preflight_report_retained: self.runs.preflight.report.is_some(),
            review_markers: SchematicReviewMarkerVisibility::default(),
        }
    }

    /// No project compilation is current.
    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        None
    }

    /// The editor shows the generated netlist and has settled nothing yet.
    fn netlist_shown(&self) -> NetlistShown {
        NetlistShown {
            settled: false,
            comparison: false,
            owned_source: false,
        }
    }

    /// The viewers hold nothing.
    fn viewer_reading(&self) -> ViewerReading {
        ViewerReading {
            eye_has_traces: false,
            eye_timebase: None,
            fft_has_data: false,
            fft_failed: false,
            fft_source: None,
        }
    }

    /// The FFT prepares its input as a new session's does.
    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions {
        FftState::default().input_options_for_waveform(time)
    }

    /// The reader stated no bit period.
    fn eye_timebase_for(&self, _owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        EyeTimebase::default()
    }

    /// Bringing the reader to the findings is the workbench's.
    fn show_problem_findings(&mut self) {}

    /// The run set is queued on the model; showing the workspace that runs
    /// it is the workbench's.
    fn queue_run_set(&mut self) {
        self.model.simulation.execution.request_simulate_run_set();
    }

    fn sync_active_schematic_to_workspace(&mut self) {
        RunTestHost::sync_active_schematic_to_workspace(self);
    }

    /// As the application ensures it: a current check of the whole design
    /// stands, and anything else is checked again.
    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String> {
        if self
            .model
            .current_design_check(&subject, CheckScope::Design, self.slot_facts())
            .is_some()
        {
            return Ok(());
        }
        self.run_design_check(subject, CheckScope::Design)
            .map(|_| ())
    }
}
