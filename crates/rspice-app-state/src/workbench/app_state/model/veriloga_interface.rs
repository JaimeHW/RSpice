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
//!
//! The same write states the module's parameters as the view's
//! `veriloga.parameter_contract`: types, defaults, ranges, units and
//! descriptions, the one contract an instance's property sheet and the
//! netlister's allowlist are both read from. A module refused for its ports
//! writes neither, and a recompile that changes nothing writes nothing.

use std::collections::HashMap;
use std::sync::Arc;

use rspice_design::library::metadata::{
    CellParameter, ParameterDeclaration, ParameterRange, ParameterScope, ParameterValueType,
    PortDomain, RangeBound, RangeValue, SourceViewPort, VERILOGA_PARAMETER_CONTRACT_KEY,
    VERILOGA_PORTS_KEY, encode_source_view_ports, encode_veriloga_parameter_contract,
    parameter_number_text,
};
use rspice_design::schematic::owned::Schematic;
use rspice_design::symbol::generated::{
    GENERATED_SYMBOL_VIEW, is_generated_symbol, store_generated_interface_symbol,
};
use rspice_design::symbol_generation::InterfacePin;
use rspice_design_model::bus::{BusDeclaration, BusNotation};
use rspice_veriloga::ast::PortDirection as ModulePortDirection;
use rspice_veriloga::canonical_ir::CanonicalValueType;
use rspice_veriloga::disciplines::Domain;
use rspice_veriloga::{
    RuntimeAbiParameterValue, RuntimeAbiRangeBound, RuntimeAbiRangeValue, RuntimeAbiSummary,
};
use rspice_vocabulary::diagnostics::{
    DiagnosticSeverity as CodeEditorSeverity, SourceDiagnostic as CodeEditorDiagnostic,
};

use super::Model;
use super::veriloga_compile::compile_project_bundle_receipt;
use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::state::workspace::{starter_veriloga_source, verilog_module_name};
use crate::state::{
    CellViewRef, LibraryManager, PortDirection, PortSpec, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry, SymbolElectricalType, View,
    ViewType,
};
use crate::state::{VerilogACompileReceipt, VerilogAInterfaceOutcome};

/// The view whose ports define a cell when it has one.
const SCHEMATIC_VIEW: &str = "schematic";

/// A clean compile's interface outcome, and whether it changed the library.
#[derive(Debug)]
pub struct AppliedInterface {
    pub outcome: VerilogAInterfaceOutcome,
    pub wrote: bool,
}

/// The interface a compiled module's formal ports declare, in the order the
/// module binds its terminals.
///
/// Every port is checked against the terminals it binds. A scalar is named
/// as its terminal is; a continuous vector's members, expanded as the design
/// model expands a bus pin, are its terminals in order; a discrete vector
/// binds exactly one. The ports between them bind every terminal once, so the
/// generated symbol's conductors are the module's, one for one.
fn compiled_interface(abi: &RuntimeAbiSummary) -> Result<Vec<SourceViewPort>, String> {
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
pub fn port_electrical_type(
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

/// What the generated symbol knows about each compiled port: its role from
/// its name and declared direction, and a thermal node from its discipline.
pub fn interface_pins(ports: &[SourceViewPort]) -> Vec<InterfacePin> {
    ports
        .iter()
        .map(|port| {
            rspice_design::pin_roles::interface_pin(
                &port.name,
                port.direction,
                port.discipline.eq_ignore_ascii_case("thermal"),
            )
        })
        .collect()
}

/// Give the cell of the Verilog-A view at `reference` the interface its
/// clean compile declares, under the rule in this module's header.
pub fn apply_compiled_interface(
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
    let contract = match encode_veriloga_parameter_contract(&compiled_parameters(abi)) {
        Ok(contract) => contract,
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
        if !same_interface(&schematic, &module) {
            return AppliedInterface {
                outcome: VerilogAInterfaceOutcome::SchematicMismatch { schematic, module },
                wrote: false,
            };
        }
        // The schematic owns the ports; the module still owns its
        // parameters, which an instance bound to this view takes.
        let parameters = cell
            .get_view(&reference.view)
            .filter(|_| !library.read_only)
            .and_then(|view| updated_view_metadata(view, None, &contract));
        let wrote = parameters.is_some();
        if let Some(metadata) = parameters
            && let Some(mut edit) = libraries.edit_library(&reference.library)
        {
            edit.replace_view_metadata(&reference.cell, &reference.view, metadata);
            edit.mark_view_modified(&reference.cell, &reference.view);
        }
        return AppliedInterface {
            outcome: VerilogAInterfaceOutcome::MatchesSchematic,
            wrote,
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
    let interface = updated_view_metadata(view, Some(encoded), &contract);
    let existing_symbol = cell.get_view(GENERATED_SYMBOL_VIEW);
    let symbol = match existing_symbol {
        // A hand-edited symbol is its author's; its drift is reported.
        Some(symbol) if !is_generated_symbol(symbol) => None,
        existing => {
            let mut symbol = existing
                .cloned()
                .unwrap_or_else(|| View::new(GENERATED_SYMBOL_VIEW, ViewType::Symbol));
            if let Err(reason) = store_generated_interface_symbol(
                &mut symbol,
                &module,
                &interface_pins(&ports),
                port_electrical_type(&ports),
            ) {
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

/// `view`'s metadata with the compile's interface, when the module defines
/// it, and its parameter contract; `None` when that is what it already says,
/// so an unchanged recompile writes nothing and dirties nothing.
fn updated_view_metadata(
    view: &View,
    ports: Option<String>,
    contract: &str,
) -> Option<HashMap<String, String>> {
    let mut metadata = view.metadata.clone();
    if let Some(ports) = ports {
        metadata.insert(VERILOGA_PORTS_KEY.to_owned(), ports);
    }
    metadata.insert(
        VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(),
        contract.to_owned(),
    );
    (metadata != view.metadata).then_some(metadata)
}

/// The parameters a compiled module declares, as its view's contract states
/// them: every public parameter, whatever its scope, because an instance of
/// a module takes all of them on its own line.
pub fn compiled_parameters(abi: &RuntimeAbiSummary) -> Vec<CellParameter> {
    let value = |value: &RuntimeAbiRangeValue| match value {
        RuntimeAbiRangeValue::Constant(value) => RangeValue::Constant(*value),
        RuntimeAbiRangeValue::Dependent(text) => RangeValue::Dependent(text.to_string()),
    };
    let bound = |bound: &RuntimeAbiRangeBound| match bound {
        RuntimeAbiRangeBound::Unbounded => RangeBound::Unbounded,
        RuntimeAbiRangeBound::Inclusive(bound) => RangeBound::Inclusive(value(bound)),
        RuntimeAbiRangeBound::Exclusive(bound) => RangeBound::Exclusive(value(bound)),
    };
    abi.parameters
        .iter()
        .map(|parameter| CellParameter {
            name: parameter.name.to_string(),
            aliases: parameter.aliases.iter().map(ToString::to_string).collect(),
            required: false,
            default_value: parameter.default.as_ref().map(|default| match default {
                RuntimeAbiParameterValue::Real(value) => parameter_number_text(*value),
                RuntimeAbiParameterValue::Integer(value) => value.to_string(),
                RuntimeAbiParameterValue::String(text) => text.to_string(),
            }),
            declaration: Some(ParameterDeclaration {
                value_type: match parameter.value_type {
                    CanonicalValueType::Integer => ParameterValueType::Integer,
                    CanonicalValueType::String => ParameterValueType::String,
                    _ => ParameterValueType::Real,
                },
                scope: match parameter.scope {
                    rspice_veriloga::ParameterScope::Instance => ParameterScope::Instance,
                    rspice_veriloga::ParameterScope::Model => ParameterScope::Model,
                },
                range: parameter.range.as_ref().map(|range| ParameterRange {
                    lower: bound(&range.lower),
                    upper: bound(&range.upper),
                    exclude: range.exclude.iter().map(value).collect(),
                }),
                units: parameter.units.as_ref().map(ToString::to_string),
                description: parameter.description.as_ref().map(ToString::to_string),
            }),
        })
        .collect()
}

/// The interface a compile that was never published defines, made the
/// cell's by the rule a published compile follows, and carried on its
/// receipt. A view created with a starter module is compiled before it
/// exists anywhere a reader could compile it, and goes through here so its
/// `veriloga.ports` and generated symbol are the ones its first published
/// compile would write. Refused when the ports could not become the cell's.
fn apply_receipt_interface(
    libraries: &mut LibraryManager,
    schematic_buffers: &HashMap<String, Schematic>,
    reference: &CellViewRef,
    receipt: VerilogACompileReceipt,
) -> Result<VerilogACompileReceipt, String> {
    let applied = apply_compiled_interface(
        libraries,
        schematic_buffers,
        reference,
        &receipt.compilation.report().abi,
    );
    if let VerilogAInterfaceOutcome::Refused(reason) = applied.outcome {
        return Err(reason);
    }
    Ok(with_interface_outcome(receipt, reference, applied.outcome))
}

/// Give the cell at `reference` a `veriloga` view holding a starter module
/// for `ports`, with the interface and the generated symbol that module's
/// compile defines, in `libraries` and `sources`. Both are copies: the caller
/// publishes them only once everything here has been made.
///
/// New Verilog-A device… and New Cell's Verilog-A view both take this one
/// path, so a cell made either way is the cell its first compile would make.
pub fn seed_veriloga_view(
    model: &Model,
    libraries: &mut LibraryManager,
    sources: &mut ProjectSourceRegistry,
    reference: &CellViewRef,
    ports: &[PortSpec],
) -> Result<VerilogACompileReceipt, String> {
    let module_name = verilog_module_name(&reference.cell);
    let mut bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        format!("{module_name}.va"),
        starter_veriloga_source(&module_name, ports),
        [],
        [],
    )
    .map_err(|error| format!("The Verilog-A source could not be created: {error}"))?;
    let receipt = compile_project_bundle_receipt(
        model.workspace.content.project.id(),
        &bundle,
        Some(&module_name),
    )
    .map_err(|diagnostics| {
        let detail = diagnostics
            .first()
            .map(|diagnostic| format!("{}: {}", diagnostic.message, diagnostic.detail))
            .unwrap_or_else(|| "the compiler returned no diagnostic".to_owned());
        format!("The starter module did not compile: {detail}")
    })?;
    bundle.mark_validated().map_err(|error| {
        format!("The compiled starter module could not keep its validation identity: {error}")
    })?;

    // The view, then the source it owns, then the interface its compile
    // defines, which writes `veriloga.ports` and the generated symbol.
    let mut view = View::new(&reference.view, ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module_name);
    libraries
        .edit_library(&reference.library)
        .ok_or_else(|| format!("Library '{}' not found", reference.library))?
        .add_view(&reference.cell, view)
        .ok_or_else(|| {
            format!(
                "Cell '{}' not found in library '{}'",
                reference.cell, reference.library
            )
        })?;
    sources
        .insert_bundle(bundle)
        .map_err(|error| format!("The Verilog-A source could not be retained: {error}"))?;
    apply_receipt_interface(
        libraries,
        &model.workspace.content.schematic_buffers,
        reference,
        receipt,
    )
    .map_err(|reason| format!("The module's ports could not become the cell's: {reason}"))
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
const INTERFACE_MISMATCH_CODE: &str = "VA-INTERFACE-SCHEMATIC-MISMATCH";

/// `receipt` with the interface outcome its compile produced. A module that
/// disagrees with its cell's schematic carries the disagreement as an error
/// the editor and the inspector show beside the compiler's own findings.
pub fn with_interface_outcome(
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
pub fn interface_mismatch_detail(
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
