//! A live complex preview, read by the calculator the results viewer evaluates
//! with.

use rspice_simulation::output_contract::PreparedSavedOutput;

use crate::analysis::calculator;
use crate::product::AnalysisInstanceId;
use crate::simulation::output_contract::fixtures::{ac, output};
use crate::simulation::output_contract::materialize_live_saved_outputs;
use crate::state::{
    AnalysisResult, ComplexExpressionPolicy, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, RetainedTrace,
    SavedOutputKind, SavedOutputStreaming,
};

#[test]
fn complex_live_outputs_keep_aligned_components_and_full_precision_source_knots() {
    let mut output = output(
        SavedOutputKind::DerivedExpression,
        "Complex preview",
        "complex(V(out),V(out))",
    );
    output.streaming = SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation;
    let contract = PreparedSavedOutput::prepare(&output, AnalysisInstanceId::new(), &ac())
        .unwrap()
        .unwrap();
    let x = (0..10_000)
        .map(|index| 1e-4 + index as f64 * 1e-18)
        .collect::<Vec<_>>();
    let y = (0..x.len())
        .map(|index| 1e-120 * (index as f64 / 17.0).sin())
        .collect::<Vec<_>>();
    let source =
        AnalysisResult::live_transient_partial(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![RetainedTrace::new(
                "out",
                x.clone(),
                y.clone(),
                "#fff",
            )]);
    let previews = materialize_live_saved_outputs(&source, &[contract]);
    assert_eq!(previews.len(), 1);
    let preview = &previews[0];
    assert!(preview.x.len() <= DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
    assert!(preview.x.windows(2).all(|pair| pair[0] < pair[1]));
    let complex = preview.complex.as_ref().unwrap();
    assert_eq!(preview.x.len(), complex.real.len());
    assert_eq!(preview.x.len(), complex.imag.len());
    for (index, &point) in preview.x.iter().enumerate() {
        let source_index = x
            .binary_search_by(|candidate| candidate.total_cmp(&point))
            .unwrap();
        assert_eq!(complex.real[index], y[source_index]);
        assert_eq!(complex.imag[index], y[source_index]);
    }
    assert!(calculator::waveform_value(preview, ComplexExpressionPolicy::Rectangular).is_ok());
}
