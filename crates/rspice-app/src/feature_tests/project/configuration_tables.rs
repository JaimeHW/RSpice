//! The Configuration page's tables as a reader works them, through whole
//! application frames: rows select as a file list's do, one menu acts on
//! every selected row, a double-click opens the view a row is bound to, the
//! band's filter finds rows anywhere in the tree, pattern bindings are rows
//! of the tree's own table, and every control takes a finger.

#![cfg(not(target_arch = "wasm32"))]

use egui::accesskit::Role;
use egui::{Context, Event, Key, Modifiers, Pos2};

use super::configuration_editing::{
    Node, click, edit, nodes, page_app, secondary_press, settle, shown,
};
use super::recovery_frames::offscreen;
use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference::{AMP_CELL, USER_LIBRARY};
use rspice_app_project::workbench::project::configuration_state::{
    ConfigurationRow, ConfigurationTab, instance_binding,
};
use rspice_app_state::state::{
    CellViewRef, ConfigurationPlatform, ConfigurationSetOverride, Point, View, ViewType,
};
use rspice_app_state::workbench::workspace::Workspace;

/// How many times `amp` is placed in `top` for the table to bind at once.
const AMPS: usize = 40;
/// The height of the window the frames are drawn in.
const WINDOW_HEIGHT: f32 = 900.0;

/// The reference project with `amp` placed forty times in `top`, and an
/// extracted view of `amp` beside its schematic.
fn forty_amps() -> RSpiceApp {
    let mut app = page_app();
    let schematic = &mut app.state.model.schematic;
    let placed = schematic
        .editor_ref()
        .design
        .document()
        .components
        .iter()
        .find(|component| component.name == "X1")
        .and_then(|component| component.library_cell.clone())
        .expect("top places amp as X1");
    for index in 3..=AMPS {
        let column = i32::try_from(index % 10).expect("a small index");
        let row = i32::try_from(index / 10).expect("a small index");
        let id = schematic
            .add_library_cell_component(Point::new(1400 + 200 * column, 200 * row), placed.clone());
        schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed instance is retained")
            .name = format!("X{index}");
    }
    app.state.sync_active_schematic_to_workspace();
    app.state
        .model
        .library_manager
        .get_library_mut(USER_LIBRARY)
        .and_then(|library| library.get_cell_mut(AMP_CELL))
        .expect("the reference holds user/amp")
        .add_view(View::new("extracted", ViewType::Extracted));
    app
}

/// The table rows a frame announced, top to bottom.
fn rows(output: &egui::FullOutput) -> Vec<Node> {
    let mut rows = nodes(output)
        .into_iter()
        .filter(|node| node.role == Role::Row)
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        let y = |node: &Node| node.at.map_or(f32::INFINITY, |at| at.y);
        y(a).total_cmp(&y(b))
    });
    rows
}

/// The row whose words begin with `start`, and where it is on the screen.
fn row_at(output: &egui::FullOutput, start: &str) -> Pos2 {
    let rows = rows(output);
    rows.iter()
        .find(|node| node.label.starts_with(start))
        .and_then(|node| node.at)
        .unwrap_or_else(|| {
            panic!(
                "no row starts {start:?} in {:#?}",
                rows.iter().map(|node| &node.label).collect::<Vec<_>>()
            )
        })
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

/// A press on the row at `at` and its release a frame later, as a hand
/// clicks: the table takes the keyboard with the row.
fn select(ctx: &Context, app: &mut RSpiceApp, at: Pos2) -> egui::FullOutput {
    let edge = |pressed| Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    };
    settle(ctx, app, vec![Event::PointerMoved(at), edge(true)]);
    settle(ctx, app, vec![edge(false)])
}

/// Two presses at `at` in the time a double-click takes.
fn double_click(ctx: &Context, app: &mut RSpiceApp, at: Pos2) -> egui::FullOutput {
    let edge = |pressed| Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    };
    settle(
        ctx,
        app,
        vec![
            Event::PointerMoved(at),
            edge(true),
            edge(false),
            edge(true),
            edge(false),
        ],
    )
}

/// Open the menu of the row at `at` and take `item` from it, through
/// "Set view to" when `view` names a choice of that submenu.
fn take_from_menu(
    ctx: &Context,
    app: &mut RSpiceApp,
    at: Pos2,
    item: &str,
    view: Option<&str>,
) -> egui::FullOutput {
    let menu = settle(ctx, app, secondary_press(at));
    let taken = click(ctx, app, &menu, Role::Button, |name| name.starts_with(item));
    match view {
        Some(view) => click(ctx, app, &taken, Role::Button, |name| name == view),
        None => taken,
    }
}

/// Forty instances found by the filter, selected with Ctrl+A and bound to
/// one view by one pick from their menu: one edit names the view first in
/// each instance's own binding, and every row states it.
#[test]
fn forty_instances_take_one_view_in_one_pick() {
    let ctx = offscreen();
    let mut app = forty_amps();
    let output = settle(&ctx, &mut app, Vec::new());
    click(&ctx, &mut app, &output, Role::TextInput, |name| {
        name == "Filter instances"
    });
    let output = settle(&ctx, &mut app, vec![Event::Text("user/amp".to_owned())]);
    assert!(
        rows(&output)
            .iter()
            .all(|row| row.label.starts_with("Instance: /X")),
        "the filter keeps the instances of amp, each by its whole path"
    );
    let first = row_at(&output, "Instance: /X1,");
    select(&ctx, &mut app, first);
    settle(&ctx, &mut app, vec![key(Key::A, Modifiers::COMMAND)]);
    assert_eq!(
        app.state
            .project
            .configuration
            .instances_table
            .selected_rows()
            .len(),
        AMPS
    );

    let output = take_from_menu(&ctx, &mut app, first, "Set view to", Some("extracted"));
    let definition = shown(&app);
    for index in 1..=AMPS {
        let path = format!("/X{index}");
        let binding = instance_binding(&definition, &path)
            .unwrap_or_else(|| panic!("{path} has a binding of its own"));
        assert_eq!(
            binding.executable_views.first().map(String::as_str),
            Some("extracted"),
            "{path}"
        );
    }
    assert_eq!(
        instance_binding(&definition, "/X2").map(|binding| &binding.executable_views[..]),
        Some(&["extracted".to_owned(), "schematic".to_owned()][..]),
        "a view list the instance already had stays behind the pick"
    );
    let shown_rows = rows(&output);
    assert!(!shown_rows.is_empty());
    for row in shown_rows {
        assert!(
            row.label.contains("View to use: extracted"),
            "{}",
            row.label
        );
    }
}

/// A double-click on a row, and Enter on the selected one, opens the view
/// it is bound to: an instance's resolved view, a cell's.
#[test]
fn a_double_click_opens_the_view_a_row_is_bound_to() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    double_click(&ctx, &mut app, row_at(&output, "Instance: XF2,"));
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("vendor", "filter", "schematic")
    );

    app.state.workbench.activate(Workspace::Project);
    app.state.project.configuration.tab = ConfigurationTab::Cells;
    let output = settle(&ctx, &mut app, Vec::new());
    select(&ctx, &mut app, row_at(&output, "Library: user, Cell: amp,"));
    settle(&ctx, &mut app, vec![key(Key::Enter, Modifiers::NONE)]);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("user", "amp", "schematic")
    );
}

/// Pattern bindings are rows of the tree's own table, under its heads and
/// on its columns: however many there are, they scroll inside it and never
/// push the tree off the page.
#[test]
fn pattern_bindings_are_rows_of_the_tree() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        // Patterns no two of which reach the same instance.
        for index in 0..60 {
            definition.overrides.push(ConfigurationSetOverride {
                instance_path: format!("/B{index}/*"),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
            });
        }
    });
    let output = settle(&ctx, &mut app, Vec::new());
    let heads = nodes(&output)
        .into_iter()
        .filter(|node| {
            [&node.label, &node.value]
                .iter()
                .any(|text| text.starts_with("Table columns: Instance, Cell"))
        })
        .count();
    assert_eq!(heads, 1, "the patterns and the tree share one head");
    let row = format!(
        "Instance: /B0/*, Cell: {dash}, View found: {dash}, View to use: schematic, \
         Bound by: Pattern binding, Model section: {dash}",
        dash = '\u{2014}'
    );
    assert!(
        rows(&output).iter().any(|node| node.label == row),
        "no row {row:?}"
    );
    assert!(
        nodes(&output)
            .iter()
            .any(|node| node.label == "Pattern bindings, 60 rows"),
        "the patterns sit under a head of their own"
    );
    for row in rows(&output) {
        let y = row.at.expect("an announced row is on the screen").y;
        assert!(
            y < WINDOW_HEIGHT,
            "{} is laid out below the window",
            row.label
        );
    }
}

/// Copy binding puts a row's binding aside and Paste binding gives it to the
/// selected rows; Remove binding returns a row to what it inherits, and
/// Delete does the same for the selected rows.
#[test]
fn copy_paste_and_remove_act_on_the_selected_rows() {
    let ctx = offscreen();
    let mut app = page_app();
    edit(&mut app, |definition| {
        let binding = definition
            .overrides
            .iter_mut()
            .find(|binding| binding.instance_path == "/X2")
            .expect("the reference binds X2");
        binding.model_section = Some("tt".to_owned());
        binding.eligible_platforms = vec![ConfigurationPlatform::Desktop];
    });
    let output = settle(&ctx, &mut app, Vec::new());
    let x2 = row_at(&output, "Instance: X2,");
    take_from_menu(&ctx, &mut app, x2, "Copy binding", None);
    let output = settle(&ctx, &mut app, Vec::new());
    let x1 = row_at(&output, "Instance: X1,");
    select(&ctx, &mut app, x1);
    take_from_menu(&ctx, &mut app, x1, "Paste binding", None);
    let definition = shown(&app);
    let pasted = instance_binding(&definition, "/X1").expect("X1 has the pasted binding");
    let copied = instance_binding(&definition, "/X2").expect("X2 keeps its own");
    assert_eq!(pasted.executable_views, copied.executable_views);
    assert_eq!(pasted.model_section.as_deref(), Some("tt"));
    assert_eq!(pasted.eligible_platforms, [ConfigurationPlatform::Desktop]);

    let output = settle(&ctx, &mut app, Vec::new());
    take_from_menu(
        &ctx,
        &mut app,
        row_at(&output, "Instance: X1,"),
        "Remove binding",
        None,
    );
    assert!(instance_binding(&shown(&app), "/X1").is_none());

    let output = settle(&ctx, &mut app, Vec::new());
    select(&ctx, &mut app, row_at(&output, "Instance: X2,"));
    settle(&ctx, &mut app, vec![key(Key::Delete, Modifiers::NONE)]);
    assert!(instance_binding(&shown(&app), "/X2").is_none());
}

/// Up and Down move the selection between rows, and the inspector shows
/// the row it lands on.
#[test]
fn the_arrows_walk_the_rows_and_the_inspector_follows() {
    let ctx = offscreen();
    let mut app = page_app();
    let output = settle(&ctx, &mut app, Vec::new());
    let shown_rows = rows(&output);
    let x1 = shown_rows
        .iter()
        .position(|row| row.label.starts_with("Instance: X1,"))
        .expect("X1 is a row");
    select(
        &ctx,
        &mut app,
        shown_rows[x1].at.expect("X1 is on the screen"),
    );
    assert_eq!(
        app.state.project.configuration.row,
        Some(ConfigurationRow::Instance("/X1".to_owned()))
    );
    settle(&ctx, &mut app, vec![key(Key::ArrowDown, Modifiers::NONE)]);
    let next = &shown_rows[x1 + 1].label;
    let Some(ConfigurationRow::Instance(path)) = app.state.project.configuration.row.clone() else {
        panic!("Down selects an instance");
    };
    let name = path.rsplit('/').next().expect("a path has a last segment");
    assert!(
        next.starts_with(&format!("Instance: {name},")),
        "Down from X1 lands on {path}, the row under it is {next}"
    );
}

/// The filter finds an instance whose parent is folded, and a folded tree
/// comes back as it was when the filter is emptied.
#[test]
fn the_filter_finds_rows_under_a_folded_parent() {
    let ctx = offscreen();
    let mut app = page_app();
    app.state.project.configuration.toggle_collapsed("/");
    let output = settle(&ctx, &mut app, Vec::new());
    assert_eq!(rows(&output).len(), 1, "only the folded top shows");
    app.state.project.configuration.instances_table.filter = "XF".to_owned();
    let output = settle(&ctx, &mut app, Vec::new());
    let found = rows(&output)
        .iter()
        .map(|row| row.label.split(',').next().unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(found, ["Instance: /XF1", "Instance: /XF2"]);
    app.state
        .project
        .configuration
        .instances_table
        .filter
        .clear();
    let output = settle(&ctx, &mut app, Vec::new());
    assert_eq!(rows(&output).len(), 1);
}

/// Under a finger the tree's twist, each chip's controls and the grid's
/// rows are a touch target, not a cursor's.
#[test]
fn the_twist_and_the_chips_take_a_finger() {
    let ctx = offscreen();
    let mut app = page_app();
    app.state.workbench.coarse_pointer = true;
    let output = settle(&ctx, &mut app, Vec::new());
    let target = rspice_app_workbench::ui::tokens::Tokens::get(&ctx)
        .metrics
        .touch_target();
    let bounds = |output: &egui::FullOutput, named: &dyn Fn(&str) -> bool| {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("the frame publishes its tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.label().is_some_and(named))
            .filter_map(|(_, node)| node.bounds())
            .collect::<Vec<_>>()
    };
    let twists = bounds(&output, &|name| name == "Close top");
    assert!(!twists.is_empty(), "the top row folds");
    for twist in twists {
        assert!(
            twist.width() as f32 >= target - 0.5 && twist.height() as f32 >= target - 0.5,
            "the twist is {} by {}",
            twist.width(),
            twist.height()
        );
    }
    let controls = bounds(&output, &|name| {
        name.starts_with("Move ") && name.ends_with(" in View list")
            || name.starts_with("Remove ") && name.ends_with(" from View list")
    });
    assert!(!controls.is_empty(), "the view list's chips have controls");
    for control in controls {
        assert!(
            control.width() as f32 >= target - 0.5,
            "a chip control is {} wide",
            control.width()
        );
    }
    let picker = bounds(&output, &|name| name == "Top cell");
    let row = picker
        .iter()
        .map(|bounds| bounds.height() as f32)
        .fold(0.0, f32::max);
    assert!(
        row >= target,
        "the Top cell row is {row} tall, under a {target} control"
    );
}
