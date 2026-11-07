//! Persisted per-instance label placement.

use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

use super::label_layout::{LayoutPoint, PlacedText, TextEdge};

/// Where an instance letters its name or its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LabelPosition {
    /// Clear of the turned body, where the shared layout puts it.
    #[default]
    Auto,
    /// Where a drawing put the text: its anchor's offset from the
    /// instance's position in world units, and which ends of the upright run
    /// sit on that anchor.
    Placed {
        offset: Point,
        horizontal: TextEdge,
        vertical: TextEdge,
    },
}

impl LabelPosition {
    /// The run's anchor and hang for an instance at `origin`, with `scale`
    /// the size of one world unit in the caller's space, when a drawing
    /// placed the text.
    #[must_use]
    pub fn placed_text(self, origin: LayoutPoint, scale: f32) -> Option<PlacedText> {
        match self {
            Self::Auto => None,
            Self::Placed {
                offset,
                horizontal,
                vertical,
            } => Some(PlacedText {
                anchor: LayoutPoint::new(
                    origin.x + offset.x as f32 * scale,
                    origin.y + offset.y as f32 * scale,
                ),
                horizontal,
                vertical,
            }),
        }
    }
}
