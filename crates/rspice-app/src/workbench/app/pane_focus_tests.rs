//! F6 and Shift+F6 walk the workbench's panes through whole frames: rail,
//! navigator, document, bottom dock, inspector, wrapping, and back again.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, frame_with, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::workbench::state::Workspace;
use crate::workbench::state::pane_focus::Pane;
use egui::accesskit::TreeUpdate;

const EVERY_PANE: [Pane; 5] = [
    Pane::ActivityRail,
    Pane::Navigator,
    Pane::Document,
    Pane::BottomDock,
    Pane::Inspector,
];

/// A press and its release, as a keyboard sends them: egui reads a second
/// press with no release between as the key repeating.
fn f6(modifiers: egui::Modifiers) -> Vec<egui::Event> {
    [true, false]
        .map(|pressed| egui::Event::Key {
            key: egui::Key::F6,
            physical_key: Some(egui::Key::F6),
            pressed,
            repeat: false,
            modifiers,
        })
        .to_vec()
}

/// The pane holding the keyboard, read from where a screen reader is told the
/// focused control sits.
fn pane_with_focus(app: &RSpiceApp, update: &TreeUpdate) -> Option<Pane> {
    if update.tree.as_ref()?.root == update.focus {
        return None;
    }
    let (_, focused) = update.nodes.iter().find(|(id, _)| *id == update.focus)?;
    let bounds = focused.bounds()?;
    let centre = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    EVERY_PANE.into_iter().find(|pane| {
        app.state
            .workbench
            .pane_focus
            .drawn_rect(*pane)
            .is_some_and(|rect| rect.contains(centre))
    })
}

/// Press F6 with `modifiers`; the frame after it is the one that draws the
/// new focus.
fn press(ctx: &egui::Context, app: &mut RSpiceApp, modifiers: egui::Modifiers) -> Option<Pane> {
    frame_with(ctx, app, DESKTOP, f6(modifiers));
    let output = frame(ctx, app, DESKTOP);
    let update = output
        .platform_output
        .accesskit_update
        .expect("AccessKit is on");
    pane_with_focus(app, &update)
}

/// An open project on its schematic, with every pane on screen.
fn schematic_workbench() -> (egui::Context, RSpiceApp) {
    let mut app = open_project_app();
    app.state.workbench.activate(Workspace::Design);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app)
}

/// An open project on a Verilog-A source document.
fn source_workbench() -> (egui::Context, RSpiceApp) {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app)
}

#[test]
fn f6_visits_each_pane_in_order_and_wraps_and_shift_f6_walks_back() {
    let (ctx, mut app) = schematic_workbench();
    // Nothing holds the keyboard yet, so the first press is the document's.
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::Document)
    );
    for expected in [
        Pane::BottomDock,
        Pane::Inspector,
        Pane::ActivityRail,
        Pane::Navigator,
        Pane::Document,
        Pane::BottomDock,
    ] {
        assert_eq!(press(&ctx, &mut app, egui::Modifiers::NONE), Some(expected));
    }
    for expected in [
        Pane::Document,
        Pane::Navigator,
        Pane::ActivityRail,
        Pane::Inspector,
        Pane::BottomDock,
    ] {
        assert_eq!(
            press(&ctx, &mut app, egui::Modifiers::SHIFT),
            Some(expected)
        );
    }
}

#[test]
fn the_panes_are_recorded_where_they_read_on_screen() {
    let (_ctx, app) = schematic_workbench();
    let rect = |pane| {
        app.state
            .workbench
            .pane_focus
            .drawn_rect(pane)
            .unwrap_or_else(|| panic!("{pane:?} is drawn"))
    };
    let document = rect(Pane::Document);
    assert!(rect(Pane::ActivityRail).right() <= rect(Pane::Navigator).left() + 1.0);
    assert!(rect(Pane::Navigator).right() <= document.left() + 1.0);
    assert!(document.right() <= rect(Pane::Inspector).left() + 1.0);
    assert!(document.bottom() <= rect(Pane::BottomDock).top() + 1.0);
}

#[test]
fn landing_in_a_source_document_puts_the_keyboard_in_its_editor() {
    let (ctx, mut app) = source_workbench();
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::Document)
    );
    let editor = app
        .state
        .workbench
        .active_source_editor
        .expect("the document is a source editor")
        .id;
    assert!(ctx.memory(|memory| memory.has_focus(editor)));
}

#[test]
fn a_pane_that_is_not_drawn_is_passed_over() {
    let (ctx, mut app) = schematic_workbench();
    app.state.workbench.inspector_visible = false;
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::Document)
    );
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::BottomDock)
    );
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::ActivityRail)
    );
}

#[test]
fn the_pane_keys_leave_a_focused_text_editor() {
    let (ctx, mut app) = source_workbench();
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::Document)
    );
    // The editor keeps its Tab for indenting, but the pane keys still leave it.
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::BottomDock)
    );
}

#[test]
fn a_modal_dialog_keeps_the_keyboard_from_the_pane_keys() {
    let (ctx, mut app) = source_workbench();
    assert_eq!(
        press(&ctx, &mut app, egui::Modifiers::NONE),
        Some(Pane::Document)
    );
    crate::workbench::commands::vocabulary::Command::CommandPalette.dispatch(&mut app);
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    let in_palette = ctx.memory(|memory| memory.focused());
    frame_with(&ctx, &mut app, DESKTOP, f6(egui::Modifiers::NONE));
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(ctx.memory(|memory| memory.focused()), in_palette);
    assert!(
        app.state.workbench.command_palette.open,
        "the palette is still open"
    );
}
