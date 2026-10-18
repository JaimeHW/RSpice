//! Smith chart rendering and retained trace readouts.

use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::{network_matrix::impedance_from_gamma, waveform::SharedWaveformValues};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_readout};
use rspice_ui_kit::tokens::Tokens;

use super::{SmithChartState, state::SParamTrace};
use crate::presentation::{PlotView, angle_readout, frequency_readout, plain_readout, well_hint};
use crate::strip::{self, LegendChip};

/// A visible retained trace and its cached coordinate columns.
/// The index identifies the trace in the source inventory; both columns contain
/// the corresponding complex coordinate of every retained point.
pub struct SmithPlotTrace<'a> {
    pub index: usize,
    pub trace: &'a SParamTrace,
    pub real: SharedWaveformValues,
    pub imaginary: SharedWaveformValues,
}

/// Draw visible retained loci and update only their local view and pin.
/// The host owns source qualification and cache invalidation. The point card
/// reads at the reader's `significant_digits`.
pub fn show(
    ui: &mut Ui,
    traces: &[SmithPlotTrace<'_>],
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Option<plot::PlotResponse> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    if traces.is_empty() {
        well_hint(
            ui,
            "No S-parameter traces — run an SP, PSP, or HBSP analysis",
        );
        return None;
    }
    let legend: Vec<LegendChip> = traces
        .iter()
        .enumerate()
        .map(|(slot, prepared)| LegendChip {
            name: &prepared.trace.name,
            color: c.traces[slot % c.traces.len()],
            on: true,
        })
        .collect();
    let mut retained_references = traces
        .iter()
        .filter_map(|prepared| prepared.trace.reference_impedance_ohm)
        .collect::<Vec<_>>();
    retained_references.sort_by(f64::total_cmp);
    retained_references.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let reference_label = match retained_references.as_slice() {
        [reference] => format!("Z₀ = {reference} Ω"),
        [] => "Coefficient loci".to_owned(),
        _ => "Per-port retained Z₀".to_owned(),
    };
    strip::StripHeader::new("SMITH", &reference_label, &legend).show(ui);

    let (x0, x1) = view.x.unwrap_or((-1.12, 1.12));
    let (y0, y1) = view.y.unwrap_or((-1.12, 1.12));
    let accessible_detail = format!(
        "{} visible loci with {} retained samples; {} reflection references retained",
        traces.len(),
        traces
            .iter()
            .map(|prepared| prepared.real.len())
            .sum::<usize>(),
        traces
            .iter()
            .filter(|prepared| prepared.trace.reference_impedance_ohm.is_some())
            .count()
    );
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Re Γ"),
        XScale::Linear,
        Axis::linear(y0, y1, "Im Γ"),
    )
    .accessible_name("Smith chart")
    .accessible_detail(&accessible_detail);
    for (slot, prepared) in traces.iter().enumerate() {
        spec.traces.push(
            // Re Γ is a coordinate, not an ordering: the locus crosses the
            // same abscissa on the way out and on the way back.
            Trace::new(
                &prepared.real,
                &prepared.imaginary,
                c.traces[slot % c.traces.len()],
            )
            .parametric()
            .cache_key(plot::trace_cache_key(0x501_00F0, prepared.index)),
        );
    }

    // The canonical chart grid: constant-resistance circles and
    // constant-reactance arcs in token grid color, |Γ| = 1 boundary in
    // the strong border color.
    let grid = c.canvas_grid;
    let boundary = c.border_strong;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let unit = (mapper.x(1.0) - mapper.x(0.0)).abs();
        let stroke = egui::Stroke::new(1.0, grid);

        // Constant resistance r: center ((r/(r+1)), 0), radius 1/(r+1).
        for r in [0.2, 0.5, 1.0, 2.0, 5.0] {
            let cx = r / (r + 1.0);
            let radius = 1.0 / (r + 1.0);
            painter.circle_stroke(
                egui::pos2(mapper.x(cx), mapper.y(0.0)),
                (radius * unit as f64) as f32,
                stroke,
            );
        }
        // Constant reactance x: circles centered (1, ±1/x) with radius
        // 1/x, clipped to the unit disc. Walk the full circle and emit a
        // polyline per contiguous in-disc run.
        for x in [0.2f64, 0.5, 1.0, 2.0, 5.0] {
            for sign in [1.0f64, -1.0] {
                let cy = sign / x;
                let radius = 1.0 / x;
                let mut run: Vec<egui::Pos2> = Vec::new();
                for step in 0..=128 {
                    let theta = std::f64::consts::TAU * step as f64 / 128.0;
                    let gx = 1.0 + radius * theta.cos();
                    let gy = cy + radius * theta.sin();
                    if gx * gx + gy * gy <= 1.0001 {
                        run.push(egui::pos2(mapper.x(gx), mapper.y(gy)));
                    } else if run.len() >= 2 {
                        painter.add(egui::Shape::line(std::mem::take(&mut run), stroke));
                    } else {
                        run.clear();
                    }
                }
                if run.len() >= 2 {
                    painter.add(egui::Shape::line(run, stroke));
                }
            }
        }
        // Real axis + boundary.
        painter.hline(
            egui::Rangef::new(mapper.x(-1.0), mapper.x(1.0)),
            center.y,
            stroke,
        );
        painter.circle_stroke(center, unit, egui::Stroke::new(1.2, boundary));
    }));

    // Region sized so the INNER plot area is square — the chart's circle
    // grid and the Γ loci must share one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let response = plot::show(&mut plot_ui, &spec, cache, None, None);
    if response.view.any() {
        let change = crate::presentation::square_xy_view_change((x0, x1), (y0, y1), response.view);
        view.apply(&change);
    }

    // Interactive readout: nearest locus point on hover, click to pin.
    // The chart space is the complex-coefficient plane. Only a physical
    // diagonal Sii trace has the retained Z₀ needed for impedance and VSWR.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for (slot, prepared) in traces.iter().enumerate() {
            let (re, im) = (&prepared.real, &prepared.imaginary);
            for i in 0..re.len() {
                let pos = crate::presentation::xy_screen_pos(
                    response.plot_rect,
                    (re[i], im[i]),
                    ranges.0,
                    ranges.1,
                );
                let d2 = pos.distance_sq(pointer);
                if d2 < best {
                    best = d2;
                    hovered = Some((slot, i));
                }
            }
        }
    }

    if response.response.clicked() {
        *pin = match hovered {
            Some(hit) if *pin == Some(hit) => None,
            Some(hit) => Some(hit),
            None => None,
        };
    }

    let pinned = pin.filter(|(slot, i)| {
        traces
            .get(*slot)
            .is_some_and(|prepared| *i < prepared.real.len())
    });
    let target = hovered.or(pinned);

    if let Some((slot, i)) = target {
        let prepared = &traces[slot];
        let (re, im) = (&prepared.real, &prepared.imaginary);
        let gamma_re = re[i];
        let gamma_im = im[i];
        let pos = crate::presentation::xy_screen_pos(
            response.plot_rect,
            (gamma_re, gamma_im),
            ranges.0,
            ranges.1,
        );
        let color = c.traces[slot % c.traces.len()];
        let painter = plot_ui.painter();
        if pinned == Some((slot, i)) {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let trace = prepared.trace;
        let frequency = trace.points.get(i).map(|p| p.frequency).unwrap_or(0.0);
        let mag = (gamma_re * gamma_re + gamma_im * gamma_im).sqrt();
        let phase = gamma_im.atan2(gamma_re);
        let digits = significant_digits;
        let mut rows = vec![
            (
                "f".to_owned(),
                frequency_readout(quantity_policy, frequency, digits),
            ),
            (
                "Γ".to_owned(),
                format!(
                    "{} ∠ {}",
                    plain_readout(mag, "", digits),
                    angle_readout(quantity_policy, phase, digits)
                ),
            ),
        ];
        if let Some(reference_impedance_ohm) = trace.reference_impedance_ohm {
            rows.push((
                "Z₀".to_owned(),
                fmt_si_readout(reference_impedance_ohm, "Ω", digits),
            ));
            rows.push((
                "Z".to_owned(),
                if let Some((resistance, reactance)) =
                    impedance_from_gamma(gamma_re, gamma_im, reference_impedance_ohm)
                {
                    impedance_readout(resistance, reactance, digits)
                } else {
                    "open".to_owned()
                },
            ));
            rows.push((
                "VSWR".to_owned(),
                if mag < 1.0 {
                    plain_readout((1.0 + mag) / (1.0 - mag), "", digits)
                } else {
                    "∞".to_owned()
                },
            ));
        } else {
            rows.push(("Readout".to_owned(), "complex coefficient only".to_owned()));
        }
        crate::presentation::point_card(
            &plot_ui,
            response.plot_rect,
            pos,
            &trace.name,
            color,
            &rows,
        );
    }
    Some(response)
}

/// Trace summary, at the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    smith: &SmithChartState,
    pin: Option<(usize, usize)>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let digits = significant_digits;
    section_header(ui, "S-parameters", None);
    let visible_traces = smith
        .traces
        .iter()
        .filter(|tr| tr.visible && !tr.points.is_empty())
        .collect::<Vec<_>>();
    let Some(trace) = visible_traces.first().copied() else {
        crate::presentation::panel_note(
            ui,
            "Trace metrics appear once S-parameter data is loaded.",
        );
        return;
    };

    let first = trace.points.first();
    let last = trace.points.last();
    let maximum_gamma = trace
        .points
        .iter()
        .map(|point| point.s.norm())
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp);
    let maximum_vswr = trace
        .reference_impedance_ohm
        .and(maximum_gamma)
        .map(|gamma| {
            if gamma < 1.0 {
                format!(
                    "{} : 1",
                    plain_readout((1.0 + gamma) / (1.0 - gamma), "", digits)
                )
            } else {
                "∞".to_owned()
            }
        });
    let marker = pin.as_ref().and_then(|(slot, point)| {
        let trace = visible_traces.get(*slot)?;
        let sample = trace.points.get(*point)?;
        let gamma = sample.s;
        let frequency = frequency_readout(quantity_policy, sample.frequency, digits);
        let readout = if let Some(reference_impedance_ohm) = trace.reference_impedance_ohm {
            if let Some((resistance, reactance)) =
                impedance_from_gamma(gamma.re, gamma.im, reference_impedance_ohm)
            {
                format!(
                    "{frequency} · {}",
                    impedance_readout(resistance, reactance, digits)
                )
            } else {
                format!("{frequency} · open")
            }
        } else {
            format!(
                "{frequency} · |S| {}",
                plain_readout(gamma.norm(), "", digits)
            )
        };
        Some(format!("{} · {readout}", trace.name))
    });
    // The chart has no frequency axis, so the sweep states itself; the point
    // count, an unpinned marker and a locus with no reference impedance have
    // nothing for the reader to read.
    let mut rows = vec![
        ("Network", trace.name.clone(), true),
        (
            "Sweep",
            match (first, last) {
                (Some(a), Some(b)) => format!(
                    "{} – {}",
                    frequency_readout(quantity_policy, a.frequency, digits),
                    frequency_readout(quantity_policy, b.frequency, digits)
                ),
                _ => "—".to_owned(),
            },
            false,
        ),
    ];
    if let Some(reference) = trace.reference_impedance_ohm {
        rows.push(("Z₀", format!("{reference} Ω"), false));
    }
    if let Some(marker) = marker {
        rows.push(("Marker", marker, false));
    }
    if let Some(value) = maximum_gamma {
        rows.push(("|Γ| max", plain_readout(value, "", digits), true));
    }
    if let Some(vswr) = maximum_vswr {
        rows.push(("Max VSWR", vswr, false));
    }
    crate::presentation::stat_table(ui, &rows);
}

/// `R ± jX` at the reader's significant digits: `50.12 Ω + j12.35 Ω`.
fn impedance_readout(resistance: f64, reactance: f64, significant_digits: usize) -> String {
    format!(
        "{} {} j{}",
        fmt_si_readout(resistance, "Ω", significant_digits),
        if reactance >= 0.0 { "+" } else { "−" },
        fmt_si_readout(reactance.abs(), "Ω", significant_digits)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An impedance reads at the chosen significant digits, not at two
    /// decimals whatever the preference says.
    #[test]
    fn an_impedance_reads_at_the_chosen_significant_digits() {
        assert_eq!(
            impedance_readout(50.123_456, -12.345_678, 4),
            "50.12 Ω − j12.35 Ω"
        );
        assert_eq!(
            impedance_readout(50.123_456, 12.345_678, 6),
            "50.1235 Ω + j12.3457 Ω"
        );
    }
}
