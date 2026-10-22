//! Executable authored connection bodies in the circuit's existing mixed host.

use std::sync::{Arc, OnceLock};

use super::connect_modules::PlannedConnectModule;
use super::mixed_modules::try_build_mixed_signal_instance;
use super::veriloga_instances::PreparedInstance;
use super::{PlannedXspiceAutoBridge, check_build_abort, xspice_auto_bridge_node_label};
use crate::{CircuitData, ElaborationError, ElaborationErrorKind, SimulationError};

#[derive(Debug)]
pub(super) enum ConnectExecution {
    /// A shipped signature, converted by the built-in connect-rule converter.
    Delegated(rspice_veriloga::connect::library::BuiltinSignature),
    Authored(AuthoredConnectBody),
}

#[derive(Debug)]
struct CompiledConnectBody {
    model: Arc<rspice_veriloga::CompiledModel>,
    artifact: Arc<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
}

/// Shared executable code; instance state is allocated independently by the
/// normal mixed host. The selected rule's parameter values key this cache.
#[derive(Debug)]
pub(super) struct AuthoredConnectBody {
    configuration: Arc<rspice_veriloga::ConnectionConfiguration>,
    source: Arc<str>,
    source_package: String,
    continuous: String,
    discrete: String,
    /// The `connect` statement that selected this body, in `source`: where a
    /// refusal with no narrower span points, such as an unknown parameter
    /// the statement overrides.
    statement: rspice_veriloga::source::Span,
    /// The deck's `.options wreal_resolution`, for the body's plain `wreal`s.
    wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    compiled: OnceLock<CompiledConnectBody>,
}

impl AuthoredConnectBody {
    pub(super) fn new(
        configuration: Arc<rspice_veriloga::ConnectionConfiguration>,
        source: Arc<str>,
        source_package: String,
        continuous: String,
        discrete: String,
        statement: rspice_veriloga::source::Span,
        wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
    ) -> Self {
        Self {
            configuration,
            source,
            source_package,
            continuous,
            discrete,
            statement,
            wreal_resolution,
            compiled: OnceLock::new(),
        }
    }

    /// A compile refusal located in the original source: the compiler's own
    /// findings when they carry a span, else the selecting statement.
    fn compile_refusal(&self, error: &rspice_veriloga::CompileError) -> ElaborationError {
        let refusal = |detail: String| {
            ElaborationError::new(ElaborationErrorKind::CompileRefusal, detail)
                .in_source(std::path::Path::new(&self.source_package))
        };
        let Some(coordinates) = super::connect_modules::original_coordinates(
            std::path::Path::new(&self.source_package),
            &self.source,
        ) else {
            return refusal(format!(
                "selected connect body could not be compiled: {error}"
            ));
        };
        let diagnostics = coordinates.diagnostics_for_error(error);
        if diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path.is_some())
        {
            return refusal("selected connect body could not be compiled".into())
                .with_compiler_diagnostics(diagnostics);
        }
        let refused = refusal(format!(
            "selected connect body could not be compiled: {error}"
        ));
        match coordinates.locate(self.statement.start as usize) {
            Some((file, line, _)) => {
                refused.at(crate::netlist::NetlistSourceLocation::in_file(file, line))
            }
            None => refused,
        }
    }
}

pub(super) fn materialize(
    circuit: &mut CircuitData,
    bridge: &PlannedXspiceAutoBridge,
    selected: &PlannedConnectModule,
    body: &AuthoredConnectBody,
    temperature: f64,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<(), SimulationError> {
    check_build_abort(abort)?;
    let refusal = |detail: String| {
        SimulationError::from(
            ElaborationError::new(ElaborationErrorKind::CompileRefusal, detail)
                .module(selected.name.as_str())
                .instance(selected.instance.as_str())
                .in_source(std::path::Path::new(&body.source_package)),
        )
    };
    if body.compiled.get().is_none() {
        let parameters: Vec<_> = selected
            .parameters
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
            .collect();
        // The body compiles with the deck's options, so its plain `wreal`
        // ports resolve as the deck's own modules' do.
        let runtime = rspice_veriloga::VerilogACompiler::new(
            super::veriloga_cache::deck_include_compiler_options(
                &super::veriloga_cache::DeckHdl::resolving_wreal(body.wreal_resolution),
            ),
        )
        .compile_connect_runtime_with_configuration(
            &body.source_package,
            &body.source,
            &selected.name,
            &parameters,
            &body.configuration,
            &super::veriloga_cache::VerilogACompileControl { abort },
        )
        .map_err(|error| {
            if abort.is_aborted() {
                SimulationError::Aborted
            } else {
                SimulationError::from(
                    body.compile_refusal(&error)
                        .module(selected.name.as_str())
                        .instance(selected.instance.as_str()),
                )
            }
        })?;
        let _ = body.compiled.set(CompiledConnectBody {
            model: Arc::new(runtime.model),
            artifact: Arc::new(runtime.canonical_ir),
        });
    }
    let compiled = body.compiled.get().expect("compiled connection body");
    let event_node = if let Some(node) = bridge.event_node {
        node
    } else {
        // XSPICE permits the same numeric node in its analog and event domains.
        // An executable connect host gives those domains distinct endpoints.
        let name = format!("{}__event", selected.instance).to_uppercase();
        if circuit.get_node_by_name(&name).is_some() {
            return Err(refusal(format!(
                "generated connect endpoint '{name}' conflicts with an existing circuit node"
            )));
        }
        let node = circuit.get_or_create_node(&name);
        circuit.rebind_xspice_event_node(bridge.node, node);
        node
    };
    let names = circuit.node_names_sorted();
    let continuous = xspice_auto_bridge_node_label(Some(&names), bridge.node);
    let discrete = xspice_auto_bridge_node_label(Some(&names), event_node);
    let nodes = compiled
        .artifact
        .hir
        .ports
        .iter()
        .map(|port| {
            if port.name == body.continuous {
                Ok(continuous.clone())
            } else if port.name == body.discrete {
                Ok(discrete.clone())
            } else if let Some(binding) = selected
                .supply_bindings
                .iter()
                .find(|binding| binding.port == port.name)
            {
                // A supply port reads the net it is bound to, live.
                Ok(xspice_auto_bridge_node_label(Some(&names), binding.node))
            } else {
                Err(refusal(format!(
                    "selected connect body declares unexpected port '{}'",
                    port.name
                )))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut instance_name = selected.instance.clone();
    if circuit
        .mixed_signal_hosts
        .iter()
        .any(|host| host.instance_name() == instance_name)
    {
        instance_name.push_str(&format!("__{event_node}"));
    }
    // The body converts against whatever it reads: its supply ports' nets,
    // or a supply the statement states.
    let mut record = super::converter_record(
        continuous,
        bridge,
        crate::circuit::ConverterKind::Authored,
        selected.name.clone(),
        instance_name.clone(),
        selected.rules(),
    );
    record.location = selected.location.clone();
    record.supply = match (selected.supply_net(), selected.stated_supply()) {
        (Some(net), _) => Some(crate::circuit::ConverterSupply::new(
            selected.stated_supply().unwrap_or(bridge.vcc),
            Some(net.to_owned()),
            crate::circuit::ConverterSource::ConnectStatement,
        )),
        (None, Some(level)) => Some(crate::circuit::ConverterSupply::new(
            level,
            None,
            crate::circuit::ConverterSource::ConnectStatement,
        )),
        (None, None) => None,
    };
    record.supply_ports = selected
        .supply_bindings
        .iter()
        .map(|binding| {
            crate::circuit::ConverterSupply::new(
                binding.level,
                Some(binding.net.clone()),
                binding.source.clone(),
            )
            .at_port(&binding.port)
        })
        .collect();
    if let Some(first) = record.supply_ports.first() {
        record.supply = Some(first.clone());
    }
    record.parameters = selected
        .parameters
        .iter()
        .map(|(name, value)| {
            crate::circuit::ConverterParameter::new(
                name,
                *value,
                "",
                None,
                crate::circuit::ConverterSource::ConnectStatement,
                false,
            )
        })
        .collect();
    let element = crate::netlist::Element {
        name: instance_name,
        kind: crate::netlist::ElementKind::Subcircuit {
            subckt_name: selected.name.clone(),
            params: Vec::new(),
        },
        nodes,
        provenance: Default::default(),
    };
    // All parameters have already participated in source elaboration. Applying
    // them again could alter $param_given or reinterpret exact folded values.
    let prepared = PreparedInstance {
        model: Arc::clone(&compiled.model),
        canonical_ir: Some(Arc::clone(&compiled.artifact)),
        overrides: Vec::new(),
        temperature,
        multiplicity: None,
    };
    let authored_nets = names.into_iter().collect();
    if !try_build_mixed_signal_instance(circuit, &element, &prepared, &authored_nets, abort)? {
        return Err(refusal(
            "selected connect body has no executable discrete port".into(),
        ));
    }
    log::info!("{record}");
    circuit.converter_insertions.push(record);
    Ok(())
}
