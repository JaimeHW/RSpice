//! Formatting a quantity for display.
//!
//! Chooses the engineering prefix and significant-figure count a value is
//! shown at. The policy is explicit so the same value reads identically in
//! a property field, a cursor readout, and a printed report.

use std::f64::consts::TAU;

use super::{
    AngleDisplay, CopiedValueFormat, FrequencyDisplay, LayoutCoordinateDisplay, LayoutDatabaseUnit,
    QuantityPresentationPolicy, TemperatureDisplay,
};

impl QuantityPresentationPolicy {
    /// Affine axis-label transform for values stored in hertz.
    #[must_use]
    pub const fn frequency_axis_transform(self) -> (f64, f64, &'static str) {
        match self.frequency_display {
            FrequencyDisplay::HertzEngineering => (1.0, 0.0, "Hz"),
            FrequencyDisplay::RadiansPerSecond => (TAU, 0.0, "rad/s"),
        }
    }

    /// Affine axis-label transform for phase arrays stored in degrees.
    #[must_use]
    pub const fn degree_axis_transform(self) -> (f64, f64, &'static str) {
        match self.angle_display {
            AngleDisplay::Degrees => (1.0, 0.0, "°"),
            AngleDisplay::Radians => (std::f64::consts::PI / 180.0, 0.0, "rad"),
        }
    }

    /// Affine transform from degrees Celsius, the unit SPICE states device
    /// and corner temperatures in, to the chosen temperature display.
    #[must_use]
    pub const fn celsius_display_transform(self) -> (f64, f64, &'static str) {
        match self.temperature_display {
            TemperatureDisplay::Celsius => (1.0, 0.0, "°C"),
            TemperatureDisplay::Kelvin => (1.0, 273.15, "K"),
            TemperatureDisplay::Fahrenheit => (1.8, 32.0, "°F"),
        }
    }

    /// Format hertz without changing the stored value.
    #[must_use]
    pub fn format_frequency(self, hertz: f64, fractional_digits: usize) -> String {
        match self.frequency_display {
            FrequencyDisplay::HertzEngineering => {
                engineering_value(hertz, "Hz", fractional_digits, false)
            }
            FrequencyDisplay::RadiansPerSecond => {
                engineering_value(hertz * TAU, "rad/s", fractional_digits, false)
            }
        }
    }

    /// Format kelvin without changing the stored value.
    #[must_use]
    pub fn format_temperature(self, kelvin: f64, fractional_digits: usize) -> String {
        let (value, unit) = match self.temperature_display {
            TemperatureDisplay::Celsius => (kelvin - 273.15, "°C"),
            TemperatureDisplay::Kelvin => (kelvin, "K"),
            TemperatureDisplay::Fahrenheit => ((kelvin - 273.15) * 1.8 + 32.0, "°F"),
        };
        fixed_value(value, unit, fractional_digits)
    }

    /// Format radians without changing the stored value.
    #[must_use]
    pub fn format_angle(self, radians: f64, fractional_digits: usize) -> String {
        let (value, unit) = match self.angle_display {
            AngleDisplay::Degrees => (radians.to_degrees(), "°"),
            AngleDisplay::Radians => (radians, "rad"),
        };
        fixed_value(value, unit, fractional_digits)
    }

    /// Format an exact PDK-layout coordinate. `database_units` remains the
    /// canonical coordinate; the authoritative DBU supplies its physical
    /// length and is never inferred from a display preference.
    #[must_use]
    pub fn format_layout_coordinate(
        self,
        database_units: f64,
        database_unit: LayoutDatabaseUnit,
    ) -> String {
        let metres = database_units * database_unit.metres();
        match self.layout_coordinate_display {
            LayoutCoordinateDisplay::Nanometres => format!("{:.3} nm", metres * 1e9),
            LayoutCoordinateDisplay::DatabaseUnits => format!("{database_units:.0} DBU"),
            LayoutCoordinateDisplay::MicrometresWithDatabaseUnitRemainder => {
                let units_per_micrometre = 1e-6 / database_unit.metres();
                let whole_micrometres = (database_units / units_per_micrometre).trunc();
                let remainder = database_units - whole_micrometres * units_per_micrometre;
                if remainder.abs() < 1e-9 {
                    format!("{whole_micrometres:.0} µm")
                } else {
                    let separator = if remainder.is_sign_negative() {
                        '-'
                    } else {
                        '+'
                    };
                    format!(
                        "{whole_micrometres:.0} µm {separator} {:.0} DBU",
                        remainder.abs()
                    )
                }
            }
        }
    }

    /// Copy an arbitrary SI value with full precision and an explicit unit.
    #[must_use]
    pub fn copy_si_value(self, value_si: f64, si_unit: &str) -> String {
        match self.copied_value_format {
            CopiedValueFormat::EngineeringNotationWithUnit => {
                engineering_value(value_si, si_unit, 17, true)
            }
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(value_si, si_unit),
        }
    }

    /// Copy a value whose unit already states its scale, such as `nV/√Hz`.
    ///
    /// Engineering notation must not prefix such a unit a second time: the
    /// value is written as it stands, never as `knV/√Hz`.
    #[must_use]
    pub fn copy_scaled_unit_value(self, value: f64, unit: &str) -> String {
        match self.copied_value_format {
            CopiedValueFormat::EngineeringNotationWithUnit => full_precision_value(value, unit),
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(value, unit),
        }
    }

    /// Copy frequency. Scientific mode always emits stored SI hertz;
    /// engineering mode follows the chosen frequency presentation.
    #[must_use]
    pub fn copy_frequency(self, hertz: f64) -> String {
        match self.copied_value_format {
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(hertz, "Hz"),
            CopiedValueFormat::EngineeringNotationWithUnit => match self.frequency_display {
                FrequencyDisplay::HertzEngineering => engineering_value(hertz, "Hz", 17, true),
                FrequencyDisplay::RadiansPerSecond => {
                    engineering_value(hertz * TAU, "rad/s", 17, true)
                }
            },
        }
    }

    /// Copy temperature. Scientific mode always emits stored SI kelvin.
    #[must_use]
    #[cfg(test)]
    pub fn copy_temperature(self, kelvin: f64) -> String {
        match self.copied_value_format {
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(kelvin, "K"),
            CopiedValueFormat::EngineeringNotationWithUnit => match self.temperature_display {
                TemperatureDisplay::Celsius => full_precision_value(kelvin - 273.15, "°C"),
                TemperatureDisplay::Kelvin => full_precision_value(kelvin, "K"),
                TemperatureDisplay::Fahrenheit => {
                    full_precision_value((kelvin - 273.15) * 1.8 + 32.0, "°F")
                }
            },
        }
    }

    /// Copy angle. Scientific mode always emits stored SI radians.
    #[must_use]
    pub fn copy_angle(self, radians: f64) -> String {
        match self.copied_value_format {
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(radians, "rad"),
            CopiedValueFormat::EngineeringNotationWithUnit => match self.angle_display {
                AngleDisplay::Degrees => full_precision_value(radians.to_degrees(), "°"),
                AngleDisplay::Radians => full_precision_value(radians, "rad"),
            },
        }
    }
}

fn fixed_value(value: f64, unit: &str, fractional_digits: usize) -> String {
    format!("{value:.fractional_digits$} {unit}")
}

/// Full-precision text for a value written against its unit as it stands.
///
/// Seventeen fixed decimals hold all seventeen significant digits a double
/// needs from 0.1 up, so an ordinary value keeps that form. Under 0.1 they
/// run out of digits, copying 1e-18 as zero; from 1e15 up the integer part
/// alone carries the double's precision, so the decimals are padding and a
/// reader counts digits to find the decade. Outside that span the shortest
/// mantissa and exponent that read back as the same double say it outright.
fn full_precision_value(value: f64, unit: &str) -> String {
    if value == 0.0 || !value.is_finite() || (0.1..1e15).contains(&value.abs()) {
        format!("{value:.17} {unit}")
    } else {
        format!("{value:e} {unit}")
    }
}

fn scientific_value(value: f64, unit: &str) -> String {
    format!("{value:.17e} {unit}")
}

fn engineering_value(
    value: f64,
    unit: &str,
    fractional_digits: usize,
    full_precision: bool,
) -> String {
    if !value.is_finite() || value == 0.0 {
        return if full_precision {
            full_precision_value(value, unit)
        } else {
            fixed_value(value, unit, fractional_digits)
        };
    }

    // Femto is the bottom of the ladder. The decade under it was written `a`,
    // a spelling no parser in the toolchain reads back — so a value that lands
    // there keeps its own exponent, which the copy path needs anyway: pinning
    // it against a femto prefix would drive a sub-femto mantissa's digits off
    // the end of the fixed decimals and copy the value as zero.
    let decade = (value.abs().log10().floor() as i32).div_euclid(3) * 3;
    if decade < -15 {
        return format!("{value:e} {unit}");
    }

    const PREFIXES: [(i32, &str); 10] = [
        (-15, "f"),
        (-12, "p"),
        (-9, "n"),
        (-6, "µ"),
        (-3, "m"),
        (0, ""),
        (3, "k"),
        (6, "M"),
        (9, "G"),
        (12, "T"),
    ];
    let exponent = decade.min(12);
    let prefix = PREFIXES
        .iter()
        .find_map(|(candidate, prefix)| (*candidate == exponent).then_some(*prefix))
        .unwrap_or("");
    if full_precision {
        return format!("{} {prefix}{unit}", mantissa_at_decade(value, exponent));
    }
    let scaled = value / 10_f64.powi(exponent);
    format!("{scaled:.fractional_digits$} {prefix}{unit}")
}

/// `value` written against `10^decade` with seventeen decimals, the digits
/// taken from the value itself.
///
/// The value's own decimal expansion, correctly rounded, is written first and
/// its point then moved as text, so mantissa × 10^decade reads back as the
/// same double. Dividing by the power of ten first rounded once more, and
/// about one value in six then read back a unit in the last place off.
fn mantissa_at_decade(value: f64, decade: i32) -> String {
    const DECIMALS: i32 = 17;
    let places = usize::try_from(DECIMALS - decade).unwrap_or(0);
    let text = format!("{:.places$}", value.abs());
    let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
    let digits = format!("{whole}{fraction}");
    // Where the point lands once the text is read at 10^decade; at or before
    // the first digit when the value sits just under its decade.
    let point = whole.len() as i64 - i64::from(decade);
    let (whole, fraction) = match usize::try_from(point) {
        Ok(point) if point > 0 => {
            let (whole, fraction) = digits.split_at(point);
            (whole.trim_start_matches('0'), fraction.to_owned())
        }
        _ => {
            let zeros = "0".repeat(point.unsigned_abs() as usize);
            ("", format!("{zeros}{digits}"))
        }
    };
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let whole = if whole.is_empty() { "0" } else { whole };
    format!("{sign}{whole}.{fraction}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantity::{AngleDisplay, CopiedValueFormat, FrequencyDisplay, TemperatureDisplay};

    #[test]
    fn presentation_converts_only_the_returned_text() {
        let hertz = 1_000_000.0;
        let kelvin = 300.0;
        let radians = std::f64::consts::FRAC_PI_2;
        let policy = QuantityPresentationPolicy {
            frequency_display: FrequencyDisplay::RadiansPerSecond,
            temperature_display: TemperatureDisplay::Fahrenheit,
            angle_display: AngleDisplay::Radians,
            ..QuantityPresentationPolicy::default()
        };

        assert_eq!(policy.format_frequency(hertz, 3), "6.283 Mrad/s");
        assert_eq!(policy.format_temperature(kelvin, 2), "80.33 °F");
        assert_eq!(policy.format_angle(radians, 3), "1.571 rad");
        assert_eq!(hertz, 1_000_000.0);
        assert_eq!(kelvin, 300.0);
        assert_eq!(radians, std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn scientific_copy_always_uses_explicit_stored_si_units() {
        let policy = QuantityPresentationPolicy {
            frequency_display: FrequencyDisplay::RadiansPerSecond,
            temperature_display: TemperatureDisplay::Fahrenheit,
            angle_display: AngleDisplay::Degrees,
            copied_value_format: CopiedValueFormat::ScientificNotationWithSiUnit,
            ..QuantityPresentationPolicy::default()
        };

        assert_eq!(policy.copy_frequency(1_000.0), "1.00000000000000000e3 Hz");
        assert_eq!(policy.copy_temperature(300.0), "3.00000000000000000e2 K");
        assert_eq!(policy.copy_angle(0.5), "5.00000000000000000e-1 rad");
    }

    /// Femto is the bottom of the prefix ladder. The decade under it has no
    /// spelling any parser in the toolchain reads back, so a value that lands
    /// there keeps its own exponent rather than being shown against a prefix —
    /// or, worse, copied as a run of zeros under a femto prefix that its
    /// mantissa cannot fill.
    #[test]
    fn a_sub_femto_magnitude_keeps_its_exponent_instead_of_an_unreadable_prefix() {
        let policy = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            ..QuantityPresentationPolicy::default()
        };
        assert_eq!(policy.format_frequency(1e-18, 3), "1e-18 Hz");
        assert_eq!(policy.format_frequency(5e-19, 3), "5e-19 Hz");
        assert_eq!(policy.copy_si_value(1e-18, "V"), "1e-18 V");
        assert_eq!(
            policy.format_frequency(2e-15, 3),
            "2.000 fHz",
            "femto is still a prefix"
        );
    }

    /// Both signs of every decade from 1e-300 to 1e300 at several
    /// mantissas, with zero, subnormals and the extremes.
    fn swept_values() -> Vec<f64> {
        let mut values = vec![
            0.0,
            1.0 / 3.0,
            0.1,
            0.0625,
            0.062_499_999_999_999_99,
            273.15,
            f64::EPSILON,
            f64::MIN_POSITIVE,
            f64::MIN_POSITIVE / 3.0,
            5e-324,
            f64::MAX,
        ];
        for exponent in -300..=300 {
            for mantissa in [1.0, 1.234_567_890_123_456_7, 3.2, 9.999_999_999_999_998] {
                values.push(mantissa * 10_f64.powi(exponent));
            }
        }
        values.iter().flat_map(|value| [*value, -*value]).collect()
    }

    /// Copied values promise full precision, so an engineering copy written
    /// without a prefix reads back as the very double it was made from.
    /// Seventeen fixed decimals carry a double's seventeen significant digits
    /// only from 0.1 up: under it they copied 1e-18 as zero and cut 3.2e-19
    /// down to nothing.
    #[test]
    fn an_unprefixed_copy_reads_back_as_the_same_double() {
        let engineering = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            ..QuantityPresentationPolicy::default()
        };
        let temperature = |temperature_display| QuantityPresentationPolicy {
            temperature_display,
            ..engineering
        };
        let angle = |angle_display| QuantityPresentationPolicy {
            angle_display,
            ..engineering
        };
        let read_back = |copied: &str| -> f64 {
            let (number, _unit) = copied.split_once(' ').expect("a value and its unit");
            number.parse().expect("the copy parses as a number")
        };
        for value in swept_values() {
            let copies = [
                (engineering.copy_scaled_unit_value(value, "nV/√Hz"), value),
                (
                    temperature(TemperatureDisplay::Kelvin).copy_temperature(value),
                    value,
                ),
                (
                    temperature(TemperatureDisplay::Celsius).copy_temperature(value),
                    value - 273.15,
                ),
                (
                    temperature(TemperatureDisplay::Fahrenheit).copy_temperature(value),
                    (value - 273.15) * 1.8 + 32.0,
                ),
                (angle(AngleDisplay::Radians).copy_angle(value), value),
                (
                    angle(AngleDisplay::Degrees).copy_angle(value),
                    value.to_degrees(),
                ),
            ];
            for (copied, shown) in copies {
                if shown.is_finite() {
                    assert_eq!(
                        read_back(&copied).to_bits(),
                        shown.to_bits(),
                        "{shown:e} copied as {copied}"
                    );
                }
            }
        }

        let noise = |value| engineering.copy_scaled_unit_value(value, "nV/√Hz");
        assert_eq!(noise(1e-18), "1e-18 nV/√Hz");
        assert_eq!(noise(-3.2e-19), "-3.2e-19 nV/√Hz");
        assert_eq!(noise(1e300), "1e300 nV/√Hz");
        assert_eq!(noise(0.05), "5e-2 nV/√Hz", "under 0.1 the decimals run out");
        assert_eq!(noise(0.1), "0.10000000000000001 nV/√Hz");
        assert_eq!(noise(1e15), "1e15 nV/√Hz");
        assert_eq!(
            noise(12.5),
            "12.50000000000000000 nV/√Hz",
            "an ordinary value keeps its fixed decimals"
        );
        assert_eq!(
            noise(25_000.0),
            "25000.00000000000000000 nV/√Hz",
            "a unit that states its scale takes no prefix"
        );
        assert_eq!(noise(-0.0), "-0.00000000000000000 nV/√Hz");
        assert_eq!(
            temperature(TemperatureDisplay::Kelvin).copy_temperature(300.0),
            "300.00000000000000000 K"
        );
    }

    /// A prefixed copy reads back as the same double when its mantissa is
    /// taken at the prefix's decade, mantissa × 10^prefix. The mantissa was
    /// the value divided by that power of ten, whose rounding put about one
    /// value in six a unit in the last place off once read back.
    #[test]
    fn a_prefixed_copy_reads_back_as_the_same_double() {
        let engineering = QuantityPresentationPolicy {
            copied_value_format: CopiedValueFormat::EngineeringNotationWithUnit,
            ..QuantityPresentationPolicy::default()
        };
        let radians_per_second = QuantityPresentationPolicy {
            frequency_display: FrequencyDisplay::RadiansPerSecond,
            ..engineering
        };
        let read_back = |copied: &str, unit: &str| -> f64 {
            let (number, scaled_unit) = copied.split_once(' ').expect("a value and its unit");
            let prefix = scaled_unit
                .strip_suffix(unit)
                .expect("the unit ends the copy");
            let decade = match prefix {
                "f" => -15,
                "p" => -12,
                "n" => -9,
                "µ" => -6,
                "m" => -3,
                "" => 0,
                "k" => 3,
                "M" => 6,
                "G" => 9,
                "T" => 12,
                other => panic!("{other:?} is not a prefix"),
            };
            let scaled = if decade == 0 {
                number.to_owned()
            } else {
                format!("{number}e{decade}")
            };
            scaled.parse().expect("the copy parses as a number")
        };
        for value in swept_values() {
            let copies = [
                (engineering.copy_si_value(value, "V"), "V", value),
                (engineering.copy_frequency(value), "Hz", value),
                (
                    radians_per_second.copy_frequency(value),
                    "rad/s",
                    value * TAU,
                ),
            ];
            for (copied, unit, shown) in copies {
                if shown.is_finite() {
                    assert_eq!(
                        read_back(&copied, unit).to_bits(),
                        shown.to_bits(),
                        "{shown:e} copied as {copied}"
                    );
                }
            }
        }

        assert_eq!(
            engineering.copy_si_value(0.008_738_147_087_027_245, "V"),
            "8.73814708702724471 mV"
        );
        assert_eq!(
            engineering.copy_si_value(1.0, "V"),
            "1.00000000000000000 V",
            "an exact value keeps its fixed decimals"
        );
        assert_eq!(
            engineering.copy_si_value(-0.5, "V"),
            "-500.00000000000000000 mV"
        );
    }

    #[test]
    fn layout_presentation_preserves_pdk_database_unit_identity() {
        let dbu = LayoutDatabaseUnit::from_metres(1e-9).unwrap();
        let policy = QuantityPresentationPolicy::default();
        assert_eq!(
            policy.format_layout_coordinate(1_234.0, dbu),
            "1 µm + 234 DBU"
        );

        let policy = QuantityPresentationPolicy {
            layout_coordinate_display: LayoutCoordinateDisplay::DatabaseUnits,
            ..policy
        };
        assert_eq!(policy.format_layout_coordinate(1_234.0, dbu), "1234 DBU");

        let policy = QuantityPresentationPolicy::default();
        assert_eq!(
            policy.format_layout_coordinate(-1_234.0, dbu),
            "-1 µm - 234 DBU"
        );
    }
}
