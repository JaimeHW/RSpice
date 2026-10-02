//! Nyquist locus rendering and stability readout over explicit retained data.

use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::section_header;

use super::{EncirclementCount, NyquistData, NyquistMargin};
use crate::presentation::{PlotView, well_hint};
use crate::strip::{self, LegendChip};

/// Measurements from the retained locus, cached by the host data version.
#[derive(Debug, Clone, Copy)]
pub struct NyquistStability {
    pub encirclements: EncirclementCount,
    pub min_distance: Option<f64>,
    pub gain_margin: Option<NyquistMargin>,
    pub phase_margin: Option<NyquistMargin>,
}

/// One nonempty retained loop-gain contour and its cached coordinate columns.
/// Both columns must contain the corresponding coordinate of every curve point.
pub struct NyquistPlot<'a> {
    pub curve: &'a NyquistData,
    pub real: &'a [f64],
    pub imaginary: &'a [f64],
    pub stability: Option<NyquistStability>,
}

/// What the tab needs, spelled the way the reader can act on it.
///
/// The locus is a *loop gain*, and the numbers beside it are the Nyquist
/// criterion applied to one. An AC node response or a harmonic-balance
/// spectrum is not a loop gain, and drawing one here with stability numbers
/// attached would put a verdict on a quantity that has none — so nothing but
/// a retained loop-gain contour reaches this sheet.
const NEEDS_LOOP_GAIN: &str =
    "No loop gain — run a stability (.STB) analysis with its Nyquist contour retained";

/// How an encirclement count reads in the panel.
fn encirclement_row(count: EncirclementCount) -> String {
    match count {
        EncirclementCount::Counted(n) => n.to_string(),
        EncirclementCount::TouchesCriticalPoint => "on −1 + j0".to_owned(),
        EncirclementCount::Unresolved { turns } => format!("unresolved ({turns:+.2} turns)"),
        EncirclementCount::NotFinite => "non-finite locus".to_owned(),
    }
}

/// The criterion, written out for the count this locus actually has.
///
/// P — the number of open-loop right-half-plane poles — is not something a
/// loop-gain sweep can measure, and no retained result here supplies it: a
/// pole-zero run on the deck the probe measured returns the poles of the
/// circuit as connected, which is the *closed* loop. So the criterion is
/// stated rather than resolved, with the one case the reader can settle from
/// the schematic (P = 0, an open loop with no unstable poles of its own)
/// spelled out.
///
/// A negative N is the exception, and it is the criterion that supplies it:
/// Z counts poles, so it cannot be negative, and a counter-clockwise winding
/// is therefore a *proof* that P is at least −N. Reading P = 0 into it the way
/// the positive case does prints Z < 0, which is not a number of poles.
fn criterion_note(count: EncirclementCount) -> String {
    let Some(n) = count.counted() else {
        return match count {
            EncirclementCount::Unresolved { turns } => format!(
                "{turns:+.2} turns about −1 + j0, and nothing certifies them as a winding: both \
                 ends of the contour are chords rather than measurements, and they only stand for \
                 the loop once |L| has fallen well below 1 at the top of the sweep and the locus \
                 is back on the real axis at the bottom. Widen the sweep — no encirclement count, \
                 and so no verdict, follows from this locus."
            ),
            _ => "The contour has no encirclement count, so the criterion Z = N + P cannot be \
                  applied to it."
                .to_owned(),
        };
    };
    let criterion = format!(
        "Nyquist criterion: Z = N + P = {n:+} + P closed-loop right-half-plane poles, where P counts \
         the open-loop ones. P has to come from a pole-zero analysis of the loop-broken deck — a \
         pole-zero run on this deck returns closed-loop poles."
    );
    if n < 0 {
        let p = -n;
        return format!(
            "{criterion} A counter-clockwise winding proves P ≥ {p}, since Z cannot be negative: \
             with P = {p} the loop is stable (Z = 0), and each open-loop pole beyond that is one \
             the loop does not close."
        );
    }
    let z = super::closed_loop_rhp_poles(n, 0);
    let verdict = if z == 0 { "stable" } else { "unstable" };
    format!("{criterion} With P = 0 the loop is {verdict} (Z = {z}).")
}

/// Viewer-scoped base for this sheet's entries in the workspace-wide derived
/// and decimation caches; the series ordinals below separate its own arrays.
/// Composed through [`plot::trace_cache_key`], which moves the ordinal clear
/// of the base instead of folding it in where the base already has bits.
pub const NYQUIST_CACHE_BASE: u64 = 0x917_0000;
/// Real-coordinate entry in the host derived-series cache.
pub const REAL_SERIES: usize = 0;
/// Imaginary-coordinate entry in the host derived-series cache.
pub const IMAGINARY_SERIES: usize = 1;
const LOCUS_SERIES: usize = 2;

/// Render a retained locus using host-selected data and local interaction state.
/// The host owns cache invalidation and applies the returned axes to its inspector.
pub fn show(
    ui: &mut Ui,
    source: Option<NyquistPlot<'_>>,
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
) -> Option<plot::PlotResponse> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(source) = source else {
        well_hint(ui, NEEDS_LOOP_GAIN);
        return None;
    };
    let NyquistPlot {
        curve,
        real: re,
        imaginary: im,
        stability: stats,
    } = source;
    let name = &curve.name;
    let point_count = curve.len();

    // The legend names the quantity that is actually plotted, which is the
    // retained contour's own name rather than a fixed label.
    let legend = [LegendChip {
        name,
        color: c.traces[0],
        on: true,
    }];
    strip::StripHeader::new("NYQ", &format!("{point_count} pts"), &legend).show(ui);

    // Equal-aspect ranges around the locus and the critical point.
    let mut extent = 1.3f64;
    for (&r, &i) in re.iter().zip(im.iter()) {
        if r.is_finite() && i.is_finite() {
            extent = extent.max(r.abs()).max(i.abs());
        }
    }
    let extent = (extent * 1.1).min(50.0);
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
        Trace::new(re, im, c.traces[0])
            .parametric()
            .cache_key(plot::trace_cache_key(NYQUIST_CACHE_BASE, LOCUS_SERIES)),
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
            rows.push(("N".to_owned(), encirclement_row(s.encirclements)));
            if let Some(d) = s.min_distance {
                rows.push(("min |1+L|".to_owned(), fmt_si(d, "", 2)));
            }
        }
        rows
    };

    let response = plot::show(&mut plot_ui, &spec, cache, None, Some(&readout));
    if response.view.any() {
        let change = crate::presentation::square_xy_view_change((x0, x1), (y0, y1), response.view);
        view.apply(&change);
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
            let pos = crate::presentation::xy_screen_pos(
                response.plot_rect,
                (re[i], im[i]),
                ranges.0,
                ranges.1,
            );
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, i));
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

    let pinned = pin.filter(|(_, i)| *i < re.len());
    let target = hovered.or(pinned);

    if let Some((_, i)) = target {
        let pos = crate::presentation::xy_screen_pos(
            response.plot_rect,
            (re[i], im[i]),
            ranges.0,
            ranges.1,
        );
        let painter = plot_ui.painter();
        if pinned == target {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, c.traces[0]));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let frequency = curve.points.get(i).map(|p| p.frequency).unwrap_or(0.0);
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
        crate::presentation::point_card(
            &plot_ui,
            response.plot_rect,
            pos,
            name,
            c.traces[0],
            &rows,
        );
    }
    Some(response)
}

/// Stability readout.
pub fn right_panel(
    ui: &mut Ui,
    stability: Option<NyquistStability>,
    quantity_policy: &QuantityPresentationPolicy,
) {
    section_header(ui, "Stability", None);
    let Some(s) = stability else {
        crate::presentation::panel_note(ui, NEEDS_LOOP_GAIN);
        return;
    };

    let margin = |margin: Option<NyquistMargin>, render: &dyn Fn(f64) -> String| -> String {
        margin.map_or_else(
            || "—".to_owned(),
            |margin| {
                format!(
                    "{} at {}",
                    render(margin.value),
                    quantity_policy.format_frequency(margin.frequency, 3)
                )
            },
        )
    };
    let rows = [
        (
            "Encirclements N (CW)",
            encirclement_row(s.encirclements),
            true,
        ),
        (
            "Min distance to −1",
            s.min_distance
                .map_or_else(|| "—".to_owned(), |v| fmt_si(v, "", 3)),
            false,
        ),
        (
            "Gain margin",
            margin(s.gain_margin, &|ratio| {
                format!("{:.2} dB", 20.0 * ratio.log10())
            }),
            false,
        ),
        (
            "Phase margin",
            margin(s.phase_margin, &|degrees| {
                quantity_policy.format_angle(degrees.to_radians(), 1)
            }),
            false,
        ),
    ];
    crate::presentation::stat_table(ui, &rows);

    // The fold, before the criterion. This sheet is the one that carries the
    // encirclement count the note points at, so the reader is being sent one
    // row up rather than to another sheet.
    if let Some(margin) = s.phase_margin
        && rspice_results::stability::phase_margin_is_folded(margin.phase_deg)
    {
        crate::presentation::panel_note(
            ui,
            &rspice_results::stability::folded_phase_margin_note(margin.phase_deg),
        );
    }

    crate::presentation::panel_note(ui, &criterion_note(s.encirclements));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The panel states the criterion instead of pronouncing a verdict it
    /// cannot support: P is not something this run retains, so it stays in
    /// the sentence rather than being silently assumed away.
    #[test]
    fn the_criterion_note_carries_the_count_and_names_what_p_needs() {
        let note = criterion_note(EncirclementCount::Counted(2));

        assert!(note.contains("Z = N + P"), "{note}");
        assert!(note.contains("+2"), "{note}");
        assert!(
            note.contains("pole-zero analysis of the loop-broken deck"),
            "{note}"
        );
        assert!(note.contains("With P = 0 the loop is unstable"), "{note}");

        let stable = criterion_note(EncirclementCount::Counted(0));
        assert!(stable.contains("With P = 0 the loop is stable"), "{stable}");
    }

    /// A contour with no winding number gets no criterion applied to it.
    #[test]
    fn a_contour_without_a_winding_number_carries_no_criterion() {
        let note = criterion_note(EncirclementCount::TouchesCriticalPoint);

        assert!(note.contains("cannot be applied"), "{note}");
        assert_eq!(
            encirclement_row(EncirclementCount::TouchesCriticalPoint),
            "on −1 + j0"
        );
        assert_eq!(
            encirclement_row(EncirclementCount::Unresolved { turns: 0.37 }),
            "unresolved (+0.37 turns)"
        );
    }

    /// A negative winding is not an unstable loop — it is a *proof* that the
    /// open loop has right-half-plane poles, since Z = N + P can never be
    /// negative. The note has to say that instead of subtracting from zero.
    #[test]
    fn a_negative_winding_states_the_open_loop_poles_it_proves() {
        let note = criterion_note(EncirclementCount::Counted(-1));

        assert!(note.contains("P ≥ 1"), "{note}");
        assert!(
            note.contains("with P = 1 the loop is stable (Z = 0)"),
            "{note}"
        );
        // The one thing it must never print is a negative pole count.
        assert!(!note.contains("(Z = -"), "{note}");
        assert!(!note.contains("With P = 0"), "{note}");
        assert!(!note.contains("unstable"), "{note}");
    }

    /// A winding the closure guard would not certify gets the reason, not a
    /// verdict: the sweep did not settle at its ends, so the chords that close
    /// the contour are the counter's own invention.
    #[test]
    fn an_uncertified_winding_names_the_unsettled_sweep_instead_of_a_verdict() {
        let note = criterion_note(EncirclementCount::Unresolved { turns: -1.0 });

        assert!(note.contains("-1.00 turns"), "{note}");
        assert!(note.contains("real axis"), "{note}");
        assert!(!note.contains("stable"), "{note}");
    }

    /// The three arrays this sheet caches must not collide with each other,
    /// and the composition is the plot engine's, not a bitwise fold.
    #[test]
    fn the_cached_series_keys_stay_distinct() {
        let keys = [REAL_SERIES, IMAGINARY_SERIES, LOCUS_SERIES]
            .map(|series| plot::trace_cache_key(NYQUIST_CACHE_BASE, series));

        assert_eq!(
            keys.iter().collect::<std::collections::HashSet<_>>().len(),
            3
        );
    }
}
