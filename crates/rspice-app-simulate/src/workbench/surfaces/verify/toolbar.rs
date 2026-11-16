//! The one toolbar row every Verify page opens with, and its empty state.
//!
//! The row holds four things and no others: the page's title; which run the
//! page reads, as a selector, only when there is more than one run it could
//! read; one plain summary line in neutral text, with a problem's mark in front
//! only when something fails; and the page's actions, on the right. There is no
//! subtitle explaining the page and no tile of big numbers: a page that has
//! nothing to show says what produces data, on one line, with the action that
//! produces it.
//!
//! Nothing on the row overlaps. The summary takes the room between the title
//! and the actions and elides there, its whole text on hover. Actions that
//! do not fit beside it fold, from the last, behind one More button that
//! lists them in order; only when not even that fits does the summary drop
//! to a line of its own, and the title elides last of all.

use egui::{Rect, Sense, Ui, pos2, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Problem, paint_problem_mark, select};
use crate::workbench::design_system::elide_text;

use super::table::TABLE_EDGE_INSET;

#[cfg(test)]
mod tests;

/// Height of the toolbar row: a control and its padding above and below.
pub const TOOLBAR_H: f32 = 36.0;
/// Height of the line a summary drops to when the row cannot hold it.
const SUMMARY_LINE_H: f32 = 22.0;
/// Space between the title, the run selector, the summary and the actions.
const GROUP_GAP: f32 = 16.0;
/// Space between two actions.
const ACTION_GAP: f32 = 6.0;
/// Width of the run selector.
const RUN_SELECT_W: f32 = 190.0;
/// Least width a summary keeps on the toolbar's own line.
const SUMMARY_MIN_W: f32 = 140.0;
/// The button the actions that do not fit fold behind.
const MORE_LABEL: &str = "More";
const MORE_ACCESSIBLE_LABEL: &str = "More actions";
/// Width of the folded actions' menu, and one row's height with a fine
/// pointer.
const OVERFLOW_MENU_W: f32 = 220.0;
const OVERFLOW_ROW_H: f32 = 28.0;
/// A folded action's check mark, and the room it keeps before the words.
const CHECK_SIDE: f32 = 14.0;
const CHECK_ROOM: f32 = 28.0;
/// A problem mark before the summary, and the advance from it to the words:
/// the kit's own, so the summary matches every other marked line.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};
/// Height of a page's empty-state line.
pub(super) const EMPTY_LINE_H: f32 = 40.0;

/// The toolbar's one summary line.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub(super) text: String,
    pub(super) problem: Option<Problem>,
}

impl Summary {
    pub(super) fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            problem: None,
        }
    }

    /// A summary that reports something failing, behind the failure mark.
    pub fn failing(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            problem: Some(Problem::Failure),
        }
    }

    /// A summary the reader must look at before relying on the page.
    pub(super) fn warning(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            problem: Some(Problem::Warning),
        }
    }

    /// Room the problem's mark takes before the words.
    fn mark_room(&self) -> f32 {
        if self.problem.is_some() {
            MARK_ADVANCE
        } else {
            0.0
        }
    }
}

/// One action on the toolbar's right.
#[derive(Debug, Clone, Copy)]
pub(super) struct ToolbarAction<'a> {
    label: &'a str,
    enabled: bool,
    toggled: bool,
    disabled_reason: Option<&'a str>,
}

impl<'a> ToolbarAction<'a> {
    pub(super) fn new(label: &'a str) -> Self {
        Self {
            label,
            enabled: true,
            toggled: false,
            disabled_reason: None,
        }
    }

    pub(super) fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// A latched control: painted in the accent while on, and checked in
    /// the More menu.
    pub(super) fn toggled(mut self, toggled: bool) -> Self {
        self.toggled = toggled;
        self
    }

    /// Why a disabled action cannot be taken, shown on hover.
    pub(super) fn disabled_reason(mut self, reason: &'a str) -> Self {
        self.disabled_reason = Some(reason);
        self
    }

    fn button(&self) -> Button<'a> {
        let button = Button::new(self.label).enabled(self.enabled);
        if self.toggled {
            button.accent()
        } else {
            button
        }
    }
}

/// What a toolbar's run selector chooses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) enum RunRole {
    /// The run the page reads, offered only when there is a choice to make.
    #[default]
    Read,
}

impl RunRole {
    /// The selector's name, and what it says with nothing chosen.
    const fn words(self) -> (&'static str, &'static str) {
        match self {
            Self::Read => ("Run", "No run selected"),
        }
    }
}

/// The runs a page could read, and the one it reads.
#[derive(Debug, Clone, Default)]
pub(super) struct RunChoice {
    /// What the select picks, as assistive technology announces it.
    pub(super) name: &'static str,
    /// Each candidate's index in the retained run list, and its label.
    pub(super) candidates: Vec<(usize, String)>,
    pub(super) selected: Option<usize>,
    pub(super) role: RunRole,
}

impl RunChoice {
    /// Whether the toolbar shows a selector at all: for the run a page
    /// reads, only when there is a choice to make; for its reference,
    /// whenever there is a run to choose.
    pub(super) fn offers_choice(&self) -> bool {
        match self.role {
            RunRole::Read => self.candidates.len() > 1,
        }
    }
}

/// What the reader did on the toolbar this frame.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct ToolbarOutput {
    /// Index of the clicked action, on the row or in the More menu.
    pub(super) action: Option<usize>,
    /// Retained-run index the reader picked.
    pub(super) run: Option<usize>,
}

/// One page's toolbar.
pub(super) struct PageToolbar<'a> {
    pub(super) title: &'a str,
    pub(super) runs: &'a RunChoice,
    pub(super) summary: Option<Summary>,
    pub(super) actions: &'a [ToolbarAction<'a>],
}

/// What each part of the row would take at its own width.
#[derive(Debug, Clone)]
struct Wants<'w> {
    /// The whole row, insets included.
    row: f32,
    title: f32,
    selector: bool,
    /// The summary's words and mark, when there is a summary.
    summary: Option<f32>,
    actions: &'w [f32],
    /// The More button.
    more: f32,
}

/// How the row is shared out.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Plan {
    /// Width the title is given: its own, unless nothing else fits.
    title: f32,
    /// How many leading actions stand on the row; the rest fold behind More.
    shown: usize,
    /// Width of the actions and More, with the gap before them.
    actions: f32,
    /// Whether the summary stays on the row rather than its own line.
    summary_inline: bool,
}

/// Share the row out: every action and the summary when they fit; else
/// the trailing actions fold behind More; else the summary takes a line of
/// its own and the actions refit beside the title; else the title elides.
fn plan(wants: &Wants<'_>) -> Plan {
    let room = (wants.row - 2.0 * TABLE_EDGE_INSET).max(0.0);
    let selector = if wants.selector {
        GROUP_GAP + RUN_SELECT_W
    } else {
        0.0
    };
    let count = wants.actions.len();
    let block = |shown: usize| {
        let standing: f32 = wants.actions[..shown].iter().sum();
        let (items, more) = if shown == count {
            (count, 0.0)
        } else {
            (shown + 1, wants.more)
        };
        if items == 0 {
            0.0
        } else {
            GROUP_GAP + standing + more + ACTION_GAP * (items - 1) as f32
        }
    };
    let summary = wants
        .summary
        .map_or(0.0, |words| GROUP_GAP + words.min(SUMMARY_MIN_W));
    let lead = wants.title + selector;
    for (summary_room, summary_inline) in [(summary, true), (0.0, wants.summary.is_none())] {
        if let Some(shown) = (0..=count)
            .rev()
            .find(|shown| lead + summary_room + block(*shown) <= room)
        {
            return Plan {
                title: wants.title,
                shown,
                actions: block(shown),
                summary_inline,
            };
        }
    }
    Plan {
        title: (room - selector - block(0)).clamp(0.0, wants.title),
        shown: 0,
        actions: block(0),
        summary_inline: wants.summary.is_none(),
    }
}

impl PageToolbar<'_> {
    pub(super) fn show(self, ui: &mut Ui) -> ToolbarOutput {
        let t = Tokens::get(ui.ctx());
        let mut output = ToolbarOutput::default();
        let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let width = visible.width().max(1.0);
        let title_font = theme::sans(tokens::FS_2, FontWeight::SemiBold);
        let title_w = text_width(ui, self.title, &title_font);
        let action_widths = self
            .actions
            .iter()
            .map(|action| action.button().measured_width(ui))
            .collect::<Vec<_>>();
        let summary_font = theme::sans(tokens::FS_1, FontWeight::Regular);
        let plan = plan(&Wants {
            row: width,
            title: title_w,
            selector: self.runs.offers_choice(),
            summary: self
                .summary
                .as_ref()
                .map(|summary| summary.mark_room() + text_width(ui, &summary.text, &summary_font)),
            actions: &action_widths,
            more: more_button().measured_width(ui),
        });
        let wrapped = self.summary.is_some() && !plan.summary_inline;
        let height = TOOLBAR_H + if wrapped { SUMMARY_LINE_H } else { 0.0 };
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let row = Rect::from_min_size(rect.min, vec2(width, TOOLBAR_H));
        #[cfg(any(test, feature = "test-support"))]
        super::raster::probe(ui, "toolbar", rect);
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );

        let title = ui.painter().layout_no_wrap(
            elide_text(ui, self.title, &title_font, plan.title),
            title_font,
            t.color.text,
        );
        let title_rect = Rect::from_min_size(
            pos2(
                row.left() + TABLE_EDGE_INSET,
                row.center().y - title.size().y * 0.5,
            ),
            title.size(),
        );
        #[cfg(any(test, feature = "test-support"))]
        super::raster::probe(ui, "toolbar.title", title_rect);
        let title_response = ui.interact(
            title_rect,
            ui.id().with("verify.toolbar.title"),
            Sense::hover(),
        );
        title_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), self.title)
        });
        ui.ctx().accesskit_node_builder(title_response.id, |node| {
            node.set_role(egui::accesskit::Role::Heading);
        });
        if plan.title < title_w {
            let _ = title_response.on_hover_text(self.title);
        }
        ui.painter().galley(title_rect.min, title, t.color.text);

        let control_h = t.metrics.ctl_h;
        let mut lead_right = row.left() + TABLE_EDGE_INSET + plan.title;
        if self.runs.offers_choice() {
            let select_rect = Rect::from_min_size(
                pos2(lead_right + GROUP_GAP, row.center().y - control_h * 0.5),
                vec2(RUN_SELECT_W, control_h),
            );
            lead_right = select_rect.right();
            #[cfg(any(test, feature = "test-support"))]
            super::raster::probe(ui, "toolbar.run", select_rect);
            let (name, nothing_chosen) = self.runs.role.words();
            let options = self
                .runs
                .candidates
                .iter()
                .map(|(_, label)| label.clone())
                .collect::<Vec<_>>();
            let shown = self
                .runs
                .candidates
                .iter()
                .find(|(index, _)| Some(*index) == self.runs.selected)
                .map_or(nothing_chosen, |(_, label)| label.as_str());
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(select_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            if let Some(picked) = select(
                &mut child,
                "verify.toolbar.run",
                name,
                shown,
                &options,
                RUN_SELECT_W,
            ) {
                output.run = self.runs.candidates.get(picked).map(|(index, _)| *index);
            }
        }

        // The actions, right to left: More last on the row, then the ones
        // that stand.
        let mut x = row.right() - TABLE_EDGE_INSET;
        let button_rect = |x: f32, width: f32| {
            Rect::from_min_size(
                pos2(x - width, row.center().y - control_h * 0.5),
                vec2(width, control_h),
            )
        };
        if plan.shown < self.actions.len() {
            let more = more_button();
            let place = button_rect(x, more.measured_width(ui));
            x = place.left() - ACTION_GAP;
            if let Some(index) = self.overflow(ui, place, more, plan.shown) {
                output.action = Some(index);
            }
        }
        for (index, (action, action_width)) in self
            .actions
            .iter()
            .zip(&action_widths)
            .enumerate()
            .take(plan.shown)
            .rev()
        {
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(button_rect(x, *action_width))
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            let response = action.button().show(&mut child);
            #[cfg(any(test, feature = "test-support"))]
            super::raster::probe(ui, "toolbar.action", response.rect);
            let response = match action.disabled_reason {
                Some(reason) => response.on_disabled_hover_text(reason),
                None => response,
            };
            if response.clicked() {
                output.action = Some(index);
            }
            x -= action_width + ACTION_GAP;
        }

        if let Some(summary) = &self.summary {
            let line = if plan.summary_inline {
                Rect::from_x_y_ranges(
                    lead_right + GROUP_GAP..=row.right() - TABLE_EDGE_INSET - plan.actions,
                    row.y_range(),
                )
            } else {
                Rect::from_x_y_ranges(
                    (rect.left() + TABLE_EDGE_INSET)..=(rect.right() - TABLE_EDGE_INSET),
                    row.bottom()..=rect.bottom(),
                )
            };
            paint_summary(ui, line, summary);
        }
        output
    }

    /// The More button at `place` and the menu it opens, listing the actions
    /// from `first` on in order. Returns the index of the action taken.
    fn overflow(&self, ui: &mut Ui, place: Rect, more: Button<'_>, first: usize) -> Option<usize> {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(place)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        let response = more.show(&mut child);
        #[cfg(any(test, feature = "test-support"))]
        super::raster::probe(ui, "toolbar.more", response.rect);
        let metrics = Tokens::get(ui.ctx()).metrics;
        let height = if metrics.is_touch() {
            metrics.touch_target()
        } else {
            OVERFLOW_ROW_H
        };
        let mut taken = None;
        egui::Popup::menu(&response)
            .width(OVERFLOW_MENU_W)
            .show(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                for (index, action) in self.actions.iter().enumerate().skip(first) {
                    let row = overflow_row(ui, action, height);
                    #[cfg(any(test, feature = "test-support"))]
                    super::raster::probe(ui, &format!("toolbar.overflow.{index}"), row.rect);
                    if action.enabled && row.clicked() {
                        taken = Some(index);
                        ui.close();
                    }
                }
            });
        taken
    }
}

/// A menu of `actions` at `position`, drawn as the More menu is: a table
/// row's context menu. `open` opens it; it stays open, in memory under `id`,
/// until an action is taken or the reader clicks elsewhere. Returns the
/// index of the action taken.
pub(super) fn action_menu(
    ctx: &egui::Context,
    id: egui::Id,
    position: egui::Pos2,
    open: bool,
    actions: &[ToolbarAction<'_>],
) -> Option<usize> {
    let metrics = Tokens::get(ctx).metrics;
    let height = if metrics.is_touch() {
        metrics.touch_target()
    } else {
        OVERFLOW_ROW_H
    };
    let mut taken = None;
    egui::Popup::new(
        id,
        ctx.clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .open_memory(open.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .width(OVERFLOW_MENU_W)
    .show(|ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        for (index, action) in actions.iter().enumerate() {
            let row = overflow_row(ui, action, height);
            #[cfg(any(test, feature = "test-support"))]
            super::raster::probe(ui, &format!("menu.{index}"), row.rect);
            if action.enabled && row.clicked() {
                taken = Some(index);
                ui.close();
            }
        }
    });
    taken
}

fn more_button() -> Button<'static> {
    Button::new(MORE_LABEL)
        .menu()
        .accessible_label(MORE_ACCESSIBLE_LABEL)
}

/// The width `text` takes in `font` on one line.
fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
        .size()
        .x
}

/// One folded action in the More menu: a check when it is latched on, its
/// label, faint and with its reason on hover when it cannot be taken.
fn overflow_row(ui: &mut Ui, action: &ToolbarAction<'_>, height: f32) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), height),
        if action.enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            action.enabled,
            action.toggled,
            action.label,
        )
    });
    if action.enabled && (response.hovered() || response.is_pointer_button_down_on()) {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let ink = if action.enabled {
        t.color.text
    } else {
        t.color.text_faint
    };
    if action.toggled {
        let check = Rect::from_center_size(
            pos2(rect.left() + CHECK_ROOM * 0.5, rect.center().y),
            egui::Vec2::splat(CHECK_SIDE),
        );
        #[cfg(any(test, feature = "test-support"))]
        super::raster::probe(ui, "toolbar.overflow.check", check);
        Icon::Check.paint(ui.painter(), check, ink);
    }
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let words = elide_text(
        ui,
        action.label,
        &font,
        (rect.width() - CHECK_ROOM - TABLE_EDGE_INSET).max(0.0),
    );
    ui.painter().text(
        pos2(rect.left() + CHECK_ROOM, rect.center().y),
        egui::Align2::LEFT_CENTER,
        words,
        font,
        ink,
    );
    theme::paint_focus_ring(ui, &response, rect);
    match action.disabled_reason {
        Some(reason) => response.on_disabled_hover_text(reason),
        None => response,
    }
}

/// Paint the summary on its line: the problem's mark, if any, then the
/// words, elided to the line with the whole text on hover.
fn paint_summary(ui: &mut Ui, line: Rect, summary: &Summary) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let mark_room = summary.mark_room();
    let room = (line.width() - mark_room).max(0.0);
    let elided = text_width(ui, &summary.text, &font) > room;
    let words = elide_text(ui, &summary.text, &font, room);
    let galley = ui.painter().layout_no_wrap(words, font, t.color.text_dim);
    let text_rect = Rect::from_min_size(
        pos2(
            line.left() + mark_room,
            line.center().y - galley.size().y * 0.5,
        ),
        galley.size(),
    );
    let response = ui.interact(
        text_rect,
        ui.id().with("verify.toolbar.summary"),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            summary.problem.map_or_else(
                || summary.text.clone(),
                |problem| problem.announce(&summary.text),
            ),
        )
    });
    if elided {
        let _ = response.on_hover_text(&summary.text);
    }
    #[cfg(any(test, feature = "test-support"))]
    super::raster::probe(ui, "toolbar.summary", text_rect);
    if let Some(problem) = summary.problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(line.left() + MARK_SIDE * 0.5, line.center().y),
                egui::Vec2::splat(MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().galley(text_rect.min, galley, t.color.text_dim);
}

/// A page's empty state: one neutral line saying what produces data, and the
/// action that produces it when the toolbar does not already carry it.
/// Returns whether the action was clicked.
pub(super) fn empty_line(ui: &mut Ui, text: &str, action: Option<&str>) -> bool {
    let t = Tokens::get(ui.ctx());
    let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
    let width = visible.width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width, EMPTY_LINE_H), Sense::hover());
    #[cfg(any(test, feature = "test-support"))]
    super::raster::probe(ui, "empty", rect);
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let button_width = action.map_or(0.0, |label| Button::new(label).measured_width(ui) + 12.0);
    let words = elide_text(
        ui,
        text,
        &font,
        (width - 2.0 * TABLE_EDGE_INSET - button_width).max(0.0),
    );
    let galley = ui.painter().layout_no_wrap(words, font, t.color.text_dim);
    let text_rect = Rect::from_min_size(
        pos2(
            rect.left() + TABLE_EDGE_INSET,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley.size(),
    );
    #[cfg(any(test, feature = "test-support"))]
    super::raster::probe(ui, "empty.text", text_rect);
    let response = ui.interact(text_rect, ui.id().with("verify.empty"), Sense::hover());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    let text_right = text_rect.right();
    ui.painter().galley(text_rect.min, galley, t.color.text_dim);
    let Some(label) = action else {
        return false;
    };
    let control_h = t.metrics.ctl_h;
    let button = Button::new(label);
    let button_rect = Rect::from_min_size(
        pos2(text_right + 12.0, rect.center().y - control_h * 0.5),
        vec2(button.measured_width(ui), control_h),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    button.show(&mut child).clicked()
}
