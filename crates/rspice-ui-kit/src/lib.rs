//! Reusable RSpice styling, widgets, and input handling.
//!
//! Widgets accept display data and return interaction outcomes. Application
//! composition and feature availability remain with their callers.

pub mod accessibility;
pub mod fonts;
pub mod icons;
pub mod input;
pub mod palette;
#[cfg(any(test, feature = "test-support"))]
pub mod raster;
pub mod theme;
pub mod tokens;
pub mod viewport;
pub mod widgets;

pub use theme::{EngineeringCanvasTheme, Theme};
pub use tokens::{Density, Mode};
