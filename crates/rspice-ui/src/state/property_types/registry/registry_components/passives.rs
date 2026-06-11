use super::*;

mod capacitor;
mod inductor;
mod resistor;
mod transformer;

impl PropertyRegistry {
    pub(in super::super) fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
        self.register_transformer();
        self.register_coupled_inductor();
        self.register_transmission_line();
    }

    /// Register the lossless transmission line (T element: Z0/TD).
    pub(in super::super) fn register_transmission_line(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("T1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("z0")
                .with_display_name("Characteristic Impedance")
                .with_description("Line impedance Z0")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(50.0))
                .with_unit("Ω")
                .with_order(10)
                .with_category("Electrical")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay")
                .with_description("One-way propagation delay (suffixes allowed, e.g. 1n)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(11)
                .with_category("Electrical")
                .required(),
        );

        self.sheets.insert(ComponentType::TransmissionLine, sheet);
    }
}
