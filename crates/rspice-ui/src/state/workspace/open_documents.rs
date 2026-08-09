//! The open workspace: what is active, what is dirty, what the hierarchy
//! resolves to.
//!
//! Navigation and buffer management over [`ProjectWorkspace`] — opening a
//! cell view, keeping its schematic buffer in sync, resolving the design
//! hierarchy against the library, and the per-project hardcopy setup that
//! rides along with it.
//!
//! Separate from the configuration and plan half in the parent, which
//! describes what a run *will* do; this describes what a session currently
//! has open.

use super::*;

impl ProjectWorkspace {
    /// Exact testbench root selected for simulation. Legacy projects without
    /// configuration sets retain their project descriptor root.
    pub fn simulation_root_reference(&self) -> CellViewRef {
        self.configuration_sets.active().map_or_else(
            || {
                CellViewRef::new(
                    &self.project.root_library,
                    &self.project.top_cell,
                    DEFAULT_SCHEMATIC_VIEW,
                )
            },
            |configuration| configuration.root().clone(),
        )
    }

    /// Resolve the exact root schematic while projecting the live editor only
    /// when it is the selected root. A different open tab can never silently
    /// replace the configuration's simulation source.
    pub fn simulation_root_schematic<'a>(
        &'a self,
        active_reference: &CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Option<&'a SchematicState> {
        let root = self.simulation_root_reference();
        if root.key().eq_ignore_ascii_case(&active_reference.key()) {
            Some(active_schematic)
        } else {
            find_schematic(self, &root)
        }
    }

    /// Bind the exact active hierarchy configuration into generated source.
    /// The SPICE comment is part of the executable bytes and therefore flows
    /// into source, snapshot, and retained-run digests without relying on
    /// mutable UI state or a side-channel receipt.
    pub fn bind_generated_netlist_provenance(&self, mut source: String) -> String {
        let insertion = source.find('\n').map_or(0, |index| index + 1);
        let mut provenance = self
            .design_management
            .semantic_digest()
            .map(|digest| format!("* RSpice design-management digest {digest}\n"))
            .unwrap_or_else(|error| format!("* RSpice design-management INVALID ({error})\n"));
        if let Some(configuration) = self.configuration_sets.active() {
            provenance.push_str(&format!(
                "* RSpice configuration-set {} revision {} digest {}\n",
                configuration.id(),
                configuration.revision(),
                configuration.semantic_digest()
            ));
        }
        source.insert_str(insertion, &provenance);
        source
    }

    /// Publish an independently mutated catalog and the owning project
    /// revision as one fail-closed transaction. Runtime invalidation uses the
    /// dirty flag while persistent lifecycle hashing authenticates the exact
    /// catalog bytes.
    pub fn replace_configuration_sets(
        &mut self,
        candidate: crate::state::ConfigurationSetCatalog,
    ) -> Result<ObjectRevision, ProjectConfigurationMutationError> {
        candidate.validate()?;
        for configuration in candidate.configurations() {
            let root = configuration.root();
            if !matches!(
                ViewType::from_name(&root.view),
                ViewType::Schematic | ViewType::Testbench
            ) {
                return Err(ProjectConfigurationMutationError::UnsupportedRootView {
                    configuration: configuration.name().to_owned(),
                    root: root.display_path(),
                });
            }
            if !self
                .schematic_buffers
                .keys()
                .any(|key| key.eq_ignore_ascii_case(&root.key()))
            {
                return Err(ProjectConfigurationMutationError::MissingRootBuffer {
                    configuration: configuration.name().to_owned(),
                    root: root.display_path(),
                });
            }
        }
        if candidate == self.configuration_sets {
            return Err(ProjectConfigurationMutationError::NoChanges);
        }
        let next_revision = self.project.revision.next()?;
        self.configuration_sets = candidate;
        self.project.revision = next_revision;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Publish a complete design-management candidate and its owning project
    /// revision atomically. Validation happens before any live state changes;
    /// failed candidates therefore cannot partially alter sheet, variant,
    /// annotation, or hierarchy-audit authority.
    pub fn replace_design_management(
        &mut self,
        candidate: crate::state::DesignManagementCatalog,
    ) -> Result<ObjectRevision, ProjectConfigurationMutationError> {
        candidate.validate().map_err(|source| {
            ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                message: source.to_string(),
            }
        })?;
        let mut published = self.design_management.clone();
        published
            .publish_reviewed_candidate(self.design_management.revision(), candidate)
            .map_err(|source| {
                ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                    message: source.to_string(),
                }
            })?;
        let next_revision = self.project.revision.next()?;
        self.design_management = published;
        self.project.revision = next_revision;
        self.project_metadata_dirty = true;
        Ok(next_revision)
    }

    /// Bind newly authored schematic objects to the currently active sheet.
    /// Legacy projects with no sheet catalog remain untouched; once the
    /// user enters multi-sheet authoring, every later object receives durable
    /// membership at the same save/sync boundary as its schematic edit.
    pub fn assign_unowned_objects_to_active_sheet(
        &mut self,
        reference: &CellViewRef,
        schematic: &SchematicState,
    ) -> Result<bool, ProjectConfigurationMutationError> {
        let key = reference.key();
        let Some(catalog) = self.design_management.sheet_catalog(&key) else {
            return Ok(false);
        };
        let Some(active_sheet_id) = catalog.active_sheet_id() else {
            return Ok(false);
        };
        let live_object_ids = schematic
            .components
            .iter()
            .map(|object| object.id)
            .chain(schematic.wires.iter().map(|object| object.id))
            .chain(schematic.buses.iter().map(|object| object.id))
            .chain(schematic.bus_taps.iter().map(|object| object.id))
            .chain(schematic.junctions.iter().map(|object| object.id))
            .chain(schematic.net_labels.iter().map(|object| object.id))
            .chain(schematic.design_notes.iter().map(|object| object.id))
            .chain(
                schematic
                    .documentation_shapes
                    .iter()
                    .map(|object| object.id),
            )
            .chain(schematic.probes.iter().map(|object| object.id))
            .collect::<Vec<_>>();

        let mut candidate = self.design_management.clone();
        let catalog = candidate
            .sheet_catalog_mut(&key)
            .expect("the cloned catalog retains the validated cell/view key");
        let receipt = catalog
            .reconcile_object_assignments(
                catalog.revision(),
                live_object_ids,
                Some(active_sheet_id),
            )
            .map_err(|source| {
                ProjectConfigurationMutationError::InvalidDesignManagementCatalog {
                    message: source.to_string(),
                }
            })?;
        if receipt.added_assignments == 0
            && receipt.removed_assignments == 0
            && receipt.removed_cross_sheet_ports == 0
        {
            return Ok(false);
        }
        self.replace_design_management(candidate)?;
        Ok(true)
    }

    /// Create a new default project and ensure its editable top cell exists in
    /// the shared library manager.
    pub fn new_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let verilog_a = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
            ProjectSourceLanguage::VerilogA,
                "sensor_bridge.va",
                "`include \"constants.vams\"\nmodule sensor_bridge(out, inp, inn);\n  parameter real gain = 100.0 from (0:inf);\n  analog V(out) <+ gain * (V(inp)-V(inn));\nendmodule",
            [],
            [],
        )
        .expect("the built-in Verilog-A example is valid");
        let automation = crate::state::ProjectSourceBundle::try_new_with_roles(
            crate::state::ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::RSpiceAutomation,
            ),
            ProjectSourceLanguage::RSpiceAutomation,
            crate::automation_workflow::AutomationStarterFile::PythonEntry.path(),
            crate::automation_workflow::DEFAULT_AUTOMATION_PYTHON,
            [
                crate::state::ProjectSourceFile::try_new(
                    crate::automation_workflow::AutomationStarterFile::RunPlan.path(),
                    crate::automation_workflow::DEFAULT_AUTOMATION_RUN_PLAN,
                )
                .expect("the built-in run plan is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::automation_workflow::AutomationStarterFile::EnvironmentLock.path(),
                    crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK,
                )
                .expect("the built-in environment lock is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::automation_workflow::AutomationStarterFile::Permissions.path(),
                    crate::automation_workflow::DEFAULT_AUTOMATION_PERMISSIONS,
                )
                .expect("the built-in permission manifest is valid"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    crate::automation_workflow::AutomationStarterFile::PythonEntry.path(),
                    crate::automation_workflow::AutomationStarterFile::RunPlan.path(),
                )
                .expect("the run-plan dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::automation_workflow::AutomationStarterFile::PythonEntry.path(),
                    crate::automation_workflow::AutomationStarterFile::EnvironmentLock.path(),
                )
                .expect("the environment-lock dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::automation_workflow::AutomationStarterFile::PythonEntry.path(),
                    crate::automation_workflow::AutomationStarterFile::Permissions.path(),
                )
                .expect("the permission-manifest dependency is valid"),
            ],
            [
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::automation_workflow::AutomationStarterFile::PythonEntry.path(),
                    crate::state::ProjectSourceRole::AutomationEntry,
                )
                .expect("the Automation entry role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::automation_workflow::AutomationStarterFile::RunPlan.path(),
                    crate::state::ProjectSourceRole::AutomationRunPlan,
                )
                .expect("the Automation run-plan role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::automation_workflow::AutomationStarterFile::EnvironmentLock.path(),
                    crate::state::ProjectSourceRole::AutomationEnvironmentLock,
                )
                .expect("the Automation environment-lock role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::automation_workflow::AutomationStarterFile::Permissions.path(),
                    crate::state::ProjectSourceRole::AutomationPermissionManifest,
                )
                .expect("the Automation permission role is valid"),
            ],
        )
        .expect("the built-in Automation workspace is valid");
        let mut project_sources = ProjectSourceRegistry::try_from_bundles([verilog_a, automation])
            .expect("the bootstrapped Code source registry is valid");
        // The canonical Verilog-A fixture is compiled during bootstrap. Python
        // Automation is intentionally left unvalidated: only the exact
        // packaged CPython worker may create that receipt.
        project_sources
            .mark_validated(ProjectSourceLanguage::VerilogA)
            .expect("the built-in Verilog-A identity is valid");
        let mut workspace = Self {
            project_sources,
            ..Self::default()
        };
        workspace.ensure_library_model(libraries);
        workspace
    }

    /// Create a genuinely empty user project. The canonical startup fixture
    /// keeps the mockup's example sources, while File > New must not force an
    /// unrelated circuit to compile or execute demonstration code.
    pub fn new_empty_bootstrapped(libraries: &mut LibraryManager) -> Self {
        let mut workspace = Self::new_bootstrapped(libraries);
        workspace.project_sources = ProjectSourceRegistry::default();
        workspace.project_sources_dirty = false;
        workspace
    }

    /// Resolve the complete executable library/cell/view closure rooted at the
    /// project testbench. Open tabs are intentionally irrelevant: this receipt
    /// follows placed hierarchical instances and the same schematic/source
    /// ownership used by netlisting.
    pub fn resolve_hierarchy(&self, libraries: &LibraryManager) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, None).resolve()
    }

    /// Resolve the hierarchy while projecting the live editor buffer over its
    /// persisted workspace copy. Rendering and validation use this form so a
    /// just-placed instance cannot disappear from the receipt until save or a
    /// view switch.
    pub fn resolve_hierarchy_with_active<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> HierarchyResolution {
        HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
            .resolve()
    }

    /// Materialize the exact multi-sheet, active-variant, and annotation
    /// projection consumed by DRC and netlisting. Authored canvas coordinates
    /// stay local to each sheet; the execution clone namespaces them by sheet
    /// order so coincident coordinates on different pages cannot create an
    /// accidental electrical connection. Explicit cross-sheet port contracts
    /// are then materialized as identically named labels at both endpoints.
    pub(super) fn materialize_design_management_schematic(
        &self,
        cell_view_key: &str,
        source: &SchematicState,
    ) -> Result<SchematicState, crate::state::DesignManagementError> {
        self.design_management.validate()?;
        let mut projected = source.clone();

        if let Some(catalog) = self.design_management.sheet_catalog(cell_view_key) {
            let offsets = catalog
                .sheets()
                .iter()
                .enumerate()
                .map(|(index, sheet)| {
                    let ordinal = i32::try_from(index).unwrap_or(i32::MAX);
                    (
                        sheet.id(),
                        crate::state::Point::new(ordinal.saturating_mul(1_000_000), 0),
                    )
                })
                .collect::<HashMap<_, _>>();
            let offset_for = |object_id: u64| {
                self.design_management
                    .sheet_for_object_or_active(cell_view_key, object_id)
                    .and_then(|sheet_id| offsets.get(&sheet_id).copied())
                    .unwrap_or_else(crate::state::Point::origin)
            };

            for component in &mut projected.components {
                component.pos = translated_point(component.pos, offset_for(component.id))?;
            }
            for wire in &mut projected.wires {
                let delta = offset_for(wire.id);
                for point in &mut wire.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for bus in &mut projected.buses {
                let delta = offset_for(bus.id);
                for point in &mut bus.points {
                    *point = translated_point(*point, delta)?;
                }
            }
            for tap in &mut projected.bus_taps {
                let delta = offset_for(tap.id);
                tap.bus_point = translated_point(tap.bus_point, delta)?;
                tap.connection_point = translated_point(tap.connection_point, delta)?;
            }
            for junction in &mut projected.junctions {
                junction.pos = translated_point(junction.pos, offset_for(junction.id))?;
            }
            for label in &mut projected.net_labels {
                label.pos = translated_point(label.pos, offset_for(label.id))?;
            }
            for note in &mut projected.design_notes {
                note.pos = translated_point(note.pos, offset_for(note.id))?;
            }
            for shape in &mut projected.documentation_shapes {
                let delta = offset_for(shape.id);
                let (minimum, maximum) = shape.bounds();
                let _ = translated_point(minimum, delta)?;
                let _ = translated_point(maximum, delta)?;
                shape.translate(delta);
            }

            for contract in catalog.cross_sheet_ports() {
                for endpoint in [&contract.definition().first, &contract.definition().second] {
                    if catalog.sheet_for_object(endpoint.object_id()) != Some(endpoint.sheet_id) {
                        return Err(crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet anchor sheet assignment",
                            identity: endpoint.object_id().to_string(),
                        });
                    }
                    let delta = offsets.get(&endpoint.sheet_id).copied().ok_or_else(|| {
                        crate::state::DesignManagementError::MissingReference {
                            domain: "cross-sheet port sheet",
                            identity: endpoint.sheet_id.to_string(),
                        }
                    })?;
                    let anchor = projected_cross_sheet_anchor(source, &projected, endpoint, delta)?;
                    let next_id = projected.next_id();
                    projected.net_labels.push(crate::state::NetLabel::new(
                        next_id,
                        anchor,
                        contract.definition().net_name.clone(),
                    ));
                }
            }
        }

        if let Some(active_variant) = self.design_management.variants().active() {
            let resolved = self
                .design_management
                .variants()
                .resolve(active_variant.id())?;
            let mut do_not_populate = HashSet::new();
            for component in &mut projected.components {
                let Some(override_value) = resolved.override_for(cell_view_key, component.id)?
                else {
                    continue;
                };
                match override_value {
                    crate::state::VariantObjectOverride::DoNotPopulate { .. } => {
                        do_not_populate.insert(component.id);
                    }
                    crate::state::VariantObjectOverride::Substitute { replacement } => {
                        let prior = component.library_cell.take();
                        let mut binding = crate::state::LibraryCellInstance::new(
                            replacement.library.clone(),
                            replacement.cell.clone(),
                            replacement.view.clone(),
                        );
                        if let Some(prior) = prior {
                            binding.terminal_order = prior.terminal_order;
                            binding.terminal_dirs = prior.terminal_dirs;
                            binding.interface_bound = prior.interface_bound;
                        }
                        component.kind = crate::state::ComponentType::CellInstance;
                        component.library_cell = Some(binding);
                        if let Some(value) = &replacement.value_override {
                            component.value.clone_from(value);
                        }
                        if let Some(section) = &replacement.model_section {
                            let mut params = crate::state::parse_params_string(&component.params);
                            params.insert("model_section".to_owned(), section.clone());
                            component.params = crate::state::format_params_string(&params);
                        }
                    }
                }
            }
            if !do_not_populate.is_empty() {
                projected
                    .components
                    .retain(|component| !do_not_populate.contains(&component.id));
                projected
                    .connections
                    .retain(|connection| !do_not_populate.contains(&connection.component_id));
            }
        }

        for component in &mut projected.components {
            if let Some(mapping) = self
                .design_management
                .annotation()
                .effective_mapping_for(cell_view_key, component.id)?
            {
                component.name.clone_from(&mapping.new_reference);
            }
        }
        projected.recalculate_runtime_state();
        Ok(projected)
    }

    /// Freeze the live editor projection and the active configuration's
    /// exact-path execution plan as one immutable value.  Legacy projects
    /// return the same projected buffers with no plan, preserving the
    /// historical generator path.
    pub fn configuration_execution_projection<'a>(
        &'a self,
        libraries: &'a LibraryManager,
        active_reference: &'a CellViewRef,
        active_schematic: &'a SchematicState,
    ) -> Result<ConfigurationExecutionProjection, ConfigurationExecutionPlanError> {
        let root = self.simulation_root_reference();
        let (resolution, plan) = if self.configuration_sets.active().is_some() {
            HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
                .resolve_all()
        } else {
            (
                HierarchyResolver::new(self, libraries, Some((active_reference, active_schematic)))
                    .resolve(),
                None,
            )
        };
        if self.configuration_sets.active().is_some() && !resolution.is_valid() {
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

        let mut schematic_buffers = self.schematic_buffers.clone();
        if let Some(existing_key) = schematic_buffers
            .keys()
            .find(|key| key.eq_ignore_ascii_case(&active_reference.key()))
            .cloned()
        {
            schematic_buffers.insert(existing_key, active_schematic.clone());
        } else {
            schematic_buffers.insert(active_reference.key(), active_schematic.clone());
        }
        for (key, schematic) in &mut schematic_buffers {
            *schematic = self
                .materialize_design_management_schematic(key, schematic)
                .map_err(|error| {
                    ConfigurationExecutionPlanError::DesignManagement(error.to_string())
                })?;
        }
        let projection = ConfigurationExecutionProjection {
            root,
            schematic_buffers,
            plan,
            connectivity: self.connectivity.clone(),
        };
        if projection.root_schematic().is_none() {
            return Err(ConfigurationExecutionPlanError::MissingRoot(
                projection.root.display_path(),
            ));
        }
        Ok(projection)
    }

    /// Ensure the workspace's top library/cell/view exists in the library tree.
    pub fn ensure_library_model(&mut self, libraries: &mut LibraryManager) {
        ensure_project_library(libraries);

        if self.active_view.library.is_empty() {
            self.active_view.library = self.project.root_library.clone();
        }
        if self.active_view.cell.is_empty() {
            self.active_view.cell = self.project.top_cell.clone();
        }
        if self.active_view.view.is_empty() {
            self.active_view.view = DEFAULT_SCHEMATIC_VIEW.to_string();
        }

        let active_view_type = self
            .open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .or_else(|| library_view_type(libraries, &self.active_view))
            .unwrap_or(ViewType::Schematic);

        ensure_cell_view(
            libraries,
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
            active_view_type,
        );

        if self.open_views.is_empty() {
            self.open_views.push(OpenCellView::new(
                self.active_view.clone(),
                active_view_type,
            ));
        }
        if self.hierarchy_stack.is_empty() {
            self.hierarchy_stack.push(self.active_view.clone());
        }

        if is_schematic_like(active_view_type) {
            self.ensure_active_buffer();
        }
        libraries.select_view(
            &self.active_view.library,
            &self.active_view.cell,
            &self.active_view.view,
        );
    }

    pub fn active_key(&self) -> String {
        self.active_view.key()
    }

    pub fn active_display_path(&self) -> String {
        self.active_view.display_path()
    }

    pub fn active_view_type(&self) -> ViewType {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| open.view_type)
            .unwrap_or(ViewType::Schematic)
    }

    pub fn ensure_active_buffer(&mut self) {
        let key = self.active_key();
        self.schematic_buffers.entry(key).or_default();
    }

    pub fn active_schematic(&self) -> Option<&SchematicState> {
        self.schematic_buffers.get(&self.active_key())
    }

    pub fn active_schematic_reference(&self) -> CellViewRef {
        if self.active_view_type() == ViewType::Symbol {
            return CellViewRef::new(
                &self.active_view.library,
                &self.active_view.cell,
                DEFAULT_SCHEMATIC_VIEW,
            );
        }
        self.active_view.clone()
    }

    pub fn active_context_schematic(&self) -> Option<&SchematicState> {
        let reference = self.active_schematic_reference();
        self.schematic_buffers.get(&reference.key())
    }

    pub fn save_active_schematic(&mut self, schematic: &SchematicState) {
        if !is_schematic_like(self.active_view_type()) {
            return;
        }
        let key = self.active_key();
        self.schematic_buffers.insert(key, schematic.clone());
        self.set_active_dirty(schematic.is_dirty);
    }

    pub fn mark_all_clean(&mut self) {
        for view in &mut self.open_views {
            view.dirty = false;
        }
        for schematic in self.schematic_buffers.values_mut() {
            schematic.is_dirty = false;
        }
        self.netlist_source_dirty = false;
        self.project_sources_dirty = false;
        self.project_metadata_dirty = false;
        self.report_documents_dirty = false;
        self.visualization_documents_dirty = false;
        self.hardcopy_setups_dirty = false;
        self.hardcopy_receipts_dirty = false;
        self.project_print_mappings_dirty = false;
        self.hardcopy_source_sets_dirty = false;
    }

    pub fn any_dirty(&self) -> bool {
        self.open_views.iter().any(|view| view.dirty)
            || self
                .schematic_buffers
                .values()
                .any(|schematic| schematic.is_dirty)
            || self.netlist_source_dirty
            || self.project_sources_dirty
            || self.project_metadata_dirty
            || self.report_documents_dirty
            || self.visualization_documents_dirty
            || self.hardcopy_setups_dirty
            || self.hardcopy_receipts_dirty
            || self.project_print_mappings_dirty
            || self.hardcopy_source_sets_dirty
    }

    /// Commit a validated page setup through the project dirty lifecycle.
    /// Re-saving byte-identical settings is a no-op and does not manufacture
    /// an unsaved project change.
    pub fn save_hardcopy_setup(
        &mut self,
        source: &crate::hardcopy::ActiveHardcopySource,
        setup: crate::hardcopy::HardcopySetup,
    ) -> Result<crate::hardcopy::SetupSaveOutcome, crate::hardcopy::HardcopyError> {
        let outcome = self.hardcopy_setups.save(source, setup)?;
        if outcome.disposition() != crate::hardcopy::SetupSaveDisposition::Unchanged {
            self.hardcopy_setups_dirty = true;
        }
        Ok(outcome)
    }

    /// Append one sealed publication outcome to the bounded project ledger.
    /// This is the only path from runtime hardcopy execution into durable
    /// project evidence.
    pub fn record_hardcopy_receipt(
        &mut self,
        receipt: crate::hardcopy::HardcopyReceipt,
    ) -> Result<(), crate::hardcopy::HardcopyError> {
        self.hardcopy_receipts.append(receipt)?;
        self.hardcopy_receipts_dirty = true;
        Ok(())
    }

    /// Persist a reusable project print-set mapping through the same project
    /// dirty lifecycle as document page setups.
    pub fn save_project_print_mapping(
        &mut self,
        table: crate::hardcopy::PrintMappingTable,
    ) -> Result<
        crate::hardcopy::PrintMappingSaveReceipt,
        crate::hardcopy::PrintMappingPersistenceError,
    > {
        let outcome = self.project_print_mappings.save(table)?;
        if outcome.disposition() != crate::hardcopy::PrintMappingSaveDisposition::Unchanged {
            self.project_print_mappings_dirty = true;
        }
        Ok(outcome)
    }

    #[must_use]
    pub fn hardcopy_source_sets(&self) -> &[crate::hardcopy::sources::HardcopySourceSet] {
        &self.hardcopy_source_sets
    }

    #[must_use]
    pub fn hardcopy_source_set(
        &self,
        source_key: &str,
    ) -> Option<&crate::hardcopy::sources::HardcopySourceSet> {
        self.hardcopy_source_sets
            .iter()
            .find(|source_set| source_set.source_key() == source_key)
    }

    /// Insert or replace one exact source-set definition as a small,
    /// validated transaction. This never clones the rest of the project.
    pub fn save_hardcopy_source_set(
        &mut self,
        source_set: crate::hardcopy::sources::HardcopySourceSet,
    ) -> Result<bool, HardcopySourceSetPersistenceError> {
        source_set
            .validate()
            .map_err(|error| HardcopySourceSetPersistenceError::Invalid {
                message: error.to_string(),
            })?;
        let source_key = source_set.source_key();
        if let Some(existing) = self
            .hardcopy_source_sets
            .iter()
            .find(|existing| existing.source_key() == source_key)
            && existing == &source_set
        {
            return Ok(false);
        }
        let mut candidate = self.hardcopy_source_sets.clone();
        if let Some(index) = candidate
            .iter()
            .position(|existing| existing.source_key() == source_key)
        {
            candidate[index] = source_set;
        } else {
            candidate.push(source_set);
        }
        validate_hardcopy_source_set_catalog(&candidate)?;
        self.hardcopy_source_sets = candidate;
        self.hardcopy_source_sets_dirty = true;
        Ok(true)
    }

    /// Remove one retained aggregate by its stable source identity.
    pub fn remove_hardcopy_source_set(&mut self, source_key: &str) -> bool {
        let before = self.hardcopy_source_sets.len();
        self.hardcopy_source_sets
            .retain(|source_set| source_set.source_key() != source_key);
        let removed = self.hardcopy_source_sets.len() != before;
        self.hardcopy_source_sets_dirty |= removed;
        removed
    }

    pub fn attach_technology(
        &mut self,
        binding: ProjectTechnologyBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        self.validate_physical_layout_technology_change(&binding)?;
        let before = self.project.revision();
        let revision = self.project.attach_technology(binding)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok(revision)
    }

    pub fn attach_technology_audited(
        &mut self,
        binding: ProjectTechnologyBinding,
        context: ProjectTechnologyChangeContext,
    ) -> Result<(ObjectRevision, ProjectTechnologyChangeReceipt), ProjectDescriptorError> {
        self.validate_physical_layout_technology_change(&binding)?;
        let before = self.project.revision();
        let (revision, receipt) = self.project.attach_technology_audited(binding, context)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok((revision, receipt))
    }

    /// Record the cloud circuit this project publishes to.
    pub fn bind_cloud_publication(
        &mut self,
        binding: ProjectCloudPublicationBinding,
    ) -> Result<ObjectRevision, ProjectDescriptorError> {
        let before = self.project.revision();
        let revision = self.project.bind_cloud_publication(binding)?;
        if revision != before {
            self.project_metadata_dirty = true;
        }
        Ok(revision)
    }

    fn validate_physical_layout_technology_change(
        &self,
        binding: &ProjectTechnologyBinding,
    ) -> Result<(), ProjectDescriptorError> {
        if self.physical_layout_documents().is_empty() {
            return Ok(());
        }
        let pin = binding.signed_package();
        for document in self.physical_layout_documents().values() {
            let technology = document.technology();
            let matches = pin.is_some_and(|pin| {
                technology.package_id() == pin.package_id()
                    && technology.revision() == pin.revision()
                    && technology.manifest_digest() == pin.manifest_digest()
                    && technology.archive_digest() == pin.archive_digest()
                    && technology.stack_id() == pin.stack_name()
            });
            if !matches {
                return Err(
                    ProjectDescriptorError::TechnologyConflictsWithPhysicalLayout {
                        owner: document.owner().display_path(),
                    },
                );
            }
        }
        Ok(())
    }

    pub fn set_netlist_source_dirty(&mut self, dirty: bool) {
        self.netlist_source_dirty = dirty;
    }

    /// Add a source document to a legacy/empty project and enter the ordinary
    /// project dirty lifecycle. Duplicate language identities are rejected.
    pub fn add_project_source(
        &mut self,
        document: ProjectSourceDocument,
    ) -> Result<(), ProjectSourceError> {
        self.project_sources.insert(document)?;
        self.project_sources_dirty = true;
        Ok(())
    }

    /// Replace exact source bytes and enter the ordinary project dirty
    /// lifecycle. An unchanged write is a no-op and retains validation.
    pub fn replace_project_source(
        &mut self,
        language: ProjectSourceLanguage,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self.project_sources.replace_content(language, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Replace one exact document in a project-owned source closure and enter
    /// the same persisted dirty lifecycle as root-document edits.
    pub fn replace_project_source_bundle_file(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        logical_path: &str,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .replace_bundle_file_content(bundle_id, logical_path, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Commit a workspace-wide replacement as one persisted source-graph
    /// transaction. Partial replacement is never observable.
    pub fn replace_project_source_bundle_files_transactionally(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        replacements: impl IntoIterator<Item = (String, String)>,
    ) -> Result<usize, ProjectSourceError> {
        let changed = self
            .project_sources
            .replace_bundle_files_transactionally(bundle_id, replacements)?;
        if changed > 0 {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Add one project-owned source document and its authenticated dependency
    /// edge as a single dirty-state transaction.
    pub fn add_project_source_bundle_file(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        importer_path: &str,
        file: crate::state::ProjectSourceFile,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .add_bundle_file(bundle_id, importer_path, file)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Persist a source document and its semantic role as one authenticated
    /// source-graph revision.
    pub fn add_project_source_bundle_file_with_role(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        importer_path: &str,
        file: crate::state::ProjectSourceFile,
        role: crate::state::ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .add_bundle_file_with_role(bundle_id, importer_path, file, role)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    pub fn append_project_source_qualification(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        record: crate::state::ProjectSourceQualificationRecord,
    ) -> Result<u64, ProjectSourceError> {
        let sequence = self
            .project_sources
            .append_bundle_qualification(bundle_id, record)?;
        self.project_sources_dirty = true;
        Ok(sequence)
    }

    /// Rename a project-owned source while atomically migrating its roles,
    /// dependency edges, and language-specific include references.
    pub fn rename_project_source_bundle_file(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        current_path: &str,
        new_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .rename_bundle_file(bundle_id, current_path, new_path)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Remove a non-root project-owned source only after the bundle's role and
    /// dependency invariants accept the transaction.
    pub fn remove_project_source_bundle_file(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        logical_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .remove_bundle_file(bundle_id, logical_path)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Assign or clear one persisted non-entry Automation role in a single
    /// dirty source-graph transaction.
    pub fn set_project_source_bundle_non_entry_role(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        logical_path: &str,
        role: Option<crate::state::ProjectSourceRole>,
    ) -> Result<bool, ProjectSourceError> {
        let changed =
            self.project_sources
                .set_bundle_non_entry_role(bundle_id, logical_path, role)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Insert a new project-owned source bundle and participate in the
    /// ordinary project dirty/save/recovery lifecycle.
    pub fn insert_project_source_bundle(
        &mut self,
        bundle: crate::state::ProjectSourceBundle,
    ) -> Result<(), ProjectSourceError> {
        self.project_sources.insert_bundle(bundle)?;
        self.project_sources_dirty = true;
        Ok(())
    }

    /// Restore a retained project-source revision as a new monotonic dirty
    /// revision. The registry owns the complete graph transaction; the
    /// workspace owns only the ordinary persisted dirty lifecycle.
    pub fn restore_project_source_bundle_revision(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
        expected_current: crate::product::ObjectRevision,
        retained_revision: crate::product::ObjectRevision,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self.project_sources.restore_bundle_revision(
            bundle_id,
            expected_current,
            retained_revision,
        )?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    /// Replace one language slot from an explicitly imported UTF-8 file while
    /// preserving monotonic slot revision and invalidating old validation.
    pub fn replace_imported_project_source(
        &mut self,
        language: ProjectSourceLanguage,
        file_name: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let changed = self
            .project_sources
            .replace_imported(language, file_name, content)?;
        if changed {
            self.project_sources_dirty = true;
        }
        Ok(changed)
    }

    pub fn remove_project_source(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Option<ProjectSourceDocument> {
        let removed = self.project_sources.remove(language);
        if removed.is_some() {
            self.project_sources_dirty = true;
        }
        removed
    }

    /// Record successful validation for the document's exact current identity.
    /// This evidence is persisted and therefore marks the project dirty only
    /// when it changes.
    pub fn mark_project_source_validated(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        let before = self
            .project_sources
            .get(language)
            .and_then(ProjectSourceDocument::validated_identity);
        let identity = self.project_sources.mark_validated(language)?;
        if before != Some(identity) {
            self.project_sources_dirty = true;
        }
        Ok(identity)
    }

    /// Retain validation for an exact source bundle, including every
    /// dependency file in its authenticated closure.
    pub fn mark_project_source_bundle_validated(
        &mut self,
        bundle_id: crate::state::ProjectSourceId,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        let before = self
            .project_sources
            .get_bundle(bundle_id)
            .and_then(crate::state::ProjectSourceBundle::validated_identity);
        let identity = self.project_sources.mark_bundle_validated(bundle_id)?;
        if before != Some(identity) {
            self.project_sources_dirty = true;
        }
        Ok(identity)
    }

    pub fn mark_project_sources_clean(&mut self) {
        self.project_sources_dirty = false;
    }

    /// Whether the Netlist workspace owns an editable source deck.
    ///
    /// A missing source is intentional: in that state the editor is showing a
    /// generated schematic artifact and must never promote edits implicitly.
    pub fn has_editable_netlist_source(&self) -> bool {
        self.netlist_source.is_some()
    }

    /// Create a project-owned source deck from the current generated artifact.
    ///
    /// This is the one ownership transition used by the explicit Netlist
    /// workspace "Make editable copy" action. Creating the source changes the
    /// persisted project, so it participates in the ordinary project dirty and
    /// save lifecycle on both native and browser targets.
    pub fn make_netlist_editable_copy(&mut self, generated: &str) -> bool {
        if self.netlist_source.is_some() {
            return false;
        }

        self.netlist_source = Some(generated.to_owned());
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    /// Replace an existing project-owned source deck.
    ///
    /// Returns `false` for generated artifacts instead of silently creating an
    /// editable source. That guard makes editor, completion, and tuner writes
    /// safe even if a caller accidentally reaches a mutation path while the
    /// generated document is active.
    pub fn replace_editable_netlist_source(&mut self, source: String) -> bool {
        let Some(owned_source) = self.netlist_source.as_mut() else {
            return false;
        };

        if *owned_source == source {
            return false;
        }

        *owned_source = source;
        self.netlist_source_dirty = true;
        true
    }

    /// Remove the project-owned source and return to schematic-generated output.
    ///
    /// Removing persisted source ownership is itself a project modification;
    /// the dirty bit remains set until an actual project save succeeds.
    pub fn return_to_generated_netlist(&mut self) -> bool {
        if self.netlist_source.take().is_none() {
            return false;
        }

        self.netlist_document = None;
        self.netlist_descriptor = None;
        self.netlist_source_path = None;
        self.netlist_source_dirty = true;
        true
    }

    pub fn open_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.active_view = reference.clone();
        if !self
            .open_views
            .iter()
            .any(|open| open.reference == reference)
        {
            self.open_views
                .push(OpenCellView::new(reference.clone(), view_type));
        }
        if is_schematic_like(view_type) {
            self.schematic_buffers.entry(reference.key()).or_default();
        }
    }

    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.hierarchy_stack.clear();
        self.hierarchy_instances.clear();
        self.hierarchy_stack.push(reference);
    }

    pub fn enter_hierarchy(&mut self, reference: CellViewRef, view_type: ViewType) {
        // No instance context (menu/browser entry): the cell name is the
        // best available occurrence label.
        let label = reference.cell.clone();
        self.descend_into(label, reference, view_type);
    }

    /// Descend into `instance`, opening its master `reference`. The
    /// occurrence path (TOP, X1, XB, ...) records the instance name.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        if self.hierarchy_stack.last() != Some(&reference) {
            self.hierarchy_stack.push(reference);
            self.hierarchy_instances.push(instance);
            self.align_occurrences();
        }
    }

    /// Keep `hierarchy_instances` exactly one shorter than the stack —
    /// older saves and external truncation re-label from cell names.
    fn align_occurrences(&mut self) {
        let want = self.hierarchy_stack.len().saturating_sub(1);
        self.hierarchy_instances.truncate(want);
        while self.hierarchy_instances.len() < want {
            let index = self.hierarchy_instances.len() + 1;
            self.hierarchy_instances
                .push(self.hierarchy_stack[index].cell.clone());
        }
    }

    /// Display labels for the occurrence path: the root cell, then the
    /// instance descended through at each level.
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.hierarchy_stack
            .iter()
            .enumerate()
            .map(|(index, reference)| {
                if index == 0 {
                    reference.cell.clone()
                } else {
                    self.hierarchy_instances
                        .get(index - 1)
                        .cloned()
                        .unwrap_or_else(|| reference.cell.clone())
                }
            })
            .collect()
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let len = self.hierarchy_stack.len();
        if len < 2 {
            return None;
        }
        self.focus_breadcrumb(len - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        if index >= self.hierarchy_stack.len() {
            return None;
        }

        self.hierarchy_stack.truncate(index + 1);
        self.align_occurrences();
        let reference = self.hierarchy_stack[index].clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        Some(reference)
    }

    pub fn close_view(&mut self, reference: &CellViewRef) {
        if self.open_views.len() <= 1 {
            return;
        }

        self.open_views.retain(|open| &open.reference != reference);
        if &self.active_view == reference
            && let Some(next) = self.open_views.last().cloned()
        {
            self.active_view = next.reference;
        }
    }

    pub fn set_active_dirty(&mut self, dirty: bool) {
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.active_view)
        {
            open.dirty = dirty;
        }
    }
}

/// Ensure the default editable project library exists.
pub fn ensure_project_library(libraries: &mut LibraryManager) {
    if libraries.get_library(DEFAULT_PROJECT_LIBRARY).is_none() {
        let mut library = Library::new(DEFAULT_PROJECT_LIBRARY);
        library
            .metadata
            .insert("role".to_string(), "project".to_string());
        library.metadata.insert(
            "description".to_string(),
            "Project design library".to_string(),
        );
        libraries.add_library(library);
    }
}

/// Ensure a cell view exists in the library manager.
pub fn ensure_cell_view(
    libraries: &mut LibraryManager,
    library_name: &str,
    cell_name: &str,
    view_name: &str,
    view_type: ViewType,
) {
    if libraries.get_library(library_name).is_none() {
        libraries.add_library(Library::new(library_name));
    }

    if let Some(library) = libraries.get_library_mut(library_name) {
        if library.get_cell(cell_name).is_none() {
            let mut cell = Cell::new(cell_name);
            cell.description = "Top-level design cell".to_string();
            cell.add_view(View::new(view_name, view_type));
            library.add_cell(cell);
            return;
        }

        if let Some(cell) = library.get_cell_mut(cell_name)
            && cell.get_view(view_name).is_none()
        {
            cell.add_view(View::new(view_name, view_type));
        }
    }
}
