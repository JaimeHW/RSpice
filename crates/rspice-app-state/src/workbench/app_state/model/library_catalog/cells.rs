//! Cells and views of the catalog: copying and renaming them, and what a
//! deletion leaves behind in the workspace.

use super::{Model, WorkspacePruned};
use crate::state::{CellViewRef, OpenCellView, SchematicState, View, ViewType};

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

impl Model {
    /// The type of the view `reference` names, or `None` where the catalog
    /// has no such view.
    pub fn cell_view_type(&self, reference: &CellViewRef) -> Option<ViewType> {
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(|view| view.view_type)
    }

    /// Take a deleted cell out of every workspace collection that named it.
    ///
    /// Every view of the cell goes, not just the schematic ones: a layout
    /// document the cell no longer owns is a document the next save refuses to
    /// write. What survives is the placements — deleting a master is not a
    /// licence to edit the drawings that used it — so they are revalidated
    /// here instead, and read as unresolved from this point on.
    pub fn prune_workspace_after_cell_deleted(
        &mut self,
        library: &str,
        cell: &str,
    ) -> WorkspacePruned {
        let prompt = self.sync_active_schematic_to_workspace();
        let removal = self.drop_workspace_references_to_cell(library, cell);
        WorkspacePruned {
            prompt,
            ..self.restore_valid_workspace_focus_after_prune(
                removal.active_removed,
                removal.hierarchy_pruned,
                true,
            )
        }
    }

    pub fn prune_workspace_after_view_deleted(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) -> WorkspacePruned {
        let prompt = self.sync_active_schematic_to_workspace();
        let deleted = CellViewRef::new(library, cell, view);
        let active_removed = self.workspace.content.active_view == deleted;
        self.workspace.remove_schematic_editor(&deleted.key());
        self.workspace
            .content
            .remove_physical_layout_document(&deleted);
        self.workspace
            .content
            .open_views
            .retain(|open| open.reference != deleted);
        let hierarchy_pruned = self
            .workspace
            .content
            .retain_valid_occurrences(|reference| reference != &deleted);

        let preferred = CellViewRef::new(
            library,
            cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let preferred = reference_exists_in(&self.library_manager, &preferred).then_some(preferred);
        WorkspacePruned {
            prompt,
            ..self.restore_valid_workspace_focus_with_preferred(
                active_removed,
                hierarchy_pruned,
                preferred,
                true,
            )
        }
    }

    pub(super) fn restore_valid_workspace_focus_after_prune(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        clear_execution_context: bool,
    ) -> WorkspacePruned {
        self.restore_valid_workspace_focus_with_preferred(
            active_removed,
            hierarchy_pruned,
            None,
            clear_execution_context,
        )
    }

    /// Bring the workspace's focus back to a view the catalog still holds,
    /// and hand back the editor's reload and the execution context's
    /// retirement for the caller to carry out after it.
    ///
    /// The focus falls back to the preferred view, else an open tab, else
    /// the project top's schematic, which a deletion never takes. The
    /// project top itself is never moved here: it is chosen, not guessed.
    fn restore_valid_workspace_focus_with_preferred(
        &mut self,
        active_removed: bool,
        hierarchy_pruned: bool,
        preferred: Option<CellViewRef>,
        clear_execution_context: bool,
    ) -> WorkspacePruned {
        let libraries = &self.library_manager;
        self.workspace
            .content
            .open_views
            .retain(|open| reference_exists_in(libraries, &open.reference));
        self.workspace
            .content
            .retain_valid_occurrences(|reference| reference_exists_in(libraries, reference));

        let active_valid = !active_removed
            && reference_exists_in(&self.library_manager, &self.workspace.content.active_view);
        let fallback = if active_valid {
            self.workspace.content.active_view.clone()
        } else {
            preferred
                .filter(|reference| reference_exists_in(&self.library_manager, reference))
                .or_else(|| {
                    self.workspace
                        .content
                        .open_views
                        .iter()
                        .find(|open| reference_exists_in(&self.library_manager, &open.reference))
                        .map(|open| open.reference.clone())
                })
                .or_else(|| {
                    let project = &self.workspace.content.project;
                    let top = CellViewRef::new(
                        &project.root_library,
                        &project.top_cell,
                        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
                    );
                    reference_exists_in(&self.library_manager, &top).then_some(top)
                })
                .or_else(|| first_schematic_reference_in(&self.library_manager))
                .unwrap_or_else(|| self.create_fallback_schematic_cell())
        };
        // Every survivor above is a view the catalog holds; the only other
        // fallback is the schematic cell the prune has just made.
        let fallback_type = self
            .cell_view_type(&fallback)
            .unwrap_or(ViewType::Schematic);

        if !self
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == fallback)
        {
            self.workspace
                .content
                .open_views
                .push(OpenCellView::new(fallback.clone(), fallback_type));
        }
        if !active_valid {
            self.workspace.content.active_view = fallback.clone();
        }
        // Whatever the restored document was reached through did not survive
        // the prune, so it is a design root again rather than an occurrence
        // whose ancestors are gone.
        if hierarchy_pruned || !active_valid {
            self.workspace.content.reroot_active_occurrence();
        }
        if is_schematic_like(fallback_type) {
            self.workspace.ensure_active_buffer();
        }
        WorkspacePruned {
            restore_active_schematic: true,
            clear_execution_context,
            ..WorkspacePruned::default()
        }
    }

    /// A new, empty schematic cell for the focus to rest on when the catalog
    /// holds no schematic at all.
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

        if let Some(mut library) = self.library_manager.edit_library(&library_name) {
            let mut cell = crate::state::Cell::new(&cell_name);
            cell.add_view(View::new(&view_name, ViewType::Schematic));
            library.add_cell(cell);
        }

        let reference = CellViewRef::new(library_name, cell_name, view_name);
        if !self
            .workspace
            .content
            .schematic_buffers
            .contains_key(&reference.key())
        {
            self.workspace
                .insert_schematic_editor(reference.key(), SchematicState::default());
        }
        reference
    }

    /// Copy a whole cell — every view and its drawn content — into a
    /// writable library under a new name. Returns the number of views
    /// copied, or the user-facing error, beside what the drawing sync had to
    /// say on the status bar: the open schematic is synced once the copy is
    /// one the catalog can take, before the copy reads the project.
    pub fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> (Result<usize, String>, Option<String>) {
        let (outcome, prompt) = self.copy_cell_hierarchy(
            src_library,
            cell,
            dst_library,
            new_name,
            super::HierarchyCopy::default(),
        );
        (outcome.map(|(_, views)| views), prompt)
    }

    /// The cell [`Self::copy_cell`] copies, refusing a copy the destination
    /// library cannot take.
    pub(super) fn cell_to_copy(
        &self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<crate::state::Cell, String> {
        let source = self
            .library_manager
            .get_library(src_library)
            .ok_or_else(|| format!("Library '{src_library}' not found"))?;
        let copy = source
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
                "Cell '{new_name}' conflicts with an existing cell in library '{dst_library}' whose name differs only in case or normalization"
            ));
        }
        Ok(copy)
    }

    /// Rename a cell and remap everything that pointed at it: view buffers,
    /// open workspace references, and the Library/Cell binding of every
    /// instance in this project's designs. Returns the number of instance
    /// bindings remapped, or the user-facing error.
    pub fn rename_cell(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> Result<(usize, WorkspacePruned), String> {
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
        if crate::state::canonical_cell_view_owner_key(library, cell, "")
            == crate::state::canonical_cell_view_owner_key(library, new_name, "")
        {
            return Err(super::libraries::case_only_rename_refusal("cell"));
        }
        let requested_identity = crate::state::canonical_cell_view_owner_key(library, new_name, "");
        if lib.cells.iter().any(|(candidate_key, candidate)| {
            candidate_key != cell
                && crate::state::canonical_cell_view_owner_key(library, &candidate.name, "")
                    == requested_identity
        }) {
            return Err(format!(
                "Cell '{new_name}' conflicts with an existing cell in library '{library}' whose name differs only in case or normalization"
            ));
        }

        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_cell_view_bundles(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's project source ownership: {error}")
            })?;
        let mut candidate_configurations = self.workspace.content.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_cell_roots(library, cell, new_name)
            .map_err(|error| {
                format!("Could not move the cell's configuration-set roots: {error}")
            })?;
        let mut candidate_design_management = self.workspace.content.design_management.clone();
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
            .content
            .prepare_rename_physical_layout_cell_documents(library, cell, new_name)
            .map_err(|error| format!("Could not rename the cell's physical layouts: {error}"))?;
        let project_mutation = self.preflight_project_library_mutation(
            crate::state::ProjectLibraryMutation::RenameCell {
                library: library.to_owned(),
                from_cell: cell.to_owned(),
                to_cell: new_name.to_owned(),
            },
        )?;

        let mut library_edit = self
            .library_manager
            .edit_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the rename"))?;
        if !library_edit.rename_cell(cell, new_name) {
            return Err(format!("Cell '{cell}' disappeared during the rename"));
        }

        // Buffers move with the cell.
        let old_prefix = format!("{library}/{cell}/");
        let moved_keys: Vec<String> = self
            .workspace
            .content
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&old_prefix))
            .cloned()
            .collect();
        for key in moved_keys {
            if let Some(buffer) = self.workspace.remove_schematic_editor(&key) {
                let tail = &key[old_prefix.len()..];
                self.workspace
                    .insert_schematic_editor(format!("{library}/{new_name}/{tail}"), buffer);
            }
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        // Open references follow.
        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                reference.cell = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.content.active_view);
        for open in &mut self.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.content.remap_occurrence_masters(&remap_ref);
        rspice_design::added_lines::remap_added_lines(
            &mut self.workspace.content.added_lines,
            &remap_ref,
        );

        // Instance bindings follow — in every buffer and the live sheet.
        let mut remapped = 0usize;
        let mut remap_schematic = |schematic: &mut crate::state::SchematicState| {
            remapped += schematic.rename_cell_bindings(library, cell, new_name);
        };
        self.workspace
            .for_each_schematic_editor_mut(|_, buffer| remap_schematic(buffer));
        remap_schematic(&mut self.schematic);

        if !renamed_source_ids.is_empty() {
            self.workspace.content.project_sources = candidate_sources;
            self.workspace.content.project_sources_dirty = true;
        }
        if renamed_configuration_roots > 0 {
            self.workspace.content.configuration_sets = candidate_configurations;
            self.workspace.content.project_metadata_dirty = true;
        }
        if design_management_changed {
            self.workspace.content.design_management = candidate_design_management;
        }
        // The project root names its top cell by name. Leaving the old name
        // behind makes `ProjectFile::validate` reject every later save.
        if self.workspace.content.project.root_library == library
            && self.workspace.content.project.top_cell == cell
        {
            self.workspace.content.project.top_cell = new_name.to_owned();
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_cell(library, new_name);
        Ok((
            remapped,
            WorkspacePruned {
                void_veriloga_evidence: renamed_source_ids,
                ..WorkspacePruned::default()
            },
        ))
    }

    /// Rename one view of one cell and remap the buffer, open references, and
    /// every instance binding that selected exactly that view. Returns the
    /// number of instance bindings remapped.
    pub fn rename_view(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> Result<(usize, WorkspacePruned), String> {
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
        if view == new_name {
            return Err(format!("View '{view}' already has that name"));
        }
        if crate::state::canonical_cell_view_owner_key(library, cell, view) == requested_identity {
            return Err(super::libraries::case_only_rename_refusal("view"));
        }
        if owning_cell.views.iter().any(|(candidate_key, candidate)| {
            candidate_key != view
                && crate::state::canonical_cell_view_owner_key(library, cell, &candidate.name)
                    == requested_identity
        }) {
            return Err(format!(
                "View '{new_name}' conflicts with an existing view in cell '{library}/{cell}' whose name differs only in case or normalization"
            ));
        }
        let candidate_layouts = self
            .workspace
            .content
            .prepare_rename_physical_layout_view_documents(library, cell, view, new_name)
            .map_err(|error| format!("Could not rename the view's physical layouts: {error}"))?;

        let mut candidate_configurations = self.workspace.content.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_view_roots(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's configuration-set roots: {error}")
            })?;
        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_view_bundles(library, cell, view, new_name)
            .map_err(|error| {
                format!("Could not move the view's project source ownership: {error}")
            })?;
        let mut candidate_design_management = self.workspace.content.design_management.clone();
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

        let renamed = self
            .library_manager
            .edit_library(library)
            .and_then(|mut library| library.rename_view(cell, view, new_name))
            .ok_or_else(|| format!("Cell '{library}/{cell}' disappeared during the rename"))?;
        if !renamed {
            return Err(format!("View '{view}' disappeared during the rename"));
        }

        let old_reference = CellViewRef::new(library, cell, view);
        let new_reference = CellViewRef::new(library, cell, new_name);
        if let Some(buffer) = self.workspace.remove_schematic_editor(&old_reference.key()) {
            self.workspace
                .insert_schematic_editor(new_reference.key(), buffer);
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if *reference == old_reference {
                *reference = new_reference.clone();
            }
        };
        remap_ref(&mut self.workspace.content.active_view);
        for open in &mut self.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.content.remap_occurrence_masters(&remap_ref);
        rspice_design::added_lines::remap_added_lines(
            &mut self.workspace.content.added_lines,
            &remap_ref,
        );

        let remapped = self.remap_instance_bindings(|schematic| {
            schematic.rename_view_bindings(library, cell, view, new_name)
        });

        self.commit_renamed_project_sources(candidate_sources, &renamed_source_ids);
        if renamed_configuration_roots > 0 {
            self.workspace.content.configuration_sets = candidate_configurations;
            self.workspace.content.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_view(library, cell, new_name);
        Ok((
            remapped,
            WorkspacePruned {
                void_veriloga_evidence: renamed_source_ids,
                ..WorkspacePruned::default()
            },
        ))
    }

    /// Rewrite the Library/Cell/View binding of every instance in every loaded
    /// buffer and in the live sheet. `rebind` applies a specific identity rename
    /// and reports the number of affected placements.
    ///
    /// The live sheet and the buffer under the active key are two copies of one
    /// document, so both move but only one is counted.
    pub(super) fn remap_instance_bindings(
        &mut self,
        rebind: impl Fn(&mut SchematicState) -> usize,
    ) -> usize {
        let active_key = self.workspace.content.active_key();
        let mut remapped = 0usize;
        self.workspace.for_each_schematic_editor_mut(|key, buffer| {
            let count = rebind(buffer);
            if !key.eq_ignore_ascii_case(&active_key) {
                remapped += count;
            }
        });
        remapped += rebind(&mut self.schematic);
        remapped
    }

    /// Loaded instances of one library's masters placed from outside it,
    /// counted once per instance.
    ///
    /// A library's own cells instancing each other is ordinary hierarchy and
    /// goes with the library; only a placement that would survive it is a
    /// reason to refuse the deletion, and it is the number the review states.
    pub fn external_instance_references_to_library(&self, library: &str) -> usize {
        let active_key = self.workspace.content.active_key();
        let owned_prefix = format!("{library}/");
        let count = |document: &rspice_design::schematic::document::SchematicDocument| {
            document
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
        let live = if self.workspace.content.active_view.library == library {
            0
        } else {
            count(self.schematic.document())
        };
        live + self
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| {
                !key.eq_ignore_ascii_case(&active_key) && !key.starts_with(&owned_prefix)
            })
            .map(|(_, schematic)| count(schematic.document()))
            .sum::<usize>()
    }

    /// Adopt a prepared source registry whose bundles' owners just moved.
    /// The transient Verilog-A evidence naming one of them is the caller's
    /// to drop, through [`WorkspacePruned::void_veriloga_evidence`].
    pub(super) fn commit_renamed_project_sources(
        &mut self,
        candidate: crate::state::ProjectSourceRegistry,
        renamed: &[crate::state::ProjectSourceId],
    ) {
        if renamed.is_empty() {
            return;
        }
        self.workspace.content.project_sources = candidate;
        self.workspace.content.project_sources_dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::pdk_config::PdkConfig;
    use crate::state::{Cell, Library};

    /// A view's type is the catalog's, and a view the catalog does not hold
    /// has none: it is not taken for a schematic.
    #[test]
    fn a_view_the_catalog_does_not_hold_has_no_type() {
        let mut model = Model::session_start(
            PdkConfig::default(),
            None,
            crate::workbench::workspace::restore_model_library_manager,
        );
        let mut library = Library::new("catalog_test");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        model.library_manager.add_library(library);

        assert_eq!(
            model.cell_view_type(&CellViewRef::new("catalog_test", "amp", "symbol")),
            Some(ViewType::Symbol)
        );
        for missing in [
            CellViewRef::new("catalog_test", "amp", "schematic"),
            CellViewRef::new("catalog_test", "gone", "symbol"),
            CellViewRef::new("gone", "amp", "symbol"),
        ] {
            assert_eq!(model.cell_view_type(&missing), None, "{missing:?}");
        }
    }
}
