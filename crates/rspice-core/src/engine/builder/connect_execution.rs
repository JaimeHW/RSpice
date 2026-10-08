//! Executable authored connection bodies in the circuit's existing mixed host.

use std::sync::{Arc, OnceLock};

use super::connect_modules::PlannedConnectModule;
use super::mixed_modules::try_build_mixed_signal_instance;
use super::veriloga_instances::PreparedInstance;
use super::{PlannedXspiceAutoBridge, check_build_abort, xspice_auto_bridge_node_label};
use crate::{CircuitData, ElaborationError, ElaborationErrorKind, SimulationError};

#[derive(Debug)]
pub(super) enum ConnectExecution {
    Delegated,
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
    source: Arc<str>,
    source_package: String,
    continuous: String,
    discrete: String,
    compiled: OnceLock<CompiledConnectBody>,
}

impl AuthoredConnectBody {
    pub(super) fn new(
        source: Arc<str>,
        source_package: String,
        continuous: String,
        discrete: String,
    ) -> Self {
        Self {
            source,
            source_package,
            continuous,
            discrete,
            compiled: OnceLock::new(),
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
        let runtime = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
            enable_ams: true,
            ..Default::default()
        })
        .compile_connect_runtime(
            &body.source_package,
            &body.source,
            &selected.name,
            &parameters,
            &super::veriloga_cache::VerilogACompileControl { abort },
        )
        .map_err(|error| {
            if abort.is_aborted() {
                SimulationError::Aborted
            } else {
                refusal(format!(
                    "selected connect body could not be compiled: {error}"
                ))
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
    Ok(())
}
