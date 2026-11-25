//! The two-net probe: with the Probe tool armed, a drag from one net to
//! another plots the voltage between them, `V(a) - V(b)`, as a differential
//! probe reads it. The press lands on the + net and the release on the -
//! net; a click without a drag probes one net, as it always has.

use egui::{Align2, Id, Painter, PointerButton, Pos2, Response, Stroke, Ui, vec2};

use super::SchematicSymbolContext;
use super::interaction::{probe_difference, probe_point};
use super::parts::CanvasParts;
use super::viewport::Viewport;
use crate::state::Point;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Where the drag that is under way began, as the Probe tool resolves a
/// press there.
#[derive(Clone, Copy, Debug)]
struct Started(Point);

fn started_id() -> Id {
    Id::new("rspice.design.difference-probe")
}

/// Follow a Probe-tool drag and, on its release, probe the voltage between
/// the net it began on and the net it ends on. Escape lets the drag go.
/// Returns whether the drag owns the pointer this frame.
pub(super) fn follow_drag(
    ui: &Ui,
    response: &Response,
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) -> bool {
    let ctx = ui.ctx();
    if response.drag_started_by(PointerButton::Primary) {
        let press = ctx.input(|input| input.pointer.press_origin());
        let Some(at) = press.and_then(|press| probe_point(state, viewport, symbol_context, press))
        else {
            return false;
        };
        ctx.data_mut(|data| data.insert_temp(started_id(), Started(at)));
        return true;
    }
    let Some(Started(from)) = ctx.data(|data| data.get_temp::<Started>(started_id())) else {
        return false;
    };
    if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        ctx.data_mut(|data| data.remove::<Started>(started_id()));
        return true;
    }
    if response.drag_stopped_by(PointerButton::Primary) {
        ctx.data_mut(|data| data.remove::<Started>(started_id()));
        let end = ctx.input(|input| input.pointer.latest_pos());
        if let Some(to) = end.and_then(|end| probe_point(state, viewport, symbol_context, end)) {
            probe_difference(state, from, to, symbol_context);
        }
        return true;
    }
    // A drag the canvas lost ends without a probe.
    if !response.dragged() {
        ctx.data_mut(|data| data.remove::<Started>(started_id()));
        return false;
    }
    true
}

/// The drag under way: a line from where it began to the pointer, the
/// begin marked + and the pointer -, so the reader sees which way the
/// difference is taken before letting go.
pub(super) fn paint(painter: &Painter, viewport: &Viewport) {
    let ctx = painter.ctx();
    let Some(Started(from)) = ctx.data(|data| data.get_temp::<Started>(started_id())) else {
        return;
    };
    let Some(pointer) = ctx.input(|input| {
        input
            .pointer
            .primary_down()
            .then(|| input.pointer.latest_pos())
            .flatten()
    }) else {
        return;
    };
    let accent = Tokens::get(ctx).color.accent;
    let from = viewport.schematic_to_screen(from);
    painter.line_segment([from, pointer], Stroke::new(1.0, accent));
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mark = |at: Pos2, sign: &str| {
        painter.text(
            at + vec2(6.0, -6.0),
            Align2::LEFT_BOTTOM,
            sign,
            font.clone(),
            accent,
        );
    };
    mark(from, "+");
    mark(pointer, "\u{2212}");
}
