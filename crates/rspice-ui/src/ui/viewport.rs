//! Stable root-viewport geometry for viewport-scoped responsive contracts.
//!
//! egui's panel layout progressively reduces the context content rectangle.
//! Dialogs render after the workbench panels, so consulting `Context` directly
//! from a dialog can accidentally treat the remaining center workspace as the
//! host viewport. Capture the root paint rectangle once, before chrome and
//! docks are allocated, and use that immutable per-frame value wherever a
//! mockup breakpoint is explicitly viewport-scoped.

use egui::{Context, Id, Rect};

const ROOT_VIEWPORT_RECT_ID: &str = "rspice/root-viewport-rect";

/// Capture the root paint viewport before any top-level panel is allocated.
pub(crate) fn capture_root_viewport(ctx: &Context, rect: Rect) {
    ctx.data_mut(|data| data.insert_temp(Id::new(ROOT_VIEWPORT_RECT_ID), rect));
}

/// Return the root paint viewport captured for this frame.
///
/// The fallback keeps isolated widget tests and previews deterministic when
/// they intentionally render without the application frame boundary.
pub(crate) fn root_viewport_rect(ctx: &Context) -> Rect {
    ctx.data(|data| data.get_temp(Id::new(ROOT_VIEWPORT_RECT_ID)))
        .unwrap_or_else(|| ctx.content_rect())
}

pub(crate) fn root_viewport_width(ctx: &Context) -> f32 {
    root_viewport_rect(ctx).width()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captured_root_viewport_is_explicit_and_replaceable_each_frame() {
        let ctx = Context::default();
        let desktop = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 720.0));
        capture_root_viewport(&ctx, desktop);
        assert_eq!(root_viewport_rect(&ctx), desktop);
        assert_eq!(root_viewport_width(&ctx), 1280.0);

        let compact = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(760.0, 720.0));
        capture_root_viewport(&ctx, compact);
        assert_eq!(root_viewport_rect(&ctx), compact);
        assert_eq!(root_viewport_width(&ctx), 760.0);
    }
}
