//! App circuit-source integration checks for portable plan transactions.
use super::*;
use crate::product::AnalysisInstanceId;
use crate::simulation::plan::AnalysisDependencyRepairContext;

fn snapshot(plan: &SimulationPlan) -> String {
    serde_json::to_string(plan).expect("plan serializes")
}

fn exact_periodic_context() -> AnalysisDependencyRepairContext {
    AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nVLO lo 0 SIN(0 1 1k)\nVRF rf 0 SIN(0 1 2k)\nR1 lo 0 1k\nR2 rf 0 1k\n.end\n",
        )
        .expect("test periodic-source fixture is exact")
}

fn set_periodic_carrier(
    plan: &mut SimulationPlan,
    id: AnalysisInstanceId,
    carrier: crate::services::simulation_runner::PeriodicCarrier,
) {
    plan.edit(id, |draft| match draft {
        AnalysisDraft::Pac(setup) => setup.carrier_idx = carrier.index(),
        AnalysisDraft::Pxf(setup) => setup.carrier_idx = carrier.index(),
        AnalysisDraft::Pnoise(setup) => setup.carrier_idx = carrier.index(),
        other => panic!("{} carries no periodic carrier", other.kind()),
    })
    .expect("a carrier position is an ordinary draft edit");
}

/// The three kinds whose prerequisite family the carrier chooses.
const CARRIER_DEPENDENT_KINDS: [AnalysisKind; 3] =
    [AnalysisKind::Pac, AnalysisKind::Pxf, AnalysisKind::Pnoise];

#[test]
fn every_declared_prerequisite_kind_repairs_to_an_exact_frozen_closure() {
    let dependent_kinds = AnalysisKind::ALL
        .into_iter()
        .filter(|kind| !kind.prerequisites().is_empty())
        .collect::<Vec<_>>();
    assert_eq!(dependent_kinds.len(), 26);

    // The carrier positions the three periodic small-signal kinds offer are
    // part of this closure, not an exemption from it: each position declares a
    // different prerequisite family, and every one of them has to repair to a
    // frozen plan exactly as a fixed role does.
    let cases = dependent_kinds
        .iter()
        .copied()
        .map(|kind| (kind, None))
        .chain(
            CARRIER_DEPENDENT_KINDS
                .into_iter()
                .flat_map(|kind| {
                    crate::services::simulation_runner::PeriodicCarrier::ALL
                        .iter()
                        .map(move |carrier| (kind, Some(*carrier)))
                })
                .collect::<Vec<_>>(),
        )
        .collect::<Vec<_>>();

    for (kind, carrier) in cases {
        let mut plan = SimulationPlan::empty();
        let (dependent, _) = plan.insert(kind).expect("dependent inserts");
        if let Some(carrier) = carrier {
            set_periodic_carrier(&mut plan, dependent, carrier);
            let expected = match carrier {
                crate::services::simulation_runner::PeriodicCarrier::Hb => {
                    AnalysisKind::HarmonicBalance
                }
                _ => AnalysisKind::Pss,
            };
            assert_eq!(
                plan.required_prerequisite_roles(dependent),
                vec![expected],
                "{kind} carried by {carrier:?} requires {expected}"
            );
        }
        let (repair, receipt) = plan
            .repair_dependencies_with_context(dependent, &exact_periodic_context())
            .unwrap_or_else(|error| panic!("{} prerequisite repair failed: {error}", kind));

        assert_eq!(repair.dependent(), dependent, "{kind}");
        assert_eq!(receipt.instance_id(), dependent, "{kind}");
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Dependency);
        assert!(plan.validation_issues().is_empty(), "{kind}");

        for (position, instance) in plan.instances().iter().enumerate() {
            let required_roles = plan.required_prerequisite_roles(instance.id());
            assert_eq!(
                instance.dependencies().len(),
                required_roles.len(),
                "{} must bind every declared role exactly once",
                instance.kind()
            );
            for prerequisite in &required_roles {
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

/// The carrier position that writes no `FROM=` keyword means what the engine
/// means by it: the nearest preceding periodic solve, of either family.
///
/// `resolve_periodic_source` in `rspice-core/src/execution/plan.rs` binds a
/// `.PAC`/`.PXF`/`.PNOISE` card without `FROM=` to whichever of `.PSS` or
/// `.HB` the deck wrote last. The plan resolves the same alternative against
/// its own order, so the instance the Studio binds is the instance the emitted
/// deck would bind — including the tie-break, which is proved here by putting
/// both families in the plan in both orders.
#[test]
fn a_preceding_carrier_binds_to_the_nearest_periodic_solve_of_either_family() {
    use crate::services::simulation_runner::PeriodicCarrier;

    for kind in CARRIER_DEPENDENT_KINDS {
        for (nearer, farther) in [
            (AnalysisKind::HarmonicBalance, AnalysisKind::Pss),
            (AnalysisKind::Pss, AnalysisKind::HarmonicBalance),
        ] {
            let mut plan = SimulationPlan::empty();
            let (_, _) = plan
                .insert(AnalysisKind::OperatingPoint)
                .expect("OP inserts");
            let (far, _) = plan.insert(farther).expect("the farther carrier inserts");
            let (_near, _) = plan.insert(nearer).expect("the nearer carrier inserts");
            // Both solves take their own operating point; leaving one unbound
            // would make the plan invalid for a reason this test is not about.
            for solve in [far, _near] {
                plan.repair_dependencies_with_context(solve, &exact_periodic_context())
                    .unwrap_or_else(|error| panic!("{kind} carrier repair failed: {error}"));
            }
            let (dependent, _) = plan.insert(kind).expect("dependent inserts");
            set_periodic_carrier(&mut plan, dependent, PeriodicCarrier::Preceding);

            assert_eq!(
                plan.required_prerequisite_roles(dependent),
                vec![nearer],
                "{kind} without FROM= follows the periodic solve written last"
            );
            plan.repair_dependencies_with_context(dependent, &exact_periodic_context())
                .unwrap_or_else(|error| panic!("{kind} closure repair failed: {error}"));
            let bound = plan
                .instance(dependent)
                .expect("dependent remains present")
                .dependencies()
                .to_vec();
            assert_eq!(bound.len(), 1, "{kind}");
            assert_eq!(bound[0].prerequisite(), nearer, "{kind}");
            // The nearest preceding enabled instance of that family, which is
            // not necessarily the one inserted above: a default `.PSS` draft
            // carries no tone sources, so repair synthesizes a valid one and
            // places it immediately before its consumer. Either way the claim
            // is the same — the *nearest* preceding solve of the family the
            // absent keyword resolved to, and never the other family at `far`.
            let position = plan
                .instances()
                .iter()
                .position(|instance| instance.id() == dependent)
                .expect("dependent remains present");
            let expected = plan.instances()[..position]
                .iter()
                .rev()
                .find(|candidate| candidate.enabled() && candidate.kind() == nearer)
                .map(|candidate| candidate.id())
                .unwrap_or_else(|| panic!("{kind} must have a {nearer} to read"));
            assert_eq!(
                bound[0].target(),
                expected,
                "{kind} must read the nearer {nearer}, not the {farther} at {far}"
            );
            assert_ne!(bound[0].target(), far, "{kind}");
            assert!(plan.validation_issues().is_empty(), "{kind}");
            plan.freeze()
                .unwrap_or_else(|error| panic!("{kind} did not freeze: {error}"));
        }
    }
}

/// A named harmonic-balance carrier is the role, whatever else the plan holds.
///
/// The preceding-solve position above resolves by order; `FROM=HB` does not.
/// A plan whose only periodic solve is a `.PSS` still requires a harmonic
/// balance of a request that names one, because the run linearizes around the
/// solution it named and binding the other family would report a different
/// measurement under this request's name.
#[test]
fn a_named_harmonic_balance_carrier_ignores_a_nearer_shooting_solve() {
    use crate::services::simulation_runner::PeriodicCarrier;

    for kind in CARRIER_DEPENDENT_KINDS {
        let mut plan = SimulationPlan::empty();
        plan.insert(AnalysisKind::OperatingPoint)
            .expect("OP inserts");
        let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
        plan.repair_dependencies_with_context(pss, &exact_periodic_context())
            .unwrap_or_else(|error| panic!("{kind} PSS repair failed: {error}"));
        let (dependent, _) = plan.insert(kind).expect("dependent inserts");
        set_periodic_carrier(&mut plan, dependent, PeriodicCarrier::Hb);

        assert_eq!(
            plan.required_prerequisite_roles(dependent),
            vec![AnalysisKind::HarmonicBalance],
            "{kind} from=hb requires a harmonic balance beside the PSS"
        );
        plan.repair_dependencies_with_context(dependent, &exact_periodic_context())
            .unwrap_or_else(|error| panic!("{kind} closure repair failed: {error}"));
        let dependencies = plan
            .instance(dependent)
            .expect("dependent remains present")
            .dependencies()
            .to_vec();
        assert_eq!(dependencies.len(), 1, "{kind}");
        assert_eq!(
            dependencies[0].prerequisite(),
            AnalysisKind::HarmonicBalance,
            "{kind}"
        );
        assert_eq!(
            plan.instance(dependencies[0].target())
                .expect("the repaired carrier is present")
                .kind(),
            AnalysisKind::HarmonicBalance,
            "{kind}"
        );
        assert!(plan.validation_issues().is_empty(), "{kind}");
        plan.freeze()
            .unwrap_or_else(|error| panic!("{kind} did not freeze: {error}"));
    }
}

/// Phase noise has no phase to diffuse on a driven carrier, whichever family
/// that carrier belongs to.
///
/// The engine refuses `.PNOISE NOISEREF=PHASE` against a harmonic-balance
/// orbit in `check_pnoise_card_carrier`, because an HB orbit's period is the
/// authored tone rather than a solver unknown. The plan editor refuses the
/// same pairing, so the refusal arrives before the run instead of after it.
#[test]
fn phase_noise_refuses_a_harmonic_balance_carrier() {
    use crate::services::simulation_runner::PeriodicCarrier;

    let mut plan = SimulationPlan::empty();
    plan.insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (dependent, _) = plan.insert(AnalysisKind::Pnoise).expect("PNOISE inserts");
    set_periodic_carrier(&mut plan, dependent, PeriodicCarrier::Hb);
    plan.edit(dependent, |draft| {
        let AnalysisDraft::Pnoise(setup) = draft else {
            panic!("expected a PNOISE draft");
        };
        setup.noise_ref_idx = 2;
    })
    .expect("a noise reference is an ordinary draft edit");

    let error = plan
        .repair_dependencies_with_context(dependent, &exact_periodic_context())
        .expect_err("a driven carrier cannot answer phase noise");
    let detail = error.to_string();
    assert!(
        detail.contains("autonomous") && detail.contains("harmonic-balance"),
        "the refusal must name the carrier and what phase noise needs: {detail}"
    );
}

/// Phase noise binds only to the carrier that has a phase to diffuse.
///
/// This also used to assert that a PAC refuses a legacy HB-PSS prerequisite,
/// through a draft with `method_idx = 1`. There is no such draft any more: the
/// solver-mode chooser had one executable position and is retired, so the
/// editor cannot ask for the harmonic-balance formulation at all. The refusal
/// itself is not gone — `execution::artifact` and `spec::validation` still
/// refuse a *sealed* specification that names it, and their own tests hold
/// them to it — it simply has no reachable draft to be provoked from here.
#[test]
fn phase_noise_requires_an_autonomous_pss_prerequisite() {
    let mut plan = SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
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
        plan.repair_dependencies_with_context(pac, &AnalysisDependencyRepairContext::default()),
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
