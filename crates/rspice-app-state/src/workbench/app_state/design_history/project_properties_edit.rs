//! The project's properties as steps of the one Undo.
//!
//! Every committed change to what the user owns of the project's
//! descriptor, its name, description, top cell or include search chain, is
//! one step holding the properties as they were and as they became. Undo
//! puts them back and Redo forward, each as a project transaction of its
//! own, so the project revision only moves forward. A step stands on the
//! properties its last move left: a change to them made since refuses it
//! and says so, rather than overwriting that change, as does a top cell to
//! go back to that the project can no longer run from. It pins the
//! properties rather than the project revision, which the step undone
//! before it has moved on: what a step overwrites is all it guards.

use super::*;
use crate::state::workspace::ProjectProperties;

/// One committed change to the project's properties.
#[derive(Debug, Clone)]
pub struct ProjectPropertiesEditRecord {
    description: String,
    before: ProjectProperties,
    after: ProjectProperties,
}

impl Model {
    /// Replace the project's properties with `candidate` as one project
    /// transaction, and record it as one step of Undo described by
    /// `description`. A new top cell must be one the project can run from:
    /// see [`Model::project_top_refusal`].
    pub fn publish_project_properties(
        &mut self,
        candidate: ProjectProperties,
        description: impl Into<String>,
    ) -> Result<(), String> {
        if self.safe_mode.project_read_only() {
            return Err(
                "The project is open read-only, so its properties cannot be changed.".to_owned(),
            );
        }
        let before = self.workspace.content.project.properties();
        if candidate == before {
            return Err("The project properties are unchanged.".to_owned());
        }
        if (&candidate.root_library, &candidate.top_cell)
            != (&before.root_library, &before.top_cell)
            && let Some(refusal) =
                self.project_top_refusal(&candidate.root_library, &candidate.top_cell)
        {
            return Err(refusal);
        }
        replace(self, &before, &candidate)?;
        self.push_project_record(
            RecordHeader::project_only(),
            ProjectDesignBody::ProjectPropertiesEdit(Box::new(ProjectPropertiesEditRecord {
                description: description.into(),
                before,
                after: candidate,
            })),
        );
        Ok(())
    }
}

impl ProjectPropertiesEditRecord {
    pub(super) fn description(&self) -> &str {
        &self.description
    }

    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        state.workspace.content.project.has_properties(&self.after)
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        state.workspace.content.project.has_properties(&self.before)
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "Project properties cannot be {operation} without an open project."
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "Project properties cannot be {operation} while the project is read-only."
            ));
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "The project properties change cannot be undone because they changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        runnable_top(state, &self.after, &self.before, "undone")?;
        replace(state, &self.after, &self.before)
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "The project properties change cannot be redone because they changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        runnable_top(state, &self.before, &self.after, "redone")?;
        replace(state, &self.before, &self.after)
    }
}

/// Refuse a step that would move the top to a cell the project can no
/// longer run from: one deleted, or whose schematic went, since.
fn runnable_top(
    state: &Model,
    current: &ProjectProperties,
    properties: &ProjectProperties,
    operation: &str,
) -> Result<(), String> {
    if (&current.root_library, &current.top_cell)
        == (&properties.root_library, &properties.top_cell)
    {
        return Ok(());
    }
    state
        .project_top_refusal(&properties.root_library, &properties.top_cell)
        .map_or(Ok(()), |refusal| {
            Err(format!(
                "The project properties change cannot be {operation}: {refusal}"
            ))
        })
}

/// Write `properties` over `current` as one project transaction. A new top
/// cell or include chain changes what a run netlists, so the generated
/// netlist's input identity no longer describes it; a new chain also changes
/// which file every relative include resolves to, so the deck's diagnostics
/// are no longer evidence about it.
fn replace(
    state: &mut Model,
    current: &ProjectProperties,
    properties: &ProjectProperties,
) -> Result<(), String> {
    state
        .workspace
        .content
        .project
        .replace_properties(properties.clone())
        .map_err(|error| sentence(&error.to_string()))?;
    state.workspace.content.project_metadata_dirty = true;
    let top_moved = (&current.root_library, &current.top_cell)
        != (&properties.root_library, &properties.top_cell);
    let chain_moved = current.include_search_paths != properties.include_search_paths;
    if top_moved || chain_moved {
        state.netlist_artifacts.current_generation_input_digest = None;
    }
    if chain_moved {
        state.invalidate_source_evidence();
    }
    Ok(())
}

/// A refusal of the descriptor's, as the status bar states it.
fn sentence(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        format!("{}{}.", first.to_uppercase(), characters.as_str())
    })
}
