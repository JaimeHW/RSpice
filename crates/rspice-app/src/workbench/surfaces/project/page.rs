//! Geometry shared by the project workspace's carded pages.
//!
//! Overview and Recovery share the landing surfaces' layout: a header band and a
//! centred content column on the application ground, with bordered cards that
//! size to their content. A sparse page then shows ground below its cards
//! instead of panels stretched to the bottom of the viewport.

use egui::{
    Align, Align2, Color32, CornerRadius, Layout, Rect, Sense, Stroke, Ui, UiBuilder, pos2, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::centered_content_rect;

use super::visible_workspace_width;

/// Shared page measure of the workbench landing surfaces, so the project
/// pages, the no-project landing, and the netlist-first landing read as one
/// product.
pub(super) const CONTENT_MAX_WIDTH: f32 = 1240.0;
pub(super) const DESKTOP_GUTTER: f32 = 30.0;
pub(super) const HEADER_TOP: f32 = 15.0;
pub(super) const HEADER_BOTTOM: f32 = 14.0;
pub(super) const BODY_TOP: f32 = 14.0;
pub(super) const BODY_BOTTOM: f32 = 26.0;
pub(super) const STACK_BREAKPOINT: f32 = 900.0;
pub(super) const COLUMN_GAP: f32 = 14.0;
pub(super) const CARD_GAP: f32 = 10.0;
const CARD_HEADER_HEIGHT: f32 = 26.0;

/// Lay a page out on the application ground: its own vertical scroll, and a
/// content column `body` receives as `(inset, width)` from the left edge.
pub(super) fn show(ui: &mut Ui, id_salt: &str, body: impl FnOnce(&mut Ui, f32, f32)) {
    let tokens = Tokens::get(ui.ctx());
    let workspace_width = visible_workspace_width(ui);
    let viewport_height = ui.available_height().max(1.0);
    egui::Frame::new().fill(tokens.color.bg_app).show(ui, |ui| {
        ui.set_min_height(viewport_height);
        ui.set_width(workspace_width);
        egui::ScrollArea::vertical()
            .id_salt(id_salt)
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_min_width(workspace_width);
                let content = centered_content_rect(
                    Rect::from_min_size(pos2(0.0, 0.0), vec2(workspace_width, 1.0)),
                    DESKTOP_GUTTER,
                    CONTENT_MAX_WIDTH,
                );
                body(ui, content.left(), content.width());
                ui.add_space(BODY_BOTTOM);
            });
    });
}

/// Run `body` in a top-down column `width` wide, `inset` from the left edge.
pub(super) fn inset_column<R>(
    ui: &mut Ui,
    inset: f32,
    width: f32,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(inset);
        ui.allocate_ui_with_layout(vec2(width, 1.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 0.0;
            body(ui)
        })
        .inner
    })
    .inner
}

/// The full-width rule that closes a page's header band.
pub(super) fn header_rule(ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(HEADER_BOTTOM);
    let rule_width = ui.available_width().max(1.0);
    let (rule_rect, _) = ui.allocate_exact_size(vec2(rule_width, 1.0), Sense::hover());
    ui.painter().hline(
        rule_rect.x_range(),
        rule_rect.top(),
        Stroke::new(1.0, tokens.color.border),
    );
}

/// The reference dashboard columns: `minmax(300px, 1.3fr) minmax(260px, 1fr)`.
pub(super) fn column_widths(content_width: f32) -> (f32, f32) {
    let usable = (content_width - COLUMN_GAP).max(2.0);
    let right = (usable * 0.435).clamp(260.0_f32.min(usable * 0.5), 500.0);
    ((usable - right).max(1.0), right)
}

/// Two columns laid out at their natural content height; the page — not the
/// cards — owns the leftover vertical space.
pub(super) fn columns(
    ui: &mut Ui,
    inset: f32,
    content_width: f32,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    let (left_width, right_width) = column_widths(content_width);
    let origin = ui.cursor().min;
    let unbounded = 40_000.0;
    let mut left_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(Rect::from_min_size(
                pos2(origin.x + inset, origin.y),
                vec2(left_width, unbounded),
            ))
            .layout(Layout::top_down(Align::Min)),
    );
    left_ui.spacing_mut().item_spacing.y = 0.0;
    left(&mut left_ui);
    let mut right_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(Rect::from_min_size(
                pos2(origin.x + inset + left_width + COLUMN_GAP, origin.y),
                vec2(right_width, unbounded),
            ))
            .layout(Layout::top_down(Align::Min)),
    );
    right_ui.spacing_mut().item_spacing.y = 0.0;
    right(&mut right_ui);
    let used = left_ui
        .min_rect()
        .height()
        .max(right_ui.min_rect().height());
    ui.allocate_rect(
        Rect::from_min_size(origin, vec2(ui.available_width().max(1.0), used)),
        Sense::hover(),
    );
}

/// A bordered card: title bar, toned meta, content-sized body.
pub(super) fn card<R>(
    ui: &mut Ui,
    title: &str,
    meta: &str,
    meta_color: Color32,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let tokens = Tokens::get(ui.ctx());
    // A frame's stroke is drawn outside its content, so content as wide as the
    // column made every card two points wider than the column. The column then
    // grew to fit it, and each card below came out two points wider again.
    let border = 1.0;
    let width = (ui.available_width() - 2.0 * border).max(1.0);
    let mut output = None;
    let shown = egui::Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(border, tokens.color.border))
        .corner_radius(tokens.radius)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_width(width);
            card_title_row(ui, title, meta, meta_color);
            output = Some(body(ui));
            ui.add_space(9.0);
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Group);
        node.set_label(title);
    });
    output.expect("the card body runs exactly once")
}

fn card_title_row(ui: &mut Ui, title: &str, meta: &str, meta_color: Color32) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, CARD_HEADER_HEIGHT), Sense::hover());
    let radius = tokens.radius as u8;
    ui.painter().rect_filled(
        rect,
        CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        },
        tokens.color.bg_panel_2,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let meta = elide_text(ui, &meta.to_uppercase(), &meta_font, (width * 0.5).max(1.0));
    let meta_width = ui
        .painter()
        .layout_no_wrap(meta.clone(), meta_font.clone(), Color32::WHITE)
        .size()
        .x;
    ui.painter().text(
        pos2(rect.right() - 10.0, rect.center().y),
        Align2::RIGHT_CENTER,
        &meta,
        meta_font,
        meta_color,
    );
    paint_elided(
        ui,
        pos2(rect.left() + 10.0, rect.center().y - tokens::FS_0 * 0.5),
        &title.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        tokens.color.text_dim,
        (width - meta_width - 28.0).max(1.0),
    );
    response.widget_info(|| {
        let label = if meta.is_empty() {
            title.to_owned()
        } else {
            format!("{title}: {meta}")
        };
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
    });
}

pub(super) fn paint_elided(
    ui: &Ui,
    position: egui::Pos2,
    text: &str,
    font: egui::FontId,
    color: Color32,
    max_width: f32,
) -> Rect {
    let text = elide_text(ui, text, &font, max_width);
    ui.painter()
        .text(position, Align2::LEFT_TOP, text, font, color)
}

pub(super) fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0;
    let mut high = characters.len();
    while low < high {
        let midpoint = (low + high).div_ceil(2);
        let candidate = characters[..midpoint]
            .iter()
            .copied()
            .chain(std::iter::once('…'))
            .collect::<String>();
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = midpoint;
        } else {
            high = midpoint - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('…'))
        .collect()
}
