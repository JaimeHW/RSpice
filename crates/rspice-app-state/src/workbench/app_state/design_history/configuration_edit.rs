//! The project's configuration changes as steps of the one Undo.
//!
//! Every committed change to the configuration catalog, an apply, a new
//! configuration, a rename, a delete or a change of what runs use, is one
//! step holding the catalog as it was and as it became. Undo puts the catalog
//! back and Redo forward, each as a project transaction of its own, so the
//! project revision only moves forward. A step stands on the catalog its
//! last move left: a change to the configurations made since refuses it and
//! says so, rather than overwriting that change. It pins the catalog rather
//! than the project revision, which the step undone before it has moved on:
//! what a step overwrites is all it guards.

use super::*;
use crate::state::ConfigurationSetCatalog;

/// One committed configuration change.
#[derive(Debug, Clone)]
pub struct ConfigurationEditRecord {
    description: String,
    before: ConfigurationSetCatalog,
    after: ConfigurationSetCatalog,
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
        self.workspace
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
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        *state.workspace.content.configuration_sets == self.before
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
                "The configuration change cannot be undone because the configurations changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        replace(state, &self.before)
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "The configuration change cannot be redone because the configurations changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        replace(state, &self.after)
    }
}

fn replace(state: &mut Model, catalog: &ConfigurationSetCatalog) -> Result<(), String> {
    state
        .workspace
        .content
        .replace_configuration_sets(catalog.clone())
        .map_err(|error| error.to_string())?;
    state.netlist_artifacts.current_generation_input_digest = None;
    Ok(())
}
