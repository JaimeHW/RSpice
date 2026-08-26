//! The on/off switch this design system paints for a boolean.
//!
//! One painter, because a switch drawn twice is a switch drawn two sizes. It
//! lived on the Simulation Studio's analysis rail, which is where the shape was
//! authored, and the rows that wanted it elsewhere could not reach down into a
//! surface for it — so the tree row that carries a boolean went on painting
//! egui's tick box, and a form narrow enough to fall back to that row showed a
//! reader a control the rest of the application had stopped using.
//!
//! It takes a clip rectangle rather than reading the `Ui`'s, because the rows
//! that carry it are laid out inside rails that hide their own overflow: a
//! switch on a row scrolled half out of its rail must be cut at the rail's edge
//! and not at the panel's.

use egui::{Rect, Stroke, Ui, vec2};

use crate::ui::tokens::Tokens;

/// Track width. Callers reserve this much for the control itself.
pub(crate) const SWITCH_WIDTH: f32 = 30.0;

/// Track height.
const SWITCH_HEIGHT: f32 = 17.0;

/// Paint one switch centred on `center`, clipped to `clip_rect`.
///
/// `on` is the value it shows; `hovered` is the reader's pointer being over the
/// control it belongs to. Neither is read from the `Ui`: every caller here is a
/// self-painted row that already resolved both, and a switch inside a disabled
/// row must not light up under a pointer that cannot move it.
pub(crate) fn paint_switch(ui: &Ui, center: egui::Pos2, on: bool, hovered: bool, clip_rect: Rect) {
    let t = Tokens::get(ui.ctx());
    let rect = Rect::from_center_size(center, vec2(SWITCH_WIDTH, SWITCH_HEIGHT));
    let fill = if on {
        t.color.accent
    } else if hovered {
        t.color.bg_hover
    } else {
        t.color.bg_inset
    };
    let painter = ui
        .painter()
        .with_clip_rect(clip_rect.intersect(ui.clip_rect()));
    painter.rect(
        rect,
        8.5,
        fill,
        Stroke::new(
            1.0,
            if on {
                t.color.accent
            } else {
                t.color.border_strong
            },
        ),
        egui::StrokeKind::Inside,
    );
    let knob_x = if on {
        rect.right() - 7.5
    } else {
        rect.left() + 7.5
    };
    painter.circle_filled(
        egui::pos2(knob_x, rect.center().y),
        5.5,
        if on {
            t.color.accent_ink
        } else {
            t.color.text_dim
        },
    );
}
