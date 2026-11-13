//! The built-in XSPICE interface registry is internally coherent: every
//! registered name resolves, ports and parameters are named and unique, and
//! declared bounds describe a non-empty range. The registry is assembled in
//! code rather than generated from a checked-in catalog, so nothing else
//! proves what a release ships is coherent.

use std::collections::HashSet;

use rspice_core::xspice::{
    CodeModel, CodeModelRegistry, ParamType, PortDirection, PortSpec, PortType,
};

/// Interfaces whose exact port and parameter contract is pinned by
/// [`validate_pinned_contract`].
const PINNED_CONTRACT_INTERFACES: &[&str] = &["d_vcd", "pspice_d_stim", "v_to_real"];

fn validate_registry(registry: &CodeModelRegistry, model_names: &[&str]) -> Vec<String> {
    let mut errors = Vec::new();

    for model_name in PINNED_CONTRACT_INTERFACES {
        let Some(model) = registry.get(model_name) else {
            errors.push(format!("pinned interface '{model_name}' is not registered"));
            continue;
        };
        validate_pinned_contract(model.as_ref(), &mut errors);
    }

    let mut seen_names = HashSet::new();
    for &model_name in model_names {
        if !seen_names.insert(model_name.to_ascii_lowercase()) {
            errors.push(format!(
                "two registered interfaces answer to the name '{model_name}'"
            ));
        }

        let Some(model) = registry.get(model_name) else {
            errors.push(format!(
                "'{model_name}' is listed by the registry but does not resolve"
            ));
            continue;
        };
        if model.name() != model_name {
            errors.push(format!(
                "'{model_name}' resolves to an interface that calls itself '{}'",
                model.name()
            ));
        }
        if model_name.trim().is_empty() {
            errors.push("an interface is registered under an empty name".to_owned());
            continue;
        }
        if model_name.chars().any(char::is_whitespace) {
            errors.push(format!("interface name '{model_name}' contains whitespace"));
        }
        // A deck names a model in whatever case its author typed. The registry
        // folds the lookup, so an interface whose own name is not already
        // folded would print here under a spelling its key does not use.
        if model_name.to_ascii_lowercase() != model_name {
            errors.push(format!(
                "interface name '{model_name}' is not lowercase, so it does not match its registry key"
            ));
        }
        if !registry.contains(&model_name.to_ascii_uppercase()) {
            errors.push(format!(
                "interface '{model_name}' is not reachable case-insensitively"
            ));
        }

        if model.ports().is_empty() {
            errors.push(format!("interface '{model_name}' declares no ports"));
        }

        let mut port_names = HashSet::new();
        for port in model.ports() {
            if port.name.trim().is_empty() {
                errors.push(format!("interface '{model_name}' has an unnamed port"));
                continue;
            }
            if !port_names.insert(port.name.to_ascii_lowercase()) {
                errors.push(format!(
                    "interface '{model_name}' repeats port name '{}'",
                    port.name
                ));
            }
            if port.allowed_types.is_empty() {
                errors.push(format!(
                    "interface '{model_name}' port '{}' allows no port type",
                    port.name
                ));
            } else if !port.allowed_types.contains(&port.default_type) {
                errors.push(format!(
                    "interface '{model_name}' port '{}' defaults to {:?}, which it does not allow",
                    port.name, port.default_type
                ));
            }
            match (port.vector_min_len, port.vector_max_len) {
                (Some(minimum), Some(maximum)) if minimum > maximum => errors.push(format!(
                    "interface '{model_name}' port '{}' accepts between {minimum} and {maximum} connections",
                    port.name
                )),
                _ => {}
            }
            if !port.is_vector && (port.vector_min_len.is_some() || port.vector_max_len.is_some()) {
                errors.push(format!(
                    "interface '{model_name}' port '{}' is scalar but declares vector bounds",
                    port.name
                ));
            }
        }

        let mut parameter_names = HashSet::new();
        for parameter in model.parameters() {
            if parameter.name.trim().is_empty() {
                errors.push(format!("interface '{model_name}' has an unnamed parameter"));
                continue;
            }
            if !parameter_names.insert(parameter.name.to_ascii_lowercase()) {
                errors.push(format!(
                    "interface '{model_name}' repeats parameter name '{}'",
                    parameter.name
                ));
            }
            if parameter.min.is_some_and(|bound| !bound.is_finite())
                || parameter.max.is_some_and(|bound| !bound.is_finite())
            {
                errors.push(format!(
                    "interface '{model_name}' parameter '{}' has a non-finite bound",
                    parameter.name
                ));
            }
            if let (Some(minimum), Some(maximum)) = (parameter.min, parameter.max)
                && minimum > maximum
            {
                errors.push(format!(
                    "interface '{model_name}' parameter '{}' has an empty range [{minimum}, {maximum}]",
                    parameter.name
                ));
            }
        }
    }

    errors
}

fn validate_pinned_contract(model: &dyn CodeModel, errors: &mut Vec<String>) {
    match model.name() {
        "pspice_d_stim" => {
            expect_description(model, "PSpice STIM digital stimulus", errors);
            expect_ports(
                model,
                &[ExpectedPort {
                    name: "out",
                    direction: PortDirection::Out,
                    default_type: PortType::Digital,
                    allowed_types: &[PortType::Digital],
                    is_vector: true,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                }],
                errors,
            );
            expect_parameters(model, &[("stim_program", ParamType::String)], errors);
            if let [parameter] = model.parameters()
                && (parameter.string_default.as_deref() != Some("")
                    || parameter.required
                    || parameter.min.is_some()
                    || parameter.max.is_some())
            {
                errors.push(
                    "interface 'pspice_d_stim' parameter 'stim_program' must remain an optional, unbounded string with an empty default"
                        .to_owned(),
                );
            }
        }
        "v_to_real" => {
            expect_description(
                model,
                "Analog input sampled into a real-valued event",
                errors,
            );
            expect_ports(
                model,
                &[
                    ExpectedPort {
                        name: "in",
                        direction: PortDirection::In,
                        default_type: PortType::Voltage,
                        allowed_types: &[
                            PortType::Voltage,
                            PortType::DifferentialVoltage,
                            PortType::Current,
                            PortType::DifferentialCurrent,
                            PortType::VoltageName,
                        ],
                        is_vector: false,
                        null_allowed: false,
                        vector_min_len: None,
                        vector_max_len: None,
                    },
                    ExpectedPort {
                        name: "out",
                        direction: PortDirection::Out,
                        default_type: PortType::Real,
                        allowed_types: &[PortType::Real],
                        is_vector: false,
                        null_allowed: false,
                        vector_min_len: None,
                        vector_max_len: None,
                    },
                ],
                errors,
            );
            expect_parameters(model, &[("gain", ParamType::Real)], errors);
            if let [parameter] = model.parameters()
                && (parameter.default != 1.0
                    || parameter.required
                    || parameter.min.is_some()
                    || parameter.max.is_some())
            {
                errors.push(
                    "interface 'v_to_real' parameter 'gain' must remain an optional, unbounded real with default 1"
                        .to_owned(),
                );
            }
        }
        "d_vcd" => {
            expect_description(model, "Digital stimulus replayed from a VCD file", errors);
            expect_ports(
                model,
                &[
                    ExpectedPort {
                        name: "out",
                        direction: PortDirection::Out,
                        default_type: PortType::Digital,
                        allowed_types: &[PortType::Digital],
                        is_vector: true,
                        null_allowed: true,
                        vector_min_len: None,
                        vector_max_len: None,
                    },
                    ExpectedPort {
                        name: "real_out",
                        direction: PortDirection::Out,
                        default_type: PortType::Real,
                        allowed_types: &[PortType::Real],
                        is_vector: true,
                        null_allowed: true,
                        vector_min_len: None,
                        vector_max_len: None,
                    },
                ],
                errors,
            );
            expect_parameters(
                model,
                &[
                    ("file", ParamType::String),
                    ("signals", ParamType::String),
                    ("toffset", ParamType::Real),
                ],
                errors,
            );
        }
        name => errors.push(format!(
            "interface '{name}' is listed as pinned but has no catalog contract"
        )),
    }
}

#[derive(Clone, Copy)]
struct ExpectedPort {
    name: &'static str,
    direction: PortDirection,
    default_type: PortType,
    allowed_types: &'static [PortType],
    is_vector: bool,
    null_allowed: bool,
    vector_min_len: Option<usize>,
    vector_max_len: Option<usize>,
}

fn expect_description(model: &dyn CodeModel, expected: &str, errors: &mut Vec<String>) {
    if model.description() != expected {
        errors.push(format!(
            "interface '{}' description changed: expected '{expected}', found '{}'",
            model.name(),
            model.description()
        ));
    }
}

fn expect_ports(model: &dyn CodeModel, expected: &[ExpectedPort], errors: &mut Vec<String>) {
    let actual = model.ports();
    if actual.len() != expected.len() {
        errors.push(format!(
            "interface '{}' port contract changed: expected {} ports, found {}",
            model.name(),
            expected.len(),
            actual.len()
        ));
        return;
    }

    for (actual, expected) in actual.iter().zip(expected) {
        if !port_matches(actual, *expected) {
            errors.push(format!(
                "interface '{}' port '{}' no longer matches its pinned descriptor contract",
                model.name(),
                expected.name
            ));
        }
    }
}

fn port_matches(actual: &PortSpec, expected: ExpectedPort) -> bool {
    actual.name == expected.name
        && actual.direction == expected.direction
        && actual.default_type == expected.default_type
        && actual.allowed_types == expected.allowed_types
        && actual.is_vector == expected.is_vector
        && actual.null_allowed == expected.null_allowed
        && actual.vector_min_len == expected.vector_min_len
        && actual.vector_max_len == expected.vector_max_len
}

fn expect_parameters(
    model: &dyn CodeModel,
    expected: &[(&str, ParamType)],
    errors: &mut Vec<String>,
) {
    let actual = model.parameters();
    if actual.len() != expected.len() {
        errors.push(format!(
            "interface '{}' parameter contract changed: expected {} parameters, found {}",
            model.name(),
            expected.len(),
            actual.len()
        ));
        return;
    }

    for (actual, (expected_name, expected_type)) in actual.iter().zip(expected) {
        if actual.name != *expected_name || actual.param_type != *expected_type {
            errors.push(format!(
                "interface '{}' parameter '{}' no longer matches its pinned descriptor contract",
                model.name(),
                expected_name
            ));
        }
    }
}

#[test]
fn complete_builtin_registry_passes_the_release_audit() {
    let registry = CodeModelRegistry::with_builtins();
    let mut model_names = registry.model_names();
    model_names.sort_unstable();

    let errors = validate_registry(&registry, &model_names);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}
