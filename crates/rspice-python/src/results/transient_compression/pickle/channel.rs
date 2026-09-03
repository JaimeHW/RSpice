//! Channel descriptor and per-sample validity projection.
//!
//! A retained channel carries its role, its unit, whether it was recorded at
//! all, and, per retained point, either a number or a typed absence reason.
//! The pickled form keeps those two apart in separate vectors so the validity
//! mask survives a round trip instead of collapsing into a placeholder zero.

use super::*;

/// One descriptor-keyed channel: role tag, node index, owner name, device
/// parameter, unit tag, availability tag, per-sample values, per-sample
/// absence reasons. A sample is present in exactly one of the last two
/// vectors, which is how the validity mask survives a round trip.
pub(crate) type CompressedChannelPersistenceState = (
    String,
    usize,
    String,
    String,
    String,
    String,
    Vec<Option<f64>>,
    Vec<Option<String>>,
);

fn channel_role_tag(role: &rspice_core::engine::TransientChannelRole) -> &'static str {
    use rspice_core::engine::TransientChannelRole;
    match role {
        TransientChannelRole::NodeVoltage { .. } => "node-voltage",
        TransientChannelRole::BranchCurrent { .. } => "branch-current",
        TransientChannelRole::DeviceObservable { .. } => "device-observable",
        TransientChannelRole::DeviceStore { .. } => "device-store",
    }
}

pub(crate) fn channel_persistence_state(
    channel: &rspice_core::engine::TransientCompressedChannel,
) -> CompressedChannelPersistenceState {
    use rspice_core::engine::TransientChannelRole;
    let role = channel.descriptor.role();
    let (node_index, owner, parameter) = match role {
        TransientChannelRole::NodeVoltage { node_index, node } => {
            (*node_index, node.clone(), String::new())
        }
        TransientChannelRole::BranchCurrent { branch } => (0, branch.clone(), String::new()),
        TransientChannelRole::DeviceObservable { device, parameter } => {
            (0, device.clone(), parameter.clone())
        }
        TransientChannelRole::DeviceStore { store } => (0, store.clone(), String::new()),
    };
    let values = channel
        .samples
        .iter()
        .map(|sample| sample.value())
        .collect::<Vec<_>>();
    let absence = channel
        .samples
        .iter()
        .map(|sample| sample.absence().map(|reason| reason.as_str().to_string()))
        .collect::<Vec<_>>();
    (
        channel_role_tag(role).to_string(),
        node_index,
        owner,
        parameter,
        channel.descriptor.unit().as_str().to_string(),
        channel.availability.as_str().to_string(),
        values,
        absence,
    )
}

pub(crate) fn rebuild_channel(
    state: CompressedChannelPersistenceState,
) -> PyResult<rspice_core::engine::TransientCompressedChannel> {
    use rspice_core::engine::{
        TransientChannelAvailability, TransientChannelDescriptor, TransientChannelRole,
        TransientChannelSample, TransientChannelUnit, TransientCompressedChannel,
        TransientSampleAbsence,
    };
    let (role_tag, node_index, owner, parameter, unit, availability, values, absence) = state;
    let role = match role_tag.as_str() {
        "node-voltage" => TransientChannelRole::NodeVoltage {
            node_index,
            node: owner,
        },
        "branch-current" => TransientChannelRole::BranchCurrent { branch: owner },
        "device-observable" => TransientChannelRole::DeviceObservable {
            device: owner,
            parameter,
        },
        "device-store" => TransientChannelRole::DeviceStore { store: owner },
        _ => {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient channel role '{role_tag}'"
            )));
        }
    };
    let unit = TransientChannelUnit::from_tag(&unit).map_err(crate::errors::value_error)?;
    let availability = TransientChannelAvailability::from_tag(&availability).ok_or_else(|| {
        crate::errors::value_error(format!(
            "unsupported compressed-transient channel availability '{availability}'"
        ))
    })?;
    if values.len() != absence.len() {
        return Err(crate::errors::value_error(format!(
            "compressed-transient channel pickle has {} values for {} validity entries",
            values.len(),
            absence.len()
        )));
    }
    let mut samples = Vec::with_capacity(values.len());
    for (index, (value, reason)) in values.into_iter().zip(absence).enumerate() {
        samples.push(match (value, reason) {
            (Some(value), None) => TransientChannelSample::Value(value),
            (None, Some(reason)) => TransientChannelSample::Absent(
                TransientSampleAbsence::from_tag(&reason).ok_or_else(|| {
                    crate::errors::value_error(format!(
                        "unsupported compressed-transient sample absence reason '{reason}'"
                    ))
                })?,
            ),
            (Some(_), Some(_)) => {
                return Err(crate::errors::value_error(format!(
                    "compressed-transient channel pickle sample {index} must be exactly one of a value and a typed absence, not both"
                )));
            }
            (None, None) => {
                return Err(crate::errors::value_error(format!(
                    "compressed-transient channel pickle sample {index} must be exactly one of a value and a typed absence, not neither"
                )));
            }
        });
    }
    Ok(TransientCompressedChannel {
        descriptor: TransientChannelDescriptor::new(role, unit)
            .map_err(crate::errors::value_error)?,
        availability,
        samples,
    })
}
