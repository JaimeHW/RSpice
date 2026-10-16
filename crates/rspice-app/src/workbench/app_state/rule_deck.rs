//! What the project's stored rule deck holds that this build did not apply.
//!
//! A deck written by a newer build can name a rule this one does not have, set
//! a severity it cannot read, or be written under a schema it does not read at
//! all. The design crate keeps all of it, so a save writes it back; this puts
//! each such thing on the Problems page, once, naming the code and the project
//! file, so a team does not sign off under a deck it believes is applied.

use std::sync::Arc;

use rspice_design::drc::DeckIssue;

use super::AppState;
use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};
use crate::product::ContentDigest;
use crate::workbench::state::PreflightRemediation;

/// Producer identity every rule-deck record carries.
const RULE_DECK_PRODUCER: &str = "rspice.design.rule-deck";

impl AppState {
    /// Publish what the stored deck could not apply, replacing what was
    /// published for it before. Asked when a project is opened and whenever
    /// the deck is edited.
    pub(crate) fn publish_rule_deck_issues(&mut self) {
        let records = rule_deck_records(self);
        // One record per kept code and at most one for the whole deck, so the
        // batch never mints one identity twice; the digest is over the deck
        // alone. Neither refusal the store has can occur.
        if let Err(error) = self.publish_diagnostics(DiagnosticInputs::RuleDeck, records) {
            debug_assert!(false, "the rule deck's records were refused: {error}");
        }
    }
}

/// The digest of what the deck could not apply, as it stands now.
///
/// Over the issues rather than the deck: deciding a rule this build reads
/// does not change what it could not read, and must not stale the records
/// saying so.
pub(super) fn live_input_digest(state: &AppState) -> Result<ContentDigest, String> {
    let issues = state
        .workspace
        .content
        .design_rules
        .issues()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    super::diagnostic_store::digest_of(&(
        state.workspace.content.project.id(),
        project_file_label(state),
        issues,
    ))
}

/// The project file the deck is stored in, as a reader finds it on disk.
fn project_file_label(state: &AppState) -> String {
    let project = &state.workspace.content.project;
    project
        .path
        .as_deref()
        .and_then(std::path::Path::file_name)
        .map_or_else(
            || format!("{} (unsaved)", project.display_name()),
            |name| name.to_string_lossy().into_owned(),
        )
}

fn rule_deck_records(state: &AppState) -> Vec<DiagnosticRecord> {
    let issues = state.workspace.content.design_rules.issues();
    let project = state.workspace.content.project.id();
    let revision = state.workspace.content.project.revision().get();
    let file = project_file_label(state);
    let document: Arc<str> = Arc::from(file.as_str());
    let keys = issues
        .iter()
        .map(|issue| {
            let (code, object) = issue_identity(issue);
            FindingKey::new(RULE_DECK_PRODUCER, code, Arc::clone(&document), object)
        })
        .collect::<Vec<_>>();
    let canonicals = mint_canonical(&keys, revision, "rule-deck");
    issues
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((issue, key), canonical)| {
            let object = match issue {
                DeckIssue::UnknownRule { code } | DeckIssue::UnreadableSeverity { code, .. } => {
                    format!("Rule deck \u{00b7} {code}")
                }
                DeckIssue::BelowFloor { rule, .. } => {
                    format!("Rule deck \u{00b7} {}", rule.rule_code())
                }
                DeckIssue::UnsupportedSchema { .. } | DeckIssue::Unreadable { .. } => {
                    "Rule deck".to_owned()
                }
            };
            DiagnosticRecord::new(
                canonical,
                issue_level(issue),
                DiagnosticSubject::new(project, object),
                format!("{file}: {issue}."),
            )
            .keyed(key)
            .with_route(Some(DiagnosticRoute::Repair(
                PreflightRemediation::RuleSetup,
            )))
        })
        .collect()
}

/// The code a deck record is published under, and the object inside the deck
/// it is about.
fn issue_identity(issue: &DeckIssue) -> (&'static str, String) {
    match issue {
        DeckIssue::UnknownRule { code } => ("RULE-DECK-UNKNOWN-RULE", code.clone()),
        DeckIssue::UnreadableSeverity { code, .. } => {
            ("RULE-DECK-UNREADABLE-SEVERITY", code.clone())
        }
        DeckIssue::BelowFloor { rule, .. } => {
            ("RULE-DECK-BELOW-FLOOR", rule.rule_code().to_owned())
        }
        DeckIssue::UnsupportedSchema { .. } => ("RULE-DECK-UNSUPPORTED-SCHEMA", String::new()),
        DeckIssue::Unreadable { .. } => ("RULE-DECK-UNREADABLE", String::new()),
    }
}

/// How much each issue stands against a sign-off.
///
/// A deck that is not applied, or a decided rule checked at its default,
/// means the design is checked under a policy the team did not write: a rule
/// they promoted to a blocker no longer blocks. A row naming a rule this build
/// does not have changes nothing this build checks, and a row clamped to its
/// rule's floor is checked more strictly than written, which hides nothing:
/// every finding it affects says what it is reported at.
const fn issue_level(issue: &DeckIssue) -> DiagnosticLevel {
    match issue {
        DeckIssue::UnknownRule { .. } | DeckIssue::BelowFloor { .. } => DiagnosticLevel::Info,
        DeckIssue::UnreadableSeverity { .. }
        | DeckIssue::UnsupportedSchema { .. }
        | DeckIssue::Unreadable { .. } => DiagnosticLevel::Warning,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck_records(state: &AppState) -> Vec<DiagnosticRecord> {
        state
            .diagnostics
            .batch(&DiagnosticInputs::RuleDeck)
            .map(|batch| batch.records.clone())
            .unwrap_or_default()
    }

    #[test]
    fn a_deck_this_build_reads_in_full_publishes_nothing() {
        let mut state = AppState::default();
        state.workspace.content.design_rules = serde_json::from_str(
            r#"{"schema_version":1,"severities":{"DRC-DANGLING-WIRE":"Error"}}"#,
        )
        .expect("a stored deck always reads");
        state.publish_rule_deck_issues();
        assert!(deck_records(&state).is_empty());
    }

    /// An unknown code is reported on the page, naming the code and the
    /// project file, rather than silently dropped.
    #[test]
    fn an_unknown_rule_code_is_reported_with_the_project_file() {
        let mut state = AppState::default();
        state.workspace.content.project.path = Some("designs/amp.rspice".into());
        state.workspace.content.design_rules = serde_json::from_str(
            r#"{"schema_version":1,"severities":{"DRC-FUTURE-RULE":"Error"}}"#,
        )
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
        state.workspace.content.project.path = Some("designs/amp.rspice".into());
        state.workspace.content.design_rules = serde_json::from_str(
            r#"{"schema_version":1,"severities":{"DRC-DUPLICATE-NAME":"Info"}}"#,
        )
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
                .workspace
                .content
                .design_rules
                .severity(rspice_design::drc::DrcViolationType::DuplicateName),
            Some(rspice_design::drc::DrcSeverity::Error)
        );
    }

    /// A deck from a newer schema is not applied, and the page says so in one
    /// record.
    #[test]
    fn a_deck_from_a_newer_schema_is_one_record() {
        let mut state = AppState::default();
        state.workspace.content.design_rules = serde_json::from_str(
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
                .workspace
                .content
                .design_rules
                .severity_overrides()
                .is_empty(),
            "nothing in it is applied"
        );
    }
}
