//! The one bar a page opens with.
//!
//! Controls stand left to right from the bar's inset: the page's filter, its
//! kind select, its actions, each one control tall. What the page reports (a
//! count, a state) stands at the right end in neutral dim words, behind a
//! problem's mark when it is one. The report gives way first: it drops its
//! leading facts, then elides, into whatever the controls leave, its whole
//! text on hover, and a control is never shortened for it. A 1-pt rule closes the bar on its last pixel row.

use std::hash::Hash;

use egui::{Align, Id, Layout, Rect, Response, Sense, Stroke, Ui, pos2, vec2};

use crate::panels::{PANEL_INSET, elide_text};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use crate::widgets::{
    FILTER_CLEAR_INSET, PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, filter_field_clear,
    paint_problem_mark, select, select_width,
};

/// The room a bar keeps above and below its controls, shared between them.
const PAGE_BAR_PAD: f32 = 4.0;
/// The gap between two controls, and between the last control and the report.
pub const PAGE_BAR_GAP: f32 = 8.0;
/// A filter's width when the bar has room for it.
const FILTER_WIDTH: f32 = 200.0;
/// The least a filter is made, whatever the bar leaves it.
const FILTER_MIN_WIDTH: f32 = 120.0;

/// A page bar's height: a control and the room around it, from the metrics,
/// so it holds its controls under a finger too.
#[must_use]
pub fn page_bar_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.ctl_h + PAGE_BAR_PAD
}

/// A page's bar: its controls, and what it reports at the right end.
#[derive(Debug, Clone, Default)]
pub struct PageBar {
    report: Option<(String, Option<Problem>)>,
}

impl PageBar {
    #[must_use]
    pub const fn new() -> Self {
        Self { report: None }
    }

    /// What the page reports at the bar's right end: neutral words, behind
    /// `problem`'s mark when the state is one.
    #[must_use]
    pub fn report(mut self, text: impl Into<String>, problem: Option<Problem>) -> Self {
        self.report = Some((text.into(), problem));
        self
    }

    /// Lay the bar across the visible width and `controls` into it, left to
    /// right, then the report into what they leave. Returns what `controls`
    /// returned.
    pub fn show<R>(self, ui: &mut Ui, controls: impl FnOnce(&mut Ui) -> R) -> R {
        let t = Tokens::get(ui.ctx());
        // The visible width, not the content's: a bar inside a wider scroll
        // still ends at the pane's edge, its report in sight.
        let width = ui
            .available_rect_before_wrap()
            .intersect(ui.clip_rect())
            .width()
            .max(1.0);
        let (rect, _) = ui.allocate_exact_size(vec2(width, page_bar_height(ui)), Sense::hover());
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border_strong),
        );
        let inner = rect.shrink2(vec2(PANEL_INSET, 0.0));
        let mut row = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
        );
        row.set_clip_rect(inner.intersect(ui.clip_rect()));
        row.spacing_mut().item_spacing.x = PAGE_BAR_GAP;
        let shown = controls(&mut row);
        if let Some((text, problem)) = &self.report {
            let used = row.min_rect();
            let left = if used.width() > 0.0 {
                used.right() + PAGE_BAR_GAP
            } else {
                inner.left()
            };
            let line = Rect::from_x_y_ranges(left..=inner.right().max(left), rect.y_range());
            paint_report(ui, line, text, *problem);
        }
        shown
    }
}

fn report_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

/// What of a report fits in `room`. A report is facts joined by " · ", the
/// one that matters most last ("9 dependencies · 1 problem"): it gives way a
/// whole fact at a time from the front, so no word is cut while a shorter
/// report would fit, and only a last fact too wide alone is elided.
fn fitted_report(ui: &Ui, text: &str, font: &egui::FontId, room: f32) -> String {
    const JOIN: &str = " \u{b7} ";
    let width = |words: &str| {
        ui.painter()
            .layout_no_wrap(words.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    };
    let mut rest = text;
    loop {
        if width(rest) <= room {
            return rest.to_owned();
        }
        match rest.split_once(JOIN) {
            Some((_, later)) => rest = later,
            None => return elide_text(ui, rest, font, room),
        }
    }
}

/// The report, right-aligned on `line`: the mark, then the words, fitted to
/// what is left with the whole text on hover.
fn paint_report(ui: &mut Ui, line: Rect, text: &str, problem: Option<Problem>) {
    let t = Tokens::get(ui.ctx());
    let font = report_font();
    let mark = if problem.is_some() {
        PROBLEM_MARK_ADVANCE
    } else {
        0.0
    };
    let room = (line.width() - mark).max(0.0);
    let words = fitted_report(ui, text, &font, room);
    if words.is_empty() {
        return;
    }
    let elided = words != text;
    let galley = ui.painter().layout_no_wrap(words, font, t.color.text_dim);
    let place = Rect::from_x_y_ranges(
        (line.right() - mark - galley.size().x)..=line.right(),
        line.y_range(),
    );
    let response = ui.interact(place, ui.id().with("page-bar.report"), Sense::hover());
    let announcement = problem.map_or_else(|| text.to_owned(), |problem| problem.announce(text));
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    if elided {
        let _ = response.on_hover_text(text);
    }
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(place.left() + PROBLEM_MARK_SIDE * 0.5, place.center().y),
                vec2(PROBLEM_MARK_SIDE, PROBLEM_MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().galley(
        pos2(
            place.left() + mark,
            place.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text_dim,
    );
}

/// A page bar's filter: the field that keeps only the rows whose words hold
/// what it holds. Its mark, or Escape, empties it.
#[derive(Debug, Clone, Copy)]
pub struct PageBarFilter<'a> {
    id: Id,
    name: &'a str,
    hint: &'a str,
}

impl<'a> PageBarFilter<'a> {
    /// A filter that assistive technology calls `name`, hinting "Filter…".
    #[must_use]
    pub fn new(id_salt: impl Hash + std::fmt::Debug, name: &'a str) -> Self {
        Self {
            id: Id::new(("page-bar.filter", id_salt)),
            name,
            hint: "Filter\u{2026}",
        }
    }

    /// The faint words the empty field shows in place of "Filter…".
    #[must_use]
    pub const fn hint(mut self, hint: &'a str) -> Self {
        self.hint = hint;
        self
    }

    pub fn show(self, ui: &mut Ui, query: &mut String) -> Response {
        let t = Tokens::get(ui.ctx());
        // egui fades the body tone for a hint; the palette's tertiary text
        // keeps its contrast on the field.
        let restored = ui.visuals_mut().weak_text_color.replace(t.color.text_faint);
        let width = FILTER_WIDTH
            .min(ui.available_width() * 0.4)
            .max(FILTER_MIN_WIDTH.min(ui.available_width()))
            .max(1.0);
        let field = ui.add_sized(
            [width, t.metrics.ctl_h],
            egui::TextEdit::singleline(query)
                .id(self.id)
                .hint_text(self.hint)
                .font(theme::field_sans())
                .vertical_align(Align::Center)
                .margin(egui::Margin {
                    left: 6,
                    right: FILTER_CLEAR_INSET,
                    top: 2,
                    bottom: 2,
                }),
        );
        ui.visuals_mut().weak_text_color = restored;
        let name = self.name.to_owned();
        ui.ctx()
            .accesskit_node_builder(field.id, |node| node.set_label(name));
        filter_field_clear(ui, &field, query);
        field
    }
}

/// A page bar's kind select: the page's "any" choice first, then each kind,
/// wide enough to say its longest choice whole. Returns the index picked.
pub fn page_bar_kind(
    ui: &mut Ui,
    id_salt: &str,
    name: &str,
    choices: &[String],
    selected: usize,
) -> Option<usize> {
    let shown = choices.get(selected).map_or("", String::as_str);
    let width = select_width(ui, choices);
    select(ui, id_salt, name, shown, choices, width)
}
