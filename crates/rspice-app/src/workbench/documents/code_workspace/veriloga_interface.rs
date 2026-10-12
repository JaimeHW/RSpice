//! What a clean Verilog-A compile does to its cell's interface.
//!
//! A cell's `schematic` view, when it has one, defines the cell's ports. A
//! module whose ports differ from it is a finding of that compile, and
//! `veriloga.ports` is left as it was. Otherwise the module's last clean
//! compile defines them: its formal ports become `veriloga.ports`, and the
//! generated `symbol` view follows. A hand-edited symbol is never rewritten
//! here. How far it has drifted from the module is measured instead, for the
//! editor and the inspector to offer "Update symbol".
//!
//! A continuous vector port is one bus pin, `d[3:0]`. Its conductors expand
//! most significant first as declared, which is the order the compiled module
//! binds its terminals in; [`compiled_interface`] checks that lane by lane
//! rather than assuming it. A discrete vector is one pin under its bare name:
//! its bits travel as one digital signal.

use std::collections::HashMap;
use std::sync::Arc;

use rspice_design::library::metadata::{
    PortDomain, SourceViewPort, VERILOGA_PORTS_KEY, decode_port_list, decode_source_view_ports,
    encode_source_view_ports,
};
use rspice_design::schematic::owned::Schematic;
use rspice_design::symbol::generated::{
    GENERATED_SYMBOL_VIEW, is_generated_symbol, store_generated_symbol,
};
use rspice_design_model::bus::{BusDeclaration, BusNotation};
use rspice_veriloga::RuntimeAbiSummary;
use rspice_veriloga::ast::PortDirection as ModulePortDirection;
use rspice_veriloga::disciplines::Domain;

use crate::state::{
    CellViewRef, LibraryManager, PortDirection, PortSpec, ProjectSourceOwner,
    ProjectSourceRegistry, SymbolDocument, SymbolElectricalType, View, ViewType,
};

use super::page::VerilogAWorkbenchState;
use super::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, VerilogACompileReceipt,
};

/// The view whose ports define a cell when it has one.
const SCHEMATIC_VIEW: &str = "schematic";

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

/// A clean compile's interface outcome, and whether it changed the library.
#[derive(Debug)]
pub(crate) struct AppliedInterface {
    pub(crate) outcome: VerilogAInterfaceOutcome,
    pub(crate) wrote: bool,
}

/// The interface a compiled module's formal ports declare, in the order the
/// module binds its terminals.
///
/// Every port is checked against the terminals it binds. A scalar is named
/// as its terminal is; a continuous vector's members, expanded as the design
/// model expands a bus pin, are its terminals in order; a discrete vector
/// binds exactly one. The ports between them bind every terminal once, so the
/// generated symbol's conductors are the module's, one for one.
pub(crate) fn compiled_interface(abi: &RuntimeAbiSummary) -> Result<Vec<SourceViewPort>, String> {
    let mut formal = abi.formal_ports.iter().collect::<Vec<_>>();
    formal.sort_by_key(|port| port.lanes.first().copied().unwrap_or(usize::MAX));
    let lane_name = |lane: usize| {
        abi.analog_ports
            .get(lane)
            .map(|port| port.name.as_str())
            .ok_or_else(|| {
                format!(
                    "module `{}` names terminal {lane}, which it does not bind",
                    abi.module_name
                )
            })
    };
    let mut next_lane = 0;
    let mut ports = Vec::with_capacity(formal.len());
    for port in formal {
        if !port
            .lanes
            .iter()
            .copied()
            .eq(next_lane..next_lane + port.lanes.len())
        {
            return Err(format!(
                "port `{}` of module `{}` does not bind its terminals in declaration order",
                port.name, abi.module_name
            ));
        }
        next_lane += port.lanes.len();
        let terminals = port
            .lanes
            .iter()
            .map(|lane| lane_name(*lane))
            .collect::<Result<Vec<_>, _>>()?;
        let name = match (port.domain, port.range, terminals.as_slice()) {
            (Domain::Continuous, Some(range), _) if terminals.len() > 1 => {
                let declaration = u32::try_from(range.msb)
                    .ok()
                    .zip(u32::try_from(range.lsb).ok())
                    .and_then(|(msb, lsb)| {
                        BusDeclaration::new(port.name.as_str(), msb, lsb, BusNotation::Square).ok()
                    })
                    .ok_or_else(|| {
                        format!(
                            "port `{}[{}:{}]` cannot be a bus pin: a pin's range runs from 0 to {}",
                            port.name,
                            range.msb,
                            range.lsb,
                            rspice_design_model::bus::MAX_BUS_MEMBER_INDEX
                        )
                    })?;
                let members = declaration
                    .members()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                if members.len() != terminals.len()
                    || members
                        .iter()
                        .zip(&terminals)
                        .any(|(member, terminal)| !member.eq_ignore_ascii_case(terminal))
                {
                    return Err(format!(
                        "bus pin `{}[{}:{}]` would expand to {} where module `{}` binds {}",
                        port.name,
                        range.msb,
                        range.lsb,
                        members.join(" "),
                        abi.module_name,
                        terminals.join(" ")
                    ));
                }
                format!("{}[{}:{}]", port.name, range.msb, range.lsb)
            }
            // A one-lane vector binds the one terminal it names, `d[0]`.
            (Domain::Continuous, _, [terminal]) => (*terminal).to_owned(),
            (Domain::Discrete, _, [_]) => port.name.to_string(),
            _ => {
                return Err(format!(
                    "port `{}` of module `{}` binds {} terminals, which no pin can carry",
                    port.name,
                    abi.module_name,
                    terminals.len()
                ));
            }
        };
        ports.push(SourceViewPort {
            name,
            direction: match port.direction {
                ModulePortDirection::Input => PortDirection::In,
                ModulePortDirection::Output => PortDirection::Out,
                ModulePortDirection::Inout => PortDirection::InOut,
            },
            discipline: port.discipline.to_string(),
            domain: match port.domain {
                Domain::Continuous => PortDomain::Continuous,
                Domain::Discrete => PortDomain::Discrete,
            },
        });
    }
    if next_lane != abi.analog_ports.len() {
        return Err(format!(
            "module `{}` binds {} terminals, but its ports name {next_lane}",
            abi.module_name,
            abi.analog_ports.len()
        ));
    }
    Ok(ports)
}

/// The electrical type a source-view port gives its pin: a discrete port's
/// pin is `Logic`, and any other takes its direction's default.
pub(crate) fn port_electrical_type(
    ports: &[SourceViewPort],
) -> impl Fn(&PortSpec) -> Option<SymbolElectricalType> + '_ {
    |spec| {
        ports
            .iter()
            .find(|port| port.name.eq_ignore_ascii_case(&spec.name))
            .filter(|port| port.domain == PortDomain::Discrete)
            .map(|_| SymbolElectricalType::Logic)
    }
}

/// Give the cell of the Verilog-A view at `reference` the interface its
/// clean compile declares, under the rule in this module's header.
pub(crate) fn apply_compiled_interface(
    libraries: &mut LibraryManager,
    schematic_buffers: &HashMap<String, Schematic>,
    reference: &CellViewRef,
    abi: &RuntimeAbiSummary,
) -> AppliedInterface {
    let refused = |reason: String| AppliedInterface {
        outcome: VerilogAInterfaceOutcome::Refused(reason),
        wrote: false,
    };
    let ports = match compiled_interface(abi) {
        Ok(ports) => ports,
        Err(reason) => return refused(reason),
    };
    let Some(library) = libraries.get_library(&reference.library) else {
        return refused(format!("library '{}' no longer exists", reference.library));
    };
    let Some(cell) = library.get_cell(&reference.cell) else {
        return refused(format!(
            "cell '{}' no longer exists",
            reference.display_path()
        ));
    };
    let module = ports.iter().map(SourceViewPort::spec).collect::<Vec<_>>();
    if cell.get_view(SCHEMATIC_VIEW).is_some() {
        let schematic = schematic_buffers
            .get(&CellViewRef::new(&reference.library, &reference.cell, SCHEMATIC_VIEW).key())
            .map(|schematic| schematic.document().interface_ports())
            .unwrap_or_default();
        let outcome = if same_interface(&schematic, &module) {
            VerilogAInterfaceOutcome::MatchesSchematic
        } else {
            VerilogAInterfaceOutcome::SchematicMismatch { schematic, module }
        };
        return AppliedInterface {
            outcome,
            wrote: false,
        };
    }
    if library.read_only {
        return refused(format!(
            "library '{}' is read-only, so neither {} nor its symbol was written",
            reference.library,
            reference.display_path()
        ));
    }
    let Some(view) = cell.get_view(&reference.view) else {
        return refused(format!(
            "view '{}' no longer exists",
            reference.display_path()
        ));
    };
    let encoded = match encode_source_view_ports(&ports) {
        Ok(encoded) => encoded,
        Err(reason) => return refused(reason),
    };
    let interface = (view.metadata.get(VERILOGA_PORTS_KEY) != Some(&encoded)).then(|| {
        let mut metadata = view.metadata.clone();
        metadata.insert(VERILOGA_PORTS_KEY.to_owned(), encoded);
        metadata
    });
    let existing_symbol = cell.get_view(GENERATED_SYMBOL_VIEW);
    let symbol = match existing_symbol {
        // A hand-edited symbol is its author's; its drift is reported.
        Some(symbol) if !is_generated_symbol(symbol) => None,
        existing => {
            let mut symbol = existing
                .cloned()
                .unwrap_or_else(|| View::new(GENERATED_SYMBOL_VIEW, ViewType::Symbol));
            if let Err(reason) =
                store_generated_symbol(&mut symbol, &module, port_electrical_type(&ports))
            {
                return refused(format!(
                    "the generated symbol could not be written: {reason}"
                ));
            }
            (existing.map(|view| &view.metadata) != Some(&symbol.metadata))
                .then(|| (existing.is_some(), symbol))
        }
    };
    let wrote = interface.is_some() || symbol.is_some();
    if wrote && let Some(mut edit) = libraries.edit_library(&reference.library) {
        if let Some(metadata) = interface {
            edit.replace_view_metadata(&reference.cell, &reference.view, metadata);
            edit.mark_view_modified(&reference.cell, &reference.view);
        }
        match symbol {
            Some((true, symbol)) => {
                edit.replace_view_metadata(&reference.cell, GENERATED_SYMBOL_VIEW, symbol.metadata);
                edit.mark_view_modified(&reference.cell, GENERATED_SYMBOL_VIEW);
            }
            Some((false, mut symbol)) => {
                symbol.modified = true;
                edit.add_view(&reference.cell, symbol);
            }
            None => {}
        }
    }
    AppliedInterface {
        outcome: VerilogAInterfaceOutcome::Defined,
        wrote,
    }
}

/// Whether a schematic's interface and a module's agree: the same names in
/// the same order, each pointing the same way. A supply pin is an `inout`
/// port, because a module has no other way to declare one.
fn same_interface(schematic: &[PortSpec], module: &[PortSpec]) -> bool {
    let direction = |direction| match direction {
        PortDirection::Supply => PortDirection::InOut,
        direction => direction,
    };
    schematic.len() == module.len()
        && schematic.iter().zip(module).all(|(left, right)| {
            left.name.eq_ignore_ascii_case(&right.name)
                && direction(left.direction) == direction(right.direction)
        })
}

/// Code of the compile finding a schematic-defined cell's module raises.
pub(crate) const INTERFACE_MISMATCH_CODE: &str = "VA-INTERFACE-SCHEMATIC-MISMATCH";

/// `receipt` with the interface outcome its compile produced. A module that
/// disagrees with its cell's schematic carries the disagreement as an error
/// the editor and the inspector show beside the compiler's own findings.
pub(crate) fn with_interface_outcome(
    mut receipt: VerilogACompileReceipt,
    reference: &CellViewRef,
    outcome: VerilogAInterfaceOutcome,
) -> VerilogACompileReceipt {
    if let VerilogAInterfaceOutcome::SchematicMismatch { schematic, module } = &outcome {
        let token = receipt.compilation.token();
        let finding = CodeEditorDiagnostic::current(
            "rspice.veriloga.interface",
            INTERFACE_MISMATCH_CODE,
            CodeEditorSeverity::Error,
            "Module ports differ from the cell's schematic",
            interface_mismatch_detail(reference, schematic, module),
            None,
            None,
            None,
            None,
            None,
        )
        .bind_validation(
            token.bundle_id.to_string(),
            token.revision,
            token.closure_digest.to_string(),
        );
        let records = receipt
            .diagnostics
            .iter()
            .cloned()
            .chain([finding])
            .collect::<Vec<_>>();
        if let Ok(collection) = CodeDiagnosticCollection::try_new(records) {
            receipt.diagnostics = Arc::new(collection);
        }
    }
    receipt.interface = Some(outcome);
    receipt
}

/// The schematic's ports and the module's, for a reader deciding which to
/// change.
pub(crate) fn interface_mismatch_detail(
    reference: &CellViewRef,
    schematic: &[PortSpec],
    module: &[PortSpec],
) -> String {
    // Spelled the way a module declares them, so the two lists compare.
    let list = |ports: &[PortSpec]| {
        if ports.is_empty() {
            return "no ports".to_owned();
        }
        ports
            .iter()
            .map(|port| {
                let direction = match port.direction {
                    PortDirection::In => "input",
                    PortDirection::Out => "output",
                    PortDirection::InOut => "inout",
                    PortDirection::Supply => "supply",
                };
                format!("{direction} {}", port.name)
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "The schematic of {}/{} declares {}; the module declares {}.",
        reference.library,
        reference.cell,
        list(schematic),
        list(module)
    )
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
                let pointed = |direction| match direction {
                    PortDirection::Supply => PortDirection::InOut,
                    direction => direction,
                };
                if pointed(pin.direction) != pointed(port.direction) {
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

/// The design check's finding for a retained compile whose module disagrees
/// with its cell's schematic: the view it names, and what to tell the reader.
pub(crate) fn interface_check_finding(
    state: &VerilogAWorkbenchState,
    sources: &ProjectSourceRegistry,
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
    let ProjectSourceOwner::CellView { reference } = bundle.owner() else {
        return None;
    };
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
