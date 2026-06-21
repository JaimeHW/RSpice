use std::collections::{HashMap, HashSet};

use super::{
    Cell, CellViewRef, LibraryCellInstance, LibraryManager, Point, PortDirection, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SchematicState, SymbolDocument, View, ViewType,
};

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
    pub fn from_authored_document(document: SymbolDocument, ports: &[PortSpec]) -> Self {
        Self::from_authored(document, ports)
    }

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

    fn from_authored(document: SymbolDocument, ports: &[PortSpec]) -> Self {
        let mut pins = Vec::new();
        let mut issues = Vec::new();

        if ports.is_empty() {
            for pin in &document.pins {
                match pin.position {
                    Some(offset) => pins.push(ResolvedSymbolPin {
                        name: pin.name.clone(),
                        direction: pin.direction,
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
                match document.pin(&port.name).and_then(|pin| pin.position) {
                    Some(offset) => pins.push(ResolvedSymbolPin {
                        name: port.name.clone(),
                        direction: port.direction,
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

    fn from_generated(ports: &[PortSpec]) -> Self {
        let document = SymbolDocument::generated_from_ports(ports);
        let pins = ports
            .iter()
            .filter_map(|port| {
                document.pin(&port.name).and_then(|pin| {
                    pin.position.map(|offset| ResolvedSymbolPin {
                        name: port.name.clone(),
                        direction: port.direction,
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

    fn from_invalid_metadata(ports: &[PortSpec], _message: String) -> Self {
        let mut resolved = Self::from_generated(ports);
        resolved.issues.push(ResolvedSymbolIssue {
            kind: ResolvedSymbolIssueKind::InvalidMetadata,
            pin_name: "symbol".to_owned(),
            point: None,
        });
        resolved
    }
}

pub struct SymbolResolver<'a> {
    libraries: &'a LibraryManager,
    schematic_buffers: &'a HashMap<String, SchematicState>,
}

impl<'a> SymbolResolver<'a> {
    pub fn new(
        libraries: &'a LibraryManager,
        schematic_buffers: &'a HashMap<String, SchematicState>,
    ) -> Self {
        Self {
            libraries,
            schematic_buffers,
        }
    }

    pub fn resolve_binding(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        self.resolve_cell(
            &binding.library,
            &binding.cell,
            None,
            binding.interface(),
            PortPreference::ExplicitFirst,
        )
    }

    pub fn resolve_reference(&self, reference: &CellViewRef) -> Option<ResolvedCellSymbol> {
        self.resolve_cell(
            &reference.library,
            &reference.cell,
            Some(&reference.view),
            None,
            PortPreference::SchematicFirst,
        )
    }

    fn resolve_cell(
        &self,
        library_name: &str,
        cell_name: &str,
        requested_view: Option<&str>,
        fallback_ports: Option<Vec<PortSpec>>,
        port_preference: PortPreference,
    ) -> Option<ResolvedCellSymbol> {
        let cell = self
            .libraries
            .get_library(library_name)
            .and_then(|library| library.get_cell(cell_name));
        let symbol_view = cell.and_then(|cell| find_symbol_view(cell, requested_view));
        let ports = self.resolve_ports(
            library_name,
            cell_name,
            symbol_view,
            fallback_ports,
            port_preference,
        );

        match authored_document(symbol_view) {
            AuthoredDocument::Loaded(document) => {
                let ports = ports.unwrap_or_default();
                return Some(ResolvedCellSymbol::from_authored(document, &ports));
            }
            AuthoredDocument::Invalid(message) => {
                let ports = ports.unwrap_or_default();
                return Some(ResolvedCellSymbol::from_invalid_metadata(&ports, message));
            }
            AuthoredDocument::Missing => {}
        }

        ports.map(|ports| ResolvedCellSymbol::from_generated(&ports))
    }

    fn resolve_ports(
        &self,
        library_name: &str,
        cell_name: &str,
        symbol_view: Option<&View>,
        fallback_ports: Option<Vec<PortSpec>>,
        port_preference: PortPreference,
    ) -> Option<Vec<PortSpec>> {
        let explicit_ports = fallback_ports.filter(|ports| !ports.is_empty());
        let schematic_ports = || self.schematic_ports(library_name, cell_name);
        let legacy_ports = || {
            symbol_view
                .and_then(legacy_ports_from_view)
                .filter(|ports| !ports.is_empty())
        };

        match port_preference {
            PortPreference::ExplicitFirst => explicit_ports
                .or_else(schematic_ports)
                .or_else(legacy_ports),
            PortPreference::SchematicFirst => {
                schematic_ports().or(explicit_ports).or_else(legacy_ports)
            }
        }
    }

    fn schematic_ports(&self, library_name: &str, cell_name: &str) -> Option<Vec<PortSpec>> {
        let reference = CellViewRef::new(library_name, cell_name, "schematic");
        self.schematic_buffers
            .get(&reference.key())
            .map(SchematicState::interface_ports)
            .filter(|ports| !ports.is_empty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortPreference {
    ExplicitFirst,
    SchematicFirst,
}

enum AuthoredDocument {
    Missing,
    Loaded(SymbolDocument),
    Invalid(String),
}

fn authored_document(view: Option<&View>) -> AuthoredDocument {
    let Some(view) = view else {
        return AuthoredDocument::Missing;
    };
    if !view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY) {
        return AuthoredDocument::Missing;
    }
    match SymbolDocument::load_from_view(view) {
        Ok(document) => AuthoredDocument::Loaded(document),
        Err(message) => AuthoredDocument::Invalid(message),
    }
}

fn find_symbol_view<'a>(cell: &'a Cell, requested_view: Option<&str>) -> Option<&'a View> {
    requested_view
        .and_then(|name| cell.get_view(name))
        .filter(|view| view.view_type == ViewType::Symbol)
        .or_else(|| {
            cell.get_view("symbol")
                .filter(|view| view.view_type == ViewType::Symbol)
        })
        .or_else(|| {
            cell.views_sorted()
                .into_iter()
                .find(|view| view.view_type == ViewType::Symbol)
        })
}

fn legacy_ports_from_view(view: &View) -> Option<Vec<PortSpec>> {
    view.metadata.get("ports").map(|encoded| {
        encoded
            .split_whitespace()
            .filter_map(|entry| {
                let (name, direction) = entry.split_once(':')?;
                let name = name.trim();
                if name.is_empty() {
                    return None;
                }
                Some(PortSpec {
                    name: name.to_owned(),
                    direction: PortDirection::parse(direction),
                })
            })
            .collect()
    })
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
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, LibraryManager, Point,
        PortDirection, SchematicState, SymbolDocument, SymbolPin, SymbolShape, View, ViewType,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> crate::state::PortSpec {
        crate::state::PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn library_with_amp(
        symbol: Option<SymbolDocument>,
    ) -> (LibraryManager, HashMap<String, SchematicState>) {
        library_with_amp_and_ports(
            symbol,
            &[("IN", PortDirection::In), ("OUT", PortDirection::Out)],
        )
    }

    fn library_with_amp_and_ports(
        symbol: Option<SymbolDocument>,
        ports: &[(&str, PortDirection)],
    ) -> (LibraryManager, HashMap<String, SchematicState>) {
        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        if let Some(document) = symbol {
            document
                .store_in_view(&mut symbol_view)
                .expect("symbol stores");
        }
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut schematic = SchematicState::default();
        for (idx, (name, _direction)) in ports.iter().enumerate() {
            let id = schematic.add_component(ComponentType::Port, Point::new(idx as i32 * 40, 0));
            schematic
                .components
                .iter_mut()
                .find(|c| c.id == id)
                .unwrap()
                .value = (*name).to_owned();
        }
        let mut buffers = HashMap::new();
        buffers.insert(
            CellViewRef::new("work", "amp", "schematic").key(),
            schematic,
        );
        (libraries, buffers)
    }

    fn library_with_invalid_symbol_metadata() -> (LibraryManager, HashMap<String, SchematicState>) {
        let (mut libraries, buffers) = library_with_amp(None);
        let symbol_view = libraries
            .get_library_mut("work")
            .and_then(|library| library.get_cell_mut("amp"))
            .and_then(|cell| cell.get_view_mut("symbol"))
            .expect("symbol view exists");
        symbol_view.metadata.insert(
            SYMBOL_DOCUMENT_METADATA_KEY.to_owned(),
            "{not-valid-json".to_owned(),
        );
        (libraries, buffers)
    }

    #[test]
    fn authored_symbol_positions_override_generated_geometry_in_interface_order() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-20, -20),
                    Point::new(20, -20),
                    Point::new(20, 20),
                ],
                closed: false,
            }],
            ..SymbolDocument::default()
        };
        let (libraries, buffers) = library_with_amp(Some(document));
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
        let pins: Vec<(&str, Point)> = resolved
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), pin.offset))
            .collect();

        assert_eq!(
            pins,
            vec![("IN", Point::new(-40, -10)), ("OUT", Point::new(70, 20))]
        );
        assert!(resolved.issues().is_empty());
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Authored));
    }

    #[test]
    fn resolver_falls_back_to_generated_symbol_when_no_authored_metadata_exists() {
        let (libraries, buffers) = library_with_amp(None);
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let resolved = resolver
            .resolve_binding(&binding)
            .expect("fallback resolves");
        let pins: Vec<(&str, Point)> = resolved
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), pin.offset))
            .collect();

        assert_eq!(pins.len(), 2);
        assert_eq!(pins[0].0, "IN");
        assert_eq!(pins[1].0, "OUT");
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
    }

    #[test]
    fn authored_unplaced_pin_reports_issue_and_is_not_connectable() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, None),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(30, 0))),
            ],
            ..SymbolDocument::default()
        };
        let (libraries, buffers) = library_with_amp(Some(document));
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
        let pins: Vec<&str> = resolved
            .connectable_pins()
            .map(|pin| pin.name.as_str())
            .collect();

        assert_eq!(pins, vec!["OUT"]);
        assert!(resolved.issues().iter().any(|issue| {
            matches!(issue.kind, ResolvedSymbolIssueKind::UnplacedPin) && issue.pin_name == "IN"
        }));
    }

    #[test]
    fn placed_binding_prefers_saved_interface_over_live_master_ports() {
        let (libraries, buffers) = library_with_amp_and_ports(
            None,
            &[
                ("IN", PortDirection::In),
                ("OUT", PortDirection::Out),
                ("BIAS", PortDirection::In),
            ],
        );
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
        let pins: Vec<&str> = resolved
            .connectable_pins()
            .map(|pin| pin.name.as_str())
            .collect();

        assert_eq!(pins, vec!["IN", "OUT"]);
    }

    #[test]
    fn invalid_authored_metadata_falls_back_to_generated_symbol_with_issue() {
        let (libraries, buffers) = library_with_invalid_symbol_metadata();
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let resolved = resolver
            .resolve_binding(&binding)
            .expect("fallback resolves");
        let pins: Vec<&str> = resolved
            .connectable_pins()
            .map(|pin| pin.name.as_str())
            .collect();

        assert_eq!(pins, vec!["IN", "OUT"]);
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
        assert!(resolved.issues().iter().any(|issue| {
            matches!(issue.kind, ResolvedSymbolIssueKind::InvalidMetadata)
                && issue.pin_name == "symbol"
        }));
    }
}
