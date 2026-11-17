//! What a list asks of the shared table, driven through replayed input at
//! the compact, relaxed and touch metrics: selecting several rows, type-ahead,
//! and the keys and presses the table reports for the page to act on.

use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect, vec2};

use super::state::type_ahead;
use super::*;
use crate::probe::{Geometry, pass};

const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 600.0;

/// The names the list shows, one per row, in the page's order.
const NAMES: [&str; 7] = [
    "Amplifier",
    "Bandgap",
    "bias",
    "Comparator",
    "Driver",
    "Bandgap_v2",
    "LDO",
];

/// A context at each metric a list is used at, by name.
fn metrics() -> Vec<(&'static str, egui::Context)> {
    let compact = egui::Context::default();
    crate::Theme::default().apply(&compact);
    let relaxed = egui::Context::default();
    crate::Theme {
        density: crate::Density::Relaxed,
        ..crate::Theme::default()
    }
    .apply(&relaxed);
    let touch = egui::Context::default();
    crate::Theme::default().apply(&touch);
    crate::Theme::default().apply_responsive_metrics_with_target(&touch, Some(44.0));
    vec![("compact", compact), ("relaxed", relaxed), ("touch", touch)]
}

fn key(key: Key, modifiers: Modifiers) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn press(pos: Pos2, button: PointerButton, modifiers: Modifiers) -> Vec<Event> {
    let edge = |pressed| Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers,
    };
    vec![Event::PointerMoved(pos), edge(true), edge(false)]
}

/// The copy command a pass sent the platform, if it sent one.
fn copied(output: &egui::FullOutput) -> Option<String> {
    output
        .platform_output
        .commands
        .iter()
        .find_map(|command| match command {
            egui::OutputCommand::CopyText(text) => Some(text.clone()),
            _ => None,
        })
}

/// A two-column cell list with state, keyed by name, drawn at one metric.
struct Picker {
    ctx: egui::Context,
    columns: Vec<Column>,
    keys: Vec<RowKey>,
    state: TableState,
    multi: bool,
    /// The input's clock, which every frame advances.
    time: f64,
}

impl Picker {
    fn new(ctx: &egui::Context, multi: bool) -> Self {
        Self {
            ctx: ctx.clone(),
            columns: vec![Column::name("Cell", 160.0), Column::words("View", 90.0)],
            keys: NAMES.iter().map(|name| RowKey::from(*name)).collect(),
            state: TableState::default(),
            multi,
            time: 0.0,
        }
    }

    /// One frame over `events` with `modifiers` held, `pause` seconds after
    /// the last one.
    fn pass(
        &mut self,
        events: Vec<Event>,
        modifiers: Modifiers,
        pause: f64,
    ) -> (TableResponse, Geometry, egui::FullOutput) {
        self.time += pause;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(WIDTH, HEIGHT))),
            events,
            modifiers,
            time: Some(self.time),
            ..Default::default()
        };
        let Self {
            ctx,
            columns,
            keys,
            state,
            multi,
            ..
        } = self;
        let mut response = TableResponse::default();
        let (output, geometry) = pass(ctx, input, |ui| {
            let mut table = DataTable::new("picker", columns, NAMES.len())
                .state(state)
                .keys(keys);
            if *multi {
                table = table.multi_select();
            }
            response = table.show(ui, |row| {
                vec![Cell::text(NAMES[row]), Cell::text("schematic")]
            });
        });
        (response, geometry, output)
    }

    fn frame(&mut self, events: Vec<Event>, modifiers: Modifiers) -> TableResponse {
        self.pass(events, modifiers, 0.1).0
    }

    /// Where row `row` is drawn.
    fn line(&mut self, row: usize) -> Rect {
        self.pass(Vec::new(), Modifiers::NONE, 0.1)
            .1
            .one(&format!("table.line.{row}"))
            .unwrap_or_else(|| panic!("row {row} is drawn"))
    }

    /// A press on row `row`'s name with `modifiers` held, and the frame
    /// after it.
    fn click(&mut self, row: usize, modifiers: Modifiers) -> TableResponse {
        let at = self.line(row).center();
        let response = self.frame(press(at, PointerButton::Primary, modifiers), modifiers);
        let _ = self.frame(Vec::new(), Modifiers::NONE);
        response
    }

    fn key(&mut self, pressed: Key, modifiers: Modifiers) -> TableResponse {
        self.frame(vec![key(pressed, modifiers)], modifiers)
    }

    fn keys(&self) -> Vec<&str> {
        self.state
            .selected_keys()
            .into_iter()
            .map(|key| match key {
                RowKey::Text(text) => text.as_str(),
                RowKey::Number(_) => "",
            })
            .collect()
    }
}

#[test]
fn ctrl_toggles_rows_and_shift_extends_from_the_anchor() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let _ = list.click(1, Modifiers::NONE);
        let response = list.click(3, Modifiers::COMMAND);
        assert!(response.selection_changed, "{metric}");
        assert_eq!(list.state.selected_rows(), vec![3, 1], "{metric}");
        assert_eq!(list.keys(), vec!["Comparator", "Bandgap"], "{metric}");

        let _ = list.click(1, Modifiers::COMMAND);
        assert_eq!(list.state.selected_rows(), vec![3], "{metric}: Ctrl again");

        let _ = list.click(3, Modifiers::NONE);
        let _ = list.click(5, Modifiers::SHIFT);
        assert_eq!(
            list.state.selected_rows(),
            vec![5, 3, 4],
            "{metric}: Shift reaches from the anchor, the pressed row leads"
        );
        let _ = list.click(0, Modifiers::COMMAND | Modifiers::SHIFT);
        let mut rows = list.state.selected_rows();
        rows.sort_unstable();
        assert_eq!(rows, vec![0, 1, 2, 3, 4, 5], "{metric}: Ctrl+Shift adds");

        let _ = list.click(2, Modifiers::NONE);
        assert_eq!(
            list.state.selected_rows(),
            vec![2],
            "{metric}: a plain press selects one"
        );
    }
}

#[test]
fn shift_with_the_arrows_extends_and_ctrl_a_selects_every_row_shown() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let _ = list.click(2, Modifiers::NONE);
        let _ = list.key(Key::ArrowDown, Modifiers::SHIFT);
        let _ = list.key(Key::ArrowDown, Modifiers::SHIFT);
        assert_eq!(list.state.selected_rows(), vec![4, 2, 3], "{metric}");
        let _ = list.key(Key::ArrowUp, Modifiers::SHIFT);
        assert_eq!(list.state.selected_rows(), vec![3, 2], "{metric}");
        let _ = list.key(Key::ArrowDown, Modifiers::NONE);
        assert_eq!(
            list.state.selected_rows(),
            vec![4],
            "{metric}: an arrow alone moves one"
        );

        list.state.filter = "band".to_owned();
        let response = list.key(Key::A, Modifiers::COMMAND);
        assert!(response.selection_changed, "{metric}");
        let mut rows = list.state.selected_rows();
        rows.sort_unstable();
        assert_eq!(rows, vec![1, 5], "{metric}: every row the filter shows");
    }
}

#[test]
fn a_context_press_on_a_selected_row_keeps_the_selection() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let _ = list.click(0, Modifiers::NONE);
        let _ = list.click(4, Modifiers::COMMAND);
        let at = list.line(0).center();
        let response = list.frame(
            press(at, PointerButton::Secondary, Modifiers::NONE),
            Modifiers::NONE,
        );
        assert_eq!(
            response.context_menu.map(|menu| menu.cell.row),
            Some(0),
            "{metric}"
        );
        assert_eq!(list.state.selected_rows(), vec![0, 4], "{metric}");

        let at = list.line(6).center();
        let _ = list.frame(
            press(at, PointerButton::Secondary, Modifiers::NONE),
            Modifiers::NONE,
        );
        assert_eq!(
            list.state.selected_rows(),
            vec![6],
            "{metric}: one outside it"
        );
    }
}

#[test]
fn a_table_that_selects_one_row_reads_ctrl_and_shift_presses_as_plain() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, false);
        let _ = list.click(1, Modifiers::NONE);
        let _ = list.click(3, Modifiers::COMMAND);
        assert_eq!(list.state.selected_rows(), vec![3], "{metric}");
        let _ = list.click(5, Modifiers::SHIFT);
        assert_eq!(list.state.selected_rows(), vec![5], "{metric}");
        let _ = list.key(Key::ArrowDown, Modifiers::SHIFT);
        assert_eq!(
            list.state.selected_rows(),
            vec![6],
            "{metric}: Shift with an arrow moves the one row"
        );
        let _ = list.key(Key::A, Modifiers::COMMAND);
        assert_eq!(list.state.selected_rows(), vec![6], "{metric}");
    }
}

#[test]
fn typed_letters_move_to_the_next_row_whose_name_starts_with_them() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let _ = list.click(0, Modifiers::NONE);
        let typed = |list: &mut Picker, text: &str, pause: f64| {
            list.pass(vec![Event::Text(text.to_owned())], Modifiers::NONE, pause)
                .0
        };
        let response = typed(&mut list, "b", 2.0);
        assert!(response.selection_changed, "{metric}");
        assert_eq!(list.state.selected_row(), Some(1), "{metric}: Bandgap");
        let _ = typed(&mut list, "b", 0.2);
        assert_eq!(
            list.state.selected_row(),
            Some(2),
            "{metric}: the same letter again moves on, ignoring case"
        );
        let _ = typed(&mut list, "b", 0.2);
        let _ = typed(&mut list, "b", 0.2);
        assert_eq!(list.state.selected_row(), Some(1), "{metric}: and cycles");

        let _ = typed(&mut list, "ba", 2.0);
        let _ = typed(&mut list, "ndgap_", 0.2);
        assert_eq!(
            list.state.selected_row(),
            Some(5),
            "{metric}: more letters refine"
        );
        let _ = typed(&mut list, "l", 2.0);
        assert_eq!(
            list.state.selected_row(),
            Some(6),
            "{metric}: a pause starts over"
        );
    }
}

#[test]
fn type_ahead_reads_the_rows_in_the_order_they_show() {
    let words = |row: usize| NAMES[row].to_lowercase();
    assert_eq!(type_ahead(&[3, 1, 2], None, "b", words), Some(1));
    assert_eq!(type_ahead(&[3, 1, 2], Some(1), "b", words), Some(2));
    assert_eq!(type_ahead(&[3, 1, 2], Some(2), "b", words), Some(1));
    assert_eq!(type_ahead(&[3, 1, 2], Some(3), "z", words), None);
    assert_eq!(type_ahead(&[], None, "b", words), None);
    assert_eq!(type_ahead(&[3], None, "", words), None);
}

#[test]
fn enter_f2_and_delete_are_reported_for_the_page_to_act_on() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let response = list.key(Key::Delete, Modifiers::NONE);
        assert!(
            !response.delete,
            "{metric}: no keyboard, nothing selected, nothing to delete"
        );
        let _ = list.click(3, Modifiers::NONE);
        let _ = list.click(4, Modifiers::COMMAND);

        let response = list.key(Key::Enter, Modifiers::NONE);
        assert_eq!(response.activate, Some(4), "{metric}: Enter runs the lead");
        assert_eq!(response.edit, None, "{metric}");
        let response = list.key(Key::F2, Modifiers::NONE);
        assert_eq!(response.rename, Some(4), "{metric}: F2 renames the lead");
        let response = list.key(Key::Delete, Modifiers::NONE);
        assert!(response.delete, "{metric}: Delete names the selection");
        assert_eq!(list.state.selected_rows(), vec![4, 3], "{metric}");

        let response = list.key(Key::F10, Modifiers::SHIFT);
        let menu = response.context_menu.expect("Shift+F10 asks for a menu");
        assert_eq!(menu.cell.row, 4, "{metric}");

        let at = list.line(1).center();
        let mut events = press(at, PointerButton::Primary, Modifiers::NONE);
        events.extend(press(at, PointerButton::Primary, Modifiers::NONE));
        let response = list.frame(events, Modifiers::NONE);
        assert_eq!(
            response.activate,
            Some(1),
            "{metric}: a double-click runs the row"
        );
    }
}

#[test]
fn ctrl_c_copies_every_selected_row_in_the_order_they_show() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let _ = list.click(4, Modifiers::NONE);
        let _ = list.click(1, Modifiers::COMMAND);
        let (_, _, output) = list.pass(vec![Event::Copy], Modifiers::NONE, 0.1);
        assert_eq!(
            copied(&output).as_deref(),
            Some("Cell\tView\nBandgap\tschematic\nDriver\tschematic"),
            "{metric}"
        );
    }
}

#[test]
fn the_selection_follows_its_keys_when_the_rows_change() {
    let mut state = TableState::default();
    let keys = |names: &[&str]| {
        names
            .iter()
            .map(|name| RowKey::from(*name))
            .collect::<Vec<_>>()
    };
    let before = keys(&["a", "b", "c", "d"]);
    let selection = |row: usize| Selection {
        row,
        key: Some(before[row].clone()),
        column: None,
    };
    let _ = state.select_only(selection(1));
    let _ = state.toggle(selection(3));
    let _ = state.toggle(selection(2));
    assert_eq!(state.selected_rows(), vec![2, 1, 3]);

    state.follow_keys(&keys(&["d", "c", "a"]));
    assert_eq!(
        state.selected_rows(),
        vec![1, 0],
        "c moved, d moved, b went"
    );
    state.follow_keys(&keys(&["a"]));
    assert_eq!(state.selected_rows(), Vec::<usize>::new());
    assert_eq!(state.selection, None);
}
