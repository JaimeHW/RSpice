//! Design management operations.
//!
//! Renumbering, renaming, and reorganizing across cells. These cross
//! document boundaries, so they go through the project design history rather
//! than schematic-local undo.

use super::controller::validate_design_management_page;
use super::widgets::{parse_reserved_ranges, reorder_sheet_ids};
use super::*;

impl RSpiceApp {
    pub(super) fn commit_design_management_subflow(
        &mut self,
        page: DesignManagementPage,
    ) -> Result<(), String> {
        validate_design_management_page(&self.state, page)?;
        if page == DesignManagementPage::MoveSelection
            && self
                .state
                .dialogs
                .design_management
                .inputs
                .move_hierarchy_effect
                == MoveHierarchyEffect::CreateChildCell
        {
            if self.state.dialogs.design_management.dirty() {
                return Err(
                    "Apply the pending Design Management draft before transferring this selection to Create hierarchy."
                        .to_owned(),
                );
            }
            if !crate::workbench::app::dialogs::hierarchy::create::create_hierarchy_available(
                &self.state,
            ) {
                return Err(
                    "Create hierarchy requires the retained complete instance-only selection and a writable active schematic."
                        .to_owned(),
                );
            }
            self.state.dialogs.design_management.close_and_discard();
            crate::workbench::app::close_design_management_dialog_route(&mut self.state);
            crate::workbench::app::dialogs::hierarchy::create::open_create_hierarchy_dialog(
                &mut self.state,
            );
            return Ok(());
        }
        let owner_key = self.state.dialogs.design_management.owner_key.clone();
        let inputs = self.state.dialogs.design_management.inputs.clone();
        let selection = self
            .state
            .dialogs
            .design_management
            .selection_object_ids
            .clone();
        let all_objects = self.state.dialogs.design_management.all_object_ids.clone();

        let (tab, receipt) = match page {
            DesignManagementPage::Manager => {
                return Err("The manager publishes through its reviewed primary action.".to_owned());
            }
            DesignManagementPage::NewSheet => {
                let personal = self
                    .state
                    .ui
                    .preferences
                    .drawing_sheet_personal_preferences();
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let mut page_format = inputs
                    .sheet_page_format
                    .without_project_owned_title_values();
                let selected_personal_id = match &page_format.authored_size {
                    crate::state::AuthoredDrawingSheetSize::Custom { snapshot } => {
                        snapshot.preset_id.clone()
                    }
                    crate::state::AuthoredDrawingSheetSize::Standard { .. } => None,
                };
                if let Some(personal_preset) = selected_personal_id
                    .as_deref()
                    .filter(|id| draft.drawing_sheet_settings().find_preset(id).is_none())
                    .and_then(|id| {
                        personal
                            .presets
                            .iter()
                            .find(|preset| preset.id.eq_ignore_ascii_case(id))
                    })
                {
                    page_format = crate::workbench::app::capture_personal_preset_into_project(
                        draft,
                        personal_preset,
                    )?
                    .format
                    .without_project_owned_title_values();
                }
                let settings = draft.drawing_sheet_settings().clone();
                if settings.new_sheet_policy == DrawingSheetNewSheetPolicy::Ask
                    && settings.remember_last_explicit_format
                    && page_format.inheritance == DrawingSheetInheritance::Explicit
                    && settings.last_explicit_format.as_ref() != Some(&page_format)
                {
                    let mut updated = settings;
                    updated.last_explicit_format = Some(page_format.clone());
                    draft
                        .update_drawing_sheet_settings(draft.revision(), updated)
                        .map_err(|error| error.to_string())?;
                }
                let catalog = draft
                    .ensure_sheet_catalog(&owner_key)
                    .map_err(|error| error.to_string())?;
                let used_pages = catalog
                    .sheets()
                    .iter()
                    .filter_map(|sheet| sheet.definition().explicit_page_number)
                    .collect::<BTreeSet<_>>();
                let explicit_page_number =
                    (1..=u32::MAX)
                        .find(|page| !used_pages.contains(page))
                        .ok_or_else(|| "No free sheet page number remains.".to_owned())?;
                let id = catalog
                    .create_sheet_with_page_format(
                        SheetDefinition {
                            name: inputs.sheet_name.trim().to_owned(),
                            template: inputs.sheet_template,
                            port_policy: inputs.sheet_port_policy,
                            explicit_page_number: Some(explicit_page_number),
                        },
                        inputs.sheet_insert_after,
                        page_format,
                    )
                    .map_err(|error| error.to_string())?;
                if catalog.object_assignments().is_empty() && !all_objects.is_empty() {
                    catalog
                        .assign_objects(catalog.revision(), id, all_objects)
                        .map_err(|error| error.to_string())?;
                }
                (
                    DesignManagementTab::Sheets,
                    format!(
                        "Created stable sheet `{}` · {}",
                        inputs.sheet_name.trim(),
                        id
                    ),
                )
            }
            DesignManagementPage::ReorderSheets => {
                let ordered_ids = reorder_sheet_ids(&self.state.dialogs.design_management)?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let catalog = draft
                    .sheet_catalog_mut(&owner_key)
                    .ok_or_else(|| "No governed sheet catalog is available.".to_owned())?;
                let revision = catalog
                    .reorder(
                        catalog.revision(),
                        ordered_ids,
                        inputs.reorder_page_numbering,
                        ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
                    )
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Sheets,
                    format!("Applied reviewed sheet order · catalog revision {revision}"),
                )
            }
            DesignManagementPage::MoveSelection => {
                let destination = inputs
                    .move_destination
                    .ok_or_else(|| "Destination sheet is required.".to_owned())?;
                let connectivity =
                    crate::workbench::app::dialogs::hierarchy::create::selected_component_sheet_move_plan(
                        &self.state,
                    )?;
                if connectivity.source_component_ids != selection {
                    return Err(
                        "The retained selection changed while the sheet move was being reviewed. Close and reopen Design Management."
                            .to_owned(),
                    );
                }
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let catalog = draft
                    .sheet_catalog_mut(&owner_key)
                    .ok_or_else(|| "No governed sheet catalog is available.".to_owned())?;
                let active = catalog.active_sheet_id();
                catalog
                    .reconcile_object_assignments(catalog.revision(), all_objects, active)
                    .map_err(|error| error.to_string())?;
                let source = connectivity
                    .source_component_ids
                    .first()
                    .and_then(|object_id| catalog.sheet_for_object(*object_id))
                    .ok_or_else(|| {
                        "The selected instances have no governed source-sheet identity.".to_owned()
                    })?;
                if inputs.move_boundary_policy == MoveBoundaryPolicy::ReviewedGlobalAliases {
                    let mut settings = catalog.settings().clone();
                    settings.connector_policy =
                        crate::state::OffSheetConnectorPolicy::NamedConnectorsCompatibility;
                    if settings != *catalog.settings() {
                        catalog
                            .set_settings(catalog.revision(), settings)
                            .map_err(|error| error.to_string())?;
                    }
                }
                let boundary_ports = connectivity
                    .boundaries
                    .into_iter()
                    .map(|boundary| CrossSheetPortDefinition {
                        net_name: boundary.net_name,
                        first: CrossSheetPortEndpoint {
                            sheet_id: source,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: boundary.stationary_wire_id,
                                point: boundary.stationary_point,
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: boundary.moved_component_id,
                                terminal_name: boundary.moved_terminal_name,
                            },
                        },
                        direction: match boundary.direction {
                            PortDirection::In => CrossSheetPortDirection::Input,
                            PortDirection::Out => CrossSheetPortDirection::Output,
                            PortDirection::InOut => CrossSheetPortDirection::InOut,
                            PortDirection::Supply => CrossSheetPortDirection::Supply,
                        },
                        signal_type: match (boundary.direction, boundary.discipline) {
                            (PortDirection::Supply, _) => CrossSheetSignalType::Power,
                            (_, PortDiscipline::Logic) => CrossSheetSignalType::Logic,
                            _ => CrossSheetSignalType::Analog,
                        },
                        discipline: match boundary.discipline {
                            PortDiscipline::Electrical => CrossSheetDiscipline::Electrical,
                            PortDiscipline::Logic => CrossSheetDiscipline::Logic,
                            PortDiscipline::Wreal => CrossSheetDiscipline::Wreal,
                            PortDiscipline::Thermal => CrossSheetDiscipline::Thermal,
                        },
                    })
                    .collect::<Vec<_>>();
                let boundary_resolution = if boundary_ports.is_empty() {
                    MoveBoundaryResolution::VerifiedNoBoundaryNets
                } else {
                    MoveBoundaryResolution::ExplicitPorts {
                        ports: boundary_ports,
                    }
                };
                let moved = catalog
                    .move_selection(MoveSelectionRequest {
                        expected_catalog_revision: catalog.revision(),
                        object_ids: connectivity.moved_object_ids,
                        destination_sheet_id: destination,
                        boundary_resolution,
                    })
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Sheets,
                    format!(
                        "Moved {} objects · {} explicit boundary ports · catalog revision {}",
                        moved.object_ids.len(),
                        moved.created_port_ids.len(),
                        moved.catalog_revision
                    ),
                )
            }
            DesignManagementPage::NewVariant => {
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let id = draft
                    .variants_mut()
                    .create(AssemblyVariantDraft {
                        name: inputs.variant_name.trim().to_owned(),
                        parent_id: inputs.variant_parent,
                        inheritance: inputs.variant_inheritance,
                        qualification_plan: inputs.variant_qualification,
                        overrides: BTreeMap::new(),
                    })
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Created governed assembly variant `{}` · {id}",
                        inputs.variant_name.trim()
                    ),
                )
            }
            DesignManagementPage::CompareVariants => {
                let reference = inputs
                    .compare_reference
                    .ok_or_else(|| "Reference variant is required.".to_owned())?;
                let comparison = inputs
                    .compare_target
                    .ok_or_else(|| "Comparison variant is required.".to_owned())?;
                let result = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?
                    .variants()
                    .compare(reference, comparison)
                    .map_err(|error| error.to_string())?;
                let (difference_count, class_label) = match inputs.difference_classes {
                    VariantDifferenceClasses::DevicesValuesDnpModels => {
                        (result.differences.len(), "device/value/DNP/model")
                    }
                    VariantDifferenceClasses::ConnectivityOnly => (
                        variant_connectivity_difference_count(&self.state, reference, comparison)?,
                        "connectivity",
                    ),
                };
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Compared immutable variants · {difference_count} {class_label} differences · {}",
                        result.semantic_digest
                    ),
                )
            }
            DesignManagementPage::VariantMatrix => {
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let mut variants = draft.variants().clone();
                let mut settings = variants.settings().clone();
                settings.missing_replacement = inputs.missing_replacement;
                settings.model_equivalence = inputs.model_equivalence;
                let settings_changed = settings != *variants.settings();
                if settings_changed {
                    variants
                        .set_settings(settings)
                        .map_err(|error| error.to_string())?;
                }
                let edits = variants
                    .variants()
                    .iter()
                    .flat_map(|variant| {
                        variant
                            .definition()
                            .overrides
                            .iter()
                            .filter_map(|(object, value)| {
                                let in_scope = inputs.matrix_scope
                                    == VariantMatrixScope::AllControlledInstances
                                    || object.cell_view_key() == owner_key;
                                in_scope.then(|| VariantMatrixEdit {
                                    variant_id: variant.id(),
                                    expected_revision: variant.revision(),
                                    object: object.clone(),
                                    replacement: match value {
                                        crate::state::VariantObjectOverride::Substitute {
                                            replacement,
                                        } => Some(replacement.clone()),
                                        crate::state::VariantObjectOverride::DoNotPopulate {
                                            ..
                                        } => None,
                                    },
                                })
                            })
                    })
                    .collect::<Vec<_>>();
                if !edits.is_empty() {
                    match variants.apply_substitution_matrix(
                        edits,
                        inputs.missing_replacement,
                        inputs.model_equivalence,
                    ) {
                        Ok(_) | Err(crate::state::DesignManagementError::NoChanges(_)) => {}
                        Err(error) => return Err(error.to_string()),
                    }
                }
                if !settings_changed {
                    return Err(
                        "The governed substitution policies and existing overrides are unchanged."
                            .to_owned(),
                    );
                }
                *draft.variants_mut() = variants;
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Validated {} governed override cells and applied substitution policies",
                        draft
                            .variants()
                            .variants()
                            .iter()
                            .map(|variant| variant.definition().overrides.len())
                            .sum::<usize>()
                    ),
                )
            }
            DesignManagementPage::RenumberPreview => {
                let design_management = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let request = renumber_request(
                    &self.state.workspace,
                    &self.state.schematic,
                    design_management,
                    &owner_key,
                    &inputs,
                )?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let preview = draft
                    .annotation()
                    .preview_renumbering(&request)
                    .map_err(|error| error.to_string())?;
                let mapping_count = preview.mappings.len();
                let id = draft
                    .annotation_mut()
                    .commit_renumbering(&preview, &request)
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Annotation,
                    format!("Created reviewed mapping {id} · {mapping_count} reference changes"),
                )
            }
            DesignManagementPage::AnnotationPolicy => {
                let ranges = parse_reserved_ranges(
                    &inputs.reserved_ranges,
                    &self.state.dialogs.design_management,
                )?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let policy = draft.annotation().policy().clone();
                let mut definition = policy.definition().clone();
                definition.prefix_allocation = inputs.prefix_allocation;
                definition.reserved_ranges = ranges;
                definition.imported_ids = inputs.imported_ids;
                let revision = draft
                    .annotation_mut()
                    .update_policy(policy.revision(), definition)
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Annotation,
                    format!("Saved annotation policy revision {revision}"),
                )
            }
            DesignManagementPage::HierarchyAudit => {
                let resolution = self.state.workspace.resolve_hierarchy_with_active(
                    &self.state.library_manager,
                    &self.state.workspace.active_view,
                    &self.state.schematic,
                );
                let configuration = match inputs.audit_configuration {
                    Some(id) => {
                        let configuration = self
                            .state
                            .workspace
                            .configuration_sets
                            .find(id)
                            .ok_or_else(|| {
                                "The selected configuration no longer exists.".to_owned()
                            })?;
                        HierarchyAuditConfiguration::ConfigurationSet {
                            id,
                            revision: configuration.revision(),
                            semantic_digest: configuration.semantic_digest(),
                        }
                    }
                    None => HierarchyAuditConfiguration::ActiveProject,
                };
                let subjects = hierarchy_audit_subjects(
                    &resolution,
                    inputs.audit_view_checks,
                    &self.state.workspace.occurrence_path(),
                );
                let request = HierarchyAuditRequest {
                    configuration,
                    view_checks: match inputs.audit_view_checks {
                        HierarchyViewChecks::AllDeclaredFallbacks => {
                            crate::state::HierarchyViewChecks::AllDeclaredFallbacks
                        }
                        HierarchyViewChecks::SelectedHierarchy => {
                            crate::state::HierarchyViewChecks::SelectedHierarchy
                        }
                    },
                    protected_boundaries: match inputs.audit_protected_boundaries {
                        ProtectedBoundaryChecks::SignaturesAndPins => {
                            crate::state::ProtectedBoundaryChecks::ValidateSignaturesAndPins
                        }
                        ProtectedBoundaryChecks::PinsOnly => {
                            crate::state::ProtectedBoundaryChecks::PinsOnly
                        }
                    },
                    subjects,
                    boundary_evidence: Vec::new(),
                };
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let id = draft
                    .run_and_record_hierarchy_audit(&request)
                    .map_err(|error| error.to_string())?;
                let audit = draft
                    .hierarchy_audits()
                    .last()
                    .ok_or_else(|| "The hierarchy audit receipt was not retained.".to_owned())?;
                (
                    DesignManagementTab::Hierarchy,
                    format!(
                        "Recorded hierarchy audit {id} · {} resolved subjects · {} findings",
                        audit.resolved_subjects(),
                        audit.findings().len()
                    ),
                )
            }
        };

        let dialog = &mut self.state.dialogs.design_management;
        dialog.page = DesignManagementPage::Manager;
        dialog.active_tab = tab;
        dialog.input_baseline = None;
        dialog.error = None;
        dialog.receipt = Some(receipt);
        dialog.discard_confirmation = false;
        dialog.body_scroll_offset = 0.0;
        Ok(())
    }
}

/// Collect every annotatable object with the hierarchy path the renumbering
/// scope is matched against.
///
/// The document the session has open sits at its occurrence, which is `/` while
/// nothing has been descended into — so "current hierarchy" at the design root
/// covers the whole design, and covers exactly the active occurrence below it.
pub(super) fn renumber_request(
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &crate::state::SchematicState,
    design_management: &DesignManagementCatalog,
    owner_key: &str,
    inputs: &SubflowInputs,
) -> Result<RenumberRequest, String> {
    let mut documents = workspace.schematic_buffers.clone();
    documents.insert(owner_key.to_owned(), active_schematic.clone());
    let occurrence = workspace.occurrence_path().to_string();
    let mut objects = Vec::new();
    for (cell_view_key, schematic) in &documents {
        let hierarchy_path = if cell_view_key == owner_key {
            occurrence.clone()
        } else {
            format!("/{}", cell_view_key.replace(['/', '\\'], "_"))
        };
        for component in &schematic.components {
            let prefix = component.kind.spice_prefix();
            if prefix.is_empty() || component.name.trim().is_empty() {
                continue;
            }
            let object = SchematicObjectKey::new(cell_view_key, component.id)
                .map_err(|error| error.to_string())?;
            let sheet_id =
                design_management.sheet_for_object_or_active(cell_view_key, component.id);
            objects.push(AnnotationObject {
                object,
                current_reference: component.name.clone(),
                device_family: prefix.to_owned(),
                sheet_id,
                hierarchy_path: hierarchy_path.clone(),
                position: AnnotationPosition {
                    x: i64::from(component.pos.x),
                    y: i64::from(component.pos.y),
                },
                connectivity_order: None,
                locked: false,
                external: false,
                imported: false,
            });
        }
    }
    let scope = match inputs.renumber_scope {
        RenumberScopeChoice::WholeProject => RenumberScope::WholeProject,
        RenumberScopeChoice::CurrentHierarchy => {
            RenumberScope::CurrentHierarchy { path: occurrence }
        }
        RenumberScopeChoice::CurrentSheet => {
            let sheet_id = design_management
                .sheet_catalog(owner_key)
                .and_then(|catalog| catalog.active_sheet_id())
                .ok_or_else(|| "The active sheet identity is unavailable.".to_owned())?;
            RenumberScope::CurrentSheet { sheet_id }
        }
    };
    Ok(RenumberRequest {
        scope,
        order: inputs.renumber_order,
        protected_references: inputs.protected_references,
        protected_reviewed: inputs.protected_references
            == ProtectedReferencePolicy::IncludeAfterReview,
        objects,
    })
}

pub(super) fn variant_connectivity_difference_count(
    state: &AppState,
    reference: AssemblyVariantId,
    comparison: AssemblyVariantId,
) -> Result<usize, String> {
    let reference = variant_connectivity_signature(state, reference)?;
    let comparison = variant_connectivity_signature(state, comparison)?;
    Ok(reference.symmetric_difference(&comparison).count())
}

/// Produce a name-independent connectivity partition for every materialized
/// schematic owned by one variant. Each set member is the sorted list of
/// stable component-terminal identities sharing a canonical electrical net.
/// Comparing these partitions detects DNP and substitution topology effects
/// without treating value/model-only changes as connectivity differences.
pub(super) fn variant_connectivity_signature(
    state: &AppState,
    variant_id: AssemblyVariantId,
) -> Result<BTreeSet<Vec<String>>, String> {
    let mut workspace = state.workspace.clone();
    workspace
        .design_management
        .variants_mut()
        .set_active(variant_id)
        .map_err(|error| error.to_string())?;
    let projection = workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
        projection.schematic_buffers(),
    );
    let mut groups = BTreeMap::<(String, String), Vec<String>>::new();
    for (cell_view_key, schematic) in projection.schematic_buffers() {
        let generated = crate::simulation::netlist_gen::generate_netlist_hierarchical(
            schematic,
            &[],
            &hierarchy,
        );
        if !generated.errors.is_empty() {
            return Err(format!(
                "Variant connectivity could not be resolved for `{cell_view_key}`: {}",
                generated.errors.join(" ")
            ));
        }
        for component in &schematic.components {
            for (terminal_name, point) in component.terminal_positions_resolved(None) {
                let net_name = generated
                    .point_to_net
                    .get(&point)
                    .cloned()
                    .unwrap_or_else(|| format!("__unconnected_{}_{}", component.id, terminal_name));
                groups
                    .entry((cell_view_key.to_ascii_lowercase(), net_name))
                    .or_default()
                    .push(format!(
                        "{}#{}:{}",
                        cell_view_key.to_ascii_lowercase(),
                        component.id,
                        terminal_name.to_ascii_lowercase()
                    ));
            }
        }
    }
    Ok(groups
        .into_values()
        .map(|mut endpoints| {
            endpoints.sort();
            endpoints.dedup();
            endpoints
        })
        .collect())
}

/// The occurrences a hierarchy audit reports on.
///
/// `SelectedHierarchy` narrows the audit to the occurrence the session is in,
/// which is the whole design while nothing has been descended into. Every
/// resolved path is read through the path type, so a configuration's required
/// path and the resolver's expanded path are compared as occurrences rather
/// than as two spellings that happen to differ.
pub(super) fn hierarchy_audit_subjects(
    resolution: &crate::state::workspace::HierarchyResolution,
    view_checks: HierarchyViewChecks,
    scope: &crate::state::InstancePath,
) -> Vec<HierarchyAuditSubject> {
    let mut subjects = resolution
        .bindings
        .iter()
        .flat_map(|binding| {
            binding.instance_paths.iter().filter_map(move |path| {
                if view_checks == HierarchyViewChecks::SelectedHierarchy
                    && !crate::state::InstancePath::parse_legacy(path)
                        .is_ok_and(|occurrence| occurrence.starts_with(scope))
                {
                    return None;
                }
                let resolved = binding.status.is_resolved().then(|| {
                    binding
                        .stop_view
                        .clone()
                        .unwrap_or_else(|| binding.reference.view.clone())
                });
                Some(HierarchyAuditSubject {
                    instance_path: path.clone(),
                    cell_name: binding.reference.cell.clone(),
                    design_view: binding.reference.view.clone(),
                    declared_fallbacks: binding.view_search_order.clone(),
                    resolved_simulation_view: resolved.clone(),
                    fallback_used: binding.used_review_fallback.then_some(resolved).flatten(),
                    child_instance_paths: Vec::new(),
                    protected_boundary_id: None,
                })
            })
        })
        .collect::<Vec<_>>();
    let paths = subjects
        .iter()
        .map(|subject| subject.instance_path.clone())
        .collect::<Vec<_>>();
    for subject in &mut subjects {
        let prefix = format!("{}/", subject.instance_path.trim_end_matches('/'));
        subject.child_instance_paths = paths
            .iter()
            .filter(|path| {
                path.starts_with(&prefix)
                    && !path[prefix.len()..].contains('/')
                    && **path != subject.instance_path
            })
            .cloned()
            .collect();
    }
    subjects
}
