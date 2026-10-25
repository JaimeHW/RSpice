//! What the project's stored rule deck holds that this build did not apply.
//!
//! A deck written by a newer build can name a rule this one does not have, set
//! a severity it cannot read, or be written under a schema it does not read at
//! all. The design crate keeps all of it, so a save writes it back; this puts
//! each such thing on the Problems page, once, naming the code and the project
//! file, so a team does not sign off under a deck it believes is applied.

use std::sync::Arc;

use rspice_design::drc::DeckIssue;

use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};
use crate::product::ContentDigest;
use crate::state::PreflightRemediation;
use crate::workbench::app_state::model::{Model, SlotFacts};

/// Producer identity every rule-deck record carries.
const RULE_DECK_PRODUCER: &str = "rspice.design.rule-deck";

impl Model {
    /// Publish what the stored deck could not apply, replacing what was
    /// published for it before. Asked when a project is opened and whenever
    /// the deck is edited.
    pub fn publish_rule_deck_issues(&mut self, facts: SlotFacts) {
        let records = rule_deck_records(self);
        // One record per kept code and at most one for the whole deck, so the
        // batch never mints one identity twice; the digest is over the deck
        // alone. Neither refusal the store has can occur.
        if let Err(error) = self.publish_diagnostics(DiagnosticInputs::RuleDeck, records, facts) {
            debug_assert!(false, "the rule deck's records were refused: {error}");
        }
    }
}

/// The digest of what the deck could not apply, as it stands now.
///
/// Over the issues rather than the deck: deciding a rule this build reads
/// does not change what it could not read, and must not stale the records
/// saying so.
pub(super) fn live_input_digest(model: &Model) -> Result<ContentDigest, String> {
    let issues = model
        .workspace
        .content
        .design_rules
        .issues()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    super::diagnostic_store::digest_of(&(
        model.workspace.content.project.id(),
        project_file_label(model),
        issues,
    ))
}

/// The project file the deck is stored in, as a reader finds it on disk.
pub(super) fn project_file_label(model: &Model) -> String {
    let project = &model.workspace.content.project;
    project
        .path
        .as_deref()
        .and_then(std::path::Path::file_name)
        .map_or_else(
            || format!("{} (unsaved)", project.display_name()),
            |name| name.to_string_lossy().into_owned(),
        )
}

fn rule_deck_records(model: &Model) -> Vec<DiagnosticRecord> {
    let issues = model.workspace.content.design_rules.issues();
    let project = model.workspace.content.project.id();
    let revision = model.workspace.content.project.revision().get();
    let file = project_file_label(model);
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
                DeckIssue::BelowFloor { rule, .. } | DeckIssue::AboveCeiling { rule, .. } => {
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
        DeckIssue::AboveCeiling { rule, .. } => {
            ("RULE-DECK-ABOVE-CEILING", rule.rule_code().to_owned())
        }
        DeckIssue::UnsupportedSchema { .. } => ("RULE-DECK-UNSUPPORTED-SCHEMA", String::new()),
        DeckIssue::Unreadable { .. } => ("RULE-DECK-UNREADABLE", String::new()),
    }
}

/// How much each issue stands against a sign-off.
///
/// A deck that is not applied, a decided rule checked at its default, or a
/// row clamped to its rule's ceiling, means the design is checked under a
/// policy the team did not write: a rule they promoted to a blocker does not
/// block. A row naming a rule this build does not have changes nothing this
/// build checks, and a row clamped to its rule's floor is checked more
/// strictly than written, which hides nothing: every finding it affects says
/// what it is reported at.
const fn issue_level(issue: &DeckIssue) -> DiagnosticLevel {
    match issue {
        DeckIssue::UnknownRule { .. } | DeckIssue::BelowFloor { .. } => DiagnosticLevel::Info,
        DeckIssue::AboveCeiling { .. }
        | DeckIssue::UnreadableSeverity { .. }
        | DeckIssue::UnsupportedSchema { .. }
        | DeckIssue::Unreadable { .. } => DiagnosticLevel::Warning,
    }
}
