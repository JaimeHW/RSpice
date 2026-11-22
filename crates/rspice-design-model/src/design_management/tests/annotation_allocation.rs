//! Allocation respects retained names and the scope of source identity.

use super::*;

fn request(objects: Vec<AnnotationObject>) -> RenumberRequest {
    RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::AcrossThenDown,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects,
    }
}

#[test]
fn preserved_names_are_reserved_before_any_editable_object_is_allocated() {
    for order in [RenumberOrder::AcrossThenDown, RenumberOrder::DownThenAcross] {
        for protection in ["locked", "external", "imported"] {
            let mut state = AnnotationState::default();
            let mut retained = annotation_object(2, "r1", 20);
            retained.locked = protection == "locked";
            retained.external = protection == "external";
            retained.imported = protection == "imported";
            let mut request = request(vec![retained, annotation_object(1, "R10", 10)]);
            request.order = order;
            let before = state.clone();
            let preview = state.preview_renumbering(&request).unwrap();
            assert_eq!(state, before, "preview must not publish");
            assert_eq!(preview.mappings.len(), 1);
            assert_eq!(preview.mappings[&object_key(1)].new_reference, "R2");
            assert!(!preview.mappings.contains_key(&object_key(2)));
            state.commit_renumbering(&preview, &request).unwrap();
            assert_eq!(
                state.effective_mappings()[&object_key(1)].new_reference,
                "R2"
            );
            assert_eq!(
                serde_json::from_str::<AnnotationState>(&serde_json::to_string(&state).unwrap())
                    .unwrap(),
                state
            );
        }
    }
}

#[test]
fn a_preserved_name_exhausts_its_reserved_range_without_publishing() {
    let mut state = AnnotationState::default();
    let policy = AnnotationPolicyDefinition {
        reserved_ranges: vec![AnnotationReservedRange {
            scope: AnnotationRangeScope::Project,
            prefixes: vec!["R".to_owned()],
            first: 1,
            last: 1,
        }],
        ..AnnotationPolicyDefinition::default()
    };
    state
        .update_policy(state.policy().revision(), policy)
        .unwrap();
    let mut retained = annotation_object(2, "R1", 20);
    retained.locked = true;
    let request = request(vec![annotation_object(1, "R10", 10), retained]);
    let before = state.clone();
    assert!(matches!(
        state.preview_renumbering(&request),
        Err(DesignManagementError::AnnotationRangeExhausted(_))
    ));
    assert_eq!(state, before);
}

#[test]
fn reviewed_and_normalized_names_remain_available_for_reallocation() {
    for imported in [false, true] {
        let mut state = AnnotationState::default();
        let mut retained = annotation_object(2, "R1", 20);
        retained.locked = !imported;
        retained.imported = imported;
        let mut request = request(vec![annotation_object(1, "R10", 10), retained]);
        if imported {
            let policy = AnnotationPolicyDefinition {
                imported_ids: ImportedReferencePolicy::NormalizeAfterReview,
                ..AnnotationPolicyDefinition::default()
            };
            state
                .update_policy(state.policy().revision(), policy)
                .unwrap();
        } else {
            request.protected_references = ProtectedReferencePolicy::IncludeAfterReview;
            let before = state.clone();
            assert!(matches!(
                state.preview_renumbering(&request),
                Err(DesignManagementError::ProtectedReferenceReviewRequired(_))
            ));
            assert_eq!(state, before);
            request.protected_reviewed = true;
        }
        let preview = state.preview_renumbering(&request).unwrap();
        assert_eq!(preview.mappings[&object_key(1)].new_reference, "R1");
        assert_eq!(preview.mappings[&object_key(2)].new_reference, "R2");
        state.commit_renumbering(&preview, &request).unwrap();
    }
}

#[test]
fn hierarchy_ranges_include_the_root_and_only_complete_descendant_segments() {
    for (scope, expected) in [
        ("/", ["R100", "R101", "R102", "R103"]),
        ("/X1", ["R1", "R100", "R101", "R2"]),
    ] {
        let mut state = AnnotationState::default();
        let policy = AnnotationPolicyDefinition {
            reserved_ranges: vec![AnnotationReservedRange {
                scope: AnnotationRangeScope::Hierarchy {
                    path: scope.to_owned(),
                },
                prefixes: vec!["R".to_owned()],
                first: 100,
                last: 103,
            }],
            ..AnnotationPolicyDefinition::default()
        };
        state
            .update_policy(state.policy().revision(), policy)
            .unwrap();
        let objects = ["/", "/X1", "/X1/X2", "/X10"]
            .into_iter()
            .enumerate()
            .map(|(index, path)| {
                let mut object =
                    annotation_object(index as u64 + 1, &format!("R{}", index + 10), 0);
                object.hierarchy_path = path.to_owned();
                object
            })
            .collect();
        let request = request(objects);
        let preview = state.preview_renumbering(&request).unwrap();
        for (index, reference) in expected.into_iter().enumerate() {
            assert_eq!(
                preview.mappings[&object_key(index as u64 + 1)].new_reference,
                reference
            );
        }
        state.commit_renumbering(&preview, &request).unwrap();
    }
}

#[test]
fn incoming_reference_uniqueness_is_scoped_to_the_owning_cell_view() {
    for owner in ["work/child/schematic", "work/top/extracted"] {
        let mut state = AnnotationState::default();
        let root = annotation_object(1, "R1", 10);
        let mut child = annotation_object(1, "r1", 20);
        child.object = SchematicObjectKey::new(owner, 1).unwrap();
        child.hierarchy_path = "/top/X1".to_owned();
        let request = request(vec![root, child]);
        let preview = state.preview_renumbering(&request).unwrap();
        assert_eq!(preview.mappings[&object_key(1)].new_reference, "R1");
        assert_eq!(
            preview.mappings[&request.objects[1].object].new_reference,
            "R2"
        );
        state.commit_renumbering(&preview, &request).unwrap();
        state.validate().unwrap();
    }
    let state = AnnotationState::default();
    let mut duplicate = annotation_object(2, "r1", 20);
    duplicate.object = SchematicObjectKey::new("WORK/TOP/SCHEMATIC", 2).unwrap();
    assert!(matches!(
        state.preview_renumbering(&request(vec![annotation_object(1, "R1", 10), duplicate])),
        Err(DesignManagementError::DuplicateReferenceDesignator(_))
    ));
}

#[test]
fn a_nested_hierarchy_range_precedes_ancestor_ranges() {
    let mut state = AnnotationState::default();
    let policy = AnnotationPolicyDefinition {
        reserved_ranges: [("/", 10), ("/X1", 100), ("/X1/X2", 200)]
            .into_iter()
            .map(|(path, first)| AnnotationReservedRange {
                scope: AnnotationRangeScope::Hierarchy {
                    path: path.to_owned(),
                },
                prefixes: vec!["R".to_owned()],
                first,
                last: first + 9,
            })
            .collect(),
        ..AnnotationPolicyDefinition::default()
    };
    state
        .update_policy(state.policy().revision(), policy)
        .unwrap();
    let objects = [("/X1", "R30"), ("/X1/X2", "R40"), ("/X1/X20", "R50")]
        .into_iter()
        .enumerate()
        .map(|(index, (path, name))| {
            let mut object = annotation_object(index as u64 + 1, name, 0);
            object.hierarchy_path = path.to_owned();
            object
        })
        .collect();
    let request = request(objects);
    let preview = state.preview_renumbering(&request).unwrap();
    for (id, reference) in [(1, "R100"), (2, "R200"), (3, "R101")] {
        assert_eq!(preview.mappings[&object_key(id)].new_reference, reference);
    }
    state.commit_renumbering(&preview, &request).unwrap();
}

/// The new name each part gets under `order`, in the order the parts were
/// listed.
fn renumbered(objects: Vec<AnnotationObject>, order: RenumberOrder) -> Vec<String> {
    let keys = objects
        .iter()
        .map(|object| object.object.clone())
        .collect::<Vec<_>>();
    let mut request = request(objects);
    request.order = order;
    let preview = AnnotationState::default()
        .preview_renumbering(&request)
        .unwrap();
    keys.iter()
        .map(|key| preview.mappings[key].new_reference.clone())
        .collect()
}

fn placed(id: u64, page: u32, zone: Option<(u32, u32)>, x: i64, y: i64) -> AnnotationObject {
    let mut object = annotation_object(id, &format!("R{}", 100 + id), x);
    object.position = AnnotationPosition { page, zone, x, y };
    object
}

/// A part a little higher at the far right of a zone row is numbered after
/// the parts to its left in that row, not before them: across then down
/// reads the sheet's zone rows, down then across its zone columns.
#[test]
fn the_two_orders_read_a_zoned_sheet_by_its_rows_or_its_columns() {
    let parts = || {
        vec![
            // Zone row 0: low at the left, a step higher at the far right.
            placed(1, 0, Some((0, 0)), 10, 40),
            placed(2, 0, Some((0, 2)), 400, 30),
            // Zone row 1, columns 0 and 1.
            placed(3, 0, Some((1, 0)), 10, 240),
            placed(4, 0, Some((1, 1)), 200, 230),
        ]
    };
    assert_eq!(
        renumbered(parts(), RenumberOrder::AcrossThenDown),
        ["R1", "R2", "R3", "R4"]
    );
    assert_eq!(
        renumbered(parts(), RenumberOrder::DownThenAcross),
        ["R1", "R4", "R2", "R3"]
    );
}

/// Without zones the coordinate itself is the band: across then down is
/// top to bottom, then left to right; down then across the other way.
#[test]
fn a_sheet_without_zones_is_ordered_by_coordinate() {
    let parts = || {
        vec![
            placed(1, 0, None, 10, 40),
            placed(2, 0, None, 400, 30),
            placed(3, 0, None, 200, 40),
        ]
    };
    assert_eq!(
        renumbered(parts(), RenumberOrder::AcrossThenDown),
        ["R2", "R1", "R3"]
    );
    assert_eq!(
        renumbered(parts(), RenumberOrder::DownThenAcross),
        ["R1", "R3", "R2"]
    );
}

/// Sheets are numbered in page order, whatever their identities sort as:
/// the top of the second page comes after the bottom of the first.
#[test]
fn sheets_are_numbered_in_page_order() {
    let [first, second] = [SheetId::new(), SheetId::new()];
    let on = |id, sheet, page, y| {
        let mut object = placed(id, page, None, 10, y);
        object.sheet_id = Some(sheet);
        object
    };
    for order in [RenumberOrder::AcrossThenDown, RenumberOrder::DownThenAcross] {
        assert_eq!(
            renumbered(
                vec![
                    on(1, second, 1, 0),
                    on(2, first, 0, 900),
                    on(3, first, 0, 10)
                ],
                order
            ),
            ["R3", "R2", "R1"]
        );
    }
}
