use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};
use crate::panels::{LogAnchor, LogSeverity, LogSource};
use crate::services::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::shell::SymbolDocumentSnapshot;
use crate::state::{
    CellViewRef, Component, ComponentType, PinFindingKind, Point, PortDirection, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SchematicState, SymbolDocument, View, ViewType,
};
use std::collections::HashMap;

const MAX_FINDING_ROWS: usize = 50;

fn view_type_for_reference(state: &AppState, reference: &CellViewRef) -> ViewType {
    state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
        .unwrap_or(ViewType::Schematic)
}

fn is_schematic_like(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

fn symbol_pin_position_remaps(
    before: &SymbolDocument,
    after: &SymbolDocument,
) -> HashMap<String, (Point, Point)> {
    let mut remaps = HashMap::new();
    for before_pin in &before.pins {
        let Some(old_position) = before_pin.position else {
            continue;
        };
        let Some(after_pin) = after.pin(&before_pin.name) else {
            continue;
        };
        let Some(new_position) = after_pin.position else {
            continue;
        };
        if old_position != new_position {
            remaps.insert(
                before_pin.name.to_ascii_lowercase(),
                (old_position, new_position),
            );
        }
    }
    remaps
}

fn remap_symbol_instance_wires(
    schematic: &mut SchematicState,
    reference: &CellViewRef,
    pin_remaps: &HashMap<String, (Point, Point)>,
) -> bool {
    if pin_remaps.is_empty() {
        return false;
    }

    let mut world_remaps = Vec::new();
    for component in &schematic.components {
        append_component_symbol_remaps(component, reference, pin_remaps, &mut world_remaps);
    }
    if world_remaps.is_empty() {
        return false;
    }

    let mut updates: Vec<(usize, usize, Point)> = Vec::new();
    for (wire_index, wire) in schematic.wires.iter().enumerate() {
        for (point_index, point) in wire.points.iter().enumerate() {
            if let Some((_, new_position)) = world_remaps
                .iter()
                .find(|(old_position, _)| point == old_position)
            {
                updates.push((wire_index, point_index, *new_position));
            }
        }
    }
    if updates.is_empty() {
        return false;
    }

    for (wire_index, point_index, new_position) in updates {
        if let Some(wire) = schematic.wires.get_mut(wire_index)
            && point_index < wire.points.len()
        {
            wire.points[point_index] = new_position;
        }
    }
    schematic.is_dirty = true;
    schematic.bump_topology_version();
    true
}

fn append_component_symbol_remaps(
    component: &Component,
    reference: &CellViewRef,
    pin_remaps: &HashMap<String, (Point, Point)>,
    world_remaps: &mut Vec<(Point, Point)>,
) {
    let Some(binding) = component.library_cell.as_ref() else {
        return;
    };
    if !binding.library.eq_ignore_ascii_case(&reference.library)
        || !binding.cell.eq_ignore_ascii_case(&reference.cell)
    {
        return;
    }

    if binding.terminal_order.is_empty() {
        for &(old_offset, new_offset) in pin_remaps.values() {
            push_world_pin_remap(component, old_offset, new_offset, world_remaps);
        }
        return;
    }

    for terminal_name in &binding.terminal_order {
        let Some(&(old_offset, new_offset)) = pin_remaps.get(&terminal_name.to_ascii_lowercase())
        else {
            continue;
        };
        push_world_pin_remap(component, old_offset, new_offset, world_remaps);
    }
}

fn push_world_pin_remap(
    component: &Component,
    old_offset: Point,
    new_offset: Point,
    world_remaps: &mut Vec<(Point, Point)>,
) {
    let old_position = component.pos + component.transform_point(old_offset);
    let new_position = component.pos + component.transform_point(new_offset);
    if old_position != new_position {
        world_remaps.push((old_position, new_position));
    }
}

fn schematic_for_workspace(state: &mut AppState, reference: &CellViewRef) -> SchematicState {
    let mut schematic = state
        .workspace
        .schematic_buffers
        .get(&reference.key())
        .cloned()
        .unwrap_or_default();
    // Workspace buffers round-trip through serde, which skips the runtime
    // ID counter and name counters: without recalculation a freshly placed
    // component reuses an existing ID and selection matches both.
    schematic.recalculate_runtime_state();
    // Views from read-only libraries open for inspection, never for edit —
    // the docbar banner explains and every edit path checks this flag.
    schematic.read_only = state
        .library_manager
        .get_library(&reference.library)
        .is_some_and(|library| library.read_only);
    schematic
}

fn log_severity_from_drc(severity: DrcSeverity) -> LogSeverity {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => LogSeverity::Error,
        DrcSeverity::Warning => LogSeverity::Warning,
        DrcSeverity::Info => LogSeverity::Info,
    }
}

fn symbol_snapshot_from_view(
    view: &View,
    fallback: &SymbolDocument,
    preserve_missing_metadata: bool,
) -> SymbolDocumentSnapshot {
    let symbol_document_metadata = match view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY) {
        Some(encoded) => Some(encoded.clone()),
        None if preserve_missing_metadata => None,
        None => serde_json::to_string(fallback).ok(),
    };
    SymbolDocumentSnapshot {
        document: fallback.clone(),
        symbol_document_metadata,
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
    }
}

fn symbol_metadata_snapshot_from_view(
    view: &View,
    fallback: &SymbolDocument,
) -> SymbolDocumentSnapshot {
    SymbolDocumentSnapshot {
        document: fallback.clone(),
        symbol_document_metadata: view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY).cloned(),
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
    }
}

fn restore_symbol_snapshot_in_view(view: &mut View, snapshot: &SymbolDocumentSnapshot) {
    match &snapshot.symbol_document_metadata {
        Some(encoded) => {
            view.metadata
                .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove(SYMBOL_DOCUMENT_METADATA_KEY);
        }
    }
    match &snapshot.generated_metadata {
        Some(encoded) => {
            view.metadata
                .insert("generated".to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove("generated");
        }
    }
    match &snapshot.ports_metadata {
        Some(encoded) => {
            view.metadata.insert("ports".to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove("ports");
        }
    }
}

impl AppState {
    pub(crate) fn active_view_read_only(&self) -> bool {
        self.library_manager
            .get_library(&self.workspace.active_view.library)
            .is_some_and(|library| library.read_only)
    }

    pub(crate) fn read_only_master_message(&self) -> String {
        let library = &self.workspace.active_view.library;
        format!("Read-only - '{library}' masters cannot be edited")
    }

    pub(crate) fn active_symbol_ports(&self) -> Vec<PortSpec> {
        let reference = &self.workspace.active_view;
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.active_view == schematic_ref {
            return self.schematic.interface_ports();
        }
        if let Some(schematic) = self.workspace.schematic_buffers.get(&schematic_ref.key()) {
            let ports = schematic.interface_ports();
            if !ports.is_empty() {
                return ports;
            }
        }

        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| view.metadata.get("ports"))
            .map(|encoded| parse_encoded_ports(encoded))
            .unwrap_or_default()
    }

    pub(crate) fn load_active_symbol_document(&self) -> Result<SymbolDocument, String> {
        let reference = &self.workspace.active_view;
        let Some(view) = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        let has_document = view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY);
        let document = SymbolDocument::load_from_view(view)?;
        if has_document {
            return Ok(document);
        }
        let ports = self.active_symbol_ports();
        if view.metadata.contains_key("generated") && !ports.is_empty() {
            return Ok(SymbolDocument::generated_from_ports(&ports));
        }
        Ok(document)
    }

    fn active_symbol_snapshot(&self, fallback: &SymbolDocument) -> SymbolDocumentSnapshot {
        let reference = &self.workspace.active_view;
        let current_document = self.load_active_symbol_document().ok();
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| {
                let preserve_missing_metadata =
                    !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY)
                        && current_document
                            .as_ref()
                            .is_some_and(|document| document == fallback);
                symbol_snapshot_from_view(view, fallback, preserve_missing_metadata)
            })
            .unwrap_or_else(|| SymbolDocumentSnapshot::from_document(fallback))
    }

    fn active_symbol_metadata_snapshot(&self, fallback: &SymbolDocument) -> SymbolDocumentSnapshot {
        let reference = &self.workspace.active_view;
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| symbol_metadata_snapshot_from_view(view, fallback))
            .unwrap_or_else(|| SymbolDocumentSnapshot::from_document(fallback))
    }

    fn restore_active_symbol_snapshot(
        &mut self,
        snapshot: &SymbolDocumentSnapshot,
    ) -> Result<(), String> {
        let reference = self.workspace.active_view.clone();
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let previous_document = self.load_active_symbol_document().ok();
        let Some(view) = self
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        restore_symbol_snapshot_in_view(view, snapshot);
        if let Some(previous_document) = previous_document {
            let pin_remaps = symbol_pin_position_remaps(&previous_document, &snapshot.document);
            for schematic in self.workspace.schematic_buffers.values_mut() {
                remap_symbol_instance_wires(schematic, &reference, &pin_remaps);
            }
        }
        self.workspace.set_active_dirty(true);
        Ok(())
    }

    pub(crate) fn store_active_symbol_document(
        &mut self,
        document: &SymbolDocument,
    ) -> Result<(), String> {
        let reference = self.workspace.active_view.clone();
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let previous_document = self.load_active_symbol_document().ok();
        let Some(view) = self
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        document.store_in_view(view)?;
        view.metadata.remove("generated");
        view.metadata.remove("ports");
        if let Some(previous_document) = previous_document {
            let pin_remaps = symbol_pin_position_remaps(&previous_document, document);
            for schematic in self.workspace.schematic_buffers.values_mut() {
                remap_symbol_instance_wires(schematic, &reference, &pin_remaps);
            }
        }
        self.workspace.set_active_dirty(true);
        Ok(())
    }

    pub(crate) fn record_symbol_edit(&mut self, before: &SymbolDocument) {
        const MAX_SYMBOL_UNDO: usize = 128;
        let key = self.workspace.active_key();
        let snapshot = self.active_symbol_snapshot(before);
        self.push_symbol_undo_snapshot(key, snapshot, MAX_SYMBOL_UNDO);
    }

    fn record_symbol_metadata_edit(&mut self, before: &SymbolDocument) {
        const MAX_SYMBOL_UNDO: usize = 128;
        let key = self.workspace.active_key();
        let snapshot = self.active_symbol_metadata_snapshot(before);
        self.push_symbol_undo_snapshot(key, snapshot, MAX_SYMBOL_UNDO);
    }

    fn push_symbol_undo_snapshot(
        &mut self,
        key: String,
        snapshot: SymbolDocumentSnapshot,
        max_len: usize,
    ) {
        let undo_stack = self
            .shell
            .symbol
            .undo_stacks
            .entry(key.clone())
            .or_default();
        undo_stack.push(snapshot);
        if undo_stack.len() > max_len {
            undo_stack.remove(0);
        }
        self.shell.symbol.redo_stacks.remove(&key);
    }

    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        self.shell
            .symbol
            .undo_stacks
            .get(&self.workspace.active_key())
            .is_some_and(|stack| !stack.is_empty())
    }

    pub(crate) fn can_redo_active_symbol_document(&self) -> bool {
        self.shell
            .symbol
            .redo_stacks
            .get(&self.workspace.active_key())
            .is_some_and(|stack| !stack.is_empty())
    }

    pub(crate) fn undo_active_symbol_document(&mut self) -> Result<bool, String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let key = self.workspace.active_key();
        let Some(previous) = self
            .shell
            .symbol
            .undo_stacks
            .get_mut(&key)
            .and_then(Vec::pop)
        else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let current_snapshot = self.active_symbol_metadata_snapshot(&current);
        self.shell
            .symbol
            .redo_stacks
            .entry(key)
            .or_default()
            .push(current_snapshot);
        self.restore_active_symbol_snapshot(&previous)?;
        Ok(true)
    }

    pub(crate) fn redo_active_symbol_document(&mut self) -> Result<bool, String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let key = self.workspace.active_key();
        let Some(next) = self
            .shell
            .symbol
            .redo_stacks
            .get_mut(&key)
            .and_then(Vec::pop)
        else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let current_snapshot = self.active_symbol_metadata_snapshot(&current);
        self.shell
            .symbol
            .undo_stacks
            .entry(key)
            .or_default()
            .push(current_snapshot);
        self.restore_active_symbol_snapshot(&next)?;
        Ok(true)
    }

    pub(crate) fn run_active_symbol_pin_checks(&mut self) {
        let ports = self.active_symbol_ports();
        let reference = self.workspace.active_view.clone();
        match self.load_active_symbol_document() {
            Ok(document) => {
                let findings = document.pin_findings(&ports);
                let mut result = DrcResult::new();
                result.completed = true;

                for (index, finding) in findings.iter().enumerate() {
                    let violation_type = match finding.kind {
                        PinFindingKind::UnplacedPin => DrcViolationType::SymbolUnplacedPin,
                        PinFindingKind::OrphanedPin => DrcViolationType::SymbolOrphanedPin,
                        PinFindingKind::PinOffGrid => DrcViolationType::SymbolPinOffGrid,
                    };
                    let point = document.pin(&finding.pin_name).and_then(|pin| pin.position);
                    let message = format!("{}: {}", violation_type.description(), finding.pin_name);
                    result.add_violation(DrcViolation::new(
                        index + 1,
                        violation_type,
                        message,
                        DrcLocation::SymbolPin {
                            reference: reference.clone(),
                            pin_name: finding.pin_name.clone(),
                            point,
                        },
                    ));
                }

                if findings.is_empty() {
                    self.dialogs.drc_results = Some(result);
                    self.dialogs.drc_checked_version = self.schematic.topology_version();
                    self.dialogs.drc_cycle = None;
                    self.push_user_message(ConsoleMessage::info("Symbol pins match schematic"));
                    return;
                }

                for violation in result.violations().iter().take(MAX_FINDING_ROWS) {
                    let anchor =
                        crate::schematic::view::violations::finding_anchor(self, violation);
                    self.log_buffer.log_anchored(
                        log_severity_from_drc(violation.severity),
                        LogSource::Drc,
                        violation.message.clone(),
                        None,
                        anchor,
                    );
                }
                let hidden = result.violations().len().saturating_sub(MAX_FINDING_ROWS);
                if hidden > 0 {
                    self.log_buffer.log(
                        LogSeverity::Info,
                        LogSource::Drc,
                        format!("+{hidden} more findings - F4 cycles through them"),
                        None,
                    );
                }
                self.push_user_message(ConsoleMessage::error(format!(
                    "Symbol check found {} issue(s)",
                    result.total_count()
                )));
                self.dialogs.drc_results = Some(result);
                self.dialogs.drc_checked_version = self.schematic.topology_version();
                self.dialogs.drc_cycle = None;
            }
            Err(error) => self.push_user_message(ConsoleMessage::warning(error)),
        }
    }

    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        match anchor {
            LogAnchor::Schematic {
                x,
                y,
                component,
                wire,
            } => {
                self.shell.view = crate::shell::WorkspaceView::Schematic;
                self.schematic.center_request = Some(Point::new(x, y));
                self.schematic.net_highlight.clear();
                self.schematic.selection.clear();
                if let Some(id) = component {
                    self.schematic.selection.select_component(id);
                }
                if let Some(id) = wire {
                    self.schematic.selection.select_wire(id);
                }
            }
            LogAnchor::Symbol {
                reference,
                pin_name,
                point,
            } => {
                self.open_workspace_view(reference);
                self.shell.view = crate::shell::WorkspaceView::Schematic;
                self.shell.symbol.select_pin(pin_name);
                if let Some(point) = point {
                    let zoom = self.shell.symbol.zoom.max(1.0);
                    self.shell.symbol.pan = (-(point.x as f32) * zoom, -(point.y as f32) * zoom);
                }
                self.shell.symbol.needs_fit = false;
            }
        }
    }

    pub(crate) fn should_save_project_for_active_document(&self) -> bool {
        self.workspace.active_view_type() == ViewType::Symbol
            || self
                .workspace
                .open_views
                .iter()
                .any(|open| open.view_type == ViewType::Symbol && open.dirty)
    }

    pub(crate) fn generate_active_symbol_document(&mut self) -> Result<(), String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let before = self.load_active_symbol_document()?;
        let ports = self.active_symbol_ports();
        let document = SymbolDocument::generated_from_ports(&ports);
        if document != before {
            self.record_symbol_metadata_edit(&before);
        }
        self.store_active_symbol_document(&document)?;
        Ok(())
    }

    /// Migrate persisted sessions away from the legacy seeded "primitives"
    /// library: any workspace tab or schematic buffer pointing into it is
    /// moved to the user library (preserving drawn content), then the
    /// placeholder library is dropped. Without the workspace scrub,
    /// `ensure_library_model` would faithfully re-create the legacy cell
    /// the active tab points at on every launch.
    pub(crate) fn migrate_legacy_primitives(&mut self) {
        const LEGACY: &str = crate::state::LibraryManager::PRIMITIVES_LIBRARY;
        let user = crate::state::LibraryManager::USER_LIBRARY;
        let legacy_prefix = format!("{LEGACY}/");

        // Move schematic buffers (the actual drawn content) to user keys.
        let legacy_keys: Vec<String> = self
            .workspace
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&legacy_prefix))
            .cloned()
            .collect();
        for key in legacy_keys {
            let Some(buffer) = self.workspace.schematic_buffers.remove(&key) else {
                continue;
            };
            let tail = &key[legacy_prefix.len()..];
            let new_key = format!("{user}/{tail}");
            self.workspace
                .schematic_buffers
                .entry(new_key)
                .or_insert(buffer);

            // Make the migrated cell/view exist in the user library so the
            // browser lists it and tabs resolve.
            if let Some((cell_name, view_name)) = tail.split_once('/')
                && let Some(library) = self.library_manager.get_library_mut(user)
            {
                if library.get_cell(cell_name).is_none() {
                    library.add_cell(crate::state::Cell::new(cell_name));
                }
                if let Some(cell) = library.get_cell_mut(cell_name)
                    && cell.get_view(view_name).is_none()
                {
                    cell.add_view(crate::state::View::new(
                        view_name,
                        crate::state::ViewType::Schematic,
                    ));
                }
            }
        }

        // Repoint tabs, breadcrumbs, and the active view.
        let remap = |reference: &mut CellViewRef| {
            if reference.library == LEGACY {
                reference.library = user.to_string();
            }
        };
        remap(&mut self.workspace.active_view);
        for open in &mut self.workspace.open_views {
            remap(&mut open.reference);
        }
        for entry in &mut self.workspace.hierarchy_stack {
            remap(entry);
        }

        self.library_manager.purge_legacy_primitives();
    }

    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        if is_schematic_like(self.workspace.active_view_type()) {
            self.workspace.save_active_schematic(&self.schematic);
            self.sync_generated_symbol_view();
        }
    }

    /// Keep the active cell's generated "symbol" view in step with its
    /// schematic interface — created when ports exist, refreshed when they
    /// change, removed when the last port goes. A hand-authored symbol
    /// view (no `generated` marker) is never touched: the user owns it.
    fn sync_generated_symbol_view(&mut self) {
        const GENERATED_KEY: &str = "generated";
        const PORTS_KEY: &str = "ports";

        let reference = self.workspace.active_view.clone();
        if !reference.view.eq_ignore_ascii_case("schematic") {
            return;
        }
        let ports = self.schematic.interface_ports();
        let Some(cell) = self
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
        else {
            return;
        };

        if ports.is_empty() {
            if cell
                .get_view("symbol")
                .is_some_and(|view| view.metadata.contains_key(GENERATED_KEY))
            {
                cell.remove_view("symbol");
            }
            return;
        }

        let encoded: String = ports
            .iter()
            .map(|port| format!("{}:{}", port.name, port.direction.keyword()))
            .collect::<Vec<_>>()
            .join(" ");
        match cell.get_view_mut("symbol") {
            Some(view) if view.metadata.contains_key(GENERATED_KEY) => {
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
            }
            Some(_) => {} // hand-authored symbol: leave it alone
            None => {
                let mut view = View::new("symbol", ViewType::Symbol);
                view.metadata
                    .insert(GENERATED_KEY.to_owned(), "ports".to_owned());
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
                cell.add_view(view);
            }
        }
    }

    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        self.workspace
            .ensure_library_model(&mut self.library_manager);
        let reference = self.workspace.active_view.clone();
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
    }

    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.sync_active_schematic_to_workspace();
        if self.workspace.active_view == reference {
            return;
        }
        self.shell.canvas_hover = None;
        self.shell.canvas_view_center = None;
        let view_type = view_type_for_reference(self, &reference);
        self.workspace.open_as_root(reference.clone(), view_type);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.push_user_message(ConsoleMessage::info(format!(
            "Opened {}",
            reference.display_path()
        )));
    }

    /// Descend into a hierarchical instance: open its master and record
    /// the instance name on the occurrence path. `None` (no instance
    /// context) labels the level with the cell name.
    pub(crate) fn descend_into_instance(
        &mut self,
        instance: Option<String>,
        reference: CellViewRef,
    ) {
        self.sync_active_schematic_to_workspace();
        let view_type = view_type_for_reference(self, &reference);
        match instance {
            Some(name) => self
                .workspace
                .descend_into(name, reference.clone(), view_type),
            None => self.workspace.enter_hierarchy(reference.clone(), view_type),
        }
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.push_user_message(ConsoleMessage::info(format!(
            "Entered {}",
            reference.display_path()
        )));
    }

    /// Copy a whole cell — every view and its drawn content — into a
    /// writable library under a new name. Returns the number of views
    /// copied, or the user-facing error.
    pub(crate) fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let source = self
            .library_manager
            .get_library(src_library)
            .ok_or_else(|| format!("Library '{src_library}' not found"))?;
        let mut copy = source
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{src_library}'"))?
            .clone();
        let destination = self
            .library_manager
            .get_library(dst_library)
            .ok_or_else(|| format!("Library '{dst_library}' not found"))?;
        if destination.read_only {
            return Err(format!("Library '{dst_library}' is read-only"));
        }
        if destination.get_cell(new_name).is_some() {
            return Err(format!(
                "Cell '{new_name}' already exists in library '{dst_library}'"
            ));
        }

        copy.name = new_name.to_owned();
        let view_names: Vec<String> = copy.views.keys().cloned().collect();
        let view_count = view_names.len();
        if let Some(destination) = self.library_manager.get_library_mut(dst_library) {
            destination.add_cell(copy);
        }

        // The drawn content lives in the workspace buffers, keyed
        // "library/cell/view" — a copy without it would be a lie.
        for view in view_names {
            let old_key = CellViewRef::new(src_library, cell, view.as_str()).key();
            let new_key = CellViewRef::new(dst_library, new_name, view.as_str()).key();
            if let Some(buffer) = self.workspace.schematic_buffers.get(&old_key).cloned() {
                self.workspace.schematic_buffers.insert(new_key, buffer);
            }
        }

        self.library_manager.select_cell(dst_library, new_name);
        Ok(view_count)
    }

    /// Rename a cell and remap everything that pointed at it: view buffers,
    /// open workspace references, and the Library/Cell binding of every
    /// instance in this project's designs. Returns the number of instance
    /// bindings remapped, or the user-facing error.
    pub(crate) fn rename_cell(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let lib = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if lib.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        if lib.get_cell(cell).is_none() {
            return Err(format!("Cell '{cell}' not found in library '{library}'"));
        }
        if lib.get_cell(new_name).is_some() {
            return Err(format!(
                "Cell '{new_name}' already exists in library '{library}'"
            ));
        }

        if let Some(lib) = self.library_manager.get_library_mut(library) {
            if let Some(mut moved) = lib.cells.remove(cell) {
                moved.name = new_name.to_owned();
                lib.cells.insert(new_name.to_owned(), moved);
            }
        }

        // Buffers move with the cell.
        let old_prefix = format!("{library}/{cell}/");
        let moved_keys: Vec<String> = self
            .workspace
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&old_prefix))
            .cloned()
            .collect();
        for key in moved_keys {
            if let Some(buffer) = self.workspace.schematic_buffers.remove(&key) {
                let tail = &key[old_prefix.len()..];
                self.workspace
                    .schematic_buffers
                    .insert(format!("{library}/{new_name}/{tail}"), buffer);
            }
        }

        // Open references follow.
        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                reference.cell = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.active_view);
        for reference in &mut self.workspace.hierarchy_stack {
            remap_ref(reference);
        }

        // Instance bindings follow — in every buffer and the live sheet.
        let mut remapped = 0usize;
        let mut remap_schematic = |schematic: &mut crate::state::SchematicState| {
            for component in &mut schematic.components {
                if let Some(binding) = component.library_cell.as_mut()
                    && binding.library == library
                    && binding.cell == cell
                {
                    binding.cell = new_name.to_owned();
                    remapped += 1;
                }
            }
        };
        for buffer in self.workspace.schematic_buffers.values_mut() {
            remap_schematic(buffer);
        }
        remap_schematic(&mut self.schematic);

        self.library_manager.select_cell(library, new_name);
        Ok(remapped)
    }

    /// Refuse an edit on a read-only view, with the console line that names
    /// the library. Returns true when the edit must be blocked.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        if !self.active_view_read_only() {
            return false;
        }
        self.push_user_message(ConsoleMessage::warning(self.read_only_master_message()));
        true
    }

    /// Ascend one hierarchy level (the U gesture / pathbar action).
    pub(crate) fn ascend_workspace_level(&mut self) {
        let len = self.workspace.hierarchy_stack.len();
        if len >= 2 {
            self.focus_workspace_breadcrumb(len - 2);
        }
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        self.sync_active_schematic_to_workspace();
        if let Some(reference) = self.workspace.focus_breadcrumb(index) {
            self.library_manager
                .select_view(&reference.library, &reference.cell, &reference.view);
            self.schematic = schematic_for_workspace(self, &reference);
        }
    }

    pub(crate) fn open_selected_instance_master(&mut self) {
        let Some(component_id) = self.schematic.selection.single_component() else {
            self.push_user_message(ConsoleMessage::warning(
                "Select one hierarchical instance first",
            ));
            return;
        };

        let Some(component) = self
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)
        else {
            return;
        };

        if component.kind != ComponentType::CellInstance {
            self.push_user_message(ConsoleMessage::warning(
                "Selected component is not a hierarchical instance",
            ));
            return;
        }

        let Some(binding) = component.library_cell.as_ref() else {
            self.push_user_message(ConsoleMessage::warning(
                "Selected instance has no Library/Cell/View binding",
            ));
            return;
        };

        let instance_name = component.name.clone();
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        self.descend_into_instance(Some(instance_name), reference);
    }
}

impl RSpiceApp {
    pub(crate) fn restore_workspace_after_project_load(&mut self) {
        self.state.restore_active_schematic_from_workspace();
        self.state.clear_transient_specialized_viewer_data();
    }
}

#[cfg(test)]
mod tests;

fn parse_encoded_ports(encoded: &str) -> Vec<PortSpec> {
    encoded
        .split_whitespace()
        .filter_map(|entry| {
            let (name, direction) = entry.split_once(':')?;
            let name = name.trim();
            if name.is_empty() {
                return None;
            }
            Some(PortSpec {
                name: name.to_owned(),
                direction: PortDirection::parse(direction),
            })
        })
        .collect()
}
