//! One memoized projection of the design that runs, checks, and exports read.
//!
//! Preparing a run, validating before save, exporting a deck, and painting the
//! configuration page all need the same value: every schematic buffer
//! materialized through design management, overlaid with the live editor
//! buffer, paired with the frozen execution plan of the active configuration.
//! Building it per caller re-resolved the whole hierarchy and re-materialized
//! every cell view, several times per frame.
//!
//! The memo is keyed on content rather than on version counters, because no
//! counter in this crate covers every mutation the projection reads. Editing
//! sessions apply property changes live and commit one undo entry when the
//! session ends, so `content_version` moves after the edit is already
//! visible; `topology_version` deliberately ignores property edits;
//! `ConnectivityContract` carries no revision at all; and
//! `DesignManagementCatalog::variants_mut`/`annotation_mut` hand out
//! unrestricted `&mut` without advancing the catalog revision — the two
//! surfaces that preview a candidate by cloning the workspace mutate through
//! exactly those. A counter-keyed memo therefore serves a stale design, which
//! is the one failure this value must not have.
//!
//! So each cell view and each authority is digested, and the digests key both
//! the projection and the per-cell-view materialization. A digest is taken
//! once per [stamp](crate::stamped): every schematic document and every
//! authority is stamped, and a stamp moves on every mutable access, through
//! whichever method or `&mut` borrow made it — which is the coverage no
//! counter had. A call in which nothing was edited therefore digests nothing,
//! one edit digests exactly one cell view, and a project whose authorities
//! cannot be serialized bypasses the cache instead of risking a stale answer.

use std::any::Any;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use sha2::Digest as _;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use crate::configuration_set::ConfigurationSetCatalog;
use crate::connectivity_contract::ConnectivityContract;
use crate::hierarchy::{
    ConfigurationExecutionPlan, HierarchyContext, HierarchyDocuments, HierarchyResolution,
    HierarchySchematic, HierarchySourceFiles, find_cell, find_library, find_view,
    hierarchy_stop_view, materialize_authoritative_source_binding, resolve_hierarchy,
};
use crate::library::LibraryCatalog;
use crate::project_sources::ProjectSourceRegistry;
use crate::stamped::Stamped;
use crate::schematic::{
    component::{LibraryCellInstance, PreparedVariantReplacement},
    document::SchematicDocument,
    owned::Schematic,
};
use crate::symbol_resolver::SymbolResolver;
use rspice_app_types::product::{ContentDigest, ObjectRevision, ProjectId};
use rspice_design_model::{
    cell_view::CellViewRef,
    design_management::{DesignManagementCatalog, DesignManagementError, VariantObjectOverride},
};

/// The live design and source metadata needed to freeze one schematic.
#[derive(Clone, Copy)]
pub struct SchematicSource<'a> {
    pub schematic: &'a Schematic,
    pub current_file: Option<&'a Path>,
    pub read_only: bool,
    pub modified: bool,
}

impl AsRef<SchematicDocument> for SchematicSource<'_> {
    fn as_ref(&self) -> &SchematicDocument {
        self.schematic.document()
    }
}

impl ProjectionSource for SchematicSource<'_> {
    fn projection_source(&self) -> SchematicSource<'_> {
        *self
    }
}

/// Borrow the metadata associated with each stored design without owning editor state.
pub trait ProjectionSources {
    fn schematic_source<'a>(&'a self, key: &str, schematic: &'a Schematic) -> SchematicSource<'a>;
}

/// Borrow design authority without retaining editor interaction or history.
pub trait ProjectionSource: AsRef<SchematicDocument> {
    fn projection_source(&self) -> SchematicSource<'_>;

    fn materialize(
        &self,
        design_management: &DesignManagementCatalog,
        cell_view_key: &str,
    ) -> Result<ProjectedSchematic, DesignManagementError> {
        let source = self.projection_source();
        let (document, repaired) = source
            .schematic
            .materialize_design_management_schematic(design_management, cell_view_key)?;
        Ok(ProjectedSchematic {
            document: Arc::new(document),
            content_digest: Arc::default(),
            modified: source.modified,
            current_file: source.current_file.map(Path::to_path_buf),
            topology_version: source
                .schematic
                .topology_version()
                .wrapping_add(repaired.topology_changes),
        })
    }
}

/// Immutable execution content, shared by cached projections until a variant changes it.
#[derive(Debug, Clone)]
pub struct ProjectedSchematic {
    document: Arc<SchematicDocument>,
    /// The document's digest, taken on first demand and shared by every
    /// clone of this materialization, so a projection rebuilt around an
    /// unchanged cell view does not digest it again. A variant replacement
    /// makes a new document and, with it, a new slot.
    content_digest: Arc<OnceLock<Option<ContentDigest>>>,
    modified: bool,
    current_file: Option<PathBuf>,
    topology_version: u64,
}

impl ProjectedSchematic {
    pub fn document(&self) -> &SchematicDocument {
        &self.document
    }

    /// The digest of the document as projected — the drawing a check or a
    /// deck reads, after design management placed its pages — or `None`
    /// where it cannot be serialized.
    pub fn content_digest(&self) -> Option<ContentDigest> {
        *self.content_digest.get_or_init(|| {
            #[cfg(any(test, feature = "schematic-test-fixtures"))]
            PROJECTED_DIGESTS.with(|count| count.set(count.get().wrapping_add(1)));
            let mut hasher = sha2::Sha256::new();
            hash_serialized(&mut hasher, &*self.document)?;
            Some(ContentDigest::from_bytes(hasher.finalize().into()))
        })
    }

    pub fn current_file(&self) -> Option<&Path> {
        self.current_file.as_deref()
    }

    pub const fn topology_version(&self) -> u64 {
        self.topology_version
    }

    fn apply_variant_replacement(&mut self, prepared: PreparedVariantReplacement) {
        prepared.apply_to(Arc::make_mut(&mut self.document));
        self.content_digest = Arc::default();
    }
}

impl AsRef<SchematicDocument> for ProjectedSchematic {
    fn as_ref(&self) -> &SchematicDocument {
        self.document()
    }
}

/// Derived design data; cloning shares immutable content, never live edits.
#[derive(Debug, Clone, Default)]
pub struct DesignProjectionCache {
    projection: RefCell<Option<Arc<DesignProjection>>>,
    materialized: RefCell<HashMap<String, (BufferMemoKey, ProjectedSchematic)>>,
    /// The last measurement of every input, reused whole while no stamp it
    /// was taken at has moved.
    inputs: RefCell<Option<Arc<DesignInputs>>>,
    /// The authorities' digest, by the stamps it was taken at.
    authority: RefCell<Option<([u64; 3], ContentDigest)>>,
    /// Each cell view's digest, by the stamp it was taken at.
    cell_views: RefCell<HashMap<String, (CellViewStamp, ContentDigest)>>,
}

impl DesignProjectionCache {
    pub fn invalidate(&mut self) {
        self.projection.get_mut().take();
    }

    #[cfg(any(test, feature = "schematic-test-fixtures"))]
    pub fn clear(&self) {
        self.projection.borrow_mut().take();
        self.materialized.borrow_mut().clear();
        self.inputs.borrow_mut().take();
        self.authority.borrow_mut().take();
        self.cell_views.borrow_mut().clear();
    }
}

/// Borrowed authorities required to materialize and resolve a design.
pub struct ProjectionContext<'a, M, F> {
    pub schematic_buffers: &'a HashMap<String, Schematic>,
    pub sources: M,
    pub configuration_sets: &'a Stamped<ConfigurationSetCatalog>,
    pub design_management: &'a Stamped<DesignManagementCatalog>,
    pub connectivity: &'a Stamped<ConnectivityContract>,
    pub project_id: ProjectId,
    pub project_revision: ObjectRevision,
    pub project_sources: &'a ProjectSourceRegistry,
    pub root: CellViewRef,
    pub source_files: &'a F,
    pub cache: &'a DesignProjectionCache,
}

struct ProjectedDocuments<'a>(&'a HashMap<String, ProjectedSchematic>);

impl HierarchyDocuments for ProjectedDocuments<'_> {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        self.0
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
            .map(|(_, schematic)| HierarchySchematic {
                document: schematic.document(),
                modified: schematic.modified,
            })
    }
}

/// Frozen live-buffer projection paired with its configuration plan. Holding
/// both in one value prevents a caller from resolving one hierarchy and
/// accidentally netlisting a different editor buffer.
///
/// The value is immutable once built. Callers share it through an [`Arc`], so
/// a projection that is still current costs a reference count rather than a
/// hierarchy walk.
pub struct DesignProjection {
    root: CellViewRef,
    schematic_buffers: HashMap<String, ProjectedSchematic>,
    plan: ConfigurationExecutionPlan,
    resolution: HierarchyResolution,
    connectivity: ConnectivityContract,
    /// Inputs this projection was built from, or `None` when they could not be
    /// digested. A keyless projection is never cached.
    key: Option<DesignProjectionKey>,
    /// One cell view's net summary, keyed by folded cell-view key.
    ///
    /// The slot is type-erased because the net summary belongs to the
    /// generator that extracts it, which sits above this module: naming that
    /// type here would point the design model back up the architecture. A
    /// `Mutex` rather than a `RefCell` keeps a frozen projection `Send +
    /// Sync`, so handing one to a worker never forces a rebuild.
    nets: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

/// The projection under the name the execution paths have always used for it.
pub type ConfigurationExecutionProjection = Arc<DesignProjection>;

/// Written by hand because the memo slot holds `dyn Any`, which has no
/// `Debug`. Its size is reported instead of its contents.
impl std::fmt::Debug for DesignProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DesignProjection")
            .field("root", &self.root)
            .field("schematic_buffers", &self.schematic_buffers)
            .field("plan", &self.plan)
            .field("resolution", &self.resolution)
            .field("connectivity", &self.connectivity)
            .field("key", &self.key)
            .field(
                "memoized_nets",
                &self
                    .nets
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .len(),
            )
            .finish()
    }
}

impl DesignProjection {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn root_schematic(&self) -> Option<&ProjectedSchematic> {
        self.schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&self.root.key()))
            .map(|(_, schematic)| schematic)
    }

    pub const fn schematic_buffers(&self) -> &HashMap<String, ProjectedSchematic> {
        &self.schematic_buffers
    }

    /// The execution plan this projection was resolved into.
    ///
    /// Every projection carries one, configured or not: a configuration decides
    /// which view each occurrence binds to, never whether the hierarchy has an
    /// execution authority at all. So a caller holding a projection holds a
    /// plan, and no surface has to state what it would show without one.
    pub const fn plan(&self) -> &ConfigurationExecutionPlan {
        &self.plan
    }

    /// Diagnostics from the same materialized hierarchy that owns the plan.
    /// Inspection retains unresolved bindings so a UI can explain why this
    /// circuit cannot execute without resolving a different authored design.
    pub const fn hierarchy_resolution(&self) -> &HierarchyResolution {
        &self.resolution
    }

    /// Admit an inspected projection through the existing execution contract.
    /// Unconfigured missing instances retain their generator diagnostics;
    /// configured circuits require their complete hierarchy to resolve.
    pub fn into_execution(self: Arc<Self>) -> Result<Arc<Self>, ConfigurationExecutionPlanError> {
        if self.plan.configuration_id().is_some() && !self.resolution.is_valid() {
            let diagnostics = self
                .resolution
                .bindings
                .iter()
                .filter(|binding| !binding.status.is_resolved())
                .map(|binding| {
                    binding.diagnostic.clone().unwrap_or_else(|| {
                        format!(
                            "{} is {}",
                            binding.reference.display_path(),
                            binding.status.label()
                        )
                    })
                })
                .collect::<Vec<_>>()
                .join("; ");
            return Err(ConfigurationExecutionPlanError::Unresolved(diagnostics));
        }
        Ok(self)
    }

    pub const fn connectivity(&self) -> &ConnectivityContract {
        &self.connectivity
    }

    /// One cell view's net summary, extracted on first demand and retained
    /// for the life of the projection.
    ///
    /// The caller owns the type and the extraction; the projection owns only
    /// the slot, so the design model never names a generator type. The
    /// extractor lives beside the generator, as
    /// `simulation::netlist_gen::projection_nets`.
    pub fn memo_nets(
        &self,
        cell_view_key: &str,
        build: impl FnOnce() -> Arc<dyn Any + Send + Sync>,
    ) -> Arc<dyn Any + Send + Sync> {
        let folded = cell_view_key.to_ascii_lowercase();
        {
            let memo = self.nets.lock().unwrap_or_else(PoisonError::into_inner);
            if let Some(nets) = memo.get(&folded) {
                return Arc::clone(nets);
            }
        }
        let nets = build();
        self.nets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(folded, Arc::clone(&nets));
        nets
    }
}

/// Every input a [`DesignProjection`] is derived from, in the cheapest exact
/// form each one has. Two projections built from equal keys are equal values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignProjectionKey {
    root: CellViewRef,
    active_key: String,
    /// Digest over the project authorities and the content of every cell
    /// view, with the live editor buffer already overlaid.
    design_epoch: ContentDigest,
    library_revision: u64,
    project_revision: ObjectRevision,
}

/// The digested inputs, retained across one call so the projection key and
/// every per-cell-view memo key are read from the same measurement, and
/// across calls for as long as every stamp it was taken at stands.
#[derive(Debug)]
struct DesignInputs {
    /// The stamps of the configuration, design-management, and connectivity
    /// authorities, in that order.
    authority: [u64; 3],
    /// Digest over the configuration, design-management, and connectivity
    /// authorities.
    authority_epoch: ContentDigest,
    /// Content digest per cell view, keyed exactly as the projection keys its
    /// buffers and ordered so the fold over them is deterministic.
    cell_views: BTreeMap<String, ContentDigest>,
    /// What each cell view's digest was taken at, under the same keys.
    stamps: HashMap<String, CellViewStamp>,
    /// Buffer spelling the live editor overlay occupies.
    active_key: String,
    /// The fold of the authorities and every cell view.
    design_epoch: ContentDigest,
}

impl DesignInputs {
    fn fold(authority_epoch: ContentDigest, cell_views: &BTreeMap<String, ContentDigest>) -> ContentDigest {
        let mut hasher = sha2::Sha256::new();
        hasher.update(authority_epoch.as_bytes());
        for (cell_view_key, digest) in cell_views {
            hasher.update((cell_view_key.len() as u64).to_le_bytes());
            hasher.update(cell_view_key.as_bytes());
            hasher.update(digest.as_bytes());
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    /// Whether this measurement still describes the design: the same
    /// authorities, the same overlay, and the same cell views at the same
    /// stamps, origins and read-only markers.
    fn stands<'s>(
        &self,
        authority: [u64; 3],
        active_key: &str,
        mut sources: impl ExactSizeIterator<Item = (&'s str, SchematicSource<'s>)>,
    ) -> bool {
        self.authority == authority
            && self.active_key == active_key
            && sources.len() == self.stamps.len()
            && sources.all(|(key, source)| {
                self.stamps
                    .get(key)
                    .is_some_and(|stamp| stamp.matches(&source))
            })
    }

    fn memo_key(&self, cell_view_key: &str) -> Option<BufferMemoKey> {
        Some(BufferMemoKey {
            source_epoch: *self.cell_views.get(cell_view_key)?,
            authority_epoch: self.authority_epoch,
        })
    }
}

/// What one cell view's digest was taken at: the document's stamp and the
/// two runtime fields the digest covers beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CellViewStamp {
    stamp: u64,
    current_file: Option<PathBuf>,
    read_only: bool,
}

impl CellViewStamp {
    fn of(source: &SchematicSource<'_>) -> Self {
        Self {
            stamp: source.schematic.content_stamp(),
            current_file: source.current_file.map(Path::to_path_buf),
            read_only: source.read_only,
        }
    }

    fn matches(&self, source: &SchematicSource<'_>) -> bool {
        self.stamp == source.schematic.content_stamp()
            && self.current_file.as_deref() == source.current_file
            && self.read_only == source.read_only
    }
}

/// Identity of one materialized cell view: the source document it was
/// projected from, and the design-management authority that projected it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BufferMemoKey {
    source_epoch: ContentDigest,
    authority_epoch: ContentDigest,
}

/// Serialize `value` into `hasher` without an intermediate buffer. `None`
/// means the value could not be serialized, which every caller reads as
/// "this projection cannot be memoized".
fn hash_serialized<T: serde::Serialize>(hasher: &mut sha2::Sha256, value: &T) -> Option<()> {
    struct Sink<'a>(&'a mut sha2::Sha256);

    impl std::io::Write for Sink<'_> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.update(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    serde_json::to_writer(Sink(hasher), value).ok()
}

/// Digest one cell view's document.
///
/// The persisted content is the projection's input, plus the two runtime
/// fields the materialized clone carries into execution: the document's
/// native origin, which relative `.include` paths resolve against, and its
/// read-only marker.
fn cell_view_digest(schematic: &impl ProjectionSource) -> Option<ContentDigest> {
    #[cfg(any(test, feature = "schematic-test-fixtures"))]
    CELL_VIEW_DIGESTS.with(|count| count.set(count.get().wrapping_add(1)));
    let source = schematic.projection_source();
    let mut hasher = sha2::Sha256::new();
    hash_serialized(&mut hasher, source.schematic.document())?;
    hasher.update([0xff]);
    hash_serialized(&mut hasher, &source.current_file)?;
    hasher.update([u8::from(source.read_only)]);
    Some(ContentDigest::from_bytes(hasher.finalize().into()))
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigurationExecutionPlanError {
    #[error("configuration hierarchy is unresolved: {0}")]
    Unresolved(String),
    #[error("configuration root {0} has no materialized schematic buffer")]
    MissingRoot(String),
    #[error("design-management projection is invalid: {0}")]
    DesignManagement(String),
}

// Count of cell-view materializations performed since the last reset, so a
// test can assert that one edit re-materializes one cell view.
#[cfg(any(test, feature = "schematic-test-fixtures"))]
thread_local! {
    static MATERIALIZATIONS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "schematic-test-fixtures"))]
pub fn reset_materialization_count() {
    MATERIALIZATIONS.with(|count| count.set(0));
}

#[cfg(any(test, feature = "schematic-test-fixtures"))]
pub fn materialization_count() -> u64 {
    MATERIALIZATIONS.with(std::cell::Cell::get)
}

// Counts of the documents digested since the last reset — the source
// documents a projection is keyed on, and the projected documents a check's
// evidence is keyed on — so a test can assert that a call with nothing edited
// digests none, and that one edit digests one.
#[cfg(any(test, feature = "schematic-test-fixtures"))]
thread_local! {
    static CELL_VIEW_DIGESTS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static PROJECTED_DIGESTS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// The source and projected documents digested on this thread since the
/// last reset.
#[cfg(any(test, feature = "schematic-test-fixtures"))]
pub fn document_digest_counts() -> (u64, u64) {
    (
        CELL_VIEW_DIGESTS.with(std::cell::Cell::get),
        PROJECTED_DIGESTS.with(std::cell::Cell::get),
    )
}

#[cfg(any(test, feature = "schematic-test-fixtures"))]
pub fn reset_document_digest_counts() {
    CELL_VIEW_DIGESTS.with(|count| count.set(0));
    PROJECTED_DIGESTS.with(|count| count.set(0));
}

impl<M: ProjectionSources, F: HierarchySourceFiles> ProjectionContext<'_, M, F> {
    /// Inspect the materialized circuit even when its configured hierarchy
    /// cannot execute. The receipt and execution route share this cache;
    /// structural materialization errors still refuse the entire projection.
    pub fn inspect_design_projection(
        &self,
        libraries: &LibraryCatalog,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        let inputs = self.design_inputs(active_reference, active_schematic);
        let key = inputs
            .as_ref()
            .map(|inputs| self.projection_key(libraries, inputs));
        if let Some(key) = key.as_ref() {
            let cached = self.cache.projection.borrow();
            if let Some(projection) = cached.as_ref()
                && projection.key.as_ref() == Some(key)
            {
                return Ok(Arc::clone(projection));
            }
        }
        let projection = Arc::new(self.build_design_projection(
            libraries,
            active_reference,
            active_schematic,
            inputs.as_deref(),
            key,
        )?);
        if projection.key.is_some() {
            *self.cache.projection.borrow_mut() = Some(Arc::clone(&projection));
        }
        Ok(projection)
    }

    /// Exact in-memory design authority for derived presentation caches.
    /// Callers must also include any inputs their own derived value adds.
    /// A non-serializable authority cannot acquire a cache identity.
    pub fn design_projection_key(
        &self,
        libraries: &LibraryCatalog,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Option<DesignProjectionKey> {
        self.design_inputs(active_reference, active_schematic)
            .map(|inputs| self.projection_key(libraries, &inputs))
    }

    fn projection_key(
        &self,
        libraries: &LibraryCatalog,
        inputs: &DesignInputs,
    ) -> DesignProjectionKey {
        DesignProjectionKey {
            root: self.root.clone(),
            active_key: inputs.active_key.clone(),
            design_epoch: inputs.design_epoch,
            library_revision: libraries.revision(),
            project_revision: self.project_revision,
        }
    }

    fn build_design_projection(
        &self,
        libraries: &LibraryCatalog,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
        inputs: Option<&DesignInputs>,
        key: Option<DesignProjectionKey>,
    ) -> Result<DesignProjection, ConfigurationExecutionPlanError> {
        let root = self.root.clone();
        // The live editor buffer replaces its persisted copy under whichever
        // spelling the workspace already holds, so a case-different tab cannot
        // introduce a second master for the same cell view.
        let active_key = self.active_buffer_key(active_reference);
        let mut schematic_buffers = HashMap::with_capacity(self.schematic_buffers.len() + 1);
        for (cell_view_key, schematic) in self.schematic_buffers {
            if *cell_view_key == active_key {
                continue;
            }
            let memo_key = inputs.and_then(|inputs| inputs.memo_key(cell_view_key));
            let materialized = self.materialized_cell_view(
                cell_view_key,
                &self.sources.schematic_source(cell_view_key, schematic),
                memo_key,
            )?;
            schematic_buffers.insert(cell_view_key.clone(), materialized);
        }
        let memo_key = inputs.and_then(|inputs| inputs.memo_key(&active_key));
        let materialized = self.materialized_cell_view(&active_key, active_schematic, memo_key)?;
        schematic_buffers.insert(active_key, materialized);
        self.cache
            .materialized
            .borrow_mut()
            .retain(|cell_view_key, _| schematic_buffers.contains_key(cell_view_key));

        self.materialize_variant_bindings(
            libraries,
            active_reference,
            active_schematic,
            &mut schematic_buffers,
        )?;

        // Resolve the exact circuit that the netlister receives. Variants can
        // remove instances, replace masters or introduce new child instances;
        // the raw authored hierarchy is not their execution authority.
        let documents = ProjectedDocuments(&schematic_buffers);
        let (resolution, plan) = resolve_hierarchy(
            HierarchyContext {
                documents: &documents,
                libraries,
                project_id: self.project_id,
                project_sources: self.project_sources,
                source_files: self.source_files,
            },
            root.clone(),
            self.configuration_sets.active(),
        );

        let projection = DesignProjection {
            root,
            schematic_buffers,
            plan,
            resolution,
            connectivity: (**self.connectivity).clone(),
            key,
            nets: Mutex::new(HashMap::new()),
        };
        if projection.root_schematic().is_none() {
            return Err(ConfigurationExecutionPlanError::MissingRoot(
                projection.root.display_path(),
            ));
        }
        Ok(projection)
    }

    /// Pin geometry belongs to the authored occurrence. Apply it after the
    /// per-document materialization cache: library symbols and other masters'
    /// live interfaces participate in the complete projection key, not in one
    /// document's source-only memo key.
    fn materialize_variant_bindings(
        &self,
        libraries: &LibraryCatalog,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
        projected: &mut HashMap<String, ProjectedSchematic>,
    ) -> Result<(), ConfigurationExecutionPlanError> {
        let variants = self.design_management.variants();
        let Some(active_variant) = variants.active_variant_id() else {
            return Ok(());
        };
        let variant = variants.resolve(active_variant).map_err(|error| {
            ConfigurationExecutionPlanError::DesignManagement(error.to_string())
        })?;
        let source_symbols = SymbolResolver::new(libraries, self.schematic_buffers)
            .with_active_schematic(active_reference, active_schematic.as_ref());
        let target_symbols = SymbolResolver::new(libraries, projected);
        let mut updates = Vec::new();
        for (object, change) in &variant.overrides {
            let VariantObjectOverride::Substitute { replacement } = change else {
                continue;
            };
            let refusal = |reason: String| {
                ConfigurationExecutionPlanError::DesignManagement(format!(
                    "Replacement of {} object {}: {reason}",
                    object.cell_view_key(),
                    object.object_id()
                ))
            };
            let (key, projected_document) = projected
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(object.cell_view_key()))
                .ok_or_else(|| refusal("source document is unavailable".to_owned()))?;
            let source_document = if key.eq_ignore_ascii_case(&active_reference.key()) {
                active_schematic.as_ref()
            } else {
                self.schematic_buffers
                    .get(key)
                    .ok_or_else(|| refusal("authored document is unavailable".to_owned()))?
                    .document()
            };
            let source = source_document
                .components
                .iter()
                .find(|component| component.id == object.object_id())
                .ok_or_else(|| refusal("source component is unavailable".to_owned()))?;
            let projected_component = projected_document
                .document()
                .components
                .iter()
                .find(|component| component.id == source.id)
                .ok_or_else(|| refusal("projected component is unavailable".to_owned()))?;
            let source_symbol = source
                .library_cell
                .as_ref()
                .and_then(|binding| source_symbols.resolve_binding(binding));
            let variant_source = source
                .variant_source(source_symbol.as_ref())
                .map_err(&refusal)?;
            let target_reference =
                CellViewRef::new(&replacement.library, &replacement.cell, &replacement.view);
            let mut target_binding = LibraryCellInstance::new(
                &target_reference.library,
                &target_reference.cell,
                &target_reference.view,
            );
            let mut target_view_type = None;
            if let Some(library) = find_library(libraries, &target_reference.library)
                && let Some(cell) = find_cell(library, &target_reference.cell)
                && let Some(view) = find_view(cell, &target_reference.view)
            {
                if hierarchy_stop_view(view.view_type) {
                    target_binding = materialize_authoritative_source_binding(
                        &target_binding,
                        library,
                        cell,
                        view,
                        self.project_id,
                        self.project_sources,
                        libraries,
                    )
                    .map_err(&refusal)?;
                }
                target_view_type = Some(view.view_type);
            }
            target_binding = target_binding
                .with_variant_model_section(replacement.model_section.as_deref(), target_view_type)
                .map_err(&refusal)?;
            let target_symbol =
                target_symbols
                    .resolve_binding(&target_binding)
                    .ok_or_else(|| {
                        refusal(format!(
                            "{} has no resolved pin contract",
                            target_reference.display_path()
                        ))
                    })?;
            let replacement = variant_source
                .prepare_replacement(projected_component, target_binding, &target_symbol)
                .map_err(&refusal)?;
            updates.push((key.clone(), replacement));
        }
        for (key, replacement) in updates {
            projected
                .get_mut(&key)
                .expect("prepared document")
                .apply_variant_replacement(replacement);
        }
        Ok(())
    }

    /// One cell view's design-management projection, reused while its source
    /// document and the design-management authority both stand still. This is
    /// what keeps a single component edit from re-materializing the whole
    /// design.
    fn materialized_cell_view(
        &self,
        cell_view_key: &str,
        source: &impl ProjectionSource,
        memo_key: Option<BufferMemoKey>,
    ) -> Result<ProjectedSchematic, ConfigurationExecutionPlanError> {
        if let Some(memo_key) = memo_key.as_ref() {
            let memo = self.cache.materialized.borrow();
            if let Some((cached_key, cached)) = memo.get(cell_view_key)
                && cached_key == memo_key
            {
                return Ok(cached.clone());
            }
        }
        #[cfg(any(test, feature = "schematic-test-fixtures"))]
        MATERIALIZATIONS.with(|count| count.set(count.get().wrapping_add(1)));
        let materialized = source
            .materialize(self.design_management, cell_view_key)
            .map_err(|error| {
                ConfigurationExecutionPlanError::DesignManagement(error.to_string())
            })?;
        if let Some(memo_key) = memo_key {
            self.cache
                .materialized
                .borrow_mut()
                .insert(cell_view_key.to_owned(), (memo_key, materialized.clone()));
        }
        Ok(materialized)
    }

    /// Buffer spelling the live editor overlay occupies: the persisted one
    /// when the workspace already holds this cell view under any casing, so a
    /// case-different tab cannot introduce a second master.
    fn active_buffer_key(&self, active_reference: &CellViewRef) -> String {
        self.schematic_buffers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(&active_reference.key()))
            .cloned()
            .unwrap_or_else(|| active_reference.key())
    }

    /// Every cell view the projection is made of, under the key it is
    /// projected under: each stored buffer, with the live editor buffer in
    /// place of its stored copy.
    fn cell_view_sources<'s>(
        &'s self,
        active_key: &'s str,
        active: SchematicSource<'s>,
    ) -> std::vec::IntoIter<(&'s str, SchematicSource<'s>)> {
        let mut sources = Vec::with_capacity(self.schematic_buffers.len() + 1);
        for (cell_view_key, schematic) in self.schematic_buffers {
            if cell_view_key != active_key {
                sources.push((
                    cell_view_key.as_str(),
                    self.sources.schematic_source(cell_view_key, schematic),
                ));
            }
        }
        sources.push((active_key, active));
        sources.into_iter()
    }

    /// Digest every input the projection is derived from. `None` means one of
    /// them could not be serialized, which the caller reads as "this
    /// projection cannot be memoized".
    ///
    /// Each digest is taken once per stamp: a call in which no stamp moved
    /// returns the last measurement whole, and one in which a cell view was
    /// edited digests that cell view and folds the rest from memory.
    fn design_inputs(
        &self,
        active_reference: &CellViewRef,
        active_schematic: &impl ProjectionSource,
    ) -> Option<Arc<DesignInputs>> {
        let authority = [
            self.configuration_sets.stamp(),
            self.design_management.stamp(),
            self.connectivity.stamp(),
        ];
        let active_key = self.active_buffer_key(active_reference);
        let active = active_schematic.projection_source();
        if let Some(last) = self.cache.inputs.borrow().as_ref()
            && last.stands(
                authority,
                &active_key,
                self.cell_view_sources(&active_key, active),
            )
        {
            return Some(Arc::clone(last));
        }

        let remembered = *self.cache.authority.borrow();
        let authority_epoch = match remembered {
            Some((stamps, epoch)) if stamps == authority => epoch,
            _ => {
                let mut hasher = sha2::Sha256::new();
                hash_serialized(&mut hasher, &**self.configuration_sets)?;
                hasher.update([0xff]);
                hash_serialized(&mut hasher, &**self.design_management)?;
                hasher.update([0xff]);
                hash_serialized(&mut hasher, &**self.connectivity)?;
                let epoch = ContentDigest::from_bytes(hasher.finalize().into());
                *self.cache.authority.borrow_mut() = Some((authority, epoch));
                epoch
            }
        };

        let mut remembered = self.cache.cell_views.borrow_mut();
        let mut cell_views = BTreeMap::new();
        let mut stamps = HashMap::new();
        for (cell_view_key, source) in self.cell_view_sources(&active_key, active) {
            let digest = match remembered.get(cell_view_key) {
                Some((stamp, digest)) if stamp.matches(&source) => *digest,
                _ => {
                    let digest = cell_view_digest(&source)?;
                    remembered.insert(
                        cell_view_key.to_owned(),
                        (CellViewStamp::of(&source), digest),
                    );
                    digest
                }
            };
            cell_views.insert(cell_view_key.to_owned(), digest);
            stamps.insert(cell_view_key.to_owned(), CellViewStamp::of(&source));
        }
        remembered.retain(|cell_view_key, _| cell_views.contains_key(cell_view_key));

        let inputs = Arc::new(DesignInputs {
            authority,
            authority_epoch,
            design_epoch: DesignInputs::fold(authority_epoch, &cell_views),
            cell_views,
            stamps,
            active_key,
        });
        *self.cache.inputs.borrow_mut() = Some(Arc::clone(&inputs));
        Some(inputs)
    }
}
