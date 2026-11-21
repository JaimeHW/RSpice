//! How a reader styled one trace beyond its colour: the width and dash of
//! its line, the symbol on its samples and how its samples are joined, as
//! ViVA's and EZwave's trace properties offer them. A view keeps the style
//! with the trace, so it survives a save and a re-run; each part left unset
//! draws as the trace's place in the palette draws it.

use serde::{Deserialize, Serialize};

/// How a reader styled one trace beyond its colour.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<TraceWidth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dash: Option<TraceDash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<TraceSymbol>,
    #[serde(default, skip_serializing_if = "TraceDraw::is_line")]
    pub draw: TraceDraw,
}

impl TraceStyle {
    /// Whether the reader styled nothing.
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    /// This style with each part that differs between `from` and `to` set as
    /// `to` sets it: one edit of a trace's style made to another trace,
    /// which keeps the parts of its own the edit did not touch.
    pub fn with_changes(self, from: Self, to: Self) -> Self {
        Self {
            width: if from.width == to.width {
                self.width
            } else {
                to.width
            },
            dash: if from.dash == to.dash {
                self.dash
            } else {
                to.dash
            },
            symbol: if from.symbol == to.symbol {
                self.symbol
            } else {
                to.symbol
            },
            draw: if from.draw == to.draw {
                self.draw
            } else {
                to.draw
            },
        }
    }
}

/// The width of a trace's line, in the steps Virtuoso offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceWidth {
    Fine,
    Medium,
    Thick,
    ExtraThick,
}

impl TraceWidth {
    pub const ALL: [Self; 4] = [Self::Fine, Self::Medium, Self::Thick, Self::ExtraThick];

    /// The line's width, in points.
    pub const fn points(self) -> f32 {
        match self {
            Self::Fine => 1.0,
            Self::Medium => 1.6,
            Self::Thick => 2.4,
            Self::ExtraThick => 3.4,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Fine => "Fine",
            Self::Medium => "Medium",
            Self::Thick => "Thick",
            Self::ExtraThick => "Extra thick",
        }
    }
}

/// The dash a trace's line is drawn in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceDash {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

impl TraceDash {
    pub const ALL: [Self; 4] = [Self::Solid, Self::Dashed, Self::Dotted, Self::DashDot];

    /// The dash and gap lengths, in points; `None` is a solid line.
    pub const fn pattern(self) -> Option<(f32, f32)> {
        match self {
            Self::Solid => None,
            Self::Dashed => Some((7.0, 4.0)),
            Self::Dotted => Some((2.0, 3.0)),
            Self::DashDot => Some((10.0, 3.0)),
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Solid => "Solid",
            Self::Dashed => "Dashed",
            Self::Dotted => "Dotted",
            Self::DashDot => "Dash-dot",
        }
    }
}

/// The symbol a trace marks its samples with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceSymbol {
    /// No symbol, even where colour-safe traces would give one.
    None,
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    Plus,
}

impl TraceSymbol {
    pub const ALL: [Self; 7] = [
        Self::None,
        Self::Circle,
        Self::Square,
        Self::Diamond,
        Self::Triangle,
        Self::Cross,
        Self::Plus,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Circle => "Circle",
            Self::Square => "Square",
            Self::Diamond => "Diamond",
            Self::Triangle => "Triangle",
            Self::Cross => "Cross",
            Self::Plus => "Plus",
        }
    }
}

/// How a trace joins its samples.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceDraw {
    /// A line through the samples.
    #[default]
    Line,
    /// Each sample a point, unjoined.
    Points,
    /// Each sample held until the next.
    Steps,
    /// A stem from zero to each sample.
    Stems,
}

impl TraceDraw {
    pub const ALL: [Self; 4] = [Self::Line, Self::Points, Self::Steps, Self::Stems];

    pub fn is_line(&self) -> bool {
        *self == Self::Line
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Line => "Line",
            Self::Points => "Points",
            Self::Steps => "Steps",
            Self::Stems => "Stems",
        }
    }
}
