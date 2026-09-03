use super::*;

#[derive(Debug, Clone, Copy)]
struct TlineStateSample {
    time: Value,
    v1: Value,
    i1: Value,
    v2: Value,
    i2: Value,
}

/// Accepted delay history required to continue one scalar transmission line.
///
/// Physical port samples preserve the line equation and breakpoint state. The
/// two delay windows additionally retain their accepted Hermite slopes: the
/// oldest slope can depend on a predecessor that has already aged out, so
/// recomputing it would not be a bit-exact continuation.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TransmissionLineCheckpoint {
    pub(crate) name: String,
    pub(crate) impedance: Value,
    pub(crate) initial_state: Option<[Value; 5]>,
    pub(crate) state_history: Vec<[Value; 5]>,
    pub(crate) forward_history: Vec<[Value; 3]>,
    pub(crate) backward_history: Vec<[Value; 3]>,
    pub(crate) launched_forward: Value,
    pub(crate) launched_backward: Value,
    pub(crate) history_initialized: bool,
    pub(crate) current_time: Value,
}

/// Exact two-port coefficients of a finite-length RG line (`R > 0`, `G > 0`,
/// `L = C = 0`), ngspice's `LTRA_MOD_RG` and Xyce's `N_DEV_LTRA` RG case.
///
/// The line is memoryless: with no reactance the propagation constant
/// `gamma = sqrt(R*G)` and characteristic impedance `Z0 = sqrt(R/G)` are real
/// and frequency independent, so the same ABCD parameters describe the line in
/// DC, AC, transient and every periodic analysis. `A = D = cosh(theta)`,
/// `B = Z0*sinh(theta)` and `C = sinh(theta)/Z0` for `theta = len*sqrt(R*G)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LtraRgTwoPort {
    /// `A = D = cosh(theta)`.
    pub(crate) cosh_theta: Value,
    /// `B = Z0*sinh(theta)`, the open-circuit transfer impedance (ohms).
    pub(crate) transfer_impedance: Value,
    /// `C = sinh(theta)/Z0`, the short-circuit transfer admittance (siemens).
    pub(crate) transfer_admittance: Value,
}

/// The per-unit-length parameters of a distributed RLGC line plus its length.
#[derive(Clone, Copy)]
pub struct DistributedRlgc {
    pub r: Value,
    pub l: Value,
    pub g: Value,
    pub c: Value,
    pub len: Value,
}

/// Three consecutive samples of a port voltage and the times they were taken
/// at, as the LTRA derivative test reads them.
#[derive(Clone, Copy)]
struct LtraDerivativeSamples {
    v_curr: Value,
    v_prev: Value,
    v_prev2: Value,
    t_curr: Value,
    t_prev: Value,
    t_prev2: Value,
}

/// The tolerances the same test compares against.
#[derive(Clone, Copy)]
struct LtraDerivativeTolerances {
    deriv_reltol: Value,
    deriv_abstol: Value,
    voltage_reltol: Value,
    steady_abstol: Value,
}

impl LtraRgTwoPort {
    /// Build the coefficients from per-unit-length `R`, `G` and a length.
    ///
    /// `B` and `C` are evaluated as `R*len*sinhc(theta)` and
    /// `G*len*sinhc(theta)` rather than as `sqrt(R/G)*sinh(theta)`. The two
    /// forms are algebraically identical, but the product form never divides
    /// one per-unit-length parameter by the other, so it stays exact for the
    /// extreme `R/G` ratios where ngspice substitutes its `1e-10` cutoff.
    pub(crate) fn try_new(r: Value, g: Value, len: Value) -> Result<Self, String> {
        if !r.is_finite() || r <= 0.0 {
            return Err(format!("RG line requires a finite positive R, got {r}"));
        }
        if !g.is_finite() || g <= 0.0 {
            return Err(format!("RG line requires a finite positive G, got {g}"));
        }
        if !len.is_finite() || len <= 0.0 {
            return Err(format!(
                "RG line requires a finite positive length, got {len}"
            ));
        }
        let theta = len * (r * g).sqrt();
        if !theta.is_finite() {
            return Err(format!(
                "RG line propagation constant len*sqrt(R*G) is not representable for R={r}, G={g}, LEN={len}"
            ));
        }
        let cosh_theta = theta.cosh();
        // sinh(theta)/theta, continuous at theta = 0 and evaluated without a
        // catastrophic cancellation for the small-theta lines that dominate
        // practical RG cards.
        let sinhc = if theta == 0.0 {
            1.0
        } else {
            theta.sinh() / theta
        };
        let transfer_impedance = r * len * sinhc;
        let transfer_admittance = g * len * sinhc;
        if !cosh_theta.is_finite()
            || !transfer_impedance.is_finite()
            || !transfer_admittance.is_finite()
            || cosh_theta < 1.0
            || transfer_impedance <= 0.0
            || transfer_admittance <= 0.0
        {
            return Err(format!(
                "RG line two-port coefficients are not representable for R={r}, G={g}, LEN={len} \
                 (cosh={cosh_theta}, B={transfer_impedance}, C={transfer_admittance})"
            ));
        }
        Ok(Self {
            cosh_theta,
            transfer_impedance,
            transfer_admittance,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DelayedInterpolationMode {
    Linear,
    Quadratic,
    Mixed,
    /// Xyce TRA's quadratic interpolation with its per-wave derivative guard.
    XyceTra,
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
    /// Characteristic impedance (Ω)
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
    /// Most recent forward wave (V1 + Z0*I1) stored into the delay history
    launched_forward: Value,
    /// Most recent backward wave (V2 + Z0*I2) stored into the delay history
    launched_backward: Value,
    /// Whether the wave history has been seeded yet
    history_initialized: bool,
    /// First accepted port state, retained even if old history samples are trimmed.
    initial_state: Option<TlineStateSample>,
    /// Absolute port state history used by distributed-RLC kernels.
    state_history: VecDeque<TlineStateSample>,
    /// Optional distributed RLC transient kernel configuration.
    distributed_rlc: Option<DistributedRlcKernel>,
    /// Optional distributed RC transient kernel configuration.
    distributed_rc: Option<DistributedRcKernel>,
    /// Cached transient companion response for the current candidate time.
    distributed_rlc_cache: Cell<Option<(Value, TlineTransientResponse)>>,
    /// Interpolation mode selected by ngspice LTRA model flags.
    ltra_interpolation_mode: DelayedInterpolationMode,
    /// Interpolation policy for an ordinary lossless transmission line.
    lossless_interpolation_mode: DelayedInterpolationMode,
    /// Relative derivative tolerance for ngspice-style LTRA breakpoints.
    ltra_breakpoint_reltol: Value,
    /// Absolute derivative tolerance for ngspice-style LTRA breakpoints.
    ltra_breakpoint_abstol: Value,
    /// Whether accepted LTRA histories use Xyce's straight-line compaction.
    ltra_history_compaction: bool,
    /// Relative area tolerance for accepted-history compaction.
    ltra_compact_reltol: Value,
    /// Absolute area tolerance for accepted-history compaction.
    ltra_compact_abstol: Value,
    /// Xyce LTRA propagation-delay step limiting policy.
    ltra_step_limit: bool,
    /// Disable the RLC impulse-response safe-step cut.
    ltra_trunc_dont_cut: bool,
    /// Optional ngspice TXL branch-current runtime for non-lossless TXL cards.
    txl: Option<txl::TxlRuntime>,
    /// TXL branch-current ordinals allocated by the circuit builder.
    txl_branch_ordinals: Option<(NodeId, NodeId)>,
    /// LTRA RLC branch-current ordinals allocated by the circuit builder.
    ltra_branch_ordinals: Option<(NodeId, NodeId)>,
    /// Whether this LTRA card is Xyce's zero-length RC/RG ideal through case.
    /// Such a line has no propagation history or distributed kernel; it is
    /// represented by an exact two-port short in every analysis.
    zero_length_pass_through: bool,
    /// Branch-current ordinals reserved for the zero-length through case.
    zero_length_branch_ordinals: Option<(NodeId, NodeId)>,
    /// Exact coefficients of a finite-length memoryless RG line.
    ltra_rg: Option<LtraRgTwoPort>,
    /// Branch-current ordinals reserved for the RG two-port.
    rg_branch_ordinals: Option<(NodeId, NodeId)>,

    /// Current simulation time
    current_time: Value,
}

impl TransmissionLine {
    #[inline]
    fn checkpoint_sample(sample: TlineStateSample) -> [Value; 5] {
        [sample.time, sample.v1, sample.i1, sample.v2, sample.i2]
    }

    #[inline]
    fn sample_from_checkpoint(sample: [Value; 5]) -> TlineStateSample {
        TlineStateSample {
            time: sample[0],
            v1: sample[1],
            i1: sample[2],
            v2: sample[3],
            i2: sample[4],
        }
    }

    /// Capture the accepted history of an ordinary lossless scalar line.
    ///
    /// Distributed LTRA/TXL kernels own additional convolution state and must
    /// not silently resume from an incomplete snapshot.  They fail closed
    /// until their complete native state has a versioned checkpoint contract.
    pub(crate) fn checkpoint_state(&self) -> Result<TransmissionLineCheckpoint, String> {
        if self.txl.is_some() || self.distributed_rlc.is_some() || self.distributed_rc.is_some() {
            return Err(format!(
                "transmission line '{}': distributed LTRA/TXL convolution state is not checkpointable",
                self.name
            ));
        }
        let checkpoint = TransmissionLineCheckpoint {
            name: self.name.clone(),
            impedance: self.z0,
            initial_state: self.initial_state.map(Self::checkpoint_sample),
            state_history: self
                .state_history
                .iter()
                .copied()
                .map(Self::checkpoint_sample)
                .collect(),
            forward_history: self.history_forward.checkpoint_samples(),
            backward_history: self.history_backward.checkpoint_samples(),
            launched_forward: self.launched_forward,
            launched_backward: self.launched_backward,
            history_initialized: self.history_initialized,
            current_time: self.current_time,
        };
        Self::validate_checkpoint_state(&checkpoint)?;
        Ok(checkpoint)
    }

    pub(crate) fn validate_checkpoint_state(
        checkpoint: &TransmissionLineCheckpoint,
    ) -> Result<(), String> {
        if checkpoint.name.is_empty() || checkpoint.name.chars().any(char::is_whitespace) {
            return Err("transmission-line checkpoint has an invalid instance name".to_string());
        }
        if !checkpoint.impedance.is_finite() || checkpoint.impedance <= 0.0 {
            return Err(format!(
                "transmission line '{}': checkpoint impedance must be finite and positive",
                checkpoint.name
            ));
        }
        if !checkpoint.current_time.is_finite() || checkpoint.current_time < 0.0 {
            return Err(format!(
                "transmission line '{}': checkpoint time must be finite and non-negative",
                checkpoint.name
            ));
        }
        if !checkpoint.launched_forward.is_finite() || !checkpoint.launched_backward.is_finite() {
            return Err(format!(
                "transmission line '{}': checkpoint waves must be finite",
                checkpoint.name
            ));
        }
        let validate_sample =
            |sample: &[Value; 5]| sample.iter().all(|value| value.is_finite()) && sample[0] >= 0.0;
        if checkpoint
            .initial_state
            .as_ref()
            .is_some_and(|sample| !validate_sample(sample))
            || checkpoint
                .state_history
                .iter()
                .any(|sample| !validate_sample(sample))
        {
            return Err(format!(
                "transmission line '{}': checkpoint history must be finite with non-negative times",
                checkpoint.name
            ));
        }
        let validate_delay_history = |history: &[[Value; 3]]| {
            history
                .iter()
                .all(|sample| sample.iter().all(|value| value.is_finite()) && sample[0] >= 0.0)
                && !history
                    .windows(2)
                    .any(|window| window[1][0] <= window[0][0])
        };
        if !validate_delay_history(&checkpoint.forward_history)
            || !validate_delay_history(&checkpoint.backward_history)
        {
            return Err(format!(
                "transmission line '{}': checkpoint delay windows must be finite with strictly increasing non-negative times",
                checkpoint.name
            ));
        }
        if checkpoint.forward_history.len() != checkpoint.state_history.len()
            || checkpoint.backward_history.len() != checkpoint.state_history.len()
            || checkpoint
                .state_history
                .iter()
                .zip(&checkpoint.forward_history)
                .zip(&checkpoint.backward_history)
                .any(|((state, forward), backward)| {
                    state[0].to_bits() != forward[0].to_bits()
                        || state[0].to_bits() != backward[0].to_bits()
                        || (state[1] + checkpoint.impedance * state[2]).to_bits()
                            != forward[1].to_bits()
                        || (state[3] + checkpoint.impedance * state[4]).to_bits()
                            != backward[1].to_bits()
                })
        {
            return Err(format!(
                "transmission line '{}': checkpoint physical and delay histories disagree",
                checkpoint.name
            ));
        }
        if checkpoint
            .state_history
            .windows(2)
            .any(|window| window[1][0] <= window[0][0])
        {
            return Err(format!(
                "transmission line '{}': checkpoint history times must be strictly increasing",
                checkpoint.name
            ));
        }
        if checkpoint
            .state_history
            .last()
            .is_some_and(|sample| sample[0] > checkpoint.current_time)
            || checkpoint
                .initial_state
                .as_ref()
                .is_some_and(|sample| sample[0] > checkpoint.current_time)
            || checkpoint.initial_state.as_ref().is_some_and(|initial| {
                checkpoint
                    .state_history
                    .first()
                    .is_some_and(|first| initial[0] > first[0])
            })
        {
            return Err(format!(
                "transmission line '{}': checkpoint history has an invalid time extent",
                checkpoint.name
            ));
        }
        if checkpoint.history_initialized
            != (checkpoint.initial_state.is_some() && !checkpoint.state_history.is_empty())
        {
            return Err(format!(
                "transmission line '{}': checkpoint initialization flag disagrees with its history",
                checkpoint.name
            ));
        }
        if !checkpoint.history_initialized
            && (checkpoint.initial_state.is_some()
                || !checkpoint.state_history.is_empty()
                || !checkpoint.forward_history.is_empty()
                || !checkpoint.backward_history.is_empty()
                || checkpoint.launched_forward.to_bits() != 0.0f64.to_bits()
                || checkpoint.launched_backward.to_bits() != 0.0f64.to_bits()
                || checkpoint.current_time.to_bits() != 0.0f64.to_bits())
        {
            return Err(format!(
                "transmission line '{}': an uninitialized checkpoint must contain canonical empty state",
                checkpoint.name
            ));
        }
        if let Some(state) = checkpoint.state_history.last() {
            let forward = state[1] + checkpoint.impedance * state[2];
            let backward = state[3] + checkpoint.impedance * state[4];
            if forward.to_bits() != checkpoint.launched_forward.to_bits()
                || backward.to_bits() != checkpoint.launched_backward.to_bits()
                || state[0].to_bits() != checkpoint.current_time.to_bits()
            {
                return Err(format!(
                    "transmission line '{}': checkpoint terminal state disagrees with its launched waves or time",
                    checkpoint.name
                ));
            }
        }
        Ok(())
    }

    /// Restore an ordinary lossless scalar line from accepted delay history.
    pub(crate) fn restore_checkpoint_state(
        &mut self,
        checkpoint: &TransmissionLineCheckpoint,
    ) -> Result<(), String> {
        Self::validate_checkpoint_state(checkpoint)?;
        if self.name != checkpoint.name {
            return Err(format!(
                "transmission-line checkpoint instance '{}' does not match '{}'",
                checkpoint.name, self.name
            ));
        }
        if self.z0.to_bits() != checkpoint.impedance.to_bits() {
            return Err(format!(
                "transmission-line checkpoint impedance {} does not match {} for '{}'",
                checkpoint.impedance, self.z0, self.name
            ));
        }
        if self.txl.is_some() || self.distributed_rlc.is_some() || self.distributed_rc.is_some() {
            return Err(format!(
                "transmission line '{}': refusing to inject lossless history into a distributed LTRA/TXL runtime",
                self.name
            ));
        }

        self.reset();
        self.initial_state = checkpoint.initial_state.map(Self::sample_from_checkpoint);
        self.history_initialized = checkpoint.history_initialized;
        for sample in checkpoint.state_history.iter().copied() {
            let sample = Self::sample_from_checkpoint(sample);
            self.state_history.push_back(sample);
        }
        self.history_forward
            .restore_checkpoint_samples(&checkpoint.forward_history)?;
        self.history_backward
            .restore_checkpoint_samples(&checkpoint.backward_history)?;
        self.launched_forward = checkpoint.launched_forward;
        self.launched_backward = checkpoint.launched_backward;
        self.current_time = checkpoint.current_time;
        self.distributed_rlc_cache.set(None);
        Ok(())
    }

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
            DelayedInterpolationMode::XyceTra => {
                let Some(previous) = prev2 else {
                    return linear();
                };
                let previous_dt = prev.time - previous.time;
                let next_dt = next.time - prev.time;
                if !(previous_dt.is_finite()
                    && previous_dt > 0.0
                    && next_dt.is_finite()
                    && next_dt > 0.0)
                {
                    return linear();
                }

                let previous_value = selector(previous);
                let current_value = selector(prev);
                let next_value = selector(next);
                let previous_slope = (current_value - previous_value) / previous_dt;
                let next_slope = (next_value - current_value) / next_dt;
                let derivative_changed = (next_slope - previous_slope).abs()
                    >= 0.99 * next_slope.abs().max(previous_slope.abs()) + 1.0;
                if derivative_changed {
                    // Match Xyce TRA's pathological-flat-segment guard before
                    // applying its linear interpolation from t2 to t3.
                    if (next_value - current_value).abs() < Value::EPSILON {
                        0.5 * (next_value + current_value)
                    } else {
                        current_value + next_slope * (target - prev.time)
                    }
                } else {
                    Self::quadratic_interpolate(Some(previous), prev, next, target, selector)
                        .unwrap_or_else(linear)
                }
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
            launched_forward: 0.0,
            launched_backward: 0.0,
            history_initialized: false,
            initial_state: None,
            state_history: VecDeque::new(),
            distributed_rlc: None,
            distributed_rc: None,
            distributed_rlc_cache: Cell::new(None),
            ltra_interpolation_mode: DelayedInterpolationMode::Quadratic,
            lossless_interpolation_mode: DelayedInterpolationMode::Quadratic,
            ltra_breakpoint_reltol: 1.0,
            ltra_breakpoint_abstol: 1.0,
            ltra_history_compaction: false,
            ltra_compact_reltol: DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT,
            ltra_compact_abstol: DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT,
            ltra_step_limit: true,
            ltra_trunc_dont_cut: false,
            txl: None,
            txl_branch_ordinals: None,
            ltra_branch_ordinals: None,
            zero_length_pass_through: false,
            zero_length_branch_ordinals: None,
            ltra_rg: None,
            rg_branch_ordinals: None,
            current_time: 0.0,
        }
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

    /// Configure the exact zero-length RC/RG through connection.
    pub fn set_zero_length_pass_through(&mut self) {
        self.zero_length_pass_through = true;
        self.txl = None;
        self.distributed_rlc = None;
        self.distributed_rc = None;
        self.distributed_rlc_cache.set(None);
    }

    /// Return whether this line is an exact zero-length RC/RG through case.
    #[inline]
    pub fn is_zero_length_pass_through(&self) -> bool {
        self.zero_length_pass_through
    }

    /// Set branch ordinals for the zero-length through connection.
    pub fn set_zero_length_branch_ordinals(&mut self, branch1: NodeId, branch2: NodeId) {
        self.zero_length_branch_ordinals = Some((branch1, branch2));
    }

    /// Return zero-length through branch ordinals, if configured.
    #[inline]
    pub fn zero_length_branch_ordinals(&self) -> Option<(NodeId, NodeId)> {
        self.zero_length_pass_through
            .then_some(self.zero_length_branch_ordinals)
            .flatten()
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
        if self.distributed_rlc.is_none() && self.distributed_rc.is_none() {
            return None;
        }
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
        if self.distributed_rlc.is_none() && self.distributed_rc.is_none() {
            return None;
        }
        self.txl.is_none().then_some(())?;
        self.ltra_branch_ordinals?;
        Some((self.branch1?, self.branch2?))
    }

    /// Return linked zero-length through branch matrix indices.
    #[inline]
    pub fn zero_length_branch_matrix_indices(&self) -> Option<(NodeId, NodeId)> {
        self.zero_length_pass_through.then_some(())?;
        self.zero_length_branch_ordinals?;
        Some((self.branch1?, self.branch2?))
    }

    /// Configure the exact finite-length RG two-port from its model card.
    ///
    /// An RG line has no reactance and therefore no propagation history: the
    /// delayed-wave and distributed kernels are cleared so nothing else can
    /// claim this instance.
    pub(crate) fn set_ltra_rg_two_port(&mut self, two_port: LtraRgTwoPort) {
        self.ltra_rg = Some(two_port);
        self.txl = None;
        self.distributed_rlc = None;
        self.distributed_rc = None;
        self.distributed_rlc_cache.set(None);
        self.zero_length_pass_through = false;
    }

    /// The exact RG two-port coefficients, if this line is an RG line.
    #[inline]
    pub(crate) fn ltra_rg_two_port(&self) -> Option<LtraRgTwoPort> {
        self.ltra_rg
    }

    /// Whether this line's branch equations are frequency independent and
    /// carry no propagation history.
    ///
    /// Both memoryless cases answer the same question for every analysis that
    /// must know whether a line contributes retained state: the `LEN=0`
    /// ideal-through special case and the finite-length RG line.
    #[inline]
    pub(crate) fn is_memoryless_two_port(&self) -> bool {
        self.zero_length_pass_through || self.ltra_rg.is_some()
    }

    /// Set branch ordinals for the RG two-port.
    pub(crate) fn set_rg_branch_ordinals(&mut self, branch1: NodeId, branch2: NodeId) {
        self.rg_branch_ordinals = Some((branch1, branch2));
    }

    /// Return RG two-port branch ordinals, if configured.
    #[inline]
    pub(crate) fn rg_branch_ordinals(&self) -> Option<(NodeId, NodeId)> {
        self.ltra_rg.as_ref()?;
        self.rg_branch_ordinals
    }

    /// Return linked RG two-port branch matrix indices.
    #[inline]
    pub(crate) fn rg_branch_matrix_indices(&self) -> Option<(NodeId, NodeId)> {
        self.ltra_rg.as_ref()?;
        self.rg_branch_ordinals?;
        Some((self.branch1?, self.branch2?))
    }

    /// Configure the ngspice-style TXL runtime for a non-lossless scalar line.
    pub fn enable_txl_runtime(&mut self, rlgc: DistributedRlgc) -> bool {
        let DistributedRlgc { r, l, g, c, len } = rlgc;
        if let Some(runtime) = txl::TxlRuntime::setup(r, l, g, c, len) {
            self.txl = Some(runtime);
            self.distributed_rlc = None;
            self.distributed_rc = None;
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
        self.distributed_rlc.is_some() || self.distributed_rc.is_some()
    }

    /// Use ngspice LTRA linear interpolation for delayed port states.
    pub fn set_ltra_linear_interpolation(&mut self) {
        self.ltra_interpolation_mode = DelayedInterpolationMode::Linear;
        self.distributed_rlc_cache.set(None);
    }

    /// Enable Xyce-compatible accepted-history compaction for a native LTRA.
    ///
    /// Xyce requires linear interpolation whenever compaction is enabled.
    pub fn set_ltra_history_compaction(
        &mut self,
        enabled: bool,
        compact_reltol: Value,
        compact_abstol: Value,
    ) {
        self.ltra_history_compaction = enabled;
        self.ltra_compact_reltol = compact_reltol;
        self.ltra_compact_abstol = compact_abstol;
        if enabled {
            self.set_ltra_linear_interpolation();
        }
    }

    /// Configure Xyce LTRA timestep flags from the resolved model card.
    pub fn set_ltra_timestep_policy(&mut self, step_limit: bool, trunc_dont_cut: bool) {
        self.ltra_step_limit = step_limit;
        self.ltra_trunc_dont_cut = trunc_dont_cut;
    }

    #[inline]
    fn ltra_straight_line_check(
        first: (Value, Value),
        middle: (Value, Value),
        last: (Value, Value),
        reltol: Value,
        abstol: Value,
    ) -> bool {
        let (x1, y1) = first;
        let (x2, y2) = middle;
        let (x3, y3) = last;
        if !([x1, y1, x2, y2, x3, y3, reltol, abstol]
            .into_iter()
            .all(Value::is_finite)
            && x1 < x2
            && x2 < x3)
            || reltol < 0.0
            || abstol < 0.0
        {
            return false;
        }
        let area1 = 0.5 * (y2.abs() + y1.abs()) * (x2 - x1);
        let area2 = 0.5 * (y3.abs() + y2.abs()) * (x3 - x2);
        let area3 = 0.5 * (y3.abs() + y1.abs()) * (x3 - x1);
        let triangle_area = (area3 - area1 - area2).abs();
        let middle_weight = (x2 - x1) / (x3 - x1);
        let interpolated_middle = y1 + middle_weight * (y3 - y1);
        // The upstream area metric is timestep-scaled and can become too
        // permissive on dense per-device histories. Preserve the intended
        // integral tolerance while also bounding the actual removed sample to
        // COMPACTABS plus floating-point roundoff. This prevents cumulative
        // waveform drift without disabling exact straight-segment compaction.
        let pointwise_scale = y1.abs().max(y2.abs()).max(y3.abs());
        let pointwise_tolerance = abstol + 16.0 * Value::EPSILON * pointwise_scale;
        (area1 + area2) * reltol + abstol > triangle_area
            && (y2 - interpolated_middle).abs() <= pointwise_tolerance
    }

    /// Compact the penultimate accepted LTRA point when all four terminal
    /// histories satisfy Xyce's triangle-area straight-line criterion.
    ///
    /// Call this only after derivative-breakpoint detection has observed the
    /// un-compacted three-point history.
    pub(crate) fn compact_ltra_history_if_straight(&mut self) -> bool {
        if !self.ltra_history_compaction
            || !self.has_distributed_rlgc()
            || self.state_history.len() < 3
        {
            return false;
        }
        let len = self.state_history.len();
        let first = self.state_history[len - 3];
        let middle = self.state_history[len - 2];
        let last = self.state_history[len - 1];
        let straight = |selector: fn(&TlineStateSample) -> Value| {
            Self::ltra_straight_line_check(
                (first.time, selector(&first)),
                (middle.time, selector(&middle)),
                (last.time, selector(&last)),
                self.ltra_compact_reltol,
                self.ltra_compact_abstol,
            )
        };
        if !straight(|sample| sample.v1)
            || !straight(|sample| sample.v2)
            || !straight(|sample| sample.i1)
            || !straight(|sample| sample.i2)
        {
            return false;
        }

        self.state_history.remove(len - 2);
        self.distributed_rlc_cache.set(None);
        true
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
            DistributedRlgc { r, l, g, c, len },
            DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT,
            DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT,
        );
    }

    /// Configure the finite-length RC special case of a scalar LTRA model.
    ///
    /// RC lines have no finite propagation delay.  They therefore use the
    /// exact diffusion convolution rather than the delayed-wave/RLC kernel.
    pub fn set_distributed_rc(&mut self, r: Value, c: Value, len: Value) {
        if !r.is_finite()
            || !c.is_finite()
            || !len.is_finite()
            || r <= 0.0
            || c <= 0.0
            || len <= 0.0
        {
            self.distributed_rc = None;
            self.distributed_rlc_cache.set(None);
            return;
        }

        let c_by_r = c / r;
        let rc_len_squared = r * c * len * len;
        let total_resistance = r * len;
        let total_capacitance = c * len;
        if ![c_by_r, rc_len_squared, total_resistance, total_capacitance]
            .into_iter()
            .all(Value::is_finite)
        {
            self.distributed_rc = None;
            self.distributed_rlc_cache.set(None);
            return;
        }

        self.distributed_rlc = None;
        self.distributed_rc = Some(DistributedRcKernel {
            c_by_r,
            rc_len_squared,
            total_resistance,
            total_capacitance,
        });
        self.distributed_rlc_cache.set(None);
    }

    /// Return whether this line uses the finite-length RC LTRA kernel.
    #[inline]
    pub fn is_distributed_rc(&self) -> bool {
        self.distributed_rc.is_some()
    }

    /// Configure a distributed-RLC kernel with ngspice-style straight-line
    /// compaction tolerances for its safe-step estimate.
    pub fn set_distributed_rlgc_with_compaction(
        &mut self,
        rlgc: DistributedRlgc,
        compact_reltol: Value,
        compact_abstol: Value,
    ) {
        let DistributedRlgc { r, l, g, c, len } = rlgc;
        if !r.is_finite()
            || !l.is_finite()
            || !g.is_finite()
            || !c.is_finite()
            || !len.is_finite()
            || l <= 0.0
            || c <= 0.0
            || len <= 0.0
            || g != 0.0
        {
            self.distributed_rlc = None;
            self.distributed_rc = None;
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
        self.distributed_rc = None;
        self.attenuation = attenuation;
        self.distributed_rlc_cache.set(None);
    }

    /// Total series resistance, inductance, and shunt capacitance for the
    /// native LTRA RLC kernel, reconstructed from the stored characteristic
    /// impedance, delay, and kernel attenuation constant: `Ltot = Z0*TD`,
    /// `Ctot = TD/Z0`, `Rtot = 2*alpha*Ltot` (ngspice LTRAtemp, G = 0).
    /// Used by the exact small-signal load (ltraacld.c form).
    pub(crate) fn ltra_ac_total_rlc(&self) -> Option<(Value, Value, Value)> {
        if self.txl.is_some() {
            return None;
        }
        if let Some(kernel) = self.distributed_rc {
            return Some((0.0, kernel.total_capacitance, kernel.total_resistance));
        }
        let kernel = self.distributed_rlc.as_ref()?;
        let ltot = self.z0 * self.td;
        let ctot = self.td / self.z0;
        let rtot = 2.0 * kernel.alpha * ltot;
        (ltot.is_finite()
            && ctot.is_finite()
            && rtot.is_finite()
            && ltot > 0.0
            && ctot > 0.0
            && rtot >= 0.0)
            .then_some((ltot, ctot, rtot))
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

    /// Select Xyce's lossless TRA interpolation policy.
    ///
    /// Xyce normally uses three-point quadratic interpolation, but switches
    /// each launched traveling wave to linear interpolation when consecutive
    /// derivatives indicate a discontinuity. This prevents quadratic ringing
    /// immediately after a propagated source edge.
    pub(crate) fn set_xyce_tra_interpolation(&mut self) {
        self.lossless_interpolation_mode = DelayedInterpolationMode::XyceTra;
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
        samples: LtraDerivativeSamples,
        tolerances: LtraDerivativeTolerances,
    ) -> bool {
        let LtraDerivativeSamples {
            v_curr,
            v_prev,
            v_prev2,
            t_curr,
            t_prev,
            t_prev2,
        } = samples;
        let LtraDerivativeTolerances {
            deriv_reltol,
            deriv_abstol,
            voltage_reltol,
            steady_abstol,
        } = tolerances;
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
        let safe_limit = if self.ltra_trunc_dont_cut {
            Value::INFINITY
        } else {
            kernel.max_safe_step
        };

        let len = self.state_history.len();
        if len < 2 {
            let limit = self.td.min(safe_limit);
            return (limit.is_finite() && limit > 0.0).then_some(limit);
        }

        let mut limit = if self.ltra_step_limit {
            self.td.min(safe_limit)
        } else {
            safe_limit
        };

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
            LtraDerivativeSamples {
                v_curr: Self::ltra_wave(curr, self.z0, kernel.attenuation, true),
                v_prev: Self::ltra_wave(prev, self.z0, kernel.attenuation, true),
                v_prev2: Self::ltra_wave(prev2, self.z0, kernel.attenuation, true),
                t_curr: curr.time,
                t_prev: prev.time,
                t_prev2: prev2.time,
            },
            LtraDerivativeTolerances {
                deriv_reltol: self.ltra_breakpoint_reltol,
                deriv_abstol: self.ltra_breakpoint_abstol,
                voltage_reltol,
                steady_abstol,
            },
        );
        let backward_changed = Self::ltra_derivative_changed(
            LtraDerivativeSamples {
                v_curr: Self::ltra_wave(curr, self.z0, kernel.attenuation, false),
                v_prev: Self::ltra_wave(prev, self.z0, kernel.attenuation, false),
                v_prev2: Self::ltra_wave(prev2, self.z0, kernel.attenuation, false),
                t_curr: curr.time,
                t_prev: prev.time,
                t_prev2: prev2.time,
            },
            LtraDerivativeTolerances {
                deriv_reltol: self.ltra_breakpoint_reltol,
                deriv_abstol: self.ltra_breakpoint_abstol,
                voltage_reltol,
                steady_abstol,
            },
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

    /// Interpolate one launched lossless-line wave from accepted port history.
    ///
    /// TRA stores and interpolates `V + Z0*I`, not voltage and current as
    /// independent signals. Keeping the derivative decision on that combined
    /// wave is necessary because different fallback decisions for V and I are
    /// not algebraically equivalent to the canonical device equation.
    fn lossless_delayed_wave(&self, time: Value, forward: bool) -> Value {
        let target = time - self.td;
        let initial = self.initial_state();
        let wave = |sample: &TlineStateSample| {
            if forward {
                sample.v1 + self.z0 * sample.i1
            } else {
                sample.v2 + self.z0 * sample.i2
            }
        };
        if self.state_history.is_empty() || target <= initial.time {
            return wave(&initial);
        }

        let mut prev2: Option<&TlineStateSample> = None;
        let mut prev: Option<&TlineStateSample> = None;
        for sample in &self.state_history {
            if sample.time >= target {
                if let Some(prev_sample) = prev {
                    if sample.time <= prev_sample.time {
                        return wave(sample);
                    }
                    return Self::delayed_interpolate(
                        self.lossless_interpolation_mode,
                        prev2,
                        prev_sample,
                        sample,
                        target,
                        wave,
                    );
                }
                return wave(sample);
            }
            prev2 = prev;
            prev = Some(sample);
        }

        self.state_history
            .back()
            .map(&wave)
            .unwrap_or_else(|| wave(&initial))
    }

    fn distributed_rc_response(
        &self,
        kernel: &DistributedRcKernel,
        time: Value,
    ) -> TlineTransientResponse {
        let Some(last) = self.state_history.back().copied() else {
            return TlineTransientResponse::uncoupled(self.dc_series_conductance(), 0.0, 0.0);
        };
        if !(time.is_finite() && time > last.time) {
            // The operating-point companion is the only well-defined RC
            // response at the initial history point.  Transient candidates
            // normally satisfy time > last.time; this guard keeps startup and
            // repeated residual probes nonsingular without fabricating delay.
            return TlineTransientResponse::uncoupled(self.dc_series_conductance(), 0.0, 0.0);
        }

        let initial = self.initial_state();
        let time_list = self
            .state_history
            .iter()
            .map(|sample| sample.time)
            .collect::<Vec<_>>();
        let coeffs = distributed_rc_coefficients(
            kernel.c_by_r,
            kernel.rc_len_squared,
            time,
            &time_list,
            DISTRIBUTED_RLC_CHOP_RELTOL,
        );

        let mut input1 = 0.0;
        let mut input2 = 0.0;

        // h1dash convolution with v1 and v2.  Xyce's RC equations have no
        // characteristic admittance multiplier on this term.
        let mut dummy1 = 0.0;
        let mut dummy2 = 0.0;
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h1dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v1 - initial.v1);
                dummy2 += coeff * (sample.v2 - initial.v2);
            }
        }
        dummy1 -= initial.v1 * coeffs.h1dash_first;
        dummy2 -= initial.v2 * coeffs.h1dash_first;
        input1 -= dummy1;
        input2 -= dummy2;

        // h2 convolution with i2 and i1.
        dummy1 = 0.0;
        dummy2 = 0.0;
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h2.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.i2 - initial.i2);
                dummy2 += coeff * (sample.i1 - initial.i1);
            }
        }
        dummy1 += initial.i2;
        dummy2 += initial.i1;
        dummy1 -= initial.i2 * coeffs.h2_first;
        dummy2 -= initial.i1 * coeffs.h2_first;
        input1 += dummy1;
        input2 += dummy2;

        // h3dash convolution with v2 and v1.
        dummy1 = 0.0;
        dummy2 = 0.0;
        for (idx, sample) in self.state_history.iter().enumerate().skip(1) {
            let coeff = coeffs.h3dash.get(idx).copied().unwrap_or(0.0);
            if coeff != 0.0 {
                dummy1 += coeff * (sample.v2 - initial.v2);
                dummy2 += coeff * (sample.v1 - initial.v1);
            }
        }
        dummy1 -= initial.v2 * coeffs.h3dash_first;
        dummy2 -= initial.v1 * coeffs.h3dash_first;
        input1 += dummy1;
        input2 += dummy2;

        TlineTransientResponse::ltra_rc(
            coeffs.h1dash_first,
            -coeffs.h3dash_first,
            -coeffs.h2_first,
            input1,
            input2,
        )
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
        if let Some(kernel) = &self.distributed_rc {
            if let Some((cached_time, response)) = self.distributed_rlc_cache.get()
                && (cached_time - time).abs() < 1e-18
            {
                return response;
            }
            let response = self.distributed_rc_response(kernel, time);
            self.distributed_rlc_cache.set(Some((time, response)));
            return response;
        }
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
        let i_eq_port1 = self.attenuation * g * self.lossless_delayed_wave(time, false);
        let i_eq_port2 = self.attenuation * g * self.lossless_delayed_wave(time, true);
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
        self.launched_forward = raw_forward;
        self.launched_backward = raw_backward;

        // Distributed LTRA uses the absolute terminal-state history below;
        // its convolution is not horizon-limited. Ordinary lossless lines use
        // these bounded traveling-wave delay buffers instead.
        if self.distributed_rlc.is_none() && self.distributed_rc.is_none() {
            self.history_forward.push(time, self.launched_forward);
            self.history_backward.push(time, self.launched_backward);
        }
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
        if self.distributed_rlc.is_none() && self.distributed_rc.is_none() {
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
        self.launched_forward
    }

    #[inline]
    pub fn launched_backward_wave(&self) -> Value {
        self.launched_backward
    }

    /// Reset for new simulation
    pub fn reset(&mut self) {
        self.history_forward.clear();
        self.history_backward.clear();
        self.launched_forward = 0.0;
        self.launched_backward = 0.0;
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

    #[test]
    fn distributed_rlc_kernel_requires_exactly_zero_shunt_conductance() {
        let mut line = TransmissionLine::new("TLTRA".to_string(), 1, 0, 2, 0, 1.0, 1.0);

        line.set_distributed_rlgc(1.0, 1.0, 1.0e-30, 1.0, 1.0);
        assert!(!line.has_distributed_rlgc());

        line.set_distributed_rlgc(1.0, 1.0, -1.0e-30, 1.0, 1.0);
        assert!(!line.has_distributed_rlgc());

        line.set_distributed_rlgc(1.0, 1.0, 0.0, 1.0, 1.0);
        assert!(line.has_distributed_rlgc());
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
    fn lossless_checkpoint_round_trip_preserves_delayed_wave_history() {
        let mut original = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 10.0);
        for sample in [
            (0.0, 0.0, 0.0, 0.0, 0.0),
            (4.0, 2.0, 0.03, -1.0, 0.02),
            (8.0, -0.5, -0.01, 1.5, -0.02),
            (12.0, 3.0, 0.04, -2.0, 0.01),
            (16.0, -1.0, 0.02, 0.5, -0.03),
            (20.0, 1.25, -0.02, 2.0, 0.01),
        ] {
            original.update_history(sample.0, sample.1, sample.2, sample.3, sample.4);
        }
        let checkpoint = original
            .checkpoint_state()
            .expect("ordinary lossless line is checkpointable");
        let expected_snapshot = checkpoint.clone();
        assert!(
            checkpoint.state_history[0][0] > 0.0,
            "the fixture must age out a predecessor so the oldest retained Hermite slope is not reconstructible"
        );
        let expected_response = original.transient_port_response(24.0);
        let expected_buffer_response = original.delayed_forward_at(14.5);

        let mut restored = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 10.0);
        restored
            .restore_checkpoint_state(&checkpoint)
            .expect("validated lossless history restores");

        assert_eq!(
            restored.checkpoint_state().unwrap(),
            expected_snapshot,
            "restored physical history must be bit-exact"
        );
        let actual_response = restored.transient_port_response(24.0);
        assert_eq!(
            actual_response.i_eq_port1().to_bits(),
            expected_response.i_eq_port1().to_bits()
        );
        assert_eq!(
            actual_response.i_eq_port2().to_bits(),
            expected_response.i_eq_port2().to_bits()
        );
        assert_eq!(
            restored.delayed_forward_at(14.5).to_bits(),
            expected_buffer_response.to_bits(),
            "the oldest retained Hermite slope must survive the round trip"
        );
    }

    #[test]
    fn transmission_line_checkpoint_rejects_incomplete_or_wrong_runtime_state() {
        let mut lossless = lossless_line(&[(0.0, 0.0), (1.0, 1.0)]);
        let mut checkpoint = lossless.checkpoint_state().unwrap();
        checkpoint.state_history[1][0] = checkpoint.state_history[0][0];
        assert!(lossless.restore_checkpoint_state(&checkpoint).is_err());

        let distributed = distributed_line(&[(0.0, 0.0), (1.0, 1.0)]);
        assert!(distributed.checkpoint_state().is_err());

        let valid = lossless.checkpoint_state().unwrap();
        lossless.name = "T2".to_string();
        assert!(lossless.restore_checkpoint_state(&valid).is_err());
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
    fn ltra_history_compaction_removes_only_a_straight_middle_point() {
        let mut line = TransmissionLine::new("TLTRA".to_string(), 1, 0, 2, 0, 2.0, 10.0);
        line.set_distributed_rlgc(1.0, 1.0, 0.0, 1.0, 1.0);
        line.set_ltra_history_compaction(true, 1.0e-3, 1.0e-12);
        line.update_history(0.0, 0.0, 0.0, 2.0, 1.0);
        line.update_history(1.0, 1.0, 0.5, 4.0, 2.0);
        line.update_history(2.0, 2.0, 1.0, 6.0, 3.0);

        assert!(line.compact_ltra_history_if_straight());
        assert_eq!(line.state_history.len(), 2);
        assert_eq!(line.state_history[0].time, 0.0);
        assert_eq!(line.state_history[1].time, 2.0);
    }

    #[test]
    fn ltra_history_compaction_requires_all_four_terminal_histories() {
        let mut line = TransmissionLine::new("TLTRA".to_string(), 1, 0, 2, 0, 2.0, 10.0);
        line.set_distributed_rlgc(1.0, 1.0, 0.0, 1.0, 1.0);
        line.set_ltra_history_compaction(true, 0.0, 1.0e-12);
        line.update_history(0.0, 0.0, 0.0, 0.0, 1.0);
        line.update_history(1.0, 1.0, 0.5, 1.0, 2.0);
        line.update_history(2.0, 2.0, 1.0, 2.0, 5.0);

        assert!(!line.compact_ltra_history_if_straight());
        assert_eq!(line.state_history.len(), 3);
    }

    #[test]
    fn ltra_straight_line_boundary_is_strict() {
        assert!(!TransmissionLine::ltra_straight_line_check(
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 0.0),
            0.0,
            1.0,
        ));
        assert!(TransmissionLine::ltra_straight_line_check(
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 0.0),
            0.0,
            1.0 + Value::EPSILON * 2.0,
        ));
    }

    #[test]
    fn ltra_compaction_rejects_dense_history_pointwise_drift() {
        assert!(!TransmissionLine::ltra_straight_line_check(
            (0.0, 0.0),
            (1.0e-12, 1.0),
            (2.0e-12, 0.0),
            1.0e-3,
            1.0e-12,
        ));
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
    fn ltra_nosteplimit_and_truncdontcut_remove_steady_state_caps() {
        let mut line = distributed_line(&[(0.0, 0.0), (1.0, 1.0)]);
        line.set_ltra_timestep_policy(false, true);

        assert_eq!(
            line.ltra_candidate_truncation_limit(2.0, 2.0, 0.0, 0.0, 0.0),
            None
        );
    }

    #[test]
    fn ltra_nosteplimit_still_limits_a_derivative_change_to_delay() {
        let mut line = distributed_line(&[(0.0, 0.0), (1.0, 0.0)]);
        line.set_ltra_timestep_policy(false, true);
        line.set_ltra_breakpoint_tolerances(0.0, 0.5);

        assert_eq!(
            line.ltra_candidate_truncation_limit(2.0, 2.0, 0.0, 0.0, 0.0),
            Some(line.delay())
        );
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
        line.set_ltra_timestep_policy(false, false);
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

    #[test]
    fn xyce_lossless_tra_uses_linear_interpolation_across_a_derivative_corner() {
        let samples = [(0.0, 1.0), (1.0e-9, 0.0), (2.0e-9, 0.0)];
        let make_line = || {
            let mut line = TransmissionLine::new("TLOSS".to_string(), 1, 0, 2, 0, 1.0, 2.0e-9);
            for &(time, v1) in &samples {
                line.update_history(time, v1, 0.0, 0.0, 0.0);
            }
            line
        };

        let quadratic = make_line();
        assert_close(
            quadratic.transient_port_response(3.5e-9).i_eq_port2(),
            -0.125,
        );

        let mut xyce = make_line();
        xyce.set_xyce_tra_interpolation();
        assert_close(xyce.transient_port_response(3.5e-9).i_eq_port2(), 0.0);
    }
}

#[cfg(test)]
mod ltra_oracle_replay {
    use super::*;

    /// Replay ngspice's committed LTRA history for ltra1_1_line's o1 line and
    /// compare the delayed-history rhs terms point by point.
    ///
    /// The fixtures were extracted with gdb from the vendored ngspice debug
    /// build running tests/transmission/ltra1_1_line.cir: committed history
    /// samples at ltraacct.c:84 (index, fractional time, v1, v2, i1, i2) and
    /// per-load branch rhs at ltraload.c:853 (fractional time and CKTdelta,
    /// LTRAinput1, LTRAinput2; last record per time is the accepted iterate).
    /// Driving the kernel with the oracle's own history isolates the
    /// convolution coefficients and delayed interpolation from solver grid
    /// differences. Unlike TXL/CPL, LTRA's clock is fully fractional.
    #[test]
    fn replay_oracle_ltra1_o1() {
        let hv_text = include_str!("testdata/ltra1_o1_hv.dat");
        let in_text = include_str!("testdata/ltra1_o1_in.dat");

        struct Hv {
            t: Value,
            v1: Value,
            v2: Value,
            i1: Value,
            i2: Value,
        }
        let hv: Vec<Hv> = hv_text
            .lines()
            .map(|line| {
                let f: Vec<&str> = line.split_whitespace().collect();
                Hv {
                    t: f[1].parse().unwrap(),
                    v1: f[2].parse().unwrap(),
                    v2: f[3].parse().unwrap(),
                    i1: f[4].parse().unwrap(),
                    i2: f[5].parse().unwrap(),
                }
            })
            .collect();
        let inputs: std::collections::HashMap<u64, (Value, Value)> = in_text
            .lines()
            .map(|line| {
                let f: Vec<&str> = line.split_whitespace().collect();
                let t: Value = f[0].parse().unwrap();
                (t.to_bits(), (f[2].parse().unwrap(), f[3].parse().unwrap()))
            })
            .collect();

        // .model lline ltra r=12.45 g=0 l=8.972e-9 c=0.468e-12 len=16
        let (r, l, c, len): (Value, Value, Value, Value) = (12.45, 8.972e-9, 0.468e-12, 16.0);
        let z0 = (l / c).sqrt();
        let td = (l * c).sqrt() * len;
        let mut line = TransmissionLine::new("o1".to_string(), 1, 0, 2, 0, z0, td);
        line.set_distributed_rlgc_with_compaction(
            DistributedRlgc {
                r,
                l,
                g: 0.0,
                c,
                len,
            },
            1.0e-3,
            1.0e-14,
        );

        let mut compared = 0usize;
        let mut worst: (Value, Value) = (0.0, 0.0);
        let mut first_bad: Option<Value> = None;
        for sample in &hv {
            if let Some(&(in1_oracle, in2_oracle)) = inputs.get(&sample.t.to_bits()) {
                let response = line.transient_port_response(sample.t);
                let scale = in1_oracle.abs().max(in2_oracle.abs()).max(1.0e-9);
                let err1 = (response.i_eq_port1() - in1_oracle).abs() / scale;
                let err2 = (response.i_eq_port2() - in2_oracle).abs() / scale;
                let err = err1.max(err2);
                compared += 1;
                if err > worst.0 {
                    worst = (err, sample.t);
                }
                if err > 1.0e-6 && first_bad.is_none() {
                    first_bad = Some(sample.t);
                    println!(
                        "first divergence at t={:.6e}: ours=({:.12e},{:.12e}) oracle=({:.12e},{:.12e})",
                        sample.t,
                        response.i_eq_port1(),
                        response.i_eq_port2(),
                        in1_oracle,
                        in2_oracle
                    );
                }
            }
            line.update_history(sample.t, sample.v1, sample.i1, sample.v2, sample.i2);
        }
        assert!(
            compared > 450,
            "expected to replay the full accepted sequence, compared {compared}"
        );
        assert!(
            first_bad.is_none() && worst.0 < 1.0e-6,
            "LTRA rhs fidelity regressed: worst rel err {:.3e} at t={:.6e}, first>1e-6 at {:?}",
            worst.0,
            worst.1,
            first_bad
        );
    }
}
