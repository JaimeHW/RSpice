//! Authored design-variable quantity and exact unit validation.

use serde::{Deserialize, Serialize};

/// Physical quantity carried by a design variable. The quantity is retained
/// independently from the expression so editors can validate units without
/// coercing the user's exact engineering input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableQuantity {
    Resistance,
    Capacitance,
    Voltage,
    Current,
    Temperature,
    Dimensionless,
}

impl DesignVariableQuantity {
    pub const ALL: [Self; 6] = [
        Self::Resistance,
        Self::Capacitance,
        Self::Voltage,
        Self::Current,
        Self::Temperature,
        Self::Dimensionless,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Resistance => "Resistance",
            Self::Capacitance => "Capacitance",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Temperature => "Temperature",
            Self::Dimensionless => "Dimensionless",
        }
    }
}

pub fn parse_design_quantity(
    expression: &str,
    quantity: DesignVariableQuantity,
) -> Result<f64, String> {
    use rspice_app_types::quantity::{
        QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
    };

    let text = expression.trim();
    let (numeric, kind) = match quantity {
        DesignVariableQuantity::Resistance => (
            strip_required_unit(text, &["ohm", "Ω"])
                .ok_or_else(|| "explicit resistance unit required (ohm or Ω)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Capacitance => (
            text.strip_suffix('F')
                .ok_or_else(|| "explicit capacitance unit required (F)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Voltage => (
            text.strip_suffix('V')
                .ok_or_else(|| "explicit voltage unit required (V)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Current => (
            text.strip_suffix('A')
                .ok_or_else(|| "explicit current unit required (A)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Temperature => (text, QuantityInputKind::Temperature),
        DesignVariableQuantity::Dimensionless => (text, QuantityInputKind::EngineeringScalar),
    };
    let numeric = if quantity == DesignVariableQuantity::Dimensionless
        || quantity == DesignVariableQuantity::Temperature
    {
        numeric.trim().to_owned()
    } else {
        normalize_explicit_unit_scalar(numeric.trim())
    };
    parse_ui_quantity(
        &numeric,
        kind,
        QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    )
    .map_err(|error| error.to_string())
}

fn normalize_explicit_unit_scalar(value: &str) -> String {
    if let Some(prefix) = value.strip_suffix('M') {
        format!("{}Meg", prefix.trim_end())
    } else {
        value.to_owned()
    }
}

fn strip_required_unit<'a>(value: &'a str, units: &[&str]) -> Option<&'a str> {
    units.iter().find_map(|unit| {
        if unit.is_ascii() {
            value
                .get(value.len().saturating_sub(unit.len())..)
                .filter(|suffix| suffix.eq_ignore_ascii_case(unit))
                .map(|_| &value[..value.len() - unit.len()])
        } else {
            value.strip_suffix(unit)
        }
    })
}
