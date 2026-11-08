//! The operating-point inspector's refusals over a whole application.

use rspice_results_ui::op_inspector::OpAction;

use crate::workbench::AppState;
use crate::workbench::documents::result_document::op_inspector::apply_action;

/// A locate that cannot be made is a refused gesture: the status bar
/// says why, and nothing is toasted.
#[test]
fn a_device_that_cannot_be_located_is_answered_on_the_status_bar() {
    let mut state = AppState::default();
    apply_action(&mut state.results_parts(), OpAction::LocateDevice(u64::MAX));
    state.drain_notices(&egui::Context::default());
    assert_eq!(
        state.prompt_text(),
        Some("The retained device no longer resolves to the active schematic.")
    );
    assert!(state.workbench.toasts.activity().is_empty());
}

/// A double-click the schematic cannot answer gets its reason on the same
/// line, and leaves the workspace where it was.
#[test]
fn a_refused_cross_probe_is_answered_on_the_status_bar() {
    let mut state = AppState::default();
    let workspace = state.workbench.workspace;
    apply_action(
        &mut state.results_parts(),
        OpAction::Refused("This retained node has no schematic mapping."),
    );
    state.drain_notices(&egui::Context::default());
    assert_eq!(
        state.prompt_text(),
        Some("This retained node has no schematic mapping.")
    );
    assert_eq!(state.workbench.workspace, workspace);
    assert!(state.workbench.toasts.activity().is_empty());
}
