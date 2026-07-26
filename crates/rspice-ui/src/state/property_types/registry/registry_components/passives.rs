use super::*;

mod capacitor;
mod inductor;
mod memristor;
mod resistor;
mod saturable_inductor;
mod transformer;

impl PropertyRegistry {
    pub(in super::super) fn register_passive_components(&mut self) {
        self.register_resistor();
        self.register_capacitor();
        self.register_inductor();
        self.register_saturable_inductor();
        self.register_memristor();
        self.register_transformer();
        self.register_coupled_inductor();
        self.register_transmission_line();
        self.register_lossy_transmission_line();
        self.register_coupled_transmission_line();
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

    /// Register the lossy transmission line (O element + LTRA/TXL card).
    /// R/L/G/C are per-length values; the physical length scales them.
    pub(in super::super) fn register_lossy_transmission_line(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("O1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Line Model")
                .with_description("Library LTRA/TXL model name; overrides the RLGC parameters")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(9)
                .with_category("Model"),
        );
        sheet.add(
            PropertyDefinition::new("kind")
                .with_display_name("Model Kind")
                .with_description(
                    "LTRA (convolution, requires G=0) or TXL (lumped approximation, allows G)",
                )
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "ltra",
                    vec!["ltra".to_string(), "txl".to_string()],
                ))
                .with_order(10)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Resistance / Length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1"))
                .with_unit("Ω/m")
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("l")
                .with_display_name("Inductance / Length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("250n"))
                .with_unit("H/m")
                .with_order(12)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("c")
                .with_display_name("Capacitance / Length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("100p"))
                .with_unit("F/m")
                .with_order(13)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("g")
                .with_display_name("Conductance / Length")
                .with_description("Shunt loss (TXL only; the native LTRA runtime requires G=0)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("S/m")
                .with_order(14)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("len")
                .with_display_name("Length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1"))
                .with_unit("m")
                .with_order(15)
                .with_category("Electrical")
                .required(),
        );

        self.sheets
            .insert(ComponentType::LossyTransmissionLine, sheet);
    }

    /// Register the coupled transmission line (P element + CPL card).
    /// Matrix entries are upper-triangle per-length values (11, 12, 22);
    /// off-diagonal C and G must be ≤ 0 (Maxwell form).
    pub(in super::super) fn register_coupled_transmission_line(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance Name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("P1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Line Model")
                .with_description("Library CPL model name; overrides the matrix parameters")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(9)
                .with_category("Model"),
        );

        let matrix = |name: &str, display: &str, unit: &str, default: &str, order: i32| {
            PropertyDefinition::new(name)
                .with_display_name(display)
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression(default))
                .with_unit(unit)
                .with_order(order)
                .with_category("Line Matrices")
        };
        sheet.add(matrix("l11", "L11 / Length", "H/m", "380n", 10));
        sheet.add(matrix("l12", "L12 / Length", "H/m", "60n", 11));
        sheet.add(matrix("l22", "L22 / Length", "H/m", "380n", 12));
        sheet.add(matrix("c11", "C11 / Length", "F/m", "120p", 13));
        sheet.add(matrix("c12", "C12 / Length", "F/m", "-12p", 14));
        sheet.add(matrix("c22", "C22 / Length", "F/m", "120p", 15));
        sheet.add(matrix("r11", "R11 / Length", "Ω/m", "0.1", 16));
        sheet.add(matrix("r12", "R12 / Length", "Ω/m", "0", 17));
        sheet.add(matrix("r22", "R22 / Length", "Ω/m", "0.1", 18));
        sheet.add(matrix("g11", "G11 / Length", "S/m", "0", 19));
        sheet.add(matrix("g12", "G12 / Length", "S/m", "0", 20));
        sheet.add(matrix("g22", "G22 / Length", "S/m", "0", 21));

        sheet.add(
            PropertyDefinition::new("length")
                .with_display_name("Length")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0.1"))
                .with_unit("m")
                .with_order(30)
                .with_category("Electrical")
                .required(),
        );

        self.sheets
            .insert(ComponentType::CoupledTransmissionLine, sheet);
    }
}
