//! Tests for plan structure and revision handling.
//!
//! Almost every case is fail-closed: a corrupt dependency cycle is rejected, an
//! exhausted revision fails without mutating, and the frozen projection is
//! deterministic and excludes disabled positions.

use super::*;
use rspice_app_types::product::RevisionError;

fn snapshot(plan: &SimulationPlan) -> String {
    serde_json::to_string(plan).expect("plan serializes")
}

#[test]
fn pvt_selected_and_other_study_clones_rebind_their_bases() {
    for kind in [
        AnalysisKind::Temperature,
        AnalysisKind::Corner,
        AnalysisKind::MonteCarlo,
        AnalysisKind::Optimization,
    ] {
        let mut plan = SimulationPlan::empty();
        let base = plan.insert(AnalysisKind::OperatingPoint).unwrap().0;
        let study = plan.insert(kind).unwrap().0;
        plan.edit(study, |draft| match draft {
            AnalysisDraft::Temperature(state) => state.base_analysis = Some(base),
            AnalysisDraft::Corner(state) => state.base_analysis = Some(base),
            AnalysisDraft::MonteCarlo(state) => state.base_analysis = Some(base),
            AnalysisDraft::Optimization(state) => state.base_analysis = Some(base),
            _ => unreachable!(),
        })
        .unwrap();
        let original = snapshot(&plan);
        let clone = plan.clone_as_new().unwrap();
        let selected = match clone.instances()[1].draft() {
            AnalysisDraft::Temperature(state) => state.base_analysis,
            AnalysisDraft::Corner(state) => state.base_analysis,
            AnalysisDraft::MonteCarlo(state) => state.base_analysis,
            AnalysisDraft::Optimization(state) => state.base_analysis,
            _ => unreachable!(),
        };
        assert_eq!(selected, Some(clone.instances()[0].id()));
        assert_ne!(selected, Some(base));
        assert_eq!(snapshot(&plan), original);
    }
}

#[test]
fn pvt_base_controls_restore_and_mode_changes_preserve_effective_settings() {
    for kind in [AnalysisKind::Temperature, AnalysisKind::Corner] {
        let mut plan = SimulationPlan::empty();
        let select = |draft: &mut AnalysisDraft, transient| match draft {
            AnalysisDraft::Temperature(state) => state.base_idx = if transient { 1 } else { 2 },
            AnalysisDraft::Corner(state) => state.base_analysis_idx = if transient { 0 } else { 1 },
            _ => unreachable!(),
        };
        let (dynamic, _) = plan.insert(kind).unwrap();
        let (static_base, _) = plan.insert(kind).unwrap();
        plan.edit(dynamic, |draft| select(draft, true)).unwrap();
        plan.edit(static_base, |draft| select(draft, false))
            .unwrap();
        let legacy: AnalysisNumericOverride = serde_json::from_str(
            r#"{"reltol":0.0001,"itl4":150,"strobe_interval":0.000001,"retain_every_signal":true}"#,
        )
        .unwrap();
        for id in [dynamic, static_base] {
            let index = plan.index_of(id).unwrap();
            plan.instances[index].numeric_override = Some(legacy.clone());
        }
        let history = serde_json::to_value(&plan.receipts).unwrap();
        let mut restored: SimulationPlan = serde_json::from_str(&snapshot(&plan)).unwrap();
        restored.prepare_after_restore();
        assert_eq!(
            restored.instance(dynamic).unwrap().numeric_override(),
            Some(&legacy)
        );
        let record = restored
            .instance(static_base)
            .unwrap()
            .numeric_override()
            .unwrap();
        assert!(record.value(NumericOverrideOption::Reltol).is_some());
        for option in [
            NumericOverrideOption::Itl4,
            NumericOverrideOption::StrobeInterval,
            NumericOverrideOption::RetainEverySignal,
        ] {
            assert!(record.value(option).is_none());
        }
        assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), history);
        let once = snapshot(&restored);
        restored.prepare_after_restore();
        assert_eq!(snapshot(&restored), once);
        let error = restored
            .edit(dynamic, |draft| select(draft, false))
            .unwrap_err();
        assert!(error.to_string().contains("select Transient under Base"));
        assert_eq!(snapshot(&restored), once);
        restored.set_numeric_override(dynamic, None).unwrap();
        restored
            .edit(dynamic, |draft| select(draft, false))
            .unwrap();
    }
}

#[test]
fn dc_study_restore_keeps_effective_controls_and_base_changes_are_atomic() {
    for kind in [AnalysisKind::MonteCarlo, AnalysisKind::Optimization] {
        let mut plan = SimulationPlan::empty();
        let (transient, _) = plan.insert(AnalysisKind::Transient).unwrap();
        let (dc, _) = plan.insert(kind).unwrap();
        let (configured, _) = plan.insert(kind).unwrap();
        let select = |draft: &mut AnalysisDraft, base| match draft {
            AnalysisDraft::MonteCarlo(state) => state.base_analysis = base,
            AnalysisDraft::Optimization(state) => state.base_analysis = base,
            _ => unreachable!(),
        };
        plan.edit(configured, |draft| select(draft, Some(transient)))
            .unwrap();
        let legacy: AnalysisNumericOverride = serde_json::from_str(
            r#"{"reltol":0.0001,"itl4":150,"strobe_interval":0.00000001,"retain_every_signal":true}"#,
        ).unwrap();
        for id in [dc, configured, transient] {
            let index = plan.index_of(id).unwrap();
            plan.instances[index].numeric_override = Some(legacy.clone());
        }
        let history = serde_json::to_value(&plan.receipts).unwrap();
        let mut restored: SimulationPlan = serde_json::from_str(&snapshot(&plan)).unwrap();
        restored.prepare_after_restore();
        let dc_options = restored.instance(dc).unwrap().numeric_override().unwrap();
        assert!(dc_options.value(NumericOverrideOption::Reltol).is_some());
        for option in [
            NumericOverrideOption::Itl4,
            NumericOverrideOption::StrobeInterval,
            NumericOverrideOption::RetainEverySignal,
        ] {
            assert!(dc_options.value(option).is_none());
        }
        assert_eq!(
            restored.instance(transient).unwrap().numeric_override(),
            Some(&legacy)
        );
        assert!(
            restored
                .instance(configured)
                .unwrap()
                .numeric_override()
                .is_none(),
            "all study defaults were superseded by the base's own settings"
        );
        assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), history);
        let once = snapshot(&restored);
        restored.prepare_after_restore();
        assert_eq!(snapshot(&restored), once);
        restored.set_numeric_override(transient, None).unwrap();
        restored
            .set_numeric_override(configured, Some(legacy))
            .unwrap();
        let once = snapshot(&restored);
        let error = restored
            .edit(configured, |draft| select(draft, None))
            .unwrap_err();
        assert!(error.to_string().contains("only DC operating points"));
        assert_eq!(snapshot(&restored), once);
        restored.set_numeric_override(configured, None).unwrap();
        restored
            .edit(configured, |draft| select(draft, None))
            .unwrap();
    }
}

#[test]
fn fourier_restore_retires_unused_overrides_and_preserves_its_producer() {
    let mut plan = SimulationPlan::empty();
    let (transient, _) = plan.insert(AnalysisKind::Transient).unwrap();
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).unwrap();
    plan.bind_dependency(fourier, AnalysisKind::Transient, transient)
        .unwrap();
    let legacy: AnalysisNumericOverride =
        serde_json::from_str(r#"{"reltol":0.0001,"itl4":200,"strobe_interval":0.000001}"#).unwrap();
    for id in [transient, fourier] {
        let index = plan.index_of(id).unwrap();
        plan.instances[index].numeric_override = Some(legacy.clone());
    }
    let mut restored: SimulationPlan = serde_json::from_str(&snapshot(&plan)).unwrap();
    let receipts = serde_json::to_value(&restored.receipts).unwrap();
    let before = restored.instance(fourier).unwrap();
    let draft = serde_json::to_value(&before.draft).unwrap();
    let dependencies = serde_json::to_value(&before.dependencies).unwrap();
    restored.prepare_after_restore();
    let after = restored.instance(fourier).unwrap();
    assert!(after.numeric_override().is_none());
    assert_eq!(serde_json::to_value(&after.draft).unwrap(), draft);
    assert_eq!(
        serde_json::to_value(&after.dependencies).unwrap(),
        dependencies
    );
    assert_eq!(
        restored.instance(transient).unwrap().numeric_override(),
        Some(&legacy)
    );
    assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), receipts);
    let once = snapshot(&restored);
    restored.prepare_after_restore();
    assert_eq!(snapshot(&restored), once);
    restored
        .edit(fourier, |_| ())
        .expect("restored Fourier remains editable");
}

#[test]
fn pss_retention_restore_preserves_reporting_and_receipts() {
    let mut plan = SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
    let (empty_pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
    let (transient, _) = plan.insert(AnalysisKind::Transient).unwrap();
    let retention: AnalysisNumericOverride =
        serde_json::from_str(r#"{"retain_every_signal":false}"#).unwrap();
    for id in [pss, empty_pss, transient] {
        let index = plan.index_of(id).unwrap();
        plan.instances[index].numeric_override = Some(retention.clone());
    }
    let index = plan.index_of(pss).unwrap();
    plan.instances[index]
        .numeric_override
        .as_mut()
        .unwrap()
        .set_for_instance(
            AnalysisKind::Pss,
            crate::numeric_override::SolverOwnership::NONE,
            NumericOverrideOption::StrobeInterval,
            "10n",
        )
        .unwrap();
    let mut restored: SimulationPlan = serde_json::from_str(&snapshot(&plan)).unwrap();
    let receipts = serde_json::to_value(&restored.receipts).unwrap();
    restored.prepare_after_restore();
    let record = restored.instance(pss).unwrap().numeric_override().unwrap();
    assert!(
        record
            .value(NumericOverrideOption::RetainEverySignal)
            .is_none()
    );
    assert_eq!(
        record.value(NumericOverrideOption::StrobeInterval),
        Some("10n".into())
    );
    assert!(
        restored
            .instance(empty_pss)
            .unwrap()
            .numeric_override()
            .is_none()
    );
    assert_eq!(
        restored.instance(transient).unwrap().numeric_override(),
        Some(&retention)
    );
    assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), receipts);
    let once = snapshot(&restored);
    restored.prepare_after_restore();
    assert_eq!(snapshot(&restored), once);
    restored
        .edit(pss, |_| ())
        .expect("restored PSS stays editable");
}

#[test]
fn restore_periodic_consumer_solver_ownership_preserves_carriers_and_history() {
    let mut plan = SimulationPlan::empty();
    let (hb, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
    let mut consumers = Vec::new();
    for kind in [
        AnalysisKind::Pac,
        AnalysisKind::Pxf,
        AnalysisKind::Pnoise,
        AnalysisKind::Pstb,
        AnalysisKind::Psp,
        AnalysisKind::Hbsp,
        AnalysisKind::Hbnoise,
        AnalysisKind::Qpac,
        AnalysisKind::Qpxf,
        AnalysisKind::Qpnoise,
    ] {
        consumers.push(plan.insert(kind).unwrap().0);
    }
    plan.prepare_after_restore();
    let legacy: AnalysisNumericOverride = serde_json::from_str(r#"{"reltol":0.000001}"#).unwrap();
    for id in std::iter::once(&hb).chain(&consumers) {
        let index = plan.index_of(*id).unwrap();
        plan.instances[index].numeric_override = Some(legacy.clone());
    }
    let drafts: Vec<_> = plan
        .instances
        .iter()
        .map(|instance| serde_json::to_value(&instance.draft).unwrap())
        .collect();
    let mut restored: SimulationPlan = serde_json::from_str(&snapshot(&plan)).unwrap();
    let history = serde_json::to_value(&restored.receipts).unwrap();
    restored.prepare_after_restore();
    assert_eq!(
        restored.instance(hb).unwrap().numeric_override(),
        Some(&legacy)
    );
    for id in &consumers {
        assert!(restored.instance(*id).unwrap().numeric_override().is_none());
        assert!(
            restored
                .set_numeric_override(*id, Some(legacy.clone()))
                .is_err()
        );
    }
    assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), history);
    assert_eq!(
        restored
            .instances
            .iter()
            .map(|instance| serde_json::to_value(&instance.draft).unwrap())
            .collect::<Vec<_>>(),
        drafts
    );
    let once = snapshot(&restored);
    restored.prepare_after_restore();
    assert_eq!(snapshot(&restored), once);
    for id in consumers {
        restored
            .edit(id, |_| ())
            .expect("restored consumer remains editable");
    }
}

/// A plan written before participation existed states no `run_at`, and must
/// reload running every analysis at every point.
///
/// The instance struct is `deny_unknown_fields`, so the whole document is
/// refused if the field is spelled wrong — but a *missing* field is refused
/// too unless it is `serde(default)`, and nothing tested that the default is
/// present or that it is `AllPoints`. Any other reading would narrow a run
/// nobody narrowed. This removes the key from a real serialized plan rather
/// than rewriting its value, because a rewritten value proves only that the
/// parser reads values.
#[test]
fn an_instance_stating_no_run_at_reloads_at_every_point() {
    let mut plan = SimulationPlan::empty();
    let (id, _) = plan.insert(AnalysisKind::Transient).expect("inserts");
    plan.set_run_at(id, AnalysisRunAt::NominalPoint)
        .expect("a transient takes a participation");

    let mut document: serde_json::Value =
        serde_json::to_value(&plan).expect("a plan serializes to JSON");
    let instance = document["instances"][0]
        .as_object_mut()
        .expect("the plan writes its instances as objects");
    assert!(
        instance.remove("run_at").is_some(),
        "the fixture must actually state a participation for its removal to mean anything"
    );

    let restored: SimulationPlan =
        serde_json::from_value(document).expect("a plan written before participation still loads");
    assert_eq!(
        restored
            .instance(id)
            .expect("the instance survives")
            .run_at(),
        &AnalysisRunAt::AllPoints,
        "an unstated participation is every point, which is what such a plan did"
    );
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

/// The carrier positions a periodic small-signal draft can hold, as this
/// module's tests drive them.
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
            instance.draft().prerequisite_roles().len()
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
