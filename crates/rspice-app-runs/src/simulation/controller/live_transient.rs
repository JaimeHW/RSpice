//! Bounded provisional analog samples and mixed-signal event history.

use super::*;

#[derive(Debug, Default)]
pub(super) struct LiveTransientAccumulator {
    current_impulses: rspice_simulation::live_transient::CurrentImpulseBuffer,
    voltage_impulses: rspice_simulation::live_transient::VoltageImpulseBuffer,
    pub(super) waveforms: Vec<LiveTransientWaveform>,
    digital_events: Vec<DigitalEventTraceEvidence>,
    real_events: Vec<RealEventTraceEvidence>,
    /// The run's declared buses, as the runner published them once.
    digital_buses: Vec<crate::state::DigitalBusEvidence>,
    retained_event_points: usize,
    events_lost: bool,
    event_loss_notice: bool,
    /// The preview could not be shown and is not tried again for this task.
    /// The samples are still kept, for a partial result.
    preview_stopped: bool,
    /// Analog points the live queue let go of while the host was busy, and the stretches of
    /// time they covered, each drawn as a gap in the live traces.
    dropped: u64,
    gaps: Vec<(f64, f64)>,
    /// The pixel columns the history was last thinned to, once it was.
    thinned_to: Option<usize>,
    /// The span of time the run fills, which the thinning columns divide: fixed for the run, so
    /// a column thinned again keeps the same least and greatest values.
    span: Option<(f64, f64)>,
}

/// What the live preview of a transient left out, for the status card to say.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LivePreviewGaps {
    /// Analog points the host was too busy to take, and the stretches of time they covered.
    pub dropped: u64,
    pub gaps: usize,
    /// The pixel columns the earlier part of the trace keeps every least and greatest value of,
    /// once the history was thinned to them.
    pub thinned_to: Option<usize>,
}

impl LivePreviewGaps {
    /// What the live plot leaves out, in words, when it leaves anything out.
    pub fn note(&self) -> Option<String> {
        let mut words = Vec::new();
        if self.dropped > 0 {
            words.push(format!(
                "The live plot could not keep up and left out {} point{}; the trace breaks \
                 where {} ({} gap{}).",
                super::run_lines::grouped(self.dropped),
                if self.dropped == 1 { "" } else { "s" },
                if self.dropped == 1 {
                    "it was"
                } else {
                    "they were"
                },
                self.gaps,
                if self.gaps == 1 { "" } else { "s" },
            ));
        }
        if let Some(columns) = self.thinned_to {
            words.push(format!(
                "Its earlier part is thinned to {columns} columns, each keeping its least \
                 and greatest value."
            ));
        }
        (!words.is_empty()).then(|| {
            words.push("The finished result has every point.".to_owned());
            words.join(" ")
        })
    }
}

#[derive(Debug)]
pub(super) struct LiveTransientWaveform {
    name: String,
    pub(super) x: Vec<f64>,
    pub(super) y: Vec<f64>,
}

impl LiveTransientAccumulator {
    /// The values the provisional history holds across its signals before it is thinned: what
    /// a frame may copy into the live preview.
    pub(super) const HISTORY_VALUES: usize = 1 << 18;
    /// The columns the history is thinned to: more than the widest plot has pixels across its
    /// data. Every signal keeps its least and greatest value in each column.
    pub(super) const THIN_COLUMNS: usize = 4_096;
    /// The fewest columns thinning goes down to when the signals' extremes fall on so many
    /// different points that the history would not fit at more.
    const FEWEST_COLUMNS: usize = 64;
    /// Live event points retained across all nodes while a run is in flight.
    ///
    /// An event history is exact — every point is a committed transition, so
    /// there is nothing in it a decimation could drop without lying about
    /// when a net changed. The provisional history therefore stops growing at
    /// this ceiling rather than being thinned, and the terminal result, which
    /// carries the whole schedule, replaces it at completion.
    pub(super) const MAX_LIVE_EVENT_POINTS: usize = 8_192;

    pub(super) fn clear(&mut self) {
        self.current_impulses.clear();
        self.voltage_impulses.clear();
        self.waveforms.clear();
        self.digital_events.clear();
        self.real_events.clear();
        self.digital_buses.clear();
        self.retained_event_points = 0;
        self.events_lost = false;
        self.event_loss_notice = false;
        self.preview_stopped = false;
        self.dropped = 0;
        self.gaps.clear();
        self.thinned_to = None;
        self.span = None;
    }

    /// The span of time the running transient fills, from where its result starts to its stop
    /// time.
    pub(super) fn set_span(&mut self, span: (f64, f64)) {
        if span.0.is_finite() && span.1 > span.0 {
            self.span = Some(span);
        }
    }

    /// Take in the rows a run's segment held, a block at a time, as the history the live
    /// preview had drawn from it: thinned as it grows, so it stays as small as one taken from the
    /// live queue.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn ingest_rows(
        &mut self,
        waveforms: &[rspice_results::waveform::RetainedWaveform],
    ) -> Result<(), String> {
        const BLOCK_ROWS: usize = 1 << 16;
        let Some(first) = waveforms.first() else {
            return Ok(());
        };
        let rows = first.x.len();
        let failed = |error: rspice_results::waveform::ColumnReadFailure| error.to_string();
        self.waveforms = waveforms
            .iter()
            .map(|waveform| LiveTransientWaveform {
                name: waveform.name.clone(),
                x: Vec::new(),
                y: Vec::new(),
            })
            .collect();
        let mut start = 0;
        while start < rows {
            let end = (start + BLOCK_ROWS).min(rows);
            let x = first.x.read_range(start..end).map_err(failed)?;
            for (history, waveform) in self.waveforms.iter_mut().zip(waveforms) {
                let y = waveform.y.read_range(start..end).map_err(failed)?;
                history.x.extend_from_slice(&x);
                history.y.extend_from_slice(&y);
            }
            self.thin_if_needed();
            start = end;
        }
        Ok(())
    }

    /// What the live preview left out so far.
    pub(super) fn gaps(&self) -> LivePreviewGaps {
        LivePreviewGaps {
            dropped: self.dropped,
            gaps: self.gaps.len(),
            thinned_to: self.thinned_to,
        }
    }

    pub(super) fn stop_preview(&mut self) {
        self.preview_stopped = true;
    }

    pub(super) fn preview_stopped(&self) -> bool {
        self.preview_stopped
    }

    fn mark_event_loss(&mut self) {
        if !self.events_lost {
            self.event_loss_notice = true;
        }
        self.events_lost = true;
    }

    pub(super) fn take_event_loss_notice(&mut self) -> bool {
        std::mem::take(&mut self.event_loss_notice)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.waveforms.is_empty() && !self.has_events()
    }

    fn has_events(&self) -> bool {
        !self.digital_events.is_empty()
            || !self.real_events.is_empty()
            || !self.current_impulses.is_empty()
            || !self.voltage_impulses.is_empty()
    }

    pub(super) fn ingest(&mut self, deltas: Vec<TransientSampleDelta>) {
        for delta in deltas {
            if !delta.event_delivery_complete {
                self.mark_event_loss();
            }
            if !delta.time.is_finite() {
                self.current_impulses.mark_lost();
                self.voltage_impulses.mark_lost();
                self.mark_event_loss();
                continue;
            }
            // Charge has its own exact time axis; malformed or compacted
            // analog columns cannot erase a valid current observation.
            if let Some(impulses) = delta.current_impulses {
                if impulses.stop_time_s == delta.time {
                    self.current_impulses.ingest(impulses);
                } else {
                    self.current_impulses.mark_lost();
                }
            }
            if let Some(impulses) = delta.voltage_impulses {
                if impulses.stop_time_s == delta.time {
                    self.voltage_impulses.ingest(impulses);
                } else {
                    self.voltage_impulses.mark_lost();
                }
            }
            if !self.events_lost {
                self.ingest_events(delta.time, delta.events, delta.real_events);
                self.ingest_buses(delta.buses);
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
            // Points the queue let go of leave a gap: an unavailable sample where the first of
            // them was, so the trace breaks there instead of joining across them.
            if let Some(dropped) = delta.dropped_before {
                self.dropped += dropped.count;
                self.gaps.push((dropped.first_s, dropped.last_s));
                let after_last = self
                    .waveforms
                    .first()
                    .and_then(|waveform| waveform.x.last())
                    .is_none_or(|last| *last < dropped.first_s);
                if after_last && dropped.first_s < delta.time {
                    for waveform in &mut self.waveforms {
                        waveform.x.push(dropped.first_s);
                        waveform.y.push(f64::NAN);
                    }
                }
            }
            for waveform in &mut self.waveforms {
                waveform.x.push(delta.time);
                waveform.y.push(samples[&waveform.name]);
            }
        }
        self.thin_if_needed();
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
        endpoint: f64,
        digital: Vec<TransientDigitalEventSample>,
        real: Vec<TransientRealEventSample>,
    ) {
        for event in digital {
            let time = event.time_s.unwrap_or(endpoint);
            if !time.is_finite() || time < 0.0 || time > endpoint {
                self.mark_event_loss();
                return;
            }
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                self.mark_event_loss();
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
                        points: Default::default(),
                    });
                    self.digital_events.len() - 1
                }
            };
            let points = self.digital_events[index].points.make_mut();
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
            let time = event.time_s.unwrap_or(endpoint);
            if !time.is_finite() || time < 0.0 || time > endpoint {
                self.mark_event_loss();
                return;
            }
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                self.mark_event_loss();
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
                        points: Default::default(),
                    });
                    self.real_events.len() - 1
                }
            };
            let points = self.real_events[index].points.make_mut();
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
            current_impulses: self.current_impulses.history(),
            voltage_impulses: self.voltage_impulses.history(),
            digital_traces,
            real_traces,
            digital_buses,
            hdl_sources: Vec::new(),
        };
        payload
            .validate_for(analysis_type)
            .is_ok()
            .then_some(payload)
    }

    /// Bound the history the live preview is built from.
    ///
    /// All traces keep one common time selection, so differential and derived outputs combine
    /// samples of the same instant. Once the history holds more values than a frame should copy,
    /// it is thinned to pixel columns over its span: every signal keeps its least and greatest
    /// value in every column, with the first and last points and every gap. When the signals'
    /// extremes fall on so many different points that the selection would not fit, the columns
    /// are halved until it does, and the preview says how many it kept. The terminal result
    /// replaces this history whole at completion.
    fn thin_if_needed(&mut self) {
        let Some(point_count) = self.waveforms.iter().map(|waveform| waveform.x.len()).min() else {
            return;
        };
        let signals = self.waveforms.len().max(1);
        let budget = (Self::HISTORY_VALUES / signals).max(4 * Self::THIN_COLUMNS);
        if point_count <= budget {
            return;
        }
        let mut columns = Self::THIN_COLUMNS;
        let selected = loop {
            let selected = self.column_extrema(point_count, columns);
            if selected.len() <= budget / 2 || columns <= Self::FEWEST_COLUMNS {
                break selected;
            }
            columns /= 2;
        };
        self.thinned_to = Some(
            self.thinned_to
                .map_or(columns, |before| before.min(columns)),
        );
        for waveform in &mut self.waveforms {
            waveform.x = selected.iter().map(|index| waveform.x[*index]).collect();
            waveform.y = selected.iter().map(|index| waveform.y[*index]).collect();
        }
    }

    /// The points that keep every signal's least and greatest value in each of `columns` equal
    /// spans of time, with the first and last points and every gap, in time order.
    fn column_extrema(&self, point_count: usize, columns: usize) -> Vec<usize> {
        let Some(axis) = self.waveforms.first().map(|waveform| &waveform.x) else {
            return Vec::new();
        };
        let (first, last) = self.span.unwrap_or((axis[0], axis[point_count - 1]));
        let span = last - first;
        let column_of = |x: f64| {
            if span > 0.0 {
                (((x - first) / span * columns as f64).max(0.0) as usize).min(columns - 1)
            } else {
                0
            }
        };
        let mut selected = vec![0, point_count - 1];
        let mut start = 0;
        while start < point_count {
            let column = column_of(axis[start]);
            let mut end = start + 1;
            while end < point_count && column_of(axis[end]) == column {
                end += 1;
            }
            for waveform in &self.waveforms {
                let (mut minimum, mut maximum) = (None::<usize>, None::<usize>);
                for index in start..end {
                    let value = waveform.y[index];
                    if value.is_nan() {
                        selected.push(index);
                        continue;
                    }
                    if minimum.is_none_or(|at| value < waveform.y[at]) {
                        minimum = Some(index);
                    }
                    if maximum.is_none_or(|at| value > waveform.y[at]) {
                        maximum = Some(index);
                    }
                }
                selected.extend(minimum);
                selected.extend(maximum);
            }
            start = end;
        }
        selected.sort_unstable();
        selected.dedup();
        selected
    }

    pub(super) fn source_analysis(
        &self,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        let waveforms = self
            .waveforms
            .iter()
            .map(|waveform| {
                RetainedTrace::new(
                    waveform.name.clone(),
                    waveform.x.clone(),
                    waveform.y.clone(),
                    "",
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
