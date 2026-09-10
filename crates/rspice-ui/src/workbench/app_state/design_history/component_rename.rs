//! One publication boundary for a component name and the live references
//! carried with it. History owns exact affected content, never retained runs.

use super::reference_preparation::{reference_from_key, validate_reference_document};
use super::references::PreparedReferences;
use super::*;
use crate::state::{AnnotationState, Component, SchematicObjectKey};
use crate::workbench::state::InlineEditAuthority;

#[derive(Debug, Clone)]
pub(super) struct ComponentRenameRecord {
    pub(super) description: String,
    document: CellViewRef,
    before: BTreeMap<String, SchematicSnapshot>,
    after: BTreeMap<String, SchematicSnapshot>,
    references: ReferenceChanges,
    annotation: Option<AnnotationChange>,
}

#[derive(Debug, Clone)]
struct AnnotationChange {
    before: AnnotationState,
    after: AnnotationState,
    before_revision: ObjectRevision,
    after_revision: ObjectRevision,
}

pub(super) struct PreparedComponentRename {
    record: ComponentRenameRecord,
    references: PreparedReferences,
    annotation: Option<DesignManagementCatalog>,
}

impl AppState {
    pub(crate) fn inline_edit_authority(&self) -> InlineEditAuthority {
        InlineEditAuthority {
            project: self.workspace.project.id(),
            document: self.workspace.active_schematic_reference(),
            occurrence: self.workspace.active_occurrence().cloned(),
            design_epoch: self.design_execution_epoch,
            document_epoch: self.active_schematic_epoch,
        }
    }

    /// Used by focus changes, document navigation, and commands before they
    /// read or replace the design. A failed commit retains the complete draft.
    pub(crate) fn commit_inline_component_edit(&mut self) -> Result<bool, String> {
        let Some(session) = self.workbench.inline_edit.session().cloned() else {
            return Ok(false);
        };
        if session.candidate.as_ref() == Some(&session.expected) {
            self.workbench.inline_edit.end();
            return Ok(false);
        }
        let result = if session.authority != self.inline_edit_authority() {
            Err("The document or hierarchy occurrence changed. Cancel this inspector draft and edit the current component.".to_owned())
        } else if let Some(candidate) = session.candidate {
            self.edit_component_transaction(&session.expected, candidate, &session.description)
        } else {
            Err(session
                .error
                .unwrap_or_else(|| "Correct the inspector field before applying it.".to_owned()))
        };
        match &result {
            Ok(_) => self.workbench.inline_edit.end(),
            Err(error) => self.workbench.inline_edit.set_error(Some(error.clone())),
        }
        result
    }

    /// A command/navigation boundary reports a refusal once and leaves the
    /// inspector's draft available for correction or explicit cancellation.
    pub(crate) fn commit_pending_inspector_edit(&mut self) -> bool {
        match self.commit_inline_component_edit() {
            Ok(_) => true,
            Err(error) => {
                self.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                    "Inspector edit was not applied: {error}"
                )));
                false
            }
        }
    }

    pub(crate) fn rename_component_transaction(
        &mut self,
        expected: &Component,
        name: String,
    ) -> Result<bool, String> {
        let mut candidate = expected.clone();
        candidate.name = name;
        self.edit_component_transaction(expected, candidate, "rename component and references")
    }

    /// Commit a complete property candidate, carrying any renamed references
    /// in the same history entry. Value-only edits keep document-local history.
    pub(crate) fn edit_component_transaction(
        &mut self,
        expected: &Component,
        candidate: Component,
        description: &str,
    ) -> Result<bool, String> {
        let document = self.workspace.active_schematic_reference();
        let components = self
            .schematic
            .prepare_component_edit(expected, candidate.clone())?;
        if self.schematic.components == components {
            return Ok(false);
        }
        if !self.project_lifecycle.project_open || document_read_only(self, &document) {
            return Err(
                "Component editing requires an open, writable project and document.".to_owned(),
            );
        }
        if self.schematic.has_pending_operation() {
            return Err(
                "Finish or cancel the active schematic gesture before editing properties."
                    .to_owned(),
            );
        }
        if candidate.name == expected.name || expected.kind.spice_prefix().is_empty() {
            let before = SchematicSnapshot::capture(&self.schematic);
            self.schematic.components = components;
            self.schematic.is_dirty = true;
            self.schematic.bump_topology_version();
            self.schematic.commit_undo_from(before, description);
            return Ok(true);
        }
        let mut after_schematic = self.schematic.clone();
        after_schematic.components = components;
        after_schematic.is_dirty = true;
        after_schematic.bump_topology_version();
        let transaction = self.prepare_schematic_reference_transaction(
            BTreeMap::from([(document.key(), self.schematic.clone())]),
            BTreeMap::from([(document.key(), after_schematic)]),
        )?;
        let annotation_before = self.workspace.design_management.annotation();
        let mut annotation_after = annotation_before.clone();
        let annotation = annotation_after
            .commit_manual_reference_edit(
                SchematicObjectKey::new(&document.key(), expected.id)
                    .map_err(|error| error.to_string())?,
                &expected.name,
                &candidate.name,
            )
            .map_err(|error| error.to_string())?
            .map(|_| -> Result<_, String> {
                Ok(AnnotationChange {
                    before: annotation_before.clone(),
                    after: annotation_after,
                    before_revision: self.workspace.project.revision(),
                    after_revision: self
                        .workspace
                        .project
                        .revision()
                        .next()
                        .map_err(|error| error.to_string())?,
                })
            })
            .transpose()?;
        let record = ComponentRenameRecord {
            description: description.to_owned(),
            document: document.clone(),
            before: capture_schematic_map(transaction.before),
            after: capture_schematic_map(transaction.after),
            references: transaction.references,
            annotation,
        };
        let documents = record
            .after
            .keys()
            .map(|key| reference_from_key(key).map(DocumentCompensation::naming))
            .collect::<Result<Vec<_>, _>>()?;
        let prepared = PreparedComponentRename {
            references: transaction.prepared_references,
            annotation: record.prepare_annotation(self, true)?,
            record,
        };
        let record = prepared.publish(self, true)?;
        for key in record.after.keys() {
            if key.eq_ignore_ascii_case(&document.key()) {
                self.schematic.undo_history.clear_redo();
            } else {
                self.workspace
                    .schematic_buffers
                    .get_mut(key)
                    .expect("guarded reference document")
                    .undo_history
                    .clear_redo();
            }
        }
        self.workspace.save_active_schematic(&self.schematic);
        self.push_project_record(
            RecordHeader::committed(documents, Some(document.clone()), Some(document)),
            ProjectDesignBody::ComponentRename(Box::new(record)),
        );
        Ok(true)
    }
}

impl ComponentRenameRecord {
    pub(super) fn after_design_matches(&self, state: &AppState) -> bool {
        self.matches(state, false)
    }
    pub(super) fn before_design_matches(&self, state: &AppState) -> bool {
        self.matches(state, true)
    }

    fn matches(&self, state: &AppState, forward: bool) -> bool {
        let expected = if forward { &self.before } else { &self.after };
        schematic_map_matches(state, expected)
            && self.references.matches(state, forward)
            && self.annotation.as_ref().is_none_or(|change| {
                state.workspace.design_management.annotation()
                    == if forward {
                        &change.before
                    } else {
                        &change.after
                    }
                    && state.workspace.project.revision()
                        == if forward {
                            change.before_revision
                        } else {
                            change.after_revision
                        }
            })
    }

    /// Availability checks inspect retained owners; command execution separately
    /// prepares the complete live dependency closure before navigation.
    fn validate_authority(
        &self,
        state: &AppState,
        forward: bool,
    ) -> Result<Option<DesignManagementCatalog>, String> {
        if !state.project_lifecycle.project_open || document_read_only(state, &self.document) {
            return Err(
                "Component rename requires an open, writable project and document.".to_owned(),
            );
        }
        if !self.matches(state, forward) {
            return Err(
                "Component rename cannot be applied because its document or references changed."
                    .to_owned(),
            );
        }
        for key in self.before.keys() {
            let reference = reference_from_key(key)?;
            let source = schematic_for_reference(state, &reference)
                .ok_or_else(|| format!("Reference document '{key}' is unavailable."))?;
            validate_reference_document(state, key, source)?;
        }
        self.references.prepare(state, forward)?;
        self.prepare_annotation(state, forward)
    }

    pub(super) fn validate_mutation(
        &self,
        state: &AppState,
        operation: &str,
    ) -> Result<(), String> {
        self.validate_authority(state, operation != "undone")
            .map(|_| ())
    }

    pub(super) fn prepare(
        &self,
        state: &AppState,
        forward: bool,
    ) -> Result<PreparedComponentRename, String> {
        let annotation = self.validate_authority(state, forward)?;
        let history = state.prepare_reference_history(&self.before, &self.after, forward)?;
        let record = Self {
            description: self.description.clone(),
            document: self.document.clone(),
            before: history.before,
            after: history.after,
            references: history.changes,
            annotation: self.annotation.clone(),
        };
        Ok(PreparedComponentRename {
            annotation,
            references: history.references,
            record,
        })
    }

    pub(super) fn reference_schematics(&self) -> &BTreeMap<String, SchematicSnapshot> {
        &self.after
    }

    fn prepare_annotation(
        &self,
        state: &AppState,
        forward: bool,
    ) -> Result<Option<DesignManagementCatalog>, String> {
        self.annotation
            .as_ref()
            .map(|change| {
                let current = &state.workspace.design_management;
                let mut candidate = current.clone();
                *candidate.annotation_mut() = if forward {
                    &change.after
                } else {
                    &change.before
                }
                .clone();
                // Exercise the same catalog and revision validation as publication
                // before history navigation or any schematic/reference changes.
                let mut prepared = current.clone();
                prepared
                    .publish_reviewed_candidate(current.revision(), candidate)
                    .map_err(|error| error.to_string())?;
                state
                    .workspace
                    .project
                    .revision()
                    .next()
                    .map_err(|error| error.to_string())?;
                Ok::<_, String>(prepared)
            })
            .transpose()
    }

    pub(super) fn reanchor_annotation_revision(
        &mut self,
        previous: ObjectRevision,
        replacement: ObjectRevision,
    ) {
        if let Some(change) = &mut self.annotation {
            for revision in [&mut change.before_revision, &mut change.after_revision] {
                if *revision == previous {
                    *revision = replacement;
                }
            }
        }
    }
}

impl PreparedComponentRename {
    pub(super) fn publish(
        mut self,
        state: &mut AppState,
        forward: bool,
    ) -> Result<ComponentRenameRecord, String> {
        if let Some(candidate) = self.annotation {
            state
                .workspace
                .replace_design_management(candidate)
                .map_err(|error| error.to_string())?;
        }
        let record = &mut self.record;
        apply_schematic_map(
            state,
            if forward {
                &record.after
            } else {
                &record.before
            },
            true,
        )?;
        self.references.publish(state);
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        state.ui.netlist.current_generation_input_digest = None;
        if let Some(change) = &mut record.annotation {
            let restored = if forward {
                &mut change.after_revision
            } else {
                &mut change.before_revision
            };
            let revision = state.workspace.project.revision();
            state.reanchor_annotation_history_revision(*restored, revision);
            *restored = revision;
        }
        Ok(self.record)
    }
}
