//! The netlist-first landing drawn through a whole application: what it says
//! about the deck that owns the project and what its two actions run.

use crate::workbench::RSpiceApp;
use crate::workbench::features::{self, Registered};
use egui::{Rect, Ui, Vec2, vec2};
use rspice_app_code::workbench::code::CodeFeature;
use rspice_app_code::workbench::code::netlist_first::*;
use rspice_app_code::workbench::documents::netlist_document::root_deck_source_index;
use rspice_app_state::workbench::workspace::Workspace;

const DECK: &str = "Precision front-end bench\n\
    * differential input stage\n\
    V1 in 0 AC 1\n\
    R1 in inn 4.7k\n\
    C1 inn 0 220p\n\
    M1 out inn 0 0 NFET W=10u L=1u\n\
    .model NFET NMOS (LEVEL=1 VTO=0.7)\n\
    .subckt buffer a y\n\
    E1 y 0 a 0 1\n\
    .ends\n\
    .tran 1n 1u\n\
    .op\n\
    .end\n";

pub(super) fn netlist_first_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .apply_imported_netlist(DECK.to_owned(), None, "front_end.sp")
    );
    assert!(app.state.is_netlist_first_without_schematic());
    app
}

/// One pass of the landing the code feature registers, drawn over `app` in
/// the context the shell lends it.
fn landing(app: &mut RSpiceApp, ui: &mut Ui) {
    let surface = CodeFeature::registry()
        .surface_section()
        .expect("the code feature registers the netlist-first landing");
    (surface.show)(&mut features::surface_cx::<CodeFeature>(app, |_| false), ui);
}

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

#[test]
fn netlist_first_empty_state_actions_use_the_canonical_commands() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();

    raise_netlist_first_action(
        &mut app.state.workbench,
        NetlistFirstAction::OpenNetlistWorkspace,
    );
    app.after_contribution();
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);

    raise_netlist_first_action(
        &mut app.state.workbench,
        NetlistFirstAction::CreateSchematic,
    );
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
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

    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    assert!(!text.contains("The deck has no source text."), "{text}");
    assert!(
        text.contains("R1 in inn 4.7k"),
        "deck lines missing in:\n{text}"
    );
    let index = root_deck_source_index(&app.state.model, &mut app.state.code.netlist);
    assert_eq!(
        deck_facts(&app.state.model, &app.state.code.netlist, &index).lines,
        13
    );
}

#[test]
fn the_landing_shows_the_deck_while_the_editor_shows_other_text() {
    let mut app = netlist_first_app();
    // An include file open in the editor, or nothing projected into it yet.
    for editor in ["* models.inc\n.model NFET NMOS (LEVEL=1)\n", ""] {
        editor.clone_into(&mut app.state.model.simulation.source.netlist_content);
        let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
        assert!(text.contains("Precision front-end bench"), "{text}");
        assert!(!text.contains("* models.inc"), "{text}");
    }
}

#[test]
fn a_long_deck_truncates_behind_a_footer_that_counts_what_was_left_out() {
    let mut app = truncated_netlist_first_app();

    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    assert!(
        text.contains("more lines"),
        "no truncation footer in:\n{text}"
    );
    assert!(
        !text.contains("R399"),
        "the deck's tail painted past the viewport"
    );
}
