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

use rspice_design::symbol::publication::{EncodedSymbolEditorBundle, bound_interface};

use super::model::{Model, interface_pins, port_electrical_type};
use crate::state::{
    CellViewRef, ModelBoundSymbolDefinition, PortSpec, SYMBOL_DOCUMENT_METADATA_KEY,
    SymbolDocument, SymbolEditorMetadata, SymbolInterface, ViewType, load_model_bound_symbol,
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
        Ok(SymbolPinCheckInputs {
            document,
            interface: self.symbol_interface(reference),
            binding: self.symbol_model_binding(reference),
        })
    }

    /// The model one symbol view is bound to, if any.
    pub fn symbol_model_binding(
        &self,
        reference: &CellViewRef,
    ) -> Option<ModelBoundSymbolDefinition> {
        self.library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| load_model_bound_symbol(view).ok().flatten())
    }

    /// What one cell's symbol answers to: the model it is bound to, else the
    /// interface its schematic or compiled module declares, else nothing but
    /// its own pins.
    ///
    /// Addressed by reference rather than only by what is in front of the
    /// reader, because the currency of a published pin finding is decided by
    /// the interface that finding was derived from — and navigating away from
    /// a symbol does not change that symbol's interface.
    pub fn symbol_interface(&self, reference: &CellViewRef) -> SymbolInterface {
        match self.symbol_model_binding(reference) {
            Some(definition) => bound_interface(&definition),
            None => self.drawn_symbol_interface(reference),
        }
    }

    /// The pins one cell's schematic or compiled module declares for its
    /// symbol; none where the symbol defines the interface itself.
    pub fn symbol_ports(&self, reference: &CellViewRef) -> Vec<PortSpec> {
        match self.drawn_symbol_interface(reference) {
            SymbolInterface::SymbolDefined => Vec::new(),
            SymbolInterface::Declared(ports) => ports,
        }
    }

    /// The interface the cell's own drawings declare: its schematic, or with
    /// no schematic pins the module's last clean compile, or the ports a
    /// generated symbol was drawn from. A schematic that exists and declares
    /// no pins is a declaration too, of none; a cell with no schematic and no
    /// module leaves the symbol to define the interface.
    fn drawn_symbol_interface(&self, reference: &CellViewRef) -> SymbolInterface {
        let schematic_ref = CellViewRef::new(&reference.library, &reference.cell, "schematic");
        if self.workspace.content.active_view == schematic_ref {
            return SymbolInterface::Declared(self.schematic.document().interface_ports());
        }
        let schematic_ports = self
            .workspace
            .content
            .schematic_buffers
            .get(&schematic_ref.key())
            .map(|schematic| schematic.document().interface_ports());
        if let Some(ports) = schematic_ports.as_ref().filter(|ports| !ports.is_empty()) {
            return SymbolInterface::Declared(ports.clone());
        }

        let cell = self
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell));
        let module = cell.and_then(module_interface_view);
        if let Some(ports) = module
            .and_then(|view| metadata_ports(&view.metadata))
            .filter(|ports| !ports.is_empty())
        {
            return SymbolInterface::Declared(ports);
        }
        let schematic_exists = schematic_ports.is_some()
            || cell
                .and_then(|cell| cell.get_view(&schematic_ref.view))
                .is_some_and(|view| view.view_type == ViewType::Schematic);
        if schematic_exists || module.is_some() {
            return SymbolInterface::Declared(schematic_ports.unwrap_or_default());
        }
        match cell
            .and_then(|cell| cell.get_view(&reference.view))
            .and_then(|view| view.metadata.get(GENERATED_PORTS_KEY))
            .map(|encoded| decode_ports(encoded))
            .filter(|ports| !ports.is_empty())
        {
            Some(ports) => SymbolInterface::Declared(ports),
            None => SymbolInterface::SymbolDefined,
        }
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

    /// Record the save about to write `reference`: a new revision, dated
    /// `saved_unix_ms`, when the symbol's artwork differs from what its last
    /// revision recorded. Returns the revision recorded, if one was.
    ///
    /// Only an authored symbol in a library the project writes has a
    /// revision history; a symbol still drawn from generated ports, or one a
    /// read-only library holds, records none.
    pub fn record_symbol_save(
        &mut self,
        reference: &CellViewRef,
        saved_unix_ms: u64,
    ) -> Result<Option<u64>, String> {
        let authored = self
            .library_manager
            .get_library(&reference.library)
            .filter(|library| !library.read_only)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some_and(|view| {
                view.view_type == ViewType::Symbol
                    && view.metadata.contains_key(SYMBOL_DOCUMENT_METADATA_KEY)
            });
        if !authored {
            return Ok(None);
        }
        let document = self.load_symbol_document(reference)?;
        let mut metadata = self.load_symbol_editor_metadata(reference, &document)?;
        let Some(revision) = metadata.record_saved_revision(&document, saved_unix_ms) else {
            return Ok(None);
        };
        let bundle = EncodedSymbolEditorBundle::encode(&document, &metadata)?;
        self.library_manager
            .edit_library(&reference.library)
            .and_then(|mut library| {
                library.store_symbol_editor_bundle(&reference.cell, &reference.view, bundle)
            })
            .ok_or_else(|| format!("View '{}' not found", reference.display_path()))?;
        Ok(Some(revision))
    }

    /// Every symbol view in a library the project writes.
    pub fn writable_symbol_views(&self) -> Vec<CellViewRef> {
        let mut views = Vec::new();
        for library in self.library_manager.libraries_sorted() {
            if library.read_only {
                continue;
            }
            for cell in library.cells.values() {
                for view in cell.views.values() {
                    if view.view_type == ViewType::Symbol {
                        views.push(CellViewRef::new(&library.name, &cell.name, &view.name));
                    }
                }
            }
        }
        views
    }

    /// One cell's symbol authoring metadata, which is where its revision
    /// history is recorded.
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

/// Everything one symbol view's pin checks were derived from: the artwork,
/// the interface it answers to, and the model it is bound to.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolPinCheckInputs {
    pub document: SymbolDocument,
    pub interface: SymbolInterface,
    pub binding: Option<ModelBoundSymbolDefinition>,
}
