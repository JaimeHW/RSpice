//! What the model says about one cell's symbol: its published artwork and
//! authoring metadata, the interface contract its pins are checked against,
//! and the stamp of the drawing that contract is read from.
//!
//! The pin checks, the waivers and the diagnostic digests read these, as do
//! the symbol editor's own workflows, so they are the model's operations.

use rspice_design::library::metadata::{
    SourceViewPort, VERILOGA_PORTS_KEY, decode_source_view_ports, metadata_ports,
    module_interface_view,
};
use rspice_design::symbol::generated::{
    GENERATED_PORTS_KEY, decode_ports, generated_interface_document,
};

use super::model::{Model, interface_pins, port_electrical_type};
use crate::state::{
    CellViewRef, PortSpec, SYMBOL_DOCUMENT_METADATA_KEY, SymbolDocument, SymbolEditorMetadata,
};

impl Model {
    /// Everything the symbol pin checks read for one symbol view.
    ///
    /// One accessor, because the digest that decides whether a published pin
    /// finding still describes the design has to be taken over exactly what
    /// the check compared — and the check and the digest both read it here.
    pub fn symbol_pin_check_inputs(
        &self,
        reference: &CellViewRef,
    ) -> Result<SymbolPinCheckInputs, String> {
        let document = self.load_symbol_document(reference)?;
        let revision = self
            .load_symbol_editor_metadata(reference, &document)?
            .revision;
        let ports = self.symbol_ports(reference);
        Ok(SymbolPinCheckInputs {
            revision,
            document,
            ports,
        })
    }

    /// The interface contract one cell's symbol is checked against.
    ///
    /// Addressed by reference rather than only by what is in front of the
    /// reader, because the currency of a published pin finding is decided by
    /// the contract that finding was derived from — and navigating away from
    /// a symbol does not change that symbol's contract.
    pub fn symbol_ports(&self, reference: &CellViewRef) -> Vec<PortSpec> {
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.content.active_view == schematic_ref {
            return self.schematic.document().interface_ports();
        }
        if let Some(schematic) = self
            .workspace
            .content
            .schematic_buffers
            .get(&schematic_ref.key())
        {
            let ports = schematic.document().interface_ports();
            if !ports.is_empty() {
                return ports;
            }
        }

        let cell = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell));
        // With no schematic, the module's last clean compile is the contract.
        if let Some(ports) = cell
            .and_then(module_interface_view)
            .and_then(|view| metadata_ports(&view.metadata))
            .filter(|ports| !ports.is_empty())
        {
            return ports;
        }
        cell.and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| view.metadata.get(GENERATED_PORTS_KEY))
            .map(|encoded| decode_ports(encoded))
            .unwrap_or_default()
    }

    /// What the interface of one cell's symbol is read from, in the words
    /// the symbol editor's actions use for it.
    pub fn symbol_contract_source(&self, reference: &CellViewRef) -> &'static str {
        let module_defined = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(module_interface_view)
            .is_some();
        if module_defined {
            "module"
        } else {
            "schematic"
        }
    }

    /// The ports of one cell's module, with their disciplines and domains,
    /// when that module's last clean compile is the cell's interface.
    fn symbol_contract_module_ports(&self, reference: &CellViewRef) -> Option<Vec<SourceViewPort>> {
        let cell = self
            .library_manager
            .get_library(&reference.library)?
            .get_cell(&reference.cell)?;
        decode_source_view_ports(
            module_interface_view(cell)?
                .metadata
                .get(VERILOGA_PORTS_KEY)?,
        )
    }

    /// The symbol generated for `ports`: for a cell a compiled module
    /// defines, the one that module's compile draws.
    pub fn generated_symbol_document(
        &self,
        reference: &CellViewRef,
        ports: &[PortSpec],
    ) -> SymbolDocument {
        match self.symbol_contract_module_ports(reference) {
            Some(typed) => generated_interface_document(
                ports,
                &interface_pins(&typed),
                port_electrical_type(&typed),
            ),
            None => SymbolDocument::generated_from_ports(ports),
        }
    }

    /// `document` brought into agreement with `ports`, a compiled module's
    /// discrete ports typed as the module declares them.
    pub fn reconcile_symbol_with_contract(
        &self,
        reference: &CellViewRef,
        document: &mut SymbolDocument,
        ports: &[PortSpec],
    ) {
        let typed = self
            .symbol_contract_module_ports(reference)
            .unwrap_or_default();
        document.reconcile_typed_ports(ports, port_electrical_type(&typed));
    }

    pub fn load_active_symbol_document(&self) -> Result<SymbolDocument, String> {
        self.load_symbol_document(&self.workspace.content.active_view)
    }

    /// One cell's symbol artwork, as published in the library.
    pub fn load_symbol_document(&self, reference: &CellViewRef) -> Result<SymbolDocument, String> {
        let Some(view) = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        let has_document = view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY);
        let document = SymbolDocument::load_from_view(view)?;
        if has_document {
            return Ok(document);
        }
        let ports = self.symbol_ports(reference);
        if view.metadata.contains_key("generated") && !ports.is_empty() {
            return Ok(SymbolDocument::generated_from_ports(&ports));
        }
        Ok(document)
    }

    /// The [stamp](rspice_design::stamped) of the drawing [`Self::symbol_ports`]
    /// reads the contract from first, or `None` where it reads only the
    /// library, whose revision covers the rest.
    ///
    /// Beside [`Self::symbol_ports`] because it must take the same branches:
    /// a stamp of any other drawing would let an edit to the contract leave
    /// the pin findings reading current.
    pub(in crate::workbench) fn symbol_contract_stamp(
        &self,
        reference: &CellViewRef,
    ) -> Option<u64> {
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.content.active_view == schematic_ref {
            return Some(self.schematic.content_stamp());
        }
        self.workspace
            .content
            .schematic_buffers
            .get(&schematic_ref.key())
            .map(|schematic| schematic.content_stamp())
    }

    /// One cell's symbol authoring metadata, which is where its published
    /// revision is recorded.
    pub fn load_symbol_editor_metadata(
        &self,
        reference: &CellViewRef,
        document: &SymbolDocument,
    ) -> Result<SymbolEditorMetadata, String> {
        let Some(view) = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
        else {
            return Err(format!("View '{}' not found", reference.display_path()));
        };
        SymbolEditorMetadata::load_from_view(view, document)
    }
}

/// Everything one symbol view's pin checks were derived from.
///
/// The published revision as well as the artwork, because a revision is what
/// the instances placed against this symbol were wired to: republishing the
/// same drawing under a new one is a new contract, and a finding measured
/// against the old one no longer describes it.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolPinCheckInputs {
    pub(in crate::workbench) revision: u64,
    pub document: SymbolDocument,
    pub ports: Vec<PortSpec>,
}
