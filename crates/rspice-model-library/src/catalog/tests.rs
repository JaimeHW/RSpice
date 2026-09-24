//! Headless catalog publication and snapshot atomicity.

use super::*;
use crate::qualification::ModelQualificationState;

fn definition(value: f64) -> ProjectModelRevisionDefinition {
    let base = ProjectModelDefinition {
        name: "nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: String::new(),
        numeric_parameters: BTreeMap::from([("vth0".to_owned(), value)]),
        string_parameters: BTreeMap::new(),
    };
    let metadata = base.reconcile_metadata(None).unwrap();
    ProjectModelRevisionDefinition::new(base, metadata)
}

#[test]
fn guarded_revision_publishes_only_after_complete_validation() {
    let mut catalog = ModelCatalog::default();
    let created = catalog
        .create_project_model_revision(
            "owned",
            &definition(0.4),
            &ModelQualificationState::default(),
        )
        .unwrap();
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest,
    } = created.after.source_authority
    else {
        panic!("project-owned fixture");
    };
    let target = ProjectModelTarget {
        library_name: "owned",
        source_id,
        library_revision: revision,
        model_revision: revision,
        model_name: "nch",
        model_digest: digest,
    };
    let before = serde_json::to_value(&catalog).unwrap();
    for stale in [
        ProjectModelTarget {
            source_id: ModelSourceId::new(),
            ..target
        },
        ProjectModelTarget {
            model_digest: ContentDigest::from_bytes([0; 32]),
            ..target
        },
    ] {
        assert!(
            catalog
                .replace_project_model_revision_in_library(
                    stale,
                    &definition(0.5),
                    &ModelQualificationState::default(),
                )
                .unwrap_err()
                .contains("changed after")
        );
        assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    }

    let mut invalid = definition(0.5);
    invalid
        .base
        .numeric_parameters
        .insert("vth0".to_owned(), f64::NAN);
    assert!(
        catalog
            .replace_project_model_revision_in_library(
                target,
                &invalid,
                &ModelQualificationState::default(),
            )
            .unwrap_err()
            .contains("must be finite")
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);

    let commit = catalog
        .replace_project_model_revision_in_library(
            target,
            &definition(0.5),
            &ModelQualificationState::default(),
        )
        .unwrap();
    assert_eq!(commit.after.models["nch"].parameters["vth0"], 0.5);
    assert_eq!(
        commit.after.project_source_revision(),
        Some(revision.next().unwrap())
    );
    assert!(commit.affects_execution);
    assert_eq!(
        serde_json::to_value(commit.before.unwrap()).unwrap(),
        before["owned"]
    );
}

#[test]
fn snapshot_replacement_preserves_retained_state_and_rejects_duplicates_atomically() {
    let mut catalog = ModelCatalog::default();
    let mut original = ModelLibrary::new("owned");
    original.expanded = true;
    catalog.add_library(original);
    let before = serde_json::to_value(&catalog).unwrap();
    assert!(
        catalog
            .replace_library_snapshot(vec![ModelLibrary::new("next"), ModelLibrary::new("next")])
            .is_err()
    );
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
    catalog
        .replace_library_snapshot(vec![ModelLibrary::new("owned")])
        .unwrap();
    assert!(catalog.get_library("owned").unwrap().expanded);
    assert_eq!(serde_json::to_value(&catalog).unwrap(), before);
}
