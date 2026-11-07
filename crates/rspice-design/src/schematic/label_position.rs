//! Persisted per-instance label placement.

use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

use super::label_layout::{INSTANCE_VALUE_WORLD, LayoutPoint, PlacedText, TextEdge};

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

/// One of the texts a drawing letters an instance's value in, when it
/// letters it in several: QSPICE draws each of a part's attributes as a text
/// of its own, at its own place and size. A source's drawing letters its
/// waveform as written, where RSpice keeps it as parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePiece {
    pub text: String,
    pub position: LabelPosition,
    /// The text's size in hundredths of an instance value's.
    pub size_percent: u16,
    /// For a text that letters more than the value alone, the instance's
    /// value and parameters as they stood when the drawing lettered it
    /// ([`super::value_label::value_and_parameters`]): the text stands only
    /// while the instance still has them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stands_for: Option<String>,
}

impl ValuePiece {
    /// The piece's type size in world units.
    #[must_use]
    pub fn size_world(&self) -> f32 {
        INSTANCE_VALUE_WORLD * f32::from(self.size_percent) / 100.0
    }
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
