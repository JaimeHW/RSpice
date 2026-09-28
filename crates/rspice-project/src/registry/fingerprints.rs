//! Canonical document fingerprint collection for save and dirty-state comparison.

use super::{
    DocumentFingerprints, ProjectDocumentId, ResultFingerprintCache, SchematicDocumentContent,
    ViewDocumentContent, digest, project_configuration_value, reference_from_key,
    result_fingerprint,
};
use crate::ProjectFile;
use rspice_app_types::product::ContentDigest;
use rspice_design::project_sources::ProjectSourceOwner;
use rspice_design_model::cell_view::CellViewRef;
use std::collections::{HashMap, HashSet};

#[cfg(any(test, feature = "document-fingerprint-observation"))]
thread_local! {
    /// Full document fingerprint passes, including retained sample scans.
    pub static FINGERPRINT_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub fn document_fingerprints(project: &ProjectFile) -> Result<DocumentFingerprints, String> {
    Ok(DocumentFingerprints::from_documents(document_digests(
        project,
    )?))
}

pub fn document_fingerprints_with_results_cache(
    project: &ProjectFile,
    cache: &ResultFingerprintCache,
) -> Result<DocumentFingerprints, String> {
    Ok(DocumentFingerprints::from_documents(
        document_digests_with_results_cache(project, Some(cache))?,
    ))
}

pub fn content_digest(project: &ProjectFile) -> Result<ContentDigest, String> {
    document_fingerprints(project).map(|fingerprints| fingerprints.content_digest())
}

pub fn document_digests(
    project: &ProjectFile,
) -> Result<HashMap<ProjectDocumentId, ContentDigest>, String> {
    document_digests_with_results_cache(project, None)
}

fn document_digests_with_results_cache(
    project: &ProjectFile,
    results_cache: Option<&ResultFingerprintCache>,
) -> Result<HashMap<ProjectDocumentId, ContentDigest>, String> {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    FINGERPRINT_PASSES.with(|passes| passes.set(passes.get() + 1));
    let mut documents = HashMap::new();
    let mut plan_payloads = project
        .workspace
        .simulation_plan_payloads
        .iter()
        .map(|record| (record.plan_id, &record.payload))
        .collect::<Vec<_>>();
    plan_payloads
        .sort_by(|(left, _), (right, _)| left.as_uuid().as_bytes().cmp(right.as_uuid().as_bytes()));

    documents.insert(
        ProjectDocumentId::ProjectConfiguration,
        digest(&project_configuration_value(
            &project.workspace.project,
            &project.libraries,
            &project.workspace.configuration_sets,
            &project.workspace.design_management,
            project.workspace.pdk_callback_receipts(),
        )?)?,
    );
    documents.insert(
        ProjectDocumentId::SimulationPlan,
        digest(&(
            project
                .execution_context
                .as_ref()
                .map(|context| &context.simulation_plan),
            plan_payloads,
        ))?,
    );
    documents.insert(
        ProjectDocumentId::ModelCatalog,
        digest(&project.execution_context.as_ref().map(|context| {
            (
                &context.model_libraries,
                &context.model_resolution_records,
                &context.model_validation_receipt,
            )
        }))?,
    );
    documents.insert(
        ProjectDocumentId::ResultHistory,
        match results_cache {
            Some(cache) => cache.digest(
                &project.simulation_results,
                &project.workspace.report_documents,
                &project.workspace.visualization_documents,
                &project.result_presentation,
            )?,
            None => result_fingerprint::digest(
                &project.simulation_results,
                &project.workspace.report_documents,
                &project.workspace.visualization_documents,
                &project.result_presentation,
            )?,
        },
    );
    // The stimulus definitions ride the project document rather than a
    // sidecar, so without an identity here an edited library would move the
    // saved file while every document in this registry still read clean —
    // "no unsaved changes" over a library that had just been republished.
    documents.insert(
        ProjectDocumentId::StimulusLibrary,
        digest(&project.workspace.stimulus_library)?,
    );
    documents.insert(
        ProjectDocumentId::VerificationSpecifications,
        // Retained as a stable registry identity for older callers. Current
        // specifications participate in the simulation-plan payload digest.
        digest(&())?,
    );
    let code_workspace_sources = project
        .workspace
        .project_sources
        .iter_bundles()
        .filter(|bundle| matches!(bundle.owner(), ProjectSourceOwner::CodeWorkspace { .. }))
        .collect::<Vec<_>>();
    documents.insert(
        ProjectDocumentId::NetlistSource,
        digest(&(
            &project.workspace.netlist_source,
            &project.workspace.netlist_source_path,
            &project.workspace.netlist_document,
            &project.workspace.netlist_descriptor,
            &project.workspace.retained_netlist_decks,
            code_workspace_sources,
        ))?,
    );

    let mut references = HashSet::new();
    for key in project.workspace.schematic_buffers.keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for key in project.workspace.physical_layout_documents().keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for (library_key, library) in project.libraries.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for view_key in cell.views.keys() {
                references.insert(CellViewRef::new(library_key, cell_key, view_key));
            }
        }
    }
    for reference in references {
        let schematic = project
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .map(|schematic| SchematicDocumentContent::from(schematic.document()));
        let physical_layout = project.workspace.physical_layout_document(&reference);
        let view = project
            .libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(ViewDocumentContent::from);
        let project_source = project
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()));
        documents.insert(
            ProjectDocumentId::CellView(reference),
            digest(&(schematic, physical_layout, view, project_source))?,
        );
    }

    Ok(documents)
}
