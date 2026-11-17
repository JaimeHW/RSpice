//! The shared table's behaviour, measured: what it selects, sorts, folds and
//! filters, how the keyboard moves it, and where its heads, spans and cells
//! land as it scrolls. Geometry comes from the probes the table publishes in
//! test builds, never from pixels.

use std::cell::Cell as Counter;
use std::collections::BTreeSet;

use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect, vec2};

use super::geometry::Columns;
use super::state::{Line, Step, arrange, step};
use super::*;
use crate::probe::{Geometry as PageGeometry, pass as page_pass};

const WIDTH: f32 = 1100.0;
const HEIGHT: f32 = 720.0;

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    ctx
}

fn key(key: Key) -> Event {
    key_with(key, Modifiers::NONE)
}

fn key_with(key: Key, modifiers: Modifiers) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn press(pos: Pos2, button: PointerButton) -> Vec<Event> {
    let edge = |pressed| Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers: Modifiers::NONE,
    };
    vec![Event::PointerMoved(pos), edge(true), edge(false)]
}

fn click(pos: Pos2) -> Vec<Event> {
    press(pos, PointerButton::Primary)
}

/// One frame of whatever `draw` lays out, at `width`, over `events`.
fn frame(
    ctx: &egui::Context,
    width: f32,
    events: Vec<Event>,
    mut draw: impl FnMut(&mut egui::Ui) -> TableResponse,
) -> (TableResponse, PageGeometry, egui::FullOutput) {
    let mut response = TableResponse::default();
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(width, HEIGHT))),
        events,
        ..Default::default()
    };
    let (output, geometry) = page_pass(ctx, input, |ui| response = draw(ui));
    (response, geometry, output)
}

fn named_columns() -> Vec<Column> {
    vec![
        Column::name("Output", 160.0),
        Column::number("Value", 96.0),
        Column::words("Pass/fail", 70.0),
    ]
}

/// "Row n", its value n, and a failure on every third row.
fn named_row(row: usize) -> Vec<Cell> {
    vec![
        Cell::text(format!("Row {row}")),
        Cell::number(format!("{row}.0")).sort_by(SortKey::Number(row as f64)),
        if row.is_multiple_of(3) {
            Cell::text("Fail").toned(Tone::Error)
        } else {
            Cell::text("Pass").toned(Tone::Neutral)
        },
    ]
}

fn keys_for(rows: usize) -> Vec<RowKey> {
    (0..rows)
        .map(|row| RowKey::from(format!("r{row}")))
        .collect()
}

/// The rows a frame showed, by their index in the page's order, top first.
fn shown(geometry: &PageGeometry, rows: usize) -> Vec<usize> {
    let mut lines = (0..rows)
        .filter_map(|row| Some((geometry.one(&format!("table.line.{row}"))?.top(), row)))
        .collect::<Vec<_>>();
    lines.sort_by(|left, right| left.0.total_cmp(&right.0));
    lines.into_iter().map(|(_, row)| row).collect()
}

fn head(geometry: &PageGeometry, column: usize) -> Rect {
    geometry
        .one(&format!("table.head.{column}"))
        .unwrap_or_else(|| panic!("column {column}'s head is painted"))
}

fn cell(geometry: &PageGeometry, column: usize) -> Rect {
    geometry
        .one(&format!("table.cell.{column}"))
        .unwrap_or_else(|| panic!("column {column} is painted"))
}

/// A table of the named rows with state, keyed, clicked open on `row`.
struct Keyed {
    ctx: egui::Context,
    columns: Vec<Column>,
    keys: Vec<RowKey>,
    rows: usize,
    state: TableState,
    max_height: f32,
}

impl Keyed {
    fn new(rows: usize) -> Self {
        Self {
            ctx: context(),
            columns: named_columns(),
            keys: keys_for(rows),
            rows,
            state: TableState::default(),
            max_height: f32::INFINITY,
        }
    }

    fn frame(&mut self, events: Vec<Event>) -> (TableResponse, PageGeometry) {
        let Self {
            ctx,
            columns,
            keys,
            rows,
            state,
            max_height,
        } = self;
        let (response, geometry, _) = frame(ctx, WIDTH, events, |ui| {
            DataTable::new("keyed", columns, *rows)
                .state(state)
                .keys(keys)
                .max_height(*max_height)
                .show(ui, named_row)
        });
        (response, geometry)
    }

    /// Click row `row`'s first cell, which the table answers by selecting it
    /// and taking the keyboard.
    fn click_row(&mut self, row: usize) -> TableResponse {
        let (_, geometry) = self.frame(Vec::new());
        let line = geometry
            .one(&format!("table.line.{row}"))
            .expect("the row is in view");
        let (response, _) = self.frame(click(line.left_center() + vec2(20.0, 0.0)));
        let _ = self.frame(Vec::new());
        response
    }

    fn keys(&mut self, pressed: Key) -> (TableResponse, PageGeometry) {
        self.frame(vec![key(pressed)])
    }
}

#[test]
fn a_click_selects_the_row_by_index_and_key() {
    let mut table = Keyed::new(6);
    let response = table.click_row(2);
    assert!(response.selection_changed);
    assert_eq!(response.clicked, Some(2));
    assert_eq!(table.state.selected_row(), Some(2));
    assert_eq!(table.state.selected_key(), Some(&RowKey::from("r2")));
    let (_, geometry) = table.frame(Vec::new());
    assert_eq!(
        geometry.one("table.selected"),
        geometry.one("table.line.2"),
        "the selected row carries the wash"
    );
}

#[test]
fn the_arrow_keys_act_only_once_the_table_has_the_keyboard() {
    let mut table = Keyed::new(6);
    table.state.select(1, Some(RowKey::from("r1")));
    let (response, _) = table.keys(Key::ArrowDown);
    assert!(!response.selection_changed);
    assert_eq!(
        table.state.selected_row(),
        Some(1),
        "an unfocused table ignores keys"
    );

    let _ = table.click_row(1);
    let (response, _) = table.keys(Key::ArrowDown);
    assert!(response.selection_changed);
    assert_eq!(table.state.selected_row(), Some(2));
    assert_eq!(table.state.selected_key(), Some(&RowKey::from("r2")));
    let _ = table.keys(Key::ArrowUp);
    let _ = table.keys(Key::ArrowUp);
    let _ = table.keys(Key::ArrowUp);
    assert_eq!(
        table.state.selected_row(),
        Some(0),
        "Up stops at the first row"
    );
    let _ = table.keys(Key::ArrowDown);
    assert_eq!(
        table.state.selected_row(),
        Some(1),
        "the table kept the keyboard"
    );
}

#[test]
fn home_end_and_the_page_keys_move_by_the_page_and_bring_the_row_into_view() {
    let mut table = Keyed::new(100);
    // A head and ten rows.
    table.max_height = TABLE_HEAD_H + 10.0 * TABLE_ROW_H;
    let _ = table.click_row(0);

    let (_, geometry) = table.keys(Key::End);
    assert_eq!(table.state.selected_row(), Some(99));
    let head = geometry.one("table.head").expect("head");
    let body = head.bottom()..=head.bottom() + 10.0 * TABLE_ROW_H + 0.5;
    let selected = geometry
        .one("table.selected")
        .expect("the last row is in view");
    assert!(
        body.contains(&selected.top()) && body.contains(&selected.bottom()),
        "the last row shows whole: {selected:?} in {body:?}"
    );

    let _ = table.keys(Key::Home);
    assert_eq!(table.state.selected_row(), Some(0));
    let _ = table.keys(Key::PageDown);
    assert_eq!(
        table.state.selected_row(),
        Some(10),
        "a page is the rows in view"
    );
    let (_, geometry) = table.keys(Key::PageDown);
    assert_eq!(table.state.selected_row(), Some(20));
    let selected = geometry.one("table.selected").expect("row 20 is in view");
    assert!(selected.bottom() <= head.bottom() + 10.0 * TABLE_ROW_H + 0.5);
    let _ = table.keys(Key::PageUp);
    assert_eq!(table.state.selected_row(), Some(10));
}

/// A table whose second and third columns open for entry.
fn editable_row(row: usize) -> Vec<Cell> {
    vec![
        Cell::text(format!("Row {row}")),
        Cell::number("1.0").editable(),
        Cell::number("2.0").editable(),
    ]
}

#[test]
fn enter_or_f2_open_a_cell_tab_moves_on_and_escape_leaves_it() {
    let ctx = context();
    let columns = named_columns();
    let mut state = TableState::default();
    let mut text = String::from("1.0");
    let draw =
        |state: &mut TableState, open: Option<(usize, usize, bool)>, text: &mut String, events| {
            frame(&ctx, WIDTH, events, |ui| {
                DataTable::new("edit", &columns, 4)
                    .state(state)
                    .editing(open.map(|(row, column, focus)| CellEditor {
                        row,
                        column,
                        text: &mut *text,
                        focus,
                    }))
                    .show(ui, editable_row)
            })
            .0
        };
    let geometry = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("edit", &columns, 4)
            .state(&mut state)
            .show(ui, editable_row)
    })
    .1;
    let name = cell(&geometry, 0);
    let row_one = geometry.one("table.line.1").expect("row 1");
    let _ = draw(
        &mut state,
        None,
        &mut text,
        click(Pos2::new(name.center().x, row_one.center().y)),
    );
    let _ = draw(&mut state, None, &mut text, Vec::new());
    assert_eq!(state.selected_row(), Some(1));

    let response = draw(&mut state, None, &mut text, vec![key(Key::Enter)]);
    assert_eq!(
        response.edit,
        Some((1, 1)),
        "Enter opens the first editable cell"
    );
    let response = draw(&mut state, None, &mut text, vec![key(Key::F2)]);
    assert_eq!(response.edit, Some((1, 1)), "so does F2");

    let _ = draw(&mut state, Some((1, 1, true)), &mut text, Vec::new());
    let response = draw(
        &mut state,
        Some((1, 1, false)),
        &mut text,
        vec![key(Key::Tab)],
    );
    assert_eq!(response.edited, Some(EditOutcome::Commit), "Tab commits");
    assert_eq!(
        response.edit,
        Some((1, 2)),
        "and opens the next editable cell"
    );

    let _ = draw(&mut state, Some((1, 2, true)), &mut text, Vec::new());
    let response = draw(
        &mut state,
        Some((1, 2, false)),
        &mut text,
        vec![key(Key::Escape)],
    );
    assert_eq!(
        response.edited,
        Some(EditOutcome::Cancel),
        "Escape leaves it"
    );
    assert_eq!(response.edit, None);
    let _ = draw(&mut state, None, &mut text, Vec::new());
    let _ = draw(&mut state, None, &mut text, vec![key(Key::ArrowDown)]);
    assert_eq!(
        state.selected_row(),
        Some(2),
        "the closed cell handed the keyboard back to the table"
    );
}

#[test]
fn the_selection_follows_its_row_through_a_sort_and_a_refreshed_run() {
    let mut table = Keyed::new(9);
    let _ = table.click_row(4);
    let (_, geometry) = table.frame(Vec::new());
    let value_head = head(&geometry, 1);
    // Descending by value: two clicks.
    let _ = table.frame(click(value_head.center()));
    let _ = table.frame(click(value_head.center()));
    let (_, geometry) = table.frame(Vec::new());
    assert_eq!(shown(&geometry, 9), vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(table.state.selected_row(), Some(4), "a sort moves no index");
    assert_eq!(geometry.one("table.selected"), geometry.one("table.line.4"));

    // The run refreshed with its rows in reverse: the key finds row 4 at 4.
    let ctx = table.ctx.clone();
    let reversed = table.keys.iter().rev().cloned().collect::<Vec<_>>();
    let (response, _, _) = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("keyed", &table.columns, 9)
            .state(&mut table.state)
            .keys(&reversed)
            .show(ui, |row| named_row(8 - row))
    });
    assert!(!response.selection_changed);
    assert_eq!(table.state.selected_row(), Some(4));
    assert_eq!(table.state.selected_key(), Some(&RowKey::from("r4")));
    let shifted = keys_for(9)[1..].to_vec();
    let _ = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("keyed", &table.columns, 8)
            .state(&mut table.state)
            .keys(&shifted)
            .show(ui, |row| named_row(row + 1))
    });
    assert_eq!(
        table.state.selected_row(),
        Some(3),
        "the key moved up a row"
    );
    let gone = keys_for(4);
    let _ = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("keyed", &table.columns, 4)
            .state(&mut table.state)
            .keys(&gone)
            .show(ui, named_row)
    });
    assert_eq!(
        table.state.selection, None,
        "a row that left takes its selection"
    );
}

#[test]
fn a_head_click_sorts_stably_a_status_worst_first_and_the_third_restores_the_order() {
    let mut table = Keyed::new(7);
    let (_, geometry) = table.frame(Vec::new());
    let status = head(&geometry, 2);

    let _ = table.frame(click(status.center()));
    let (_, geometry) = table.frame(Vec::new());
    assert_eq!(
        shown(&geometry, 7),
        vec![0, 3, 6, 1, 2, 4, 5],
        "errors first, each tone in the page's order"
    );
    assert_eq!(
        table.state.sort,
        Some(Sort {
            column: 2,
            direction: SortDirection::Ascending
        })
    );
    let _ = table.frame(click(status.center()));
    let (_, geometry) = table.frame(Vec::new());
    assert_eq!(shown(&geometry, 7), vec![1, 2, 4, 5, 0, 3, 6]);
    let _ = table.frame(click(status.center()));
    let (_, geometry) = table.frame(Vec::new());
    assert_eq!(table.state.sort, None);
    assert_eq!(shown(&geometry, 7), (0..7).collect::<Vec<_>>());
}

#[test]
fn an_unsortable_column_and_a_table_without_state_keep_the_page_order() {
    let ctx = context();
    let mut columns = named_columns();
    columns[1] = Column::number("Value", 96.0).unsortable();
    let mut state = TableState::default();
    let geometry = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("plain", &columns, 5)
            .state(&mut state)
            .show(ui, named_row)
    })
    .1;
    let value = head(&geometry, 1);
    for _ in 0..2 {
        let _ = frame(&ctx, WIDTH, click(value.center()), |ui| {
            DataTable::new("plain", &columns, 5)
                .state(&mut state)
                .show(ui, named_row)
        });
    }
    assert_eq!(state.sort, None);

    let status = head(&geometry, 2);
    let stateless = |events| {
        frame(&ctx, WIDTH, events, |ui| {
            DataTable::new("stateless", &columns, 5).show(ui, named_row)
        })
        .1
    };
    let _ = stateless(click(status.center()));
    assert_eq!(shown(&stateless(Vec::new()), 5), (0..5).collect::<Vec<_>>());
}

#[test]
fn sort_keys_put_missing_values_last_either_way() {
    use std::cmp::Ordering::{Greater, Less};
    let missing = SortKey::Missing;
    let one = SortKey::Number(1.0);
    let nan = SortKey::Number(f64::NAN);
    for direction in [SortDirection::Ascending, SortDirection::Descending] {
        assert_eq!(missing.order(&one, direction), Greater);
        assert_eq!(nan.order(&one, direction), Greater);
        assert_eq!(one.order(&missing, direction), Less);
    }
    let error = SortKey::Status(Tone::Error.severity());
    let caution = SortKey::Status(Tone::Caution.severity());
    assert_eq!(error.order(&caution, SortDirection::Ascending), Less);
    assert_eq!(
        SortKey::Text("alpha".into())
            .order(&SortKey::Text("Beta".into()), SortDirection::Ascending),
        Less,
        "words compare without case"
    );
    assert_eq!(Cell::absent().sort_key(), SortKey::Missing);
    assert_eq!(
        Cell::text("Fail").toned(Tone::Error).sort_key(),
        SortKey::Status(Tone::Error.severity())
    );
}

fn groups() -> Vec<Group> {
    vec![Group::new("Typical", 0), Group::new("Slow", 3)]
}

#[test]
fn the_filter_hides_rows_and_the_groups_it_empties() {
    let ctx = context();
    let columns = named_columns();
    let groups = groups();
    let mut state = TableState {
        filter: "ROW 4".to_owned(),
        ..TableState::default()
    };
    let geometry = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("filtered", &columns, 6)
            .state(&mut state)
            .groups(&groups)
            .show(ui, named_row)
    })
    .1;
    assert_eq!(shown(&geometry, 6), vec![4]);
    assert!(
        geometry.one("table.group.Typical").is_none(),
        "an emptied group goes"
    );
    let slow = geometry
        .one("table.group.Slow")
        .expect("the group with a match stays");
    assert_eq!(
        geometry.one("table.line.4").map(|row| row.top()),
        Some(slow.bottom())
    );
}

#[test]
fn a_group_head_click_folds_it_and_the_fold_lives_in_the_state() {
    let ctx = context();
    let columns = named_columns();
    let groups = groups();
    let mut state = TableState::default();
    let draw = |state: &mut TableState, events| {
        frame(&ctx, WIDTH, events, |ui| {
            DataTable::new("grouped", &columns, 6)
                .state(state)
                .groups(&groups)
                .show(ui, named_row)
        })
        .1
    };
    let geometry = draw(&mut state, Vec::new());
    assert_eq!(shown(&geometry, 6), (0..6).collect::<Vec<_>>());
    let typical = geometry.one("table.group.Typical").expect("group head");
    assert_eq!(
        geometry.one("table.line.0").map(|row| row.top()),
        Some(typical.bottom())
    );

    let _ = draw(&mut state, click(typical.center()));
    assert!(state.is_folded("Typical"));
    let geometry = draw(&mut state, Vec::new());
    assert_eq!(
        shown(&geometry, 6),
        vec![3, 4, 5],
        "a folded group shows its head only"
    );
    assert!(geometry.one("table.group.Typical").is_some());

    // The page keeps the state between sessions: a copy folds the same.
    let mut restored = state.clone();
    let geometry = draw(&mut restored, Vec::new());
    assert_eq!(shown(&geometry, 6), vec![3, 4, 5]);
}

#[test]
fn rows_are_arranged_by_group_filter_and_a_stable_sort() {
    let groups = groups();
    let folded = BTreeSet::from(["Slow".to_owned()]);
    let lines = arrange(6, &groups, &folded, |row| row != 1, None);
    assert_eq!(
        lines,
        vec![
            Line::Group { group: 0, rows: 2 },
            Line::Row(0),
            Line::Row(2),
            Line::Group { group: 1, rows: 3 },
        ]
    );
    let keys = [2, 1, 2, 1, 0, 0];
    let order = |left: usize, right: usize| keys[left].cmp(&keys[right]);
    let lines = arrange(6, &[], &BTreeSet::new(), |_| true, Some(&order));
    assert_eq!(
        lines,
        [4, 5, 1, 3, 0, 2].map(Line::Row).to_vec(),
        "equal rows keep the page's order"
    );
    let ungrouped = arrange(
        4,
        &[Group::new("Late", 2)],
        &BTreeSet::new(),
        |_| true,
        None,
    );
    assert_eq!(
        ungrouped[..2],
        [Line::Row(0), Line::Row(1)],
        "rows before a group stand alone"
    );
}

#[test]
fn a_step_moves_among_the_rows_shown() {
    let shown = [4, 0, 2, 7];
    assert_eq!(step(&shown, Some(0), Step::Down, 2), Some(2));
    assert_eq!(step(&shown, Some(0), Step::Up, 2), Some(4));
    assert_eq!(step(&shown, Some(4), Step::Up, 2), Some(4));
    assert_eq!(step(&shown, Some(4), Step::PageDown, 2), Some(2));
    assert_eq!(step(&shown, Some(2), Step::Home, 2), Some(4));
    assert_eq!(step(&shown, Some(2), Step::End, 2), Some(7));
    assert_eq!(
        step(&shown, Some(9), Step::Down, 2),
        Some(4),
        "a hidden selection restarts"
    );
    assert_eq!(step(&shown, None, Step::End, 2), Some(7));
    assert_eq!(step(&[], Some(1), Step::Down, 2), None);
}

#[test]
fn ctrl_c_copies_the_selected_row_under_the_column_heads() {
    let mut table = Keyed::new(4);
    let _ = table.click_row(2);
    let ctx = table.ctx.clone();
    let (_, _, output) = frame(&ctx, WIDTH, vec![Event::Copy], |ui| {
        DataTable::new("keyed", &table.columns, 4)
            .state(&mut table.state)
            .keys(&table.keys)
            .show(ui, named_row)
    });
    let copied = output
        .platform_output
        .commands
        .iter()
        .find_map(|command| match command {
            egui::OutputCommand::CopyText(text) => Some(text.clone()),
            _ => None,
        });
    assert_eq!(
        copied.as_deref(),
        Some("Output\tValue\tPass/fail\nRow 2\t2.0\tPass")
    );
}

#[test]
fn a_double_click_and_a_secondary_click_report_the_cell_and_where() {
    let mut table = Keyed::new(4);
    let (_, geometry) = table.frame(Vec::new());
    let value = Pos2::new(
        cell(&geometry, 1).center().x,
        geometry.one("table.line.3").expect("row 3").center().y,
    );
    let mut events = click(value);
    events.extend(click(value));
    let (response, _) = table.frame(events);
    assert_eq!(
        response.double_clicked,
        Some(CellRef {
            row: 3,
            column: Some(1)
        })
    );
    let _ = table.frame(Vec::new());
    let (response, _) = table.frame(press(value, PointerButton::Secondary));
    let menu = response.context_menu.expect("a context menu request");
    assert_eq!(
        menu.cell,
        CellRef {
            row: 3,
            column: Some(1)
        }
    );
    assert_eq!(menu.position, value);
    assert_eq!(table.state.selected_row(), Some(3));

    let (response, _) = table.frame(vec![key_with(Key::F10, Modifiers::SHIFT)]);
    assert_eq!(
        response.context_menu.map(|menu| menu.cell.row),
        Some(3),
        "Shift+F10 asks for the selected row's menu"
    );
}

#[test]
fn cell_selection_follows_the_click_and_the_left_and_right_keys() {
    let ctx = context();
    let columns = named_columns();
    let mut state = TableState::default();
    let draw = |state: &mut TableState, events| {
        frame(&ctx, WIDTH, events, |ui| {
            DataTable::new("cells", &columns, 3)
                .state(state)
                .cell_selection()
                .show(ui, named_row)
        })
        .1
    };
    let geometry = draw(&mut state, Vec::new());
    let target = Pos2::new(
        cell(&geometry, 1).center().x,
        geometry.one("table.line.1").expect("row").center().y,
    );
    let _ = draw(&mut state, click(target));
    let _ = draw(&mut state, Vec::new());
    assert_eq!(state.selected_column(), Some(1));
    let _ = draw(&mut state, vec![key(Key::ArrowRight)]);
    assert_eq!(state.selected_column(), Some(2));
    let _ = draw(&mut state, vec![key(Key::ArrowRight)]);
    assert_eq!(
        state.selected_column(),
        Some(2),
        "Right stops at the last column"
    );
    let _ = draw(&mut state, vec![key(Key::ArrowLeft)]);
    let _ = draw(&mut state, vec![key(Key::ArrowLeft)]);
    assert_eq!(state.selected_column(), Some(0));
}

/// The row's wash is the selection: a selected cell that only reads carries
/// no box, and one that edits is boxed, as the cell Enter opens.
#[test]
fn only_a_selected_cell_that_edits_is_boxed() {
    let ctx = context();
    let columns = named_columns();
    let mut state = TableState::default();
    let row = |row: usize| {
        let mut cells = named_row(row);
        cells[1] = cells[1].clone().editable();
        cells
    };
    let draw = |state: &mut TableState, events| {
        frame(&ctx, WIDTH, events, |ui| {
            DataTable::new("boxed", &columns, 3)
                .state(state)
                .cell_selection()
                .show(ui, row)
        })
        .1
    };
    let geometry = draw(&mut state, Vec::new());
    let name = Pos2::new(
        cell(&geometry, 0).center().x,
        geometry.one("table.line.1").expect("row").center().y,
    );
    let _ = draw(&mut state, click(name));
    let geometry = draw(&mut state, Vec::new());
    assert_eq!(state.selected_column(), Some(0));
    assert_eq!(geometry.one("table.selected"), geometry.one("table.line.1"));
    assert!(
        geometry.one("table.cell_box").is_none(),
        "a name cell that does not edit is not boxed"
    );

    let _ = draw(&mut state, vec![key(Key::ArrowRight)]);
    let geometry = draw(&mut state, Vec::new());
    assert_eq!(state.selected_column(), Some(1));
    let line = geometry.one("table.line.1").expect("row");
    let boxed = geometry.one("table.cell_box").expect("the cell that edits");
    assert_eq!(boxed.x_range(), cell(&geometry, 1).x_range());
    assert_eq!(boxed.y_range(), line.y_range());

    let _ = draw(&mut state, vec![key(Key::ArrowRight)]);
    let geometry = draw(&mut state, Vec::new());
    assert_eq!(state.selected_column(), Some(2));
    assert!(geometry.one("table.cell_box").is_none());
}

/// Twelve 120-point columns, wider than the 1100-point page.
fn wide_columns(count: usize, width: f32) -> Vec<Column> {
    (0..count)
        .map(|index| Column::number(format!("P{index}"), width))
        .collect()
}

fn wide_row(columns: usize) -> impl Fn(usize) -> Vec<Cell> {
    move |row| {
        (0..columns)
            .map(|column| Cell::number(format!("{row}:{column}")))
            .collect()
    }
}

#[test]
fn frozen_columns_keep_their_place_while_the_rest_scroll_under_them() {
    let ctx = context();
    let columns = wide_columns(12, 120.0);
    let mut state = TableState::default();
    let draw = |state: &mut TableState| {
        frame(&ctx, WIDTH, Vec::new(), |ui| {
            DataTable::new("frozen", &columns, 3)
                .state(state)
                .freeze(2)
                .show(ui, wide_row(12))
        })
        .1
    };
    let before = draw(&mut state);
    state.scroll_x = 300.0;
    let after = draw(&mut state);
    assert_eq!(state.scroll_x, 300.0);
    for frozen in 0..2 {
        assert_eq!(
            cell(&before, frozen),
            cell(&after, frozen),
            "column {frozen} stays"
        );
        assert_eq!(head(&before, frozen), head(&after, frozen));
    }
    assert_eq!(cell(&after, 5).left(), cell(&before, 5).left() - 300.0);
    for scrolled in [5, 8] {
        assert_eq!(
            head(&after, scrolled).x_range(),
            cell(&after, scrolled).x_range(),
            "column {scrolled}'s head is over it"
        );
    }
    // Columns 2 and 3 scrolled under the frozen ones, which end at 240.
    assert!(after.one("table.cell.2").is_none());
    assert!(after.one("table.cell.3").is_none());
    assert!(cell(&after, 4).right() > 240.0);
    assert_eq!(cell(&after, 1).right(), 240.0);
}

/// A table as narrow as a phone holds the first column only, so the values
/// keep room beside it.
#[test]
fn a_narrow_table_holds_the_first_column_only() {
    let ctx = context();
    let columns = wide_columns(12, 120.0);
    let mut state = TableState::default();
    let draw = |state: &mut TableState| {
        frame(&ctx, 390.0, Vec::new(), |ui| {
            DataTable::new("frozen", &columns, 3)
                .state(state)
                .freeze(2)
                .show(ui, wide_row(12))
        })
        .1
    };
    let before = draw(&mut state);
    state.scroll_x = 100.0;
    let after = draw(&mut state);
    assert_eq!(state.scroll_x, 100.0);
    assert_eq!(cell(&before, 0), cell(&after, 0), "the first column stays");
    assert_eq!(head(&before, 0), head(&after, 0));
    assert_eq!(
        cell(&after, 1).left(),
        cell(&before, 1).left() - 100.0,
        "the second column, which a wider table holds, scrolls"
    );
}

/// A narrow table shows its pinned column directly after the first and keeps
/// it put with it while the values scroll; the page's memory and the response
/// still name columns in the page's order. A wider table keeps the order.
#[test]
fn a_narrow_table_keeps_its_pinned_column_beside_the_first() {
    let ctx = context();
    let mut columns = wide_columns(8, 120.0);
    columns.push(Column::words("Status", 80.0));
    let pinned = columns.len() - 1;
    let mut state = TableState::default();
    state.sort = Some(Sort {
        column: 3,
        direction: SortDirection::Descending,
    });
    let draw = |state: &mut TableState, width: f32, events: Vec<Event>| {
        frame(&ctx, width, events, |ui| {
            DataTable::new("pinned", &columns, 3)
                .state(state)
                .freeze(1)
                .pin_when_narrow(pinned)
                .cell_selection()
                .show(ui, wide_row(9))
        })
    };
    let (_, before, _) = draw(&mut state, 390.0, Vec::new());
    let name = head(&before, 0);
    let status = before
        .one("table.pinned")
        .expect("the pinned column's head");
    assert_eq!(status, head(&before, pinned));
    assert_eq!(
        status.left(),
        name.right(),
        "the pinned column follows the name"
    );
    assert!(
        status.right() <= 390.0,
        "the pinned column is in view: {status:?}"
    );
    assert!(head(&before, 1).left() >= status.right());

    state.scroll_x = 200.0;
    let (_, after, _) = draw(&mut state, 390.0, Vec::new());
    assert_eq!(head(&after, pinned), status, "the pinned column stays put");
    assert_eq!(cell(&after, pinned).x_range(), status.x_range());
    assert_eq!(
        state.sort.map(|sort| sort.column),
        Some(3),
        "the sort names the page's column"
    );

    let (response, _, _) = draw(
        &mut state,
        390.0,
        press(cell(&after, pinned).center(), PointerButton::Secondary),
    );
    assert_eq!(
        response.context_menu.map(|menu| menu.cell.column),
        Some(Some(pinned)),
        "a press on the pinned column names the page's column"
    );
    assert_eq!(state.selected_column(), Some(pinned));

    let (_, desktop, _) = draw(&mut state, WIDTH, Vec::new());
    assert!(
        head(&desktop, pinned).left() > head(&desktop, 7).left(),
        "a wider table keeps the page's order"
    );

    // Narrowness is the table's own width: one in a phone-wide column of a
    // wide window pins too.
    let (_, column, _) = frame(&ctx, WIDTH, Vec::new(), |ui| {
        let rect = egui::Rect::from_min_size(ui.max_rect().min, vec2(390.0, HEIGHT));
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            DataTable::new("pinned", &columns, 3)
                .state(&mut state)
                .freeze(1)
                .pin_when_narrow(pinned)
                .cell_selection()
                .show(ui, wide_row(9))
        })
        .inner
    });
    assert_eq!(
        head(&column, pinned).left(),
        head(&column, 0).right(),
        "a phone-wide table in a wide window pins its column"
    );
}

#[test]
fn a_span_names_its_run_once_centred_and_keeps_to_its_columns_as_they_scroll() {
    let ctx = context();
    let columns = wide_columns(12, 120.0);
    let bands = vec![
        vec![Span::new("Corner", 0, 1), Span::new("Supply 1.62 V", 3, 5)],
        Span::runs(2, &["", "SS", "SS", "SS", "FF"]),
    ];
    let mut state = TableState::default();
    let draw = |state: &mut TableState| {
        frame(&ctx, WIDTH, Vec::new(), |ui| {
            DataTable::new("bands", &columns, 2)
                .state(state)
                .freeze(2)
                .bands(&bands)
                .show(ui, wide_row(12))
        })
        .1
    };
    for offset in [0.0, 100.0] {
        state.scroll_x = offset;
        let geometry = draw(&mut state);
        let rows = geometry.all("table.band");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|band| band.height() == TABLE_HEAD_H));
        assert_eq!(
            rows[1].bottom(),
            geometry.one("table.head").expect("head").top()
        );

        let spans = geometry.all("table.span.0");
        assert_eq!(spans.len(), 1, "the frozen columns' band cells are blank");
        let span = spans[0];
        assert_eq!(span.left(), head(&geometry, 3).left(), "at {offset}");
        assert_eq!(span.right(), head(&geometry, 5).right(), "at {offset}");
        let labels = geometry.all("table.span.0.label");
        assert_eq!(labels.len(), 1, "one label for the run");
        assert!((labels[0].center().x - span.center().x).abs() < 0.5);

        let runs = geometry.all("table.span.1");
        assert_eq!(runs.len(), 2, "the empty span draws nothing");
        assert_eq!(runs[0].left(), head(&geometry, 3).left());
        assert_eq!(runs[0].right(), head(&geometry, 5).right());
    }
}

#[test]
fn a_wide_cross_tab_paints_only_the_columns_in_view() {
    let ctx = context();
    let columns = wide_columns(500, 96.0);
    let read = Counter::new(0_usize);
    let mut state = TableState::default();
    let row = wide_row(500);
    let (_, geometry, output) = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("cross", &columns, 40)
            .state(&mut state)
            .freeze(1)
            .max_height(TABLE_HEAD_H + 12.0 * TABLE_ROW_H)
            .show(ui, |index| {
                read.set(read.get() + 1);
                row(index)
            })
    });
    let painted = (0..500)
        .filter(|column| geometry.one(&format!("table.cell.{column}")).is_some())
        .collect::<Vec<_>>();
    // 1100 points of 96-point columns: twelve, and a thirteenth part shown.
    assert!(painted.len() <= 13, "painted columns {painted:?}");
    assert_eq!(painted[0], 0);
    let cells = (0..500)
        .map(|column| geometry.all(&format!("table.cell.{column}")).len())
        .sum::<usize>();
    let rows = geometry.all("table.row").len();
    assert!(rows <= 14, "{rows} rows laid out");
    assert_eq!(cells, rows * painted.len());
    assert!(read.get() <= 14, "{} rows read", read.get());
    assert!(
        output.shapes.len() < 2_000,
        "{} shapes for one frame",
        output.shapes.len()
    );
    let heads = (0..500)
        .filter(|column| geometry.one(&format!("table.head.{column}")).is_some())
        .count();
    assert_eq!(heads, painted.len());
}

#[test]
fn the_visible_columns_follow_the_scroll() {
    let columns = Columns::new(vec![100.0; 10], 2, 500.0);
    assert_eq!(columns.scrolled_in_view(0.0, 500.0), 2..5);
    // Scrolled 150: content 350..650 shows right of the freeze edge.
    assert_eq!(columns.scrolled_in_view(150.0, 500.0), 3..7);
    assert_eq!(
        columns.in_view(150.0, 500.0).collect::<Vec<_>>(),
        vec![0, 1, 3, 4, 5, 6]
    );
    assert_eq!(columns.column_at(150.0, 150.0), Some(1), "a frozen column");
    assert_eq!(columns.column_at(250.0, 150.0), Some(4));
    assert_eq!(
        columns.reveal(2, 150.0, 500.0),
        0.0,
        "a column under the freeze edge"
    );
    assert_eq!(
        columns.reveal(8, 0.0, 500.0),
        400.0,
        "a column past the right edge"
    );
    assert_eq!(
        columns.reveal(4, 150.0, 500.0),
        150.0,
        "a column in view stays"
    );
    let squeezed = Columns::new(vec![100.0; 4], 3, 320.0);
    assert_eq!(
        squeezed.frozen(),
        0,
        "no frozen column when the rest would have no room"
    );
}

#[test]
fn touch_rows_and_heads_take_the_preferred_target() {
    let ctx = context();
    crate::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(48.0));
    let columns = named_columns();
    let (_, geometry, _) = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("touch", &columns, 3).show(ui, named_row)
    });
    assert_eq!(
        geometry.one("table.head").map(|head| head.height()),
        Some(48.0)
    );
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().all(|row| row.height() == 48.0));

    let fine = context();
    let (_, geometry, _) = frame(&fine, WIDTH, Vec::new(), |ui| {
        DataTable::new("fine", &columns, 3).show(ui, named_row)
    });
    assert_eq!(
        geometry.one("table.head").map(|head| head.height()),
        Some(24.0)
    );
    assert!(
        geometry
            .all("table.row")
            .iter()
            .all(|row| row.height() == 24.0)
    );
}

/// Every line of a grid follows the density: the relaxed grid row is taller
/// than the compact one, for heads, rows and the bands over them alike; under
/// a coarse pointer the lines a finger presses take the target while a band
/// keeps the density's grid row.
#[test]
fn a_grid_takes_its_line_heights_from_the_metrics() {
    let columns = wide_columns(4, 120.0);
    let bands = vec![vec![Span::new("Corner", 1, 3)]];
    let measure = |ctx: &egui::Context| {
        let (_, geometry, _) = frame(ctx, WIDTH, Vec::new(), |ui| {
            DataTable::new("density", &columns, 2)
                .bands(&bands)
                .show(ui, wide_row(4))
        });
        let height = |key: &str| geometry.one(key).map(|rect| rect.height());
        (
            height("table.head"),
            height("table.row"),
            height("table.band"),
        )
    };
    let compact = context();
    assert_eq!(measure(&compact), (Some(24.0), Some(24.0), Some(24.0)));

    let relaxed = egui::Context::default();
    crate::Theme {
        density: crate::Density::Relaxed,
        ..crate::Theme::default()
    }
    .apply(&relaxed);
    let grid = Tokens::get(&relaxed).metrics.grid_row_h;
    assert!(grid > 24.0, "relaxed is roomier than compact");
    assert_eq!(measure(&relaxed), (Some(grid), Some(grid), Some(grid)));

    let touch = context();
    crate::Theme::default().apply_responsive_metrics_with_target(&touch, Some(48.0));
    assert_eq!(measure(&touch), (Some(48.0), Some(48.0), Some(24.0)));
}

#[test]
fn a_table_without_state_draws_as_it_always_has() {
    let ctx = context();
    let columns = named_columns();
    let (response, geometry, _) = frame(&ctx, WIDTH, Vec::new(), |ui| {
        DataTable::new("plain", &columns, 3).show(ui, named_row)
    });
    assert!(!response.selection_changed);
    let head = geometry.one("table.head").expect("head");
    assert_eq!(
        head,
        Rect::from_min_size(Pos2::ZERO, vec2(WIDTH, TABLE_HEAD_H))
    );
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].top(), head.bottom());
    assert_eq!(
        rows[0].width(),
        WIDTH,
        "the name column takes the spare width"
    );
    assert_eq!(geometry.one("table.selected"), None);
    // The value column's numbers end on one edge, its cell less the padding.
    let values = geometry.all("table.right.plain.1");
    assert_eq!(values.len(), 3);
    assert!(
        values
            .iter()
            .all(|value| value.right() == values[0].right())
    );
    assert_eq!(values[0].right(), cell(&geometry, 1).right() - CELL_PAD);
}
