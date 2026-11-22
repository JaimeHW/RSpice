//! Persisted per-instance label placement.

use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

use super::component::Component;
use super::label_layout::{INSTANCE_VALUE_WORLD, LayoutPoint, PlacedText, TextEdge};

/// One of the two texts every instance letters for itself. Each belongs to
/// its part: a click on it takes the part, and a drag moves the text alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstanceText {
    /// The reference designator: `R1`.
    Name,
    /// The value: `10k`.
    Value,
}

impl InstanceText {
    /// Where `component` letters this text.
    #[must_use]
    pub const fn position(self, component: &Component) -> LabelPosition {
        match self {
            Self::Name => component.name_label_pos,
            Self::Value => component.value_label_pos,
        }
    }

    /// Where `component` letters this text, to change.
    pub const fn position_mut(self, component: &mut Component) -> &mut LabelPosition {
        match self {
            Self::Name => &mut component.name_label_pos,
            Self::Value => &mut component.value_label_pos,
        }
    }
}

/// Where an instance letters its name or its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LabelPosition {
    /// Clear of the turned body, where the shared layout puts it.
    #[default]
    Auto,
    /// Where a drawing put the text: its anchor's offset from the
    /// instance's position in world units, which ends of the run sit on
    /// that anchor, and whether the run reads upward rather than upright
    /// ([`PlacedText::upward`]).
    Placed {
        offset: Point,
        horizontal: TextEdge,
        vertical: TextEdge,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        upward: bool,
        /// The text's size in hundredths of an instance's ordinary lettering
        /// of the same kind: a drawing's text keeps the drawing's size.
        #[serde(
            default = "ordinary_size_percent",
            skip_serializing_if = "is_ordinary_size_percent"
        )]
        size_percent: u16,
    },
}

const fn ordinary_size_percent() -> u16 {
    100
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's skip_serializing_if hands the field by reference."
)]
const fn is_ordinary_size_percent(percent: &u16) -> bool {
    *percent == ordinary_size_percent()
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
    /// The text's size in hundredths of an instance's ordinary lettering.
    #[must_use]
    pub const fn size_percent(self) -> u16 {
        match self {
            Self::Auto => ordinary_size_percent(),
            Self::Placed { size_percent, .. } => size_percent,
        }
    }

    /// The text's type size in world units, where `ordinary` is the size an
    /// instance letters this kind of text at.
    #[must_use]
    pub fn size_world(self, ordinary: f32) -> f32 {
        ordinary * f32::from(self.size_percent()) / 100.0
    }

    /// The text where `placed` (world units) stands now, moved by `delta`, for
    /// an instance at `origin`: what a drag of the text leaves. Text that
    /// stood where the layout put it keeps that spot as its starting point,
    /// so the first step moves it from where the reader saw it.
    #[must_use]
    pub fn moved(self, placed: PlacedText, origin: Point, delta: Point) -> Self {
        let offset = match self {
            Self::Placed { offset, .. } => offset,
            Self::Auto => Point::new(
                (placed.anchor.x - origin.x as f32).round() as i32,
                (placed.anchor.y - origin.y as f32).round() as i32,
            ),
        };
        Self::Placed {
            offset: Point::new(offset.x + delta.x, offset.y + delta.y),
            horizontal: placed.horizontal,
            vertical: placed.vertical,
            upward: placed.upward,
            size_percent: self.size_percent(),
        }
    }

    /// The placement once its part turns or flips, where `turn` takes an
    /// offset from the part's position as the part's own turn takes it. The
    /// anchor goes where the turn takes it, and the text, which stays
    /// upright, hangs off the anchor on the side the turn takes its old side
    /// to, so it keeps its place beside the body. Text the layout places
    /// follows the turned body by itself.
    #[must_use]
    pub fn turned(self, turn: impl Fn(Point) -> Point) -> Self {
        let Self::Placed {
            offset,
            horizontal,
            vertical,
            upward,
            size_percent,
        } = self
        else {
            return self;
        };
        // The side of the anchor the text hangs on, as a screen direction.
        // Read upward, a run's `horizontal` edge is along its reading, which
        // runs up the screen, and its `vertical` edge across it.
        let side = if upward {
            Point::new(edge_sign(vertical), -edge_sign(horizontal))
        } else {
            Point::new(edge_sign(horizontal), edge_sign(vertical))
        };
        let side = turn(side);
        let (horizontal, vertical) = if upward {
            (sign_edge(-side.y), sign_edge(side.x))
        } else {
            (sign_edge(side.x), sign_edge(side.y))
        };
        Self::Placed {
            offset: turn(offset),
            horizontal,
            vertical,
            upward,
            size_percent,
        }
    }

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
                upward,
                ..
            } => Some(PlacedText {
                anchor: LayoutPoint::new(
                    origin.x + offset.x as f32 * scale,
                    origin.y + offset.y as f32 * scale,
                ),
                horizontal,
                vertical,
                upward,
            }),
        }
    }
}

/// Which way from its anchor a run hangs along one axis: toward greater
/// coordinates when its start edge sits on the anchor.
const fn edge_sign(edge: TextEdge) -> i32 {
    match edge {
        TextEdge::Start => 1,
        TextEdge::Center => 0,
        TextEdge::End => -1,
    }
}

/// The edge that sits on the anchor of a run hanging the way `sign` says.
const fn sign_edge(sign: i32) -> TextEdge {
    match sign.signum() {
        1 => TextEdge::Start,
        -1 => TextEdge::End,
        _ => TextEdge::Center,
    }
}
