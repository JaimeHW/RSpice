//! Application run selection and visible-trace preference for retained Bode results.

use super::{AnalysisResult, SimulationRun};
#[cfg(test)]
use super::{AnalysisType, WaveformData};
use crate::product::AnalysisInstanceId;
pub use rspice_results::bode::retained::{AcBodeShape, AcBodeSummary};

/// The run's normal frequency response: the first one carrying traces.
fn first_response(run: &SimulationRun) -> Option<(usize, &AnalysisResult)> {
    run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis.analysis_type.is_bode_response() && !analysis.waveforms.is_empty()
    })
}

/// The response produced by one exact prepared analysis instance.
///
/// Analysis kind and display label are deliberately not used as identity:
/// both can be identical when a run contains multiple AC configurations.
fn response_by_source_instance(
    run: &SimulationRun,
    source_instance_id: AnalysisInstanceId,
) -> Option<(usize, &AnalysisResult)> {
    run.analyses.iter().enumerate().find(|(_, analysis)| {
        analysis
            .provenance
            .as_ref()
            .is_some_and(|provenance| provenance.source_instance_id() == source_instance_id)
    })
}

/// The response a result browser's selection names.
///
/// A selected AC result with current provenance is re-resolved through its
/// stable prepared-instance identity. Legacy results, which predate that
/// identity, remain addressable by their run-local index. When the current
/// selection is not a frequency response, the run's normal response fallback
/// is retained.
fn selected_response(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<(usize, &AnalysisResult)> {
    let Some(analysis_index) = selected_analysis_index else {
        return first_response(run);
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return first_response(run);
    };
    if !analysis.analysis_type.is_bode_response() {
        return first_response(run);
    }
    match analysis.provenance.as_ref() {
        Some(provenance) => response_by_source_instance(run, provenance.source_instance_id()),
        None => Some((analysis_index, analysis)),
    }
}

/// Resolve the frequency response selected in a result browser; see
/// [`selected_response`].
pub fn ac_bode_summary_for_selection(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<AcBodeSummary> {
    let (analysis_index, analysis) = selected_response(run, selected_analysis_index)?;
    ac_bode_summary_for_analysis(analysis, analysis_index)
}

/// The shape of the response a result browser's selection names, without
/// measuring it; see [`AcBodeShape`].
pub fn ac_bode_shape_for_selection(
    run: &SimulationRun,
    selected_analysis_index: Option<usize>,
) -> Option<AcBodeShape> {
    let (analysis_index, analysis) = selected_response(run, selected_analysis_index)?;
    ac_bode_shape_for_analysis(analysis, analysis_index)
}

/// Which of one analysis' retained traces form a frequency response.
///
/// No sample is read: the answer is the magnitude trace this module would
/// measure and the phase trace named after it, if the run retained one.
pub fn ac_bode_shape_for_analysis(
    analysis: &AnalysisResult,
    analysis_index: usize,
) -> Option<AcBodeShape> {
    rspice_results::bode::retained::ac_bode_shape_for_analysis(
        analysis.analysis_type,
        &analysis.waveforms,
        analysis_index,
        |waveform| waveform.visible,
    )
}

pub fn ac_bode_summary_for_analysis(
    analysis: &AnalysisResult,
    analysis_index: usize,
) -> Option<AcBodeSummary> {
    rspice_results::bode::retained::ac_bode_summary_for_analysis(
        analysis.analysis_type,
        &analysis.waveforms,
        analysis_index,
        |waveform| waveform.visible,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};

    fn ac_analysis(waveforms: Vec<WaveformData>) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(waveforms)
    }

    fn wave(name: &str, x: &[f64], y: &[f64], visible: bool) -> WaveformData {
        let mut waveform = WaveformData::new(name, x.to_vec(), y.to_vec(), "#fff");
        waveform.visible = visible;
        waveform
    }

    #[test]
    fn ac_summary_prefers_visible_magnitude_trace() {
        let frequency = [1.0, 10.0];
        let hidden = [1.0, 1.0];
        let visible = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(in)|", &frequency, &hidden, false),
            wave("|V(out)|", &frequency, &visible, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(out)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn ac_summary_uses_last_matching_magnitude_when_visibility_ties() {
        let frequency = [1.0, 10.0];
        let first = [1.0, 1.0];
        let last = [10.0, 1.0];
        let analysis = ac_analysis(vec![
            wave("|V(first)|", &frequency, &first, true),
            wave("|V(last)|", &frequency, &last, true),
        ]);

        let summary = ac_bode_summary_for_analysis(&analysis, 0).expect("AC summary");

        assert_eq!(summary.signal, "V(last)");
        assert_eq!(summary.mag_index, 1);
    }

    #[test]
    fn an_unbound_selection_falls_back_to_the_first_ac_analysis_with_waveforms() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "OP"));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        let summary = ac_bode_summary_for_selection(&run, None).expect("AC summary");

        assert_eq!(summary.analysis_index, 1);
        assert_eq!(summary.signal, "V(out)");
    }

    #[test]
    fn the_fallback_does_not_skip_an_unusable_first_ac_analysis() {
        let frequency = [1.0, 10.0];
        let magnitude = [10.0, 1.0];
        let mut run = SimulationRun::new(7);
        run.add_analysis(ac_analysis(vec![wave(
            "phase(V(in))",
            &frequency,
            &magnitude,
            true,
        )]));
        run.add_analysis(ac_analysis(vec![wave(
            "|V(out)|", &frequency, &magnitude, true,
        )]));

        assert_eq!(ac_bode_summary_for_selection(&run, None), None);
    }

    #[test]
    fn ac_selection_resolves_two_same_kind_results_by_source_instance() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x42; 32]);
        let frequency = [1.0, 10.0];
        let mut run = SimulationRun::new(9);
        run.add_analysis(
            ac_analysis(vec![wave("|V(low_band)|", &frequency, &[10.0, 1.0], true)])
                .with_provenance(
                    super::super::AnalysisResultProvenance::new(
                        first_id,
                        ObjectRevision::INITIAL,
                        snapshot,
                        Vec::new(),
                    )
                    .expect("first provenance"),
                ),
        );
        run.add_analysis(
            ac_analysis(vec![wave(
                "|V(high_band)|",
                &frequency,
                &[100.0, 10.0],
                true,
            )])
            .with_provenance(
                super::super::AnalysisResultProvenance::new(
                    second_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("second provenance"),
            ),
        );

        let (first_index, first) = response_by_source_instance(&run, first_id).expect("first AC");
        let first = ac_bode_summary_for_analysis(first, first_index).expect("first summary");
        let (second_index, second) =
            response_by_source_instance(&run, second_id).expect("second AC");
        let second = ac_bode_summary_for_analysis(second, second_index).expect("second summary");
        assert_eq!(
            (first.analysis_index, first.signal.as_str()),
            (0, "V(low_band)")
        );
        assert_eq!(
            (second.analysis_index, second.signal.as_str()),
            (1, "V(high_band)")
        );

        assert_eq!(
            ac_bode_summary_for_selection(&run, Some(0))
                .expect("selected first AC")
                .signal,
            "V(low_band)"
        );
        assert_eq!(
            ac_bode_summary_for_selection(&run, Some(1))
                .expect("selected second AC")
                .signal,
            "V(high_band)"
        );
    }
}
