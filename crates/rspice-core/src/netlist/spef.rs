//! SPEF (IEEE 1481) parasitic ingestion for post-layout simulation.
//!
//! A SPEF file describes each net's extracted RLC network: internal
//! subnodes, grounded and coupling capacitances, resistances, inductances,
//! and which instance pin or top-level port sits on which subnode. Ingestion
//! is a *back-annotation* transform on a parsed [`Netlist`]:
//!
//! * every `*I inst:pin` connection rewires that instance's terminal from
//!   the ideal net onto its SPEF subnode (`inst__pin`), so the parasitic
//!   resistance genuinely sits between driver and load;
//! * `*P` port connections keep the original net name, preserving the
//!   deck's external connectivity;
//! * the net's R/L/C elements are appended as ordinary resistors,
//!   inductors, and capacitors (coupling caps included), with SPEF units
//!   honored.
//!
//! Decks reference a SPEF file with `.spef_include "file.spef"`
//! (Spectre's spelling) — `.include file.spef` routes there too. DSPF
//! files need none of this: DSPF is SPICE syntax and parses directly.
//!
//! Physically realizable reduced `*R_NET` pi models are lowered to an
//! equivalent C-R-C network. Every retained single-pole load delay must agree
//! with the far-side `R1*C1` time constant; a general timing macromodel is not
//! silently presented as an exact analog topology.
//! Lumped-capacitance `*R_NET` records and the `*C_NET` lumped-capacitance
//! extension are lowered to a capacitor to ground. Reduced records are kept
//! deliberately strict: unsupported multi-driver or pole/residue timing views
//! fail closed instead of being mistaken for parallel physical networks.

use std::collections::{HashMap, HashSet};

use super::ast::{Element, ElementKind};
use super::{
    Netlist, ParseError, ParseWithAbortError, ensure_parse_not_aborted, finish_non_aborting_parse,
    poll_parse_abort,
};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};

/// One parsed SPEF node reference.
#[derive(Debug, Clone, PartialEq)]
enum NodeRef {
    /// Bare net or port name.
    Net(String),
    /// `net:idx` internal subnode.
    SubNode(String, String),
    /// `inst:pin` instance-pin node.
    Pin(String, String),
}

/// One `*CONN` entry of a `*D_NET`.
#[derive(Debug, Clone)]
struct Conn {
    node: NodeRef,
    /// `*P` (port) vs `*I` (instance pin).
    is_port: bool,
    line: usize,
}

#[derive(Debug, Clone)]
struct Cap {
    a: NodeRef,
    /// `None` for a grounded capacitance.
    b: Option<NodeRef>,
    farads: Value,
    line: usize,
}

#[derive(Debug, Clone)]
struct Res {
    a: NodeRef,
    b: NodeRef,
    ohms: Value,
    line: usize,
}

#[derive(Debug, Clone)]
struct Induc {
    a: NodeRef,
    b: NodeRef,
    henries: Value,
    line: usize,
}

#[derive(Debug, Clone)]
struct DNet {
    name: String,
    line: usize,
    conns: Vec<Conn>,
    caps: Vec<Cap>,
    ress: Vec<Res>,
    inductors: Vec<Induc>,
}

/// One load timing descriptor in an IEEE reduced-net driver view.
#[derive(Debug, Clone)]
struct ReducedLoad {
    pin: NodeRef,
    /// Single-pole Elmore delay in the product of the declared R/C units.
    elmore_seconds: Value,
    line: usize,
}

/// One driver-specific reduced pi model (`C2-R1-C1`).
#[derive(Debug, Clone)]
struct DriverReduction {
    driver: NodeRef,
    cell: String,
    c2_farads: Value,
    r1_ohms: Value,
    c1_farads: Value,
    loads: Vec<ReducedLoad>,
    line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReducedNetKind {
    /// IEEE 1481 `*R_NET` (pi model or lumped capacitance).
    Resistance,
    /// Lumped-capacitance `*C_NET` vendor extension.
    Capacitance,
}

#[derive(Debug, Clone)]
struct ReducedNet {
    kind: ReducedNetKind,
    name: String,
    total_capacitance: Value,
    line: usize,
    drivers: Vec<DriverReduction>,
}

/// Parsed SPEF document.
#[derive(Debug, Default)]
pub struct SpefFile {
    nets: Vec<DNet>,
    reduced_nets: Vec<ReducedNet>,
}

/// Summary of one back-annotation pass.
#[derive(Debug, Default, Clone)]
pub struct SpefReport {
    /// Detailed and reduced net sections applied.
    pub nets: usize,
    /// Instance terminals rewired onto SPEF subnodes.
    pub rewired_pins: usize,
    /// Pin connections that could not be matched to a deck element.
    pub skipped_pins: usize,
    /// Parasitic resistors added.
    pub resistors: usize,
    /// Parasitic capacitors added.
    pub capacitors: usize,
    /// Parasitic inductors added.
    pub inductors: usize,
}

impl SpefFile {
    /// Parse SPEF text.
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_abort(content, &NoAbort))
    }

    /// Parse SPEF text with cooperative cancellation.
    pub fn parse_with_abort(
        content: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Parser::new(content).parse_with_abort(abort)
    }

    /// Back-annotate `netlist` with this file's parasitics.
    pub fn apply(&self, netlist: &mut Netlist) -> SpefReport {
        match self.apply_with_abort(netlist, &NoAbort) {
            Ok(report) => report,
            Err(ParseWithAbortError::Aborted) => {
                unreachable!("NoAbort cannot cancel SPEF back-annotation")
            }
            Err(ParseWithAbortError::Parse(_)) => {
                unreachable!("SPEF back-annotation does not produce parse errors")
            }
        }
    }

    /// Back-annotate `netlist` with cooperative cancellation.
    pub fn apply_with_abort(
        &self,
        netlist: &mut Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<SpefReport, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let mut staged = netlist.clone();
        ensure_parse_not_aborted(abort)?;
        let report = self.apply_in_place_with_abort(&mut staged, abort, false)?;
        ensure_parse_not_aborted(abort)?;
        *netlist = staged;
        Ok(report)
    }

    /// Strict transactional annotation used by filesystem-backed imports.
    /// Every parsed record must resolve and the file must alter the retained
    /// circuit; compatibility callers of [`Self::apply`] keep their historic
    /// best-effort behavior.
    pub(crate) fn apply_path_backed_with_abort(
        &self,
        netlist: &mut Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<SpefReport, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let mut staged = netlist.clone();
        ensure_parse_not_aborted(abort)?;
        let report = self.apply_in_place_with_abort(&mut staged, abort, true)?;
        if report.nets == 0 {
            return Err(spef_annotation_error(
                0,
                "document contains no supported net annotation",
            ));
        }
        if report.resistors + report.capacitors + report.inductors == 0 {
            return Err(spef_annotation_error(
                0,
                "document did not apply any supported R/L/C parasitic",
            ));
        }
        ensure_parse_not_aborted(abort)?;
        *netlist = staged;
        Ok(report)
    }

    fn apply_in_place_with_abort(
        &self,
        netlist: &mut Netlist,
        abort: &dyn AbortSignal,
        strict: bool,
    ) -> Result<SpefReport, ParseWithAbortError> {
        let mut report = SpefReport::default();
        let mut element_index: HashMap<String, usize> = HashMap::new();
        let mut original_nodes: HashSet<String> = HashSet::new();
        for (idx, element) in netlist.elements.iter().enumerate() {
            poll_parse_abort(abort, idx)?;
            element_index.insert(element.name.to_ascii_uppercase(), idx);
            original_nodes.extend(element.nodes.iter().map(|node| node.to_ascii_uppercase()));
        }
        original_nodes.insert("0".to_owned());
        let mut subckt_ports: HashMap<String, Vec<String>> = HashMap::new();
        for (subckt_index, def) in netlist.subcircuits.iter().enumerate() {
            poll_parse_abort(abort, subckt_index)?;
            let mut ports = Vec::with_capacity(def.ports.len());
            for (port_index, port) in def.ports.iter().enumerate() {
                poll_parse_abort(abort, port_index)?;
                ports.push(port.to_ascii_uppercase());
            }
            subckt_ports.insert(def.name.to_ascii_uppercase(), ports);
        }

        let mut new_elements: Vec<Element> = Vec::new();
        let mut parasitic_seq = 0_usize;
        let mut occupied_element_names: HashSet<String> = element_index.keys().cloned().collect();
        let mut occupied_nodes = original_nodes.clone();
        let declared_nets: HashSet<String> = self
            .nets
            .iter()
            .map(|net| net.name.to_ascii_uppercase())
            .chain(
                self.reduced_nets
                    .iter()
                    .map(|net| net.name.to_ascii_uppercase()),
            )
            .collect();
        let declared_pins: HashSet<(String, String)> = self
            .nets
            .iter()
            .flat_map(|net| net.conns.iter())
            .filter_map(|conn| match &conn.node {
                NodeRef::Pin(instance, pin) if !conn.is_port => {
                    Some((instance.to_ascii_uppercase(), pin.to_ascii_uppercase()))
                }
                _ => None,
            })
            .collect();

        for (net_index, net) in self.nets.iter().enumerate() {
            poll_parse_abort(abort, net_index)?;
            report.nets += 1;
            let mut net_parasitics = 0usize;
            let mut net_is_anchored = original_nodes.contains(&net.name.to_ascii_uppercase());

            // Bare references (ports are referenced by name, which already
            // is a deck node) pass through; subnodes and pins get generated
            // names. Everything is uppercased to match the parser's node
            // normalization, so parasitics land on the deck's actual nodes.
            let node_name = |reference: &NodeRef| -> String {
                match reference {
                    NodeRef::Net(name) => name.to_ascii_uppercase(),
                    NodeRef::SubNode(net, idx) => sanitize(&format!("{net}__{idx}")),
                    NodeRef::Pin(inst, pin) => sanitize(&format!("{inst}__{pin}")),
                }
            };

            // Rewire instance pins onto their SPEF subnodes.
            for (conn_index, conn) in net.conns.iter().enumerate() {
                poll_parse_abort(abort, conn_index)?;
                if conn.is_port {
                    if strict {
                        let NodeRef::Net(port) = &conn.node else {
                            return Err(spef_annotation_error(
                                conn.line,
                                format!(
                                    "*P connection on net `{}` is not a top-level node",
                                    net.name
                                ),
                            ));
                        };
                        if !original_nodes.contains(&port.to_ascii_uppercase()) {
                            return Err(spef_annotation_error(
                                conn.line,
                                format!(
                                    "top-level port `{port}` on net `{}` is absent from the deck",
                                    net.name
                                ),
                            ));
                        }
                        net_is_anchored = true;
                    }
                    continue;
                }
                let NodeRef::Pin(inst, pin) = &conn.node else {
                    if strict {
                        return Err(spef_annotation_error(
                            conn.line,
                            format!("*I connection on net `{}` is not an instance pin", net.name),
                        ));
                    }
                    continue;
                };
                match rewire_pin(
                    netlist,
                    &element_index,
                    &subckt_ports,
                    inst,
                    pin,
                    &net.name,
                    &node_name(&conn.node),
                ) {
                    Ok(()) => {
                        report.rewired_pins += 1;
                        net_is_anchored = true;
                    }
                    Err(reason) => {
                        if strict {
                            return Err(spef_annotation_error(
                                conn.line,
                                format!("pin {inst}:{pin} on net `{}`: {reason}", net.name),
                            ));
                        }
                        report.skipped_pins += 1;
                        log::warn!(
                            "SPEF: pin {inst}:{pin} on net {} skipped: {reason}",
                            net.name
                        );
                    }
                }
            }

            for (cap_index, cap) in net.caps.iter().enumerate() {
                poll_parse_abort(abort, cap_index)?;
                if !(cap.farads.is_finite() && cap.farads > 0.0) {
                    if strict {
                        return Err(spef_annotation_error(
                            cap.line,
                            format!(
                                "capacitance on net `{}` must be finite and positive, got {}",
                                net.name, cap.farads
                            ),
                        ));
                    }
                    continue;
                }
                if strict {
                    validate_node_reference(
                        &cap.a,
                        &original_nodes,
                        &declared_nets,
                        &declared_pins,
                        cap.line,
                    )?;
                    if let Some(reference) = &cap.b {
                        validate_node_reference(
                            reference,
                            &original_nodes,
                            &declared_nets,
                            &declared_pins,
                            cap.line,
                        )?;
                    }
                }
                let n1 = node_name(&cap.a);
                let n2 = cap
                    .b
                    .as_ref()
                    .map(&node_name)
                    .unwrap_or_else(|| "0".to_owned());
                let name =
                    next_parasitic_name("CSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
                new_elements.push(Element {
                    name,
                    kind: ElementKind::Capacitor {
                        value: cap.farads,
                        value_expr: None,
                        initial_voltage: None,
                        model: None,
                        instance_params: Vec::new(),
                        deferred_params: Vec::new(),
                    },
                    nodes: vec![n1, n2],
                    provenance: crate::netlist::ElementProvenance::Authored,
                });
                report.capacitors += 1;
                net_parasitics += 1;
            }

            for (res_index, res) in net.ress.iter().enumerate() {
                poll_parse_abort(abort, res_index)?;
                if !(res.ohms.is_finite() && res.ohms > 0.0) {
                    if strict {
                        return Err(spef_annotation_error(
                            res.line,
                            format!(
                                "resistance on net `{}` must be finite and positive, got {}",
                                net.name, res.ohms
                            ),
                        ));
                    }
                    continue;
                }
                if strict {
                    validate_node_reference(
                        &res.a,
                        &original_nodes,
                        &declared_nets,
                        &declared_pins,
                        res.line,
                    )?;
                    validate_node_reference(
                        &res.b,
                        &original_nodes,
                        &declared_nets,
                        &declared_pins,
                        res.line,
                    )?;
                }
                let name =
                    next_parasitic_name("RSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
                new_elements.push(Element {
                    name,
                    kind: ElementKind::Resistor {
                        value: res.ohms,
                        value_expr: None,
                        model: None,
                        instance_params: Vec::new(),
                        deferred_params: Vec::new(),
                    },
                    nodes: vec![node_name(&res.a), node_name(&res.b)],
                    provenance: crate::netlist::ElementProvenance::Authored,
                });
                report.resistors += 1;
                net_parasitics += 1;
            }

            for (inductor_index, inductor) in net.inductors.iter().enumerate() {
                poll_parse_abort(abort, inductor_index)?;
                if !(inductor.henries.is_finite() && inductor.henries > 0.0) {
                    if strict {
                        return Err(spef_annotation_error(
                            inductor.line,
                            format!(
                                "inductance on net `{}` must be finite and positive, got {}",
                                net.name, inductor.henries
                            ),
                        ));
                    }
                    continue;
                }
                if strict {
                    validate_node_reference(
                        &inductor.a,
                        &original_nodes,
                        &declared_nets,
                        &declared_pins,
                        inductor.line,
                    )?;
                    validate_node_reference(
                        &inductor.b,
                        &original_nodes,
                        &declared_nets,
                        &declared_pins,
                        inductor.line,
                    )?;
                }
                let name =
                    next_parasitic_name("LSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
                new_elements.push(Element {
                    name,
                    kind: ElementKind::Inductor {
                        value: inductor.henries,
                        value_expr: None,
                        initial_current: None,
                        model: None,
                        instance_params: Vec::new(),
                        deferred_params: Vec::new(),
                    },
                    nodes: vec![node_name(&inductor.a), node_name(&inductor.b)],
                    provenance: crate::netlist::ElementProvenance::Authored,
                });
                report.inductors += 1;
                net_parasitics += 1;
            }

            if strict && !net_is_anchored {
                return Err(spef_annotation_error(
                    net.line,
                    format!(
                        "net `{}` has no matching deck node, top-level port, or rewired instance pin",
                        net.name
                    ),
                ));
            }
            if strict && net_parasitics == 0 {
                return Err(spef_annotation_error(
                    net.line,
                    format!(
                        "net `{}` did not apply any supported R/L/C parasitic",
                        net.name
                    ),
                ));
            }
        }

        for element in &new_elements {
            occupied_nodes.extend(element.nodes.iter().map(|node| node.to_ascii_uppercase()));
        }

        for (net_index, net) in self.reduced_nets.iter().enumerate() {
            poll_parse_abort(abort, self.nets.len().saturating_add(net_index))?;
            report.nets += 1;
            debug_assert!(
                net.kind == ReducedNetKind::Resistance || net.drivers.is_empty(),
                "*C_NET must remain a lumped-capacitance record"
            );
            let canonical_net = net.name.to_ascii_uppercase();
            if !original_nodes.contains(&canonical_net) {
                if strict {
                    return Err(spef_annotation_error(
                        net.line,
                        format!("reduced net `{}` is absent from the deck", net.name),
                    ));
                }
                continue;
            }

            let Some(driver) = net.drivers.first() else {
                let name =
                    next_parasitic_name("CSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
                new_elements.push(spef_capacitor(
                    name,
                    canonical_net,
                    "0".to_owned(),
                    net.total_capacitance,
                ));
                report.capacitors += 1;
                continue;
            };

            // Parsing rejects multiple reductions: they are alternative
            // driver viewpoints, not physical networks that can be placed in
            // parallel. One driver view lowers to a concrete pi network.
            let NodeRef::Pin(driver_instance, driver_pin) = &driver.driver else {
                if strict {
                    return Err(spef_annotation_error(
                        driver.line,
                        format!(
                            "reduced-net driver on `{}` is not an instance pin",
                            net.name
                        ),
                    ));
                }
                continue;
            };
            if let Err(reason) = validate_reduced_driver(
                netlist,
                &element_index,
                &subckt_ports,
                driver_instance,
                driver_pin,
                &driver.cell,
                &canonical_net,
            ) {
                if strict {
                    return Err(spef_annotation_error(
                        driver.line,
                        format!(
                            "driver {driver_instance}:{driver_pin} on reduced net `{}`: {reason}",
                            net.name
                        ),
                    ));
                }
                continue;
            }

            let far_node = next_reduced_node_name(&net.name, &mut occupied_nodes)?;
            let mut seen_loads = HashSet::new();
            let mut rewired_loads = 0usize;
            for (load_index, load) in driver.loads.iter().enumerate() {
                poll_parse_abort(abort, load_index)?;
                debug_assert!(load.elmore_seconds.is_finite() && load.elmore_seconds > 0.0);
                let NodeRef::Pin(instance, pin) = &load.pin else {
                    if strict {
                        return Err(spef_annotation_error(
                            load.line,
                            format!("load on reduced net `{}` is not an instance pin", net.name),
                        ));
                    }
                    continue;
                };
                let load_key = (instance.to_ascii_uppercase(), pin.to_ascii_uppercase());
                if !seen_loads.insert(load_key) {
                    if strict {
                        return Err(spef_annotation_error(
                            load.line,
                            format!(
                                "duplicate load `{instance}:{pin}` on reduced net `{}`",
                                net.name
                            ),
                        ));
                    }
                    continue;
                }
                if instance.eq_ignore_ascii_case(driver_instance)
                    && pin.eq_ignore_ascii_case(driver_pin)
                {
                    if strict {
                        return Err(spef_annotation_error(
                            load.line,
                            format!(
                                "driver `{driver_instance}:{driver_pin}` is also listed as a load on reduced net `{}`",
                                net.name
                            ),
                        ));
                    }
                    continue;
                }
                match rewire_pin(
                    netlist,
                    &element_index,
                    &subckt_ports,
                    instance,
                    pin,
                    &net.name,
                    &far_node,
                ) {
                    Ok(()) => rewired_loads += 1,
                    Err(reason) if strict => {
                        return Err(spef_annotation_error(
                            load.line,
                            format!(
                                "load {instance}:{pin} on reduced net `{}`: {reason}",
                                net.name
                            ),
                        ));
                    }
                    Err(reason) => {
                        report.skipped_pins += 1;
                        log::warn!(
                            "SPEF: reduced-net load {instance}:{pin} on {} skipped: {reason}",
                            net.name
                        );
                    }
                }
            }
            if rewired_loads == 0 {
                if strict {
                    return Err(spef_annotation_error(
                        driver.line,
                        format!("reduced net `{}` did not resolve any load pins", net.name),
                    ));
                }
                continue;
            }

            let c2_name =
                next_parasitic_name("CSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
            new_elements.push(spef_capacitor(
                c2_name,
                canonical_net.clone(),
                "0".to_owned(),
                driver.c2_farads,
            ));
            let resistor_name =
                next_parasitic_name("RSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
            new_elements.push(spef_resistor(
                resistor_name,
                canonical_net,
                far_node.clone(),
                driver.r1_ohms,
            ));
            let c1_name =
                next_parasitic_name("CSPEF", &mut parasitic_seq, &mut occupied_element_names)?;
            new_elements.push(spef_capacitor(
                c1_name,
                far_node,
                "0".to_owned(),
                driver.c1_farads,
            ));
            report.rewired_pins += rewired_loads;
            report.resistors += 1;
            report.capacitors += 2;
        }

        for (index, element) in new_elements.into_iter().enumerate() {
            poll_parse_abort(abort, index)?;
            netlist.elements.push(element);
        }
        ensure_parse_not_aborted(abort)?;
        Ok(report)
    }
}

fn spef_capacitor(name: String, a: String, b: String, farads: Value) -> Element {
    Element {
        name,
        kind: ElementKind::Capacitor {
            value: farads,
            value_expr: None,
            initial_voltage: None,
            model: None,
            instance_params: Vec::new(),
            deferred_params: Vec::new(),
        },
        nodes: vec![a, b],
        // SPEF is authored circuit input. The deterministic CSPEF/RSPEF name
        // and reduced far-node spelling retain its source identity without
        // misclassifying it as a helper owned by a SPICE element.
        provenance: crate::netlist::ElementProvenance::Authored,
    }
}

fn spef_resistor(name: String, a: String, b: String, ohms: Value) -> Element {
    Element {
        name,
        kind: ElementKind::Resistor {
            value: ohms,
            value_expr: None,
            model: None,
            instance_params: Vec::new(),
            deferred_params: Vec::new(),
        },
        nodes: vec![a, b],
        provenance: crate::netlist::ElementProvenance::Authored,
    }
}

fn next_reduced_node_name(
    net: &str,
    occupied: &mut HashSet<String>,
) -> Result<String, ParseWithAbortError> {
    let base = sanitize(&format!("__SPEF_REDUCED_{net}"));
    let mut suffix = 0usize;
    loop {
        let candidate = if suffix == 0 {
            base.clone()
        } else {
            format!("{base}_{suffix}")
        };
        if occupied.insert(candidate.clone()) {
            return Ok(candidate);
        }
        suffix = suffix.checked_add(1).ok_or_else(|| {
            spef_annotation_error(
                0,
                format!("too many node-name collisions while lowering reduced net `{net}`"),
            )
        })?;
    }
}

fn next_parasitic_name(
    prefix: &str,
    sequence: &mut usize,
    occupied: &mut HashSet<String>,
) -> Result<String, ParseWithAbortError> {
    loop {
        *sequence = sequence.checked_add(1).ok_or_else(|| {
            spef_annotation_error(0, "too many parasitic elements to assign unique names")
        })?;
        let candidate = format!("{prefix}{sequence}");
        if occupied.insert(candidate.clone()) {
            return Ok(candidate);
        }
    }
}

fn spef_annotation_error(line: usize, message: impl Into<String>) -> ParseWithAbortError {
    ParseError::Syntax {
        line,
        message: format!("SPEF annotation: {}", message.into()),
    }
    .into()
}

fn validate_node_reference(
    reference: &NodeRef,
    original_nodes: &HashSet<String>,
    declared_nets: &HashSet<String>,
    declared_pins: &HashSet<(String, String)>,
    line: usize,
) -> Result<(), ParseWithAbortError> {
    match reference {
        NodeRef::Net(name) => {
            if !original_nodes.contains(&name.to_ascii_uppercase()) {
                return Err(spef_annotation_error(
                    line,
                    format!("node `{name}` is absent from the deck"),
                ));
            }
        }
        NodeRef::SubNode(net, index) => {
            if index.is_empty() || !declared_nets.contains(&net.to_ascii_uppercase()) {
                return Err(spef_annotation_error(
                    line,
                    format!("subnode `{net}:{index}` does not belong to a declared *D_NET"),
                ));
            }
        }
        NodeRef::Pin(instance, pin) => {
            let key = (instance.to_ascii_uppercase(), pin.to_ascii_uppercase());
            if !declared_pins.contains(&key) {
                return Err(spef_annotation_error(
                    line,
                    format!("pin node `{instance}:{pin}` has no matching *I connection"),
                ));
            }
        }
    }
    Ok(())
}

/// Move one instance terminal from the ideal net to its SPEF subnode.
fn rewire_pin(
    netlist: &mut Netlist,
    element_index: &HashMap<String, usize>,
    subckt_ports: &HashMap<String, Vec<String>>,
    inst: &str,
    pin: &str,
    net: &str,
    new_node: &str,
) -> Result<(), String> {
    let upper = inst.to_ascii_uppercase();
    let element_idx = element_index
        .get(&upper)
        .or_else(|| element_index.get(&format!("X{upper}")))
        .copied()
        .ok_or_else(|| format!("no element named `{inst}` in the deck"))?;

    let element = &netlist.elements[element_idx];
    let terminal = terminal_index(element, subckt_ports, pin)
        .ok_or_else(|| format!("pin `{pin}` not resolvable on `{}`", element.name))?;

    let element = &mut netlist.elements[element_idx];
    let current = element
        .nodes
        .get(terminal)
        .ok_or_else(|| format!("terminal {terminal} out of range on `{}`", element.name))?;
    if !current.eq_ignore_ascii_case(net) {
        return Err(format!(
            "terminal `{pin}` of `{}` connects to `{current}`, not net `{net}`",
            element.name
        ));
    }
    element.nodes[terminal] = new_node.to_owned();
    Ok(())
}

fn validate_reduced_driver(
    netlist: &mut Netlist,
    element_index: &HashMap<String, usize>,
    subckt_ports: &HashMap<String, Vec<String>>,
    instance: &str,
    pin: &str,
    cell: &str,
    net: &str,
) -> Result<(), String> {
    // Reusing the terminal resolution/ownership check with the same node
    // validates the driver without moving it off the near side of the pi.
    rewire_pin(
        netlist,
        element_index,
        subckt_ports,
        instance,
        pin,
        net,
        net,
    )?;

    let upper = instance.to_ascii_uppercase();
    let element_idx = element_index
        .get(&upper)
        .or_else(|| element_index.get(&format!("X{upper}")))
        .copied()
        .ok_or_else(|| format!("no element named `{instance}` in the deck"))?;
    let element = &netlist.elements[element_idx];
    let expected_cell = match &element.kind {
        ElementKind::Subcircuit { subckt_name, .. } => subckt_name.as_str(),
        // Native independent sources are useful reduced-network drivers in
        // standalone analog decks. Give them one exact pseudo-cell spelling
        // rather than accepting arbitrary *CELL metadata.
        ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => "VOLTAGE_SOURCE",
        ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => "CURRENT_SOURCE",
        _ => {
            return Err(format!(
                "element `{}` cannot be qualified as a reduced-net driver cell",
                element.name
            ));
        }
    };
    if !expected_cell.eq_ignore_ascii_case(cell) {
        return Err(format!(
            "*CELL `{cell}` does not match driver type `{expected_cell}`"
        ));
    }
    Ok(())
}

/// Resolve a SPEF pin name to a terminal index on a deck element.
fn terminal_index(
    element: &Element,
    subckt_ports: &HashMap<String, Vec<String>>,
    pin: &str,
) -> Option<usize> {
    let pin_upper = pin.to_ascii_uppercase();
    match &element.kind {
        ElementKind::Subcircuit { subckt_name, .. } => subckt_ports
            .get(&subckt_name.to_ascii_uppercase())?
            .iter()
            .position(|port| *port == pin_upper),
        ElementKind::Mosfet { .. } => position_in(&["D", "G", "S", "B"], &pin_upper),
        ElementKind::Bjt { .. } => position_in(&["C", "B", "E", "S"], &pin_upper),
        ElementKind::Diode { .. } => {
            position_in(&["A", "K"], &pin_upper).or_else(|| position_in(&["P", "N"], &pin_upper))
        }
        ElementKind::Resistor { .. }
        | ElementKind::Capacitor { .. }
        | ElementKind::Inductor { .. } => position_in(&["1", "2"], &pin_upper)
            .or_else(|| position_in(&["P", "N"], &pin_upper))
            .or_else(|| position_in(&["A", "B"], &pin_upper)),
        ElementKind::VoltageSource(_)
        | ElementKind::VoltageSourceDeferred(_)
        | ElementKind::CurrentSource(_)
        | ElementKind::CurrentSourceDeferred(_) => position_in(&["1", "2"], &pin_upper)
            .or_else(|| position_in(&["P", "N"], &pin_upper))
            .or_else(|| position_in(&["+", "-"], &pin_upper)),
        _ => None,
    }
    .filter(|idx| *idx < element.nodes.len())
}

fn position_in(table: &[&str], pin: &str) -> Option<usize> {
    table.iter().position(|candidate| *candidate == pin)
}

/// SPICE node names may not carry SPEF's `:`/`/`/`[`/`]` characters; the
/// result is uppercased to match the parser's node normalization.
fn sanitize(raw: &str) -> String {
    raw.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

struct Parser<'a> {
    lines: std::str::Lines<'a>,
    line_num: usize,
    name_map: HashMap<u64, String>,
    delimiter: char,
    cap_scale: Value,
    res_scale: Value,
    induc_scale: Value,
}

/// Sections within a `*D_NET`.
#[derive(Clone, Copy, PartialEq)]
enum NetSection {
    None,
    Conn,
    Cap,
    Res,
    Induc,
}

struct ReducedDriverBuilder {
    driver: NodeRef,
    line: usize,
    cell: Option<String>,
    pi: Option<(Value, Value, Value)>,
    loads_started: bool,
    loads: Vec<ReducedLoad>,
}

struct ReducedNetBuilder {
    kind: ReducedNetKind,
    name: String,
    total_capacitance: Value,
    line: usize,
    drivers: Vec<DriverReduction>,
    active_driver: Option<ReducedDriverBuilder>,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Self {
            lines: content.lines(),
            line_num: 0,
            name_map: HashMap::new(),
            delimiter: ':',
            cap_scale: 1e-12, // SPEF default exchange unit is PF / OHM
            res_scale: 1.0,
            induc_scale: 1.0,
        }
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::Syntax {
            line: self.line_num,
            message: format!("SPEF: {}", message.into()),
        }
    }

    fn parse_with_abort(
        mut self,
        abort: &dyn AbortSignal,
    ) -> Result<SpefFile, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let mut nets = Vec::new();
        let mut reduced_nets = Vec::new();
        let mut in_name_map = false;
        let mut current: Option<DNet> = None;
        let mut current_reduced: Option<ReducedNetBuilder> = None;
        let mut declared_net_names = HashSet::new();
        let mut section = NetSection::None;

        while let Some(raw) = self.lines.next() {
            self.line_num += 1;
            poll_parse_abort(abort, self.line_num - 1)?;
            let line = strip_comment_with_abort(raw, abort)?.trim();
            if line.is_empty() {
                continue;
            }
            let mut fields: Vec<&str> = Vec::new();
            for (field_index, field) in line.split_whitespace().enumerate() {
                poll_parse_abort(abort, field_index)?;
                fields.push(field);
            }
            let keyword = fields[0].to_ascii_uppercase();

            // Name-map entries: `*<index> <name>`.
            if in_name_map && let Some(index) = parse_map_index(fields[0]) {
                let name = fields
                    .get(1)
                    .ok_or_else(|| self.error(format!("name-map entry *{index} has no name")))
                    .map_err(ParseWithAbortError::from)?;
                if self.name_map.insert(index, (*name).to_owned()).is_some() {
                    return Err(self
                        .error(format!("duplicate name-map index *{index}"))
                        .into());
                }
                continue;
            }

            if let Some(reduced) = current_reduced.as_mut() {
                match keyword.as_str() {
                    "*END" => {
                        let mut reduced = current_reduced.take().ok_or_else(|| {
                            self.error("internal reduced-net parser state was lost")
                        })?;
                        if let Some(driver) = reduced.active_driver.take() {
                            reduced.drivers.push(
                                self.finish_reduced_driver(driver)
                                    .map_err(ParseWithAbortError::from)?,
                            );
                        }
                        if reduced.kind == ReducedNetKind::Capacitance
                            && !reduced.drivers.is_empty()
                        {
                            return Err(self
                                .error("*C_NET accepts only a lumped capacitance")
                                .into());
                        }
                        if reduced.drivers.len() > 1 {
                            return Err(self
                                .error(format!(
                                    "reduced net `{}` has multiple driver views; they cannot be materialized as parallel physical networks",
                                    reduced.name
                                ))
                                .into());
                        }
                        reduced_nets.push(ReducedNet {
                            kind: reduced.kind,
                            name: reduced.name,
                            total_capacitance: reduced.total_capacitance,
                            line: reduced.line,
                            drivers: reduced.drivers,
                        });
                        section = NetSection::None;
                    }
                    "*DRIVER" if reduced.kind == ReducedNetKind::Resistance => {
                        if fields.len() != 2 {
                            return Err(self
                                .error("malformed *DRIVER record (expected exactly 2 fields)")
                                .into());
                        }
                        if let Some(driver) = reduced.active_driver.take() {
                            reduced.drivers.push(
                                self.finish_reduced_driver(driver)
                                    .map_err(ParseWithAbortError::from)?,
                            );
                        }
                        let driver = self
                            .parse_pin_ref(fields[1])
                            .map_err(ParseWithAbortError::from)?;
                        if !matches!(driver, NodeRef::Pin(_, _)) {
                            return Err(self.error("*DRIVER must name an instance pin").into());
                        }
                        reduced.active_driver = Some(ReducedDriverBuilder {
                            driver,
                            line: self.line_num,
                            cell: None,
                            pi: None,
                            loads_started: false,
                            loads: Vec::new(),
                        });
                    }
                    "*CELL" if reduced.kind == ReducedNetKind::Resistance => {
                        if fields.len() != 2 {
                            return Err(self
                                .error("malformed *CELL record (expected exactly 2 fields)")
                                .into());
                        }
                        let cell = self
                            .resolve_name(fields[1])
                            .map_err(ParseWithAbortError::from)?;
                        if cell.eq_ignore_ascii_case("UNKNOWN_DRIVER") {
                            return Err(self.error("UNKNOWN_DRIVER is invalid in *R_NET").into());
                        }
                        let driver = reduced
                            .active_driver
                            .as_mut()
                            .ok_or_else(|| self.error("*CELL record appears before *DRIVER"))?;
                        if driver.cell.replace(cell).is_some() {
                            return Err(self.error("duplicate *CELL record").into());
                        }
                    }
                    "*C2_R1_C1" if reduced.kind == ReducedNetKind::Resistance => {
                        if fields.len() != 4 {
                            return Err(self
                                .error("malformed *C2_R1_C1 record (expected exactly 4 fields)")
                                .into());
                        }
                        let driver = reduced
                            .active_driver
                            .as_mut()
                            .ok_or_else(|| self.error("*C2_R1_C1 record appears before *DRIVER"))?;
                        if driver.cell.is_none() {
                            return Err(self.error("*C2_R1_C1 record appears before *CELL").into());
                        }
                        if driver.pi.is_some() {
                            return Err(self.error("duplicate *C2_R1_C1 record").into());
                        }
                        let c2 = self
                            .parse_scaled_value(fields[1], self.cap_scale, "C2", false)
                            .map_err(ParseWithAbortError::from)?;
                        let r1 = self
                            .parse_scaled_value(fields[2], self.res_scale, "R1", false)
                            .map_err(ParseWithAbortError::from)?;
                        let c1 = self
                            .parse_scaled_value(fields[3], self.cap_scale, "C1", false)
                            .map_err(ParseWithAbortError::from)?;
                        let sum = c1 + c2;
                        let tolerance = reduced.total_capacitance.max(sum) * 1.0e-4;
                        if !sum.is_finite() || (sum - reduced.total_capacitance).abs() > tolerance {
                            return Err(self
                                .error(format!(
                                    "*C2_R1_C1 capacitance is not conserved: C1+C2={sum:.17e} F, total={:.17e} F",
                                    reduced.total_capacitance
                                ))
                                .into());
                        }
                        driver.pi = Some((c2, r1, c1));
                    }
                    "*LOADS" if reduced.kind == ReducedNetKind::Resistance => {
                        if fields.len() != 1 {
                            return Err(self
                                .error("malformed *LOADS record (expected no arguments)")
                                .into());
                        }
                        let driver = reduced
                            .active_driver
                            .as_mut()
                            .ok_or_else(|| self.error("*LOADS record appears before *DRIVER"))?;
                        if driver.pi.is_none() {
                            return Err(self
                                .error("*LOADS record appears before *C2_R1_C1")
                                .into());
                        }
                        if driver.loads_started {
                            return Err(self.error("duplicate *LOADS record").into());
                        }
                        driver.loads_started = true;
                    }
                    "*RC" if reduced.kind == ReducedNetKind::Resistance => {
                        if fields.len() != 3 {
                            return Err(self
                                .error("malformed *RC record (expected exactly 3 fields)")
                                .into());
                        }
                        let driver = reduced
                            .active_driver
                            .as_mut()
                            .ok_or_else(|| self.error("*RC record appears before *DRIVER"))?;
                        if !driver.loads_started {
                            return Err(self.error("*RC record appears before *LOADS").into());
                        }
                        let pin = self
                            .parse_pin_ref(fields[1])
                            .map_err(ParseWithAbortError::from)?;
                        if !matches!(pin, NodeRef::Pin(_, _)) {
                            return Err(self.error("*RC must name an instance pin").into());
                        }
                        let delay_scale = self.res_scale * self.cap_scale;
                        if !delay_scale.is_finite() || delay_scale <= 0.0 {
                            return Err(self
                                .error("R/C unit product for *RC is invalid or non-finite")
                                .into());
                        }
                        let elmore_seconds = self
                            .parse_scaled_value(fields[2], delay_scale, "*RC Elmore delay", false)
                            .map_err(ParseWithAbortError::from)?;
                        driver.loads.push(ReducedLoad {
                            pin,
                            elmore_seconds,
                            line: self.line_num,
                        });
                    }
                    "*Q" | "*K" => {
                        return Err(self
                            .error(format!(
                                "reduced-net pole/residue record {keyword} is not materializable as a passive R/C network"
                            ))
                            .into());
                    }
                    _ => {
                        return Err(self
                            .error(format!(
                                "unexpected record `{}` inside {}",
                                fields[0],
                                if reduced.kind == ReducedNetKind::Resistance {
                                    "*R_NET"
                                } else {
                                    "*C_NET"
                                }
                            ))
                            .into());
                    }
                }
                continue;
            }

            match keyword.as_str() {
                "*SPEF" | "*DESIGN" | "*DATE" | "*VENDOR" | "*PROGRAM" | "*VERSION"
                | "*DESIGN_FLOW" | "*DIVIDER" | "*BUS_DELIMITER" | "*T_UNIT" => {
                    in_name_map = false;
                }
                "*DELIMITER" => {
                    in_name_map = false;
                    if let Some(d) = fields.get(1).and_then(|f| f.chars().next()) {
                        self.delimiter = d;
                    }
                }
                "*C_UNIT" => {
                    in_name_map = false;
                    self.cap_scale = self
                        .parse_unit(
                            &fields,
                            &[
                                ("FF", 1e-15),
                                ("PF", 1e-12),
                                ("NF", 1e-9),
                                ("UF", 1e-6),
                                ("F", 1.0),
                            ],
                        )
                        .map_err(ParseWithAbortError::from)?;
                }
                "*R_UNIT" => {
                    in_name_map = false;
                    self.res_scale = self
                        .parse_unit(&fields, &[("OHM", 1.0), ("KOHM", 1e3), ("MOHM", 1e6)])
                        .map_err(ParseWithAbortError::from)?;
                }
                "*L_UNIT" => {
                    in_name_map = false;
                    self.induc_scale = self
                        .parse_unit(&fields, &[("HENRY", 1.0), ("MH", 1e-3), ("UH", 1e-6)])
                        .map_err(ParseWithAbortError::from)?;
                }
                "*NAME_MAP" => in_name_map = true,
                "*PORTS" | "*PHYSICAL_PORTS" => in_name_map = false,
                "*D_NET" => {
                    in_name_map = false;
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    if fields.len() != 3 && fields.len() != 5 {
                        return Err(self
                            .error("malformed *D_NET header (expected 3 fields or optional *V confidence)")
                            .into());
                    }
                    self.validate_optional_routing_confidence(&fields)
                        .map_err(ParseWithAbortError::from)?;
                    let name_field = fields
                        .get(1)
                        .ok_or_else(|| self.error("*D_NET without a net name"))
                        .map_err(ParseWithAbortError::from)?;
                    let total_capacitance = fields
                        .get(2)
                        .ok_or_else(|| self.error("*D_NET without a total capacitance"))
                        .map_err(ParseWithAbortError::from)?;
                    self.parse_scaled_value(
                        total_capacitance,
                        self.cap_scale,
                        "*D_NET total capacitance",
                        true,
                    )
                    .map_err(ParseWithAbortError::from)?;
                    let name = self
                        .resolve_name(name_field)
                        .map_err(ParseWithAbortError::from)?;
                    if !declared_net_names.insert(name.to_ascii_uppercase()) {
                        return Err(self
                            .error(format!("duplicate net annotation `{name}`"))
                            .into());
                    }
                    current = Some(DNet {
                        name,
                        line: self.line_num,
                        conns: Vec::new(),
                        caps: Vec::new(),
                        ress: Vec::new(),
                        inductors: Vec::new(),
                    });
                    section = NetSection::None;
                }
                "*R_NET" | "*C_NET" => {
                    in_name_map = false;
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    if fields.len() != 3 && fields.len() != 5 {
                        return Err(self
                            .error(format!(
                                "malformed {keyword} header (expected 3 fields or optional *V confidence)"
                            ))
                            .into());
                    }
                    self.validate_optional_routing_confidence(&fields)
                        .map_err(ParseWithAbortError::from)?;
                    let kind = if keyword == "*R_NET" {
                        ReducedNetKind::Resistance
                    } else {
                        ReducedNetKind::Capacitance
                    };
                    let name = self
                        .resolve_name(fields[1])
                        .map_err(ParseWithAbortError::from)?;
                    if !declared_net_names.insert(name.to_ascii_uppercase()) {
                        return Err(self
                            .error(format!("duplicate net annotation `{name}`"))
                            .into());
                    }
                    let total_capacitance = self
                        .parse_scaled_value(
                            fields[2],
                            self.cap_scale,
                            &format!("{keyword} total capacitance"),
                            false,
                        )
                        .map_err(ParseWithAbortError::from)?;
                    current_reduced = Some(ReducedNetBuilder {
                        kind,
                        name,
                        total_capacitance,
                        line: self.line_num,
                        drivers: Vec::new(),
                        active_driver: None,
                    });
                    section = NetSection::None;
                }
                "*CONN" => section = NetSection::Conn,
                "*CAP" => section = NetSection::Cap,
                "*RES" => section = NetSection::Res,
                "*INDUC" => section = NetSection::Induc,
                "*END" => {
                    if let Some(net) = current.take() {
                        nets.push(net);
                    }
                    section = NetSection::None;
                }
                "*P" | "*I" if section == NetSection::Conn => {
                    let net = current
                        .as_mut()
                        .ok_or_else(|| self.error("connection entry outside *D_NET"))
                        .map_err(ParseWithAbortError::from)?;
                    let node_field = fields
                        .get(1)
                        .ok_or_else(|| self.error("connection entry without a node"))
                        .map_err(ParseWithAbortError::from)?;
                    let node = self
                        .parse_node_ref(node_field)
                        .map_err(ParseWithAbortError::from)?;
                    net.conns.push(Conn {
                        node,
                        is_port: keyword == "*P",
                        line: self.line_num,
                    });
                }
                "*N" if section == NetSection::Conn => {
                    // Internal-node coordinates: topology only, no action.
                }
                _ if section == NetSection::Cap => {
                    let net = current
                        .as_mut()
                        .ok_or_else(|| self.error("*CAP entry outside *D_NET"))
                        .map_err(ParseWithAbortError::from)?;
                    // `id node value` (ground) or `id node node value`.
                    match fields.len() {
                        3 => {
                            let farads = self
                                .parse_scaled_value(fields[2], self.cap_scale, "capacitance", false)
                                .map_err(ParseWithAbortError::from)?;
                            let a = self
                                .parse_node_ref(fields[1])
                                .map_err(ParseWithAbortError::from)?;
                            net.caps.push(Cap {
                                a,
                                b: None,
                                farads,
                                line: self.line_num,
                            });
                        }
                        4 => {
                            let farads = self
                                .parse_scaled_value(fields[3], self.cap_scale, "capacitance", false)
                                .map_err(ParseWithAbortError::from)?;
                            let a = self
                                .parse_node_ref(fields[1])
                                .map_err(ParseWithAbortError::from)?;
                            let b = self
                                .parse_node_ref(fields[2])
                                .map_err(ParseWithAbortError::from)?;
                            net.caps.push(Cap {
                                a,
                                b: Some(b),
                                farads,
                                line: self.line_num,
                            });
                        }
                        _ => {
                            return Err(self
                                .error(format!(
                                    "malformed *CAP entry `{line}` (expected 3 or 4 fields)"
                                ))
                                .into());
                        }
                    }
                }
                _ if section == NetSection::Res => {
                    let net = current
                        .as_mut()
                        .ok_or_else(|| self.error("*RES entry outside *D_NET"))
                        .map_err(ParseWithAbortError::from)?;
                    if fields.len() != 4 {
                        return Err(self
                            .error(format!("malformed *RES entry `{line}` (expected 4 fields)"))
                            .into());
                    }
                    let ohms = self
                        .parse_scaled_value(fields[3], self.res_scale, "resistance", false)
                        .map_err(ParseWithAbortError::from)?;
                    let a = self
                        .parse_node_ref(fields[1])
                        .map_err(ParseWithAbortError::from)?;
                    let b = self
                        .parse_node_ref(fields[2])
                        .map_err(ParseWithAbortError::from)?;
                    net.ress.push(Res {
                        a,
                        b,
                        ohms,
                        line: self.line_num,
                    });
                }
                _ if section == NetSection::Induc => {
                    let net = current
                        .as_mut()
                        .ok_or_else(|| self.error("*INDUC entry outside *D_NET"))
                        .map_err(ParseWithAbortError::from)?;
                    if fields.len() != 4 {
                        return Err(self
                            .error(format!(
                                "malformed *INDUC entry `{line}` (expected 4 fields)"
                            ))
                            .into());
                    }
                    let henries = self
                        .parse_scaled_value(fields[3], self.induc_scale, "inductance", false)
                        .map_err(ParseWithAbortError::from)?;
                    let a = self
                        .parse_node_ref(fields[1])
                        .map_err(ParseWithAbortError::from)?;
                    let b = self
                        .parse_node_ref(fields[2])
                        .map_err(ParseWithAbortError::from)?;
                    net.inductors.push(Induc {
                        a,
                        b,
                        henries,
                        line: self.line_num,
                    });
                }
                _ => {}
            }
        }

        if let Some(reduced) = current_reduced {
            return Err(self
                .error(format!(
                    "reduced net `{}` is missing its terminating *END",
                    reduced.name
                ))
                .into());
        }
        if let Some(net) = current.take() {
            nets.push(net);
        }
        ensure_parse_not_aborted(abort)?;
        Ok(SpefFile { nets, reduced_nets })
    }

    fn validate_optional_routing_confidence(&self, fields: &[&str]) -> Result<(), ParseError> {
        if fields.len() == 3 {
            return Ok(());
        }
        if !fields[3].eq_ignore_ascii_case("*V") {
            return Err(self.error("optional routing confidence must begin with *V"));
        }
        let confidence: u32 = fields[4]
            .parse()
            .map_err(|_| self.error("routing confidence must be a positive integer"))?;
        if confidence == 0 {
            return Err(self.error("routing confidence must be a positive integer"));
        }
        Ok(())
    }

    fn finish_reduced_driver(
        &self,
        driver: ReducedDriverBuilder,
    ) -> Result<DriverReduction, ParseError> {
        let cell = driver
            .cell
            .ok_or_else(|| self.error("*DRIVER is missing its required *CELL record"))?;
        let (c2_farads, r1_ohms, c1_farads) = driver
            .pi
            .ok_or_else(|| self.error("*DRIVER is missing its required *C2_R1_C1 record"))?;
        if !driver.loads_started {
            return Err(self.error("*DRIVER is missing its required *LOADS record"));
        }
        if driver.loads.is_empty() {
            return Err(self.error("*LOADS must contain at least one *RC load record"));
        }
        // IEEE *RC values are load-specific Elmore delays, not physical
        // connectivity. A static passive pi has exactly one far-side transfer
        // time constant, R1*C1. It is exact for multiple loads only when every
        // single-pole descriptor names that same transfer. Anything else
        // needs a timing/macromodel runtime and must fail closed here.
        let physical_delay = r1_ohms * c1_farads;
        if !physical_delay.is_finite() || physical_delay <= 0.0 {
            return Err(self.error(format!(
                "*C2_R1_C1 far-side time constant is invalid or non-finite ({physical_delay})"
            )));
        }
        for load in &driver.loads {
            let tolerance = physical_delay.max(load.elmore_seconds) * 1.0e-4;
            if (load.elmore_seconds - physical_delay).abs() > tolerance {
                return Err(self.error(format!(
                    "*RC load Elmore delay {:.17e} s cannot be represented by the passive pi far-side R1*C1 delay {physical_delay:.17e} s",
                    load.elmore_seconds
                )));
            }
        }
        Ok(DriverReduction {
            driver: driver.driver,
            cell,
            c2_farads,
            r1_ohms,
            c1_farads,
            loads: driver.loads,
            line: driver.line,
        })
    }

    fn parse_unit(&self, fields: &[&str], table: &[(&str, Value)]) -> Result<Value, ParseError> {
        if fields.len() != 3 {
            return Err(self.error("unit statement must contain exactly a multiplier and unit"));
        }
        let multiplier: Value = fields
            .get(1)
            .and_then(|f| f.parse().ok())
            .ok_or_else(|| self.error("unit statement without a multiplier"))?;
        if !multiplier.is_finite() || multiplier <= 0.0 {
            return Err(self.error(format!(
                "unit multiplier must be finite and positive, got {multiplier}"
            )));
        }
        let unit = fields
            .get(2)
            .map(|f| f.to_ascii_uppercase())
            .ok_or_else(|| self.error("unit statement without a unit"))?;
        let scale = table
            .iter()
            .find(|(name, _)| *name == unit)
            .map(|(_, scale)| *scale)
            .ok_or_else(|| self.error(format!("unsupported unit `{unit}`")))?;
        let resolved = multiplier * scale;
        if !resolved.is_finite() || resolved <= 0.0 {
            return Err(self.error(format!("unit scale is invalid or non-finite ({resolved})")));
        }
        Ok(resolved)
    }

    fn parse_value(&self, field: &str) -> Result<Value, ParseError> {
        let value: Value = field
            .parse()
            .map_err(|_| self.error(format!("`{field}` is not a number")))?;
        if !value.is_finite() {
            return Err(self.error(format!("`{field}` is not finite")));
        }
        Ok(value)
    }

    fn parse_scaled_value(
        &self,
        field: &str,
        scale: Value,
        quantity: &str,
        allow_zero: bool,
    ) -> Result<Value, ParseError> {
        let value = self.parse_value(field)?;
        if value < 0.0 || (!allow_zero && value == 0.0) {
            let requirement = if allow_zero {
                "non-negative"
            } else {
                "strictly positive"
            };
            return Err(self.error(format!("{quantity} must be {requirement}, got {value}")));
        }
        let scaled = value * scale;
        if !scaled.is_finite() || scaled < 0.0 || (!allow_zero && scaled == 0.0) {
            return Err(self.error(format!(
                "scaled {quantity} is invalid or non-finite ({scaled})"
            )));
        }
        Ok(scaled)
    }

    /// Resolve `*<index>` through the name map; pass names through.
    fn resolve_name(&self, field: &str) -> Result<String, ParseError> {
        match parse_map_index(field) {
            Some(index) => self
                .name_map
                .get(&index)
                .cloned()
                .ok_or_else(|| self.error(format!("name-map reference `{field}` is undefined"))),
            None if field.is_empty() => Err(self.error("empty SPEF name")),
            None => Ok(field.to_owned()),
        }
    }

    /// Parse a node reference: `name`, `*3`, `name:4`, `*3:A`, `inst:pin`.
    fn parse_node_ref(&self, field: &str) -> Result<NodeRef, ParseError> {
        match field.split_once(self.delimiter) {
            None => Ok(NodeRef::Net(self.resolve_name(field)?)),
            Some((base, sub)) => {
                let base = self.resolve_name(base)?;
                if sub.is_empty() {
                    return Err(self.error(format!("node reference `{field}` has an empty suffix")));
                }
                if parse_map_index(sub).is_some() {
                    return Ok(NodeRef::Pin(base, self.resolve_name(sub)?));
                }
                // A purely numeric suffix is an internal subnode; anything
                // else is an instance pin.
                if sub.chars().all(|ch| ch.is_ascii_digit()) {
                    Ok(NodeRef::SubNode(base, sub.to_owned()))
                } else {
                    Ok(NodeRef::Pin(base, sub.to_owned()))
                }
            }
        }
    }

    /// Parse a grammar position that requires `instance<delimiter>pin`.
    /// Numeric pin names are pins here, not detailed-net internal subnodes.
    fn parse_pin_ref(&self, field: &str) -> Result<NodeRef, ParseError> {
        let (instance, pin) = field.split_once(self.delimiter).ok_or_else(|| {
            self.error(format!(
                "instance pin `{field}` is missing delimiter `{}`",
                self.delimiter
            ))
        })?;
        let instance = self.resolve_name(instance)?;
        if pin.is_empty() {
            return Err(self.error(format!("instance pin `{field}` has an empty pin name")));
        }
        let pin = if parse_map_index(pin).is_some() {
            self.resolve_name(pin)?
        } else {
            pin.to_owned()
        };
        Ok(NodeRef::Pin(instance, pin))
    }
}

/// `*<digits>` name-map reference.
fn parse_map_index(field: &str) -> Option<u64> {
    field.strip_prefix('*')?.parse().ok()
}

/// SPEF `//` end-of-line comments.
fn strip_comment_with_abort<'a>(
    line: &'a str,
    abort: &dyn AbortSignal,
) -> Result<&'a str, ParseWithAbortError> {
    let bytes = line.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        if *byte == b'/' && bytes.get(index + 1) == Some(&b'/') {
            return Ok(&line[..index]);
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spef_parser_aborts_after_multiple_mid_document_polls() {
        let mut source =
            String::from("*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*D_NET data 1\n*CAP\n");
        for index in 0..1_024 {
            source.push_str(&format!("{} data:{} 1\n", index + 1, index + 1));
        }
        source.push_str("*END\n");
        let abort = crate::abort_signal::CountingAbort::new(20);

        let result = SpefFile::parse_with_abort(&source, &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 20, "SPEF parsing must poll during work");
    }

    #[test]
    fn spef_parser_aborts_inside_a_large_single_record() {
        let mut source = String::from("*SPEF");
        for index in 0..4_096 {
            source.push_str(&format!(" field{index}"));
        }
        source.push('\n');
        let abort = crate::abort_signal::CountingAbort::new(12);

        let result = SpefFile::parse_with_abort(&source, &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(
            abort.count() > 12,
            "single-record tokenization must remain cancellable"
        );
    }

    #[test]
    fn spef_back_annotation_aborts_transactionally_mid_network() {
        let mut source =
            String::from("*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*D_NET data 1\n*CAP\n");
        for index in 0..1_024 {
            source.push_str(&format!("{} data:{} 1\n", index + 1, index + 1));
        }
        source.push_str("*END\n");
        let spef = SpefFile::parse(&source).expect("fixture SPEF parses");
        let mut netlist = Netlist::default();
        let original_elements = netlist.elements.len();
        let abort = crate::abort_signal::CountingAbort::new(8);

        let result = spef.apply_with_abort(&mut netlist, &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 8, "SPEF application must poll during work");
        assert_eq!(
            netlist.elements.len(),
            original_elements,
            "an aborted back-annotation must not publish partial elements"
        );
    }

    #[test]
    fn parses_reduced_pi_and_lumped_capacitance_records_with_name_mapping() {
        let source = "\
*SPEF \"IEEE 1481-2009\"
*DELIMITER :
*C_UNIT 1 NF
*R_UNIT 1 KOHM
*NAME_MAP
*1 in
*2 I1
*3 2
*4 ILOAD
*5 1
*R_NET *1 3
*DRIVER *2:*3
*CELL CURRENT_SOURCE
*C2_R1_C1 1 2 2
*LOADS
*RC *4:*5 4
*END
*C_NET shunt 5
*END
";

        let spef = SpefFile::parse(source).expect("reduced SPEF parses");

        assert_eq!(spef.reduced_nets.len(), 2);
        let pi = &spef.reduced_nets[0];
        assert_eq!(pi.name, "in");
        assert!((pi.total_capacitance - 3.0e-9).abs() <= f64::EPSILON * 3.0e-9);
        assert_eq!(pi.drivers.len(), 1);
        assert_eq!(
            pi.drivers[0].driver,
            NodeRef::Pin("I1".to_owned(), "2".to_owned())
        );
        assert_eq!(pi.drivers[0].r1_ohms, 2.0e3);
        assert!((pi.drivers[0].loads[0].elmore_seconds - 4.0e-6).abs() <= f64::EPSILON * 4.0e-6);
        assert_eq!(spef.reduced_nets[1].kind, ReducedNetKind::Capacitance);
        assert!((spef.reduced_nets[1].total_capacitance - 5.0e-9).abs() <= f64::EPSILON * 5.0e-9);
    }

    #[test]
    fn reduced_spef_parser_aborts_during_large_load_section() {
        let mut source = String::from(
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n",
        );
        for index in 0..4_096 {
            source.push_str(&format!("*RC R{index}:1 1\n"));
        }
        source.push_str("*END\n");
        let abort = crate::abort_signal::CountingAbort::new(80);

        let result = SpefFile::parse_with_abort(&source, &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 80, "reduced parsing must poll during work");
    }

    #[test]
    fn reduced_spef_application_aborts_transactionally_during_load_rewiring() {
        let mut source = String::from(
            "*SPEF \"IEEE 1481-2009\"\n*C_UNIT 1 PF\n*R_UNIT 1 OHM\n*R_NET in 2\n*DRIVER V1:1\n*CELL VOLTAGE_SOURCE\n*C2_R1_C1 1 1 1\n*LOADS\n",
        );
        let mut deck = String::from("reduced cancellation\nV1 in 0 DC 0\n");
        for index in 0..1_024 {
            source.push_str(&format!("*RC R{index}:1 1\n"));
            deck.push_str(&format!("R{index} in 0 1meg\n"));
        }
        source.push_str("*END\n");
        deck.push_str(".op\n.end\n");
        let spef = SpefFile::parse(&source).expect("large reduced SPEF parses");
        let mut netlist = Netlist::parse(&deck).expect("large fixture deck parses");
        let original_nodes: Vec<_> = netlist
            .elements
            .iter()
            .map(|element| element.nodes.clone())
            .collect();
        let abort = crate::abort_signal::CountingAbort::new(24);

        let result = spef.apply_with_abort(&mut netlist, &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 24, "reduced lowering must poll during work");
        assert_eq!(
            netlist
                .elements
                .iter()
                .map(|element| element.nodes.clone())
                .collect::<Vec<_>>(),
            original_nodes,
            "an aborted reduced annotation must not publish partial rewiring"
        );
    }

    #[test]
    fn multiple_reduced_driver_views_fail_closed() {
        let source = "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 PF
*R_UNIT 1 OHM
*R_NET in 2
*DRIVER V1:1
*CELL VOLTAGE_SOURCE
*C2_R1_C1 1 1 1
*LOADS
*RC R1:1 1
*DRIVER V2:1
*CELL VOLTAGE_SOURCE
*C2_R1_C1 1 1 1
*LOADS
*RC R2:1 1
*END
";

        let error = SpefFile::parse(source).expect_err("alternative driver views cannot add");

        assert!(error.to_string().contains("multiple driver views"));
    }

    #[test]
    fn reduced_load_delay_must_be_representable_by_the_passive_pi() {
        let source = "\
*SPEF \"IEEE 1481-2009\"
*C_UNIT 1 PF
*R_UNIT 1 KOHM
*R_NET in 2
*DRIVER V1:1
*CELL VOLTAGE_SOURCE
*C2_R1_C1 1 2 1
*LOADS
*RC R1:1 9
*END
";

        let error = SpefFile::parse(source)
            .expect_err("an unrelated Elmore delay cannot become a passive far node");

        assert!(error.to_string().contains("cannot be represented"));
        assert!(error.to_string().contains("R1*C1"));
    }

    const SPEF: &str = r#"
*SPEF "IEEE 1481-2009"
*DESIGN "demo"
*DIVIDER /
*DELIMITER :
*BUS_DELIMITER [ ]
*T_UNIT 1 NS
*C_UNIT 1 FF
*R_UNIT 1 KOHM
*L_UNIT 1 HENRY

*NAME_MAP
*1 in
*2 XBUF

*PORTS
*1 I

*D_NET *1 2.4
*CONN
*P *1 I
*I *2:A I *C 1.0 2.0
*CAP
1 *1:1 1.5 // grounded
2 *1:2 *2:A 0.9
*RES
1 *1 *1:1 0.5
2 *1:1 *2:A 1.0
*END
"#;

    fn deck() -> Netlist {
        Netlist::parse(
            "spef demo\n\
             .subckt buf A Y\n\
             R1 A Y 100\n\
             .ends\n\
             V1 in 0 DC 1\n\
             XBUF in out buf\n\
             R2 out 0 1k\n\
             .op\n\
             .end\n",
        )
        .expect("deck parses")
    }

    #[test]
    fn parses_units_names_and_sections() {
        let spef = SpefFile::parse(SPEF).expect("spef parses");
        assert_eq!(spef.nets.len(), 1);
        let net = &spef.nets[0];
        assert_eq!(net.name, "in");
        assert_eq!(net.conns.len(), 2);
        assert_eq!(net.caps.len(), 2);
        assert_eq!(net.ress.len(), 2);
        // 1.5 in 1-FF units.
        assert!((net.caps[0].farads - 1.5e-15).abs() < 1e-22);
        // 0.5 in 1-KOHM units.
        assert!((net.ress[0].ohms - 500.0).abs() < 1e-9);
    }

    #[test]
    fn parses_every_standard_inductance_unit() {
        for (unit, expected) in [("HENRY", 2.0), ("MH", 2.0e-3), ("UH", 2.0e-6)] {
            let source = format!(
                "*SPEF \"IEEE 1481-2009\"\n*L_UNIT 1 {unit}\n*D_NET in 0\n*INDUC\n1 in in:1 2\n*END\n"
            );
            let spef = SpefFile::parse(&source).expect("standard inductance unit parses");
            assert_eq!(spef.nets[0].inductors.len(), 1);
            assert_eq!(spef.nets[0].inductors[0].henries, expected, "{unit}");
        }
    }

    #[test]
    fn generated_inductor_names_do_not_collide_with_authored_elements() {
        let spef = SpefFile::parse(
            "*SPEF \"IEEE 1481-2009\"\n*L_UNIT 1 UH\n*D_NET in 0\n*INDUC\n1 in in:1 2\n*END\n",
        )
        .expect("SPEF parses");
        let mut netlist = Netlist::parse(
            "collision guard\nI1 0 in DC 0\nLSPEF1 spare 0 1u\nR1 spare 0 1\n.op\n.end\n",
        )
        .expect("deck parses");

        let report = spef.apply(&mut netlist);

        assert_eq!(report.inductors, 1);
        assert_eq!(
            netlist
                .elements
                .iter()
                .filter(|element| element.name.eq_ignore_ascii_case("LSPEF1"))
                .count(),
            1,
            "the authored element name must remain unique"
        );
        assert!(
            netlist
                .elements
                .iter()
                .any(|element| element.name.eq_ignore_ascii_case("LSPEF2")),
            "the generated element must advance past the collision"
        );
    }

    #[test]
    fn apply_rewires_pins_and_adds_parasitics() {
        let spef = SpefFile::parse(SPEF).expect("spef parses");
        let mut netlist = deck();
        let report = spef.apply(&mut netlist);

        assert_eq!(report.nets, 1);
        assert_eq!(report.rewired_pins, 1);
        assert_eq!(report.skipped_pins, 0);
        assert_eq!(report.resistors, 2);
        assert_eq!(report.capacitors, 2);

        // The XBUF A-terminal moved off the ideal net onto its pin node.
        let xbuf = netlist
            .elements
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case("XBUF"))
            .expect("XBUF present");
        assert_eq!(xbuf.nodes[0], "XBUF__A");

        // The parasitic chain reaches from the port node (deck names
        // preserved) through the subnode to the rewired pin.
        let res_nodes: Vec<&[String]> = netlist
            .elements
            .iter()
            .filter(|e| e.name.starts_with("RSPEF"))
            .map(|e| e.nodes.as_slice())
            .collect();
        assert!(res_nodes.iter().any(|n| n[0] == "IN" && n[1] == "IN__1"));
        assert!(
            res_nodes
                .iter()
                .any(|n| n[0] == "IN__1" && n[1] == "XBUF__A")
        );
    }

    #[test]
    fn mismatched_pin_net_is_skipped_not_rewired() {
        let bad = SPEF.replace("*1 in", "*1 othernet");
        let spef = SpefFile::parse(&bad).expect("spef parses");
        let mut netlist = deck();
        let report = spef.apply(&mut netlist);
        // XBUF:A sits on net1 in the deck, not `othernet` — skip + warn.
        assert_eq!(report.rewired_pins, 0);
        assert_eq!(report.skipped_pins, 1);
    }
}
