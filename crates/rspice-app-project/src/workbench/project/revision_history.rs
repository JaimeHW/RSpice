//! Project revision and audit history backed exclusively by persisted owners.
//!
//! Validated schematic revisions own exact restorable snapshots. Simulation
//! plans, model/PDK bindings, and governed design-management receipts expose
//! their retained identities and metadata without inventing missing actors,
//! timestamps, or restoration authority.

use std::sync::Arc;

use egui::{Color32, Context, Frame, Grid, RichText, Stroke, TextEdit, Ui};

use crate::product::ContentDigest;
use crate::state::model_library::ModelSourceAuthority;
use crate::state::{
    CellViewRef, ValidatedRevisionJournal, ValidatedRevisionObjectDelta,
    ValidatedRevisionSemanticDelta, ValidatedSchematicRevision, ValidatedSchematicRevisionId,
    ViewType,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, LineHeights, RowKey, SortKey, TABLE_EDGE_INSET,
};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
    text_field,
};
use crate::workbench::design_system::section_header;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::project_revision_history::{
    PendingRestore, ProjectAuditArtifactKind, ProjectAuditComparison, ProjectAuditRow,
    ProjectAuditRowsSource, ProjectRevisionArtifactFilter, ProjectRevisionComparison,
    ProjectRevisionHistoryDialogState, ShownRows,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

const TITLE: &str = "Project revision and audit history";
/// A fact the record's owner does not keep: left blank, as an empty cell,
/// rather than stated in every row of a column that has nothing to say.
const NOT_RETAINED: &str = "";

impl ProjectAuditRow {
    fn matches_query(&self, query: &str) -> bool {
        query.is_empty()
            || [
                self.kind.label(),
                &self.revision,
                &self.time,
                &self.actor,
                &self.change,
                &self.affected_evidence,
                &self.status,
            ]
            .iter()
            .any(|value| value.to_ascii_lowercase().contains(query))
    }
}

/// The retained metadata two records of one family differ in.
impl ProjectAuditComparison {
    fn between(first: &ProjectAuditRow, second: &ProjectAuditRow) -> Self {
        let mut fields = Vec::new();
        comparison_field(&mut fields, "Revision", &first.revision, &second.revision);
        comparison_field(&mut fields, "Actor", &first.actor, &second.actor);
        comparison_field(&mut fields, "Change", &first.change, &second.change);
        comparison_field(
            &mut fields,
            "Affected records",
            &first.affected_evidence,
            &second.affected_evidence,
        );
        comparison_field(&mut fields, "Status", &first.status, &second.status);
        Self {
            label: format!("{} \u{2192} {}", first.revision, second.revision),
            fields,
        }
    }
}

/// What the body asked for this frame, carried out once it has drawn.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum HistoryAction {
    #[default]
    None,
    Compare,
    CompareWithCurrent,
    Restore,
}

pub(crate) fn open_project_revision_history(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    platform: &PlatformState,
) {
    // The command is greyed with no project open.
    if !platform.project_lifecycle.is_open() {
        return;
    }
    if let Err(error) = model.schematic.document().validated_revisions.validate() {
        *dialog = ProjectRevisionHistoryDialogState {
            open: true,
            expected_journal: model.schematic.document().validated_revisions.clone(),
            authority: Some(SchematicEditAuthority::capture(model)),
            error: Some(format!(
                "The schematic's revision history could not be read intact: {error}"
            )),
            ..ProjectRevisionHistoryDialogState::default()
        };
        return;
    }
    *dialog = ProjectRevisionHistoryDialogState {
        open: true,
        authority: Some(SchematicEditAuthority::capture(model)),
        expected_journal: model.schematic.document().validated_revisions.clone(),
        ..ProjectRevisionHistoryDialogState::default()
    };
    if let Some(record) = model
        .schematic
        .document()
        .validated_revisions
        .records()
        .last()
    {
        dialog.stand_on(schematic_row_key(record.id()));
    }
}

/// Open the history standing on the record `key` names, a revision of any
/// schematic view the project keeps; on the newest revision of the
/// schematic in front when the history does not list it.
pub(crate) fn open_project_revision_history_on(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    platform: &PlatformState,
    key: String,
) {
    open_project_revision_history(dialog, model, platform);
    // The rows the history lists: the schematic in front's, then every
    // other view's the project keeps.
    let active = model.workspace.content.active_schematic_reference().key();
    let listed = std::iter::once(&model.schematic.document().validated_revisions)
        .chain(
            model
                .workspace
                .content
                .schematic_buffers
                .iter()
                .filter(|(view, _)| **view != active)
                .map(|(_, schematic)| &schematic.document().validated_revisions),
        )
        .flat_map(ValidatedRevisionJournal::records)
        .any(|record| schematic_row_key(record.id()) == key);
    if listed && dialog.open && dialog.error.is_none() {
        dialog.stand_on(key);
    }
}

impl ProjectFeature {
    /// Draw Project revision history while it is open, and carry out what
    /// its body asked for: a comparison or a restore.
    pub(in crate::workbench) fn render_project_revision_history_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        if !cx.state.project_revision_history.open {
            return;
        }
        finish_pending_restore(cx);
        let rows = kept_audit_rows(&mut cx.state.project_revision_history, cx.model);
        let stale = history_authority_error(&cx.state.project_revision_history, cx.model);
        // A project open read-only restores nothing. A read-only view
        // refuses only its own revisions, which the selection's note says.
        let project_read_only = cx.model.safe_mode.project_read_only();
        let mut body_scroll_offset = cx.state.project_revision_history.body_scroll_offset;
        let mut dialog = Dialog::new(TITLE, "Close")
            .description(
                "Inspect saved project revisions, what changed, and the audit history of schematics, simulations, models, and other records.",
            )
            .size(DialogSize::SimulationWorkflow)
            .initial_height(610.0)
            .overlay_scrollbar()
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        let transaction_error = cx
            .state
            .project_revision_history
            .error
            .clone()
            .or_else(|| stale.clone());
        if let Some(error) = transaction_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Revision history cannot continue",
                error,
            );
        } else if project_read_only {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The active schematic or project is read-only. Filtering and comparison remain available; schematic restore is disabled.",
            );
        }
        // Why nothing can be restored, which the band over the body says.
        let blocked = stale
            .or_else(|| project_read_only.then(|| "The project is open read-only.".to_owned()));

        let mut action = HistoryAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = revision_history_body(
                ui,
                &rows,
                cx.model,
                blocked.as_deref(),
                &mut cx.state.project_revision_history,
            );
            None
        });
        cx.state.project_revision_history.body_scroll_offset = body_scroll_offset;
        handle_revision_history_action(cx, action, &rows);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            cx.state.project_revision_history.close();
        }
    }
}

fn handle_revision_history_action(
    cx: &mut Cx<'_, ProjectFeature>,
    action: HistoryAction,
    rows: &[ProjectAuditRow],
) {
    match action {
        HistoryAction::None => {}
        HistoryAction::Compare => {
            compare_selected_revisions(&mut cx.state.project_revision_history, cx.model, rows);
        }
        HistoryAction::CompareWithCurrent => {
            compare_with_current(&mut cx.state.project_revision_history, cx.model, rows);
        }
        HistoryAction::Restore => restore_selected_revision(cx, rows),
    }
}

/// The selected rows, in the order the table lists them.
fn selected_rows<'a>(
    dialog: &ProjectRevisionHistoryDialogState,
    rows: &'a [ProjectAuditRow],
) -> Vec<&'a ProjectAuditRow> {
    dialog
        .selected()
        .iter()
        .filter_map(|key| rows.iter().find(|row| row.key == *key))
        .collect()
}

fn compare_selected_revisions(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    rows: &[ProjectAuditRow],
) {
    let selected = dialog.selected();
    if selected.len() != 2 {
        dialog.error = Some("Select exactly two retained records to compare.".to_owned());
        return;
    }
    let Some(first) = rows.iter().find(|row| row.key == selected[0]) else {
        dialog.error = Some("The first selected record no longer exists.".to_owned());
        return;
    };
    let Some(second) = rows.iter().find(|row| row.key == selected[1]) else {
        dialog.error = Some("The second selected record no longer exists.".to_owned());
        return;
    };
    if first.kind != second.kind {
        dialog.error = Some(
            "Semantic comparison requires two records from the same project-artifact family."
                .to_owned(),
        );
        return;
    }

    let comparison = match (first.schematic_revision, second.schematic_revision) {
        (Some(first_id), Some(second_id)) => {
            let Some(first_record) = find_project_schematic_revision(model, first_id) else {
                dialog.error =
                    Some("The first selected schematic revision no longer exists.".to_owned());
                return;
            };
            let Some(second_record) = find_project_schematic_revision(model, second_id) else {
                dialog.error =
                    Some("The second selected schematic revision no longer exists.".to_owned());
                return;
            };
            let (older, newer) = if first_record.sequence() <= second_record.sequence() {
                (first_record, second_record)
            } else {
                (second_record, first_record)
            };
            ProjectRevisionComparison::Schematic {
                label: format!(
                    "{} \u{2192} {}",
                    revision_label(older),
                    revision_label(newer)
                ),
                delta: older.semantic_delta_to(newer),
                with_current: false,
            }
        }
        (None, None) => ProjectRevisionComparison::RetainedMetadata(
            ProjectAuditComparison::between(first, second),
        ),
        _ => {
            dialog.error = Some(
                "The selected records do not expose compatible semantic identities.".to_owned(),
            );
            return;
        }
    };
    dialog.comparison = Some(comparison);
    dialog.error = None;
}

/// Compare the one selected schematic revision with its view's drawing as
/// it stands: the open schematic's, or the one the project keeps for a view
/// that is not in front.
fn compare_with_current(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    rows: &[ProjectAuditRow],
) {
    let delta = (|| {
        let [row] = selected_rows(dialog, rows)[..] else {
            return Err("Select one schematic revision to compare with the current design.");
        };
        let (Some(id), Some(view)) = (row.schematic_revision, row.view.as_ref()) else {
            return Err(NOT_A_DRAWING);
        };
        let record = find_project_schematic_revision(model, id)
            .ok_or("The selected schematic revision no longer exists.")?;
        let delta = if *view == model.workspace.content.active_schematic_reference() {
            model
                .schematic
                .editor_ref()
                .design
                .semantic_delta_since(record)
        } else {
            model
                .workspace
                .content
                .schematic_buffers
                .get(&view.key())
                .ok_or("The selected revision's view is no longer in the project.")?
                .semantic_delta_since(record)
        };
        Ok((revision_label(record), delta))
    })();
    match delta {
        Ok((revision, delta)) => {
            dialog.comparison = Some(ProjectRevisionComparison::Schematic {
                label: format!("{revision} \u{2192} current design"),
                delta,
                with_current: true,
            });
            dialog.error = None;
        }
        Err(error) => dialog.error = Some(error.to_owned()),
    }
}

/// Restore the one selected schematic revision: in place when its view is
/// in front, and otherwise once the view it opens is.
fn restore_selected_revision(cx: &mut Cx<'_, ProjectFeature>, rows: &[ProjectAuditRow]) {
    let target = match selected_rows(&cx.state.project_revision_history, rows)[..] {
        [row] => restore_target(row, cx.model),
        _ => Err("Select exactly one validated schematic revision to restore.".to_owned()),
    };
    let result = match target {
        Ok((revision, None)) => restore_in_place(cx, revision).map(|()| true),
        Ok((revision, Some(view))) => {
            // The shell brings the view to the front once this frame's
            // dialogs have drawn; the restore waits for it there.
            cx.wb
                .outbox
                .push(ProjectRequest::OpenView(view.clone(), ViewType::Schematic));
            cx.state.project_revision_history.pending_restore =
                Some(PendingRestore { view, revision });
            Ok(false)
        }
        Err(error) => Err(error),
    };
    settle_restore(cx, result);
}

/// Carry out a restore that opened its view, now that the view is in front.
fn finish_pending_restore(cx: &mut Cx<'_, ProjectFeature>) {
    let Some(pending) = cx.state.project_revision_history.pending_restore.take() else {
        return;
    };
    if cx.model.workspace.content.active_view != pending.view {
        cx.state.project_revision_history.error = Some(format!(
            "{} did not open, so its revision was not restored.",
            pending.view.display_path()
        ));
        return;
    }
    // The history stands on the schematic the restore opened.
    let authority = SchematicEditAuthority::capture(&*cx.model);
    let dialog = &mut cx.state.project_revision_history;
    dialog.authority = Some(authority);
    dialog.expected_journal = cx.model.schematic.document().validated_revisions.clone();
    dialog.rows_source = None;
    let result = restore_in_place(cx, pending.revision).map(|()| true);
    settle_restore(cx, result);
}

/// The dialog after a restore was asked for: re-armed on the journal it now
/// shows when the revision was put back, waiting when its view is opening,
/// and stating the refusal otherwise.
fn settle_restore(cx: &mut Cx<'_, ProjectFeature>, result: Result<bool, String>) {
    let authority = matches!(result, Ok(true)).then(|| SchematicEditAuthority::capture(&*cx.model));
    let dialog = &mut cx.state.project_revision_history;
    dialog.restore_confirmation = false;
    match result {
        Ok(restored) => {
            if restored {
                dialog.authority = authority;
                dialog.expected_journal = cx.model.schematic.document().validated_revisions.clone();
                dialog.rows_source = None;
            }
            // The restored design is on the canvas behind the dialog.
            dialog.error = None;
        }
        Err(error) => dialog.error = Some(error),
    }
}

/// Put revision `id` of the schematic in front back, as one step of Undo.
fn restore_in_place(
    cx: &mut Cx<'_, ProjectFeature>,
    id: ValidatedSchematicRevisionId,
) -> Result<(), String> {
    history_authority_error(&cx.state.project_revision_history, cx.model).map_or(Ok(()), Err)?;
    if cx.model.safe_mode.project_read_only() || cx.model.schematic_edit_read_only() {
        return Err("The active schematic or project is read-only.".to_owned());
    }
    if cx.state.project_revision_history.expected_journal
        != cx.model.schematic.document().validated_revisions
    {
        return Err(
            "The revision history changed since it opened. Close and reopen it.".to_owned(),
        );
    }
    cx.model
        .schematic
        .restore_validated_revision(id)
        .map_err(|error| error.to_string())?;
    if let Some(prompt) = cx.model.sync_active_schematic_to_workspace() {
        cx.wb.prompt(prompt);
    }
    // What the run's preflight decided, the run prepared and the
    // netlist generated was for the design the restore replaced.
    // The next two calls are what RSpiceApp::invalidate_simulation_preflight does.
    cx.runs.preflight.invalidate();
    cx.handles.run_controller.clear_prepared_run();
    cx.model.netlist_artifacts.current_generation_input_digest = None;
    Ok(())
}

/// Why a record cannot be restored or compared with the current design.
const NOT_A_DRAWING: &str =
    "Only a schematic revision keeps a drawing to restore or to compare with the current design.";

/// The revision `row` restores and the view it opens first, when that view
/// is not the one in front; or why it cannot be restored.
fn restore_target(
    row: &ProjectAuditRow,
    model: &Model,
) -> Result<(ValidatedSchematicRevisionId, Option<CellViewRef>), String> {
    let (Some(revision), Some(view)) = (row.schematic_revision, row.view.as_ref()) else {
        return Err(NOT_A_DRAWING.to_owned());
    };
    let path = view.display_path();
    if row.current {
        return Err(format!("{path} already matches this revision."));
    }
    if model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if model
        .library_manager
        .get_library(&view.library)
        .is_some_and(|library| library.read_only)
    {
        return Err(format!("{path} is in a read-only library."));
    }
    let in_front = *view == model.workspace.content.active_view;
    if in_front && model.schematic_edit_read_only() {
        return Err(format!("{path} is open read-only."));
    }
    Ok((revision, (!in_front).then(|| view.clone())))
}

fn history_authority_error(
    dialog: &ProjectRevisionHistoryDialogState,
    model: &Model,
) -> Option<String> {
    dialog
        .authority
        .as_ref()
        .ok_or_else(|| "Revision history is out of date. Close and reopen it.".to_owned())
        .and_then(|authority| authority.validate_presentation(model, "Project revision history"))
        .err()
}

/// What the selection allows, and why not where it does not: the restore,
/// with the view it opens first; a comparison with the current design; a
/// comparison of two records.
struct Offer {
    restore: Result<(ValidatedSchematicRevisionId, Option<CellViewRef>), String>,
    current: Result<(), String>,
    compare: Result<(), String>,
}

impl Offer {
    fn of(selected: &[&ProjectAuditRow], model: &Model, touch: bool) -> Self {
        let restore = match selected {
            [] => Err("Select a schematic revision to restore it.".to_owned()),
            [row] => restore_target(row, model),
            _ => Err("Restore takes one revision.".to_owned()),
        };
        let current = match selected {
            [] => {
                Err("Select a schematic revision to compare it with the current design.".to_owned())
            }
            [row] if row.schematic_revision.is_some() => Ok(()),
            [_] => Err(NOT_A_DRAWING.to_owned()),
            _ => Err("Compare with current takes one revision.".to_owned()),
        };
        let compare = match selected {
            [] => Err("Select two records to compare them.".to_owned()),
            [_] => Err(second_record_hint(touch).to_owned()),
            [first, second] if first.kind != second.kind => {
                Err("Compare takes two records of one kind.".to_owned())
            }
            [_, _] => Ok(()),
            more => Err(format!(
                "Compare takes two records; {} are selected.",
                more.len()
            )),
        };
        Self {
            restore,
            current,
            compare,
        }
    }
}

/// How the reader adds a second record to the selection, which a finger
/// does by the mark before the row and a mouse by Ctrl.
fn second_record_hint(touch: bool) -> &'static str {
    if touch {
        "Tap the mark before a second record to compare the two."
    } else {
        "Ctrl+click a second record to compare the two."
    }
}

/// The sentence under the actions: how to choose what they act on, why
/// Restore cannot act on what is chosen, which view it opens first, and,
/// while it asks for confirmation, what it replaces.
fn selection_note(
    selected: &[&ProjectAuditRow],
    offer: &Offer,
    confirming: bool,
    blocked: bool,
    touch: bool,
) -> String {
    match selected {
        [] if touch => "Tap a schematic revision to restore it or to compare it with the current design; tap the marks before two records to compare them.".to_owned(),
        [] => "Select a schematic revision to restore it or to compare it with the current design; Ctrl+click a second record to compare the two.".to_owned(),
        [row] => {
            let when = if row.time.is_empty() {
                String::new()
            } else {
                format!(" of {}", row.time)
            };
            let restore = match &offer.restore {
                // The band over the body says why.
                _ if blocked => None,
                Ok((_, Some(view))) if confirming => {
                    return format!(
                        "Restore opens {} and replaces its drawing with revision {}{when}. Undo takes it back.",
                        view.display_path(),
                        row.revision
                    );
                }
                Ok((_, None)) if confirming => {
                    return format!(
                        "Restore replaces the drawing of {} with revision {}{when}. Undo takes it back.",
                        row.view.as_ref().map(CellViewRef::display_path).unwrap_or_default(),
                        row.revision
                    );
                }
                Ok((_, Some(view))) => Some(format!("Restore opens {} first.", view.display_path())),
                Ok((_, None)) => None,
                Err(reason) => Some(reason.clone()),
            };
            restore
                .into_iter()
                .chain([second_record_hint(touch).to_owned()])
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => ["Restore and Compare with current take one record.".to_owned()]
            .into_iter()
            .chain(offer.compare.clone().err())
            .collect::<Vec<_>>()
            .join(" "),
    }
}

/// The height the table may take before its rows scroll.
const TABLE_MAX_HEIGHT: f32 = 286.0;
/// The room the table's sort mark takes beside a column's head.
const SORT_MARK_ROOM: f32 = 11.0;

fn revision_history_body(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    model: &Model,
    blocked: Option<&str>,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> HistoryAction {
    let t = Tokens::get(ui.ctx());
    let touch = t.metrics.is_touch();
    let mut action = HistoryAction::None;
    // The filter and the search on one row, on the body's inset with
    // everything under it.
    ui.horizontal(|ui| {
        let filter_options = ProjectRevisionArtifactFilter::ALL
            .iter()
            .map(|filter| filter.label().to_owned())
            .collect::<Vec<_>>();
        if let Some(index) = select(
            ui,
            "project-revision-artifact-filter",
            "Project artifact filter",
            dialog.artifact_filter.label(),
            &filter_options,
            180.0,
        ) {
            dialog.artifact_filter = ProjectRevisionArtifactFilter::ALL[index];
            dialog.comparison = None;
            dialog.restore_confirmation = false;
        }
        let width = ui.available_width().max(120.0);
        text_field(
            ui,
            TextEdit::singleline(&mut dialog.query)
                .hint_text("Revision, actor, artifact, or message\u{2026}"),
            width,
        );
    });

    refresh_shown(ui, dialog, rows);
    let shown = dialog.shown.clone();
    if shown.rows.is_empty() {
        // Nothing shown is selected: the actions act on what the table
        // lists.
        dialog.table.selection = None;
        dialog.table.others.clear();
        Frame::new()
            .fill(t.color.bg_inset)
            .stroke(Stroke::new(1.0, t.color.border_strong))
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.add_space(24.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(if dialog.query.trim().is_empty() {
                            format!(
                                "No retained {} records in this project",
                                dialog.artifact_filter.label().to_ascii_lowercase()
                            )
                        } else {
                            "No retained records match this search".to_owned()
                        })
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                    );
                });
                ui.add_space(24.0);
            });
    } else {
        let response = history_table(ui, rows, &shown, dialog);
        if response.selection_changed {
            dialog.comparison = None;
            dialog.restore_confirmation = false;
            dialog.error = None;
        }
        // Enter, or a double click, on a schematic revision shows what
        // changed since it.
        if let Some(row) = response.activate
            && rows[shown.rows[row]].schematic_revision.is_some()
        {
            action = HistoryAction::CompareWithCurrent;
        }
    }

    let selected = selected_rows(dialog, rows);
    let offer = Offer::of(&selected, model, touch);
    if let pressed @ (HistoryAction::Compare
    | HistoryAction::CompareWithCurrent
    | HistoryAction::Restore) = action_buttons(ui, dialog, &offer, blocked)
    {
        action = pressed;
    }
    let confirming = dialog.restore_confirmation && offer.restore.is_ok() && blocked.is_none();
    let note = selection_note(&selected, &offer, confirming, blocked.is_some(), touch);
    ui.add(egui::Label::new(RichText::new(note).color(t.color.text_dim)).wrap());

    if let Some(comparison) = dialog.comparison.as_ref() {
        revision_comparison(ui, comparison);
    }
    action
}

/// The history table: every column any record fills, each as wide as its
/// widest words while the table fits, and several rows selectable for a
/// comparison, by Ctrl or by the mark a finger finds before each row.
fn history_table(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    shown: &ShownRows,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> crate::ui::widgets::table::TableResponse {
    let t = Tokens::get(ui.ctx());
    let line = LineHeights::grid(&t);
    // What the columns share: the table's width, less the scroll bar when
    // its rows overflow and the room the selection marks take before the
    // first column under a finger.
    let mut room = ui
        .available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width();
    if line.head + line.row * shown.rows.len() as f32 > TABLE_MAX_HEIGHT + 0.5 {
        room -= ui.spacing().scroll.allocated_width();
    }
    if t.metrics.is_touch() {
        room -= t.metrics.touch_target() - TABLE_EDGE_INSET;
    }
    let widths = fit_widths(&shown.natural, room);
    let columns = shown
        .columns
        .iter()
        .zip(&widths)
        .map(|(&column, &width)| Column::words(COLUMNS[column], width))
        .collect::<Vec<_>>();
    DataTable::new(
        "workbench.project.revision_history",
        &columns,
        shown.rows.len(),
    )
    .state(&mut dialog.table)
    .keys(&shown.keys)
    .multi_select()
    .freeze(1)
    .max_height(TABLE_MAX_HEIGHT)
    .show(ui, |index| {
        row_cells(&rows[shown.rows[index]], &shown.columns)
    })
}

/// Restore, the two comparisons, and while Restore asks for confirmation,
/// its Cancel: in one row where they fit, one above another, as wide as the
/// body, where they do not. A control that cannot act says why on hover;
/// the note under them says it for a finger.
fn action_buttons(
    ui: &mut Ui,
    dialog: &mut ProjectRevisionHistoryDialogState,
    offer: &Offer,
    blocked: Option<&str>,
) -> HistoryAction {
    #[derive(Clone, Copy)]
    enum Press {
        Restore,
        Confirm,
        Cancel,
        Current,
        Compare,
    }
    let restore = match (blocked, &offer.restore) {
        (Some(reason), _) => Err(reason),
        (None, Ok(_)) => Ok(()),
        (None, Err(reason)) => Err(reason.as_str()),
    };
    let confirming = dialog.restore_confirmation && restore.is_ok();
    let mut buttons: Vec<(&str, Result<(), &str>, bool, Press)> = if confirming {
        vec![
            ("Confirm restore", Ok(()), true, Press::Confirm),
            ("Cancel", Ok(()), false, Press::Cancel),
        ]
    } else {
        vec![("Restore selected\u{2026}", restore, false, Press::Restore)]
    };
    buttons.push((
        "Compare with current",
        offer.current.as_ref().map(|_| ()).map_err(String::as_str),
        false,
        Press::Current,
    ));
    buttons.push((
        "Compare selected",
        offer.compare.as_ref().map(|_| ()).map_err(String::as_str),
        false,
        Press::Compare,
    ));
    let spacing = ui.spacing().item_spacing.x;
    let natural = buttons
        .iter()
        .map(|(label, _, destructive, _)| {
            Button::new(label)
                .destructive(*destructive)
                .measured_width(ui)
        })
        .sum::<f32>()
        + spacing * (buttons.len() - 1) as f32;
    let width = ui.available_width();
    let stacked = natural > width;
    let mut pressed = None;
    let mut show = |ui: &mut Ui| {
        for (label, gate, destructive, press) in &buttons {
            let mut button = Button::new(label)
                .enabled(gate.is_ok())
                .destructive(*destructive);
            if stacked {
                button = button.min_width(width);
            }
            let response = button.show(ui);
            if response.clicked() {
                pressed = Some(*press);
            }
            if let Err(reason) = gate {
                let _ = response.on_disabled_hover_text(*reason);
            }
        }
    };
    if stacked {
        ui.vertical(|ui| show(ui));
    } else {
        ui.horizontal(|ui| show(ui));
    }
    match pressed {
        Some(Press::Restore) => {
            dialog.restore_confirmation = true;
            HistoryAction::None
        }
        Some(Press::Confirm) => HistoryAction::Restore,
        Some(Press::Cancel) => {
            dialog.restore_confirmation = false;
            HistoryAction::None
        }
        Some(Press::Current) => HistoryAction::CompareWithCurrent,
        Some(Press::Compare) => HistoryAction::Compare,
        None => HistoryAction::None,
    }
}

/// The rows the artifact filter and the search let through, by their place
/// in `rows`.
fn visible_rows(
    rows: &[ProjectAuditRow],
    dialog: &ProjectRevisionHistoryDialogState,
) -> Vec<usize> {
    let query = dialog.query.trim().to_ascii_lowercase();
    rows.iter()
        .enumerate()
        .filter(|(_, row)| dialog.artifact_filter.accepts(row.kind) && row.matches_query(&query))
        .map(|(index, _)| index)
        .collect()
}

/// Work out what the table shows again when the rows, the filter or the
/// search moved since it last was.
fn refresh_shown(
    ui: &Ui,
    dialog: &mut ProjectRevisionHistoryDialogState,
    rows: &[ProjectAuditRow],
) {
    let current = dialog
        .shown
        .source
        .as_ref()
        .is_some_and(|(source, filter, query)| {
            *source == dialog.rows_source
                && *filter == dialog.artifact_filter
                && *query == dialog.query
        });
    if current {
        return;
    }
    let shown = visible_rows(rows, dialog);
    let keys = shown
        .iter()
        .map(|&row| RowKey::Text(rows[row].key.clone()))
        .collect::<Vec<_>>();
    // The columns and their widths come from every record, so neither moves
    // while the reader filters or searches.
    let columns = shown_columns(rows);
    let natural = natural_widths(ui, rows, &columns);
    dialog.shown = ShownRows {
        source: Some((
            dialog.rows_source,
            dialog.artifact_filter,
            dialog.query.clone(),
        )),
        rows: shown.into(),
        keys: keys.into(),
        columns: columns.into(),
        natural: natural.into(),
    };
}

/// The rows the dialog keeps, built again only when what they were built
/// from has moved since.
fn kept_audit_rows(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
) -> Arc<[ProjectAuditRow]> {
    let journal = &model.schematic.document().validated_revisions;
    let source = ProjectAuditRowsSource {
        journal_len: journal.records().len(),
        journal_head: journal.records().last().map(ValidatedSchematicRevision::id),
        project_revision: model.workspace.content.project.revision().get(),
        active_schematic_epoch: model.active_schematic_epoch,
    };
    if dialog.rows_source != Some(source) {
        dialog.rows = project_audit_rows(model).into();
        dialog.rows_source = Some(source);
    }
    Arc::clone(&dialog.rows)
}

fn project_audit_rows(model: &Model) -> Vec<ProjectAuditRow> {
    #[cfg(any(test, feature = "test-support"))]
    crate::workbench::app::dialogs::project_revision_history::AUDIT_ROW_BUILDS
        .with(|builds| builds.set(builds.get() + 1));
    let mut rows = Vec::new();
    // The schematic open in front, which is the symbol's own while a symbol
    // is, then every other schematic view the project keeps.
    let active = model.workspace.content.active_schematic_reference();
    let active_key = active.key();
    let design = model.schematic.editor_ref().design;
    append_schematic_rows(
        &mut rows,
        &model.schematic.document().validated_revisions,
        || design.validated_design_content_digest().ok(),
        Some(active),
        true,
    );
    let mut retained_schematics = model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .filter(|entry| entry.0.as_str() != active_key.as_str())
        .collect::<Vec<_>>();
    retained_schematics.sort_by(|(left, _), (right, _)| left.cmp(right));
    for (key, schematic) in retained_schematics {
        append_schematic_rows(
            &mut rows,
            &schematic.document().validated_revisions,
            || schematic.validated_design_content_digest().ok(),
            CellViewRef::from_key(key),
            false,
        );
    }
    append_simulation_plan_rows(&mut rows, model);
    append_model_and_pdk_rows(&mut rows, model);
    append_governed_rows(&mut rows, model);
    rows
}

fn find_project_schematic_revision(
    model: &Model,
    id: ValidatedSchematicRevisionId,
) -> Option<&ValidatedSchematicRevision> {
    model
        .schematic
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == id)
        .or_else(|| {
            model
                .workspace
                .content
                .schematic_buffers
                .values()
                .flat_map(|schematic| schematic.document().validated_revisions.records())
                .find(|record| record.id() == id)
        })
}

/// One row per revision of `view`'s journal, newest first. `drawn` is the
/// digest of the view's drawing as it stands, asked only of a view with
/// revisions to hold it against.
fn append_schematic_rows(
    rows: &mut Vec<ProjectAuditRow>,
    journal: &ValidatedRevisionJournal,
    drawn: impl FnOnce() -> Option<ContentDigest>,
    view: Option<CellViewRef>,
    active_view: bool,
) {
    if journal.records().is_empty() {
        return;
    }
    let drawn = drawn();
    let latest = journal.records().last().map(ValidatedSchematicRevision::id);
    for record in journal.records().iter().rev() {
        let status = if record.is_accepted_baseline() {
            "accepted"
        } else if active_view && Some(record.id()) == latest {
            "working"
        } else {
            "validated"
        };
        let made = record.created_unix_ms();
        rows.push(ProjectAuditRow {
            key: schematic_row_key(record.id()),
            kind: ProjectAuditArtifactKind::Schematic,
            revision: short_digest(record),
            time: written_time(made),
            made_unix_ms: (made != 0).then_some(made),
            age: format_revision_time(made),
            actor: record.author().to_owned(),
            change: if record.is_accepted_baseline() {
                "Schematic accepted".to_owned()
            } else {
                record.revision_note().to_owned()
            },
            affected_evidence: affected_schematic_evidence(record),
            status: status.to_owned(),
            schematic_revision: Some(record.id()),
            view: view.clone(),
            current: drawn == Some(record.design_content_digest()),
        });
    }
}

fn append_simulation_plan_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(plan) = model.sim_setup.analysis_plan.as_ref() {
        rows.push(simulation_plan_row(
            model.sim_setup.active_plan_name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.instances().len(),
            model.sim_setup.active_plan_lineage(),
            true,
        ));
    }
    for plan in model.sim_setup.inactive_plans().iter().rev() {
        rows.push(simulation_plan_row(
            plan.name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.analysis_plan().instances().len(),
            plan.lineage(),
            false,
        ));
    }
}

fn simulation_plan_row(
    name: &str,
    id: String,
    revision: String,
    analysis_count: usize,
    lineage: crate::workbench::app_state::SimulationPlanLineage,
    active: bool,
) -> ProjectAuditRow {
    let lineage_evidence = match (lineage.source_plan_id(), lineage.source_revision()) {
        (Some(source), Some(source_revision)) => {
            format!("cloned from {source} @ {}", source_revision.get())
        }
        _ => "root plan".to_owned(),
    };
    ProjectAuditRow {
        key: format!("simulation-plan:{id}:{revision}"),
        kind: ProjectAuditArtifactKind::SimulationPlan,
        revision,
        time: NOT_RETAINED.to_owned(),
        made_unix_ms: None,
        age: String::new(),
        actor: NOT_RETAINED.to_owned(),
        change: format!(
            "{} \u{00b7} {name}",
            if active { "Active" } else { "Retained" }
        ),
        affected_evidence: format!(
            "{analysis_count} {} \u{00b7} {lineage_evidence}",
            if analysis_count == 1 {
                "analysis"
            } else {
                "analyses"
            }
        ),
        status: if active { "working" } else { "retained" }.to_owned(),
        schematic_revision: None,
        view: None,
        current: false,
    }
}

fn append_model_and_pdk_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(binding) = model.workspace.content.project.technology_binding() {
        let digest = binding
            .source_closure()
            .iter()
            .find(|source| source.path == binding.root_source())
            .map(|source| short_text(&source.digest.to_string()))
            .unwrap_or_else(|| model.workspace.content.project.revision().get().to_string());
        rows.push(ProjectAuditRow {
            key: format!("technology-binding:{}:{}", binding.model_library(), digest),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision: digest,
            time: NOT_RETAINED.to_owned(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Project metadata transaction".to_owned(),
            change: format!("Attached technology \u{00b7} {}", binding.display_label()),
            affected_evidence: format!(
                "{} models \u{00b7} {} pinned sources \u{00b7} {} process sections",
                binding.model_count(),
                binding.source_closure().len(),
                binding.process_sections().len()
            ),
            status: "attached".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }

    for library in model.model_library_manager.libraries_sorted() {
        let (revision, status, authority): (String, &str, String) = match library.source_authority {
            ModelSourceAuthority::ProjectOwned {
                source_id,
                revision,
                digest,
            } => (
                revision.get().to_string(),
                "pinned",
                format!(
                    "project source {source_id} \u{00b7} {}",
                    short_text(&digest.to_string())
                ),
            ),
            ModelSourceAuthority::External => {
                let root_digest = library
                    .root_path
                    .as_ref()
                    .and_then(|root| {
                        library
                            .source_closure
                            .iter()
                            .find(|source| &source.path == root)
                    })
                    .map(|source| short_text(&source.digest.to_string()));
                match root_digest {
                    Some(digest) => (
                        digest.clone(),
                        "pinned",
                        format!("external source \u{00b7} root digest {digest}"),
                    ),
                    None => (
                        "unversioned".to_owned(),
                        "unpinned",
                        "external source has no accepted closure".to_owned(),
                    ),
                }
            }
            ModelSourceAuthority::RetainedImport { source_id, digest } => (
                short_text(&digest.to_string()),
                "pinned",
                format!(
                    "retained import {source_id} \u{00b7} root digest {}",
                    short_text(&digest.to_string())
                ),
            ),
            ModelSourceAuthority::BuiltIn => (
                nonempty_or(&library.version, "built-in"),
                "built-in",
                "signed application catalog".to_owned(),
            ),
        };
        rows.push(ProjectAuditRow {
            key: format!("model-library:{}:{revision}", library.name),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision,
            time: NOT_RETAINED.to_owned(),
            made_unix_ms: None,
            age: String::new(),
            actor: authority,
            change: format!(
                "Model library \u{00b7} {}{}",
                library.name,
                if library.pdk_name.trim().is_empty() {
                    String::new()
                } else {
                    format!(" \u{00b7} {}", library.pdk_name)
                }
            ),
            affected_evidence: format!(
                "{} models \u{00b7} {} corners \u{00b7} {} pinned sources",
                library.model_count(),
                library.corner_count(),
                library.source_closure.len()
            ),
            status: status.to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
}

fn append_governed_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    let management = &model.workspace.content.design_management;
    for entry in management.annotation().journal().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("annotation-journal:{}", entry.id()),
            kind: ProjectAuditArtifactKind::GovernedRecord,
            revision: entry.sequence().to_string(),
            time: NOT_RETAINED.to_owned(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Annotation policy transaction".to_owned(),
            change: format!("Committed annotation policy r{}", entry.policy_revision()),
            affected_evidence: format!(
                "{} mappings \u{00b7} policy {} \u{00b7} receipt {}",
                entry.mappings().len(),
                short_text(&entry.policy_digest().to_string()),
                short_text(&entry.semantic_digest().to_string())
            ),
            status: "committed".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
    for receipt in management.hierarchy_audits().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("hierarchy-audit:{}", receipt.id()),
            kind: ProjectAuditArtifactKind::GovernedRecord,
            revision: receipt.sequence().to_string(),
            time: NOT_RETAINED.to_owned(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Hierarchy audit transaction".to_owned(),
            change: "Hierarchy audit receipt".to_owned(),
            affected_evidence: format!(
                "{} resolved subjects \u{00b7} {} findings \u{00b7} receipt {}",
                receipt.resolved_subjects(),
                receipt.findings().len(),
                short_text(&receipt.semantic_digest().to_string())
            ),
            status: if receipt.passed() {
                "passed"
            } else {
                "findings"
            }
            .to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
}

fn revision_comparison(ui: &mut Ui, comparison: &ProjectRevisionComparison) {
    match comparison {
        ProjectRevisionComparison::Schematic {
            label,
            delta,
            with_current,
        } => {
            semantic_comparison(ui, delta, Some(label), *with_current);
        }
        ProjectRevisionComparison::RetainedMetadata(comparison) => {
            section_header(ui, "Retained metadata comparison", Some(&comparison.label));
            if comparison.fields.is_empty() {
                ui.label("The selected retained records expose identical metadata.");
                return;
            }
            Grid::new("retained-project-record-comparison")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    comparison_heading(ui, "Field");
                    comparison_heading(ui, "First");
                    comparison_heading(ui, "Second");
                    ui.end_row();
                    for (field, first, second) in &comparison.fields {
                        ui.label(*field);
                        ui.monospace(first);
                        ui.monospace(second);
                        ui.end_row();
                    }
                });
        }
    }
}

fn semantic_comparison(
    ui: &mut Ui,
    delta: &ValidatedRevisionSemanticDelta,
    label: Option<&str>,
    with_current: bool,
) {
    section_header(ui, "Semantic comparison", label);
    if delta.is_empty() {
        ui.label(if with_current {
            "The current design matches this revision."
        } else {
            "The selected validated revisions contain identical schematic semantics."
        });
        return;
    }
    Grid::new("validated-revision-semantic-delta")
        .num_columns(4)
        .striped(true)
        .show(ui, |ui| {
            comparison_heading(ui, "Domain");
            comparison_heading(ui, "Added");
            comparison_heading(ui, "Removed");
            comparison_heading(ui, "Modified");
            ui.end_row();
            delta_row(ui, "Components", delta.components);
            delta_row(ui, "Wires", delta.wires);
            delta_row(ui, "Buses", delta.buses);
            delta_row(ui, "Bus taps", delta.bus_taps);
            delta_row(ui, "Junctions", delta.junctions);
            delta_row(ui, "Net labels", delta.net_labels);
            delta_row(ui, "Design notes", delta.design_notes);
            delta_row(ui, "Documentation shapes", delta.documentation_shapes);
            ui.label("Connections");
            ui.monospace(delta.connections_added.to_string());
            ui.monospace(delta.connections_removed.to_string());
            ui.monospace("\u{2014}");
            ui.end_row();
            ui.label("Document policy");
            ui.monospace("\u{2014}");
            ui.monospace("\u{2014}");
            ui.monospace(if delta.grid_changed || delta.document_policy_changed {
                "changed"
            } else {
                "\u{2014}"
            });
            ui.end_row();
        });
}

fn delta_row(ui: &mut Ui, label: &str, delta: ValidatedRevisionObjectDelta) {
    ui.label(label);
    ui.monospace(delta.added.to_string());
    ui.monospace(delta.removed.to_string());
    ui.monospace(delta.modified.to_string());
    ui.end_row();
}

/// The history table's columns, in order.
const COLUMNS: [&str; 6] = [
    "Revision",
    "Time",
    "Actor",
    "Change",
    "Affected records",
    "Status",
];
/// The columns whose identifiers and times are set in mono.
const MONO_COLUMNS: [bool; 6] = [true, true, false, false, false, false];
/// The narrowest a column with too much to say is cut to. A table that
/// cannot fit with its widest columns cut that far keeps every column whole
/// and scrolls sideways under its first one, as on a phone.
const LEAST_CUT_WIDTH: f32 = 96.0;

/// One row's text in column `column` of [`COLUMNS`].
fn row_text(row: &ProjectAuditRow, column: usize) -> &str {
    match column {
        0 => &row.revision,
        1 => &row.time,
        2 => &row.actor,
        3 => &row.change,
        4 => &row.affected_evidence,
        _ => &row.status,
    }
}

/// One row's cells in the `shown` columns: identifiers and times in mono,
/// the time with how long ago it was on hover, and sorted by the moment.
fn row_cells(row: &ProjectAuditRow, shown: &[usize]) -> Vec<Cell> {
    shown
        .iter()
        .map(|&column| match (column, row.made_unix_ms) {
            (1, Some(made)) => Cell::number(&row.time)
                .hint(&row.age)
                .sort_by(SortKey::Number(made as f64)),
            (1, None) => Cell::empty().sort_by(SortKey::Missing),
            _ if MONO_COLUMNS[column] => Cell::number(row_text(row, column)),
            _ => Cell::text(row_text(row, column)),
        })
        .collect()
}

/// The columns, by index, that any of `rows` says something in. A column no
/// row fills (times the project never retained) would be a heading over blank
/// cells.
fn shown_columns(rows: &[ProjectAuditRow]) -> Vec<usize> {
    (0..COLUMNS.len())
        .filter(|&column| rows.iter().any(|row| !row_text(row, column).is_empty()))
        .collect()
}

/// How wide each `shown` column is to hold its widest words and its head
/// with the sort mark, with the padding the table puts either side of a
/// cell's words: the panel inset at the table's outer edges.
fn natural_widths(ui: &Ui, rows: &[ProjectAuditRow], shown: &[usize]) -> Vec<f32> {
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    };
    let head = theme::sans(tokens::FS_0, FontWeight::Medium);
    let last = shown.len().saturating_sub(1);
    shown
        .iter()
        .enumerate()
        .map(|(place, &column)| {
            let font = if MONO_COLUMNS[column] {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            };
            let words = rows
                .iter()
                .map(|row| measure(row_text(row, column), &font))
                .fold(measure(COLUMNS[column], &head) + SORT_MARK_ROOM, f32::max);
            let lead = if place == 0 {
                TABLE_EDGE_INSET
            } else {
                CELL_PAD
            };
            let trail = if place == last {
                TABLE_EDGE_INSET
            } else {
                CELL_PAD
            };
            (words + lead + trail + 2.0).ceil()
        })
        .collect()
}

/// Natural column widths fitted to `room`. Room to spare widens every column
/// in proportion. Too little is taken from the widest columns alone: they
/// are cut to one common width, as wide as still fits, and every column
/// narrower than that keeps its whole text. Shrinking every column in
/// proportion elided an eight-letter revision ("built-i...") beside a column
/// of prose that could have given the room up. A table that would cut a
/// column below [`LEAST_CUT_WIDTH`] keeps its natural widths and scrolls.
fn fit_widths(natural: &[f32], room: f32) -> Vec<f32> {
    let total = natural.iter().sum::<f32>().max(1.0);
    if total <= room {
        let scale = room / total;
        return natural
            .iter()
            .map(|width| (width * scale).floor())
            .collect();
    }
    let mut sorted = natural.to_vec();
    sorted.sort_by(f32::total_cmp);
    let mut below = 0.0;
    let mut cap = 0.0;
    for (index, width) in sorted.iter().enumerate() {
        let wider = (sorted.len() - index) as f32;
        // Every column from this one up cut to the same width.
        let candidate = (room - below) / wider;
        if candidate <= *width {
            cap = candidate;
            break;
        }
        below += width;
    }
    if cap < LEAST_CUT_WIDTH {
        return natural.to_vec();
    }
    natural.iter().map(|width| width.min(cap).floor()).collect()
}

/// A comparison table's column heading, in the history table's face.
fn comparison_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
}

/// The history table's key for a validated schematic revision, which its
/// selection holds.
pub(crate) fn schematic_row_key(id: ValidatedSchematicRevisionId) -> String {
    format!("schematic:{}", id.as_uuid())
}

fn short_digest(record: &ValidatedSchematicRevision) -> String {
    short_text(&record.revision_digest().to_string())
}

/// A schematic revision as a comparison's heading names it: its digest and
/// when it was made.
fn revision_label(record: &ValidatedSchematicRevision) -> String {
    format!(
        "{} ({})",
        short_digest(record),
        crate::time_compat::utc_stamp(record.created_unix_ms())
    )
}

/// When a record was made, as the table's Time column writes it; nothing
/// for the unknown time zero stands for.
fn written_time(made_unix_ms: u64) -> String {
    if made_unix_ms == 0 {
        String::new()
    } else {
        crate::time_compat::utc_stamp(made_unix_ms)
    }
}

fn short_text(value: &str) -> String {
    value.chars().take(10).collect()
}

fn nonempty_or(value: &str, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value.to_owned()
    }
}

fn format_revision_time(timestamp_ms: u64) -> String {
    if timestamp_ms == 0 {
        return "time unavailable".to_owned();
    }
    let Ok(now) = crate::time_compat::checked_unix_time_ms() else {
        return "time unavailable".to_owned();
    };
    if timestamp_ms > now {
        return "clock skew".to_owned();
    }
    crate::time_compat::relative_age(std::time::Duration::from_millis(now - timestamp_ms))
}

fn affected_schematic_evidence(record: &ValidatedSchematicRevision) -> String {
    let findings = record.finding_counts();
    let mut parts = vec![record.view_identity().to_owned()];
    if record.dependency_count() > 0 {
        parts.push(format!("{} dependencies", record.dependency_count()));
    }
    if findings.advisories > 0 {
        parts.push(format!("{} advisories", findings.advisories));
    }
    if findings.blockers > 0 {
        parts.push(format!("{} blockers", findings.blockers));
    }
    if parts.len() == 1 {
        parts.push(format!("project revision {}", record.project_revision()));
    }
    parts.join(" \u{00b7} ")
}

fn comparison_field(
    fields: &mut Vec<(&'static str, String, String)>,
    label: &'static str,
    first: &str,
    second: &str,
) {
    if first != second {
        fields.push((label, first.to_owned(), second.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProjectId;
    use crate::state::{
        ComponentType, Point, PreflightReport, ValidatedRevisionRequest, ValidationFindingCounts,
    };
    use crate::workbench::project::tests::harness;

    fn append(state: &mut crate::state::SchematicState, note: &str) {
        let digest = state
            .editor_ref()
            .design
            .validated_design_content_digest()
            .unwrap();
        state
            .append_validated_revision(ValidatedRevisionRequest {
                project_id: ProjectId::new().to_string(),
                project_revision: 1,
                view_identity: "user/top/schematic".to_owned(),
                revision_note: note.to_owned(),
                author: CURRENT_TEST_ACTOR.to_owned(),
                validation_receipt_digest: digest,
                finding_counts: ValidationFindingCounts::default(),
                dependencies: Vec::new(),
                advisory_dispositions: Vec::new(),
            })
            .unwrap();
    }

    const CURRENT_TEST_ACTOR: &str = "Test engineer";

    /// A row of a record that keeps no time and no drawing.
    fn metadata_row(key: &str, kind: ProjectAuditArtifactKind, change: &str) -> ProjectAuditRow {
        ProjectAuditRow {
            key: key.to_owned(),
            kind,
            revision: key.to_owned(),
            time: NOT_RETAINED.to_owned(),
            made_unix_ms: None,
            age: String::new(),
            actor: NOT_RETAINED.to_owned(),
            change: change.to_owned(),
            affected_evidence: String::new(),
            status: "retained".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        }
    }

    /// Every string the body painted, with where, after it settled at
    /// `width`.
    fn painted_body(
        width: f32,
        rows: &[ProjectAuditRow],
        model: &Model,
        dialog: &mut ProjectRevisionHistoryDialogState,
    ) -> Vec<(String, egui::Rect)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 600.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE)
                            .show(ctx, |ui| {
                                let _ = revision_history_body(ui, rows, model, None, dialog);
                            });
                    },
                )
                .shapes;
        }
        shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn search_is_bound_to_retained_record_fields() {
        let mut state = crate::state::SchematicState::default();
        append(&mut state, "Update compensation");
        let mut rows = Vec::new();
        append_schematic_rows(
            &mut rows,
            &state.document().validated_revisions,
            || None,
            None,
            true,
        );
        let mut dialog = ProjectRevisionHistoryDialogState {
            query: "compensation".to_owned(),
            ..ProjectRevisionHistoryDialogState::default()
        };
        assert_eq!(visible_rows(&rows, &dialog).len(), 1);
        dialog.query = "fixture-only-row".to_owned();
        assert!(visible_rows(&rows, &dialog).is_empty());
    }

    #[test]
    fn semantic_compare_and_restore_are_exact_and_undoable() {
        let mut state = crate::state::SchematicState::default();
        append(&mut state, "Empty baseline");
        let first = state.document().validated_revisions.records()[0].clone();
        state.add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut state, "Add resistor");
        let second = state.document().validated_revisions.records()[1].clone();
        let delta = first.semantic_delta_to(&second);
        assert_eq!(delta.components.added, 1);

        state
            .restore_validated_revision(first.id())
            .expect("validated restore");
        assert!(state.document().components.is_empty());
        assert!(state.undo());
        assert_eq!(state.document().components.len(), 1);
    }

    #[test]
    fn artifact_filters_never_relabel_records() {
        let rows = vec![
            metadata_row(
                "schematic:one",
                ProjectAuditArtifactKind::Schematic,
                "schematic",
            ),
            metadata_row(
                "simulation-plan:two",
                ProjectAuditArtifactKind::SimulationPlan,
                "plan",
            ),
        ];
        let dialog = ProjectRevisionHistoryDialogState {
            artifact_filter: ProjectRevisionArtifactFilter::SimulationPlans,
            ..ProjectRevisionHistoryDialogState::default()
        };
        let visible = visible_rows(&rows, &dialog);
        assert_eq!(visible.len(), 1);
        assert_eq!(
            rows[visible[0]].kind,
            ProjectAuditArtifactKind::SimulationPlan
        );
    }

    /// A table too wide for its room cuts its widest columns to one width and
    /// leaves every narrower column whole; one with room to spare widens
    /// every column in proportion; one that would cut a column to a sliver,
    /// as a phone's would, keeps every column whole and scrolls.
    #[test]
    fn a_table_that_cannot_fit_cuts_only_its_widest_columns() {
        let fitted = fit_widths(&[60.0, 120.0, 190.0, 260.0, 60.0], 600.0);
        assert_eq!(fitted, vec![60.0, 120.0, 180.0, 180.0, 60.0]);
        let roomy = fit_widths(&[50.0, 150.0], 400.0);
        assert_eq!(roomy, vec![100.0, 300.0]);
        let narrow = fit_widths(&[100.0, 200.0, 300.0], 250.0);
        assert_eq!(narrow, vec![100.0, 200.0, 300.0]);
    }

    /// The table fits the dialog's width, every column of it, with its
    /// headings as written and nothing below it but the actions and what
    /// they need: it used to run its last columns off the dialog behind a
    /// horizontal scroll bar, head them in upper-case mono, fill unkept
    /// facts with `not retained` and close on two cards explaining the
    /// engine.
    #[test]
    fn the_history_table_fits_its_width_and_says_nothing_it_does_not_know() {
        const WIDTH: f32 = 736.0;
        let harness = harness();
        let long = "16 models \u{b7} 0 corners \u{b7} 0 pinned sources \u{b7} more evidence";
        let rows = vec![ProjectAuditRow {
            revision: "built-in".to_owned(),
            actor: "signed application catalog".to_owned(),
            affected_evidence: long.to_owned(),
            status: "built-in".to_owned(),
            ..metadata_row(
                "model-library:one",
                ProjectAuditArtifactKind::ModelOrPdk,
                "Model library \u{b7} RSpice Foundation",
            )
        }];
        let mut dialog = ProjectRevisionHistoryDialogState {
            open: true,
            ..ProjectRevisionHistoryDialogState::default()
        };
        let texts = painted_body(WIDTH, &rows, &harness.model, &mut dialog);
        for heading in COLUMNS {
            // No row retained a time, so no column stands over blank cells.
            let drawn = texts.iter().any(|(text, _)| text == heading);
            assert_eq!(drawn, heading != "Time", "{heading:?}: {texts:?}");
        }
        // Every cell but the widest is stated whole; the widest alone is cut
        // to what the dialog has left, and shows whole where it is pointed
        // at or held.
        for whole in (0..COLUMNS.len())
            .map(|column| row_text(&rows[0], column))
            .filter(|text| !text.is_empty() && *text != long)
        {
            assert!(
                texts.iter().any(|(text, _)| text == whole),
                "{whole:?} is elided: {texts:?}"
            );
        }
        assert!(
            texts.iter().any(|(text, _)| text.ends_with('\u{2026}')
                && long.starts_with(text.trim_end_matches('\u{2026}').trim_end())),
            "{long:?} is not cut: {texts:?}"
        );
        for (text, rect) in &texts {
            assert!(
                rect.right() <= WIDTH + 0.5,
                "{text:?} runs past the table's edge to {}",
                rect.right()
            );
            for gone in [
                "REVISION",
                "not retained",
                "Semantic comparison",
                "Results never change",
            ] {
                assert!(!text.contains(gone), "{text:?} is still stated");
            }
        }
    }

    /// Each revision states the date and time it was made in the table, not
    /// an age alone, and a comparison names the revisions it compares by
    /// that date and time too.
    #[test]
    fn a_revision_states_when_it_was_made_and_so_does_its_comparison() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Baseline");
        harness
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut harness.model.schematic, "Add resistor");
        let records = harness
            .model
            .schematic
            .document()
            .validated_revisions
            .records()
            .to_vec();
        let rows = project_audit_rows(&harness.model);
        let mut dialog = ProjectRevisionHistoryDialogState {
            open: true,
            ..ProjectRevisionHistoryDialogState::default()
        };
        let texts = painted_body(1200.0, &rows, &harness.model, &mut dialog);
        let made = crate::time_compat::utc_stamp(records[1].created_unix_ms());
        assert!(
            texts.iter().any(|(text, _)| *text == made),
            "{made:?} is not in the table: {texts:?}"
        );
        assert!(
            !texts.iter().any(|(text, _)| text == "just now"),
            "an age stands for the time: {texts:?}"
        );

        dialog.stand_on(schematic_row_key(records[1].id()));
        compare_with_current(&mut dialog, &harness.model, &rows);
        let Some(ProjectRevisionComparison::Schematic { label, .. }) = &dialog.comparison else {
            panic!("no comparison: {:?}", dialog.error);
        };
        assert!(label.contains(&made), "{label:?}");
    }

    /// Restore says why it cannot act on what is selected: a record that
    /// keeps no drawing, a revision the drawing already matches, a second
    /// record beside the first. Nothing selected says how to choose.
    #[test]
    fn restore_states_why_it_cannot_act() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Baseline");
        let rows = project_audit_rows(&harness.model);
        let revision = rows
            .iter()
            .find(|row| row.schematic_revision.is_some())
            .expect("the revision is listed");
        let plan = metadata_row(
            "simulation-plan:one",
            ProjectAuditArtifactKind::SimulationPlan,
            "Active",
        );
        let note = |selected: &[&ProjectAuditRow], touch: bool| {
            selection_note(
                selected,
                &Offer::of(selected, &harness.model, touch),
                false,
                false,
                touch,
            )
        };
        let shown = revision
            .view
            .as_ref()
            .map(CellViewRef::display_path)
            .unwrap();
        assert!(revision.current, "nothing changed since the revision");
        assert_eq!(
            Offer::of(&[revision], &harness.model, false).restore,
            Err(format!("{shown} already matches this revision."))
        );
        assert!(note(&[revision], false).starts_with(&format!("{shown} already matches")));
        assert_eq!(
            Offer::of(&[&plan], &harness.model, false).restore,
            Err(NOT_A_DRAWING.to_owned())
        );
        assert!(note(&[&plan], true).starts_with(NOT_A_DRAWING));
        assert!(
            note(&[&plan], true)
                .ends_with("Tap the mark before a second record to compare the two.")
        );
        assert!(note(&[&plan], false).ends_with("Ctrl+click a second record to compare the two."));
        assert_eq!(
            note(&[revision, &plan], false),
            "Restore and Compare with current take one record. Compare takes two records of one kind."
        );
        assert!(note(&[], true).starts_with("Tap a schematic revision"));
    }

    /// A revision of a schematic that is not in front asks the shell to open
    /// it and waits; a view that did not open is said not to have, and
    /// nothing is restored.
    #[test]
    fn a_revision_of_a_view_not_in_front_opens_it_first() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Amplifier");
        // The drawing moved on since its revision.
        harness
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        let amp = CellViewRef::new("user", "amp", "schematic");
        let drawing = harness.model.schematic.editor_ref().design.clone();
        harness
            .model
            .workspace
            .content
            .schematic_buffers
            .insert(amp.key(), drawing);
        harness.model.schematic = crate::state::SchematicState::default();
        let rows = project_audit_rows(&harness.model);
        let row = rows
            .iter()
            .find(|row| row.view.as_ref() == Some(&amp))
            .expect("the other view's revision is listed");
        assert!(!row.current);
        assert_eq!(
            Offer::of(&[row], &harness.model, true).restore,
            Ok((row.schematic_revision.unwrap(), Some(amp.clone())))
        );
        harness.state.project_revision_history = ProjectRevisionHistoryDialogState {
            open: true,
            authority: Some(SchematicEditAuthority::capture(&harness.model)),
            expected_journal: harness
                .model
                .schematic
                .document()
                .validated_revisions
                .clone(),
            ..ProjectRevisionHistoryDialogState::default()
        };
        harness
            .state
            .project_revision_history
            .stand_on(row.key.clone());

        restore_selected_revision(&mut harness.cx(), &rows);

        let dialog = &harness.state.project_revision_history;
        assert_eq!(dialog.error, None);
        let pending = dialog.pending_restore.clone().expect("the restore waits");
        assert_eq!(pending.view, amp);
        match harness.wb.outbox.take::<ProjectRequest>() {
            Some(ProjectRequest::OpenView(view, ViewType::Schematic)) => assert_eq!(view, amp),
            other => panic!("the view is not opened: {other:?}"),
        }

        // The shell did not bring it to the front.
        finish_pending_restore(&mut harness.cx());
        let dialog = &harness.state.project_revision_history;
        assert_eq!(dialog.pending_restore, None);
        assert_eq!(
            dialog.error.as_deref(),
            Some("user/amp/schematic did not open, so its revision was not restored.")
        );
        assert_eq!(
            harness.model.workspace.content.schematic_buffers[&amp.key()]
                .document()
                .components
                .len(),
            1
        );
    }

    /// A restore in the project feature's context does what the application
    /// did in place: the design is the revision's, the dialog is re-armed on
    /// the journal it now shows, and what was decided, prepared or generated
    /// for the replaced design is dropped. A restore refused, here because
    /// the journal moved since the dialog read it, leaves all of it alone
    /// and is stated in the dialog.
    #[test]
    fn a_revision_restored_in_the_project_context_ends_where_the_restore_in_place_did() {
        for journal_moved in [false, true] {
            let mut harness = harness();
            let schematic = &mut harness.model.schematic;
            let baseline_components = schematic.document().components.len();
            append(schematic, "Baseline");
            let baseline = schematic.document().validated_revisions.records()[0].id();
            let read_at_open = schematic.document().validated_revisions.clone();
            schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
            append(schematic, "Add resistor");
            harness.state.project_revision_history = ProjectRevisionHistoryDialogState {
                open: true,
                authority: Some(SchematicEditAuthority::capture(&harness.model)),
                expected_journal: if journal_moved {
                    read_at_open
                } else {
                    harness
                        .model
                        .schematic
                        .document()
                        .validated_revisions
                        .clone()
                },
                restore_confirmation: true,
                ..ProjectRevisionHistoryDialogState::default()
            };
            harness
                .state
                .project_revision_history
                .stand_on(schematic_row_key(baseline));
            let digest = ContentDigest::from_bytes([7; 32]);
            harness
                .model
                .netlist_artifacts
                .current_generation_input_digest = Some(digest);
            harness.runs.preflight.report = Some(PreflightReport {
                project_revision: 1,
                topology_root: "user/top/schematic".to_owned(),
                topology_revision: 1,
                topology_closure: Vec::new(),
                simulation_plan_id: None,
                simulation_plan_revision: None,
                blockers: Vec::new(),
                advisories: Vec::new(),
                prepared: None,
            });

            let rows = project_audit_rows(&harness.model);
            restore_selected_revision(&mut harness.cx(), &rows);

            let dialog = &harness.state.project_revision_history;
            let model = &harness.model;
            assert!(
                !dialog.restore_confirmation,
                "journal moved {journal_moved}"
            );
            if journal_moved {
                assert_eq!(
                    dialog.error.as_deref(),
                    Some("The revision history changed since it opened. Close and reopen it.")
                );
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components + 1
                );
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    Some(digest)
                );
                assert!(harness.runs.preflight.report.is_some());
            } else {
                assert_eq!(dialog.error, None);
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components
                );
                assert_eq!(
                    dialog.expected_journal,
                    model.schematic.document().validated_revisions
                );
                assert_eq!(history_authority_error(dialog, model), None);
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    None
                );
                assert!(harness.runs.preflight.report.is_none());
            }
        }
    }
}
