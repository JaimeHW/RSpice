//! Tests proving corrupt numerical evidence is quarantined without losing result identity.

use super::*;
use crate::state::AnalysisType;

#[test]
fn corrupted_waveform_keeps_identity_but_quarantines_every_exact_value_route() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.25, f64::NAN], "#00aaff"),
        ]),
    );
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let waveform = SourceWaveformPresentationKey::new(analysis, "V(out)");
    let selection = ResultBrowserSelectionKey::Waveform(waveform.clone());

    assert!(result_signal_stable_path(&waveform, &[run.clone()]).is_ok());
    assert!(
        validate_result_browser_selection_evidence(&selection, &[run.clone()])
            .expect_err("corrupted values must be quarantined")
            .contains("quarantined")
    );
    assert!(
        exact_result_signal_last_sample(&waveform, &[run.clone()])
            .expect_err("last-sample copy must fail closed")
            .contains("quarantined")
    );
    assert!(
        exact_result_browser_selection_bundle(&[selection], &[run])
            .expect_err("batch export must fail closed")
            .contains("quarantined")
    );
}
