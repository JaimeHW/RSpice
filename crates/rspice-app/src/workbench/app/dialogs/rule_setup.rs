//! Design rule setup: the project's rule deck, one row per rule.
//!
//! The deck is project state — which findings block a run is a decision the
//! team shares and the project carries — so it is edited here, beside the
//! connectivity contract, and not in Preferences, which are the reader's own.
//! Each row states the rule's code and default beside the severity this
//! project sets, so the deck can be audited against what RSpice ships; a
//! severity below the rule's floor is shown and refused, with the reason.

use egui::{Context, Grid, RichText, ScrollArea, Ui};
use rspice_design::drc::{
    DeckIssue, DesignRuleDeck, DrcRuleCategory, DrcSeverity, DrcViolationType,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogSize, DialogTransactionTone, select_with_disabled,
};
use crate::workbench::app_state::AppState;

const TITLE: &str = "Design rule setup";
const PRIMARY: &str = "Apply";
const DESCRIPTION: &str = "The severity each design rule is reported at in this project, and \
    whether a finding of it may be waived. Saved with the project.";

/// Why an option below a rule's floor cannot be chosen.
const REFUSES_THE_RUN: &str = "A finding of this rule refuses the run whatever it is reported as";
const INVENTED_VALUE: &str = "The run would simulate a value nobody entered";
/// Why an option above a rule's ceiling cannot be chosen.
const NEVER_BLOCKS: &str = "This rule is about the project's libraries and never blocks a run";

/// The draft deck the dialog edits, and why the last edit was refused.
#[derive(Debug, Clone, Default)]
pub(crate) struct RuleSetupDialogState {
    pub(crate) open: bool,
    draft: DesignRuleDeck,
    refusal: Option<String>,
}

pub(crate) fn open_rule_setup(state: &mut AppState) {
    if !state.project_lifecycle.is_open() {
        state.prompt("Design rule setup requires an open project.");
        return;
    }
    state.dialogs.rule_setup = RuleSetupDialogState {
        open: true,
        draft: (*state.workspace.content.design_rules).clone(),
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

/// Draw the dialog while it is open. Returns whether the draft was applied,
/// so the caller drops any run prepared under the old deck.
pub(in crate::workbench) fn render_rule_setup_dialog(ctx: &Context, state: &mut AppState) -> bool {
    if !state.dialogs.rule_setup.open {
        return false;
    }
    let read_only = state.workbench.safe_mode.project_read_only();
    let unread = unread_issue(&state.dialogs.rule_setup.draft);
    let editable = !read_only && unread.is_none();
    let changed = state.dialogs.rule_setup.draft != *state.workspace.content.design_rules;
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
    } else if let Some(refusal) = state.dialogs.rule_setup.refusal.clone() {
        dialog =
            dialog.transaction_state(DialogTransactionTone::Error, "Severity refused", refusal);
    }
    let session = &mut state.dialogs.rule_setup;
    match dialog.show(ctx, |ui| rule_table(ui, session, editable)) {
        DialogChoice::Primary => {
            apply_rule_setup(state);
            true
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.dialogs.rule_setup = RuleSetupDialogState::default();
            false
        }
        DialogChoice::None | DialogChoice::Secondary => false,
    }
}

/// Make the draft the project's deck. The reports produced under the old one
/// go stale through their digests, and preflight authority is dropped,
/// because what blocks a run has changed.
fn apply_rule_setup(state: &mut AppState) {
    let draft = state.dialogs.rule_setup.draft.clone();
    *state.workspace.content.design_rules = draft;
    state.workspace.content.project_metadata_dirty = true;
    state.workbench.preflight.invalidate();
    state.publish_rule_deck_issues();
    state.dialogs.rule_setup = RuleSetupDialogState::default();
}

fn rule_table(ui: &mut Ui, session: &mut RuleSetupDialogState, editable: bool) {
    let tokens = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("rule-setup-table")
        .show(ui, |ui| {
            Grid::new("rule-setup-grid")
                .num_columns(6)
                .striped(true)
                .spacing([12.0, 2.0])
                .show(ui, |ui| {
                    for label in ["Rule", "Code", "Default", "Project", "Waivable", "Floor"] {
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

fn rule_row(
    ui: &mut Ui,
    session: &mut RuleSetupDialogState,
    rule: DrcViolationType,
    editable: bool,
) {
    let entry = rule.rule();
    ui.label(entry.title).on_hover_text(entry.explanation);
    ui.monospace(entry.code);
    ui.label(entry.default_severity.display_name());

    let effective = session.draft.effective_severity(rule);
    let decided = session.draft.severity(rule).is_some();
    let mut options = vec![format!(
        "Default ({})",
        entry.default_severity.display_name()
    )];
    options.extend(
        DrcSeverity::ALL
            .iter()
            .map(|severity| severity.display_name().to_owned()),
    );
    let disabled = refused_options(rule);
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
                index => session
                    .draft
                    .set_severity(rule, DrcSeverity::ALL[index - 1]),
            };
            session.refusal = result.err().map(|refusal| refusal.to_string());
        }
    });
    ui.label(if entry.waivable { "Yes" } else { "No" });
    ui.label(floor_cell(rule));
    ui.end_row();
}

/// The lowest the rule may be set, and, where a finding on an instance the
/// deck cannot write is held higher whatever is set, that floor beside it.
fn floor_cell(rule: DrcViolationType) -> String {
    let floor = rule.rule().floor;
    if floor.unwritten_instance > floor.deck {
        format!(
            "{} \u{00b7} {} if unwritten",
            floor.deck.display_name(),
            floor.unwritten_instance.display_name()
        )
    } else {
        floor.deck.display_name().to_owned()
    }
}

/// The project-severity options a rule refuses, by their place in the select
/// (the default is option 0), each with the reason it is refused.
fn refused_options(rule: DrcViolationType) -> Vec<(usize, &'static str)> {
    let floor = rule.rule().floor.deck;
    let ceiling = rule.rule().max_severity;
    let below = if floor >= DrcSeverity::Error {
        REFUSES_THE_RUN
    } else {
        INVENTED_VALUE
    };
    DrcSeverity::ALL
        .iter()
        .enumerate()
        .filter_map(|(index, severity)| {
            let reason = if *severity < floor {
                below
            } else if *severity > ceiling {
                NEVER_BLOCKS
            } else {
                return None;
            };
            Some((index + 1, reason))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app::RSpiceApp;

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
            *app.state.workspace.content.design_rules
        );

        app.state
            .dialogs
            .rule_setup
            .draft
            .set_severity(DrcViolationType::FloatingNode, DrcSeverity::Info)
            .expect("a floating node may be demoted");
        app.state.workspace.content.project_metadata_dirty = false;
        apply_rule_setup(&mut app.state);
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

    /// A severity below a rule's floor is offered, so the reader sees the
    /// whole range, but cannot be picked, and says why.
    #[test]
    fn a_severity_below_the_floor_is_refused_with_the_reason() {
        assert_eq!(
            refused_options(DrcViolationType::DuplicateName),
            vec![(1, REFUSES_THE_RUN), (2, REFUSES_THE_RUN)]
        );
        assert_eq!(
            refused_options(DrcViolationType::MissingParameter),
            vec![(1, INVENTED_VALUE)]
        );
        assert!(refused_options(DrcViolationType::DanglingWire).is_empty());
        // An unused cellview is a note about the libraries: nothing above a
        // warning may be picked for it.
        assert_eq!(
            refused_options(DrcViolationType::UnusedCellView),
            vec![(3, NEVER_BLOCKS), (4, NEVER_BLOCKS)]
        );
        // An empty reference may be set to anything; the instance the deck
        // cannot write is held at an error by the checker, and the floor
        // column says so.
        assert!(refused_options(DrcViolationType::EmptyName).is_empty());
        assert_eq!(
            floor_cell(DrcViolationType::EmptyName),
            "Info \u{00b7} Error if unwritten"
        );
        assert_eq!(floor_cell(DrcViolationType::DuplicateName), "Error");
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
