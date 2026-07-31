//! The symbol editor's document lifecycle.
//!
//! Loading the active cell's symbol document and its editor metadata,
//! writing edits back, and the undo/redo stack over both. Symbol undo is
//! separate from schematic undo because a symbol is a different document
//! with its own edit history, even though the two share a workspace.
//!
//! Also here: the pin checks that compare a symbol's ports against the
//! schematic that instantiates it, since they read the same document.

use crate::diagnostics::{ConsoleMessage, LogSeverity, LogSource};
use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::state::{
    CellViewRef, MAX_SYMBOL_DOCUMENT_BYTES, PinFindingKind, PortSpec, SYMBOL_DOCUMENT_METADATA_KEY,
    SYMBOL_EDITOR_METADATA_KEY, SymbolDocument, SymbolEditorMetadata,
};
use crate::workbench::SymbolDocumentSnapshot;
use crate::workbench::app_state::AppState;

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

    /// Atomically publish symbol geometry and its authoring-only metadata.
    pub(crate) fn store_active_symbol_editor_bundle(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
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
        let current_snapshot = self.active_symbol_metadata_snapshot(&current);
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
        let current_snapshot = self.active_symbol_metadata_snapshot(&current);
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
