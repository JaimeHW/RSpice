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
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::state::Workspace;

use super::replay::Replay;

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
    app.execute_command_key(DesignCommand::PlaceBusTap.key());
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
    app.execute_command_key(DesignCommand::PlacePin.key());
    assert!(draws_dialog(&mut app, "Create pins"));
    assert!(app.state.design.pin_port.open);
}

#[test]
fn place_text_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    app.execute_command_key(DesignCommand::PlaceText.key());
    assert!(draws_dialog(&mut app, "Place text or design note"));
    assert!(app.state.design.design_note.open);
}

#[test]
fn draw_documentation_shape_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    app.execute_command_key(DesignCommand::PlaceShape.key());
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
    app.execute_command_key(DesignCommand::RenameSelection.key());
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
    app.execute_command_key(DesignCommand::GridSnapRouting.key());
    assert!(draws_dialog(&mut app, "Grid, snap and wire routing"));
    assert!(app.state.design.grid_snap_routing.open);
}

#[test]
fn display_options_open_and_draw_in_the_frame() {
    let mut app = design_app();
    app.execute_command_key(DesignCommand::VisibilityOptions.key());
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

/// Descend into hierarchy's step draws through the feature registry, which
/// answers what it raised straight after it draws, so nothing runs between
/// the reader's choice and the shell carrying it out.
#[test]
fn the_descend_step_has_its_choice_answered_straight_after_it_draws() {
    let squeeze = |text: &str| text.split_whitespace().collect::<String>();
    let order = squeeze(include_str!("../workbench/frame/dialog_order.rs"));
    let spec = order
        .split("DialogSpec{")
        .find(|spec| spec.contains("DialogKey(\"design.descend_hierarchy\")"))
        .expect("the shell's dialog order draws Descend into hierarchy");
    assert!(spec.contains(&squeeze(
        "render: |app, ctx| features::render_dialog(
            app, ctx, DialogKey(\"design.descend_hierarchy\")
        ),"
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

#[test]
fn design_review_comments_open_and_draw_in_the_frame() {
    let mut app = design_app();
    app.execute_command_key(DesignCommand::ReviewComments.key());
    assert!(draws_dialog(&mut app, "Design review comments"));
    assert!(app.state.design.design_review_comments.open);
}

#[test]
fn design_rule_setup_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    app.execute_command_key(DesignCommand::RuleSetup.key());
    assert!(draws_dialog(&mut app, "Design rule setup"));
    assert!(app.state.design.rule_setup.open);
}

#[test]
fn bulk_edit_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    resistor(&mut app, Point::new(100, 100));
    app.execute_command_key(DesignCommand::SelectionBulkEdit.key());
    assert!(draws_dialog(&mut app, "Bulk edit"));
    assert!(app.state.design.selection_bulk_edit.open);
}

#[test]
fn engineering_table_view_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    resistor(&mut app, Point::new(100, 100));
    app.execute_command_key(DesignCommand::EngineeringTableView.key());
    assert!(draws_dialog(&mut app, "Engineering table view"));
    assert!(app.state.design.engineering_table.open);
}

#[test]
fn stimulus_link_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(100, 100));
    let reference = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the source is placed")
        .spice_instance_name();
    crate::workbench::app::open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        crate::workbench::app::StimulusLinkMode::Extract,
    )
    .expect("the save dialog opens");
    assert!(draws_dialog(
        &mut app,
        &format!("Save {reference} as a definition")
    ));
    assert!(app.state.design.stimulus_link.open);
}

#[test]
fn create_model_bound_symbol_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    crate::workbench::app::open_create_model_bound_symbol_dialog(&mut app.state);
    assert!(draws_dialog(&mut app, "Create model-bound symbol"));
    assert!(app.state.design.create_model_bound_symbol.open);
}

#[test]
fn the_component_editor_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let id = resistor(&mut app, Point::new(0, 0));
    let name = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the placed resistor")
        .name
        .clone();
    crate::workbench::app::open_property_editor(&mut app.state, id);
    assert!(draws_dialog(&mut app, &format!("{name} properties")));
    assert!(app.state.design.tabbed_property_dialog.open);
}

/// Re-adopting a stimulus from the component editor reopens the editor on
/// the instance's new card in the frame the reader chose it, and the status
/// bar says so.
#[test]
fn a_readoption_reopens_the_editor_in_the_frame_it_is_chosen() {
    use crate::state::stimulus_library::definition::StimulusDefinition;
    use crate::state::stimulus_library::draft::DefinitionDraft;
    use crate::state::stimulus_library::now_unix_ms;

    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    let model = &mut replay.app.state.model;
    let mut definition =
        StimulusDefinition::new("sensor_drive", ComponentType::VoltageSourceSin, now_unix_ms)
            .expect("a definition");
    definition.value = "0".to_owned();
    definition.params = "va=1 freq=1k".to_owned();
    let mut source = Component::new(7, ComponentType::VoltageSourceSin, Point::origin());
    source.name = "V1".to_owned();
    definition.adopt_onto(&mut source).expect("adopt");
    let library = &mut model.workspace.content.stimulus_library;
    library.insert(definition).expect("insert");
    let mut draft = DefinitionDraft::new(library.get("sensor_drive").expect("held").clone());
    draft.edit(|working| working.params = "va=1 freq=2k".to_owned());
    library.apply(&mut draft, now_unix_ms);
    model
        .schematic
        .document_mut_for_test()
        .components
        .push(source);
    crate::workbench::app::open_property_editor(&mut replay.app.state, 7);

    replay.click("Re-adopt r2", Role::Button);

    let state = &replay.app.state;
    let source = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == 7)
        .expect("the source");
    assert!(source.params.contains("freq=2k"), "{}", source.params);
    assert!(state.design.tabbed_property_dialog.open);
    assert_eq!(state.design.tabbed_property_dialog.component_id, Some(7));
    assert_eq!(
        state.prompt_text(),
        Some("V1 re-adopted sensor_drive r2; its card is the library's again.")
    );
}

#[test]
fn the_connectivity_manager_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    resistor(&mut app, Point::new(0, 0));
    crate::workbench::app::open_connectivity_manager(
        &mut app.state.design.connectivity_manager,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.platform,
    );
    assert!(draws_dialog(
        &mut app,
        "Connectivity, buses and global nets"
    ));
    assert!(app.state.design.connectivity_manager.open);
}

/// A net revealed from the connectivity manager is revealed in the frame it
/// is chosen: the manager closes and the canvas selects what the net joins.
#[test]
fn a_connectivity_reveal_is_carried_out_in_the_frame_it_is_chosen() {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    replay
        .app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    crate::workbench::app::open_connectivity_manager(
        &mut replay.app.state.design.connectivity_manager,
        &mut replay.app.state.model,
        &mut replay.app.state.workbench,
        &replay.app.state.platform,
    );
    let at = replay
        .frames(Vec::new())
        .into_iter()
        .find(|control| control.role == Role::Button && control.label == "Reveal")
        .expect("a net row offers Reveal")
        .center;
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), press(true)]);
    replay.frames(vec![press(false)]);
    let state = &replay.app.state;
    assert!(!state.design.connectivity_manager.open);
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert!(!state.model.schematic.session.editor.selection.is_empty());
}

/// Three resistors in a chain with the first two selected; the ids of the
/// selected two.
fn two_of_three_resistors_selected(app: &mut RSpiceApp) -> (u64, u64) {
    let schematic = &mut app.state.model.schematic;
    let first = schematic.add_component(ComponentType::Resistor, Point::origin());
    let second = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
    schematic.add_component(ComponentType::Resistor, Point::new(160, 0));
    schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
    schematic.add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
    schematic.session.editor.selection.select_component(first);
    schematic.session.editor.selection.select_component(second);
    (first, second)
}

#[test]
fn create_hierarchy_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    two_of_three_resistors_selected(&mut app);
    crate::workbench::app::open_create_hierarchy_dialog(
        &mut app.state.design.create_hierarchy,
        &app.state.model,
        &mut app.state.workbench,
    );
    assert!(draws_dialog(&mut app, "Create hierarchical cell"));
    assert!(app.state.design.create_hierarchy.open);
}

/// An extraction chosen in Create hierarchy is carried out in the frame it is
/// chosen: the new cell opens and the form closes.
#[test]
fn an_extraction_is_carried_out_in_the_frame_it_is_chosen() {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    two_of_three_resistors_selected(&mut replay.app);
    let parent = replay.app.state.model.workspace.content.active_view.clone();
    crate::workbench::app::open_create_hierarchy_dialog(
        &mut replay.app.state.design.create_hierarchy,
        &replay.app.state.model,
        &mut replay.app.state.workbench,
    );
    let child = replay.app.state.design.create_hierarchy.cell_name.clone();
    replay.click("Create and descend", Role::Button);
    let state = &replay.app.state;
    assert!(!state.design.create_hierarchy.open);
    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new(&parent.library, &child, "schematic")
    );
}

/// A voltage source placed and selected alone; its id.
fn selected_voltage_source(app: &mut RSpiceApp) -> u64 {
    let schematic = &mut app.state.model.schematic;
    let id = schematic.add_component(ComponentType::VoltageSource, Point::origin());
    schematic.session.editor.selection.select_only_component(id);
    id
}

#[test]
fn replace_instance_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    selected_voltage_source(&mut app);
    crate::workbench::app::open_replace_instance_dialog(
        &mut app.state.design.replace_instance,
        &app.state.design.property_registry,
        &app.state.model,
        &mut app.state.workbench,
    );
    assert!(draws_dialog(&mut app, "Replace instance"));
    assert!(app.state.design.replace_instance.open);
}

/// Replace swaps the selected instance for the chosen master in the frame it
/// is pressed, closes the form and says what it kept.
#[test]
fn a_replacement_is_carried_out_in_the_frame_it_is_chosen() {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    selected_voltage_source(&mut replay.app);
    crate::workbench::app::open_replace_instance_dialog(
        &mut replay.app.state.design.replace_instance,
        &replay.app.state.design.property_registry,
        &replay.app.state.model,
        &mut replay.app.state.workbench,
    );
    assert!(replay.app.state.design.replace_instance.open);
    replay.click("Replace", Role::Button);
    let state = &replay.app.state;
    assert!(!state.design.replace_instance.open);
    assert!(
        state
            .prompt_text()
            .is_some_and(|text| text.starts_with("Replaced ")),
        "{:?}",
        state.prompt_text()
    );
}

/// The lookup-table XSPICE device's placement form, opened over the open
/// schematic; the device's display name.
fn open_lookup_table_placement(app: &mut RSpiceApp) -> &'static str {
    let descriptor = crate::state::engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.model_type == "d_lut")
        .expect("the lookup-table device");
    let ports = crate::state::builtin_xspice_vector_ports(descriptor).expect("its vector ports");
    let source = crate::workbench::app::schematic_editor_request_source(&app.state);
    app.state.design.builtin_xspice_placement.open(
        descriptor.stable_id,
        descriptor.display_name,
        ports,
        source,
    );
    descriptor.display_name
}

#[test]
fn builtin_xspice_placement_opens_and_draws_in_the_frame() {
    let mut app = design_app();
    let name = open_lookup_table_placement(&mut app);
    assert!(draws_dialog(&mut app, &format!("Configure {name}")));
    assert!(app.state.design.builtin_xspice_placement.open);
}

/// Place arms the configured device in the frame it is pressed and closes
/// the form.
#[test]
fn a_configured_device_is_armed_in_the_frame_it_is_placed() {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    open_lookup_table_placement(&mut replay.app);
    replay.click("Place", Role::Button);
    let state = &replay.app.state;
    assert!(!state.design.builtin_xspice_placement.open);
    assert!(
        state
            .prompt_text()
            .is_some_and(|text| text.ends_with("is armed; it will snap to the schematic grid.")),
        "{:?}",
        state.prompt_text()
    );
}
