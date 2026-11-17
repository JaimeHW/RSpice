//! The Overview's Cells card: every cell of the project, the top cell first,
//! then the testbenches its configurations run, then those with unsaved
//! changes, then the rest; each row opens what it names.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{Cell, Column, DataTable, SortKey, TableState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectPage;

use super::super::page::{CardLink, list_card};
use super::snapshot::{CellRowSnapshot, OverviewSnapshot};
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
    let columns = [
        Column::name("Cell", 120.0),
        Column::words("Library", 112.0),
        Column::words("Role", 84.0),
        Column::name("Views", 120.0),
        Column::number("Instances", 84.0),
    ];
    let response = DataTable::new(
        "workbench.project.overview.cells",
        &columns,
        snapshot.cell_row_count(),
    )
    .state(state)
    .freeze(1)
    .max_height(room)
    .show(ui, |index| {
        snapshot
            .cell_row(index)
            .map_or_else(|| vec![Cell::empty(); 5], row_cells)
    });
    // A click opens the row, as a link does; Enter does from the keyboard.
    response
        .activate
        .or(response.clicked)
        .and_then(|index| snapshot.cell_row(index))
        .map(|row| row.intent.clone())
}

fn row_cells(row: &CellRowSnapshot) -> Vec<Cell> {
    let name = Cell::text(&row.name);
    vec![
        // Unsaved is the editor tab's neutral dot, not a warning: nothing is
        // wrong with a cell being edited.
        if row.unsaved { name.unsaved() } else { name },
        Cell::text(&row.library),
        Cell::text(row.role),
        Cell::text(&row.views),
        row.instances.map_or_else(
            || Cell::absent().sort_by(SortKey::Missing),
            |count| Cell::number(count.to_string()).sort_by(SortKey::Number(count as f64)),
        ),
    ]
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
