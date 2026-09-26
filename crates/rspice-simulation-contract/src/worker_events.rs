//! Worker event-history packets and lossless retained-result conversions.

use crate::worker_result_values::{WorkerDigitalEventTrace, WorkerRealEventTrace};
use rspice_results::current_impulses::CurrentImpulseHistoryEvidence;
use rspice_results::events::{DigitalBusEvidence, DigitalBusSourceEvidence, TransientEventHistory};
use serde::{Deserialize, Serialize};

/// One digital bus declared over the digital traces beside it, on the wire.
///
/// The declaration crosses; the word does not. Reassembling a bus is
/// `rspice_core::execution::bus_events` reading the member histories, so a
/// worker and its host can never disagree about what a bus held.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerDigitalBus {
    pub name: String,
    pub msb: i64,
    pub lsb: i64,
    pub members: Vec<String>,
    pub source: DigitalBusSourceEvidence,
}

/// Every event node a transient run committed, on the wire.
///
/// Missing legacy fields retain unavailable history. The response protocol
/// version prevents a stale worker from silently omitting new observations.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkerEventHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_impulses: Option<CurrentImpulseHistoryEvidence>,
    #[serde(default)]
    pub digital: Vec<WorkerDigitalEventTrace>,
    #[serde(default)]
    pub real: Vec<WorkerRealEventTrace>,
    /// Buses declared over `digital`. Defaulted so a protocol-14 worker's
    /// response, which could not have declared one, reads as declaring none.
    #[serde(default)]
    pub buses: Vec<WorkerDigitalBus>,
}

impl From<TransientEventHistory> for WorkerEventHistory {
    fn from(value: TransientEventHistory) -> Self {
        Self {
            current_impulses: value.current_impulses,
            digital: value.digital.into_iter().map(Into::into).collect(),
            real: value.real.into_iter().map(Into::into).collect(),
            buses: value
                .digital_buses
                .into_iter()
                .map(|bus| WorkerDigitalBus {
                    name: bus.name,
                    msb: bus.msb,
                    lsb: bus.lsb,
                    members: bus.members,
                    source: bus.source,
                })
                .collect(),
        }
    }
}

impl From<WorkerEventHistory> for TransientEventHistory {
    fn from(value: WorkerEventHistory) -> Self {
        Self {
            current_impulses: value.current_impulses,
            digital: value.digital.into_iter().map(Into::into).collect(),
            real: value.real.into_iter().map(Into::into).collect(),
            digital_buses: value
                .buses
                .into_iter()
                .map(|bus| DigitalBusEvidence {
                    name: bus.name,
                    msb: bus.msb,
                    lsb: bus.lsb,
                    members: bus.members,
                    source: bus.source,
                })
                .collect(),
        }
    }
}
