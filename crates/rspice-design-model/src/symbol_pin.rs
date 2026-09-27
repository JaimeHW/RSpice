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
}
