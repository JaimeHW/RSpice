//! Section framing shared by engineering inspectors. Keeps the existing egui session IDs.

use super::{
    PANEL_SECTION_H, schematic_section_header as design_schematic_section_header,
    section_header as design_section_header,
};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Space a section leaves above the first block of its body, below the last,
/// and between two blocks of the same body.
///
/// One measure governs every section — property list, tree, annotation card
/// or action row — so the panel reads as a single rhythm instead of a stack
/// of separately tuned boxes, and no section is framed unevenly.
pub const INSPECTOR_SECTION_PADDING: f32 = 8.0;

fn inspector_section_state_id() -> egui::Id {
    egui::Id::new("workbench.inspector.property-list-open")
}

/// Top of the open section's body, so a second block in the same body can
/// tell itself apart from the first one.
fn inspector_section_body_id() -> egui::Id {
    egui::Id::new("workbench.inspector.section-body-top")
}

pub fn begin_inspector_sections(ui: &mut Ui) {
    ui.data_mut(|data| {
        data.insert_temp(inspector_section_state_id(), -1.0_f32);
        data.remove_temp::<f32>(inspector_section_body_id());
    });
}

/// Close the body of the section above, if there is one.
fn close_open_section(ui: &mut Ui) {
    let previous_bottom = ui.data_mut(|data| {
        data.get_temp::<f32>(inspector_section_state_id())
            .unwrap_or(-1.0)
    });
    if previous_bottom >= 0.0 {
        ui.add_space(previous_bottom);
    }
}

/// Open a body under the header just painted, and record the step the body
/// owes its last block.
fn open_section_body(ui: &mut Ui) {
    ui.add_space(INSPECTOR_SECTION_PADDING);
    let body_top = ui.cursor().top();
    ui.data_mut(|data| {
        data.insert_temp(inspector_section_state_id(), INSPECTOR_SECTION_PADDING);
        data.insert_temp(inspector_section_body_id(), body_top);
    });
}

/// One section step between two blocks of the same body. The body's first
/// block already sits below the header's padding and takes no further gap.
pub fn section_block_gap(ui: &mut Ui) {
    let body_top = ui.data_mut(|data| data.get_temp::<f32>(inspector_section_body_id()));
    if body_top.is_none_or(|top| ui.cursor().top() > top + 0.5) {
        ui.add_space(INSPECTOR_SECTION_PADDING);
    }
}

pub fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    close_open_section(ui);
    design_section_header(ui, title, meta);
    open_section_body(ui);
}

/// Schematic-dock section heading: the same rhythm as [`section_header`] with
/// the schematic's larger, tracked EDA typography.
pub fn schematic_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    close_open_section(ui);
    design_schematic_section_header(ui, title, meta);
    open_section_body(ui);
}

pub fn schematic_section_header_action(
    ui: &mut Ui,
    title: &str,
    action: &str,
    enabled: bool,
) -> Response {
    close_open_section(ui);
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));

    let title_job = egui::text::LayoutJob::single_section(
        title.to_uppercase(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_2, FontWeight::SemiBold),
            color: t.color.text_dim,
            extra_letter_spacing: 0.055 * tokens::FS_2,
            ..Default::default()
        },
    );
    let title_galley = ui.fonts_mut(|fonts| fonts.layout_job(title_job));
    ui.painter().galley(
        Pos2::new(
            rect.left() + 10.0,
            rect.center().y - title_galley.size().y * 0.5,
        ),
        title_galley,
        t.color.text_dim,
    );

    let action_galley = ui.painter().layout_no_wrap(
        action.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let action_rect = Rect::from_min_max(
        Pos2::new(
            rect.right() - 10.0 - action_galley.size().x - 10.0,
            rect.top() + 2.0,
        ),
        Pos2::new(rect.right() - 8.0, rect.bottom() - 2.0),
    );
    let mut response = ui.interact(
        action_rect,
        ui.id().with(("schematic-section-action", title, action)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if !enabled {
        crate::widgets::mark_response_disabled(&mut response);
    }
    if response.hovered() && enabled {
        ui.painter()
            .rect_filled(action_rect, t.radius, t.color.bg_hover);
    }
    ui.painter().galley(
        Pos2::new(
            action_rect.center().x - action_galley.size().x * 0.5,
            action_rect.center().y - action_galley.size().y * 0.5,
        ),
        action_galley,
        t.color.text_dim,
    );
    theme::paint_focus_ring_outset(ui, &response, action_rect);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, action.to_owned())
    });
    open_section_body(ui);
    response
}

pub fn finish_inspector_sections(ui: &mut Ui) {
    let bottom = ui
        .data_mut(|data| {
            data.remove_temp::<f32>(inspector_section_body_id());
            data.remove_temp::<f32>(inspector_section_state_id())
        })
        .unwrap_or(-1.0);
    if bottom >= 0.0 {
        ui.add_space(bottom);
    }
}

pub fn muted_inspector_copy(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (ui.available_width() - 20.0).max(1.0),
    );
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), galley.size().y + 16.0),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().galley(
        Pos2::new(rect.left() + 10.0, rect.top() + 8.0),
        galley,
        t.color.text_dim,
    );
}
