//! The netlist-first landing drawn through a whole application: what it says
//! about the deck that owns the project and what its two actions run.

use crate::workbench::RSpiceApp;
use crate::workbench::design::surface::netlist_first::tests::{
    NETLIST_FIRST_BASELINES, SUPPORTED_VIEWPORTS, netlist_first_app,
};
use crate::workbench::design::surface::netlist_first::*;
use crate::workbench::documents::netlist_document::root_deck_source_index;
use crate::workbench::state::Workspace;
use egui::{Rect, Ui, Vec2, pos2, vec2};

fn truncated_netlist_first_app() -> RSpiceApp {
    let mut deck = String::from("Long resistor ladder bench\n");
    for element in 0..400 {
        deck.push_str(&format!("R{element} n{element} 0 1k\n"));
    }
    deck.push_str(".op\n.end\n");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(deck, None, "ladder.sp"));
    assert!(app.state.is_netlist_first_without_schematic());
    app
}

fn netlist_first_canvas(size: Vec2, truncated: bool) -> crate::ui::raster::Canvas {
    let mut app = if truncated {
        truncated_netlist_first_app()
    } else {
        netlist_first_app()
    };
    crate::ui::raster::render(size, |ui, _| show(ui, &mut app.state))
}

fn regression_height(canvas: &crate::ui::raster::Canvas) -> usize {
    canvas.content_height().max(1)
}

#[test]
fn supported_netlist_first_landings_match_their_reviewed_visual_baselines() {
    for baseline in NETLIST_FIRST_BASELINES {
        let canvas = netlist_first_canvas(baseline.size, baseline.truncated);
        canvas.assert_regression(
            baseline.name,
            regression_height(&canvas),
            baseline.fingerprint,
        );
    }
}

#[test]
fn netlist_first_empty_state_actions_use_the_canonical_commands() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();

    raise_netlist_first_action(&mut app.state, NetlistFirstAction::OpenNetlistWorkspace);
    app.after_contribution();
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);

    raise_netlist_first_action(&mut app.state, NetlistFirstAction::CreateSchematic);
    app.after_contribution();
    assert!(app.state.project.new_cell_dialog);
    assert!(app.state.project.new_cell_create_schematic);
}

/// Every string the page paints in one pass, so wording is asserted on the
/// galleys rather than read off pixels.
fn painted_text(size: Vec2, mut pass: impl FnMut(&mut Ui)) -> Vec<String> {
    fn walk(shape: &egui::epaint::Shape, out: &mut Vec<String>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(text.galley.text().to_owned()),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, out);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| pass(ui));
                },
            )
            .shapes;
    }
    let mut out = Vec::new();
    for clipped in &shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

#[test]
fn the_page_names_the_deck_its_actions_and_its_facts() {
    let mut app = netlist_first_app();
    let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app.state)).join("\n");
    for expected in [
        EYEBROW,
        "Open Netlist & Script Editor",
        "Create schematic\u{2026}",
        "Promote to a schematic-driven project",
        "Deck source",
        "front_end",
        "Deck summary",
        "Devices",
        "Analyses",
        "Precision front-end bench",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
}

#[test]
fn a_restored_session_shows_its_deck_before_the_editor_has_been_opened() {
    let saved = netlist_first_app();
    let session = serde_json::to_string(&saved.state).expect("session serializes");
    let mut app = RSpiceApp::test_instance();
    app.state = serde_json::from_str(&session).expect("session restores");
    assert!(app.state.is_netlist_first_without_schematic());

    let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app.state)).join("\n");
    assert!(!text.contains("The deck has no source text."), "{text}");
    assert!(
        text.contains("R1 in inn 4.7k"),
        "deck lines missing in:\n{text}"
    );
    let index = root_deck_source_index(&app.state.model, &mut app.state.code.netlist);
    assert_eq!(deck_facts(&app.state, &index).lines, 13);
}

#[test]
fn the_landing_shows_the_deck_while_the_editor_shows_other_text() {
    let mut app = netlist_first_app();
    // An include file open in the editor, or nothing projected into it yet.
    for editor in ["* models.inc\n.model NFET NMOS (LEVEL=1)\n", ""] {
        editor.clone_into(&mut app.state.model.simulation.source.netlist_content);
        let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app.state)).join("\n");
        assert!(text.contains("Precision front-end bench"), "{text}");
        assert!(!text.contains("* models.inc"), "{text}");
    }
}

#[test]
fn a_long_deck_truncates_behind_a_footer_that_counts_what_was_left_out() {
    let mut app = truncated_netlist_first_app();

    let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app.state)).join("\n");
    assert!(
        text.contains("more lines"),
        "no truncation footer in:\n{text}"
    );
    assert!(
        !text.contains("R399"),
        "the deck's tail painted past the viewport"
    );
}

#[test]
fn the_page_paints_the_full_landing_at_all_supported_viewports() {
    for ((_, size), minimum_content_height) in SUPPORTED_VIEWPORTS.into_iter().zip([470, 760, 740])
    {
        let canvas = netlist_first_canvas(size, false);
        let content_height = canvas.content_height();
        assert!(
            content_height >= minimum_content_height,
            "{}x{}: content stops at row {content_height}",
            size.x,
            size.y
        );
    }
}

/// On the two-column layout the deck panel and the rail both carry ink in
/// their own columns — the failure mode this catches is the old page,
/// whose body was one empty band.
#[test]
fn the_two_column_landing_fills_both_columns() {
    let mut app = netlist_first_app();
    let size = vec2(1440.0, 900.0);
    let canvas = crate::ui::raster::render(size, |ui, _| show(ui, &mut app.state));

    // Content column: 1240 centered in 1440 leaves 100 on each side. The
    // preview occupies the left column and the rail the last 322 points.
    let preview_probe = Rect::from_min_size(pos2(160.0, 300.0), vec2(300.0, 200.0));
    let rail_probe = Rect::from_min_size(pos2(1040.0, 160.0), vec2(240.0, 60.0));
    for (label, probe) in [("deck preview", preview_probe), ("action rail", rail_probe)] {
        assert!(
            canvas
                .pixels_in(probe)
                .any(|pixel| pixel != canvas.background()),
            "{label} painted nothing"
        );
    }
}
