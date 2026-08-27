//! BODE derivations — the active run's frequency-response stability numbers (unity-gain,
//! phase and gain margins) for the right panel's inspector card, plus the
//! ordinary-noise spectrum instrument. The Bode sheet itself renders through
//! the waves pane-stack; margins and curves read the same data by
//! construction.

use std::sync::Arc;

use egui::Ui;

use crate::quantity::QuantityPresentationPolicy;
use crate::state::{
    AnalysisResult, AnalysisType, SharedWaveformValues, ac_bode_summary_for_selection,
};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::BodeDerived;

/// The selected frequency-response signal pair's computed stability numbers.
struct BodeModel {
    /// The phase trace as *displayed*: the retained ±180°-wrapped samples, or
    /// the unwrapped series when the continuous toggle is on. This is a
    /// presentation choice only — the margins are always measured on the
    /// unwrapped branch, whichever trace is painted.
    phase_deg: Option<SharedWaveformValues>,
    margins: BodeDerived,
    /// Whether the sweep proves its lowest-frequency gain is the DC gain.
    /// The card's labels depend on it: an unproven claim of DC gain is a
    /// wrong reading of a right number.
    adc_is_dc: bool,
}

/// Summary facts of the selected retained ordinary-noise spectrum for the
/// right panel's card. The spectrum itself renders through the waves
/// pane-stack's nV/√Hz projection.
struct NoiseSpectrumModel {
    frequency: SharedWaveformValues,
    trace_count: usize,
    total_rms: Option<f64>,
    input_rms: Option<f64>,
    band: Option<(f64, f64)>,
}

fn noise_waveform_is_renderable(waveform: &crate::state::WaveformData) -> bool {
    if waveform.x.len() != waveform.y.len() || waveform.x.len() < 2 {
        return false;
    }
    if waveform
        .y
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return false;
    }
    let mut previous = None;
    let mut positive_count = 0_usize;
    for frequency in waveform.x.iter().copied() {
        if !frequency.is_finite() || frequency <= 0.0 {
            return false;
        }
        if previous.is_some_and(|previous| frequency <= previous) {
            return false;
        }
        previous = Some(frequency);
        positive_count += 1;
    }
    positive_count >= 2
}

pub(super) fn ordinary_noise_spectrum_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && matches!(
            analysis.analysis_type,
            AnalysisType::Noise | AnalysisType::Hbnoise
        )
        && analysis.waveforms.iter().any(|waveform| {
            (is_input_noise_name(&waveform.name) || is_output_noise_name(&waveform.name))
                && noise_waveform_is_renderable(waveform)
        })
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
    let run = simulation.active_run().ok_or(NoMargins::NoResponse)?;
    let summary = ac_bode_summary_for_selection(run, simulation.active_analysis_idx)
        .ok_or(NoMargins::NoResponse)?;

    // Fail closed, as every sibling sheet does. The summary resolves which
    // exact analysis it read, so the gate names that one rather than the
    // ordinal selection.
    let analysis = run
        .analyses
        .get(summary.analysis_index)
        .ok_or(NoMargins::NoResponse)?;
    if !analysis.success {
        return Err(NoMargins::AnalysisFailed(analysis.error_message.clone()));
    }

    let adc_is_dc = summary.metrics.adc_is_dc;
    let phase = summary
        .phase_index
        .zip(summary.phase_deg.as_ref())
        .map(|(phase_index, phase)| (phase_index, Arc::clone(phase)));

    // Margins + extremes from the curves, cached on (data version, resolved
    // magnitude waveform) — the crossings and folds are O(points) and both
    // panels read them every frame.
    let version = simulation.data_version;
    let margins = match state.ui.results.bode {
        Some(d)
            if d.version == version
                && d.analysis_index == summary.analysis_index
                && d.mag_index == summary.mag_index =>
        {
            d
        }
        _ => {
            let metrics = summary.metrics;
            let d = BodeDerived {
                version,
                analysis_index: summary.analysis_index,
                mag_index: summary.mag_index,
                adc_db: metrics.adc_db,
                ugf: metrics.ugf,
                pm_deg: metrics.pm_deg,
                f180: metrics.f180,
                gm_db: metrics.gm_db,
                f3db: metrics.f3db,
            };
            state.ui.results.bode = Some(d);
            d
        }
    };

    // Displayed phase: optionally unwrapped into a continuous curve. This
    // toggle moves the painted trace and nothing else — the margins above
    // are measured on the unwrapped branch either way.
    let phase_deg = match &phase {
        Some((phase_index, raw)) if state.ui.results.phase_continuous => {
            let key = (summary.analysis_index as u64) << 32 | *phase_index as u64;
            Some(state.ui.results.derived.unwrapped(key, raw))
        }
        Some((_, raw)) => Some(Arc::clone(raw)),
        None => None,
    };

    Ok(BodeModel {
        phase_deg,
        margins,
        adc_is_dc,
    })
}

fn normalized_noise_name(name: &str) -> String {
    name.trim()
        .to_ascii_lowercase()
        .replace([' ', '-', '.'], "")
}

fn is_input_noise_name(name: &str) -> bool {
    let name = normalized_noise_name(name);
    name == "inoise"
        || name == "inoise_spectrum"
        || name == "inoisespectrum"
        || name == "v(inoise)"
        || name == "v(inoise_spectrum)"
        || name == "v(inoisespectrum)"
}

fn is_output_noise_name(name: &str) -> bool {
    let name = normalized_noise_name(name);
    name == "onoise"
        || name == "onoise_spectrum"
        || name == "onoisespectrum"
        || name == "v(onoise)"
        || name == "v(onoise_spectrum)"
        || name == "v(onoisespectrum)"
}

fn is_noise_contributor_name(name: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    name.starts_with("noise(") && name.ends_with(')')
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
    if let Some(selected) = state.simulation.active_analysis_idx
        && let Some(analysis) = run.analyses.get(selected)
        && is_noise_analysis(analysis.analysis_type)
    {
        return ordinary_noise_spectrum_is_renderable(analysis).then_some(selected);
    }
    run.analyses
        .iter()
        .position(ordinary_noise_spectrum_is_renderable)
}

fn selected_noise_analysis(state: &AppState) -> Option<(usize, &AnalysisResult)> {
    let run = state.simulation.active_run()?;
    let selected = selected_noise_analysis_index(state)?;
    Some((selected, &run.analyses[selected]))
}

fn build_noise_model(state: &AppState) -> Option<NoiseSpectrumModel> {
    let (_, analysis) = selected_noise_analysis(state)?;
    let input_referred = analysis.waveforms.iter().find(|waveform| {
        is_input_noise_name(&waveform.name) && noise_waveform_is_renderable(waveform)
    });
    let anchor = match input_referred {
        Some(waveform) => waveform,
        None => analysis.waveforms.iter().find(|waveform| {
            is_output_noise_name(&waveform.name) && noise_waveform_is_renderable(waveform)
        })?,
    };

    let frequency = Arc::clone(&anchor.x);
    let trace_count = if input_referred.is_some() {
        1
    } else {
        analysis
            .waveforms
            .iter()
            .filter(|waveform| {
                !is_input_noise_name(&waveform.name)
                    && (is_output_noise_name(&waveform.name)
                        || is_noise_contributor_name(&waveform.name))
                    && noise_waveform_is_renderable(waveform)
                    && waveform.x.as_slice() == frequency.as_slice()
            })
            .count()
    };
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
        band,
    })
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    if state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| analysis.analysis_type.is_raw_frequency_curve())
    {
        section_header(ui, "Distortion curves", None);
        super::panel_note(
            ui,
            "Fundamental response and Volterra product ratios are retained as exact complex phasors; the plot projects their magnitude to dB and dBc without changing zero into a finite floor.",
        );
        return;
    }
    section_header(ui, "Stability", None);
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let model = match build_model(state) {
        Ok(model) => model,
        Err(NoMargins::NoResponse) => {
            super::panel_note(ui, "No usable frequency response in the active run.");
            return;
        }
        Err(NoMargins::AnalysisFailed(reason)) => {
            super::panel_note(
                ui,
                &match reason {
                    Some(reason) => format!(
                        "The selected frequency response did not converge, so no margins are reported: {reason}"
                    ),
                    None => "The selected frequency response did not converge, so no margins are reported. Its retained vectors are what the engine emitted before it stopped, not a measured response.".to_owned(),
                },
            );
            return;
        }
    };
    let rows = margin_rows(model.margins, model.adc_is_dc, &quantity_policy);
    super::stat_table(ui, &rows);

    if model.phase_deg.is_none() {
        super::panel_note(
            ui,
            "Phase data unavailable for this response — re-run the analysis to compute margins.",
        );
    } else if model.adc_is_dc {
        super::panel_note(
            ui,
            "Margins measured on the simulated curves; the plot markers show the same values.",
        );
    } else {
        super::panel_note(
            ui,
            "Margins measured on the simulated curves; the plot markers show the same values. The sweep does not open flat, so the gain shown is the one at its lowest frequency — not the DC gain — and f₋₃dB is referenced to that.",
        );
    }
}

/// The stability card's rows.
///
/// Split out because the low-frequency labels are a claim about the sweep,
/// not decoration: `gain_db.first()` is the gain at `f_min`, and calling it
/// `A_dc` on a sweep opened above the dominant pole reports mid-rolloff gain
/// as DC gain — and puts `f₋₃dB` 3 dB below a figure that was never the DC
/// gain either. The label says which quantity it is, and `f₋₃dB` names the
/// same reference.
fn margin_rows(
    m: BodeDerived,
    adc_is_dc: bool,
    quantity_policy: &QuantityPresentationPolicy,
) -> [(&'static str, String, bool); 6] {
    let fmt_opt =
        |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String { v.map_or("—".to_owned(), f) };
    [
        (
            "Phase margin",
            fmt_opt(m.pm_deg, &|v| {
                quantity_policy.format_angle(v.to_radians(), 1)
            }),
            true,
        ),
        (
            "Gain margin",
            fmt_opt(m.gm_db, &|v| format!("{v:.1} dB")),
            true,
        ),
        (
            "Unity-gain freq",
            fmt_opt(m.ugf, &|v| quantity_policy.format_frequency(v, 1)),
            false,
        ),
        (
            "f₁₈₀",
            fmt_opt(m.f180, &|v| quantity_policy.format_frequency(v, 0)),
            false,
        ),
        (
            if adc_is_dc { "A_dc" } else { "A(f_min)" },
            fmt_opt(m.adc_db, &|v| format!("{v:.1} dB")),
            false,
        ),
        (
            if adc_is_dc {
                "f₋₃dB"
            } else {
                "f₋₃dB re A(f_min)"
            },
            fmt_opt(m.f3db, &|v| quantity_policy.format_frequency(v, 0)),
            false,
        ),
    ]
}

pub(super) fn noise_spectrum_right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Noise spectrum", None);
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_noise_model(state) else {
        super::panel_note(ui, "No valid ordinary noise spectrum is selected.");
        return;
    };
    let band = model.band.unwrap_or_else(|| {
        (
            model.frequency.first().copied().unwrap_or_default(),
            model.frequency.last().copied().unwrap_or_default(),
        )
    });
    let rows = [
        (
            "Band",
            format!(
                "{} – {}",
                quantity_policy.format_frequency(band.0, 2),
                quantity_policy.format_frequency(band.1, 2)
            ),
            false,
        ),
        ("Traces", model.trace_count.to_string(), false),
        (
            "Output integrated",
            model
                .total_rms
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.6e} V rms")),
            model.total_rms.is_some(),
        ),
        (
            "Input referred",
            model
                .input_rms
                .map_or_else(|| "—".to_owned(), |value| format!("{value:.6e} V rms")),
            model.input_rms.is_some(),
        ),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        "The retained source vectors are power spectral density (V²/Hz). The plot applies the exact square-root amplitude-density conversion and displays nV/√Hz without altering source samples.",
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
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert!(state.simulation.select_analysis(0));
        let first = build_model(&mut state).expect("first model");
        assert_eq!(first.margins.analysis_index, 0);
        assert_eq!(first.margins.adc_db, Some(20.0));

        assert!(state.simulation.select_analysis(1));
        let second = build_model(&mut state).expect("second model");
        assert_eq!(second.margins.analysis_index, 1);
        assert_eq!(second.margins.adc_db, Some(40.0));
    }

    fn noise_result(analysis_type: AnalysisType, label: &str, name: &str) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, label).with_waveforms(vec![WaveformData::new(
            name,
            vec![1.0, 10.0],
            vec![1.0e-18, 1.0e-16],
            "#fff",
        )])
    }

    /// A sweep that opens above the dominant pole has no DC gain in it. The
    /// card must not label mid-rolloff gain `A_dc`, and `f₋₃dB` has to name
    /// the reference it was actually measured from.
    #[test]
    fn the_low_frequency_gain_is_labelled_by_what_the_sweep_proves() {
        let margins = BodeDerived {
            version: 0,
            analysis_index: 0,
            mag_index: 0,
            adc_db: Some(20.0),
            ugf: Some(1.0e4),
            pm_deg: Some(45.0),
            f180: Some(3.0e4),
            gm_db: Some(12.0),
            f3db: Some(1.0e2),
        };
        let policy = QuantityPresentationPolicy::default();

        let labels =
            |adc_is_dc| margin_rows(margins, adc_is_dc, &policy).map(|(label, _, _)| label);

        assert_eq!(labels(true)[4], "A_dc");
        assert_eq!(labels(true)[5], "f₋₃dB");
        assert_eq!(labels(false)[4], "A(f_min)");
        assert_eq!(labels(false)[5], "f₋₃dB re A(f_min)");
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
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        let model = build_model(&mut state).expect("Bode model");

        assert!(!model.adc_is_dc);
        let policy = QuantityPresentationPolicy::default();
        assert_eq!(
            margin_rows(model.margins, model.adc_is_dc, &policy)[4].0,
            "A(f_min)"
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
        state.simulation.runs = vec![run];
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
        state.simulation.runs = vec![run];
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

        state.simulation.active_analysis_idx = None;
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
        state.simulation.runs = vec![run];
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
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        let model = build_noise_model(&state).expect("input-referred noise model");
        // Input-referred evidence takes the card without mixing in output
        // or contributor traces.
        assert_eq!(model.trace_count, 1);
    }

    #[test]
    fn noise_spectrum_rejects_nonpositive_or_nonmonotonic_retained_samples() {
        let invalid_value =
            AnalysisResult::new(1, AnalysisType::Noise, "zero").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 0.0], "#fff"),
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
    fn hbnoise_psd_uses_the_noise_density_instrument() {
        let hbnoise =
            AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE").with_waveforms(vec![
                WaveformData::new("onoise", vec![1.0e3, 1.0e4], vec![1.0e-18, 2.0e-18], "#fff"),
            ]);
        assert!(ordinary_noise_spectrum_is_renderable(&hbnoise));
    }
}
