//! `.SP` port discovery and excitation from a netlist.
//!
//! A deck declares its RF ports by annotating voltage sources with
//! `portnum=<n> [z0=<ohms>]`. Turning those annotations into an ordered,
//! validated port list — and then driving one port at a time to extract a
//! column of the admittance matrix — is identical work for every front-end,
//! and used to exist as four separate copies with four slightly different
//! error strings.
//!
//! Port numbers must be dense and unique starting at 1. That is stricter than
//! simply sorting whatever was found: a deck that declares ports 1 and 3 has
//! almost certainly lost port 2 to a typo, and silently relabelling port 3 as
//! port 2 would produce a plausible S-matrix describing the wrong network.

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::netlist::{Element, ElementKind, ElementProvenance, Netlist, SourceSpec};
use crate::{ResourceKind, ResourceLimitError, Value};
use std::collections::HashSet;

/// One `.SP` port resolved from a netlist annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct SParameterPort {
    /// Declared `portnum`, one-based. Guaranteed dense after collection.
    pub number: usize,
    /// Name of the voltage source carrying the annotation.
    pub source_name: String,
    /// Node the port is measured at — its reference plane.
    pub node_pos: String,
    /// Negative terminal node.
    pub node_neg: String,
    /// Reference impedance in ohms; always positive and finite.
    pub z0: Value,
    /// Whether `z0` is already a resistor in the circuit.
    pub realization: PortRealization,
}

/// How a port's reference impedance is represented in the netlist.
///
/// Concise annotations are expanded during circuit construction; a lowered
/// netlist already contains the source's physical reference resistor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortRealization {
    /// A concise `portnum=` annotation whose series Z0 still needs lowering.
    Ideal,
    /// A Thevenin generator drives the plane through a real `z0` resistor, as
    /// Xyce's `P` element does.
    Thevenin,
}

/// Why a deck's `.SP` port declarations could not be used.
#[derive(Debug, Clone, PartialEq)]
pub enum PortError {
    /// Port preparation was cancelled before publication.
    Aborted,
    /// Materializing the physical port would exceed the circuit budget.
    ResourceLimit(ResourceLimitError),
    /// An annotated source had fewer than two terminals.
    MissingTerminals { source_name: String },
    /// A `z0=` annotation was not a positive, finite resistance.
    InvalidReferenceImpedance { source_name: String, z0: Value },
    /// The deck declared no ports at all.
    NoPortsDeclared,
    /// Port numbers were not dense and unique from 1.
    NonDensePortNumbers {
        expected: usize,
        found: usize,
        source_name: String,
    },
    /// A previously collected port source vanished before excitation, or is
    /// not a voltage source. Both indicate the netlist was mutated between
    /// collection and excitation.
    PortSourceUnusable { source_name: String, reason: String },
}

impl std::fmt::Display for PortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aborted => write!(f, "S-parameter port preparation aborted"),
            Self::ResourceLimit(error) => error.fmt(f),
            Self::MissingTerminals { source_name } => write!(
                f,
                "S-parameter port source '{source_name}' must have positive and negative nodes"
            ),
            Self::InvalidReferenceImpedance { source_name, z0 } => write!(
                f,
                "S-parameter port source '{source_name}' has invalid z0 {z0}; \
                 expected a positive impedance"
            ),
            Self::NoPortsDeclared => write!(
                f,
                "S-parameter analysis requires voltage sources annotated with \
                 portnum=<n> [z0=<ohms>]"
            ),
            Self::NonDensePortNumbers {
                expected,
                found,
                source_name,
            } => write!(
                f,
                "S-parameter port numbers must be dense and unique starting at 1; \
                 expected {expected}, found {found} on '{source_name}'"
            ),
            Self::PortSourceUnusable {
                source_name,
                reason,
            } => write!(f, "S-parameter port '{source_name}' {reason}"),
        }
    }
}

impl std::error::Error for PortError {}

/// Materialize configured reference planes as terminated RF ports.
///
/// This is for a deck without authored RF ports. Every plane receives one
/// zero-DC generator behind its reference impedance; source and internal node
/// names cannot alias existing circuit names. Validation and cancellation
/// leave the supplied netlist unchanged. The resulting annotations are the
/// same ones consumed by the card-driven SP runner.
pub fn declare_ports_with_abort(
    netlist: &mut Netlist,
    configured: &[super::Port],
    abort: &dyn AbortSignal,
) -> Result<Vec<SParameterPort>, PortError> {
    if abort.is_aborted() {
        return Err(PortError::Aborted);
    }
    if configured.is_empty() {
        return Err(PortError::NoPortsDeclared);
    }
    match collect_ports(netlist) {
        Ok(ports) => return Err(PortError::PortSourceUnusable {
            source_name: ports[0].source_name.clone(),
            reason: "already declares RF ports; use those declarations instead of adding configured ports".into(),
        }),
        Err(PortError::NoPortsDeclared) => {}
        Err(error) => return Err(error),
    }
    let mut names = PortNames::new(netlist, abort)?;
    let ground = netlist.ground_policy();
    let canonical_node = |node: &str| ground.canonical_node(node.trim()).to_ascii_uppercase();
    for (index, port) in configured.iter().enumerate() {
        if abort.is_aborted() {
            return Err(PortError::Aborted);
        }
        let source_name = format!("__RSPICE_SP_PORT{}", index + 1);
        if port.number != index + 1 {
            return Err(PortError::NonDensePortNumbers {
                expected: index + 1,
                found: port.number,
                source_name,
            });
        }
        if !port.z0.is_finite() || port.z0 <= 0.0 {
            return Err(PortError::InvalidReferenceImpedance {
                source_name,
                z0: port.z0,
            });
        }
        if port.node_pos.trim().is_empty() || port.node_neg.trim().is_empty() {
            return Err(PortError::MissingTerminals { source_name });
        }
        names.nodes.insert(canonical_node(&port.node_pos));
        names.nodes.insert(canonical_node(&port.node_neg));
    }
    let mut additions = Vec::new();
    let mut ports = Vec::with_capacity(configured.len());
    for port in configured {
        let source_name = unique_name(
            &mut names.elements,
            &format!("__RSPICE_SP_PORT{}", port.number),
            abort,
        )?;
        let resistor_name = unique_name(
            &mut names.elements,
            &format!("__RSPICE_SP_Z0_{}", port.number),
            abort,
        )?;
        let internal = unique_name(
            &mut names.nodes,
            &format!("__RSPICE_SP_PORT{}_INT", port.number),
            abort,
        )?;
        let node_pos = canonical_node(&port.node_pos);
        let node_neg = canonical_node(&port.node_neg);
        additions.push(Element {
            name: source_name.clone(),
            nodes: vec![internal.clone(), node_neg.clone()],
            kind: ElementKind::VoltageSource(SourceSpec::RfPort {
                inner: Box::new(SourceSpec::Dc(0.0)),
                port: crate::netlist::SourceRfPort {
                    portnum: port.number,
                    z0: port.z0,
                    power: None,
                    frequency: None,
                    phase: None,
                    reference_plane: Some(node_pos.clone()),
                },
            }),
            provenance: ElementProvenance::Authored,
        });
        additions.push(reference_resistor(
            resistor_name,
            &source_name,
            &node_pos,
            internal,
            port.z0,
        ));
        ports.push(SParameterPort {
            number: port.number,
            source_name,
            node_pos,
            node_neg,
            z0: port.z0,
            realization: PortRealization::Thevenin,
        });
    }
    if abort.is_aborted() {
        return Err(PortError::Aborted);
    }
    netlist.elements.extend(additions);
    Ok(ports)
}

struct PortNames {
    elements: HashSet<String>,
    nodes: HashSet<String>,
}

impl PortNames {
    fn new(netlist: &Netlist, abort: &dyn AbortSignal) -> Result<Self, PortError> {
        Self::from_elements(netlist, &netlist.elements, abort)
    }

    fn from_elements(
        netlist: &Netlist,
        elements: &[Element],
        abort: &dyn AbortSignal,
    ) -> Result<Self, PortError> {
        let mut names = Self {
            elements: HashSet::new(),
            nodes: HashSet::new(),
        };
        for element in elements {
            if abort.is_aborted() {
                return Err(PortError::Aborted);
            }
            names.elements.insert(element.name.to_ascii_uppercase());
            for node in &element.nodes {
                names.nodes.insert(node.to_ascii_uppercase());
            }
        }
        for node in
            netlist
                .global_nodes
                .iter()
                .map(String::as_str)
                .chain(netlist.initial_conditions.iter().flat_map(|ic| {
                    std::iter::once(ic.node.as_str()).chain(ic.reference.as_deref())
                }))
                .chain(netlist.node_sets.iter().flat_map(|hint| {
                    std::iter::once(hint.node.as_str()).chain(hint.reference.as_deref())
                }))
        {
            if abort.is_aborted() {
                return Err(PortError::Aborted);
            }
            names.nodes.insert(node.to_ascii_uppercase());
        }
        Ok(names)
    }
}

fn unique_name(
    used: &mut HashSet<String>,
    base: &str,
    abort: &dyn AbortSignal,
) -> Result<String, PortError> {
    for index in 0_u64.. {
        if abort.is_aborted() {
            return Err(PortError::Aborted);
        }
        let candidate = if index == 0 {
            base.to_owned()
        } else {
            format!("{base}_{index}")
        };
        if used.insert(candidate.to_ascii_uppercase()) {
            return Ok(candidate);
        }
    }
    unreachable!("a circuit cannot occupy every u64-suffixed helper name")
}

fn reference_resistor(
    name: String,
    owner: &str,
    plane: &str,
    internal: String,
    z0: Value,
) -> Element {
    Element {
        name,
        nodes: vec![plane.to_owned(), internal],
        kind: ElementKind::Resistor {
            value: z0,
            value_expr: None,
            model: None,
            instance_params: Vec::new(),
            deferred_params: Vec::new(),
        },
        provenance: ElementProvenance::GeneratedPassiveHelper {
            owner: owner.to_owned(),
            role: crate::netlist::GeneratedPassiveHelperRole::SeriesResistance,
        },
    }
}

/// Collect and validate every `portnum`-annotated voltage source, in port order.
pub fn collect_ports(netlist: &Netlist) -> Result<Vec<SParameterPort>, PortError> {
    collect_element_ports(&netlist.elements, &NoAbort)
}

/// Find a lowered port's resistor and external plane from its ownership,
/// independently of generated names or resistor orientation.
pub(crate) fn reference_impedance_helper<'a>(
    elements: &'a [Element],
    source: &Element,
    z0: Value,
    abort: &dyn AbortSignal,
) -> Result<Option<(&'a Element, &'a str)>, PortError> {
    if source.nodes.len() < 2 {
        return Err(PortError::MissingTerminals {
            source_name: source.name.clone(),
        });
    }
    let mut found = None;
    for helper in elements {
        if abort.is_aborted() {
            return Err(PortError::Aborted);
        }
        if !matches!(&helper.provenance, ElementProvenance::GeneratedPassiveHelper {
            owner, role: crate::netlist::GeneratedPassiveHelperRole::SeriesResistance,
        } if owner.eq_ignore_ascii_case(&source.name))
        {
            continue;
        }
        let plane = if helper.nodes.len() == 2 {
            if helper.nodes[0].eq_ignore_ascii_case(&source.nodes[0]) {
                Some(helper.nodes[1].as_str())
            } else if helper.nodes[1].eq_ignore_ascii_case(&source.nodes[0]) {
                Some(helper.nodes[0].as_str())
            } else {
                None
            }
        } else {
            None
        };
        if found.is_some()
            || plane.is_none()
            || !matches!(&helper.kind, ElementKind::Resistor { value, .. } if *value == z0)
        {
            return Err(PortError::PortSourceUnusable {
                source_name: source.name.clone(),
                reason: "has an invalid reference-impedance helper".into(),
            });
        }
        found = plane.map(|plane| (helper, plane));
    }
    Ok(found)
}

fn collect_element_ports(
    elements: &[Element],
    abort: &dyn AbortSignal,
) -> Result<Vec<SParameterPort>, PortError> {
    let mut ports = Vec::new();
    for element in elements {
        if abort.is_aborted() {
            return Err(PortError::Aborted);
        }
        let ElementKind::VoltageSource(spec) = &element.kind else {
            continue;
        };
        let Some(port) = spec.rf_port() else {
            continue;
        };
        if element.nodes.len() < 2 {
            return Err(PortError::MissingTerminals {
                source_name: element.name.clone(),
            });
        }
        if !port.z0.is_finite() || port.z0 <= 0.0 {
            return Err(PortError::InvalidReferenceImpedance {
                source_name: element.name.clone(),
                z0: port.z0,
            });
        }
        // A Thevenin port declares the plane it feeds, because the generator's
        // own terminal is the far side of the reference resistor.
        let (node_pos, realization) = match port.reference_plane.as_ref() {
            Some(plane) => (plane.clone(), PortRealization::Thevenin),
            None => {
                // Lowered ngspice annotations retain their waveform convention
                // in SourceRfPort. The owned helper identifies the true plane
                // without mistaking its internal generator node for a new port.
                reference_impedance_helper(elements, element, port.z0, abort)?.map_or_else(
                    || (element.nodes[0].clone(), PortRealization::Ideal),
                    |(_, plane)| (plane.to_owned(), PortRealization::Thevenin),
                )
            }
        };
        ports.push(SParameterPort {
            number: port.portnum,
            source_name: element.name.clone(),
            node_pos,
            node_neg: element.nodes[1].clone(),
            z0: port.z0,
            realization,
        });
    }

    if ports.is_empty() {
        return Err(PortError::NoPortsDeclared);
    }

    ports.sort_by_key(|port| port.number);
    for (index, port) in ports.iter().enumerate() {
        let expected = index + 1;
        if port.number != expected {
            return Err(PortError::NonDensePortNumbers {
                expected,
                found: port.number,
                source_name: port.source_name.clone(),
            });
        }
    }
    Ok(ports)
}

/// Give every port a real reference impedance, so one extraction serves both
/// declaration styles.
///
/// The same lowering runs during ordinary circuit construction: RF source
/// impedance affects DC, AC, transient and noise as well as scattering.
///
/// Returns the ports restated as Thevenin, in the same order.
pub fn normalize_ports(
    netlist: &mut Netlist,
    ports: &[SParameterPort],
) -> Result<Vec<SParameterPort>, PortError> {
    let mut names = PortNames::new(netlist, &NoAbort)?;
    normalize_port_elements(&mut netlist.elements, ports, &mut names, &NoAbort)
}

/// Expand concise RF annotations after hierarchy and parameter resolution.
pub(crate) fn materialize_rf_ports(
    netlist: &Netlist,
    elements: &mut Vec<Element>,
    max_elements: usize,
    abort: &dyn AbortSignal,
) -> Result<(), PortError> {
    let ports = match collect_element_ports(elements, abort) {
        Ok(ports) => ports,
        Err(PortError::NoPortsDeclared) => return Ok(()),
        Err(error) => return Err(error),
    };
    let additions = ports
        .iter()
        .filter(|port| port.realization == PortRealization::Ideal)
        .count();
    if additions == 0 {
        return Ok(());
    }
    ResourceLimitError::ensure(
        ResourceKind::FlattenedElements,
        elements.len().saturating_add(additions),
        max_elements,
    )
    .map_err(PortError::ResourceLimit)?;
    let mut names = PortNames::from_elements(netlist, elements, abort)?;
    normalize_port_elements(elements, &ports, &mut names, abort)?;
    Ok(())
}

fn normalize_port_elements(
    elements: &mut Vec<Element>,
    ports: &[SParameterPort],
    names: &mut PortNames,
    abort: &dyn AbortSignal,
) -> Result<Vec<SParameterPort>, PortError> {
    let mut normalized = Vec::with_capacity(ports.len());
    for port in ports {
        if abort.is_aborted() {
            return Err(PortError::Aborted);
        }
        if port.realization == PortRealization::Thevenin {
            normalized.push(port.clone());
            continue;
        }

        let internal_node = unique_name(
            &mut names.nodes,
            &format!("__RSPICE_SP_{}_PORT", port.source_name.to_ascii_uppercase()),
            abort,
        )?;
        let resistor_name = unique_name(
            &mut names.elements,
            &format!("__RSPICE_SP_{}_Z0", port.source_name.to_ascii_uppercase()),
            abort,
        )?;

        let element = elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(&port.source_name))
            .ok_or_else(|| PortError::PortSourceUnusable {
                source_name: port.source_name.clone(),
                reason: "disappeared from the netlist".to_string(),
            })?;
        if !matches!(element.kind, ElementKind::VoltageSource(_)) {
            return Err(PortError::PortSourceUnusable {
                source_name: port.source_name.clone(),
                reason: "is not a voltage source".to_string(),
            });
        }
        if element.nodes.len() < 2 {
            return Err(PortError::MissingTerminals {
                source_name: port.source_name.clone(),
            });
        }
        // The source retreats behind the new resistor; the plane keeps its
        // name, so everything already measuring this port still measures it.
        element.nodes[0] = internal_node.clone();
        elements.push(reference_resistor(
            resistor_name,
            &port.source_name,
            &port.node_pos,
            internal_node,
            port.z0,
        ));

        normalized.push(SParameterPort {
            realization: PortRealization::Thevenin,
            ..port.clone()
        });
    }
    Ok(normalized)
}

/// Rewrite a source's AC excitation to `magnitude` at zero phase, preserving
/// everything else the spec carries.
fn replace_source_ac(spec: &mut SourceSpec, magnitude: Value) {
    let current = std::mem::replace(spec, SourceSpec::Dc(0.0));
    *spec = current.with_ac(magnitude, 0.0);
}

/// Drive exactly one port with a unit AC excitation and silence every other
/// source in the deck.
///
/// Zeroing *all* sources first, not just the port sources, is deliberate: a
/// stray independent source left driving the circuit would superimpose its own
/// response onto the extracted column and corrupt that column of `Y`.
///
/// `excited_port` is a zero-based index into `ports`.
pub fn set_excitations(
    netlist: &mut Netlist,
    ports: &[SParameterPort],
    excited_port: usize,
) -> Result<(), PortError> {
    for element in &mut netlist.elements {
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                replace_source_ac(spec, 0.0);
            }
            _ => {}
        }
    }

    for (index, port) in ports.iter().enumerate() {
        let element = netlist
            .elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(&port.source_name))
            .ok_or_else(|| PortError::PortSourceUnusable {
                source_name: port.source_name.clone(),
                reason: "disappeared from the netlist".to_string(),
            })?;
        let ElementKind::VoltageSource(spec) = &mut element.kind else {
            return Err(PortError::PortSourceUnusable {
                source_name: port.source_name.clone(),
                reason: "is not a voltage source".to_string(),
            });
        };
        replace_source_ac(spec, if index == excited_port { 1.0 } else { 0.0 });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Netlist {
        Netlist::parse(source).expect("deck parses")
    }

    const TWO_PORT: &str = "* two port\n\
                            V1 p1 0 DC 0 AC 1 portnum 1 z0 50\n\
                            R1 p1 p2 50\n\
                            V2 p2 0 DC 0 AC 0 portnum 2 z0 75\n\
                            .end\n";

    #[test]
    fn collects_ports_in_declared_order_with_terminals_and_impedances() {
        let ports = collect_ports(&parse(TWO_PORT)).expect("ports collect");

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].number, 1);
        assert_eq!(ports[0].source_name, "V1");
        // Node names arrive canonicalized to upper case by the parser.
        assert_eq!(ports[0].node_pos, "P1");
        assert_eq!(ports[0].node_neg, "0");
        assert_eq!(ports[0].z0, 50.0);
        assert_eq!(ports[1].source_name, "V2");
        assert_eq!(ports[1].z0, 75.0);
    }

    #[test]
    fn ports_are_sorted_by_number_not_by_deck_position() {
        let deck = "* reversed\n\
                    V2 p2 0 DC 0 AC 0 portnum 2 z0 50\n\
                    R1 p1 p2 50\n\
                    V1 p1 0 DC 0 AC 1 portnum 1 z0 50\n\
                    .end\n";
        let ports = collect_ports(&parse(deck)).expect("ports collect");
        assert_eq!(
            ports
                .iter()
                .map(|p| p.source_name.as_str())
                .collect::<Vec<_>>(),
            ["V1", "V2"]
        );
    }

    /// Xyce's `P` element declares a port too, and the plane it declares is its
    /// own terminal — not the generator node hidden behind the reference
    /// resistor the parser lowers it to.
    #[test]
    fn xyce_port_elements_declare_ports_at_their_own_terminals() {
        let deck = "* p elements\n\
                    P1 p1 0 PORT=1 Z0=50 AC 1\n\
                    R1 p1 p2 25\n\
                    P2 p2 0 PORT=2 Z0=75\n\
                    .end\n";
        let ports = collect_ports(&parse(deck)).expect("ports collect");

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].source_name, "P1");
        assert_eq!(ports[0].node_pos, "P1");
        assert_eq!(ports[0].z0, 50.0);
        assert_eq!(ports[0].realization, PortRealization::Thevenin);
        // A port with no source tokens is still a port: it is terminated, not
        // absent, and S-parameters are measured at every port including the
        // ones that are never driven.
        assert_eq!(ports[1].source_name, "P2");
        assert_eq!(ports[1].node_pos, "P2");
        assert_eq!(ports[1].z0, 75.0);
        assert_eq!(ports[1].realization, PortRealization::Thevenin);
    }

    /// An annotated ideal source is at its own reference plane, so it must not
    /// be mistaken for the Thevenin form.
    #[test]
    fn annotated_sources_stay_ideal_ports() {
        let ports = collect_ports(&parse(TWO_PORT)).expect("ports collect");
        assert!(
            ports
                .iter()
                .all(|port| port.realization == PortRealization::Ideal)
        );
    }

    #[test]
    fn a_deck_without_annotations_is_rejected() {
        let deck = "* no ports\nV1 a 0 AC 1\nR1 a 0 50\n.end\n";
        assert_eq!(collect_ports(&parse(deck)), Err(PortError::NoPortsDeclared));
    }

    #[test]
    fn sparse_port_numbering_is_rejected_rather_than_renumbered() {
        let deck = "* gap at 2\n\
                    V1 p1 0 DC 0 AC 1 portnum 1 z0 50\n\
                    R1 p1 p2 50\n\
                    V2 p2 0 DC 0 AC 0 portnum 3 z0 50\n\
                    .end\n";
        assert_eq!(
            collect_ports(&parse(deck)),
            Err(PortError::NonDensePortNumbers {
                expected: 2,
                found: 3,
                source_name: "V2".to_string(),
            })
        );
    }

    /// Normalization moves an annotated ideal source behind a real resistor,
    /// leaving the reference plane where every other reference to the port
    /// already points.
    #[test]
    fn normalization_gives_an_annotated_port_a_real_reference_impedance() {
        let mut netlist = parse(TWO_PORT);
        let ports = collect_ports(&netlist).expect("ports collect");
        let normalized = normalize_ports(&mut netlist, &ports).expect("normalizes");

        assert!(
            normalized
                .iter()
                .all(|port| port.realization == PortRealization::Thevenin)
        );
        // The planes are untouched; only what sits behind them changed.
        assert_eq!(normalized[0].node_pos, "P1");
        assert_eq!(normalized[1].node_pos, "P2");

        let source = netlist
            .elements
            .iter()
            .find(|element| element.name == "V1")
            .expect("port source survives");
        assert_eq!(source.nodes[0], "__RSPICE_SP_V1_PORT");

        let resistor = netlist
            .elements
            .iter()
            .find(|element| element.name == "__RSPICE_SP_V1_Z0")
            .expect("reference impedance is now a component");
        match &resistor.kind {
            ElementKind::Resistor { value, .. } => assert_eq!(*value, 50.0),
            other => panic!("expected a Z0 resistor, got {other:?}"),
        }
        assert_eq!(
            resistor.nodes,
            vec!["P1".to_string(), "__RSPICE_SP_V1_PORT".to_string()]
        );
    }

    /// A `P` element is already Thevenin, so normalization must leave it be
    /// rather than stacking a second reference impedance in front of it.
    #[test]
    fn normalization_leaves_an_already_physical_port_alone() {
        let deck = "* p element\n\
                    P1 p1 0 PORT=1 Z0=50 AC 1\n\
                    R1 p1 0 25\n\
                    .end\n";
        let mut netlist = parse(deck);
        let ports = collect_ports(&netlist).expect("ports collect");
        let before = netlist.elements.len();

        let normalized = normalize_ports(&mut netlist, &ports).expect("normalizes");

        assert_eq!(netlist.elements.len(), before);
        assert_eq!(normalized, ports);
    }

    #[test]
    fn excitation_drives_one_port_and_silences_every_other_source() {
        let deck = "* stray source\n\
                    V1 p1 0 DC 0 AC 1 portnum 1 z0 50\n\
                    R1 p1 p2 50\n\
                    V2 p2 0 DC 0 AC 0 portnum 2 z0 50\n\
                    I1 p2 0 AC 3\n\
                    .end\n";
        let mut netlist = parse(deck);
        let ports = collect_ports(&netlist).expect("ports collect");

        set_excitations(&mut netlist, &ports, 1).expect("excitation applies");

        /// Unwrap the RF-port and distortion wrappers, then read the AC term.
        fn ac_magnitude_of(spec: &SourceSpec) -> Value {
            match spec {
                SourceSpec::RfPort { inner, .. } | SourceSpec::Distortion { inner, .. } => {
                    ac_magnitude_of(inner)
                }
                SourceSpec::Ac { magnitude, .. }
                | SourceSpec::DcAc {
                    ac_magnitude: magnitude,
                    ..
                }
                | SourceSpec::DcAcTransient {
                    ac_magnitude: magnitude,
                    ..
                } => *magnitude,
                _ => 0.0,
            }
        }

        let ac_magnitude = |name: &str| -> Value {
            let element = netlist
                .elements
                .iter()
                .find(|e| e.name.eq_ignore_ascii_case(name))
                .expect("element present");
            let (ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)) =
                &element.kind
            else {
                panic!("{name} is not a source");
            };
            ac_magnitude_of(spec)
        };

        assert_eq!(ac_magnitude("V1"), 0.0);
        assert_eq!(ac_magnitude("V2"), 1.0);
        // The stray current source must be silenced, or its response would be
        // superimposed on the extracted admittance column.
        assert_eq!(ac_magnitude("I1"), 0.0);
    }
}
