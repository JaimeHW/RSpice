//! Native device and behavioral F/Q sampling over the complete periodic MNA state.

use super::*;
use crate::device::MatrixStamper;
use crate::device::semiconductor::BJT_PERIODIC_BIAS_DIM;
use std::collections::BTreeMap;

/// One native BJT's junction bias in one time sample.
type BjtSampleBias = [Value; BJT_PERIODIC_BIAS_DIM];

/// The junction bias each native BJT was evaluated at in each time sample of
/// a Newton iterate, sample-major. Each sample's junctions are limited
/// against the bias the same sample took on the iterate the step leaves, as
/// each of Xyce's HB time points keeps its own device state and limits its
/// own junctions (N_LOA_HBLoader.C:760-830, HBINT VOLTLIM on by default at
/// N_ANP_HB.C:139). A trial is a function of its coordinates and its
/// reference, so restoring both restores its residual exactly.
#[derive(Debug, Clone)]
pub(crate) struct PeriodicJunctionBias {
    /// The bias each sample is limited against.
    reference: Vec<BjtSampleBias>,
    /// The bias each sample was evaluated at, limited against `reference`.
    evaluated: Vec<BjtSampleBias>,
    /// Whether limiting held any sample off its own bias.
    limited: bool,
    /// Whether `evaluated` is each device's opening load, assigned outright
    /// rather than limited against `reference`.
    opening: bool,
}

impl PeriodicJunctionBias {
    /// A trial step from this iterate: limited against its evaluated bias.
    pub(super) fn trial(&self) -> Self {
        Self {
            reference: self.evaluated.clone(),
            evaluated: self.evaluated.clone(),
            limited: false,
            opening: false,
        }
    }

    /// Whether the residual was evaluated with some junction held off the
    /// iterate's own bias, so that it is not the circuit's.
    pub(crate) fn limited(&self) -> bool {
        self.limited
    }
}

/// How one time sample evaluates the native BJTs' junctions.
enum JunctionBias<'a> {
    /// At the sample's own bias.
    Unlimited,
    /// Limited against `reference`, recording each bias in `evaluated`.
    Limit {
        reference: &'a [BjtSampleBias],
        evaluated: &'a mut [BjtSampleBias],
        limited: &'a mut bool,
    },
    /// At the bias the limiting pass recorded for this sample.
    Recorded(&'a [BjtSampleBias]),
}

pub(super) struct NativeStamp {
    residual: Vec<Value>,
    residual_scale: Vec<Value>,
    contributions: Vec<(usize, Value)>,
    jacobian: Vec<(usize, usize, Value)>,
    /// A stamp addressed a row or column outside the system: a defect of the
    /// device's wiring, whatever state it was sampled at.
    misaddressed: bool,
    /// A stamped value, or the row it accumulated into, left the finite
    /// range at the sampled state.
    nonfinite: bool,
}

impl NativeStamp {
    fn new(unknowns: usize) -> Self {
        Self {
            residual: vec![0.0; unknowns],
            residual_scale: vec![0.0; unknowns],
            contributions: Vec::new(),
            jacobian: Vec::new(),
            misaddressed: false,
            nonfinite: false,
        }
    }
    fn clear(&mut self) {
        self.residual.fill(0.0);
        self.residual_scale.fill(0.0);
        self.contributions.clear();
        self.jacobian.clear();
        self.misaddressed = false;
        self.nonfinite = false;
    }

    /// The verdict on `device`'s F and Q stamps at one time sample.
    ///
    /// A stamp outside the system is a defect of the circuit and ends the
    /// solve. A non-finite value is a state the device cannot evaluate: the
    /// solver proposed it, so it is a rejected Newton trial that damping and
    /// continuation step around, as a transient step whose device evaluation
    /// overflows is a rejected step, and only a solve with no other state to
    /// try reports it.
    fn verdict(f: &Self, q: &Self, device: impl FnOnce() -> String) -> Result<(), HbError> {
        if f.misaddressed || q.misaddressed {
            Err(HbError::InvalidCircuit(format!(
                "{} stamped outside the periodic system",
                device()
            )))
        } else if f.nonfinite || q.nonfinite {
            Err(HbError::NonFiniteSample(format!(
                "{} produced non-finite F/Q entries",
                device()
            )))
        } else {
            Ok(())
        }
    }
}

impl MatrixStamper for NativeStamp {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row == 0 || col == 0 || value == 0.0 {
            return;
        }
        if row > self.residual.len() || col > self.residual.len() {
            self.misaddressed = true;
        } else if !value.is_finite() {
            self.nonfinite = true;
        } else {
            self.jacobian.push((row - 1, col - 1, value));
        }
    }
    fn stamp_rhs(&mut self, row: usize, value: Value) {
        if row == 0 {
            return;
        }
        if let Some(entry) = self.residual.get_mut(row - 1) {
            *entry += value;
            self.residual_scale[row - 1] += value.abs();
            self.contributions.push((row - 1, value));
            self.nonfinite |= !entry.is_finite() || !self.residual_scale[row - 1].is_finite();
        } else {
            self.misaddressed = true;
        }
    }
}

/// Retain the stable per-device term ordering without a dense row-by-device
/// tensor or a hash lookup at every collocation sample.
fn record_native_terms(
    waveforms: &mut Vec<(usize, Vec<Value>)>,
    terms: &[(usize, Value)],
    time: usize,
    count: usize,
) -> Result<(), HbError> {
    if time == 0 {
        waveforms.try_reserve_exact(terms.len()).map_err(|error| {
            HbError::InvalidCircuit(format!("native device term allocation failed: {error}"))
        })?;
        for &(row, _) in terms {
            let mut values = Vec::new();
            values.try_reserve_exact(count).map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "native device waveform allocation failed: {error}"
                ))
            })?;
            values.resize(count, 0.0);
            waveforms.push((row, values));
        }
    }
    if waveforms.len() != terms.len() {
        return Err(HbError::InvalidCircuit(
            "native device physical term count changes over the orbit".into(),
        ));
    }
    for ((row, waveform), &(actual_row, value)) in waveforms.iter_mut().zip(terms) {
        if *row != actual_row {
            return Err(HbError::InvalidCircuit(
                "native device physical term changes its row over the orbit".into(),
            ));
        }
        waveform[time] = value;
    }
    Ok(())
}

impl HbSolver {
    pub(super) fn ac_response_branch_start(&self) -> usize {
        self.capacitor_rate_start() + self.periodic_capacitors.len()
    }

    pub(super) fn mos_rate_branch_start(&self) -> usize {
        self.ac_response_branch_start()
            + self.native_bsim3.iter().filter(|d| d.uses_ac_nqs()).count() * 3
            + self.native_bsim4.iter().filter(|d| d.uses_ac_nqs()).count() * 3
    }

    pub(super) fn is_ac_response_current_row(&self, row: usize) -> bool {
        let start = self.num_nodes + self.ac_response_branch_start();
        row >= start
            && row < self.num_nodes + self.mos_rate_branch_start()
            && (row - start).is_multiple_of(3)
    }

    pub(super) fn has_native_periodic_devices(&self) -> bool {
        !self.native_bjts.is_empty()
            || !self.native_bsim3.is_empty()
            || !self.native_bsim4.is_empty()
            || !self.native_mos.is_empty()
            || !self.behavioral_sources.is_empty()
            || !self.periodic_capacitors.is_empty()
            || self.has_veriloga_devices()
    }

    fn has_veriloga_devices(&self) -> bool {
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            return true;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if !self.generated_nonlinear_devices.is_empty() {
            return true;
        }
        false
    }

    pub(crate) fn set_periodic_behavioral_sources(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        autonomous: bool,
        max_values: usize,
        retained: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), HbError> {
        self.validate_configuration()?;
        let period = self.config.fundamental_freq.recip();
        let certify = |name: &str, periodic: bool, cycles: Value, interval: Option<Value>| {
            if !periodic {
                return Err(HbError::InvalidCircuit(format!(
                    "behavioral source '{name}' is not certified {} over the configured period {period:e} s",
                    if autonomous {
                        "constant in explicit time"
                    } else {
                        "periodic"
                    },
                )));
            }
            if !autonomous {
                if !cycles.is_finite() || cycles.round() > self.num_harmonics as Value {
                    return Err(HbError::InvalidCircuit(format!(
                        "behavioral source '{name}' requires at least {:.0} harmonics, beyond the configured {}",
                        cycles.ceil(),
                        self.num_harmonics,
                    )));
                }
                let points = (2.0 * cycles.round() + 1.0)
                    .max(interval.map_or(0.0, |interval| (2.0 * period / interval).ceil()));
                if !points.is_finite()
                    || points > self.fft.size() as Value
                    || interval.is_some_and(|value| !value.is_finite() || value <= 0.0)
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "behavioral source '{name}' requires at least {points:.0} collocation points; increase HB oversampling or time points from {}",
                        self.fft.size(),
                    )));
                }
            }
            Ok(())
        };
        self.validate_behavioral_bindings(sources)?;
        if retained {
            Self::validate_behavioral_response_frequency(sources)?;
        }
        for source in &sources.current_sources {
            source
                .validate_periodic_integral_rates()
                .map_err(HbError::InvalidCircuit)?;
            certify(
                &source.name,
                source.has_periodic_shooting_equation(period, autonomous),
                source.max_authored_tone_cycles(period),
                source.minimum_pss_interval(false),
            )?;
        }
        for source in &sources.voltage_sources {
            source
                .validate_periodic_integral_rates()
                .map_err(HbError::InvalidCircuit)?;
            certify(
                &source.name,
                source.has_periodic_shooting_equation(period, autonomous),
                source.max_authored_tone_cycles(period),
                source.minimum_pss_interval(false),
            )?;
        }
        let prescribed = self.prepare_prescribed_integrals(sources, max_values, retained, abort)?;
        self.register_integral_coordinates(sources)?;
        self.prescribed_integrals = prescribed;
        self.periodic_integral_budget = (!retained).then_some(max_values);
        self.quasi_prescribed_integrals = None;
        self.behavioral_sources = sources.clone();
        self.behavioral_phase_dimensions = 0;
        Ok(())
    }

    fn validate_behavioral_bindings(
        &self,
        sources: &crate::device::behavioral::BehavioralSources,
    ) -> Result<(), HbError> {
        let unknowns = self.num_nodes + self.exact_mna_branches().len();
        let valid = |pos: usize, neg: usize, supported: bool, indices: Vec<usize>| {
            supported
                && pos <= self.num_nodes
                && neg <= self.num_nodes
                && indices.into_iter().all(|index| index < unknowns)
        };
        for source in &sources.current_sources {
            if !valid(
                source.node_pos,
                source.node_neg,
                source.has_periodic_carrier_frequency_context(),
                source.bound_solution_indices().collect(),
            ) {
                return Err(HbError::InvalidCircuit(format!(
                    "behavioral source '{}' has unsupported periodic state or MNA bindings",
                    source.name
                )));
            }
        }
        for source in &sources.voltage_sources {
            let branch = source
                .branch_ordinal
                .checked_sub(1)
                .and_then(|index| self.exact_mna_branches().get(index));
            if !valid(
                source.node_pos,
                source.node_neg,
                source.has_periodic_carrier_frequency_context(),
                source.bound_solution_indices().collect(),
            ) || !matches!(branch, Some(ExactMnaBranch::ConstitutivePort { node_pos, node_neg, .. })
                    if *node_pos == source.node_pos && *node_neg == source.node_neg)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "behavioral voltage source '{}' has unsupported periodic state or branch bindings",
                    source.name
                )));
            }
        }
        Ok(())
    }

    pub(super) fn validate_behavioral_response_frequency(
        sources: &crate::device::behavioral::BehavioralSources,
    ) -> Result<(), HbError> {
        // A valid carrier context is not a translated-frequency response
        // operator. Keep the original dependency after independent-phase lift.
        for (name, dependent) in sources
            .current_sources
            .iter()
            .map(|source| (&source.name, source.is_frequency_dependent()))
            .chain(
                sources
                    .voltage_sources
                    .iter()
                    .map(|source| (&source.name, source.is_frequency_dependent())),
            )
        {
            if dependent {
                return Err(HbError::InvalidCircuit(format!(
                    "behavioral source '{name}' requires frequency-dependent equations in the periodic response solver"
                )));
            }
        }
        Ok(())
    }

    pub(crate) fn set_quasi_periodic_behavioral_sources(
        &mut self,
        sources: &crate::device::behavioral::BehavioralSources,
        grid: &crate::analysis::quasi_periodic::QuasiPeriodicGrid,
    ) -> Result<(), HbError> {
        // Validate physical references before appending any independent inputs.
        self.validate_behavioral_bindings(sources)?;
        let unknowns = self
            .num_nodes
            .checked_add(self.physical_branch_count())
            .and_then(|count| count.checked_add(sources.integral_count()))
            .and_then(|count| count.checked_add(self.periodic_capacitors.len()))
            .and_then(|count| {
                count.checked_add(
                    self.periodic_capacitors
                        .iter()
                        .map(|cap| cap.expression.program.sdt_count)
                        .sum(),
                )
            })
            .ok_or_else(|| {
                HbError::InvalidCircuit("QPSS integral MNA dimension exceeds this platform".into())
            })?;
        let mut lifted = sources.clone();
        for source in &mut lifted.current_sources {
            source
                .lift_quasi_periodic(grid, unknowns)
                .map_err(|error| {
                    HbError::InvalidCircuit(format!("behavioral source '{}': {error}", source.name))
                })?;
        }
        for source in &mut lifted.voltage_sources {
            source
                .lift_quasi_periodic(grid, unknowns)
                .map_err(|error| {
                    HbError::InvalidCircuit(format!("behavioral source '{}': {error}", source.name))
                })?;
        }
        let mut capacitors = self.periodic_capacitors.clone();
        for capacitor in &mut capacitors {
            capacitor
                .expression
                .lift_quasi_periodic(grid)
                .map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "capacitor '{}': {error}",
                        capacitor.expression.name
                    ))
                })?;
        }
        self.register_integral_coordinates(sources)?;
        self.periodic_capacitors = capacitors;
        self.behavioral_sources = lifted;
        self.prescribed_integrals.clear();
        self.periodic_integral_budget = None;
        self.quasi_prescribed_integrals = None;
        self.behavioral_phase_dimensions = grid.dimensions().len();
        Ok(())
    }

    pub(super) fn electrical_node(&self, node: usize) -> bool {
        self.non_electrical_nodes.binary_search(&node).is_err()
    }

    pub(crate) fn add_native_bjt(&mut self, mut bjt: crate::device::Bjt) -> Result<(), HbError> {
        bjt.prepare_periodic_mna(self.num_nodes)
            .map_err(HbError::InvalidCircuit)?;
        if bjt
            .mna_coupling_nodes()
            .iter()
            .any(|&node| node > self.num_nodes)
            || bjt
                .mna_rbi_branch_matrix_node(self.num_nodes)
                .is_some_and(|node| {
                    !self.exact_mna_branches().iter().any(|branch| {
                        matches!(branch, ExactMnaBranch::ConstitutivePort { branch_ordinal, .. }
                        if *branch_ordinal == node - self.num_nodes)
                    })
                })
        {
            return Err(HbError::InvalidCircuit(format!(
                "BJT '{}' has unregistered periodic MNA coordinates",
                bjt.name
            )));
        }
        for node in [bjt.node_rth, bjt.node_xf1, bjt.node_xf2] {
            if node != 0
                && let Err(index) = self.non_electrical_nodes.binary_search(&(node - 1))
            {
                self.non_electrical_nodes.insert(index, node - 1);
            }
        }
        self.native_bjts.push(bjt);
        Ok(())
    }

    pub(crate) fn add_native_bsim3(
        &mut self,
        device: crate::device::mosfet::bsim3v3::Bsim3v3Device,
    ) -> Result<(), HbError> {
        if device
            .periodic_coupling_nodes()
            .iter()
            .any(|&node| node > self.num_nodes)
            || (device.uses_trnqs() && device.node_charge_deficit == 0)
        {
            return Err(HbError::InvalidCircuit(format!(
                "BSIM3 '{}' has unregistered periodic MNA coordinates",
                device.name,
            )));
        }
        let node = device.node_charge_deficit;
        if node != 0
            && let Err(index) = self.non_electrical_nodes.binary_search(&(node - 1))
        {
            self.non_electrical_nodes.insert(index, node - 1);
        }
        for name in device.ac_nqs_response_names() {
            let branch_ordinal = self.exact_mna_branches().len() + 1;
            self.try_push_periodic_mna_branch(
                ExactMnaBranch::AuxiliaryState {
                    branch_ordinal,
                    canonical: false,
                },
                &name,
            )?;
        }
        self.native_bsim3.push(device);
        Ok(())
    }

    pub(crate) fn add_native_bsim4(
        &mut self,
        device: crate::device::mosfet::bsim4v8::Bsim4v8Device,
    ) -> Result<(), HbError> {
        if device
            .periodic_coupling_nodes()
            .iter()
            .any(|&node| node > self.num_nodes)
            || (device.uses_trnqs() && device.node_charge_deficit == 0)
        {
            return Err(HbError::InvalidCircuit(format!(
                "BSIM4 '{}' has unregistered periodic MNA coordinates",
                device.name,
            )));
        }
        let node = device.node_charge_deficit;
        if node != 0
            && let Err(index) = self.non_electrical_nodes.binary_search(&(node - 1))
        {
            self.non_electrical_nodes.insert(index, node - 1);
        }
        for name in device.ac_nqs_response_names() {
            let branch_ordinal = self.exact_mna_branches().len() + 1;
            self.try_push_periodic_mna_branch(
                ExactMnaBranch::AuxiliaryState {
                    branch_ordinal,
                    canonical: false,
                },
                &name,
            )?;
        }
        self.native_bsim4.push(device);
        Ok(())
    }

    pub(crate) fn add_native_mos(&mut self, device: crate::device::Mosfet) -> Result<(), HbError> {
        if device
            .periodic_coupling_nodes()
            .iter()
            .any(|&node| node > self.num_nodes)
        {
            return Err(HbError::InvalidCircuit(format!(
                "MOSFET '{}' has unregistered periodic nodes",
                device.name
            )));
        }
        for name in device.periodic_rate_names().into_iter().flatten() {
            let branch_ordinal = self.exact_mna_branches().len() + 1;
            self.try_push_periodic_mna_branch(
                ExactMnaBranch::AuxiliaryState {
                    branch_ordinal,
                    canonical: false,
                },
                &name,
            )?;
        }
        self.native_mos.push(device);
        Ok(())
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
    )]
    fn sample_native_devices(
        &mut self,
        solution: &[Value],
        time: Value,
        behavioral_inputs: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
        small_signal: bool,
        junctions: JunctionBias<'_>,
    ) -> Result<(), HbError> {
        self.sample_native_devices_selected(
            solution,
            time,
            behavioral_inputs,
            f,
            q,
            small_signal,
            None,
            junctions,
        )
    }

    /// Each native BJT's own bias in every time sample of `state`, or with
    /// `opening` the bias its first Newton load assigns there. None where
    /// the devices do not limit: without native BJTs, or on an oversampled
    /// grid, where a sample's limited bias is not held to its own KCL.
    pub(super) fn own_junction_bias(
        &mut self,
        state: &HbSolverState,
        opening: bool,
    ) -> Result<Option<PeriodicJunctionBias>, HbError> {
        if self.native_bjts.is_empty() || !self.every_sample_is_an_equation() {
            return Ok(None);
        }
        let waves = self.native_state_waveforms(state)?;
        let mut solution = vec![0.0; waves.len()];
        let mut evaluated = Vec::with_capacity(self.fft.size() * self.native_bjts.len());
        let mut limited = false;
        for time in 0..self.fft.size() {
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            evaluated.extend(self.native_bjts.iter().map(|bjt| {
                let own = bjt.periodic_bias(&solution);
                if !opening {
                    return own;
                }
                let bias = bjt.initial_periodic_bias(&solution);
                limited |= bias != own;
                bias
            }));
        }
        Ok(Some(PeriodicJunctionBias {
            reference: evaluated.clone(),
            evaluated,
            limited,
            opening,
        }))
    }

    /// The junction bias of time sample `time` within `bias`, when `bias`
    /// covers every native BJT in every sample.
    fn sample_junction_range(
        &self,
        bias: &PeriodicJunctionBias,
        time: usize,
    ) -> Option<std::ops::Range<usize>> {
        let devices = self.native_bjts.len();
        (bias.evaluated.len() == devices * self.fft.size()
            && bias.reference.len() == bias.evaluated.len())
        .then(|| time * devices..(time + 1) * devices)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
    )]
    fn sample_native_devices_selected(
        &mut self,
        solution: &[Value],
        time: Value,
        behavioral_inputs: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
        small_signal: bool,
        selected: Option<&[bool]>,
        mut junctions: JunctionBias<'_>,
    ) -> Result<(), HbError> {
        if solution.len() != self.num_nodes + self.exact_mna_branches().len() {
            return Err(HbError::InvalidCircuit(
                "native periodic sample does not match the complete MNA state".into(),
            ));
        }
        f.clear();
        q.clear();
        for (index, bjt) in self.native_bjts.iter_mut().enumerate() {
            if selected.is_some_and(|rows| {
                !bjt.mna_coupling_nodes()
                    .iter()
                    .any(|&node| node > 0 && rows[node - 1])
            }) {
                continue;
            }
            let bias = match &mut junctions {
                JunctionBias::Unlimited => None,
                JunctionBias::Limit {
                    reference,
                    evaluated,
                    limited,
                } => {
                    let bias = bjt.limit_periodic_bias(solution, &reference[index]);
                    evaluated[index] = bias;
                    let held = bias != bjt.periodic_bias(solution);
                    **limited |= held;
                    held.then_some(bias)
                }
                JunctionBias::Recorded(evaluated) => {
                    Some(evaluated[index]).filter(|&bias| bias != bjt.periodic_bias(solution))
                }
            };
            match bias {
                Some(bias) => bjt.stamp_periodic_fq_at_bias(solution, &bias, f, q),
                None => bjt.stamp_periodic_fq(solution, f, q),
            }
            NativeStamp::verdict(f, q, || format!("BJT '{}'", bjt.name))?;
        }
        let mut response_start = self.num_nodes + self.ac_response_branch_start();
        for device in &self.native_bsim3 {
            let auxiliary = device.uses_ac_nqs().then(|| {
                let nodes = [response_start + 1, response_start + 2, response_start + 3];
                response_start += 3;
                nodes
            });
            if selected.is_some_and(|rows| {
                !device
                    .periodic_coupling_nodes()
                    .iter()
                    .any(|&node| node > 0 && rows[node - 1])
                    && !auxiliary.is_some_and(|nodes| nodes.into_iter().any(|node| rows[node - 1]))
            }) {
                continue;
            }
            if small_signal && let Some(auxiliary) = auxiliary {
                device
                    .stamp_ac_nqs_response(solution, auxiliary, f, q)
                    .map_err(HbError::InvalidCircuit)?;
            } else {
                device
                    .stamp_periodic_fq(solution, f, q)
                    .map_err(HbError::InvalidCircuit)?;
                if let Some(auxiliary) = auxiliary {
                    // Response-only states remain exactly zero in the carrier.
                    for node in auxiliary {
                        f.stamp(node, node, 1.0);
                        f.stamp_rhs(node, -solution[node - 1]);
                    }
                }
            }
            NativeStamp::verdict(f, q, || format!("BSIM3 '{}'", device.name))?;
        }
        for device in &self.native_bsim4 {
            let auxiliary = device.uses_ac_nqs().then(|| {
                let nodes = [response_start + 1, response_start + 2, response_start + 3];
                response_start += 3;
                nodes
            });
            if selected.is_some_and(|rows| {
                !device
                    .periodic_coupling_nodes()
                    .iter()
                    .any(|&node| node > 0 && rows[node - 1])
                    && !auxiliary.is_some_and(|nodes| nodes.into_iter().any(|node| rows[node - 1]))
            }) {
                continue;
            }
            if small_signal && let Some(auxiliary) = auxiliary {
                device
                    .stamp_ac_nqs_response(solution, auxiliary, f, q)
                    .map_err(HbError::InvalidCircuit)?;
            } else {
                device
                    .stamp_periodic_fq(solution, f, q)
                    .map_err(HbError::InvalidCircuit)?;
                if let Some(auxiliary) = auxiliary {
                    // Response-only states remain exactly zero in the carrier.
                    for node in auxiliary {
                        f.stamp(node, node, 1.0);
                        f.stamp_rhs(node, -solution[node - 1]);
                    }
                }
            }
            NativeStamp::verdict(f, q, || format!("BSIM4 '{}'", device.name))?;
        }
        for device in &self.native_mos {
            let auxiliary = (!device.uses_legacy_bsim()).then(|| {
                let nodes = [response_start + 1, response_start + 2, response_start + 3];
                response_start += 3;
                nodes
            });
            if selected.is_some_and(|rows| {
                !device
                    .periodic_coupling_nodes()
                    .iter()
                    .any(|&node| node > 0 && rows[node - 1])
                    && !auxiliary.is_some_and(|nodes| nodes.into_iter().any(|node| rows[node - 1]))
            }) {
                continue;
            }
            device
                .stamp_periodic_physical_fq(solution, auxiliary, f, q)
                .map_err(HbError::InvalidCircuit)?;
            NativeStamp::verdict(f, q, || format!("MOSFET '{}'", device.name))?;
        }
        let integral_start = self.num_nodes + self.physical_branch_count();
        #[cfg(feature = "veriloga")]
        for device in &mut self.veriloga_nonlinear_devices {
            if selected.is_some_and(|rows| !device.rows().iter().any(|&row| rows[row])) {
                continue;
            }
            device.stamp_periodic_fq(solution, time, f, q)?;
            NativeStamp::verdict(f, q, || format!("Verilog-A device '{}'", device.name()))?;
        }
        #[cfg(feature = "veriloga-builtins-base")]
        for device in &mut self.generated_nonlinear_devices {
            if selected.is_some_and(|rows| !device.stamped_rows().iter().any(|&row| rows[row])) {
                continue;
            }
            device.stamp_periodic_fq(solution, f, q)?;
            NativeStamp::verdict(f, q, || {
                format!("generated Verilog-A device '{}'", device.name())
            })?;
        }
        let prescribed = if let Some(prescribed) = &self.quasi_prescribed_integrals {
            prescribed.sample(
                &behavioral_inputs[solution.len()..],
                &solution[integral_start..],
            )
        } else {
            self.prescribed_integrals
                .iter()
                .enumerate()
                .map(|(index, spectrum)| {
                    spectrum
                        .as_ref()
                        .filter(|spectrum| !small_signal || !spectrum.is_circuit_driven())
                        .map(|spectrum| {
                            (
                                spectrum.value(
                                    time * self.config.fundamental_freq,
                                    solution[integral_start + index],
                                ),
                                self.config.fundamental_freq,
                            )
                        })
                })
                .collect::<Vec<_>>()
        };
        let point = crate::device::behavioral::BehavioralFqPoint {
            inputs: behavioral_inputs,
            time,
            num_nodes: self.num_nodes,
            unknowns: solution.len(),
            integral_start,
            prescribed_integrals: &prescribed,
        };
        let rate_start = self.num_nodes + self.capacitor_rate_start();
        let behavioral_count = self.behavioral_sources.integral_count();
        let mut capacitor_integral_start = integral_start + behavioral_count;
        for (index, capacitor) in self.periodic_capacitors.iter_mut().enumerate() {
            let rate = rate_start + index;
            let start = capacitor_integral_start;
            capacitor_integral_start += capacitor.expression.program.sdt_count;
            if selected.is_some_and(|rows| {
                !rows[rate]
                    && !(start..capacitor_integral_start).any(|row| rows[row])
                    && !capacitor.branch.is_some_and(|branch| rows[branch])
                    && ![capacitor.pos, capacitor.neg]
                        .into_iter()
                        .any(|node| node > 0 && rows[node - 1])
            }) {
                continue;
            }
            capacitor
                .stamp(point, self.config.fundamental_freq, (start, rate), f, q)
                .map_err(HbError::InvalidCircuit)?;
            NativeStamp::verdict(f, q, || {
                format!("capacitor '{}'", capacitor.expression.name)
            })?;
        }
        let point = crate::device::behavioral::BehavioralFqPoint {
            prescribed_integrals: &prescribed[..prescribed.len().min(behavioral_count)],
            ..point
        };
        match selected {
            Some(_) => self
                .behavioral_sources
                .stamp_periodic_fq_selected(point, f, q, selected),
            None => self.behavioral_sources.stamp_periodic_fq(point, f, q),
        }
        .map_err(HbError::InvalidCircuit)?;
        NativeStamp::verdict(f, q, || "the behavioral sources".to_string())?;
        Ok(())
    }

    fn native_dc_sample(&mut self, state: &HbSolverState) -> Result<NativeStamp, HbError> {
        let solution: Vec<_> = state
            .x
            .iter()
            .chain(&state.mna_branch_currents)
            .map(|s| s[0].re)
            .collect();
        let mut f = NativeStamp::new(solution.len());
        let mut q = NativeStamp::new(solution.len());
        self.sample_native_devices(
            &solution,
            0.0,
            &solution,
            &mut f,
            &mut q,
            false,
            JunctionBias::Unlimited,
        )?;
        Ok(f)
    }

    /// The shared native F/Q implementation at physical time for HB, or at
    /// zero time with explicit independent-phase inputs for QPSS.
    pub(super) fn periodic_native_sample_selected(
        &mut self,
        solution: &[Value],
        time: Value,
        phases: &[Value],
        jacobian: bool,
        selected: Option<&[bool]>,
        small_signal: bool,
    ) -> Result<crate::analysis::quasi_periodic::solve::Sample, HbError> {
        if !time.is_finite()
            || phases.len() != self.behavioral_phase_dimensions
            || phases.iter().any(|v| !v.is_finite())
        {
            return Err(HbError::InvalidCircuit(
                "behavioral phase inputs do not match the registered quasiperiodic grid".into(),
            ));
        }
        let mut inputs = Vec::with_capacity(solution.len() + phases.len());
        inputs.extend_from_slice(solution);
        inputs.extend_from_slice(phases);
        let mut f = NativeStamp::new(solution.len());
        let mut q = NativeStamp::new(solution.len());
        self.sample_native_devices_selected(
            solution,
            time,
            &inputs,
            &mut f,
            &mut q,
            small_signal,
            selected,
            JunctionBias::Unlimited,
        )?;
        Ok(crate::analysis::quasi_periodic::solve::Sample {
            current: f.contributions,
            charge: q.contributions,
            conductance: if jacobian { f.jacobian } else { Vec::new() },
            capacitance: if jacobian { q.jacobian } else { Vec::new() },
        })
    }

    pub(super) fn add_native_dc_residual(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<(), HbError> {
        if !self.has_native_periodic_devices() {
            return Ok(());
        }
        let f = self.native_dc_sample(state)?;
        for ((residual, scale), (value, magnitude)) in state
            .residual
            .iter_mut()
            .chain(&mut state.mna_branch_residual)
            .zip(
                state
                    .residual_scale
                    .iter_mut()
                    .chain(&mut state.mna_branch_residual_scale),
            )
            .zip(f.residual.into_iter().zip(f.residual_scale))
        {
            residual[0].re += value;
            scale[0] += magnitude;
        }
        Ok(())
    }

    pub(super) fn add_native_dc_jacobian(
        &mut self,
        state: &HbSolverState,
        jacobian: &mut [Vec<Value>],
    ) -> Result<(), HbError> {
        if self.has_native_periodic_devices() {
            for (row, col, value) in self.native_dc_sample(state)?.jacobian {
                jacobian[row][col] -= value;
            }
        }
        Ok(())
    }

    pub(super) fn native_state_waveforms(
        &mut self,
        state: &HbSolverState,
    ) -> Result<Vec<Vec<Value>>, HbError> {
        let mut waves = self.periodic_state_waveforms(state, "native device F/Q evaluation")?;
        for spectrum in &state.mna_branch_currents {
            let waveform = self.fft.to_time_domain(spectrum);
            if waveform.len() != self.fft.size() {
                return Err(HbError::InvalidCircuit(
                    "native device branch-current waveform is invalid".into(),
                ));
            }
            if waveform.iter().any(|v| !v.is_finite()) {
                return Err(HbError::NonFiniteSample(
                    "native device branch-current waveform is non-finite".into(),
                ));
            }
            waves.push(waveform);
        }
        Ok(waves)
    }

    /// Visit native models at each unlimited physical bias of the retained
    /// periodic state, including branch-current and non-electrical coordinates.
    pub(crate) fn visit_native_noise_samples(
        &mut self,
        state: &HbSolverState,
        abort: &dyn AbortSignal,
        mut visit: impl FnMut(
            usize,
            usize,
            (
                &[crate::device::Bjt],
                &[crate::device::Bsim3v3Device],
                &[crate::device::Bsim4v8Device],
                &[crate::device::Mosfet],
            ),
            &[Value],
        ) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        if self.native_bjts.is_empty()
            && self.native_bsim3.is_empty()
            && self.native_bsim4.is_empty()
            && self.native_mos.is_empty()
        {
            return Ok(());
        }
        let waves = self.native_state_waveforms(state)?;
        let count = self.fft.size();
        let mut solution = vec![0.0; waves.len()];
        for time in 0..count {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            for bjt in &mut self.native_bjts {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                bjt.update_mna_static_probe(&solution);
            }
            for device in &mut self.native_bsim3 {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                device
                    .update_periodic_noise_probe(&solution)
                    .map_err(HbError::InvalidCircuit)?;
            }
            for device in &mut self.native_bsim4 {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                device
                    .update_periodic_noise_probe(&solution)
                    .map_err(HbError::InvalidCircuit)?;
            }
            for device in &mut self.native_mos {
                device.update_periodic_noise_probe(&solution);
            }
            visit(
                time,
                count,
                (
                    &self.native_bjts,
                    &self.native_bsim3,
                    &self.native_bsim4,
                    &self.native_mos,
                ),
                &solution,
            )?;
        }
        Ok(())
    }

    /// The carrier's fundamental, in hertz.
    pub(crate) fn fundamental_freq(&self) -> Value {
        self.config.fundamental_freq
    }

    /// Visit each collocation sample of the retained periodic state as one
    /// solution over the periodic unknowns: every node voltage, then every
    /// periodic MNA branch current, as the device samplers read them.
    pub(crate) fn visit_periodic_samples(
        &mut self,
        state: &HbSolverState,
        abort: &dyn AbortSignal,
        mut visit: impl FnMut(usize, usize, &[Value]) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        let waves = self.native_state_waveforms(state)?;
        let count = self.fft.size();
        let mut solution = vec![0.0; waves.len()];
        for time in 0..count {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            visit(time, count, &solution)?;
        }
        Ok(())
    }

    pub(super) fn add_native_periodic_residual(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<(), HbError> {
        if !self.has_native_periodic_devices() {
            return Ok(());
        }
        let waves = self.native_state_waveforms(state)?;
        let size = waves.len();
        let times = self.fft.size();
        let mut solution = vec![0.0; size];
        let mut f = NativeStamp::new(size);
        let mut q = NativeStamp::new(size);
        let mut f_time = Vec::new();
        let mut q_time = Vec::new();
        let integral_start = self.num_nodes + self.physical_branch_count();
        // The bias is the state's; it goes back whatever the samples return.
        // An opening load is evaluated as assigned.
        let mut junctions = state.junction_bias.take();
        if let Some(bias) = &mut junctions
            && !bias.opening
        {
            bias.limited = false;
        }
        let sampled = (0..times).try_for_each(|time| {
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            let sample_time = time as Value / times as Value / self.config.fundamental_freq;
            let mode = match junctions
                .as_mut()
                .and_then(|bias| Some((self.sample_junction_range(bias, time)?, bias)))
            {
                Some((range, bias)) if bias.opening => {
                    JunctionBias::Recorded(&bias.evaluated[range])
                }
                Some((range, bias)) => JunctionBias::Limit {
                    reference: &bias.reference[range.clone()],
                    evaluated: &mut bias.evaluated[range],
                    limited: &mut bias.limited,
                },
                None => JunctionBias::Unlimited,
            };
            self.sample_native_devices(
                &solution,
                sample_time,
                &solution,
                &mut f,
                &mut q,
                false,
                mode,
            )?;
            // Known primitive rows are already exact spectral constraints.
            // Transform only physical equations and unresolved integral rates.
            let sampled_row = |(row, _): &(usize, Value)| {
                row.checked_sub(integral_start)
                    .and_then(|index| self.prescribed_integrals.get(index))
                    .is_none_or(Option::is_none)
            };
            f.contributions.retain(sampled_row);
            q.contributions.retain(sampled_row);
            record_native_terms(&mut f_time, &f.contributions, time, times)?;
            record_native_terms(&mut q_time, &q.contributions, time, times)
        });
        state.junction_bias = junctions;
        sampled?;
        self.add_native_waveform_terms(state, f_time, false)?;
        self.add_native_waveform_terms(state, q_time, true)?;
        self.add_prescribed_integral_residual(state)
    }

    fn add_native_waveform_terms(
        &mut self,
        state: &mut HbSolverState,
        waveforms: Vec<(usize, Vec<Value>)>,
        charge: bool,
    ) -> Result<(), HbError> {
        let omega = std::f64::consts::TAU * self.config.fundamental_freq;
        // Transform the physical contributions before taking magnitudes. A
        // DC current must not inflate the tolerance of a small AC harmonic,
        // and opposing currents at that harmonic must retain both magnitudes.
        for (row, waveform) in waveforms {
            let spectrum = self.checked_periodic_spectrum(
                &waveform,
                self.num_harmonics,
                if charge {
                    "native device charge term"
                } else {
                    "native device current term"
                },
            )?;
            let (residual, scale) = if row < self.num_nodes {
                (&mut state.residual[row], &mut state.residual_scale[row])
            } else {
                (
                    &mut state.mna_branch_residual[row - self.num_nodes],
                    &mut state.mna_branch_residual_scale[row - self.num_nodes],
                )
            };
            for (k, mut contribution) in spectrum.into_iter().enumerate() {
                if charge {
                    contribution *= Complex64::new(0.0, omega * k as Value);
                }
                residual[k] += contribution;
                scale[k] += contribution.norm();
            }
        }
        Ok(())
    }

    pub(super) fn native_bjt_spectra(
        &mut self,
        state: &HbSolverState,
        harmonics: usize,
        charge: bool,
        small_signal: bool,
    ) -> Result<Vec<(usize, usize, Vec<Complex64>)>, HbError> {
        if small_signal {
            Self::validate_behavioral_response_frequency(&self.behavioral_sources)?;
        }
        if !self.has_native_periodic_devices() {
            return Ok(Vec::new());
        }
        let waves = self.native_state_waveforms(state)?;
        let mut solution = vec![0.0; waves.len()];
        let mut f = NativeStamp::new(waves.len());
        let mut q = NativeStamp::new(waves.len());
        let times = self.fft.size();
        let mut entries: BTreeMap<(usize, usize), Vec<Value>> = BTreeMap::new();
        for time in 0..times {
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            let sample_time = time as Value / times as Value / self.config.fundamental_freq;
            let mode = match state
                .junction_bias
                .as_ref()
                .and_then(|bias| Some(&bias.evaluated[self.sample_junction_range(bias, time)?]))
            {
                Some(evaluated) => JunctionBias::Recorded(evaluated),
                None => JunctionBias::Unlimited,
            };
            self.sample_native_devices(
                &solution,
                sample_time,
                &solution,
                &mut f,
                &mut q,
                small_signal,
                mode,
            )?;
            for &(row, col, value) in if charge { &q.jacobian } else { &f.jacobian } {
                let sum = &mut entries
                    .entry((row, col))
                    .or_insert_with(|| vec![0.0; times])[time];
                *sum += value;
                if !sum.is_finite() {
                    return Err(HbError::NonFiniteSample(
                        "native device Jacobian accumulation is non-finite".into(),
                    ));
                }
            }
        }
        let mut spectra = Vec::with_capacity(entries.len());
        for ((row, col), wave) in entries {
            let spectrum =
                self.checked_periodic_spectrum(&wave, harmonics, "native device Jacobian")?;
            if !spectrum.is_empty() {
                spectra.push((row, col, spectrum));
            }
        }
        Ok(spectra)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xyce_hb_carrier_registration_does_not_certify_response_frequency() {
        use crate::config::ExpressionDialect;
        use crate::device::behavioral::{BehavioralCurrentSource, BehavioralSources};
        let mut source = BehavioralCurrentSource::new("BI".into(), 1, 0, "1+frequency").unwrap();
        source.set_expression_dialect(ExpressionDialect::Xyce);
        let mut sources = BehavioralSources::new();
        sources.current_sources.push(source);
        let mut solver = HbSolver::new(HbConfig::new(1e3).with_harmonics(1), 1);
        solver.add_resistor(1, 0, 1.0);
        solver
            .set_periodic_behavioral_sources(&sources, false, 1_000_000, false, &NoAbort)
            .unwrap();
        let state = HbSolverState::new(1, 1);
        assert!(solver.native_bjt_spectra(&state, 1, false, false).is_ok());
        for error in [
            solver
                .native_bjt_spectra(&state, 1, false, true)
                .unwrap_err(),
            solver
                .set_periodic_behavioral_sources(&sources, false, 1_000_000, true, &NoAbort)
                .unwrap_err(),
        ] {
            assert!(
                error.to_string().contains("frequency-dependent equations"),
                "{error}"
            );
        }
        sources.current_sources[0].set_frequency(1e3);
        assert!(
            solver
                .set_periodic_behavioral_sources(&sources, false, 1_000_000, false, &NoAbort)
                .is_err()
        );
    }

    #[test]
    fn native_terms_preserve_harmonic_scales_without_dc_masking() {
        let mut solver = HbSolver::new(HbConfig::new(1e6).with_harmonics(2), 1);
        let count = solver.fft.size();
        for (factor, accepted) in [(0.9999, true), (0.99, false)] {
            let mut waves = Vec::new();
            let mut dc = NativeStamp::new(1);
            dc.stamp_rhs(1, 2.0);
            dc.stamp_rhs(1, -2.0);
            assert_eq!(dc.residual[0], 0.0);
            assert_eq!(dc.residual_scale[0], 4.0);
            for time in 0..count {
                let ac = 1e-4 * (std::f64::consts::TAU * time as Value / count as Value).cos();
                record_native_terms(
                    &mut waves,
                    &[(0, 2.0 + ac), (0, -2.0 - factor * ac)],
                    time,
                    count,
                )
                .unwrap();
            }
            let mut state = HbSolverState::new(1, 2);
            solver
                .add_native_waveform_terms(&mut state, waves, false)
                .unwrap();
            let expected = 0.5e-4 * (1.0 + factor);
            assert!((state.residual_scale[0][1] - expected).abs() < 1e-15);
            assert!(state.residual[0][1].norm() < expected);
            assert_eq!(
                state.rows_converged_with_branch_tolerances(1e-3, 1e-15, 1e-15),
                accepted
            );
        }
    }

    /// A state some device cannot evaluate is a sample the solver proposed,
    /// so a non-finite F, Q or Jacobian entry from any device is a rejected
    /// trial that damping and continuation step around. A stamp outside the
    /// system is a defect of the circuit whatever the state, and ends the
    /// solve.
    #[test]
    fn a_non_finite_device_sample_is_a_rejected_trial() {
        let verdict = |f: &NativeStamp, q: &NativeStamp| {
            NativeStamp::verdict(f, q, || "BJT 'Q1'".to_string())
        };
        let mut f = NativeStamp::new(2);
        let mut q = NativeStamp::new(2);
        f.stamp(1, 2, 1.0);
        f.stamp_rhs(1, -1.0);
        assert!(verdict(&f, &q).is_ok());

        f.stamp(2, 1, Value::INFINITY);
        assert!(matches!(verdict(&f, &q), Err(HbError::NonFiniteSample(_))));
        f.clear();
        // Finite contributions whose row sum overflows.
        q.stamp_rhs(2, Value::MAX);
        q.stamp_rhs(2, Value::MAX);
        let error = verdict(&f, &q).unwrap_err();
        assert!(matches!(error, HbError::NonFiniteSample(_)), "{error}");
        assert!(error.to_string().contains("BJT 'Q1'"), "{error}");
        q.clear();

        f.stamp(3, 1, 1.0);
        assert!(matches!(verdict(&f, &q), Err(HbError::InvalidCircuit(_))));
        f.clear();
        q.stamp_rhs(3, Value::NAN);
        assert!(matches!(verdict(&f, &q), Err(HbError::InvalidCircuit(_))));
    }
}
