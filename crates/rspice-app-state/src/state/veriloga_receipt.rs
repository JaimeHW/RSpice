//! What a Verilog-A compile established about its source: the receipt the
//! code workspace keeps for the exact source it compiled, what the compile
//! did to its cell's interface, and the device it made placeable.
//!
//! Runtime state only: none of it is persisted.

use std::sync::Arc;

use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::state::{CellViewRef, PortSpec};

/// Retained metadata from the latest compile of the exact current source.
#[derive(Debug, Clone)]
pub struct VerilogACompileReceipt {
    pub compilation: rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt,
    pub bytecode_available: bool,
    pub diagnostics: Arc<CodeDiagnosticCollection>,
    /// What this compile did to its cell's interface. `None` for a compile
    /// that was not a cell view's published result.
    pub interface: Option<VerilogAInterfaceOutcome>,
    /// The device this compile made placeable, for the editor to offer
    /// "Place in <schematic>": the first clean compile a reader starts of a
    /// device New Verilog-A device… created, once it has a port.
    pub placement: Option<VerilogAPlacementHandoff>,
}

/// A device New Verilog-A device… created, and the schematic the form was
/// opened from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAPlacementHandoff {
    /// The device's Verilog-A cell view.
    pub view: CellViewRef,
    /// The schematic to place it in. `None` when the form was opened from
    /// anything else; the design's top schematic is offered then.
    pub schematic: Option<CellViewRef>,
}

/// What a clean compile did to its cell's interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerilogAInterfaceOutcome {
    /// The module's ports are the cell's interface.
    Defined,
    /// The cell's schematic defines its ports, and the module agrees.
    MatchesSchematic,
    /// The cell's schematic defines its ports, and the module's differ.
    SchematicMismatch {
        schematic: Vec<PortSpec>,
        module: Vec<PortSpec>,
    },
    /// Nothing was written, for the reason given.
    Refused(String),
}
