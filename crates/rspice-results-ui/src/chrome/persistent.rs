//! Persistent Results page geometry and pane chrome over retained display data.

use egui::{Align, Layout, Rect, RichText, Sense, Ui, UiBuilder, pos2, vec2};
use rspice_results::number_format::fmt_si_significant;
use rspice_results::visualization_document::{
    Annotation, FamilyMeasurementSummary, FamilyStatistic, PageLayout, PaneId,
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

/// Allocate each retained pane once in a clipped slot, preserving document order.
pub fn page<R>(
    ui: &mut Ui,
    layout: PageLayout,
    pane_ids: impl ExactSizeIterator<Item = PaneId>,
    mut render_pane: impl FnMut(&mut Ui, usize) -> Option<R>,
) -> Option<R> {
    // Reserve the stage exactly once. Pane viewers render into clipped slots
    // inside that fixed rectangle, so their own minimum sizes and scroll areas
    // cannot enlarge the Results document or create recursive scroll growth.
    let size = finite_stage_size(ui.available_size());
    let (stage_rect, _) = ui.allocate_exact_size(size, Sense::hover());
    // The gutters between panes are the workspace's own ground, so each pane
    // reads as a card of its own and one pane's ruler never meets the next
    // pane's analysis row.
    ui.painter()
        .rect_filled(stage_rect, 0.0, Tokens::get(ui.ctx()).color.bg_app);
    let slots = bounded_pane_slots(stage_rect, layout, pane_ids.len());
    let mut activated_pane_id = None;
    for (index, (pane_id, rect)) in pane_ids.zip(slots).enumerate() {
        let mut pane_ui = ui.new_child(
            UiBuilder::new()
                .id_salt(("persistent-result-pane-slot", pane_id.get()))
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        pane_ui.set_clip_rect(pane_ui.clip_rect().intersect(rect));
        let activated = render_pane(&mut pane_ui, index);
        activated_pane_id = activated.or(activated_pane_id);
    }
    activated_pane_id
}

/// The gutter between two panes, across and down alike.
const PANE_GAP: f32 = 4.0;

fn finite_stage_size(size: egui::Vec2) -> egui::Vec2 {
    vec2(finite_extent(size.x), finite_extent(size.y))
}

fn finite_extent(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// One pane's extent along an axis of `count` panes, in whole points so
/// the panes are the same size to a point and every gutter the same width,
/// and the gutter between two of them.
fn bounded_axis_cells(extent: f32, count: usize) -> (f32, f32) {
    let count = count.max(1);
    if count == 1 {
        return (extent, 0.0);
    }
    let gap = PANE_GAP.min(extent / (count - 1) as f32);
    let cell = ((extent - gap * (count - 1) as f32) / count as f32)
        .floor()
        .max(0.0);
    (cell, gap)
}

/// How many columns and rows `layout` sets `pane_count` panes in.
fn grid_of(layout: PageLayout, pane_count: usize) -> (usize, usize) {
    let pane_count = pane_count.max(1);
    match layout {
        PageLayout::SinglePane => (1, 1),
        PageLayout::Rows => (1, pane_count),
        PageLayout::Columns => (pane_count, 1),
        PageLayout::Grid { columns } => {
            let columns = usize::from(columns.max(1)).min(pane_count);
            (columns, pane_count.div_ceil(columns))
        }
    }
}

/// Where the pane at `index` of `pane_count` sits on a page laid out as
/// `layout`, in the reader's words: "Top", "Bottom right", "Row 4". A page of
/// one pane has no place to name.
pub fn pane_place(layout: PageLayout, index: usize, pane_count: usize) -> Option<String> {
    if pane_count < 2 || index >= pane_count {
        return None;
    }
    let (columns, rows) = grid_of(layout, pane_count);
    let (column, row) = (index % columns, index / columns);
    // A side of two or three cells is named by its ends and middle; a longer
    // one is counted.
    let side = |at: usize, cells: usize, ends: [&'static str; 2]| match (cells, at) {
        (1, _) => Some(None),
        (2 | 3, 0) => Some(Some(ends[0])),
        (2, 1) | (3, 2) => Some(Some(ends[1])),
        (3, 1) => Some(Some("middle")),
        _ => None,
    };
    let place = match (
        side(row, rows, ["top", "bottom"]),
        side(column, columns, ["left", "right"]),
    ) {
        (Some(Some("middle")), Some(Some("middle"))) => "middle".to_owned(),
        (Some(Some(vertical)), Some(Some(horizontal))) => format!("{vertical} {horizontal}"),
        (Some(Some(side)), Some(None)) | (Some(None), Some(Some(side))) => side.to_owned(),
        _ if columns == 1 => format!("row {}", row + 1),
        _ if rows == 1 => format!("column {}", column + 1),
        _ => format!("row {}, column {}", row + 1, column + 1),
    };
    let mut letters = place.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
}

fn bounded_pane_slots(stage: Rect, layout: PageLayout, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }
    let stage = Rect::from_min_size(stage.min, finite_stage_size(stage.size()));
    let (columns, rows) = grid_of(layout, pane_count);
    let (pane_width, horizontal_gap) = bounded_axis_cells(stage.width(), columns);
    let (pane_height, vertical_gap) = bounded_axis_cells(stage.height(), rows);

    (0..pane_count)
        .map(|index| {
            let column = index % columns;
            let row = index / columns;
            let min = pos2(
                stage.left() + column as f32 * (pane_width + horizontal_gap),
                stage.top() + row as f32 * (pane_height + vertical_gap),
            );
            // The last column and row take the stage's odd points, so the
            // panes meet its edges rather than leave a sliver of ground.
            let max = pos2(
                if column + 1 == columns {
                    stage.right()
                } else {
                    min.x + pane_width
                },
                if row + 1 == rows {
                    stage.bottom()
                } else {
                    min.y + pane_height
                },
            );
            Rect::from_min_max(min, max).intersect(stage)
        })
        .collect()
}

fn bounded_inset(rect: Rect, requested: f32) -> Rect {
    let inset = requested
        .max(0.0)
        .min(rect.width() * 0.5)
        .min(rect.height() * 0.5);
    rect.shrink(inset)
}

/// Draw a pane frame around host-composed controls and its qualified viewer.
///
/// Every pane has the same hairline frame. The pane being worked in, when
/// it is `marked`, is told apart by its analysis rows alone, which take the
/// selected-row fill: an accent frame round the plot fought the traces.
pub fn pane_frame<R>(
    ui: &mut Ui,
    pane_id: PaneId,
    marked: bool,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    if marked {
        crate::strip::select_header_rows(ui.ctx(), rect);
    }
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().rect_stroke(
        rect,
        0.0,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("persistent-result-pane", pane_id.get()))
            .max_rect(bounded_inset(rect, 1.0))
            .layout(Layout::top_down(Align::Min)),
        add_contents,
    )
    .inner
}

/// One kept measurement as the pane read it on the run it shows.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementReadout {
    /// The measurement's name.
    pub name: String,
    /// What it read and in what unit, or why it read nothing.
    pub reading: Result<(f64, String), String>,
    /// Over a family, the statistic it reports and what the members read.
    pub family: Option<FamilyMeasurementSummary>,
}

/// The pane's kept measurements, as read on its run, and its notes, in one
/// line above its viewer, after the plotted signals the run it reads does
/// not have. Values carry the document's significant digits.
pub fn evidence_bar(
    ui: &mut Ui,
    pane_id: PaneId,
    missing_signals: &[&str],
    measurements: &[MeasurementReadout],
    annotations: &[Annotation],
    significant_digits: usize,
) {
    if missing_signals.is_empty() && measurements.is_empty() && annotations.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(6, 3))
        .show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .id_salt(("persistent-pane-evidence", pane_id.get()))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for name in missing_signals {
                            ui.label(
                                RichText::new(format!("{name} not in this run"))
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.warn),
                            );
                        }
                        for measurement in measurements {
                            let statistic = measurement.family.as_ref().map(|f| f.statistic);
                            let name = statistic.map_or_else(
                                || measurement.name.clone(),
                                |statistic| format!("{} {}", measurement.name, statistic.label()),
                            );
                            let (text, color) = match &measurement.reading {
                                Ok((value, unit)) => (
                                    measurement_text(
                                        &name,
                                        Ok((*value, unit, statistic)),
                                        significant_digits,
                                    ),
                                    t.color.text,
                                ),
                                Err(why) => (
                                    measurement_text(&name, Err(why), significant_digits),
                                    t.color.text_dim,
                                ),
                            };
                            ui.label(
                                RichText::new(text)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(color),
                            );
                            if let Some(family) = &measurement.family {
                                ui.label(
                                    RichText::new(family_text(family, significant_digits))
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                );
                            }
                        }
                        for annotation in annotations {
                            ui.label(
                                RichText::new(format!("Note: {}", annotation.text))
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                        }
                    });
                });
        });
}

/// One measurement as the evidence bar states it: its name, and what it
/// read in engineering notation in its unit, or why it read nothing.
///
/// A family measurement's value is the statistic it names; a yield is a
/// percentage.
fn measurement_text(
    name: &str,
    reading: Result<(f64, &str, Option<FamilyStatistic>), &str>,
    significant_digits: usize,
) -> String {
    match reading {
        Ok((value, _, Some(FamilyStatistic::Yield))) => format!("{name} = {}", percent(value)),
        Ok((value, unit, _)) => {
            format!(
                "{name} = {}",
                fmt_si_significant(value, unit, significant_digits)
            )
        }
        Err(why) => format!("{name}: {why}"),
    }
}

/// The most failing members a family measurement names before it counts
/// the rest: the bar is one line, and the population's own views list
/// every trial.
const NAMED_FAILURES: usize = 2;

/// What a family measurement's members added up to, after its value: how
/// many were measured, the distribution, and the yield against its limits
/// with the members that failed.
fn family_text(family: &FamilyMeasurementSummary, significant_digits: usize) -> String {
    let si = |value: f64| fmt_si_significant(value, "", significant_digits);
    let mut parts = vec![if family.measured == family.members {
        format!("n {}", family.members)
    } else {
        format!("n {} of {}", family.measured, family.members)
    }];
    parts.push(format!("mean {}", si(family.mean)));
    parts.push(format!(
        "\u{3c3} {}",
        family.std_dev.map_or_else(|| "\u{2014}".to_owned(), si)
    ));
    parts.push(format!("min {}", si(family.min)));
    parts.push(format!("max {}", si(family.max)));
    if let (Some(passed), Some(share)) = (family.passed, family.yield_percent()) {
        let limits = match (family.lower, family.upper) {
            (Some(lower), Some(upper)) => format!("[{}, {}]", si(lower), si(upper)),
            (Some(lower), None) => format!("\u{2265} {}", si(lower)),
            (None, Some(upper)) => format!("\u{2264} {}", si(upper)),
            (None, None) => String::new(),
        };
        // A kept yield already leads the line with its percentage.
        let share = if family.statistic == FamilyStatistic::Yield {
            String::new()
        } else {
            format!(" ({})", percent(share))
        };
        parts.push(format!("yield {passed}/{}{share} {limits}", family.members));
        if !family.failing.is_empty() {
            let mut named = family.failing[..family.failing.len().min(NAMED_FAILURES)].join(", ");
            if family.failing.len() > NAMED_FAILURES {
                named.push_str(&format!(" +{}", family.failing.len() - NAMED_FAILURES));
            }
            parts.push(format!("fails {named}"));
        }
    }
    parts.join(" \u{b7} ")
}

/// A share in percent, to one decimal unless it is whole.
fn percent(share: f64) -> String {
    if share.fract() == 0.0 {
        format!("{share:.0} %")
    } else {
        format!("{share:.1} %")
    }
}

/// What the reader asked of one pane through its menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneAction {
    /// Move this pane one place earlier in its page's order.
    MoveEarlier,
    /// Move this pane one place later in its page's order.
    MoveLater,
    /// Share this pane's X range with the rest of its page, or stop sharing it.
    ToggleLinkX,
    /// Take this pane out of the document.
    Remove,
}

/// Which of the pane menu's edits the host can carry out.
#[derive(Debug, Clone, Copy, Default)]
pub struct PaneMenu {
    /// The pane has a predecessor on its page to change places with.
    pub earlier: bool,
    /// The pane has a successor on its page to change places with.
    pub later: bool,
    /// Whether the pane's X axis is linked, or `None` when no other pane on
    /// its page has an X axis it could share.
    pub x_link: Option<bool>,
    /// A page keeps at least one pane, so its last one cannot be removed.
    pub removable: bool,
}

/// The pane's menu, on the row of every strip the pane drew inside `rect`:
/// a right-click on the row, or on the analysis chip it starts with, offers
/// to move the pane, link its X axis, or remove it. Returns what the reader
/// chose; the host validates and applies it.
pub fn pane_menu(ui: &Ui, rect: Rect, menu: PaneMenu) -> Option<PaneAction> {
    let mut action = None;
    for row in crate::strip::header_rows(ui.ctx(), rect) {
        crate::context_menu::context_menu(&row, |ui| {
            let mut item = |ui: &mut Ui, enabled: bool, label: &str, chosen: PaneAction| {
                if ui.add_enabled(enabled, egui::Button::new(label)).clicked() {
                    action = Some(chosen);
                    ui.close();
                }
            };
            item(
                ui,
                menu.earlier,
                "Move pane earlier",
                PaneAction::MoveEarlier,
            );
            item(ui, menu.later, "Move pane later", PaneAction::MoveLater);
            ui.separator();
            item(
                ui,
                menu.x_link.is_some(),
                if menu.x_link == Some(true) {
                    "Unlink X axis"
                } else {
                    "Link X axis"
                },
                PaneAction::ToggleLinkX,
            );
            ui.separator();
            item(ui, menu.removable, "Remove pane", PaneAction::Remove);
        });
    }
    action
}

/// Whether a press this frame landed in the pane at `rect` with nothing
/// over it there: the press that makes a pane the one worked in.
pub fn pressed_in(ui: &Ui, rect: Rect) -> bool {
    ui.input(|input| input.pointer.any_pressed()) && ui.rect_contains_pointer(rect)
}

/// State why Latest tracking cannot advance while preserving the retained view.
pub fn tracking_banner(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let height = ui.text_style_height(&egui::TextStyle::Body) + 10.0;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    // The mark says this is a warning; the reason stays neutral text.
    rspice_ui_kit::widgets::paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(
            pos2(rect.left() + 13.5, rect.center().y),
            egui::Vec2::splat(11.0),
        ),
        rspice_ui_kit::widgets::Problem::Warning,
    );
    ui.painter().text(
        pos2(rect.left() + 25.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        reason,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, reason));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(reason);
        node.set_description(rspice_ui_kit::widgets::Problem::Warning.announcement());
    });
}

pub fn unavailable_surface(ui: &mut Ui, title: &str, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect.shrink(20.0))
            .layout(Layout::top_down_justified(Align::Center)),
        |ui| {
            ui.add_space((ui.available_height() * 0.35).max(0.0));
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.canvas_text),
            );
            ui.label(
                RichText::new(reason)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.canvas_text_dim),
            );
        },
    );
}

#[cfg(test)]
mod tests;
