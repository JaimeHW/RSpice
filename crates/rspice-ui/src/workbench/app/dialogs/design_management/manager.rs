//! The design manager surface: the tabbed body listing libraries, cells,
//! and views.

use super::widgets::*;
use super::*;

pub(super) fn design_management_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let first_focus = design_management_tabs(ui, &mut dialog.active_tab);
    let action = match dialog.active_tab {
        DesignManagementTab::Sheets => sheets_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Variants => variants_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Annotation => annotation_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Hierarchy => {
            hierarchy_manager_body(ui, dialog, workspace, libraries, schematic, write_allowed)
        }
    };
    concept_banner(
        ui,
        "Every change is one reversible working-revision transaction. Existing results retain their original hierarchy, variant, and annotation manifests.",
        true,
    );
    if let Some(receipt) = dialog.receipt.as_deref() {
        receipt_banner(ui, receipt);
    }
    let _ = first_focus;
    action
}

pub(super) fn design_management_tabs(
    ui: &mut Ui,
    active: &mut DesignManagementTab,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width();
    let (strip_rect, strip_response) =
        ui.allocate_exact_size(vec2(available, TAB_HEIGHT), Sense::hover());
    ui.painter().rect_filled(strip_rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip_rect.x_range(),
        strip_rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let mut x = strip_rect.left();
    let mut first = None;
    for tab in DesignManagementTab::ALL {
        let rect = egui::Rect::from_min_size(
            egui::pos2(x, strip_rect.top()),
            vec2(TAB_MIN_WIDTH, TAB_HEIGHT),
        );
        let response = ui.interact(
            rect,
            ui.id().with(("design-management-tab", tab.label())),
            Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                *active == tab,
                tab.label(),
            )
        });
        if response.clicked() {
            *active = tab;
        }
        if *active == tab {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
            ui.painter().hline(
                egui::Rangef::new(rect.left(), rect.right()),
                rect.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        ui.painter().text(
            rect.left_center() + vec2(12.0, 0.0),
            egui::Align2::LEFT_CENTER,
            tab.label(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if *active == tab {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        crate::ui::theme::paint_focus_ring(ui, &response, rect);
        first.get_or_insert(response);
        x += TAB_MIN_WIDTH;
    }
    first.unwrap_or(strip_response)
}

pub(super) fn sheets_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let catalog = draft.sheet_catalog(&dialog.owner_key);
        let sheets = catalog
            .map(|catalog| {
                catalog
                    .sheets()
                    .iter()
                    .map(|sheet| (sheet.id(), sheet.name().to_owned()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let active_id = catalog.and_then(|catalog| catalog.active_sheet_id());
        let ports = catalog
            .map(|catalog| catalog.cross_sheet_ports().to_vec())
            .unwrap_or_default();
        section_header(
            left,
            "Multi-sheet design",
            Some(&format!("{} sheets \u{00b7} connected", sheets.len())),
        );
        paint_table_header(
            left,
            &[0.12, 0.31, 0.16, 0.22, 0.19],
            &["Order", "Sheet", "Ports", "Off-sheet nets", "State"],
        );
        if sheets.is_empty() {
            empty_table_row(
                left,
                "No governed sheets. Create the first stable sheet identity.",
            );
        } else {
            for (index, (id, name)) in sheets.iter().enumerate() {
                let port_count = ports
                    .iter()
                    .filter(|port| {
                        let definition = port.definition();
                        definition.first.sheet_id == *id || definition.second.sheet_id == *id
                    })
                    .count();
                let values = [
                    format!("{:02}", index + 1),
                    name.clone(),
                    port_count.to_string(),
                    port_count.to_string(),
                    if Some(*id) == active_id {
                        "current"
                    } else {
                        "governed"
                    }
                    .to_owned(),
                ];
                paint_table_row(left, &[0.12, 0.31, 0.16, 0.22, 0.19], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("New sheet\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::NewSheet);
            }
            if Button::new("Reorder\u{2026}")
                .enabled(write_allowed && sheets.len() > 1)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::ReorderSheets);
            }
            if Button::new("Move selection to sheet\u{2026}")
                .enabled(
                    write_allowed && sheets.len() > 1 && !dialog.selection_object_ids.is_empty(),
                )
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::MoveSelection);
            }
        });

        section_header(right, "Sheet contract", None);
        let mut selected = active_id;
        if setting_combo_by_id(right, "Active sheet", &sheets, &mut selected, write_allowed)
            && let Some(id) = selected
            && let Some(catalog) = draft.sheet_catalog_mut(&dialog.owner_key)
        {
            apply_domain_result(
                &mut dialog.error,
                catalog.set_active(id),
                "Active sheet changed.",
            );
        }
        if let Some(settings) = draft
            .sheet_catalog(&dialog.owner_key)
            .map(|catalog| catalog.settings().clone())
        {
            let mut connector = settings.connector_policy;
            let mut page_numbering = settings.page_numbering;
            let mut delete_behavior = settings.delete_behavior;
            let mut changed = false;
            changed |= setting_combo(
                right,
                "Off-sheet connector policy",
                &[
                    (
                        crate::state::OffSheetConnectorPolicy::TypedPortsWithExplicitDirection,
                        "Typed ports with explicit direction",
                    ),
                    (
                        crate::state::OffSheetConnectorPolicy::NamedConnectorsCompatibility,
                        "Named connectors \u{00b7} compatibility",
                    ),
                ],
                &mut connector,
                write_allowed,
            );
            changed |= setting_combo(
                right,
                "Page numbering",
                &[
                    (
                        SheetPageNumbering::StableProjectOrder,
                        "Stable project order",
                    ),
                    (SheetPageNumbering::PerPrintSet, "Per print set"),
                ],
                &mut page_numbering,
                write_allowed,
            );
            changed |= setting_combo(
                right,
                "Delete behavior",
                &[
                    (
                        SheetDeleteBehavior::BlockWhileReferenced,
                        "Block while referenced",
                    ),
                    (
                        SheetDeleteBehavior::MoveReferencesToReviewedReplacement,
                        "Move references to reviewed replacement",
                    ),
                ],
                &mut delete_behavior,
                write_allowed,
            );
            if changed && let Some(catalog) = draft.sheet_catalog_mut(&dialog.owner_key) {
                let revision = catalog.revision();
                apply_domain_result(
                    &mut dialog.error,
                    catalog.set_settings(
                        revision,
                        crate::state::SheetCatalogSettings {
                            connector_policy: connector,
                            page_numbering,
                            delete_behavior,
                        },
                    ),
                    "Sheet contract updated.",
                );
            }
        } else {
            muted_note(right, "Create a sheet to establish the sheet contract.");
        }
        concept_banner(
            right,
            "Sheet identity is stable across reordering, printing, and annotation. Moving objects creates explicit ports and preserves source-object history.",
            false,
        );
    });
    action
}

pub(super) fn variants_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let variants = draft
            .variants()
            .variants()
            .iter()
            .map(|variant| {
                (
                    variant.id(),
                    variant.name().to_owned(),
                    variant.definition().parent.as_ref().map(|parent| parent.id),
                    variant.definition().overrides.clone(),
                )
            })
            .collect::<Vec<_>>();
        let names = variants
            .iter()
            .map(|(id, name, _, _)| (*id, name.clone()))
            .collect::<Vec<_>>();
        let active = draft.variants().active_variant_id();
        section_header(
            left,
            "Assembly variants",
            Some(&format!("{} governed variants", variants.len())),
        );
        paint_table_header(
            left,
            &[0.25, 0.20, 0.19, 0.12, 0.24],
            &["Variant", "Parent", "Substitutions", "DNP", "Qualification"],
        );
        if variants.is_empty() {
            empty_table_row(
                left,
                "No assembly variants. Create the first governed variant.",
            );
        } else {
            for (id, name, parent, overrides) in &variants {
                let parent_name = parent
                    .and_then(|parent| names.iter().find(|(id, _)| *id == parent))
                    .map_or_else(|| "base".to_owned(), |(_, name)| name.clone());
                let substitutions = overrides
                    .values()
                    .filter(|value| {
                        matches!(
                            value,
                            crate::state::VariantObjectOverride::Substitute { .. }
                        )
                    })
                    .count();
                let dnp = overrides
                    .values()
                    .filter(|value| {
                        matches!(
                            value,
                            crate::state::VariantObjectOverride::DoNotPopulate { .. }
                        )
                    })
                    .count();
                let values = [
                    name.clone(),
                    parent_name,
                    substitutions.to_string(),
                    dnp.to_string(),
                    if Some(*id) == active {
                        "current"
                    } else {
                        "governed"
                    }
                    .to_owned(),
                ];
                paint_table_row(left, &[0.25, 0.20, 0.19, 0.12, 0.24], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("New variant\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::NewVariant);
            }
            if Button::new("Compare variants\u{2026}")
                .enabled(variants.len() > 1)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::CompareVariants);
            }
            if Button::new("Substitution matrix\u{2026}")
                .enabled(write_allowed && !variants.is_empty())
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::VariantMatrix);
            }
        });

        let mut selected = active;
        if setting_combo_by_id(
            right,
            "Active assembly variant",
            &names,
            &mut selected,
            write_allowed,
        ) && let Some(id) = selected
        {
            apply_domain_result(
                &mut dialog.error,
                draft.variants_mut().set_active(id),
                "Active variant changed.",
            );
        }
        let settings = draft.variants().settings().clone();
        let mut inheritance = settings.inheritance;
        let mut missing = settings.missing_replacement;
        let mut compatibility = settings.result_compatibility;
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Inheritance",
            &[
                (
                    VariantInheritance::OverrideChangedObjectsOnly,
                    "Override changed objects only",
                ),
                (
                    VariantInheritance::IndependentReviewedCopy,
                    "Materialize reviewed copy",
                ),
            ],
            &mut inheritance,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Missing replacement",
            &[
                (MissingReplacementPolicy::Block, "Block netlist and release"),
                (
                    MissingReplacementPolicy::ExplicitDoNotPopulate,
                    "DNP with explicit approval",
                ),
            ],
            &mut missing,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Result compatibility",
            &[
                (
                    crate::state::VariantResultCompatibility::ExactVariantIdentityRequired,
                    "Exact variant identity required",
                ),
                (
                    crate::state::VariantResultCompatibility::AllowReviewedOverlay,
                    "Allow reviewed overlay",
                ),
            ],
            &mut compatibility,
            write_allowed,
        );
        if changed {
            apply_domain_result(
                &mut dialog.error,
                draft
                    .variants_mut()
                    .set_settings(crate::state::AssemblyVariantSettings {
                        inheritance,
                        missing_replacement: missing,
                        model_equivalence: settings.model_equivalence,
                        result_compatibility: compatibility,
                    }),
                "Assembly variant contract updated.",
            );
        }
        let resolved = active
            .and_then(|id| draft.variants().resolve(id).ok())
            .map(|resolved| resolved.overrides)
            .unwrap_or_default();
        let substitutions = resolved
            .values()
            .filter_map(|value| match value {
                crate::state::VariantObjectOverride::Substitute { replacement } => {
                    Some(replacement)
                }
                crate::state::VariantObjectOverride::DoNotPopulate { .. } => None,
            })
            .collect::<Vec<_>>();
        let equivalent = substitutions
            .iter()
            .filter(|replacement| replacement.port_equivalence_digest.is_some())
            .count();
        let model_reviews = substitutions
            .iter()
            .filter(|replacement| {
                replacement.qualification != crate::state::VariantQualificationState::Current
            })
            .count();
        property_row_toned(
            right,
            "Port equivalence",
            &format!("{equivalent} / {}", substitutions.len()),
            if equivalent == substitutions.len() {
                Tokens::get(right.ctx()).color.ok
            } else {
                Tokens::get(right.ctx()).color.warn
            },
        );
        property_row_toned(
            right,
            "Model sections",
            &format!("{model_reviews} reviews"),
            if model_reviews == 0 {
                Tokens::get(right.ctx()).color.ok
            } else {
                Tokens::get(right.ctx()).color.warn
            },
        );
        property_row(right, "Release evidence", "variant-bound");
    });
    action
}

pub(super) fn annotation_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let policy = draft.annotation().policy().clone();
        let ranges = policy.definition().reserved_ranges.clone();
        section_header(
            left,
            "Reference-designator ownership",
            Some(if draft.annotation().journal().is_empty() {
                "preview required"
            } else {
                "mapping current"
            }),
        );
        paint_table_header(
            left,
            &[0.24, 0.19, 0.23, 0.13, 0.21],
            &["Scope", "Prefix", "Reserved range", "Used", "Collision"],
        );
        if ranges.is_empty() {
            empty_table_row(
                left,
                "No reserved ranges. Project allocation is policy-owned.",
            );
        } else {
            for range in &ranges {
                let scope = match &range.scope {
                    AnnotationRangeScope::Project => "/top".to_owned(),
                    AnnotationRangeScope::Sheet { sheet_id } => draft
                        .sheet_catalog(&dialog.owner_key)
                        .and_then(|catalog| catalog.find(*sheet_id))
                        .map_or_else(|| sheet_id.to_string(), |sheet| sheet.name().to_owned()),
                    AnnotationRangeScope::Hierarchy { path } => path.clone(),
                };
                let values = [
                    scope,
                    range.prefixes.join(", "),
                    format!("{}\u{2026}{}", range.first, range.last),
                    annotation_range_used(draft, range).to_string(),
                    "none".to_owned(),
                ];
                paint_table_row(left, &[0.24, 0.19, 0.23, 0.13, 0.21], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Preview renumbering\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::RenumberPreview);
            }
            if Button::new("Edit annotation policy\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::AnnotationPolicy);
            }
        });

        let mut definition = policy.definition().clone();
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Reference designators",
            &[
                (
                    crate::state::ReferenceDesignatorBehavior::StableAcrossVariants,
                    "Stable across variants",
                ),
                (
                    crate::state::ReferenceDesignatorBehavior::RenumberSelectedScope,
                    "Renumber selected scope",
                ),
            ],
            &mut definition.reference_designators,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Annotation scope",
            &[
                (
                    crate::state::DefaultAnnotationScope::WholeProject,
                    "Whole project",
                ),
                (
                    crate::state::DefaultAnnotationScope::CurrentHierarchy,
                    "Current hierarchy",
                ),
                (
                    crate::state::DefaultAnnotationScope::CurrentSheet,
                    "Current sheet",
                ),
            ],
            &mut definition.default_scope,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Collision policy",
            &[
                (
                    AnnotationCollisionPolicy::PreviewAndBlock,
                    "Preview and block",
                ),
                (
                    AnnotationCollisionPolicy::AllocateNextFreeRange,
                    "Allocate next free range",
                ),
            ],
            &mut definition.collision_policy,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Backannotation",
            &[
                (
                    crate::state::BackannotationPolicy::GenerateReviewedMapping,
                    "Generate reviewed mapping",
                ),
                (crate::state::BackannotationPolicy::Disabled, "Disabled"),
            ],
            &mut definition.backannotation,
            write_allowed,
        );
        if changed {
            let revision = policy.revision();
            apply_domain_result(
                &mut dialog.error,
                draft.annotation_mut().update_policy(revision, definition),
                "Annotation contract updated.",
            );
        }
        concept_banner(
            right,
            "A renumber operation never rewrites historical netlists, results, reports, or review anchors. It creates a stable old-to-new mapping.",
            true,
        );
    });
    action
}

pub(super) fn hierarchy_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    let resolution =
        workspace.resolve_hierarchy_with_active(libraries, &workspace.active_view, schematic);
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        section_header(
            left,
            "Hierarchy and view resolution",
            Some(&format!(
                "{} cells \u{00b7} {} views",
                resolution.bindings.len(),
                resolution.total_instances
            )),
        );
        paint_table_header(
            left,
            &[0.25, 0.19, 0.17, 0.22, 0.17],
            &[
                "Instance path",
                "Cell",
                "Design view",
                "Simulation view",
                "Configuration",
            ],
        );
        if resolution.bindings.is_empty() {
            empty_table_row(left, "The active hierarchy has no child view bindings.");
        } else {
            for binding in &resolution.bindings {
                let path = binding
                    .instance_paths
                    .first()
                    .cloned()
                    .unwrap_or_else(|| binding.reference.display_path());
                let configuration = resolution.configuration_id.map_or_else(
                    || "active project".to_owned(),
                    |id| {
                        workspace.configuration_sets.find(id).map_or_else(
                            || id.to_string(),
                            |configuration| configuration.name().to_owned(),
                        )
                    },
                );
                let values = [
                    path,
                    binding.reference.cell.clone(),
                    binding.reference.view.clone(),
                    binding
                        .stop_view
                        .clone()
                        .unwrap_or_else(|| binding.reference.view.clone()),
                    configuration,
                ];
                paint_table_row(left, &[0.25, 0.19, 0.17, 0.22, 0.17], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Edit hierarchy binding\u{2026}")
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::OpenConfigurationBinding;
            }
            if Button::new("Configuration sets\u{2026}").show(ui).clicked() {
                action = DesignManagementBodyAction::OpenConfigurationSets;
            }
            if Button::new("Audit unresolved views\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::HierarchyAudit);
            }
        });

        let mut settings = draft.hierarchy_settings().clone();
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Edit-in-place depth",
            &[
                (
                    crate::state::EditInPlaceDepth::CurrentAndParentContext,
                    "Current + parent context",
                ),
                (crate::state::EditInPlaceDepth::CurrentOnly, "Current only"),
                (
                    crate::state::EditInPlaceDepth::TwoParentLevels,
                    "Two parent levels",
                ),
            ],
            &mut settings.edit_in_place_depth,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Missing view",
            &[
                (
                    crate::state::MissingHierarchyViewPolicy::BlockNetlist,
                    "Block netlist",
                ),
                (
                    crate::state::MissingHierarchyViewPolicy::UseDeclaredFallbackOrder,
                    "Use declared fallback order",
                ),
            ],
            &mut settings.missing_view,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Black-box policy",
            &[
                (
                    crate::state::HierarchyBlackBoxPolicy::RequireSignedBoundaryContract,
                    "Require signed boundary contract",
                ),
                (
                    crate::state::HierarchyBlackBoxPolicy::AllowProjectAbstract,
                    "Allow project abstract",
                ),
            ],
            &mut settings.black_box,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Cycle detection",
            &[(
                crate::state::HierarchyCyclePolicy::BlockSaveAndIdentifyPath,
                "Block save and identify path",
            )],
            &mut settings.cycle_detection,
            write_allowed,
        );
        if changed {
            apply_domain_result(
                &mut dialog.error,
                draft.set_hierarchy_settings(settings),
                "Hierarchy contract updated.",
            );
        }
        let t = Tokens::get(right.ctx());
        property_row_toned(
            right,
            "Unresolved views",
            &resolution.unresolved_instances().to_string(),
            if resolution.is_valid() {
                t.color.ok
            } else {
                t.color.err
            },
        );
        let protected =
            draft
                .hierarchy_audits()
                .last()
                .map_or("not audited".to_owned(), |receipt| {
                    if receipt.passed() {
                        "qualified"
                    } else {
                        "findings"
                    }
                    .to_owned()
                });
        property_row(right, "Protected boundaries", &protected);
        let digest = resolution
            .configuration_digest
            .map_or_else(|| "active-project".to_owned(), |digest| digest.to_string());
        property_row(right, "Configuration digest", &digest);
    });
    action
}

pub(super) fn annotation_range_used(
    catalog: &DesignManagementCatalog,
    range: &AnnotationReservedRange,
) -> usize {
    catalog
        .annotation()
        .journal()
        .iter()
        .flat_map(|entry| entry.mappings().values())
        .filter(|mapping| {
            range.prefixes.iter().any(|prefix| {
                mapping.new_reference.starts_with(prefix)
                    && mapping.new_reference[prefix.len()..]
                        .parse::<u32>()
                        .is_ok_and(|number| number >= range.first && number <= range.last)
            })
        })
        .count()
}
