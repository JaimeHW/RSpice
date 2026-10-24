//! `workbench/app_state/rule_deck` observed through a whole session.

use crate::state::PreflightRemediation;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::app_state::diagnostic_store::DiagnosticRecord;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;

fn deck_records(state: &AppState) -> Vec<DiagnosticRecord> {
    state
        .model
        .diagnostics
        .batch(&DiagnosticInputs::RuleDeck)
        .map(|batch| batch.records.clone())
        .unwrap_or_default()
}

#[test]
fn a_deck_this_build_reads_in_full_publishes_nothing() {
    let mut state = AppState::default();
    state.model.workspace.content.design_rules =
        serde_json::from_str(r#"{"schema_version":1,"severities":{"DRC-DANGLING-WIRE":"Error"}}"#)
            .expect("a stored deck always reads");
    state.publish_rule_deck_issues();
    assert!(deck_records(&state).is_empty());
}

/// An unknown code is reported on the page, naming the code and the
/// project file, rather than silently dropped.
#[test]
fn an_unknown_rule_code_is_reported_with_the_project_file() {
    let mut state = AppState::default();
    state.model.workspace.content.project.path = Some("designs/amp.rspice".into());
    state.model.workspace.content.design_rules =
        serde_json::from_str(r#"{"schema_version":1,"severities":{"DRC-FUTURE-RULE":"Error"}}"#)
            .expect("a stored deck always reads");
    state.publish_rule_deck_issues();
    let records = deck_records(&state);
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(&*record.canonical.code, "RULE-DECK-UNKNOWN-RULE");
    assert!(
        record.message.starts_with("amp.rspice: "),
        "{}",
        record.message
    );
    assert!(
        record.message.contains("DRC-FUTURE-RULE"),
        "{}",
        record.message
    );
    assert_eq!(record.level, DiagnosticLevel::Info);
    assert_eq!(
        record.route,
        Some(DiagnosticRoute::Repair(PreflightRemediation::RuleSetup))
    );
}

/// A stored row below its rule's floor is applied at the floor and
/// reported, naming the rule's code and the project file.
#[test]
fn a_row_below_the_floor_is_reported_as_clamped() {
    let mut state = AppState::default();
    state.model.workspace.content.project.path = Some("designs/amp.rspice".into());
    state.model.workspace.content.design_rules =
        serde_json::from_str(r#"{"schema_version":1,"severities":{"DRC-DUPLICATE-NAME":"Info"}}"#)
            .expect("a stored deck always reads");
    state.publish_rule_deck_issues();
    let records = deck_records(&state);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(&*records[0].canonical.code, "RULE-DECK-BELOW-FLOOR");
    assert_eq!(records[0].level, DiagnosticLevel::Info);
    assert!(
        records[0].message.starts_with("amp.rspice: "),
        "{}",
        records[0].message
    );
    assert!(
        records[0].message.contains("DRC-DUPLICATE-NAME"),
        "{}",
        records[0].message
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .design_rules
            .severity(rspice_design::drc::DrcViolationType::DuplicateName),
        Some(rspice_design::drc::DrcSeverity::Error)
    );
}

/// A stored row that promotes an unused cellview to an error is applied
/// as a warning, and the page says so in one record.
#[test]
fn a_row_above_the_ceiling_is_reported_as_clamped() {
    let mut state = AppState::default();
    state.model.workspace.content.project.path = Some("designs/amp.rspice".into());
    state.model.workspace.content.design_rules = serde_json::from_str(
        r#"{"schema_version":1,"severities":{"DRC-UNUSED-CELLVIEW":"Error"}}"#,
    )
    .expect("a stored deck always reads");
    state.publish_rule_deck_issues();
    state.publish_rule_deck_issues();
    let records = deck_records(&state);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(&*records[0].canonical.code, "RULE-DECK-ABOVE-CEILING");
    assert_eq!(records[0].level, DiagnosticLevel::Warning);
    assert!(
        records[0].message.starts_with("amp.rspice: ")
            && records[0].message.contains("DRC-UNUSED-CELLVIEW"),
        "{}",
        records[0].message
    );
    assert_eq!(
        state
            .model
            .workspace
            .content
            .design_rules
            .severity(rspice_design::drc::DrcViolationType::UnusedCellView),
        Some(rspice_design::drc::DrcSeverity::Warning)
    );
}

/// A deck from a newer schema is not applied, and the page says so in one
/// record.
#[test]
fn a_deck_from_a_newer_schema_is_one_record() {
    let mut state = AppState::default();
    state.model.workspace.content.design_rules = serde_json::from_str(
        r#"{"schema_version":3,"severities":{"DRC-DANGLING-WIRE":"Error","DRC-X":"Info"}}"#,
    )
    .expect("a stored deck always reads");
    state.publish_rule_deck_issues();
    let records = deck_records(&state);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(&*records[0].canonical.code, "RULE-DECK-UNSUPPORTED-SCHEMA");
    assert_eq!(records[0].level, DiagnosticLevel::Warning);
    assert!(records[0].message.contains("deck schema 3"));
    assert!(
        state
            .model
            .workspace
            .content
            .design_rules
            .severity_overrides()
            .is_empty(),
        "nothing in it is applied"
    );
}
