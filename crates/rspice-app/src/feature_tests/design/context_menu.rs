//! The canvas context menu driven through a whole application.

use crate::workbench::app_state::AppState;
use crate::workbench::commands::answer_surface_requests;
use crate::workbench::state::Workspace;
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
        !app.command_key_enabled(DesignCommand::DescendHierarchy.key()),
        "the descent dialog lists schematic masters only"
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

fn context_menu_view(state: &AppState, ctx: &Context) -> Option<ContextMenuView> {
    rspice_app_design::schematic::view::context_menu::context_menu_view(&state.canvas_facts(), ctx)
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
    );
    answer_surface_requests(state);
}

fn open_operating_point(state: &mut AppState) {
    rspice_app_design::schematic::view::context_menu::open_operating_point(
        &mut state.canvas_parts(),
    );
    answer_surface_requests(state);
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
        ProjectSourceLanguage, ProjectSourceOwner, View, ViewType,
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
                ProjectSourceLanguage::VerilogA,
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
        let mut state = AppState::default();
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
        let symbol_context = schematic_symbol_context(&state);

        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, 27.0, true);
            });
        });

        let mut key_still_available = true;
        let _ = ctx.run_ui(context_key_input(key), |root| {
            egui::CentralPanel::default().show(root, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, 27.0, false);
                key_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(Modifiers::NONE, key));
            });
        });

        assert!(
            state.design.tabbed_property_dialog.open || state.design.object_properties.open,
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
    let mut state = AppState::default();
    let label = NetLabel::new(73, Point::new(40, 40), "afe_out");
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("net label · afe_out"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        label.pos,
        &symbol_context,
    );
    assert!(state.design.object_properties.open);
    assert!(!state.design.rename_selection.open);
    assert!(matches!(
        state.design.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::NetLabel(ref draft))
            if draft.original == label
    ));
}

#[test]
fn design_note_context_exposes_only_compatible_object_lifecycle_actions() {
    let mut state = AppState::default();
    let note = DesignNote::new(
        74,
        Point::new(40, 40),
        DesignNoteKind::ReviewNote,
        "Review bias path",
    )
    .unwrap();
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(note.clone());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_design_note(note.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    // A note turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &state).0);
    assert!(action_availability(ContextAction::Mirror, &state).0);
    let summary = selection_summary(&state, ContextTarget::Canvas);
    assert!(summary.contains("Review note"));
    assert!(summary.contains("Review bias path"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        note.pos,
        &symbol_context,
    );
    assert!(matches!(
        state.design.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DesignNote(ref draft))
            if draft.original == note
    ));
}

#[test]
fn documentation_shape_context_exposes_the_complete_non_electrical_lifecycle() {
    let mut state = AppState::default();
    let shape = DocumentationShape::new(
        75,
        DocumentationShapeGeometry::Callout {
            tip: Point::new(10, 10),
            elbow: Point::new(30, 20),
            box_corner: Point::new(80, 50),
        },
    )
    .unwrap();
    state
        .model
        .schematic
        .document_mut_for_test()
        .documentation_shapes
        .push(shape.clone());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_documentation_shape(shape.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    // A shape turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &state).0);
    assert!(action_availability(ContextAction::Mirror, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("Callout"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        Point::new(30, 20),
        &symbol_context,
    );
    assert!(matches!(
        state.design.object_properties.draft,
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

    let mut state = AppState::default();
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
    state
        .model
        .workspace
        .insert_schematic_editor(MASTER.to_owned(), master);
    let instance = state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(instance);

    assert!(
        !action_availability(ContextAction::UpdateInstanceInterface, &state).0,
        "a placement that still matches its master is not stale"
    );

    state
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

    assert!(action_availability(ContextAction::UpdateInstanceInterface, &state).0);

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::UpdateInstanceInterface,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert_eq!(
        state.model.schematic.document().components[0]
            .library_cell
            .as_ref()
            .expect("the instance stays bound")
            .terminal_order,
        ["ain"]
    );
    assert!(!action_availability(ContextAction::UpdateInstanceInterface, &state).0);
}

/// The replacement row is offered exactly where the command is, and opens
/// the same reviewed transaction. A row with its own answer would offer a
/// replacement the dialog would immediately refuse.
#[test]
fn the_replace_instance_row_is_offered_and_runs_only_for_one_replaceable_instance() {
    let mut state = AppState::default();
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "no selection is not one replaceable instance"
    );

    let first = state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(40, 0));
    let second = state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(120, 0));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    assert!(action_availability(ContextAction::ReplaceInstance, &state).0);
    assert_eq!(
        action_availability(ContextAction::ReplaceInstance, &state).0,
        replace_instance_available(&state.design.property_registry, &state.model),
        "the row must answer with the command's own predicate"
    );

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(second);
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "two selected instances are not one replaceable instance"
    );

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::ReplaceInstance,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert!(state.design.replace_instance.open);
    assert_eq!(state.design.replace_instance.source_component_id, first);
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
    let view = context_menu_view(state, ui.ctx()).expect("context target");
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
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(duplicate_schematic_selection_at(
        state.design.floating_copy_parts(),
        &mut state.model,
        &mut state.workbench,
        Point::new(60, 30)
    ));
    let symbol_context = schematic_symbol_context(&state);

    assert!(action_availability(ContextAction::Rotate, &state).0);
    execute_context_action(
        ContextAction::Rotate,
        &mut state,
        Point::new(60, 30),
        &symbol_context,
    );
    assert!(rspice_app_design::workbench::app::dialogs::selection::commands::floating::place_floating_paste(
        &mut state.design.floating_paste,
        &mut state.model,
        &mut state.workbench
    ));
    let quarter = rspice_design::schematic::rotation::Rotation::default().rotate_cw();
    let turned: Vec<_> = state
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
/// keys to finish the job (audit E-9). Each row is offered on every target
/// and runs its command: Cut takes the part onto the clipboard, Paste puts
/// it back where the menu was opened, and Mirror vertically flips the
/// selection top to bottom.
#[test]
fn cut_paste_and_mirror_vertically_rows_run_their_commands() {
    use rspice_app_design::schematic::view::selection_transform::SelectionTransform;

    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    state.model.schematic.document_mut_for_test().wires.clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    let symbol_context = schematic_symbol_context(&state);

    for target in [ContextTarget::Component(41), ContextTarget::Canvas] {
        let actions: Vec<ContextAction> = menu_entries(&state, target)
            .iter()
            .filter_map(|entry| match entry {
                ContextEntry::Command(command) => Some(command.action),
                ContextEntry::Separator => None,
            })
            .collect();
        for action in [
            ContextAction::Cut,
            ContextAction::Paste,
            ContextAction::MirrorVertical,
        ] {
            assert!(actions.contains(&action), "{target:?} lacks {action:?}");
        }
    }
    assert!(action_availability(ContextAction::Cut, &state).0);
    assert!(
        !action_availability(ContextAction::Paste, &state).0,
        "nothing has been copied yet"
    );

    execute_context_action(
        ContextAction::Cut,
        &mut state,
        Point::origin(),
        &symbol_context,
    );
    assert!(state.model.schematic.document().components.is_empty());
    assert_eq!(state.design.schematic_clipboard.components.len(), 1);
    assert!(action_availability(ContextAction::Paste, &state).0);

    let click = Point::new(200, 100);
    let offset = click - state.design.schematic_clipboard.origin;
    execute_context_action(ContextAction::Paste, &mut state, click, &symbol_context);
    assert!(rspice_app_design::workbench::app::dialogs::selection::commands::floating::place_floating_paste(
        &mut state.design.floating_paste,
        &mut state.model,
        &mut state.workbench
    ));
    let pasted = state.model.schematic.document().components.clone();
    assert_eq!(pasted.len(), 1);
    assert_eq!(pasted[0].pos, Point::new(20, 30) + offset);
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .has_component(pasted[0].id)
    );

    assert!(action_availability(ContextAction::MirrorVertical, &state).0);
    execute_context_action(
        ContextAction::MirrorVertical,
        &mut state,
        click,
        &symbol_context,
    );
    let mirrored = &state.model.schematic.document().components[0];
    assert_ne!(
        (mirrored.mirror_v, mirrored.mirror_h, mirrored.rotation),
        (pasted[0].mirror_v, pasted[0].mirror_h, pasted[0].rotation),
        "the part was not flipped"
    );
    assert_eq!(
        state.model.schematic.undo_description(),
        Some(SelectionTransform::MirrorVertical.description())
    );
}

/// The Arrange page offers Snap Selection to Grid, the command the Design
/// menu's Arrange ends with, under the same command id, and its row puts an
/// off-grid part back on the grid in one undo step. A phone has no menu bar,
/// so without the row a finger could not snap at all.
#[test]
fn the_arrange_page_snaps_the_selection_to_the_grid() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    state.model.schematic.document_mut_for_test().wires.clear();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(
            41,
            ComponentType::Resistor,
            Point::new(323, 367),
        ));
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    let symbol_context = schematic_symbol_context(&state);
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
        !action_availability(snap.action, &state).0,
        "nothing is selected to snap"
    );

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(action_availability(snap.action, &state).0);
    execute_context_action(snap.action, &mut state, Point::origin(), &symbol_context);

    assert_eq!(state.model.schematic.document().grid_size, 10);
    assert_eq!(
        state.model.schematic.document().components[0].pos,
        Point::new(320, 370)
    );
    assert_ne!(
        state.model.schematic.document().components[0].pos,
        Point::new(323, 367)
    );
    assert_eq!(
        state.model.schematic.undo_description(),
        Some("snap selection to grid")
    );
}

/// Under "Selected net across hierarchy" an Alt+click net highlight is
/// carried into the instance a user descends into. Picking that instance
/// cleared it first: the click before the descend key or a double-click,
/// and the context menu that offers Descend, so the child lit nothing and
/// neither did the parent on the way back up. Picking a descendable
/// instance now leaves it lit; picking an ordinary part still clears it.
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
    assert_eq!(lit(&state), None, "an ordinary part still clears it");
}
