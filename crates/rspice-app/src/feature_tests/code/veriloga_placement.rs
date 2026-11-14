//! New Verilog-A device… hands the device back to the schematic it was
//! created from, once the reader's own compile makes it placeable.
//!
//! These pins create a device through the form, compile it the way a
//! worker's answer is published, and press what the compile result offers.

use super::veriloga_interface::{compile_and_publish, press_strip_button};
use crate::state::VerilogAPlacementHandoff;
use crate::state::{Cell, CellViewRef, ComponentType, Library, Tool, View, ViewType};
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::veriloga_placement::{
    offered_veriloga_placement, veriloga_placement_target,
};
use crate::workbench::app::dialogs::library::new_veriloga_device::open_new_veriloga_device_form_in;
use crate::workbench::app::veriloga_design_tests::{
    open_in_design, open_project_app, themed_context,
};
use crate::workbench::app::veriloga_interface_tests::edit_source;
use crate::workbench::app_state::prompt_text;
use crate::workbench::features;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::veriloga;

const LIBRARY: &str = "behavioral";

/// The module the reader writes: two ports, named the way the form named it.
const TWO_PORTS: &str = "`include \"disciplines.vams\"\nmodule gain(inp, out);\n  input inp;\n  output out;\n  electrical inp, out;\n  analog V(out) <+ 2 * V(inp);\nendmodule\n";

/// An open project with a writable library, New Verilog-A device… opened
/// from a `bench` schematic that is not the design's top, and a device `gain`
/// created with no ports. Returns the device's view and the schematic the
/// form was opened from.
fn device_created_from_a_schematic() -> (RSpiceApp, CellViewRef, CellViewRef) {
    let mut app = open_project_app();
    let mut bench = Cell::new("bench");
    bench.add_view(View::new("schematic", ViewType::Schematic));
    let mut library = Library::new(LIBRARY);
    library.add_cell(bench);
    app.state.model.library_manager.add_library(library);
    let origin = CellViewRef::new(LIBRARY, "bench", "schematic");
    app.state
        .model
        .workspace
        .insert_schematic_editor(origin.key(), crate::state::SchematicState::default());
    app.state.open_workspace_view(origin.clone());
    app.state.workbench.activate(Workspace::Design);
    assert_eq!(
        app.state.model.workspace.content.active_view_type(),
        ViewType::Schematic,
        "the form is opened from a schematic"
    );
    open_new_veriloga_device_form_in(&app.state.model, &mut app.state.code.new_veriloga_device);
    let form = &mut app.state.code.new_veriloga_device;
    form.library = LIBRARY.to_owned();
    form.name = "gain".to_owned();
    let reference = app
        .commit_new_veriloga_device()
        .expect("the device is created");
    (app, reference, origin)
}

/// The schematic canvas's interaction, which holds the keyboard once a
/// placement is armed.
fn canvas() -> egui::Id {
    egui::Id::new("rspice-schematic-canvas-interaction")
}

fn offered(app: &RSpiceApp) -> Option<VerilogAPlacementHandoff> {
    app.state
        .code
        .code_workspace
        .veriloga
        .receipt
        .as_ref()?
        .placement
        .clone()
}

/// The starter compile at creation is not the reader's and offers nothing;
/// nor does a compile that leaves the module without ports. The first one
/// that gives it a port offers the schematic the form came from, and Place
/// switches there and arms the device as the shelf would.
#[test]
fn a_new_devices_first_compile_with_a_port_offers_to_place_it_where_it_came_from() {
    let (mut app, reference, origin) = device_created_from_a_schematic();
    assert!(
        offered(&app).is_none(),
        "the starter compile at creation is not the reader's"
    );

    compile_and_publish(&mut app);
    assert!(
        offered(&app).is_none(),
        "a module with no ports has nothing to place"
    );

    edit_source(&mut app, TWO_PORTS);
    compile_and_publish(&mut app);
    assert_eq!(
        offered(&app),
        Some(VerilogAPlacementHandoff {
            view: reference.clone(),
            schematic: Some(origin.clone()),
        })
    );
    assert_eq!(
        offered_veriloga_placement(
            app.state.code.code_workspace.veriloga.receipt.as_ref(),
            &app.state.model
        )
        .map(|(_, target)| target),
        Some(origin.clone())
    );

    veriloga::place_offered_device(&mut features::cx(&mut app), &egui::Context::default())
        .expect("the device arms");
    app.after_contribution();

    assert_eq!(app.state.model.workspace.content.active_view, origin);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let binding = &app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_library_cell
        .as_ref()
        .expect("the device is armed")
        .binding;
    assert_eq!(
        (
            binding.library.as_str(),
            binding.cell.as_str(),
            binding.view.as_str()
        ),
        (LIBRARY, "gain", "veriloga")
    );
    assert_eq!(binding.terminal_order, ["inp", "out"]);
    assert!(offered(&app).is_none(), "the offer is taken");
}

/// Place, pressed on the strip, is carried out in the frame it is pressed
/// in: the schematic it named is open on the Design canvas with the device
/// armed, the offer is taken, and the canvas holds the keyboard.
#[test]
fn pressing_place_arms_the_device_on_the_design_canvas_in_the_pressing_frame() {
    let (mut app, reference, origin) = device_created_from_a_schematic();
    edit_source(&mut app, TWO_PORTS);
    compile_and_publish(&mut app);
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();

    press_strip_button(&ctx, &mut app, "Place in bench");

    assert_eq!(app.state.model.workspace.content.active_view, origin);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let binding = &app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_library_cell
        .as_ref()
        .expect("the device is armed")
        .binding;
    assert_eq!(
        (
            binding.library.as_str(),
            binding.cell.as_str(),
            binding.view.as_str()
        ),
        (LIBRARY, "gain", "veriloga")
    );
    assert!(offered(&app).is_none(), "the offer is taken");
    assert_eq!(
        ctx.memory(|memory| memory.focused()),
        Some(canvas()),
        "the canvas holds the keyboard"
    );
}

/// Place refused when it is taken changes nothing but the status line: the
/// route and the page stay as they were, nothing is armed, the offer
/// stands, and the canvas is not handed the keyboard.
#[test]
fn a_refused_place_changes_no_route_and_gives_no_focus_only_the_prompt() {
    let (mut app, reference, _) = device_created_from_a_schematic();
    edit_source(&mut app, TWO_PORTS);
    compile_and_publish(&mut app);
    open_in_design(&mut app.state, &reference);
    // The strip still offers the device, but a netlist template that cannot
    // bind it makes the cell view unplaceable, so the take refuses.
    {
        let mut library = app
            .state
            .model
            .library_manager
            .edit_library(LIBRARY)
            .expect("the library is writable");
        let mut metadata = library
            .library()
            .get_cell("gain")
            .and_then(|cell| cell.get_view("veriloga"))
            .expect("the device's view")
            .metadata
            .clone();
        metadata.insert("netlist.template".to_owned(), "not a template".to_owned());
        library.replace_view_metadata("gain", "veriloga", metadata);
    }
    let ctx = themed_context();
    let route = app.state.workbench.current_route();

    press_strip_button(&ctx, &mut app, "Place in bench");

    assert_eq!(app.state.workbench.current_route(), route);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_library_cell
            .is_none()
    );
    assert!(offered(&app).is_some(), "the offer stands");
    assert_ne!(ctx.memory(|memory| memory.focused()), Some(canvas()));
    let prompt = prompt_text(&app.state.workbench).expect("the refusal is said");
    assert!(prompt.contains("cannot be placed yet"), "{prompt}");
}

/// The offer belongs to one compile result: a later compile does not make it
/// again.
#[test]
fn the_placement_offer_is_made_once() {
    let (mut app, _, _) = device_created_from_a_schematic();
    edit_source(&mut app, TWO_PORTS);
    compile_and_publish(&mut app);
    assert!(offered(&app).is_some());

    compile_and_publish(&mut app);

    assert!(offered(&app).is_none());
}

/// A remembered schematic whose tab was closed gives way to the design's top
/// schematic. The device's own cell is never offered, and a design with no
/// schematic to offer offers nothing.
#[test]
fn a_closed_origin_offers_the_designs_top_schematic_and_otherwise_nothing() {
    let mut app = open_project_app();
    let mut bench = Cell::new("bench");
    bench.add_view(View::new("schematic", ViewType::Schematic));
    let mut library = Library::new(LIBRARY);
    library.add_cell(bench);
    library.add_cell(Cell::new("gain"));
    app.state.model.library_manager.add_library(library);
    let bench = CellViewRef::new(LIBRARY, "bench", "schematic");
    let handoff = VerilogAPlacementHandoff {
        view: CellViewRef::new(LIBRARY, "gain", "veriloga"),
        schematic: Some(bench.clone()),
    };
    let project = &app.state.model.workspace.content.project;
    let top = CellViewRef::new(&project.root_library, &project.top_cell, "schematic");

    assert_eq!(
        veriloga_placement_target(&app.state.model, &handoff),
        Some(top.clone()),
        "bench's tab is not open"
    );
    app.state.open_workspace_view(bench.clone());
    assert_eq!(
        veriloga_placement_target(&app.state.model, &handoff),
        Some(bench.clone())
    );

    app.state.model.workspace.content.close_view(&bench);
    app.state.model.workspace.content.project.top_cell = "gain".to_owned();
    assert_eq!(veriloga_placement_target(&app.state.model, &handoff), None);
}
