//! Property sheets for XSPICE code-model devices.

use super::*;
use crate::state::DeviceImplementation;

/// Property sheets for the XSPICE code-model blocks.
///
/// Each sheet is generated from the code model's own parameter list in
/// `rspice_core::xspice`, which is the same list the netlist generator builds
/// the `.MODEL` card from. Hand-written sheets drifted from that catalog —
/// sixty-eight parameters across seventeen blocks had no editor field at all —
/// and deriving both from one source is what keeps them in step. The test at
/// the bottom of this file is the guard.
impl PropertyRegistry {
    pub(in super::super) fn register_xspice_components(&mut self) {
        let models = rspice_core::xspice::CodeModelRegistry::with_builtins();
        for kind in ComponentType::ALL {
            let DeviceImplementation::Xspice { model_type } = kind.descriptor().implementation
            else {
                continue;
            };
            let Some(model) = models.get(model_type) else {
                continue;
            };
            self.sheets
                .insert(kind, code_model_sheet(model.parameters()));
        }
    }
}

fn code_model_sheet(parameters: &[rspice_core::xspice::ParamSpec]) -> PropertySheet {
    let mut sheet = PropertySheet::new();
    sheet.add(
        PropertyDefinition::new("name")
            .with_display_name("Instance Name")
            .with_type(PropertyType::String)
            .with_default(PropertyValue::string("A1"))
            .with_order(0)
            .with_category("Instance")
            .required(),
    );
    for (index, parameter) in parameters.iter().enumerate() {
        sheet.add(code_model_property(parameter, 10 + index as i32));
    }
    sheet
}

/// Presentation for one code-model parameter.
///
/// The catalog carries the name, type, default, and required flag; it does not
/// carry a human label, a unit, or a grouping, and several of its descriptions
/// are empty. Those four things are supplied here and nowhere else.
fn code_model_property(
    parameter: &rspice_core::xspice::ParamSpec,
    order: i32,
) -> PropertyDefinition {
    use rspice_core::xspice::ParamType;

    let name = parameter.name.as_str();
    let (label, fallback_description) = code_model_label(name);
    let facet = code_model_facet(name, parameter.param_type);

    let mut definition = PropertyDefinition::new(name)
        .with_display_name(label)
        .with_order(order)
        .with_category(facet.category);

    definition = match parameter.param_type {
        ParamType::Boolean => definition
            .with_type(PropertyType::Boolean)
            .with_default(PropertyValue::boolean(parameter.default != 0.0)),
        ParamType::String => {
            definition
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(
                    parameter.string_default.clone().unwrap_or_default(),
                ))
        }
        ParamType::Integer => definition
            .with_type(PropertyType::Number)
            .with_default(PropertyValue::number(parameter.default)),
        // Vector and complex parameters are carried verbatim in the bracketed
        // syntax the model card demands; there is no per-element editor.
        ParamType::StringVector
        | ParamType::RealVector
        | ParamType::IntegerVector
        | ParamType::ComplexVector
        | ParamType::Complex => definition
            .with_type(PropertyType::String)
            .with_default(PropertyValue::string("")),
        _ => definition
            .with_type(PropertyType::Expression)
            .with_default(PropertyValue::number(parameter.default)),
    };

    if let Some(unit) = facet.unit {
        definition = definition.with_unit(unit);
    }
    let description = if parameter.description.is_empty() {
        fallback_description
    } else {
        parameter.description.as_str()
    };
    if !description.is_empty() {
        definition = definition.with_description(description);
    }
    if parameter.required {
        definition = definition.required();
    }
    definition
}

struct CodeModelFacet {
    category: &'static str,
    unit: Option<&'static str>,
}

/// Group and unit for a parameter, keyed off the naming convention the XSPICE
/// catalog follows throughout: `*_delay` and `t_*` are times, `*_load` are
/// capacitive loads, and the rest shape the transfer behavior.
fn code_model_facet(name: &str, param_type: rspice_core::xspice::ParamType) -> CodeModelFacet {
    use rspice_core::xspice::ParamType;

    // `inertial_delay` is a mode flag that happens to end in `_delay`, so the
    // type has to decide before the name does.
    if matches!(param_type, ParamType::Boolean) || name == "family" {
        return CodeModelFacet {
            category: if name == "family" || name == "inertial_delay" {
                "Digital"
            } else {
                "Behavior"
            },
            unit: None,
        };
    }
    if name.ends_with("_delay") || name == "delay" || name == "t_rise" || name == "t_fall" {
        return CodeModelFacet {
            category: "Timing",
            unit: Some("s"),
        };
    }
    if name.ends_with("_load") {
        return CodeModelFacet {
            category: "Loading",
            unit: Some("F"),
        };
    }
    if name == "ic" || name == "out_ic" {
        return CodeModelFacet {
            category: "Initial State",
            unit: None,
        };
    }
    CodeModelFacet {
        category: "Behavior",
        unit: None,
    }
}

/// Label and fallback description for a parameter name.
///
/// The catalog leaves several descriptions empty and its names are terse, so
/// the readable forms live here. An unlisted name falls back to a humanized
/// spelling rather than being rejected, so a code-model parameter added to the
/// core still reaches the editor.
fn code_model_label(name: &str) -> (String, &'static str) {
    const LABELS: &[(&str, &str, &str)] = &[
        ("in_offset", "Input Offset", ""),
        ("out_offset", "Output Offset", ""),
        ("in_gain", "Input Gain", ""),
        ("out_gain", "Output Gain", ""),
        ("num_offset", "Numerator Offset", ""),
        ("num_gain", "Numerator Gain", ""),
        ("den_offset", "Denominator Offset", ""),
        ("den_gain", "Denominator Gain", ""),
        ("den_lower_limit", "Denominator Floor", ""),
        ("den_domain", "Denominator Smoothing", ""),
        ("out_lower_limit", "Lower Limit", ""),
        ("out_upper_limit", "Upper Limit", ""),
        ("limit_range", "Smoothing Range", ""),
        ("out_ic", "Initial Output", ""),
        ("ic", "Initial State", ""),
        (
            "fraction",
            "Fractional Range",
            "Read the smoothing range as a fraction rather than an absolute value",
        ),
        (
            "inertial_delay",
            "Inertial Delay",
            "Cancel a pending transition when the input reverts before the delay elapses",
        ),
        (
            "family",
            "Logic Family",
            "Named logic family supplying the loading and level defaults",
        ),
        (
            "input_load",
            "Input Load",
            "Capacitive load presented on each input",
        ),
        (
            "enable_load",
            "Enable Load",
            "Capacitive load presented on the enable input",
        ),
        (
            "data_load",
            "Data Load",
            "Capacitive load on the data input",
        ),
        (
            "clk_load",
            "Clock Load",
            "Capacitive load on the clock input",
        ),
        ("set_load", "Set Load", "Capacitive load on the set input"),
        (
            "reset_load",
            "Reset Load",
            "Capacitive load on the reset input",
        ),
        (
            "jk_load",
            "J/K Load",
            "Capacitive load on the J and K inputs",
        ),
        (
            "sr_load",
            "S/R Load",
            "Capacitive load on the S and R inputs",
        ),
        ("sr_delay", "S/R Delay", ""),
        ("clk_delay", "Clock Delay", ""),
        ("enable_delay", "Enable Delay", ""),
        ("set_delay", "Set Delay", ""),
        ("reset_delay", "Reset Delay", ""),
        ("rise_delay", "Rise Delay", ""),
        ("fall_delay", "Fall Delay", ""),
        ("delay", "Delay", ""),
        ("t_rise", "Rise Time", "Output transition time low to high"),
        ("t_fall", "Fall Time", "Output transition time high to low"),
        (
            "in_low",
            "Input Low Threshold",
            "Analog level at or below which the input reads as a digital zero",
        ),
        (
            "in_high",
            "Input High Threshold",
            "Analog level at or above which the input reads as a digital one",
        ),
        ("out_low", "Output Low", "Analog level driven for a zero"),
        ("out_high", "Output High", "Analog level driven for a one"),
        (
            "out_undef",
            "Output Undefined",
            "Analog level driven for an unknown state",
        ),
        ("gain", "Gain", ""),
    ];

    for (key, label, description) in LABELS {
        if name == *key {
            return ((*label).to_owned(), description);
        }
    }
    (humanize(name), "")
}

fn humanize(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut characters = word.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every parameter an XSPICE code model accepts must be reachable from its
    /// property sheet.
    ///
    /// The sheets are generated from the same registry the model card is built
    /// from, so this passes by construction — it is here to fail loudly if
    /// anyone reintroduces a hand-written sheet.
    #[test]
    fn every_code_model_parameter_is_reachable_from_its_property_sheet() {
        let registry = PropertyRegistry::new();
        let models = rspice_core::xspice::CodeModelRegistry::with_builtins();
        let mut unreachable = Vec::new();

        for kind in ComponentType::ALL {
            let DeviceImplementation::Xspice { model_type } = kind.descriptor().implementation
            else {
                continue;
            };
            let Some(model) = models.get(model_type) else {
                panic!("{model_type} has a schematic component but no code model");
            };
            let sheet = registry
                .get(kind)
                .unwrap_or_else(|| panic!("{model_type} has no property sheet"));
            for parameter in model.parameters() {
                if !sheet
                    .iter()
                    .any(|definition| definition.name.eq_ignore_ascii_case(&parameter.name))
                {
                    unreachable.push(format!("{model_type}.{}", parameter.name));
                }
            }
        }

        assert!(
            unreachable.is_empty(),
            "code-model parameters with no editor field: {}",
            unreachable.join(", ")
        );
    }

    /// A generated sheet still has to read like a hand-written one: every field
    /// labelled, grouped, and — where the quantity has one — carrying a unit.
    #[test]
    fn generated_sheets_are_labelled_grouped_and_united() {
        let registry = PropertyRegistry::new();
        let sheet = registry
            .get(ComponentType::XspiceDFlipFlop)
            .expect("D flip-flop sheet");

        let field = |name: &str| {
            sheet
                .iter()
                .find(|definition| definition.name == name)
                .unwrap_or_else(|| panic!("{name} is missing"))
        };
        assert_eq!(field("clk_delay").display_name, "Clock Delay");
        assert_eq!(field("clk_delay").category, "Timing");
        assert_eq!(field("clk_delay").unit.as_deref(), Some("s"));
        assert_eq!(field("clk_load").category, "Loading");
        assert_eq!(field("clk_load").unit.as_deref(), Some("F"));
        assert_eq!(field("ic").category, "Initial State");
        assert!(
            sheet
                .iter()
                .all(|definition| !definition.display_name.is_empty()),
            "every generated field carries a label"
        );
    }
}
