//! Application integration fixtures for runtime results and retained documents.

mod qpac;
pub(crate) use qpac::qpac_retained_test_fixture;
mod qpnoise;
pub(crate) use qpnoise::qpnoise_retained_test_fixture;
mod qpxf;
pub(crate) use qpxf::qpxf_retained_test_fixture;

pub(crate) fn retained_manual_fixture(
    source: &str,
    kind: crate::state::AnalysisType,
) -> crate::state::AnalysisResult {
    let run = super::controller::test_execution::run_manual_batch(source);
    let mut analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == kind)
        .expect("checked batch contains the requested fixture analysis")
        .clone();
    // Display and legacy-import fixtures are detached from this checked run.
    // They retain numerical evidence but must not claim its run identity.
    analysis.provenance = None;
    analysis
}
