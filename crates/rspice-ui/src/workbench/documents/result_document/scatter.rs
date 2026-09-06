//! SCATTER — one Monte-Carlo population read as a correlation: a point per
//! trial, the requirement windows of the measured axes, a least-squares fit,
//! and a data-space brush that selects trials.
//!
//! Every point is one retained trial. Nothing is smoothed, binned or
//! resampled, and a sampled variable is only ever laid against a measurement
//! when the result says the two are indexed the same way — see
//! [`super::population`].

use std::sync::Arc;

use egui::Ui;

use crate::ui::plot::{self, Axis, InteractionMode, PlotSpec, XScale, fmt_si};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, chip, section_header, segmented, select};

use super::SheetContext;
use super::population::{
    self, ColumnKind, PopulationColumn, PopulationPlan, TrialStatus, UNPAIRED_REASON,
};
use super::strip::{self, LegendChip};
use super::well_hint;

/// What the reader is told when the run carries no sampled population.
const ABSENT_STATE: &str = "No Monte Carlo population in this run — run a Monte Carlo analysis";

/// The most brushed trials the register lists before it stops listing and
/// states the count instead.
const MAX_LISTED_TRIALS: usize = 12;

/// Session controls for the scatter sheet.
#[derive(Debug, Clone)]
pub(crate) struct ScatterSheetState {
    /// Column names, never ordinals: a re-run that retains one measurement
    /// fewer must not silently move the reader onto a different axis.
    pub(crate) x: Option<String>,
    pub(crate) y: Option<String>,
    pub(crate) color_by_status: bool,
    pub(crate) fit: bool,
    /// Retained trial rows inside the brush, in trial order.
    pub(crate) selection: Vec<usize>,
    /// The brush rectangle in data space: x0, y0, x1, y1.
    pub(crate) brush: Option<[f64; 4]>,
}

impl Default for ScatterSheetState {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            color_by_status: true,
            fit: true,
            selection: Vec::new(),
            brush: None,
        }
    }
}

/// The two columns the sheet is drawing, and whether they may be read
/// together at all.
struct Pair {
    x: usize,
    y: usize,
}

/// Column order for the X select: what was swept first, then what was
/// measured. For Y the other way round — a correlation is read as a
/// measurement against a cause.
fn axis_order(plan: &PopulationPlan, measurements_first: bool) -> Vec<usize> {
    let mut order = (0..plan.columns.len()).collect::<Vec<_>>();
    order.sort_by_key(|index| {
        let measured = plan.columns[*index].kind == ColumnKind::Measurement;
        (u8::from(measured != measurements_first), *index)
    });
    order
}

fn column_index(plan: &PopulationPlan, name: Option<&str>, fallback: usize) -> usize {
    name.and_then(|name| plan.column_index(name))
        .unwrap_or(fallback)
}

/// Which two columns are on the axes this frame.
fn active_pair(plan: &PopulationPlan, sheet: &ScatterSheetState) -> Option<Pair> {
    if plan.columns.len() < 2 {
        return None;
    }
    let x_order = axis_order(plan, false);
    let y_order = axis_order(plan, true);
    let x = column_index(plan, sheet.x.as_deref(), *x_order.first()?);
    let default_y = y_order.iter().copied().find(|index| *index != x)?;
    let y = column_index(plan, sheet.y.as_deref(), default_y);
    Some(Pair { x, y })
}

/// The trials both columns measured, as (row, x, y).
fn paired_points(plan: &PopulationPlan, pair: &Pair) -> Vec<(usize, f64, f64)> {
    let (x_column, y_column) = (&plan.columns[pair.x], &plan.columns[pair.y]);
    if !plan.columns_are_paired(x_column, y_column) {
        return Vec::new();
    }
    (0..plan.trial_count())
        .filter_map(|row| {
            let x = x_column.values.get(row).copied().flatten()?;
            let y = y_column.values.get(row).copied().flatten()?;
            Some((row, x, y))
        })
        .collect()
}

/// The interval one axis is ruled over: the measured span padded, widened to
/// take in a nearby requirement bound so the margin is visible.
fn axis_range(values: &[f64], column: &PopulationColumn) -> (f64, f64) {
    let mut low = values.iter().copied().fold(f64::INFINITY, f64::min);
    let mut high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !(low.is_finite() && high.is_finite()) {
        return (0.0, 1.0);
    }
    if (high - low).abs() <= f64::EPSILON * high.abs().max(1.0) {
        let pad = if low == 0.0 { 1.0 } else { low.abs() * 1.0e-3 };
        return (low - pad, high + pad);
    }
    let span = high - low;
    // A bound further than a quarter of the span outside the data would
    // squash the cloud to a line; the register still counts what is beyond it.
    if let Some(limit) = column.limit.as_ref() {
        for bound in [limit.min, limit.max].into_iter().flatten() {
            if bound > high && bound - high <= span * 0.25 {
                high = bound;
            }
            if bound < low && low - bound <= span * 0.25 {
                low = bound;
            }
        }
    }
    let pad = (high - low) * 0.06;
    (low - pad, high + pad)
}

// ---------------------------------------------------------------------------
// the sheet bar
// ---------------------------------------------------------------------------

/// The domain controls this sheet owns, drawn left-aligned in the sheet bar.
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let Some(plan) = population::plan(context) else {
        return false;
    };
    let Some(pair) = active_pair(&plan, &context.results.scatter) else {
        return false;
    };

    let names = plan
        .columns
        .iter()
        .map(|column| column.name.clone())
        .collect::<Vec<_>>();
    let x_options = axis_order(&plan, false)
        .into_iter()
        .map(|index| names[index].clone())
        .collect::<Vec<_>>();
    let y_options = axis_order(&plan, true)
        .into_iter()
        .map(|index| names[index].clone())
        .collect::<Vec<_>>();

    if let Some(picked) = select(
        ui,
        "rspice.results.scatter.x",
        "Scatter X column",
        &names[pair.x],
        &x_options,
        150.0,
    ) {
        context.results.scatter.x = x_options.get(picked).cloned();
        clear_selection(context);
    }
    if let Some(picked) = select(
        ui,
        "rspice.results.scatter.y",
        "Scatter Y column",
        &names[pair.y],
        &y_options,
        150.0,
    ) {
        context.results.scatter.y = y_options.get(picked).cloned();
        clear_selection(context);
    }

    let mut colour = usize::from(!context.results.scatter.color_by_status);
    if segmented(
        ui,
        "rspice.results.scatter.colour",
        &["pass / fail", "none"],
        &mut colour,
        SegmentedWidth::Natural,
    ) {
        context.results.scatter.color_by_status = colour == 0;
    }

    let fit = context.results.scatter.fit;
    let response = chip(ui, "FIT", fit).on_hover_text(if fit {
        "Least-squares line and its correlation"
    } else {
        "No fit drawn"
    });
    if response.clicked() {
        context.results.scatter.fit = !fit;
    }

    let limits = context.results.show_spec_limits;
    let response = chip(ui, "LIM", limits).on_hover_text(if limits {
        "Requirement windows shaded on the violating side"
    } else {
        "Requirement windows hidden"
    });
    if response.clicked() {
        context.results.show_spec_limits = !limits;
    }

    let response = chip(ui, "\u{21c4}", false).on_hover_text("Swap the two axes");
    if response.clicked() {
        let scatter = &mut context.results.scatter;
        std::mem::swap(&mut scatter.x, &mut scatter.y);
        if scatter.x.is_none() {
            scatter.x = Some(names[pair.y].clone());
        }
        if scatter.y.is_none() {
            scatter.y = Some(names[pair.x].clone());
        }
        clear_selection(context);
    }

    let t = Tokens::get(ui.ctx());
    let variables = plan
        .columns
        .iter()
        .filter(|column| column.kind == ColumnKind::SampledVariable)
        .count();
    ui.add_space(8.0);
    // Truncated, not wrapped: the bar is one row, and a readout that
    // outgrows the space left by the controls draws over the export menu.
    ui.add(
        egui::Label::new(
            egui::RichText::new(format!(
                "{} trials \u{b7} {variables} sampled \u{b7} {} measured",
                plan.trial_count(),
                plan.columns.len() - variables
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
        )
        .truncate(),
    );
    true
}

fn clear_selection(context: &mut SheetContext<'_>) {
    context.results.scatter.selection.clear();
    context.results.scatter.brush = None;
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the scatter sheet.
pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let Some(plan) = population::plan(context) else {
        well_hint(ui, ABSENT_STATE);
        return;
    };
    let Some(pair) = active_pair(&plan, &context.results.scatter) else {
        well_hint(
            ui,
            "This Monte Carlo retained one column — a correlation needs two",
        );
        return;
    };
    let (x_column, y_column) = (&plan.columns[pair.x], &plan.columns[pair.y]);
    if !plan.columns_are_paired(x_column, y_column) {
        well_hint(ui, UNPAIRED_REASON);
        return;
    }
    let points = paired_points(&plan, &pair);
    if points.len() < 2 {
        well_hint(ui, "Fewer than two trials measured both of these columns");
        return;
    }

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let sheet = context.results.scatter.clone();
    let show_limits = context.results.show_spec_limits;

    let failing = points
        .iter()
        .filter(|(row, _, _)| plan.status.get(*row) == Some(&TrialStatus::Failing))
        .count();
    let subtitle = if sheet.selection.is_empty() {
        format!(
            "{} vs {} \u{b7} {} trials \u{b7} {failing} failing",
            y_column.name,
            x_column.name,
            points.len()
        )
    } else {
        format!(
            "{} vs {} \u{b7} {} trials selected \u{b7} Esc clears",
            y_column.name,
            x_column.name,
            sheet.selection.len()
        )
    };
    let mut legend = vec![LegendChip {
        name: "trial",
        color: c.traces[1],
        on: true,
    }];
    if sheet.color_by_status && failing > 0 {
        legend.push(LegendChip {
            name: "failing trial",
            color: c.err,
            on: true,
        });
    }
    if sheet.fit {
        legend.push(LegendChip {
            name: "least-squares fit",
            color: c.traces[0],
            on: true,
        });
    }
    if show_limits && (x_column.limit.is_some() || y_column.limit.is_some()) {
        legend.push(LegendChip {
            name: "requirement window",
            color: c.err,
            on: true,
        });
    }
    strip::StripHeader::new("MC", &subtitle, &legend).show(ui);

    let xs = points.iter().map(|(_, x, _)| *x).collect::<Vec<_>>();
    let ys = points.iter().map(|(_, _, y)| *y).collect::<Vec<_>>();
    let (x0, x1) = axis_range(&xs, x_column);
    let (y0, y1) = axis_range(&ys, y_column);

    let accessible = format!(
        "{} against {}; {} trials, {failing} failing. \
         Drag to brush a rectangle of trials, click to select one, Escape clears.",
        y_column.name,
        x_column.name,
        points.len()
    );
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, x_column.unit.clone()).with_label(x_column.name.clone()),
        XScale::Linear,
        Axis::linear(y0, y1, y_column.unit.clone()).with_label(y_column.name.clone()),
    )
    .accessible_name("Monte Carlo correlation")
    .accessible_detail(&accessible);
    spec.left_margin = 68.0;

    let fit = sheet
        .fit
        .then(|| population::least_squares(&xs, &ys))
        .flatten();
    let selected = sheet.selection.clone();
    let painted = PaintedCloud {
        points: points.clone(),
        status: plan.status.clone(),
        selected,
        colour_by_status: sheet.color_by_status,
        fit,
        x_limit: show_limits.then(|| x_column.limit.clone()).flatten(),
        y_limit: show_limits.then(|| y_column.limit.clone()).flatten(),
        brush: sheet.brush,
        colors: CloudColors {
            point: c.traces[1],
            failing: c.err,
            selected: c.accent,
            fit: c.traces[0],
            wash: c.err.gamma_multiply(0.09),
            limit: c.err,
        },
    };
    spec.underlay = Some(Box::new(move |painter, mapper| {
        painted.draw(painter, mapper)
    }));

    // Select mode: the primary drag is the brush, so the plot must not also
    // pan under it.
    plot::set_interaction_mode(ui.ctx(), InteractionMode::Select);
    let response = plot::show(ui, &spec, &mut context.results.cache, None, None);
    super::record_drawn_axes(context.results, super::ResultViewer::Scatter, &response);

    apply_input(ui, context, &plan, &points, &response, ((x0, x1), (y0, y1)));
    paint_hover_card(ui, &plan, &points, &response, ((x0, x1), (y0, y1)));
}

#[derive(Clone, Copy)]
struct CloudColors {
    point: egui::Color32,
    failing: egui::Color32,
    selected: egui::Color32,
    fit: egui::Color32,
    wash: egui::Color32,
    limit: egui::Color32,
}

/// Everything the underlay draws, owned so the closure outlives the borrow.
struct PaintedCloud {
    points: Vec<(usize, f64, f64)>,
    status: Vec<TrialStatus>,
    selected: Vec<usize>,
    colour_by_status: bool,
    fit: Option<(f64, f64)>,
    x_limit: Option<population::PopulationLimit>,
    y_limit: Option<population::PopulationLimit>,
    brush: Option<[f64; 4]>,
    colors: CloudColors,
}

impl PaintedCloud {
    fn draw(&self, painter: &egui::Painter, mapper: &plot::PlotMapper) {
        // Requirement windows first: the violating side of each bound, washed.
        if let Some(limit) = self.x_limit.as_ref() {
            if let Some(min) = limit.min {
                let right = mapper.x(min);
                painter.rect_filled(
                    egui::Rect::from_min_max(
                        mapper.rect.left_top(),
                        egui::pos2(right.min(mapper.rect.right()), mapper.rect.bottom()),
                    ),
                    0.0,
                    self.colors.wash,
                );
                self.vline(painter, mapper, right);
            }
            if let Some(max) = limit.max {
                let left = mapper.x(max);
                painter.rect_filled(
                    egui::Rect::from_min_max(
                        egui::pos2(left.max(mapper.rect.left()), mapper.rect.top()),
                        mapper.rect.right_bottom(),
                    ),
                    0.0,
                    self.colors.wash,
                );
                self.vline(painter, mapper, left);
            }
        }
        if let Some(limit) = self.y_limit.as_ref() {
            if let Some(min) = limit.min {
                let bottom = mapper.y(min);
                painter.rect_filled(
                    egui::Rect::from_min_max(
                        egui::pos2(mapper.rect.left(), bottom.max(mapper.rect.top())),
                        mapper.rect.right_bottom(),
                    ),
                    0.0,
                    self.colors.wash,
                );
                self.hline(painter, mapper, bottom);
            }
            if let Some(max) = limit.max {
                let top = mapper.y(max);
                painter.rect_filled(
                    egui::Rect::from_min_max(
                        mapper.rect.left_top(),
                        egui::pos2(mapper.rect.right(), top.min(mapper.rect.bottom())),
                    ),
                    0.0,
                    self.colors.wash,
                );
                self.hline(painter, mapper, top);
            }
        }

        // The fit, under the cloud.
        if let Some((slope, intercept)) = self.fit {
            let left = mapper.rect.left();
            let right = mapper.rect.right();
            let at = |screen_x: f32| {
                let fraction = f64::from(screen_x - left) / f64::from(right - left).max(1.0);
                let data_x = self.x_at(mapper, fraction);
                mapper.y(slope * data_x + intercept)
            };
            painter.add(egui::Shape::dashed_line(
                &[egui::pos2(left, at(left)), egui::pos2(right, at(right))],
                egui::Stroke::new(1.4, self.colors.fit),
                6.0,
                4.0,
            ));
        }

        for (row, x, y) in &self.points {
            let point = egui::pos2(mapper.x(*x), mapper.y(*y));
            if !mapper.rect.contains(point) {
                continue;
            }
            let failing =
                self.colour_by_status && self.status.get(*row) == Some(&TrialStatus::Failing);
            let selected = self.selected.binary_search(row).is_ok();
            if failing {
                // A failing trial is an error cross, not a differently
                // coloured dot: colour alone is not a distinction.
                let arm = 3.0;
                let stroke = egui::Stroke::new(1.3, self.colors.failing);
                painter.line_segment(
                    [point + egui::vec2(-arm, -arm), point + egui::vec2(arm, arm)],
                    stroke,
                );
                painter.line_segment(
                    [point + egui::vec2(-arm, arm), point + egui::vec2(arm, -arm)],
                    stroke,
                );
            } else {
                painter.circle_filled(point, 1.8, self.colors.point);
            }
            if selected {
                painter.circle_stroke(point, 4.0, egui::Stroke::new(1.2, self.colors.selected));
            }
        }

        if let Some([bx0, by0, bx1, by1]) = self.brush {
            let rect = egui::Rect::from_two_pos(
                egui::pos2(mapper.x(bx0), mapper.y(by0)),
                egui::pos2(mapper.x(bx1), mapper.y(by1)),
            );
            painter.rect(
                rect.intersect(mapper.rect),
                0.0,
                self.colors.selected.gamma_multiply(0.08),
                egui::Stroke::new(1.0, self.colors.selected),
                egui::StrokeKind::Inside,
            );
        }
    }

    /// Data-space X at a fraction of the plot width. The mapper only maps
    /// forward, and the fit line needs the inverse at the two edges.
    fn x_at(&self, mapper: &plot::PlotMapper, fraction: f64) -> f64 {
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
        for (_, x, _) in &self.points {
            low = low.min(*x);
            high = high.max(*x);
        }
        // Recover the axis interval from two known screen positions.
        let (left, right) = (mapper.x(low), mapper.x(high));
        if (right - left).abs() < f32::EPSILON {
            return low;
        }
        let scale = (high - low) / f64::from(right - left);
        let axis_low = low - f64::from(left - mapper.rect.left()) * scale;
        let axis_high = high + f64::from(mapper.rect.right() - right) * scale;
        axis_low + fraction * (axis_high - axis_low)
    }

    fn vline(&self, painter: &egui::Painter, mapper: &plot::PlotMapper, x: f32) {
        if mapper.rect.x_range().contains(x) {
            painter.vline(
                x,
                mapper.rect.y_range(),
                egui::Stroke::new(1.0, self.colors.limit),
            );
        }
    }

    fn hline(&self, painter: &egui::Painter, mapper: &plot::PlotMapper, y: f32) {
        if mapper.rect.y_range().contains(y) {
            painter.hline(
                mapper.rect.x_range(),
                y,
                egui::Stroke::new(1.0, self.colors.limit),
            );
        }
    }
}

/// Screen position of one data point inside the drawn plot rectangle.
fn point_position(
    response: &plot::PlotResponse,
    ranges: ((f64, f64), (f64, f64)),
    x: f64,
    y: f64,
) -> egui::Pos2 {
    super::xy_screen_pos(response.plot_rect, (x, y), ranges.0, ranges.1)
}

/// Data-space coordinates of a screen position.
fn data_position(
    response: &plot::PlotResponse,
    ranges: ((f64, f64), (f64, f64)),
    at: egui::Pos2,
) -> (f64, f64) {
    let rect = response.plot_rect;
    let fx = f64::from(at.x - rect.left()) / f64::from(rect.width().max(1.0));
    let fy = f64::from(rect.bottom() - at.y) / f64::from(rect.height().max(1.0));
    (
        ranges.0.0 + fx * (ranges.0.1 - ranges.0.0),
        ranges.1.0 + fy * (ranges.1.1 - ranges.1.0),
    )
}

fn nearest_point(
    response: &plot::PlotResponse,
    points: &[(usize, f64, f64)],
    ranges: ((f64, f64), (f64, f64)),
    at: egui::Pos2,
) -> Option<usize> {
    let mut best = 12.0_f32 * 12.0;
    let mut hit = None;
    for (row, x, y) in points {
        let distance = point_position(response, ranges, *x, *y).distance_sq(at);
        if distance < best {
            best = distance;
            hit = Some(*row);
        }
    }
    hit
}

/// Brush, click and keyboard, applied to the sheet's selection.
fn apply_input(
    ui: &Ui,
    context: &mut SheetContext<'_>,
    plan: &Arc<PopulationPlan>,
    points: &[(usize, f64, f64)],
    response: &plot::PlotResponse,
    ranges: ((f64, f64), (f64, f64)),
) {
    let anchor_id = response.response.id.with("scatter.brush");
    if response
        .response
        .drag_started_by(egui::PointerButton::Primary)
        && let Some(at) = response.response.interact_pointer_pos()
    {
        let anchor = data_position(response, ranges, response.plot_rect.clamp(at));
        ui.memory_mut(|memory| memory.data.insert_temp(anchor_id, anchor));
    }
    if let Some((ax, ay)) = ui.memory(|memory| memory.data.get_temp::<(f64, f64)>(anchor_id)) {
        let corner = response
            .response
            .interact_pointer_pos()
            .map(|at| data_position(response, ranges, response.plot_rect.clamp(at)));
        if let Some((bx, by)) = corner {
            context.results.scatter.brush = Some([ax, ay, bx, by]);
            if response
                .response
                .drag_stopped_by(egui::PointerButton::Primary)
            {
                ui.memory_mut(|memory| memory.data.remove::<(f64, f64)>(anchor_id));
                let selection = points
                    .iter()
                    .filter(|(_, x, y)| {
                        (ax.min(bx)..=ax.max(bx)).contains(x)
                            && (ay.min(by)..=ay.max(by)).contains(y)
                    })
                    .map(|(row, _, _)| *row)
                    .collect::<Vec<_>>();
                if selection.is_empty() {
                    context.results.scatter.brush = None;
                }
                context.results.scatter.selection = selection;
            }
        }
        return;
    }

    if response.response.clicked()
        && let Some(at) = response.response.interact_pointer_pos()
    {
        context.results.scatter.brush = None;
        context.results.scatter.selection = nearest_point(response, points, ranges, at)
            .map(|row| vec![row])
            .unwrap_or_default();
    }
    if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        clear_selection(context);
    }
    // A trial is reproduced from its seed, and the seed is what the register
    // states; there is no per-trial waveform set to open.
    let _ = plan;
}

fn paint_hover_card(
    ui: &Ui,
    plan: &Arc<PopulationPlan>,
    points: &[(usize, f64, f64)],
    response: &plot::PlotResponse,
    ranges: ((f64, f64), (f64, f64)),
) {
    let Some(at) = response.response.hover_pos() else {
        return;
    };
    if !response.plot_rect.contains(at) {
        return;
    }
    let Some(row) = nearest_point(response, points, ranges, at) else {
        return;
    };
    let Some((_, x, y)) = points.iter().copied().find(|(index, _, _)| *index == row) else {
        return;
    };
    let position = point_position(response, ranges, x, y);
    let status = match plan.status.get(row) {
        Some(TrialStatus::Failing) => "fails a requirement",
        Some(TrialStatus::Unmeasured) => "not measured",
        _ => "pass",
    };
    let title = plan
        .trials
        .get(row)
        .map_or_else(|| format!("Trial {row}"), |trial| trial.label.clone());
    let color = Tokens::get(ui.ctx()).color.traces[1];
    super::point_card(
        ui,
        response.plot_rect,
        position,
        &title,
        color,
        &[
            ("x".to_owned(), fmt_si(x, "", 4)),
            ("y".to_owned(), fmt_si(y, "", 4)),
            ("status".to_owned(), status.to_owned()),
        ],
    );
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Correlation register and the brushed trial list.
pub fn right_panel(ui: &mut Ui, context: &mut SheetContext<'_>) {
    section_header(ui, "Correlation", None);
    let Some(plan) = population::plan(context) else {
        super::panel_note(
            ui,
            "Correlation appears once a Monte Carlo population is loaded.",
        );
        return;
    };
    let Some(pair) = active_pair(&plan, &context.results.scatter) else {
        super::panel_note(ui, "A correlation needs two retained columns.");
        return;
    };
    let (x_column, y_column) = (&plan.columns[pair.x], &plan.columns[pair.y]);
    if !plan.columns_are_paired(x_column, y_column) {
        super::panel_note(ui, UNPAIRED_REASON);
        return;
    }
    let points = paired_points(&plan, &pair);
    let rows = register_rows(&plan, &points, x_column, y_column);
    super::stat_table(ui, &rows);

    let selection = context.results.scatter.selection.clone();
    if selection.is_empty() {
        super::panel_note(
            ui,
            "Drag on the plot to brush a rectangle of trials; click selects one and Esc clears.",
        );
        return;
    }
    section_header(ui, "Brushed trials", None);
    let listed = selection.len().min(MAX_LISTED_TRIALS);
    let mut rows = Vec::with_capacity(listed);
    for row in selection.iter().take(listed) {
        let Some((_, x, y)) = points.iter().copied().find(|(index, _, _)| index == row) else {
            continue;
        };
        let label = plan
            .trials
            .get(*row)
            .map_or_else(|| format!("Trial {row}"), |trial| trial.label.clone());
        let status = match plan.status.get(*row) {
            Some(TrialStatus::Failing) => "fail",
            Some(TrialStatus::Unmeasured) => "no result",
            _ => "pass",
        };
        rows.push((
            label,
            format!(
                "{} \u{b7} {} \u{b7} {status}",
                fmt_si(x, &x_column.unit, 3),
                fmt_si(y, &y_column.unit, 3)
            ),
        ));
    }
    let rows = rows
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    crate::ui::widgets::measurement_table(ui, &rows);
    if selection.len() > listed {
        super::panel_note(
            ui,
            &format!(
                "{} more brushed trials are not listed.",
                selection.len() - listed
            ),
        );
    }
    if ui.button("Clear brush").clicked() {
        clear_selection(context);
    }
    super::panel_note(
        ui,
        "A trial is reproduced from the seed in its name. No per-trial waveform set is \
         retained, so there is nothing further to open here.",
    );
}

fn register_rows(
    plan: &PopulationPlan,
    points: &[(usize, f64, f64)],
    x_column: &PopulationColumn,
    y_column: &PopulationColumn,
) -> Vec<(&'static str, String, bool)> {
    let xs = points.iter().map(|(_, x, _)| *x).collect::<Vec<_>>();
    let ys = points.iter().map(|(_, _, y)| *y).collect::<Vec<_>>();
    let failing = points
        .iter()
        .filter(|(row, _, _)| plan.status.get(*row) == Some(&TrialStatus::Failing))
        .count();
    let correlation = population::pearson(&xs, &ys);
    let fit = population::least_squares(&xs, &ys);
    let unmeasured = plan.trial_count() - points.len();

    let mut rows = vec![
        (
            "Trials",
            if unmeasured == 0 {
                format!("{} measured \u{b7} {failing} failing", points.len())
            } else {
                format!(
                    "{} measured \u{b7} {failing} failing \u{b7} {unmeasured} without a value",
                    points.len()
                )
            },
            true,
        ),
        (
            "Pearson r",
            correlation.map_or_else(
                || "Not defined \u{2014} a column has no spread".to_owned(),
                |r| format!("{r:+.3} \u{b7} R\u{b2} {:.3}", r * r),
            ),
            true,
        ),
        (
            "Fit",
            fit.map_or_else(
                || "Not defined".to_owned(),
                |(slope, intercept)| {
                    // The whole equation, named: a bare slope and intercept
                    // beside one another cannot be told apart, and the reader
                    // has to be able to hold this against a datasheet.
                    format!(
                        "{} = {} \u{b7} {} {} {}",
                        y_column.name,
                        fmt_si(slope, &slope_unit(x_column, y_column), 4),
                        x_column.name,
                        if intercept < 0.0 { "-" } else { "+" },
                        fmt_si(intercept.abs(), &y_column.unit, 4)
                    )
                },
            ),
            false,
        ),
        (
            "Mean X",
            population::mean(&xs).map_or_else(
                || "\u{2014}".to_owned(),
                |mean| fmt_si(mean, &x_column.unit, 4),
            ),
            false,
        ),
        (
            "Mean Y",
            population::mean(&ys).map_or_else(
                || "\u{2014}".to_owned(),
                |mean| fmt_si(mean, &y_column.unit, 4),
            ),
            false,
        ),
    ];
    rows.push(("Y requirement", requirement_row(y_column, &ys), false));
    rows.push(("X requirement", requirement_row(x_column, &xs), false));
    rows
}

/// The unit of a slope, as the two columns state theirs.
fn slope_unit(x_column: &PopulationColumn, y_column: &PopulationColumn) -> String {
    match (y_column.unit.as_str(), x_column.unit.as_str()) {
        ("", "") => String::new(),
        (y, "") => y.to_owned(),
        ("", x) => format!("1/{x}"),
        (y, x) => format!("{y}/{x}"),
    }
}

/// What the register says about one axis' requirement, or about the sampled
/// variable that has none.
fn requirement_row(column: &PopulationColumn, values: &[f64]) -> String {
    match column.limit.as_ref() {
        Some(limit) => {
            let beyond = values.iter().filter(|value| !limit.passes(**value)).count();
            format!("{} \u{b7} {beyond} beyond", limit.text)
        }
        None if column.kind == ColumnKind::SampledVariable => population::std_dev(values)
            .map_or_else(
                || "sampled variable".to_owned(),
                |sigma| {
                    format!(
                        "sampled variable \u{b7} std dev {}",
                        fmt_si(sigma, &column.unit, 3)
                    )
                },
            ),
        None => "No requirement bounds this measurement".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::ResultsState;
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, FamilyMeasurementEvidence,
        FamilyMemberId, FamilyMemberMeasurements, MonteCarloVariableMetadata, SimulationRun,
        SpecEntry, SpecPointScope,
    };

    /// A thousand-trial population whose measurement is a rank-matched
    /// function of the sampled variable, with a named failing tail.
    fn population_analysis(trials: usize) -> AnalysisResult {
        let samples: Vec<f64> = (0..trials)
            .map(|index| (index as f64 - trials as f64 / 2.0) / (trials as f64 / 2.0))
            .collect();
        let members = samples
            .iter()
            .enumerate()
            .map(|(index, sample)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 0x73a4 + index as u64,
                    },
                    vec![FamilyMeasurementEvidence {
                        name: "gain_dc".to_owned(),
                        value: Some(40.0 + 2.0 * sample),
                        passed: true,
                        error: None,
                    }],
                )
            })
            .collect::<Vec<_>>();
        let mean = samples.iter().sum::<f64>() / trials as f64;
        let variance =
            samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (trials - 1) as f64;
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x73a4,
                runs_requested: trials,
                runs_completed: trials,
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    name: "XBRIDGE.dR".to_owned(),
                    mean,
                    std_dev: variance.sqrt(),
                    min: samples.iter().copied().fold(f64::INFINITY, f64::min),
                    max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                    samples,
                }],
                member_measurements: members,
            },
        )
    }

    /// The three slices this sheet is handed: one retained run, the authored
    /// requirements, and an empty presentation state.
    fn fixture(
        trials: usize,
        limit: Option<f64>,
    ) -> (
        crate::state::SimulationState,
        crate::state::ProjectWorkspace,
        ResultsState,
    ) {
        let mut run = SimulationRun::new(1);
        run.add_analysis(population_analysis(trials));
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs = vec![run];
        assert!(simulation.select_run(0));
        assert!(simulation.select_analysis(0));
        let mut workspace = crate::state::ProjectWorkspace::default();
        if let Some(min) = limit {
            workspace.specs.push(SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: String::new(),
                min: Some(min),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            });
        }
        (simulation, workspace, ResultsState::default())
    }

    fn context<'a>(
        simulation: &'a crate::state::SimulationState,
        workspace: &'a crate::state::ProjectWorkspace,
        results: &'a mut ResultsState,
    ) -> SheetContext<'a> {
        SheetContext {
            simulation,
            workspace,
            results,
            policy: crate::quantity::QuantityPresentationPolicy::default(),
        }
    }

    /// The X select offers what was swept before what was measured, and Y the
    /// other way round: a correlation is read as a measurement against a
    /// cause, and the defaults must land there without the reader choosing.
    #[test]
    fn the_default_axes_are_the_measurement_against_its_sampled_cause() {
        let (simulation, workspace, mut results) = fixture(64, None);
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let pair = active_pair(&plan, &ctx.results.scatter).expect("two columns");

        assert_eq!(plan.columns[pair.x].name, "XBRIDGE.dR");
        assert_eq!(plan.columns[pair.y].name, "gain_dc");
        assert_eq!(plan.columns[pair.x].kind, ColumnKind::SampledVariable);
        assert_eq!(plan.columns[pair.y].kind, ColumnKind::Measurement);
    }

    /// The register states the correlation, the fit, and how many trials are
    /// beyond the requirement — the three numbers the sheet exists for.
    #[test]
    fn the_register_states_the_correlation_the_fit_and_the_trials_beyond_the_limit() {
        let (simulation, workspace, mut results) = fixture(101, Some(39.5));
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let pair = active_pair(&plan, &ctx.results.scatter).expect("two columns");
        let points = paired_points(&plan, &pair);
        assert_eq!(points.len(), 101);

        let rows = register_rows(&plan, &points, &plan.columns[pair.x], &plan.columns[pair.y]);
        let by_name = |needle: &str| {
            rows.iter()
                .find(|row| row.0 == needle)
                .map(|row| row.1.clone())
                .unwrap_or_else(|| panic!("the register has no {needle} row"))
        };

        assert_eq!(by_name("Trials"), "101 measured \u{b7} 38 failing");
        assert!(
            by_name("Pearson r").starts_with("+1.000 \u{b7} R\u{b2} 1.000"),
            "{}",
            by_name("Pearson r")
        );
        // gain_dc = 40 + 2·dR, so the slope is 2 dB per unit and the
        // intercept is 40 dB.
        assert_eq!(
            by_name("Fit"),
            "gain_dc = 2.0000 dB \u{b7} XBRIDGE.dR + 40.0000 dB",
            "the fit states the equation the reader holds against a datasheet"
        );
        assert_eq!(
            by_name("Y requirement"),
            "\u{2265} 39.500 dB \u{b7} 38 beyond"
        );
        assert!(
            by_name("X requirement").starts_with("sampled variable \u{b7} std dev"),
            "{}",
            by_name("X requirement")
        );
    }

    /// The absent state names the analysis that would fill the sheet.
    #[test]
    fn the_absent_state_names_the_analysis_that_would_fill_it() {
        assert_eq!(
            ABSENT_STATE,
            "No Monte Carlo population in this run — run a Monte Carlo analysis"
        );
        let simulation = crate::state::SimulationState::default();
        let workspace = crate::state::ProjectWorkspace::default();
        let mut results = ResultsState::default();
        let mut ctx = context(&simulation, &workspace, &mut results);
        assert!(population::plan(&mut ctx).is_none());
    }

    /// A column with no spread has no correlation to report, and the register
    /// says so rather than printing a zero that reads as "uncorrelated".
    #[test]
    fn a_column_without_spread_reports_no_correlation_rather_than_zero() {
        let (simulation, workspace, mut results) = fixture(32, None);
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let flat = PopulationColumn {
            name: "flat".to_owned(),
            kind: ColumnKind::Measurement,
            unit: String::new(),
            values: vec![Some(1.0); plan.trial_count()],
            limit: None,
        };
        let points = (0..plan.trial_count())
            .map(|row| (row, 1.0, row as f64))
            .collect::<Vec<_>>();
        let rows = register_rows(&plan, &points, &flat, &plan.columns[1]);
        assert_eq!(
            rows.iter()
                .find(|row| row.0 == "Pearson r")
                .map(|r| r.1.as_str()),
            Some("Not defined \u{2014} a column has no spread")
        );
    }

    /// The axis takes in a requirement bound that sits just outside the data,
    /// so the margin is visible — and refuses one far enough away to squash
    /// the cloud into a line.
    #[test]
    fn the_axis_widens_for_a_nearby_bound_and_refuses_a_distant_one() {
        let values: Vec<f64> = (0..101).map(|index| f64::from(index) / 100.0).collect();
        let near = PopulationColumn {
            name: "m".to_owned(),
            kind: ColumnKind::Measurement,
            unit: String::new(),
            values: Vec::new(),
            limit: Some(population::PopulationLimit {
                min: None,
                max: Some(1.2),
                text: String::new(),
            }),
        };
        let (_, high) = axis_range(&values, &near);
        assert!(high > 1.2, "the bound at 1.2 is not on the axis: {high}");

        let far = PopulationColumn {
            limit: Some(population::PopulationLimit {
                min: None,
                max: Some(50.0),
                text: String::new(),
            }),
            ..near.clone()
        };
        let (_, high) = axis_range(&values, &far);
        assert!(
            high < 2.0,
            "a bound fifty times the span squashed the cloud: {high}"
        );
    }

    /// The slope unit is the two columns' units divided, and stays empty when
    /// neither column stated one — never an invented "1".
    #[test]
    fn the_slope_unit_is_the_two_columns_units_divided() {
        let column = |unit: &str| PopulationColumn {
            name: "c".to_owned(),
            kind: ColumnKind::Measurement,
            unit: unit.to_owned(),
            values: Vec::new(),
            limit: None,
        };
        assert_eq!(slope_unit(&column(""), &column("")), "");
        assert_eq!(slope_unit(&column("%"), &column("V")), "V/%");
        assert_eq!(slope_unit(&column(""), &column("V")), "V");
        assert_eq!(slope_unit(&column("%"), &column("")), "1/%");
    }
}
