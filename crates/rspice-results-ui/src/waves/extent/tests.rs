//! Independent panes retain their geometry within one model generation.
use super::*;
use crate::waves::ProjectionOptions;
use crate::{derived::DerivedSeries, waveform::WaveformData};
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive,
};
use rspice_ui_kit::tokens::Tokens;

#[test]
fn each_pane_of_a_two_unit_strip_keeps_its_own_envelope() {
    let mut run = rspice_results::run::SimulationRun::new(
        1,
        0.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0).with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0], "#fff"),
            WaveformData::new(
                "I(R1)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0e-3, 2.0e-3],
                "#0af",
            ),
        ]),
    );
    let models = super::super::build_models(
        &[&run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            selection: None,
            hidden_family_traces: &HashSet::new(),
        },
        |_| None,
    );
    let mut cache = FamilyEnvelopeCache::default();
    let model = &models[0];
    let panes = model.unit_panes();
    assert_eq!(
        panes.len(),
        2,
        "the fixture must produce a two-unit strip: {:?}",
        panes.iter().map(|pane| pane.unit).collect::<Vec<_>>()
    );

    let generation = 3;
    let first: Vec<_> = panes
        .iter()
        .map(|pane| family_envelopes(&mut cache, generation, model, pane))
        .collect();

    let baseline = ExtentWork::reset();
    for _ in 0..5 {
        for (index, pane) in panes.iter().enumerate() {
            let again = family_envelopes(&mut cache, generation, model, pane);
            assert!(
                std::sync::Arc::ptr_eq(&first[index], &again),
                "pane {index} ({}) was rebuilt by a sibling pane's question",
                pane.unit
            );
        }
    }
    assert_eq!(
        ExtentWork::read().envelopes - baseline.envelopes,
        0,
        "the panes of one strip evicted each other's envelopes"
    );

    // A new generation of models still empties the cache: the envelope is a
    // memo of the models, never of the pane identity alone.
    let next = family_envelopes(&mut cache, generation + 1, model, &panes[0]);
    assert!(!std::sync::Arc::ptr_eq(&first[0], &next));
    assert_eq!(
        cache.entry_count(),
        1,
        "the previous generation's envelopes were kept alongside the new one"
    );
}
