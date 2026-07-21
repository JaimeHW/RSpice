use super::*;

fn sheet(name: &str, page: u32) -> SheetDefinition {
    SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: Some(page),
    }
}

fn substitution(cell: &str, qualification: VariantQualificationState) -> ComponentSubstitution {
    ComponentSubstitution {
        library: "project".to_owned(),
        cell: cell.to_owned(),
        view: "schematic".to_owned(),
        value_override: None,
        model_section: None,
        port_equivalence_digest: Some(ContentDigest::from_bytes([7; 32])),
        qualification,
    }
}

fn variant_draft(
    name: &str,
    parent_id: Option<AssemblyVariantId>,
    overrides: BTreeMap<SchematicObjectKey, VariantObjectOverride>,
) -> AssemblyVariantDraft {
    AssemblyVariantDraft {
        name: name.to_owned(),
        parent_id,
        inheritance: VariantInheritance::OverrideChangedObjectsOnly,
        qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
        overrides,
    }
}

fn annotation_object(id: u64, reference: &str, x: i64) -> AnnotationObject {
    AnnotationObject {
        object: object_key(id),
        current_reference: reference.to_owned(),
        device_family: "R".to_owned(),
        sheet_id: None,
        hierarchy_path: "/top".to_owned(),
        position: AnnotationPosition { x, y: 0 },
        connectivity_order: Some(id),
        locked: false,
        external: false,
        imported: false,
    }
}

fn object_key(id: u64) -> SchematicObjectKey {
    SchematicObjectKey::new("work/top/schematic", id).unwrap()
}

#[test]
fn empty_catalog_round_trips_and_rejects_unknown_fields() {
    let catalog = DesignManagementCatalog::default();
    assert!(catalog.is_empty());
    catalog.validate().expect("default catalog validates");
    let encoded = serde_json::to_string(&catalog).expect("serialize catalog");
    let decoded: DesignManagementCatalog =
        serde_json::from_str(&encoded).expect("deserialize catalog");
    assert_eq!(decoded, catalog);
    assert_eq!(
        decoded.semantic_digest().unwrap(),
        catalog.semantic_digest().unwrap()
    );

    let mut value = serde_json::to_value(catalog).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("invented".to_owned(), serde_json::Value::Bool(true));
    assert!(serde_json::from_value::<DesignManagementCatalog>(value).is_err());
}

#[test]
fn sheet_order_changes_presentation_without_changing_stable_identity() {
    let mut catalog = SheetCatalog::default();
    let afe = catalog.create_sheet(sheet("AFE core", 1), None).unwrap();
    let bias = catalog
        .create_sheet(sheet("Bias and reference", 2), Some(afe))
        .unwrap();
    let afe_digest = catalog.find(afe).unwrap().semantic_digest();
    let revision = catalog.revision();

    catalog
        .reorder(
            revision,
            vec![bias, afe],
            ReorderPageNumbering::RetainExplicitPageNumbers,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        )
        .unwrap();

    assert_eq!(
        catalog
            .sheets()
            .iter()
            .map(DesignSheet::id)
            .collect::<Vec<_>>(),
        vec![bias, afe]
    );
    assert_eq!(catalog.find(afe).unwrap().semantic_digest(), afe_digest);
    assert!(matches!(
        catalog.reorder(
            revision,
            vec![afe, bias],
            ReorderPageNumbering::RetainExplicitPageNumbers,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        ),
        Err(DesignManagementError::RevisionConflict { .. })
    ));
}

#[test]
fn sheet_move_and_reconciliation_are_atomic_and_remove_dead_ports() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1, 2])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), destination, [3])
        .unwrap();
    let receipt = catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: destination,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "VREF".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: source,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: 2,
                            point: Point::new(14, 9),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: destination,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: 3,
                            terminal_name: "VREF".to_owned(),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .unwrap();
    assert_eq!(catalog.sheet_for_object(1), Some(destination));
    assert_eq!(receipt.created_port_ids.len(), 1);
    let retained = catalog.cross_sheet_ports()[0].definition();
    assert_eq!(
        retained.first.anchor,
        CrossSheetPortAnchor::WirePoint {
            wire_id: 2,
            point: Point::new(14, 9),
        }
    );
    assert_eq!(retained.second.object_id(), 3);

    let reconciled = catalog
        .reconcile_object_assignments(catalog.revision(), [1, 4], Some(destination))
        .unwrap();
    assert_eq!(reconciled.added_assignments, 1);
    assert_eq!(reconciled.removed_assignments, 2);
    assert_eq!(reconciled.removed_cross_sheet_ports, 1);
    assert_eq!(catalog.sheet_for_object(4), Some(destination));
    assert!(catalog.cross_sheet_ports().is_empty());
}

#[test]
fn typed_cross_sheet_anchor_rejects_ambiguous_terminal_without_mutation() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1, 2])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), destination, [3])
        .unwrap();
    let before = catalog.clone();
    let result = catalog.move_selection(MoveSelectionRequest {
        expected_catalog_revision: catalog.revision(),
        object_ids: vec![1],
        destination_sheet_id: destination,
        boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
            ports: vec![CrossSheetPortDefinition {
                net_name: "VREF".to_owned(),
                first: CrossSheetPortEndpoint {
                    sheet_id: source,
                    anchor: CrossSheetPortAnchor::ComponentTerminal {
                        component_id: 2,
                        terminal_name: String::new(),
                    },
                },
                second: CrossSheetPortEndpoint {
                    sheet_id: destination,
                    anchor: CrossSheetPortAnchor::WirePoint {
                        wire_id: 3,
                        point: Point::new(0, 0),
                    },
                },
                direction: CrossSheetPortDirection::Output,
                signal_type: CrossSheetSignalType::Analog,
                discipline: CrossSheetDiscipline::Electrical,
            }],
        },
    });
    assert!(matches!(
        result,
        Err(DesignManagementError::InvalidText {
            field: "component terminal",
            ..
        })
    ));
    assert_eq!(catalog, before);
}

#[test]
fn verified_empty_boundary_move_is_distinct_from_empty_explicit_ports() {
    let mut catalog = SheetCatalog::default();
    let source = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let destination = catalog.create_sheet(sheet("AFE", 2), Some(source)).unwrap();
    catalog
        .assign_objects(catalog.revision(), source, [1])
        .unwrap();
    let receipt = catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: destination,
            boundary_resolution: MoveBoundaryResolution::VerifiedNoBoundaryNets,
        })
        .unwrap();
    assert!(receipt.created_port_ids.is_empty());
    assert_eq!(catalog.sheet_for_object(1), Some(destination));

    let before = catalog.clone();
    assert!(matches!(
        catalog.move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![1],
            destination_sheet_id: source,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts { ports: Vec::new() },
        }),
        Err(DesignManagementError::EmptyExplicitBoundaryPorts)
    ));
    assert_eq!(catalog, before);
}

#[test]
fn variant_resolution_is_immutable_and_comparison_does_not_mutate() {
    let mut catalog = AssemblyVariantCatalog::default();
    let base = catalog
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                object_key(10),
                VariantObjectOverride::Substitute {
                    replacement: substitution(
                        "resistor_industrial",
                        VariantQualificationState::Current,
                    ),
                },
            )]),
        ))
        .unwrap();
    let child = catalog
        .create(variant_draft(
            "Automotive",
            Some(base),
            BTreeMap::from([(
                object_key(11),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-42".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let before = catalog.clone();
    let resolved = catalog.resolve(child).unwrap();
    assert_eq!(resolved.lineage.len(), 2);
    assert_eq!(resolved.overrides.len(), 2);
    let comparison = catalog.compare(base, child).unwrap();
    assert_eq!(comparison.differences.len(), 1);
    assert_eq!(catalog, before, "comparison must be read-only");

    let base_revision = catalog.find(base).unwrap().revision();
    assert!(matches!(
        catalog.update(
            base,
            base_revision,
            variant_draft("Industrial revised", None, BTreeMap::new()),
        ),
        Err(DesignManagementError::VariantHasDependents(id)) if id == base
    ));
    assert_eq!(catalog, before);
}

#[test]
fn substitution_matrix_enforces_qualification_before_any_commit() {
    let mut catalog = AssemblyVariantCatalog::default();
    let id = catalog
        .create(variant_draft("Industrial", None, BTreeMap::new()))
        .unwrap();
    let revision = catalog.find(id).unwrap().revision();
    let before = catalog.clone();
    let result = catalog.apply_substitution_matrix(
        vec![VariantMatrixEdit {
            variant_id: id,
            expected_revision: revision,
            object: object_key(44),
            replacement: Some(substitution(
                "candidate",
                VariantQualificationState::ReviewRequired,
            )),
        }],
        MissingReplacementPolicy::Block,
        ModelEquivalencePolicy::RequireQualifiedReplacement,
    );
    assert!(matches!(
        result,
        Err(DesignManagementError::UnqualifiedReplacement(object)) if object == object_key(44)
    ));
    assert_eq!(catalog, before);
}

#[test]
fn renumber_preview_is_deterministic_and_commit_retains_immutable_mapping() {
    let mut state = AnnotationState::default();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![
            annotation_object(2, "R20", 20),
            annotation_object(1, "R10", 10),
        ],
    };
    let first = state.preview_renumbering(&request).unwrap();
    let second = state.preview_renumbering(&request).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first.mappings.get(&object_key(1)).unwrap().new_reference,
        "R1"
    );
    assert_eq!(
        first.mappings.get(&object_key(2)).unwrap().new_reference,
        "R2"
    );

    let id = state.commit_renumbering(&first, &request).unwrap();
    let retained = state.journal().last().unwrap();
    assert_eq!(retained.id(), id);
    assert_eq!(retained.mappings(), &first.mappings);
    let encoded = serde_json::to_string(&state).unwrap();
    let restored: AnnotationState = serde_json::from_str(&encoded).unwrap();
    assert_eq!(restored, state);
}

#[test]
fn invalid_annotation_policy_is_rejected_without_partial_mutation() {
    let mut state = AnnotationState::default();
    let before = state.clone();
    let definition = AnnotationPolicyDefinition {
        reserved_ranges: vec![
            AnnotationReservedRange {
                scope: AnnotationRangeScope::Project,
                prefixes: vec!["R".to_owned()],
                first: 1,
                last: 10,
            },
            AnnotationReservedRange {
                scope: AnnotationRangeScope::Project,
                prefixes: vec!["R".to_owned()],
                first: 5,
                last: 15,
            },
        ],
        ..AnnotationPolicyDefinition::default()
    };
    let revision = state.policy().revision();
    assert!(matches!(
        state.update_policy(revision, definition),
        Err(DesignManagementError::OverlappingAnnotationRanges)
    ));
    assert_eq!(state, before);
}

#[test]
fn effective_annotation_folds_partial_journal_entries_in_sequence_order() {
    let mut state = AnnotationState::default();
    let first_sheet = SheetId::new();
    let second_sheet = SheetId::new();
    let mut first_object = annotation_object(1, "R10", 10);
    first_object.sheet_id = Some(first_sheet);
    let mut second_object = annotation_object(2, "R20", 20);
    second_object.sheet_id = Some(second_sheet);
    let first_request = RenumberRequest {
        scope: RenumberScope::CurrentSheet {
            sheet_id: first_sheet,
        },
        order: RenumberOrder::SheetThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![first_object.clone(), second_object.clone()],
    };
    let first_preview = state.preview_renumbering(&first_request).unwrap();
    state
        .commit_renumbering(&first_preview, &first_request)
        .unwrap();

    first_object.current_reference = first_preview.mappings[&object_key(1)].new_reference.clone();
    let second_request = RenumberRequest {
        scope: RenumberScope::CurrentSheet {
            sheet_id: second_sheet,
        },
        order: RenumberOrder::SheetThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![first_object, second_object],
    };
    let second_preview = state.preview_renumbering(&second_request).unwrap();
    state
        .commit_renumbering(&second_preview, &second_request)
        .unwrap();

    let effective = state.effective_mappings();
    assert_eq!(effective.len(), 2);
    assert_eq!(effective[&object_key(1)].new_reference, "R1");
    assert_eq!(effective[&object_key(2)].new_reference, "R2");
}

#[test]
fn annotation_authority_rejects_cycles_and_unrelated_object_conflation() {
    let mut state = AnnotationState::default();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![
            annotation_object(1, "R10", 10),
            annotation_object(2, "R20", 20),
        ],
    };
    let preview = state.preview_renumbering(&request).unwrap();
    state.commit_renumbering(&preview, &request).unwrap();
    let target = object_key(3);
    state.object_authorities.insert(
        object_key(1),
        AnnotationObjectAuthority::Redirect {
            target: target.clone(),
        },
    );
    state.object_authorities.insert(
        object_key(2),
        AnnotationObjectAuthority::Redirect {
            target: target.clone(),
        },
    );
    assert!(matches!(
        state.validate(),
        Err(DesignManagementError::AnnotationAuthorityConflation {
            target: ref actual,
            ..
        }) if actual == &target
    ));

    let mut cycle = AnnotationState::default();
    cycle.object_authorities.insert(
        object_key(1),
        AnnotationObjectAuthority::Redirect {
            target: object_key(2),
        },
    );
    cycle.object_authorities.insert(
        object_key(2),
        AnnotationObjectAuthority::Redirect {
            target: object_key(1),
        },
    );
    assert!(matches!(
        cycle.validate(),
        Err(DesignManagementError::AnnotationAuthorityCycle(_))
    ));
}

#[test]
fn hierarchy_audit_records_cycle_and_protected_boundary_failures() {
    let mut catalog = DesignManagementCatalog::default();
    let request = HierarchyAuditRequest {
        configuration: HierarchyAuditConfiguration::ActiveProject,
        view_checks: HierarchyViewChecks::AllDeclaredFallbacks,
        protected_boundaries: ProtectedBoundaryChecks::ValidateSignaturesAndPins,
        subjects: vec![
            HierarchyAuditSubject {
                instance_path: "/top".to_owned(),
                cell_name: "top".to_owned(),
                design_view: "schematic".to_owned(),
                declared_fallbacks: vec!["schematic".to_owned()],
                resolved_simulation_view: Some("schematic".to_owned()),
                fallback_used: Some("schematic".to_owned()),
                child_instance_paths: vec!["/top/X1".to_owned()],
                protected_boundary_id: None,
            },
            HierarchyAuditSubject {
                instance_path: "/top/X1".to_owned(),
                cell_name: "vendor".to_owned(),
                design_view: "symbol".to_owned(),
                declared_fallbacks: vec!["protected-spice".to_owned()],
                resolved_simulation_view: Some("protected-spice".to_owned()),
                fallback_used: Some("protected-spice".to_owned()),
                child_instance_paths: vec!["/top".to_owned()],
                protected_boundary_id: Some("vendor-boundary".to_owned()),
            },
        ],
        boundary_evidence: vec![ProtectedBoundaryEvidence {
            boundary_id: "vendor-boundary".to_owned(),
            signature_valid: false,
            pins_match: false,
        }],
    };
    let id = catalog.run_and_record_hierarchy_audit(&request).unwrap();
    let receipt = catalog.hierarchy_audits().last().unwrap();
    assert_eq!(receipt.id(), id);
    assert!(!receipt.passed());
    let kinds = receipt
        .findings()
        .iter()
        .map(|finding| finding.kind)
        .collect::<BTreeSet<_>>();
    assert!(kinds.contains(&HierarchyAuditFindingKind::HierarchyCycle));
    assert!(kinds.contains(&HierarchyAuditFindingKind::InvalidProtectedBoundarySignature));
    assert!(kinds.contains(&HierarchyAuditFindingKind::ProtectedBoundaryPinMismatch));
}

#[test]
fn cell_view_sheet_ownership_and_reviewed_publish_are_deterministic() {
    let mut live = DesignManagementCatalog::default();
    let sheet_id = live
        .bootstrap_for_cell_view(" Project/Top/Schematic ", "Main", [8, 9])
        .unwrap();
    assert_eq!(
        live.sheet_for_object_or_active("project/top/schematic", 8),
        Some(sheet_id)
    );
    assert_eq!(
        live.sheet_for_object_or_active("PROJECT/TOP/SCHEMATIC", 500),
        Some(sheet_id),
        "legacy unassigned objects inherit active/first sheet without mutation"
    );
    let original_revision = live.revision();
    let mut candidate = live.clone();
    candidate
        .sheet_catalog_mut("project/top/schematic")
        .unwrap()
        .create_sheet(sheet("Power", 2), Some(sheet_id))
        .unwrap();
    let new_revision = live
        .publish_reviewed_candidate(original_revision, candidate)
        .unwrap();
    assert_eq!(new_revision, original_revision + 1);
    assert_eq!(
        live.sheet_catalog("project/top/schematic")
            .unwrap()
            .sheets()
            .len(),
        2
    );
}

#[test]
fn cell_rename_and_delete_remap_sheet_catalog_ownership_atomically() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    catalog
        .bootstrap_for_cell_view("work/amp/testbench", "Bench", [2])
        .unwrap();
    let original_revision = catalog.revision();
    let renamed = catalog
        .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
        .unwrap();
    assert_eq!(renamed.affected_sheet_catalogs, 2);
    assert_eq!(renamed.catalog_revision, original_revision + 1);
    assert!(catalog.sheet_catalog("work/amp/schematic").is_none());
    assert!(catalog.sheet_catalog("work/amp_rev_b/schematic").is_some());

    let removed = catalog
        .remove_sheet_catalog_for_view("work/amp_rev_b/testbench")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    let removed = catalog
        .remove_sheet_catalogs_for_cell("work", "amp_rev_b")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    assert!(catalog.sheet_catalogs().is_empty());
    let revision = catalog.revision();
    let copy = catalog
        .copy_cell_sheet_catalogs("work", "missing", "work", "copy")
        .unwrap();
    assert_eq!(copy.copied_sheet_catalogs, 0);
    assert_eq!(copy.catalog_revision, revision);
    assert_eq!(catalog.revision(), revision);
}

#[test]
fn deleting_sheet_ownership_blocks_while_annotation_range_references_it() {
    let mut catalog = DesignManagementCatalog::default();
    let sheet_id = catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let mut policy = catalog.annotation().policy().definition().clone();
    policy.reserved_ranges.push(AnnotationReservedRange {
        scope: AnnotationRangeScope::Sheet { sheet_id },
        prefixes: vec!["R".to_owned()],
        first: 1,
        last: 399,
    });
    let revision = catalog.annotation().policy().revision();
    catalog
        .annotation_mut()
        .update_policy(revision, policy)
        .unwrap();
    let before = catalog.clone();
    assert!(matches!(
        catalog.remove_sheet_catalog_for_view("work/amp/schematic"),
        Err(DesignManagementError::SheetCatalogReferenced(id)) if id == sheet_id
    ));
    assert_eq!(catalog, before);
}

#[test]
fn cell_rename_remaps_live_variant_and_annotation_object_owners() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let old_object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
    let new_object = SchematicObjectKey::new("work/amp_rev_b/schematic", 1).unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                old_object.clone(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-9".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: old_object.clone(),
            current_reference: "R10".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: catalog
                .sheet_catalog("work/amp/schematic")
                .unwrap()
                .active_sheet_id(),
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = catalog.annotation().preview_renumbering(&request).unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .unwrap();

    let receipt = catalog
        .rename_cell_sheet_catalogs("work", "amp", "amp_rev_b")
        .unwrap();
    assert_eq!(receipt.remapped_variant_objects, 1);
    assert_eq!(receipt.remapped_annotation_objects, 1);
    let resolved = catalog.variants().resolve(variant).unwrap();
    assert!(!resolved.overrides.contains_key(&old_object));
    assert!(resolved.overrides.contains_key(&new_object));
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp_rev_b/schematic", 1)
            .unwrap()
            .is_some()
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp/schematic", 1)
            .unwrap()
            .is_none(),
        "renamed ownership must not remain effective under the old key"
    );
    assert_eq!(catalog.annotation().journal().len(), 1);
    assert!(matches!(
        catalog.annotation().object_authorities().get(&old_object),
        Some(AnnotationObjectAuthority::Redirect { target }) if target == &new_object
    ));
    let encoded = serde_json::to_string(&catalog).unwrap();
    let restored: DesignManagementCatalog = serde_json::from_str(&encoded).unwrap();
    assert_eq!(restored, catalog);
}

#[test]
fn cell_delete_blocks_live_variant_then_tombstones_annotation_without_rewriting_history() {
    let mut catalog = DesignManagementCatalog::default();
    catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1])
        .unwrap();
    let object = SchematicObjectKey::new("work/amp/schematic", 1).unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                object.clone(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-11".to_owned(),
                },
            )]),
        ))
        .unwrap();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: object.clone(),
            current_reference: "R8".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = catalog.annotation().preview_renumbering(&request).unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .unwrap();

    let before = catalog.clone();
    assert!(matches!(
        catalog.remove_sheet_catalogs_for_cell("work", "amp"),
        Err(DesignManagementError::LiveVariantObjectReference { variant: id, object: ref key })
            if id == variant && key == &object
    ));
    assert_eq!(catalog, before, "blocked deletion must be atomic");

    let revision = catalog.variants().find(variant).unwrap().revision();
    catalog
        .variants_mut()
        .update(
            variant,
            revision,
            variant_draft("Industrial", None, BTreeMap::new()),
        )
        .unwrap();
    let removed = catalog
        .remove_sheet_catalogs_for_cell("work", "amp")
        .unwrap();
    assert_eq!(removed.affected_sheet_catalogs, 1);
    assert_eq!(removed.remapped_annotation_objects, 1);
    assert_eq!(catalog.annotation().journal().len(), 1);
    assert!(catalog.annotation().effective_mappings().is_empty());
    assert!(matches!(
        catalog.annotation().object_authorities().get(&object),
        Some(AnnotationObjectAuthority::Tombstone)
    ));
    assert!(matches!(
        catalog.annotation().preview_renumbering(&request),
        Err(DesignManagementError::InactiveAnnotationObjectAuthority(ref key)) if key == &object
    ));
}

#[test]
fn cell_copy_regenerates_sheet_port_identity_and_clones_sheet_annotation_policy() {
    let mut catalog = DesignManagementCatalog::default();
    let main = catalog
        .bootstrap_for_cell_view("work/amp/schematic", "Main", [1, 2, 3])
        .unwrap();
    let sheets = catalog.sheet_catalog_mut("work/amp/schematic").unwrap();
    let auxiliary = sheets
        .create_sheet(sheet("Auxiliary", 2), Some(main))
        .unwrap();
    sheets
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: sheets.revision(),
            object_ids: vec![3],
            destination_sheet_id: auxiliary,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![CrossSheetPortDefinition {
                    net_name: "BIAS".to_owned(),
                    first: CrossSheetPortEndpoint {
                        sheet_id: main,
                        anchor: CrossSheetPortAnchor::ComponentTerminal {
                            component_id: 1,
                            terminal_name: "BIAS_OUT".to_owned(),
                        },
                    },
                    second: CrossSheetPortEndpoint {
                        sheet_id: auxiliary,
                        anchor: CrossSheetPortAnchor::WirePoint {
                            wire_id: 3,
                            point: Point::new(21, -4),
                        },
                    },
                    direction: CrossSheetPortDirection::Output,
                    signal_type: CrossSheetSignalType::Analog,
                    discipline: CrossSheetDiscipline::Electrical,
                }],
            },
        })
        .unwrap();
    let source_port = sheets.cross_sheet_ports()[0].id();
    let mut policy = catalog.annotation().policy().definition().clone();
    policy.reserved_ranges.push(AnnotationReservedRange {
        scope: AnnotationRangeScope::Sheet { sheet_id: main },
        prefixes: vec!["R".to_owned()],
        first: 1,
        last: 399,
    });
    let policy_revision = catalog.annotation().policy().revision();
    catalog
        .annotation_mut()
        .update_policy(policy_revision, policy)
        .unwrap();
    let annotation_request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
            current_reference: "R12".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(main),
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let annotation_preview = catalog
        .annotation()
        .preview_renumbering(&annotation_request)
        .unwrap();
    catalog
        .annotation_mut()
        .commit_renumbering(&annotation_preview, &annotation_request)
        .unwrap();
    let variant = catalog
        .variants_mut()
        .create(variant_draft(
            "Industrial",
            None,
            BTreeMap::from([(
                SchematicObjectKey::new("work/amp/schematic", 1).unwrap(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-1".to_owned(),
                },
            )]),
        ))
        .unwrap();

    let receipt = catalog
        .copy_cell_sheet_catalogs("work", "amp", "work", "amp_copy")
        .unwrap();
    assert_eq!(receipt.copied_sheet_catalogs, 1);
    assert_ne!(receipt.sheet_identity_map[&main], main);
    assert_ne!(receipt.port_identity_map[&source_port], source_port);
    let copied = catalog.sheet_catalog("work/amp_copy/schematic").unwrap();
    assert_eq!(
        copied.sheet_for_object(1),
        Some(receipt.sheet_identity_map[&main])
    );
    assert_eq!(
        copied.cross_sheet_ports()[0].definition().first.sheet_id,
        receipt.sheet_identity_map[&main]
    );
    assert_eq!(
        copied.cross_sheet_ports()[0].definition().first.anchor,
        CrossSheetPortAnchor::ComponentTerminal {
            component_id: 1,
            terminal_name: "BIAS_OUT".to_owned(),
        }
    );
    assert!(
        catalog
            .annotation()
            .policy()
            .definition()
            .reserved_ranges
            .iter()
            .any(|range| range.scope
                == AnnotationRangeScope::Sheet {
                    sheet_id: receipt.sheet_identity_map[&main],
                })
    );
    let resolved = catalog.variants().resolve(variant).unwrap();
    assert!(
        resolved
            .overrides
            .contains_key(&SchematicObjectKey::new("work/amp/schematic", 1).unwrap())
    );
    assert!(
        !resolved
            .overrides
            .contains_key(&SchematicObjectKey::new("work/amp_copy/schematic", 1).unwrap())
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp/schematic", 1)
            .unwrap()
            .is_some()
    );
    assert!(
        catalog
            .annotation()
            .effective_mapping_for("work/amp_copy/schematic", 1)
            .unwrap()
            .is_none()
    );
}
