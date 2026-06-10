//! The VOLTA design system.
//!
//! A self-contained visual foundation for the application: design tokens,
//! color palettes, embedded fonts, vector icons, and the widget library.
//! Nothing in this module depends on application state — the shell
//! (`crate::shell`) composes these primitives into the IDE.
//!
//! # Structure
//!
//! - [`palette`] — generated color tables (3 directions × dark/light)
//! - [`tokens`] — token resolution ([`tokens::Tokens`]) and metric scales
//! - [`theme`] — user-facing theme selection + egui style mapping
//! - [`fonts`] — embedded IBM Plex faces and weighted families
//! - [`icons`] — stroke-based vector icon set
//! - [`widgets`] — the widget vocabulary (buttons, chips, pills, trees, …)
//! - [`plot`] — the strip-plot engine (axes, traces, markers, cursors)

pub mod fonts;
pub mod icons;
pub mod palette;
pub mod plot;
pub mod theme;
pub mod tokens;
pub mod widgets;

pub use theme::{FontWeight, Theme};
pub use tokens::{Density, Direction, Mode, Tokens};
