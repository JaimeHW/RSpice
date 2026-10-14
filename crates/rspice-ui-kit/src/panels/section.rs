//! Section headers: the one design every panel section is titled in.
//!
//! A header is a [`super::PANEL_SECTION_H`] band in the panel's second
//! surface tone, ruled across its top, reaching the panel's edges. Its title
//! is small semibold sans, painted as written, on the panel inset. What it may
//! carry at the trailing edge — metadata, a problem's mark, one action — ends
//! on the inset on the right. Every helper here paints that one header; they
//! differ only in what they put at the trailing edge.

use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

use super::body::{after_header, before_header, panel_bleed, panel_body, panel_inset};
use super::{PANEL_SECTION_H, elide_text};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use crate::widgets::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};

/// A header whose title and metadata cannot share a line: the title over the
/// metadata, both from the inset.
pub(super) const PANEL_SECTION_WRAPPED_H: f32 = 42.0;
/// Least space between a header's title and what follows it on the line.
pub(super) const SECTION_HEADER_COLUMN_GAP: f32 = 8.0;
/// How far a header action's hover fill reaches past its words.
const ACTION_PAD: Vec2 = Vec2::new(6.0, 3.0);

/// What one header says.
#[derive(Clone, Copy)]
struct Header<'a> {
    title: &'a str,
    meta: Option<&'a str>,
    problem: Option<Problem>,
    /// One action at the trailing edge, and whether it can be taken.
    action: Option<(&'a str, bool)>,
    explanation: Option<&'a str>,
}

impl<'a> Header<'a> {
    fn titled(title: &'a str) -> Self {
        Self {
            title,
            meta: None,
            problem: None,
            action: None,
            explanation: None,
        }
    }
}

/// A panel section's header: its title as written — sentence case, no letter
/// tracking — with optional metadata at the trailing edge.
pub fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let _ = header(
        ui,
        Header {
            meta,
            ..Header::titled(title)
        },
    );
}

/// [`section_header`] whose metadata needs a sentence to be read correctly:
/// `explanation` is the header's hover text in place of its own words.
pub fn section_header_explained(ui: &mut Ui, title: &str, meta: &str, explanation: &str) {
    let _ = header(
        ui,
        Header {
            meta: Some(meta),
            explanation: Some(explanation),
            ..Header::titled(title)
        },
    );
}

/// [`section_header`] with one action at its trailing edge in place of
/// metadata — "Properties…" on an instance's identity, say. Returns the
/// action's response; a disabled action still reports hovers, so its reason
/// can be shown.
pub fn section_header_action(ui: &mut Ui, title: &str, action: &str, enabled: bool) -> Response {
    header(
        ui,
        Header {
            action: Some((action, enabled)),
            ..Header::titled(title)
        },
    )
    .expect("a header with an action returns the action's response")
}

/// One section: its header, with `status` as the header's metadata and a
/// `problem`'s mark in front of it, then `body` on the panel inset and rhythm.
///
/// A problem's mark is the header's only colour. Outside a panel body the
/// section is a body of its own, so a section dropped into a dialog or a
/// surface is inset and spaced exactly as one in the inspector is.
pub fn code_inspector_section(
    ui: &mut Ui,
    title: &str,
    status: Option<&str>,
    problem: Option<Problem>,
    body: impl FnOnce(&mut Ui),
) {
    panel_body(ui, |ui| {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let _ = header(
                ui,
                Header {
                    meta: status,
                    problem,
                    ..Header::titled(title)
                },
            );
            body(ui);
        });
    });
}

/// The second header level: a group of rows inside one section's body — the
/// watches under a debugger's controls, one run's signals in a list of runs.
///
/// The rule between the two levels: a section header titles a section of the
/// panel and is the only header with a band; a subsection header titles a
/// group inside a section's body, so it never opens a panel or stands where a
/// section would. It is one row tall, in the section title's face and the
/// faint tone, on the panel inset, with no band and no rule, and it is a block
/// of the body: one section gap above it unless it is the body's first.
pub fn subsection_header(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    super::section_block_gap(ui);
    let inset = panel_inset(ui);
    let height = super::property_row_height(ui);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
        node.set_level(4);
    });
    let font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let left = rect.left() + inset;
    let shown = elide_text(ui, title, &font, (rect.right() - inset - left).max(0.0));
    let galley = layout(ui, &shown, &font, t.color.text_faint);
    ui.painter().galley(
        Pos2::new(left, rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text_faint,
    );
}

/// The header's spoken name: its title, then its metadata.
pub(super) fn section_header_full_text(title: &str, meta: Option<&str>) -> String {
    meta.map_or_else(|| title.to_owned(), |meta| format!("{title}, {meta}"))
}

pub(super) fn section_header_wraps(
    available_width: f32,
    desired_title_width: f32,
    desired_meta_width: f32,
    has_meta: bool,
) -> bool {
    let available_width = available_width.max(0.0);
    desired_title_width > available_width
        || (has_meta
            && desired_title_width + SECTION_HEADER_COLUMN_GAP + desired_meta_width
                > available_width)
}

/// Allocate a section header from the measured copy instead of a fixed ratio.
///
/// Short title/metadata pairs remain complete at the narrowest dock width.
/// Only genuinely over-constrained pairs are elided, with the title retaining
/// the slightly larger share because it identifies the section.
pub(super) fn section_header_column_widths(
    available_width: f32,
    desired_title_width: f32,
    desired_meta_width: f32,
    has_meta: bool,
) -> (f32, f32) {
    let available_width = available_width.max(0.0);
    if !has_meta {
        return (available_width, 0.0);
    }

    let columns_width = (available_width - SECTION_HEADER_COLUMN_GAP).max(0.0);
    if desired_title_width + desired_meta_width <= columns_width {
        return (
            (columns_width - desired_meta_width).max(0.0),
            desired_meta_width.max(0.0),
        );
    }

    let meta_region_width = desired_meta_width.max(0.0).min(columns_width * 0.45);
    (
        (columns_width - meta_region_width).max(0.0),
        meta_region_width,
    )
}

fn layout(
    ui: &Ui,
    text: &str,
    font: &egui::FontId,
    color: Color32,
) -> std::sync::Arc<egui::Galley> {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), color)
}

/// Lay the header out on the panel rhythm, paint it, and return its action's
/// response when it has one.
fn header(ui: &mut Ui, header: Header<'_>) -> Option<Response> {
    before_header(ui);
    let response = paint(ui, header);
    after_header(ui);
    response
}

fn paint(ui: &mut Ui, header: Header<'_>) -> Option<Response> {
    let t = Tokens::get(ui.ctx());
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let action_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let inset = panel_inset(ui);
    let width = ui.available_width().max(1.0);
    let content_width = (width - 2.0 * inset).max(0.0);

    let title_width = layout(ui, header.title, &title_font, t.color.text_dim)
        .size()
        .x;
    let mark_room = match (header.problem, header.meta) {
        (None, _) => 0.0,
        (Some(_), Some(_)) => PROBLEM_MARK_ADVANCE,
        (Some(_), None) => PROBLEM_MARK_SIDE,
    };
    let meta_width = header.meta.map_or(0.0, |meta| {
        layout(ui, meta, &meta_font, t.color.text_faint).size().x
    }) + mark_room;
    let action_width = header.action.map_or(0.0, |(action, _)| {
        layout(ui, action, &action_font, t.color.text_dim).size().x
    });
    let trailing = meta_width.max(action_width);
    let has_trailing = trailing > 0.0;
    // An action stays on the title's line, eliding the title if it has to:
    // a control moved under the words it acts for reads as a second row.
    let wraps = header.action.is_none()
        && section_header_wraps(content_width, title_width, trailing, has_trailing);
    let height = if wraps {
        PANEL_SECTION_WRAPPED_H
    } else {
        PANEL_SECTION_H
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let spoken = section_header_full_text(header.title, header.meta);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), spoken.clone())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(spoken.clone());
        node.set_level(3);
    });
    let _ = response.on_hover_text(
        header
            .explanation
            .map_or_else(|| spoken.clone(), str::to_owned),
    );

    let band = panel_bleed(ui, rect);
    let fill = t.color.bg_panel_2;
    ui.painter().rect_filled(
        band,
        0.0,
        Color32::from_rgba_unmultiplied(fill.r(), fill.g(), fill.b(), 204),
    );
    ui.painter()
        .hline(band.x_range(), band.top(), Stroke::new(1.0, t.color.border));

    let left = rect.left() + inset;
    let right = rect.right() - inset;
    let clip = Rect::from_x_y_ranges(left..=right.max(left), rect.y_range());
    let painter = ui.painter().with_clip_rect(clip.intersect(ui.clip_rect()));

    if wraps && !has_trailing {
        // A title alone takes the band's two lines before it is elided.
        let mut job = egui::text::LayoutJob::simple(
            header.title.to_owned(),
            title_font,
            t.color.text_dim,
            content_width.max(1.0),
        );
        job.wrap.max_rows = 2;
        job.wrap.break_anywhere = true;
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        painter.galley(
            Pos2::new(left, rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
        return None;
    }
    if wraps {
        // Title over its metadata, both from the inset and centred as one
        // block in the band. Right-aligned on a line of its own, the metadata
        // hung under nothing and read as a layout fault.
        let title = elide_text(ui, header.title, &title_font, content_width);
        let title_galley = layout(ui, &title, &title_font, t.color.text_dim);
        let meta = header.meta.map(|meta| {
            let shown = elide_text(ui, meta, &meta_font, (content_width - mark_room).max(0.0));
            layout(ui, &shown, &meta_font, t.color.text_faint)
        });
        let line_gap = 2.0;
        let second_line = meta.as_ref().map_or(
            if header.problem.is_some() {
                PROBLEM_MARK_SIDE
            } else {
                0.0
            },
            |meta| meta.size().y,
        );
        let block = title_galley.size().y
            + if second_line > 0.0 {
                line_gap + second_line
            } else {
                0.0
            };
        let top = rect.center().y - block * 0.5;
        let second_top = top + title_galley.size().y + line_gap;
        painter.galley(Pos2::new(left, top), title_galley, t.color.text_dim);
        if let Some(problem) = header.problem {
            paint_problem_mark(
                &painter,
                Rect::from_center_size(
                    Pos2::new(
                        left + PROBLEM_MARK_SIDE * 0.5,
                        second_top + second_line * 0.5,
                    ),
                    Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                problem,
            );
        }
        if let Some(meta) = meta {
            painter.galley(
                Pos2::new(left + mark_room, second_top),
                meta,
                t.color.text_faint,
            );
        }
        return None;
    }

    let (title_region, trailing_region) = if header.action.is_some() {
        let columns = (content_width - SECTION_HEADER_COLUMN_GAP - action_width).max(0.0);
        (columns, action_width.min(content_width))
    } else {
        section_header_column_widths(content_width, title_width, trailing, has_trailing)
    };
    let title = elide_text(ui, header.title, &title_font, title_region);
    let title_galley = layout(ui, &title, &title_font, t.color.text_dim);
    painter.galley(
        Pos2::new(left, rect.center().y - title_galley.size().y * 0.5),
        title_galley,
        t.color.text_dim,
    );

    if let Some((action, enabled)) = header.action {
        let galley = layout(
            ui,
            action,
            &action_font,
            if enabled {
                t.color.text_dim
            } else {
                t.color.text_faint
            },
        );
        let text = Rect::from_min_size(
            Pos2::new(
                right - galley.size().x,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley.size(),
        );
        let target = text.expand2(ACTION_PAD).intersect(rect);
        let mut response = ui.interact(
            target,
            ui.id()
                .with(("section-header-action", header.title, action)),
            if enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if !enabled {
            crate::widgets::mark_response_disabled(&mut response);
        }
        if enabled && response.hovered() {
            ui.painter().rect_filled(target, t.radius, t.color.bg_hover);
        }
        ui.painter().galley(
            text.min,
            galley,
            if enabled {
                t.color.text_dim
            } else {
                t.color.text_faint
            },
        );
        theme::paint_focus_ring_outset(ui, &response, target);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, action.to_owned())
        });
        return Some(if enabled {
            response.on_hover_cursor(egui::CursorIcon::PointingHand)
        } else {
            response
        });
    }

    let trailing_left = right - trailing_region;
    let mut meta_right = right;
    if let Some(meta) = header.meta {
        let shown = elide_text(ui, meta, &meta_font, (trailing_region - mark_room).max(0.0));
        let galley = layout(ui, &shown, &meta_font, t.color.text_faint);
        meta_right = right - galley.size().x;
        painter.galley(
            Pos2::new(meta_right, rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_faint,
        );
        meta_right -= PROBLEM_MARK_ADVANCE - PROBLEM_MARK_SIDE;
    }
    if let Some(problem) = header.problem {
        let center = Pos2::new(
            (meta_right - PROBLEM_MARK_SIDE * 0.5).max(trailing_left + PROBLEM_MARK_SIDE * 0.5),
            rect.center().y,
        );
        paint_problem_mark(
            &painter,
            Rect::from_center_size(center, Vec2::splat(PROBLEM_MARK_SIDE)),
            problem,
        );
    }
    None
}
