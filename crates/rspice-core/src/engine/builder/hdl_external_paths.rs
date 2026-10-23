//! Paths into the deck's hierarchy that HDL code names.
//!
//! `$cds_get_analog_value("xdiv.m")` reads a node, and
//! `$cds_get_analog_value("xdiv.vp", 0, "flow")` the branch current of an
//! element that owns one, anywhere in the deck. A path is absolute from the
//! deck root and spelled as `.print` and `.save` spell the flattened names:
//! instance names joined by `.`, with a leading `$root.` accepted and `:` read
//! as `.`. There is no module-relative resolution, so a path means the same
//! object whichever module names it. A port of a subcircuit instance
//! (`xdiv.a`) is the net the instance connects it to.
//!
//! A read is resolved once every node and branch of the deck exists, because
//! a branch's solution index follows the last node.
//!
//! `$analog_node_alias(net, "xdiv.m")` and `$analog_port_alias` in a
//! module's `analog initial` bind a local net to a deck node the same paths
//! name. The net is a hidden terminal of the instance, connected while the
//! instance is built ([`bind_alias_terminals`]), so it reads and loads the
//! deck node as any terminal does.

use std::collections::HashMap;

use crate::circuit::CircuitData;
use crate::engine::SimulationError;
use crate::netlist::{Element, ElementKind, Netlist};

/// One scope of the deck's hierarchy: the root, or a subcircuit instance.
struct Scope<'a> {
    /// Its flattened path; empty at the root.
    path: String,
    /// The elements its definition declares.
    elements: &'a [Element],
    /// Its ports, each with the flattened net the instance connects it to.
    ports: Vec<(&'a str, String)>,
}

impl Scope<'_> {
    /// The flattened name of `local`, a net named inside this scope.
    fn net(&self, netlist: &Netlist, local: &str) -> String {
        if let Some((_, actual)) = self
            .ports
            .iter()
            .find(|(port, _)| port.eq_ignore_ascii_case(local))
        {
            return actual.clone();
        }
        if self.path.is_empty()
            || netlist.ground_policy().is_ground(local)
            || netlist
                .global_nodes
                .iter()
                .any(|global| global.eq_ignore_ascii_case(local))
        {
            local.to_string()
        } else {
            format!("{}.{local}", self.path)
        }
    }

    /// The nets its definition names, ports first, each once, as a refusal
    /// lists them: in lower case, as the deck's names are case-insensitive.
    fn net_names(&self, netlist: &Netlist) -> Vec<String> {
        let mut names: Vec<String> = self
            .ports
            .iter()
            .map(|(port, _)| port.to_ascii_lowercase())
            .collect();
        for node in self.elements.iter().flat_map(|element| &element.nodes) {
            if !netlist.ground_policy().is_ground(node)
                && !names.iter().any(|name| name.eq_ignore_ascii_case(node))
            {
                names.push(node.to_ascii_lowercase());
            }
        }
        names
    }

    fn element(&self, name: &str) -> Option<&Element> {
        self.elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
    }

    fn describe(&self) -> String {
        if self.path.is_empty() {
            "the deck's top level".into()
        } else {
            format!("`{}` exists; it", self.path.to_ascii_lowercase())
        }
    }
}

/// The path as the flattened namespace spells it.
fn normalize(path: &str) -> String {
    let path = path.trim();
    let path = match path.get(..6) {
        Some(prefix) if prefix.eq_ignore_ascii_case("$root.") => &path[6..],
        _ => path,
    };
    path.replace(':', ".")
}

/// Walk `segments` down the instance hierarchy as far as it goes, and return
/// the deepest scope with the segments that remain (never fewer than one).
///
/// An instance name may itself contain `.`: a SPICE leaf of an HDL design is
/// a card at deck scope named `<instance>.<path>`. So each step tries the
/// longest run of segments that names an instance first.
fn deepest_scope<'a>(netlist: &'a Netlist, segments: &[&str]) -> (Scope<'a>, usize) {
    let mut scope = Scope {
        path: String::new(),
        elements: &netlist.elements,
        ports: Vec::new(),
    };
    let mut at = 0;
    'descend: while segments.len() - at > 1 {
        for len in (1..segments.len() - at).rev() {
            let name = segments[at..at + len].join(".");
            let Some(element) = scope.element(&name) else {
                continue;
            };
            let ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
                continue;
            };
            let Some(definition) = netlist
                .subcircuits
                .iter()
                .find(|definition| definition.name.eq_ignore_ascii_case(subckt_name))
            else {
                continue;
            };
            let ports = definition
                .ports
                .iter()
                .zip(&element.nodes)
                .map(|(port, node)| (port.as_str(), scope.net(netlist, node)))
                .collect();
            let path = if scope.path.is_empty() {
                element.name.clone()
            } else {
                format!("{}.{}", scope.path, element.name)
            };
            scope = Scope {
                path,
                elements: &definition.elements,
                ports,
            };
            at += len;
            continue 'descend;
        }
        break;
    }
    (scope, at)
}

/// The circuit node a path names, `0` for ground.
pub(super) fn resolve_node(
    circuit: &CircuitData,
    netlist: &Netlist,
    path: &str,
) -> Result<usize, String> {
    let flat = normalize(path);
    if netlist.ground_policy().is_ground(&flat) {
        return Ok(0);
    }
    if let Some(node) = circuit.get_node_by_name(&flat) {
        return Ok(node);
    }
    let segments: Vec<&str> = flat.split('.').collect();
    let (scope, at) = deepest_scope(netlist, &segments);
    let leaf = segments[at..].join(".");
    if scope
        .ports
        .iter()
        .any(|(port, _)| port.eq_ignore_ascii_case(&leaf))
    {
        let net = scope.net(netlist, &leaf);
        if netlist.ground_policy().is_ground(&net) {
            return Ok(0);
        }
        if let Some(node) = circuit.get_node_by_name(&net) {
            return Ok(node);
        }
    }
    let mut detail = format!("{} has no node `{leaf}`", scope.describe());
    if !scope.path.is_empty() {
        detail.push_str(&format!(
            "; its nodes are {}",
            scope.net_names(netlist).join(", ")
        ));
    }
    if scope.element(&leaf).is_some() {
        detail.push_str(&format!(
            "; `{leaf}` is an element, whose current `\"flow\"` reads"
        ));
    }
    Err(detail)
}

/// The one-based solution index of the branch current a path's element owns.
pub(super) fn resolve_branch(
    circuit: &CircuitData,
    netlist: &Netlist,
    path: &str,
) -> Result<usize, String> {
    let flat = normalize(path);
    let segments: Vec<&str> = flat.split('.').collect();
    let (scope, at) = deepest_scope(netlist, &segments);
    let leaf = segments[at..].join(".");
    let element = scope.element(&leaf);
    let capacitor =
        element.is_some_and(|element| matches!(element.kind, ElementKind::Capacitor { .. }));
    if !capacitor && let Some(branch) = circuit.get_branch_by_name(&flat) {
        return Ok(circuit.get_branch_matrix_index(branch));
    }
    if element.is_some() {
        return Err(format!(
            "`{flat}` owns no branch current: `\"flow\"` reads a voltage source, an inductor or a \
             voltage-controlled or current-controlled voltage source"
        ));
    }
    let mut detail = format!("{} has no element `{leaf}`", scope.describe());
    if !scope.path.is_empty() {
        let names: Vec<String> = scope
            .elements
            .iter()
            .map(|element| element.name.to_ascii_lowercase())
            .collect();
        detail.push_str(&format!("; its elements are {}", names.join(", ")));
    }
    Err(detail)
}

/// Bind every mixed module's reads of deck objects by path, or refuse the
/// deck naming the call, the path and the nearest scope that exists.
pub(super) fn bind_external_probes(
    circuit: &mut CircuitData,
    netlist: &Netlist,
) -> Result<(), SimulationError> {
    for host in 0..circuit.mixed_signal_hosts.len() {
        let requests = circuit.mixed_signal_hosts[host].external_probe_requests();
        for request in requests {
            let resolved = if request.flow {
                resolve_branch(circuit, netlist, &request.path)
            } else {
                resolve_node(circuit, netlist, &request.path)
            };
            let index = resolved.map_err(|detail| {
                SimulationError::Netlist(format!(
                    "instance '{}': `{}` names `{}`, which the deck cannot read: {detail}",
                    circuit.mixed_signal_hosts[host].instance_name(),
                    request.call,
                    request.path
                ))
            })?;
            circuit.mixed_signal_hosts[host].bind_external_probe(request.slot, index);
        }
    }
    Ok(())
}

/// The flattened spelling of the deck node an alias binds, or `None` when its
/// path names none. `spellings` maps each flattened node name, upper-cased,
/// to its spelling.
///
/// `$analog_port_alias` binds only a port of an instance; `$analog_node_alias`
/// any node, a port of an instance included.
fn alias_node(
    netlist: &Netlist,
    spellings: &HashMap<String, &str>,
    alias: &rspice_veriloga::canonical_ir::HirNodeAlias,
) -> Option<String> {
    let lookup = |net: &str| {
        if netlist.ground_policy().is_ground(net) {
            Some("0".to_string())
        } else {
            spellings
                .get(&net.to_ascii_uppercase())
                .map(|spelling| spelling.to_string())
        }
    };
    let flat = normalize(&alias.path);
    let segments: Vec<&str> = flat.split('.').collect();
    let (scope, at) = deepest_scope(netlist, &segments);
    let leaf = segments[at..].join(".");
    let port = !scope.path.is_empty()
        && scope
            .ports
            .iter()
            .any(|(port, _)| port.eq_ignore_ascii_case(&leaf));
    if port {
        return lookup(&scope.net(netlist, &leaf));
    }
    if alias.port {
        return None;
    }
    lookup(&flat)
}

/// Connect a module's alias terminals: each hidden terminal after its
/// declared ports takes the deck node its alias path names.
///
/// `nodes` holds the nodes of the terminals before `first`, the first alias
/// terminal; one node per alias is appended. An alias whose path names
/// nothing keeps a net of the instance's own, `<instance>.<net>` as an
/// internal net is named, and its terminal is returned among the unbound
/// ones, so `$analog_node_alias` returns 0 there; each such alias also
/// returns one warning naming the path.
#[allow(clippy::too_many_arguments)]
pub(super) fn bind_alias_terminals(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    flat_elements: &[Element],
    instance: &str,
    first: usize,
    aliases: &[rspice_veriloga::canonical_ir::HirNodeAlias],
    nodes: &mut Vec<usize>,
) -> (Vec<usize>, Vec<rspice_veriloga::SourceCompileDiagnostic>) {
    let spellings: HashMap<String, &str> = flat_elements
        .iter()
        .flat_map(|element| &element.nodes)
        .map(|node| (node.to_ascii_uppercase(), node.as_str()))
        .collect();
    let mut unbound = Vec::new();
    let mut warnings = Vec::new();
    for (offset, alias) in aliases.iter().enumerate() {
        match alias_node(netlist, &spellings, alias) {
            Some(node) => nodes.push(circuit.get_or_create_node(&node)),
            None => {
                nodes.push(circuit.get_or_create_node(&format!("{instance}.{}", alias.net)));
                unbound.push(first + offset);
                let call = if alias.port {
                    "$analog_port_alias"
                } else {
                    "$analog_node_alias"
                };
                warnings.push(rspice_veriloga::SourceCompileDiagnostic {
                    severity: rspice_veriloga::CompileDiagnosticSeverity::Warning,
                    phase: rspice_veriloga::CompileDiagnosticPhase::Semantic,
                    code: "VA-SEM-ALIAS-UNRESOLVED".into(),
                    message: format!(
                        "instance '{instance}': `{call}({}, \"{}\")` names no {} of the deck, so \
                         `{}` stays a net of the instance's own and the call returns 0",
                        alias.net,
                        alias.path,
                        if alias.port { "instance port" } else { "node" },
                        alias.net
                    ),
                    path: None,
                    byte_start: None,
                    byte_end: None,
                    line: None,
                    column: None,
                });
            }
        }
    }
    (unbound, warnings)
}
