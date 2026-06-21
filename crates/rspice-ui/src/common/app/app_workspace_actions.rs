use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};
use crate::panels::{LogAnchor, LogSeverity, LogSource};
use crate::services::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::shell::SymbolDocumentSnapshot;
use crate::state::{
    CellViewRef, Component, ComponentType, OpenCellView, PinFindingKind, Point, PortDirection,
    PortSpec, SYMBOL_DOCUMENT_METADATA_KEY, SchematicState, SymbolDocument, View, ViewType,
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

fn reference_exists_in(libraries: &crate::state::LibraryManager, reference: &CellViewRef) -> bool {
    libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .is_some()
}

fn first_schematic_reference_in(libraries: &crate::state::LibraryManager) -> Option<CellViewRef> {
    for library in libraries.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell.views_sorted() {
                if is_schematic_like(view.view_type) {
                    return Some(CellViewRef::new(&library.name, &cell.name, &view.name));
                }
            }
        }
    }
    None
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
        let Some(old_position) = before_pin.position.map(|position| position - before.origin)
        else {
            continue;
        };
        let Some(after_pin) = after.pin(&before_pin.name) else {
            continue;
        };
        let Some(new_position) = after_pin.position.map(|position| position - after.origin) else {
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
        let active_view_type = self.workspace.active_view_type();
        let dirty_symbol_view = self
            .workspace
            .open_views
            .iter()
            .any(|open| open.view_type == ViewType::Symbol && open.dirty);
        let project_backed_schematic = is_schematic_like(active_view_type)
            && self.schematic.current_file.is_none()
            && (self.schematic.is_dirty || self.workspace.any_dirty());

        active_view_type == ViewType::Symbol
            || dirty_symbol_view
            || self.workspace.netlist_source_dirty
            || project_backed_schematic
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
    pub(in crate::common::app) fn prune_workspace_after_cell_deleted(
        &mut self,
        library: &str,
        cell: &str,
    ) {
        self.sync_active_schematic_to_workspace();
        let active_removed = self.workspace.active_view.library == library
            && self.workspace.active_view.cell == cell;
        let project_root_removed = self.workspace.project.root_library == library
            && self.workspace.project.top_cell == cell;
        let prefix = format!("{library}/{cell}/");
        self.workspace
            .schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .open_views
            .retain(|open| open.reference.library != library || open.reference.cell != cell);
        let old_hierarchy_len = self.workspace.hierarchy_stack.len();
        self.workspace
            .hierarchy_stack
            .retain(|reference| reference.library != library || reference.cell != cell);
        let hierarchy_pruned = self.workspace.hierarchy_stack.len() != old_hierarchy_len;
        if active_removed || self.workspace.hierarchy_stack.is_empty() {
            self.workspace.hierarchy_instances.clear();
        } else {
            self.workspace
                .hierarchy_instances
                .truncate(self.workspace.hierarchy_stack.len().saturating_sub(1));
        }
        self.restore_valid_workspace_focus_after_prune(
            active_removed,
            hierarchy_pruned,
            project_root_removed,
            true,
        );
    }

    pub(in crate::common::app) fn prune_workspace_after_view_deleted(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) {
        self.sync_active_schematic_to_workspace();
        let deleted = CellViewRef::new(library, cell, view);
        let active_removed = self.workspace.active_view == deleted;
        let project_root_removed = self.workspace.project.root_library == library
            && self.workspace.project.top_cell == cell
            && view == crate::state::workspace::DEFAULT_SCHEMATIC_VIEW;
        self.workspace.schematic_buffers.remove(&deleted.key());
        self.workspace
            .open_views
            .retain(|open| open.reference != deleted);
        let old_hierarchy_len = self.workspace.hierarchy_stack.len();
        self.workspace
            .hierarchy_stack
            .retain(|reference| reference != &deleted);
        let hierarchy_pruned = self.workspace.hierarchy_stack.len() != old_hierarchy_len;
        if active_removed || self.workspace.hierarchy_stack.is_empty() {
            self.workspace.hierarchy_instances.clear();
        } else {
            self.workspace
                .hierarchy_instances
                .truncate(self.workspace.hierarchy_stack.len().saturating_sub(1));
        }

        let preferred = CellViewRef::new(
            library,
            cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let preferred = reference_exists_in(&self.library_manager, &preferred).then_some(preferred);
        self.restore_valid_workspace_focus_with_preferred(
            active_removed,
            hierarchy_pruned,
            preferred,
            project_root_removed,
            true,
        );
    }

    fn restore_valid_workspace_focus_after_prune(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        repoint_project_root: bool,
        clear_execution_context: bool,
    ) {
        self.restore_valid_workspace_focus_with_preferred(
            active_removed,
            hierarchy_pruned,
            None,
            repoint_project_root,
            clear_execution_context,
        );
    }

    fn restore_valid_workspace_focus_with_preferred(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        preferred: Option<CellViewRef>,
        repoint_project_root: bool,
        clear_execution_context: bool,
    ) {
        let libraries = &self.library_manager;
        self.workspace
            .open_views
            .retain(|open| reference_exists_in(libraries, &open.reference));
        self.workspace
            .hierarchy_stack
            .retain(|reference| reference_exists_in(libraries, reference));

        let active_valid = !active_removed
            && reference_exists_in(&self.library_manager, &self.workspace.active_view);
        let fallback = if active_valid {
            self.workspace.active_view.clone()
        } else {
            preferred
                .filter(|reference| reference_exists_in(&self.library_manager, reference))
                .or_else(|| {
                    self.workspace
                        .open_views
                        .iter()
                        .find(|open| reference_exists_in(&self.library_manager, &open.reference))
                        .map(|open| open.reference.clone())
                })
                .unwrap_or_else(|| self.create_fallback_schematic_cell())
        };
        let fallback_type = view_type_for_reference(self, &fallback);

        if !self
            .workspace
            .open_views
            .iter()
            .any(|open| open.reference == fallback)
        {
            self.workspace
                .open_views
                .push(OpenCellView::new(fallback.clone(), fallback_type));
        }
        if !active_valid {
            self.workspace.active_view = fallback.clone();
        }
        if self.workspace.hierarchy_stack.is_empty()
            || !self
                .workspace
                .hierarchy_stack
                .iter()
                .any(|reference| reference == &self.workspace.active_view)
            || hierarchy_pruned
        {
            self.workspace.hierarchy_stack = vec![self.workspace.active_view.clone()];
            self.workspace.hierarchy_instances.clear();
        }
        if is_schematic_like(fallback_type) {
            self.workspace.ensure_active_buffer();
        }
        if repoint_project_root {
            self.repoint_project_root_to_surviving_schematic(&fallback, fallback_type);
        }
        self.restore_active_schematic_from_workspace();
        if clear_execution_context {
            self.clear_design_execution_context();
        }
    }

    fn repoint_project_root_to_surviving_schematic(
        &mut self,
        fallback: &CellViewRef,
        fallback_type: ViewType,
    ) {
        let root = if is_schematic_like(fallback_type)
            && reference_exists_in(&self.library_manager, fallback)
        {
            fallback.clone()
        } else {
            self.workspace
                .open_views
                .iter()
                .find(|open| {
                    is_schematic_like(open.view_type)
                        && reference_exists_in(&self.library_manager, &open.reference)
                })
                .map(|open| open.reference.clone())
                .or_else(|| first_schematic_reference_in(&self.library_manager))
                .unwrap_or_else(|| self.create_fallback_schematic_cell())
        };
        self.workspace.project.root_library = root.library;
        self.workspace.project.top_cell = root.cell;
    }

    fn create_fallback_schematic_cell(&mut self) -> CellViewRef {
        let library_name = crate::state::workspace::DEFAULT_PROJECT_LIBRARY.to_string();
        let view_name = crate::state::workspace::DEFAULT_SCHEMATIC_VIEW.to_string();
        if self.library_manager.get_library(&library_name).is_none() {
            self.library_manager
                .add_library(crate::state::Library::new(&library_name));
        }

        let mut index = 1usize;
        let cell_name = loop {
            let candidate = format!("untitled_{index}");
            let exists = self
                .library_manager
                .get_library(&library_name)
                .and_then(|library| library.get_cell(&candidate))
                .is_some();
            if !exists {
                break candidate;
            }
            index += 1;
        };

        if let Some(library) = self.library_manager.get_library_mut(&library_name) {
            let mut cell = crate::state::Cell::new(&cell_name);
            cell.add_view(View::new(&view_name, ViewType::Schematic));
            library.add_cell(cell);
        }

        let reference = CellViewRef::new(library_name, cell_name, view_name);
        self.workspace.project.root_library = reference.library.clone();
        self.workspace.project.top_cell = reference.cell.clone();
        self.workspace
            .schematic_buffers
            .entry(reference.key())
            .or_default();
        reference
    }

    pub(crate) fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        self.sync_active_schematic_to_workspace();
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

        if let Some(lib) = self.library_manager.get_library_mut(library)
            && let Some(mut moved) = lib.cells.remove(cell)
        {
            moved.name = new_name.to_owned();
            lib.cells.insert(new_name.to_owned(), moved);
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
mod tests {
    use crate::common::app::AppState;
    use crate::panels::{LogAnchor, LogSource};
    use crate::services::drc::{DrcLocation, DrcViolationType};
    use crate::shell::WorkspaceView;
    use crate::state::{
        Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, Point,
        PortDirection, PortSpec, ResolvedSymbolSource, Rotation, SYMBOL_DOCUMENT_METADATA_KEY,
        SchematicState, SymbolDocument, SymbolPin, SymbolResolver, View, ViewType, Wire,
    };

    fn symbol_document(pins: &[(&str, PortDirection, Point)]) -> SymbolDocument {
        SymbolDocument {
            pins: pins
                .iter()
                .map(|(name, direction, position)| {
                    SymbolPin::new(*name, *direction, Some(*position))
                })
                .collect(),
            ..SymbolDocument::default()
        }
    }

    fn amp_binding(pins: &[(&str, PortDirection)]) -> LibraryCellInstance {
        let ports: Vec<PortSpec> = pins
            .iter()
            .map(|(name, direction)| PortSpec {
                name: (*name).to_owned(),
                direction: *direction,
            })
            .collect();
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&ports);
        binding
    }

    fn state_with_amp_symbol(document: SymbolDocument) -> AppState {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("initial symbol stores");
        amp.add_view(symbol_view);
        library.add_cell(amp);
        state.library_manager.add_library(library);

        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
        state
    }

    fn state_with_work_cell(cell_name: &str) -> AppState {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new(cell_name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
        state.library_manager.add_library(library);
        state.open_workspace_view(CellViewRef::new("work", cell_name, "schematic"));
        state
    }

    fn state_with_amp_symbol_pin(pin_name: &str, position: Option<Point>) -> AppState {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = pin_name.to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
        state
            .store_active_symbol_document(&SymbolDocument {
                pins: vec![SymbolPin::new(pin_name, PortDirection::In, position)],
                ..SymbolDocument::default()
            })
            .expect("symbol document stores");

        state
    }

    fn state_with_unplaced_amp_symbol_pins(count: usize) -> AppState {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        for index in 0..count {
            let port_id = schematic.add_component(ComponentType::Port, Point::new(index as i32, 0));
            schematic
                .components
                .iter_mut()
                .find(|component| component.id == port_id)
                .expect("port exists")
                .value = format!("P{index}");
        }
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
        state
            .store_active_symbol_document(&SymbolDocument::default())
            .expect("symbol document stores");
        state
    }

    /// A persisted session whose active tab points into the legacy seeded
    /// "primitives" library must come back with the drawn content moved to
    /// the user library and the placeholder library gone — otherwise
    /// `ensure_library_model` resurrects the placeholder cell every launch.
    #[test]
    fn legacy_primitives_content_migrates_to_user_library() {
        let mut state = AppState::default();

        // Simulate the legacy session: seeded library + an open tab with
        // a drawn schematic inside it.
        let mut legacy = Library::new("primitives");
        let mut cell = Cell::new("ISource AC");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        legacy.add_cell(cell);
        state.library_manager.add_library(legacy);

        let mut drawn = crate::state::SchematicState::default();
        drawn.add_component(ComponentType::Njfet, Point::new(0, 0));
        state
            .workspace
            .schematic_buffers
            .insert("primitives/ISource AC/schematic".to_owned(), drawn);
        state.workspace.active_view =
            crate::state::CellViewRef::new("primitives", "ISource AC", "schematic");

        state.migrate_legacy_primitives();

        assert!(
            state.library_manager.get_library("primitives").is_none(),
            "legacy library must be gone"
        );
        assert_eq!(state.workspace.active_view.library, "user");
        let migrated = state
            .workspace
            .schematic_buffers
            .get("user/ISource AC/schematic")
            .expect("buffer migrated to the user library");
        assert_eq!(migrated.components.len(), 1, "drawn content preserved");
        assert!(
            state
                .library_manager
                .get_library("user")
                .and_then(|lib| lib.get_cell("ISource AC"))
                .is_some(),
            "migrated cell exists in the user library"
        );
    }

    #[test]
    fn leaving_symbol_view_does_not_save_stale_schematic_under_symbol_key() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        let mut top = Cell::new("top2");
        top.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(top);
        state.library_manager.add_library(library);

        let amp_schematic = CellViewRef::new("work", "amp", "schematic");
        let amp_symbol = CellViewRef::new("work", "amp", "symbol");
        let top_schematic = CellViewRef::new("work", "top2", "schematic");

        let mut amp_buffer = SchematicState::default();
        amp_buffer.add_component(ComponentType::Port, Point::new(0, 0));
        state
            .workspace
            .schematic_buffers
            .insert(amp_schematic.key(), amp_buffer.clone());
        state.schematic = amp_buffer;
        state.workspace.active_view = amp_schematic.clone();
        state.workspace.open_views = vec![crate::state::OpenCellView::new(
            amp_schematic.clone(),
            ViewType::Schematic,
        )];

        state.open_workspace_view(amp_symbol.clone());
        assert_eq!(state.workspace.active_view_type(), ViewType::Symbol);

        state.open_workspace_view(top_schematic);

        assert!(
            !state
                .workspace
                .schematic_buffers
                .contains_key(&amp_symbol.key()),
            "switching away from symbol must not persist the live schematic under the symbol key"
        );
    }

    #[test]
    fn active_symbol_ports_read_the_paired_schematic_contract() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "OUT".to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        let ports = state.active_symbol_ports();

        assert_eq!(ports.len(), 1);
        assert_eq!(ports[0].name, "OUT");
    }

    #[test]
    fn generating_active_symbol_document_writes_symbol_view_metadata() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "IN".to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        state
            .generate_active_symbol_document()
            .expect("symbol document generated");

        let view = state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists");
        let doc = crate::state::SymbolDocument::load_from_view(view)
            .expect("stored symbol document parses");
        assert_eq!(
            doc.pin("IN").expect("IN pin exists").position,
            Some(Point::new(30, 0))
        );
        assert!(
            !view.metadata.contains_key("generated"),
            "a stored symbol document is now hand-authored state, not a generated fallback"
        );
        assert!(
            !view.metadata.contains_key("ports"),
            "stored symbol metadata must not keep stale fallback port text"
        );
    }

    #[test]
    fn generate_symbol_document_is_one_undoable_transaction() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "IN".to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        state
            .generate_active_symbol_document()
            .expect("symbol document generated");

        assert!(
            state.can_undo_active_symbol_document(),
            "generation should create one undoable editor transaction"
        );
        assert!(
            state
                .undo_active_symbol_document()
                .expect("undo generated symbol"),
            "the generated document should undo in one step"
        );
        let undone = state
            .load_active_symbol_document()
            .expect("undone symbol document loads");
        assert!(
            undone.body.is_empty() && undone.pins.is_empty(),
            "undo should restore the pre-generation symbol document"
        );
        assert!(
            state
                .redo_active_symbol_document()
                .expect("redo generated symbol"),
            "the generated document should redo in one step"
        );
        assert!(
            !state
                .load_active_symbol_document()
                .expect("redone symbol document loads")
                .body
                .is_empty(),
            "redo should restore generated symbol artwork"
        );
    }

    #[test]
    fn undo_generate_symbol_restores_generated_fallback_metadata_state() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "IN".to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        assert!(
            !state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("symbol"))
                .expect("symbol view exists")
                .metadata
                .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
            "fixture starts without authored symbol metadata"
        );

        state
            .generate_active_symbol_document()
            .expect("symbol document generated");
        assert!(
            state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("symbol"))
                .expect("symbol view exists")
                .metadata
                .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
            "generate writes authored symbol metadata"
        );

        assert!(
            state
                .undo_active_symbol_document()
                .expect("undo generated symbol"),
            "undo should apply"
        );

        let view = state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists");
        assert!(
            !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
            "undo must restore the missing authored metadata state"
        );
        let resolver =
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
        let resolved = resolver
            .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
            .expect("symbol resolves after undo");
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
    }

    #[test]
    fn undo_first_manual_symbol_edit_restores_generated_fallback_source() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut schematic = SchematicState::default();
        let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "IN".to_owned();
        state.workspace.schematic_buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        let before = state
            .load_active_symbol_document()
            .expect("pre-edit fallback document loads");
        assert!(
            !state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("symbol"))
                .expect("symbol view exists")
                .metadata
                .contains_key(SYMBOL_DOCUMENT_METADATA_KEY)
        );

        state.record_symbol_edit(&before);
        state
            .store_active_symbol_document(&SymbolDocument {
                body: vec![crate::state::SymbolShape::Dot {
                    center: Point::origin(),
                    radius: 3,
                }],
                ..before.clone()
            })
            .expect("first manual edit stores authored metadata");

        assert!(
            state
                .undo_active_symbol_document()
                .expect("undo first manual edit")
        );
        let view = state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .expect("symbol view exists");
        assert!(
            !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
            "undo must restore the missing authored metadata state"
        );
        let resolver =
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
        let resolved = resolver
            .resolve_reference(&CellViewRef::new("work", "amp", "symbol"))
            .expect("symbol resolves after undo");
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
    }

    #[test]
    fn storing_symbol_document_remaps_open_instance_wires_by_pin_name() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            ],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("initial symbol stores");
        amp.add_view(symbol_view);
        library.add_cell(amp);
        let mut top = Cell::new("top");
        top.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(top);
        state.library_manager.add_library(library);

        let ports = vec![
            PortSpec {
                name: "IN".to_owned(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "OUT".to_owned(),
                direction: PortDirection::Out,
            },
        ];
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&ports);

        let mut parent = SchematicState::default();
        parent.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding),
        );
        parent
            .wires
            .push(Wire::segment(7, Point::new(60, 50), Point::new(0, 50)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
        state
            .store_active_symbol_document(&SymbolDocument {
                pins: vec![
                    SymbolPin::new("IN", PortDirection::In, Some(Point::new(-20, 10))),
                    SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
                ],
                ..SymbolDocument::default()
            })
            .expect("updated symbol stores");

        let parent = state
            .workspace
            .schematic_buffers
            .get("work/top/schematic")
            .expect("parent schematic remains open");
        assert_eq!(parent.wires[0].points[0], Point::new(80, 60));
        assert_eq!(parent.wires[0].points[1], Point::new(0, 50));
    }

    #[test]
    fn storing_symbol_document_remaps_instance_wires_when_origin_moves() {
        let before = SymbolDocument {
            origin: Point::new(20, 0),
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(30, 0)),
            )],
            ..SymbolDocument::default()
        };
        let mut state = state_with_amp_symbol(before);

        let mut parent = SchematicState::default();
        parent.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(amp_binding(&[("IN", PortDirection::In)])),
        );
        parent
            .wires
            .push(Wire::segment(7, Point::new(110, 50), Point::new(0, 50)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

        state
            .store_active_symbol_document(&SymbolDocument {
                origin: Point::origin(),
                pins: vec![SymbolPin::new(
                    "IN",
                    PortDirection::In,
                    Some(Point::new(30, 0)),
                )],
                ..SymbolDocument::default()
            })
            .expect("updated symbol stores");

        let parent = state
            .workspace
            .schematic_buffers
            .get("work/top/schematic")
            .expect("parent schematic remains open");
        assert_eq!(parent.wires[0].points[0], Point::new(130, 50));
        assert_eq!(parent.wires[0].points[1], Point::new(0, 50));
    }

    #[test]
    fn storing_symbol_document_remaps_rotated_and_mirrored_instance_wires() {
        let mut state = state_with_amp_symbol(symbol_document(&[
            ("IN", PortDirection::In, Point::new(-30, 10)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]));

        let mut parent = SchematicState::default();
        parent.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_rotation(Rotation::R90)
                .with_mirror_h(true)
                .with_library_cell(amp_binding(&[
                    ("IN", PortDirection::In),
                    ("OUT", PortDirection::Out),
                ])),
        );
        parent
            .wires
            .push(Wire::segment(8, Point::new(90, 80), Point::new(90, 120)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

        state
            .store_active_symbol_document(&symbol_document(&[
                ("IN", PortDirection::In, Point::new(-10, -20)),
                ("OUT", PortDirection::Out, Point::new(40, 0)),
            ]))
            .expect("updated symbol stores");

        let parent = state
            .workspace
            .schematic_buffers
            .get("work/top/schematic")
            .expect("parent schematic remains open");
        assert_eq!(parent.wires[0].points[0], Point::new(120, 60));
        assert_eq!(parent.wires[0].points[1], Point::new(90, 120));
    }

    #[test]
    fn storing_symbol_document_applies_wire_remaps_once() {
        let mut state = state_with_amp_symbol(symbol_document(&[
            ("IN", PortDirection::In, Point::new(0, 0)),
            ("OUT", PortDirection::Out, Point::new(10, 0)),
        ]));

        let mut parent = SchematicState::default();
        parent.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50)).with_library_cell(
                amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]),
            ),
        );
        parent
            .wires
            .push(Wire::segment(9, Point::new(100, 50), Point::new(100, 0)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "top", "schematic").key(), parent);

        state
            .store_active_symbol_document(&symbol_document(&[
                ("IN", PortDirection::In, Point::new(10, 0)),
                ("OUT", PortDirection::Out, Point::new(20, 0)),
            ]))
            .expect("updated symbol stores");

        let parent = state
            .workspace
            .schematic_buffers
            .get("work/top/schematic")
            .expect("parent schematic remains open");
        assert_eq!(
            parent.wires[0].points[0],
            Point::new(110, 50),
            "the IN endpoint must stop at IN's new location, not then match OUT's old location"
        );
    }

    #[test]
    fn storing_symbol_document_remaps_all_open_parent_buffers() {
        let mut state = state_with_amp_symbol(symbol_document(&[
            ("IN", PortDirection::In, Point::new(-40, 0)),
            ("OUT", PortDirection::Out, Point::new(40, 0)),
        ]));
        let binding = amp_binding(&[("IN", PortDirection::In), ("OUT", PortDirection::Out)]);

        let mut top = SchematicState::default();
        top.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding.clone()),
        );
        top.components.push(
            Component::new(2, ComponentType::CellInstance, Point::new(200, 0))
                .with_library_cell(binding.clone()),
        );
        top.wires
            .push(Wire::segment(10, Point::new(60, 50), Point::new(0, 50)));
        top.wires
            .push(Wire::segment(11, Point::new(160, 0), Point::new(160, -50)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "top", "schematic").key(), top);

        let mut tb = SchematicState::default();
        tb.components.push(
            Component::new(3, ComponentType::CellInstance, Point::new(-10, 20))
                .with_library_cell(binding),
        );
        tb.wires
            .push(Wire::segment(12, Point::new(-50, 20), Point::new(-90, 20)));
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "tb", "schematic").key(), tb);

        state
            .store_active_symbol_document(&symbol_document(&[
                ("IN", PortDirection::In, Point::new(-20, 10)),
                ("OUT", PortDirection::Out, Point::new(40, 0)),
            ]))
            .expect("updated symbol stores");

        let top = state
            .workspace
            .schematic_buffers
            .get("work/top/schematic")
            .expect("top schematic remains open");
        assert_eq!(top.wires[0].points[0], Point::new(80, 60));
        assert_eq!(top.wires[1].points[0], Point::new(180, 10));

        let tb = state
            .workspace
            .schematic_buffers
            .get("work/tb/schematic")
            .expect("testbench schematic remains open");
        assert_eq!(tb.wires[0].points[0], Point::new(-30, 30));
    }

    #[test]
    fn symbol_pin_checks_store_structured_drc_results_with_symbol_anchors() {
        let mut state = state_with_amp_symbol_pin("IN", None);

        state.run_active_symbol_pin_checks();

        let result = state.dialogs.drc_results.as_ref().expect("result stored");
        assert!(result.violations().iter().any(|violation| {
            violation.violation_type == DrcViolationType::SymbolUnplacedPin
                && matches!(
                    &violation.location,
                    DrcLocation::SymbolPin { pin_name, .. } if pin_name == "IN"
                )
        }));
        assert!(state.log_buffer.entries().any(|entry| {
            matches!(
                &entry.anchor,
                Some(LogAnchor::Symbol { pin_name, .. }) if pin_name == "IN"
            )
        }));
    }

    #[test]
    fn symbol_pin_checks_cap_console_rows_and_keep_all_results() {
        let mut state = state_with_unplaced_amp_symbol_pins(55);

        state.run_active_symbol_pin_checks();

        let result = state.dialogs.drc_results.as_ref().expect("result stored");
        assert_eq!(result.violations().len(), 55);

        let drc_rows: Vec<_> = state
            .log_buffer
            .entries()
            .filter(|entry| entry.source == LogSource::Drc)
            .collect();
        let anchored_symbol_rows = drc_rows
            .iter()
            .filter(|entry| matches!(entry.anchor, Some(LogAnchor::Symbol { .. })))
            .count();
        assert_eq!(anchored_symbol_rows, 50);
        assert!(
            drc_rows
                .iter()
                .any(|entry| entry.message.contains("+5 more findings"))
        );
    }

    #[test]
    fn copy_cell_flushes_live_active_schematic_before_copying_buffers() {
        let mut state = state_with_work_cell("amp");
        let active_key = CellViewRef::new("work", "amp", "schematic").key();
        state
            .workspace
            .schematic_buffers
            .insert(active_key, SchematicState::default());
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(20, 20));

        let copied = state
            .copy_cell("work", "amp", "work", "amp_copy")
            .expect("copy succeeds");

        let copy = state
            .workspace
            .schematic_buffers
            .get("work/amp_copy/schematic")
            .expect("copy buffer exists");
        assert_eq!(copied, 1);
        assert_eq!(copy.components.len(), 1);
        assert_eq!(copy.components[0].kind, ComponentType::Resistor);
    }

    #[test]
    fn symbol_log_anchor_opens_symbol_view_and_selects_pin() {
        let mut state = state_with_amp_symbol_pin("IN", Some(Point::new(-30, 0)));
        let reference = CellViewRef::new("work", "amp", "symbol");

        state.jump_to_log_anchor(LogAnchor::Symbol {
            reference: reference.clone(),
            pin_name: "IN".to_owned(),
            point: Some(Point::new(-30, 0)),
        });

        assert_eq!(state.workspace.active_view, reference);
        assert_eq!(state.shell.symbol.selected_pin.as_deref(), Some("IN"));
        assert_eq!(state.shell.view, WorkspaceView::Schematic);
    }

    #[test]
    fn symbol_violation_cycle_opens_symbol_view_and_selects_pin() {
        let mut state = state_with_amp_symbol_pin("IN", None);
        state.run_active_symbol_pin_checks();
        state.shell.symbol.clear_selection();

        crate::schematic::view::violations::cycle_violation(&mut state, 1);

        assert_eq!(
            state.workspace.active_view,
            CellViewRef::new("work", "amp", "symbol")
        );
        assert_eq!(state.shell.symbol.selected_pin.as_deref(), Some("IN"));
    }

    #[test]
    fn symbol_undo_history_is_scoped_to_active_view() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        for cell_name in ["amp_a", "amp_b"] {
            let mut cell = Cell::new(cell_name);
            cell.add_view(View::new("schematic", ViewType::Schematic));
            cell.add_view(View::new("symbol", ViewType::Symbol));
            library.add_cell(cell);
        }
        state.library_manager.add_library(library);

        state.open_workspace_view(CellViewRef::new("work", "amp_a", "symbol"));
        let before_a = SymbolDocument {
            name_anchor: Point::new(-10, -10),
            ..SymbolDocument::default()
        };
        let after_a = SymbolDocument {
            name_anchor: Point::new(-20, -20),
            ..SymbolDocument::default()
        };
        state.record_symbol_edit(&before_a);
        state
            .store_active_symbol_document(&after_a)
            .expect("store amp_a symbol");

        state.open_workspace_view(CellViewRef::new("work", "amp_b", "symbol"));
        let before_b = SymbolDocument {
            name_anchor: Point::new(10, 10),
            ..SymbolDocument::default()
        };
        state
            .store_active_symbol_document(&before_b)
            .expect("store amp_b symbol");

        assert!(
            !state
                .undo_active_symbol_document()
                .expect("undo amp_b symbol"),
            "undo from amp_a must not apply to amp_b"
        );
        assert_eq!(
            state
                .load_active_symbol_document()
                .expect("amp_b symbol loads")
                .name_anchor,
            Point::new(10, 10)
        );

        state.open_workspace_view(CellViewRef::new("work", "amp_a", "symbol"));
        assert!(
            state
                .undo_active_symbol_document()
                .expect("undo amp_a symbol"),
            "amp_a keeps its own undo stack"
        );
        assert_eq!(
            state
                .load_active_symbol_document()
                .expect("amp_a symbol loads")
                .name_anchor,
            Point::new(-10, -10)
        );
    }

    #[test]
    fn symbol_store_refuses_read_only_libraries() {
        let mut state = AppState::default();

        let mut library = Library::new("readonly");
        library.read_only = true;
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        state.library_manager.add_library(library);
        state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

        let error = state
            .store_active_symbol_document(&SymbolDocument::default())
            .expect_err("read-only symbol store is rejected");

        assert_eq!(error, "Read-only - 'readonly' masters cannot be edited");
    }

    #[test]
    fn read_only_symbol_edit_paths_use_consistent_refusal_text() {
        let mut state = AppState::default();

        let mut library = Library::new("readonly");
        library.read_only = true;
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        state.library_manager.add_library(library);
        state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));

        let expected = "Read-only - 'readonly' masters cannot be edited";

        assert_eq!(
            state
                .store_active_symbol_document(&SymbolDocument::default())
                .expect_err("store should be refused"),
            expected
        );
        assert_eq!(
            state
                .generate_active_symbol_document()
                .expect_err("generate should be refused"),
            expected
        );
        assert_eq!(
            state
                .undo_active_symbol_document()
                .expect_err("undo should be refused"),
            expected
        );
        assert_eq!(
            state
                .redo_active_symbol_document()
                .expect_err("redo should be refused"),
            expected
        );

        assert!(state.deny_read_only_edit());
        let warning = state
            .log_buffer
            .entries()
            .last()
            .expect("read-only warning is logged");
        assert_eq!(warning.message, expected);
    }

    #[test]
    fn opening_symbol_view_loads_the_paired_schematic_context() {
        let mut state = AppState::default();

        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        state.library_manager.add_library(library);

        let mut paired = SchematicState::default();
        let port_id = paired.add_component(ComponentType::Port, Point::new(0, 0));
        paired
            .components
            .iter_mut()
            .find(|component| component.id == port_id)
            .expect("port exists")
            .value = "PAIR".to_owned();
        state
            .workspace
            .schematic_buffers
            .insert(CellViewRef::new("work", "amp", "schematic").key(), paired);

        state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

        let ports = state.schematic.interface_ports();
        assert_eq!(ports.len(), 1);
        assert_eq!(ports[0].name, "PAIR");
    }

    #[test]
    fn symbol_dirty_state_routes_ordinary_save_to_project() {
        let mut state = AppState::default();
        let reference = CellViewRef::new("user", "top", "symbol");
        state.workspace.open_view(reference, ViewType::Symbol);
        state.workspace.set_active_dirty(true);

        assert!(state.should_save_project_for_active_document());
    }

    #[test]
    fn project_schematic_dirty_state_routes_ordinary_save_to_project() {
        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state.workspace.set_active_dirty(true);

        assert!(state.should_save_project_for_active_document());
    }

    #[test]
    fn standalone_schematic_current_file_keeps_ordinary_save_on_schematic_file() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(std::path::PathBuf::from("standalone.rsch"));
        state.schematic.is_dirty = true;
        state.workspace.set_active_dirty(true);

        assert!(!state.should_save_project_for_active_document());
    }

    #[test]
    fn dirty_manual_netlist_source_routes_ordinary_save_to_project() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(std::path::PathBuf::from("standalone.rsch"));
        state.workspace.netlist_source = Some("deck\n.op\n.end\n".to_owned());
        state.workspace.set_netlist_source_dirty(true);

        assert!(state.should_save_project_for_active_document());
        assert!(state.workspace.any_dirty());

        state.workspace.mark_all_clean();

        assert!(!state.workspace.any_dirty());
    }
}

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
