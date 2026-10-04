//! BODE derivations — the active run's frequency-response stability numbers (unity-gain,
//! phase and gain margins) for the right panel's inspector card, plus the
//! ordinary-noise spectrum instrument. The Bode sheet itself renders through
//! the waves pane-stack; margins and curves read the same data by
//! construction.

use std::sync::Arc;

use egui::Ui;

use crate::state::{AnalysisResult, AnalysisType, ac_bode_shape_for_selection};
use crate::workbench::AppState;
use rspice_results_ui::bode::inspector::{self, BodeInspector, NoiseSpectrumModel};

use super::BodeDerived;

/// The selected frequency-response signal pair's computed stability numbers.
struct BodeModel {
    /// Whether the selected response retained a phase trace.
    phase_available: bool,
    /// The measured response, including whether the sweep proves its
    /// lowest-frequency gain is the DC gain — the card's labels depend on it,
    /// and an unproven claim of DC gain is a wrong reading of a right number.
    margins: BodeDerived,
}

pub(super) use rspice_results::noise_spectrum::NoiseSpectrumShape;

fn resolve_noise_spectrum_shape(analysis: &AnalysisResult) -> Option<NoiseSpectrumShape> {
    rspice_results::noise_spectrum::resolve_noise_spectrum_shape(&analysis.waveforms, || {
        super::frame_work::note(super::frame_work::DatasetWalk::NoiseSpectrumScan);
    })
}

/// The same answer, resolved once per dataset generation.
///
/// Its memo checks the retained history revision on every read. A tab-strip
/// query before frame preparation cannot reuse the shape of an older source.
pub(super) fn noise_spectrum_shape(
    state: &AppState,
    dataset_id: crate::product::DatasetId,
    analysis: &AnalysisResult,
) -> Option<NoiseSpectrumShape> {
    let key = super::AnalysisPresentationKey::new(dataset_id, analysis);
    state
        .ui
        .results
        .noise_spectrum_shapes
        .get_or_insert_with(&state.simulation, key, || {
            resolve_noise_spectrum_shape(analysis)
        })
}

/// Whether one analysis holds an ordinary-noise spectrum this workspace can
/// draw, through the memo that owns the question.
pub(super) fn ordinary_noise_spectrum_is_renderable_in(
    state: &AppState,
    dataset_id: crate::product::DatasetId,
    analysis: &AnalysisResult,
) -> bool {
    is_ordinary_noise_result(analysis)
        && noise_spectrum_shape(state, dataset_id, analysis).is_some()
}

/// The analysis kinds an ordinary-noise spectrum can come from, and the
/// requirement that the solve behind it completed. Free of the dataset, so
/// it is the cheap half of every gate below.
fn is_ordinary_noise_result(analysis: &AnalysisResult) -> bool {
    rspice_results::noise_spectrum::is_ordinary_noise_result(
        analysis.success,
        analysis.analysis_type,
    )
}

/// The same question without a session to memoize against, for the printed
/// page and the visualization document, which resolve a run they hold
/// directly rather than the one the workspace has open.
pub(super) fn ordinary_noise_spectrum_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::noise_spectrum::ordinary_noise_spectrum_is_renderable(
        analysis.success,
        analysis.analysis_type,
        &analysis.waveforms,
        || super::frame_work::note(super::frame_work::DatasetWalk::NoiseSpectrumScan),
    )
}

/// Why the stability card has no numbers to show.
#[derive(Debug)]
enum NoMargins {
    /// The active run holds no frequency response this sheet can read.
    NoResponse,
    /// A response is selected, but the solve behind it failed. Its retained
    /// vectors are whatever the engine emitted before giving up, and margins
    /// read off them are not measurements. The engine's own reason travels
    /// with the refusal when it gave one.
    AnalysisFailed(Option<String>),
}

fn build_model(state: &mut AppState) -> Result<BodeModel, NoMargins> {
    let simulation = &state.simulation;
    if simulation.active_analysis().is_some_and(|a| {
        matches!(
            a.analysis_type,
            crate::state::AnalysisType::Qpac | crate::state::AnalysisType::Qpxf
        )
    }) {
        return Err(NoMargins::NoResponse);
    }
    let run = simulation.active_run().ok_or(NoMargins::NoResponse)?;
    // Which traces, not what they measure. Resolving the summary here — and
    // only then consulting the memo below — meant the memo saved nothing: the
    // conversion and every crossing search had already happened by the time
    // its key was known.
    let shape = ac_bode_shape_for_selection(run, simulation.view.active_analysis_idx)
        .ok_or(NoMargins::NoResponse)?;

    // Fail closed, as every sibling sheet does. The shape resolves which
    // exact analysis it read, so the gate names that one rather than the
    // ordinal selection.
    let analysis = run
        .analyses
        .get(shape.analysis_index)
        .ok_or(NoMargins::NoResponse)?;
    if !analysis.success {
        return Err(NoMargins::AnalysisFailed(analysis.error_message.clone()));
    }

    let phase_available = shape
        .phase_index
        .is_some_and(|index| analysis.waveforms.get(index).is_some());

    // Margins + extremes from the curves, cached on (data version, resolved
    // magnitude waveform) — the crossings and folds are O(points) and both
    // panels read them every frame.
    let version = simulation.view.data_version;
    let margins = match state.ui.results.bode {
        Some(d)
            if d.version == version
                && d.analysis_index == shape.analysis_index
                && d.mag_index == shape.mag_index =>
        {
            d
        }
        _ => {
            super::frame_work::note(super::frame_work::DatasetWalk::BodeMargins);
            let metrics =
                crate::state::ac_bode_summary_for_analysis(analysis, shape.analysis_index)
                    .ok_or(NoMargins::NoResponse)?
                    .metrics;
            let d = BodeDerived {
                version,
                analysis_index: shape.analysis_index,
                mag_index: shape.mag_index,
                metrics,
            };
            state.ui.results.bode = Some(d);
            d
        }
    };

    Ok(BodeModel {
        phase_available,
        margins,
    })
}

/// Analyses whose selection states which noise result the reader means. A
/// selection outside this family — a transient carried over from another
/// viewer — says nothing about noise at all.
fn is_noise_analysis(analysis_type: AnalysisType) -> bool {
    matches!(
        analysis_type,
        AnalysisType::Noise | AnalysisType::Pnoise | AnalysisType::Hbnoise | AnalysisType::Qpnoise
    )
}

/// The one noise analysis both halves of the ordinary-noise sheet read.
///
/// A selected noise analysis binds strictly. If it carries no renderable
/// ordinary spectrum — a phase-noise result, or one whose solve failed — the
/// card is empty and says why; quietly substituting a neighbouring result
/// would put a different analysis's contributors under the reader's
/// selection. The run-wide fallback applies only when the selection expresses
/// no noise intent for the binding to honour.
pub(super) fn selected_noise_analysis_index(state: &AppState) -> Option<usize> {
    let run = state.simulation.active_run()?;
    selected_noise_analysis_index_with(state.simulation.view.active_analysis_idx, run, |analysis| {
        ordinary_noise_spectrum_is_renderable_in(state, run.dataset_id, analysis)
    })
}

/// The same binding, from the selection and the run rather than the session.
///
/// The printed page resolves the analysis it prints from a run and a selection
/// it already holds, and it must reach the same one the sheet did — a page
/// that steps to a neighbouring result puts another analysis' contributors
/// under the selected one's name, on paper.
pub(super) fn selected_noise_analysis_index_in(
    globally_selected: Option<usize>,
    run: &crate::state::SimulationRun,
) -> Option<usize> {
    selected_noise_analysis_index_with(
        globally_selected,
        run,
        ordinary_noise_spectrum_is_renderable,
    )
}

/// The binding rule itself, stated once.
///
/// The sheet resolves renderability through the workspace memo and the
/// printed page resolves it directly, but they must reach the same analysis:
/// a page that steps to a neighbouring result puts another analysis'
/// contributors under the selected one's name, on paper. So the rule takes
/// the oracle as a parameter rather than being written out twice.
fn selected_noise_analysis_index_with(
    globally_selected: Option<usize>,
    run: &crate::state::SimulationRun,
    renderable: impl Fn(&AnalysisResult) -> bool,
) -> Option<usize> {
    if let Some(selected) = globally_selected
        && let Some(analysis) = run.analyses.get(selected)
        && is_noise_analysis(analysis.analysis_type)
    {
        return renderable(analysis).then_some(selected);
    }
    run.analyses.iter().position(renderable)
}

fn selected_noise_analysis(state: &AppState) -> Option<(usize, &AnalysisResult)> {
    let run = state.simulation.active_run()?;
    let selected = selected_noise_analysis_index(state)?;
    Some((selected, &run.analyses[selected]))
}

fn build_noise_model(state: &AppState) -> Option<NoiseSpectrumModel> {
    let run = state.simulation.active_run()?;
    let (_, analysis) = selected_noise_analysis(state)?;
    let shape = noise_spectrum_shape(state, run.dataset_id, analysis)?;
    let frequency = Arc::clone(&analysis.waveforms.get(shape.anchor)?.x);
    let trace_count = shape.trace_count;
    let (total_rms, input_rms, band) = analysis
        .noise_summary
        .as_ref()
        .map_or((None, None, None), |summary| {
            (summary.total_rms, summary.input_rms, Some(summary.band))
        });

    Some(NoiseSpectrumModel {
        frequency,
        trace_count,
        total_rms,
        input_rms,
        input_rms_unit: analysis
            .noise_summary
            .as_ref()
            .map_or("RMS (unit not retained)", |summary| {
                summary.input_rms_unit()
            }),
        band,
    })
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    if let Some(analysis) = state.simulation.active_analysis() {
        let specialized = match analysis.analysis_type {
            AnalysisType::Qpxf => Some(BodeInspector::Qpxf(match &analysis.result_payload {
                Some(crate::state::AnalysisResultPayload::Qpxf { response }) => Some(response),
                _ => None,
            })),
            AnalysisType::Qpac => Some(BodeInspector::Qpac(match &analysis.result_payload {
                Some(crate::state::AnalysisResultPayload::Qpac { response }) => Some(response),
                _ => None,
            })),
            kind if kind.is_raw_frequency_curve() => Some(BodeInspector::Distortion),
            _ => None,
        };
        if let Some(model) = specialized {
            inspector::right_panel(ui, model, &quantity_policy);
            return;
        }
    }
    let model = build_model(state);
    let inspector = match &model {
        Ok(model) => BodeInspector::Stability {
            metrics: model.margins.metrics,
            phase_available: model.phase_available,
        },
        Err(NoMargins::NoResponse) => BodeInspector::NoResponse,
        Err(NoMargins::AnalysisFailed(reason)) => BodeInspector::AnalysisFailed(reason.as_deref()),
    };
    inspector::right_panel(ui, inspector, &quantity_policy);
}

pub(super) fn noise_spectrum_right_panel(ui: &mut Ui, state: &mut AppState) {
    let model = build_noise_model(state);
    inspector::noise_spectrum_right_panel(
        ui,
        model.as_ref(),
        &state.ui.preferences.quantity_presentation_policy(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun, WaveformData,
    };

    fn ac_result(
        source_id: AnalysisInstanceId,
        signal: &str,
        magnitude: [f64; 2],
    ) -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![WaveformData::new(
                format!("|{signal}|"),
                vec![1.0, 10.0],
                magnitude.to_vec(),
                "#fff",
            )])
            .with_provenance(
                AnalysisResultProvenance::new(
                    source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x73; 32]),
                    Vec::new(),
                )
                .expect("valid AC provenance"),
            )
    }

    #[test]
    fn bode_model_follows_the_selected_same_kind_ac_instance() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let mut run = SimulationRun::new(1);
        run.add_analysis(ac_result(first_id, "V(first)", [10.0, 1.0]));
        run.add_analysis(ac_result(second_id, "V(second)", [100.0, 10.0]));

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));

        assert!(state.simulation.select_analysis(0));
        let first = build_model(&mut state).expect("first model");
        assert_eq!(first.margins.analysis_index, 0);
        assert_eq!(first.margins.metrics.adc_db, Some(20.0));

        assert!(state.simulation.select_analysis(1));
        let second = build_model(&mut state).expect("second model");
        assert_eq!(second.margins.analysis_index, 1);
        assert_eq!(second.margins.metrics.adc_db, Some(40.0));
    }

    fn noise_result(analysis_type: AnalysisType, label: &str, name: &str) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, label).with_waveforms(vec![WaveformData::new(
            name,
            vec![1.0, 10.0],
            vec![1.0e-18, 1.0e-16],
            "#fff",
        )])
    }

    /// The whole sheet, end to end: a sweep opened two decades above the
    /// dominant pole reaches the card labelled for what it is.
    #[test]
    fn a_sweep_that_starts_mid_rolloff_reaches_the_card_as_a_f_min() {
        // A single pole at 10 Hz, swept from 1 kHz: the gain has already
        // fallen 40 dB by the first sample.
        let frequency = (0..=30)
            .map(|i| 10f64.powf(3.0 + i as f64 / 10.0))
            .collect::<Vec<_>>();
        let magnitude = frequency
            .iter()
            .map(|f| 1000.0 / (1.0 + (f / 10.0).powi(2)).sqrt())
            .collect::<Vec<_>>();
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
                "|V(out)|", frequency, magnitude, "#fff",
            )]),
        );

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        let model = build_model(&mut state).expect("Bode model");

        assert!(!model.margins.metrics.adc_is_dc);
        fn collect(shape: &egui::Shape, text: &mut Vec<String>) {
            match shape {
                egui::Shape::Text(value) => text.push(value.galley.text().to_owned()),
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, text)),
                _ => {}
            }
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| right_panel(ui, &mut state));
            },
        );
        let mut text = Vec::new();
        for shape in output.shapes {
            collect(&shape.shape, &mut text);
        }
        assert!(text.iter().any(|label| label == "A(f_min)"), "{text:?}");
        assert!(
            text.iter().any(|label| label == "f₋₃dB re A(f_min)"),
            "{text:?}"
        );
    }

    /// A diverged AC run still carries whatever partial vectors the engine
    /// emitted before it gave up. Margins read off those vectors are not
    /// measurements, and the sibling sheets all refuse them.
    #[test]
    fn margins_are_withheld_when_the_selected_ac_run_failed() {
        let mut failed = ac_result(AnalysisInstanceId::new(), "V(out)", [10.0, 1.0]);
        failed.success = false;
        failed.error_message = Some("AC analysis did not converge".to_owned());
        let mut run = SimulationRun::new(1);
        run.add_analysis(failed);

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        match build_model(&mut state) {
            Err(NoMargins::AnalysisFailed(reason)) => {
                assert_eq!(reason.as_deref(), Some("AC analysis did not converge"));
            }
            Err(NoMargins::NoResponse) => panic!("the response exists; only its solve failed"),
            Ok(_) => panic!("a diverged AC run must not produce margins"),
        }
    }

    /// A failed noise run is not a renderable spectrum, exactly as a failed
    /// harmonic-balance or phase-noise run is not.
    #[test]
    fn a_failed_noise_analysis_is_not_a_renderable_ordinary_spectrum() {
        let mut failed = noise_result(AnalysisType::Noise, "NOISE", "onoise");
        failed.success = false;
        failed.error_message = Some("noise analysis did not converge".to_owned());

        assert!(!ordinary_noise_spectrum_is_renderable(&failed));
    }

    /// The contributor table and the spectrum card both bind through
    /// `selected_noise_analysis_index`. When the reader has selected a noise
    /// analysis that carries no ordinary spectrum, substituting a different
    /// analysis puts another run's contributors under the selection.
    #[test]
    fn a_selected_noise_analysis_is_never_replaced_by_a_different_one() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(noise_result(AnalysisType::Pnoise, "PNOISE", "phase_noise"));
        run.add_analysis(noise_result(AnalysisType::Noise, "NOISE", "onoise"));
        run.add_analysis(AnalysisResult::new(3, AnalysisType::Transient, "TRAN"));

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));

        // The selection is a noise analysis with no ordinary spectrum: the
        // card is empty, not filled from the neighbouring NOISE result.
        assert!(state.simulation.select_analysis(0));
        assert_eq!(selected_noise_analysis_index(&state), None);

        // The selection is the ordinary-noise result itself.
        assert!(state.simulation.select_analysis(1));
        assert_eq!(selected_noise_analysis_index(&state), Some(1));

        // The selection carries no noise intent at all, so the run's own
        // spectrum is the only available answer and is used.
        assert!(state.simulation.select_analysis(2));
        assert_eq!(selected_noise_analysis_index(&state), Some(1));

        state.simulation.view.active_analysis_idx = None;
        assert_eq!(selected_noise_analysis_index(&state), Some(1));
    }

    #[test]
    fn phase_noise_data_is_never_relabelled_as_an_ordinary_spectrum() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE").with_waveforms(vec![
                WaveformData::new("phase_noise", vec![1.0, 10.0], vec![-90.0, -110.0], "#fff"),
            ]),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Noise, "NOISE").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 1.0e-16], "#fff"),
            ]),
        );

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));

        // The selected PNOISE analysis holds no ordinary spectrum. Its own
        // phase-noise trace must not be relabelled as one, and neither may
        // the neighbouring NOISE result be substituted for it.
        assert!(state.simulation.select_analysis(0));
        assert!(build_noise_model(&state).is_none());

        // Selecting the ordinary-noise result gives the card its evidence.
        // The nV/√Hz conversion itself is pinned by the waves pane-stack
        // projection tests.
        assert!(state.simulation.select_analysis(1));
        let model = build_noise_model(&state).expect("ordinary noise spectrum");
        assert_eq!(model.frequency.as_slice(), &[1.0, 10.0]);
        assert_eq!(model.trace_count, 1);
    }

    #[test]
    fn noise_model_prefers_retained_input_referred_spectrum_without_mixing_references() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Noise, "NOISE").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0, 10.0], vec![4.0e-18, 9.0e-18], "#fff"),
                WaveformData::new(
                    "inoise_spectrum",
                    vec![1.0, 10.0],
                    vec![16.0e-18, 25.0e-18],
                    "#fff",
                ),
                WaveformData::new(
                    "noise(R1:thermal)",
                    vec![1.0, 10.0],
                    vec![1.0e-18, 2.25e-18],
                    "#fff",
                ),
            ]),
        );

        let mut state = AppState::default();
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        let model = build_noise_model(&state).expect("input-referred noise model");
        // Input-referred evidence takes the card without mixing in output
        // or contributor traces.
        assert_eq!(model.trace_count, 1);
    }

    #[test]
    fn noise_spectrum_rejects_negative_or_nonmonotonic_retained_samples() {
        let invalid_value =
            AnalysisResult::new(1, AnalysisType::Noise, "negative").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, -1.0e-9], "#fff"),
            ]);
        let invalid_axis =
            AnalysisResult::new(2, AnalysisType::Noise, "axis").with_waveforms(vec![
                WaveformData::new("inoise", vec![10.0, 1.0], vec![1.0e-9, 2.0e-9], "#fff"),
            ]);
        assert!(!ordinary_noise_spectrum_is_renderable(&invalid_value));
        assert!(!ordinary_noise_spectrum_is_renderable(&invalid_axis));
    }

    #[test]
    fn contributor_only_noise_data_never_impersonates_a_total_reference_spectrum() {
        let contributor_only = AnalysisResult::new(1, AnalysisType::Noise, "contributors")
            .with_waveforms(vec![WaveformData::new(
                "noise(R1:thermal)",
                vec![1.0, 10.0],
                vec![1.0e-18, 2.0e-18],
                "#fff",
            )]);
        assert!(!ordinary_noise_spectrum_is_renderable(&contributor_only));
    }

    #[test]
    fn hbnoise_spot_spectrum_keeps_its_result_inspector_accessible() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Hbnoise, "spot HBNOISE").with_waveforms(vec![
                WaveformData::new("onoise", vec![1e3], vec![1e-18], "#fff"),
            ]);
        assert!(ordinary_noise_spectrum_is_renderable(&analysis));
        let shape = resolve_noise_spectrum_shape(&analysis).unwrap();
        assert_eq!(shape.anchor, 0);
        assert_eq!(shape.trace_count, 1);
    }

    #[test]
    fn hbnoise_psd_uses_the_noise_density_instrument() {
        let hbnoise =
            AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0e3, 1.0e4], vec![1.0e-18, 2.0e-18], "#fff"),
            ]);
        assert!(ordinary_noise_spectrum_is_renderable(&hbnoise));
    }
}
