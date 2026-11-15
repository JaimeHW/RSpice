//! The Design toolbar at a desktop window's width, through whole frames of
//! the application: every tool lies before the run controls, and the ones
//! the row has no room for wait behind More tools.

use std::collections::HashMap;

use egui::accesskit::{Affine, NodeId, Point as AxPoint, Role, TreeUpdate};
use egui::{Rect, pos2, vec2};

use crate::feature_tests::replay::{Replay, only};
use crate::state::{Point, Rotation};

use super::symbol_in_place::amp_project;

/// A window of the size a small desktop display gives the application.
const WINDOW: egui::Vec2 = vec2(1024.0, 768.0);

/// Every named control under the context toolbar, with its bounds.
fn toolbar_controls(tree: &TreeUpdate) -> Vec<(String, Rect)> {
    let nodes: HashMap<NodeId, &egui::accesskit::Node> =
        tree.nodes.iter().map(|(id, node)| (*id, node)).collect();
    let root = tree.tree.as_ref().expect("the whole tree").root;
    let mut found = Vec::new();
    let mut pending = vec![(root, Affine::IDENTITY, false)];
    while let Some((id, parent, inside)) = pending.pop() {
        let node = nodes[&id];
        let transform = node.transform().map_or(parent, |own| parent * *own);
        let inside =
            inside || (node.role() == Role::Toolbar && node.label() == Some("Context toolbar"));
        if inside
            && node.role() != Role::Toolbar
            && let (Some(bounds), Some(label)) = (node.bounds(), node.label())
        {
            let min = transform * AxPoint::new(bounds.x0, bounds.y0);
            let max = transform * AxPoint::new(bounds.x1, bounds.y1);
            found.push((
                label.to_owned(),
                Rect::from_min_max(
                    pos2(min.x as f32, min.y as f32),
                    pos2(max.x as f32, max.y as f32),
                ),
            ));
        }
        pending.extend(
            node.children()
                .iter()
                .map(|child| (*child, transform, inside)),
        );
    }
    found
}

/// The toolbar of the frame `replay` draws next: every control lies inside
/// the window and none lies under another, so no tool is drawn under the
/// run controls. Returns the controls.
fn assert_no_tool_under_another(replay: &mut Replay) -> Vec<(String, Rect)> {
    replay.frames(vec![]);
    let tree = replay
        .frame(vec![])
        .platform_output
        .accesskit_update
        .expect("a frame with AccessKit on publishes its tree");
    let controls = toolbar_controls(&tree);
    assert!(
        controls
            .iter()
            .any(|(label, _)| label.starts_with("Run active simulation plan")),
        "the run controls stand in the toolbar: {controls:?}"
    );
    for (label, rect) in &controls {
        assert!(
            rect.right() <= WINDOW.x + 0.5,
            "{label:?} runs past the window: {rect:?}"
        );
    }
    for (index, (label, rect)) in controls.iter().enumerate() {
        for (other, other_rect) in &controls[index + 1..] {
            let nested = rect.expand(0.5).contains_rect(*other_rect)
                || other_rect.expand(0.5).contains_rect(*rect);
            assert!(
                nested || !rect.shrink(0.5).intersects(other_rect.shrink(0.5)),
                "{label:?} {rect:?} lies under {other:?} {other_rect:?}"
            );
        }
    }
    controls
}

/// A session with the `amp` symbol placed once on the sheet in front, in a
/// window 1024 points wide.
fn amp_at_1024() -> (Replay, crate::state::CellViewRef) {
    let mut replay = Replay::new().sized(WINDOW);
    let (_, symbol, _) = amp_project(
        &mut replay.app,
        &[(Point::new(100, 80), Rotation::R0, false)],
    );
    (replay, symbol)
}

/// At 1024 px the symbol row lays every tool it has room for before the run
/// controls, and sets the rest aside behind More tools, in order: the grid
/// spacing, the placed preview and Focus workspace among them. Each answers
/// from there as its button does.
#[test]
fn at_1024_the_symbol_row_sets_aside_what_it_has_no_room_for() {
    let (mut replay, symbol) = amp_at_1024();
    replay.app.state.open_workspace_view(symbol);
    let controls = assert_no_tool_under_another(&mut replay);
    for drawn in ["Select", "Undo", "Fit"] {
        assert!(
            controls.iter().any(|(label, _)| label.starts_with(drawn)),
            "{drawn} stands in the row: {controls:?}"
        );
    }
    for set_aside in ["Preview as placed on a sheet", "Focus workspace"] {
        assert!(
            !controls.iter().any(|(label, _)| label == set_aside),
            "{set_aside} has no room in the row: {controls:?}"
        );
    }

    let menu = replay.click("More tools", Role::Button);
    let rows: Vec<&str> = menu
        .iter()
        .filter(|control| control.role == Role::Button)
        .map(|control| control.label.as_str())
        .collect();
    let at = |label: &str| {
        rows.iter()
            .position(|row| *row == label)
            .unwrap_or_else(|| panic!("More tools lists {label}: {rows:?}"))
    };
    assert!(at("Grid spacing") < at("Preview as placed on a sheet"));
    assert!(at("Preview as placed on a sheet") < at("Focus workspace"));

    assert!(!replay.app.state.design.symbol.editor.preview_as_placed);
    replay.click("Preview as placed on a sheet", Role::Button);
    assert!(replay.app.state.design.symbol.editor.preview_as_placed);

    let spacing = replay.app.state.design.symbol.editor.grid_spacing;
    replay.click("More tools", Role::Button);
    replay.click("Grid spacing", Role::Button);
    assert_ne!(
        replay.app.state.design.symbol.editor.grid_spacing, spacing,
        "Grid spacing steps to the next spacing"
    );
}

/// At 1024 px the schematic row lays no tool under the run controls; what
/// it has no room for waits behind More tools, Focus workspace last, and
/// answers from there.
#[test]
fn at_1024_the_schematic_row_sets_aside_what_it_has_no_room_for() {
    let (mut replay, _) = amp_at_1024();
    let controls = assert_no_tool_under_another(&mut replay);
    assert!(
        !controls.iter().any(|(label, _)| label == "Focus workspace"),
        "Focus workspace has no room in the row: {controls:?}"
    );
    let menu = replay.click("More tools", Role::Button);
    only(&menu, "Focus workspace", Role::Button);
    assert!(!replay.app.state.workbench.focus_mode);
    replay.click("Focus workspace", Role::Button);
    assert!(replay.app.state.workbench.focus_mode);
}
