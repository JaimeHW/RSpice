//! Resolving a testbench hierarchy to exact Library/Cell/View bindings.
//!
//! Resolution walks the placed instances and answers one question per row:
//! which view will actually be netlisted, and did the configuration say so or
//! did the search order fall back?  A row records that distinction rather than
//! hiding it, so an unresolved or fallback binding can never be mistaken for a
//! configured one.  Nothing here mutates the workspace — the resolver borrows
//! it and returns a [`HierarchyResolution`].
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

use super::*;

use crate::state::{HierarchyPathError, InstancePath, InstancePathPattern};

#[cfg(test)]
mod tests;

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

/// One master as the deck declares it: the name it is emitted under, the cell
/// view it came from, and every occurrence that instantiates it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterRecord {
    name: String,
    reference: CellViewRef,
    occurrences: Vec<InstancePath>,
}

impl MasterRecord {
    fn new(name: String, reference: CellViewRef) -> Self {
        Self {
            name,
            reference,
            occurrences: Vec::new(),
        }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationExecutionPlan {
    root: CellViewRef,
    bindings: BTreeMap<String, ConfigurationExecutionBinding>,
    masters: BTreeMap<MasterKey, MasterRecord>,
    /// Master identity per occurrence, keyed by [`InstancePath::fold_key`].
    occurrence_masters: BTreeMap<String, MasterKey>,
    configuration_id: Option<crate::state::ConfigurationSetId>,
    configuration_revision: u64,
    configuration_digest: ContentDigest,
}

impl ConfigurationExecutionPlan {
    pub const fn root(&self) -> &CellViewRef {
        &self.root
    }

    pub fn binding(&self, instance_path: &InstancePath) -> Option<&ConfigurationExecutionBinding> {
        self.bindings.get(&instance_path.fold_key())
    }

    pub fn bindings(&self) -> impl ExactSizeIterator<Item = &ConfigurationExecutionBinding> {
        self.bindings.values()
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
        self.occurrence_masters.get(&instance_path.fold_key())
    }

    pub const fn configuration_id(&self) -> Option<crate::state::ConfigurationSetId> {
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
pub(crate) fn master_closure_digest(
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
pub(crate) fn assign_master_names<'a>(
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
    Recursive,
    DepthLimit,
    InstanceLimit,
}

impl HierarchyBindingStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Modified => "modified",
            Self::Unresolved => "unresolved",
            Self::Recursive => "recursive",
            Self::DepthLimit => "depth limit",
            Self::InstanceLimit => "instance limit",
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
            Self::Recursive => 3,
            Self::DepthLimit => 4,
            Self::InstanceLimit => 5,
        }
    }
}

/// One row in the resolved hierarchy-binding manifest. Repeated masters are
/// grouped while `instance_count` retains their exact expanded multiplicity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedHierarchyBinding {
    pub reference: CellViewRef,
    pub purpose: String,
    pub view_search_order: Vec<String>,
    pub stop_view: Option<String>,
    pub model_section: String,
    pub status: HierarchyBindingStatus,
    pub instance_count: usize,
    /// Exact expanded instance paths represented by this grouped semantic
    /// binding, rendered canonically. Paths remain available for configuration
    /// review and exact override audit even when repeated masters are grouped
    /// in the table.
    pub instance_paths: Vec<String>,
    /// True when the active configuration explicitly permits and records a
    /// reviewed fallback outside its primary ordered view policy.
    pub used_review_fallback: bool,
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
    pub total_instances: usize,
    pub resolved_instances: usize,
    pub configuration_id: Option<crate::state::ConfigurationSetId>,
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
pub(super) struct HierarchyResolver<'a> {
    workspace: &'a ProjectWorkspace,
    libraries: &'a LibraryManager,
    active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    /// The active configuration's overrides with their patterns canonicalized,
    /// so selection compares one grammar against itself.
    overrides: Vec<crate::state::ConfigurationSetOverride>,
    rows: Vec<ResolvedHierarchyBinding>,
    row_indices: HashMap<String, usize>,
    /// Occurrences merged into each row, keyed by [`InstancePath::fold_key`]
    /// and valued by the spelling the row displays.
    row_occurrences: Vec<BTreeMap<String, String>>,
    total_instances: usize,
    resolved_instances: usize,
    encountered_instance_paths: HashMap<String, InstancePath>,
    execution_bindings: BTreeMap<String, PendingExecutionBinding>,
    /// Fold keys in the order the walk reached them, which is the order the
    /// design places its instances and therefore the order masters are named
    /// in. The map above is ordered lexically, which is not the same thing.
    execution_order: Vec<String>,
}

#[derive(Clone)]
pub(super) struct HierarchyMaster<'a> {
    schematic: Option<&'a SchematicState>,
    view_type: Option<ViewType>,
    view_modified: bool,
    library_read_only: bool,
    library_has_technology: bool,
    materialized_binding: Option<LibraryCellInstance>,
}

impl<'a> HierarchyResolver<'a> {
    pub(super) fn new(
        workspace: &'a ProjectWorkspace,
        libraries: &'a LibraryManager,
        active_overlay: Option<(&'a CellViewRef, &'a SchematicState)>,
    ) -> Self {
        Self {
            workspace,
            libraries,
            active_overlay,
            overrides: workspace
                .configuration_sets
                .active()
                .map(|configuration| {
                    configuration
                        .overrides()
                        .iter()
                        .map(canonical_override)
                        .collect()
                })
                .unwrap_or_default(),
            rows: Vec::new(),
            row_indices: HashMap::new(),
            row_occurrences: Vec::new(),
            total_instances: 0,
            resolved_instances: 0,
            encountered_instance_paths: HashMap::new(),
            execution_bindings: BTreeMap::new(),
            execution_order: Vec::new(),
        }
    }

    pub(super) fn resolve(self) -> HierarchyResolution {
        self.resolve_all().0
    }

    pub(super) fn resolve_all(mut self) -> (HierarchyResolution, ConfigurationExecutionPlan) {
        let active_configuration = self.workspace.configuration_sets.active();
        let root = active_configuration.map_or_else(
            || {
                CellViewRef::new(
                    &self.workspace.project.root_library,
                    &self.workspace.project.top_cell,
                    DEFAULT_SCHEMATIC_VIEW,
                )
            },
            |configuration| configuration.root().clone(),
        );
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
            self.total_instances = 1;
            let row = self.binding_row(
                root.clone(),
                None,
                &design_root,
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row, &design_root);
        } else {
            self.resolve_reference(root.clone(), None, &design_root, 0, true, &mut ancestors);
        }
        for (text, purpose) in required_paths {
            let Some((occurrence, diagnostic)) = self.unmet_requirement(&text, &purpose) else {
                continue;
            };
            self.total_instances = self.total_instances.saturating_add(1);
            let row = self.binding_row(
                root.clone(),
                None,
                &occurrence,
                1,
                false,
                (HierarchyBindingStatus::Unresolved, Some(diagnostic)),
            );
            self.upsert(row, &occurrence);
        }
        if let Some(error) = self.execution_model_section_conflict() {
            self.total_instances = self.total_instances.saturating_add(1);
            let row = self.binding_row(
                root.clone(),
                None,
                &design_root,
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row, &design_root);
        }
        let active_configuration = self.workspace.configuration_sets.active();
        let resolution = HierarchyResolution {
            bindings: self.rows,
            total_instances: self.total_instances,
            resolved_instances: self.resolved_instances,
            configuration_id: active_configuration.map(|configuration| configuration.id()),
            configuration_revision: active_configuration
                .map(|configuration| configuration.revision()),
            configuration_digest: active_configuration
                .map(|configuration| configuration.semantic_digest()),
        };
        let plan = seal_plan(
            root,
            self.execution_bindings,
            &self.execution_order,
            active_configuration,
        );
        (resolution, plan)
    }

    /// The instance to attribute an unmet configuration requirement to, and
    /// why it is unmet — or `None` when the expanded hierarchy satisfies it.
    ///
    /// `text` is the authored spelling, which is where the pre-implicit-root
    /// grammar is still admitted. A requirement naming one instance is looked
    /// up by identity; a pattern is matched against every instance the walk
    /// reached, because that is the only thing a pattern can be measured
    /// against.
    fn unmet_requirement(&mut self, text: &str, purpose: &str) -> Option<(InstancePath, String)> {
        match InstancePath::parse_legacy(text) {
            Ok(path) => {
                let key = path.fold_key();
                if self.encountered_instance_paths.contains_key(&key) {
                    return None;
                }
                self.encountered_instance_paths.insert(key, path.clone());
                Some((
                    path.clone(),
                    format!("{purpose} {path} does not exist in the expanded hierarchy"),
                ))
            }
            Err(HierarchyPathError::WildcardInPath(_)) => {
                let diagnostic = match InstancePathPattern::parse_legacy(text) {
                    Ok(pattern) => {
                        if self
                            .encountered_instance_paths
                            .values()
                            .any(|candidate| pattern.matches(candidate))
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

    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<CellViewRef>,
    ) {
        self.encountered_instance_paths
            .insert(instance_path.fold_key(), instance_path.clone());
        if self.total_instances >= MAX_HIERARCHY_RESOLUTION_INSTANCES {
            let mut row = self.binding_row(
                requested,
                binding,
                instance_path,
                depth,
                is_root,
                (
                    HierarchyBindingStatus::InstanceLimit,
                    Some(format!(
                        "hierarchy exceeds the supported limit of {MAX_HIERARCHY_RESOLUTION_INSTANCES} expanded instances"
                    )),
                ),
            );
            row.instance_count = 1;
            self.total_instances = self.total_instances.saturating_add(1);
            self.upsert(row, instance_path);
            return;
        }
        self.total_instances += 1;

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
            self.upsert(row, instance_path);
            return;
        }

        // Compiled code models are executable leaves, not project library
        // masters.  Resolve their frozen catalog contract directly so an
        // active configuration cannot turn a valid placement into an
        // unresolved L/C/V reference or silently substitute another view.
        if let Some(binding) = binding
            && binding.is_builtin_xspice()
        {
            self.resolve_builtin_xspice(requested, binding, instance_path, depth, is_root);
            return;
        }
        if let Some(binding) = binding
            && binding.is_generated_veriloga()
        {
            self.resolve_generated_veriloga(requested, binding, instance_path, depth, is_root);
            return;
        }

        let search_order = self.view_search_order(&requested.view, is_root, instance_path);
        let (master, resolution_error) =
            match self.resolve_master(&requested, binding, &search_order) {
                Ok(master) => (master, None),
                Err(error) => (None, Some(error)),
            };
        let resolved_reference = master
            .as_ref()
            .and_then(|(_, reference)| reference.clone())
            .unwrap_or_else(|| requested.clone());
        let used_review_fallback = master.is_some()
            && self.used_review_fallback(
                &requested.view,
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
                master.map(|(master, _)| master),
                HierarchyBindingStatus::Recursive,
                false,
                Some(format!("recursive hierarchy: {chain}")),
            );
            self.upsert(row, instance_path);
            return;
        }

        let (master, mut status, mut diagnostic) = match master {
            Some((master, _)) => {
                let modified = master.view_modified
                    || master.schematic.is_some_and(|schematic| schematic.is_dirty)
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
                        "no executable master resolved for {} using {}",
                        hierarchy_display_path(&requested),
                        search_order.join(" → ")
                    ))
                }),
            ),
        };

        let current_platform = crate::state::ConfigurationPlatform::current();
        if !self.configured_platform_eligible(instance_path, current_platform) {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "binding at {instance_path} is not supported by this execution target ({})",
                current_platform.label()
            ));
        }
        if status.is_resolved()
            && self.configured_platform_declared(
                instance_path,
                crate::state::ConfigurationPlatform::Browser,
            )
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
        let configured_model_section = self.configured_model_section(instance_path);
        if let Some(section) = configured_model_section.as_deref()
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
        if let Some(section) = configured_model_section.as_deref()
            && status.is_resolved()
            && let Some(source_path) = master
                .as_ref()
                .and_then(|value| value.materialized_binding.as_ref())
                .and_then(|binding| binding.source_path.as_deref())
            && let Err(error) = validate_configured_model_section(source_path, section)
        {
            status = HierarchyBindingStatus::Unresolved;
            diagnostic = Some(format!(
                "model section '{section}' at {instance_path} is unavailable: {error}"
            ));
        }

        let project_veriloga = if status.is_resolved()
            && master
                .as_ref()
                .and_then(|value| value.view_type)
                .is_some_and(|view_type| view_type == ViewType::VerilogA)
        {
            match project_veriloga_binding_for_view(
                self.workspace,
                self.libraries,
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
        let matched_stop = self.matched_stop_view(instance_path, &resolved_reference.view);
        // A stop is executable only when the selected view is itself a
        // materialized terminal implementation. Treating a schematic as a black
        // box would emit an X-instance without any defining source, so the walk
        // descends and says so instead.
        let stop_boundary =
            matched_stop.is_some() && resolved_view_type.is_some_and(hierarchy_stop_view);
        // The design root is not a boundary candidate: a configuration that
        // stopped at it would netlist nothing, so a stop naming its view says
        // nothing about the root.
        let mut warnings = Vec::new();
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
            self.stage_binding(PendingExecutionBinding {
                instance_path: instance_path.clone(),
                resolved_reference: resolved_reference.clone(),
                resolved_view_type: view_type,
                materialized_binding,
                model_section: configured_model_section,
                stop_boundary,
                project_veriloga,
            });
        }

        let mut row = self.binding_row_with_master(
            resolved_reference.clone(),
            binding,
            instance_path,
            depth,
            is_root,
            master.clone(),
            status,
            used_review_fallback,
            diagnostic,
        );
        row.warnings = warnings;
        self.upsert(row, instance_path);
        if status.is_resolved() {
            self.resolved_instances += 1;
        }

        if stop_boundary {
            return;
        }

        let Some(schematic) = master.as_ref().and_then(|master| master.schematic) else {
            return;
        };
        let children = schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| {
                component
                    .library_cell
                    .clone()
                    .map(|binding| (component.name.clone(), binding))
            })
            .collect::<Vec<_>>();
        if children.is_empty() {
            return;
        }

        ancestors.push(resolved_reference.clone());
        for (instance_name, child) in &children {
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
                Ok(child_path) => self.resolve_reference(
                    requested,
                    Some(child),
                    &child_path,
                    depth + 1,
                    false,
                    ancestors,
                ),
                Err(error) => {
                    self.total_instances = self.total_instances.saturating_add(1);
                    let row = self.binding_row(
                        requested,
                        Some(child),
                        instance_path,
                        depth + 1,
                        false,
                        (HierarchyBindingStatus::Unresolved, Some(error.to_string())),
                    );
                    self.upsert(row, instance_path);
                }
            }
        }
        ancestors.pop();
    }

    fn resolve_builtin_xspice(
        &mut self,
        requested: CellViewRef,
        binding: &LibraryCellInstance,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
    ) {
        let mut status = HierarchyBindingStatus::Resolved;
        let mut diagnostic = validate_builtin_xspice_binding(binding).err();
        if diagnostic.is_some() {
            status = HierarchyBindingStatus::Unresolved;
        }

        let current_platform = crate::state::ConfigurationPlatform::current();
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
                "model section '{section}' at {instance_path} cannot be applied to a compiled built-in XSPICE device"
            ));
        }

        if status.is_resolved() {
            self.stage_binding(PendingExecutionBinding {
                instance_path: instance_path.clone(),
                resolved_reference: requested.clone(),
                resolved_view_type: ViewType::Custom,
                materialized_binding: Some(binding.clone()),
                model_section: None,
                stop_boundary: true,
                project_veriloga: None,
            });
        }

        let mut row = self.binding_row(
            requested,
            Some(binding),
            instance_path,
            depth,
            is_root,
            (status, diagnostic),
        );
        row.purpose = "built-in XSPICE code model".to_owned();
        row.view_search_order = vec!["xspice".to_owned()];
        row.stop_view = Some("xspice".to_owned());
        row.model_section.clear();
        self.upsert(row, instance_path);
        if status.is_resolved() {
            self.resolved_instances += 1;
        }
    }

    fn resolve_generated_veriloga(
        &mut self,
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
        let current_platform = crate::state::ConfigurationPlatform::current();
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
            self.stage_binding(PendingExecutionBinding {
                instance_path: instance_path.clone(),
                resolved_reference: requested.clone(),
                resolved_view_type: ViewType::VerilogA,
                materialized_binding: Some(binding.clone()),
                model_section: None,
                stop_boundary: true,
                project_veriloga: None,
            });
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
        self.upsert(row, instance_path);
        if status.is_resolved() {
            self.resolved_instances += 1;
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
            None,
            status,
            false,
            diagnostic,
        )
    }

    fn binding_row_with_master(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &InstancePath,
        depth: usize,
        is_root: bool,
        master: Option<HierarchyMaster<'_>>,
        status: HierarchyBindingStatus,
        used_review_fallback: bool,
        diagnostic: Option<String>,
    ) -> ResolvedHierarchyBinding {
        let search_order = self.view_search_order(
            binding.map_or(reference.view.as_str(), |value| value.view.as_str()),
            is_root,
            instance_path,
        );
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();
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
        let stop_view = if self.workspace.configuration_sets.active().is_some() {
            self.configured_stop_views(instance_path)
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
            model_section: self.model_section(&reference, binding, instance_path),
            reference,
            purpose: purpose.to_owned(),
            view_search_order: search_order,
            stop_view,
            status,
            instance_count: 1,
            instance_paths: vec![instance_path.to_string()],
            used_review_fallback,
            diagnostic,
            warnings: Vec::new(),
        }
    }

    fn configured_primary_views(
        &self,
        requested: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
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
        let configured = self
            .configured_override(instance_path)
            .map_or(configuration.executable_view_policy(), |scoped| {
                scoped.executable_views.as_slice()
            });
        order.extend(configured.iter().cloned());
        deduplicate_view_order(&mut order);
        order
    }

    fn view_search_order(
        &self,
        requested: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return hierarchy_view_search_order(requested, is_root);
        };
        let mut order = self.configured_primary_views(requested, is_root, instance_path);
        if configuration.definition().unresolved_policy
            == crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview
        {
            order.extend(hierarchy_view_search_order(requested, is_root));
        }
        deduplicate_view_order(&mut order);
        order
    }

    fn used_review_fallback(
        &self,
        requested: &str,
        resolved: &str,
        is_root: bool,
        instance_path: &InstancePath,
    ) -> bool {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return false;
        };
        configuration.definition().unresolved_policy
            == crate::state::UnresolvedBindingPolicy::ExplicitFallbackWithReview
            && !self
                .configured_primary_views(requested, is_root, instance_path)
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(resolved))
    }

    fn configured_stop_views(&self, instance_path: &InstancePath) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return Vec::new();
        };
        if let Some(scoped) = self.configured_override(instance_path)
            && let Some(stop_view) = &scoped.stop_view
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
        instance_path: &InstancePath,
        resolved_view: &str,
    ) -> Option<String> {
        self.configured_stop_views(instance_path)
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
            .unwrap_or_else(|| hierarchy_model_section(self.libraries, reference, binding))
    }

    fn configured_model_section(&self, instance_path: &InstancePath) -> Option<String> {
        self.configured_override(instance_path)
            .and_then(|scoped| scoped.model_section.clone())
    }

    fn configured_platform_eligible(
        &self,
        instance_path: &InstancePath,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.configured_override(instance_path)
            .is_none_or(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn configured_platform_declared(
        &self,
        instance_path: &InstancePath,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.configured_override(instance_path)
            .is_some_and(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    /// The override governing one instance, selected from the canonicalized
    /// patterns so the authored spelling cannot change which one wins.
    fn configured_override(
        &self,
        instance_path: &InstancePath,
    ) -> Option<&crate::state::ConfigurationSetOverride> {
        selected_configuration_override(&self.overrides, &instance_path.to_string())
    }

    /// Retain one resolved occurrence, in walk order, for the plan this
    /// resolution seals. A second visit to one instance is impossible — the
    /// walk descends each placed instance once — so the order vector and the
    /// map stay in step.
    fn stage_binding(&mut self, binding: PendingExecutionBinding) {
        let key = binding.instance_path.fold_key();
        if self
            .execution_bindings
            .insert(key.clone(), binding)
            .is_none()
        {
            self.execution_order.push(key);
        }
    }

    fn execution_model_section_conflict(&self) -> Option<String> {
        let mut sources = HashMap::<String, (Option<&str>, &InstancePath)>::new();
        for binding in self.execution_bindings.values() {
            let Some(materialized) = binding.materialized_binding.as_ref() else {
                continue;
            };
            let Some(source_path) = materialized.source_path.as_deref() else {
                continue;
            };
            let key = configured_source_identity(source_path);
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
    ) -> Result<Option<(HierarchyMaster<'a>, Option<CellViewRef>)>, String> {
        let library = find_library(self.libraries, &requested.library);
        let cell = library.and_then(|library| find_cell(library, &requested.cell));
        let source_bound = binding
            .and_then(|value| value.source_path.as_ref())
            .is_some();

        // Compatibility mode retains the historical placed-binding authority.
        // Configuration mode below instead materializes each selected L/C/V
        // from the authoritative library view.
        if self.workspace.configuration_sets.active().is_none() && source_bound {
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
                    let materialized_binding = binding
                        .map(|placed| materialize_schematic_binding(placed, &reference, schematic))
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
            if self.workspace.configuration_sets.active().is_some()
                && hierarchy_stop_view(view_type)
            {
                let Some(placed) = binding else {
                    return Err(format!(
                        "configuration root {} cannot materialize source view '{}' without an instance interface",
                        requested.display_path(),
                        candidate
                    ));
                };
                let materialized = materialize_authoritative_source_binding(
                    placed,
                    library.expect("view implies library"),
                    cell.expect("view implies cell"),
                    view,
                    self.workspace,
                    self.libraries,
                )?;
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

    fn find_schematic(&self, reference: &CellViewRef) -> Option<&'a SchematicState> {
        if let Some((overlay_reference, schematic)) = self.active_overlay
            && overlay_reference
                .key()
                .eq_ignore_ascii_case(&reference.key())
        {
            return Some(schematic);
        }
        find_schematic(self.workspace, reference)
    }

    fn validate_source_binding(&self, binding: &LibraryCellInstance) -> Result<(), String> {
        let Some(library) = find_library(self.libraries, &binding.library) else {
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
            ViewType::Spice | ViewType::VerilogA | ViewType::Verilog | ViewType::Extracted
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
        if view.view_type == ViewType::VerilogA
            && self.workspace.configuration_sets.active().is_some()
        {
            let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
            let project_binding =
                project_veriloga_binding_for_view(self.workspace, self.libraries, &reference)?;
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
        if !source_path.is_absolute() {
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
        if !source_paths_match(source_path, authoritative_path) {
            return Err(format!(
                "source-backed binding {}/{}/{} conflicts with the authoritative source path",
                binding.library, binding.cell, binding.view
            ));
        }
        validate_source_file(source_path, view.view_type, binding)
    }

    fn upsert(&mut self, mut row: ResolvedHierarchyBinding, occurrence: &InstancePath) {
        // Rows are grouped by the executable binding contract, not by their
        // current outcome. Repeated instances of one master must remain one
        // review row while `instance_paths` preserves every exact occurrence;
        // a recursive or unresolved occurrence then promotes the aggregate to
        // the most severe observed status. Configuration variants still split
        // naturally through their ordered views, stop, model, and fallback
        // fields below.
        let key = format!(
            "{}|{}|{}|{}|{}",
            row.reference.key().to_ascii_lowercase(),
            row.view_search_order.join(",").to_ascii_lowercase(),
            row.stop_view
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase(),
            row.model_section.to_ascii_lowercase(),
            row.used_review_fallback,
        );
        if let Some(index) = self.row_indices.get(&key).copied() {
            // Occurrences are merged by path identity, so two spellings of one
            // instance count once and the listed order is the paths' rather
            // than the order the walk happened to reach them in.
            let occurrences = &mut self.row_occurrences[index];
            occurrences.insert(occurrence.fold_key(), occurrence.to_string());
            let existing = &mut self.rows[index];
            existing.instance_count = existing.instance_count.saturating_add(row.instance_count);
            existing.instance_paths = occurrences.values().cloned().collect();
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
            return;
        }
        self.row_indices.insert(key, self.rows.len());
        self.row_occurrences.push(BTreeMap::from([(
            occurrence.fold_key(),
            occurrence.to_string(),
        )]));
        self.rows.push(row);
    }
}

/// Turn one walk's staged occurrences into the frozen execution plan.
///
/// Three things are decided here and nowhere else: each occurrence's closure
/// digest, which occurrences are one master, and what that master is called.
/// Everything downstream — the emitted deck, the receipt, the repair actions —
/// reads those three answers rather than deriving a fourth.
fn seal_plan(
    root: CellViewRef,
    pending: BTreeMap<String, PendingExecutionBinding>,
    order: &[String],
    configuration: Option<&crate::state::ConfigurationSet>,
) -> ConfigurationExecutionPlan {
    let mut children = HashMap::<String, Vec<String>>::new();
    for key in order {
        let binding = &pending[key];
        if let Some(parent) = binding.instance_path.parent() {
            children
                .entry(parent.fold_key())
                .or_default()
                .push(key.clone());
        }
    }
    let mut digests = HashMap::<String, ContentDigest>::with_capacity(pending.len());
    for key in order.iter().rev() {
        let binding = &pending[key];
        let child_digests = children
            .get(key)
            .map(|keys| {
                keys.iter()
                    .filter_map(|child| digests.get(child).copied())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        digests.insert(
            key.clone(),
            master_closure_digest(
                &binding.resolved_reference,
                binding.resolved_view_type,
                binding.model_section.as_deref(),
                binding.stop_boundary,
                binding
                    .project_veriloga
                    .as_ref()
                    .map(ConfigurationVerilogABinding::source_closure_digest),
                &child_digests,
            ),
        );
    }

    let mut bindings = BTreeMap::new();
    let mut master_order = Vec::new();
    let mut reserved = HashSet::new();
    for key in order {
        let pending = &pending[key];
        let closure = digests[key];
        let emits_subcircuit = !pending.instance_path.is_root()
            && matches!(
                pending.resolved_view_type,
                ViewType::Schematic | ViewType::Testbench
            );
        if emits_subcircuit {
            master_order.push((
                MasterKey::new(&pending.resolved_reference, closure),
                pending.resolved_reference.clone(),
                pending.instance_path.clone(),
            ));
        } else if let Some(module) = pending
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
        bindings.insert(
            key.clone(),
            ConfigurationExecutionBinding {
                instance_path: pending.instance_path.clone(),
                resolved_reference: pending.resolved_reference.clone(),
                resolved_view_type: pending.resolved_view_type,
                materialized_binding: pending.materialized_binding.clone(),
                model_section: pending.model_section.clone(),
                stop_boundary: pending.stop_boundary,
                project_veriloga: pending.project_veriloga.clone(),
                binding_closure_digest: closure,
            },
        );
    }

    let names = assign_master_names(
        master_order
            .iter()
            .map(|(key, reference, _)| (key, reference)),
        &reserved,
    );
    let mut masters = BTreeMap::<MasterKey, MasterRecord>::new();
    let mut occurrence_masters = BTreeMap::new();
    for (key, reference, occurrence) in master_order {
        masters
            .entry(key.clone())
            .or_insert_with(|| MasterRecord::new(names[&key].clone(), reference))
            .occurrences
            .push(occurrence.clone());
        occurrence_masters.insert(occurrence.fold_key(), key);
    }

    ConfigurationExecutionPlan {
        root,
        bindings,
        masters,
        occurrence_masters,
        configuration_id: configuration.map(crate::state::ConfigurationSet::id),
        configuration_revision: configuration.map_or(UNCONFIGURED_PLAN_REVISION, |configuration| {
            configuration.revision()
        }),
        configuration_digest: configuration.map_or_else(
            || {
                ContentDigest::from_bytes(
                    sha2::Sha256::digest(UNCONFIGURED_PLAN_AUTHORITY.as_bytes()).into(),
                )
            },
            |configuration| configuration.semantic_digest(),
        ),
    }
}

/// One override with its pattern re-spelled canonically.
///
/// This is the boundary the authored spelling stops at: a definition saved
/// before the design root became implicit still carries it as a segment, and a
/// pattern that names the root cannot match a path that does not.
fn canonical_override(
    scoped: &crate::state::ConfigurationSetOverride,
) -> crate::state::ConfigurationSetOverride {
    let mut scoped = scoped.clone();
    if let Ok(pattern) = InstancePathPattern::parse_legacy(&scoped.instance_path) {
        scoped.instance_path = pattern.to_string();
    }
    scoped
}

pub(super) fn find_library<'a>(libraries: &'a LibraryManager, name: &str) -> Option<&'a Library> {
    libraries
        .libraries_by_key()
        .find(|(key, library)| {
            key.eq_ignore_ascii_case(name) || library.name.eq_ignore_ascii_case(name)
        })
        .map(|(_, library)| library)
}

pub(super) fn find_cell<'a>(library: &'a Library, name: &str) -> Option<&'a Cell> {
    library
        .cells
        .iter()
        .find(|(key, cell)| key.eq_ignore_ascii_case(name) || cell.name.eq_ignore_ascii_case(name))
        .map(|(_, cell)| cell)
}

pub(super) fn find_view<'a>(cell: &'a Cell, name: &str) -> Option<&'a View> {
    cell.views
        .iter()
        .find(|(key, view)| key.eq_ignore_ascii_case(name) || view.name.eq_ignore_ascii_case(name))
        .map(|(_, view)| view)
}

pub(super) fn find_schematic<'a>(
    workspace: &'a ProjectWorkspace,
    reference: &CellViewRef,
) -> Option<&'a SchematicState> {
    workspace
        .schematic_buffers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
        .map(|(_, schematic)| schematic)
}
