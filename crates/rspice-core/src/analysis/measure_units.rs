//! Physical units of measured values, independent of display preferences.

use serde::{Deserialize, Serialize};

mod infer;
mod native;
mod parse;
pub(super) use native::annotate_native;

pub use infer::measurement_units;

pub(super) fn annotate(
    statements: &[&crate::netlist::measure::MeasureStatement],
    results: &mut [super::MeasureResult],
    axis: Option<&MeasurementUnit>,
    signals: &std::collections::HashMap<String, MeasurementUnit>,
) {
    for (result, units) in results
        .iter_mut()
        .zip(measurement_units(statements, axis, signals))
    {
        result.units = Some(units);
    }
}

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

    fn interval(&self) -> Self {
        if self.parsed().is_some_and(|unit| unit.bias != 0.0) {
            Self::Known("K".into())
        } else {
            self.clone()
        }
    }

    fn product(&self, rhs: &Self, divide: bool) -> Self {
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

    fn power(&self, power: f64) -> Self {
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
mod tests;
