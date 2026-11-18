//! The Netlist workspace from the keyboard: every command it offers has a
//! key or a row in a menu shown there, Show value says at the caret what a
//! hover says under the pointer, an include of a sectioned library binds its
//! section from a select on its row, which Tab reaches as it reaches any
//! control, and a menu's Paste the browser answers late or refuses still
//! pastes or says how to.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{
    DESKTOP, frame, frame_with, open_project_app, themed_context,
};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document;
use rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index;
use rspice_app_code::workbench::documents::virtual_text_editor::{
    BrowserPaste, deliver_browser_paste,
};
use rspice_app_state::state::command_catalog::{CommandKey, CommandPlatform};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::application_menus::ALL_MENUS;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{
    MenuRow, resolve_menu_rows,
};
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;
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

/// Show value is in the deck editor's right-click menu and its Edit menu,
/// offered on the deck and answering there.
#[test]
fn show_value_is_a_row_of_the_deck_editors_menus() {
    let (_ctx, app) = deck_on_the_netlist_workspace(VALUE_DECK);
    let key = CodeCommand::ShowValueAtCaret.key();
    for entries in [
        app.source_editor_menu(),
        app.menu_entries(
            rspice_app_workbench::workbench::chrome::title_bar::application_menus::ApplicationMenu::Edit,
        ),
    ] {
        let rows = resolve_menu_rows(
            entries,
            &app,
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        );
        let row = rows
            .iter()
            .find_map(|row| match row {
                MenuRow::Command(row) if row.command == key => Some(row),
                _ => None,
            })
            .expect("the menu has a Show value row");
        assert_eq!(row.label, "Show value");
        assert_eq!(row.shortcut, "Ctrl+Shift+H");
        assert_eq!(row.availability, CommandAvailability::Available);
    }
}

/// Every text a frame painted.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// Ctrl+Shift+`key`, pressed, as a Linux keyboard gives it.
fn ctrl_shift(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            ctrl: true,
            shift: true,
            command: true,
            ..egui::Modifiers::NONE
        },
    }
}

/// The deck's editor focused with its caret at character `caret`.
fn caret_at(ctx: &egui::Context, app: &mut RSpiceApp, caret: usize) {
    let editor = netlist_document::editor_id_in(&app.state.model, &app.state.code.netlist);
    queue_caret_char_index(ctx, editor, caret);
    ctx.memory_mut(|memory| memory.request_focus(editor));
    frame(ctx, app, DESKTOP);
    frame(ctx, app, DESKTOP);
    assert_eq!(app.state.code.netlist.cursor_char_index, caret);
}

/// Ctrl+Shift+H with the caret on a parameter a card reads shows the card a
/// hover there shows, `w = 2u` and where `w` is defined, at the caret, and
/// says it on the status line; the card is not there before the key, and
/// moving the caret takes it away.
#[test]
fn show_value_shows_at_the_caret_what_a_hover_there_shows() {
    let (ctx, mut app) = deck_on_the_netlist_workspace(VALUE_DECK);
    let reader = VALUE_DECK.find("W={w}").expect("the card reads w") + "W={".len();
    caret_at(&ctx, &mut app, reader);
    let before = frame(&ctx, &mut app, DESKTOP);
    assert!(
        !painted(&before).iter().any(|text| text == "= 2u"),
        "no card before it is asked for"
    );

    frame_with(&ctx, &mut app, DESKTOP, vec![ctrl_shift(egui::Key::H)]);
    let shown = frame(&ctx, &mut app, DESKTOP);
    let texts = painted(&shown);
    assert!(
        texts.iter().any(|text| text == "= 2u"),
        "the card states the value: {texts:?}"
    );
    assert!(
        texts.iter().any(|text| text == "defined line 2"),
        "the card says where the value comes from: {texts:?}"
    );
    assert_eq!(app.state.prompt_text(), Some("w = 2u · defined line 2"));

    caret_at(&ctx, &mut app, reader + 4);
    let moved = frame(&ctx, &mut app, DESKTOP);
    assert!(
        !painted(&moved).iter().any(|text| text == "= 2u"),
        "the card leaves when the caret moves"
    );
}

/// From the palette or a menu, Show value with the caret just after a
/// parameter's name reads that name; on a line with nothing to read, the
/// status line says there is nothing there.
#[test]
fn show_value_reads_the_name_the_caret_ends_and_says_when_there_is_none() {
    let (ctx, mut app) = deck_on_the_netlist_workspace(VALUE_DECK);
    let after = VALUE_DECK.find(".param l=").expect("l is defined") + ".param l".len();
    caret_at(&ctx, &mut app, after);
    app.execute_command_key(CodeCommand::ShowValueAtCaret.key());
    frame(&ctx, &mut app, DESKTOP);
    let shown = frame(&ctx, &mut app, DESKTOP);
    assert!(
        painted(&shown).iter().any(|text| text == "= 6u"),
        "l = {{w*3}} with w = 2u, the value before the name's binding: {:?}",
        app.state.prompt_text()
    );
    assert_eq!(app.state.prompt_text(), Some("l = 6u"));

    // The title line names no parameter, expression or model.
    caret_at(&ctx, &mut app, 2);
    app.execute_command_key(CodeCommand::ShowValueAtCaret.key());
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.prompt_text(),
        Some("No value or name at the caret.")
    );
}

/// A browser that refuses a menu's Paste leaves the deck as it was, and
/// the status line says how to paste from the keyboard, whose paste a
/// browser always hands the page.
#[test]
fn a_refused_paste_says_how_to_paste_from_the_keyboard() {
    let (ctx, mut app) = deck_on_the_netlist_workspace(VALUE_DECK);
    caret_at(&ctx, &mut app, 2);
    let editor = netlist_document::editor_id_in(&app.state.model, &app.state.code.netlist);
    deliver_browser_paste(&ctx, editor, BrowserPaste::Refused);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(
        app.state.prompt_text(),
        Some("The browser did not let RSpice read the clipboard. Press Ctrl+V to paste.")
    );

    // An answer pastes at the caret like the keyboard's paste.
    deliver_browser_paste(&ctx, editor, BrowserPaste::Text("X".to_owned()));
    frame(&ctx, &mut app, DESKTOP);
    assert!(
        app.state
            .model
            .simulation
            .source
            .netlist_content
            .starts_with("vaXlue hover\n"),
        "{}",
        app.state.model.simulation.source.netlist_content
    );
}

/// The amplifier deck kept as text with its corner library, whose `.lib`
/// card binds `tt`.
const CORNER_DECK: &str =
    "* amplifier\n.lib \"corners.lib\" tt\nR1 out 0 {rload}\nV1 out 0 1\n.op\n.end\n";
const CORNERS: &str =
    "* corners\n.lib tt\n.param rload=10k\n.endl tt\n.lib ff\n.param rload=5k\n.endl ff\n";

/// Where the frame put the node of `role` named `label`.
fn node_rect(
    output: &egui::FullOutput,
    role: egui::accesskit::Role,
    label: &str,
) -> Option<egui::Rect> {
    output
        .platform_output
        .accesskit_update
        .as_ref()?
        .nodes
        .iter()
        .find(|(_, node)| node.role() == role && node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .map(|bounds| {
            egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
}

/// A primary click at `at`, and the frames that settle it.
fn click(ctx: &egui::Context, app: &mut RSpiceApp, at: egui::Pos2) -> egui::FullOutput {
    frame_with(ctx, app, DESKTOP, vec![egui::Event::PointerMoved(at)]);
    for pressed in [true, false] {
        frame_with(
            ctx,
            app,
            DESKTOP,
            vec![egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
    frame(ctx, app, DESKTOP);
    frame(ctx, app, DESKTOP)
}

/// The include row of a sectioned library shows a select of its sections
/// with the bound one in its well; choosing another from its list rewrites
/// the deck's own `.lib` card to it.
#[test]
fn an_include_rows_section_select_binds_the_section_chosen() {
    let mut app = open_project_app();
    assert!(
        app.state.import_deck_files(vec![
            ("amp.cir".to_owned(), CORNER_DECK.as_bytes().to_vec()),
            ("corners.lib".to_owned(), CORNERS.as_bytes().to_vec()),
        ]),
        "{:?}",
        app.state.prompt_text()
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    let ctx = themed_context();
    let mut output = frame(&ctx, &mut app, DESKTOP);
    for _ in 0..4 {
        output = frame(&ctx, &mut app, DESKTOP);
    }
    let select = node_rect(
        &output,
        egui::accesskit::Role::ComboBox,
        "Section of corners.lib: tt",
    )
    .expect("the include row offers its section's select");

    let opened = click(&ctx, &mut app, select.center());
    let ff = node_rect(&opened, egui::accesskit::Role::ListBoxOption, "ff")
        .expect("the select lists every section the library declares");
    click(&ctx, &mut app, ff.center());

    assert!(
        app.state
            .model
            .simulation
            .source
            .netlist_content
            .contains(".lib \"corners.lib\" ff\n"),
        "the card binds the section chosen: {}",
        app.state.model.simulation.source.netlist_content
    );
}
