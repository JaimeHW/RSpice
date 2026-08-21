//! A saved project must reopen a run whose tasks used the whole tag protocol.
//!
//! Project load re-derives a run's receipt and re-authenticates its retained
//! results against it, so a tag the receipt layer refuses does not merely
//! block a run — it makes the saved project refuse to open. These cases hold
//! the four tags that were once refused (HBSP, HBNOISE, PSP and the PSS
//! spectrum) through a full write, validate and restore.

use super::*;

use crate::simulation::plan::AnalysisKind;
use crate::state::CanonicalAnalysisKind;

const SNAPSHOT: [u8; 32] = [0xd0; 32];

/// One dispatched run of an RF plan: the operating point, a harmonic balance
/// and the two analyses that read it, then a periodic steady state, the
/// spectrum retained from it, and the periodic S-parameters that read it.
fn rf_run_tasks() -> Vec<(CanonicalAnalysisKind, Vec<usize>)> {
    vec![
        (CanonicalAnalysisKind::DcOp, Vec::new()),
        (CanonicalAnalysisKind::HarmonicBalance, vec![0]),
        (CanonicalAnalysisKind::Hbsp, vec![1]),
        (CanonicalAnalysisKind::Hbnoise, vec![1]),
        (CanonicalAnalysisKind::Pss, vec![0]),
        (CanonicalAnalysisKind::PssSpectrum, vec![4]),
        (CanonicalAnalysisKind::Psp, vec![4]),
    ]
}

/// A sealed run written exactly as `ProjectSimulationResults` writes it, with
/// `retained` leading results kept — a run may be cancelled partway, and the
/// retained results are then a prefix of the task list.
fn sealed_rf_results(retained: usize) -> ProjectSimulationResults {
    let snapshot = ContentDigest::from_bytes(SNAPSHOT);
    let rows = rf_run_tasks();
    let ids = rows
        .iter()
        .map(|_| AnalysisInstanceId::new())
        .collect::<Vec<_>>();

    let mut run = SimulationRun::new(77);
    let mut tasks = Vec::with_capacity(rows.len());
    for (index, (kind, dependencies)) in rows.iter().enumerate() {
        let dependency_ids = dependencies
            .iter()
            .map(|index| ids[*index])
            .collect::<Vec<_>>();
        tasks.push(
            PreparedRunTaskReceipt::new(
                ids[index],
                ObjectRevision::INITIAL,
                dependency_ids.clone(),
                kind.tag(),
                ContentDigest::from_bytes([0xe0_u8.wrapping_add(index as u8); 32]),
            )
            .unwrap_or_else(|error| panic!("tag {} must seal a task: {error}", kind.tag())),
        );
        if index < retained {
            run.add_analysis(
                AnalysisResult::new(
                    index as u64 + 1,
                    kind.result_analysis_type(),
                    format!("{kind:?}"),
                )
                .with_provenance(
                    AnalysisResultProvenance::new(
                        ids[index],
                        ObjectRevision::INITIAL,
                        snapshot,
                        dependency_ids,
                    )
                    .expect("a retained result names its authenticated task"),
                ),
            );
        }
    }

    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        snapshot,
        ContentDigest::from_bytes([0xd1; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xd2; 32])),
        tasks,
    )
    .expect("the RF run seals a receipt");
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("the fixture run seals explicitly");

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 77;
    ProjectSimulationResults::from_state(&simulation)
}

#[test]
fn a_run_using_the_whole_tag_protocol_survives_write_validate_and_restore() {
    let written = sealed_rf_results(rf_run_tasks().len());
    let encoded = serde_json::to_string(&written).expect("results serialize");
    let decoded =
        serde_json::from_str::<ProjectSimulationResults>(&encoded).expect("results deserialize");
    decoded
        .validate()
        .expect("a run using tags above the retired cap must still validate");

    let mut restored = SimulationState::default();
    decoded
        .apply_to_state(&mut restored)
        .expect("the written run restores into a session");

    let run = restored.runs.first().expect("one restored run");
    let Some(SimulationRunProvenance::Prepared(receipt)) = run.provenance() else {
        panic!("the restored run keeps its prepared receipt");
    };
    assert_eq!(
        receipt
            .tasks()
            .iter()
            .map(PreparedRunTaskReceipt::analysis_kind_tag)
            .collect::<Vec<_>>(),
        rf_run_tasks()
            .iter()
            .map(|(kind, _)| kind.tag())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        run.analyses
            .iter()
            .map(|analysis| analysis.analysis_type)
            .collect::<Vec<_>>(),
        rf_run_tasks()
            .iter()
            .map(|(kind, _)| kind.result_analysis_type())
            .collect::<Vec<_>>()
    );
}

#[test]
fn every_abort_prefix_of_that_run_also_reopens() {
    // A cancelled or failed RF run keeps only the results it produced. Each
    // truncation must still authenticate, which is what makes the spectrum's
    // position after the solve it reads load-bearing rather than cosmetic.
    for retained in 0..=rf_run_tasks().len() {
        let written = sealed_rf_results(retained);
        let encoded = serde_json::to_string(&written).expect("results serialize");
        let decoded = serde_json::from_str::<ProjectSimulationResults>(&encoded)
            .expect("results deserialize");
        decoded
            .validate()
            .unwrap_or_else(|error| panic!("a run retaining {retained} result(s): {error}"));
    }
}

#[test]
fn a_persisted_task_tag_must_match_the_plan_analysis_it_names() {
    // Persisted receipt validation compares a task's tag against the kind of
    // the plan analysis it names. That comparison and what dispatch stamps are
    // now one table, so this reads the table rather than a second list.
    for kind in AnalysisKind::ALL {
        let tag = analysis_kind_tag_for_plan_kind(kind);
        assert_eq!(tag, kind.canonical_kind().tag());
        assert!(
            CanonicalAnalysisKind::from_tag(tag).is_some(),
            "{kind:?} claims tag {tag}, which the receipt layer would refuse"
        );
    }
}
