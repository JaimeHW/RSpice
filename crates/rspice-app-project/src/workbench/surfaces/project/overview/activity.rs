//! The Overview's Activity card: every run, validated revision, library
//! publication and checkpoint the project keeps, newest first, each row
//! opening what it names; and the words its rows are made of.

use egui::{Color32, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens;
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, SortKey, TABLE_EDGE_INSET, TableState,
};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::state::Workspace;

use super::super::page::{CardLink, list_card};
use super::cells::empty_sentence;
use super::snapshot::{OperationSnapshot, OverviewSnapshot};
use super::{OverviewIntent, command_gate};

/// The Activity card, `height` tall when it `fill`s and at most that when
/// it does not: its table, and "All runs", which opens Results.
pub(super) fn activity_card(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    snapshot: &OverviewSnapshot,
    state: &mut TableState,
    (height, fill): (f32, bool),
) -> Option<OverviewIntent> {
    let results = ids::open_workspace(Workspace::Results);
    let (enabled, disabled_reason) = command_gate(cx, results);
    let (opened, all) = list_card(
        ui,
        "Activity",
        &CardLink {
            label: "All runs",
            enabled,
            disabled_reason,
        },
        height,
        fill,
        |ui, room| activity_table(ui, snapshot, state, room),
    );
    if all {
        Some(OverviewIntent::Command(results))
    } else {
        opened
    }
}

fn activity_table(
    ui: &mut Ui,
    snapshot: &OverviewSnapshot,
    state: &mut TableState,
    room: f32,
) -> Option<OverviewIntent> {
    if snapshot.operations.is_empty() {
        // The revision and whether it is saved are the header's and the
        // Unsaved work row's to say.
        empty_sentence(ui, "No runs, validated revisions or checkpoints yet");
        return None;
    }
    let operations = &snapshot.operations;
    // The detail column ends with a run's engine, so a narrow table gives the
    // engine up first; the elapsed time has a column of its own. A time and a
    // duration are never cut: their columns are as wide as the widest one.
    let (when, elapsed) = fitted_widths(ui, operations, snapshot.now_ms);
    let columns = [
        Column::words("When", when),
        Column::name("Operation", 150.0),
        Column::number("Elapsed", elapsed),
        Column::name("Detail", 120.0),
    ];
    let response = DataTable::new(
        "workbench.project.overview.activity",
        &columns,
        operations.len(),
    )
    .state(state)
    .freeze(1)
    .pin_when_narrow(1)
    .max_height(room)
    .show(ui, |index| row_cells(&operations[index], snapshot.now_ms));
    response
        .activate
        .or(response.clicked)
        .and_then(|index| operations.get(index))
        .and_then(|operation| operation.intent.clone())
}

fn row_cells(operation: &OperationSnapshot, now_ms: u64) -> Vec<Cell> {
    let when = operation.when(now_ms).map_or_else(
        || Cell::absent().sort_by(SortKey::Missing),
        |(short, full)| {
            let cell = Cell::number(&short);
            let cell = if short == full { cell } else { cell.hint(full) };
            cell.sort_by(SortKey::Number(
                operation.timestamp_ms.unwrap_or_default() as f64
            ))
        },
    );
    let event = operation.tone.problem().map_or_else(
        || Cell::text(&operation.event),
        |mark| Cell::problem(&operation.event, mark),
    );
    let elapsed = operation.elapsed.map_or_else(Cell::empty, |seconds| {
        Cell::number(crate::quantity::format_elapsed(seconds)).sort_by(SortKey::Number(seconds))
    });
    vec![when, event, elapsed, Cell::text(&operation.detail)]
}

/// The least a fitted column is made: room for its head and the sort mark
/// beside it.
const FITTED_HEAD_WIDTH: f32 = 72.0;

/// How wide the When and Elapsed columns are made: each as wide as the
/// widest value it shows, with the room the table leaves either side of a
/// cell's words. Both are written in mono, so the value with the most
/// characters is the widest.
fn fitted_widths(ui: &Ui, operations: &[OperationSnapshot], now_ms: u64) -> (f32, f32) {
    let widest = |values: &mut dyn Iterator<Item = String>| {
        values
            .max_by_key(|value| value.chars().count())
            .unwrap_or_default()
    };
    let when = widest(
        &mut operations
            .iter()
            .filter_map(|operation| operation.when(now_ms).map(|(short, _)| short)),
    );
    let elapsed = widest(
        &mut operations
            .iter()
            .filter_map(|operation| operation.elapsed.map(crate::quantity::format_elapsed)),
    );
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measure = |text: String| {
        ui.painter()
            .layout_no_wrap(text, font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    };
    (
        // The first column starts at the table's edge inset.
        (measure(when) + TABLE_EDGE_INSET + CELL_PAD + 2.0).max(FITTED_HEAD_WIDTH),
        (measure(elapsed) + 2.0 * CELL_PAD + 2.0).max(FITTED_HEAD_WIDTH),
    )
}

pub(super) fn run_title(sequence: u64, label: &str) -> String {
    let prefix = format!("Run {sequence}");
    if label.starts_with(&prefix) {
        label.to_owned()
    } else {
        format!("{prefix} · {label}")
    }
}

pub(super) fn unix_seconds_to_millis(seconds: f64) -> Option<u64> {
    if !seconds.is_finite() || seconds < 0.0 || seconds > u64::MAX as f64 / 1_000.0 {
        return None;
    }
    Some((seconds * 1_000.0).round() as u64)
}
