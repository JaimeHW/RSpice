//! Manual assignment receipts, legacy digest compatibility and ownership refusal.

use super::*;

fn annotated() -> (AnnotationState, RenumberRequest) {
    let mut state = AnnotationState::default();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![annotation_object(1, "R10", 10)],
    };
    let preview = state.preview_renumbering(&request).unwrap();
    state.commit_renumbering(&preview, &request).unwrap();
    (state, request)
}

#[test]
fn reference_projection_accepts_recorded_lineage_and_refuses_unrelated_names() {
    let (mut state, _) = annotated();
    state
        .commit_manual_reference_edit(object_key(1), "R1", "R20")
        .unwrap()
        .unwrap();
    let before = state.clone();
    for source in ["R10", "r1"] {
        assert_eq!(
            state
                .projected_reference_assignments([(object_key(1), source)])
                .unwrap()[&object_key(1)],
            "R20"
        );
    }
    assert!(
        state
            .projected_reference_assignments([(object_key(1), "R20"), (object_key(2), "R99")])
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        state.projected_reference_assignments([(object_key(1), "R99")]),
        Err(DesignManagementError::StaleAnnotationReference { .. })
    ));
    assert!(matches!(
        state.projected_reference_assignments([(object_key(1), "R10"), (object_key(1), "R10")]),
        Err(DesignManagementError::DuplicateScopedSchematicObject(_))
    ));
    assert_eq!(state, before);
    state
        .remap_object_owners("work", "top", "work", "renamed")
        .unwrap();
    let target = SchematicObjectKey::new("work/renamed/schematic", 1).unwrap();
    assert_eq!(
        state
            .projected_reference_assignments([(target.clone(), "R10")])
            .unwrap()[&target],
        "R20"
    );
    state.tombstone_objects(|object| object == &target).unwrap();
    assert!(
        state
            .projected_reference_assignments([(target, "R10")])
            .unwrap()
            .is_empty()
    );
}

#[test]
fn manual_assignment_retains_legacy_evidence_and_allows_future_annotation() {
    let (mut state, mut request) = annotated();
    let retained = state.journal()[0].clone();
    let legacy = serde_json::to_value(&state).unwrap();
    assert!(legacy["journal"][0].get("origin").is_none());
    assert_eq!(
        serde_json::from_value::<AnnotationState>(legacy).unwrap(),
        state
    );

    state
        .commit_manual_reference_edit(object_key(1), "R1", "R10")
        .unwrap()
        .unwrap();
    assert_eq!(state.journal()[0], retained);
    assert_eq!(
        state.journal()[1].origin(),
        AnnotationJournalOrigin::ManualEdit
    );
    assert_eq!(
        state.effective_mappings()[&object_key(1)].new_reference,
        "R10"
    );
    let wire = serde_json::to_value(&state).unwrap();
    assert_eq!(wire["journal"][1]["origin"], "manual-edit");
    assert_eq!(
        serde_json::from_value::<AnnotationState>(wire.clone()).unwrap(),
        state
    );
    let mut tampered = wire;
    tampered["journal"][1]["origin"] = serde_json::json!("reviewed-renumbering");
    assert!(serde_json::from_value::<AnnotationState>(tampered).is_err());

    request.objects[0].current_reference = "R10".to_owned();
    let preview = state.preview_renumbering(&request).unwrap();
    state.commit_renumbering(&preview, &request).unwrap();
    assert_eq!(
        state.effective_mappings()[&object_key(1)].new_reference,
        "R1"
    );
    assert_eq!(state.journal()[0], retained);
    assert_eq!(
        state.journal()[2].origin(),
        AnnotationJournalOrigin::ReviewedRenumbering
    );
}

#[test]
fn manual_assignment_refuses_stale_or_inactive_authority_without_mutation() {
    let (mut state, _) = annotated();
    let before = state.clone();
    assert!(matches!(
        state.commit_manual_reference_edit(object_key(1), "R10", "R7"),
        Err(DesignManagementError::StaleAnnotationReference { .. })
    ));
    assert!(
        state
            .commit_manual_reference_edit(object_key(1), "R1", "bad name")
            .is_err()
    );
    assert_eq!(state, before);
    assert_eq!(
        state
            .commit_manual_reference_edit(object_key(1), "R1", "R1")
            .unwrap(),
        None
    );
    assert_eq!(
        state
            .commit_manual_reference_edit(object_key(2), "legacy name", "R3")
            .unwrap(),
        None
    );
    assert_eq!(
        state
            .commit_manual_reference_edit(object_key(2), "R2", "R3")
            .unwrap(),
        None
    );
    assert_eq!(state, before);
    state
        .object_authorities
        .insert(object_key(1), AnnotationObjectAuthority::Tombstone);
    let before = state.clone();
    assert!(matches!(
        state.commit_manual_reference_edit(object_key(1), "R1", "R9"),
        Err(DesignManagementError::InactiveAnnotationObjectAuthority(_))
    ));
    assert_eq!(state, before);
}

#[test]
fn manual_assignment_follows_relocated_object_authority() {
    let (mut state, _) = annotated();
    let destination = SchematicObjectKey::new("work/renamed/schematic", 1).unwrap();
    state.object_authorities.insert(
        object_key(1),
        AnnotationObjectAuthority::Redirect {
            target: destination.clone(),
        },
    );
    let retained = state.journal()[0].clone();
    state
        .commit_manual_reference_edit(destination.clone(), "R1", "R7")
        .unwrap();
    let effective = state.effective_mappings();
    assert_eq!(effective.len(), 1);
    assert_eq!(effective[&destination].new_reference, "R7");
    assert_eq!(state.journal()[0], retained);
    assert!(
        state
            .commit_manual_reference_edit(object_key(1), "R7", "R8")
            .is_err()
    );
}

#[test]
fn old_journal_digest_material_is_unchanged() {
    let (state, _) = annotated();
    let entry = &state.journal()[0];
    #[derive(Serialize)]
    struct LegacyMaterial<'a> {
        id: AnnotationJournalId,
        sequence: u64,
        policy_revision: u64,
        policy_digest: ContentDigest,
        request_digest: ContentDigest,
        mappings: &'a BTreeMap<SchematicObjectKey, AnnotationMapping>,
    }
    let encoded = serde_json::to_value(entry).unwrap();
    let legacy = LegacyMaterial {
        id: entry.id(),
        sequence: entry.sequence(),
        policy_revision: entry.policy_revision(),
        policy_digest: entry.policy_digest(),
        request_digest: serde_json::from_value(encoded["request_digest"].clone()).unwrap(),
        mappings: entry.mappings(),
    };
    assert_eq!(
        digest("rspice-annotation-journal-entry-semantic/v1", &legacy).unwrap(),
        entry.semantic_digest()
    );
}
