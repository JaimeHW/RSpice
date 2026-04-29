use super::*;

mod bjt;
mod diode;
mod jfet;
mod mosfet;

fn symbol_variant_property(
    display_name: &str,
    description: &str,
    default_variant: &str,
    options: &[&str],
) -> PropertyDefinition {
    PropertyDefinition::new("symbol")
        .with_display_name(display_name)
        .with_description(description)
        .with_type(PropertyType::Enum)
        .with_default(PropertyValue::enumeration(
            default_variant,
            options.iter().map(|option| (*option).to_string()).collect(),
        ))
        .with_order(900)
        .with_category("Appearance")
}

impl PropertyRegistry {
    pub(in super::super) fn register_semiconductors(&mut self) {
        // Diode with commercial-grade parameters
        self.register_diode();

        // MOSFET (NMOS/PMOS) with full Spectre-compatible parameters
        self.register_mosfet();

        // BJT (NPN/PNP) with commercial-grade parameters
        self.register_bjt();

        // JFET (NJFET/PJFET)
        self.register_jfet();
    }
}
