//! The Netlist workspace from the keyboard: every command it offers has a
//! key or a row in a menu shown there.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{
    DESKTOP, frame, open_project_app, themed_context,
};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_state::state::command_catalog::{CommandKey, CommandPlatform};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::application_menus::ALL_MENUS;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{
    MenuRow, resolve_menu_rows,
};
use rspice_app_workbench::workbench::framework::host::Host;

/// The code feature's commands that act only away from the Netlist
/// workspace, where their own menus and keys carry them: the Verilog-A cell
/// view's compile, its build settings and its symbol, and the schematic's
/// jump into the generated netlist. Every other command of the feature acts
/// on the Netlist workspace, a command added later included.
const ELSEWHERE: [CodeCommand; 5] = [
    CodeCommand::CompileVerilogA,
    CodeCommand::CancelVerilogACompile,
    CodeCommand::VerilogABuildSettings,
    CodeCommand::VerilogAOpenSymbol,
    CodeCommand::ShowInNetlist,
];

/// A deck with parameters, a derived one and a card that reads both, so a
/// hover has an answer on each.
const VALUE_DECK: &str = "value hover\n\
.param w=2u\n\
.param l={w*3}\n\
M1 d g s b nch W={w} L={l*2}\n\
.model nch nmos\n\
.end\n";

/// The application on the Netlist workspace with `deck` open and its
/// editor drawn.
fn deck_on_the_netlist_workspace(deck: &str) -> (egui::Context, RSpiceApp) {
    let mut app = open_project_app();
    app.state.seed_test_deck(deck.to_owned());
    app.state.workbench.activate(Workspace::Netlist);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    frame(&ctx, &mut app, DESKTOP);
    (ctx, app)
}

/// The keys of every row `entries` shows on `platform`, a row that waits for
/// its command to be offered included: whenever the command is offered, the
/// row is there.
fn menu_keys(
    app: &RSpiceApp,
    platform: CommandPlatform,
    os: egui::os::OperatingSystem,
) -> Vec<CommandKey> {
    fn walk(rows: &[MenuRow], out: &mut Vec<CommandKey>) {
        for row in rows {
            match row {
                MenuRow::Command(row) => out.push(row.command),
                MenuRow::Submenu { rows, .. } => walk(rows, out),
                MenuRow::Separator => {}
            }
        }
    }
    let mut keys = Vec::new();
    for entries in ALL_MENUS
        .into_iter()
        .map(|menu| app.menu_entries(menu))
        .chain([app.source_editor_menu()])
    {
        walk(&resolve_menu_rows(entries, app, platform, os), &mut keys);
    }
    keys
}

/// The route table: each command the Netlist workspace offers is reached by
/// its key or by a row of a menu the workspace shows, on the desktop and in
/// the browser, where a browser keeps some of the desktop's keys for itself.
#[test]
fn every_command_the_netlist_workspace_offers_has_a_key_or_a_menu_row() {
    let (_ctx, app) = deck_on_the_netlist_workspace(VALUE_DECK);
    assert!(
        app.state.workbench.active_source_editor.is_some(),
        "the deck's editor is in front"
    );
    for (platform, os) in [
        (CommandPlatform::Desktop, egui::os::OperatingSystem::Windows),
        (CommandPlatform::Browser, egui::os::OperatingSystem::Nix),
    ] {
        let rows = menu_keys(&app, platform, os);
        let unreached: Vec<&str> = CodeCommand::ALL
            .into_iter()
            .filter(|command| !ELSEWHERE.contains(command))
            .filter(|command| {
                let keyed = command
                    .declared_spec()
                    .bindings
                    .iter()
                    .any(|binding| binding.platforms.contains(&platform));
                !keyed && !rows.contains(&command.key())
            })
            .map(|command| command.declared_spec().label)
            .collect();
        assert!(
            unreached.is_empty(),
            "{platform:?}: no key and no menu row for {unreached:?}"
        );
    }
}
