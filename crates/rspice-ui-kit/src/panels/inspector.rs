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

/// Where every line of inspector text starts, measured from the panel edge,
/// and where it stops short of the far one: a property row's own padding.
///
/// Blocks that paint their own rows — a viewer's statistics table, a note —
/// read this rather than restating it, so they stay on the rows' column.
pub const INSPECTOR_TEXT_INSET: f32 = super::PROPERTY_ROW_PAD;

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

/// [`section_header`] whose metadata qualifies everything in the section —
/// how its values were computed, say — and needs a sentence to be read
/// correctly. The sentence is the header's hover text.
pub fn section_header_explained(ui: &mut Ui, title: &str, meta: &str, explanation: &str) {
    close_open_section(ui);
    super::section_header_explained(ui, title, meta, explanation);
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

/// A sentence in an inspector body: what a section's rows mean, rather than
/// one more of them.
///
/// The one place an inspector writes prose. It wraps inside the same left and
/// right inset a property row gives its label and value, so a note lines up
/// with the rows around it and never starts on, or runs into, the panel edge
/// — which is what a bare `Label` added to the body does, because the inset
/// belongs to the rows rather than to the panel.
pub fn muted_inspector_copy(ui: &mut Ui, text: &str) {
    let tone = Tokens::get(ui.ctx()).color.text_dim;
    toned_inspector_copy(ui, text, tone);
}

/// [`muted_inspector_copy`] in a tone that carries meaning: a warning a reader
/// should act on, or a remark quieter than the rows it sits under.
pub fn toned_inspector_copy(ui: &mut Ui, text: &str, tone: Color32) {
    let inset = INSPECTOR_TEXT_INSET;
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        tone,
        (ui.available_width() - 2.0 * inset).max(1.0),
    );
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(
            ui.available_width(),
            galley.size().y + 2.0 * INSPECTOR_COPY_PAD_Y,
        ),
        Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().galley(
        Pos2::new(rect.left() + inset, rect.top() + INSPECTOR_COPY_PAD_Y),
        galley,
        tone,
    );
}

/// Space above and below a note's text: the gap a property row leaves between
/// its own edge and the text centred in it.
const INSPECTOR_COPY_PAD_Y: f32 = 8.0;

/// A block that is neither a row nor a note — a table, a scrolled list, a
/// checkbox — placed on the inset the rows' labels start at and stopped at the
/// inset their values end at.
pub fn inspector_inset<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let inset = INSPECTOR_TEXT_INSET as i8;
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(inset, 0))
        .show(ui, add)
        .inner
}
