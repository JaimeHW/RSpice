use super::*;

/// Property sheets for the XSPICE code-model blocks.
///
/// Each sheet exposes exactly the parameters the netlist generator can
/// place on the block's `.MODEL` card (see `netlist_gen::xspice`), so an
/// edit in the properties panel always reaches the deck.
impl PropertyRegistry {
    pub(in super::super) fn register_xspice_components(&mut self) {
        self.register_xspice_analog();
        self.register_xspice_digital();
        self.register_xspice_bridges();
    }

    fn register_xspice_analog(&mut self) {
        // Gain block
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop(
            "gain",
            "Gain",
            "Output = gain * (input + in_offset) + out_offset",
            1.0,
            10,
        ));
        sheet.add(number_prop("in_offset", "Input Offset", "", 0.0, 11));
        sheet.add(number_prop("out_offset", "Output Offset", "", 0.0, 12));
        self.sheets.insert(ComponentType::XspiceGain, sheet);

        // Summer / multiplier share output shaping parameters
        for kind in [ComponentType::XspiceSummer, ComponentType::XspiceMultiplier] {
            let mut sheet = xspice_sheet_base("A1");
            sheet.add(number_prop("out_gain", "Output Gain", "", 1.0, 10));
            sheet.add(number_prop("out_offset", "Output Offset", "", 0.0, 11));
            self.sheets.insert(kind, sheet);
        }

        // Divider
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop("num_gain", "Numerator Gain", "", 1.0, 10));
        sheet.add(number_prop("den_gain", "Denominator Gain", "", 1.0, 11));
        sheet.add(number_prop(
            "den_lower_limit",
            "Denominator Floor",
            "Smallest denominator magnitude before limiting",
            1e-10,
            12,
        ));
        self.sheets.insert(ComponentType::XspiceDivider, sheet);

        // Limiter
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop("gain", "Gain", "", 1.0, 10));
        sheet.add(number_prop("out_lower_limit", "Lower Limit", "", -1.0, 11));
        sheet.add(number_prop("out_upper_limit", "Upper Limit", "", 1.0, 12));
        sheet.add(number_prop(
            "limit_range",
            "Smoothing Range",
            "Transition band approaching the limits",
            1e-6,
            13,
        ));
        self.sheets.insert(ComponentType::XspiceLimiter, sheet);

        // Integrator
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop("gain", "Gain", "", 1.0, 10));
        sheet.add(number_prop("out_ic", "Initial Output", "", 0.0, 11));
        sheet.add(number_prop("out_lower_limit", "Lower Limit", "", -1e12, 12));
        sheet.add(number_prop("out_upper_limit", "Upper Limit", "", 1e12, 13));
        self.sheets.insert(ComponentType::XspiceIntegrator, sheet);

        // Differentiator
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop("gain", "Gain", "", 1.0, 10));
        sheet.add(number_prop("out_lower_limit", "Lower Limit", "", -1e12, 11));
        sheet.add(number_prop("out_upper_limit", "Upper Limit", "", 1e12, 12));
        self.sheets
            .insert(ComponentType::XspiceDifferentiator, sheet);
    }

    fn register_xspice_digital(&mut self) {
        // Gates and buffers: rise/fall delays
        for kind in [
            ComponentType::XspiceBuffer,
            ComponentType::XspiceInverter,
            ComponentType::XspiceAndGate,
            ComponentType::XspiceOrGate,
            ComponentType::XspiceNandGate,
            ComponentType::XspiceNorGate,
            ComponentType::XspiceXorGate,
        ] {
            let mut sheet = xspice_sheet_base("A1");
            sheet.add(delay_prop("rise_delay", "Rise Delay", 10));
            sheet.add(delay_prop("fall_delay", "Fall Delay", 11));
            self.sheets.insert(kind, sheet);
        }

        // Tri-state buffer: single delay
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(delay_prop("delay", "Delay", 10));
        self.sheets.insert(ComponentType::XspiceTristate, sheet);

        // Sequential blocks: initial state plus output delays
        for kind in [
            ComponentType::XspiceDFlipFlop,
            ComponentType::XspiceJkFlipFlop,
            ComponentType::XspiceSrLatch,
        ] {
            let mut sheet = xspice_sheet_base("A1");
            sheet.add(
                PropertyDefinition::new("ic")
                    .with_display_name("Initial State")
                    .with_description("Power-up output state (0 or 1)")
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(0.0))
                    .with_range(0.0, 1.0)
                    .with_order(10)
                    .with_category("Behavior"),
            );
            sheet.add(delay_prop("rise_delay", "Rise Delay", 11));
            sheet.add(delay_prop("fall_delay", "Fall Delay", 12));
            self.sheets.insert(kind, sheet);
        }
    }

    fn register_xspice_bridges(&mut self) {
        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop(
            "in_low",
            "Low Threshold",
            "Analog level below which the digital output is 0",
            1.0,
            10,
        ));
        sheet.add(number_prop(
            "in_high",
            "High Threshold",
            "Analog level above which the digital output is 1",
            2.0,
            11,
        ));
        self.sheets.insert(ComponentType::XspiceAdcBridge, sheet);

        let mut sheet = xspice_sheet_base("A1");
        sheet.add(number_prop(
            "out_low",
            "Low Level",
            "Analog output for digital 0",
            0.0,
            10,
        ));
        sheet.add(number_prop(
            "out_high",
            "High Level",
            "Analog output for digital 1",
            3.3,
            11,
        ));
        self.sheets.insert(ComponentType::XspiceDacBridge, sheet);
    }
}

fn xspice_sheet_base(default_name: &str) -> PropertySheet {
    let mut sheet = PropertySheet::new();
    sheet.add(
        PropertyDefinition::new("name")
            .with_display_name("Instance Name")
            .with_type(PropertyType::String)
            .with_default(PropertyValue::string(default_name))
            .with_order(0)
            .with_category("Instance")
            .required(),
    );
    sheet
}

fn number_prop(
    name: &str,
    display: &str,
    description: &str,
    default: f64,
    order: i32,
) -> PropertyDefinition {
    let mut prop = PropertyDefinition::new(name)
        .with_display_name(display)
        .with_type(PropertyType::Number)
        .with_default(PropertyValue::number(default))
        .with_order(order)
        .with_category("Behavior");
    if !description.is_empty() {
        prop = prop.with_description(description);
    }
    prop
}

fn delay_prop(name: &str, display: &str, order: i32) -> PropertyDefinition {
    PropertyDefinition::new(name)
        .with_display_name(display)
        .with_description("Event propagation delay (suffixes allowed, e.g. 1n)")
        .with_type(PropertyType::Expression)
        .with_default(PropertyValue::expression("1n"))
        .with_unit("s")
        .with_order(order)
        .with_category("Timing")
}
