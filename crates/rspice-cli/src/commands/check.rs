//! Check Command - Validate netlist syntax

use crate::cli::{CheckArgs, CliError};
use rspice_core::{Engine, Netlist};
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
    pub element: Option<String>,
    pub line: Option<usize>,
    pub code: Option<String>,
}

impl ValidationResult {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_element_warning(&mut self, element: impl Into<String>, message: impl Into<String>) {
        self.warnings.push(ValidationIssue {
            message: message.into(),
            element: Some(element.into()),
            line: None,
            code: None,
        });
    }
}

/// Execute the check command
pub fn execute(args: CheckArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    if !quiet {
        println!("Checking: {}", args.input.display());
    }

    let netlist = match crate::commands::parse_netlist_input(&args.input) {
        Ok(n) => n,
        Err(e @ CliError::InputNotFound { .. } | e @ CliError::InputReadError { .. }) => {
            return Err(e);
        }
        Err(e) => {
            if args.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "valid": false,
                        "strict_valid": false,
                        "errors": [{"message": e.to_string()}],
                        "warnings": [],
                    })
                );
            } else {
                println!("✗ Syntax error: {}", e);
            }
            return Err(CliError::parse_error(e.to_string()));
        }
    };

    let mut result = ValidationResult::default();
    add_parser_diagnostics(&netlist, &mut result);

    // Always-on topology checks: these decks produce singular systems, so
    // catching them statically beats a NaN at runtime.
    check_topology(&netlist, &mut result);
    check_xspice_build(&netlist, &mut result);

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

fn check_xspice_build(netlist: &Netlist, result: &mut ValidationResult) {
    if !netlist_contains_xspice(netlist) {
        return;
    }

    if let Err(error) = Engine::default().build_circuit(netlist) {
        result.errors.push(ValidationIssue {
            message: format!("XSPICE build validation failed: {error}"),
            element: None,
            line: None,
            code: None,
        });
    }
}

fn netlist_contains_xspice(netlist: &Netlist) -> bool {
    fn elements_contain_xspice(elements: &[rspice_core::netlist::Element]) -> bool {
        elements.iter().any(|element| {
            matches!(
                element.kind,
                rspice_core::netlist::ElementKind::Xspice { .. }
            )
        })
    }

    elements_contain_xspice(&netlist.elements)
        || netlist
            .subcircuits
            .iter()
            .any(|subckt| elements_contain_xspice(&subckt.elements))
}

/// Detect circuit topologies that make the MNA matrix singular:
/// loops of ideal voltage sources (and DC-shorted inductors), and nodes
/// whose only connections are current sources.
fn check_topology(netlist: &Netlist, result: &mut ValidationResult) {
    use rspice_core::netlist::ElementKind;

    let canonical = |node: &str| -> String {
        let lower = node.to_ascii_lowercase();
        if lower == "gnd" {
            "0".to_string()
        } else {
            lower
        }
    };

    // Union-find over voltage-source/inductor edges: an edge that joins two
    // already-connected nodes closes a loop the DC matrix cannot solve.
    let mut parent: HashMap<String, String> = HashMap::new();
    fn find(parent: &mut HashMap<String, String>, node: &str) -> String {
        let mut current = node.to_string();
        loop {
            let up = parent
                .entry(current.clone())
                .or_insert_with(|| current.clone())
                .clone();
            if up == current {
                return current;
            }
            let grand = parent.get(&up).cloned().unwrap_or_else(|| up.clone());
            parent.insert(current, grand);
            current = up;
        }
    }

    for elem in &netlist.elements {
        let is_vsrc_edge = matches!(
            elem.kind,
            ElementKind::VoltageSource(_) | ElementKind::Inductor { .. }
        );
        if !is_vsrc_edge || elem.nodes.len() < 2 {
            continue;
        }
        let a = find(&mut parent, &canonical(&elem.nodes[0]));
        let b = find(&mut parent, &canonical(&elem.nodes[1]));
        if a == b {
            result.errors.push(ValidationIssue {
                message: format!(
                    "'{}' closes a loop of voltage sources/inductors — the DC \
                     system is singular",
                    elem.name
                ),
                element: Some(elem.name.clone()),
                line: None,
                code: None,
            });
        } else {
            parent.insert(a, b);
        }
    }

    // A node touched only by current sources has no element defining its
    // voltage; KCL there may even be unsatisfiable.
    let mut only_current: HashMap<String, bool> = HashMap::new();
    for elem in &netlist.elements {
        let is_current_source = matches!(elem.kind, ElementKind::CurrentSource(_));
        for node in &elem.nodes {
            let node = canonical(node);
            if node == "0" {
                continue;
            }
            only_current
                .entry(node)
                .and_modify(|flag| *flag &= is_current_source)
                .or_insert(is_current_source);
        }
    }
    let mut current_only_nodes: Vec<&String> = only_current
        .iter()
        .filter(|(_, only)| **only)
        .map(|(node, _)| node)
        .collect();
    current_only_nodes.sort();
    for node in current_only_nodes {
        result.warnings.push(ValidationIssue {
            message: format!(
                "node '{}' connects only to current sources; its voltage is \
                 undefined without a conductive path",
                node
            ),
            element: None,
            line: None,
            code: None,
        });
    }
}

fn add_parser_diagnostics(netlist: &Netlist, result: &mut ValidationResult) {
    for diagnostic in &netlist.diagnostics {
        match diagnostic.severity {
            rspice_core::netlist::DiagnosticSeverity::Warning => {
                result.warnings.push(ValidationIssue {
                    message: diagnostic.message.clone(),
                    element: None,
                    line: (diagnostic.line != 0).then_some(diagnostic.line),
                    code: Some(diagnostic.code.clone()),
                });
            }
        }
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
        ElementKind::Diode { model, .. } => Some(model.clone()),
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
        "valid": result.is_ok(),
        "strict_valid": result.is_ok() && result.warnings.is_empty(),
        "errors": result.errors.iter().map(|e| serde_json::json!({
            "message": &e.message,
            "element": &e.element,
            "line": e.line,
            "code": &e.code,
        })).collect::<Vec<_>>(),
        "warnings": result.warnings.iter().map(|w| serde_json::json!({
            "message": &w.message,
            "element": &w.element,
            "line": w.line,
            "code": &w.code,
        })).collect::<Vec<_>>(),
    });
    match serde_json::to_string_pretty(&json) {
        Ok(text) => println!("{text}"),
        Err(e) => eprintln!("Error: failed to serialize check report: {e}"),
    }
}

fn output_text(result: &ValidationResult, quiet: bool) {
    for error in &result.errors {
        println!("✗ Error: {}", error.message);
    }
    for warning in &result.warnings {
        if warning.element.is_none()
            && let Some(line) = warning.line
        {
            println!("Warning [line {line}]: {}", warning.message);
            continue;
        }
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
