//! Portable display-quantity policy and interactive quantity parsing.
//!
//! This policy is separate from the simulator's deck lexer and from PDK
//! database-unit authority. It never rewrites stored result values.

pub mod engineering;
mod format;
mod layout;
mod parse;
mod preferences;

pub use engineering::{
    EngineeringPrecision, format_engineering_value, format_engineering_value_with,
    parse_engineering_value,
};
pub use layout::LayoutDatabaseUnit;
pub use parse::{QuantityInputKind, UiNumberLocale, parse_ui_quantity};
pub use preferences::{
    AngleDisplay, CopiedValueFormat, DecimalSeparatorInput, EngineeringSuffixPolicy,
    FrequencyDisplay, LayoutCoordinateDisplay, QuantityPresentationPolicy, TemperatureDisplay,
    TimeFrequencyInput, UnitSystem, UnitsPreferences,
};
