//! SI-prefixed number formatting for axes, readouts and cursor blocks.

/// The engineering ladder, yotta down to yocto: the scale-factor suffixes
/// Cadence's analog expression language prints (`aelSuffixNotation`, Y to y).
///
/// Past either end the mantissa stays in 1‥1000 and the decade is stated as
/// an exponent on the number, `24.03e-27 V`, as that language leaves the `e`
/// and exponent in place once its suffixes run out. Piling the value onto the
/// last prefix instead (`0.02402879 yV`) prints a mantissa no engineering
/// readout ever shows and hides the decade in a run of leading zeros.
const SI_MAX: i32 = 8;
const SI_MIN: i32 = -8;

const SI_LADDER: [(f64, i32); 17] = [
    (1.0e24, 8),
    (1.0e21, 7),
    (1.0e18, 6),
    (1.0e15, 5),
    (1.0e12, 4),
    (1.0e9, 3),
    (1.0e6, 2),
    (1.0e3, 1),
    (1.0, 0),
    (1.0e-3, -1),
    (1.0e-6, -2),
    (1.0e-9, -3),
    (1.0e-12, -4),
    (1.0e-15, -5),
    (1.0e-18, -6),
    (1.0e-21, -7),
    (1.0e-24, -8),
];

const SI_PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

/// The prefix the unit takes at ladder index `index`; none past the ladder,
/// where [`si_exponent`] states the decade instead.
fn si_prefix(index: i32) -> &'static str {
    if (SI_MIN..=SI_MAX).contains(&index) {
        SI_PREFIXES[(index - SI_MIN) as usize]
    } else {
        ""
    }
}

/// The exponent the mantissa takes at ladder index `index`: none on the
/// ladder, `e-27` one step below yocto, `e27` one step above yotta.
fn si_exponent(index: i32) -> String {
    if (SI_MIN..=SI_MAX).contains(&index) {
        String::new()
    } else {
        format!("e{}", 3 * index)
    }
}

/// The scale an axis tick ends in: the prefix, or past the ladder the
/// exponent with the typographic minus every tick is signed with.
fn tick_scale(index: i32) -> String {
    si_exponent(index).replace('-', "−") + si_prefix(index)
}

/// `value` expressed under the prefix `index` names.
///
/// Positive indices divide and negative ones multiply, so on the ladder up to
/// 10^21 the factor is an exactly representable power of ten and the mantissa
/// carries no rounding the value did not already have. The factor is applied
/// in two steps past 10^300, which a subnormal needs and no single `f64` holds.
fn si_scaled(value: f64, index: i32) -> f64 {
    let decades = 3 * index.abs();
    let first = 10f64.powi(decades.min(300));
    let rest = 10f64.powi(decades - decades.min(300));
    if index >= 0 {
        value / first / rest
    } else {
        value * first * rest
    }
}

/// Engineering ladder index: mantissa in 1‥1000. Zero and the non-finite
/// values take no scale at all.
fn si_index(magnitude: f64) -> i32 {
    if magnitude == 0.0 || !magnitude.is_finite() {
        return 0;
    }
    if (SI_LADDER[SI_LADDER.len() - 1].0..1.0e27).contains(&magnitude) {
        return SI_LADDER
            .iter()
            .find(|(floor, _)| magnitude >= *floor)
            .map_or(SI_MIN, |(_, index)| *index);
    }
    // Past the ladder, the decade that puts the mantissa in 1‥1000. The
    // logarithm can land a hair either side of an exact power, so the guess
    // is settled against the mantissa it produces.
    let index = (magnitude.log10() / 3.0).floor() as i32;
    let mantissa = si_scaled(magnitude, index);
    if mantissa >= 1.0e3 {
        index + 1
    } else if mantissa < 1.0 {
        index - 1
    } else {
        index
    }
}

/// Tick ladder index. Human-scale values (0.1 V, 0.8 V, 240) stay plain
/// decimals like the design's voltage axes; prefixes start below 0.1 and at
/// 1000.
fn tick_index(magnitude: f64) -> i32 {
    if (0.1..1.0e3).contains(&magnitude) {
        0
    } else {
        si_index(magnitude)
    }
}

/// Whether printing `scaled` to `decimals` places would carry it out of its
/// own prefix. 999.9999 k is 1 M, and printing it as "1000k" reads as a
/// decade error on the axis.
fn overflows_prefix(scaled: f64, decimals: usize) -> bool {
    let factor = 10f64.powi(decimals.min(15) as i32);
    (scaled.abs() * factor).round() / factor >= 1.0e3
}

/// One ladder placement: the index to print under, promoted when rounding
/// would carry the mantissa past 1000. Past yotta the promotion moves the
/// exponent on, so no end of the ladder prints a four-digit mantissa.
fn placed(magnitude: f64, index: i32, decimals: usize) -> i32 {
    if overflows_prefix(si_scaled(magnitude, index), decimals) {
        index + 1
    } else {
        index
    }
}

/// Trim the trailing zeros a fixed-decimal rendering leaves behind, and use
/// the typographic minus the design draws on plots.
fn trimmed(mut text: String) -> String {
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    text.replace('-', "−")
}

/// An axis tick that carries its own unit: `si_tick_label(5e-4, "s", 2)` →
/// `"500 µs"`, `si_tick_label(-3e-3, "V", 2)` → `"−3 mV"`.
///
/// [`tick_label_with_step`] drops the unit, because a result viewer's axis
/// states the quantity once in its own caption and has room to. A 56 point
/// waveform strip has no caption track, so its ticks carry the unit
/// themselves — and they have to be trimmed and signed exactly as every other
/// tick in the product is, or the same number reads two ways on two surfaces.
pub fn si_tick_label(value: f64, unit: &str, digits: usize) -> String {
    let (scaled, index) = scale_si(value, digits);
    let mantissa = trimmed(format!("{scaled:.digits$}"));
    let exponent = si_exponent(index).replace('-', "−");
    let prefix = si_prefix(index);
    if is_plane_angle(unit) {
        format!("{mantissa}{exponent}{prefix}{unit}")
    } else {
        format!("{mantissa}{exponent} {prefix}{unit}")
            .trim_end()
            .to_owned()
    }
}

/// A ladder tick with the unit its axis has no caption to carry:
/// `tick_with_unit("2m", "V")` → `"2 mV"`, `tick_with_unit("1.5", "V")` →
/// `"1.5 V"`, and a plane angle against its number, `"45°"`.
///
/// The spelling [`si_tick_label`] gives a tick that is not on a ladder, so a
/// number reads one way on every surface whose ticks state their own unit.
pub fn tick_with_unit(label: &str, unit: &str) -> String {
    let mantissa = label.trim_end_matches(char::is_alphabetic);
    let prefix = &label[mantissa.len()..];
    if is_plane_angle(unit) {
        format!("{mantissa}{prefix}{unit}")
    } else {
        format!("{mantissa} {prefix}{unit}").trim_end().to_owned()
    }
}

/// A value in free text: `significant_digits` significant digits, an SI
/// prefix on the unit, and a true minus sign.
///
/// - A value is rounded once, to the digits asked for; a rounding that
///   carries into the next decade drops the digit it no longer has room for
///   (999.95 at four digits is `1.000 kV`, never `1.0000 kV`).
/// - Zeros are printed only as digits of a rounded value. A value that has
///   fewer digits than the preference prints the digits it has: a node at
///   exactly 4.1 V reads `4.1 V`, not `4.100000 V`, while 1.7999999 V at four
///   digits reads `1.800 V` because those zeros were resolved. A column or a
///   readout list keeps every digit instead ([`fmt_column_reading`]).
/// - A unit that takes no prefix ([`unit_takes_prefix`]) is never given one:
///   a ratio of 0.0012 reads `0.0012`, a damping factor never `316.2 m`, and
///   far from unity the decade is an exponent on the number (`4.1e−12`).
/// - A plane angle stands against its number (`45°`); a temperature keeps
///   its space (`27 °C`).
/// - Negative zero reads `0`.
///
/// Engineering export serializes the original `f64` samples, not this text.
pub fn fmt_si_significant(value: f64, unit: &str, significant_digits: usize) -> String {
    let (number, prefix) = significant_number(
        value,
        significant_digits,
        unit_takes_prefix(unit),
        Zeros::Trim,
    );
    joined(&number, prefix, unit)
}

/// A decibel or plane-angle figure's places: to the hundredth.
pub const DECIBEL_PLACES: usize = 2;

/// A share's places: to the tenth of a percent.
pub const SHARE_PLACES: usize = 1;

/// `value` to `decimals` places with a true minus sign and no negative
/// zero, so every row of a column stands at one precision and its decimal
/// points align: a decibel to its hundredth, a share to its tenth of a
/// percent. A value within half a step of zero reads `0.00`, never `−0.00`.
#[must_use]
pub fn fixed_places(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let unsigned = text.trim_start_matches('-');
    if unsigned
        .chars()
        .all(|character| matches!(character, '0' | '.'))
    {
        return unsigned.to_owned();
    }
    text.replace('-', "\u{2212}")
}

/// [`fixed_places`] with an explicit sign on a value that does not read as
/// zero, for a change: `+1.50`, `−0.60`, `0.00`.
#[must_use]
pub fn signed_fixed_places(value: f64, decimals: usize) -> String {
    let text = fixed_places(value, decimals);
    if value > 0.0 && text.chars().any(|digit| matches!(digit, '1'..='9')) {
        format!("+{text}")
    } else {
        text
    }
}

/// Whether a value in `unit` is read to a fixed resolution rather than to
/// significant digits: a level in decibels (`dB`, `dBc`, `dBm`, `dBc/Hz`,
/// `dB/dec`) or a plane angle in degrees. A logarithmic or angular reading
/// has a resolution, not a magnitude, so `−3.0002 dB` and `−40 dB` both
/// read to the hundredth.
#[must_use]
pub fn reads_at_fixed_places(unit: &str) -> bool {
    unit == "°"
        || unit
            .split('/')
            .next()
            .is_some_and(|numerator| numerator.starts_with("dB"))
}

/// A reading in free text: a decibel or a plane angle to the hundredth
/// (`−3.00 dB`, `45.00°`), any other value as [`fmt_si_significant`] spells
/// it.
#[must_use]
pub fn fmt_reading(value: f64, unit: &str, significant_digits: usize) -> String {
    if reads_at_fixed_places(unit) {
        joined(&fixed_places(value, DECIBEL_PLACES), "", unit)
    } else {
        fmt_si_significant(value, unit, significant_digits)
    }
}

/// A reading in a table column or a readout list: [`fmt_reading`], with
/// every one of the reader's `significant_digits`, so a column reads
/// `8.300 ns` beside `6.917 ns` and its decimal points fall in line.
#[must_use]
pub fn fmt_column_reading(value: f64, unit: &str, significant_digits: usize) -> String {
    if reads_at_fixed_places(unit) {
        return fmt_reading(value, unit, significant_digits);
    }
    let (number, prefix) = significant_number(
        value,
        significant_digits,
        unit_takes_prefix(unit),
        Zeros::Keep,
    );
    joined(&number, prefix, unit)
}

/// [`fmt_column_reading`] with an explicit sign on a positive value, for a
/// change or a margin: `+1.50 dB`, `−600.0 µV`, and zero unsigned.
#[must_use]
pub fn fmt_signed_reading(value: f64, unit: &str, significant_digits: usize) -> String {
    let text = fmt_column_reading(value, unit, significant_digits);
    if value > 0.0 && text.chars().any(|digit| matches!(digit, '1'..='9')) {
        format!("+{text}")
    } else {
        text
    }
}

/// A complex value with one unit: `1.200 + 3.400j V`. Both parts stand
/// under one prefix and at one resolution, the larger part's at
/// `significant_digits`, so a part far smaller than the other reads as the
/// zero it is at that resolution (`1.200 + 0.000j V`).
#[must_use]
pub fn fmt_complex(real: f64, imaginary: f64, unit: &str, significant_digits: usize) -> String {
    let larger = real.abs().max(imaginary.abs());
    SharedReading::new(larger, larger, unit, significant_digits).complex(real, imaginary, unit)
}

/// The most decades one resolution reads a set of values across: past it,
/// the smallest would keep none of its digits beside the largest.
pub const SHARED_DECADES: i32 = 4;

/// How values read side by side, as the cells of one matrix do: at one
/// resolution, or each on its own decade where one resolution cannot read
/// them all.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SharedReading {
    /// Every value under one ladder index and to one count of decimals: the
    /// largest at the reader's digits, every other at its resolution.
    Fixed {
        index: i32,
        decimals: usize,
        prefixed: bool,
    },
    /// Each value in engineering notation at the reader's digits, its decade
    /// an exponent on the number, so one unit still names every part.
    Engineering { significant_digits: usize },
    /// No finite, nonzero value to take a resolution from.
    Bare,
}

impl SharedReading {
    /// The reading of values in `unit` whose largest part is `largest` and
    /// whose smallest nonzero magnitude is `smallest`: one resolution, the
    /// largest's at `significant_digits`, unless they span more than
    /// [`SHARED_DECADES`] decades, when each reads in engineering notation.
    #[must_use]
    pub fn new(largest: f64, smallest: f64, unit: &str, significant_digits: usize) -> Self {
        let digits = significant_digits.clamp(1, 17);
        if !(largest > 0.0 && largest.is_finite()) {
            return Self::Bare;
        }
        if smallest > 0.0 && smallest.is_finite() && largest / smallest > 10f64.powi(SHARED_DECADES)
        {
            return Self::Engineering {
                significant_digits: digits,
            };
        }
        let prefixed = unit_takes_prefix(unit);
        let (text, index) = placed_significant(largest, digits, prefixed, Zeros::Keep);
        let decimals = text.split_once('.').map_or(0, |(_, digits)| digits.len());
        Self::Fixed {
            index,
            decimals,
            prefixed,
        }
    }

    /// `value`'s number under this reading, and the prefix its unit takes.
    fn part(self, value: f64) -> (String, &'static str) {
        match self {
            Self::Fixed {
                index,
                decimals,
                prefixed,
            } => {
                let (exponent, prefix) = decade(index, prefixed);
                (
                    fixed_places(si_scaled(value, index), decimals) + &exponent,
                    prefix,
                )
            }
            Self::Engineering { significant_digits } if value != 0.0 && value.is_finite() => {
                let (text, index) =
                    placed_significant(value.abs(), significant_digits, true, Zeros::Keep);
                let sign = if value < 0.0 { "\u{2212}" } else { "" };
                (format!("{sign}{text}{}", decade(index, false).0), "")
            }
            Self::Engineering { .. } | Self::Bare => {
                (significant_number(value, 1, false, Zeros::Trim).0, "")
            }
        }
    }

    /// `value` in `unit` under this reading: `0.0100`, `12.00e−6`,
    /// `0.050 kΩ`.
    #[must_use]
    pub fn reading(self, value: f64, unit: &str) -> String {
        let (number, prefix) = self.part(value);
        joined(&number, prefix, unit)
    }

    /// A complex value in `unit` under this reading, its unit named once:
    /// `0.2500 − 0.0100j`.
    #[must_use]
    pub fn complex(self, real: f64, imaginary: f64, unit: &str) -> String {
        let ((real, prefix), (imaginary, _)) = (self.part(real), self.part(imaginary));
        let (sign, imaginary) = match imaginary.strip_prefix('\u{2212}') {
            Some(magnitude) => ('\u{2212}', magnitude.to_owned()),
            None => ('+', imaginary),
        };
        joined(&format!("{real} {sign} {imaginary}j"), prefix, unit)
    }
}

/// A value whose unit is not known, in the suffix notation SPICE and ADE's
/// outputs print: `6.917n`. The prefix still states the decade; no unit is
/// guessed.
#[must_use]
pub fn fmt_si_suffixed(value: f64, significant_digits: usize) -> String {
    let (number, prefix) = significant_number(value, significant_digits, true, Zeros::Keep);
    format!("{number}{prefix}")
}

/// A measured scalar in a column, in the unit its measurement implies:
/// `symbol` is the measurement's unit as the engine records it (`s`, `deg`,
/// `ohm`, `V/s`, `1`), `None` when it is not known, which reads bare with
/// its prefix ([`fmt_si_suffixed`]).
#[must_use]
pub fn fmt_measured(value: f64, symbol: Option<&str>, significant_digits: usize) -> String {
    match symbol {
        Some(symbol) => fmt_column_reading(value, &measured_unit(symbol), significant_digits),
        None => fmt_si_suffixed(value, significant_digits),
    }
}

/// A measurement unit as the engine records it, as a reader reads it:
/// `deg` is `°`, `degC` is `°C`, `ohm` is `Ω`, `V*s` is `V·s`, and a
/// dimensionless `1` is no unit at all.
#[must_use]
pub fn measured_unit(symbol: &str) -> String {
    match symbol.trim() {
        "1" | "ratio" | "unitless" | "dimensionless" | "count" => String::new(),
        "deg" | "degree" | "degrees" => "°".to_owned(),
        "degC" | "celsius" => "°C".to_owned(),
        "ohm" | "ohms" => "Ω".to_owned(),
        "percent" => "%".to_owned(),
        symbol => symbol
            .replace("sqrt(Hz)", "√Hz")
            .replace("ohm", "Ω")
            .replace("degC", "°C")
            .replace("deg", "°")
            .replace("^2", "²")
            .replace("^3", "³")
            .replace('*', "·"),
    }
}

/// Whether a printed reading leaves its unit's prefix slot blank: its unit
/// takes a prefix and the value was printed without one (`1.144 V`, not
/// `604.8 mV`). A right-aligned column holds the slot's room after such a
/// reading, so its digits end where a prefixed neighbour's do. A unit that
/// never takes a prefix (`−3.00 dB`, `45.00°`, `12.5 %`) and a bare number
/// have no slot.
#[must_use]
pub fn leaves_prefix_slot_blank(reading: &str) -> bool {
    let Some((_, unit)) = reading.trim_end().rsplit_once(' ') else {
        return false;
    };
    // A number after the last space is a part of the value, not its unit:
    // the imaginary part of `0.540 − 0.452j`.
    if unit
        .chars()
        .next()
        .is_none_or(|first| first.is_ascii_digit() || matches!(first, '.' | '+' | '-' | '\u{2212}'))
    {
        return false;
    }
    let mut characters = unit.chars();
    let prefixed = characters.next().is_some_and(|first| {
        let rest = characters.as_str();
        SI_PREFIXES
            .iter()
            .any(|prefix| prefix.starts_with(first) && !prefix.is_empty())
            && !rest.is_empty()
            && (unit_takes_prefix(rest) || unit_carries_its_decade(unit))
    });
    !prefixed && unit_takes_prefix(unit)
}

/// Whether `unit` names a plane angle, which stands against its number.
pub(crate) fn is_plane_angle(unit: &str) -> bool {
    unit == "°" || unit.starts_with("°/")
}

/// A number and its unit as one reading: a space between them, none before
/// a plane angle's degree sign, and none after a number with no unit.
fn joined(number: &str, prefix: &str, unit: &str) -> String {
    if is_plane_angle(unit) {
        format!("{number}{unit}")
    } else {
        format!("{number} {prefix}{unit}").trim_end().to_owned()
    }
}

/// Significant digits a swept coordinate is spelled to: as many as a `.step`
/// legend prints, so two points a deck declares a digit apart are never
/// named alike. A coordinate names a point the netlist stated; it is not a
/// reading, so the reader's digits do not round it.
const COORDINATE_DIGITS: usize = 6;

/// A swept value spelled as a netlist states it, the way LTspice's `.step`
/// legend and ADE's design-point labels name a point: the scale suffix on
/// the number, `2k`, `1.5Meg`, `10µ`, and the unit after it where the swept
/// thing names one, `2kΩ`, `1MΩ`. A temperature keeps the space every
/// reading of one has, `27 °C`. A bare number keeps SPICE's `Meg`, where a
/// lone `M` is milli; before a unit the letters are SI's.
pub fn spice_coordinate(value: f64, unit: &str) -> String {
    let prefixed = unit.is_empty() || unit_takes_prefix(unit);
    let (number, prefix) = significant_number(value, COORDINATE_DIGITS, prefixed, Zeros::Trim);
    let prefix = if unit.is_empty() && prefix == "M" {
        "Meg"
    } else {
        prefix
    };
    let space = if unit.starts_with('°') && !is_plane_angle(unit) {
        " "
    } else {
        ""
    };
    format!("{number}{prefix}{space}{unit}")
}

/// The unit a `.STEP` of a device's own value sweeps, read from the
/// device's letter as SPICE reads it: a resistor in ohms, a capacitor in
/// farads, an inductor in henries, a voltage source in volts and a current
/// source in amperes. A device of any other kind names no unit.
#[must_use]
pub fn swept_device_unit(device: &str) -> &'static str {
    match device
        .chars()
        .next()
        .map(|letter| letter.to_ascii_lowercase())
    {
        Some('r') => "Ω",
        Some('c') => "F",
        Some('l') => "H",
        Some('v') => "V",
        Some('i') => "A",
        _ => "",
    }
}

/// Whether a value in `unit` is written with an SI prefix.
///
/// A dimensionless value (no unit, a ratio stated as `/V` or `1/V`, a count
/// of standard deviations, a fraction of a unit interval), a level in
/// decibels, an angle (`°`, `rad`, but not `rad/s`), a temperature in degrees
/// and a percentage are never prefixed, and a compound unit is prefixed as
/// its numerator is (`2 MV/s`, `−20 dB/dec`, `45 °/kHz`). Nor is a unit that
/// already names its own decade: `1786 nV/√Hz`, never `1.786 knV/√Hz`.
#[must_use]
pub fn unit_takes_prefix(unit: &str) -> bool {
    if unit_carries_its_decade(unit) {
        return false;
    }
    let numerator = unit.split('/').next().unwrap_or_default();
    !(numerator.is_empty()
        || unit == "rad"
        || numerator.starts_with("dB")
        || matches!(
            numerator,
            "1" | "°" | "°C" | "°F" | "deg" | "%" | "σ" | "sigma" | "UI"
        ))
}

/// Whether a unit already names its decade, so a value in it is written as
/// it stands: the noise densities the noise sheet reports in.
#[must_use]
pub fn unit_carries_its_decade(unit: &str) -> bool {
    matches!(unit, "nV/√Hz" | "nA/√Hz" | "ns/√Hz")
}

/// [`fmt_si_significant`] with an explicit sign on a positive value, for a
/// change or a margin: `+1.5 dB`, `−0.6 dB`, and zero unsigned.
pub fn fmt_si_signed(value: f64, unit: &str, significant_digits: usize) -> String {
    let text = fmt_si_significant(value, unit, significant_digits);
    if value > 0.0 && !text.starts_with('0') {
        format!("+{text}")
    } else {
        text
    }
}

/// Whether a printed value keeps the zeros its rounding leaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zeros {
    /// Free text: zeros only where the rounding resolved them.
    Trim,
    /// A column or a readout list: every one of the reader's digits.
    Keep,
}

/// The number part of a value and the prefix its unit takes.
fn significant_number(
    value: f64,
    significant_digits: usize,
    prefixed: bool,
    zeros: Zeros,
) -> (String, &'static str) {
    let digits = significant_digits.clamp(1, 17);
    if value.is_nan() {
        return ("NaN".to_owned(), "");
    }
    if value.is_infinite() {
        let text = if value < 0.0 { "\u{2212}inf" } else { "inf" };
        return (text.to_owned(), "");
    }
    if value == 0.0 {
        return ("0".to_owned(), "");
    }
    let (text, index) = placed_significant(value.abs(), digits, prefixed, zeros);
    let (exponent, prefix) = decade(index, prefixed);
    let sign = if value < 0.0 { "\u{2212}" } else { "" };
    (format!("{sign}{text}{exponent}"), prefix)
}

/// A positive, finite `magnitude` to `digits` significant digits: its
/// mantissa's text and the ladder index it is printed under.
fn placed_significant(
    magnitude: f64,
    digits: usize,
    prefixed: bool,
    zeros: Zeros,
) -> (String, i32) {
    let mut index = band_index(magnitude, prefixed);
    let mut text = significant_mantissa(si_scaled(magnitude, index), digits, zeros);
    // A rounding that carries the mantissa out of its band (999.95 → 1000)
    // is printed under the next band, where it has its digits back.
    // The test stays in mantissa space: scaled back, a carry at the top of
    // the double would overflow to infinity.
    let ceiling = match (prefixed, index) {
        (false, 0) => 1.0e6,
        (false, -2) => 1.0e2,
        _ => 1.0e3,
    };
    if text.parse::<f64>().is_ok_and(|rounded| rounded >= ceiling) {
        // Without a prefix, the plain band spans 0.0001 to a million, so a
        // carry into it or out of it skips the indices it covers.
        index = match (prefixed, index) {
            (false, -2) => 0,
            (false, 0) => 2,
            _ => index + 1,
        };
        text = significant_mantissa(si_scaled(magnitude, index), digits, zeros);
    }
    (text, index)
}

/// How ladder index `index` is written: the exponent on the number and the
/// prefix on the unit. A prefixed unit takes the prefix on the ladder and
/// the exponent past it; a unit that takes no prefix takes the exponent
/// outside its plain band.
fn decade(index: i32, prefixed: bool) -> (String, &'static str) {
    let on_ladder = prefixed && (SI_MIN..=SI_MAX).contains(&index);
    let exponent = if index == 0 || on_ladder {
        String::new()
    } else {
        format!("e{}", 3 * index).replace('-', "\u{2212}")
    };
    let prefix = if on_ladder { si_prefix(index) } else { "" };
    (exponent, prefix)
}

/// The ladder index a magnitude is printed under: the SI prefix ladder for a
/// prefixed unit; plain decimals from 0.0001 to a million for a unit that
/// takes no prefix, as `%g` keeps them, and an engineering exponent past
/// them.
fn band_index(magnitude: f64, prefixed: bool) -> i32 {
    if prefixed || !(1.0e-4..1.0e6).contains(&magnitude) {
        si_index(magnitude)
    } else {
        0
    }
}

/// `mantissa` (positive) to `digits` significant digits, keeping only the
/// zeros the rounding resolved unless `zeros` keeps them all.
fn significant_mantissa(mantissa: f64, digits: usize, zeros: Zeros) -> String {
    let mut decade = mantissa.log10().floor() as i32;
    let mut text = round_to_significant(mantissa, digits, decade);
    // Rounding up to the next power of ten (9.9996 → 10.000) leaves one digit
    // more than was asked for; the decade it reached sets the decimals.
    if text
        .parse::<f64>()
        .is_ok_and(|rounded| rounded >= 10f64.powi(decade + 1))
    {
        decade += 1;
        text = round_to_significant(mantissa, digits, decade);
    }
    if zeros == Zeros::Keep {
        return text;
    }
    let decimals = (digits as i32 - 1 - decade).max(0);
    let trimmed = if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text.as_str()
    };
    // The zeros go when the value is the shorter number to well within the
    // last printed digit, or to within the arithmetic noise of a double (a
    // kelvin-to-celsius subtraction leaves 27.000000000001): they would claim
    // a resolution the value does not need.
    let resolution = 10f64.powi(-decimals) * 1.0e-6;
    let tolerance = resolution.max(mantissa * 1.0e-12);
    match trimmed.parse::<f64>() {
        Ok(short) if (short - mantissa).abs() <= tolerance => trimmed.to_owned(),
        _ => text,
    }
}

/// `mantissa` rounded to `digits` significant digits, given its decade.
fn round_to_significant(mantissa: f64, digits: usize, decade: i32) -> String {
    let decimals = digits as i32 - 1 - decade;
    if decimals >= 0 {
        format!("{:.*}", decimals as usize, mantissa)
    } else {
        let quantum = 10f64.powi(-decimals);
        format!("{:.0}", (mantissa / quantum).round() * quantum)
    }
}

/// `value` placed on the ladder for `decimals` places: the mantissa and the
/// ladder index it is printed under.
fn scale_si(value: f64, decimals: usize) -> (f64, i32) {
    let magnitude = value.abs();
    if magnitude == 0.0 || !magnitude.is_finite() {
        return (value, 0);
    }
    let index = placed(magnitude, si_index(magnitude), decimals);
    (si_scaled(value, index), index)
}

/// Short tick label for an axis value: SI-prefixed, no unit, trailing zeros
/// trimmed (`200_000.0` → `"200k"`, `0.4` → `"0.4"`).
///
/// Three decimals is the fallback for a value with no tick spacing to derive
/// its precision from — a readout, an axis end, a delta. An axis with a step
/// uses [`tick_label_with_step`], which is the only way adjacent ticks are
/// guaranteed to read differently.
pub fn tick_label(value: f64) -> String {
    tick_label_with_decimals(value, 3)
}

/// Tick label for an axis whose ticks are `step` apart.
///
/// The decimal count comes from the step, not from a constant: at 1000:1 zoom
/// a fixed three decimals printed every tick on the axis as the same string,
/// which is an axis that has stopped saying anything. Enough places to
/// resolve one step is exactly enough for two neighbours to differ, and
/// trailing zeros are trimmed so an ordinary axis is unchanged.
pub fn tick_label_with_step(value: f64, step: f64) -> String {
    if !(step.is_finite() && step > 0.0) {
        return tick_label(value);
    }
    if value == 0.0 || !value.is_finite() {
        return tick_label(value);
    }
    let magnitude = value.abs();
    let mut index = tick_index(magnitude);
    let mut decimals = step_decimals(si_scaled(step, index));
    let promoted = placed(magnitude, index, decimals);
    if promoted != index {
        index = promoted;
        decimals = step_decimals(si_scaled(step, index));
    }
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + &tick_scale(index)
}

/// The labels of evenly spaced axis ticks `step` apart, under one prefix.
///
/// A prefix chosen tick by tick put three scales on one axis ("999.8, 1k,
/// 1.0002k", or "0, 50m, 0.1, 0.15"), which a reader has to convert before
/// comparing two neighbours. The axis takes one: the prefix of its largest
/// tick, or the one either side of it when that states every tick in fewer
/// characters or with fewer leading zeros, so a window across 1000 reads
/// `999.8, 1000, 1000.2` and one from zero to 150 m reads `0, 50m, 100m,
/// 150m`. Where the axis reaches the next prefix, the ticks that reach it
/// read in it, as [`across_prefix_boundary`] says: one from 200 to 1000
/// reads `200, 400, … 800, 1k`, never `0.2k` and never `1000`.
pub fn axis_tick_labels(values: &[f64], step: f64) -> Vec<String> {
    let largest = values
        .iter()
        .map(|value| value.abs())
        .filter(|value| value.is_finite())
        .fold(0.0_f64, f64::max);
    if !(step.is_finite() && step > 0.0) || largest == 0.0 {
        return values.iter().map(|value| tick_label(*value)).collect();
    }
    let base = si_index(largest);
    // The shortest labels, then the fewest that start "0.", then the
    // largest tick's own prefix.
    let cost = |labels: &Vec<String>| {
        let widest = labels
            .iter()
            .map(|label| label.chars().count())
            .max()
            .unwrap_or(0);
        let fractional = labels
            .iter()
            .filter(|label| label.trim_start_matches('\u{2212}').starts_with("0."))
            .count();
        (widest, fractional)
    };
    [base, base + 1, base - 1]
        .into_iter()
        .map(|index| (index, labels_under(values, step, index)))
        .min_by_key(|(_, labels)| cost(labels))
        .map(|(index, labels)| across_prefix_boundary(values, step, index, labels))
        .unwrap_or_default()
}

/// The labels of `values`, ticks `step` apart, under the prefix `index`.
fn labels_under(values: &[f64], step: f64, index: i32) -> Vec<String> {
    let decimals = step_decimals(si_scaled(step, index));
    values
        .iter()
        .map(|value| {
            if *value == 0.0 || !value.is_finite() {
                tick_label(*value)
            } else {
                trimmed(format!("{:.*}", decimals, si_scaled(*value, index))) + &tick_scale(index)
            }
        })
        .collect()
}

/// `labels`, ticks `step` apart under the prefix `index`, with every tick
/// that reads a thousand or more of that prefix restated in the next one
/// up, where each such tick reads shorter so: the top of a current axis
/// that reaches a milliampere reads `1m`, not `1000µ`, and its other ticks
/// keep `800µ`. A window only just across a boundary keeps its digits
/// (`999.9, 1000, 1000.1`), since `1.0001k` would read longer.
fn across_prefix_boundary(
    values: &[f64],
    step: f64,
    index: i32,
    labels: Vec<String>,
) -> Vec<String> {
    let decimals = step_decimals(si_scaled(step, index));
    let thousand = 1_000.0 - 0.5 * 10f64.powi(-(decimals as i32));
    let up = index + 1;
    let mut restated = labels.clone();
    for ((value, label), slot) in values.iter().zip(&labels).zip(restated.iter_mut()) {
        if !value.is_finite() || si_scaled(value.abs(), index) < thousand {
            continue;
        }
        let shorter = labels_under(&[*value], step, up).remove(0);
        if shorter.chars().count() >= label.chars().count() {
            return labels;
        }
        *slot = shorter;
    }
    restated
}

/// The labels of an offset axis' ticks, each a signed distance from the
/// anchor, under one prefix as [`axis_tick_labels`] chooses it.
pub fn offset_tick_labels(deltas: &[f64], step: f64) -> Vec<String> {
    let magnitudes = deltas.iter().map(|delta| delta.abs()).collect::<Vec<_>>();
    axis_tick_labels(&magnitudes, step)
        .into_iter()
        .zip(deltas)
        .map(|(label, delta)| {
            if label == "0" {
                label
            } else if *delta < 0.0 {
                format!("\u{2212}{label}")
            } else {
                format!("+{label}")
            }
        })
        .collect()
}

/// The anchor an offset axis states once, beside its ticks: the value every
/// tick label is an offset from, with its unit.
pub fn offset_anchor_label(anchor: f64, unit: &str, step: f64) -> String {
    let sign = if anchor < 0.0 { "−" } else { "+" };
    let magnitude = anchor.abs();
    let index = if magnitude == 0.0 || !magnitude.is_finite() {
        0
    } else {
        let start = tick_index(magnitude);
        placed(magnitude, start, step_decimals(si_scaled(step, start)))
    };
    let decimals = step_decimals(si_scaled(step, index));
    let mantissa = trimmed(format!("{:.*}", decimals, si_scaled(magnitude, index)));
    let exponent = si_exponent(index).replace('-', "−");
    format!("{sign}{mantissa}{exponent} {}{unit}", si_prefix(index))
        .trim_end()
        .to_owned()
}

/// Decimal places needed to resolve one step. A step of 0.25 needs one
/// place, 2e-4 needs four, and 20 needs none.
fn step_decimals(scaled_step: f64) -> usize {
    let step = scaled_step.abs();
    // The finiteness test leads so it is the one that catches NaN: the pair
    // rejects exactly what it rejected when the first half was written `!(step
    // > 0.0)`, which is every step that is not a positive real number.
    if !step.is_finite() || step <= 0.0 {
        return 3;
    }
    (-step.log10()).ceil().clamp(0.0, 15.0) as usize
}

/// Whether an axis this narrow around this centre can label its ticks
/// absolutely at all.
///
/// Past this ratio every tick label shares a leading run of digits with every
/// other, and the reader is left comparing the last two characters of eight.
/// The instrument states the common part once instead.
pub const OFFSET_NOTATION_RATIO: f64 = 1.0e-4;

/// Whether `[min, max]` should be labeled as offsets from a stated anchor.
pub fn wants_offset_notation(min: f64, max: f64) -> bool {
    let span = max - min;
    let centre = (min + max) * 0.5;
    span.is_finite()
        && span > 0.0
        && centre.is_finite()
        && centre != 0.0
        && span / centre.abs() < OFFSET_NOTATION_RATIO
}

fn tick_label_with_decimals(value: f64, decimals: usize) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    if !value.is_finite() {
        return format!("{value}").replace('-', "−");
    }
    let magnitude = value.abs();
    let index = placed(magnitude, tick_index(magnitude), decimals);
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + &tick_scale(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `.step` point reads as the netlist wrote it: `.STEP PARAM r LIST
    /// 1k 2k 1Meg` names its points `1k`, `2k` and `1Meg`, never `2000`, and
    /// a swept device or temperature carries its unit.
    #[test]
    fn a_swept_value_reads_as_the_netlist_states_it() {
        assert_eq!(spice_coordinate(2000.0, ""), "2k");
        assert_eq!(spice_coordinate(1500.0, ""), "1.5k");
        assert_eq!(spice_coordinate(1.0e6, ""), "1Meg");
        assert_eq!(spice_coordinate(1.0e-5, ""), "10µ");
        assert_eq!(spice_coordinate(10.0e-12, ""), "10p");
        assert_eq!(spice_coordinate(0.5, ""), "500m");
        assert_eq!(spice_coordinate(0.0, ""), "0");
        assert_eq!(spice_coordinate(-2.5, ""), "−2.5");
        assert_eq!(spice_coordinate(2000.0, "Ω"), "2kΩ");
        assert_eq!(spice_coordinate(1.0e6, "Ω"), "1MΩ");
        assert_eq!(spice_coordinate(1.5, "V"), "1.5V");
        assert_eq!(spice_coordinate(27.0, "°C"), "27 °C");
        assert_eq!(spice_coordinate(-40.0, "°C"), "−40 °C");
        // A `.STEP DEC` point keeps the digits a legend prints, not twelve.
        assert_eq!(spice_coordinate(10f64.powf(1.0 / 3.0), ""), "2.15443");
        // Points a digit apart in the deck stay apart.
        assert_ne!(spice_coordinate(1000.1, ""), spice_coordinate(1000.2, ""));
    }

    #[test]
    fn a_swept_device_names_its_unit_by_its_letter() {
        assert_eq!(swept_device_unit("R1"), "Ω");
        assert_eq!(swept_device_unit("rload"), "Ω");
        assert_eq!(swept_device_unit("C2"), "F");
        assert_eq!(swept_device_unit("L1"), "H");
        assert_eq!(swept_device_unit("V1"), "V");
        assert_eq!(swept_device_unit("Ibias"), "A");
        assert_eq!(swept_device_unit("Q1"), "");
        assert_eq!(swept_device_unit(""), "");
    }

    /// A ladder tick and an off-ladder tick of the same number read alike,
    /// which is the whole reason the first one has a spelling of its own.
    #[test]
    fn a_ladder_tick_with_its_unit_reads_like_an_off_ladder_one() {
        assert_eq!(tick_with_unit("2m", "V"), "2 mV");
        assert_eq!(tick_with_unit("500µ", "s"), "500 µs");
        assert_eq!(tick_with_unit("−1.5", "A"), "−1.5 A");
        assert_eq!(tick_with_unit("4k", ""), "4 k");
        assert_eq!(tick_with_unit("2m", "V"), si_tick_label(2e-3, "V", 2));
        assert_eq!(tick_with_unit("500µ", "s"), si_tick_label(5e-4, "s", 2));
        // A phase axis's top tick names its degree against the number.
        assert_eq!(tick_with_unit("45", "°"), "45°");
        assert_eq!(si_tick_label(45.0, "°", 2), "45°");
    }

    /// A step with no positive magnitude falls back to three decimals.
    ///
    /// The guard reads `!step.is_finite() || step <= 0.0`, which is the same
    /// set the earlier `!(step > 0.0) || !step.is_finite()` rejected — the
    /// finiteness half is what catches NaN either way. This pins that: a bare
    /// `step <= 0.0` on its own would let NaN through to
    /// `(-NaN.log10()).ceil().clamp(..)`, whose `as usize` cast is 0, and
    /// every tick on the axis would lose its decimals at once.
    ///
    /// The test is over magnitudes because the function is: it takes `.abs()`
    /// before the guard, so a negative step is an ordinary step read
    /// backwards, not a rejected one.
    #[test]
    fn a_step_with_no_positive_magnitude_keeps_the_default_decimals() {
        for step in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0] {
            assert_eq!(
                step_decimals(step),
                3,
                "a step of {step} did not fall back to the default"
            );
        }
        // The ordinary path is untouched, in either direction.
        assert_eq!(step_decimals(0.25), 1);
        assert_eq!(step_decimals(-0.25), 1);
        assert_eq!(step_decimals(20.0), 0);
    }

    #[test]
    fn si_formatting() {
        assert_eq!(fmt_si_significant(2.0e-6, "s", 4), "2 µs");
        assert_eq!(fmt_si_significant(10.4e6, "Hz", 3), "10.4 MHz");
        assert_eq!(fmt_si_significant(0.0, "V", 3), "0 V");
        assert_eq!(fmt_si_signed(1.5, "dB", 4), "+1.5 dB");
        assert_eq!(fmt_si_signed(-0.6, "dB", 4), "\u{2212}0.6 dB");
        assert_eq!(fmt_si_signed(0.0, "dB", 4), "0 dB");
    }

    #[test]
    fn tick_labels_trim() {
        assert_eq!(tick_label(200_000.0), "200k");
        assert_eq!(tick_label(0.4), "0.4");
        assert_eq!(tick_label(-20.0), "−20");
        assert_eq!(tick_label(0.0), "0");
        assert_eq!(tick_label(1.0e9), "1G");
    }

    #[test]
    fn the_si_ladder_reaches_yotta_and_yocto_in_both_directions() {
        assert_eq!(tick_label(1.0e12), "1T");
        assert_eq!(tick_label(2.5e12), "2.5T");
        assert_eq!(tick_label(1.0e-15), "1f");
        assert_eq!(tick_label(4.7e-18), "4.7a");
        assert_eq!(fmt_si_significant(3.3e12, "Hz", 4), "3.3 THz");
        assert_eq!(fmt_si_significant(2.0e-18, "F", 4), "2 aF");
        // Past tera and atto the ladder carries on through the prefixes the
        // reference tools print, rather than parking the value on the last.
        assert_eq!(fmt_si_significant(2.402_879e-20, "V", 7), "24.02879 zV");
        assert_eq!(fmt_si_significant(3.1e-24, "C", 4), "3.1 yC");
        assert_eq!(fmt_si_significant(4.2e15, "Hz", 4), "4.2 PHz");
        assert_eq!(fmt_si_significant(1.5e18, "Hz", 4), "1.5 EHz");
        assert_eq!(fmt_si_significant(6.0e21, "W", 4), "6 ZW");
        assert_eq!(fmt_si_significant(7.5e24, "W", 4), "7.5 YW");
        assert_eq!(tick_label(5.0e-21), "5z");
        assert_eq!(tick_label(-2.0e15), "−2P");
        assert_eq!(si_tick_label(1.0e-24, "s", 2), "1 ys");
    }

    /// Below yocto and above yotta the mantissa stays in 1‥1000 and the
    /// decade moves into an exponent on the number, as Cadence's suffix
    /// notation leaves the `e` and exponent once its suffixes run out. A value
    /// is never parked on the last prefix with a run of leading zeros.
    #[test]
    fn past_the_ladder_a_value_states_its_decade_as_an_exponent() {
        assert_eq!(fmt_si_significant(2.402_879e-26, "V", 7), "24.02879e−27 V");
        assert_eq!(fmt_si_significant(2.402_879e-26, "V", 7), "24.02879e−27 V");
        assert_eq!(fmt_si_significant(-5.0e-30, "A", 4), "−5e−30 A");
        assert_eq!(fmt_si_significant(3.0e27, "Hz", 4), "3e27 Hz");
        assert_eq!(fmt_si_significant(4.56e31, "Hz", 4), "45.6e30 Hz");
        assert_eq!(fmt_si_significant(2.5e-28, "F", 4), "250e−30 F");
        assert_eq!(fmt_si_significant(2.5e-28, "", 4), "250e−30");
        // A tick signs its exponent as it signs its value.
        assert_eq!(tick_label(-3.0e-27), "−3e−27");
        assert_eq!(si_tick_label(3.0e-27, "V", 2), "3e−27 V");
        assert_eq!(tick_with_unit(&tick_label(3.0e-27), "V"), "3e−27 V");
        assert_eq!(offset_anchor_label(2.0e-27, "s", 1.0e-33), "+2e−27 s");
        // Rounding past 1000 promotes across the ladder's ends both ways.
        assert_eq!(fmt_si_significant(999.999_99e-27, "V", 4), "1.000 yV");
        assert_eq!(fmt_si_significant(999.999_99e24, "V", 4), "1.000e27 V");
        // The far ends of the double stay finite and bounded.
        assert_eq!(
            fmt_si_significant(f64::MIN_POSITIVE, "V", 4),
            "22.25e−309 V"
        );
        assert_eq!(fmt_si_significant(5.0e-324, "V", 2), "4.9e−324 V");
        assert_eq!(fmt_si_significant(f64::MAX, "V", 4), "179.8e306 V");
        assert_eq!(fmt_si_significant(f64::INFINITY, "V", 4), "inf V");
        assert_eq!(tick_label_with_step(f64::NAN, 1.0), "NaN");
    }

    #[test]
    fn a_mantissa_that_rounds_up_promotes_its_prefix() {
        // 999.9999 k is 1 M. Printing it under the smaller prefix reads as a
        // decade error on the axis.
        assert_eq!(tick_label(999_999.9), "1M");
        assert_eq!(tick_label(0.999_999_5), "1");
        assert_eq!(tick_label(999.999_9e-6), "1m");
        assert_eq!(fmt_si_significant(999_999.9, "Hz", 4), "1.000 MHz");
        assert_eq!(fmt_si_significant(999.999_99, "V", 4), "1.000 kV");
    }

    /// Every tick on an axis carries the same prefix, across six decades of
    /// axis span and either side of a prefix boundary.
    #[test]
    fn an_axis_carries_one_prefix() {
        let ticks = |values: &[f64], step: f64| axis_tick_labels(values, step);
        assert_eq!(
            ticks(&[0.0, 0.05, 0.1, 0.15], 0.05),
            ["0", "50m", "100m", "150m"]
        );
        assert_eq!(
            ticks(&[999.8, 999.9, 1000.0, 1000.1, 1000.2], 0.1),
            ["999.8", "999.9", "1000", "1000.1", "1000.2"]
        );
        assert_eq!(
            ticks(&[200.0, 400.0, 600.0, 800.0, 1000.0], 200.0),
            ["200", "400", "600", "800", "1k"]
        );
        assert_eq!(
            ticks(&[-1.0, -0.5, 0.0, 0.5, 1.0], 0.5),
            ["\u{2212}1", "\u{2212}0.5", "0", "0.5", "1"]
        );
        for decade in -9..=3 {
            let step = 2.0 * 10f64.powi(decade);
            let values = (0..=5)
                .map(|index| f64::from(index) * step)
                .collect::<Vec<_>>();
            let labels = ticks(&values, step);
            // Every tick but one the next prefix up names whole.
            let prefixes = labels[1..labels.len() - 1]
                .iter()
                .map(|label| label.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.'))
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(prefixes.len(), 1, "{labels:?}");
            for label in &labels {
                let digits = label.trim_end_matches(|c: char| !c.is_ascii_digit());
                assert!(digits.parse::<f64>().unwrap() < 1_000.0, "{labels:?}");
            }
        }
        assert_eq!(
            offset_tick_labels(&[-0.0002, 0.0, 0.0002], 0.0002),
            ["\u{2212}200\u{b5}", "0", "+200\u{b5}"]
        );
    }

    /// A current axis that reaches a milliampere names its top tick 1 m, as
    /// its other ticks stay in microamperes; a window just across a
    /// thousand keeps its digits.
    #[test]
    fn a_tick_that_reaches_the_next_prefix_reads_in_it() {
        let values = (-4..=5)
            .map(|index| f64::from(index) * 2.0e-4)
            .collect::<Vec<_>>();
        let labels = axis_tick_labels(&values, 2.0e-4);
        assert_eq!(labels.last().unwrap(), "1m");
        assert_eq!(labels[labels.len() - 2], "800\u{b5}");
        assert_eq!(labels[0], "\u{2212}800\u{b5}");
        assert_eq!(
            axis_tick_labels(&[999.9, 1000.0, 1000.1], 0.1),
            ["999.9", "1000", "1000.1"]
        );
        // An axis its largest tick's own prefix states as briefly keeps it.
        assert_eq!(
            axis_tick_labels(&[-1.0e-3, -5.0e-4, 0.0, 5.0e-4, 1.0e-3], 5.0e-4),
            ["\u{2212}1m", "\u{2212}0.5m", "0", "0.5m", "1m"]
        );
    }

    #[test]
    fn tick_precision_follows_the_step_so_neighbours_differ() {
        assert_eq!(tick_label_with_step(1.000_2, 2.0e-4), "1.0002");
        assert_eq!(tick_label_with_step(1.000_4, 2.0e-4), "1.0004");
        // An ordinary axis is unchanged: the extra places are trimmed.
        assert_eq!(tick_label_with_step(0.4, 0.2), "0.4");
        assert_eq!(tick_label_with_step(200_000.0, 50_000.0), "200k");
        assert_eq!(tick_label_with_step(0.0, 0.25), "0");
        // No step to derive from falls back to the fixed rendering.
        assert_eq!(tick_label_with_step(0.4, 0.0), "0.4");
    }

    #[test]
    fn an_offset_axis_states_its_anchor_once_and_its_ticks_as_deltas() {
        assert_eq!(offset_anchor_label(1.0e-3, "s", 2.0e-8), "+1 ms");
        assert_eq!(offset_anchor_label(-2.5e-6, "s", 1.0e-11), "−2.5 µs");
        assert_eq!(offset_anchor_label(1.000_03e-3, "s", 1.0e-8), "+1.00003 ms");
        assert_eq!(
            offset_tick_labels(&[-4.0e-8, 0.0, 2.0e-8], 2.0e-8),
            ["−40n", "0", "+20n"]
        );
    }

    #[test]
    fn offset_notation_engages_only_where_absolute_labels_stop_working() {
        assert!(!wants_offset_notation(0.0, 1.0));
        assert!(!wants_offset_notation(0.999, 1.001));
        assert!(wants_offset_notation(1.0 - 1.0e-5, 1.0 + 1.0e-5));
        assert!(wants_offset_notation(1.0e-3, 1.0e-3 + 1.0e-9));
        // A window straddling zero has no common part to state.
        assert!(!wants_offset_notation(-1.0, 1.0));
    }

    /// A rounding that carries into the next decade prints exactly the digits
    /// asked for, under the prefix it carried into.
    #[test]
    fn a_rounding_carry_keeps_the_digit_count() {
        assert_eq!(fmt_si_significant(999.95, "V", 4), "1.000 kV");
        assert_eq!(fmt_si_significant(9.999_99, "V", 4), "10.00 V");
        assert_eq!(fmt_si_significant(99.996, "A", 4), "100.0 A");
        assert_eq!(fmt_si_significant(1.0e-24, "V", 3), "1 yV");
        assert_eq!(fmt_si_significant(0.000_999_99, "", 3), "0.00100");
        assert_eq!(fmt_si_significant(999_999.96, "", 7), "1.000000e6");
    }

    /// Negative zero is zero, and every negative value carries the true
    /// minus the axes beside it are drawn with.
    #[test]
    fn negative_values_take_the_true_minus_and_negative_zero_takes_none() {
        assert_eq!(fmt_si_significant(-0.0, "V", 4), "0 V");
        assert_eq!(fmt_si_significant(-0.0, "", 7), "0");
        assert_eq!(fmt_si_significant(-3.0e-3, "V", 4), "\u{2212}3 mV");
        assert_eq!(fmt_si_significant(-179.999_9, "°", 7), "\u{2212}179.9999°");
        assert_eq!(
            fmt_si_significant(-4.1e-12, "", 4),
            "\u{2212}4.1e\u{2212}12"
        );
        assert!(!fmt_si_significant(-12.5, "dB", 4).contains('-'));
    }

    /// A ratio, a damping factor and a percentage are never given a prefix,
    /// and nor is a unit that already names its decade.
    #[test]
    fn a_dimensionless_value_takes_no_prefix() {
        assert_eq!(fmt_si_significant(0.4266, "sigma", 4), "0.4266 sigma");
        assert_eq!(fmt_si_significant(0.62, "UI", 4), "0.62 UI");
        assert_eq!(fmt_si_significant(0.002, "1/V", 4), "0.002 1/V");
        assert_eq!(fmt_si_significant(0.316_227_8, "", 5), "0.31623");
        assert_eq!(fmt_si_significant(0.707, "", 4), "0.707");
        assert_eq!(fmt_si_significant(12.5, "%", 4), "12.5 %");
        assert_eq!(fmt_si_significant(-0.999_990_2, "", 7), "\u{2212}0.9999902");
        assert_eq!(fmt_si_significant(4.1e-12, "", 4), "4.1e\u{2212}12");
        assert_eq!(fmt_si_significant(1.234_567e7, "", 4), "12.35e6");
        assert_eq!(fmt_si_significant(-42.0, "dBc", 4), "\u{2212}42 dBc");
        assert_eq!(fmt_si_significant(1786.13, "nV/√Hz", 6), "1786.13 nV/√Hz");
        assert_eq!(fmt_si_significant(2.5e-3, "/V", 4), "0.0025 /V");
        // A unit keeps its prefix.
        assert_eq!(fmt_si_significant(0.707, "V", 4), "707 mV");
    }

    /// Zeros are digits of a rounded value, never padding of an exact one.
    #[test]
    fn zeros_are_printed_only_where_the_rounding_resolved_them() {
        assert_eq!(fmt_si_significant(4.1, "V", 7), "4.1 V");
        assert_eq!(fmt_si_significant(4.1e-3, "V", 7), "4.1 mV");
        assert_eq!(fmt_si_significant(1.5e-15, "F", 7), "1.5 fF");
        assert_eq!(fmt_si_significant(1.799_999_9, "V", 4), "1.800 V");
        assert_eq!(fmt_si_significant(1.000_2, "V", 4), "1.000 V");
        assert_eq!(fmt_si_significant(1.000_2, "V", 7), "1.0002 V");
        assert_eq!(fmt_si_significant(1.49, "V", 3), "1.49 V");
    }

    /// Free text prints the digits the value has, not the digits the
    /// preference allows: `4.100000 V` is a formatter default, not a reading.
    #[test]
    fn free_text_drops_the_zeros_that_only_pad_it_out() {
        assert_eq!(fmt_si_significant(4.1, "V", 7), "4.1 V");
        assert_eq!(fmt_si_significant(0.9, "V", 7), "900 mV");
        assert_eq!(fmt_si_significant(0.0, "A", 7), "0 A");
        // Digits the value does have are kept, to the stated precision.
        assert_eq!(fmt_si_significant(12.345_678, "V", 7), "12.34568 V");
        assert_eq!(fmt_si_significant(96.861_12e-6, "A", 4), "96.86 µA");
        // A bound is the same readout at the compact precision.
        assert_eq!(fmt_si_significant(1.0e7, "Hz", 4), "10 MHz");
    }

    /// A decibel and a plane angle in degrees read to the hundredth, at any
    /// digits preference, and a value within half a step of zero reads
    /// `0.00`, never `−0.00` or a run of exponent digits.
    #[test]
    fn decibels_and_degrees_read_to_the_hundredth() {
        assert_eq!(fmt_reading(-3.0002, "dB", 4), "\u{2212}3.00 dB");
        assert_eq!(fmt_reading(-40.0, "dB", 4), "\u{2212}40.00 dB");
        assert_eq!(fmt_reading(-0.004, "dB", 4), "0.00 dB");
        assert_eq!(fmt_reading(-43.43e-9, "dB", 4), "0.00 dB");
        assert_eq!(fmt_reading(-91.87e-6, "dB", 7), "0.00 dB");
        assert_eq!(fmt_reading(-42.0, "dBc", 4), "\u{2212}42.00 dBc");
        assert_eq!(fmt_reading(-120.456, "dBc/Hz", 4), "\u{2212}120.46 dBc/Hz");
        assert_eq!(fmt_reading(-20.0, "dB/dec", 4), "\u{2212}20.00 dB/dec");
        assert_eq!(fmt_reading(45.0, "°", 4), "45.00°");
        assert_eq!(fmt_reading(-179.999_9, "°", 7), "\u{2212}180.00°");
        assert_eq!(fmt_column_reading(-3.0002, "dB", 7), "\u{2212}3.00 dB");
        assert_eq!(fmt_column_reading(45.678, "°", 3), "45.68°");
        assert_eq!(fmt_signed_reading(1.5, "dB", 4), "+1.50 dB");
        assert_eq!(fmt_signed_reading(-0.6, "dB", 4), "\u{2212}0.60 dB");
        assert_eq!(fmt_signed_reading(0.001, "dB", 4), "0.00 dB");
        assert_eq!(fmt_reading(f64::NEG_INFINITY, "dB", 4), "\u{2212}inf dB");
        // Every other unit keeps its significant digits.
        assert_eq!(fmt_reading(0.604_83, "V", 4), "604.8 mV");
        assert!(!reads_at_fixed_places("°C"));
        assert!(!reads_at_fixed_places("rad"));
    }

    /// A plane angle stands against its number; a temperature in degrees
    /// keeps its space.
    #[test]
    fn a_plane_angle_takes_no_space_and_a_temperature_keeps_it() {
        assert_eq!(fmt_si_significant(45.0, "°", 4), "45°");
        assert_eq!(fmt_si_significant(3.6, "°/kHz", 4), "3.6°/kHz");
        assert_eq!(fmt_si_significant(27.0, "°C", 4), "27 °C");
        assert_eq!(fmt_column_reading(27.0, "°C", 4), "27.00 °C");
        assert_eq!(fmt_measured(45.0, Some("deg"), 4), "45.00°");
        assert_eq!(fmt_measured(27.0, Some("degC"), 4), "27.00 °C");
    }

    /// A complex value names its unit once, both parts under one prefix and
    /// at the larger part's resolution.
    #[test]
    fn a_complex_value_names_its_unit_once() {
        assert_eq!(fmt_complex(1.2, 3.4, "V", 4), "1.200 + 3.400j V");
        assert_eq!(fmt_complex(1.2, -3.4, "V", 4), "1.200 \u{2212} 3.400j V");
        assert_eq!(
            fmt_complex(-1.2e-3, 3.4e-3, "A", 4),
            "\u{2212}1.200 + 3.400j mA"
        );
        assert_eq!(fmt_complex(604.8e-3, 1.0e-9, "V", 4), "604.8 + 0.0j mV");
        assert_eq!(fmt_complex(0.540, -0.452, "", 3), "0.540 \u{2212} 0.452j");
        assert_eq!(fmt_complex(50.0, 25.0, "Ω", 4), "50.00 + 25.00j Ω");
        assert_eq!(fmt_complex(0.0, -0.0, "V", 4), "0 + 0j V");
        assert_eq!(fmt_complex(1.0, -1.0e-9, "V", 4), "1.000 + 0.000j V");
    }

    /// Values read side by side take one resolution, the largest part's at
    /// the reader's digits, and one prefix; past four decades apart, each
    /// reads in engineering notation at those digits instead.
    #[test]
    fn values_side_by_side_read_at_one_resolution() {
        let shared = SharedReading::new(0.25, 0.01, "", 4);
        assert_eq!(shared.complex(0.25, 0.0, ""), "0.2500 + 0.0000j");
        assert_eq!(shared.complex(0.01, -0.0, ""), "0.0100 + 0.0000j");
        assert_eq!(shared.reading(-0.01, ""), "\u{2212}0.0100");
        let ohms = SharedReading::new(1_200.0, 50.0, "\u{3a9}", 4);
        assert_eq!(ohms.reading(50.0, "\u{3a9}"), "0.050 k\u{3a9}");
        assert_eq!(
            ohms.complex(1_200.0, 50.0, "\u{3a9}"),
            "1.200 + 0.050j k\u{3a9}"
        );
        // Four decades apart still read at one resolution; past four, not.
        assert!(matches!(
            SharedReading::new(1.0, 1.0e-4, "", 4),
            SharedReading::Fixed { decimals: 3, .. }
        ));
        let apart = SharedReading::new(0.25, 1.2e-5, "", 4);
        assert_eq!(
            apart.complex(0.25, -1.2e-5, ""),
            "250.0e\u{2212}3 \u{2212} 12.00e\u{2212}6j"
        );
        assert_eq!(apart.complex(1.0, 0.0, ""), "1.000 + 0j");
        assert_eq!(
            SharedReading::new(2.0e6, 10.0, "\u{3a9}", 4).complex(2.0e6, -10.0, "\u{3a9}"),
            "2.000e6 \u{2212} 10.00j \u{3a9}"
        );
        assert_eq!(
            SharedReading::new(0.0, 0.0, "", 4).complex(0.0, 0.0, ""),
            "0 + 0j"
        );
    }

    /// In a column or a readout list a value keeps the reader's digits, so
    /// its decimal points fall in line; free text may drop them.
    #[test]
    fn a_column_keeps_the_readers_digits() {
        assert_eq!(fmt_column_reading(8.3e-9, "s", 4), "8.300 ns");
        assert_eq!(fmt_column_reading(6.917e-9, "s", 4), "6.917 ns");
        assert_eq!(fmt_si_significant(8.3e-9, "s", 4), "8.3 ns");
        assert_eq!(fmt_column_reading(4.1, "V", 7), "4.100000 V");
        assert_eq!(fmt_column_reading(0.9, "V", 4), "900.0 mV");
        assert_eq!(fmt_column_reading(1.0e7, "Hz", 4), "10.00 MHz");
        assert_eq!(fmt_column_reading(999.95, "V", 4), "1.000 kV");
        assert_eq!(fmt_column_reading(0.5, "", 4), "0.5000");
        assert_eq!(fmt_column_reading(-0.0, "V", 4), "0 V");
        assert_eq!(fmt_column_reading(-2.5e-3, "V", 3), "\u{2212}2.50 mV");
    }

    /// A measured value names the unit its measurement implies, at the
    /// reader's digits; an unknown unit stays bare and keeps its prefix, and
    /// a dimensionless one takes none.
    #[test]
    fn a_measured_value_names_its_unit() {
        assert_eq!(fmt_measured(6.917e-9, Some("s"), 4), "6.917 ns");
        assert_eq!(fmt_measured(8.3e-9, Some("s"), 4), "8.300 ns");
        assert_eq!(fmt_measured(1.25e6, Some("V/s"), 4), "1.250 MV/s");
        assert_eq!(fmt_measured(2.0e3, Some("ohm"), 4), "2.000 kΩ");
        assert_eq!(fmt_measured(3.3e-9, Some("V*s"), 4), "3.300 nV·s");
        assert_eq!(fmt_measured(-3.0002, Some("dB"), 4), "\u{2212}3.00 dB");
        assert_eq!(fmt_measured(0.25, Some("1"), 4), "0.2500");
        assert_eq!(fmt_measured(6.917e-9, None, 4), "6.917n");
        assert_eq!(fmt_measured(-1.5e3, None, 3), "\u{2212}1.50k");
        assert_eq!(fmt_measured(42.0, None, 4), "42.00");
    }

    /// A reading whose unit takes a prefix but was printed without one leaves
    /// the prefix slot blank; a prefixed one fills it, and a unit that never
    /// takes a prefix has none.
    #[test]
    fn a_bare_unit_leaves_its_prefix_slot_blank() {
        assert!(leaves_prefix_slot_blank(&fmt_column_reading(1.144, "V", 4)));
        assert!(!leaves_prefix_slot_blank(&fmt_column_reading(
            0.604_8, "V", 4
        )));
        assert!(leaves_prefix_slot_blank("50.00 \u{3a9}"));
        assert!(!leaves_prefix_slot_blank("1.000 k\u{3a9}"));
        assert!(leaves_prefix_slot_blank("1.000 m"));
        assert!(!leaves_prefix_slot_blank("1.000 mm"));
        assert!(!leaves_prefix_slot_blank("2.000 MV/s"));
        assert!(leaves_prefix_slot_blank("2.000 V/s"));
        assert!(!leaves_prefix_slot_blank("1786 nV/\u{221a}Hz"));
        assert!(leaves_prefix_slot_blank("1.200 + 3.400j V"));
        assert!(!leaves_prefix_slot_blank("0.540 \u{2212} 0.452j"));
        assert!(!leaves_prefix_slot_blank("\u{2212}3.00 dB"));
        assert!(!leaves_prefix_slot_blank("45.00\u{b0}"));
        assert!(!leaves_prefix_slot_blank("12.50 %"));
        assert!(!leaves_prefix_slot_blank("0.5000"));
        assert!(!leaves_prefix_slot_blank("6.917n"));
    }
}
