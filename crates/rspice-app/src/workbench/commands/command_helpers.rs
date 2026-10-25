//! Command-dispatch helpers that apply concrete workbench state transitions.

use super::*;

pub(super) fn open_command_palette(state: &mut AppState) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::CommandPalette);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.prompt(format!("The Command Palette could not be opened: {error}"));
        return;
    }
    state.workbench.command_palette.open_routed();
}

pub(super) fn open_help_center(state: &mut AppState, page: crate::workbench::app::HelpCenterPage) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::HelpCenter);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.prompt(format!("The Help Center could not be opened: {error}"));
        return;
    }
    state.shell.help_center.open_routed(page);
}

/// Whether the netlist document — not merely the workspace that hosts it — is
/// the thing on screen.
pub(super) fn netlist_page_is_visible(state: &crate::workbench::AppState) -> bool {
    state.workbench.workspace == Workspace::Netlist
        && state.code.code_workspace.page
            == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
}

/// Whether a code document is the thing on screen: one of the Netlist
/// workspace's pages, or a Verilog-A cell view the Design workspace is
/// showing.
pub(super) fn code_document_is_visible(state: &crate::workbench::AppState) -> bool {
    state.workbench.workspace == Workspace::Netlist
        || crate::workbench::documents::code_workspace::design_hosts_veriloga(
            state.workbench.workspace,
            &state.model.workspace,
        )
}

pub(super) fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
}

pub(super) const fn workspace_available(project_open: bool, workspace: Workspace) -> bool {
    project_open || matches!(workspace, Workspace::Project)
}

pub(super) fn active_symbol_editor(app: &RSpiceApp) -> bool {
    matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) && app.state.model.workspace.content.active_view_type() == crate::state::ViewType::Symbol
}

/// Mockup-owned `G` command: cycle only the canvas presentation. Snap pitch
/// and target classes remain independent in Grid, snap and wire routing.
pub(super) fn cycle_canvas_grid(app: &mut RSpiceApp) {
    if active_symbol_editor(app) {
        let enabled = !app.state.design.symbol.editor.show_grid;
        app.state.design.symbol.editor.show_grid = enabled;
        return;
    }

    app.state.design.cycle_grid_style();
}

pub(in crate::workbench) fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.model.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

pub(super) fn schematic_selection_has_live_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    let selection = &schematic.session.editor.selection;
    schematic
        .document()
        .components
        .iter()
        .any(|component| selection.has_component(component.id))
        || schematic
            .document()
            .wires
            .iter()
            .any(|wire| selection.has_wire(wire.id))
        || schematic
            .document()
            .junctions
            .iter()
            .any(|junction| selection.has_junction(junction.pos))
        || schematic
            .document()
            .net_labels
            .iter()
            .any(|label| selection.has_net_label(label.id))
        || schematic
            .document()
            .buses
            .iter()
            .any(|bus| selection.has_bus(bus.id))
        || schematic
            .document()
            .bus_taps
            .iter()
            .any(|tap| selection.has_bus_tap(tap.id))
        || schematic
            .document()
            .design_notes
            .iter()
            .any(|note| selection.has_design_note(note.id))
        || schematic
            .document()
            .documentation_shapes
            .iter()
            .any(|shape| selection.has_documentation_shape(shape.id))
        || schematic
            .document()
            .probes
            .iter()
            .any(|probe| selection.has_probe(probe.id))
}

/// Return whether Delete can resolve at least one selected identity to a live
/// complete object. Segment and vertex handles are edit subobjects rather than
/// independently persisted conductors, so the destructive review promotes a
/// live handle to its owning wire before commit.
pub(super) fn schematic_selection_has_deletable_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    schematic_selection_has_live_object(schematic)
        || schematic.document().wires.iter().any(|wire| {
            schematic
                .session
                .editor
                .selection
                .wire_segments
                .iter()
                .any(|selected| {
                    selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
                })
                || schematic
                    .session
                    .editor
                    .selection
                    .wire_vertices
                    .iter()
                    .any(|selected| {
                        selected.wire_id == wire.id && selected.vertex_index < wire.vertex_count()
                    })
        })
}

pub(super) fn schematic_selection_has_duplicable_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    let selection = &schematic.session.editor.selection;
    selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && (schematic
            .document()
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || schematic
                .document()
                .wires
                .iter()
                .any(|wire| selection.has_wire(wire.id))
            || schematic
                .document()
                .net_labels
                .iter()
                .any(|label| selection.has_net_label(label.id))
            || schematic
                .document()
                .buses
                .iter()
                .any(|bus| selection.has_bus(bus.id))
            || schematic
                .document()
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id))
            || schematic
                .document()
                .design_notes
                .iter()
                .any(|note| selection.has_design_note(note.id))
            || schematic
                .document()
                .documentation_shapes
                .iter()
                .any(|shape| selection.has_documentation_shape(shape.id))
            || schematic
                .document()
                .probes
                .iter()
                .any(|probe| selection.has_probe(probe.id)))
}

pub(super) fn set_tool(app: &mut RSpiceApp, tool: Tool) {
    activate_workspace(app, Workspace::Design);
    if app.state.design.move_selection.armed && tool != Tool::MoveSelection {
        app.state.design.move_selection.close();
    }
    if app.state.design.stretch_selection.armed && tool != Tool::StretchSelection {
        app.state.design.stretch_selection.close();
    }
    if app.state.design.array_selection.armed && tool != Tool::ArraySelection {
        app.state.design.array_selection.close();
    }
    app.state.model.schematic.arm_tool(tool);
}

pub(super) fn open_recent_projects(state: &mut AppState) {
    state.open_project_launcher();
    state.project.project_launcher_filter = ProjectLauncherFilter::Recent;
}

pub(super) const fn reset_active_view_available(_workspace: Workspace) -> bool {
    true
}

pub(crate) fn reset_active_view(app: &mut RSpiceApp) {
    app.state.workbench.clear_navigator_filter();
    match app.state.workbench.workspace {
        Workspace::Project => {
            app.state.workbench.command_query.clear();
        }
        Workspace::Design => {
            if app.state.model.workspace.content.active_view_type()
                == crate::state::ViewType::Symbol
            {
                app.state.design.symbol.editor.zoom = 1.0;
                app.state.design.symbol.editor.pan = (0.0, 0.0);
                app.state.design.symbol.editor.needs_fit = true;
                app.state.design.symbol.editor.clear_selection();
                app.state.design.symbol.editor.marquee_start = None;
                app.state.design.symbol.editor.marquee_current = None;
            } else {
                app.state.model.schematic.session.editor.zoom = 1.0;
                app.state.model.schematic.session.editor.pan = (0.0, 0.0);
                app.state.model.schematic.session.editor.needs_fit = true;
                app.state.model.schematic.session.editor.center_request = None;
                app.state.model.schematic.session.editor.selection.clear();
                app.state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection_rect
                    .cancel();
                app.state
                    .model
                    .schematic
                    .session
                    .editor
                    .net_highlight
                    .clear();
            }
        }
        Workspace::Simulate => {
            // The studio's view state is what narrows its two registries.
            // Everything else on these pages is the plan itself, which a
            // view reset has no business touching.
            app.state.simulate.saved_output_filter.clear();
            app.state.simulate.specification_filter.clear();
            app.state.simulate.specification_evidence_filter = SpecificationEvidenceFilter::All;
        }
        // Which definition the library is reading, and the drafts open behind
        // it, are the whole of this workspace's view state; the definitions
        // themselves are the project and a view reset never touches them.
        Workspace::Stimulus => {
            app.state.design.selected_stimulus_definition = None;
            app.state.design.stimulus_editor.clear();
            app.state.design.stimulus_browser.clear();
        }
        Workspace::Results => {
            app.route_command_key(ids::RESET_ACTIVE_VIEW);
        }
        Workspace::Verify => {
            app.state.simulate.selected_specification = None;
        }
        Workspace::Models => {
            app.state.models.selected_model = None;
        }
        Workspace::Netlist => {
            app.state.code.netlist.cursor_line = 0;
            app.state.code.netlist.completion_open = false;
            app.state.code.netlist.completion_index = 0;
        }
    }
}

pub(super) fn open_new_cell_dialog(app: &mut RSpiceApp) {
    let selected = app
        .state
        .model
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            app.state
                .model
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target_library = selected.or_else(|| {
        app.state
            .model
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });
    let library_revision = app.state.model.library_manager.revision();

    let dialogs = &mut app.state.project;
    dialogs.new_cell_library = target_library.unwrap_or_default();
    dialogs.new_cell_name.clear();
    dialogs.new_cell_description.clear();
    dialogs.new_cell_create_schematic = true;
    dialogs.new_cell_create_symbol = false;
    dialogs.new_cell_create_testbench = false;
    dialogs.new_cell_create_veriloga = false;
    dialogs.new_cell_error = None;
    dialogs.new_cell_library_revision = library_revision;
    dialogs.new_cell_dialog = true;
}

pub(in crate::workbench) fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(
        &mut app.state,
        action,
        app.file_workflow_io.as_ref(),
        app.export_workflow_io.as_ref(),
    );
}
