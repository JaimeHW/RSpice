//! NYQUIST — the loop-gain locus on the complex plane, with the unit
//! circle, the −1 critical point, and stability numbers in the right panel.
//!
//! Built on the plot grammar like every other viewer: token grid and frame,
//! decimated token traces, bordered marker tags — no private palette.

use egui::Ui;

use crate::analysis::nyquist::NyquistData;
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{self, LegendChip};
use super::well_hint;

/// The active curve and its actual cache identity (prefer the selected one).
fn active_curve(state: &AppState) -> Option<(usize, &NyquistData)> {
    let nyquist = &state.analysis.nyquist_state;
    nyquist
        .curves
        .get(nyquist.selected)
        .map(|curve| (nyquist.selected, curve))
        .or_else(|| nyquist.curves.first().map(|curve| (0, curve)))
        .filter(|(_, curve)| !curve.is_empty())
}

/// Stability numbers, cached on the data version and selected curve.
#[derive(Debug, Clone, Copy)]
pub struct NyquistDerived {
    pub(crate) version: u64,
    pub(crate) curve_index: usize,
    pub(crate) encirclements: i32,
    /// The Nyquist stability criterion also requires the number of open-loop
    /// right-half-plane poles. Current viewer data does not retain that
    /// authority, so this remains explicit instead of being inferred.
    pub(crate) open_loop_rhp_poles: Option<u32>,
    pub(crate) min_distance: Option<f64>,
    pub(crate) gain_margin: Option<f64>,
    pub(crate) phase_margin: Option<f64>,
}

fn derived(state: &mut AppState) -> Option<NyquistDerived> {
    let version = state.simulation.data_version;
    let curve_index = active_curve(state)?.0;
    if let Some(cached) = state.ui.results.nyquist
        && cached.version == version
        && cached.curve_index == curve_index
    {
        return Some(cached);
    }
    let curve = active_curve(state)?.1;
    let computed = NyquistDerived {
        version,
        curve_index,
        encirclements: curve.count_encirclements(),
        open_loop_rhp_poles: None,
        min_distance: curve.min_distance_from_critical(),
        gain_margin: curve.gain_margin(),
        phase_margin: curve.phase_margin(),
    };
    state.ui.results.nyquist = Some(computed);
    Some(computed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NyquistStabilityVerdict {
    Stable,
    Unstable,
    Indeterminate,
}

impl NyquistStabilityVerdict {
    const fn label(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Unstable => "unstable",
            Self::Indeterminate => "indeterminate",
        }
    }
}

fn stability_verdict(
    encirclements: i32,
    open_loop_rhp_poles: Option<u32>,
) -> NyquistStabilityVerdict {
    let Some(open_loop_rhp_poles) = open_loop_rhp_poles else {
        return NyquistStabilityVerdict::Indeterminate;
    };
    if i64::from(encirclements) == -i64::from(open_loop_rhp_poles) {
        NyquistStabilityVerdict::Stable
    } else {
        NyquistStabilityVerdict::Unstable
    }
}

const fn curve_cache_key(base: u64, curve_index: usize) -> u64 {
    base ^ (curve_index as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Nyquist locus.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();

    let Some((curve_index, curve)) = active_curve(state) else {
        well_hint(ui, "No loop-gain data — run an AC analysis");
        return;
    };
    let name = curve.name.clone();
    let point_count = curve.len();

    // Component arrays for the plot engine, cached per data version.
    let (re, im) = {
        let points: Vec<_> = curve.points.clone();
        let derived = &mut state.ui.results.derived;
        let re = derived.get_or(curve_cache_key(0x917_0001, curve_index), || {
            std::sync::Arc::new(points.iter().map(|p| p.real).collect::<Vec<_>>())
        });
        let im = derived.get_or(curve_cache_key(0x917_0002, curve_index), || {
            std::sync::Arc::new(points.iter().map(|p| p.imag).collect::<Vec<_>>())
        });
        (re, im)
    };

    let stats = derived(state);

    let legend = [LegendChip {
        name: "loop gain",
        color: c.traces[0],
        on: true,
    }];
    strip::StripHeader::new("NYQ", &format!("{name} · {point_count} pts"), &legend).show(ui);

    // Equal-aspect ranges around the locus and the critical point.
    let mut extent = 1.3f64;
    for (&r, &i) in re.iter().zip(im.iter()) {
        if r.is_finite() && i.is_finite() {
            extent = extent.max(r.abs()).max(i.abs());
        }
    }
    let extent = (extent * 1.1).min(50.0);
    let view = state.ui.results.plot_view(super::ResultViewer::Nyquist, 0);
    let (x0, x1) = view.x.unwrap_or((-extent, extent));
    let (y0, y1) = view.y.unwrap_or((-extent, extent));

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Re"),
        XScale::Linear,
        Axis::linear(y0, y1, "Im"),
    )
    .accessible_name("Nyquist plot");
    spec.ref_lines.push(plot::RefLine { y: 0.0 });
    spec.traces.push(
        // Re L(jω) is a coordinate, not an ordering — the locus encircles.
        Trace::new(&re, &im, c.traces[0])
            .parametric()
            .cache_key(curve_cache_key(0x917_00FF, curve_index)),
    );

    // Critical point.
    spec.markers.push(plot::Marker {
        x: -1.0,
        y: 0.0,
        color: c.err,
        label: "−1 + j0".to_owned(),
        drop_line: false,
        label_dy: 0.0,
        shape: plot::MarkerShape::Point,
    });

    // Unit circle + vertical axis underlay.
    let grid = c.canvas_grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let radius = (mapper.x(1.0) - mapper.x(0.0)).abs();
        painter.circle_stroke(center, radius, egui::Stroke::new(1.0, grid));
        painter.vline(
            center.x,
            egui::Rangef::new(mapper.rect.top(), mapper.rect.bottom()),
            egui::Stroke::new(1.0, grid),
        );
    }));

    // Region sized so the INNER plot area is square: stability reading
    // needs the unit circle and the locus on one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let stats_for_readout = stats;
    let readout = move |_x: f64| -> Vec<(String, String)> {
        let mut rows = Vec::new();
        if let Some(s) = stats_for_readout {
            rows.push(("N".to_owned(), s.encirclements.to_string()));
            if let Some(d) = s.min_distance {
                rows.push(("min |1+L|".to_owned(), fmt_si(d, "", 2)));
            }
        }
        rows
    };

    let response = plot::show(
        &mut plot_ui,
        &spec,
        &mut state.ui.results.cache,
        None,
        Some(&readout),
    );
    if response.view.any() {
        let change = super::square_xy_view_change((x0, x1), (y0, y1), response.view);
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Nyquist, 0)
            .apply(&change);
    }

    // Nearest locus point on hover, click to pin — gain/phase/frequency at
    // a spot on the locus is the question a Nyquist plot exists to answer.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for i in 0..re.len() {
            let pos = super::xy_screen_pos(response.plot_rect, (re[i], im[i]), ranges.0, ranges.1);
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, i));
            }
        }
    }

    if response.response.clicked() {
        let pins = &mut state.ui.results.rf_pin;
        match hovered {
            Some(hit) if pins.get(&super::ResultViewer::Nyquist) == Some(&hit) => {
                pins.remove(&super::ResultViewer::Nyquist);
            }
            Some(hit) => {
                pins.insert(super::ResultViewer::Nyquist, hit);
            }
            None => {
                pins.remove(&super::ResultViewer::Nyquist);
            }
        }
    }

    let pinned = state
        .ui
        .results
        .rf_pin
        .get(&super::ResultViewer::Nyquist)
        .copied()
        .filter(|(_, i)| *i < re.len());
    let target = hovered.or(pinned);

    if let Some((_, i)) = target {
        let pos = super::xy_screen_pos(response.plot_rect, (re[i], im[i]), ranges.0, ranges.1);
        let painter = plot_ui.painter();
        if pinned == target {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, c.traces[0]));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let frequency = active_curve(state)
            .and_then(|(_, curve)| curve.points.get(i).map(|p| p.frequency))
            .unwrap_or(0.0);
        let magnitude = (re[i] * re[i] + im[i] * im[i]).sqrt();
        let phase_radians = im[i].atan2(re[i]);
        let rows = [
            (
                "f".to_owned(),
                quantity_policy.format_frequency(frequency, 2),
            ),
            ("Re".to_owned(), fmt_si(re[i], "", 3)),
            ("Im".to_owned(), fmt_si(im[i], "", 3)),
            (
                "|L| ∠".to_owned(),
                format!(
                    "{magnitude:.3} ∠ {}",
                    quantity_policy.format_angle(phase_radians, 1)
                ),
            ),
        ];
        super::point_card(&plot_ui, response.plot_rect, pos, &name, c.traces[0], &rows);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Stability readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Stability", None);
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(s) = derived(state) else {
        super::panel_note(
            ui,
            "Stability numbers appear once AC loop-gain data is loaded.",
        );
        return;
    };

    let verdict = stability_verdict(s.encirclements, s.open_loop_rhp_poles);
    let fmt_opt =
        |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String { v.map_or("—".to_owned(), f) };
    let rows = [
        ("Encirclements", s.encirclements.to_string(), true),
        ("Verdict", verdict.label().to_owned(), true),
        (
            "Open-loop RHP poles",
            s.open_loop_rhp_poles
                .map_or_else(|| "not retained".to_owned(), |count| count.to_string()),
            true,
        ),
        (
            "Min distance to −1",
            fmt_opt(s.min_distance, &|v| fmt_si(v, "", 3)),
            false,
        ),
        (
            "Gain margin",
            fmt_opt(s.gain_margin, &|v| format!("{:.1} dB", 20.0 * v.log10())),
            false,
        ),
        (
            "Phase margin",
            fmt_opt(s.phase_margin, &|degrees| {
                quantity_policy.format_angle(degrees.to_radians(), 1)
            }),
            false,
        ),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        if verdict == NyquistStabilityVerdict::Indeterminate {
            "Encirclements are retained, but closed-loop stability is indeterminate without the retained open-loop RHP pole count."
        } else {
            "Verdict applies the retained open-loop RHP pole count to encirclements around −1 + j0."
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::nyquist::NyquistData;

    #[test]
    fn selected_curve_is_part_of_the_derived_cache_authority() {
        let mut state = AppState::default();
        state.analysis.nyquist_state.curves = vec![
            NyquistData::from_arrays("near", &[1.0], &[0.0], &[0.0]),
            NyquistData::from_arrays("far", &[1.0], &[-1.0], &[2.0]),
        ];

        let first = derived(&mut state).expect("first selected curve");
        assert_eq!(first.curve_index, 0);
        assert_eq!(first.min_distance, Some(1.0));

        state.analysis.nyquist_state.selected = 1;
        let second = derived(&mut state).expect("second selected curve");
        assert_eq!(second.curve_index, 1);
        assert_eq!(second.min_distance, Some(2.0));
    }

    #[test]
    fn stability_is_indeterminate_without_open_loop_pole_authority() {
        assert_eq!(
            stability_verdict(0, None),
            NyquistStabilityVerdict::Indeterminate
        );
        assert_eq!(
            stability_verdict(0, Some(0)),
            NyquistStabilityVerdict::Stable
        );
        assert_eq!(
            stability_verdict(-1, Some(1)),
            NyquistStabilityVerdict::Stable
        );
        assert_eq!(
            stability_verdict(0, Some(1)),
            NyquistStabilityVerdict::Unstable
        );
    }
}
