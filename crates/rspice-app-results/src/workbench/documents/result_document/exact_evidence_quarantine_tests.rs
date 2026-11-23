//! Tests proving corrupt numerical evidence is quarantined without losing result identity.

use super::*;
use crate::state::AnalysisType;

#[test]
fn corrupted_waveform_keeps_identity_but_quarantines_every_exact_value_route() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.25, f64::NAN], "#00aaff"),
        ]),
    );
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let waveform = SourceWaveformPresentationKey::new(analysis, "V(out)");
    let selection = ResultBrowserSelectionKey::Waveform(waveform.clone());

    assert!(result_signal_stable_path(&waveform, &[run.clone()]).is_ok());
    let mut host = ResultsTestHost::default();
    host.model.simulation.retained.runs.push(run.clone());
    let reads = host.results_reads();
    assert!(
        result_browser_selection_evidence(&selection, reads.results, reads.model)
            .expect_err("corrupted values must be quarantined")
            .contains("cannot be copied or exported")
    );
    assert!(
        exact_result_signal_last_sample(&waveform, &[run.clone()], Default::default())
            .expect_err("last-sample copy must fail closed")
            .contains("cannot be copied or exported")
    );
    assert!(
        exact_result_browser_selection_bundle(&[selection], &[run])
            .expect_err("batch export must fail closed")
            .contains("cannot be copied or exported")
    );
}

/// The ticked quantities the browser's batch actions act on are resolved in
/// one pass, and each answers as it does resolved on its own: its stable path
/// gives its place, a name two signals share resolves to neither, and a
/// corrupted analysis refuses exact values.
#[test]
fn a_selection_resolved_at_once_answers_as_each_quantity_resolved_alone() {
    let trace = |name: &str, last: f64| {
        RetainedTrace::new(name, vec![0.0, 1.0], vec![0.0, last], "#00aaff")
    };
    let mut clean = SimulationRun::new(1);
    clean.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            trace("V(b)", 1.0),
            trace("V(a)", 1.0),
            trace("V(twice)", 1.0),
            trace("V(twice)", 2.0),
        ]),
    );
    let mut corrupted = SimulationRun::new(2);
    corrupted.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![trace("V(out)", f64::NAN)]),
    );
    let runs = vec![clean, corrupted];
    let keys = [
        ("V(b)", 0),
        ("V(a)", 0),
        ("V(twice)", 0),
        ("V(gone)", 0),
        ("V(out)", 1),
    ]
    .into_iter()
    .map(|(name, run)| {
        let analysis = AnalysisPresentationKey::new(runs[run].dataset_id, &runs[run].analyses[0]);
        ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(analysis, name))
    })
    .collect::<std::collections::HashSet<_>>();
    let mut host = ResultsTestHost::default();
    for run in &runs {
        host.model.simulation.retained.runs.push(run.clone());
    }
    let reads = host.results_reads();

    let indices = rspice_results_ui::signal_index::SignalIndices::default();
    let resolved = resolve_result_browser_selection(
        &keys,
        &runs,
        (&indices, &host.model.simulation.retained.runs.revision()),
    );

    let mut alone = keys.iter().cloned().collect::<Vec<_>>();
    alone.sort_by_key(|key| result_browser_selection_stable_path(key, &runs).unwrap_or_default());
    assert_eq!(
        resolved
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>(),
        alone
    );
    for (key, signal) in &resolved {
        let waveform = key.waveform().expect("every key is a signal's");
        assert_eq!(
            signal.map(|(.., trace)| trace.name.clone()),
            waveform
                .resolve(&runs)
                .map(|(.., trace)| trace.name.clone()),
            "{key:?}"
        );
        assert_eq!(
            resolved_selection_evidence(key, *signal, reads.results, reads.model),
            result_browser_selection_evidence(key, reads.results, reads.model),
            "{key:?}"
        );
    }
}
