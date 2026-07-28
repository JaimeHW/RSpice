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

use crate::Value;
use crate::netlist::{ElementKind, Netlist, SourceSpec};

/// One `.SP` port resolved from a netlist annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct SParameterPort {
    /// Declared `portnum`, one-based. Guaranteed dense after collection.
    pub number: usize,
    /// Name of the voltage source carrying the annotation.
    pub source_name: String,
    /// Positive terminal node.
    pub node_pos: String,
    /// Negative terminal node.
    pub node_neg: String,
    /// Reference impedance in ohms; always positive and finite.
    pub z0: Value,
}

/// Why a deck's `.SP` port declarations could not be used.
#[derive(Debug, Clone, PartialEq)]
pub enum PortError {
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

/// Collect and validate every `portnum`-annotated voltage source, in port order.
pub fn collect_ports(netlist: &Netlist) -> Result<Vec<SParameterPort>, PortError> {
    let mut ports = Vec::new();
    for element in &netlist.elements {
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
        ports.push(SParameterPort {
            number: port.portnum,
            source_name: element.name.clone(),
            node_pos: element.nodes[0].clone(),
            node_neg: element.nodes[1].clone(),
            z0: port.z0,
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
