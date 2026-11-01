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

/// What a menu row shows in its leading column, before the label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuRowLead {
    /// Nothing: the label keeps its column.
    Blank,
    /// The icon of the command's toolbar button.
    Icon(WorkbenchIcon),
    /// The row switches a state: a check mark while it is on, nothing while
    /// it is off.
    Check(bool),
    /// The row is one of a group of which one is on: a dot on that one.
    Radio(bool),
}

/// One menu row's paint: its hover fill, leading mark, label and trailing
/// mark. The label underlines `access` while the access keys show.
pub(super) fn paint_menu_row(
    ui: &Ui,
    rect: Rect,
    highlighted: bool,
    enabled: bool,
    lead: MenuRowLead,
    label: &str,
    access: Option<char>,
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
    match lead {
        MenuRowLead::Blank | MenuRowLead::Check(false) | MenuRowLead::Radio(false) => {}
        // A painted dot, centred where a check mark stands.
        MenuRowLead::Radio(true) => {
            ui.painter().circle_filled(
                icon_rect.center(),
                3.0,
                if enabled { t.color.text } else { ink },
            );
        }
        MenuRowLead::Icon(icon) => icon.paint(ui.painter(), icon_rect, ink),
        // Painted as a vector: the bundled faces have no check glyph. Full
        // ink even at rest, so the state reads before the row is hovered.
        MenuRowLead::Check(true) => WorkbenchIcon::Check.paint(
            ui.painter(),
            icon_rect,
            if enabled { t.color.text } else { ink },
        ),
    }
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
    paint_access_label(
        &ui.painter().with_clip_rect(egui::Rect::from_min_max(
            egui::pos2(label_left, rect.top()),
            egui::pos2(label_right, rect.bottom()),
        )),
        egui::Pos2::new(label_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        painted_label,
        label_font,
        ink,
        access.filter(|_| access_cues_shown(ui.ctx())),
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

/// Paint `text` at `anchor`, underlining the letter `access` names (see
/// [`application_menus::access_key_index`]) when it is given. The underline
/// is a rule drawn under the glyph, not a combining mark, so it needs
/// nothing of the font.
pub(super) fn paint_access_label(
    painter: &egui::Painter,
    anchor: egui::Pos2,
    align: egui::Align2,
    text: String,
    font: egui::FontId,
    ink: egui::Color32,
    access: Option<char>,
) {
    let index = access.and_then(|key| application_menus::access_key_index(&text, key));
    let galley = painter.layout_no_wrap(text, font, ink);
    let origin = align.anchor_size(anchor, galley.size()).min;
    if let Some(index) = index
        && let Some(row) = galley.rows.first()
        && let Some(glyph) = row.glyphs.get(index)
    {
        let left = origin.x + row.pos.x + glyph.pos.x;
        let baseline = origin.y + row.pos.y + glyph.pos.y;
        painter.hline(
            left..=left + glyph.advance_width,
            (baseline + 2.0).round() - 0.5,
            egui::Stroke::new(1.0, ink),
        );
    }
    painter.galley(origin, galley, ink);
}
