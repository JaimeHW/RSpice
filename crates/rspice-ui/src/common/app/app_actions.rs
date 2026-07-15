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
    app_shortcuts::{ShortcutInputSnapshot, resolve_shortcuts},
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

fn symbol_shortcut_command_allowed(
    command: ShortcutCommand,
    snapshot: &ShortcutInputSnapshot,
) -> bool {
    if matches!(
        command,
        ShortcutCommand::SelectTool
            | ShortcutCommand::PlaceWire
            | ShortcutCommand::PlaceProbe
            | ShortcutCommand::Place(crate::state::ComponentType::Capacitor)
            | ShortcutCommand::Place(crate::state::ComponentType::Diode)
            | ShortcutCommand::Place(crate::state::ComponentType::Ground)
            | ShortcutCommand::RotateSelection
            | ShortcutCommand::MirrorSelectionHorizontal
            | ShortcutCommand::MirrorSelectionVertical
            | ShortcutCommand::ZoomFit
    ) {
        return snapshot.plain();
    }
    true
}

impl RSpiceApp {
    /// Handle keyboard shortcuts
    pub(super) fn handle_shortcuts(&mut self, ctx: &Context) {
        if shortcut_dispatch_blocked(&self.state, ctx) {
            return;
        }
        let has_focus = ctx.memory(|memory| memory.focused().is_some());
        let snapshot = ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, has_focus));
        for resolved in resolve_shortcuts(
            &snapshot,
            crate::workbench::commands::current_command_platform(),
        ) {
            if !resolved.command.shortcut_context_matches(self) {
                continue;
            }
            let consumed =
                ctx.input_mut(|input| input.consume_key(snapshot.modifiers(), resolved.key));
            if !consumed {
                continue;
            }
            let command = resolved.command;
            if !command.is_enabled(self) {
                continue;
            }
            if self.state.workspace.active_view_type() == crate::state::ViewType::Symbol
                && !symbol_shortcut_command_allowed(command, &snapshot)
            {
                continue;
            }
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
                self.execute_project_file_shortcut(crate::common::menu_bar::FileMenuAction::Save)
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
                self.state.schematic.tool = Tool::Select;
            }
            ShortcutCommand::PlaceWire => {
                self.state.schematic.tool = Tool::Wire;
            }
            ShortcutCommand::Place(ComponentType::Ground) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Ground);
            }
            ShortcutCommand::Place(ComponentType::VoltageSource) => {
                self.state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
            }
            ShortcutCommand::Place(ComponentType::CurrentSource) => {
                self.state.schematic.tool = Tool::Place(ComponentType::CurrentSource);
            }
            ShortcutCommand::Place(ComponentType::Capacitor) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Capacitor);
            }
            ShortcutCommand::Place(ComponentType::Inductor) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Inductor);
            }
            ShortcutCommand::Place(ComponentType::Diode) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Diode);
            }
            ShortcutCommand::Place(ComponentType::Nmos) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Nmos);
            }
            ShortcutCommand::Place(ComponentType::NpnBjt) => {
                self.state.schematic.tool = Tool::Place(ComponentType::NpnBjt);
            }
            ShortcutCommand::PlaceProbe => {
                self.state.schematic.tool = Tool::Probe;
            }
            ShortcutCommand::Place(ComponentType::Resistor) => {
                self.state.schematic.tool = Tool::Place(ComponentType::Resistor);
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
                if let Some(comp_id) = self.state.schematic.selection.single_component() {
                    super::open_property_editor(&mut self.state, comp_id);
                }
            }
            ShortcutCommand::Cancel => {
                if self.state.workbench.drawer.is_some() {
                    self.state.workbench.close_drawer();
                } else if self.state.tabbed_property_dialog.open {
                    self.state.tabbed_property_dialog.close();
                } else if self.state.workbench.workspace
                    == crate::workbench::state::Workspace::Results
                    && self.state.ui.results.cursors.any()
                {
                    self.state.ui.results.clear_cursors();
                } else {
                    self.state.schematic.tool = Tool::Select;
                    self.state.schematic.cancel_wire();
                    self.state.schematic.selection.clear();
                    self.state.schematic.selection_rect.cancel();
                }
            }
            ShortcutCommand::RunSimulation => {
                if self.state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
                    if self.state.manual_deck_run_block_reason().is_none() {
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
                {
                    self.state.simulation.trigger_abort = true;
                }
            }
            ShortcutCommand::RunChecks => {
                crate::common::menu_bar::run_design_rule_check(&mut self.state);
            }
            ShortcutCommand::NextViolation => {
                self.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
                crate::schematic::view::violations::cycle_violation(&mut self.state, 1);
            }
            ShortcutCommand::PreviousViolation => {
                self.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
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
            ShortcutCommand::ZoomFit => {
                self.state.schematic.needs_fit = true;
            }
            ShortcutCommand::ZoomOneToOne => {
                self.state.schematic.zoom = 1.0;
            }
            ShortcutCommand::PlaceLabel => {
                self.state.schematic.tool = Tool::Label;
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
                self.state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Design);
                self.state.workbench.navigator_visible = true;
                self.state.workbench.focus_navigator_search = true;
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
            ShortcutCommand::PlaceProbe => {
                self.state.ui.symbol.tool = SymbolTool::PlacePin;
                self.state.ui.symbol.clear_selection();
                true
            }
            ShortcutCommand::PlaceWire => {
                self.state.ui.symbol.tool = SymbolTool::Polyline;
                self.state.ui.symbol.pending_polyline.clear();
                true
            }
            ShortcutCommand::Place(crate::state::ComponentType::Capacitor) => {
                self.state.ui.symbol.tool = SymbolTool::Circle;
                self.state.ui.symbol.shape_start = None;
                true
            }
            ShortcutCommand::Place(crate::state::ComponentType::Diode) => {
                self.state.ui.symbol.tool = SymbolTool::Arrow;
                true
            }
            ShortcutCommand::Place(crate::state::ComponentType::Ground) => {
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
            ShortcutCommand::Place(crate::state::ComponentType::VoltageSource)
            | ShortcutCommand::Place(crate::state::ComponentType::CurrentSource)
            | ShortcutCommand::Place(crate::state::ComponentType::Inductor)
            | ShortcutCommand::Place(crate::state::ComponentType::Nmos)
            | ShortcutCommand::Place(crate::state::ComponentType::NpnBjt)
            | ShortcutCommand::Place(crate::state::ComponentType::Resistor)
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
        self.state.schematic.paste_at(anchor);
    }

    pub(super) fn action_edit_cut(&mut self) {
        self.state.schematic.copy_selection();
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_delete(&mut self) {
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_select_all(&mut self) {
        let schematic = &mut self.state.schematic;
        schematic.selection.clear();
        schematic.selection.components = schematic.components.iter().map(|c| c.id).collect();
        schematic.selection.wires = schematic.wires.iter().map(|w| w.id).collect();
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
}

#[cfg(test)]
mod symbol_action_tests {
    use super::*;
    use crate::state::{PortDirection, PortSpec, SymbolPin};

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

    #[test]
    fn modified_app_symbol_tool_shortcuts_are_rejected() {
        assert!(symbol_shortcut_command_allowed(
            ShortcutCommand::SelectTool,
            &ShortcutInputSnapshot::from_modifiers_for_test(egui::Modifiers::NONE)
        ));

        for modifiers in [
            egui::Modifiers {
                alt: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                command: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                ctrl: true,
                ..egui::Modifiers::NONE
            },
            egui::Modifiers {
                shift: true,
                ..egui::Modifiers::NONE
            },
        ] {
            assert!(!symbol_shortcut_command_allowed(
                ShortcutCommand::SelectTool,
                &ShortcutInputSnapshot::from_modifiers_for_test(modifiers)
            ));
            assert!(!symbol_shortcut_command_allowed(
                ShortcutCommand::Place(crate::state::ComponentType::Capacitor),
                &ShortcutInputSnapshot::from_modifiers_for_test(modifiers)
            ));
        }

        assert!(symbol_shortcut_command_allowed(
            ShortcutCommand::Copy,
            &ShortcutInputSnapshot::from_modifiers_for_test(egui::Modifiers {
                ctrl: true,
                command: true,
                ..egui::Modifiers::NONE
            })
        ));
    }
}
