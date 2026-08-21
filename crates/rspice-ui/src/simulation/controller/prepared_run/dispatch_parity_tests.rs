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

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::services::drc::DrcResult;
use crate::simulation::plan::{AnalysisKind, SimulationPlan};
use crate::state::{CanonicalAnalysisKind, PreparedRunTaskReceipt};

/// A design with a checked schematic and an attached technology — the state a
/// plan needs before preflight will look at its analyses at all.
fn preflight_ready_state() -> AppState {
    let mut state = AppState::default();
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind == crate::simulation::run_set::RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }
    crate::workbench::examples::load_example("Voltage Divider", &mut state.schematic);
    let mut drc = DrcResult::new();
    drc.completed = true;
    state.dialogs.drc_results = Some(drc);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    state.provision_test_project_technology_contract();
    state
}

fn plan_mut(state: &mut AppState) -> &mut SimulationPlan {
    state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a fresh project owns a stable analysis plan")
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

/// Name the fixture's own supply as the PSS tone source, and give that supply
/// a periodic waveform to be driven by.
///
/// The default draft names a differential input this fixture does not have,
/// and a shooting solve needs a time-varying source. Both are draft- and
/// design-authoring facts, not protocol ones.
fn drive_pss_from_the_fixture_supply(state: &mut AppState) {
    let supply = state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.name == "VCC")
        .expect("the fixture design owns a supply named VCC");
    // The PSS draft's default fundamental is 1 kHz.
    supply.value = "SIN(0 1 1k)".to_owned();
    state.sync_active_schematic_to_workspace();

    let plan = plan_mut(state);
    let pss = plan
        .instances()
        .iter()
        .find(|instance| instance.enabled() && instance.kind() == AnalysisKind::Pss)
        .map(crate::simulation::plan::AnalysisInstance::id)
        .expect("the fixture plan holds a PSS instance");
    plan.edit(pss, |draft| {
        let crate::simulation::plan::AnalysisDraft::Pss(draft) = draft else {
            panic!("a PSS instance owns a PSS draft");
        };
        draft.tone_sources = "VCC".to_owned();
    })
    .expect("the PSS tone source edits");
}

/// The canonical tag of every task a plan compiles to, in queue order.
fn queued_tags(state: &AppState) -> Result<Vec<u8>, Vec<String>> {
    Ok(compiled_queue(state)?
        .iter()
        .map(|task| crate::simulation::execution::analysis_kind_tag(&task.queued_analysis().spec))
        .collect())
}

fn compiled_queue(
    state: &AppState,
) -> Result<Vec<crate::simulation::execution::PreparedTask>, Vec<String>> {
    let controller = SimulationController::new();
    let plan = controller.build_analysis_plan(state)?;
    let sealed = state
        .model_library_manager
        .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        .map_err(|error| vec![error])?;
    controller.build_queue_from_plan(state, &plan, &sealed)
}

/// Seal the receipt rows a compiled queue authenticates, which is the step
/// dispatch performs and preflight now performs with it.
fn task_receipts(state: &AppState) -> Vec<PreparedRunTaskReceipt> {
    compiled_queue(state)
        .unwrap_or_else(|errors| panic!("the fixture plan compiles: {}", errors.join("; ")))
        .iter()
        .map(|task| {
            let tag = crate::simulation::execution::analysis_kind_tag(&task.queued_analysis().spec);
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
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
fn the_periodic_network_kinds_seal_a_receipt() {
    // HBSP, HBNOISE and PSP are the three advertised kinds whose tags the
    // receipt layer refused. Their compiled queues must now authenticate.
    //
    // These stop at the receipt rather than at a whole prepared snapshot: the
    // harmonic-balance prerequisite emits `.hb <f> harmonics=N`, which the
    // engine's `.HB` parser reads as frequencies only, so no HB-rooted plan
    // reaches a snapshot at all. That is a separate defect in the deck the
    // dialog writes, not in the tag protocol, and pinning it here would tie
    // this case to it.
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

    let controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
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
fn every_executable_kind_compiles_to_a_tag_the_receipt_layer_accepts() {
    // The ratchet. A kind with no execution blocker is advertised as runnable,
    // so every task it compiles to must produce a task receipt. This is what
    // makes a future kind's tag land on both sides of the protocol at once
    // instead of a year later, after a preflight has already lied.
    let mut checked = 0_usize;
    for kind in AnalysisKind::ALL {
        if kind.execution_blocker().is_some() {
            continue;
        }
        let mut state = preflight_ready_state();
        only(&mut state, &with_prerequisites(kind));

        let tags = match queued_tags(&state) {
            Ok(tags) => tags,
            // A default draft that needs an authored output or source is not a
            // tag-protocol failure, and this test is not the place to author
            // one. What must never happen is a compiled task whose tag the
            // receipt layer refuses, and an uncompiled plan has none.
            Err(_) => continue,
        };
        assert!(!tags.is_empty(), "{kind:?} compiled to no task at all");
        for tag in tags {
            PreparedRunTaskReceipt::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                tag,
                ContentDigest::from_bytes([tag; 32]),
            )
            .unwrap_or_else(|error| {
                panic!("{kind:?} dispatches tag {tag}, which no receipt accepts: {error}")
            });
        }
        checked += 1;
    }
    assert!(
        checked >= 20,
        "the ratchet must cover the executable catalogue, not a handful of kinds ({checked})"
    );
}

#[test]
fn a_plan_kinds_declared_tag_is_the_tag_its_task_actually_carries() {
    // Persisted receipt validation checks a task's tag against the kind of the
    // plan analysis it names, so that declaration and what dispatch stamps
    // must be the same number. They were two lists; this is the test that says
    // they are one. A kind whose default draft needs authoring never reaches
    // the queue, and an uncompiled plan makes no claim to check.
    let mut checked = 0_usize;
    for kind in AnalysisKind::ALL {
        if kind.execution_blocker().is_some() {
            continue;
        }
        let mut state = preflight_ready_state();
        let ids = only(&mut state, &with_prerequisites(kind));
        let own_instance = *ids.last().expect("the kind under test is inserted last");

        let controller = SimulationController::new();
        let Ok(plan) = controller.build_analysis_plan(&state) else {
            continue;
        };
        let Ok(sealed) = state
            .model_library_manager
            .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        else {
            continue;
        };
        let Ok(tasks) = controller.build_queue_from_plan(&state, &plan, &sealed) else {
            continue;
        };
        let own_task = tasks
            .iter()
            .find(|task| task.instance_id() == own_instance)
            .unwrap_or_else(|| panic!("{kind:?} compiles a task of its own"));
        assert_eq!(
            crate::simulation::execution::analysis_kind_tag(&own_task.queued_analysis().spec),
            kind.canonical_kind().tag(),
            "{kind:?} dispatches a tag its own plan-kind declaration does not name"
        );
        checked += 1;
    }
    assert!(checked >= 20, "only {checked} kinds were checked");
}
