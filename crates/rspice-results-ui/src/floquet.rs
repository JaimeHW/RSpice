//! Periodic stability on the unit circle: each Floquet multiplier of a PSS
//! or PSTB orbit drawn on the complex plane, beside a table of its
//! magnitude, phase, exponent, verdict and margin.

use crate::presentation::{PlotView, angle_readout, well_hint, with_table_state};
use crate::strip::{self, LegendChip};
use egui::Ui;
use rspice_results::floquet::{
    FloquetOrbitKindEvidence, FloquetStabilityVerdictEvidence, PstbStabilityClassificationEvidence,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, XScale, fmt_si_significant};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable, LineHeights, SortKey, Tone};
use rspice_vocabulary::quantity::{AngleDisplay, QuantityPresentationPolicy};

/// One Floquet mode of the orbit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloquetMode {
    /// The multiplier μ, real and imaginary parts.
    pub multiplier: (f64, f64),
    /// The exponent λ with μ = e^{λT}: its real part in 1/s, its imaginary
    /// part in rad/s.
    pub exponent: (f64, f64),
    /// How much of the mode the probe sees, from 0 to 1, where the run
    /// measured it.
    pub participation: Option<f64>,
    /// The autonomous orbit's phase mode, which sits at 1 by construction.
    pub trivial: bool,
    /// The root of unity the mode sits on, where the run looked for one.
    pub subharmonic: Option<u64>,
}

/// What one mode says about the orbit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeVerdict {
    /// The autonomous orbit's phase mode: no verdict of its own.
    Phase,
    Stable,
    /// On the unit circle, within the run's band.
    Marginal,
    Unstable,
}

impl ModeVerdict {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Phase => "Phase mode",
            Self::Stable => "Stable",
            Self::Marginal => "Marginal",
            Self::Unstable => "Unstable",
        }
    }
}

impl FloquetMode {
    pub fn magnitude(&self) -> f64 {
        self.multiplier.0.hypot(self.multiplier.1)
    }

    pub fn angle_rad(&self) -> f64 {
        self.multiplier.1.atan2(self.multiplier.0)
    }

    /// The decibels the multiplier sits inside the unit circle, −20 log₁₀|μ|:
    /// negative outside it. The phase mode has none.
    pub fn margin_db(&self) -> Option<f64> {
        (!self.trivial).then(|| -20.0 * self.magnitude().log10())
    }

    /// The mode's verdict where a magnitude past 1 + `band` is unstable and
    /// one within `band` of 1 is marginal, as the run classified the orbit.
    pub fn verdict(&self, band: f64) -> ModeVerdict {
        let magnitude = self.magnitude();
        if self.trivial {
            ModeVerdict::Phase
        } else if magnitude > 1.0 + band {
            ModeVerdict::Unstable
        } else if magnitude >= (1.0 - band).max(0.0) {
            ModeVerdict::Marginal
        } else {
            ModeVerdict::Stable
        }
    }
}

/// The exponent of a multiplier over a period of `period_s`: λ = ln(μ)/T on
/// the principal branch.
pub fn exponent_of(multiplier: (f64, f64), period_s: f64) -> (f64, f64) {
    let (re, im) = multiplier;
    (re.hypot(im).ln() / period_s, im.atan2(re) / period_s)
}

/// One orbit's Floquet spectrum and the run's verdict on it.
pub struct FloquetData {
    pub label: String,
    /// The kind of run, as the sheet's header tags it.
    pub kind: &'static str,
    pub modes: Vec<FloquetMode>,
    /// The half-width about the unit circle the run read as marginal; past
    /// 1 + `band` a mode is unstable.
    pub band: f64,
    pub verdict: FloquetStabilityVerdictEvidence,
    pub classification: Option<PstbStabilityClassificationEvidence>,
    pub orbit: FloquetOrbitKindEvidence,
    pub period_s: f64,
    /// The instance whose participation the run measured.
    pub probe: Option<String>,
}

impl FloquetData {
    /// The smallest margin of any mode with one.
    pub fn min_margin_db(&self) -> Option<f64> {
        self.modes
            .iter()
            .filter_map(FloquetMode::margin_db)
            .min_by(f64::total_cmp)
    }

    pub fn unstable_count(&self) -> usize {
        self.modes
            .iter()
            .filter(|mode| mode.verdict(self.band) == ModeVerdict::Unstable)
            .count()
    }
}

pub const fn verdict_label(verdict: FloquetStabilityVerdictEvidence) -> &'static str {
    match verdict {
        FloquetStabilityVerdictEvidence::Stable => "Stable",
        FloquetStabilityVerdictEvidence::Unstable => "Unstable",
        FloquetStabilityVerdictEvidence::Marginal => "Marginal",
        FloquetStabilityVerdictEvidence::Indeterminate => "Indeterminate",
    }
}

/// The bifurcation the run read off the spectrum, where it says more than
/// the verdict does.
const fn classification_label(
    classification: PstbStabilityClassificationEvidence,
) -> Option<&'static str> {
    match classification {
        PstbStabilityClassificationEvidence::UnstableReal => Some("real multiplier"),
        PstbStabilityClassificationEvidence::UnstableComplex => Some("complex pair"),
        PstbStabilityClassificationEvidence::PeriodDoubling => Some("period doubling"),
        PstbStabilityClassificationEvidence::NeimarkSacker => Some("Neimark–Sacker"),
        PstbStabilityClassificationEvidence::SaddleNode => Some("saddle-node"),
        PstbStabilityClassificationEvidence::Stable
        | PstbStabilityClassificationEvidence::Marginal
        | PstbStabilityClassificationEvidence::Indeterminate => None,
    }
}

/// The run's verdict, and the bifurcation where it names one.
pub fn verdict_text(data: &FloquetData) -> String {
    match data.classification.and_then(classification_label) {
        Some(kind) => format!("{} · {kind}", verdict_label(data.verdict)),
        None => verdict_label(data.verdict).to_owned(),
    }
}

/// What the sheet's interaction asked of the host.
pub struct FloquetResponse {
    pub plot: plot::PlotResponse,
    pub fit_clicked: bool,
    pub pin_changed: bool,
}

/// The table goes beside the plane when the sheet leaves this much width
/// past a square plane.
const TABLE_BESIDE_MIN_WIDTH: f32 = 560.0;
/// The most of the height the table takes when it goes under the plane;
/// a spectrum with fewer modes takes only the rows it lists.
const TABLE_SHARE_UNDER_PLANE: f32 = 0.42;

/// Draw the multipliers on the plane with the unit circle, and the table
/// beside it. A mode selected in the table or clicked on the plane is
/// `pin`.
pub fn show(
    ui: &mut Ui,
    data: Option<&FloquetData>,
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> Option<FloquetResponse> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(data) = data else {
        well_hint(
            ui,
            "No Floquet multipliers — run PSTB, or PSS with its stability computed",
        );
        return None;
    };
    let colour = |verdict: ModeVerdict| match verdict {
        ModeVerdict::Phase => c.text_dim,
        ModeVerdict::Stable => c.traces[0],
        ModeVerdict::Marginal => c.warn,
        ModeVerdict::Unstable => c.err,
    };
    let legend = [
        LegendChip {
            name: "stable",
            color: colour(ModeVerdict::Stable),
            on: true,
        },
        LegendChip {
            name: "marginal",
            color: colour(ModeVerdict::Marginal),
            on: true,
        },
        LegendChip {
            name: "unstable",
            color: colour(ModeVerdict::Unstable),
            on: true,
        },
    ];
    let drawing_view = *view;
    // The sheet bar already names the analysis.
    let header = strip::StripHeader::new(
        "FLOQUET",
        &format!("{} multipliers", data.modes.len()),
        &legend,
    )
    .zoomed(drawing_view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        *view = PlotView::default();
    }

    // The unit circle and every multiplier, on one scale.
    let boundary = 1.0 + data.band;
    let extent = data
        .modes
        .iter()
        .map(FloquetMode::magnitude)
        .filter(|magnitude| magnitude.is_finite())
        .fold(boundary, f64::max)
        * 1.15;
    let (x0, x1) = drawing_view.x.unwrap_or((-extent, extent));
    let (y0, y1) = drawing_view.y.unwrap_or((-extent, extent));
    let detail = format!(
        "{} · {} multipliers; {}",
        data.kind,
        data.modes.len(),
        verdict_text(data)
    );
    let mut spec = PlotSpec::new(
        real_axis((x0, x1)),
        XScale::Linear,
        imaginary_axis((y0, y1)),
    )
    .accessible_name("Floquet multipliers on the unit circle")
    .pinch_zooms_both_axes()
    .with_y_without_unit("IM")
    .accessible_detail(&detail);
    spec.ref_lines.push(plot::RefLine { y: 0.0 });

    let places: Vec<(f64, f64)> = data.modes.iter().map(|mode| mode.multiplier).collect();
    let glyphs: Vec<(ModeVerdict, egui::Color32)> = data
        .modes
        .iter()
        .map(|mode| {
            let verdict = mode.verdict(data.band);
            (verdict, colour(verdict))
        })
        .collect();
    let underlay_places = places.clone();
    let (grid, circle) = (c.canvas_grid, c.text_faint);
    let band = data.band;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        painter.vline(
            mapper.x(0.0),
            egui::Rangef::new(mapper.rect.top(), mapper.rect.bottom()),
            egui::Stroke::new(1.0, grid),
        );
        let origin = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let unit = (mapper.x(1.0) - mapper.x(0.0)).abs();
        painter.circle_stroke(origin, unit, egui::Stroke::new(1.2, circle));
        // A boundary the run set past the unit circle, dashed.
        if band > 0.0 {
            let radius = unit * (1.0 + band) as f32;
            let steps = 96;
            for step in (0..steps).step_by(2) {
                let angle = |k: usize| std::f32::consts::TAU * k as f32 / steps as f32;
                let at = |a: f32| origin + radius * egui::vec2(a.cos(), -a.sin());
                painter.line_segment(
                    [at(angle(step)), at(angle(step + 1))],
                    egui::Stroke::new(1.0, circle),
                );
            }
        }
        for (&(x, y), &(verdict, color)) in underlay_places.iter().zip(&glyphs) {
            let center = egui::pos2(mapper.x(x), mapper.y(y));
            if verdict == ModeVerdict::Phase {
                painter.circle_stroke(center, 4.5, egui::Stroke::new(1.8, color));
            } else {
                let r = 4.5;
                let stroke = egui::Stroke::new(1.8, color);
                painter.line_segment(
                    [center + egui::vec2(-r, -r), center + egui::vec2(r, r)],
                    stroke,
                );
                painter.line_segment(
                    [center + egui::vec2(-r, r), center + egui::vec2(r, -r)],
                    stroke,
                );
            }
        }
    }));

    let avail = ui.available_rect_before_wrap();
    let (plane_area, table_area) = if avail.width() - avail.height() >= TABLE_BESIDE_MIN_WIDTH {
        let plane =
            egui::Rect::from_min_size(avail.min, egui::vec2(avail.height(), avail.height()));
        (
            plane,
            egui::Rect::from_min_max(egui::pos2(plane.right(), avail.top()), avail.max),
        )
    } else {
        let lines = LineHeights::grid(&Tokens::get(ui.ctx()));
        // The head, every row and the frame's two rules.
        let rows = lines.head + lines.row * data.modes.len() as f32 + 2.0;
        let table = rows.min(avail.height() * TABLE_SHARE_UNDER_PLANE).round();
        let split = avail.bottom() - table;
        (
            egui::Rect::from_min_max(avail.min, egui::pos2(avail.right(), split)),
            egui::Rect::from_min_max(egui::pos2(avail.left(), split), avail.max),
        )
    };
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(plane_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    // The plane fills its area with one unit as long on both axes, so the
    // circle stays round. Twice: the Y gutter is as wide as the labels of
    // the range it shows.
    let mut ranges = ((x0, x1), (y0, y1));
    for _ in 0..2 {
        let plot = plot::plot_rect(&plot_ui, &spec).size();
        ranges = plot::equal_scale_ranges(plot, (x0, x1), (y0, y1));
        spec.x = real_axis(ranges.0);
        spec.y = imaginary_axis(ranges.1);
    }
    let response = plot::show(&mut plot_ui, &spec, cache, None, None);
    if response.view.any() {
        let change = crate::presentation::equal_scale_view_change(
            ranges.0,
            ranges.1,
            response.view,
            response.plot_rect.size(),
        );
        view.apply(&change);
    }

    let mut hovered = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 16.0f32 * 16.0;
        for (index, &place) in places.iter().enumerate() {
            let pos =
                crate::presentation::xy_screen_pos(response.plot_rect, place, ranges.0, ranges.1);
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, index));
            }
        }
    }
    let mut pin_changed = false;
    if response.response.clicked() {
        *pin = match hovered {
            Some(hit) if *pin == Some(hit) => None,
            hit => hit,
        };
        pin_changed = true;
    }
    let pinned = pin.filter(|(_, index)| *index < data.modes.len());
    if let Some((_, index)) = hovered.or(pinned) {
        let mode = &data.modes[index];
        let pos = crate::presentation::xy_screen_pos(
            response.plot_rect,
            places[index],
            ranges.0,
            ranges.1,
        );
        let color = colour(mode.verdict(data.band));
        let painter = plot_ui.painter();
        if pinned == hovered.or(pinned) {
            painter.circle_stroke(pos, 8.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.5, c.accent));
        let mut rows = vec![
            (
                "|μ|".to_owned(),
                fmt_si_significant(mode.magnitude(), "", digits),
            ),
            (
                "∠μ".to_owned(),
                angle_readout(quantity_policy, mode.angle_rad(), digits),
            ),
        ];
        if let Some(margin) = mode.margin_db() {
            rows.push(("Margin".to_owned(), format!("{} dB", hundredths(margin))));
        }
        let title = format!("μ{}", index + 1);
        crate::presentation::point_card(&plot_ui, response.plot_rect, pos, &title, color, &rows);
    }

    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(table_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    pin_changed |= mode_table(&mut table_ui, data, pin, quantity_policy, digits);
    Some(FloquetResponse {
        plot: response,
        fit_clicked: header.fit_clicked,
        pin_changed,
    })
}

/// A multiplier's real part: dimensionless, named in the X row.
fn real_axis(range: (f64, f64)) -> Axis {
    Axis::linear(range.0, range.1, "").with_label("Re")
}

/// A multiplier's imaginary part: dimensionless, named at the head of the
/// Y column.
fn imaginary_axis(range: (f64, f64)) -> Axis {
    Axis::linear(range.0, range.1, "").with_label("Im")
}

/// A margin to the hundredth of a decibel, as every decibel column of the
/// Results details reads, with a true minus sign.
fn hundredths(decibels: f64) -> String {
    let text = format!("{decibels:.2}");
    if text == "-0.00" {
        return "0.00".to_owned();
    }
    text.replace('-', "\u{2212}")
}

/// A cell holding a number whose column head carries its unit.
fn figure(value: f64, unit: &str, digits: usize) -> Cell {
    Cell::number(crate::phase_noise::plain(value, unit, digits)).sort_by(SortKey::Number(value))
}

/// Every mode in a table, its selection the mode pinned on the plane.
/// Returns whether the reader selected another mode, or none.
fn mode_table(
    ui: &mut Ui,
    data: &FloquetData,
    pin: &mut Option<(usize, usize)>,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> bool {
    let (angle_head, angle_unit) = match quantity_policy.angle_display {
        AngleDisplay::Degrees => ("∠μ (°)", "°"),
        AngleDisplay::Radians => ("∠μ (rad)", "rad"),
    };
    let measured = data.modes.iter().any(|mode| mode.participation.is_some());
    let mut columns = vec![
        Column::number("#", 30.0),
        Column::number("|μ|", 76.0),
        Column::number(angle_head, 76.0),
        Column::number("Re λ (1/s)", 86.0),
        Column::number("Im λ (rad/s)", 86.0),
        Column::number("Margin (dB)", 84.0),
    ];
    if measured {
        columns.push(Column::number("Participation", 92.0));
    }
    // Last, so the spare width falls after its words and the numbers read
    // as one group.
    columns.push(Column::words("Verdict", 110.0));
    let revision = egui::Id::new(("floquet-modes", &data.label, data.modes.len(), digits)).value();
    with_table_state(ui, "floquet-modes", |ui, state| {
        let pinned = pin
            .map(|(_, index)| index)
            .filter(|index| *index < data.modes.len());
        if state.selected_row() != pinned {
            match pinned {
                Some(index) => state.select(index, None),
                None => state.selection = None,
            }
        }
        let response = DataTable::new("rspice.results.floquet.modes", &columns, data.modes.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(ui.available_height())
            .show(ui, |index| {
                let mode = &data.modes[index];
                let angle = match quantity_policy.angle_display {
                    AngleDisplay::Degrees => mode.angle_rad().to_degrees(),
                    AngleDisplay::Radians => mode.angle_rad(),
                };
                let verdict = mode.verdict(data.band);
                let mut words = verdict.label().to_owned();
                if let Some(order) = mode.subharmonic {
                    words.push_str(&format!(" · 1/{order} subharmonic"));
                }
                let mut verdict_cell = Cell::text(words).sort_by(SortKey::Status(match verdict {
                    ModeVerdict::Phase => 0,
                    ModeVerdict::Stable => 1,
                    ModeVerdict::Marginal => 2,
                    ModeVerdict::Unstable => 3,
                }));
                verdict_cell = match verdict {
                    ModeVerdict::Unstable => verdict_cell.toned(Tone::Error),
                    ModeVerdict::Marginal => verdict_cell.toned(Tone::Caution),
                    ModeVerdict::Stable | ModeVerdict::Phase => verdict_cell,
                };
                let mut cells = vec![
                    Cell::number((index + 1).to_string())
                        .sort_by(SortKey::Number((index + 1) as f64)),
                    figure(mode.magnitude(), "", digits),
                    figure(angle, angle_unit, digits),
                    figure(mode.exponent.0, "1/s", digits),
                    figure(mode.exponent.1, "rad/s", digits),
                    mode.margin_db().map_or_else(Cell::absent, |margin| {
                        Cell::number(hundredths(margin)).sort_by(SortKey::Number(margin))
                    }),
                ];
                if measured {
                    cells.push(
                        mode.participation
                            .map_or_else(Cell::absent, |share| figure(share, "", digits)),
                    );
                }
                cells.push(verdict_cell);
                cells
            });
        if response.selection_changed {
            *pin = state.selected_row().map(|index| (0, index));
            return true;
        }
        false
    })
}

/// The run's verdict and what makes it, at the reader's `digits`.
pub fn right_panel(ui: &mut Ui, data: Option<&FloquetData>, digits: usize) {
    section_header(ui, "Stability", None);
    let Some(data) = data else {
        crate::presentation::panel_note(
            ui,
            "Multipliers appear once a PSTB run, or a PSS run with its stability computed, finishes.",
        );
        return;
    };
    let mut rows: Vec<(&str, String, bool)> = vec![
        ("Verdict", verdict_text(data), true),
        ("Unstable modes", data.unstable_count().to_string(), false),
        (
            "Min margin",
            data.min_margin_db().map_or_else(
                || "—".to_owned(),
                |margin| format!("{} dB", hundredths(margin)),
            ),
            false,
        ),
        (
            "Boundary |μ|",
            fmt_si_significant(1.0 + data.band, "", digits),
            false,
        ),
        (
            "Orbit",
            match data.orbit {
                FloquetOrbitKindEvidence::Driven => "Driven",
                FloquetOrbitKindEvidence::Autonomous => "Autonomous",
            }
            .to_owned(),
            false,
        ),
        (
            "Period",
            fmt_si_significant(data.period_s, "s", digits),
            false,
        ),
    ];
    if let Some(probe) = &data.probe {
        rows.push(("Probe", probe.clone(), false));
    }
    crate::presentation::stat_table(ui, &rows);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(re: f64, im: f64) -> FloquetMode {
        FloquetMode {
            multiplier: (re, im),
            exponent: exponent_of((re, im), 1.0e-6),
            participation: None,
            trivial: false,
            subharmonic: None,
        }
    }

    #[test]
    fn a_mode_reads_its_margin_exponent_and_verdict_where_the_closed_form_puts_them() {
        // μ = −1.25: 1.938 dB outside the circle, λ = (ln 1.25 + jπ)/T.
        let unstable = mode(-1.25, 0.0);
        assert!((unstable.margin_db().unwrap() + 20.0 * 1.25_f64.log10()).abs() < 1e-12);
        assert_eq!(hundredths(unstable.margin_db().unwrap()), "\u{2212}1.94");
        assert!((unstable.exponent.0 - 1.25_f64.ln() / 1.0e-6).abs() < 1e-3);
        assert!((unstable.exponent.1 - std::f64::consts::PI / 1.0e-6).abs() < 1e-3);
        assert_eq!(unstable.verdict(0.0), ModeVerdict::Unstable);
        // |0.4 ± 0.3j| = 0.5: 6.021 dB inside.
        let inside = mode(0.4, 0.3);
        assert!((inside.margin_db().unwrap() - 20.0 * 2.0_f64.log10()).abs() < 1e-12);
        assert_eq!(inside.verdict(0.0), ModeVerdict::Stable);
        // Within the run's band of the circle the mode is marginal, and
        // only past it unstable.
        assert_eq!(
            mode(1.0 + 5.0e-7, 0.0).verdict(1.0e-6),
            ModeVerdict::Marginal
        );
        assert_eq!(mode(1.0 + 5.0e-7, 0.0).verdict(0.0), ModeVerdict::Unstable);
        let phase = FloquetMode {
            trivial: true,
            ..mode(1.0, 0.0)
        };
        assert_eq!(phase.verdict(0.0), ModeVerdict::Phase);
        assert_eq!(phase.margin_db(), None);
    }
}
