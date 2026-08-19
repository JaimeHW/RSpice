//! Workspace-level edit actions.
//!
//! The clipboard, selection, undo/redo, and hierarchy operations that a menu
//! item or a shortcut resolves to. These act on the whole active document
//! rather than on one selected object, and they are the actions that a
//! read-only schematic refuses.

mod symbol_document;

use crate::diagnostics::{ConsoleMessage, LogAnchor, LogSeverity};
use crate::schematic::view::SchematicSymbolContext;
use crate::services::drc::DrcSeverity;
use crate::state::{
    CellViewRef, Component, ComponentType, OpenCellView, Point, PortDirection, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SchematicState, SymbolDocument, View,
    ViewType,
};
use crate::workbench::SymbolDocumentSnapshot;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::{AppState, DesignManagementHistoryEntry};
use crate::workbench::state::WorkspaceDocumentId;
use std::collections::{BTreeMap, HashMap};

pub(super) const MAX_FINDING_ROWS: usize = 50;

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

/// Where each pin's terminal moved to, keyed by the name it had before.
///
/// This answers one question only — did a terminal move — so that the wire
/// endpoints attached to it move with it. It cannot answer whether a pin was
/// renamed: the old name is simply absent from `after`, which is
/// indistinguishable from a deletion. `renames` supplies that half, declared
/// by whoever performed the rename, so a pin that was renamed *and* moved is
/// still followed to its new position.
pub(super) fn symbol_pin_position_remaps(
    before: &SymbolDocument,
    after: &SymbolDocument,
    renames: &std::collections::BTreeMap<String, String>,
) -> HashMap<String, (Point, Point)> {
    let mut remaps = HashMap::new();
    for before_pin in &before.pins {
        let Some(old_position) = before_pin.position.map(|position| position - before.origin)
        else {
            continue;
        };
        let after_name = renames
            .get(&before_pin.name)
            .map_or(before_pin.name.as_str(), String::as_str);
        let Some(after_pin) = after.pin(after_name) else {
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

pub(super) fn remap_symbol_instance_wires(
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
    let old_offset = component.transform_point(old_offset);
    let new_offset = component.transform_point(new_offset);
    let old_position = Point::new(
        component.pos.x.saturating_add(old_offset.x),
        component.pos.y.saturating_add(old_offset.y),
    );
    let new_position = Point::new(
        component.pos.x.saturating_add(new_offset.x),
        component.pos.y.saturating_add(new_offset.y),
    );
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
        .unwrap_or_else(|| state.new_schematic_document());
    // Workspace buffers round-trip through serde, which skips the runtime
    // ID counter and name counters: without recalculation a freshly placed
    // component reuses an existing ID and selection matches both.
    schematic.recalculate_runtime_state();
    schematic.snap_engine = state.ui.schematic_snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    state.ui.schematic_snap.grid_size = schematic.grid_size;
    schematic.wire_drawing.routing_mode = state.ui.schematic_routing_mode;
    schematic.bus_drawing.routing_mode = state.ui.schematic_routing_mode;
    // Views from read-only libraries open for inspection, never for edit —
    // the docbar banner explains and every edit path checks this flag.
    schematic.read_only = state.workbench.safe_mode.project_read_only()
        || state
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only);
    schematic
}

impl AppState {
    /// Copy the active complete-object selection using the exact rendered
    /// terminal geometry of authored and generated symbols.
    ///
    /// `SchematicState::copy_selection` remains a state-only fallback for
    /// primitive fixtures. Production UI commands must enter here so a wire
    /// joining two authored cell pins is captured with the selected
    /// instances instead of being silently omitted.
    pub(crate) fn copy_active_schematic_selection(&mut self) -> bool {
        if self.schematic.selection.is_empty() {
            return false;
        }
        let symbols = SchematicSymbolContext::from_state(self);
        self.schematic.clipboard =
            self.schematic
                .capture_complete_selection_resolved(|component| {
                    symbols
                        .named_terminal_points(component)
                        .into_iter()
                        .map(|(_, point)| point)
                        .collect()
                });
        true
    }

    pub(crate) fn rebuild_active_connections_from_symbols(&mut self) {
        let terminals = {
            let symbols = SchematicSymbolContext::from_state(self);
            self.schematic
                .components
                .iter()
                .flat_map(|component| {
                    symbols
                        .named_terminal_points(component)
                        .into_iter()
                        .map(move |(name, point)| (component.id, name, point))
                })
                .collect::<Vec<_>>()
        };
        self.schematic
            .rebuild_connections_from_terminals(&terminals);
    }

    /// Resolve the user defaults into one explicit, project-portable document
    /// policy. This is the only constructor production UI paths use for a new
    /// schematic/testbench document. Existing buffers are never rewritten.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        use crate::state::{
            NetNamingPolicy, OperatingPointAnnotationPolicy, PropertyCommitPolicy,
            SchematicGridPitch, SelectionCrossingPolicy, WireJunctionPolicy,
        };
        use crate::workbench::ChoicePreference;

        let mut schematic = SchematicState::default();
        // Snap targets and radius are device-local session preferences. A new
        // document inherits those exact choices, while its pitch is resolved
        // below into project-portable policy and then projected back into the
        // runtime engine.
        schematic.snap_engine = self.ui.schematic_snap.clone();
        let preferences = &self.ui.preferences;
        schematic.document_policy.grid_pitch =
            match preferences.choice(ChoicePreference::SchematicGrid) {
                0 => SchematicGridPitch::Mil50,
                1 => SchematicGridPitch::Mil25,
                2 => SchematicGridPitch::Metric,
                _ => unreachable!("schematic grid preference is normalized before use"),
            };
        schematic.document_policy.wire_junctions =
            match preferences.choice(ChoicePreference::WireJunctionBehavior) {
                0 => WireJunctionPolicy::OrthogonalAutomatic,
                1 => WireJunctionPolicy::OrthogonalManual,
                2 => WireJunctionPolicy::AnyAngle,
                _ => unreachable!("wire policy is normalized before use"),
            };
        schematic.document_policy.selection_crossing =
            match preferences.choice(ChoicePreference::SelectionCrossingPolicy) {
                0 => SelectionCrossingPolicy::Directional,
                1 => SelectionCrossingPolicy::EnclosedOnly,
                2 => SelectionCrossingPolicy::Intersecting,
                _ => unreachable!("selection policy is normalized before use"),
            };
        schematic.document_policy.net_naming =
            match preferences.choice(ChoicePreference::NetNamingPolicy) {
                0 => NetNamingPolicy::StrictCaseSensitive,
                1 => NetNamingPolicy::SpiceCompatibleRelaxed,
                _ => unreachable!("net naming policy is normalized before use"),
            };
        schematic.document_policy.property_commit =
            match preferences.choice(ChoicePreference::PropertyCommitPolicy) {
                0 => PropertyCommitPolicy::Atomic,
                1 => PropertyCommitPolicy::ApplyValidFields,
                _ => unreachable!("property policy is normalized before use"),
            };
        schematic.document_policy.operating_point_annotations =
            match preferences.choice(ChoicePreference::OperatingPointAnnotation) {
                0 => OperatingPointAnnotationPolicy::VoltagesAndSelectedCurrents,
                1 => OperatingPointAnnotationPolicy::VoltagesOnly,
                2 => OperatingPointAnnotationPolicy::Hidden,
                _ => unreachable!("operating-point policy is normalized before use"),
            };

        schematic.reconcile_grid_pitch_runtime();
        let routing_mode = self.ui.schematic_routing_mode;
        schematic.wire_drawing.set_routing_mode(routing_mode);
        schematic.bus_drawing.routing_mode = routing_mode;
        schematic
    }
}

pub(super) fn log_severity_from_drc(severity: DrcSeverity) -> LogSeverity {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => LogSeverity::Error,
        DrcSeverity::Warning => LogSeverity::Warning,
        DrcSeverity::Info => LogSeverity::Info,
    }
}

pub(super) fn symbol_snapshot_from_view(
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
        symbol_editor_metadata: view.metadata.get(SYMBOL_EDITOR_METADATA_KEY).cloned(),
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
        renames: std::collections::BTreeMap::new(),
    }
}

pub(super) fn symbol_metadata_snapshot_from_view(
    view: &View,
    fallback: &SymbolDocument,
) -> SymbolDocumentSnapshot {
    SymbolDocumentSnapshot {
        document: fallback.clone(),
        symbol_document_metadata: view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY).cloned(),
        symbol_editor_metadata: view.metadata.get(SYMBOL_EDITOR_METADATA_KEY).cloned(),
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
        renames: std::collections::BTreeMap::new(),
    }
}

pub(super) fn restore_symbol_snapshot_in_view(view: &mut View, snapshot: &SymbolDocumentSnapshot) {
    match &snapshot.symbol_document_metadata {
        Some(encoded) => {
            view.metadata
                .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove(SYMBOL_DOCUMENT_METADATA_KEY);
        }
    }
    match &snapshot.symbol_editor_metadata {
        Some(encoded) => {
            view.metadata
                .insert(SYMBOL_EDITOR_METADATA_KEY.to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove(SYMBOL_EDITOR_METADATA_KEY);
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
    /// Whether the active cellview refuses writes, whatever kind of document
    /// it is.
    ///
    /// Safe mode is one of the three owners of that refusal, alongside a
    /// read-only library master and a read-only hierarchy reference. It
    /// belongs here rather than only in the schematic gate because every
    /// document class — symbol, layout, hierarchy reference, review comment —
    /// writes into the same project that safe mode opened read-only.
    pub(crate) fn active_view_read_only(&self) -> bool {
        self.workbench.safe_mode.project_read_only()
            || self.workspace.active_read_only_reference()
            || self
                .library_manager
                .get_library(&self.workspace.active_view.library)
                .is_some_and(|library| library.read_only)
    }

    /// Single late-bound authority for every schematic mutation surface.
    ///
    /// Safe mode can be activated after a tool or dialog was armed, so callers
    /// must not rely only on the persisted schematic flag captured earlier.
    pub(crate) fn schematic_edit_read_only(&self) -> bool {
        self.schematic.read_only || self.active_view_read_only()
    }

    /// Names the owner of the refusal, in the order
    /// [`Self::active_view_read_only`] consults them.
    pub(crate) fn read_only_master_message(&self) -> String {
        if self.workbench.safe_mode.project_read_only() {
            return "Safe mode opened this project read-only - restart without safe mode to edit"
                .to_owned();
        }
        if self.workspace.active_read_only_reference() {
            return "Read-only hierarchy reference - reopen the view in an editable context to modify it"
                .to_owned();
        }
        let library = &self.workspace.active_view.library;
        format!("Read-only - '{library}' masters cannot be edited")
    }

    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigate_to_log_anchor(anchor);
    }

    fn navigate_to_log_anchor(&mut self, anchor: LogAnchor) {
        match anchor {
            LogAnchor::Schematic {
                x,
                y,
                component,
                wire,
            } => {
                self.workbench
                    .activate(crate::workbench::state::Workspace::Design);
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
                self.workbench
                    .activate(crate::workbench::state::Workspace::Design);
                self.ui.symbol.select_pin(pin_name);
                if let Some(point) = point {
                    let zoom = self.ui.symbol.zoom.max(1.0);
                    self.ui.symbol.pan = (-(point.x as f32) * zoom, -(point.y as f32) * zoom);
                }
                self.ui.symbol.needs_fit = false;
            }
        }
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
        self.workspace.remap_occurrence_masters(&remap);

        self.library_manager.purge_legacy_primitives();
    }

    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        if is_schematic_like(self.workspace.active_view_type()) {
            let active = self.workspace.active_schematic_reference();
            self.reconcile_active_sheet_membership(&active);
            // Nothing may be written back to the workspace still claiming a
            // master's netlist identity after that master is gone — including
            // a placement an undo has just restored, which is otherwise the
            // one way a deleted cell comes back looking resolved.
            self.schematic
                .revalidate_instance_bindings(&self.library_manager);
            self.workspace.save_active_schematic(&self.schematic);
            self.sync_generated_symbol_view();
        }
    }

    /// Bring the active cell view's sheet catalog back into step with its
    /// drawing, and state in the project history anything the drawing cannot
    /// put back on its own.
    ///
    /// Membership travels through the schematic step that is being synced: an
    /// object an undo has just restored returns to the sheet it was drawn on.
    /// A cross-sheet port contract cannot travel that way — nothing in the
    /// drawing holds it — so losing one is a project transaction of its own,
    /// which restores both the contract and the anchors it names.
    fn reconcile_active_sheet_membership(&mut self, active: &CellViewRef) {
        let recorded = self
            .schematic
            .undo_history
            .take_restored_sheet_assignments();
        // Only a cell view that already holds contracts can lose one, and only
        // that case needs the retained before-state. Every other document
        // synchronizes without copying its design authority.
        let before = self
            .workspace
            .design_management
            .sheet_catalog(&active.key())
            .is_some_and(|catalog| !catalog.cross_sheet_ports().is_empty())
            .then(|| {
                (
                    self.workspace.design_management.clone(),
                    self.workspace
                        .schematic_buffers
                        .get(&active.key())
                        .cloned()
                        .unwrap_or_else(|| self.schematic.clone()),
                )
            });
        let receipt = match self.workspace.assign_unowned_objects_to_active_sheet(
            active,
            &self.schematic,
            &recorded,
        ) {
            Ok(receipt) => receipt,
            Err(error) => {
                self.push_user_message(ConsoleMessage::warning(format!(
                    "Sheet membership could not be updated: {error}"
                )));
                return;
            }
        };
        let committed_revision = self.workspace.project.revision();
        let assignments = self
            .workspace
            .design_management
            .sheet_catalog(&active.key())
            .map(|catalog| catalog.object_assignments().clone())
            .unwrap_or_default();
        self.schematic
            .undo_history
            .set_live_sheet_assignments(assignments);
        let removed_ports = receipt.map_or(0, |receipt| receipt.removed_cross_sheet_ports);
        let Some((before, before_schematic)) = before.filter(|_| removed_ports > 0) else {
            return;
        };
        let after = self.workspace.design_management.clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: "drop cross-sheet connections whose anchors were deleted".to_owned(),
            owner: active.clone(),
            before,
            after,
            before_schematics: BTreeMap::from([(active.key(), before_schematic)]),
            after_schematics: BTreeMap::from([(active.key(), self.schematic.clone())]),
            committed_revision,
        });
        let subject = if removed_ports == 1 {
            "connection"
        } else {
            "connections"
        };
        self.push_user_message(ConsoleMessage::warning(format!(
            "Undo restores {removed_ports} cross-sheet {subject} dropped with the objects they \
             anchor to."
        )));
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
        let mut symbol_error = None;
        let ports = self.schematic.interface_ports();
        let existing_symbol = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view("symbol"))
            .cloned();
        if self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .is_none()
        {
            return;
        }

        if ports.is_empty() {
            if !existing_symbol
                .as_ref()
                .is_some_and(|view| view.metadata.contains_key(GENERATED_KEY))
            {
                return;
            }
            if let Some(cell) = self
                .library_manager
                .get_library_mut(&reference.library)
                .and_then(|library| library.get_cell_mut(&reference.cell))
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
        match existing_symbol {
            Some(mut view) if view.metadata.contains_key(GENERATED_KEY) => {
                let before = view.metadata.clone();
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
                if let Err(error) =
                    SymbolDocument::generated_from_ports(&ports).store_in_view(&mut view)
                {
                    symbol_error =
                        Some(format!("Generated symbol could not be refreshed: {error}"));
                } else if view.metadata != before
                    && let Some(target) = self
                        .library_manager
                        .get_library_mut(&reference.library)
                        .and_then(|library| library.get_cell_mut(&reference.cell))
                        .and_then(|cell| cell.get_view_mut("symbol"))
                {
                    target.metadata = view.metadata;
                }
            }
            Some(_) => {} // hand-authored symbol: leave it alone
            None => {
                let mut view = View::new("symbol", ViewType::Symbol);
                view.metadata
                    .insert(GENERATED_KEY.to_owned(), "ports".to_owned());
                view.metadata.insert(PORTS_KEY.to_owned(), encoded);
                if let Err(error) =
                    SymbolDocument::generated_from_ports(&ports).store_in_view(&mut view)
                {
                    symbol_error = Some(format!("Generated symbol could not be created: {error}"));
                } else if let Some(cell) = self
                    .library_manager
                    .get_library_mut(&reference.library)
                    .and_then(|library| library.get_cell_mut(&reference.cell))
                {
                    cell.add_view(view);
                }
            }
        }
        if let Some(error) = symbol_error {
            self.push_user_message(ConsoleMessage::warning(error));
        }
    }

    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        self.workspace
            .ensure_library_model(&mut self.library_manager);
        let reference = self.workspace.active_view.clone();
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        // Project persistence stores topology, not the derived rubber-band
        // connection cache. Rebuild against the same authored/generated
        // symbol geometry that is rendered and netlisted.
        self.rebuild_active_connections_from_symbols();
        self.refresh_active_design_check_projection();
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
    }

    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.sync_active_schematic_to_workspace();
        if self.workspace.active_view == reference {
            self.workbench
                .documents
                .activate(WorkspaceDocumentId::CellView(reference));
            return;
        }
        self.ui.canvas_hover = None;
        self.ui.canvas_view_center = None;
        let view_type = view_type_for_reference(self, &reference);
        self.workspace.activate_view(reference.clone(), view_type);
        self.workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference.clone()));
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.push_user_message(ConsoleMessage::info(format!(
            "Opened {}",
            reference.display_path()
        )));
    }

    /// Descend into a hierarchical instance: open its master and record the
    /// instance name on the active document's occurrence. Without an instance
    /// there is no occurrence step to record, so the master opens as its own
    /// design root rather than inheriting a name it was not reached by.
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
            None => self.workspace.open_as_root(reference.clone(), view_type),
        }
        self.workspace.set_active_read_only_reference(false);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        let schematic_reference = self.workspace.active_schematic_reference();
        self.schematic = schematic_for_workspace(self, &schematic_reference);
        self.bump_active_schematic_epoch();
        self.refresh_active_design_check_projection();
        self.push_user_message(ConsoleMessage::info(format!(
            "Entered {}",
            reference.display_path()
        )));
    }

    pub(crate) fn prune_workspace_after_cell_deleted(&mut self, library: &str, cell: &str) {
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
        let hierarchy_pruned = self.workspace.retain_valid_occurrences(|reference| {
            reference.library != library || reference.cell != cell
        });
        self.restore_valid_workspace_focus_after_prune(
            active_removed,
            hierarchy_pruned,
            project_root_removed,
            true,
        );
    }

    pub(crate) fn prune_workspace_after_view_deleted(
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
        self.workspace.remove_physical_layout_document(&deleted);
        self.workspace
            .open_views
            .retain(|open| open.reference != deleted);
        let hierarchy_pruned = self
            .workspace
            .retain_valid_occurrences(|reference| reference != &deleted);

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
            .retain_valid_occurrences(|reference| reference_exists_in(libraries, reference));

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
        // Whatever the restored document was reached through did not survive
        // the prune, so it is a design root again rather than an occurrence
        // whose ancestors are gone.
        if hierarchy_pruned || !active_valid {
            self.workspace.reroot_active_occurrence();
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
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(dst_library, new_name, "");
        if destination.cells.values().any(|candidate| {
            crate::state::canonical_cell_view_owner_key(dst_library, &candidate.name, "")
                == requested_identity
        }) {
            return Err(format!(
                "Cell '{new_name}' conflicts with an existing canonical cell identity in library '{dst_library}'"
            ));
        }

        self.sync_active_schematic_to_workspace();

        let mut candidate_sources = self.workspace.project_sources.clone();
        let copied_source_ids = candidate_sources
            .clone_cell_view_bundles(src_library, cell, dst_library, new_name)
            .map_err(|error| {
                format!("Could not copy the cell's project source bundles: {error}")
            })?;
        let mut candidate_design_management = self.workspace.design_management.clone();
        let copied_sheet_catalogs = candidate_design_management
            .copy_cell_sheet_catalogs(src_library, cell, dst_library, new_name)
            .map_err(|error| format!("Could not copy the cell's sheet-catalog ownership: {error}"))?
            .copied_sheet_catalogs;
        let candidate_layouts = self
            .workspace
            .prepare_copy_physical_layout_cell_documents(src_library, cell, dst_library, new_name)
            .map_err(|error| format!("Could not copy the cell's physical layouts: {error}"))?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CopyCell {
                source_library: src_library.to_owned(),
                source_cell: cell.to_owned(),
                target_library: dst_library.to_owned(),
                target_cell: new_name.to_owned(),
            },
        )?;

        copy.name = new_name.to_owned();
        let view_names: Vec<String> = copy.views.keys().cloned().collect();
        let view_count = view_names.len();
        self.library_manager
            .get_library_mut(dst_library)
            .ok_or_else(|| format!("Library '{dst_library}' disappeared during the copy"))?
            .add_cell(copy);

        // The drawn content lives in the workspace buffers, keyed
        // "library/cell/view" — a copy without it would be a lie.
        for view in view_names {
            let old_key = CellViewRef::new(src_library, cell, view.as_str()).key();
            let new_key = CellViewRef::new(dst_library, new_name, view.as_str()).key();
            if let Some(buffer) = self.workspace.schematic_buffers.get(&old_key).cloned() {
                self.workspace.schematic_buffers.insert(new_key, buffer);
            }
        }
        self.workspace
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        if !copied_source_ids.is_empty() {
            self.workspace.project_sources = candidate_sources;
            self.workspace.project_sources_dirty = true;
        }
        if copied_sheet_catalogs > 0 {
            self.workspace.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

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
        if cell == new_name {
            return Err(format!("Cell '{cell}' already has that name"));
        }
        let requested_identity = crate::state::canonical_cell_view_owner_key(library, new_name, "");
        if lib.cells.iter().any(|(candidate_key, candidate)| {
            candidate_key != cell
                && crate::state::canonical_cell_view_owner_key(library, &candidate.name, "")
                    == requested_identity
        }) {
            return Err(format!(
                "Cell '{new_name}' conflicts with an existing canonical cell identity in library '{library}'"
            ));
        }

        let mut candidate_sources = self.workspace.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_cell_view_bundles(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's project source ownership: {error}")
            })?;
        let mut candidate_configurations = self.workspace.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_cell_roots(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's configuration-set roots: {error}")
            })?;
        let mut candidate_design_management = self.workspace.design_management.clone();
        let design_management_receipt = candidate_design_management
            .rename_cell_sheet_catalogs(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's Design Management ownership: {error}")
            })?;
        let design_management_changed = design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_variant_objects > 0
            || design_management_receipt.remapped_annotation_objects > 0;
        let candidate_layouts = self
            .workspace
            .prepare_rename_physical_layout_cell_documents(library, cell, new_name)
            .map_err(|error| format!("Could not rename the cell's physical layouts: {error}"))?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameCell {
                library: library.to_owned(),
                from_cell: cell.to_owned(),
                to_cell: new_name.to_owned(),
            },
        )?;

        let library_mut = self
            .library_manager
            .get_library_mut(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the rename"))?;
        let mut moved = library_mut
            .cells
            .remove(cell)
            .ok_or_else(|| format!("Cell '{cell}' disappeared during the rename"))?;
        moved.name = new_name.to_owned();
        library_mut.cells.insert(new_name.to_owned(), moved);

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
        self.workspace
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        // Open references follow.
        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                reference.cell = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.active_view);
        for open in &mut self.workspace.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.remap_occurrence_masters(&remap_ref);

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

        if !renamed_source_ids.is_empty() {
            self.workspace.project_sources = candidate_sources;
            self.workspace.project_sources_dirty = true;
            let transient_uses_renamed = self
                .ui
                .code_workspace
                .veriloga
                .receipt
                .as_ref()
                .is_some_and(|receipt| renamed_source_ids.contains(&receipt.token.bundle_id))
                || self
                    .ui
                    .code_workspace
                    .veriloga
                    .pending
                    .as_ref()
                    .is_some_and(|pending| renamed_source_ids.contains(&pending.token.bundle_id));
            if transient_uses_renamed {
                self.ui.code_workspace.veriloga = Default::default();
            }
        }
        if renamed_configuration_roots > 0 {
            self.workspace.configuration_sets = candidate_configurations;
            self.workspace.project_metadata_dirty = true;
        }
        if design_management_changed {
            self.workspace.design_management = candidate_design_management;
        }
        // The project root names its top cell by name. Leaving the old name
        // behind makes `ProjectFile::validate` reject every later save.
        if self.workspace.project.root_library == library && self.workspace.project.top_cell == cell
        {
            self.workspace.project.top_cell = new_name.to_owned();
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_cell(library, new_name);
        Ok(remapped)
    }

    /// Create one empty writable library as a project transaction.
    pub(crate) fn create_library(&mut self, name: &str) -> Result<(), String> {
        if let Some(conflict) = conflicting_library_name(&self.library_manager, name, None) {
            return Err(format!(
                "Library '{name}' conflicts with the existing canonical library identity '{conflict}'"
            ));
        }
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::CreateLibrary {
                library: name.to_owned(),
            },
        )?;

        self.library_manager
            .add_library(crate::state::Library::new(name));
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_library(name);
        Ok(())
    }

    /// Rename a library and remap everything that named it: view buffers,
    /// open workspace references, the Library/Cell binding of every instance,
    /// owned sources, Design Management ownership, and the project root.
    /// Returns the number of instance bindings remapped.
    pub(crate) fn rename_library(
        &mut self,
        library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let source = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if source.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        if crate::state::canonical_cell_view_owner_key(library, "", "")
            == crate::state::canonical_cell_view_owner_key(new_name, "", "")
        {
            return Err(format!(
                "Library '{library}' already has that identity; a name differing only in case or normalization is the same library"
            ));
        }
        if let Some(conflict) =
            conflicting_library_name(&self.library_manager, new_name, Some(library))
        {
            return Err(format!(
                "Library '{new_name}' conflicts with the existing canonical library identity '{conflict}'"
            ));
        }
        let candidate_layouts = self
            .workspace
            .prepare_rename_physical_layout_library_documents(library, new_name)
            .map_err(|error| format!("Could not rename the library's physical layouts: {error}"))?;

        let mut candidate_configurations = self.workspace.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_library_roots(library, new_name)
            .map_err(|error| {
                format!("Could not move the library's configuration-set roots: {error}")
            })?;
        let mut candidate_sources = self.workspace.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_library_bundles(library, new_name)
            .map_err(|error| {
                format!("Could not move the library's project source ownership: {error}")
            })?;
        let mut candidate_design_management = self.workspace.design_management.clone();
        let design_management_receipt = candidate_design_management
            .rename_library_sheet_catalogs(library, new_name)
            .map_err(|error| {
                format!("Could not move the library's Design Management ownership: {error}")
            })?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameLibrary {
                from_library: library.to_owned(),
                to_library: new_name.to_owned(),
            },
        )?;

        let mut moved = self
            .library_manager
            .remove_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the rename"))?;
        moved.name = new_name.to_owned();
        self.library_manager.add_library(moved);

        let old_prefix = format!("{library}/");
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
                    .insert(format!("{new_name}/{tail}"), buffer);
            }
        }
        self.workspace
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library {
                reference.library = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.active_view);
        for open in &mut self.workspace.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.remap_occurrence_masters(&remap_ref);

        let remapped = self.remap_instance_bindings(|binding| {
            let matched = binding.library == library;
            if matched {
                binding.library = new_name.to_owned();
            }
            matched
        });

        self.commit_renamed_project_sources(candidate_sources, &renamed_source_ids);
        if renamed_configuration_roots > 0 {
            self.workspace.configuration_sets = candidate_configurations;
            self.workspace.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_variant_objects > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.workspace.design_management = candidate_design_management;
        }
        if self.workspace.project.root_library == library {
            self.workspace.project.root_library = new_name.to_owned();
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_library(new_name);
        Ok(remapped)
    }

    /// Remove one library and everything it owns. Every reason the removal is
    /// refused is reported on its own, before any state changes, because a
    /// partially deleted library is unrecoverable. Returns the cell count.
    pub(crate) fn delete_library(&mut self, library: &str) -> Result<usize, String> {
        self.require_library_deletable(library)?;
        let cells: Vec<String> = self
            .library_manager
            .get_library(library)
            .map(|target| {
                target
                    .cells_sorted()
                    .iter()
                    .map(|cell| cell.name.clone())
                    .collect()
            })
            .unwrap_or_default();

        let mut candidate_design_management = self.workspace.design_management.clone();
        let mut design_management_changed = false;
        for cell in &cells {
            let receipt = candidate_design_management
                .remove_sheet_catalogs_for_cell(library, cell)
                .map_err(|error| {
                    format!(
                        "Cannot delete library '{library}': Design Management still references cell '{library}/{cell}' ({error})."
                    )
                })?;
            design_management_changed |=
                receipt.affected_sheet_catalogs > 0 || receipt.remapped_annotation_objects > 0;
        }
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::DeleteLibrary {
                library: library.to_owned(),
            },
        )?;

        self.library_manager
            .remove_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the deletion"))?;
        if design_management_changed {
            self.workspace.design_management = candidate_design_management;
        }
        let owned_layouts = self
            .workspace
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library == library)
            .map(|document| document.owner().clone())
            .collect::<Vec<_>>();
        for owner in &owned_layouts {
            self.workspace.remove_physical_layout_document(owner);
        }
        self.prune_workspace_after_library_deleted(library);
        self.publish_project_library_mutation(project_mutation);

        Ok(cells.len())
    }

    /// The first reason this library cannot be deleted, or `None`. The
    /// deletion review asks exactly this question, so it can never promise a
    /// deletion the transaction then refuses.
    pub(crate) fn library_deletion_blocker(&self, library: &str) -> Option<String> {
        self.require_library_deletable(library).err()
    }

    fn require_library_deletable(&self, library: &str) -> Result<(), String> {
        let target = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; it cannot be deleted"
            ));
        }
        if self.workspace.project.root_library == library {
            return Err(format!(
                "Library '{library}' holds the project root cell '{}'. Repoint the project root before deleting it.",
                self.workspace.project.top_cell
            ));
        }
        let scope = format!("library '{library}'");
        require_no_configuration_roots(self, &scope, |root| root.library == library)?;
        let references = self.external_instance_references_to_library(library);
        if references > 0 {
            return Err(format!(
                "{references} loaded instance{} outside {scope} still reference a master in it. Replace or delete them before deleting the library.",
                if references == 1 { "" } else { "s" }
            ));
        }
        require_no_owned_sources(self, library)?;
        // Layouts this library owns go with it; a layout elsewhere that places
        // one of its masters would be left dangling, so that one blocks.
        let foreign_masters = self
            .workspace
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library != library)
            .filter(|document| {
                document
                    .instances()
                    .values()
                    .any(|instance| instance.master.library == library)
            })
            .count();
        if foreign_masters > 0 {
            return Err(format!(
                "{foreign_masters} physical layout document{} outside {scope} still place{} a master from it. Remove those placements first.",
                if foreign_masters == 1 { "" } else { "s" },
                if foreign_masters == 1 { "s" } else { "" }
            ));
        }
        Ok(())
    }

    /// Rename one view of one cell and remap the buffer, open references, and
    /// every instance binding that selected exactly that view. Returns the
    /// number of instance bindings remapped.
    pub(crate) fn rename_view(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let owner = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if owner.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        let owning_cell = owner
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{library}'"))?;
        if owning_cell.get_view(view).is_none() {
            return Err(format!("View '{library}/{cell}/{view}' not found"));
        }
        let requested_identity =
            crate::state::canonical_cell_view_owner_key(library, cell, new_name);
        if crate::state::canonical_cell_view_owner_key(library, cell, view) == requested_identity {
            return Err(format!(
                "View '{view}' already has that identity; a name differing only in case or normalization is the same view"
            ));
        }
        if owning_cell.views.iter().any(|(candidate_key, candidate)| {
            candidate_key != view
                && crate::state::canonical_cell_view_owner_key(library, cell, &candidate.name)
                    == requested_identity
        }) {
            return Err(format!(
                "View '{new_name}' conflicts with an existing canonical view identity in cell '{library}/{cell}'"
            ));
        }
        let candidate_layouts = self
            .workspace
            .prepare_rename_physical_layout_view_documents(library, cell, view, new_name)
            .map_err(|error| format!("Could not rename the view's physical layouts: {error}"))?;

        let mut candidate_configurations = self.workspace.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_view_roots(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's configuration-set roots: {error}")
            })?;
        let mut candidate_sources = self.workspace.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_view_bundles(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's project source ownership: {error}")
            })?;
        let mut candidate_design_management = self.workspace.design_management.clone();
        let design_management_receipt = candidate_design_management
            .rename_view_sheet_catalogs(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's Design Management ownership: {error}")
            })?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameView {
                library: library.to_owned(),
                cell: cell.to_owned(),
                from_view: view.to_owned(),
                to_view: new_name.to_owned(),
            },
        )?;

        let owning_cell = self
            .library_manager
            .get_library_mut(library)
            .and_then(|library| library.get_cell_mut(cell))
            .ok_or_else(|| format!("Cell '{library}/{cell}' disappeared during the rename"))?;
        let mut moved = owning_cell
            .views
            .remove(view)
            .ok_or_else(|| format!("View '{view}' disappeared during the rename"))?;
        moved.name = new_name.to_owned();
        owning_cell.views.insert(new_name.to_owned(), moved);

        let old_reference = CellViewRef::new(library, cell, view);
        let new_reference = CellViewRef::new(library, cell, new_name);
        if let Some(buffer) = self
            .workspace
            .schematic_buffers
            .remove(&old_reference.key())
        {
            self.workspace
                .schematic_buffers
                .insert(new_reference.key(), buffer);
        }
        self.workspace
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if *reference == old_reference {
                *reference = new_reference.clone();
            }
        };
        remap_ref(&mut self.workspace.active_view);
        for open in &mut self.workspace.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.remap_occurrence_masters(&remap_ref);

        let remapped = self.remap_instance_bindings(|binding| {
            let matched =
                binding.library == library && binding.cell == cell && binding.view == view;
            if matched {
                binding.view = new_name.to_owned();
            }
            matched
        });

        self.commit_renamed_project_sources(candidate_sources, &renamed_source_ids);
        if renamed_configuration_roots > 0 {
            self.workspace.configuration_sets = candidate_configurations;
            self.workspace.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.workspace.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_view(library, cell, new_name);
        Ok(remapped)
    }

    /// Rewrite the Library/Cell/View binding of every instance in every loaded
    /// buffer and in the live sheet. `rebind` edits the bindings it claims and
    /// reports whether it did, leaving the rest exactly as they were.
    ///
    /// The live sheet and the buffer under the active key are two copies of one
    /// document, so both move but only one is counted.
    fn remap_instance_bindings(
        &mut self,
        rebind: impl Fn(&mut crate::state::LibraryCellInstance) -> bool,
    ) -> usize {
        let active_key = self.workspace.active_key();
        let mut remapped = 0usize;
        let mut remap_schematic = |schematic: &mut SchematicState, counted: bool| {
            for component in &mut schematic.components {
                if let Some(binding) = component.library_cell.as_mut()
                    && rebind(binding)
                    && counted
                {
                    remapped += 1;
                }
            }
        };
        for (key, buffer) in &mut self.workspace.schematic_buffers {
            let counted = !key.eq_ignore_ascii_case(&active_key);
            remap_schematic(buffer, counted);
        }
        remap_schematic(&mut self.schematic, true);
        remapped
    }

    /// Loaded instances of one library's masters placed from outside it,
    /// counted once per instance.
    ///
    /// A library's own cells instancing each other is ordinary hierarchy and
    /// goes with the library; only a placement that would survive it is a
    /// reason to refuse the deletion, and it is the number the review states.
    pub(crate) fn external_instance_references_to_library(&self, library: &str) -> usize {
        let active_key = self.workspace.active_key();
        let owned_prefix = format!("{library}/");
        let count = |schematic: &SchematicState| {
            schematic
                .components
                .iter()
                .filter(|component| {
                    component
                        .library_cell
                        .as_ref()
                        .is_some_and(|binding| binding.library == library)
                })
                .count()
        };
        let live = if self.workspace.active_view.library == library {
            0
        } else {
            count(&self.schematic)
        };
        live + self
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| {
                !key.eq_ignore_ascii_case(&active_key) && !key.starts_with(&owned_prefix)
            })
            .map(|(_, schematic)| count(schematic))
            .sum::<usize>()
    }

    /// Adopt a prepared source registry and drop transient Verilog-A evidence
    /// that named a bundle whose owner just moved.
    fn commit_renamed_project_sources(
        &mut self,
        candidate: crate::state::ProjectSourceRegistry,
        renamed: &[crate::state::ProjectSourceId],
    ) {
        if renamed.is_empty() {
            return;
        }
        self.workspace.project_sources = candidate;
        self.workspace.project_sources_dirty = true;
        let transient_uses_renamed = self
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .is_some_and(|receipt| renamed.contains(&receipt.token.bundle_id))
            || self
                .ui
                .code_workspace
                .veriloga
                .pending
                .as_ref()
                .is_some_and(|pending| renamed.contains(&pending.token.bundle_id));
        if transient_uses_renamed {
            self.ui.code_workspace.veriloga = Default::default();
        }
    }

    fn prune_workspace_after_library_deleted(&mut self, library: &str) {
        self.sync_active_schematic_to_workspace();
        let active_removed = self.workspace.active_view.library == library;
        let prefix = format!("{library}/");
        self.workspace
            .schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .open_views
            .retain(|open| open.reference.library != library);
        let hierarchy_pruned = self
            .workspace
            .retain_valid_occurrences(|reference| reference.library != library);
        self.restore_valid_workspace_focus_after_prune(
            active_removed,
            hierarchy_pruned,
            false,
            true,
        );
    }

    /// Refuse an edit on a read-only view, with the console line that names
    /// the library. Returns true when the edit must be blocked.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        if !self.schematic_edit_read_only() {
            return false;
        }
        let live_lock_holder = self
            .workbench
            .live_write_locks
            .schematic_views
            .get(&self.workspace.active_key())
            .cloned();
        let message = if self.workbench.safe_mode.project_read_only() {
            "Safe mode is read-only; no design data was changed.".to_owned()
        } else if self.active_view_read_only() {
            self.read_only_master_message()
        } else if let Some(holder) = live_lock_holder {
            format!("{holder} holds the write lease on this schematic; no design data was changed.")
        } else if self.workbench.live_write_locks.mirror {
            "This is the host's working copy; request the write lease from the live session \
             to edit."
                .to_owned()
        } else {
            "The active schematic is read-only; no design data was changed.".to_owned()
        };
        self.push_user_message(ConsoleMessage::warning(message));
        true
    }

    /// Ascend one hierarchy level (the U gesture / pathbar action).
    pub(crate) fn ascend_workspace_level(&mut self) {
        let depth = self.workspace.occurrence_depth();
        if depth >= 2 {
            self.focus_workspace_breadcrumb(depth - 2);
        }
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        self.sync_active_schematic_to_workspace();
        if let Some(reference) = self.workspace.focus_breadcrumb(index) {
            self.library_manager
                .select_view(&reference.library, &reference.cell, &reference.view);
            self.schematic = schematic_for_workspace(self, &reference);
            self.bump_active_schematic_epoch();
            self.refresh_active_design_check_projection();
        }
    }

    pub(crate) fn bump_active_schematic_epoch(&mut self) {
        self.active_schematic_epoch = self.active_schematic_epoch.wrapping_add(1);
    }

    /// Return the exact schematic master of one hierarchy instance.
    ///
    /// A generic `CellInstance` can also bind a SPICE, symbol, or Verilog-A
    /// view. Those bindings are valid design objects, but they are not
    /// descendable hierarchy and must not enable the descend command.
    pub(crate) fn hierarchy_master_for_component(
        &self,
        component_id: u64,
    ) -> Option<(String, CellViewRef)> {
        let component = self
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)?;
        if component.kind != ComponentType::CellInstance {
            return None;
        }
        let binding = component.library_cell.as_ref()?;
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        let view_type = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| view.view_type)?;
        (view_type == ViewType::Schematic).then(|| (component.name.clone(), reference))
    }

    /// Return the exact project-owned Verilog-A source view bound to one
    /// schematic instance. Merely spelling a view `veriloga` is insufficient:
    /// the library view, cell-view-owned source bundle, and selected module
    /// contract must all resolve before navigation is offered.
    pub(crate) fn veriloga_source_for_component(&self, component_id: u64) -> Option<CellViewRef> {
        let component = self
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)?;
        if component.kind != ComponentType::CellInstance {
            return None;
        }
        let binding = component.library_cell.as_ref()?;
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        crate::state::workspace::project_veriloga_binding_for_view(
            &self.workspace,
            &self.library_manager,
            &reference,
        )
        .ok()?;
        Some(reference)
    }

    /// Open the exact Verilog-A source owned by a bound schematic instance.
    /// Resolution happens before any workspace mutation, so stale or partial
    /// bindings fail closed and leave the active document untouched.
    pub(crate) fn open_veriloga_source_for_component(&mut self, component_id: u64) -> bool {
        let Some(reference) = self.veriloga_source_for_component(component_id) else {
            return false;
        };
        self.open_workspace_view(reference);
        self.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        self.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA;
        true
    }

    pub(crate) fn selected_hierarchy_master(&self) -> Option<(String, CellViewRef)> {
        self.schematic
            .selection
            .single_component()
            .and_then(|component_id| self.hierarchy_master_for_component(component_id))
    }

    pub(crate) fn hierarchical_edit_in_place_enabled(&self) -> bool {
        self.ui
            .preferences
            .toggle(crate::workbench::TogglePreference::HierarchicalEditInPlace)
    }

    pub(crate) fn open_selected_instance_master(&mut self) {
        let Some((instance_name, reference)) = self.selected_hierarchy_master() else {
            self.push_user_message(ConsoleMessage::warning(
                "Select one instance with a resolved schematic master first",
            ));
            return;
        };
        if self.hierarchical_edit_in_place_enabled() {
            self.descend_into_instance(Some(instance_name), reference);
        } else {
            self.open_workspace_view(reference);
        }
    }

    /// Why the selected instance cannot be shown in the generated deck, or
    /// `None` when it can. Every surface that offers the jump asks this one
    /// question, so a disabled row, a disabled palette entry and the refusal
    /// the action itself would report cannot word the same block differently.
    pub(crate) fn selected_instance_netlist_block(&self) -> Option<&'static str> {
        selected_instance_generated_line(self).err()
    }

    /// Open the generated deck at the selected instance's card, the way
    /// [`Self::open_selected_instance_master`] opens its schematic master.
    ///
    /// The Netlist workspace, the generated primary document and the caret
    /// move as one transaction, and the console states where the instance was
    /// found. The schematic selection is deliberately left alone: the netlist
    /// navigator's reveal reads it to bring the round trip back.
    pub(crate) fn show_selected_instance_in_netlist(&mut self) {
        let Ok(location) = selected_instance_generated_line(self) else {
            return;
        };
        crate::workbench::documents::netlist_document::open_generated_primary(self);
        self.ui.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
        self.workbench
            .activate(crate::workbench::state::Workspace::Netlist);
        self.ui.netlist.cursor_line = location.line.saturating_sub(1);
        self.ui.netlist.requested_line = Some(location.line);
        let announced = self.ui.messages().format(
            if location.stale {
                crate::workbench::MessageId::NetlistShowInstanceStale
            } else {
                crate::workbench::MessageId::NetlistShowInstanceLocated
            },
            &[
                ("instance", location.instance.as_str()),
                ("line", &location.line.to_string()),
            ],
        );
        self.push_user_message(ConsoleMessage::info(announced));
    }
}

/// Where one selected schematic instance appears in the retained generated
/// deck, and whether that deck still matches the project it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedInstanceLocation {
    /// One-based generated line carrying the instance's own card.
    line: usize,
    /// Emitted SPICE identity written on that line.
    instance: String,
    /// Whether the retained artifact predates the current project input.
    stale: bool,
}

/// Locate the selected schematic instance in the retained generated deck.
///
/// The error is the exact reason the jump is unavailable, worded for the
/// disabled control that reports it, so availability and execution can never
/// disagree about why. A stale artifact still resolves — it is what the
/// project last generated — and the caller says so rather than refusing.
fn selected_instance_generated_line(
    state: &AppState,
) -> Result<GeneratedInstanceLocation, &'static str> {
    const NO_DECK: &str = "no generated netlist yet — open the Netlist workspace to generate one";

    let component_id = state
        .schematic
        .selection
        .single_component()
        .ok_or("select one instance")?;
    if state.ui.netlist.generated_source.trim().is_empty() {
        return Err(NO_DECK);
    }
    let artifact = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(crate::state::NetlistDocument::generated_artifact)
        .ok_or(NO_DECK)?;
    let identity = crate::state::GeneratedSourceMapEntry::component_identity_for(
        &state.workspace.active_view.key(),
        component_id,
    );
    let line = artifact
        .generated_lines_for_component(&identity)
        .first()
        .copied()
        .ok_or("no netlist line for this instance")?;
    Ok(GeneratedInstanceLocation {
        line,
        instance: artifact
            .source_map_entry(line)
            .and_then(crate::state::GeneratedSourceMapEntry::instance_identity)
            .unwrap_or_default()
            .to_owned(),
        stale: state.ui.netlist.generated_input_digest
            != state.ui.netlist.current_generation_input_digest,
    })
}

impl RSpiceApp {
    pub(crate) fn restore_workspace_after_project_load(&mut self) {
        self.state.restore_active_schematic_from_workspace();
        self.state.clear_transient_specialized_viewer_data();
    }
}

/// The existing library whose canonical identity `name` would collide with,
/// ignoring one map key the caller is about to vacate.
fn conflicting_library_name(
    libraries: &crate::state::LibraryManager,
    name: &str,
    vacating: Option<&str>,
) -> Option<String> {
    let requested = crate::state::canonical_cell_view_owner_key(name, "", "");
    libraries
        .libraries_by_key()
        .find(|(key, library)| {
            Some(*key) != vacating
                && crate::state::canonical_cell_view_owner_key(&library.name, "", "") == requested
        })
        .map(|(_, library)| library.name.clone())
}

/// Refuse an operation while a configuration set still names the scope as its
/// executable root, so a valid catalog can never be left dangling.
fn require_no_configuration_roots(
    state: &AppState,
    scope: &str,
    matches: impl Fn(&CellViewRef) -> bool,
) -> Result<(), String> {
    let names = state
        .workspace
        .configuration_sets
        .configurations()
        .iter()
        .filter(|configuration| matches(configuration.root()))
        .map(|configuration| configuration.name().to_owned())
        .collect::<Vec<_>>();
    if names.is_empty() {
        return Ok(());
    }
    let mut listed = names.iter().take(4).cloned().collect::<Vec<_>>().join(", ");
    if names.len() > 4 {
        listed.push_str(&format!(" and {} more", names.len() - 4));
    }
    Err(format!(
        "Configuration set roots still reference {scope} ({listed}). Rebind or remove those configurations first."
    ))
}

/// Refuse a library deletion while it owns a project source bundle or a
/// Verilog-A view, because every Verilog-A view must own exactly one persisted
/// source closure and a deleted library takes both with it.
fn require_no_owned_sources(state: &AppState, library: &str) -> Result<(), String> {
    let bundles = state
        .workspace
        .project_sources
        .iter_bundles()
        .filter(|bundle| {
            matches!(
                bundle.owner(),
                crate::state::ProjectSourceOwner::CellView { reference }
                    if reference.library == library
            )
        })
        .count();
    if bundles > 0 {
        return Err(format!(
            "Library '{library}' owns {bundles} project source bundle{}. Delete those views first.",
            if bundles == 1 { "" } else { "s" }
        ));
    }
    let veriloga_views = state
        .library_manager
        .get_library(library)
        .map_or(0, |library| {
            library
                .cells
                .values()
                .flat_map(|cell| cell.views.values())
                .filter(|view| view.view_type == ViewType::VerilogA)
                .count()
        });
    if veriloga_views > 0 {
        return Err(format!(
            "Library '{library}' owns {veriloga_views} Verilog-A view{}. Delete those views first.",
            if veriloga_views == 1 { "" } else { "s" }
        ));
    }
    Ok(())
}

pub(super) fn parse_encoded_ports(encoded: &str) -> Vec<PortSpec> {
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

#[cfg(test)]
mod tests;
