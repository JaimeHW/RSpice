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
        DesignManagementTab::Annotation => {
            super::operations::refresh_range_use(dialog, workspace, schematic);
            annotation_manager_body(ui, dialog, write_allowed)
        }
        DesignManagementTab::Hierarchy => {
            hierarchy_manager_body(ui, dialog, workspace, libraries, schematic, write_allowed)
        }
    };
    // Only what the operation did that the tabs do not already show.
    if let Some(outcome) = dialog.outcome.as_deref() {
        muted_note(ui, outcome);
    }
    let _ = first_focus;
    action
}

pub(super) fn design_management_tabs(
    ui: &mut Ui,
    active: &mut DesignManagementTab,
) -> egui::Response {
    let labels = DesignManagementTab::ALL.map(DesignManagementTab::label);
    let mut index = DesignManagementTab::ALL
        .iter()
        .position(|tab| tab == active)
        .unwrap_or_default();
    let first = crate::ui::widgets::dialog_tabs(ui, "design-management-tabs", &labels, &mut index);
    *active = DesignManagementTab::ALL[index];
    first
}

/// How many cross-sheet ports reach `sheet`, and how many nets leave it
/// through them. A port pairs this sheet with one other for one net, so a
/// net that reaches two other sheets is two ports and one net. Net names fold
/// case, as the catalog's own port key does.
pub(super) fn sheet_crossings<'a>(
    ports: impl IntoIterator<Item = &'a CrossSheetPortDefinition>,
    sheet: SheetId,
) -> (usize, usize) {
    let mut port_count = 0;
    let mut nets = BTreeSet::new();
    for port in ports {
        if port.first.sheet_id == sheet || port.second.sheet_id == sheet {
            port_count += 1;
            nets.insert(port.net_name.to_lowercase());
        }
    }
    (port_count, nets.len())
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
            Some(&counted(sheets.len(), "sheet")),
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
                let (port_count, net_count) =
                    sheet_crossings(ports.iter().map(|port| port.definition()), *id);
                let values = [
                    format!("{:02}", index + 1),
                    name.clone(),
                    port_count.to_string(),
                    net_count.to_string(),
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

        // Which sheet is current is navigation, and the sheet strip owns it:
        // a second owner inside this draft would publish an activation the
        // session had already left, and the reviewed order lives in the
        // strip's drag rather than in a free-text list of names.
        section_header(right, "Sheet contract", None);
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
                        SheetDeleteBehavior::DeleteObjects,
                        "Delete with its objects",
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
            Some(&counted(variants.len(), "variant")),
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
            // One variant is compared with the base design.
            if Button::new("Compare variants\u{2026}")
                .enabled(!variants.is_empty())
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::CompareVariants);
            }
            if Button::new("Substitution matrix\u{2026}")
                .enabled(!variants.is_empty())
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::VariantMatrix);
            }
        });

        // With no variant there is nothing to choose: the row says so,
        // rather than offering an empty list.
        if names.is_empty() {
            fact_row(right, ACTIVE_VARIANT, "None");
        } else {
            let mut selected = active;
            if setting_combo_by_id(right, ACTIVE_VARIANT, &names, &mut selected, write_allowed)
                && let Some(id) = selected
            {
                apply_domain_result(
                    &mut dialog.error,
                    draft.variants_mut().set_active(id),
                    "Active variant changed.",
                );
            }
        }
        let settings = draft.variants().settings().clone();
        let mut inheritance = settings.inheritance;
        let mut equivalence = settings.model_equivalence;
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
        // Which replacements the inspector offers a part as its substitute.
        changed |= setting_combo(
            right,
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
            &mut equivalence,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Result compatibility",
            &[
                (
                    crate::state::VariantResultCompatibility::ExactVariantIdentityRequired,
                    "Same variant only",
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
                        model_equivalence: equivalence,
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
                crate::state::VariantObjectOverride::DoNotPopulate { .. }
                | crate::state::VariantObjectOverride::Fitted { .. } => None,
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
        // Both rows are counts, and a count reads as a count.
        fact_row(
            right,
            "Port equivalence",
            &format!("{equivalent} of {}", substitutions.len()),
        );
        fact_row(right, "Model sections", &counted(model_reviews, "review"));
    });
    action
}

/// The Variants tab's row naming the variant the design builds.
const ACTIVE_VARIANT: &str = "Active assembly variant";

pub(super) fn annotation_manager_body(
    ui: &mut Ui,
    dialog: &DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_ref() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let policy = draft.annotation().policy().clone();
        let ranges = policy.definition().reserved_ranges.clone();
        section_header(left, "Reserved ranges", None);
        const COLUMNS: [f32; 5] = [0.17, 0.13, 0.19, 0.1, 0.41];
        paint_table_header(
            left,
            &COLUMNS,
            &["Scope", "Prefix", "Reserved range", "Used", "Collision"],
        );
        if ranges.is_empty() {
            empty_table_row(left, "No reserved ranges: every prefix numbers from 1.");
        } else {
            let scope = |range: &AnnotationReservedRange| match &range.scope {
                AnnotationRangeScope::Project => crate::state::InstancePath::root().to_string(),
                AnnotationRangeScope::Sheet { sheet_id } => draft
                    .sheet_catalog(&dialog.owner_key)
                    .and_then(|catalog| catalog.find(*sheet_id))
                    .map_or_else(|| sheet_id.to_string(), |sheet| sheet.name().to_owned()),
                AnnotationRangeScope::Hierarchy { path } => path.clone(),
            };
            let uses = dialog.range_use.as_ref().map(|(_, uses)| uses.as_slice());
            for (index, range) in ranges.iter().enumerate() {
                let range_use = uses.and_then(|uses| uses.get(index));
                // A collision is a name in the range that a part outside its
                // scope holds, or another range reserving the same names.
                let mut collisions = Vec::new();
                if let Some(range_use) = range_use {
                    let foreign = &range_use.foreign;
                    if !foreign.is_empty() {
                        let shown = foreign
                            .iter()
                            .take(3)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ");
                        collisions.push(if foreign.len() > 3 {
                            format!("{shown} and {} more", foreign.len() - 3)
                        } else {
                            shown
                        });
                    }
                    for other in range_use
                        .overlapping
                        .iter()
                        .filter_map(|other| ranges.get(*other))
                    {
                        collisions.push(format!(
                            "overlaps {} {}\u{2026}{}",
                            scope(other),
                            other.first,
                            other.last
                        ));
                    }
                }
                let values = [
                    scope(range),
                    range.prefixes.join(", "),
                    format!("{}\u{2026}{}", range.first, range.last),
                    range_use.map_or_else(String::new, |range_use| range_use.used.to_string()),
                    if collisions.is_empty() {
                        "none".to_owned()
                    } else {
                        collisions.join("; ")
                    },
                ];
                paint_table_row(left, &COLUMNS, &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Renumber\u{2026}")
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

        // The policy in force, which Edit annotation policy changes, and
        // what the last renumbering did.
        section_header(right, "Policy", None);
        let allocation = policy.definition().prefix_allocation;
        fact_row(
            right,
            "Prefix allocation",
            PREFIX_ALLOCATIONS
                .iter()
                .find(|(value, _)| *value == allocation)
                .map_or("", |(_, label)| label),
        );
        let last = draft
            .annotation()
            .journal()
            .iter()
            .rev()
            .find(|entry| {
                entry.origin() == crate::state::AnnotationJournalOrigin::ReviewedRenumbering
            })
            .map_or_else(
                || "None".to_owned(),
                |entry| {
                    let changed = entry
                        .mappings()
                        .values()
                        .filter(|mapping| mapping.old_reference != mapping.new_reference)
                        .count();
                    format!("{} changed", counted(changed, "name"))
                },
            );
        fact_row(right, "Last renumbering", &last);
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
    let resolution = workspace.resolve_hierarchy_with_active(
        libraries,
        &workspace.content.active_view,
        schematic,
    );
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        section_header(
            left,
            "Hierarchy and view resolution",
            Some(&format!(
                "{} \u{00b7} {}",
                counted(resolution.bindings.len(), "cell"),
                counted(resolution.total_instances, "view")
            )),
        );
        paint_table_header(
            left,
            &[0.20, 0.14, 0.19, 0.22, 0.25],
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
                    .occurrences
                    .first()
                    .map_or_else(|| binding.reference.display_path(), |path| path.to_string());
                let configuration = resolution.configuration_id.map_or_else(
                    || "active project".to_owned(),
                    |id| {
                        workspace.content.configuration_sets.find(id).map_or_else(
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
                paint_table_row(left, &[0.20, 0.14, 0.19, 0.22, 0.25], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Edit hierarchy binding\u{2026}")
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::OpenConfigurationBinding;
            }
            if Button::new("Configurations").show(ui).clicked() {
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
        property_row_problem(
            right,
            "Unresolved views",
            &resolution.unresolved_instances().to_string(),
            (!resolution.is_valid()).then_some(Problem::Failure),
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
        fact_row(right, "Protected boundaries", &protected);
        let digest = resolution
            .configuration_digest
            .map_or_else(|| "active-project".to_owned(), |digest| digest.to_string());
        property_row(right, "Configuration digest", &digest);
    });
    action
}
