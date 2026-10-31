//! Library lifecycle actions.
//!
//! Creating, renaming and deleting a whole library, each as one project
//! transaction, and the checks that refuse a deletion before any state
//! changes.

use super::{LibraryRemoval, Model, WorkspacePruned};
use crate::state::{CellViewRef, ViewType};

impl Model {
    /// Create one empty writable library as a project transaction.
    pub fn create_library(&mut self, name: &str) -> Result<(), String> {
        if let Some(conflict) = conflicting_library_name(&self.library_manager, name, None) {
            return Err(format!(
                "Library '{name}' conflicts with the existing library '{conflict}', whose name differs only in case or normalization"
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
    pub fn rename_library(
        &mut self,
        library: &str,
        new_name: &str,
    ) -> Result<(usize, WorkspacePruned), String> {
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
                "Library '{new_name}' conflicts with the existing library '{conflict}', whose name differs only in case or normalization"
            ));
        }
        let candidate_layouts = self
            .workspace
            .content
            .prepare_rename_physical_layout_library_documents(library, new_name)
            .map_err(|error| format!("Could not rename the library's physical layouts: {error}"))?;

        let mut candidate_configurations = self.workspace.content.configuration_sets.clone();
        let renamed_configuration_roots = candidate_configurations
            .rename_library_roots(library, new_name)
            .map_err(|error| {
                format!("Could not move the library's configuration-set roots: {error}")
            })?;
        let mut candidate_sources = self.workspace.content.project_sources.clone();
        let renamed_source_ids = candidate_sources
            .rename_library_bundles(library, new_name)
            .map_err(|error| {
                format!("Could not move the library's project source ownership: {error}")
            })?;
        let mut candidate_design_management = self.workspace.content.design_management.clone();
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
                    .insert_schematic_editor(format!("{new_name}/{tail}"), buffer);
            }
        }
        self.workspace
            .content
            .commit_prepared_physical_layout_catalog(candidate_layouts);

        let remap_ref = |reference: &mut CellViewRef| {
            if reference.library == library {
                reference.library = new_name.to_owned();
            }
        };
        remap_ref(&mut self.workspace.content.active_view);
        for open in &mut self.workspace.content.open_views {
            remap_ref(&mut open.reference);
        }
        self.workspace.content.remap_occurrence_masters(&remap_ref);

        let remapped = self.remap_instance_bindings(|schematic| {
            schematic.rename_library_bindings(library, new_name)
        });

        self.commit_renamed_project_sources(candidate_sources, &renamed_source_ids);
        if renamed_configuration_roots > 0 {
            self.workspace.content.configuration_sets = candidate_configurations;
            self.workspace.content.project_metadata_dirty = true;
        }
        if design_management_receipt.affected_sheet_catalogs > 0
            || design_management_receipt.remapped_variant_objects > 0
            || design_management_receipt.remapped_annotation_objects > 0
        {
            self.workspace.content.design_management = candidate_design_management;
        }
        if self.workspace.content.project.root_library == library {
            self.workspace.content.project.root_library = new_name.to_owned();
        }
        self.publish_project_library_mutation(project_mutation);

        self.library_manager.select_library(new_name);
        Ok((
            remapped,
            WorkspacePruned {
                void_veriloga_evidence: renamed_source_ids,
                ..WorkspacePruned::default()
            },
        ))
    }

    /// Remove one library and everything it owns. Every reason the removal is
    /// refused is reported on its own, before any state changes, because a
    /// partially deleted library is unrecoverable. Returns the cell count,
    /// with the revision the caller publishes once it has carried out what
    /// the removal left outside the model.
    pub fn delete_library(&mut self, library: &str) -> Result<LibraryRemoval, String> {
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

        let mut candidate_design_management = self.workspace.content.design_management.clone();
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
            self.workspace.content.design_management = candidate_design_management;
        }
        let owned_layouts = self
            .workspace
            .content
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library == library)
            .map(|document| document.owner().clone())
            .collect::<Vec<_>>();
        for owner in &owned_layouts {
            self.workspace
                .content
                .remove_physical_layout_document(owner);
        }
        let pruned = self.prune_workspace_after_library_deleted(library);

        Ok(LibraryRemoval {
            cells: cells.len(),
            pruned,
            mutation: project_mutation,
        })
    }

    /// The first reason this library cannot be deleted, or `None`. The
    /// deletion review asks exactly this question, so it can never promise a
    /// deletion the transaction then refuses.
    pub fn library_deletion_blocker(&self, library: &str) -> Option<String> {
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
        if self.workspace.content.project.root_library == library {
            return Err(format!(
                "Library '{library}' holds the project root cell '{}'. Repoint the project root before deleting it.",
                self.workspace.content.project.top_cell
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
            .content
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

    fn prune_workspace_after_library_deleted(&mut self, library: &str) -> WorkspacePruned {
        let prompt = self.sync_active_schematic_to_workspace();
        let active_removed = self.workspace.content.active_view.library == library;
        let prefix = format!("{library}/");
        self.workspace
            .content
            .schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .session
            .schematic_sessions
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .content
            .open_views
            .retain(|open| open.reference.library != library);
        let hierarchy_pruned = self
            .workspace
            .content
            .retain_valid_occurrences(|reference| reference.library != library);
        WorkspacePruned {
            prompt,
            ..self.restore_valid_workspace_focus_after_prune(
                active_removed,
                hierarchy_pruned,
                false,
                true,
            )
        }
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
    model: &Model,
    scope: &str,
    matches: impl Fn(&CellViewRef) -> bool,
) -> Result<(), String> {
    let names = model
        .workspace
        .content
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
fn require_no_owned_sources(model: &Model, library: &str) -> Result<(), String> {
    let bundles = model
        .workspace
        .content
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
    let veriloga_views = model
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
