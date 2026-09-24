//! The symbol editor's document lifecycle.
//!
//! Loading the active cell's symbol document and its editor metadata,
//! writing edits back, and the undo/redo stack over both. Symbol undo is
//! separate from schematic undo because a symbol is a different document
//! with its own edit history, even though the two share a workspace.
//!
//! Also here: the pin checks that compare a symbol's ports against the
//! schematic that instantiates it, since they read the same document.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use crate::diagnostics::{ConsoleMessage, LogSeverity, LogSource};
use crate::schematic::symbol_editor::{SymbolSaveCheck, symbol_save_checks};
use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::state::{
    CellViewRef, MAX_SYMBOL_DOCUMENT_BYTES, ModelBoundSymbolDefinition, PinFindingKind, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SchematicState, SymbolDocument,
    SymbolEditorMetadata,
};
use crate::workbench::app_state::AppState;
use crate::workbench::{SymbolCommitIntent, SymbolDocumentSnapshot};

use super::{
    MAX_FINDING_ROWS, log_severity_from_drc, parse_encoded_ports, remap_symbol_instance_wires,
    restore_symbol_snapshot_in_view, symbol_metadata_snapshot_from_view,
    symbol_pin_position_remaps, symbol_snapshot_from_view,
};

impl AppState {
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

    pub(crate) fn load_active_symbol_editor_metadata(
        &self,
        document: &SymbolDocument,
    ) -> Result<SymbolEditorMetadata, String> {
        let reference = &self.workspace.active_view;
        let Some(view) = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        SymbolEditorMetadata::load_from_view(view, document)
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
            let pin_remaps = symbol_pin_position_remaps(
                &previous_document,
                &snapshot.document,
                &snapshot.renames,
            );
            for schematic in self.workspace.schematic_buffers.values_mut() {
                remap_symbol_instance_wires(schematic, &reference, &pin_remaps);
            }
            remap_symbol_instance_wires(&mut self.schematic, &reference, &pin_remaps);
        }
        self.apply_symbol_pin_renames(&reference, &snapshot.renames);
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
            let pin_remaps =
                symbol_pin_position_remaps(&previous_document, document, &BTreeMap::new());
            for schematic in self.workspace.schematic_buffers.values_mut() {
                remap_symbol_instance_wires(schematic, &reference, &pin_remaps);
            }
            remap_symbol_instance_wires(&mut self.schematic, &reference, &pin_remaps);
        }
        self.workspace.set_active_dirty(true);
        Ok(())
    }

    /// Atomically publish symbol geometry and its authoring-only metadata.
    pub(crate) fn store_active_symbol_editor_bundle(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<(), String> {
        self.commit_active_symbol_edit(document, metadata, &SymbolCommitIntent::default())
    }

    /// Publish symbol geometry and metadata, carrying what the author meant.
    ///
    /// The intent is the half of the edit that comparing two documents
    /// cannot recover. A pin rename reaches every instance from here and
    /// nowhere else, because this is the one point at which the old name and
    /// the new one are both still known.
    pub(crate) fn commit_active_symbol_edit(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
        intent: &SymbolCommitIntent,
    ) -> Result<(), String> {
        let reference = self.workspace.active_view.clone();
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        document.validate()?;
        let encoded_document = serde_json::to_string(document)
            .map_err(|error| format!("Could not serialize symbol metadata: {error}"))?;
        if encoded_document.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Could not serialize symbol metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                encoded_document.len()
            ));
        }
        let encoded_editor = metadata.encode()?;
        let previous_document = self.load_active_symbol_document().ok();
        let Some(view) = self
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        view.metadata
            .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), encoded_document);
        view.metadata
            .insert(SYMBOL_EDITOR_METADATA_KEY.to_owned(), encoded_editor);
        view.metadata.remove("generated");
        view.metadata.remove("ports");
        view.modified = true;
        if let Some(previous_document) = previous_document {
            let pin_remaps =
                symbol_pin_position_remaps(&previous_document, document, &intent.renames);
            for schematic in self.workspace.schematic_buffers.values_mut() {
                remap_symbol_instance_wires(schematic, &reference, &pin_remaps);
            }
            remap_symbol_instance_wires(&mut self.schematic, &reference, &pin_remaps);
        }
        let renamed = self.apply_symbol_pin_renames(&reference, &intent.renames);
        if renamed > 0 {
            self.push_user_message(ConsoleMessage::info(format!(
                "Renamed {renamed} instance terminal(s) across placed instances of {}",
                reference.display_path()
            )));
        }
        // The undo entry this edit sits on top of has to be able to put the
        // names back, so it carries the reverse of what was just applied.
        if !intent.renames.is_empty() {
            let key = self.workspace.active_key();
            if let Some(entry) = self
                .ui
                .symbol
                .undo_stacks
                .get_mut(&key)
                .and_then(|stack| stack.last_mut())
            {
                entry.renames = intent
                    .renames
                    .iter()
                    .map(|(from, to)| (to.clone(), from.clone()))
                    .collect();
            }
        }
        self.workspace.set_active_dirty(true);
        Ok(())
    }

    /// The model definition the active symbol view is bound to, if any.
    fn active_model_bound_symbol_definition(&self) -> Option<ModelBoundSymbolDefinition> {
        let reference = &self.workspace.active_view;
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| {
                ModelBoundSymbolDefinition::load_from_view(view)
                    .ok()
                    .flatten()
            })
    }

    /// The publication contract of the active symbol view.
    pub(crate) fn active_symbol_save_checks(
        &self,
        document: &SymbolDocument,
        ports: &[PortSpec],
    ) -> Vec<SymbolSaveCheck> {
        let definition = self.active_model_bound_symbol_definition();
        symbol_save_checks(
            definition.as_ref(),
            &self.workspace.active_view.cell,
            document,
            ports,
        )
    }

    /// The first contract row that refuses publication, if any.
    ///
    /// This is the headless half of the save transaction: the command runs it
    /// before the dialog exists, so a symbol that cannot be published is
    /// refused by name rather than by a disabled button.
    pub(crate) fn refuse_unsavable_active_symbol(&self) -> Option<String> {
        let ports = self.active_symbol_ports();
        let document = match self.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => return Some(error),
        };
        self.active_symbol_save_checks(&document, &ports)
            .into_iter()
            .find(|check| !check.passed)
            .map(|check| check.refusal())
    }

    /// Publish a symbol revision, refusing on the first contract row that
    /// fails.
    ///
    /// The refusal is the point: the dialog disables its primary control on
    /// the same rows, so a command that published without consulting them
    /// would be a second, more permissive save path for the same document.
    pub(crate) fn publish_active_symbol_revision(
        &mut self,
        document: &SymbolDocument,
        metadata: &mut SymbolEditorMetadata,
        revision_note: &str,
    ) -> Result<u64, String> {
        let ports = self.active_symbol_ports();
        if let Some(failed) = self
            .active_symbol_save_checks(document, &ports)
            .into_iter()
            .find(|check| !check.passed)
        {
            return Err(failed.refusal());
        }
        let mut candidate = metadata.clone();
        let revision = candidate.publish_revision(document, revision_note)?;
        self.store_active_symbol_editor_bundle(document, &candidate)?;
        *metadata = candidate;
        let key = self.workspace.active_key();
        self.ui.symbol.mark_save_point(key);
        Ok(revision)
    }

    /// Bring the active symbol's pins back into agreement with the interface
    /// its schematic declares, without touching a single body shape.
    ///
    /// Returns the console line describing what changed.
    pub(crate) fn update_active_symbol_pins_from_contract(&mut self) -> Result<String, String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let ports = self.active_symbol_ports();
        if ports.is_empty() {
            return Err(
                "No schematic interface declares this cell, so there is no contract to update from."
                    .to_owned(),
            );
        }
        let before = self.load_active_symbol_document()?;
        let metadata = self.load_active_symbol_editor_metadata(&before)?;
        let mut document = before.clone();
        document.reconcile_ports(&ports);
        if document == before {
            return Ok("Symbol pins already match the interface contract".to_owned());
        }
        let added = document.pins.len().saturating_sub(before.pins.len());
        let orphaned = document
            .pin_findings(&ports)
            .into_iter()
            .filter(|finding| finding.kind == PinFindingKind::OrphanedPin)
            .count();
        self.record_symbol_edit(&before);
        self.store_active_symbol_editor_bundle(&document, &metadata)?;
        Ok(format!(
            "Symbol pins updated from the interface contract: {added} added, {orphaned} left orphaned"
        ))
    }

    /// The refusal message for the symbol editor's read-only banner, naming
    /// which of the four owners holds the lock.
    pub(crate) fn symbol_editor_lock_message(&self) -> String {
        if let Some(holder) = self
            .workbench
            .live_write_locks
            .schematic_views
            .get(&self.workspace.active_key())
        {
            return format!("{holder} holds the write lease on this symbol");
        }
        if self.workbench.live_write_locks.mirror {
            return "This is the host's working copy; request the write lease from the live session to edit."
                .to_owned();
        }
        self.read_only_master_message()
    }

    /// Whether copying the cell into an editable library is a route out of
    /// the lock. It is not one when the whole project refuses writes.
    pub(crate) fn symbol_editor_copy_available(&self) -> bool {
        !self.workbench.safe_mode.project_read_only()
            && !self.workbench.live_write_locks.mirror
            && !self.workspace.active_read_only_reference()
    }

    /// Carry a pin rename to every placement of the cellview.
    ///
    /// An instance wires to a pin by name, in `WireConnection::terminal_name`
    /// and in the binding's `terminal_order`. Leaving either behind after a
    /// rename does not merely mislabel the connection — it detaches it, and
    /// the netlist emits the instance with a terminal nothing drives.
    /// Returns how many terminal names were rewritten.
    fn apply_symbol_pin_renames(
        &mut self,
        reference: &CellViewRef,
        renames: &BTreeMap<String, String>,
    ) -> usize {
        if renames.is_empty() {
            return 0;
        }
        let lowered: BTreeMap<String, String> = renames
            .iter()
            .map(|(from, to)| (from.to_ascii_lowercase(), to.clone()))
            .collect();
        let mut renamed = 0;
        for schematic in self.workspace.schematic_buffers.values_mut() {
            renamed += rename_instance_terminals(schematic, reference, &lowered);
        }
        renamed += rename_instance_terminals(&mut self.schematic, reference, &lowered);
        renamed
    }

    pub(crate) fn record_symbol_edit(&mut self, before: &SymbolDocument) {
        const MAX_SYMBOL_UNDO: usize = 128;
        let key = self.workspace.active_key();
        let snapshot = self.active_symbol_snapshot(before);
        self.push_symbol_undo_snapshot(key, snapshot, MAX_SYMBOL_UNDO);
    }

    pub(super) fn record_symbol_metadata_edit(&mut self, before: &SymbolDocument) {
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
        let undo_stack = self.ui.symbol.undo_stacks.entry(key.clone()).or_default();
        undo_stack.push(snapshot);
        if undo_stack.len() > max_len {
            undo_stack.remove(0);
        }
        self.ui.symbol.redo_stacks.remove(&key);
    }

    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        self.ui
            .symbol
            .undo_stacks
            .get(&self.workspace.active_key())
            .is_some_and(|stack| !stack.is_empty())
    }

    pub(crate) fn can_redo_active_symbol_document(&self) -> bool {
        self.ui
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
        let Some(previous) = self.ui.symbol.undo_stacks.get_mut(&key).and_then(Vec::pop) else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let mut current_snapshot = self.active_symbol_metadata_snapshot(&current);
        current_snapshot.renames = previous.inverted_renames();
        self.ui
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
        let Some(next) = self.ui.symbol.redo_stacks.get_mut(&key).and_then(Vec::pop) else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let mut current_snapshot = self.active_symbol_metadata_snapshot(&current);
        current_snapshot.renames = next.inverted_renames();
        self.ui
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
                        format!("+{hidden} more findings - use the finding navigation commands"),
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
}

/// Rewrite the terminal names of every instance of `reference` in one sheet.
///
/// `renames` is keyed by lowercased old name; instance bindings and wire
/// connections both match terminals case-insensitively, so both are rewritten
/// from the same table and cannot drift apart.
fn rename_instance_terminals(
    schematic: &mut SchematicState,
    reference: &CellViewRef,
    renames: &BTreeMap<String, String>,
) -> usize {
    let instances: Vec<u64> = schematic
        .components
        .iter()
        .filter(|component| {
            component.library_cell.as_ref().is_some_and(|binding| {
                binding.library.eq_ignore_ascii_case(&reference.library)
                    && binding.cell.eq_ignore_ascii_case(&reference.cell)
            })
        })
        .map(|component| component.id)
        .collect();
    if instances.is_empty() {
        return 0;
    }

    let mut renamed = 0;
    for component in &mut schematic.components {
        let Some(binding) = component.library_cell.as_mut() else {
            continue;
        };
        if !instances.contains(&component.id) {
            continue;
        }
        for terminal in &mut binding.terminal_order {
            if let Some(new_name) = renames.get(&terminal.to_ascii_lowercase()) {
                *terminal = new_name.clone();
            }
        }
    }
    for connection in &mut schematic.connections {
        if !instances.contains(&connection.component_id) {
            continue;
        }
        if let Some(new_name) = renames.get(&connection.terminal_name.to_ascii_lowercase()) {
            connection.terminal_name = new_name.clone();
            renamed += 1;
        }
    }
    if renamed > 0 {
        schematic.is_dirty = true;
        schematic.bump_topology_version();
    }
    renamed
}
