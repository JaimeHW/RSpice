//! Property sheets for hierarchy interface objects: ports, pins, and
//! subcircuit instances.

use super::*;

impl PropertyRegistry {
    pub(in super::super) fn register_interface_components(&mut self) {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("name")
                .with_display_name("Reference designator")
                .with_description(
                    "Optional displayed object identifier; durable interface identity is owned by the schematic object",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(0)
                .with_category("Identity"),
        );
        sheet.add(
            PropertyDefinition::new("value")
                .with_display_name("Port name")
                .with_description("Named net and containing-cell interface terminal")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("BIAS_EN"))
                .with_order(1)
                .with_category("Interface")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("dir")
                .with_display_name("Direction")
                .with_description("Direction used by generated symbols and hierarchy checks")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "inout",
                    ["in", "out", "inout", "supply"]
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                ))
                .with_order(10)
                .with_category("Interface"),
        );
        sheet.add(
            PropertyDefinition::new("signal_type")
                .with_display_name("Signal type")
                .with_description("Logic, analog, or power interface semantics")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "analog",
                    ["logic", "analog", "power"]
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                ))
                .with_order(11)
                .with_category("Interface"),
        );
        sheet.add(
            PropertyDefinition::new("discipline")
                .with_display_name("Discipline")
                .with_description("Electrical, logic, real-valued, or thermal connection domain")
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(
                    "electrical",
                    ["electrical", "logic", "wreal", "thermal"]
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                ))
                .with_order(12)
                .with_category("Interface"),
        );
        // The cell's `.SUBCKT` port order, and so the node order of every
        // instance of it. Placement already writes `interface_order=`, the
        // interface reader already sorts by it, and the dialog's apply path
        // already rejects a duplicate — only the editor field was missing, so
        // an interface could be ordered when it was drawn and never after.
        // Zero is not a position: it means "fall back to document order",
        // which is what the contract reader does with an absent entry.
        sheet.add(
            PropertyDefinition::new("interface_order")
                .with_display_name("Interface order")
                .with_description(
                    "One-based position in the subcircuit port list; 0 follows document order",
                )
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(0.0, 1024.0)
                .with_order(13)
                .with_category("Interface"),
        );
        sheet.add(
            PropertyDefinition::new("documentation")
                .with_display_name("Documentation")
                .with_description(
                    "Durable interface documentation exported with the design contract",
                )
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(20)
                .with_category("Documentation")
                .required(),
        );
        self.sheets.insert(ComponentType::Port, sheet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_sheet_keeps_refdes_optional_and_exposes_the_typed_contract() {
        let registry = PropertyRegistry::new();
        let sheet = registry
            .get(ComponentType::Port)
            .expect("interface component sheet");

        let name = sheet.get("name").expect("reference designator");
        assert!(!name.required);
        assert_eq!(name.default_value, PropertyValue::string(""));

        assert!(
            sheet
                .get("value")
                .is_some_and(|definition| definition.required)
        );
        for (field, expected_options) in [
            ("dir", &["in", "out", "inout", "supply"][..]),
            ("signal_type", &["logic", "analog", "power"][..]),
            (
                "discipline",
                &["electrical", "logic", "wreal", "thermal"][..],
            ),
        ] {
            let definition = sheet
                .get(field)
                .unwrap_or_else(|| panic!("missing {field}"));
            let PropertyValue::Enum { options, .. } = &definition.default_value else {
                panic!("{field} is not schema-typed as an enum");
            };
            assert_eq!(
                options.iter().map(String::as_str).collect::<Vec<_>>(),
                expected_options
            );
        }
        assert!(matches!(
            sheet
                .get("documentation")
                .expect("documentation")
                .default_value,
            PropertyValue::String(_)
        ));
    }
}
