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
use crate::state::{DesignManagementCatalog, SheetCatalog, SheetId};
use crate::workbench::app_state::DesignManagementHistoryEntry;

impl Model {
    /// The refusal of a sheet operation where the open cell view has no
    /// catalog.
    pub(crate) const NO_SHEET_CATALOG: &'static str =
        "The active cell view has no governed sheet catalog.";
    /// The refusal of a sheet operation naming a sheet the catalog has
    /// dropped.
    pub(crate) const NO_SUCH_SHEET: &'static str =
        "That sheet is no longer part of the governed catalog.";

    /// The sheet catalog of the cell view the editor has open, if it has one.
    pub(crate) fn active_sheet_catalog(&self) -> Option<&SheetCatalog> {
        self.workspace
            .content
            .design_management
            .sheet_catalog(&self.workspace.content.active_schematic_reference().key())
    }

    /// The open cell view's sheet catalog while it has more than one sheet.
    /// One sheet is the drawing itself, so there is no sheet to tell apart.
    pub(crate) fn multi_sheet_catalog(&self) -> Option<&SheetCatalog> {
        self.active_sheet_catalog()
            .filter(|catalog| catalog.sheets().len() > 1)
    }

    /// The sheet the open cell view's catalog has active.
    pub(crate) fn active_sheet_id(&self) -> Option<SheetId> {
        self.active_sheet_catalog()
            .and_then(SheetCatalog::active_sheet_id)
    }

    /// What a message calls a sheet of the open cell view's catalog: its
    /// name, or its id where the catalog no longer holds it.
    pub(crate) fn sheet_name(&self, id: SheetId) -> String {
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
    pub(crate) fn activate_sheet(
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
    pub(crate) fn go_to_sheet(
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
    pub(crate) fn step_sheet(
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
    pub(crate) fn enter_active_sheet(&mut self) {
        let editor = &mut self.schematic.session.editor;
        editor.selection.clear();
        editor.net_highlight.clear();
        editor.needs_drawing_sheet_fit = true;
        editor.needs_fit = false;
    }

    /// Apply one edit to the active cell view's sheet catalog and publish the
    /// result as a single project transaction, so undo restores the whole
    /// change.
    pub(crate) fn edit_sheet_catalog<T>(
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

    /// Publish `candidate` over `before` as one project transaction, with the
    /// schematic buffers it moves objects between.
    pub(crate) fn commit_sheet_catalog(
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
