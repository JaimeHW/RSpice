//! Copying a library, a view, or a cell with the cells it instantiates, and
//! moving a cell to another library: each one project transaction and one
//! step of Undo.

use std::collections::BTreeSet;

use super::{Model, WorkspacePruned};
use crate::state::{Cell, CellViewRef, ProjectLibraryMutation, View};

/// How Copy cell takes the cells the copied cell instantiates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HierarchyCopy {
    /// Copy every cell of a writable library the cell instantiates, at any
    /// depth, into the destination library under its own name.
    pub include_children: bool,
    /// The copies place the copies, not the cells they were copied from.
    pub repoint: bool,
}

/// One cell a hierarchical copy makes: where it comes from and the name it
/// takes in the destination library.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CellCopy {
    source: (String, String),
    target: (String, String),
}

impl Model {
    /// Copy `cell` into `dst_library` under `new_name`, and with
    /// `hierarchy` the cells it instantiates under their own names, as one
    /// step. Returns the number of cells and of views copied, or the
    /// user-facing error, beside what the drawing sync had to say on the
    /// status bar.
    pub fn copy_cell_hierarchy(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
        hierarchy: HierarchyCopy,
    ) -> (Result<(usize, usize), String>, Option<String>) {
        if let Err(error) = self.cell_to_copy(src_library, cell, dst_library, new_name) {
            return (Err(error), None);
        }
        let prompt = self.sync_active_schematic_to_workspace();
        let mut copies = vec![CellCopy {
            source: (src_library.to_owned(), cell.to_owned()),
            target: (dst_library.to_owned(), new_name.to_owned()),
        }];
        if hierarchy.include_children {
            for (library, child) in self.cells_instantiated_by(src_library, cell) {
                copies.push(CellCopy {
                    target: (dst_library.to_owned(), child.clone()),
                    source: (library, child),
                });
            }
        }
        let outcome = self.copy_cells(&copies, hierarchy.include_children && hierarchy.repoint);
        (outcome, prompt)
    }

    /// Every cell of a writable library that `library`/`cell` instantiates,
    /// in its schematics or its layouts, at any depth, the cell itself
    /// excepted, in the order it is first reached.
    fn cells_instantiated_by(&self, library: &str, cell: &str) -> Vec<(String, String)> {
        let content = &self.workspace.content;
        let writable = |library: &str| {
            self.library_manager
                .get_library(library)
                .is_some_and(|found| !found.read_only)
        };
        let root = (library.to_ascii_lowercase(), cell.to_ascii_lowercase());
        let mut seen = BTreeSet::from([root]);
        let mut found = Vec::new();
        let mut queue = vec![(library.to_owned(), cell.to_owned())];
        while let Some((library, cell)) = queue.pop() {
            let prefix = format!("{library}/{cell}/");
            let mut children = content
                .schematic_buffers
                .iter()
                .filter(|(key, _)| key.starts_with(&prefix))
                .flat_map(|(_, schematic)| {
                    schematic
                        .document()
                        .components
                        .iter()
                        .filter_map(|component| component.library_cell.as_ref())
                        .map(|binding| (binding.library.clone(), binding.cell.clone()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            children.extend(
                content
                    .physical_layout_documents()
                    .values()
                    .filter(|document| {
                        document.owner().library == library && document.owner().cell == cell
                    })
                    .flat_map(|document| {
                        document
                            .instances()
                            .values()
                            .map(|instance| {
                                (
                                    instance.master.library.clone(),
                                    instance.master.cell.clone(),
                                )
                            })
                            .collect::<Vec<_>>()
                    }),
            );
            children.sort();
            for (child_library, child) in children {
                let identity = (
                    child_library.to_ascii_lowercase(),
                    child.to_ascii_lowercase(),
                );
                let exists = self
                    .library_manager
                    .get_library(&child_library)
                    .is_some_and(|found| found.get_cell(&child).is_some());
                if exists && writable(&child_library) && seen.insert(identity) {
                    found.push((child_library.clone(), child.clone()));
                    queue.push((child_library, child));
                }
            }
        }
        found
    }

    /// Copy each of `copies` and everything it owns as one transaction; with
    /// `repoint`, each copy places the copies of the cells `copies` names
    /// where its original placed those cells. Returns the cells and the
    /// views copied.
    fn copy_cells(&mut self, copies: &[CellCopy], repoint: bool) -> Result<(usize, usize), String> {
        let mut records = Vec::with_capacity(copies.len());
        for copy in copies {
            let (library, cell) = &copy.source;
            let (target_library, target_cell) = &copy.target;
            let record = self
                .cell_to_copy(library, cell, target_library, target_cell)
                .map_err(|error| match records.is_empty() {
                    true => error,
                    false => format!(
                        "{library}/{cell}, which the copied cell instantiates, cannot be copied: {error}"
                    ),
                })?;
            records.push(record);
        }
        let mut targets = BTreeSet::new();
        for copy in copies {
            let (library, cell) = &copy.target;
            if !targets.insert((library.to_ascii_lowercase(), cell.to_ascii_lowercase())) {
                return Err(format!(
                    "{}/{}, which the copied cell instantiates, would be copied to '{library}/{cell}', which another copy takes.",
                    copy.source.0, copy.source.1
                ));
            }
        }
        let views = records.iter().map(|record| record.views.len()).sum();
        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let mut copied_source_ids = Vec::new();
        let mut candidate_design_management = self.workspace.content.design_management.clone();
        let mut copied_sheet_catalogs = 0;
        for copy in copies {
            let ((library, cell), (target_library, target_cell)) = (&copy.source, &copy.target);
            copied_source_ids.extend(
                candidate_sources
                    .clone_cell_view_bundles(library, cell, target_library, target_cell)
                    .map_err(|error| {
                        format!(
                            "Could not copy the project source bundles of {library}/{cell}: {error}"
                        )
                    })?,
            );
            copied_sheet_catalogs += candidate_design_management
                .copy_cell_sheet_catalogs(library, cell, target_library, target_cell)
                .map_err(|error| {
                    format!(
                        "Could not copy the sheet-catalog ownership of {library}/{cell}: {error}"
                    )
                })?
                .copied_sheet_catalogs;
        }
        let pairs = copies
            .iter()
            .map(|copy| {
                (
                    (copy.source.0.as_str(), copy.source.1.as_str()),
                    (copy.target.0.as_str(), copy.target.1.as_str()),
                )
            })
            .collect::<Vec<_>>();
        let candidate_layouts = self
            .workspace
            .content
            .prepare_copy_physical_layout_cells(&pairs, repoint)
            .map_err(|error| format!("Could not copy the physical layouts: {error}"))?;
        let mut operations = copies.iter().map(|copy| ProjectLibraryMutation::CopyCell {
            source_library: copy.source.0.clone(),
            source_cell: copy.source.1.clone(),
            target_library: copy.target.0.clone(),
            target_cell: copy.target.1.clone(),
        });
        let mutation = match copies.len() {
            1 => operations.next().expect("one copy"),
            _ => ProjectLibraryMutation::Batch {
                operations: operations.collect(),
            },
        };
        let project_mutation = self.preflight_project_library_mutation(mutation)?;

        let project = self.workspace.content.project.id();
        for (copy, mut record) in copies.iter().zip(records) {
            let ((library, cell), (target_library, target_cell)) = (&copy.source, &copy.target);
            record.name.clone_from(target_cell);
            // A copied model-bound symbol is a symbol of its own: it names
            // its new place and carries its own binding.
            let binding_id = rspice_design::model_bound_symbol::model_bound_binding_id(
                project,
                target_library,
                target_cell,
            );
            rspice_design::model_bound_symbol::restamp_model_bound_identity(
                &mut record,
                target_library,
                Some(&binding_id),
            );
            let view_names = record.views.keys().cloned().collect::<Vec<_>>();
            self.library_manager
                .edit_library(target_library)
                .ok_or_else(|| format!("Library '{target_library}' disappeared during the copy"))?
                .add_cell(record);
            // The drawn content lives in the workspace buffers. The
            // validation journal does not travel: a copy has been reviewed
            // by nobody.
            for view in view_names {
                let from = CellViewRef::new(library, cell, view.as_str()).key();
                let to = CellViewRef::new(target_library, target_cell, view.as_str()).key();
                if let Some(mut buffer) = self
                    .workspace
                    .schematic_editor(&from)
                    .map(|source| source.copy_without_validated_revisions())
                {
                    if repoint {
                        for (from, to) in &pairs {
                            buffer.rebind_cell_bindings(*from, *to);
                        }
                    }
                    self.workspace.insert_schematic_editor(to, buffer);
                }
            }
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);
        if !copied_source_ids.is_empty() {
            self.workspace.content.project_sources = candidate_sources;
            self.workspace.content.project_sources_dirty = true;
        }
        if copied_sheet_catalogs > 0 {
            self.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        let (library, cell) = &copies[0].target;
        self.library_manager.select_cell(library, cell);
        Ok((copies.len(), views))
    }

    /// Copy one view into a cell of a writable library under a name of its
    /// own, with its drawing, the source it owns, its sheet catalog and its
    /// layout; the cell is made when it does not exist. Returns what the
    /// drawing sync had to say on the status bar.
    pub fn copy_view(
        &mut self,
        source: &CellViewRef,
        target: &CellViewRef,
    ) -> (Result<(), String>, Option<String>) {
        let record = match self.view_to_copy(source, target) {
            Ok(record) => record,
            Err(error) => return (Err(error), None),
        };
        let prompt = self.sync_active_schematic_to_workspace();
        (self.copy_view_into(record, source, target), prompt)
    }

    /// The view [`Self::copy_view`] copies, refusing a copy the target cell
    /// cannot take.
    fn view_to_copy(&self, source: &CellViewRef, target: &CellViewRef) -> Result<View, String> {
        target
            .validate_name_segments()
            .map_err(|error| format!("The copy's name is invalid: {error}"))?;
        let view = self
            .library_manager
            .get_library(&source.library)
            .and_then(|library| library.get_cell(&source.cell))
            .and_then(|cell| cell.get_view(&source.view))
            .ok_or_else(|| format!("View '{}' not found", source.display_path()))?
            .clone();
        let destination = self
            .library_manager
            .get_library(&target.library)
            .ok_or_else(|| format!("Library '{}' not found", target.library))?;
        if destination.read_only {
            return Err(format!("Library '{}' is read-only", target.library));
        }
        let cell_identity =
            crate::state::canonical_cell_view_owner_key(&target.library, &target.cell, "");
        let Some(cell) = destination.cells.values().find(|cell| {
            crate::state::canonical_cell_view_owner_key(&target.library, &cell.name, "")
                == cell_identity
        }) else {
            return Ok(view);
        };
        if cell.name != target.cell {
            return Err(format!(
                "Cell '{}' conflicts with the existing cell '{}' of library '{}', whose name differs only in case or normalization",
                target.cell, cell.name, target.library
            ));
        }
        let view_identity = crate::state::canonical_cell_view_owner_key(
            &target.library,
            &target.cell,
            &target.view,
        );
        if cell.views.values().any(|view| {
            crate::state::canonical_cell_view_owner_key(&target.library, &target.cell, &view.name)
                == view_identity
        }) {
            return Err(format!(
                "Cell '{}/{}' already has a view named '{}'",
                target.library, target.cell, target.view
            ));
        }
        Ok(view)
    }

    fn copy_view_into(
        &mut self,
        mut record: View,
        source: &CellViewRef,
        target: &CellViewRef,
    ) -> Result<(), String> {
        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let copied_source_ids = candidate_sources
            .clone_view_bundles(source, target)
            .map_err(|error| format!("Could not copy the view's project source: {error}"))?;
        let mut candidate_design_management = self.workspace.content.design_management.clone();
        let copied_sheet_catalogs = candidate_design_management
            .copy_view_sheet_catalog(
                (&source.library, &source.cell, &source.view),
                (&target.library, &target.cell, &target.view),
            )
            .map_err(|error| format!("Could not copy the view's sheet catalog: {error}"))?
            .copied_sheet_catalogs;
        let candidate_layouts = match self.workspace.content.physical_layout_document(source) {
            Some(original) => Some(
                original
                    .copy_for_cell(&source.library, &source.cell, &target.library, &target.cell)
                    .and_then(|copy| {
                        copy.rename_view_references(
                            &target.library,
                            &target.cell,
                            &source.view,
                            &target.view,
                        )
                    })
                    .and_then(|copy| {
                        self.workspace
                            .content
                            .prepare_insert_physical_layout_document(copy)
                    })
                    .map_err(|error| format!("Could not copy the view's layout: {error}"))?,
            ),
            None => None,
        };
        let project_mutation =
            self.preflight_project_library_mutation(ProjectLibraryMutation::CopyView {
                source_library: source.library.clone(),
                source_cell: source.cell.clone(),
                source_view: source.view.clone(),
                target_library: target.library.clone(),
                target_cell: target.cell.clone(),
                target_view: target.view.clone(),
            })?;

        record.name.clone_from(&target.view);
        // The file a view was read from is its original's, not the copy's.
        record.file_path = None;
        // A model-bound symbol copied to another cell names its new place.
        let mut stamped = Cell::new(&target.cell);
        stamped.add_view(record);
        if (&source.library, &source.cell) != (&target.library, &target.cell) {
            let binding_id = rspice_design::model_bound_symbol::model_bound_binding_id(
                self.workspace.content.project.id(),
                &target.library,
                &target.cell,
            );
            rspice_design::model_bound_symbol::restamp_model_bound_identity(
                &mut stamped,
                &target.library,
                Some(&binding_id),
            );
        }
        let record = stamped
            .views
            .into_values()
            .next()
            .expect("the copy is the cell's one view");
        let mut library = self
            .library_manager
            .edit_library(&target.library)
            .ok_or_else(|| format!("Library '{}' disappeared during the copy", target.library))?;
        if library.library().get_cell(&target.cell).is_none() {
            library.add_cell(Cell::new(&target.cell));
        }
        library.add_view(&target.cell, record);
        drop(library);
        if let Some(buffer) = self
            .workspace
            .schematic_editor(&source.key())
            .map(|original| original.copy_without_validated_revisions())
        {
            self.workspace.insert_schematic_editor(target.key(), buffer);
        }
        if let Some(layouts) = candidate_layouts {
            self.workspace
                .content
                .commit_prepared_physical_layout_catalog(layouts);
        }
        if !copied_source_ids.is_empty() {
            self.workspace.content.project_sources = candidate_sources;
            self.workspace.content.project_sources_dirty = true;
        }
        if copied_sheet_catalogs > 0 {
            self.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager
            .select_view(&target.library, &target.cell, &target.view);
        Ok(())
    }

    /// Copy a whole library under a new name: every cell, view and drawing
    /// in it, the sources, sheet catalogs and layouts they own, as a
    /// writable library of this project. The copy's cells place each other
    /// where the original's placed each other. Returns the number of cells
    /// copied, beside what the drawing sync had to say on the status bar.
    pub fn copy_library(
        &mut self,
        library: &str,
        new_name: &str,
    ) -> (Result<usize, String>, Option<String>) {
        if let Err(error) = self.library_to_copy(library, new_name) {
            return (Err(error), None);
        }
        let prompt = self.sync_active_schematic_to_workspace();
        (self.copy_library_into(library, new_name), prompt)
    }

    fn library_to_copy(&self, library: &str, new_name: &str) -> Result<(), String> {
        self.library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        CellViewRef::new(new_name, "cell", "view")
            .validate_name_segments()
            .map_err(|error| format!("The copy's name is invalid: {error}"))?;
        if let Some(conflict) =
            super::libraries::conflicting_library_name(&self.library_manager, new_name, None)
        {
            return Err(format!(
                "Library '{new_name}' conflicts with the existing library '{conflict}', whose name differs only in case or normalization"
            ));
        }
        Ok(())
    }

    fn copy_library_into(&mut self, library: &str, new_name: &str) -> Result<usize, String> {
        let mut copy = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?
            .clone();
        let cells = copy.cells.keys().cloned().collect::<Vec<_>>();
        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let mut copied_source_ids = Vec::new();
        let mut candidate_design_management = self.workspace.content.design_management.clone();
        let mut copied_sheet_catalogs = 0;
        for cell in &cells {
            copied_source_ids.extend(
                candidate_sources
                    .clone_cell_view_bundles(library, cell, new_name, cell)
                    .map_err(|error| {
                        format!(
                            "Could not copy the project source bundles of {library}/{cell}: {error}"
                        )
                    })?,
            );
            copied_sheet_catalogs += candidate_design_management
                .copy_cell_sheet_catalogs(library, cell, new_name, cell)
                .map_err(|error| {
                    format!(
                        "Could not copy the sheet-catalog ownership of {library}/{cell}: {error}"
                    )
                })?
                .copied_sheet_catalogs;
        }
        let pairs = cells
            .iter()
            .map(|cell| ((library, cell.as_str()), (new_name, cell.as_str())))
            .collect::<Vec<_>>();
        let candidate_layouts = self
            .workspace
            .content
            .prepare_copy_physical_layout_cells(&pairs, true)
            .map_err(|error| format!("Could not copy the library's physical layouts: {error}"))?;
        let project_mutation =
            self.preflight_project_library_mutation(ProjectLibraryMutation::CopyLibrary {
                source_library: library.to_owned(),
                target_library: new_name.to_owned(),
            })?;

        // The copy is the project's own: writable, kept in the project, and
        // known by its own name.
        copy.name = new_name.to_owned();
        copy.path = None;
        copy.read_only = false;
        copy.metadata
            .remove(rspice_design::library::LIBRARY_DISPLAY_NAME);
        let project = self.workspace.content.project.id();
        for cell in copy.cells.values_mut() {
            let binding_id = rspice_design::model_bound_symbol::model_bound_binding_id(
                project, new_name, &cell.name,
            );
            rspice_design::model_bound_symbol::restamp_model_bound_identity(
                cell,
                new_name,
                Some(&binding_id),
            );
        }
        self.library_manager.add_library(copy);

        let prefix = format!("{library}/");
        let keys = self
            .workspace
            .content
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&prefix))
            .cloned()
            .collect::<Vec<_>>();
        for key in keys {
            if let Some(mut buffer) = self
                .workspace
                .schematic_editor(&key)
                .map(|original| original.copy_without_validated_revisions())
            {
                buffer.rename_library_bindings(library, new_name);
                let tail = &key[prefix.len()..];
                self.workspace
                    .insert_schematic_editor(format!("{new_name}/{tail}"), buffer);
            }
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);
        if !copied_source_ids.is_empty() {
            self.workspace.content.project_sources = candidate_sources;
            self.workspace.content.project_sources_dirty = true;
        }
        if copied_sheet_catalogs > 0 {
            self.workspace.content.design_management = candidate_design_management;
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_library(new_name);
        Ok(cells.len())
    }

    /// Move a cell to another writable library under `new_name`, and remap
    /// everything that named it, as a rename does: its drawings, open tabs,
    /// every placement in schematics and layouts, its sources, configuration
    /// roots, Design Management ownership and the project top. Returns the
    /// number of instance bindings remapped and what the move left outside
    /// the model.
    pub fn move_cell(
        &mut self,
        library: &str,
        cell: &str,
        target_library: &str,
        new_name: &str,
    ) -> Result<(usize, WorkspacePruned), String> {
        let source = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        if source.read_only {
            return Err(format!("Library '{library}' is read-only"));
        }
        let mut moved = source
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{cell}' not found in library '{library}'"))?
            .clone();
        if crate::state::canonical_cell_view_owner_key(library, "", "")
            == crate::state::canonical_cell_view_owner_key(target_library, "", "")
        {
            return Err(
                "A cell moves to another library; within its own it is renamed.".to_owned(),
            );
        }
        let destination = self
            .library_manager
            .get_library(target_library)
            .ok_or_else(|| format!("Library '{target_library}' not found"))?;
        if destination.read_only {
            return Err(format!("Library '{target_library}' is read-only"));
        }
        CellViewRef::new(target_library, new_name, "view")
            .validate_name_segments()
            .map_err(|error| format!("The cell's name is invalid: {error}"))?;
        let requested = crate::state::canonical_cell_view_owner_key(target_library, new_name, "");
        if destination.cells.values().any(|candidate| {
            crate::state::canonical_cell_view_owner_key(target_library, &candidate.name, "")
                == requested
        }) {
            return Err(format!(
                "Library '{target_library}' already has a cell named '{new_name}', or one whose name differs only in case"
            ));
        }
        let (from, to) = ((library, cell), (target_library, new_name));

        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let moved_source_ids =
            candidate_sources
                .move_cell_view_bundles(from, to)
                .map_err(|error| {
                    format!("Could not move the cell's project source ownership: {error}")
                })?;
        let mut candidate_configurations = self.workspace.content.configuration_sets.clone();
        let moved_configuration_roots = candidate_configurations
            .move_cell_roots(from, to)
            .map_err(|error| {
                format!("Could not move the cell's configuration-set roots: {error}")
            })?;
        let mut candidate_design_management = self.workspace.content.design_management.clone();
        let design_management_receipt = candidate_design_management
            .move_cell_sheet_catalogs(from, to)
            .map_err(|error| {
                format!("Could not move the cell's Design Management ownership: {error}")
            })?;
        let candidate_layouts = self
            .workspace
            .content
            .prepare_rebind_physical_layout_cell_documents(from, to)
            .map_err(|error| format!("Could not move the cell's physical layouts: {error}"))?;
        let project_mutation =
            self.preflight_project_library_mutation(ProjectLibraryMutation::MoveCell {
                source_library: library.to_owned(),
                source_cell: cell.to_owned(),
                target_library: target_library.to_owned(),
                target_cell: new_name.to_owned(),
            })?;

        self.library_manager
            .edit_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the move"))?
            .remove_cell(cell);
        moved.name = new_name.to_owned();
        rspice_design::model_bound_symbol::restamp_model_bound_identity(
            &mut moved,
            target_library,
            None,
        );
        self.library_manager
            .edit_library(target_library)
            .ok_or_else(|| format!("Library '{target_library}' disappeared during the move"))?
            .add_cell(moved);

        let old_prefix = format!("{library}/{cell}/");
        let moved_keys = self
            .workspace
            .content
            .schematic_buffers
            .keys()
            .filter(|key| key.starts_with(&old_prefix))
            .cloned()
            .collect::<Vec<_>>();
        for key in moved_keys {
            if let Some(buffer) = self.workspace.remove_schematic_editor(&key) {
                let tail = &key[old_prefix.len()..];
                self.workspace
                    .insert_schematic_editor(format!("{target_library}/{new_name}/{tail}"), buffer);
            }
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library && reference.cell == cell {
                target_library.clone_into(&mut reference.library);
                new_name.clone_into(&mut reference.cell);
            }
        };
        remap_ref(&mut self.workspace.content.active_view);
        for open in &mut self.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.content.remap_occurrence_masters(remap_ref);

        let remapped =
            self.remap_instance_bindings(|schematic| schematic.rebind_cell_bindings(from, to));

        self.commit_renamed_project_sources(candidate_sources, &moved_source_ids);
        if moved_configuration_roots > 0 {
            self.workspace.content.configuration_sets = candidate_configurations;
            self.workspace.content.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_variant_objects > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.workspace.content.design_management = candidate_design_management;
        }
        let project = &mut self.workspace.content.project;
        if project.root_library == library && project.top_cell == cell {
            target_library.clone_into(&mut project.root_library);
            new_name.clone_into(&mut project.top_cell);
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_cell(target_library, new_name);
        Ok((
            remapped,
            WorkspacePruned {
                void_veriloga_evidence: moved_source_ids,
                ..WorkspacePruned::default()
            },
        ))
    }
}
