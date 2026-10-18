//! Inspectors for qualified frequency-response and ordinary-noise results.

use crate::presentation::{angle_readout, frequency_readout, plain_readout};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_core::engine::{QpacAnalysisResult, QpxfAnalysisResult};
use rspice_results::bode::AcBodeMetrics;
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::panels::section_header;

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
    pub total_rms: Option<f64>,
    pub input_rms: Option<f64>,
    pub input_rms_unit: &'static str,
    pub band: Option<(f64, f64)>,
}

/// Render the inspector for the response qualified by the host document,
/// its readouts at the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    model: BodeInspector<'_>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let (metrics, phase_available) = match model {
        BodeInspector::Qpxf(response) => {
            section_header(ui, "QPXF unit transfers", None);
            if let Some(response) = response {
                let m = &response.metadata;
                let mut rows = vec![
                    (
                        "Output",
                        format!(
                            "{} at {}",
                            qpxf_output_label(&m.request.output),
                            tuple_label(&m.request.output_lattice)
                        ),
                        true,
                    ),
                    (
                        "Tones",
                        frequency_list(&m.grid.frequencies_hz, quantity_policy, significant_digits),
                        false,
                    ),
                    (
                        "Sources",
                        m.input_sources
                            .iter()
                            .map(|source| source.name.clone())
                            .collect::<Vec<_>>()
                            .join(", "),
                        false,
                    ),
                ];
                if m.request.group_delay {
                    let undefined = response
                        .transfers
                        .iter()
                        .filter_map(|t| t.group_delay.as_ref())
                        .flatten()
                        .filter(|d| !matches!(d, rspice_core::engine::QpxfGroupDelay::Finite(_)))
                        .count();
                    if undefined > 0 {
                        rows.push((
                            "Group delay",
                            format!("{undefined} points undefined"),
                            false,
                        ));
                    }
                }
                crate::presentation::stat_table(ui, &rows);
            }
            return;
        }
        BodeInspector::Qpac(response) => {
            section_header(ui, "QPAC conversion response", None);
            if let Some(response) = response {
                let m = &response.metadata;
                let rows = [
                    ("Source", m.request.input_source.clone(), true),
                    (
                        "Sideband in → out",
                        format!(
                            "{} → {}",
                            tuple_label(&m.request.input_lattice),
                            tuple_label(&m.request.output_lattice)
                        ),
                        false,
                    ),
                    (
                        "Tones",
                        frequency_list(&m.tone_frequencies_hz, quantity_policy, significant_digits),
                        false,
                    ),
                ];
                crate::presentation::stat_table(ui, &rows);
            }
            return;
        }
        // The distortion sheet's whole panel was one implementation sentence.
        BodeInspector::Distortion => return,
        BodeInspector::Stability {
            metrics,
            phase_available,
        } => (metrics, phase_available),
        BodeInspector::NoResponse => {
            section_header(ui, "Stability", None);
            crate::presentation::panel_note(ui, "No AC response in this run.");
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
                    None => "The selected frequency response did not converge, so no margins are reported.".to_owned(),
                },
            );
            return;
        }
    };
    section_header(ui, "Stability", None);
    let rows = margin_rows(metrics, quantity_policy, significant_digits);
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
    } else if !metrics.adc_is_dc {
        crate::presentation::panel_note(
            ui,
            "Sweep starts above the dominant pole; lower fstart for DC gain.",
        );
    }
}

/// A signed tone tuple as the reader writes it: `(1, −1)`.
fn tuple_label(tuple: &[i32]) -> String {
    format!(
        "({})",
        tuple
            .iter()
            .map(|k| if *k < 0 {
                format!("−{}", k.unsigned_abs())
            } else {
                k.to_string()
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// A QPXF output as a signal accessor, never the Rust enum behind it.
fn qpxf_output_label(output: &rspice_core::engine::QpxfOutput) -> String {
    match output {
        rspice_core::engine::QpxfOutput::Voltage { positive, negative } => {
            if negative.is_empty() || negative == "0" {
                format!("V({positive})")
            } else {
                format!("V({positive},{negative})")
            }
        }
        rspice_core::engine::QpxfOutput::BranchCurrent { branch } => format!("I({branch})"),
    }
}

/// A tone list in the reader's frequency units.
fn frequency_list(
    frequencies: &[f64],
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> String {
    frequencies
        .iter()
        .map(|frequency| frequency_readout(quantity_policy, *frequency, significant_digits))
        .collect::<Vec<_>>()
        .join(", ")
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
    significant_digits: usize,
) -> [(&'static str, String, bool); 6] {
    let adc_is_dc = m.adc_is_dc;
    let fmt_opt =
        |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String { v.map_or("—".to_owned(), f) };
    let frequency = |v: f64| frequency_readout(quantity_policy, v, significant_digits);
    let decibels = |v: f64| plain_readout(v, "dB", significant_digits);
    [
        (
            "Phase margin",
            fmt_opt(m.pm_deg, &|v| {
                angle_readout(quantity_policy, v.to_radians(), significant_digits)
            }),
            true,
        ),
        ("Gain margin", fmt_opt(m.gm_db, &decibels), true),
        ("Unity-gain freq", fmt_opt(m.ugf, &frequency), false),
        ("f₁₈₀", fmt_opt(m.f180, &frequency), false),
        (
            if adc_is_dc { "A_dc" } else { "A(f_min)" },
            fmt_opt(m.adc_db, &decibels),
            false,
        ),
        (
            if adc_is_dc {
                "f₋₃dB"
            } else {
                "f₋₃dB re A(f_min)"
            },
            fmt_opt(m.f3db, &frequency),
            false,
        ),
    ]
}

/// Render the summary of the selected, validated ordinary-noise spectrum,
/// its readouts at the reader's `significant_digits`.
pub fn noise_spectrum_right_panel(
    ui: &mut Ui,
    model: Option<&NoiseSpectrumModel>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    section_header(ui, "Noise spectrum", None);
    let Some(model) = model else {
        crate::presentation::panel_note(ui, "No noise spectrum in this run.");
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
                frequency_readout(quantity_policy, band.0, significant_digits),
                frequency_readout(quantity_policy, band.1, significant_digits)
            ),
            false,
        ),
        (
            "Output integrated",
            model.total_rms.map_or_else(
                || "—".to_owned(),
                |value| rspice_ui_kit::plot::fmt_si_readout(value, "V rms", significant_digits),
            ),
            model.total_rms.is_some(),
        ),
        (
            "Input referred",
            model.input_rms.map_or_else(
                || "—".to_owned(),
                |value| {
                    rspice_ui_kit::plot::fmt_si_readout(
                        value,
                        model.input_rms_unit,
                        significant_digits,
                    )
                },
            ),
            model.input_rms.is_some(),
        ),
    ];
    crate::presentation::stat_table(ui, &rows);
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
                4,
            )
            .map(|(label, _, _)| label)
        };

        assert_eq!(labels(true)[4], "A_dc");
        assert_eq!(labels(true)[5], "f₋₃dB");
        assert_eq!(labels(false)[4], "A(f_min)");
        assert_eq!(labels(false)[5], "f₋₃dB re A(f_min)");
    }

    /// The stability card reads at the reader's significant digits, not at a
    /// decimal count fixed per row.
    #[test]
    fn the_margins_read_at_the_chosen_significant_digits() {
        let margins = AcBodeMetrics {
            adc_db: Some(62.345_678),
            adc_is_dc: true,
            ugf: Some(1.234_567_8e6),
            pm_deg: Some(61.234_567),
            pm_phase_deg: Some(-118.765_433),
            f180: Some(4.567_891e6),
            gm_db: Some(12.345_678),
            f3db: Some(123.456_78),
            ..Default::default()
        };
        let policy = QuantityPresentationPolicy::default();
        let values = |digits| margin_rows(margins, &policy, digits).map(|(_, value, _)| value);
        assert_eq!(
            values(4),
            [
                "61.23 °",
                "12.35 dB",
                "1.235 MHz",
                "4.568 MHz",
                "62.35 dB",
                "123.5 Hz"
            ]
        );
        assert_eq!(values(6)[0], "61.2346 °");
        assert_eq!(values(6)[2], "1.23457 MHz");
    }
}
