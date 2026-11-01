//! Property sheet for the memristor, including the window function and the
//! state bounds its model integrates between.

use super::*;

/// One TEAM model-card parameter: netlist key, label, blurb, default, unit.
///
/// The card is a flat list of scalars with no cross-dependencies beyond the
/// window selector, so a table beats eighteen near-identical builder chains.
type TeamParam = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    Option<&'static str>,
);

/// Every scalar TEAM (`MEMRISTOR LEVEL=2`) parameter the engine reads, in
/// card order, with the engine's own defaults.
///
/// The engine *rejects* a TEAM card carrying anything outside this set, so it
/// is a closed list rather than a selection: leaving one out is the only way
/// to make it unreachable from the schematic.
const TEAM_KINETICS: &[TeamParam] = &[
    (
        "kon",
        "On rate (KON)",
        "State velocity coefficient below the ON current threshold; negative drives x toward XON",
        "-8e-13",
        Some("m/s"),
    ),
    (
        "koff",
        "Off rate (KOFF)",
        "State velocity coefficient above the OFF current threshold",
        "8e-13",
        Some("m/s"),
    ),
    (
        "alphaon",
        "On exponent (ALPHAON)",
        "Current-overdrive exponent for the ON branch",
        "3",
        None,
    ),
    (
        "alphaoff",
        "Off exponent (ALPHAOFF)",
        "Current-overdrive exponent for the OFF branch",
        "3",
        None,
    ),
    (
        "ion",
        "On threshold (ION)",
        "Current below which the ON branch drives the state",
        "8.9u",
        Some("A"),
    ),
    (
        "ioff",
        "Off threshold (IOFF)",
        "Current above which the OFF branch drives the state",
        "115u",
        Some("A"),
    ),
];

const TEAM_WINDOW: &[TeamParam] = &[
    (
        "d",
        "Device thickness (D)",
        "Film thickness; read by the Joglekar and Biolek windows",
        "115u",
        Some("m"),
    ),
    (
        "p",
        "Window exponent (P)",
        "Window sharpness; read by the Joglekar, Biolek, and Prodromakis windows",
        "115u",
        None,
    ),
    (
        "j",
        "Window scale (J)",
        "Prodromakis window scale factor",
        "115u",
        None,
    ),
    (
        "aon",
        "On bound (AON)",
        "Lower state bound used by the TEAM window",
        "0",
        Some("m"),
    ),
    (
        "aoff",
        "Off bound (AOFF)",
        "Upper state bound used by the TEAM window",
        "3n",
        Some("m"),
    ),
    (
        "wc",
        "Window constant (WC)",
        "TEAM window decay constant",
        "1.07p",
        Some("m"),
    ),
];

fn team_property(param: &TeamParam, order: i32, category: &str) -> PropertyDefinition {
    let (key, label, description, default, unit) = *param;
    let mut definition = PropertyDefinition::new(key)
        .with_display_name(label)
        .with_description(description)
        .with_type(PropertyType::Expression)
        .with_default(PropertyValue::expression(default))
        .with_order(order)
        .with_category(category)
        .advanced();
    if let Some(unit) = unit {
        definition = definition.with_unit(unit);
    }
    definition
}

impl PropertyCatalog {
    /// Register the memristor (Xyce YMEMRISTOR + TEAM LEVEL=2 model card).
    ///
    /// `LEVEL=4` (PEM) is deliberately absent: it is parameterized by external
    /// data files rather than scalars, which is a project-packaging question
    /// and not a property sheet. A PEM card reached through the Model field
    /// still simulates.
    pub(super) fn register_memristor(&mut self) {
        let mut sheet = PropertySheet::new();

        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Instance name")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("MR1"))
                .with_order(0)
                .with_category("Instance")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("model")
                .with_display_name("Model")
                .with_description("Library MEMRISTOR model name; overrides every card field below")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(9)
                .with_category("Model"),
        );
        sheet.add(
            PropertyDefinition::new("ron")
                .with_display_name("On resistance (RON)")
                .with_description("Fully-doped (low) resistance state")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("50"))
                .with_unit("Ω")
                .with_order(10)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("roff")
                .with_display_name("Off resistance (ROFF)")
                .with_description("Undoped (high) resistance state")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1k"))
                .with_unit("Ω")
                .with_order(11)
                .with_category("Electrical"),
        );
        sheet.add(
            PropertyDefinition::new("ivrelation")
                .with_display_name("I-V relation")
                .with_description("0 linear drift, 1 Simmons tunnel barrier")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1.0)
                .with_order(12)
                .with_category("Electrical"),
        );

        // State bounds. The DC operating point gauges x against XON, so these
        // are the two fields that decide which branch a solve starts on.
        sheet.add(
            PropertyDefinition::new("xon")
                .with_display_name("On state (XON)")
                .with_description("State variable at full conduction")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("0"))
                .with_unit("m")
                .with_order(20)
                .with_category("State"),
        );
        sheet.add(
            PropertyDefinition::new("xoff")
                .with_display_name("Off state (XOFF)")
                .with_description("State variable at full depletion")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("3n"))
                .with_unit("m")
                .with_order(21)
                .with_category("State"),
        );
        sheet.add(
            PropertyDefinition::new("xscaling")
                .with_display_name("State scaling (XSCALING)")
                .with_description("Scale applied to the internal state variable")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1"))
                .with_order(22)
                .with_category("State"),
        );

        for (index, param) in TEAM_KINETICS.iter().enumerate() {
            sheet.add(team_property(param, 30 + index as i32, "Kinetics"));
        }

        sheet.add(
            PropertyDefinition::new("wt")
                .with_display_name("Window (WT)")
                .with_description(
                    "Window shaping the state velocity: 0 none, 1 Joglekar, 2 Biolek, \
                     3 Prodromakis, 4 TEAM",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 4.0)
                .with_order(40)
                .with_category("Window"),
        );
        for (index, param) in TEAM_WINDOW.iter().enumerate() {
            sheet.add(team_property(param, 41 + index as i32, "Window"));
        }

        self.sheets.insert(ComponentType::Memristor, sheet);
    }
}
