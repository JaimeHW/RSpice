//! The working project's document digests, each held under the stamps of
//! what it reads.
//!
//! A capture clones the whole project to digest it, and an edit to one
//! drawing of a project of thousands of cells paid for every cell. Here
//! each document, and each share of a cell view, keeps its digest while the
//! stamps of what it reads stand: an edit in the editor digests its one
//! drawing again, and a library edit its catalog and views. The digests are
//! the ones a capture of the same working state gets, which debug builds
//! check on every held answer.

use std::collections::{HashMap, HashSet};

use rspice_design::library::ViewType;
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::product::ContentDigest;

use super::fingerprints::{
    drawing_digest, layout_digest, model_catalog_digest, netlist_source_digest,
    owns_cell_view_sources, simulation_plan_digest, source_digest, stimulus_library_digest,
    view_digest,
};
use super::{
    DocumentFingerprints, ProjectDocumentId, cell_view_digest, configuration_digest, digest,
    library_catalog_digest, project_descriptor_digest, reference_from_key,
};
use crate::{ProjectExecutionContext, ProjectLibraries, ProjectWorkspace};

/// The working project as its owners hold it, with the stamp of each.
#[derive(Clone, Copy)]
pub struct WorkingSource<'a> {
    pub workspace: &'a ProjectWorkspace,
    /// Moves on every write to `workspace`.
    pub workspace_stamp: u64,
    /// The drawing in the editor, which stands for the active view's
    /// buffered copy while a schematic or testbench view is active.
    pub active_schematic: &'a Schematic,
    pub libraries: &'a ProjectLibraries,
    /// The persisted simulation plan's stamp, then the model catalog's.
    pub execution_stamp: [u64; 4],
    /// The retained results' stamp and the presentation's digest.
    pub results_stamp: (u64, ContentDigest),
}

/// Every working document's digest, kept between comparisons.
#[derive(Debug, Clone, Default)]
pub struct WorkingDocuments {
    documents: HashMap<ProjectDocumentId, ContentDigest>,
    held: Option<Held>,
}

/// The stamps the digests were taken under, and the shares they compose.
#[derive(Debug, Clone)]
struct Held {
    workspace: u64,
    /// The library catalog's stamp and its references'.
    catalog: (u64, u64),
    layouts: u64,
    execution: [u64; 4],
    /// The plan and model catalog as last captured, under `execution`.
    execution_context: ProjectExecutionContext,
    results: (u64, ContentDigest),
    catalog_digest: ContentDigest,
    configuration_sets: (u64, ContentDigest),
    design_management: (u64, ContentDigest),
    /// The key of the view whose drawing the editor holds, when one does.
    active: Option<String>,
    cell_views: HashMap<CellViewRef, CellViewShares>,
}

/// A cell view's four shares, and the stamp of the drawing digested.
#[derive(Debug, Clone, Copy)]
struct CellViewShares {
    drawing: Option<u64>,
    shares: [ContentDigest; 4],
}

impl WorkingDocuments {
    /// Every working document's digest, as of the last [`Self::update`].
    pub fn documents(&self) -> &HashMap<ProjectDocumentId, ContentDigest> {
        &self.documents
    }

    /// The working project's fingerprints, as a capture would take them.
    pub fn fingerprints(&self) -> DocumentFingerprints {
        DocumentFingerprints::from_documents(self.documents.clone())
    }

    /// Forget every digest: the next update digests every document.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Bring every digest up to `source`, digesting again only the
    /// documents and shares whose stamps moved, and return the documents
    /// whose digest changed, appeared or went. `execution_context` is called
    /// when the plan or model catalog moved, `result_history` when the
    /// results, the presentation or the workspace did. Any failure forgets
    /// every digest, so the next update starts whole.
    pub fn update(
        &mut self,
        source: &WorkingSource<'_>,
        execution_context: impl FnOnce() -> Result<ProjectExecutionContext, String>,
        result_history: impl FnOnce() -> Result<ContentDigest, String>,
    ) -> Result<Vec<ProjectDocumentId>, String> {
        let updated = match self.editor_only(source) {
            Some(updated) => updated,
            None => self.rescan(source, execution_context, result_history),
        };
        if updated.is_err() {
            self.clear();
        }
        updated
    }

    /// When nothing but the editor's drawing can have moved, its one
    /// document brought up to date; `None` when anything else moved.
    fn editor_only(
        &mut self,
        source: &WorkingSource<'_>,
    ) -> Option<Result<Vec<ProjectDocumentId>, String>> {
        let held = self.held.as_mut()?;
        let libraries = source.libraries;
        if held.workspace != source.workspace_stamp
            || held.catalog != (libraries.content_stamp(), libraries.references_stamp())
            || held.execution != source.execution_stamp
            || held.results != source.results_stamp
        {
            return None;
        }
        let Some(reference) = held.active.as_deref().and_then(reference_from_key) else {
            return Some(Ok(Vec::new()));
        };
        let stamp = source.active_schematic.content_stamp();
        let shares = held.cell_views.get_mut(&reference)?;
        if shares.drawing == Some(stamp) {
            return Some(Ok(Vec::new()));
        }
        Some(
            drawing_digest(Some(source.active_schematic)).map(|drawing| {
                shares.drawing = Some(stamp);
                shares.shares[0] = drawing;
                digested();
                let id = ProjectDocumentId::CellView(reference);
                let digest = cell_view_digest(&shares.shares);
                if self.documents.insert(id.clone(), digest) == Some(digest) {
                    Vec::new()
                } else {
                    vec![id]
                }
            }),
        )
    }

    /// Every document again, keeping each share whose stamps stand.
    fn rescan(
        &mut self,
        source: &WorkingSource<'_>,
        execution_context: impl FnOnce() -> Result<ProjectExecutionContext, String>,
        result_history: impl FnOnce() -> Result<ContentDigest, String>,
    ) -> Result<Vec<ProjectDocumentId>, String> {
        let workspace = source.workspace;
        let libraries = source.libraries;
        let previous = self.held.take();
        let held = previous.as_ref();
        let workspace_moved = held.is_none_or(|held| held.workspace != source.workspace_stamp);
        let catalog = (libraries.content_stamp(), libraries.references_stamp());
        let catalog_moved = held.is_none_or(|held| held.catalog != catalog);
        let layouts = workspace.physical_layout_stamp();
        let layouts_moved = held.is_none_or(|held| held.layouts != layouts);
        let mut documents = HashMap::with_capacity(self.documents.len());

        let (execution_context, execution_moved) = match held {
            Some(held) if held.execution == source.execution_stamp => {
                (held.execution_context.clone(), false)
            }
            _ => (execution_context()?, true),
        };

        let catalog_digest = match held {
            Some(held) if !catalog_moved => held.catalog_digest,
            _ => library_catalog_digest(libraries)?,
        };
        let configuration_sets = kept_or_digested(
            held.map(|held| held.configuration_sets),
            workspace.configuration_sets.stamp(),
            || digest(&workspace.configuration_sets),
        )?;
        let design_management = kept_or_digested(
            held.map(|held| held.design_management),
            workspace.design_management.stamp(),
            || digest(&workspace.design_management),
        )?;
        let kept = |id: &ProjectDocumentId| self.documents.get(id).copied();
        let configuration = ProjectDocumentId::ProjectConfiguration;
        let configuration_digest = match kept(&configuration) {
            Some(kept)
                if !workspace_moved
                    && !catalog_moved
                    && held.is_some_and(|held| {
                        held.configuration_sets.1 == configuration_sets.1
                            && held.design_management.1 == design_management.1
                    }) =>
            {
                kept
            }
            _ => {
                digested();
                configuration_digest(
                    project_descriptor_digest(&workspace.project)?,
                    catalog_digest,
                    configuration_sets.1,
                    design_management.1,
                )
            }
        };
        documents.insert(configuration, configuration_digest);

        let mut document = |id: ProjectDocumentId,
                            moved: bool,
                            digest: &mut dyn FnMut() -> Result<ContentDigest, String>|
         -> Result<(), String> {
            let value = match kept(&id) {
                Some(kept) if !moved => kept,
                _ => {
                    digested();
                    digest()?
                }
            };
            documents.insert(id, value);
            Ok(())
        };
        document(
            ProjectDocumentId::SimulationPlan,
            workspace_moved || execution_moved,
            &mut || plan_digest(workspace, &execution_context),
        )?;
        document(
            ProjectDocumentId::ModelCatalog,
            execution_moved,
            &mut || model_catalog_digest(Some(&execution_context)),
        )?;
        let results_moved =
            workspace_moved || held.is_none_or(|held| held.results != source.results_stamp);
        let mut result_history = Some(result_history);
        document(ProjectDocumentId::ResultHistory, results_moved, &mut || {
            result_history
                .take()
                .expect("the result history is digested once")()
        })?;
        document(
            ProjectDocumentId::StimulusLibrary,
            workspace_moved,
            &mut || stimulus_library_digest(workspace),
        )?;
        document(
            ProjectDocumentId::NetlistSource,
            workspace_moved,
            &mut || netlist_source_digest(workspace),
        )?;

        let active = matches!(
            workspace.active_view_type(),
            Some(ViewType::Schematic | ViewType::Testbench)
        )
        .then(|| workspace.active_key());
        let drawing_of = |reference: &CellViewRef| {
            let key = reference.key();
            if active.as_deref() == Some(key.as_str()) {
                Some(source.active_schematic)
            } else {
                workspace.schematic_buffers.get(&key)
            }
        };
        let mut references = HashSet::new();
        for key in workspace
            .schematic_buffers
            .keys()
            .chain(active.iter())
            .chain(workspace.physical_layout_documents().keys())
        {
            if let Some(reference) = reference_from_key(key) {
                references.insert(reference);
            }
        }
        for (library_key, library) in libraries.libraries_by_key() {
            for (cell_key, cell) in &library.cells {
                for view_key in cell.views.keys() {
                    references.insert(CellViewRef::new(library_key, cell_key, view_key));
                }
            }
        }
        let owns_sources = owns_cell_view_sources(&workspace.project_sources);
        let nothing = digest(&())?;
        let mut cell_views = HashMap::with_capacity(references.len());
        for reference in references {
            let before = held.and_then(|held| held.cell_views.get(&reference));
            let schematic = drawing_of(&reference);
            let stamp = schematic.map(Schematic::content_stamp);
            let mut shares = [nothing; 4];
            let mut moved = before.is_none();
            let mut share = |index: usize,
                             kept: bool,
                             digest: &mut dyn FnMut() -> Result<ContentDigest, String>|
             -> Result<(), String> {
                shares[index] = match before {
                    Some(before) if kept => before.shares[index],
                    _ => {
                        let value = digest()?;
                        moved |= before.is_none_or(|before| before.shares[index] != value);
                        value
                    }
                };
                Ok(())
            };
            let drawing_kept = before.is_some_and(|before| before.drawing == stamp);
            share(0, drawing_kept, &mut || match schematic {
                Some(schematic) => drawing_digest(Some(schematic)),
                None => Ok(nothing),
            })?;
            share(
                1,
                !layouts_moved,
                &mut || match workspace.physical_layout_document(&reference) {
                    Some(layout) => layout_digest(Some(layout)),
                    None => Ok(nothing),
                },
            )?;
            share(2, !catalog_moved, &mut || match libraries
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
            {
                Some(view) => view_digest(Some(view)),
                None => Ok(nothing),
            })?;
            share(3, !workspace_moved, &mut || {
                if owns_sources {
                    source_digest(&workspace.project_sources, true, &reference)
                } else {
                    Ok(nothing)
                }
            })?;
            let id = ProjectDocumentId::CellView(reference.clone());
            let value = match kept(&id) {
                Some(kept) if !moved => kept,
                _ => {
                    digested();
                    cell_view_digest(&shares)
                }
            };
            documents.insert(id, value);
            cell_views.insert(
                reference,
                CellViewShares {
                    drawing: stamp,
                    shares,
                },
            );
        }

        let mut changed = documents
            .iter()
            .filter(|(id, value)| self.documents.get(*id) != Some(*value))
            .map(|(id, _)| id.clone())
            .chain(
                self.documents
                    .keys()
                    .filter(|id| !documents.contains_key(*id))
                    .cloned(),
            )
            .collect::<Vec<_>>();
        changed.sort_by_cached_key(ProjectDocumentId::stable_key);
        self.documents = documents;
        self.held = Some(Held {
            workspace: source.workspace_stamp,
            catalog,
            layouts,
            execution: source.execution_stamp,
            execution_context,
            results: source.results_stamp,
            catalog_digest,
            configuration_sets,
            design_management,
            active,
            cell_views,
        });
        Ok(changed)
    }
}

/// `held`'s digest while its stamp is `stamp`, else `digest` taken now.
fn kept_or_digested(
    held: Option<(u64, ContentDigest)>,
    stamp: u64,
    digest: impl FnOnce() -> Result<ContentDigest, String>,
) -> Result<(u64, ContentDigest), String> {
    match held {
        Some((held, value)) if held == stamp => Ok((held, value)),
        _ => Ok((stamp, digest()?)),
    }
}

/// The plan document as a save writes it: every plan's payload seeded the
/// way [`crate::ProjectFile::new_with_execution_context`] seeds it.
fn plan_digest(
    workspace: &ProjectWorkspace,
    context: &ProjectExecutionContext,
) -> Result<ContentDigest, String> {
    let mut seeded = ProjectWorkspace::default();
    seeded.specs.clone_from(&workspace.specs);
    seeded
        .simulation_plan_payloads
        .clone_from(&workspace.simulation_plan_payloads);
    seeded.seed_plan_data(&context.simulation_plan);
    simulation_plan_digest(
        Some(&context.simulation_plan),
        &seeded.simulation_plan_payloads,
    )
}

/// Count one document digested from its content.
fn digested() {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    super::observe(&super::DOCUMENT_DIGESTS);
}
