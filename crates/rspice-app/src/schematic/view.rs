//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use crate::workbench::app::symbol_context_revision;

use egui::Ui;

use crate::workbench::app_state::AppState;

use super::symbols::SymbolLibrary;

mod array_interaction;
mod bus_interaction;
mod context_menu;
pub(crate) mod drawing_sheet;
mod interaction;
mod keyboard_navigation;
mod mobile_controls;
mod move_interaction;
mod preview;
mod requests;
pub(crate) use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;
use rspice_schematic_editor::view::symbol_context::SelectionWindow;
mod scene;
mod selection_drag;
pub(crate) mod selection_layout;
pub(crate) mod sheet_visibility;
mod shelf_drag;
mod snap_resolution;
mod stretch_interaction;
pub(crate) mod violations;

use rspice_schematic_editor::view::{
    canvas, coordinates,
    cross_probe::{self, FailureSiteSelection, LocateSignalError},
    drawing, navigation, viewport,
};

use self::coordinates::viewport_from_camera;
use self::drawing_sheet::resolve_active_drawing_sheet;
use self::interaction::handle_tool_interactions;
use self::keyboard_navigation::handle_keyboard_object_navigation;
use self::navigation::handle_viewport_navigation;
use self::preview::{draw_interaction_previews, draw_shelf_drag_preview};
use self::scene::draw_scene;
use self::shelf_drag::{
    ShelfDropOutcome, can_accept_shelf_drop, commit_shelf_drop, handle_placement_transform_keys,
};

pub(crate) use self::interaction::{
    ensure_probe_visible_with_feedback, ensure_retained_probe_visible_with_feedback,
    toggle_probe_with_feedback,
};
pub(crate) use self::mobile_controls::show as show_mobile_canvas_controls;
pub(crate) use self::sheet_visibility::retain_selection_on_active_sheet;
pub(crate) use self::shelf_drag::{
    SchematicShelfDragPayload, handle_pre_render_placement_transform,
};

/// Whether the typed component-shelf payload is currently over the schematic
/// canvas rectangle captured during the previous settled frame.
///
/// Application shortcut resolution runs before this frame's canvas response
/// exists, so it deliberately uses egui's retained response for the stable
/// canvas id. This closes the single-frame ordering gap for R/M drag-ghost
/// transforms without accepting a drop outside the canvas.
pub(crate) fn shelf_drag_over_schematic_canvas(ctx: &egui::Context) -> bool {
    egui::DragAndDrop::has_payload_of_type::<SchematicShelfDragPayload>(ctx)
        && canvas::contains_pointer(ctx)
}

pub(super) fn schematic_design_view(
    state: &AppState,
) -> rspice_schematic_editor::view::design_view::DesignView<'_> {
    let key = state.workspace.content.active_schematic_reference().key();
    rspice_schematic_editor::view::design_view::DesignView {
        document: state.schematic.document(),
        canvas_cache: state.schematic.canvas_cache(),
        sheet_catalog: state
            .workspace
            .content
            .design_management
            .sheet_catalog(&key),
        review_markers: state.ui.schematic_visibility.review_markers,
    }
}

/// Compose the editor's resolved symbols from the current app-owned sources.
pub(crate) fn schematic_symbol_context(state: &AppState) -> SchematicSymbolContext {
    let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
        state.library_manager.catalog(),
        &state.workspace.content.schematic_buffers,
    );
    SchematicSymbolContext::new(
        state.schematic.document(),
        state
            .schematic
            .session
            .editor
            .pending_library_cell
            .as_ref()
            .map(|pending| &pending.binding),
        &resolver,
        symbol_context_revision(state),
    )
}

fn refresh_symbol_context_after_interactions(
    state: &AppState,
    symbol_context: &mut SchematicSymbolContext,
    before_topology_version: u64,
) -> bool {
    if state.schematic.topology_version() == before_topology_version {
        return false;
    }
    *symbol_context = schematic_symbol_context(state);
    true
}

fn canvas_accessibility_view(state: &AppState) -> canvas::CanvasAccessibilityView<'_> {
    canvas::CanvasAccessibilityView {
        design: schematic_design_view(state),
        editor: &state.schematic.session.editor,
        keyboard_focus: state.dialogs.interaction.schematic_keyboard_focus,
        filter: state.ui.schematic_selection_filter,
        traversal_enabled: state
            .ui
            .preferences
            .toggle(crate::workbench::TogglePreference::CanvasKeyboardNavigation),
    }
}

fn schematic_accessibility_shortcuts(
    state: &AppState,
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::workbench::commands::vocabulary::Command;
    crate::workbench::app_state::accessibility_shortcut_summary(
        state.ui.preferences.shortcuts(),
        platform,
        operating_system,
        &[
            Command::SelectTool,
            Command::PlaceWire,
            Command::PlaceBus,
            Command::PlaceBusTap,
            Command::PlaceJunction,
            Command::PlaceProbe,
            Command::PlacePin,
            Command::PlaceText,
            Command::PlaceShape,
            Command::MoveSelection,
            Command::StretchSelection,
            Command::ArraySelection,
            Command::ZoomFit,
            Command::Cancel,
        ],
    )
}

/// Select a result signal only through the map belonging to the active drawing.
pub(crate) fn select_signal_conductor(
    state: &mut AppState,
    signal: &str,
) -> Result<String, LocateSignalError> {
    let current = result_mapping_is_current(state);
    let occurrence = state.workspace.content.occurrence_path();
    let (document, editor) = state.schematic.document_and_editor();
    let view = cross_probe::CrossProbeView {
        document,
        occurrence: &occurrence,
        net_to_points: current.then_some(&state.simulation.cross_probe.net_to_points),
    };
    cross_probe::select_signal_conductor(&view, editor, signal)
}

/// Query the same host-authorized geometry that selection uses.
pub(crate) fn drawn_failure_site_count(
    state: &AppState,
    nets: &[String],
    devices: &[String],
) -> Result<usize, LocateSignalError> {
    let occurrence = state.workspace.content.occurrence_path();
    let view = cross_probe::CrossProbeView {
        document: state.schematic.document(),
        occurrence: &occurrence,
        net_to_points: result_mapping_is_current(state)
            .then_some(&state.simulation.cross_probe.net_to_points),
    };
    cross_probe::drawn_failure_site_count(&view, nets, devices)
}

/// Whether the retained cross-probe map belongs to the open cell as it is
/// drawn right now.
fn result_mapping_is_current(state: &AppState) -> bool {
    state.simulation.cross_probe.is_current_for(
        &state.workspace.content.active_view,
        state.schematic.topology_version(),
    )
}

/// Mark a failed run's objects while retaining app ownership of map freshness.
pub(crate) fn select_failure_sites(
    state: &mut AppState,
    nets: &[String],
    devices: &[String],
) -> Result<FailureSiteSelection, LocateSignalError> {
    let current = result_mapping_is_current(state);
    let occurrence = state.workspace.content.occurrence_path();
    let (document, editor) = state.schematic.document_and_editor();
    let view = cross_probe::CrossProbeView {
        document,
        occurrence: &occurrence,
        net_to_points: current.then_some(&state.simulation.cross_probe.net_to_points),
    };
    cross_probe::select_failure_sites(&view, editor, nets, devices)
}

/// Render the schematic view (central canvas)
pub fn render_schematic_view(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    let available = ui.available_rect_before_wrap();
    let mut symbol_context = schematic_symbol_context(state);
    let drawing_sheet = resolve_active_drawing_sheet(state);

    if state.schematic.session.editor.needs_drawing_sheet_fit {
        state.schematic.session.editor.needs_drawing_sheet_fit = false;
        state.schematic.session.editor.needs_fit = false;
        state.schematic.zoom_to_fit_world_rect(
            drawing_sheet.geometry.paper.as_tuple(),
            available.width() as f64,
            available.height() as f64,
            drawing_sheet::FIT_SCREEN_INSET,
        );
    } else if state.schematic.session.editor.needs_fit {
        state.schematic.session.editor.needs_fit = false;
        let bounds = symbol_context.content_bounds(state.schematic.document());
        state.schematic.zoom_to_fit_bounds(
            bounds,
            available.width() as f64,
            available.height() as f64,
        );
    }
    if let Some(target) = state.schematic.session.editor.center_request.take() {
        state
            .schematic
            .center_view_on(target, available.width() as f64, available.height() as f64);
    }

    let response = canvas::interact(ui, available, || {
        state.application_modal_open() || state.workbench.drawer.is_some()
    });
    if response.clicked() || response.secondary_clicked() || response.drag_started() {
        state.dialogs.interaction.schematic_keyboard_focus = None;
    }
    ui.advance_cursor_after_rect(available);
    let painter = ui.painter_at(available);

    // Input first, painting second. Pan/zoom and tool edits apply BEFORE
    // the camera is built and the scene is painted — the old order drew
    // last frame's state, so the canvas trailed the cursor by a full
    // frame during pans and drags.
    handle_viewport_navigation(
        ui,
        &response,
        available,
        &mut state.schematic.session.editor.zoom,
        &mut state.schematic.session.editor.pan,
    );
    let viewport = viewport_from_camera(
        state.schematic.session.editor.pan,
        state.schematic.session.editor.zoom,
        available,
        ui.ctx().pixels_per_point(),
    );
    let shelf_drag = response.dnd_hover_payload::<SchematicShelfDragPayload>();
    let shelf_drag_position = shelf_drag
        .as_ref()
        .and_then(|_| ui.ctx().pointer_hover_pos());
    let shelf_drag_over_canvas = shelf_drag.is_some() && shelf_drag_position.is_some();
    handle_placement_transform_keys(&response, state, shelf_drag_over_canvas);
    if let Some(payload) = shelf_drag.as_deref() {
        ui.ctx()
            .set_cursor_icon(if can_accept_shelf_drop(state, payload) {
                egui::CursorIcon::Copy
            } else {
                egui::CursorIcon::NoDrop
            });
    }
    let dropped_payload = response.dnd_release_payload::<SchematicShelfDragPayload>();
    let before_interactions_topology = state.schematic.topology_version();
    // Keep the pre-interaction route state available to the context-menu
    // layer for diagnostics. Secondary click is exclusively a context-menu
    // gesture; routes commit with Enter or primary double-click.
    let routing_was_active = state.schematic.session.editor.wire_drawing.active
        || state.schematic.session.editor.bus_drawing.active
        || !state
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .points
            .is_empty();
    if let (Some(payload), Some(position)) = (dropped_payload.as_deref(), shelf_drag_position) {
        let drop_position =
            snap_resolution::resolve_grid_pointer(state, &viewport, position).snapped_position;
        match commit_shelf_drop(state, payload, drop_position) {
            ShelfDropOutcome::Placed => {
                state.ui.toasts.success(
                    ui.ctx(),
                    "Component placed",
                    format!(
                        "{} was placed at ({}, {}).",
                        payload.component_type().display_name(),
                        drop_position.x,
                        drop_position.y
                    ),
                );
            }
            ShelfDropOutcome::ReadOnly => {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Drop not permitted",
                    "The active schematic is read-only; no component was placed.",
                );
            }
            ShelfDropOutcome::RequiresConfiguration => {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Create pins first",
                    "A pin needs a name. Use Create pins (Shift+P).",
                );
            }
        }
    } else {
        handle_tool_interactions(ui, &response, state, &viewport, &symbol_context);
    }
    refresh_symbol_context_after_interactions(
        state,
        &mut symbol_context,
        before_interactions_topology,
    );
    let before_context_menu_topology = state.schematic.topology_version();
    context_menu::handle_context_menu(
        &response,
        state,
        &viewport,
        routing_was_active,
        &symbol_context,
    );
    handle_keyboard_object_navigation(&response, state, &symbol_context);
    refresh_symbol_context_after_interactions(
        state,
        &mut symbol_context,
        before_context_menu_topology,
    );

    // Refresh the frame-coherent canvas cache (culling bounds + hover
    // hit-test index) after interactions may have edited topology.
    state.schematic.ensure_canvas_cache();

    draw_scene(
        &painter,
        available,
        &viewport,
        state,
        symbol_library,
        &symbol_context,
        &drawing_sheet,
    );
    draw_interaction_previews(
        &painter,
        &response,
        state,
        &viewport,
        &symbol_context,
        symbol_library,
    );
    if dropped_payload.is_none()
        && let (Some(payload), Some(position)) = (shelf_drag.as_deref(), shelf_drag_position)
        && can_accept_shelf_drop(state, payload)
    {
        draw_shelf_drag_preview(
            &painter,
            state,
            &viewport,
            payload,
            position,
            symbol_library,
        );
    }
    // Report the cursor position in grid units; the workbench status bar shows it.
    let to_grid_units = |pos: egui::Pos2, state: &AppState| {
        let grid = f64::from(state.schematic.document().grid_size.max(1));
        let x = ((f64::from(pos.x - available.min.x)) - state.schematic.session.editor.pan.0)
            / state.schematic.session.editor.zoom
            / grid;
        let y = ((f64::from(pos.y - available.min.y)) - state.schematic.session.editor.pan.1)
            / state.schematic.session.editor.zoom
            / grid;
        (x, y)
    };
    state.ui.canvas_hover = shelf_drag_position
        .or_else(|| response.hover_pos())
        .map(|cursor| to_grid_units(cursor, state));
    state.ui.canvas_view_center = Some(to_grid_units(available.center(), state));

    let shortcut_platform = crate::workbench::app_state::runtime_command_platform(ui.ctx());
    let operating_system = ui.ctx().os();
    let shortcuts = schematic_accessibility_shortcuts(state, shortcut_platform, operating_system);
    canvas::finish(
        ui,
        &response,
        available,
        &canvas_accessibility_view(state),
        &shortcuts,
    );
    crate::workbench::app_state::report_engineering_canvas_focus(
        &response,
        state.workspace.content.active_view_type(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Component, ComponentType, Library, LibraryCellInstance, Point, PortDirection,
        PortSpec, SchematicState, SymbolDocument, SymbolPin, View, ViewType,
    };

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn state_with_probed_wire() -> AppState {
        let mut state = AppState::default();
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::new(1, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.content.active_view.clone(),
            std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
            std::collections::HashMap::new(),
            state.schematic.topology_version(),
        );
        state
    }

    #[test]
    fn a_result_from_a_different_drawing_never_selects_stale_geometry() {
        let mut state = state_with_probed_wire();
        // Any structural edit invalidates the retained point map.
        state.schematic.bump_topology_version();

        let error = select_signal_conductor(&mut state, "V(out)")
            .expect_err("the map is no longer current");

        assert_eq!(error, LocateSignalError::NoCurrentMap);
        assert!(state.schematic.session.editor.selection.is_empty());
    }

    #[test]
    fn a_scoped_trace_name_locates_the_conductor_by_its_leaf() {
        let mut state = AppState::default();
        state.workspace.descend_into(
            "X1".to_owned(),
            crate::state::CellViewRef::new("user", "amp", "schematic"),
            ViewType::Schematic,
        );
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::new(1, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.content.active_view.clone(),
            std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
            std::collections::HashMap::new(),
            state.schematic.topology_version(),
        );

        let net =
            select_signal_conductor(&mut state, "V(x1.out)").expect("the leaf is on this sheet");

        assert_eq!(net, "OUT");
        assert!(state.schematic.session.editor.selection.wires.contains(&1));
    }

    #[test]
    fn a_signal_read_in_another_instance_never_selects_a_same_named_conductor() {
        let mut state = state_with_probed_wire();

        let error = select_signal_conductor(&mut state, "V(x1.out)")
            .expect_err("this tab is editing the design root");

        assert_eq!(error, LocateSignalError::OtherOccurrence("/x1".to_owned()));
        assert!(error.message("V(x1.out)").contains("/x1"));
        assert!(state.schematic.session.editor.selection.is_empty());
    }

    #[test]
    fn second_refresh_resolves_cell_added_after_initial_interaction_refresh() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![SymbolPin::new(
                "OUT",
                PortDirection::Out,
                Some(Point::new(40, 0)),
            )],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let mut context = schematic_symbol_context(&state);
        let before_interactions_topology = state.schematic.topology_version();
        assert!(!refresh_symbol_context_after_interactions(
            &state,
            &mut context,
            before_interactions_topology,
        ));

        // The context-menu pass runs after the tool-interaction refresh. A
        // duplicate/place action can therefore mutate topology between the
        // first refresh and painting.
        let before_context_menu_topology = state.schematic.topology_version();
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[port("OUT", PortDirection::Out)]);
        let id = state
            .schematic
            .add_library_cell_component(Point::new(100, 50), binding);

        let refreshed = refresh_symbol_context_after_interactions(
            &state,
            &mut context,
            before_context_menu_topology,
        );
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("component placed");

        assert!(refreshed);
        assert!(context.resolved_symbol(component).is_some());
        let mut generated_preview = component.clone();
        generated_preview.id = u64::MAX;
        assert!(
            context.resolved_symbol(&generated_preview).is_some(),
            "generated array members resolve authored symbols by immutable binding"
        );
    }

    #[test]
    fn symbol_context_does_not_accept_an_interface_generated_substitute() {
        let mut state = AppState::default();
        let mut binding = LibraryCellInstance::new("missing", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        let id = state
            .schematic
            .add_library_cell_component(Point::origin(), binding);

        let context = schematic_symbol_context(&state);
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("component placed");

        assert!(context.resolved_symbol(component).is_none());
    }

    #[test]
    fn symbol_context_revision_tracks_library_and_schematic_interface_sources() {
        let mut state = AppState::default();
        let baseline = symbol_context_revision(&state);

        state.library_manager.add_library(Library::new("work"));
        let library_changed = symbol_context_revision(&state);
        assert_ne!(library_changed, baseline);

        state
            .workspace
            .insert_schematic_editor("work/amp/schematic".to_owned(), SchematicState::default());
        state
            .workspace
            .schematic_editor_mut("work/amp/schematic")
            .unwrap()
            .editor
            .add_component(ComponentType::Port, Point::origin());
        assert_ne!(symbol_context_revision(&state), library_changed);
    }

    #[test]
    fn accessibility_description_summarizes_scene_and_tool_without_selection_churn() {
        use crate::state::{Junction, NetLabel, Wire};

        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(2, vec![Point::new(0, 0), Point::new(10, 0)]));
        state
            .schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(3, Point::new(10, 0)));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(4, Point::new(10, 0), "OUT"));
        state.schematic.document_mut_for_test().design_notes.push(
            crate::state::DesignNote::new(
                5,
                Point::new(20, 10),
                crate::state::DesignNoteKind::PlainText,
                "Bias network",
            )
            .unwrap(),
        );
        state.schematic.session.editor.selection.select_component(1);
        state.schematic.session.editor.tool = crate::state::Tool::Wire;

        let description = canvas::accessibility_description(
            &canvas_accessibility_view(&state),
            &schematic_accessibility_shortcuts(
                &state,
                crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            ),
        );

        assert!(description.starts_with(
            "1 component, 1 wire, 0 buses, 0 bus taps, 1 junction, 1 net label, 1 design note, 0 documentation shapes, 0 probe flags."
        ));
        assert!(description.contains("Active tool: Wire."));
        assert!(!description.contains("Selected instance"));
        assert!(!description.contains("item selected"));
        assert!(description.contains(
            "Arrow keys select the nearest eligible schematic object in each direction."
        ));
        assert!(description.contains("Escape: Cancel active command"));
        assert!(description.contains("Shift+T: Place text or note"));
        assert!(description.contains("S: Stretch selection"));
        assert_eq!(
            canvas::selection_accessibility_status(&canvas_accessibility_view(&state)),
            "Selected instance unnamed, Resistor."
        );
        state.schematic.session.editor.selection.clear();
        assert_eq!(
            canvas::accessibility_description(
                &canvas_accessibility_view(&state),
                &schematic_accessibility_shortcuts(
                    &state,
                    crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
                    egui::os::OperatingSystem::Windows,
                )
            ),
            description
        );
        assert_eq!(
            canvas::selection_accessibility_status(&canvas_accessibility_view(&state)),
            "No schematic object selected."
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn canvas_accessibility_uses_a_stable_node_and_concise_live_selection_status() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut component = Component::new(17, ComponentType::Resistor, Point::new(40, 20));
        component.name = "RGAIN".to_owned();
        component.value = "499 ohm".to_owned();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(component);
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(17);

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| render_schematic_view(ui, &mut state, None));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("schematic accessibility tree")
            .nodes;
        let canvas = nodes
            .iter()
            .find(|(_, node)| node.role() == egui::accesskit::Role::Canvas)
            .map(|(_, node)| node)
            .expect("schematic canvas node");
        assert_eq!(canvas.label(), Some("Schematic canvas"));
        assert!(
            canvas
                .description()
                .is_some_and(|description| description.contains(
                    "Arrow keys select the nearest eligible schematic object in each direction."
                ))
        );
        assert_eq!(canvas.live(), None);

        let status = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Status
                    && node.label() == Some("Selected instance RGAIN, 499 ohm.")
            })
            .map(|(_, node)| node)
            .expect("concise schematic selection status node");
        assert_eq!(status.live(), Some(egui::accesskit::Live::Polite));
    }
}
