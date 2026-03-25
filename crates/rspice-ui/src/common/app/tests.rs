use super::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Default)]
struct MockAppFileWorkflowIo {
    open_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
    save_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
    load_results:
        RefCell<VecDeque<Result<crate::state::SchematicState, crate::io::SchematicIoError>>>,
    save_results: RefCell<VecDeque<Result<(), crate::io::SchematicIoError>>>,
    open_dialog_calls: Cell<usize>,
    save_dialog_calls: Cell<usize>,
    load_calls: Cell<usize>,
    save_calls: Cell<usize>,
    last_save_default_name: RefCell<Option<Option<String>>>,
}

impl MockAppFileWorkflowIo {
    fn push_open_dialog_result(&self, result: Result<PathBuf, crate::io::SchematicIoError>) {
        self.open_dialog_results.borrow_mut().push_back(result);
    }

    fn push_save_dialog_result(&self, result: Result<PathBuf, crate::io::SchematicIoError>) {
        self.save_dialog_results.borrow_mut().push_back(result);
    }

    fn push_load_result(
        &self,
        result: Result<crate::state::SchematicState, crate::io::SchematicIoError>,
    ) {
        self.load_results.borrow_mut().push_back(result);
    }

    fn push_save_result(&self, result: Result<(), crate::io::SchematicIoError>) {
        self.save_results.borrow_mut().push_back(result);
    }

    fn open_dialog_calls(&self) -> usize {
        self.open_dialog_calls.get()
    }

    fn save_dialog_calls(&self) -> usize {
        self.save_dialog_calls.get()
    }

    fn load_calls(&self) -> usize {
        self.load_calls.get()
    }

    fn save_calls(&self) -> usize {
        self.save_calls.get()
    }

    fn last_save_default_name(&self) -> Option<Option<String>> {
        self.last_save_default_name.borrow().clone()
    }
}

impl crate::common::file_workflow::FileWorkflowIo for Rc<MockAppFileWorkflowIo> {
    fn show_open_dialog(&self) -> Result<PathBuf, crate::io::SchematicIoError> {
        self.open_dialog_calls
            .set(self.open_dialog_calls.get().saturating_add(1));
        self.open_dialog_results
            .borrow_mut()
            .pop_front()
            .expect("test must provide show_open_dialog result")
    }

    fn show_save_dialog(
        &self,
        default_name: Option<&str>,
    ) -> Result<PathBuf, crate::io::SchematicIoError> {
        self.save_dialog_calls
            .set(self.save_dialog_calls.get().saturating_add(1));
        *self.last_save_default_name.borrow_mut() =
            Some(default_name.map(std::string::ToString::to_string));
        self.save_dialog_results
            .borrow_mut()
            .pop_front()
            .expect("test must provide show_save_dialog result")
    }

    fn load_schematic(
        &self,
        _path: &Path,
    ) -> Result<crate::state::SchematicState, crate::io::SchematicIoError> {
        self.load_calls.set(self.load_calls.get().saturating_add(1));
        self.load_results
            .borrow_mut()
            .pop_front()
            .expect("test must provide load_schematic result")
    }

    fn save_schematic(
        &self,
        _schematic: &crate::state::SchematicState,
        _path: &Path,
    ) -> Result<(), crate::io::SchematicIoError> {
        self.save_calls.set(self.save_calls.get().saturating_add(1));
        self.save_results
            .borrow_mut()
            .pop_front()
            .expect("test must provide save_schematic result")
    }
}

fn schematic_with_component(
    kind: crate::state::ComponentType,
    x: i32,
    y: i32,
) -> crate::state::SchematicState {
    let mut schematic = crate::state::SchematicState::default();
    schematic.add_component(kind, crate::state::Point::new(x, y));
    schematic
}

fn make_test_app() -> RSpiceApp {
    RSpiceApp::new_for_tests(AppState::default())
}

#[test]
fn test_prepare_frame_applies_first_frame_setup_once() {
    let mut app = make_test_app();
    app.first_frame = true;

    let ctx = egui::Context::default();
    app.prepare_frame(&ctx);
    assert!(!app.first_frame, "first-frame preparation should complete");

    app.prepare_frame(&ctx);
    assert!(
        !app.first_frame,
        "subsequent frame preparation should be stable"
    );
}

fn seed_eye_data(state: &mut AppState) {
    let mut eye = crate::analysis::eye_diagram::data::EyeData::default();
    eye.add_trace(crate::analysis::eye_diagram::data::EyeTrace::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0],
    ));
    state.analysis.eye_diagram_state.load_data(eye);
}

fn seed_bode_data(state: &mut AppState) {
    let mut response = crate::analysis::bode::data::FrequencyResponse::new("tf");
    response.add_point(crate::analysis::bode::data::FrequencyPoint::new(
        1.0, 1.0, 0.0,
    ));
    let mut data = crate::analysis::bode::BodeData::new();
    data.add_response(response);
    state.analysis.bode_plot_state.load_data(data);
}

fn seed_nyquist_data(state: &mut AppState) {
    let curve = crate::analysis::nyquist::NyquistData::from_arrays(
        "loop",
        &[1.0, 10.0],
        &[0.5, 0.25],
        &[0.0, -0.2],
    );
    state.analysis.nyquist_state.load_data(curve);
}

#[test]
fn test_replace_waveform_results_refreshes_simulation_waveforms() {
    let mut state = AppState::default();
    let initial_version = state.simulation.data_version;
    let waveform =
        crate::state::WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 0.5], "#4aa3ff");

    state.replace_waveform_results(vec![waveform.clone()]);

    assert_eq!(state.simulation.waveforms, vec![waveform]);
    assert_eq!(state.simulation.node_to_waveform.get("V(out)"), Some(&0));
    assert_ne!(state.simulation.data_version, initial_version);
}

#[test]
fn test_set_waveform_view_x_range_rejects_invalid_bounds() {
    let mut state = AppState::default();

    assert_eq!(
        state.set_waveform_view_x_range(f64::NAN, 1.0),
        Err(WaveformViewRangeError::NonFiniteBounds)
    );
    assert_eq!(
        state.set_waveform_view_x_range(1.0, 1.0),
        Err(WaveformViewRangeError::NonPositiveRange)
    );
    assert_eq!(
        state.set_waveform_view_x_range(2.0, 1.0),
        Err(WaveformViewRangeError::NonPositiveRange)
    );
}

#[test]
fn test_set_waveform_view_x_range_updates_window_and_clamps_to_loaded_bounds() {
    let mut state = AppState::default();
    state.replace_waveform_results(vec![crate::state::WaveformData::new(
        "V(out)",
        vec![0.0, 1.0],
        vec![0.0, 0.5],
        "#4aa3ff",
    )]);
    let waveforms = state.simulation.waveforms.clone();
    state.waveform_viewer.load_from_simulation(&waveforms);

    state
        .set_waveform_view_x_range(-0.5, 2.0)
        .expect("range should be accepted");

    assert_eq!(state.waveform_viewer.view.x_min, 0.0);
    assert_eq!(state.waveform_viewer.view.x_max, 1.0);
}

#[test]
fn test_app_state_default() {
    let state = AppState::default();
    assert!(
        !state.panels.project_browser,
        "Browser should be hidden by default"
    );
    assert!(
        state.panels.properties,
        "Properties should be visible by default"
    );
    assert!(
        state.panels.bottom_panel,
        "Bottom panel should be visible by default"
    );
    assert_eq!(
        state.panels.active_bottom_tab,
        BottomPanelTab::Log,
        "Log tab should be active by default"
    );
}

#[test]
fn test_panel_sizes_default() {
    let sizes = PanelSizes::default();
    assert_eq!(sizes.waveform_height, 300.0);
    assert_eq!(sizes.console_height, 120.0);
    assert_eq!(sizes.browser_width, 220.0);
    assert_eq!(sizes.properties_width, 250.0);
}

#[test]
fn test_console_message_info() {
    let msg = ConsoleMessage::info("Test message");
    assert_eq!(msg.level, ConsoleLevel::Info);
    assert_eq!(msg.message, "Test message");
}

#[test]
fn test_console_message_warning() {
    let msg = ConsoleMessage::warning("Warning message");
    assert_eq!(msg.level, ConsoleLevel::Warning);
}

#[test]
fn test_console_message_error() {
    let msg = ConsoleMessage::error("Error message");
    assert_eq!(msg.level, ConsoleLevel::Error);
}

#[test]
fn test_dialog_state_default() {
    let dialogs = DialogState::default();
    assert!(!dialogs.simulation_dialog);
    assert!(!dialogs.simulation_options);
    assert!(!dialogs.about);
    assert!(!dialogs.preferences);
    assert!(!dialogs.shortcuts_help);
    assert_eq!(dialogs.simulation_options_state.active_tab, 0);
    assert!(dialogs.simulation_options_errors.is_empty());
    assert!((dialogs.simulation_options_config.reltol - 1e-3).abs() < 1e-15);
}

#[test]
fn test_panel_visibility_serialization() {
    let panels = PanelVisibility {
        project_browser: true,
        results_browser: false,
        properties: false,
        bottom_panel: true,
        active_bottom_tab: BottomPanelTab::Waveform,
        signal_browser: false,
        script_console: false,
    };
    let ser = PanelVisibilitySer::from(&panels);
    assert!(ser.project_browser);
    assert!(!ser.properties);
    assert!(ser.bottom_panel);
    assert_eq!(ser.active_bottom_tab, 1); // Waveform = 1

    let panels2: PanelVisibility = ser.into();
    assert!(panels2.project_browser);
    assert!(!panels2.properties);
    assert!(panels2.bottom_panel);
    assert_eq!(panels2.active_bottom_tab, BottomPanelTab::Waveform);
}

#[test]
fn test_panel_visibility_deserialization_legacy_console_index_maps_to_log() {
    let legacy = PanelVisibilitySer {
        project_browser: false,
        results_browser: false,
        properties: true,
        bottom_panel: true,
        active_bottom_tab: 0,
        signal_browser: false,
        script_console: false,
    };
    let panels: PanelVisibility = legacy.into();
    assert_eq!(panels.active_bottom_tab, BottomPanelTab::Log);
}

#[test]
fn test_panel_sizes_serialization() {
    let sizes = PanelSizes {
        waveform_height: 300.0,
        console_height: 150.0,
        browser_width: 280.0,
        properties_width: 320.0,
    };
    let ser = PanelSizesSer::from(&sizes);
    assert_eq!(ser.waveform_height, 300.0);

    let sizes2: PanelSizes = ser.into();
    assert_eq!(sizes2.waveform_height, 300.0);
    assert_eq!(sizes2.console_height, 150.0);
}

#[test]
fn test_viewer_workspace_serialization_round_trip() {
    let mut workspace = crate::viewers::ViewerWorkspace::default();
    workspace.open_or_focus(crate::viewers::ActiveViewer::BodePlot);
    workspace.open_or_focus(crate::viewers::ActiveViewer::Histogram);
    workspace.focus(crate::viewers::ActiveViewer::BodePlot);

    let serialized = ViewerWorkspaceSer::from(&workspace);
    assert_eq!(serialized.tabs.len(), 3);
    assert_eq!(
        serialized.tabs,
        vec![
            crate::viewers::ActiveViewer::Waveform.id(),
            crate::viewers::ActiveViewer::BodePlot.id(),
            crate::viewers::ActiveViewer::Histogram.id(),
        ]
    );

    let restored: crate::viewers::ViewerWorkspace = serialized.into();
    assert_eq!(
        restored.tabs(),
        &[
            crate::viewers::ActiveViewer::Waveform,
            crate::viewers::ActiveViewer::BodePlot,
            crate::viewers::ActiveViewer::Histogram,
        ]
    );
    assert_eq!(
        restored.active_viewer(),
        crate::viewers::ActiveViewer::BodePlot
    );
}

#[test]
fn test_viewer_workspace_deserialization_filters_invalid_ids() {
    let serialized = ViewerWorkspaceSer {
        tabs: vec![255, crate::viewers::ActiveViewer::Nyquist.id()],
        active_index: 999,
    };

    let restored: crate::viewers::ViewerWorkspace = serialized.into();
    assert_eq!(restored.tabs(), &[crate::viewers::ActiveViewer::Nyquist]);
    assert_eq!(
        restored.active_viewer(),
        crate::viewers::ActiveViewer::Nyquist
    );
}

#[test]
fn test_app_state_open_viewer_routes_to_waveform_tab() {
    let mut state = AppState::default();
    seed_eye_data(&mut state);
    state.panels.active_bottom_tab = BottomPanelTab::Log;

    state.open_viewer(crate::viewers::ActiveViewer::EyeDiagram);

    assert!(state.panels.bottom_panel);
    assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Waveform);
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::EyeDiagram
    );
    assert!(
        state
            .viewer_workspace
            .contains(crate::viewers::ActiveViewer::EyeDiagram)
    );
}

#[test]
fn test_app_state_open_viewer_in_tab_respects_target_tab() {
    let mut state = AppState::default();
    seed_bode_data(&mut state);

    state.open_viewer_in_tab(
        crate::viewers::ActiveViewer::BodePlot,
        BottomPanelTab::Automation,
    );

    assert_eq!(state.panels.active_bottom_tab, BottomPanelTab::Automation);
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::BodePlot
    );
}

#[test]
fn test_app_state_close_active_viewer_keeps_workspace_non_empty() {
    let mut state = AppState::default();
    seed_nyquist_data(&mut state);
    state.open_viewer(crate::viewers::ActiveViewer::Nyquist);
    assert_eq!(state.active_viewer(), crate::viewers::ActiveViewer::Nyquist);

    state.close_active_viewer();
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::Waveform
    );
    assert!(
        state
            .viewer_workspace
            .contains(crate::viewers::ActiveViewer::Waveform)
    );

    state.close_active_viewer();
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::Waveform
    );
    assert_eq!(state.viewer_workspace.tab_count(), 1);
}

#[test]
fn test_app_state_serialization_round_trip_preserves_viewer_workspace() {
    let mut state = AppState::default();
    seed_bode_data(&mut state);
    seed_nyquist_data(&mut state);
    state.open_viewer(crate::viewers::ActiveViewer::BodePlot);
    state.open_viewer(crate::viewers::ActiveViewer::Nyquist);
    state
        .viewer_workspace
        .focus(crate::viewers::ActiveViewer::BodePlot);

    let encoded = serde_json::to_string(&state).expect("serialize app state");
    let restored: AppState = serde_json::from_str(&encoded).expect("deserialize app state");

    assert_eq!(
        restored.viewer_workspace.tabs(),
        &[
            crate::viewers::ActiveViewer::Waveform,
            crate::viewers::ActiveViewer::BodePlot,
            crate::viewers::ActiveViewer::Nyquist,
        ]
    );
    assert_eq!(
        restored.active_viewer(),
        crate::viewers::ActiveViewer::BodePlot
    );
}

#[test]
fn test_app_state_deserialization_legacy_payload_defaults_viewer_workspace() {
    let mut state = AppState::default();
    seed_eye_data(&mut state);
    state.open_viewer(crate::viewers::ActiveViewer::EyeDiagram);
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::EyeDiagram
    );
    assert_eq!(state.viewer_workspace.tab_count(), 2);

    let mut value = serde_json::to_value(&state).expect("serialize to value");
    let map = value
        .as_object_mut()
        .expect("serialized app state should be object");
    map.remove("viewer_workspace");

    let restored: AppState = serde_json::from_value(value).expect("deserialize legacy payload");
    assert_eq!(
        restored.active_viewer(),
        crate::viewers::ActiveViewer::Waveform
    );
    assert_eq!(restored.viewer_workspace.tab_count(), 1);
}

#[test]
fn test_theme_is_dark_by_default() {
    let state = AppState::default();
    assert!(
        state.theme.is_dark,
        "Theme should be dark by default for EDA"
    );
}

// =========================================================================
// Save Confirmation Dialog Tests
// Commercial-grade testing for unsaved changes workflow
// =========================================================================

#[test]
fn test_confirmation_action_dialog_titles() {
    // Verify all actions have appropriate dialog titles
    assert_eq!(
        ConfirmationAction::FileNew.dialog_title(),
        "Create New Schematic",
        "FileNew should have descriptive title"
    );
    assert_eq!(
        ConfirmationAction::FileOpen.dialog_title(),
        "Open Schematic",
        "FileOpen should have descriptive title"
    );
    assert_eq!(
        ConfirmationAction::Exit.dialog_title(),
        "Exit RSpice",
        "Exit should have descriptive title"
    );
}

#[test]
fn test_confirmation_action_prompt_messages() {
    // All actions should have clear, user-friendly prompts
    let message = ConfirmationAction::FileNew.prompt_message();
    assert!(
        message.contains("unsaved"),
        "Prompt should mention unsaved changes"
    );
    assert!(message.contains("save"), "Prompt should mention saving");
}

#[test]
fn test_confirmation_dialog_state_default() {
    let state = ConfirmationDialogState::default();
    assert!(!state.visible, "Dialog should be hidden by default");
    assert!(
        state.pending_action.is_none(),
        "No pending action by default"
    );
}

#[test]
fn test_confirmation_dialog_state_show() {
    let mut state = ConfirmationDialogState::default();

    // Test showing dialog for FileNew
    state.show(ConfirmationAction::FileNew);
    assert!(state.visible, "Dialog should be visible after show()");
    assert_eq!(
        state.pending_action,
        Some(ConfirmationAction::FileNew),
        "Pending action should be set"
    );

    // Test showing dialog for different action
    state.show(ConfirmationAction::FileOpen);
    assert!(state.visible, "Dialog should remain visible");
    assert_eq!(
        state.pending_action,
        Some(ConfirmationAction::FileOpen),
        "Pending action should be updated"
    );
}

#[test]
fn test_confirmation_dialog_state_close() {
    let mut state = ConfirmationDialogState::default();

    // Show then close
    state.show(ConfirmationAction::FileNew);
    state.close();

    assert!(!state.visible, "Dialog should be hidden after close()");
    assert!(
        state.pending_action.is_none(),
        "Pending action should be cleared after close()"
    );
}

#[test]
fn test_confirmation_dialog_state_is_showing() {
    let mut state = ConfirmationDialogState::default();

    // Not showing anything initially
    assert!(
        !state.is_showing(ConfirmationAction::FileNew),
        "Should not be showing FileNew initially"
    );

    // Show FileNew
    state.show(ConfirmationAction::FileNew);
    assert!(
        state.is_showing(ConfirmationAction::FileNew),
        "Should be showing FileNew after show()"
    );
    assert!(
        !state.is_showing(ConfirmationAction::FileOpen),
        "Should not be showing FileOpen"
    );
    assert!(
        !state.is_showing(ConfirmationAction::Exit),
        "Should not be showing Exit"
    );
}

#[test]
fn test_confirmation_response_enum_completeness() {
    // Verify all three commercial-standard responses exist
    let responses = [
        ConfirmationResponse::Yes,
        ConfirmationResponse::No,
        ConfirmationResponse::Cancel,
    ];
    assert_eq!(
        responses.len(),
        3,
        "Should have exactly 3 response options (Yes/No/Cancel)"
    );

    // Verify they are all distinct
    assert_ne!(ConfirmationResponse::Yes, ConfirmationResponse::No);
    assert_ne!(ConfirmationResponse::Yes, ConfirmationResponse::Cancel);
    assert_ne!(ConfirmationResponse::No, ConfirmationResponse::Cancel);
}

#[test]
fn test_app_state_has_confirmation_dialog() {
    let state = AppState::default();

    // Verify confirmation dialog is accessible and properly initialized
    assert!(
        !state.dialogs.confirmation_dialog.visible,
        "Confirmation dialog should be hidden by default"
    );
    assert!(
        state.dialogs.confirmation_dialog.pending_action.is_none(),
        "No pending action on fresh AppState"
    );
}

#[test]
fn test_app_state_exit_requested_default() {
    let state = AppState::default();
    assert!(
        !state.exit_requested,
        "Exit should not be requested by default"
    );
}

#[test]
fn test_process_exit_request_noop_when_not_requested() {
    let mut app = make_test_app();
    app.state.exit_requested = false;
    let ctx = Context::default();

    app.process_exit_request(&ctx);

    assert!(
        !app.state.exit_requested,
        "exit flag should remain false when no exit was requested"
    );
}

#[test]
fn test_process_exit_request_clears_flag_after_dispatch() {
    let mut app = make_test_app();
    app.state.exit_requested = true;
    let ctx = Context::default();

    app.process_exit_request(&ctx);

    assert!(
        !app.state.exit_requested,
        "exit flag should be cleared after close command dispatch"
    );
}

#[test]
fn test_confirmation_action_is_copy_and_eq() {
    // Verify ConfirmationAction implements Copy and Eq for efficiency
    let action = ConfirmationAction::FileNew;
    let action_copy = action; // Copy
    assert_eq!(
        action, action_copy,
        "ConfirmationAction should implement Eq"
    );

    // Verify all variants can be compared
    let actions = [
        ConfirmationAction::FileNew,
        ConfirmationAction::FileOpen,
        ConfirmationAction::Exit,
    ];
    for (i, a) in actions.iter().enumerate() {
        for (j, b) in actions.iter().enumerate() {
            if i == j {
                assert_eq!(a, b, "Same action should be equal");
            } else {
                assert_ne!(a, b, "Different actions should not be equal");
            }
        }
    }
}

#[test]
fn test_confirmation_dialog_workflow_complete_cycle() {
    // Simulate a complete workflow: dirty state -> show dialog -> close
    let mut state = ConfirmationDialogState::default();

    // Initial state
    assert!(!state.visible);

    // User triggers action that requires confirmation
    state.show(ConfirmationAction::Exit);
    assert!(state.visible);
    assert_eq!(state.pending_action, Some(ConfirmationAction::Exit));

    // User cancels
    state.close();
    assert!(!state.visible);
    assert!(state.pending_action.is_none());

    // Trigger another action
    state.show(ConfirmationAction::FileNew);
    assert!(state.is_showing(ConfirmationAction::FileNew));

    // Complete the action
    let action = state.pending_action;
    state.close();
    assert_eq!(action, Some(ConfirmationAction::FileNew));
}

#[test]
fn test_confirmation_yes_executes_pending_exit_after_successful_save() {
    let mut app = make_test_app();
    let temp = tempfile::tempdir().expect("tempdir should create");
    let save_path = temp.path().join("confirmation-save-success.rsch");
    app.state.schematic.current_file = Some(save_path.clone());
    app.state.schematic.is_dirty = true;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::Exit);

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert!(
        app.state.exit_requested,
        "successful save should allow pending exit action to proceed"
    );
    assert!(
        save_path.exists(),
        "successful confirmation save should persist schematic file"
    );
    assert!(
        !app.state.schematic.is_dirty,
        "successful save should clear dirty flag"
    );
    assert!(!app.state.dialogs.confirmation_dialog.visible);
    assert!(
        app.state
            .dialogs
            .confirmation_dialog
            .pending_action
            .is_none()
    );
}

#[test]
fn test_confirmation_yes_does_not_execute_pending_exit_when_save_fails() {
    let mut app = make_test_app();
    let temp = tempfile::tempdir().expect("tempdir should create");
    let invalid_file_target = temp.path().to_path_buf(); // existing directory: save should fail
    app.state.schematic.current_file = Some(invalid_file_target);
    app.state.schematic.is_dirty = true;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::Exit);

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert!(
        !app.state.exit_requested,
        "failed save must not continue to pending exit action"
    );
    assert!(
        app.state.schematic.is_dirty,
        "failed save should keep schematic dirty"
    );
    assert!(
        app.state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Save failed")),
        "failed save should emit an error message"
    );
    assert!(!app.state.dialogs.confirmation_dialog.visible);
    assert!(
        app.state
            .dialogs
            .confirmation_dialog
            .pending_action
            .is_none()
    );
}

#[test]
fn test_confirmation_yes_does_not_execute_pending_new_when_save_fails() {
    use crate::state::{Component, ComponentType, Point};

    let mut app = make_test_app();
    app.state.schematic.components.push(
        Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k"),
    );
    let temp = tempfile::tempdir().expect("tempdir should create");
    let invalid_file_target = temp.path().to_path_buf(); // existing directory: save should fail
    app.state.schematic.current_file = Some(invalid_file_target);
    app.state.schematic.is_dirty = true;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileNew);

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert!(
        !app.state.schematic.components.is_empty(),
        "failed save must not continue into destructive FileNew action"
    );
    assert!(
        app.state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Save failed")),
        "failed save should emit an error message"
    );
}

#[test]
fn test_confirmation_yes_file_open_with_save_as_cancelled_does_not_open() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 10, 10);
    app.state.schematic.is_dirty = true;
    app.state.schematic.current_file = None;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileOpen);

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_save_dialog_result(Err(crate::io::SchematicIoError::Cancelled));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert_eq!(io.save_dialog_calls(), 1);
    assert_eq!(io.save_calls(), 0);
    assert_eq!(io.open_dialog_calls(), 0);
    assert_eq!(io.load_calls(), 0);
    assert_eq!(app.state.schematic.components.len(), 1);
    assert!(app.state.schematic.is_dirty);
    assert!(!app.state.dialogs.confirmation_dialog.visible);
    assert!(
        app.state
            .dialogs
            .confirmation_dialog
            .pending_action
            .is_none()
    );
}

#[test]
fn test_confirmation_yes_file_open_executes_after_save_as_success() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 5, 5);
    app.state.schematic.is_dirty = true;
    app.state.schematic.current_file = None;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileOpen);

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_save_dialog_result(Ok(PathBuf::from("saved-before-open.rsch")));
    io.push_save_result(Ok(()));
    io.push_open_dialog_result(Ok(PathBuf::from("opened-after-save.rsch")));
    let mut loaded = schematic_with_component(ComponentType::Capacitor, 15, 15);
    loaded.current_file = Some(PathBuf::from("opened-after-save.rsch"));
    loaded.is_dirty = false;
    io.push_load_result(Ok(loaded));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert_eq!(io.save_dialog_calls(), 1);
    assert_eq!(io.save_calls(), 1);
    assert_eq!(io.open_dialog_calls(), 1);
    assert_eq!(io.load_calls(), 1);
    assert_eq!(app.state.schematic.components.len(), 1);
    assert_eq!(
        app.state.schematic.components[0].kind,
        ComponentType::Capacitor
    );
    assert_eq!(
        app.state.schematic.current_file,
        Some(PathBuf::from("opened-after-save.rsch"))
    );
    assert!(!app.state.schematic.is_dirty);
    assert!(
        app.state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Saved:")),
        "save-before-open should emit save message"
    );
    assert!(
        app.state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Opened:")),
        "pending open should emit open message"
    );
}

#[test]
fn test_action_file_open_uses_injected_file_workflow_backend() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 1, 1);
    app.state.schematic.is_dirty = false;

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_open_dialog_result(Ok(PathBuf::from("opened-by-action.rsch")));
    let mut loaded = schematic_with_component(ComponentType::Inductor, 25, 25);
    loaded.current_file = Some(PathBuf::from("opened-by-action.rsch"));
    loaded.is_dirty = false;
    io.push_load_result(Ok(loaded));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.action_file_open();

    assert_eq!(io.open_dialog_calls(), 1);
    assert_eq!(io.load_calls(), 1);
    assert_eq!(app.state.schematic.components.len(), 1);
    assert_eq!(
        app.state.schematic.components[0].kind,
        ComponentType::Inductor
    );
}

#[test]
fn test_action_file_save_uses_injected_file_workflow_backend() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 3, 3);
    app.state.schematic.current_file = Some(PathBuf::from("current-save-path.rsch"));
    app.state.schematic.is_dirty = true;

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_save_result(Ok(()));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    let saved = app.action_file_save();

    assert!(saved);
    assert_eq!(io.save_dialog_calls(), 0);
    assert_eq!(io.save_calls(), 1);
    assert!(!app.state.schematic.is_dirty);
}

#[test]
fn test_action_file_save_as_passes_current_filename_to_backend() {
    let mut app = make_test_app();
    app.state.schematic.current_file = Some(PathBuf::from("designs/my-cell.rsch"));

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_save_dialog_result(Err(crate::io::SchematicIoError::Cancelled));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    let saved = crate::common::file_actions::action_file_save_as_with_io(&mut app.state, &io);

    assert!(!saved);
    assert_eq!(io.save_dialog_calls(), 1);
    assert_eq!(
        io.last_save_default_name(),
        Some(Some("my-cell.rsch".to_string()))
    );
}

#[test]
fn test_confirmation_no_executes_pending_open_without_save_attempt() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 7, 7);
    app.state.schematic.is_dirty = true;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileOpen);

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_open_dialog_result(Ok(PathBuf::from("opened-from-no.rsch")));
    let mut loaded = schematic_with_component(ComponentType::Capacitor, 40, 40);
    loaded.current_file = Some(PathBuf::from("opened-from-no.rsch"));
    loaded.is_dirty = false;
    io.push_load_result(Ok(loaded));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.handle_confirmation_response(ConfirmationResponse::No);

    assert_eq!(io.save_dialog_calls(), 0);
    assert_eq!(io.save_calls(), 0);
    assert_eq!(io.open_dialog_calls(), 1);
    assert_eq!(io.load_calls(), 1);
    assert_eq!(
        app.state.schematic.components[0].kind,
        ComponentType::Capacitor
    );
    assert!(!app.state.schematic.is_dirty);
}

#[test]
fn test_confirmation_cancel_skips_pending_open_action() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 9, 9);
    app.state.schematic.is_dirty = true;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileOpen);

    let io = Rc::new(MockAppFileWorkflowIo::default());
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.handle_confirmation_response(ConfirmationResponse::Cancel);

    assert_eq!(io.save_dialog_calls(), 0);
    assert_eq!(io.save_calls(), 0);
    assert_eq!(io.open_dialog_calls(), 0);
    assert_eq!(io.load_calls(), 0);
    assert_eq!(
        app.state.schematic.components[0].kind,
        ComponentType::Resistor
    );
    assert!(app.state.schematic.is_dirty);
}

#[test]
fn test_confirmation_yes_file_open_save_as_error_does_not_open() {
    use crate::state::ComponentType;

    let mut app = make_test_app();
    app.state.schematic = schematic_with_component(ComponentType::Resistor, 11, 11);
    app.state.schematic.is_dirty = true;
    app.state.schematic.current_file = None;
    app.state
        .dialogs
        .confirmation_dialog
        .show(ConfirmationAction::FileOpen);

    let io = Rc::new(MockAppFileWorkflowIo::default());
    io.push_save_dialog_result(Err(crate::io::SchematicIoError::Io(
        "save-dialog-failure".to_string(),
    )));
    app.set_file_workflow_io_for_test(Box::new(io.clone()));

    app.handle_confirmation_response(ConfirmationResponse::Yes);

    assert_eq!(io.save_dialog_calls(), 1);
    assert_eq!(io.save_calls(), 0);
    assert_eq!(io.open_dialog_calls(), 0);
    assert_eq!(io.load_calls(), 0);
    assert_eq!(
        app.state.schematic.components[0].kind,
        ComponentType::Resistor
    );
    assert!(app.state.schematic.is_dirty);
    assert!(
        app.state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Save As failed")),
        "save-as dialog failure should be surfaced to the user"
    );
}
