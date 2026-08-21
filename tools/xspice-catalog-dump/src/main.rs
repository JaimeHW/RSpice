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

use rspice_core::xspice::CodeModelRegistry;

/// Number of built-in interfaces a shipped registry exposes.
///
/// The registry has no external artifact to diff against, so this is the
/// stand-in: a model that disappears because its registration was dropped in a
/// refactor fails here rather than at the first deck that names it. Update it
/// in the same commit that adds or removes a `register` call, and say which
/// model moved.
const EXPECTED_BUILTIN_MODEL_COUNT: usize = 113;

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
