//! Validate and emit the generated Verilog-A catalog compiled into a release.

use std::collections::HashSet;
use std::process::ExitCode;

use rspice_core::device::veriloga_builtins::{
    GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION, GeneratedVerilogAModelDescriptor,
    generated_veriloga_model_descriptor, generated_veriloga_model_descriptors,
};

const EXPECTED_SHIPPED_MODEL_COUNT: usize = 42;

fn main() -> ExitCode {
    let validate_only = match parse_arguments() {
        Ok(validate_only) => validate_only,
        Err(error) => {
            eprintln!("{error}");
            eprintln!("usage: rspice-veriloga-catalog-dump [--validate-only]");
            return ExitCode::FAILURE;
        }
    };
    let descriptors = generated_veriloga_model_descriptors();
    let mut errors = validate_catalog(descriptors);

    if descriptors.len() != EXPECTED_SHIPPED_MODEL_COUNT {
        errors.push(format!(
            "compiled catalog contains {} models; a shipped catalog must contain exactly {EXPECTED_SHIPPED_MODEL_COUNT}",
            descriptors.len()
        ));
    }

    if !errors.is_empty() {
        eprintln!("generated Verilog-A catalog audit failed:");
        for error in errors {
            eprintln!("  - {error}");
        }
        return ExitCode::FAILURE;
    }

    if validate_only {
        println!(
            "generated Verilog-A catalog is valid: abi={} models={}",
            GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION,
            descriptors.len()
        );
        return ExitCode::SUCCESS;
    }

    println!(
        "CATALOG abi={} models={}",
        GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION,
        descriptors.len()
    );
    for descriptor in descriptors {
        dump_descriptor(descriptor);
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

fn validate_catalog(descriptors: &[GeneratedVerilogAModelDescriptor]) -> Vec<String> {
    let mut errors = Vec::new();
    let mut model_names = HashSet::new();

    for descriptor in descriptors {
        let label = if descriptor.model_name.is_empty() {
            "<unnamed>"
        } else {
            descriptor.model_name
        };
        if !model_names.insert(descriptor.model_name.to_ascii_lowercase()) {
            errors.push(format!("duplicate case-insensitive model name '{label}'"));
        }
        if descriptor.abi_version != GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION {
            errors.push(format!(
                "model '{label}' uses descriptor ABI {}, expected {}",
                descriptor.abi_version, GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION
            ));
        }
        if descriptor.model_name.trim().is_empty() {
            errors.push("a model has an empty model name".to_owned());
        }
        if descriptor.module_name.trim().is_empty() {
            errors.push(format!("model '{label}' has an empty module name"));
        }
        validate_digest(
            label,
            "canonical source fingerprint",
            descriptor.source_digest,
            16,
            &mut errors,
        );
        validate_digest(
            label,
            "checkpoint identity",
            descriptor.checkpoint_identity,
            64,
            &mut errors,
        );
        if descriptor.terminals.is_empty() {
            errors.push(format!("model '{label}' has no external terminals"));
        }
        if descriptor.terminals.len() + descriptor.internal_node_names.len()
            != descriptor.total_node_count
        {
            errors.push(format!(
                "model '{label}' declares {} total nodes but exposes {} terminals and {} internal nodes",
                descriptor.total_node_count,
                descriptor.terminals.len(),
                descriptor.internal_node_names.len()
            ));
        }

        let mut node_names = HashSet::new();
        for terminal in descriptor.terminals {
            if terminal.name.trim().is_empty() {
                errors.push(format!("model '{label}' has an empty terminal name"));
            }
            if terminal.discipline.trim().is_empty() {
                errors.push(format!(
                    "model '{label}' terminal '{}' has no discipline",
                    terminal.name
                ));
            }
            if !node_names.insert(terminal.name.to_ascii_lowercase()) {
                errors.push(format!(
                    "model '{label}' repeats node name '{}'",
                    terminal.name
                ));
            }
        }
        for internal_node in descriptor.internal_node_names {
            if internal_node.trim().is_empty() {
                errors.push(format!("model '{label}' has an empty internal node name"));
            }
            if !node_names.insert(internal_node.to_ascii_lowercase()) {
                errors.push(format!(
                    "model '{label}' repeats node name '{internal_node}'"
                ));
            }
        }

        let mut parameter_names = HashSet::new();
        for parameter in descriptor.parameters {
            if parameter.name.trim().is_empty() {
                errors.push(format!("model '{label}' has an empty parameter name"));
            }
            for name in std::iter::once(parameter.name).chain(parameter.aliases.iter().copied()) {
                if name.trim().is_empty() {
                    errors.push(format!(
                        "model '{label}' parameter '{}' has an empty alias",
                        parameter.name
                    ));
                }
                if !parameter_names.insert(name.to_ascii_lowercase()) {
                    errors.push(format!(
                        "model '{label}' repeats parameter name or alias '{name}'"
                    ));
                }
            }
            if parameter.default.is_some_and(|value| !value.is_finite()) {
                errors.push(format!(
                    "model '{label}' parameter '{}' has a non-finite literal default",
                    parameter.name
                ));
            }
            if parameter
                .minimum
                .is_some_and(|bound| !bound.value.is_finite())
                || parameter
                    .maximum
                    .is_some_and(|bound| !bound.value.is_finite())
                || parameter
                    .excluded_values
                    .iter()
                    .any(|value| !value.is_finite())
            {
                errors.push(format!(
                    "model '{label}' parameter '{}' has non-finite constraint metadata",
                    parameter.name
                ));
            }
            if let (Some(minimum), Some(maximum)) = (parameter.minimum, parameter.maximum)
                && (minimum.value > maximum.value
                    || (minimum.value == maximum.value && (minimum.exclusive || maximum.exclusive)))
            {
                errors.push(format!(
                    "model '{label}' parameter '{}' has an empty static range",
                    parameter.name
                ));
            }
        }

        match generated_veriloga_model_descriptor(descriptor.model_name) {
            Some(resolved) if std::ptr::eq(resolved, descriptor) => {}
            Some(_) => errors.push(format!(
                "model '{label}' resolves to a different descriptor instance"
            )),
            None => errors.push(format!(
                "model '{label}' cannot be resolved through the compiled registry"
            )),
        }
    }

    errors
}

fn validate_digest(
    model: &str,
    kind: &str,
    digest: &str,
    expected_len: usize,
    errors: &mut Vec<String>,
) {
    if digest.len() != expected_len || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        errors.push(format!(
            "model '{model}' has an invalid {kind}; expected {expected_len} hexadecimal characters"
        ));
    }
}

fn dump_descriptor(descriptor: &GeneratedVerilogAModelDescriptor) {
    println!(
        "MODEL {} module={} source={} checkpoint={} terminals={} parameters={} internal_nodes={} branches={}",
        descriptor.model_name,
        descriptor.module_name,
        descriptor.source_digest,
        descriptor.checkpoint_identity,
        descriptor.terminals.len(),
        descriptor.parameters.len(),
        descriptor.internal_node_names.len(),
        descriptor.branch_count
    );
    for terminal in descriptor.terminals {
        println!(
            "  TERMINAL {} direction={:?} discipline={}",
            terminal.name, terminal.direction, terminal.discipline
        );
    }
    for parameter in descriptor.parameters {
        println!(
            "  PARAMETER {} scope={:?} integer={} aliases={:?} default={:?} minimum={:?} maximum={:?} excluded={:?} dynamic_constraints={}",
            parameter.name,
            parameter.scope,
            parameter.is_integer,
            parameter.aliases,
            parameter.default,
            parameter.minimum,
            parameter.maximum,
            parameter.excluded_values,
            parameter.has_dynamic_constraints
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_catalog_passes_contract_audit() {
        let descriptors = generated_veriloga_model_descriptors();
        assert_eq!(descriptors.len(), EXPECTED_SHIPPED_MODEL_COUNT);
        assert_eq!(validate_catalog(descriptors), Vec::<String>::new());
    }
}
