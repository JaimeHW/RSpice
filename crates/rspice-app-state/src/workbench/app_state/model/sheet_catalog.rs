//! The open cell view's sheet catalog: which sheet the session edits, and
//! the one transaction an edit of the catalog publishes.
//!
//! Sheets are project-owned identities, so every edit here publishes one
//! reviewed design-management transaction against the live project rather
//! than editing a dialog draft. Activation is navigation, not authoring, so
//! it syncs the live schematic into its buffer first, then retires selection
//! and net highlighting: stale identities from the sheet being left must
//! never reach a command run on the sheet being entered. Whichever feature
//! brings a sheet forward, the sheet strip, a chord, the hierarchy tree or a
//! followed finding, it is this one operation.

use super::Model;
use crate::state::{
    DesignManagementCatalog, SchematicState, Selection, SheetCatalog, SheetDefinition,
    SheetDeleteReceipt, SheetDeleteResolution, SheetId,
};
use crate::workbench::app_state::design_history::DesignManagementHistoryEntry;
use std::collections::{BTreeMap, HashSet};

impl Model {
    /// The refusal of a sheet operation where the open cell view has no
    /// catalog.
    pub const NO_SHEET_CATALOG: &'static str =
        "The active cell view has no governed sheet catalog.";
    /// The refusal of a sheet operation naming a sheet the catalog has
    /// dropped.
    pub const NO_SUCH_SHEET: &'static str = "That sheet is no longer part of the governed catalog.";

    /// The sheet catalog of the cell view the editor has open, if it has one.
    pub fn active_sheet_catalog(&self) -> Option<&SheetCatalog> {
        self.workspace
            .content
            .design_management
            .sheet_catalog(&self.workspace.content.active_schematic_reference().key())
    }

    /// The open cell view's sheet catalog while it has more than one sheet.
    /// One sheet is the drawing itself, so there is no sheet to tell apart.
    pub fn multi_sheet_catalog(&self) -> Option<&SheetCatalog> {
        self.active_sheet_catalog()
            .filter(|catalog| catalog.sheets().len() > 1)
    }

    /// The sheet the open cell view's catalog has active.
    pub fn active_sheet_id(&self) -> Option<SheetId> {
        self.active_sheet_catalog()
            .and_then(SheetCatalog::active_sheet_id)
    }

    /// What a message calls a sheet of the open cell view's catalog: its
    /// name, or its id where the catalog no longer holds it.
    pub fn sheet_name(&self, id: SheetId) -> String {
        self.active_sheet_catalog()
            .and_then(|catalog| catalog.find(id))
            .map_or_else(|| id.to_string(), |sheet| sheet.name().to_owned())
    }

    /// Make one sheet the canvas the session is editing, answering what the
    /// status bar says of it either way.
    ///
    /// Re-activating the sheet already active is inert: it publishes nothing,
    /// so a chip redrawn every frame cannot advance the project revision.
    /// Otherwise `let_go` first lets go of any gesture still under way on the
    /// sheet being left; the caller holds it.
    pub fn activate_sheet(
        &mut self,
        id: SheetId,
        let_go: impl FnOnce(&mut Self),
    ) -> Result<String, String> {
        let name = self.sheet_name(id);
        if self.active_sheet_id() == Some(id) {
            return Ok(format!("Sheet `{name}` is already active."));
        }
        // Objects authored on the sheet being left take their membership from
        // the sheet that is still active, so the buffer is synchronized first.
        // The outcome the caller states replaces anything the sync would say.
        let_go(self);
        let _ = self.sync_active_schematic_to_workspace();
        self.edit_sheet_catalog("activate sheet", |catalog| {
            catalog.set_active(id).map_err(|error| error.to_string())
        })?;
        self.enter_active_sheet();
        Ok(format!("Sheet `{name}` is active."))
    }

    /// [`Self::activate_sheet`] for the sheet at `index` in catalog order.
    pub fn go_to_sheet(
        &mut self,
        index: usize,
        let_go: impl FnOnce(&mut Self),
    ) -> Result<String, String> {
        let Some(id) = self
            .active_sheet_catalog()
            .and_then(|catalog| catalog.sheets().get(index))
            .map(|sheet| sheet.id())
        else {
            return Err(Self::NO_SUCH_SHEET.to_owned());
        };
        self.activate_sheet(id, let_go)
    }

    /// [`Self::go_to_sheet`] for the sheet `delta` places from the active
    /// one, wrapping at the ends.
    pub fn step_sheet(
        &mut self,
        delta: isize,
        let_go: impl FnOnce(&mut Self),
    ) -> Result<String, String> {
        let Some(catalog) = self.active_sheet_catalog() else {
            return Err(Self::NO_SHEET_CATALOG.to_owned());
        };
        let count = catalog.sheets().len();
        if count < 2 {
            return Err("The active cell view has a single sheet.".to_owned());
        }
        let current = catalog
            .active_sheet_id()
            .and_then(|active| {
                catalog
                    .sheets()
                    .iter()
                    .position(|sheet| sheet.id() == active)
            })
            .unwrap_or_default();
        // Wrapping is the surface's choice, not the catalog's: `next_sheet`
        // stops at the ends so the durable order stays a line rather than a
        // ring.
        let index = (current as isize + delta).rem_euclid(count as isize) as usize;
        self.go_to_sheet(index, let_go)
    }

    /// Retire everything that named the sheet just left and frame the new one.
    pub fn enter_active_sheet(&mut self) {
        let editor = &mut self.schematic.session.editor;
        editor.selection.clear();
        editor.net_highlight.clear();
        editor.needs_drawing_sheet_fit = true;
        editor.needs_fit = false;
    }

    /// The part of `source` on the active sheet: each object it selects that
    /// the sheet holds, with the wire segments and vertices it selects on
    /// those wires. Every object is on the sheet of a cell view without a
    /// catalog.
    pub fn selection_on_active_sheet(&self, source: &Selection) -> Selection {
        let mut selection = Selection::default();
        // This runs twice a frame. Nothing selected keeps nothing, and the sheet
        // catalog is looked up once rather than once per object.
        if source.is_empty() {
            return selection;
        }
        let catalog = self.active_sheet_catalog();
        let on_sheet = |id: u64| {
            rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id)
        };

        for object in &self.schematic.document().components {
            if source.has_component(object.id) && on_sheet(object.id) {
                selection.select_component(object.id);
            }
        }
        let wire_parts = !source.wire_segments.is_empty() || !source.wire_vertices.is_empty();
        for object in &self.schematic.document().wires {
            let whole = source.has_wire(object.id);
            if !(whole || wire_parts) || !on_sheet(object.id) {
                continue;
            }
            if whole {
                selection.select_wire(object.id);
            }
            if wire_parts {
                selection.wire_segments.extend(
                    source
                        .wire_segments
                        .iter()
                        .filter(|selected| selected.wire_id == object.id)
                        .copied(),
                );
                selection.wire_vertices.extend(
                    source
                        .wire_vertices
                        .iter()
                        .filter(|selected| selected.wire_id == object.id)
                        .copied(),
                );
            }
        }
        for object in &self.schematic.document().junctions {
            if source.has_junction(object.pos) && on_sheet(object.id) {
                selection.select_junction(object.pos);
            }
        }
        for object in &self.schematic.document().buses {
            if source.has_bus(object.id) && on_sheet(object.id) {
                selection.select_bus(object.id);
            }
        }
        for object in &self.schematic.document().bus_taps {
            if source.has_bus_tap(object.id) && on_sheet(object.id) {
                selection.select_bus_tap(object.id);
            }
        }
        for object in &self.schematic.document().net_labels {
            if source.has_net_label(object.id) && on_sheet(object.id) {
                selection.select_net_label(object.id);
            }
        }
        for object in &self.schematic.document().design_notes {
            if source.has_design_note(object.id) && on_sheet(object.id) {
                selection.select_design_note(object.id);
            }
        }
        for object in &self.schematic.document().documentation_shapes {
            if source.has_documentation_shape(object.id) && on_sheet(object.id) {
                selection.select_documentation_shape(object.id);
            }
        }
        for object in &self.schematic.document().probes {
            if source.has_probe(object.id) && on_sheet(object.id) {
                selection.select_probe(object.id);
            }
        }
        selection
    }

    /// Keep in the session's selection only the objects on the active sheet.
    /// This is the sheet-switch boundary applied: stale IDs from a previously
    /// active sheet never reach keyboard commands or context-menu actions.
    pub fn retain_selection_on_active_sheet(&mut self) {
        let selection = self.selection_on_active_sheet(&self.schematic.session.editor.selection);
        self.schematic.session.editor.selection = selection;
    }

    /// Apply one edit to the active cell view's sheet catalog and publish the
    /// result as a single project transaction, so undo restores the whole
    /// change.
    pub fn edit_sheet_catalog<T>(
        &mut self,
        description: &str,
        edit: impl FnOnce(&mut SheetCatalog) -> Result<T, String>,
    ) -> Result<T, String> {
        let key = self.workspace.content.active_schematic_reference().key();
        let before = (*self.workspace.content.design_management).clone();
        let mut candidate = before.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .ok_or_else(|| Self::NO_SHEET_CATALOG.to_owned())?;
        let outcome = edit(catalog)?;
        self.commit_sheet_catalog(description, before, candidate)?;
        Ok(outcome)
    }

    /// Delete sheet `id` of the open cell view with every object it holds,
    /// as one step: the catalog drops the sheet and the drawing drops its
    /// objects in the same transaction, so one undo gives both back.
    pub fn delete_sheet_with_its_objects(
        &mut self,
        id: SheetId,
    ) -> Result<SheetDeleteReceipt, String> {
        let owner = self.workspace.content.active_schematic_reference();
        let key = owner.key();
        let before = (*self.workspace.content.design_management).clone();
        let mut candidate = before.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .ok_or_else(|| Self::NO_SHEET_CATALOG.to_owned())?;
        let receipt = catalog
            .delete_sheet(catalog.revision(), id, SheetDeleteResolution::DeleteObjects)
            .map_err(|error| error.to_string())?;
        let doomed = receipt
            .deleted_object_ids
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        let after_schematic = self.schematic.without_objects_candidate(&doomed);
        if after_schematic.is_some() && self.schematic.session.read_only {
            return Err("The schematic is read-only, so its objects stay.".to_owned());
        }
        candidate.validate().map_err(|error| error.to_string())?;
        let before_schematic = self.schematic.clone();
        let committed_revision = self
            .workspace
            .content
            .replace_design_management(candidate)
            .map_err(|error| error.to_string())?;
        if let Some(after_schematic) = after_schematic {
            self.schematic = after_schematic;
            self.workspace
                .insert_schematic_editor(key.clone(), self.schematic.clone());
        }
        let after = (*self.workspace.content.design_management).clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: "delete sheet".to_owned(),
            owner,
            before,
            after,
            before_schematics: BTreeMap::from([(key.clone(), before_schematic)]),
            after_schematics: BTreeMap::from([(key, self.schematic.clone())]),
            references: Default::default(),
            committed_revision,
        });
        Ok(receipt)
    }

    /// Duplicate sheet `source` of the open cell view, drawing and all, as
    /// one step: a new sheet, named by `definition` from the catalog and the
    /// sheet it copies, goes after it on the same drawing sheet, holding a
    /// copy of every object on `source` where the original stands, with
    /// fresh identities and reference names. A drawing with no catalog yet
    /// is one sheet, and `None` duplicates it: everything drawn becomes
    /// Sheet 1 in the same step. One undo takes the new sheet and its copy
    /// away together. The new sheet becomes the sheet in view in the same
    /// step. Returns the new sheet.
    pub fn duplicate_sheet(
        &mut self,
        source: Option<SheetId>,
        definition: impl FnOnce(&SheetCatalog, SheetId) -> Result<SheetDefinition, String>,
    ) -> Result<SheetId, String> {
        if self.schematic.session.read_only {
            return Err("The schematic is read-only, so its sheets stay as they are.".to_owned());
        }
        let owner = self.workspace.content.active_schematic_reference();
        let key = owner.key();
        let before = (*self.workspace.content.design_management).clone();
        let mut candidate = before.clone();
        let source = match source {
            Some(source) => source,
            None => candidate
                .bootstrap_for_cell_view(&key, "Sheet 1", live_object_ids(&self.schematic))
                .map_err(|error| error.to_string())?,
        };
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .ok_or_else(|| Self::NO_SHEET_CATALOG.to_owned())?;
        let definition = definition(catalog, source)?;
        let page_format = catalog
            .find(source)
            .ok_or_else(|| Self::NO_SUCH_SHEET.to_owned())?
            .page_format()
            .clone();
        let on_source = catalog.objects_on_sheet(source).collect::<HashSet<_>>();
        let duplicate = catalog
            .create_sheet_with_page_format(definition, Some(source), page_format)
            .map_err(|error| error.to_string())?;
        // The copy becomes the sheet in view in the same step, so one undo
        // also returns to the sheet it was made from.
        catalog
            .set_active(duplicate)
            .map_err(|error| error.to_string())?;

        let before_schematic = self.schematic.clone();
        let mut copying = self.schematic.clone();
        copying.session.editor.selection = self.every_object_among(&on_source);
        let symbols = self.schematic_symbol_context();
        let drawing = copying.capture_complete_selection_resolved(
            |component| symbols.terminal_points(component),
            &|id| on_source.contains(&id),
        );
        let existing = live_object_ids(&self.schematic);
        // Every object already drawn is on another sheet than the copy's, so
        // the copy's dots join only its own wires.
        if let Some(after_schematic) = self
            .schematic
            .pasted_in_place_candidate(&drawing, |id| existing.contains(&id))?
        {
            let copied = live_object_ids(&after_schematic)
                .into_iter()
                .filter(|id| !existing.contains(id))
                .collect::<Vec<_>>();
            if !copied.is_empty() {
                catalog
                    .assign_objects(catalog.revision(), duplicate, copied)
                    .map_err(|error| error.to_string())?;
            }
            self.schematic = after_schematic;
        }
        candidate.validate().map_err(|error| error.to_string())?;
        let committed_revision = match self.workspace.content.replace_design_management(candidate) {
            Ok(revision) => revision,
            Err(error) => {
                self.schematic = before_schematic;
                return Err(error.to_string());
            }
        };
        self.workspace
            .insert_schematic_editor(key.clone(), self.schematic.clone());
        let after = (*self.workspace.content.design_management).clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: "duplicate sheet".to_owned(),
            owner,
            before,
            after,
            before_schematics: BTreeMap::from([(key.clone(), before_schematic)]),
            after_schematics: BTreeMap::from([(key, self.schematic.clone())]),
            references: Default::default(),
            committed_revision,
        });
        self.enter_active_sheet();
        Ok(duplicate)
    }

    /// Every object of the open drawing whose identity `ids` holds, as a
    /// selection.
    fn every_object_among(&self, ids: &HashSet<u64>) -> Selection {
        let document = self.schematic.document();
        let mut selection = Selection::default();
        for object in document.components.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_component(object.id);
        }
        for object in document.wires.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_wire(object.id);
        }
        for object in document.junctions.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_junction(object.pos);
        }
        for object in document.buses.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_bus(object.id);
        }
        for object in document.bus_taps.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_bus_tap(object.id);
        }
        for object in document.net_labels.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_net_label(object.id);
        }
        for object in document.design_notes.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_design_note(object.id);
        }
        for object in document
            .documentation_shapes
            .iter()
            .filter(|o| ids.contains(&o.id))
        {
            selection.select_documentation_shape(object.id);
        }
        for object in document.probes.iter().filter(|o| ids.contains(&o.id)) {
            selection.select_probe(object.id);
        }
        selection
    }

    /// Publish `candidate` over `before` as one project transaction, with the
    /// schematic buffers it moves objects between.
    pub fn commit_sheet_catalog(
        &mut self,
        description: &str,
        before: DesignManagementCatalog,
        candidate: DesignManagementCatalog,
    ) -> Result<(), String> {
        candidate.validate().map_err(|error| error.to_string())?;
        let schematic_tx = self.prepare_design_management_schematic_transaction(&candidate)?;
        let owner = self.workspace.content.active_schematic_reference();
        let committed_revision = self
            .workspace
            .content
            .replace_design_management(candidate)
            .map_err(|error| error.to_string())?;
        self.apply_design_management_schematic_transaction(&schematic_tx);
        let after = (*self.workspace.content.design_management).clone();
        self.record_design_management_transaction(DesignManagementHistoryEntry {
            description: description.to_owned(),
            owner,
            before,
            after,
            before_schematics: schematic_tx.before,
            after_schematics: schematic_tx.after,
            references: schematic_tx.references,
            committed_revision,
        });
        Ok(())
    }
}

/// Every stable identity the drawing `schematic` holds.
fn live_object_ids(schematic: &SchematicState) -> HashSet<u64> {
    let document = schematic.document();
    let mut ids = HashSet::new();
    ids.extend(document.components.iter().map(|object| object.id));
    ids.extend(document.wires.iter().map(|object| object.id));
    ids.extend(document.buses.iter().map(|object| object.id));
    ids.extend(document.bus_taps.iter().map(|object| object.id));
    ids.extend(document.junctions.iter().map(|object| object.id));
    ids.extend(document.net_labels.iter().map(|object| object.id));
    ids.extend(document.design_notes.iter().map(|object| object.id));
    ids.extend(document.documentation_shapes.iter().map(|object| object.id));
    ids.extend(document.probes.iter().map(|object| object.id));
    ids
}
