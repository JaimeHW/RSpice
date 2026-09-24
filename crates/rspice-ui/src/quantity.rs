//! Unit-safe presentation and interactive quantity parsing.
//!
//! Interactive quantity policy remains distinct from deck parsing and from
//! PDK/layout database units. The `spice_value` submodule uses the core deck
//! suffix table for authored simulation fields; display values never rewrite
//! stored results.

pub(crate) mod engineering;
mod format;
mod layout;
mod locale;
mod parse;
mod preferences;
pub(crate) mod spice_value;
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
