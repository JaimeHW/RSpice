//! The design feature observed through a whole application: each dialog it
//! draws, opened as its command opens it and drawn by the real frame through
//! the feature's own context.

use egui::accesskit::{Node, Role};
use egui::{Context, Rect, pos2, vec2};

use crate::state::{
    Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance,
    ModelBoundSymbolDefinition, Point, SymbolDefinitionImport, View, ViewType,
};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;

/// A whole application on the Design workspace of an open project.
fn design_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app
}

/// Place a resistor on the open schematic; its id.
fn resistor(app: &mut RSpiceApp, at: Point) -> u64 {
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, at)
}

/// Three frames of the whole application; the last one's AccessKit nodes.
fn frame_nodes(app: &mut RSpiceApp) -> Vec<Node> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut frame = eframe::Frame::_new_kittest();
    let mut nodes = Vec::new();
    for pass in 1..=3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0))),
                time: Some(f64::from(pass) * 0.1),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        );
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes.into_iter().map(|(_, node)| node).collect()
}

/// Three frames of the whole application; whether the last one drew a
/// dialog titled `title`.
fn draws_dialog(app: &mut RSpiceApp, title: &str) -> bool {
    frame_nodes(app)
        .iter()
        .any(|node| node.role() == Role::Dialog && node.label() == Some(title))
}

/// How many of the last of three frames' nodes are labelled `label`.
fn labelled(app: &mut RSpiceApp, label: &str) -> usize {
    frame_nodes(app)
        .iter()
        .filter(|node| node.label() == Some(label))
        .count()
}

#[test]
fn place_bus_tap_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceBusTap.execute(&mut app);
    assert!(draws_dialog(&mut app, "Place bus tap"));
    assert!(app.state.design.bus_tap.open);
}

/// The net label is an entry on the canvas, not a dialog: the card it draws
/// carries its title.
#[test]
fn net_label_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let before = labelled(&mut app, "Net label");
    assert!(crate::workbench::app::open_net_label_placement(
        &mut app.state,
        Point::new(0, 0)
    ));
    assert_eq!(labelled(&mut app, "Net label"), before + 1);
    assert!(app.state.design.net_label_placement.open);
}

#[test]
fn create_pins_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlacePin.execute(&mut app);
    assert!(draws_dialog(&mut app, "Create pins"));
    assert!(app.state.design.pin_port.open);
}

#[test]
fn place_text_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceText.execute(&mut app);
    assert!(draws_dialog(&mut app, "Place text or design note"));
    assert!(app.state.design.design_note.open);
}

#[test]
fn draw_documentation_shape_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::PlaceShape.execute(&mut app);
    assert!(draws_dialog(&mut app, "Draw documentation shape"));
    assert!(app.state.design.documentation_shape.open);
}

#[test]
fn create_array_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = resistor(&mut app, Point::new(0, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(id);
    Command::ArraySelection.execute(&mut app);
    assert!(draws_dialog(&mut app, "Create array"));
    assert!(app.state.design.array_selection.open);
}

#[test]
fn net_label_properties_open_and_draw_in_the_frame() {
    let mut app = design_app();
    let label = app
        .state
        .model
        .schematic
        .add_net_label(Point::new(0, 0), "sense".to_owned());
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_net_label(label);
    Command::ObjectProperties.execute(&mut app);
    assert!(draws_dialog(&mut app, "Net label properties"));
    assert!(app.state.design.object_properties.open);
}

#[test]
fn rename_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = resistor(&mut app, Point::new(0, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(id);
    Command::RenameSelection.execute(&mut app);
    assert!(draws_dialog(&mut app, "Rename"));
    assert!(app.state.design.rename_selection.open);
}

#[test]
fn find_and_replace_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::FindReplace.execute(&mut app);
    assert!(draws_dialog(&mut app, "Find and replace"));
    assert!(app.state.design.find_replace.open);
}

#[test]
fn shared_properties_open_and_draw_in_the_frame() {
    let mut app = design_app();
    let first = resistor(&mut app, Point::new(0, 0));
    let second = resistor(&mut app, Point::new(100, 0));
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_component(first);
    selection.select_component(second);
    Command::ObjectProperties.execute(&mut app);
    assert!(draws_dialog(&mut app, "Properties"));
    assert!(app.state.design.shared_properties.open);
}

#[test]
fn grid_snap_and_routing_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    Command::GridSnapRouting.execute(&mut app);
    assert!(draws_dialog(&mut app, "Grid, snap and wire routing"));
    assert!(app.state.design.grid_snap_routing.open);
}

#[test]
fn display_options_open_and_draw_in_the_frame() {
    let mut app = design_app();
    Command::VisibilityOptions.execute(&mut app);
    assert!(draws_dialog(&mut app, "Display options"));
    assert!(app.state.design.schematic_visibility_dialog.open);
}

/// An instance of a schematic cell placed on the open schematic and
/// selected; the master it descends into.
fn selected_hierarchy_instance(app: &mut RSpiceApp) -> CellViewRef {
    let mut cell = Cell::new("afe_core");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut library = Library::new("hierarchy_test");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    let mut instance =
        Component::new(91, ComponentType::CellInstance, Point::new(100, 100)).with_library_cell(
            LibraryCellInstance::new("hierarchy_test", "afe_core", "schematic"),
        );
    instance.name = "XAFE".to_owned();
    let schematic = &mut app.state.model.schematic;
    schematic.document_mut_for_test().components.push(instance);
    schematic.session.editor.selection.select_only_component(91);
    CellViewRef::new("hierarchy_test", "afe_core", "schematic")
}

#[test]
fn descend_into_hierarchy_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    selected_hierarchy_instance(&mut app);
    Command::DescendHierarchy.execute(&mut app);
    assert!(draws_dialog(&mut app, "Descend into hierarchy"));
    assert!(app.state.design.descend_hierarchy.open);
}

/// Descend into hierarchy's step has what it raised answered straight after
/// it draws, so nothing runs between the reader's choice and the shell
/// carrying it out.
#[test]
fn the_descend_step_has_its_choice_answered_straight_after_it_draws() {
    let squeeze = |text: &str| text.split_whitespace().collect::<String>();
    let order = squeeze(include_str!("../workbench/frame/dialog_order.rs"));
    let spec = order
        .split("DialogSpec{")
        .find(|spec| spec.contains("DialogKey(\"design.descend_hierarchy\")"))
        .expect("the shell's dialog order draws Descend into hierarchy");
    assert!(spec.contains(&squeeze(
        "render: |app, ctx| {
            DesignFeature::render_descend_hierarchy_dialog(&mut features::cx(app), ctx);
            app.after_contribution();
        },"
    )));
}

/// A descent chosen in a frame is carried out in that frame: the master
/// opens, the dialog closes and the status bar says so.
#[test]
fn a_descent_is_carried_out_in_the_frame_it_is_chosen() {
    let mut app = design_app();
    let child = selected_hierarchy_instance(&mut app);
    Command::DescendHierarchy.execute(&mut app);
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut frame = eframe::Frame::_new_kittest();
    let mut run = |app: &mut RSpiceApp, time: f64, events: Vec<egui::Event>| {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0))),
                time: Some(time),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        );
    };
    run(&mut app, 0.1, Vec::new());
    run(&mut app, 0.2, Vec::new());
    assert!(app.state.design.descend_hierarchy.open);
    assert_ne!(app.state.model.workspace.content.active_view, child);
    // The form's first control takes the caret, and a focused control keeps
    // Enter for itself; with nothing focused, Enter is the primary.
    ctx.memory_mut(|memory| {
        if let Some(id) = memory.focused() {
            memory.surrender_focus(id);
        }
    });
    let enter = egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: Some(egui::Key::Enter),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    run(&mut app, 0.3, vec![enter]);
    assert_eq!(app.state.model.workspace.content.active_view, child);
    assert!(!app.state.design.descend_hierarchy.open);
    assert_eq!(
        app.state.prompt_text(),
        Some("Descended into the instance; the parent's unsaved changes are kept.")
    );
}

#[test]
fn import_symbol_definition_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    crate::workbench::app::open_symbol_import_dialog(&mut app.state);
    assert!(draws_dialog(&mut app, "Import symbol definition"));
    assert!(app.state.design.symbol_import.open);
}

#[test]
fn the_component_form_designer_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let library_name = app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();
    let template = ModelBoundSymbolDefinition::review_only(&library_name, "review_form");
    let imported = SymbolDefinitionImport::from_bytes(
        br#"<svg><rect x="0" y="0" width="40" height="20"/></svg>"#,
        "review.svg",
        Some(template),
    )
    .expect("typed review definition");
    let library = app
        .state
        .model
        .library_manager
        .get_library_mut(&library_name)
        .expect("project library");
    crate::state::prepare_symbol_construction(&imported.definition, library)
        .expect("construction plan")
        .commit(library)
        .expect("commit definition");
    app.state
        .model
        .library_manager
        .select_view(&library_name, "review_form", "symbol");
    crate::workbench::app::open_symbol_parameter_form_dialog(&mut app.state);
    assert!(draws_dialog(&mut app, "Open component form designer"));
    assert!(app.state.design.symbol_parameter_form.open);
}
