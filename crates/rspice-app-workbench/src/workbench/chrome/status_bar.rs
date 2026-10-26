//! Compact application status with exact engineering context.

use egui::{Align, Frame, Layout, Panel, Sense, Ui, Vec2};

use crate::simulation::status::EngineAvailability;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::app_state::diagnostic_store::listing::DiagnosticCounts;
use crate::workbench::framework::host::{
    Host, StatusCell, StatusControl, StatusKey, StatusMark, StatusText, StatusZone,
};
use crate::workbench::framework::workbench_host::WorkbenchHost;

use super::super::layout::LayoutSpec;

const STATUS_PADDING_X: f32 = 9.0;
const STATUS_MARK_GAP: f32 = 5.0;
pub const STATUS_FONT_SIZE: f32 = tokens::FS_1;

pub fn show(root: &mut Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let visibility = status_visibility(ctx.content_rect().width());
    let shown = Panel::bottom("workbench.status_bar")
        .exact_size(layout.status_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(root, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let bar_width = ui.available_width();
            let height = ui.available_height();
            let frame = app.status_frame(ctx);
            let workspace = app.workbench().workspace;
            let (leading, trailing) = status_zones(app, &frame, workspace, visibility);
            let mut leading_desired = leading.each_ref().map(|cell| status_cell_width(ui, cell));
            let trailing_desired = trailing.each_ref().map(|cell| status_cell_width(ui, cell));
            let others_desired = leading
                .iter()
                .zip(leading_desired)
                .filter(|(cell, _)| !yields_room(cell.as_ref()))
                .map(|(_, width)| width)
                .sum::<f32>();
            let right_desired = trailing_desired.iter().sum::<f32>();
            // The prompt takes what the other items leave and never squeezes
            // one of them: it is the answer to a gesture, they are the state.
            let room = (bar_width - others_desired - right_desired).max(0.0);
            let mut prompt_width = 0.0;
            for (cell, width) in leading.iter().zip(&mut leading_desired) {
                if yields_room(cell.as_ref()) {
                    *width = width.min(room);
                    prompt_width += *width;
                }
            }
            let left_desired = prompt_width + others_desired;
            let groups = status_group_allocation(left_desired, right_desired, bar_width);
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(groups.left, height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        let widths = fit_status_widths(leading_desired, groups.left);
                        // Rules go between items, so the last item on the left
                        // draws none: a rule there would face empty bar.
                        let last = leading.iter().rposition(Option::is_some).unwrap_or(0);
                        for (order, cell) in leading.into_iter().enumerate() {
                            if let Some(cell) = cell {
                                paint_status_cell(
                                    ui,
                                    app,
                                    &frame,
                                    cell,
                                    order != last,
                                    widths[order],
                                );
                            }
                        }
                    },
                );

                ui.add_space(groups.gap);

                ui.allocate_ui_with_layout(
                    Vec2::new(groups.right, height),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        let widths = fit_status_widths(trailing_desired, groups.right);
                        // The right group lays out from the window edge, so
                        // each item's rule falls between it and the one to its
                        // right; the rightmost would draw on the edge itself.
                        let rightmost = trailing.iter().position(Option::is_some).unwrap_or(0);
                        for (order, cell) in trailing.into_iter().enumerate() {
                            if let Some(cell) = cell {
                                paint_status_cell(
                                    ui,
                                    app,
                                    &frame,
                                    cell,
                                    order != rightmost,
                                    widths[order],
                                );
                            }
                        }
                    },
                );
            });
        });
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::ContentInfo);
        node.set_label("Application status");
    });
}

/// The most cells one end of the bar holds.
const ZONE_CELLS: usize = 8;

/// A cell with something to show this frame, and what it shows.
struct ShownCell<'a, H: Host> {
    cell: &'static StatusCell<H>,
    text: StatusText<'a>,
}

/// One end of the bar, by place.
type Zone<'a, H> = [Option<ShownCell<'a, H>>; ZONE_CELLS];

/// Offer each place to the cells the host lists for it, in their order: the
/// first that reports in `workspace`, fits the bar, and has something to say
/// takes it.
fn status_zones<'a, H: Host>(
    app: &mut H,
    frame: &'a H::StatusFrame,
    workspace: H::WorkspaceKey,
    visibility: StatusVisibility,
) -> (Zone<'a, H>, Zone<'a, H>) {
    let mut leading: Zone<'a, H> = Default::default();
    let mut trailing: Zone<'a, H> = Default::default();
    for cell in app.status_cells() {
        if !cell.workspace.is_none_or(|only| only == workspace) || !visibility.shows(cell.key) {
            continue;
        }
        let zone = match cell.zone {
            StatusZone::Leading => &mut leading,
            StatusZone::Trailing => &mut trailing,
        };
        if zone[cell.order].is_none() {
            zone[cell.order] = (cell.content)(app, frame).map(|text| ShownCell { cell, text });
        }
    }
    (leading, trailing)
}

/// The width a place asks for: none when nothing shows there.
fn status_cell_width<H: Host>(ui: &egui::Ui, cell: &Option<ShownCell<'_, H>>) -> f32 {
    cell.as_ref().map_or(0.0, |shown| {
        desired_status_width(ui, &shown.text.text, shown.text.mark)
    })
}

/// Whether the place holds the prompt, which takes only the room the other
/// cells leave.
fn yields_room<H: Host>(cell: Option<&ShownCell<'_, H>>) -> bool {
    cell.is_some_and(|shown| shown.cell.key == StatusKey::Prompt)
}

/// Paint `shown` at `width`, and do what pressing it does.
fn paint_status_cell<H: Host>(
    ui: &mut egui::Ui,
    app: &mut H,
    frame: &H::StatusFrame,
    shown: ShownCell<'_, H>,
    rule_after: bool,
    width: f32,
) {
    let ShownCell {
        cell,
        text:
            StatusText {
                text,
                mark,
                control,
                hover,
            },
    } = shown;
    let response = match control {
        StatusControl::Label => status_item_sized(ui, &text, mark, false, rule_after, width),
        StatusControl::Button => status_item_sized(ui, &text, mark, true, rule_after, width),
        StatusControl::Guarded { enabled } => {
            ui.scope(|ui| {
                if !enabled {
                    ui.disable();
                }
                status_item_sized(ui, &text, mark, true, rule_after, width)
            })
            .inner
        }
    };
    if cell.key == StatusKey::Prompt {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(&*text);
            node.set_live(egui::accesskit::Live::Polite);
        });
    }
    let response = match hover {
        Some(hover) if control == (StatusControl::Guarded { enabled: false }) => {
            response.on_disabled_hover_text(hover)
        }
        Some(hover) => response.on_hover_text(hover),
        None => response,
    };
    if response.clicked()
        && let Some(activate) = cell.activate
    {
        activate(app, frame);
    }
}

/// The prompt the bar shows this frame. One that has expired is dropped here;
/// one that has not asks for the frame that will drop it, because an idle
/// window paints no frame of its own.
pub fn current_prompt(
    ctx: &egui::Context,
    slot: &mut Option<crate::workbench::state::StatusPrompt>,
) -> Option<String> {
    let prompt = slot.as_ref()?;
    match prompt.remaining() {
        Some(remaining) => {
            ctx.request_repaint_after(remaining);
            Some(prompt.text.clone())
        }
        None => {
            *slot = None;
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusVisibility {
    pub coordinates: bool,
    pub selection: bool,
    pub platform: bool,
}

impl StatusVisibility {
    /// Whether the bar has room for what `key` reports: the dense metadata
    /// gives way on a narrow bar.
    fn shows(self, key: StatusKey) -> bool {
        match key {
            StatusKey::Context => self.coordinates,
            StatusKey::Selection => self.selection,
            StatusKey::Platform => self.platform,
            StatusKey::Prompt
            | StatusKey::Checks
            | StatusKey::Page
            | StatusKey::Zoom
            | StatusKey::Engine => true,
        }
    }
}

pub fn status_visibility(viewport_width: f32) -> StatusVisibility {
    let show_secondary = viewport_width > 900.0;
    StatusVisibility {
        coordinates: show_secondary,
        selection: show_secondary,
        platform: show_secondary,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatusGroupAllocation {
    pub left: f32,
    pub right: f32,
    pub gap: f32,
}

/// Reproduce the mockup's `justify-content: space-between` status composition.
/// Each group keeps its intrinsic content width while space is available. Only
/// genuine overflow compresses both groups, in proportion to their content.
pub fn status_group_allocation(
    left_desired: f32,
    right_desired: f32,
    available_width: f32,
) -> StatusGroupAllocation {
    let available_width = available_width.max(0.0);
    let left_desired = left_desired.max(0.0);
    let right_desired = right_desired.max(0.0);
    let desired = left_desired + right_desired;
    if desired <= available_width {
        return StatusGroupAllocation {
            left: left_desired,
            right: right_desired,
            gap: available_width - desired,
        };
    }
    if desired <= f32::EPSILON {
        return StatusGroupAllocation {
            left: 0.0,
            right: 0.0,
            gap: available_width,
        };
    }
    let scale = available_width / desired;
    let left = left_desired * scale;
    StatusGroupAllocation {
        left,
        right: available_width - left,
        gap: 0.0,
    }
}

/// What the checks segment states, from the same list the Problems page
/// lists and the tab badge counts, or nothing.
///
/// With nothing open, any retained result describes the bootstrap placeholder
/// rather than the reader's work, and the coordinate segment already says no
/// project is loaded. Checks that have never run are not a finding either.
///
/// Three surfaces used to derive this independently — the status bar from
/// `dialogs.drc_results` or the netlist summary depending on which workspace
/// was open, the badge from the page, the page from four producers — and they
/// disagreed routinely. There is one list now, so there is one answer, and
/// the bar reads it once per pass for both the text and the mark.
pub fn check_summary(counts: DiagnosticCounts) -> Option<String> {
    if counts == DiagnosticCounts::default() {
        return None;
    }
    let mut label = format!(
        "Problems · {} error{} · {} advisor{}",
        counts.errors,
        if counts.errors == 1 { "" } else { "s" },
        counts.advisories,
        if counts.advisories == 1 { "y" } else { "ies" }
    );
    use std::fmt::Write as _;
    if counts.stale > 0 {
        let _ = write!(label, " · {} stale", counts.stale);
    }
    // Findings whose inputs cannot be read are not stale — nothing is known
    // to have moved — and saying so would send the reader to re-run checks
    // that cannot run.
    if counts.unavailable > 0 {
        let _ = write!(label, " · {} unavailable", counts.unavailable);
    }
    Some(label)
}

/// The mark in front of the checks segment: a failure while anything errors,
/// a warning while anything warns, is stale or cannot be evaluated, and
/// nothing otherwise, the way a compiler's problem count reads. A note or a
/// hint is counted among the advisories but raises no mark.
pub fn check_problem(counts: DiagnosticCounts) -> Option<Problem> {
    if counts.errors > 0 {
        Some(Problem::Failure)
    } else if counts.warnings > 0 || counts.stale > 0 || counts.unavailable > 0 {
        Some(Problem::Warning)
    } else {
        None
    }
}

/// The mark in front of the engine segment: a failure when the engine cannot
/// run, the one live dot while a run is in progress, and nothing otherwise.
pub fn engine_status_mark(availability: &EngineAvailability, run_in_progress: bool) -> StatusMark {
    if availability.failure_reason().is_some() {
        StatusMark::Problem(Problem::Failure)
    } else if run_in_progress {
        StatusMark::Live
    } else {
        StatusMark::None
    }
}

fn status_padding_x(ui: &egui::Ui) -> f32 {
    if ui.ctx().content_rect().width() <= 820.0 {
        7.0
    } else {
        STATUS_PADDING_X
    }
}

fn status_mark_size(mark: StatusMark) -> f32 {
    match mark {
        StatusMark::None => 0.0,
        // The kit's own, so the bar's refusal is the mark the panel above it
        // sets. Only the gap differs: `STATUS_MARK_GAP` is tighter than the
        // kit's, because the bar is one 22 pt strip rather than a row of body
        // text.
        StatusMark::Problem(_) => crate::ui::widgets::PROBLEM_MARK_SIDE,
        StatusMark::Live => 6.0,
        StatusMark::Processor => 16.0,
    }
}

pub fn status_text_font() -> egui::FontId {
    theme::mono(STATUS_FONT_SIZE, FontWeight::Regular)
}

fn desired_status_width(ui: &egui::Ui, text: &str, mark: StatusMark) -> f32 {
    let t = Tokens::get(ui.ctx());
    let text_width = ui
        .painter()
        .layout_no_wrap(text.to_owned(), status_text_font(), t.color.text_dim)
        .size()
        .x;
    let mark_width = if matches!(mark, StatusMark::None) {
        0.0
    } else {
        status_mark_size(mark) + STATUS_MARK_GAP
    };
    status_padding_x(ui) * 2.0 + mark_width + text_width
}

fn fit_status_widths<const N: usize>(desired: [f32; N], available: f32) -> [f32; N] {
    let total = desired.iter().sum::<f32>();
    if total <= available || total <= f32::EPSILON {
        return desired;
    }
    let scale = (available / total).clamp(0.0, 1.0);
    desired.map(|width| width * scale)
}

fn status_item_sized(
    ui: &mut egui::Ui,
    text: &str,
    mark: StatusMark,
    interactive: bool,
    rule_after: bool,
    width: f32,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = status_text_font();
    let padding_x = status_padding_x(ui);
    let mark_size = status_mark_size(mark);
    let mark_width = if matches!(mark, StatusMark::None) {
        0.0
    } else {
        mark_size + STATUS_MARK_GAP
    };
    let width = width.min(ui.available_width().max(0.0));
    let text_width = (width - padding_x * 2.0 - mark_width).max(0.0);
    let painted_text = ellipsize_status_text(ui.painter(), text, &font, text_width);
    let galley = ui
        .painter()
        .layout_no_wrap(painted_text, font, t.color.text_dim);
    let sense = if interactive {
        Sense::click()
    } else {
        Sense::hover().difference(Sense::focusable_noninteractive())
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, ui.available_height()), sense);
    // A screen reader cannot see the mark, so it hears the severity first.
    let announced = match mark {
        StatusMark::Problem(problem) => problem.announce(text),
        StatusMark::None | StatusMark::Live | StatusMark::Processor => text.to_owned(),
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if interactive {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            &announced,
        )
    });

    if interactive && response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if rule_after {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }

    let mut text_x = rect.left() + padding_x;
    if !matches!(mark, StatusMark::None) {
        let mark_rect = egui::Rect::from_center_size(
            egui::pos2(text_x + mark_size * 0.5, rect.center().y),
            Vec2::splat(mark_size),
        );
        match mark {
            StatusMark::Problem(problem) => {
                paint_problem_mark(ui.painter(), mark_rect, problem);
            }
            StatusMark::Live => {
                ui.painter()
                    .circle_filled(mark_rect.center(), 3.0, t.color.accent);
            }
            StatusMark::Processor => paint_processor(ui.painter(), mark_rect, t.color.text_dim),
            StatusMark::None => {}
        }
        text_x += mark_width;
    }
    ui.painter().with_clip_rect(rect).galley(
        egui::pos2(text_x, rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text_dim,
    );
    if interactive {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
}

fn ellipsize_status_text(
    painter: &egui::Painter,
    value: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if maximum_width <= 0.0 {
        return String::new();
    }
    if painter
        .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return value.to_owned();
    }
    let ellipsis = "\u{2026}";
    if painter
        .layout_no_wrap(ellipsis.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        > maximum_width
    {
        return String::new();
    }
    let characters = value.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once('\u{2026}'))
            .collect::<String>();
        if painter
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('\u{2026}'))
        .collect()
}

fn paint_processor(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let body = rect.shrink(3.0);
    painter.rect_stroke(
        body,
        1.0,
        egui::Stroke::new(1.0, color),
        egui::StrokeKind::Inside,
    );
    painter.rect_stroke(
        body.shrink(2.0),
        0.0,
        egui::Stroke::new(1.0, color),
        egui::StrokeKind::Inside,
    );
    for offset in [4.0, 7.0, 10.0] {
        painter.hline(
            egui::Rangef::new(rect.left(), body.left()),
            rect.top() + offset,
            egui::Stroke::new(1.0, color),
        );
        painter.hline(
            egui::Rangef::new(body.right(), rect.right()),
            rect.top() + offset,
            egui::Stroke::new(1.0, color),
        );
    }
}
