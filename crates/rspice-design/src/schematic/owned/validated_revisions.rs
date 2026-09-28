//! Validated revision records and guarded journal rollback belong to the design.
use super::super::{document::SchematicDocument, validated_revision::*};
use super::Schematic;
use rspice_app_types::product::ContentDigest;
impl Schematic {
    fn validated_revision_source(&self) -> ValidatedRevisionSource<'_> {
        ValidatedRevisionSource {
            grid_size: self.document.grid_size,
            document_policy: self.document.document_policy,
            components: &self.document.components,
            wires: &self.document.wires,
            buses: &self.document.buses,
            bus_taps: &self.document.bus_taps,
            junctions: &self.document.junctions,
            net_labels: &self.document.net_labels,
            design_notes: &self.document.design_notes,
            documentation_shapes: &self.document.documentation_shapes,
            connections: &self.document.connections,
        }
    }
    pub fn validated_design_content_digest(&self) -> Result<ContentDigest, ValidatedRevisionError> {
        self.validated_revision_source().design_content_digest()
    }
    pub fn seed_accepted_revision_baseline(
        &mut self,
        accepted: &Schematic,
        project_id: &str,
        project_revision: u64,
        view_identity: &str,
        clock: impl FnOnce() -> Result<u64, &'static str>,
    ) -> Result<Option<ValidatedSchematicRevisionId>, ValidatedRevisionError> {
        self.document
            .validated_revisions
            .seed_accepted_revision_baseline(
                accepted.validated_revision_source(),
                project_id,
                project_revision,
                view_identity,
                clock,
            )
    }
    pub fn append_validated_revision(
        &mut self,
        request: ValidatedRevisionRequest,
        clock: impl FnOnce() -> Result<u64, &'static str>,
    ) -> Result<ValidatedSchematicRevisionId, ValidatedRevisionError> {
        let SchematicDocument {
            grid_size,
            document_policy,
            components,
            wires,
            buses,
            bus_taps,
            junctions,
            net_labels,
            design_notes,
            documentation_shapes,
            connections,
            validated_revisions,
            ..
        } = &mut self.document;
        let source = ValidatedRevisionSource {
            grid_size: *grid_size,
            document_policy: *document_policy,
            components,
            wires,
            buses,
            bus_taps,
            junctions,
            net_labels,
            design_notes,
            documentation_shapes,
            connections,
        };
        let id = validated_revisions.append_validated_revision(request, source, clock)?;
        Ok(id)
    }
    pub fn remove_unpublished_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        self.document
            .validated_revisions
            .remove_unpublished_tail(id)
    }
    pub fn copy_without_validated_revisions(&self) -> Self {
        let mut copy = self.clone();
        copy.document.validated_revisions = ValidatedRevisionJournal::default();
        copy
    }
    pub fn record_validated_save_revision(
        &mut self,
        request: ValidatedRevisionRequest,
        original_journal: ValidatedRevisionJournal,
        clock: impl FnOnce() -> Result<u64, &'static str>,
    ) -> Result<
        (
            ValidatedSchematicRevisionId,
            ValidatedRevisionJournal,
            ValidatedRevisionJournal,
            ContentDigest,
        ),
        String,
    > {
        let revision_id = match self.append_validated_revision(request, clock) {
            Ok(id) => id,
            Err(error) => {
                self.document.validated_revisions = original_journal;
                return Err(format!("Validated revision could not be recorded: {error}"));
            }
        };
        let expected_journal = self.document.validated_revisions.clone();
        let expected_design_digest = match self.validated_design_content_digest() {
            Ok(digest) => digest,
            Err(error) => {
                self.document.validated_revisions = original_journal;
                return Err(format!(
                    "The guarded working-design digest could not be recorded: {error}"
                ));
            }
        };
        Ok((
            revision_id,
            original_journal,
            expected_journal,
            expected_design_digest,
        ))
    }
    pub fn rollback_validated_save_journal(
        &mut self,
        original_journal: &ValidatedRevisionJournal,
        expected_journal: &ValidatedRevisionJournal,
        expected_design_digest: ContentDigest,
    ) -> Result<bool, String> {
        if &self.document.validated_revisions != expected_journal {
            return Err(
                "its validated revision history changed after publication began".to_owned(),
            );
        }
        let current_design_digest = self
            .validated_design_content_digest()
            .map_err(|error| format!("its working-design guard could not be verified: {error}"))?;
        self.document.validated_revisions = original_journal.clone();
        Ok(current_design_digest != expected_design_digest)
    }
}
