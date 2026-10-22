//! SPICE leaves inside an HDL hierarchy: Verilog-AMS on top.
//!
//! A design may instantiate a deck `.subckt`, a device a deck model card
//! makes, or Verilog-AMS LRM 2.4 Annex E's `resistor`, `capacitor` and
//! `inductor`. The compiler takes the deck's cells as a table
//! ([`external_cells`]) and lists each leaf of a compiled design with the
//! nets it connects. Once the deck's HDL is compiled, every leaf becomes a
//! card at deck scope named `<instance>.<path>`: on the X-card's own nodes for
//! the root's ports, and on `<instance>.<net>` for a net inside the design,
//! which is the analog half's internal node of that name when it has one. The
//! deck is then built once more with those cards in it, so the flattener
//! expands them as it expands any card, scoped models included, and every
//! later stage sees ordinary deck elements.
//!
//! A discrete port of the design on such a node, and one an in-design
//! discipline boundary exposes, is an event port of the host on that node
//! ([`hoisted_node`]), which `mixed_boundaries` converts as it converts a
//! deck-level port.

use crate::netlist::{Element, ElementKind, Netlist};
use crate::{ElaborationError, ElaborationErrorKind, SimulationError};
use rspice_veriloga::ExternalCell;
use rspice_veriloga::canonical_ir::{CanonicalIrArtifact, HirExternalLeaf, HirExternalNet};

/// A device a model card makes: its element letter, terminals and the
/// instance parameters a design may override.
struct ModelDevice {
    letter: char,
    ports: &'static [&'static str],
    parameters: &'static [&'static str],
}

fn model_device(model_type: &str) -> Option<ModelDevice> {
    let (letter, ports, parameters): (char, &[&str], &[&str]) =
        match model_type.to_ascii_lowercase().as_str() {
            "d" => ('d', &["a", "c"], &["area", "m"]),
            "npn" | "pnp" => ('q', &["c", "b", "e"], &["area", "m"]),
            "nmos" | "pmos" => (
                'm',
                &["d", "g", "s", "b"],
                &["w", "l", "m", "nf", "ad", "as", "pd", "ps"],
            ),
            "njf" | "pjf" => ('j', &["d", "g", "s"], &["area", "m"]),
            "sw" => ('s', &["p", "n", "cp", "cn"], &["m"]),
            _ => return None,
        };
    Some(ModelDevice {
        letter,
        ports,
        parameters,
    })
}

/// The cells `netlist` offers a design: its `.subckt`s, then the model cards
/// whose devices a design may instantiate. A name both declare is the
/// subcircuit's.
pub(super) fn external_cells(netlist: &Netlist) -> Vec<ExternalCell> {
    let mut cells: Vec<ExternalCell> = netlist
        .subcircuits
        .iter()
        .map(|subckt| ExternalCell {
            name: subckt.name.clone(),
            ports: subckt.ports.clone(),
            parameters: subckt
                .params
                .iter()
                .map(|(name, _)| name)
                .chain(subckt.expr_params.iter().map(|(name, _)| name))
                .cloned()
                .collect(),
        })
        .collect();
    for model in &netlist.models {
        let Some(device) = model_device(&model.model_type) else {
            continue;
        };
        if cells
            .iter()
            .any(|cell| cell.name.eq_ignore_ascii_case(&model.name))
        {
            continue;
        }
        cells.push(ExternalCell {
            name: model.name.clone(),
            ports: device.ports.iter().map(|port| (*port).to_owned()).collect(),
            parameters: device
                .parameters
                .iter()
                .map(|name| (*name).to_owned())
                .collect(),
        });
    }
    cells
}

/// The node name a leaf port or a hoisted port of `element`'s design lands on.
pub(super) fn node_name(
    element: &Element,
    artifact: &CanonicalIrArtifact,
    net: &HirExternalNet,
) -> Option<String> {
    match net {
        HirExternalNet::Ground => Some("0".to_owned()),
        HirExternalNet::Port(port) => {
            let first: usize = super::mixed_modules::port_node_widths(artifact)
                .take(usize::from(*port))
                .sum();
            element.nodes.get(first).cloned()
        }
        HirExternalNet::Net(name) => Some(format!("{}.{name}", element.name)),
    }
}

/// The circuit node a hoisted port of `element`'s design lands on.
pub(super) fn hoisted_node(
    circuit: &mut crate::CircuitData,
    element: &Element,
    artifact: &CanonicalIrArtifact,
    net: &HirExternalNet,
) -> Option<usize> {
    let name = node_name(element, artifact, net)?;
    Some(if name == "0" {
        0
    } else {
        circuit.get_or_create_node(&name)
    })
}

/// The nets inside `element`'s design that its leaves and hoisted ports name,
/// folded as the deck's own nets are. They are the design's, so the analog
/// half's internal node of the same name is the same net, not a collision.
pub(super) fn design_nets(
    element: &Element,
    artifact: &CanonicalIrArtifact,
) -> std::collections::HashSet<String> {
    artifact
        .hir
        .external_leaves
        .iter()
        .flat_map(|leaf| leaf.connections.iter())
        .chain(artifact.hir.hoisted_ports.iter().map(|port| &port.net))
        .filter_map(|net| match net {
            HirExternalNet::Net(name) => Some(super::canonical_observable_name(&format!(
                "{}.{name}",
                element.name
            ))),
            _ => None,
        })
        .collect()
}

/// `netlist` with every leaf of the designs its HDL X-cards bind as a card
/// at deck scope, or `None` when no design has a leaf. `design` names the
/// compiled design an X-card's master binds, if any.
pub(super) fn deck_with_leaves(
    netlist: &Netlist,
    flat_elements: &[Element],
    design: impl Fn(&str) -> Option<std::sync::Arc<CanonicalIrArtifact>>,
) -> Result<Option<Netlist>, SimulationError> {
    let mut lines = String::new();
    let mut names = Vec::new();
    for element in flat_elements {
        let ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
            continue;
        };
        let Some(artifact) = design(subckt_name) else {
            continue;
        };
        if artifact.hir.external_leaves.is_empty() {
            continue;
        }
        let declared: usize = super::mixed_modules::port_node_widths(&artifact).sum();
        if element.nodes.len() != declared {
            // The instance itself refuses its node count, by name, later.
            continue;
        }
        for leaf in &artifact.hir.external_leaves {
            let name = format!("{}.{}", element.name, leaf.instance);
            let nodes = leaf
                .connections
                .iter()
                .map(|net| node_name(element, &artifact, net))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    refuse(&name, &leaf.cell, "names a port its design does not have")
                })?;
            lines.push_str(&card(netlist, leaf, names.len(), &nodes, &name)?);
            lines.push('\n');
            names.push((name, nodes));
        }
    }
    if names.is_empty() {
        return Ok(None);
    }
    let parsed = crate::netlist::parse_netlist(&format!("* SPICE leaves\n{lines}.end\n")).map_err(
        |error| {
            SimulationError::Netlist(format!(
                "the SPICE leaves of the deck's HDL designs: {error}"
            ))
        },
    )?;
    if parsed.elements.len() != names.len() {
        return Err(SimulationError::Netlist(format!(
            "the SPICE leaves of the deck's HDL designs read as {} cards, not {}",
            parsed.elements.len(),
            names.len()
        )));
    }
    let mut deck = netlist.clone();
    deck.elements
        .extend(
            parsed
                .elements
                .into_iter()
                .zip(names)
                .map(|(mut element, (name, nodes))| {
                    element.name = name;
                    // The parser folds node names, and a net inside the
                    // design is the analog half's internal node, which keeps
                    // the instance's own spelling (`X1.inner`). The leaf
                    // takes that spelling back, so both are one node.
                    for (parsed, spelled) in element.nodes.iter_mut().zip(nodes) {
                        if parsed.eq_ignore_ascii_case(&spelled) {
                            *parsed = spelled;
                        }
                    }
                    element
                }),
        );
    Ok(Some(deck))
}

/// One leaf as a SPICE card, named `<letter>__rspice_leaf<ordinal>` until the
/// caller gives it its hierarchical name.
fn card(
    netlist: &Netlist,
    leaf: &HirExternalLeaf,
    ordinal: usize,
    nodes: &[String],
    name: &str,
) -> Result<String, SimulationError> {
    let nodes = nodes.join(" ");
    let parameters = |leaf: &HirExternalLeaf| {
        leaf.parameters
            .iter()
            .map(|(parameter, value)| format!(" {parameter}={value:e}"))
            .collect::<String>()
    };
    if let Some(subckt) = netlist
        .subcircuits
        .iter()
        .find(|subckt| subckt.name.eq_ignore_ascii_case(&leaf.cell))
    {
        return Ok(format!(
            "x__rspice_leaf{ordinal} {nodes} {}{}",
            subckt.name,
            parameters(leaf)
        ));
    }
    let primitive = match leaf.cell.as_str() {
        "resistor" => Some(('r', "r")),
        "capacitor" => Some(('c', "c")),
        "inductor" => Some(('l', "l")),
        _ => None,
    };
    if let Some((letter, parameter)) = primitive {
        let (_, value) = leaf
            .parameters
            .iter()
            .find(|(name, _)| name == parameter)
            .ok_or_else(|| {
                refuse(
                    name,
                    &leaf.cell,
                    format!("needs its value, `{parameter}`, which the design does not give"),
                )
            })?;
        return Ok(format!("{letter}__rspice_leaf{ordinal} {nodes} {value:e}"));
    }
    let model = netlist
        .models
        .iter()
        .find(|model| model.name.eq_ignore_ascii_case(&leaf.cell))
        .and_then(|model| Some((model, model_device(&model.model_type)?)))
        .ok_or_else(|| refuse(name, &leaf.cell, "names a cell this deck does not declare"))?;
    Ok(format!(
        "{}__rspice_leaf{ordinal} {nodes} {}{}",
        model.1.letter,
        model.0.name,
        parameters(leaf)
    ))
}

fn refuse(instance: &str, cell: &str, detail: impl Into<String>) -> SimulationError {
    ElaborationError::new(ElaborationErrorKind::UnknownModule, detail)
        .instance(instance)
        .module(cell)
        .into()
}
