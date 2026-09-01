//! Transactional interleave for one mixed Verilog-AMS module.
//!
//! The analog solver owns Newton iteration and calls [`MixedSignalHost::stamp`]
//! for every evaluation. The host owns the digital time wheel. A trial begins
//! by delivering the exact digital slot at its timestamp; after Newton
//! convergence [`MixedSignalHost::settle_analog_bridges`] samples every A/D
//! bridge simultaneously and reports whether a same-time digital/D/A change
//! requires another Newton solve. Nothing is committed until
//! [`MixedSignalHost::accept_trial`].
//!
//! # The two time bases
//!
//! The analog side names a timepoint in seconds, chosen by a step controller
//! answering to local truncation error, and it lands wherever it lands. The
//! digital side counts ticks of a declared precision. The host floors the
//! former onto the latter — see
//! [`TimeResolution::seconds_to_floor_ticks`](crate::xspice::event_scheduler::TimeResolution::seconds_to_floor_ticks)
//! for why flooring rather than rounding — and keeps the unquantized analog
//! time for everything that is answered in seconds: the trial's own bookkeeping,
//! the interpolated instant an A/D bridge crossed its threshold, and the
//! breakpoint [`MixedSignalHost::next_event_time`] hands back.
//!
//! Several analog timepoints therefore share one tick, which is what a declared
//! precision means, and the host's monotonicity is enforced on the *analog*
//! time rather than on the tick: a repeat of an accepted timepoint would
//! advance the integrator twice.
//!
//! # Which node index is which
//!
//! Every node this module names — a module terminal, an A/D sense pair, a D/A
//! output pair — is a **circuit-node id**, with `0` meaning ground, exactly as
//! [`VerilogADevice`] takes its terminal mapping. The matrix row for a node is
//! `id - 1`, and ground has no row. Bridges used to be given raw matrix rows
//! instead, which made `0` mean ground on one side of the module and the
//! circuit's first node on the other; a bridge referred to ground then stamped
//! its Thevenin conductance onto whichever node happened to occupy row zero.

use std::fmt;
use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::device::{VerilogADevice, VerilogADeviceCheckpoint};
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::vm::IntegrationCoefficients;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

use super::host::DigitalHost;
use super::{DigitalRunError, TIME_UNIT_EXPONENT, parse_four_state};
use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};
use crate::xspice::settle_cost;
use crate::xspice::threshold_crossing::threshold_crossing_time;

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

/// One payload of a mixed module's running state, shared with the trial
/// rollback image until something writes through it.
///
/// This is `SharedXspiceInstance`'s idiom, generic because the mixed host has
/// three payloads with the same shape rather than one. A trial used to capture
/// its rollback by deep-copying the whole module — the compiled analog device
/// with its context and history, the digital host with its store, scheduler,
/// process slots and sensitivity index, and both bridge tables — at every
/// attempted timepoint, whether or not the trial went on to touch any of it.
/// Behind an [`Arc`] that capture is a reference-count bump, and the copy is
/// deferred to the first write through a handle the image still shares.
///
/// The rollback image this produces is the image the deep copy produced.
/// [`Arc::make_mut`] copies whenever the pointer is shared, so an image that
/// aliases a payload observes every subsequent write on a fresh allocation and
/// never on its own; and writing is the only way to reach that path, because
/// [`Self::make_mut`] is the only mutable view. `DerefMut` is deliberately not
/// implemented, so every mutation site is spelled out.
#[derive(Clone)]
struct MixedCell<T>(Arc<T>);

impl<T: Clone> MixedCell<T> {
    fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    /// Take a mutable view, copying the payload first if a rollback image
    /// still shares it.
    ///
    /// Each copy this actually takes is counted, so `mixed_trial_copy_ratchet`
    /// can fail a change that puts the deep copies back.
    #[inline]
    fn make_mut(&mut self) -> &mut T {
        let (value, copied) = settle_cost::make_mut_reporting_copy(&mut self.0);
        if copied {
            settle_cost::note_mixed_trial_deep_copy();
        }
        value
    }
}

impl<T> std::ops::Deref for MixedCell<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

/// One scalar analog-to-digital bridge.
///
/// `positive` and `negative` are circuit-node ids; `0` is ground.
#[derive(Clone)]
struct AdcBridge {
    signal: DigitalSignalId,
    positive: usize,
    negative: usize,
    low: f64,
    high: f64,
}

/// One scalar digital-to-analog Thevenin bridge.
///
/// `positive` and `negative` are circuit-node ids; `0` is ground.
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

impl DacBridge {
    /// The analog level for a digital value that is neither 0 nor 1.
    ///
    /// IEEE 1364-2005 section 4.2.2 makes a `reg` without an initial
    /// assignment `x`, so a D/A bridge sees `x` at time zero in any design that
    /// does not initialise its output — which is most of them. Refusing there
    /// would make the common case unrunnable, so the boundary answers the way
    /// the `dac_bridge` code model answers it: the midpoint of the two levels,
    /// which is `out_undef`'s value exactly when both levels are given and it
    /// is not. See `xspice::models::bridges`'s `dac_bridge_out_undef`, and the
    /// same delegation in `engine::builder::connect_modules`.
    fn undefined_level(&self) -> f64 {
        self.low + (self.high - self.low) / 2.0
    }
}

/// One continuous-net probe of Verilog-AMS LRM 2.4 section 7.3.3, wired to the
/// circuit.
///
/// `positive` and `negative` are circuit-node ids and `0` is ground, exactly as
/// they are on a bridge — and for the same reason: the module's two halves
/// name the same node, so they had better name it the same way.
///
/// The plan names the probe's nets by the author's own identifiers, because the
/// analog levels and the discrete plan are lowered by two passes that share no
/// numbering. Resolving those names to circuit nodes is this table, built once
/// when the module is wired and never touched again — which is why it lives
/// beside the bridge declarations rather than in the rollback image.
#[derive(Clone)]
struct AnalogProbeWiring {
    positive: usize,
    negative: usize,
}

/// The bridge declarations, fixed once the module is wired.
///
/// Separate from the moving state because a bridge is added before the first
/// trial and never during one, so a rollback image can share this table for
/// the whole run without ever copying it.
#[derive(Clone, Default)]
struct Bridges {
    adc: Vec<AdcBridge>,
    dac: Vec<DacBridge>,
}

#[derive(Clone)]
struct MixedState {
    analog: MixedCell<VerilogADevice>,
    digital: MixedCell<DigitalHost>,
    bridges: MixedCell<Bridges>,
    /// Differential voltage each A/D bridge saw at the last accepted timepoint,
    /// which is the far end of the interval a threshold crossing is
    /// interpolated in. Parallel to `bridges.adc`.
    accepted_adc_voltages: Vec<f64>,
    /// Interpolated analog time of each A/D bridge's most recent accepted
    /// transition, unquantized. Parallel to `bridges.adc`.
    accepted_adc_transition_times: Vec<Option<f64>>,
    /// The continuous-net potential each of the plan's probes read at the last
    /// accepted timepoint, parallel to `MixedSignalHost::analog_probes`.
    ///
    /// This is the whole of Verilog-AMS LRM 2.4 section 7.3.6.3's answer for a
    /// process that wakes on its *own* schedule — a `#delay` or a digital
    /// edge — rather than on an analog one: at the moment such a process runs
    /// there is no Newton candidate at this timepoint yet, and the most recent
    /// analog value that exists is the one the solver last committed. A
    /// process woken by an A/D transition instead reads the converged
    /// candidate, because `settle_analog_bridges` refreshes the bank from it
    /// before publishing the transition that wakes the process.
    accepted_probe_values: Vec<f64>,
    accepted_tick: u64,
    accepted_time: f64,
    started: bool,
}

#[derive(Clone)]
struct ActiveTrial {
    rollback: MixedState,
    tick: u64,
    time_seconds: f64,
    timestep_seconds: f64,
    /// Whether this trial was opened by [`MixedSignalHost::begin_probe_trial`]
    /// and therefore may never be committed.
    ///
    /// A solver assembles a residual at times that are not candidate accepted
    /// endpoints — an LTE probe, a static residual capture at a timepoint
    /// already committed — and each of those needs the module's continuous
    /// equations and its D/A levels stamped. A probe trial delivers exactly
    /// that and nothing else: [`MixedSignalHost::accept_trial`] refuses it by
    /// name, so no route to committing one exists to be taken by mistake.
    probe: bool,
    bridge_iterations: u32,
    /// Whether the last [`MixedSignalHost::settle_analog_bridges`] of this
    /// trial found the boundary quiet. `false` before the first one, so a
    /// trial accepted without sampling its bridges is refused rather than
    /// committed on an unexamined boundary.
    bridges_quiet: bool,
    /// Interpolated crossing times published during this trial, parallel to
    /// `bridges.adc`, folded into the accepted state on acceptance.
    transition_times: Vec<Option<f64>>,
    /// Differential voltage each A/D bridge was last sampled at, parallel to
    /// `bridges.adc`. The last settle of the trial that is accepted saw the
    /// accepted solution, so this becomes the far end of the interval the next
    /// timepoint's crossings are interpolated in — without the caller having
    /// to hand the accepted solution back a second time.
    sampled_adc_voltages: Vec<f64>,
    /// Continuous-net probe values sampled during this trial, parallel to
    /// `MixedSignalHost::analog_probes`, folded into the accepted state on
    /// acceptance for the reason `sampled_adc_voltages` is.
    probe_values: Vec<f64>,
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
///
/// `Clone` exists because [`CircuitData`](crate::CircuitData) is cloneable and
/// this now lives in it: an AC sweep hands each worker thread an independent
/// copy of the whole circuit. Cloning is cheap for the same reason a trial's
/// rollback capture is — every payload is behind a [`MixedCell`], so a clone is
/// three reference-count bumps and the copy is deferred to the first write.
#[derive(Clone)]
pub struct MixedSignalHost {
    /// The deck's own name for this instance, carried so a refusal can say
    /// which X-card it is about rather than which module.
    instance: String,
    source_digest: String,
    resolution: TimeResolution,
    state: MixedState,
    trial: Option<ActiveTrial>,
    /// Every continuous-net probe the discrete half declares, resolved to
    /// circuit nodes. Empty for a module whose processes read no analog value,
    /// which is what makes the whole cross-domain read path cost such a module
    /// nothing.
    analog_probes: Vec<AnalogProbeWiring>,
    max_circuit_node: usize,
    max_bridge_iterations: u32,
}

impl fmt::Debug for MixedSignalHost {
    /// Hand-written because neither the compiled analog device nor the digital
    /// host is `Debug`, and because the useful summary of a running mixed
    /// module is its identity and its boundary shape rather than its state.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MixedSignalHost")
            .field("instance", &self.instance)
            .field("source_digest", &self.source_digest)
            .field("adc_bridges", &self.state.bridges.adc.len())
            .field("dac_bridges", &self.state.bridges.dac.len())
            .field("accepted_time", &self.state.accepted_time)
            .field("trial_active", &self.trial.is_some())
            .finish()
    }
}

impl MixedSignalHost {
    /// Compile and start one module. `terminal_nodes` maps analog ports to the
    /// outer solver's circuit-node ids, where `0` is ground.
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
        Self::from_compiled(
            instance,
            Arc::new(runtime.model),
            &runtime.canonical_ir,
            terminal_nodes,
            scheduler_limits,
        )
    }

    /// Start one module from artifacts that are already compiled and cached.
    ///
    /// This is the entry the deck route takes, and [`Self::compile`] is its
    /// composition with a compiler invocation. Splitting them is what lets a
    /// `.VERILOGA` include be compiled exactly once for a build: the engine's
    /// on-disk cache produces a `CompiledModel` and a
    /// [`CanonicalIrArtifact`](rspice_veriloga::canonical_ir::CanonicalIrArtifact)
    /// for every `.va` it reads, and a mixed module needs those same two
    /// artifacts rather than a second compile of the same text. Compiling twice
    /// would also be compiling under two different `CompilerOptions`, so the
    /// analog half a device stamped and the analog half a host stamped could
    /// differ without anything saying so.
    ///
    /// The `model` and `canonical_ir` pair must be the pair one compilation
    /// produced. That is checked, not assumed: `VerilogADevice` construction
    /// refuses a mismatched digest, and the engine's cache validates the pair
    /// before admitting it.
    pub(crate) fn from_compiled(
        instance: &str,
        model: Arc<rspice_veriloga::CompiledModel>,
        canonical_ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
        terminal_nodes: &[usize],
        scheduler_limits: SchedulerLimits,
    ) -> Result<Self, MixedSignalError> {
        if canonical_ir.digital.is_empty() || canonical_ir.mir.equations.is_empty() {
            return Err(MixedSignalError::Compile {
                detail: format!(
                    "module `{}` is not mixed: it must contain both analog equations and digital processes or drivers",
                    canonical_ir.mir.module_name
                ),
            });
        }

        // The same backend selection the device builder makes. A mixed module
        // is one more Verilog-A instance as far as the continuous half is
        // concerned, so it must not reach a different runtime than the analog
        // instance beside it would.
        #[cfg(any(feature = "veriloga-native", feature = "veriloga-wasm-jit"))]
        let analog = VerilogADevice::try_new_with_canonical_ir(
            instance,
            model,
            canonical_ir,
            terminal_nodes,
        );
        #[cfg(not(any(feature = "veriloga-native", feature = "veriloga-wasm-jit")))]
        let analog = VerilogADevice::try_new(instance, model, terminal_nodes);
        let mut analog = analog.map_err(|error| MixedSignalError::Compile {
            detail: format!("analog device construction failed: {error}"),
        })?;
        analog
            .try_begin_analysis(2)
            .map_err(|error| MixedSignalError::Analog {
                detail: format!("transient initialization failed: {error}"),
            })?;

        let analog_probes = wire_analog_probes(canonical_ir, &analog)?;

        let resolution = TimeResolution::new(TIME_UNIT_EXPONENT).map_err(DigitalRunError::from)?;
        let max_bridge_iterations = scheduler_limits.max_delta_cycles_per_tick.max(1);
        let mut digital = DigitalHost::new(&canonical_ir.digital, resolution, scheduler_limits);
        // Before `start`, because `start` places every process's first
        // activation at tick zero and an `initial` block that probes a
        // continuous net runs there. Nothing has been solved yet, so what it
        // reads is the zero vector — which is not a substitute for a solution,
        // it *is* the solution state of an unsolved matrix, and the same thing
        // a node voltage read before the first solve would give.
        let initial_probe_values = vec![0.0; analog_probes.len()];
        digital.sample_analog_potentials(&initial_probe_values);
        digital.start()?;
        let source_digest = canonical_ir.metadata.source_digest.to_string();
        Ok(Self {
            instance: instance.to_string(),
            source_digest,
            resolution,
            state: MixedState {
                analog: MixedCell::new(analog),
                digital: MixedCell::new(digital),
                bridges: MixedCell::new(Bridges::default()),
                accepted_adc_voltages: Vec::new(),
                accepted_adc_transition_times: Vec::new(),
                accepted_probe_values: initial_probe_values,
                accepted_tick: 0,
                accepted_time: 0.0,
                started: false,
            },
            trial: None,
            analog_probes,
            max_circuit_node: terminal_nodes.iter().copied().max().unwrap_or(0),
            max_bridge_iterations,
        })
    }

    /// Every continuous-net probe's differential potential, out of one circuit
    /// solution.
    ///
    /// The same arithmetic an A/D bridge samples with, through the same
    /// `node_voltage` — a probe and a bridge that name one node must agree
    /// about its voltage, and the way to guarantee that is for there to be one
    /// function that answers.
    fn sample_analog_probes(&self, circuit_voltages: &[f64]) -> Vec<f64> {
        self.analog_probes
            .iter()
            .map(|probe| {
                node_voltage(circuit_voltages, probe.positive)
                    - node_voltage(circuit_voltages, probe.negative)
            })
            .collect()
    }

    /// The deck's name for this instance.
    pub(crate) fn instance_name(&self) -> &str {
        &self.instance
    }

    /// Every circuit node this module's matrix contributions can reach, in
    /// ascending order and without ground.
    ///
    /// The matrix topology builder needs this before the first stamp: a
    /// conductance has nowhere to land unless its `(row, col)` already exists
    /// in the sparsity pattern. Analog terminals couple through the module's
    /// own equations and D/A bridges couple their two nodes, so the union is
    /// what a conservative dense block must span — the same answer, and for the
    /// same reason, that `engine::matrix` computes for a `VerilogADevice`.
    pub(crate) fn coupled_nodes(&self) -> Vec<usize> {
        let mut nodes: Vec<usize> = (0..self.state.analog.num_terminals())
            .map(|terminal| self.state.analog.node_for_terminal(terminal))
            .chain(
                self.state
                    .bridges
                    .dac
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .filter(|node| *node > 0)
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }

    /// Every boundary net's committed four-state value, paired with the circuit
    /// node the deck attached it to.
    ///
    /// Both bridge directions are reported: an A/D bridge's signal is what the
    /// module *read* off that node and a D/A bridge's is what it *drove* onto
    /// it, and a waveform viewer wants both. Speculative state is never
    /// reported — this reads the accepted store, and every trial that has not
    /// been accepted has already been rolled back.
    pub(crate) fn boundary_digital_values<F>(&self, mut sink: F)
    where
        F: FnMut(usize, &FourStateValue),
    {
        for bridge in &self.state.bridges.adc {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(bridge.positive, value);
            }
        }
        for bridge in &self.state.bridges.dac {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(bridge.positive, value);
            }
        }
    }

    /// Add a scalar analog-to-digital bridge with hysteresis.
    ///
    /// `positive` and `negative` are circuit-node ids; `0` is ground.
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
        self.state.bridges.make_mut().adc.push(AdcBridge {
            signal: id,
            positive,
            negative,
            low: low_threshold,
            high: high_threshold,
        });
        self.state.accepted_adc_voltages.push(0.0);
        self.state.accepted_adc_transition_times.push(None);
        Ok(())
    }

    /// Add a scalar digital-to-analog Thevenin bridge.
    ///
    /// `positive` and `negative` are circuit-node ids; `0` is ground.
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
        self.state.bridges.make_mut().dac.push(DacBridge {
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
    ///
    /// A tick's seconds, not a floored analog time: this is the value D5
    /// clause 2 asks the step controller to stop bit-exactly at, and the tick
    /// is where the event actually is.
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
    ///
    /// `time_seconds` is the analog timepoint, wherever the step controller
    /// put it. It is floored onto the tick grid to name the digital slot, and
    /// kept unquantized for everything answered in seconds.
    pub fn begin_trial(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), MixedSignalError> {
        self.begin_trial_inner(
            time_seconds,
            timestep_seconds,
            integration,
            initial_step,
            final_step,
            false,
        )
    }

    /// Start a trial that is guaranteed never to be committed.
    ///
    /// The solver's rollbackable probe. `stamp_xspice_transient_trial` is the
    /// same shape one layer out: evaluate, stamp, restore, decide nothing. A
    /// probe is what lets a Newton iteration see this module's continuous
    /// equations and its D/A levels without the boundary having been decided,
    /// and what lets a residual be assembled at a timepoint that is not a
    /// candidate endpoint at all — an LTE probe, or the static residual capture
    /// a Xyce OneStep step takes *after* committing the timepoint it is
    /// capturing.
    ///
    /// The one difference from [`Self::begin_trial`] is the monotonicity check.
    /// That check exists because acceptance advances the analog integrator, so
    /// a second acceptance at one instant would advance it twice; a probe never
    /// reaches that path — [`Self::accept_trial`] refuses one by name — so the
    /// check has nothing to protect and would refuse legitimate assemblies.
    /// Every other guard, the missed-breakpoint one included, still applies.
    pub(crate) fn begin_probe_trial(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), MixedSignalError> {
        self.begin_trial_inner(
            time_seconds,
            timestep_seconds,
            integration,
            initial_step,
            final_step,
            true,
        )
    }

    fn begin_trial_inner(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
        probe: bool,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("begin a trial")?;
        if !timestep_seconds.is_finite() || timestep_seconds < 0.0 {
            return Err(MixedSignalError::TrialProtocol {
                detail: "timestep must be finite and nonnegative".into(),
            });
        }
        let tick = self
            .resolution
            .seconds_to_floor_ticks(time_seconds)
            .map_err(DigitalRunError::from)?;
        // Monotonicity is enforced on the analog time, not on the tick. Many
        // trials share one tick once the step controller is in charge, and all
        // of them are legitimate; what is not legitimate is repeating or
        // preceding a timepoint already accepted, because `accept_trial`
        // advances the analog integrator and a second acceptance at the same
        // instant would advance it twice.
        if !probe && self.state.started && time_seconds <= self.state.accepted_time {
            return Err(MixedSignalError::TrialProtocol {
                detail: format!(
                    "trial time {time_seconds:e} s does not advance past the accepted mixed-module \
                     time {:e} s; a repeated timepoint would advance the integrator twice",
                    self.state.accepted_time
                ),
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
            // Ask before taking the mutable view. `advance_to` on a tick with
            // nothing due is a no-op, but taking the view is not: it copies the
            // whole digital host out of the rollback image that was just
            // captured. Most timepoints of an LTE-controlled transient have no
            // digital event due, and this is the predicate that lets them cost
            // nothing — the same shape as the XSPICE drain's
            // `has_event_at_or_before`.
            if self
                .state
                .digital
                .next_tick()
                .is_some_and(|next| next <= tick)
            {
                // The slot about to run may contain a process that probes a
                // continuous net. Nothing has been solved at this timepoint
                // yet, so what it reads is the last accepted analog solution —
                // Verilog-AMS LRM 2.4 section 7.3.6.3's "analog value
                // calculated for the time corresponding to a real promotion of
                // the digital time", held from the last timepoint at or before
                // this tick. Refreshed before `advance_to` rather than inside
                // it, so every process in the slot sees one solution.
                let probes = self.state.accepted_probe_values.clone();
                let digital = self.state.digital.make_mut();
                digital.sample_analog_potentials(&probes);
                digital.advance_to(tick)?;
            }
            let analog = self.state.analog.make_mut();
            analog.try_set_analysis_type(2).map_err(analog_error)?;
            analog
                .try_set_analysis_step(initial_step, final_step)
                .map_err(analog_error)?;
            analog.try_set_time(time_seconds).map_err(analog_error)?;
            analog
                .try_set_timestep(timestep_seconds)
                .map_err(analog_error)?;
            analog
                .try_set_integration_coefficients(integration)
                .map_err(analog_error)?;
            Ok::<(), MixedSignalError>(())
        })();
        if let Err(error) = prepare {
            self.state = rollback;
            return Err(error);
        }
        let transition_times = self.state.accepted_adc_transition_times.clone();
        let sampled_adc_voltages = self.state.accepted_adc_voltages.clone();
        let probe_values = self.state.accepted_probe_values.clone();
        self.trial = Some(ActiveTrial {
            rollback,
            tick,
            time_seconds,
            timestep_seconds,
            probe,
            bridge_iterations: 0,
            bridges_quiet: false,
            transition_times,
            sampled_adc_voltages,
            probe_values,
        });
        Ok(())
    }

    /// Whether a trial is open.
    pub(crate) fn trial_active(&self) -> bool {
        self.trial.is_some()
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
        let probes = self
            .trial
            .as_ref()
            .map(|trial| trial.probe_values.clone())
            .unwrap_or_default();
        let digital = self.state.digital.make_mut();
        digital.sample_analog_potentials(&probes);
        digital.force_many(&parsed, tick)?;
        // A drive published into the slot can move a D/A input, so the
        // boundary is no longer known quiet.
        if let Some(trial) = self.trial.as_mut() {
            trial.bridges_quiet = false;
        }
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
            .make_mut()
            .try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add)
            .map_err(analog_error)?;
        for bridge in &self.state.bridges.dac {
            let value = self.state.digital.read(bridge.signal).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
                }
            })?;
            let level = match value.bit(0) {
                FourStateBit::Zero => bridge.low,
                FourStateBit::One => bridge.high,
                FourStateBit::Unknown | FourStateBit::HighImpedance => bridge.undefined_level(),
            };
            let conductance = 1.0 / bridge.resistance;
            // Ground has no matrix row, so a bridge referred to it stamps only
            // its live side. Stamping row `0` for it would land on whichever
            // node occupies the first row — including, for a module whose own
            // first terminal is there, its own equation.
            let positive = matrix_row(bridge.positive);
            let negative = matrix_row(bridge.negative);
            if let Some(row) = positive {
                matrix_add(row, row, conductance);
                rhs_add(row, conductance * level);
            }
            if let Some(row) = negative {
                matrix_add(row, row, conductance);
                rhs_add(row, -conductance * level);
            }
            if let (Some(positive), Some(negative)) = (positive, negative) {
                matrix_add(positive, negative, -conductance);
                matrix_add(negative, positive, -conductance);
            }
        }
        Ok(())
    }

    /// Sample all A/D bridges from one converged candidate, publish their
    /// changes simultaneously, and settle every same-time delta cycle.
    /// Returns true when digital activity changed any D/A input and Newton must
    /// be repeated at the same timestamp.
    ///
    /// Each transition is dated by interpolating its threshold crossing inside
    /// the trial's step, by the same rule the Xyce DIG code models date theirs
    /// — [`threshold_crossing_time`]. That instant is kept unquantized in the
    /// accepted state; the digital slot it is published into is its floor,
    /// which under whole-tick lockstep is this trial's own tick. It cannot be
    /// an earlier one that matters:
    /// [`Self::begin_trial`] has already refused a step that passed a scheduled
    /// event, so nothing is queued between the crossing and here to reorder
    /// against.
    pub fn settle_analog_bridges(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<bool, MixedSignalError> {
        let tick = self.active_tick()?;
        let (time_seconds, timestep_seconds, iterations) = {
            let trial = self
                .trial
                .as_mut()
                .expect("active_tick validated the trial");
            trial.bridge_iterations = trial.bridge_iterations.saturating_add(1);
            (
                trial.time_seconds,
                trial.timestep_seconds,
                trial.bridge_iterations,
            )
        };
        if iterations > self.max_bridge_iterations {
            return Err(MixedSignalError::BridgeIterationLimit {
                tick,
                limit: self.max_bridge_iterations,
            });
        }
        self.validate_solution(circuit_voltages)?;
        let before = self.dac_values()?;
        let mut drives = Vec::new();
        let mut crossings = Vec::new();
        let mut sampled = Vec::with_capacity(self.state.bridges.adc.len());
        let mut publish_tick = tick;
        for (index, bridge) in self.state.bridges.adc.iter().enumerate() {
            let voltage = node_voltage(circuit_voltages, bridge.positive)
                - node_voltage(circuit_voltages, bridge.negative);
            sampled.push(voltage);
            let (bit, threshold) = if voltage <= bridge.low {
                (Some(FourStateBit::Zero), bridge.low)
            } else if voltage >= bridge.high {
                (Some(FourStateBit::One), bridge.high)
            } else {
                (None, 0.0)
            };
            let Some(bit) = bit else { continue };
            let next = FourStateValue::splat(1, bit);
            if self.state.digital.read(bridge.signal) == Some(&next) {
                continue;
            }
            let crossing = threshold_crossing_time(
                time_seconds,
                time_seconds - timestep_seconds,
                timestep_seconds,
                self.state.accepted_adc_voltages[index],
                voltage,
                threshold,
            );
            let crossing_tick = self
                .resolution
                .seconds_to_floor_ticks(crossing)
                .map_err(DigitalRunError::from)?;
            publish_tick = publish_tick.max(crossing_tick);
            drives.push((bridge.signal, next));
            crossings.push((index, crossing));
        }
        // Sampled from the same converged candidate the bridges were, and
        // published into the store *before* the transitions that wake the
        // processes reading it. That ordering is what makes the standard's own
        // sampler exact: `always @(posedge clk) x = V(in);` wakes in the delta
        // cycle this publish opens, and reads the analog solution the edge it
        // woke on was itself detected in — Verilog-AMS LRM 2.4 section
        // 7.3.6.3's "analog value calculated for the time corresponding to a
        // real promotion of the digital time", with the two domains at one
        // timepoint and nothing to interpolate between.
        let probe_values = self.sample_analog_probes(circuit_voltages);
        if !drives.is_empty() {
            let digital = self.state.digital.make_mut();
            digital.sample_analog_potentials(&probe_values);
            digital.force_many(&drives, publish_tick)?;
            if let Some(trial) = self.trial.as_mut() {
                for (index, crossing) in crossings {
                    trial.transition_times[index] = Some(crossing);
                }
            }
        }
        let changed = before != self.dac_values()?;
        if let Some(trial) = self.trial.as_mut() {
            trial.bridges_quiet = !changed;
            trial.sampled_adc_voltages = sampled;
            trial.probe_values = probe_values;
        }
        Ok(changed)
    }

    /// Commit both domains atomically after bridges and Newton are quiet.
    ///
    /// The quiet is a precondition, and it is enforced: a trial whose last
    /// [`Self::settle_analog_bridges`] reported that a D/A input moved still
    /// owes the solver a Newton pass at this timestamp, and one that never
    /// sampled its bridges has an unexamined boundary. Committing either would
    /// advance the analog integrator over a boundary state the analog solution
    /// was never solved against.
    pub fn accept_trial(&mut self) -> Result<(), MixedSignalError> {
        let trial = self
            .trial
            .as_ref()
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "there is no active trial to accept".into(),
            })?;
        if trial.probe {
            return Err(MixedSignalError::TrialProtocol {
                detail: "a probe trial cannot be accepted; probes exist so a residual can be \
                         assembled at a timepoint the solver is not committing"
                    .into(),
            });
        }
        if !trial.bridges_quiet {
            return Err(MixedSignalError::TrialProtocol {
                detail: "settle_analog_bridges must report the boundary quiet before a trial is \
                         accepted; sample the bridges, and repeat Newton while they keep moving"
                    .into(),
            });
        }
        if let Err(error) = self.state.analog.validate_advance_state() {
            let rollback = self.trial.take().expect("checked above").rollback;
            self.state = rollback;
            return Err(analog_error(error));
        }
        let trial = self.trial.take().expect("checked above");
        self.state.analog.make_mut().apply_validated_advance_state();
        self.state.accepted_tick = trial.tick;
        self.state.accepted_time = trial.time_seconds;
        self.state.started = true;
        self.state.accepted_adc_transition_times = trial.transition_times;
        // The settle that reported the boundary quiet is the one that saw the
        // solution this acceptance keeps, so its samples are the accepted
        // voltages, and they become the far end of the interval the next
        // timepoint's crossings are interpolated in. Taking them from there
        // rather than asking the caller to hand the accepted solution back is
        // what keeps the two from ever disagreeing about which solution was
        // kept.
        self.state.accepted_adc_voltages = trial.sampled_adc_voltages;
        // And the probe bank the same settle sampled becomes what a process
        // waking on its own schedule at a later tick reads, for the same
        // reason: it is the analog solution this acceptance kept.
        self.state.accepted_probe_values = trial.probe_values;
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

    /// Interpolated analog time of the most recent accepted transition on an
    /// A/D bridge signal, in seconds and unquantized.
    ///
    /// `None` when that bridge has not transitioned since the module started.
    ///
    /// Restricted to the tests that pin the interpolation because there is no
    /// production reader yet: the engine route that would carry this into a
    /// digital trace does not exist, and a getter published ahead of its
    /// caller is a public commitment made on a guess about what the caller
    /// will want. The interpolation itself is not test-only — it decides the
    /// tick a transition is published into on every run.
    #[cfg(test)]
    fn last_transition_time(&self, signal: &str) -> Result<Option<f64>, MixedSignalError> {
        let id = self.state.digital.signal(signal)?;
        self.state
            .bridges
            .adc
            .iter()
            .position(|bridge| bridge.signal == id)
            .map(|index| self.state.accepted_adc_transition_times[index])
            .ok_or_else(|| MixedSignalError::InvalidBridge {
                detail: format!("`{signal}` is not an A/D bridge on this module"),
            })
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
                    .bridges
                    .adc
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .chain(
                self.state
                    .bridges
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

    /// Check a candidate solution covers every node this module reads.
    ///
    /// The highest circuit-node id occupies matrix row `id - 1`, so a solution
    /// of `n` entries covers ids up to and including `n`.
    fn validate_solution(&self, values: &[f64]) -> Result<(), MixedSignalError> {
        if self.max_circuit_node > values.len() {
            return Err(MixedSignalError::TrialProtocol {
                detail: format!(
                    "circuit solution has {} entries but mixed module references circuit node {}",
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
            .bridges
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

/// Resolve every continuous-net probe the discrete half declares to a pair of
/// circuit nodes.
///
/// Verilog-AMS LRM 2.4 section 7.3.3 lets a process probe any continuous net of
/// its module. What this host can *reach* is narrower, and the narrowing is the
/// deck's rather than the standard's: a module terminal is attached to a
/// circuit node the deck named, so its potential is an entry of the solution
/// vector this host is handed on every Newton evaluation. A net declared
/// `ground` is the reference, and reads zero for the same reason ground has no
/// matrix row.
///
/// An internal analog net is refused by name. Its solver index is assigned
/// after the module is built — `try_set_internal_node_indices` is the
/// builder's, not this constructor's — so wiring one here would either capture
/// a stale index or need a second wiring pass that runs later and could
/// disagree with this one. A terminal is the shape every published connect
/// module and every sampler in the standard's own examples probes, so the
/// refusal names the gap rather than guessing across it.
fn wire_analog_probes(
    canonical_ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
    analog: &VerilogADevice,
) -> Result<Vec<AnalogProbeWiring>, MixedSignalError> {
    let probes = &canonical_ir.digital.analog_probes;
    if probes.is_empty() {
        return Ok(Vec::new());
    }
    let terminals: Vec<&str> = analog
        .terminal_names()
        .iter()
        .map(|name| name.as_str())
        .collect();
    let resolve = |net: &str| -> Result<usize, MixedSignalError> {
        if canonical_ir
            .hir
            .ground_nodes
            .iter()
            .any(|ground| ground == net)
        {
            return Ok(0);
        }
        if let Some(terminal) = terminals.iter().position(|name| *name == net) {
            return Ok(analog.node_for_terminal(terminal));
        }
        Err(MixedSignalError::InvalidBridge {
            detail: format!(
                "`{net}` is probed from a discrete-domain expression but is not a terminal of \
                 module `{}`; Verilog-AMS LRM 2.4 section 7.3.3 allows a process to probe any \
                 continuous net, and this boundary reaches only the ones the deck attached to a \
                 circuit node",
                canonical_ir.mir.module_name
            ),
        })
    };
    probes
        .iter()
        .map(|probe| {
            Ok(AnalogProbeWiring {
                positive: resolve(&probe.positive)?,
                negative: match &probe.negative {
                    Some(negative) => resolve(negative)?,
                    None => 0,
                },
            })
        })
        .collect()
}

/// The matrix row a circuit-node id occupies, or `None` for ground.
#[inline]
fn matrix_row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

/// The voltage a circuit-node id holds in a solution vector.
///
/// Ground is zero by definition and has no entry, exactly as
/// `VerilogADevice::solution_value` reads it.
#[inline]
fn node_voltage(values: &[f64], node: usize) -> f64 {
    match matrix_row(node) {
        Some(row) => values.get(row).copied().unwrap_or(0.0),
        None => 0.0,
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

    /// Circuit nodes: `1` is the module's `p` terminal, `3` the A/D sense node,
    /// `4` the D/A output node. Both bridges are referred to ground, which is
    /// node `0` and has no matrix row.
    fn host() -> MixedSignalHost {
        let mut host =
            MixedSignalHost::compile(MIXED, None, "xmixed", &[1, 0], SchedulerLimits::default())
                .expect("mixed source compiles and starts");
        host.add_adc_bridge("adc", 3, 0, 0.4, 0.6)
            .expect("A/D bridge");
        host.add_dac_bridge("dac", 4, 0, 0.0, 5.0, 100.0)
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
        .expect("begin trial");
    }

    /// Settle to quiet the way a driver does — repeating Newton for as long as
    /// the boundary keeps moving — and then commit.
    fn settle_and_accept(host: &mut MixedSignalHost, voltages: &[f64]) {
        while host
            .settle_analog_bridges(voltages)
            .expect("bridges settle")
        {}
        host.accept_trial().expect("accept a quiet trial");
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
    fn a_ground_referred_dac_bridge_leaves_the_modules_own_terminal_row_alone() {
        // The module's `p` terminal is circuit node 1, which is matrix row 0.
        // The D/A bridge is referred to ground. Ground has no row, so the
        // bridge's Thevenin conductance and its source term belong only to row
        // 3 — the bridge's own node 4. A bridge given raw matrix rows instead
        // reads its ground reference as row 0 and adds 1/100 S plus a 50 mA
        // source to the module's own equation, which is a short from the
        // module's terminal to the digital output level.
        let mut host = host();
        begin(&mut host, 0);
        host.settle_analog_bridges(&[0.0, 1.0, 1.0, 0.0]).unwrap();
        assert_eq!(
            host.read_digital("dac").unwrap(),
            "1",
            "the A/D crossing must have driven the D/A input high"
        );

        let mut matrix = Vec::new();
        let mut rhs = Vec::new();
        host.stamp(
            &[0.0, 1.0, 1.0, 0.0],
            |row, col, value| matrix.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        )
        .unwrap();

        let bridge_conductance = 1.0 / 100.0;
        assert!(
            !matrix
                .iter()
                .any(|&(row, col, value)| (row == 0 || col == 0)
                    && (value.abs() - bridge_conductance).abs() < 1e-15),
            "the bridge's conductance must not reach the module's terminal row: {matrix:?}"
        );
        let row_zero_rhs: f64 = rhs
            .iter()
            .filter(|&&(row, _)| row == 0)
            .map(|&(_, value)| value)
            .sum();
        assert!(
            row_zero_rhs.abs() < 1e-15,
            "a ground-referred bridge must contribute nothing to row 0, got {row_zero_rhs}"
        );

        // And the live side is stamped exactly once, undiminished.
        let row_three_rhs: f64 = rhs
            .iter()
            .filter(|&&(row, _)| row == 3)
            .map(|&(_, value)| value)
            .sum();
        assert!((row_three_rhs - 0.05).abs() < 1e-15);
        let row_three_diagonal: f64 = matrix
            .iter()
            .filter(|&&(row, col, _)| row == 3 && col == 3)
            .map(|&(_, _, value)| value)
            .sum();
        assert!((row_three_diagonal - bridge_conductance).abs() < 1e-15);
    }

    #[test]
    fn an_off_grid_trial_time_lands_in_the_tick_it_is_inside() {
        // What an LTE-controlled transient actually offers: timepoints that
        // are nowhere near an integer nanosecond. The host used to refuse
        // every one of them.
        let mut host = host();
        host.begin_trial(0.0, 0.0, IntegrationCoefficients::inactive(), true, false)
            .expect("t = 0");
        settle_and_accept(&mut host, &[0.0; 4]);

        for time in [3.7e-10, 8.1e-10, 1.4e-9, 1.93e-9, 2.0e-9] {
            host.begin_trial(
                time,
                1.0e-10,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .unwrap_or_else(|error| panic!("{time:e} s must be accepted: {error}"));
            settle_and_accept(&mut host, &[0.0; 4]);
        }
    }

    #[test]
    fn a_repeated_or_receding_timepoint_is_refused() {
        let mut host = host();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        host.begin_trial(
            1.0e-9,
            1.0e-9,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .unwrap();
        settle_and_accept(&mut host, &[0.0; 4]);

        // The same instant again would apply the integrator's advance twice.
        let error = host
            .begin_trial(
                1.0e-9,
                1.0e-9,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .expect_err("a repeat of the accepted timepoint is refused");
        assert!(matches!(error, MixedSignalError::TrialProtocol { .. }));

        let error = host
            .begin_trial(
                5.0e-10,
                1.0e-10,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .expect_err("a timepoint before the accepted one is refused");
        assert!(matches!(error, MixedSignalError::TrialProtocol { .. }));

        // A later timepoint inside the same tick is not a repeat.
        host.begin_trial(
            1.5e-9,
            5.0e-10,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .expect("a distinct later timepoint in the same tick is a real step");
    }

    #[test]
    fn an_unsettled_boundary_is_not_accepted() {
        let mut host = host();
        begin(&mut host, 0);
        let error = host
            .accept_trial()
            .expect_err("a trial whose bridges were never sampled is refused");
        assert!(matches!(error, MixedSignalError::TrialProtocol { .. }));
        host.settle_analog_bridges(&[0.0; 4]).unwrap();
        host.accept_trial().expect("a quiet boundary commits");

        // A settle that moved a D/A input still owes Newton a pass.
        begin(&mut host, 1);
        assert!(host.settle_analog_bridges(&[0.0, 0.0, 1.0, 0.0]).unwrap());
        let error = host
            .accept_trial()
            .expect_err("a moving boundary is refused");
        assert!(matches!(error, MixedSignalError::TrialProtocol { .. }));
        assert!(!host.settle_analog_bridges(&[0.0, 0.0, 1.0, 0.0]).unwrap());
        host.accept_trial().expect("the second pass is quiet");
    }

    #[test]
    fn an_ad_transition_is_dated_by_its_interpolated_crossing() {
        let mut host = host();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        // The first sample resolves the bridge net from `z` to `0`, which is a
        // transition. There is no step to interpolate inside at time zero, so
        // it is dated there.
        assert_eq!(host.last_transition_time("adc").unwrap(), Some(0.0));

        // The sense node ramps 0 V to 1 V across the nanosecond ending at
        // 1 ns, and the A/D bridge's high threshold is 0.6 V. The crossing is
        // six tenths of the way through the step, not at its end.
        host.begin_trial(
            1.0e-9,
            1.0e-9,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .unwrap();
        while host
            .settle_analog_bridges(&[0.0, 0.0, 1.0, 0.0])
            .expect("settles")
        {}
        host.accept_trial().unwrap();

        let crossing = host
            .last_transition_time("adc")
            .unwrap()
            .expect("the bridge transitioned");
        assert!(
            (crossing - 0.6e-9).abs() < 1.0e-21,
            "expected the interpolated crossing at 0.6 ns, got {crossing:e}"
        );
        assert!(
            crossing < 1.0e-9,
            "an uninterpolated sampler would have dated this at the step's end"
        );
    }

    #[test]
    fn an_undefined_digital_level_stamps_the_midpoint_rather_than_refusing() {
        // `q` is a `reg` whose initial value the source does assign; `dac`
        // follows it through a continuous assignment, which has not run at the
        // instant a bridge is first read in a design that does not initialise.
        let source = r#"
module undriven(p, n, dac);
  inout p, n; electrical p, n;
  output dac; reg dac;
  always @(p) dac <= 1'b0;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let mut host = MixedSignalHost::compile(
            source,
            None,
            "xundriven",
            &[1, 0],
            SchedulerLimits::default(),
        )
        .expect("compiles");
        host.add_dac_bridge("dac", 2, 0, 1.0, 5.0, 100.0).unwrap();
        assert_eq!(host.read_digital("dac").unwrap(), "x");

        begin(&mut host, 0);
        let mut rhs = Vec::new();
        host.stamp(
            &[0.0, 0.0],
            |_, _, _| {},
            |row, value| rhs.push((row, value)),
        )
        .expect("an x on a D/A bridge stamps rather than refusing");
        let row_one: f64 = rhs
            .iter()
            .filter(|&&(row, _)| row == 1)
            .map(|&(_, value)| value)
            .sum();
        // out_undef is the midpoint of 1 V and 5 V, over 100 ohms.
        assert!(
            (row_one - 0.03).abs() < 1e-15,
            "expected the 3 V midpoint through 100 ohms, got {row_one}"
        );
    }

    #[test]
    fn rejected_trial_rolls_back_process_resume_event_driver_and_bridge_state() {
        let mut host = host();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);

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
        settle_and_accept(&mut direct, &[0.0; 4]);
        begin(&mut direct, 1);
        direct.force_digital(&[("clk", "1")]).unwrap();
        settle_and_accept(&mut direct, &[0.0; 4]);
        let checkpoint = direct.checkpoint().unwrap();

        let mut resumed = host();
        resumed.restore(&checkpoint).unwrap();
        for candidate in [&mut direct, &mut resumed] {
            begin(candidate, 3);
            settle_and_accept(candidate, &[0.0; 4]);
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
        host.add_adc_bridge("seed", 3, 0, 0.4, 0.6).unwrap();
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

    /// A trial that touches nothing must copy nothing.
    ///
    /// The cost ratchet for the copy-on-write rework, in
    /// `engine::xspice_settle_ratchet`'s style but scripted here rather than
    /// run off a deck, because the mixed host has no deck route yet. The
    /// structure it protects is the same one `SharedXspiceInstance` protects:
    /// a rollback capture is a reference-count bump, and the deep copy of the
    /// compiled analog device or of the digital host is deferred to the first
    /// write through a handle the image still shares.
    #[test]
    fn mixed_trial_copy_ratchet() {
        // A ceiling, not a target. Measured at 41 over these 40 timepoints:
        // one copy of the analog cell per trial, because `begin_trial` always
        // writes the device's time and that write is the first after the
        // capture, plus exactly one copy of the digital host — for the single
        // trial that had an event due. Everything after the first write in a
        // trial (the advance applied on acceptance, every A/D publication,
        // every Newton stamp) goes through a cell the image no longer shares
        // and costs nothing, and the bridge tables are never written during a
        // trial at all.
        //
        // The digital figure is the one to watch: it is small because
        // `begin_trial` asks whether anything is due before taking the
        // mutable view. Drop that predicate and it becomes one per timepoint;
        // revert the cells to a whole-state deep copy and the count moves by a
        // multiple rather than by a margin.
        const TIMEPOINTS: u64 = 40;
        const MAX_COPIES: u64 = TIMEPOINTS + 8;

        let mut host = host();
        settle_cost::reset();
        host.begin_trial(0.0, 0.0, IntegrationCoefficients::inactive(), true, false)
            .unwrap();
        settle_and_accept(&mut host, &[0.0; 4]);
        for step in 1..TIMEPOINTS {
            // Deliberately off-grid and sub-tick, which is the shape a step
            // controller produces and the shape that used to be refused.
            let time = step as f64 * 1.3e-10;
            host.begin_trial(
                time,
                1.3e-10,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .unwrap();
            settle_and_accept(&mut host, &[0.0; 4]);
        }
        let copies = settle_cost::counts().mixed_trial_deep_copies;

        assert!(
            copies <= MAX_COPIES,
            "{copies} copy-on-write copies over {TIMEPOINTS} timepoints, over the ceiling of \
             {MAX_COPIES}. This is a cost regression, not a correctness failure: a rollback \
             capture must be a reference-count bump, and each cell must be copied only by the \
             first write that a capture still shares."
        );
        assert!(
            copies > 0,
            "a run that copies nothing at all means the counter and the work are no longer in \
             the same place, not that the work became free"
        );
    }
}
