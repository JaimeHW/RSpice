//! The canvas context menu driven through a whole application.

use crate::workbench::app_state::AppState;
use crate::workbench::commands::answer_surface_requests;
use egui::{Context, Key, Modifiers, Rect, Ui, pos2, vec2};
use rspice_app_design::schematic::view::SchematicSymbolContext;
use rspice_app_design::schematic::view::context_menu::{ARRANGE_ENTRIES, ContextEntry};
use rspice_app_design::schematic::view::schematic_symbol_context;
use rspice_app_design::workbench::app::dialogs::replace_instance::replace_instance_available;
use rspice_app_design::workbench::app::dialogs::selection::commands::duplicate_schematic_selection_at;
use rspice_app_design::workbench::app_state::interaction_state::ContextTarget;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_state::state::Point;
use rspice_app_state::state::Tool;
use rspice_app_state::state::{
    Component, ComponentType, DesignNote, DesignNoteKind, DocumentationShape,
    DocumentationShapeGeometry, Junction, LibraryCellInstance, NetLabel, Wire,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_schematic_editor::view::context_menu::ContextMenuBinding;
use rspice_schematic_editor::view::context_menu::ContextMenuRequest;
use rspice_schematic_editor::view::context_menu::ContextMenuView;
use rspice_schematic_editor::view::context_menu::{self as context_surface, ContextAction};
use rspice_schematic_editor::view::pointer_target::PointerTarget;

/// The row names its shortcut, the Descend command, so the command must do
/// what the row does on a Verilog-A instance: it was disabled there, because
/// a module is not a schematic master.
#[test]
fn the_descend_shortcut_opens_a_veriloga_instances_view_as_its_row_does() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let (_, reference) = select_a_veriloga_instance(&mut app.state);
    assert!(app.command_key_enabled(DesignCommand::DescendHierarchyDirect.key()));
    assert!(
        app.command_key_enabled(DesignCommand::DescendHierarchy.key()),
        "Descend with options opens a module as its form offers"
    );

    app.execute_command_key(DesignCommand::DescendHierarchyDirect.key());

    assert_eq!(app.state.model.workspace.content.active_view, reference);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
}

// The menu answers over the canvas's facts and acts over its parts. These
// lend both from a test's state under the menu's own names, and answer what
// the menu asked of another workspace as the shell does.

fn action_availability(action: ContextAction, state: &AppState) -> (bool, &'static str) {
    rspice_app_design::schematic::view::context_menu::action_availability(
        action,
        &state.canvas_facts(),
    )
}

fn menu_entries(state: &AppState, target: ContextTarget) -> Vec<ContextEntry> {
    rspice_app_design::schematic::view::context_menu::menu_entries(&state.canvas_facts(), target)
}

fn selection_summary(state: &AppState, target: ContextTarget) -> String {
    rspice_app_design::schematic::view::context_menu::selection_summary(
        &state.canvas_facts(),
        target,
    )
}

fn context_menu_binding(state: &AppState) -> Option<ContextMenuBinding> {
    rspice_app_design::schematic::view::context_menu::context_menu_binding(&state.canvas_facts())
}

fn context_menu_view(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    ctx: &Context,
) -> Option<ContextMenuView> {
    rspice_app_design::schematic::view::context_menu::context_menu_view(
        &state.canvas_facts(),
        symbol_context,
        ctx,
    )
}

fn execute_context_action(
    action: ContextAction,
    state: &mut AppState,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    rspice_app_design::schematic::view::context_menu::execute_context_action(
        action,
        &mut state.canvas_parts(),
        click_pos,
        symbol_context,
    );
    answer_surface_requests(state);
}

fn apply_context_request(
    state: &mut AppState,
    request: ContextMenuRequest,
    symbol_context: &SchematicSymbolContext,
) {
    rspice_app_design::schematic::view::context_menu::apply_context_request(
        &mut state.canvas_parts(),
        request,
        symbol_context,
        &Context::default(),
    );
    answer_surface_requests(state);
}

fn open_operating_point(state: &mut AppState) {
    rspice_app_design::schematic::view::context_menu::open_operating_point(
        &mut state.canvas_parts(),
    );
    answer_surface_requests(state);
}

/// A project open on the Design workspace, where a row that raises its
/// command has it carried out.
fn design_app() -> crate::workbench::RSpiceApp {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app
}

/// A row clicked, and then the command it raised, as the frame carries it
/// out.
fn run_context_action(
    app: &mut crate::workbench::RSpiceApp,
    action: ContextAction,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    execute_context_action(action, &mut app.state, click_pos, symbol_context);
    app.after_contribution();
}

fn select_context_target(state: &mut AppState, target: Option<PointerTarget>) -> ContextTarget {
    rspice_app_design::schematic::view::context_menu::select_context_target(
        &mut state.canvas_parts(),
        target,
    )
}

fn context_key_input(key: Key) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
        focused: true,
        events: vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
        ..Default::default()
    }
}

/// One Verilog-A instance of `behavioral/gain/veriloga`, whose view owns its
/// module's source, selected.
fn state_with_selected_veriloga_instance() -> (AppState, u64, rspice_app_state::state::CellViewRef)
{
    let mut state = AppState::default();
    let (id, reference) = select_a_veriloga_instance(&mut state);
    (state, id, reference)
}

/// Place one Verilog-A instance of `behavioral/gain/veriloga` on `state`'s
/// otherwise empty schematic and select it.
pub(crate) fn select_a_veriloga_instance(
    state: &mut AppState,
) -> (u64, rspice_app_state::state::CellViewRef) {
    use rspice_app_state::state::{
        Cell, CellViewRef, Library, PortDirection, PortSpec, ProjectSourceBundle,
        ProjectSourceOwner, View, ViewType,
    };

    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let reference = CellViewRef::new("behavioral", "gain", "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut cell = Cell::new("gain");
    cell.add_view(view);
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(reference.clone()),
                "gain.va",
                "module gain(p, n);\n  inout p, n;\n  electrical p, n;\nendmodule\n",
                [],
                [],
            )
            .expect("a cell-view bundle"),
        )
        .expect("one bundle per cell view");
    let mut binding = LibraryCellInstance::new("behavioral", "gain", "veriloga");
    binding.bind_interface(&[
        PortSpec {
            name: "p".to_owned(),
            direction: PortDirection::InOut,
        },
        PortSpec {
            name: "n".to_owned(),
            direction: PortDirection::InOut,
        },
    ]);
    let id = state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    (id, reference)
}

/// "Open Verilog-A view" opens the module's cell view as a Design tab: no
/// descent, because a module has no occurrences below it.
#[test]
fn open_veriloga_view_opens_the_instances_module_in_design() {
    let (mut state, _, reference) = state_with_selected_veriloga_instance();
    assert!(action_availability(ContextAction::OpenVerilogAView, &state).0);
    assert!(
        !action_availability(ContextAction::DescendHierarchy, &state).0,
        "a module is not a schematic master to descend into"
    );

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::OpenVerilogAView,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert_eq!(state.model.workspace.content.active_view, reference);
    assert_eq!(state.workbench.workspace, Workspace::Design);
    let open = state
        .model
        .workspace
        .content
        .open_views
        .iter()
        .find(|open| open.reference == reference)
        .expect("the module's view is an open tab");
    assert_eq!(
        open.occurrence,
        rspice_design::occurrence::DocumentOccurrence::rooted(reference.clone()),
        "opening a module records no occurrence step"
    );
}

/// New Verilog-A device… from the canvas opens the form, and is refused with
/// a reason where no library could take the cell.
#[test]
fn new_veriloga_device_opens_the_form_from_the_canvas() {
    let mut state = AppState::default();
    assert!(action_availability(ContextAction::NewVerilogADevice, &state).0);
    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::NewVerilogADevice,
        &mut state,
        Point::origin(),
        &symbol_context,
    );
    assert!(state.code.new_veriloga_device.open);

    let names = state
        .model
        .library_manager
        .libraries_sorted()
        .iter()
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    for name in names {
        state
            .model
            .library_manager
            .get_library_mut(&name)
            .expect("the library")
            .read_only = true;
    }
    let (enabled, reason) = action_availability(ContextAction::NewVerilogADevice, &state);
    assert!(!enabled);
    assert!(reason.contains("writable"), "{reason}");
}

/// A dialog the menu asked for counts as open from the moment it asked, so
/// a later gesture in the same frame is refused, as it was when the menu
/// opened the dialog in place. The shell then opens it.
#[test]
fn a_dialog_the_menu_asked_for_holds_the_canvas_modal_until_it_opens() {
    let mut state = AppState::default();
    let symbol_context = schematic_symbol_context(&state);
    assert!(!state.canvas_parts().application_modal_open());
    rspice_app_design::schematic::view::context_menu::execute_context_action(
        ContextAction::NewVerilogADevice,
        &mut state.canvas_parts(),
        Point::origin(),
        &symbol_context,
    );
    assert!(!state.code.new_veriloga_device.open);
    assert!(!state.application_modal_open());
    assert!(state.canvas_application_modal_open());
    let parts = state.canvas_parts();
    assert!(parts.application_modal_open());
    assert!(parts.dialog_modal_open());

    answer_surface_requests(&mut state);
    assert!(state.code.new_veriloga_device.open);
    assert!(state.application_modal_open());
    assert!(state.dialog_modal_open());
}

#[test]
fn focused_keyboard_context_row_activates_with_enter_or_space() {
    for key in [Key::Enter, Key::Space] {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        let mut app = design_app();
        let state = &mut app.state;
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .clear();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(7, ComponentType::Resistor, Point::origin()));
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(7);
        state.design.interaction.context_target = Some((ContextTarget::Component(7), (0, 0)));
        let symbol_context = schematic_symbol_context(&*state);

        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_context_contents(ui, state, &symbol_context, 27.0, true);
            });
        });

        let mut key_still_available = true;
        let _ = ctx.run_ui(context_key_input(key), |root| {
            egui::CentralPanel::default().show(root, |ui| {
                render_context_contents(ui, state, &symbol_context, 27.0, false);
                key_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(Modifiers::NONE, key));
            });
        });

        app.after_contribution();
        assert!(
            app.state.design.tabbed_property_dialog.open || app.state.design.object_properties.open,
            "{key:?} should activate the focused Object properties row"
        );
        assert!(
            !key_still_available,
            "{key:?} should be owned by the focused context-menu row"
        );
    }
}

#[test]
fn net_label_context_exposes_the_complete_object_lifecycle() {
    let mut app = design_app();
    let label = NetLabel::new(73, Point::new(40, 40), "afe_out");
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);

    assert!(action_availability(ContextAction::Properties, &app.state).0);
    assert!(action_availability(ContextAction::Copy, &app.state).0);
    assert!(action_availability(ContextAction::Duplicate, &app.state).0);
    assert!(action_availability(ContextAction::Delete, &app.state).0);
    assert!(selection_summary(&app.state, ContextTarget::Canvas).contains("net label · afe_out"));

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::Properties,
        label.pos,
        &symbol_context,
    );
    assert!(app.state.design.object_properties.open);
    assert!(!app.state.design.rename_selection.open);
    assert!(matches!(
        app.state.design.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::NetLabel(ref draft))
            if draft.original == label
    ));
}

#[test]
fn design_note_context_exposes_only_compatible_object_lifecycle_actions() {
    let mut app = design_app();
    let note = DesignNote::new(
        74,
        Point::new(40, 40),
        DesignNoteKind::ReviewNote,
        "Review bias path",
    )
    .unwrap();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(note.clone());
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_design_note(note.id);

    assert!(action_availability(ContextAction::Properties, &app.state).0);
    assert!(action_availability(ContextAction::Copy, &app.state).0);
    assert!(action_availability(ContextAction::Duplicate, &app.state).0);
    assert!(action_availability(ContextAction::Delete, &app.state).0);
    // A note turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &app.state).0);
    assert!(action_availability(ContextAction::Mirror, &app.state).0);
    let summary = selection_summary(&app.state, ContextTarget::Canvas);
    assert!(summary.contains("Review note"));
    assert!(summary.contains("Review bias path"));

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::Properties,
        note.pos,
        &symbol_context,
    );
    assert!(matches!(
        app.state.design.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DesignNote(ref draft))
            if draft.original == note
    ));
}

#[test]
fn documentation_shape_context_exposes_the_complete_non_electrical_lifecycle() {
    let mut app = design_app();
    let shape = DocumentationShape::new(
        75,
        DocumentationShapeGeometry::Callout {
            tip: Point::new(10, 10),
            elbow: Point::new(30, 20),
            box_corner: Point::new(80, 50),
        },
    )
    .unwrap();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .documentation_shapes
        .push(shape.clone());
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_documentation_shape(shape.id);

    assert!(action_availability(ContextAction::Properties, &app.state).0);
    assert!(action_availability(ContextAction::Copy, &app.state).0);
    assert!(action_availability(ContextAction::Duplicate, &app.state).0);
    assert!(action_availability(ContextAction::Delete, &app.state).0);
    // A shape turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &app.state).0);
    assert!(action_availability(ContextAction::Mirror, &app.state).0);
    assert!(selection_summary(&app.state, ContextTarget::Canvas).contains("Callout"));

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::Properties,
        Point::new(30, 20),
        &symbol_context,
    );
    assert!(matches!(
        app.state.design.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DocumentationShape(ref draft))
            if draft.original == shape
    ));
}

/// The canvas row asks the schematic the same staleness question the
/// Design command asks, and runs the same repair. A row with its own
/// answer would offer the repair where the deck does not need it.
#[test]
fn the_interface_repair_row_is_offered_and_runs_only_for_a_stale_instance() {
    const MASTER: &str = "work/div/schematic";

    let mut app = design_app();
    let mut master = rspice_app_state::state::SchematicState::default();
    let port = master.add_component(ComponentType::Port, Point::new(20, 0));
    master
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained")
        .value = "a".to_owned();
    let mut binding = LibraryCellInstance::new("work", "div", "schematic");
    binding.bind_interface(&master.interface_ports());
    app.state
        .model
        .workspace
        .insert_schematic_editor(MASTER.to_owned(), master);
    let instance = app
        .state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(instance);

    assert!(
        !action_availability(ContextAction::UpdateInstanceInterface, &app.state).0,
        "a placement that still matches its master is not stale"
    );

    app.state
        .model
        .workspace
        .schematic_editor_mut(MASTER)
        .expect("the fixture registers the master")
        .editor
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.value == "a")
        .expect("the master declares port a")
        .value = "ain".to_owned();

    assert!(action_availability(ContextAction::UpdateInstanceInterface, &app.state).0);

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::UpdateInstanceInterface,
        Point::origin(),
        &symbol_context,
    );

    assert_eq!(
        app.state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .expect("the instance stays bound")
            .terminal_order,
        ["ain"]
    );
    assert!(!action_availability(ContextAction::UpdateInstanceInterface, &app.state).0);
}

/// The replacement row is offered exactly where the command is, and opens
/// the same reviewed transaction. A row with its own answer would offer a
/// replacement the dialog would immediately refuse.
#[test]
fn the_replace_instance_row_is_offered_and_runs_only_for_one_replaceable_instance() {
    let mut app = design_app();
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &app.state).0,
        "no selection is not one replaceable instance"
    );

    let first = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(40, 0));
    let second = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(120, 0));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    assert!(action_availability(ContextAction::ReplaceInstance, &app.state).0);
    assert_eq!(
        action_availability(ContextAction::ReplaceInstance, &app.state).0,
        replace_instance_available(&app.state.design.property_registry, &app.state.model),
        "the row must answer with the command's own predicate"
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(second);
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &app.state).0,
        "two selected instances are not one replaceable instance"
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::ReplaceInstance,
        Point::origin(),
        &symbol_context,
    );

    assert!(app.state.design.replace_instance.open);
    assert_eq!(app.state.design.replace_instance.source_component_id, first);
}

/// One reported device, selected on the canvas.
fn state_with_reported_device_op(device: &str) -> AppState {
    use rspice_app_state::state::{AnalysisResult, AnalysisType, SimulationRun};

    let mut state = AppState::default();
    let mut component = Component::new(1, ComponentType::Nmos, Point::new(40, 30));
    component.name = device.to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);

    let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
    analysis.device_op = Some(rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: device.to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: vec![("id", 1.0e-4)],
            module: None,
        }],
    });
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.model.simulation.retained.runs.insert(0, run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

#[test]
fn operating_point_hop_carries_the_clicked_device_into_the_op_inspector() {
    let mut state = state_with_reported_device_op("M1");
    assert!(action_availability(ContextAction::OperatingPoint, &state).0);

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.results.results.session.viewer, ResultViewer::Op);
    assert_eq!(
        state.results.results.session.op_filter, "M1",
        "the Op inspector must arrive selected on the device the menu named"
    );
}

#[test]
fn operating_point_hop_leaves_the_report_unfiltered_when_the_device_is_unreported() {
    let mut state = state_with_reported_device_op("M1");
    // A second instance the run never reported. Filtering to it would show
    // an empty inspector instead of the report the reader asked to open.
    let mut unreported = Component::new(2, ComponentType::Nmos, Point::new(80, 30));
    unreported.name = "M2".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(unreported);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(2);

    // A stale filter is the failure this guards: the previous device's name
    // was left in place, so the inspector opened on another instance's row
    // and read as a hop that had worked.
    state.results.results.session.op_filter = "M1".to_owned();

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.results.results.session.viewer, ResultViewer::Op);
    assert!(state.results.results.session.op_filter.is_empty());
}

/// The hop selects the operating point, not just the workspace and viewer.
///
/// The Op inspector renders the *selected* analysis. A run of [OP, TRAN] with
/// the transient selected — which is where a reader usually is when they go
/// looking for a bias point — landed on "not a DC operating-point result": the
/// workspace and the viewer were right and the one thing that decides what they
/// show was left wherever it had been.
#[test]
fn operating_point_hop_selects_the_operating_point_analysis() {
    use rspice_app_state::state::{AnalysisResult, AnalysisType};

    let mut state = state_with_reported_device_op("M1");
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    state
        .model
        .simulation
        .retained
        .runs
        .first_mut()
        .expect("the fixture run is retained")
        .add_analysis(transient);
    state.model.simulation.view.active_analysis_idx = Some(1);
    assert_eq!(
        state
            .model
            .simulation
            .active_analysis()
            .expect("the fixture selects an analysis")
            .analysis_type,
        AnalysisType::Transient,
        "the reader starts on the transient"
    );

    open_operating_point(&mut state);

    assert_eq!(
        state
            .model
            .simulation
            .active_analysis()
            .expect("the hop leaves an analysis selected")
            .analysis_type,
        AnalysisType::DcOp,
        "the hop must select the result its viewer can render"
    );
    assert_eq!(state.results.results.session.op_filter, "M1");
}

fn render_context_contents(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    row_height: f32,
    focus_first: bool,
) {
    let view = context_menu_view(state, symbol_context, ui.ctx()).expect("context target");
    if let Some(request) =
        context_surface::render_context_contents(ui, &view, row_height, focus_first)
    {
        apply_context_request(state, request, symbol_context);
    }
}

#[test]
fn context_requests_recheck_the_opening_selection_source_and_authority() {
    for change in [
        "source",
        "selection",
        "target",
        "tool",
        "read-only",
        "modal",
        "none",
    ] {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .clear();
        for id in [7, 8] {
            state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .push(Component::new(
                    id,
                    ComponentType::Resistor,
                    Point::new(id as i32 * 10, 0),
                ));
        }
        state.model.schematic.init_undo_history();
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(7);
        state.design.interaction.context_target = Some((ContextTarget::Component(7), (70, 0)));
        let request = ContextMenuRequest {
            binding: context_menu_binding(&state).unwrap(),
            action: ContextAction::Delete,
        };
        match change {
            "source" => state.bump_active_schematic_epoch(),
            "selection" => state
                .model
                .schematic
                .session
                .editor
                .selection
                .select_only_component(8),
            "target" => {
                state.design.interaction.context_target =
                    Some((ContextTarget::Component(8), (80, 0)))
            }
            "tool" => state.model.schematic.session.editor.tool = Tool::Wire,
            "read-only" => state.model.schematic.session.read_only = true,
            "modal" => state.settings.preferences_open = true,
            "none" => {}
            _ => unreachable!(),
        }
        let before = state.model.schematic.document().clone();
        let selection = state.model.schematic.session.editor.selection.clone();
        let symbol_context = schematic_symbol_context(&state);
        apply_context_request(&mut state, request, &symbol_context);
        if change == "none" {
            assert_eq!(state.model.schematic.document().components.len(), 1);
            assert_eq!(state.model.schematic.document().components[0].id, 8);
            assert!(state.model.schematic.undo());
            assert_eq!(
                state.model.schematic.document().components,
                before.components
            );
        } else {
            assert_eq!(
                state.model.schematic.document().components,
                before.components,
                "{change}"
            );
            assert_eq!(
                state.model.schematic.session.editor.selection, selection,
                "{change}"
            );
            assert!(!state.model.schematic.can_undo(), "{change}");
        }
    }
}

/// Sheet 2 reuses Sheet 1's layout: a wire joins its two parts at the very
/// points where R3 and R4 stand unconnected on Sheet 1, and a junction sits
/// where Sheet 1's selected wires cross without one. Copy took that wire
/// and that dot as if they joined the selection, so every paste carried a
/// short the source sheet does not have (audit U-5).
#[test]
fn copy_takes_no_wire_or_junction_from_another_sheet() {
    let mut state = super::two_sheet_state(&[1, 3, 5, 7], &[2, 6]);
    let r3 = Component::new(1, ComponentType::Resistor, Point::origin());
    let r4 = Component::new(3, ComponentType::Resistor, Point::new(100, 0));
    let r3_right = r3.terminal_positions()[1].1;
    let r4_left = r4.terminal_positions()[0].1;
    let drawing = state.model.schematic.document_mut_for_test();
    drawing.components.extend([r3, r4]);
    drawing.wires.push(Wire::segment(2, r3_right, r4_left));
    drawing
        .wires
        .push(Wire::segment(5, Point::new(-50, -50), Point::new(150, -50)));
    drawing
        .wires
        .push(Wire::segment(7, Point::new(50, -100), Point::new(50, 0)));
    drawing
        .junctions
        .push(Junction::new(6, Point::new(50, -50)));
    let selection = &mut state.model.schematic.session.editor.selection;
    for id in [1, 3] {
        selection.select_component(id);
    }
    for id in [5, 7] {
        selection.select_wire(id);
    }

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Copy,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    let clipboard = &state.design.schematic_clipboard;
    assert_eq!(clipboard.components.len(), 2);
    let mut copied: Vec<u64> = clipboard.wires.iter().map(|wire| wire.id).collect();
    copied.sort_unstable();
    assert_eq!(copied, vec![5, 7], "Sheet 2's wire was copied");
    assert!(
        clipboard.junctions.is_empty(),
        "Sheet 2's junction was copied: {:?}",
        clipboard.junctions
    );
}

/// G-1 and G-12: while a duplicate floats under the pointer, R turns it, and
/// so does the long-press menu's Rotate row, which is a finger's R. The part
/// it was copied from stays as it was.
#[test]
fn the_rotate_row_turns_a_floating_duplicate_not_its_original() {
    let mut app = design_app();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(duplicate_schematic_selection_at(
        app.state.design.floating_copy_parts(),
        &mut app.state.model,
        &mut app.state.workbench,
        Point::new(60, 30)
    ));
    let symbol_context = schematic_symbol_context(&app.state);

    assert!(action_availability(ContextAction::Rotate, &app.state).0);
    run_context_action(
        &mut app,
        ContextAction::Rotate,
        Point::new(60, 30),
        &symbol_context,
    );
    assert!(rspice_app_design::workbench::app::dialogs::selection::commands::floating::place_floating_paste(
        &mut app.state.design.floating_paste,
        &mut app.state.model,
        &mut app.state.workbench
    ));
    let quarter = rspice_design::schematic::rotation::Rotation::default().rotate_cw();
    let turned: Vec<_> = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|part| (part.id == 41, part.rotation))
        .collect();
    assert_eq!(
        turned,
        vec![(true, Default::default()), (false, quarter)],
        "only the copy turned"
    );
}

/// The canvas menu had Copy, Duplicate and Delete but no Cut, no Paste and
/// no vertical mirror, so a reader working from the menu had to reach for
/// keys to finish the job (audit E-9). A part's menu offers Cut and Mirror
/// vertically, the sheet's offers Paste once something is copied, and each
/// row runs its command: Cut takes the part onto the clipboard, Paste puts
/// it back where the menu was opened, and Mirror vertically flips the
/// selection top to bottom.
#[test]
fn cut_paste_and_mirror_vertically_rows_run_their_commands() {
    use rspice_app_design::schematic::view::selection_transform::SelectionTransform;

    let mut app = design_app();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    let symbol_context = schematic_symbol_context(&app.state);

    let actions = |state: &AppState, target| -> Vec<ContextAction> {
        menu_entries(state, target)
            .iter()
            .filter_map(|entry| match entry {
                ContextEntry::Command(command) => Some(command.action),
                ContextEntry::Separator => None,
            })
            .collect()
    };
    let part_menu = actions(&app.state, ContextTarget::Component(41));
    for action in [ContextAction::Cut, ContextAction::MirrorVertical] {
        assert!(
            part_menu.contains(&action),
            "the part's menu lacks {action:?}"
        );
    }
    assert!(action_availability(ContextAction::Cut, &app.state).0);
    assert!(
        !action_availability(ContextAction::Paste, &app.state).0,
        "nothing has been copied yet"
    );

    run_context_action(
        &mut app,
        ContextAction::Cut,
        Point::origin(),
        &symbol_context,
    );
    assert!(app.state.model.schematic.document().components.is_empty());
    assert_eq!(app.state.design.schematic_clipboard.components.len(), 1);
    assert!(actions(&app.state, ContextTarget::Canvas).contains(&ContextAction::Paste));
    assert!(action_availability(ContextAction::Paste, &app.state).0);

    let click = Point::new(200, 100);
    let offset = click - app.state.design.schematic_clipboard.origin;
    run_context_action(&mut app, ContextAction::Paste, click, &symbol_context);
    assert!(rspice_app_design::workbench::app::dialogs::selection::commands::floating::place_floating_paste(
        &mut app.state.design.floating_paste,
        &mut app.state.model,
        &mut app.state.workbench
    ));
    let pasted = app.state.model.schematic.document().components.clone();
    assert_eq!(pasted.len(), 1);
    assert_eq!(pasted[0].pos, Point::new(20, 30) + offset);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .has_component(pasted[0].id)
    );

    assert!(action_availability(ContextAction::MirrorVertical, &app.state).0);
    run_context_action(
        &mut app,
        ContextAction::MirrorVertical,
        click,
        &symbol_context,
    );
    let mirrored = &app.state.model.schematic.document().components[0];
    assert_ne!(
        (mirrored.mirror_v, mirrored.mirror_h, mirrored.rotation),
        (pasted[0].mirror_v, pasted[0].mirror_h, pasted[0].rotation),
        "the part was not flipped"
    );
    assert_eq!(
        app.state.model.schematic.undo_description(),
        Some(SelectionTransform::MirrorVertical.description())
    );
}

/// The Arrange page offers Snap Selection to Grid, the command the Design
/// menu's Arrange ends with, under the same command id, and its row puts an
/// off-grid part back on the grid in one undo step. A phone has no menu bar,
/// so without the row a finger could not snap at all.
#[test]
fn the_arrange_page_snaps_the_selection_to_the_grid() {
    let mut app = design_app();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .clear();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(
            41,
            ComponentType::Resistor,
            Point::new(323, 367),
        ));
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    let symbol_context = schematic_symbol_context(&app.state);
    let snap = ARRANGE_ENTRIES
        .iter()
        .find_map(|entry| match entry {
            ContextEntry::Command(command)
                if command.shortcut_command == Some(DesignCommand::SnapSelectionToGrid.key()) =>
            {
                Some(command)
            }
            _ => None,
        })
        .expect("the Arrange page offers Snap Selection to Grid");
    assert_eq!(
        snap.label,
        DesignCommand::SnapSelectionToGrid.declared_spec().label
    );
    assert!(
        !action_availability(snap.action, &app.state).0,
        "nothing is selected to snap"
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(action_availability(snap.action, &app.state).0);
    run_context_action(&mut app, snap.action, Point::origin(), &symbol_context);

    assert_eq!(app.state.model.schematic.document().grid_size, 10);
    assert_eq!(
        app.state.model.schematic.document().components[0].pos,
        Point::new(320, 370)
    );
    assert_ne!(
        app.state.model.schematic.document().components[0].pos,
        Point::new(323, 367)
    );
    assert_eq!(
        app.state.model.schematic.undo_description(),
        Some("snap selection to grid")
    );
}

/// Under "Selected net across hierarchy" an Alt+click net highlight is
/// carried into the instance a user descends into. Picking that instance
/// cleared it first: the click before the descend key or a double-click,
/// and the context menu that offers Descend, so the child lit nothing and
/// neither did the parent on the way back up. Picking a descendable
/// instance now leaves it lit, and so does picking an ordinary part.
#[test]
fn picking_an_instance_to_descend_keeps_the_net_highlight_it_carries() {
    use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES};
    use rspice_app_state::state::SchematicNetHighlighting;
    use rspice_schematic_editor::requests::{EditorAction, EditorRequest};

    fn lit(state: &AppState) -> Option<String> {
        let highlight = &state.model.schematic.session.editor.net_highlight;
        highlight
            .active
            .then(|| highlight.selected_net_name.clone())
            .flatten()
    }
    fn click(state: &mut AppState, id: u64) {
        let request = EditorRequest {
            source: rspice_app_design::schematic::view::requests::editor_request_source(state),
            selection: state.model.schematic.session.editor.selection.clone(),
            action: EditorAction::SelectPointer {
                target: Some(PointerTarget::Component(id)),
                additive: false,
                alt_held: false,
            },
        };
        rspice_app_design::schematic::view::requests::apply_editor_request(
            &mut state.canvas_parts(),
            request,
        );
    }

    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.open_workspace_view(reference.top.clone());
    state.design.schematic_visibility.net_highlighting =
        SchematicNetHighlighting::SelectedNetAcrossHierarchy;
    let component_named = |state: &AppState, name: &str| {
        state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == name)
            .map(|component| component.id)
            .expect("top places the component")
    };
    let x1 = component_named(&state, AMP_INSTANCES[0]);

    for by_menu in [false, true] {
        assert!(state.light_net_named("in"), "top has a net named in");
        if by_menu {
            select_context_target(&mut state, Some(PointerTarget::Component(x1)));
        } else {
            click(&mut state, x1);
        }
        assert_eq!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .single_component(),
            Some(x1)
        );
        assert_eq!(lit(&state).as_deref(), Some("in"), "menu {by_menu}");

        let symbol_context = schematic_symbol_context(&state);
        execute_context_action(
            ContextAction::DescendHierarchy,
            &mut state,
            Point::origin(),
            &symbol_context,
        );
        assert_eq!(lit(&state).as_deref(), Some("IN"), "X1's port is lit");
        state.navigation().ascend_workspace_level();
        assert_eq!(lit(&state).as_deref(), Some("in"), "back up, in is lit");
        state.model.schematic.session.editor.selection.clear();
    }

    let part = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| {
            rspice_app_design::workbench::app::actions::workspace::hierarchy_master_for_component(
                &state.model,
                component.id,
            )
            .is_none()
        })
        .map(|component| component.id)
        .expect("top places an ordinary part");
    click(&mut state, part);
    assert_eq!(
        lit(&state).as_deref(),
        Some("in"),
        "an ordinary part keeps it"
    );
}

/// Align and Distribute say what the selection lacks in the layout's own
/// words, on the Design menu and on the right-click Arrange page alike: a
/// third part to distribute, or the wire that cannot be lined up. Both said
/// something else, the menu that the command was unavailable and the page
/// that two objects were needed while two were selected.
#[test]
fn align_and_distribute_name_what_the_selection_lacks_on_both_routes() {
    use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;
    use rspice_schematic_editor::view::context_menu::ArrangeAction;
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let document = app.state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.wires.clear();
    for (id, x) in [(41, 0), (42, 120)] {
        document.components.push(Component::new(
            id,
            ComponentType::Resistor,
            Point::new(x, 0),
        ));
    }
    document
        .wires
        .push(Wire::new(5, vec![Point::new(0, 200), Point::new(100, 200)]));
    app.state.sync_active_schematic_to_workspace();
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(41);
    selection.select_component(42);

    let third = "Select at least three compatible objects to distribute";
    assert_eq!(
        app.command_availability(DesignCommand::DistributeSelectionHorizontal.key()),
        CommandAvailability::Disabled(third)
    );
    assert_eq!(
        action_availability(
            ContextAction::ArrangeCommand(ArrangeAction::DistributeHorizontal),
            &app.state
        ),
        (false, third)
    );
    assert!(app.command_key_enabled(DesignCommand::AlignSelectionLeft.key()));

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(5);
    let conductor = "Select only instances and non-electrical annotations; conductors and net-label anchors cannot be aligned safely";
    assert_eq!(
        app.command_availability(DesignCommand::AlignSelectionLeft.key()),
        CommandAvailability::Disabled(conductor)
    );
    assert_eq!(
        action_availability(
            ContextAction::ArrangeCommand(ArrangeAction::AlignLeft),
            &app.state
        ),
        (false, conductor)
    );
}

/// Right-click > Properties on two selected parts opens the form for what
/// they share, as Q and Edit > Properties do. The row was greyed, asking for
/// one object, while Q on the same selection opened the form.
#[test]
fn the_properties_row_opens_the_shared_form_for_several_parts() {
    let mut app = design_app();
    let document = app.state.model.schematic.document_mut_for_test();
    document.components.clear();
    for (id, x) in [(41, 0), (42, 120)] {
        document.components.push(Component::new(
            id,
            ComponentType::Resistor,
            Point::new(x, 0),
        ));
    }
    app.state.sync_active_schematic_to_workspace();
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_only_component(41);
    selection.select_component(42);
    let target = select_context_target(&mut app.state, Some(PointerTarget::Component(42)));
    assert_eq!(target, ContextTarget::Component(42));
    assert!(app.command_key_enabled(DesignCommand::ObjectProperties.key()));
    assert!(action_availability(ContextAction::Properties, &app.state).0);

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::Properties,
        Point::new(120, 0),
        &symbol_context,
    );

    assert!(app.state.design.shared_properties.open);
    assert!(!app.state.design.object_properties.open);
}

/// A row runs through the dispatcher as its menu-bar command does, so "."
/// repeats it: Rotate from the right-click menu, then ".", turns the part a
/// second quarter. "." repeated whatever had last run from a key, a menu or
/// the toolbar instead.
#[test]
fn repeat_runs_again_what_a_right_click_row_ran() {
    use crate::workbench::commands::vocabulary::ShellCommand;

    let mut app = design_app();
    let document = app.state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.components.push(Component::new(
        41,
        ComponentType::Resistor,
        Point::new(20, 30),
    ));
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    let rotation = |app: &crate::workbench::RSpiceApp| {
        app.state.model.schematic.document().components[0].rotation
    };
    let quarter = rspice_design::schematic::rotation::Rotation::default().rotate_cw();

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::Rotate,
        Point::new(20, 30),
        &symbol_context,
    );
    assert_eq!(rotation(&app), quarter);

    app.execute_command_key(ShellCommand::RepeatLastCommand.key());
    app.after_contribution();
    assert_eq!(rotation(&app), quarter.rotate_cw());
}

/// Reset label position, run from a part's menu, puts its moved name and
/// value back where the layout places them, leaves the part itself and an
/// unmoved part alone, and one undo moves them back.
#[test]
fn the_reset_label_row_puts_a_moved_name_and_value_back_as_one_undo_step() {
    use rspice_design::schematic::label_layout::TextEdge;
    use rspice_design::schematic::label_position::LabelPosition;

    let placed = LabelPosition::Placed {
        offset: Point::new(60, -40),
        horizontal: TextEdge::Start,
        vertical: TextEdge::End,
        upward: false,
        size_percent: 100,
    };
    let mut app = design_app();
    let mut moved =
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k");
    moved.name_label_pos = placed;
    moved.value_label_pos = placed;
    let unmoved =
        Component::new(2, ComponentType::Resistor, Point::new(200, 0)).with_name_value("R2", "2k");
    app.state.model.schematic.document_mut_for_test().components =
        vec![moved.clone(), unmoved.clone()];
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);

    let symbol_context = schematic_symbol_context(&app.state);
    run_context_action(
        &mut app,
        ContextAction::ResetLabelPosition,
        Point::origin(),
        &symbol_context,
    );

    let parts = &app.state.model.schematic.document().components;
    assert_eq!(parts[0].name_label_pos, LabelPosition::Auto);
    assert_eq!(parts[0].value_label_pos, LabelPosition::Auto);
    assert_eq!(
        (parts[0].pos, parts[0].value.as_str()),
        (moved.pos, "1k"),
        "the part itself stays put"
    );
    assert_eq!(parts[1], unmoved);
    assert!(app.state.model.schematic.undo());
    assert_eq!(app.state.model.schematic.document().components[0], moved);
    assert!(
        !app.state.model.schematic.can_undo(),
        "the reset was one step"
    );
}

/// A wire's and a net label's right-click menus light their net, as
/// Design ▸ Highlight net does, and name its key. A part's pins are on
/// several nets, so its menu has no such row. Highlight net was a Design
/// menu row alone, with no key.
#[test]
fn the_highlight_net_row_lights_the_net_of_the_wire_or_label_clicked() {
    let mut app = design_app();
    let document = app.state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.wires.clear();
    document
        .wires
        .push(Wire::new(5, vec![Point::new(0, 0), Point::new(200, 0)]));
    document
        .wires
        .push(Wire::new(6, vec![Point::new(0, 100), Point::new(200, 100)]));
    document
        .net_labels
        .push(NetLabel::new(73, Point::new(200, 100), "out"));
    document.components.push(Component::new(
        41,
        ComponentType::Resistor,
        Point::new(400, 0),
    ));
    app.state.model.schematic.recalculate_runtime_state();
    app.state.model.schematic.bump_topology_version();
    app.state.sync_active_schematic_to_workspace();
    let symbol_context = schematic_symbol_context(&app.state);

    // The wire: the row is there, names the key, and lights the wire's net.
    let target = select_context_target(&mut app.state, Some(PointerTarget::Wire(5)));
    app.state.design.interaction.context_target = Some((target, (100, 0)));
    let view = context_menu_view(&app.state, &symbol_context, &Context::default())
        .expect("the menu is open");
    let highlight = view
        .entries
        .iter()
        .find_map(|entry| match entry {
            context_surface::ContextEntry::Command(command)
                if command.action == ContextAction::HighlightNet =>
            {
                Some(command.clone())
            }
            _ => None,
        })
        .expect("a wire's menu lights its net");
    assert_eq!(highlight.label, "Highlight net");
    assert_eq!(highlight.shortcut, "`");
    assert!(highlight.enabled, "{}", highlight.disabled_reason);
    run_context_action(
        &mut app,
        ContextAction::HighlightNet,
        Point::new(100, 0),
        &symbol_context,
    );
    let lit = |app: &crate::workbench::RSpiceApp, wire| {
        app.state
            .model
            .schematic
            .session
            .editor
            .net_highlight
            .is_wire_highlighted(wire)
    };
    assert!(lit(&app, 5) && !lit(&app, 6));

    // The label: its net is the other wire's.
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(73);
    app.state.design.interaction.context_target = Some((ContextTarget::Canvas, (200, 100)));
    assert!(
        menu_entries(&app.state, ContextTarget::Canvas)
            .iter()
            .any(|entry| matches!(entry, ContextEntry::Command(command)
                if command.action == ContextAction::HighlightNet))
    );
    run_context_action(
        &mut app,
        ContextAction::HighlightNet,
        Point::new(200, 100),
        &symbol_context,
    );
    assert!(lit(&app, 6) && !lit(&app, 5));

    // A part: no row.
    let target = select_context_target(&mut app.state, Some(PointerTarget::Component(41)));
    assert!(
        !menu_entries(&app.state, target)
            .iter()
            .any(|entry| matches!(entry, ContextEntry::Command(command)
                if command.action == ContextAction::HighlightNet))
    );
}

/// A pin end's right-click menu marks the pin as meant to stay open, shows
/// the row checked while it is, and the same row clears the mark. Only the
/// no-connect tool could mark a pin.
#[test]
fn a_pin_ends_no_connection_row_marks_the_pin_and_clears_it() {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = design_app();
    let document = app.state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.wires.clear();
    let part = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(400, 0));
    app.state.model.schematic.clear_undo_history();
    app.state.sync_active_schematic_to_workspace();
    let symbol_context = schematic_symbol_context(&app.state);
    let component = app.state.model.schematic.document().components[0].clone();
    let (pin, end) = symbol_context.named_terminal_points(&component)[0].clone();
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    app.state.design.interaction.context_target = Some((ContextTarget::Pin(part), (end.x, end.y)));

    // One frame of the open menu: where it drew the row, and whether the
    // row reads as checked.
    let frame = |app: &mut crate::workbench::RSpiceApp, events: Vec<egui::Event>| {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
                events,
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show(root, |ui| {
                    render_context_contents(ui, &mut app.state, &symbol_context, 27.0, false);
                });
            },
        );
        app.after_contribution();
        output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
            .find(|(_, node)| node.label() == Some("No connection"))
            .map(|(_, node)| {
                let bounds = node.bounds().expect("the row is drawn");
                (
                    pos2(
                        ((bounds.x0 + bounds.x1) * 0.5) as f32,
                        ((bounds.y0 + bounds.y1) * 0.5) as f32,
                    ),
                    node.toggled() == Some(egui::accesskit::Toggled::True),
                )
            })
            .expect("the pin's menu has a No connection row")
    };
    let click = |at| {
        [true, false].map(|pressed| {
            vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Modifiers::NONE,
                },
            ]
        })
    };
    let marked = |app: &crate::workbench::RSpiceApp| {
        app.state.model.schematic.document().components[0].is_no_connect(&pin)
    };

    let (row, checked) = frame(&mut app, Vec::new());
    assert!(!checked && !marked(&app), "the pin starts open to wiring");
    for events in click(row) {
        frame(&mut app, events);
    }
    assert!(marked(&app), "the row marked pin {pin} no-connect");
    let (row, checked) = frame(&mut app, Vec::new());
    assert!(checked, "the row reads as checked while the pin is marked");

    for events in click(row) {
        frame(&mut app, events);
    }
    assert!(!marked(&app), "the second use cleared the mark");
    assert!(!frame(&mut app, Vec::new()).1, "and the row is unchecked");
}
