use super::*;
use crate::run::ExecutionTarget;

fn source(number: u64, analysis_id: u64, coordinates: Vec<f64>, values: Vec<f64>) -> SimulationRun {
    let mut run = SimulationRun::new(number, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(
        AnalysisResult::new(analysis_id, AnalysisType::Transient, "TRAN", 0.0)
            .with_waveforms(vec![RetainedWaveform::new("V(out)", coordinates, values)]),
    );
    run
}

fn options(alignment: ComparisonAlignmentDraft) -> ComparisonOptions<'static> {
    ComparisonOptions {
        alignment,
        alignment_signal: "V(out)",
        threshold: 0.0,
        maximum_lag_samples: 2,
        absolute_tolerance: 0.0,
        relative_tolerance: 0.0,
        difference_trace: false,
    }
}

#[test]
fn comparison_records_threshold_and_cross_correlation_alignment_parameters() {
    let threshold_candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![-1.0, 1.0, 3.0]);
    let threshold_baseline = source(2, 29, vec![10.0, 11.0, 12.0], vec![-2.0, 2.0, 4.0]);
    let threshold_receipt = execute_comparison(
        &threshold_candidate,
        &threshold_candidate.analyses[0],
        &threshold_baseline,
        options(ComparisonAlignmentDraft::FirstThresholdCrossing),
    )
    .expect("threshold alignment must execute")
    .receipt;
    assert!(matches!(
        threshold_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::FirstThresholdCrossing {
            signal_key,
            threshold: 0.0,
            baseline_crossing: 10.5,
            candidate_crossing: 0.5,
        } if signal_key == "signal:0"
    ));
    assert_eq!(
        threshold_receipt.policy.execution.resampling,
        ComparisonResamplingPolicy::BaselineOntoCandidateGrid
    );

    let correlation_candidate = source(
        1,
        17,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        vec![0.0, 0.0, 1.0, 0.0, -1.0, 0.0],
    );
    let correlation_baseline = source(
        2,
        29,
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
    );
    let correlation_receipt = execute_comparison(
        &correlation_candidate,
        &correlation_candidate.analyses[0],
        &correlation_baseline,
        options(ComparisonAlignmentDraft::CrossCorrelation),
    )
    .expect("correlation alignment must execute")
    .receipt;
    assert!(matches!(
        correlation_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::CrossCorrelation {
            selected_lag_samples: 1,
            sample_interval: 1.0,
            baseline_shift: 1.0,
            ..
        }
    ));
}

#[test]
fn comparison_fails_closed_for_nonmonotonic_source_coordinates() {
    let candidate = source(1, 17, vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]);
    let baseline = source(2, 29, vec![0.0, 1.0, 0.5], vec![0.0, 1.0, 2.0]);
    let error = execute_comparison(
        &candidate,
        &candidate.analyses[0],
        &baseline,
        options(ComparisonAlignmentDraft::AbsoluteXAxis),
    )
    .expect_err("nonmonotonic immutable data must never be resampled");
    assert!(error.contains("nonmonotonic"));
}
