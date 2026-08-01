//! Resolving a testbench hierarchy to exact Library/Cell/View bindings.
//!
//! Resolution walks the placed instances and answers one question per row:
//! which view will actually be netlisted, and did the configuration say so or
//! did the search order fall back?  A row records that distinction rather than
//! hiding it, so an unresolved or fallback binding can never be mistaken for a
//! configured one.  Nothing here mutates the workspace — the resolver borrows
//! it and returns a [`HierarchyResolution`].

use super::*;

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
    /// binding. Paths remain available for configuration review and exact
    /// override audit even when repeated masters are grouped in the table.
    pub instance_paths: Vec<String>,
    /// True when the active configuration explicitly permits and records a
    /// reviewed fallback outside its primary ordered view policy.
    pub used_review_fallback: bool,
    pub diagnostic: Option<String>,
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
    rows: Vec<ResolvedHierarchyBinding>,
    row_indices: HashMap<String, usize>,
    total_instances: usize,
    resolved_instances: usize,
    encountered_instance_paths: HashSet<String>,
    execution_bindings: BTreeMap<String, ConfigurationExecutionBinding>,
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
            rows: Vec::new(),
            row_indices: HashMap::new(),
            total_instances: 0,
            resolved_instances: 0,
            encountered_instance_paths: HashSet::new(),
            execution_bindings: BTreeMap::new(),
        }
    }

    pub(super) fn resolve(self) -> HierarchyResolution {
        self.resolve_all().0
    }

    pub(super) fn resolve_all(
        mut self,
    ) -> (HierarchyResolution, Option<ConfigurationExecutionPlan>) {
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
        let mut ancestors = Vec::new();
        if let Some(error) = active_configuration
            .and_then(|configuration| validate_override_pattern_authority(configuration).err())
        {
            self.total_instances = 1;
            let row = self.binding_row(
                root.clone(),
                None,
                "/top",
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row);
        } else {
            self.resolve_reference(root.clone(), None, "/top", 0, true, &mut ancestors);
        }
        for (path, purpose) in required_paths {
            let matched = if path.contains('*') {
                self.encountered_instance_paths
                    .iter()
                    .any(|candidate| instance_path_pattern_matches(&path, candidate))
            } else {
                self.encountered_instance_paths
                    .contains(&path.to_ascii_lowercase())
            };
            if !matched {
                self.encountered_instance_paths
                    .insert(path.to_ascii_lowercase());
                self.total_instances = self.total_instances.saturating_add(1);
                let row = self.binding_row(
                    root.clone(),
                    None,
                    &path,
                    1,
                    false,
                    (
                        HierarchyBindingStatus::Unresolved,
                        Some(format!(
                            "{purpose} {path} does not exist in the expanded hierarchy"
                        )),
                    ),
                );
                self.upsert(row);
            }
        }
        if let Some(error) = self.execution_model_section_conflict() {
            self.total_instances = self.total_instances.saturating_add(1);
            let row = self.binding_row(
                root.clone(),
                None,
                "/top",
                0,
                true,
                (HierarchyBindingStatus::Unresolved, Some(error)),
            );
            self.upsert(row);
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
        let plan = active_configuration.map(|configuration| ConfigurationExecutionPlan {
            root,
            bindings: self.execution_bindings,
            configuration_id: configuration.id(),
            configuration_revision: configuration.revision(),
            configuration_digest: configuration.semantic_digest(),
        });
        (resolution, plan)
    }

    fn resolve_reference(
        &mut self,
        requested: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
        depth: usize,
        is_root: bool,
        ancestors: &mut Vec<CellViewRef>,
    ) {
        self.encountered_instance_paths
            .insert(instance_path.to_ascii_lowercase());
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
            self.upsert(row);
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
            self.upsert(row);
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
            self.upsert(row);
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

        let stop_boundary = master
            .as_ref()
            .and_then(|value| value.view_type)
            .is_some_and(|view_type| self.stops_at(instance_path, Some(view_type)));
        if status.is_resolved()
            && self.workspace.configuration_sets.active().is_some()
            && let Some(master) = master.as_ref()
            && let Some(view_type) = master.view_type
        {
            let mut materialized_binding = if is_root {
                None
            } else {
                master.materialized_binding.clone()
            };
            if let Some(materialized) = materialized_binding.as_mut()
                && matches!(view_type, ViewType::Schematic | ViewType::Testbench)
            {
                materialized.module_name = Some(configured_subcircuit_name(
                    &resolved_reference,
                    instance_path,
                ));
            }
            self.execution_bindings.insert(
                instance_path.to_ascii_lowercase(),
                ConfigurationExecutionBinding {
                    instance_path: instance_path.to_owned(),
                    resolved_reference: resolved_reference.clone(),
                    resolved_view_type: view_type,
                    materialized_binding,
                    model_section: configured_model_section,
                    stop_boundary,
                    project_veriloga,
                },
            );
        }

        let row = self.binding_row_with_master(
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
        self.upsert(row);
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
            let child_path = format!("{instance_path}/{instance_name}");
            self.resolve_reference(
                CellViewRef::new(&child.library, &child.cell, requested_view),
                Some(child),
                &child_path,
                depth + 1,
                false,
                ancestors,
            );
        }
        ancestors.pop();
    }

    fn binding_row(
        &self,
        reference: CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
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
        instance_path: &str,
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
        let stop_view = if self.workspace.configuration_sets.active().is_some() {
            self.configured_stop_views(instance_path)
                .into_iter()
                .find(|stop| {
                    terminal_view.is_some_and(|view_type| {
                        view_type.display_name().eq_ignore_ascii_case(stop)
                    }) || search_order
                        .iter()
                        .any(|candidate| candidate.eq_ignore_ascii_case(stop))
                })
        } else if is_root {
            None
        } else {
            terminal_view
                .map(|view_type| view_type.display_name().to_owned())
                .or_else(|| {
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
            instance_paths: vec![instance_path.to_owned()],
            used_review_fallback,
            diagnostic,
        }
    }

    fn configured_primary_views(
        &self,
        requested: &str,
        is_root: bool,
        instance_path: &str,
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
        let configured = selected_configuration_override(configuration.overrides(), instance_path)
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
        instance_path: &str,
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
        instance_path: &str,
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

    fn configured_stop_views(&self, instance_path: &str) -> Vec<String> {
        let Some(configuration) = self.workspace.configuration_sets.active() else {
            return Vec::new();
        };
        if let Some(scoped) =
            selected_configuration_override(configuration.overrides(), instance_path)
            && let Some(stop_view) = &scoped.stop_view
        {
            return vec![stop_view.clone()];
        }
        configuration.stop_views().to_vec()
    }

    fn stops_at(&self, instance_path: &str, resolved_view: Option<ViewType>) -> bool {
        let Some(resolved_view) = resolved_view else {
            return false;
        };
        // A stop is executable only when the selected view is itself a
        // materialized terminal implementation.  Treating a schematic as a
        // black box would emit an X-instance without any defining source.
        if !hierarchy_stop_view(resolved_view) {
            return false;
        }
        self.configured_stop_views(instance_path)
            .iter()
            .any(|stop| resolved_view.display_name().eq_ignore_ascii_case(stop))
    }

    fn model_section(
        &self,
        reference: &CellViewRef,
        binding: Option<&LibraryCellInstance>,
        instance_path: &str,
    ) -> String {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
                    .and_then(|scoped| scoped.model_section.clone())
            })
            .unwrap_or_else(|| hierarchy_model_section(self.libraries, reference, binding))
    }

    fn configured_model_section(&self, instance_path: &str) -> Option<String> {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .and_then(|scoped| scoped.model_section.clone())
    }

    fn configured_platform_eligible(
        &self,
        instance_path: &str,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .is_none_or(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn configured_platform_declared(
        &self,
        instance_path: &str,
        platform: crate::state::ConfigurationPlatform,
    ) -> bool {
        self.workspace
            .configuration_sets
            .active()
            .and_then(|configuration| {
                selected_configuration_override(configuration.overrides(), instance_path)
            })
            .is_some_and(|scoped| scoped.eligible_platforms.contains(&platform))
    }

    fn execution_model_section_conflict(&self) -> Option<String> {
        let mut sources = HashMap::<String, (Option<&str>, &str)>::new();
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
                sources.insert(key, (section, binding.instance_path()));
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

    fn upsert(&mut self, row: ResolvedHierarchyBinding) {
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
            let existing = &mut self.rows[index];
            existing.instance_count = existing.instance_count.saturating_add(row.instance_count);
            existing.instance_paths.extend(row.instance_paths);
            existing
                .instance_paths
                .sort_by_key(|path| path.to_ascii_lowercase());
            existing
                .instance_paths
                .dedup_by(|left, right| left.eq_ignore_ascii_case(right));
            if row.status.severity() > existing.status.severity() {
                existing.status = row.status;
                existing.diagnostic = row.diagnostic;
            }
            return;
        }
        self.row_indices.insert(key, self.rows.len());
        self.rows.push(row);
    }
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
