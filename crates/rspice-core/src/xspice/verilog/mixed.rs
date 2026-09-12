//! Analog equations and electrical boundaries for a mixed Verilog-AMS module.
//!
//! A standalone host owns its digital runtime. Once enrolled in a circuit it
//! holds a signal view, and the circuit's shared runtime owns all HDL processes,
//! drivers and event queues. The analog solver evaluates transactional trials;
//! the circuit settles every participant before stamping or accepting a trial.
//!
//! Pure HDL port connections share resolved event bits in that runtime. Ports
//! attached to continuous devices retain their A/D or D/A electrical bridges;
//! a vector may have both kinds of bit connection.
//!
//! # The three time bases
//!
//! The analog side names a timepoint in seconds, chosen by a step controller
//! answering to local truncation error, and it lands wherever it lands. The
//! digital side counts ticks of a declared precision. A third kernel — XSPICE
//! — names its own event times in seconds again. Three different questions
//! cross between them, and they do not quantize the same way. Conflating them
//! is the mistake this section exists to prevent. **This is the one place all
//! three are stated; a call site states which of them it is using and why, and
//! never invents a fourth base.**
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
//! *Which tick does a wake from the other event kernel land on?* is answered
//! by the **ceiling** —
//! [`TimeResolution::seconds_to_ceil_ticks`](crate::xspice::event_scheduler::TimeResolution::seconds_to_ceil_ticks).
//! This one is not an analog event and the LRM's nearest-tick rule does not
//! reach it: XSPICE and the HDL are two discrete kernels on two time bases,
//! and the two directions of that boundary are the two halves of *one* map.
//! HDL to XSPICE is exact — an HDL tick `T` is delivered at precisely the
//! instant `T` names, which is what `external_models.rs` stamps. The inverse
//! of an exact map is "not before": an off-grid XSPICE instant is delivered at
//! the least tick whose own instant is not before it, so the HDL never labels
//! a foreign-kernel event earlier than it happened. Exact one way,
//! least-tick-not-before the other.
//!
//! A fourth question is not a fourth base. It asks which of the answers above
//! a particular crossing gets, and it is written here because the answer is
//! only correct when read beside them.
//!
//! *Which instant does a crossing the digital half caused get?* **The trial's
//! endpoint**, and its tick is that endpoint's floored tick — the first
//! question's answer rather than the second's. A step in which the discrete
//! half moved something the analog equations read — a D/A bridge's level, or
//! a discrete variable an analog block references — is a step whose continuous
//! problem changed inside it. A threshold an A/D input crosses on the re-solve
//! was carried across *by that movement*, at the instant the movement
//! happened; it is not a root the interval's own solution passed through,
//! because no single solution of this interval ever passed through it.
//! Interpolating it across the whole step dates it wherever two iterates
//! happen to differ, which can be *before* the edge that caused it — an effect
//! preceding its cause, and a `$abstime` no physics produced. So it is dated
//! where its cause is, and [`MixedSignalHost::trial_boundary_refinement_time`]
//! asks the controller for no interior root on such a step. A crossing with no
//! such feedback before it in the same trial is the circuit's own: it keeps
//! the interpolated instant and the nearest-tick rule above, because dating it
//! at the endpoint would be a setup/hold-class error on a sampling edge.
//!
//! Nearest-tick here would not violate any standard, and this document does
//! not claim it would. A `#1` from a process dated at tick `T` fires at
//! `T + 1` — one whole time unit later in the digital base, whatever the
//! physical instant that woke it — and measuring that delay against the *other*
//! kernel's clock instead is exactly what the LRM's own nearest-tick A/D rule
//! does by design one paragraph above, where a crossing at `T·Δ + 0.49Δ` is
//! published at `T` and its `#1` elapses `0.51Δ` after the crossing. The
//! ceiling is an engineering choice for a boundary whose far side names exact
//! seconds and has no declared precision to round to: it keeps the two halves
//! of the map inverse to each other, and keeps `$realtime` from naming an
//! instant before the event that caused the wake.
//!
//! "Exact" in the HDL-to-XSPICE direction is exact for an event the wheel
//! itself timed — the stepper lands on a scheduled tick bit-exactly, through a
//! breakpoint. An HDL edge that a *mid-step* A/D crossing caused reaches
//! XSPICE at the trial's endpoint instead, because the continuous half has a
//! solution only there and `XspiceDigitalParticipant::settle_active` refuses a
//! wave assembled at any other instant.
//!
//! Either way the unquantized analog time is kept for everything that is
//! answered in seconds: the trial's own bookkeeping, the interpolated instant
//! an A/D bridge crossed its threshold — which is also the `$abstime` every
//! process woken by that crossing reads — and the breakpoint
//! [`MixedSignalHost::next_event_time`] hands back. The one quantity that
//! stays at the trial's endpoint is the *analog candidate instant*, because
//! the continuous half has a solution only there; `DigitalHost`'s
//! `force_many_from_analog_at` is where the two are carried apart.
//!
//! Several analog timepoints therefore share one tick, which is what a declared
//! precision means, and the host's monotonicity is enforced on the *analog*
//! time rather than on the tick: a repeat of an accepted timepoint would
//! advance the integrator twice.
//!
//! The RSpice guarantee those two quantities together amount to: **in a
//! process woken by a boundary event — an A/D crossing, or a wake from the
//! other event kernel — `$abstime` is that event's own physical instant, and
//! it need not equal `$realtime` scaled by the time unit.** The tick answers
//! "where on the wheel am I", `$abstime` answers "when did the cause happen",
//! and at a sub-tick instant only one of them can be exact. They are reported
//! apart rather than one being rounded onto the other. What is guaranteed
//! across them is the ordering: within one trial the published tick never
//! decreases, and nothing a woken process schedules can come due at a physical
//! time before the instant that woke it.
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

mod analog_samples;
mod shared;
use analog_samples::{AnalogModelParticipant, PreparedAnalogStamp};
use shared::MixedDigital;
pub(crate) use shared::{MixedDigitalCoordinator, SharedDigitalTrial};

use std::fmt;
use std::sync::Arc;

use rspice_veriloga::canonical_ir::VectorBounds;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::{DigitalAnalogProbeId, DigitalSignalId};
use rspice_veriloga::device::{
    VerilogADevice, VerilogADeviceCheckpoint, VerilogAEvaluationSnapshot,
};
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::vm::IntegrationCoefficients;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

use super::host::DigitalHost;
use super::{DigitalRunError, parse_four_state};
use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};
use crate::xspice::settle_cost;
use crate::xspice::threshold_crossing::threshold_crossing_time;

/// Which half of a boundary a reported bit came from.
///
/// A deck node can carry both: one module drives it through a D/A bridge while
/// another samples it through an A/D bridge. The two bits are then answers to
/// different questions — what was put on the net, and what a particular reader
/// made of the voltage that resulted — and they disagree over the interval the
/// analog node spends crossing that reader's threshold. Anything that has to
/// publish *the* value of the net resolves that with this, and the driver wins:
/// the net's value is what its driver put there.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BoundaryBitSource {
    /// A D/A bridge: what the module drives onto the deck node.
    Driven,
    /// An A/D bridge: what the module read off the deck node.
    Sampled,
}

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
/// # The one move this count does not take
///
/// A move the discrete half's *own* schedule explains is not evidence of that
/// loop, and is not counted: see [`BoundaryMove::Scheduled`]. The premise above
/// — a resolved waveform moves a boundary net once in several accepted points —
/// holds only while the stepper has points to spare between two activations. A
/// clock whose period is a few minimum steps does not leave it any: the step
/// after a landed activation restarts at a tenth of the gap to the next one
/// (`transient::breakpoint`), the floor clamps that back up, and the stepper
/// then lands on every tick and on nothing else. Such a clock moved its D/A
/// bridge at every accepted point and was refused here as a zero-delay loop —
/// a 10 fs `always #0.01` toggle under a millisecond maximum step at point 129,
/// with the analog side entirely passive. The schedule is the fact there, and
/// the bound that belongs to a schedule is the transient stepper's own
/// (`sub_minimum_activation_check`), which measures the rate the run advances
/// at rather than the boundary.
///
/// The exchange is deliberate: feedback through a module that *does* delay its
/// reaction — `always @(c) #1 y = ~c` — is now bounded by that schedule guard
/// rather than by this one, and this one keeps the loop it was written for, the
/// one with no delay anywhere in it.
///
/// 128 rather than a smaller number because the cost of being wrong is
/// asymmetric: a false positive refuses a deck that would have run, and a false
/// negative costs the extra timepoints it takes to reach the ceiling.
const MAX_CONSECUTIVE_BOUNDARY_FLIPS: u32 = 128;

/// How many of a boundary net's most recent values a diagnostic carries.
///
/// Eight, because the evidence a reader needs from a chattering net is the
/// *pattern* — an alternation says feedback, a run of one value says the count
/// is measuring something else — and eight values show a period-two or
/// period-four alternation unambiguously. They are carried as two bits each in
/// a `u16` so an accepted timepoint costs a shift rather than an allocation.
const BOUNDARY_VALUE_HISTORY: u32 = 8;

/// What one recorded value did to a boundary net, as the flip run counts it.
///
/// The three cases are what separate the loop [`MAX_CONSECUTIVE_BOUNDARY_FLIPS`]
/// is about from a schedule that merely runs faster than the stepper can spare
/// points for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundaryMove {
    /// The net holds the value the previous record left it at. The run ends:
    /// an interval the solver resolved without the boundary moving is exactly
    /// what a loop never produces.
    Still,
    /// The net moved at a timepoint the discrete half had an activation of its
    /// own due at — the schedule's own tick, and the module's own doing.
    ///
    /// The run neither grows nor restarts. It does not grow because the move
    /// is explained: a process woke on its own delay and assigned the net, and
    /// no analog solution took part in deciding that it should. It does not
    /// restart either, because a scheduled activation is no evidence that a
    /// loop underneath it has stopped — a design with both keeps whatever run
    /// its unexplained moves have built, and is still refused at the ceiling.
    Scheduled,
    /// The net moved with nothing in the discrete half's queue to explain it,
    /// so the only thing that can have moved it is the analog solution at this
    /// timepoint. This is the move the ceiling counts.
    Unexplained,
}

/// One boundary net's retained values and consecutive movement count.
/// Accepted histories record every timepoint; diagnostic probe histories
/// record only changes and are stored separately from accepted state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BoundaryNetHistory {
    /// Consecutive recorded values that moved this net with nothing scheduled
    /// to explain the move.
    run: u32,
    /// The last [`BOUNDARY_VALUE_HISTORY`] recorded values, two bits each,
    /// most recent in the low bits.
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

    fn push(&mut self, bit: FourStateBit, move_kind: BoundaryMove) {
        self.recent = (self.recent << 2) | Self::code(bit);
        self.filled = self.filled.saturating_add(1).min(BOUNDARY_VALUE_HISTORY);
        self.run = match move_kind {
            BoundaryMove::Still => 0,
            BoundaryMove::Scheduled => self.run,
            BoundaryMove::Unexplained => self.run.saturating_add(1),
        };
    }

    /// The retained values, oldest first.
    fn values(&self) -> Vec<String> {
        (0..self.filled)
            .rev()
            .map(|slot| Self::spelling((self.recent >> (2 * slot)) & 0b11).to_string())
            .collect()
    }
}

/// Diagnostic-only transitions from completed, rejected solver probes.
/// Retain changes so a failed damping attempt's repeated identical samples
/// cannot erase earlier switching. Acceptance and checkpoints exclude this.
#[derive(Clone, Default)]
struct BoundaryProbeHistory {
    time: Option<f64>,
    probes: usize,
    adc: Vec<BoundaryNetHistory>,
    dac: Vec<BoundaryNetHistory>,
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
    /// accepted-flip run, settle passes within one trial for a pass limit, or
    /// transitions in the retained rejected-probe samples.
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
    /// The analog half evaluated to NaN or infinity at the trial point the
    /// solver handed it. Rejectable: the Newton loop cuts dt or steps the
    /// sources, exactly as it does for a plain analog Verilog-A instance.
    AnalogNonFinite { detail: String },
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
            Self::AnalogNonFinite { detail } => write!(
                f,
                "mixed Verilog analog half produced a non-finite value at a trial iterate: {detail}"
            ),
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
/// counts from the least significant end, which is where the discrete half's
/// own bit selects are resolved to — not the index the module names the bit
/// by, which on a `[7:4]` net is a different number and is what
/// [`AdcBridge::signal_name`] carries.
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

impl AdcBridge {
    /// A zero-width threshold belongs to the side the signal is entering. At
    /// a stationary threshold retain the resolved level (zero at startup).
    /// This prevents a rising root from being postponed by the low-first test.
    fn decision(
        &self,
        voltage: f64,
        previous: Option<f64>,
        held: Option<FourStateBit>,
    ) -> Option<(FourStateBit, f64)> {
        if self.low == self.high && voltage == self.low {
            let bit = if previous.is_some_and(|previous| previous < voltage) {
                FourStateBit::One
            } else if previous.is_some_and(|previous| previous > voltage) {
                FourStateBit::Zero
            } else if held == Some(FourStateBit::One) {
                FourStateBit::One
            } else {
                FourStateBit::Zero
            };
            Some((bit, self.low))
        } else if voltage <= self.low {
            Some((FourStateBit::Zero, self.low))
        } else if voltage >= self.high {
            Some((FourStateBit::One, self.high))
        } else {
            None
        }
    }
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

    fn driven_level(&self, bit: FourStateBit) -> Option<f64> {
        match bit {
            FourStateBit::Zero => Some(self.low),
            FourStateBit::One => Some(self.high),
            FourStateBit::Unknown => Some(self.undefined_level()),
            FourStateBit::HighImpedance => None,
        }
    }
}

/// One continuous-net probe of Verilog-AMS LRM 2.4 section 7.3.3, wired to the
/// circuit.
///
/// `positive` and `negative` are one-based solution indices; `0` is the
/// reference. A potential uses a node difference. A flow uses its current
/// unknown against the reference, with `scale` preserving authored direction.
///
/// The plan names the probe's nets by the author's own identifiers, because the
/// analog levels and the discrete plan are lowered by two passes that share no
/// numbering. Resolving those names to circuit nodes is this table, built once
/// when the module is wired and never touched again — which is why it lives
/// beside the bridge declarations rather than in the rollback image.
#[derive(Clone)]
enum AnalogProbeWiring {
    Solution {
        positive: usize,
        negative: usize,
        scale: f64,
    },
    Variable {
        name: String,
    },
}

impl AnalogProbeWiring {
    fn sample(&self, solution: &[f64]) -> Option<f64> {
        match self {
            Self::Solution {
                positive,
                negative,
                scale,
            } => Some(
                scale * (node_voltage(solution, *positive) - node_voltage(solution, *negative)),
            ),
            Self::Variable { .. } => None,
        }
    }
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

/// One vector boundary port, as a bus over the deck nodes its bits landed on.
///
/// Declared where the module is wired, because that is the only place that
/// knows both halves of it: the range the module's author wrote, and the deck
/// nodes the X-card named for its bits. Everything downstream sees node ids
/// and would have to guess which of them were one word.
///
/// The engine turns this into a `DigitalBusDeclaration` when a run starts, by
/// naming the nodes; it is not that type here because a boundary sits eleven
/// layers below the result types and may not reach up for one.
#[derive(Clone, Debug)]
pub(crate) struct BoundaryBus {
    /// `<instance>.<port>` — the deck's name for the instance and the module's
    /// for the port, which is the only pair that is unique across a deck.
    pub(crate) name: String,
    /// Declared most significant index, exactly as the module wrote it.
    pub(crate) msb: i64,
    /// Declared least significant index, exactly as the module wrote it.
    pub(crate) lsb: i64,
    /// Circuit-node ids of the members, declared MSB first.
    pub(crate) members: Vec<usize>,
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
    phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
    initial_step: bool,
    final_step: bool,
    time_seconds: f64,
    timestep_seconds: f64,
    integration: IntegrationCoefficients,
    state_integration: IntegrationCoefficients,
}

impl AnalogSolverInputs {
    /// What [`VerilogADevice::try_begin_analysis`] leaves a fresh device
    /// holding: `VmContext::reset_analysis_state` zeroes the time and the
    /// timestep and deactivates the integration coefficients, and neither
    /// analysis-step flag is set until a trial sets one.
    const fn analysis_start(analysis: u8) -> Self {
        Self {
            analysis,
            phase: rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
            initial_step: false,
            final_step: false,
            time_seconds: 0.0,
            timestep_seconds: 0.0,
            integration: IntegrationCoefficients::inactive(),
            state_integration: IntegrationCoefficients::inactive(),
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
    probes: Vec<Option<f64>>,
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
    probe_history: BoundaryProbeHistory,
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
    /// Ordinary published variables and reporting scalars need their pre-trial
    /// image for exact readback and checkpoint rollback.
    analog_evaluation: VerilogAEvaluationSnapshot,
    discrete_inputs: Vec<f64>,
    transition_times: Vec<Option<f64>>,
    sampled_adc_voltages: Vec<f64>,
    probe_values: Vec<Option<f64>>,
    adc_moved: Vec<bool>,
    dac_moved: Vec<bool>,
    /// Every D/A bridge's bit as the trial opened, before the discrete half
    /// was given the chance to run. Parallel to `bridges.dac`.
    ///
    /// The trial's own `dac_moved` is written at the *end* of a settle pass,
    /// which is after that pass has already had to decide how to date its A/D
    /// crossings. This is the same question asked early enough to answer that.
    dac_at_trial_start: Vec<FourStateBit>,
    /// Every discrete quantity an analog block reads, as the *store* held it
    /// when the trial opened. Parallel to `MixedSignalHost::discrete_inputs`.
    ///
    /// The other half of the same early question, for the other half of the
    /// digital-to-analog path: a variable the discrete half writes and the
    /// analog equations read moves the continuous problem exactly as a D/A
    /// bridge's level does, and moves it without any bridge bit changing.
    ///
    /// Read from the store rather than from the analog device, so what it
    /// answers is "did the *digital* world move this". The device's own copy
    /// (`discrete_inputs`, kept for rollback) is written by
    /// [`MixedSignalHost::sample_discrete_inputs`] at the end of a settle
    /// pass, which is again too late to date that pass's crossings with.
    discrete_at_trial_start: Vec<Option<f64>>,
}

/// Everything a rejected trial has to put back.
///
/// The analog device is deliberately not in here; [`MixedSignalHost::analog`]
/// says why.
#[derive(Clone)]
struct MixedState {
    digital: MixedCell<MixedDigital>,
    initial_digital: Option<MixedCell<MixedDigital>>,
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
    /// Scheduled activations sample the trial's candidate solution; this bank
    /// provides accepted history and the initial values of explicit host drives.
    accepted_probe_values: Vec<Option<f64>>,
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
    start_digital: bool,
    /// Whether the discrete half moved something the analog equations read,
    /// anywhere in the circuit, inside this trial — a D/A bridge's bit or a
    /// discrete variable an analog block references, this instance's own or
    /// another enrolled instance's as the coordinator reports it.
    ///
    /// Such a movement steps the analog solution at the instant it happened.
    /// An A/D threshold the step then crosses was crossed *by that event*, at
    /// the tick it happened on, and is not a root the analog solver can go
    /// back and find inside the interval: no solution of that interval ever
    /// passed through the threshold. So the crossing is dated at the trial's
    /// endpoint and no refinement is asked for — the fourth question of this
    /// module's "three time bases" carries the argument in full.
    ///
    /// The predicate is the movement of a quantity the *analog* half reads,
    /// not "the discrete half ran". A divider, a counter or a state machine
    /// ticking inside this interval whose state no analog equation reads moves
    /// nothing continuous, and an analog input that crosses a threshold while
    /// one of them ticks was moved by the circuit — it keeps its interpolated
    /// crossing and its refinement, because dating it at the endpoint would be
    /// a setup/hold-class error on a sampling edge.
    ///
    /// What remains approximate: an A/D crossing the circuit causes in the
    /// same interval as such a movement is still dated at that movement's
    /// tick. Both events are inside one step and no second solve can separate
    /// them, because re-solving lands on the movement again.
    digital_feedback: bool,
    /// Whether the discrete half had an activation of its own due at or before
    /// this trial's tick when the trial opened.
    ///
    /// Read before the queue is drained, because draining it is what consumes
    /// the evidence: afterwards every activation this timepoint ran has left
    /// the wheel and the module looks as idle as a comparator that only ever
    /// reacts to its input. A boundary this trial moves is then the schedule's
    /// own move rather than the analog solution's, which is the one thing
    /// [`MAX_CONSECUTIVE_BOUNDARY_FLIPS`] must not count.
    ///
    /// An enrolled instance schedules into the circuit's shared queue and
    /// cannot see it from here, so for one of those this opens `false` and the
    /// coordinator reports the queue's own answer through
    /// [`MixedSignalHost::note_scheduled_activation`] — the same shape
    /// `digital_feedback` is given by
    /// [`MixedSignalHost::note_shared_digital_feedback`], and for the same
    /// reason.
    scheduled_activation: bool,
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
    rollback: MixedCell<MixedDigital>,
    /// The analog device's solver inputs as they stood when the trial opened.
    ///
    /// The device's *accepted* record needs no image, but these five inputs do:
    /// they are what a device carries between evaluations, and one of them —
    /// the timepoint — is encoded into a checkpoint. A rejected trial that left
    /// its own timepoint standing would make a restart image taken afterwards
    /// name a time the run never accepted.
    analog_inputs: AnalogSolverInputs,
    tick: u64,
    /// The highest tick this trial has published an A/D transition at.
    ///
    /// A crossing is dated by interpolating between the accepted solution and
    /// the candidate one, and every settle pass of a trial is a Newton
    /// iteration against a *different* candidate over that same interval: a
    /// root one of them finds can sit anywhere inside it, including before a
    /// crossing an earlier pass published and the discrete half has already
    /// run on. Dating that root on its own would set the store's clock back
    /// inside one trial — `$realtime` running backwards, an effect dated
    /// before the cause it followed — so its tick is clamped forward onto this
    /// mark, exactly as an interior crossing is clamped forward onto the
    /// trial's own tick. The instant is untouched either way: `$abstime` still
    /// reports where the root is.
    ///
    /// Trial-scoped, because a rejected trial takes its publications back with
    /// it, and an accepted one leaves the next trial a floored tick that is
    /// never below this mark — every crossing is interpolated inside
    /// `[time - dt, time]` and rounds no further than `ceil(time)`.
    published_tick: u64,
    time_seconds: f64,
    timestep_seconds: f64,
    /// Whether this trial was opened as a probe by
    /// [`MixedSignalHost::begin_trial_with_integration_rules`] and may never be committed.
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
    /// One normal analog evaluation shared by variable sampling and stamping.
    /// Candidate-only storage; every trial boundary invalidates its contents.
    prepared_analog: PreparedAnalogStamp,
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
    /// Stateful analog operators evaluate their candidates from
    /// its own *committed* record — `filters::…::candidate_evaluation` opens
    /// with a clone of `self.committed`, the integration slots read
    /// `state_values_prev`, and `VmContext::apply_validated_advance_state` is
    /// the only writer that promotes a candidate into the accepted record. A
    /// trial reaches that promotion only through
    /// [`Self::accept_trial`], so a trial that is rejected leaves the accepted
    /// state bit-identical and leaves behind only candidate state that the
    /// next evaluation recomputes from the same accepted record. The small
    /// bank of externally supplied discrete variables is restored separately
    /// from `TrialVectors::discrete_inputs`, alongside scalar solver inputs.
    /// Ordinary evaluation variables also need their pre-trial scalar image:
    /// they are observable in readback/checkpoints even when they carry no
    /// accepted operator history. `TrialVectors::analog_evaluation` restores
    /// that bank without cloning the device or replaying its equations.
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
    /// Digital processes start only after the circuit-wide analog initialization
    /// barrier. This is separate from the first accepted boundary trial.
    digital_started: bool,
    trial: Option<ActiveTrial>,
    /// Every continuous-net probe the discrete half declares, resolved to
    /// circuit nodes. Empty for a module whose processes read no analog value,
    /// which is what makes the whole cross-domain read path cost such a module
    /// nothing.
    analog_probes: Vec<AnalogProbeWiring>,
    discrete_inputs: Vec<DiscreteAnalogInput>,
    /// Every vector boundary port, as a bus over the deck nodes its bits
    /// landed on. Empty for a module whose discrete ports are all scalar,
    /// which is every module this route carried before vectors were bridged.
    ///
    /// Wiring data, not running state: it is fixed before the first trial and
    /// never read by one, so it sits beside the probe table rather than inside
    /// [`MixedState`].
    boundary_buses: Vec<BoundaryBus>,
    event_nodes: Vec<usize>,
    max_circuit_node: usize,
    max_bridge_iterations: u32,
    /// Smallest interval the analog solver is allowed to advance by, or zero
    /// when nothing has declared one. See [`Self::set_analog_step_floor`].
    analog_step_floor: f64,
}

#[derive(Clone)]
struct DiscreteAnalogInput {
    signal: DigitalSignalId,
    variable: usize,
    signed: bool,
    real: bool,
    name: String,
}

/// An exclusive reservation of one validated mixed candidate. A failed later
/// participant drops earlier reservations and unwinds their speculative state.
pub(crate) struct PreparedMixedAcceptance<'a> {
    host: &'a mut MixedSignalHost,
    trial: Option<ActiveTrial>,
}

impl PreparedMixedAcceptance<'_> {
    pub(crate) fn commit(mut self) {
        let trial = self.trial.take().expect("unconsumed mixed acceptance");
        self.host.apply_prepared_acceptance(trial);
    }
}

impl Drop for PreparedMixedAcceptance<'_> {
    fn drop(&mut self) {
        if let Some(trial) = self.trial.take() {
            self.host.unwind(trial);
        }
    }
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
    /// Compile and start one module with a discrete execution plan. Its analog
    /// component may contain equations, procedural effects, or no work.
    /// `terminal_nodes` maps analog ports to the
    /// outer solver's circuit-node ids, where `0` is ground.
    /// An analog initialization control request is left for the caller to
    /// consume and prevents digital startup.
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
        let mut host = Self::from_compiled(
            instance,
            Arc::new(runtime.model),
            &runtime.canonical_ir,
            terminal_nodes,
            scheduler_limits,
            &rspice_veriloga::NoPipelineControl,
        )?;
        host.begin_analog_analysis(2)?;
        if !host.analog.has_accepted_analog_tasks() {
            host.start_digital_execution()?;
        }
        Ok(host)
    }

    /// Construct one module without executing analog initializers or digital
    /// processes. The engine starts them at the analysis initialization barrier.
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
        control: &dyn rspice_veriloga::PipelineControl,
    ) -> Result<Self, MixedSignalError> {
        Self::from_compiled_with_analog_setup(
            instance,
            model,
            canonical_ir,
            terminal_nodes,
            &[],
            scheduler_limits,
            Default::default(),
            control,
            &mut |device| {
                if device.num_internal_nodes() != 0 || device.num_branch_unknowns() != 0 {
                    Err("mixed model requires an owning circuit to allocate internal-node or branch-current solver indices".into())
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Bind internal-node and branch-current indices before cross-domain
    /// probes are resolved. The owning solver allocates these unknowns.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_compiled_with_analog_setup(
        instance: &str,
        model: Arc<rspice_veriloga::CompiledModel>,
        canonical_ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
        terminal_nodes: &[usize],
        parameters: &[(&str, f64)],
        scheduler_limits: SchedulerLimits,
        simulation_parameters: rspice_veriloga_runtime::GeneratedSimulationParameters,
        control: &dyn rspice_veriloga::PipelineControl,
        setup: &mut dyn FnMut(&mut VerilogADevice) -> Result<(), String>,
    ) -> Result<Self, MixedSignalError> {
        if canonical_ir.digital.is_empty() {
            return Err(MixedSignalError::Compile {
                detail: format!(
                    "module `{}` has no digital processes or drivers for the mixed host",
                    canonical_ir.mir.module_name
                ),
            });
        }

        // The same backend selection the device builder makes. A mixed module
        // is one more Verilog-A instance as far as the continuous half is
        // concerned, so it must not reach a different runtime than the analog
        // instance beside it would.
        let analog = VerilogADevice::try_new_with_simulation_parameters_and_control(
            instance,
            model,
            Some(canonical_ir),
            terminal_nodes,
            parameters,
            simulation_parameters,
            control,
        );
        let mut analog = analog.map_err(|error| MixedSignalError::Compile {
            detail: format!("analog device construction failed: {error}"),
        })?;
        setup(&mut analog).map_err(|detail| MixedSignalError::Compile { detail })?;
        if (0..analog.num_internal_nodes()).any(|index| {
            analog
                .internal_node_index(index)
                .is_none_or(|node| node == 0)
        }) || (0..analog.num_branch_unknowns()).any(|index| {
            analog
                .branch_current_index(index)
                .is_none_or(|node| node == 0)
        }) {
            return Err(MixedSignalError::Compile {
                detail:
                    "mixed model topology contains unbound internal-node or branch-current indices"
                        .into(),
            });
        }

        let analog_probes = wire_analog_probes(canonical_ir, &analog)?;
        let discrete_inputs: Vec<_> = canonical_ir
            .hir
            .variables
            .iter()
            .filter(|variable| variable.is_state)
            .filter_map(|variable| {
                canonical_ir
                    .digital
                    .signals
                    .iter()
                    .find(|signal| signal.name == variable.name)
                    .map(|signal| DiscreteAnalogInput {
                        signal: signal.id,
                        variable: usize::from(variable.id),
                        signed: signal.integer,
                        real: signal.kind.is_real(),
                        name: signal.name.to_string(),
                    })
            })
            .collect();
        let max_circuit_node = analog_solver_nodes(&analog).max().unwrap_or(0);

        let resolution = TimeResolution::new(canonical_ir.digital.timing.precision_exponent)
            .map_err(DigitalRunError::from)?;
        let max_bridge_iterations = scheduler_limits.max_delta_cycles_per_tick.max(1);
        let mut digital = DigitalHost::new(&canonical_ir.digital, resolution, scheduler_limits);
        let dependencies: Vec<_> = analog_probes
            .iter()
            .enumerate()
            .filter(|(_, probe)| matches!(probe, AnalogProbeWiring::Variable { .. }))
            .map(|(index, _)| {
                (
                    DigitalAnalogProbeId::from(index),
                    discrete_inputs.iter().map(|input| input.signal).collect(),
                )
            })
            .collect();
        digital.bind_analog_variable_inputs(&dependencies)?;
        // Physical probes start at the unsolved zero vector; retained variable
        // samples remain unavailable until normal analog evaluation publishes
        // them. Candidate-driven activation refreshes this bank before running
        // any process that reads an analog value.
        let initial_probe_values = analog_probes
            .iter()
            .map(|probe| probe.sample(&[]))
            .collect::<Vec<_>>();
        digital.sample_analog_probes(&initial_probe_values);
        let source_digest = canonical_ir.metadata.source_digest.to_string();
        Ok(Self {
            prepared_analog: PreparedAnalogStamp::default(),
            instance: instance.to_string(),
            source_digest,
            resolution,
            analog: MixedCell::new(analog),
            analog_inputs: AnalogSolverInputs::analysis_start(2),
            state: MixedState {
                digital: MixedCell::new(MixedDigital::Owned(Box::new(digital))),
                initial_digital: None,
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
            digital_started: false,
            trial: None,
            analog_probes,
            discrete_inputs,
            boundary_buses: Vec::new(),
            event_nodes: Vec::new(),
            max_circuit_node,
            max_bridge_iterations,
            analog_step_floor: 0.0,
        })
    }

    /// Reset both domains for a fresh analysis, retaining compiled code and
    /// bridge wiring. CircuitData stages hosts before publishing this reset.
    pub(crate) fn begin_analog_analysis(&mut self, analysis: u8) -> Result<(), MixedSignalError> {
        self.begin_analog_analysis_in_phase(
            analysis,
            rspice_veriloga_runtime::AnalogAnalysisPhase::Point,
        )
    }

    pub(crate) fn begin_analog_analysis_in_phase(
        &mut self,
        analysis: u8,
        phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("begin an analysis")?;
        self.prepared_analog.invalidate();
        self.analog
            .make_mut()
            .try_begin_analysis_in_phase(analysis, phase)
            .map_err(analog_error)?;
        self.analog_inputs = AnalogSolverInputs::analysis_start(analysis);
        self.analog_inputs.phase = phase;
        self.state.digital = MixedCell::new(self.state.digital.fresh());
        self.state.initial_digital = None;
        self.state.accepted_adc_voltages.fill(0.0);
        self.state.accepted_adc_transition_times.fill(None);
        for (slot, probe) in self
            .state
            .accepted_probe_values
            .iter_mut()
            .zip(&self.analog_probes)
        {
            *slot = probe.sample(&[]);
        }
        self.state.adc_history.fill(BoundaryNetHistory::default());
        self.state.dac_history.fill(BoundaryNetHistory::default());
        self.state.accepted_tick = 0;
        self.state.accepted_time = 0.0;
        self.state.started = false;
        self.digital_started = false;
        self.scratch.probe_history.time = None;
        Ok(())
    }

    pub(crate) fn set_temperature(&mut self, temperature: f64) -> Result<(), MixedSignalError> {
        self.require_idle("configure temperature")?;
        self.prepared_analog.invalidate();
        self.analog
            .make_mut()
            .try_set_temperature(temperature)
            .map_err(analog_error)
    }

    /// Set the instance multiplicity (`m=` / `$mfactor`) of the module's
    /// continuous half: it stamps as m parallel copies, which is the same
    /// factor a plain `VerilogADevice` beside it applies, applied by the same
    /// device method. Every stamping entry — [`Self::stamp`], the prepared
    /// path it takes when a process reads an analog variable, and
    /// [`Self::stamp_static_dae`] — reaches the analog half through that
    /// device, so there is one place to scale and this is it.
    ///
    /// What it does *not* scale is the boundary. The discrete half has no
    /// multiplicity to begin with, and neither do the D/A bridges: a bridge
    /// here is this route's stand-in for the connect module clause 7 would
    /// insert on the net, built from the deck's supply and the XSPICE bridge
    /// library's source resistance — the very numbers an explicit `d2a`
    /// A-card instance beside it would use. That instance would not inherit
    /// an X-card's `m`, and the analog route has no bridge to scale at all,
    /// so scaling one here would be a rule the plain Verilog-A carve-out does
    /// not have. It would also change only how stiffly the boundary is driven
    /// rather than what it drives, which is not a module contribution.
    pub(crate) fn set_multiplicity(&mut self, multiplicity: f64) -> Result<(), MixedSignalError> {
        self.require_idle("configure multiplicity")?;
        self.prepared_analog.invalidate();
        self.analog
            .make_mut()
            .try_set_multiplicity(multiplicity)
            .map_err(analog_error)
    }

    pub(crate) fn visit_equation_abstols(
        &self,
        current_abstol: f64,
        visit: impl FnMut(usize, f64),
    ) {
        self.analog.visit_equation_abstols(current_abstol, visit);
    }

    pub(crate) fn one_step_dae_split_safe(&self) -> bool {
        self.analog.one_step_dae_split_safe()
    }

    pub(crate) fn requires_nodeset_phase(&self) -> bool {
        self.analog.requires_nodeset_phase()
    }

    pub(crate) fn set_analog_analysis_phase(
        &mut self,
        phase: rspice_veriloga_runtime::AnalogAnalysisPhase,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("configure analysis phase")?;
        self.prepared_analog.invalidate();
        self.analog
            .make_mut()
            .try_set_analysis_phase(phase)
            .map_err(analog_error)?;
        self.analog_inputs.phase = phase;
        Ok(())
    }

    /// Start digital processes after every analog initializer has completed and
    /// its accepted control calls have been handled by the analysis host.
    pub(crate) fn start_digital_execution(&mut self) -> Result<(), MixedSignalError> {
        self.require_idle("start digital execution")?;
        if self.analog.has_accepted_analog_tasks() {
            return Err(MixedSignalError::TrialProtocol {
                detail: "analog initialization tasks must be handled before digital execution"
                    .into(),
            });
        }
        if !self.digital_started {
            if !self.analog_probes.is_empty() {
                self.state.initial_digital = Some(self.state.digital.clone());
            }
            let digital = self.state.digital.make_mut();
            digital.sample_analog_probes(&self.state.accepted_probe_values);
            if self.analog_probes.is_empty() {
                digital.start()?;
            } else {
                // Initial processes that read analog quantities must wait for
                // the time-zero candidate, just like a later scheduled read.
                digital.prepare_start()?;
            }
            self.digital_started = true;
        }
        Ok(())
    }

    /// The deck's name for this instance.
    pub(crate) fn instance_name(&self) -> &str {
        &self.instance
    }

    pub(crate) fn set_simulation_parameters(
        &mut self,
        parameters: rspice_veriloga_runtime::GeneratedSimulationParameters,
    ) {
        self.prepared_analog.invalidate();
        self.analog.make_mut().set_simulation_parameters(parameters);
    }

    pub(crate) fn analog_device(&self) -> &VerilogADevice {
        &self.analog
    }

    /// Instantaneous D/A level or impedance changes cannot be compared with
    /// smooth analog history. Report the final candidate's actual electrical
    /// change, including Z release, to the integration restart contract. A
    /// purely event-connected bit or an intermediate delta glitch adds none.
    pub(crate) fn candidate_discontinuity(&self) -> bool {
        if self.analog.discontinuity_rising() {
            return true;
        }
        let Some(trial) = &self.trial else {
            return false;
        };
        self.state.bridges.dac.iter().any(|bridge| {
            let level = |digital: &MixedDigital| {
                bridge.driven_level(
                    digital
                        .read(bridge.signal)
                        .map_or(FourStateBit::HighImpedance, |value| value.bit(bridge.bit)),
                )
            };
            level(&trial.rollback) != level(&self.state.digital)
        })
    }

    pub(crate) fn analysis_step(&self) -> (bool, bool) {
        (
            self.analog_inputs.initial_step,
            self.analog_inputs.final_step,
        )
    }

    pub(crate) fn set_analysis_step(
        &mut self,
        initial: bool,
        final_step: bool,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("configure analysis step")?;
        if self.analysis_step() != (initial, final_step) {
            self.prepared_analog.invalidate();
            self.analog
                .make_mut()
                .try_set_analysis_step(initial, final_step)
                .map_err(analog_error)?;
            self.analog_inputs.initial_step = initial;
            self.analog_inputs.final_step = final_step;
        }
        Ok(())
    }

    pub(crate) fn validate_analog_task_delivery(&self) -> Result<(), MixedSignalError> {
        if self.trial.is_some() {
            return Err(MixedSignalError::TrialProtocol {
                detail: "analog task delivery requires the mixed-signal trial to be accepted or rejected".into(),
            });
        }
        Ok(())
    }

    pub(crate) fn visit_accepted_analog_tasks(
        &mut self,
        consume: &mut dyn FnMut(rspice_veriloga_runtime::AnalogTaskEvent<'_>),
    ) -> Result<(), MixedSignalError> {
        self.validate_analog_task_delivery()?;
        // A read with no pending calls must not copy a shared rollback image.
        if self.analog.has_accepted_analog_tasks() {
            self.analog.make_mut().visit_accepted_analog_tasks(consume);
        }
        Ok(())
    }

    /// Keep every physical and event identity aligned with the circuit's
    /// final node numbering, before the first analysis begins.
    pub(crate) fn remap_circuit_nodes(&mut self, remap: impl Fn(usize) -> usize + Copy) {
        debug_assert!(!self.digital_started && self.trial.is_none());
        self.prepared_analog.invalidate();
        let branch_nodes: Vec<_> = (0..self.analog.num_branch_unknowns())
            .map(|index| remap(self.analog.branch_current_index(index).unwrap()))
            .collect();
        let analog = self.analog.make_mut();
        analog.remap_circuit_nodes(remap);
        // The device remaps potential nodes itself; its separately allocated
        // current unknowns also use this circuit's node numbering.
        analog.set_branch_current_indices(&branch_nodes);
        let bridges = self.state.bridges.make_mut();
        for bridge in &mut bridges.adc {
            bridge.positive = remap(bridge.positive);
            bridge.negative = remap(bridge.negative);
        }
        for bridge in &mut bridges.dac {
            bridge.positive = remap(bridge.positive);
            bridge.negative = remap(bridge.negative);
        }
        for probe in &mut self.analog_probes {
            if let AnalogProbeWiring::Solution {
                positive, negative, ..
            } = probe
            {
                *positive = remap(*positive);
                *negative = remap(*negative);
            }
        }
        for bus in &mut self.boundary_buses {
            for member in &mut bus.members {
                *member = remap(*member);
            }
        }
        for node in &mut self.event_nodes {
            *node = remap(*node);
        }
        self.event_nodes.sort_unstable();
        self.event_nodes.dedup();
        self.max_circuit_node = analog_solver_nodes(&self.analog)
            .chain(
                bridges
                    .adc
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .chain(
                bridges
                    .dac
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .max()
            .unwrap_or(0);
    }

    /// Physical module terminals, excluding the discrete port views.
    pub(crate) fn electrical_terminal_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.analog
            .terminal_names()
            .iter()
            .enumerate()
            .filter(|(_, name)| {
                !self
                    .state
                    .digital
                    .plan()
                    .signals
                    .iter()
                    .any(|signal| signal.name == **name)
            })
            .map(|(index, _)| self.analog.node_for_terminal(index))
    }

    /// Called only at fresh circuit elaboration after all connection validation.
    fn strip_event_boundaries(&mut self, nodes: &std::collections::BTreeSet<usize>) {
        let bridges = self.state.bridges.make_mut();
        self.event_nodes.extend(
            bridges
                .adc
                .iter()
                .filter(|bridge| nodes.contains(&bridge.positive))
                .map(|bridge| bridge.positive),
        );
        self.event_nodes.extend(
            bridges
                .dac
                .iter()
                .filter(|bridge| nodes.contains(&bridge.positive))
                .map(|bridge| bridge.positive),
        );
        self.event_nodes.sort_unstable();
        self.event_nodes.dedup();
        bridges
            .adc
            .retain(|bridge| !nodes.contains(&bridge.positive));
        bridges
            .dac
            .retain(|bridge| !nodes.contains(&bridge.positive));
        self.state.accepted_adc_voltages = vec![0.0; bridges.adc.len()];
        self.state.accepted_adc_transition_times = vec![None; bridges.adc.len()];
        self.state.adc_history = vec![BoundaryNetHistory::default(); bridges.adc.len()];
        self.state.dac_history = vec![BoundaryNetHistory::default(); bridges.dac.len()];
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
        let mut nodes: Vec<usize> = analog_solver_nodes(&self.analog)
            .chain(
                self.state
                    .bridges
                    .dac
                    .iter()
                    .flat_map(|bridge| [bridge.positive, bridge.negative]),
            )
            .filter(|node| *node > 0 && self.event_nodes.binary_search(node).is_err())
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }

    /// The deck nodes this module drives through a D/A bridge, ground aside.
    ///
    /// Both endpoints of each bridge, because the pair is what it stamps: a
    /// bridge referenced to a named supply moves that node as hard as it moves
    /// its output.
    ///
    /// Two callers need exactly this set, for the same reason and on the same
    /// side of it as the XSPICE `dac_bridge`. Generic transient voltage LTE
    /// cannot judge these nodes: at a scheduled edge the bridge's source moves
    /// discontinuously, the predictor still carries the pre-edge slope, and the
    /// difference it reports is the edge itself rather than truncation error —
    /// no smaller step removes it, so the controller walks the step down to the
    /// solver floor chasing it. And a force-accepted point must not have the
    /// global voltage-delta limiter clip the same edge back toward the level it
    /// just left. `xspice::models::bridges` answers both questions this way for
    /// its own `out` port, and `xspice::XspiceInstance` publishes it through
    /// `transient_voltage_lte_excluded_nodes`; a mixed module's D/A output is
    /// the same object with the same step semantics.
    ///
    /// Only bridges that survived elaboration are here: a boundary that landed
    /// on a pure event net has no analog node to speak of and
    /// `strip_event_boundaries` has already removed it.
    pub(crate) fn dac_bridge_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.state
            .bridges
            .dac
            .iter()
            .flat_map(|bridge| [bridge.positive, bridge.negative])
            .filter(|node| *node > 0)
    }

    /// The deck node each built-in boundary carries its bit on, ground aside.
    ///
    /// One entry per discrete port: `mixed_modules` gives a port exactly one
    /// bridge, A/D or D/A, so this is what "how many discrete endpoints meet on
    /// this net" has to count. [`Self::boundary_connections`] cannot serve that
    /// question — it reports each bridge's reference endpoint as well, and a
    /// deck that references a bridge to a named supply would count that port
    /// twice.
    pub(crate) fn boundary_port_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.state
            .bridges
            .adc
            .iter()
            .map(|bridge| bridge.positive)
            .chain(self.state.bridges.dac.iter().map(|bridge| bridge.positive))
            .filter(|node| *node > 0)
    }

    /// Signal names and electrical endpoints of the built-in boundaries.
    /// Wiring is available before digital startup, so circuit elaboration can
    /// validate it against event ports declared by later instances.
    pub(crate) fn boundary_connections(&self) -> impl Iterator<Item = (&str, usize)> + '_ {
        self.state
            .bridges
            .adc
            .iter()
            .flat_map(|bridge| {
                [
                    (bridge.signal_name.as_str(), bridge.positive),
                    (bridge.signal_name.as_str(), bridge.negative),
                ]
            })
            .chain(self.state.bridges.dac.iter().flat_map(|bridge| {
                [
                    (bridge.signal_name.as_str(), bridge.positive),
                    (bridge.signal_name.as_str(), bridge.negative),
                ]
            }))
    }

    /// Every boundary net's committed four-state value, paired with the circuit
    /// node the deck attached it to and with which half of the boundary it came
    /// from.
    ///
    /// Both bridge directions are reported, and the direction is reported with
    /// them: an A/D bridge's signal is what the module *read* off that node and
    /// a D/A bridge's is what it *drove* onto it. Those are different claims
    /// about the same net and they disagree for as long as the analog node
    /// takes to cross the reader's threshold, so a caller that has to name one
    /// value for the net needs to know which is which. Speculative state is
    /// never reported — this reads the accepted store, and every trial that has
    /// not been accepted has already been rolled back.
    pub(crate) fn boundary_digital_values<F>(&self, mut sink: F)
    where
        F: FnMut(usize, FourStateBit, BoundaryBitSource),
    {
        for bridge in &self.state.bridges.adc {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(
                    bridge.positive,
                    value.bit(bridge.bit),
                    BoundaryBitSource::Sampled,
                );
            }
        }
        for bridge in &self.state.bridges.dac {
            if let Some(value) = self.state.digital.read(bridge.signal) {
                sink(
                    bridge.positive,
                    value.bit(bridge.bit),
                    BoundaryBitSource::Driven,
                );
            }
        }
    }

    /// Every bus this module's vector boundary ports declare over deck nodes.
    ///
    /// Empty for a module with no vector discrete port, which is what makes a
    /// scalar-only deck publish the same empty bus table it always did.
    pub(crate) fn boundary_buses(&self) -> &[BoundaryBus] {
        &self.boundary_buses
    }

    /// Record that one vector boundary port's bits are one word.
    ///
    /// A declaration, like a bridge: taken while the module is being wired and
    /// never during a trial, which is why it is refused on a running host
    /// rather than merged into one.
    pub(crate) fn declare_boundary_bus(
        &mut self,
        bus: BoundaryBus,
    ) -> Result<(), MixedSignalError> {
        self.require_idle("declare a boundary bus")?;
        self.boundary_buses.push(bus);
        Ok(())
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
        let range = self.state.digital.declared_range(id);
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.bridges.make_mut().adc.push(AdcBridge {
            signal: id,
            bit,
            signal_name: boundary_net_name(signal, bit, width, range),
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
        let range = self.state.digital.declared_range(id);
        self.max_circuit_node = self.max_circuit_node.max(positive).max(negative);
        self.state.bridges.make_mut().dac.push(DacBridge {
            signal: id,
            bit,
            signal_name: boundary_net_name(signal, bit, width, range),
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
        self.begin_trial_with_integration_rules(
            time_seconds,
            timestep_seconds,
            integration,
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
    #[cfg(test)]
    pub(crate) fn begin_probe_trial(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(), MixedSignalError> {
        self.begin_trial_with_integration_rules(
            time_seconds,
            timestep_seconds,
            integration,
            integration,
            initial_step,
            final_step,
            true,
        )
    }

    /// Declare the smallest interval the analog solver may advance by.
    ///
    /// Zero — the default, and what a host driven directly rather than by the
    /// transient stepper keeps — means no solver has declared one, and every
    /// activation stepped past is treated as a lost breakpoint. A positive
    /// floor is the stepper's hard minimum timestep.
    pub(crate) fn set_analog_step_floor(&mut self, floor: f64) {
        self.analog_step_floor = if floor.is_finite() && floor > 0.0 {
            floor
        } else {
            0.0
        };
    }

    /// The last accepted analog time, or zero before the first acceptance.
    fn accepted_analog_time(&self) -> f64 {
        if self.state.started {
            self.state.accepted_time
        } else {
            0.0
        }
    }

    /// Begin a circuit trial with separate derivative and internal-state rules.
    /// A probe may inspect an already accepted timepoint but cannot commit.
    ///
    /// # Contract for a schedule finer than the analog resolution
    ///
    /// The digital scheduler owns the exact time and the ordering of every
    /// activation; the analog solver owes an activation only a timepoint at or
    /// after it. A trial past a pending activation is therefore opened rather
    /// than refused whenever that activation is closer to the accepted analog
    /// time than [`Self::set_analog_step_floor`]'s interval — no analog time
    /// exists between the two, so the activation is drained with the rest of
    /// the queue at this trial's tick, in tick order, and every activation in
    /// that window coalesces onto one analog timepoint. A `` `timescale
    /// 1ps/1fs `` clock, a sub-picosecond `#delay` chained onto an A/D
    /// crossing, and two activations inside one breakpoint tolerance all land
    /// this way. [`MixedSignalError::MissedDigitalBreakpoint`] is kept for an
    /// activation the stepper had a legal interval to and skipped anyway.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn begin_trial_with_integration_rules(
        &mut self,
        time_seconds: f64,
        timestep_seconds: f64,
        integration: IntegrationCoefficients,
        state_integration: IntegrationCoefficients,
        initial_step: bool,
        final_step: bool,
        probe: bool,
    ) -> Result<(), MixedSignalError> {
        if !self.digital_started {
            return Err(MixedSignalError::TrialProtocol {
                detail: "digital execution must start before a mixed trial".into(),
            });
        }
        self.require_idle("begin a trial")?;
        if !timestep_seconds.is_finite() || timestep_seconds < 0.0 {
            return Err(MixedSignalError::TrialProtocol {
                detail: "timestep must be finite and nonnegative".into(),
            });
        }
        // Reject the complete configuration before timestep setup can promote
        // an operating-point candidate or invalidate any retained state.
        integration.validate().map_err(analog_error)?;
        state_integration.validate().map_err(analog_error)?;
        if integration.active != state_integration.active {
            return Err(MixedSignalError::TrialProtocol {
                detail: "derivative and state integration rules must share the active analysis"
                    .into(),
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
        // Before anything below drains the queue: see
        // `ActiveTrial::scheduled_activation`.
        let scheduled_activation = self
            .state
            .digital
            .next_tick()
            .is_some_and(|next| next <= tick);
        if let Some(next) = self.state.digital.next_tick()
            && next < tick
        {
            let scheduled_seconds = self
                .resolution
                .ticks_to_seconds(next)
                .map_err(DigitalRunError::from)?;
            // An activation the stepper had a legal interval to and stepped
            // over is a lost breakpoint. One closer to the accepted analog
            // time than the solver's hard minimum is not reachable at all, so
            // it is landed on this timepoint instead of ending the run: the
            // queue below drains to this trial's tick in tick order, which is
            // where the activation keeps its exact time and its ordering.
            if !self.analog_step_floor.is_finite()
                || self.analog_step_floor <= 0.0
                || scheduled_seconds - self.accepted_analog_time() >= self.analog_step_floor
            {
                return Err(MixedSignalError::MissedDigitalBreakpoint {
                    scheduled_seconds,
                    trial_seconds: time_seconds,
                });
            }
        }

        // Taken before anything in this function can run the discrete half,
        // so the settle pass can tell a boundary this trial moved from one
        // that was already where it is. Both halves of the digital-to-analog
        // path are imaged, because either one changes the continuous problem
        // inside the step and the dating rule cannot tell them apart.
        read_dac_bits(&self.state, &mut self.scratch.trial.dac_at_trial_start)?;
        read_discrete_inputs(
            &self.state,
            &self.discrete_inputs,
            &mut self.scratch.trial.discrete_at_trial_start,
        );
        let rollback = self.state.digital.clone();
        let previous_inputs = self.analog_inputs;
        self.analog
            .capture_evaluation_state(&mut self.scratch.trial.analog_evaluation);
        let inputs = AnalogSolverInputs {
            analysis: 2,
            phase: self.analog_inputs.phase,
            initial_step,
            final_step,
            time_seconds,
            timestep_seconds,
            integration,
            state_integration,
        };
        // A due process can read V(...). Defer its activation until stamp or
        // bridge settling supplies this trial's candidate analog solution.
        // Each Newton probe starts from `rollback`, so the process is replayed
        // against the current candidate instead of retaining an earlier read.
        let prepare = self.apply_analog_inputs(inputs).and_then(|()| {
            // A digital-only activation has no analog read to defer. Preserve
            // its immediate begin-trial behavior without copying an idle host.
            if self.analog_probes.is_empty()
                && self
                    .state
                    .digital
                    .next_tick()
                    .is_some_and(|next| next <= tick)
            {
                self.state.digital.make_mut().advance_to(tick)?;
            }
            Ok(())
        });
        if let Err(error) = prepare {
            self.state.digital = rollback;
            // These are inputs this device was holding a moment ago, so
            // putting them back cannot be refused for being invalid; and if
            // the device has become unusable, the refusal worth reporting is
            // the one that made it so rather than a consequence of it.
            let _ = self.apply_analog_inputs(previous_inputs);
            let _ = self
                .analog
                .make_mut()
                .restore_evaluation_state(&self.scratch.trial.analog_evaluation);
            return Err(error);
        }
        self.analog_inputs = inputs;
        let start_digital =
            time_seconds == 0.0 && !self.state.started && self.state.initial_digital.is_some();
        if start_digital {
            self.state.digital = self.state.initial_digital.as_ref().unwrap().clone();
        }
        // Refilled rather than allocated. `clone_from` and `resize` keep the
        // allocation the last trial handed back, and every one of these is the
        // same length on every trial of a run, so an opened trial allocates
        // nothing.
        let mut vectors = std::mem::take(&mut self.scratch.trial);
        vectors.discrete_inputs.clear();
        vectors
            .discrete_inputs
            .extend(self.discrete_inputs.iter().map(|input| {
                self.analog
                    .discrete_state_value(input.variable)
                    .expect("canonical discrete input is a state variable")
            }));
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
            start_digital,
            digital_feedback: false,
            scheduled_activation,
            rollback,
            analog_inputs: previous_inputs,
            tick,
            published_tick: tick,
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

    /// Record that some enrolled instance's discrete half moved something an
    /// analog block reads inside this trial.
    ///
    /// One instance's D/A bridge and another's A/D bridge can sit on one deck
    /// node, and one instance's discrete variable steers the current its own
    /// analog block pushes into a node any other instance may sense, so what
    /// moved need not belong to the instance whose crossing is being dated.
    /// The coordinator makes the comparison for every enrolled instance at
    /// once and reports the disjunction here.
    pub(crate) fn note_shared_digital_feedback(&mut self) {
        if let Some(trial) = self.trial.as_mut() {
            trial.digital_feedback = true;
        }
    }

    /// Record that the circuit's shared process queue had an activation of its
    /// own due at or before this trial's tick when the trial opened.
    ///
    /// An enrolled instance holds values, not a queue, so it cannot read this
    /// for itself; the coordinator owns the wheel every enrolled instance
    /// schedules into and reports its answer here. The fact is the circuit's
    /// rather than this instance's on purpose: one instance's clock and
    /// another's D/A bridge can meet on one deck node, so the question a
    /// boundary move has to be judged against is whether *anything* was
    /// scheduled to run at this timepoint. See
    /// [`ActiveTrial::scheduled_activation`].
    pub(crate) fn note_scheduled_activation(&mut self) {
        if let Some(trial) = self.trial.as_mut() {
            trial.scheduled_activation = true;
        }
    }

    /// Whether the discrete half has moved anything this instance's analog
    /// equations read since the open trial began — a D/A bridge's bit, or a
    /// discrete variable an analog block references.
    ///
    /// Both halves are asked of the *store*, against the image of it taken
    /// before anything in the trial could run the discrete half, so what is
    /// answered is "did the digital world move this, inside this interval". A
    /// variable the analog half writes and the discrete half only reads never
    /// answers yes here, because the analog half does not write the store.
    ///
    /// Answers nothing when no trial is open: with no trial there is no
    /// interval for anything to have moved inside.
    pub(crate) fn digital_feedback_since_trial_start(&self) -> Result<bool, MixedSignalError> {
        let Some(trial) = self.trial.as_ref() else {
            return Ok(false);
        };
        if dac_bits_differ(&self.state, &trial.vectors.dac_at_trial_start)? {
            return Ok(true);
        }
        Ok(discrete_reads_differ(
            &self.state,
            &self.discrete_inputs,
            &trial.vectors.discrete_at_trial_start,
        ))
    }

    fn advance_trial_digital(&mut self, circuit_voltages: &[f64]) -> Result<(), MixedSignalError> {
        let tick = self.active_tick()?;
        let start = self.trial.as_ref().is_some_and(|trial| trial.start_digital);
        if start
            || self
                .state
                .digital
                .next_tick()
                .is_some_and(|next| next <= tick)
        {
            fill_analog_probes(
                &self.analog_probes,
                circuit_voltages,
                &mut self.scratch.probes,
            );
            let probes = std::mem::take(&mut self.scratch.probes);
            let advanced =
                self.advance_digital_at_candidate(circuit_voltages, &probes, tick, start);
            self.scratch.probes = probes;
            advanced?;
        }
        Ok(())
    }

    fn sample_discrete_inputs(&mut self) -> Result<bool, MixedSignalError> {
        let mut changed = false;
        for input in &self.discrete_inputs {
            let value = read_discrete_input(&self.state, input).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!(
                        "analog read of discrete signal `{}` has an X, Z, or non-finite value",
                        input.name
                    ),
                }
            })?;
            if self
                .analog
                .discrete_state_value(input.variable)
                .map(f64::to_bits)
                != Some(value.to_bits())
            {
                self.analog
                    .make_mut()
                    .sample_discrete_state(input.variable, value)
                    .map_err(analog_error)?;
                changed = true;
            }
        }
        Ok(changed)
    }

    /// Push one set of solver inputs into the analog device.
    ///
    /// The one writer of those five, so a trial and a trial's undo cannot
    /// drift apart in what they consider the device's inputs to be. The two
    /// analysis setters return without touching anything when the value they
    /// are handed is the value the device already holds, which is what makes
    /// the undo path cost a handful of comparisons.
    fn apply_analog_inputs(&mut self, inputs: AnalogSolverInputs) -> Result<(), MixedSignalError> {
        self.prepared_analog.invalidate();
        let analog = self.analog.make_mut();
        analog
            .try_set_analysis_type(inputs.analysis)
            .map_err(analog_error)?;
        analog
            .try_set_analysis_phase(inputs.phase)
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
            .try_set_integration_rules(inputs.integration, inputs.state_integration)
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
        if self.state.digital.is_view() {
            return Err(MixedSignalError::TrialProtocol {
                detail:
                    "circuit-owned digital inputs must be driven through the circuit coordinator"
                        .into(),
            });
        }
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
        digital.sample_analog_probes(&probes);
        if self
            .analog_probes
            .iter()
            .any(|probe| matches!(probe, AnalogProbeWiring::Variable { .. }))
        {
            let MixedDigital::Owned(digital) = digital else {
                unreachable!("view refused above")
            };
            digital.prepare_forces(&parsed, tick)?;
        } else {
            digital.force_many(&parsed, tick)?;
        }
        // A drive published into the slot can move a D/A input, so the
        // boundary is no longer known quiet.
        if let Some(trial) = self.trial.as_mut() {
            trial.bridges_quiet = false;
        }
        Ok(())
    }

    /// Stamp both the module's continuous equations and every active D/A
    /// bridge at a point the solver has already settled on.
    ///
    /// The accepted-point evaluation, the candidate inspection and the
    /// module's own tests all take this entry: whatever the analog half
    /// refuses here, there is no iterate left to throw away, so the refusal is
    /// the run's. [`Self::stamp_trial`] is the Newton-loop entry.
    pub fn stamp<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        matrix_add: M,
        rhs_add: R,
    ) -> Result<(), MixedSignalError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        self.stamp_classified(circuit_voltages, matrix_add, rhs_add, analog_accepted_error)
    }

    /// The same stamp at a Newton TRIAL point, where a non-finite analog value
    /// is a property of the candidate rather than of the module.
    ///
    /// One entry per answer instead of one entry and a classification: the
    /// wording a mixed host's failure carries — "at a trial iterate" — is a
    /// claim about which loop asked, and only the caller knows that.
    pub(crate) fn stamp_trial<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        matrix_add: M,
        rhs_add: R,
    ) -> Result<(), MixedSignalError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        self.stamp_classified(circuit_voltages, matrix_add, rhs_add, analog_trial_error)
    }

    fn stamp_classified<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
        mut rhs_add: R,
        classify: AnalogRefusal,
    ) -> Result<(), MixedSignalError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        self.active_tick()?;
        self.validate_solution(circuit_voltages)?;
        self.advance_trial_digital(circuit_voltages)?;
        self.sample_discrete_inputs()?;
        if self
            .analog_probes
            .iter()
            .any(|probe| matches!(probe, AnalogProbeWiring::Variable { .. }))
        {
            self.prepared_analog.prepare(
                self.analog.make_mut(),
                &self.discrete_inputs,
                circuit_voltages,
                classify,
            )?;
            self.prepared_analog.stamp(&mut matrix_add, &mut rhs_add);
        } else {
            self.analog
                .make_mut()
                .try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add)
                .map_err(|error| classify(&error))?;
        }
        self.stamp_dac_bridges(&mut matrix_add, &mut rhs_add)
    }

    /// Observe the already settled candidate without running HDL, resampling
    /// A/D inputs, or changing the analog operator/task state.
    pub(crate) fn stamp_static_dae<M, R>(
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
        if !self.trial.as_ref().is_some_and(|trial| trial.bridges_quiet) {
            return Err(MixedSignalError::TrialProtocol {
                detail: "static history requires a settled mixed boundary".into(),
            });
        }
        self.analog
            .make_mut()
            .try_stamp_with_mode(
                circuit_voltages,
                &mut matrix_add,
                &mut rhs_add,
                rspice_veriloga::vm::VerilogAEvaluationMode::StaticDaeProbe,
            )
            .map_err(analog_error)?;
        self.stamp_dac_bridges(&mut matrix_add, &mut rhs_add)
    }

    fn stamp_dac_bridges<M, R>(
        &self,
        matrix_add: &mut M,
        rhs_add: &mut R,
    ) -> Result<(), MixedSignalError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        for bridge in &self.state.bridges.dac {
            let value = self.state.digital.read(bridge.signal).ok_or_else(|| {
                MixedSignalError::InvalidBridge {
                    detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
                }
            })?;
            let Some(level) = bridge.driven_level(value.bit(bridge.bit)) else {
                continue;
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

    /// Ask the transient controller to solve at the earliest A/D crossing this
    /// settled trial published inside its own interval.
    ///
    /// Read from the trial rather than predicted before it, because the fact
    /// that decides whether a crossing is an interior root at all — did the
    /// discrete half move something the analog equations read in this interval
    /// — is one the discrete half has to run to establish.
    /// [`Self::settle_into`] has already applied that fact when it dated each
    /// transition: a crossing on an interval such a movement happened in is
    /// dated at the endpoint, so it is not interior here and asks for nothing.
    /// The two rules are therefore one rule, written once — the fourth
    /// question of this module's "three time bases".
    pub(crate) fn trial_boundary_refinement_time(
        &self,
        minimum_timestep: f64,
    ) -> Result<Option<f64>, MixedSignalError> {
        let Some(trial) = self.trial.as_ref() else {
            return Ok(None);
        };
        if !self.state.started || trial.time_seconds <= self.state.accepted_time {
            return Ok(None);
        }
        if !minimum_timestep.is_finite() || minimum_timestep <= 0.0 {
            return Err(MixedSignalError::TrialProtocol { detail: "boundary root refinement requires the solver's positive finite minimum timestep".into() });
        }
        let time = trial.time_seconds;
        let start = self.state.accepted_time;
        // A physical root cannot be resolved more finely than the solver can
        // advance. Keep this tolerance in seconds, independent of HDL precision.
        let tolerance = (64.0 * f64::EPSILON * time.abs().max(start.abs())).max(minimum_timestep);
        let mut first_allowed = (start + minimum_timestep).max(start.next_up());
        if first_allowed - start < minimum_timestep {
            first_allowed = first_allowed.next_up();
        }
        let mut earliest: Option<f64> = None;
        for (moved, crossing) in trial
            .vectors
            .adc_moved
            .iter()
            .zip(&trial.vectors.transition_times)
        {
            // A bridge that did not move this trial keeps the accepted
            // transition time it was opened with, which is not this interval's.
            if !*moved {
                continue;
            }
            let Some(crossing) = *crossing else {
                continue;
            };
            if time - crossing <= tolerance {
                continue;
            }
            // Land on the entering side of a representable root, as the analog
            // cross operator does. Never quantize this physical solve to HDL ticks.
            let target = crossing.max(start).next_up().max(first_allowed);
            if target >= time {
                continue;
            }
            earliest = Some(earliest.map_or(target, |current| current.min(target)));
        }
        Ok(earliest)
    }

    /// Sample all A/D bridges from one converged candidate, publish each
    /// transition at its own instant in ascending crossing order, and settle
    /// every same-time delta cycle. Returns true when digital activity changed
    /// any D/A input and Newton must be repeated at the same timestamp.
    ///
    /// Each transition is dated by interpolating its threshold crossing inside
    /// the trial's step, by the same rule the Xyce DIG code models date theirs
    /// — [`threshold_crossing_time`]. That instant is kept unquantized in the
    /// accepted state, it is what every process the transition wakes reads as
    /// `$abstime`, and the digital slot it is published into is the tick
    /// *nearest* it, because Verilog-AMS LRM 2.4 section 7.3.6.1 places an
    /// analog event in the digital domain at the nearest digital time tick.
    ///
    /// Two bridges that crossed at two instants are two events. They are
    /// published one after the other, earliest first, each at its own tick —
    /// not as one bank at the latest of their ticks, which would delay the
    /// earlier one by a whole tick and report its `$abstime` as the later
    /// one's.
    ///
    /// Nearest here and floored in [`Self::begin_trial`] are not a
    /// contradiction, because they quantize two different quantities — see
    /// this module's "three time bases". That one floors *the trial's*
    /// timestamp to bound how far the digital world may be advanced. This one
    /// rounds *the transition's* timestamp to name the tick its event belongs
    /// to, and the standard fixes that at the nearest tick.
    ///
    /// The `max` is what keeps the rounding one-directional. A crossing in the
    /// lower half of the trial's tick rounds to the tick before it, which is a
    /// slot the digital world has already left, so the clamp publishes it here
    /// instead; a crossing in the upper half rounds to the tick after, which is
    /// still ahead and is left alone. The mark it is clamped against belongs to
    /// the *trial*, not to this pass (`ActiveTrial::published_tick`), because a
    /// later Newton iteration of the same trial can interpolate a root behind
    /// one already published. Nothing is queued between the crossing and here
    /// to reorder against, because [`Self::begin_trial`] has already refused a
    /// step that passed a scheduled event.
    ///
    /// Only the crossing's zero-delay consequences execute at this physical
    /// time. Its reporting tick — nearest to the crossing, then clamped
    /// forward, which is this lane's quantization and not the ceiling the
    /// other event kernel's wakes take — does not authorize draining unrelated
    /// timers; those remain pending until analog time reaches their timestamp.
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
        fill_analog_probes(&self.analog_probes, circuit_voltages, &mut scratch.probes);
        let start = self.trial.as_ref().is_some_and(|trial| trial.start_digital);
        if start
            || self
                .state
                .digital
                .next_tick()
                .is_some_and(|next| next <= tick)
        {
            self.advance_digital_at_candidate(circuit_voltages, &scratch.probes, tick, start)?;
        }
        // Established after the discrete half has run and before the first
        // crossing of this pass is dated: whether the digital world moved
        // something the analog equations read is what decides how to date one,
        // and the trial's own `dac_moved` and the device's own copy of the
        // discrete reads are not written until the end of the pass.
        if self.digital_feedback_since_trial_start()?
            && let Some(trial) = self.trial.as_mut()
        {
            trial.digital_feedback = true;
        }
        let digital_feedback = self
            .trial
            .as_ref()
            .is_some_and(|trial| trial.digital_feedback);
        scratch.bit_drives.clear();
        scratch.drives.clear();
        scratch.crossings.clear();
        scratch.sampled.clear();
        for (index, bridge) in self.state.bridges.adc.iter().enumerate() {
            let voltage = node_voltage(circuit_voltages, bridge.positive)
                - node_voltage(circuit_voltages, bridge.negative);
            scratch.sampled.push(voltage);
            let held = self
                .state
                .digital
                .read(bridge.signal)
                .map(|value| value.bit(bridge.bit));
            let Some((bit, threshold)) = bridge.decision(
                voltage,
                self.state
                    .started
                    .then_some(self.state.accepted_adc_voltages[index]),
                held,
            ) else {
                continue;
            };
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
            // A step the digital half moved the analog equations in stepped
            // this node itself, at the tick that movement happened on, so
            // there is no interior root to interpolate towards: the transition
            // is this endpoint's own.
            let crossing = if digital_feedback {
                time_seconds
            } else {
                threshold_crossing_time(
                    time_seconds,
                    time_seconds - timestep_seconds,
                    timestep_seconds,
                    self.state.accepted_adc_voltages[index],
                    voltage,
                    threshold,
                )
            };
            scratch.bit_drives.push((index, bit));
            scratch.crossings.push((index, crossing));
        }
        // Each transition carries its own instant, so the pass publishes in
        // ascending crossing order rather than as one bank: two bridges that
        // crossed at different times are two events, and collapsing them onto
        // the later one's tick delays the earlier one by a whole tick.
        order_publications_by_crossing(&mut scratch.bit_drives, &mut scratch.crossings);
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
        // Carried by the trial rather than seeded here, so the running maximum
        // spans every Newton iteration of it — see `ActiveTrial::published_tick`.
        let mut published_tick = self
            .trial
            .as_ref()
            .map_or(tick, |trial| trial.published_tick);
        let mut group = 0;
        while group < scratch.crossings.len() {
            let crossing = scratch.crossings[group].1;
            let end = scratch.crossings[group..]
                .iter()
                .position(|(_, other)| *other != crossing)
                .map_or(scratch.crossings.len(), |offset| group + offset);
            // One drive per signal, not per bridge. A vector boundary port is
            // N bridges over one discrete signal, and the store publishes a
            // whole signal at a time (`store.rs`'s `force` refuses a value
            // that is not the declared width), so the bits that moved at this
            // instant are composed onto the value the signal holds now and the
            // port publishes as one transition — which is what the discrete
            // half's own vector assignment is. Bits whose bridge is inside its
            // threshold window, or crossed at another instant, keep what they
            // hold until their own publication.
            compose_bit_drives(
                &self.state,
                &scratch.bit_drives[group..end],
                &mut scratch.drives,
            )?;
            // Verilog-AMS LRM 2.4 section 7.3.6.1: an analog event crossing
            // into the digital domain lands on the *nearest* digital time
            // tick. This is the transition's own timestamp being quantized,
            // which is not the mapping `begin_trial` applies to the trial's
            // timestamp — see this module's "three time bases".
            //
            // An endpoint-dated transition is exempt, and must be: rounding
            // the trial's own timestamp would publish into the tick *after*
            // the one the trial is running, which is the digital world being
            // run past an instant the integrator has accepted. The trial's
            // floored tick is where this transition belongs, because the
            // event that caused it happened on that tick.
            //
            // `published_tick` carries the running maximum rather than the
            // group's own answer, which is what keeps the sequence monotone: a
            // crossing in the lower half of a tick rounds to a slot the
            // digital world has already left, and an earlier group — of this
            // pass or of an earlier Newton iteration of this same trial — may
            // have left a later one still.
            let crossing_tick = if digital_feedback {
                tick
            } else {
                self.resolution
                    .seconds_to_ticks(crossing)
                    .map_err(DigitalRunError::from)?
            };
            published_tick = published_tick.max(crossing_tick);
            if let Some(trial) = self.trial.as_mut() {
                trial.published_tick = published_tick;
            }
            if !scratch.drives.is_empty() {
                if self.state.digital.is_view() {
                    self.state.digital.make_mut().force_many_from_analog_at(
                        &scratch.drives,
                        published_tick,
                        crossing,
                        time_seconds,
                    )?;
                } else {
                    self.with_analog_participant(circuit_voltages, |digital, producer| {
                        digital.sample_analog_probes(&scratch.probes);
                        digital.force_many_from_analog_at(
                            &scratch.drives,
                            published_tick,
                            crossing,
                            time_seconds,
                            producer,
                        )
                    })?;
                }
                if let Some(trial) = self.trial.as_mut() {
                    for &(index, _) in &scratch.bit_drives[group..end] {
                        trial.vectors.transition_times[index] = Some(crossing);
                        trial.vectors.adc_moved[index] = true;
                    }
                }
            }
            group = end;
        }
        read_dac_bits(&self.state, &mut scratch.dac_after)?;
        // Circuit-owned processes see the complete A/D bank before any analog
        // equation reads their outputs. The group samples these inputs at stamp.
        let changed = (!self.state.digital.is_view() && self.sample_discrete_inputs()?)
            || scratch.dac_before != scratch.dac_after;
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
        let trial = self.prepare_trial_acceptance()?;
        self.apply_prepared_acceptance(trial);
        Ok(())
    }

    /// Reserve a validated candidate for the circuit's acceptance barrier.
    /// Dropping the reservation restores this host; committing cannot fail.
    pub(crate) fn prepare_circuit_acceptance(
        &mut self,
    ) -> Result<PreparedMixedAcceptance<'_>, (String, MixedSignalError)> {
        match self.prepare_trial_acceptance() {
            Ok(trial) => Ok(PreparedMixedAcceptance {
                host: self,
                trial: Some(trial),
            }),
            Err(error) => {
                if let Some(trial) = self.trial.take() {
                    self.unwind(trial);
                }
                Err((self.instance_name().to_string(), error))
            }
        }
    }

    fn prepare_trial_acceptance(&mut self) -> Result<ActiveTrial, MixedSignalError> {
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
        let trial = self.trial.take().expect("checked above");
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
        self.scratch.adc_history = histories.0;
        self.scratch.dac_history = histories.1;
        Ok(trial)
    }

    /// Called only with the candidate returned by prepare_trial_acceptance,
    /// while a reservation excludes any mutation of this host.
    fn apply_prepared_acceptance(&mut self, mut trial: ActiveTrial) {
        self.prepared_analog.invalidate();
        std::mem::swap(&mut self.state.adc_history, &mut self.scratch.adc_history);
        std::mem::swap(&mut self.state.dac_history, &mut self.scratch.dac_history);
        self.analog.make_mut().apply_validated_advance_state();
        self.state.accepted_tick = trial.tick;
        self.state.accepted_time = trial.time_seconds;
        self.state.started = true;
        self.state.initial_digital = None;
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
        self.scratch.probe_history.time = None;
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
        for (input, value) in self
            .discrete_inputs
            .iter()
            .zip(&trial.vectors.discrete_inputs)
        {
            let _ = self
                .analog
                .make_mut()
                .sample_discrete_state(input.variable, *value);
        }
        let _ = self.undo_analog_inputs(trial.analog_inputs);
        let _ = self
            .analog
            .make_mut()
            .restore_evaluation_state(&trial.vectors.analog_evaluation);
        self.scratch.trial = trial.vectors;
    }

    /// Restore every digital, event and driver bit to the state at
    /// [`begin_trial`](Self::begin_trial).
    ///
    /// Also restore scalar solver inputs, externally supplied discrete state
    /// and the ordinary evaluation-variable image. The bridge tables cannot
    /// have moved, because adding one requires an idle host. The accepted bank
    /// cannot have moved, because [`Self::accept_trial`] is its only writer.
    /// The analog device's accepted operator history remains unchanged: a trial
    /// that does not reach
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
        if trial.probe && trial.bridges_quiet {
            self.record_boundary_probe(trial.time_seconds);
        }
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
        if self.state.digital.is_view() {
            return Err(MixedSignalError::TrialProtocol {
                detail: "a circuit-owned digital domain requires a circuit checkpoint".into(),
            });
        }
        if !self.digital_started {
            return Err(MixedSignalError::TrialProtocol {
                detail: "cannot checkpoint a mixed module before digital execution starts".into(),
            });
        }
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
        if self.state.digital.is_view() || checkpoint.state.digital.is_view() {
            return Err(MixedSignalError::TrialProtocol {
                detail: "a circuit-owned digital domain requires a circuit checkpoint".into(),
            });
        }
        if checkpoint.source_digest != self.source_digest {
            return Err(MixedSignalError::TrialProtocol {
                detail: "checkpoint source identity does not match this mixed module".into(),
            });
        }
        self.analog
            .validate_checkpoint_state(&checkpoint.analog_checkpoint)
            .map_err(analog_error)?;
        self.analog = checkpoint.analog.clone();
        self.prepared_analog.invalidate();
        self.analog_inputs = checkpoint.analog_inputs;
        self.state = checkpoint.state.clone();
        self.digital_started = true;
        self.scratch.probe_history.time = None;
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

    fn record_boundary_probe(&mut self, time: f64) {
        let history = &mut self.scratch.probe_history;
        let bridges = &self.state.bridges;
        if history.time != Some(time)
            || history.adc.len() != bridges.adc.len()
            || history.dac.len() != bridges.dac.len()
        {
            history.time = Some(time);
            history.probes = 0;
            history
                .adc
                .resize(bridges.adc.len(), BoundaryNetHistory::default());
            history
                .dac
                .resize(bridges.dac.len(), BoundaryNetHistory::default());
            history.adc.fill(BoundaryNetHistory::default());
            history.dac.fill(BoundaryNetHistory::default());
        }
        let record = |signal, bit, entry: &mut BoundaryNetHistory| {
            let bit = self
                .state
                .digital
                .read(signal)
                .map_or(FourStateBit::HighImpedance, |value| value.bit(bit));
            if entry.filled == 0 || entry.recent & 3 != BoundaryNetHistory::code(bit) {
                // Rejected probes of one timepoint: every retained sample is a
                // transition the solver produced, and there is no schedule to
                // attribute one to inside a single tick.
                entry.push(
                    bit,
                    if entry.filled == 0 {
                        BoundaryMove::Still
                    } else {
                        BoundaryMove::Unexplained
                    },
                );
            }
        };
        for (bridge, entry) in bridges.adc.iter().zip(&mut history.adc) {
            record(bridge.signal, bridge.bit, entry);
        }
        for (bridge, entry) in bridges.dac.iter().zip(&mut history.dac) {
            record(bridge.signal, bridge.bit, entry);
        }
        history.probes = history.probes.saturating_add(1);
    }

    /// Describe observed switching only after the caller has exhausted its
    /// convergence recovery. These samples never decide whether a solve passes.
    pub(crate) fn rejected_probe_activity(&self, time: f64) -> Option<String> {
        let history = &self.scratch.probe_history;
        if history.time != Some(time) {
            return None;
        }
        let mut nets = Vec::new();
        let mut elided = 0;
        for (signal, node, read_by_module, entry) in self
            .state
            .bridges
            .adc
            .iter()
            .zip(&history.adc)
            .map(|(bridge, entry)| (&bridge.signal_name, bridge.positive, true, entry))
            .chain(
                self.state
                    .bridges
                    .dac
                    .iter()
                    .zip(&history.dac)
                    .map(|(bridge, entry)| (&bridge.signal_name, bridge.positive, false, entry)),
            )
        {
            if entry.run < 3 {
                continue;
            }
            if nets.len() == 32 {
                elided += 1;
                continue;
            }
            nets.push(
                BoundaryNetActivity {
                    signal: signal.clone(),
                    node,
                    read_by_module,
                    moves: entry.run,
                    recent: entry.values(),
                }
                .to_string(),
            );
        }
        if nets.is_empty() {
            return None;
        }
        let omitted = if elided == 0 {
            String::new()
        } else {
            format!("; {elided} additional switching nets omitted")
        };
        Some(format!(
            "mixed Verilog-AMS instance '{}' observed repeated boundary switching at t={time:e}s during {} rejected solver probes (retaining the last transition values): {}{omitted}",
            self.instance,
            history.probes,
            nets.join("; "),
        ))
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
        // One reading of this timepoint, shared by every net on it: whether a
        // process was due to run here is a fact about the timepoint, not about
        // a bridge.
        let classify = |moved: bool| match (moved, trial.scheduled_activation) {
            (false, _) => BoundaryMove::Still,
            (true, true) => BoundaryMove::Scheduled,
            (true, false) => BoundaryMove::Unexplained,
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
                classify(trial.vectors.adc_moved.get(index).copied().unwrap_or(false)),
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
                classify(trial.vectors.dac_moved.get(index).copied().unwrap_or(false)),
            );
            dac.push(entry);
        }
    }

    /// The diagnostic for a boundary that has moved at every accepted timepoint
    /// for too long, or `None` when none has.
    ///
    /// "Too long" counts only the moves no schedule explains — the rule is
    /// [`BoundaryMove`], and the reason it exists is on
    /// [`MAX_CONSECUTIVE_BOUNDARY_FLIPS`]. The reported `moved N times` is that
    /// same count, so the number in the message is the number the ceiling was
    /// compared against.
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

/// Put the bits one settle moved, and their crossing times, into ascending
/// crossing order — the order the analog world produced them in.
///
/// The two vectors are filled in lockstep by `settle_into`, one entry per
/// moved bridge, and they stay aligned through this: a caller that reads
/// `bit_drives[i]` and `crossings[i]` is reading one transition either side of
/// the reordering. A pass moves a handful of bridges at most — an insertion
/// sort over both at once is what keeps them aligned without a permutation
/// buffer, and it is stable, so bridges that crossed at the same instant keep
/// bridge order and publish as one bank.
fn order_publications_by_crossing(
    bit_drives: &mut [(usize, FourStateBit)],
    crossings: &mut [(usize, f64)],
) {
    debug_assert_eq!(bit_drives.len(), crossings.len());
    for index in 1..crossings.len() {
        let mut slot = index;
        while slot > 0 && crossings[slot - 1].1 > crossings[slot].1 {
            crossings.swap(slot - 1, slot);
            bit_drives.swap(slot - 1, slot);
            slot -= 1;
        }
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
fn boundary_net_name(signal: &str, bit: u32, width: u32, range: VectorBounds) -> String {
    if width <= 1 {
        signal.to_string()
    } else {
        // `bit` is the position the bridge reads; the module's name for that
        // bit is the range's, and on a `[7:4]` net the two are not the same
        // number.
        format!("{signal}[{}]", range.index_at(i64::from(bit)))
    }
}

/// Read every D/A bridge's driven bit into `out`.
///
/// A free function, and taking the state rather than the host, so a caller can
/// hold the scratch vectors apart from the host while it fills one of them.
/// `out` is cleared first, so its allocation survives from call to call.
/// Whether any D/A bridge holds a bit other than the one `reference` records.
///
/// The comparison a [`read_dac_bits`] pair would make, without the second
/// vector: this answers a yes/no question on the settle path, where allocating
/// or refilling a bank per pass would be paying a vector to learn a bool.
fn dac_bits_differ(
    state: &MixedState,
    reference: &[FourStateBit],
) -> Result<bool, MixedSignalError> {
    if reference.len() != state.bridges.dac.len() {
        return Ok(true);
    }
    for (bridge, was) in state.bridges.dac.iter().zip(reference) {
        let bit = state
            .digital
            .read(bridge.signal)
            .map(|value| value.bit(bridge.bit))
            .ok_or_else(|| MixedSignalError::InvalidBridge {
                detail: format!("D/A signal `{}` disappeared", bridge.signal_name),
            })?;
        if bit != *was {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The value the store holds for one discrete quantity an analog block reads,
/// or `None` when it is X, Z, or not finite.
///
/// The one reader of that question, so what the dating rule compares and what
/// [`MixedSignalHost::sample_discrete_inputs`] hands the analog device cannot
/// disagree about a value. `None` is not a value the analog half can be given
/// at all: `sample_discrete_inputs` refuses the run by name when it is next
/// asked for one, which is why the comparison below can treat it as just
/// another reading.
fn read_discrete_input(state: &MixedState, input: &DiscreteAnalogInput) -> Option<f64> {
    let value = if state.digital.is_real(input.signal) {
        state.digital.read_real(input.signal)
    } else {
        state
            .digital
            .read(input.signal)
            .and_then(|value| value.to_integer(input.signed))
            .map(|value| value as f64)
    };
    value.filter(|value| value.is_finite())
}

fn read_discrete_inputs(
    state: &MixedState,
    inputs: &[DiscreteAnalogInput],
    out: &mut Vec<Option<f64>>,
) {
    out.clear();
    out.extend(inputs.iter().map(|input| read_discrete_input(state, input)));
}

/// Whether the store holds a different value for any analog-read discrete
/// quantity than `reference` recorded.
///
/// Compared by bit pattern, as [`MixedSignalHost::sample_discrete_inputs`]
/// compares: a discrete quantity is an exact integer or an exact real the
/// discrete half assigned, and a value that merely rounds to the same one is a
/// different value the analog equations will read differently.
fn discrete_reads_differ(
    state: &MixedState,
    inputs: &[DiscreteAnalogInput],
    reference: &[Option<f64>],
) -> bool {
    if reference.len() != inputs.len() {
        return true;
    }
    inputs.iter().zip(reference).any(|(input, was)| {
        read_discrete_input(state, input).map(f64::to_bits) != was.map(f64::to_bits)
    })
}

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
fn fill_analog_probes(
    probes: &[AnalogProbeWiring],
    circuit_voltages: &[f64],
    out: &mut Vec<Option<f64>>,
) {
    out.clear();
    out.extend(probes.iter().map(|probe| probe.sample(circuit_voltages)));
}

fn analog_solver_nodes(analog: &VerilogADevice) -> impl Iterator<Item = usize> + '_ {
    (0..analog.num_terminals())
        .map(|terminal| analog.node_for_terminal(terminal))
        .chain(
            (0..analog.num_internal_nodes()).filter_map(|index| analog.internal_node_index(index)),
        )
        .chain(
            (0..analog.num_branch_unknowns())
                .filter_map(|index| analog.branch_current_index(index)),
        )
}

/// Resolve potential and flow probes to the owning solver's solution indices.
/// Branch identity is resolved before circuit nodes are collapsed.
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
        if net == "0"
            || canonical_ir
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
        if let Some(index) = canonical_ir
            .hir
            .internal_nodes
            .iter()
            .position(|node| node.name == net)
            && let Some(node) = analog.internal_node_index(index).filter(|node| *node != 0)
        {
            return Ok(node);
        }
        Err(MixedSignalError::InvalidBridge {
            detail: format!(
                "`{net}` is probed from a discrete-domain expression but has no solver-node mapping in module `{}`",
                canonical_ir.mir.module_name
            ),
        })
    };
    use rspice_veriloga::canonical_ir::digital::DigitalAnalogProbeTarget;
    use rspice_veriloga::canonical_ir::digital::DigitalAnalogQuantity as AccessKind;
    let resolve_mir_node = |name: &str| {
        if name == "0"
            || canonical_ir
                .hir
                .ground_nodes
                .iter()
                .any(|ground| ground == name)
        {
            Ok(None)
        } else {
            canonical_ir
                .mir
                .nodes
                .iter()
                .find(|node| node.name == name)
                .map(|node| Some(node.id))
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!("analog flow probe names unknown node `{name}`"),
                })
        }
    };
    probes
        .iter()
        .map(|probe| {
            let (positive, negative, declared) = match &probe.target {
                DigitalAnalogProbeTarget::Variable { name } => {
                    if analog.variable(name).is_none() {
                        return Err(MixedSignalError::InvalidBridge {
                            detail: format!(
                                "analog-owned variable `{name}` has no retained evaluation slot"
                            ),
                        });
                    }
                    return Ok(AnalogProbeWiring::Variable {
                        name: name.to_string(),
                    });
                }
                DigitalAnalogProbeTarget::Nodes { positive, negative } => {
                    (positive.as_str(), negative.as_deref(), None)
                }
                DigitalAnalogProbeTarget::Branch { name } => {
                    let branch = canonical_ir
                        .hir
                        .branches
                        .iter()
                        .find(|branch| branch.name == *name)
                        .ok_or_else(|| MixedSignalError::InvalidBridge {
                            detail: format!("analog probe names undeclared branch `{name}`"),
                        })?;
                    (
                        branch.pos_node.as_str(),
                        Some(branch.neg_node.as_str()),
                        Some(name),
                    )
                }
            };
            if probe.quantity == AccessKind::Potential {
                return Ok(AnalogProbeWiring::Solution {
                    positive: resolve(positive)?,
                    negative: negative.map_or(Ok(0), resolve)?,
                    scale: 1.0,
                });
            }
            // Match authored MIR identities before mapping into the circuit. Tied
            // terminals must not merge distinct branches or their current unknowns.
            let pos = resolve_mir_node(positive)?;
            let neg = negative.map_or(Ok(None), resolve_mir_node)?;
            let unknown = canonical_ir
                .mir
                .branch_unknowns
                .iter()
                .find(|unknown| {
                    unknown.declared_name.as_ref() == declared
                        && ((unknown.pos_node == pos && unknown.neg_node == neg)
                            || (unknown.pos_node == neg && unknown.neg_node == pos))
                })
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!(
                        "flow probe `{}` has no simultaneous solver current in module `{}`",
                        probe.spelling(),
                        canonical_ir.mir.module_name
                    ),
                })?;
            let current = analog
                .branch_current_index(usize::from(unknown.id))
                .filter(|index| *index != 0)
                .ok_or_else(|| MixedSignalError::InvalidBridge {
                    detail: format!(
                        "flow probe `{}` has no solver-current mapping",
                        probe.spelling()
                    ),
                })?;
            Ok(AnalogProbeWiring::Solution {
                positive: current,
                negative: 0,
                scale: if unknown.pos_node == pos && unknown.neg_node == neg {
                    1.0
                } else {
                    -1.0
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

/// How one stamping path words the analog half's refusal.
///
/// The same stamping code serves the Newton loop and the accepted-point
/// evaluation, and the two cannot answer the same way: only the first has an
/// iterate to reject. The entry the caller picks carries the answer, rather
/// than the stamp inferring it from state it does not have.
pub(super) type AnalogRefusal = fn(&rspice_veriloga::vm::VmError) -> MixedSignalError;

/// Classify the analog half's refusal at an ACCEPTED point.
///
/// Nothing here is speculative: the solver has settled on this solution, so a
/// refusal is the run's however numeric it looks.
pub(super) fn analog_accepted_error(error: &rspice_veriloga::vm::VmError) -> MixedSignalError {
    analog_error(error)
}

/// Classify the analog half's refusal at a Newton TRIAL evaluation.
///
/// A mixed module's continuous equations are the same Verilog-A the plain
/// analog route runs, so a `ln()` that leaves its domain at an overshooting
/// iterate is the same rejectable trial here as it is there — the module is
/// well posed and the *point* is not. Nothing else changes: a structural
/// refusal, and every failure at an accepted point
/// (`analog_accepted_error` above), still ends the run where it happens.
pub(super) fn analog_trial_error(error: &rspice_veriloga::vm::VmError) -> MixedSignalError {
    match error {
        rspice_veriloga::vm::VmError::InvalidNumericResult(detail) => {
            MixedSignalError::AnalogNonFinite {
                detail: detail.clone(),
            }
        }
        other => MixedSignalError::Analog {
            detail: other.to_string(),
        },
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

    /// A host with one activation pending, reported with the analog time that
    /// is accepted behind it.
    ///
    /// The A/D edge at the second trial leaves [`MIXED`]'s `#2` resume behind,
    /// so the pending activation is two ticks after the tick the crossing was
    /// published into and the interval to it from the accepted point is a
    /// whole number of nanoseconds.
    fn a_pending_activation() -> (MixedSignalHost, f64, f64) {
        let mut host = host();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        begin(&mut host, 1);
        settle_and_accept(&mut host, &[0.0, 0.0, 1.0, 0.0]);
        let due = host
            .next_event_time()
            .expect("the wheel is readable")
            .expect("the A/D edge leaves its `#2` resume behind");
        (host, 1.0e-9, due)
    }

    /// The missed-breakpoint refusal is kept for an activation the stepper had
    /// a legal interval to, and only for that one.
    ///
    /// A mixed module's next activation is one of the stepper's own landing
    /// targets, so no engine path steps past a reachable one. That is the
    /// point — and it also means the refusal can no longer be provoked from a
    /// deck, which is how a guard rots into one that fires on the wrong side.
    /// Both sides of the line are therefore pinned from the module's own
    /// interface: an activation a whole floor or more after the accepted time
    /// is a lost breakpoint, and the same activation under a floor wider than
    /// the interval to it is not a fault at all — no analog time exists between
    /// the accepted point and it.
    #[test]
    fn only_an_activation_with_an_interval_to_it_is_a_missed_breakpoint() {
        let (mut host, accepted, due) = a_pending_activation();
        let interval = due - accepted;
        assert!(
            interval > 0.0,
            "the pending activation must be ahead of the accepted point, \
             {due:e}s against {accepted:e}s"
        );
        let past = due + 1.0e-9;

        host.set_analog_step_floor(interval * 0.5);
        let error = host
            .begin_trial(
                past,
                past - accepted,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .expect_err("a trial past a reachable activation is a lost breakpoint");
        assert!(
            matches!(
                error,
                MixedSignalError::MissedDigitalBreakpoint {
                    scheduled_seconds,
                    ..
                } if scheduled_seconds == due
            ),
            "the refusal must name the activation that was stepped over, got {error}"
        );

        let (mut host, accepted, due) = a_pending_activation();
        let past = due + 1.0e-9;
        host.set_analog_step_floor((due - accepted) * 2.0);
        host.begin_trial(
            past,
            past - accepted,
            IntegrationCoefficients::inactive(),
            false,
            false,
        )
        .expect("an activation no analog step can reach is landed, not refused");
        host.reject_trial().expect("the trial rolls back");
    }

    #[test]
    fn runtime_delays_and_clock_queries_retain_physical_activation_and_roll_back() {
        let source = "module clocked(p,adc); inout p; electrical p; input adc; wire adc;
            real absolute, reported; reg ok, unrelated;
            initial begin absolute=0.0; reported=0.0; ok=0; unrelated=0; #1 unrelated=1; end
            always @(posedge adc) begin
                absolute=$abstime; reported=$realtime; ok=($time==1 && $stime==1);
                #(adc & 1'bx) reported=reported+$realtime;
                #($realtime/2.0) absolute=$abstime;
            end
            analog I(p)<+(absolute*1e9+reported+ok)/1000.0;
            endmodule";
        let mut host =
            MixedSignalHost::compile(source, None, "clock", &[1], SchedulerLimits::default())
                .unwrap();
        host.add_adc_bridge("adc", 0, (2, 0), 0.4, 0.6).unwrap();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0, 0.0]);
        let stamp = |host: &mut MixedSignalHost| {
            while host.settle_analog_bridges(&[0.0, 0.6]).unwrap() {}
            let mut rhs = 0.0;
            host.stamp(
                &[0.0, 0.6],
                |_, _, _| {},
                |row, value| {
                    if row == 0 {
                        rhs += value;
                    }
                },
            )
            .unwrap();
            rhs
        };
        // This crossing reports tick 1 while its physical time is 0.65 ns.
        // X delay becomes inactive-region zero; the following real delay rounds
        // 0.5 module units to one tick. The independent tick-1 timer stays queued.
        for reject in [true, false] {
            host.begin_trial(
                0.65e-9,
                0.65e-9,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .unwrap();
            assert!((stamp(&mut host) + 3.65e-3).abs() < 1e-12);
            assert_eq!(host.read_digital("unrelated").unwrap(), "0");
            assert!((host.next_event_time().unwrap().unwrap() - 1e-9).abs() < 1e-20);
            if reject {
                host.reject_trial().unwrap();
                assert_eq!(host.read_digital("ok").unwrap(), "0");
                assert_eq!(host.read_digital("adc").unwrap(), "0");
            } else {
                host.accept_trial().unwrap();
            }
        }
        let checkpoint = host.checkpoint().unwrap();
        begin(&mut host, 1);
        assert!((stamp(&mut host) + 3.65e-3).abs() < 1e-12);
        assert_eq!(host.read_digital("unrelated").unwrap(), "1");
        host.accept_trial().unwrap();
        begin(&mut host, 2);
        assert!((stamp(&mut host) + 5e-3).abs() < 1e-12);
        host.accept_trial().unwrap();
        // Restoring accepted state must also discard the later activation clock.
        host.restore(&checkpoint).unwrap();
        begin(&mut host, 1);
        settle_and_accept(&mut host, &[0.0, 0.6]);
        begin(&mut host, 2);
        assert!((stamp(&mut host) + 5e-3).abs() < 1e-12);
        host.accept_trial().unwrap();
    }

    #[test]
    fn delayed_nonblocking_updates_survive_mixed_rejection_and_checkpoint() {
        let source = "module timed(p,adc); inout p; electrical p; input adc; wire adc;
            real held; reg issued, independent;
            initial begin held=0.0; issued=0; independent=0; independent<=#1 1; end
            always @(posedge adc) begin
                held <= #($realtime/2.0) $abstime;
                issued=1;
            end
            analog I(p)<+held*1e6+issued*1e-3;
            endmodule";
        let mut host =
            MixedSignalHost::compile(source, None, "nba", &[1], SchedulerLimits::default())
                .unwrap();
        host.add_adc_bridge("adc", 0, (2, 0), 0.4, 0.6).unwrap();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0, 0.0]);
        let stamp = |host: &mut MixedSignalHost| {
            while host.settle_analog_bridges(&[0.0, 0.6]).unwrap() {}
            let mut rhs = 0.0;
            host.stamp(
                &[0.0, 0.6],
                |_, _, _| {},
                |row, value| {
                    if row == 0 {
                        rhs += value;
                    }
                },
            )
            .unwrap();
            rhs
        };
        for reject in [true, false] {
            host.begin_trial(
                0.65e-9,
                0.65e-9,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .unwrap();
            assert!(
                (stamp(&mut host) + 1e-3).abs() < 1e-12,
                "continuation runs immediately, held value waits"
            );
            assert_eq!(host.read_digital("issued").unwrap(), "1");
            assert_eq!(
                host.read_digital("independent").unwrap(),
                "0",
                "unrelated tick-1 NBA stays queued"
            );
            assert!((host.next_event_time().unwrap().unwrap() - 1e-9).abs() < 1e-20);
            if reject {
                host.reject_trial().unwrap();
                assert_eq!(host.read_digital("issued").unwrap(), "0");
            } else {
                host.accept_trial().unwrap();
            }
        }
        let checkpoint = host.checkpoint().unwrap();
        for replay in 0..2 {
            if replay == 1 {
                host.restore(&checkpoint).unwrap();
            }
            begin(&mut host, 1);
            assert!((stamp(&mut host) + 1e-3).abs() < 1e-12);
            assert_eq!(host.read_digital("independent").unwrap(), "1");
            host.accept_trial().unwrap();
            assert!((host.next_event_time().unwrap().unwrap() - 2e-9).abs() < 1e-20);
            for reject in [true, false] {
                begin(&mut host, 2);
                assert!(
                    (stamp(&mut host) + 1.65e-3).abs() < 1e-12,
                    "delivered RHS keeps the captured 0.65 ns physical time"
                );
                if reject {
                    host.reject_trial().unwrap();
                } else {
                    host.accept_trial().unwrap();
                }
            }
            assert!(host.next_event_time().unwrap().is_none());
        }
    }

    #[test]
    fn computed_event_expressions_and_direct_captures_survive_mixed_rejection_and_checkpoint() {
        let source = "module events(p,arm,adc); inout p; electrical p;
            input arm,adc; wire arm,adc; real held,initial_held; reg requested,independent;
            initial begin
              held=0.0; initial_held=0.0; requested=0; independent=0;
              initial_held <= @(posedge adc) 0.25e-9;
              independent <= #2 1;
            end
            always @(posedge arm) begin held <= @(posedge adc) $abstime; requested=1; end
            analog I(p)<+(held+initial_held)*1e6+requested*1e-3;
            endmodule";
        for computed in [false, true] {
            let source = if computed {
                source
                    .replace("posedge arm", "posedge (arm & ($abstime > 0.5e-9))")
                    .replace("posedge adc", "posedge (adc & ($abstime > 1e-9))")
            } else {
                source.to_string()
            };
            let mut host =
                MixedSignalHost::compile(&source, None, "events", &[1], SchedulerLimits::default())
                    .unwrap();
            host.add_adc_bridge("arm", 0, (2, 0), 0.4, 0.6).unwrap();
            host.add_adc_bridge("adc", 0, (3, 0), 0.4, 0.6).unwrap();
            begin(&mut host, 0);
            settle_and_accept(&mut host, &[0.0, 0.0, 0.0]);
            let stamp = |host: &mut MixedSignalHost, adc: f64| {
                let voltages = [0.0, 0.6, adc];
                while host.settle_analog_bridges(&voltages).unwrap() {}
                let mut rhs = 0.0;
                host.stamp(
                    &voltages,
                    |_, _, _| {},
                    |row, value| {
                        if row == 0 {
                            rhs += value;
                        }
                    },
                )
                .unwrap();
                rhs
            };
            for reject in [true, false] {
                host.begin_trial(
                    0.65e-9,
                    0.65e-9,
                    IntegrationCoefficients::inactive(),
                    false,
                    false,
                )
                .unwrap();
                assert!((stamp(&mut host, 0.0) + 1e-3).abs() < 1e-12);
                assert_eq!(
                    host.read_digital("requested").unwrap(),
                    "1",
                    "event capture must not suspend its creator"
                );
                if reject {
                    host.reject_trial().unwrap();
                    assert_eq!(host.read_digital("requested").unwrap(), "0");
                } else {
                    host.accept_trial().unwrap();
                }
            }
            let checkpoint = host.checkpoint().unwrap();
            for replay in 0..2 {
                if replay == 1 {
                    host.restore(&checkpoint).unwrap();
                }
                for reject in [true, false] {
                    host.begin_trial(
                        1.65e-9,
                        1e-9,
                        IntegrationCoefficients::inactive(),
                        false,
                        false,
                    )
                    .unwrap();
                    assert!(
                        (stamp(&mut host, 0.6) + 1.9e-3).abs() < 1e-12,
                        "both captures retain their original real RHS"
                    );
                    assert_eq!(
                        host.read_digital("independent").unwrap(),
                        "0",
                        "physical event must not drain the unrelated tick-2 timer"
                    );
                    if reject {
                        host.reject_trial().unwrap();
                    } else {
                        host.accept_trial().unwrap();
                    }
                }
                assert!((host.next_event_time().unwrap().unwrap() - 2e-9).abs() < 1e-20);
                host.begin_trial(
                    2e-9,
                    0.35e-9,
                    IntegrationCoefficients::inactive(),
                    false,
                    false,
                )
                .unwrap();
                assert!((stamp(&mut host, 0.6) + 1.9e-3).abs() < 1e-12);
                assert_eq!(host.read_digital("independent").unwrap(), "1");
                host.accept_trial().unwrap();
                assert!(host.next_event_time().unwrap().is_none());
            }
        }
    }

    #[test]
    fn repeat_controls_restore_partial_counts_on_mixed_rejection_and_checkpoint() {
        let source = "module repeated(p,arm,adc); inout p; electrical p;
            input arm,adc; wire arm,adc; real held,initial_held,blocking_held; reg requested,done,independent;
            initial begin
              held=0.0; initial_held=0.0; blocking_held=0.0; requested=0; done=0; independent=0;
              initial_held <= repeat (2) @(posedge adc) 0.25e-9;
              independent <= #2 1;
            end
            always @(posedge arm) begin
              held <= repeat (2) @(posedge adc) $abstime; requested=1;
              blocking_held = repeat (2) @(posedge adc) $abstime; done=1;
            end
            analog I(p)<+(held+initial_held+blocking_held)*1e6+requested*1e-3;
            endmodule";
        for computed in [false, true] {
            let source = if computed {
                source.replace("posedge adc", "posedge (adc & ($abstime > 1e-9))")
            } else {
                source.to_string()
            };
            let mut host = MixedSignalHost::compile(
                &source,
                None,
                "repeated",
                &[1],
                SchedulerLimits::default(),
            )
            .unwrap();
            host.add_adc_bridge("arm", 0, (2, 0), 0.4, 0.6).unwrap();
            host.add_adc_bridge("adc", 0, (3, 0), 0.4, 0.6).unwrap();
            begin(&mut host, 0);
            settle_and_accept(&mut host, &[0.0, 0.0, 0.0]);
            let stamp = |host: &mut MixedSignalHost, adc: f64| {
                let voltages = [0.0, 0.6, adc];
                while host.settle_analog_bridges(&voltages).unwrap() {}
                let mut rhs = 0.0;
                host.stamp(
                    &voltages,
                    |_, _, _| {},
                    |row, value| {
                        if row == 0 {
                            rhs += value;
                        }
                    },
                )
                .unwrap();
                rhs
            };
            host.begin_trial(
                0.65e-9,
                0.65e-9,
                IntegrationCoefficients::inactive(),
                false,
                false,
            )
            .unwrap();
            assert!((stamp(&mut host, 0.0) + 1e-3).abs() < 1e-12);
            host.accept_trial().unwrap();
            // The first occurrence changes only the pending count. Rejection
            // must restore it without discarding the captured RHS or baseline.
            for reject in [true, false] {
                host.begin_trial(
                    1.15e-9,
                    0.5e-9,
                    IntegrationCoefficients::inactive(),
                    false,
                    false,
                )
                .unwrap();
                assert!((stamp(&mut host, 0.6) + 1e-3).abs() < 1e-12);
                assert_eq!(host.read_digital("done").unwrap(), "0");
                if reject {
                    host.reject_trial().unwrap();
                } else {
                    host.accept_trial().unwrap();
                }
            }
            let checkpoint = host.checkpoint().unwrap();
            for replay in 0..2 {
                if replay == 1 {
                    host.restore(&checkpoint).unwrap();
                }
                host.begin_trial(
                    1.35e-9,
                    0.2e-9,
                    IntegrationCoefficients::inactive(),
                    false,
                    false,
                )
                .unwrap();
                assert!((stamp(&mut host, 0.0) + 1e-3).abs() < 1e-12);
                host.accept_trial().unwrap();
                for reject in [true, false] {
                    host.begin_trial(
                        1.65e-9,
                        0.3e-9,
                        IntegrationCoefficients::inactive(),
                        false,
                        false,
                    )
                    .unwrap();
                    assert!((stamp(&mut host, 0.6) + 2.55e-3).abs() < 1e-12);
                    assert_eq!(host.read_digital("done").unwrap(), "1");
                    assert_eq!(host.read_digital("independent").unwrap(), "0");
                    if reject {
                        host.reject_trial().unwrap();
                    } else {
                        host.accept_trial().unwrap();
                    }
                }
                assert!((host.next_event_time().unwrap().unwrap() - 2e-9).abs() < 1e-20);
                host.begin_trial(
                    2e-9,
                    0.35e-9,
                    IntegrationCoefficients::inactive(),
                    false,
                    false,
                )
                .unwrap();
                assert!((stamp(&mut host, 0.6) + 2.55e-3).abs() < 1e-12);
                assert_eq!(host.read_digital("independent").unwrap(), "1");
                host.accept_trial().unwrap();
            }
        }
    }

    #[test]
    fn discrete_analog_inputs_restore_on_rejection_and_checkpoint() {
        let source = "module shared(p); inout p; electrical p; real state; initial begin state=0.25; #1 state=1.25; end analog I(p)<+state; endmodule";
        let mut host =
            MixedSignalHost::compile(source, None, "x", &[1], SchedulerLimits::default()).unwrap();
        let stamp = |host: &mut MixedSignalHost| {
            while host.settle_analog_bridges(&[0.0]).unwrap() {}
            let mut rhs = 0.0;
            host.stamp(&[0.0], |_, _, _| {}, |_, value| rhs += value)
                .unwrap();
            rhs
        };
        begin(&mut host, 0);
        assert_eq!(stamp(&mut host), -0.25);
        host.accept_trial().unwrap();
        let checkpoint = host.checkpoint().unwrap();
        begin(&mut host, 1);
        assert_eq!(stamp(&mut host), -1.25);
        host.reject_trial().unwrap();
        assert_eq!(
            host.analog
                .discrete_state_value(host.discrete_inputs[0].variable),
            Some(0.25)
        );
        assert_eq!(
            host.analog.checkpoint_state().unwrap(),
            checkpoint.analog_checkpoint
        );
        begin(&mut host, 1);
        assert_eq!(stamp(&mut host), -1.25);
        host.accept_trial().unwrap();
        host.restore(&checkpoint).unwrap();
        begin(&mut host, 1);
        assert_eq!(stamp(&mut host), -1.25);
    }

    #[test]
    fn rejected_probe_diagnostics_do_not_enter_accepted_state_or_checkpoints() {
        let mut host = host();
        let checkpoint = host.checkpoint().unwrap();
        let valid_rule = IntegrationCoefficients::backward_euler(1e-9).unwrap();
        let invalid_state = IntegrationCoefficients {
            derivative_scale: f64::NAN,
            ..valid_rule
        };
        host.begin_trial_with_integration_rules(
            1e-9,
            1e-9,
            valid_rule,
            invalid_state,
            false,
            false,
            true,
        )
        .unwrap_err();
        assert!(!host.trial_active());
        assert_eq!(
            host.analog.checkpoint_state().unwrap(),
            checkpoint.analog_checkpoint
        );
        assert_eq!(host.analog_inputs.time_seconds, 0.0);
        assert_eq!(
            host.analog_inputs.integration,
            IntegrationCoefficients::inactive()
        );
        assert_eq!(
            host.analog_inputs.state_integration,
            IntegrationCoefficients::inactive()
        );
        let adc = host.read_digital("adc");
        let q = host.read_digital("q");
        let sample = |host: &mut MixedSignalHost, time, voltage| {
            host.begin_probe_trial(time, 0.0, IntegrationCoefficients::inactive(), true, false)
                .unwrap();
            while host
                .settle_analog_bridges(&[0.0, 0.0, voltage, 0.0])
                .unwrap()
            {}
            host.reject_trial().unwrap();
        };
        for _ in 0..4 {
            sample(&mut host, 0.0, 0.0);
            sample(&mut host, 0.0, 1.0);
        }
        // A later unsuccessful damping plateau must not erase observed flips.
        for _ in 0..20 {
            sample(&mut host, 0.0, 0.0);
        }
        let detail = host.rejected_probe_activity(0.0).unwrap();
        assert!(detail.contains("0 1 0 1"), "{detail}");
        assert!(detail.contains("28 rejected solver probes"), "{detail}");
        assert_eq!(host.read_digital("adc"), adc);
        assert_eq!(host.read_digital("q"), q);
        assert_eq!(host.state.accepted_time, 0.0);
        assert!(!host.state.started);
        assert!(host.state.adc_history.iter().all(|entry| entry.filled == 0));
        assert!(host.state.dac_history.iter().all(|entry| entry.filled == 0));
        assert!(host.state.digital.next_tick().is_none());
        let after_probes = host.checkpoint().unwrap();
        host.restore(&after_probes).unwrap();
        assert!(host.rejected_probe_activity(0.0).is_none());

        for _ in 0..4 {
            sample(&mut host, 0.0, 0.0);
            sample(&mut host, 0.0, 1.0);
        }
        sample(&mut host, 1e-9, 0.0);
        assert!(host.rejected_probe_activity(0.0).is_none());
        assert!(host.rejected_probe_activity(1e-9).is_none());
        host.restore(&checkpoint).unwrap();
        for _ in 0..4 {
            sample(&mut host, 0.0, 0.0);
            sample(&mut host, 0.0, 1.0);
        }
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        assert!(host.rejected_probe_activity(0.0).is_none());
    }

    #[test]
    fn reinitialization_resets_both_domains_and_retains_boundary_wiring() {
        let mut reused = host();
        begin(&mut reused, 0);
        settle_and_accept(&mut reused, &[0.0, 0.0, 1.0, 0.0]);
        assert_eq!(reused.read_digital("q").unwrap(), "1");
        assert!(reused.state.digital.next_tick().is_some());

        reused.begin_analog_analysis(2).unwrap();
        assert!(!reused.digital_started);
        assert!(!reused.state.started);
        assert_eq!(reused.state.accepted_time, 0.0);
        assert!(reused.state.digital.next_tick().is_none());
        assert!(reused.checkpoint().is_err());
        assert!(
            reused
                .begin_trial(0.0, 0.0, IntegrationCoefficients::inactive(), true, false)
                .is_err()
        );
        reused.start_digital_execution().unwrap();
        assert_eq!(reused.read_digital("q").unwrap(), "0");
        assert_eq!(reused.state.bridges.adc.len(), 1);
        assert_eq!(reused.state.bridges.dac.len(), 1);
        let mut fresh = host();
        for tick in [0, 2, 3] {
            let voltages = [0.0, 0.0, if tick == 0 { 1.0 } else { 0.0 }, 0.0];
            for host in [&mut reused, &mut fresh] {
                begin(host, tick);
                settle_and_accept(host, &voltages);
            }
            for signal in ["q", "dac"] {
                assert_eq!(
                    reused.read_digital(signal).unwrap(),
                    fresh.read_digital(signal).unwrap()
                );
            }
            assert_eq!(
                reused.state.digital.next_tick(),
                fresh.state.digital.next_tick()
            );
            assert_eq!(
                reused.analog.checkpoint_state().unwrap(),
                fresh.analog.checkpoint_state().unwrap()
            );
        }
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

    /// Settle a trial without committing it, and report the root it asks for.
    fn boundary_root_of_trial(host: &mut MixedSignalHost, voltages: &[f64]) -> Option<f64> {
        while host
            .settle_analog_bridges(voltages)
            .expect("bridges settle")
        {}
        let target = host
            .trial_boundary_refinement_time(1.0e-20)
            .expect("the inspection succeeds");
        host.reject_trial().expect("a settled trial rolls back");
        target
    }

    /// A step a D/A bridge moved in carries no interior A/D root.
    ///
    /// The voltage moved because the bridge moved it, at the tick it moved on,
    /// so there is nothing for the analog solver to go back and find: the
    /// interval's own solution never passed through the threshold. Asking for
    /// one is the refinement storm — the solver rejects the step, halves it,
    /// lands on the same event again, and repeats until it reaches its floor.
    #[test]
    fn a_step_a_boundary_moved_in_has_no_interior_root() {
        let mut host = host();
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        begin(&mut host, 1);
        settle_and_accept(&mut host, &[0.0; 4]);
        assert_eq!(
            host.next_event_time().expect("the wheel is readable"),
            None,
            "this interval has to start with an empty wheel to be a control"
        );

        // A rising crossing inside (1 ns, 2 ns] with no boundary moving: an
        // interior root, and the solver is asked to land on it.
        let rising = [0.0, 0.0, 1.0, 0.0];
        begin(&mut host, 2);
        let target = boundary_root_of_trial(&mut host, &rising)
            .expect("a crossing with no D/A movement is an interior root");
        assert!(
            target > 1.0e-9 && target < 2.0e-9,
            "the root must be inside the step, got {target:e}"
        );

        // The same crossing in a trial the circuit reports a D/A movement for
        // — another enrolled instance's bridge on this node — is that
        // movement's own transition, not a root inside the interval.
        begin(&mut host, 2);
        host.note_shared_digital_feedback();
        assert_eq!(
            boundary_root_of_trial(&mut host, &rising),
            None,
            "a boundary that moved inside the step leaves no interior root"
        );

        // Publish the crossing for real. `MIXED`'s edge process drives `dac`
        // from `q`, so this trial moves a D/A bridge of its own and the wheel
        // is left due at tick 4 by the `#2`.
        begin(&mut host, 2);
        settle_and_accept(&mut host, &rising);
        let resume = host
            .next_event_time()
            .expect("the wheel is readable")
            .expect("the crossing woke the edge process onto its `#2`");
        assert_eq!(
            (resume / 1.0e-9).round() as u64,
            2 + EDGE_PROCESS_DELAY_TICKS,
            "the resume must be the `#2` after the publication tick, got {resume:e}"
        );
    }

    /// The other half of the same rule: a step the discrete half moved an
    /// *analog-read variable* in carries no interior root towards the crossing
    /// that movement caused either.
    ///
    /// The feedback path here is a variable rather than a D/A bridge — this
    /// module declares no D/A bridge at all — which is exactly what a rule
    /// watching only bridge bits misses. `ca`'s own crossing is interior and
    /// stays interior: nothing digital had moved when the first iterate found
    /// it. `cb`'s is found on the re-solve `gain` caused, and interpolating it
    /// over the whole step would ask the controller to land at 1.2 ns —
    /// *before* the 1.6 ns edge that caused it, on a solution that never
    /// existed. The root the controller is given is `ca`'s, and only `ca`'s.
    #[test]
    fn a_step_a_discrete_read_moved_in_has_no_interior_root() {
        // `gain` is the analog-read discrete quantity; `ca` and `cb` are A/D
        // inputs on nodes 3 and 4. `spun` would be the control's shape — a
        // variable no analog equation reads — and is deliberately absent, so
        // every digital write here reaches the continuous problem.
        let source = r#"
module discrete_feedback(p, n, ca, cb);
  inout p, n;
  electrical p, n;
  input ca, cb;
  wire ca, cb;
  integer gain;
  initial gain = 1;
  always @(posedge ca) gain = 2;
  analog I(p, n) <+ gain * V(p, n) / 1000.0;
endmodule
"#;
        let mut host = MixedSignalHost::compile(
            source,
            None,
            "xfeedback",
            &[1, 0],
            SchedulerLimits::default(),
        )
        .expect("the feedback module compiles and starts");
        host.add_adc_bridge("ca", 0, (3, 0), 0.4, 0.6)
            .expect("the first A/D bridge");
        host.add_adc_bridge("cb", 0, (4, 0), 0.4, 0.6)
            .expect("the second A/D bridge");

        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        begin(&mut host, 1);
        settle_and_accept(&mut host, &[0.0; 4]);

        // The control: the same `cb` ramp with nothing digital moving ahead of
        // it. 0 V to 3 V across (1 ns, 2 ns] puts the 0.6 V threshold a fifth
        // of the way in, and the controller is asked to land there.
        begin(&mut host, 2);
        let control = boundary_root_of_trial(&mut host, &[0.0, 0.0, 0.0, 3.0])
            .expect("a crossing with no feedback before it is an interior root");
        assert!(
            (control - 1.2e-9).abs() < 1.0e-12,
            "the control must be the interpolated crossing at 1.2 ns, got {control:e}"
        );

        begin(&mut host, 2);
        // The first iterate: `ca` crosses six tenths of the way through the
        // step, and the process it wakes writes `gain`.
        while host
            .settle_analog_bridges(&[0.0, 0.0, 1.0, 0.0])
            .expect("bridges settle")
        {}
        // The re-solve that feedback caused, over the same interval: `cb`
        // crosses on it, and a plain interpolation would date that at 1.2 ns.
        while host
            .settle_analog_bridges(&[0.0, 0.0, 1.0, 3.0])
            .expect("bridges settle")
        {}
        let target = host
            .trial_boundary_refinement_time(1.0e-20)
            .expect("the inspection succeeds");
        host.reject_trial().expect("a settled trial rolls back");
        let target = target.expect("`ca`'s own crossing is still an interior root");
        assert!(
            (target - 1.6e-9).abs() < 1.0e-12,
            "the only interior root is `ca`'s own crossing at 1.6 ns, got {target:e}"
        );
        assert!(
            target > control,
            "the controller must not be sent back to the {control:e} s artefact the \
             re-solve produced, and was sent to {target:e} s"
        );
    }

    /// The control the whole distinction rests on: the discrete half runs
    /// inside the interval and moves no boundary, while the circuit carries an
    /// analog input across a threshold.
    ///
    /// A divider, a counter or a state machine ticking is not a reason to stop
    /// interpolating. That crossing is the circuit's own, it keeps its
    /// interpolated instant and its interior root, and dating it at the
    /// endpoint instead would be a setup/hold-class error on a sampling edge —
    /// which is what gating on "the discrete half ran" rather than on "a
    /// boundary moved" would have done to it.
    #[test]
    fn a_wheel_that_moves_no_boundary_leaves_the_circuits_own_crossing_interior() {
        // `spin` schedules an activation at tick 3 that assigns a variable no
        // bridge reads. `dac` is written once, in `initial`, so the D/A
        // boundary is fixed for the whole run.
        let source = r#"
module spin(p, n, adc, dac);
  inout p, n;
  electrical p, n;
  input adc; wire adc;
  output dac; reg dac;
  reg spun;
  initial begin dac = 1'b0; spun = 1'b0; #3 spun = 1'b1; end
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;
        let mut host =
            MixedSignalHost::compile(source, None, "xspin", &[1, 0], SchedulerLimits::default())
                .expect("the control module compiles and starts");
        host.add_adc_bridge("adc", 0, (3, 0), 0.4, 0.6)
            .expect("A/D bridge");
        host.add_dac_bridge("dac", 0, (4, 0), 0.0, 5.0, 100.0)
            .expect("D/A bridge");

        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        let due = host
            .next_event_time()
            .expect("the wheel is readable")
            .expect("the module scheduled its tick-3 activation");
        assert_eq!(
            (due / 1.0e-9).round() as u64,
            3,
            "the control needs an activation inside the step, got {due:e}"
        );

        // One step from zero that ends ON the activation, which is where the
        // step controller has to put it: a trial ending past a scheduled
        // activation is the missed breakpoint `begin_trial` refuses. The
        // endpoint is the wheel's own instant rather than the `3.0e-9` that
        // spells it, because `3.0e-9` parses a hair below three ticks and
        // would floor into tick 2 — the activation would then not run inside
        // the step at all and this would stop being a control.
        host.begin_trial(due, due, IntegrationCoefficients::inactive(), false, false)
            .expect("the crossing step begins");
        while host
            .settle_analog_bridges(&[0.0, 0.0, 1.0, 0.0])
            .expect("bridges settle")
        {}
        // Vacuity: the discrete half has to have run inside this interval, or
        // the case says nothing about a wheel that ticks quietly.
        assert_eq!(
            host.next_event_time().expect("the wheel is readable"),
            None,
            "the tick-3 activation must run inside this trial for it to be a control"
        );
        let target = host
            .trial_boundary_refinement_time(1.0e-20)
            .expect("the inspection succeeds");
        host.reject_trial().expect("a settled trial rolls back");
        let target = target.expect(
            "a wheel that ticks without moving a boundary must leave the circuit's own \
             crossing an interior root",
        );
        assert!(
            target > 0.0 && target < due,
            "the root must be inside the step, got {target:e}"
        );
        // And it is the interpolated crossing, not the endpoint: 0 V to 1 V
        // across the step puts the 0.6 V threshold at 1.8 ns.
        assert!(
            (target - 1.8e-9).abs() < 1.0e-12,
            "the root must be the interpolated crossing at 1.8 ns, got {target:e}"
        );
    }

    /// A module with no schedule of its own: its D/A output moves only because
    /// the analog side moved its A/D input, which is the loop
    /// [`MAX_CONSECUTIVE_BOUNDARY_FLIPS`] exists for.
    const UNSCHEDULED_FLIP: &str = r#"
module unscheduled_flip(p, n, adc, dac);
  inout p, n;
  electrical p, n;
  input adc; wire adc;
  output dac; reg dac;
  initial dac = 1'b0;
  always @(adc) dac = ~dac;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

    /// The same boundary, moved by the module's own femtosecond-scale clock
    /// instead: one activation per tick, and the analog side takes no part.
    const SCHEDULED_FLIP: &str = r#"
module scheduled_flip(p, n, dac);
  inout p, n;
  electrical p, n;
  output dac; reg dac;
  initial dac = 1'b0;
  always #1 dac = ~dac;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

    fn flip_host(source: &str, instance: &str, adc: bool) -> MixedSignalHost {
        let mut host =
            MixedSignalHost::compile(source, None, instance, &[1, 0], SchedulerLimits::default())
                .expect("the flip module compiles and starts");
        if adc {
            host.add_adc_bridge("adc", 0, (3, 0), 0.4, 0.6)
                .expect("A/D bridge");
        }
        host.add_dac_bridge("dac", 0, (4, 0), 0.0, 5.0, 100.0)
            .expect("D/A bridge");
        host
    }

    /// **The accepted-flip ceiling, kept.** A boundary that moves at every
    /// accepted timepoint with nothing scheduled to explain it is still
    /// refused at the ceiling, with the count in the message.
    ///
    /// The deck-level fixtures for cross-domain feedback all refuse at
    /// *startup*, from the rejected-probe diagnostic, so none of them reaches
    /// this path at all: an accepted-history refusal needs a loop that settles
    /// inside each timepoint and moves again at the next one. It is driven
    /// here from the module's own interface, which is the only place that
    /// shape can be produced deterministically — and it is the case the
    /// schedule exclusion had to leave alone.
    #[test]
    fn a_boundary_flip_run_refuses_a_move_no_schedule_explains() {
        let mut host = flip_host(UNSCHEDULED_FLIP, "xunscheduled", true);
        let mut refusal = None;
        for tick in 0..(u64::from(MAX_CONSECUTIVE_BOUNDARY_FLIPS) + 16) {
            begin(&mut host, tick);
            let sense = if tick % 2 == 1 { 1.0 } else { 0.0 };
            while host
                .settle_analog_bridges(&[0.0, 0.0, sense, 0.0])
                .expect("bridges settle")
            {}
            if let Err(error) = host.accept_trial() {
                refusal = Some((tick, error));
                break;
            }
        }
        let (tick, error) = refusal
            .expect("a boundary the analog side moves at every accepted timepoint must be refused");
        assert_eq!(
            host.next_event_time().expect("the wheel is readable"),
            None,
            "the module must have nothing scheduled, or the refusal proves nothing about \
             an unexplained move"
        );
        let MixedSignalError::BoundaryOscillation {
            limit,
            within_one_timepoint,
            nets,
            ..
        } = &error
        else {
            panic!("the wrong refusal at tick {tick}: {error}");
        };
        assert_eq!(*limit, MAX_CONSECUTIVE_BOUNDARY_FLIPS);
        assert!(
            !*within_one_timepoint,
            "this is the accepted-history ceiling, not the settle one: {error}"
        );
        let moves = u64::from(MAX_CONSECUTIVE_BOUNDARY_FLIPS) + 1;
        assert!(
            nets.iter()
                .any(|net| net.contains(&format!("moved {moves} times"))
                    && net.contains("driven by the module")),
            "the refusal must report the run that tripped the ceiling: {error}"
        );
    }

    /// **The schedule's own tick, not counted.** A clock that moves its D/A
    /// bridge at every accepted timepoint runs past the ceiling.
    ///
    /// One activation per tick and one accepted timepoint per activation is
    /// what a stepper does with a clock whose period is a few minimum steps,
    /// and it used to be refused here as a zero-delay loop at the 129th point
    /// — the analog side having taken no part in any of them. The vacuity
    /// guard is the move count: the boundary has to actually move at more
    /// accepted points than the ceiling for the run to say anything.
    #[test]
    fn a_boundary_flip_run_does_not_count_a_scheduled_clocks_own_tick() {
        let mut host = flip_host(SCHEDULED_FLIP, "xscheduled", false);
        begin(&mut host, 0);
        settle_and_accept(&mut host, &[0.0; 4]);
        let mut previous = host.read_digital("dac").expect("dac is readable");
        let mut moves = 0_u32;
        for tick in 1..(u64::from(MAX_CONSECUTIVE_BOUNDARY_FLIPS) + 40) {
            begin(&mut host, tick);
            settle_and_accept(&mut host, &[0.0; 4]);
            let now = host.read_digital("dac").expect("dac is readable");
            if now != previous {
                moves += 1;
            }
            previous = now;
        }
        assert!(
            moves > MAX_CONSECUTIVE_BOUNDARY_FLIPS,
            "the clock has to move the boundary at more accepted timepoints than the \
             ceiling for this to be the case it is about, saw {moves}"
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
        // `dac` is a `reg` no process has assigned yet: the one process that
        // would assign it waits on `trig`, which nothing drives, so it has not
        // run at the instant the bridge is first read. That is the ordinary
        // shape of a design that does not initialise its boundary output.
        //
        // The sensitivity is a discrete net rather than the module's own
        // `electrical p`, because `@(p)` on an analog net is not a digital
        // event control at all — Verilog-AMS LRM 2.4 reaches an analog
        // quantity from the discrete domain through an A/D connect, and
        // reaches an analog *event* through `cross`/`above` inside an analog
        // block. The front end refuses it, and what this test is about is the
        // undefined level, not the sensitivity that leaves it undefined.
        let source = r#"
module undriven(p, n, trig, dac);
  inout p, n; electrical p, n;
  input trig; wire trig;
  output dac; reg dac;
  always @(trig) dac <= 1'b0;
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
