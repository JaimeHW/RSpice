//! Revision history of a Verilog-A file: every revision of its cell's files
//! the project keeps, newest first, and restoring one as a new revision.
//!
//! It opens from the file's menu on the Files rail. Choosing a revision is
//! the dialog's own; restoring rewrites the cell's files, so the shell
//! carries it out once the dialog has drawn.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Ui};

use crate::state::ProjectSourceBundle;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize, band_header,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::{LifecycleAction, SourceDocumentRequest};
use crate::workbench::design_system::inspector::{muted_inspector_copy, toned_inspector_copy};
use crate::workbench::design_system::{Fact, fact_rows, property_row_height, section_block_gap};
use crate::workbench::documents::code_workspace::{self, SourceEditingState};
use crate::workbench::{MessageCatalog, MessageId};

use super::source_document::{dialog_inset, leaf_name, sections};

/// The gap between two columns of a revision's row.
const COLUMN_GAP: f32 = 16.0;

/// One revision the project keeps of a cell's files.
struct RevisionRow {
    revision: u64,
    /// The start of the revision's content digest: two revisions with the
    /// same digest hold the same files.
    digest: String,
    /// The revision's files: its main file, then the files beside it.
    files: Vec<String>,
    /// Whether the cell's files are this revision now.
    current: bool,
}

/// The revisions `bundle` keeps, newest first.
fn revision_rows(bundle: &ProjectSourceBundle) -> Vec<RevisionRow> {
    bundle
        .revision_history()
        .iter()
        .rev()
        .map(|snapshot| RevisionRow {
            revision: snapshot.revision().get(),
            digest: snapshot
                .closure_digest()
                .to_string()
                .chars()
                .take(12)
                .collect(),
            files: std::iter::once(snapshot.root().logical_path())
                .chain(snapshot.files().iter().map(|file| file.logical_path()))
                .map(str::to_owned)
                .collect(),
            current: snapshot.closure_digest() == bundle.closure_digest(),
        })
        .collect()
}

/// Draw the revision history open on the Code workspace's runtime state,
/// over the model and the platform's state. Returns the restore it asked
/// for, which the shell carries out.
pub(super) fn render(
    code: &mut SourceEditingState,
    model: &Model,
    platform: &PlatformState,
    messages: MessageCatalog,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    let history = code.source_history.as_ref()?;
    let Some(bundle) = model
        .workspace
        .content
        .project_sources
        .get_bundle(history.bundle_id)
    else {
        // The cell's files went away under the open history.
        code.source_history = None;
        return None;
    };
    let rows = revision_rows(bundle);
    let source_path = history.source_path.clone();
    let selected = history.selected_revision;
    let error = history.error.clone();
    let held =
        code_workspace::source_file_mutation_block_reason_in(model, code, platform, messages);

    let dialog = Dialog::new(
        messages.format(
            MessageId::CodeSourceHistoryTitle,
            &[("file", &leaf_name(&source_path))],
        ),
        messages.text(MessageId::CommonClose),
    )
    .description(messages.text(MessageId::CodeSourceHistoryDescription))
    .size(DialogSize::Confirmation)
    .flush_body()
    // The rows fit at every desktop size; a bar would only reserve a gutter
    // that left the right edge wider than the left.
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl);

    let mut chosen = None;
    let mut restore = false;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        sections(ui, |ui| {
            band_header(
                ui,
                &messages.text(MessageId::CodeSourceRetainedClosures),
                (!rows.is_empty())
                    .then(|| rows.len().to_string())
                    .as_deref(),
            );
            // The rows run edge to edge, as the band over them does; their
            // words keep the dialog's inset.
            let columns = Columns::measure(ui, &rows, messages);
            for row in &rows {
                let picked = selected == Some(row.revision);
                if revision_row(ui, row, &columns, picked, messages).clicked() {
                    chosen = Some(row.revision);
                }
            }
            dialog_inset(ui, |ui| {
                restore = restore_controls(
                    ui,
                    &rows,
                    selected,
                    held.as_deref(),
                    error.as_deref(),
                    messages,
                );
            });
        });
        None
    });

    if let Some(revision) = chosen
        && let Some(history) = code.source_history.as_mut()
    {
        history.selected_revision = Some(revision);
        history.error = None;
    }
    if restore {
        return Some(SourceDocumentRequest::Document {
            action: LifecycleAction::Restore,
            logical_path: source_path,
        });
    }
    if matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
    ) {
        code.source_history = None;
    }
    None
}

/// Under the rows: the selected revision's files, why a restore failed, and
/// Restore. A history with nothing but the current revision says so instead.
/// Returns whether Restore was pressed.
fn restore_controls(
    ui: &mut Ui,
    rows: &[RevisionRow],
    selected: Option<u64>,
    held: Option<&str>,
    error: Option<&str>,
    messages: MessageCatalog,
) -> bool {
    if !rows.iter().any(|row| !row.current) {
        muted_inspector_copy(ui, &messages.text(MessageId::CodeSourceNoRetainedRevision));
        return false;
    }
    let picked = rows.iter().find(|row| selected == Some(row.revision));
    if let Some(row) = picked {
        let label = messages.text(MessageId::CodeNavigatorFiles);
        let files = row
            .files
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        fact_rows(ui, &[Fact::path(&label, &files)]);
    }
    if let Some(error) = error {
        toned_inspector_copy(ui, error, Tokens::get(ui.ctx()).color.err);
    }
    section_block_gap(ui);
    // What a restore of the selection would be refused for, said on the
    // button before the press.
    let refusal = match picked {
        None => Some(messages.text(MessageId::CodeSourceHistoryPickRevision)),
        Some(row) if row.current => Some(messages.text(MessageId::CodeSourceHistoryAlreadyCurrent)),
        Some(_) => held.map(str::to_owned),
    };
    let response = Button::new(&messages.text(MessageId::CodeSourceRestoreAsNewRevision))
        .enabled(refusal.is_none())
        .show(ui);
    let pressed = response.clicked();
    if let Some(reason) = refusal {
        response.on_disabled_hover_text(reason);
    }
    pressed
}

/// Where each column of the rows starts, from the row's left edge: every
/// row's revision, file count and digest stand in one column with the
/// others'.
struct Columns {
    files: f32,
    digest: f32,
}

impl Columns {
    fn measure(ui: &Ui, rows: &[RevisionRow], messages: MessageCatalog) -> Self {
        let font = row_font();
        let widest = |text: &dyn Fn(&RevisionRow) -> String| {
            rows.iter()
                .map(|row| {
                    ui.painter()
                        .layout_no_wrap(text(row), font.clone(), egui::Color32::PLACEHOLDER)
                        .size()
                        .x
                })
                .fold(0.0_f32, f32::max)
        };
        let revision = widest(&|row| revision_label(row, messages));
        let files = widest(&|row| files_label(row, messages));
        let files_at = DIALOG_BODY_INSET + revision + COLUMN_GAP;
        Self {
            files: files_at,
            digest: files_at + files + COLUMN_GAP,
        }
    }
}

fn row_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Regular)
}

fn revision_label(row: &RevisionRow, messages: MessageCatalog) -> String {
    messages.format(
        MessageId::CodeSourceRevision,
        &[("revision", &row.revision.to_string())],
    )
}

fn files_label(row: &RevisionRow, messages: MessageCatalog) -> String {
    messages.format(
        if row.files.len() == 1 {
            MessageId::CodeSourceDocumentSingular
        } else {
            MessageId::CodeSourceDocuments
        },
        &[("count", &row.files.len().to_string())],
    )
}

/// One revision, flush to the dialog's edges: its number, how many files it
/// holds, its digest, and whether it is the current one. A press selects it.
fn revision_row(
    ui: &mut Ui,
    row: &RevisionRow,
    columns: &Columns,
    selected: bool,
    messages: MessageCatalog,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = row_font();
    let height = property_row_height(ui);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    let revision = revision_label(row, messages);
    let files = files_label(row, messages);
    // The row is painted, so nothing here reaches a screen reader on its
    // own; the row says all of it.
    let announced = messages.format(
        if row.current {
            MessageId::CodeSourceRevisionAccessibleCurrent
        } else {
            MessageId::CodeSourceRevisionAccessible
        },
        &[
            ("revision", &row.revision.to_string()),
            ("digest", &row.digest),
            ("documents", &files),
        ],
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &announced,
        )
    });
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if selected {
            painter.rect_filled(rect, 0.0, t.color.accent_dim);
        }
        let y = rect.center().y;
        painter.text(
            egui::pos2(rect.left() + DIALOG_BODY_INSET, y),
            egui::Align2::LEFT_CENTER,
            &revision,
            font.clone(),
            t.color.text,
        );
        painter.text(
            egui::pos2(rect.left() + columns.files, y),
            egui::Align2::LEFT_CENTER,
            &files,
            font.clone(),
            t.color.text_dim,
        );
        let mut right = rect.right() - DIALOG_BODY_INSET;
        if row.current {
            let current = painter.text(
                egui::pos2(right, y),
                egui::Align2::RIGHT_CENTER,
                messages.text(MessageId::CodeSourceCurrentClosure),
                font,
                t.color.text_dim,
            );
            right = current.left() - COLUMN_GAP;
        }
        let left = rect.left() + columns.digest;
        painter
            .with_clip_rect(egui::Rect::from_x_y_ranges(
                left..=right.max(left),
                rect.y_range(),
            ))
            .text(
                egui::pos2(left, y),
                egui::Align2::LEFT_CENTER,
                &row.digest,
                theme::mono(tokens::FS_1, FontWeight::Regular),
                t.color.text_dim,
            );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}
