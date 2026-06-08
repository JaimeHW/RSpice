use super::*;

#[derive(Debug, Clone, Copy)]
struct TlineStateSample {
    time: Value,
    v1: Value,
    i1: Value,
    v2: Value,
    i2: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelayedInterpolationMode {
    Linear,
    Quadratic,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct TransmissionLine {
    /// Instance name
    pub name: String,

    // Port 1 nodes
    pub node1_pos: NodeId,
    pub node1_neg: NodeId,

    // Port 2 nodes
    pub node2_pos: NodeId,
    pub node2_neg: NodeId,

    // Parameters
    /// Characteristic impedance (Î©)
    pub z0: Value,
    /// Propagation delay (s)
    pub td: Value,
    /// Frequency for loss calculation (optional)
    pub freq: Option<Value>,
    /// Normalized length (optional)
    pub nl: Option<Value>,
    /// One-way attenuation factor (0 < a <= 1)
    attenuation: Value,
    /// DC equivalent series resistance used to couple near/far conductors
    /// during operating-point solves. `0` means "ideal short fallback".
    dc_series_resistance: Value,
    /// Characteristic loss-dispersion time constant used to smooth the
    /// delayed-wave history for RLGC model-card lines.
    loss_time_constant: Value,

    // Internal state
    /// Branch indices for current variables
    branch1: Option<NodeId>,
    branch2: Option<NodeId>,

    // History buffers for delayed values
    /// V1 + Z0*I1 history
    history_forward: DelayBuffer,
    /// V2 + Z0*I2 history
    history_backward: DelayBuffer,
    /// Smoothed forward wave stored into the delay history
    filtered_forward_wave: Value,
    /// Smoothed backward wave stored into the delay history
    filtered_backward_wave: Value,
    /// Whether the filtered wave state has been seeded yet
    history_initialized: bool,
    /// First accepted port state, retained even if old history samples are trimmed.
    initial_state: Option<TlineStateSample>,
    /// Absolute port state history used by distributed-RLC kernels.
    state_history: VecDeque<TlineStateSample>,
    /// Optional distributed RLC transient kernel configuration.
    distributed_rlc: Option<DistributedRlcKernel>,
    /// Cached transient companion response for the current candidate time.
    distributed_rlc_cache: Cell<Option<(Value, TlineTransientResponse)>>,
    /// Interpolation mode selected by ngspice LTRA model flags.
    ltra_interpolation_mode: DelayedInterpolationMode,
    /// Relative derivative tolerance for ngspice-style LTRA breakpoints.
    ltra_breakpoint_reltol: Value,
    /// Absolute derivative tolerance for ngspice-style LTRA breakpoints.
    ltra_breakpoint_abstol: Value,
    /// Optional ngspice TXL branch-current runtime for non-lossless TXL cards.
    txl: Option<txl::TxlRuntime>,
    /// TXL branch-current ordinals allocated by the circuit builder.
    txl_branch_ordinals: Option<(NodeId, NodeId)>,
    /// LTRA RLC branch-current ordinals allocated by the circuit builder.
    ltra_branch_ordinals: Option<(NodeId, NodeId)>,

    /// Current simulation time
    current_time: Value,
}

impl TransmissionLine {
    #[inline]
    fn quadratic_interp_coefficients(
        t: Value,
        t1: Value,
        t2: Value,
        t3: Value,
    ) -> Option<(Value, Value, Value)> {
        if t == t1 {
            return Some((1.0, 0.0, 0.0));
        }
        if t == t2 {
            return Some((0.0, 1.0, 0.0));
        }
        if t == t3 {
            return Some((0.0, 0.0, 1.0));
        }
        if (t2 - t1) == 0.0 || (t3 - t2) == 0.0 || (t1 - t3) == 0.0 {
            return None;
        }

        let mut f1 = (t - t2) * (t - t3);
        let mut f2 = (t - t1) * (t - t3);
        let mut f3 = (t - t1) * (t - t2);

        f1 /= (t1 - t2) * (t1 - t3);
        f2 /= (t2 - t1) * (t2 - t3);
        f3 /= (t3 - t1) * (t3 - t2);
        Some((f1, f2, f3))
    }

    #[inline]
    fn linear_interp_coefficients(t: Value, t1: Value, t2: Value) -> Option<(Value, Value)> {
        if t1 == t2 {
            return None;
        }
        if t == t1 {
            return Some((1.0, 0.0));
        }
        if t == t2 {
            return Some((0.0, 1.0));
        }
        let w2 = (t - t1) / (t2 - t1);
        Some((1.0 - w2, w2))
    }

    #[inline]
    fn linear_interpolate<F>(
        prev: &TlineStateSample,
        next: &TlineStateSample,
        target: Value,
        selector: F,
    ) -> Value
    where
        F: Fn(&TlineStateSample) -> Value + Copy,
    {
        if let Some((l0, l1)) = Self::linear_interp_coefficients(target, prev.time, next.time) {
            l0 * selector(prev) + l1 * selector(next)
        } else {
            selector(next)
        }
    }

    #[inline]
    fn quadratic_interpolate<F>(
        prev2: Option<&TlineStateSample>,
        prev: &TlineStateSample,
        next: &TlineStateSample,
        target: Value,
        selector: F,
    ) -> Option<Value>
    where
        F: Fn(&TlineStateSample) -> Value + Copy,
    {
        let sample0 = prev2?;
        let (q0, q1, q2) =
            Self::quadratic_interp_coefficients(target, sample0.time, prev.time, next.time)?;
        let v0 = selector(sample0);
        let v1 = selector(prev);
        let v2 = selector(next);
        Some(q0 * v0 + q1 * v1 + q2 * v2)
    }

    #[inline]
    fn delayed_interpolate<F>(
        mode: DelayedInterpolationMode,
        prev2: Option<&TlineStateSample>,
        prev: &TlineStateSample,
        next: &TlineStateSample,
        target: Value,
        selector: F,
    ) -> Value
    where
        F: Fn(&TlineStateSample) -> Value + Copy,
    {
        let linear = || Self::linear_interpolate(prev, next, target, selector);
        match mode {
            DelayedInterpolationMode::Linear => linear(),
            DelayedInterpolationMode::Quadratic => {
                Self::quadratic_interpolate(prev2, prev, next, target, selector)
                    .unwrap_or_else(linear)
            }
            DelayedInterpolationMode::Mixed => {
                if let Some(quadratic) =
                    Self::quadratic_interpolate(prev2, prev, next, target, selector)
                {
                    let prev_value = selector(prev);
                    let next_value = selector(next);
                    if quadratic >= prev_value.min(next_value)
                        && quadratic <= prev_value.max(next_value)
                    {
                        return quadratic;
                    }
                }
                linear()
            }
        }
    }

    /// Create a new lossless transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
    ) -> Self {
        Self {
            name,
            node1_pos,
            node1_neg,
            node2_pos,
            node2_neg,
            z0,
            td,
            freq: None,
            nl: None,
            attenuation: 1.0,
            dc_series_resistance: 0.0,
            loss_time_constant: 0.0,
            branch1: None,
            branch2: None,
            history_forward: DelayBuffer::new(td),
            history_backward: DelayBuffer::new(td),
            filtered_forward_wave: 0.0,
            filtered_backward_wave: 0.0,
            history_initialized: false,
            initial_state: None,
            state_history: VecDeque::new(),
            distributed_rlc: None,
            distributed_rlc_cache: Cell::new(None),
            ltra_interpolation_mode: DelayedInterpolationMode::Quadratic,
            ltra_breakpoint_reltol: 1.0,
            ltra_breakpoint_abstol: 1.0,
            txl: None,
            txl_branch_ordinals: None,
            ltra_branch_ordinals: None,
            current_time: 0.0,
        }
    }

    /// Create from frequency and normalized length
    pub fn from_frequency(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        freq: Value,
        nl: Value,
    ) -> Self {
        // TD = NL / freq (number of wavelengths at frequency)
        let td = nl / freq;

        let mut tl = Self::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);
        tl.freq = Some(freq);
        tl.nl = Some(nl);
        tl
    }

    /// Set branch indices for MNA
    pub fn set_branches(&mut self, branch1: NodeId, branch2: NodeId) {
        self.branch1 = Some(branch1);
        self.branch2 = Some(branch2);
    }

    /// Set branch ordinals for the native TXL runtime.
    pub fn set_txl_branch_ordinals(&mut self, branch1: NodeId, branch2: NodeId) {
        self.txl_branch_ordinals = Some((branch1, branch2));
    }

    /// Set branch ordinals for the scalar LTRA RLC runtime.
    pub fn set_ltra_branch_ordinals(&mut self, branch1: NodeId, branch2: NodeId) {
        self.ltra_branch_ordinals = Some((branch1, branch2));
    }

    /// Return TXL branch ordinals, if this line uses the native TXL runtime.
    #[inline]
    pub fn txl_branch_ordinals(&self) -> Option<(NodeId, NodeId)> {
        self.txl.as_ref()?;
        self.txl_branch_ordinals
    }

    /// Return LTRA RLC branch ordinals, if this line uses branch-current history.
    #[inline]
    pub fn ltra_branch_ordinals(&self) -> Option<(NodeId, NodeId)> {
        self.distributed_rlc.as_ref()?;
        self.txl.is_none().then_some(())?;
        self.ltra_branch_ordinals
    }

    /// Return linked TXL branch matrix indices.
    #[inline]
    pub fn txl_branch_matrix_indices(&self) -> Option<(NodeId, NodeId)> {
        self.txl.as_ref()?;
        Some((self.branch1?, self.branch2?))
    }

    /// Return linked LTRA RLC branch matrix indices.
    #[inline]
    pub fn ltra_branch_matrix_indices(&self) -> Option<(NodeId, NodeId)> {
        self.distributed_rlc.as_ref()?;
        self.txl.is_none().then_some(())?;
        self.ltra_branch_ordinals?;
        Some((self.branch1?, self.branch2?))
    }

    /// Configure the ngspice-style TXL runtime for a non-lossless scalar line.
    pub fn enable_txl_runtime(
        &mut self,
        r: Value,
        l: Value,
        g: Value,
        c: Value,
        len: Value,
    ) -> bool {
        if let Some(runtime) = txl::TxlRuntime::setup(r, l, g, c, len) {
            self.txl = Some(runtime);
            self.distributed_rlc = None;
            self.distributed_rlc_cache.set(None);
            true
        } else {
            false
        }
    }

    /// Return whether this line is using the native non-lossless TXL runtime.
    #[inline]
    pub fn has_txl_runtime(&self) -> bool {
        self.txl.is_some()
    }

    /// Return the current transient TXL branch stamp.
    pub(crate) fn txl_transient_stamp(&self, time: Value) -> Option<TxlTransientStamp> {
        self.txl.as_ref()?.transient_stamp(time)
    }

    /// Seed native TXL history from the operating point.
    pub(crate) fn initialize_txl_history(
        &mut self,
        time: Value,
        v1: Value,
        i1: Value,
        v2: Value,
        i2: Value,
    ) {
        if let Some(txl) = &mut self.txl {
            txl.initialize(time, v1, i1, v2, i2);
        }
    }

    /// Accept a native TXL transient point.
    pub(crate) fn accept_txl_history(
        &mut self,
        time: Value,
        v1: Value,
        i1: Value,
        v2: Value,
        i2: Value,
    ) {
        if let Some(txl) = &mut self.txl {
            txl.accept(time, v1, i1, v2, i2);
        }
    }

    /// Get characteristic impedance
    #[inline]
    pub fn impedance(&self) -> Value {
        self.z0
    }

    /// Get propagation delay
    #[inline]
    pub fn delay(&self) -> Value {
        self.td
    }

    /// Set one-way attenuation factor.
    ///
    /// Values are clamped to the physically meaningful range `(0, 1]`.
    pub fn set_attenuation(&mut self, attenuation: Value) {
        self.attenuation = attenuation.clamp(1e-6, 1.0);
    }

    /// Get one-way attenuation factor.
    #[inline]
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Configure the DC equivalent series resistance used by OP/DC analyses.
    pub fn set_dc_series_resistance(&mut self, resistance: Value) {
        if resistance.is_finite() && resistance > 0.0 {
            self.dc_series_resistance = resistance;
        } else {
            self.dc_series_resistance = 0.0;
        }
    }

    /// Get the configured DC series resistance.
    #[inline]
    pub fn dc_series_resistance(&self) -> Value {
        self.dc_series_resistance
    }

    /// Configure the lossy-line history smoothing time constant.
    pub fn set_loss_time_constant(&mut self, tau: Value) {
        self.loss_time_constant = if tau.is_finite() && tau > 0.0 {
            tau
        } else {
            0.0
        };
    }

    /// Get the configured lossy-line history smoothing time constant.
    #[inline]
    pub fn loss_time_constant(&self) -> Value {
        self.loss_time_constant
    }

    #[inline]
    pub fn has_distributed_rlgc(&self) -> bool {
        self.distributed_rlc.is_some()
    }

    /// Use ngspice LTRA linear interpolation for delayed port states.
    pub fn set_ltra_linear_interpolation(&mut self) {
        self.ltra_interpolation_mode = DelayedInterpolationMode::Linear;
        self.distributed_rlc_cache.set(None);
    }

    /// Use ngspice LTRA quadratic interpolation for delayed port states.
    pub fn set_ltra_quadratic_interpolation(&mut self) {
        self.ltra_interpolation_mode = DelayedInterpolationMode::Quadratic;
        self.distributed_rlc_cache.set(None);
    }

    /// Use ngspice LTRA mixed interpolation for delayed port states.
    pub fn set_ltra_mixed_interpolation(&mut self) {
        self.ltra_interpolation_mode = DelayedInterpolationMode::Mixed;
        self.distributed_rlc_cache.set(None);
    }

    /// Configure a distributed-RLC transient kernel for lossy scalar propagation.
    ///
    /// This follows the ngspice LTRA RLC special case for `G = 0`, which is the
    /// physically relevant regime for the copied transmission regression decks.
    pub fn set_distributed_rlgc(&mut self, r: Value, l: Value, g: Value, c: Value, len: Value) {
        self.set_distributed_rlgc_with_compaction(
            r,
            l,
            g,
            c,
            len,
            DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT,
            DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT,
        );
    }

    /// Configure a distributed-RLC kernel with ngspice-style straight-line
    /// compaction tolerances for its safe-step estimate.
    pub fn set_distributed_rlgc_with_compaction(
        &mut self,
        r: Value,
        l: Value,
        g: Value,
        c: Value,
        len: Value,
        compact_reltol: Value,
        compact_abstol: Value,
    ) {
        if !r.is_finite()
            || !l.is_finite()
            || !g.is_finite()
            || !c.is_finite()
            || !len.is_finite()
            || l <= 0.0
            || c <= 0.0
            || len <= 0.0
            || g.abs() > 1e-18
        {
            self.distributed_rlc = None;
            self.distributed_rlc_cache.set(None);
            return;
        }

        let alpha = 0.5 * (r / l);
        let beta = alpha;
        let attenuation = (-beta * self.td).exp().clamp(1e-6, 1.0);
        let max_safe_step =
            distributed_rlc_max_safe_step(self.td, alpha, beta, compact_reltol, compact_abstol)
                .unwrap_or(self.td);
        self.distributed_rlc = Some(DistributedRlcKernel {
            alpha,
            beta,
            attenuation,
            int_h1dash: if alpha > 0.0 { -1.0 } else { 0.0 },
            int_h2: if alpha > 0.0 { 1.0 - attenuation } else { 0.0 },
            int_h3dash: if alpha > 0.0 { -attenuation } else { 0.0 },
            max_safe_step,
        });
        self.attenuation = attenuation;
        self.distributed_rlc_cache.set(None);
    }

    /// Configure ngspice LTRA derivative-change breakpoint tolerances.
    pub fn set_ltra_breakpoint_tolerances(&mut self, reltol: Value, abstol: Value) {
        self.ltra_breakpoint_reltol = if reltol.is_finite() && reltol >= 0.0 {
            reltol
        } else {
            1.0
        };
        self.ltra_breakpoint_abstol = if abstol.is_finite() && abstol >= 0.0 {
            abstol
        } else {
            1.0
        };
    }

    #[inline]
    fn ltra_wave(sample: &TlineStateSample, z0: Value, attenuation: Value, forward: bool) -> Value {
        if forward {
            (sample.v1 + sample.i1 * z0) * attenuation
        } else {
            (sample.v2 + sample.i2 * z0) * attenuation
        }
    }

    #[inline]
    fn ltra_current_input(v: Value, i: Value, admittance: Value, attenuation: Value) -> Value {
        (v * admittance + i) * attenuation
    }

    #[inline]
    fn ltra_wave_is_steady(v1: Value, v2: Value, v3: Value, reltol: Value, abstol: Value) -> bool {
        let max = v1.max(v2).max(v3);
        let min = v1.min(v2).min(v3);
        let threshold = (50.0 * (reltol / 3.0 * (v1 + v2 + v3) + abstol)).abs();
        max - min < threshold
    }

    #[inline]
    fn ltra_derivative_changed(
        v_curr: Value,
        v_prev: Value,
        v_prev2: Value,
        t_curr: Value,
        t_prev: Value,
        t_prev2: Value,
        deriv_reltol: Value,
        deriv_abstol: Value,
        voltage_reltol: Value,
        steady_abstol: Value,
    ) -> bool {
        let dt_curr = t_curr - t_prev;
        if !(dt_curr.is_finite() && dt_curr > 0.0) {
            return false;
        }
        let d_curr = (v_curr - v_prev) / dt_curr;
        let d_prev = if t_prev > t_prev2 {
            (v_prev - v_prev2) / (t_prev - t_prev2)
        } else {
            0.0
        };
        let threshold = deriv_reltol * d_curr.abs().max(d_prev.abs()) + deriv_abstol;
        (d_curr - d_prev).abs() >= threshold
            && !Self::ltra_wave_is_steady(v_curr, v_prev, v_prev2, voltage_reltol, steady_abstol)
    }

    /// Return the ngspice LTRA candidate-step truncation limit for scalar RLC lines.
    pub(crate) fn ltra_candidate_truncation_limit(
        &self,
        candidate_time: Value,
        candidate_v1: Value,
        candidate_i1: Value,
        candidate_v2: Value,
        candidate_i2: Value,
    ) -> Option<Value> {
        let kernel = self.distributed_rlc.as_ref()?;
        let mut limit = kernel.max_safe_step;

        let len = self.state_history.len();
        if len < 2 {
            return (limit.is_finite() && limit > 0.0).then_some(limit);
        }

        let curr = self.state_history.get(len - 1)?;
        let prev = self.state_history.get(len - 2)?;
        let candidate_dt = candidate_time - curr.time;
        let prev_dt = curr.time - prev.time;
        if !(candidate_dt.is_finite() && candidate_dt > 0.0 && prev_dt.is_finite() && prev_dt > 0.0)
        {
            return (limit.is_finite() && limit > 0.0).then_some(limit);
        }

        let admit = self.conductance();
        let candidate_port2 =
            Self::ltra_current_input(candidate_v2, candidate_i2, admit, kernel.attenuation);
        let curr_port2 = Self::ltra_current_input(curr.v2, curr.i2, admit, kernel.attenuation);
        let prev_port2 = Self::ltra_current_input(prev.v2, prev.i2, admit, kernel.attenuation);
        let candidate_port1 =
            Self::ltra_current_input(candidate_v1, candidate_i1, admit, kernel.attenuation);
        let curr_port1 = Self::ltra_current_input(curr.v1, curr.i1, admit, kernel.attenuation);
        let prev_port1 = Self::ltra_current_input(prev.v1, prev.i1, admit, kernel.attenuation);

        let d1 = (candidate_port2 - curr_port2) / candidate_dt;
        let d2 = (curr_port2 - prev_port2) / prev_dt;
        let d3 = (candidate_port1 - curr_port1) / candidate_dt;
        let d4 = (curr_port1 - prev_port1) / prev_dt;

        let port2_changed = (d1 - d2).abs()
            >= self.ltra_breakpoint_reltol * d1.abs().max(d2.abs()) + self.ltra_breakpoint_abstol;
        let port1_changed = (d3 - d4).abs()
            >= self.ltra_breakpoint_reltol * d3.abs().max(d4.abs()) + self.ltra_breakpoint_abstol;
        if port1_changed || port2_changed {
            limit = limit.min(self.td);
        }

        (limit.is_finite() && limit > 0.0).then_some(limit)
    }

    /// Return the ngspice LTRA derivative breakpoint arrival time, if needed.
    pub(crate) fn ltra_derivative_breakpoint_arrival(
        &self,
        voltage_reltol: Value,
        steady_abstol: Value,
    ) -> Option<Value> {
        let kernel = self.distributed_rlc.as_ref()?;
        let len = self.state_history.len();
        if len < 2 {
            return None;
        }

        let curr = self.state_history.get(len - 1)?;
        let prev = self.state_history.get(len - 2)?;
        let prev2 = if len >= 3 {
            self.state_history.get(len - 3).unwrap_or(prev)
        } else {
            prev
        };

        let forward_changed = Self::ltra_derivative_changed(
            Self::ltra_wave(curr, self.z0, kernel.attenuation, true),
            Self::ltra_wave(prev, self.z0, kernel.attenuation, true),
            Self::ltra_wave(prev2, self.z0, kernel.attenuation, true),
            curr.time,
            prev.time,
            prev2.time,
            self.ltra_breakpoint_reltol,
            self.ltra_breakpoint_abstol,
            voltage_reltol,
            steady_abstol,
        );
        let backward_changed = Self::ltra_derivative_changed(
            Self::ltra_wave(curr, self.z0, kernel.attenuation, false),
            Self::ltra_wave(prev, self.z0, kernel.attenuation, false),
            Self::ltra_wave(prev2, self.z0, kernel.attenuation, false),
            curr.time,
            prev.time,
            prev2.time,
            self.ltra_breakpoint_reltol,
            self.ltra_breakpoint_abstol,
            voltage_reltol,
            steady_abstol,
        );

        if forward_changed || backward_changed {
            Some(prev.time + self.td)
        } else {
            None
        }
    }

    #[inline]
    pub fn distributed_rlgc_max_safe_step(&self) -> Option<Value> {
        self.distributed_rlc
            .as_ref()
            .map(|kernel| kernel.max_safe_step)
    }

    /// Get DC equivalent conductance used by OP/DC fallback stamping.
    #[inline]
    pub fn dc_series_conductance(&self) -> Value {
        let r = if self.dc_series_resistance > 0.0 {
            self.dc_series_resistance
        } else {
            TLINE_DC_SHORT_RESISTANCE
        };
        1.0 / r
    }

    /// Get propagation velocity (if freq and nl are set)
    pub fn velocity(&self) -> Option<Value> {
        match (self.freq, self.nl) {
            (Some(f), Some(nl)) => {
                // v = wavelength * freq = (length/nl) * freq
                // But we don't have physical length, just normalized
                Some(f / nl * self.td)
            }
            _ => None,
        }
    }

    #[inline]
    fn initial_state(&self) -> TlineStateSample {
        self.initial_state.unwrap_or(TlineStateSample {
            time: 0.0,
            v1: 0.0,
            i1: 0.0,
            v2: 0.0,
            i2: 0.0,
        })
    }

    fn delayed_state(&self, time: Value) -> TlineStateSample {
        let target = time - self.td;
        let initial = self.initial_state();
        if self.state_history.is_empty() || target <= initial.time {
            return initial;
        }

        let mode = if self.distributed_rlc.is_some() && self.txl.is_none() {
            self.ltra_interpolation_mode
        } else {
            DelayedInterpolationMode::Quadratic
        };

        let mut prev2: Option<&TlineStateSample> = None;
        let mut prev: Option<&TlineStateSample> = None;
        for sample in &self.state_history {
            if sample.time >= target {
                if let Some(prev_sample) = prev {
                    if sample.time <= prev_sample.time {
                        return *sample;
                    }
                    return TlineStateSample {
                        time: target,
                        v1: Self::delayed_interpolate(
                            mode,
                            prev2,
                            prev_sample,
                            sample,
                            target,
                            |s| s.v1,
                        ),
                        i1: Self::delayed_interpolate(
                            mode,
                            prev2,
                            prev_sample,
                            sample,
                            target,
                            |s| s.i1,
                        ),
                        v2: Self::delayed_interpolate(
                            mode,
                            prev2,
                            prev_sample,
                            sample,
                            target,
                            |s| s.v2,
                        ),
                        i2: Self::delayed_interpolate(
                            mode,
                            prev2,
                            prev_sample,
                            sample,
                            target,
                            |s| s.i2,
                        ),
                    };
                }
                return *sample;
            }
            prev2 = prev;
            prev = Some(sample);
        }

        self.state_history.back().copied().unwrap_or(initial)
    }

    fn distributed_rlc_response(
        &self,
        kernel: &DistributedRlcKernel,
        time: Value,
    ) -> TlineTransientResponse {
        let g = self.conductance();
        let initial = self.initial_state();
        let history_len = self.state_history.len();
        if history_len == 0 {
            return TlineTransientResponse::uncoupled(g, 0.0, 0.0);
        }

        let delayed = self.delayed_state(time);
        let mut input1 = kernel.attenuation * (g * delayed.v2 + delayed.i2);
        let mut input2 = kernel.attenuation * (g * delayed.v1 + delayed.i1);

        let last_time = self
            .state_history
            .back()
            .map(|sample| sample.time)
            .unwrap_or(0.0);
        if time <= last_time {
            return TlineTransientResponse::uncoupled(g, input1, input2);
        }

        let time_list = self
            .state_history
            .iter()
            .map(|sample| sample.time)
            .collect::<Vec<_>>();
        let coeffs = distributed_rlc_coefficients(
            self.td,
            kernel.alpha,
            kernel.beta,
            time,
            &time_list,
            DISTRIBUTED_RLC_CHOP_RELTOL,
        );

        let mut dummy1 = 0.0;
        let mut dummy2 = 0.0;
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h1dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v1 - initial.v1);
                dummy2 += coeff * (sample.v2 - initial.v2);
            }
        }
        dummy1 += initial.v1 * kernel.int_h1dash;
        dummy2 += initial.v2 * kernel.int_h1dash;
        dummy1 -= initial.v1 * coeffs.h1dash_first;
        dummy2 -= initial.v2 * coeffs.h1dash_first;
        input1 -= g * dummy1;
        input2 -= g * dummy2;

        dummy1 = if coeffs.h2_first != 0.0 {
            (delayed.i2 - initial.i2) * coeffs.h2_first
        } else {
            0.0
        };
        dummy2 = if coeffs.h2_first != 0.0 {
            (delayed.i1 - initial.i1) * coeffs.h2_first
        } else {
            0.0
        };
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h2.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.i2 - initial.i2);
                dummy2 += coeff * (sample.i1 - initial.i1);
            }
        }
        dummy1 += initial.i2 * kernel.int_h2;
        dummy2 += initial.i1 * kernel.int_h2;
        input1 += dummy1;
        input2 += dummy2;

        dummy1 = if coeffs.h3dash_first != 0.0 {
            (delayed.v2 - initial.v2) * coeffs.h3dash_first
        } else {
            0.0
        };
        dummy2 = if coeffs.h3dash_first != 0.0 {
            (delayed.v1 - initial.v1) * coeffs.h3dash_first
        } else {
            0.0
        };
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h3dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v2 - initial.v2);
                dummy2 += coeff * (sample.v1 - initial.v1);
            }
        }
        dummy1 += initial.v2 * kernel.int_h3dash;
        dummy2 += initial.v1 * kernel.int_h3dash;
        input1 += g * dummy1;
        input2 += g * dummy2;

        // Match ngspice's LTRA RLC load split: only the local h1dash startup
        // term is stamped into the matrix, while the h2/h3dash first terms stay
        // on the delayed-history RHS. Treating h2/h3dash as same-step matrix
        // coupling creates nonphysical cross-port interaction before one delay.
        TlineTransientResponse::uncoupled(g * (1.0 + coeffs.h1dash_first), input1, input2)
    }

    /// Return the transient companion conductance and equivalent currents.
    pub(crate) fn transient_port_response(&self, time: Value) -> TlineTransientResponse {
        if let Some(kernel) = &self.distributed_rlc {
            if let Some((cached_time, response)) = self.distributed_rlc_cache.get()
                && (cached_time - time).abs() < 1e-18
            {
                return response;
            }
            let response = self.distributed_rlc_response(kernel, time);
            self.distributed_rlc_cache.set(Some((time, response)));
            return response;
        }

        let g = self.conductance();
        let delayed = self.delayed_state(time);
        let i_eq_port1 = self.attenuation * (g * delayed.v2 + delayed.i2);
        let i_eq_port2 = self.attenuation * (g * delayed.v1 + delayed.i1);
        TlineTransientResponse::uncoupled(g, i_eq_port1, i_eq_port2)
    }

    /// Update history buffers with current state
    pub fn update_history(&mut self, time: Value, v1: Value, i1: Value, v2: Value, i2: Value) {
        let raw_forward = v1 + self.z0 * i1;
        let raw_backward = v2 + self.z0 * i2;

        // Store the launched traveling waves directly in the delay history.
        // Timestep-local smoothing made the line response depend on the accepted
        // solver step sequence, which is nonphysical and destabilized delayed
        // arrivals once transmission-line breakpoints were added.
        if !self.history_initialized {
            self.history_initialized = true;
        }
        self.filtered_forward_wave = raw_forward;
        self.filtered_backward_wave = raw_backward;

        // Forward wave: V1 + Z0*I1 propagates to port 2
        self.history_forward.push(time, self.filtered_forward_wave);

        // Backward wave: V2 + Z0*I2 propagates to port 1
        self.history_backward
            .push(time, self.filtered_backward_wave);
        self.state_history.push_back(TlineStateSample {
            time,
            v1,
            i1,
            v2,
            i2,
        });
        if self.initial_state.is_none() {
            self.initial_state = self.state_history.front().copied();
        }
        self.distributed_rlc_cache.set(None);
        if self.distributed_rlc.is_none() {
            let history_horizon = self.td * 1.5;
            while let Some(sample) = self.state_history.front() {
                if time - sample.time > history_horizon {
                    self.state_history.pop_front();
                } else {
                    break;
                }
            }
        }
        self.current_time = time;
    }

    /// Get delayed forward wave (arrives at port 2)
    pub fn delayed_forward(&self) -> Value {
        self.delayed_forward_at(self.current_time)
    }

    /// Get delayed backward wave (arrives at port 1)
    pub fn delayed_backward(&self) -> Value {
        self.delayed_backward_at(self.current_time)
    }

    /// Get delayed forward wave at an explicit simulation time.
    pub fn delayed_forward_at(&self, time: Value) -> Value {
        self.delayed_forward_raw_at(time) * self.attenuation
    }

    /// Get delayed backward wave at an explicit simulation time.
    pub fn delayed_backward_at(&self, time: Value) -> Value {
        self.delayed_backward_raw_at(time) * self.attenuation
    }

    /// Get the delayed forward history wave without applying one-way attenuation.
    pub fn delayed_forward_raw_at(&self, time: Value) -> Value {
        self.history_forward.get_delayed(time, self.td)
    }

    /// Get the delayed backward history wave without applying one-way attenuation.
    pub fn delayed_backward_raw_at(&self, time: Value) -> Value {
        self.history_backward.get_delayed(time, self.td)
    }

    #[inline]
    pub fn launched_forward_wave(&self) -> Value {
        self.filtered_forward_wave
    }

    #[inline]
    pub fn launched_backward_wave(&self) -> Value {
        self.filtered_backward_wave
    }

    /// Reset for new simulation
    pub fn reset(&mut self) {
        self.history_forward.clear();
        self.history_backward.clear();
        self.filtered_forward_wave = 0.0;
        self.filtered_backward_wave = 0.0;
        self.history_initialized = false;
        self.initial_state = None;
        self.state_history.clear();
        self.distributed_rlc_cache.set(None);
        if let Some(txl) = &mut self.txl {
            txl.reset();
        }
        self.current_time = 0.0;
    }

    /// Get equivalent conductance (G = 1/Z0)
    #[inline]
    pub fn conductance(&self) -> Value {
        1.0 / self.z0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn distributed_line(samples: &[(Value, Value)]) -> TransmissionLine {
        let mut line = TransmissionLine::new("TLTRA".to_string(), 1, 0, 2, 0, 1.0, 1.0);
        line.set_distributed_rlgc(1.0, 1.0, 0.0, 1.0, 1.0);
        for &(time, v1) in samples {
            line.update_history(time, v1, 0.0, 0.0, 0.0);
        }
        line
    }

    fn lossless_line(samples: &[(Value, Value)]) -> TransmissionLine {
        let mut line = TransmissionLine::new("TLOSS".to_string(), 1, 0, 2, 0, 1.0, 2.0);
        for &(time, v1) in samples {
            line.update_history(time, v1, 0.0, 0.0, 0.0);
        }
        line
    }

    fn assert_close(actual: Value, expected: Value) {
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn ltra_lininterp_uses_linear_delayed_state() {
        let mut line = distributed_line(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
        line.set_ltra_linear_interpolation();

        assert_close(line.delayed_state(2.5).v1, 2.5);
    }

    #[test]
    fn ltra_quadinterp_uses_quadratic_when_three_points_are_available() {
        let mut line = distributed_line(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
        line.set_ltra_quadratic_interpolation();

        assert_close(line.delayed_state(2.5).v1, 2.25);
    }

    #[test]
    fn ltra_quadinterp_uses_linear_when_previous_point_is_missing() {
        let mut line = distributed_line(&[(1.0, 1.0), (2.0, 4.0)]);
        line.set_ltra_quadratic_interpolation();

        assert_close(line.delayed_state(2.5).v1, 2.5);
    }

    #[test]
    fn ltra_mixedinterp_falls_back_to_linear_on_quadratic_overshoot() {
        let mut line = distributed_line(&[(0.0, 100.0), (1.0, 1.0), (2.0, 2.0)]);
        line.set_ltra_mixed_interpolation();

        assert_close(line.delayed_state(2.5).v1, 1.5);
    }

    #[test]
    fn ltra_steady_wave_guard_uses_current_abstol_floor() {
        let reltol = 0.0;
        let current_abstol = 1.0e-12;
        let voltage_abstol = 1.0e-6;
        let small_wave = 1.0e-9;

        assert!(!TransmissionLine::ltra_wave_is_steady(
            small_wave,
            0.0,
            0.0,
            reltol,
            current_abstol,
        ));
        assert!(TransmissionLine::ltra_wave_is_steady(
            small_wave,
            0.0,
            0.0,
            reltol,
            voltage_abstol,
        ));
    }

    #[test]
    fn ltra_candidate_truncation_keeps_max_safe_step_without_history_pair() {
        let line = distributed_line(&[(0.0, 0.0)]);
        let limit = line
            .ltra_candidate_truncation_limit(0.1, 0.0, 0.0, 0.0, 0.0)
            .unwrap();

        assert_close(limit, line.distributed_rlgc_max_safe_step().unwrap());
    }

    #[test]
    fn ltra_candidate_truncation_cuts_to_delay_on_derivative_change() {
        let mut line = distributed_line(&[(0.0, 0.0), (1.0, 0.0)]);
        line.distributed_rlc.as_mut().unwrap().max_safe_step = 2.0;
        line.set_ltra_breakpoint_tolerances(0.0, 0.5);

        let limit = line
            .ltra_candidate_truncation_limit(2.0, 2.0, 0.0, 0.0, 0.0)
            .unwrap();

        assert_close(limit, line.delay());
    }

    #[test]
    fn ltra_candidate_truncation_uses_current_like_input() {
        let mut line = TransmissionLine::new("TLTRA".to_string(), 1, 0, 2, 0, 2.0, 1.0);
        line.set_distributed_rlgc(1.0, 1.0, 0.0, 1.0, 1.0);
        line.distributed_rlc.as_mut().unwrap().max_safe_step = 2.0;
        line.set_ltra_breakpoint_tolerances(0.0, 0.1);
        line.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
        line.update_history(1.0, 0.0, 0.0, 2.0, -1.0);

        let limit = line
            .ltra_candidate_truncation_limit(2.0, 0.0, 0.0, 4.0, -2.0)
            .unwrap();

        assert_close(limit, 2.0);
    }

    #[test]
    fn non_distributed_lossless_lines_keep_quadratic_delayed_state() {
        let mut line = lossless_line(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
        line.set_ltra_linear_interpolation();

        assert_close(line.delayed_state(3.5).v1, 2.25);
    }
}
