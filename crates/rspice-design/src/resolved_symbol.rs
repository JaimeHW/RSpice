//! Resolved symbol geometry and pin diagnostics, independent of library lookup.

use std::collections::HashSet;

use rspice_design_model::Point;
use rspice_design_model::port::{PortDirection, PortSpec};
use rspice_design_model::symbol_pin::SymbolPinSide;

use crate::symbol::SymbolDocument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSymbolSource {
    Authored,
    Generated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSymbolIssueKind {
    UnplacedPin,
    OrphanedPin,
    PinOffGrid,
    InvalidMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSymbolIssue {
    pub kind: ResolvedSymbolIssueKind,
    pub pin_name: String,
    pub point: Option<Point>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSymbolPin {
    pub name: String,
    pub direction: PortDirection,
    /// Body edge the pin is drawn against. Its lead and its name both follow
    /// this, so neither can be inferred from the terminal coordinate alone.
    pub side: SymbolPinSide,
    pub offset: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCellSymbol {
    source: ResolvedSymbolSource,
    document: SymbolDocument,
    pins: Vec<ResolvedSymbolPin>,
    issues: Vec<ResolvedSymbolIssue>,
}

impl ResolvedCellSymbol {
    pub fn source(&self) -> ResolvedSymbolSource {
        self.source
    }

    pub fn document(&self) -> &SymbolDocument {
        &self.document
    }

    pub fn issues(&self) -> &[ResolvedSymbolIssue] {
        &self.issues
    }

    pub fn connectable_pins(&self) -> impl Iterator<Item = &ResolvedSymbolPin> {
        self.pins.iter()
    }

    pub fn effective_point(&self, point: Point) -> Point {
        point - self.document.origin
    }

    pub fn effective_pin_offset(&self, pin: &ResolvedSymbolPin) -> Point {
        self.effective_point(pin.offset)
    }

    /// Inner end of a pin's lead, where it meets the body outline.
    pub fn pin_lead_inner(&self, pin: &ResolvedSymbolPin) -> Point {
        self.effective_point(crate::symbol_generation::lead_inner(
            pin.offset,
            pin.side,
            self.document.drawn_body_bounds(),
        ))
    }

    /// Where a pin's name is drawn: inside the body, clear of the outline.
    pub fn pin_label_anchor(&self, pin: &ResolvedSymbolPin) -> Point {
        self.effective_point(crate::symbol_generation::pin_label_anchor(
            pin.offset,
            pin.side,
            self.document.drawn_body_bounds(),
        ))
    }

    pub fn from_authored_document(document: SymbolDocument, ports: &[PortSpec]) -> Self {
        let mut pins = Vec::new();
        let mut issues = Vec::new();

        if ports.is_empty() {
            for pin in &document.pins {
                match pin.position {
                    Some(offset) => pins.push(ResolvedSymbolPin {
                        name: pin.name.clone(),
                        direction: pin.direction,
                        side: document.pin_side(pin),
                        offset,
                    }),
                    None => issues.push(ResolvedSymbolIssue {
                        kind: ResolvedSymbolIssueKind::UnplacedPin,
                        pin_name: pin.name.clone(),
                        point: None,
                    }),
                }
            }
        } else {
            let port_names = port_name_set(ports);
            for port in ports {
                match document
                    .pin(&port.name)
                    .and_then(|pin| pin.position.map(|offset| (pin, offset)))
                {
                    Some((pin, offset)) => pins.push(ResolvedSymbolPin {
                        name: port.name.clone(),
                        direction: port.direction,
                        side: document.pin_side(pin),
                        offset,
                    }),
                    None => issues.push(ResolvedSymbolIssue {
                        kind: ResolvedSymbolIssueKind::UnplacedPin,
                        pin_name: port.name.clone(),
                        point: None,
                    }),
                }
            }

            for pin in document
                .pins
                .iter()
                .filter(|pin| !port_names.contains(&pin.name.to_ascii_lowercase()))
            {
                issues.push(ResolvedSymbolIssue {
                    kind: ResolvedSymbolIssueKind::OrphanedPin,
                    pin_name: pin.name.clone(),
                    point: pin.position,
                });
            }
        }

        for pin in document
            .pins
            .iter()
            .filter(|pin| pin.position.is_some() && !pin.terminal_on_grid())
        {
            issues.push(ResolvedSymbolIssue {
                kind: ResolvedSymbolIssueKind::PinOffGrid,
                pin_name: pin.name.clone(),
                point: pin.position,
            });
        }

        Self {
            source: ResolvedSymbolSource::Authored,
            document,
            pins,
            issues,
        }
    }

    pub fn from_generated(ports: &[PortSpec]) -> Self {
        let document = SymbolDocument::generated_from_ports(ports);
        let pins = ports
            .iter()
            .filter_map(|port| {
                document.pin(&port.name).and_then(|pin| {
                    pin.position.map(|offset| ResolvedSymbolPin {
                        name: port.name.clone(),
                        direction: port.direction,
                        side: document.pin_side(pin),
                        offset,
                    })
                })
            })
            .collect();

        Self {
            source: ResolvedSymbolSource::Generated,
            document,
            pins,
            issues: Vec::new(),
        }
    }

    pub fn from_invalid_metadata(ports: &[PortSpec], _message: String) -> Self {
        let mut resolved = Self::from_generated(ports);
        resolved.issues.push(ResolvedSymbolIssue {
            kind: ResolvedSymbolIssueKind::InvalidMetadata,
            pin_name: "symbol".to_owned(),
            point: None,
        });
        resolved
    }
}

fn port_name_set(ports: &[PortSpec]) -> HashSet<String> {
    ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect()
}
