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
//! So each cell view and each authority is digested, once per call, and the
//! digests key both the projection and the per-cell-view materialization.
//! Digesting is a fraction of what it replaces — one hierarchy walk plus a
//! design-management projection and a deep clone per cell view — and one edit
//! moves exactly one cell-view digest. A project whose authorities cannot be
//! serialized bypasses the cache instead of risking a stale answer.

use std::any::Any;
use std::sync::{Arc, Mutex, PoisonError};

use sha2::Digest as _;

use super::*;

#[cfg(test)]
mod tests;

/// Frozen live-buffer projection paired with its configuration plan. Holding
/// both in one value prevents a caller from resolving one hierarchy and
/// accidentally netlisting a different editor buffer.
///
/// The value is immutable once built. Callers share it through an [`Arc`], so
/// a projection that is still current costs a reference count rather than a
/// hierarchy walk.
pub struct DesignProjection {
    root: CellViewRef,
    schematic_buffers: HashMap<String, SchematicState>,
    plan: Option<ConfigurationExecutionPlan>,
    connectivity: crate::state::ConnectivityContract,
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

    pub fn root_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&self.root.key()))
            .map(|(_, schematic)| schematic)
    }

    pub const fn schematic_buffers(&self) -> &HashMap<String, SchematicState> {
        &self.schematic_buffers
    }

    /// The execution plan this projection was resolved into.
    ///
    /// Every projection carries one, configured or not: a configuration decides
    /// which view each occurrence binds to, never whether the hierarchy has an
    /// execution authority at all. The option survives only because several
    /// surfaces still branch on it; each of those branches now selects between
    /// identical outcomes.
    pub const fn plan(&self) -> Option<&ConfigurationExecutionPlan> {
        self.plan.as_ref()
    }

    pub const fn connectivity(&self) -> &crate::state::ConnectivityContract {
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
    project_revision: crate::product::ObjectRevision,
}

/// The digested inputs, retained across one call so the projection key and
/// every per-cell-view memo key are read from the same measurement.
struct DesignInputs {
    /// Digest over the configuration, design-management, and connectivity
    /// authorities.
    authority_epoch: ContentDigest,
    /// Content digest per cell view, keyed exactly as the projection keys its
    /// buffers and ordered so the fold over them is deterministic.
    cell_views: BTreeMap<String, ContentDigest>,
    /// Buffer spelling the live editor overlay occupies.
    active_key: String,
}

impl DesignInputs {
    fn design_epoch(&self) -> ContentDigest {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.authority_epoch.as_bytes());
        for (cell_view_key, digest) in &self.cell_views {
            hasher.update((cell_view_key.len() as u64).to_le_bytes());
            hasher.update(cell_view_key.as_bytes());
            hasher.update(digest.as_bytes());
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    fn memo_key(&self, cell_view_key: &str) -> Option<BufferMemoKey> {
        Some(BufferMemoKey {
            source_epoch: *self.cell_views.get(cell_view_key)?,
            authority_epoch: self.authority_epoch,
        })
    }
}

/// Identity of one materialized cell view: the source document it was
/// projected from, and the design-management authority that projected it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BufferMemoKey {
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
fn cell_view_digest(schematic: &SchematicState) -> Option<ContentDigest> {
    let mut hasher = sha2::Sha256::new();
    hash_serialized(&mut hasher, schematic)?;
    hasher.update([0xff]);
    hash_serialized(&mut hasher, &schematic.current_file)?;
    hasher.update([u8::from(schematic.read_only)]);
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
#[cfg(test)]
thread_local! {
    static MATERIALIZATIONS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) fn reset_materialization_count() {
    MATERIALIZATIONS.with(|count| count.set(0));
}

#[cfg(test)]
pub(super) fn materialization_count() -> u64 {
    MATERIALIZATIONS.with(std::cell::Cell::get)
}

impl ProjectWorkspace {
    /// The design as the configured hierarchy executes it, built once per
    /// change to anything it derives from.
    ///
    /// The returned value is shared and immutable; callers that need to
    /// compare two projections compare the pointers.
    pub fn design_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
    ) -> Result<Arc<DesignProjection>, ConfigurationExecutionPlanError> {
        let inputs = self.design_inputs(active_reference, active_schematic);
        let key = inputs.as_ref().map(|inputs| DesignProjectionKey {
            root: self.simulation_root_reference(),
            active_key: inputs.active_key.clone(),
            design_epoch: inputs.design_epoch(),
            library_revision: libraries.revision(),
            project_revision: self.project.revision(),
        });
        if let Some(key) = key.as_ref() {
            let cached = self.design_projection_cache.borrow();
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
            inputs.as_ref(),
            key,
        )?);
        if projection.key.is_some() {
            *self.design_projection_cache.borrow_mut() = Some(Arc::clone(&projection));
        }
        Ok(projection)
    }

    /// Freeze the live editor projection and its exact-path execution plan as
    /// one immutable value. A project with no configuration set seals the same
    /// kind of plan from the placed bindings, so every caller reads one shape.
    pub fn configuration_execution_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        self.design_projection(libraries, active_reference, active_schematic)
    }

    fn build_design_projection(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
        inputs: Option<&DesignInputs>,
        key: Option<DesignProjectionKey>,
    ) -> Result<DesignProjection, ConfigurationExecutionPlanError> {
        let root = self.simulation_root_reference();
        let configured = self.configuration_sets.active().is_some();
        let (resolution, plan) =
            HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
                .resolve_all();
        // Only a configuration blocks on an unresolved hierarchy. Without one,
        // an unresolved cell is reported by the generator against the instance
        // that could not be netlisted, which is the behaviour every legacy
        // project already relies on.
        if configured && !resolution.is_valid() {
            let diagnostics = resolution
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

        // The live editor buffer replaces its persisted copy under whichever
        // spelling the workspace already holds, so a case-different tab cannot
        // introduce a second master for the same cell view.
        let active_key = self.active_buffer_key(active_reference);
        let mut schematic_buffers = HashMap::with_capacity(self.schematic_buffers.len() + 1);
        for (cell_view_key, schematic) in &self.schematic_buffers {
            if *cell_view_key == active_key {
                continue;
            }
            let memo_key = inputs.and_then(|inputs| inputs.memo_key(cell_view_key));
            let materialized = self.materialized_cell_view(cell_view_key, schematic, memo_key)?;
            schematic_buffers.insert(cell_view_key.clone(), (*materialized).clone());
        }
        let memo_key = inputs.and_then(|inputs| inputs.memo_key(&active_key));
        let materialized = self.materialized_cell_view(&active_key, active_schematic, memo_key)?;
        schematic_buffers.insert(active_key, (*materialized).clone());
        self.materialized_buffers
            .borrow_mut()
            .retain(|cell_view_key, _| schematic_buffers.contains_key(cell_view_key));

        let projection = DesignProjection {
            root,
            schematic_buffers,
            plan: Some(plan),
            connectivity: self.connectivity.clone(),
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

    /// One cell view's design-management projection, reused while its source
    /// document and the design-management authority both stand still. This is
    /// what keeps a single component edit from re-materializing the whole
    /// design.
    fn materialized_cell_view(
        &self,
        cell_view_key: &str,
        source: &SchematicState,
        memo_key: Option<BufferMemoKey>,
    ) -> Result<Arc<SchematicState>, ConfigurationExecutionPlanError> {
        if let Some(memo_key) = memo_key.as_ref() {
            let memo = self.materialized_buffers.borrow();
            if let Some((cached_key, cached)) = memo.get(cell_view_key)
                && cached_key == memo_key
            {
                return Ok(Arc::clone(cached));
            }
        }
        #[cfg(test)]
        MATERIALIZATIONS.with(|count| count.set(count.get().wrapping_add(1)));
        let materialized = Arc::new(
            self.materialize_design_management_schematic(cell_view_key, source)
                .map_err(|error| {
                    ConfigurationExecutionPlanError::DesignManagement(error.to_string())
                })?,
        );
        if let Some(memo_key) = memo_key {
            self.materialized_buffers.borrow_mut().insert(
                cell_view_key.to_owned(),
                (memo_key, Arc::clone(&materialized)),
            );
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

    /// Digest every input the projection is derived from. `None` means one of
    /// them could not be serialized, which the caller reads as "this
    /// projection cannot be memoized".
    fn design_inputs(
        &self,
        active_reference: &CellViewRef,
        active_schematic: &SchematicState,
    ) -> Option<DesignInputs> {
        let mut hasher = sha2::Sha256::new();
        hash_serialized(&mut hasher, &self.configuration_sets)?;
        hasher.update([0xff]);
        hash_serialized(&mut hasher, &self.design_management)?;
        hasher.update([0xff]);
        hash_serialized(&mut hasher, &self.connectivity)?;
        let authority_epoch = ContentDigest::from_bytes(hasher.finalize().into());

        let active_key = self.active_buffer_key(active_reference);
        let mut cell_views = BTreeMap::new();
        for (cell_view_key, schematic) in &self.schematic_buffers {
            if *cell_view_key == active_key {
                continue;
            }
            cell_views.insert(cell_view_key.clone(), cell_view_digest(schematic)?);
        }
        cell_views.insert(active_key.clone(), cell_view_digest(active_schematic)?);

        Some(DesignInputs {
            authority_epoch,
            cell_views,
            active_key,
        })
    }
}
