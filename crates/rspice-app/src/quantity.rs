//! Unit-safe presentation and interactive quantity parsing.
//!
//! Portable policy comes from `rspice-app-types`. Platform locale discovery,
//! authored-deck parsing, and result-import units stay with their consumers.

mod locale;
pub(crate) use rspice_simulation_contract::spice_value;

pub use locale::platform_number_locale;
#[cfg(test)]
pub use rspice_app_types::quantity::AngleDisplay;
pub(crate) use rspice_app_types::quantity::engineering;
pub use rspice_app_types::quantity::{
    EngineeringPrecision, LayoutDatabaseUnit, QuantityInputKind, QuantityPresentationPolicy,
    UiNumberLocale, UnitsPreferences, format_elapsed, format_engineering_value,
    parse_engineering_value, parse_ui_quantity,
};

/// A temperature stated in degrees Celsius, as a number in the unit
/// Temperature display chose: whole values stay whole, at most two decimals,
/// and a negative value takes a true minus sign.
pub(crate) fn temperature_number(celsius: f64, policy: QuantityPresentationPolicy) -> String {
    let (scale, offset, _) = policy.celsius_display_transform();
    let text = format!("{:.2}", celsius * scale + offset);
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" {
        "0".to_owned()
    } else {
        text.replace('-', "\u{2212}")
    }
}

/// A Celsius temperature with its display unit: `27 °C`, `300.15 K`.
pub(crate) fn temperature_label(celsius: f64, policy: QuantityPresentationPolicy) -> String {
    temperature_list_label(&[celsius], policy)
}

/// Celsius temperatures as one list in the display unit: `−40, 27, 125 °C`.
pub(crate) fn temperature_list_label(
    celsius: &[f64],
    policy: QuantityPresentationPolicy,
) -> String {
    let numbers = celsius
        .iter()
        .map(|value| temperature_number(*value, policy))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{numbers} {}", policy.celsius_display_transform().2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_app_types::quantity::TemperatureDisplay;

    /// A Celsius temperature reads in the chosen unit, with nothing after
    /// the point that is not there.
    #[test]
    fn temperatures_read_in_the_chosen_unit() {
        let mut policy = QuantityPresentationPolicy::default();
        policy.temperature_display = TemperatureDisplay::Celsius;
        assert_eq!(
            temperature_list_label(&[-40.0, 27.0], policy),
            "\u{2212}40, 27 °C"
        );
        policy.temperature_display = TemperatureDisplay::Kelvin;
        assert_eq!(temperature_label(27.0, policy), "300.15 K");
        policy.temperature_display = TemperatureDisplay::Fahrenheit;
        assert_eq!(temperature_label(27.0, policy), "80.6 °F");
        assert_eq!(temperature_label(-17.7777778, policy), "0 °F");
    }
}
