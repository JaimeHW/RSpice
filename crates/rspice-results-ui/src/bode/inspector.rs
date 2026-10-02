//! Inspectors for qualified frequency-response and ordinary-noise results.

use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_core::engine::{QpacAnalysisResult, QpxfAnalysisResult};
use rspice_results::bode::AcBodeMetrics;
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::widgets::section_header;

/// The selected response and the evidence its inspector may display.
pub enum BodeInspector<'a> {
    Stability {
        metrics: AcBodeMetrics,
        phase_available: bool,
    },
    NoResponse,
    AnalysisFailed(Option<&'a str>),
    Qpac(Option<&'a QpacAnalysisResult>),
    Qpxf(Option<&'a QpxfAnalysisResult>),
    Distortion,
}

/// Summary facts of the selected retained ordinary-noise spectrum for the
/// right panel's card. The spectrum itself renders through the waves
/// pane-stack's nV/√Hz projection.
pub struct NoiseSpectrumModel {
    pub frequency: SharedWaveformValues,
    pub trace_count: usize,
    pub total_rms: Option<f64>,
    pub input_rms: Option<f64>,
    pub input_rms_unit: &'static str,
    pub band: Option<(f64, f64)>,
}

/// Render the inspector for the response qualified by the host document.
pub fn right_panel(
    ui: &mut Ui,
    model: BodeInspector<'_>,
    quantity_policy: &QuantityPresentationPolicy,
) {
    let (metrics, phase_available) = match model {
        BodeInspector::Qpxf(response) => {
            section_header(ui, "QPXF unit transfers", None);
            crate::presentation::panel_note(
                ui,
                "The horizontal axis is physical output frequency. Each curve is the output per unit excitation of its named input source and signed tone tuple. Finite sampled group delays are shown in seconds; undefined intervals are omitted from the curves and retained in the CSV table.",
            );
            if let Some(response) = response {
                let m = &response.metadata;
                crate::presentation::panel_note(
                    ui,
                    &format!(
                        "{} sources × {} input tuples. Output {:?} at tuple {:?}. Authored axis: {:?}; tones {:?} Hz.",
                        m.input_sources.len(),
                        m.input_lattices.len(),
                        m.request.output,
                        m.request.output_lattice,
                        m.request.frequency_axis,
                        m.grid.frequencies_hz
                    ),
                );
                if m.request.group_delay {
                    let undefined = response
                        .transfers
                        .iter()
                        .filter_map(|t| t.group_delay.as_ref())
                        .flatten()
                        .filter(|d| !matches!(d, rspice_core::engine::QpxfGroupDelay::Finite(_)))
                        .count();
                    crate::presentation::panel_note(
                        ui,
                        &format!(
                            "Group-delay magnitude floor: {}. {} undefined delay samples. Delay uses sampled phase differences on the selected frequency grid.",
                            m.request.group_delay_magnitude_floor, undefined
                        ),
                    );
                }
            }
            return;
        }
        BodeInspector::Qpac(response) => {
            section_header(ui, "QPAC conversion response", None);
            crate::presentation::panel_note(
                ui,
                "The horizontal axis is the signed probe offset. Physical input and output frequencies equal offset plus their tone tuple dotted with the QPSS tones. Full signed-tuple responses are available in the result table and CSV export.",
            );
            if let Some(response) = response {
                let m = &response.metadata;
                crate::presentation::panel_note(
                    ui,
                    &format!(
                        "Source: {} ({:?}), magnitude {}, phase {}°. Input tuple {:?}; output tuple {:?}. Tones: {:?} Hz.",
                        m.request.input_source,
                        m.input_quantity,
                        m.request.magnitude,
                        m.request.phase_degrees,
                        m.request.input_lattice,
                        m.request.output_lattice,
                        m.tone_frequencies_hz
                    ),
                );
            }
            return;
        }
        BodeInspector::Distortion => {
            section_header(ui, "Distortion curves", None);
            crate::presentation::panel_note(
                ui,
                "Fundamental response and Volterra product ratios are retained as exact complex phasors; the plot projects their magnitude to dB and dBc without changing zero into a finite floor.",
            );
            return;
        }
        BodeInspector::Stability {
            metrics,
            phase_available,
        } => (metrics, phase_available),
        BodeInspector::NoResponse => {
            section_header(ui, "Stability", None);
            crate::presentation::panel_note(ui, "No usable frequency response in the active run.");
            return;
        }
        BodeInspector::AnalysisFailed(reason) => {
            section_header(ui, "Stability", None);
            crate::presentation::panel_note(
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
    section_header(ui, "Stability", None);
    let rows = margin_rows(metrics, quantity_policy);
    crate::presentation::stat_table(ui, &rows);

    // A folded phase margin reads like a verdict and is not one. It goes above
    // the sweep-provenance notes because it is the stronger claim: the others
    // qualify what the number is referenced to, this one says the number alone
    // cannot settle the question it looks like it answers.
    if let Some(loop_phase) = metrics.pm_phase_deg
        && rspice_results::stability::phase_margin_is_folded(loop_phase)
    {
        crate::presentation::panel_note(
            ui,
            &rspice_results::stability::folded_phase_margin_note(loop_phase),
        );
    }

    if !phase_available {
        crate::presentation::panel_note(
            ui,
            "Phase data unavailable for this response — re-run the analysis to compute margins.",
        );
    } else if metrics.adc_is_dc {
        crate::presentation::panel_note(
            ui,
            "Margins measured on the simulated curves; the plot markers show the same values.",
        );
    } else {
        crate::presentation::panel_note(
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
    m: AcBodeMetrics,
    quantity_policy: &QuantityPresentationPolicy,
) -> [(&'static str, String, bool); 6] {
    let adc_is_dc = m.adc_is_dc;
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

/// Render the summary of the selected, validated ordinary-noise spectrum.
pub fn noise_spectrum_right_panel(
    ui: &mut Ui,
    model: Option<&NoiseSpectrumModel>,
    quantity_policy: &QuantityPresentationPolicy,
) {
    section_header(ui, "Noise spectrum", None);
    let Some(model) = model else {
        crate::presentation::panel_note(ui, "No valid ordinary noise spectrum is selected.");
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
            model.input_rms.map_or_else(
                || "—".to_owned(),
                |value| format!("{value:.6e} {}", model.input_rms_unit),
            ),
            model.input_rms.is_some(),
        ),
    ];
    crate::presentation::stat_table(ui, &rows);
    crate::presentation::panel_note(
        ui,
        "The plot takes the square root of retained power spectral density and displays nV/√Hz for voltage or nA/√Hz for current, or ns/√Hz for timing, without altering source samples.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sweep that opens above the dominant pole has no DC gain in it. The
    /// card must not label mid-rolloff gain `A_dc`, and `f₋₃dB` has to name
    /// the reference it was actually measured from.
    #[test]
    fn the_low_frequency_gain_is_labelled_by_what_the_sweep_proves() {
        let margins = AcBodeMetrics {
            adc_db: Some(20.0),
            adc_is_dc: false,
            ugf: Some(1.0e4),
            pm_deg: Some(45.0),
            pm_phase_deg: Some(-135.0),
            f180: Some(3.0e4),
            gm_db: Some(12.0),
            f3db: Some(1.0e2),
            ..Default::default()
        };
        let policy = QuantityPresentationPolicy::default();

        let labels = |adc_is_dc| {
            margin_rows(
                AcBodeMetrics {
                    adc_is_dc,
                    ..margins
                },
                &policy,
            )
            .map(|(label, _, _)| label)
        };

        assert_eq!(labels(true)[4], "A_dc");
        assert_eq!(labels(true)[5], "f₋₃dB");
        assert_eq!(labels(false)[4], "A(f_min)");
        assert_eq!(labels(false)[5], "f₋₃dB re A(f_min)");
    }
}
