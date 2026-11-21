//! Retained pole-zero map, root readouts and local plot interaction.

use crate::{
    presentation::{PlotView, frequency_readout, well_hint, with_table_state},
    strip::{self, LegendChip},
};
use egui::Ui;
use rspice_results::pole_zero::{
    data::{ComplexRoot, PoleZeroData},
    metrics::{pole_stability, summarize_roots},
};
use rspice_ui_kit::{
    panels::section_header,
    plot::{self, Axis, PlotSpec, XScale, fmt_si_significant},
    tokens::Tokens,
    widgets::{
        SegmentedWidth, segmented,
        table::{Align, Cell, Column, DataTable, SortKey},
    },
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

/// The plot response and header reset request to apply to the host's shared view.
pub struct PoleZeroResponse {
    pub plot: plot::PlotResponse,
    pub fit_clicked: bool,
    /// The reader pinned a root, or let one go, on the map or in the table.
    pub pin_changed: bool,
}

/// Where the map puts a root.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Plane {
    /// σ and jω as they are.
    Linear,
    /// The angle kept and the radius `1 + log10(|s| / floor)`: the
    /// half-planes, the axes and the damping ratio read as on the linear
    /// plane, and roots decades apart stand apart instead of collapsing onto
    /// the origin beside the largest. `floor` is the decade at or below the
    /// smallest nonzero root, so every nonzero root sits outside the unit
    /// radius and a root at the origin stays there.
    Logarithmic { floor: f64 },
}

impl Plane {
    fn of(data: &PoleZeroData, logarithmic: bool) -> Self {
        if !logarithmic {
            return Self::Linear;
        }
        let smallest = data
            .roots
            .iter()
            .map(ComplexRoot::natural_frequency)
            .filter(|magnitude| magnitude.is_finite() && *magnitude > 0.0)
            .fold(f64::INFINITY, f64::min);
        let floor = if smallest.is_finite() {
            10f64.powf(smallest.log10().floor())
        } else {
            1.0
        };
        Self::Logarithmic { floor }
    }

    /// A root's place on the map.
    fn place(self, real: f64, imaginary: f64) -> (f64, f64) {
        let Self::Logarithmic { floor } = self else {
            return (real, imaginary);
        };
        let magnitude = real.hypot(imaginary);
        if !(magnitude.is_finite() && magnitude > 0.0) {
            return (real, imaginary);
        }
        let radius = 1.0 + (magnitude / floor).log10().max(0.0);
        (real / magnitude * radius, imaginary / magnitude * radius)
    }

    /// The axis over `[min, max]` of the map, quantity `label`, in rad/s.
    fn axis(self, min: f64, max: f64, label: &str) -> Axis {
        let Self::Logarithmic { floor } = self else {
            return Axis::linear(min, max, "rad/s").with_label(label);
        };
        // A tick where each decade's circle crosses the axis, labelled with
        // the value there, which on an axis is |s| signed.
        let reach = min.abs().max(max.abs()).ceil().max(1.0) as i64;
        let step = (reach / 4).max(1);
        let mut axis = Axis::linear(min, max, "rad/s").with_label(label);
        axis.ticks = (-reach..=reach)
            .filter(|k| k % step == 0)
            .map(|k| k as f64)
            .filter(|k| (min..=max).contains(k))
            .map(|k| {
                let text = if k == 0.0 {
                    "0".to_owned()
                } else {
                    rspice_results::number_format::si_tick_label(
                        k.signum() * floor * 10f64.powf(k.abs() - 1.0),
                        "",
                        3,
                    )
                };
                (k, text)
            })
            .collect();
        axis
    }
}

/// The sheet's own control in its bar: the plane on linear or logarithmic
/// axes. Returns the reader's new choice, `true` for logarithmic.
pub fn domain_bar(ui: &mut Ui, logarithmic: bool) -> Option<bool> {
    crate::chrome::bars::bar_label(ui, "Plane");
    let mut selected = usize::from(logarithmic);
    segmented(
        ui,
        "rspice.results.pole-zero.plane",
        &["Linear", "Logarithmic"],
        &mut selected,
        SegmentedWidth::Natural,
    )
    .then_some(selected == 1)
}

fn accessible_root_inventory(data: &PoleZeroData) -> String {
    data.roots
        .iter()
        .enumerate()
        .map(|(index, root)| {
            format!(
                "{} {} sigma {:.9e}, omega {:.9e}",
                if root.is_pole() { "pole" } else { "zero" },
                index + 1,
                root.real,
                root.imag
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// The dominant pole as a frequency: a real one signed by its half-plane, a
/// pair by its natural frequency, whose damping reads on the row after. The
/// table holds its σ and jω.
fn format_dominant_pole(
    root: &ComplexRoot,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> String {
    if root.is_real() {
        real_root_text(root, quantity_policy, significant_digits)
    } else {
        frequency_readout(
            quantity_policy,
            root.natural_frequency() / std::f64::consts::TAU,
            significant_digits,
        )
    }
}

/// A real root as the frequency of its real part, signed: `+1 kHz` sits in
/// the right half-plane and `−1 kHz` in the left, so a Miller zero never
/// reads like its stable mirror.
fn real_root_text(
    root: &ComplexRoot,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> String {
    let sign = if root.real < 0.0 { "−" } else { "+" };
    format!(
        "{sign}{}",
        frequency_readout(
            quantity_policy,
            root.real.abs() / std::f64::consts::TAU,
            significant_digits
        )
    )
}

/// The root table's names: poles p1, p2… and zeros z1, z2… in retained
/// order, each root of a conjugate pair on its own row.
fn root_names(data: &PoleZeroData) -> Vec<String> {
    let (mut poles, mut zeros) = (0usize, 0usize);
    data.roots
        .iter()
        .map(|root| {
            if root.is_pole() {
                poles += 1;
                format!("p{poles}")
            } else {
                zeros += 1;
                format!("z{zeros}")
            }
        })
        .collect()
}

/// The root table's columns: σ and jω in rad/s as the s-plane measures
/// them, f₀ in Hz, and ζ and Q of a root off the real axis.
/// The root column's width.
const ROOT_NAME_WIDTH: f32 = 56.0;
/// Each figure column's least width: a signed seven-digit value in krad/s.
const ROOT_FIGURE_WIDTH: f32 = 128.0;

/// The name, then five figure columns of one width that share what the
/// sheet leaves, so no column takes the spare and none is narrower than its
/// widest value.
fn root_columns() -> [Column; 6] {
    let figure = |head: &str| Column::name(head, ROOT_FIGURE_WIDTH).align(Align::Right);
    [
        Column::words("Root", ROOT_NAME_WIDTH),
        figure("σ"),
        figure("jω"),
        figure("f₀"),
        figure("ζ"),
        figure("Q"),
    ]
}

/// Q in a table cell: [`q_text`] without the words a cell has no room for.
fn q_cell(q: f64, digits: usize) -> String {
    if q.is_infinite() {
        "∞".to_owned()
    } else {
        q_text(q, digits)
    }
}

/// One root's row: its name, σ and jω in rad/s, f₀ in Hz, and ζ and Q when
/// it sits off the real axis.
fn root_cells(
    name: &str,
    root: &ComplexRoot,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<Cell> {
    let natural = root.natural_frequency();
    let mut cells = vec![
        Cell::text(name).sort_by(SortKey::Text(name.to_owned())),
        Cell::number(fmt_si_significant(root.real, "rad/s", digits))
            .sort_by(SortKey::Number(root.real)),
        Cell::number(fmt_si_significant(root.imag, "rad/s", digits))
            .sort_by(SortKey::Number(root.imag)),
        Cell::number(frequency_readout(
            quantity_policy,
            natural / std::f64::consts::TAU,
            digits,
        ))
        .sort_by(SortKey::Number(natural)),
    ];
    if root.is_real() {
        cells.extend([Cell::empty(), Cell::empty()]);
    } else {
        let zeta = root.damping_ratio();
        let q = if root.real.abs() > 1e-12 {
            natural / (-2.0 * root.real)
        } else {
            f64::INFINITY
        };
        cells.push(
            Cell::number(fmt_si_significant(zeta, "", digits)).sort_by(SortKey::Number(zeta)),
        );
        cells.push(Cell::number(q_cell(q, digits)).sort_by(SortKey::Number(q)));
    }
    cells
}

/// Q = 1/(2ζ) as a card prints it. A growing pair has ζ < 0, so its Q keeps
/// its minus sign and says where it sits rather than reading like the
/// decaying pair of the same magnitude.
fn q_text(q: f64, digits: usize) -> String {
    if q.is_infinite() {
        "∞ · imaginary-axis pole".to_owned()
    } else if q < 0.0 {
        format!("{} · RHP", fmt_si_significant(q, "", digits))
    } else {
        fmt_si_significant(q, "", digits)
    }
}

/// The hover card of one root: σ and jω in rad/s, as the s-plane measures
/// them, beside f₀ in Hz, and the signed Q of a complex root off the axis.
fn root_card_rows(
    root: &ComplexRoot,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<(String, String)> {
    let magnitude = root.natural_frequency();
    let mut rows = vec![
        (
            "σ".to_owned(),
            fmt_si_significant(root.real, "rad/s", digits),
        ),
        (
            "jω".to_owned(),
            fmt_si_significant(root.imag, "rad/s", digits),
        ),
        (
            "f₀".to_owned(),
            frequency_readout(quantity_policy, magnitude / std::f64::consts::TAU, digits),
        ),
    ];
    if root.imag.abs() > 1e-12 && root.real.abs() > 1e-12 {
        rows.push((
            "Q".to_owned(),
            q_text(magnitude / (-2.0 * root.real), digits),
        ));
    }
    rows
}

/// The least width beside the square map the root table takes, every
/// column at its least; on a narrower sheet the table goes under the map.
const TABLE_BESIDE_MIN_WIDTH: f32 = ROOT_NAME_WIDTH + 5.0 * ROOT_FIGURE_WIDTH;
/// The share of a narrow sheet's height the map takes above the table.
const MAP_SHARE_ABOVE_TABLE: f32 = 0.58;

/// Render the s-plane map, on logarithmic radial axes when `logarithmic`,
/// and the table of every root beside or under it; the root card and the
/// table read at the reader's `significant_digits`.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    data: Option<&PoleZeroData>,
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
    logarithmic: bool,
) -> Option<PoleZeroResponse> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(data) = data else {
        well_hint(ui, "No root data — run a pole-zero analysis");
        return None;
    };

    let legend = [
        LegendChip {
            name: "poles x",
            color: c.traces[5],
            on: true,
        },
        LegendChip {
            name: "zeros",
            color: c.traces[1],
            on: true,
        },
    ];
    let drawing_view = *view;
    let header = strip::StripHeader::new(
        "PZ",
        &format!("{} · {} roots", data.name, data.roots.len()),
        &legend,
    )
    .zoomed(drawing_view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        *view = PlotView::default();
    }

    // Symmetric ranges around the roots, where the plane puts them.
    let plane = Plane::of(data, logarithmic);
    let places: Vec<(f64, f64)> = data
        .roots
        .iter()
        .map(|root| plane.place(root.real, root.imag))
        .collect();
    let mut extent = 1.0f64;
    for &(x, y) in &places {
        if x.is_finite() && y.is_finite() {
            extent = extent.max(x.abs()).max(y.abs());
        }
    }
    let extent = extent * if plane == Plane::Linear { 1.25 } else { 1.1 };
    let (x0, x1) = drawing_view.x.unwrap_or((-extent, extent));
    let (y0, y1) = drawing_view.y.unwrap_or((-extent, extent));

    let summary = summarize_roots(data);
    let inventory = accessible_root_inventory(data);
    let accessible_detail = format!(
        "{} poles and {} zeros; {} pole stability verdict; stable left half-plane shaded; exact retained roots: {inventory}",
        summary.pole_count,
        summary.zero_count,
        pole_stability(data).label()
    );
    let mut spec = PlotSpec::new(
        plane.axis(x0, x1, "σ"),
        XScale::Linear,
        plane.axis(y0, y1, "jω"),
    )
    .accessible_name("Pole-zero plot")
    .pinch_zooms_both_axes()
    .accessible_detail(&accessible_detail);
    spec.ref_lines.push(plot::RefLine { y: 0.0 });
    // Stable left half-plane.
    spec.bands.push(plot::Band {
        x0,
        x1: 0.0_f64.clamp(x0, x1),
    });

    // Root glyphs as an underlay: ✕ for poles, ○ for zeros; on the
    // logarithmic plane, a circle at each decade.
    let roots = &data.roots;
    let underlay_places = &places;
    let decades = match plane {
        Plane::Linear => 0,
        Plane::Logarithmic { .. } => extent.ceil() as usize,
    };
    let pole_color = c.traces[5];
    let zero_color = c.traces[1];
    let grid = c.canvas_grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        painter.vline(
            mapper.x(0.0),
            egui::Rangef::new(mapper.rect.top(), mapper.rect.bottom()),
            egui::Stroke::new(1.0, grid),
        );
        let origin = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        for decade in 1..=decades {
            let radius = (mapper.x(decade as f64) - mapper.x(0.0)).abs();
            painter.circle_stroke(origin, radius, egui::Stroke::new(1.0, grid));
        }
        for (root, &(x, y)) in roots.iter().zip(underlay_places) {
            let center = egui::pos2(mapper.x(x), mapper.y(y));
            if root.is_pole() {
                let r = 4.5;
                let stroke = egui::Stroke::new(1.8, pole_color);
                painter.line_segment(
                    [center + egui::vec2(-r, -r), center + egui::vec2(r, r)],
                    stroke,
                );
                painter.line_segment(
                    [center + egui::vec2(-r, r), center + egui::vec2(r, -r)],
                    stroke,
                );
            } else {
                painter.circle_stroke(center, 4.5, egui::Stroke::new(1.8, zero_color));
            }
        }
    }));

    // The map's region sized so the INNER plot area is square — σ and jω on
    // one scale — and the table beside it, or under it on a narrow sheet.
    let avail = ui.available_rect_before_wrap();
    let (map_area, table_area) = if avail.width() - avail.height() >= TABLE_BESIDE_MIN_WIDTH {
        let map = egui::Rect::from_min_size(avail.min, egui::vec2(avail.height(), avail.height()));
        (
            map,
            egui::Rect::from_min_max(egui::pos2(map.right(), avail.top()), avail.max),
        )
    } else {
        let split = avail.top() + (avail.height() * MAP_SHARE_ABOVE_TABLE).round();
        (
            egui::Rect::from_min_max(avail.min, egui::pos2(avail.right(), split)),
            egui::Rect::from_min_max(egui::pos2(avail.left(), split), avail.max),
        )
    };
    let rect = plot::square_outer_rect(map_area, &spec);
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

    // Nearest root on hover, click to pin: σ, jω, natural frequency, and Q
    // turn the s-plane picture into numbers.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 16.0f32 * 16.0;
        for (i, &place) in places.iter().enumerate() {
            let pos =
                crate::presentation::xy_screen_pos(response.plot_rect, place, ranges.0, ranges.1);
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, i));
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

    let pinned = pin.filter(|(_, i)| *i < data.roots.len());
    let target = hovered.or(pinned);

    if let Some((_, i)) = target {
        let root = &data.roots[i];
        let pos =
            crate::presentation::xy_screen_pos(response.plot_rect, places[i], ranges.0, ranges.1);
        let color = if root.is_pole() {
            c.traces[5]
        } else {
            c.traces[1]
        };
        let painter = plot_ui.painter();
        if pinned == target {
            painter.circle_stroke(pos, 8.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.5, c.accent));

        let rows = root_card_rows(root, quantity_policy, significant_digits);
        let title = if root.is_pole() { "pole" } else { "zero" };
        crate::presentation::point_card(&plot_ui, response.plot_rect, pos, title, color, &rows);
    }

    let mut table_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(table_area)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    pin_changed |= root_table(
        &mut table_ui,
        data,
        pin,
        quantity_policy,
        significant_digits,
    );
    Some(PoleZeroResponse {
        plot: response,
        fit_clicked: header.fit_clicked,
        pin_changed,
    })
}

/// Every root in a table, its selection the root pinned on the map. Returns
/// whether the reader selected another root, or none, in the table.
fn root_table(
    ui: &mut Ui,
    data: &PoleZeroData,
    pin: &mut Option<(usize, usize)>,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> bool {
    let names = root_names(data);
    let columns = root_columns();
    let revision = egui::Id::new(("pole-zero-roots", &data.name, data.roots.len(), digits)).value();
    with_table_state(ui, "pole-zero-roots", |ui, state| {
        let pinned = pin
            .map(|(_, index)| index)
            .filter(|index| *index < data.roots.len());
        if state.selected_row() != pinned {
            match pinned {
                Some(index) => state.select(index, None),
                None => state.selection = None,
            }
        }
        let response = DataTable::new("rspice.results.pole-zero.roots", &columns, data.roots.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(ui.available_height())
            .show(ui, |index| {
                root_cells(&names[index], &data.roots[index], quantity_policy, digits)
            });
        if response.selection_changed {
            *pin = state.selected_row().map(|index| (0, index));
            return true;
        }
        false
    })
}

/// The stability summary of the roots, at the reader's
/// `significant_digits`; the sheet's table lists every root.
pub fn right_panel(
    ui: &mut Ui,
    data: Option<&PoleZeroData>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let digits = significant_digits;
    section_header(ui, "Stability", None);
    let Some(data) = data else {
        crate::presentation::panel_note(ui, "Roots appear once a pole-zero analysis runs.");
        return;
    };

    let verdict = pole_stability(data);
    let summary = summarize_roots(data);
    // The verdict and what makes it are one statement: "Unstable" with the
    // RHP count beside it, rather than a verdict row and two rows of "none".
    let mut verdict_text = verdict.label().to_owned();
    if summary.right_half_plane_poles > 0 {
        verdict_text.push_str(&format!(
            " · {} RHP pole{}",
            summary.right_half_plane_poles,
            if summary.right_half_plane_poles == 1 {
                ""
            } else {
                "s"
            }
        ));
    }
    if summary.imaginary_axis_poles > 0 {
        verdict_text.push_str(&format!(
            " · {} on the imaginary axis",
            summary.imaginary_axis_poles
        ));
    }
    let mut summary_rows: Vec<(&str, String, bool)> = vec![("Verdict", verdict_text, true)];
    if let Some(root) = summary.dominant_pole {
        summary_rows.push((
            "Dominant pole",
            format_dominant_pole(root, quantity_policy, digits),
            false,
        ));
        if !root.is_real() {
            summary_rows.push((
                "Dominant ζ",
                fmt_si_significant(root.damping_ratio(), "", digits),
                false,
            ));
        }
    }
    if let Some(q) = summary.worst_q {
        summary_rows.push(("Worst Q", q_text(q, digits), false));
    }
    if let Some(gain) = data.gain {
        summary_rows.push(("Gain", fmt_si_significant(gain, "", digits), false));
    }
    crate::presentation::stat_table(ui, &summary_rows);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The words of one row of the root table.
    fn row_text(name: &str, root: &ComplexRoot) -> Vec<String> {
        let policy = QuantityPresentationPolicy::default();
        root_cells(name, root, &policy, 4)
            .into_iter()
            .map(|cell| cell.text)
            .collect()
    }

    /// A real root in the right half-plane reads differently from its
    /// left-half-plane mirror: σ keeps its sign.
    #[test]
    fn a_real_root_keeps_the_side_of_the_plane_it_sits_on() {
        let kilohertz = std::f64::consts::TAU * 1.0e3;
        assert_eq!(
            row_text("z1", &ComplexRoot::zero(kilohertz, 0.0))[1],
            "6.283 krad/s"
        );
        assert_eq!(
            row_text("z2", &ComplexRoot::zero(-kilohertz, 0.0))[1],
            "\u{2212}6.283 krad/s"
        );
    }

    /// Every root lists, each of a conjugate pair on its own row, with σ
    /// and jω in rad/s, f₀ in Hz, and ζ and Q off the real axis.
    #[test]
    fn every_root_lists_with_its_units() {
        let mut data = PoleZeroData::new("forty roots");
        for index in 1..=10 {
            let sigma = -(index as f64) * 1.0e3;
            data.roots.push(ComplexRoot::pole(sigma, 4.0e3));
            data.roots.push(ComplexRoot::pole(sigma, -4.0e3));
            data.roots.push(ComplexRoot::zero(sigma * 10.0, 0.0));
            data.roots.push(ComplexRoot::pole(sigma * 100.0, 0.0));
        }
        let names = root_names(&data);
        assert_eq!(names.len(), 40);
        assert_eq!(
            names.iter().filter(|name| name.starts_with('p')).count(),
            30
        );
        assert_eq!(names[39], "p30");

        let natural = std::f64::consts::TAU * 1.0e3;
        let pair = ComplexRoot::pole(-0.1 * natural, natural * 0.99f64.sqrt());
        assert_eq!(
            row_text("p1", &pair),
            [
                "p1",
                "\u{2212}628.3 rad/s",
                "6.252 krad/s",
                "1 kHz",
                "0.1",
                "5"
            ]
        );
        let real = row_text("p2", &ComplexRoot::pole(-natural, 0.0));
        assert_eq!(real[4..], ["", ""], "a real root has no ζ or Q");
    }

    /// The logarithmic plane keeps each root's angle, so its half-plane
    /// and its damping read as on the linear plane, and puts roots decades
    /// apart a decade's radius apart.
    #[test]
    fn the_logarithmic_plane_keeps_angles_and_spreads_decades() {
        let mut data = PoleZeroData::new("decades apart");
        data.roots.push(ComplexRoot::pole(-1.0e3, 0.0));
        data.roots.push(ComplexRoot::pole(-1.0e8, 0.0));
        data.roots.push(ComplexRoot::zero(3.0e5, 4.0e5));
        let plane = Plane::of(&data, true);
        assert_eq!(plane, Plane::Logarithmic { floor: 1.0e3 });
        let place = |root: &ComplexRoot| plane.place(root.real, root.imag);

        let (x, y) = place(&data.roots[0]);
        assert!((x + 1.0).abs() < 1e-12 && y == 0.0, "{x}, {y}");
        let (x, _) = place(&data.roots[1]);
        assert!((x + 6.0).abs() < 1e-12, "{x}");
        let (x, y) = place(&data.roots[2]);
        assert!(x > 0.0, "a right-half-plane zero stays right");
        assert!((y.atan2(x) - 4f64.atan2(3.0)).abs() < 1e-12);
        assert!((x.hypot(y) - (1.0 + 500.0f64.log10())).abs() < 1e-12);

        assert_eq!(Plane::of(&data, false).place(3.0, 4.0), (3.0, 4.0));
        let ticks = plane.axis(-7.0, 7.0, "σ").ticks;
        assert!(
            ticks.contains(&(-6.0, "\u{2212}100 M".to_owned())),
            "{ticks:?}"
        );
    }

    #[test]
    fn root_summary_reports_rhp_and_axis_content_without_inventing_reduction() {
        let mut data = PoleZeroData::new("mixed");
        data.roots.push(ComplexRoot::pole(1.0, 0.0));
        data.roots.push(ComplexRoot::pole(0.0, 2.0));

        let summary = summarize_roots(&data);
        assert_eq!(summary.right_half_plane_poles, 1);
        assert_eq!(summary.imaginary_axis_poles, 1);
        assert_eq!(summary.worst_q, Some(f64::INFINITY));
        assert!(accessible_root_inventory(&data).contains("pole 2"));
    }

    /// The hover card gives σ and jω in rad/s, so jω never reads as a
    /// frequency in Hz beside f₀, and a growing pair's Q keeps its sign.
    #[test]
    fn the_root_card_gives_s_plane_units_and_a_signed_q() {
        let policy = QuantityPresentationPolicy::default();
        let natural = std::f64::consts::TAU * 1.0e3;
        let zeta = -0.1;
        let root = ComplexRoot::pole(-zeta * natural, natural * (1.0f64 - zeta * zeta).sqrt());
        let rows = root_card_rows(&root, &policy, 4);
        let text: Vec<String> = rows.iter().map(|(k, v)| format!("{k} {v}")).collect();
        assert_eq!(
            text,
            [
                "σ 628.3 rad/s",
                "jω 6.252 krad/s",
                "f₀ 1 kHz",
                "Q \u{2212}5 · RHP"
            ]
        );
        let decaying = ComplexRoot::pole(zeta * natural, root.imag);
        let rows = root_card_rows(&decaying, &policy, 4);
        assert_eq!(rows[0].1, "\u{2212}628.3 rad/s");
        assert_eq!(rows[3].1, "5");
    }

    /// The dominant pole reads at the reader's significant digits whether it
    /// is real or complex.
    #[test]
    fn the_dominant_pole_reads_at_the_chosen_significant_digits() {
        let policy = QuantityPresentationPolicy::default();
        let real = ComplexRoot::pole(-std::f64::consts::TAU * 1_234.567, 0.0);
        assert_eq!(format_dominant_pole(&real, &policy, 4), "−1.235 kHz");
        assert_eq!(format_dominant_pole(&real, &policy, 6), "−1.23457 kHz");
        let natural = std::f64::consts::TAU * 15_915.49;
        let complex = ComplexRoot::pole(-0.05 * natural, natural * 0.9975f64.sqrt());
        assert_eq!(format_dominant_pole(&complex, &policy, 4), "15.92 kHz");
    }
}
