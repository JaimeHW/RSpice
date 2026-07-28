//! Design management subflows: the guided forms behind each operation.

use super::widgets::*;
use super::*;

pub(super) fn design_management_subflow_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> egui::Response {
    let page = dialog.page;
    let before = subflow_before(dialog, page);
    let subject = subflow_subject(dialog, page);
    let after = subflow_after(dialog, page);
    let mut first_response = None;
    split_surface(ui, SUBFLOW_LEFT_FRACTION, |left, right| {
        section_header(
            left,
            &format!("{} \u{00b7} semantic preview", page.code()),
            Some(&format!(
                "revision {}",
                dialog.draft.as_ref().map_or(0, |draft| draft.revision())
            )),
        );
        semantic_change_map(left, &before, &subject, &after, page.title());
        let preview_tokens = Tokens::get(left.ctx());
        left.columns(2, |columns| {
            let (preview, properties) = columns.split_at_mut(1);
            schematic_preview(&mut preview[0], page.code());
            property_row_toned(
                &mut properties[0],
                "Invariant",
                page.invariant(),
                preview_tokens.color.ok,
            );
            property_row(
                &mut properties[0],
                "Validation",
                "connectivity \u{00b7} hierarchy \u{00b7} configuration",
            );
            property_row(&mut properties[0], "Recovery", "one explicit predecessor");
            property_row_toned(
                &mut properties[0],
                "Stale results",
                "enumerated after preview",
                preview_tokens.color.warn,
            );
        });

        section_header(right, "Transaction inputs", Some("source resolved"));
        first_response = Some(subflow_fields(
            right,
            dialog,
            workspace,
            libraries,
            schematic,
            write_allowed,
        ));
        if let Some(receipt) = dialog.receipt.as_deref() {
            receipt_banner(right, receipt);
        }
    });
    first_response.unwrap_or_else(|| {
        let (_, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
        response
    })
}

pub(super) fn subflow_fields(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    _libraries: &crate::state::LibraryManager,
    _schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> egui::Response {
    let page = dialog.page;
    match page {
        DesignManagementPage::Manager => unreachable!("manager has a dedicated body"),
        DesignManagementPage::NewSheet => {
            let sheet_error = dialog
                .inputs
                .sheet_name
                .trim()
                .is_empty()
                .then_some("Sheet name is required.");
            let response = input_field(
                ui,
                "Sheet name",
                &mut dialog.inputs.sheet_name,
                "Power and references",
                sheet_error,
                "Stable, project-owned sheet name.",
            );
            let sheets = sheet_choices(dialog);
            field_combo_by_id(
                ui,
                "Insert after",
                &sheets,
                &mut dialog.inputs.sheet_insert_after,
                write_allowed,
            );
            field_combo(
                ui,
                "Template",
                &[
                    (SheetTemplate::AnalogSchematic, "Analog schematic"),
                    (
                        SheetTemplate::MixedSignalSchematic,
                        "Mixed-signal schematic",
                    ),
                    (SheetTemplate::BlankGovernedSheet, "Blank governed sheet"),
                ],
                &mut dialog.inputs.sheet_template,
                write_allowed,
            );
            field_combo(
                ui,
                "Port policy",
                &[
                    (SheetPortPolicy::TypedOffSheetPorts, "Typed off-sheet ports"),
                    (SheetPortPolicy::NoAutomaticPorts, "No automatic ports"),
                ],
                &mut dialog.inputs.sheet_port_policy,
                write_allowed,
            );
            response
        }
        DesignManagementPage::ReorderSheets => {
            let order_error = reorder_sheet_ids(dialog).err();
            let response = input_field(
                ui,
                "New order",
                &mut dialog.inputs.reorder_order_text,
                "AFE core \u{2192} Bias and reference",
                order_error.as_deref(),
                "Complete sheet names separated by arrows, in the reviewed presentation order.",
            );
            field_combo(
                ui,
                "Page numbering",
                &[
                    (
                        ReorderPageNumbering::UpdatePrintPageNumbers,
                        "Update print page numbers",
                    ),
                    (
                        ReorderPageNumbering::RetainExplicitPageNumbers,
                        "Retain explicit page numbers",
                    ),
                ],
                &mut dialog.inputs.reorder_page_numbering,
                write_allowed,
            );
            read_only_field(
                ui,
                "Cross references",
                "Update display only \u{00b7} stable IDs retained",
                "Reordering cannot mutate connectivity or stable identities.",
            );
            response
        }
        DesignManagementPage::MoveSelection => {
            let response = read_only_field(
                ui,
                "Selection",
                &dialog.selection_summary,
                "Complete selected objects captured when Design Management opened.",
            );
            let sheets = sheet_choices(dialog);
            field_combo_by_id(
                ui,
                "Destination",
                &sheets,
                &mut dialog.inputs.move_destination,
                write_allowed,
            );
            field_combo(
                ui,
                "Boundary nets",
                &[
                    (MoveBoundaryPolicy::TypedPorts, "Create typed ports"),
                    (
                        MoveBoundaryPolicy::ReviewedGlobalAliases,
                        "Create reviewed global aliases",
                    ),
                    (MoveBoundaryPolicy::Block, "Block move"),
                ],
                &mut dialog.inputs.move_boundary_policy,
                write_allowed,
            );
            field_combo(
                ui,
                "Hierarchy effect",
                &[
                    (
                        MoveHierarchyEffect::SameCell,
                        "Same cell \u{00b7} sheet move",
                    ),
                    (
                        MoveHierarchyEffect::CreateChildCell,
                        "Create child cell instead",
                    ),
                ],
                &mut dialog.inputs.move_hierarchy_effect,
                write_allowed,
            );
            response
        }
        DesignManagementPage::NewVariant => {
            let variant_error = dialog
                .inputs
                .variant_name
                .trim()
                .is_empty()
                .then_some("Variant name is required.");
            let response = input_field(
                ui,
                "Name",
                &mut dialog.inputs.variant_name,
                "Automotive high-temperature",
                variant_error,
                "Stable assembly-variant identity.",
            );
            let variants = variant_choices(dialog);
            field_combo_optional_base(
                ui,
                "Parent",
                &variants,
                &mut dialog.inputs.variant_parent,
                "Base design",
                write_allowed,
            );
            field_combo(
                ui,
                "Inheritance",
                &[
                    (
                        VariantInheritance::OverrideChangedObjectsOnly,
                        "Override changed objects only",
                    ),
                    (
                        VariantInheritance::IndependentReviewedCopy,
                        "Independent reviewed copy",
                    ),
                ],
                &mut dialog.inputs.variant_inheritance,
                write_allowed,
            );
            field_combo(
                ui,
                "Qualification",
                &[
                    (
                        VariantQualificationPlan::InvalidateAffectedTests,
                        "Invalidate affected tests",
                    ),
                    (
                        VariantQualificationPlan::CreateEmptyQualificationPlan,
                        "Create empty qualification plan",
                    ),
                ],
                &mut dialog.inputs.variant_qualification,
                write_allowed,
            );
            response
        }
        DesignManagementPage::CompareVariants => {
            let variants = variant_choices(dialog);
            let response = field_combo_by_id(
                ui,
                "Reference",
                &variants,
                &mut dialog.inputs.compare_reference,
                true,
            );
            field_combo_by_id(
                ui,
                "Comparison",
                &variants,
                &mut dialog.inputs.compare_target,
                true,
            );
            field_combo(
                ui,
                "Difference classes",
                &[
                    (
                        VariantDifferenceClasses::DevicesValuesDnpModels,
                        "Devices + values + DNP + models",
                    ),
                    (
                        VariantDifferenceClasses::ConnectivityOnly,
                        "Connectivity only",
                    ),
                ],
                &mut dialog.inputs.difference_classes,
                true,
            );
            response
        }
        DesignManagementPage::VariantMatrix => {
            let response = field_combo(
                ui,
                "Scope",
                &[
                    (
                        VariantMatrixScope::AllControlledInstances,
                        "All variant-controlled instances",
                    ),
                    (VariantMatrixScope::CurrentHierarchy, "Current hierarchy"),
                ],
                &mut dialog.inputs.matrix_scope,
                write_allowed,
            );
            field_combo(
                ui,
                "Missing replacement",
                &[
                    (MissingReplacementPolicy::Block, "Block"),
                    (
                        MissingReplacementPolicy::ExplicitDoNotPopulate,
                        "Explicit DNP",
                    ),
                ],
                &mut dialog.inputs.missing_replacement,
                write_allowed,
            );
            field_combo(
                ui,
                "Model equivalence",
                &[
                    (
                        ModelEquivalencePolicy::RequireQualifiedReplacement,
                        "Require qualified replacement",
                    ),
                    (
                        ModelEquivalencePolicy::AllowReviewCandidate,
                        "Allow review candidate",
                    ),
                ],
                &mut dialog.inputs.model_equivalence,
                write_allowed,
            );
            response
        }
        DesignManagementPage::RenumberPreview => {
            let response = field_combo(
                ui,
                "Scope",
                &[
                    (RenumberScopeChoice::WholeProject, "Whole project"),
                    (RenumberScopeChoice::CurrentHierarchy, "Current hierarchy"),
                    (RenumberScopeChoice::CurrentSheet, "Current sheet"),
                ],
                &mut dialog.inputs.renumber_scope,
                write_allowed,
            );
            field_combo(
                ui,
                "Order",
                &[
                    (
                        RenumberOrder::HierarchyThenCoordinates,
                        "Hierarchy then coordinates",
                    ),
                    (
                        RenumberOrder::SheetThenCoordinates,
                        "Sheet then coordinates",
                    ),
                    (RenumberOrder::ConnectivityOrder, "Connectivity order"),
                ],
                &mut dialog.inputs.renumber_order,
                write_allowed,
            );
            field_combo(
                ui,
                "Protected references",
                &[
                    (
                        ProtectedReferencePolicy::RetainLockedAndExternalIds,
                        "Retain locked and external IDs",
                    ),
                    (
                        ProtectedReferencePolicy::IncludeAfterReview,
                        "Include after review",
                    ),
                ],
                &mut dialog.inputs.protected_references,
                write_allowed,
            );
            response
        }
        DesignManagementPage::AnnotationPolicy => {
            let range_error = parse_reserved_ranges(&dialog.inputs.reserved_ranges, dialog).err();
            let response = field_combo(
                ui,
                "Prefix allocation",
                &[
                    (
                        AnnotationPrefixAllocation::ByDeviceFamily,
                        "By device family",
                    ),
                    (AnnotationPrefixAllocation::BySheet, "By sheet"),
                    (AnnotationPrefixAllocation::ByHierarchy, "By hierarchy"),
                ],
                &mut dialog.inputs.prefix_allocation,
                write_allowed,
            );
            input_field(
                ui,
                "Reserved ranges",
                &mut dialog.inputs.reserved_ranges,
                "R,C 1\u{2026}399; U 400\u{2026}599",
                range_error.as_deref(),
                "Project-owned prefix ranges separated by semicolons.",
            );
            field_combo(
                ui,
                "Imported IDs",
                &[
                    (
                        ImportedReferencePolicy::PreserveWithSourceMap,
                        "Preserve with source map",
                    ),
                    (
                        ImportedReferencePolicy::NormalizeAfterReview,
                        "Normalize after review",
                    ),
                ],
                &mut dialog.inputs.imported_ids,
                write_allowed,
            );
            response
        }
        DesignManagementPage::HierarchyAudit => {
            let configurations = workspace
                .configuration_sets
                .configurations()
                .iter()
                .map(|configuration| (configuration.id(), configuration.name().to_owned()))
                .collect::<Vec<_>>();
            let response = field_combo_optional_base(
                ui,
                "Configuration",
                &configurations,
                &mut dialog.inputs.audit_configuration,
                "active project",
                true,
            );
            field_combo(
                ui,
                "View checks",
                &[
                    (
                        HierarchyViewChecks::AllDeclaredFallbacks,
                        "All declared fallbacks",
                    ),
                    (HierarchyViewChecks::SelectedHierarchy, "Selected hierarchy"),
                ],
                &mut dialog.inputs.audit_view_checks,
                true,
            );
            field_combo(
                ui,
                "Protected boundaries",
                &[
                    (
                        ProtectedBoundaryChecks::SignaturesAndPins,
                        "Validate signatures and pins",
                    ),
                    (ProtectedBoundaryChecks::PinsOnly, "Pins only"),
                ],
                &mut dialog.inputs.audit_protected_boundaries,
                true,
            );
            response
        }
    }
}
