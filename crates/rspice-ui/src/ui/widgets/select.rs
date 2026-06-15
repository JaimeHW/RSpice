//! Select — the design-system dropdown: an inset mono well with a chevron,
//! opening a token-styled option list. Returns the picked index.

use egui::{Sense, Shape, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Render a select control showing `selected`; returns `Some(index)` when
/// an option is picked this frame.
pub fn select(
    ui: &mut Ui,
    id_salt: &str,
    selected: &str,
    options: &[String],
    width: f32,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        Stroke::new(1.0, theme::mix(c.border, c.border_strong, hover)),
    );
    painter.text(
        egui::pos2(rect.left() + 8.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        selected,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text,
    );
    let cx = rect.right() - 11.0;
    let cy = rect.center().y - 0.5;
    painter.add(Shape::line(
        vec![
            egui::pos2(cx - 3.5, cy - 1.5),
            egui::pos2(cx, cy + 2.5),
            egui::pos2(cx + 3.5, cy - 1.5),
        ],
        Stroke::new(1.4, c.text_dim),
    ));

    let popup_id = ui.make_persistent_id(("volta.select", id_salt));
    if response.clicked() {
        ui.memory_mut(|memory| memory.toggle_popup(popup_id));
    }

    let mut picked = None;
    egui::popup::popup_below_widget(
        ui,
        popup_id,
        &response,
        egui::PopupCloseBehavior::CloseOnClick,
        |ui| {
            ui.set_min_width(width.max(rect.width()));
            ui.spacing_mut().item_spacing.y = 0.0;
            for (index, option) in options.iter().enumerate() {
                let is_current = option == selected;
                let (row, row_response) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 24.0), Sense::click());
                let painter = ui.painter();
                if row_response.hovered() {
                    painter.rect_filled(row, t.radius, c.bg_hover);
                }
                painter.text(
                    egui::pos2(row.left() + 8.0, row.center().y),
                    egui::Align2::LEFT_CENTER,
                    option,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    if is_current { c.accent } else { c.text },
                );
                if row_response
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    picked = Some(index);
                }
            }
        },
    );
    picked
}
