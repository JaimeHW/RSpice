use std::fmt;

use super::{
    DecimalSeparatorInput, EngineeringSuffixPolicy, QuantityPresentationPolicy, TimeFrequencyInput,
};

/// Quantity expected by one typed UI field. Returned values use SI base units:
/// seconds, hertz, kelvin, or radians.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityInputKind {
    /// Unit is supplied by the surrounding property schema. The field accepts
    /// only a number plus an engineering multiplier.
    EngineeringScalar,
    Time,
    Frequency,
    Temperature,
    /// A temperature interval. Unlike [`Self::Temperature`], no absolute
    /// scale offset is applied (1 K == 1 °C; Fahrenheit intervals scale).
    TemperatureDelta,
    Angle,
}

/// Locale information supplied by the platform boundary. No locale is
/// guessed or embedded in the engineering domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UiNumberLocale {
    pub decimal_separator: Option<char>,
}

/// Parse one interactive quantity field into its SI base value.
///
/// This parser is intentionally unrelated to the SPICE deck parser. It never
/// changes a deck dialect and never infers layout database units.
pub fn parse_ui_quantity(
    text: &str,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<f64, QuantityInputError> {
    let source = normalize_decimal_separator(text.trim(), policy, locale)?;
    if source.is_empty() {
        return Err(QuantityInputError::Empty);
    }
    if matches!(
        source.to_ascii_lowercase().as_str(),
        "nan" | "+nan" | "-nan" | "inf" | "+inf" | "-inf" | "infinity" | "+infinity" | "-infinity"
    ) {
        return Err(QuantityInputError::NonFinite);
    }
    let split = numeric_prefix_len(&source);
    if split == 0 {
        return Err(QuantityInputError::InvalidNumber(source));
    }
    let (number, suffix) = source.split_at(split);
    let value = number
        .parse::<f64>()
        .map_err(|_| QuantityInputError::InvalidNumber(number.to_owned()))?;
    if !value.is_finite() {
        return Err(QuantityInputError::NonFinite);
    }
    let suffix = suffix.trim();

    let si = match kind {
        QuantityInputKind::EngineeringScalar => {
            value * parse_prefix(suffix, policy.engineering_suffixes, false)?
        }
        QuantityInputKind::Time => parse_time_or_frequency(value, suffix, "s", policy)?,
        QuantityInputKind::Frequency => {
            if let Some(prefix) = suffix.strip_suffix("rad/s") {
                value * parse_prefix(prefix, policy.engineering_suffixes, true)?
                    / std::f64::consts::TAU
            } else {
                parse_time_or_frequency(value, suffix, "Hz", policy)?
            }
        }
        QuantityInputKind::Temperature => parse_temperature(value, suffix)?,
        QuantityInputKind::TemperatureDelta => parse_temperature_delta(value, suffix)?,
        QuantityInputKind::Angle => parse_angle(value, suffix)?,
    };

    if !si.is_finite() {
        return Err(QuantityInputError::NonFinite);
    }
    Ok(si)
}

fn normalize_decimal_separator(
    text: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<String, QuantityInputError> {
    let decimal = match policy.decimal_separator_input {
        DecimalSeparatorInput::PeriodEverywhere => '.',
        DecimalSeparatorInput::LocaleAwareUiPortableFiles => {
            locale.decimal_separator.unwrap_or('.')
        }
    };
    if decimal != '.' && decimal != ',' {
        return Err(QuantityInputError::UnsupportedDecimalSeparator(decimal));
    }
    if decimal == ',' {
        if text.contains('.') && text.contains(',') {
            return Err(QuantityInputError::AmbiguousDecimalSeparator);
        }
        Ok(text.replace(',', "."))
    } else if text.contains(',') {
        Err(QuantityInputError::AmbiguousDecimalSeparator)
    } else {
        Ok(text.to_owned())
    }
}

fn numeric_prefix_len(source: &str) -> usize {
    let bytes = source.as_bytes();
    let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let mut digits = 0;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
        digits += 1;
    }
    if bytes.get(index) == Some(&b'.') {
        index += 1;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0;
    }
    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        let exponent_start = index;
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let mut exponent_digits = 0;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
            exponent_digits += 1;
        }
        if exponent_digits == 0 {
            return exponent_start;
        }
    }
    index
}

fn parse_time_or_frequency(
    value: f64,
    suffix: &str,
    unit: &'static str,
    policy: QuantityPresentationPolicy,
) -> Result<f64, QuantityInputError> {
    if suffix.is_empty() {
        return if policy.time_frequency_input == TimeFrequencyInput::InferFromFieldQuantity {
            Ok(value)
        } else {
            Err(QuantityInputError::MissingUnit(unit))
        };
    }
    let prefix = suffix
        .strip_suffix(unit)
        .ok_or_else(|| QuantityInputError::UnexpectedUnit(suffix.to_owned()))?;
    Ok(value * parse_prefix(prefix, policy.engineering_suffixes, true)?)
}

fn parse_prefix(
    prefix: &str,
    policy: EngineeringSuffixPolicy,
    explicit_unit: bool,
) -> Result<f64, QuantityInputError> {
    let multiplier = match policy {
        EngineeringSuffixPolicy::StrictRspice => match prefix {
            "" => 1.0,
            "T" => 1e12,
            "G" => 1e9,
            // M is unambiguous when an explicit SI unit follows; a bare mega
            // multiplier uses the documented `Meg` spelling.
            "M" if explicit_unit => 1e6,
            "Meg" => 1e6,
            "k" => 1e3,
            "m" => 1e-3,
            "u" | "µ" => 1e-6,
            "n" => 1e-9,
            "p" => 1e-12,
            "f" => 1e-15,
            "a" => 1e-18,
            "mil" => 25.4e-6,
            _ => {
                return Err(QuantityInputError::UnknownEngineeringPrefix(
                    prefix.to_owned(),
                ));
            }
        },
        EngineeringSuffixPolicy::ClassicSpiceCompatibility => {
            let folded = prefix.to_ascii_lowercase();
            match folded.as_str() {
                "" => 1.0,
                "t" => 1e12,
                "g" => 1e9,
                "meg" => 1e6,
                "k" => 1e3,
                "m" => 1e-3,
                "u" => 1e-6,
                "n" => 1e-9,
                "p" => 1e-12,
                "f" => 1e-15,
                "a" => 1e-18,
                "mil" => 25.4e-6,
                _ if prefix == "µ" => 1e-6,
                _ => {
                    return Err(QuantityInputError::UnknownEngineeringPrefix(
                        prefix.to_owned(),
                    ));
                }
            }
        }
    };
    Ok(multiplier)
}

fn parse_temperature(value: f64, suffix: &str) -> Result<f64, QuantityInputError> {
    let kelvin = match suffix {
        "K" => value,
        "C" | "°C" => value + 273.15,
        "F" | "°F" => (value - 32.0) / 1.8 + 273.15,
        "" => return Err(QuantityInputError::MissingUnit("K, °C, or °F")),
        _ => return Err(QuantityInputError::UnexpectedUnit(suffix.to_owned())),
    };
    if kelvin < -1e-12 {
        Err(QuantityInputError::BelowAbsoluteZero)
    } else {
        Ok(kelvin.max(0.0))
    }
}

fn parse_temperature_delta(value: f64, suffix: &str) -> Result<f64, QuantityInputError> {
    match suffix {
        "K" | "C" | "°C" => Ok(value),
        "F" | "°F" => Ok(value / 1.8),
        "" => Err(QuantityInputError::MissingUnit("K, °C, or °F")),
        _ => Err(QuantityInputError::UnexpectedUnit(suffix.to_owned())),
    }
}

fn parse_angle(value: f64, suffix: &str) -> Result<f64, QuantityInputError> {
    match suffix {
        "rad" => Ok(value),
        "deg" | "°" => Ok(value.to_radians()),
        "" => Err(QuantityInputError::MissingUnit("rad or °")),
        _ => Err(QuantityInputError::UnexpectedUnit(suffix.to_owned())),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityInputError {
    Empty,
    InvalidNumber(String),
    NonFinite,
    MissingUnit(&'static str),
    UnexpectedUnit(String),
    UnknownEngineeringPrefix(String),
    AmbiguousDecimalSeparator,
    UnsupportedDecimalSeparator(char),
    BelowAbsoluteZero,
}

impl fmt::Display for QuantityInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("value is empty"),
            Self::InvalidNumber(value) => write!(f, "invalid number: {value}"),
            Self::NonFinite => f.write_str("value must be finite"),
            Self::MissingUnit(unit) => write!(f, "explicit unit required ({unit})"),
            Self::UnexpectedUnit(unit) => write!(f, "unexpected unit: {unit}"),
            Self::UnknownEngineeringPrefix(prefix) => {
                write!(f, "unknown engineering prefix: {prefix}")
            }
            Self::AmbiguousDecimalSeparator => f.write_str("ambiguous decimal separator"),
            Self::UnsupportedDecimalSeparator(separator) => {
                write!(f, "unsupported locale decimal separator: {separator}")
            }
            Self::BelowAbsoluteZero => f.write_str("temperature is below absolute zero"),
        }
    }
}

impl std::error::Error for QuantityInputError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantity::{DecimalSeparatorInput, EngineeringSuffixPolicy, TimeFrequencyInput};

    #[test]
    fn strict_time_and_frequency_fields_require_explicit_units() {
        let policy = QuantityPresentationPolicy::default();
        assert_eq!(
            parse_ui_quantity(
                "10",
                QuantityInputKind::Frequency,
                policy,
                UiNumberLocale::default()
            ),
            Err(QuantityInputError::MissingUnit("Hz"))
        );
        assert_eq!(
            parse_ui_quantity(
                "10MHz",
                QuantityInputKind::Frequency,
                policy,
                UiNumberLocale::default()
            )
            .unwrap(),
            10e6
        );
        let parsed = parse_ui_quantity(
            "250ns",
            QuantityInputKind::Time,
            policy,
            UiNumberLocale::default(),
        )
        .unwrap();
        assert!((parsed - 250e-9).abs() <= f64::EPSILON * 2.0);
    }

    #[test]
    fn strict_ui_engineering_scalars_disambiguate_meg_m_and_mil() {
        let policy = QuantityPresentationPolicy::default();
        let parse = |text| {
            parse_ui_quantity(
                text,
                QuantityInputKind::EngineeringScalar,
                policy,
                UiNumberLocale::default(),
            )
        };
        assert_eq!(parse("10Meg").unwrap(), 10e6);
        assert_eq!(parse("10m").unwrap(), 10e-3);
        assert_eq!(parse("10mil").unwrap(), 254e-6);
        assert!(matches!(
            parse("10M"),
            Err(QuantityInputError::UnknownEngineeringPrefix(prefix)) if prefix == "M"
        ));
    }

    #[test]
    fn inference_is_quantity_scoped_and_never_changes_deck_parsing() {
        let policy = QuantityPresentationPolicy {
            time_frequency_input: TimeFrequencyInput::InferFromFieldQuantity,
            ..QuantityPresentationPolicy::default()
        };
        assert_eq!(
            parse_ui_quantity(
                "2.5",
                QuantityInputKind::Frequency,
                policy,
                UiNumberLocale::default()
            )
            .unwrap(),
            2.5
        );
        assert_eq!(
            parse_ui_quantity(
                "2.5",
                QuantityInputKind::Time,
                policy,
                UiNumberLocale::default()
            )
            .unwrap(),
            2.5
        );
    }

    #[test]
    fn classic_suffix_compatibility_keeps_m_as_milli() {
        let policy = QuantityPresentationPolicy {
            engineering_suffixes: EngineeringSuffixPolicy::ClassicSpiceCompatibility,
            ..QuantityPresentationPolicy::default()
        };
        assert_eq!(
            parse_ui_quantity(
                "1Ms",
                QuantityInputKind::Time,
                policy,
                UiNumberLocale::default()
            )
            .unwrap(),
            1e-3
        );
        assert_eq!(
            parse_ui_quantity(
                "1MeGs",
                QuantityInputKind::Time,
                policy,
                UiNumberLocale::default()
            )
            .unwrap(),
            1e6
        );
    }

    #[test]
    fn platform_supplies_locale_decimal_mark_without_a_baked_locale_fixture() {
        let policy = QuantityPresentationPolicy::default();
        assert_eq!(
            parse_ui_quantity(
                "1,5 kHz",
                QuantityInputKind::Frequency,
                policy,
                UiNumberLocale {
                    decimal_separator: Some(','),
                },
            )
            .unwrap(),
            1_500.0
        );

        let period_policy = QuantityPresentationPolicy {
            decimal_separator_input: DecimalSeparatorInput::PeriodEverywhere,
            ..policy
        };
        assert!(matches!(
            parse_ui_quantity(
                "1,5 kHz",
                QuantityInputKind::Frequency,
                period_policy,
                UiNumberLocale {
                    decimal_separator: Some(','),
                },
            ),
            Err(QuantityInputError::AmbiguousDecimalSeparator)
        ));
    }

    #[test]
    fn temperature_and_angle_parse_to_si_base_units() {
        let policy = QuantityPresentationPolicy::default();
        assert!(
            (parse_ui_quantity(
                "26.85 °C",
                QuantityInputKind::Temperature,
                policy,
                UiNumberLocale::default(),
            )
            .unwrap()
                - 300.0)
                .abs()
                < 1e-12
        );
        assert_eq!(
            parse_ui_quantity(
                "18 °F",
                QuantityInputKind::TemperatureDelta,
                policy,
                UiNumberLocale::default(),
            )
            .unwrap(),
            10.0
        );
        assert!(
            (parse_ui_quantity(
                "180°",
                QuantityInputKind::Angle,
                policy,
                UiNumberLocale::default(),
            )
            .unwrap()
                - std::f64::consts::PI)
                .abs()
                < 1e-12
        );
    }
}
