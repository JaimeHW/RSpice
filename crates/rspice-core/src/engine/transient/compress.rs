//! Waveform compression of an accepted transient result: a multi-channel Ramer-Douglas-Peucker
//! decimation that keeps every sample an authored output schedule reads, with every discarded
//! sample checked against the linearly interpolated retained waveform.

use super::*;

impl Engine {
    /// Compress one already-published accepted transient result.
    ///
    /// The typed `.FFT`, `.FOUR`, and `.MEASURE` products are evaluated on the
    /// exact accepted trajectory and stored in the container, so a consumer
    /// never has to recompute them from a decimated expansion. The authored
    /// output schedule is read from `netlist`, so the retained grid keeps every
    /// sample an `OUTPUTTIMEPOINTS` or `INITIAL_INTERVAL` schedule reads.
    ///
    /// This is the composition point for a caller that already holds an
    /// accepted result — an authored `.OPTIONS RESTART` segment, or a replayed
    /// run — and wants the same container the compressed entry points build.
    pub fn compress_transient_result_with_abort(
        &self,
        netlist: &Netlist,
        result: &TransientResult,
        compression: &CompressionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResultCompressed, SimulationError> {
        self.compress_published_transient(netlist, result, compression, abort)
    }

    pub(super) fn compress_published_transient(
        &self,
        netlist: &Netlist,
        result: &TransientResult,
        compression: &CompressionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResultCompressed, SimulationError> {
        let limits = self.config.resource_limits;
        let post_results = evaluate_transient_post_results(netlist, result, limits, abort)?;
        let schedule = TransientCompressionOutputSchedule::from_netlist(
            netlist,
            result,
            limits.max_result_values,
        );
        compress_transient_result(
            result,
            compression,
            &schedule,
            post_results,
            TransientResultIdentity::default(),
            abort,
        )
    }
}

/// Output schedule a compressed retained grid must be able to reproduce.
///
/// Compression composes with an authored output schedule by treating every
/// accepted sample the schedule reads as mandatory: `OUTPUTTIMEPOINTS` stops
/// are exact accepted solver points, and each `INITIAL_INTERVAL` output row
/// either names an accepted point or linearly interpolates one bracket of
/// them. Retaining those samples lets the same projection reproduce the same
/// rows from the compressed result, which the compressor then verifies.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TransientCompressionOutputSchedule<'a> {
    /// Authored `.OPTIONS OUTPUT OUTPUTTIMEPOINTS` stops.
    pub(crate) output_time_points: &'a [Value],
    /// Authored `.OPTIONS OUTPUT INITIAL_INTERVAL` lattice.
    pub(crate) interval_schedule: Option<&'a XyceOutputIntervalSchedule>,
    /// Inclusive output-window start, the authored `.TRAN` TSTART.
    pub(crate) start_time: Value,
    /// Inclusive output-window stop.
    pub(crate) stop_time: Value,
    /// Row budget the authored lattice must respect.
    pub(crate) max_points: usize,
}

impl<'a> TransientCompressionOutputSchedule<'a> {
    /// The schedule authored by one netlist for one accepted transient result.
    pub(crate) fn from_netlist(
        netlist: &'a Netlist,
        result: &TransientResult,
        max_points: usize,
    ) -> Self {
        let start_time = Self::authored_output_start_time(netlist)
            .filter(|start| start.is_finite() && *start >= 0.0)
            .or_else(|| result.time.first().copied())
            .unwrap_or(0.0);
        let stop_time = result.time.last().copied().unwrap_or(start_time);
        Self {
            output_time_points: &netlist.options.output_time_points,
            interval_schedule: netlist.options.output_interval_schedule.as_ref(),
            start_time: start_time.min(stop_time),
            stop_time,
            max_points,
        }
    }

    fn authored_output_start_time(netlist: &Netlist) -> Option<Value> {
        netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Tran { start, .. } => Some(start.unwrap_or(0.0)),
            _ => None,
        })
    }
}

/// One accepted analog channel, sampled on the complete accepted grid.
pub(super) struct AcceptedTransientChannel {
    descriptor: TransientChannelDescriptor,
    availability: TransientChannelAvailability,
    samples: Vec<TransientChannelSample>,
}

impl AcceptedTransientChannel {
    fn sample(&self, index: usize) -> Option<Value> {
        self.samples.get(index).and_then(|sample| sample.value())
    }

    fn retained(&self, indices: &[usize]) -> TransientCompressedChannel {
        TransientCompressedChannel {
            descriptor: self.descriptor.clone(),
            availability: self.availability,
            samples: match self.availability {
                TransientChannelAvailability::Available => {
                    indices.iter().map(|&index| self.samples[index]).collect()
                }
                TransientChannelAvailability::NotProjected => Vec::new(),
            },
        }
    }
}

/// Build the descriptor-keyed accepted channel inventory.
///
/// A channel the authored output projection did not retain keeps its
/// descriptor and declares [`TransientChannelAvailability::NotProjected`]; it
/// never becomes a column of zeros. A sample the producing run computed as
/// non-finite becomes an explicit absence instead of a published number, and
/// the `NaN` padding that [`TransientResult::record_device_op_sample`] writes
/// for a device parameter that did not report becomes an explicit
/// not-recorded absence.
pub(super) fn accepted_transient_channels(
    result: &TransientResult,
) -> Result<Vec<AcceptedTransientChannel>, SimulationError> {
    let point_count = result.time.len();
    let projected = |waveform: &Vec<Value>| {
        if waveform.is_empty() && point_count != 0 {
            TransientChannelAvailability::NotProjected
        } else {
            TransientChannelAvailability::Available
        }
    };
    let mut channels = Vec::with_capacity(
        result.voltages.len()
            + result.branch_currents.len()
            + result.device_op_traces.len()
            + result.store_traces.len(),
    );
    // An event-only net keeps its node-voltage channel here, empty and
    // `NotProjected`, and loses it at the document boundary instead.
    //
    // This inventory is not merely a list of channels: it *is* the compressed
    // result's node namespace. `TransientResultCompressed::node_names` and
    // `num_nodes` are derived from these channels, and every index-addressed
    // accessor resolves through `NodeVoltage { node_index }`, so dropping one
    // channel would renumber the namespace and take the event-only net's
    // name out of it — exactly the MNA alignment the run is required to keep.
    // The document builders read the same emptiness test the uncompressed path
    // does and publish no voltage descriptor for such a node.
    for (node_index, (name, waveform)) in result.node_names.iter().zip(&result.voltages).enumerate()
    {
        channels.push(AcceptedTransientChannel {
            descriptor: TransientChannelDescriptor::for_role(TransientChannelRole::NodeVoltage {
                node_index,
                node: name.clone(),
            })
            .map_err(SimulationError::Circuit)?,
            availability: projected(waveform),
            samples: waveform
                .iter()
                .map(|value| TransientChannelSample::from_solver_value(*value))
                .collect(),
        });
    }
    for (name, waveform) in result.branch_names.iter().zip(&result.branch_currents) {
        channels.push(AcceptedTransientChannel {
            descriptor: TransientChannelDescriptor::for_role(TransientChannelRole::BranchCurrent {
                branch: name.clone(),
            })
            .map_err(SimulationError::Circuit)?,
            availability: projected(waveform),
            samples: waveform
                .iter()
                .map(|value| TransientChannelSample::from_solver_value(*value))
                .collect(),
        });
    }
    for trace in &result.device_op_traces {
        channels.push(AcceptedTransientChannel {
            descriptor: TransientChannelDescriptor::for_role(
                TransientChannelRole::DeviceObservable {
                    device: trace.device_name.clone(),
                    parameter: trace.parameter.clone(),
                },
            )
            .map_err(SimulationError::Circuit)?,
            availability: TransientChannelAvailability::Available,
            samples: trace
                .values
                .iter()
                .map(|value| {
                    if value.is_finite() {
                        TransientChannelSample::Value(*value)
                    } else {
                        TransientChannelSample::Absent(TransientSampleAbsence::NotRecorded)
                    }
                })
                .collect(),
        });
    }
    for trace in &result.store_traces {
        channels.push(AcceptedTransientChannel {
            descriptor: TransientChannelDescriptor::for_role(TransientChannelRole::DeviceStore {
                store: trace.name.clone(),
            })
            .map_err(SimulationError::Circuit)?,
            availability: TransientChannelAvailability::Available,
            samples: trace
                .values
                .iter()
                .map(|value| TransientChannelSample::from_solver_value(*value))
                .collect(),
        });
    }
    Ok(channels)
}

/// Compress one accepted transient result into a complete result container.
///
/// The retained grid is chosen so that every channel's linear reconstruction
/// stays inside `abs_tol + rel_tol * |actual|` at every discarded sample, so
/// that every mandatory schedule point survives exactly, and so that no
/// retained gap exceeds `maximum_retained_interval`.
///
/// # Restart
///
/// `.OPTIONS RESTART` is *not* refused here. An authored restart run is a
/// sequence of independent segments, each of which produces its own accepted
/// result; compressing a segment only decimates that segment's published
/// waveforms and cannot touch the exact solver state a later segment resumes
/// from, exactly as `--checkpoint`/`--resume` already compose. The remaining
/// refusal is a frontend policy about restart *output* file semantics, not a
/// property of this compressor.
pub(super) fn compress_transient_result(
    result: &TransientResult,
    config: &CompressionConfig,
    schedule: &TransientCompressionOutputSchedule<'_>,
    post_results: TransientPostResults,
    identity: TransientResultIdentity,
    abort: &dyn AbortSignal,
) -> Result<TransientResultCompressed, SimulationError> {
    let point_count = result.time.len();
    if !config.abs_tol.is_finite() || config.abs_tol < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression abs_tol must be finite and non-negative, got {}",
            config.abs_tol
        )));
    }
    if !config.rel_tol.is_finite() || config.rel_tol < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression rel_tol must be finite and non-negative, got {}",
            config.rel_tol
        )));
    }
    if !config.maximum_retained_interval.is_finite() || config.maximum_retained_interval < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Compression maximum interval must be finite and non-negative, got {}",
            config.maximum_retained_interval
        )));
    }
    if result.step_sizes.len() != point_count
        || result.voltages.len() != result.num_nodes
        || result.node_names.len() != result.num_nodes
        || result.branch_currents.len() != result.branch_names.len()
        || result
            .voltages
            .iter()
            .any(|waveform| !waveform.is_empty() && waveform.len() != point_count)
        || result
            .branch_currents
            .iter()
            .any(|waveform| !waveform.is_empty() && waveform.len() != point_count)
        || result
            .device_op_traces
            .iter()
            .any(|trace| trace.values.len() != point_count)
        || result
            .store_traces
            .iter()
            .any(|trace| trace.values.len() != point_count)
    {
        return Err(SimulationError::Circuit(
            "Cannot compress a malformed or misaligned transient analog result".to_string(),
        ));
    }
    if result
        .step_sizes
        .iter()
        .any(|step| !step.is_finite() || *step < 0.0)
    {
        return Err(SimulationError::Circuit(
            "Cannot compress a transient with non-finite or negative accepted step sizes"
                .to_string(),
        ));
    }
    if result
        .time
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
        || result.time.last().is_some_and(|time| !time.is_finite())
    {
        return Err(SimulationError::Circuit(
            "Cannot compress a transient with non-finite or non-increasing time points".to_string(),
        ));
    }
    if config.enabled && config.maximum_retained_interval > 0.0 {
        for window in result.time.windows(2) {
            if compression_interval_exceeds(window[0], window[1], config.maximum_retained_interval)
            {
                return Err(SimulationError::Circuit(format!(
                    "Compression maximum retained interval {:.17e}s cannot be guaranteed because the accepted solver grid has a {:.17e}s gap from {:.17e}s to {:.17e}s",
                    config.maximum_retained_interval,
                    window[1] - window[0],
                    window[0],
                    window[1]
                )));
            }
        }
    }

    let channels = accepted_transient_channels(result)?;
    let mandatory_indices = mandatory_schedule_indices(result, schedule)?;

    let finish = |indices: Vec<usize>,
                  worst: Option<TransientCompressionErrorObservation>,
                  post_results: TransientPostResults,
                  identity: TransientResultIdentity|
     -> TransientResultCompressed {
        let stored_points = indices.len();
        TransientResultCompressed {
            instance_unknowns: result.instance_unknowns.clone(),
            potential_units: result.potential_units.clone(),
            event_only_nodes: result.event_only_nodes.clone(),
            voltage_impulses: result.voltage_impulses.clone(),
            current_impulses: result.current_impulses.clone(),
            time: indices.iter().map(|&index| result.time[index]).collect(),
            step_sizes: indices
                .iter()
                .map(|&index| result.step_sizes[index])
                .collect(),
            channels: channels
                .iter()
                .map(|channel| channel.retained(&indices))
                .collect(),
            digital_traces: result.digital_traces.clone(),
            digital_buses: result.digital_buses.clone(),
            real_traces: result.real_traces.clone(),
            hdl_sources: result.hdl_sources.clone(),
            post_results,
            identity,
            compression_ratio: if stored_points == 0 {
                1.0
            } else {
                point_count as Value / stored_points as Value
            },
            input_points: point_count,
            compression_report: TransientCompressionReport::new(
                config,
                point_count,
                stored_points,
                worst,
            ),
        }
    };

    if point_count <= 2 || !config.enabled {
        let compressed = finish((0..point_count).collect(), None, post_results, identity);
        compressed.validate().map_err(SimulationError::Circuit)?;
        return Ok(compressed);
    }

    let mut retained = vec![false; point_count];
    retained[0] = true;
    retained[point_count - 1] = true;
    for &index in &mandatory_indices {
        retained[index] = true;
    }
    let mut anchors = std::iter::once(0)
        .chain(mandatory_indices.iter().copied())
        .chain(std::iter::once(point_count - 1))
        .collect::<Vec<_>>();
    anchors.sort_unstable();
    anchors.dedup();
    let mut segments = anchors
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect::<Vec<_>>();
    while let Some((start, end)) = segments.pop() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if end <= start + 1 {
            continue;
        }

        let duration = result.time[end] - result.time[start];
        let interval_split = if config.maximum_retained_interval > 0.0
            && duration > config.maximum_retained_interval
        {
            let target = result.time[start] + config.maximum_retained_interval;
            Some(
                ((start + 1)..end)
                    .take_while(|&index| result.time[index] <= target)
                    .last()
                    .unwrap_or(start + 1),
            )
        } else {
            None
        };

        let mut worst_index = interval_split;
        let mut worst_ratio = if interval_split.is_some() {
            Value::INFINITY
        } else {
            1.0
        };
        if interval_split.is_none() {
            let t0 = result.time[start];
            let duration = result.time[end] - t0;
            for point in (start + 1)..end {
                if point.is_multiple_of(4096) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Match TransientResultCompressed::interpolate_channel exactly.
                // Multiplying by a rounded reciprocal can change a small
                // reconstruction error enough to invalidate its certificate.
                let fraction = (result.time[point] - t0) / duration;
                for channel in &channels {
                    // An absent endpoint or sample has no reconstruction to
                    // measure. Skipping it here is the same deterministic rule
                    // the certificate below applies.
                    let (Some(actual), Some(left), Some(right)) = (
                        channel.sample(point),
                        channel.sample(start),
                        channel.sample(end),
                    ) else {
                        continue;
                    };
                    let predicted = left + fraction * (right - left);
                    let tolerance = config.abs_tol + config.rel_tol * actual.abs();
                    if !tolerance.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Compression tolerance overflowed at input sample {point}"
                        )));
                    }
                    let error = (actual - predicted).abs();
                    let ratio = if !error.is_finite() {
                        Value::INFINITY
                    } else if tolerance > 0.0 {
                        error / tolerance
                    } else if error == 0.0 {
                        0.0
                    } else {
                        Value::INFINITY
                    };
                    if ratio > worst_ratio {
                        worst_ratio = ratio;
                        worst_index = Some(point);
                    }
                }
            }
        }

        if let Some(split) = worst_index
            && worst_ratio > 1.0
        {
            retained[split] = true;
            segments.push((start, split));
            segments.push((split, end));
        }
    }

    let indices = retained
        .iter()
        .enumerate()
        .filter_map(|(index, keep)| keep.then_some(index))
        .collect::<Vec<_>>();
    if config.maximum_retained_interval > 0.0 {
        for retained_window in indices.windows(2) {
            let start = result.time[retained_window[0]];
            let stop = result.time[retained_window[1]];
            if compression_interval_exceeds(start, stop, config.maximum_retained_interval) {
                return Err(SimulationError::Circuit(format!(
                    "Compression failed to enforce the maximum retained interval {:.17e}s between {:.17e}s and {:.17e}s",
                    config.maximum_retained_interval, start, stop
                )));
            }
        }
    }
    let worst_observed =
        verify_compressed_transient_error(result, &channels, config, &indices, abort)?;
    verify_retained_grid_reproduces_schedule(result, schedule, &indices)?;
    let compressed = finish(indices, worst_observed, post_results, identity);
    compressed.validate().map_err(SimulationError::Circuit)?;
    Ok(compressed)
}

/// Accepted-grid indices the authored output schedule reads.
///
/// `OUTPUTTIMEPOINTS` stops must be exact accepted solver points; a requested
/// stop the solver never landed on is an authored-input error rather than
/// something to approximate. An `INITIAL_INTERVAL` lattice row is either an
/// accepted point or a linear interpolation between two neighbouring accepted
/// points, so both bracket samples are mandatory.
fn mandatory_schedule_indices(
    result: &TransientResult,
    schedule: &TransientCompressionOutputSchedule<'_>,
) -> Result<Vec<usize>, SimulationError> {
    let result_window = result
        .time
        .first()
        .copied()
        .zip(result.time.last().copied());
    let mut indices = Vec::new();
    for (schedule_index, &time) in schedule.output_time_points.iter().enumerate() {
        if !time.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Cannot compress non-finite mandatory output time at index {schedule_index}"
            )));
        }
        if !result_window.is_some_and(|(start, stop)| time >= start && time <= stop) {
            continue;
        }
        let index = result
            .time
            .binary_search_by(|candidate| candidate.total_cmp(&time))
            .map_err(|_| {
                SimulationError::Circuit(format!(
                    "Cannot compress transient because mandatory output time {time:.17e}s is not an exact accepted solver point"
                ))
            })?;
        indices.push(index);
    }

    if let Some(interval_schedule) = schedule.interval_schedule {
        for event in interval_output_events(schedule, &result.time, interval_schedule)? {
            let (accepted_index, interpolated) = event;
            if accepted_index >= result.time.len() {
                return Err(SimulationError::Circuit(format!(
                    "Cannot compress transient because an INITIAL_INTERVAL output row names accepted sample {accepted_index}, which does not exist"
                )));
            }
            indices.push(accepted_index);
            if interpolated {
                let previous = accepted_index.checked_sub(1).ok_or_else(|| {
                    SimulationError::Circuit(
                        "Cannot compress transient because an interpolated INITIAL_INTERVAL output row has no accepted bracket"
                            .to_string(),
                    )
                })?;
                indices.push(previous);
            }
        }
    }

    indices.sort_unstable();
    indices.dedup();
    Ok(indices)
}

/// Replay the authored interval-output scheduler over one time grid.
///
/// Each returned entry is `(accepted_index, interpolated)`.
fn interval_output_events(
    schedule: &TransientCompressionOutputSchedule<'_>,
    times: &[Value],
    interval_schedule: &XyceOutputIntervalSchedule,
) -> Result<Vec<(usize, bool)>, SimulationError> {
    if !schedule.output_time_points.is_empty() {
        return Err(SimulationError::Circuit(
            "Cannot compress transient output that combines OUTPUTTIMEPOINTS and INITIAL_INTERVAL"
                .to_string(),
        ));
    }
    let stop_time = times
        .last()
        .copied()
        .filter(|stop| *stop <= schedule.stop_time)
        .unwrap_or(schedule.stop_time);
    let start_time = schedule.start_time.min(stop_time);
    interval_schedule
        .output_events(times, start_time, stop_time, schedule.max_points)
        .map(|events| {
            events
                .into_iter()
                .map(|event| (event.accepted_index, event.interpolation_time.is_some()))
                .collect()
        })
        .map_err(SimulationError::from)
}

/// Prove that the retained grid reproduces the authored output rows exactly.
///
/// The interval scheduler is accepted-step dependent, so a retained subset is
/// only a faithful substitute when replaying the scheduler over it produces
/// the same rows, reading the same accepted samples. This is checked rather
/// than assumed, and a mismatch fails closed instead of publishing a result
/// whose authored output grid silently moved.
fn verify_retained_grid_reproduces_schedule(
    result: &TransientResult,
    schedule: &TransientCompressionOutputSchedule<'_>,
    indices: &[usize],
) -> Result<(), SimulationError> {
    let Some(interval_schedule) = schedule.interval_schedule else {
        return Ok(());
    };
    if indices.len() == result.time.len() {
        return Ok(());
    }
    let retained_times = indices
        .iter()
        .map(|&index| result.time[index])
        .collect::<Vec<_>>();
    let accepted = interval_output_events(schedule, &result.time, interval_schedule)?;
    let retained = interval_output_events(schedule, &retained_times, interval_schedule)?;
    let same_rows = accepted.len() == retained.len()
        && accepted.iter().zip(&retained).all(
            |((accepted_index, accepted_interpolated), (retained_index, interpolated))| {
                accepted_interpolated == interpolated
                    && indices.get(*retained_index) == Some(accepted_index)
            },
        );
    if same_rows {
        Ok(())
    } else {
        Err(SimulationError::Circuit(format!(
            "Compression retained {} of {} accepted points but the authored INITIAL_INTERVAL output lattice no longer reproduces its {} rows from them",
            indices.len(),
            result.time.len(),
            accepted.len()
        )))
    }
}

/// Independently certify the reconstruction error of the final published
/// retained grid. Candidate errors from an earlier coarse RDP segment cannot
/// be reported because later splits repair that segment.
///
/// A discarded sample is only certified where the channel has a value at the
/// sample and at both retained endpoints. An absent sample has no
/// reconstruction and no error; it is skipped deterministically rather than
/// being coerced into a number.
pub(super) fn verify_compressed_transient_error(
    result: &TransientResult,
    channels: &[AcceptedTransientChannel],
    config: &CompressionConfig,
    retained_indices: &[usize],
    abort: &dyn AbortSignal,
) -> Result<Option<TransientCompressionErrorObservation>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let comparable = channels
        .iter()
        .filter(|channel| channel.availability == TransientChannelAvailability::Available)
        .collect::<Vec<_>>();
    if comparable.is_empty() || retained_indices.len() >= result.time.len() {
        return Ok(None);
    }

    let mut worst: Option<TransientCompressionErrorObservation> = None;
    let mut scanned_points = 0usize;
    for retained_window in retained_indices.windows(2) {
        let start = retained_window[0];
        let end = retained_window[1];
        if end <= start + 1 {
            continue;
        }
        let t0 = result.time[start];
        let duration = result.time[end] - t0;
        for point in (start + 1)..end {
            scanned_points = scanned_points.saturating_add(1);
            if scanned_points.is_multiple_of(4096) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Match TransientResultCompressed::interpolate_channel exactly.
            // Multiplying by a rounded reciprocal can change a small
            // reconstruction error enough to invalidate its certificate.
            let fraction = (result.time[point] - t0) / duration;
            for channel in &comparable {
                let (Some(actual), Some(left), Some(right)) = (
                    channel.sample(point),
                    channel.sample(start),
                    channel.sample(end),
                ) else {
                    continue;
                };
                let signal = channel.descriptor.signal();
                let predicted = left + fraction * (right - left);
                let absolute_error = (actual - predicted).abs();
                let allowed_tolerance = config.abs_tol + config.rel_tol * actual.abs();
                if !absolute_error.is_finite() || !allowed_tolerance.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "Compression error certificate became non-finite for '{}:{}' at input sample {point}",
                        signal.kind.as_str(),
                        signal.canonical_name
                    )));
                }
                let tolerance_utilization = if allowed_tolerance > 0.0 {
                    absolute_error / allowed_tolerance
                } else if absolute_error == 0.0 {
                    0.0
                } else {
                    Value::INFINITY
                };
                if !tolerance_utilization.is_finite()
                    || tolerance_utilization > 1.0 + 64.0 * Value::EPSILON
                {
                    return Err(SimulationError::Circuit(format!(
                        "Compression final-grid verification failed for '{}:{}' at t={:.17e}s: error {:.17e}, tolerance {:.17e}",
                        signal.kind.as_str(),
                        signal.canonical_name,
                        result.time[point],
                        absolute_error,
                        allowed_tolerance
                    )));
                }
                let relative_error = if actual == 0.0 {
                    None
                } else {
                    let relative = absolute_error / actual.abs();
                    relative.is_finite().then_some(relative)
                };
                let observation = TransientCompressionErrorObservation {
                    signal,
                    input_sample_index: point,
                    time: result.time[point],
                    actual_value: actual,
                    absolute_error,
                    relative_error,
                    allowed_tolerance,
                    tolerance_utilization,
                };
                if worst.as_ref().is_none_or(|current| {
                    observation.tolerance_utilization > current.tolerance_utilization
                }) {
                    worst = Some(observation);
                }
            }
        }
    }
    Ok(worst)
}

fn compression_interval_exceeds(start: Value, stop: Value, maximum_interval: Value) -> bool {
    let gap = stop - start;
    let scale = start
        .abs()
        .max(stop.abs())
        .max(maximum_interval.abs())
        .max(Value::MIN_POSITIVE);
    gap > maximum_interval + 64.0 * Value::EPSILON * scale
}
