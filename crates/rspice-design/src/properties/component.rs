//! Component field mapping, exact property serialization, and source contracts.

use std::collections::HashMap;

use super::{PropertySheet, PropertyValue};
use crate::parameters::{format_params_string, parse_params_string};
use crate::schematic::component::{Component, InstanceMultiplicity};
use crate::schematic::component_type::ComponentType;

/// Returns the primary property name for a given component type.
///
/// In SPICE, each component type has a primary parameter:
/// - Resistor: "r" (resistance in Ohms)
/// - Capacitor: "c" (capacitance in Farads)
/// - Inductor: "l" (inductance in Henries)
/// - VoltageSource: "dc" (DC voltage)
/// - CurrentSource: "dc" (DC current)
/// - Diode: "is" (saturation current, but typically just uses model)
/// - MOSFET: "w" (width) or model-dependent
/// - BJT: model-dependent
///
/// This matches Cadence Spectre's component definition format.
pub fn get_primary_property_name(kind: ComponentType) -> &'static str {
    match kind {
        ComponentType::Resistor => "r",
        ComponentType::Capacitor => "c",
        ComponentType::Inductor => "l",
        ComponentType::Transformer => "lp",
        ComponentType::CoupledInductor => "k",
        ComponentType::VoltageSource => "dc",
        ComponentType::VoltageSourceAc => "ac",
        ComponentType::VoltageSourcePulse => "v1",
        ComponentType::VoltageSourceSin => "vo",
        ComponentType::VoltageSourceExp => "v1",
        ComponentType::VoltageSourceSffm => "vo",
        ComponentType::VoltageSourceAm => "vo",
        ComponentType::VoltageSourcePat => "vhi",
        ComponentType::VoltageSourceNoise => "na",
        ComponentType::CurrentSource => "dc",
        ComponentType::CurrentSourceAc => "ac",
        ComponentType::CurrentSourcePulse => "i1",
        ComponentType::CurrentSourceSin => "io",
        ComponentType::CurrentSourceExp => "i1",
        ComponentType::CurrentSourceSffm => "vo",
        ComponentType::CurrentSourceAm => "vo",
        ComponentType::CurrentSourcePat => "vhi",
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => "pwl_data",
        ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => "file",
        // TRRANDOM leads its card with TYPE, which is a distribution name
        // rather than a quantity; the sample interval is the number that tells
        // two draws of the same distribution apart, so that is the field the
        // canvas value carries. See `independent_source_parameter_names`, where
        // `ts` is likewise the one field the primary value falls back to.
        ComponentType::VoltageSourceRandom | ComponentType::CurrentSourceRandom => "ts",
        ComponentType::Diode
        | ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos
        | ComponentType::NmosSoi
        | ComponentType::PmosSoi
        | ComponentType::NpnBjt
        | ComponentType::PnpBjt
        | ComponentType::NpnBjt4
        | ComponentType::PnpBjt4
        | ComponentType::NpnBjt5
        | ComponentType::PnpBjt5
        | ComponentType::Njfet
        | ComponentType::Pjfet
        | ComponentType::Nmesfet
        | ComponentType::Pmesfet => "model",
        ComponentType::Vcvs => "gain",
        ComponentType::Vccs => "gm",
        ComponentType::Ccvs => "rm",
        ComponentType::Cccs => "gain",
        ComponentType::OpAmp => "gain",
        ComponentType::GenericSwitch => "control",
        ComponentType::CurrentSourceNoise => "na",
        ComponentType::Ground => "name",
        // Catch-all for any other component types
        _ => "value",
    }
}

pub fn collect_properties_with_sheet(
    component: &Component,
    sheet: Option<&PropertySheet>,
) -> HashMap<String, PropertyValue> {
    let mut properties = HashMap::new();

    // Always include instance name
    properties.insert(
        "name".to_string(),
        PropertyValue::String(component.name.clone()),
    );

    if let Some(sheet) = sheet
        && let Some(def) = sheet.iter().find(|def| def.name == "symbol")
        && let PropertyValue::Enum { options, .. } = &def.default_value
    {
        let selected = component
            .symbol_variant
            .clone()
            .unwrap_or_else(|| options.first().cloned().unwrap_or_default());
        properties.insert(
            "symbol".to_string(),
            PropertyValue::Enum {
                selected,
                options: options.clone(),
            },
        );
    }

    // Get the primary property name for this component type
    let primary_prop = get_primary_property_name(component.kind);

    // Parse the component value as the primary property. A newly placed PWL
    // source has no authored value yet, so seed its transaction from the CDF
    // default at this bridge boundary. Once inside a transaction, an explicit
    // empty string remains authored input and is rejected by the PWL editor.
    if component.value.is_empty() && component.kind.is_pwl_source() {
        if let Some(default) = sheet
            .and_then(|sheet| sheet.iter().find(|def| def.name == primary_prop))
            .map(|def| def.default_value.clone())
        {
            properties.insert(primary_prop.to_owned(), default);
        }
    } else if !component.value.is_empty() {
        // A file path is text, never a quantity: routing one through the
        // expression editor would have it parsed for units and a drive letter
        // read as an exponent.
        let value = if component.kind.is_pwl_source()
            || component.kind.is_pwl_file_source()
            || component.kind == ComponentType::Port
        {
            PropertyValue::String(component.value.clone())
        } else {
            PropertyValue::Expression(component.value.clone())
        };
        properties.insert(primary_prop.to_string(), value);
    }

    // A legacy Port may only carry `dir=`. Materialize the complete typed
    // contract at the property boundary so every schema field is editable and
    // applying the unchanged form upgrades it to the durable representation.
    // Canonical contract values intentionally win over legacy aliases such as
    // `input`, `digital`, or `real`.
    if component.kind == ComponentType::Port
        && let Some(contract) = component.port_contract()
    {
        for (key, value) in [
            ("dir", contract.direction.keyword().to_owned()),
            ("signal_type", contract.signal_type.keyword().to_owned()),
            ("discipline", contract.discipline.keyword().to_owned()),
            ("documentation", contract.documentation),
        ] {
            properties.insert(
                key.to_owned(),
                property_value_from_schema(key, value, sheet),
            );
        }
    }

    // Parse additional parameters from params string
    let parsed_params = parse_params_string(&component.params);
    for (key, value) in parsed_params {
        // Skip if this is the primary property (already handled)
        if key == primary_prop {
            continue;
        }

        if component.kind == ComponentType::Port && is_port_contract_property(&key) {
            continue;
        }

        let prop_value = property_value_from_schema(&key, value, sheet);

        properties.insert(key, prop_value);
    }

    // On a cell instance `m` is the flattener's physical multiplier, which
    // lives in a typed field rather than in parameter text. It is offered here
    // unconditionally, and an instance standing for itself reads as 1, because
    // an editor that hides the reserved name until someone has already set it
    // is an editor that cannot set it.
    if component.kind == ComponentType::CellInstance {
        properties.insert(
            InstanceMultiplicity::PARAMETER_NAME.to_owned(),
            PropertyValue::number(
                component
                    .multiplicity
                    .map_or(1.0, InstanceMultiplicity::value),
            ),
        );
    }

    properties
}

/// Everything the engine will do with this source's fields, at both strengths.
///
/// The rules live in `super::source_contract`; this is the one
/// place a placed component is resolved into the field set they read, so no
/// surface can assemble a different view of the same instance. A component that
/// is not an independent source resolves to an empty list.
pub fn component_source_contract(
    component: &Component,
    values: &HashMap<String, PropertyValue>,
    sheet: &crate::properties::PropertySheet,
) -> Vec<crate::properties::SourceContractFinding> {
    let params = parse_params_string(&component.params);
    let primary = get_primary_property_name(component.kind);
    let fields = crate::properties::SourceFields::new(values, sheet, &params)
        .with_primary(primary, &component.value);
    crate::properties::source_contract_findings(component.kind, &fields)
}

/// Why this source's fields may not be committed to the design, if they may not.
///
/// Every path that writes a source parameter asks this one function: the modal
/// component editor's commit, and the inspector's inline field editor. A
/// refusal only one of them enforces is not a refusal — the `TD=-1n` the dialog
/// turned away was going in through the inline field beside it.
pub fn source_commit_refusal(
    component: &Component,
    values: &HashMap<String, PropertyValue>,
    sheet: &crate::properties::PropertySheet,
) -> Option<String> {
    component_source_contract(component, values, sheet)
        .into_iter()
        .find(|finding| finding.strength == crate::properties::ContractStrength::Refusal)
        .map(|finding| finding.message)
}

/// Applies properties from a PropertyValue HashMap back to a Component.
///
/// This is the "write" direction of the bridge - saving edited properties
/// from the property dialog back to the Component struct.
///
/// # Process
/// 1. Update `component.name` from "name" property
/// 2. Update `component.value` from primary property (r/c/l/dc/etc.)
/// 3. Serialize remaining properties to `component.params`
///
/// # Arguments
/// * `component` - The component to update
/// * `properties` - HashMap of edited property values
/// * `sheet` - The component's authoritative schema, used for default filtering
///
/// Malformed or duplicate existing parameters return an error before any
/// field changes, so a property-map round trip cannot discard authored text.
pub fn apply_properties_with_sheet(
    component: &mut Component,
    properties: &HashMap<String, PropertyValue>,
    sheet: Option<&PropertySheet>,
) -> Result<(), String> {
    crate::parameters::validate_parameter_text(&component.params)?;
    let primary_prop = get_primary_property_name(component.kind);

    // Update instance name
    if let Some(PropertyValue::String(name)) = properties.get("name") {
        // Validation is performed against the normalized reference; persist
        // that same identity so whitespace cannot create visually identical
        // but electrically distinct instance names.
        component.name = name.trim().to_owned();
    }

    // Update primary value
    if let Some(prop_value) = properties.get(primary_prop) {
        component.value = property_value_to_string(prop_value);
    }

    if let Some(PropertyValue::Enum { selected, .. }) = properties.get("symbol") {
        component.symbol_variant = if selected.is_empty() || selected == "default" {
            None
        } else {
            Some(selected.clone())
        };
    }

    // `m` returns to the typed field it was read from. A value the field
    // cannot hold, and the plain 1 that means "stands for itself", both leave
    // the instance carrying no multiplier at all rather than a stored default
    // the netlist would then have to emit.
    if component.kind == ComponentType::CellInstance {
        component.multiplicity = properties
            .get(InstanceMultiplicity::PARAMETER_NAME)
            .map(property_value_to_string)
            .and_then(|authored| InstanceMultiplicity::parse(&authored).ok())
            .filter(|multiplicity| multiplicity.value() != 1.0);
    }

    // Collect secondary parameters
    let mut secondary_params: HashMap<String, String> = if component.kind == ComponentType::Port {
        // Port contracts can coexist with future/extension metadata. Preserve
        // fields unknown to this property sheet instead of erasing them while
        // upgrading a legacy contract.
        parse_params_string(&component.params)
    } else {
        HashMap::new()
    };

    for (key, value) in properties {
        // Skip name and primary property, and the reserved multiplicity that
        // the typed field above already took: on a cell instance `m` is never
        // free-text parameter text.
        if key == "name"
            || key == primary_prop
            || key == "symbol"
            || key == "model_corner"
            || (component.kind == ComponentType::CellInstance
                && key == InstanceMultiplicity::PARAMETER_NAME)
        {
            continue;
        }

        // Skip empty values
        let string_value = property_value_to_string(value);
        if string_value.is_empty() {
            continue;
        }

        // Check if this value differs from the default
        let is_default = if let Some(sheet) = sheet {
            if let Some(def) = sheet.iter().find(|d| d.name == *key) {
                property_values_equal(value, &def.default_value)
            } else {
                false // Unknown property, include it
            }
        } else {
            false
        };

        // Port contract fields stay explicit even at schema defaults. This
        // makes their durable meaning independent of defaults changing in a
        // future release and upgrades legacy `dir=`-only components safely.
        if !is_default || (component.kind == ComponentType::Port && is_port_contract_property(key))
        {
            secondary_params.insert(key.clone(), string_value);
        } else if component.kind == ComponentType::Port {
            // Only the Port branch seeds `secondary_params` from the existing
            // params, so only it can carry a stale entry past an edit that
            // returned the field to its default. Every other kind starts empty
            // and drops defaults by simply not inserting them.
            secondary_params.remove(key);
        }
    }

    if component.kind == ComponentType::Port
        && let Some(contract) = component.port_contract()
    {
        secondary_params
            .entry("dir".to_owned())
            .or_insert_with(|| contract.direction.keyword().to_owned());
        secondary_params
            .entry("signal_type".to_owned())
            .or_insert_with(|| contract.signal_type.keyword().to_owned());
        secondary_params
            .entry("discipline".to_owned())
            .or_insert_with(|| contract.discipline.keyword().to_owned());
        secondary_params
            .entry("documentation".to_owned())
            .or_insert(contract.documentation);
    }

    // Format secondary parameters into params string
    component.params = format_params_string(&secondary_params);
    Ok(())
}

fn is_port_contract_property(key: &str) -> bool {
    matches!(key, "dir" | "signal_type" | "discipline" | "documentation")
}

fn property_value_from_schema(
    key: &str,
    value: String,
    sheet: Option<&PropertySheet>,
) -> PropertyValue {
    let Some(definition) =
        sheet.and_then(|sheet| sheet.iter().find(|definition| definition.name == key))
    else {
        return PropertyValue::Expression(value);
    };

    match &definition.default_value {
        PropertyValue::Number { .. } => {
            if let Ok(number) = rspice_app_types::quantity::parse_engineering_value(&value) {
                PropertyValue::Number {
                    value: number,
                    unit: definition.unit.clone(),
                }
            } else {
                PropertyValue::Expression(value)
            }
        }
        PropertyValue::String(_) => PropertyValue::String(value),
        PropertyValue::Expression(_) => PropertyValue::Expression(value),
        PropertyValue::Boolean(_) => PropertyValue::Boolean(matches!(
            value.to_lowercase().as_str(),
            "true" | "1" | "yes" | "on"
        )),
        PropertyValue::Enum { options, .. } => PropertyValue::Enum {
            selected: value,
            options: options.clone(),
        },
    }
}

/// Converts a PropertyValue to its string representation.
///
/// Used for serialization to SPICE netlist format.
pub fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        // Rust's finite-f64 display is the shortest decimal that round-trips
        // to the same binary value. This boundary feeds the durable SPICE
        // model, so presentation-oriented engineering formatting (which may
        // round) must never be used here.
        PropertyValue::Number { value, .. } => value.to_string(),
        PropertyValue::String(s) => s.clone(),
        PropertyValue::Expression(expr) => expr.clone(),
        PropertyValue::Enum { selected, .. } => selected.clone(),
        PropertyValue::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
    }
}

/// Compares two PropertyValues for equality (ignoring units in numbers).
///
/// Used to detect if a value has been modified from the default.
fn property_values_equal(a: &PropertyValue, b: &PropertyValue) -> bool {
    match (a, b) {
        (PropertyValue::Number { value: va, .. }, PropertyValue::Number { value: vb, .. }) => {
            va == vb || (va.is_nan() && vb.is_nan())
        }
        (PropertyValue::String(sa), PropertyValue::String(sb)) => sa == sb,
        (PropertyValue::Expression(ea), PropertyValue::Expression(eb)) => ea == eb,
        (PropertyValue::Enum { selected: sa, .. }, PropertyValue::Enum { selected: sb, .. }) => {
            sa == sb
        }
        (PropertyValue::Boolean(ba), PropertyValue::Boolean(bb)) => ba == bb,
        // Different types are not equal
        _ => false,
    }
}

#[cfg(test)]
mod pwl_tests;
#[cfg(test)]
mod tests;
