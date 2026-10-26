//! Clearing the design's execution context reaches past the results: the
//! code workspace's runtime goes, and the Console keeps its history.

use crate::diagnostics::LogSource;
use crate::workbench::app_state::AppState;

#[test]
fn clearing_design_execution_context_clears_source_runtime_and_keeps_the_console() {
    let mut state = AppState::default();
    state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
    state.code.code_workspace.automation.debug.watches.push(
        crate::workbench::documents::code_workspace::AutomationWatch {
            expression: "project.old_value".to_owned(),
            value: "17".to_owned(),
            error: None,
        },
    );
    state.code.netlist.diagnostics = std::sync::Arc::new(
        crate::state::netlist_diagnostics::NetlistDiagnosticCollection::try_new(
            vec![crate::state::netlist_diagnostics::Diagnostic::error(
                "old project diagnostic",
            )],
            "",
        )
        .unwrap(),
    );
    state
        .model
        .log_buffer
        .warning(LogSource::Run, "old project warning");
    state.clear_design_execution_context();

    assert_eq!(
        state.code.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
    assert!(
        state
            .code
            .code_workspace
            .automation
            .debug
            .watches
            .is_empty()
    );
    assert_eq!(state.code.netlist.diagnostics.summary().total(), 0);
    assert!(
        state
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message == "old project warning"),
        "the Console is the session's record and keeps its history"
    );
}
