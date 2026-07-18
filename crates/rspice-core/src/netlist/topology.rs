//! Topology-level netlist reductions.
//!
//! These transformations operate on already-flattened elements before circuit
//! construction. They preserve simulator-level device invariants by removing
//! ideal topological shorts before individual device builders validate values.

use super::{Element, ElementKind, XspiceDigitalNode, XspicePort};
use crate::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Xyce 7.10 `DeviceOptions::zeroResistanceTol` default.
pub const XYCE_DEFAULT_ZERO_RESISTANCE_TOL: Value = 1.0e-100;

/// Result of topology preprocessing.
#[derive(Debug, Clone)]
pub struct TopologyReduction {
    pub elements: Vec<Element>,
}

/// Xyce-compatible connectivity warnings computed from a flattened circuit.
///
/// Node identity is case-insensitive, while each diagnostic retains the first
/// source spelling so user-facing messages point back to the deck naturally.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectivityDiagnostics {
    /// Non-ground nodes adjacent to no more than one distinct device.
    pub one_device_terminal_nodes: Vec<String>,
    /// Non-ground nodes whose device lead groups do not reach ground at DC.
    pub no_dc_path_nodes: Vec<String>,
}

/// A flattened element whose Xyce DC lead grouping is not modeled yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectivityAnalysisError {
    pub element: String,
    pub reason: String,
}

impl std::fmt::Display for ConnectivityAnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "cannot analyze connectivity for element '{}': {}",
            self.element, self.reason
        )
    }
}

impl std::error::Error for ConnectivityAnalysisError {}

/// Compute Xyce-style one-device and no-DC-path topology diagnostics.
///
/// Xyce assigns every external device terminal a lead-group number. Terminals
/// in the same nonzero group are considered DC-connected; capacitors put each
/// lead in a separate group. This routine mirrors those maps for native SPICE
/// elements and fails closed for code models whose connectivity is model
/// specific.
pub fn analyze_xyce_connectivity(
    elements: &[Element],
) -> Result<ConnectivityDiagnostics, ConnectivityAnalysisError> {
    let mut union = NodeUnion::default();
    let mut attachments: BTreeMap<String, (String, BTreeSet<String>)> = BTreeMap::new();

    for element in elements {
        union.collect_element_nodes(element);
        let mut seen_on_device = BTreeSet::new();
        for node in connectivity_terminal_nodes(element)? {
            let key = node_key(node);
            if seen_on_device.insert(key.clone()) {
                attachments
                    .entry(key)
                    .or_insert_with(|| (normalize_node_name(node), BTreeSet::new()))
                    .1
                    .insert(element.name.to_ascii_uppercase());
            }
        }
        for group in xyce_dc_lead_groups(element)? {
            if let Some((first, rest)) = group.split_first() {
                for node in rest {
                    union.union_nodes(first, node);
                }
            }
        }
    }

    let one_device_terminal_nodes = attachments
        .values()
        .filter(|(node, devices)| !is_ground_name(node) && devices.len() <= 1)
        .map(|(node, _)| node.clone())
        .collect();

    let ground_root = union
        .index_by_key
        .get(&node_key("0"))
        .map(|index| union.root_of(*index));
    let mut no_dc_path_nodes = union
        .entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| !is_ground_name(&entry.original))
        .filter(|(index, _)| ground_root.is_none_or(|ground| union.root_of(*index) != ground))
        .map(|(_, entry)| entry.original.clone())
        .collect::<Vec<_>>();
    no_dc_path_nodes.sort_by_key(|node| node_key(node));

    Ok(ConnectivityDiagnostics {
        one_device_terminal_nodes,
        no_dc_path_nodes,
    })
}

fn connectivity_terminal_nodes(element: &Element) -> Result<Vec<&str>, ConnectivityAnalysisError> {
    let mut nodes = element.nodes.iter().map(String::as_str).collect::<Vec<_>>();
    match &element.kind {
        ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
            nodes.push(&control_nodes.0);
            nodes.push(&control_nodes.1);
        }
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            ..
        } => {
            nodes.push(control_pos);
            nodes.push(control_neg);
        }
        ElementKind::Xspice { .. } => {
            return Err(ConnectivityAnalysisError {
                element: element.name.clone(),
                reason: "XSPICE lead groups are model-specific".to_string(),
            });
        }
        _ => {}
    }
    Ok(nodes)
}

fn xyce_dc_lead_groups(element: &Element) -> Result<Vec<Vec<&str>>, ConnectivityAnalysisError> {
    let all = || vec![element.nodes.iter().map(String::as_str).collect::<Vec<_>>()];
    let separate = || {
        element
            .nodes
            .iter()
            .map(|node| vec![node.as_str()])
            .collect::<Vec<_>>()
    };
    let groups = match &element.kind {
        ElementKind::Capacitor { .. } => separate(),
        ElementKind::Bjt { .. } if element.nodes.len() >= 3 => {
            let mut groups = vec![vec![
                element.nodes[0].as_str(),
                element.nodes[1].as_str(),
                element.nodes[2].as_str(),
            ]];
            groups.extend(element.nodes[3..].iter().map(|node| vec![node.as_str()]));
            groups
        }
        ElementKind::Mosfet { .. } if element.nodes.len() >= 4 => vec![
            vec![element.nodes[0].as_str(), element.nodes[2].as_str()],
            vec![element.nodes[1].as_str()],
            vec![element.nodes[3].as_str()],
        ],
        ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } if element.nodes.len() >= 3 => {
            vec![
                vec![element.nodes[0].as_str(), element.nodes[2].as_str()],
                vec![element.nodes[1].as_str()],
            ]
        }
        ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
            let mut groups = all();
            groups.push(vec![control_nodes.0.as_str(), control_nodes.1.as_str()]);
            groups
        }
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            ..
        } => {
            let mut groups = all();
            groups.push(vec![control_pos.as_str(), control_neg.as_str()]);
            groups
        }
        ElementKind::TransmissionLine { .. } if element.nodes.len() >= 4 => vec![
            vec![element.nodes[0].as_str(), element.nodes[1].as_str()],
            vec![element.nodes[2].as_str(), element.nodes[3].as_str()],
        ],
        ElementKind::Coupling { .. } => Vec::new(),
        ElementKind::Subcircuit { .. }
        | ElementKind::VoltageSourceDeferred(_)
        | ElementKind::CurrentSourceDeferred(_) => {
            return Err(ConnectivityAnalysisError {
                element: element.name.clone(),
                reason: "element must be resolved during flattening".to_string(),
            });
        }
        ElementKind::Xspice { .. } => {
            return Err(ConnectivityAnalysisError {
                element: element.name.clone(),
                reason: "XSPICE lead groups are model-specific".to_string(),
            });
        }
        ElementKind::Resistor { .. }
        | ElementKind::Inductor { .. }
        | ElementKind::JilesAthertonInductor { .. }
        | ElementKind::VoltageSource(_)
        | ElementKind::CurrentSource(_)
        | ElementKind::Diode { .. }
        | ElementKind::XyceMemristor { .. }
        | ElementKind::Cccs { .. }
        | ElementKind::Ccvs { .. }
        | ElementKind::BehavioralVoltage { .. }
        | ElementKind::BehavioralCurrent { .. }
        | ElementKind::ISwitch { .. }
        | ElementKind::GenericSwitch { .. } => all(),
        ElementKind::Bjt { .. }
        | ElementKind::Mosfet { .. }
        | ElementKind::Jfet { .. }
        | ElementKind::Mesfet { .. }
        | ElementKind::TransmissionLine { .. } => {
            return Err(ConnectivityAnalysisError {
                element: element.name.clone(),
                reason: format!(
                    "unexpected terminal count {} for native device",
                    element.nodes.len()
                ),
            });
        }
    };
    Ok(groups)
}

#[derive(Debug, Clone)]
struct NodeEntry {
    parent: usize,
    rank: u8,
    original: String,
    representative: String,
}

/// Collapse nodes connected by explicit zero/near-zero resistors, matching
/// Xyce's opt-in supernode topology reduction.
pub fn reduce_supernode_topology(
    elements: Vec<Element>,
    zero_resistance_tol: Value,
) -> TopologyReduction {
    let tolerance = zero_resistance_tol.max(0.0);
    let mut union = NodeUnion::default();
    let mut has_short_resistors = false;

    for element in &elements {
        union.collect_element_nodes(element);
    }

    for element in &elements {
        if is_supernode_short_resistor(element, tolerance) {
            has_short_resistors = true;
            if element.nodes.len() >= 2
                && !is_ground_name(&element.nodes[0])
                && !is_ground_name(&element.nodes[1])
            {
                union.union_nodes(&element.nodes[0], &element.nodes[1]);
            }
        }
    }

    if !union.has_collapses() && !has_short_resistors {
        return TopologyReduction { elements };
    }

    union.finalize_representatives();

    let elements = elements
        .into_iter()
        .map(|element| union.remap_element(element))
        .filter(|element| {
            !is_supernode_short_resistor(element, tolerance) && !collapsed_noop_element(element)
        })
        .collect();

    TopologyReduction { elements }
}

#[derive(Debug, Default)]
struct NodeUnion {
    entries: Vec<NodeEntry>,
    index_by_key: HashMap<String, usize>,
    collapsed: bool,
}

impl NodeUnion {
    fn collect_element_nodes(&mut self, element: &Element) {
        for node in &element.nodes {
            self.ensure_node(node);
        }

        match &element.kind {
            ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
                self.ensure_node(&control_nodes.0);
                self.ensure_node(&control_nodes.1);
            }
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                ..
            } => {
                self.ensure_node(control_pos);
                self.ensure_node(control_neg);
            }
            ElementKind::Xspice { ports, .. } => {
                for port in ports {
                    self.collect_xspice_port_nodes(port);
                }
            }
            _ => {}
        }
    }

    fn collect_xspice_port_nodes(&mut self, port: &XspicePort) {
        match port {
            XspicePort::Analog(name)
            | XspicePort::Digital(name)
            | XspicePort::ExplicitDigital(name)
            | XspicePort::DigitalInverted(name)
            | XspicePort::Conductance(name)
            | XspicePort::Current(name)
            | XspicePort::Hybrid(name) => {
                self.ensure_node(name);
            }
            XspicePort::AnalogVector(nodes) | XspicePort::DigitalVector(nodes) => {
                for node in nodes {
                    self.ensure_node(node);
                }
            }
            XspicePort::DigitalVectorMixed(nodes) => {
                for node in nodes {
                    self.ensure_node(&node.name);
                }
            }
            XspicePort::DifferentialVoltage { pos, neg }
            | XspicePort::DifferentialCurrent { pos, neg }
            | XspicePort::DifferentialConductance { pos, neg }
            | XspicePort::DifferentialHybrid { pos, neg } => {
                self.ensure_node(pos);
                self.ensure_node(neg);
            }
            XspicePort::VoltageName(_) | XspicePort::Null => {}
        }
    }

    fn ensure_node(&mut self, name: &str) -> usize {
        let key = node_key(name);
        if let Some(index) = self.index_by_key.get(&key) {
            return *index;
        }

        let index = self.entries.len();
        self.entries.push(NodeEntry {
            parent: index,
            rank: 0,
            original: normalize_node_name(name),
            representative: normalize_node_name(name),
        });
        self.index_by_key.insert(key, index);
        index
    }

    fn find(&mut self, index: usize) -> usize {
        let parent = self.entries[index].parent;
        if parent == index {
            return index;
        }
        let root = self.find(parent);
        self.entries[index].parent = root;
        root
    }

    fn root_of(&self, mut index: usize) -> usize {
        while self.entries[index].parent != index {
            index = self.entries[index].parent;
        }
        index
    }

    fn union_nodes(&mut self, left: &str, right: &str) {
        let left = self.ensure_node(left);
        let right = self.ensure_node(right);
        let mut left_root = self.find(left);
        let mut right_root = self.find(right);
        if left_root == right_root {
            return;
        }

        self.collapsed = true;
        if self.entries[left_root].rank < self.entries[right_root].rank {
            std::mem::swap(&mut left_root, &mut right_root);
        }
        self.entries[right_root].parent = left_root;
        if self.entries[left_root].rank == self.entries[right_root].rank {
            self.entries[left_root].rank += 1;
        }
    }

    fn has_collapses(&self) -> bool {
        self.collapsed
    }

    fn finalize_representatives(&mut self) {
        for index in 0..self.entries.len() {
            self.find(index);
        }

        let mut representatives: HashMap<usize, String> = HashMap::new();
        for index in 0..self.entries.len() {
            let root = self.entries[index].parent;
            let original = self.entries[index].original.clone();
            representatives
                .entry(root)
                .and_modify(|current| {
                    if should_replace_representative(current, &original) {
                        *current = original.clone();
                    }
                })
                .or_insert(original);
        }

        for index in 0..self.entries.len() {
            let root = self.entries[index].parent;
            if let Some(representative) = representatives.get(&root) {
                self.entries[index].representative = representative.clone();
            }
        }
    }

    fn remap_node(&self, name: &str) -> String {
        let key = node_key(name);
        let Some(index) = self.index_by_key.get(&key).copied() else {
            return normalize_node_name(name);
        };
        let root = self.root_of(index);
        self.entries[root].representative.clone()
    }

    fn remap_element(&self, mut element: Element) -> Element {
        for node in &mut element.nodes {
            *node = self.remap_node(node);
        }

        element.kind = match element.kind {
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralVoltage {
                expression: remap_behavioral_voltage_probes(&expression, |node| {
                    self.remap_node(node)
                }),
                tc1,
                tc2,
            },
            ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralCurrent {
                expression: remap_behavioral_voltage_probes(&expression, |node| {
                    self.remap_node(node)
                }),
                tc1,
                tc2,
            },
            ElementKind::Vcvs {
                gain,
                gain_expr,
                control_nodes,
            } => ElementKind::Vcvs {
                gain,
                gain_expr,
                control_nodes: (
                    self.remap_node(&control_nodes.0),
                    self.remap_node(&control_nodes.1),
                ),
            },
            ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                control_nodes: (
                    self.remap_node(&control_nodes.0),
                    self.remap_node(&control_nodes.1),
                ),
            },
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                model,
                initial_state,
            } => ElementKind::VSwitch {
                control_pos: self.remap_node(&control_pos),
                control_neg: self.remap_node(&control_neg),
                model,
                initial_state,
            },
            ElementKind::GenericSwitch {
                model,
                control_expression,
                initial_state,
            } => ElementKind::GenericSwitch {
                model,
                control_expression: remap_behavioral_voltage_probes(&control_expression, |node| {
                    self.remap_node(node)
                }),
                initial_state,
            },
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ports,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            } => ElementKind::Xspice {
                model,
                pspice_u_timing,
                ports: ports
                    .into_iter()
                    .map(|port| self.remap_xspice_port(port))
                    .collect(),
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            },
            other => other,
        };

        element
    }

    fn remap_xspice_port(&self, port: XspicePort) -> XspicePort {
        match port {
            XspicePort::Analog(name) => XspicePort::Analog(self.remap_node(&name)),
            XspicePort::Digital(name) => XspicePort::Digital(self.remap_node(&name)),
            XspicePort::ExplicitDigital(name) => {
                XspicePort::ExplicitDigital(self.remap_node(&name))
            }
            XspicePort::DigitalInverted(name) => {
                XspicePort::DigitalInverted(self.remap_node(&name))
            }
            XspicePort::AnalogVector(nodes) => XspicePort::AnalogVector(
                nodes
                    .into_iter()
                    .map(|node| self.remap_node(&node))
                    .collect(),
            ),
            XspicePort::DigitalVector(nodes) => XspicePort::DigitalVector(
                nodes
                    .into_iter()
                    .map(|node| self.remap_node(&node))
                    .collect(),
            ),
            XspicePort::DigitalVectorMixed(nodes) => XspicePort::DigitalVectorMixed(
                nodes
                    .into_iter()
                    .map(|node| XspiceDigitalNode::new(self.remap_node(&node.name), node.inverted))
                    .collect(),
            ),
            XspicePort::Conductance(name) => XspicePort::Conductance(self.remap_node(&name)),
            XspicePort::Current(name) => XspicePort::Current(self.remap_node(&name)),
            XspicePort::VoltageName(name) => XspicePort::VoltageName(name),
            XspicePort::DifferentialVoltage { pos, neg } => XspicePort::DifferentialVoltage {
                pos: self.remap_node(&pos),
                neg: self.remap_node(&neg),
            },
            XspicePort::DifferentialCurrent { pos, neg } => XspicePort::DifferentialCurrent {
                pos: self.remap_node(&pos),
                neg: self.remap_node(&neg),
            },
            XspicePort::DifferentialConductance { pos, neg } => {
                XspicePort::DifferentialConductance {
                    pos: self.remap_node(&pos),
                    neg: self.remap_node(&neg),
                }
            }
            XspicePort::Hybrid(name) => XspicePort::Hybrid(self.remap_node(&name)),
            XspicePort::DifferentialHybrid { pos, neg } => XspicePort::DifferentialHybrid {
                pos: self.remap_node(&pos),
                neg: self.remap_node(&neg),
            },
            XspicePort::Null => XspicePort::Null,
        }
    }
}

fn is_supernode_short_resistor(element: &Element, tolerance: Value) -> bool {
    let ElementKind::Resistor {
        value,
        value_expr,
        model,
        ..
    } = &element.kind
    else {
        return false;
    };

    model.is_none() && value_expr.is_none() && value.is_finite() && value.abs() <= tolerance
}

fn collapsed_noop_element(element: &Element) -> bool {
    if !first_two_nodes_collapsed(element) {
        return false;
    }

    match &element.kind {
        ElementKind::Resistor { .. } => true,
        ElementKind::Capacitor {
            initial_voltage, ..
        } => initial_voltage.is_none_or(|value| value == 0.0),
        _ => false,
    }
}

fn first_two_nodes_collapsed(element: &Element) -> bool {
    element.nodes.len() >= 2 && node_key(&element.nodes[0]) == node_key(&element.nodes[1])
}

fn normalize_node_name(name: &str) -> String {
    if is_ground_name(name) {
        "0".to_string()
    } else {
        name.to_ascii_uppercase()
    }
}

fn node_key(name: &str) -> String {
    if is_ground_name(name) {
        "0".to_string()
    } else {
        name.to_ascii_uppercase()
    }
}

fn is_ground_name(name: &str) -> bool {
    crate::compat::ground::is_spice_ground_name(name)
}

fn should_replace_representative(current: &str, candidate: &str) -> bool {
    if is_ground_name(current) {
        return false;
    }
    is_ground_name(candidate)
}

fn remap_behavioral_voltage_probes(
    expression: &str,
    remap_node: impl Fn(&str) -> String,
) -> String {
    let chars: Vec<char> = expression.chars().collect();
    let mut out = String::with_capacity(expression.len());
    let mut i = 0usize;

    while i < chars.len() {
        let c = chars[i];
        if is_ident_start(c) {
            let ident_start = i;
            i += 1;
            while i < chars.len() && is_ident_continue(chars[i]) {
                i += 1;
            }
            let ident: String = chars[ident_start..i].iter().collect();

            let mut ws_idx = i;
            while ws_idx < chars.len() && chars[ws_idx].is_whitespace() {
                ws_idx += 1;
            }

            if ident.eq_ignore_ascii_case("V")
                && ws_idx < chars.len()
                && chars[ws_idx] == '('
                && let Some((inner, end_idx)) = extract_parenthesized(&chars, ws_idx)
            {
                out.push_str(&ident);
                out.push('(');
                out.push_str(&remap_voltage_probe_args(&inner, &remap_node));
                out.push(')');
                i = end_idx + 1;
                continue;
            }

            out.push_str(&ident);
            continue;
        }

        out.push(c);
        i += 1;
    }

    out
}

fn remap_voltage_probe_args(inner: &str, remap_node: &impl Fn(&str) -> String) -> String {
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if parts.len() == 1 {
        return remap_probe_node(parts[0], remap_node);
    }
    if parts.len() == 2 {
        let pos = remap_probe_node(parts[0], remap_node);
        let neg = remap_probe_node(parts[1], remap_node);
        return format!("{pos},{neg}");
    }
    inner.to_string()
}

fn remap_probe_node(raw: &str, remap_node: &impl Fn(&str) -> String) -> String {
    if raw.is_empty() || !is_simple_probe_name(raw) {
        return raw.to_string();
    }
    remap_node(raw)
}

fn is_simple_probe_name(raw: &str) -> bool {
    raw.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '#' | ':' | '$'))
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':'
}

fn extract_parenthesized(chars: &[char], lparen_idx: usize) -> Option<(String, usize)> {
    if chars.get(lparen_idx).copied() != Some('(') {
        return None;
    }

    let mut depth = 0usize;
    let mut i = lparen_idx;
    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let inner: String = chars[lparen_idx + 1..i].iter().collect();
                    return Some((inner, i));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::{Netlist, flatten_netlist_with_models};

    #[test]
    fn xyce_connectivity_reports_one_device_and_capacitor_only_global_nodes() {
        let deck = "connectivity diagnostics\n\
                    V1 root 0 1\n\
                    Rbad root BAD_ONE 10\n\
                    X1 root CELL\n\
                    X2 root CELL\n\
                    X3 root CELL\n\
                    X4 root CELL\n\
                    .subckt CELL p\n\
                    Cbad p $G_BAD_PATH 1p\n\
                    .ends\n\
                    .end\n";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let flat = flatten_netlist_with_models(&netlist)
            .expect("deck flattens")
            .elements;

        let diagnostics = analyze_xyce_connectivity(&flat).expect("topology is supported");
        assert_eq!(
            diagnostics.one_device_terminal_nodes,
            vec!["BAD_ONE".to_string()]
        );
        assert_eq!(
            diagnostics.no_dc_path_nodes,
            vec!["$G_BAD_PATH".to_string()]
        );
    }

    #[test]
    fn xyce_connectivity_uses_controlled_source_lead_groups() {
        let deck = "controlled-source lead groups\n\
                    V1 out 0 1\n\
                    G1 out 0 sense ref 1m\n\
                    R1 sense 0 1k\n\
                    C1 ref isolated 1p\n\
                    C2 ref isolated 1p\n\
                    .end\n";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let flat = flatten_netlist_with_models(&netlist)
            .expect("deck flattens")
            .elements;

        let diagnostics = analyze_xyce_connectivity(&flat).expect("topology is supported");
        assert!(
            !diagnostics
                .no_dc_path_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case("ref"))
        );
        assert!(
            diagnostics
                .no_dc_path_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case("isolated")),
            "diagnostics={diagnostics:?}"
        );
    }

    #[test]
    fn xyce_transmission_line_dc_groups_follow_port_pairs() {
        // Xyce 7.10 TRA devConMap is [1, 1, 2, 2]: the positive and
        // negative terminal of each port share a DC lead group.
        let deck = "transmission-line lead groups\n\
                    V1 a 0 1\n\
                    T1 a b c d Z0=50 TD=1n\n\
                    .end\n";
        let netlist = Netlist::parse(deck).expect("deck parses");
        let flat = flatten_netlist_with_models(&netlist)
            .expect("deck flattens")
            .elements;

        let diagnostics = analyze_xyce_connectivity(&flat).expect("topology is supported");
        assert!(
            !diagnostics
                .no_dc_path_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case("b")),
            "port 1 negative terminal shares the grounded port-1 group"
        );
        assert!(
            diagnostics
                .no_dc_path_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case("c"))
        );
        assert!(
            diagnostics
                .no_dc_path_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case("d"))
        );
    }

    #[test]
    fn supernode_reduction_collapses_zero_resistor_nodes_and_removes_noop_passives() {
        let deck = "supernode reduction\n\
                    .options topology supernode=true\n\
                    V1 a 0 5\n\
                    Rshort a b 0\n\
                    Rremoved a b 500\n\
                    Cremoved b a 1p\n\
                    Rload b 0 1k\n\
                    B1 out 0 V=V(b,a)+V(b)\n\
                    .end\n";
        let netlist = Netlist::parse(deck).expect("deck parses");
        assert_eq!(netlist.options.topology_supernode, Some(true));

        let flat = flatten_netlist_with_models(&netlist)
            .expect("deck flattens")
            .elements;
        let reduced = reduce_supernode_topology(flat, XYCE_DEFAULT_ZERO_RESISTANCE_TOL);

        assert!(reduced.elements.iter().all(|element| {
            !element.name.eq_ignore_ascii_case("Rshort")
                && !element.name.eq_ignore_ascii_case("Rremoved")
        }));
        assert!(
            reduced
                .elements
                .iter()
                .all(|element| !element.name.eq_ignore_ascii_case("Cremoved"))
        );
        let load = reduced
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("Rload"))
            .expect("load remains");
        assert_eq!(
            load.nodes
                .iter()
                .map(|node| node_key(node))
                .collect::<Vec<_>>(),
            vec!["A".to_string(), "0".to_string()]
        );
        let bsource = reduced
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("B1"))
            .expect("behavioral source remains");
        match &bsource.kind {
            ElementKind::BehavioralVoltage { expression, .. } => {
                assert_eq!(
                    expression.replace(' ', "").to_ascii_uppercase(),
                    "V(A,A)+V(A)"
                );
            }
            other => panic!("unexpected kind {other:?}"),
        }
    }

    #[test]
    fn supernode_reduction_uses_configured_zero_resistance_tolerance() {
        let deck = "supernode tolerance\n\
                    .options topology supernode=true\n\
                    .options device zeroresistancetol=1e-10\n\
                    Rshort a 0 1e-12\n\
                    Rkept b 0 1e-9\n\
                    .end\n";
        let netlist = Netlist::parse(deck).expect("deck parses");
        assert_eq!(netlist.options.topology_supernode, Some(true));
        assert_eq!(netlist.options.device_zero_resistance_tol, Some(1.0e-10));

        let flat = flatten_netlist_with_models(&netlist)
            .expect("deck flattens")
            .elements;
        let reduced = reduce_supernode_topology(flat, 1.0e-10);

        assert!(
            reduced
                .elements
                .iter()
                .all(|element| !element.name.eq_ignore_ascii_case("Rshort"))
        );
        assert!(
            reduced
                .elements
                .iter()
                .any(|element| element.name.eq_ignore_ascii_case("Rkept"))
        );
    }
}
