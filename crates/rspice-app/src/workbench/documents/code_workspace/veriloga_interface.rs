//! Where a Verilog-A cell's symbol stands against the interface its module's
//! compile gave the cell, and what the editor says about it.
//!
//! The model makes a clean compile's ports the cell's. What is left here is
//! the reader's side of it: how far a hand-edited symbol has drifted from the
//! module's ports and the symbol "Update symbol" reconciles them to, the
//! finding the strip above the source states, the design check's finding for
//! a module that disagrees with its cell's schematic, and the placement a new
//! device's first clean compile offers.

use rspice_app_state::workbench::app_state::model::{
    interface_mismatch_detail, port_electrical_type,
};
use rspice_design::library::metadata::{
    VERILOGA_PORTS_KEY, decode_port_list, decode_source_view_ports,
};
use rspice_design::symbol::generated::{GENERATED_SYMBOL_VIEW, is_generated_symbol};

use crate::state::{
    CellViewRef, LibraryManager, PortSpec, ProjectSourceBundle, ProjectSourceRegistry,
    SymbolDocument, ViewType,
};

use super::page::VerilogAWorkbenchState;
use crate::state::{VerilogACompileReceipt, VerilogAInterfaceOutcome};
use crate::workbench::{MessageCatalog, MessageId};

/// The view whose ports define a cell when it has one.
const SCHEMATIC_VIEW: &str = "schematic";

/// `receipt`, offering to place the device New Verilog-A device… created at
/// `reference` when this compile is the first a reader started that leaves
/// it placeable: its `ports` became the cell's interface, or agree with the
/// schematic that defines it, and there is at least one. A starter compile
/// at creation never comes through here, and the offer is made once.
pub(crate) fn offer_placement(
    state: &mut VerilogAWorkbenchState,
    mut receipt: VerilogACompileReceipt,
    reference: &CellViewRef,
    ports: usize,
) -> VerilogACompileReceipt {
    let placeable = ports > 0
        && matches!(
            receipt.interface,
            Some(VerilogAInterfaceOutcome::Defined | VerilogAInterfaceOutcome::MatchesSchematic)
        );
    if placeable
        && let Some(index) = state
            .placement_handoffs
            .iter()
            .position(|handoff| &handoff.view == reference)
    {
        receipt.placement = Some(state.placement_handoffs.remove(index));
    }
    receipt
}

/// How far a hand-edited symbol has drifted from the module's ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogASymbolDrift {
    /// Ports the symbol has no placed pin for.
    pub missing: Vec<String>,
    /// Pins no port declares. They stay until their author removes them.
    pub extra: Vec<String>,
    /// Pins that point a way their port no longer does.
    pub turned: Vec<String>,
}

impl VerilogASymbolDrift {
    /// Whether Update symbol has anything to do. Pins no port declares are
    /// their author's to remove, in the symbol editor.
    pub fn updatable(&self) -> bool {
        !self.missing.is_empty() || !self.turned.is_empty()
    }
}

/// "Symbol out of date: en missing": which ports the symbol is behind on.
pub(crate) fn symbol_drift_text(drift: &VerilogASymbolDrift, messages: MessageCatalog) -> String {
    let details = [
        (&drift.missing, MessageId::VerilogASymbolStripMissing),
        (&drift.turned, MessageId::VerilogASymbolStripTurned),
        (&drift.extra, MessageId::VerilogASymbolStripExtra),
    ]
    .into_iter()
    .filter(|(names, _)| !names.is_empty())
    .map(|(names, clause)| messages.format(clause, &[("names", &listed_names(names))]))
    .collect::<Vec<_>>()
    .join("; ");
    messages.format(MessageId::VerilogASymbolStrip, &[("details", &details)])
}

/// Up to three names, then how many more.
fn listed_names(names: &[String]) -> String {
    const SHOWN: usize = 3;
    let mut text = names
        .iter()
        .take(SHOWN)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    if names.len() > SHOWN {
        text.push_str(&format!(" +{}", names.len() - SHOWN));
    }
    text
}

/// Where the open cell view's symbol stands against its interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerilogACellSymbol {
    /// The cell's schematic defines its ports and keeps its symbol.
    SchematicDefined,
    /// The module defines the ports; the cell has no symbol yet.
    None,
    /// The module defines the ports, and the symbol is generated from them.
    Generated,
    /// The module defines the ports, and a hand-edited symbol agrees.
    InStep,
    /// The module defines the ports, and a hand-edited symbol has drifted.
    Drifted(VerilogASymbolDrift),
}

/// [`VerilogACellSymbol`] for the open view, measured once per library
/// revision rather than once per frame: it parses the symbol document.
#[derive(Debug, Clone, Default)]
pub struct VerilogASymbolStanding {
    measured: Option<(CellViewRef, u64)>,
    standing: Option<VerilogACellSymbol>,
}

impl VerilogASymbolStanding {
    /// The standing of `reference`'s symbol, if it is the view last measured.
    pub(crate) fn of(&self, reference: &CellViewRef) -> Option<&VerilogACellSymbol> {
        self.measured
            .as_ref()
            .filter(|(measured, _)| measured == reference)
            .and(self.standing.as_ref())
    }
}

/// Measure the open Verilog-A view's symbol again if the library changed.
pub(crate) fn refresh_symbol_standing(
    state: &mut VerilogAWorkbenchState,
    libraries: &LibraryManager,
    active: &CellViewRef,
    active_type: ViewType,
) {
    if active_type != ViewType::VerilogA {
        if state.symbol_standing.measured.is_some() {
            state.symbol_standing = VerilogASymbolStanding::default();
        }
        return;
    }
    let revision = libraries.revision();
    if state
        .symbol_standing
        .measured
        .as_ref()
        .is_some_and(|(measured, at)| measured == active && *at == revision)
    {
        return;
    }
    state.symbol_standing = VerilogASymbolStanding {
        standing: symbol_standing(libraries, active),
        measured: Some((active.clone(), revision)),
    };
}

/// Where the symbol of the cell owning the Verilog-A view at `reference`
/// stands against the cell's interface.
pub(crate) fn symbol_standing(
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Option<VerilogACellSymbol> {
    let cell = libraries
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?;
    if cell.get_view(SCHEMATIC_VIEW).is_some() {
        return Some(VerilogACellSymbol::SchematicDefined);
    }
    let Some(symbol) = cell
        .get_view(GENERATED_SYMBOL_VIEW)
        .filter(|view| view.view_type == ViewType::Symbol)
    else {
        return Some(VerilogACellSymbol::None);
    };
    if is_generated_symbol(symbol) {
        return Some(VerilogACellSymbol::Generated);
    }
    let ports = cell
        .get_view(&reference.view)?
        .metadata
        .get(VERILOGA_PORTS_KEY)
        .map(|encoded| decode_port_list(encoded))?;
    let document = SymbolDocument::load_from_view(symbol).ok()?;
    let drift = symbol_drift(&document, &ports);
    Some(
        if drift.missing.is_empty() && drift.extra.is_empty() && drift.turned.is_empty() {
            VerilogACellSymbol::InStep
        } else {
            VerilogACellSymbol::Drifted(drift)
        },
    )
}

fn symbol_drift(document: &SymbolDocument, ports: &[PortSpec]) -> VerilogASymbolDrift {
    let mut drift = VerilogASymbolDrift {
        missing: Vec::new(),
        extra: Vec::new(),
        turned: Vec::new(),
    };
    for port in ports {
        match document.pin(&port.name) {
            Some(pin) if pin.position.is_some() => {
                if !pin.states_direction(port.direction) {
                    drift.turned.push(port.name.clone());
                }
            }
            _ => drift.missing.push(port.name.clone()),
        }
    }
    drift.extra = document
        .pins
        .iter()
        .filter(|pin| {
            !ports
                .iter()
                .any(|port| port.name.eq_ignore_ascii_case(&pin.name))
        })
        .map(|pin| pin.name.clone())
        .collect();
    drift
}

/// The hand-edited symbol of the cell owning `reference`, reconciled with
/// the module's ports: missing pins added, directions taken from the ports,
/// discrete ports' pins `Logic`. Extra pins are kept for their author.
pub(crate) fn reconciled_symbol(
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Result<SymbolDocument, String> {
    let cell = libraries
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .ok_or_else(|| format!("{} no longer exists", reference.display_path()))?;
    let encoded = cell
        .get_view(&reference.view)
        .and_then(|view| view.metadata.get(VERILOGA_PORTS_KEY))
        .ok_or_else(|| {
            format!(
                "{} has no compiled interface yet; compile the module first",
                reference.display_path()
            )
        })?;
    let symbol = cell
        .get_view(GENERATED_SYMBOL_VIEW)
        .filter(|view| view.view_type == ViewType::Symbol && !is_generated_symbol(view))
        .ok_or_else(|| {
            format!(
                "{}/{} has no hand-edited symbol to update",
                reference.library, reference.cell
            )
        })?;
    let typed = decode_source_view_ports(encoded).unwrap_or_default();
    let ports = decode_port_list(encoded);
    let mut document = SymbolDocument::load_from_view(symbol)?;
    document.reconcile_typed_ports(&ports, port_electrical_type(&typed));
    Ok(document)
}

/// A finding the editor's strip states about the open view's interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerilogAInterfaceFinding<'a> {
    /// The current compile's ports differ from the cell's schematic.
    SchematicMismatch {
        schematic: &'a [PortSpec],
        module: &'a [PortSpec],
    },
    /// The current compile's ports could not be written.
    Refused(&'a str),
    /// The hand-edited symbol has drifted from the module's ports.
    SymbolDrift(&'a VerilogASymbolDrift),
}

/// What the strip above the source should say about the open view's
/// interface, if anything.
pub(crate) fn interface_finding<'a>(
    state: &'a VerilogAWorkbenchState,
    receipt: Option<&'a VerilogACompileReceipt>,
    reference: &CellViewRef,
) -> Option<VerilogAInterfaceFinding<'a>> {
    match receipt.and_then(|receipt| receipt.interface.as_ref()) {
        Some(VerilogAInterfaceOutcome::SchematicMismatch { schematic, module }) => {
            return Some(VerilogAInterfaceFinding::SchematicMismatch { schematic, module });
        }
        Some(VerilogAInterfaceOutcome::Refused(reason)) => {
            return Some(VerilogAInterfaceFinding::Refused(reason));
        }
        _ => {}
    }
    match state.symbol_standing.of(reference) {
        Some(VerilogACellSymbol::Drifted(drift)) => {
            Some(VerilogAInterfaceFinding::SymbolDrift(drift))
        }
        _ => None,
    }
}

/// The view of `bundle` that selects `module`: for a source shared by the
/// cells of a file's modules, that module's cell, otherwise the owner's view.
pub(crate) fn view_selecting_module<'a>(
    libraries: &LibraryManager,
    bundle: &'a ProjectSourceBundle,
    module: &str,
) -> Option<&'a CellViewRef> {
    let selects = |reference: &&CellViewRef| {
        libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| Some((cell, cell.get_view(&reference.view)?)))
            .and_then(|(cell, view)| {
                crate::state::workspace::metadata_value(
                    [&view.metadata, &cell.metadata],
                    &["veriloga.module", "netlist.module"],
                )
            })
            .is_some_and(|selected| selected.trim().eq_ignore_ascii_case(module))
    };
    bundle
        .cell_views()
        .find(selects)
        .or_else(|| bundle.cell_views().next())
}

/// The design check's finding for a retained compile whose module disagrees
/// with its cell's schematic: the view it names, and what to tell the reader.
pub(crate) fn interface_check_finding(
    state: &VerilogAWorkbenchState,
    sources: &ProjectSourceRegistry,
    libraries: &LibraryManager,
) -> Option<(CellViewRef, String)> {
    let receipt = state.receipt.as_ref()?;
    let Some(VerilogAInterfaceOutcome::SchematicMismatch { schematic, module }) =
        receipt.interface.as_ref()
    else {
        return None;
    };
    // A receipt speaks for the exact revision it compiled; an edit since
    // has made it history, and its finding with it.
    let token = receipt.compilation.token();
    let bundle = sources.get_bundle(token.bundle_id).filter(|bundle| {
        bundle.revision().get() == token.revision && bundle.closure_digest() == token.closure_digest
    })?;
    let compiled = receipt.compilation.report().abi.module_name.to_string();
    let reference = view_selecting_module(libraries, bundle, &compiled)?;
    Some((
        reference.clone(),
        format!(
            "Verilog-A view {} compiles to ports that differ from its cell's schematic. {} \
             A run that uses this view refuses it until the two agree.",
            reference.display_path(),
            interface_mismatch_detail(reference, schematic, module)
        ),
    ))
}
