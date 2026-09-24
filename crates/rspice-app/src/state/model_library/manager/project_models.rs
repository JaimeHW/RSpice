//! Application entry points for portable project-model catalog transactions.

use super::*;
use rspice_model_library::ProjectModelTarget;

impl ModelLibraryManager {
    pub fn create_editable_project_copy(
        &mut self,
        source_library_name: &str,
        source_model_name: &str,
        project_library_name: &str,
    ) -> Result<ProjectModelCommit, String> {
        self.catalog.create_editable_project_copy(
            source_library_name,
            source_model_name,
            project_library_name,
        )
    }

    pub fn create_project_model_revision(
        &mut self,
        library_name: &str,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        self.catalog
            .create_project_model_revision(library_name, definition, qualification)
    }

    pub fn replace_project_model_revision_in_library(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_name: &str,
        expected_model_digest: ContentDigest,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        self.catalog.replace_project_model_revision_in_library(
            ProjectModelTarget {
                library_name,
                source_id: expected_source_id,
                library_revision: expected_library_revision,
                model_revision: expected_model_revision,
                model_name: expected_model_name,
                model_digest: expected_model_digest,
            },
            definition,
            qualification,
        )
    }

    pub fn replace_project_model_qualification(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        self.catalog.replace_project_model_qualification(
            ProjectModelTarget {
                library_name,
                source_id: expected_source_id,
                library_revision: expected_library_revision,
                model_revision: expected_model_revision,
                model_name,
                model_digest: expected_model_digest,
            },
            qualification,
        )
    }

    pub fn replace_project_model_correlation(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        correlation: &ModelCorrelationState,
    ) -> Result<ProjectModelCommit, String> {
        self.catalog.replace_project_model_correlation(
            ProjectModelTarget {
                library_name,
                source_id: expected_source_id,
                library_revision: expected_library_revision,
                model_revision: expected_model_revision,
                model_name,
                model_digest: expected_model_digest,
            },
            correlation,
        )
    }
}

#[cfg(test)]
impl ModelLibraryManager {
    /// Seed a sectionless test model through the production revision transaction.
    pub(crate) fn create_project_model(
        &mut self,
        library_name: &str,
        definition: &ProjectModelDefinition,
    ) -> Result<ProjectModelCommit, String> {
        let metadata = definition.reconcile_metadata(None)?;
        self.create_project_model_revision(
            library_name,
            &ProjectModelRevisionDefinition::new(definition.clone(), metadata),
            &ModelQualificationState::default(),
        )
    }
}
