//! Complete slow-time missions with exact source events and bounded retention.
use super::*;
use crate::{ResourceKind, ResourceLimitError};

#[derive(Debug, Clone)]
pub enum SpectralEnvelopeStepping {
    /// Constant requested interval, shortened at reports, events and stop.
    Fixed {
        method: SpectralEnvelopeMethod,
        step: Value,
    },
    Adaptive {
        initial_step: Value,
        control: SpectralEnvelopeControl,
    },
}

#[derive(Debug, Clone)]
pub struct SpectralEnvelopeMissionConfig {
    pub stepping: SpectralEnvelopeStepping,
    /// Strictly increasing, exact slow-time reporting deadlines. Start, stop,
    /// and both sides of every event are retained independently of this list.
    pub reporting_times: Vec<Value>,
    pub retain_accepted_steps: bool,
    pub maximum_steps: usize,
    pub events: SpectralEnvelopeEventConfig,
}

/// A retained observation contains no integrator history. Rows have the same
/// names and signed Fourier grid as `mission.final_state`.
#[derive(Debug, Clone)]
pub struct NetlistEnvelopeSample {
    pub time: Value,
    pub source_side: EnvelopeSourceSide,
    pub spectra: Vec<Vec<Complex64>>,
    pub order: usize,
    pub iterations: usize,
    pub normalized_residual: Value,
}

#[derive(Debug, Clone)]
pub struct NetlistEnvelopeTransition {
    pub time: Value,
    /// Coulombs; non-source coordinates are empty. Never add these impulses
    /// to the finite branch-current samples.
    pub current_impulses: Vec<Vec<Complex64>>,
    /// Slow derivatives; ideal voltage-source coordinates are empty.
    pub slow_rates: Vec<Vec<Complex64>>,
}

#[derive(Debug, Clone)]
pub struct NetlistEnvelopeMission {
    pub samples: Vec<NetlistEnvelopeSample>,
    pub transitions: Vec<NetlistEnvelopeTransition>,
    pub final_state: NetlistEnvelopeState,
    pub accepted_steps: usize,
    pub rejected_steps: usize,
    /// Periodic solves used by slow integration, including rejected trials.
    /// Initialization and event projection are not counted here.
    pub spectral_solves: usize,
    pub maximum_error_ratio: Value,
}

impl PreparedSpectralEnvelope {
    fn validate_mission(
        &self,
        start: Value,
        request: &SpectralEnvelopeMissionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
        let unknowns = self.carrier_sources.len();
        if request.maximum_steps == 0 {
            return Err(invalid("mission step limit must be positive"));
        }
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            request.reporting_times.len(),
            self.limits.max_analysis_points,
        )?;
        let mut last = None;
        for &time in &request.reporting_times {
            check_abort(abort)?;
            self.validate_time(time)?;
            if time < start || last.is_some_and(|last| time <= last) {
                return Err(invalid(
                    "reporting times must increase strictly inside the mission window",
                ));
            }
            last = Some(time);
        }
        let step = match &request.stepping {
            SpectralEnvelopeStepping::Fixed { step, .. } => *step,
            SpectralEnvelopeStepping::Adaptive {
                initial_step,
                control,
            } => {
                control.validate(unknowns).map_err(numerical_error)?;
                *initial_step
            }
        };
        if !step.is_finite() || step <= 0.0 {
            return Err(invalid("mission step must be positive and finite"));
        }
        request.events.solver.validate().map_err(numerical_error)?;
        for tolerances in [
            &request.events.charge_tolerances,
            &request.events.rate_tolerances,
        ] {
            if tolerances.len() != unknowns
                || tolerances.iter().any(|v| !v.is_finite() || *v <= 0.0)
            {
                return Err(invalid(
                    "mission events require a positive finite charge and rate tolerance for every MNA row",
                ));
            }
        }
        Ok(step)
    }

    /// Initialize at the outgoing side of t=0, then execute through the
    /// prepared stop time. The optional DC state is a frozen-solve guess.
    pub fn run_mission_with_abort(
        &mut self,
        request: &SpectralEnvelopeMissionConfig,
        dc_seed: Option<&PeriodicDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeMission, SimulationError> {
        check_abort(abort)?;
        self.validate_mission(0.0, request, abort)?;
        let initial =
            self.initialize_with_abort(0.0, EnvelopeSourceSide::RightLimit, dc_seed, abort)?;
        self.continue_mission_with_abort(initial, request, abort)
    }

    /// Continue an accepted state from this exact prepared circuit. Output
    /// storage is charged against the same limit as numerical work. Failure
    /// publishes no partial mission and does not change the prepared budget.
    pub fn continue_mission_with_abort(
        &mut self,
        initial: NetlistEnvelopeState,
        request: &SpectralEnvelopeMissionConfig,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeMission, SimulationError> {
        check_abort(abort)?;
        self.validate_state(&initial)?;
        let step = self.validate_mission(initial.time(), request, abort)?;
        let limits = self.limits;
        let result = self.mission(initial, request, step, abort);
        self.limits = limits;
        result
    }

    fn reserve_mission_values(&mut self, values: usize) -> Result<(), SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            values,
            self.limits.max_result_values,
        )?;
        self.limits.max_result_values -= values;
        Ok(())
    }

    fn retain_sample(
        &mut self,
        state: &NetlistEnvelopeState,
        samples: &mut Vec<NetlistEnvelopeSample>,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            samples.len().saturating_add(1),
            self.limits.max_analysis_points,
        )?;
        let values = state
            .spectra()
            .len()
            .saturating_mul(self.grid().len())
            .saturating_mul(2)
            .saturating_add(6);
        self.reserve_mission_values(values)?;
        let mut spectra = Vec::with_capacity(state.spectra().len());
        for row in state.spectra() {
            check_abort(abort)?;
            spectra.push(row.clone());
        }
        samples.push(NetlistEnvelopeSample {
            time: state.time(),
            source_side: state.source_side(),
            spectra,
            order: state.order(),
            iterations: state.iterations(),
            normalized_residual: state.normalized_residual(),
        });
        Ok(())
    }

    fn mission(
        &mut self,
        mut state: NetlistEnvelopeState,
        request: &SpectralEnvelopeMissionConfig,
        mut step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<NetlistEnvelopeMission, SimulationError> {
        let mut samples = Vec::new();
        let mut transitions = Vec::new();
        let mut accepted_steps = 0usize;
        let mut rejected_steps = 0usize;
        let mut spectral_solves = 0usize;
        let mut maximum_error_ratio = 0.0_f64;
        let mut reporting = request.reporting_times.iter().copied().peekable();
        let at_initial_event = self.source_events_at(state.time())?.next().is_some();
        if at_initial_event && state.source_side() == EnvelopeSourceSide::Published {
            return Err(invalid(
                "mission starting on a source event requires an explicit incoming or outgoing state",
            ));
        }
        self.retain_sample(&state, &mut samples, abort)?;
        loop {
            check_abort(abort)?;
            if state.source_side() == EnvelopeSourceSide::LeftLimit
                && self.source_events_at(state.time())?.next().is_some()
            {
                let event = self.transition_event_with_abort(&state, &request.events, abort)?;
                let values = event
                    .current_impulses
                    .iter()
                    .chain(&event.slow_rates)
                    .fold(1usize, |n, row| {
                        n.saturating_add(row.len().saturating_mul(2))
                    });
                self.reserve_mission_values(values)?;
                state = event.state;
                transitions.push(NetlistEnvelopeTransition {
                    time: state.time(),
                    current_impulses: event.current_impulses,
                    slow_rates: event.slow_rates,
                });
                self.retain_sample(&state, &mut samples, abort)?;
            }
            while reporting.peek().is_some_and(|time| *time <= state.time()) {
                reporting.next();
            }
            if state.time() == self.config.stop_time {
                break;
            }
            ResourceLimitError::ensure(
                ResourceKind::AnalysisPoints,
                accepted_steps.saturating_add(1),
                request.maximum_steps.min(self.limits.max_analysis_points),
            )?;
            let deadline = reporting.peek().copied().unwrap_or(self.config.stop_time);
            let next_event = self.next_source_event_after(state.time())?;
            state = match &request.stepping {
                SpectralEnvelopeStepping::Fixed { method, step } => {
                    let deadline = next_event.map_or(deadline, |event| deadline.min(event));
                    let time = (state.time() + step).min(deadline);
                    if time <= state.time() {
                        return Err(invalid("mission step does not advance representable time"));
                    }
                    let side = if next_event == Some(time) {
                        EnvelopeSourceSide::LeftLimit
                    } else {
                        EnvelopeSourceSide::Published
                    };
                    spectral_solves = spectral_solves.saturating_add(1);
                    self.step_with_abort(&state, time, *method, side, abort)?
                }
                SpectralEnvelopeStepping::Adaptive { control, .. } => {
                    let advance = self.advance_with_abort(
                        &state,
                        step,
                        deadline,
                        EnvelopeSourceSide::Published,
                        control,
                        abort,
                    )?;
                    step = advance.suggested_step;
                    rejected_steps = rejected_steps.saturating_add(advance.rejected_steps);
                    spectral_solves = spectral_solves.saturating_add(advance.spectral_solves);
                    maximum_error_ratio = maximum_error_ratio.max(advance.error_ratio);
                    advance.state
                }
            };
            accepted_steps += 1;
            if request.retain_accepted_steps
                || state.time() == deadline
                || state.time() == self.config.stop_time
                || state.source_side() == EnvelopeSourceSide::LeftLimit
            {
                self.retain_sample(&state, &mut samples, abort)?;
            }
        }
        check_abort(abort)?;
        Ok(NetlistEnvelopeMission {
            samples,
            transitions,
            final_state: state,
            accepted_steps,
            rejected_steps,
            spectral_solves,
            maximum_error_ratio,
        })
    }
}
