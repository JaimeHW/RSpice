//! Section headers — uppercase letterspaced panel section titles with an
//! optional inline action.

use egui::{Response, Sense, Ui, vec2};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

/// A panel section header ("HIERARCHY", "RUN HISTORY", …) with an optional
/// right-aligned action ("Descend", "Clear", "＋ Add"). Returns the action's
/// response when an action label is provided and it was interacted with.
pub fn section_header(ui: &mut Ui, title: &str, action: Option<&str>) -> Option<Response> {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;

    let title_galley = {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &title.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_0, FontWeight::SemiBold),
                color: c.text_faint,
                extra_letter_spacing: 0.09 * tokens::FS_0,
                ..Default::default()
            },
        );
        ui.fonts(|f| f.layout_job(job))
    };

    let width = ui.available_width();
    let height = title_galley.size().y + 9.0 + 5.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return None;
    }

    let painter = ui.painter();
    let baseline_y = rect.top() + 9.0;
    painter.galley(
        egui::pos2(rect.left() + 12.0, baseline_y),
        title_galley,
        c.text_faint,
    );

    let action_label = action?;
    let action_galley = ui.fonts(|f| {
        f.layout_no_wrap(
            action_label.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        )
    });
    let action_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - 12.0 - action_galley.size().x - 12.0,
            baseline_y - 2.0,
        ),
        action_galley.size() + vec2(12.0, 4.0),
    );
    let response = ui.interact(
        action_rect,
        ui.id().with(("section-action", title, action_label)),
        Sense::click(),
    );
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);
    if hover > 0.0 {
        painter.rect_filled(
            action_rect,
            t.radius,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    painter.galley(
        action_rect.min + vec2(6.0, 2.0),
        action_galley,
        mix(c.text_faint, c.text, hover),
    );

    Some(response.on_hover_cursor(egui::CursorIcon::PointingHand))
}
