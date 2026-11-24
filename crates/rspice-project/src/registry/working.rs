//! The working project's document digests, each held under the stamps of
//! what it reads.
//!
//! A capture clones the whole project to digest it, and an edit to one
//! drawing of a project of thousands of cells paid for every cell. Here
//! each document, and each share of a cell view, keeps its digest while the
//! stamps of what it reads stand, and a library edit digests its catalog and
//! views again. The digests are the ones a capture of the same working state
//! gets, which debug builds check on every held answer.
//!
//! A drawing is read from where it stands in its history instead: at a
//! position it was digested at, its share is that digest; at any other
//! position the history named, once the position the baseline left it at is
//! known, it reads unsaved with no digest at all, as an edit that moved it
//! there. An edit costs no digest, an undo back to where the save left the
//! drawing reads clean, and only content the history cannot name, and the
//! drawings at a save, are digested.
//!
//! A deck the project keeps as text is read by its texts' digests, each
//! taken once by the text itself. Once the netlist source is settled on a
//! baseline, a text with no digest taken is one kept since, an edit in the
//! Netlist editor or a deck added, and the netlist source reads unsaved
//! with no digest: a key typed in a 100 MB deck digests nothing. The texts
//! are digested when the next baseline is settled.
//!
//! The result history is read by its views' revisions first. A view's
//! revision is part of what a save writes of it and moves on every change,
//! so once the views the baseline holds are known, a view at a revision the
//! baseline does not hold it at, or a view added or taken out, makes the
//! history unsaved with no digest: a tick on a view of a hundred thousand
//! traces digests nothing. The history is digested when its views are the
//! baseline's, and when the next baseline is settled.

use std::collections::{HashMap, HashSet};

use rspice_design::deck::{DeckText, ProjectDeck};
use rspice_design::library::ViewType;
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::product::{ContentDigest, ObjectRevision, ResultDocumentId};

use super::fingerprints::{
    drawing_digest, layout_digest, model_catalog_digest, netlist_source_digest,
    netlist_source_digest_with, owns_cell_view_sources, simulation_plan_digest, source_digest,
    stimulus_library_digest, view_digest,
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
    positions: Positions,
    decks: DeckTexts,
    views: SavedViews,
}

/// A view by its identity and revision.
pub type ViewRevision = (ResultDocumentId, ObjectRevision);

/// The views of `workspace` by identity and revision, in its order.
pub fn view_revisions(workspace: &ProjectWorkspace) -> Vec<ViewRevision> {
    workspace
        .visualization_documents
        .iter()
        .map(|view| (view.id(), view.revision()))
        .collect()
}

/// How the result history reads the views the project keeps.
#[derive(Debug, Clone, Default)]
struct SavedViews {
    /// The views the baseline holds, by identity and revision, in its
    /// order, once the baseline was settled.
    saved: Option<Vec<ViewRevision>>,
    /// Whether the result history reads unsaved, its views not the ones the
    /// baseline holds.
    unsaved: bool,
}

impl SavedViews {
    /// The views of `workspace` by identity and revision when they are not
    /// the ones the baseline holds, in which case the result history is not
    /// the baseline's whatever else it holds; `None` when they are, or the
    /// baseline's are not known.
    fn edited(&self, workspace: &ProjectWorkspace) -> Option<Vec<ViewRevision>> {
        let saved = self.saved.as_ref()?;
        let views = view_revisions(workspace);
        (views != *saved).then_some(views)
    }

    /// The result history's share: unsaved, named by the views' revisions,
    /// when they are not the baseline's, else `history` digested.
    fn share(
        &mut self,
        workspace: &ProjectWorkspace,
        history: impl FnOnce() -> Result<ContentDigest, String>,
    ) -> Result<ContentDigest, String> {
        if let Some(views) = self.edited(workspace) {
            self.unsaved = true;
            return digest(&("unsaved result history", views));
        }
        self.unsaved = false;
        history()
    }
}

/// How the netlist source reads the texts of the decks the project keeps.
#[derive(Debug, Clone, Copy, Default)]
struct DeckTexts {
    /// Whether the netlist source was settled on the baseline: a text with
    /// no digest taken is then one kept since, and the netlist source reads
    /// unsaved from it with no digest.
    settled: bool,
    /// Whether the netlist source reads unsaved, a text it holds having no
    /// digest taken.
    unsaved: bool,
}

/// The drawing share digested at each position of a drawing's history.
#[derive(Debug, Clone, Default)]
struct Positions {
    digests: HashMap<u64, ContentDigest>,
    /// The cell views whose drawing stood, when the baseline was last
    /// settled, at a position `digests` holds and with the content the
    /// baseline holds: that is where the baseline left it, and the drawing
    /// at any other named position was edited since.
    settled: HashSet<CellViewRef>,
}

/// Where a drawing stood in its history when its share was taken, and
/// whether the share is its digest rather than the mark of a position no
/// digest was taken at, which reads unsaved.
#[derive(Debug, Clone, Copy)]
struct DrawingPosition {
    name: u64,
    digested: bool,
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
    /// Where that drawing stood in its history.
    position: Option<DrawingPosition>,
    shares: [ContentDigest; 4],
}

impl WorkingDocuments {
    /// Every working document's digest, as of the last [`Self::update`].
    pub fn documents(&self) -> &HashMap<ProjectDocumentId, ContentDigest> {
        &self.documents
    }

    /// The working project's fingerprints, as a capture would take them,
    /// but for the documents [`Self::read_unsaved`] names.
    pub fn fingerprints(&self) -> DocumentFingerprints {
        DocumentFingerprints::from_documents(self.documents.clone())
    }

    /// The documents whose drawing stands at a position named since the
    /// baseline was settled, the netlist source while a deck's text kept
    /// since has no digest, and the result history while a view stands at a
    /// revision the baseline does not hold: each reads unsaved, with no
    /// digest of its content.
    pub fn read_unsaved(&self) -> Vec<ProjectDocumentId> {
        let mut unsaved = self.held.as_ref().map_or_else(Vec::new, |held| {
            held.cell_views
                .iter()
                .filter(|(_, shares)| shares.position.is_some_and(|position| !position.digested))
                .map(|(reference, _)| ProjectDocumentId::CellView(reference.clone()))
                .collect::<Vec<_>>()
        });
        if self.decks.unsaved {
            unsaved.push(ProjectDocumentId::NetlistSource);
        }
        if self.views.unsaved {
            unsaved.push(ProjectDocumentId::ResultHistory);
        }
        unsaved
    }

    /// Settle the digests on a new `baseline`, as a save, an open or a
    /// revert takes one: every drawing read unsaved from its position is
    /// digested, and each drawing whose cell view the baseline holds as it
    /// stands is known to stand where the baseline left it; the decks'
    /// texts are digested, and a text kept after this reads unsaved; the
    /// views the baseline holds, `saved_views`, are known, and a result
    /// history read unsaved is read again against them, `result_history`
    /// digesting it when its views are the baseline's. Returns the
    /// documents whose digest changed. A failure forgets every digest.
    pub fn settle(
        &mut self,
        source: &WorkingSource<'_>,
        (baseline, saved_views): (Option<&DocumentFingerprints>, Option<Vec<ViewRevision>>),
        result_history: impl FnOnce() -> Result<ContentDigest, String>,
    ) -> Result<Vec<ProjectDocumentId>, String> {
        let settled = self.settle_drawings(source, (baseline, saved_views), result_history);
        if settled.is_err() {
            self.clear();
        }
        settled
    }

    fn settle_drawings(
        &mut self,
        source: &WorkingSource<'_>,
        (baseline, saved_views): (Option<&DocumentFingerprints>, Option<Vec<ViewRevision>>),
        result_history: impl FnOnce() -> Result<ContentDigest, String>,
    ) -> Result<Vec<ProjectDocumentId>, String> {
        let mut changed = Vec::new();
        self.views.saved = saved_views;
        if self.views.unsaved {
            let history = self.views.share(source.workspace, || {
                digested();
                result_history()
            })?;
            if self
                .documents
                .insert(ProjectDocumentId::ResultHistory, history)
                != Some(history)
            {
                changed.push(ProjectDocumentId::ResultHistory);
            }
        }
        if self.decks.unsaved {
            let netlist = netlist_source_digest(source.workspace)?;
            digested();
            self.decks.unsaved = false;
            if self
                .documents
                .insert(ProjectDocumentId::NetlistSource, netlist)
                != Some(netlist)
            {
                changed.push(ProjectDocumentId::NetlistSource);
            }
        }
        self.decks.settled = true;
        if let Some(held) = self.held.as_mut() {
            for (reference, shares) in &mut held.cell_views {
                let Some(position) = shares
                    .position
                    .as_mut()
                    .filter(|position| !position.digested)
                else {
                    continue;
                };
                let key = reference.key();
                let schematic = if held.active.as_deref() == Some(key.as_str()) {
                    Some(source.active_schematic)
                } else {
                    source.workspace.schematic_buffers.get(&key)
                }
                .ok_or_else(|| format!("the drawing of '{key}' is no longer held"))?;
                let drawing = drawing_digest(Some(schematic))?;
                drawing_digested();
                digested();
                self.positions.digests.insert(position.name, drawing);
                position.digested = true;
                shares.shares[0] = drawing;
                let id = ProjectDocumentId::CellView(reference.clone());
                let digest = cell_view_digest(&shares.shares);
                if self.documents.insert(id.clone(), digest) != Some(digest) {
                    changed.push(id);
                }
            }
        }
        self.positions.settled = self.held.as_ref().map_or_else(HashSet::new, |held| {
            held.cell_views
                .iter()
                .filter(|(reference, shares)| {
                    let id = ProjectDocumentId::CellView((*reference).clone());
                    shares.position.is_some()
                        && baseline.and_then(|baseline| baseline.get(&id))
                            == self.documents.get(&id)
                })
                .map(|(reference, _)| reference.clone())
                .collect()
        });
        changed.sort_by_cached_key(ProjectDocumentId::stable_key);
        Ok(changed)
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
            self.positions
                .share(&reference, source.active_schematic)
                .map(|(drawing, position)| {
                    shares.drawing = Some(stamp);
                    shares.position = Some(position);
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
        let views = &mut self.views;
        document(ProjectDocumentId::ResultHistory, results_moved, &mut || {
            views.share(workspace, || {
                result_history
                    .take()
                    .expect("the result history is digested once")()
            })
        })?;
        document(
            ProjectDocumentId::StimulusLibrary,
            workspace_moved,
            &mut || stimulus_library_digest(workspace),
        )?;
        let decks = &mut self.decks;
        let mut netlist_unsaved = decks.unsaved;
        document(
            ProjectDocumentId::NetlistSource,
            workspace_moved,
            &mut || {
                let (digest, unsaved) = netlist_share(workspace, decks.settled)?;
                netlist_unsaved = unsaved;
                Ok(digest)
            },
        )?;
        decks.unsaved = netlist_unsaved;

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
        let positions = &mut self.positions;
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
            let mut position = before
                .filter(|_| drawing_kept)
                .and_then(|before| before.position);
            share(0, drawing_kept, &mut || match schematic {
                Some(schematic) => positions.share(&reference, schematic).map(|(drawing, at)| {
                    position = Some(at);
                    drawing
                }),
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
                    position,
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

impl Positions {
    /// The drawing share of `reference`'s `schematic` and where it stands:
    /// the digest taken at its position when one was; the mark of an unsaved
    /// drawing when the history named a position other than the one the
    /// baseline left it at; else its digest, taken now and kept for its
    /// position. Content the history gave no name stands at its stamp.
    fn share(
        &mut self,
        reference: &CellViewRef,
        schematic: &Schematic,
    ) -> Result<(ContentDigest, DrawingPosition), String> {
        let named = schematic.history_state();
        let name = named.unwrap_or_else(|| schematic.content_stamp());
        if let Some(drawing) = self.digests.get(&name) {
            return Ok((
                *drawing,
                DrawingPosition {
                    name,
                    digested: true,
                },
            ));
        }
        if named.is_some() && self.settled.contains(reference) {
            return Ok((
                digest(&("unsaved drawing position", name))?,
                DrawingPosition {
                    name,
                    digested: false,
                },
            ));
        }
        let drawing = drawing_digest(Some(schematic))?;
        drawing_digested();
        self.digests.insert(name, drawing);
        Ok((
            drawing,
            DrawingPosition {
                name,
                digested: true,
            },
        ))
    }
}

/// The netlist source's digest, and whether it reads unsaved. Once the
/// decks' texts are `settled`, a text with no digest taken is one kept
/// since, and the source reads unsaved, named by the texts it holds rather
/// than digested; before, every text is digested.
fn netlist_share(
    workspace: &ProjectWorkspace,
    settled: bool,
) -> Result<(ContentDigest, bool), String> {
    if !settled {
        return Ok((netlist_source_digest(workspace)?, false));
    }
    if let Some(netlist) = netlist_source_digest_with(workspace, true)? {
        return Ok((netlist, false));
    }
    let texts = workspace
        .decks
        .iter()
        .filter_map(ProjectDeck::stored_deck)
        .flat_map(|stored| {
            std::iter::once(&stored.text).chain(stored.files.iter().map(|file| &file.text))
        })
        .map(DeckText::identity)
        .collect::<Vec<_>>();
    Ok((digest(&("unsaved deck text", texts))?, true))
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

/// Count one drawing digested from its content.
fn drawing_digested() {
    #[cfg(any(test, feature = "document-fingerprint-observation"))]
    super::observe(&super::DRAWING_DIGESTS);
}
