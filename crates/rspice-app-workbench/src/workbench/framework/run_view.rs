//! The run tier's host over a feature command's context.
//!
//! A run started from a feature's command reads and reports through the
//! context the command runs in: the model, the runs and the session
//! directly, what design and the results workspace show through their
//! peers, and what code shows as the command's feature lends it. Each
//! operation does what the application state's own host does, at the same
//! point and in the same order.

use rspice_app_runs::simulation::{
    NetlistShown, ProjectCompileReceipt, RunHost, ViewerHost, ViewerReading,
};
use rspice_app_runs::workbench::slots::runs::{RunsState, request_run_set};
use rspice_app_state::workbench::app_state::design_checks::CheckScope;
use rspice_app_state::workbench::app_state::model::SlotFacts;
use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::FftInputOptions;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::command_registry::Registry;
use super::feature::Peers;
use super::ports::ResultsPeer;
use crate::state::{CellViewRef, RunEvent, RunEventSink, SpecializedViewerCacheProvenance};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::workbench_session::WorkbenchSession;

/// What code shows that a run reads: the receipt of the project's current
/// Verilog-A compilation and which netlist document the editor shows. Read
/// once, when the host is built; while the host lives, code's state is lent
/// only shared, so what was read is what code shows throughout.
#[derive(Clone, Copy)]
pub struct RunCodeFacts<'v> {
    pub veriloga_compilation: Option<&'v ProjectCompileReceipt>,
    pub netlist_shown: NetlistShown,
}

/// The run tier's host over a feature command's context.
pub struct RunView<'v, 'a> {
    model: &'v mut Model,
    runs: &'v mut RunsState,
    wb: &'v mut WorkbenchSession,
    platform: &'v PlatformState,
    peers: &'v mut Peers<'a>,
    code: RunCodeFacts<'v>,
    commands: &'static Registry,
}

impl<'v, 'a> RunView<'v, 'a> {
    /// The run tier's host over a command's parts: the model, the runs,
    /// the session and the platform state it runs over, the peers it
    /// reads design and the results workspace through, and what code
    /// shows.
    pub fn new(
        model: &'v mut Model,
        runs: &'v mut RunsState,
        wb: &'v mut WorkbenchSession,
        platform: &'v PlatformState,
        peers: &'v mut Peers<'a>,
        code: RunCodeFacts<'v>,
        commands: &'static Registry,
    ) -> Self {
        Self {
            model,
            runs,
            wb,
            platform,
            peers,
            code,
            commands,
        }
    }
}

/// The viewers are the results workspace's, and take each report there.
impl RunEventSink for RunView<'_, '_> {
    fn apply_run_event(&mut self, event: RunEvent) {
        self.peers.run_events().apply_run_event(
            event,
            self.model,
            self.wb,
            self.platform,
            &mut self.runs.preflight,
            self.commands,
        );
    }
}

impl ViewerHost for RunView<'_, '_> {
    fn model(&self) -> &Model {
        self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        self.peers.results().viewer_reading()
    }

    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions {
        self.peers.results().fft_input_options(time)
    }

    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.peers.results().eye_timebase_for(owner)
    }
}

impl RunHost for RunView<'_, '_> {
    type Frozen = FrozenRun;

    fn freeze(&self) -> FrozenRun {
        FrozenRun {
            model: self.model.clone(),
            runs: self.runs.clone(),
            wb: self.wb.clone(),
            platform: self.platform.clone(),
            review_markers: self.peers.design().review_markers(),
            veriloga_compilation: self.code.veriloga_compilation.cloned(),
            netlist_shown: self.code.netlist_shown,
            results: self.peers.results().frozen(),
            commands: self.commands,
        }
    }

    fn model_mut(&mut self) -> &mut Model {
        self.model
    }

    fn runs(&self) -> &RunsState {
        self.runs
    }

    fn runs_mut(&mut self) -> &mut RunsState {
        self.runs
    }

    fn prompt(&mut self, text: impl Into<String>) {
        self.wb.prompt(text);
    }

    fn slot_facts(&self) -> SlotFacts {
        self.runs.slot_facts(self.peers.design().review_markers())
    }

    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        self.code.veriloga_compilation
    }

    fn netlist_shown(&self) -> NetlistShown {
        self.code.netlist_shown
    }

    fn show_problem_findings(&mut self) {
        self.wb.show_problem_findings();
    }

    fn queue_run_set(&mut self) {
        request_run_set(self.model, self.runs);
    }

    fn sync_active_schematic_to_workspace(&mut self) {
        if let Some(prompt) = self.model.sync_active_schematic_to_workspace() {
            self.wb.prompt(prompt);
        }
    }

    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String> {
        let facts = self.slot_facts();
        let (outcome, prompt) = self
            .model
            .ensure_design_check(subject, CheckScope::Design, facts);
        if let Some(prompt) = prompt {
            self.wb.prompt(prompt);
        }
        outcome
    }
}

/// What a campaign started from a simulate command reads, frozen when it
/// starts: its own copy of the model, of what code and design stated then,
/// and of the results workspace. What the run tier tells it reaches only
/// those copies; what it would tell the reader, nobody sees.
pub struct FrozenRun {
    model: Model,
    // RunHost needs the runs; a campaign never reads them.
    runs: RunsState,
    // A report into the results workspace's copy is applied over these, so
    // the copy needs them; whatever a frozen run writes into them, toasts and
    // outbox pushes included, is discarded with the copy, as it is today.
    wb: WorkbenchSession,
    platform: PlatformState,
    review_markers: SchematicReviewMarkerVisibility,
    veriloga_compilation: Option<ProjectCompileReceipt>,
    netlist_shown: NetlistShown,
    results: Box<dyn ResultsPeer>,
    commands: &'static Registry,
}

impl RunEventSink for FrozenRun {
    fn apply_run_event(&mut self, event: RunEvent) {
        self.results.apply_run_event(
            event,
            &mut self.model,
            &mut self.wb,
            &self.platform,
            &mut self.runs.preflight,
            self.commands,
        );
    }
}

impl ViewerHost for FrozenRun {
    fn model(&self) -> &Model {
        &self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        self.results.viewer_reading()
    }

    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions {
        self.results.fft_input_options(time)
    }

    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results.eye_timebase_for(owner)
    }
}

impl RunHost for FrozenRun {
    type Frozen = FrozenRun;

    fn freeze(&self) -> FrozenRun {
        FrozenRun {
            model: self.model.clone(),
            runs: self.runs.clone(),
            wb: self.wb.clone(),
            platform: self.platform.clone(),
            review_markers: self.review_markers,
            veriloga_compilation: self.veriloga_compilation.clone(),
            netlist_shown: self.netlist_shown,
            results: self.results.frozen(),
            commands: self.commands,
        }
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

    /// Nobody reads a frozen copy's status bar.
    fn prompt(&mut self, _text: impl Into<String>) {}

    fn slot_facts(&self) -> SlotFacts {
        self.runs.slot_facts(self.review_markers)
    }

    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        self.veriloga_compilation.as_ref()
    }

    fn netlist_shown(&self) -> NetlistShown {
        self.netlist_shown
    }

    /// Nobody reads a frozen copy's Problems page.
    fn show_problem_findings(&mut self) {}

    /// The run set is queued on the copy's model; the reader's workspace
    /// stays where it is.
    fn queue_run_set(&mut self) {
        request_run_set(&mut self.model, &mut self.runs);
    }

    fn sync_active_schematic_to_workspace(&mut self) {
        let _ = self.model.sync_active_schematic_to_workspace();
    }

    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model
            .ensure_design_check(subject, CheckScope::Design, facts)
            .0
    }
}
