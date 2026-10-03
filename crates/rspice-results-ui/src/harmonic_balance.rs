//! Retained coefficient-spectrum presentation for HB, Fourier, recorded FFT and QPSS.

use crate::presentation::{PlotView, well_hint};
use crate::strip::{self, LegendChip};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si, sample_at};
use rspice_ui_kit::widgets::section_header;
use std::sync::Arc;

pub mod recorded_fft;

/// One retained complex coefficient sequence.  `magnitude` is the exact
/// result-conversion magnitude, not a display-derived dB or RMS estimate.
pub struct HarmonicTrace {
    pub name: String,
    pub frequency: SharedWaveformValues,
    pub magnitude: SharedWaveformValues,
    pub color: egui::Color32,
    pub cache_key: u64,
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
    /// Absent for HB, `.FOUR` and the PSS spectrum, which is what keeps their
    /// rendering exactly what it was.
    pub fft: Option<rspice_results::fft::spectrum::FftSpectrumEvidence>,
    pub qpss: Option<Arc<rspice_core::engine::QpssOperatingPoint>>,
    /// The smallest positive magnitude retained, for the decade ordinate.
    pub smallest_positive_magnitude: Option<f64>,
}

/// Why the host could not bind a drawable coefficient spectrum.
pub enum SpectrumAbsence<'a> {
    Failed(&'a str),
    IncompleteHistory(String),
    Unavailable,
}

/// Requests applied by the host to the viewport that produced this frame.
pub struct SpectrumResponse {
    pub fit: bool,
    pub plot: Option<plot::PlotResponse>,
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

/// Render the qualified coefficient spectrum and request navigation.
pub fn show(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    view: PlotView,
    quantity_policy: &QuantityPresentationPolicy,
    cache: &mut plot::DecimationCache,
) -> SpectrumResponse {
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
        plot: None,
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
    // second cache. HB and `.FOUR` keep the linear ordinate they had.
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
        "Exact retained harmonic-balance magnitude coefficients. Solver tone configuration, harmonic order, convergence iterations, fundamental, and THD are shown only when retained.",
    );
    spec.left_margin = 64.0;
    if decades.is_some() {
        spec = spec.with_log_y();
    } else {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Retained coefficients are discrete.  The stem underlay makes that
    // fact clear while the thin trace preserves shared cursor/readout and
    // keyboard accessibility behaviour from the Results plot primitive.
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
                    Arc::clone(&trace.frequency),
                    Arc::clone(&trace.magnitude),
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

    let readout = |frequency: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            "f".to_owned(),
            quantity_policy.format_frequency(frequency, 3),
        )];
        for trace in model.traces.iter().take(3) {
            rows.push((
                trace.name.clone(),
                fmt_si(
                    sample_at(&trace.frequency, &trace.magnitude, frequency),
                    "",
                    4,
                ),
            ));
        }
        if model.traces.len() > 3 {
            rows.push((
                "signals".to_owned(),
                format!("+{} more", model.traces.len() - 3),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    SpectrumResponse {
        plot: Some(response),
        ..requested
    }
}

/// Display metadata carried by the same retained coefficient spectrum.
pub fn right_panel(
    ui: &mut Ui,
    source: Result<&HarmonicBalanceModel, SpectrumAbsence<'_>>,
    quantity_policy: &QuantityPresentationPolicy,
) {
    section_header(ui, "Retained spectrum", None);
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
                SpectrumAbsence::Unavailable => crate::presentation::panel_note(
                    ui,
                    "Select a completed HB, QPSS or Fourier result with retained complex coefficients.",
                ),
            }
            return;
        }
    };
    let lowest_retained = model
        .traces
        .iter()
        .flat_map(|trace| trace.frequency.iter().copied())
        .filter(|frequency| *frequency > 0.0)
        .min_by(f64::total_cmp);
    let highest_retained = model
        .traces
        .iter()
        .flat_map(|trace| trace.frequency.iter().copied())
        .max_by(f64::total_cmp);

    let retained_samples = format!(
        "{} spectral sample{} · f₀ not retained",
        model.retained_frequency_count,
        if model.retained_frequency_count == 1 {
            ""
        } else {
            "s"
        }
    );
    // A recorded FFT states the transform the engine performed, so the
    // "Not retained" rows below would be false of it.
    if let Some(spectrum) = &model.fft {
        crate::presentation::stat_table(ui, &recorded_fft::inspector_rows(spectrum));
    } else if let Some(point) = &model.qpss {
        let config = point.config();
        let rows = [
            (
                "Independent tones",
                config
                    .grid
                    .frequencies_hz
                    .iter()
                    .map(|frequency| quantity_policy.format_frequency(*frequency, 6))
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
                format!("{:.6e}", point.normalized_residual()),
                false,
            ),
            ("Amplitude", "Peak; signed DC unchanged".into(), false),
        ];
        crate::presentation::stat_table(ui, &rows);
        crate::presentation::panel_note(
            ui,
            "Each component has a signed tone tuple. Export CSV for the complete signed Fourier coefficients and their tuples.",
        );
    } else {
        let rows = [
            ("Tones / f₀", retained_samples, true),
            ("Harmonic order", "Not retained".to_owned(), false),
            (
                "Convergence",
                "Completed · solver iterations not retained".to_owned(),
                false,
            ),
            ("Fundamental", "Not retained".to_owned(), false),
            ("THD", "Not retained".to_owned(), true),
        ];
        crate::presentation::stat_table(ui, &rows);
    }

    section_header(ui, "Retained spectrum", None);
    let rows = [
        ("Signals", model.traces.len().to_string(), false),
        (
            "Lowest frequency",
            lowest_retained.map_or_else(
                || "DC only".to_owned(),
                |frequency| quantity_policy.format_frequency(frequency, 3),
            ),
            false,
        ),
        (
            "Highest frequency",
            highest_retained.map_or("—".to_owned(), |frequency| {
                quantity_policy.format_frequency(frequency, 3)
            }),
            false,
        ),
        (
            "Magnitude range",
            format!(
                "{} … {}",
                fmt_si(model.magnitude_min, "", 4),
                fmt_si(model.magnitude_max, "", 4)
            ),
            false,
        ),
    ];
    crate::presentation::stat_table(ui, &rows);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padded_bounds_keep_single_retained_frequency_plotable() {
        let (minimum, maximum) = padded_bounds(1.0e6, 1.0e6).expect("finite singleton");
        assert!(minimum < 1.0e6 && maximum > 1.0e6);
        assert!(padded_bounds(f64::NAN, 1.0).is_none());
    }
}
