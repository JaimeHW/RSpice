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
pub fn capture_root_viewport(ctx: &Context, rect: Rect) {
    ctx.data_mut(|data| data.insert_temp(Id::new(ROOT_VIEWPORT_RECT_ID), rect));
}

/// Return the root paint viewport captured for this frame.
///
/// The fallback keeps isolated widget tests and previews deterministic when
/// they intentionally render without the application frame boundary.
pub fn root_viewport_rect(ctx: &Context) -> Rect {
    ctx.data(|data| data.get_temp(Id::new(ROOT_VIEWPORT_RECT_ID)))
        .unwrap_or_else(|| ctx.content_rect())
}

pub fn root_viewport_width(ctx: &Context) -> f32 {
    root_viewport_rect(ctx).width()
}
