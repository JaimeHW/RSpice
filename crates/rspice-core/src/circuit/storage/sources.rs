use super::*;

#[derive(Debug, Default, Clone)]
pub struct VoltageSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for AC/HB analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for AC/HB analysis
    pub ac_phases: Vec<Value>,
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Pre-baked CSC indices: [br->np, np->br, br->nn, nn->br] per source
    csc_indices: Vec<[Option<CscIndex>; 4]>,
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
}

#[derive(Debug, Clone, Copy)]
struct TransientSourceContext {
    tstep: Value,
    tstop: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PwlCacheKey {
    path: String,
    time_scale_bits: u64,
    value_scale_bits: u64,
    time_offset_bits: u64,
    value_offset_bits: u64,
}

impl PwlCacheKey {
    fn new(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Self {
        Self {
            path: path.to_string(),
            time_scale_bits: time_scale.to_bits(),
            value_scale_bits: value_scale.to_bits(),
            time_offset_bits: time_offset.to_bits(),
            value_offset_bits: value_offset.to_bits(),
        }
    }
}

fn pwl_waveform_cache()
-> &'static RwLock<HashMap<PwlCacheKey, Arc<crate::device::pwl_file::PwlWaveform>>> {
    static CACHE: OnceLock<
        RwLock<HashMap<PwlCacheKey, Arc<crate::device::pwl_file::PwlWaveform>>>,
    > = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn pwl_error_log_cache() -> &'static RwLock<HashSet<PwlCacheKey>> {
    static CACHE: OnceLock<RwLock<HashSet<PwlCacheKey>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashSet::new()))
}

impl VoltageSources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
        self.source_specs.push(None);
        self.csc_indices.push([None; 4]);
    }

    /// Add voltage source with full AC and transient specification
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
        self.csc_indices.push([None; 4]);
    }

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Link indices to StaticMatrix for O(1) stamping
    pub fn link_indices(&mut self, matrix: &StaticMatrix, get_branch_idx: impl Fn(usize) -> usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = get_branch_idx(self.branch_indices[i]);

            // br->np and np->br
            if np > 0 {
                self.csc_indices[i][0] = matrix.get_index(br - 1, np - 1);
                self.csc_indices[i][1] = matrix.get_index(np - 1, br - 1);
            }
            // br->nn and nn->br
            if nn > 0 {
                self.csc_indices[i][2] = matrix.get_index(br - 1, nn - 1);
                self.csc_indices[i][3] = matrix.get_index(nn - 1, br - 1);
            }
        }
    }

    /// Stamp all voltage sources using pre-baked CSC indices
    #[inline]
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i];

            // Stamp matrix entries using pre-baked indices
            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp voltage sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i] * scale;

            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp all voltage sources
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];
            let v = self.dc_values[i];

            // MNA stamp: add branch equation V(n+) - V(n-) = Vs
            if br > 0 && np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if br > 0 && nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            if br > 0 {
                rhs[br - 1] = v;
            }
        }
    }

    /// Update voltage source RHS values for transient analysis at time t
    ///
    /// Evaluates time-varying sources (PULSE, SIN, PWL, EXP) at the given time
    /// and updates the RHS vector. Matrix structure is unchanged.
    #[inline]
    pub fn update_transient_rhs(
        &self,
        rhs: &mut [Value],
        time: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        let context = self.transient_context;
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);

            let v = match &self.source_specs[i] {
                Some(spec) => Self::evaluate_source_at_time_with_context(spec, time, context),
                None => self.dc_values[i], // DC only
            };

            rhs[br - 1] = v;
        }
    }

    /// Maximum absolute change expected from time-varying sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .filter_map(|spec| spec.as_ref())
            .map(|spec| {
                (Self::evaluate_source_at_time_with_context(spec, t1, context)
                    - Self::evaluate_source_at_time_with_context(spec, t0, context))
                .abs()
            })
            .fold(0.0, Value::max)
    }

    #[inline]
    pub fn max_dc_to_transient_delta(&self, time: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .enumerate()
            .filter_map(|(idx, spec)| spec.as_ref().map(|spec| (idx, spec)))
            .map(|(idx, spec)| {
                (Self::evaluate_source_at_time_with_context(spec, time, context)
                    - self.dc_values[idx])
                    .abs()
            })
            .fold(0.0, Value::max)
    }

    pub(crate) fn load_pwl_waveform_cached(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Result<Arc<crate::device::pwl_file::PwlWaveform>, String> {
        let key = PwlCacheKey::new(path, time_scale, value_scale, time_offset, value_offset);

        if let Ok(cache) = pwl_waveform_cache().read()
            && let Some(wf) = cache.get(&key)
        {
            return Ok(Arc::clone(wf));
        }

        let waveform = crate::device::pwl_file::load_pwl_file(path)
            .map_err(|e| format!("failed to load PWL file '{}': {}", path, e))?
            .with_scaling(time_scale, value_scale, time_offset, value_offset);
        let waveform = Arc::new(waveform);

        if let Ok(mut cache) = pwl_waveform_cache().write() {
            let entry = cache.entry(key).or_insert_with(|| Arc::clone(&waveform));
            return Ok(Arc::clone(entry));
        }

        Ok(waveform)
    }

    fn log_pwl_error_once(key: PwlCacheKey, msg: &str) {
        if let Ok(mut logged) = pwl_error_log_cache().write() {
            if logged.insert(key) {
                log::warn!("{}", msg);
            }
            return;
        }
        log::warn!("{}", msg);
    }

    #[inline]
    fn pulse_step_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstep).unwrap_or(1e-12).max(1e-18)
    }

    #[inline]
    fn pulse_stop_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstop).unwrap_or(1e99).max(1e-18)
    }

    #[inline]
    fn sin_frequency_default(context: Option<TransientSourceContext>) -> Value {
        context
            .map(|ctx| ctx.tstop)
            .filter(|tstop| tstop.is_finite() && *tstop > 0.0)
            .map(|tstop| 1.0 / tstop)
            .unwrap_or(1e3)
    }

    /// ngspice's analysis-scaled frequency defaults for SFFM/AM: an omitted
    /// frequency becomes `cycles / tstop` (vsrcload.c uses 5 and 500).
    #[inline]
    fn modulated_frequency_default(
        cycles: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        context
            .map(|ctx| ctx.tstop)
            .filter(|tstop| tstop.is_finite() && *tstop > 0.0)
            .map(|tstop| cycles / tstop)
            .unwrap_or(cycles * 1e3)
    }

    #[inline]
    fn resolve_pulse_timing(
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        width_defaults_to_zero: bool,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value, Value, Value) {
        Self::resolve_pulse_timing_with_defaults(
            delay,
            rise,
            fall,
            width,
            period,
            width_defaults_to_zero,
            Self::pulse_step_default(context),
            Self::pulse_stop_default(context),
        )
    }

    /// Resolve PULSE timing fields against explicit tstep/tstop defaults.
    /// Shared with breakpoint scheduling so accepted timesteps land on the
    /// same edges the waveform actually produces.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn resolve_pulse_timing_with_defaults(
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        width_defaults_to_zero: bool,
        step_default: Value,
        stop_default: Value,
    ) -> (Value, Value, Value, Value, Value) {
        let period_was_omitted = period.is_nan();

        let td = if delay.is_finite() {
            delay.max(0.0)
        } else {
            0.0
        };
        let tr = if rise.is_nan() { step_default } else { rise };
        let tf = if fall.is_nan() { step_default } else { fall };
        let pw = if width.is_nan() && width_defaults_to_zero {
            0.0
        } else if width.is_nan() {
            stop_default
        } else {
            width
        };
        let per = if period.is_nan() {
            stop_default
        } else {
            period
        };

        let tr = if tr.is_finite() && tr > 0.0 {
            tr
        } else {
            step_default
        };
        let tf = if tf.is_finite() && tf > 0.0 {
            tf
        } else {
            step_default
        };
        let pw = if pw.is_finite() && pw >= 0.0 {
            pw
        } else {
            stop_default
        };
        let per = if period_was_omitted {
            // Match ngspice's transient-context defaults for one-shot pulse
            // decks: omitted PER must not restart the waveform before the
            // default high interval has completed inside the active analysis.
            stop_default + tr + pw + tf
        } else if per.is_finite() && per > 0.0 {
            per
        } else {
            stop_default
        };

        (td, tr, tf, pw, per)
    }

    /// ngspice's EXP timing defaults (vsrcload.c): TD1, TAU1, and TAU2
    /// fall back to the transient step when omitted *or zero*, TD2 to
    /// TD1 + step.
    #[inline]
    fn resolve_exp_timing(
        td1: Value,
        tau1: Value,
        td2: Value,
        tau2: Value,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value, Value) {
        let step = Self::pulse_step_default(context);
        let td1 = if td1.is_finite() && td1 != 0.0 {
            td1
        } else {
            step
        };
        let tau1 = if tau1.is_finite() && tau1 != 0.0 {
            tau1
        } else {
            step
        };
        let td2 = if td2.is_finite() && td2 != 0.0 {
            td2
        } else {
            td1 + step
        };
        let tau2 = if tau2.is_finite() && tau2 != 0.0 {
            tau2
        } else {
            step
        };
        (td1, tau1, td2, tau2)
    }

    #[inline]
    fn evaluate_source_at_time_with_context(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        use crate::netlist::SourceSpec;
        use std::f64::consts::PI;

        match spec {
            SourceSpec::Dc(v) => *v,
            SourceSpec::Ac { .. } => 0.0, // AC sources are DC=0 in transient
            // TRNOISE expands into a PWL sample train before circuit
            // construction; an unexpanded spec is zero-mean by definition.
            SourceSpec::TrNoise { .. } => 0.0,
            SourceSpec::DcAc { dc_value, .. } => *dc_value,
            SourceSpec::DcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context(transient, time, context)
            }
            SourceSpec::DcAcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context(transient, time, context)
            }
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                phase,
                width_defaults_to_zero,
            } => {
                let (delay, rise, fall, width, period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *width_defaults_to_zero,
                    context,
                );
                if time < delay {
                    return *v1;
                }
                let phase_time = if period.is_finite() && period > 0.0 {
                    let phase_cycles = (phase / 360.0).rem_euclid(1.0);
                    if phase_cycles > 0.0 {
                        phase_cycles * period - period
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                let t_rel = time - delay + phase_time;
                let t = if period.is_finite() && period > 0.0 && t_rel > period {
                    t_rel - period * (t_rel / period).floor()
                } else {
                    t_rel
                };
                if t <= 0.0 || t >= rise + width + fall {
                    *v1
                } else if t < rise {
                    v1 + (v2 - v1) * t / rise
                } else if t < rise + width {
                    *v2
                } else if t < rise + width + fall {
                    v2 + (v1 - v2) * (t - rise - width) / fall
                } else {
                    *v1
                }
            }
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                let frequency = if frequency.is_finite() && *frequency != 0.0 {
                    *frequency
                } else {
                    Self::sin_frequency_default(context)
                };
                if time < *delay {
                    // ngspice holds VO + VA*sin(PHASE) before the delay,
                    // not the bare offset (vsrcload.c).
                    offset + amplitude * phase.sin()
                } else {
                    let t = time - delay;
                    offset
                        + amplitude
                            * (-damping * t).exp()
                            * (2.0 * PI * frequency * t + phase).sin()
                }
            }
            SourceSpec::Pwl { points } => {
                if points.is_empty() {
                    return 0.0;
                }
                if time <= points[0].0 {
                    return points[0].1;
                }
                if time >= points[points.len() - 1].0 {
                    return points[points.len() - 1].1;
                }
                // Linear interpolation
                for j in 0..points.len() - 1 {
                    if time >= points[j].0 && time < points[j + 1].0 {
                        let (t1, v1) = points[j];
                        let (t2, v2) = points[j + 1];
                        return v1 + (v2 - v1) * (time - t1) / (t2 - t1);
                    }
                }
                0.0
            }
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
            } => {
                let key =
                    PwlCacheKey::new(path, *time_scale, *value_scale, *time_offset, *value_offset);
                match Self::load_pwl_waveform_cached(
                    path,
                    *time_scale,
                    *value_scale,
                    *time_offset,
                    *value_offset,
                ) {
                    Ok(waveform) => waveform.value_at(time),
                    Err(err) => {
                        Self::log_pwl_error_once(key, &err);
                        *value_offset
                    }
                }
            }
            SourceSpec::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            } => {
                let (td1, tau1, td2, tau2) =
                    Self::resolve_exp_timing(*td1, *tau1, *td2, *tau2, context);
                if time <= td1 {
                    *v1
                } else if time <= td2 {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                } else {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                        - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
                }
            }
            SourceSpec::Sffm {
                offset,
                amplitude,
                carrier_freq,
                modulation_index,
                signal_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                // ngspice vsrcload.c SFFM semantics, including the exact
                // omitted-parameter defaults and the MDI clamp.
                let fc = if carrier_freq.is_finite() && *carrier_freq > 0.0 {
                    *carrier_freq
                } else {
                    Self::modulated_frequency_default(5.0, context)
                };
                let fm = if signal_freq.is_finite() && *signal_freq != 0.0 {
                    *signal_freq
                } else {
                    Self::modulated_frequency_default(500.0, context)
                };
                let mdi = if modulation_index.is_finite() {
                    modulation_index.clamp(0.0, fc / fm)
                } else {
                    90.0_f64.min(fc / fm)
                };
                let t = time - delay;
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + amplitude
                            * ((2.0 * PI * fc * t + phasec)
                                + mdi * (2.0 * PI * fm * t + phasem).sin())
                            .sin()
                }
            }
            SourceSpec::Am {
                offset,
                modulation_offset,
                modulation_amplitude,
                modulating_freq,
                carrier_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                // ngspice vsrcload.c AM semantics.
                let fm = if modulating_freq.is_finite() && *modulating_freq > 0.0 {
                    *modulating_freq
                } else {
                    Self::modulated_frequency_default(5.0, context)
                };
                let fc = if carrier_freq.is_finite() && *carrier_freq > 0.0 {
                    *carrier_freq
                } else {
                    Self::modulated_frequency_default(500.0, context)
                };
                let t = time - delay;
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + (modulation_offset
                            + modulation_amplitude * (2.0 * PI * fm * t + phasem).sin())
                            * (2.0 * PI * fc * t + phasec).sin()
                }
            }
        }
    }

    /// Enforce voltage source constraints on solution vector after force-accept
    ///
    /// When Newton iteration fails to converge and we force-accept a solution,
    /// the voltage source node values may not satisfy V(n+) - V(n-) = Vs.
    /// This method corrects the solution vector to enforce this constraint
    /// for display purposes and to prevent drift.
    pub fn enforce_voltage_constraints(&self, solution: &mut [Value], time: Value) -> bool {
        let mut changed = false;
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];

            // Get the source value at this time
            let v_source = match &self.source_specs[i] {
                Some(spec) => {
                    Self::evaluate_source_at_time_with_context(spec, time, self.transient_context)
                }
                None => self.dc_values[i],
            };
            changed |= project_two_terminal_voltage(solution, np, nn, v_source);
        }
        changed
    }
}

/// Current source storage (SoA)
#[derive(Debug, Default, Clone)]
pub struct CurrentSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for HB/AC analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for HB/AC analysis
    pub ac_phases: Vec<Value>,
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
}

impl CurrentSources {
    #[inline]
    fn finite_dc_value(&self, index: usize) -> Value {
        let value = self.dc_values[index];
        if value.is_finite() { value } else { 0.0 }
    }

    pub fn index_by_name(&self, name: &str) -> Option<usize> {
        self.names
            .iter()
            .position(|source_name| source_name.eq_ignore_ascii_case(name))
    }

    pub fn value_at_time(&self, index: usize, time: Value) -> Value {
        let Some(dc_value) = self.dc_values.get(index).copied() else {
            return 0.0;
        };
        let dc_value = if dc_value.is_finite() { dc_value } else { 0.0 };
        match self.source_specs.get(index).and_then(Option::as_ref) {
            Some(spec) => VoltageSources::evaluate_source_at_time_with_context(
                spec,
                time,
                self.transient_context,
            ),
            None => dc_value,
        }
    }

    pub fn values_at_time(&self, time: Value) -> Vec<Value> {
        (0..self.names.len())
            .map(|index| self.value_at_time(index, time))
            .collect()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, dc_value: Value) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
        self.source_specs.push(None);
    }

    /// Add current source with AC parameters
    pub fn add_with_ac(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(None);
    }

    /// Add current source with AC and transient specification.
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
    }

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
    }

    /// Set AC parameters for existing source
    pub fn set_ac(&mut self, index: usize, magnitude: Value, phase: Value) {
        if index < self.ac_magnitudes.len() {
            self.ac_magnitudes[index] = magnitude;
            self.ac_phases[index] = phase;
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Stamp all current sources
    #[inline]
    pub fn stamp_all(&self, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.finite_dc_value(i);

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }

    /// Stamp current sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_scaled(&self, rhs: &mut [Value], scale: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.finite_dc_value(i) * scale;

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }

    /// Update RHS contribution of time-varying current sources at transient time.
    ///
    /// `stamp_dc_direct` already stamped DC values, so this applies only the
    /// delta between waveform and DC.
    #[inline]
    pub fn update_transient_rhs(&self, rhs: &mut [Value], time: Value) {
        for i in 0..self.names.len() {
            let Some(spec) = self.source_specs[i].as_ref() else {
                continue;
            };

            let value = VoltageSources::evaluate_source_at_time_with_context(
                spec,
                time,
                self.transient_context,
            );
            let delta = value - self.finite_dc_value(i);
            if !delta.is_finite() || delta == 0.0 {
                continue;
            }

            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            if np > 0 {
                rhs[np - 1] -= delta;
            }
            if nn > 0 {
                rhs[nn - 1] += delta;
            }
        }
    }

    /// Maximum absolute change expected from time-varying current sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .filter_map(|spec| spec.as_ref())
            .map(|spec| {
                (VoltageSources::evaluate_source_at_time_with_context(spec, t1, context)
                    - VoltageSources::evaluate_source_at_time_with_context(spec, t0, context))
                .abs()
            })
            .fold(0.0, Value::max)
    }

    #[inline]
    pub fn max_dc_to_transient_delta(&self, time: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .enumerate()
            .filter_map(|(idx, spec)| spec.as_ref().map(|spec| (idx, spec)))
            .map(|(idx, spec)| {
                (VoltageSources::evaluate_source_at_time_with_context(spec, time, context)
                    - self.finite_dc_value(idx))
                .abs()
            })
            .fold(0.0, Value::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::SourceSpec;

    fn assert_close(actual: Value, expected: Value) {
        let tolerance = expected.abs().max(1.0) * 1.0e-12;
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} tolerance={tolerance:.17e}"
        );
    }

    fn transient_context(tstep: Value, tstop: Value) -> Option<TransientSourceContext> {
        Some(TransientSourceContext { tstep, tstop })
    }

    #[test]
    fn sin_omitted_frequency_defaults_to_inverse_stop_time() {
        let spec = SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: Value::NAN,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            2.5e-9,
            transient_context(1.0e-9, 10.0e-9),
        );

        assert_close(value, 1.0);
    }

    #[test]
    fn sin_zero_frequency_defaults_to_inverse_stop_time() {
        let spec = SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 0.0,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            2.5e-9,
            transient_context(1.0e-9, 10.0e-9),
        );

        assert_close(value, 1.0);
    }

    #[test]
    fn exp_omitted_timing_resolves_to_ngspice_tstep_defaults() {
        // EXP(0 1): TD1=TAU1=TAU2=tstep, TD2=TD1+tstep (vsrcload.c).
        let spec = SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: Value::NAN,
            tau1: Value::NAN,
            td2: Value::NAN,
            tau2: Value::NAN,
        };
        let ctx = transient_context(1.0e-9, 10.0e-9);

        // Holds V1 through TD1.
        let early = VoltageSources::evaluate_source_at_time_with_context(&spec, 0.5e-9, ctx);
        assert_close(early, 0.0);

        // Rising region: V1 + (V2-V1)*(1 - exp(-(t-TD1)/TAU1)).
        let rising = VoltageSources::evaluate_source_at_time_with_context(&spec, 1.5e-9, ctx);
        assert_close(rising, 1.0 - (-0.5f64).exp());

        // Decaying region adds the TD2 term.
        let decaying = VoltageSources::evaluate_source_at_time_with_context(&spec, 3.0e-9, ctx);
        assert_close(decaying, (1.0 - (-2.0f64).exp()) - (1.0 - (-1.0f64).exp()));
    }

    #[test]
    fn exp_explicit_zero_timing_also_resolves_to_defaults() {
        // ngspice treats an explicit 0.0 for TD1/TAU1/TD2/TAU2 exactly
        // like an omitted value.
        let spec = SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: 0.0,
            tau1: 0.0,
            td2: 0.0,
            tau2: 0.0,
        };
        let ctx = transient_context(1.0e-9, 10.0e-9);
        let rising = VoltageSources::evaluate_source_at_time_with_context(&spec, 1.5e-9, ctx);
        assert_close(rising, 1.0 - (-0.5f64).exp());
    }

    #[test]
    fn sin_holds_phase_value_before_delay() {
        // SIN with PHASE=90deg holds VO + VA*sin(PHASE) until TD.
        let spec = SourceSpec::Sin {
            offset: 1.0,
            amplitude: 2.0,
            frequency: 1.0e3,
            delay: 5.0e-9,
            damping: 0.0,
            phase: std::f64::consts::FRAC_PI_2,
        };
        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            1.0e-9,
            transient_context(1.0e-9, 10.0e-9),
        );
        assert_close(value, 3.0);
    }

    #[test]
    fn pulse_width_omitted_after_explicit_rise_and_fall_defaults_to_zero() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 1.0e-9,
            rise: 2.0e-9,
            fall: 3.0e-9,
            width: Value::NAN,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: true,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            3.5e-9,
            transient_context(0.5e-9, 20.0e-9),
        );

        assert_close(value, 5.0 / 6.0);
    }

    #[test]
    fn pulse_phase_shifts_waveform_like_ngspice_xspice_mode() {
        let spec = SourceSpec::Pulse {
            v1: -1.0,
            v2: 1.0,
            delay: 0.0,
            rise: 1.0e-5,
            fall: 1.0e-5,
            width: 5.0e-4,
            period: 1.0e-3,
            phase: 45.0,
            width_defaults_to_zero: false,
        };

        let ctx = transient_context(2.0e-5, 2.0e-3);
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 0.0, ctx),
            -1.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 8.85e-4, ctx),
            1.0,
        );
    }

    #[test]
    fn current_source_pwl_without_dc_uses_zero_transient_baseline() {
        let mut sources = CurrentSources::new();
        sources.add_with_ac_and_spec(
            "is".to_string(),
            1,
            0,
            Value::NAN,
            0.0,
            0.0,
            Some(SourceSpec::Pwl {
                points: vec![(0.0, 0.0), (1.0e-6, 1.0e-3)],
            }),
        );

        let mut rhs = vec![0.0];
        sources.stamp_all(&mut rhs);
        assert_eq!(rhs[0], 0.0);

        sources.update_transient_rhs(&mut rhs, 0.5e-6);
        assert_close(rhs[0], -0.5e-3);
        assert_close(sources.max_dc_to_transient_delta(0.5e-6), 0.5e-3);
    }
}
