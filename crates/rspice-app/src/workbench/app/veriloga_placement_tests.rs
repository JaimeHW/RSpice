//! New Verilog-A device… hands the device back to the schematic it was
//! created from, once the reader's own compile makes it placeable.
//!
//! These pins create a device through the form, compile it the way a
//! worker's answer is published, and press what the compile result offers.

use super::veriloga_design_tests::{DESKTOP, open_project_app};
use super::veriloga_interface_tests::{compile_and_publish, edit_source};
use super::*;
use crate::state::{Cell, CellViewRef, ComponentType, Library, Tool, View, ViewType};
use crate::workbench::documents::code_workspace::VerilogAPlacementHandoff;
use crate::workbench::state::Workspace;

const LIBRARY: &str = "behavioral";

/// The module the reader writes: two ports, named the way the form named it.
const TWO_PORTS: &str = "`include \"disciplines.vams\"\nmodule rspice_gain_va(inp, out);\n  input inp;\n  output out;\n  electrical inp, out;\n  analog V(out) <+ 2 * V(inp);\nendmodule\n";

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
    app.state.library_manager.add_library(library);
    let origin = CellViewRef::new(LIBRARY, "bench", "schematic");
    app.state
        .workspace
        .insert_schematic_editor(origin.key(), crate::state::SchematicState::default());
    app.state.open_workspace_view(origin.clone());
    app.state.workbench.activate(Workspace::Design);
    assert_eq!(
        app.state.workspace.content.active_view_type(),
        ViewType::Schematic,
        "the form is opened from a schematic"
    );
    crate::workbench::app::open_new_veriloga_device_form(&mut app.state);
    let form = &mut app.state.dialogs.new_veriloga_device;
    form.library = LIBRARY.to_owned();
    form.name = "gain".to_owned();
    let reference = app
        .commit_new_veriloga_device()
        .expect("the device is created");
    (app, reference, origin)
}

fn offered(app: &RSpiceApp) -> Option<VerilogAPlacementHandoff> {
    app.state
        .ui
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
        app.state
            .offered_veriloga_placement()
            .map(|(_, target)| target),
        Some(origin.clone())
    );

    let armed = app
        .state
        .place_offered_veriloga_device()
        .expect("the device arms");

    assert!(armed.contains(&origin.display_path()), "{armed}");
    assert_eq!(app.state.workspace.content.active_view, origin);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.schematic.session.editor.tool,
        Tool::Place(ComponentType::CellInstance)
    );
    let binding = &app
        .state
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
    app.state.library_manager.add_library(library);
    let bench = CellViewRef::new(LIBRARY, "bench", "schematic");
    let handoff = VerilogAPlacementHandoff {
        view: CellViewRef::new(LIBRARY, "gain", "veriloga"),
        schematic: Some(bench.clone()),
    };
    let project = &app.state.workspace.content.project;
    let top = CellViewRef::new(&project.root_library, &project.top_cell, "schematic");

    assert_eq!(
        app.state.veriloga_placement_target(&handoff),
        Some(top.clone()),
        "bench's tab is not open"
    );
    app.state.open_workspace_view(bench.clone());
    assert_eq!(
        app.state.veriloga_placement_target(&handoff),
        Some(bench.clone())
    );

    app.state.workspace.content.close_view(&bench);
    app.state.workspace.content.project.top_cell = "gain".to_owned();
    assert_eq!(app.state.veriloga_placement_target(&handoff), None);
}

/// New Verilog-A device… empty, filled and refusing a port, at desktop and
/// phone widths, and the compile result offering "Place in <schematic>".
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_new_device_form_and_the_placement_offer_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let mut write = |name: &str, canvas: &crate::ui::raster::Canvas, height: usize| {
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    for (name, cell, ports) in [
        ("new-veriloga-device-empty", "", ""),
        (
            "new-veriloga-device-filled",
            "opamp_ideal",
            "inp:in inn:in out:out vdd vss",
        ),
        (
            "new-veriloga-device-refused",
            "opamp_ideal",
            "inp:in out:sideways",
        ),
    ] {
        for (suffix, size) in [("1440x900", DESKTOP), ("390x844", egui::vec2(390.0, 844.0))] {
            let mut app = open_project_app();
            app.state.library_manager.add_library(Library::new(LIBRARY));
            app.state.workbench.activate(Workspace::Design);
            app.state.workbench.coarse_pointer = size.x < 500.0;
            crate::workbench::app::open_new_veriloga_device_form(&mut app.state);
            let form = &mut app.state.dialogs.new_veriloga_device;
            form.library = LIBRARY.to_owned();
            form.name = cell.to_owned();
            form.ports = ports.to_owned();
            let canvas = crate::ui::raster::render(size, |ui, _| {
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            });
            write(&format!("{name}-{suffix}"), &canvas, size.y as usize);
        }
    }

    let (mut app, _, _) = device_created_from_a_schematic();
    edit_source(&mut app, TWO_PORTS);
    compile_and_publish(&mut app);
    for (suffix, size) in [("1440x900", DESKTOP), ("390x844", egui::vec2(390.0, 844.0))] {
        app.state.workbench.coarse_pointer = size.x < 500.0;
        let canvas = crate::ui::raster::render(size, |ui, _| app.render_frame_chrome(ui));
        write(
            &format!("veriloga-place-offer-{suffix}"),
            &canvas,
            size.y as usize,
        );
    }
}
