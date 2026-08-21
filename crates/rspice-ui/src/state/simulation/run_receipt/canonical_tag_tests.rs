//! What the receipt layer accepts, and where a synthesized task may sit.
//!
//! Two rules are pinned here. The canonical tag protocol must accept exactly
//! the tags dispatch can stamp — it once refused four of them, which turned a
//! green preflight into a refusal at Run. And a retained result must answer
//! for the task at its own index, which is why a task that reads another
//! task's output is queued after it.

use super::*;

use crate::state::{AnalysisResultProvenance, CanonicalAnalysisKind};

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn tag_of(kind: CanonicalAnalysisKind) -> u8 {
    kind.tag()
}

fn task_receipt(
    id: AnalysisInstanceId,
    dependencies: Vec<AnalysisInstanceId>,
    kind: CanonicalAnalysisKind,
    config_byte: u8,
) -> PreparedRunTaskReceipt {
    PreparedRunTaskReceipt::new(
        id,
        ObjectRevision::INITIAL,
        dependencies,
        tag_of(kind),
        digest(config_byte),
    )
    .unwrap_or_else(|error| panic!("{kind:?} must be a dispatchable task receipt: {error}"))
}

fn plan_receipt(tasks: Vec<PreparedRunTaskReceipt>) -> PreparedRunReceipt {
    PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        digest(0x71),
        digest(0x72),
        PreparedSourceCheckReceipt::SchematicDrc(digest(0x73)),
        tasks,
    )
    .expect("a fixture plan receipt is well formed")
}

fn result_for(task: &PreparedRunTaskReceipt, snapshot: ContentDigest) -> AnalysisResult {
    AnalysisResult::new(1, task.result_analysis_type(), "result").with_provenance(
        AnalysisResultProvenance::new(
            task.instance_id(),
            task.source_revision(),
            snapshot,
            task.dependencies().to_vec(),
        )
        .expect("a fixture result names its authenticated task"),
    )
}

/// The four kinds dispatch gained after the receipt cap shipped.
///
/// Each has no execution blocker and a real engine service, so each can reach
/// the receipt layer on a run the operator was told was runnable.
const TAGS_ADDED_AFTER_THE_CAP: [(CanonicalAnalysisKind, AnalysisType); 4] = [
    (CanonicalAnalysisKind::Hbsp, AnalysisType::Hbsp),
    (CanonicalAnalysisKind::Hbnoise, AnalysisType::Hbnoise),
    (CanonicalAnalysisKind::Psp, AnalysisType::Psp),
    (
        CanonicalAnalysisKind::PssSpectrum,
        AnalysisType::HarmonicBalance,
    ),
];

#[test]
fn each_tag_added_after_the_cap_constructs_validates_and_names_its_result_family() {
    for (index, (kind, expected_family)) in TAGS_ADDED_AFTER_THE_CAP.into_iter().enumerate() {
        let tag = tag_of(kind);
        assert!(
            tag > 25,
            "{kind:?} is only interesting because its tag {tag} is above the retired cap"
        );

        let id = AnalysisInstanceId::new();
        let task = PreparedRunTaskReceipt::new(
            id,
            ObjectRevision::INITIAL,
            Vec::new(),
            tag,
            digest(0x80 + index as u8),
        )
        .unwrap_or_else(|error| panic!("tag {tag} must be accepted: {error}"));

        assert_eq!(task.analysis_kind_tag(), tag);
        assert_eq!(task.canonical_kind(), kind);
        assert_eq!(task.result_analysis_type(), expected_family);

        // The whole-run receipt is what dispatch actually seals, and it is
        // where the refusal surfaced. Build one per tag.
        let receipt = plan_receipt(vec![task]);
        let snapshot = receipt.prepared_snapshot_digest();
        let result = result_for(&receipt.tasks()[0], snapshot);
        receipt
            .validate_result_prefix(std::slice::from_ref(&result))
            .unwrap_or_else(|error| {
                panic!("a result produced by tag {tag} must authenticate: {error}")
            });
    }
}

#[test]
fn an_undefined_tag_is_still_refused() {
    let highest = CanonicalAnalysisKind::ALL[CanonicalAnalysisKind::ALL.len() - 1].tag();
    for tag in [highest + 1, highest + 7, u8::MAX] {
        let error = PreparedRunTaskReceipt::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            tag,
            digest(0x90),
        )
        .expect_err("the protocol stays closed above what this binary defines");
        assert!(
            error.contains(&format!("unknown analysis kind tag {tag}")),
            "{error}"
        );
    }
}

/// The queue a default PSS produces: the operating point it is seeded from,
/// the shooting solve, and the spectrum task that reads that solve's periodic
/// state at a different index.
fn pss_spectrum_run() -> (PreparedRunReceipt, Vec<AnalysisResult>) {
    let op = AnalysisInstanceId::new();
    let pss = AnalysisInstanceId::new();
    let spectrum = AnalysisInstanceId::new();
    let receipt = plan_receipt(vec![
        task_receipt(op, Vec::new(), CanonicalAnalysisKind::DcOp, 0xa0),
        task_receipt(pss, vec![op], CanonicalAnalysisKind::Pss, 0xa1),
        task_receipt(
            spectrum,
            vec![pss],
            CanonicalAnalysisKind::PssSpectrum,
            0xa2,
        ),
    ]);
    let snapshot = receipt.prepared_snapshot_digest();
    let results = receipt
        .tasks()
        .iter()
        .map(|task| result_for(task, snapshot))
        .collect::<Vec<_>>();
    (receipt, results)
}

#[test]
fn a_run_ending_in_the_spectrum_task_authenticates_at_every_truncation() {
    let (receipt, results) = pss_spectrum_run();

    // A run can stop after any task: cancelled, failed, or still going. Every
    // prefix must authenticate, which is only true while the task that reads
    // another task's output is queued after it — an earlier position would
    // leave a hole at its own index in every partial run.
    for length in 0..=results.len() {
        receipt
            .validate_result_prefix(&results[..length])
            .unwrap_or_else(|error| {
                panic!("the first {length} result(s) must authenticate: {error}")
            });
    }

    assert!(
        receipt
            .validate_result_prefix(&[
                results[0].clone(),
                results[1].clone(),
                results[2].clone(),
                results[2].clone(),
            ])
            .is_err(),
        "more results than authenticated tasks must fail"
    );
}

#[test]
fn the_spectrum_result_cannot_be_moved_ahead_of_the_solve_it_reads() {
    let (receipt, results) = pss_spectrum_run();
    let [op, pss, spectrum] = <[AnalysisResult; 3]>::try_from(results).expect("three results");

    assert!(
        receipt
            .validate_result_prefix(&[op.clone(), spectrum.clone()])
            .is_err(),
        "the spectrum cannot occupy the PSS task's index"
    );
    assert!(
        receipt
            .validate_result_prefix(&[op, spectrum, pss])
            .is_err(),
        "reordering the pair fails the positional zip in both directions"
    );
}

#[test]
fn the_spectrum_reuses_the_harmonic_balance_result_family() {
    // Harmonics are indexed by frequency and the periodic waveform by time, so
    // the spectrum is not a second PSS result. It is drawn by the viewer that
    // already owns retained coefficients, and the receipt has to agree or a
    // correct spectrum fails to authenticate.
    assert_eq!(
        CanonicalAnalysisKind::PssSpectrum.result_analysis_type(),
        AnalysisType::HarmonicBalance
    );
    assert_ne!(
        CanonicalAnalysisKind::PssSpectrum.result_analysis_type(),
        CanonicalAnalysisKind::Pss.result_analysis_type()
    );
}
