//! The netlist-first landing drawn through a whole application: what it says
//! about the deck the project runs and what its two actions run.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{frame, themed_context};
use crate::workbench::features::{self, Registered};
use egui::{Rect, Ui, Vec2, vec2};
use rspice_app_code::workbench::code::CodeFeature;
use rspice_app_code::workbench::code::netlist_first::*;
use rspice_app_code::workbench::documents::netlist_document::root_deck_source_index_now;
use rspice_app_design::workbench::chrome::document_bar::design::design_documents;
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
            .open_test_deck(DECK.to_owned(), None, "front_end.sp")
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

/// Every string the page paints in one pass, so wording is asserted on the
/// galleys rather than read off pixels.
fn painted_text(size: Vec2, mut pass: impl FnMut(&mut Ui)) -> Vec<String> {
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

/// Every string a whole application frame paints once it has settled.
fn painted_application(app: &mut RSpiceApp, size: Vec2) -> String {
    let ctx = themed_context();
    let mut output = frame(&ctx, app, size);
    for _ in 0..3 {
        output = frame(&ctx, app, size);
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out.join("\n")
}

#[test]
fn the_page_names_the_deck_and_its_two_ways_on_and_lists_nothing_of_it() {
    let mut app = netlist_first_app();
    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    for expected in [
        "NETLIST-FIRST PROJECT",
        "front_end.sp",
        "Open in Netlist",
        "Create schematic\u{2026}",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
    // The deck's text is the Netlist workspace's, and its facts the
    // inspector's: the page repeats neither.
    for absent in [
        "R1 in inn 4.7k",
        "Precision front-end bench",
        "Deck summary",
        "Subcircuits",
    ] {
        assert!(!text.contains(absent), "{absent:?} painted in:\n{text}");
    }
}

#[test]
fn the_design_page_of_a_deck_project_draws_nothing_of_a_schematic() {
    let mut app = netlist_first_app();
    app.state.workbench.activate(Workspace::Design);
    app.state.workbench.navigator_visible = true;
    app.state.workbench.inspector_visible = true;

    let text = painted_application(&mut app, vec2(1440.0, 900.0));
    // No schematic tools, tab, selection or cursor read-out.
    for absent in ["Check cellview", "No object selected", "x \u{2014}"] {
        assert!(!text.contains(absent), "{absent:?} painted in:\n{text}");
    }
    assert!(design_documents(&app.state.model).is_empty());
    // The navigator says there is no schematic; the inspector states the
    // deck's facts.
    for expected in [
        "No schematic in this project.",
        "front_end.sp",
        "Devices",
        "Subcircuits",
        "Includes",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
}

#[test]
fn a_restored_session_counts_its_deck_before_the_editor_has_been_opened() {
    let saved = netlist_first_app();
    let session = serde_json::to_string(&saved.state).expect("session serializes");
    let mut app = RSpiceApp::test_instance();
    app.state = serde_json::from_str(&session).expect("session restores");
    assert!(app.state.is_netlist_first_without_schematic());

    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    assert!(text.contains("front_end.sp"), "{text}");
    let index = root_deck_source_index_now(&app.state.model, &mut app.state.code.netlist);
    let facts = deck_facts(
        &app.state.model,
        &app.state.code.netlist,
        &index,
        app.state.workbench.messages(),
    );
    assert_eq!(facts.lines, 13);
    assert!(facts.path.ends_with("front_end.sp"), "{}", facts.path);
}

#[test]
fn the_facts_count_the_deck_while_the_editor_shows_other_text() {
    let mut app = netlist_first_app();
    // An include file open in the editor, or nothing projected into it yet.
    for editor in ["* models.inc\n.model NFET NMOS (LEVEL=1)\n", ""] {
        editor.clone_into(&mut app.state.model.simulation.source.netlist_content);
        let index = root_deck_source_index_now(&app.state.model, &mut app.state.code.netlist);
        let facts = deck_facts(
            &app.state.model,
            &app.state.code.netlist,
            &index,
            app.state.workbench.messages(),
        );
        assert_eq!(facts.lines, 13, "editor: {editor:?}");
        assert_eq!(facts.devices, 5, "editor: {editor:?}");
    }
}
