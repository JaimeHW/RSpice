//! How one menu row is painted: hover fill, icon, label and the mark at its
//! right edge. A command row and a submenu row share it, so they cannot drift
//! apart in column or ink.

use super::*;

const SUBMENU_CHEVRON_SIDE: f32 = 12.0;

/// What a menu row shows at its right edge.
#[derive(Debug, Clone, Copy)]
pub(super) enum MenuRowTrailing<'a> {
    /// The command's key, faint, or nothing when it has none.
    Shortcut(&'a str),
    /// A chevron: the row opens a submenu.
    Submenu,
}

/// One menu row's paint: its hover fill, icon, label and trailing mark.
pub(super) fn paint_menu_row(
    ui: &Ui,
    rect: Rect,
    highlighted: bool,
    enabled: bool,
    icon: WorkbenchIcon,
    label: &str,
    trailing: MenuRowTrailing<'_>,
) {
    let t = Tokens::get(ui.ctx());
    if highlighted {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let ink = if enabled {
        if highlighted {
            t.color.text
        } else {
            t.color.text_dim
        }
    } else {
        t.color.text_dim.gamma_multiply(0.4)
    };
    let faint = if enabled {
        t.color.text_faint
    } else {
        t.color.text_faint.gamma_multiply(0.4)
    };
    let icon_rect = egui::Rect::from_min_size(
        egui::Pos2::new(rect.left() + 7.0, rect.center().y - 8.5),
        Vec2::splat(17.0),
    );
    icon.paint(ui.painter(), icon_rect, ink);
    let label_left = rect.left() + 31.0;
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let trailing_width = match trailing {
        MenuRowTrailing::Shortcut("") => 0.0,
        MenuRowTrailing::Shortcut(shortcut) => {
            ui.painter()
                .layout_no_wrap(shortcut.to_owned(), shortcut_font.clone(), ink)
                .size()
                .x
                + 7.0
        }
        MenuRowTrailing::Submenu => SUBMENU_CHEVRON_SIDE + 7.0,
    };
    let label_right = rect.right() - 7.0 - trailing_width;
    let label_width = (label_right - label_left).max(8.0);
    let painted_label = ellipsize_to_width(ui.painter(), label, &label_font, label_width);
    // Keep the label in its mockup grid column even if a platform font has
    // wider metrics than the bundled face. The measured ellipsis provides a
    // readable ending; the clip is the final invariant that prevents a long
    // localized label from ever painting through the shortcut column.
    ui.painter()
        .with_clip_rect(egui::Rect::from_min_max(
            egui::pos2(label_left, rect.top()),
            egui::pos2(label_right, rect.bottom()),
        ))
        .text(
            egui::Pos2::new(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            painted_label,
            label_font,
            ink,
        );
    match trailing {
        MenuRowTrailing::Shortcut("") => {}
        MenuRowTrailing::Shortcut(shortcut) => {
            ui.painter().text(
                egui::Pos2::new(rect.right() - 7.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                shortcut,
                shortcut_font,
                faint,
            );
        }
        // Painted as a vector: the bundled faces have no chevron glyph.
        MenuRowTrailing::Submenu => WorkbenchIcon::ChevronRight.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.right() - 7.0 - SUBMENU_CHEVRON_SIDE / 2.0,
                    rect.center().y,
                ),
                Vec2::splat(SUBMENU_CHEVRON_SIDE),
            ),
            faint,
        ),
    }
}
