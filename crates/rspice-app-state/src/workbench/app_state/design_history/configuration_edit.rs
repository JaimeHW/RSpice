//! The project's configuration changes as steps of the one Undo.
//!
//! Every committed change to the configuration catalog, an apply, a new
//! configuration, a rename, a delete or a change of what runs use, is one
//! step holding the catalog as it was and as it became. Undo puts the catalog
//! back and Redo forward, each as a project transaction of its own, so the
//! project revision only moves forward. A step stands on the revision its
//! last move left: a project change made since refuses it and says so, rather
//! than overwriting that change.

use super::*;
use crate::state::ConfigurationSetCatalog;

/// One committed configuration change.
#[derive(Debug, Clone)]
pub struct ConfigurationEditRecord {
    description: String,
    before: ConfigurationSetCatalog,
    after: ConfigurationSetCatalog,
    undo_guard_revision: ObjectRevision,
    redo_guard_revision: Option<ObjectRevision>,
}

impl Model {
    /// Replace the project's configuration catalog with `candidate` as one
    /// project transaction, and record it as one step of Undo described by
    /// `description`.
    pub fn publish_configuration_sets(
        &mut self,
        candidate: ConfigurationSetCatalog,
        description: impl Into<String>,
    ) -> Result<(), String> {
        let before = (*self.workspace.content.configuration_sets).clone();
        let undo_guard_revision = self
            .workspace
            .content
            .replace_configuration_sets(candidate)
            .map_err(|error| error.to_string())?;
        let after = (*self.workspace.content.configuration_sets).clone();
        self.netlist_artifacts.current_generation_input_digest = None;
        self.push_project_record(
            RecordHeader::project_only(),
            ProjectDesignBody::ConfigurationEdit(Box::new(ConfigurationEditRecord {
                description: description.into(),
                before,
                after,
                undo_guard_revision,
                redo_guard_revision: None,
            })),
        );
        Ok(())
    }
}

impl ConfigurationEditRecord {
    pub(super) fn description(&self) -> &str {
        &self.description
    }

    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        *state.workspace.content.configuration_sets == self.after
            && state.workspace.content.project.revision() == self.undo_guard_revision
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        *state.workspace.content.configuration_sets == self.before
            && self
                .redo_guard_revision
                .is_some_and(|revision| state.workspace.content.project.revision() == revision)
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "Configurations cannot be {operation} without an open project."
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "Configurations cannot be {operation} while the project is read-only."
            ));
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "The configuration change cannot be undone because the project changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        self.redo_guard_revision = Some(replace(state, &self.before)?);
        Ok(())
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "The configuration change cannot be redone because the project changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        self.undo_guard_revision = replace(state, &self.after)?;
        Ok(())
    }
}

fn replace(state: &mut Model, catalog: &ConfigurationSetCatalog) -> Result<ObjectRevision, String> {
    let revision = state
        .workspace
        .content
        .replace_configuration_sets(catalog.clone())
        .map_err(|error| error.to_string())?;
    state.netlist_artifacts.current_generation_input_digest = None;
    Ok(revision)
}
