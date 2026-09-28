//! Resolve authored or generated symbols from borrowed design documents.

use rspice_design::library::{Cell, LibraryCatalog, View, ViewType};
use rspice_design::resolved_symbol::ResolvedCellSymbol;
use rspice_design::schematic::{component::LibraryCellInstance, document::SchematicDocument};
use rspice_design::symbol::{SYMBOL_DOCUMENT_METADATA_KEY, SymbolDocument};
use rspice_design_model::{
    cell_view::CellViewRef,
    port::{PortDirection, PortSpec},
};
use std::collections::HashMap;

pub struct SymbolResolver<'a, S: AsRef<SchematicDocument>> {
    libraries: &'a LibraryCatalog,
    schematic_buffers: &'a HashMap<String, S>,
    active_schematic: Option<(&'a CellViewRef, &'a SchematicDocument)>,
}

impl<'a, S: AsRef<SchematicDocument>> SymbolResolver<'a, S> {
    pub fn new(libraries: &'a LibraryCatalog, schematic_buffers: &'a HashMap<String, S>) -> Self {
        Self {
            libraries,
            schematic_buffers,
            active_schematic: None,
        }
    }

    /// Overlay an unsaved editor buffer without cloning the workspace.
    pub(crate) fn with_active_schematic(
        mut self,
        reference: &'a CellViewRef,
        schematic: &'a SchematicDocument,
    ) -> Self {
        self.active_schematic = Some((reference, schematic));
        self
    }

    pub fn resolve_binding(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        self.resolve_cell(
            &binding.library,
            &binding.cell,
            Some(&binding.view),
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
            requested_view,
            symbol_view,
            fallback_ports,
            port_preference,
        );

        match authored_document(symbol_view) {
            AuthoredDocument::Loaded(document) => {
                let ports = ports.unwrap_or_default();
                return Some(ResolvedCellSymbol::from_authored_document(document, &ports));
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
        requested_view: Option<&str>,
        symbol_view: Option<&View>,
        fallback_ports: Option<Vec<PortSpec>>,
        port_preference: PortPreference,
    ) -> Option<Vec<PortSpec>> {
        let explicit_ports = fallback_ports.filter(|ports| !ports.is_empty());
        let schematic_ports = || self.schematic_ports(library_name, cell_name, requested_view);
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

    fn schematic_ports(
        &self,
        library_name: &str,
        cell_name: &str,
        requested_view: Option<&str>,
    ) -> Option<Vec<PortSpec>> {
        let requested = CellViewRef::new(
            library_name,
            cell_name,
            requested_view.unwrap_or("schematic"),
        );
        let requested_key = requested.key();
        let is_schematic = self.schematic_buffers.contains_key(&requested_key)
            || self
                .schematic_buffers
                .keys()
                .any(|key| key.eq_ignore_ascii_case(&requested_key))
            || self
                .active_schematic
                .is_some_and(|(active, _)| active.key().eq_ignore_ascii_case(&requested_key))
            || self
                .libraries
                .get_library(library_name)
                .and_then(|library| library.get_cell(cell_name))
                .and_then(|cell| cell.get_view(&requested.view))
                .is_some_and(|view| view.view_type == ViewType::Schematic);
        let reference = if is_schematic {
            requested
        } else {
            CellViewRef::new(library_name, cell_name, "schematic")
        };
        if let Some((active, schematic)) = self.active_schematic
            && active.key().eq_ignore_ascii_case(&reference.key())
        {
            let ports = schematic.interface_ports();
            return (!ports.is_empty()).then_some(ports);
        }
        self.schematic_buffers
            .get(&reference.key())
            .or_else(|| {
                self.schematic_buffers
                    .iter()
                    .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
                    .map(|(_, schematic)| schematic)
            })
            .map(|schematic| schematic.as_ref().interface_ports())
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
