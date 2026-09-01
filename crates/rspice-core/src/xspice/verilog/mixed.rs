//! Transactional interleave for one mixed Verilog-AMS module.
//!
//! The analog solver owns Newton iteration and calls [`MixedSignalHost::stamp`]
//! for every evaluation. The host owns the digital time wheel. A trial begins
//! by delivering the exact digital slot at its timestamp; after Newton
//! convergence [`MixedSignalHost::settle_analog_bridges`] samples every A/D
//! bridge simultaneously and reports whether a same-time digital/D/A change
//! requires another Newton solve. Nothing is committed until
//! [`MixedSignalHost::accept_trial`].

use std::fmt;

use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::device::{VerilogADevice, VerilogADeviceCheckpoint};
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::vm::IntegrationCoefficients;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

use super::host::DigitalHost;
use super::{DigitalRunError, TIME_UNIT_EXPONENT, parse_four_state};
use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};

/// A failure at the mixed transient boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum MixedSignalError {
    /// Source compilation or analog runtime construction failed.
    Compile { detail: String },
    /// Digital execution failed.
    Digital(DigitalRunError),
    /// Analog evaluation or accepted-state handling failed.
    Analog { detail: String },
    /// The caller violated the begin/stamp/settle/accept-or-reject protocol.
    TrialProtocol { detail: String },
    /// An event boundary was skipped by the analog stepper.
    MissedDigitalBreakpoint {
        scheduled_seconds: f64,
        trial_seconds: f64,
    },
    /// A bridge declaration cannot be executed without guessing.
    InvalidBridge { detail: String },
    /// A D/A input was X or Z, for which no analog level is defined.
    IndeterminateDigitalBridge { signal: String, value: String },
    /// Cross-domain feedback did not quiet within the scheduler's delta cap.
    BridgeIterationLimit { tick: u64, limit: u32 },
}

impl fmt::Display for MixedSignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile { detail } => write!(f, "mixed Verilog compilation failed: {detail}"),
            Self::Digital(error) => write!(f, "mixed Verilog digital execution failed: {error}"),
            Self::Analog { detail } => write!(f, "mixed Verilog analog execution failed: {detail}"),
            Self::TrialProtocol { detail } => {
                write!(f, "mixed Verilog trial protocol error: {detail}")
            }
            Self::MissedDigitalBreakpoint {
                scheduled_seconds,
                trial_seconds,
            } => write!(
                f,
                "analog trial at {trial_seconds:e} s stepped past digital breakpoint {scheduled_seconds:e} s"
            ),
            Self::InvalidBridge { detail } => write!(f, "invalid mixed-signal bridge: {detail}"),
            Self::IndeterminateDigitalBridge { signal, value } => write!(
                f,
                "D/A bridge `{signal}` holds `{value}`; X and Z have no implicit analog level"
            ),
            Self::BridgeIterationLimit { tick, limit } => write!(
                f,
                "mixed-signal bridges at tick {tick} did not settle within {limit} iterations"
            ),
        }
    }
}

impl std::error::Error for MixedSignalError {}

impl From<DigitalRunError> for MixedSignalError {
    fn from(error: DigitalRunError) -> Self {
        Self::Digital(error)
    }
}

#[derive(Clone)]
struct AdcBridge {
    signal: DigitalSignalId,
    positive: usize,
    negative: usize,
    low: f64,
    high: f64,
}

#[derive(Clone)]
struct DacBridge {
    signal: DigitalSignalId,
    signal_name: String,
    positive: usize,
    negative: usize,
    low: f64,
    high: f64,
    resistance: f64,
}

#[derive(Clone)]
struct MixedState {
    analog: VerilogADevice,
    digital: DigitalHost,
    accepted_tick: u64,
    started: bool,
    adc: Vec<AdcBridge>,
    dac: Vec<DacBridge>,
}

#[derive(Clone)]
struct ActiveTrial {
    rollback: MixedState,
    tick: u64,
    bridge_iterations: u32,
}

/// Opaque, exact restart image for a settled mixed module.
///
/// It retains the event queue, sequence counter, process resumptions, deferred
/// updates, resolved drivers, bridge definitions, and accepted analog state.
#[derive(Clone)]
pub struct MixedSignalCheckpoint {
    source_digest: String,
    analog_checkpoint: VerilogADeviceCheckpoint,
    state: MixedState,
}

/// One compiled mixed module integrated with an outer transient solver.
pub struct MixedSignalHost {
    source_digest: String,
    resolution: TimeResolution,
    state: MixedState,
    trial: Option<ActiveTrial>,
    max_circuit_node: usize,
    max_bridge_iterations: u32,
}

impl MixedSignalHost {
    /// Compile and start one module. `terminal_nodes` maps analog ports to the
    /// outer solver's node indices.
    pub fn compile(
        source: &str,
        module: Option<&str>,
        instance: &str,
        terminal_nodes: &[usize],
        scheduler_limits: SchedulerLimits,
    ) -> Result<Self, MixedSignalError> {
        let compiler = VerilogACompiler::new(CompilerOptions {
            enable_ams: true,
            ..CompilerOptions::default()
        });
        let runtime = compiler.compile_runtime(source, module).map_err(|error| {
            MixedSignalError::Compile {
                detail: error.to_string(),
            }
        })?;
        if runtime.canonical_ir.digital.is_empty() || runtime.canonical_ir.mir.equations.is_empty()
        {
            return Err(MixedSignalError::Compile {
                detail: format!(
                    "module `{}` is not mixed: it must contain both analog equations and digital processes or drivers",
                    runtime.canonical_ir.mir.module_name
                ),
            });
        }

        #[cfg(any(feature = "veriloga-native", feature = "veriloga-wasm-jit"))]
        let analog = VerilogADevice::try_new_with_canonical_ir(
            instance,
            runtime.model,
            &runtime.canonical_ir,
            terminal_nodes,
        );
        #[cfg(not(any(feature = "veriloga-native", feature = "veriloga-wasm-jit")))]
        let analog = VerilogADevice::try_new(instance, runtime.model, terminal_nodes);
        let mut analog = analog.map_err(|error| MixedSignalError::Compile {
            detail: format!("analog device construction failed: {error}"),
        })?;
        analog
            .try_begin_analysis(2)
            .map_err(|error| MixedSignalError::Analog {
                detail: format!("transient initialization failed: {error}"),
            })?;

        let resolution = TimeResolution::new(TIME_UNIT_EXPONENT).map_err(DigitalRunError::from)?;
        let max_bridge_iterations = scheduler_limits.max_delta_cycles_per_tick.max(1);
        let mut digital =
            DigitalHost::new(&runtime.canonical_ir.digital, resolution, scheduler_limits);
        digital.start()?;
        let source_digest = runtime.canonical_ir.metadata.source_digest.to_string();
        Ok(Self {
            source_digest,
            resolution,
            state: MixedState {
                analog,
                digital,
                accepted_tick: 0,
                started: false,
                adc: Vec::new(),
                dac: Vec::new(),
            },
            trial: None,
            max_circuit_node: terminal_nodes.iter().copied().max().unwrap_or(0),
            max_bridge_iterations,
        })
    }

    /// Add a scalar analog-to-digital bridge with hysteresis.
    pub fn add_adc_bridge(
        &mut self,
        signal: &str,
        positive: usize,
        negative: usize,
        low_threshold: f64,
        high_threshold: f64,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("add a bridge")?;
        if !low_threshold.is_finite()
            || !high_threshold.is_finite()
            || low_threshold > high_threshold
        {
            return Err(MixedSignalError::InvalidBridge {
                detail: "A/D thresholds must be finite and low <= high".into(),
            });
        }
        let id = self.scalar_signal(signal)?;
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.adc.push(AdcBridge {
            signal: id,
            positive,
            negative,
            low: low_threshold,
            high: high_threshold,
        });
        Ok(())
    }

    /// Add a scalar digital-to-analog Thevenin bridge.
    pub fn add_dac_bridge(
        &mut self,
        signal: &str,
        positive: usize,
        negative: usize,
        low_level: f64,
        high_level: f64,
        output_resistance: f64,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("add a bridge")?;
        if !low_level.is_finite()
            || !high_level.is_finite()
            || !output_resistance.is_finite()
            || output_resistance <= 0.0
        {
            return Err(MixedSignalError::InvalidBridge {
                detail:
                    "D/A levels must be finite and output resistance must be finite and positive"
                        .into(),
            });
        }
        let id = self.scalar_signal(signal)?;
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.dac.push(DacBridge {
            signal: id,
            signal_name: signal.into(),
            positive,
            negative,
            low: low_level,
            high: high_level,
            resistance: output_resistance,
        });
        Ok(())
    }

    /// Earliest exact event time the analog stepper must use as a breakpoint.
    pub fn next_event_time(&self) -> Result<Option<f64>, MixedSignalError> {
        self.state
            .digital
            .next_tick()
            .map(|tick| {
                self.resolution
                    .ticks_to_seconds(tick)
                    .map_err(DigitalRunError::from)
            })
            .transpose()
            .map_err(Into::into)
    }

    /// Start a rollbackable analog trial and deliver the exact digital slot at
    /// this timestamp before the first Newton stamp.
    pub fn begin_trial(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("begin a trial")?;
        if !timestep_seconds.is_finite() || timestep_seconds < 0.0 {
            return Err(MixedSignalError::TrialProtocol {
                detail: "timestep must be finite and nonnegative".into(),
            });
        }
        let tick = self
            .resolution
            .seconds_to_ticks(time_seconds)
            .map_err(DigitalRunError::from)?;
        let grid_time = self
            .resolution
            .ticks_to_seconds(tick)
            .map_err(DigitalRunError::from)?;
        let grid_tolerance =
            f64::EPSILON * time_seconds.abs().max(grid_time.abs()).max(1.0e-30) * 4.0;
        if (grid_time - time_seconds).abs() > grid_tolerance {
            return Err(MixedSignalError::TrialProtocol {
                detail: format!(
                    "trial time {time_seconds:e} s is not on the host's exact 1 ns event grid; use next_event_time breakpoints"
                ),
            });
        }
        if tick < self.state.accepted_tick {
            return Err(MixedSignalError::TrialProtocol {
                detail: "trial time precedes the accepted mixed-module time".into(),
            });
        }
        if let Some(next) = self.state.digital.next_tick()
            && next < tick
        {
            return Err(MixedSignalError::MissedDigitalBreakpoint {
                scheduled_seconds: self
                    .resolution
                    .ticks_to_seconds(next)
                    .map_err(DigitalRunError::from)?,
                trial_seconds: time_seconds,
            });
        }

        let rollback = self.state.clone();
        let prepare = (|| {
            self.state.digital.advance_to(tick)?;
            self.state
                .analog
                .try_set_analysis_type(2)
                .map_err(analog_error)?;
            self.state
                .analog
                .try_set_analysis_step(initial_step, final_step)
                .map_err(analog_error)?;
            self.state
                .analog
                .try_set_time(time_seconds)
                .map_err(analog_error)?;
            self.state
                .analog
                .try_set_timestep(timestep_seconds)
                .map_err(analog_error)?;
            self.state
                .analog
                .try_set_integration_coefficients(integration)
                .map_err(analog_error)?;
            Ok::<(), MixedSignalError>(())
        })();
        if let Err(error) = prepare {
            self.state = rollback;
            return Err(error);
        }
        self.trial = Some(ActiveTrial {
            rollback,
            tick,
            bridge_iterations: 0,
        });
        Ok(())
    }

    /// Apply co-timed external digital input drives during the active trial.
    pub fn force_digital(&mut self, drives: &[(&str, &str)]) -> Result<(), MixedSignalError> {
        let tick = self.active_tick()?;
        let mut parsed = Vec::with_capacity(drives.len());
        for &(name, spelling) in drives {
            let signal = self.state.digital.signal(name)?;
            let value =
                parse_four_state(spelling).ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!("`{spelling}` is not a four-state value for `{name}`"),
                })?;
            parsed.push((signal, value));
        }
        self.state.digital.force_many(&parsed, tick)?;
        Ok(())
    }

    /// Stamp both the module's continuous equations and every active D/A
    /// bridge. Call this on every Newton evaluation.
    pub fn stamp<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
        mut rhs_add: R,
    ) -> Result<(), MixedSignalError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        self.active_tick()?;
        self.validate_solution(circuit_voltages)?;
        self.state
            .analog
            .try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add)
            .map_err(analog_error)?;
        for bridge in &self.state.dac {
            let value = self.state.digital.read(bridge.signal).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
                }
            })?;
            let level = match value.bit(0) {
                FourStateBit::Zero => bridge.low,
                FourStateBit::One => bridge.high,
                FourStateBit::Unknown | FourStateBit::HighImpedance => {
                    return Err(MixedSignalError::IndeterminateDigitalBridge {
                        signal: bridge.signal_name.clone(),
                        value: value.spelling(),
                    });
                }
            };
            let conductance = 1.0 / bridge.resistance;
            matrix_add(bridge.positive, bridge.positive, conductance);
            matrix_add(bridge.positive, bridge.negative, -conductance);
            matrix_add(bridge.negative, bridge.positive, -conductance);
            matrix_add(bridge.negative, bridge.negative, conductance);
            rhs_add(bridge.positive, conductance * level);
            rhs_add(bridge.negative, -conductance * level);
        }
        Ok(())
    }

    /// Sample all A/D bridges from one converged candidate, publish their
    /// changes simultaneously, and settle every same-time delta cycle.
    /// Returns true when digital activity changed any D/A input and Newton must
    /// be repeated at the same timestamp.
    pub fn settle_analog_bridges(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<bool, MixedSignalError> {
        let tick = self.active_tick()?;
        let trial = self
            .trial
            .as_mut()
            .expect("active_tick validated the trial");
        trial.bridge_iterations = trial.bridge_iterations.saturating_add(1);
        if trial.bridge_iterations > self.max_bridge_iterations {
            return Err(MixedSignalError::BridgeIterationLimit {
                tick,
                limit: self.max_bridge_iterations,
            });
        }
        self.validate_solution(circuit_voltages)?;
        let before = self.dac_values()?;
        let mut drives = Vec::new();
        for bridge in &self.state.adc {
            let voltage = circuit_voltages[bridge.positive] - circuit_voltages[bridge.negative];
            let bit = if voltage <= bridge.low {
                Some(FourStateBit::Zero)
            } else if voltage >= bridge.high {
                Some(FourStateBit::One)
            } else {
                None
            };
            if let Some(bit) = bit {
                let next = FourStateValue::splat(1, bit);
                if self.state.digital.read(bridge.signal) != Some(&next) {
                    drives.push((bridge.signal, next));
                }
            }
        }
        if !drives.is_empty() {
            self.state.digital.force_many(&drives, tick)?;
        }
        Ok(before != self.dac_values()?)
    }

    /// Commit both domains atomically after bridges and Newton are quiet.
    pub fn accept_trial(&mut self) -> Result<(), MixedSignalError> {
        let trial = self
            .trial
            .as_ref()
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "there is no active trial to accept".into(),
            })?;
        if let Err(error) = self.state.analog.validate_advance_state() {
            let rollback = self.trial.take().expect("checked above").rollback;
            self.state = rollback;
            return Err(analog_error(error));
        }
        self.state.analog.apply_validated_advance_state();
        self.state.accepted_tick = trial.tick;
        self.state.started = true;
        self.trial = None;
        Ok(())
    }

    /// Restore every analog, digital, event, driver, and bridge bit to the
    /// state at [`begin_trial`](Self::begin_trial).
    pub fn reject_trial(&mut self) -> Result<(), MixedSignalError> {
        let trial = self
            .trial
            .take()
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "there is no active trial to reject".into(),
            })?;
        self.state = trial.rollback;
        Ok(())
    }

    /// Capture a restart image. Speculative state is never checkpointable.
    pub fn checkpoint(&self) -> Result<MixedSignalCheckpoint, MixedSignalError> {
        if self.trial.is_some() {
            return Err(MixedSignalError::TrialProtocol {
                detail: "cannot checkpoint an unaccepted mixed trial".into(),
            });
        }
        let analog_checkpoint = self.state.analog.checkpoint_state().map_err(analog_error)?;
        Ok(MixedSignalCheckpoint {
            source_digest: self.source_digest.clone(),
            analog_checkpoint,
            state: self.state.clone(),
        })
    }

    /// Restore a checkpoint into a freshly compiled semantically identical
    /// host, validating analog and source identity before mutation.
    pub fn restore(&mut self, checkpoint: &MixedSignalCheckpoint) -> Result<(), MixedSignalError> {
        self.require_idle("restore a checkpoint")?;
        if checkpoint.source_digest != self.source_digest {
            return Err(MixedSignalError::TrialProtocol {
                detail: "checkpoint source identity does not match this mixed module".into(),
            });
        }
        self.state
            .analog
            .validate_checkpoint_state(&checkpoint.analog_checkpoint)
            .map_err(analog_error)?;
        self.state = checkpoint.state.clone();
        self.max_circuit_node = (0..self.state.analog.num_terminals())
            .map(|terminal| self.state.analog.node_for_terminal(terminal))
            .chain(
                self.state
                    .adc
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .chain(
                self.state
                    .dac
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .max()
            .unwrap_or(0);
        Ok(())
    }

    /// Read a digital signal without changing scheduling state.
    pub fn read_digital(&self, signal: &str) -> Result<String, MixedSignalError> {
        let id = self.state.digital.signal(signal)?;
        Ok(self
            .state
            .digital
            .read(id)
            .map(FourStateValue::spelling)
            .unwrap_or_default())
    }

    fn active_tick(&self) -> Result<u64, MixedSignalError> {
        self.trial
            .as_ref()
            .map(|trial| trial.tick)
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "begin_trial must precede this operation".into(),
            })
    }

    fn require_idle(&self, operation: &str) -> Result<(), MixedSignalError> {
        if self.trial.is_some() {
            return Err(MixedSignalError::TrialProtocol {
                detail: format!("cannot {operation} while a trial is active"),
            });
        }
        Ok(())
    }

    fn scalar_signal(&self, signal: &str) -> Result<DigitalSignalId, MixedSignalError> {
        let id = self.state.digital.signal(signal)?;
        let width = self
            .state
            .digital
            .read(id)
            .map(FourStateValue::width)
            .unwrap_or(0);
        if width != 1 {
            return Err(MixedSignalError::InvalidBridge {
                detail: format!(
                    "bridge signal `{signal}` is {width} bits wide; only scalar bridges are supported"
                ),
            });
        }
        Ok(id)
    }

    fn validate_solution(&self, values: &[f64]) -> Result<(), MixedSignalError> {
        if self.max_circuit_node >= values.len() {
            return Err(MixedSignalError::TrialProtocol {
                detail: format!(
                    "circuit solution has {} entries but mixed module references node {}",
                    values.len(),
                    self.max_circuit_node
                ),
            });
        }
        if values.iter().any(|value| !value.is_finite()) {
            return Err(MixedSignalError::Analog {
                detail: "circuit solution contains a non-finite voltage".into(),
            });
        }
        Ok(())
    }

    fn dac_values(&self) -> Result<Vec<FourStateValue>, MixedSignalError> {
        self.state
            .dac
            .iter()
            .map(|bridge| {
                self.state
                    .digital
                    .read(bridge.signal)
                    .cloned()
                    .ok_or_else(|| MixedSignalError::InvalidBridge {
                        detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
                    })
            })
            .collect()
    }
}

fn analog_error(error: impl fmt::Display) -> MixedSignalError {
    MixedSignalError::Analog {
        detail: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIXED: &str = r#"
module mixed_runtime(p, n, adc, clk, q, dac);
  inout p, n;
  electrical p, n;
  input adc, clk;
  output q, dac;
  wire adc, clk, dac;
  reg q;
  initial q = 1'b0;
  always @(posedge adc or posedge clk) begin
    q <= ~q;
    #2 q <= 1'b0;
  end
  assign dac = q;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

    fn host() -> MixedSignalHost {
        let mut host =
            MixedSignalHost::compile(MIXED, None, "xmixed", &[1, 0], SchedulerLimits::default())
                .expect("mixed source compiles and starts");
        host.add_adc_bridge("adc", 2, 0, 0.4, 0.6)
            .expect("A/D bridge");
        host.add_dac_bridge("dac", 3, 0, 0.0, 5.0, 100.0)
            .expect("D/A bridge");
        host
    }

    fn begin(host: &mut MixedSignalHost, tick: u64) {
        host.begin_trial(
            tick as f64 * 1.0e-9,
            if tick == 0 { 0.0 } else { 1.0e-9 },
            IntegrationCoefficients::inactive(),
            tick == 0,
            false,
        )
        .expect("begin exact-grid trial");
    }

    #[test]
    fn continuous_stamp_and_bridge_delta_cycles_interleave_at_one_timepoint() {
        let mut host = host();
        begin(&mut host, 0);
        assert!(!host.settle_analog_bridges(&[0.0, 1.0, 0.0, 0.0]).unwrap());
        let mut matrix = Vec::new();
        let mut rhs = Vec::new();
        host.stamp(
            &[0.0, 1.0, 0.0, 0.0],
            |row, col, value| matrix.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        )
        .unwrap();
        assert!(
            matrix
                .iter()
                .any(|&(_, _, g)| (g.abs() - 0.001).abs() < 1e-15),
            "analog equation must stamp every evaluation: {matrix:?}"
        );
        host.accept_trial().unwrap();

        begin(&mut host, 1);
        assert!(host.settle_analog_bridges(&[0.0, 1.0, 1.0, 0.0]).unwrap());
        assert_eq!(host.read_digital("q").unwrap(), "1");
        assert_eq!(host.read_digital("dac").unwrap(), "1");
        let mut high_rhs = 0.0;
        host.stamp(
            &[0.0, 1.0, 1.0, 0.0],
            |_, _, _| {},
            |row, value| {
                if row == 3 {
                    high_rhs += value;
                }
            },
        )
        .unwrap();
        assert!((high_rhs - 0.05).abs() < 1e-15);
    }

    #[test]
    fn rejected_trial_rolls_back_process_resume_event_driver_and_bridge_state() {
        let mut host = host();
        begin(&mut host, 0);
        host.settle_analog_bridges(&[0.0; 4]).unwrap();
        host.accept_trial().unwrap();

        begin(&mut host, 1);
        host.force_digital(&[("clk", "1")]).unwrap();
        assert_eq!(host.read_digital("q").unwrap(), "1");
        assert!((host.next_event_time().unwrap().unwrap() - 3.0e-9).abs() < 1.0e-20);
        host.reject_trial().unwrap();
        assert_eq!(host.read_digital("q").unwrap(), "0");
        assert_eq!(host.read_digital("clk").unwrap(), "z");
        assert_eq!(host.next_event_time().unwrap(), None);
    }

    #[test]
    fn checkpoint_resume_preserves_deltas_delays_and_resolved_drivers() {
        let mut direct = host();
        begin(&mut direct, 0);
        direct.settle_analog_bridges(&[0.0; 4]).unwrap();
        direct.accept_trial().unwrap();
        begin(&mut direct, 1);
        direct.force_digital(&[("clk", "1")]).unwrap();
        direct.accept_trial().unwrap();
        let checkpoint = direct.checkpoint().unwrap();

        let mut resumed = host();
        resumed.restore(&checkpoint).unwrap();
        for candidate in [&mut direct, &mut resumed] {
            begin(candidate, 3);
            candidate.settle_analog_bridges(&[0.0; 4]).unwrap();
            candidate.accept_trial().unwrap();
        }
        assert_eq!(
            direct.read_digital("q").unwrap(),
            resumed.read_digital("q").unwrap()
        );
        assert_eq!(
            direct.read_digital("dac").unwrap(),
            resumed.read_digital("dac").unwrap()
        );
        assert_eq!(
            direct.next_event_time().unwrap(),
            resumed.next_event_time().unwrap()
        );
    }

    #[test]
    fn co_timed_external_inputs_are_published_before_one_delta_settle() {
        let source = r#"
module simultaneous(p, n, a, b, q);
  inout p, n; electrical p, n;
  input a, b; output q; wire a, b; reg q;
  initial q = 1'b0;
  always @(posedge a or posedge b) q <= ~q;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let mut host = MixedSignalHost::compile(
            source,
            None,
            "xsimultaneous",
            &[1, 0],
            SchedulerLimits::default(),
        )
        .unwrap();
        begin(&mut host, 0);
        host.force_digital(&[("a", "1"), ("b", "1")]).unwrap();
        assert_eq!(
            host.read_digital("q").unwrap(),
            "1",
            "one sensitivity activation, not two sequential settles"
        );
    }

    #[test]
    fn mixed_runtime_fails_closed_on_malformed_and_non_mixed_sources() {
        let malformed = "module bad(p,n); inout p,n; electrical p,n; always @( endmodule";
        assert!(matches!(
            MixedSignalHost::compile(malformed, None, "xbad", &[1, 0], SchedulerLimits::default()),
            Err(MixedSignalError::Compile { .. })
        ));
        let pure_analog =
            "module a(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule";
        assert!(matches!(
            MixedSignalHost::compile(pure_analog, None, "xa", &[1, 0], SchedulerLimits::default()),
            Err(MixedSignalError::Compile { .. })
        ));
    }

    #[test]
    fn scheduler_resource_limit_aborts_a_mixed_combinational_loop() {
        let source = r#"
module mixed_osc(p, n, seed, q);
  inout p, n; electrical p, n;
  input seed; output q; wire seed; reg q;
  always @(q or seed) begin
    case (q) 1'b1: q = 1'b0; default: q = 1'b1; endcase
  end
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let limits = SchedulerLimits {
            max_delta_cycles_per_tick: 32,
            max_events_per_tick: 128,
            ..SchedulerLimits::default()
        };
        let mut host = MixedSignalHost::compile(source, None, "xosc", &[1, 0], limits).unwrap();
        host.add_adc_bridge("seed", 2, 0, 0.4, 0.6).unwrap();
        begin(&mut host, 0);
        let error = host
            .settle_analog_bridges(&[0.0, 0.0, 1.0])
            .expect_err("loop must hit a scheduler ceiling");
        assert!(matches!(
            error,
            MixedSignalError::Digital(DigitalRunError::Scheduler(_))
        ));
        host.reject_trial().unwrap();
    }
}
