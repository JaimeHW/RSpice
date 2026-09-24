//! Slow-time Fourier execution and physical envelope result projection.
use super::*;
use rspice_core::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicSampling, QuasiPeriodicSolveConfig,
    SpectralEnvelopeControl, SpectralEnvelopeMethod,
};
use rspice_core::engine::{
    EnvelopeCarrierBasis, EnvelopeEventTolerances, NetlistEnvelopeMission, QpssSourceTone,
    SpectralEnvelopeConfig, SpectralEnvelopeMissionConfig, SpectralEnvelopeStepping,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeMultirateConfig {
    pub method: SpectralEnvelopeMethod,
    pub adaptive: bool,
    pub minimum_step: f64,
    pub maximum_step: Option<f64>,
    pub relative_tolerance: f64,
    pub voltage_absolute_tolerance: f64,
    pub current_absolute_tolerance: f64,
    pub auxiliary_absolute_tolerance: f64,
    pub maximum_rejections: usize,
    pub maximum_steps: usize,
    /// Empty uses the main harmonic order for each carrier.
    pub harmonics: Vec<usize>,
    /// A single count broadcasts to each carrier. Exact counts override it.
    pub oversample: Vec<usize>,
    pub collocation_points: Vec<usize>,
    pub max_mixing_order: Option<usize>,
    pub source_tones: Vec<QpssSourceTone>,
    pub solver: QuasiPeriodicSolveConfig,
    pub dc_initialization: bool,
    pub source_time_step: Option<f64>,
    pub events: EnvelopeEventTolerances,
}

impl Default for EnvelopeMultirateConfig {
    fn default() -> Self {
        Self {
            method: SpectralEnvelopeMethod::Bdf2,
            adaptive: true,
            minimum_step: 1e-12,
            maximum_step: None,
            relative_tolerance: 1e-3,
            voltage_absolute_tolerance: 1e-6,
            current_absolute_tolerance: 1e-9,
            auxiliary_absolute_tolerance: 1e-9,
            maximum_rejections: 24,
            maximum_steps: 100_000,
            harmonics: Vec::new(),
            oversample: vec![2],
            collocation_points: Vec::new(),
            max_mixing_order: None,
            source_tones: Vec::new(),
            solver: Default::default(),
            dc_initialization: false,
            source_time_step: None,
            events: Default::default(),
        }
    }
}

impl EnvelopeMultirateConfig {
    pub(crate) fn core_config(
        &self,
        carriers: &[f64],
        order: usize,
        stop: f64,
        step: f64,
        sources: &[String],
    ) -> Result<SpectralEnvelopeConfig, String> {
        let positive = |v: f64| v.is_finite() && v > 0.0;
        if !positive(step)
            || !positive(self.maximum_step.unwrap_or(step))
            || (self.adaptive
                && (!positive(self.minimum_step)
                    || self.maximum_step.unwrap_or(step) < self.minimum_step
                    || !self.relative_tolerance.is_finite()
                    || !(0.0..1.0).contains(&self.relative_tolerance)
                    || ![
                        self.voltage_absolute_tolerance,
                        self.current_absolute_tolerance,
                        self.auxiliary_absolute_tolerance,
                    ]
                    .into_iter()
                    .all(positive)))
            || self.maximum_steps == 0
        {
            return Err("Multirate Envelope needs positive step bounds and absolute tolerances, relative tolerance in [0,1), and a positive step limit".into());
        }
        self.events.validate().map_err(|e| e.to_string())?;
        if carriers.is_empty() {
            return Err("Envelope requires a carrier frequency".into());
        }
        if carriers.len() == 1 && self.max_mixing_order.is_some() {
            return Err("Envelope mixing-order limits require multiple carriers".into());
        }
        let expand = |values: &[usize], label: &str| -> Result<Vec<usize>, String> {
            let values = if values.len() == 1 {
                vec![values[0]; carriers.len()]
            } else {
                values.to_vec()
            };
            if values.len() != carriers.len() || values.contains(&0) {
                return Err(format!(
                    "Envelope {label} needs one positive count, or a count for each carrier"
                ));
            }
            Ok(values)
        };
        let harmonics = if self.harmonics.is_empty() {
            vec![order; carriers.len()]
        } else {
            expand(&self.harmonics, "harmonic orders")?
        };
        let sampling = if self.collocation_points.is_empty() {
            QuasiPeriodicSampling::Oversample(expand(&self.oversample, "oversampling")?)
        } else {
            QuasiPeriodicSampling::Exact(expand(&self.collocation_points, "collocation points")?)
        };
        let carrier = if carriers.len() == 1 {
            let samples = match &sampling {
                QuasiPeriodicSampling::Exact(points) => points[0],
                QuasiPeriodicSampling::Oversample(factors) => harmonics[0]
                    .checked_mul(2)
                    .and_then(|v| v.checked_add(1))
                    .and_then(|v| v.checked_mul(factors[0]))
                    .and_then(usize::checked_next_power_of_two)
                    .ok_or("Envelope carrier grid overflowed")?,
            };
            EnvelopeCarrierBasis::Periodic {
                frequency_hz: carriers[0],
                harmonics: harmonics[0],
                samples,
            }
        } else {
            EnvelopeCarrierBasis::QuasiPeriodic {
                grid: QuasiPeriodicGridConfig {
                    frequencies_hz: carriers.to_vec(),
                    harmonics,
                    sampling,
                    max_mixing_order: self.max_mixing_order,
                },
            }
        };
        let mut names = std::collections::HashSet::new();
        for source in sources {
            if source.trim().is_empty()
                || source.trim() != source
                || !names.insert(source.to_ascii_lowercase())
            {
                return Err("Envelope modulation sources must be exact and unique".into());
            }
        }
        let mut bindings = std::collections::HashSet::new();
        for binding in &self.source_tones {
            if binding.source.trim().is_empty()
                || binding.source.trim() != binding.source
                || binding.tone >= carriers.len()
                || names.contains(&binding.source.to_ascii_lowercase())
                || !bindings.insert((binding.source.to_ascii_lowercase(), binding.tone))
            {
                return Err("Envelope carrier bindings must name unique source/tone pairs, within the tone list, separate from modulation sources".into());
            }
        }
        let config = SpectralEnvelopeConfig {
            carrier,
            solver: self.solver.clone(),
            source_tones: self.source_tones.clone(),
            modulation_sources: sources.to_vec(),
            stop_time: stop,
            source_time_step: self.source_time_step.unwrap_or(step),
        };
        config.validate().map_err(|e| e.to_string())?;
        Ok(config)
    }
}

pub(super) fn run(
    netlist: &rspice_core::Netlist,
    config: &EnvelopeRunConfig,
    settings: &EnvelopeMultirateConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    ensure_not_aborted(abort)?;
    let step = config.envelope_step.unwrap_or(config.stop_time / 1200.0);
    let core = settings
        .core_config(
            &config.carrier_tones().collect::<Vec<_>>(),
            config.num_harmonics,
            config.stop_time,
            step,
            &config.modulation_sources,
        )
        .map_err(ServiceRunError::Failure)?;
    let engine = super::super::build_resolved_periodic_engine(
        netlist,
        settings.solver.relative_tolerance,
        "Envelope resolved engine configuration",
    )?;
    let mut prepared = engine
        .prepare_spectral_envelope_with_abort(netlist, core, abort)
        .map_err(|e| ServiceRunError::from_core("Multirate Envelope preparation", e))?;
    let events = prepared
        .event_config_with_abort(&settings.events, abort)
        .map_err(|e| ServiceRunError::from_core("Envelope event configuration", e))?;
    let mut absolute = vec![settings.voltage_absolute_tolerance; prepared.node_names().len()];
    absolute.extend(std::iter::repeat_n(
        settings.current_absolute_tolerance,
        prepared.branch_names().len(),
    ));
    absolute.extend(std::iter::repeat_n(
        settings.auxiliary_absolute_tolerance,
        prepared.auxiliary_names().len(),
    ));
    let stepping = if settings.adaptive {
        SpectralEnvelopeStepping::Adaptive {
            initial_step: step,
            control: SpectralEnvelopeControl {
                method: settings.method,
                minimum_step: settings.minimum_step,
                maximum_step: settings.maximum_step.unwrap_or(step),
                relative_tolerance: settings.relative_tolerance,
                absolute_tolerances: absolute,
                max_rejections: settings.maximum_rejections,
            },
        }
    } else {
        SpectralEnvelopeStepping::Fixed {
            method: settings.method,
            step: step.min(settings.maximum_step.unwrap_or(step)),
        }
    };
    if !netlist.options.output_time_points.is_empty()
        && netlist.options.output_interval_schedule.is_some()
    {
        return Err(ServiceRunError::Failure(
            "Envelope cannot combine a strobe interval and explicit reporting times".into(),
        ));
    }
    let interval_schedule = netlist.options.output_interval_schedule.is_some();
    let explicit = !netlist.options.output_time_points.is_empty();
    let mut times = if explicit {
        for time in &netlist.options.output_time_points {
            if !time.is_finite() || *time < 0.0 {
                return Err(ServiceRunError::Failure(
                    "Envelope reporting times must be finite and nonnegative".into(),
                ));
            }
        }
        netlist
            .options
            .output_time_points
            .iter()
            .copied()
            .filter(|t| *t <= config.stop_time)
            .collect()
    } else if !interval_schedule && config.adaptive_mode == EnvelopeAdaptiveMode::FixedEnvelopeStep
    {
        uniform_envelope_times(config.stop_time, step, abort)?
    } else {
        Vec::new()
    };
    times.sort_by(Value::total_cmp);
    times.dedup();
    let request = SpectralEnvelopeMissionConfig {
        stepping,
        reporting_times: times,
        retain_accepted_steps: interval_schedule
            || (!explicit && config.adaptive_mode == EnvelopeAdaptiveMode::Enabled),
        maximum_steps: settings.maximum_steps,
        events,
    };
    let seed = if settings.dc_initialization {
        let dc = engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|e| ServiceRunError::from_core("Envelope DC initial guess", e))?;
        Some(
            rspice_core::engine::PeriodicDcOperatingPointSeed::try_new(
                dc.node_names.into_iter().skip(1).collect(),
                dc.branch_names,
                dc.node_voltages
                    .into_iter()
                    .skip(1)
                    .chain(dc.branch_currents)
                    .collect(),
            )
            .map_err(|e| ServiceRunError::from_core("Envelope DC initial guess", e))?,
        )
    } else {
        None
    };
    let mission = prepared
        .run_mission_with_abort(&request, seed.as_ref(), abort)
        .map_err(|e| ServiceRunError::from_core("Multirate Envelope", e))?;
    drop(prepared);
    project(mission, netlist, settings.adaptive, abort)
}

fn scalar(name: &str, value: Value, unit: &str, time: Option<Value>) -> rspice_core::MeasureResult {
    let mut result = rspice_core::MeasureResult::success(name, value);
    let unit = rspice_core::analysis::MeasurementUnit::Known(unit.into());
    result.units = Some(rspice_core::analysis::MeasurementUnits {
        value: unit.clone(),
        raw_value: unit,
        axis: rspice_core::analysis::MeasurementUnit::Known("s".into()),
    });
    result.event_axis = time;
    result
}

fn project(
    mission: NetlistEnvelopeMission,
    netlist: &rspice_core::Netlist,
    adaptive: bool,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    let state = &mission.final_state;
    let grid = state.grid();
    let bins: Vec<_> = grid
        .frequencies_hz()
        .iter()
        .enumerate()
        .filter_map(|(i, f)| (*f >= 0.0).then_some(i))
        .collect();
    let mut time = mission
        .samples
        .iter()
        .map(|sample| sample.time)
        .collect::<Vec<_>>();
    if let Some(schedule) = &netlist.options.output_interval_schedule {
        let mut unique = time.clone();
        unique.dedup();
        let projection =
            rspice_core::analysis::transient::TransientOutputProjection::from_accepted_times(
                &unique,
                &[],
                Some(schedule),
                unique[0],
                *unique.last().unwrap(),
                rspice_core::ResourceLimits::default().max_analysis_points,
            )
            .map_err(ServiceRunError::Failure)?;
        time = projection.times().to_vec();
        for event in &mission.transitions {
            time.push(event.time);
        }
        time.sort_by(Value::total_cmp);
        // Exactly two sides per event, one sample at every other report.
        time.dedup();
        let count = time.len().saturating_add(mission.transitions.len());
        let limit = rspice_core::ResourceLimits::default().max_analysis_points;
        if count > limit {
            return Err(ServiceRunError::resource_limit(
                rspice_core::ResourceKind::AnalysisPoints,
                count,
                limit,
            ));
        }
        let mut sided = Vec::with_capacity(count);
        let mut event = mission.transitions.iter().peekable();
        for (index, value) in time.into_iter().enumerate() {
            poll_periodically(abort, index)?;
            sided.push(value);
            if event.peek().is_some_and(|event| event.time == value) {
                sided.push(value);
                event.next();
            }
        }
        time = sided;
    }
    let mut coordinates = Vec::new();
    for (row, name) in state.node_names().iter().enumerate() {
        if netlist.saves.retains_voltage_operand(name) {
            coordinates.push((row, format!("V({name})"), "V"));
        }
    }
    for (index, name) in state.branch_names().iter().enumerate() {
        let name = format!("I({name})");
        if netlist.saves.selects(&name) {
            coordinates.push((state.node_names().len() + index, name, "A"));
        }
    }
    for selected in &netlist.saves.signals {
        use rspice_core::netlist::SaveSignal;
        if matches!(
            selected,
            SaveSignal::Current(_) | SaveSignal::DeviceParam { .. }
        ) {
            let selection = rspice_core::netlist::SaveSet {
                signals: vec![selected.clone()],
            };
            if !state
                .branch_names()
                .iter()
                .any(|name| selection.selects(&format!("I({name})")))
            {
                return Err(ServiceRunError::Failure(format!(
                    "Multirate Envelope cannot retain {selected:?}; select an MNA branch current or use full-waveform execution"
                )));
            }
        }
    }
    if coordinates.is_empty() {
        return Err(ServiceRunError::Failure(
            "Multirate Envelope found no selected node or MNA branch traces".into(),
        ));
    }
    let retained =
        envelope_retained_value_count(time.len(), coordinates.len().saturating_mul(bins.len()))?;
    let mut source_values = mission
        .samples
        .len()
        .saturating_mul(state.spectra().len())
        .saturating_mul(grid.len())
        .saturating_mul(2);
    let mut scalar_values = 12usize;
    for event in &mission.transitions {
        ensure_not_aborted(abort)?;
        for row in event.current_impulses.iter().chain(&event.slow_rates) {
            source_values = source_values.saturating_add(row.len().saturating_mul(2));
        }
        for row in &event.current_impulses {
            if !row.is_empty() {
                scalar_values = scalar_values.saturating_add(bins.len().saturating_mul(6));
            }
        }
    }
    source_values = source_values.saturating_add(scalar_values);
    let limit = rspice_core::ResourceLimits::default().max_result_values;
    if retained.saturating_add(source_values) > limit {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            retained.saturating_add(source_values),
            limit,
        ));
    }
    let mut waveforms = Vec::new();
    for (row, name, unit) in coordinates {
        for &bin in &bins {
            ensure_not_aborted(abort)?;
            let factor = if bin == grid.dc_index() { 1.0 } else { 2.0 };
            let mut values = Vec::with_capacity(time.len());
            for (index, &t) in time.iter().enumerate() {
                poll_periodically(abort, index)?;
                let lower = mission.samples.partition_point(|s| s.time < t);
                let upper = mission.samples.partition_point(|s| s.time <= t);
                let value = if lower != upper {
                    // The first repeated timestamp is incoming, the second outgoing.
                    let sample = if time.get(index + 1) == Some(&t) {
                        lower
                    } else {
                        upper - 1
                    };
                    mission.samples[sample].spectra[row][bin]
                } else {
                    let a = &mission.samples[lower - 1];
                    let b = &mission.samples[lower];
                    let weight = (t - a.time) / (b.time - a.time);
                    a.spectra[row][bin] * (1.0 - weight) + b.spectra[row][bin] * weight
                } * factor;
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(ServiceRunError::Failure(
                        "Envelope peak-amplitude projection overflowed".into(),
                    ));
                }
                values.push(Complex64::new(value.re, value.im));
            }
            let tuple = grid.indices()[bin]
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            waveforms.push(EnvelopeWaveform {
                is_complex: bin != grid.dc_index(),
                name: format!(
                    "ENV({name}; k=[{tuple}]; f={:.12e}Hz)",
                    grid.frequencies_hz()[bin]
                ),
                unit,
                values,
            });
        }
    }
    let mut measurements = vec![
        scalar(
            "Envelope accepted steps",
            mission.accepted_steps as Value,
            "1",
            None,
        ),
        scalar(
            "Envelope rejected steps",
            mission.rejected_steps as Value,
            "1",
            None,
        ),
        scalar(
            "Envelope spectral solves",
            mission.spectral_solves as Value,
            "1",
            None,
        ),
    ];
    if adaptive {
        measurements.push(scalar(
            "Envelope maximum error ratio",
            mission.maximum_error_ratio,
            "1",
            None,
        ));
    }
    for event in &mission.transitions {
        for (index, name) in state.branch_names().iter().enumerate() {
            ensure_not_aborted(abort)?;
            let impulse = &event.current_impulses[state.node_names().len() + index];
            if impulse.is_empty() {
                continue;
            }
            for &bin in &bins {
                let value = impulse[bin] * if bin == grid.dc_index() { 1.0 } else { 2.0 };
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(ServiceRunError::Failure(
                        "Envelope impulse display scaling overflowed".into(),
                    ));
                }
                let label = format!(
                    "Envelope impulse I({name}) k={:?} t={:.17e}s",
                    grid.indices()[bin],
                    event.time
                );
                measurements.push(scalar(
                    &format!("{label} real"),
                    value.re,
                    "C",
                    Some(event.time),
                ));
                measurements.push(scalar(
                    &format!("{label} imaginary"),
                    value.im,
                    "C",
                    Some(event.time),
                ));
            }
        }
    }
    ensure_not_aborted(abort)?;
    Ok(EnvelopeData {
        time,
        waveforms,
        measurements,
        convergence: None,
    })
}
