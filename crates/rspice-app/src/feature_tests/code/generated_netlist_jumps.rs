//! Every jump to the generated netlist arrives at it: "Open generated
//! netlist" and the Connectivity manager's source map put the generated
//! netlist of the design as it stands in front of the Netlist workspace,
//! whatever deck or include was in front, and whether or not the Netlist page
//! was ever opened.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use crate::workbench::examples::hierarchy_reference;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document::fixtures::seed_owned_dependency;
use rspice_app_code::workbench::documents::netlist_document::{
    ActiveNetlistDocument, open_netlist_dependency_from_root_in,
};
use rspice_app_design::workbench::app::dialogs::design_request::DesignRequest;
use rspice_app_state::workbench::workspace::Workspace;

/// The engineer's own deck, kept in the project beside the design.
const DECK: &str = "engineer's deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

/// The hierarchical reference design in Design, with the engineer's own deck
/// in the project.
fn design_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app.state.model.workspace.content.netlist_source = Some(DECK.to_owned());
    app.state.workbench.activate(Workspace::Design);
    app
}

/// [`design_app`] with the engineer's deck on the Netlist page, drawn.
fn deck_in_front_app() -> RSpiceApp {
    let mut app = design_app();
    app.state.workbench.activate(Workspace::Netlist);
    app.state.model.simulation.source.netlist_content = DECK.to_owned();
    app.state.code.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    app
}

/// [`design_app`] with an editable deck's include file in front on the
/// Netlist page, drawn.
fn include_in_front_app() -> RSpiceApp {
    let mut app = design_app();
    seed_owned_dependency(&mut app.state.model, &mut app.state.code.netlist);
    app.state.workbench.activate(Workspace::Netlist);
    let state = &mut app.state;
    open_netlist_dependency_from_root_in(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        ActiveNetlistDocument::OwnedSource,
        "models/resistor.inc",
    )
    .expect("the include opens");
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    assert!(
        app.state.code.netlist.active_dependency_identity.is_some(),
        "the include is in front before the jump"
    );
    app
}

/// Every string `output` painted, one per line.
fn painted(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(shape.galley.text());
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}

/// After two frames, the Netlist workspace shows the generated netlist of
/// the design, and nothing of `hidden`.
fn assert_generated_in_front(app: &mut RSpiceApp, hidden: &str, from: &str) {
    let ctx = themed_context();
    frame(&ctx, app, DESKTOP);
    let output = frame(&ctx, app, DESKTOP);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist, "{from}");
    assert_eq!(
        app.state.code.netlist.active_document,
        ActiveNetlistDocument::Generated,
        "{from}: the generated netlist is in front"
    );
    assert!(
        app.state.code.netlist.active_dependency_identity.is_none(),
        "{from}: no include is in front"
    );
    let generated = app.state.model.netlist_artifacts.generated_source.clone();
    assert!(
        generated
            .lines()
            .any(|line| line.to_ascii_lowercase().starts_with(".subckt")),
        "{from}: the design's netlist was generated: {generated}"
    );
    assert_eq!(
        app.state.model.simulation.source.netlist_content, generated,
        "{from}: the editor holds the generated netlist"
    );
    let first = generated
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default();
    let painted = painted(&output);
    assert!(
        painted.contains(first),
        "{from}: the generated netlist is painted"
    );
    assert!(
        !painted.contains(hidden),
        "{from}: what was in front before is not painted"
    );
}

#[test]
fn open_generated_netlist_from_the_schematic_shows_it_before_the_page_was_ever_opened() {
    let mut app = design_app();
    app.execute_command_key(CodeCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, "engineer's deck", "from the schematic");
}

#[test]
fn open_generated_netlist_replaces_the_editable_deck_in_front() {
    let mut app = deck_in_front_app();
    app.execute_command_key(CodeCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, "engineer's deck", "over the deck");
}

#[test]
fn open_generated_netlist_replaces_an_include_tab_in_front() {
    let mut app = include_in_front_app();
    app.execute_command_key(CodeCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, ".param rmodel=1k", "over the include");
}

/// The Connectivity manager's source map lands on the generated netlist at
/// the first line it maps, even with an include in front.
#[test]
fn the_connectivity_source_map_lands_on_the_generated_netlist() {
    let mut app = include_in_front_app();
    app.state.workbench.activate(Workspace::Design);
    app.answer_design_request(DesignRequest::OpenSourceMap);
    let first_mapped = app
        .state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(rspice_app_state::state::NetlistDocument::generated_artifact)
        .and_then(|artifact| artifact.source_map().first())
        .map(rspice_app_state::state::GeneratedSourceMapEntry::generated_line);
    assert!(first_mapped.is_some(), "the generated netlist maps a line");
    assert_eq!(app.state.code.netlist.requested_line, first_mapped);
    assert_generated_in_front(&mut app, ".param rmodel=1k", "from the source map");
}
