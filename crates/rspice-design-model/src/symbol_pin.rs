//! Shared electrical and geometric pin classifications for authored and model-bound symbols.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolElectricalType {
    Analog,
    Logic,
    Power,
    Ground,
    Passive,
}

impl SymbolElectricalType {
    pub const ALL: [Self; 5] = [
        Self::Analog,
        Self::Logic,
        Self::Power,
        Self::Ground,
        Self::Passive,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Analog => "analog",
            Self::Logic => "logic",
            Self::Power => "power",
            Self::Ground => "ground",
            Self::Passive => "passive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolPinSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl SymbolPinSide {
    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Top, Self::Bottom];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    /// The side a pin is on once the symbol turns a quarter clockwise on
    /// screen, where y grows downward: a pin on the left comes up on top.
    pub const fn turned_clockwise(self) -> Self {
        match self {
            Self::Left => Self::Top,
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
        }
    }

    /// The side a pin is on once the symbol turns a quarter counter-clockwise.
    pub const fn turned_counter_clockwise(self) -> Self {
        match self {
            Self::Left => Self::Bottom,
            Self::Bottom => Self::Right,
            Self::Right => Self::Top,
            Self::Top => Self::Left,
        }
    }

    /// The side a pin is on once the symbol is mirrored left to right.
    pub const fn mirrored_horizontally(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top | Self::Bottom => self,
        }
    }

    /// The side a pin is on once the symbol is mirrored top to bottom.
    pub const fn mirrored_vertically(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left | Self::Right => self,
        }
    }
}

/// The mark a pin draws where its lead meets the body.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolPinStyle {
    /// The lead alone.
    #[default]
    Plain,
    /// A bubble: the signal is active low.
    Inverted,
    /// A wedge inside the body: the input is edge-triggered.
    Clock,
    /// Both the bubble and the wedge.
    InvertedClock,
}

impl SymbolPinStyle {
    pub const ALL: [Self; 4] = [
        Self::Plain,
        Self::Inverted,
        Self::Clock,
        Self::InvertedClock,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Inverted => "inverted",
            Self::Clock => "clock",
            Self::InvertedClock => "inverted clock",
        }
    }

    pub const fn is_plain(&self) -> bool {
        matches!(self, Self::Plain)
    }

    /// Whether the pin draws a bubble at the body end of its lead.
    pub const fn bubble(self) -> bool {
        matches!(self, Self::Inverted | Self::InvertedClock)
    }

    /// Whether the pin draws a clock wedge inside the body.
    pub const fn wedge(self) -> bool {
        matches!(self, Self::Clock | Self::InvertedClock)
    }
}
