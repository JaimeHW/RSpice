//! Headless validated-revision cases relocated from application state.
use super::*;
use crate::schematic::{
    component_edit::ComponentPlacement, component_type::ComponentType,
    document_policy::NetNamingPolicy, rotation::Rotation,
};
use rspice_design_model::Point;
use uuid::Uuid;

fn place(state: &mut Schematic, kind: ComponentType, position: Point) {
    state.add_component(
        kind,
        ComponentPlacement {
            position,
            rotation: Rotation::R0,
            mirror_h: false,
        },
        None,
    );
}

fn request(state: &Schematic, project_id: Uuid) -> ValidatedRevisionRequest {
    let design_digest = state
        .validated_design_content_digest()
        .expect("snapshot digest");
    ValidatedRevisionRequest {
        project_id: project_id.to_string(),
        project_revision: 3,
        view_identity: "user/top/schematic".to_owned(),
        revision_note: "Validate compensation network".to_owned(),
        author: "Local project editor".to_owned(),
        validation_receipt_digest: design_digest,
        finding_counts: ValidationFindingCounts {
            blockers: 0,
            advisories: 1,
        },
        dependencies: vec![ValidatedRevisionDependency::new(
            "schematic:user/top/schematic",
            design_digest,
        )],
        advisory_dispositions: vec![AdvisoryDisposition::accepted(
            "drc:unconnected_pin:R1.+",
            "Validated compensation network",
        )],
    }
}

#[test]
fn journal_retains_restorable_hash_linked_revisions() {
    let project_id = Uuid::new_v4();
    let mut state = Schematic::default();
    place(&mut state, ComponentType::Resistor, Point::new(10, 10));
    let first = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("first revision");
    state.document.components[0].value = "2k".to_owned();
    let second = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("second revision");

    state
        .document()
        .validated_revisions
        .validate()
        .expect("valid journal");
    assert_eq!(state.document().validated_revisions.records().len(), 2);
    assert_ne!(first, second);
    assert!(
        state
            .restore_validated_revision(first)
            .expect("restore first")
    );
    assert_eq!(state.document().components[0].value, "1k");
}

#[test]
fn restore_recovers_document_semantics_and_grid() {
    let project_id = Uuid::new_v4();
    let mut state = Schematic::default();
    place(&mut state, ComponentType::Resistor, Point::new(10, 10));
    state.document.document_policy.net_naming = NetNamingPolicy::SpiceCompatibleRelaxed;
    state.document.grid_size = 4;
    let saved = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("validated revision");

    state.document.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
    state.document.grid_size = 10;
    assert!(state.restore_validated_revision(saved).expect("restore"));

    assert_eq!(
        state.document().document_policy.net_naming,
        NetNamingPolicy::SpiceCompatibleRelaxed
    );
    assert_eq!(state.document().grid_size, 4);
}

#[test]
fn semantic_delta_reports_stable_object_changes() {
    let project_id = Uuid::new_v4();
    let mut state = Schematic::default();
    place(&mut state, ComponentType::Resistor, Point::new(10, 10));
    let first = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("first revision");
    state.document.components[0].value = "2k".to_owned();
    place(&mut state, ComponentType::Capacitor, Point::new(20, 10));
    state.document.grid_size = 8;
    let second = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("second revision");
    let first = state
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == first)
        .unwrap();
    let second = state
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == second)
        .unwrap();
    let delta = first.semantic_delta_to(second);
    assert_eq!(delta.components.added, 1);
    assert_eq!(delta.components.modified, 1);
    assert!(delta.grid_changed);
}

#[test]
fn unpublished_removal_is_tail_only() {
    let project_id = Uuid::new_v4();
    let mut state = Schematic::default();
    place(&mut state, ComponentType::Resistor, Point::origin());
    let first = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("first revision");
    let second = state
        .append_validated_revision(request(&state, project_id), || Ok(1))
        .expect("second revision");
    assert_eq!(
        state.remove_unpublished_validated_revision(first),
        Err(ValidatedRevisionError::NotUnpublishedTail)
    );
    state
        .remove_unpublished_validated_revision(second)
        .expect("remove exact tail");
    assert_eq!(state.document().validated_revisions.records().len(), 1);
}

#[test]
fn migrated_baseline_preserves_prior_accepted_design() {
    let project_id = Uuid::new_v4();
    let mut accepted = Schematic::default();
    place(&mut accepted, ComponentType::Capacitor, Point::new(20, 20));
    let mut working = accepted.clone();
    working.document.components[0].value = "2p".to_owned();
    let baseline = working
        .seed_accepted_revision_baseline(
            &accepted,
            &project_id.to_string(),
            7,
            "user/top/schematic",
            || Ok(1),
        )
        .expect("seed baseline")
        .expect("baseline id");
    working
        .append_validated_revision(request(&working, project_id), || Ok(1))
        .expect("validated successor");
    assert!(
        working
            .restore_validated_revision(baseline)
            .expect("restore baseline")
    );
    assert_eq!(working.document().components[0].value, "1u");
}

#[test]
fn blockers_and_undisposed_advisories_fail_closed() {
    let project_id = Uuid::new_v4();
    let mut state = Schematic::default();
    let mut blocked = request(&state, project_id);
    blocked.finding_counts.blockers = 1;
    assert_eq!(
        state.append_validated_revision(blocked, || Ok(1)),
        Err(ValidatedRevisionError::BlockingFindings(1))
    );
    let mut undisposed = request(&state, project_id);
    undisposed.advisory_dispositions.clear();
    assert_eq!(
        state.append_validated_revision(undisposed, || Ok(1)),
        Err(ValidatedRevisionError::AdvisoryDispositionCount {
            expected: 1,
            actual: 0,
        })
    );
}
