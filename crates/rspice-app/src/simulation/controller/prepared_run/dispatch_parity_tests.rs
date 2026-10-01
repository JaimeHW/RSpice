//! Preflight and dispatch must agree about what is runnable.
//!
//! Preflight prepares a snapshot and reports the plan runnable; dispatch then
//! seals a receipt from that same snapshot. When only dispatch built the
//! receipt, a plan could clear preflight and be refused at Run — which is what
//! happened to every plan holding a default PSS, HBSP, HBNOISE or PSP, because
//! the receipt layer refused the tags those kinds carry. These cases pin the
//! agreement rather than the four tags: a kind added later gets the same
//! guarantee without anyone remembering to add a case.

use super::*;
mod catalog;
mod multirate_envelope;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisKind, SimulationPlan};
use crate::state::{CanonicalAnalysisKind, PreparedRunTaskReceipt};

use super::tests::runnable_state as preflight_ready_state;

fn plan_mut(state: &mut AppState) -> &mut SimulationPlan {
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a fresh project owns a stable analysis plan")
}

#[test]
fn independent_ac_data_instances_keep_their_own_parameter_tables() {
    use crate::simulation::plan::AnalysisDraft;
    let mut state = preflight_ready_state();
    let ids = only(
        &mut state,
        &[
            AnalysisKind::OperatingPoint,
            AnalysisKind::AcData,
            AnalysisKind::AcData,
        ],
    );
    for component in &mut state.schematic.document_mut_for_test().components {
        match component.name.as_str() {
            "VCC" => {
                component.kind = crate::state::ComponentType::VoltageSourceAc;
                component.value = "1".into();
            }
            "R1" | "R2" => component.value = "1k".into(),
            _ => {}
        }
    }
    state.sync_active_schematic_to_workspace();
    for (index, id) in ids[1..].iter().enumerate() {
        plan_mut(&mut state)
            .edit(*id, |draft| {
                let AnalysisDraft::AcData(draft) = draft else {
                    unreachable!()
                };
                draft.frequencies = "1k, 0".into();
                if index == 0 {
                    draft.parameter_columns.push(Default::default());
                    draft.parameter_columns[0].name = "R1:R".into();
                    draft.parameter_columns[0].values = "1k, 3k".into();
                }
            })
            .unwrap();
    }
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .unwrap();
    controller.run_authorization.retain(snapshot).unwrap();
    let dispatch = controller.consume_snapshot_for_dispatch(&state).unwrap();
    assert!(!dispatch.executable_netlist().contains(".DATA"));
    let mut ac_count = 0;
    for task in dispatch.into_tasks() {
        if !matches!(task.spec(), AnalysisSpec::AcData { .. }) {
            assert!(!task.executable_netlist().contains(".DATA"));
            continue;
        }
        let parsed = rspice_core::Netlist::parse(task.executable_netlist()).unwrap();
        assert_eq!(parsed.data_tables.len(), 1);
        assert_eq!(
            parsed.data_tables[0].params.len(),
            if ac_count == 0 { 2 } else { 1 }
        );
        let result = crate::simulation::controller::test_execution::run_resolved_task(
            task.resolve_dependency_artifacts(&Default::default())
                .unwrap(),
        )
        .unwrap();
        let rspice_simulation::results::SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = result
        else {
            panic!("AC result")
        };
        assert_eq!(frequencies, [1000.0, 0.0]);
        let expected = if ac_count == 0 {
            [0.5, 0.25]
        } else {
            [0.5, 0.5]
        };
        for (actual, expected) in waveforms["V(OUT)"].y_values.iter().zip(expected) {
            assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
        }
        ac_count += 1;
    }
    assert_eq!(ac_count, 2);
}

/// The kind, preceded by everything it declares a prerequisite on, transitively
/// and without repeats.
fn with_prerequisites(kind: AnalysisKind) -> Vec<AnalysisKind> {
    fn visit(kind: AnalysisKind, ordered: &mut Vec<AnalysisKind>) {
        if ordered.contains(&kind) {
            return;
        }
        for prerequisite in kind.prerequisites() {
            visit(*prerequisite, ordered);
        }
        ordered.push(kind);
    }
    let mut ordered = Vec::new();
    visit(kind, &mut ordered);
    ordered
}

/// Leave exactly `kinds` enabled in the project's own plan, bound to each
/// other where a prerequisite says they must be.
///
/// The plan's identity is preserved: the project keeps its variables, outputs
/// and specifications under that identity, and preparation refuses a plan
/// whose payload it cannot find.
fn only(state: &mut AppState, kinds: &[AnalysisKind]) -> Vec<AnalysisInstanceId> {
    let plan = plan_mut(state);
    for existing in plan
        .instances()
        .iter()
        .map(crate::simulation::plan::AnalysisInstance::id)
        .collect::<Vec<_>>()
    {
        plan.set_enabled(existing, false)
            .expect("a fresh plan's default analysis has no enabled dependent");
    }

    let mut inserted = Vec::with_capacity(kinds.len());
    let mut by_kind = std::collections::HashMap::new();
    for kind in kinds.iter().copied() {
        let (id, _) = plan
            .insert(kind)
            .unwrap_or_else(|error| panic!("{kind:?} inserts: {error}"));
        // A PSS names the tone it drives, and no default can know one. Binding
        // a dependent revalidates the prerequisite it binds to, so the tone has
        // to be authored the moment the PSS exists rather than once the plan is
        // assembled — otherwise PAC, PNOISE, PXF, PSTB and PSP all refuse to
        // bind, and this fixture would be asserting about tone sources instead
        // of about tags.
        if kind == AnalysisKind::Pss {
            name_the_fixture_tone_source(plan, id);
        }
        for prerequisite in kind.prerequisites() {
            let target = *by_kind
                .get(prerequisite)
                .unwrap_or_else(|| panic!("{kind:?} needs a {prerequisite:?} to bind to"));
            plan.bind_dependency(id, *prerequisite, target)
                .unwrap_or_else(|error| panic!("{kind:?} binds {prerequisite:?}: {error}"));
        }
        by_kind.entry(kind).or_insert(id);
        inserted.push(id);
    }
    inserted
}

/// The one independent source the fixture design owns.
const FIXTURE_TONE_SOURCE: &str = "VCC";

/// Point a PSS instance at the source the fixture design actually has.
fn name_the_fixture_tone_source(plan: &mut SimulationPlan, pss: AnalysisInstanceId) {
    plan.edit(pss, |draft| {
        let crate::simulation::plan::AnalysisDraft::Pss(draft) = draft else {
            panic!("a PSS instance owns a PSS draft");
        };
        draft.tone_sources = FIXTURE_TONE_SOURCE.to_owned();
    })
    .expect("the PSS tone source edits");
}

/// Give the fixture's supply a periodic waveform for the shooting solve to
/// settle on. The tone that names it is authored when the PSS is inserted.
fn drive_pss_from_the_fixture_supply(state: &mut AppState) {
    let supply = state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.name == FIXTURE_TONE_SOURCE)
        .expect("the fixture design owns a supply named VCC");
    // The PSS draft's default fundamental is 1 kHz.
    supply.kind = crate::state::ComponentType::VoltageSourceSin;
    supply.value = "SIN(0 1 1k)".to_owned();
    state.sync_active_schematic_to_workspace();
}

/// The canonical tag of every task a plan compiles to, in queue order.
fn queued_tags(state: &AppState) -> Result<Vec<u8>, Vec<String>> {
    Ok(compiled_queue(state)?
        .iter()
        .map(|task| {
            rspice_simulation::execution_identity::analysis_kind_tag(&task.queued_analysis().spec)
        })
        .collect())
}

fn compiled_queue(
    state: &AppState,
) -> Result<Vec<crate::simulation::execution::PreparedTask>, Vec<String>> {
    let plan = SimulationController::build_analysis_plan(&state.sim_setup)?;
    let sealed = state
        .model_library_manager
        .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        .map_err(|error| vec![error])?;
    SimulationController::build_queue_from_plan(state, &plan, &sealed)
}

/// Seal the receipt rows a compiled queue authenticates, which is the step
/// dispatch performs and preflight now performs with it.
fn task_receipts(state: &AppState) -> Vec<PreparedRunTaskReceipt> {
    compiled_queue(state)
        .unwrap_or_else(|errors| panic!("the fixture plan compiles: {}", errors.join("; ")))
        .iter()
        .map(|task| {
            let tag = rspice_simulation::execution_identity::analysis_kind_tag(
                &task.queued_analysis().spec,
            );
            PreparedRunTaskReceipt::new(
                task.instance_id(),
                task.source_revision(),
                task.dependencies().to_vec(),
                tag,
                task.config_digest(),
            )
            .unwrap_or_else(|error| panic!("tag {tag} must produce a task receipt: {error}"))
        })
        .collect()
}

#[test]
fn a_retaining_pss_plan_prepares_to_one_identity_every_time() {
    // The spectrum task is synthesized rather than authored, and its identity
    // is hashed into the prepared snapshot digest. Minting a fresh one made an
    // unchanged plan prepare differently on every build, so dispatch — which
    // rebuilds and compares — expired the authorization preflight had just
    // issued. Deriving it from the PSS it reads is what makes the plan one
    // run rather than a new one each time it is looked at.
    let mut state = preflight_ready_state();
    only(&mut state, &with_prerequisites(AnalysisKind::Pss));
    drive_pss_from_the_fixture_supply(&mut state);

    let baseline =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("baseline snapshot")
            .metadata();
    for attempt in 0..8 {
        let rebuilt = SimulationController::build_prepared_snapshot(
            &state,
            SimulationRunIntent::SimulateRunSet,
        )
        .expect("rebuilt snapshot")
        .metadata();
        assert_eq!(
            rebuilt.snapshot_digest, baseline.snapshot_digest,
            "rebuild {attempt} of an unchanged PSS plan moved the prepared identity"
        );
        assert_eq!(rebuilt.analysis_ids, baseline.analysis_ids);
    }
}

#[test]
fn the_receipt_preflight_seals_is_the_receipt_dispatch_seals() {
    // The invariant, stated directly. Preflight can only promise that a run
    // will dispatch if it builds the same object dispatch will refuse or
    // accept — anything less is a prediction. Equality here is what makes the
    // promise true, and it holds because both sides call one sealer.
    let mut state = preflight_ready_state();
    only(&mut state, &with_prerequisites(AnalysisKind::Pss));
    drive_pss_from_the_fixture_supply(&mut state);

    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("the plan prepares");
    let preflight_receipt = snapshot
        .prepared_run_receipt()
        .expect("preflight seals a receipt");

    controller
        .run_authorization
        .retain(snapshot)
        .expect("the snapshot authorizes");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("the permit is consumed");
    let dispatch_receipt = dispatch
        .prepared_run_receipt(crate::state::AnalysisResultSourceDomain::SimulationPlan)
        .expect("dispatch seals a receipt");

    assert_eq!(preflight_receipt, dispatch_receipt);
}

#[test]
fn a_default_pss_plan_prepares_a_receipt_instead_of_being_refused_at_dispatch() {
    let mut state = preflight_ready_state();
    only(&mut state, &with_prerequisites(AnalysisKind::Pss));
    drive_pss_from_the_fixture_supply(&mut state);

    // The default PSS retains twenty harmonics, and a retaining PSS queues a
    // companion spectrum task. That companion is what carried the tag the
    // receipt layer refused, so a plan with nothing unusual in it was reported
    // runnable and then blocked.
    let tags = queued_tags(&state).expect("a default PSS plan compiles");
    assert_eq!(
        tags,
        vec![
            CanonicalAnalysisKind::DcOp.tag(),
            CanonicalAnalysisKind::Pss.tag(),
            CanonicalAnalysisKind::PssSpectrum.tag(),
        ],
        "a retaining PSS earns its spectrum a prepared task of its own"
    );

    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("a default PSS plan prepares");
    let receipt = snapshot
        .prepared_run_receipt()
        .expect("preflight seals the receipt dispatch will seal");
    assert_eq!(
        receipt
            .tasks()
            .iter()
            .map(PreparedRunTaskReceipt::analysis_kind_tag)
            .collect::<Vec<_>>(),
        tags
    );

    let metadata = controller
        .prepare_run_set_for_preflight(&state)
        .expect("preflight reports the plan runnable");
    assert_eq!(metadata.task_count, tags.len());
}

#[test]
fn a_kind_without_a_solver_is_still_refused_by_its_own_blocker() {
    // The receipt layer now accepts every tag the canonical assignment can
    // emit, including those whose solver is absent from this build. That is
    // deliberate: the closed protocol describes what this binary's tag
    // assignment produces, and refusing a kind that has no solver is the
    // execution blocker's job. Losing that would turn a named "not available
    // in this engine build" into an anonymous unknown-tag refusal.
    let mut blocked = 0_usize;
    for kind in AnalysisKind::ALL {
        let Some(reason) = kind.execution_blocker() else {
            continue;
        };
        assert!(
            CanonicalAnalysisKind::from_tag(kind.canonical_kind().tag()).is_some(),
            "{kind:?} carries a tag outside the protocol"
        );

        let mut state = preflight_ready_state();
        only(&mut state, &with_prerequisites(kind));
        let errors = compiled_queue(&state)
            .expect_err("a kind with no solver cannot compile a dispatchable queue");
        assert!(
            errors.iter().any(|error| error.contains(reason)),
            "{kind:?} must be refused by name, not by tag: {errors:?}"
        );
        blocked += 1;
    }
    assert_eq!(
        blocked, 0,
        "the engine-blocked catalogue changed; re-read what the blockers now cover"
    );
}

#[test]
fn the_periodic_network_kinds_seal_a_receipt() {
    // HBSP, HBNOISE and PSP are the three advertised kinds whose tags the
    // receipt layer refused. Their compiled queues must now authenticate.
    //
    // These stop at the receipt rather than at a whole prepared snapshot,
    // which is all the tag protocol is about. The HB-rooted pair reach a whole
    // snapshot too now that the `.HB` card the dialog writes is one the parser
    // reads — see `an_hb_rooted_plan_prepares_a_whole_receipt`.
    for kind in [AnalysisKind::Hbsp, AnalysisKind::Hbnoise, AnalysisKind::Psp] {
        let mut state = preflight_ready_state();
        only(&mut state, &with_prerequisites(kind));
        if kind.prerequisites().contains(&AnalysisKind::Pss) {
            drive_pss_from_the_fixture_supply(&mut state);
        }

        let receipts = task_receipts(&state);
        assert!(
            receipts
                .iter()
                .any(|task| task.analysis_kind_tag() == kind.canonical_kind().tag()),
            "{kind:?} must reach the queue under its own canonical tag"
        );
        assert!(
            receipts
                .iter()
                .any(|task| task.result_analysis_type()
                    == kind.canonical_kind().result_analysis_type()),
            "{kind:?} must name a result family a retained result can match"
        );
    }
}

#[test]
fn a_psp_plan_prepares_a_whole_receipt() {
    // PSP is rooted at PSS rather than harmonic balance, so it does reach a
    // prepared snapshot — the end-to-end case for a tag above the retired cap.
    let mut state = preflight_ready_state();
    only(&mut state, &with_prerequisites(AnalysisKind::Psp));
    drive_pss_from_the_fixture_supply(&mut state);

    let snapshot =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
            .expect("a PSP plan prepares");
    let receipt = snapshot
        .prepared_run_receipt()
        .expect("a PSP plan seals the receipt dispatch will seal");
    assert!(
        receipt
            .tasks()
            .iter()
            .any(|task| task.analysis_kind_tag() == CanonicalAnalysisKind::Psp.tag()),
        "the sealed receipt authenticates the PSP task itself"
    );
}

#[test]
fn an_hb_rooted_plan_prepares_a_whole_receipt() {
    // Harmonic balance is advertised with no execution blocker, and until the
    // `.HB` card stopped carrying `harmonics=`/`oversample=` — keys the engine
    // parser refuses — no plan holding one could be prepared at all. Every
    // analysis in such a plan was refused, not just the HB.
    //
    // HBSP and HBNOISE ride the same card through their prerequisite, so all
    // three are pinned together: whatever the HB dialog writes has to survive
    // the parse that preparation performs.
    for kind in [
        AnalysisKind::HarmonicBalance,
        AnalysisKind::Hbsp,
        AnalysisKind::Hbnoise,
    ] {
        let mut state = preflight_ready_state();
        only(&mut state, &with_prerequisites(kind));

        let mut controller = SimulationController::new();
        let snapshot = SimulationController::build_prepared_snapshot(
            &state,
            SimulationRunIntent::SimulateRunSet,
        )
        .unwrap_or_else(|error| panic!("a default {kind:?} plan prepares: {error}"));
        let receipt = snapshot
            .prepared_run_receipt()
            .unwrap_or_else(|error| panic!("{kind:?} seals a prepared-run receipt: {error}"));
        assert!(
            receipt
                .tasks()
                .iter()
                .any(|task| task.analysis_kind_tag() == kind.canonical_kind().tag()),
            "the sealed receipt must authenticate the {kind:?} task itself"
        );

        let metadata = controller
            .prepare_run_set_for_preflight(&state)
            .unwrap_or_else(|error| panic!("preflight reports a {kind:?} plan runnable: {error}"));
        assert_eq!(metadata.task_count, receipt.tasks().len());
    }
}
