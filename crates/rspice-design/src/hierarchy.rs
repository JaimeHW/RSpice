//! Resolving a testbench hierarchy to exact Library/Cell/View bindings.
//!
//! Resolution walks the placed instances and answers one question per row:
//! which view will actually be netlisted, and did the configuration say so or
//! did the search order fall back?  A row records that distinction rather than
//! hiding it, so an unresolved or fallback binding can never be mistaken for a
//! configured one.  Nothing here mutates the design — the resolver borrows
//! its inputs and returns a [`HierarchyResolution`].
//!
//! The walk starts at [`InstancePath::root`], which owns no segment: the design
//! root is implicit, so a resolved path names instances only and re-rooting the
//! top cell cannot invalidate one. Every path this module carries is an
//! [`InstancePath`]; the authored spellings a configuration stores are parsed
//! once, where they enter, so a definition written before the root became
//! implicit selects the same instances as a migrated one.
//!
//! Resolution also decides master identity, because that is a property of what
//! resolved rather than of who emits it. Every occurrence carries a digest over
//! its whole binding closure, occurrences with equal digests are one master,
//! and each master is named once here. A generator that named masters itself
//! would be a second answer to the same question.

use crate::configuration_set::{
    BindingTarget, ConfigurationCellBinding, ConfigurationPlatform, ConfigurationSet,
    ConfigurationSetId, ConfigurationSetOverride, UnresolvedBindingPolicy, cell_binding_key,
};
use crate::library::{Cell, Library, LibraryCatalog, View, ViewType};
use crate::project_sources::{ProjectSourceId, ProjectSourceOwner, ProjectSourceRegistry};
use crate::schematic::component::LibraryCellInstance;
use crate::schematic::component_type::ComponentType;
use crate::schematic::device_catalog::validate_builtin_xspice_binding;
use crate::schematic::document::SchematicDocument;
use crate::schematic::functional_catalog::validate_builtin_functional_binding;
use crate::schematic::generated_veriloga_catalog::validate_generated_veriloga_binding;
use crate::schematic::port::PortSpec;
use rspice_design_model::cell_view::{CellViewRef, DEFAULT_SCHEMATIC_VIEW};
use rspice_vocabulary::hierarchy_path::{
    HierarchyPathError, InstancePath, InstancePathPattern, PatternSegment,
};
use rspice_vocabulary::product::{ContentDigest, ProjectId};
use sha2::Digest as _;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

mod bill_of_materials;
mod materialize;
mod occurrences;
pub mod port_fit;
mod source;
use materialize::*;
pub use materialize::{
    DEFAULT_STOP_VIEW_TYPES, SourceViewSource, hierarchy_stop_view,
    materialize_authoritative_source_binding, metadata_value, project_veriloga_binding_for_view,
    source_view_ports, source_view_source,
};
pub use occurrences::{HierarchyOccurrences, OccurrenceBinding, PlanBindings, RowOccurrences};
use occurrences::{OccurrenceEdge, OccurrenceNode, fold};
pub use source::HierarchySource;

#[cfg(test)]
mod binding_target_tests;
#[cfg(test)]
mod tests;

/// Maximum legal hierarchy depth. This is deliberately generous for real
/// designs while placing a deterministic bound on corrupt or hostile project
/// data before it reaches netlisting.
const MAX_HIERARCHY_RESOLUTION_DEPTH: usize = 128;

/// One borrowed document and its modification state for hierarchy review.
#[derive(Clone, Copy)]
pub struct HierarchySchematic<'a> {
    pub document: &'a SchematicDocument,
    pub modified: bool,
}

/// Document lookup over the caller's selected immutable design projection.
pub trait HierarchyDocuments {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>>;
}

/// Host operations needed to validate a filesystem-backed source binding.
pub trait HierarchySourceFiles {
    fn source_paths_match(&self, left: &Path, right: &Path) -> bool;
    fn configured_source_identity(&self, path: &Path) -> String;
    fn validate_source_file(
        &self,
        source_path: &Path,
        view_type: ViewType,
        binding: &LibraryCellInstance,
    ) -> Result<(), String>;
}

/// Borrowed authorities used by one non-mutating hierarchy resolution.
pub struct HierarchyContext<'a, D, F> {
    pub documents: &'a D,
    pub libraries: &'a LibraryCatalog,
    pub project_id: ProjectId,
    pub project_sources: &'a ProjectSourceRegistry,
    pub source_files: &'a F,
}

/// The built-in view search order for one requested view: what resolution
/// follows when no configuration is active, and what reviewed fallback
/// appends after a configured list.
pub fn default_view_search_order(requested: &str, is_root: bool) -> Vec<String> {
    hierarchy_view_search_order(requested, is_root)
}

pub fn resolve_hierarchy<'a, D: HierarchyDocuments, F: HierarchySourceFiles>(
    context: HierarchyContext<'a, D, F>,
    root: CellViewRef,
    configuration: Option<&'a ConfigurationSet>,
) -> (HierarchyResolution, ConfigurationExecutionPlan) {
    HierarchyResolver::new(context, root, configuration).resolve_all()
}

/// One immutable, exact-path executable binding consumed by hierarchical
/// netlist generation.  The placed schematic binding is deliberately not
/// retained as execution authority: `materialized_binding` is rebuilt from
/// the resolved Library/Cell/View and its authoritative view metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionBinding {
    instance_path: InstancePath,
    resolved_reference: CellViewRef,
    resolved_view_type: ViewType,
    materialized_binding: Option<LibraryCellInstance>,
    model_section: Option<String>,
    stop_boundary: bool,
    project_veriloga: Option<ConfigurationVerilogABinding>,
    /// Digest over everything this occurrence and its whole descendant subtree
    /// resolve to. Two occurrences whose subtrees resolve identically carry
    /// equal digests and therefore share one emitted master; two that differ
    /// anywhere below them do not.
    binding_closure_digest: ContentDigest,
}

/// Exact project-owned behavioral source selected for one configuration
/// binding. This is derived from the active configuration and source registry;
/// it is never accepted from placed-instance or filesystem metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationVerilogABinding {
    source_bundle_id: ProjectSourceId,
    source_closure_digest: ContentDigest,
    selected_module: String,
    source_key: String,
    netlist_alias: String,
}

impl ConfigurationVerilogABinding {
    pub const fn source_bundle_id(&self) -> ProjectSourceId {
        self.source_bundle_id
    }

    pub const fn source_closure_digest(&self) -> ContentDigest {
        self.source_closure_digest
    }

    pub fn selected_module(&self) -> &str {
        &self.selected_module
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }
}

impl ConfigurationExecutionBinding {
    pub const fn instance_path(&self) -> &InstancePath {
        &self.instance_path
    }

    pub const fn resolved_reference(&self) -> &CellViewRef {
        &self.resolved_reference
    }

    pub const fn resolved_view_type(&self) -> ViewType {
        self.resolved_view_type
    }

    pub const fn materialized_binding(&self) -> Option<&LibraryCellInstance> {
        self.materialized_binding.as_ref()
    }

    pub fn model_section(&self) -> Option<&str> {
        self.model_section.as_deref()
    }

    pub const fn stop_boundary(&self) -> bool {
        self.stop_boundary
    }

    pub const fn project_veriloga(&self) -> Option<&ConfigurationVerilogABinding> {
        self.project_veriloga.as_ref()
    }

    /// Digest over this occurrence's whole resolved subtree. Two occurrences
    /// carry the same digest exactly when they instantiate the same master —
    /// which is what [`ConfigurationExecutionPlan::occurrence_master`] answers
    /// without re-deriving it.
    pub const fn binding_closure_digest(&self) -> ContentDigest {
        self.binding_closure_digest
    }
}

/// Revision of the authority governing an unconfigured resolution. Nothing has
/// ever revised the absence of a configuration.
const UNCONFIGURED_PLAN_REVISION: u64 = 0;

/// Domain tag digested for the authority of an unconfigured resolution, so
/// every such plan reports one digest rather than borrowing a configuration's.
const UNCONFIGURED_PLAN_AUTHORITY: &str = "rspice.hierarchy.plan.unconfigured";

/// Separator between a master's stem and the qualifier that disambiguates it.
const MASTER_QUALIFIER: &str = "__";

/// Identity of one emitted subcircuit master: which Library/Cell/View it is,
/// and what its whole binding closure resolved to.
///
/// The folded reference is stored rather than derived so ordering — folded
/// reference first, then closure — costs no allocation, and so two spellings
/// of one cell view can never key two masters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MasterKey {
    folded_reference: String,
    closure: ContentDigest,
}

impl MasterKey {
    pub fn new(reference: &CellViewRef, closure: ContentDigest) -> Self {
        Self {
            folded_reference: reference.key().to_ascii_lowercase(),
            closure,
        }
    }
}

/// One master as the deck declares it: the name it is emitted under.
///
/// Which cell view it came from is carried by the [`MasterKey`] that addresses
/// it, and which occurrences instantiate it by the plan's occurrence-to-master
/// map. Copying either in here would be a second answer to a question the plan
/// already owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterRecord {
    name: String,
}

impl MasterRecord {
    fn new(name: String) -> Self {
        Self { name }
    }

    /// The `.subckt` name this master is declared and instantiated under.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Frozen per-instance hierarchy authority for one resolution of the design.
/// Keys are [`InstancePath::fold_key`] values; the bindings retain the
/// design's own spelling for receipts and diagnostics.
///
/// A resolution produces a plan whether or not a configuration set is active:
/// the configuration decides *which* view each occurrence binds to, never
/// whether the hierarchy has an execution authority at all. An unconfigured
/// resolution therefore carries no configuration identity, revision `0`, and
/// the digest of the empty authority — one value shared by every unconfigured
/// plan, because they are all governed by the same absence.
///
/// The plan holds the resolved hierarchy as the graph of its distinct
/// contexts, so it is as large as the design's cells and placements, not as
/// large as its expansion; an occurrence's binding is found by its path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionPlan {
    root: CellViewRef,
    occurrences: Arc<HierarchyOccurrences>,
    masters: BTreeMap<MasterKey, MasterRecord>,
    configuration_id: Option<ConfigurationSetId>,
    configuration_revision: u64,
    configuration_digest: ContentDigest,
}

impl ConfigurationExecutionPlan {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    /// What runs at one occurrence. Every occurrence of one context shares
    /// the binding, whose own path is the first of them the design places.
    pub fn binding(&self, instance_path: &InstancePath) -> Option<&ConfigurationExecutionBinding> {
        let node = self.occurrences.node_at(instance_path)?;
        self.occurrences.nodes[node].binding.as_ref()
    }

    /// Every occurrence that runs something, with its own path, in the order
    /// the design places them. Counting them costs nothing; walking them
    /// costs what they are.
    pub fn bindings(&self) -> PlanBindings<'_> {
        PlanBindings::new(&self.occurrences)
    }

    /// Each distinct binding once, in the order the design first places it,
    /// named by that first occurrence's path.
    pub fn distinct_bindings(&self) -> impl Iterator<Item = &ConfigurationExecutionBinding> {
        self.occurrences
            .first_reached()
            .into_iter()
            .filter_map(|node| self.occurrences.nodes[node].binding.as_ref())
    }

    /// Every master this plan declares, ordered by folded reference and then
    /// by closure so the deck's definition order is a property of the design
    /// rather than of the walk.
    pub fn masters(&self) -> impl ExactSizeIterator<Item = (&MasterKey, &MasterRecord)> {
        self.masters.iter()
    }

    pub fn master(&self, key: &MasterKey) -> Option<&MasterRecord> {
        self.masters.get(key)
    }

    /// The master one occurrence instantiates, when this plan governs that
    /// occurrence and it resolves to an emitted master.
    pub fn occurrence_master(&self, instance_path: &InstancePath) -> Option<&MasterKey> {
        let node = self.occurrences.node_at(instance_path)?;
        self.occurrences.nodes[node].master.as_ref()
    }

    pub const fn configuration_id(&self) -> Option<ConfigurationSetId> {
        self.configuration_id
    }

    pub const fn configuration_revision(&self) -> u64 {
        self.configuration_revision
    }

    pub const fn configuration_digest(&self) -> ContentDigest {
        self.configuration_digest
    }
}

/// Digest over one occurrence's whole binding closure.
///
/// Post-order by construction: the caller passes its children's digests, so two
/// occurrences agree only when their entire subtrees resolve identically. The
/// children are sorted before folding, because the order a walk happens to
/// reach siblings in is not part of what the master is.
pub fn master_closure_digest(
    reference: &CellViewRef,
    view_type: ViewType,
    model_section: Option<&str>,
    stop_boundary: bool,
    project_veriloga: Option<ContentDigest>,
    children: &[ContentDigest],
) -> ContentDigest {
    fn absorb(hasher: &mut sha2::Sha256, value: &str) {
        hasher.update((value.len() as u64).to_le_bytes());
        hasher.update(value.as_bytes());
    }

    let mut hasher = sha2::Sha256::new();
    absorb(&mut hasher, &reference.key().to_ascii_lowercase());
    absorb(&mut hasher, view_type.display_name());
    absorb(
        &mut hasher,
        &model_section.unwrap_or_default().to_ascii_lowercase(),
    );
    hasher.update([u8::from(model_section.is_some())]);
    hasher.update([u8::from(stop_boundary)]);
    match project_veriloga {
        Some(digest) => {
            hasher.update([1_u8]);
            hasher.update(digest.as_bytes());
        }
        None => hasher.update([0_u8]),
    }
    let mut ordered = children.to_vec();
    ordered.sort_unstable();
    hasher.update((ordered.len() as u64).to_le_bytes());
    for child in ordered {
        hasher.update(child.as_bytes());
    }
    ContentDigest::from_bytes(hasher.finalize().into())
}

/// The deck spelling of one identifier: SPICE names are ASCII, so anything
/// else folds to an underscore rather than reaching the engine unreadable.
fn master_stem(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

/// Name every master, in first-occurrence order, so a deck names the same
/// cell the same way twice.
///
/// The base name is the cell. The first master of a cell name owns it; a later
/// master of that name from another library is qualified by its library, and a
/// later master of the same library/cell — a second binding closure — takes the
/// next `__vN`. `reserved` holds names already spoken for by model-bound
/// masters, which are engine modules this pass may not shadow.
pub fn assign_master_names<'a>(
    ordered: impl IntoIterator<Item = (&'a MasterKey, &'a CellViewRef)>,
    reserved: &HashSet<String>,
) -> BTreeMap<MasterKey, String> {
    let mut taken = reserved.clone();
    let mut stem_owner = HashMap::<String, String>::new();
    let mut names = BTreeMap::new();
    for (key, reference) in ordered {
        if names.contains_key(key) {
            continue;
        }
        let stem = master_stem(&reference.cell);
        let library = reference.library.to_ascii_lowercase();
        let owner = stem_owner
            .entry(stem.to_ascii_lowercase())
            .or_insert_with(|| library.clone());
        let base = if *owner == library {
            stem
        } else {
            format!(
                "{stem}{MASTER_QUALIFIER}{}",
                master_stem(&reference.library)
            )
        };
        let mut candidate = base.clone();
        let mut ordinal = 2_u32;
        while !taken.insert(candidate.to_ascii_lowercase()) {
            candidate = format!("{base}{MASTER_QUALIFIER}v{ordinal}");
            ordinal += 1;
        }
        names.insert(key.clone(), candidate);
    }
    names
}

/// One resolved occurrence, before its subtree is known.
///
/// The closure digest cannot be computed until every descendant has resolved,
/// so the walk stages this and [`seal_plan`] turns the whole staging map into
/// bindings at once. A binding never exists with a digest that does not yet
/// describe it.
struct PendingExecutionBinding {
    instance_path: InstancePath,
    resolved_reference: CellViewRef,
    resolved_view_type: ViewType,
    materialized_binding: Option<LibraryCellInstance>,
    model_section: Option<String>,
    stop_boundary: bool,
    project_veriloga: Option<ConfigurationVerilogABinding>,
}

/// Resolution state for one grouped library/cell/view binding in the complete
/// testbench hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyBindingStatus {
    Resolved,
    Modified,
    Unresolved,
    /// The configuration binds the instance to another cell whose pins do
    /// not fit the ones it places.
    Incompatible,
    Recursive,
    DepthLimit,
}

impl HierarchyBindingStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Modified => "modified",
            Self::Unresolved => "unresolved",
            Self::Incompatible => "incompatible",
            Self::Recursive => "recursive",
            Self::DepthLimit => "depth limit",
        }
    }

    pub fn is_resolved(self) -> bool {
        matches!(self, Self::Resolved | Self::Modified)
    }

    pub fn is_modified(self) -> bool {
        self == Self::Modified
    }

    fn severity(self) -> u8 {
        match self {
            Self::Resolved => 0,
            Self::Modified => 1,
            Self::Unresolved => 2,
            Self::Incompatible => 3,
            Self::Recursive => 4,
            Self::DepthLimit => 5,
        }
    }
}

/// Which ordered view list chose a row's view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyBindingRule {
    /// No configuration is active; the built-in search order chose.
    Default,
    /// The active configuration's global view list.
    ViewList,
    /// A cell binding for the instance's master library and cell.
    CellBinding,
    /// An override naming the instance itself.
    InstanceBinding,
}

/// One row in the resolved hierarchy-binding manifest. Repeated masters are
/// grouped while `instance_count` retains their exact expanded multiplicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHierarchyBinding {
    pub reference: CellViewRef,
    /// The cell view the instances place, when the configuration runs
    /// another cell in their place; `None` when `reference` is of the cell
    /// they place.
    pub placed: Option<CellViewRef>,
    /// The pins the instances place, in the order they place them: what a
    /// cell bound in their place has to fit. Empty for the design root.
    pub placed_ports: Vec<PortSpec>,
    pub purpose: String,
    pub view_search_order: Vec<String>,
    pub stop_view: Option<String>,
    pub model_section: String,
    pub status: HierarchyBindingStatus,
    pub instance_count: usize,
    /// The expanded occurrences this grouped binding stands for: their paths
    /// are walked when asked for, never held, so a cell placed a million
    /// times costs one row.
    pub occurrences: RowOccurrences,
    /// True when the active configuration explicitly permits and records a
    /// reviewed fallback outside its primary ordered view policy.
    pub used_review_fallback: bool,
    /// The ordered view list that chose this row's view.
    pub rule: HierarchyBindingRule,
    pub diagnostic: Option<String>,
    /// Configured intent this row honours differently than it reads.
    ///
    /// A warning never changes `status`: the binding is resolved and will be
    /// netlisted. It exists because a configuration can ask for something the
    /// selected view cannot deliver — a stop at a schematic, which has no
    /// source to stand in for the cell — and silently doing the other thing
    /// would leave the author reading a policy the hierarchy does not follow.
    pub warnings: Vec<String>,
}

/// Immutable resolution receipt for the project configuration surface and
/// preflight diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyResolution {
    pub bindings: Vec<ResolvedHierarchyBinding>,
    /// Every occurrence as the graph of the contexts it resolved in: the
    /// tree a page opens an instance of at a time.
    pub occurrences: Arc<HierarchyOccurrences>,
    pub total_instances: usize,
    pub resolved_instances: usize,
    pub configuration_id: Option<ConfigurationSetId>,
    pub configuration_revision: Option<u64>,
    pub configuration_digest: Option<ContentDigest>,
}

impl HierarchyResolution {
    pub fn unresolved_instances(&self) -> usize {
        self.total_instances.saturating_sub(self.resolved_instances)
    }

    pub fn is_valid(&self) -> bool {
        self.unresolved_instances() == 0
    }
}
struct HierarchyResolver<'a, D, F> {
    context: HierarchyContext<'a, D, F>,
    root: CellViewRef,
    configuration: Option<&'a ConfigurationSet>,
    /// The active configuration's overrides with their patterns canonicalized,
    /// so selection compares one grammar against itself.
    overrides: Vec<ConfigurationSetOverride>,
    /// The active configuration's cell bindings, keyed by the folded cell
    /// each one names.
    cell_bindings: HashMap<(String, String), &'a ConfigurationCellBinding>,
    /// Each override's pattern, parsed once.
    patterns: Vec<Option<InstancePathPattern>>,
    /// The overrides whose pattern names every position, by the folded path
    /// it names, so a lookup tries only those naming the instance; the rest
    /// hold a wildcard and are tried against every instance.
    named_overrides: HashMap<String, Vec<usize>>,
    wildcard_overrides: Vec<usize>,
    rows: Vec<ResolvedHierarchyBinding>,
    row_indices: HashMap<String, usize>,
    /// One node per distinct occurrence context, in the order the walk made
    /// them; the design root is the first.
    nodes: Vec<OccurrenceNode>,
    /// What each node runs, before its subtree's digest is known.
    pending: Vec<Option<PendingExecutionBinding>>,
    /// The node each context resolved to, so a second occurrence of it is
    /// not resolved again.
    contexts: HashMap<OccurrenceContext, usize>,
    /// Row entries no node holds, with the path each names.
    listed: Vec<(usize, InstancePath)>,
}

/// Everything an occurrence's resolution depends on besides its path: what
/// it places, where its placement is drawn, the cells above it, and which
/// overrides can still reach it or an instance under it. Two occurrences
/// alike in all of these resolve alike, down to every instance under them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OccurrenceContext {
    requested: String,
    /// The drawn placement, by its address in the master that draws it.
    placement: usize,
    depth: usize,
    ancestors: String,
    overrides: Vec<usize>,
}

#[derive(Clone)]
struct HierarchyMaster<'a> {
    schematic: Option<HierarchySchematic<'a>>,
    view_type: Option<ViewType>,
    view_modified: bool,
    library_read_only: bool,
    library_has_technology: bool,
    materialized_binding: Option<LibraryCellInstance>,
}

struct BindingRowOutcome<'a> {
    master: Option<HierarchyMaster<'a>>,
    status: HierarchyBindingStatus,
    used_review_fallback: bool,
    diagnostic: Option<String>,
    /// The cell view the instance places, when the configuration ran
    /// another cell in its place: what its bindings are looked up by.
    placed: Option<CellViewRef>,
}

impl<'a, D: HierarchyDocuments, F: HierarchySourceFiles> HierarchyResolver<'a, D, F> {
    fn new(
        context: HierarchyContext<'a, D, F>,
        root: CellViewRef,
        configuration: Option<&'a ConfigurationSet>,
    ) -> Self {
        let patterns = configuration
            .map(|configuration| {
                configuration
                    .overrides()
                    .iter()
                    .map(|scoped| InstancePathPattern::parse(&scoped.instance_path).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut named_overrides = HashMap::<String, Vec<usize>>::new();
        let mut wildcard_overrides = Vec::new();
        for (at, pattern) in patterns.iter().enumerate() {
            let Some(pattern) = pattern else {
                continue;
            };
            match named_path_key(pattern) {
                Some(key) => named_overrides.entry(key).or_default().push(at),
                None => wildcard_overrides.push(at),
            }
        }
        Self {
            context,
            root,
            configuration,
            overrides: configuration
                .map(|configuration| {
                    configuration
                        .overrides()
                        .iter()
                        .map(canonical_override)
                        .collect()
                })
                .unwrap_or_default(),
            cell_bindings: configuration
                .map(|configuration| {
                    configuration
                        .cell_bindings()
                        .iter()
                        .map(|binding| (cell_binding_key(&binding.library, &binding.cell), binding))
                        .collect()
                })
                .unwrap_or_default(),
            patterns,
            named_overrides,
            wildcard_overrides,
            rows: Vec::new(),
            row_indices: HashMap::new(),
            nodes: Vec::new(),
            pending: Vec::new(),
            contexts: HashMap::new(),
            listed: Vec::new(),
        }
    }

    fn resolve_all(mut self) -> (HierarchyResolution, ConfigurationExecutionPlan) {
        let active_configuration = self.configuration;
        let root = self.root.clone();
        let required_paths = active_configuration
            .map(|configuration| {
                let mut paths = vec![(
                    configuration.dut_path().to_owned(),
                    "configured DUT path".to_owned(),
                )];
                paths.extend(configuration.overrides().iter().map(|scoped| {
                    (
                        scoped.instance_path.clone(),
                        "scoped configuration override".to_owned(),
                    )
                }));
                paths
            })
            .unwrap_or_default();
        let design_root = InstancePath::root();
        let mut ancestors = Vec::new();
        if let Some(error) = active_configuration
            .and_then(|configuration| validate_override_pattern_authority(configuration).err())
        {
            let row = self.binding_row(
                root.clone(),
                None,
                &design_root,
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.list(row, design_root.clone());
        } else {
            let reaching = (0..self.patterns.len())
                .filter(|at| self.patterns[*at].is_some())
                .collect::<Vec<_>>();
            self.resolve_reference(
                root.clone(),
                None,
                &design_root,
                0,
                true,
                &mut ancestors,
                "",
                &reaching,
            );
        }
        let mut graph = HierarchyOccurrences {
            nodes: std::mem::take(&mut self.nodes),
            listed: Vec::new(),
        };
        graph.seal();
        let mut reported = ReportedPaths::default();
        for (text, purpose) in required_paths {
            let Some((occurrence, diagnostic)) =
                unmet_requirement(&graph, &mut reported, &text, &purpose)
            else {
                continue;
            };
            let row = self.binding_row(
                root.clone(),
                None,
                &occurrence,
                1,
                false,
                (HierarchyBindingStatus::Unresolved, Some(diagnostic)),
            );
            self.list(row, occurrence);
        }
        if let Some(error) = self.execution_model_section_conflict() {
            let row = self.binding_row(
                root.clone(),
                None,
                &design_root,
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.list(row, design_root);
        }
        graph.listed = std::mem::take(&mut self.listed);
        let masters = seal_plan(&mut graph, std::mem::take(&mut self.pending));
        let graph = Arc::new(graph);
        let mut total_instances = 0_usize;
        for (index, row) in self.rows.iter_mut().enumerate() {
            row.instance_count = graph.row_count(index);
            row.occurrences = RowOccurrences::new(Arc::clone(&graph), index);
            total_instances = total_instances.saturating_add(row.instance_count);
        }
        let resolved_instances = graph
            .nodes
            .iter()
            .filter(|node| node.resolved)
            .fold(0_usize, |total, node| {
                total.saturating_add(node.multiplicity)
            });
        let resolution = HierarchyResolution {
            bindings: self.rows,
            occurrences: Arc::clone(&graph),
            total_instances,
            resolved_instances,
            configuration_id: active_configuration.map(|configuration| configuration.id()),
            configuration_revision: active_configuration
                .map(|configuration| configuration.revision()),
            configuration_digest: active_configuration
                .map(|configuration| configuration.semantic_digest()),
        };
        let plan = ConfigurationExecutionPlan {
            root,
            occurrences: graph,
            masters,
            configuration_id: active_configuration.map(ConfigurationSet::id),
            configuration_revision: active_configuration
                .map_or(UNCONFIGURED_PLAN_REVISION, |configuration| {
                    configuration.revision()
                }),
            configuration_digest: active_configuration.map_or_else(
                || {
                    ContentDigest::from_bytes(
                        sha2::Sha256::digest(UNCONFIGURED_PLAN_AUTHORITY.as_bytes()).into(),
                    )
                },
                |configuration| configuration.semantic_digest(),
            ),
        };
        (resolution, plan)
    }

    /// The overrides of `reaching` that can still reach an instance named
    /// `name` placed at `depth`, or one under it: a pattern matches at one
    /// depth only, so it reaches on while each position it pins matches.
    fn reaching_below(&self, reaching: &[usize], depth: usize, name: &str) -> Vec<usize> {
        reaching
            .iter()
            .copied()
            .filter(|at| {
                self.patterns[*at].as_ref().is_some_and(|pattern| {
                    pattern
                        .segments()
                        .get(depth)
                        .is_some_and(|segment| segment_matches(segment, name))
                })
            })
            .collect()
    }

    /// Resolve the occurrence at `instance_path` into a new node, and each
    /// instance under it into the node its context resolves to, resolving a
    /// context the first time it is reached and only then. `reaching` holds
    /// the overrides that can reach this occurrence or one under it; with the
    /// cells above it, its depth and its placement, that is all that makes it
    /// resolve as it does, so the occurrence it was first reached at speaks
    /// for every other.
    #[allow(clippy::too_many_arguments)]
    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<CellViewRef>,
        above: &str,
        reaching: &[usize],
    ) -> usize {
        let node = self.nodes.len();
        self.nodes.push(OccurrenceNode::default());
        self.pending.push(None);

        if depth > MAX_HIERARCHY_RESOLUTION_DEPTH {
            let row = self.binding_row(
                requested,
                binding,
                instance_path,
                depth,
                is_root,
                (
                    HierarchyBindingStatus::DepthLimit,
                    Some(format!(
                        "hierarchy exceeds the supported depth of {MAX_HIERARCHY_RESOLUTION_DEPTH}"
                    )),
                ),
            );
            self.upsert(row, node);
            return node;
        }

        // A built-in device is the engine's, not a library cell: there is no
        // cell of the project's in whose place another could run, so a
        // binding naming one is refused here rather than ignored.
        if let Some(placed) = binding
            && placed.is_executable_builtin()
            && let Some(target) = self.configured_target(&requested, instance_path, is_root)
        {
            let diagnostic = format!(
                "{instance_path} is a built-in device, so it cannot be bound to {}; remove the binding",
                target.display_path()
            );
            let row = self.binding_row(
                requested,
                binding,
                instance_path,
                depth,
                is_root,
                (HierarchyBindingStatus::Incompatible, Some(diagnostic)),
            );
            self.upsert(row, node);
            return node;
        }

        // Compiled code models are executable leaves, not project library
        // masters.  Resolve their frozen catalog contract directly so an
        // active configuration cannot turn a valid placement into an
        // unresolved L/C/V reference or silently substitute another view.
        if let Some(binding) = binding
            && (binding.is_builtin_xspice() || binding.is_builtin_functional())
        {
            self.resolve_builtin_leaf(node, requested, binding, instance_path, depth, is_root);
            return node;
        }
        if let Some(binding) = binding
            && binding.is_generated_veriloga()
        {
            self.resolve_generated_veriloga(
                node,
                requested,
                binding,
                instance_path,
                depth,
                is_root,
            );
            return node;
        }

        // A binding to another cell searches that cell for the views the
        // placed one's bindings name: the view list stays the placement's,
        // the cell it is searched in is the target. The target stands in for
        // the placement it replaces, so nothing the placement copied from its
        // own cell (its pin order, its model card) reaches the target's.
        let target = self
            .configured_target(&requested, instance_path, is_root)
            .map(|target| CellViewRef::new(&target.library, &target.cell, &requested.view));
        let stand_in = target
            .as_ref()
            .map(|target| LibraryCellInstance::new(&target.library, &target.cell, &target.view));
        let placement = stand_in.as_ref().or(binding);
        let searched = target.as_ref().unwrap_or(&requested);
        let search_order =
            self.view_search_order(&requested, &requested.view, is_root, instance_path);
        let configured_model_section = self.configured_model_section(instance_path);
        let (master, resolution_error) = match self.resolve_master(
            searched,
            placement,
            &search_order,
            configured_model_section.as_deref(),
        ) {
            Ok(master) => (master, None),
            Err(error) => (None, Some(error)),
        };
        let resolved_reference = master
            .as_ref()
            .and_then(|(_, reference)| reference.clone())
            .unwrap_or_else(|| searched.clone());
        let used_review_fallback = master.is_some()
            && self.used_review_fallback(
                &requested,
                &resolved_reference.view,
                is_root,
                instance_path,
            );
        let identity = hierarchy_identity(&resolved_reference);

        if let Some(cycle_start) = ancestors
            .iter()
            .position(|ancestor| hierarchy_identity(ancestor) == identity)
        {
            let chain = ancestors
                .iter()
                .skip(cycle_start)
                .chain(std::iter::once(&resolved_reference))
                .map(hierarchy_display_path)
                .collect::<Vec<_>>()
                .join(" → ");
            let row = self.binding_row_with_master(
                resolved_reference,
                binding,
                instance_path,
                depth,
                is_root,
                BindingRowOutcome {
                    master: master.map(|(master, _)| master),
                    status: HierarchyBindingStatus::Recursive,
                    used_review_fallback: false,
                    diagnostic: Some(format!("recursive hierarchy: {chain}")),
                    placed: target.as_ref().map(|_| requested.clone()),
                },
            );
            self.upsert(row, node);
            return node;
        }

        let (master, mut status, mut diagnostic) = match master {
            Some((master, _)) => {
                let modified = master.view_modified
                    || master.schematic.is_some_and(|schematic| schematic.modified)
                    || used_review_fallback;
                (
                    Some(master),
                    if modified {
                        HierarchyBindingStatus::Modified
                    } else {
                        HierarchyBindingStatus::Resolved
                    },
                    None,
                )
            }
            None => (
                None,
                HierarchyBindingStatus::Unresolved,
                resolution_error.or_else(|| {
                    Some(format!(
                        "{} has no {} view",
                        hierarchy_display_path(searched),
                        view_alternatives(&search_order)
                    ))
                }),
            ),
        };
        // The target is measured against the pins this occurrence places,
        // every time the hierarchy resolves: a cell that fitted when it was
        // bound and stopped fitting since binds nothing and says which pins.
        let mut warnings = Vec::new();
        if target.is_some()
            && status.is_resolved()
            && let Some(bound) = master
                .as_ref()
                .and_then(|master| master.materialized_binding.as_ref())
        {
            let fit = port_fit::port_fit(
                &placed_ports(binding),
                &bound.interface().unwrap_or_default(),
            );
            let cell = hierarchy_display_path(&resolved_reference);
            match fit.refusal() {
                Some(pins) => {
                    status = HierarchyBindingStatus::Incompatible;
                    diagnostic = Some(format!(
                        "{cell} no longer fits {instance_path}: {pins}; bind it to a cell that holds its pins, or remove the binding"
                    ));
                }
                None => warnings.extend(fit.unconnected_warning(&cell)),
            }
        }

        let current_platform = ConfigurationPlatform::current();
        if !self.configured_platform_eligible(instance_path, current_platform) {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} is not supported by this execution target ({})",
                current_platform.label()
            ));
        }
        if status.is_resolved()
            && self.configured_platform_declared(instance_path, ConfigurationPlatform::Browser)
            && master
                .as_ref()
                .and_then(|value| value.materialized_binding.as_ref())
                .and_then(|binding| binding.source_path.as_ref())
                .is_some_and(|path| !is_project_virtual_source_path(path))
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} declares Browser eligibility, but its filesystem-backed source is unavailable in this browser session"
            ));
        }
        let model_section = configured_model_section.or_else(|| {
            master
                .as_ref()
                .and_then(|master| master.materialized_binding.as_ref())
                .and_then(|binding| binding.model_section.clone())
        });
        if let Some(section) = model_section.as_deref()
            && status.is_resolved()
            && master
                .as_ref()
                .and_then(|value| value.view_type)
                .is_some_and(|view_type| {
                    !matches!(view_type, ViewType::Spice | ViewType::Extracted)
                })
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} requires a source-backed SPICE or extracted view"
            ));
        }
        let project_veriloga = if status.is_resolved()
            && master
                .as_ref()
                .and_then(|value| value.view_type)
                .is_some_and(|view_type| view_type.is_hdl_source())
        {
            match project_veriloga_binding_for_view(
                self.context.project_id,
                self.context.project_sources,
                self.context.libraries,
                &resolved_reference,
            ) {
                Ok(binding) => Some(binding),
                Err(error) => {
                    status = HierarchyBindingStatus::Unresolved;
                    diagnostic = Some(error);
                    None
                }
            }
        } else {
            None
        };

        let resolved_view_type = master.as_ref().and_then(|value| value.view_type);
        let matched_stop =
            self.matched_stop_view(&requested, &resolved_reference.view, instance_path);
        // A stop is executable only when the selected view is itself a
        // materialized terminal implementation. Treating a schematic as a black
        // box would emit an X-instance without any defining source, so the walk
        // descends and says so instead.
        let stop_boundary =
            matched_stop.is_some() && resolved_view_type.is_some_and(hierarchy_stop_view);
        // The design root is not a boundary candidate: a configuration that
        // stopped at it would netlist nothing, so a stop naming its view says
        // nothing about the root.
        if let Some(stop) = matched_stop.as_deref()
            && !stop_boundary
            && !is_root
            && status.is_resolved()
            && let Some(view_type) = resolved_view_type
        {
            warnings.push(format!(
                "stop view '{stop}' matched by name but {} is a {} view; the hierarchy descends into it",
                resolved_reference.display_path(),
                view_type.display_name()
            ));
        }
        if status.is_resolved()
            && let Some(master) = master.as_ref()
            && let Some(view_type) = master.view_type
        {
            // The design root has no instance interface to materialize: it is
            // the deck, not one of the deck's masters.
            let materialized_binding = if is_root {
                None
            } else {
                master.materialized_binding.clone()
            };
            self.stage_binding(
                node,
                PendingExecutionBinding {
                    instance_path: instance_path.clone(),
                    resolved_reference: resolved_reference.clone(),
                    resolved_view_type: view_type,
                    materialized_binding,
                    model_section: model_section.clone(),
                    stop_boundary,
                    project_veriloga,
                },
            );
        }

        let mut row = self.binding_row_with_master(
            resolved_reference.clone(),
            binding,
            instance_path,
            depth,
            is_root,
            BindingRowOutcome {
                master: master.clone(),
                status,
                used_review_fallback,
                diagnostic,
                placed: target.as_ref().map(|_| requested.clone()),
            },
        );
        if let Some(section) = model_section {
            row.model_section = section;
        }
        row.warnings = warnings;
        self.upsert(row, node);
        if status.is_resolved() {
            self.nodes[node].resolved = true;
        }

        // A cell that does not fit binds nothing, so nothing under it runs.
        if stop_boundary || status == HierarchyBindingStatus::Incompatible {
            return node;
        }

        let Some(schematic) = master.as_ref().and_then(|master| master.schematic) else {
            return node;
        };
        if !schematic
            .document
            .components
            .iter()
            .any(|component| component.kind == ComponentType::CellInstance)
        {
            return node;
        }
        let above = format!("{above}|{}", hierarchy_identity(&resolved_reference));
        ancestors.push(resolved_reference.clone());
        for component in &schematic.document.components {
            if component.kind != ComponentType::CellInstance {
                continue;
            }
            let Some(child) = component.library_cell.as_ref() else {
                continue;
            };
            let instance_name = &component.name;
            let requested_view = if child.view.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW
            } else {
                child.view.as_str()
            };
            let requested = CellViewRef::new(&child.library, &child.cell, requested_view);
            // An instance the design cannot name has no path to resolve at, so
            // it is reported against its parent rather than skipped: a child
            // that silently disappears from the manifest is a hierarchy the
            // receipt claims is complete and is not.
            match instance_path.child(instance_name) {
                Ok(child_path) => {
                    let reaching = self.reaching_below(reaching, depth, instance_name);
                    let context = OccurrenceContext {
                        requested: requested.key().to_ascii_lowercase(),
                        placement: std::ptr::from_ref(child).addr(),
                        depth: depth + 1,
                        ancestors: above.clone(),
                        overrides: reaching.clone(),
                    };
                    let resolved = match self.contexts.get(&context) {
                        Some(&existing) => existing,
                        None => {
                            let created = self.resolve_reference(
                                requested,
                                Some(child),
                                &child_path,
                                depth + 1,
                                false,
                                ancestors,
                                &above,
                                &reaching,
                            );
                            self.contexts.insert(context, created);
                            created
                        }
                    };
                    self.nodes[node].children.push(OccurrenceEdge {
                        name: instance_name.clone(),
                        folded: fold(instance_name),
                        node: resolved,
                    });
                }
                Err(error) => {
                    let row = self.binding_row(
                        requested,
                        Some(child),
                        instance_path,
                        depth + 1,
                        false,
                        (HierarchyBindingStatus::Unresolved, Some(error.to_string())),
                    );
                    self.upsert(row, node);
                }
            }
        }
        ancestors.pop();
        node
    }

    /// Resolve a compiled built-in placement: an XSPICE code model or a
    /// functional part. Both are leaves the engine owns, so neither a view
    /// search nor a model section can apply to them.
    fn resolve_builtin_leaf(
        &mut self,
        node: usize,
        requested: CellViewRef,
        binding: &LibraryCellInstance,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
    ) {
        let functional = binding.is_builtin_functional();
        let (kind, purpose, view) = if functional {
            ("functional part", "built-in functional part", "functional")
        } else {
            ("XSPICE device", "built-in XSPICE code model", "xspice")
        };
        let mut status = HierarchyBindingStatus::Resolved;
        let mut diagnostic = if functional {
            validate_builtin_functional_binding(binding).err()
        } else {
            validate_builtin_xspice_binding(binding).err()
        };
        if diagnostic.is_some() {
            status = HierarchyBindingStatus::Unresolved;
        }

        let current_platform = ConfigurationPlatform::current();
        if status.is_resolved()
            && !self.configured_platform_eligible(instance_path, current_platform)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} is not supported by this execution target ({})",
                current_platform.label()
            ));
        }
        if status.is_resolved()
            && let Some(section) = self.configured_model_section(instance_path)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} cannot be applied to a compiled built-in {kind}"
            ));
        }

        if status.is_resolved() {
            self.stage_binding(
                node,
                PendingExecutionBinding {
                    instance_path: instance_path.clone(),
                    resolved_reference: requested.clone(),
                    resolved_view_type: ViewType::Custom,
                    materialized_binding: Some(binding.clone()),
                    model_section: None,
                    stop_boundary: true,
                    project_veriloga: None,
                },
            );
        }

        let mut row = self.binding_row(
            requested,
            Some(binding),
            instance_path,
            depth,
            is_root,
            (status, diagnostic),
        );
        row.purpose = purpose.to_owned();
        row.view_search_order = vec![view.to_owned()];
        row.stop_view = Some(view.to_owned());
        row.model_section.clear();
        self.upsert(row, node);
        if status.is_resolved() {
            self.nodes[node].resolved = true;
        }
    }

    fn resolve_generated_veriloga(
        &mut self,
        node: usize,
        requested: CellViewRef,
        binding: &LibraryCellInstance,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
    ) {
        let mut status = HierarchyBindingStatus::Resolved;
        let mut diagnostic = validate_generated_veriloga_binding(binding).err();
        if diagnostic.is_some() {
            status = HierarchyBindingStatus::Unresolved;
        }
        let current_platform = ConfigurationPlatform::current();
        if status.is_resolved()
            && !self.configured_platform_eligible(instance_path, current_platform)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} is not supported by this execution target ({})",
                current_platform.label()
            ));
        }
        if status.is_resolved()
            && let Some(section) = self.configured_model_section(instance_path)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} cannot be applied to a compiled generated Verilog-A device"
            ));
        }
        if status.is_resolved() {
            self.stage_binding(
                node,
                PendingExecutionBinding {
                    instance_path: instance_path.clone(),
                    resolved_reference: requested.clone(),
                    resolved_view_type: ViewType::VerilogA,
                    materialized_binding: Some(binding.clone()),
                    model_section: None,
                    stop_boundary: true,
                    project_veriloga: None,
                },
            );
        }
        let mut row = self.binding_row(
            requested,
            Some(binding),
            instance_path,
            depth,
            is_root,
            (status, diagnostic),
        );
        row.purpose = "compiled generated Verilog-A model".to_owned();
        row.view_search_order = vec!["veriloga-generated".to_owned()];
        row.stop_view = Some("veriloga-generated".to_owned());
        row.model_section.clear();
        self.upsert(row, node);
        if status.is_resolved() {
            self.nodes[node].resolved = true;
        }
    }

    fn binding_row(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
        outcome: (HierarchyBindingStatus, Option<String>),
    ) -> ResolvedHierarchyBinding {
        let (status, diagnostic) = outcome;
        self.binding_row_with_master(
            reference,
            binding,
            instance_path,
            depth,
            is_root,
            BindingRowOutcome {
                master: None,
                status,
                used_review_fallback: false,
                diagnostic,
                placed: None,
            },
        )
    }

    fn binding_row_with_master(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
        outcome: BindingRowOutcome<'_>,
    ) -> ResolvedHierarchyBinding {
        let BindingRowOutcome {
            master,
            status,
            used_review_fallback,
            diagnostic,
            placed,
        } = outcome;
        // The bindings that govern a row are the placed cell's, whichever
        // cell runs; what the placement copied from its own cell describes
        // nothing about another one.
        let governed = placed.as_ref().unwrap_or(&reference);
        let own = binding.filter(|_| placed.is_none());
        let search_order = self.view_search_order(
            governed,
            binding.map_or(reference.view.as_str(), |value| value.view.as_str()),
            is_root,
            instance_path,
        );
        let source_bound = own.and_then(|value| value.source_path.as_ref()).is_some();
        let terminal_view = master
            .as_ref()
            .and_then(|value| value.view_type)
            .filter(|view_type| hierarchy_stop_view(*view_type));
        let purpose = if is_root {
            "testbench root"
        } else if source_bound || terminal_view.is_some() {
            "macro-model"
        } else if master
            .is_some_and(|value| value.library_read_only && value.library_has_technology)
        {
            "foundry devices"
        } else if depth == 1 {
            "design under test"
        } else {
            "hierarchical cell"
        };
        // The column names a view, never a view type: a configured stop selects
        // the view it spells, and an unresolved row still shows which stop its
        // ordered search would reach.
        let stop_view = if self.configuration.is_some() {
            self.configured_stop_views(governed, instance_path)
                .into_iter()
                .find(|stop| {
                    stop.eq_ignore_ascii_case(&reference.view)
                        || search_order
                            .iter()
                            .any(|candidate| candidate.eq_ignore_ascii_case(stop))
                })
        } else if is_root {
            None
        } else {
            terminal_view.map(|_| reference.view.clone()).or_else(|| {
                search_order
                    .iter()
                    .rev()
                    .find(|view| hierarchy_stop_view(ViewType::from_name(view)))
                    .cloned()
            })
        };
        ResolvedHierarchyBinding {
            model_section: self.model_section(&reference, own, instance_path),
            rule: self.binding_rule(governed, instance_path),
            placed_ports: if is_root {
                Vec::new()
            } else {
                placed_ports(binding)
            },
            placed,
            reference,
            purpose: purpose.to_owned(),
            view_search_order: search_order,
            stop_view,
            status,
            instance_count: 1,
            occurrences: RowOccurrences::listed([]),
            used_review_fallback,
            diagnostic,
            warnings: Vec::new(),
        }
    }

    fn binding_rule(
        &self,
        master: &CellViewRef,
        instance_path: &InstancePath,
    ) -> HierarchyBindingRule {
        self.configuration
            .map_or(HierarchyBindingRule::Default, |configuration| {
                self.configured_view_list(configuration, master, instance_path)
                    .0
            })
    }

    /// The ordered view list that governs one instance, and the rule that
    /// chose it: the most specific override naming views, then a cell
    /// binding for its master's library and cell, then the configuration's
    /// global view list. An override that names no views inherits them, so
    /// setting its model section or platforms never changes its view.
    ///
    /// A binding that runs another cell chose the row even where it names no
    /// views of its own: the instance's own binding when it names one, else
    /// its cell's. The views it inherits are still searched in that cell.
    fn configured_view_list(
        &self,
        configuration: &'a ConfigurationSet,
        master: &CellViewRef,
        instance_path: &InstancePath,
    ) -> (HierarchyBindingRule, &[String]) {
        if let Some(scoped) = self.configured_view_override(instance_path) {
            return (
                HierarchyBindingRule::InstanceBinding,
                &scoped.executable_views,
            );
        }
        let cell_binding = self.configured_cell_binding(master);
        let views = cell_binding
            .map(|binding| binding.executable_views.as_slice())
            .filter(|views| !views.is_empty())
            .unwrap_or(configuration.executable_view_policy());
        let own_target = self
            .configured_override(instance_path)
            .is_some_and(|scoped| scoped.target.is_some());
        let rule = if own_target {
            HierarchyBindingRule::InstanceBinding
        } else if cell_binding.is_some() {
            HierarchyBindingRule::CellBinding
        } else {
            HierarchyBindingRule::ViewList
        };
        (rule, views)
    }

    /// The most specific override matching one instance that names views:
    /// the one whose view list, and stop, govern it.
    fn configured_view_override(
        &self,
        instance_path: &InstancePath,
    ) -> Option<&ConfigurationSetOverride> {
        self.most_specific_override(instance_path, |scoped| !scoped.executable_views.is_empty())
    }

    /// The most specific of the overrides `admits` that matches one
    /// instance, over the patterns parsed once for the whole resolution: a
    /// lookup runs several times for every row, so it tries only the
    /// overrides that name the instance and those holding a wildcard, in
    /// catalog order, as a walk of every override would.
    fn most_specific_override(
        &self,
        instance_path: &InstancePath,
        admits: impl Fn(&ConfigurationSetOverride) -> bool,
    ) -> Option<&ConfigurationSetOverride> {
        let mut candidates = self.wildcard_overrides.clone();
        if let Some(named) = self.named_overrides.get(&instance_path.fold_key()) {
            candidates.extend(named);
            candidates.sort_unstable();
        }
        candidates
            .into_iter()
            .filter_map(|at| {
                let scoped = &self.overrides[at];
                let pattern = self.patterns[at].as_ref()?;
                (admits(scoped) && pattern.matches(instance_path))
                    .then(|| (scoped, pattern.specificity()))
            })
            .max_by_key(|(_, specificity)| *specificity)
            .map(|(scoped, _)| scoped)
    }

    /// The cell binding for one master's library and cell, matched the way
    /// the catalog folds those names.
    fn configured_cell_binding(
        &self,
        master: &CellViewRef,
    ) -> Option<&'a ConfigurationCellBinding> {
        if self.cell_bindings.is_empty() {
            return None;
        }
        self.cell_bindings
            .get(&cell_binding_key(&master.library, &master.cell))
            .copied()
    }

    /// The cell an instance of `placed` at `instance_path` runs in its
    /// place, in the precedence its view list follows: its own binding's
    /// target, then its cell binding's. `None` runs the placed cell. The
    /// design root is the deck itself and is never replaced.
    fn configured_target(
        &self,
        placed: &CellViewRef,
        instance_path: &InstancePath,
        is_root: bool,
    ) -> Option<&'a BindingTarget> {
        if is_root {
            return None;
        }
        let configuration = self.configuration?;
        let own =
            selected_configuration_override(configuration.overrides(), &instance_path.to_string())
                .and_then(|scoped| scoped.target.as_ref());
        own.or_else(|| {
            self.configured_cell_binding(placed)
                .and_then(|binding| binding.target.as_ref())
        })
        .filter(|target| !target.names(&placed.library, &placed.cell))
    }

    fn configured_primary_views(
        &self,
        master: &CellViewRef,
        requested: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> Vec<String> {
        let Some(configuration) = self.configuration else {
            return hierarchy_view_search_order(requested, is_root);
        };
        let mut order = Vec::new();
        if is_root {
            order.push(if requested.eq_ignore_ascii_case("symbol") {
                DEFAULT_SCHEMATIC_VIEW.to_owned()
            } else {
                requested.to_ascii_lowercase()
            });
        }
        order.extend(
            self.configured_view_list(configuration, master, instance_path)
                .1
                .iter()
                .cloned(),
        );
        deduplicate_view_order(&mut order);
        order
    }

    fn view_search_order(
        &self,
        master: &CellViewRef,
        requested: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> Vec<String> {
        let Some(configuration) = self.configuration else {
            return hierarchy_view_search_order(requested, is_root);
        };
        let mut order = self.configured_primary_views(master, requested, is_root, instance_path);
        if configuration.definition().unresolved_policy
            == UnresolvedBindingPolicy::ExplicitFallbackWithReview
        {
            order.extend(hierarchy_view_search_order(requested, is_root));
        }
        deduplicate_view_order(&mut order);
        order
    }

    fn used_review_fallback(
        &self,
        requested: &CellViewRef,
        resolved: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> bool {
        let Some(configuration) = self.configuration else {
            return false;
        };
        configuration.definition().unresolved_policy
            == UnresolvedBindingPolicy::ExplicitFallbackWithReview
            && !self
                .configured_primary_views(requested, &requested.view, is_root, instance_path)
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(resolved))
    }

    /// The configured stops for one instance, in the precedence its view list
    /// follows: the override's stop view, then its master's cell binding's,
    /// then the configuration's global stop views.
    fn configured_stop_views(
        &self,
        master: &CellViewRef,
        instance_path: &InstancePath,
    ) -> Vec<String> {
        let Some(configuration) = self.configuration else {
            return Vec::new();
        };
        if let Some(stop_view) = self
            .configured_view_override(instance_path)
            .and_then(|scoped| scoped.stop_view.as_ref())
            .or_else(|| {
                self.configured_cell_binding(master)
                    .and_then(|binding| binding.stop_view.as_ref())
            })
        {
            return vec![stop_view.clone()];
        }
        configuration.stop_views().to_vec()
    }

    /// The configured stop that names the selected view, if any.
    ///
    /// A stop names a view, so `spice_tt` stops at the view called `spice_tt`
    /// and at nothing else; a view's type is not one of its names. Whether the
    /// match may actually terminate the hierarchy is a separate question, and
    /// [`hierarchy_stop_view`] is the one authority on it.
    fn matched_stop_view(
        &self,
        placed: &CellViewRef,
        resolved_view: &str,
        instance_path: &InstancePath,
    ) -> Option<String> {
        self.configured_stop_views(placed, instance_path)
            .into_iter()
            .find(|stop| stop.eq_ignore_ascii_case(resolved_view))
    }

    fn model_section(
        &self,
        reference: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
    ) -> String {
        self.configured_model_section(instance_path)
            .unwrap_or_else(|| hierarchy_model_section(self.context.libraries, reference, binding))
    }

    fn configured_model_section(&self, instance_path: &InstancePath) -> Option<String> {
        self.configured_override(instance_path)
            .and_then(|scoped| scoped.model_section.clone())
    }

    fn configured_platform_eligible(
        &self,
        instance_path: &InstancePath,
        platform: ConfigurationPlatform,
    ) -> bool {
        self.configured_override(instance_path)
            .is_none_or(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn configured_platform_declared(
        &self,
        instance_path: &InstancePath,
        platform: ConfigurationPlatform,
    ) -> bool {
        self.configured_override(instance_path)
            .is_some_and(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    /// The override governing one instance, selected from the canonicalized
    /// patterns so the authored spelling cannot change which one wins.
    fn configured_override(
        &self,
        instance_path: &InstancePath,
    ) -> Option<&ConfigurationSetOverride> {
        self.most_specific_override(instance_path, |_| true)
    }

    /// Retain what one node runs for the plan this resolution seals.
    fn stage_binding(&mut self, node: usize, binding: PendingExecutionBinding) {
        self.pending[node] = Some(binding);
    }

    fn execution_model_section_conflict(&self) -> Option<String> {
        let mut sources = HashMap::<String, (Option<&str>, &InstancePath)>::new();
        for binding in self.pending.iter().flatten() {
            let Some(materialized) = binding.materialized_binding.as_ref() else {
                continue;
            };
            let Some(source_path) = materialized.source_path.as_deref() else {
                continue;
            };
            let key = self
                .context
                .source_files
                .configured_source_identity(source_path);
            let section = binding.model_section.as_deref();
            if let Some((existing_section, existing_path)) = sources.get(&key).copied() {
                if existing_section != section {
                    return Some(format!(
                        "source '{}' has conflicting model-section bindings '{}' at {} and '{}' at {}",
                        source_path.display(),
                        existing_section.unwrap_or("<entire source>"),
                        existing_path,
                        section.unwrap_or("<entire source>"),
                        binding.instance_path
                    ));
                }
            } else {
                sources.insert(key, (section, &binding.instance_path));
            }
        }
        None
    }

    fn resolve_master(
        &self,
        requested: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        search_order: &[String],
        configured_model_section: Option<&str>,
    ) -> Result<Option<(HierarchyMaster<'a>, Option<CellViewRef>)>, String> {
        let library = find_library(self.context.libraries, &requested.library);
        let cell = library.and_then(|library| find_cell(library, &requested.cell));
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();
        // A Verilog, Verilog-A or Verilog-AMS view runs the project bundle
        // its cell view owns, configured or not (`source_view_source`):
        // Spectre's default view list reaches `veriloga`, and a placed
        // binding's copy of its source and module is only what placement saw.
        let requests_veriloga = cell
            .and_then(|cell| find_view(cell, &requested.view))
            .is_some_and(|view| view.view_type.is_hdl_source());

        // Compatibility mode retains the historical placed-binding authority.
        // Configuration mode below instead materializes each selected L/C/V
        // from the authoritative library view.
        if self.configuration.is_none() && source_bound && !requests_veriloga {
            if !search_order
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(&requested.view))
            {
                return Ok(None);
            }
            let Some(view) = cell.and_then(|cell| find_view(cell, &requested.view)) else {
                return Ok(None);
            };
            let binding = binding.expect("source-bound branch has a placed binding");
            self.validate_source_binding(binding)?;
            return Ok(Some((
                HierarchyMaster {
                    schematic: None,
                    view_type: Some(view.view_type),
                    view_modified: view.modified,
                    library_read_only: library.is_some_and(|library| library.read_only),
                    library_has_technology: library
                        .is_some_and(|library| !library.technology.trim().is_empty()),
                    materialized_binding: Some(binding.clone()),
                },
                Some(requested.clone()),
            )));
        }

        for candidate in search_order {
            // A buffer without an authoritative library/cell/view identity is
            // an orphan, not an executable master. Corrupt or partially
            // restored workspaces must fail closed.
            let Some(view) = cell.and_then(|cell| find_view(cell, candidate)) else {
                continue;
            };
            let reference = CellViewRef::new(
                &library.expect("view implies library").name,
                &cell.expect("view implies cell").name,
                &view.name,
            );
            let view_type = view.view_type;
            if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
                if let Some(schematic) = self.find_schematic(&reference) {
                    let master = (
                        library.expect("view implies library"),
                        cell.expect("view implies cell"),
                        view,
                    );
                    let materialized_binding = binding
                        .map(|placed| {
                            materialize_schematic_binding(
                                placed,
                                master,
                                &reference,
                                schematic.document,
                            )
                        })
                        .transpose()?;
                    return Ok(Some((
                        HierarchyMaster {
                            schematic: Some(schematic),
                            view_type: Some(view_type),
                            view_modified: view.modified,
                            library_read_only: library.is_some_and(|library| library.read_only),
                            library_has_technology: library
                                .is_some_and(|library| !library.technology.trim().is_empty()),
                            materialized_binding,
                        },
                        Some(reference),
                    )));
                }
                continue;
            }
            if hierarchy_stop_view(view_type)
                && (self.configuration.is_some() || view_type.is_hdl_source())
            {
                let Some(placed) = binding else {
                    return Err(format!(
                        "configuration root {} cannot materialize source view '{}' without an instance interface",
                        requested.display_path(),
                        candidate
                    ));
                };
                let mut materialized = materialize_authoritative_source_binding(
                    placed,
                    library.expect("view implies library"),
                    cell.expect("view implies cell"),
                    view,
                    self.context.project_id,
                    self.context.project_sources,
                    self.context.libraries,
                )?;
                if let Some(section) = configured_model_section {
                    materialized.model_section = Some(section.to_owned());
                }
                self.validate_source_binding(&materialized)?;
                return Ok(Some((
                    HierarchyMaster {
                        schematic: None,
                        view_type: Some(view_type),
                        view_modified: view.modified,
                        library_read_only: library.is_some_and(|library| library.read_only),
                        library_has_technology: library
                            .is_some_and(|library| !library.technology.trim().is_empty()),
                        materialized_binding: Some(materialized),
                    },
                    Some(reference),
                )));
            }
        }
        Ok(None)
    }

    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'a>> {
        self.context.documents.find_schematic(reference)
    }

    fn validate_source_binding(&self, binding: &LibraryCellInstance) -> Result<(), String> {
        let Some(library) = find_library(self.context.libraries, &binding.library) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative library",
                binding.library, binding.cell
            ));
        };
        let Some(cell) = find_cell(library, &binding.cell) else {
            return Err(format!(
                "source-backed binding {}/{} has no authoritative cell",
                binding.library, binding.cell
            ));
        };
        let Some(view) = find_view(cell, &binding.view) else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative view",
                binding.library, binding.cell, binding.view
            ));
        };
        if !matches!(
            view.view_type,
            ViewType::Spice
                | ViewType::VerilogA
                | ViewType::Verilog
                | ViewType::VerilogAms
                | ViewType::Extracted
        ) {
            return Err(format!(
                "source-backed binding {}/{}/{} is not an executable source view",
                binding.library, binding.cell, binding.view
            ));
        }
        if binding.terminal_order.is_empty() {
            return Err(format!(
                "source-backed binding {}/{}/{} has no validated terminal contract",
                binding.library, binding.cell, binding.view
            ));
        }
        let source_path = binding
            .source_path
            .as_deref()
            .expect("validated only for source-backed bindings");
        if view.view_type.is_hdl_source() {
            let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
            let project_binding = project_veriloga_binding_for_view(
                self.context.project_id,
                self.context.project_sources,
                self.context.libraries,
                &reference,
            )?;
            if source_path != Path::new(project_binding.source_key())
                || binding.module_name.as_deref().is_none_or(|module| {
                    !module.eq_ignore_ascii_case(project_binding.netlist_alias())
                })
            {
                return Err(format!(
                    "source-backed binding {} does not match its exact project-owned Verilog-A bundle",
                    reference.display_path()
                ));
            }
            return Ok(());
        }
        // Projects are portable metadata: a binding authored on Windows must
        // still read as an absolute identity on macOS and Linux, and vice
        // versa. Judge the syntax, not the host that happens to be running.
        if !rspice_model_library::is_portable_absolute_path(source_path) {
            return Err(format!(
                "source-backed binding {}/{}/{} does not have an absolute source identity",
                binding.library, binding.cell, binding.view
            ));
        }
        let authoritative_path = view
            .file_path
            .as_deref()
            .or_else(|| metadata_source_path(&view.metadata))
            .or_else(|| metadata_source_path(&cell.metadata));
        let Some(authoritative_path) = authoritative_path else {
            return Err(format!(
                "source-backed binding {}/{}/{} has no authoritative source identity",
                binding.library, binding.cell, binding.view
            ));
        };
        // An absolute identity belonging to another host's syntax must never be
        // handed to the filesystem below. `/models/amp.cir` resolves against
        // the current drive on Windows, so both the canonicalizing comparison
        // and the source-file probe could answer about an unrelated local file
        // instead of reporting that the binding is simply unavailable here.
        #[cfg(not(target_arch = "wasm32"))]
        if rspice_model_library::is_foreign_platform_absolute_path(source_path)
            || rspice_model_library::is_foreign_platform_absolute_path(authoritative_path)
        {
            return Err(format!(
                "source-backed binding {}/{}/{} retains a foreign-platform source identity '{}' that is unavailable on this host; re-import or repair the binding",
                binding.library,
                binding.cell,
                binding.view,
                source_path.display()
            ));
        }
        if !self
            .context
            .source_files
            .source_paths_match(source_path, authoritative_path)
        {
            return Err(format!(
                "source-backed binding {}/{}/{} conflicts with the authoritative source path",
                binding.library, binding.cell, binding.view
            ));
        }
        self.context
            .source_files
            .validate_source_file(source_path, view.view_type, binding)
    }

    /// Count each occurrence of `node` in `row`'s group.
    fn upsert(&mut self, row: ResolvedHierarchyBinding, node: usize) {
        let index = self.group(row);
        self.nodes[node].rows.push(index);
    }

    /// Count `occurrence`, which no node holds, in `row`'s group.
    fn list(&mut self, row: ResolvedHierarchyBinding, occurrence: InstancePath) {
        let index = self.group(row);
        self.listed.push((index, occurrence));
    }

    /// The group `row` belongs to, made if it is the first of it. Counts and
    /// occurrences are the graph's, once the walk is done.
    fn group(&mut self, mut row: ResolvedHierarchyBinding) -> usize {
        // Rows are grouped by the executable binding contract, not by their
        // current outcome. Repeated instances of one master must remain one
        // review row while its occurrences name every exact instance;
        // a recursive or unresolved occurrence then promotes the aggregate to
        // the most severe observed status. Configuration variants still split
        // naturally through their ordered views, stop, model, and fallback
        // fields below, and so do rows a different rule chose.
        let key = format!(
            "{}|{}|{}|{}|{}|{}|{:?}",
            row.reference.key().to_ascii_lowercase(),
            row.placed
                .as_ref()
                .map(|placed| placed.key().to_ascii_lowercase())
                .unwrap_or_default(),
            row.view_search_order.join(",").to_ascii_lowercase(),
            row.stop_view
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase(),
            row.model_section.to_ascii_lowercase(),
            row.used_review_fallback,
            row.rule,
        );
        if let Some(index) = self.row_indices.get(&key).copied() {
            let existing = &mut self.rows[index];
            // Warnings are the row's, not one occurrence's: merged occurrences
            // share the grouped binding contract that provoked them.
            for warning in std::mem::take(&mut row.warnings) {
                if !existing.warnings.contains(&warning) {
                    existing.warnings.push(warning);
                }
            }
            if row.status.severity() > existing.status.severity() {
                existing.status = row.status;
                existing.diagnostic = row.diagnostic;
            }
            return index;
        }
        let index = self.rows.len();
        self.row_indices.insert(key, index);
        self.rows.push(row);
        index
    }
}

/// Whether a pattern position admits an instance named `name`.
fn segment_matches(segment: &PatternSegment, name: &str) -> bool {
    match segment {
        PatternSegment::Star => true,
        PatternSegment::Name(expected) => fold(expected) == fold(name),
    }
}

/// The folded path a pattern names when it pins every position by name, as
/// [`InstancePath::fold_key`] spells it; `None` for one holding a wildcard.
fn named_path_key(pattern: &InstancePathPattern) -> Option<String> {
    if pattern.is_root() {
        return Some("/".to_owned());
    }
    let mut key = String::new();
    for segment in pattern.segments() {
        let PatternSegment::Name(name) = segment else {
            return None;
        };
        key.push('/');
        key.push_str(&fold(name));
    }
    Some(key)
}

/// The paths reported missing from the expanded hierarchy: by their folded
/// key, so a path is reported once however it is spelled, and whole, for a
/// pattern to be matched against.
#[derive(Default)]
struct ReportedPaths {
    paths: Vec<InstancePath>,
    keys: HashSet<String>,
}

/// The instance to attribute an unmet configuration requirement to, and why
/// it is unmet — or `None` when the expanded hierarchy satisfies it.
///
/// `text` is the authored spelling. A requirement naming one instance is
/// looked up by its path; a pattern is matched against every instance the
/// hierarchy places, a context at a time, because that is the only thing a
/// pattern can be measured against. `reported` holds the paths already
/// reported missing, so one is reported once.
fn unmet_requirement(
    graph: &HierarchyOccurrences,
    reported: &mut ReportedPaths,
    text: &str,
    purpose: &str,
) -> Option<(InstancePath, String)> {
    match InstancePath::parse(text) {
        Ok(path) => {
            if graph.node_at(&path).is_some() || !reported.keys.insert(path.fold_key()) {
                return None;
            }
            reported.paths.push(path.clone());
            Some((
                path.clone(),
                format!("{purpose} {path} does not exist in the expanded hierarchy"),
            ))
        }
        Err(HierarchyPathError::WildcardInPath(_)) => {
            let diagnostic = match InstancePathPattern::parse(text) {
                Ok(pattern) => {
                    if places_a_match(graph, &pattern)
                        || reported.paths.iter().any(|seen| pattern.matches(seen))
                    {
                        return None;
                    }
                    format!("{purpose} {pattern} matches no instance in the expanded hierarchy")
                }
                Err(error) => format!("{purpose} {text} is not a hierarchy pattern: {error}"),
            };
            Some((InstancePath::root(), diagnostic))
        }
        Err(error) => Some((
            InstancePath::root(),
            format!("{purpose} {text} is not a hierarchical path: {error}"),
        )),
    }
}

/// Whether the hierarchy places an instance `pattern` matches: a walk of the
/// graph that tries each context once at each position of the pattern.
fn places_a_match(graph: &HierarchyOccurrences, pattern: &InstancePathPattern) -> bool {
    fn from(
        graph: &HierarchyOccurrences,
        segments: &[PatternSegment],
        node: usize,
        at: usize,
        tried: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if at == segments.len() {
            return true;
        }
        if let Some(found) = tried.get(&(node, at)) {
            return *found;
        }
        let found = graph.nodes[node].children.iter().any(|edge| {
            segment_matches(&segments[at], &edge.name)
                && from(graph, segments, edge.node, at + 1, tried)
        });
        tried.insert((node, at), found);
        found
    }
    !graph.nodes.is_empty() && from(graph, pattern.segments(), 0, 0, &mut HashMap::new())
}

/// Seal what each node runs into the frozen execution plan, and name its
/// masters.
///
/// Three things are decided here and nowhere else: each node's closure
/// digest, which nodes are one master, and what that master is called.
/// Everything downstream — the emitted deck, the receipt, the repair actions —
/// reads those three answers rather than deriving a fourth.
fn seal_plan(
    graph: &mut HierarchyOccurrences,
    pending: Vec<Option<PendingExecutionBinding>>,
) -> BTreeMap<MasterKey, MasterRecord> {
    // Children before parents: a closure digest folds its children's.
    let mut digests = vec![None::<ContentDigest>; graph.nodes.len()];
    let mut order = graph.topological_order();
    order.reverse();
    for node in order {
        let Some(binding) = pending[node].as_ref() else {
            continue;
        };
        let children = graph.nodes[node]
            .children
            .iter()
            .filter_map(|edge| digests[edge.node])
            .collect::<Vec<_>>();
        digests[node] = Some(master_closure_digest(
            &binding.resolved_reference,
            binding.resolved_view_type,
            binding.model_section.as_deref(),
            binding.stop_boundary,
            binding
                .project_veriloga
                .as_ref()
                .map(ConfigurationVerilogABinding::source_closure_digest),
            &children,
        ));
    }

    let mut master_order = Vec::new();
    let mut reserved = HashSet::new();
    for node in graph.first_reached() {
        let Some(binding) = pending[node].as_ref() else {
            continue;
        };
        let Some(closure) = digests[node] else {
            continue;
        };
        let emits_subcircuit = !binding.instance_path.is_root()
            && matches!(
                binding.resolved_view_type,
                ViewType::Schematic | ViewType::Testbench
            );
        if emits_subcircuit {
            master_order.push((
                node,
                MasterKey::new(&binding.resolved_reference, closure),
                binding.resolved_reference.clone(),
            ));
        } else if let Some(module) = binding
            .materialized_binding
            .as_ref()
            .and_then(|binding| binding.module_name.as_deref())
            .map(str::trim)
            .filter(|module| !module.is_empty())
        {
            // A model-bound master is an engine module the deck references by
            // its own name. A generated subcircuit may not shadow it.
            reserved.insert(module.to_ascii_lowercase());
        }
    }
    let names = assign_master_names(
        master_order
            .iter()
            .map(|(_, key, reference)| (key, reference)),
        &reserved,
    );
    let mut masters = BTreeMap::<MasterKey, MasterRecord>::new();
    for (node, key, _) in master_order {
        masters
            .entry(key.clone())
            .or_insert_with(|| MasterRecord::new(names[&key].clone()));
        graph.nodes[node].master = Some(key);
    }

    for (node, pending) in pending.into_iter().enumerate() {
        let (Some(pending), Some(closure)) = (pending, digests[node]) else {
            continue;
        };
        graph.nodes[node].binding = Some(ConfigurationExecutionBinding {
            instance_path: pending.instance_path,
            resolved_reference: pending.resolved_reference,
            resolved_view_type: pending.resolved_view_type,
            materialized_binding: pending.materialized_binding,
            model_section: pending.model_section,
            stop_boundary: pending.stop_boundary,
            project_veriloga: pending.project_veriloga,
            binding_closure_digest: closure,
        });
    }
    masters
}

/// One override with its pattern re-spelled in the canonical grammar.
fn canonical_override(scoped: &ConfigurationSetOverride) -> ConfigurationSetOverride {
    let mut scoped = scoped.clone();
    if let Ok(pattern) = InstancePathPattern::parse(&scoped.instance_path) {
        scoped.instance_path = pattern.to_string();
    }
    scoped
}

pub fn find_library<'a>(libraries: &'a LibraryCatalog, name: &str) -> Option<&'a Library> {
    libraries
        .libraries_by_key()
        .find(|(key, library)| {
            key.eq_ignore_ascii_case(name) || library.name.eq_ignore_ascii_case(name)
        })
        .map(|(_, library)| library)
}

pub fn find_cell<'a>(library: &'a Library, name: &str) -> Option<&'a Cell> {
    library
        .cells
        .iter()
        .find(|(key, cell)| key.eq_ignore_ascii_case(name) || cell.name.eq_ignore_ascii_case(name))
        .map(|(_, cell)| cell)
}

pub fn find_view<'a>(cell: &'a Cell, name: &str) -> Option<&'a View> {
    cell.views
        .iter()
        .find(|(key, view)| key.eq_ignore_ascii_case(name) || view.name.eq_ignore_ascii_case(name))
        .map(|(_, view)| view)
}
