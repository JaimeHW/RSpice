//! Structural identity for a fully built circuit.
//!
//! Run planners need this before publishing any coordinate output.  The
//! fingerprint is assembled from the actual MNA unknown layout and frozen
//! sparse pattern plus the flattened component/model vocabulary.  Numeric
//! values are deliberately absent, so ordinary parameter sweeps keep one
//! topology identity while conditionals and topology-selecting model options
//! change it.
//!
//! The computation lives here rather than on `Engine` because
//! [`TopologyFingerprint`] is a planning identity: it is consumed by the deck
//! materializer that publishes coordinate output, and an engine that reached
//! up for it would invert the layer order (see `tests/module_layering.rs`).
//! Both directions of the work are still done exactly once — the engine
//! elaborates, this module names the result.

use std::collections::BTreeMap;

use super::{TopologyComponent, TopologyFingerprint};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::engine::{Engine, SimulationError};
use crate::netlist::{
    Element, ElementKind, FlattenerConfig, Netlist, XspicePort,
    flatten_netlist_with_models_config_with_abort, reduce_supernode_topology,
};

/// Fingerprint the solver topology an engine produces from a parsed netlist.
pub fn topology_fingerprint(
    engine: &Engine,
    netlist: &Netlist,
) -> Result<TopologyFingerprint, SimulationError> {
    topology_fingerprint_with_abort(engine, netlist, &NoAbort)
}

/// Fingerprint the solver topology while observing cooperative aborts.
pub fn topology_fingerprint_with_abort(
    engine: &Engine,
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<TopologyFingerprint, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let engine = engine.resolved_for_netlist(netlist);

    // The matrix is the authoritative elaborated sparsity, including
    // generated/private unknowns selected by model options.
    let circuit = engine.build_circuit_with_abort(netlist, abort)?;
    let stamp_sparsity = if circuit.matrix_size() == 0 {
        Vec::new()
    } else {
        engine.build_matrix(&circuit)?.stored_positions().collect()
    };
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }

    // `build_circuit_with_abort` restarted and consumed the deterministic
    // statistical stream. Restart it again so topology descriptors are
    // produced from the exact same flattened coordinate, not the next
    // pseudo-random draw.
    netlist.params.restart_statistical_stream();
    let mut flattened = flatten_netlist_with_models_config_with_abort(
        netlist,
        FlattenerConfig {
            max_depth: engine.config.resource_limits.max_hierarchy_depth,
            max_elements: engine.config.resource_limits.max_flattened_elements,
            ..FlattenerConfig::default()
        },
        abort,
    )
    .map_err(|error| {
        SimulationError::Circuit(format!(
            "topology fingerprint hierarchy flattening failed: {error}"
        ))
    })?;
    if netlist.options.topology_supernode.unwrap_or(false) {
        flattened.elements = reduce_supernode_topology(
            flattened.elements,
            netlist
                .options
                .device_zero_resistance_tol
                .unwrap_or(crate::netlist::XYCE_DEFAULT_ZERO_RESISTANCE_TOL),
        )
        .elements;
    }

    let mut nodes = vec!["0".to_string()];
    nodes.extend(circuit.node_names_sorted());
    let canonical_nodes = nodes
        .iter()
        .map(|name| (name.trim().to_ascii_lowercase(), name.clone()))
        .collect::<BTreeMap<_, _>>();
    let ground_policy = netlist.ground_policy();

    let mut model_types = BTreeMap::new();
    // Source order never decides which spelling becomes canonical.
    for model in netlist.models.iter().chain(flattened.scoped_models.iter()) {
        let candidate = model.model_type.to_ascii_lowercase();
        model_types
            .entry(model.name.to_ascii_lowercase())
            .and_modify(|kind| {
                if candidate < *kind {
                    kind.clone_from(&candidate);
                }
            })
            .or_insert(candidate);
    }

    let mut components = Vec::with_capacity(flattened.elements.len().saturating_add(1));
    for element in &flattened.elements {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let (kind, model_reference) = element_kind_and_model(&element.kind);
        let model_identity = model_reference.map_or_else(
            || format!("builtin:{kind}"),
            |name| {
                let implementation = model_types
                    .get(&name.to_ascii_lowercase())
                    .map_or("untyped", String::as_str);
                format!("model:{implementation}:{}", name.to_ascii_lowercase())
            },
        );
        let terminals = element_terminal_names(element)
            .into_iter()
            .map(|terminal| {
                let canonical = ground_policy.canonical_node(&terminal);
                canonical_nodes
                    .get(&canonical.to_ascii_lowercase())
                    .cloned()
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "cannot fingerprint elaborated topology: flattened component '{}' terminal '{}' is absent from the built circuit node namespace",
                            element.name, terminal
                        ))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        components.push(
            TopologyComponent::new(
                kind,
                &element.name,
                model_identity,
                terminals,
                std::iter::empty(),
            )
            .map_err(topology_error)?,
        );
    }

    // Component descriptors retain identity/terminals; this aggregate
    // descriptor retains the exact assembled matrix positions.  Keeping
    // the two facts in one collision-resistant fingerprint avoids
    // assigning shared nonlinear positions to an arbitrary device.
    components.push(
        TopologyComponent::new(
            "assembled-mna",
            "rspice-assembled-circuit",
            format!("sparse-mna-v1:devices={}", circuit.device_count()),
            nodes.iter().cloned(),
            stamp_sparsity,
        )
        .map_err(topology_error)?,
    );

    let mut unknown_layout = circuit
        .node_names_sorted()
        .into_iter()
        .map(|name| format!("v({name})"))
        .collect::<Vec<_>>();
    unknown_layout.extend(
        circuit
            .branch_names_sorted()
            .into_iter()
            .map(|name| format!("i({name})")),
    );
    unknown_layout
        .extend((0..circuit.hidden_state_count()).map(|index| format!("state-unknown-{index}")));
    let state_layout = (0..circuit.hidden_state_count())
        .map(|index| format!("state-{index}"))
        .collect::<Vec<_>>();

    TopologyFingerprint::from_materialized(nodes, unknown_layout, state_layout, components)
        .map_err(topology_error)
}

fn topology_error(error: impl std::fmt::Display) -> SimulationError {
    SimulationError::Circuit(format!("cannot fingerprint elaborated topology: {error}"))
}

fn element_kind_and_model(kind: &ElementKind) -> (&'static str, Option<&str>) {
    match kind {
        ElementKind::Resistor { model, .. } => ("resistor", model.as_deref()),
        ElementKind::Capacitor { model, .. } => ("capacitor", model.as_deref()),
        ElementKind::Inductor { model, .. } => ("inductor", model.as_deref()),
        ElementKind::JilesAthertonInductor { model, .. } => {
            ("jiles-atherton-inductor", Some(model))
        }
        ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => {
            ("voltage-source", None)
        }
        ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => {
            ("current-source", None)
        }
        ElementKind::Diode { model, .. } => ("diode", Some(model)),
        ElementKind::Bjt { model, .. } => ("bjt", Some(model)),
        ElementKind::Mosfet { model, .. } => ("mosfet", Some(model)),
        ElementKind::Jfet { model, .. } => ("jfet", Some(model)),
        ElementKind::Mesfet { model, .. } => ("mesfet", Some(model)),
        ElementKind::XyceMemristor { model, .. } => ("xyce-memristor", Some(model)),
        ElementKind::Vcvs { .. } => ("vcvs", None),
        ElementKind::Cccs { .. } => ("cccs", None),
        ElementKind::Vccs { .. } => ("vccs", None),
        ElementKind::PspiceChebyshev { .. } => ("pspice-chebyshev", None),
        ElementKind::Ccvs { .. } => ("ccvs", None),
        ElementKind::BehavioralVoltage { .. } => ("behavioral-voltage", None),
        ElementKind::BehavioralCurrent { .. } => ("behavioral-current", None),
        ElementKind::VSwitch { model, .. } => ("voltage-switch", Some(model)),
        ElementKind::ISwitch { model, .. } => ("current-switch", Some(model)),
        ElementKind::GenericSwitch { model, .. } => ("generic-switch", Some(model)),
        ElementKind::TransmissionLine { model, .. } => ("transmission-line", model.as_deref()),
        ElementKind::Coupling { model, .. } => ("coupling", model.as_deref()),
        ElementKind::Subcircuit { subckt_name, .. } => ("subcircuit", Some(subckt_name)),
        ElementKind::Xspice { model, .. } => ("xspice", Some(model)),
    }
}

fn element_terminal_names(element: &Element) -> Vec<String> {
    let mut terminals = element.nodes.clone();
    match &element.kind {
        ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
            terminals.extend([control_nodes.0.clone(), control_nodes.1.clone()]);
        }
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            ..
        } => terminals.extend([control_pos.clone(), control_neg.clone()]),
        ElementKind::Xspice { ports, .. } => {
            for port in ports {
                append_xspice_port_nodes(port, &mut terminals);
            }
        }
        _ => {}
    }
    terminals
}

fn append_xspice_port_nodes(port: &XspicePort, terminals: &mut Vec<String>) {
    match port {
        XspicePort::Analog(node)
        | XspicePort::Digital(node)
        | XspicePort::ExplicitDigital(node)
        | XspicePort::DigitalInverted(node)
        | XspicePort::Conductance(node)
        | XspicePort::Current(node)
        | XspicePort::Hybrid(node) => terminals.push(node.clone()),
        XspicePort::AnalogVector(nodes) | XspicePort::DigitalVector(nodes) => {
            terminals.extend(nodes.iter().cloned());
        }
        XspicePort::DigitalVectorMixed(nodes) => {
            terminals.extend(nodes.iter().map(|node| node.name.clone()));
        }
        XspicePort::DifferentialVoltage { pos, neg }
        | XspicePort::DifferentialCurrent { pos, neg }
        | XspicePort::DifferentialConductance { pos, neg }
        | XspicePort::DifferentialHybrid { pos, neg } => {
            terminals.extend([pos.clone(), neg.clone()]);
        }
        XspicePort::VoltageName(_) | XspicePort::Null => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_values_do_not_change_the_assembled_topology_fingerprint() {
        let first = Netlist::parse("same topology\nV1 in 0 1\nR1 in 0 1k\n.end\n")
            .expect("first deck parses");
        let second = Netlist::parse("same topology\nV1 in 0 2\nR1 in 0 2k\n.end\n")
            .expect("second deck parses");
        let engine = Engine::default();

        assert_eq!(
            topology_fingerprint(&engine, &first).expect("first topology"),
            topology_fingerprint(&engine, &second).expect("second topology")
        );
    }

    #[test]
    fn component_membership_and_connectivity_change_the_fingerprint() {
        let base = Netlist::parse("base topology\nV1 in 0 1\nR1 in 0 1k\n.end\n")
            .expect("base deck parses");
        let parallel =
            Netlist::parse("parallel topology\nV1 in 0 1\nR1 in 0 1k\nR2 in 0 2k\n.end\n")
                .expect("parallel deck parses");
        let extra_node =
            Netlist::parse("extra topology\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n")
                .expect("extra-node deck parses");
        let engine = Engine::default();
        let base = topology_fingerprint(&engine, &base).expect("base topology");

        assert_ne!(
            base,
            topology_fingerprint(&engine, &parallel).expect("parallel topology")
        );
        assert_ne!(
            base,
            topology_fingerprint(&engine, &extra_node).expect("extra-node topology")
        );
    }

    #[test]
    fn terminal_polarity_order_is_part_of_the_fingerprint() {
        let forward = Netlist::parse("forward polarity\nV1 a b 1\nR1 a 0 1k\nR2 b 0 1k\n.end\n")
            .expect("forward deck parses");
        let reverse = Netlist::parse("reverse polarity\nV1 b a 1\nR1 a 0 1k\nR2 b 0 1k\n.end\n")
            .expect("reverse deck parses");
        let engine = Engine::default();

        assert_ne!(
            topology_fingerprint(&engine, &forward).expect("forward topology"),
            topology_fingerprint(&engine, &reverse).expect("reverse topology"),
            "terminal order and source polarity must remain structural"
        );
    }
}
