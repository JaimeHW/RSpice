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
        ..Default::default()
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
    let result = run(&base, &mut checkpoint, 3..4, &NoAbort, &|_| {
        panic!("restored trial was evaluated again")
    })
    .unwrap();
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
