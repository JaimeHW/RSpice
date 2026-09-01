//! Emit the executable built-in XSPICE interface registry in a reviewable form.
//!
//! With no arguments the compiled registry is printed model by model.
//! `--validate-only` audits it instead and prints one line, which is the form
//! CI runs: the registry is assembled in code rather than generated from a
//! checked-in catalog, so nothing else proves that what a release ships is
//! internally coherent — that every registered name resolves, that ports and
//! parameters are named and unique, and that the declared bounds describe a
//! non-empty range.

use std::collections::HashSet;
use std::process::ExitCode;

use rspice_core::xspice::{
    CodeModel, CodeModelRegistry, ParamType, PortDirection, PortSpec, PortType,
};

/// Number of built-in interfaces at the catalog-audit baseline.
///
/// The registry had 113 interfaces when this audit was introduced. New models
/// belong in [`BUILTIN_ADDITIONS_SINCE_BASELINE`] with an identity-specific
/// descriptor check; keeping the baseline separate prevents a bare count bump
/// from silently blessing an unrelated registration.
const BUILTIN_MODEL_BASELINE_COUNT: usize = 113;

/// Intentional additions since the 113-model audit baseline.
///
/// These are not merely names used to adjust the expected count. Each entry is
/// matched below and its executable port/parameter contract is pinned by
/// [`validate_added_builtin_contract`].
const BUILTIN_ADDITIONS_SINCE_BASELINE: &[&str] = &["pspice_d_stim", "v_to_real"];

const EXPECTED_BUILTIN_MODEL_COUNT: usize =
    BUILTIN_MODEL_BASELINE_COUNT + BUILTIN_ADDITIONS_SINCE_BASELINE.len();

fn main() -> ExitCode {
    let validate_only = match parse_arguments() {
        Ok(validate_only) => validate_only,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("usage: rspice-xspice-catalog-dump [--validate-only]");
            return ExitCode::FAILURE;
        }
    };

    let registry = CodeModelRegistry::with_builtins();
    let mut model_names = registry.model_names();
    model_names.sort_unstable();

    let errors = validate_registry(&registry, &model_names);
    if !errors.is_empty() {
        eprintln!("built-in XSPICE registry audit failed:");
        for error in errors {
            eprintln!("  - {error}");
        }
        return ExitCode::FAILURE;
    }

    if validate_only {
        println!(
            "built-in XSPICE registry is valid: models={}",
            model_names.len()
        );
        return ExitCode::SUCCESS;
    }

    for model_name in model_names {
        let model = registry
            .get(model_name)
            .expect("a name returned by the registry must resolve");
        println!("MODEL {model_name}");
        for port in model.ports() {
            println!(
                "  PORT {} {:?} {:?} vector={} null={} min={:?} max={:?}",
                port.name,
                port.direction,
                port.default_type,
                port.is_vector,
                port.null_allowed,
                port.vector_min_len,
                port.vector_max_len
            );
        }
        for parameter in model.parameters() {
            println!(
                "  PARAM {} {:?} required={} min={:?} max={:?}",
                parameter.name,
                parameter.param_type,
                parameter.required,
                parameter.min,
                parameter.max
            );
        }
    }

    ExitCode::SUCCESS
}

fn parse_arguments() -> Result<bool, String> {
    let mut validate_only = false;
    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "--validate-only" if !validate_only => validate_only = true,
            "--validate-only" => return Err("--validate-only was specified more than once".into()),
            _ => return Err(format!("unknown argument '{argument}'")),
        }
    }
    Ok(validate_only)
}

fn validate_registry(registry: &CodeModelRegistry, model_names: &[&str]) -> Vec<String> {
    let mut errors = Vec::new();

    if model_names.len() != EXPECTED_BUILTIN_MODEL_COUNT {
        errors.push(format!(
            "registry exposes {} built-in models; a shipped registry must expose exactly {EXPECTED_BUILTIN_MODEL_COUNT}",
            model_names.len()
        ));
    }

    for model_name in BUILTIN_ADDITIONS_SINCE_BASELINE {
        let Some(model) = registry.get(model_name) else {
            errors.push(format!(
                "intentional post-baseline interface '{model_name}' is not registered"
            ));
            continue;
        };
        validate_added_builtin_contract(model.as_ref(), &mut errors);
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

fn validate_added_builtin_contract(model: &dyn CodeModel, errors: &mut Vec<String>) {
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
        name => errors.push(format!(
            "post-baseline interface '{name}' has no identity-specific catalog contract"
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
                "interface '{}' port '{}' no longer matches its post-baseline descriptor contract",
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
                "interface '{}' parameter '{}' no longer matches its post-baseline descriptor contract",
                model.name(),
                expected_name
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_baseline_models_have_exact_catalog_contracts() {
        let registry = CodeModelRegistry::with_builtins();
        let mut errors = Vec::new();

        for model_name in BUILTIN_ADDITIONS_SINCE_BASELINE {
            let model = registry
                .get(model_name)
                .unwrap_or_else(|| panic!("post-baseline model '{model_name}' must resolve"));
            validate_added_builtin_contract(model.as_ref(), &mut errors);
        }

        assert!(errors.is_empty(), "{}", errors.join("\n"));
    }

    #[test]
    fn complete_builtin_registry_passes_the_release_audit() {
        let registry = CodeModelRegistry::with_builtins();
        let mut model_names = registry.model_names();
        model_names.sort_unstable();

        let errors = validate_registry(&registry, &model_names);
        assert!(errors.is_empty(), "{}", errors.join("\n"));
    }
}
