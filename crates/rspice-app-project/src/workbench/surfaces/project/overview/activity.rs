//! The Overview's Activity card: every run, validated revision, library
//! publication and checkpoint the project keeps, newest first, each row
//! opening what it names; and the words its rows are made of.

use egui::Ui;

use crate::ui::widgets::table::{Cell, Column, DataTable, SortKey, TableState};
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
    // The detail column ends with a run's engine, so a narrow table gives the
    // engine up first; the elapsed time has a column of its own.
    let columns = [
        Column::words("When", 140.0),
        Column::name("Operation", 150.0),
        Column::number("Elapsed", 72.0),
        Column::name("Detail", 120.0),
    ];
    let operations = &snapshot.operations;
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
