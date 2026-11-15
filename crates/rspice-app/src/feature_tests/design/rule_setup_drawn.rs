//! The rule setup dialog drawn through a whole application.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::rule_setup::*;
use rspice_design::drc::DeckIssue;
use rspice_design::drc::DrcSeverity;
use rspice_design::drc::DrcViolationType;

/// Apply the draft as the dialog's Apply does, over the whole state.
fn apply_rule_setup(state: &mut AppState) {
    let review_markers = state.design.schematic_visibility.review_markers;
    rspice_app_design::workbench::app::dialogs::rule_setup::apply_rule_setup(
        &mut state.design.rule_setup,
        review_markers,
        &mut state.model,
        &mut state.runs,
    );
}

#[test]
fn the_dialog_opens_on_the_projects_deck_and_applies_its_draft() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .workspace
        .content
        .design_rules
        .set_severity(DrcViolationType::DanglingWire, DrcSeverity::Error)
        .expect("a dangling wire may be promoted");
    open_rule_setup(
        &mut app.state.design.rule_setup,
        &app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    assert!(app.state.design.rule_setup.open);
    assert_eq!(
        app.state.design.rule_setup.draft,
        *app.state.model.workspace.content.design_rules
    );

    app.state
        .design
        .rule_setup
        .draft
        .set_severity(DrcViolationType::FloatingNode, DrcSeverity::Info)
        .expect("a floating node may be demoted");
    app.state.model.workspace.content.project_metadata_dirty = false;
    apply_rule_setup(&mut app.state);
    assert!(!app.state.design.rule_setup.open);
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .design_rules
            .severity(DrcViolationType::FloatingNode),
        Some(DrcSeverity::Info)
    );
}

#[test]
fn a_deck_this_build_cannot_read_opens_for_inspection_only() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.workspace.content.design_rules =
        serde_json::from_str(r#"{"schema_version":9}"#).expect("a stored deck always reads");
    open_rule_setup(
        &mut app.state.design.rule_setup,
        &app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    assert!(matches!(
        unread_issue(&app.state.design.rule_setup.draft),
        Some(DeckIssue::UnsupportedSchema { version: 9 })
    ));
}
