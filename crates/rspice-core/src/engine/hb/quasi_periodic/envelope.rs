//! Prepared netlist execution of the slow-time Fourier F/Q equations.
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{
    SpectralEnvelopeControl, SpectralEnvelopeMethod, SpectralEnvelopeState,
    advance_spectral_envelope_with_abort,
};
use crate::circuit::SourceTimeSide;
use crate::engine::PeriodicDcOperatingPointSeed;

#[cfg(test)]
mod tests;

/// A periodic carrier has one real clock; independent carriers retain their
/// signed tone tuples without inventing a common fundamental.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EnvelopeCarrierBasis {
    Periodic {
        frequency_hz: Value,
        harmonics: usize,
        samples: usize,
    },
    QuasiPeriodic {
        grid: QuasiPeriodicGridConfig,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpectralEnvelopeConfig {
    pub carrier: EnvelopeCarrierBasis,
    pub solver: QuasiPeriodicSolveConfig,
    /// AC-only carrier drives require explicit tone assignments. Authored
    /// periodic waveforms supply their own clocks, as in QPSS.
    pub source_tones: Vec<QpssSourceTone>,
    /// These whole independent sources are sampled in slow time, including
    /// their waveform's DC value. Their AC annotations are not carrier drives.
    pub modulation_sources: Vec<String>,
    pub stop_time: Value,
    /// Fixed time basis for omitted waveform parameters. Adaptive trial step
    /// changes must never change a source's pulse rise time or default clock.
    pub source_time_step: Value,
}

/// Source limits at the exact physical timestamp, without nudging the clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeSourceSide {
    Published,
    LeftLimit,
    RightLimit,
}

impl From<EnvelopeSourceSide> for SourceTimeSide {
    fn from(side: EnvelopeSourceSide) -> Self {
        match side {
            EnvelopeSourceSide::Published => Self::Published,
            EnvelopeSourceSide::LeftLimit => Self::LeftLimit,
            EnvelopeSourceSide::RightLimit => Self::RightLimit,
        }
    }
}

#[derive(Debug)]
struct Coordinates {
    grid: Arc<QuasiPeriodicGrid>,
    nodes: Vec<String>,
    branches: Vec<String>,
    auxiliaries: Vec<String>,
    modulation_sources: Vec<String>,
}

/// Opaque accepted history bound to one prepared circuit. Clones can branch
/// a mission, but a separately prepared circuit cannot consume this state.
/// This in-memory handle is not a serialized periodic operating point.
#[derive(Debug, Clone)]
pub struct NetlistEnvelopeState {
    coordinates: Arc<Coordinates>,
    numerical: SpectralEnvelopeState,
}

impl NetlistEnvelopeState {
    pub fn time(&self) -> Value {
        self.numerical.time()
    }
    pub fn grid(&self) -> &Arc<QuasiPeriodicGrid> {
        &self.coordinates.grid
    }
    /// Coordinate order is nodes, physical branches, then auxiliary states.
    pub fn spectra(&self) -> &[Vec<Complex64>] {
        self.numerical.spectra()
    }
    pub fn node_names(&self) -> &[String] {
        &self.coordinates.nodes
    }
    pub fn branch_names(&self) -> &[String] {
        &self.coordinates.branches
    }
    pub fn auxiliary_names(&self) -> &[String] {
        &self.coordinates.auxiliaries
    }
    pub fn iterations(&self) -> usize {
        self.numerical.iterations()
    }
    pub fn normalized_residual(&self) -> Value {
        self.numerical.normalized_residual()
    }
    pub fn order(&self) -> usize {
        self.numerical.order()
    }
    /// Drop the older stencil only. Event algebraic jumps/impulses must be
    /// handled separately; this does not project a new physical state.
    pub fn restart_integration_history(&mut self) {
        self.numerical.restart_integration_history();
    }
}

#[derive(Debug, Clone)]
pub struct NetlistEnvelopeAdvance {
    pub state: NetlistEnvelopeState,
    pub suggested_step: Value,
    pub error_ratio: Value,
    pub rejected_steps: usize,
    pub spectral_solves: usize,
}

struct SlowSource {
    voltage: bool,
    index: usize,
    rows: Vec<(usize, Value)>,
}

/// Owns the elaborated waveform snapshots and fixed F/Q device registry.
/// Trial calls sample the original sources, so rejection or out-of-order
/// error probes cannot overwrite the waveforms or accepted charge history.
pub struct PreparedSpectralEnvelope {
    config: SpectralEnvelopeConfig,
    circuit: CircuitData,
    solver: HbSolver,
    coordinates: Arc<Coordinates>,
    carrier_sources: Vec<Vec<Complex64>>,
    slow_sources: Vec<SlowSource>,
    limits: ResourceLimits,
}

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("spectral Envelope: {}", message.into()))
}

impl Engine {
    pub fn prepare_spectral_envelope_with_abort(
        &self,
        netlist: &Netlist,
        config: SpectralEnvelopeConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedSpectralEnvelope, SimulationError> {
        check_abort(abort)?;
        if !config.stop_time.is_finite()
            || config.stop_time <= 0.0
            || !config.source_time_step.is_finite()
            || config.source_time_step <= 0.0
        {
            return Err(invalid(
                "stop time and source time step must be positive and finite",
            ));
        }
        config.solver.validate().map_err(numerical_error)?;
        let engine = self.resolved_for_netlist(netlist);
        let limits = engine.config.resource_limits;
        let grid = Arc::new(
            match &config.carrier {
                EnvelopeCarrierBasis::Periodic {
                    frequency_hz,
                    harmonics,
                    samples,
                } => QuasiPeriodicGrid::periodic_with_abort(
                    *frequency_hz,
                    *harmonics,
                    *samples,
                    &limits,
                    abort,
                ),
                EnvelopeCarrierBasis::QuasiPeriodic { grid } => {
                    QuasiPeriodicGrid::new_with_abort(grid.clone(), &limits, abort)
                }
            }
            .map_err(numerical_error)?,
        );
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "spectral Envelope")?;
        circuit.set_independent_source_context(
            crate::circuit::SourceTimeBasis {
                tstep: config.source_time_step,
                tstop: config.stop_time,
            },
            engine.config.spice_dialect,
            limits,
        );
        let mut selected = BTreeSet::new();
        let mut slow_sources = Vec::new();
        let mut canonical_names = Vec::new();
        for name in &config.modulation_sources {
            check_abort(abort)?;
            if name.is_empty() || name.trim() != name || !selected.insert(name.to_ascii_lowercase())
            {
                return Err(invalid("modulation source names must be exact and unique"));
            }
            let matches = circuit
                .voltage_sources
                .names
                .iter()
                .enumerate()
                .map(|(i, n)| (true, i, n))
                .chain(
                    circuit
                        .current_sources
                        .names
                        .iter()
                        .enumerate()
                        .map(|(i, n)| (false, i, n)),
                )
                .filter(|(_, _, n)| n.eq_ignore_ascii_case(name))
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(invalid(format!(
                    "modulation source '{name}' is unknown or ambiguous"
                )));
            }
            let (voltage, index, canonical) = matches[0];
            canonical_names.push(canonical.clone());
            let rows = if voltage {
                vec![(
                    circuit.num_nodes() + circuit.voltage_sources.branch_indices[index] - 1,
                    1.0,
                )]
            } else {
                [
                    (circuit.current_sources.node_pos[index], -1.0),
                    (circuit.current_sources.node_neg[index], 1.0),
                ]
                .into_iter()
                .filter(|(node, _)| *node > 0)
                .map(|(node, sign)| (node - 1, sign))
                .collect()
            };
            slow_sources.push(SlowSource {
                voltage,
                index,
                rows,
            });
        }
        let unknowns = circuit
            .num_nodes()
            .saturating_add(circuit.num_branches())
            .saturating_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .saturating_add(circuit.behavioral_sources.integral_count())
            .saturating_add(circuit.capacitors.periodic_auxiliary_count())
            .saturating_add(Self::hb_device_auxiliary_count(&circuit));
        let bounded = source_workspace_limits(unknowns, &grid, &limits).map_err(numerical_error)?;
        crate::analysis::quasi_periodic::solve::check_workload(
            unknowns,
            &grid,
            &config.solver.linear,
            &bounded,
        )
        .map_err(numerical_error)?;
        let solver = engine.qpss_circuit_solver(&circuit, &grid, false)?;
        solver
            .validate_spectral_envelope_circuit()
            .map_err(numerical_error)?;
        let nodes = engine.hb_build_node_names(&circuit, circuit.num_nodes());
        let mut branches = solver
            .try_periodic_mna_branch_names()
            .map_err(|e| invalid(e.to_string()))?;
        let auxiliaries = branches.split_off(solver.physical_branch_count());
        let carrier_sources = sources::build_with_excluded_sources(
            &engine,
            &circuit,
            &config.source_tones,
            grid.clone(),
            &selected,
            abort,
        )?;
        check_abort(abort)?;
        Ok(PreparedSpectralEnvelope {
            config,
            circuit,
            solver,
            carrier_sources,
            slow_sources,
            limits,
            coordinates: Arc::new(Coordinates {
                grid,
                nodes,
                branches,
                auxiliaries,
                modulation_sources: canonical_names,
            }),
        })
    }
}

fn source_workspace_limits(
    unknowns: usize,
    grid: &QuasiPeriodicGrid,
    limits: &ResourceLimits,
) -> Result<ResourceLimits, QuasiPeriodicError> {
    // Prepared carrier RHS and one sampled RHS coexist with numerical work.
    let retained = unknowns.saturating_mul(grid.len()).saturating_mul(4);
    crate::ResourceLimitError::ensure(
        crate::ResourceKind::ResultValues,
        retained,
        limits.max_result_values,
    )?;
    let mut bounded = *limits;
    bounded.max_result_values -= retained;
    Ok(bounded)
}

impl PreparedSpectralEnvelope {
    pub fn config(&self) -> &SpectralEnvelopeConfig {
        &self.config
    }
    pub fn grid(&self) -> &Arc<QuasiPeriodicGrid> {
        &self.coordinates.grid
    }
    pub fn node_names(&self) -> &[String] {
        &self.coordinates.nodes
    }
    pub fn branch_names(&self) -> &[String] {
        &self.coordinates.branches
    }
    pub fn auxiliary_names(&self) -> &[String] {
        &self.coordinates.auxiliaries
    }
    pub fn modulation_sources(&self) -> &[String] {
        &self.coordinates.modulation_sources
    }

    fn validate_time(&self, time: Value) -> Result<(), SimulationError> {
        if !time.is_finite() || time < 0.0 || time > self.config.stop_time {
            Err(invalid(
                "time must be finite and inside the configured [0, stop] window",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_state(&self, previous: &NetlistEnvelopeState) -> Result<(), SimulationError> {
        if !Arc::ptr_eq(&self.coordinates, &previous.coordinates) {
            return Err(invalid("history belongs to a different prepared circuit"));
        }
        Ok(())
    }

    fn bind(&self, numerical: SpectralEnvelopeState) -> NetlistEnvelopeState {
        NetlistEnvelopeState {
            coordinates: self.coordinates.clone(),
            numerical,
        }
    }

    fn sources_at(
        &self,
        time: Value,
        side: EnvelopeSourceSide,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Complex64>>, QuasiPeriodicError> {
        let mut sources = self.carrier_sources.clone();
        let dc = self.grid().dc_index();
        for (source, name) in self.slow_sources.iter().zip(self.modulation_sources()) {
            if abort.is_aborted() {
                return Err(QuasiPeriodicError::Aborted);
            }
            let value = if source.voltage {
                self.circuit.voltage_sources.transient_value_at_on_side(
                    source.index,
                    time,
                    side.into(),
                )
            } else {
                self.circuit
                    .current_sources
                    .value_at_time_on_side(source.index, time, side.into())
            };
            if !value.is_finite() {
                return Err(QuasiPeriodicError::InvalidCircuit(format!(
                    "modulation source '{name}' is non-finite at {time:e}"
                )));
            }
            for &(row, sign) in &source.rows {
                sources[row][dc].re += sign * value;
                if !sources[row][dc].re.is_finite() {
                    return Err(QuasiPeriodicError::InvalidCircuit(
                        "slow source accumulation overflowed".into(),
                    ));
                }
            }
        }
        Ok(sources)
    }

    /// Solve the frozen periodic equations at `time` and capture physical Q
    /// and flux. A supplied configured OP is only a Newton guess; the frozen
    /// equations must still converge. Without it, the initial guess is zero.
    pub fn initialize_with_abort(
        &mut self,
        time: Value,
        side: EnvelopeSourceSide,
        dc_seed: Option<&PeriodicDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeState, SimulationError> {
        check_abort(abort)?;
        self.validate_time(time)?;
        let limits = source_workspace_limits(self.carrier_sources.len(), self.grid(), &self.limits)
            .map_err(numerical_error)?;
        let sources = self
            .sources_at(time, side, abort)
            .map_err(numerical_error)?;
        let seed = dc_seed.map(|seed| self.dc_guess(seed, abort)).transpose()?;
        let solution = self
            .solver
            .solve_quasi_periodic_with_abort(
                self.coordinates.grid.clone(),
                &self.config.solver,
                &sources,
                seed.as_deref(),
                &limits,
                abort,
            )
            .map_err(numerical_error)?;
        let numerical = self
            .solver
            .initialize_spectral_envelope_with_abort(time, &solution, &limits, abort)
            .map_err(numerical_error)?;
        Ok(self.bind(numerical))
    }

    fn dc_guess(
        &self,
        seed: &PeriodicDcOperatingPointSeed,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Complex64>>, SimulationError> {
        seed.validate_for_circuit(&self.circuit)?;
        let mut guess = vec![vec![Complex64::ZERO; self.grid().len()]; self.carrier_sources.len()];
        let (voltages, currents) = seed.solution().split_at(seed.node_names().len());
        for (names, available, values, offset) in [
            (self.node_names(), seed.node_names(), voltages, 0),
            (
                self.branch_names(),
                seed.branch_names(),
                currents,
                self.node_names().len(),
            ),
        ] {
            for (row, name) in names.iter().enumerate() {
                check_abort(abort)?;
                if let Some(index) = available.iter().position(|n| n.eq_ignore_ascii_case(name)) {
                    guess[offset + row][self.grid().dc_index()] =
                        Complex64::new(values[index], 0.0);
                } else if offset == 0 || row < self.circuit.num_branches() {
                    return Err(invalid(format!(
                        "DC initial guess has no coordinate '{name}'"
                    )));
                }
            }
        }
        Ok(guess)
    }

    /// Advance to an exact time without automatic error estimation. The
    /// caller owns the schedule and must split intervals at source events.
    pub fn step_with_abort(
        &mut self,
        previous: &NetlistEnvelopeState,
        time: Value,
        method: SpectralEnvelopeMethod,
        side: EnvelopeSourceSide,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeState, SimulationError> {
        check_abort(abort)?;
        self.validate_state(previous)?;
        self.validate_time(time)?;
        let limits = source_workspace_limits(self.carrier_sources.len(), self.grid(), &self.limits)
            .map_err(numerical_error)?;
        let numerical = self
            .trial(&previous.numerical, time, method, side, &limits, abort)
            .map_err(numerical_error)?;
        Ok(self.bind(numerical))
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "physical time, method, source side and trial limits are independent"
    )]
    fn trial(
        &mut self,
        previous: &SpectralEnvelopeState,
        time: Value,
        method: SpectralEnvelopeMethod,
        side: EnvelopeSourceSide,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeState, QuasiPeriodicError> {
        let sources = self.sources_at(time, side, abort)?;
        self.solver.step_spectral_envelope_with_abort(
            previous,
            time,
            method,
            &self.config.solver,
            &sources,
            limits,
            abort,
        )
    }

    /// Take one error-controlled step, landing no later than `deadline`.
    /// `deadline_side` is used only at that exact endpoint; interior probes
    /// use the published waveform. The caller must provide the next event or
    /// output deadline and handle algebraic jumps before continuing past it.
    #[expect(
        clippy::too_many_arguments,
        reason = "accepted state, controller and event endpoint are independent"
    )]
    pub fn advance_with_abort(
        &mut self,
        previous: &NetlistEnvelopeState,
        requested_step: Value,
        deadline: Value,
        deadline_side: EnvelopeSourceSide,
        control: &SpectralEnvelopeControl,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeAdvance, SimulationError> {
        check_abort(abort)?;
        self.validate_state(previous)?;
        self.validate_time(deadline)?;
        let limits = source_workspace_limits(self.carrier_sources.len(), self.grid(), &self.limits)
            .map_err(numerical_error)?;
        let advance = advance_spectral_envelope_with_abort(
            &previous.numerical,
            requested_step,
            deadline,
            control,
            &limits,
            abort,
            |state, time, method, limits, abort| {
                let side = if time.to_bits() == deadline.to_bits() {
                    deadline_side
                } else {
                    EnvelopeSourceSide::Published
                };
                self.trial(state, time, method, side, limits, abort)
            },
        )
        .map_err(numerical_error)?;
        Ok(NetlistEnvelopeAdvance {
            state: self.bind(advance.state),
            suggested_step: advance.suggested_step,
            error_ratio: advance.error_ratio,
            rejected_steps: advance.rejected_steps,
            spectral_solves: advance.spectral_solves,
        })
    }
}
