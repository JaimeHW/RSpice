//! Open result document: every saved result document of the project, the
//! most recent first, each with the run and analysis it reads, to bring one
//! forward as a tab of Results.

use egui::{Context, Id, Ui};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{
    Cell, Column, DataTable, LineHeights, RowKey, TableState, fitted_width, least_width,
};
use rspice_ui_kit::widgets::{
    DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize, FILTER_CLEAR_INSET,
    filter_field_clear, name_control,
};

/// The form's memory: the list's selection and the filter typed over it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpenDraft {
    pub table: TableState,
}

/// One saved document as the form lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenRow {
    /// The document's identity, which keeps the selection on it while the
    /// list changes.
    pub key: String,
    pub title: String,
    /// The run its first pane reads, as the data browser names it.
    pub run: String,
    /// The analysis that pane draws.
    pub analysis: String,
}

impl OpenDraft {
    /// The row the reader chose, by its place among the rows the host
    /// listed.
    #[must_use]
    pub fn chosen(&self) -> Option<usize> {
        self.table.selected_row()
    }
}

const TITLE: &str = "Open result document";
const LIST_ID: &str = "rspice.open-result-document.rows";
const FILTER_LABEL: &str = "Filter documents";
/// The most rows the list shows before it scrolls inside the form.
const ROWS_IN_VIEW: f32 = 12.0;

/// Draw the form over `rows`. A double-click or Enter on a row opens it, as
/// Open does with the row selected.
pub fn show(ctx: &Context, draft: &mut OpenDraft, rows: &[OpenRow]) -> DialogChoice {
    let columns = columns(ctx, rows);
    // The surface's 1 pt border and the body's inset on either side.
    let width = least_width(&columns) + 2.0 * (1.0 + DIALOG_BODY_INSET);
    let filter = Id::new("rspice.open-result-document.filter");
    let keys = rows
        .iter()
        .map(|row| RowKey::Text(row.key.clone()))
        .collect::<Vec<_>>();
    let mut activated = false;
    let choice = Dialog::new(TITLE, "Open")
        .size(DialogSize::Transaction)
        .width_from_content(width)
        .initial_focus(DialogInitialFocus::Control(filter))
        .primary_enabled(draft.chosen().is_some())
        .overlay_scrollbar()
        .show_with_initial_body_focus(ctx, |ui| {
            filter_field(ui, filter, &mut draft.table.filter);
            ui.add_space(6.0);
            if rows.is_empty() {
                ui.label("The project has no saved result document yet.");
                return Some(filter);
            }
            let row_height = LineHeights::list(&Tokens::get(ui.ctx())).row;
            let response = DataTable::new(LIST_ID, &columns, rows.len())
                .state(&mut draft.table)
                .keys(&keys)
                .max_height(row_height * (ROWS_IN_VIEW + 1.0))
                .show(ui, |index| {
                    let row = &rows[index];
                    vec![
                        Cell::text(row.title.as_str()),
                        Cell::text(row.run.as_str()),
                        Cell::text(row.analysis.as_str()),
                    ]
                });
            activated = response.activate.is_some();
            Some(filter)
        });
    if activated && draft.chosen().is_some() {
        DialogChoice::Primary
    } else {
        choice
    }
}

/// Each column as wide as the widest thing in it, the name taking whatever
/// the form has left.
fn columns(ctx: &Context, rows: &[OpenRow]) -> [Column; 3] {
    // A `Ui` of its own, for the font cache alone: nothing is allocated in it
    // and nothing is painted from it.
    let probe = Ui::new(
        ctx.clone(),
        Id::new("rspice.open-result-document.measure"),
        egui::UiBuilder::new()
            .max_rect(ctx.content_rect())
            .invisible(),
    );
    let fitted = |head: &str, cell: fn(&OpenRow) -> &str| {
        let cells = rows
            .iter()
            .map(|row| Cell::text(cell(row)))
            .collect::<Vec<_>>();
        fitted_width(&probe, head, &cells)
    };
    [
        Column::name("Name", fitted("Name", |row| &row.title).max(160.0)),
        Column::words("Run", fitted("Run", |row| &row.run)),
        Column::words("Analysis", fitted("Analysis", |row| &row.analysis)),
    ]
}

/// The filter over the list, as wide as the form.
fn filter_field(ui: &mut Ui, id: Id, filter: &mut String) {
    let field = ui.add(
        egui::TextEdit::singleline(filter)
            .id(id)
            .desired_width(f32::INFINITY)
            .margin(egui::Margin {
                left: 6,
                right: FILTER_CLEAR_INSET,
                top: 4,
                bottom: 4,
            })
            .hint_text("Filter by name, run or analysis\u{2026}"),
    );
    name_control(ui, &field, FILTER_LABEL);
    filter_field_clear(ui, &field, filter);
}
