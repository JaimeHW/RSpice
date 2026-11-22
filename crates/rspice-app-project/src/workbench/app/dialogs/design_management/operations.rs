//! Design management operations.
//!
//! Renumbering, renaming, and reorganizing across cells. These cross
//! document boundaries, so they go through the project design history rather
//! than schematic-local undo.

use super::controller::validate_design_management_page;
use super::widgets::parse_reserved_ranges;
use super::*;
use crate::schematic::view::drawing_sheet::{
    DRAWING_SHEET_ORIGIN, DRAWING_SHEET_WORLD_UNIT_UM, DrawingSheetGeometry,
};
use crate::state::DrawingSheetZoneLayout;
use crate::workbench::project::commands::ProjectRequest;

pub(super) fn commit_design_management_subflow(
    parts: &mut DesignManagementParts<'_>,
    page: DesignManagementPage,
) -> Result<(), String> {
    validate_design_management_page(parts.dialog, parts.model, page)?;
    if page == DesignManagementPage::MoveSelection
        && parts.dialog.inputs.move_hierarchy_effect == MoveHierarchyEffect::CreateChildCell
    {
        if parts.dialog.dirty() {
            return Err(
                "Apply the pending Design Management draft before transferring this selection to Create hierarchy."
                    .to_owned(),
            );
        }
        if !parts.model.create_hierarchy_available() {
            return Err(
                "Create hierarchy requires the retained complete instance-only selection and a writable active schematic."
                    .to_owned(),
            );
        }
        parts.dialog.close_and_discard();
        super::close_design_management_dialog_route(parts.wb);
        // Create hierarchy is design's: the shell opens it as this dialog returns.
        parts.wb.outbox.push(ProjectRequest::OpenCreateHierarchy);
        return Ok(());
    }
    let owner_key = parts.dialog.owner_key.clone();
    let inputs = parts.dialog.inputs.clone();
    let selection = parts.dialog.selection_object_ids.clone();
    let all_objects = parts.dialog.all_object_ids.clone();

    // An operation reports only what it produced that the manager's own
    // tabs do not show — a created-object count, a difference count, a
    // finding count. That it completed is reported by its page closing.
    let (tab, outcome) = match page {
        DesignManagementPage::Manager => {
            return Err("The manager publishes through its reviewed primary action.".to_owned());
        }
        DesignManagementPage::NewSheet => {
            let personal = parts.model.preferences.drawing_sheet_personal_preferences();
            let draft = parts
                .dialog
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
                page_format =
                    crate::workbench::app_state::Model::capture_personal_preset_into_project(
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
            let explicit_page_number = (1..=u32::MAX)
                .find(|page| !used_pages.contains(page))
                .ok_or_else(|| "No free sheet page number remains.".to_owned())?;
            let id = catalog
                .create_sheet_with_page_format(
                    SheetDefinition {
                        name: inputs.sheet_name.trim().to_owned(),
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
            (DesignManagementTab::Sheets, None)
        }
        DesignManagementPage::MoveSelection => {
            let destination = inputs
                .move_destination
                .ok_or_else(|| "Destination sheet is required.".to_owned())?;
            let connectivity = parts.model.selected_component_sheet_move_plan()?;
            if connectivity.source_component_ids != selection {
                return Err(
                    "The retained selection changed while the sheet move was being reviewed. Close and reopen Design Management."
                        .to_owned(),
                );
            }
            let draft = parts
                .dialog
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
            // One owner builds the typed boundary contract, so the strip's
            // "Move selection here" and this reviewed page cannot disagree
            // about what a boundary net becomes.
            let boundary_resolution =
                crate::workbench::app_state::Model::sheet_move_boundary_resolution(
                    &connectivity,
                    source,
                    destination,
                );
            let moved = catalog
                .move_selection(MoveSelectionRequest {
                    expected_catalog_revision: catalog.revision(),
                    object_ids: connectivity.moved_object_ids,
                    destination_sheet_id: destination,
                    boundary_resolution,
                })
                .map_err(|error| error.to_string())?;
            // The boundary ports are the part the author did not ask for.
            (
                DesignManagementTab::Sheets,
                (!moved.created_port_ids.is_empty()).then(|| {
                    format!(
                        "{} boundary port{} created for the move",
                        moved.created_port_ids.len(),
                        if moved.created_port_ids.len() == 1 {
                            ""
                        } else {
                            "s"
                        }
                    )
                }),
            )
        }
        DesignManagementPage::NewVariant => {
            let draft = parts
                .dialog
                .draft
                .as_mut()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
            draft
                .variants_mut()
                .create(AssemblyVariantDraft {
                    name: inputs.variant_name.trim().to_owned(),
                    parent_id: inputs.variant_parent,
                    inheritance: inputs.variant_inheritance,
                    qualification_plan: inputs.variant_qualification,
                    overrides: BTreeMap::new(),
                })
                .map_err(|error| error.to_string())?;
            (DesignManagementTab::Variants, None)
        }
        DesignManagementPage::CompareVariants => {
            let reference = inputs
                .compare_reference
                .ok_or_else(|| "Reference variant is required.".to_owned())?;
            let comparison = inputs
                .compare_target
                .ok_or_else(|| "Comparison variant is required.".to_owned())?;
            let result = parts
                .dialog
                .draft
                .as_ref()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?
                .variants()
                .compare(Some(reference), Some(comparison))
                .map_err(|error| error.to_string())?;
            let (difference_count, class_label) = match inputs.difference_classes {
                VariantDifferenceClasses::DevicesValuesDnpModels => {
                    (result.differences.len(), "device/value/DNP/model")
                }
                VariantDifferenceClasses::ConnectivityOnly => (
                    variant_connectivity_difference_count(parts.model, reference, comparison)?,
                    "connectivity",
                ),
            };
            // The comparison has nowhere else to land: no tab lists it.
            (
                DesignManagementTab::Variants,
                Some(format!(
                    "{difference_count} {class_label} difference{}",
                    if difference_count == 1 { "" } else { "s" }
                )),
            )
        }
        // A read-out's Close changes nothing and goes back to the tab it
        // was opened from.
        DesignManagementPage::VariantMatrix => (DesignManagementTab::Variants, None),
        DesignManagementPage::RenumberPreview => {
            let design_management = parts
                .dialog
                .draft
                .as_ref()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
            let request = renumber_request(
                &parts.model.workspace,
                &parts.model.schematic,
                design_management,
                &owner_key,
                &inputs,
            )?;
            let draft = parts
                .dialog
                .draft
                .as_mut()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
            let preview = draft
                .annotation()
                .preview_renumbering(&request)
                .map_err(|error| error.to_string())?;
            // A part renumbered to the name it already has is recorded,
            // but it is not a change.
            let changes = preview
                .mappings
                .values()
                .filter(|mapping| mapping.old_reference != mapping.new_reference)
                .count();
            draft
                .annotation_mut()
                .commit_renumbering(&preview, &request)
                .map_err(|error| error.to_string())?;
            (
                DesignManagementTab::Annotation,
                Some(format!(
                    "{} changed",
                    super::widgets::counted(changes, "name")
                )),
            )
        }
        DesignManagementPage::AnnotationPolicy => {
            let ranges = parse_reserved_ranges(&inputs.reserved_ranges)?;
            let draft = parts
                .dialog
                .draft
                .as_mut()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
            let policy = draft.annotation().policy().clone();
            let mut definition = policy.definition().clone();
            definition.prefix_allocation = inputs.prefix_allocation;
            definition.reserved_ranges = ranges;
            draft
                .annotation_mut()
                .update_policy(policy.revision(), definition)
                .map_err(|error| error.to_string())?;
            (DesignManagementTab::Annotation, None)
        }
        DesignManagementPage::HierarchyAudit => {
            let resolution = parts.model.workspace.resolve_hierarchy_with_active(
                &parts.model.library_manager,
                &parts.model.workspace.content.active_view,
                &parts.model.schematic,
            );
            let configuration = match inputs.audit_configuration {
                Some(id) => {
                    let configuration = parts
                        .model
                        .workspace
                        .content
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
                &parts.model.workspace.content.occurrence_path(),
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
            let draft = parts
                .dialog
                .draft
                .as_mut()
                .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
            draft
                .run_and_record_hierarchy_audit(&request)
                .map_err(|error| error.to_string())?;
            let audit = draft
                .hierarchy_audits()
                .last()
                .ok_or_else(|| "The hierarchy audit was not recorded.".to_owned())?;
            // The tab says "qualified" or "findings"; the counts are here.
            (
                DesignManagementTab::Hierarchy,
                Some(format!(
                    "{} subject{} audited · {} finding{}",
                    audit.resolved_subjects(),
                    if audit.resolved_subjects() == 1 {
                        ""
                    } else {
                        "s"
                    },
                    audit.findings().len(),
                    if audit.findings().len() == 1 { "" } else { "s" }
                )),
            )
        }
    };

    let dialog = &mut *parts.dialog;
    dialog.page = DesignManagementPage::Manager;
    dialog.active_tab = tab;
    dialog.input_baseline = None;
    dialog.error = None;
    dialog.outcome = outcome;
    dialog.discard_confirmation = false;
    dialog.body_scroll_offset = 0.0;
    Ok(())
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
    let mut documents = workspace
        .content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.clone(), schematic))
        .collect::<BTreeMap<_, _>>();
    documents.insert(owner_key.to_owned(), active_schematic.editor_ref().design);
    let occurrence = workspace.content.occurrence_path().to_string();
    let mut objects = Vec::new();
    for (cell_view_key, schematic) in &documents {
        let hierarchy_path = if cell_view_key == owner_key {
            occurrence.clone()
        } else {
            format!("/{}", cell_view_key.replace(['/', '\\'], "_"))
        };
        let sheets = sheet_layouts(design_management, cell_view_key);
        for component in &schematic.document().components {
            let prefix = component.kind.spice_prefix();
            if prefix.is_empty() || component.name.trim().is_empty() {
                continue;
            }
            let object = SchematicObjectKey::new(cell_view_key, component.id)
                .map_err(|error| error.to_string())?;
            let locked = inputs.renumber_keep.contains(&object);
            let sheet_id =
                design_management.sheet_for_object_or_active(cell_view_key, component.id);
            let (page, zones) = sheet_id
                .and_then(|id| sheets.get(&id))
                .map_or((0, None), |(page, zones)| (*page, zones.as_ref()));
            objects.push(AnnotationObject {
                object,
                current_reference: component.name.clone(),
                device_family: prefix.to_owned(),
                sheet_id,
                hierarchy_path: hierarchy_path.clone(),
                position: AnnotationPosition {
                    page,
                    zone: zones.map(|zones| zone_of(zones, component.pos)),
                    x: i64::from(component.pos.x),
                    y: i64::from(component.pos.y),
                },
                locked,
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
        // A ticked part keeps its name: the tick is the protection.
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects,
    })
}

/// Work the Renumber list out again if what it was worked out from has
/// changed since: the scope, the order, the Keep ticks or the draft.
pub(super) fn refresh_renumber_table(
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    schematic: &crate::state::SchematicState,
) {
    let (Some(draft), Some(draft_basis)) = (dialog.draft.as_ref(), dialog.draft_basis()) else {
        dialog.renumber_table = None;
        return;
    };
    let basis = RenumberBasis {
        scope: dialog.inputs.renumber_scope,
        order: dialog.inputs.renumber_order,
        keep: dialog.inputs.renumber_keep.clone(),
        draft: draft_basis,
    };
    if dialog
        .renumber_table
        .as_ref()
        .is_some_and(|table| table.basis == basis)
    {
        return;
    }
    let table = match renumber_request(
        workspace,
        schematic,
        draft,
        &dialog.owner_key,
        &dialog.inputs,
    ) {
        Err(problem) => RenumberTable {
            basis,
            rows: Vec::new(),
            problem: Some(problem),
        },
        Ok(request) => {
            let (mappings, problem) = match draft.annotation().preview_renumbering(&request) {
                Ok(preview) => (preview.mappings, None),
                // Every part in scope is kept: nothing would change.
                Err(crate::state::DesignManagementError::NoChanges(_)) => (BTreeMap::new(), None),
                Err(error) => (BTreeMap::new(), Some(renumber_problem(&error))),
            };
            let rows = request
                .sequence()
                .into_iter()
                .map(|object| RenumberRow {
                    object: object.object.clone(),
                    current: object.current_reference.clone(),
                    new: mappings.get(&object.object).map_or_else(
                        || object.current_reference.clone(),
                        |mapping| mapping.new_reference.clone(),
                    ),
                    sheet: sheet_label(draft, &dialog.owner_key, object),
                    kept: object.locked,
                })
                .collect();
            RenumberTable {
                basis,
                rows,
                problem,
            }
        }
    };
    dialog.renumber_table = Some(table);
}

/// Work out again what each reserved range holds, if the draft has changed
/// since: the names the design's parts hold once the draft's renumberings
/// are applied.
pub(super) fn refresh_range_use(
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    schematic: &crate::state::SchematicState,
) {
    let (Some(draft), Some(basis)) = (dialog.draft.as_ref(), dialog.draft_basis()) else {
        dialog.range_use = None;
        return;
    };
    if dialog
        .range_use
        .as_ref()
        .is_some_and(|(current, _)| *current == basis)
    {
        return;
    }
    let annotation = draft.annotation();
    let uses = renumber_request(
        workspace,
        schematic,
        draft,
        &dialog.owner_key,
        &SubflowInputs::default(),
    )
    .map(|request| {
        let effective = annotation.effective_mappings();
        let objects = request
            .objects
            .into_iter()
            .map(|mut object| {
                if let Some(mapping) = effective.get(&object.object) {
                    object.current_reference.clone_from(&mapping.new_reference);
                }
                object
            })
            .collect::<Vec<_>>();
        annotation
            .policy()
            .definition()
            .reserved_range_use(&objects)
    })
    .unwrap_or_default();
    dialog.range_use = Some((basis, uses));
}

/// Why the parts cannot be renumbered, in the reader's words where the
/// model's are an engineer's.
fn renumber_problem(error: &crate::state::DesignManagementError) -> String {
    use crate::state::DesignManagementError as Error;
    match error {
        Error::EmptyRenumberScope => "This scope holds no parts to renumber.".to_owned(),
        Error::AnnotationRangeExhausted(prefix) => {
            format!("No free {prefix} number is left in the reserved ranges.")
        }
        Error::DuplicateReferenceDesignator(name) => {
            format!("Two parts are named {name}: rename one before renumbering.")
        }
        error => error.to_string(),
    }
}

/// The sheet a listed part is on, with its cell's name when the part is in
/// another cell view than the open one.
fn sheet_label(
    catalog: &DesignManagementCatalog,
    owner_key: &str,
    object: &AnnotationObject,
) -> String {
    let cell_view_key = object.object.cell_view_key();
    let sheet = object
        .sheet_id
        .and_then(|id| catalog.sheet_catalog(cell_view_key)?.find(id))
        .map_or_else(String::new, |sheet| sheet.name().to_owned());
    if cell_view_key == owner_key {
        return sheet;
    }
    let cell = cell_view_key.split('/').nth(1).unwrap_or(cell_view_key);
    if sheet.is_empty() {
        cell.to_owned()
    } else {
        format!("{cell} \u{00b7} {sheet}")
    }
}

/// Each sheet of `cell_view_key` with its place in page order, from 0, and
/// the zones of the frame it prints on, if that frame has any.
fn sheet_layouts(
    design_management: &DesignManagementCatalog,
    cell_view_key: &str,
) -> BTreeMap<SheetId, (u32, Option<DrawingSheetZoneLayout>)> {
    let Some(catalog) = design_management.sheet_catalog(cell_view_key) else {
        return BTreeMap::new();
    };
    let default_format = &design_management.drawing_sheet_settings().default_format;
    catalog
        .sheets()
        .iter()
        .enumerate()
        .map(|(page, sheet)| {
            let format = match sheet.page_format().inheritance {
                DrawingSheetInheritance::ProjectDefault => default_format,
                DrawingSheetInheritance::Explicit | DrawingSheetInheritance::UserDefault => {
                    sheet.page_format()
                }
            };
            let zones = DrawingSheetGeometry::from_format(format).zones;
            (sheet.id(), (u32::try_from(page).unwrap_or(u32::MAX), zones))
        })
        .collect()
}

/// The zone `point` lies in, as its row from the top and its column from
/// the left. A point off the frame takes the nearest zone, so every part of
/// a zoned sheet is read by the same bands.
fn zone_of(zones: &DrawingSheetZoneLayout, point: crate::state::Point) -> (u32, u32) {
    let unit_um = DRAWING_SHEET_WORLD_UNIT_UM as f64;
    let x_um = ((f64::from(point.x) - DRAWING_SHEET_ORIGIN.0) * unit_um).round() as i64;
    let y_um = ((f64::from(point.y) - DRAWING_SHEET_ORIGIN.1) * unit_um).round() as i64;
    let index = |boundaries: &mut dyn Iterator<Item = i64>, at: i64| {
        u32::try_from(boundaries.filter(|boundary| at >= *boundary).count()).unwrap_or(u32::MAX)
    };
    (
        index(&mut zones.row_boundaries_um(), y_um),
        index(&mut zones.column_boundaries_um(), x_um),
    )
}

pub fn variant_connectivity_difference_count(
    model: &Model,
    reference: AssemblyVariantId,
    comparison: AssemblyVariantId,
) -> Result<usize, String> {
    let reference = variant_connectivity_signature(model, reference)?;
    let comparison = variant_connectivity_signature(model, comparison)?;
    Ok(reference.symmetric_difference(&comparison).count())
}

/// Produce a name-independent connectivity partition for every materialized
/// schematic owned by one variant. Each set member is the sorted list of
/// stable component-terminal identities sharing a canonical electrical net.
/// Comparing these partitions detects DNP and substitution topology effects
/// without treating value/model-only changes as connectivity differences.
pub(super) fn variant_connectivity_signature(
    model: &Model,
    variant_id: AssemblyVariantId,
) -> Result<BTreeSet<Vec<String>>, String> {
    let mut workspace = model.workspace.clone();
    workspace
        .content
        .design_management
        .variants_mut()
        .set_active(variant_id)
        .map_err(|error| error.to_string())?;
    let projection = workspace
        .configuration_execution_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
        )
        .map_err(|error| error.to_string())?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
        model.library_manager.catalog(),
        &projection,
    );
    let mut groups = BTreeMap::<(String, String), Vec<String>>::new();
    for (cell_view_key, schematic) in projection.schematic_buffers() {
        let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
            schematic,
            &[],
            &hierarchy,
            &rspice_simulation::netlist_gen::NetlistSourceData::new(
                &crate::simulation::table_route::SourceFiles,
            ),
        );
        if !generated.errors.is_empty() {
            return Err(format!(
                "Variant connectivity could not be resolved for `{cell_view_key}`: {}",
                generated.errors.join(" ")
            ));
        }
        for component in &schematic.document().components {
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

/// The occurrences a hierarchy audit reports on, each listing the instances
/// it places.
///
/// `SelectedHierarchy` narrows the audit to the occurrence the session is in,
/// which is the whole design while nothing has been descended into. Every
/// resolved path is read through the path type, so a configuration's required
/// path and the resolver's expanded path are compared as occurrences rather
/// than as two spellings that happen to differ.
///
/// Each occurrence lists its children as [`list_children`] files them.
pub(super) fn hierarchy_audit_subjects(
    resolution: &crate::state::workspace::HierarchyResolution,
    view_checks: HierarchyViewChecks,
    scope: &crate::state::InstancePath,
) -> Vec<HierarchyAuditSubject> {
    let mut subjects = resolution
        .bindings
        .iter()
        .flat_map(|binding| {
            binding.occurrences.iter().filter_map(move |occurrence| {
                if view_checks == HierarchyViewChecks::SelectedHierarchy
                    && !occurrence.starts_with(scope)
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
                    instance_path: occurrence.to_string(),
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
    list_children(&mut subjects);
    subjects
}

/// Lists under each subject the subjects whose paths extend it by one
/// instance, in one pass: a design that expands to a million instances lists
/// its children in the time it takes to name them.
pub(super) fn list_children(subjects: &mut [HierarchyAuditSubject]) {
    let index = subjects
        .iter()
        .enumerate()
        .map(|(at, subject)| (subject.instance_path.as_str(), at))
        .collect::<std::collections::HashMap<_, _>>();
    let parents = subjects
        .iter()
        .map(|subject| {
            enclosing_path(&subject.instance_path).and_then(|parent| index.get(parent).copied())
        })
        .collect::<Vec<_>>();
    for (child, parent) in parents.into_iter().enumerate() {
        if let Some(parent) = parent {
            let path = subjects[child].instance_path.clone();
            subjects[parent].child_instance_paths.push(path);
        }
    }
}

/// The path `path` extends by one instance: `/` for an instance the top
/// places, and none for the top itself.
fn enclosing_path(path: &str) -> Option<&str> {
    let path = path.trim_end_matches('/');
    let cut = path.rfind('/')?;
    Some(if cut == 0 { "/" } else { &path[..cut] })
}
