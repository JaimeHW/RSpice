use super::*;

pub(super) fn source_symbol_property(
    description: &str,
    default_variant: &str,
    options: &[&str],
) -> PropertyDefinition {
    PropertyDefinition::new("symbol")
        .with_display_name("Symbol")
        .with_description(description)
        .with_type(PropertyType::Enum)
        .with_default(PropertyValue::enumeration(
            default_variant,
            options.iter().map(|option| (*option).to_string()).collect(),
        ))
        .with_order(900)
        .with_category("Appearance")
}
