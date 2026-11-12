//! The Automation page's Validate, Run workflow, Stop workflow, breakpoint
//! toggle and artifact export, in whole frames of the application.
//!
//! The page raises each for the dispatcher, which carries it out once the
//! page has drawn. A press must still take effect in the frame that made
//! it, and that frame must ask for the next at once, so what the press
//! changed is drawn without waiting for the reader to move the pointer.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use egui::accesskit::Role;
use egui::{Key, Modifiers, Pos2};

use crate::automation_workflow::{ArtifactKind, RenderedArtifact};
use crate::feature_tests::replay::{Control, Replay, controls, only};
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::workbench::documents::code_workspace::{
    self, AutomationArtifactStore, AutomationExecutionState, AutomationRuntimeLaunchMode,
    CodeWorkspacePage, PendingAutomationRuntimeExecution, SourceOperationToken,
};
use crate::workbench::state::Workspace;

/// A session with a project open, on the Automation page.
fn automation_page() -> Replay {
    let mut replay = Replay::new();
    replay.app.state.workbench.activate(Workspace::Netlist);
    replay.app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    replay
}

/// The controls the last of three settling frames published.
fn settled(replay: &mut Replay) -> Vec<Control> {
    replay.frames(Vec::new())
}

/// The one control named `label` with `role` in a settled frame.
fn control(replay: &mut Replay, label: &str, role: Role) -> Pos2 {
    only(&settled(replay), label, role).center
}

/// Press the control at `at` in one frame and release it in the next; what
/// the frame of the press left and what the frame of the release put out.
fn press_and_release(
    replay: &mut Replay,
    at: Pos2,
    answered: impl Fn(&Replay) -> bool,
) -> egui::FullOutput {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(at), button(true)]);
    assert!(!answered(replay), "a press is carried out on its release");
    replay.frame(vec![button(false)])
}

/// Whether `output` asks for the next frame at once.
fn repaints_at_once(output: &egui::FullOutput) -> bool {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .is_some_and(|viewport| viewport.repaint_delay.is_zero())
}

/// The controls the tree `output` published.
fn published(output: &egui::FullOutput) -> Vec<Control> {
    controls(
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("a frame with AccessKit on publishes its tree"),
    )
}

/// Whether `controls` hold a button named `label`.
fn has_button(controls: &[Control], label: &str) -> bool {
    controls
        .iter()
        .any(|control| control.role == Role::Button && control.label == label)
}

/// The exact Automation closure the page shows.
fn token(replay: &Replay) -> (crate::state::ProjectSourceId, SourceOperationToken) {
    let bundle = replay
        .app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .expect("the project has an Automation workspace");
    (
        bundle.id(),
        SourceOperationToken {
            project_id: replay.app.state.model.workspace.content.project.id(),
            revision: bundle.revision().get(),
            content_digest: bundle.closure_digest(),
        },
    )
}

/// Whether the workspace was validated, or its validation refused.
fn validation_answered(replay: &Replay) -> bool {
    let runtime = &replay.app.state.code.code_workspace.automation;
    runtime.diagnostic_token.is_some() || runtime.last_error.is_some()
}

/// The page's Validate validates the workspace once the page has drawn, in
/// the frame of its release, and that frame asks for the next at once.
#[test]
fn validate_is_carried_out_in_the_frame_of_its_release() {
    let mut replay = automation_page();
    let at = control(&mut replay, "Validate", Role::Button);
    assert!(!validation_answered(&replay));
    let release = press_and_release(&mut replay, at, validation_answered);
    assert!(
        validation_answered(&replay),
        "Validate is carried out in the frame of its release"
    );
    assert!(repaints_at_once(&release));
}

/// Run workflow asks for the run once the page has drawn, in the frame of
/// its release: the workspace is validated first, with the run deferred to
/// the validation's end. That frame asks for the next at once.
#[test]
fn run_workflow_is_carried_out_in_the_frame_of_its_release() {
    let mut replay = automation_page();
    let at = control(&mut replay, "Run workflow", Role::Button);
    let asked = |replay: &Replay| {
        replay
            .app
            .state
            .code
            .code_workspace
            .automation
            .deferred_runtime_launch
            == Some(AutomationRuntimeLaunchMode::Run)
    };
    let release = press_and_release(&mut replay, at, asked);
    assert!(asked(&replay), "Run workflow is carried out on its release");
    assert!(validation_answered(&replay));
    assert!(repaints_at_once(&release));
}

/// Stop workflow stops the managed run once the page has drawn, in the
/// frame of its release. That frame still draws Stop workflow, as the
/// workflow was when the page drew, and asks for the next at once, which
/// draws Run workflow.
#[test]
fn stop_workflow_is_carried_out_in_the_frame_of_its_release() {
    let mut replay = automation_page();
    let (bundle_id, token) = token(&replay);
    let runtime = &mut replay.app.state.code.code_workspace.automation;
    runtime.execution = AutomationExecutionState::PythonBroker {
        token,
        dry_run: false,
    };
    runtime.runtime_execution = Some(PendingAutomationRuntimeExecution {
        request_id: 1,
        bundle_id,
        token,
        mode: AutomationRuntimeLaunchMode::Run,
        session_id: None,
        last_call_id: 0,
        project_handle: None,
        plan_handle: None,
        preview_handle: None,
        run_handle: None,
        simulation_run_id: None,
        pending_execute_call_id: None,
    });
    let at = control(&mut replay, "Stop workflow", Role::Button);
    let stopped = |replay: &Replay| {
        matches!(
            replay.app.state.code.code_workspace.automation.execution,
            AutomationExecutionState::Cancelled
        )
    };
    let release = press_and_release(&mut replay, at, stopped);
    assert!(
        stopped(&replay),
        "Stop workflow is carried out on its release"
    );
    assert!(
        has_button(&published(&release), "Stop workflow"),
        "the frame of the release draws the workflow as it was"
    );
    assert!(repaints_at_once(&release));
    let next = published(&replay.frame(Vec::new()));
    assert!(has_button(&next, "Run workflow"));
    assert!(!has_button(&next, "Stop workflow"));
}

/// F9 in the editor sets a breakpoint on the caret's line once the page has
/// drawn, in the frame of the key. That frame's debugger still lists no
/// breakpoints and asks for the next frame at once, which lists it.
#[test]
fn a_breakpoint_toggle_is_carried_out_in_the_frame_of_its_key() {
    let mut replay = automation_page();
    let (bundle_id, _) = token(&replay);
    let root = replay
        .app
        .state
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .expect("the Automation workspace")
        .root()
        .logical_path()
        .to_owned();
    settled(&mut replay);
    let editor =
        code_workspace::source_editor_id(ProjectSourceLanguage::RSpiceAutomation, bundle_id, &root);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    settled(&mut replay);
    let breakpoints = |replay: &Replay| {
        replay
            .app
            .state
            .code
            .code_workspace
            .automation
            .debug
            .breakpoints
            .len()
    };
    assert_eq!(breakpoints(&replay), 0);
    let key = |pressed| egui::Event::Key {
        key: Key::F9,
        physical_key: Some(Key::F9),
        pressed,
        repeat: false,
        modifiers: Modifiers::NONE,
    };
    let toggle = replay.frame(vec![key(true), key(false)]);
    assert_eq!(
        breakpoints(&replay),
        1,
        "the toggle is carried out in the frame of its key"
    );
    let header = |controls: &[Control]| {
        controls
            .iter()
            .any(|control| control.role == Role::Heading && control.label == "Breakpoints")
    };
    assert!(
        !header(&published(&toggle)),
        "the frame of the key draws the debugger as it was"
    );
    assert!(repaints_at_once(&toggle));
    assert!(header(&published(&replay.frame(Vec::new()))));
}

/// A save destination that counts each time it is asked for, and is then
/// cancelled.
struct CountedSave(Arc<AtomicUsize>);

impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for CountedSave {
    fn show_save_dialog(
        &self,
        _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<PathBuf>, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(None)
    }

    fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// An artifact row's press asks where to export once the page has drawn, in
/// the frame of its release, and that frame asks for the next at once.
#[test]
fn an_artifact_export_is_carried_out_in_the_frame_of_its_release() {
    let mut replay = automation_page();
    let asked = Arc::new(AtomicUsize::new(0));
    replay.app.export_workflow_io = Box::new(CountedSave(asked.clone()));
    replay.app.state.code.code_workspace.automation.artifacts =
        AutomationArtifactStore::try_new(vec![RenderedArtifact::new(
            ArtifactKind::SummaryJson,
            b"{}".to_vec(),
        )])
        .expect("one artifact");
    let at = control(&mut replay, "Export summary.json", Role::Button);
    let exported = |_: &Replay| asked.load(Ordering::SeqCst) > 0;
    let release = press_and_release(&mut replay, at, exported);
    assert_eq!(
        asked.load(Ordering::SeqCst),
        1,
        "the export is carried out on its release"
    );
    assert!(repaints_at_once(&release));
}
