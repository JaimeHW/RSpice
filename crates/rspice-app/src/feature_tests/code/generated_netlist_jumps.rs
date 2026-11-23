//! Every jump to the generated netlist arrives at it: "Open generated
//! netlist" and the Connectivity manager's source map put the generated
//! netlist of the design as it stands in front of the Netlist workspace,
//! whatever deck or include was in front, and whether or not the Netlist page
//! was ever opened.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use crate::workbench::examples::hierarchy_reference;
use rspice_app_code::workbench::code::commands::SourceCommand;
use rspice_app_code::workbench::documents::netlist_document::fixtures::{
    DECK_INCLUDE, seed_deck_with_include,
};
use rspice_app_code::workbench::documents::netlist_document::{
    NetlistRoot, open_netlist_dependency_from_root_in,
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
    app.state.seed_test_deck(DECK);
    app.state.workbench.activate(Workspace::Design);
    app
}

/// [`design_app`] with the engineer's deck on the Netlist page, drawn.
fn deck_in_front_app() -> RSpiceApp {
    let mut app = design_app();
    app.state.workbench.activate(Workspace::Netlist);
    app.state.seed_test_deck(DECK);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    app
}

/// [`design_app`] with an editable deck's include file in front on the
/// Netlist page, drawn.
fn include_in_front_app() -> RSpiceApp {
    let mut app = design_app();
    seed_deck_with_include(&mut app.state.model, &mut app.state.code.netlist);
    app.state.workbench.activate(Workspace::Netlist);
    let state = &mut app.state;
    open_netlist_dependency_from_root_in(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        NetlistRoot::Deck,
        DECK_INCLUDE,
        state.workbench.messages(),
    )
    .expect("the include opens");
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    assert!(
        app.state.code.netlist.front.include().is_some(),
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
        app.state.code.netlist.front.root_kind(),
        NetlistRoot::Generated,
        "{from}: the generated netlist is in front"
    );
    assert!(
        app.state.code.netlist.front.include().is_none(),
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
    app.execute_command_key(SourceCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, "engineer's deck", "from the schematic");
}

#[test]
fn open_generated_netlist_replaces_the_editable_deck_in_front() {
    let mut app = deck_in_front_app();
    app.execute_command_key(SourceCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, "engineer's deck", "over the deck");
}

#[test]
fn open_generated_netlist_replaces_an_include_tab_in_front() {
    let mut app = include_in_front_app();
    app.execute_command_key(SourceCommand::GenerateNetlist.key());
    assert_generated_in_front(&mut app, ".param rmodel=1k", "over the include");
}

/// A net's Netlist button in the Connectivity manager lands on the generated
/// netlist at the first card that connects that net, even with an include in
/// front, and types nothing into the navigator's filter.
#[test]
fn the_connectivity_source_map_lands_on_the_nets_first_card() {
    let mut app = include_in_front_app();
    app.state.workbench.activate(Workspace::Design);
    app.answer_design_request(DesignRequest::OpenSourceMap(String::new()));
    let (net, line) = app
        .state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(rspice_app_state::state::NetlistDocument::generated_artifact)
        .and_then(|artifact| {
            let last = artifact
                .source_map()
                .iter()
                .rev()
                .find(|entry| !entry.nets().is_empty())?;
            let net = last.nets().last()?.clone();
            let first = artifact.source_map().iter().find(|entry| {
                entry
                    .nets()
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(&net))
            })?;
            Some((net, first.generated_line()))
        })
        .expect("the generated netlist maps a card to a net");

    app.state.workbench.activate(Workspace::Design);
    app.answer_design_request(DesignRequest::OpenSourceMap(net.to_ascii_uppercase()));

    assert_eq!(app.state.code.netlist.requested_line, Some(line), "{net}");
    assert_eq!(app.state.workbench.navigator_filter(), "");
    assert_generated_in_front(&mut app, ".param rmodel=1k", "from the source map");
}

/// A filter left in the Netlist workspace's navigator would hide the
/// schematic source of the line the jump lands on: the jump clears it, and
/// the navigator lists that source.
#[test]
fn the_connectivity_source_map_clears_a_filter_left_in_the_netlist_navigator() {
    let mut app = include_in_front_app();
    app.state
        .workbench
        .set_navigator_filter("no card is named this");
    app.state.workbench.activate(Workspace::Design);

    app.answer_design_request(DesignRequest::OpenSourceMap(String::new()));

    assert_eq!(
        app.state
            .workbench
            .navigator_trees
            .filter(Workspace::Netlist),
        "",
        "the jump leaves no filter in the Netlist navigator"
    );
    app.state.workbench.navigator_visible = true;
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    let painted = painted(&output).to_lowercase();
    assert!(
        painted.contains("schematic source"),
        "the navigator lists the line's schematic source:\n{painted}"
    );
}
