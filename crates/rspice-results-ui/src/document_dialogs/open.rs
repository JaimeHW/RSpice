//! Open result document: every saved result document of the project, the
//! most recently changed first, each with the run and analysis it reads and
//! when it last changed, to bring one forward as a tab of Results.

use egui::{Context, Id, Ui};
use rspice_ui_kit::panels::{WorkbenchIcon, empty_state};
use rspice_ui_kit::theme;
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, LineHeights, RowKey, SortKey, TABLE_EDGE_INSET, TableState,
    fitted_width,
};
use rspice_ui_kit::widgets::{
    DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize, FILTER_CLEAR_INSET,
    filter_field_clear,
};

/// The form's memory: the list's selection and the words typed over it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpenDraft {
    pub table: TableState,
    /// Rows none of whose words hold this, case aside, are left out.
    pub filter: String,
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
    /// When it last changed, in milliseconds since the Unix epoch; zero for
    /// a document that carries no time.
    pub modified_ms: u64,
    /// That time as the reader's clock writes it, empty for none.
    pub modified: String,
}

impl OpenRow {
    /// Whether any of the row's words holds `needle`, already lower-cased.
    fn holds(&self, needle: &str) -> bool {
        [&self.title, &self.run, &self.analysis, &self.modified]
            .iter()
            .any(|words| words.to_lowercase().contains(needle))
    }
}

impl OpenDraft {
    /// The identity of the row the reader chose.
    #[must_use]
    pub fn chosen(&self) -> Option<&str> {
        match self.table.selected_key()? {
            RowKey::Text(key) => Some(key),
            RowKey::Number(_) => None,
        }
    }
}

const TITLE: &str = "Open result document";
const LIST_ID: &str = "rspice.open-result-document.rows";
const FILTER_HINT: &str = "Filter documents\u{2026}";
/// The most rows the list shows before it scrolls inside the form.
const ROWS_IN_VIEW: usize = 12;
/// The room each column but the last keeps after its widest entry, so no
/// column's words run into the next one's.
const COLUMN_GAP: f32 = 16.0;
/// The least height the list keeps, so it holds the kit's empty state whole
/// when nothing matches.
const EMPTY_HEIGHT: f32 = 132.0;

/// Draw the form over `rows`, the most recently changed first. A
/// double-click or Enter on a row opens it, as Open does with the row
/// selected; the first row the filter keeps is selected until the reader
/// picks another, so Enter opens it at once.
pub fn show(ctx: &Context, draft: &mut OpenDraft, rows: &[OpenRow]) -> DialogChoice {
    // The columns fit every row, not just the ones the filter keeps, so the
    // form holds its width while the reader types.
    let (columns, columns_width) = columns(ctx, rows);
    // The surface's 1 pt border and the body's inset on either side.
    let width = columns_width + 2.0 * (1.0 + DIALOG_BODY_INSET);
    let needle = draft.filter.trim().to_lowercase();
    let shown = rows
        .iter()
        .filter(|row| needle.is_empty() || row.holds(&needle))
        .collect::<Vec<_>>();
    let keys = shown
        .iter()
        .map(|row| RowKey::Text(row.key.clone()))
        .collect::<Vec<_>>();
    keep_a_row_chosen(draft, &shown, &keys);
    let filter = Id::new("rspice.open-result-document.filter");
    let mut activated = false;
    let choice = Dialog::new(TITLE, "Open")
        .description(
            "Every saved result document of the project, the most recently changed first; \
             the one opened becomes a tab of Results.",
        )
        .size(DialogSize::Transaction)
        .width_from_content(width)
        .overlay_scrollbar()
        .initial_focus(DialogInitialFocus::Control(filter))
        .primary_enabled(draft.chosen().is_some())
        .show_with_initial_body_focus(ctx, |ui| {
            filter_field(ui, filter, &mut draft.filter);
            ui.add_space(6.0);
            let row_height = LineHeights::list(&Tokens::get(ui.ctx())).row;
            // The list keeps one height whatever the filter keeps: that of
            // every row, and never less than the empty state's, so the form
            // keeps the size it opened with while the reader types.
            let height = (row_height * (rows.len().min(ROWS_IN_VIEW) + 1) as f32).max(EMPTY_HEIGHT);
            let list = ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                ui.set_min_height(height);
                if shown.is_empty() {
                    let (title, description) = if rows.is_empty() {
                        (
                            "No saved result document",
                            "New result document\u{2026} keeps a view of a run as one.".to_owned(),
                        )
                    } else {
                        (
                            "No document matches",
                            format!(
                                "No name, run, analysis or time holds \u{201c}{}\u{201d}.",
                                draft.filter.trim()
                            ),
                        )
                    };
                    empty_state(ui, WorkbenchIcon::Search, title, &description);
                    return None;
                }
                Some(
                    DataTable::new(LIST_ID, &columns, shown.len())
                        .state(&mut draft.table)
                        .keys(&keys)
                        .max_height(row_height * (ROWS_IN_VIEW + 1) as f32)
                        .show(ui, |index| {
                            let row = shown[index];
                            vec![
                                Cell::text(row.title.as_str()),
                                Cell::text(row.run.as_str()),
                                Cell::text(row.analysis.as_str()),
                                modified_cell(row),
                            ]
                        }),
                )
            });
            activated = list
                .inner
                .is_some_and(|response| response.activate.is_some());
            Some(filter)
        });
    if activated && draft.chosen().is_some() {
        DialogChoice::Primary
    } else {
        choice
    }
}

/// Keep the selection on a row the filter shows: the one the reader picked
/// while it shows, else the first, and none while no row shows.
fn keep_a_row_chosen(draft: &mut OpenDraft, shown: &[&OpenRow], keys: &[RowKey]) {
    let held = draft
        .chosen()
        .and_then(|key| shown.iter().position(|row| row.key == key));
    match (held, keys.first()) {
        (Some(_), _) => {}
        (None, Some(first)) => draft.table.select(0, Some(first.clone())),
        (None, None) => {
            draft.table.selection = None;
            draft.table.others.clear();
            draft.table.anchor = None;
        }
    }
}

/// When the row's document last changed, in the reader's clock, sorted by
/// the time itself; a dash for a document that carries none.
fn modified_cell(row: &OpenRow) -> Cell {
    if row.modified.is_empty() {
        Cell::absent()
    } else {
        Cell::number(row.modified.as_str()).sort_by(SortKey::Number(row.modified_ms as f64))
    }
}

/// Every column as wide as its widest entry, with a gap after it but for
/// the time at the table's edge, the name taking any width the form has to
/// spare; and the width of them all.
fn columns(ctx: &Context, rows: &[OpenRow]) -> ([Column; 4], f32) {
    // A `Ui` of its own, for the font cache alone: nothing is allocated in it
    // and nothing is painted from it.
    let probe = Ui::new(
        ctx.clone(),
        Id::new("rspice.open-result-document.measure"),
        egui::UiBuilder::new()
            .max_rect(ctx.content_rect())
            .invisible(),
    );
    let fitted = |head: &str, cell: fn(&OpenRow) -> Cell| {
        let cells = rows.iter().map(cell).collect::<Vec<_>>();
        fitted_width(&probe, head, &cells)
    };
    // The table holds its outer columns' words off its edges by more than a
    // cell's padding.
    let edge = TABLE_EDGE_INSET - CELL_PAD;
    let name = fitted("Name", |row| Cell::text(row.title.as_str())) + edge + COLUMN_GAP;
    let run = fitted("Run", |row| Cell::text(row.run.as_str())) + COLUMN_GAP;
    let analysis = fitted("Analysis", |row| Cell::text(row.analysis.as_str())) + COLUMN_GAP;
    let modified = fitted("Modified", modified_cell) + edge;
    (
        [
            Column::name("Name", name),
            Column::words("Run", run),
            Column::words("Analysis", analysis),
            Column::words("Modified", modified),
        ],
        name + run + analysis + modified,
    )
}

/// The filter over the list, as wide as the form: the house search field,
/// its magnifier at the left and its clear at the right.
fn filter_field(ui: &mut Ui, id: Id, filter: &mut String) {
    let t = Tokens::get(ui.ctx());
    let clearable = !filter.is_empty();
    let field = ui.add_sized(
        [ui.available_width(), t.metrics.ctl_h],
        egui::TextEdit::singleline(filter)
            .id(id)
            .hint_text(FILTER_HINT)
            .font(theme::field_sans())
            .margin(egui::Margin {
                left: 29,
                right: if clearable { FILTER_CLEAR_INSET } else { 8 },
                top: 5,
                bottom: 5,
            }),
    );
    ui.ctx().accesskit_node_builder(field.id, |node| {
        node.set_label(FILTER_HINT);
        node.set_description("Filter by name, run, analysis or time");
    });
    WorkbenchIcon::Search.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(field.rect.left() + 15.0, field.rect.center().y),
            egui::vec2(16.0, 16.0),
        ),
        t.color.text_faint,
    );
    filter_field_clear(ui, &field, filter);
}
