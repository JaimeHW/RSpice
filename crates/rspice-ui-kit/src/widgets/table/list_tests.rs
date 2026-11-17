//! What a list asks of the shared table, driven through replayed input at
//! the compact, relaxed and touch metrics: selecting several rows, type-ahead,
//! the keys and presses the table reports for the page to act on, and what a
//! finger does in place of Ctrl, a right button and a hover.

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
pub(super) fn metrics() -> Vec<(&'static str, egui::Context)> {
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

pub(super) fn key(key: Key, modifiers: Modifiers) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

pub(super) fn press(pos: Pos2, button: PointerButton, modifiers: Modifiers) -> Vec<Event> {
    let edge = |pressed| Event::PointerButton {
        pos,
        button,
        pressed,
        modifiers,
    };
    vec![Event::PointerMoved(pos), edge(true), edge(false)]
}

/// A name too long for any column to hold, so the table cuts it short.
const LONG: &str = "Bias network for the folded-cascode tail current, trimmed against the \
                    bandgap reference over temperature, supply and every process corner the \
                    foundry characterises, then checked again at the slow-slow and fast-fast \
                    extremes the sign-off flow names";

/// Every run of words a pass painted, its tooltips included.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    fn walk(shape: &egui::Shape, words: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => words.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, words);
                }
            }
            _ => {}
        }
    }
    let mut words = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut words);
    }
    words
}

/// A finger, or the pointer a platform makes of it, going down at `at` or
/// lifting there.
fn finger(at: Pos2, down: bool) -> Vec<Event> {
    vec![
        Event::Touch {
            device_id: egui::TouchDeviceId(0),
            id: egui::TouchId(0),
            phase: if down {
                egui::TouchPhase::Start
            } else {
                egui::TouchPhase::End
            },
            pos: at,
            force: None,
        },
        Event::PointerMoved(at),
        Event::PointerButton {
            pos: at,
            button: PointerButton::Primary,
            pressed: down,
            modifiers: Modifiers::NONE,
        },
    ]
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
    names: [&'static str; 7],
    columns: Vec<Column>,
    keys: Vec<RowKey>,
    state: TableState,
    multi: bool,
    /// The list is asked to take the keyboard.
    focus: bool,
    /// The input's clock, which every frame advances.
    time: f64,
}

impl Picker {
    fn new(ctx: &egui::Context, multi: bool) -> Self {
        Self {
            ctx: ctx.clone(),
            names: NAMES,
            columns: vec![Column::name("Cell", 160.0), Column::words("View", 90.0)],
            keys: NAMES.iter().map(|name| RowKey::from(*name)).collect(),
            state: TableState::default(),
            multi,
            focus: false,
            time: ctx.input(|input| input.time),
        }
    }

    /// One frame over `events` in which the list is asked to take the
    /// keyboard.
    fn pass_focused(&mut self, events: Vec<Event>) -> TableResponse {
        self.focus = true;
        let response = self.frame(events, Modifiers::NONE);
        self.focus = false;
        response
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
            names,
            columns,
            keys,
            state,
            multi,
            focus,
            ..
        } = self;
        let mut response = TableResponse::default();
        let (output, geometry) = pass(ctx, input, |ui| {
            let mut table = DataTable::new("picker", columns, NAMES.len())
                .state(state)
                .keys(keys)
                .focus(*focus);
            if *multi {
                table = table.multi_select();
            }
            response = table.show(ui, |row| {
                vec![Cell::text(names[row]), Cell::text("schematic")]
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

    /// A tap on row `row`'s selection mark, and the frame after it.
    fn tap_mark(&mut self, row: usize) -> TableResponse {
        let (_, geometry, _) = self.pass(Vec::new(), Modifiers::NONE, 0.1);
        let at = geometry
            .one(&format!("table.mark.{row}"))
            .unwrap_or_else(|| panic!("row {row} has a mark"))
            .center();
        let response = self.frame(
            press(at, PointerButton::Primary, Modifiers::NONE),
            Modifiers::NONE,
        );
        let _ = self.frame(Vec::new(), Modifiers::NONE);
        response
    }

    /// A press at `at` held still for `seconds`, a frame each quarter
    /// second, then let go: the menus it asked for, whether each held frame
    /// painted `words`, and the row the release clicked.
    fn hold(
        &mut self,
        at: Pos2,
        seconds: f64,
        touch: bool,
        words: &str,
    ) -> (Vec<ContextRequest>, Vec<bool>, Option<usize>) {
        let edges = |down: bool| {
            if touch {
                finger(at, down)
            } else {
                let mut events = vec![Event::PointerMoved(at)];
                events.push(Event::PointerButton {
                    pos: at,
                    button: PointerButton::Primary,
                    pressed: down,
                    modifiers: Modifiers::NONE,
                });
                events
            }
        };
        let mut asked = Vec::new();
        let mut shown = Vec::new();
        asked.extend(self.frame(edges(true), Modifiers::NONE).context_menu);
        for _ in 0..(seconds / 0.25).round() as usize {
            let (response, _, output) = self.pass(Vec::new(), Modifiers::NONE, 0.25);
            asked.extend(response.context_menu);
            shown.push(painted(&output).iter().any(|painted| painted == words));
        }
        let response = self.frame(edges(false), Modifiers::NONE);
        asked.extend(response.context_menu);
        (asked, shown, response.clicked)
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

/// A list asked to take the keyboard has it without a press, and Left and
/// Right, which have nowhere to go in a flat list, are reported for the page
/// to move to the neighbouring list.
#[test]
fn a_list_takes_the_keyboard_when_asked_and_reports_left_and_right() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, false);
        list.state.select(2, Some(RowKey::from(NAMES[2])));
        let response = list.frame(vec![key(Key::ArrowRight, Modifiers::NONE)], Modifiers::NONE);
        assert!(
            !response.focused,
            "{metric}: nothing gave the list the keyboard"
        );
        assert_eq!(response.across, None, "{metric}");

        let _ = list.pass_focused(Vec::new());
        let response = list.frame(vec![key(Key::ArrowRight, Modifiers::NONE)], Modifiers::NONE);
        assert!(response.focused, "{metric}: the list holds the keyboard");
        assert_eq!(response.across, Some(1), "{metric}: Right");
        let response = list.key(Key::ArrowLeft, Modifiers::NONE);
        assert_eq!(response.across, Some(-1), "{metric}: Left");
        let response = list.key(Key::ArrowDown, Modifiers::NONE);
        assert_eq!(
            response.across, None,
            "{metric}: Down moves within the list"
        );
        assert_eq!(list.state.selected_rows(), vec![3], "{metric}");
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

/// A finger has no Ctrl to hold: under a coarse pointer each row of a table
/// that selects several carries a mark a tap toggles, on a target a finger
/// can find, while a tap on the row itself still selects it alone. A fine
/// pointer gets no marks.
#[test]
fn a_tap_on_a_rows_mark_toggles_it_into_the_selection_under_a_coarse_pointer() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, true);
        let (_, geometry, _) = list.pass(Vec::new(), Modifiers::NONE, 0.1);
        let touch = metric == "touch";
        assert_eq!(
            geometry.one("table.mark.0").is_some(),
            touch,
            "{metric}: marks only under a finger"
        );
        let single = Picker::new(&ctx, false)
            .pass(Vec::new(), Modifiers::NONE, 0.1)
            .1;
        assert!(
            single.one("table.mark.0").is_none(),
            "{metric}: a table that selects one row has no marks"
        );
        if !touch {
            continue;
        }
        let target = crate::tokens::Tokens::get(&ctx).metrics.touch_target();
        let (mark, line) = (
            geometry.one("table.mark.3").expect("a mark"),
            geometry.one("table.line.3").expect("row 3"),
        );
        assert_eq!(mark.width(), target, "a target a finger finds");
        assert_eq!(mark.y_range(), line.y_range(), "as tall as its row");
        assert_eq!(mark.left(), line.left(), "before the first column's words");

        let _ = list.click(1, Modifiers::NONE);
        let response = list.tap_mark(3);
        assert!(response.selection_changed);
        assert_eq!(response.clicked, None, "a mark selects; it opens nothing");
        assert_eq!(list.state.selected_rows(), vec![3, 1]);
        let _ = list.tap_mark(5);
        assert_eq!(list.state.selected_rows(), vec![5, 1, 3]);
        let _ = list.tap_mark(3);
        assert_eq!(
            list.state.selected_rows(),
            vec![5, 1],
            "a second tap lets go"
        );
        let _ = list.click(4, Modifiers::NONE);
        assert_eq!(
            list.state.selected_rows(),
            vec![4],
            "a tap on the row selects it alone"
        );
    }
}

/// A press held still on a row asks for its menu, once, under a coarse
/// pointer or wherever the platform reports a finger; on a selected row the
/// selection stays, so the menu acts on all of it, and letting go clicks
/// nothing. A mouse has its right button, so holding it asks nothing.
#[test]
fn a_press_held_still_asks_for_the_rows_menu_under_a_coarse_pointer() {
    for (metric, ctx) in metrics() {
        let touch = metric == "touch";
        for finger in [false, true] {
            let mut list = Picker::new(&ctx, true);
            let _ = list.click(1, Modifiers::NONE);
            let _ = list.click(4, Modifiers::COMMAND);
            let at = list.line(4).center();
            let (asked, _, clicked) = list.hold(at, 1.5, finger, "");
            assert_eq!(clicked, None, "{metric}: a held press is not a click");
            if touch || finger {
                assert_eq!(
                    asked.len(),
                    1,
                    "{metric}, finger {finger}: one menu a press"
                );
                assert_eq!(asked[0].cell.row, 4, "{metric}");
                assert_eq!(
                    list.state.selected_rows(),
                    vec![4, 1],
                    "{metric}: the menu acts on the selection"
                );
            } else {
                assert!(asked.is_empty(), "{metric}, finger {finger}: {asked:?}");
            }

            let at = list.line(2).center();
            let (asked, _, clicked) = list.hold(at, 0.5, finger, "");
            assert!(asked.is_empty(), "{metric}: a short press asks nothing");
            assert_eq!(clicked, Some(2), "{metric}: it is a click");
        }
    }
}

/// Words a cell cuts short show in full while the pointer rests on them, at
/// every metric; words that fit show once, in their cell.
#[test]
fn an_elided_cell_shows_its_full_words_on_hover() {
    for (metric, ctx) in metrics() {
        let mut list = Picker::new(&ctx, false);
        list.names[2] = LONG;
        let rest = |list: &mut Picker, at: Pos2, words: &str| {
            let _ = list.frame(vec![Event::PointerMoved(at)], Modifiers::NONE);
            (0..8).any(|_| {
                let (_, _, output) = list.pass(Vec::new(), Modifiers::NONE, 0.25);
                painted(&output)
                    .iter()
                    .filter(|painted| *painted == words)
                    .count()
                    == 1
            })
        };
        let (_, _, output) = list.pass(Vec::new(), Modifiers::NONE, 0.1);
        assert!(
            !painted(&output).iter().any(|painted| painted == LONG),
            "{metric}: the cell cuts the name short"
        );
        let cut = list.line(2).left_center() + vec2(60.0, 0.0);
        assert!(rest(&mut list, cut, LONG), "{metric}: the full name shows");
        let fits = list.line(3).left_center() + vec2(60.0, 0.0);
        let _ = list.frame(vec![Event::PointerMoved(fits)], Modifiers::NONE);
        for _ in 0..8 {
            let (_, _, output) = list.pass(Vec::new(), Modifiers::NONE, 0.25);
            let words = painted(&output);
            assert_eq!(
                words
                    .iter()
                    .filter(|painted| *painted == "Comparator")
                    .count(),
                1,
                "{metric}: words that fit need no tip"
            );
            assert!(!words.iter().any(|painted| painted == LONG), "{metric}");
        }
    }
}

/// A finger cannot hover, so a press held on cut-short words shows them in
/// full for as long as it is held, under a coarse pointer or wherever the
/// platform reports a finger. A tap first keeps the hover tip away, so only
/// the held press can show them.
#[test]
fn an_elided_cell_shows_its_full_words_under_a_held_press() {
    for (metric, ctx) in metrics() {
        let touch = metric == "touch";
        for finger in [false, true] {
            let mut list = Picker::new(&ctx, false);
            list.names[2] = LONG;
            let at = list.line(2).left_center() + vec2(60.0, 0.0);
            let _ = list.frame(
                press(at, PointerButton::Primary, Modifiers::NONE),
                Modifiers::NONE,
            );
            let (_, shown, _) = list.hold(at, 1.5, finger, LONG);
            assert_eq!(
                shown.iter().any(|shown| *shown),
                touch || finger,
                "{metric}, finger {finger}: {shown:?}"
            );
            if touch || finger {
                assert!(shown.last() == Some(&true), "{metric}: shown while held");
                assert!(!shown[0], "{metric}: not before the press is held");
            }
        }
    }
}

/// A row pressed and carried out of one list onto another's row is reported
/// by the list it was let go over, which names the list and row it came
/// from; the list it came from reports no click.
#[test]
fn a_row_dragged_onto_another_lists_row_is_reported_where_it_lands() {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    let columns = [Column::name("Cell", 160.0)];
    let keys = NAMES
        .iter()
        .map(|name| RowKey::from(*name))
        .collect::<Vec<_>>();
    let (mut from, mut to) = (TableState::default(), TableState::default());
    let mut time = 0.0;
    let mut frame = |events: Vec<Event>| {
        time += 0.05;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(WIDTH, HEIGHT))),
            events,
            time: Some(time),
            ..Default::default()
        };
        let mut answers = (TableResponse::default(), TableResponse::default());
        let (_, geometry) = pass(&ctx, input, |ui| {
            // Each list in a column of its own, side by side.
            let column = |left: f32| {
                egui::UiBuilder::new()
                    .max_rect(Rect::from_min_size(
                        Pos2::new(left, 0.0),
                        vec2(300.0, HEIGHT),
                    ))
                    .layout(egui::Layout::top_down(egui::Align::Min))
            };
            ui.scope_builder(column(0.0), |ui| {
                answers.0 = DataTable::new("from", &columns, NAMES.len())
                    .list()
                    .state(&mut from)
                    .keys(&keys)
                    .drag_rows()
                    .show(ui, |row| vec![Cell::text(NAMES[row])]);
            });
            ui.scope_builder(column(400.0), |ui| {
                answers.1 = DataTable::new("to", &columns, NAMES.len())
                    .list()
                    .state(&mut to)
                    .keys(&keys)
                    .drops()
                    .show(ui, |row| vec![Cell::text(NAMES[row])]);
            });
        });
        (answers, geometry)
    };
    let (_, geometry) = frame(Vec::new());
    let line = |row: usize| {
        let mut lines = geometry.all(&format!("table.line.{row}"));
        lines.sort_by(|left, right| left.left().total_cmp(&right.left()));
        (lines[0].center(), lines[1].center())
    };
    let (source, _) = line(1);
    let (_, target) = line(3);
    let button = |at: Pos2, pressed: bool| Event::PointerButton {
        pos: at,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };

    frame(vec![Event::PointerMoved(source), button(source, true)]);
    let mut clicked = None;
    for step in 1..=6 {
        let at = source + (target - source) * (step as f32 / 6.0);
        let ((dragged_from, _), _) = frame(vec![Event::PointerMoved(at)]);
        clicked = clicked.or(dragged_from.clicked);
    }
    let ((released_from, landed), _) = frame(vec![button(target, false)]);
    let (row, drag) = landed
        .dropped
        .expect("the list it landed on reports the drop");
    assert_eq!(row, 3, "it landed on the row it was let go over");
    assert!(drag.is_from("from") && !drag.is_from("to"));
    assert_eq!(drag.row, 1, "it names the row carried");
    assert_eq!(
        clicked.or(released_from.clicked),
        None,
        "a drag is no click"
    );
}
