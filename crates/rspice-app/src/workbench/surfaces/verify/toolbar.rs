//! The one toolbar row every Verify page opens with, and its empty state.
//!
//! The row holds four things and no others: the page's title; which run the
//! page reads, as a selector, only when there is more than one run it could
//! read; one plain summary line in neutral text, with a problem's mark in front
//! only when something fails; and the page's actions, on the right. There is no
//! subtitle explaining the page and no tile of big numbers: a page that has
//! nothing to show says what produces data, on one line, with the action that
//! produces it.

use egui::{Rect, Sense, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Problem, paint_problem_mark, select};

use super::table::{TABLE_EDGE_INSET, elide};

/// Height of the toolbar row: a control and its padding above and below.
pub(super) const TOOLBAR_H: f32 = 36.0;
/// Height of the line a summary drops to when the row cannot hold it.
const SUMMARY_LINE_H: f32 = 22.0;
/// Space between the title, the run selector and the summary.
const GROUP_GAP: f32 = 16.0;
/// Space between two actions.
const ACTION_GAP: f32 = 6.0;
/// Width of the run selector.
const RUN_SELECT_W: f32 = 190.0;
/// Least width a summary keeps on the toolbar's own line.
const SUMMARY_MIN_W: f32 = 140.0;
/// A problem mark before the summary, and the advance from it to the words:
/// the kit's own, so the summary matches every other marked line.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};
/// Height of a page's empty-state line.
pub(super) const EMPTY_LINE_H: f32 = 40.0;

/// The toolbar's one summary line.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Summary {
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
    pub(super) fn failing(text: impl Into<String>) -> Self {
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

    /// A latched control: painted in the accent while on.
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

/// The runs a page could read, and the one it reads.
#[derive(Debug, Clone, Default)]
pub(super) struct RunChoice {
    /// Each candidate's index in the retained run list, and its label.
    pub(super) candidates: Vec<(usize, String)>,
    pub(super) selected: Option<usize>,
}

impl RunChoice {
    /// Whether the toolbar shows a selector at all: only when there is a
    /// choice to make.
    pub(super) fn offers_choice(&self) -> bool {
        self.candidates.len() > 1
    }
}

/// What the reader did on the toolbar this frame.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct ToolbarOutput {
    /// Index of the clicked action.
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

impl PageToolbar<'_> {
    pub(super) fn show(self, ui: &mut Ui) -> ToolbarOutput {
        let t = Tokens::get(ui.ctx());
        let mut output = ToolbarOutput::default();
        let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let width = visible.width().max(1.0);
        let title_font = theme::sans(tokens::FS_2, FontWeight::SemiBold);
        let title = ui
            .painter()
            .layout_no_wrap(self.title.to_owned(), title_font, t.color.text);
        let action_widths = self
            .actions
            .iter()
            .map(|action| action.button().measured_width(ui))
            .collect::<Vec<_>>();
        let actions_width = action_widths.iter().sum::<f32>()
            + ACTION_GAP * action_widths.len().saturating_sub(1) as f32;
        let selector_width = if self.runs.offers_choice() {
            RUN_SELECT_W + GROUP_GAP
        } else {
            0.0
        };
        let summary_left = TABLE_EDGE_INSET + title.size().x + GROUP_GAP + selector_width;
        let summary_right = width
            - TABLE_EDGE_INSET
            - if actions_width > 0.0 {
                actions_width + GROUP_GAP
            } else {
                0.0
            };
        let summary_inline = summary_right - summary_left >= SUMMARY_MIN_W;
        let wrapped = self.summary.is_some() && !summary_inline;
        let height = TOOLBAR_H + if wrapped { SUMMARY_LINE_H } else { 0.0 };
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let row = Rect::from_min_size(rect.min, vec2(width, TOOLBAR_H));
        #[cfg(test)]
        super::raster::probe(ui, "toolbar", rect);
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );

        let title_rect = Rect::from_min_size(
            pos2(
                row.left() + TABLE_EDGE_INSET,
                row.center().y - title.size().y * 0.5,
            ),
            title.size(),
        );
        #[cfg(test)]
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
        ui.painter().galley(title_rect.min, title, t.color.text);

        let control_h = t.metrics.ctl_h;
        if self.runs.offers_choice() {
            let select_rect = Rect::from_min_size(
                pos2(
                    title_rect.right() + GROUP_GAP,
                    row.center().y - control_h * 0.5,
                ),
                vec2(RUN_SELECT_W, control_h),
            );
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
                .map_or("No run selected", |(_, label)| label.as_str());
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(select_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            if let Some(picked) = select(
                &mut child,
                "verify.toolbar.run",
                "Run",
                shown,
                &options,
                RUN_SELECT_W,
            ) {
                output.run = self.runs.candidates.get(picked).map(|(index, _)| *index);
            }
        }

        let mut x = row.right() - TABLE_EDGE_INSET;
        for (index, (action, action_width)) in
            self.actions.iter().zip(&action_widths).enumerate().rev()
        {
            let button_rect = Rect::from_min_size(
                pos2(x - action_width, row.center().y - control_h * 0.5),
                vec2(*action_width, control_h),
            );
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(button_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            let response = action.button().show(&mut child);
            #[cfg(test)]
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
            let line = if summary_inline {
                Rect::from_x_y_ranges(summary_left..=summary_right, row.y_range())
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
}

/// Paint the summary on its line: the problem's mark, if any, then the words.
fn paint_summary(ui: &mut Ui, line: Rect, summary: &Summary) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let mark_room = if summary.problem.is_some() {
        MARK_ADVANCE
    } else {
        0.0
    };
    let words = elide(
        ui,
        &summary.text,
        &font,
        (line.width() - mark_room).max(0.0),
    );
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
    #[cfg(test)]
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
    #[cfg(test)]
    super::raster::probe(ui, "empty", rect);
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let button_width = action.map_or(0.0, |label| Button::new(label).measured_width(ui) + 12.0);
    let words = elide(
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
    #[cfg(test)]
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
