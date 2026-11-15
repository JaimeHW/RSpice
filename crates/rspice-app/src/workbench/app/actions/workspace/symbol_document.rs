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

use rspice_schematic_editor::symbol_editor::inspector::LabelParameter;
use rspice_schematic_editor::symbol_editor::interaction::SymbolRequestSource;
use std::collections::BTreeMap;

use crate::diagnostics::{LogAnchor, LogSeverity, LogSource};
use crate::state::{
    CellViewRef, PortSpec, SYMBOL_DOCUMENT_METADATA_KEY, SchematicState, SymbolDocument,
    SymbolEditorMetadata, SymbolInterface,
};
use crate::ui::accessibility::counted;
use crate::workbench::app_state::{Model, SlotFacts};
use crate::workbench::state::WorkbenchSession;

use super::symbol_in_place::in_place_session;
use super::{NavigationParts, read_only_master_message};
use crate::workbench::app_state::design_checks::location_key;
use crate::workbench::app_state::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};
use crate::workbench::{SymbolCommitIntent, SymbolDocumentSnapshot};
use rspice_design::symbol::publication::{EncodedSymbolEditorBundle, check_symbol_pins};

use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::history::SchematicSnapshot;
use rspice_design::symbol::edit::{
    SymbolParentEdit, SymbolParentWiring, places_symbol, resolve_symbol_instance_wiring,
    symbol_metadata_snapshot_from_view, symbol_pin_position_remaps, symbol_snapshot_from_view,
};

/// Producer identity every symbol pin finding carries.
const SYMBOL_PIN_PRODUCER: &str = "rspice.design.symbol.pins";

#[derive(Debug, Clone, Default)]
pub struct SymbolUiState {
    pub editor: rspice_schematic_editor::symbol_editor::session::SymbolEditorSession,
    pub history: rspice_design::symbol::edit::SymbolHistory,
    /// The symbol being edited in place on a parent schematic, if any.
    pub in_place: Option<Box<super::symbol_in_place::SymbolInPlace>>,
}

/// Publish one frame of a canvas gesture.
///
/// The gesture is one edit however many frames publish it: the parents'
/// wiring is resolved at its first frame, every frame moves the held ends
/// from that baseline, and the canvas closes it with
/// [`finish_symbol_parent_edit`] when the pointer lets go.
pub(crate) fn store_symbol_canvas_frame(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
) -> Result<(), String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let key = model.workspace.content.active_key();
    if symbol.history.parent_edit(&key).is_none() {
        begin_symbol_parent_edit(symbol, model);
    }
    publish_active_symbol_edit(
        symbol,
        model,
        wb,
        document,
        metadata,
        &SymbolCommitIntent::default(),
    )
}

/// The ports the interface its schematic declares gives the active symbol.
pub(crate) fn active_symbol_ports(model: &Model) -> Vec<PortSpec> {
    model.symbol_ports(&model.workspace.content.active_view)
}

/// The parameters a label on the active symbol can letter, each with its
/// default: the fields of the parameter form a model binds the symbol to,
/// else what its cell declares to instances, through its compiled module,
/// its schematic or the symbol view itself.
pub(crate) fn active_symbol_label_parameters(model: &Model) -> Vec<LabelParameter> {
    use rspice_design::library::metadata::{cell_parameter_contract, module_interface_view};
    let reference = &model.workspace.content.active_view;
    if let Some(definition) = model.symbol_model_binding(reference) {
        return definition
            .parameter_form
            .fields()
            .map(|field| LabelParameter {
                name: field.key.clone(),
                default: field.default.display_string(),
            })
            .collect();
    }
    let Some(cell) = model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
    else {
        return Vec::new();
    };
    [
        module_interface_view(cell),
        cell.get_view("schematic"),
        cell.get_view(&reference.view),
    ]
    .into_iter()
    .flatten()
    .find_map(|view| {
        cell_parameter_contract(cell, Some(view))
            .ok()
            .filter(|parameters| !parameters.is_empty())
    })
    .unwrap_or_default()
    .into_iter()
    .map(|parameter| LabelParameter {
        name: parameter.name,
        default: parameter.default_value.unwrap_or_default(),
    })
    .collect()
}

/// Bring the active symbol's pins back into agreement with the interface
/// its schematic declares, without touching a single body shape.
///
/// Returns the console line describing what changed.
pub(crate) fn update_active_symbol_pins_from_contract(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
) -> Result<String, String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let reference = model.workspace.content.active_view.clone();
    let ports = match model.symbol_interface(&reference) {
        SymbolInterface::Declared(ports) if !ports.is_empty() => ports,
        SymbolInterface::Declared(_) => {
            return Err(format!(
                "The {} declares no pins, so there are none to update from.",
                model.symbol_contract_source(&reference)
            ));
        }
        SymbolInterface::SymbolDefined => {
            return Err(
                "This cell has no schematic, so its symbol defines its own pins.".to_owned(),
            );
        }
    };
    let before = model.load_active_symbol_document()?;
    let metadata = load_active_symbol_editor_metadata(model, &before)?;
    let mut document = before.clone();
    model.reconcile_symbol_with_contract(
        &model.workspace.content.active_view,
        &mut document,
        &ports,
    );
    if document == before {
        return Ok("Symbol pins already match the interface contract".to_owned());
    }
    let added = document.pins.len().saturating_sub(before.pins.len());
    let orphaned = document
        .pin_summary(&SymbolInterface::Declared(ports))
        .orphaned;
    record_symbol_edit(symbol, model, &before, "Update pins from contract");
    store_active_symbol_editor_bundle(symbol, model, wb, &document, &metadata)?;
    Ok(format!(
        "Symbol pins updated from the interface contract: {added} added, {orphaned} left orphaned"
    ))
}

/// Bring the active symbol's schematic into agreement with the symbol the
/// other way: declare a port for each symbol pin the schematic lacks, with
/// the pin's name and direction, in a column on the schematic's sheet, as
/// one undo step of the schematic. No port is ever removed. A schematic
/// never opened gets the new sheet `new_sheet` makes, as opening it would.
///
/// Returns the console line describing what changed.
pub(crate) fn create_schematic_pins_from_active_symbol(
    model: &mut Model,
    new_sheet: impl FnOnce(&Model) -> SchematicState,
) -> Result<String, String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let reference = model.workspace.content.active_view.clone();
    let missing = model
        .symbol_pins_missing_from_schematic(&reference)
        .map_err(|reason| format!("No schematic pins were created: {reason}."))?;
    let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
    let key = schematic_ref.key();
    if !model.workspace.content.schematic_buffers.contains_key(&key) {
        let sheet = new_sheet(model);
        model.workspace.insert_schematic_editor(key.clone(), sheet);
    }
    let mut schematic = model
        .workspace
        .schematic_editor_mut(&key)
        .ok_or_else(|| format!("View '{}' not found", schematic_ref.display_path()))?;
    let outcome = schematic
        .editor
        .declare_ports_for_symbol_pins(&missing, "Create schematic pins from symbol")
        .map_err(|error| format!("No schematic pins were created: {error}."))?;
    drop(schematic);
    let mut line = match outcome.added.len() {
        0 => "No schematic pins were created".to_owned(),
        _ => format!(
            "Added {} to {}: {}",
            counted(outcome.added.len(), "pin", "pins"),
            schematic_ref.display_path(),
            outcome.added.join(", ")
        ),
    };
    for (name, reason) in &outcome.refused {
        line.push_str(&format!("; {name} was left out: {reason}"));
    }
    line.push('.');
    Ok(line)
}

pub(crate) fn symbol_editor_request_source(
    model: &Model,
    wb: &WorkbenchSession,
) -> SymbolRequestSource {
    SymbolRequestSource {
        project: model.workspace.content.project.id(),
        document: model.workspace.content.active_view.clone(),
        occurrence: model.workspace.content.active_occurrence().cloned(),
        design_epoch: model.design_execution_epoch,
        document_epoch: model.active_schematic_epoch,
        library_revision: model.library_manager.revision(),
        window: wb.window_session.current().value(),
    }
}

/// Project one retained view session into the active editor slot. Navigation
/// advances request authority; it does not turn passive rendering into an edit.
pub(crate) fn project_active_symbol_session(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &WorkbenchSession,
) {
    use rspice_schematic_editor::symbol_editor::interaction::{
        bind_canvas_source, resume_canvas_source,
    };

    // Leaving the symbol by any route but the session's own exit ends an
    // in-place edit; what it changed stays, as any symbol edit does.
    if symbol
        .in_place
        .as_ref()
        .is_some_and(|session| session.symbol != model.workspace.content.active_view)
    {
        symbol.in_place = None;
    }
    let source = symbol_editor_request_source(model, wb);
    let is_symbol =
        model.workspace.content.active_view_type() == Some(crate::state::ViewType::Symbol);
    let open_views = &model.workspace.content.open_views;
    let retained = &mut model.workspace.session.symbol_sessions;
    let still_open = |previous: &SymbolRequestSource| {
        previous.project == source.project
            && previous.design_epoch == source.design_epoch
            && open_views
                .iter()
                .any(|open| open.reference == previous.document)
    };
    retained.retain(|_, session| session.canvas_source.as_ref().is_some_and(still_open));
    let same_document = symbol
        .editor
        .canvas_source
        .as_ref()
        .is_some_and(|previous| {
            previous.project == source.project
                && previous.design_epoch == source.design_epoch
                && previous.document == source.document
        });
    if !is_symbol || !same_document {
        if symbol
            .editor
            .canvas_source
            .as_ref()
            .is_some_and(|previous| previous.window == source.window)
        {
            // User navigation ends active input. Projecting a different
            // window keeps it with the window that still owns it.
            symbol.editor.clear_drag_state();
            symbol.editor.marquee_start = None;
            symbol.editor.marquee_current = None;
            symbol.editor.inspector.end_field_edit();
        }
        // Clipboard contents travel across views; every other field is view-local.
        let clipboard = std::mem::take(&mut symbol.editor.clipboard);
        let previous = std::mem::take(&mut symbol.editor);
        if let Some(previous_source) = &previous.canvas_source
            && still_open(previous_source)
        {
            retained.insert(previous_source.document.key(), previous);
        }
        if is_symbol && let Some(mut session) = retained.remove(&source.document.key()) {
            resume_canvas_source(&mut session, source.clone());
            symbol.editor = session;
        }
        symbol.editor.clipboard = clipboard;
    }
    if is_symbol {
        bind_canvas_source(&mut symbol.editor, source);
    }
}

fn active_symbol_metadata_snapshot(
    model: &Model,
    fallback: &SymbolDocument,
) -> SymbolDocumentSnapshot {
    let reference = &model.workspace.content.active_view;
    model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| symbol_metadata_snapshot_from_view(view, fallback))
        .unwrap_or_else(|| SymbolDocumentSnapshot::from_document(fallback))
}

fn restore_active_symbol_snapshot(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    snapshot: &SymbolDocumentSnapshot,
) -> Result<(), String> {
    let reference = model.workspace.content.active_view.clone();
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    with_symbol_parent_edit(symbol, model, |symbol, model| {
        let Some(()) = model
            .library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| {
                library.restore_symbol_snapshot(&reference.cell, &reference.view, snapshot)
            })
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        apply_symbol_parent_edit(symbol, model, &snapshot.document, &snapshot.renames);
        apply_symbol_pin_renames(model, &reference, &snapshot.renames);
        Ok(())
    })?;
    model.workspace.content.set_active_dirty(true);
    Ok(())
}

/// Whether the active symbol's history holds a step to take back.
#[cfg(test)]
pub(crate) fn can_undo_active_symbol_document(symbol: &SymbolUiState, model: &Model) -> bool {
    symbol
        .history
        .can_undo(&model.workspace.content.active_key())
}

/// Step the active symbol's history back; `Ok(false)` when it holds
/// no step to take.
pub(crate) fn undo_active_symbol_document(
    symbol: &mut SymbolUiState,
    model: &mut Model,
) -> Result<bool, String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let key = model.workspace.content.active_key();
    let Some(previous) = symbol.history.pop_undo(&key) else {
        return Ok(false);
    };
    let current = model.load_active_symbol_document()?;
    let mut current_snapshot = active_symbol_metadata_snapshot(model, &current);
    current_snapshot.renames = previous.inverted_renames();
    current_snapshot.step.clone_from(&previous.step);
    symbol.history.finish_undo(key, current_snapshot);
    restore_active_symbol_snapshot(symbol, model, &previous)?;
    Ok(true)
}

/// Step the active symbol's history forward; `Ok(false)` when it holds
/// no step to take.
pub(crate) fn redo_active_symbol_document(
    symbol: &mut SymbolUiState,
    model: &mut Model,
) -> Result<bool, String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let key = model.workspace.content.active_key();
    let Some(next) = symbol.history.pop_redo(&key) else {
        return Ok(false);
    };
    let current = model.load_active_symbol_document()?;
    let mut current_snapshot = active_symbol_metadata_snapshot(model, &current);
    current_snapshot.renames = next.inverted_renames();
    current_snapshot.step.clone_from(&next.step);
    symbol.history.finish_redo(key, current_snapshot);
    restore_active_symbol_snapshot(symbol, model, &next)?;
    Ok(true)
}

pub(crate) fn load_active_symbol_editor_metadata(
    model: &Model,
    document: &SymbolDocument,
) -> Result<SymbolEditorMetadata, String> {
    model.load_symbol_editor_metadata(&model.workspace.content.active_view, document)
}

fn active_symbol_snapshot(model: &Model, fallback: &SymbolDocument) -> SymbolDocumentSnapshot {
    let reference = &model.workspace.content.active_view;
    let current_document = model.load_active_symbol_document().ok();
    model
        .library_manager
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

/// The refusal message for the symbol editor's read-only banner, naming
/// which of the four owners holds the lock.
pub(crate) fn symbol_editor_lock_message(model: &Model) -> String {
    if let Some(holder) = model
        .live_write_locks
        .schematic_views
        .get(&model.workspace.content.active_key())
    {
        return format!("{holder} holds the write lease on this symbol");
    }
    if model.live_write_locks.mirror {
        return "This is the host's working copy; request the write lease from the live session to edit."
            .to_owned();
    }
    read_only_master_message(model)
}

/// Whether copying the cell into an editable library is a route out of
/// the lock. It is not one when the whole project refuses writes.
pub(crate) fn symbol_editor_copy_available(model: &Model) -> bool {
    !model.safe_mode.project_read_only()
        && !model.live_write_locks.mirror
        && !model.workspace.content.active_read_only_reference()
}

/// Draw the active symbol from its ports, as one undoable edit of the
/// symbol when the drawing changes.
pub(crate) fn generate_active_symbol_document(
    symbol: &mut SymbolUiState,
    model: &mut Model,
) -> Result<(), String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    let before = model.load_active_symbol_document()?;
    let ports = active_symbol_ports(model);
    let document = model.generated_symbol_document(&model.workspace.content.active_view, &ports);
    if document != before {
        record_symbol_metadata_edit(symbol, model, &before);
    }
    store_active_symbol_document(symbol, model, &document)?;
    Ok(())
}

fn record_symbol_metadata_edit(symbol: &mut SymbolUiState, model: &Model, before: &SymbolDocument) {
    const MAX_SYMBOL_UNDO: usize = 128;
    let key = model.workspace.content.active_key();
    let mut snapshot = active_symbol_metadata_snapshot(model, before);
    snapshot.step = "Generate symbol".to_owned();
    push_symbol_undo_snapshot(symbol, key, snapshot, MAX_SYMBOL_UNDO);
}

pub(crate) fn store_active_symbol_document(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    document: &SymbolDocument,
) -> Result<(), String> {
    let reference = model.workspace.content.active_view.clone();
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    with_symbol_parent_edit(symbol, model, |symbol, model| {
        let Some(result) = model
            .library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| {
                library.store_symbol_document(&reference.cell, &reference.view, document)
            })
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        result?;
        apply_symbol_parent_edit(symbol, model, document, &BTreeMap::new());
        Ok(())
    })?;
    model.workspace.content.set_active_dirty(true);
    Ok(())
}

/// Put the active symbol back as `restore` writes it, the way Revert to
/// saved does: every placed instance takes back the pin names it took since
/// the save, the wires it holds follow the restored pins, and the symbol is
/// left with no steps to undo or redo, since none of them leads from the
/// restored drawing.
pub(crate) fn revert_active_symbol_with<T, E>(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    restore: impl FnOnce(&mut Model) -> Result<T, E>,
) -> Result<T, E> {
    let key = model.workspace.content.active_key();
    let reference = model.workspace.content.active_view.clone();
    begin_symbol_parent_edit(symbol, model);
    let restored = restore(model);
    if restored.is_ok() {
        let saved_names = model.unsaved_pin_renames.take(&reference.key());
        if let Ok(document) = model.load_active_symbol_document() {
            // The instances' wires are held by the names the pins had before
            // the revert; each renamed one follows its pin to its saved name.
            let renames = symbol
                .history
                .parent_edit(&key)
                .map(|edit| {
                    edit.base
                        .pins
                        .iter()
                        .filter_map(|pin| {
                            saved_names
                                .get(&pin.name.to_ascii_lowercase())
                                .map(|saved| (pin.name.clone(), saved.clone()))
                        })
                        .collect()
                })
                .unwrap_or_default();
            apply_symbol_parent_edit(symbol, model, &document, &renames);
            rename_placed_terminals(model, &reference, &saved_names);
        }
    }
    finish_symbol_parent_edit(symbol, model);
    symbol.history.forget(&key);
    symbol.editor.clear_selection();
    restored
}

/// Run one complete symbol edit: its parent wiring is resolved before it
/// and recorded in each parent's history after it.
fn with_symbol_parent_edit<R>(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    edit: impl FnOnce(&mut SymbolUiState, &mut Model) -> R,
) -> R {
    begin_symbol_parent_edit(symbol, model);
    let result = edit(symbol, model);
    finish_symbol_parent_edit(symbol, model);
    result
}

/// Hold, in every open schematic that places the active symbol's cell,
/// the wire vertices each instance terminal holds now.
///
/// Any edit still held open is finished first: its end never arrived, and
/// resolving over it would read its half-moved vertices as the baseline.
fn begin_symbol_parent_edit(symbol: &mut SymbolUiState, model: &mut Model) {
    finish_symbol_parent_edit(symbol, model);
    let Ok(base) = model.load_active_symbol_document() else {
        return;
    };
    let key = model.workspace.content.active_key();
    // Edited in place, the parent's wires are in view and a pin is moved
    // against them: one left behind is the open-pin mark's to show, not
    // a wire the edit drags along unseen.
    if in_place_session(&symbol.in_place, model).is_some() {
        let parents = Vec::new();
        symbol
            .history
            .begin_parent_edit(key, SymbolParentEdit { base, parents });
        return;
    }
    let reference = model.workspace.content.active_view.clone();
    let content = &model.workspace.content;
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
        model.schematic.document(),
        content.design_management.sheet_catalog(&active_key),
        &reference,
        &base,
    ));
    symbol
        .history
        .begin_parent_edit(key, SymbolParentEdit { base, parents });
}

/// Carry the held wire ends to where `document` puts their terminals.
fn apply_symbol_parent_edit(
    symbol: &SymbolUiState,
    model: &mut Model,
    document: &SymbolDocument,
    renames: &BTreeMap<String, String>,
) {
    let key = model.workspace.content.active_key();
    let Some(edit) = symbol.history.parent_edit(&key) else {
        return;
    };
    let pin_remaps = symbol_pin_position_remaps(&edit.base, document, renames);
    for parent in &edit.parents {
        match &parent.buffer {
            Some(buffer) => {
                model
                    .workspace
                    .for_each_schematic_editor_mut(|key, schematic| {
                        if key == buffer {
                            schematic.apply_symbol_instance_wiring(&parent.wiring, &pin_remaps);
                        }
                    });
            }
            None => {
                model
                    .schematic
                    .apply_symbol_instance_wiring(&parent.wiring, &pin_remaps);
            }
        }
    }
}

/// Close the held edit: each parent it changed gets one undo record
/// spanning the whole edit.
pub(crate) fn finish_symbol_parent_edit(symbol: &mut SymbolUiState, model: &mut Model) {
    const DESCRIPTION: &str = "move wires with symbol pins";
    let Some(edit) = symbol.history.take_parent_edit() else {
        return;
    };
    for parent in edit.parents {
        match parent.buffer {
            Some(buffer) => {
                let mut before = Some(parent.before);
                model
                    .workspace
                    .for_each_schematic_editor_mut(|key, schematic| {
                        if key == buffer
                            && let Some(before) = before.take()
                        {
                            schematic.commit_undo_from(before, DESCRIPTION);
                        }
                    });
            }
            None => {
                model.schematic.commit_undo_from(parent.before, DESCRIPTION);
            }
        }
    }
}

/// Atomically publish symbol geometry and its authoring-only metadata.
pub(crate) fn store_active_symbol_editor_bundle(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
) -> Result<(), String> {
    commit_active_symbol_edit(
        symbol,
        model,
        wb,
        document,
        metadata,
        &SymbolCommitIntent::default(),
    )
}

/// Publish symbol geometry and metadata, carrying what the author meant.
///
/// The intent is the half of the edit that comparing two documents
/// cannot recover. A pin rename reaches every instance from here and
/// nowhere else, because this is the one point at which the old name and
/// the new one are both still known.
pub(crate) fn commit_active_symbol_edit(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
    intent: &SymbolCommitIntent,
) -> Result<(), String> {
    if model.active_view_read_only() {
        return Err(read_only_master_message(model));
    }
    with_symbol_parent_edit(symbol, model, |symbol, model| {
        publish_active_symbol_edit(symbol, model, wb, document, metadata, intent)
    })
}

fn publish_active_symbol_edit(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
    intent: &SymbolCommitIntent,
) -> Result<(), String> {
    let reference = model.workspace.content.active_view.clone();
    let bundle = EncodedSymbolEditorBundle::encode(document, metadata)?;
    let Some(()) = model
        .library_manager
        .edit_library(&reference.library)
        .and_then(|mut library| {
            library.store_symbol_editor_bundle(&reference.cell, &reference.view, bundle)
        })
    else {
        return Err(format!("View '{}' not found", reference.display_path()));
    };

    apply_symbol_parent_edit(symbol, model, document, &intent.renames);
    let renamed = apply_symbol_pin_renames(model, &reference, &intent.renames);
    if renamed > 0 {
        wb.prompt(format!(
            "Renamed {} across placed instances of {}.",
            counted(renamed, "instance terminal", "instance terminals"),
            reference.display_path()
        ));
    }
    // The undo entry this edit sits on top of has to be able to put the
    // names back, so it carries the reverse of what was just applied.
    if !intent.renames.is_empty() {
        let key = model.workspace.content.active_key();
        symbol.history.record_inverse_renames(&key, &intent.renames);
    }
    model.workspace.content.set_active_dirty(true);
    Ok(())
}

/// Carry a pin rename to every placement of the cellview, and keep it
/// among the renames Revert to saved takes back.
///
/// An instance wires to a pin by name, in `WireConnection::terminal_name`
/// and in the binding's `terminal_order`. Leaving either behind after a
/// rename does not merely mislabel the connection — it detaches it, and
/// the netlist emits the instance with a terminal nothing drives.
/// Returns how many terminal names were rewritten.
fn apply_symbol_pin_renames(
    model: &mut Model,
    reference: &CellViewRef,
    renames: &BTreeMap<String, String>,
) -> usize {
    model
        .unsaved_pin_renames
        .record(&reference.key(), renames);
    rename_placed_terminals(model, reference, renames)
}

/// Rewrite the terminal names `renames` (old name to new) gives every
/// placement of the cellview. Returns how many were rewritten.
fn rename_placed_terminals(
    model: &mut Model,
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
    model
        .workspace
        .for_each_schematic_editor_mut(|_, schematic| {
            renamed += schematic.rename_instance_terminals(reference, &lowered);
        });
    renamed += model
        .schematic
        .rename_instance_terminals(reference, &lowered);
    renamed
}

/// Record the edit away from `before` as one undo step called `step`.
pub(crate) fn record_symbol_edit(
    symbol: &mut SymbolUiState,
    model: &Model,
    before: &SymbolDocument,
    step: &str,
) {
    const MAX_SYMBOL_UNDO: usize = 128;
    let key = model.workspace.content.active_key();
    let mut snapshot = active_symbol_snapshot(model, before);
    snapshot.step = step.to_owned();
    push_symbol_undo_snapshot(symbol, key, snapshot, MAX_SYMBOL_UNDO);
}

fn push_symbol_undo_snapshot(
    symbol: &mut SymbolUiState,
    key: String,
    snapshot: SymbolDocumentSnapshot,
    max_len: usize,
) {
    symbol.history.record_edit(key, snapshot, max_len);
}

/// How many pin findings stand against the active symbol view, or `None`
/// where the checks have not run against it.
///
/// The distinction is the point: `Some(0)` says the checks ran and the
/// pins match, which is a different claim from nobody having looked, and
/// a control that offered to clear nothing would be a dead control.
pub(in crate::workbench) fn active_symbol_pin_finding_count(model: &Model) -> Option<usize> {
    active_symbol_pin_records(model).map(|records| records.len())
}

/// Withdraw the active symbol view's pin findings.
pub(in crate::workbench) fn clear_active_symbol_pin_checks(model: &mut Model) {
    let subject = model.workspace.content.active_view.clone();
    model.withdraw_diagnostics(&DiagnosticInputs::SymbolPins { subject });
}

fn active_symbol_pin_records(model: &Model) -> Option<&[DiagnosticRecord]> {
    let published = DiagnosticInputs::SymbolPins {
        subject: model.workspace.content.active_view.clone(),
    };
    model
        .diagnostics
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
pub(crate) fn cycle_active_symbol_pin_finding(nav: &mut NavigationParts<'_>, step: isize) -> bool {
    let mut visitable: Vec<VisitablePinFinding> = active_symbol_pin_records(nav.model)
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
    let standing = nav
        .model
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
    nav.model
        .diagnostics
        .set_current_finding(Some(found.identity));
    nav.navigate_to_log_anchor(found.anchor);
    nav.wb.prompt(format!(
        "Finding {}/{} — {} · {}",
        index + 1,
        len,
        found.level.display_name(),
        found.message,
    ));
    true
}

/// Check the active symbol's pins against its schematic and publish what
/// the check finds, with one Console line saying it ran.
pub(crate) fn run_active_symbol_pin_checks(model: &mut Model, facts: SlotFacts) {
    let reference = model.workspace.content.active_view.clone();
    let inputs = match model.symbol_pin_check_inputs(&reference) {
        Ok(inputs) => inputs,
        Err(error) => {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Symbol pin checks could not run: {error}"),
            );
            return;
        }
    };
    let source = model.symbol_contract_source(&reference);
    let mut result = check_symbol_pins(
        &inputs.document,
        &inputs.interface,
        inputs.binding.as_ref(),
        &reference,
        source,
    );
    // The symbol's own waivers, stored in the document the checks read.
    rspice_design::drc::CheckWaivers::for_subject(&inputs.document.waivers).apply(&mut result);
    let found = result.total_count();
    if let Err(error) = publish_active_symbol_pin_check_result(model, facts, &result) {
        model.record(
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
        match inputs.interface {
            SymbolInterface::SymbolDefined => {
                "Symbol pin check: the symbol defines its own pins.".to_owned()
            }
            SymbolInterface::Declared(_) => format!("Symbol pin check: pins match the {source}."),
        }
    } else {
        format!(
            "Symbol pin check: {}. See Problems.",
            counted(found, "finding", "findings")
        )
    };
    model.record(LogSeverity::Info, LogSource::Studio, summary);
}

/// Publish what the symbol pin checks found, as canonical records.
///
/// Its own entry point with its own guard rather than a loosened
/// [`Model::publish_active_design_check_result`]: that guard is right to
/// refuse a symbol, because schematic design-check evidence is keyed to a
/// cell view that has a schematic and carries a receipt the symbol checks
/// have no part in.
fn publish_active_symbol_pin_check_result(
    model: &mut Model,
    facts: SlotFacts,
    result: &rspice_design::drc::DrcResult,
) -> Result<(), String> {
    if model.workspace.content.active_view_type() != Some(crate::state::ViewType::Symbol) {
        return Err("symbol pin findings can only be published from a symbol view".to_owned());
    }
    let subject = model.workspace.content.active_view.clone();
    // Derived before the batch is inserted, because resolving a route
    // reads the state the insertion takes mutably.
    let mut records = symbol_pin_records(model, facts, &subject, result);
    // The symbol's own waivers that do not do what they say are stated
    // with the pass that found that out.
    if let Ok(document) = model.load_symbol_document(&subject) {
        records.extend(crate::workbench::app_state::waivers::stored_waiver_records(
            model,
            &subject,
            &document.waivers,
            result,
        ));
    }
    model.publish_diagnostics(DiagnosticInputs::SymbolPins { subject }, records, facts)
}

/// One pin finding the tour can visit: how bad it is, what the store calls
/// it, which pin it lands on, and what it says.
struct VisitablePinFinding {
    level: DiagnosticLevel,
    identity: uuid::Uuid,
    anchor: LogAnchor,
    message: String,
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
    model: &Model,
    facts: SlotFacts,
    subject: &CellViewRef,
    result: &rspice_design::drc::DrcResult,
) -> Vec<DiagnosticRecord> {
    let project = model.workspace.content.project.id();
    let revision = model.workspace.content.project.revision().get();
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

            let route = crate::schematic::view::violations::location_anchor(
                model,
                facts.review_markers,
                &violation.location,
            )
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
