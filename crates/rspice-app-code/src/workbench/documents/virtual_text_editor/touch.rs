//! The editor under a finger.
//!
//! A finger dragging scrolls the source, a tap places the caret, and a double
//! tap or a long press selects a word. A selection a finger made carries two
//! handles, one under each end, that a finger drags to adjust it. Whether a
//! gesture is a finger is decided per press, so a mouse on the same device
//! still selects by dragging.

use super::*;

/// Radius of a selection handle's knob.
const HANDLE_RADIUS: f32 = 6.0;
/// Gap between the bottom of the selected row and a knob.
const HANDLE_DROP: f32 = 2.0;

#[derive(Clone, Copy, Default)]
pub(super) struct TouchGesture {
    /// The press in progress, or the last one, was a finger.
    touch_press: bool,
    /// The selection was made by a finger and shows its handles.
    handles: bool,
}

impl TouchGesture {
    pub(super) fn begin_press(&mut self, touch: bool) {
        self.touch_press = touch;
        if !touch {
            self.handles = false;
        }
    }

    pub(super) const fn is_touch(self) -> bool {
        self.touch_press
    }

    pub(super) fn show_handles(&mut self, shown: bool) {
        self.handles = shown;
    }
}

/// Where the two handles hang, in screen coordinates: the bottom of the row
/// at the start and at the end of the primary selection. `None` when there
/// is no selection to adjust, or an end is folded away.
fn handle_roots(
    session: &VirtualEditorSession,
    origin: egui::Pos2,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) -> Option<[egui::Pos2; 2]> {
    if !session.touch.handles {
        return None;
    }
    let range = session.model.selections().last()?.range();
    if range.is_empty() {
        return None;
    }
    let root = |index: usize| {
        let (line, column) = session.model.line_column_for_char(index);
        let row = session.model.visible_row_for_logical_line(line)?;
        Some(egui::pos2(
            origin.x + style.gutter_width + style.code_left_padding + column as f32 * char_width,
            origin.y + style.top_padding + (row + 1) as f32 * style.line_height,
        ))
    };
    Some([root(range.start)?, root(range.end)?])
}

fn knob_center(root: egui::Pos2) -> egui::Pos2 {
    root + Vec2::new(0.0, HANDLE_DROP + HANDLE_RADIUS)
}

/// The selection a dragged handle asks for, if one is being dragged. The
/// finger holds the knob below the text, so the end it moves is the row
/// above the knob, where the reader sees it.
pub(super) fn drag_selection_handles(
    ui: &Ui,
    editor_id: egui::Id,
    session: &mut VirtualEditorSession,
    focused: bool,
    origin: egui::Pos2,
    viewport: egui::Rect,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) -> Option<TextSelection> {
    if !focused {
        return None;
    }
    let roots = handle_roots(session, origin, style, char_width)?;
    let range = session.model.selections().last()?.range();
    let mut dragged = None;
    for (end, root) in roots.into_iter().enumerate() {
        let knob = knob_center(root);
        let response = ui.interact(
            egui::Rect::from_center_size(knob, Vec2::splat(crate::ui::tokens::TOUCH_TARGET)),
            editor_id.with(("selection-handle", end)),
            Sense::drag(),
        );
        if response.dragged()
            && let Some(pointer) = response.interact_pointer_pos()
        {
            let target = pointer - Vec2::new(0.0, knob.y - root.y + style.line_height * 0.5);
            let (line, column) =
                hit_test(&session.model, target, origin, viewport, style, char_width);
            let fixed = if end == 0 { range.end } else { range.start };
            dragged = Some(TextSelection {
                anchor: fixed,
                cursor: session.model.char_at_line_column(line, column),
            });
        }
    }
    dragged
}

pub(super) fn paint_selection_handles(
    painter: &egui::Painter,
    session: &VirtualEditorSession,
    focused: bool,
    origin: egui::Pos2,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) {
    if !focused {
        return;
    }
    let Some(roots) = handle_roots(session, origin, style, char_width) else {
        return;
    };
    for root in roots {
        let knob = knob_center(root);
        painter.vline(
            root.x,
            (root.y - style.line_height)..=knob.y,
            Stroke::new(1.5, style.accent),
        );
        painter.circle_filled(knob, HANDLE_RADIUS, style.accent);
    }
}
