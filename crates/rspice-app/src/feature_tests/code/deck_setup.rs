//! Simulate for a project whose simulation is its deck: one table of the
//! deck's own setup, read-only at its lines, Open in Netlist and Run deck on
//! the toolbar, and nothing of a plan to validate or preflight.

use egui::accesskit::Role;
use egui::{Modifiers, Pos2, vec2};

use super::netlist_first::netlist_first_app;
use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{frame, themed_context};
use rspice_app_code::workbench::code::deck_setup::{SetupSection, setup_cards};
use rspice_app_code::workbench::documents::netlist_document::{
    NetlistTab, root_deck_source_index_now,
};
use rspice_app_state::workbench::workspace::Workspace;

/// Every string a whole application frame paints once it has settled.
fn painted_application(app: &mut RSpiceApp) -> String {
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
    let ctx = themed_context();
    let size = vec2(1440.0, 900.0);
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

/// A press and release at `at`, as a reader's click.
fn click_at(replay: &mut Replay, at: Pos2) {
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), press(true)]);
    replay.frames(vec![press(false)]);
}

/// The deck is in front of the Netlist workspace.
fn deck_in_front(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Netlist
        && matches!(app.state.code.netlist.front, NetlistTab::Deck(_))
}

#[test]
fn the_simulate_page_of_a_deck_project_is_one_table_of_its_setup_and_offers_no_plan() {
    let mut app = netlist_first_app();
    app.state.workbench.activate(Workspace::Simulate);
    app.state.workbench.navigator_visible = true;
    app.state.workbench.inspector_visible = true;

    let text = painted_application(&mut app);
    // The page's bar, the table's heads and rows, the toolbar's two
    // actions, and the deck's facts in the inspector.
    for expected in [
        "Deck setup",
        "2 analyses · 0 settings",
        "Line",
        "Statement",
        "Arguments",
        ".tran",
        "1n 1u",
        ".op",
        "Open in Netlist",
        "Run deck",
        "Subcircuits",
    ] {
        assert!(
            text.lines().any(|line| line == expected),
            "missing {expected:?} in:\n{text}"
        );
    }
    // Nothing of a plan: no validation, preflight, plan list, clone, plot
    // or prior run, and no plan editor.
    for absent in [
        "Validate plan",
        "Run preflight",
        "Plans",
        "Clone plan",
        "Show plot",
        "Open prior run",
        "Default plan",
        "Add analysis",
    ] {
        assert!(!text.contains(absent), "{absent:?} painted in:\n{text}");
    }
}

#[test]
fn a_row_opens_the_deck_in_the_netlist_workspace_at_its_line() {
    let mut replay = Replay::of(netlist_first_app());
    replay.app.state.workbench.activate(Workspace::Simulate);
    replay.frames(Vec::new());

    replay.click("Line: 11, Statement: .tran, Arguments: 1n 1u", Role::Row);

    assert!(
        deck_in_front(&replay.app),
        "the deck is in front of Netlist"
    );
    assert_eq!(replay.app.state.code.netlist.cursor_line, 10);
}

#[test]
fn the_toolbar_opens_the_deck_in_the_netlist_workspace() {
    let mut replay = Replay::of(netlist_first_app());
    replay.app.state.workbench.activate(Workspace::Simulate);

    replay.click("Open in Netlist", Role::Button);

    assert!(
        deck_in_front(&replay.app),
        "the deck is in front of Netlist"
    );
}

#[test]
fn a_deck_that_states_no_analysis_says_so_and_opens_in_netlist() {
    const DECK: &str = "no analysis\n\
        V1 in 0 1\n\
        R1 in 0 1k\n\
        .options reltol=1e-4\n\
        .end\n";
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.open_test_deck(DECK.to_owned(), None, "bare.sp"));
    assert!(app.state.is_netlist_first_without_schematic());
    app.state.workbench.activate(Workspace::Simulate);
    let text = painted_application(&mut app);
    for expected in [
        "Deck setup",
        "0 analyses · 1 setting",
        "No analysis",
        "bare.sp states no analysis. Add one in Netlist to run it.",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
    assert!(
        !text.lines().any(|line| line == "Statement"),
        "no table for a deck without an analysis:\n{text}"
    );

    // The toolbar's Open in Netlist, and the page's own below it.
    let mut replay = Replay::of(app);
    let mut buttons: Vec<Pos2> = replay
        .frames(Vec::new())
        .into_iter()
        .filter(|control| control.label == "Open in Netlist" && control.role == Role::Button)
        .map(|control| control.center)
        .collect();
    buttons.sort_by(|a, b| a.y.total_cmp(&b.y));
    let [_toolbar, page] = buttons[..] else {
        panic!("the toolbar and the page each offer Open in Netlist: {buttons:?}");
    };
    click_at(&mut replay, page);

    assert!(
        deck_in_front(&replay.app),
        "the deck is in front of Netlist"
    );
}

#[test]
fn the_setup_lists_analyses_options_temperature_and_top_level_parameters_in_deck_order() {
    const DECK: &str = "setup cards\n\
        .param gain=10\n\
        .subckt amp a y g=1\n\
        .param inner=2\n\
        E1 y 0 a 0 {g}\n\
        .ends\n\
        V1 in 0 1\n\
        X1 in out amp\n\
        .options reltol=1e-4\n\
        + abstol=1e-12\n\
        .temp 27 85\n\
        .step param gain list 1 10\n\
        .tran 1n 1u\n\
        .end\n";
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.open_test_deck(DECK.to_owned(), None, "setup.sp"));
    let index = root_deck_source_index_now(&app.state.model, &mut app.state.code.netlist);
    let cards = setup_cards(&index);
    let listed: Vec<(SetupSection, usize, &str, String)> = cards
        .iter()
        .map(|card| (card.section, card.line, card.statement(), card.arguments()))
        .collect();
    assert_eq!(
        listed,
        vec![
            (SetupSection::Parameters, 2, ".param", "gain=10".to_owned()),
            (
                SetupSection::Options,
                9,
                ".options",
                "reltol=1e-4 abstol=1e-12".to_owned()
            ),
            (SetupSection::Temperature, 11, ".temp", "27 85".to_owned()),
            (
                SetupSection::Analyses,
                12,
                ".step",
                "param gain list 1 10".to_owned()
            ),
            (SetupSection::Analyses, 13, ".tran", "1n 1u".to_owned()),
        ]
    );
}
