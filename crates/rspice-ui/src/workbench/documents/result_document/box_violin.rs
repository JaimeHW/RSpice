//! BOX / VIOLIN — the shape of a retained population against the requirement
//! that bounds it: a kernel-density body, the quartile box, Tukey or extreme
//! whiskers, the mean, the outliers, and the failing trials as crosses.
//!
//! Two groupings, and the difference between them is what the ordinate means.
//! One measurement is drawn in its own engineering unit. All measurements at
//! once cannot be — they have unrelated units and one- or two-sided bounds —
//! so each is normalized to its own requirement and the limit becomes a
//! single line at zero for every column. That normalization is the sheet's
//! one derived quantity, and both spellings of it are stated in the head.

use egui::Ui;

use crate::ui::plot::{self, Axis, InteractionMode, PlotSpec, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{
    SegmentedWidth, chip, section_header, segmented, select, select_with_disabled,
};

use super::SheetContext;
use super::population::{
    self, BoxStatistics, ColumnKind, PopulationColumn, PopulationLimit, PopulationPlan,
    TrialStatus, Whiskers,
};
use super::strip::{self, LegendChip};
use super::well_hint;

/// What the reader is told when the run carries no sampled population.
const ABSENT_STATE: &str = "No family distribution in this run — run a Monte Carlo analysis";

/// Why the corner grouping cannot be chosen.
const CORNER_REASON: &str = "A box per corner needs a sampled population at each corner. A corner sweep retains one \
     value per corner, and no runner produces a Monte Carlo per corner.";

/// What the columns are grouped by.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum BoxGrouping {
    /// Every bounded measurement, normalized to its own requirement.
    #[default]
    AllMeasurements,
    /// One column in its own engineering unit.
    OneMeasurement,
}

/// How a margin is expressed when every measurement shares one ordinate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum MarginScale {
    /// Signed distance to the bound, in sample standard deviations.
    /// Three of them is a capability of one.
    #[default]
    Sigma,
    /// The Specs sheet's margin, as a percentage of the requirement.
    Percent,
}

/// Session controls for the box/violin sheet.
#[derive(Debug, Clone)]
pub(crate) struct BoxViolinSheetState {
    pub(crate) grouping: BoxGrouping,
    pub(crate) scale: MarginScale,
    /// Column name, never an ordinal.
    pub(crate) measurement: Option<String>,
    pub(crate) violin: bool,
    pub(crate) whiskers: Whiskers,
    pub(crate) outliers: bool,
}

impl Default for BoxViolinSheetState {
    fn default() -> Self {
        Self {
            grouping: BoxGrouping::default(),
            scale: MarginScale::default(),
            measurement: None,
            violin: true,
            whiskers: Whiskers::default(),
            outliers: true,
        }
    }
}

/// One drawn column: the values on the ordinate the grouping chose, and what
/// the head under it says.
struct Column {
    name: String,
    /// Ordinate values in trial order, `None` where the trial had no value.
    values: Vec<Option<f64>>,
    sorted: Vec<f64>,
    statistics: BoxStatistics,
    /// The requirement as it lands on the ordinate this column is drawn on.
    /// A normalized column carries the bound at zero and the original
    /// requirement's own spelling as its text.
    limit: Option<PopulationLimit>,
    measured: usize,
    passing: usize,
    unit: String,
    /// Whether the ordinate is a margin rather than the measured quantity.
    normalized: bool,
}

impl Column {
    /// One exact value on this column's ordinate.
    ///
    /// A normalized ordinate is a count of standard deviations or a
    /// percentage, and an engineering prefix on either spells nonsense:
    /// `fmt_si` turned a median of 0.4266 sigma into "426.6187 msigma".
    fn format(&self, value: f64) -> String {
        if self.normalized {
            format!("{value:.3} {}", self.unit)
        } else {
            fmt_si(value, &self.unit, 4)
        }
    }

    fn yield_percent(&self) -> Option<f64> {
        (self.measured > 0).then(|| 100.0 * self.passing as f64 / self.measured as f64)
    }

    fn limit_text(&self) -> String {
        self.limit
            .as_ref()
            .map_or_else(|| "No requirement".to_owned(), |limit| limit.text.clone())
    }

    /// Each bound and the direction that breaks it.
    fn bounds(&self) -> Vec<(f64, bool)> {
        self.limit.as_ref().map_or_else(Vec::new, |limit| {
            limit
                .min
                .map(|min| (min, true))
                .into_iter()
                .chain(limit.max.map(|max| (max, false)))
                .collect()
        })
    }
}

/// Which columns the current grouping draws.
fn drawn_columns(
    plan: &PopulationPlan,
    sheet: &BoxViolinSheetState,
) -> Result<Vec<Column>, &'static str> {
    match sheet.grouping {
        BoxGrouping::AllMeasurements => {
            let columns = plan
                .columns
                .iter()
                .filter(|column| column.kind == ColumnKind::Measurement && column.limit.is_some())
                .filter_map(|column| normalized_column(column, sheet.scale, sheet.whiskers))
                .collect::<Vec<_>>();
            if columns.is_empty() {
                return Err(
                    "No measurement in this run carries a requirement — a normalized \
                     comparison needs a bound to normalize to",
                );
            }
            Ok(columns)
        }
        BoxGrouping::OneMeasurement => {
            let index = selected_column(plan, sheet)
                .ok_or("This run retains no column with a distribution to draw")?;
            let column = &plan.columns[index];
            Ok(vec![
                engineering_column(column, sheet.whiskers)
                    .ok_or("Fewer than two trials measured this column")?,
            ])
        }
    }
}

/// Which single column the reader selected, or the first that has values.
fn selected_column(plan: &PopulationPlan, sheet: &BoxViolinSheetState) -> Option<usize> {
    let usable = |index: &usize| {
        plan.columns[*index]
            .values
            .iter()
            .filter(|value| value.is_some())
            .count()
            >= 2
    };
    sheet
        .measurement
        .as_deref()
        .and_then(|name| plan.column_index(name))
        .filter(usable)
        .or_else(|| (0..plan.columns.len()).find(usable))
}

/// One measurement on its own engineering ordinate.
fn engineering_column(column: &PopulationColumn, whiskers: Whiskers) -> Option<Column> {
    let values = column.values.clone();
    let measured = column.measured_values();
    let sorted = population::sorted(&measured);
    let statistics = population::box_statistics(&sorted, whiskers)?;
    let limit = column.limit.as_ref();
    let passing = limit.map_or(measured.len(), |limit| {
        measured
            .iter()
            .filter(|value| limit.passes(**value))
            .count()
    });
    Some(Column {
        name: column.name.clone(),
        values,
        sorted,
        statistics,
        limit: limit.cloned(),
        measured: measured.len(),
        passing,
        unit: column.unit.clone(),
        normalized: false,
    })
}

/// One measurement normalized to its own requirement, so every column shares
/// an ordinate and one limit line at zero.
fn normalized_column(
    column: &PopulationColumn,
    scale: MarginScale,
    whiskers: Whiskers,
) -> Option<Column> {
    let limit = column.limit.as_ref()?;
    let measured = column.measured_values();
    let sigma = population::std_dev(&measured);
    let project = |value: f64| -> Option<f64> {
        match scale {
            MarginScale::Sigma => {
                let sigma = sigma?;
                (sigma > 0.0).then_some(limit.signed_margin(value)? / sigma)
            }
            MarginScale::Percent => limit.margin_percent(value),
        }
    };
    let values = column
        .values
        .iter()
        .map(|value| value.and_then(project))
        .collect::<Vec<_>>();
    let projected = values.iter().copied().flatten().collect::<Vec<_>>();
    let sorted = population::sorted(&projected);
    let statistics = population::box_statistics(&sorted, whiskers)?;
    Some(Column {
        name: column.name.clone(),
        passing: projected.iter().filter(|margin| **margin >= 0.0).count(),
        measured: projected.len(),
        values,
        sorted,
        statistics,
        // Every normalized column is bounded below at zero, and keeps the
        // spelling of the requirement it was normalized to.
        limit: Some(PopulationLimit {
            min: Some(0.0),
            max: None,
            text: limit.text.clone(),
        }),
        // Spelled out rather than as σ: the bundled mono face — which every
        // exact value on this sheet is set in — has no sigma glyph, and a
        // unit that paints as a missing-glyph box is worse than a word.
        unit: match scale {
            MarginScale::Sigma => "sigma".to_owned(),
            MarginScale::Percent => "%".to_owned(),
        },
        normalized: true,
    })
}

// ---------------------------------------------------------------------------
// the sheet bar
// ---------------------------------------------------------------------------

const GROUP_OPTIONS: [&str; 3] = [
    "All measurements",
    "One measurement",
    "Corners \u{b7} none in this run",
];

/// The domain controls this sheet owns, drawn left-aligned in the sheet bar.
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let Some(plan) = population::plan(context) else {
        return false;
    };
    let sheet = context.results.box_violin.clone();
    let options = GROUP_OPTIONS.map(str::to_owned).to_vec();
    let selected = match sheet.grouping {
        BoxGrouping::AllMeasurements => 0,
        BoxGrouping::OneMeasurement => 1,
    };
    if let Some(picked) = select_with_disabled(
        ui,
        "rspice.results.box.grouping",
        "Distribution grouping",
        &options[selected],
        &options,
        &[(2, CORNER_REASON)],
        176.0,
    ) {
        context.results.box_violin.grouping = if picked == 1 {
            BoxGrouping::OneMeasurement
        } else {
            BoxGrouping::AllMeasurements
        };
    }

    match sheet.grouping {
        BoxGrouping::AllMeasurements => {
            let mut scale = usize::from(sheet.scale == MarginScale::Percent);
            if segmented(
                ui,
                "rspice.results.box.scale",
                &["\u{3c3} to limit", "% of limit"],
                &mut scale,
                SegmentedWidth::Natural,
            ) {
                context.results.box_violin.scale = if scale == 1 {
                    MarginScale::Percent
                } else {
                    MarginScale::Sigma
                };
            }
        }
        BoxGrouping::OneMeasurement => {
            let names = plan
                .columns
                .iter()
                .map(|column| column.name.clone())
                .collect::<Vec<_>>();
            let index = selected_column(&plan, &sheet).unwrap_or(0);
            if let Some(picked) = select(
                ui,
                "rspice.results.box.measurement",
                "Measured column",
                names.get(index).map_or("", String::as_str),
                &names,
                150.0,
            ) {
                context.results.box_violin.measurement = names.get(picked).cloned();
            }
        }
    }

    let mut body = usize::from(sheet.violin);
    if segmented(
        ui,
        "rspice.results.box.body",
        &["Box", "Box + violin"],
        &mut body,
        SegmentedWidth::Natural,
    ) {
        context.results.box_violin.violin = body == 1;
    }

    let mut whiskers = usize::from(sheet.whiskers == Whiskers::Extremes);
    if segmented(
        ui,
        "rspice.results.box.whiskers",
        &["1.5 IQR", "min\u{2013}max"],
        &mut whiskers,
        SegmentedWidth::Natural,
    ) {
        context.results.box_violin.whiskers = if whiskers == 1 {
            Whiskers::Extremes
        } else {
            Whiskers::Tukey
        };
    }

    let outliers = sheet.outliers;
    let response = chip(ui, "OUT", outliers).on_hover_text(if outliers {
        "Samples beyond the whiskers drawn individually"
    } else {
        "Samples beyond the whiskers hidden"
    });
    if response.clicked() {
        context.results.box_violin.outliers = !outliers;
    }

    let limits = context.results.show_spec_limits;
    let response = chip(ui, "LIM", limits).on_hover_text(if limits {
        "Requirement line drawn with the violating side shaded"
    } else {
        "Requirement line hidden"
    });
    if response.clicked() {
        context.results.show_spec_limits = !limits;
    }

    // No trailing readout: with the grouping select, two segmented controls
    // and the two toggles, the bar already fills the 1024 pt gate, and the
    // run's own facts read better beside the yield they qualify.
    true
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the box/violin sheet.
pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let Some(plan) = population::plan(context) else {
        well_hint(ui, ABSENT_STATE);
        return;
    };
    let sheet = context.results.box_violin.clone();
    let columns = match drawn_columns(&plan, &sheet) {
        Ok(columns) => columns,
        Err(reason) => {
            well_hint(ui, reason);
            return;
        }
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let show_limits = context.results.show_spec_limits;
    let failing = plan.failing_count();

    let subtitle = match sheet.grouping {
        BoxGrouping::AllMeasurements => format!(
            "{} measurement{} \u{b7} {} trials \u{b7} {} \u{b7} whiskers {}",
            columns.len(),
            if columns.len() == 1 { "" } else { "s" },
            plan.trial_count(),
            match sheet.scale {
                MarginScale::Sigma => "margin to limit in sample \u{3c3}",
                MarginScale::Percent => "margin as % of the requirement",
            },
            whisker_note(sheet.whiskers)
        ),
        BoxGrouping::OneMeasurement => format!(
            "{} \u{b7} {} trials \u{b7} {} \u{b7} whiskers {}",
            columns[0].name,
            plan.trial_count(),
            columns[0].limit_text(),
            whisker_note(sheet.whiskers)
        ),
    };
    let mut legend = vec![LegendChip {
        name: "quartile box",
        color: c.traces[1],
        on: true,
    }];
    if sheet.violin {
        legend.push(LegendChip {
            name: "kernel density",
            color: c.traces[4 % c.traces.len()],
            on: true,
        });
    }
    if failing > 0 {
        legend.push(LegendChip {
            name: "failing trial",
            color: c.err,
            on: true,
        });
    }
    if show_limits {
        legend.push(LegendChip {
            name: "requirement",
            color: c.err,
            on: true,
        });
    }
    strip::StripHeader::new("DIST", &subtitle, &legend).show(ui);

    let (y0, y1) = ordinate_range(&columns, show_limits);
    let slots = columns.len() as f64;
    let mut x_axis = Axis::with_ticks(
        0.0,
        slots,
        "",
        &(0..columns.len())
            .map(|slot| slot as f64 + 0.5)
            .collect::<Vec<_>>(),
    );
    for (slot, tick) in x_axis.ticks.iter_mut().enumerate() {
        tick.1 = columns[slot].name.clone();
    }
    let unit = columns[0].unit.clone();
    let accessible = accessible_detail(&columns);
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, Axis::linear(y0, y1, unit.clone()))
        .accessible_name("Population distribution")
        .accessible_detail(&accessible);
    spec.left_margin = 68.0;

    let painted = PaintedColumns {
        columns,
        status: plan.status.clone(),
        violin: sheet.violin,
        outliers: sheet.outliers,
        limits: show_limits,
        colors: ColumnColors {
            body: c.traces[4 % c.traces.len()].gamma_multiply(0.35),
            box_fill: c.traces[1].gamma_multiply(0.30),
            box_stroke: c.traces[1],
            median: c.text,
            mean: c.traces[0],
            whisker: c.text_dim,
            outlier: c.text_faint,
            failing: c.err,
            limit: c.err,
            wash: c.err.gamma_multiply(0.09),
        },
    };
    spec.underlay = Some(Box::new(move |painter, mapper| {
        painted.draw(painter, mapper)
    }));

    // A distribution has no viewport gesture of its own; Select keeps the
    // ordinate the one the register is quoting.
    plot::set_interaction_mode(ui.ctx(), InteractionMode::Select);
    let response = plot::show(ui, &spec, &mut context.results.cache, None, None);
    super::record_drawn_axes(context.results, super::ResultViewer::BoxViolin, &response);
}

const fn whisker_note(whiskers: Whiskers) -> &'static str {
    match whiskers {
        Whiskers::Tukey => "1.5 IQR",
        Whiskers::Extremes => "min to max",
    }
}

fn accessible_detail(columns: &[Column]) -> String {
    let mut detail = String::new();
    for column in columns {
        let yield_percent = column.yield_percent().map_or_else(
            || "no yield".to_owned(),
            |value| format!("{value:.1} % yield"),
        );
        detail.push_str(&format!(
            "{}: median {:.4}, {yield_percent}. ",
            column.name, column.statistics.median
        ));
    }
    detail
}

/// The ordinate the columns share, padded, always taking in the bound.
fn ordinate_range(columns: &[Column], show_limits: bool) -> (f64, f64) {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for column in columns {
        low = low.min(column.statistics.minimum);
        high = high.max(column.statistics.maximum);
        if show_limits {
            for (bound, _) in column.bounds() {
                low = low.min(bound);
                high = high.max(bound);
            }
        }
    }
    if !(low.is_finite() && high.is_finite()) {
        return (0.0, 1.0);
    }
    if (high - low).abs() <= f64::EPSILON * high.abs().max(1.0) {
        let pad = if low == 0.0 { 1.0 } else { low.abs() * 1.0e-3 };
        return (low - pad, high + pad);
    }
    let pad = (high - low) * 0.08;
    (low - pad, high + pad)
}

#[derive(Clone, Copy)]
struct ColumnColors {
    body: egui::Color32,
    box_fill: egui::Color32,
    box_stroke: egui::Color32,
    median: egui::Color32,
    mean: egui::Color32,
    whisker: egui::Color32,
    outlier: egui::Color32,
    failing: egui::Color32,
    limit: egui::Color32,
    wash: egui::Color32,
}

struct PaintedColumns {
    columns: Vec<Column>,
    status: Vec<TrialStatus>,
    violin: bool,
    outliers: bool,
    limits: bool,
    colors: ColumnColors,
}

/// How many points the kernel density is evaluated at down one column.
const VIOLIN_STEPS: usize = 64;

/// The widest a single column is drawn, in points. Without it a lone
/// measurement's violin spans the whole well.
const COLUMN_MAX_WIDTH: f32 = 190.0;

impl PaintedColumns {
    fn draw(&self, painter: &egui::Painter, mapper: &plot::PlotMapper) {
        let slot_width = mapper.rect.width() / self.columns.len().max(1) as f32;
        // A single measurement owns the whole well, and a violin drawn at
        // 42 % of it is a wash, not a column. The drawn width is capped so
        // one column reads as a column and many still share the well evenly.
        let drawn = slot_width.min(COLUMN_MAX_WIDTH);
        for (slot, column) in self.columns.iter().enumerate() {
            let centre = mapper.rect.left() + slot_width * (slot as f32 + 0.5);
            // In the normalized grouping the requirement is not an overlay:
            // it is the origin of the ordinate every column is measured on,
            // so it is drawn whether or not the limit overlay is armed.
            if self.limits || column.normalized {
                self.draw_bounds(painter, mapper, column, centre, slot_width);
            }
            if self.violin {
                self.draw_violin(painter, mapper, column, centre, drawn);
            }
            self.draw_box(painter, mapper, column, centre, drawn);
            self.draw_samples(painter, mapper, column, centre, drawn);
        }
    }

    fn draw_bounds(
        &self,
        painter: &egui::Painter,
        mapper: &plot::PlotMapper,
        column: &Column,
        centre: f32,
        slot_width: f32,
    ) {
        let half = slot_width * 0.5;
        let span = egui::Rangef::new(centre - half, centre + half);
        for (bound, breaks_below) in column.bounds() {
            let y = mapper.y(bound);
            if !mapper.rect.y_range().contains(y) {
                continue;
            }
            // A lower bound is broken from underneath, an upper one from
            // above; the wash covers exactly the side that violates.
            let wash = if breaks_below {
                egui::Rect::from_min_max(
                    egui::pos2(span.min, y),
                    egui::pos2(span.max, mapper.rect.bottom()),
                )
            } else {
                egui::Rect::from_min_max(
                    egui::pos2(span.min, mapper.rect.top()),
                    egui::pos2(span.max, y),
                )
            };
            painter.rect_filled(wash.intersect(mapper.rect), 0.0, self.colors.wash);
            painter.line_segment(
                [egui::pos2(span.min, y), egui::pos2(span.max, y)],
                egui::Stroke::new(1.2, self.colors.limit),
            );
        }
    }

    fn draw_violin(
        &self,
        painter: &egui::Painter,
        mapper: &plot::PlotMapper,
        column: &Column,
        centre: f32,
        slot_width: f32,
    ) {
        let Some(bandwidth) = population::silverman_bandwidth(&column.sorted) else {
            return;
        };
        let low = column.statistics.minimum - bandwidth;
        let high = column.statistics.maximum + bandwidth;
        let step = (high - low) / VIOLIN_STEPS as f64;
        if !(step.is_finite() && step > 0.0) {
            return;
        }
        let densities = (0..=VIOLIN_STEPS)
            .map(|index| {
                let at = low + step * index as f64;
                (
                    at,
                    population::kernel_density(&column.sorted, bandwidth, at),
                )
            })
            .collect::<Vec<_>>();
        let peak = densities
            .iter()
            .map(|(_, density)| *density)
            .fold(0.0_f64, f64::max);
        if peak <= 0.0 {
            return;
        }
        let half = slot_width * 0.42;
        // One trapezoid per density step rather than one outline: a violin is
        // not convex, and `convex_polygon` fans its vertices, which draws a
        // chord straight across the waist.
        for pair in densities.windows(2) {
            let [(low_at, low_density), (high_at, high_density)] = [pair[0], pair[1]];
            let (low_y, high_y) = (mapper.y(low_at), mapper.y(high_at));
            let low_width = half * (low_density / peak) as f32;
            let high_width = half * (high_density / peak) as f32;
            if low_width <= 0.05 && high_width <= 0.05 {
                continue;
            }
            painter.add(egui::Shape::convex_polygon(
                vec![
                    egui::pos2(centre - low_width, low_y),
                    egui::pos2(centre + low_width, low_y),
                    egui::pos2(centre + high_width, high_y),
                    egui::pos2(centre - high_width, high_y),
                ],
                self.colors.body,
                egui::Stroke::NONE,
            ));
        }
    }

    fn draw_box(
        &self,
        painter: &egui::Painter,
        mapper: &plot::PlotMapper,
        column: &Column,
        centre: f32,
        slot_width: f32,
    ) {
        let statistics = column.statistics;
        let half = slot_width * 0.14;
        let box_rect = egui::Rect::from_min_max(
            egui::pos2(centre - half, mapper.y(statistics.q3)),
            egui::pos2(centre + half, mapper.y(statistics.q1)),
        );
        painter.rect(
            box_rect,
            0.0,
            self.colors.box_fill,
            egui::Stroke::new(1.0, self.colors.box_stroke),
            egui::StrokeKind::Inside,
        );
        let median_y = mapper.y(statistics.median);
        painter.line_segment(
            [
                egui::pos2(centre - half, median_y),
                egui::pos2(centre + half, median_y),
            ],
            egui::Stroke::new(1.6, self.colors.median),
        );
        // Whiskers, with a cap at each end.
        let whisker = egui::Stroke::new(1.0, self.colors.whisker);
        for (from, to) in [
            (statistics.q3, statistics.whisker_high),
            (statistics.q1, statistics.whisker_low),
        ] {
            painter.line_segment(
                [
                    egui::pos2(centre, mapper.y(from)),
                    egui::pos2(centre, mapper.y(to)),
                ],
                whisker,
            );
            let cap = half * 0.6;
            painter.line_segment(
                [
                    egui::pos2(centre - cap, mapper.y(to)),
                    egui::pos2(centre + cap, mapper.y(to)),
                ],
                whisker,
            );
        }
        // The mean as a diamond, so it is never mistaken for the median.
        let mean_y = mapper.y(statistics.mean);
        let arm = 3.5;
        painter.add(egui::Shape::convex_polygon(
            vec![
                egui::pos2(centre, mean_y - arm),
                egui::pos2(centre + arm, mean_y),
                egui::pos2(centre, mean_y + arm),
                egui::pos2(centre - arm, mean_y),
            ],
            self.colors.mean,
            egui::Stroke::NONE,
        ));
    }

    /// Outliers and failing trials, drawn individually because a summary that
    /// hides the trial nobody can reproduce is not evidence.
    fn draw_samples(
        &self,
        painter: &egui::Painter,
        mapper: &plot::PlotMapper,
        column: &Column,
        centre: f32,
        slot_width: f32,
    ) {
        let statistics = column.statistics;
        for (row, value) in column
            .values
            .iter()
            .enumerate()
            .filter_map(|(row, value)| value.map(|value| (row, value)))
        {
            let failing = self.status.get(row) == Some(&TrialStatus::Failing);
            let outlier = value < statistics.whisker_low || value > statistics.whisker_high;
            if !(failing || self.outliers && outlier) {
                continue;
            }
            let y = mapper.y(value);
            if !mapper.rect.y_range().contains(y) {
                continue;
            }
            // Spread the marks across the slot deterministically, so a stack
            // of equal values is countable instead of one dot.
            let offset = slot_width * 0.30 * (((row % 7) as f32 / 6.0) - 0.5);
            let at = egui::pos2(centre + offset, y);
            if failing {
                let arm = 3.0;
                let stroke = egui::Stroke::new(1.3, self.colors.failing);
                painter.line_segment(
                    [at + egui::vec2(-arm, -arm), at + egui::vec2(arm, arm)],
                    stroke,
                );
                painter.line_segment(
                    [at + egui::vec2(-arm, arm), at + egui::vec2(arm, -arm)],
                    stroke,
                );
            } else {
                painter.circle_stroke(at, 2.0, egui::Stroke::new(1.0, self.colors.outlier));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Per-column yield, then the statistics of the column in hand.
pub fn right_panel(ui: &mut Ui, context: &mut SheetContext<'_>) {
    section_header(ui, "Distribution", None);
    let Some(plan) = population::plan(context) else {
        super::panel_note(
            ui,
            "Statistics appear once a Monte Carlo population is loaded.",
        );
        return;
    };
    let sheet = context.results.box_violin.clone();
    let columns = match drawn_columns(&plan, &sheet) {
        Ok(columns) => columns,
        Err(reason) => {
            super::panel_note(ui, reason);
            return;
        }
    };

    // In the all-measurements grouping the register lists every column's
    // verdict and then details the one in hand. The mockup's rotated matrix
    // head does not fit a 292 pt register, and a column of statistics per
    // measurement would elide to nothing.
    let mut chosen = 0;
    if sheet.grouping == BoxGrouping::AllMeasurements {
        let mut request = None;
        for (slot, column) in columns.iter().enumerate() {
            let selected = sheet
                .measurement
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(&column.name));
            if selected {
                chosen = slot;
            }
            let verdict = column.yield_percent().map_or_else(
                || "no requirement".to_owned(),
                |value| {
                    format!(
                        "{value:.2} % \u{b7} {} fail",
                        column.measured - column.passing
                    )
                },
            );
            let response = ui.selectable_label(selected, format!("{}  {verdict}", column.name));
            if response.clicked() {
                request = Some(column.name.clone());
            }
        }
        if let Some(name) = request {
            context.results.box_violin.measurement = Some(name);
        }
        ui.separator();
    }

    let column = &columns[chosen.min(columns.len() - 1)];
    let rows = statistics_rows(column, &plan);
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        match sheet.grouping {
            BoxGrouping::AllMeasurements => {
                "Each column is the signed distance from its own requirement, so the limit is \
                 one line at zero and positive passes. The violin width is scaled per column \
                 to its own peak density."
            }
            BoxGrouping::OneMeasurement => {
                "Quartiles are the interpolated definition; the yield interval is Wilson's, \
                 which keeps a bound at a hundred percent. A trial is reproduced from the \
                 seed in its name; no per-trial waveform set is retained."
            }
        },
    );
}

fn statistics_rows(column: &Column, plan: &PopulationPlan) -> Vec<(&'static str, String, bool)> {
    let statistics = column.statistics;
    let value = |value: f64| column.format(value);
    let outliers = column
        .sorted
        .iter()
        .filter(|sample| **sample < statistics.whisker_low || **sample > statistics.whisker_high)
        .count();
    let yield_row = column.yield_percent().map_or_else(
        || "No requirement bounds this column".to_owned(),
        |percent| {
            population::wilson_interval(column.passing, column.measured).map_or_else(
                || format!("{percent:.2} %"),
                |(low, high)| format!("{percent:.2} % \u{b7} 95 % CI {low:.2}\u{2013}{high:.2}"),
            )
        },
    );
    let capability = column
        .limit
        .as_ref()
        .and_then(|limit| population::cpk(&column.sorted, limit));
    let worst = worst_trial(column, plan);

    let mut run = format!(
        "{} of {} completed \u{b7} seed 0x{:X}",
        plan.runs_completed, plan.runs_requested, plan.seed
    );
    if plan.failures > 0 {
        run.push_str(&format!(" \u{b7} {} diverged", plan.failures));
    }

    vec![
        (
            "Trials",
            format!("{} measured of {}", column.measured, plan.trial_count()),
            true,
        ),
        ("Run", run, false),
        ("Requirement", column.limit_text(), false),
        ("Median", value(statistics.median), true),
        ("Q1", value(statistics.q1), false),
        ("Q3", value(statistics.q3), false),
        ("IQR", value(statistics.iqr()), false),
        (
            "Whiskers",
            format!(
                "{} \u{2026} {}",
                value(statistics.whisker_low),
                value(statistics.whisker_high)
            ),
            false,
        ),
        ("Mean", value(statistics.mean), false),
        (
            "Std dev",
            population::std_dev(&column.sorted).map_or_else(|| "\u{2014}".to_owned(), value),
            false,
        ),
        (
            "Range",
            format!(
                "{} \u{2026} {}",
                value(statistics.minimum),
                value(statistics.maximum)
            ),
            false,
        ),
        ("Outliers", outliers.to_string(), false),
        ("Yield", yield_row, true),
        (
            "Cpk",
            capability.map_or_else(
                || "Not defined for this requirement".to_owned(),
                |value| format!("{value:.2}"),
            ),
            false,
        ),
        (
            "Worst trial",
            worst.unwrap_or_else(|| "\u{2014}".to_owned()),
            false,
        ),
    ]
}

/// The trial furthest into the violating side, named the way it is reproduced.
fn worst_trial(column: &Column, plan: &PopulationPlan) -> Option<String> {
    let (row, value) = column
        .values
        .iter()
        .enumerate()
        .filter_map(|(row, value)| value.map(|value| (row, value)))
        .min_by(|left, right| {
            // With a requirement, the worst trial is the one with the least
            // margin; without one, the sample furthest from the median.
            let score = |value: f64| {
                column.limit.as_ref().map_or_else(
                    || -(value - column.statistics.median).abs(),
                    |limit| limit.signed_margin(value).unwrap_or(f64::INFINITY),
                )
            };
            score(left.1).total_cmp(&score(right.1))
        })?;
    let label = plan
        .trials
        .get(row)
        .map_or_else(|| format!("Trial {row}"), |trial| trial.label.clone());
    Some(format!("{label} \u{b7} {}", column.format(value)))
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

    /// Two measured columns with different units and different bound shapes,
    /// which is exactly what the normalization exists for.
    fn population_analysis(trials: usize) -> AnalysisResult {
        let members = (0..trials)
            .map(|index| {
                let position = (index as f64 - trials as f64 / 2.0) / (trials as f64 / 2.0);
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 0x73a4 + index as u64,
                    },
                    vec![
                        FamilyMeasurementEvidence {
                            name: "gain_dc".to_owned(),
                            value: Some(40.0 + 2.0 * position),
                            passed: true,
                            error: None,
                        },
                        FamilyMeasurementEvidence {
                            name: "vos".to_owned(),
                            value: Some(60.0 * position),
                            passed: true,
                            error: None,
                        },
                    ],
                )
            })
            .collect::<Vec<_>>();
        let samples: Vec<f64> = (0..trials).map(|index| index as f64).collect();
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x73a4,
                runs_requested: trials,
                runs_completed: trials,
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    name: "RGAIN.r".to_owned(),
                    mean: (trials as f64 - 1.0) / 2.0,
                    std_dev: population::std_dev(&samples).expect("a spread"),
                    min: 0.0,
                    max: trials as f64 - 1.0,
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
        workspace.specs.push(SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: String::new(),
            min: Some(39.5),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        });
        workspace.specs.push(SpecEntry {
            measurement: "vos".to_owned(),
            expression: String::new(),
            min: Some(-50.0),
            max: Some(50.0),
            unit: "V".to_owned(),
            scope: SpecPointScope::AllPoints,
        });
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

    /// The all-measurements grouping draws only what a requirement bounds,
    /// because there is nothing to normalize an unbounded column to.
    #[test]
    fn the_normalized_grouping_draws_only_the_bounded_measurements() {
        let (simulation, workspace, mut results) = fixture(101);
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let columns = drawn_columns(&plan, &ctx.results.box_violin).expect("two bounded columns");

        assert_eq!(
            columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
            ["gain_dc", "vos"],
            "the sampled variable has no requirement and must not be normalized"
        );
        assert!(
            columns
                .iter()
                .all(|column| column.bounds() == [(0.0, true)])
        );
        assert_eq!(columns[0].unit, "sigma");
    }

    /// The σ normalization is the signed margin over the sample σ, so three
    /// of them is exactly a capability of one — the property the scale is
    /// chosen for.
    #[test]
    fn three_sigma_of_margin_lands_at_three_on_the_normalized_ordinate() {
        let (simulation, workspace, mut results) = fixture(1_001);
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let gain = &plan.columns[plan.column_index("gain_dc").expect("the bounded column")];
        let sigma = population::std_dev(&gain.measured_values()).expect("a spread");
        let limit = gain.limit.as_ref().expect("a requirement");

        let normalized =
            normalized_column(gain, MarginScale::Sigma, Whiskers::Tukey).expect("a column");
        let raw = gain.values[0].expect("the first trial measured");
        let expected = limit.signed_margin(raw).expect("a margin") / sigma;
        let drawn = normalized.values[0].expect("the first trial is on the ordinate");
        assert!((drawn - expected).abs() < 1.0e-12, "{drawn} vs {expected}");

        // And the percentage spelling is the Specs sheet's margin.
        let percent =
            normalized_column(gain, MarginScale::Percent, Whiskers::Tukey).expect("a column");
        assert_eq!(percent.values[0], limit.margin_percent(raw));
        assert_eq!(percent.unit, "%");
    }

    /// A trial on the wrong side of the bound lands below zero on the shared
    /// ordinate, and the column's yield counts it.
    #[test]
    fn a_failing_trial_lands_below_zero_and_is_counted() {
        let (simulation, workspace, mut results) = fixture(101);
        let mut ctx = context(&simulation, &workspace, &mut results);
        let plan = population::plan(&mut ctx).expect("a population");
        let columns = drawn_columns(&plan, &ctx.results.box_violin).expect("two bounded columns");
        let gain = &columns[0];

        assert_eq!(gain.measured, 101);
        assert_eq!(
            gain.measured - gain.passing,
            38,
            "gain_dc = 40 + 2p over p in [-1, 1] leaves every p under -0.25 below 39.5 dB"
        );
        let below = gain
            .values
            .iter()
            .flatten()
            .filter(|margin| **margin < 0.0)
            .count();
        assert_eq!(below, 38);
        let percent = gain.yield_percent().expect("a yield");
        assert!((percent - 100.0 * 63.0 / 101.0).abs() < 1.0e-9, "{percent}");
    }

    /// The single-measurement grouping keeps the column's own unit and states
    /// its own requirement, rather than a normalized ordinate.
    #[test]
    fn the_single_grouping_keeps_the_engineering_unit() {
        let (simulation, workspace, mut results) = fixture(101);
        let mut ctx = context(&simulation, &workspace, &mut results);
        ctx.results.box_violin.grouping = BoxGrouping::OneMeasurement;
        ctx.results.box_violin.measurement = Some("vos".to_owned());
        let plan = population::plan(&mut ctx).expect("a population");
        let columns = drawn_columns(&plan, &ctx.results.box_violin).expect("one column");

        assert_eq!(columns.len(), 1);
        assert_eq!(columns[0].name, "vos");
        assert_eq!(columns[0].unit, "V");
        assert_eq!(columns[0].bounds(), [(-50.0, true), (50.0, false)]);
        assert_eq!(columns[0].limit_text(), "-50 \u{2026} 50 V");
    }

    /// A sampled variable has no requirement, and the register says so rather
    /// than inventing a yield for it.
    #[test]
    fn a_sampled_variable_is_drawable_and_reports_no_requirement() {
        let (simulation, workspace, mut results) = fixture(64);
        let mut ctx = context(&simulation, &workspace, &mut results);
        ctx.results.box_violin.grouping = BoxGrouping::OneMeasurement;
        ctx.results.box_violin.measurement = Some("RGAIN.r".to_owned());
        let plan = population::plan(&mut ctx).expect("a population");
        let columns = drawn_columns(&plan, &ctx.results.box_violin).expect("one column");

        assert_eq!(columns[0].name, "RGAIN.r");
        assert_eq!(columns[0].yield_percent(), Some(100.0));
        let rows = statistics_rows(&columns[0], &plan);
        let by_name = |needle: &str| {
            rows.iter()
                .find(|row| row.0 == needle)
                .map(|row| row.1.clone())
                .unwrap_or_else(|| panic!("no {needle} row"))
        };
        assert_eq!(by_name("Requirement"), "No requirement");
        assert_eq!(by_name("Cpk"), "Not defined for this requirement");
    }

    /// The register states the yield with the interval the sample supports,
    /// and names the worst trial by the seed that reproduces it.
    #[test]
    fn the_register_states_the_yield_interval_and_names_the_worst_trial() {
        let (simulation, workspace, mut results) = fixture(101);
        let mut ctx = context(&simulation, &workspace, &mut results);
        ctx.results.box_violin.grouping = BoxGrouping::OneMeasurement;
        ctx.results.box_violin.measurement = Some("gain_dc".to_owned());
        let plan = population::plan(&mut ctx).expect("a population");
        let columns = drawn_columns(&plan, &ctx.results.box_violin).expect("one column");
        let rows = statistics_rows(&columns[0], &plan);
        let by_name = |needle: &str| {
            rows.iter()
                .find(|row| row.0 == needle)
                .map(|row| row.1.clone())
                .unwrap_or_else(|| panic!("no {needle} row"))
        };

        assert!(by_name("Yield").contains("95 % CI"), "{}", by_name("Yield"));
        assert!(
            by_name("Worst trial").starts_with("Trial 0 \u{b7} seed 29604"),
            "{}",
            by_name("Worst trial")
        );
        assert_eq!(by_name("Requirement"), "\u{2265} 39.500 dB");
        assert_eq!(by_name("Trials"), "101 measured of 101");
    }

    /// The corner grouping is offered and refused in place, with the reason
    /// the engine actually has.
    #[test]
    fn the_corner_grouping_is_offered_and_refused_with_its_reason() {
        assert_eq!(GROUP_OPTIONS[2], "Corners \u{b7} none in this run");
        assert!(CORNER_REASON.contains("no runner produces a Monte Carlo per corner"));
        assert_eq!(
            ABSENT_STATE,
            "No family distribution in this run — run a Monte Carlo analysis"
        );
    }
}
