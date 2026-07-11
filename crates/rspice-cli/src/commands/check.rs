//! Check Command - Validate netlist syntax

use crate::cli::{CheckArgs, CliError};
use rspice_core::{Engine, Netlist};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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

    let _external_guard = XspiceCheckExternalRuntimeGuard::install();
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

struct XspiceCheckExternalRuntimeGuard {
    previous_process: Option<Arc<dyn rspice_core::xspice::DigitalProcessRuntimeFactory>>,
    previous_cosim: Option<Arc<dyn rspice_core::xspice::DigitalCosimRuntimeFactory>>,
}

impl XspiceCheckExternalRuntimeGuard {
    fn install() -> Self {
        let previous_process = rspice_core::xspice::set_digital_process_runtime_factory(Some(
            Arc::new(CheckDigitalProcessFactory),
        ));
        let previous_cosim = rspice_core::xspice::set_digital_cosim_runtime_factory(Some(
            Arc::new(CheckDigitalCosimFactory),
        ));
        Self {
            previous_process,
            previous_cosim,
        }
    }
}

impl Drop for XspiceCheckExternalRuntimeGuard {
    fn drop(&mut self) {
        let previous_process = self.previous_process.take();
        let previous_cosim = self.previous_cosim.take();
        let _ = rspice_core::xspice::set_digital_process_runtime_factory(previous_process);
        let _ = rspice_core::xspice::set_digital_cosim_runtime_factory(previous_cosim);
    }
}

struct CheckDigitalProcessFactory;

impl rspice_core::xspice::DigitalProcessRuntimeFactory for CheckDigitalProcessFactory {
    fn start(
        &self,
        _spec: &rspice_core::xspice::DigitalProcessSpec,
    ) -> rspice_core::xspice::CmResult<Box<dyn rspice_core::xspice::DigitalProcessRuntime>> {
        Ok(Box::new(CheckDigitalProcessRuntime))
    }
}

struct CheckDigitalProcessRuntime;

impl rspice_core::xspice::DigitalProcessRuntime for CheckDigitalProcessRuntime {
    fn exchange(
        &mut self,
        _signed_time: rspice_core::Value,
        _input_bytes: &[u8],
        output_bytes: &mut [u8],
    ) -> rspice_core::xspice::CmResult<()> {
        output_bytes.fill(0);
        Ok(())
    }
}

struct CheckDigitalCosimFactory;

impl rspice_core::xspice::DigitalCosimRuntimeFactory for CheckDigitalCosimFactory {
    fn start(
        &self,
        spec: &rspice_core::xspice::DigitalCosimSpec,
    ) -> rspice_core::xspice::CmResult<Box<dyn rspice_core::xspice::DigitalCosimRuntime>> {
        Ok(Box::new(CheckDigitalCosimRuntime {
            output_count: spec.output_count,
            inout_count: spec.inout_count,
        }))
    }
}

struct CheckDigitalCosimRuntime {
    output_count: usize,
    inout_count: usize,
}

impl CheckDigitalCosimRuntime {
    fn step_result(&self, time: rspice_core::Value) -> rspice_core::xspice::DigitalCosimStep {
        rspice_core::xspice::DigitalCosimStep {
            vtime: time,
            outputs: vec![rspice_core::xspice::DigitalValue::default(); self.output_count],
            inouts: vec![rspice_core::xspice::DigitalValue::default(); self.inout_count],
        }
    }
}

impl rspice_core::xspice::DigitalCosimRuntime for CheckDigitalCosimRuntime {
    fn initialize(
        &mut self,
        time: rspice_core::Value,
        _inputs: &[rspice_core::xspice::DigitalValue],
        _inouts: &[rspice_core::xspice::DigitalValue],
    ) -> rspice_core::xspice::CmResult<rspice_core::xspice::DigitalCosimStep> {
        Ok(self.step_result(time))
    }

    fn step(
        &mut self,
        time: rspice_core::Value,
        _inputs: &[rspice_core::xspice::DigitalValue],
        _inouts: &[rspice_core::xspice::DigitalValue],
        _events: &[rspice_core::xspice::DigitalCosimInputEvent],
    ) -> rspice_core::xspice::CmResult<rspice_core::xspice::DigitalCosimStep> {
        Ok(self.step_result(time))
    }
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
    if let Ok(flattened) = rspice_core::netlist::flatten_netlist_with_models(netlist)
        && let Ok(diagnostics) =
            rspice_core::netlist::analyze_xyce_connectivity(&flattened.elements)
    {
        for node in diagnostics.one_device_terminal_nodes {
            result.warnings.push(ValidationIssue {
                message: format!("Voltage Node ({node}) connected to only 1 device Terminal"),
                element: None,
                line: None,
                code: Some("TOPOLOGY_ONE_DEVICE_TERMINAL".to_string()),
            });
        }
        for node in diagnostics.no_dc_path_nodes {
            result.warnings.push(ValidationIssue {
                message: format!("Voltage Node ({node}) does not have a DC path to ground"),
                element: None,
                line: None,
                code: Some("TOPOLOGY_NO_DC_PATH".to_string()),
            });
        }
        return;
    }

    // Model-specific XSPICE lead groups are not available to the core
    // analyzer. Preserve the older top-level adjacency check as a conservative
    // fallback instead of silently omitting connectivity feedback.
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
