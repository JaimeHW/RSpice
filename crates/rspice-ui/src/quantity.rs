//! Unit-safe presentation and interactive quantity parsing.
//!
//! This domain is deliberately separate from the SPICE deck lexer and from
//! PDK/layout database units. It normalizes external result units on import and
//! presents immutable engineering values without rewriting stored results.

pub(crate) mod engineering;
mod format;
mod layout;
mod locale;
mod parse;
mod preferences;
pub(crate) mod unit;

pub use engineering::{
    EngineeringPrecision, format_engineering_value, format_engineering_value_with,
    parse_engineering_value,
};
pub use layout::LayoutDatabaseUnit;
pub use locale::platform_number_locale;
pub use parse::{QuantityInputKind, UiNumberLocale, parse_ui_quantity};
pub use preferences::{
    AngleDisplay, CopiedValueFormat, DecimalSeparatorInput, EngineeringSuffixPolicy,
    FrequencyDisplay, LayoutCoordinateDisplay, QuantityPresentationPolicy, TemperatureDisplay,
    TimeFrequencyInput, UnitSystem, UnitsPreferences,
};
