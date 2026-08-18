//! Keyboard shortcut dispatch, and the edit actions it resolves to.
//!
//! [`RSpiceApp::handle_shortcuts`] is the one place a key press becomes a
//! command: it resolves the chord against the active profile, checks the
//! command is available in this workspace, and dispatches. The submodules
//! hold the actions themselves — file operations, workspace navigation,
//! property edits, and image export.

pub(in crate::workbench) mod export_image;
pub(in crate::workbench) mod file;
pub(in crate::workbench) mod property_edit;
pub(in crate::workbench) mod sheets;
pub(in crate::workbench) mod workspace;

use egui::{Context, Popup};

use crate::diagnostics::ConsoleMessage;
use crate::schematic::view::SchematicSymbolContext;
use crate::state::{Point, SymbolDocument, SymbolShape};
use crate::workbench::commands::vocabulary::Command as ShortcutCommand;
use crate::workbench::{
    SymbolClipboard, SymbolSelection, mirror_point_h_about, mirror_point_v_about,
    mirror_shape_h_about, mirror_shape_v_about, rotate_point_cw_about, rotate_shape_cw_about,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::shortcuts::{
    ShortcutEnvironment, ShortcutInputSnapshot, engineering_canvas_has_focus,
    runtime_command_platform,
};

fn shortcut_dispatch_blocked(state: &AppState, ctx: &Context) -> bool {
    state.application_modal_open() || state.workbench.drawer.is_some() || Popup::is_any_open(ctx)
}

fn symbol_clipboard_from_selection(
    document: &SymbolDocument,
    selection: &SymbolSelection,
) -> SymbolClipboard {
    let shapes = selection
        .shapes
        .iter()
        .filter_map(|index| document.body.get(*index).cloned())
        .collect();
    let pins = selection
        .pins
        .iter()
        .filter_map(|name| document.pin(name).cloned())
        .collect();
    SymbolClipboard { pins, shapes }
}

fn unique_symbol_pin_name(document: &SymbolDocument, base: &str) -> String {
    let mut candidate = format!("{base}_copy");
    let mut suffix = 2usize;
    while document
        .pins
        .iter()
        .any(|pin| pin.name.eq_ignore_ascii_case(&candidate))
    {
        candidate = format!("{base}_copy{suffix}");
        suffix += 1;
    }
    candidate
}

impl RSpiceApp {
    /// Handle keyboard shortcuts
    pub(in crate::workbench) fn handle_shortcuts(&mut self, ctx: &Context) {
        if shortcut_dispatch_blocked(&self.state, ctx) {
            self.state.shortcut_resolver.reset();
            return;
        }
        let active_view = self.state.workspace.active_view_type();
        let canvas_focus =
            engineering_canvas_has_focus(ctx, self.state.workbench.workspace, active_view);
        if crate::schematic::view::handle_pre_render_placement_transform(
            ctx,
            &mut self.state,
            canvas_focus,
        ) {
            self.state.shortcut_resolver.reset();
            return;
        }
        let non_canvas_focus = ctx.memory(|memory| memory.focused().is_some()) && !canvas_focus;
        let snapshot =
            ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, non_canvas_focus));
        let injected_now =
            ctx.input(|input| std::time::Duration::from_secs_f64(input.time.max(0.0)));
        let profile = self.state.ui.preferences.shortcuts().clone();
        let platform = runtime_command_platform(ctx);
        let environment = ShortcutEnvironment {
            workspace: self.state.workbench.workspace,
            active_view,
            canvas_focus,
        };
        let mut resolver = std::mem::take(&mut self.state.shortcut_resolver);
        let resolution = resolver.resolve(
            &snapshot,
            &profile,
            platform,
            environment,
            injected_now,
            |command| command.availability(self).is_available(),
        );
        self.state.shortcut_resolver = resolver;

        if let Some(delay) = resolution.repaint_after {
            ctx.request_repaint_after(delay);
        }
        let consumed = resolution
            .consume
            .iter()
            .all(|(key, modifiers)| ctx.input_mut(|input| input.consume_key(*modifiers, *key)));
        if consumed && let Some(command) = resolution.command {
            self.execute_shortcut_command(command);
        }
    }

    pub(in crate::workbench) fn execute_shortcut_command(&mut self, command: ShortcutCommand) {
        use crate::state::{ComponentType, Tool};

        if self.state.workspace.active_view_type() == crate::state::ViewType::Symbol
            && self.execute_symbol_shortcut_command(command)
        {
            return;
        }

        if self.state.schematic_edit_read_only() && command_edits_schematic(command) {
            self.state.deny_read_only_edit();
            return;
        }

        match command {
            ShortcutCommand::ProjectLauncher => self.state.workbench.open_project_launcher(),
            ShortcutCommand::NewProject => self.execute_project_file_shortcut(
                crate::workbench::menu_bar::FileMenuAction::NewProject,
            ),
            ShortcutCommand::OpenProject => self.execute_project_file_shortcut(
                crate::workbench::menu_bar::FileMenuAction::OpenProject,
            ),
            ShortcutCommand::Save => {
                self.execute_project_file_shortcut(crate::workbench::menu_bar::FileMenuAction::Save)
            }
            ShortcutCommand::SaveAs => self.execute_project_file_shortcut(
                crate::workbench::menu_bar::FileMenuAction::SaveProjectAs,
            ),
            ShortcutCommand::SaveAll => self
                .execute_project_file_shortcut(crate::workbench::menu_bar::FileMenuAction::SaveAll),
            ShortcutCommand::CloseActiveDocument => self.execute_project_file_shortcut(
                crate::workbench::menu_bar::FileMenuAction::CloseActiveDocument,
            ),
            ShortcutCommand::CloseProject => self.execute_project_file_shortcut(
                crate::workbench::menu_bar::FileMenuAction::CloseProject,
            ),
            ShortcutCommand::Undo => self.action_edit_undo(),
            ShortcutCommand::Redo => self.action_edit_redo(),
            ShortcutCommand::Copy => self.action_edit_copy(),
            ShortcutCommand::Paste => self.action_edit_paste(),
            ShortcutCommand::Cut => self.action_edit_cut(),
            ShortcutCommand::Delete => self.action_edit_delete(),
            ShortcutCommand::SelectAll => self.action_edit_select_all(),
            ShortcutCommand::ToggleNavigator => {
                ShortcutCommand::ToggleNavigator.execute(self);
            }
            ShortcutCommand::ToggleConsole => {
                ShortcutCommand::ToggleConsole.execute(self);
            }
            ShortcutCommand::KeyboardShortcuts => {
                self.state.dialogs.shortcuts_help = true;
            }
            ShortcutCommand::SelectTool => {
                self.state.schematic.arm_tool(Tool::Select);
            }
            ShortcutCommand::PlaceWire => {
                self.state.schematic.arm_tool(Tool::Wire);
            }
            ShortcutCommand::PlaceBus => {
                self.state.schematic.arm_tool(Tool::Bus);
            }
            ShortcutCommand::PlaceBusTap => {
                self.state.dialogs.bus_tap.open();
            }
            ShortcutCommand::PlaceJunction => {
                self.state.schematic.arm_tool(Tool::Junction);
            }
            ShortcutCommand::Place(ComponentType::Ground) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Ground));
            }
            ShortcutCommand::Place(ComponentType::VoltageSource) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::VoltageSource));
            }
            ShortcutCommand::Place(ComponentType::CurrentSource) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::CurrentSource));
            }
            ShortcutCommand::Place(ComponentType::Capacitor) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Capacitor));
            }
            ShortcutCommand::Place(ComponentType::Inductor) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Inductor));
            }
            ShortcutCommand::Place(ComponentType::Diode) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Diode));
            }
            ShortcutCommand::Place(ComponentType::Nmos) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Nmos));
            }
            ShortcutCommand::Place(ComponentType::NpnBjt) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::NpnBjt));
            }
            ShortcutCommand::PlaceProbe => {
                self.state.schematic.arm_tool(Tool::Probe);
            }
            ShortcutCommand::PlacePin
            | ShortcutCommand::PlaceText
            | ShortcutCommand::PlaceShape
            | ShortcutCommand::MoveSelection
            | ShortcutCommand::StretchSelection
            | ShortcutCommand::ArraySelection
            | ShortcutCommand::ReplaceInstance => command.execute(self),
            ShortcutCommand::SymbolPinTool
            | ShortcutCommand::SymbolPolylineTool
            | ShortcutCommand::SymbolRectangleTool
            | ShortcutCommand::SymbolCircleTool
            | ShortcutCommand::SymbolArcTool
            | ShortcutCommand::SymbolPolygonTool
            | ShortcutCommand::SymbolTextTool
            | ShortcutCommand::SymbolRotatePin
            | ShortcutCommand::SymbolMirrorPin
            | ShortcutCommand::SymbolSave => {
                command.execute(self);
            }
            ShortcutCommand::Place(ComponentType::Resistor) => {
                self.state
                    .schematic
                    .arm_tool(Tool::Place(ComponentType::Resistor));
            }
            ShortcutCommand::RotateSelection => {
                self.state.schematic.preview_rotation =
                    self.state.schematic.preview_rotation.rotate_cw();
                if !self.state.schematic.selection.is_empty() {
                    self.rotate_schematic_selection_with_symbols();
                }
            }
            ShortcutCommand::MirrorSelectionHorizontal => {
                if !self.state.schematic.selection.is_empty() {
                    self.mirror_schematic_selection_h_with_symbols();
                }
            }
            ShortcutCommand::MirrorSelectionVertical => {
                if !self.state.schematic.selection.is_empty() {
                    self.mirror_schematic_selection_v_with_symbols();
                }
            }
            ShortcutCommand::ObjectProperties => {
                crate::workbench::app::open_selected_object_properties(&mut self.state);
            }
            ShortcutCommand::Cancel => {
                if self.state.dialogs.move_selection.armed {
                    crate::workbench::app::cancel_armed_move_selection(&mut self.state);
                } else if self.state.dialogs.stretch_selection.armed {
                    crate::workbench::app::cancel_armed_stretch_selection(&mut self.state);
                } else if self.state.dialogs.array_selection.armed {
                    crate::workbench::app::cancel_armed_array_selection(&mut self.state);
                } else if self.state.workbench.drawer.is_some() {
                    self.state.workbench.close_drawer();
                } else if self.state.dialogs.object_properties.open {
                    self.state.dialogs.object_properties.attempt_close();
                } else if self.state.tabbed_property_dialog.open {
                    self.state.tabbed_property_dialog.attempt_close();
                } else if self.state.workbench.workspace
                    == crate::workbench::state::Workspace::Results
                    && self.state.ui.results.cursors.any()
                {
                    self.state.ui.results.clear_cursors();
                } else {
                    self.state.schematic.cancel_interaction_step();
                }
            }
            ShortcutCommand::RunSimulation => {
                if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
                    if self.manual_deck_run_block_reason().is_none() {
                        self.state.request_netlist_manual_deck_run();
                    }
                } else {
                    self.state.workbench.preflight.request_run_and_queue();
                }
            }
            ShortcutCommand::StopSimulation => {
                if self.state.simulation.can_request_abort_active_run()
                    && crate::simulation::execution::execution_target_supports_cancellation()
                    && let Err(error) = self.state.simulation.request_abort_active_run()
                {
                    self.state.push_sim_message(ConsoleMessage::warning(error));
                }
            }
            ShortcutCommand::RunChecks => {
                crate::workbench::menu_bar::run_design_rule_check(&mut self.state);
            }
            ShortcutCommand::NextViolation => {
                crate::schematic::view::violations::cycle_violation(&mut self.state, 1);
            }
            ShortcutCommand::PreviousViolation => {
                crate::schematic::view::violations::cycle_violation(&mut self.state, -1);
            }
            ShortcutCommand::NextWorkspace => {
                self.state.workbench.cycle_workspace(false);
            }
            ShortcutCommand::ZoomIn | ShortcutCommand::ZoomOut => command.execute(self),
            ShortcutCommand::ToggleLinkedCursors => {
                self.state.ui.results.toggle_linked_cursors();
            }
            ShortcutCommand::ZoomFit
            | ShortcutCommand::FitSchematicContent
            | ShortcutCommand::DrawingSheetLayers => command.execute(self),
            ShortcutCommand::ZoomOneToOne => {
                self.state.schematic.zoom = 1.0;
            }
            ShortcutCommand::PlaceLabel => {
                self.state.schematic.arm_tool(Tool::Label);
            }
            ShortcutCommand::PlaceInstance => {
                self.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
                self.state.workbench.navigator_visible = true;
                self.state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);
                self.state.workbench.design_panel =
                    crate::workbench::state::DesignPanel::ComponentShelf;
                self.state.workbench.focus_placement_search = true;
            }
            ShortcutCommand::DescendHierarchy => {
                crate::workbench::app::open_descend_hierarchy_dialog(&mut self.state);
            }
            ShortcutCommand::DescendHierarchyDirect => {
                self.state.open_selected_instance_master();
            }
            ShortcutCommand::AscendHierarchy => {
                self.state.ascend_workspace_level();
            }
            ShortcutCommand::FindInDesign => {
                if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
                    self.state.ui.netlist.find.open = true;
                } else {
                    self.state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
                    self.state.workbench.navigator_visible = true;
                    self.state.workbench.focus_navigator_search = true;
                }
            }
            ShortcutCommand::CommandPalette => {
                self.state.dialogs.command_palette.open();
            }
            _ => command.execute(self),
        }
    }

    fn execute_project_file_shortcut(
        &mut self,
        action: crate::workbench::menu_bar::FileMenuAction,
    ) {
        crate::workbench::menu_bar::dispatch_file_menu_action(
            &mut self.state,
            action,
            self.file_workflow_io.as_ref(),
            self.export_workflow_io.as_ref(),
        );
    }

    fn execute_symbol_shortcut_command(&mut self, command: ShortcutCommand) -> bool {
        use crate::workbench::SymbolTool;

        match command {
            ShortcutCommand::Undo => {
                self.action_edit_undo();
                true
            }
            ShortcutCommand::Redo => {
                self.action_edit_redo();
                true
            }
            ShortcutCommand::Delete => {
                self.delete_selected_symbol_item(false);
                true
            }
            ShortcutCommand::Cut => {
                self.delete_selected_symbol_item(true);
                true
            }
            ShortcutCommand::Copy => {
                self.copy_selected_symbol_shape();
                true
            }
            ShortcutCommand::Paste => {
                self.paste_symbol_shape();
                true
            }
            ShortcutCommand::SelectAll => {
                self.select_all_symbol_items();
                true
            }
            ShortcutCommand::SelectTool => {
                self.state.ui.symbol.tool = SymbolTool::Select;
                true
            }
            ShortcutCommand::SymbolPinTool => {
                self.state.ui.symbol.tool = SymbolTool::PlacePin;
                let next = self
                    .state
                    .load_active_symbol_document()
                    .ok()
                    .and_then(|document| {
                        document
                            .pins
                            .iter()
                            .find(|pin| pin.position.is_none())
                            .map(|pin| pin.name.clone())
                    });
                if let Some(pin) = next {
                    self.state.ui.symbol.select_pin(pin);
                } else {
                    self.state.ui.symbol.clear_selection();
                }
                true
            }
            ShortcutCommand::SymbolPolylineTool => {
                self.state.ui.symbol.tool = SymbolTool::Line;
                self.state.ui.symbol.pending_polyline.clear();
                true
            }
            ShortcutCommand::SymbolRectangleTool => {
                self.state.ui.symbol.tool = SymbolTool::Rectangle;
                self.state.ui.symbol.shape_start = None;
                true
            }
            ShortcutCommand::SymbolCircleTool => {
                self.state.ui.symbol.tool = SymbolTool::Circle;
                self.state.ui.symbol.shape_start = None;
                true
            }
            ShortcutCommand::SymbolArcTool => {
                self.state.ui.symbol.tool = SymbolTool::Arc;
                self.state.ui.symbol.shape_start = None;
                true
            }
            ShortcutCommand::SymbolPolygonTool => {
                self.state.ui.symbol.tool = SymbolTool::Polygon;
                self.state.ui.symbol.pending_polyline.clear();
                true
            }
            ShortcutCommand::SymbolTextTool => {
                self.state.ui.symbol.tool = SymbolTool::Text;
                true
            }
            ShortcutCommand::SymbolRotatePin
            | ShortcutCommand::SymbolMirrorPin
            | ShortcutCommand::SymbolSave => {
                command.execute(self);
                true
            }
            ShortcutCommand::RotateSelection => {
                self.transform_selected_symbol_item(rotate_point_cw_about, rotate_shape_cw_about);
                true
            }
            ShortcutCommand::MirrorSelectionHorizontal => {
                self.transform_selected_symbol_item(mirror_point_h_about, mirror_shape_h_about);
                true
            }
            ShortcutCommand::MirrorSelectionVertical => {
                self.transform_selected_symbol_item(mirror_point_v_about, mirror_shape_v_about);
                true
            }
            ShortcutCommand::Cancel => {
                self.finish_pending_symbol_polyline_from_shortcut();
                self.state.ui.symbol.tool = SymbolTool::Select;
                self.state.ui.symbol.clear_drag_state();
                self.state.ui.symbol.shape_start = None;
                self.state.ui.symbol.marquee_start = None;
                self.state.ui.symbol.marquee_current = None;
                true
            }
            ShortcutCommand::ZoomIn | ShortcutCommand::ZoomOut => {
                command.execute(self);
                true
            }
            ShortcutCommand::ZoomFit => {
                self.state.ui.symbol.needs_fit = true;
                true
            }
            ShortcutCommand::ZoomOneToOne => {
                self.state.ui.symbol.zoom = 4.0;
                self.state.ui.symbol.pan = (0.0, 0.0);
                true
            }
            ShortcutCommand::RunChecks => {
                self.state.run_active_symbol_pin_checks();
                true
            }
            ShortcutCommand::NextViolation => {
                crate::schematic::view::violations::cycle_violation(&mut self.state, 1);
                true
            }
            ShortcutCommand::PreviousViolation => {
                crate::schematic::view::violations::cycle_violation(&mut self.state, -1);
                true
            }
            ShortcutCommand::ObjectProperties => {
                self.state.workbench.inspector_visible = true;
                self.state.workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
                true
            }
            ShortcutCommand::Place(_)
            | ShortcutCommand::PlaceWire
            | ShortcutCommand::PlaceBus
            | ShortcutCommand::PlaceBusTap
            | ShortcutCommand::PlaceJunction
            | ShortcutCommand::PlaceProbe
            | ShortcutCommand::PlacePin
            | ShortcutCommand::PlaceText
            | ShortcutCommand::PlaceShape
            | ShortcutCommand::MoveSelection
            | ShortcutCommand::StretchSelection
            | ShortcutCommand::ArraySelection
            | ShortcutCommand::ReplaceInstance
            | ShortcutCommand::PlaceLabel
            | ShortcutCommand::PlaceInstance
            | ShortcutCommand::DescendHierarchy
            | ShortcutCommand::DescendHierarchyDirect
            | ShortcutCommand::AscendHierarchy => true,
            _ => false,
        }
    }

    fn rotate_schematic_selection_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(&mut self.state);
        crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
            &mut self.state,
            |schematic| {
                schematic.rotate_selection_resolved(|component| {
                    symbol_context.terminal_points(component)
                })
            },
        );
    }

    fn mirror_schematic_selection_h_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(&mut self.state);
        crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
            &mut self.state,
            |schematic| {
                schematic.mirror_selection_h_resolved(|component| {
                    symbol_context.terminal_points(component)
                })
            },
        );
    }

    fn mirror_schematic_selection_v_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(&mut self.state);
        crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
            &mut self.state,
            |schematic| {
                schematic.mirror_selection_v_resolved(|component| {
                    symbol_context.terminal_points(component)
                })
            },
        );
    }

    pub(crate) fn select_all_symbol_items(&mut self) {
        let document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        self.state
            .ui
            .symbol
            .set_selection(SymbolSelection::all_in(&document));
    }

    pub(crate) fn copy_selected_symbol_shape(&mut self) {
        let document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let selection = self.state.ui.symbol.effective_selection();
        self.state.ui.symbol.clipboard = symbol_clipboard_from_selection(&document, &selection);
    }

    pub(crate) fn paste_symbol_shape(&mut self) {
        if self.state.deny_read_only_edit() {
            return;
        }
        let clipboard = self.state.ui.symbol.clipboard.clone();
        if clipboard.is_empty() {
            return;
        }
        let mut document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let metadata = match self.state.load_active_symbol_editor_metadata(&document) {
            Ok(metadata) => metadata,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let Some((min, max)) = clipboard.bounds() else {
            return;
        };
        let target = self.symbol_paste_target();
        let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
        let delta = target - center;
        self.state.record_symbol_edit(&document);
        let mut selection = SymbolSelection::default();
        for mut shape in clipboard.shapes {
            shape.translate(delta);
            document.body.push(shape);
            if let Some(index) = document.body.len().checked_sub(1) {
                selection.shapes.insert(index);
            }
        }
        for mut pin in clipboard.pins {
            pin.name = unique_symbol_pin_name(&document, &pin.name);
            if let Some(position) = pin.position.as_mut() {
                *position = *position + delta;
            }
            pin.offset = pin.position.map_or(0, |position| match pin.side() {
                crate::state::SymbolPinSide::Left | crate::state::SymbolPinSide::Right => {
                    position.y
                }
                crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => {
                    position.x
                }
            });
            selection.pins.insert(pin.name.clone());
            document.pins.push(pin);
        }
        self.state.ui.symbol.set_selection(selection);
        if let Err(error) = self
            .state
            .store_active_symbol_editor_bundle(&document, &metadata)
        {
            self.state.push_user_message(ConsoleMessage::warning(error));
        }
    }

    pub(crate) fn delete_selected_symbol_item(&mut self, cut: bool) {
        let selection = self.state.ui.symbol.effective_selection();
        if selection.is_empty() {
            return;
        }
        if self.state.deny_read_only_edit() {
            return;
        }
        let mut document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let metadata = match self.state.load_active_symbol_editor_metadata(&document) {
            Ok(metadata) => metadata,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let before = document.clone();
        let mut clipboard = SymbolClipboard::default();
        let mut changed = false;

        for index in selection.shapes.iter().rev().copied() {
            if index < document.body.len() {
                let removed = document.body.remove(index);
                if cut {
                    clipboard.shapes.push(removed);
                }
                changed = true;
            }
        }

        let mut retained = Vec::with_capacity(document.pins.len());
        for pin in std::mem::take(&mut document.pins) {
            if selection.pins.contains(&pin.name) {
                if cut {
                    clipboard.pins.push(pin);
                }
                changed = true;
            } else {
                retained.push(pin);
            }
        }
        document.pins = retained;

        if cut {
            clipboard.shapes.reverse();
            self.state.ui.symbol.clipboard = clipboard;
        }
        if changed {
            self.state.record_symbol_edit(&before);
            self.state.ui.symbol.clear_selection();
            if let Err(error) = self
                .state
                .store_active_symbol_editor_bundle(&document, &metadata)
            {
                self.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
    }

    fn transform_selected_symbol_item(
        &mut self,
        pin_transform: impl Fn(Point, Point) -> Point,
        shape_transform: impl Fn(&mut SymbolShape, Point),
    ) {
        let selection = self.state.ui.symbol.effective_selection();
        if selection.is_empty() {
            return;
        }
        if self.state.deny_read_only_edit() {
            return;
        }
        let mut document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let mut metadata = match self.state.load_active_symbol_editor_metadata(&document) {
            Ok(metadata) => metadata,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let before = document.clone();
        let origin = document.origin;
        let mut changed = false;
        for index in selection.shapes.iter().copied() {
            if let Some(shape) = document.body.get_mut(index) {
                shape_transform(shape, origin);
                changed = true;
            }
        }
        for name in &selection.pins {
            if let Some(pin) = document.pin_mut(name)
                && let Some(position) = pin.position
            {
                let transformed = pin_transform(position, origin);
                pin.position = Some(transformed);
                pin.offset = match pin.side() {
                    crate::state::SymbolPinSide::Left | crate::state::SymbolPinSide::Right => {
                        transformed.y
                    }
                    crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => {
                        transformed.x
                    }
                };
                changed = true;
            }
        }
        for kind in &selection.attributes {
            if let Some(attribute) = metadata.attribute_mut(*kind) {
                attribute.position = pin_transform(attribute.position, origin);
                match kind {
                    crate::state::SymbolAttributeKind::Reference => {
                        document.name_anchor = attribute.position;
                    }
                    crate::state::SymbolAttributeKind::Value => {
                        document.value_anchor = attribute.position;
                    }
                    crate::state::SymbolAttributeKind::Model => {}
                }
                changed = true;
            }
        }
        if changed {
            self.state.record_symbol_edit(&before);
            if let Err(error) = self
                .state
                .store_active_symbol_editor_bundle(&document, &metadata)
            {
                self.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
    }

    fn symbol_paste_target(&self) -> Point {
        self.state
            .ui
            .canvas_hover
            .or(self.state.ui.canvas_view_center)
            .map(|(x, y)| Point::new(x.round() as i32, y.round() as i32))
            .unwrap_or_else(|| Point::new(10, 10))
    }

    fn finish_pending_symbol_polyline_from_shortcut(&mut self) {
        if self.state.ui.symbol.pending_polyline.len() < 2 {
            self.state.ui.symbol.pending_polyline.clear();
            return;
        }
        if self.state.deny_read_only_edit() {
            self.state.ui.symbol.pending_polyline.clear();
            return;
        }
        let mut document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        let before = document.clone();
        let points = std::mem::take(&mut self.state.ui.symbol.pending_polyline);
        document.body.push(SymbolShape::Polyline {
            points,
            closed: false,
        });
        self.state.record_symbol_edit(&before);
        if let Some(index) = document.body.len().checked_sub(1) {
            self.state.ui.symbol.select_shape(index);
        }
        if let Err(error) = self.state.store_active_symbol_document(&document) {
            self.state.push_user_message(ConsoleMessage::warning(error));
        }
    }

    fn try_project_design_undo(&mut self) -> bool {
        match self.state.undo_project_design() {
            Ok(Some(description)) => {
                self.invalidate_simulation_preflight();
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Undo: {description}")));
            }
            Ok(None) => return false,
            Err(error) => self.state.push_user_message(ConsoleMessage::warning(error)),
        }
        true
    }

    fn try_project_design_redo(&mut self) -> bool {
        match self.state.redo_project_design() {
            Ok(Some(description)) => {
                self.invalidate_simulation_preflight();
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Redo: {description}")));
            }
            Ok(None) => return false,
            Err(error) => self.state.push_user_message(ConsoleMessage::warning(error)),
        }
        true
    }

    fn try_active_document_undo(&mut self) -> bool {
        if self.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            return match self.state.undo_active_symbol_document() {
                Ok(true) => {
                    self.state
                        .push_user_message(ConsoleMessage::info("Undo: symbol edit"));
                    true
                }
                Ok(false) => false,
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::warning(error));
                    true
                }
            };
        }
        if !self.state.schematic.can_undo() {
            return false;
        }
        let description = self
            .state
            .schematic
            .undo_description()
            .unwrap_or("schematic edit")
            .to_owned();
        if self.state.schematic.undo() {
            self.state.ui.schematic_snap = self.state.schematic.snap_engine.clone();
            self.state.sync_active_schematic_to_workspace();
            self.state
                .push_user_message(ConsoleMessage::info(format!("Undo: {description}")));
            return true;
        }
        false
    }

    fn try_active_document_redo(&mut self) -> bool {
        if self.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            return match self.state.redo_active_symbol_document() {
                Ok(true) => {
                    self.state
                        .push_user_message(ConsoleMessage::info("Redo: symbol edit"));
                    true
                }
                Ok(false) => false,
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::warning(error));
                    true
                }
            };
        }
        if !self.state.schematic.can_redo() {
            return false;
        }
        let description = self
            .state
            .schematic
            .redo_description()
            .unwrap_or("schematic edit")
            .to_owned();
        if self.state.schematic.redo() {
            self.state.ui.schematic_snap = self.state.schematic.snap_engine.clone();
            self.state.sync_active_schematic_to_workspace();
            self.state
                .push_user_message(ConsoleMessage::info(format!("Redo: {description}")));
            return true;
        }
        false
    }

    pub(crate) fn action_edit_undo(&mut self) {
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && self.state.ui.code_workspace.page
                == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        {
            match crate::workbench::documents::netlist_document::undo_netlist_edit(&mut self.state)
            {
                Ok(Some(description)) => {
                    self.state
                        .push_user_message(ConsoleMessage::info(format!("Undo: {description}")));
                    return;
                }
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::warning(error));
                    return;
                }
                Ok(None) => {}
            }
        }
        let project_first = self.state.project_undo_owns_active_document();
        if (project_first && self.try_project_design_undo())
            || self.try_active_document_undo()
            || (!project_first && self.try_project_design_undo())
        {
            return;
        }
        self.state
            .push_user_message(ConsoleMessage::info("Nothing to undo"));
    }

    pub(crate) fn action_edit_redo(&mut self) {
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && self.state.ui.code_workspace.page
                == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
        {
            match crate::workbench::documents::netlist_document::redo_netlist_edit(&mut self.state)
            {
                Ok(Some(description)) => {
                    self.state
                        .push_user_message(ConsoleMessage::info(format!("Redo: {description}")));
                    return;
                }
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::warning(error));
                    return;
                }
                Ok(None) => {}
            }
        }
        let project_first = self.state.project_redo_owns_active_document();
        if (project_first && self.try_project_design_redo())
            || self.try_active_document_redo()
            || (!project_first && self.try_project_design_redo())
        {
            return;
        }
        self.state
            .push_user_message(ConsoleMessage::info("Nothing to redo"));
    }

    pub(in crate::workbench) fn action_edit_copy(&mut self) {
        if self.state.workbench.workspace == crate::workbench::state::Workspace::Results {
            if let Some(text) =
                crate::workbench::documents::result_document::copy_cursor_text(&mut self.state)
            {
                self.state.ui.clipboard_text_request = Some(text);
            }
            return;
        }
        crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(&mut self.state);
        self.state.copy_active_schematic_selection();
    }

    pub(in crate::workbench) fn action_edit_paste(&mut self) {
        let anchor = self.state.schematic_paste_anchor();
        if !self.state.schematic.paste_at(anchor) {
            self.state.push_user_message(ConsoleMessage::warning(
                "Paste could not be completed at the current canvas target".to_owned(),
            ));
        }
    }

    pub(in crate::workbench) fn action_edit_cut(&mut self) {
        crate::workbench::app::open_cut_selection_dialog(&mut self.state);
    }

    pub(in crate::workbench) fn action_edit_delete(&mut self) {
        crate::workbench::app::open_delete_selection_dialog(&mut self.state);
    }

    pub(in crate::workbench) fn action_edit_select_all(&mut self) {
        crate::workbench::app::open_select_all_dialog(&mut self.state);
    }
}

/// Commands that mutate the open schematic — refused on read-only views.
/// Copy and Select All are reads; tools that only inspect (select, probe)
/// and navigation stay live.
fn command_edits_schematic(command: ShortcutCommand) -> bool {
    crate::workbench::commands::command_edits_schematic(command)
}

#[cfg(test)]
mod shortcut_ownership_tests {
    use super::*;

    fn assert_grid_pitch_contract(app: &RSpiceApp, pitch: crate::state::SchematicGridPitch) {
        let expected = pitch.canvas_grid_size();
        assert_eq!(app.state.schematic.document_policy.grid_pitch, pitch);
        assert_eq!(app.state.schematic.grid_size, expected);
        assert_eq!(app.state.schematic.snap_engine.grid_size, expected);
        assert_eq!(app.state.ui.schematic_snap.grid_size, expected);
    }

    #[test]
    fn document_undo_and_redo_reconcile_all_grid_pitch_authorities() {
        use crate::state::SchematicGridPitch;

        let mut app = RSpiceApp::test_instance();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        assert!(
            app.state
                .schematic
                .with_undo("change schematic grid pitch", |schematic| {
                    schematic.document_policy.grid_pitch = SchematicGridPitch::Mil25;
                    schematic.grid_size = SchematicGridPitch::Mil25.canvas_grid_size();
                    schematic.snap_engine.grid_size = SchematicGridPitch::Mil25.canvas_grid_size();
                })
        );
        app.state.ui.schematic_snap = app.state.schematic.snap_engine.clone();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);

        app.action_edit_undo();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil50);

        app.action_edit_redo();
        assert_grid_pitch_contract(&app, SchematicGridPitch::Mil25);
    }

    #[test]
    fn open_popup_blocks_application_shortcut_dispatch() {
        let ctx = Context::default();
        let state = AppState::default();
        assert!(!shortcut_dispatch_blocked(&state, &ctx));

        Popup::open_id(&ctx, egui::Id::new("shortcut-ownership-test"));

        assert!(shortcut_dispatch_blocked(&state, &ctx));
    }

    #[test]
    fn open_responsive_drawer_blocks_background_shortcut_dispatch() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);

        assert!(shortcut_dispatch_blocked(&state, &ctx));
    }

    #[test]
    fn linked_cursor_command_toggles_real_results_state() {
        let mut app = RSpiceApp::test_instance();
        assert!(!app.state.ui.results.linked_cursors);
        app.execute_shortcut_command(ShortcutCommand::ToggleLinkedCursors);
        assert!(app.state.ui.results.linked_cursors);
        app.execute_shortcut_command(ShortcutCommand::ToggleLinkedCursors);
        assert!(!app.state.ui.results.linked_cursors);
    }

    #[test]
    fn copy_shortcut_in_results_copies_cursor_readout_not_schematic_selection() {
        let mut app = RSpiceApp::test_instance();
        let analysis =
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![0.0, 2.0],
                    "#ffbd2e",
                )]);
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
        app.state.ui.results.cursors.a = Some(0.5);
        app.state.ui.results.cursor_strip = Some(0);

        app.execute_shortcut_command(ShortcutCommand::Copy);

        let copied = app
            .state
            .ui
            .clipboard_text_request
            .as_deref()
            .expect("cursor readout copied");
        assert!(copied.starts_with('A'));
        assert!(copied.contains("V(out)"));
    }

    #[test]
    fn object_properties_shortcut_dispatches_bus_and_guards_dirty_cancel() {
        use crate::state::{Bus, BusDeclaration, Point};

        let mut app = RSpiceApp::test_instance();
        app.state.schematic.buses.push(
            Bus::segment(
                87,
                Point::new(0, 0),
                Point::new(20, 0),
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap(),
        );
        app.state.schematic.selection.select_only_bus(87);

        app.execute_shortcut_command(ShortcutCommand::ObjectProperties);
        assert!(matches!(
            app.state.dialogs.object_properties.draft,
            Some(crate::workbench::app::ObjectPropertiesDraft::Bus(_))
        ));
        let Some(crate::workbench::app::ObjectPropertiesDraft::Bus(draft)) =
            app.state.dialogs.object_properties.draft.as_mut()
        else {
            unreachable!()
        };
        draft.declaration = "ADDR[7:0]".to_owned();
        app.state.dialogs.object_properties.mark_edited();

        app.execute_shortcut_command(ShortcutCommand::Cancel);
        assert!(app.state.dialogs.object_properties.open);
        assert!(app.state.dialogs.object_properties.discard_confirm);
        app.execute_shortcut_command(ShortcutCommand::Cancel);
        assert!(!app.state.dialogs.object_properties.open);
    }

    #[test]
    fn next_marker_resolves_again_after_first_jump_activates_design() {
        use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
        use crate::workbench::commands::vocabulary::CommandPlatform;

        let mut app = RSpiceApp::test_instance();
        let mut result = DrcResult::new();
        for (id, x) in [(1, 0.0), (2, 10.0)] {
            result.add_violation(DrcViolation::new(
                id,
                DrcViolationType::DanglingWire,
                format!("Finding {id}"),
                DrcLocation::Point { x, y: 0.0 },
            ));
        }
        result.completed = true;
        // Availability resolves against the design-check receipt, so publish
        // through the canonical owner and let it refresh the legacy canvas
        // projection the violation cursor still reads.
        app.state
            .publish_active_design_check_result(result)
            .expect("publish the active design-check receipt");
        app.state.refresh_active_design_check_projection();
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Verify);

        app.execute_shortcut_command(ShortcutCommand::NextViolation);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
        assert_eq!(app.state.dialogs.drc_cycle, Some(0));

        let snapshot = ShortcutInputSnapshot::from_events_for_test(
            &[egui::Event::Key {
                key: egui::Key::F8,
                physical_key: Some(egui::Key::F8),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            false,
        );
        let profile = app.state.ui.preferences.shortcuts().clone();
        let environment = ShortcutEnvironment {
            workspace: app.state.workbench.workspace,
            active_view: app.state.workspace.active_view_type(),
            canvas_focus: false,
        };
        let mut resolver = std::mem::take(&mut app.state.shortcut_resolver);
        let resolution = resolver.resolve(
            &snapshot,
            &profile,
            CommandPlatform::Desktop,
            environment,
            std::time::Duration::from_millis(1),
            |command| command.availability(&app).is_available(),
        );
        app.state.shortcut_resolver = resolver;
        assert_eq!(resolution.command, Some(ShortcutCommand::NextViolation));

        app.execute_shortcut_command(resolution.command.unwrap());
        assert_eq!(app.state.dialogs.drc_cycle, Some(1));
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Design
        );
    }
}

#[cfg(test)]
mod symbol_action_tests {
    use super::*;
    use crate::state::{Cell, CellViewRef, Library, PortDirection, SymbolPin, View, ViewType};

    #[test]
    fn registered_symbol_tools_are_available_and_execute_complete_state_transitions() {
        let mut app = RSpiceApp::test_instance();
        let reference = CellViewRef::new("shortcut_tools", "amp", "symbol");
        let mut library = Library::new("shortcut_tools");
        let mut cell = Cell::new("amp");
        let mut view = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![
                SymbolPin::new("PLACED", PortDirection::In, Some(Point::new(-10, 0))),
                SymbolPin::new("NEXT", PortDirection::Out, None),
            ],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut view)
        .unwrap();
        cell.add_view(view);
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);

        for command in [
            ShortcutCommand::SelectTool,
            ShortcutCommand::SymbolPinTool,
            ShortcutCommand::SymbolPolylineTool,
            ShortcutCommand::SymbolRectangleTool,
            ShortcutCommand::SymbolCircleTool,
            ShortcutCommand::SymbolArcTool,
            ShortcutCommand::SymbolPolygonTool,
            ShortcutCommand::SymbolTextTool,
            ShortcutCommand::ZoomFit,
        ] {
            assert!(command.availability(&app).is_available(), "{command:?}");
        }

        app.execute_shortcut_command(ShortcutCommand::SymbolPinTool);
        assert_eq!(
            app.state.ui.symbol.tool,
            crate::workbench::SymbolTool::PlacePin
        );
        assert_eq!(app.state.ui.symbol.selected_pin.as_deref(), Some("NEXT"));

        app.state.ui.symbol.pending_polyline = vec![Point::new(0, 0)];
        app.execute_shortcut_command(ShortcutCommand::SymbolPolylineTool);
        assert_eq!(app.state.ui.symbol.tool, crate::workbench::SymbolTool::Line);
        assert!(app.state.ui.symbol.pending_polyline.is_empty());

        for (command, expected) in [
            (
                ShortcutCommand::SymbolRectangleTool,
                crate::workbench::SymbolTool::Rectangle,
            ),
            (
                ShortcutCommand::SymbolCircleTool,
                crate::workbench::SymbolTool::Circle,
            ),
            (
                ShortcutCommand::SymbolArcTool,
                crate::workbench::SymbolTool::Arc,
            ),
            (
                ShortcutCommand::SymbolPolygonTool,
                crate::workbench::SymbolTool::Polygon,
            ),
            (
                ShortcutCommand::SymbolTextTool,
                crate::workbench::SymbolTool::Text,
            ),
            (
                ShortcutCommand::SelectTool,
                crate::workbench::SymbolTool::Select,
            ),
        ] {
            app.state.ui.symbol.shape_start = Some(Point::new(10, 10));
            app.execute_shortcut_command(command);
            assert_eq!(app.state.ui.symbol.tool, expected);
            if matches!(
                command,
                ShortcutCommand::SymbolRectangleTool
                    | ShortcutCommand::SymbolCircleTool
                    | ShortcutCommand::SymbolArcTool
            ) {
                assert!(app.state.ui.symbol.shape_start.is_none());
            }
        }

        app.state.ui.symbol.needs_fit = false;
        app.execute_shortcut_command(ShortcutCommand::ZoomFit);
        assert!(app.state.ui.symbol.needs_fit);
    }

    #[test]
    fn symbol_clipboard_copies_selected_shapes_and_pins() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-10, 0))),
                SymbolPin::new("TRIM", PortDirection::InOut, Some(Point::new(0, 10))),
            ],
            body: vec![SymbolShape::Dot {
                center: Point::origin(),
                radius: 3,
            }],
            ..SymbolDocument::default()
        };
        let mut selection = SymbolSelection::default();
        selection.pins.insert("IN".to_owned());
        selection.pins.insert("TRIM".to_owned());
        selection.shapes.insert(0);

        let clipboard = symbol_clipboard_from_selection(&document, &selection);

        assert_eq!(clipboard.shapes.len(), 1);
        assert_eq!(
            clipboard
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>(),
            vec!["IN", "TRIM"]
        );
    }
}
