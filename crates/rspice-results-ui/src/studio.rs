//! Visualization Studio presentation over caller-owned selections and source summaries.

pub mod chrome;
pub mod dock;
pub mod inspector;
pub mod sections;
pub mod stage;
pub mod viewers;
pub mod widgets;

pub const COMPACT_BREAKPOINT: f32 = 820.0;
pub const TOUCH_DOCK_HEIGHT: f32 = 52.0;
pub const PANEL_HEADING_HEIGHT: f32 = 29.0;

pub const fn bar_content_height(target_height: f32, vertical_margin: f32) -> f32 {
    target_height - vertical_margin * 2.0
}
