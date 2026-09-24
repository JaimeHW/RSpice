//! Physical measurement units and bounded unit expressions shared by core and results.

use serde::{Deserialize, Serialize};

mod parse;

/// Missing historical metadata is represented by the enclosing `Option`.
/// `Unknown` explicitly records that a current producer cannot infer a unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "symbol",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum MeasurementUnit {
    Unknown,
    Known(String),
}

/// Units of the published scalar, the pre-projection scalar, and its axis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementUnits {
    pub value: MeasurementUnit,
    pub raw_value: MeasurementUnit,
    pub axis: MeasurementUnit,
}

impl MeasurementUnits {
    pub fn validate(&self) -> Result<(), String> {
        for unit in [&self.value, &self.raw_value, &self.axis] {
            unit.validate()?;
        }
        Ok(())
    }
}

impl MeasurementUnit {
    pub fn known(symbol: &str) -> Result<Self, String> {
        parse::Unit::parse(symbol)?;
        Ok(Self::Known(symbol.to_owned()))
    }

    pub fn symbol(&self) -> Option<&str> {
        match self {
            Self::Known(symbol) => Some(symbol),
            Self::Unknown => None,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Known(symbol) => parse::Unit::parse(symbol).map(|_| ()),
            Self::Unknown => Ok(()),
        }
    }

    /// Convert a measured value to the user's unit. Unknown is never unitless.
    pub fn convert_value(&self, value: f64, requested: &str) -> Result<f64, String> {
        let source = self
            .parsed()
            .ok_or_else(|| "measurement unit is unknown".to_owned())?;
        let target = parse::Unit::parse(requested)?;
        if source.dimensions != target.dimensions {
            return Err(format!(
                "measurement unit {} is incompatible with {requested}",
                self.symbol().unwrap_or("unknown")
            ));
        }
        // Form the conversion once: dividing each sample by an approximate
        // SI prefix needlessly moves values such as 0.35 V below 350 mV.
        let scale = source.scale / target.scale;
        let bias = (source.bias - target.bias) / target.scale;
        let converted = if scale.is_finite() && scale > 0.0 && bias.is_finite() {
            value.mul_add(scale, bias)
        } else {
            // Extreme compound scales can have an unrepresentable ratio even
            // when applying the source and target scales in stages is finite.
            (value.mul_add(source.scale, source.bias) - target.bias) / target.scale
        };
        if !value.is_finite() || !converted.is_finite() {
            return Err("measurement unit conversion is non-finite".to_owned());
        }
        Ok(converted)
    }

    fn parsed(&self) -> Option<parse::Unit> {
        self.symbol()
            .and_then(|symbol| parse::Unit::parse(symbol).ok())
    }

    /// Whether two parsed units have identical dimensions, scale, and bias.
    pub fn same_scale_as(&self, rhs: &Self) -> bool {
        self.parsed().zip(rhs.parsed()).is_some_and(|(a, b)| a == b)
    }

    pub fn interval(&self) -> Self {
        if self.parsed().is_some_and(|unit| unit.bias != 0.0) {
            Self::Known("K".into())
        } else {
            self.clone()
        }
    }

    pub fn product(&self, rhs: &Self, divide: bool) -> Self {
        let Some((left, right)) = self.parsed().zip(rhs.parsed()) else {
            return Self::Unknown;
        };
        let Some(unit) = left.product(right, divide) else {
            return Self::Unknown;
        };
        if unit.scale == 1.0 {
            Self::Known(unit.canonical_symbol())
        } else {
            Self::known(&format!(
                "({}){}({})",
                self.symbol().unwrap_or_default(),
                if divide { "/" } else { "*" },
                rhs.symbol().unwrap_or_default()
            ))
            .unwrap_or(Self::Unknown)
        }
    }

    pub fn power(&self, power: f64) -> Self {
        let Some(unit) = self.parsed().and_then(|unit| unit.power(power)) else {
            return Self::Unknown;
        };
        if unit.scale == 1.0 {
            Self::Known(unit.canonical_symbol())
        } else {
            Self::known(&format!("({})^{power}", self.symbol().unwrap_or_default()))
                .unwrap_or(Self::Unknown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(symbol: &str) -> MeasurementUnit {
        MeasurementUnit::known(symbol).unwrap()
    }

    #[test]
    fn measurement_unit_conversions_keep_dimensions_prefixes_and_offsets() {
        for (source, value, target, expected) in [
            ("V", 0.25, "mV", 250.0),
            ("A", 2e-6, "µA", 2.0),
            ("ohm", 1e6, "MΩ", 1.0),
            ("ohm", 1e-3, "mohm", 1.0),
            ("V^2/Hz", 1e-12, "(uV)^2/Hz", 1.0),
            ("V/sqrt(Hz)", 1e-9, "nV/sqrt(Hz)", 1.0),
            ("A/√Hz", 1e-9, "nA/√(Hz)", 1.0),
            ("ratio", 0.25, "%", 25.0),
            ("dBc/Hz", -120.0, "dBc/Hz", -120.0),
            ("dBc/Hz", -120.0, "(dBc/Hz)", -120.0),
            ("(dBc/Hz)/Hz", -1.0, "(dBc/Hz)/kHz", -1000.0),
            ("A*s", 1e-9, "nC", 1.0),
            ("V/A", 1000.0, "kohm", 1.0),
            ("V/s", 1e6, "V/us", 1.0),
            ("1", 0.5, "%", 50.0),
            ("rad", std::f64::consts::PI, "deg", 180.0),
            ("degC", 0.0, "K", 273.15),
            ("K", 300.0, "°C", 26.85),
        ] {
            let actual = unit(source).convert_value(value, target).unwrap();
            assert!(
                (actual - expected).abs() < 1e-12 * expected.abs().max(1.0),
                "{source} -> {target}: {actual}"
            );
        }
        assert!(unit("V").convert_value(1.0, "A").is_err());
        assert_eq!(unit("V").convert_value(0.35, "mV").unwrap(), 350.0);
        assert!(unit("dB").convert_value(1.0, "1").is_err());
        assert!(unit("dBc/Hz").convert_value(-120.0, "dB/Hz").is_err());
        assert!(unit("dBc/Hz").convert_value(-120.0, "dBc/kHz").is_err());
        assert!(MeasurementUnit::Unknown.convert_value(1.0, "1").is_err());
        assert!(unit("V").convert_value(f64::INFINITY, "V").is_err());
        for invalid in [
            "",
            "mystery",
            "V/A junk",
            "V^0.3",
            "degC/s",
            "V^999",
            "V\n",
            &"(".repeat(257),
        ] {
            assert!(MeasurementUnit::known(invalid).is_err(), "{invalid:?}");
        }
        let serialized = serde_json::to_string(&unit("V/sqrt(Hz)")).unwrap();
        let restored: MeasurementUnit = serde_json::from_str(&serialized).unwrap();
        restored.validate().unwrap();
        assert_eq!(restored, unit("V/sqrt(Hz)"));
    }
}
