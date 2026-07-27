//! Unit-safe presentation and interactive quantity parsing.
//!
//! This domain is deliberately separate from the SPICE deck lexer and from
//! PDK/layout database units. It converts immutable SI values at the UI edge;
//! it never changes stored engineering data.

mod format;
mod layout;
mod locale;
mod parse;
mod preferences;

pub use layout::{LayoutDatabaseUnit, LayoutDatabaseUnitError};
pub use locale::platform_number_locale;
pub use parse::{QuantityInputError, QuantityInputKind, UiNumberLocale, parse_ui_quantity};
pub use preferences::{
    AngleDisplay, CopiedValueFormat, DecimalSeparatorInput, EngineeringSuffixPolicy,
    FrequencyDisplay, LayoutCoordinateDisplay, QuantityPresentationPolicy, TemperatureDisplay,
    TimeFrequencyInput, UnitSystem, UnitsPreferences,
};
