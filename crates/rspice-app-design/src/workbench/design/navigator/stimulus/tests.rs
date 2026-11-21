//! What the library browser and the definition inspector are held to.
//!
//! Both docks are gated here because they are read together: a count in the
//! browser's footer and a count in the inspector's Adopters heading are the
//! same walk of the same design, and a test that only ever opened one of them
//! would not notice the two disagreeing.

/// The dock's own minimum, at the fit the workspace is held to.
pub const NAVIGATOR: egui::Vec2 = egui::vec2(228.0, 640.0);
