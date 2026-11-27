//! The data browser's saved result documents: a group above the runs, one
//! row per document, each picking that document for Results to show, as a
//! run's analysis row picks the run.

use egui::{Response, Ui};

use super::super::super::design_system::WorkbenchIcon;
use super::results_rows::result_browser_row_height;
use crate::product::ResultDocumentId;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::chrome::document_bar::results::{self as documents, ListedDocument};
use crate::workbench::documents::result_document::ResultsParts;

/// Where the group's disclosure is kept for the session.
const GROUP_ID: &str = "workbench.results.browser-documents";
/// How far in a document's row starts: one level, as a run's analyses do.
const ROW_INSET: f32 = 28.0;
/// The side of the row's document glyph.
const GLYPH: f32 = 14.0;
/// The unsaved marker, as the document strips draw it.
const DIRTY: f32 = 6.0;

/// Whether the documents group is open: it is until the reader closes it.
pub(super) fn group_expanded(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp::<bool>(egui::Id::new(GROUP_ID)))
        .unwrap_or(true)
}

fn set_group_expanded(ctx: &egui::Context, expanded: bool) {
    ctx.data_mut(|data| data.insert_temp(egui::Id::new(GROUP_ID), expanded));
}

/// What the group's head asks for once drawn: a click, or Left and Right
/// while it has the focus, closes or opens the group. A query holds it
/// open, since every document it lists matches.
pub(super) fn documents_head_actions(head: &Response, expanded: bool, filtering: bool) {
    if filtering {
        return;
    }
    if head.clicked() {
        set_group_expanded(&head.ctx, !expanded);
    }
    if head.has_focus() {
        let (collapse, expand) = head.ctx.input_mut(|input| {
            (
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft),
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight),
            )
        });
        if (collapse && expanded) || (expand && !expanded) {
            set_group_expanded(&head.ctx, expand);
        }
    }
}

/// One saved document's row: its name, in the accent while Results shows
/// it, and the unsaved marker while it holds edits the project has not
/// saved.
pub(super) fn document_row(ui: &mut Ui, document: &ListedDocument, shown: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), result_browser_row_height(&t)),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            shown,
            &document.title,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(if document.dirty {
            "Saved result document, unsaved changes"
        } else {
            "Saved result document"
        });
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, c.bg_hover);
    }
    let glyph_left = rect.left() + ROW_INSET;
    WorkbenchIcon::Results.paint(
        ui.painter(),
        egui::Rect::from_min_size(
            egui::pos2(glyph_left, rect.center().y - GLYPH * 0.5),
            egui::Vec2::splat(GLYPH),
        ),
        if shown { c.accent } else { c.text_dim },
    );
    let text_left = glyph_left + GLYPH + 6.0;
    let mut text_right = rect.right() - 8.0;
    if document.dirty {
        let center = egui::pos2(text_right - DIRTY * 0.5, rect.center().y);
        ui.painter().circle_filled(center, DIRTY * 0.5, c.text_dim);
        text_right -= DIRTY + 8.0;
    }
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let title = super::super::super::design_system::elide_text(
        ui,
        &document.title,
        &font,
        text_right - text_left,
    );
    let shortened = title != document.title;
    ui.painter().text(
        egui::pos2(text_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        font,
        if shown { c.accent } else { c.text },
    );
    theme::paint_focus_ring(ui, &response, rect);
    if shortened {
        response.on_hover_text(&document.title)
    } else {
        response
    }
}

/// What a document's row asks for once drawn: a click shows the document,
/// and its menu renames or deletes it. Deleting asks nothing first, unsaved
/// edits or not: it is a step Undo takes back.
pub(super) fn document_row_actions(
    state: &mut ResultsParts<'_>,
    document: &ListedDocument,
    shown: bool,
    response: &Response,
) {
    if response.clicked() && !shown {
        show(state, document.id);
    }
    let refusal = state
        .model
        .safe_mode
        .project_read_only()
        .then_some("the project is open read-only");
    let mut rename = false;
    let mut delete = false;
    response.context_menu(|ui| {
        rename = ui
            .add_enabled(refusal.is_none(), egui::Button::new("Rename\u{2026}"))
            .on_disabled_hover_text(refusal.unwrap_or_default())
            .clicked();
        delete = ui
            .add_enabled(refusal.is_none(), egui::Button::new("Delete"))
            .on_disabled_hover_text(refusal.unwrap_or_default())
            .clicked();
        if rename || delete {
            ui.close();
        }
    });
    if rename && show(state, document.id) {
        documents::open_rename(state);
    }
    if delete && show(state, document.id) {
        let said = documents::delete_shown_document(state)
            .unwrap_or_else(|error| format!("Could not delete {}: {error}", document.title));
        state.workbench.prompt(said);
    }
}

/// Show the saved document `id`, as picking its row does.
fn show(state: &mut ResultsParts<'_>, id: ResultDocumentId) -> bool {
    documents::shown_document(state.results_reads()) == Some(id)
        || documents::show_visualization_document(state, &id)
}
