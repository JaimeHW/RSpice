//! The mockup's `.segmented` control: contiguous option buttons in one
//! bordered group.
//!
//! A segmented control is how this product says "one of these, and here are
//! all of them" in a single row — the scope of a browser, the mode of a
//! preference. It reads as one object: a shared border and inset fill, hair
//! rules between the options, and the chosen one lifted with the active fill
//! plus an accent underline. Arrow keys walk the options, because a control
//! that behaves like a radio group must be reachable like one.

use egui::{Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// `.segmented button { min-width: 54px; padding: 0 8px; }`
const SEGMENT_MIN_WIDTH: f32 = 54.0;
const SEGMENT_HORIZONTAL_PADDING: f32 = 8.0;
const PHONE_MAX_WIDTH: f32 = 560.0;

/// How the group sizes itself against the space it is given.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SegmentedWidth {
    /// Hug the options, as `display: inline-flex` does. The group is an
    /// object in a row, not the row itself.
    Natural,
    /// Fill the available width at the phone breakpoint, where the owning
    /// grid item stretches. The options keep their natural widths and the
    /// remaining inset area stays deliberately blank.
    PhoneStretch,
}

/// Contiguous option buttons matching the mockup `.segmented` control.
///
/// Returns whether the selection changed.
pub(crate) fn segmented(
    ui: &mut Ui,
    id_salt: &'static str,
    options: &[&str],
    selected: &mut usize,
    sizing: SegmentedWidth,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h;
    let natural_widths = options
        .iter()
        .map(|label| {
            let text_width = ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(
                        (*label).to_owned(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text,
                    )
                    .size()
                    .x
            });
            (text_width + SEGMENT_HORIZONTAL_PADDING * 2.0).max(SEGMENT_MIN_WIDTH)
        })
        .collect::<Vec<_>>();
    let desired_width = natural_widths.iter().sum::<f32>();
    let phone = ui.ctx().content_rect().width() <= PHONE_MAX_WIDTH;
    let width = if phone && sizing == SegmentedWidth::PhoneStretch {
        ui.available_width()
    } else {
        desired_width.min(ui.available_width())
    };
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let scale = if desired_width > 0.0 {
        (rect.width() / desired_width).min(1.0)
    } else {
        1.0
    };
    let mut changed = false;
    ui.painter().rect(
        rect,
        t.radius,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let mut cell_left = rect.left();
    for (index, label) in options.iter().enumerate() {
        let cell_width = natural_widths[index] * scale;
        let cell = Rect::from_min_max(
            egui::pos2(cell_left, rect.top()),
            egui::pos2(
                if index + 1 == options.len() && rect.width() <= desired_width {
                    rect.right()
                } else {
                    cell_left + cell_width
                },
                rect.bottom(),
            ),
        );
        cell_left = cell.right();
        let response = ui.interact(
            cell,
            ui.make_persistent_id((id_salt, index)),
            Sense::click(),
        );
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::RadioButton, true, *selected == index, *label)
        });
        let active = *selected == index;
        let fill = if active {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        };
        if fill != egui::Color32::TRANSPARENT {
            ui.painter().rect_filled(cell.shrink(1.0), 0.0, fill);
        }
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        if active {
            ui.painter().hline(
                (cell.left() + 1.0)..=(cell.right() - 1.0),
                cell.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
        ui.painter().text(
            cell.center(),
            egui::Align2::CENTER_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if active {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, cell);
        let keyboard_previous =
            response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::ArrowLeft));
        let keyboard_next =
            response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::ArrowRight));
        let next = if keyboard_previous {
            index.checked_sub(1)
        } else if keyboard_next {
            (index + 1 < options.len()).then_some(index + 1)
        } else {
            None
        };
        if let Some(next) = next {
            *selected = next;
            let next_id = ui.make_persistent_id((id_salt, next));
            ui.memory_mut(|memory| memory.request_focus(next_id));
            changed = true;
        } else if response.clicked() && !active {
            *selected = index;
            changed = true;
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_metrics_match_the_mockup_control() {
        assert_eq!(SEGMENT_MIN_WIDTH, 54.0);
        assert_eq!(SEGMENT_HORIZONTAL_PADDING, 8.0);
        assert_eq!(PHONE_MAX_WIDTH, 560.0);
    }

    /// Options are sized to their own labels, never to an equal share: the
    /// control is a row of named choices, not a table.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn segmented_options_keep_mockup_natural_widths() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(800.0, 600.0))),
            ..Default::default()
        };
        let mut selected = 0;

        let output = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = segmented(
                    ui,
                    "widgets.test.density",
                    &["Compact", "Comfortable"],
                    &mut selected,
                    SegmentedWidth::PhoneStretch,
                );
            });
        });
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        let widths = ["Compact", "Comfortable"].map(|label| {
            nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::RadioButton && node.label() == Some(label)
                })
                .and_then(|(_, node)| node.bounds())
                .map(|bounds| bounds.x1 - bounds.x0)
                .unwrap_or_else(|| panic!("missing segmented option {label}"))
        });

        assert!(widths[0] >= SEGMENT_MIN_WIDTH as f64);
        assert!(widths[1] > widths[0]);
    }
}
