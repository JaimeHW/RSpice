use super::*;
use crate::plan_model::AnalysisLifecycleState;
use crate::run_set::RunSetDimensionKind;
use rspice_app_types::product::AnalysisInstanceId;

fn new_setup() -> SimulationSetupDocument {
    SimulationSetupDocument {
        analysis_plan: Some(SimulationPlan::new()),
        run_set: crate::legacy_plan_migration::default_global_run_set(),
        ..SimulationSetupDocument::default()
    }
}

fn clone_instance_id(setup: &SimulationSetupDocument) -> AnalysisInstanceId {
    setup.analysis_plan.as_ref().unwrap().instances()[0].id()
}

#[test]
fn plan_names_are_trimmed_validated_and_case_insensitively_unique() {
    let name = SimulationPlanName::new("  Post-layout sweep  ").expect("name is valid");
    assert_eq!(name.as_str(), "Post-layout sweep");
    assert!(SimulationPlanName::new("\n").is_err());
    assert!(SimulationPlanName::new("a".repeat(97)).is_err());

    let mut setup = new_setup();
    let error = setup
        .clone_active_plan(
            "LAB CHARACTERIZATION",
            SimulationPlanCloneOptions::default(),
        )
        .expect_err("names are unique without ASCII-case ambiguity");
    assert!(matches!(
        error,
        SimulationPlanCatalogError::DuplicateName(_)
    ));
}

#[test]
fn cloning_activates_a_fresh_plan_and_retains_an_independent_source() {
    let mut setup = new_setup();
    let source_id = setup.analysis_plan.as_ref().unwrap().id();
    let source_instance_id = setup.analysis_plan.as_ref().unwrap().instances()[0].id();
    setup.options.reltol = 2.5e-5;
    setup.save_policy.retained_dataset_limit = 7;
    setup
        .run_set
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
        .unwrap()
        .source = "netlist-source:VDD".to_owned();
    setup
        .model_bindings
        .push(rspice_model_library::SimulationPlanModelBinding {
            library_name: "foundry-models".to_owned(),
            source_digest: rspice_app_types::product::ContentDigest::from_bytes([7; 32]),
            selected_corner: Some("TT".to_owned()),
        });
    setup
        .analysis_plan
        .as_mut()
        .unwrap()
        .edit(source_instance_id, |draft| {
            let crate::analysis_draft::AnalysisDraft::Transient(draft) = draft else {
                panic!("expected transient");
            };
            draft.stop = "42u".to_owned();
        })
        .unwrap();

    let clone = setup
        .clone_active_plan(
            "Lab characterization · variant",
            SimulationPlanCloneOptions::default(),
        )
        .expect("clone commits");
    let clone_id = clone.cloned_plan_id;

    assert_eq!(setup.plan_count(), 2);
    assert_eq!(
        setup.active_plan_name().as_str(),
        "Lab characterization · variant"
    );
    assert_eq!(setup.analysis_plan.as_ref().unwrap().id(), clone_id);
    assert_ne!(clone_id, source_id);
    assert_eq!(setup.options.reltol, 2.5e-5);
    assert_eq!(setup.save_policy.retained_dataset_limit, 7);
    assert_eq!(clone.source_plan_id, source_id);
    assert_eq!(clone.contents, SimulationPlanCloneOptions::default());
    assert_eq!(
        clone.analysis_identity_map,
        vec![(source_instance_id, clone_instance_id(&setup))]
    );
    assert_eq!(
        setup.active_plan_lineage(),
        SimulationPlanLineage::cloned_from_with_contents(
            source_id,
            setup.inactive_plans()[0].revision(),
            SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
        )
    );
    let clone_instance = &setup.analysis_plan.as_ref().unwrap().instances()[0];
    assert_ne!(clone_instance.id(), source_instance_id);
    assert_eq!(clone_instance.lifecycle(), AnalysisLifecycleState::Draft);
    assert!(setup.analysis_plan.as_ref().unwrap().receipts().is_empty());
    assert!(
        setup
            .analysis_plan
            .as_ref()
            .unwrap()
            .tombstones()
            .is_empty()
    );
    assert_eq!(setup.inactive_plans()[0].id(), source_id);
    assert_eq!(
        setup.inactive_plans()[0]
            .run_set()
            .dimensions
            .iter()
            .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
            .unwrap()
            .source,
        "netlist-source:VDD"
    );
    assert_eq!(
        setup.inactive_plans()[0].model_bindings()[0]
            .selected_corner
            .as_deref(),
        Some("TT")
    );
    setup
        .run_set
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
        .unwrap()
        .source = "netlist-source:VCORE".to_owned();
    setup.model_bindings[0].selected_corner = Some("FF".to_owned());
    setup.save_policy.retained_dataset_limit = 3;

    setup
        .activate_plan(source_id)
        .expect("source can be reactivated");
    assert_eq!(
        setup.active_plan_name().as_str(),
        SimulationPlanName::default().as_str()
    );
    assert_eq!(setup.analysis_plan.as_ref().unwrap().id(), source_id);
    assert_eq!(
        setup.analysis_plan.as_ref().unwrap().instances()[0].id(),
        source_instance_id
    );
    assert_eq!(setup.inactive_plans()[0].id(), clone_id);
    assert_eq!(
        setup
            .run_set
            .dimensions
            .iter()
            .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
            .unwrap()
            .source,
        "netlist-source:VDD",
        "switching restores the source plan's independent run-set document"
    );
    assert_eq!(
        setup.model_bindings[0].selected_corner.as_deref(),
        Some("TT"),
        "switching restores the source plan's model section"
    );
    assert_eq!(setup.save_policy.retained_dataset_limit, 7);
    assert_eq!(
        setup.inactive_plans()[0]
            .run_set()
            .dimensions
            .iter()
            .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
            .unwrap()
            .source,
        "netlist-source:VCORE",
        "the clone retains its independently edited run set"
    );
    assert_eq!(
        setup.inactive_plans()[0].model_bindings()[0]
            .selected_corner
            .as_deref(),
        Some("FF"),
        "the clone retains its independently edited model section"
    );
    assert_eq!(
        setup.inactive_plans()[0]
            .save_policy()
            .retained_dataset_limit,
        3
    );
}

#[test]
fn create_rename_archive_restore_and_activate_preserve_identity() {
    let mut setup = new_setup();
    let original_id = setup.analysis_plan.as_ref().unwrap().id();
    let created_id = setup
        .create_plan("Fresh characterization")
        .expect("fresh plan commits");
    assert_ne!(created_id, original_id);
    assert_eq!(setup.active_plan_name().as_str(), "Fresh characterization");
    assert_eq!(
        setup.active_plan_lineage(),
        SimulationPlanLineage::default()
    );
    assert!(setup.model_bindings.is_empty());

    setup
        .rename_plan(original_id, "Archived source")
        .expect("inactive rename preserves identity");
    assert_eq!(setup.inactive_plans()[0].name().as_str(), "Archived source");
    assert_eq!(setup.inactive_plans()[0].id(), original_id);
    setup
        .archive_plan(original_id)
        .expect("inactive plan archives");
    assert!(setup.inactive_plans()[0].archived());
    assert!(matches!(
        setup.activate_plan(original_id),
        Err(SimulationPlanCatalogError::PlanArchived(id)) if id == original_id
    ));
    setup
        .restore_plan(original_id)
        .expect("archive is recoverable");
    setup
        .activate_plan(original_id)
        .expect("restored plan activates");
    assert_eq!(setup.analysis_plan.as_ref().unwrap().id(), original_id);
    assert_eq!(setup.active_plan_name().as_str(), "Archived source");
    assert!(matches!(
        setup.archive_plan(original_id),
        Err(SimulationPlanCatalogError::ActivePlanCannotBeArchived(id)) if id == original_id
    ));
}
