//! Engineering-notation parsing and formatting ("1k", "10u", "3.3Meg").

/// Word forms for the scale factors, tried ahead of the SPICE letters.
///
/// These are a UI convenience rather than SPICE: no deck reader accepts
/// `1micro`. They are matched first because every one of them begins with a
/// scale letter, and unit letters after a scale are ignored — so `micro`
/// would otherwise read as milli followed by the "unit" `icro`.
const WORD_SCALE_FACTORS: &[(&str, f64)] = &[
    ("tera", 1e12),
    ("giga", 1e9),
    ("mega", 1e6),
    ("kilo", 1e3),
    ("milli", 1e-3),
    ("micro", 1e-6),
    ("nano", 1e-9),
    ("pico", 1e-12),
    ("femto", 1e-15),
];

/// The SPICE scale factors, longest token first.
///
/// `mil` (one thousandth of an inch) and `meg` both have to precede the `m`
/// that starts them. `mil` is here because the engine's own deck reader has
/// it: `crates/rspice-core/src/netlist/lexer.rs` resolves `MIL` to 25.4e-6 in
/// every element-value position, so a field that read `1mil` as one milli
/// would disagree with the netlist the field is compiled into.
///
/// There is no `a` for atto. A scale factor followed by ignored unit letters
/// means the `A` of a one-ampere source is a unit, not a decade, and ngspice
/// has no atto either.
const SPICE_SCALE_FACTORS: &[(&str, f64)] = &[
    ("mil", 25.4e-6),
    ("meg", 1e6),
    ("t", 1e12),
    ("g", 1e9),
    ("k", 1e3),
    ("m", 1e-3),
    ("u", 1e-6),
    ("n", 1e-9),
    ("p", 1e-12),
    ("f", 1e-15),
];

/// Parse an engineering notation value (e.g., "1k", "10u", "3.3meg", "1ns").
///
/// The rule, in order:
///
/// 1. A leading number — digits, one point, a sign, an `e`/`E` exponent.
/// 2. The rest is the suffix: trimmed, lowercased, with U+00B5 MICRO SIGN and
///    U+03BC GREEK SMALL LETTER MU folded to `u`.
/// 3. A word form from [`WORD_SCALE_FACTORS`], then a SPICE scale factor from
///    [`SPICE_SCALE_FACTORS`], longest token first.
/// 4. Whatever follows the scale factor must be alphabetic, and is **ignored**
///    — it is the unit. This is SPICE: `1ns` is one nanosecond, `10kHz` is ten
///    kilohertz, `2.2uF` is 2.2 microfarad.
/// 5. A suffix that is alphabetic but starts with no scale factor is a bare
///    unit and multiplies by one: `5V`, `1A`, `3s`.
/// 6. Anything else is an error: `1k5`, `1N4148`, `12%`, and the empty string.
///
/// Two consequences are SPICE's, and are deliberate rather than defects:
///
/// * `1mHz` is 1e-3, not one millihertz-as-megahertz — `m` is the milli scale
///   and `Hz` is an ignored unit. SPICE has no case-sensitive `M`; mega must
///   be spelled `meg`. (The engine's deck lexer reads the three-letter `MHZ`
///   as megahertz instead; that spelling is the one place the two disagree.)
/// * `1F` is 1e-15, because `f` is femto. A farad has to be written with a
///   scale factor in front of it (`1pF`) to read as a capacitance.
///
/// Returns the numeric value or an error message.
pub fn parse_engineering_value(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("Empty value".to_string());
    }

    // Find where the number ends and the suffix begins
    let mut numeric_end = 0;
    for (index, character) in trimmed.char_indices() {
        if character.is_ascii_digit() || matches!(character, '.' | '-' | '+' | 'e' | 'E') {
            numeric_end = index + character.len_utf8();
        } else {
            break;
        }
    }

    // Parse the numeric part
    let (numeric_str, suffix) = trimmed.split_at(numeric_end);
    let base_value: f64 = numeric_str
        .parse()
        .map_err(|_| format!("Cannot parse '{}' as number", numeric_str))?;

    // Parse the suffix - take the rest and compare lowercase/normalized
    let suffix = suffix.trim();
    let suffix_lower = suffix.to_lowercase();

    // Normalize micro sign: U+00B5 (µ MICRO SIGN) and U+03BC (μ GREEK SMALL LETTER MU) both map to 'u'
    let normalized_suffix = suffix_lower.replace(['\u{00B5}', '\u{03BC}'], "u");

    let multiplier = scale_factor(&normalized_suffix).ok_or_else(|| {
        format!(
            "Unknown suffix: '{}' (normalized: '{}')",
            suffix, normalized_suffix
        )
    })?;

    Ok(base_value * multiplier)
}

/// The decade a normalized suffix names, or `None` when the text is not a
/// scale factor followed by a unit.
///
/// The tables are ordered so that the first token that prefixes the suffix is
/// the longest one that can: a shorter token would leave a remainder that
/// still contains this one's, so a failed unit check here can never hide a
/// match further down.
fn scale_factor(suffix: &str) -> Option<f64> {
    if suffix.is_empty() {
        return Some(1.0);
    }
    for (token, multiplier) in WORD_SCALE_FACTORS.iter().chain(SPICE_SCALE_FACTORS) {
        if let Some(unit) = suffix.strip_prefix(token) {
            return is_unit_text(unit).then_some(*multiplier);
        }
    }
    // No scale factor: a bare unit sits on the same decade as the number.
    is_unit_text(suffix).then_some(1.0)
}

/// Whether the text after a scale factor is a unit rather than more value.
///
/// Alphabetic in the Unicode sense, so `1kΩ` reads as a kilohm; a digit,
/// a sign or a symbol makes the whole string a rejection instead, which is
/// what keeps `1k5` and the part number `1N4148` out of the numeric surfaces.
fn is_unit_text(text: &str) -> bool {
    text.chars().all(char::is_alphabetic)
}

/// How much of a mantissa a surface shows.
///
/// The suffix ladder is settled — every surface picks the same decade and
/// spells it the same way. How many digits follow the point is not: a
/// schematic label wants the shortest text that still says the value, an
/// editor field wants a width that does not jump under a keystroke, and a
/// summary of typed data wants the digits back as typed.
///
/// A whole mantissa prints without a point under every policy, so `1k` is
/// never `1.000k`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineeringPrecision {
    /// The fewest decimals that still name the mantissa, up to three:
    /// `4.7k`, `1.25k`, `3.142k`.
    Adaptive,
    /// Always this many decimals: `4.700k`. Values in a column line up.
    Fixed(u8),
    /// At most this many decimals, trailing zeros dropped: `3.14159k`.
    UpTo(u8),
}

/// Format a value with engineering notation, showing the fewest decimals that
/// still name it.
pub fn format_engineering_value(value: f64) -> String {
    format_engineering_value_with(value, EngineeringPrecision::Adaptive)
}

/// Format a value with engineering notation under a chosen decimal policy.
pub fn format_engineering_value_with(value: f64, precision: EngineeringPrecision) -> String {
    let abs_value = value.abs();

    // Zero sits on no decade, so it takes no suffix — and no sign, which is
    // what keeps an arithmetic `-0.0` from reaching a reader as `-0`.
    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    // Rescaling by a decade carries rounding error, so "is this whole?" is
    // asked with a tolerance rather than an equality: 3.3e6 / 1e6 is not
    // exactly 3.3, and 3.300000000000000Meg is nobody's reading of it.
    let eps = 1e-9;
    let is_whole = (scaled.round() - scaled).abs() < eps;
    if is_whole {
        return format!("{:.0}{}", scaled.round(), suffix);
    }

    match precision {
        EngineeringPrecision::Adaptive => {
            let is_one_decimal = ((scaled * 10.0).round() - scaled * 10.0).abs() < eps;
            let is_two_decimal = ((scaled * 100.0).round() - scaled * 100.0).abs() < eps;
            if is_one_decimal {
                format!("{:.1}{}", scaled, suffix)
            } else if is_two_decimal {
                format!("{:.2}{}", scaled, suffix)
            } else {
                format!("{:.3}{}", scaled, suffix)
            }
        }
        EngineeringPrecision::Fixed(decimals) => {
            format!("{:.*}{}", usize::from(decimals), scaled, suffix)
        }
        EngineeringPrecision::UpTo(decimals) => {
            let padded = format!("{:.*}", usize::from(decimals), scaled);
            let trimmed = padded.trim_end_matches('0').trim_end_matches('.');
            format!("{}{}", trimmed, suffix)
        }
    }
}

/// One row per value: the adaptive form the schematic and netlist surfaces
/// show, the three-decimal form the property editor shows, and the
/// up-to-six-decimal form the PWL summary line shows.
///
/// Three copies of the suffix ladder once stood behind those three columns.
/// They now stand behind one, and every string here was measured from the
/// three originals — so a decimal policy cannot drift into another surface
/// without this table saying so.
#[cfg(test)]
pub(crate) const PRECISION_CHARACTERIZATION: &[(f64, &str, &str, &str)] = &[
    (0.0, "0", "0", "0"),
    (1.0, "1", "1", "1"),
    (47.0, "47", "47", "47"),
    (4.7e3, "4.7k", "4.700k", "4.7k"),
    (3141.59, "3.142k", "3.142k", "3.14159k"),
    (3.3e6, "3.3Meg", "3.300Meg", "3.3Meg"),
    (2.5e9, "2.5G", "2.500G", "2.5G"),
    (1e12, "1T", "1T", "1T"),
    (1e15, "1000T", "1000T", "1000T"),
    // Rounding edges: the three-decimal policies carry 999.9995 up a decade
    // in the text while the six-decimal one still shows the digits, and a
    // mantissa that is not quite whole reads as whole at three decimals.
    (999.9995, "1000.000", "1000.000", "999.9995"),
    (1.0004999, "1.000", "1.000", "1.0005"),
    (1.25, "1.25", "1.250", "1.25"),
    (12.3456789, "12.346", "12.346", "12.345679"),
    (0.5, "500m", "500m", "500m"),
    (2.2e-3, "2.2m", "2.200m", "2.2m"),
    (1.5e-6, "1.5u", "1.500u", "1.5u"),
    (100.0e-9, "100n", "100n", "100n"),
    (47.0e-12, "47p", "47p", "47p"),
    (1e-15, "1f", "1f", "1f"),
    (1e-18, "1a", "1a", "1a"),
    (-6.8e3, "-6.8k", "-6.800k", "-6.8k"),
    (-0.25, "-250m", "-250m", "-250m"),
    (-3.3e6, "-3.3Meg", "-3.300Meg", "-3.3Meg"),
];

#[cfg(test)]
mod tests {
    use super::{
        EngineeringPrecision, PRECISION_CHARACTERIZATION, format_engineering_value,
        format_engineering_value_with, parse_engineering_value,
    };

    /// Zero sits on no decade, and an arithmetic `-0.0` is still zero. The
    /// property editor's own ladder used to let a negative zero reach a
    /// signed format and print `-0`; one entry point gives one answer.
    #[test]
    fn zero_prints_without_a_sign_under_every_precision() {
        for precision in [
            EngineeringPrecision::Adaptive,
            EngineeringPrecision::Fixed(3),
            EngineeringPrecision::UpTo(6),
        ] {
            assert_eq!(format_engineering_value_with(0.0, precision), "0");
            assert_eq!(format_engineering_value_with(-0.0, precision), "0");
        }
    }

    /// Every column of the shared table, read through the policy that names
    /// it. Each surface pins its own column against its own call, in its own
    /// module: the property editor in `state::property_types`, the summary
    /// line in `properties::pwl_editor::data`.
    #[test]
    fn the_displayed_decimals_stay_with_the_policy_that_chose_them() {
        for (value, adaptive, three_decimal, up_to_six) in PRECISION_CHARACTERIZATION {
            assert_eq!(
                &format_engineering_value(*value),
                adaptive,
                "adaptive form of {value}"
            );
            assert_eq!(
                &format_engineering_value_with(*value, EngineeringPrecision::Fixed(3)),
                three_decimal,
                "three-decimal form of {value}"
            );
            assert_eq!(
                &format_engineering_value_with(*value, EngineeringPrecision::UpTo(6)),
                up_to_six,
                "up-to-six-decimal form of {value}"
            );
        }
    }

    /// The suffix ladder every engineering-notation surface in the crate
    /// reads, including the two Unicode micro signs a keyboard or a pasted
    /// datasheet can produce: U+00B5 MICRO SIGN and U+03BC GREEK SMALL LETTER
    /// MU, which look alike and must mean the same decade.
    #[test]
    fn the_suffix_ladder_follows_spice_and_accepts_both_micro_signs() {
        for (text, expected) in [
            ("10", 10.0),
            ("1.5k", 1.5e3),
            ("1.5K", 1.5e3),
            ("2.2u", 2.2e-6),
            ("2.2\u{00B5}", 2.2e-6),
            ("2.2\u{03BC}", 2.2e-6),
            ("2.2micro", 2.2e-6),
            // `m` is milli and `meg` is mega, in any case. SPICE has no
            // case-sensitive `M`, so a value that reads as mega to a human
            // has to be spelled out to be read as mega here.
            ("3m", 3e-3),
            ("3M", 3e-3),
            ("3meg", 3e6),
            ("3Meg", 3e6),
            ("3MEG", 3e6),
            ("1t", 1e12),
            ("1g", 1e9),
            ("1n", 1e-9),
            ("1p", 1e-12),
            ("1f", 1e-15),
            ("1e3", 1e3),
            ("-4.7k", -4.7e3),
        ] {
            let parsed = parse_engineering_value(text)
                .unwrap_or_else(|error| panic!("{text} should parse: {error}"));
            assert!(
                (parsed - expected).abs() <= expected.abs() * 1e-12,
                "{text} parsed as {parsed}, expected {expected}"
            );
        }
    }

    /// A suffix that is not a scale factor followed by a unit is a parse
    /// failure. What makes a part number a rejection is the digits inside it:
    /// `1N4148` is nano followed by `4148`, and `4148` is not a unit, so the
    /// text is refused rather than read as 1e-9.
    #[test]
    fn a_suffix_that_is_not_a_scale_and_a_unit_is_an_error() {
        for text in [
            "", "1N4148", "2N3904", "abc", "1k5", "1.5.2", "12%", "1u/s", "1k-2",
        ] {
            assert!(
                parse_engineering_value(text).is_err(),
                "{text:?} must not parse as an engineering value"
            );
        }
    }

    /// SPICE reads the letters after a scale factor as the unit and ignores
    /// them, and a UI field that refused them refused what a deck accepts: a
    /// source authored `5V` reached the Excitations page as a family name with
    /// no value at all.
    ///
    /// Two rows here look wrong and are not. `1mHz` is one milli-something —
    /// `m` is milli in SPICE and `Hz` is the ignored unit — and `1F` is one
    /// femto-something, because `f` is femto. Both are what a deck reader
    /// does with the same text.
    #[test]
    fn a_unit_after_the_scale_factor_is_ignored_the_way_spice_ignores_it() {
        for (text, expected) in [
            ("5V", 5.0),
            ("1ns", 1e-9),
            ("10kHz", 1e4),
            ("3.3meg", 3.3e6),
            ("1mHz", 1e-3),
            ("1A", 1.0),
            ("2.2uF", 2.2e-6),
            ("1F", 1e-15),
            ("1\u{00B5}s", 1e-6),
            ("100mV", 0.1),
            ("47pF", 47e-12),
            ("1megohm", 1e6),
            ("1kohm", 1e3),
            ("5 volts", 5.0),
            ("2.5s", 2.5),
            ("1millisecond", 1e-3),
            // `mil` is one thousandth of an inch, as it is in the engine's own
            // deck lexer, and the word `milli` still has to beat it.
            ("1mil", 25.4e-6),
            ("2milli", 2e-3),
            // Atto is gone: `a` is an ordinary unit letter, so a one-ampere
            // source reads as one ampere rather than as 1e-18.
            ("1a", 1.0),
            ("1atto", 1.0),
            ("1e3", 1e3),
            ("-4.7k", -4.7e3),
        ] {
            let parsed = parse_engineering_value(text)
                .unwrap_or_else(|error| panic!("{text} should parse: {error}"));
            assert!(
                (parsed - expected).abs() <= expected.abs() * 1e-12,
                "{text} parsed as {parsed}, expected {expected}"
            );
        }
    }

    /// The formatter still spells the decade below femto `a`, which this
    /// parser now reads as a unit rather than as atto — so a sub-femto value
    /// no longer survives a round trip, and the netlist hover that checks the
    /// round trip falls back to the raw float instead of showing `1a`.
    ///
    /// Pinned rather than fixed: the `a` the formatter writes is shared with
    /// two other suffix ladders, and changing it is not this module's call.
    #[test]
    fn the_sub_femto_suffix_the_formatter_writes_no_longer_reads_back_as_atto() {
        assert_eq!(format_engineering_value(1e-18), "1a");
        assert_eq!(parse_engineering_value("1a").unwrap(), 1.0);
    }

    #[test]
    fn formatting_round_trips_through_the_parser_at_every_decade() {
        for value in [
            3.3e6, 4.7e3, 1.0, 2.2e-3, 1.5e-6, 100.0e-9, 47.0e-12, 1.0e-15, 0.0, -6.8e3,
        ] {
            let formatted = format_engineering_value(value);
            let parsed = parse_engineering_value(&formatted)
                .unwrap_or_else(|error| panic!("{formatted} should parse back: {error}"));
            assert!(
                (parsed - value).abs() <= value.abs() * 1e-9,
                "{value} formatted as {formatted}, which parsed back as {parsed}"
            );
        }
    }
}
