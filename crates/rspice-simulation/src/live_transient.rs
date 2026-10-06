//! Stateful publication and bounded delivery of accepted transient samples.
//!
//! Native and browser hosts share event-change tracking, sparse current updates
//! and queue retention. Complete terminal results remain on their own path.

use std::sync::{Arc, Mutex};

mod impulses;
use impulses::PublishedCurrentImpulses;
pub use impulses::{
    CurrentImpulseBuffer, CurrentImpulseDelta, CurrentImpulseUpdate, LiveTransientQueue,
};

/// Maximum UI-only transient deltas waiting for an application frame.
///
/// The solver owns the authoritative full result. This queue exists only for
/// live presentation, so a background tab or suspended browser must not let it
/// grow to the full multi-million-point analysis ceiling. When it fills, the
/// oldest undisplayed point is replaced by newer evidence; terminal retention
/// remains lossless and atomically replaces the live document.
const MAX_PENDING_LIVE_TRANSIENT_SAMPLES: usize = 8_192;

/// One fully accepted transient point published by the engine while the
/// producing analysis is still running. Only retained analog traces are
/// included, so the message is compact and has the same names as the final
/// result conversion.
///
/// The event fields carry only the nodes whose committed value differs from
/// the one this run last published. The engine reports its whole committed
/// event state at every accepted point, which for a settled net is the same
/// value thousands of times over; a live message that repeated it would cost
/// a name and a number per node per step to say nothing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientSampleDelta {
    pub time: f64,
    pub waveforms: Vec<TransientWaveformSample>,
    /// Digital event nodes that changed at this accepted time.
    #[serde(default)]
    pub events: Vec<TransientDigitalEventSample>,
    /// Real-valued event nodes that changed at this accepted time.
    #[serde(default)]
    pub real_events: Vec<TransientRealEventSample>,
    /// The run's digital bus declarations, published once.
    ///
    /// The table is run-constant — the engine hands the same borrowed slice
    /// at every accepted point — so it rides the first message that can name
    /// its members and is empty on every one after. Repeating it would cost a
    /// name per member per step to say what the first message already said.
    #[serde(default)]
    pub buses: Vec<TransientDigitalBusSample>,
    /// Newly accepted charge observations, independently timed and sequenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_impulses: Option<CurrentImpulseDelta>,
}

/// One digital bus a run declared, as it crosses to the live viewer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientDigitalBusSample {
    pub name: String,
    pub msb: i64,
    pub lsb: i64,
    /// Member node names, declared MSB first — the engine's node ids resolved
    /// through the run's node table, because a live viewer holds names.
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientWaveformSample {
    pub name: String,
    pub value: f64,
    pub y_unit: String,
}

/// One digital event node's newly committed value at an accepted point.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientDigitalEventSample {
    pub name: String,
    /// XSPICE 12-state event code, the same encoding the terminal result's
    /// event evidence carries.
    pub value_code: u8,
}

/// One real-valued event node's newly committed value at an accepted point.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientRealEventSample {
    pub name: String,
    pub value: f64,
}

/// A run's live sample publisher, shared by queue and callback delivery.
/// Construct one for each run so event and impulse cursors start empty.
pub struct LiveTransientPublisher {
    transient_samples: Option<Arc<Mutex<LiveTransientQueue>>>,
    transient_sample_observer: Option<TransientSampleObserver>,
    published_events: Mutex<PublishedEventValues>,
}

pub type TransientSampleObserver = fn(&TransientSampleDelta);

impl LiveTransientPublisher {
    pub fn new(
        transient_samples: Option<Arc<Mutex<LiveTransientQueue>>>,
        transient_sample_observer: Option<TransientSampleObserver>,
    ) -> Self {
        Self {
            transient_samples,
            transient_sample_observer,
            published_events: Mutex::default(),
        }
    }

    pub fn observe(&self, sample: rspice_core::abort_signal::TransientSample<'_>) {
        if self.transient_samples.is_none() && self.transient_sample_observer.is_none() {
            return;
        }
        let Some(&time) = sample.time.last() else {
            return;
        };
        let mut waveforms = Vec::with_capacity(
            sample
                .node_voltages
                .len()
                .saturating_add(sample.branch_currents.len()),
        );
        for (index, values) in sample.node_voltages.iter().enumerate() {
            let Some(&value) = values.last() else {
                continue;
            };
            let name = sample
                .node_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| (index + 1).to_string());
            waveforms.push(TransientWaveformSample {
                name,
                value,
                y_unit: "V".to_owned(),
            });
        }
        for (index, values) in sample.branch_currents.iter().enumerate() {
            let Some(&value) = values.last() else {
                continue;
            };
            let branch = sample
                .branch_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("branch{}", index + 1));
            let name = if branch.len() >= 3
                && (branch.starts_with("I(") || branch.starts_with("i("))
                && branch.ends_with(')')
            {
                branch
            } else {
                format!("I({branch})")
            };
            waveforms.push(TransientWaveformSample {
                name,
                value,
                y_unit: "A".to_owned(),
            });
        }
        let (events, real_events) = self.changed_event_values(&sample);
        let buses = self.declared_buses(&sample);
        let current_impulses = match self.published_events.lock() {
            Ok(mut published) => published.currents.publish(&sample),
            Err(poisoned) => poisoned.into_inner().currents.publish(&sample),
        };
        let delta = TransientSampleDelta {
            time,
            waveforms,
            events,
            real_events,
            buses,
            current_impulses,
        };
        if let Some(samples) = &self.transient_samples {
            push_live_transient_sample(samples, delta.clone());
        }
        if let Some(observer) = self.transient_sample_observer {
            observer(&delta);
        }
    }
}

/// The last event value this run published for each node, so an accepted
/// point can report changes instead of the whole committed state.
///
/// One signal serves one analysis — `run_simulation_thread` builds it after
/// the deck is prepared and drops it when the engine returns — so the map is
/// empty at the first accepted point of every run without being cleared.
/// [`AbortSignal`](rspice_core::abort_signal::AbortSignal) observes through a
/// shared reference, hence the lock; it is uncontended, because only the
/// solver thread reports samples.
#[derive(Debug, Default)]
struct PublishedEventValues {
    currents: PublishedCurrentImpulses,
    digital: std::collections::HashMap<rspice_core::NodeId, u8>,
    real: std::collections::HashMap<rspice_core::NodeId, u64>,
    /// Whether this run has already published its bus declarations.
    buses: bool,
}

/// The netlist name of an event node, or `None` when the node table does not
/// cover it. Event values are keyed by node id, where ground is zero and never
/// appears, so `node_names[node_id - 1]` is the node's name.
fn event_node_name(node_names: &[String], node: rspice_core::NodeId) -> Option<&str> {
    node_names
        .get(node.checked_sub(1)?)
        .map(String::as_str)
        .filter(|name| !name.trim().is_empty())
}

pub fn push_live_transient_sample(
    samples: &Arc<Mutex<LiveTransientQueue>>,
    delta: TransientSampleDelta,
) {
    let mut samples = match samples.lock() {
        Ok(samples) => samples,
        Err(poisoned) => poisoned.into_inner(),
    };
    samples.push(delta);
}

impl LiveTransientPublisher {
    /// The event nodes whose committed value differs from the one this run
    /// last published, and the record of what was published updated to match.
    ///
    /// A node the node table cannot name is skipped without being recorded:
    /// it stays a candidate, so the first accepted point at which a name does
    /// resolve reports it rather than treating it as already seen.
    fn changed_event_values(
        &self,
        sample: &rspice_core::abort_signal::TransientSample<'_>,
    ) -> (
        Vec<TransientDigitalEventSample>,
        Vec<TransientRealEventSample>,
    ) {
        if sample.digital_values.is_empty() && sample.real_values.is_empty() {
            return (Vec::new(), Vec::new());
        }
        let mut published = match self.published_events.lock() {
            Ok(published) => published,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut events = Vec::new();
        for &(node, code) in sample.digital_values {
            let Some(name) = event_node_name(sample.node_names, node) else {
                continue;
            };
            if published.digital.insert(node, code.0) == Some(code.0) {
                continue;
            }
            events.push(TransientDigitalEventSample {
                name: name.to_owned(),
                value_code: code.0,
            });
        }
        let mut real_events = Vec::new();
        for &(node, value) in sample.real_values {
            let Some(name) = event_node_name(sample.node_names, node) else {
                continue;
            };
            // Compared as bits so the record is an exact account of what was
            // published: two values that are not equal as floats are never
            // treated as the same reported value.
            let bits = value.to_bits();
            if published.real.insert(node, bits) == Some(bits) {
                continue;
            }
            real_events.push(TransientRealEventSample {
                name: name.to_owned(),
                value,
            });
        }
        (events, real_events)
    }

    /// The run's bus declarations, the first time they can be stated.
    ///
    /// The engine's table names members by node id; a live message names them
    /// the way the terminal evidence does, so a bus whose members the node
    /// table cannot name yet is not published *and not recorded* — the next
    /// accepted point tries again rather than the run losing its table to one
    /// early sample.
    fn declared_buses(
        &self,
        sample: &rspice_core::abort_signal::TransientSample<'_>,
    ) -> Vec<TransientDigitalBusSample> {
        if sample.digital_buses.is_empty() {
            return Vec::new();
        }
        let mut published = match self.published_events.lock() {
            Ok(published) => published,
            Err(poisoned) => poisoned.into_inner(),
        };
        if published.buses {
            return Vec::new();
        }
        let mut buses = Vec::with_capacity(sample.digital_buses.len());
        for bus in sample.digital_buses {
            let mut members = Vec::with_capacity(bus.members.len());
            for &node in &bus.members {
                let Some(name) = event_node_name(sample.node_names, node) else {
                    return Vec::new();
                };
                members.push(name.to_owned());
            }
            buses.push(TransientDigitalBusSample {
                name: bus.name.clone(),
                msb: bus.msb,
                lsb: bus.lsb,
                members,
            });
        }
        published.buses = true;
        buses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publisher_streams_live_current_impulses_as_sparse_accepted_suffixes() {
        use rspice_core::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = LiveTransientPublisher::new(Some(Arc::clone(&samples)), None);
        let mut traces = vec![CurrentImpulseTrace {
            derivatives: Vec::new(),
            owner: CurrentImpulseOwner::DeviceLead {
                device_name: "Q1".into(),
                parameter: "ic".into(),
            },
            complete: true,
            points: vec![],
        }];
        let mut received = CurrentImpulseBuffer::default();
        for index in 0..4 {
            if index == 1 || index == 3 {
                traces[0].points.push(CurrentImpulsePoint {
                    time: index as f64 - 0.2,
                    charge_coulombs: if index == 1 { -0.002 } else { 0.004 },
                });
            }
            LiveTransientPublisher::observe(
                &signal,
                rspice_core::abort_signal::TransientSample {
                    time: &[0.0, index as f64],
                    node_names: &[],
                    node_voltages: &[],
                    branch_names: &[],
                    branch_currents: &[],
                    current_impulses: Some(&traces),
                    digital_values: &[],
                    digital_buses: &[],
                    real_values: &[],
                },
            );
            let mut drained = samples.lock().unwrap().drain();
            let delta = drained.pop().unwrap().current_impulses.unwrap();
            if index == 2 {
                assert!(
                    delta.traces.is_empty(),
                    "unchanged cumulative history is not retransmitted"
                );
            }
            if index == 3 {
                assert_eq!(
                    delta.traces[0].points.len(),
                    1,
                    "only the new suffix crosses the boundary"
                );
            }
            received.ingest(delta);
        }
        let history = received.history().unwrap();
        history.validate().unwrap();
        assert!(history.delivery_complete);
        assert_eq!(history.traces, traces);
    }

    #[test]
    fn publisher_publishes_only_the_latest_committed_transient_point() {
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = LiveTransientPublisher::new(Some(Arc::clone(&samples)), None);
        let result = rspice_core::engine::TransientResult {
            current_impulses: None,
            time: vec![0.0, 2.5e-9],
            step_sizes: vec![0.0, 2.5e-9],
            voltages: vec![vec![0.0, 1.25], Vec::new()],
            branch_currents: vec![vec![0.0, -2.0e-3]],
            num_nodes: 2,
            node_names: vec!["out".to_owned(), "discarded".to_owned()],
            branch_names: vec!["V1".to_owned()],
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        LiveTransientPublisher::observe(
            &signal,
            rspice_core::abort_signal::TransientSample {
                time: &result.time,
                node_names: &result.node_names,
                node_voltages: &result.voltages,
                branch_names: &result.branch_names,
                branch_currents: &result.branch_currents,
                current_impulses: result.current_impulses.as_deref(),
                digital_values: &[],
                digital_buses: &[],
                real_values: &[],
            },
        );

        let sample = samples.lock().unwrap().drain().remove(0);
        assert_eq!(sample.time, 2.5e-9);
        assert_eq!(sample.waveforms.len(), 2);
        assert_eq!(sample.waveforms[0].name, "out");
        assert_eq!(sample.waveforms[0].value, 1.25);
        assert_eq!(sample.waveforms[0].y_unit, "V");
        assert_eq!(sample.waveforms[1].name, "I(V1)");
        assert_eq!(sample.waveforms[1].value, -2.0e-3);
        assert_eq!(sample.waveforms[1].y_unit, "A");
        assert!(sample.events.is_empty());
        assert!(sample.real_events.is_empty());
    }

    #[test]
    fn publisher_publishes_event_nodes_only_when_their_value_changes() {
        use rspice_core::abort_signal::{DigitalEventCode, TransientSample};

        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = LiveTransientPublisher::new(Some(Arc::clone(&samples)), None);
        let node_names = vec!["clk".to_owned(), "d".to_owned(), "vsense".to_owned()];
        let voltages = vec![vec![0.0, 0.0, 0.0], Vec::new(), Vec::new()];
        // Node 4 has no entry in the node table, so it cannot be named and is
        // never reported; nodes 1 and 2 are digital, node 3 is real-valued.
        let observe = |time: &[f64],
                       digital: &[(rspice_core::NodeId, DigitalEventCode)],
                       real: &[(rspice_core::NodeId, f64)]| {
            signal.observe(TransientSample {
                time,
                node_names: &node_names,
                node_voltages: &voltages,
                branch_names: &[],
                branch_currents: &[],
                current_impulses: None,
                digital_values: digital,
                digital_buses: &[],
                real_values: real,
            });
        };

        observe(
            &[0.0],
            &[(1, DigitalEventCode(0)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 2.5)],
        );
        observe(
            &[0.0, 1.0e-9],
            &[(1, DigitalEventCode(0)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 2.5)],
        );
        observe(
            &[0.0, 1.0e-9, 2.0e-9],
            &[(1, DigitalEventCode(1)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 9.0)],
        );

        let queued = samples.lock().expect("live queue").drain();
        assert_eq!(queued.len(), 3);

        assert_eq!(
            queued[0].events,
            vec![
                TransientDigitalEventSample {
                    name: "clk".to_owned(),
                    value_code: 0,
                },
                TransientDigitalEventSample {
                    name: "d".to_owned(),
                    value_code: 12,
                },
            ]
        );
        assert_eq!(
            queued[0].real_events,
            vec![TransientRealEventSample {
                name: "vsense".to_owned(),
                value: 1.5,
            }]
        );

        assert!(
            queued[1].events.is_empty() && queued[1].real_events.is_empty(),
            "an unchanged committed state must publish nothing"
        );

        assert_eq!(
            queued[2].events,
            vec![TransientDigitalEventSample {
                name: "clk".to_owned(),
                value_code: 1,
            }]
        );
        assert!(
            queued[2].real_events.is_empty(),
            "the only real node that changed has no name in the node table"
        );
    }

    #[test]
    fn live_transient_queue_is_bounded_for_suspended_ui_consumers() {
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        for index in 0..MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 17 {
            push_live_transient_sample(
                &samples,
                TransientSampleDelta {
                    current_impulses: None,
                    time: index as f64,
                    waveforms: Vec::new(),
                    events: Vec::new(),
                    real_events: Vec::new(),
                    buses: Vec::new(),
                },
            );
        }

        let samples = samples.lock().expect("live queue");
        assert_eq!(samples.samples.len(), MAX_PENDING_LIVE_TRANSIENT_SAMPLES);
        assert_eq!(samples.samples.front().expect("oldest retained").time, 17.0);
        assert_eq!(
            samples.samples.back().expect("latest retained").time,
            (MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 16) as f64
        );
    }
}
