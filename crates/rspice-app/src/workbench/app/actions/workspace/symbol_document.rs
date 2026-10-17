//! The symbol editor's document lifecycle.
//!
//! Loading the active cell's symbol document and its editor metadata,
//! writing edits back, and the undo/redo stack over both. Symbol undo is
//! separate from schematic undo because a symbol is a different document
//! with its own edit history, even though the two share a workspace.
//!
//! Also here: the pin checks that compare a symbol's ports against the
//! schematic that instantiates it, since they read the same document, and
//! the publication of what they find as canonical records. Their findings
//! used to stop at a field beside the canvas, so the one list of every
//! standing condition held none of them.

#[cfg(test)]
mod tests;

use rspice_schematic_editor::symbol_editor::interaction::SymbolRequestSource;
use std::collections::BTreeMap;

use crate::diagnostics::{LogAnchor, LogSeverity, LogSource};
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, PinFindingKind, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SymbolDocument, SymbolEditorMetadata,
};
use crate::ui::accessibility::counted;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_checks::location_key;
use crate::workbench::app_state::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};
use crate::workbench::{SymbolCommitIntent, SymbolDocumentSnapshot};
use rspice_design::symbol::publication::{
    EncodedSymbolEditorBundle, SymbolSaveCheck, check_symbol_pins, symbol_save_checks,
};

use rspice_design::library::metadata::{metadata_ports, module_interface_view};
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::history::SchematicSnapshot;
use rspice_design::symbol::edit::{
    SymbolParentEdit, SymbolParentWiring, places_symbol, resolve_symbol_instance_wiring,
    symbol_metadata_snapshot_from_view, symbol_pin_position_remaps, symbol_snapshot_from_view,
};
use rspice_design::symbol::generated::{GENERATED_PORTS_KEY, decode_ports};

/// Producer identity every symbol pin finding carries.
const SYMBOL_PIN_PRODUCER: &str = "rspice.design.symbol.pins";

impl AppState {
    pub(crate) fn symbol_editor_request_source(&self) -> SymbolRequestSource {
        SymbolRequestSource {
            project: self.workspace.content.project.id(),
            document: self.workspace.content.active_view.clone(),
            occurrence: self.workspace.content.active_occurrence().cloned(),
            design_epoch: self.design_execution_epoch,
            document_epoch: self.active_schematic_epoch,
            library_revision: self.library_manager.revision(),
            window: self.workbench.window_session.current().value(),
        }
    }

    /// Project one retained view session into the active editor slot. Navigation
    /// advances request authority; it does not turn passive rendering into an edit.
    pub(crate) fn project_active_symbol_session(&mut self) {
        use rspice_schematic_editor::symbol_editor::interaction::{
            bind_canvas_source, resume_canvas_source,
        };

        let source = self.symbol_editor_request_source();
        let is_symbol = self.workspace.content.active_view_type() == crate::state::ViewType::Symbol;
        let open_views = &self.workspace.content.open_views;
        let retained = &mut self.workspace.session.symbol_sessions;
        let still_open = |previous: &SymbolRequestSource| {
            previous.project == source.project
                && previous.design_epoch == source.design_epoch
                && open_views
                    .iter()
                    .any(|open| open.reference == previous.document)
        };
        retained.retain(|_, session| session.canvas_source.as_ref().is_some_and(still_open));
        let same_document = self
            .ui
            .symbol
            .editor
            .canvas_source
            .as_ref()
            .is_some_and(|previous| {
                previous.project == source.project
                    && previous.design_epoch == source.design_epoch
                    && previous.document == source.document
            });
        if !is_symbol || !same_document {
            if self
                .ui
                .symbol
                .editor
                .canvas_source
                .as_ref()
                .is_some_and(|previous| previous.window == source.window)
            {
                // User navigation ends active input. Projecting a different
                // window keeps it with the window that still owns it.
                self.ui.symbol.editor.clear_drag_state();
                self.ui.symbol.editor.marquee_start = None;
                self.ui.symbol.editor.marquee_current = None;
                self.ui.symbol.editor.inspector.end_field_edit();
            }
            // Clipboard contents travel across views; every other field is view-local.
            let clipboard = std::mem::take(&mut self.ui.symbol.editor.clipboard);
            let previous = std::mem::take(&mut self.ui.symbol.editor);
            if let Some(previous_source) = &previous.canvas_source
                && still_open(previous_source)
            {
                retained.insert(previous_source.document.key(), previous);
            }
            if is_symbol && let Some(mut session) = retained.remove(&source.document.key()) {
                resume_canvas_source(&mut session, source.clone());
                self.ui.symbol.editor = session;
            }
            self.ui.symbol.editor.clipboard = clipboard;
        }
        if is_symbol {
            bind_canvas_source(&mut self.ui.symbol.editor, source);
        }
    }

    pub(crate) fn active_symbol_ports(&self) -> Vec<PortSpec> {
        self.symbol_ports(&self.workspace.content.active_view)
    }

    /// The interface contract one cell's symbol is checked against.
    ///
    /// Addressed by reference rather than only by what is in front of the
    /// reader, because the currency of a published pin finding is decided by
    /// the contract that finding was derived from — and navigating away from
    /// a symbol does not change that symbol's contract.
    pub(crate) fn symbol_ports(&self, reference: &CellViewRef) -> Vec<PortSpec> {
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.content.active_view == schematic_ref {
            return self.schematic.document().interface_ports();
        }
        if let Some(schematic) = self
            .workspace
            .content
            .schematic_buffers
            .get(&schematic_ref.key())
        {
            let ports = schematic.document().interface_ports();
            if !ports.is_empty() {
                return ports;
            }
        }

        let cell = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell));
        // With no schematic, the module's last clean compile is the contract.
        if let Some(ports) = cell
            .and_then(module_interface_view)
            .and_then(|view| metadata_ports(&view.metadata))
            .filter(|ports| !ports.is_empty())
        {
            return ports;
        }
        cell.and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| view.metadata.get(GENERATED_PORTS_KEY))
            .map(|encoded| decode_ports(encoded))
            .unwrap_or_default()
    }

    /// The [stamp](rspice_design::stamped) of the drawing [`Self::symbol_ports`]
    /// reads the contract from first, or `None` where it reads only the
    /// library, whose revision covers the rest.
    ///
    /// Beside [`Self::symbol_ports`] because it must take the same branches:
    /// a stamp of any other drawing would let an edit to the contract leave
    /// the pin findings reading current.
    pub(in crate::workbench) fn symbol_contract_stamp(
        &self,
        reference: &CellViewRef,
    ) -> Option<u64> {
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.content.active_view == schematic_ref {
            return Some(self.schematic.content_stamp());
        }
        self.workspace
            .content
            .schematic_buffers
            .get(&schematic_ref.key())
            .map(|schematic| schematic.content_stamp())
    }

    pub(crate) fn load_active_symbol_document(&self) -> Result<SymbolDocument, String> {
        self.load_symbol_document(&self.workspace.content.active_view)
    }

    /// One cell's symbol artwork, as published in the library.
    pub(crate) fn load_symbol_document(
        &self,
        reference: &CellViewRef,
    ) -> Result<SymbolDocument, String> {
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
        let ports = self.symbol_ports(reference);
        if view.metadata.contains_key("generated") && !ports.is_empty() {
            return Ok(SymbolDocument::generated_from_ports(&ports));
        }
        Ok(document)
    }

    pub(crate) fn load_active_symbol_editor_metadata(
        &self,
        document: &SymbolDocument,
    ) -> Result<SymbolEditorMetadata, String> {
        self.load_symbol_editor_metadata(&self.workspace.content.active_view, document)
    }

    /// One cell's symbol authoring metadata, which is where its published
    /// revision is recorded.
    pub(crate) fn load_symbol_editor_metadata(
        &self,
        reference: &CellViewRef,
        document: &SymbolDocument,
    ) -> Result<SymbolEditorMetadata, String> {
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
        let reference = &self.workspace.content.active_view;
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
        let reference = &self.workspace.content.active_view;
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
        let reference = self.workspace.content.active_view.clone();
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        self.with_symbol_parent_edit(|state| {
            let Some(()) = state
                .library_manager
                .edit_library(&reference.library)
                .and_then(|mut library| {
                    library.restore_symbol_snapshot(&reference.cell, &reference.view, snapshot)
                })
            else {
                return Err(format!("View '{}' not found", reference.display_path()));
            };
            state.apply_symbol_parent_edit(&snapshot.document, &snapshot.renames);
            state.apply_symbol_pin_renames(&reference, &snapshot.renames);
            Ok(())
        })?;
        self.workspace.content.set_active_dirty(true);
        Ok(())
    }

    pub(crate) fn store_active_symbol_document(
        &mut self,
        document: &SymbolDocument,
    ) -> Result<(), String> {
        let reference = self.workspace.content.active_view.clone();
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        self.with_symbol_parent_edit(|state| {
            let Some(result) = state
                .library_manager
                .edit_library(&reference.library)
                .and_then(|mut library| {
                    library.store_symbol_document(&reference.cell, &reference.view, document)
                })
            else {
                return Err(format!("View '{}' not found", reference.display_path()));
            };
            result?;
            state.apply_symbol_parent_edit(document, &BTreeMap::new());
            Ok(())
        })?;
        self.workspace.content.set_active_dirty(true);
        Ok(())
    }

    /// Run one complete symbol edit: its parent wiring is resolved before it
    /// and recorded in each parent's history after it.
    fn with_symbol_parent_edit<R>(&mut self, edit: impl FnOnce(&mut Self) -> R) -> R {
        self.begin_symbol_parent_edit();
        let result = edit(self);
        self.finish_symbol_parent_edit();
        result
    }

    /// Hold, in every open schematic that places the active symbol's cell,
    /// the wire vertices each instance terminal holds now.
    ///
    /// Any edit still held open is finished first: its end never arrived, and
    /// resolving over it would read its half-moved vertices as the baseline.
    pub(crate) fn begin_symbol_parent_edit(&mut self) {
        self.finish_symbol_parent_edit();
        let Ok(base) = self.load_active_symbol_document() else {
            return;
        };
        let reference = self.workspace.content.active_view.clone();
        let content = &self.workspace.content;
        let mut parents: Vec<SymbolParentWiring> = content
            .schematic_buffers
            .iter()
            .filter_map(|(key, design)| {
                parent_wiring(
                    Some(key.clone()),
                    design.document(),
                    content.design_management.sheet_catalog(key),
                    &reference,
                    &base,
                )
            })
            .collect();
        let active_key = content.active_schematic_reference().key();
        parents.extend(parent_wiring(
            None,
            self.schematic.document(),
            content.design_management.sheet_catalog(&active_key),
            &reference,
            &base,
        ));
        let key = content.active_key();
        self.ui
            .symbol
            .history
            .begin_parent_edit(key, SymbolParentEdit { base, parents });
    }

    /// Carry the held wire ends to where `document` puts their terminals.
    fn apply_symbol_parent_edit(
        &mut self,
        document: &SymbolDocument,
        renames: &BTreeMap<String, String>,
    ) {
        let key = self.workspace.content.active_key();
        let Some(edit) = self.ui.symbol.history.parent_edit(&key) else {
            return;
        };
        let pin_remaps = symbol_pin_position_remaps(&edit.base, document, renames);
        for parent in &edit.parents {
            match &parent.buffer {
                Some(buffer) => {
                    self.workspace
                        .for_each_schematic_editor_mut(|key, schematic| {
                            if key == buffer {
                                schematic.apply_symbol_instance_wiring(&parent.wiring, &pin_remaps);
                            }
                        });
                }
                None => {
                    self.schematic
                        .apply_symbol_instance_wiring(&parent.wiring, &pin_remaps);
                }
            }
        }
    }

    /// Close the held edit: each parent it changed gets one undo record
    /// spanning the whole edit.
    pub(crate) fn finish_symbol_parent_edit(&mut self) {
        const DESCRIPTION: &str = "move wires with symbol pins";
        let Some(edit) = self.ui.symbol.history.take_parent_edit() else {
            return;
        };
        for parent in edit.parents {
            match parent.buffer {
                Some(buffer) => {
                    let mut before = Some(parent.before);
                    self.workspace
                        .for_each_schematic_editor_mut(|key, schematic| {
                            if key == buffer
                                && let Some(before) = before.take()
                            {
                                schematic.commit_undo_from(before, DESCRIPTION);
                            }
                        });
                }
                None => {
                    self.schematic.commit_undo_from(parent.before, DESCRIPTION);
                }
            }
        }
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
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        self.with_symbol_parent_edit(|state| {
            state.publish_active_symbol_edit(document, metadata, intent)
        })
    }

    /// Publish one frame of a canvas gesture.
    ///
    /// The gesture is one edit however many frames publish it: the parents'
    /// wiring is resolved at its first frame, every frame moves the held ends
    /// from that baseline, and the canvas closes it with
    /// [`AppState::finish_symbol_parent_edit`] when the pointer lets go.
    pub(crate) fn store_symbol_canvas_frame(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<(), String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let key = self.workspace.content.active_key();
        if self.ui.symbol.history.parent_edit(&key).is_none() {
            self.begin_symbol_parent_edit();
        }
        self.publish_active_symbol_edit(document, metadata, &SymbolCommitIntent::default())
    }

    fn publish_active_symbol_edit(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
        intent: &SymbolCommitIntent,
    ) -> Result<(), String> {
        let reference = self.workspace.content.active_view.clone();
        let bundle = EncodedSymbolEditorBundle::encode(document, metadata)?;
        let Some(()) = self
            .library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| {
                library.store_symbol_editor_bundle(&reference.cell, &reference.view, bundle)
            })
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };

        self.apply_symbol_parent_edit(document, &intent.renames);
        let renamed = self.apply_symbol_pin_renames(&reference, &intent.renames);
        if renamed > 0 {
            self.prompt(format!(
                "Renamed {} across placed instances of {}.",
                counted(renamed, "instance terminal", "instance terminals"),
                reference.display_path()
            ));
        }
        // The undo entry this edit sits on top of has to be able to put the
        // names back, so it carries the reverse of what was just applied.
        if !intent.renames.is_empty() {
            let key = self.workspace.content.active_key();
            self.ui
                .symbol
                .history
                .record_inverse_renames(&key, &intent.renames);
        }
        self.workspace.content.set_active_dirty(true);
        Ok(())
    }

    /// The model definition the active symbol view is bound to, if any.
    fn active_model_bound_symbol_definition(&self) -> Option<ModelBoundSymbolDefinition> {
        let reference = &self.workspace.content.active_view;
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| crate::state::load_model_bound_symbol(view).ok().flatten())
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
            &self.workspace.content.active_view.cell,
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
        let key = self.workspace.content.active_key();
        self.ui.symbol.history.mark_save_point(key);
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
            .get(&self.workspace.content.active_key())
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
            && !self.workspace.content.active_read_only_reference()
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
        self.workspace
            .for_each_schematic_editor_mut(|_, schematic| {
                renamed += schematic.rename_instance_terminals(reference, &lowered);
            });
        renamed += self
            .schematic
            .rename_instance_terminals(reference, &lowered);
        renamed
    }

    pub(crate) fn record_symbol_edit(&mut self, before: &SymbolDocument) {
        const MAX_SYMBOL_UNDO: usize = 128;
        let key = self.workspace.content.active_key();
        let snapshot = self.active_symbol_snapshot(before);
        self.push_symbol_undo_snapshot(key, snapshot, MAX_SYMBOL_UNDO);
    }

    pub(super) fn record_symbol_metadata_edit(&mut self, before: &SymbolDocument) {
        const MAX_SYMBOL_UNDO: usize = 128;
        let key = self.workspace.content.active_key();
        let snapshot = self.active_symbol_metadata_snapshot(before);
        self.push_symbol_undo_snapshot(key, snapshot, MAX_SYMBOL_UNDO);
    }

    fn push_symbol_undo_snapshot(
        &mut self,
        key: String,
        snapshot: SymbolDocumentSnapshot,
        max_len: usize,
    ) {
        self.ui.symbol.history.record_edit(key, snapshot, max_len);
    }

    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        self.ui
            .symbol
            .history
            .can_undo(&self.workspace.content.active_key())
    }

    pub(crate) fn can_redo_active_symbol_document(&self) -> bool {
        self.ui
            .symbol
            .history
            .can_redo(&self.workspace.content.active_key())
    }

    pub(crate) fn undo_active_symbol_document(&mut self) -> Result<bool, String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let key = self.workspace.content.active_key();
        let Some(previous) = self.ui.symbol.history.pop_undo(&key) else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let mut current_snapshot = self.active_symbol_metadata_snapshot(&current);
        current_snapshot.renames = previous.inverted_renames();
        self.ui.symbol.history.finish_undo(key, current_snapshot);
        self.restore_active_symbol_snapshot(&previous)?;
        Ok(true)
    }

    pub(crate) fn redo_active_symbol_document(&mut self) -> Result<bool, String> {
        if self.active_view_read_only() {
            return Err(self.read_only_master_message());
        }
        let key = self.workspace.content.active_key();
        let Some(next) = self.ui.symbol.history.pop_redo(&key) else {
            return Ok(false);
        };
        let current = self.load_active_symbol_document()?;
        let mut current_snapshot = self.active_symbol_metadata_snapshot(&current);
        current_snapshot.renames = next.inverted_renames();
        self.ui.symbol.history.finish_redo(key, current_snapshot);
        self.restore_active_symbol_snapshot(&next)?;
        Ok(true)
    }

    /// Everything the symbol pin checks read for one symbol view.
    ///
    /// One accessor, because the digest that decides whether a published pin
    /// finding still describes the design has to be taken over exactly what
    /// the check compared — and the check and the digest both read it here.
    pub(in crate::workbench) fn symbol_pin_check_inputs(
        &self,
        reference: &CellViewRef,
    ) -> Result<SymbolPinCheckInputs, String> {
        let document = self.load_symbol_document(reference)?;
        let revision = self
            .load_symbol_editor_metadata(reference, &document)?
            .revision;
        let ports = self.symbol_ports(reference);
        Ok(SymbolPinCheckInputs {
            revision,
            document,
            ports,
        })
    }

    /// Publish what the symbol pin checks found, as canonical records.
    ///
    /// Its own entry point with its own guard rather than a loosened
    /// [`Self::publish_active_design_check_result`]: that guard is right to
    /// refuse a symbol, because schematic design-check evidence is keyed to a
    /// cell view that has a schematic and carries a receipt the symbol checks
    /// have no part in.
    pub(crate) fn publish_active_symbol_pin_check_result(
        &mut self,
        result: &rspice_design::drc::DrcResult,
    ) -> Result<(), String> {
        if self.workspace.content.active_view_type() != crate::state::ViewType::Symbol {
            return Err("symbol pin findings can only be published from a symbol view".to_owned());
        }
        let subject = self.workspace.content.active_view.clone();
        // Derived before the batch is inserted, because resolving a route
        // reads the state the insertion takes mutably.
        let mut records = symbol_pin_records(self, &subject, result);
        // The symbol's own waivers that do not do what they say are stated
        // with the pass that found that out.
        if let Ok(document) = self.load_symbol_document(&subject) {
            records.extend(crate::workbench::app_state::waivers::stored_waiver_records(
                self,
                &subject,
                &document.waivers,
                result,
            ));
        }
        self.publish_diagnostics(DiagnosticInputs::SymbolPins { subject }, records)
    }

    /// How many pin findings stand against the active symbol view, or `None`
    /// where the checks have not run against it.
    ///
    /// The distinction is the point: `Some(0)` says the checks ran and the
    /// pins match, which is a different claim from nobody having looked, and
    /// a control that offered to clear nothing would be a dead control.
    pub(in crate::workbench) fn active_symbol_pin_finding_count(&self) -> Option<usize> {
        self.active_symbol_pin_records()
            .map(|records| records.len())
    }

    /// Withdraw the active symbol view's pin findings.
    pub(in crate::workbench) fn clear_active_symbol_pin_checks(&mut self) {
        let subject = self.workspace.content.active_view.clone();
        self.withdraw_diagnostics(&DiagnosticInputs::SymbolPins { subject });
    }

    fn active_symbol_pin_records(&self) -> Option<&[DiagnosticRecord]> {
        let published = DiagnosticInputs::SymbolPins {
            subject: self.workspace.content.active_view.clone(),
        };
        self.diagnostics
            .batch(&published)
            .map(|batch| batch.records.as_slice())
    }

    /// Step the finding tour through the active symbol's pin findings, worst
    /// first, and land on the pin the finding it stops at is about.
    ///
    /// Returns `false` where there is nothing to visit. The findings come from
    /// the store rather than from a projection beside it, which is what lets
    /// the finding this stops at be *the* current finding — the same selection
    /// the Problems grid draws its current row from.
    pub(crate) fn cycle_active_symbol_pin_finding(&mut self, step: isize) -> bool {
        let mut visitable: Vec<VisitablePinFinding> = self
            .active_symbol_pin_records()
            .unwrap_or_default()
            .iter()
            .filter_map(|record| match &record.route {
                Some(DiagnosticRoute::Anchor(anchor)) => Some(VisitablePinFinding {
                    level: record.level,
                    identity: record.canonical.diagnostic_id,
                    anchor: anchor.clone(),
                    message: record.message.clone(),
                }),
                Some(DiagnosticRoute::NetlistDiagnostic(_) | DiagnosticRoute::Repair(_))
                | Some(DiagnosticRoute::CodeDocument { .. } | DiagnosticRoute::RunLogLine { .. })
                | None => None,
            })
            .collect();
        if visitable.is_empty() {
            return false;
        }
        // Worst first; stable within a severity.
        visitable.sort_by(|left, right| right.level.cmp(&left.level));

        let len = visitable.len() as isize;
        let standing = self
            .diagnostics
            .current_finding()
            .and_then(|current| visitable.iter().position(|found| found.identity == current));
        let index = match standing {
            // First jump lands on the worst finding, not the second one.
            None if step >= 0 => 0,
            None => (len - 1) as usize,
            Some(current) => ((current as isize + step).rem_euclid(len)) as usize,
        };
        let found = visitable.swap_remove(index);
        self.diagnostics.set_current_finding(Some(found.identity));
        self.jump_to_log_anchor(found.anchor);
        self.prompt(format!(
            "Finding {}/{} — {} · {}",
            index + 1,
            len,
            found.level.display_name(),
            found.message,
        ));
        true
    }

    pub(crate) fn run_active_symbol_pin_checks(&mut self) {
        let reference = self.workspace.content.active_view.clone();
        let inputs = match self.symbol_pin_check_inputs(&reference) {
            Ok(inputs) => inputs,
            Err(error) => {
                self.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("Symbol pin checks could not run: {error}"),
                );
                return;
            }
        };
        let mut result = check_symbol_pins(&inputs.document, &inputs.ports, &reference);
        // The symbol's own waivers, stored in the document the checks read.
        rspice_design::drc::CheckWaivers::for_subject(&inputs.document.waivers).apply(&mut result);
        let found = result.total_count();
        if let Err(error) = self.publish_active_symbol_pin_check_result(&result) {
            self.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Symbol pin check findings could not be published: {error}"),
            );
            return;
        }
        // One Console line records that the check ran, at the severity of an
        // event rather than of a finding: an error line here would stand on
        // the Problems page as a problem of its own, beside the findings it
        // counts. The findings themselves are what the page, the counts and
        // the canvas tour all read.
        let summary = if found == 0 {
            "Symbol pin check: pins match the schematic.".to_owned()
        } else {
            format!(
                "Symbol pin check: {}. See Problems.",
                counted(found, "finding", "findings")
            )
        };
        self.record(LogSeverity::Info, LogSource::Studio, summary);
    }
}

/// One pin finding the tour can visit: how bad it is, what the store calls
/// it, which pin it lands on, and what it says.
struct VisitablePinFinding {
    level: DiagnosticLevel,
    identity: uuid::Uuid,
    anchor: LogAnchor,
    message: String,
}

/// Everything one symbol view's pin checks were derived from.
///
/// The published revision as well as the artwork, because a revision is what
/// the instances placed against this symbol were wired to: republishing the
/// same drawing under a new one is a new contract, and a finding measured
/// against the old one no longer describes it.
#[derive(Debug, Clone, serde::Serialize)]
pub(in crate::workbench) struct SymbolPinCheckInputs {
    pub(in crate::workbench) revision: u64,
    pub(in crate::workbench) document: SymbolDocument,
    pub(in crate::workbench) ports: Vec<PortSpec>,
}

/// One canonical record per pin finding.
///
/// The route is resolved through the one resolver every "go to this finding"
/// surface shares, and a symbol pin location already addresses the pin on its
/// own symbol — so following a finding lands on that pin rather than on the
/// document that happens to hold it. No `drawn` test is needed, the way the
/// schematic rule records need one: a pin location carries the symbol it is
/// in, so it resolves without asking whichever drawing is in front of the
/// reader where its object is.
fn symbol_pin_records(
    state: &AppState,
    subject: &CellViewRef,
    result: &rspice_design::drc::DrcResult,
) -> Vec<DiagnosticRecord> {
    let project = state.workspace.content.project.id();
    let revision = state.workspace.content.project.revision().get();
    let document_id = subject.key();
    // Keyed by the pin each finding is about, through the one key the design
    // rules use, and minted by the one function every store record is minted
    // by: a re-check names an unchanged pin finding exactly as before.
    let keys: Vec<FindingKey> = result
        .violations()
        .iter()
        .map(|violation| {
            FindingKey::new(
                SYMBOL_PIN_PRODUCER,
                violation.violation_type.rule_code(),
                document_id.as_str(),
                location_key(&violation.location),
            )
        })
        .collect();
    let canonicals = mint_canonical(&keys, revision, &format!("symbol-pins:{document_id}"));
    result
        .violations()
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((violation, key), mut canonical)| {
            use crate::workbench::app_state::waivers;

            let route =
                crate::schematic::view::violations::location_anchor(state, &violation.location)
                    .map(DiagnosticRoute::Anchor);
            canonical.suppression = violation.waiver.as_ref().map(waivers::suppression_of);
            DiagnosticRecord::new(
                canonical,
                violation.severity.into(),
                DiagnosticSubject::new(project, violation.location.display())
                    .in_cell_view(subject.clone()),
                violation.message.clone(),
            )
            .keyed(key)
            .with_details(violation.violation_type.suggested_fix())
            .with_rule(violation.violation_type.rule().explanation)
            .with_route(route)
            .with_waiver_action(waivers::finding_waiver_action(subject, violation))
        })
        .collect()
}

/// One open schematic's hold on the instances of `reference`, or `None` when
/// it places none.
///
/// A wire end belongs to a terminal only when the wire is drawn on the
/// instance's own sheet. An object no sheet claims stands on the active
/// sheet, which is where the canvas draws it.
fn parent_wiring(
    buffer: Option<String>,
    document: &SchematicDocument,
    catalog: Option<&crate::state::SheetCatalog>,
    reference: &CellViewRef,
    symbol: &SymbolDocument,
) -> Option<SymbolParentWiring> {
    if !places_symbol(document, reference) {
        return None;
    }
    let sheet_of = |id: u64| {
        catalog.and_then(|catalog| {
            catalog
                .sheet_for_object(id)
                .or_else(|| catalog.active_sheet_id())
        })
    };
    let wiring = resolve_symbol_instance_wiring(document, reference, symbol, |component, wire| {
        sheet_of(component) == sheet_of(wire)
    });
    Some(SymbolParentWiring {
        buffer,
        before: SchematicSnapshot::capture(document),
        wiring,
    })
}
