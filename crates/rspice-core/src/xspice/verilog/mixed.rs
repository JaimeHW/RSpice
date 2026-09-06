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
//! digital side counts ticks of a declared precision. Two different questions
//! cross between them, and they do not quantize the same way. Conflating them
//! is the mistake this section exists to prevent.
//!
//! *How far may the digital world be advanced?* is answered from the trial's
//! own timestamp, **floored** onto the tick grid — see
//! [`TimeResolution::seconds_to_floor_ticks`](crate::xspice::event_scheduler::TimeResolution::seconds_to_floor_ticks)
//! for why flooring rather than rounding. Rounding up here would run the
//! digital world past an analog instant the integrator has not accepted, which
//! is the one thing conservative lockstep forbids.
//!
//! *Which tick does an A/D transition's event land on?* is answered from that
//! transition's own interpolated crossing time, rounded to the **nearest**
//! tick, because Verilog-AMS LRM 2.4 section 7.3.6.1 places an analog event in
//! the digital domain at the nearest digital time tick.
//! [`MixedSignalHost::settle_analog_bridges`] clamps the answer forward
//! against the trial's tick, so this rounding can move a transition later but
//! never back into a slot the digital world has left.
//!
//! Either way the unquantized analog time is kept for everything that is
//! answered in seconds: the trial's own bookkeeping, the interpolated instant
//! an A/D bridge crossed its threshold, and the breakpoint
//! [`MixedSignalHost::next_event_time`] hands back.
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

/// How many consecutive accepted timepoints may each move one boundary net
/// before the interleave calls it feedback rather than signal.
///
/// A boundary net a resolved waveform drives moves at most once in several
/// accepted timepoints. Both directions make that so, and neither is a
/// coincidence: a D/A output's next activation is a breakpoint the stepper
/// lands on exactly, and between two of them the LTE controller re-expands the
/// step; an A/D input's threshold crossing is interpolated inside one accepted
/// step, so resolving the crossing is what the controller is doing rather than
/// something it does repeatedly.
///
/// A net that moves at *every* accepted timepoint for this long is therefore
/// not a signal this stepper failed to resolve. It is a boundary the analog
/// solution flips and that flips the analog solution: the comparator whose
/// digital inverse drives its own reference, with no delay anywhere in the
/// loop. Such a loop has no consistent value at one timepoint, so there is
/// nothing for a smaller step to find, and the run would otherwise chatter to
/// `tstop` and report a trace.
///
/// 128 rather than a smaller number because the cost of being wrong is
/// asymmetric: a false positive refuses a deck that would have run, and a false
/// negative costs the extra timepoints it takes to reach the ceiling.
const MAX_CONSECUTIVE_BOUNDARY_FLIPS: u32 = 128;

/// How many of a boundary net's most recent accepted values a diagnostic
/// carries.
///
/// Eight, because the evidence a reader needs from a chattering net is the
/// *pattern* — an alternation says feedback, a run of one value says the count
/// is measuring something else — and eight values show a period-two or
/// period-four alternation unambiguously. They are carried as two bits each in
/// a `u16` so an accepted timepoint costs a shift rather than an allocation.
const BOUNDARY_VALUE_HISTORY: u32 = 8;

/// One boundary net's recent history at accepted timepoints.
///
/// Kept in the accepted state, so a rejected trial's chatter is not counted:
/// the whole question this answers is whether the *committed* boundary keeps
/// moving.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BoundaryNetHistory {
    /// Consecutive accepted timepoints whose trial moved this net.
    run: u32,
    /// The values it held at the last [`BOUNDARY_VALUE_HISTORY`] accepted
    /// timepoints, two bits each, most recent in the low bits.
    recent: u16,
    /// How many of those slots have been written, so a young net does not
    /// report seven zeroes it never held.
    filled: u32,
}

impl BoundaryNetHistory {
    fn code(bit: FourStateBit) -> u16 {
        match bit {
            FourStateBit::Zero => 0,
            FourStateBit::One => 1,
            FourStateBit::Unknown => 2,
            FourStateBit::HighImpedance => 3,
        }
    }

    fn spelling(code: u16) -> &'static str {
        match code {
            0 => "0",
            1 => "1",
            2 => "x",
            _ => "z",
        }
    }

    fn push(&mut self, bit: FourStateBit, moved: bool) {
        self.recent = (self.recent << 2) | Self::code(bit);
        self.filled = self.filled.saturating_add(1).min(BOUNDARY_VALUE_HISTORY);
        self.run = if moved { self.run.saturating_add(1) } else { 0 };
    }

    /// The retained values, oldest first.
    fn values(&self) -> Vec<String> {
        (0..self.filled)
            .rev()
            .map(|slot| Self::spelling((self.recent >> (2 * slot)) & 0b11).to_string())
            .collect()
    }
}

/// One boundary net's part in a settle that would not quiet.
///
/// Crate-private, and rendered into the error rather than carried into it. The
/// structured form would be three public types — this, an enum for which
/// ceiling tripped, and a struct holding the list — and the only consumer any
/// of them would have is [`MixedSignalError`]'s own `Display`, which is not
/// worth three entries of this crate's public-surface budget. If a caller ever
/// needs to *branch* on a participant rather than read about one, that is the
/// point to publish them.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryNetActivity {
    /// The module's own name for the net.
    signal: String,
    /// Circuit node the deck attached it to; `0` is ground.
    node: usize,
    /// Whether the module reads this net across an A/D bridge, or drives it
    /// across a D/A one.
    read_by_module: bool,
    /// How many times the net moved: consecutive accepted timepoints for an
    /// accepted-flip run, settle passes within one trial for a pass limit.
    moves: u32,
    /// The values it took, oldest first.
    recent: Vec<String>,
}

impl fmt::Display for BoundaryNetActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.read_by_module {
            "read by the module"
        } else {
            "driven by the module"
        };
        write!(
            f,
            "net `{}` on circuit node {} ({side}) moved {} times and took {}",
            self.signal,
            self.node,
            self.moves,
            self.recent.join(" ")
        )
    }
}

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
    /// Cross-domain feedback the analog stepper cannot resolve, with the nets
    /// that were in it.
    ///
    /// A cross-domain zero-delay loop is unbounded in the same way a same-tick
    /// digital one is, and it is diagnosed the same way: the ceiling exists to
    /// turn an unbounded process into evidence, and the evidence is the
    /// participants.
    BoundaryOscillation {
        /// Digital tick of the timepoint that tripped the ceiling.
        tick: u64,
        /// The ceiling's value.
        limit: u32,
        /// Whether the ceiling counted settle passes inside one trial, rather
        /// than consecutive accepted timepoints.
        within_one_timepoint: bool,
        /// The boundary nets that moved, busiest first and then in wiring order
        /// so the report is reproducible, each already rendered with its
        /// circuit node, its direction and the values it took.
        nets: Vec<String>,
    },
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
            Self::BoundaryOscillation {
                tick,
                limit,
                within_one_timepoint,
                nets,
            } => {
                let what = if *within_one_timepoint {
                    "settle passes at one timepoint"
                } else {
                    "consecutive accepted timepoints"
                };
                write!(
                    f,
                    "the analog/digital boundary moved on {limit} {what} up to tick {tick}"
                )?;
                for net in nets {
                    write!(f, "; {net}")?;
                }
                write!(
                    f,
                    ". A boundary the analog solution moves and that moves the analog solution has \
                     no consistent value at one timepoint, so no smaller step resolves it; break \
                     the loop with a delay, a `connectrules` transition time, or analog hysteresis"
                )
            }
        }
    }
}

impl std::error::Error for MixedSignalError {}

impl From<DigitalRunError> for MixedSignalError {
    fn from(error: DigitalRunError) -> Self {
        Self::Digital(error)
    }
}

/// One payload of a mixed module's running state, shared with a capture of it
/// until something writes through the live handle.
///
/// This is `SharedXspiceInstance`'s idiom, generic because the mixed host has
/// several payloads with the same shape rather than one. A trial used to
/// capture its rollback by deep-copying the whole module — the digital host
/// with its store, scheduler, process slots and sensitivity index, and both
/// bridge tables — at every attempted timepoint, whether or not the trial went
/// on to touch any of it. Behind an [`Arc`] that capture is a reference-count
/// bump, and the copy is deferred to the first write through a handle the
/// capture still shares.
///
/// The image this produces is the image the deep copy produced.
/// [`Arc::make_mut`] copies whenever the pointer is shared, so an image that
/// aliases a payload observes every subsequent write on a fresh allocation and
/// never on its own; and writing is the only way to reach that path, because
/// [`Self::make_mut`] is the only mutable view. `DerefMut` is deliberately not
/// implemented, so every mutation site is spelled out.
///
/// Deferral is only worth having where the write is *conditional*. The analog
/// device is written by every trial without exception, so wrapping it in a
/// capture the trial immediately unshared bought nothing and cost a deep copy
/// per trial; it is captured by nothing on the trial path now — see
/// [`MixedSignalHost::analog`] — and stays in a cell only for the captures
/// that are genuinely rare, a host clone and a checkpoint.
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

/// One analog-to-digital bridge, carrying one bit of one discrete signal.
///
/// `positive` and `negative` are circuit-node ids; `0` is ground.
///
/// A bridge is per *bit* rather than per signal because a deck names one node
/// per conductor: an `input [7:0]` boundary is eight nets, so it is eight
/// bridges over one signal. `bit` is the position [`FourStateValue::bit`]
/// counts from the least significant end, which is the position the discrete
/// half's own bit selects are lowered to.
#[derive(Clone)]
struct AdcBridge {
    signal: DigitalSignalId,
    bit: u32,
    /// The boundary net's own spelling, carried so a boundary diagnostic can
    /// name it: the module's name for a scalar, and `name[bit]` for one bit of
    /// a vector, because two bits of one port are two nets and a diagnostic
    /// that called both `count` would name neither. `DacBridge` has carried
    /// one since it was written; this side needed one for the first time when
    /// a boundary that would not settle had to name its participants rather
    /// than only its instance.
    signal_name: String,
    positive: usize,
    negative: usize,
    low: f64,
    high: f64,
}

/// One digital-to-analog Thevenin bridge, carrying one bit of one discrete
/// signal.
///
/// `positive` and `negative` are circuit-node ids; `0` is ground. `bit` is
/// read exactly as [`AdcBridge::bit`] is.
#[derive(Clone)]
struct DacBridge {
    signal: DigitalSignalId,
    bit: u32,
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

/// The solver inputs the host pushes into the analog device before it is
/// evaluated.
///
/// Mirrored here rather than read back off the device, because the host is
/// their only writer and the device publishes no getter for them. Small enough
/// to be `Copy`, which is what makes a trial able to carry the previous set
/// without allocating: putting these five back is the whole of what a rejected
/// trial owes the analog device — see [`MixedSignalHost::analog`].
#[derive(Clone, Copy)]
struct AnalogSolverInputs {
    analysis: u8,
    initial_step: bool,
    final_step: bool,
    time_seconds: f64,
    timestep_seconds: f64,
    integration: IntegrationCoefficients,
}

impl AnalogSolverInputs {
    /// What [`VerilogADevice::try_begin_analysis`] leaves a transient device
    /// holding: `VmContext::reset_analysis_state` zeroes the time and the
    /// timestep and deactivates the integration coefficients, and neither
    /// analysis-step flag is set until a trial sets one.
    const fn transient_start() -> Self {
        Self {
            analysis: 2,
            initial_step: false,
            final_step: false,
            time_seconds: 0.0,
            timestep_seconds: 0.0,
            integration: IntegrationCoefficients::inactive(),
        }
    }
}

/// The working vectors the trial machinery fills and empties.
///
/// One per host, kept across trials, because every one of these is written from
/// scratch by the pass that reads it and none of them outlives the call that
/// fills it. Allocating them per call made a settle cost a dozen `malloc`s to
/// answer a question about two bridges — the boundary tables are a handful of
/// entries wide, so the allocation dominated the arithmetic.
///
/// Held by value rather than behind a [`MixedCell`]: a capture must never see
/// one, and a host clone is welcome to start with empty ones.
#[derive(Clone, Default)]
struct TrialScratch {
    /// Each D/A bridge's driven bit before and after one boundary settle.
    ///
    /// Bits rather than [`FourStateValue`]s, which are two heap planes each.
    /// A bridge carries one bit of its signal, so its bit *is* what it drives
    /// and comparing bits is comparing what the analog side sees.
    dac_before: Vec<FourStateBit>,
    dac_after: Vec<FourStateBit>,
    /// Differential voltage each A/D bridge was sampled at.
    sampled: Vec<f64>,
    /// The continuous-net probe bank one settle sampled.
    probes: Vec<f64>,
    /// The bits one settle found moved, before they are composed into whole
    /// signal values: `(A/D bridge index, new bit)`, in bridge order.
    bit_drives: Vec<(usize, FourStateBit)>,
    /// The A/D transitions one settle publishes — one entry per *signal*, not
    /// per bit, because a vector boundary port publishes as one transition.
    drives: Vec<(DigitalSignalId, FourStateValue)>,
    /// The interpolated crossing times, paired with the bridge index.
    crossings: Vec<(usize, f64)>,
    /// The boundary histories an acceptance would produce, computed before
    /// anything is committed so a chattering boundary can still be refused.
    adc_history: Vec<BoundaryNetHistory>,
    dac_history: Vec<BoundaryNetHistory>,
    /// The five vectors of the last finished trial, ready to be refilled.
    trial: TrialVectors,
}

/// The per-trial vectors, moved between [`TrialScratch`] and [`ActiveTrial`].
///
/// A trial's own bookkeeping is parallel to the bridge tables and to the probe
/// list, so every vector here is the same length on every trial of a run.
/// Passing them back and forth rather than allocating a set per trial is what
/// makes an opened trial cost no allocation at all.
#[derive(Clone, Default)]
struct TrialVectors {
    transition_times: Vec<Option<f64>>,
    sampled_adc_voltages: Vec<f64>,
    probe_values: Vec<f64>,
    adc_moved: Vec<bool>,
    dac_moved: Vec<bool>,
}

/// Everything a rejected trial has to put back.
///
/// The analog device is deliberately not in here; [`MixedSignalHost::analog`]
/// says why.
#[derive(Clone)]
struct MixedState {
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
    /// Recent accepted history of each A/D boundary net, parallel to
    /// `bridges.adc`.
    adc_history: Vec<BoundaryNetHistory>,
    /// Recent accepted history of each D/A boundary net, parallel to
    /// `bridges.dac`.
    dac_history: Vec<BoundaryNetHistory>,
    accepted_tick: u64,
    accepted_time: f64,
    started: bool,
}

#[derive(Clone)]
struct ActiveTrial {
    /// The digital host as it stood when the trial opened — the whole of what
    /// a rejected trial has to put back.
    ///
    /// The rest of [`MixedState`] is not here because a trial cannot move it.
    /// The bridge tables are declarations, and `add_adc_bridge` and
    /// `add_dac_bridge` both `require_idle`, so no trial is open when one is
    /// added. The accepted bank — voltages, transition times, probe values,
    /// boundary histories, tick, time, `started` — is written at exactly one
    /// place, [`MixedSignalHost::accept_trial`], and every refusal that
    /// unwinds a trial there happens strictly before the first of those
    /// writes. Copying them into an image per trial was copying values no
    /// trial could have changed.
    rollback: MixedCell<DigitalHost>,
    /// The analog device's solver inputs as they stood when the trial opened.
    ///
    /// The device's *accepted* record needs no image, but these five inputs do:
    /// they are what a device carries between evaluations, and one of them —
    /// the timepoint — is encoded into a checkpoint. A rejected trial that left
    /// its own timepoint standing would make a restart image taken afterwards
    /// name a time the run never accepted.
    analog_inputs: AnalogSolverInputs,
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
    /// This trial's own bookkeeping, borrowed from the host's scratch and
    /// handed back when the trial finishes.
    ///
    /// * `transition_times` — interpolated crossing times published during this
    ///   trial, parallel to `bridges.adc`, folded into the accepted state on
    ///   acceptance.
    /// * `sampled_adc_voltages` — differential voltage each A/D bridge was last
    ///   sampled at, parallel to `bridges.adc`. The last settle of the trial
    ///   that is accepted saw the accepted solution, so this becomes the far
    ///   end of the interval the next timepoint's crossings are interpolated
    ///   in — without the caller having to hand the accepted solution back a
    ///   second time.
    /// * `probe_values` — continuous-net probe values sampled during this
    ///   trial, parallel to `MixedSignalHost::analog_probes`, folded into the
    ///   accepted state on acceptance for the reason `sampled_adc_voltages` is.
    /// * `adc_moved` / `dac_moved` — whether any settle of this trial moved
    ///   each boundary net, parallel to `bridges.adc` and `bridges.dac`.
    vectors: TrialVectors,
}

/// Opaque, exact restart image for a settled mixed module.
///
/// It retains the event queue, sequence counter, process resumptions, deferred
/// updates, resolved drivers, bridge definitions, and accepted analog state.
#[derive(Clone)]
pub struct MixedSignalCheckpoint {
    source_digest: String,
    analog_checkpoint: VerilogADeviceCheckpoint,
    /// The analog device itself, carried beside its accepted-state checkpoint
    /// because it is no longer inside `state`.
    ///
    /// `analog_checkpoint` is what a restore *validates* against — device
    /// identity, resolved shape, and every accepted value — and this is what a
    /// restore installs. Both are needed and neither substitutes for the
    /// other: the checkpoint carries no compiled program, no topology and no
    /// solver caches by design, so it cannot reconstruct a device; and the
    /// device alone would let a restore install state into a host the payload
    /// does not belong to.
    analog: MixedCell<VerilogADevice>,
    /// The solver inputs that device is holding, so a resumed host and the host
    /// it resumed from agree about the timepoint the analog half last saw.
    analog_inputs: AnalogSolverInputs,
    state: MixedState,
}

/// One compiled mixed module integrated with an outer transient solver.
///
/// `Clone` exists because [`CircuitData`](crate::CircuitData) is cloneable and
/// this now lives in it: an AC sweep hands each worker thread an independent
/// copy of the whole circuit. Cloning is cheap because every payload is behind
/// a [`MixedCell`], so a clone is three reference-count bumps and the copy is
/// deferred to the first write.
#[derive(Clone)]
pub struct MixedSignalHost {
    /// The deck's own name for this instance, carried so a refusal can say
    /// which X-card it is about rather than which module.
    instance: String,
    source_digest: String,
    resolution: TimeResolution,
    /// The module's continuous half, outside the trial's rollback image.
    ///
    /// A trial writes this device on every single attempt — the analysis type,
    /// the timepoint, the timestep, the companion coefficients, and then a
    /// Newton evaluation — so a capture taken beside it was unshared by the
    /// very next statement and the deferral bought nothing. It bought a deep
    /// copy per trial instead: 47,836 copies over the 43,017 trials of the
    /// sigma-delta benchmark, 29,701 of them for probe trials rolled back
    /// unconditionally, which was 14.5 % of that run spent copying a device to
    /// throw the copy away.
    ///
    /// It is out here because **a rejected trial has nothing in the device to
    /// restore**, and that is a property of the runtime rather than a hope
    /// about one. Every stateful analog operator evaluates its candidate from
    /// its own *committed* record — `filters::…::candidate_evaluation` opens
    /// with a clone of `self.committed`, the integration slots read
    /// `state_values_prev`, and `VmContext::apply_validated_advance_state` is
    /// the only writer that promotes a candidate into the accepted record. A
    /// trial reaches that promotion only through
    /// [`Self::accept_trial`], so a trial that is rejected leaves the accepted
    /// state bit-identical and leaves behind only candidate state that the
    /// next evaluation recomputes from the same accepted record.
    ///
    /// The plain analog route has always relied on exactly this: a rejected
    /// transient timestep re-runs `prepare_veriloga_timepoint` and re-stamps
    /// every `VerilogADevice` in the circuit without restoring one, and every
    /// Newton iteration of an accepted step re-evaluates them in place. The
    /// mixed host was the outlier, paying for a stronger guarantee than the
    /// device's own contract needs. `a_rejected_trial_leaves_the_analog_device_
    /// accepted_state_untouched` pins the property this depends on.
    ///
    /// It keeps a [`MixedCell`] all the same, because the two captures that
    /// *are* rare still want the deferral: a host clone (a `CircuitData` copy)
    /// and a [checkpoint](Self::checkpoint).
    analog: MixedCell<VerilogADevice>,
    /// The solver inputs [`Self::analog`] is currently holding.
    analog_inputs: AnalogSolverInputs,
    state: MixedState,
    /// The working vectors of the trial machinery, kept across trials.
    scratch: TrialScratch,
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
            analog: MixedCell::new(analog),
            analog_inputs: AnalogSolverInputs::transient_start(),
            state: MixedState {
                digital: MixedCell::new(digital),
                bridges: MixedCell::new(Bridges::default()),
                accepted_adc_voltages: Vec::new(),
                accepted_adc_transition_times: Vec::new(),
                accepted_probe_values: initial_probe_values,
                adc_history: Vec::new(),
                dac_history: Vec::new(),
                accepted_tick: 0,
                accepted_time: 0.0,
                started: false,
            },
            scratch: TrialScratch::default(),
            trial: None,
            analog_probes,
            max_circuit_node: terminal_nodes.iter().copied().max().unwrap_or(0),
            max_bridge_iterations,
        })
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
        let mut nodes: Vec<usize> = (0..self.analog.num_terminals())
            .map(|terminal| self.analog.node_for_terminal(terminal))
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
        F: FnMut(usize, FourStateBit),
    {
        for bridge in &self.state.bridges.adc {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(bridge.positive, value.bit(bridge.bit));
            }
        }
        for bridge in &self.state.bridges.dac {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(bridge.positive, value.bit(bridge.bit));
            }
        }
    }

    /// Add an analog-to-digital bridge with hysteresis onto one bit of
    /// `signal`.
    ///
    /// `bit` is `0` for a scalar net and the bit's own position for one
    /// conductor of a vector. `nodes` is the `(positive, negative)` circuit-node
    /// pair the boundary is sampled across, where `0` is ground — one argument
    /// because it is one differential net, which is how every reader of the
    /// bridge tables treats it.
    pub fn add_adc_bridge(
        &mut self,
        signal: &str,
        bit: u32,
        nodes: (usize, usize),
        low_threshold: f64,
        high_threshold: f64,
    ) -> Result<(), MixedSignalError> {
        let (positive, negative) = nodes;
        self.require_idle("add a bridge")?;
        if !low_threshold.is_finite()
            || !high_threshold.is_finite()
            || low_threshold > high_threshold
        {
            return Err(MixedSignalError::InvalidBridge {
                detail: "A/D thresholds must be finite and low <= high".into(),
            });
        }
        let (id, width) = self.signal_bit(signal, bit)?;
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.bridges.make_mut().adc.push(AdcBridge {
            signal: id,
            bit,
            signal_name: boundary_net_name(signal, bit, width),
            positive,
            negative,
            low: low_threshold,
            high: high_threshold,
        });
        self.state.accepted_adc_voltages.push(0.0);
        self.state.accepted_adc_transition_times.push(None);
        self.state.adc_history.push(BoundaryNetHistory::default());
        Ok(())
    }

    /// Add a digital-to-analog Thevenin bridge driven by one bit of `signal`.
    ///
    /// `bit` and `nodes` are read exactly as [`Self::add_adc_bridge`] reads
    /// them.
    pub fn add_dac_bridge(
        &mut self,
        signal: &str,
        bit: u32,
        nodes: (usize, usize),
        low_level: f64,
        high_level: f64,
        output_resistance: f64,
    ) -> Result<(), MixedSignalError> {
        let (positive, negative) = nodes;
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
        let (id, width) = self.signal_bit(signal, bit)?;
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.bridges.make_mut().dac.push(DacBridge {
            signal: id,
            bit,
            signal_name: boundary_net_name(signal, bit, width),
            positive,
            negative,
            low: low_level,
            high: high_level,
            resistance: output_resistance,
        });
        self.state.dac_history.push(BoundaryNetHistory::default());
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

        let rollback = self.state.digital.clone();
        let previous_inputs = self.analog_inputs;
        let inputs = AnalogSolverInputs {
            analysis: 2,
            initial_step,
            final_step,
            time_seconds,
            timestep_seconds,
            integration,
        };
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
            self.apply_analog_inputs(inputs)
        })();
        if let Err(error) = prepare {
            self.state.digital = rollback;
            // These are inputs this device was holding a moment ago, so
            // putting them back cannot be refused for being invalid; and if
            // the device has become unusable, the refusal worth reporting is
            // the one that made it so rather than a consequence of it.
            let _ = self.apply_analog_inputs(previous_inputs);
            return Err(error);
        }
        self.analog_inputs = inputs;
        // Refilled rather than allocated. `clone_from` and `resize` keep the
        // allocation the last trial handed back, and every one of these is the
        // same length on every trial of a run, so an opened trial allocates
        // nothing.
        let mut vectors = std::mem::take(&mut self.scratch.trial);
        vectors
            .transition_times
            .clone_from(&self.state.accepted_adc_transition_times);
        vectors
            .sampled_adc_voltages
            .clone_from(&self.state.accepted_adc_voltages);
        vectors
            .probe_values
            .clone_from(&self.state.accepted_probe_values);
        vectors.adc_moved.clear();
        vectors
            .adc_moved
            .resize(self.state.bridges.adc.len(), false);
        vectors.dac_moved.clear();
        vectors
            .dac_moved
            .resize(self.state.bridges.dac.len(), false);
        self.trial = Some(ActiveTrial {
            rollback,
            analog_inputs: previous_inputs,
            tick,
            time_seconds,
            timestep_seconds,
            probe,
            bridge_iterations: 0,
            bridges_quiet: false,
            vectors,
        });
        Ok(())
    }

    /// Whether a trial is open.
    pub(crate) fn trial_active(&self) -> bool {
        self.trial.is_some()
    }

    /// Push one set of solver inputs into the analog device.
    ///
    /// The one writer of those five, so a trial and a trial's undo cannot
    /// drift apart in what they consider the device's inputs to be. The two
    /// analysis setters return without touching anything when the value they
    /// are handed is the value the device already holds, which is what makes
    /// the undo path cost a handful of comparisons.
    fn apply_analog_inputs(&mut self, inputs: AnalogSolverInputs) -> Result<(), MixedSignalError> {
        let analog = self.analog.make_mut();
        analog
            .try_set_analysis_type(inputs.analysis)
            .map_err(analog_error)?;
        analog
            .try_set_analysis_step(inputs.initial_step, inputs.final_step)
            .map_err(analog_error)?;
        analog
            .try_set_time(inputs.time_seconds)
            .map_err(analog_error)?;
        analog
            .try_set_timestep(inputs.timestep_seconds)
            .map_err(analog_error)?;
        analog
            .try_set_integration_coefficients(inputs.integration)
            .map_err(analog_error)?;
        Ok(())
    }

    /// Put the analog device's solver inputs back to where a trial found them.
    fn undo_analog_inputs(&mut self, inputs: AnalogSolverInputs) -> Result<(), MixedSignalError> {
        self.apply_analog_inputs(inputs)?;
        self.analog_inputs = inputs;
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
        let probes = self
            .trial
            .as_ref()
            .map(|trial| trial.vectors.probe_values.clone())
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
        self.analog
            .make_mut()
            .try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add)
            .map_err(analog_error)?;
        for bridge in &self.state.bridges.dac {
            let value = self.state.digital.read(bridge.signal).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
                }
            })?;
            let level = match value.bit(bridge.bit) {
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
    /// accepted state, and the digital slot it is published into is the tick
    /// *nearest* it, because Verilog-AMS LRM 2.4 section 7.3.6.1 places an
    /// analog event in the digital domain at the nearest digital time tick.
    ///
    /// Nearest here and floored in [`Self::begin_trial`] are not a
    /// contradiction, because they quantize two different quantities — see
    /// this module's "two time bases". That one floors *the trial's* timestamp
    /// to bound how far the digital world may be advanced. This one rounds
    /// *the transition's* timestamp to name the tick its event belongs to, and
    /// the standard fixes that at the nearest tick.
    ///
    /// The `max` is what keeps the rounding one-directional. A crossing in the
    /// lower half of the trial's tick rounds to the tick before it, which is a
    /// slot the digital world has already left, so the clamp publishes it here
    /// instead; a crossing in the upper half rounds to the tick after, which is
    /// still ahead and is left alone. Nothing is queued between the crossing
    /// and here to reorder against, because [`Self::begin_trial`] has already
    /// refused a step that passed a scheduled event.
    ///
    /// One consequence to know when reading this, because it is not obvious
    /// from the clamp: publishing forward also *opens* that later slot.
    /// [`DigitalHost::force_many`] settles the digital world to whichever tick
    /// it is handed, so an event already dated at the tick after this trial's
    /// runs in the same settle, at an analog time short of its own tick's
    /// seconds. The overshoot is bounded by one tick, because a crossing is
    /// interpolated inside the trial's own step and so cannot round further
    /// than the tick above it, and it is confined to the trial: a rejected
    /// step restores the scheduler with the rest of the state.
    ///
    /// [`DigitalHost::force_many`]: super::host::DigitalHost::force_many
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
        // Borrowed for the whole pass, so the bridge tables and the store can
        // be read while the working vectors are written. Handed back at every
        // exit — the `?`s below give up their allocations rather than their
        // correctness, and each of those refusals ends the run.
        let mut scratch = std::mem::take(&mut self.scratch);
        let settled = self.settle_into(
            &mut scratch,
            tick,
            time_seconds,
            timestep_seconds,
            circuit_voltages,
        );
        self.scratch = scratch;
        settled
    }

    /// [`Self::settle_analog_bridges`]'s body, with the scratch vectors held
    /// apart from the host so both can be borrowed at once.
    fn settle_into(
        &mut self,
        scratch: &mut TrialScratch,
        tick: u64,
        time_seconds: f64,
        timestep_seconds: f64,
        circuit_voltages: &[f64],
    ) -> Result<bool, MixedSignalError> {
        read_dac_bits(&self.state, &mut scratch.dac_before)?;
        scratch.bit_drives.clear();
        scratch.drives.clear();
        scratch.crossings.clear();
        scratch.sampled.clear();
        let mut publish_tick = tick;
        for (index, bridge) in self.state.bridges.adc.iter().enumerate() {
            let voltage = node_voltage(circuit_voltages, bridge.positive)
                - node_voltage(circuit_voltages, bridge.negative);
            scratch.sampled.push(voltage);
            let (bit, threshold) = if voltage <= bridge.low {
                (Some(FourStateBit::Zero), bridge.low)
            } else if voltage >= bridge.high {
                (Some(FourStateBit::One), bridge.high)
            } else {
                (None, 0.0)
            };
            let Some(bit) = bit else { continue };
            // Compared as a bit rather than by building the value the bridge
            // would publish: a bridge carries one bit, and a `FourStateValue`
            // is two heap planes, so building one to discover the boundary has
            // not moved was an allocation for the common answer.
            if self
                .state
                .digital
                .read(bridge.signal)
                .map(|value| value.bit(bridge.bit))
                == Some(bit)
            {
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
            // Verilog-AMS LRM 2.4 section 7.3.6.1: an analog event crossing
            // into the digital domain lands on the *nearest* digital time
            // tick. This is the transition's own timestamp being quantized,
            // which is not the mapping `begin_trial` applies to the trial's
            // timestamp — see this module's "two time bases".
            let crossing_tick = self
                .resolution
                .seconds_to_ticks(crossing)
                .map_err(DigitalRunError::from)?;
            publish_tick = publish_tick.max(crossing_tick);
            scratch.bit_drives.push((index, bit));
            scratch.crossings.push((index, crossing));
        }
        // One drive per signal, not per bridge. A vector boundary port is N
        // bridges over one discrete signal, and the store publishes a whole
        // signal at a time (`store.rs`'s `force` refuses a value that is not
        // the declared width), so the bits that moved are composed onto the
        // value the signal holds now and the port publishes as one transition
        // — which is what the discrete half's own vector assignment is. Bits
        // whose bridge is inside its threshold window did not move and keep
        // what they held.
        compose_bit_drives(&self.state, &scratch.bit_drives, &mut scratch.drives)?;
        // Sampled from the same converged candidate the bridges were, and
        // published into the store *before* the transitions that wake the
        // processes reading it. That ordering is what makes the standard's own
        // sampler exact: `always @(posedge clk) x = V(in);` wakes in the delta
        // cycle this publish opens, and reads the analog solution the edge it
        // woke on was itself detected in — Verilog-AMS LRM 2.4 section
        // 7.3.6.3's "analog value calculated for the time corresponding to a
        // real promotion of the digital time", with the two domains at one
        // timepoint and nothing to interpolate between.
        fill_analog_probes(&self.analog_probes, circuit_voltages, &mut scratch.probes);
        if !scratch.drives.is_empty() {
            let digital = self.state.digital.make_mut();
            digital.sample_analog_potentials(&scratch.probes);
            digital.force_many(&scratch.drives, publish_tick)?;
            if let Some(trial) = self.trial.as_mut() {
                for &(index, crossing) in &scratch.crossings {
                    trial.vectors.transition_times[index] = Some(crossing);
                    trial.vectors.adc_moved[index] = true;
                }
            }
        }
        read_dac_bits(&self.state, &mut scratch.dac_after)?;
        let changed = scratch.dac_before != scratch.dac_after;
        if let Some(trial) = self.trial.as_mut() {
            // Which D/A nets moved, not merely that one did. The boundary
            // diagnostic names participants, and a `!=` on the whole vector
            // knows only that the set moved.
            for (index, (was, now)) in scratch
                .dac_before
                .iter()
                .zip(&scratch.dac_after)
                .enumerate()
            {
                if was != now {
                    trial.vectors.dac_moved[index] = true;
                }
            }
            trial.bridges_quiet = !changed;
            std::mem::swap(
                &mut trial.vectors.sampled_adc_voltages,
                &mut scratch.sampled,
            );
            std::mem::swap(&mut trial.vectors.probe_values, &mut scratch.probes);
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
        if let Err(error) = self.analog.validate_advance_state() {
            let trial = self.trial.take().expect("checked above");
            self.unwind(trial);
            return Err(analog_error(error));
        }
        let mut trial = self.trial.take().expect("checked above");
        // Fold this timepoint into each boundary net's accepted history before
        // anything is committed, so a boundary that has been moving at every
        // accepted timepoint is refused with the analog integrator still where
        // the trial found it. Folded into the scratch pair rather than a fresh
        // one, so the refusal that never fires costs no allocation.
        let mut histories = (
            std::mem::take(&mut self.scratch.adc_history),
            std::mem::take(&mut self.scratch.dac_history),
        );
        self.fold_boundary_histories(&trial, &mut histories.0, &mut histories.1);
        if let Some(oscillation) = self.boundary_flip_run(&histories.0, &histories.1, trial.tick) {
            self.scratch.adc_history = histories.0;
            self.scratch.dac_history = histories.1;
            self.unwind(trial);
            return Err(oscillation);
        }
        // Swapped rather than assigned, so the bank the accepted state is
        // giving up becomes next acceptance's scratch instead of a free.
        std::mem::swap(&mut self.state.adc_history, &mut histories.0);
        std::mem::swap(&mut self.state.dac_history, &mut histories.1);
        self.scratch.adc_history = histories.0;
        self.scratch.dac_history = histories.1;
        self.analog.make_mut().apply_validated_advance_state();
        self.state.accepted_tick = trial.tick;
        self.state.accepted_time = trial.time_seconds;
        self.state.started = true;
        std::mem::swap(
            &mut self.state.accepted_adc_transition_times,
            &mut trial.vectors.transition_times,
        );
        // The settle that reported the boundary quiet is the one that saw the
        // solution this acceptance keeps, so its samples are the accepted
        // voltages, and they become the far end of the interval the next
        // timepoint's crossings are interpolated in. Taking them from there
        // rather than asking the caller to hand the accepted solution back is
        // what keeps the two from ever disagreeing about which solution was
        // kept.
        std::mem::swap(
            &mut self.state.accepted_adc_voltages,
            &mut trial.vectors.sampled_adc_voltages,
        );
        // And the probe bank the same settle sampled becomes what a process
        // waking on its own schedule at a later tick reads, for the same
        // reason: it is the analog solution this acceptance kept.
        std::mem::swap(
            &mut self.state.accepted_probe_values,
            &mut trial.vectors.probe_values,
        );
        self.scratch.trial = trial.vectors;
        Ok(())
    }

    /// Put a trial back the way it found things, and take its vectors back.
    ///
    /// The three refusals that unwind a trial — a failed advance validation, a
    /// chattering boundary, and [`Self::reject_trial`] — differ only in what
    /// they report, so they say what they undo in one place. The two analysis
    /// setters put the device's own inputs back and cannot be refused for
    /// values the device was holding a moment ago; if one is, the refusal worth
    /// reporting is the caller's rather than a consequence of it.
    fn unwind(&mut self, trial: ActiveTrial) {
        self.state.digital = trial.rollback;
        let _ = self.undo_analog_inputs(trial.analog_inputs);
        self.scratch.trial = trial.vectors;
    }

    /// Restore every digital, event and driver bit to the state at
    /// [`begin_trial`](Self::begin_trial).
    ///
    /// Plus the analog device's five solver inputs, which are scalars. That is
    /// the whole restore, and the three things it does not name are not
    /// omissions. The bridge tables cannot have moved, because adding one
    /// requires an idle host. The accepted bank cannot have moved, because
    /// [`Self::accept_trial`] is its only writer. And the analog device's
    /// *state* has nothing to put back: a trial that does not reach
    /// `accept_trial` never reaches `apply_validated_advance_state`, which is
    /// the only promotion of a candidate into the device's accepted record, so
    /// what a rejected trial leaves behind is candidate state the next
    /// evaluation recomputes from the same accepted record. [`Self::analog`]
    /// carries the argument in full.
    pub fn reject_trial(&mut self) -> Result<(), MixedSignalError> {
        let trial = self
            .trial
            .take()
            .ok_or_else(|| MixedSignalError::TrialProtocol {
                detail: "there is no active trial to reject".into(),
            })?;
        let inputs = trial.analog_inputs;
        self.unwind(trial);
        // Reported here and swallowed in the other two unwinds, because this
        // is the one that has no refusal of its own to report.
        self.undo_analog_inputs(inputs)
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
        let analog_checkpoint = self.analog.checkpoint_state().map_err(analog_error)?;
        Ok(MixedSignalCheckpoint {
            source_digest: self.source_digest.clone(),
            analog_checkpoint,
            analog: self.analog.clone(),
            analog_inputs: self.analog_inputs,
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
        self.analog
            .validate_checkpoint_state(&checkpoint.analog_checkpoint)
            .map_err(analog_error)?;
        self.analog = checkpoint.analog.clone();
        self.analog_inputs = checkpoint.analog_inputs;
        self.state = checkpoint.state.clone();
        self.max_circuit_node = (0..self.analog.num_terminals())
            .map(|terminal| self.analog.node_for_terminal(terminal))
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

    /// The boundary histories this trial's acceptance would produce.
    ///
    /// Computed rather than applied, so the caller can refuse before anything
    /// is committed. A net whose signal has disappeared from the store — which
    /// `stamp` and `dac_values` both refuse on — is recorded as high impedance
    /// rather than skipped, so the parallel indexing holds.
    fn fold_boundary_histories(
        &self,
        trial: &ActiveTrial,
        adc: &mut Vec<BoundaryNetHistory>,
        dac: &mut Vec<BoundaryNetHistory>,
    ) {
        let read_bit = |signal, bit| {
            self.state
                .digital
                .read(signal)
                .map_or(FourStateBit::HighImpedance, |value| value.bit(bit))
        };
        adc.clear();
        for (index, bridge) in self.state.bridges.adc.iter().enumerate() {
            let mut entry = self
                .state
                .adc_history
                .get(index)
                .copied()
                .unwrap_or_default();
            entry.push(
                read_bit(bridge.signal, bridge.bit),
                trial.vectors.adc_moved.get(index).copied().unwrap_or(false),
            );
            adc.push(entry);
        }
        dac.clear();
        for (index, bridge) in self.state.bridges.dac.iter().enumerate() {
            let mut entry = self
                .state
                .dac_history
                .get(index)
                .copied()
                .unwrap_or_default();
            entry.push(
                read_bit(bridge.signal, bridge.bit),
                trial.vectors.dac_moved.get(index).copied().unwrap_or(false),
            );
            dac.push(entry);
        }
    }

    /// The diagnostic for a boundary that has moved at every accepted timepoint
    /// for too long, or `None` when none has.
    fn boundary_flip_run(
        &self,
        adc_history: &[BoundaryNetHistory],
        dac_history: &[BoundaryNetHistory],
        tick: u64,
    ) -> Option<MixedSignalError> {
        let tripped = adc_history
            .iter()
            .chain(dac_history)
            .any(|entry| entry.run > MAX_CONSECUTIVE_BOUNDARY_FLIPS);
        if !tripped {
            return None;
        }
        let mut nets: Vec<BoundaryNetActivity> = self
            .state
            .bridges
            .adc
            .iter()
            .zip(adc_history)
            .map(|(bridge, entry)| BoundaryNetActivity {
                signal: bridge.signal_name.clone(),
                node: bridge.positive,
                read_by_module: true,
                moves: entry.run,
                recent: entry.values(),
            })
            .chain(
                self.state
                    .bridges
                    .dac
                    .iter()
                    .zip(dac_history)
                    .map(|(bridge, entry)| BoundaryNetActivity {
                        signal: bridge.signal_name.clone(),
                        node: bridge.positive,
                        read_by_module: false,
                        moves: entry.run,
                        recent: entry.values(),
                    }),
            )
            .filter(|net| net.moves > 0)
            .collect();
        // Busiest first, and stable in wiring order under a tie, so two runs of
        // one deck report the participants in one order.
        nets.sort_by(|left, right| right.moves.cmp(&left.moves));
        Some(MixedSignalError::BoundaryOscillation {
            tick,
            limit: MAX_CONSECUTIVE_BOUNDARY_FLIPS,
            within_one_timepoint: false,
            nets: nets.iter().map(BoundaryNetActivity::to_string).collect(),
        })
    }

    /// The diagnostic for a trial whose boundary settle never reported quiet.
    ///
    /// Reached from the engine's own settle loop
    /// (`circuit::mixed_signal::settle_to_quiet`), which owns that ceiling
    /// because it owns the Newton pass a moving boundary owes. The nets are the
    /// ones this trial's settles moved, with the values they currently hold —
    /// within one trial there is no history to show, because a settle pass is
    /// not a timepoint.
    pub(crate) fn boundary_settle_oscillation(&self, limit: u32) -> MixedSignalError {
        let Some(trial) = self.trial.as_ref() else {
            return MixedSignalError::TrialProtocol {
                detail: "there is no active trial whose boundary could be unsettled".into(),
            };
        };
        let read = |signal, bit| {
            self.state
                .digital
                .read(signal)
                .map_or(FourStateBit::HighImpedance, |value| value.bit(bit))
        };
        let spelling =
            |bit| BoundaryNetHistory::spelling(BoundaryNetHistory::code(bit)).to_string();
        let mut nets: Vec<BoundaryNetActivity> = self
            .state
            .bridges
            .adc
            .iter()
            .zip(&trial.vectors.adc_moved)
            .map(|(bridge, moved)| BoundaryNetActivity {
                signal: bridge.signal_name.clone(),
                node: bridge.positive,
                read_by_module: true,
                moves: u32::from(*moved),
                recent: vec![spelling(read(bridge.signal, bridge.bit))],
            })
            .chain(
                self.state
                    .bridges
                    .dac
                    .iter()
                    .zip(&trial.vectors.dac_moved)
                    .map(|(bridge, moved)| BoundaryNetActivity {
                        signal: bridge.signal_name.clone(),
                        node: bridge.positive,
                        read_by_module: false,
                        moves: u32::from(*moved),
                        recent: vec![spelling(read(bridge.signal, bridge.bit))],
                    }),
            )
            .filter(|net| net.moves > 0)
            .collect();
        nets.sort_by(|left, right| right.moves.cmp(&left.moves));
        MixedSignalError::BoundaryOscillation {
            tick: trial.tick,
            limit,
            within_one_timepoint: true,
            nets: nets.iter().map(BoundaryNetActivity::to_string).collect(),
        }
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

    /// Resolve a bridge's signal and check the bit it claims exists.
    ///
    /// Returns the signal and its declared width, because the caller needs the
    /// width to spell the net: one conductor of a vector is `name[bit]` and a
    /// scalar is just `name`, and only the width separates them.
    ///
    /// A width of zero is a real net. It has no bits to bridge, so it is
    /// refused here as well as at the builder, which is the same rule
    /// `add_adc_bridge` and `add_dac_bridge` have always resolved a name
    /// against — the plan is the authority, so the classification and the
    /// lookup cannot disagree.
    fn signal_bit(
        &self,
        signal: &str,
        bit: u32,
    ) -> Result<(DigitalSignalId, u32), MixedSignalError> {
        let id = self.state.digital.signal(signal)?;
        let width = self
            .state
            .digital
            .read(id)
            .map(FourStateValue::width)
            .unwrap_or(0);
        if width == 0 {
            return Err(MixedSignalError::InvalidBridge {
                detail: format!(
                    "bridge signal `{signal}` carries no bits; a real net is not a logic boundary"
                ),
            });
        }
        if bit >= width {
            return Err(MixedSignalError::InvalidBridge {
                detail: format!(
                    "bridge signal `{signal}` is {width} bits wide and has no bit {bit}"
                ),
            });
        }
        Ok((id, width))
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
}

/// Fold one settle's moved bits into one whole-signal drive per signal.
///
/// `out` is cleared first and left holding the values [`DigitalHost::force_many`]
/// will publish, in the order the first moved bit of each signal was found —
/// which for a scalar-only boundary is bridge order, exactly what it was when
/// every bridge published its own one-bit value.
///
/// Each signal starts from what it holds now rather than from `x`, because the
/// bits this settle did not move have not changed and a whole-signal write
/// would otherwise erase them. A signal the store cannot read is refused here
/// rather than published as a guess: it is the same disappearance `stamp` and
/// [`read_dac_bits`] refuse on.
///
/// [`DigitalHost::force_many`]: super::host::DigitalHost::force_many
fn compose_bit_drives(
    state: &MixedState,
    bit_drives: &[(usize, FourStateBit)],
    out: &mut Vec<(DigitalSignalId, FourStateValue)>,
) -> Result<(), MixedSignalError> {
    out.clear();
    for &(index, value) in bit_drives {
        let bridge = state
            .bridges
            .adc
            .get(index)
            .expect("a moved bit names the bridge it was sampled from");
        let slot = match out.iter_mut().find(|(held, _)| *held == bridge.signal) {
            Some(slot) => slot,
            None => {
                let current = state.digital.read(bridge.signal).cloned().ok_or_else(|| {
                    MixedSignalError::InvalidBridge {
                        detail: format!("A/D signal `{}` disappeared", bridge.signal_name),
                    }
                })?;
                out.push((bridge.signal, current));
                out.last_mut().expect("just pushed")
            }
        };
        slot.1.set_bit(bridge.bit, value);
    }
    Ok(())
}

/// The boundary net one bridge carries, as a diagnostic should spell it.
///
/// A scalar boundary is the signal, because the net and the signal are the
/// same conductor. One bit of a vector is `name[bit]`, in the position
/// numbering the discrete half's own bit selects use — so a reader can take
/// the spelling straight back to the module's source.
fn boundary_net_name(signal: &str, bit: u32, width: u32) -> String {
    if width <= 1 {
        signal.to_string()
    } else {
        format!("{signal}[{bit}]")
    }
}

/// Read every D/A bridge's driven bit into `out`.
///
/// A free function, and taking the state rather than the host, so a caller can
/// hold the scratch vectors apart from the host while it fills one of them.
/// `out` is cleared first, so its allocation survives from call to call.
fn read_dac_bits(state: &MixedState, out: &mut Vec<FourStateBit>) -> Result<(), MixedSignalError> {
    out.clear();
    for bridge in &state.bridges.dac {
        let bit = state
            .digital
            .read(bridge.signal)
            .map(|value| value.bit(bridge.bit))
            .ok_or_else(|| MixedSignalError::InvalidBridge {
                detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
            })?;
        out.push(bit);
    }
    Ok(())
}

/// Every continuous-net probe's differential potential, out of one circuit
/// solution.
///
/// The same arithmetic an A/D bridge samples with, through the same
/// `node_voltage` — a probe and a bridge that name one node must agree about
/// its voltage, and the way to guarantee that is for there to be one function
/// that answers.
fn fill_analog_probes(probes: &[AnalogProbeWiring], circuit_voltages: &[f64], out: &mut Vec<f64>) {
    out.clear();
    out.extend(probes.iter().map(|probe| {
        node_voltage(circuit_voltages, probe.positive)
            - node_voltage(circuit_voltages, probe.negative)
    }));
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
        host.add_adc_bridge("adc", 0, (3, 0), 0.4, 0.6)
            .expect("A/D bridge");
        host.add_dac_bridge("dac", 0, (4, 0), 0.0, 5.0, 100.0)
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

    /// Ticks of `MIXED`'s `#2`, which is what makes the published slot
    /// observable from outside the host.
    const EDGE_PROCESS_DELAY_TICKS: u64 = 2;

    /// Drive one A/D crossing inside a chosen step and report the tick its
    /// transition was published into.
    ///
    /// The tick is read back through the design rather than from a private
    /// field. `MIXED`'s `always @(posedge adc or posedge clk)` block suspends
    /// on `#2` immediately after the edge, so the resume event left behind by
    /// the settle sits exactly two ticks after the slot the transition was
    /// published into, and [`MixedSignalHost::next_event_time`] reports it in
    /// seconds. A publication that moved by a tick moves this by a tick.
    ///
    /// `previous_time` and `previous_voltage` are the accepted timepoint the
    /// crossing is interpolated back towards; `time` and `timestep` are the
    /// step it is interpolated inside. The sense node reaches 1 V, which is
    /// over the bridge's 0.6 V high threshold, so the crossing is at
    /// `time - timestep * (1.0 - 0.6) / (1.0 - previous_voltage)`.
    fn published_tick_of_one_crossing(
        previous_time: f64,
        previous_voltage: f64,
        time: f64,
        timestep: f64,
    ) -> u64 {
        let mut host = host();

        // Time zero resolves the bridge net from `z` to `0`. That is not a
        // posedge, so it wakes nothing and leaves the wheel empty — which is
        // what lets a later resume event be attributed to this crossing alone.
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        assert_eq!(
            host.next_event_time().expect("the wheel is readable"),
            None,
            "resolving the bridge to `0` must not have woken the edge process"
        );

        // The far end of the interval: an accepted timepoint whose sense
        // voltage is still under the high threshold, so the bridge has not
        // moved and this becomes `accepted_adc_voltages`.
        host.begin_trial(
            previous_time,
            previous_time,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .expect("begin the interval's far end");
        settle_and_accept(&mut host, &[0.0, 0.0, previous_voltage, 0.0]);
        assert_eq!(
            host.read_digital("adc").expect("the bridge net exists"),
            "0",
            "the far end of the interval must still be below the threshold"
        );

        // The step the crossing happens inside.
        host.begin_trial(
            time,
            timestep,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .expect("begin the crossing step");
        settle_and_accept(&mut host, &[0.0, 0.0, 1.0, 0.0]);
        assert_eq!(
            host.read_digital("adc").expect("the bridge net exists"),
            "1",
            "the sense node crossed the high threshold, so the bridge must read `1`"
        );

        let resume = host
            .next_event_time()
            .expect("the wheel is readable")
            .expect("the edge process suspended on its `#2`");
        let resume_tick = (resume / 1.0e-9).round() as u64;
        assert!(
            resume_tick >= EDGE_PROCESS_DELAY_TICKS,
            "a `#2` resume cannot be dated before tick {EDGE_PROCESS_DELAY_TICKS}, saw {resume:e} s"
        );
        resume_tick - EDGE_PROCESS_DELAY_TICKS
    }

    /// **Verilog-AMS LRM 2.4 section 7.3.6.1**, upper half: an A/D transition
    /// whose interpolated crossing is past the middle of its tick belongs to
    /// the *next* tick, because that is the nearest one.
    ///
    /// The trial is at 2.9 ns, so the trial's own tick — the floored one, which
    /// bounds how far the digital world may advance — is 2. The crossing is at
    /// 2.86 ns. Flooring the crossing would publish into tick 2 and is the
    /// behaviour this replaces; the nearest tick is 3.
    #[test]
    fn a_crossing_past_the_middle_of_its_tick_publishes_into_the_next_one() {
        assert_eq!(
            published_tick_of_one_crossing(2.8e-9, 0.0, 2.9e-9, 0.1e-9),
            3,
            "a crossing at 2.86 ns is nearest tick 3, not tick 2"
        );
    }

    /// **Section 7.3.6.1**, lower half, and the forward clamp that bounds it.
    ///
    /// The trial is at 3.4 ns, so its own tick is 3. The step reaches back to
    /// 0.9 ns, which puts the crossing at 2.4 ns, whose nearest tick is 2 — a
    /// slot the digital world has already left. Rounding alone would publish
    /// into the past; the clamp against the trial's own tick publishes it here
    /// instead. This is the direction in which nearest and floor must agree,
    /// and the assertion is that they do.
    ///
    /// The trial time is 3.4 ns rather than a round 3.0 ns because a decimal
    /// literal need not sit on the tick grid: the grid is defined by
    /// [`TimeResolution::ticks_to_seconds`], and `3.0e-9` parses a hair *below*
    /// `3.0 * 1e-9`, so it floors to tick 2 and would have made this a test of
    /// the wrong pair of ticks.
    #[test]
    fn a_crossing_before_the_middle_of_its_tick_is_clamped_forward_to_the_trial() {
        assert_eq!(
            published_tick_of_one_crossing(0.9e-9, 0.0, 3.4e-9, 2.5e-9),
            3,
            "a crossing rounding back to tick 2 must be clamped forward to the trial's tick 3"
        );
    }

    /// **Section 7.3.6.1**, the tie. A crossing landing exactly on a half tick
    /// is equidistant from two ticks, and `f64::round` is half-away-from-zero,
    /// so it goes to the later one.
    ///
    /// The step runs from 2.1 ns at 0.2 V to 2.9 ns at 1.0 V, so the 0.6 V
    /// threshold is crossed exactly half way, at 2.5 ns. The direction of the
    /// tie is pinned rather than left to be discovered, because it decides
    /// which slot a dead-centre transition is published into.
    #[test]
    fn a_crossing_exactly_on_the_half_tick_publishes_into_the_later_one() {
        assert_eq!(
            published_tick_of_one_crossing(2.1e-9, 0.2, 2.9e-9, 0.8e-9),
            3,
            "a crossing at exactly 2.5 ns ties, and the tie goes to tick 3"
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
        host.add_dac_bridge("dac", 0, (2, 0), 1.0, 5.0, 100.0)
            .unwrap();
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
        host.add_adc_bridge("seed", 0, (3, 0), 0.4, 0.6).unwrap();
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
    /// digital host is deferred to the first write through a handle the image
    /// still shares.
    #[test]
    fn mixed_trial_copy_ratchet() {
        // A ceiling, not a target. It used to be 41 over these 40 timepoints —
        // one copy of the analog cell per trial, because `begin_trial` always
        // writes the device's time and that write was the first after a
        // capture that shared it, plus one copy of the digital host for the
        // single trial that had an event due. The analog device is no longer
        // captured by a trial at all, so only the digital copies remain and
        // the whole run costs one.
        //
        // Everything after the first write in a trial (every A/D publication,
        // every Newton stamp) goes through a cell no image shares and costs
        // nothing, and the bridge tables are never written during a trial at
        // all.
        //
        // The digital figure is the one to watch: it is small because
        // `begin_trial` asks whether anything is due before taking the
        // mutable view. Drop that predicate and it becomes one per timepoint;
        // put the analog device back in the image and it becomes one more per
        // timepoint on top.
        const TIMEPOINTS: u64 = 40;
        const MAX_COPIES: u64 = 8;

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

    /// A rejected trial leaves the analog device's accepted record untouched.
    ///
    /// This is the property that lets the device sit outside the rollback
    /// image, so it is pinned rather than argued. The trial is driven the whole
    /// way — the timepoint, the timestep, the companion coefficients, a Newton
    /// stamp against a solution that is not the accepted one, and a boundary
    /// settle that publishes an A/D transition — and then rejected. Nothing in
    /// `VerilogADeviceCheckpoint` may move: it carries every accepted variable,
    /// every integration slot, every filter and detector record, and the
    /// `$discontinuity` edge.
    ///
    /// It also pins the digital half, which *is* in the image, so a change that
    /// took the analog device out of the image by taking the digital host out
    /// with it fails here rather than in a benchmark's waveform.
    #[test]
    fn a_rejected_trial_leaves_the_analog_device_accepted_state_untouched() {
        let mut host = host();
        // One accepted timepoint first, so the accepted record under test is a
        // record the device actually wrote rather than its construction state.
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);

        let before_analog = host.analog.checkpoint_state().expect("accepted state");
        let before_digital = host.read_digital("q").expect("q reads");
        let before_time = host.state.accepted_time;
        let before_tick = host.state.accepted_tick;
        let before_voltages = host.state.accepted_adc_voltages.clone();
        let before_probes = host.state.accepted_probe_values.clone();
        let before_transitions = host.state.accepted_adc_transition_times.clone();
        let before_adc_history = host.state.adc_history.clone();
        let before_dac_history = host.state.dac_history.clone();

        // A trial that moves as much as a trial can. Node 3 — the third matrix
        // row, so the third entry — is over the A/D bridge's high threshold,
        // so the settle publishes a transition and runs the digital slot the
        // rising edge wakes.
        const DURING: [f64; 4] = [0.5, 0.0, 1.0, 0.0];
        begin(&mut host, 4);
        assert!(host.settle_analog_bridges(&DURING).is_ok());
        host.stamp(&DURING, |_, _, _| {}, |_, _| {})
            .expect("stamp during the trial");
        assert_ne!(
            host.read_digital("q").expect("q reads"),
            before_digital,
            "the trial has to move the digital half, or this pins nothing"
        );
        host.reject_trial().expect("reject");

        assert_eq!(
            host.analog.checkpoint_state().expect("accepted state"),
            before_analog,
            "a rejected trial moved the analog device's accepted record. That record is what \
             `MixedSignalHost::analog` promises a trial cannot reach without `accept_trial`, and \
             the device sits outside the rollback image on the strength of it"
        );
        assert_eq!(host.read_digital("q").expect("q reads"), before_digital);
        assert_eq!(host.state.accepted_time, before_time);
        assert_eq!(host.state.accepted_tick, before_tick);
        assert_eq!(host.state.accepted_adc_voltages, before_voltages);
        assert_eq!(host.state.accepted_probe_values, before_probes);
        assert_eq!(host.state.accepted_adc_transition_times, before_transitions);
        assert_eq!(host.state.adc_history, before_adc_history);
        assert_eq!(host.state.dac_history, before_dac_history);
    }
}
