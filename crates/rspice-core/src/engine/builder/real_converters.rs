//! Materialize the shipped real-valued connect modules (`r2e`, `e2r`,
//! `rbidir`): the converter a connect rule selecting one inserts on a real
//! boundary, and its insertion record.
//!
//! The declaration's defaults convert exactly as a real port no connect rule
//! reaches: `r2e` with an ideal drive and equal edges is the stock
//! `real_to_v` with that transition time, and `e2r` publishing every change
//! is the stock `v_to_real`. Anything else, and every `rbidir`, converts
//! through `xspice/models/connect_real.rs`.

use std::sync::Arc;

use super::connect_modules::{self, PlannedConnectModule};
use super::{
    PlannedXspiceAutoBridge, XspiceAutoBridgeContext, XspiceAutoBridgeKind, converter_record,
    xspice_auto_bridge_node_label, xspice_creation_error,
};
use crate::xspice::{AUTO_BRIDGE_PREFIX, CodeModel, CodeModelVectorParams, PortConnection};
use crate::{CircuitData, SimulationError};

/// The converter one real boundary takes, before it is instantiated.
struct Converter {
    /// The instance name's tag, which is the module's name.
    label: &'static str,
    kind: crate::circuit::ConverterKind,
    model: Arc<dyn CodeModel>,
    connections: Vec<PortConnection>,
    parameters: Vec<(String, f64)>,
    /// Whether port 1 is an ideal voltage output that needs a branch.
    branch: bool,
}

/// Materialize one real boundary whose connect rule selects a shipped real
/// module, and log its insertion record.
pub(super) fn materialize(
    circuit: &mut CircuitData,
    bridge: &PlannedXspiceAutoBridge,
    selected: &PlannedConnectModule,
    context: XspiceAutoBridgeContext<'_>,
) -> Result<(), SimulationError> {
    let event_node = bridge.event_node.unwrap_or(bridge.node);
    let names = context
        .node_names
        .map(<[String]>::to_vec)
        .unwrap_or_else(|| circuit.node_names_sorted());
    let node_label = xspice_auto_bridge_node_label(Some(&names), bridge.node);
    let law = connect_modules::real_converter_law(selected, bridge.kind)?;
    let stock = |name: &str| {
        circuit.xspice_registry.get(name).ok_or_else(|| {
            SimulationError::Circuit(format!("Failed to resolve the stock XSPICE model '{name}'"))
        })
    };

    let Converter {
        label,
        kind,
        model,
        connections,
        parameters,
        branch,
    } = match bridge.kind {
        XspiceAutoBridgeKind::RealToV => {
            let (rout, trise, tfall) = (law.value("rout"), law.value("trise"), law.value("tfall"));
            let ideal = rout == 0.0;
            let output = if ideal {
                PortConnection::Analog(bridge.node)
            } else {
                PortConnection::AnalogVector(vec![bridge.node])
            };
            let (model, parameters): (Arc<dyn CodeModel>, _) = if ideal && trise == tfall {
                (stock("real_to_v")?, vec![("transition_time".into(), trise)])
            } else {
                let mut parameters = vec![("trise".into(), trise), ("tfall".into(), tfall)];
                if !ideal {
                    parameters.push(("rout".into(), rout));
                }
                (
                    Arc::new(crate::xspice::models::ConnectR2e { ideal }),
                    parameters,
                )
            };
            Converter {
                label: "r2e",
                kind: crate::circuit::ConverterKind::R2e,
                model,
                connections: vec![PortConnection::Real(event_node), output],
                parameters,
                branch: ideal,
            }
        }
        XspiceAutoBridgeKind::VToReal => {
            let (vdelta, ttol) = (law.value("vdelta"), law.value("ttol"));
            let (model, parameters): (Arc<dyn CodeModel>, _) = if vdelta == 0.0 && ttol == 0.0 {
                (stock("v_to_real")?, Vec::new())
            } else {
                (
                    Arc::new(crate::xspice::models::ConnectE2r),
                    vec![("vdelta".into(), vdelta), ("ttol".into(), ttol)],
                )
            };
            Converter {
                label: "e2r",
                kind: crate::circuit::ConverterKind::E2r,
                model,
                connections: vec![
                    PortConnection::Analog(bridge.node),
                    PortConnection::Real(event_node),
                ],
                parameters,
                branch: false,
            }
        }
        XspiceAutoBridgeKind::RealBidi => Converter {
            label: "rbidir",
            kind: crate::circuit::ConverterKind::Rbidir,
            model: Arc::new(crate::xspice::models::ConnectRbidir),
            // The resolved net in, the receiving sample out on the same net,
            // the drive onto the node, and the node read back.
            connections: vec![
                PortConnection::Real(event_node),
                PortConnection::Real(event_node),
                PortConnection::AnalogVector(vec![bridge.node]),
                PortConnection::Analog(bridge.node),
            ],
            parameters: ["rout", "trise", "tfall", "vdelta", "ttol"]
                .into_iter()
                .map(|name| (name.to_string(), law.value(name)))
                .collect(),
            branch: false,
        },
        _ => {
            return Err(SimulationError::from(crate::ElaborationError::new(
                crate::ElaborationErrorKind::Internal,
                format!(
                    "node '{node_label}' reached the real converters with a {} boundary",
                    super::xspice_auto_bridge_kind_label(bridge.kind)
                ),
            )));
        }
    };

    let instance_name = format!("{AUTO_BRIDGE_PREFIX}{label}_{event_node}");
    let mut instance = crate::xspice::XspiceInstance::new_with_string_vectors(
        instance_name.clone(),
        model,
        connections,
        &parameters,
        CodeModelVectorParams {
            string_params: &[],
            string_vector_params: &[],
            real_vector_params: &[],
            integer_vector_params: &[],
        },
    )
    .map_err(|e| {
        xspice_creation_error(
            format!(
                "Failed to create the connect-rule converter '{instance_name}' on node \
                 '{node_label}'"
            ),
            e,
        )
    })?;
    instance.set_temperature(context.temperature);
    instance.set_ramptime(context.ramptime);
    instance.set_digital_delay_type(context.digital_delay_type);
    instance.set_resource_limits(context.resource_limits);
    if branch {
        let ordinal = circuit.allocate_branch_named(&format!("{instance_name}#out"));
        instance.set_output_branch(1, ordinal).map_err(|e| {
            SimulationError::Circuit(format!(
                "Failed to assign the branch of the connect-rule converter '{instance_name}': {e}"
            ))
        })?;
    }
    instance.init().map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to initialize the connect-rule converter '{instance_name}' on node \
             '{node_label}': {e}"
        ))
    })?;

    let mut record = converter_record(
        node_label,
        bridge,
        kind,
        selected.name.clone(),
        instance_name,
        selected.rules(),
    );
    record.location = selected.location.clone();
    record.parameters = law.record();
    log::info!("{record}");
    circuit.converter_insertions.push(record);
    circuit.add_xspice_instance(instance);
    Ok(())
}
