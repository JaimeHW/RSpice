//! Inspectors for qualified frequency-response and ordinary-noise results.

use crate::presentation::frequency_readout;
use egui::Ui;
use rspice_results::bode::AcBodeMetrics;
use rspice_results::engine_evidence::{
    QpacAnalysisResult, QpxfAnalysisResult, QpxfGroupDelay, QpxfOutput,
};
use rspice_results::waveform::HeldColumn;
use rspice_ui_kit::panels::{panel_inset, section_header};
use rspice_ui_kit::plot::fmt_column_reading;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// The selected response and the evidence its inspector may display.
pub enum BodeInspector<'a> {
    Response(Box<BodeResponse<'a>>),
    NoResponse,
    AnalysisFailed(Option<&'a str>),
    Qpac(Option<&'a QpacAnalysisResult>),
    Qpxf(Option<&'a QpxfAnalysisResult>),
    Distortion,
}

/// One measured frequency response.
///
/// A stability analysis's loop gain reads as the loop's margins; any other
/// response is a response, whose phase margin is no stability margin of
/// anything — a closed-loop node voltage crossing unity says nothing about
/// the loop that drives it — so it reads as gain and bandwidth instead.
pub struct BodeResponse<'a> {
    pub metrics: AcBodeMetrics,
    /// Whether the response retained a phase trace.
    pub phase_available: bool,
    /// The retained response measured, as the run names it.
    pub signal: &'a str,
    /// Whether the response is a loop gain, which alone has margins.
    pub loop_gain: bool,
    /// Every response of the analysis the card can measure, the measured one
    /// among them. With two or more the card offers the choice.
    pub candidates: &'a [String],
    /// Every member's response, when the analysis is one of a family: the
    /// quantity the members step, which heads their column, and each one's
    /// figures, a row each.
    pub family: Option<(&'a str, &'a [MemberResponse])>,
}

/// One member of a family's responses, as the card's table reads it.
#[derive(Debug, Clone)]
pub struct MemberResponse {
    /// The member's value in the quantity its family steps, `1n`, or its
    /// label when no one quantity names it.
    pub name: String,
    pub metrics: AcBodeMetrics,
}

/// Summary facts of the selected retained ordinary-noise spectrum for the
/// right panel's card. The spectrum itself renders through the waves
/// pane-stack's nV/√Hz projection.
pub struct NoiseSpectrumModel {
    pub frequency: HeldColumn,
    pub total_rms: Option<f64>,
    /// What the output-referred density integrates to: `V rms`, or `A rms`
    /// for a current.
    pub total_rms_unit: &'static str,
    /// Why there is no output figure, when there is none.
    pub total_absent: Option<&'static str>,
    pub input_rms: Option<f64>,
    pub input_rms_unit: &'static str,
    /// Why there is no input-referred figure, when there is none.
    pub input_absent: Option<&'static str>,
    pub band: Option<(f64, f64)>,
    /// An edge's jitter, for a periodic noise run sampled at an edge's
    /// timing: its density is the edge's timing in s²/Hz, not a voltage.
    pub jitter: Option<EdgeJitter>,
}

/// An edge's jitter from its timing density over the band: the rms of the
/// edge's deviation, and Jc and Jcc where the sweep reaches half the carrier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeJitter {
    pub rms_s: Option<f64>,
    pub cycle: Option<rspice_results::phase_noise::CycleJitter>,
}

/// Render the inspector for the response qualified by the host document,
/// its readouts at the reader's `significant_digits`. Returns the index in
/// [`BodeResponse::candidates`] of a response the reader chose to measure
/// instead.
pub fn right_panel(
    ui: &mut Ui,
    model: BodeInspector<'_>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Option<usize> {
    match model {
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
                        .filter(|d| !matches!(d, QpxfGroupDelay::Finite(_)))
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
            None
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
            None
        }
        // The distortion sheet's whole panel was one implementation sentence.
        BodeInspector::Distortion => None,
        BodeInspector::Response(response) => {
            response_card(ui, *response, quantity_policy, significant_digits)
        }
        BodeInspector::NoResponse => {
            section_header(ui, "Response", None);
            crate::presentation::panel_note(ui, "No AC response in this run.");
            None
        }
        BodeInspector::AnalysisFailed(reason) => {
            section_header(ui, "Response", None);
            crate::presentation::panel_note(
                ui,
                &match reason {
                    Some(reason) => format!(
                        "The selected frequency response did not converge, so no margins are reported: {reason}"
                    ),
                    None => "The selected frequency response did not converge, so no margins are reported.".to_owned(),
                },
            );
            None
        }
    }
}

/// The card of one measured response: the loop's margins for a loop gain,
/// gain and bandwidth for anything else, under the response it measures.
fn response_card(
    ui: &mut Ui,
    response: BodeResponse<'_>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Option<usize> {
    let BodeResponse {
        metrics,
        phase_available,
        signal,
        loop_gain,
        candidates,
        family,
    } = response;
    section_header(ui, if loop_gain { "Stability" } else { "Response" }, None);
    let measured = if loop_gain { "Loop gain" } else { "Response" };
    let picked = (candidates.len() > 1)
        .then(|| measured_chooser(ui, measured, signal, candidates))
        .flatten();
    let mut rows = if loop_gain {
        stability_rows(signal, metrics, quantity_policy, significant_digits)
    } else {
        response_rows(signal, metrics, quantity_policy, significant_digits)
    };
    // The chooser above names the response already.
    if candidates.len() > 1 {
        rows.remove(0);
    }
    // A family's figures are a row per member under the response they
    // measure, not the first member's alone.
    let family = family.filter(|(_, members)| !loop_gain && members.len() > 1);
    if family.is_some() {
        rows.truncate(usize::from(candidates.len() <= 1));
    }
    crate::presentation::stat_table(ui, &rows);
    if let Some((quantity, members)) = family {
        member_table(ui, quantity, members, quantity_policy, significant_digits);
    }

    if !loop_gain {
        crate::presentation::panel_note(
            ui,
            "Stability margins are read from a loop gain: run a stability (.STB) analysis.",
        );
        if !metrics.adc_is_dc {
            crate::presentation::panel_note(
                ui,
                "Sweep starts above the dominant pole; lower fstart for DC gain.",
            );
        }
        return picked;
    }

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
    picked
}

/// A family's response figures, one row per member: its value in the
/// quantity the family steps, its gain at the bottom of the sweep, its
/// bandwidth and where its gain crosses unity.
fn member_table(
    ui: &mut Ui,
    quantity: &str,
    members: &[MemberResponse],
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    use rspice_ui_kit::widgets::table::{Align, Cell, DataTable, SortKey};
    let adc_is_dc = members.iter().all(|member| member.metrics.adc_is_dc);
    let (low, bandwidth) = low_frequency_labels(adc_is_dc);
    let frequency = |value: Option<f64>| {
        let text = value.map_or_else(
            || NOT_REACHED.to_owned(),
            |hertz| frequency_readout(quantity_policy, hertz, significant_digits),
        );
        Cell::number(text).sort_by(SortKey::Number(value.unwrap_or(f64::INFINITY)))
    };
    let rows: Vec<Vec<Cell>> = members
        .iter()
        .enumerate()
        .map(|(index, member)| {
            let m = member.metrics;
            vec![
                Cell::number(member.name.clone()).sort_by(SortKey::Number(index as f64)),
                Cell::number(or_dash(m.adc_db.map(|decibels| {
                    fmt_column_reading(decibels, "dB", significant_digits)
                })))
                .sort_by(SortKey::Number(m.adc_db.unwrap_or(f64::NEG_INFINITY))),
                frequency(m.f3db),
                frequency(m.ugf),
            ]
        })
        .collect();
    let mut columns = crate::fft::view::columns::fitted_to(
        ui,
        &[
            (quantity, Align::Left, true),
            (low, Align::Right, true),
            ("\u{2212}3 dB", Align::Right, true),
            ("Unity gain", Align::Right, true),
        ],
        &rows,
    );
    if let [_, _, bandwidth_column, unity] = columns.as_mut_slice() {
        *bandwidth_column = bandwidth_column.clone().tooltip(bandwidth);
        *unity = unity.clone().tooltip("Unity-gain frequency");
    }
    crate::presentation::with_table_state(ui, "bode-family", |ui, state| {
        DataTable::new("rspice.results.bode.family", &columns, rows.len())
            .state(state)
            .show(ui, |index| rows[index].clone())
    });
}

/// The choice of which response the card measures: a label, then a select
/// the panel's width. Returns the index of a response the reader picked.
fn measured_chooser(
    ui: &mut Ui,
    label: &str,
    signal: &str,
    candidates: &[String],
) -> Option<usize> {
    let inset = panel_inset(ui);
    let dim = Tokens::get(ui.ctx()).color.text_dim;
    ui.horizontal(|ui| {
        ui.add_space(inset);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(dim),
        );
    });
    let picked = ui
        .horizontal(|ui| {
            ui.add_space(inset);
            let width = (ui.available_width() - inset).max(80.0);
            rspice_ui_kit::widgets::select(
                ui,
                "rspice.results.bode.measured",
                label,
                signal,
                candidates,
                width,
            )
        })
        .inner;
    ui.add_space(5.0);
    picked.filter(|index| candidates.get(*index).is_some_and(|name| name != signal))
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
fn qpxf_output_label(output: &QpxfOutput) -> String {
    match output {
        QpxfOutput::Voltage { positive, negative } => {
            if negative.is_empty() || negative == "0" {
                format!("V({positive})")
            } else {
                format!("V({positive},{negative})")
            }
        }
        QpxfOutput::BranchCurrent { branch } => format!("I({branch})"),
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

/// How the lowest swept frequency's gain is labelled.
///
/// The label is a claim about the sweep, not decoration: `gain_db.first()`
/// is the gain at `f_min`, and calling it `A_dc` on a sweep opened above the
/// dominant pole reports mid-rolloff gain as DC gain — and puts the −3 dB
/// point 3 dB below a figure that was never the DC gain either. So the
/// label says which quantity it is, and the bandwidth names the same
/// reference.
fn low_frequency_labels(adc_is_dc: bool) -> (&'static str, &'static str) {
    if adc_is_dc {
        ("A_dc", "\u{2212}3 dB bandwidth")
    } else {
        ("A(f_min)", "\u{2212}3 dB re A(f_min)")
    }
}

/// A row's value, or a dash for a figure the sweep holds nothing to read
/// from.
fn or_dash(value: Option<String>) -> String {
    value.unwrap_or_else(|| "\u{2014}".to_owned())
}

/// A frequency the response crosses at, or that it does not cross within
/// the sweep.
const NOT_REACHED: &str = "Not reached";

/// What a figure the card cannot read reads in its row; a note under the
/// table says why.
const NOT_AVAILABLE: &str = "Not available";

/// The stability card's rows: the loop gain measured, then each margin and
/// the frequency it was found at, as the Nyquist panel prints them, then the
/// loop's gain at the bottom of the sweep.
fn stability_rows(
    signal: &str,
    m: AcBodeMetrics,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<(&'static str, String, bool)> {
    let (low, _) = low_frequency_labels(m.adc_is_dc);
    let margin = |value: Option<f64>, hertz: Option<f64>| {
        value
            .zip(hertz)
            .map(|(value, hertz)| super::margins::Margin { value, hertz })
    };
    let mut rows = vec![("Loop gain", signal.to_owned(), false)];
    rows.extend(super::margins::rows(
        margin(m.pm_deg, m.ugf),
        margin(m.gm_db, m.f180),
        quantity_policy,
        significant_digits,
    ));
    rows.push((
        low,
        or_dash(
            m.adc_db
                .map(|decibels| fmt_column_reading(decibels, "dB", significant_digits)),
        ),
        false,
    ));
    rows
}

/// The response card's rows: the response measured, its gain at the bottom
/// of the sweep, its bandwidth, and where its gain crosses unity.
fn response_rows(
    signal: &str,
    m: AcBodeMetrics,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<(&'static str, String, bool)> {
    let (low, bandwidth) = low_frequency_labels(m.adc_is_dc);
    let frequency = |value: Option<f64>| {
        value.map_or_else(
            || NOT_REACHED.to_owned(),
            |hertz| frequency_readout(quantity_policy, hertz, significant_digits),
        )
    };
    vec![
        ("Response", signal.to_owned(), false),
        (
            low,
            or_dash(
                m.adc_db
                    .map(|decibels| fmt_column_reading(decibels, "dB", significant_digits)),
            ),
            true,
        ),
        (bandwidth, frequency(m.f3db), true),
        ("Unity-gain freq", frequency(m.ugf), false),
    ]
}

/// The figures the response card shows, worded and formatted as it shows
/// them, without the line naming the response: what a printed figure of
/// the response carries under its plot, where its key names the response.
pub fn card_figures(
    signal: &str,
    metrics: AcBodeMetrics,
    loop_gain: bool,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<(&'static str, String)> {
    let rows = if loop_gain {
        stability_rows(signal, metrics, quantity_policy, significant_digits)
    } else {
        response_rows(signal, metrics, quantity_policy, significant_digits)
    };
    rows.into_iter()
        .skip(1)
        .map(|(label, value, _)| (label, value))
        .collect()
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
    let figure = |value: Option<f64>, unit: &str| {
        (
            value.map_or_else(
                || NOT_AVAILABLE.to_owned(),
                |value| fmt_column_reading(value, unit, significant_digits),
            ),
            value.is_some(),
        )
    };
    let mut rows = vec![(
        "Band",
        format!(
            "{} – {}",
            frequency_readout(quantity_policy, band.0, significant_digits),
            frequency_readout(quantity_policy, band.1, significant_digits)
        ),
        false,
    )];
    match model.jitter {
        Some(jitter) => {
            let (text, strong) = figure(jitter.rms_s, "s rms");
            rows.push(("Jitter", text, strong));
            if let Some(cycle) = jitter.cycle {
                let (text, _) = figure(Some(cycle.cycle_s), "s rms");
                rows.push(("Jc", text, false));
                let (text, _) = figure(Some(cycle.cycle_to_cycle_s), "s rms");
                rows.push(("Jcc", text, false));
            }
        }
        None => {
            let (text, strong) = figure(model.total_rms, model.total_rms_unit);
            rows.push(("Output integrated", text, strong));
        }
    }
    let (text, strong) = figure(model.input_rms, model.input_rms_unit);
    rows.push(("Input referred", text, strong));
    crate::presentation::stat_table(ui, &rows);
    let output_absent = model.total_absent.filter(|_| model.jitter.is_none());
    for why in output_absent.into_iter().chain(model.input_absent) {
        crate::presentation::panel_note(ui, why);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sweep that opens above the dominant pole has no DC gain in it. The
    /// card must not label mid-rolloff gain `A_dc`, and the bandwidth has to
    /// name the reference it was actually measured from.
    #[test]
    fn the_low_frequency_gain_is_labelled_by_what_the_sweep_proves() {
        let margins = AcBodeMetrics {
            adc_db: Some(20.0),
            adc_is_dc: false,
            ugf: Some(1.0e4),
            f3db: Some(1.0e2),
            ..Default::default()
        };
        let policy = QuantityPresentationPolicy::default();

        let labels = |adc_is_dc| {
            response_rows(
                "V(out)",
                AcBodeMetrics {
                    adc_is_dc,
                    ..margins
                },
                &policy,
                4,
            )
            .into_iter()
            .map(|(label, _, _)| label)
            .collect::<Vec<_>>()
        };

        assert_eq!(labels(true)[1..3], ["A_dc", "\u{2212}3 dB bandwidth"]);
        assert_eq!(
            labels(false)[1..3],
            ["A(f_min)", "\u{2212}3 dB re A(f_min)"]
        );
    }

    /// Only a loop gain has stability margins: a response card states gain
    /// and bandwidth and no margin, and each card names what it measured.
    #[test]
    fn only_a_loop_gain_reads_as_margins_and_each_card_names_its_response() {
        let metrics = AcBodeMetrics {
            adc_db: Some(12.04),
            adc_is_dc: true,
            ugf: Some(1_232.8),
            pm_deg: Some(27.144_6),
            pm_phase_deg: Some(-152.855_4),
            f180: Some(1_732.05),
            gm_db: Some(6.020_6),
            f3db: Some(510.0),
            ..Default::default()
        };
        let policy = QuantityPresentationPolicy::default();

        let response = response_rows("V(out)", metrics, &policy, 4);
        assert_eq!(response[0], ("Response", "V(out)".to_owned(), false));
        assert!(
            response
                .iter()
                .all(|(label, _, _)| !label.contains("margin")),
            "{response:?}"
        );

        let stability = stability_rows("Loop Gain", metrics, &policy, 4);
        let rows = stability
            .iter()
            .map(|(label, value, _)| (*label, value.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            rows,
            [
                ("Loop gain", "Loop Gain"),
                ("Phase margin", "27.14\u{b0}"),
                ("Gain crossover", "1.233 kHz"),
                ("Gain margin", "6.02 dB"),
                ("Phase crossover", "1.732 kHz"),
                ("A_dc", "12.04 dB"),
            ]
        );
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
        let values = |digits| {
            stability_rows("Loop Gain", margins, &policy, digits)
                .into_iter()
                .map(|(_, value, _)| value)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            values(4)[1..],
            [
                "61.23\u{b0}",
                "1.235 MHz",
                "12.35 dB",
                "4.568 MHz",
                "62.35 dB"
            ]
        );
        // A degree reads to the hundredth at any digits; a frequency keeps
        // the reader's.
        assert_eq!(values(6)[1..3], ["61.23\u{b0}", "1.23457 MHz"]);
    }
}
