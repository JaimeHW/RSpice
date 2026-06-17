//! Persistent symbol-view document state.
//!
//! A symbol view is the public contract of a cell: body artwork, label
//! anchors, and the pins that a parent schematic can wire to. The paired
//! schematic supplies the canonical port list when it exists; this document
//! stores the user's symbol-specific placement and artwork.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{Point, PortDirection, PortSpec, View};

/// Metadata key used by `View::metadata` for the symbol document JSON.
pub const SYMBOL_DOCUMENT_METADATA_KEY: &str = "rspice.symbol.document.v1";

/// Terminal-grid spacing in schematic coordinate units.
///
/// The design spec shows this as a 40 px editor lattice; in the Rust model
/// one schematic unit renders as four px at the symbol editor's 100% scale.
pub const SYMBOL_TERMINAL_GRID: i32 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolPin {
    pub name: String,
    pub direction: PortDirection,
    pub position: Option<Point>,
}

impl SymbolPin {
    pub fn new(name: impl Into<String>, direction: PortDirection, position: Option<Point>) -> Self {
        Self {
            name: name.into(),
            direction,
            position,
        }
    }

    pub fn terminal_on_grid(&self) -> bool {
        self.position.is_some_and(|point| {
            point.x % SYMBOL_TERMINAL_GRID == 0 && point.y % SYMBOL_TERMINAL_GRID == 0
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolShape {
    Polyline {
        points: Vec<Point>,
        closed: bool,
    },
    Circle {
        center: Point,
        radius: i32,
    },
    Arc {
        center: Point,
        radius: i32,
        start_degrees: i32,
        sweep_degrees: i32,
    },
    Arrow {
        tip: Point,
        rotation_quarters: i32,
    },
    Dot {
        center: Point,
        radius: i32,
    },
}

impl SymbolShape {
    pub fn translate(&mut self, delta: Point) {
        self.map_points(|point| point + delta);
    }

    pub fn rotate_cw(&mut self) {
        self.map_points(|point| Point::new(-point.y, point.x));
        match self {
            SymbolShape::Arc { start_degrees, .. } => {
                *start_degrees += 90;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters += 1;
            }
            _ => {}
        }
    }

    pub fn mirror_h(&mut self) {
        self.map_points(|point| Point::new(-point.x, point.y));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees = 180 - *start_degrees - *sweep_degrees;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (2 - *rotation_quarters).rem_euclid(4);
            }
            _ => {}
        }
    }

    pub fn mirror_v(&mut self) {
        self.map_points(|point| Point::new(point.x, -point.y));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees = -*start_degrees - *sweep_degrees;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (-*rotation_quarters).rem_euclid(4);
            }
            _ => {}
        }
    }

    fn map_points(&mut self, transform: impl Fn(Point) -> Point) {
        match self {
            SymbolShape::Polyline { points, .. } => {
                for point in points {
                    *point = transform(*point);
                }
            }
            SymbolShape::Circle { center, .. }
            | SymbolShape::Arc { center, .. }
            | SymbolShape::Dot { center, .. } => {
                *center = transform(*center);
            }
            SymbolShape::Arrow { tip, .. } => {
                *tip = transform(*tip);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolLabelAnchors {
    pub name: Point,
    pub value: Point,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolDocument {
    pub pins: Vec<SymbolPin>,
    pub body: Vec<SymbolShape>,
    pub origin: Point,
    pub name_anchor: Point,
    pub value_anchor: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinSummary {
    Match,
    Unplaced(usize),
    Orphaned(usize),
    NoSchematic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinFindingKind {
    UnplacedPin,
    OrphanedPin,
    PinOffGrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinFinding {
    pub kind: PinFindingKind,
    pub pin_name: String,
}

impl Default for SymbolDocument {
    fn default() -> Self {
        Self {
            pins: Vec::new(),
            body: Vec::new(),
            origin: Point::origin(),
            name_anchor: Point::new(-20, -40),
            value_anchor: Point::new(-20, 40),
        }
    }
}

impl SymbolDocument {
    pub fn generated_from_ports(ports: &[PortSpec]) -> Self {
        let generated = super::generate_symbol(ports);
        let body_half_width =
            (generated.width / 2 - SYMBOL_TERMINAL_GRID).max(SYMBOL_TERMINAL_GRID);
        let body_half_height = (generated.height / 2).max(SYMBOL_TERMINAL_GRID * 2);
        let pins = generated
            .pins
            .into_iter()
            .map(|pin| SymbolPin::new(pin.name, pin.direction, Some(pin.offset)))
            .collect();
        Self {
            pins,
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-body_half_width, -body_half_height),
                    Point::new(body_half_width, -body_half_height),
                    Point::new(body_half_width, body_half_height),
                    Point::new(-body_half_width, body_half_height),
                ],
                closed: true,
            }],
            origin: Point::origin(),
            name_anchor: Point::new(
                -body_half_width,
                -body_half_height - SYMBOL_TERMINAL_GRID * 2,
            ),
            value_anchor: Point::new(
                -body_half_width,
                body_half_height + SYMBOL_TERMINAL_GRID * 2,
            ),
        }
    }

    pub fn load_from_view(view: &View) -> Result<Self, String> {
        let Some(raw) = view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY) else {
            return Ok(Self::default());
        };
        serde_json::from_str(raw).map_err(|err| format!("Invalid symbol metadata: {err}"))
    }

    pub fn store_in_view(&self, view: &mut View) -> Result<(), String> {
        let raw = serde_json::to_string(self)
            .map_err(|err| format!("Could not serialize symbol metadata: {err}"))?;
        view.metadata
            .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), raw);
        view.modified = true;
        Ok(())
    }

    pub fn pin(&self, name: &str) -> Option<&SymbolPin> {
        self.pins
            .iter()
            .find(|pin| pin.name.eq_ignore_ascii_case(name))
    }

    pub fn pin_mut(&mut self, name: &str) -> Option<&mut SymbolPin> {
        self.pins
            .iter_mut()
            .find(|pin| pin.name.eq_ignore_ascii_case(name))
    }

    pub fn reconcile_ports(&mut self, ports: &[PortSpec]) {
        let mut existing = HashSet::new();
        for port in ports {
            if let Some(pin) = self.pin_mut(&port.name) {
                pin.name = port.name.clone();
                pin.direction = port.direction;
            } else {
                self.pins
                    .push(SymbolPin::new(&port.name, port.direction, None));
            }
            existing.insert(port.name.to_ascii_lowercase());
        }

        self.pins.sort_by_key(|pin| {
            ports
                .iter()
                .position(|port| port.name.eq_ignore_ascii_case(&pin.name))
                .unwrap_or(ports.len())
        });
    }

    pub fn pin_summary(&self, ports: &[PortSpec]) -> PinSummary {
        if ports.is_empty() {
            return PinSummary::NoSchematic;
        }
        let port_names = port_name_set(ports);
        let unplaced = ports
            .iter()
            .filter(|port| {
                self.pin(&port.name)
                    .is_none_or(|pin| pin.position.is_none())
            })
            .count();
        if unplaced > 0 {
            return PinSummary::Unplaced(unplaced);
        }
        let orphaned = self
            .pins
            .iter()
            .filter(|pin| !port_names.contains(&pin.name.to_ascii_lowercase()))
            .count();
        if orphaned > 0 {
            return PinSummary::Orphaned(orphaned);
        }
        PinSummary::Match
    }

    pub fn pin_findings(&self, ports: &[PortSpec]) -> Vec<PinFinding> {
        let port_names = port_name_set(ports);
        let mut findings = Vec::new();
        for port in ports {
            if self
                .pin(&port.name)
                .is_none_or(|pin| pin.position.is_none())
            {
                findings.push(PinFinding {
                    kind: PinFindingKind::UnplacedPin,
                    pin_name: port.name.clone(),
                });
            }
        }
        for pin in &self.pins {
            if !port_names.contains(&pin.name.to_ascii_lowercase()) && !ports.is_empty() {
                findings.push(PinFinding {
                    kind: PinFindingKind::OrphanedPin,
                    pin_name: pin.name.clone(),
                });
            }
            if pin.position.is_some() && !pin.terminal_on_grid() {
                findings.push(PinFinding {
                    kind: PinFindingKind::PinOffGrid,
                    pin_name: pin.name.clone(),
                });
            }
        }
        findings
    }

    pub fn labels(&self) -> SymbolLabelAnchors {
        SymbolLabelAnchors {
            name: self.name_anchor,
            value: self.value_anchor,
        }
    }
}

fn port_name_set(ports: &[PortSpec]) -> HashSet<String> {
    ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect()
}
