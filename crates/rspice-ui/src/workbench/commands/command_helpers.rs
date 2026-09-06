//! Command-dispatch helpers that apply concrete workbench state transitions.

use super::*;

pub(super) fn open_command_palette(state: &mut AppState) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::CommandPalette);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "The Command Palette could not be opened: {error}"
        )));
        return;
    }
    state.dialogs.command_palette.open_routed();
}

pub(super) fn open_help_center(state: &mut AppState, page: crate::workbench::app::HelpCenterPage) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::HelpCenter);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "The Help Center could not be opened: {error}"
        )));
        return;
    }
    state.dialogs.help_center.open_routed(page);
}

/// Whether the netlist document — not merely the workspace that hosts it — is
/// the thing on screen.
pub(super) fn netlist_page_is_visible(state: &crate::workbench::AppState) -> bool {
    state.workbench.workspace == Workspace::Netlist
        && state.ui.code_workspace.page
            == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
}

pub(super) fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
}

pub(super) const fn workspace_available(project_open: bool, workspace: Workspace) -> bool {
    project_open || matches!(workspace, Workspace::Project)
}

/// Open a Visualization Studio tool, and say so when that means leaving the
/// result view behind.
///
/// These tools operate across visualization documents, so the Studio is
/// genuinely their home — but a menu item that silently swaps the surface
/// under the reader is a surprise. Announcing the move in the console leaves
/// a record of what happened and why the plot went away.
pub(super) fn open_studio_tool(app: &mut RSpiceApp, tool: &str) {
    if app.state.workbench.current_route().surface_id() != super::super::SurfaceId::Results {
        return;
    }
    crate::workbench::documents::visualization_studio::open(app);
    if app.state.workbench.current_route().surface_id()
        == super::super::SurfaceId::VisualizationStudio
    {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                "{tool} opened in Visualization Studio, which owns tools that span visualization documents. The result view is one Back away."
            )));
    }
}

pub(super) fn active_symbol_editor(app: &RSpiceApp) -> bool {
    matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) && app.state.workspace.active_view_type() == crate::state::ViewType::Symbol
}

/// Mockup-owned `G` command: cycle only the canvas presentation. Snap pitch
/// and target classes remain independent in Grid, snap and wire routing.
pub(super) fn cycle_canvas_grid(app: &mut RSpiceApp) {
    if active_symbol_editor(app) {
        let enabled = !app.state.ui.symbol.show_grid;
        app.state.ui.symbol.show_grid = enabled;
        return;
    }

    app.state.ui.cycle_grid_style();
}

pub(super) fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

pub(super) fn schematic_selection_has_live_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    let selection = &schematic.selection;
    schematic
        .components
        .iter()
        .any(|component| selection.has_component(component.id))
        || schematic
            .wires
            .iter()
            .any(|wire| selection.has_wire(wire.id))
        || schematic
            .junctions
            .iter()
            .any(|junction| selection.has_junction(junction.pos))
        || schematic
            .net_labels
            .iter()
            .any(|label| selection.has_net_label(label.id))
        || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
        || schematic
            .bus_taps
            .iter()
            .any(|tap| selection.has_bus_tap(tap.id))
        || schematic
            .design_notes
            .iter()
            .any(|note| selection.has_design_note(note.id))
        || schematic
            .documentation_shapes
            .iter()
            .any(|shape| selection.has_documentation_shape(shape.id))
        || schematic
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
        || schematic.wires.iter().any(|wire| {
            schematic.selection.wire_segments.iter().any(|selected| {
                selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
            }) || schematic.selection.wire_vertices.iter().any(|selected| {
                selected.wire_id == wire.id && selected.vertex_index < wire.vertex_count()
            })
        })
}

pub(super) fn schematic_selection_has_duplicable_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    let selection = &schematic.selection;
    selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && (schematic
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || schematic
                .wires
                .iter()
                .any(|wire| selection.has_wire(wire.id))
            || schematic
                .net_labels
                .iter()
                .any(|label| selection.has_net_label(label.id))
            || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
            || schematic
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id))
            || schematic
                .design_notes
                .iter()
                .any(|note| selection.has_design_note(note.id))
            || schematic
                .documentation_shapes
                .iter()
                .any(|shape| selection.has_documentation_shape(shape.id))
            || schematic
                .probes
                .iter()
                .any(|probe| selection.has_probe(probe.id)))
}

pub(super) fn set_tool(app: &mut RSpiceApp, tool: Tool) {
    activate_workspace(app, Workspace::Design);
    if app.state.dialogs.move_selection.armed && tool != Tool::MoveSelection {
        app.state.dialogs.move_selection.close();
    }
    if app.state.dialogs.stretch_selection.armed && tool != Tool::StretchSelection {
        app.state.dialogs.stretch_selection.close();
    }
    if app.state.dialogs.array_selection.armed && tool != Tool::ArraySelection {
        app.state.dialogs.array_selection.close();
    }
    app.state.schematic.arm_tool(tool);
}

pub(super) fn open_recent_projects(workbench: &mut WorkbenchState) {
    workbench.open_project_launcher();
    workbench.project_launcher_filter = ProjectLauncherFilter::Recent;
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
            if app.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
                app.state.ui.symbol.zoom = 1.0;
                app.state.ui.symbol.pan = (0.0, 0.0);
                app.state.ui.symbol.needs_fit = true;
                app.state.ui.symbol.clear_selection();
                app.state.ui.symbol.marquee_start = None;
                app.state.ui.symbol.marquee_current = None;
            } else {
                app.state.schematic.zoom = 1.0;
                app.state.schematic.pan = (0.0, 0.0);
                app.state.schematic.needs_fit = true;
                app.state.schematic.center_request = None;
                app.state.schematic.selection.clear();
                app.state.schematic.selection_rect.cancel();
                app.state.schematic.net_highlight.clear();
            }
        }
        Workspace::Simulate => {
            // The studio's view state is what narrows its two registries.
            // Everything else on these pages is the plan itself, which a
            // view reset has no business touching.
            app.state.workbench.saved_output_filter.clear();
            app.state.workbench.specification_filter.clear();
            app.state.workbench.specification_evidence_filter = SpecificationEvidenceFilter::All;
        }
        // Which definition the library is reading is the whole of this
        // workspace's view state; the definitions themselves are the project.
        Workspace::Stimulus => {
            app.state.workbench.selected_stimulus_definition = None;
        }
        Workspace::Results => {
            let viewer = app.state.ui.results.viewer;
            app.state
                .ui
                .results
                .views
                .retain(|(candidate, _, _), _| *candidate != viewer);
            app.state.ui.results.clear_cursors();
            app.state.ui.results.rf_pin.remove(&viewer);
            if viewer == crate::workbench::ResultViewer::Waves {
                app.state.ui.results.hidden_strips.clear();
                app.state.ui.results.maximized_strip = None;
            }
            app.state
                .workbench
                .visualization_studio
                .reset_transient_view();
        }
        Workspace::Verify => {
            app.state.workbench.selected_specification = None;
        }
        Workspace::Models => {
            app.state.workbench.selected_model = None;
        }
        Workspace::Netlist => {
            app.state.ui.netlist.cursor_line = 0;
            app.state.ui.netlist.completion_open = false;
            app.state.ui.netlist.completion_index = 0;
        }
    }
}

pub(super) fn open_new_cell_dialog(app: &mut RSpiceApp) {
    let selected = app
        .state
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            app.state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target_library = selected.or_else(|| {
        app.state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });
    let library_revision = app.state.library_manager.revision();

    let dialogs = &mut app.state.dialogs;
    dialogs.new_cell_library = target_library.unwrap_or_default();
    dialogs.new_cell_name.clear();
    dialogs.new_cell_description.clear();
    dialogs.new_cell_create_schematic = true;
    dialogs.new_cell_create_symbol = false;
    dialogs.new_cell_create_testbench = false;
    dialogs.new_cell_error = None;
    dialogs.new_cell_library_revision = library_revision;
    dialogs.new_cell_dialog = true;
}

pub(super) fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(
        &mut app.state,
        action,
        app.file_workflow_io.as_ref(),
        app.export_workflow_io.as_ref(),
    );
}
