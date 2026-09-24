//! Unit-safe presentation and interactive quantity parsing.
//!
//! Portable policy comes from `rspice-app-types`. Platform locale discovery,
//! authored-deck parsing, and result-import units stay with their consumers.

mod locale;
pub(crate) use rspice_simulation_contract::spice_value;
pub(crate) mod unit;

pub use locale::platform_number_locale;
pub(crate) use rspice_app_types::quantity::engineering;
pub use rspice_app_types::quantity::{
    AngleDisplay, EngineeringPrecision, LayoutDatabaseUnit, QuantityInputKind,
    QuantityPresentationPolicy, UiNumberLocale, UnitsPreferences, format_engineering_value,
    format_engineering_value_with, parse_engineering_value, parse_ui_quantity,
};
