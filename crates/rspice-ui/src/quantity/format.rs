use std::f64::consts::TAU;

use super::{
    AngleDisplay, CopiedValueFormat, FrequencyDisplay, LayoutCoordinateDisplay, LayoutDatabaseUnit,
    QuantityPresentationPolicy, TemperatureDisplay, UnitSystem,
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

    /// Affine axis-label transform for temperatures stored in Celsius.
    #[must_use]
    pub const fn celsius_axis_transform(self) -> (f64, f64, &'static str) {
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

    /// Format an arbitrary SI value with an engineering prefix.
    #[must_use]
    pub fn format_si_value(self, value_si: f64, si_unit: &str, fractional_digits: usize) -> String {
        engineering_value(value_si, si_unit, fractional_digits, false)
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
        match self.unit_system {
            UnitSystem::ImperialLayout => {
                format!("{:.6} mil · {database_units:.0} DBU", metres / 25.4e-6)
            }
            UnitSystem::Si => engineering_value(metres, "m", 3, false),
            UnitSystem::MixedEngineering => match self.layout_coordinate_display {
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
            },
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
    pub fn copy_temperature(self, kelvin: f64) -> String {
        match self.copied_value_format {
            CopiedValueFormat::ScientificNotationWithSiUnit => scientific_value(kelvin, "K"),
            CopiedValueFormat::EngineeringNotationWithUnit => match self.temperature_display {
                TemperatureDisplay::Celsius => full_fixed_value(kelvin - 273.15, "°C"),
                TemperatureDisplay::Kelvin => full_fixed_value(kelvin, "K"),
                TemperatureDisplay::Fahrenheit => {
                    full_fixed_value((kelvin - 273.15) * 1.8 + 32.0, "°F")
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
                AngleDisplay::Degrees => full_fixed_value(radians.to_degrees(), "°"),
                AngleDisplay::Radians => full_fixed_value(radians, "rad"),
            },
        }
    }
}

fn fixed_value(value: f64, unit: &str, fractional_digits: usize) -> String {
    format!("{value:.fractional_digits$} {unit}")
}

fn full_fixed_value(value: f64, unit: &str) -> String {
    format!("{value:.17} {unit}")
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
            full_fixed_value(value, unit)
        } else {
            fixed_value(value, unit, fractional_digits)
        };
    }

    const PREFIXES: [(i32, &str); 11] = [
        (-18, "a"),
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
    let exponent = ((value.abs().log10().floor() as i32).div_euclid(3) * 3).clamp(-18, 12);
    let prefix = PREFIXES
        .iter()
        .find_map(|(candidate, prefix)| (*candidate == exponent).then_some(*prefix))
        .unwrap_or("");
    let scaled = value / 10_f64.powi(exponent);
    if full_precision {
        format!("{scaled:.17} {prefix}{unit}")
    } else {
        format!("{scaled:.fractional_digits$} {prefix}{unit}")
    }
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
