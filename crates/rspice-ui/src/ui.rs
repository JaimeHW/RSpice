//! The RSpice design system.
//!
//! A self-contained visual foundation for the application: design tokens,
//! color palettes, embedded fonts, vector icons, and the widget library.
//! Nothing in this module depends on application state — the workbench
//! (`crate::workbench`) composes these primitives into the IDE.
//!
//! # Structure
//!
//! - [`palette`] — mockup-governed Instrument dark/light semantic colors
//! - [`tokens`] — token resolution ([`tokens::Tokens`]) and metric scales
//! - [`theme`] — user-facing theme selection + egui style mapping
//! - [`fonts`] — embedded IBM Plex faces and weighted families
//! - [`icons`] — stroke-based vector icon set
//! - [`widgets`] — the widget vocabulary (buttons, chips, pills, trees, …)
//! - [`plot`] — the strip-plot engine (axes, traces, markers, cursors)

pub(crate) mod accessibility;
pub(crate) mod fonts;
pub(crate) mod icons;
pub(crate) mod palette;
pub(crate) mod plot;
// Test-only, and at this layer because it depends on nothing above it: a
// rasterizer that reached for application state could not be used to render
// application state.
#[cfg(test)]
pub(crate) mod raster;
pub(crate) mod theme;
pub(crate) mod tokens;
pub(crate) mod viewport;
pub(crate) mod widgets;

pub use theme::{EngineeringCanvasTheme, Theme};
#[cfg(test)]
pub use tokens::Direction;
pub use tokens::{Density, Mode};
