//! Design management subflows: the guided forms behind each operation.

use super::widgets::*;
use super::*;

pub(super) fn design_management_subflow_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    drawing_sheet_personal: &crate::workbench::DrawingSheetPersonalPreferences,
    write_allowed: bool,
) -> egui::Response {
    let mut first_response = None;
    if dialog.page == DesignManagementPage::RenumberPreview {
        // The scope and order beside the list they produce: every part's
        // new name, live, with its Keep tick.
        let mut toggled = None;
        split_surface(ui, RENUMBER_FIELDS_FRACTION, |left, right| {
            Frame::NONE
                .inner_margin(Margin::symmetric(PANE_INSET as i8, 0))
                .show(left, |pane| {
                    first_response = Some(subflow_fields(
                        pane,
                        dialog,
                        workspace,
                        libraries,
                        schematic,
                        drawing_sheet_personal,
                        write_allowed,
                    ));
                });
            if let Some(table) = dialog.renumber_table.as_ref() {
                toggled = renumber_list(right, table, write_allowed);
            }
        });
        if let Some(object) = toggled
            && !dialog.inputs.renumber_keep.remove(&object)
        {
            dialog.inputs.renumber_keep.insert(object);
        }
    } else if dialog.page.read_out() {
        // The scope beside the table it shows, as Renumber's fields stand
        // beside its names.
        split_surface(ui, RENUMBER_FIELDS_FRACTION, |left, right| {
            Frame::NONE
                .inner_margin(Margin::symmetric(PANE_INSET as i8, 0))
                .show(left, |pane| {
                    first_response = Some(subflow_fields(
                        pane,
                        dialog,
                        workspace,
                        libraries,
                        schematic,
                        drawing_sheet_personal,
                        write_allowed,
                    ));
                });
            if let Some(readout) = dialog.variant_readout.as_ref() {
                super::variant_readout::variant_readout_list(right, readout);
            }
        });
    } else {
        single_pane(ui, |pane| {
            first_response = Some(subflow_fields(
                pane,
                dialog,
                workspace,
                libraries,
                schematic,
                drawing_sheet_personal,
                write_allowed,
            ));
        });
    }
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
    drawing_sheet_personal: &crate::workbench::DrawingSheetPersonalPreferences,
    write_allowed: bool,
) -> egui::Response {
    let page = dialog.page;
    match page {
        DesignManagementPage::Manager => unreachable!("manager has a dedicated body"),
        DesignManagementPage::NewSheet => {
            // The name starts empty and Create waits for one: an empty
            // field is not yet a mistake, so it is not marked as one.
            let response = input_field(
                ui,
                "Sheet name",
                &mut dialog.inputs.sheet_name,
                "",
                None,
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
                "Port policy",
                &[
                    (SheetPortPolicy::TypedOffSheetPorts, "Typed off-sheet ports"),
                    (SheetPortPolicy::NoAutomaticPorts, "No automatic ports"),
                ],
                &mut dialog.inputs.sheet_port_policy,
                write_allowed,
            );
            let settings = workspace.content.design_management.drawing_sheet_settings();
            let current = dialog
                .draft
                .as_ref()
                .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
                .and_then(|catalog| catalog.active())
                .map(|sheet| {
                    sheet
                        .page_format()
                        .try_update(|format| {
                            format.inheritance = DrawingSheetInheritance::Explicit;
                        })
                        .expect("an active sheet format can be captured explicitly")
                        .without_project_owned_title_values()
                });
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("Drawing sheet")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            if settings.new_sheet_policy == DrawingSheetNewSheetPolicy::Ask {
                crate::ui::widgets::Combo::from_id_salt("new-sheet-drawing-format")
                    .selected_text(dialog.inputs.sheet_page_format.display())
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut dialog.inputs.sheet_page_format,
                            settings.default_format.clone(),
                            format!(
                                "Project default \u{00b7} {}",
                                settings.default_format.display()
                            ),
                        );
                        if let Some(current) = current.clone() {
                            ui.selectable_value(
                                &mut dialog.inputs.sheet_page_format,
                                current.clone(),
                                format!("Current sheet \u{00b7} {}", current.display()),
                            );
                        }
                        ui.separator();
                        for standard in DrawingSheetStandard::ALL {
                            let format = SchematicSheetFormat::from_standard(
                                standard,
                                dialog.inputs.sheet_page_format.orientation,
                            );
                            ui.selectable_value(
                                &mut dialog.inputs.sheet_page_format,
                                format,
                                standard.label(),
                            );
                        }
                        if !settings.presets.is_empty() {
                            ui.separator();
                            for preset in &settings.presets {
                                ui.selectable_value(
                                    &mut dialog.inputs.sheet_page_format,
                                    preset.format.clone(),
                                    format!("Custom \u{00b7} {}", preset.name),
                                );
                            }
                        }
                        let uncaptured_personal = drawing_sheet_personal
                            .presets
                            .iter()
                            .filter(|preset| settings.find_preset(&preset.id).is_none())
                            .collect::<Vec<_>>();
                        if !uncaptured_personal.is_empty() {
                            ui.separator();
                            ui.weak("Personal custom sizes");
                            for preset in uncaptured_personal {
                                ui.selectable_value(
                                    &mut dialog.inputs.sheet_page_format,
                                    preset.format.clone(),
                                    format!("Personal \u{00b7} {}", preset.name),
                                );
                            }
                        }
                    });
                ui.weak(
                    "The project policy asks for an explicit drawing-sheet choice on every new sheet. A selected personal size is copied into the project before the sheet is created.",
                );
            } else {
                let mut format_label = dialog.inputs.sheet_page_format.display();
                ui.add_enabled(
                    false,
                    egui::TextEdit::singleline(&mut format_label).desired_width(f32::INFINITY),
                );
                ui.weak(match settings.new_sheet_policy {
                    DrawingSheetNewSheetPolicy::ProjectDefault => {
                        "This sheet will follow the project drawing-sheet default."
                    }
                    DrawingSheetNewSheetPolicy::MatchCurrent => {
                        "This sheet will copy the current sheet's format as an explicit snapshot."
                    }
                    DrawingSheetNewSheetPolicy::Ask => unreachable!(),
                });
            }
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
            let response = input_field(
                ui,
                "Name",
                &mut dialog.inputs.variant_name,
                "",
                None,
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
        // What is shown can be narrowed on a project that cannot be
        // written: the scope changes nothing.
        DesignManagementPage::VariantMatrix => field_combo(
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
            true,
        ),
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
                    (RenumberOrder::AcrossThenDown, "Across then down"),
                    (RenumberOrder::DownThenAcross, "Down then across"),
                ],
                &mut dialog.inputs.renumber_order,
                write_allowed,
            );
            response
        }
        DesignManagementPage::AnnotationPolicy => {
            let range_error = parse_reserved_ranges(&dialog.inputs.reserved_ranges).err();
            let response = field_combo(
                ui,
                "Prefix allocation",
                &PREFIX_ALLOCATIONS,
                &mut dialog.inputs.prefix_allocation,
                write_allowed,
            );
            input_field(
                ui,
                "Reserved ranges",
                &mut dialog.inputs.reserved_ranges,
                "None",
                range_error.as_deref(),
                "Project-owned prefix ranges separated by semicolons.",
            );
            response
        }
        DesignManagementPage::HierarchyAudit => {
            let configurations = workspace
                .content
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
