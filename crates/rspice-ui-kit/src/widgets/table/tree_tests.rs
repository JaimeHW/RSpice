//! The shared table drawn as a plain list or a tree, driven through replayed
//! input at the compact, relaxed and touch metrics: no heads, the list row
//! height, a leading icon, twist and indent, the keys that open, close and
//! walk a tree, and the roles a reader's assistive technology hears.

use egui::accesskit::{NodeId, Role};
use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect, vec2};

use super::list_tests::{key, metrics, press};
use super::*;
use crate::probe::{Geometry, pass};
use crate::widgets::tree::{TREE_INDENT, TWIST_ROOM};

/// A library rail: each row's name, its level, and, for a row with
/// children, whether they show.
const ROWS: [(&str, u8, Option<bool>); 4] = [
    ("Library", 0, Some(true)),
    ("Amplifier", 1, None),
    ("Bandgap", 1, Some(false)),
    ("Notes", 0, None),
];

/// Row `row`'s one cell: a folder for a row with children, a file for one
/// without, a leaf's room under an open row so its name lines up.
fn rail_cell(row: usize) -> Cell {
    let (name, level, open) = ROWS[row];
    let cell = Cell::text(name).indent(level).icon(if open.is_some() {
        Icon::Folder
    } else {
        Icon::File
    });
    match open {
        Some(open) => cell.twist(open),
        None if level > 0 => cell.leaf(),
        None => cell,
    }
}

/// Every run of words a pass painted, and where it starts.
fn painted_at(output: &egui::FullOutput) -> Vec<(String, Pos2)> {
    fn walk(shape: &egui::Shape, words: &mut Vec<(String, Pos2)>) {
        match shape {
            egui::Shape::Text(text) => words.push((text.galley.text().to_owned(), text.pos)),
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

/// A rail drawn as `kind`, with state, at one metric.
struct Rail {
    ctx: egui::Context,
    columns: Vec<Column>,
    keys: Vec<RowKey>,
    state: TableState,
    kind: Kind,
    multi: bool,
    time: f64,
}

impl Rail {
    fn new(ctx: &egui::Context, kind: Kind) -> Self {
        Self {
            ctx: ctx.clone(),
            columns: vec![Column::name("Cell", 160.0)],
            keys: ROWS.iter().map(|(name, ..)| RowKey::from(*name)).collect(),
            state: TableState::default(),
            kind,
            multi: false,
            time: ctx.input(|input| input.time),
        }
    }

    /// One frame over `events` with `modifiers` held: what the table
    /// reported, where it drew, what it painted, and where it began.
    fn pass(
        &mut self,
        events: Vec<Event>,
        modifiers: Modifiers,
    ) -> (TableResponse, Geometry, egui::FullOutput, f32) {
        self.time += 0.1;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(400.0, 600.0))),
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
            kind,
            multi,
            ..
        } = self;
        let mut response = TableResponse::default();
        let mut top = 0.0;
        let (output, geometry) = pass(ctx, input, |ui| {
            top = ui.cursor().top();
            let mut table = DataTable::new("rail", columns, ROWS.len())
                .state(state)
                .keys(keys);
            table = match *kind {
                Kind::Grid => table,
                Kind::List => table.list(),
                Kind::Tree => table.tree(),
            };
            if *multi {
                table = table.multi_select();
            }
            response = table.show(ui, |row| vec![rail_cell(row)]);
        });
        (response, geometry, output, top)
    }

    fn frame(&mut self, events: Vec<Event>, modifiers: Modifiers) -> TableResponse {
        self.pass(events, modifiers).0
    }

    fn geometry(&mut self) -> Geometry {
        self.pass(Vec::new(), Modifiers::NONE).1
    }

    /// A press on row `row` with `modifiers` held, and the frame after.
    fn click(&mut self, row: usize, modifiers: Modifiers) -> TableResponse {
        let at = self
            .geometry()
            .one(&format!("table.line.{row}"))
            .unwrap_or_else(|| panic!("row {row} is drawn"))
            .center();
        let response = self.frame(press(at, PointerButton::Primary, modifiers), modifiers);
        let _ = self.frame(Vec::new(), Modifiers::NONE);
        response
    }

    fn key(&mut self, pressed: Key) -> TableResponse {
        self.frame(vec![key(pressed, Modifiers::NONE)], Modifiers::NONE)
    }
}

/// A list draws no heads: its first row starts where the list does, and
/// every row is a list row tall under the live metrics, a finger's target
/// under a coarse pointer. A grid keeps its heads.
#[test]
fn a_list_draws_no_heads_and_takes_the_list_row_height() {
    for (metric, ctx) in metrics() {
        for kind in [Kind::List, Kind::Tree] {
            let mut rail = Rail::new(&ctx, kind);
            let (_, geometry, _, top) = rail.pass(Vec::new(), Modifiers::NONE);
            assert!(
                geometry.one("table.head").is_none(),
                "{metric}: a list has no heads"
            );
            let metrics = Tokens::get(&ctx).metrics;
            let row = if metrics.is_touch() {
                metrics.touch_target()
            } else {
                metrics.row_h
            };
            let first = geometry.one("table.line.0").expect("row 0");
            assert_eq!(first.height(), row, "{metric}");
            assert_eq!(first.top(), top, "{metric}: the rows start the list");
        }
        let grid = Rail::new(&ctx, Kind::Grid).geometry();
        assert!(
            grid.one("table.head").is_some(),
            "{metric}: a grid keeps its heads"
        );
    }
}

/// A list copies its selected rows' words, with no head line over them.
#[test]
fn a_list_copies_its_rows_without_a_head_line() {
    for (metric, ctx) in metrics() {
        let mut rail = Rail::new(&ctx, Kind::List);
        rail.multi = true;
        let _ = rail.click(0, Modifiers::NONE);
        let _ = rail.click(3, Modifiers::COMMAND);
        let (_, _, output, _) = rail.pass(vec![Event::Copy], Modifiers::NONE);
        let copied = output
            .platform_output
            .commands
            .iter()
            .find_map(|command| match command {
                egui::OutputCommand::CopyText(text) => Some(text.clone()),
                _ => None,
            });
        assert_eq!(copied.as_deref(), Some("Library\nNotes"), "{metric}");
    }
}

/// An indent steps a row's words in by a level; a twist and an icon stand
/// before them; a leaf's room lines its words up with a sibling's that has a
/// twist. Under a coarse pointer the twist is a target a finger finds.
#[test]
fn a_trees_indent_twist_and_icon_stand_before_its_words() {
    for (metric, ctx) in metrics() {
        let mut rail = Rail::new(&ctx, Kind::Tree);
        let (_, geometry, output, _) = rail.pass(Vec::new(), Modifiers::NONE);
        let words = painted_at(&output);
        let left = |name: &str| {
            words
                .iter()
                .find(|(words, _)| words == name)
                .map(|(_, at)| at.x)
                .unwrap_or_else(|| panic!("{metric}: {name} is painted"))
        };
        assert_eq!(
            left("Amplifier") - left("Library"),
            TREE_INDENT,
            "{metric}: one level in"
        );
        assert_eq!(
            left("Bandgap"),
            left("Amplifier"),
            "{metric}: a leaf lines up with its sibling's twist"
        );
        assert!(
            left("Notes") < left("Library"),
            "{metric}: no twist, no room"
        );

        let twist = geometry.one("table.twist.0").expect("Library has a twist");
        let metrics = Tokens::get(&ctx).metrics;
        let room = if metrics.is_touch() {
            metrics.touch_target()
        } else {
            TWIST_ROOM
        };
        assert_eq!(twist.width(), room, "{metric}");
        assert!(
            twist.right() < left("Library"),
            "{metric}: before the words"
        );
        assert!(
            geometry.one("table.twist.1").is_none(),
            "{metric}: a leaf has no twist"
        );
    }
}

/// A click on a twist asks to open or close its row without selecting it;
/// Left closes an open row and steps out to the parent, Right opens a closed
/// row and steps into an open one, and a top row without children stays.
#[test]
fn a_twist_and_left_and_right_open_close_and_walk_a_tree() {
    for (metric, ctx) in metrics() {
        let mut rail = Rail::new(&ctx, Kind::Tree);
        let twist = rail
            .geometry()
            .one("table.twist.2")
            .expect("Bandgap has a twist")
            .center();
        let response = rail.frame(
            press(twist, PointerButton::Primary, Modifiers::NONE),
            Modifiers::NONE,
        );
        assert_eq!(response.expand, Some((2, true)), "{metric}: open it");
        assert_eq!(response.clicked, None, "{metric}");
        assert_eq!(
            rail.state.selected_row(),
            None,
            "{metric}: nothing selected"
        );

        let _ = rail.click(0, Modifiers::NONE);
        assert_eq!(
            rail.key(Key::ArrowLeft).expand,
            Some((0, false)),
            "{metric}"
        );
        let _ = rail.key(Key::ArrowRight);
        assert_eq!(
            rail.state.selected_row(),
            Some(1),
            "{metric}: into the child"
        );
        let _ = rail.key(Key::ArrowLeft);
        assert_eq!(
            rail.state.selected_row(),
            Some(0),
            "{metric}: out to the parent"
        );

        let _ = rail.click(2, Modifiers::NONE);
        assert_eq!(
            rail.key(Key::ArrowRight).expand,
            Some((2, true)),
            "{metric}"
        );
        let _ = rail.key(Key::ArrowLeft);
        assert_eq!(
            rail.state.selected_row(),
            Some(0),
            "{metric}: a closed row's parent"
        );

        let _ = rail.click(3, Modifiers::NONE);
        let response = rail.key(Key::ArrowLeft);
        assert_eq!(response.expand, None, "{metric}");
        assert_eq!(
            rail.state.selected_row(),
            Some(3),
            "{metric}: a top row stays"
        );
    }
}

/// Whether `node` sits under `ancestor` in the accessibility tree.
fn descends(nodes: &[(NodeId, egui::accesskit::Node)], ancestor: NodeId, node: NodeId) -> bool {
    nodes
        .iter()
        .find(|(id, _)| *id == ancestor)
        .is_some_and(|(_, parent)| {
            parent
                .children()
                .iter()
                .any(|child| *child == node || descends(nodes, *child, node))
        })
}

/// A grid's rows announce as rows, a list's as options and a tree's as
/// items, each saying whether it is selected, under the table's one stop,
/// which says it selects several; a tree's items say their level and
/// whether their children show.
#[test]
fn rows_announce_as_selectable_grid_rows_list_options_or_tree_items() {
    for (metric, ctx) in metrics() {
        ctx.enable_accesskit();
        for (kind, holder, item) in [
            (Kind::Grid, Role::Grid, Role::Row),
            (Kind::List, Role::ListBox, Role::ListBoxOption),
            (Kind::Tree, Role::Tree, Role::TreeItem),
        ] {
            let mut rail = Rail::new(&ctx, kind);
            rail.multi = true;
            let _ = rail.click(1, Modifiers::NONE);
            let _ = rail.click(2, Modifiers::COMMAND);
            let (_, _, output, _) = rail.pass(Vec::new(), Modifiers::NONE);
            let nodes = output
                .platform_output
                .accesskit_update
                .expect("an accessibility tree")
                .nodes;
            let (table, node) = nodes
                .iter()
                .find(|(_, node)| node.role() == holder)
                .unwrap_or_else(|| panic!("{metric}: a {holder:?}"));
            assert!(node.is_multiselectable(), "{metric}: {holder:?}");
            let row = |name: &str| {
                nodes
                    .iter()
                    .find(|(_, node)| {
                        node.role() == item
                            && node.label().is_some_and(|label| label.ends_with(name))
                    })
                    .unwrap_or_else(|| panic!("{metric}: {name} as {item:?}"))
            };
            for (name, selected) in [("Library", false), ("Amplifier", true), ("Bandgap", true)] {
                let (id, node) = row(name);
                assert_eq!(node.is_selected(), Some(selected), "{metric}: {name}");
                assert!(
                    descends(&nodes, *table, *id),
                    "{metric}: {name} under the {holder:?}"
                );
            }
            if kind == Kind::Tree {
                assert_eq!(row("Library").1.level(), Some(1), "{metric}");
                assert_eq!(row("Library").1.is_expanded(), Some(true), "{metric}");
                assert_eq!(row("Amplifier").1.level(), Some(2), "{metric}");
                assert_eq!(
                    row("Amplifier").1.is_expanded(),
                    None,
                    "{metric}: no children"
                );
                assert_eq!(row("Bandgap").1.is_expanded(), Some(false), "{metric}");
            }
        }
    }
}
