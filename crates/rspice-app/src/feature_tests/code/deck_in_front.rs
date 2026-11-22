//! The deck in front of the Netlist workspace, in whole frames of the
//! application, for a project its deck drives: the generated netlist, with
//! no schematic to generate one from, stands on a tab beside the deck's,
//! which brings the deck back, and the project overview's Open SPICE deck
//! and the landing's Open the deck bring the deck to the front whatever was
//! there.

use egui::accesskit::Role;

use crate::feature_tests::replay::{Control, Replay, only};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document::NetlistTab;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;

/// A project its deck drives, on the Netlist workspace with the generated
/// netlist in front: Open generated netlist pressed, with no schematic to
/// generate one from. What the frame after it published.
fn deck_project_on_its_generated_netlist() -> (Replay, Vec<Control>) {
    let mut replay = Replay::of(super::netlist_first::netlist_first_app());
    replay.app.state.workbench.activate(Workspace::Netlist);
    replay.frames(Vec::new());
    assert!(
        matches!(replay.app.state.code.netlist.front, NetlistTab::Deck(_)),
        "the Netlist workspace opens on the deck"
    );
    replay
        .app
        .execute_command_key(CodeCommand::GenerateNetlist.key());
    let shown = replay.frames(Vec::new());
    assert_eq!(
        replay.app.state.code.netlist.front,
        NetlistTab::Generated,
        "Open generated netlist puts the generated netlist in front"
    );
    (replay, shown)
}

/// With nothing generated and the generated netlist in front, the strip
/// shows its tab and the deck's beside it, though the project has one
/// deck; the deck's tab brings the deck back.
#[test]
fn the_decks_tab_stands_beside_the_generated_netlist_and_brings_the_deck_back() {
    let (mut replay, shown) = deck_project_on_its_generated_netlist();
    only(&shown, "Generated netlist", Role::Tab);
    only(&shown, "front_end.sp", Role::Tab);

    replay.click("front_end.sp", Role::Tab);

    assert!(
        matches!(replay.app.state.code.netlist.front, NetlistTab::Deck(_)),
        "the deck's tab brings the deck to the front"
    );
}

/// Open SPICE deck on the project overview brings the deck to the front of
/// the Netlist workspace, whatever was in front there.
#[test]
fn open_spice_deck_brings_the_deck_to_the_front() {
    let (mut replay, _) = deck_project_on_its_generated_netlist();
    replay.app.state.workbench.activate(Workspace::Project);
    replay.app.state.project.project_page = ProjectPage::Overview;

    replay.click("Open SPICE deck", Role::Button);

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Netlist);
    assert!(
        matches!(replay.app.state.code.netlist.front, NetlistTab::Deck(_)),
        "the deck is in front"
    );
}

/// The Design workspace's landing, for a project with no schematic, opens
/// the deck in the Netlist workspace: the deck in front, whatever was there.
#[test]
fn the_landings_open_the_deck_brings_the_deck_to_the_front() {
    let (mut replay, _) = deck_project_on_its_generated_netlist();
    replay.app.state.workbench.activate(Workspace::Design);

    replay.click("Open in Netlist", Role::Button);

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Netlist);
    assert!(
        matches!(replay.app.state.code.netlist.front, NetlistTab::Deck(_)),
        "the deck is in front"
    );
}

/// The landing's other way on makes a schematic: the new cell dialog, set
/// to create one.
#[test]
fn the_landings_create_schematic_opens_the_new_cell_dialog() {
    let mut replay = Replay::of(super::netlist_first::netlist_first_app());
    replay.app.state.workbench.activate(Workspace::Design);
    replay.frames(Vec::new());

    replay.click("Create schematic\u{2026}", Role::Button);

    assert!(replay.app.state.project.new_cell_dialog);
    assert!(replay.app.state.project.new_cell_create_schematic);
}
