//! Tests for plan structure and revision handling.
//!
//! Almost every case is fail-closed: a corrupt dependency cycle is rejected, an
//! exhausted revision fails without mutating, and the frozen projection is
//! deterministic and excludes disabled positions.

use super::*;

fn snapshot(plan: &SimulationPlan) -> String {
    serde_json::to_string(plan).expect("plan serializes")
}

fn exact_periodic_context() -> AnalysisDependencyRepairContext {
    AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nVLO lo 0 SIN(0 1 1k)\nVRF rf 0 SIN(0 1 2k)\nR1 lo 0 1k\nR2 rf 0 1k\n.end\n",
        )
        .expect("test periodic-source fixture is exact")
}

fn configure_high_frequency_fourier(plan: &mut SimulationPlan, id: AnalysisInstanceId) {
    plan.edit(id, |draft| {
        let AnalysisDraft::Fourier(draft) = draft else {
            panic!("expected Fourier draft");
        };
        draft.fundamental = "100Meg".to_owned();
        draft.harmonics = "10".to_owned();
        draft.start_time = "0".to_owned();
        draft.stop_time = "100n".to_owned();
    })
    .expect("Fourier draft edits");
}

fn configure_fine_transient(plan: &mut SimulationPlan, id: AnalysisInstanceId) {
    plan.edit(id, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected Transient draft");
        };
        draft.start = "0".to_owned();
        draft.stop = "1u".to_owned();
        draft.step = "100p".to_owned();
        draft.max_step = "100p".to_owned();
    })
    .expect("Transient draft edits");
}

#[test]
fn envelope_initializer_remains_internal_and_dependency_free() {
    let mut plan = SimulationPlan::empty();
    let (envelope, _) = plan
        .insert(AnalysisKind::Envelope)
        .expect("Envelope inserts");

    for selection in 0..=2 {
        plan.edit(envelope, |draft| {
            let AnalysisDraft::Envelope(draft) = draft else {
                panic!("expected Envelope draft");
            };
            draft.initial_periodic_solve_idx = selection;
        })
        .expect("Envelope initializer edits");
        assert!(plan.instance(envelope).unwrap().dependencies().is_empty());
        assert!(plan.validation_issues().is_empty());
    }
}

#[test]
fn restore_prunes_the_retired_external_envelope_initializer_role() {
    let mut plan = SimulationPlan::empty();
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (hb, _) = plan
        .insert(AnalysisKind::HarmonicBalance)
        .expect("HB inserts");
    plan.bind_dependency(hb, AnalysisKind::OperatingPoint, op)
        .expect("HB binds OP");
    let (envelope, _) = plan
        .insert(AnalysisKind::Envelope)
        .expect("Envelope inserts");
    let envelope_index = plan.index_of(envelope).expect("Envelope index");
    plan.instances[envelope_index]
        .dependencies
        .push(AnalysisDependency::new(AnalysisKind::HarmonicBalance, hb));

    plan.prepare_after_restore();

    assert!(plan.instance(envelope).unwrap().dependencies().is_empty());
    assert!(plan.validation_issues().is_empty());
}

#[test]
fn fourier_repair_skips_a_coarse_transient_and_inserts_a_compatible_one() {
    let mut plan = SimulationPlan::new();
    let original = plan.instances()[0].id();
    let original_draft = plan.instance(original).unwrap().draft().clone();
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut plan, fourier);

    let (repair, _) = plan
        .repair_dependencies(fourier)
        .expect("repair synthesizes an adequate transient");
    assert_eq!(repair.inserted().len(), 1);
    let target = plan.instance(fourier).unwrap().dependencies()[0].target();
    assert_ne!(target, original);
    assert!(plan.dependency_candidate_is_compatible(fourier, AnalysisKind::Transient, target));
    assert_eq!(
        serde_json::to_string(plan.instance(original).unwrap().draft()).unwrap(),
        serde_json::to_string(&original_draft).unwrap(),
        "repair must not mutate an unrelated coarse transient"
    );
    assert!(plan.validation_issues().is_empty());
}

#[test]
fn fourier_repair_reuses_an_earlier_compatible_transient() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    configure_fine_transient(&mut plan, transient);
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut plan, fourier);

    let (repair, _) = plan
        .repair_dependencies(fourier)
        .expect("repair reuses the compatible transient");
    assert!(repair.inserted().is_empty());
    assert_eq!(
        plan.instance(fourier).unwrap().dependencies()[0].target(),
        transient
    );
}

#[test]
fn fourier_repair_skips_an_invalid_transient_but_invalid_fourier_is_atomic() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    plan.edit(transient, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected Transient draft");
        };
        draft.step = "unfinished(".to_owned();
    })
    .expect("in-progress transient edit is retained");
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut plan, fourier);
    let (repair, _) = plan
        .repair_dependencies(fourier)
        .expect("invalid producer is replaced, not bound");
    assert_eq!(repair.inserted().len(), 1);
    assert_ne!(
        plan.instance(fourier).unwrap().dependencies()[0].target(),
        transient
    );

    let mut invalid_consumer = SimulationPlan::new();
    let (fourier, _) = invalid_consumer
        .insert(AnalysisKind::Fourier)
        .expect("Fourier inserts");
    invalid_consumer
        .edit(fourier, |draft| {
            let AnalysisDraft::Fourier(draft) = draft else {
                panic!("expected Fourier draft");
            };
            draft.fundamental = "unfinished(".to_owned();
        })
        .expect("in-progress Fourier edit is retained");
    let before = snapshot(&invalid_consumer);
    assert!(matches!(
        invalid_consumer.repair_dependencies(fourier),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { .. })
    ));
    assert_eq!(snapshot(&invalid_consumer), before);
}

#[test]
fn incompatible_fourier_binding_is_editable_but_cannot_freeze_or_auto_bind() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    configure_fine_transient(&mut plan, transient);
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut plan, fourier);
    plan.bind_dependency(fourier, AnalysisKind::Transient, transient)
        .expect("compatible dependency binds");

    plan.edit(transient, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected Transient draft");
        };
        draft.step = "10n".to_owned();
        draft.max_step = "10n".to_owned();
    })
    .expect("draft remains editable while temporarily incompatible");
    assert!(plan.validation_issues().iter().any(|issue| matches!(
        issue,
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            target,
            ..
        } if *dependent == fourier && *target == transient
    )));
    plan.validate_structure()
        .expect("configuration incompatibility is non-structural while editing");
    assert!(matches!(
        plan.freeze(),
        Err(AnalysisPlanError::InvalidPlan(_))
    ));

    let mut auto = SimulationPlan::new();
    let coarse = auto.instances()[0].id();
    let (fourier, _) = auto.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut auto, fourier);
    auto.auto_bind_dependencies(fourier)
        .expect("auto-bind completes without a bad edge");
    assert!(auto.instance(fourier).unwrap().dependencies().is_empty());
    assert!(matches!(
        auto.bind_dependency(fourier, AnalysisKind::Transient, coarse),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { .. })
    ));
}

#[test]
fn invalid_bound_transient_is_reported_and_atomically_replaced_by_repair() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    configure_fine_transient(&mut plan, transient);
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    configure_high_frequency_fourier(&mut plan, fourier);
    plan.bind_dependency(fourier, AnalysisKind::Transient, transient)
        .expect("compatible dependency binds");

    plan.edit(transient, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected Transient draft");
        };
        draft.step = "unfinished(".to_owned();
    })
    .expect("in-progress producer edit is retained");

    assert!(plan.validation_issues().iter().any(|issue| matches!(
        issue,
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            target,
            detail,
            ..
        } if *dependent == fourier
            && *target == transient
            && detail.contains("Transient configuration is invalid")
    )));
    let (repair, _) = plan
        .repair_dependencies(fourier)
        .expect("invalid producer is replaced");
    assert_eq!(repair.inserted().len(), 1);
    assert_ne!(
        plan.instance(fourier).unwrap().dependencies()[0].target(),
        transient
    );
    assert!(plan.validation_issues().is_empty());
}

#[test]
fn lifecycle_vocabulary_matches_the_normative_mockup_ids() {
    let states = [
        (AnalysisLifecycleState::Absent, "absent"),
        (AnalysisLifecycleState::Draft, "draft"),
        (AnalysisLifecycleState::Invalid, "invalid"),
        (AnalysisLifecycleState::Ready, "ready"),
        (AnalysisLifecycleState::PreflightReady, "preflight-ready"),
        (AnalysisLifecycleState::Blocked, "blocked"),
        (AnalysisLifecycleState::Queued, "queued"),
        (AnalysisLifecycleState::Running, "running"),
        (AnalysisLifecycleState::Paused, "paused"),
        (AnalysisLifecycleState::Completed, "completed"),
        (AnalysisLifecycleState::Failed, "failed"),
        (AnalysisLifecycleState::Cancelled, "cancelled"),
        (AnalysisLifecycleState::Disabled, "disabled"),
        (AnalysisLifecycleState::Removed, "removed"),
        (AnalysisLifecycleState::SameState, "same-state"),
    ];
    for (state, stable_id) in states {
        assert_eq!(
            serde_json::to_string(&state).expect("state serializes"),
            format!("\"{stable_id}\"")
        );
        assert_eq!(state.to_string(), stable_id);
    }

    let commands = [
        (AnalysisLifecycleCommand::Insert, "insert"),
        (AnalysisLifecycleCommand::Edit, "edit"),
        (AnalysisLifecycleCommand::Clone, "clone"),
        (AnalysisLifecycleCommand::Enable, "enable"),
        (AnalysisLifecycleCommand::Disable, "disable"),
        (AnalysisLifecycleCommand::Reorder, "reorder"),
        (AnalysisLifecycleCommand::Dependency, "dependency"),
        (AnalysisLifecycleCommand::Validate, "validate"),
        (AnalysisLifecycleCommand::Preflight, "preflight"),
        (AnalysisLifecycleCommand::Execute, "execute"),
        (AnalysisLifecycleCommand::Remove, "remove"),
    ];
    for (command, stable_id) in commands {
        assert_eq!(
            serde_json::to_string(&command).expect("command serializes"),
            format!("\"{stable_id}\"")
        );
        assert_eq!(command.to_string(), stable_id);
    }
}

#[test]
fn plan_diagnostics_render_as_concise_product_language() {
    let dependent = AnalysisInstanceId::new();
    let issue = AnalysisPlanIssue::MissingPrerequisite {
        dependent,
        prerequisite: AnalysisKind::OperatingPoint,
    };
    let text = issue.to_string();
    assert!(text.contains(&dependent.to_string()));
    assert!(text.contains("op prerequisite"));
    assert!(!text.contains("MissingPrerequisite"));

    let error = AnalysisPlanError::InvalidPlan(vec![issue, AnalysisPlanIssue::NoEnabledInstances]);
    let text = error.to_string();
    assert!(text.contains("Review 1 additional plan issue."));
    assert!(!text.contains('['));
    assert!(!text.contains("MissingPrerequisite"));
}

#[test]
fn fresh_plan_has_one_enabled_transient() {
    let plan = SimulationPlan::new();
    assert_eq!(plan.revision(), ObjectRevision::INITIAL);
    assert_eq!(plan.instances().len(), 1);
    let instance = &plan.instances()[0];
    assert_eq!(instance.kind(), AnalysisKind::Transient);
    assert!(instance.enabled());
    assert!(plan.validation_issues().is_empty());
}

#[test]
fn empty_plan_is_editable_but_not_dispatchable() {
    let plan = SimulationPlan::empty();
    assert_eq!(plan.revision(), ObjectRevision::INITIAL);
    assert!(plan.instances().is_empty());
    assert!(plan.tombstones().is_empty());
    assert!(plan.receipts().is_empty());
    plan.validate_structure()
        .expect("an empty working plan is structurally valid");
    assert!(matches!(
        plan.freeze(),
        Err(AnalysisPlanError::InvalidPlan(issues))
            if issues == vec![AnalysisPlanIssue::NoEnabledInstances]
    ));
}

#[test]
fn plan_clone_refreshes_all_identities_and_remaps_the_dependency_graph() {
    let mut source = SimulationPlan::new();
    let transient = source.instances()[0].id();
    source
        .edit(transient, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                panic!("expected transient draft");
            };
            draft.stop = "19u".to_owned();
        })
        .expect("source draft edits");
    let (op, _) = source
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (ac, _) = source.insert(AnalysisKind::Ac).expect("AC inserts");
    source
        .bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .expect("dependency binds");
    let (retired, _) = source
        .insert(AnalysisKind::DcSweep)
        .expect("disposable instance inserts");
    source
        .remove(retired, vec![RunId::new()])
        .expect("disposable instance retires");
    source
        .commit_configuration_change("Design variables were updated.")
        .expect("source configuration receipt commits");

    let clone = source.clone_as_new().expect("valid source plan clones");

    assert_ne!(clone.id(), source.id());
    assert_eq!(clone.revision(), ObjectRevision::INITIAL);
    assert!(clone.tombstones().is_empty());
    assert!(clone.receipts().is_empty());
    assert!(clone.configuration_receipts().is_empty());
    assert_eq!(clone.instances().len(), source.instances().len());

    let source_ids = source
        .instances()
        .iter()
        .map(AnalysisInstance::id)
        .collect::<HashSet<_>>();
    assert!(
        clone
            .instances()
            .iter()
            .all(|instance| !source_ids.contains(&instance.id()))
    );

    for (source_instance, cloned_instance) in source.instances().iter().zip(clone.instances()) {
        assert_eq!(cloned_instance.kind(), source_instance.kind());
        assert_eq!(cloned_instance.enabled(), source_instance.enabled());
        assert_eq!(
            serde_json::to_value(cloned_instance.draft()).unwrap(),
            serde_json::to_value(source_instance.draft()).unwrap()
        );
        assert_eq!(cloned_instance.created_revision(), ObjectRevision::INITIAL);
        assert_eq!(cloned_instance.modified_revision(), ObjectRevision::INITIAL);
        assert_eq!(
            cloned_instance.lifecycle(),
            if cloned_instance.enabled() {
                AnalysisLifecycleState::Draft
            } else {
                AnalysisLifecycleState::Disabled
            }
        );
    }

    let cloned_op = clone
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::OperatingPoint)
        .expect("cloned OP exists")
        .id();
    let cloned_ac = clone
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Ac)
        .expect("cloned AC exists");
    assert_eq!(
        cloned_ac.dependencies(),
        &[AnalysisDependency::new(
            AnalysisKind::OperatingPoint,
            cloned_op
        )]
    );
    clone.validate_structure().expect("clone remains valid");
}

#[test]
fn configuration_change_has_a_durable_interleaved_revision_receipt() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    let (_, edit_receipt) = plan.edit(transient, |_| ()).expect("analysis edit commits");
    let configuration_receipt = plan
        .commit_configuration_change("Saved outputs were updated.")
        .expect("configuration change commits");
    let disable_receipt = plan
        .set_enabled(transient, false)
        .expect("subsequent analysis mutation commits");

    assert_eq!(edit_receipt.sequence(), 1);
    assert_eq!(configuration_receipt.sequence(), 2);
    assert_eq!(disable_receipt.sequence(), 3);
    assert_eq!(
        configuration_receipt.source_revision(),
        edit_receipt.committed_revision()
    );
    assert_eq!(
        disable_receipt.source_revision(),
        configuration_receipt.committed_revision()
    );
    assert_eq!(plan.revision(), disable_receipt.committed_revision());
    assert_eq!(plan.configuration_receipts(), &[configuration_receipt]);
    plan.validate_structure()
        .expect("interleaved receipt sequence remains structurally valid");

    let json = snapshot(&plan);
    let mut restored: SimulationPlan =
        serde_json::from_str(&json).expect("configuration receipts round-trip");
    restored
        .validate_structure()
        .expect("restored receipt sequence remains valid");
    let next = restored
        .commit_configuration_change("Specifications were updated.")
        .expect("restored sequence remains appendable");
    assert_eq!(next.sequence(), 4);
}

#[test]
fn invalid_configuration_detail_is_atomic() {
    let mut plan = SimulationPlan::new();
    let before = snapshot(&plan);
    assert_eq!(
        plan.commit_configuration_change("line one\nline two"),
        Err(AnalysisPlanError::InvalidConfigurationChangeDetail)
    );
    assert_eq!(snapshot(&plan), before);
}

#[test]
fn restore_relinquishes_execution_authority_without_changing_identity_or_revision() {
    for lifecycle in [
        AnalysisLifecycleState::Queued,
        AnalysisLifecycleState::Running,
        AnalysisLifecycleState::Paused,
    ] {
        let mut plan = SimulationPlan::new();
        let id = plan.instances[0].id;
        let revision = plan.revision;
        plan.instances[0].lifecycle = lifecycle;

        plan.prepare_after_restore();

        assert_eq!(plan.instances[0].id, id);
        assert_eq!(plan.revision, revision);
        assert_eq!(plan.instances[0].lifecycle, AnalysisLifecycleState::Draft);
        plan.edit(id, |_| ())
            .expect("restored instance is editable without stale runner authority");
    }
}

#[test]
fn insertion_allows_missing_prerequisite_but_freeze_rejects_it() {
    let mut plan = SimulationPlan::new();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    assert!(
        plan.validation_issues()
            .contains(&AnalysisPlanIssue::MissingPrerequisite {
                dependent: ac,
                prerequisite: AnalysisKind::OperatingPoint,
            })
    );
    plan.validate_structure()
        .expect("editable missing bindings are not corruption");
    assert!(matches!(
        plan.freeze(),
        Err(AnalysisPlanError::InvalidPlan(_))
    ));
}

#[test]
fn quasi_periodic_dependencies_form_a_stable_explicit_graph() {
    let mut plan = SimulationPlan::new();
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (qpss, _) = plan.insert(AnalysisKind::Qpss).expect("QPSS inserts");
    plan.auto_bind_dependencies(qpss).expect("QPSS binds to OP");
    let (qpac, _) = plan.insert(AnalysisKind::Qpac).expect("QPAC inserts");
    plan.auto_bind_dependencies(qpac)
        .expect("QPAC binds to QPSS");

    assert_eq!(
        plan.instance(qpss).expect("QPSS exists").dependencies(),
        &[AnalysisDependency::new(AnalysisKind::OperatingPoint, op)]
    );
    assert_eq!(
        plan.instance(qpac).expect("QPAC exists").dependencies(),
        &[AnalysisDependency::new(AnalysisKind::Qpss, qpss)]
    );
    assert!(plan.freeze().is_ok());
}

#[test]
fn dependency_repair_inserts_a_missing_prerequisite_before_its_consumer() {
    let mut plan = SimulationPlan::empty();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");

    let (repair, receipt) = plan
        .repair_dependencies(ac)
        .expect("missing OP repairs atomically");

    assert_eq!(repair.dependent(), ac);
    assert_eq!(repair.inserted().len(), 1);
    assert!(repair.enabled().is_empty());
    assert!(repair.moved().is_empty());
    assert_eq!(repair.bound().len(), 1);
    let op = repair.inserted()[0];
    assert_eq!(plan.instances()[0].id(), op);
    assert_eq!(plan.instances()[1].id(), ac);
    assert_eq!(
        plan.instance(ac).unwrap().dependencies(),
        &[AnalysisDependency::new(AnalysisKind::OperatingPoint, op)]
    );
    assert_eq!(receipt.command(), AnalysisLifecycleCommand::Dependency);
    assert_eq!(receipt.instance_id(), ac);
    assert!(plan.freeze().is_ok());
}

#[test]
fn dependency_repair_reuses_and_enables_a_disabled_prerequisite_identity() {
    let mut plan = SimulationPlan::empty();
    let op = AnalysisInstanceId::new();
    plan.insert_draft_with_id(
        op,
        AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        false,
        0,
    )
    .expect("disabled OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");

    let (repair, _) = plan
        .repair_dependencies(ac)
        .expect("disabled OP is enabled and bound");

    assert!(repair.inserted().is_empty());
    assert_eq!(repair.enabled(), &[op]);
    assert!(repair.moved().is_empty());
    assert!(plan.instance(op).unwrap().enabled());
    assert_eq!(plan.instance(ac).unwrap().dependencies()[0].target(), op);
    assert!(plan.freeze().is_ok());
}

#[test]
fn dependency_repair_moves_a_later_prerequisite_without_replacing_its_identity() {
    let mut plan = SimulationPlan::empty();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("later OP inserts");

    let (repair, _) = plan
        .repair_dependencies(ac)
        .expect("later OP is moved and bound");

    assert!(repair.inserted().is_empty());
    assert!(repair.enabled().is_empty());
    assert_eq!(repair.moved(), &[op]);
    assert_eq!(plan.instances()[0].id(), op);
    assert_eq!(plan.instances()[1].id(), ac);
    assert_eq!(plan.instance(ac).unwrap().dependencies()[0].target(), op);
    assert!(plan.freeze().is_ok());
}

#[test]
fn dependency_repair_builds_the_complete_multi_level_prerequisite_closure() {
    let mut plan = SimulationPlan::empty();
    let (qpac, _) = plan.insert(AnalysisKind::Qpac).expect("QPAC inserts");

    let (repair, receipt) = plan
        .repair_dependencies(qpac)
        .expect("QPAC closure repairs");

    assert_eq!(repair.inserted().len(), 2);
    assert_eq!(repair.bound().len(), 2);
    assert_eq!(receipt.committed_revision(), plan.revision());
    assert_eq!(
        plan.instances()
            .iter()
            .map(AnalysisInstance::kind)
            .collect::<Vec<_>>(),
        vec![
            AnalysisKind::OperatingPoint,
            AnalysisKind::Qpss,
            AnalysisKind::Qpac,
        ]
    );
    let op = plan.instances()[0].id();
    let qpss = plan.instances()[1].id();
    assert_eq!(
        plan.instance(qpss).unwrap().dependencies(),
        &[AnalysisDependency::new(AnalysisKind::OperatingPoint, op)]
    );
    assert_eq!(
        plan.instance(qpac).unwrap().dependencies(),
        &[AnalysisDependency::new(AnalysisKind::Qpss, qpss)]
    );
    assert!(plan.freeze().is_ok());
}

#[test]
fn every_declared_prerequisite_kind_repairs_to_an_exact_frozen_closure() {
    let dependent_kinds = AnalysisKind::ALL
        .into_iter()
        .filter(|kind| !kind.prerequisites().is_empty())
        .collect::<Vec<_>>();
    assert_eq!(dependent_kinds.len(), 24);

    for kind in dependent_kinds {
        let mut plan = SimulationPlan::empty();
        let (dependent, _) = plan.insert(kind).expect("dependent inserts");
        let (repair, receipt) = plan
            .repair_dependencies_with_context(dependent, &exact_periodic_context())
            .unwrap_or_else(|error| panic!("{} prerequisite repair failed: {error}", kind));

        assert_eq!(repair.dependent(), dependent, "{kind}");
        assert_eq!(receipt.instance_id(), dependent, "{kind}");
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Dependency);
        assert!(plan.validation_issues().is_empty(), "{kind}");

        for (position, instance) in plan.instances().iter().enumerate() {
            assert_eq!(
                instance.dependencies().len(),
                instance.prerequisite_roles().len(),
                "{} must bind every declared role exactly once",
                instance.kind()
            );
            for prerequisite in instance.prerequisite_roles() {
                let bindings = instance
                    .dependencies()
                    .iter()
                    .filter(|dependency| dependency.prerequisite() == *prerequisite)
                    .collect::<Vec<_>>();
                assert_eq!(bindings.len(), 1, "{} -> {prerequisite}", instance.kind());
                let target_position = plan
                    .instances()
                    .iter()
                    .position(|candidate| candidate.id() == bindings[0].target())
                    .expect("repaired target remains present");
                assert!(
                    target_position < position,
                    "{} -> {prerequisite}",
                    instance.kind()
                );
                assert_eq!(
                    plan.instances()[target_position].kind(),
                    *prerequisite,
                    "{} -> {prerequisite}",
                    instance.kind()
                );
                assert!(plan.instances()[target_position].enabled());
            }
        }
        plan.freeze()
            .unwrap_or_else(|error| panic!("{kind} repaired plan did not freeze: {error}"));
    }
}

#[test]
fn periodic_consumers_reject_legacy_hb_pss_and_phase_noise_requires_autonomous_pss() {
    let mut plan = SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.method_idx = 1;
    })
    .expect("legacy PSS state remains editable");
    assert!(matches!(
        plan.bind_dependency(pac, AnalysisKind::Pss, pss),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { detail, .. })
            if detail.contains("HB-PSS")
    ));

    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.method_idx = 0;
        pss.osc_mode = false;
        pss.osc_node.clear();
        // A driven solve needs a tone, and only the design can name one.
        pss.tone_sources = "VSRC".to_owned();
    })
    .expect("shooting PSS state edits");
    let (pnoise, _) = plan.insert(AnalysisKind::Pnoise).expect("PNOISE inserts");
    plan.edit(pnoise, |draft| {
        let AnalysisDraft::Pnoise(pnoise) = draft else {
            panic!("expected PNOISE draft");
        };
        pnoise.noise_ref_idx = 2;
    })
    .expect("phase-noise state edits");
    assert!(matches!(
        plan.bind_dependency(pnoise, AnalysisKind::Pss, pss),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { detail, .. })
            if detail.contains("autonomous")
    ));

    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.osc_mode = true;
        pss.osc_node = "out".to_owned();
        // Going autonomous drops the driven tone this draft carried. Keeping
        // both is the contradiction `PssConfig::validate` refuses: the tone and
        // the oscillator node are two different answers to what the period is.
        pss.tone_sources.clear();
    })
    .expect("autonomous PSS state edits");
    plan.bind_dependency(pnoise, AnalysisKind::Pss, pss)
        .expect("phase noise accepts an autonomous shooting PSS");
}

#[test]
fn guided_prerequisite_configuration_is_atomic_bound_and_fail_closed() {
    let mut plan = SimulationPlan::empty();
    let (pnoise, _) = plan.insert(AnalysisKind::Pnoise).expect("PNOISE inserts");
    plan.edit(pnoise, |draft| {
        let AnalysisDraft::Pnoise(pnoise) = draft else {
            panic!("expected PNOISE draft");
        };
        pnoise.noise_ref_idx = 2;
    })
    .expect("phase-noise state edits");
    let before_len = plan.instances().len();
    let mut pss = crate::simulation::dialog::PssDialogState::default();
    pss.osc_mode = true;
    pss.osc_node.clear();
    pss.tone_sources.clear();
    let (prepared, receipt) = plan
        .prepare_prerequisite_for_configuration(
            pnoise,
            AnalysisKind::Pss,
            AnalysisDraft::Pss(pss),
            &exact_periodic_context(),
        )
        .expect("guided prerequisite is retained atomically");

    assert_eq!(plan.instances().len(), before_len + 2);
    assert_eq!(receipt.related_instance_id(), Some(pnoise));
    assert_eq!(receipt.outcome(), AnalysisLifecycleState::Disabled);
    let prepared_position = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == prepared)
        .expect("prepared PSS remains ordered");
    assert_eq!(
        plan.instances()[prepared_position].kind(),
        AnalysisKind::Pss
    );
    assert!(!plan.instances()[prepared_position].enabled());
    let op_dependency = plan.instances()[prepared_position].dependencies()[0];
    let op_position = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == op_dependency.target())
        .expect("machine-inferable OP prerequisite is prepared");
    assert!(op_position < prepared_position);
    assert_eq!(
        plan.instances()[op_position].kind(),
        AnalysisKind::OperatingPoint
    );
    assert!(plan.instances()[op_position].enabled());
    assert!(plan.instance(pnoise).unwrap().dependencies().is_empty());
    assert!(plan.validation_issues().iter().any(|issue| matches!(
        issue,
        AnalysisPlanIssue::MissingPrerequisite { dependent, prerequisite }
            if *dependent == pnoise && *prerequisite == AnalysisKind::Pss
    )));

    let before_rejected = snapshot(&plan);
    assert!(matches!(
        plan.prepare_prerequisite_for_configuration(
            pnoise,
            AnalysisKind::OperatingPoint,
            AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
            &exact_periodic_context(),
        ),
        Err(AnalysisPlanError::UnexpectedDependencyRole { .. })
    ));
    assert_eq!(snapshot(&plan), before_rejected);
}

#[test]
fn periodic_repair_requires_exact_sources_and_rolls_back_without_them() {
    let mut plan = SimulationPlan::empty();
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    let before = snapshot(&plan);
    assert!(matches!(
        plan.repair_dependencies(pac),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { detail, .. })
            if detail.contains("catalog is unavailable")
    ));
    assert_eq!(snapshot(&plan), before, "failed repair must be atomic");

    let (repair, _) = plan
        .repair_dependencies_with_context(pac, &exact_periodic_context())
        .expect("exact circuit sources synthesize PSS");
    let pss = repair
        .inserted()
        .iter()
        .copied()
        .find(|id| {
            plan.instance(*id)
                .is_some_and(|instance| instance.kind() == AnalysisKind::Pss)
        })
        .expect("repair inserted PSS");
    let AnalysisDraft::Pss(pss) = plan.instance(pss).unwrap().draft() else {
        panic!("repair inserted PSS");
    };
    assert_eq!(pss.tone_sources, "VLO, VRF");
}

#[test]
fn periodic_auto_bind_is_atomic_when_the_source_contract_is_unavailable() {
    let mut plan = SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.tone_sources = "VLO, VRF".to_owned();
    })
    .expect("PSS sources edit");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, pss)
        .expect("PSS binds before catalog loss");
    let before = snapshot(&plan);

    assert!(matches!(
        plan.auto_bind_dependencies_with_context(
            pac,
            &AnalysisDependencyRepairContext::periodic_sources_unavailable(
                "source elaboration failed",
            ),
        ),
        Err(AnalysisPlanError::DependencyConfigurationInvalid { detail, .. })
            if detail.contains("source elaboration failed")
    ));
    assert_eq!(snapshot(&plan), before, "failed auto-bind must be atomic");
}

#[test]
fn periodic_repair_does_not_reuse_a_pss_with_unknown_or_omitted_sources() {
    let mut plan = SimulationPlan::empty();
    let (stale_pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.edit(stale_pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.tone_sources = "STALE_SOURCE".to_owned();
    })
    .expect("stale PSS source identity remains editable");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    let context = exact_periodic_context();
    assert!(!plan.dependency_candidate_is_compatible_with_context(
        pac,
        AnalysisKind::Pss,
        stale_pss,
        &context,
    ));

    let (repair, _) = plan
        .repair_dependencies_with_context(pac, &context)
        .expect("repair inserts a circuit-exact PSS");
    let repaired_pss = repair
        .inserted()
        .iter()
        .copied()
        .find(|id| {
            plan.instance(*id)
                .is_some_and(|instance| instance.kind() == AnalysisKind::Pss)
        })
        .expect("repair inserted PSS");
    assert_ne!(repaired_pss, stale_pss);
    assert_eq!(
        plan.instance(pac).unwrap().dependencies()[0].target(),
        repaired_pss
    );
    let AnalysisDraft::Pss(stale) = plan.instance(stale_pss).unwrap().draft() else {
        panic!("stale instance remains PSS");
    };
    assert_eq!(stale.tone_sources, "STALE_SOURCE");
}

#[test]
fn periodic_repair_reuses_an_exact_autonomous_pss_without_driven_sources() {
    let mut plan = SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.tone_sources.clear();
        pss.osc_mode = true;
        pss.osc_node = "out".to_owned();
    })
    .expect("autonomous PSS state edits");
    let (pnoise, _) = plan.insert(AnalysisKind::Pnoise).expect("PNOISE inserts");
    plan.edit(pnoise, |draft| {
        let AnalysisDraft::Pnoise(pnoise) = draft else {
            panic!("expected PNOISE draft");
        };
        pnoise.noise_ref_idx = 2;
    })
    .expect("phase-noise state edits");
    let context = AnalysisDependencyRepairContext::exact_periodic_sources(
        "autonomous fixture\nR1 out 0 1k\n.end\n",
    )
    .expect("an exact empty source catalog is authoritative");

    let (repair, _) = plan
        .repair_dependencies_with_context(pnoise, &context)
        .expect("repair reuses the exact autonomous PSS");

    assert!(repair.inserted().iter().all(|id| {
        plan.instance(*id)
            .is_some_and(|instance| instance.kind() != AnalysisKind::Pss)
    }));
    assert_eq!(
        plan.instance(pnoise).unwrap().dependencies()[0].target(),
        pss
    );
}

#[test]
fn dependency_repairability_rejects_roles_the_consumer_does_not_declare() {
    let mut plan = SimulationPlan::empty();
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    assert!(!plan.dependency_prerequisite_is_repairable(op, AnalysisKind::Ac));
}

#[test]
fn dependency_repair_replaces_self_dangling_wrong_kind_and_duplicate_bindings() {
    fn assert_repaired(mut plan: SimulationPlan, dependent: AnalysisInstanceId) {
        let (repair, _) = plan
            .repair_dependencies(dependent)
            .expect("recoverable corrupt binding repairs atomically");
        assert!(repair.changed());
        let instance = plan.instance(dependent).expect("dependent remains present");
        assert_eq!(
            instance.dependencies().len(),
            instance.prerequisite_roles().len()
        );
        assert!(plan.validation_issues().is_empty());
        assert!(plan.freeze().is_ok());
    }

    let mut self_bound = SimulationPlan::empty();
    let (ac, _) = self_bound.insert(AnalysisKind::Ac).expect("AC inserts");
    self_bound.instances[0]
        .dependencies
        .push(AnalysisDependency::new(AnalysisKind::OperatingPoint, ac));
    assert_repaired(self_bound, ac);

    let mut dangling = SimulationPlan::empty();
    let (ac, _) = dangling.insert(AnalysisKind::Ac).expect("AC inserts");
    dangling.instances[0]
        .dependencies
        .push(AnalysisDependency::new(
            AnalysisKind::OperatingPoint,
            AnalysisInstanceId::new(),
        ));
    assert_repaired(dangling, ac);

    let mut wrong_kind = SimulationPlan::empty();
    let (transient, _) = wrong_kind
        .insert(AnalysisKind::Transient)
        .expect("Transient inserts");
    let (ac, _) = wrong_kind.insert(AnalysisKind::Ac).expect("AC inserts");
    let ac_index = wrong_kind.index_of(ac).expect("AC index");
    wrong_kind.instances[ac_index]
        .dependencies
        .push(AnalysisDependency::new(
            AnalysisKind::OperatingPoint,
            transient,
        ));
    assert_repaired(wrong_kind, ac);

    let mut duplicate = SimulationPlan::empty();
    let (first_op, _) = duplicate
        .insert(AnalysisKind::OperatingPoint)
        .expect("first OP inserts");
    let (second_op, _) = duplicate
        .insert(AnalysisKind::OperatingPoint)
        .expect("second OP inserts");
    let (ac, _) = duplicate.insert(AnalysisKind::Ac).expect("AC inserts");
    let ac_index = duplicate.index_of(ac).expect("AC index");
    duplicate.instances[ac_index].dependencies.extend([
        AnalysisDependency::new(AnalysisKind::OperatingPoint, first_op),
        AnalysisDependency::new(AnalysisKind::OperatingPoint, second_op),
    ]);
    assert_repaired(duplicate, ac);

    let mut unexpected = SimulationPlan::empty();
    let (ac, _) = unexpected.insert(AnalysisKind::Ac).expect("AC inserts");
    unexpected.instances[0]
        .dependencies
        .push(AnalysisDependency::new(AnalysisKind::Transient, ac));
    let (repair, _) = unexpected
        .repair_dependencies(ac)
        .expect("unexpected edge is removed while the required OP is repaired");
    assert_eq!(repair.removed().len(), 1);
    assert_eq!(repair.inserted().len(), 1);
    assert!(unexpected.validation_issues().is_empty());
    assert!(unexpected.freeze().is_ok());
}

#[test]
fn dependency_repair_preserves_an_existing_valid_binding() {
    let mut plan = SimulationPlan::empty();
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .expect("AC binds OP");
    let identities = plan
        .instances()
        .iter()
        .map(AnalysisInstance::id)
        .collect::<Vec<_>>();
    let dependencies = plan.instance(ac).unwrap().dependencies().to_vec();

    let (repair, receipt) = plan
        .repair_dependencies(ac)
        .expect("valid binding remains valid");

    assert!(!repair.changed());
    assert_eq!(
        plan.instances()
            .iter()
            .map(AnalysisInstance::id)
            .collect::<Vec<_>>(),
        identities
    );
    assert_eq!(plan.instance(ac).unwrap().dependencies(), dependencies);
    assert_eq!(receipt.instance_id(), ac);
    assert!(plan.freeze().is_ok());
}

#[test]
fn dependency_repair_rolls_back_when_a_reused_target_is_executing() {
    let mut plan = SimulationPlan::empty();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("later OP inserts");
    let op_index = plan.index_of(op).unwrap();
    plan.instances[op_index].lifecycle = AnalysisLifecycleState::Running;
    let before = snapshot(&plan);

    assert_eq!(
        plan.repair_dependencies(ac),
        Err(AnalysisPlanError::InstanceExecuting(op))
    );
    assert_eq!(snapshot(&plan), before);
}

#[test]
fn deep_clone_is_inserted_after_source_and_edits_do_not_alias() {
    let mut plan = SimulationPlan::new();
    let source = plan.instances()[0].id();
    plan.edit(source, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected transient");
        };
        draft.stop = "7u".to_owned();
    })
    .expect("source edits");
    let (clone, _) = plan.clone_instance(source).expect("clone succeeds");
    assert_eq!(plan.instances()[1].id(), clone);
    plan.edit(clone, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("expected transient");
        };
        draft.stop = "9u".to_owned();
    })
    .expect("clone edits");
    let AnalysisDraft::Transient(source_draft) = plan.instance(source).unwrap().draft() else {
        panic!("expected transient");
    };
    assert_eq!(source_draft.stop, "7u");
}

#[test]
fn failed_kind_changing_edit_is_fully_atomic() {
    let mut plan = SimulationPlan::new();
    let id = plan.instances()[0].id();
    let before = snapshot(&plan);
    let revision = plan.revision();
    let error = plan
        .edit(id, |draft| {
            *draft = AnalysisDraft::for_kind(AnalysisKind::Ac);
        })
        .expect_err("kind replacement must fail");
    assert!(matches!(error, AnalysisPlanError::DraftKindMismatch { .. }));
    assert_eq!(plan.revision(), revision);
    assert_eq!(snapshot(&plan), before);
}

#[test]
fn referenced_target_cannot_be_disabled_removed_or_reordered_after_dependent() {
    let mut plan = SimulationPlan::new();
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .expect("binds");

    for action in [0, 1, 2] {
        let before = snapshot(&plan);
        let result = match action {
            0 => plan.set_enabled(op, false).map(|_| ()),
            1 => plan.remove(op, Vec::new()).map(|_| ()),
            _ => plan.reorder(op, plan.instances().len() - 1).map(|_| ()),
        };
        assert!(result.is_err());
        assert_eq!(snapshot(&plan), before);
    }
}

#[test]
fn disabled_consumers_release_prerequisites_and_reenable_for_explicit_repair() {
    let mut plan = SimulationPlan::new();
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .expect("AC binds OP");

    plan.set_enabled(ac, false).expect("AC disables");
    plan.set_enabled(op, false)
        .expect("disabled AC no longer forces OP enabled");
    assert!(plan.validation_issues().iter().all(|issue| {
        !matches!(
            issue,
            AnalysisPlanIssue::DisabledDependency { dependent, .. } if *dependent == ac
        )
    }));

    let enable_receipt = plan.set_enabled(ac, true).expect("AC reenables as a draft");
    assert_eq!(enable_receipt.command(), AnalysisLifecycleCommand::Enable);
    assert!(
        plan.validation_issues()
            .contains(&AnalysisPlanIssue::MissingPrerequisite {
                dependent: ac,
                prerequisite: AnalysisKind::OperatingPoint,
            })
    );
    plan.validate_structure()
        .expect("a reenabled analysis may await explicit prerequisite repair");

    let (repair, _) = plan
        .repair_dependencies(ac)
        .expect("repair reuses and enables the retained OP identity");
    assert_eq!(repair.enabled(), &[op]);
    assert!(plan.validation_issues().is_empty());
}

#[test]
fn reenable_preserves_a_valid_exact_prerequisite_binding() {
    let mut plan = SimulationPlan::empty();
    let (first_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("first OP inserts");
    let (_second_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("second OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, first_op)
        .expect("AC binds the first exact OP identity");

    plan.set_enabled(ac, false).expect("AC disables");
    plan.set_enabled(ac, true).expect("AC reenables");

    assert_eq!(
        plan.instance(ac).unwrap().dependencies(),
        &[AnalysisDependency::new(
            AnalysisKind::OperatingPoint,
            first_op
        )]
    );
}

#[test]
fn disabled_instances_still_fail_closed_on_structural_dependency_corruption() {
    let mut plan = SimulationPlan::empty();
    let ac = AnalysisInstanceId::new();
    plan.insert_draft_with_id(ac, AnalysisDraft::for_kind(AnalysisKind::Ac), false, 0)
        .expect("disabled AC inserts");
    let missing = AnalysisInstanceId::new();
    plan.instances[0].dependencies.push(AnalysisDependency::new(
        AnalysisKind::OperatingPoint,
        missing,
    ));

    let error = plan
        .validate_structure()
        .expect_err("disabled drafts cannot hide dangling identities");
    let AnalysisPlanError::InvalidPlan(issues) = error else {
        panic!("expected structural validation failure");
    };
    assert!(issues.contains(&AnalysisPlanIssue::DanglingDependency {
        dependent: ac,
        target: missing,
    }));
}

#[test]
fn binding_rejects_self_dangling_wrong_disabled_and_later_targets() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (disabled_op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 1)
        .expect("OP inserts");
    plan.set_enabled(disabled_op, false).expect("disables");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    let (later_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");

    assert!(matches!(
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, ac),
        Err(AnalysisPlanError::SelfDependency { .. })
    ));
    assert!(matches!(
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, AnalysisInstanceId::new()),
        Err(AnalysisPlanError::DependencyTargetMissing { .. })
    ));
    assert!(matches!(
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, transient),
        Err(AnalysisPlanError::DependencyTargetWrongKind { .. })
    ));
    assert!(matches!(
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, disabled_op),
        Err(AnalysisPlanError::DependencyTargetDisabled { .. })
    ));
    assert!(matches!(
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, later_op),
        Err(AnalysisPlanError::DependencyTargetNotEarlier { .. })
    ));
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
        .expect("valid target binds");
}

#[test]
fn auto_bind_chooses_latest_enabled_matching_predecessor() {
    let mut plan = SimulationPlan::new();
    let (first, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts");
    let (latest, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 1)
        .expect("OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    let receipt = plan.auto_bind_dependencies(ac).expect("auto-bind succeeds");
    assert_eq!(receipt.command(), AnalysisLifecycleCommand::Dependency);
    assert_eq!(receipt.kind(), AnalysisKind::Ac);
    assert!(!receipt.detail().is_empty());
    assert_eq!(
        plan.instance(ac).unwrap().dependencies()[0].target(),
        latest
    );
    assert_ne!(first, latest);
}

#[test]
fn remove_tombstone_prevents_identity_reuse_and_retains_runs() {
    let mut plan = SimulationPlan::new();
    let id = plan.instances()[0].id();
    let run = RunId::new();
    let receipt = plan.remove(id, vec![run, run]).expect("remove succeeds");
    assert_eq!(receipt.command(), AnalysisLifecycleCommand::Remove);
    assert_eq!(receipt.kind(), AnalysisKind::Transient);
    assert_eq!(receipt.outcome(), AnalysisLifecycleState::Removed);
    assert!(!receipt.detail().is_empty());
    let tombstone = &plan.tombstones()[0];
    assert_eq!(tombstone.id(), id);
    assert_eq!(tombstone.prior_run_ids(), &[run]);
    assert!(
        plan.validation_issues()
            .contains(&AnalysisPlanIssue::NoEnabledInstances)
    );
    plan.validate_structure()
        .expect("zero enabled instances remains an editable plan");
    let before = snapshot(&plan);
    assert!(matches!(
        plan.insert_draft_with_id(
            id,
            AnalysisDraft::for_kind(AnalysisKind::Transient),
            true,
            0
        ),
        Err(AnalysisPlanError::RetiredIdentity(retired)) if retired == id
    ));
    assert_eq!(snapshot(&plan), before);
}

#[test]
fn supplied_plan_ids_and_instance_ids_are_preserved_and_validated() {
    let plan_id = SimulationPlanId::new();
    let instance_id = AnalysisInstanceId::new();
    let instance = AnalysisInstance::supplied(
        instance_id,
        AnalysisKind::Transient,
        AnalysisDraft::for_kind(AnalysisKind::Transient),
        true,
        Vec::new(),
        ObjectRevision::INITIAL,
        ObjectRevision::INITIAL,
    )
    .expect("supplied instance is valid");
    let plan =
        SimulationPlan::from_ordered_instances(plan_id, ObjectRevision::INITIAL, vec![instance])
            .expect("supplied plan is valid");
    assert_eq!(plan.id(), plan_id);
    assert_eq!(plan.instances()[0].id(), instance_id);
}

#[test]
fn supplied_corrupt_cycle_is_rejected_fail_closed() {
    let op_id = AnalysisInstanceId::new();
    let ac_id = AnalysisInstanceId::new();
    let op = AnalysisInstance::supplied(
        op_id,
        AnalysisKind::OperatingPoint,
        AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        true,
        vec![AnalysisDependency::new(AnalysisKind::Ac, ac_id)],
        ObjectRevision::INITIAL,
        ObjectRevision::INITIAL,
    )
    .expect("local instance shape is valid before graph validation");
    let ac = AnalysisInstance::supplied(
        ac_id,
        AnalysisKind::Ac,
        AnalysisDraft::for_kind(AnalysisKind::Ac),
        true,
        vec![AnalysisDependency::new(AnalysisKind::OperatingPoint, op_id)],
        ObjectRevision::INITIAL,
        ObjectRevision::INITIAL,
    )
    .expect("local instance shape is valid before graph validation");

    let error = SimulationPlan::from_ordered_instances(
        SimulationPlanId::new(),
        ObjectRevision::INITIAL,
        vec![op, ac],
    )
    .expect_err("cyclic supplied graph must be rejected");
    let AnalysisPlanError::InvalidPlan(issues) = error else {
        panic!("expected structural validation diagnostics");
    };
    assert!(
        issues
            .iter()
            .any(|issue| matches!(issue, AnalysisPlanIssue::DependencyCycle { .. }))
    );
}

#[test]
fn exhausted_revision_fails_without_any_mutation() {
    let revision = ObjectRevision::new(u64::MAX).expect("maximum revision is representable");
    let instance = AnalysisInstance::supplied(
        AnalysisInstanceId::new(),
        AnalysisKind::Transient,
        AnalysisDraft::for_kind(AnalysisKind::Transient),
        true,
        Vec::new(),
        revision,
        revision,
    )
    .expect("instance is locally valid");
    let mut plan =
        SimulationPlan::from_ordered_instances(SimulationPlanId::new(), revision, vec![instance])
            .expect("maximum persisted revision is structurally valid");
    let before = snapshot(&plan);

    assert!(matches!(
        plan.insert(AnalysisKind::DcSweep),
        Err(AnalysisPlanError::Revision(RevisionError::Exhausted))
    ));
    assert_eq!(snapshot(&plan), before);
}

#[test]
fn frozen_projection_is_deterministic_and_excludes_disabled_positions() {
    let mut plan = SimulationPlan::new();
    let transient = plan.instances()[0].id();
    let (disabled, insert_receipt) = plan.insert(AnalysisKind::DcSweep).expect("DC inserts");
    let disable_receipt = plan.set_enabled(disabled, false).expect("DC disables");
    assert_eq!(insert_receipt.sequence(), 1);
    assert_eq!(insert_receipt.command(), AnalysisLifecycleCommand::Insert);
    assert_eq!(insert_receipt.kind(), AnalysisKind::DcSweep);
    assert_eq!(insert_receipt.outcome(), AnalysisLifecycleState::Draft);
    assert!(!insert_receipt.detail().is_empty());
    assert_eq!(insert_receipt.source_revision(), ObjectRevision::INITIAL);
    assert_eq!(disable_receipt.sequence(), 2);
    assert_eq!(disable_receipt.command(), AnalysisLifecycleCommand::Disable);
    assert_eq!(disable_receipt.outcome(), AnalysisLifecycleState::Disabled);
    let persisted_receipt = serde_json::to_value(&disable_receipt).expect("receipt serializes");
    assert_eq!(persisted_receipt["command"], "disable");
    assert_eq!(persisted_receipt["kind"], "dc");
    assert_eq!(persisted_receipt["outcome"], "disabled");
    assert!(
        persisted_receipt["detail"]
            .as_str()
            .is_some_and(|detail| !detail.is_empty())
    );
    assert_eq!(
        insert_receipt.committed_revision(),
        disable_receipt.source_revision()
    );
    let frozen_a = plan.freeze().expect("plan freezes");
    let frozen_b = plan.freeze().expect("plan freezes again");
    assert_eq!(
        serde_json::to_string(&frozen_a).unwrap(),
        serde_json::to_string(&frozen_b).unwrap()
    );
    assert_eq!(frozen_a.instances().len(), 1);
    assert_eq!(frozen_a.instances()[0].id(), transient);
    assert_eq!(frozen_a.instances()[0].order(), 1);
}
