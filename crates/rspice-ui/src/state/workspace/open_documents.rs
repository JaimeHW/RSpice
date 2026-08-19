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
            crate::state::AutomationStarterFile::PythonEntry.path(),
            crate::state::DEFAULT_AUTOMATION_PYTHON,
            [
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::RunPlan.path(),
                    crate::state::DEFAULT_AUTOMATION_RUN_PLAN,
                )
                .expect("the built-in run plan is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                    crate::state::DEFAULT_ENVIRONMENT_LOCK,
                )
                .expect("the built-in environment lock is valid"),
                crate::state::ProjectSourceFile::try_new(
                    crate::state::AutomationStarterFile::Permissions.path(),
                    crate::state::DEFAULT_AUTOMATION_PERMISSIONS,
                )
                .expect("the built-in permission manifest is valid"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::RunPlan.path(),
                )
                .expect("the run-plan dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                )
                .expect("the environment-lock dependency is valid"),
                crate::state::ProjectSourceDependency::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::AutomationStarterFile::Permissions.path(),
                )
                .expect("the permission-manifest dependency is valid"),
            ],
            [
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::PythonEntry.path(),
                    crate::state::ProjectSourceRole::AutomationEntry,
                )
                .expect("the Automation entry role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::RunPlan.path(),
                    crate::state::ProjectSourceRole::AutomationRunPlan,
                )
                .expect("the Automation run-plan role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::EnvironmentLock.path(),
                    crate::state::ProjectSourceRole::AutomationEnvironmentLock,
                )
                .expect("the Automation environment-lock role is valid"),
                crate::state::ProjectSourceRoleBinding::try_new(
                    crate::state::AutomationStarterFile::Permissions.path(),
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

    /// Create a genuinely empty project under the requested identity. The
    /// canonical startup fixture keeps the mockup's example sources, while
    /// File > New must not force an unrelated circuit to compile or execute
    /// demonstration code.
    ///
    /// The identity is adopted before the library model is ensured, so the
    /// seeded design library, top cell, active view, open tab, hierarchy root
    /// and schematic buffer key are all the requested ones — never the default
    /// ones renamed afterwards. The caller owns validating the three names
    /// against the persisted cell/view and project-name contracts; nothing
    /// here can report a rejection to the reader.
    pub fn new_empty_bootstrapped(
        libraries: &mut LibraryManager,
        name: &str,
        root_library: &str,
        top_cell: &str,
    ) -> Self {
        let mut project = ProjectDescriptor::default();
        project.name = name.to_owned();
        project.root_library = root_library.to_owned();
        project.top_cell = top_cell.to_owned();
        let active_view = CellViewRef::new(root_library, top_cell, DEFAULT_SCHEMATIC_VIEW);
        let mut schematic_buffers = HashMap::new();
        schematic_buffers.insert(active_view.key(), SchematicState::default());
        let mut workspace = Self {
            project,
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            schematic_buffers,
            active_view,
            ..Self::default()
        };
        workspace.ensure_library_model(libraries);
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
                    // A materialized crossing is exactly what an authored
                    // off-sheet connector is, so it carries the contract's
                    // direction rather than reading as a plain local name.
                    let next_id = projected.next_id();
                    projected.net_labels.push(crate::state::NetLabel::off_sheet(
                        next_id,
                        anchor,
                        contract.definition().net_name.clone(),
                        contract.definition().direction,
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

    /// Ensure the workspace's top library/cell/view exists in the library tree.
    pub fn ensure_library_model(&mut self, libraries: &mut LibraryManager) {
        ensure_project_library(libraries, &self.project.root_library);

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
        // Restore paths that never run project migration — session restore —
        // reach the occurrence model only here. On a live workspace this is an
        // identity, because the projection already mirrors the active
        // document.
        self.adopt_breadcrumb_for_active_document();

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

    /// Ensure `reference` has an open document and make it the active one.
    ///
    /// A document that is already open keeps the occurrence and the read-only
    /// marking it was opened with; one that is not opens as a design root,
    /// because nothing was descended through to reach it.
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
        self.project_active_occurrence();
    }

    /// Activate the document `reference` names. This is the tab gesture: it
    /// restores the occurrence that document was opened at rather than
    /// re-rooting the session on the master.
    pub fn activate_view(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference, view_type);
    }

    /// Open `reference` as a design root, discarding whatever occurrence the
    /// document previously carried. Only File/browser entry re-roots a
    /// document; every other gesture reaches one through an instance.
    pub fn open_as_root(&mut self, reference: CellViewRef, view_type: ViewType) {
        self.open_view(reference.clone(), view_type);
        self.set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Descend into `instance`, opening its master `reference` on the active
    /// document's occurrence.
    pub fn descend_into(&mut self, instance: String, reference: CellViewRef, view_type: ViewType) {
        let mut occurrence = self.active_occurrence_or_root();
        let already_open = occurrence.terminal_master() == &reference;
        self.open_view(reference.clone(), view_type);
        if already_open {
            return;
        }
        occurrence.descend(instance, reference);
        self.set_active_occurrence(occurrence);
    }

    /// The occurrence the active document is editing.
    pub fn active_occurrence(&self) -> Option<&DocumentOccurrence> {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .map(|open| &open.occurrence)
    }

    /// The active document's occurrence, or the root occurrence its reference
    /// implies while no document claims it.
    fn active_occurrence_or_root(&self) -> DocumentOccurrence {
        self.active_occurrence()
            .cloned()
            .unwrap_or_else(|| DocumentOccurrence::rooted(self.active_view.clone()))
    }

    fn set_active_occurrence(&mut self, occurrence: DocumentOccurrence) {
        let active = self.active_view.clone();
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == active)
        {
            occurrence.debug_assert_opens(&open.reference);
            open.occurrence = occurrence;
        }
        self.project_active_occurrence();
    }

    /// Root every document that carries no occurrence at its own reference.
    ///
    /// This is the one repair for a tab record written before documents owned
    /// an occurrence, and it never invents a step: a document restored without
    /// one is a root, not a guessed descent.
    fn root_unrooted_occurrences(&mut self) {
        for open in &mut self.open_views {
            if open.occurrence.is_unrooted() || open.occurrence.terminal_master() != &open.reference
            {
                open.occurrence = DocumentOccurrence::rooted(open.reference.clone());
            }
        }
    }

    /// Refresh the session-global breadcrumb from the active document.
    ///
    /// The two vectors are a read-only projection for surfaces that have not
    /// moved onto the per-document occurrence yet; the occurrence on the open
    /// document is the authority, and this is the only writer.
    fn project_active_occurrence(&mut self) {
        let occurrence = self.active_occurrence_or_root();
        self.hierarchy_stack = occurrence.masters().cloned().collect();
        self.hierarchy_instances = occurrence
            .steps
            .iter()
            .map(|step| step.instance_name.clone())
            .collect();
    }

    /// Display labels for the active occurrence: the root cell, then the
    /// instance descended through at each level.
    pub fn occurrence_labels(&self) -> Vec<String> {
        self.active_occurrence_or_root().labels()
    }

    /// The occurrence the active document is editing, as an instance path.
    pub fn occurrence_path(&self) -> crate::state::InstancePath {
        self.active_occurrence_or_root().instance_path()
    }

    /// Levels on the active occurrence, counting the design root.
    pub fn occurrence_depth(&self) -> usize {
        self.active_occurrence_or_root().depth()
    }

    /// Whether the active document was opened as a read-only hierarchy
    /// reference.
    pub fn active_read_only_reference(&self) -> bool {
        self.open_views
            .iter()
            .find(|open| open.reference == self.active_view)
            .is_some_and(|open| open.read_only_reference)
    }

    pub fn set_active_read_only_reference(&mut self, read_only: bool) {
        let active = self.active_view.clone();
        if let Some(open) = self
            .open_views
            .iter_mut()
            .find(|open| open.reference == active)
        {
            open.read_only_reference = read_only;
        }
    }

    /// Re-root the active document's occurrence at the document itself —
    /// what a prune leaves behind once whatever it was reached through is
    /// gone.
    pub fn reroot_active_occurrence(&mut self) {
        let reference = self.active_view.clone();
        self.set_active_occurrence(DocumentOccurrence::rooted(reference));
    }

    /// Pop one hierarchy level (the U gesture). Returns the new focus.
    pub fn ascend_one(&mut self) -> Option<CellViewRef> {
        let depth = self.occurrence_depth();
        if depth < 2 {
            return None;
        }
        self.focus_breadcrumb(depth - 2)
    }

    pub fn focus_breadcrumb(&mut self, index: usize) -> Option<CellViewRef> {
        let mut occurrence = self.active_occurrence_or_root();
        if index >= occurrence.depth() {
            return None;
        }

        occurrence.truncate_to(index);
        let reference = occurrence.terminal_master().clone();
        self.open_view(reference.clone(), ViewType::Schematic);
        self.set_active_occurrence(occurrence);
        Some(reference)
    }

    /// Prune every open document's occurrence to what still exists.
    ///
    /// A document whose root master is gone closes; one that passes through a
    /// master that is gone keeps the deepest prefix still entirely valid and
    /// re-targets onto that prefix's terminal master, because an occurrence
    /// step is only ever created by descending into a schematic. Nothing is
    /// invented to fill a gap. Returns whether any occurrence changed.
    pub fn retain_valid_occurrences(&mut self, is_valid: impl Fn(&CellViewRef) -> bool) -> bool {
        let mut pruned = false;
        self.open_views.retain_mut(
            |open| match open.occurrence.retain_valid_prefix(&is_valid) {
                OccurrencePrune::Intact => true,
                OccurrencePrune::Truncated => {
                    open.reference = open.occurrence.terminal_master().clone();
                    open.view_type = ViewType::Schematic;
                    pruned = true;
                    true
                }
                OccurrencePrune::Rootless => {
                    pruned = true;
                    false
                }
            },
        );
        // A document re-targeted onto a master another tab already shows is
        // the same document twice; the first one keeps it.
        let mut seen = HashSet::new();
        self.open_views
            .retain(|open| seen.insert(open.reference.key()));
        if !self
            .open_views
            .iter()
            .any(|open| open.reference == self.active_view)
            && let Some(next) = self.open_views.first()
        {
            self.active_view = next.reference.clone();
        }
        self.project_active_occurrence();
        pruned
    }

    /// Rewrite the masters a library, cell, or view rename moved, on every
    /// open document's occurrence. Callers remap `active_view` and each
    /// document's `reference` first, so the terminal-master invariant holds
    /// across the whole transaction.
    pub fn remap_occurrence_masters(&mut self, mut remap: impl FnMut(&mut CellViewRef)) {
        for open in &mut self.open_views {
            for master in open.occurrence.masters_mut() {
                remap(master);
            }
            open.occurrence.debug_assert_opens(&open.reference);
        }
        self.project_active_occurrence();
    }

    /// The occurrence a session-global breadcrumb spells, and how many of its
    /// levels it could not name. Zipping stops at the shorter of the two
    /// vectors, because a missing instance name cannot be invented.
    fn breadcrumb_occurrence(&self) -> Option<(DocumentOccurrence, usize)> {
        let root = self.hierarchy_stack.first().cloned()?;
        let mut occurrence = DocumentOccurrence::rooted(root);
        for (master, instance) in self
            .hierarchy_stack
            .iter()
            .skip(1)
            .zip(&self.hierarchy_instances)
        {
            occurrence.descend(instance.clone(), master.clone());
        }
        let unnamed = self.hierarchy_stack.len() - occurrence.depth();
        Some((occurrence, unnamed))
    }

    /// Adopt a breadcrumb that describes the document already in front.
    ///
    /// Restore paths that never run project migration reach the occurrence
    /// model here, and so does every schematic restore, so this must never
    /// re-target which document is active: the breadcrumb records where a
    /// session had navigated, not which document a caller just opened. A
    /// breadcrumb that ends anywhere else is dropped in favour of the
    /// projection.
    fn adopt_breadcrumb_for_active_document(&mut self) {
        self.root_unrooted_occurrences();
        match self.breadcrumb_occurrence() {
            Some((occurrence, _)) if occurrence.terminal_master() == &self.active_view => {
                self.set_active_occurrence(occurrence);
            }
            _ => self.project_active_occurrence(),
        }
    }

    /// Fold a save's session-global breadcrumb onto the document it described.
    ///
    /// Every document is first rooted at its own reference, then the active
    /// one adopts the breadcrumb. A save whose two vectors disagree keeps only
    /// the prefix both spell, and adopts it only if it ends at a document that
    /// is actually open — an occurrence that named a master no tab shows would
    /// address a different instance than the document on screen. Returns the
    /// load warning that repair owes the reader.
    pub fn migrate_document_occurrences(&mut self) -> Option<String> {
        self.root_unrooted_occurrences();
        let Some((occurrence, unnamed)) = self.breadcrumb_occurrence() else {
            self.project_active_occurrence();
            return None;
        };
        let terminal = occurrence.terminal_master().clone();
        let adopted = self
            .open_views
            .iter()
            .any(|open| open.reference == terminal);

        if !adopted {
            self.project_active_occurrence();
            return Some(format!(
                "This project's saved hierarchy breadcrumb ended at {}, which no open document \
                 shows; the active document was restored at its own root instead.",
                terminal.display_path()
            ));
        }

        self.active_view = terminal;
        self.set_active_occurrence(occurrence);
        (unnamed > 0).then(|| {
            format!(
                "This project's saved hierarchy breadcrumb named {unnamed} level(s) it carried no \
                 instance name for; the occurrence was kept at {} rather than guessing them.",
                self.occurrence_path()
            )
        })
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
        self.project_active_occurrence();
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

/// Ensure the project's editable design library exists under the name the
/// descriptor claims as its root.
pub fn ensure_project_library(libraries: &mut LibraryManager, name: &str) {
    if libraries.get_library(name).is_none() {
        let mut library = Library::new(name);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn master(cell: &str) -> CellViewRef {
        CellViewRef::new("work", cell, "schematic")
    }

    /// A session that descended `labels` from the default root, as the
    /// gestures themselves build it.
    fn descended(labels: &[&str]) -> ProjectWorkspace {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.simulation_root_reference();
        workspace.open_as_root(root, ViewType::Schematic);
        for (index, label) in labels.iter().enumerate() {
            workspace.descend_into(
                (*label).to_owned(),
                master(&format!("level_{index}")),
                ViewType::Schematic,
            );
        }
        workspace
    }

    #[test]
    fn the_occurrence_path_names_instances_below_the_implicit_root() {
        assert!(descended(&[]).occurrence_path().is_root());
        assert_eq!(descended(&["X1"]).occurrence_path().to_string(), "/X1");
        assert_eq!(
            descended(&["X1", "XB"]).occurrence_path().to_string(),
            "/X1/XB"
        );
        assert!(
            descended(&["X 1"]).occurrence_path().is_root(),
            "a label the grammar cannot name resolves to the root, not to half a path"
        );
        assert!(descended(&["X1", "X 2"]).occurrence_path().is_root());
    }

    /// The defect this model exists to kill: one session-global breadcrumb
    /// meant the second tab's descent overwrote the first tab's, and coming
    /// back to a tab reported whichever path the last navigation left behind.
    #[test]
    fn two_documents_reached_through_different_parents_keep_their_own_occurrence() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XA".to_owned(), master("afe"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XA");

        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.descend_into("XB".to_owned(), master("bias"), ViewType::Schematic);
        workspace.descend_into("XR".to_owned(), master("ref"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XB/XR");

        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert_eq!(
            workspace.occurrence_path().to_string(),
            "/XA",
            "activating a document restores the occurrence it was opened at"
        );
        workspace.activate_view(master("ref"), ViewType::Schematic);
        assert_eq!(workspace.occurrence_path().to_string(), "/XB/XR");
        assert_eq!(
            workspace.hierarchy_stack,
            vec![master("tb"), master("bias"), master("ref")],
            "the legacy breadcrumb is a projection of whichever document is active"
        );
    }

    #[test]
    fn every_open_document_ends_its_occurrence_at_the_master_it_shows() {
        let workspace = descended(&["X1", "XB"]);
        for open in &workspace.open_views {
            open.occurrence.debug_assert_opens(&open.reference);
            assert_eq!(open.occurrence.terminal_master(), &open.reference);
        }
    }

    #[test]
    fn a_read_only_reference_marking_belongs_to_the_document_it_was_opened_on() {
        let mut workspace = ProjectWorkspace::default();
        workspace.open_as_root(master("tb"), ViewType::Schematic);
        workspace.open_as_root(master("afe"), ViewType::Schematic);
        workspace.set_active_read_only_reference(true);
        assert!(workspace.active_read_only_reference());

        workspace.activate_view(master("tb"), ViewType::Schematic);
        assert!(
            !workspace.active_read_only_reference(),
            "the other document was never opened read-only"
        );
        workspace.activate_view(master("afe"), ViewType::Schematic);
        assert!(
            workspace.active_read_only_reference(),
            "returning to the reference document still refuses writes"
        );
    }

    #[test]
    fn pruning_truncates_to_what_survives_and_closes_a_rootless_document() {
        let mut workspace = descended(&["X1", "XB"]);
        let deepest = workspace.active_view.clone();
        assert!(workspace.retain_valid_occurrences(|reference| reference.cell != "level_0"));
        assert!(
            workspace
                .open_views
                .iter()
                .all(|open| open.reference != deepest),
            "the document below a master that is gone folds onto the surviving prefix"
        );
        assert!(workspace.occurrence_path().is_root());

        let mut rootless = ProjectWorkspace::default();
        rootless.open_as_root(master("keep"), ViewType::Schematic);
        rootless.open_as_root(master("gone"), ViewType::Schematic);
        rootless.descend_into("X1".to_owned(), master("child"), ViewType::Schematic);
        assert!(rootless.retain_valid_occurrences(|reference| reference.cell != "gone"));
        assert!(
            rootless
                .open_views
                .iter()
                .any(|open| open.reference == master("keep")),
            "an unrelated document is untouched"
        );
        assert!(
            rootless
                .open_views
                .iter()
                .all(|open| open.reference.cell != "gone" && open.reference.cell != "child"),
            "a document whose root is gone has no occurrence left, so it closes"
        );
    }

    #[test]
    fn a_legacy_breadcrumb_migrates_onto_the_document_it_described() {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.active_view.clone();
        workspace.open_view(master("afe"), ViewType::Schematic);
        workspace.hierarchy_stack = vec![root.clone(), master("afe")];
        workspace.hierarchy_instances = vec!["XAFE".to_owned()];

        assert!(workspace.migrate_document_occurrences().is_none());
        assert_eq!(workspace.occurrence_path().to_string(), "/XAFE");
        assert_eq!(workspace.active_view, master("afe"));
        assert_eq!(
            workspace
                .active_occurrence()
                .map(|occurrence| &occurrence.root),
            Some(&root)
        );
    }

    #[test]
    fn disagreeing_legacy_arrays_keep_the_shorter_prefix_and_warn() {
        let mut workspace = ProjectWorkspace::default();
        let root = workspace.active_view.clone();
        workspace.open_view(master("afe"), ViewType::Schematic);
        workspace.open_view(master("bias"), ViewType::Schematic);
        workspace.hierarchy_stack = vec![root, master("afe"), master("bias")];
        workspace.hierarchy_instances = vec!["XAFE".to_owned()];

        let warning = workspace
            .migrate_document_occurrences()
            .expect("a breadcrumb that cannot be spelled owes the reader a warning");
        assert!(warning.contains("1 level"), "{warning}");
        assert_eq!(
            workspace.occurrence_path().to_string(),
            "/XAFE",
            "the level with no instance name is dropped, never invented"
        );
        assert_eq!(workspace.active_view, master("afe"));
    }

    /// A crossing contract and a hand-placed connector must produce the same
    /// object, or the canvas would draw a materialized crossing as an ordinary
    /// local name and checks would never ask it for a partner.
    #[test]
    fn a_materialized_crossing_is_a_pair_of_off_sheet_connectors() {
        use crate::state::{
            CellViewRef, CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
            CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
            MoveBoundaryResolution, MoveSelectionRequest, Point, SheetDefinition, SheetPortPolicy,
            SheetTemplate,
        };

        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let first = schematic
            .add_wire(vec![Point::origin(), Point::new(10, 0)])
            .expect("first wire");
        let second = schematic
            .add_wire(vec![Point::origin(), Point::new(0, 10)])
            .expect("second wire");

        let source_sheet = workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Input", [first, second])
            .expect("bootstrap sheet ownership");
        let catalog = workspace
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let destination_sheet = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Output".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(source_sheet),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![second],
                destination_sheet_id: destination_sheet,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: source_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: first,
                                point: Point::origin(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination_sheet,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: second,
                                point: Point::origin(),
                            },
                        },
                        direction: CrossSheetPortDirection::Supply,
                        signal_type: CrossSheetSignalType::Power,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("move with explicit boundary contract");

        assert!(
            schematic.net_labels.is_empty(),
            "the crossing is a sheet contract, not an authored label"
        );
        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("materialize governed design");
        let crossing: Vec<_> = projected
            .net_labels
            .iter()
            .filter(|label| label.name == "BIAS")
            .collect();

        assert_eq!(crossing.len(), 2, "one connector per side of the contract");
        for label in &crossing {
            assert_eq!(
                label.kind,
                crate::state::NetLabelKind::OffSheet {
                    direction: CrossSheetPortDirection::Supply
                },
                "a materialized crossing carries the contract's own direction"
            );
        }
        assert_ne!(
            crossing[0].pos, crossing[1].pos,
            "the pair lands in the two sheets' separate coordinate namespaces"
        );
    }
}
