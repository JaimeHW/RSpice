//! The Overview's Cells card: every cell of the project, the top cell first,
//! then the testbenches its configurations run, then those with unsaved
//! changes, then the rest, each lot the most recently saved first; each row
//! opens what it names.

use egui::{Color32, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{CELL_PAD, Cell, Column, DataTable, SortKey, TableState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectPage;

use super::super::page::{CardLink, list_card};
use super::snapshot::{CellRowSnapshot, DAY_MS, OverviewSnapshot};
use super::{OverviewIntent, command_gate};

/// The Cells card, `height` tall when it `fill`s and at most that when it
/// does not: its table, and "All cells", which opens the Library page.
pub(super) fn cells_card(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    snapshot: &OverviewSnapshot,
    state: &mut TableState,
    (height, fill): (f32, bool),
) -> Option<OverviewIntent> {
    let library = ProjectCommand::Page(ProjectPage::Library).key();
    let (enabled, disabled_reason) = command_gate(cx, library);
    let (opened, all) = list_card(
        ui,
        "Cells",
        &CardLink {
            label: "All cells",
            enabled,
            disabled_reason,
        },
        height,
        fill,
        |ui, room| cells_table(ui, snapshot, state, room),
    );
    if all {
        Some(OverviewIntent::Command(library))
    } else {
        opened
    }
}

fn cells_table(
    ui: &mut Ui,
    snapshot: &OverviewSnapshot,
    state: &mut TableState,
    room: f32,
) -> Option<OverviewIntent> {
    if snapshot.cell_row_count() == 0 {
        empty_sentence(ui, "The project has no cells yet");
        return None;
    }
    // A cell, where it is, what it is to the project and when a save last
    // wrote it come first; the time is read in full, its column as wide as
    // the widest time it shows, as the Activity card's When is. A card too
    // narrow for every column's least width gives up whole columns from its
    // right, the instances and then the views, rather than scrolling
    // sideways with its edge through a word, as the Activity card does;
    // views given up are the cell's hint. The Cell column is as wide as the
    // names it lists, so a wide card gives its spare to Views, the last
    // column of words, and the instances stay beside them.
    let modified = modified_width(ui, snapshot);
    let name = name_width(ui, snapshot);
    let columns = [
        Column::words("Cell", name),
        Column::words("Library", 104.0),
        Column::words("Role", 84.0),
        Column::words("Modified", modified),
        Column::name("Views", 120.0),
        Column::number("Instances", 84.0),
    ];
    let least = [name, 104.0, 84.0, modified, 120.0, 84.0];
    let room_wide = ui.available_width();
    let shown = (KEPT_COLUMNS..=least.len())
        .rev()
        .find(|&count| least[..count].iter().sum::<f32>() <= room_wide)
        .unwrap_or(KEPT_COLUMNS);
    let response = DataTable::new(
        "workbench.project.overview.cells",
        &columns[..shown],
        snapshot.cell_row_count(),
    )
    .state(state)
    .freeze(1)
    .max_height(room)
    .show(ui, |index| {
        snapshot.cell_row(index).map_or_else(
            || vec![Cell::empty(); shown],
            |row| {
                let mut cells = row_cells(row, snapshot.now_ms);
                if shown <= VIEWS_COLUMN {
                    let name = cells.remove(0).hint(format!("Views: {}", row.views));
                    cells.insert(0, name);
                }
                cells.truncate(shown);
                cells
            },
        )
    });
    // A click opens the row, as a link does; Enter does from the keyboard.
    response
        .activate
        .or(response.clicked)
        .and_then(|index| snapshot.cell_row(index))
        .map(|row| row.intent.clone())
}

fn row_cells(row: &CellRowSnapshot, now_ms: u64) -> Vec<Cell> {
    let name = Cell::text(&row.name);
    let modified = match row.modified(now_ms) {
        Some((short, full)) => {
            let cell = Cell::number(&short);
            let cell = if short == full { cell } else { cell.hint(full) };
            cell.sort_by(SortKey::Number(row.saved_ms.unwrap_or_default() as f64))
        }
        // A cell made here and never saved: its edits are the newest there
        // are.
        None if row.unsaved => Cell::number("Unsaved").sort_by(SortKey::Number(now_ms as f64)),
        None => Cell::absent().sort_by(SortKey::Missing),
    };
    let library = Cell::text(&row.library);
    vec![
        // Unsaved is the editor tab's neutral dot, not a warning: nothing is
        // wrong with a cell being edited.
        if row.unsaved { name.unsaved() } else { name },
        match &row.library_hint {
            Some(hint) => library.hint(hint),
            None => library,
        },
        Cell::text(row.role),
        modified,
        Cell::text(&row.views),
        row.instances.map_or_else(
            || Cell::absent().sort_by(SortKey::Missing),
            |count| Cell::number(count.to_string()).sort_by(SortKey::Number(count as f64)),
        ),
    ]
}

/// The columns a narrow card keeps: the cell, its library, its role and
/// when it was saved.
const KEPT_COLUMNS: usize = 4;
/// Where the Views column stands.
const VIEWS_COLUMN: usize = 4;

/// The least the Modified column is made: room for its head and the sort
/// mark beside it, as wide as the Instances column.
const MODIFIED_HEAD_WIDTH: f32 = 84.0;

/// The least and the most the Cell column is made: room for its head and
/// sort mark, and room for a long name before the rest of the row is pushed
/// off a narrow card. A longer name is cut, and its row's hover names it.
const NAME_LEAST: f32 = 104.0;
const NAME_MOST: f32 = 280.0;
/// What the unsaved mark takes before a name.
const UNSAVED_MARK: f32 = 14.0;
/// How many of the longest names, by their count of characters, are
/// measured: in a proportional face the widest is among them, and a project
/// of thousands of cells is not laid out name by name on every frame.
const NAMES_MEASURED: usize = 16;

/// How wide the Cell column is made: as wide as the widest name it lists,
/// between [`NAME_LEAST`] and [`NAME_MOST`].
fn name_width(ui: &Ui, snapshot: &OverviewSnapshot) -> f32 {
    let mut longest = snapshot
        .cell_rows()
        .map(|row| (row.name.chars().count(), row.name.as_str()))
        .collect::<Vec<_>>();
    let measured = longest.len().min(NAMES_MEASURED);
    if measured == 0 {
        return NAME_LEAST;
    }
    longest.select_nth_unstable_by(measured - 1, |a, b| b.0.cmp(&a.0));
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let widest = longest[..measured]
        .iter()
        .map(|(_, name)| {
            ui.painter()
                .layout_no_wrap((*name).to_owned(), font.clone(), Color32::PLACEHOLDER)
                .size()
                .x
        })
        .fold(0.0, f32::max);
    (widest + 2.0 * CELL_PAD + UNSAVED_MARK + 2.0)
        .ceil()
        .clamp(NAME_LEAST, NAME_MOST)
}

/// How wide the Modified column is made: as wide as the widest time it
/// shows, written in mono. Inside a day a time is told as an age, of which
/// "59 min ago" is the longest; past one as its date and time, all of one
/// length; so the oldest saved time decides which.
fn modified_width(ui: &Ui, snapshot: &OverviewSnapshot) -> f32 {
    let oldest = snapshot.cell_rows().filter_map(|row| row.saved_ms).min();
    let widest = match oldest {
        Some(at) if snapshot.now_ms.saturating_sub(at) >= DAY_MS => {
            crate::time_compat::local_stamp(at)
        }
        Some(_) => "59 min ago".to_owned(),
        None => return MODIFIED_HEAD_WIDTH,
    };
    let words = ui
        .painter()
        .layout_no_wrap(
            widest,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            Color32::PLACEHOLDER,
        )
        .size()
        .x;
    (words + 2.0 * CELL_PAD + 2.0).max(MODIFIED_HEAD_WIDTH)
}

/// What a list card says when it has nothing to list: one sentence, wrapped
/// rather than cut.
pub(super) fn empty_sentence(ui: &mut Ui, sentence: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(14, 8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(sentence)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}
