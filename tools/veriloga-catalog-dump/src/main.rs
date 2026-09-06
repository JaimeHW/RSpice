//! Validate and emit the generated Verilog-A catalog compiled into a release.

use std::collections::HashSet;
use std::process::ExitCode;

use rspice_core::device::veriloga_builtins::{
    GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION, GeneratedVerilogAModelDescriptor,
    generated_veriloga_model_descriptor, generated_veriloga_model_descriptors,
};

const EXPECTED_SHIPPED_MODEL_COUNT: usize = 43;

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
    errors.extend(validate_release_documentation(descriptors));

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

/// Nouns the README counts when it says how large the catalog is.
const COUNTED_NOUNS: &[&str] = &["model", "models", "device", "devices"];

/// How many words may sit between the number and the noun it counts.
///
/// Three covers the longest phrasing the README uses today, "43 generated
/// Verilog-A models"; the fourth is slack for a rewrite.
const COUNTED_NOUN_LOOKAHEAD: usize = 4;

/// Every number the README states as a count of shipped generated models.
///
/// This scans for counted-noun phrases rather than for fixed sentences. The
/// audit's job is to catch a *stale number*, not to dictate how the release
/// documentation is worded: an earlier version of this check pinned five
/// exact sentence fragments, and a routine README rewrite that kept every
/// count correct failed the release gate for weeks.
fn documented_model_counts(readme: &str) -> Vec<usize> {
    fn bare(word: &str) -> &str {
        word.trim_matches(|character: char| !character.is_alphanumeric() && character != '-')
    }

    let words: Vec<&str> = readme.split_whitespace().collect();
    let mut counts = Vec::new();
    for (index, word) in words.iter().enumerate() {
        let Ok(number) = bare(word).parse::<usize>() else {
            continue;
        };
        for following in words.iter().skip(index + 1).take(COUNTED_NOUN_LOOKAHEAD) {
            let candidate = bare(following).to_ascii_lowercase();
            if COUNTED_NOUNS.contains(&candidate.as_str()) {
                counts.push(number);
                break;
            }
            // A count and its noun never straddle a sentence boundary, so
            // "runs 4 analyses. Every device ..." is not a claim.
            if following.ends_with('.') {
                break;
            }
        }
    }
    counts
}

fn validate_release_documentation(descriptors: &[GeneratedVerilogAModelDescriptor]) -> Vec<String> {
    const README: &str = include_str!("../../../README.md");

    let count = descriptors.len();
    let documented = documented_model_counts(README);

    if documented.is_empty() {
        return vec![format!(
            "README.md states no generated-model count; a release ships {count} and the \
             documentation must say so"
        )];
    }

    documented
        .iter()
        .filter(|documented| **documented != count)
        .map(|stale| {
            format!("README.md states a catalog of {stale} models; this build ships {count}")
        })
        .collect()
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

    #[test]
    fn the_published_readme_states_the_shipped_count_and_nothing_else() {
        assert_eq!(
            validate_release_documentation(generated_veriloga_model_descriptors()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn counted_nouns_are_recognized_however_the_claim_is_phrased() {
        assert_eq!(documented_model_counts("43 models"), vec![43]);
        assert_eq!(documented_model_counts("43 CMC models generated"), vec![43]);
        assert_eq!(
            documented_model_counts("and 43 generated Verilog-A models."),
            vec![43]
        );
        assert_eq!(documented_model_counts("The 43 devices today"), vec![43]);
        assert_eq!(documented_model_counts("| `43` | Device count |"), vec![43]);
    }

    #[test]
    fn a_number_that_counts_something_else_is_not_a_catalog_claim() {
        assert!(documented_model_counts("9 harmonics by default").is_empty());
        assert!(documented_model_counts("exits 80 on failure").is_empty());
        // A count and its noun never straddle a sentence boundary.
        assert!(documented_model_counts("runs 4 analyses. Every device stamps").is_empty());
    }

    #[test]
    fn a_stale_count_is_reported_against_the_compiled_catalog() {
        // The failure this audit exists for: documentation left behind by a
        // catalog that grew.
        let counts = documented_model_counts("42 generated Verilog-A models");
        assert_eq!(counts, vec![42]);
        assert_ne!(counts[0], EXPECTED_SHIPPED_MODEL_COUNT);
    }
}
