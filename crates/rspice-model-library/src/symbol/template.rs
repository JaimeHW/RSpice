//! Shared grammar for model-bound instance netlist templates.

/// Validate the deliberately small instance-template language used by
/// model-bound symbols. Keeping the grammar here prevents authoring,
/// placement, and netlist generation from accepting different contracts.
pub fn validate_library_netlist_template(template: &str) -> Result<(), String> {
    let template = template.trim();
    if template.is_empty() {
        return Err("template is empty".to_owned());
    }
    if template.len() > 512 {
        return Err("template exceeds the 512-byte limit".to_owned());
    }
    if template.chars().any(char::is_control) {
        return Err("template contains a line break or control character".to_owned());
    }
    let tokens = template.split_ascii_whitespace().collect::<Vec<_>>();
    let Some(reference) = tokens.first().copied() else {
        return Err("template is empty".to_owned());
    };
    let reference_is_bound = reference == "{ref}"
        || reference.strip_suffix("{name}").is_some_and(|prefix| {
            !prefix.is_empty()
                && prefix
                    .chars()
                    .all(|character| character.is_ascii_alphabetic())
        });
    if !reference_is_bound {
        return Err(
            "template must start with {ref} or an ASCII device prefix followed by {name}"
                .to_owned(),
        );
    }
    if !matches!(
        tokens.as_slice(),
        [_, "{nodes}", "{model}"] | [_, "{nodes}", "{model}", "{params}"]
    ) {
        return Err(
            "template must be <reference> {nodes} {model} with optional trailing {params}"
                .to_owned(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_bound_template_grammar_is_single_line_and_positional() {
        assert!(validate_library_netlist_template("M{name} {nodes} {model} {params}").is_ok());
        assert!(validate_library_netlist_template("{ref} {nodes} {model}").is_ok());
        for invalid in [
            ".include {nodes} {model}",
            "M{name} {model} {nodes}",
            "M{name} {nodes} {model}\n.end",
            "M{name} {nodes} {model} fixed=1",
            "M{name} {nodes} {model} {params} {params}",
        ] {
            assert!(
                validate_library_netlist_template(invalid).is_err(),
                "unexpectedly accepted {invalid:?}"
            );
        }
    }
}
