//! Resolved symbol geometry and pin diagnostics, independent of library lookup.

use std::collections::HashSet;

use rspice_design_model::Point;
use rspice_design_model::port::{PortDirection, PortSpec};
use rspice_design_model::symbol_pin::SymbolPinSide;

use crate::symbol::{SymbolDocument, SymbolShape};

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

    /// Where a pin's name is drawn: inside the body, clear of the outline and
    /// of the pin's lead.
    ///
    /// A symbol that draws its leads as artwork (an imported LTspice or SVG
    /// symbol) has a body that reaches out to every terminal, so the lead the
    /// renderer would draw has no length and the name would start on the
    /// artwork's lead. The name steps past the end of that lead instead, as a
    /// generated symbol's steps past the end of the lead it draws.
    pub fn pin_label_anchor(&self, pin: &ResolvedSymbolPin) -> Point {
        let anchor = crate::symbol_generation::pin_label_anchor(
            pin.offset,
            pin.side,
            self.document.drawn_body_bounds(),
        );
        let step = crate::symbol_generation::inward_step(pin.side);
        let inward =
            |point: Point| (point.x - pin.offset.x) * step.x + (point.y - pin.offset.y) * step.y;
        let past_lead = artwork_lead_end(&self.document, pin.offset, pin.side).map(|end| {
            Point::new(
                end.x + step.x * crate::symbol_generation::GENERATED_LABEL_GUTTER,
                end.y + step.y * crate::symbol_generation::GENERATED_LABEL_GUTTER,
            )
        });
        self.effective_point(match past_lead {
            Some(past) if inward(past) > inward(anchor) => past,
            _ => anchor,
        })
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

/// Where a lead the artwork draws from `terminal` ends: the inner end of the
/// straight run of body lines that leaves the terminal along its pin's axis,
/// toward the body. `None` when the artwork draws no lead there, or when the
/// run reaches past the middle of the body, which makes it an outline edge
/// the terminal happens to sit on rather than a lead.
fn artwork_lead_end(
    document: &SymbolDocument,
    terminal: Point,
    side: SymbolPinSide,
) -> Option<Point> {
    let step = crate::symbol_generation::inward_step(side);
    let inward = |point: Point| (point.x - terminal.x) * step.x + (point.y - terminal.y) * step.y;
    let on_axis = |point: Point| match side {
        SymbolPinSide::Left | SymbolPinSide::Right => point.y == terminal.y,
        SymbolPinSide::Top | SymbolPinSide::Bottom => point.x == terminal.x,
    };
    let segments = document
        .body
        .iter()
        .flat_map(|shape| match shape {
            SymbolShape::Polyline { points, closed } => {
                let closing =
                    (*closed && points.len() > 2).then(|| (points[points.len() - 1], points[0]));
                points
                    .windows(2)
                    .map(|pair| (pair[0], pair[1]))
                    .chain(closing)
                    .collect::<Vec<_>>()
            }
            _ => Vec::new(),
        })
        .filter(|(from, to)| on_axis(*from) && on_axis(*to))
        .collect::<Vec<_>>();
    let mut reach = terminal;
    while let Some(next) = segments
        .iter()
        .filter_map(|&(from, to)| {
            if from == reach {
                Some(to)
            } else if to == reach {
                Some(from)
            } else {
                None
            }
        })
        .filter(|point| inward(*point) > inward(reach))
        .max_by_key(|point| inward(*point))
    {
        reach = next;
    }
    let (min, max) = document.drawn_body_bounds()?;
    let middle = Point::new(min.x / 2 + max.x / 2, min.y / 2 + max.y / 2);
    (reach != terminal && inward(reach) <= inward(middle)).then_some(reach)
}

fn port_name_set(ports: &[PortSpec]) -> HashSet<String> {
    ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::SymbolPin;

    /// An 80 x 60 box whose body draws its own 10-wu leads out to IN
    /// (-40, 0), OUT (40, 0) and BIAS (0, 40), as an imported symbol does.
    fn artwork_leads() -> SymbolDocument {
        let line = |from: (i32, i32), to: (i32, i32)| SymbolShape::Polyline {
            points: vec![Point::new(from.0, from.1), Point::new(to.0, to.1)],
            closed: false,
        };
        SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
                SymbolPin::new("BIAS", PortDirection::In, Some(Point::new(0, 40))),
            ],
            body: vec![
                SymbolShape::Polyline {
                    points: vec![
                        Point::new(-30, -30),
                        Point::new(30, -30),
                        Point::new(30, 30),
                        Point::new(-30, 30),
                    ],
                    closed: true,
                },
                line((-40, 0), (-30, 0)),
                line((40, 0), (30, 0)),
                line((0, 30), (0, 40)),
            ],
            ..SymbolDocument::default()
        }
    }

    /// A pin name steps past a lead the artwork draws, a gutter inside the
    /// outline, rather than starting on the lead at the terminal.
    #[test]
    fn a_pin_name_steps_past_a_lead_the_artwork_draws() {
        let symbol = ResolvedCellSymbol::from_authored_document(artwork_leads(), &[]);
        let anchors = symbol
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), symbol.pin_label_anchor(pin)))
            .collect::<Vec<_>>();
        assert_eq!(
            anchors,
            [
                ("IN", Point::new(-27, 0)),
                ("OUT", Point::new(27, 0)),
                ("BIAS", Point::new(0, 27)),
            ]
        );
    }

    /// A body edge a terminal happens to sit on is not a lead: a pin at a
    /// box's corner keeps the anchor the outline gives it.
    #[test]
    fn an_outline_edge_through_a_terminal_is_not_a_lead() {
        let mut document = artwork_leads();
        document.pins = vec![SymbolPin::new(
            "C",
            PortDirection::In,
            Some(Point::new(-30, -30)),
        )];
        let symbol = ResolvedCellSymbol::from_authored_document(document.clone(), &[]);
        let pin = symbol.connectable_pins().next().expect("one pin");
        assert_eq!(
            symbol.pin_label_anchor(pin),
            crate::symbol_generation::pin_label_anchor(
                pin.offset,
                pin.side,
                document.drawn_body_bounds()
            )
        );
    }
}
