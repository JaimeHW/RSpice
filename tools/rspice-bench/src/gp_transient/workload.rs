//! Fixed-horizon adaptive public workloads and quota-boundary measurement.

use super::observation::Observation;
use crate::error::BenchError;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::{TransientResult, TransientStartupMode};
use rspice_core::{
    Engine, GpTransientPhaseModel, Netlist, ResourceKind, SimulationConfig, SimulationError,
    SpiceDialect,
};
use serde::Serialize;
use std::sync::atomic::Ordering;
use std::time::Instant;

const STEP: f64 = 4e-12;

pub(super) struct Workload {
    pub config: SimulationConfig,
    pub netlist: Netlist,
    pub deck: String,
    pub stop: f64,
    pub schedule: Vec<f64>,
    pub cancel_at: f64,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(super) struct Waveform {
    pub points: usize,
    pub observed_samples: usize,
    pub abort_polls: usize,
    pub blake3: String,
}

#[derive(Serialize)]
pub(super) struct Cancellation {
    pub request_to_return_ms: f64,
    pub request_simulation_time_seconds: f64,
    pub accepted_samples: usize,
    pub accepted_after_request: usize,
    pub polls_after_request: usize,
}

impl Workload {
    pub fn new(
        devices: u32,
        steps: u32,
        model: GpTransientPhaseModel,
        checkpoints: bool,
    ) -> Result<Self, BenchError> {
        let mut deck = String::from(
            "GP public transient benchmark\nVC c 0 2\nVB b 0 DC .6 SIN(.6 .005 1G)\n\
             .model qm NPN IS=1e-16 BF=100 BR=1 TF=1n PTF=21 RB=100 RBM=20 IRB=1e-5 RE=1 RC=2\n\
             .options RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 GMIN=0\n",
        );
        for index in 0..devices {
            deck.push_str(&format!("Q{index} c b 0 qm\n"));
        }
        deck.push_str(".end\n");
        let netlist = Netlist::parse(&deck).map_err(|error| policy(error.to_string()))?;
        let config = SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        };
        // Keep the horizon and observation clocks independent of adaptive steps.
        let schedule = if checkpoints {
            [steps / 4, steps / 2, 3 * steps / 4]
                .map(|index| f64::from(index) * STEP)
                .to_vec()
        } else {
            Vec::new()
        };
        Ok(Self {
            config,
            netlist,
            deck,
            stop: f64::from(steps) * STEP,
            schedule,
            cancel_at: f64::from(steps) * STEP * 0.5,
        })
    }

    pub fn execute(
        &self,
        engine: &Engine,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        if self.schedule.is_empty() {
            engine.run_tran_with_abort(&self.netlist, self.stop, STEP, abort)
        } else {
            engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    &self.netlist,
                    self.stop,
                    STEP,
                    TransientStartupMode::OperatingPoint,
                    &self.schedule,
                    abort,
                )
                // The public operation includes construction and release of the
                // retained checkpoints. The output waveform is released later.
                .map(|(waveform, _checkpoints)| waveform)
        }
    }

    pub fn measure(&self, engine: &Engine) -> Result<(f64, Waveform), BenchError> {
        let observer = Observation::new(f64::INFINITY);
        let start = Instant::now();
        let result = self.execute(engine, &observer).map_err(core_error)?;
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        if result.time.last().copied() != Some(self.stop)
            || result
                .time
                .iter()
                .chain(&result.step_sizes)
                .chain(result.voltages.iter().flatten())
                .chain(result.branch_currents.iter().flatten())
                .any(|value| !value.is_finite())
        {
            return Err(policy("GP run returned incomplete or nonfinite waveforms"));
        }
        let mut digest = blake3::Hasher::new();
        for name in result.node_names.iter().chain(&result.branch_names) {
            digest.update(&(name.len() as u64).to_le_bytes());
            digest.update(name.as_bytes());
        }
        for column in [&result.time, &result.step_sizes]
            .into_iter()
            .chain(&result.voltages)
            .chain(&result.branch_currents)
        {
            digest.update(&(column.len() as u64).to_le_bytes());
            for value in column {
                digest.update(&value.to_le_bytes());
            }
        }
        Ok((
            elapsed,
            Waveform {
                points: result.time.len(),
                observed_samples: observer.samples.load(Ordering::Relaxed),
                abort_polls: observer.polls.load(Ordering::Relaxed),
                blake3: digest.finalize().to_hex().to_string(),
            },
        ))
    }

    pub fn cancel(&self, engine: &Engine) -> Result<Cancellation, BenchError> {
        let observer = Observation::new(self.cancel_at);
        let outcome = self.execute(engine, &observer);
        let returned = Instant::now();
        match outcome {
            Err(SimulationError::Aborted) => {}
            Err(error) => return Err(core_error(error)),
            Ok(_) => return Err(policy("GP cancellation returned a successful waveform")),
        }
        let requested = observer
            .requested
            .get()
            .ok_or_else(|| policy("GP run aborted before the accepted-sample trigger"))?;
        let record = Cancellation {
            request_to_return_ms: returned.duration_since(requested.0).as_secs_f64() * 1000.0,
            request_simulation_time_seconds: requested.1,
            accepted_samples: observer.samples.load(Ordering::Relaxed),
            accepted_after_request: observer.samples_after_request.load(Ordering::Relaxed),
            polls_after_request: observer.polls_after_request.load(Ordering::Relaxed),
        };
        // On these nonlinear decks, Newton polls before the next accepted
        // step. This is a work-count assertion, independent of machine speed.
        if record.accepted_after_request != 0 || record.polls_after_request == 0 {
            return Err(policy(
                "GP cancellation did not stop before the next acceptance",
            ));
        }
        Ok(record)
    }

    /// Find the smallest successful transport quota by following typed refusals.
    /// The quota cannot influence equations or step choices. Verify the final
    /// boundary at one byte less; only this resource error may advance the probe.
    pub fn peak_transport_bytes(&self) -> Result<usize, BenchError> {
        let mut config = self.config.clone();
        let ceiling = config.resource_limits.max_transport_history_bytes;
        let mut limit = 0;
        for _ in 0..128 {
            config.resource_limits.max_transport_history_bytes = limit;
            match self.execute(&Engine::new(config.clone()), &NoAbort) {
                Ok(_) => {
                    if limit > 0 {
                        config.resource_limits.max_transport_history_bytes = limit - 1;
                        match self.execute(&Engine::new(config), &NoAbort) {
                            Err(SimulationError::ResourceLimit(error))
                                if error.resource == ResourceKind::TransportHistoryBytes
                                    && error.limit == limit - 1
                                    && error.requested == limit => {}
                            _ => return Err(policy("GP transport quota boundary was not stable")),
                        }
                    }
                    return Ok(limit);
                }
                Err(SimulationError::ResourceLimit(error))
                    if error.resource == ResourceKind::TransportHistoryBytes
                        && error.limit == limit
                        && error.requested > limit
                        && error.requested <= ceiling =>
                {
                    limit = error.requested;
                }
                Err(error) => return Err(core_error(error)),
            }
        }
        Err(policy(
            "GP transport quota probe exceeded 128 increasing requests",
        ))
    }
}

fn core_error(source: SimulationError) -> BenchError {
    BenchError::CoreTransient {
        context: "public GP transient benchmark failed".into(),
        source: Box::new(source),
    }
}

pub(super) fn policy(message: impl Into<String>) -> BenchError {
    BenchError::BenchmarkPolicy {
        message: message.into(),
    }
}
