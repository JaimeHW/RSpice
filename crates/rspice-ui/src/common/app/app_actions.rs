use egui::{Context, Popup};

use crate::schematic::view::SchematicSymbolContext;
use crate::state::{Point, SymbolDocument, SymbolShape};
use crate::workbench::commands::Command as ShortcutCommand;
use crate::workbench::{
    SymbolClipboard, SymbolSelection, mirror_point_h_about, mirror_point_v_about,
    mirror_shape_h_about, mirror_shape_v_about, rotate_point_cw_about, rotate_shape_cw_about,
};

use super::{
    AppState, ConsoleMessage, RSpiceApp,
    app_shortcuts::{
        ShortcutEnvironment, ShortcutInputSnapshot, engineering_canvas_has_focus,
        runtime_command_platform,
    },
};

fn shortcut_dispatch_blocked(state: &AppState, ctx: &Context) -> bool {
    state.application_modal_open() || state.workbench.drawer.is_some() || Popup::is_any_open(ctx)
}

fn symbol_pin_is_contract(ports: &[crate::state::PortSpec], name: &str) -> bool {
    ports
        .iter()
        .any(|port| port.name.eq_ignore_ascii_case(name))
}

fn symbol_clipboard_from_selection(
    document: &SymbolDocument,
    selection: &SymbolSelection,
    ports: &[crate::state::PortSpec],
) -> SymbolClipboard {
    let shapes = selection
        .shapes
        .iter()
        .filter_map(|index| document.body.get(*index).cloned())
        .collect();
    let pins = selection
        .pins
        .iter()
        .filter(|name| !symbol_pin_is_contract(ports, name))
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
    pub(super) fn handle_shortcuts(&mut self, ctx: &Context) {
        if shortcut_dispatch_blocked(&self.state, ctx) {
            self.state.shortcut_resolver.reset();
            return;
        }
        let active_view = self.state.workspace.active_view_type();
        let canvas_focus =
            engineering_canvas_has_focus(ctx, self.state.workbench.workspace, active_view);
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

    pub(super) fn execute_shortcut_command(&mut self, command: ShortcutCommand) {
        use crate::state::{ComponentType, Tool};

        if self.state.workspace.active_view_type() == crate::state::ViewType::Symbol
            && self.execute_symbol_shortcut_command(command)
        {
            return;
        }

        if self.state.schematic.read_only && command_edits_schematic(command) {
            self.state.deny_read_only_edit();
            return;
        }

        match command {
            ShortcutCommand::ProjectLauncher => self.state.workbench.open_project_launcher(),
            ShortcutCommand::NewProject => self
                .execute_project_file_shortcut(crate::common::menu_bar::FileMenuAction::NewProject),
            ShortcutCommand::OpenProject => self.execute_project_file_shortcut(
                crate::common::menu_bar::FileMenuAction::OpenProject,
            ),
            ShortcutCommand::Save => {
                if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist
                    && self.state.ui.netlist.active_document
                        == crate::workbench::netlist_document::ActiveNetlistDocument::OwnedSource
                {
                    self.state.ui.netlist.save_dialog.open = true;
                    self.state.ui.netlist.save_dialog.error = None;
                } else {
                    self.execute_project_file_shortcut(
                        crate::common::menu_bar::FileMenuAction::Save,
                    )
                }
            }
            ShortcutCommand::SaveAs => self.execute_project_file_shortcut(
                crate::common::menu_bar::FileMenuAction::SaveProjectAs,
            ),
            ShortcutCommand::SaveAll => {
                self.execute_project_file_shortcut(crate::common::menu_bar::FileMenuAction::SaveAll)
            }
            ShortcutCommand::CloseActiveDocument => self.execute_project_file_shortcut(
                crate::common::menu_bar::FileMenuAction::CloseActiveDocument,
            ),
            ShortcutCommand::CloseProject => self.execute_project_file_shortcut(
                crate::common::menu_bar::FileMenuAction::CloseProject,
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
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Select,
                );
            }
            ShortcutCommand::PlaceWire => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Wire,
                );
            }
            ShortcutCommand::PlaceBus => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Bus,
                );
            }
            ShortcutCommand::PlaceBusTap => {
                self.state.dialogs.bus_tap.open();
            }
            ShortcutCommand::PlaceJunction => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Junction,
                );
            }
            ShortcutCommand::Place(ComponentType::Ground) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Ground),
                );
            }
            ShortcutCommand::Place(ComponentType::VoltageSource) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::VoltageSource),
                );
            }
            ShortcutCommand::Place(ComponentType::CurrentSource) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::CurrentSource),
                );
            }
            ShortcutCommand::Place(ComponentType::Capacitor) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Capacitor),
                );
            }
            ShortcutCommand::Place(ComponentType::Inductor) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Inductor),
                );
            }
            ShortcutCommand::Place(ComponentType::Diode) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Diode),
                );
            }
            ShortcutCommand::Place(ComponentType::Nmos) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Nmos),
                );
            }
            ShortcutCommand::Place(ComponentType::NpnBjt) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::NpnBjt),
                );
            }
            ShortcutCommand::PlaceProbe => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Probe,
                );
            }
            ShortcutCommand::SymbolPinTool
            | ShortcutCommand::SymbolPolylineTool
            | ShortcutCommand::SymbolCircleTool
            | ShortcutCommand::SymbolArcTool
            | ShortcutCommand::SymbolArrowTool
            | ShortcutCommand::SymbolDotTool => {
                command.execute(self);
            }
            ShortcutCommand::Place(ComponentType::Resistor) => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Place(ComponentType::Resistor),
                );
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
                super::open_selected_object_properties(&mut self.state);
            }
            ShortcutCommand::Cancel => {
                if self.state.workbench.drawer.is_some() {
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
                    crate::workbench::commands::cancel_schematic_tool(&mut self.state.schematic);
                    self.state.schematic.selection.clear();
                    self.state.schematic.selection_rect.cancel();
                }
            }
            ShortcutCommand::RunSimulation => {
                if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
                    if self.manual_deck_run_block_reason().is_none() {
                        self.state.request_netlist_manual_deck_run();
                    }
                } else if self.state.can_run_simulation() {
                    self.state.request_run_set_simulation();
                    self.state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Simulate);
                }
            }
            ShortcutCommand::StopSimulation => {
                if self.state.simulation.is_running
                    && crate::simulation::execution::execution_target_supports_cancellation()
                    && let Err(error) = self.state.simulation.request_abort_active_run()
                {
                    self.state.push_sim_message(ConsoleMessage::warning(error));
                }
            }
            ShortcutCommand::RunChecks => {
                crate::common::menu_bar::run_design_rule_check(&mut self.state);
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
            ShortcutCommand::ZoomIn => {
                self.state.schematic.zoom = (self.state.schematic.zoom * 1.25).min(8.0);
            }
            ShortcutCommand::ZoomOut => {
                self.state.schematic.zoom = (self.state.schematic.zoom / 1.25).max(0.1);
            }
            ShortcutCommand::ToggleLinkedCursors => {
                self.state.ui.results.toggle_linked_cursors();
            }
            ShortcutCommand::ZoomFit => {
                self.state.schematic.needs_fit = true;
            }
            ShortcutCommand::ZoomOneToOne => {
                self.state.schematic.zoom = 1.0;
            }
            ShortcutCommand::PlaceLabel => {
                crate::workbench::commands::arm_schematic_tool(
                    &mut self.state.schematic,
                    Tool::Label,
                );
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

    fn execute_project_file_shortcut(&mut self, action: crate::common::menu_bar::FileMenuAction) {
        crate::common::menu_bar::dispatch_file_menu_action(
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
                match self.state.undo_active_symbol_document() {
                    Ok(true) => self
                        .state
                        .push_user_message(ConsoleMessage::info("Undo: symbol edit")),
                    Ok(false) => self
                        .state
                        .push_user_message(ConsoleMessage::info("Nothing to undo")),
                    Err(error) => self.state.push_user_message(ConsoleMessage::warning(error)),
                }
                true
            }
            ShortcutCommand::Redo => {
                match self.state.redo_active_symbol_document() {
                    Ok(true) => self
                        .state
                        .push_user_message(ConsoleMessage::info("Redo: symbol edit")),
                    Ok(false) => self
                        .state
                        .push_user_message(ConsoleMessage::info("Nothing to redo")),
                    Err(error) => self.state.push_user_message(ConsoleMessage::warning(error)),
                }
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
                self.state.ui.symbol.tool = SymbolTool::Polyline;
                self.state.ui.symbol.pending_polyline.clear();
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
            ShortcutCommand::SymbolArrowTool => {
                self.state.ui.symbol.tool = SymbolTool::Arrow;
                true
            }
            ShortcutCommand::SymbolDotTool => {
                self.state.ui.symbol.tool = SymbolTool::Dot;
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
            ShortcutCommand::ZoomIn => {
                self.state.ui.symbol.zoom = (self.state.ui.symbol.zoom * 1.25).min(16.0);
                true
            }
            ShortcutCommand::ZoomOut => {
                self.state.ui.symbol.zoom = (self.state.ui.symbol.zoom / 1.25).max(0.1);
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
            | ShortcutCommand::PlaceLabel
            | ShortcutCommand::PlaceInstance
            | ShortcutCommand::DescendHierarchy
            | ShortcutCommand::AscendHierarchy => true,
            _ => false,
        }
    }

    fn rotate_schematic_selection_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        self.state
            .schematic
            .rotate_selection_resolved(|component| symbol_context.terminal_points(component));
    }

    fn mirror_schematic_selection_h_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        self.state
            .schematic
            .mirror_selection_h_resolved(|component| symbol_context.terminal_points(component));
    }

    fn mirror_schematic_selection_v_with_symbols(&mut self) {
        let symbol_context = SchematicSymbolContext::from_state(&self.state);
        self.state
            .schematic
            .mirror_selection_v_resolved(|component| symbol_context.terminal_points(component));
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
        let ports = self.state.active_symbol_ports();
        self.state.ui.symbol.clipboard =
            symbol_clipboard_from_selection(&document, &selection, &ports);
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
            selection.pins.insert(pin.name.clone());
            document.pins.push(pin);
        }
        self.state.ui.symbol.set_selection(selection);
        if let Err(error) = self.state.store_active_symbol_document(&document) {
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
        let ports = self.state.active_symbol_ports();
        let mut document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
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
        for mut pin in std::mem::take(&mut document.pins) {
            if selection.pins.contains(&pin.name) {
                if symbol_pin_is_contract(&ports, &pin.name) {
                    if pin.position.is_some() {
                        pin.position = None;
                        changed = true;
                    }
                    retained.push(pin);
                } else {
                    if cut {
                        clipboard.pins.push(pin);
                    }
                    changed = true;
                }
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
            if let Err(error) = self.state.store_active_symbol_document(&document) {
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
                pin.position = Some(pin_transform(position, origin));
                changed = true;
            }
        }
        if changed {
            self.state.record_symbol_edit(&before);
            if let Err(error) = self.state.store_active_symbol_document(&document) {
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

    pub(super) fn action_edit_undo(&mut self) {
        if self.state.schematic.can_undo() {
            let desc = self
                .state
                .schematic
                .undo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.undo() {
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Undo: {}", desc)));
            }
        } else {
            self.state
                .push_user_message(ConsoleMessage::info("Nothing to undo"));
        }
    }

    pub(super) fn action_edit_redo(&mut self) {
        if self.state.schematic.can_redo() {
            let desc = self
                .state
                .schematic
                .redo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.redo() {
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Redo: {}", desc)));
            }
        } else {
            self.state
                .push_user_message(ConsoleMessage::info("Nothing to redo"));
        }
    }

    pub(super) fn action_edit_copy(&mut self) {
        self.state.schematic.copy_selection();
    }

    pub(super) fn action_edit_paste(&mut self) {
        let anchor = self.state.schematic_paste_anchor();
        if !self.state.schematic.paste_at(anchor) {
            self.state.push_user_message(ConsoleMessage::warning(
                "Paste could not be completed at the current canvas target".to_owned(),
            ));
        }
    }

    pub(super) fn action_edit_cut(&mut self) {
        self.state.schematic.copy_selection();
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_delete(&mut self) {
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_select_all(&mut self) {
        self.state.schematic.select_all_objects();
    }
}

/// Commands that mutate the open schematic — refused on read-only views.
/// Copy and Select All are reads; tools that only inspect (select, probe)
/// and navigation stay live.
fn command_edits_schematic(command: ShortcutCommand) -> bool {
    matches!(
        command,
        ShortcutCommand::Undo
            | ShortcutCommand::Redo
            | ShortcutCommand::Paste
            | ShortcutCommand::Cut
            | ShortcutCommand::Delete
            | ShortcutCommand::PlaceWire
            | ShortcutCommand::PlaceBus
            | ShortcutCommand::PlaceBusTap
            | ShortcutCommand::PlaceJunction
            | ShortcutCommand::PlaceLabel
            | ShortcutCommand::Place(_)
            | ShortcutCommand::RotateSelection
            | ShortcutCommand::MirrorSelectionHorizontal
            | ShortcutCommand::MirrorSelectionVertical
            | ShortcutCommand::ObjectProperties
    )
}

#[cfg(test)]
mod shortcut_ownership_tests {
    use super::*;

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
            Some(super::super::ObjectPropertiesDraft::Bus(_))
        ));
        let Some(super::super::ObjectPropertiesDraft::Bus(draft)) =
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
        use crate::workbench::commands::CommandPlatform;

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
        app.state.dialogs.drc_results = Some(result);
        app.state.dialogs.drc_checked_version = app.state.schematic.topology_version();
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
    use crate::state::{
        Cell, CellViewRef, Library, PortDirection, PortSpec, SymbolPin, View, ViewType,
    };

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
            ShortcutCommand::SymbolCircleTool,
            ShortcutCommand::SymbolArcTool,
            ShortcutCommand::SymbolArrowTool,
            ShortcutCommand::SymbolDotTool,
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
        assert_eq!(
            app.state.ui.symbol.tool,
            crate::workbench::SymbolTool::Polyline
        );
        assert!(app.state.ui.symbol.pending_polyline.is_empty());

        for (command, expected) in [
            (
                ShortcutCommand::SymbolCircleTool,
                crate::workbench::SymbolTool::Circle,
            ),
            (
                ShortcutCommand::SymbolArcTool,
                crate::workbench::SymbolTool::Arc,
            ),
            (
                ShortcutCommand::SymbolArrowTool,
                crate::workbench::SymbolTool::Arrow,
            ),
            (
                ShortcutCommand::SymbolDotTool,
                crate::workbench::SymbolTool::Dot,
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
                ShortcutCommand::SymbolCircleTool | ShortcutCommand::SymbolArcTool
            ) {
                assert!(app.state.ui.symbol.shape_start.is_none());
            }
        }

        app.state.ui.symbol.needs_fit = false;
        app.execute_shortcut_command(ShortcutCommand::ZoomFit);
        assert!(app.state.ui.symbol.needs_fit);
    }

    #[test]
    fn symbol_clipboard_copies_shapes_and_non_contract_pins_only() {
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
        let ports = vec![PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        }];
        let mut selection = SymbolSelection::default();
        selection.pins.insert("IN".to_owned());
        selection.pins.insert("TRIM".to_owned());
        selection.shapes.insert(0);

        let clipboard = symbol_clipboard_from_selection(&document, &selection, &ports);

        assert_eq!(clipboard.shapes.len(), 1);
        assert_eq!(
            clipboard
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>(),
            vec!["TRIM"]
        );
    }
}
