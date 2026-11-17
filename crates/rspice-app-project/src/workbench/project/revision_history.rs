//! Project revision and audit history backed exclusively by persisted owners.
//!
//! Validated schematic revisions own exact restorable snapshots. Simulation
//! plans, model/PDK bindings, and governed design-management receipts expose
//! their retained identities and metadata without inventing missing actors,
//! timestamps, or restoration authority.
//!
//! The dialog lists every record above a divider the reader drags, and the
//! selected record below it: what changed since a revision, named object by
//! object, or what a record keeps, with Restore and the choice of what to
//! compare it with. On a phone the list takes the whole body and a tapped
//! record opens on a page of its own.

use std::sync::Arc;

use egui::{Color32, Context, Frame, Id, Margin, Painter, RichText, TextEdit, Ui};

use crate::product::ContentDigest;
use crate::state::model_library::ModelSourceAuthority;
use crate::state::{
    CellViewRef, ValidatedRevisionChange, ValidatedRevisionJournal, ValidatedRevisionSemanticDelta,
    ValidatedSchematicRevision, ValidatedSchematicRevisionId, ViewType,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, Group, LineHeights, RowKey, SortKey, TABLE_EDGE_INSET,
    TableResponse, TableState,
};
use crate::ui::widgets::{
    Button, DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize,
    DialogTransactionTone, band_header, select, text_field,
};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::project_revision_history::{
    PendingRestore, ProjectAuditArtifactKind, ProjectAuditComparison, ProjectAuditRow,
    ProjectAuditRowsSource, ProjectRevisionArtifactFilter, ProjectRevisionComparison,
    ProjectRevisionHistoryDialogState, ShownRows,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::docks::{apply_splitter_step, hold_panel_height, surface_splitter};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::state::workbench_session::WidthClass;

const TITLE: &str = "Project revision and audit history";
/// What a cell says, on hover over its absent mark, when the record's owner
/// keeps no such fact.
const NOT_RECORDED: &str = "Not recorded";
/// Between the parts of a list in one cell.
const SEPARATOR: &str = " \u{b7} ";
/// The split between the record list and the selected record.
const SPLIT_ID: &str = "workbench.project.revision_history.split";

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

/// The retained facts two records of one kind differ in.
impl ProjectAuditComparison {
    fn between(first: &ProjectAuditRow, second: &ProjectAuditRow) -> Self {
        let mut fields = Vec::new();
        comparison_field(&mut fields, "Revision", &first.revision, &second.revision);
        comparison_field(&mut fields, "Time", &first.time, &second.time);
        comparison_field(&mut fields, "Actor", &first.actor, &second.actor);
        comparison_field(&mut fields, "Change", &first.change, &second.change);
        comparison_field(
            &mut fields,
            "Affected records",
            &first.affected_evidence,
            &second.affected_evidence,
        );
        comparison_field(&mut fields, "Status", &first.status, &second.status);
        Self { fields }
    }
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
    /// Draw Project revision history while it is open, and carry out the
    /// restore its body asked for.
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
        // The list takes whatever height the surface has and scrolls inside
        // it; nothing else in the body scrolls.
        let mut dialog = Dialog::new(TITLE, "Close")
            .description(
                "Inspect saved project revisions, what changed since each, and the audit history of schematics, simulations, models, and other records.",
            )
            .size(DialogSize::RecordHistory)
            .fill_available_height()
            .flush_body()
            .manual_body_scroll()
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
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
        let phone = WidthClass::for_width(ctx.content_rect().width()) == WidthClass::Phone;

        let mut restore = false;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            restore = revision_history_body(
                ui,
                &rows,
                cx.model,
                blocked.as_deref(),
                phone,
                &mut cx.state.project_revision_history,
            );
            // The body is the surface's whole track, so the footer stays
            // at the surface's foot however little the record says.
            ui.expand_to_include_rect(ui.max_rect());
            None
        });
        if restore {
            restore_selected_revision(cx, &rows);
        }
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            cx.state.project_revision_history.close();
        }
    }
}

/// The selected row, while the history stands on one it lists.
fn selected_row<'a>(
    dialog: &ProjectRevisionHistoryDialogState,
    rows: &'a [ProjectAuditRow],
) -> Option<&'a ProjectAuditRow> {
    let key = dialog.selected()?;
    rows.iter().find(|row| row.key == key)
}

/// Work the selected record's comparison out again when the selection, the
/// record it is compared with or the rows it was worked from moved. A
/// record newly selected is compared with the design as it stands when it
/// is a schematic revision, and with nothing otherwise.
fn refresh_comparison(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    rows: &[ProjectAuditRow],
) {
    let Some(row) = selected_row(dialog, rows) else {
        dialog.compare_with = None;
        dialog.comparison = None;
        dialog.compared = None;
        return;
    };
    let moved_on = dialog
        .compared
        .as_ref()
        .is_some_and(|(compared, _)| *compared != row.key);
    let gone = dialog
        .compare_with
        .as_ref()
        .is_some_and(|other| !rows.iter().any(|candidate| candidate.key == *other));
    if moved_on || gone {
        dialog.compare_with = None;
    }
    let wanted = (row.key.clone(), dialog.compare_with.clone());
    if dialog.compared.as_ref() == Some(&wanted) {
        return;
    }
    let other = wanted
        .1
        .as_ref()
        .and_then(|other| rows.iter().find(|candidate| candidate.key == *other));
    dialog.comparison = comparison_of(row, other, model);
    dialog.changes = TableState::default();
    dialog.compared = Some(wanted);
}

/// What `row` is compared with `other`, or with the design as it stands
/// when `other` is none and `row` is a schematic revision; none for any
/// other record compared with nothing.
fn comparison_of(
    row: &ProjectAuditRow,
    other: Option<&ProjectAuditRow>,
    model: &Model,
) -> Option<ProjectRevisionComparison> {
    let Some(id) = row.schematic_revision else {
        return other.map(|other| {
            ProjectRevisionComparison::RetainedMetadata(ProjectAuditComparison::between(row, other))
        });
    };
    let path = row
        .view
        .as_ref()
        .map(CellViewRef::display_path)
        .unwrap_or_default();
    Some(match other {
        None => match changes_since(row, model) {
            Ok((record, delta)) => ProjectRevisionComparison::Schematic {
                title: "Changes to the current design",
                sides: format!(
                    "From revision {} to the current design of {path}.",
                    revision_label(record)
                ),
                delta,
                with_current: true,
            },
            Err(error) => ProjectRevisionComparison::Unavailable(error.to_owned()),
        },
        Some(other) => {
            let records = (
                find_project_schematic_revision(model, id),
                other
                    .schematic_revision
                    .and_then(|other| find_project_schematic_revision(model, other)),
            );
            let (Some(first), Some(second)) = records else {
                return Some(ProjectRevisionComparison::Unavailable(
                    "The revision compared with no longer exists.".to_owned(),
                ));
            };
            let (older, newer) = if first.sequence() <= second.sequence() {
                (first, second)
            } else {
                (second, first)
            };
            ProjectRevisionComparison::Schematic {
                title: "Changes between two revisions",
                sides: format!(
                    "From revision {} to revision {}, in {path}.",
                    revision_label(older),
                    revision_label(newer)
                ),
                delta: older.semantic_delta_to(newer),
                with_current: false,
            }
        }
    })
}

/// The revision `row` names, and what changed in its view's drawing since
/// it was recorded: the open schematic's, or the one the project keeps for
/// a view that is not in front.
fn changes_since<'a>(
    row: &ProjectAuditRow,
    model: &'a Model,
) -> Result<
    (
        &'a ValidatedSchematicRevision,
        ValidatedRevisionSemanticDelta,
    ),
    &'static str,
> {
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
    Ok((record, delta))
}

/// The records `row` may be compared with, after the comparison it starts
/// with: for a schematic revision the design as it stands and then the
/// other revisions of its view, for any other record nothing and then the
/// other records of its kind; each as the choice names it.
fn compare_choices<'a>(
    row: &ProjectAuditRow,
    rows: &'a [ProjectAuditRow],
) -> (&'static str, Vec<(&'a ProjectAuditRow, String)>) {
    let others = rows
        .iter()
        .filter(|other| {
            other.key != row.key
                && other.kind == row.kind
                && (row.schematic_revision.is_none() || other.view == row.view)
        })
        .map(|other| {
            let named = if other.time.is_empty() {
                format!("{}{SEPARATOR}{}", other.revision, other.change)
            } else {
                format!("{} ({})", other.revision, other.time)
            };
            (other, named)
        })
        .collect();
    let first = if row.schematic_revision.is_some() {
        "Current design"
    } else {
        "No comparison"
    };
    (first, others)
}

/// Restore the one selected schematic revision: in place when its view is
/// in front, and otherwise once the view it opens is.
fn restore_selected_revision(cx: &mut Cx<'_, ProjectFeature>, rows: &[ProjectAuditRow]) {
    let target = match selected_row(&cx.state.project_revision_history, rows) {
        Some(row) => restore_target(row, cx.model),
        None => Err("Select a validated schematic revision to restore.".to_owned()),
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

/// The sentence under Restore: why it cannot act on the selected revision,
/// which view it opens first, and, while it asks for confirmation, what it
/// replaces. Nothing while the band over the body says why nothing can be
/// restored, or while Restore has nothing to add.
fn restore_note(
    row: &ProjectAuditRow,
    restore: &Result<(ValidatedSchematicRevisionId, Option<CellViewRef>), String>,
    confirming: bool,
    blocked: bool,
) -> Option<String> {
    let when = if row.time.is_empty() {
        String::new()
    } else {
        format!(" of {}", row.time)
    };
    match restore {
        _ if blocked => None,
        Ok((_, Some(view))) if confirming => Some(format!(
            "Restore opens {} and replaces its drawing with revision {}{when}. Undo takes it back.",
            view.display_path(),
            row.revision
        )),
        Ok((_, None)) if confirming => Some(format!(
            "Restore replaces the drawing of {} with revision {}{when}. Undo takes it back.",
            row.view
                .as_ref()
                .map(CellViewRef::display_path)
                .unwrap_or_default(),
            row.revision
        )),
        Ok((_, Some(view))) => Some(format!("Restore opens {} first.", view.display_path())),
        Ok((_, None)) => None,
        Err(reason) => Some(reason.clone()),
    }
}

/// The room the table's sort mark takes beside a column's head.
const SORT_MARK_ROOM: f32 = 11.0;
/// What a select's well holds besides its value: the value's lead and the
/// chevron's room, with a point either side.
const SELECT_CHROME: f32 = 8.0 + 24.0 + 2.0;

/// The history's body: the filter and the search pinned at the top, the
/// records under them, and the selected record below a divider the reader
/// drags. On a phone the records take the whole body, and a tapped record
/// opens on a page of its own with Back. Returns whether the reader
/// confirmed a restore.
fn revision_history_body(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    model: &Model,
    blocked: Option<&str>,
    phone: bool,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> bool {
    if phone && dialog.record_page && selected_row(dialog, rows).is_some() {
        if inset(ui, |ui| {
            Button::new("Back")
                .icon(Icon::ChevronLeft)
                .show(ui)
                .clicked()
        }) {
            dialog.record_page = false;
            dialog.restore_confirmation = false;
            dialog.reveal = true;
        }
        return record_pane(ui, rows, model, blocked, dialog);
    }
    dialog.record_page = false;
    filter_row(ui, dialog);
    refresh_shown(ui, dialog, rows);
    if phone {
        let height = ui.available_height();
        let response = history_list(ui, rows, dialog, height);
        if response.clicked.is_some() || response.activate.is_some() {
            dialog.record_page = true;
        }
        return false;
    }

    // The list over the selected record, split where the reader dragged it,
    // or else as tall as its rows up to half the body, each side keeping
    // room for its head and a few rows.
    let t = Tokens::get(ui.ctx());
    let line = LineHeights::grid(&t);
    let room = ui.available_height();
    let least_list = line.head + 3.0 * line.row;
    let least_record = crate::ui::panels::PANEL_SECTION_H
        + 2.0 * DIALOG_BODY_INSET
        + t.metrics.ctl_h
        + line.head
        + 2.0 * line.row;
    let most_list = (room - least_record).max(least_list);
    let range = least_list..=most_list;
    let rows_height = line.head + line.row * dialog.shown.rows.len() as f32 + 1.0;
    let held = dialog
        .list_height
        .unwrap_or_else(|| rows_height.min(room / 2.0))
        .clamp(least_list, most_list);
    let split = Id::new(SPLIT_ID);
    hold_panel_height(ui.ctx(), split, held);
    let shown = egui::Panel::top(split)
        .frame(Frame::NONE)
        .show_separator_line(true)
        .resizable(true)
        .default_size(held)
        .size_range(range.clone())
        .show(ui, |ui| {
            let response = history_list(ui, rows, dialog, ui.available_height());
            // Claimed whole, so the size egui keeps is the one dragged to.
            ui.expand_to_include_rect(ui.max_rect());
            response
        });
    let (dragged, keys) = surface_splitter(
        ui.ctx(),
        split,
        "Resize the record list",
        held,
        range,
        line.row,
    );
    if dragged {
        dialog.list_height = Some(shown.response.rect.height().clamp(least_list, most_list));
        dialog.reveal = true;
    }
    if let Some(action) = keys {
        dialog.list_height = apply_splitter_step(held, action, line.row)
            .map(|height| height.clamp(least_list, most_list));
        dialog.reveal = true;
        ui.ctx().request_repaint();
    }
    record_pane(ui, rows, model, blocked, dialog)
}

/// `contents` inset from the body's edges as the dialog's own rows are.
fn inset<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::NONE
        .inner_margin(Margin::same(DIALOG_BODY_INSET as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = DIALOG_BODY_INSET / 2.0;
            contents(ui)
        })
        .inner
}

/// Words that explain, in the dimmed register, wrapped to the room.
fn note(ui: &mut Ui, words: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            RichText::new(words)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
}

/// The artifact filter and the search on one row, inset.
fn filter_row(ui: &mut Ui, dialog: &mut ProjectRevisionHistoryDialogState) {
    inset(ui, |ui| {
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
    });
}

/// The record list, `height` tall: every column any record fills, each as
/// wide as its widest words while the list fits, newest first, one record
/// selected. Its rows scroll inside it under a frozen head.
fn history_list(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    dialog: &mut ProjectRevisionHistoryDialogState,
    height: f32,
) -> TableResponse {
    let shown = dialog.shown.clone();
    if shown.rows.is_empty() {
        // Nothing shown is selected: the record below is one the list shows.
        dialog.table.selection = None;
        dialog.table.others.clear();
        let words = if dialog.query.trim().is_empty() {
            format!(
                "No retained {} records in this project.",
                dialog.artifact_filter.label().to_ascii_lowercase()
            )
        } else {
            "No retained records match this search.".to_owned()
        };
        inset(ui, |ui| note(ui, &words));
        return TableResponse::default();
    }
    let t = Tokens::get(ui.ctx());
    let line = LineHeights::grid(&t);
    // What the columns share: the list's width, less the scroll bar when
    // its rows overflow.
    let mut room = ui
        .available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width();
    if line.head + line.row * shown.rows.len() as f32 > height + 0.5 {
        room -= ui.spacing().scroll.allocated_width();
    }
    let whole = shown
        .columns
        .iter()
        .map(|&column| WHOLE_COLUMNS[column])
        .collect::<Vec<_>>();
    let widths = fit_widths(&shown.natural, &whole, room);
    let columns = shown
        .columns
        .iter()
        .zip(&widths)
        .map(|(&column, &width)| Column::words(COLUMNS[column], width))
        .collect::<Vec<_>>();
    // Frozen through the revision, so what a row is stays put while the
    // rest scroll under it; a phone's keeps its time beside it.
    let frozen = shown
        .columns
        .iter()
        .position(|&column| column == REVISION)
        .map_or(1, |place| place + 1);
    let pinned = shown.columns.iter().position(|&column| column == TIME);
    let reveal = std::mem::take(&mut dialog.reveal);
    let mut table = DataTable::new(
        "workbench.project.revision_history",
        &columns,
        shown.rows.len(),
    )
    .state(&mut dialog.table)
    .keys(&shown.keys)
    .freeze(frozen)
    .max_height(height)
    .reveal_selection(reveal);
    if let Some(pinned) = pinned {
        table = table.pin_when_narrow(pinned);
    }
    let response = table.show(ui, |index| {
        row_cells(&rows[shown.rows[index]], &shown.columns)
    });
    if response.selection_changed || response.clicked.is_some() {
        dialog.restore_confirmation &= !response.selection_changed;
        dialog.error = None;
    }
    response
}

/// The pads either side of the words of the column at `place` of `last + 1`:
/// the panel inset at the table's outer edges, the cell pad between.
fn edge_pads(place: usize, last: usize) -> f32 {
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
    lead + trail
}

/// The selected record below the divider, or on a phone's page of its own:
/// what it is compared with and Restore, then the comparison, which scrolls
/// inside the room left. Returns whether the reader confirmed a restore.
fn record_pane(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    model: &Model,
    blocked: Option<&str>,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> bool {
    refresh_comparison(dialog, model, rows);
    let Some(row) = selected_row(dialog, rows) else {
        band_header(ui, "No record selected", None);
        inset(ui, |ui| {
            note(
                ui,
                "Select a revision to see what changed since it and to restore it, or another record to see what it keeps.",
            );
        });
        return false;
    };
    let comparison = dialog.comparison.clone();
    let (title, meta) = match &comparison {
        Some(ProjectRevisionComparison::Schematic { title, delta, .. }) => (
            (*title).to_owned(),
            (!delta.is_empty()).then(|| counted(delta.changes.len(), "change", "changes")),
        ),
        Some(ProjectRevisionComparison::RetainedMetadata(compared)) => (
            "Differences from the record compared".to_owned(),
            (!compared.fields.is_empty())
                .then(|| counted(compared.fields.len(), "difference", "differences")),
        ),
        Some(ProjectRevisionComparison::Unavailable(_)) => {
            ("Comparison unavailable".to_owned(), None)
        }
        None => (format!("{} record", row.kind.label()), None),
    };
    band_header(ui, &title, meta.as_deref());
    let restore = inset(ui, |ui| {
        if let Some(ProjectRevisionComparison::Schematic { sides, .. }) = &comparison {
            note(ui, sides);
        }
        record_controls(ui, row, rows, model, blocked, dialog)
    });
    match &comparison {
        Some(ProjectRevisionComparison::Schematic {
            delta,
            with_current,
            ..
        }) => {
            if delta.is_empty() {
                inset(ui, |ui| {
                    note(
                        ui,
                        if *with_current {
                            "Nothing differs: the current design is this revision's drawing."
                        } else {
                            "Nothing differs: both revisions keep the same drawing."
                        },
                    );
                });
            } else {
                named_changes(ui, delta, &mut dialog.changes);
            }
        }
        Some(ProjectRevisionComparison::RetainedMetadata(compared)) => {
            if compared.fields.is_empty() {
                inset(ui, |ui| {
                    note(ui, "Nothing differs: both records keep the same facts.");
                });
            } else {
                facts_table(
                    ui,
                    "workbench.project.revision_history.differences",
                    &["Fact", "This record", "Compared record"],
                    compared
                        .fields
                        .iter()
                        .map(|(field, first, second)| vec![*field, first.as_str(), second.as_str()])
                        .collect(),
                );
            }
        }
        Some(ProjectRevisionComparison::Unavailable(reason)) => {
            inset(ui, |ui| note(ui, reason));
        }
        None => facts_table(
            ui,
            "workbench.project.revision_history.record",
            &["Fact", "Value"],
            record_facts(row),
        ),
    }
    restore
}

/// What the selected record is compared with, and Restore with its
/// confirmation, in one row where they fit and one above the other where
/// they do not; then why Restore cannot act, or what it is about to do. A
/// record that keeps no drawing has no Restore, and one with nothing of its
/// kind to compare with no choice.
fn record_controls(
    ui: &mut Ui,
    row: &ProjectAuditRow,
    rows: &[ProjectAuditRow],
    model: &Model,
    blocked: Option<&str>,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> bool {
    #[derive(Clone, Copy)]
    enum Press {
        Restore,
        Confirm,
        Cancel,
    }
    let (first, others) = compare_choices(row, rows);
    let drawn = row.schematic_revision.is_some();
    let restore = match blocked {
        Some(reason) => Err(reason.to_owned()),
        None => restore_target(row, model),
    };
    let confirming = dialog.restore_confirmation && restore.is_ok();
    let buttons: Vec<(&str, Result<(), &str>, Press)> = if !drawn {
        Vec::new()
    } else if confirming {
        vec![
            ("Confirm restore", Ok(()), Press::Confirm),
            ("Cancel", Ok(()), Press::Cancel),
        ]
    } else {
        vec![(
            "Restore\u{2026}",
            restore.as_ref().map(|_| ()).map_err(String::as_str),
            Press::Restore,
        )]
    };
    let choosing = drawn || !others.is_empty();
    let options = std::iter::once(first.to_owned())
        .chain(others.iter().map(|(_, named)| named.clone()))
        .collect::<Vec<_>>();
    let chosen = dialog
        .compare_with
        .as_ref()
        .and_then(|key| others.iter().position(|(other, _)| other.key == *key))
        .map_or(0, |place| place + 1);

    let t = Tokens::get(ui.ctx());
    let spacing = ui.spacing().item_spacing.x;
    let label = "Compare with";
    let label_width = words_width(ui.painter(), label, false);
    // As wide as its longest choice in the select's own face, with the
    // well's lead and the chevron's room, so no choice is cut.
    let select_width = options
        .iter()
        .map(|option| {
            ui.painter()
                .layout_no_wrap(option.clone(), theme::field_sans(), Color32::PLACEHOLDER)
                .size()
                .x
        })
        .fold(0.0, f32::max)
        + SELECT_CHROME;
    let buttons_width = buttons
        .iter()
        .map(|(words, _, _)| Button::new(words).measured_width(ui))
        .sum::<f32>()
        + spacing * buttons.len().saturating_sub(1) as f32;
    let width = ui.available_width();
    let choice_width = if choosing {
        label_width + spacing + select_width
    } else {
        0.0
    };
    let one_row = choice_width + spacing + buttons_width <= width;

    let mut picked = None;
    let mut pressed = None;
    let mut choice = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            let room = (ui.available_width()).max(1.0);
            picked = select(
                ui,
                "project-revision-compare-with",
                label,
                &options[chosen],
                &options,
                select_width.min(room),
            );
        });
    };
    let mut actions = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            for (words, gate, press) in &buttons {
                let response = Button::new(words).enabled(gate.is_ok()).show(ui);
                if response.clicked() {
                    pressed = Some(*press);
                }
                if let Err(reason) = gate {
                    let _ = response.on_disabled_hover_text(*reason);
                }
            }
        });
    };
    if one_row {
        ui.horizontal(|ui| {
            if choosing {
                choice(ui);
            }
            if !buttons.is_empty() {
                // Restore sits at the row's trailing edge.
                ui.add_space((ui.available_width() - buttons_width).max(0.0));
                actions(ui);
            }
        });
    } else {
        if choosing {
            choice(ui);
        }
        if !buttons.is_empty() {
            actions(ui);
        }
    }
    if let Some(place) = picked {
        dialog.compare_with = place
            .checked_sub(1)
            .and_then(|place| others.get(place))
            .map(|(other, _)| other.key.clone());
    }
    if drawn && let Some(words) = restore_note(row, &restore, confirming, blocked.is_some()) {
        note(ui, &words);
    }
    match pressed {
        Some(Press::Restore) => {
            dialog.restore_confirmation = true;
            false
        }
        Some(Press::Confirm) => true,
        Some(Press::Cancel) => {
            dialog.restore_confirmation = false;
            false
        }
        None => false,
    }
}

/// What changed between two drawings, object by object, grouped by the
/// part of the drawing it is in: only the parts that changed, each group's
/// head counting its rows. The table runs edge to edge and scrolls inside
/// the room left.
fn named_changes(ui: &mut Ui, delta: &ValidatedRevisionSemanticDelta, state: &mut TableState) {
    let mut changes = delta.changes.iter().collect::<Vec<_>>();
    changes.sort_by_key(|change| (change.domain, change.kind));
    let mut groups = Vec::new();
    for (index, change) in changes.iter().enumerate() {
        if index == 0 || changes[index - 1].domain != change.domain {
            groups.push(Group::new(change.domain.label(), index));
        }
    }
    let kinds = ["Added", "Changed", "Removed"].map(|words| (words, false));
    let kind_width = fixed_width(ui, "Change", kinds, TABLE_EDGE_INSET, CELL_PAD) + SORT_MARK_ROOM;
    let height = ui.available_height();
    let columns = [
        Column::words("Change", kind_width),
        Column::name("Object", LEAST_CUT_WIDTH),
    ];
    let _ = DataTable::new(
        "workbench.project.revision_history.changes",
        &columns,
        changes.len(),
    )
    .state(state)
    .groups(&groups)
    .max_height(height)
    .show(ui, |index| {
        let change = changes[index];
        vec![
            Cell::text(change.kind.label()),
            Cell::text(object_words(change)),
        ]
    });
}

/// A changed object as its row states it: its name, and for a changed one
/// what changed in it.
fn object_words(change: &ValidatedRevisionChange) -> String {
    if change.detail.is_empty() {
        change.name.clone()
    } else {
        format!("{}: {}", change.name, change.detail)
    }
}

/// A table of facts: a first column naming each and the rest flexible, each
/// value cut at a word with the whole of it on hover, edge to edge and
/// scrolling inside the room left. A fact a record does not keep is the
/// absent mark.
fn facts_table(ui: &mut Ui, id: &str, heads: &[&str], rows: Vec<Vec<&str>>) {
    let names = rows.iter().map(|row| (row[0], false));
    let name_width = fixed_width(ui, heads[0], names, TABLE_EDGE_INSET, CELL_PAD) + SORT_MARK_ROOM;
    let columns = std::iter::once(Column::words(heads[0], name_width))
        .chain(
            heads[1..]
                .iter()
                .map(|head| Column::name(*head, LEAST_CUT_WIDTH)),
        )
        .collect::<Vec<_>>();
    let height = ui.available_height();
    let _ = DataTable::new(id, &columns, rows.len())
        .max_height(height)
        .show(ui, |index| {
            let row = &rows[index];
            std::iter::once(Cell::text(row[0]))
                .chain(row[1..].iter().map(|value| {
                    if value.is_empty() {
                        Cell::absent().hint(NOT_RECORDED)
                    } else {
                        Cell::text(*value)
                    }
                }))
                .collect()
        });
}

/// What a record keeps, fact by fact, for its own table.
fn record_facts(row: &ProjectAuditRow) -> Vec<Vec<&str>> {
    vec![
        vec!["Kind", row.kind.label()],
        vec!["Revision", &row.revision],
        vec!["Time", &row.time],
        vec!["Actor", &row.actor],
        vec!["Change", &row.change],
        vec!["Affected records", &row.affected_evidence],
        vec!["Status", &row.status],
    ]
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

/// Work out what the list shows again when the rows, the filter or the
/// search moved since it last was, and bring the selection back into view.
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
    dialog.reveal = true;
}

/// The rows the dialog keeps, built again only when what they were built
/// from has moved since; the selected record's comparison is worked out
/// again with them.
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
        dialog.compared = None;
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
        );
    }
    append_simulation_plan_rows(&mut rows, model);
    append_model_and_pdk_rows(&mut rows, model);
    append_governed_rows(&mut rows, model);
    rows
}

/// One row per revision of `view`'s journal, newest first. `drawn` is the
/// digest of the view's drawing as it stands, asked only of a view with
/// revisions to hold it against.
fn append_schematic_rows(
    rows: &mut Vec<ProjectAuditRow>,
    journal: &ValidatedRevisionJournal,
    drawn: impl FnOnce() -> Option<ContentDigest>,
    view: Option<CellViewRef>,
) {
    if journal.records().is_empty() {
        return;
    }
    let drawn = drawn();
    for record in journal.records().iter().rev() {
        let current = drawn == Some(record.design_content_digest());
        // The drawing as it stands is this revision's: what the reader most
        // needs to know of it, before which kind of revision it is.
        let status = if current {
            "Current"
        } else if record.is_accepted_baseline() {
            "Accepted"
        } else {
            "Validated"
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
            current,
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
            format!("Cloned from {source} at revision {}", source_revision.get())
        }
        _ => "Root plan".to_owned(),
    };
    ProjectAuditRow {
        key: format!("simulation-plan:{id}:{revision}"),
        kind: ProjectAuditArtifactKind::SimulationPlan,
        revision,
        time: String::new(),
        made_unix_ms: None,
        age: String::new(),
        actor: String::new(),
        change: name.to_owned(),
        affected_evidence: format!(
            "{}{SEPARATOR}{lineage_evidence}",
            counted(analysis_count, "analysis", "analyses")
        ),
        status: if active { "Active" } else { "Retained" }.to_owned(),
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
            time: String::new(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Project metadata transaction".to_owned(),
            change: format!("Attached technology {}", binding.display_label()),
            affected_evidence: [
                counted(binding.model_count(), "model", "models"),
                counted(
                    binding.source_closure().len(),
                    "pinned source",
                    "pinned sources",
                ),
                counted(
                    binding.process_sections().len(),
                    "process section",
                    "process sections",
                ),
            ]
            .join(SEPARATOR),
            status: "Attached".to_owned(),
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
                "Pinned",
                format!(
                    "Project source {source_id}{SEPARATOR}{}",
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
                        "Pinned",
                        format!("External source{SEPARATOR}root digest {digest}"),
                    ),
                    None => (
                        "Unversioned".to_owned(),
                        "Not pinned",
                        "External source with no accepted closure".to_owned(),
                    ),
                }
            }
            ModelSourceAuthority::RetainedImport { source_id, digest } => (
                short_text(&digest.to_string()),
                "Pinned",
                format!(
                    "Retained import {source_id}{SEPARATOR}root digest {}",
                    short_text(&digest.to_string())
                ),
            ),
            ModelSourceAuthority::BuiltIn => (
                nonempty_or(&library.version, "Built-in"),
                "Built-in",
                "Signed application catalog".to_owned(),
            ),
        };
        rows.push(ProjectAuditRow {
            key: format!("model-library:{}:{revision}", library.name),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision,
            time: String::new(),
            made_unix_ms: None,
            age: String::new(),
            actor: authority,
            change: if library.pdk_name.trim().is_empty() {
                format!("Model library {}", library.name)
            } else {
                format!(
                    "Model library {}{SEPARATOR}{}",
                    library.name, library.pdk_name
                )
            },
            affected_evidence: [
                counted(library.model_count(), "model", "models"),
                counted(library.corner_count(), "corner", "corners"),
                counted(
                    library.source_closure.len(),
                    "pinned source",
                    "pinned sources",
                ),
            ]
            .join(SEPARATOR),
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
            time: String::new(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Annotation policy transaction".to_owned(),
            change: format!(
                "Committed annotation policy revision {}",
                entry.policy_revision()
            ),
            affected_evidence: [
                counted(entry.mappings().len(), "mapping", "mappings"),
                format!("policy {}", short_text(&entry.policy_digest().to_string())),
                format!(
                    "receipt {}",
                    short_text(&entry.semantic_digest().to_string())
                ),
            ]
            .join(SEPARATOR),
            status: "Committed".to_owned(),
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
            time: String::new(),
            made_unix_ms: None,
            age: String::new(),
            actor: "Hierarchy audit transaction".to_owned(),
            change: "Hierarchy audit receipt".to_owned(),
            affected_evidence: [
                counted(
                    receipt.resolved_subjects(),
                    "resolved subject",
                    "resolved subjects",
                ),
                counted(receipt.findings().len(), "finding", "findings"),
                format!(
                    "receipt {}",
                    short_text(&receipt.semantic_digest().to_string())
                ),
            ]
            .join(SEPARATOR),
            status: if receipt.passed() {
                "Passed"
            } else {
                "Has findings"
            }
            .to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        });
    }
}

/// The history list's columns, in order, by their places below.
const COLUMNS: [&str; 7] = [
    "Kind",
    "Revision",
    "Time",
    "Actor",
    "Change",
    "Affected records",
    "Status",
];
const KIND: usize = 0;
const REVISION: usize = 1;
const TIME: usize = 2;
/// The columns whose identifiers and times are set in mono.
const MONO_COLUMNS: [bool; 7] = [false, true, true, false, false, false, false];
/// The columns a list too wide for its room never cuts: what a record is,
/// when it was made and its status are read whole or not at all.
const WHOLE_COLUMNS: [bool; 7] = [true, true, true, false, false, false, true];
/// The narrowest a column with too much to say is cut to. A list that
/// cannot fit with its widest columns cut that far keeps every column whole
/// and scrolls sideways under its first ones, as on a phone.
const LEAST_CUT_WIDTH: f32 = 96.0;

/// One row's text in column `column` of [`COLUMNS`].
fn row_text(row: &ProjectAuditRow, column: usize) -> &str {
    match column {
        KIND => row.kind.label(),
        REVISION => &row.revision,
        TIME => &row.time,
        3 => &row.actor,
        4 => &row.change,
        5 => &row.affected_evidence,
        _ => &row.status,
    }
}

/// One row's cells in the `shown` columns: identifiers and times in mono,
/// the time with how long ago it was on hover and sorted by the moment, and
/// a fact the record does not keep the absent mark. The table cuts words too
/// long for their column at a word or a list's separator, whole on hover.
fn row_cells(row: &ProjectAuditRow, shown: &[usize]) -> Vec<Cell> {
    shown
        .iter()
        .map(|&column| {
            let text = row_text(row, column);
            match (column, row.made_unix_ms) {
                (TIME, Some(made)) => Cell::number(text)
                    .hint(&row.age)
                    .sort_by(SortKey::Number(made as f64)),
                _ if text.is_empty() => Cell::absent().hint(NOT_RECORDED),
                _ if MONO_COLUMNS[column] => Cell::number(text),
                _ => Cell::text(text),
            }
        })
        .collect()
}

/// The columns, by index, that any of `rows` says something in, and the
/// kind of record only when the rows are of more than one kind. A column no
/// row fills (times the project never retained) would be a heading over
/// absent marks.
fn shown_columns(rows: &[ProjectAuditRow]) -> Vec<usize> {
    let mixed = rows.iter().any(|row| row.kind != rows[0].kind);
    (0..COLUMNS.len())
        .filter(|&column| match column {
            KIND => mixed,
            _ => rows.iter().any(|row| !row_text(row, column).is_empty()),
        })
        .collect()
}

/// How wide each `shown` column is to hold its widest words and its head
/// with the sort mark, with the padding the table puts either side of a
/// cell's words: the panel inset at the table's outer edges.
fn natural_widths(ui: &Ui, rows: &[ProjectAuditRow], shown: &[usize]) -> Vec<f32> {
    let last = shown.len().saturating_sub(1);
    shown
        .iter()
        .enumerate()
        .map(|(place, &column)| {
            let words = rows
                .iter()
                .map(|row| words_width(ui.painter(), row_text(row, column), MONO_COLUMNS[column]))
                .fold(head_width(ui, COLUMNS[column]) + SORT_MARK_ROOM, f32::max);
            (words + edge_pads(place, last) + 2.0).ceil()
        })
        .collect()
}

/// How wide `text` is in a table cell's face, mono or sans.
fn words_width(painter: &Painter, text: &str, mono: bool) -> f32 {
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    painter
        .layout_no_wrap(text.to_owned(), font, Color32::PLACEHOLDER)
        .size()
        .x
}

/// How wide a column head's words are.
fn head_width(ui: &Ui, head: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(
            head.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            Color32::PLACEHOLDER,
        )
        .size()
        .x
}

/// How wide a fixed column is to hold its head and `words`, each mono or
/// not, whole, with `lead` and `trail` of padding either side.
fn fixed_width<'a>(
    ui: &Ui,
    head: &str,
    words: impl IntoIterator<Item = (&'a str, bool)>,
    lead: f32,
    trail: f32,
) -> f32 {
    let widest = words
        .into_iter()
        .map(|(text, mono)| words_width(ui.painter(), text, mono))
        .fold(head_width(ui, head), f32::max);
    (widest + lead + trail + 2.0).ceil()
}

/// Natural column widths fitted to `room`. Room to spare widens every column
/// in proportion. Too little is taken from the widest columns alone that
/// are not `whole`: they are cut to one common width, as wide as still
/// fits, and every column narrower than that, or whole, keeps its whole
/// text. A list that would cut a column below [`LEAST_CUT_WIDTH`] keeps its
/// natural widths and scrolls.
fn fit_widths(natural: &[f32], whole: &[bool], room: f32) -> Vec<f32> {
    let total = natural.iter().sum::<f32>().max(1.0);
    if total <= room {
        let scale = room / total;
        return natural
            .iter()
            .map(|width| (width * scale).floor())
            .collect();
    }
    let kept = natural
        .iter()
        .zip(whole)
        .filter(|(_, whole)| **whole)
        .map(|(width, _)| width)
        .sum::<f32>();
    let mut sorted = natural
        .iter()
        .zip(whole)
        .filter(|(_, whole)| !**whole)
        .map(|(width, _)| *width)
        .collect::<Vec<_>>();
    sorted.sort_by(f32::total_cmp);
    let mut below = kept;
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
    natural
        .iter()
        .zip(whole)
        .map(|(width, whole)| {
            if *whole {
                *width
            } else {
                width.min(cap).floor()
            }
        })
        .collect()
}

/// The history table's key for a validated schematic revision, which its
/// selection holds.
pub(crate) fn schematic_row_key(id: ValidatedSchematicRevisionId) -> String {
    format!("schematic:{}", id.as_uuid())
}

fn short_digest(record: &ValidatedSchematicRevision) -> String {
    short_text(&record.revision_digest().to_string())
}

/// A schematic revision as a comparison names it: its digest and when it
/// was made.
fn revision_label(record: &ValidatedSchematicRevision) -> String {
    format!(
        "{} ({})",
        short_digest(record),
        crate::time_compat::utc_stamp(record.created_unix_ms())
    )
}

/// When a record was made, as the list's Time column writes it; nothing
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

/// `count` of a thing, named in the singular for one.
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
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
        parts.push(counted(
            record.dependency_count(),
            "dependency",
            "dependencies",
        ));
    }
    if findings.advisories > 0 {
        parts.push(counted(
            findings.advisories as usize,
            "advisory",
            "advisories",
        ));
    }
    if findings.blockers > 0 {
        parts.push(counted(findings.blockers as usize, "blocker", "blockers"));
    }
    if parts.len() == 1 {
        parts.push(format!("project revision {}", record.project_revision()));
    }
    parts.join(SEPARATOR)
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
        ComponentType, Point, PreflightReport, ValidatedRevisionChangeKind,
        ValidatedRevisionDomain, ValidatedRevisionRequest, ValidationFindingCounts,
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
            time: String::new(),
            made_unix_ms: None,
            age: String::new(),
            actor: String::new(),
            change: change.to_owned(),
            affected_evidence: String::new(),
            status: "Retained".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        }
    }

    /// Every string the body painted after it settled at `width`.
    fn painted_body(
        width: f32,
        rows: &[ProjectAuditRow],
        model: &Model,
        dialog: &mut ProjectRevisionHistoryDialogState,
    ) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 700.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE)
                            .show(ctx, |ui| {
                                let _ = revision_history_body(ui, rows, model, None, false, dialog);
                            });
                    },
                )
                .shapes;
        }
        shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }

    /// The selected record's comparison, worked out as the dialog does.
    fn compared(
        dialog: &mut ProjectRevisionHistoryDialogState,
        model: &Model,
        rows: &[ProjectAuditRow],
    ) -> ProjectRevisionComparison {
        refresh_comparison(dialog, model, rows);
        dialog.comparison.clone().expect("a comparison")
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
        assert_eq!(
            delta.count(
                ValidatedRevisionDomain::Components,
                ValidatedRevisionChangeKind::Added
            ),
            1
        );

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

    /// Each revision states the date and time it was made, not an age alone;
    /// the revision selected is compared with the design as it stands,
    /// named by that date and time, and what changed since it is named
    /// object by object.
    #[test]
    fn a_revision_states_when_it_was_made_and_what_changed_since() {
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
        assert!(texts.contains(&made), "{made:?} is not listed: {texts:?}");
        assert!(
            !texts.iter().any(|text| text == "just now"),
            "an age stands for the time: {texts:?}"
        );

        // The baseline, which the resistor moved on from.
        dialog.stand_on(schematic_row_key(records[0].id()));
        let ProjectRevisionComparison::Schematic {
            title,
            sides,
            delta,
            with_current,
        } = compared(&mut dialog, &harness.model, &rows)
        else {
            panic!("no drawing comparison: {:?}", dialog.comparison);
        };
        assert!(with_current);
        assert_eq!(title, "Changes to the current design");
        let baseline = crate::time_compat::utc_stamp(records[0].created_unix_ms());
        assert!(sides.contains(&baseline), "{sides:?}");
        let added = delta
            .changes
            .iter()
            .find(|change| change.kind == ValidatedRevisionChangeKind::Added)
            .expect("the resistor is named as added");
        assert_eq!(added.domain, ValidatedRevisionDomain::Components);
        let texts = painted_body(1200.0, &rows, &harness.model, &mut dialog);
        for said in [title, &sides, &object_words(added), "Components"] {
            assert!(texts.iter().any(|text| text == said), "{said:?}: {texts:?}");
        }
    }

    /// A revision compared with another of its view states the changes from
    /// the older to the newer, whichever is selected; the choice offers the
    /// design as it stands and the view's other revisions, and no record of
    /// another kind. Records of another kind are compared fact by fact.
    #[test]
    fn a_record_is_compared_with_the_one_chosen() {
        let mut harness = harness();
        append(&mut harness.model.schematic, "Baseline");
        harness
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut harness.model.schematic, "Add resistor");
        harness
            .model
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(40, 20));
        append(&mut harness.model.schematic, "Add capacitor");
        let records = harness
            .model
            .schematic
            .document()
            .validated_revisions
            .records()
            .to_vec();
        let rows = project_audit_rows(&harness.model);
        let row = |id: ValidatedSchematicRevisionId| {
            rows.iter()
                .find(|row| row.key == schematic_row_key(id))
                .unwrap()
        };
        let (first, others) = compare_choices(row(records[0].id()), &rows);
        assert_eq!(first, "Current design");
        assert_eq!(
            others
                .iter()
                .map(|(other, _)| other.key.clone())
                .collect::<Vec<_>>(),
            vec![
                schematic_row_key(records[2].id()),
                schematic_row_key(records[1].id())
            ]
        );

        for (selected, chosen) in [(0, 2), (2, 0)] {
            let mut dialog = ProjectRevisionHistoryDialogState::default();
            dialog.stand_on(schematic_row_key(records[selected].id()));
            refresh_comparison(&mut dialog, &harness.model, &rows);
            dialog.compare_with = Some(schematic_row_key(records[chosen].id()));
            let ProjectRevisionComparison::Schematic {
                title,
                sides,
                delta,
                with_current,
            } = compared(&mut dialog, &harness.model, &rows)
            else {
                panic!("no drawing comparison: {:?}", dialog.comparison);
            };
            assert!(!with_current);
            assert_eq!(title, "Changes between two revisions");
            let older = sides
                .find(&revision_label(&records[0]))
                .expect("the older is named");
            let newer = sides
                .find(&revision_label(&records[2]))
                .expect("the newer is named");
            assert!(older < newer, "{sides:?}");
            assert_eq!(
                delta.count(
                    ValidatedRevisionDomain::Components,
                    ValidatedRevisionChangeKind::Added
                ),
                2
            );
            // Another selection starts again from the design as it stands.
            dialog.stand_on(schematic_row_key(records[1].id()));
            refresh_comparison(&mut dialog, &harness.model, &rows);
            assert_eq!(dialog.compare_with, None);
        }

        let mut plans = [
            metadata_row(
                "simulation-plan:one",
                ProjectAuditArtifactKind::SimulationPlan,
                "Bias sweep",
            ),
            metadata_row(
                "simulation-plan:two",
                ProjectAuditArtifactKind::SimulationPlan,
                "Bias sweep",
            ),
        ];
        plans[1].status = "Active".to_owned();
        assert_eq!(compare_choices(&plans[0], &plans).0, "No comparison");
        assert!(comparison_of(&plans[0], None, &harness.model).is_none());
        let Some(ProjectRevisionComparison::RetainedMetadata(facts)) =
            comparison_of(&plans[0], Some(&plans[1]), &harness.model)
        else {
            panic!("the plans are not compared");
        };
        assert_eq!(
            facts.fields,
            vec![
                (
                    "Revision",
                    "simulation-plan:one".to_owned(),
                    "simulation-plan:two".to_owned()
                ),
                ("Status", "Retained".to_owned(), "Active".to_owned()),
            ]
        );
    }

    /// Restore says why it cannot act on the selected record: one that
    /// keeps no drawing, a revision the drawing already matches; and while
    /// it asks for confirmation, what it replaces.
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
        let shown = revision
            .view
            .as_ref()
            .map(CellViewRef::display_path)
            .unwrap();
        assert!(revision.current, "nothing changed since the revision");
        let refused = restore_target(revision, &harness.model);
        assert_eq!(
            refused,
            Err(format!("{shown} already matches this revision."))
        );
        assert_eq!(
            restore_note(revision, &refused, false, false),
            Some(format!("{shown} already matches this revision."))
        );
        assert_eq!(restore_note(revision, &refused, false, true), None);
        assert_eq!(
            restore_target(&plan, &harness.model),
            Err(NOT_A_DRAWING.to_owned())
        );
        let confirmed = Ok((revision.schematic_revision.unwrap(), None));
        assert!(
            restore_note(revision, &confirmed, true, false).is_some_and(|note| note.starts_with(
                &format!(
                    "Restore replaces the drawing of {shown} with revision {}",
                    revision.revision
                )
            ))
        );
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
            restore_target(row, &harness.model),
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
