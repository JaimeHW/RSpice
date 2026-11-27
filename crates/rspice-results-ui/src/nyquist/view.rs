//! Nyquist locus rendering and stability readout over explicit retained data.

use egui::Ui;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_column_reading};
use rspice_ui_kit::tokens::Tokens;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use super::{EncirclementCount, NyquistData, NyquistMargin};
use crate::presentation::{PlotView, angle_readout, frequency_readout, well_hint};
use crate::strip::{self, LegendChip};

/// Measurements from the retained locus, cached by the host data version.
#[derive(Debug, Clone, Copy)]
pub struct NyquistStability {
    pub encirclements: EncirclementCount,
    pub min_distance: Option<f64>,
    pub gain_margin: Option<NyquistMargin>,
    pub phase_margin: Option<NyquistMargin>,
}

/// One nonempty retained loop-gain contour, the locus drawn through it and
/// that locus's cached coordinate columns. The columns must contain the
/// corresponding coordinate of every point of `drawn`; `mirror` is the
/// negated imaginary column, the branch for negative frequencies.
pub struct NyquistPlot<'a> {
    /// The retained samples.
    pub curve: &'a NyquistData,
    /// The locus as drawn, on the interpolant the margins are read from.
    pub drawn: &'a NyquistData,
    pub real: &'a [f64],
    pub imaginary: &'a [f64],
    pub mirror: &'a [f64],
    pub stability: Option<NyquistStability>,
}

/// Where the plot is framed, as the reader asks from the sheet's bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NyquistFrame {
    /// The critical point and the unit circle, where the margins are read:
    /// the frame the sheet opens on.
    CriticalPoint,
    /// Every sample of the locus and its mirror.
    WholeLocus,
}

/// The frame the sheet opens on: the critical point, the unit circle the
/// phase margin is read on, and the point the gain margin is read at, with
/// room on the far side of −1, all on one scale.
///
/// A loop with 60 dB of gain at DC starts a thousand units out. Framing all
/// of it shrinks the unit circle to a few pixels about the origin, where
/// every question a Nyquist plot answers is asked.
pub fn critical_frame(stability: Option<NyquistStability>) -> ((f64, f64), (f64, f64)) {
    let (mut left, mut right) = (-2.0f64, 1.0f64);
    if let Some(margin) = stability.and_then(|stability| stability.gain_margin) {
        let crossing = -1.0 / margin.value;
        if crossing.is_finite() {
            left = left.min(crossing);
            right = right.max(crossing);
        }
    }
    let half = (right - left) * 0.5 * 1.1;
    let center = (left + right) * 0.5;
    ((center - half, center + half), (-half, half))
}

/// A square frame round every finite sample of the locus and its mirror,
/// the critical point and the unit circle.
pub fn locus_frame(real: &[f64], imaginary: &[f64]) -> ((f64, f64), (f64, f64)) {
    let (mut left, mut right, mut reach) = (-1.0f64, 1.0f64, 1.0f64);
    for (&r, &i) in real.iter().zip(imaginary) {
        if r.is_finite() && i.is_finite() {
            left = left.min(r);
            right = right.max(r);
            reach = reach.max(i.abs());
        }
    }
    let half = ((right - left) * 0.5).max(reach) * 1.05;
    let center = (left + right) * 0.5;
    ((center - half, center + half), (-half, half))
}

/// The sheet's own controls in its bar: frame the critical point, as the
/// sheet opens, or the whole locus.
pub fn domain_bar(ui: &mut Ui) -> Option<NyquistFrame> {
    crate::chrome::bars::bar_label(ui, "Frame");
    let mut frame = None;
    for (label, choice) in [
        ("Critical point", NyquistFrame::CriticalPoint),
        ("Whole locus", NyquistFrame::WholeLocus),
    ] {
        if rspice_ui_kit::widgets::Button::new(label)
            .show(ui)
            .clicked()
        {
            frame = Some(choice);
        }
    }
    frame
}

/// What the tab needs, spelled the way the reader can act on it.
///
/// The locus is a *loop gain*, and the numbers beside it are the Nyquist
/// criterion applied to one. An AC node response or a harmonic-balance
/// spectrum is not a loop gain, and drawing one here with stability numbers
/// attached would put a verdict on a quantity that has none — so nothing but
/// a retained loop-gain contour reaches this sheet.
/// The legend's name for the branch drawn for negative frequencies.
const MIRROR_LABEL: &str = "L(\u{2212}jω)";

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
    if n < 0 {
        let p = -n;
        return format!(
            "N = {n:+}: a counter-clockwise winding proves the open loop has at least {p} \
             right-half-plane pole(s), and with exactly {p} the loop is stable."
        );
    }
    let z = super::closed_loop_rhp_poles(n, 0);
    let verdict = if z == 0 { "stable" } else { "unstable" };
    format!("N = {n:+}: {verdict} if the open loop has no right-half-plane poles.")
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
const MIRROR_SERIES: usize = 3;

/// Render a retained locus using host-selected data and local interaction state.
/// The host owns cache invalidation and applies the returned axes to its inspector.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    source: Option<NyquistPlot<'_>>,
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
    kind_tag: &str,
) -> Option<plot::PlotResponse> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(source) = source else {
        well_hint(ui, NEEDS_LOOP_GAIN);
        return None;
    };
    let NyquistPlot {
        curve,
        drawn,
        real: re,
        imaginary: im,
        mirror,
        stability: stats,
    } = source;
    let name = &curve.name;
    let point_count = curve.len();

    // The legend names the quantity that is actually plotted, which is the
    // retained contour's own name rather than a fixed label.
    let legend = [
        LegendChip {
            name,
            color: c.traces[0],
            on: true,
            glyph: strip::LegendGlyph::Line,
        },
        LegendChip {
            name: MIRROR_LABEL,
            color: c.traces[0],
            on: true,
            glyph: strip::LegendGlyph::Line,
        },
    ];
    strip::StripHeader::new(kind_tag, &format!("{point_count} pts"), &legend).show(ui);

    // Opened on the critical point; the reader's zoom, or the bar's whole
    // locus, replaces it until a double-click brings it back.
    let (frame_x, frame_y) = critical_frame(stats);
    let (x0, x1) = view.x.unwrap_or(frame_x);
    let (y0, y1) = view.y.unwrap_or(frame_y);

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Re"),
        XScale::Linear,
        Axis::linear(y0, y1, "Im"),
    )
    .accessible_name("Nyquist plot")
    .pinch_zooms_both_axes();
    // Both axes through the origin, alike.
    spec.ref_lines.push(plot::RefLine { y: 0.0 });
    spec.x_ref_lines.push(0.0);
    spec.traces.push(
        // Re L(jω) is a coordinate, not an ordering — the locus encircles.
        Trace::new(re, im, c.traces[0])
            .parametric()
            .cache_key(plot::trace_cache_key(NYQUIST_CACHE_BASE, LOCUS_SERIES)),
    );
    // The branch for negative frequencies closes the contour the
    // encirclements are counted round, so it is drawn, dashed.
    spec.traces.push(
        Trace::new(re, mirror, c.traces[0])
            .parametric()
            .dashed()
            .thin()
            .cache_key(plot::trace_cache_key(NYQUIST_CACHE_BASE, MIRROR_SERIES)),
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

    // Each margin where it is read on the locus: the gain margin where the
    // locus crosses the negative real axis, the phase margin where it
    // crosses the unit circle.
    if let Some(stability) = stats {
        if let Some(margin) = stability.gain_margin {
            let crossing = -1.0 / margin.value;
            if crossing.is_finite() {
                spec.markers.push(plot::Marker::point(
                    crossing,
                    0.0,
                    c.warn,
                    format!(
                        "GM {}",
                        fmt_column_reading(20.0 * margin.value.log10(), "dB", significant_digits)
                    ),
                ));
            }
        }
        if let Some(margin) = stability.phase_margin {
            let (sin, cos) = margin.phase_deg.to_radians().sin_cos();
            spec.markers.push(plot::Marker::point(
                cos,
                sin,
                c.warn,
                format!(
                    "PM {}",
                    angle_readout(
                        quantity_policy,
                        margin.value.to_radians(),
                        significant_digits
                    )
                ),
            ));
        }
    }

    // The unit circle underlay.
    let grid = c.canvas_grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let radius = (mapper.x(1.0) - mapper.x(0.0)).abs();
        painter.circle_stroke(center, radius, egui::Stroke::new(1.0, grid));
    }));

    // The plane fills its region with Re and Im on one scale, as stability
    // reading needs the unit circle round and the locus true to it: the
    // long side's range widens, the plot never shrinks to a square. Twice:
    // the Y gutter is as wide as the labels of the range it shows.
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(ui.available_rect_before_wrap())
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let mut ranges = ((x0, x1), (y0, y1));
    for _ in 0..2 {
        let plot = plot::plot_rect(&plot_ui, &spec).size();
        ranges = plot::equal_scale_ranges(plot, (x0, x1), (y0, y1));
        spec.x = Axis::linear(ranges.0.0, ranges.0.1, "Re");
        spec.y = Axis::linear(ranges.1.0, ranges.1.1, "Im");
    }

    let stats_for_readout = stats;
    let readout = move |_x: f64| -> Vec<(String, String)> {
        let mut rows = Vec::new();
        if let Some(s) = stats_for_readout {
            rows.push(("N".to_owned(), encirclement_row(s.encirclements)));
            if let Some(d) = s.min_distance {
                rows.push((
                    "min |1+L|".to_owned(),
                    fmt_column_reading(d, "", significant_digits),
                ));
            }
        }
        rows
    };

    let response = plot::show(&mut plot_ui, &spec, cache, None, Some(&readout));
    if response.view.any() {
        let change = crate::presentation::equal_scale_view_change(
            ranges.0,
            ranges.1,
            response.view,
            response.plot_rect.size(),
        );
        view.apply(&change);
    }

    // Nearest locus point on hover, click to pin — gain/phase/frequency at
    // a spot on the locus is the question a Nyquist plot exists to answer.
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

    // The retained samples, while the pointer is over the plot: the curve
    // between them is the interpolant the margins are read on, not data.
    if response
        .response
        .hover_pos()
        .is_some_and(|pointer| response.plot_rect.contains(pointer))
    {
        let painter = plot_ui.painter().with_clip_rect(response.plot_rect);
        for point in &curve.points {
            let pos = crate::presentation::xy_screen_pos(
                response.plot_rect,
                (point.real, point.imag),
                ranges.0,
                ranges.1,
            );
            painter.circle_filled(pos, 2.5, c.traces[0]);
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

        let frequency = drawn.points.get(i).map(|p| p.frequency).unwrap_or(0.0);
        let magnitude = (re[i] * re[i] + im[i] * im[i]).sqrt();
        let phase_radians = im[i].atan2(re[i]);
        let rows = [
            (
                "f".to_owned(),
                frequency_readout(quantity_policy, frequency, significant_digits),
            ),
            (
                "Re".to_owned(),
                fmt_column_reading(re[i], "", significant_digits),
            ),
            (
                "Im".to_owned(),
                fmt_column_reading(im[i], "", significant_digits),
            ),
            (
                "|L| ∠".to_owned(),
                crate::presentation::polar_readout(
                    quantity_policy,
                    magnitude,
                    "",
                    phase_radians,
                    significant_digits,
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

/// The loop's margins and the crossovers they are read at, as the card
/// rows them. The gain margin is a factor here; both cards print it in
/// decibels.
fn margin_rows(
    s: &NyquistStability,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> [(&'static str, String, bool); 4] {
    crate::bode::margins::rows(
        s.phase_margin.map(|margin| crate::bode::margins::Margin {
            value: margin.value,
            hertz: margin.frequency,
        }),
        s.gain_margin.map(|margin| crate::bode::margins::Margin {
            value: 20.0 * margin.value.log10(),
            hertz: margin.frequency,
        }),
        quantity_policy,
        significant_digits,
    )
}

/// The figures a printed Nyquist chart carries under its plot: the loop's
/// margins and crossovers, worded and formatted as the card shows them.
pub fn card_figures(
    stability: &NyquistStability,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<(&'static str, String)> {
    margin_rows(stability, quantity_policy, significant_digits)
        .into_iter()
        .map(|(label, value, _)| (label, value))
        .collect()
}

/// Stability readout.
pub fn right_panel(
    ui: &mut Ui,
    stability: Option<NyquistStability>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    section_header(ui, "Stability", None);
    let Some(s) = stability else {
        crate::presentation::panel_note(ui, NEEDS_LOOP_GAIN);
        return;
    };

    let mut rows = vec![
        (
            "Encirclements N (CW)",
            encirclement_row(s.encirclements),
            true,
        ),
        (
            "Min distance to −1",
            s.min_distance.map_or_else(
                || "—".to_owned(),
                |v| fmt_column_reading(v, "", significant_digits),
            ),
            false,
        ),
    ];
    rows.extend(margin_rows(&s, quantity_policy, significant_digits));
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
    /// cannot support: P is not something this run retains, so the verdict
    /// stays conditional on it rather than being silently assumed away.
    #[test]
    fn the_criterion_note_carries_the_count_and_names_what_p_needs() {
        let note = criterion_note(EncirclementCount::Counted(2));

        assert!(note.contains("N = +2"), "{note}");
        assert!(
            note.contains("unstable if the open loop has no right-half-plane poles"),
            "{note}"
        );

        let stable = criterion_note(EncirclementCount::Counted(0));
        assert!(
            stable.contains("stable if the open loop has no right-half-plane poles"),
            "{stable}"
        );
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

        assert!(note.contains("at least 1 right-half-plane pole"), "{note}");
        assert!(note.contains("with exactly 1 the loop is stable"), "{note}");
        // The one thing it must never print is a negative pole count.
        assert!(!note.contains("(Z = -"), "{note}");
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
        let keys = [REAL_SERIES, IMAGINARY_SERIES, LOCUS_SERIES, MIRROR_SERIES]
            .map(|series| plot::trace_cache_key(NYQUIST_CACHE_BASE, series));

        assert_eq!(
            keys.iter().collect::<std::collections::HashSet<_>>().len(),
            4
        );
    }

    /// The sheet opens on the critical point and the unit circle whatever
    /// the loop's gain at DC, and widens only to keep a gain margin's
    /// crossing beyond −1 in view.
    #[test]
    fn the_sheet_opens_framed_on_the_critical_point() {
        let stability = |ratio: f64| NyquistStability {
            encirclements: EncirclementCount::Counted(0),
            min_distance: None,
            gain_margin: Some(NyquistMargin {
                value: ratio,
                frequency: 1.0e3,
                phase_deg: -180.0,
            }),
            phase_margin: None,
        };
        let contains = |((x0, x1), (y0, y1)): ((f64, f64), (f64, f64)), x: f64, y: f64| {
            x0 < x && x < x1 && y0 < y && y < y1
        };

        let stable = critical_frame(Some(stability(2.0)));
        for (x, y) in [
            (-1.0, 0.0),
            (1.0, 0.0),
            (0.0, 1.0),
            (0.0, -1.0),
            (-2.0, 0.0),
        ] {
            assert!(contains(stable, x, y), "{stable:?} holds ({x}, {y})");
        }
        assert!((stable.0.1 - stable.0.0 - (stable.1.1 - stable.1.0)).abs() < 1e-12);
        assert!(stable.0.1 - stable.0.0 < 4.0, "{stable:?}");

        // A loop past the critical point crosses the axis at −4: in view.
        let unstable = critical_frame(Some(stability(0.25)));
        assert!(contains(unstable, -4.0, 0.0), "{unstable:?}");
        assert!(contains(unstable, 1.0, 0.0), "{unstable:?}");

        // The whole locus is the bar's other frame.
        let whole = locus_frame(&[1000.0, -0.5, 0.0], &[0.0, -40.0, 0.0]);
        assert!(contains(whole, 1000.0, 0.0) && contains(whole, -0.5, -40.0));
        assert!(contains(whole, -0.5, 40.0), "the mirror is in the frame");
    }
}
