//! Bounded provisional analog samples and mixed-signal event history.

use super::*;

#[derive(Debug, Default)]
pub(super) struct LiveTransientAccumulator {
    pub(super) waveforms: Vec<LiveTransientWaveform>,
    digital_events: Vec<DigitalEventTraceEvidence>,
    real_events: Vec<RealEventTraceEvidence>,
    /// The run's declared buses, as the runner published them once.
    digital_buses: Vec<crate::state::DigitalBusEvidence>,
    retained_event_points: usize,
}

#[derive(Debug)]
pub(super) struct LiveTransientWaveform {
    name: String,
    pub(super) x: Vec<f64>,
    pub(super) y: Vec<f64>,
}

impl LiveTransientAccumulator {
    pub(super) const MAX_SOURCE_SAMPLES: usize = 8_192;
    pub(super) const COMPACTED_SOURCE_SAMPLES: usize =
        crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES;
    /// Live event points retained across all nodes while a run is in flight.
    ///
    /// An event history is exact — every point is a committed transition, so
    /// there is nothing in it a decimation could drop without lying about
    /// when a net changed. The provisional history therefore stops growing at
    /// this ceiling rather than being thinned, and the terminal result, which
    /// carries the whole schedule, replaces it at completion.
    pub(super) const MAX_LIVE_EVENT_POINTS: usize = 8_192;

    pub(super) fn clear(&mut self) {
        self.waveforms.clear();
        self.digital_events.clear();
        self.real_events.clear();
        self.digital_buses.clear();
        self.retained_event_points = 0;
    }

    pub(super) fn is_empty(&self) -> bool {
        self.waveforms.is_empty() && !self.has_events()
    }

    fn has_events(&self) -> bool {
        !self.digital_events.is_empty() || !self.real_events.is_empty()
    }

    pub(super) fn ingest(&mut self, deltas: Vec<TransientSampleDelta>) {
        for delta in deltas {
            if !delta.time.is_finite() {
                continue;
            }
            let mut samples = HashMap::with_capacity(delta.waveforms.len());
            let mut malformed = false;
            for sample in delta.waveforms {
                if !sample.value.is_finite() {
                    malformed = true;
                    break;
                }
                if samples.insert(sample.name, sample.value).is_some() {
                    malformed = true;
                    break;
                }
            }
            if malformed {
                continue;
            }
            // Events are per-node timelines, not columns of the shared analog
            // grid, so they are kept whenever the message itself is sound.
            // The alignment rule below governs only the grid.
            self.ingest_events(delta.time, delta.events, delta.real_events);
            self.ingest_buses(delta.buses);
            if samples.is_empty() {
                continue;
            }

            if self.waveforms.is_empty() {
                let mut samples = samples.into_iter().collect::<Vec<_>>();
                samples.sort_by(|left, right| left.0.cmp(&right.0));
                self.waveforms = samples
                    .into_iter()
                    .map(|(name, value)| LiveTransientWaveform {
                        name,
                        x: vec![delta.time],
                        y: vec![value],
                    })
                    .collect();
                continue;
            }
            if samples.len() != self.waveforms.len()
                || self.waveforms.iter().any(|waveform| {
                    !samples.contains_key(&waveform.name)
                        || waveform
                            .x
                            .last()
                            .is_some_and(|previous| *previous >= delta.time)
                })
            {
                // A live point is one aligned solver sample. Publishing a
                // partial or schema-changing delta would make differential
                // expressions combine different times, so reject the whole
                // provisional point. The terminal result remains authoritative.
                continue;
            }
            for waveform in &mut self.waveforms {
                waveform.x.push(delta.time);
                waveform.y.push(samples[&waveform.name]);
            }
        }
        self.compact_if_needed();
    }

    /// Fold one accepted point's changed event values into the provisional
    /// per-node histories.
    ///
    /// The history is change-compressed and strictly increasing in time, the
    /// same shape the engine records into a terminal result: a repeated value
    /// or a time that does not advance is dropped for that node alone, so one
    /// stale message cannot corrupt the nodes beside it.
    fn ingest_events(
        &mut self,
        time: f64,
        digital: Vec<TransientDigitalEventSample>,
        real: Vec<TransientRealEventSample>,
    ) {
        for event in digital {
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                return;
            }
            // The typed decoder is the bound: a code is one of the thirteen
            // XSPICE states or it is not a code, and asking the type that owns
            // the encoding leaves no second spelling of the ceiling to drift.
            if event.name.trim().is_empty()
                || rspice_core::xspice::DigitalValue::from_event_code(event.value_code).is_none()
            {
                continue;
            }
            let index = match self
                .digital_events
                .iter()
                .position(|trace| trace.node_name == event.name)
            {
                Some(index) => index,
                None => {
                    self.digital_events.push(DigitalEventTraceEvidence {
                        node_name: event.name,
                        points: Vec::new(),
                    });
                    self.digital_events.len() - 1
                }
            };
            let points = &mut self.digital_events[index].points;
            if points
                .last()
                .is_some_and(|last| last.time_s >= time || last.value_code == event.value_code)
            {
                continue;
            }
            points.push(DigitalEventPointEvidence {
                time_s: time,
                value_code: event.value_code,
            });
            self.retained_event_points += 1;
        }
        for event in real {
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                return;
            }
            if event.name.trim().is_empty() || !event.value.is_finite() {
                continue;
            }
            let index = match self
                .real_events
                .iter()
                .position(|trace| trace.node_name == event.name)
            {
                Some(index) => index,
                None => {
                    self.real_events.push(RealEventTraceEvidence {
                        node_name: event.name,
                        points: Vec::new(),
                    });
                    self.real_events.len() - 1
                }
            };
            let points = &mut self.real_events[index].points;
            if points
                .last()
                .is_some_and(|last| last.time_s >= time || last.value == event.value)
            {
                continue;
            }
            points.push(RealEventPointEvidence {
                time_s: time,
                value: event.value,
            });
            self.retained_event_points += 1;
        }
    }

    /// Adopt the run's bus declarations from the one message that carries
    /// them.
    ///
    /// The runner publishes the run-constant table once, so a later message
    /// carrying one is a second claim about the same run and is ignored
    /// rather than merged: the first one is what the engine declared, and a
    /// live viewer must not have to reconcile two.
    fn ingest_buses(&mut self, buses: Vec<TransientDigitalBusSample>) {
        if buses.is_empty() || !self.digital_buses.is_empty() {
            return;
        }
        self.digital_buses = buses
            .into_iter()
            .map(|bus| crate::state::DigitalBusEvidence {
                name: bus.name,
                msb: bus.msb,
                lsb: bus.lsb,
                members: bus.members,
                source: crate::state::DigitalBusSourceEvidence::Engine,
            })
            .collect();
    }

    /// The provisional event schedule as retained evidence, ordered by node
    /// name so the result digest is a function of the history alone.
    ///
    /// A history the validator would reject is offered as nothing at all,
    /// exactly as the terminal conversion does: the Events sheet must never
    /// have to decide whether its own evidence is usable.
    pub(super) fn event_payload(
        &self,
        analysis_type: AnalysisType,
    ) -> Option<AnalysisResultPayload> {
        if !self.has_events() {
            return None;
        }
        let mut digital_traces = self.digital_events.clone();
        digital_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
        let mut real_traces = self.real_events.clone();
        real_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
        // A bus whose members the run has not yet reported a value for is
        // withheld: it would be a declaration over traces that are not there,
        // which the validator refuses and which would take the whole
        // provisional payload down with it. The declaration arrives the frame
        // the last member first changes.
        let mut digital_buses = self
            .digital_buses
            .iter()
            .filter(|bus| {
                bus.members.iter().all(|member| {
                    digital_traces
                        .iter()
                        .any(|trace| &trace.node_name == member)
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        digital_buses.sort_by(|left, right| left.name.cmp(&right.name));
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            digital_buses,
        };
        payload
            .validate_for(analysis_type)
            .is_ok()
            .then_some(payload)
    }

    /// Bound the source arrays used to build the provisional live document.
    ///
    /// All traces retain one common time selection, so differential and
    /// derived expressions remain aligned. Each displayed trace contributes
    /// bucket extrema to that shared selection; deterministic evenly spaced
    /// points fill any spare capacity. The terminal solver result is untouched
    /// and replaces this provisional cache at completion.
    fn compact_if_needed(&mut self) {
        let Some(point_count) = self.waveforms.iter().map(|waveform| waveform.x.len()).min() else {
            return;
        };
        if point_count <= Self::MAX_SOURCE_SAMPLES || point_count <= 2 {
            return;
        }

        let target = Self::COMPACTED_SOURCE_SAMPLES.min(point_count).max(2);
        let maximum_extrema_series = ((target - 2) / 2).max(1);
        let extrema_series = self.waveforms.len().min(maximum_extrema_series).max(1);
        let bucket_count = ((target - 2) / (2 * extrema_series)).max(1);
        let interior = point_count - 2;
        let mut selected = std::collections::BTreeSet::new();
        selected.insert(0usize);
        selected.insert(point_count - 1);

        for bucket in 0..bucket_count {
            let start = 1 + bucket * interior / bucket_count;
            let end = 1 + (bucket + 1) * interior / bucket_count;
            if start >= end {
                continue;
            }
            for waveform in self.waveforms.iter().take(extrema_series) {
                let mut minimum = start;
                let mut maximum = start;
                for index in start + 1..end {
                    if waveform.y[index].total_cmp(&waveform.y[minimum]).is_lt() {
                        minimum = index;
                    }
                    if waveform.y[index].total_cmp(&waveform.y[maximum]).is_gt() {
                        maximum = index;
                    }
                }
                selected.insert(minimum);
                selected.insert(maximum);
            }
        }
        for slot in 1..target - 1 {
            if selected.len() >= target {
                break;
            }
            selected.insert(slot * (point_count - 1) / (target - 1));
        }
        let selected = selected.into_iter().take(target).collect::<Vec<_>>();
        for waveform in &mut self.waveforms {
            waveform.x = selected.iter().map(|index| waveform.x[*index]).collect();
            waveform.y = selected.iter().map(|index| waveform.y[*index]).collect();
        }
    }

    pub(super) fn source_analysis(
        &self,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        let waveforms = self
            .waveforms
            .iter()
            .enumerate()
            .map(|(index, waveform)| {
                WaveformData::new(
                    waveform.name.clone(),
                    waveform.x.clone(),
                    waveform.y.clone(),
                    SimulationController::color_for_index(index),
                )
            })
            .collect();
        let analysis = AnalysisResult::live_transient_partial(1, analysis_type, label)
            .with_waveforms(waveforms);
        match self.event_payload(analysis_type) {
            Some(payload) => analysis.with_result_payload(payload),
            None => analysis,
        }
    }
}
