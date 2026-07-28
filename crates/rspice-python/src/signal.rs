//! SPICE output-specification parsing.
//!
//! `.FOUR`, `.PRINT`, `.PLOT`, and `.PROBE` all address circuit quantities
//! with the same probe grammar. This module owns that grammar for the Python
//! bindings so a branch current or a differential node pair is resolved
//! identically wherever a spec string is accepted.
//!
//! Recognized forms (operator case is irrelevant):
//!
//! - `V(out)` — a node voltage referenced to ground
//! - `V(out,ref)` — the differential voltage `V(out) - V(ref)`
//! - `I(V1)` — the branch current through an element
//! - `out` — a bare node name, equivalent to `V(out)`
//!
//! A bare name is accepted because SPICE decks and interactive tooling both
//! use it, and because it keeps the pre-existing node-only accessors working
//! unchanged.
//!
//! Parsing is interpreter-free: it returns a message the binding layer maps
//! onto an exception, so the grammar stays unit-testable without an embedded
//! CPython.

/// One parsed circuit-output specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SignalSpec {
    /// Node voltage, optionally differential against a reference node.
    Voltage {
        node: String,
        reference: Option<String>,
    },
    /// Branch current through a named element.
    Current { element: String },
}

impl SignalSpec {
    /// Canonical SPICE rendering, used in diagnostics so an error names the
    /// probe the caller asked for rather than an internal fragment.
    pub(crate) fn label(&self) -> String {
        match self {
            SignalSpec::Voltage {
                node,
                reference: None,
            } => format!("V({node})"),
            SignalSpec::Voltage {
                node,
                reference: Some(reference),
            } => format!("V({node},{reference})"),
            SignalSpec::Current { element } => format!("I({element})"),
        }
    }
}

/// Split `OP(...)` into its operator and the argument list, if it has that
/// shape. The closing parenthesis must be the final character.
fn split_probe_call(spec: &str) -> Option<(&str, &str)> {
    let open = spec.find('(')?;
    let inner = spec.strip_suffix(')')?;
    let operator = spec[..open].trim();
    // `inner` still carries the operator prefix and the open parenthesis.
    Some((operator, &inner[open + 1..]))
}

/// Parse a SPICE output specification.
///
/// Rejects an empty or malformed spec, and any probe operator this API does
/// not evaluate, rather than silently reinterpreting the text as a node name.
pub(crate) fn parse_signal_spec(spec: &str) -> Result<SignalSpec, String> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        return Err("output specification must not be empty".to_string());
    }

    let Some((operator, arguments)) = split_probe_call(trimmed) else {
        // A bare name is a node voltage.
        if trimmed.contains(['(', ')', ',']) {
            return Err(format!(
                "malformed output specification '{spec}'; expected V(node), V(node,ref), \
                 I(element), or a node name"
            ));
        }
        return Ok(SignalSpec::Voltage {
            node: trimmed.to_string(),
            reference: None,
        });
    };

    let parts: Vec<&str> = arguments.split(',').map(str::trim).collect();
    if parts.iter().any(|part| part.is_empty()) {
        return Err(format!(
            "malformed output specification '{spec}': empty argument"
        ));
    }

    match (operator.to_ascii_lowercase().as_str(), parts.len()) {
        ("v", 1) => Ok(SignalSpec::Voltage {
            node: parts[0].to_string(),
            reference: None,
        }),
        ("v", 2) => Ok(SignalSpec::Voltage {
            node: parts[0].to_string(),
            reference: Some(parts[1].to_string()),
        }),
        ("i", 1) => Ok(SignalSpec::Current {
            element: parts[0].to_string(),
        }),
        ("v", count) => Err(format!(
            "V() takes one node or a node pair, got {count} arguments in '{spec}'"
        )),
        ("i", count) => Err(format!(
            "I() takes one element name, got {count} arguments in '{spec}'"
        )),
        (other, _) => Err(format!(
            "unsupported output operator '{other}' in '{spec}'; expected V or I"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voltage(node: &str, reference: Option<&str>) -> SignalSpec {
        SignalSpec::Voltage {
            node: node.to_string(),
            reference: reference.map(str::to_string),
        }
    }

    #[test]
    fn parses_every_supported_probe_form() {
        assert_eq!(parse_signal_spec("V(out)").unwrap(), voltage("out", None));
        assert_eq!(parse_signal_spec("v(OUT)").unwrap(), voltage("OUT", None));
        assert_eq!(
            parse_signal_spec(" V( out , ref ) ").unwrap(),
            voltage("out", Some("ref"))
        );
        assert_eq!(
            parse_signal_spec("I(V1)").unwrap(),
            SignalSpec::Current {
                element: "V1".to_string()
            }
        );
        assert_eq!(parse_signal_spec("out").unwrap(), voltage("out", None));
    }

    #[test]
    fn labels_round_trip_to_canonical_spice_form() {
        assert_eq!(parse_signal_spec("out").unwrap().label(), "V(out)");
        assert_eq!(parse_signal_spec("V(a,b)").unwrap().label(), "V(a,b)");
        assert_eq!(parse_signal_spec("i(L1)").unwrap().label(), "I(L1)");
    }

    #[test]
    fn rejects_malformed_and_unsupported_specs() {
        for spec in [
            "", "   ", "V()", "V(a,)", "V(a,b,c)", "I(a,b)", "P(R1)", "V(a",
        ] {
            assert!(
                parse_signal_spec(spec).is_err(),
                "expected '{spec}' to be rejected"
            );
        }
    }
}
