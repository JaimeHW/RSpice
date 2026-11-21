//! Retained coefficient-spectrum presentation for HB, Fourier, recorded FFT and QPSS.
//!
//! An HB, `.FOUR`, PSS or QPSS spectrum is drawn as levels the reader
//! chooses (magnitude, dB, dBm into a stated resistance or dBc), voltages and
//! currents on strips of their own, with the phase and the waveform the lines
//! add up to on further strips when asked. A recorded `.FFT` keeps the
//! decade ordinate it is read on.

use crate::presentation::{PlotView, frequency_readout, well_hint};
use crate::strip::{self, LegendChip};
use crate::waves::ReadoutPolicy;
use egui::Ui;
use rspice_results::harmonic_spectrum::lines::CoefficientLine;
use rspice_results::harmonic_spectrum::settings::{CoefficientQuantity, CoefficientSettings};
use rspice_results::waveform::HeldColumn;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_significant};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::sync::Arc;

mod bar;
mod details;
pub mod recorded_fft;
mod sheet;

pub use bar::domain_bar;

/// One retained complex coefficient sequence.  `magnitude` is the exact
/// result-conversion magnitude, not a display-derived dB or RMS estimate.
pub struct HarmonicTrace {
    pub name: String,
    pub frequency: HeldColumn,
    pub magnitude: HeldColumn,
    /// The complex lines the magnitudes were taken from, when the trace
    /// keeps them: the phase, the levels and the waveform are read here.
    pub lines: Arc<[CoefficientLine]>,
    pub color: egui::Color32,
    pub cache_key: u64,
}

impl HarmonicTrace {
    /// What the trace's signal measures.
    #[must_use]
    pub fn quantity(&self) -> CoefficientQuantity {
        CoefficientQuantity::of(&self.name)
    }
}

/// All evidence needed by both the plot and the inspector.
pub struct HarmonicBalanceModel {
    pub label: String,
    pub traces: Vec<HarmonicTrace>,
    pub frequency_max: f64,
    pub magnitude_min: f64,
    pub magnitude_max: f64,
    pub retained_frequency_count: usize,
    /// The recorded `.FFT` evidence, when the selected analysis carries it.
    /// Absent for HB, `.FOUR` and the PSS spectrum.
    pub fft: Option<rspice_results::fft::spectrum::FftSpectrumEvidence>,
    pub qpss: Option<Arc<rspice_results::engine_evidence::QpssOperatingPoint>>,
    /// The smallest positive magnitude retained, for the decade ordinate.
    pub smallest_positive_magnitude: Option<f64>,
}

impl HarmonicBalanceModel {
    /// The two tones the analysis was solved for, when it states them: a
    /// QPSS lattice's first two independent tones.
    #[must_use]
    pub fn stated_tones(&self) -> Option<[f64; 2]> {
        let point = self.qpss.as_ref()?;
        match point.config().grid.frequencies_hz.as_slice() {
            [first, second, ..] => Some([*first, *second]),
            _ => None,
        }
    }
}

/// Why the host could not bind a drawable coefficient spectrum.
pub enum SpectrumAbsence<'a> {
    Failed(&'a str),
    IncompleteHistory(String),
    Unavailable,
}

/// Requests applied by the host to the viewports that produced this frame.
pub struct SpectrumResponse {
    pub fit: bool,
    /// Each strip's plot, by the index its viewport is kept under.
    pub plots: Vec<(usize, plot::PlotResponse)>,
}

/// Explain the selected spectrum's retained failure or incomplete history.
pub fn show_absent(ui: &mut Ui, reason: SpectrumAbsence<'_>) {
    match reason {
        SpectrumAbsence::Failed(error) => {
            well_hint(ui, &format!("Spectrum execution failed: {error}"))
        }
        SpectrumAbsence::IncompleteHistory(sentence) => well_hint(ui, &sentence),
        SpectrumAbsence::Unavailable => well_hint(
            ui,
            "No retained complex coefficient spectrum for the selected analysis",
        ),
    }
}

fn padded_bounds(minimum: f64, maximum: f64) -> Option<(f64, f64)> {
    if !minimum.is_finite() || !maximum.is_finite() || minimum > maximum {
        return None;
    }
    if minimum < maximum {
        let pad = ((maximum - minimum) * 0.08).max(f64::EPSILON);
        return Some((minimum - pad, maximum + pad));
    }
    let pad = (minimum.abs() * 0.08).max(1.0);
    Some((minimum - pad, maximum + pad))
}

fn automatic_x_range(model: &HarmonicBalanceModel) -> Option<(f64, f64)> {
    if !model.frequency_max.is_finite() || model.frequency_max < 0.0 {
        return None;
    }
    if model.frequency_max == 0.0 {
        return Some((0.0, 1.0));
    }
    Some((0.0, model.frequency_max * 1.08))
}

fn automatic_y_range(model: &HarmonicBalanceModel) -> Option<(f64, f64)> {
    // A zero reference is part of a coefficient/stem plot, not a claim that
    // every coefficient is positive.  Negative values can occur in legacy
    // converted data, so keep their exact extent too.
    let lower = model.magnitude_min.min(0.0);
    let upper = model.magnitude_max.max(0.0);
    if lower >= 0.0 {
        return (upper > 0.0).then_some((0.0, upper * 1.08));
    }
    padded_bounds(lower, upper)
}

/// Render the qualified coefficient spectrum and request navigation; its
/// cursor reads at the reader's significant digits and by the reader's
/// cursor interpolation. `views` holds each strip's viewport by its index,
/// [`strip_count`] of them.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    settings: &CoefficientSettings,
    views: &[PlotView],
    quantity_policy: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
    pin: &mut Option<f64>,
) -> SpectrumResponse {
    if model.fft.is_some() || model.traces.iter().any(|trace| trace.lines.is_empty()) {
        return show_recorded(
            ui,
            model,
            views.first().copied().unwrap_or_default(),
            quantity_policy,
            presentation,
            cache,
            pin,
        );
    }
    sheet::show(
        ui,
        model,
        settings,
        views,
        quantity_policy,
        presentation,
        cache,
        pin,
    )
}

/// How many strips, and so viewports, the sheet draws for `model`.
#[must_use]
pub fn strip_count(model: &HarmonicBalanceModel, settings: &CoefficientSettings) -> usize {
    if model.fft.is_some() || model.traces.iter().any(|trace| trace.lines.is_empty()) {
        1
    } else {
        sheet::strips(model, settings).len()
    }
}

/// A recorded `.FFT`: the engine's magnitudes on logarithmic decades, one
/// strip, as it has always been drawn.
fn show_recorded(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    view: PlotView,
    quantity_policy: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
    pin: &mut Option<f64>,
) -> SpectrumResponse {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let legend = model
        .traces
        .iter()
        .map(|trace| LegendChip {
            name: &trace.name,
            color: trace.color,
            on: true,
        })
        .collect::<Vec<_>>();
    let header = strip::StripHeader::new(
        "SPECTRUM",
        &format!(
            "{} · {} retained spectral samples",
            model.label, model.retained_frequency_count
        ),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    let requested = SpectrumResponse {
        fit: header.fit_clicked,
        plots: Vec::new(),
    };

    let Some((auto_x0, auto_x1)) = automatic_x_range(model) else {
        well_hint(ui, "Retained HB frequency axis is degenerate");
        return requested;
    };
    let Some((auto_y0, auto_y1)) = automatic_y_range(model) else {
        well_hint(ui, "Retained HB magnitudes are degenerate");
        return requested;
    };
    let (x0, x1) = view.x.unwrap_or((auto_x0, auto_x1));
    let (y0, y1) = view.y.unwrap_or((auto_y0, auto_y1));
    if !(x0 < x1 && y0 < y1) {
        well_hint(ui, "Retained HB plot range is invalid");
        return requested;
    }

    let (frequency_scale, frequency_offset, frequency_unit) =
        quantity_policy.frequency_axis_transform();
    // A recorded FFT is drawn on logarithmic decades over the engine's own
    // linear magnitudes: the same picture as dB, with no derived array and no
    // second cache.
    let decades = model.fft.as_ref().and_then(|_| {
        recorded_fft::decade_ordinate(model.smallest_positive_magnitude, model.magnitude_max, "")
    });
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Hz").with_display_transform(
            frequency_scale,
            frequency_offset,
            frequency_unit,
        ),
        XScale::Linear,
        decades
            .clone()
            .unwrap_or_else(|| Axis::linear_with(y0, y1, "", 7).with_label("magnitude")),
    )
    .accessible_name("Retained complex coefficient spectrum")
    .accessible_detail(
        "Exact retained magnitude coefficients. Solver tone configuration, harmonic order, convergence iterations, fundamental, and THD are shown only when retained.",
    );
    if decades.is_some() {
        spec = spec.with_log_y();
    } else {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // One painted segment per coefficient, so a long spectrum draws none:
    // a recorded FFT can hold half a million bins, where HB holds tens.
    let stems = if model.retained_frequency_count > recorded_fft::MAX_STEMMED_COEFFICIENTS {
        Vec::new()
    } else {
        model
            .traces
            .iter()
            .map(|trace| {
                (
                    trace.frequency.clone(),
                    trace.magnitude.clone(),
                    trace.color,
                )
            })
            .collect::<Vec<_>>()
    };
    let stem_floor = decades.as_ref().map_or(0.0, |axis| axis.min);
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let baseline = mapper.y(stem_floor);
        for (frequency, magnitude, color) in &stems {
            for (&x, &y) in frequency.iter().zip(magnitude.iter()) {
                painter.line_segment(
                    [
                        egui::pos2(mapper.x(x), baseline),
                        egui::pos2(mapper.x(x), mapper.y(y)),
                    ],
                    egui::Stroke::new(1.0, *color),
                );
            }
        }
    }));
    for (index, trace) in model.traces.iter().enumerate() {
        spec.traces.push(
            Trace::new(&trace.frequency, &trace.magnitude, trace.color)
                .thin()
                .marker_style(index)
                .cache_key(trace.cache_key),
        );
    }

    // Every trace reads at the cursor: the panel the readout sits in
    // scrolls, so no signal is left off.
    let readout = |frequency: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            "f".to_owned(),
            frequency_readout(quantity_policy, frequency, digits),
        )];
        for trace in &model.traces {
            rows.push((
                trace.name.clone(),
                fmt_si_significant(
                    presentation.cursor_sample(&trace.frequency, &trace.magnitude, frequency),
                    "",
                    digits,
                ),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    crate::chrome::pinned_readout::follow(ui, &response, spec.x_scale, pin, &readout);
    SpectrumResponse {
        plots: vec![(0, response)],
        ..requested
    }
}

/// The figures and tables read off the same retained coefficient spectrum,
/// at the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    source: Result<&HarmonicBalanceModel, SpectrumAbsence<'_>>,
    settings: &CoefficientSettings,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let model = match source {
        Ok(model) => model,
        Err(reason) => {
            match reason {
                SpectrumAbsence::Failed(error) => crate::presentation::panel_note(
                    ui,
                    &format!("Spectrum execution failed: {error}"),
                ),
                SpectrumAbsence::IncompleteHistory(sentence) => {
                    crate::presentation::panel_note(ui, &sentence)
                }
                SpectrumAbsence::Unavailable => {
                    crate::presentation::panel_note(ui, "No harmonic-balance result in this run.")
                }
            }
            return;
        }
    };
    // A recorded FFT states the transform the engine performed; a quasi-period
    // steady state states the lattice it was solved on.
    if let Some(spectrum) = &model.fft {
        section_header(ui, "Spectrum", None);
        crate::presentation::stat_table(
            ui,
            &recorded_fft::inspector_rows(spectrum, quantity_policy, significant_digits),
        );
        return;
    }
    if let Some(point) = &model.qpss {
        section_header(ui, "Lattice", None);
        let config = point.config();
        let rows = [
            (
                "Independent tones",
                config
                    .grid
                    .frequencies_hz
                    .iter()
                    .map(|frequency| {
                        frequency_readout(quantity_policy, *frequency, significant_digits)
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                true,
            ),
            (
                "Harmonic orders",
                config
                    .grid
                    .harmonics
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                false,
            ),
            (
                "Mixing order",
                config
                    .grid
                    .max_mixing_order
                    .map_or_else(|| "Full lattice".into(), |order| order.to_string()),
                false,
            ),
            ("Newton iterations", point.iterations().to_string(), false),
            (
                "Normalized residual",
                format!("{:.2e}", point.normalized_residual()),
                false,
            ),
        ];
        crate::presentation::stat_table(ui, &rows);
    }
    details::show(ui, model, settings, quantity_policy, significant_digits);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::harmonic_spectrum::settings::CoefficientScale;

    /// A 1 MHz tone's 40 harmonic lines on V(out), each 1/k² of the
    /// fundamental (H3 at −19.08 dBc), and a current I(V1) beside it.
    fn forty_harmonics() -> HarmonicBalanceModel {
        let trace = |name: &str, scale: f64| {
            let lines: Arc<[CoefficientLine]> = (0..=40)
                .map(|order| {
                    let magnitude = if order == 0 {
                        0.5 * scale
                    } else {
                        scale / f64::from(order * order)
                    };
                    CoefficientLine {
                        frequency: f64::from(order) * 1.0e6,
                        re: magnitude,
                        im: 0.0,
                    }
                })
                .collect();
            let frequency: Vec<f64> = lines.iter().map(|line| line.frequency).collect();
            let magnitude: Vec<f64> = lines.iter().map(CoefficientLine::magnitude).collect();
            HarmonicTrace {
                name: name.to_owned(),
                frequency: HeldColumn::from(frequency),
                magnitude: HeldColumn::from(magnitude),
                lines,
                color: egui::Color32::WHITE,
                cache_key: 1,
            }
        };
        HarmonicBalanceModel {
            label: "HB".to_owned(),
            traces: vec![trace("V(out)", 1.0), trace("I(V1)", 1.0e-3)],
            frequency_max: 40.0e6,
            magnitude_min: 0.0,
            magnitude_max: 1.0,
            retained_frequency_count: 41,
            fft: None,
            qpss: None,
            smallest_positive_magnitude: Some(1.0e-3 / 1600.0),
        }
    }

    /// What the details pane announced for `model`, on a screen tall enough
    /// that no row of its table scrolls out of view.
    fn details(model: &HarmonicBalanceModel, settings: &CoefficientSettings) -> Vec<String> {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 6000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    right_panel(
                        ui,
                        Ok(model),
                        settings,
                        &QuantityPresentationPolicy::default(),
                        4,
                    );
                });
            },
        )
        .platform_output
        .accesskit_update
        .expect("details accessibility tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
        .collect()
    }

    /// Every one of forty harmonics of each signal is a row of that
    /// signal's line table, the third reads its level under the fundamental
    /// in dBc, and each signal's THD reads in percent and in dB.
    #[test]
    fn an_hb_spectrum_lists_every_harmonic_and_reads_the_third_in_dbc() {
        let model = forty_harmonics();
        let settings = CoefficientSettings {
            scale: CoefficientScale::Dbc,
            ..CoefficientSettings::default()
        };
        let rows = details(&model, &settings);
        let harmonics = rows.iter().filter(|row| row.starts_with("Line: H")).count();
        assert_eq!(harmonics, 2 * 40, "{rows:?}");
        let third = rows
            .iter()
            .find(|row| row.starts_with("Line: H3,"))
            .expect("H3 listed");
        assert!(third.contains("dBc: −19.08,"), "{third}");
        // The THD of lines at 1/k² of the fundamental: √(ζ(4) − 1).
        for value in ["28.69 %", "−10.85 dB"] {
            assert!(rows.iter().any(|row| row == value), "no {value}: {rows:?}");
        }
    }

    /// Voltages and currents stand on strips of their own, as do the phase
    /// and the waveform when asked for.
    #[test]
    fn voltages_and_currents_are_never_read_against_one_axis() {
        let model = forty_harmonics();
        let units = |settings: &CoefficientSettings| {
            sheet::strips(&model, settings)
                .iter()
                .map(|strip| strip.axes[0].unit.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(units(&CoefficientSettings::default()), ["dBV", "dB re 1 A"]);
        let all = CoefficientSettings {
            scale: CoefficientScale::Dbm,
            phase: true,
            waveform: true,
            ..CoefficientSettings::default()
        };
        assert_eq!(units(&all), ["dBm", "°", "V", "dBm", "°", "A"]);
        assert_eq!(strip_count(&model, &all), 6);
    }

    #[test]
    fn padded_bounds_keep_single_retained_frequency_plotable() {
        let (minimum, maximum) = padded_bounds(1.0e6, 1.0e6).expect("finite singleton");
        assert!(minimum < 1.0e6 && maximum > 1.0e6);
        assert!(padded_bounds(f64::NAN, 1.0).is_none());
    }
}
