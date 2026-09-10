//! Engineering units at result-data boundaries.
//!
//! Samples are normalized to the stated canonical unit. Angles use degrees
//! (the result viewer's phase convention); absolute temperatures use kelvin.
//! Missing metadata is not a dimensionless declaration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum UnitDimension {
    Dimensionless,
    Time,
    Frequency,
    Voltage,
    Current,
    Resistance,
    Conductance,
    Power,
    Capacitance,
    Inductance,
    Temperature,
    Angle,
    LogRatio,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EngineeringUnit {
    pub(crate) dimension: UnitDimension,
    scale: f64,
    bias: f64,
}

impl EngineeringUnit {
    pub(crate) fn parse(raw: &str) -> Result<Self, String> {
        let symbol = raw
            .trim()
            .replace(['µ', 'μ'], "u")
            .replace('Ω', "ohm")
            .replace('°', "deg");
        let contract = match symbol.as_str() {
            "s" => (UnitDimension::Time, 1.0),
            "ms" => (UnitDimension::Time, 1e-3),
            "us" => (UnitDimension::Time, 1e-6),
            "ns" => (UnitDimension::Time, 1e-9),
            "ps" => (UnitDimension::Time, 1e-12),
            "fs" => (UnitDimension::Time, 1e-15),
            "Hz" | "hz" => (UnitDimension::Frequency, 1.0),
            "mHz" => (UnitDimension::Frequency, 1e-3),
            "kHz" | "KHz" => (UnitDimension::Frequency, 1e3),
            "MHz" => (UnitDimension::Frequency, 1e6),
            "GHz" => (UnitDimension::Frequency, 1e9),
            "THz" => (UnitDimension::Frequency, 1e12),
            "V" => (UnitDimension::Voltage, 1.0),
            "mV" => (UnitDimension::Voltage, 1e-3),
            "uV" => (UnitDimension::Voltage, 1e-6),
            "nV" => (UnitDimension::Voltage, 1e-9),
            "kV" => (UnitDimension::Voltage, 1e3),
            "A" => (UnitDimension::Current, 1.0),
            "mA" => (UnitDimension::Current, 1e-3),
            "uA" => (UnitDimension::Current, 1e-6),
            "nA" => (UnitDimension::Current, 1e-9),
            "pA" => (UnitDimension::Current, 1e-12),
            "ohm" => (UnitDimension::Resistance, 1.0),
            "mohm" => (UnitDimension::Resistance, 1e-3),
            "kohm" => (UnitDimension::Resistance, 1e3),
            "Mohm" => (UnitDimension::Resistance, 1e6),
            "Gohm" => (UnitDimension::Resistance, 1e9),
            "S" => (UnitDimension::Conductance, 1.0),
            "mS" => (UnitDimension::Conductance, 1e-3),
            "uS" => (UnitDimension::Conductance, 1e-6),
            "nS" => (UnitDimension::Conductance, 1e-9),
            "W" => (UnitDimension::Power, 1.0),
            "mW" => (UnitDimension::Power, 1e-3),
            "uW" => (UnitDimension::Power, 1e-6),
            "nW" => (UnitDimension::Power, 1e-9),
            "kW" => (UnitDimension::Power, 1e3),
            "F" => (UnitDimension::Capacitance, 1.0),
            "mF" => (UnitDimension::Capacitance, 1e-3),
            "uF" => (UnitDimension::Capacitance, 1e-6),
            "nF" => (UnitDimension::Capacitance, 1e-9),
            "pF" => (UnitDimension::Capacitance, 1e-12),
            "fF" => (UnitDimension::Capacitance, 1e-15),
            "H" => (UnitDimension::Inductance, 1.0),
            "mH" => (UnitDimension::Inductance, 1e-3),
            "uH" => (UnitDimension::Inductance, 1e-6),
            "nH" => (UnitDimension::Inductance, 1e-9),
            "K" => (UnitDimension::Temperature, 1.0),
            "degC" => (UnitDimension::Temperature, 1.0),
            _ => match symbol.to_ascii_lowercase().as_str() {
                "1" | "unitless" | "dimensionless" => (UnitDimension::Dimensionless, 1.0),
                "second" | "seconds" => (UnitDimension::Time, 1.0),
                "hertz" => (UnitDimension::Frequency, 1.0),
                "volt" | "volts" => (UnitDimension::Voltage, 1.0),
                "amp" | "amps" | "ampere" | "amperes" => (UnitDimension::Current, 1.0),
                "ohms" => (UnitDimension::Resistance, 1.0),
                "megohm" | "megohms" => (UnitDimension::Resistance, 1e6),
                "siemens" | "siemen" => (UnitDimension::Conductance, 1.0),
                "watt" | "watts" => (UnitDimension::Power, 1.0),
                "farad" | "farads" => (UnitDimension::Capacitance, 1.0),
                "henry" | "henries" => (UnitDimension::Inductance, 1.0),
                "kelvin" | "celsius" => (UnitDimension::Temperature, 1.0),
                "rad" | "radian" | "radians" => {
                    (UnitDimension::Angle, 180.0 / std::f64::consts::PI)
                }
                "deg" | "degree" | "degrees" => (UnitDimension::Angle, 1.0),
                "db" => (UnitDimension::LogRatio, 1.0),
                "%" | "percent" => (UnitDimension::Dimensionless, 0.01),
                _ => return Err(format!("{raw:?} is not a recognized engineering unit")),
            },
        };
        Ok(Self {
            dimension: contract.0,
            scale: contract.1,
            bias: if symbol == "degC" || symbol.eq_ignore_ascii_case("celsius") {
                273.15
            } else {
                0.0
            },
        })
    }

    pub(crate) fn canonical_symbol(self) -> &'static str {
        match self.dimension {
            UnitDimension::Dimensionless => "1",
            UnitDimension::Time => "s",
            UnitDimension::Frequency => "Hz",
            UnitDimension::Voltage => "V",
            UnitDimension::Current => "A",
            UnitDimension::Resistance => "Ω",
            UnitDimension::Conductance => "S",
            UnitDimension::Power => "W",
            UnitDimension::Capacitance => "F",
            UnitDimension::Inductance => "H",
            UnitDimension::Temperature => "K",
            UnitDimension::Angle => "°",
            UnitDimension::LogRatio => "dB",
        }
    }

    /// Convert a parsed decimal sample without double-rounding SI prefixes.
    /// The caller validates both the input and the converted result.
    pub(crate) fn normalize_decimal(self, decimal: &str, value: f64) -> f64 {
        if self.bias == 0.0 {
            decimal_power_scaled(decimal, self.scale).unwrap_or(value * self.scale)
        } else {
            value.mul_add(self.scale, self.bias)
        }
    }
}
fn decimal_power_scaled(value: &str, scale: f64) -> Option<f64> {
    let scale_exponent = [
        (1e-15, -15),
        (1e-12, -12),
        (1e-9, -9),
        (1e-6, -6),
        (1e-3, -3),
        (0.01, -2),
        (1.0, 0),
        (1e3, 3),
        (1e6, 6),
        (1e9, 9),
        (1e12, 12),
    ]
    .into_iter()
    .find_map(|(candidate, exponent)| (scale == candidate).then_some(exponent))?;
    let (mantissa, source_exponent) = if let Some(separator) = value.find(['e', 'E']) {
        (
            &value[..separator],
            value[separator + 1..].parse::<i32>().ok()?,
        )
    } else {
        (value, 0_i32)
    };
    let combined_exponent = source_exponent.checked_add(scale_exponent)?;
    format!("{mantissa}e{combined_exponent}")
        .parse::<f64>()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized(text: &str, unit: &str) -> f64 {
        EngineeringUnit::parse(unit)
            .unwrap()
            .normalize_decimal(text, text.parse().unwrap())
    }

    #[test]
    fn prefixes_preserve_case_and_round_decimal_values_once() {
        assert_eq!(normalized("10", "µA"), 1e-5);
        assert_eq!(normalized("2.5e-2", "mV"), 2.5e-5);
        assert_eq!(normalized("1", "mHz"), 0.001);
        assert_eq!(normalized("1", "MHz"), 1_000_000.0);
        assert_eq!(normalized("1", "mΩ"), 0.001);
        assert_eq!(normalized("1", "MΩ"), 1_000_000.0);
        assert_eq!(
            EngineeringUnit::parse("ms").unwrap().canonical_symbol(),
            "s"
        );
        assert_eq!(
            EngineeringUnit::parse("mS").unwrap().canonical_symbol(),
            "S"
        );
    }

    #[test]
    fn angles_and_absolute_temperatures_have_one_declared_canonical_unit() {
        assert_eq!(normalized("3.141592653589793", "rad"), 180.0);
        assert_eq!(normalized("-90", "°"), -90.0);
        assert_eq!(
            EngineeringUnit::parse("rad").unwrap().canonical_symbol(),
            "°"
        );
        for celsius in ["°C", "degC", "celsius", "Celsius"] {
            assert_eq!(normalized("25", celsius), 298.15);
            assert_eq!(normalized("-273.15", celsius), 0.0);
            assert_eq!(
                EngineeringUnit::parse(celsius).unwrap().canonical_symbol(),
                "K"
            );
        }
        assert_eq!(normalized("25", "K"), 25.0);
        assert!(EngineeringUnit::parse("C").is_err());
    }

    #[test]
    fn explicit_dimensionless_and_logarithmic_units_remain_distinct() {
        assert_eq!(normalized("12.5", "%"), 0.125);
        assert_eq!(EngineeringUnit::parse("%").unwrap().canonical_symbol(), "1");
        assert_eq!(normalized("-20", "dB"), -20.0);
        assert_eq!(
            EngineeringUnit::parse("dB").unwrap().canonical_symbol(),
            "dB"
        );
        assert!(EngineeringUnit::parse("").is_err());
        assert!(EngineeringUnit::parse("mystery").is_err());
    }
}
