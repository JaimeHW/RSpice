//! The open project's authoritative physical-layout documents: the one
//! way a Layout view gains one, and the one way an edit reaches it.

use super::Model;

impl Model {
    /// Initialize authoritative physical content for an existing Layout view.
    /// This is the shared entry point future editors and stream importers must
    /// use; it refuses catalog labels that do not resolve to an exact editable
    /// Layout view and never invents a technology or database unit.
    pub fn initialize_physical_layout_document(
        &mut self,
        owner: crate::state::CellViewRef,
    ) -> Result<crate::product::ObjectRevision, String> {
        if self.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be created".to_owned(),
            );
        }
        owner
            .validate_name_segments()
            .map_err(|error| format!("Layout owner is invalid: {error}"))?;
        let library = self
            .library_manager
            .get_library(&owner.library)
            .ok_or_else(|| format!("Layout owner library '{}' does not exist", owner.library))?;
        if library.read_only {
            return Err(format!(
                "Layout owner library '{}' is read-only",
                owner.library
            ));
        }
        let view = library
            .get_cell(&owner.cell)
            .and_then(|cell| cell.get_view(&owner.view))
            .ok_or_else(|| format!("Layout owner '{}' does not exist", owner.display_path()))?;
        if view.view_type != crate::state::ViewType::Layout {
            return Err(format!(
                "Layout owner '{}' is a {} view, not a Layout view",
                owner.display_path(),
                view.view_type.display_name()
            ));
        }
        if self
            .workspace
            .content
            .physical_layout_document(&owner)
            .is_some()
        {
            return Err(format!(
                "Layout owner '{}' already has an authoritative physical document",
                owner.display_path()
            ));
        }
        let technology = self.exact_project_layout_technology_binding()?;
        let document = crate::state::PhysicalLayoutDocument::try_new(owner.clone(), technology)
            .map_err(|error| format!("Physical layout could not be initialized: {error}"))?;
        let revision = document.revision();
        self.workspace
            .content
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical layout was not committed: {error}"))?;
        if let Some(mut library) = self.library_manager.edit_library(&owner.library) {
            library.mark_view_modified(&owner.cell, &owner.view);
        }
        if let Some(open) = self
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == owner)
        {
            open.dirty = true;
        }
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.clear_project_design_history();
        Ok(revision)
    }

    /// Apply one atomic, revision-checked edit batch to an authoritative
    /// physical-layout cellview and publish no partial state on failure.
    pub fn apply_physical_layout_transaction(
        &mut self,
        owner: &crate::state::CellViewRef,
        expected_revision: crate::product::ObjectRevision,
        edits: &[crate::state::LayoutEdit],
    ) -> Result<crate::product::ObjectRevision, String> {
        if self.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only; physical layout cannot be edited".to_owned(),
            );
        }
        let library = self
            .library_manager
            .get_library(&owner.library)
            .ok_or_else(|| format!("Layout owner library '{}' does not exist", owner.library))?;
        if library.read_only {
            return Err(format!(
                "Layout owner library '{}' is read-only",
                owner.library
            ));
        }
        let view = library
            .get_cell(&owner.cell)
            .and_then(|cell| cell.get_view(&owner.view))
            .ok_or_else(|| format!("Layout owner '{}' does not exist", owner.display_path()))?;
        if view.view_type != crate::state::ViewType::Layout {
            return Err(format!(
                "Layout owner '{}' is a {} view, not a Layout view",
                owner.display_path(),
                view.view_type.display_name()
            ));
        }
        let exact_technology = self.exact_project_layout_technology_binding()?;
        let allowed_layer_purposes = self
            .exact_project_pdk_package()?
            .manifest()
            .layers
            .iter()
            .flat_map(|layer| {
                layer.purposes.iter().map(move |purpose| {
                    (
                        layer.name.to_ascii_lowercase(),
                        purpose.to_ascii_lowercase(),
                    )
                })
            })
            .collect::<std::collections::BTreeSet<_>>();
        let mut document = self
            .workspace
            .content
            .physical_layout_document(owner)
            .ok_or_else(|| {
                format!(
                    "Layout owner '{}' has no authoritative physical document",
                    owner.display_path()
                )
            })?
            .clone();
        if document.technology() != &exact_technology {
            return Err(format!(
                "Layout owner '{}' is bound to a different signed technology than the project",
                owner.display_path()
            ));
        }
        let revision = document
            .apply_transaction(expected_revision, edits)
            .map_err(|error| format!("Physical-layout transaction failed: {error}"))?;
        for (kind, id, layer, purpose) in document
            .shapes()
            .iter()
            .map(|(id, shape)| {
                (
                    "shape",
                    id.to_string(),
                    shape.layer_purpose.layer.as_str(),
                    shape.layer_purpose.purpose.as_str(),
                )
            })
            .chain(document.texts().iter().map(|(id, text)| {
                (
                    "text",
                    id.to_string(),
                    text.layer_purpose.layer.as_str(),
                    text.layer_purpose.purpose.as_str(),
                )
            }))
        {
            if !allowed_layer_purposes
                .contains(&(layer.to_ascii_lowercase(), purpose.to_ascii_lowercase()))
            {
                return Err(format!(
                    "Physical-layout edit failed: {kind} {id} uses layer/purpose '{layer}/{purpose}', which the project's signed PDK does not define"
                ));
            }
        }
        self.workspace
            .content
            .commit_physical_layout_document(document)
            .map_err(|error| format!("Physical-layout transaction was not committed: {error}"))?;
        if let Some(mut library) = self.library_manager.edit_library(&owner.library) {
            library.mark_view_modified(&owner.cell, &owner.view);
        }
        if let Some(open) = self
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == *owner)
        {
            open.dirty = true;
        }
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.clear_project_design_history();
        Ok(revision)
    }
}
