//! The schematic canvas drawn by a whole application's frame.

use crate::schematic::view::*;
use crate::state::{Component, ComponentType, Point};
use crate::workbench::app_state::AppState;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn canvas_accessibility_uses_a_stable_node_and_concise_live_selection_status() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let mut component = Component::new(17, ComponentType::Resistor, Point::new(40, 20));
    component.name = "RGAIN".to_owned();
    component.value = "499 ohm".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(17);

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    render_schematic_view(
                        ui,
                        &mut crate::schematic::view::canvas_frame(&mut state),
                        None,
                    )
                });
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("schematic accessibility tree")
        .nodes;
    let canvas = nodes
        .iter()
        .find(|(_, node)| node.role() == egui::accesskit::Role::Canvas)
        .map(|(_, node)| node)
        .expect("schematic canvas node");
    assert_eq!(canvas.label(), Some("Schematic canvas"));
    assert!(
        canvas
            .description()
            .is_some_and(|description| description.contains(
                "Arrow keys select the nearest eligible schematic object in each direction."
            ))
    );
    assert_eq!(canvas.live(), None);

    let status = nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Status
                && node.label() == Some("Selected instance RGAIN, 499 ohm.")
        })
        .map(|(_, node)| node)
        .expect("concise schematic selection status node");
    assert_eq!(status.live(), Some(egui::accesskit::Live::Polite));
}

/// Two frames of a mouse resting at `at` over the bare canvas, with
/// `buttons` held: the cursor the frame asked for, and the canvas-wide
/// horizontal and vertical hairlines it painted, by position.
fn rest_mouse(
    ctx: &egui::Context,
    state: &mut AppState,
    at: egui::Pos2,
    buttons: &[egui::PointerButton],
) -> (egui::CursorIcon, Vec<f32>, Vec<f32>) {
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let mut canvas = egui::Rect::NOTHING;
    let mut output = None;
    for frame in 0..2 {
        let mut events = vec![egui::Event::PointerMoved(at)];
        if frame == 0 {
            events.extend(buttons.iter().map(|button| egui::Event::PointerButton {
                pos: at,
                button: *button,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }));
        }
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                events,
                ..Default::default()
            },
            |ui| {
                canvas = ui.available_rect_before_wrap();
                render_schematic_view(ui, &mut crate::schematic::view::canvas_frame(state), None);
            },
        ));
    }
    let output = output.expect("two frames ran");
    let (mut rows, mut columns) = (Vec::new(), Vec::new());
    for clipped in &output.shapes {
        if let egui::Shape::LineSegment { points: [a, b], .. } = clipped.shape {
            if a.y == b.y && a.x <= canvas.left() && b.x >= canvas.right() {
                rows.push(a.y);
            } else if a.x == b.x && a.y <= canvas.top() && b.y >= canvas.bottom() {
                columns.push(a.x);
            }
        }
    }
    (output.platform_output.cursor_icon, rows, columns)
}

/// C-16, D1 section 5: the full-screen crosshair is a preference, off by
/// default. When on, a mouse over the canvas loses the OS cursor to two
/// canvas-wide hairlines: through the pointer in Select, through the
/// snapped point a click lands on in a drawing tool. Panning hides them.
#[test]
fn the_full_screen_crosshair_follows_a_mouse_when_chosen() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = AppState::default();
    state.design.grid = crate::state::GridStyle::Off;
    let editor = &mut state.model.schematic.session.editor;
    (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
    (editor.pan, editor.zoom) = ((0.0, 0.0), 1.0);
    state.model.schematic.document_mut_for_test().grid_size = 10;
    let at = egui::pos2(203.3, 151.7);

    let (cursor, rows, columns) = rest_mouse(&ctx, &mut state, at, &[]);
    assert_eq!(cursor, egui::CursorIcon::Default);
    assert!(rows.is_empty() && columns.is_empty(), "off by default");

    state
        .model
        .preferences
        .choose_schematic_full_crosshair(true);
    let (cursor, rows, columns) = rest_mouse(&ctx, &mut state, at, &[]);
    assert_eq!(cursor, egui::CursorIcon::None);
    assert_eq!(
        (rows, columns),
        (vec![151.5], vec![203.5]),
        "through the pointer"
    );

    state.model.schematic.arm_tool(crate::state::Tool::Wire);
    let (cursor, rows, columns) = rest_mouse(&ctx, &mut state, at, &[]);
    assert_eq!(cursor, egui::CursorIcon::None);
    assert_eq!(
        (rows, columns),
        (vec![150.5], vec![200.5]),
        "through the grid point the click lands on"
    );

    let (cursor, rows, columns) = rest_mouse(&ctx, &mut state, at, &[egui::PointerButton::Middle]);
    assert_eq!(cursor, egui::CursorIcon::Grabbing);
    assert!(
        rows.is_empty() && columns.is_empty(),
        "hidden while panning"
    );
}

/// Press the right button at `from`, carry it to `to` and let go, one
/// event per frame over a bare canvas at unit zoom.
fn right_drag(state: &mut AppState, from: egui::Pos2, to: egui::Pos2) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let editor = &mut state.model.schematic.session.editor;
    (editor.needs_fit, editor.needs_drawing_sheet_fit) = (false, false);
    (editor.pan, editor.zoom) = ((0.0, 0.0), 1.0);
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    for event in [
        egui::Event::PointerMoved(from),
        button(from, true),
        egui::Event::PointerMoved(to),
        button(to, false),
    ] {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events: vec![event],
                ..Default::default()
            },
            |ui| render_schematic_view(ui, &mut crate::schematic::view::canvas_frame(state), None),
        );
    }
}

/// The right button pans only when Navigation says so, and even then a
/// right click that does not move is still the context menu.
#[test]
fn a_right_drag_pans_only_when_the_right_button_is_chosen() {
    let from = egui::pos2(300.0, 200.0);
    let to = from + egui::vec2(40.0, 0.0);

    let mut shipped = AppState::default();
    right_drag(&mut shipped, from, to);
    assert_eq!(shipped.model.schematic.session.editor.pan, (0.0, 0.0));

    let mut right = AppState::default();
    right.model.preferences.choose_right_button_pan();
    right_drag(&mut right, from, to);
    assert_eq!(right.model.schematic.session.editor.pan, (40.0, 0.0));
    assert!(right.design.interaction.context_target.is_none());

    right_drag(&mut right, from, from);
    assert_eq!(right.model.schematic.session.editor.pan, (0.0, 0.0));
    assert!(
        right.design.interaction.context_target.is_some(),
        "a right click that stays put opens the context menu"
    );
}
