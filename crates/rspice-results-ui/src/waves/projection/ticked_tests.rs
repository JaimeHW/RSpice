//! A strip model holds the curves the reader has on the plot and no others: an
//! analysis of a million signals with three ticked builds three traces, and the
//! data browser stays the catalogue of the rest.

use super::*;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_results::run::{ExecutionTarget, SimulationRun};

/// A transient of `signals` node voltages, none of which the dataset plots,
/// with `V(n0)` plotted by the dataset itself.
fn wide_transient(signals: usize) -> SimulationRun<AnalysisResult<RetainedTrace>> {
    let time = vec![0.0, 1.0e-9, 2.0e-9];
    let waveforms = (0..signals)
        .map(|index| {
            let mut trace = RetainedTrace::new(
                format!("V(n{index})"),
                time.clone(),
                vec![index as f64; 3],
                "#fff",
            )
            .with_unit("V");
            trace.visible = index == 0;
            trace
        })
        .collect();
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(waveforms),
    );
    run
}

/// The traces the strip model of `run` holds with `ticks`, built once with
/// the analysis's signals indexed and once walking every column, which agree.
fn traces_of(
    run: &SimulationRun<AnalysisResult<RetainedTrace>>,
    ticks: &HashMap<SourceWaveformPresentationKey, bool>,
) -> Vec<String> {
    let indices = crate::signal_index::SignalIndices::default();
    let revision = rspice_vocabulary::source_revision::SourceRevision::default();
    let built = |signals| {
        let models = super::super::build_models(
            &[run],
            &ExecutedDeckArchive::default(),
            &mut DerivedSeries::default(),
            &Tokens::default(),
            super::super::ProjectionOptions {
                phase_continuous: false,
                complex_display: Default::default(),
                ac: Default::default(),
                selection: None,
                hidden_family_traces: &HashSet::new(),
                waveform_visibility: ticks,
                signals,
            },
            |_| None,
        );
        assert_eq!(models.len(), 1, "one transient builds one strip model");
        models[0]
            .traces
            .iter()
            .map(|trace| trace.source_waveform_name.clone())
            .collect::<Vec<_>>()
    };
    let indexed = built(Some(super::super::SignalLookup {
        indices: &indices,
        revision: &revision,
    }));
    assert_eq!(indexed, built(None), "an index changes nothing on the plot");
    indexed
}

#[test]
fn a_strip_model_built_from_a_large_analysis_holds_only_the_ticked_traces() {
    let run = wide_transient(100_000);
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let tick = |name: &str, on: bool| (SourceWaveformPresentationKey::new(analysis, name), on);

    // With no ticks, the dataset's own choice is what is on the plot.
    assert_eq!(traces_of(&run, &HashMap::new()), ["V(n0)"]);

    // Three ticked signals and the dataset's own, in the analysis' column order.
    let ticks: HashMap<_, _> = [
        tick("V(n99999)", true),
        tick("V(n5)", true),
        tick("V(n4242)", true),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        traces_of(&run, &ticks),
        ["V(n0)", "V(n5)", "V(n4242)", "V(n99999)"]
    );

    // A signal the reader unticks is off the plot, the dataset's own too.
    let ticks: HashMap<_, _> = [tick("V(n0)", false), tick("V(n7)", true)]
        .into_iter()
        .collect();
    assert_eq!(traces_of(&run, &ticks), ["V(n7)"]);
}
