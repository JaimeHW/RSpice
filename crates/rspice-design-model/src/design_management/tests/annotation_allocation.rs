//! Allocation respects retained names and the scope of source identity.

use super::*;

fn request(objects: Vec<AnnotationObject>) -> RenumberRequest {
    RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects,
    }
}

#[test]
fn preserved_names_are_reserved_before_any_editable_object_is_allocated() {
    for order in [
        RenumberOrder::HierarchyThenCoordinates,
        RenumberOrder::SheetThenCoordinates,
        RenumberOrder::ConnectivityOrder,
    ] {
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
