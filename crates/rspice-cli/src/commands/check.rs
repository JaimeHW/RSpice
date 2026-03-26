//! Check Command - Validate netlist syntax

use crate::cli::{CheckArgs, CliError};
use rspice_core::Netlist;
use std::collections::{HashMap, HashSet};

/// Validation result
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

#[derive(Debug)]
pub struct ValidationIssue {
    pub message: String,
    #[allow(dead_code)] // Reserved for line-specific error reporting
    pub line: Option<usize>,
    pub element: Option<String>,
}

impl ValidationResult {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    #[allow(dead_code)] // Reserved for future validation warnings
    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.warnings.push(ValidationIssue {
            message: message.into(),
            line: None,
            element: None,
        });
    }

    pub fn add_element_warning(&mut self, element: impl Into<String>, message: impl Into<String>) {
        self.warnings.push(ValidationIssue {
            message: message.into(),
            line: None,
            element: Some(element.into()),
        });
    }
}

/// Execute the check command
pub fn execute(args: CheckArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    if !quiet {
        println!("Checking: {}", args.input.display());
    }

    let netlist = match Netlist::parse_file(&args.input) {
        Ok(n) => n,
        Err(e) => {
            if args.json {
                println!(
                    "{}",
                    serde_json::json!({"valid": false, "errors": [{"message": e.to_string()}]})
                );
            } else {
                println!("✗ Syntax error: {}", e);
            }
            return Err(CliError::parse_error(e.to_string()));
        }
    };

    let mut result = ValidationResult::default();

    if args.connectivity {
        check_connectivity(&netlist, &mut result);
    }

    if args.models {
        check_model_references(&netlist, &mut result);
    }

    if args.json {
        output_json(&result);
    } else {
        output_text(&result, quiet);
    }

    if args.strict && !result.warnings.is_empty() {
        return Err(CliError::InvalidArgument {
            message: format!("{} warning(s) in strict mode", result.warnings.len()),
            suggestion: None,
        });
    }

    if result.is_ok() {
        Ok(())
    } else {
        Err(CliError::parse_error(format!(
            "{} error(s)",
            result.errors.len()
        )))
    }
}

fn check_connectivity(netlist: &Netlist, result: &mut ValidationResult) {
    let mut node_connections: HashMap<String, usize> = HashMap::new();
    let mut node_elements: HashMap<String, Vec<String>> = HashMap::new();

    // Element.nodes contains the node connections directly
    for elem in &netlist.elements {
        for node in &elem.nodes {
            if node != "0" && node.to_lowercase() != "gnd" {
                *node_connections.entry(node.clone()).or_insert(0) += 1;
                node_elements
                    .entry(node.clone())
                    .or_default()
                    .push(elem.name.clone());
            }
        }
    }

    for (node, count) in &node_connections {
        if *count == 1 {
            let elements = node_elements.get(node).unwrap();
            result.add_element_warning(
                elements.first().unwrap_or(&String::new()),
                format!("Node '{}' has only one connection", node),
            );
        }
    }
}

fn check_model_references(netlist: &Netlist, result: &mut ValidationResult) {
    let defined: HashSet<String> = netlist.models.iter().map(|m| m.name.clone()).collect();

    for elem in &netlist.elements {
        if let Some(model_name) = get_element_model(elem)
            && !defined.contains(&model_name)
            && !is_builtin_model(&model_name)
        {
            result.add_element_warning(&elem.name, format!("Undefined model '{}'", model_name));
        }
    }
}

fn get_element_model(elem: &rspice_core::netlist::Element) -> Option<String> {
    use rspice_core::netlist::ElementKind;
    match &elem.kind {
        ElementKind::Diode { model } => Some(model.clone()),
        ElementKind::Bjt { model, .. } => Some(model.clone()),
        ElementKind::Mosfet { model, .. } => Some(model.clone()),
        ElementKind::Jfet { model, .. } => Some(model.clone()),
        ElementKind::Mesfet { model, .. } => Some(model.clone()),
        _ => None,
    }
}

fn is_builtin_model(name: &str) -> bool {
    matches!(
        name.to_uppercase().as_str(),
        "NMOS" | "PMOS" | "NPN" | "PNP" | "D" | "1N4148" | "2N2222" | "2N3904" | "2N3906"
    )
}

fn output_json(result: &ValidationResult) {
    let json = serde_json::json!({
        "valid": result.is_ok() && result.warnings.is_empty(),
        "errors": result.errors.iter().map(|e| serde_json::json!({"message": e.message})).collect::<Vec<_>>(),
        "warnings": result.warnings.iter().map(|w| serde_json::json!({"message": w.message, "element": w.element})).collect::<Vec<_>>(),
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn output_text(result: &ValidationResult, quiet: bool) {
    for error in &result.errors {
        println!("✗ Error: {}", error.message);
    }
    for warning in &result.warnings {
        if let Some(ref elem) = warning.element {
            println!("⚠ Warning [{}]: {}", elem, warning.message);
        } else {
            println!("⚠ Warning: {}", warning.message);
        }
    }
    if !quiet && result.is_ok() && result.warnings.is_empty() {
        println!("✓ Netlist is valid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::default();
        assert!(result.is_ok());
        result.add_warning("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_builtin_models() {
        assert!(is_builtin_model("NMOS"));
        assert!(!is_builtin_model("CUSTOM"));
    }
}
