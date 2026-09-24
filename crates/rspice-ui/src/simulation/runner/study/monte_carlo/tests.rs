//! Studio Monte Carlo tests cover resume, cancellation, publication, and legacy input.

use super::*;
use crate::simulation::config::{AcAnalysisConfig, AcSweepType};
use rspice_core::ResourceLimits;
use rspice_core::abort_signal::{AtomicAbort, ImmediateAbort, NoAbort};
use std::num::NonZeroUsize;
use std::ops::Range;

const SOURCE: &str = "Checkpoint study\n.param r=1k\nV1 in 0 DC 1 AC 1\nR1 in out {r}\nC1 out 0 1u\n.meas AC gain FIND VM(out) AT=1k\n.meas AC bounded FIND VM(out) AT=1k GOAL=1000 TOL=0.01\n.mc 6 uniform 0.2 seed 37 START=3\n.end\n";
fn base() -> StudyRunConfig {
    let analysis = AnalysisConfig::Ac(AcAnalysisConfig {
        start_freq: 1000.0,
        stop_freq: 1000.0,
        num_points: 1,
        sweep_type: AcSweepType::Linear,
    });
    StudyRunConfig {
        postprocess: None,
        constraints: vec![],
        objective_terms: vec![],
        instance_id: AnalysisInstanceId::new(),
        source_revision: ObjectRevision::INITIAL,
        analysis_line: analysis.to_spice(),
        analysis: analysis.into(),
        numeric_options: ".OPTIONS RELTOL=1e-5".into(),
        measurements: vec!["gain".into(), "bounded".into()],
        histogram_bins: 7,
    }
}
fn run(
    base: &StudyRunConfig,
    checkpoint: &mut Option<StudyMonteCarloCheckpoint>,
    range: Range<usize>,
    abort: &dyn AbortSignal,
    publish: &(dyn Fn(&StudyMonteCarloCheckpoint) -> Result<(), SimulationError> + Sync),
) -> Result<services::MonteCarloData, SimulationError> {
    run_monte_carlo_with_continuation(
        base,
        McVariationSource::ParameterTolerance,
        SOURCE,
        None,
        None,
        abort,
        Some(MonteCarloContinuation {
            checkpoint,
            trial_range: Some(range),
            publish_every: NonZeroUsize::new(1).unwrap(),
            publish,
        }),
    )
}

#[test]
fn studio_monte_carlo_checkpoint_resume_preserves_verdicts_and_pools_exact_trials() {
    let mut base = base();
    let mut head = None;
    run(&base, &mut head, 3..5, &NoAbort, &|_| Ok(())).unwrap();
    let mut tail = None;
    run(&base, &mut tail, 7..9, &NoAbort, &|_| Ok(())).unwrap();
    let mut pooled = head.unwrap();
    let limits = ResourceLimits::default();
    pooled
        .merge_with_limits(tail.as_ref().unwrap(), limits, &NoAbort)
        .unwrap();
    pooled
        .merge_with_limits(tail.as_ref().unwrap(), limits, &NoAbort)
        .unwrap();
    assert_eq!(pooled.completed_indices().collect::<Vec<_>>(), [3, 4, 7, 8]);
    let bytes = pooled.to_bytes_with_limits(limits, &NoAbort).unwrap();
    let mut restored =
        Some(StudyMonteCarloCheckpoint::from_bytes_with_limits(&bytes, limits, &NoAbort).unwrap());
    let publications = AtomicUsize::new(0);
    // A reporting-only edit can reuse the same trial population.
    base.histogram_bins = 5;
    // The editor advances the plan revision even when only reporting changes.
    base.source_revision = base.source_revision.next().unwrap();
    let resumed = run(&base, &mut restored, 3..9, &NoAbort, &|checkpoint| {
        publications.fetch_add(1, Ordering::Relaxed);
        assert!(checkpoint.completed_trials() >= 5);
        Ok(())
    })
    .unwrap();
    assert_eq!(
        publications.load(Ordering::Relaxed),
        2,
        "only two missing circuits are evaluated"
    );
    let uninterrupted = run_monte_carlo(
        &base,
        McVariationSource::ParameterTolerance,
        SOURCE,
        None,
        None,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(resumed.trial_measurements, uninterrupted.trial_measurements);
    assert_eq!(resumed.runs_completed, 6);
    assert_eq!(resumed.num_failures, 0);
    for member in &resumed.trial_measurements {
        let measured = member.evidence_for("bounded").unwrap();
        assert!(measured.value.is_some());
        assert!(!measured.passed);
        assert!(measured.error.as_deref().unwrap().contains("GOAL"));
    }
    for variable in &resumed.variables {
        let expected = uninterrupted
            .variables
            .iter()
            .find(|value| value.name == variable.name)
            .unwrap();
        assert_eq!(
            variable
                .samples
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected
                .samples
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(variable.mean.to_bits(), expected.mean.to_bits());
        assert_eq!(variable.std_dev.to_bits(), expected.std_dev.to_bits());
        assert_eq!(variable.histogram, expected.histogram);
        assert_eq!(variable.bin_edges, expected.bin_edges);
        assert_eq!(variable.mean_confidence, expected.mean_confidence);
    }
    crate::state::FamilyMemberMeasurements::validate_monte_carlo_sequence(
        &resumed.trial_measurements,
        37,
        6,
        6,
        0,
        resumed
            .variables
            .iter()
            .map(|v| (v.name.as_str(), v.samples.as_slice())),
    )
    .unwrap();
    let before = restored.clone();
    base.numeric_options = ".OPTIONS RELTOL=1e-4".into();
    assert!(
        run(&base, &mut restored, 3..9, &NoAbort, &|_| panic!(
            "mismatched population published"
        ))
        .unwrap_err()
        .to_string()
        .contains("population")
    );
    assert_eq!(restored, before);
}

#[test]
fn studio_monte_carlo_checkpoint_survives_cancellation_and_publication_failure() {
    let base = base();
    let flag = AtomicAbort::new();
    let mut checkpoint = None;
    let error = run(&base, &mut checkpoint, 3..4, &flag, &|_| {
        flag.set();
        Ok(())
    })
    .unwrap_err();
    assert!(matches!(error, SimulationError::Aborted));
    assert_eq!(checkpoint.as_ref().unwrap().completed_trials(), 1);
    let bytes = checkpoint
        .as_ref()
        .unwrap()
        .to_bytes_with_limits(ResourceLimits::default(), &NoAbort)
        .unwrap();
    checkpoint = Some(
        StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
    );
    let publications = AtomicUsize::new(0);
    let retained = checkpoint.clone();
    let result = run(&base, &mut checkpoint, 3..4, &NoAbort, &|snapshot| {
        publications.fetch_add(1, Ordering::Relaxed);
        assert_eq!(Some(snapshot), retained.as_ref());
        Ok(())
    })
    .unwrap();
    assert_eq!(
        publications.load(Ordering::Relaxed),
        1,
        "cached trials must be retained on the new run"
    );
    assert_eq!(result.runs_completed, 1);
    assert!(
        !result.trial_measurements[0]
            .evidence_for("bounded")
            .unwrap()
            .passed
    );
    let mut failed_publication = None;
    let calls = AtomicUsize::new(0);
    assert!(
        run(&base, &mut failed_publication, 3..4, &NoAbort, &|_| {
            calls.fetch_add(1, Ordering::Relaxed);
            Err(SimulationError::InvalidConfig(
                "checkpoint destination unavailable".into(),
            ))
        })
        .unwrap_err()
        .to_string()
        .contains("destination unavailable")
    );
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(failed_publication, checkpoint);
    let before = checkpoint.clone();
    assert!(
        run(
            &base,
            &mut checkpoint,
            3..4,
            &ImmediateAbort,
            &|_| unreachable!()
        )
        .is_err()
    );
    assert_eq!(checkpoint, before);
}

#[test]
fn studio_monte_carlo_checkpoint_reuses_trials_after_report_card_edits() {
    let base = base();
    let mut checkpoint = None;
    run(&base, &mut checkpoint, 3..5, &NoAbort, &|_| Ok(())).unwrap();
    let source = SOURCE.replace(
        "START=3",
        "START=3 CONFIDENCE=80 CI BOOTSTRAP RESAMPLES=32 BOOTSEED=77",
    );
    let publications = AtomicUsize::new(0);
    let resumed = run_monte_carlo_with_continuation(
        &base,
        McVariationSource::ParameterTolerance,
        &source,
        None,
        None,
        &NoAbort,
        Some(MonteCarloContinuation {
            checkpoint: &mut checkpoint,
            trial_range: None,
            publish_every: NonZeroUsize::new(1).unwrap(),
            publish: &|_| {
                publications.fetch_add(1, Ordering::Relaxed);
                Ok(())
            },
        }),
    )
    .unwrap();
    assert_eq!(
        publications.load(Ordering::Relaxed),
        4,
        "only missing trials solve"
    );
    let fresh = run_monte_carlo(
        &base,
        McVariationSource::ParameterTolerance,
        &source,
        None,
        None,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(resumed.trial_measurements, fresh.trial_measurements);
    for (actual, expected) in resumed.variables.iter().zip(&fresh.variables) {
        assert_eq!(actual.samples, expected.samples);
        assert_eq!(actual.mean_confidence, expected.mean_confidence);
    }
    let before = checkpoint.clone();
    run_monte_carlo_with_continuation(
        &base,
        McVariationSource::ParameterTolerance,
        &source.replace("seed 37", "seed 38"),
        None,
        None,
        &NoAbort,
        Some(MonteCarloContinuation {
            checkpoint: &mut checkpoint,
            trial_range: None,
            publish_every: NonZeroUsize::new(1).unwrap(),
            publish: &|_| panic!("incompatible source must not publish"),
        }),
    )
    .unwrap_err();
    assert_eq!(checkpoint, before);
}

#[test]
fn studio_monte_carlo_legacy_unitless_checkpoints_remain_readable_but_cannot_resume() {
    use crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointInput;
    use crate::state::MonteCarloCheckpointEvidence;
    let base = base();
    let limits = ResourceLimits::default();
    let mut typed = None;
    run(&base, &mut typed, 3..5, &NoAbort, &|_| Ok(())).unwrap();
    let typed = typed.unwrap();
    for partially_typed in [false, true] {
        let mut legacy = typed.clone();
        for (index, row) in &mut legacy.observations {
            if !partially_typed || *index == 3 {
                for observation in row {
                    observation.unit = None;
                }
            }
        }
        let bytes = legacy.to_bytes_with_limits(limits, &NoAbort).unwrap();
        let restored =
            StudyMonteCarloCheckpoint::from_bytes_with_limits(&bytes, limits, &NoAbort).unwrap();
        assert_eq!(restored, legacy);
        assert_eq!(
            MonteCarloCheckpointEvidence::from_bytes(bytes.clone().into())
                .unwrap()
                .bytes(),
            bytes.as_slice(),
            "historical evidence remains byte-for-byte intact",
        );
        let old_value = &restored.observations[&3][0];
        assert_eq!(old_value.value_in_unit("mV").unwrap(), old_value.value);
        let typed_value = &typed.observations[&3][0];
        assert!(
            (typed_value.value_in_unit("mV").unwrap().unwrap() - old_value.value.unwrap() * 1000.0)
                .abs()
                < 1e-10
        );
        assert!(
            MonteCarloCheckpointInput::from_bytes(bytes)
                .unwrap_err()
                .to_string()
                .contains("physical unit metadata")
        );
        // Neither a fully cached request nor a partial resume may publish or
        // append fresh observations with a different interpretation of units.
        for range in [3..5, 3..6] {
            let mut checkpoint = Some(restored.clone());
            let error = run(&base, &mut checkpoint, range, &NoAbort, &|_| {
                panic!("incompatible checkpoint must not publish")
            })
            .unwrap_err();
            assert!(error.to_string().contains("physical unit metadata"));
            assert_eq!(checkpoint.as_ref(), Some(&restored));
        }
    }
}
