//! Parsing a typed quantity.
//!
//! Accepts SPICE suffixes, engineering prefixes, and plain exponents,
//! resolved against the field's expected kind so an ambiguous suffix
//! (`M` for milli against `MEG`) is read the way SPICE reads it.

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
            value * parse_prefix(suffix, "", policy.engineering_suffixes)?
        }
        QuantityInputKind::Time => parse_time_or_frequency(value, suffix, "s", policy)?,
        QuantityInputKind::Frequency => {
            if let Some(prefix) = strip_unit(suffix, "rad/s", policy.engineering_suffixes) {
                value * parse_prefix(prefix, "rad/s", policy.engineering_suffixes)?
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
    let prefix = strip_unit(suffix, unit, policy.engineering_suffixes)
        .ok_or_else(|| QuantityInputError::UnexpectedUnit(suffix.to_owned()))?;
    Ok(value * parse_prefix(prefix, unit, policy.engineering_suffixes)?)
}

/// `suffix` without the field's `unit` at its end. SPICE spelling folds case,
/// so `1MS` and `10KHZ` carry their units there as well.
fn strip_unit<'a>(suffix: &'a str, unit: &str, policy: EngineeringSuffixPolicy) -> Option<&'a str> {
    match policy {
        EngineeringSuffixPolicy::StrictRspice => suffix.strip_suffix(unit),
        EngineeringSuffixPolicy::ClassicSpiceCompatibility => {
            let split = suffix.len().checked_sub(unit.len())?;
            (suffix.is_char_boundary(split) && suffix[split..].eq_ignore_ascii_case(unit))
                .then(|| &suffix[..split])
        }
    }
}

/// The decade a field's engineering prefix names, under the field's policy.
///
/// `unit` is the unit typed after the prefix (empty for a field whose unit
/// comes from its schema). It matters because SPICE reads the two together:
/// `MHz` is megahertz but `Ms` is a millisecond.
///
/// SPICE spelling reads the prefix and unit exactly as the engine's deck
/// lexer does, through [`spice_suffix_scale`]. Strict spelling keeps its own
/// case-sensitive table, so that `M` and `m` can be told apart where a human
/// is looking at the letter, and refuses the few spellings the engine would
/// read as another value (`mHz`, `Ms`), so that a typed field never means
/// one thing and runs another.
///
/// Neither policy has atto. `a` is a unit letter — the `A` of a one-ampere
/// source — in every reader a typed value travels through, the engine's own
/// deck lexer included.
fn parse_prefix(
    prefix: &str,
    unit: &str,
    policy: EngineeringSuffixPolicy,
) -> Result<f64, QuantityInputError> {
    let unknown = || QuantityInputError::UnknownEngineeringPrefix(prefix.to_owned());
    let spelled = format!("{prefix}{unit}");
    let (engine, consumed) = spice_suffix_scale(&spelled);
    // Whether the engine reads the typed prefix as a scale factor, rather than
    // stopping inside it or taking its letter for a unit (`A`, `V`, `S`).
    let engine_reads_prefix =
        prefix.is_empty() || (consumed >= prefix.chars().count() && engine != 1.0);
    match policy {
        EngineeringSuffixPolicy::StrictRspice => {
            let multiplier = match prefix {
                "" => 1.0,
                "T" => 1e12,
                "G" => 1e9,
                // M is unambiguous when an explicit SI unit follows; a bare
                // mega multiplier uses the documented `Meg` spelling.
                "M" if !unit.is_empty() => 1e6,
                "Meg" => 1e6,
                "k" => 1e3,
                "m" => 1e-3,
                "u" | "µ" => 1e-6,
                "n" => 1e-9,
                "p" => 1e-12,
                "f" => 1e-15,
                "mil" => 25.4e-6,
                _ => return Err(unknown()),
            };
            if engine_reads_prefix && engine != multiplier {
                return Err(QuantityInputError::SpiceReadsDifferently(spelled));
            }
            Ok(multiplier)
        }
        // The micro sign is the one SI spelling the engine has no reading
        // for at all, so it cannot be read two ways.
        EngineeringSuffixPolicy::ClassicSpiceCompatibility if prefix == "µ" => Ok(1e-6),
        EngineeringSuffixPolicy::ClassicSpiceCompatibility if engine_reads_prefix => Ok(engine),
        EngineeringSuffixPolicy::ClassicSpiceCompatibility => Err(unknown()),
    }
}

/// The scale the engine's deck lexer gives the suffix at the head of `text`,
/// and how many characters of it that reading consumes.
///
/// A mirror of `rspice_core`'s `spice_suffix_scale`, which this crate cannot
/// depend on. `rspice-design` types every prefix and unit spelling into a
/// SPICE-spelling field and into the engine, and fails if they disagree.
fn spice_suffix_scale(text: &str) -> (f64, usize) {
    let chars = text.chars().collect::<Vec<_>>();
    let Some(&first) = chars.first() else {
        return (1.0, 0);
    };
    if chars.len() >= 3 {
        let three = chars[..3].iter().collect::<String>().to_ascii_uppercase();
        let scale = match three.as_str() {
            "MEG" | "MHZ" => Some(1e6),
            "MIL" => Some(25.4e-6),
            "GHZ" => Some(1e9),
            "KHZ" => Some(1e3),
            "UHZ" => Some(1e-6),
            "NHZ" => Some(1e-9),
            "PHZ" => Some(1e-12),
            "FHZ" => Some(1e-15),
            "THZ" => Some(1e12),
            _ => None,
        };
        if let Some(scale) = scale {
            return (scale, 3);
        }
    }
    if chars.len() >= 2 {
        let prefix = first.to_ascii_uppercase();
        let unit = chars[1].to_ascii_uppercase();
        let scale = match unit {
            'S' | 'F' | 'H' => match prefix {
                'N' => Some(1e-9),
                'P' => Some(1e-12),
                'U' => Some(1e-6),
                'M' => Some(1e-3),
                'F' if unit == 'S' => Some(1e-15),
                _ => None,
            },
            'V' | 'A' | 'M' => match prefix {
                'T' => Some(1e12),
                'G' => Some(1e9),
                'K' => Some(1e3),
                'M' => Some(1e-3),
                'U' => Some(1e-6),
                'N' => Some(1e-9),
                'P' => Some(1e-12),
                'F' => Some(1e-15),
                _ => None,
            },
            _ => None,
        };
        if let Some(scale) = scale {
            return (scale, 2);
        }
    }
    match first.to_ascii_uppercase() {
        'T' => (1e12, 1),
        'G' => (1e9, 1),
        'K' => (1e3, 1),
        'M' => (1e-3, 1),
        'U' => (1e-6, 1),
        'N' => (1e-9, 1),
        'P' => (1e-12, 1),
        'F' => (1e-15, 1),
        'X' if chars.len() == 1 || !chars[1].is_ascii_alphabetic() => (1e6, 1),
        'V' | 'A' | 'S' => (1.0, 1),
        _ => (1.0, 0),
    }
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
    /// Strict spelling refuses a prefix and unit the engine reads as another
    /// value (`mHz` is megahertz to SPICE).
    SpiceReadsDifferently(String),
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
            Self::SpiceReadsDifferently(spelling) => {
                write!(
                    f,
                    "SPICE reads {spelling} as another value; use an exponent"
                )
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

    /// SPICE spelling is the engine's: `MHz` is megahertz whatever its case,
    /// and the field's unit may be typed in any case, as a deck would have it.
    #[test]
    fn spice_spelling_reads_prefix_and_unit_the_way_the_engine_does() {
        let policy = QuantityPresentationPolicy {
            engineering_suffixes: EngineeringSuffixPolicy::ClassicSpiceCompatibility,
            ..QuantityPresentationPolicy::default()
        };
        let parse = |text, kind| parse_ui_quantity(text, kind, policy, UiNumberLocale::default());
        assert_eq!(parse("10MHz", QuantityInputKind::Frequency), Ok(10e6));
        assert_eq!(parse("10mhz", QuantityInputKind::Frequency), Ok(10e6));
        assert_eq!(parse("10KHZ", QuantityInputKind::Frequency), Ok(10e3));
        assert_eq!(parse("10MegHz", QuantityInputKind::Frequency), Ok(10e6));
        assert_eq!(parse("1MS", QuantityInputKind::Time), Ok(1e-3));
        assert_eq!(
            parse("2pF", QuantityInputKind::EngineeringScalar),
            Ok(2e-12)
        );
        assert_eq!(parse("1X", QuantityInputKind::EngineeringScalar), Ok(1e6));
        assert!(parse("1kOhm", QuantityInputKind::EngineeringScalar).is_err());
    }

    /// Strict spelling is case-sensitive, and refuses the spellings the engine
    /// would read as another value instead of reading them its own way.
    #[test]
    fn strict_spelling_refuses_what_the_engine_reads_as_another_value() {
        let policy = QuantityPresentationPolicy::default();
        let parse = |text, kind| parse_ui_quantity(text, kind, policy, UiNumberLocale::default());
        assert_eq!(parse("10MHz", QuantityInputKind::Frequency), Ok(10e6));
        for (text, kind, spelling) in [
            ("1mHz", QuantityInputKind::Frequency, "mHz"),
            ("1Ms", QuantityInputKind::Time, "Ms"),
            ("1Mrad/s", QuantityInputKind::Frequency, "Mrad/s"),
        ] {
            assert_eq!(
                parse(text, kind),
                Err(QuantityInputError::SpiceReadsDifferently(
                    spelling.to_owned()
                )),
                "{text}"
            );
        }
    }

    /// Atto left every reader in the toolchain, this one included. `a` is a
    /// unit letter, not a decade, so a prefix position holding it is an
    /// unknown prefix under both policies rather than 1e-18.
    #[test]
    fn no_policy_reads_a_as_atto_any_more() {
        for suffixes in [
            EngineeringSuffixPolicy::StrictRspice,
            EngineeringSuffixPolicy::ClassicSpiceCompatibility,
        ] {
            let policy = QuantityPresentationPolicy {
                engineering_suffixes: suffixes,
                ..QuantityPresentationPolicy::default()
            };
            for (text, kind) in [
                ("1a", QuantityInputKind::EngineeringScalar),
                ("1as", QuantityInputKind::Time),
                ("1aHz", QuantityInputKind::Frequency),
            ] {
                assert!(
                    matches!(
                        parse_ui_quantity(text, kind, policy, UiNumberLocale::default()),
                        Err(QuantityInputError::UnknownEngineeringPrefix(prefix)) if prefix == "a"
                    ),
                    "{text} under {suffixes:?} must not resolve an atto decade"
                );
            }
        }
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
