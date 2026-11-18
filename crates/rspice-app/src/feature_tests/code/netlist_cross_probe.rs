//! Going from a line of the generated netlist to what the schematic drew it
//! from, through whole application frames: Show in schematic on the editor's
//! line, from its key, its menu row, or a double-click on an instance's name.

use egui::accesskit::Role;
use egui::{Key, Modifiers, PointerButton, Pos2};

use crate::feature_tests::replay::{Replay, only};
use crate::workbench::RSpiceApp;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document::editor_id_in;
use rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index;
use rspice_app_state::state::{
    CellViewRef, Component, ComponentType, GeneratedSourceMapEntry, Library, LibraryCellInstance,
    Point, PortDirection, PortSpec, SchematicState, View, ViewType,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

/// The top-level resistor, and the instances of the two `amp` cells.
const R9: u64 = 801;
const XA: u64 = 802;
const XB: u64 = 803;
/// The resistor each `amp` holds, both named `R1`.
const WORK_R1: u64 = 811;
const LAB_R1: u64 = 812;
/// The lab amplifier's instance name, long enough that the middle of its
/// card falls inside it.
const XB_NAME: &str = "XLABORATORY_AMPLIFIER_STAGE";

fn resistor(id: u64, name: &str, at: Point) -> Component {
    let mut component = Component::new(id, ComponentType::Resistor, at);
    component.name = name.to_owned();
    component.value = "1k".to_owned();
    component
}

fn lab_amp() -> CellViewRef {
    CellViewRef::new("lab", "amp", "schematic")
}

/// The ports each `amp` draws, and so the interface its instances bind.
const AMP_PORTS: [&str; 2] = ["in", "out"];

/// A design placing cell `amp` from two libraries, `work` and `lab`, whose
/// schematics each hold a resistor named `R1` (the lab's far from the
/// origin), beside a resistor `R9` of its own; generated, with its netlist on
/// the Netlist page.
fn two_amps() -> Replay {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let top = app.state.model.schematic.document_mut_for_test();
    top.components.push(resistor(R9, "R9", Point::new(0, 0)));
    for (id, library, name) in [(XA, "work", "XA"), (XB, "lab", XB_NAME)] {
        let mut binding = LibraryCellInstance::new(library, "amp", "schematic");
        binding.bind_interface(&AMP_PORTS.map(|port| PortSpec {
            name: port.to_owned(),
            direction: PortDirection::InOut,
        }));
        let mut instance = Component::new(id, ComponentType::CellInstance, Point::new(200, 80))
            .with_library_cell(binding);
        instance.name = name.to_owned();
        top.components.push(instance);
    }
    app.state.sync_active_schematic_to_workspace();
    for (library, id, at) in [
        ("work", WORK_R1, Point::new(60, 60)),
        ("lab", LAB_R1, Point::new(4000, 3000)),
    ] {
        let libraries = &mut app.state.model.library_manager;
        if libraries.get_library(library).is_none() {
            libraries.add_library(Library::new(library));
        }
        let cell = libraries
            .get_library_mut(library)
            .expect("the library exists")
            .get_or_create_cell("amp");
        if cell.get_view("schematic").is_none() {
            cell.add_view(View::new("schematic", ViewType::Schematic));
        }
        let mut master = SchematicState::default();
        for (index, port) in AMP_PORTS.into_iter().enumerate() {
            let placed = master.add_component(
                ComponentType::Port,
                Point::new(-200, 200 * i32::try_from(index).expect("two ports")),
            );
            master
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == placed)
                .expect("the placed port is kept")
                .value = port.to_owned();
        }
        master
            .document_mut_for_test()
            .components
            .push(resistor(id, "R1", at));
        app.state
            .model
            .workspace
            .insert_schematic_editor(CellViewRef::new(library, "amp", "schematic").key(), master);
    }
    let mut replay = Replay::of(app);
    replay
        .app
        .execute_command_key(CodeCommand::GenerateNetlist.key());
    replay.frames(Vec::new());
    assert_eq!(replay.app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        replay.app.state.model.netlist_artifacts.generation_error, None,
        "the design generates"
    );
    replay
}

/// The one-based line of the generated netlist written for `component` of
/// the cell view `view`, as the generator recorded it.
fn generated_line(replay: &Replay, view: &CellViewRef, component: u64) -> usize {
    let artifact = replay
        .app
        .state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .expect("the netlist is generated");
    let identity = GeneratedSourceMapEntry::component_identity_for(&view.key(), component);
    *artifact
        .generated_lines_for_component(&identity)
        .first()
        .unwrap_or_else(|| panic!("a line for {identity}:\n{}", artifact.source()))
}

/// The text of the one-based `line` the editor shows.
fn shown_line(replay: &Replay, line: usize) -> String {
    replay
        .app
        .state
        .model
        .simulation
        .source
        .netlist_content
        .split('\n')
        .nth(line - 1)
        .expect("the line is shown")
        .to_owned()
}

/// The editor given the keyboard, with its caret `column` characters into
/// the one-based `line`.
fn caret_at(replay: &mut Replay, line: usize, column: usize) {
    let index = replay
        .app
        .state
        .model
        .simulation
        .source
        .netlist_content
        .split('\n')
        .take(line - 1)
        .map(|text| text.chars().count() + 1)
        .sum::<usize>()
        + column;
    let editor = editor_id_in(&replay.app.state.model, &replay.app.state.code.netlist);
    queue_caret_char_index(replay.ctx(), editor, index);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frames(Vec::new());
    assert_eq!(replay.app.state.code.netlist.cursor_line, line - 1);
}

/// The source editor's menu, opened from the keyboard at the caret.
fn open_menu(replay: &mut Replay) -> Vec<crate::feature_tests::replay::Control> {
    replay.key(Key::F10, Modifiers::SHIFT)
}

fn selected_component(replay: &Replay) -> Option<u64> {
    replay
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()
}

/// Alt+F12 on a line of the `amp` body of the second library opens that
/// library's `amp`, not the first's, with that body's `R1` selected and
/// brought into view. Both bodies write an `R1`; only where the generator
/// wrote each line tells them apart.
#[test]
fn a_body_line_opens_its_own_librarys_cell_with_the_instance_selected() {
    let mut replay = two_amps();
    let line = generated_line(&replay, &lab_amp(), LAB_R1);
    assert!(shown_line(&replay, line).starts_with("R1 "));
    caret_at(&mut replay, line, 0);

    replay.key(Key::F12, Modifiers::ALT);

    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert_eq!(state.model.workspace.content.active_view, lab_amp());
    assert_eq!(selected_component(&replay), Some(LAB_R1));
    // Where the resistor now sits on the canvas: inside the window, which
    // the view the schematic opened at could not show.
    let editor = &state.model.schematic.session.editor;
    let on_canvas = (
        editor.pan.0 + 4000.0 * editor.zoom,
        editor.pan.1 + 3000.0 * editor.zoom,
    );
    assert!(
        (0.0..1440.0).contains(&on_canvas.0) && (0.0..1000.0).contains(&on_canvas.1),
        "the instance is brought into view: {on_canvas:?}"
    );
}

/// Show in schematic from the menu, with the caret on a node of a
/// top-level card, selects and lights that net on the design's schematic.
#[test]
fn a_net_under_the_caret_is_lit_on_the_schematic() {
    let mut replay = two_amps();
    let top = replay.app.state.model.workspace.content.active_view.clone();
    let line = generated_line(&replay, &top, R9);
    let card = shown_line(&replay, line);
    let net = card
        .split_whitespace()
        .nth(1)
        .expect("the card writes its first node")
        .to_owned();
    caret_at(
        &mut replay,
        line,
        card.find(&net).expect("the node is written") + 1,
    );

    open_menu(&mut replay);
    replay.click("Show in schematic", Role::MenuItem);

    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert_eq!(state.model.workspace.content.active_view, top);
    let highlight = &state.model.schematic.session.editor.net_highlight;
    assert!(highlight.active, "the net is lit");
    assert!(
        highlight
            .selected_net_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(&net)),
        "{net} is the net lit: {:?}",
        highlight.selected_net_name
    );
    // A net no wire draws is selected by the instance on it.
    assert_eq!(selected_component(&replay), Some(R9));
}

/// A line the schematic did not write, such as the deck's `.end`, offers
/// Show in schematic greyed, saying why, and the key does nothing.
#[test]
fn a_line_no_schematic_wrote_offers_nothing_and_says_why() {
    let mut replay = two_amps();
    let lines = replay
        .app
        .state
        .model
        .simulation
        .source
        .netlist_content
        .split('\n')
        .count();
    let end = (1..=lines)
        .rev()
        .find(|line| shown_line(&replay, *line).trim() == ".end")
        .expect("the deck ends");
    caret_at(&mut replay, end, 0);

    let menu = open_menu(&mut replay);
    only(&menu, "Show in schematic", Role::MenuItem);
    assert_eq!(
        replay
            .app
            .command_availability(CodeCommand::ShowInSchematic.key()),
        CommandAvailability::Disabled("this line has no schematic source")
    );
    replay.key(Key::Escape, Modifiers::NONE);
    replay.key(Key::F12, Modifiers::ALT);
    assert_eq!(replay.app.state.workbench.workspace, Workspace::Netlist);
}

/// The middle of the one-based `line`, as the editor publishes it: the
/// line the caret is on, so the caret is put on it first.
fn middle_of(replay: &mut Replay, line: usize) -> Pos2 {
    caret_at(replay, line, 0);
    let text = shown_line(replay, line);
    replay
        .frames(Vec::new())
        .into_iter()
        .find(|control| control.role == Role::TextRun && control.value.trim_end() == text)
        .unwrap_or_else(|| panic!("the editor publishes {text:?}"))
        .center
}

/// A double-click on an instance's name on the generated netlist goes to
/// that instance on its schematic.
#[test]
fn double_clicking_an_instance_name_shows_it_on_the_schematic() {
    let mut replay = two_amps();
    let top = replay.app.state.model.workspace.content.active_view.clone();
    let line = generated_line(&replay, &top, XB);
    assert!(shown_line(&replay, line).starts_with(XB_NAME));
    let at = middle_of(&mut replay, line);
    let click = |pressed| egui::Event::PointerButton {
        pos: at,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(at)]);
    replay.frame(vec![click(true), click(false)]);
    replay.frame(vec![click(true), click(false)]);
    replay.frames(Vec::new());

    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert_eq!(state.model.workspace.content.active_view, top);
    assert_eq!(selected_component(&replay), Some(XB));
}
