//! The run tier's host over a simulate command's context.
//!
//! A run started from one of simulate's commands reads and reports through
//! the context the command runs in: the model, the runs and the session
//! directly, and what code, design and the results workspace show through
//! their peers. Each operation does what the application state's own host
//! does, at the same point and in the same order.

use rspice_results_ui::eye_diagram::EyeTimebase;
use rspice_results_ui::fft::FftInputOptions;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt;

use super::SimulateFeature;
use crate::simulation::{NetlistShown, RunHost, SimulationController, ViewerHost, ViewerReading};
use crate::state::{CellViewRef, RunEvent, RunEventSink, SpecializedViewerCacheProvenance};
use crate::workbench::app_state::design_checks::CheckScope;
use crate::workbench::app_state::{Model, SlotFacts};
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::framework::feature::{Cx, Peers};
use crate::workbench::framework::ports::ResultsPeer;
use crate::workbench::slots::{PlatformState, RunsState};
use crate::workbench::state::{WorkbenchSession, Workspace};

/// The run tier's host over a simulate command's context.
pub(super) struct RunView<'v, 'a> {
    model: &'v mut Model,
    runs: &'v mut RunsState,
    wb: &'v mut WorkbenchSession,
    platform: &'v PlatformState,
    peers: &'v mut Peers<'a>,
    commands: &'static Registry,
}

/// The run tier's host over `cx`, and beside it the run controller it drives.
pub(super) fn run_host<'v, 'a>(
    cx: &'v mut Cx<'a, SimulateFeature>,
) -> (RunView<'v, 'a>, &'v mut SimulationController) {
    let host = RunView {
        model: &mut *cx.model,
        runs: &mut *cx.runs,
        wb: &mut *cx.wb,
        platform: &*cx.platform,
        peers: &mut cx.peers,
        commands: cx.commands,
    };
    (host, &mut *cx.handles.run_controller)
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
            veriloga_compilation: self.peers.code().veriloga_compilation().cloned(),
            netlist_shown: self.peers.code().netlist_shown(),
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
        slot_facts(self.runs, self.peers.design().review_markers())
    }

    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt> {
        self.peers.code().veriloga_compilation()
    }

    fn netlist_shown(&self) -> NetlistShown {
        self.peers.code().netlist_shown()
    }

    fn show_problem_findings(&mut self) {
        self.wb.show_problem_findings();
    }

    fn queue_run_set(&mut self) {
        request_run_set(self.model, self.runs);
        self.wb.activate(Workspace::Simulate);
    }

    fn sync_active_schematic_to_workspace(&mut self) {
        if let Some(prompt) = self.model.sync_active_schematic_to_workspace() {
            self.wb.prompt(prompt);
        }
    }

    fn ensure_design_check(&mut self, subject: CellViewRef) -> Result<(), String> {
        let facts = self.slot_facts();
        let (outcome, prompt) = ensure_design_check(self.model, subject, facts);
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
pub(super) struct FrozenRun {
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
        slot_facts(&self.runs, self.review_markers)
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
        ensure_design_check(&mut self.model, subject, facts).0
    }
}

/// What model operations read of the slots: whether the runs retain a
/// preflight report, and which review markers design shows.
fn slot_facts(runs: &RunsState, review_markers: SchematicReviewMarkerVisibility) -> SlotFacts {
    SlotFacts {
        preflight_report_retained: runs.preflight.report.is_some(),
        review_markers,
    }
}

/// Request a run of the Simulate workspace's run set, which replaces a
/// netlist rerun the runs had queued.
fn request_run_set(model: &mut Model, runs: &mut RunsState) {
    runs.manual_run.rerun_queued = false;
    model.simulation.execution.request_simulate_run_set();
}

/// Check `subject` as Check Design does, unless a current check of it already
/// describes it, with what the drawing sync had to say.
fn ensure_design_check(
    model: &mut Model,
    subject: CellViewRef,
    facts: SlotFacts,
) -> (Result<(), String>, Option<String>) {
    if model
        .current_design_check(&subject, CheckScope::Design, facts)
        .is_some()
    {
        return (Ok(()), None);
    }
    let (outcome, prompt) = model.run_synced_design_check(subject, CheckScope::Design, facts);
    (outcome.map(|_| ()), prompt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;
    use crate::workbench::commands::vocabulary::command_registry;
    use crate::workbench::framework::feature::LowerHandles;

    /// What the run tier's host over simulate's context over `app` freezes,
    /// lending it the application's parts as the frame does.
    fn frozen_from(app: &mut RSpiceApp) -> FrozenRun {
        let handles = LowerHandles {
            export_io: app.export_workflow_io.as_ref(),
            run_controller: &mut app.simulation_controller,
            model_hub: &mut app.model_hub,
        };
        let state = &mut app.state;
        let mut cx = Cx::new(
            &mut *state.simulate,
            &mut [],
            &mut state.model,
            &mut *state.workbench,
            &mut state.platform,
            &mut state.runs,
            handles,
            Peers::new(
                Some(&mut *state.design),
                Some(&*state.cloud),
                Some(&*state.hardcopy),
                Some(&mut *state.results),
                None,
                Some(&*state.code),
                Some(&*state.models),
            ),
            command_registry(),
        );
        let (host, _) = run_host(&mut cx);
        host.freeze()
    }

    #[test]
    fn a_campaigns_frozen_host_leaves_the_live_results_and_runs_untouched() {
        let mut app = RSpiceApp::test_instance();
        let live_source = app.state.results.analysis.fft_state.selected_source.clone();
        let live_rerun = app.state.runs.manual_run.rerun_queued;
        let mut frozen = frozen_from(&mut app);

        frozen.apply_run_event(RunEvent::FftSourceChosen {
            source: "V(out)".to_owned(),
        });
        frozen.runs_mut().manual_run.rerun_queued = !live_rerun;

        assert_eq!(
            frozen.viewer_reading().fft_source.as_deref(),
            Some("V(out)")
        );
        assert_eq!(frozen.runs().manual_run.rerun_queued, !live_rerun);
        assert_eq!(
            app.state.results.analysis.fft_state.selected_source,
            live_source
        );
        assert_eq!(app.state.runs.manual_run.rerun_queued, live_rerun);
    }
}
