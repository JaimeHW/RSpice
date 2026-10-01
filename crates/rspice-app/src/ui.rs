//! Application access to shared UI primitives and result plotting.

pub(crate) mod plot;

#[cfg(test)]
pub(crate) use rspice_ui_kit::raster;
#[cfg(test)]
pub use rspice_ui_kit::tokens::Direction;
pub use rspice_ui_kit::{Density, EngineeringCanvasTheme, Mode, Theme};
pub(crate) use rspice_ui_kit::{
    accessibility, icons, input, palette, theme, tokens, viewport, widgets,
};
