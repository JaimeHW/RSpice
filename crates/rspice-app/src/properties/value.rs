//! Schema-aware quantity and expression parsing, independent of editor state.

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::state::property_types::{PropertyDefinition, PropertyValue};

pub fn parse_number_source(
    def: &PropertyDefinition,
    text: &str,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Result<PropertyValue, String> {
    let quantity_error = match parse_ui_quantity(
        text,
        property_quantity_kind(def),
        quantity_policy,
        number_locale,
    ) {
        Ok(value) => {
            let value = PropertyValue::number(value_for_property_schema(def, value));
            def.validate(&value)?;
            return Ok(value);
        }
        Err(error) => error,
    };

    let trimmed = text.trim();
    if trimmed.is_empty() || has_incomplete_exponent(trimmed) {
        return Err(quantity_error.to_string());
    }
    parse_expression_source(def, trimmed, quantity_policy, number_locale)
}

fn has_incomplete_exponent(source: &str) -> bool {
    let Some(index) = source.rfind(['e', 'E']) else {
        return false;
    };
    let (coefficient, exponent) = source.split_at(index);
    coefficient.parse::<f64>().is_ok() && matches!(exponent, "e" | "E" | "e+" | "E+" | "e-" | "E-")
}

/// Parse an expression-capable property without allowing numeric constants to
/// evade the property's quantity policy or numeric range. A source that can be
/// evaluated without parameters is a constant quantity; a source with an
/// unresolved parameter remains a symbolic SPICE expression.
pub fn parse_expression_source(
    def: &PropertyDefinition,
    text: &str,
    quantity_policy: QuantityPresentationPolicy,
    number_locale: UiNumberLocale,
) -> Result<PropertyValue, String> {
    let quantity_error = match parse_ui_quantity(
        text,
        property_quantity_kind(def),
        quantity_policy,
        number_locale,
    ) {
        Ok(value) => {
            let value = PropertyValue::number(value_for_property_schema(def, value));
            def.validate(&value)?;
            return Ok(value);
        }
        Err(error) => error,
    };

    let trimmed = text.trim();
    if trimmed.is_empty() {
        return if def.required {
            Err(format!("{} expression is empty", def.display_name))
        } else {
            Ok(PropertyValue::Expression(String::new()))
        };
    }
    let expression = trimmed
        .strip_prefix('{')
        .and_then(|inner| inner.strip_suffix('}'))
        .unwrap_or(trimmed)
        .trim();
    let parsed = rspice_core::netlist::expr::parse_expression(expression)
        .map_err(|error| format!("{} expression: {error}", def.display_name))?;

    // Successful evaluation with an empty parameter context proves this is a
    // constant, not a symbolic parameter expression. It must therefore obey
    // the same explicit-unit policy as any other literal quantity.
    match rspice_core::netlist::expr::evaluate(
        &parsed,
        &rspice_core::netlist::expr::ParamContext::new(),
    ) {
        Ok(constant) => {
            if property_quantity_kind(def) == QuantityInputKind::EngineeringScalar {
                if !constant.is_finite() {
                    return Err(format!("{} must be finite", def.display_name));
                }
                let value = PropertyValue::number(constant);
                def.validate(&value)?;
                return Ok(value);
            }
            return Err(quantity_error.to_string());
        }
        Err(rspice_core::netlist::expr::ExprError::UndefinedParam(_)) => {}
        Err(error) => return Err(format!("{} expression: {error}", def.display_name)),
    }

    Ok(PropertyValue::Expression(expression.to_owned()))
}

pub fn property_quantity_kind(def: &PropertyDefinition) -> QuantityInputKind {
    match def.unit.as_deref() {
        Some("s") => QuantityInputKind::Time,
        Some("Hz") => QuantityInputKind::Frequency,
        Some("°" | "deg" | "rad") => QuantityInputKind::Angle,
        Some("K" | "°F") => QuantityInputKind::Temperature,
        Some("°C") if def.name.eq_ignore_ascii_case("temp") => QuantityInputKind::Temperature,
        Some("°C") => QuantityInputKind::TemperatureDelta,
        _ => QuantityInputKind::EngineeringScalar,
    }
}

/// Registry numeric values retain the unit declared by their schema. The
/// unit-safe parser returns SI, so only legacy degree/Celsius schemas need a
/// boundary conversion before the property bridge writes them.
fn value_for_property_schema(def: &PropertyDefinition, value_si: f64) -> f64 {
    match def.unit.as_deref() {
        Some("°" | "deg") => value_si.to_degrees(),
        Some("°C") if def.name.eq_ignore_ascii_case("temp") => value_si - 273.15,
        Some("°F") => (value_si - 273.15) * 1.8 + 32.0,
        _ => value_si,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::property_types::PropertyType;

    #[test]
    fn registry_units_select_safe_interactive_quantity_kinds() {
        let time = PropertyDefinition::new("td").with_unit("s");
        let frequency = PropertyDefinition::new("freq").with_unit("Hz");
        let phase = PropertyDefinition::new("phase").with_unit("°");
        let delta_temperature = PropertyDefinition::new("dtemp").with_unit("°C");
        assert_eq!(property_quantity_kind(&time), QuantityInputKind::Time);
        assert_eq!(
            property_quantity_kind(&frequency),
            QuantityInputKind::Frequency
        );
        assert_eq!(property_quantity_kind(&phase), QuantityInputKind::Angle);
        assert_eq!(
            property_quantity_kind(&delta_temperature),
            QuantityInputKind::TemperatureDelta
        );
        assert_eq!(
            value_for_property_schema(&phase, std::f64::consts::FRAC_PI_2),
            90.0
        );
    }

    #[test]
    fn numeric_source_parser_distinguishes_quantities_expressions_and_incomplete_text() {
        let definition = PropertyDefinition::new("gain").with_type(PropertyType::Number);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert_eq!(
            parse_number_source(&definition, "10k", policy, locale),
            Ok(PropertyValue::number(10_000.0))
        );
        assert_eq!(
            parse_number_source(&definition, "gain", policy, locale),
            Ok(PropertyValue::Expression("gain".to_owned()))
        );
        assert_eq!(
            parse_number_source(&definition, "{gain * 2}", policy, locale),
            Ok(PropertyValue::Expression("gain * 2".to_owned()))
        );
        assert!(parse_number_source(&definition, "1e", policy, locale).is_err());
        assert!(parse_number_source(&definition, "", policy, locale).is_err());
        assert!(parse_number_source(&definition, "{", policy, locale).is_err());
    }

    #[test]
    fn numeric_source_parser_live_validates_ranges_and_constant_errors() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Number)
            .with_range(0.0, 10.0);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert!(parse_number_source(&definition, "11", policy, locale).is_err());
        assert!(parse_number_source(&definition, "6 * 2", policy, locale).is_err());
        assert!(parse_number_source(&definition, "1 / 0", policy, locale).is_err());
        assert_eq!(
            parse_number_source(&definition, "gain_parameter", policy, locale),
            Ok(PropertyValue::Expression("gain_parameter".to_owned()))
        );
    }

    #[test]
    fn expression_parser_enforces_real_registry_phase_units_and_range() {
        let registry = crate::properties::PropertyEditorSchema::new();
        let phase = registry
            .get(crate::state::ComponentType::VoltageSource)
            .and_then(|sheet| sheet.get("acphase"))
            .expect("voltage-source AC phase definition");
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert!(parse_expression_source(phase, "400", policy, locale).is_err());
        assert!(parse_expression_source(phase, "2 * 200", policy, locale).is_err());
        assert!(parse_expression_source(phase, "400 deg", policy, locale).is_err());

        let parsed = parse_expression_source(phase, "90 deg", policy, locale)
            .expect("explicit in-range phase");
        assert_eq!(parsed.as_number(), Some(90.0));
        assert_eq!(
            parse_expression_source(phase, "phase_parameter", policy, locale),
            Ok(PropertyValue::Expression("phase_parameter".to_owned()))
        );
    }

    #[test]
    fn scalar_constant_expressions_are_evaluated_and_range_checked() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Expression)
            .with_range(0.0, 10.0);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        assert_eq!(
            parse_expression_source(&definition, "4 * 2", policy, locale),
            Ok(PropertyValue::number(8.0))
        );
        assert!(parse_expression_source(&definition, "4 * 3", policy, locale).is_err());
    }

    #[test]
    fn expression_evaluation_errors_are_not_misclassified_as_symbolic() {
        let definition = PropertyDefinition::new("gain")
            .with_display_name("Gain")
            .with_type(PropertyType::Expression);
        let policy = QuantityPresentationPolicy::default();
        let locale = UiNumberLocale::default();

        let division = parse_expression_source(&definition, "1 / 0", policy, locale)
            .expect_err("division by zero is invalid");
        assert!(division.contains("Division by zero"));

        let unknown = parse_expression_source(&definition, "unknown_function(1)", policy, locale)
            .expect_err("unknown function is invalid");
        assert!(unknown.contains("Unknown function"));
    }
}
