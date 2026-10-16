//! Design rule setup: the project's rule deck, one row per rule.
//!
//! The deck is project state — which findings block a run is a decision the
//! team shares and the project carries — so it is edited here, beside the
//! connectivity contract, and not in Preferences, which are the reader's own.
//! Each row states the rule's code and default beside the severity this
//! project sets, so the deck can be audited against what RSpice ships; a
//! severity below the rule's floor is shown and refused, with the reason.

use egui::{Context, Grid, RichText, ScrollArea, Ui};
use rspice_design::drc::{DeckIssue, DesignRuleDeck, DrcRuleCategory, DrcSeverity, DrcViolationType};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, DialogTransactionTone, select_with_disabled};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const TITLE: &str = "Design rule setup";
const PRIMARY: &str = "Apply";
const DESCRIPTION: &str = "The severity each design rule is reported at in this project, and \
    whether a finding of it may be waived. Saved with the project.";

/// Why an option below a rule's floor cannot be chosen.
const REFUSES_THE_RUN: &str = "A finding of this rule refuses the run whatever it is reported as";
const INVENTED_VALUE: &str = "The run would simulate a value nobody entered";

/// The draft deck the dialog edits, and why the last edit was refused.
#[derive(Debug, Clone, Default)]
pub(crate) struct RuleSetupDialogState {
    pub(crate) open: bool,
    draft: DesignRuleDeck,
    refusal: Option<String>,
}

pub(crate) fn open_rule_setup(state: &mut AppState) {
    if !state.project_lifecycle.is_open() {
        state.push_user_message(ConsoleMessage::warning(
            "Design rule setup requires an open project.",
        ));
        return;
    }
    state.dialogs.rule_setup = RuleSetupDialogState {
        open: true,
        draft: state.workspace.content.design_rules.clone(),
        refusal: None,
    };
}

/// The issue that keeps the whole stored deck from being read, if any.
fn unread_issue(deck: &DesignRuleDeck) -> Option<DeckIssue> {
    deck.issues().into_iter().find(|issue| {
        matches!(
            issue,
            DeckIssue::UnsupportedSchema { .. } | DeckIssue::Unreadable { .. }
        )
    })
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_rule_setup_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.rule_setup.open {
            return;
        }
        let read_only = self.state.workbench.safe_mode.project_read_only();
        let unread = unread_issue(&self.state.dialogs.rule_setup.draft);
        let editable = !read_only && unread.is_none();
        let changed = self.state.dialogs.rule_setup.draft != self.state.workspace.content.design_rules;
        let mut dialog = Dialog::new(TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Manager)
            .ghost("Close")
            .primary_enabled(editable && changed);
        if let Some(issue) = &unread {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "The stored deck is not applied",
                issue.to_string(),
            );
        } else if read_only {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The project is read-only.",
            );
        } else if let Some(refusal) = self.state.dialogs.rule_setup.refusal.clone() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Severity refused",
                refusal,
            );
        }
        let session = &mut self.state.dialogs.rule_setup;
        let choice = dialog.show(ctx, |ui| rule_table(ui, session, editable));
        match choice {
            DialogChoice::Primary => self.apply_rule_setup(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.rule_setup = RuleSetupDialogState::default();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// Make the draft the project's deck. The reports produced under the old
    /// one go stale through their digests, and preflight authority is dropped,
    /// because what blocks a run has changed.
    fn apply_rule_setup(&mut self) {
        let draft = self.state.dialogs.rule_setup.draft.clone();
        self.state.workspace.content.design_rules = draft;
        self.state.workspace.content.project_metadata_dirty = true;
        self.invalidate_simulation_preflight();
        self.state.publish_rule_deck_issues();
        self.state.dialogs.rule_setup = RuleSetupDialogState::default();
    }
}

fn rule_table(ui: &mut Ui, session: &mut RuleSetupDialogState, editable: bool) {
    let tokens = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("rule-setup-table")
        .show(ui, |ui| {
            Grid::new("rule-setup-grid")
                .num_columns(5)
                .striped(true)
                .spacing([12.0, 2.0])
                .show(ui, |ui| {
                    for label in ["Rule", "Code", "Default", "Project", "Waivable"] {
                        ui.label(
                            RichText::new(label)
                                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                .color(tokens.color.text_dim),
                        );
                    }
                    ui.end_row();
                    let mut category: Option<DrcRuleCategory> = None;
                    for rule in DrcViolationType::ALL {
                        if category != Some(rule.category()) {
                            category = Some(rule.category());
                            ui.label(
                                RichText::new(rule.category().label())
                                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                    .color(tokens.color.text_faint),
                            );
                            ui.end_row();
                        }
                        rule_row(ui, session, rule, editable);
                    }
                });
        });
}

fn rule_row(ui: &mut Ui, session: &mut RuleSetupDialogState, rule: DrcViolationType, editable: bool) {
    let entry = rule.rule();
    ui.label(entry.title).on_hover_text(entry.explanation);
    ui.monospace(entry.code);
    ui.label(entry.default_severity.display_name());

    let effective = session.draft.effective_severity(rule);
    let decided = session.draft.severity(rule).is_some();
    let mut options = vec![format!("Default ({})", entry.default_severity.display_name())];
    options.extend(DrcSeverity::ALL.iter().map(|severity| severity.display_name().to_owned()));
    let reason = if entry.floor >= DrcSeverity::Error {
        REFUSES_THE_RUN
    } else {
        INVENTED_VALUE
    };
    let disabled = DrcSeverity::ALL
        .iter()
        .enumerate()
        .filter(|(_, severity)| **severity < entry.floor && **severity != entry.default_severity)
        .map(|(index, _)| (index + 1, reason))
        .collect::<Vec<_>>();
    let selected = if decided {
        effective.display_name().to_owned()
    } else {
        options[0].clone()
    };
    ui.add_enabled_ui(editable, |ui| {
        let picked = select_with_disabled(
            ui,
            &format!("rule-setup.{}", entry.code),
            &format!("Project severity for {}", entry.title),
            &selected,
            &options,
            &disabled,
            120.0,
        );
        if let Some(index) = picked {
            let result = match index {
                0 => session.draft.clear(rule),
                index => session.draft.set_severity(rule, DrcSeverity::ALL[index - 1]),
            };
            session.refusal = result.err().map(|refusal| refusal.to_string());
        }
    });
    ui.label(if entry.waivable { "Yes" } else { "No" });
    ui.end_row();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dialog_opens_on_the_projects_deck_and_applies_its_draft() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .content
            .design_rules
            .set_severity(DrcViolationType::DanglingWire, DrcSeverity::Error)
            .expect("a dangling wire may be promoted");
        open_rule_setup(&mut app.state);
        assert!(app.state.dialogs.rule_setup.open);
        assert_eq!(
            app.state.dialogs.rule_setup.draft,
            app.state.workspace.content.design_rules
        );

        app.state
            .dialogs
            .rule_setup
            .draft
            .set_severity(DrcViolationType::FloatingNode, DrcSeverity::Info)
            .expect("a floating node may be demoted");
        app.state.workspace.content.project_metadata_dirty = false;
        app.apply_rule_setup();
        assert!(!app.state.dialogs.rule_setup.open);
        assert!(app.state.workspace.content.project_metadata_dirty);
        assert_eq!(
            app.state
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
        app.state.workspace.content.design_rules =
            serde_json::from_str(r#"{"schema_version":9}"#).expect("a stored deck always reads");
        open_rule_setup(&mut app.state);
        assert!(matches!(
            unread_issue(&app.state.dialogs.rule_setup.draft),
            Some(DeckIssue::UnsupportedSchema { version: 9 })
        ));
    }
}
