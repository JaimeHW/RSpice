//! Canonical document fingerprint collection for save and dirty-state comparison.

use super::{
    DocumentFingerprints, ProjectDocumentId, ResultFingerprintCache, SchematicDocumentContent,
    ViewDocumentContent, cell_view_digest, configuration_digest, digest, library_catalog_digest,
    project_descriptor_digest, reference_from_key, result_fingerprint,
};
use crate::{ProjectExecutionContext, ProjectFile, ProjectWorkspace};
use rspice_design::library::View;
use rspice_design::physical_layout::PhysicalLayoutDocument;
use rspice_design::project_sources::{
    ProjectSourceBundle, ProjectSourceOwner, ProjectSourceRegistry,
};
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::cell_view::CellViewRef;
use rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord;
use rspice_simulation_contract::setup_state::SimulationSetup;
use rspice_vocabulary::product::ContentDigest;
use std::collections::{HashMap, HashSet};

#[cfg(any(test, feature = "document-fingerprint-observation"))]
thread_local! {
    /// Full document fingerprint passes, including retained sample scans.
    pub static FINGERPRINT_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Whole working-project captures: one per
    /// [`crate::ProjectWorkingSet::capture`].
    pub static PROJECT_CAPTURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Project documents digested from their content, one by one.
    pub static DOCUMENT_DIGESTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Drawings the working project's held digests digested from their
    /// content, rather than read from where they stand in their history.
    pub static DRAWING_DIGESTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Set while a check recomputes work that was already counted.
    static UNOBSERVED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Count one pass of `counter`, unless it runs inside [`unobserved`].
#[cfg(any(test, feature = "document-fingerprint-observation"))]
pub(crate) fn observe(counter: &'static std::thread::LocalKey<std::cell::Cell<usize>>) {
    if !UNOBSERVED.with(std::cell::Cell::get) {
        counter.with(|count| count.set(count.get() + 1));
    }
}

/// Run `work` without counting its captures and fingerprint passes, so the
/// counts stay what the application itself asked for. For a check that
/// recomputes an answer it is about to compare with one already counted.
pub fn unobserved<R>(work: impl FnOnce() -> R) -> R {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    {
        struct Restore(bool);
        impl Drop for Restore {
            fn drop(&mut self) {
                UNOBSERVED.with(|flag| flag.set(self.0));
            }
        }
        let _restore = Restore(UNOBSERVED.with(|flag| flag.replace(true)));
        work()
    }
    #[cfg(not(any(test, feature = "document-fingerprint-observation")))]
    work()
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
    observe(&FINGERPRINT_PASSES);
    let mut documents = HashMap::new();
    let mut insert = |id: ProjectDocumentId, digest: ContentDigest| {
        #[cfg(any(test, feature = "document-fingerprint-observation"))]
        observe(&DOCUMENT_DIGESTS);
        documents.insert(id, digest);
    };
    let workspace = &project.workspace;
    insert(
        ProjectDocumentId::ProjectConfiguration,
        configuration_digest(
            project_descriptor_digest(&workspace.project)?,
            library_catalog_digest(&project.libraries)?,
            digest(&workspace.configuration_sets)?,
            digest(&workspace.design_management)?,
        ),
    );
    insert(
        ProjectDocumentId::SimulationPlan,
        simulation_plan_digest(
            project
                .execution_context
                .as_ref()
                .map(|context| &context.simulation_plan),
            &workspace.simulation_plan_payloads,
        )?,
    );
    insert(
        ProjectDocumentId::ModelCatalog,
        model_catalog_digest(project.execution_context.as_ref())?,
    );
    insert(
        ProjectDocumentId::ResultHistory,
        match results_cache {
            Some(cache) => cache.digest(
                &project.simulation_results,
                &workspace.report_documents,
                &workspace.visualization_documents,
                &project.result_presentation,
            )?,
            None => result_fingerprint::digest(
                &project.simulation_results,
                &workspace.report_documents,
                &workspace.visualization_documents,
                &project.result_presentation,
            )?,
        },
    );
    insert(
        ProjectDocumentId::StimulusLibrary,
        stimulus_library_digest(workspace)?,
    );
    insert(
        ProjectDocumentId::NetlistSource,
        netlist_source_digest(workspace)?,
    );

    let mut references = HashSet::new();
    for key in workspace.schematic_buffers.keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for key in workspace.physical_layout_documents().keys() {
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
    let owns_sources = owns_cell_view_sources(&workspace.project_sources);
    for reference in references {
        let parts = [
            drawing_digest(workspace.schematic_buffers.get(&reference.key()))?,
            layout_digest(workspace.physical_layout_document(&reference))?,
            view_digest(
                project
                    .libraries
                    .get_library(&reference.library)
                    .and_then(|library| library.get_cell(&reference.cell))
                    .and_then(|cell| cell.get_view(&reference.view)),
            )?,
            source_digest(&workspace.project_sources, owns_sources, &reference)?,
        ];
        insert(
            ProjectDocumentId::CellView(reference),
            cell_view_digest(&parts),
        );
    }

    Ok(documents)
}

/// The persisted simulation plan and every plan's payload, in plan order.
pub(super) fn simulation_plan_digest(
    plan: Option<&SimulationSetup>,
    payloads: &[SimulationPlanPayloadRecord],
) -> Result<ContentDigest, String> {
    let mut payloads = payloads
        .iter()
        .map(|record| (record.plan_id, &record.payload))
        .collect::<Vec<_>>();
    payloads
        .sort_by(|(left, _), (right, _)| left.as_uuid().as_bytes().cmp(right.as_uuid().as_bytes()));
    digest(&(plan, payloads))
}

/// The model catalog, its resolution records and its validation receipt.
pub(super) fn model_catalog_digest(
    context: Option<&ProjectExecutionContext>,
) -> Result<ContentDigest, String> {
    digest(&context.map(|context| {
        (
            &context.model_libraries,
            &context.model_resolution_records,
            &context.model_validation_receipt,
        )
    }))
}

/// The stimulus definitions ride the project document rather than a
/// sidecar, so without an identity of their own an edited library would move
/// the saved file while every document in this registry still read clean:
/// "no unsaved changes" over a library that had just been republished.
pub(super) fn stimulus_library_digest(
    workspace: &ProjectWorkspace,
) -> Result<ContentDigest, String> {
    digest(&workspace.stimulus_library)
}

/// The decks, the code workspace's own sources and the added lines.
pub(super) fn netlist_source_digest(workspace: &ProjectWorkspace) -> Result<ContentDigest, String> {
    let code_workspace_sources = workspace
        .project_sources
        .iter_bundles()
        .filter(|bundle| matches!(bundle.owner(), ProjectSourceOwner::CodeWorkspace))
        .map(ProjectSourceBundle::authored_content)
        .collect::<Vec<_>>();
    digest(&(
        &workspace.decks,
        code_workspace_sources,
        &workspace.added_lines,
    ))
}

/// A cell view's drawing share: the document a save writes of it.
pub(super) fn drawing_digest(schematic: Option<&Schematic>) -> Result<ContentDigest, String> {
    digest(&schematic.map(|schematic| SchematicDocumentContent::from(schematic.document())))
}

/// A cell view's physical layout share.
pub(super) fn layout_digest(
    layout: Option<&PhysicalLayoutDocument>,
) -> Result<ContentDigest, String> {
    digest(&layout)
}

/// A cell view's entry in its library.
pub(super) fn view_digest(view: Option<&View>) -> Result<ContentDigest, String> {
    digest(&view.map(ViewDocumentContent::from))
}

/// Whether any project source is owned by a cell view: when none is, every
/// cell view's source share is the empty one.
pub(super) fn owns_cell_view_sources(sources: &ProjectSourceRegistry) -> bool {
    sources
        .iter_bundles()
        .any(|bundle| matches!(bundle.owner(), ProjectSourceOwner::CellView { .. }))
}

/// A cell view's project source share: the source it owns, as authored.
pub(super) fn source_digest(
    sources: &ProjectSourceRegistry,
    owns_sources: bool,
    reference: &CellViewRef,
) -> Result<ContentDigest, String> {
    let bundle = owns_sources
        .then(|| sources.bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone())))
        .flatten();
    digest(&bundle.map(ProjectSourceBundle::authored_content))
}
