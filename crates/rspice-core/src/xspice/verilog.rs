//! Native execution of digital IEEE 1364-2005 Verilog.
//!
//! A caller hands this module Verilog source and a stimulus and gets back one
//! observation per vector. Everything between — compiling the source to a
//! [`CanonicalDigitalPlan`], resolving its nets, running its processes on the
//! event kernel — is here or in the two submodules below it.
//!
//! # Two entry points, one for each shape of caller
//!
//! [`run_digital_verilog`] is the one-call route and compiles on every call.
//! [`CompiledDigitalDesign`] is the same route with the compile hoisted out, for
//! a caller with many stimuli and one design; the one-call route is literally
//! the composition of its two halves. See its documentation for what is shared
//! between two runs of one compiled design (the plan, which is immutable) and
//! what is not (all of the running state).
//!
//! [`CompiledDigitalDesign::link`] combines compiled instances into a single
//! executable plan. Connected net ports share signals and drivers; variable
//! outputs contribute through continuous assignments. The resulting design
//! runs through the same host, with one scheduling region order and precision.
//!
//! # Why it lives under `xspice`
//!
//! Not because it is an XSPICE code model; it is not. Because of the layering
//! ratchet in `tests/module_layering.rs`. This host is built on
//! [`event_scheduler`](super::event_scheduler), the discrete-event kernel,
//! which sits at rank 9 for reasons its own module documentation gives. A
//! module that names the kernel must sit strictly above rank 9, and the only
//! rank above it is `circuit`'s — which this has no business being part of.
//! Sitting *beside* the kernel is the honest placement available today: same
//! rank, no upward edge, and no rank renumbering in a file several lanes share.
//!
//! # The time-unit ruling
//!
//! Each module retains its declared time unit and precision through elaboration.
//!
//! Mixed modules execute through [`MixedSignalHost`] and the circuit-wide mixed
//! coordinator. Analog candidates and digital settlement participate in one
//! accepted/rejected trial. Physical event times remain separate from the
//! declared HDL time precision; advancing a reporting tick does not execute a
//! future physical timer.
//!
//! Typed event-only HDL and XSPICE ports share resolved signals directly. At an
//! electrical boundary, the builder allocates a private event endpoint and a
//! converter with the selected loading, timing, and threshold behavior. Each
//! vector conductor retains its identity, while simultaneous input decisions
//! publish as one bank. Four-state bidirectional conversion tracks external
//! drivers independently of its own electrical contribution.
//!
//! # Real-valued nets and connection bodies
//!
//! Shared real nets retain their declared resolution and publish accepted real
//! event traces. HDL and XSPICE outputs each contribute their original driver;
//! resolved observations never become additional drivers. Standard `wreal`
//! enforces one driver. Explicit RNM resolution extensions can combine drivers.
//!
//! The builder supplies default `real_to_v` and `v_to_real` conversions for
//! unidirectional electrical boundaries. Selected authored connection bodies
//! override those defaults and execute through the same mixed host. A physical
//! real inout requires an authored bidirectional body or an explicit real inout
//! template defining its electrical behavior and driver ownership.
//!
//! Connection-rule selection distinguishes real and four-state ports as well
//! as discipline and direction. Built-in delegation requires an equivalent
//! shipped declaration; a matching module name does not replace an authored
//! body. Mixed child modules retain their equations and discrete processes.
//! Automatic connection insertion at internal HDL hierarchy boundaries remains
//! separate compiler elaboration work.
//!
//! [`DigitalHost::advance_to`]: host::DigitalHost::advance_to

pub(crate) mod effects;
pub(crate) mod host;
#[cfg(test)]
mod linked_tests;
mod mixed;
pub(crate) mod store;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan;
use rspice_veriloga::canonical_ir::digital_link::DigitalLinkNet;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};

use host::DigitalHost;
pub use host::DigitalRunError;
pub(crate) use mixed::checkpoint as participant_checkpoint;
pub(crate) use mixed::coordinator_checkpoint;
pub(crate) use mixed::{
    BoundaryBitSource, BoundaryBus, CandidateLedger, CoordinatorEquilibriumImage,
    HostEquilibriumImage, MixedDigitalCoordinator, PreparedMixedAcceptance, SharedTrialCursor,
};
pub use mixed::{MixedSignalCheckpoint, MixedSignalError, MixedSignalHost};

/// What one tick of the digital host's clock is, and why it is not read from
/// the source.
///
/// Default scale for existing sources with no explicit declaration.
#[cfg(test)]
const TIME_UNIT_EXPONENT: i8 = -9;

/// One port of the design, as a stimulus names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalPort {
    /// The port's name in the design.
    pub name: String,
    /// Declared width in bits. A scalar is one, and **zero is a real-valued
    /// (`wreal`) port**, which Verilog-AMS LRM 2.4 section 3.7 gives no bits at
    /// all.
    ///
    /// Zero rather than an `Option` or a second field, because the compiler
    /// already spells "has no bit width" that way — a process-local `real` and
    /// a real net both carry width zero through the whole front end — and two
    /// spellings of one fact are two chances for them to disagree.
    ///
    /// It is not what decides the port's domain. The *design* decides: the
    /// net-type keyword its author wrote is read back from the compiled plan,
    /// and a stimulus whose width says otherwise is refused with
    /// [`DigitalRunError::StimulusValueDomain`] rather than believed.
    pub width: u32,
}

/// A free-running clock the stimulus drives rather than the vectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalClock {
    /// The input port carrying the clock.
    pub port: String,
    /// Half period, in compiled design ticks.
    pub half_period: u64,
}

/// Everything a caller supplies alongside the source.
///
/// The timing rule is the oracle harness's, restated so that this crate does
/// not have to name the harness's types: inputs for vector `k` are applied at
/// `k * step`, and outputs are sampled at `k * step + settle`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalStimulus {
    /// Which module of the source to run, for a multi-module file.
    pub module: Option<String>,
    /// Inputs the vector columns drive, in column order. The clock is not one
    /// of these.
    pub inputs: Vec<DigitalPort>,
    /// Ports sampled into the trace, in the order they appear in an
    /// observation.
    pub outputs: Vec<DigitalPort>,
    /// A clock the stimulus drives, if the design has one.
    pub clock: Option<DigitalClock>,
    /// Design ticks between successive vectors.
    pub step: u64,
    /// Design ticks after a vector is applied at which outputs are sampled.
    pub settle: u64,
    /// One entry per vector; each is one four-state spelling per driven input,
    /// most significant bit first, exactly as wide as the port.
    pub vectors: Vec<Vec<String>>,
}

/// One sample of every observed output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalObservation {
    /// Zero-based vector index this observation belongs to.
    pub step: usize,
    /// Output name and its rendered value, in stimulus order.
    ///
    /// A four-state port renders as its `%b` spelling over `0 1 x z`. A real
    /// port renders as a decimal that reads back to the same `f64` exactly —
    /// Rust's shortest round-tripping form — so a trace can be compared
    /// textually without a tolerance deciding what counts as equal.
    pub values: Vec<(String, String)>,
}

/// The result of one digital run.
///
/// # One observation per vector, and what that is not
///
/// The resolution is the stimulus's, not the host's. A design may change a
/// signal many times between two sample instants — a `#delay` chain, a clock
/// the stimulus drives, a delta cycle — and none of it appears here; what
/// appears is the value at `k * step + settle`, which is what a `.stim`-style
/// harness asks for.
///
/// A caller that wants the transitions themselves has no route to them today.
/// The compile-once split does not open one: [`CompiledDigitalDesign::run`]
/// builds and drops its host inside the call, so there is nothing left for a
/// reader to read afterwards, and a per-tick observation would have to be
/// collected *during* the run — a callback or a signal subscription on the
/// stimulus, not an accessor on this. That is a change to what a run is asked
/// for rather than a change to what it returns, so it is left undone and
/// recorded here rather than approximated with a finer `step`, which would
/// change what the design sees as well as what the caller sees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalRunReport {
    /// One observation per vector, in order.
    pub observations: Vec<DigitalObservation>,
}

/// One digital design, compiled once and runnable any number of times.
///
/// # Why the split exists
///
/// [`run_digital_verilog`] runs the Verilog front end on every call. For a
/// caller with one stimulus that is the honest cost of the run, and it is what
/// the convenience call still charges. For a caller with many — a sweep, a
/// regression harness, a measurement that wants to know what *evaluating* a
/// real-number model costs — it is a compile repeated for nothing, and at
/// realistic run lengths it is most of the wall time. `.../tests/
/// verilog_rnm_performance.rs` measures both columns side by side, which is the
/// evidence this split was worth making.
///
/// # What is shared between runs, and what is not
///
/// Shared: the compiled plan, which is immutable — it is the front end's output
/// and nothing downstream writes to it. Not shared: **every** piece of running
/// state. [`Self::run`] builds a fresh [`DigitalHost`] — signal store, event
/// queue, process resumption slots, sensitivity index — for each call and drops
/// it when the call returns, so there is no state for one run to leave behind
/// for the next and no way for two designs' runs to interleave into each other.
/// That is a structural property rather than a discipline: the only thing a run
/// can reach across a call boundary is behind an `Arc` it holds by shared
/// reference.
///
/// # The relationship to [`MixedSignalHost`]
///
/// [`MixedSignalHost::compile`] is already compile-once by construction — it
/// keeps its `DigitalHost` for the whole of an outer transient, because the
/// analog solver drives it one trial at a time and the digital state has to
/// survive between trials. This type gives the vector-driven route the same
/// shape for the opposite reason: its state must *not* survive between runs.
/// The two meet at [`DigitalHost::from_plan`], which is the one place a
/// compiled plan becomes a running host; neither builds its own.
pub struct CompiledDigitalDesign {
    /// The front end's output, shared by every host built from it.
    plan: Arc<CanonicalDigitalPlan>,
    ports: Vec<rspice_veriloga::canonical_ir::digital_link::DigitalLinkPort>,
    signal_aliases: std::collections::BTreeMap<String, DigitalSignalId>,
    /// The module that was compiled, so a stimulus naming another one is
    /// refused rather than silently run against this.
    module: String,
    /// Fixed here rather than per run, so two runs of one design cannot be on
    /// different time bases. See the compiler-resolved module timing.
    resolution: TimeResolution,
}

impl CompiledDigitalDesign {
    /// Duration of one stimulus/event tick in this compiled design.
    pub fn time_resolution(&self) -> TimeResolution {
        self.resolution
    }

    /// Compile one module of a digital Verilog source.
    ///
    /// Every refusal [`run_digital_verilog`] makes before anything runs is made
    /// here: the front end, followed by the checks that the module is digital
    /// and only digital. Active timing directives are handled by the compiler.
    pub fn compile(source: &str, module: Option<&str>) -> Result<Self, DigitalRunError> {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let artifact = compiler
            .compile_canonical_ir_module(source, module)
            .map_err(|error| DigitalRunError::Compile {
                detail: error.to_string(),
            })?;

        if !artifact.digital.has_executable_content() {
            return Err(DigitalRunError::NoDigitalContent {
                module: artifact.mir.module_name.to_string(),
            });
        }
        if !artifact.mir.equations.is_empty() || !artifact.digital.absdelta.is_empty() {
            return Err(DigitalRunError::MixedSignalModule {
                module: artifact.mir.module_name.to_string(),
                equations: artifact.mir.equations.len(),
            });
        }

        use rspice_veriloga::canonical_ir::digital_link::{DigitalLinkDirection, DigitalLinkPort};
        let mut ports = Vec::new();
        for port in &artifact.hir.ports {
            let Some(signal) = artifact
                .digital
                .signals
                .iter()
                .find(|signal| signal.name == port.name)
            else {
                continue;
            };
            let direction = match port.direction {
                rspice_veriloga::ast::PortDirection::Input => DigitalLinkDirection::Input,
                rspice_veriloga::ast::PortDirection::Output => DigitalLinkDirection::Output,
                rspice_veriloga::ast::PortDirection::Inout => DigitalLinkDirection::Inout,
            };
            ports.push(DigitalLinkPort {
                name: port.name.to_string(),
                signal: signal.id,
                direction,
            });
        }
        Ok(Self {
            ports,
            signal_aliases: Default::default(),
            module: artifact.mir.module_name.to_string(),
            resolution: TimeResolution::new(artifact.digital.timing.precision_exponent)?,
            plan: Arc::new(artifact.digital),
        })
    }

    /// Connect compiled instances into one executable digital design. Net ports
    /// share resolution directly; variable outputs retain separate storage and
    /// drive their connections continuously. Inputs may be supplied through a
    /// named net or a qualified instance signal such as `controller.clk`.
    ///
    /// Connections are whole ports with matching domains and widths. Different
    /// packed ranges are positional. Stimulus ticks use the finest precision of
    /// all linked instances; each module keeps its own delay and time units.
    pub fn link(
        name: &str,
        instances: &[(&str, &Self)],
        nets: &[DigitalLinkNet],
    ) -> Result<Self, DigitalRunError> {
        Self::link_with_control(name, instances, nets, &rspice_veriloga::NoPipelineControl)
    }

    /// The cancellable form used by a circuit or worker during elaboration.
    pub fn link_with_control(
        name: &str,
        instances: &[(&str, &Self)],
        nets: &[DigitalLinkNet],
        control: &dyn rspice_veriloga::PipelineControl,
    ) -> Result<Self, DigitalRunError> {
        use rspice_veriloga::canonical_ir::digital_link::{
            DigitalLinkDirection, DigitalLinkInstance, DigitalLinkPort, link_digital_plans,
        };
        if name.is_empty() {
            return Err(DigitalRunError::Compile {
                detail: "linked design name must not be empty".into(),
            });
        }
        let inputs: Vec<_> = instances
            .iter()
            .map(|(name, design)| DigitalLinkInstance {
                name,
                plan: &design.plan,
                ports: &design.ports,
            })
            .collect();
        let linked = link_digital_plans(&inputs, nets, control).map_err(|diagnostics| {
            DigitalRunError::Compile {
                detail: diagnostics
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; "),
            }
        })?;
        let mut signal_aliases = linked.signal_names;
        // Retain aliases through repeated linking, including collapsed net names.
        for instance in &linked.instances {
            let design = instances
                .iter()
                .find(|(name, _)| *name == instance.name)
                .unwrap()
                .1;
            for (alias, original) in &design.signal_aliases {
                let name = format!("{}.{}", instance.name, alias);
                let signal = instance.signals[usize::from(*original)];
                if signal_aliases
                    .insert(name.clone(), signal)
                    .is_some_and(|old| old != signal)
                {
                    return Err(DigitalRunError::Compile {
                        detail: format!("linked name '{name}' identifies different signals"),
                    });
                }
            }
        }
        let ports = nets
            .iter()
            .map(|net| DigitalLinkPort {
                name: net.name.clone(),
                signal: signal_aliases[&net.name],
                direction: DigitalLinkDirection::Inout,
            })
            .collect();
        Ok(Self {
            module: name.into(),
            ports,
            signal_aliases,
            resolution: TimeResolution::new(linked.plan.timing.precision_exponent)?,
            plan: Arc::new(linked.plan),
        })
    }

    /// Run one stimulus against this design.
    ///
    /// Nothing of a previous run reaches this one: the host is built here and
    /// dropped at the end, so the design starts at time zero with every process
    /// unstarted and every signal at its declared initial value, exactly as a
    /// fresh [`run_digital_verilog`] would.
    pub fn run(&self, stimulus: &DigitalStimulus) -> Result<DigitalRunReport, DigitalRunError> {
        if let Some(requested) = stimulus.module.as_deref()
            && requested != self.module
        {
            return Err(DigitalRunError::StimulusModule {
                compiled: self.module.clone(),
                requested: requested.to_string(),
            });
        }
        self.execute(stimulus)
    }
}

/// What the design is and how big it is, not what it contains.
///
/// Written out rather than derived because the derived form is the whole
/// canonical plan — every process's control-flow graph — which is what a
/// compiler dump is for and not what a failing assertion should print.
impl std::fmt::Debug for CompiledDigitalDesign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledDigitalDesign")
            .field("module", &self.module)
            .field("signals", &self.plan.signals.len())
            .field("processes", &self.plan.processes.len())
            .finish()
    }
}

/// Compile digital Verilog and run it against a stimulus.
///
/// The whole route, in one call: preprocess and compile the source to a
/// canonical digital plan, build a signal store and a process host over it,
/// start every process at time zero, then walk the stimulus applying inputs and
/// sampling outputs on the timing rule [`DigitalStimulus`] states.
///
/// It is the composition of [`CompiledDigitalDesign::compile`] and
/// [`CompiledDigitalDesign::run`] and nothing else, so a caller with one
/// stimulus keeps the one-call route and a caller with many can hoist the
/// compile out of its loop.
pub fn run_digital_verilog(
    source: &str,
    stimulus: &DigitalStimulus,
) -> Result<DigitalRunReport, DigitalRunError> {
    CompiledDigitalDesign::compile(source, stimulus.module.as_deref())?.execute(stimulus)
}

impl CompiledDigitalDesign {
    /// The run itself, with the module agreement already decided.
    ///
    /// Split from [`Self::run`] so that [`run_digital_verilog`], which compiled
    /// the module the stimulus named and therefore cannot disagree with it,
    /// does not pay a string comparison to prove it.
    fn execute(&self, stimulus: &DigitalStimulus) -> Result<DigitalRunReport, DigitalRunError> {
        let mut host = DigitalHost::from_plan(
            Arc::clone(&self.plan),
            self.resolution,
            SchedulerLimits::HDL,
        );
        host.start()?;

        // Resolve every name once. A stimulus naming a port the design does not
        // declare is a mistake in the caller, and finding it before anything runs
        // makes the diagnostic name the port rather than a time.
        //
        // The design is asked what each port carries, and the stimulus is checked
        // against that answer rather than trusted for it: a `.stim` that has
        // drifted away from its design would otherwise drive a real net with bits
        // and produce a trace that looks like a run.
        let names = DesignNames {
            aliases: &self.signal_aliases,
            // A linked design's roots are its instances, which the aliases
            // already name; a compiled module is its own root.
            root: self
                .signal_aliases
                .is_empty()
                .then_some(self.module.as_str()),
        };
        let inputs = stimulus
            .inputs
            .iter()
            .map(|port| resolve_port(&host, port, &names))
            .collect::<Result<Vec<_>, DigitalRunError>>()?;
        let outputs = stimulus
            .outputs
            .iter()
            .map(|port| resolve_port(&host, port, &names))
            .collect::<Result<Vec<_>, DigitalRunError>>()?;
        let clock = stimulus
            .clock
            .as_ref()
            .map(|clock| {
                Ok::<_, DigitalRunError>((
                    resolve_signal(&host, &clock.port, &names)?,
                    clock.half_period,
                ))
            })
            .transpose()?;

        let mut observations = Vec::with_capacity(stimulus.vectors.len());
        let mut clock_level = FourStateBit::Zero;
        let mut next_clock_edge = clock.map(|(_, half)| half);

        // `initial clk = 1'b0;` — the level the free-running clock holds before its
        // first edge, which is what the generated testbench declares.
        if let Some((signal, _)) = clock {
            host.force(signal, FourStateValue::splat(1, FourStateBit::Zero), 0)?;
        }

        for (index, vector) in stimulus.vectors.iter().enumerate() {
            let apply_at = (index as u64)
                .checked_mul(stimulus.step)
                .ok_or(DigitalRunError::TickOverflow)?;
            let sample_at = apply_at
                .checked_add(stimulus.settle)
                .ok_or(DigitalRunError::TickOverflow)?;

            // The clock is advanced first at a tick both it and a vector fall on.
            // Two `initial`/`always` blocks writing different signals at one time
            // are a race IEEE 1364-2005 leaves to the simulator; fixing the order
            // here makes the run reproducible, and the corpus's clocked designs are
            // edge-triggered on the rising edge, which never coincides with a
            // vector boundary under `step = 2 * half_period`.
            if let Some((signal, half)) = clock {
                while let Some(edge) = next_clock_edge {
                    if edge > apply_at {
                        break;
                    }
                    host.advance_to(edge)?;
                    clock_level = invert(clock_level);
                    host.force(signal, FourStateValue::splat(1, clock_level), edge)?;
                    next_clock_edge = Some(
                        edge.checked_add(half)
                            .ok_or(DigitalRunError::TickOverflow)?,
                    );
                }
            }

            host.advance_to(apply_at)?;
            for (port, spelling) in inputs.iter().zip(vector) {
                let ResolvedPort {
                    signal,
                    width,
                    name,
                } = port;
                if *width == 0 {
                    let value: f64 =
                        spelling
                            .parse()
                            .map_err(|_| DigitalRunError::RealSpelling {
                                port: name.clone(),
                                spelling: spelling.clone(),
                            })?;
                    host.force_real(*signal, value, apply_at)?;
                    continue;
                }
                let value =
                    parse_four_state(spelling).ok_or_else(|| DigitalRunError::VectorSpelling {
                        port: name.clone(),
                        spelling: spelling.clone(),
                    })?;
                if value.width() != *width {
                    return Err(DigitalRunError::VectorWidth {
                        port: name.clone(),
                        declared: *width,
                        offered: value.width(),
                    });
                }
                host.force(*signal, value, apply_at)?;
            }

            // Sample after the settle, with every clock edge in between delivered.
            if let Some((signal, half)) = clock {
                while let Some(edge) = next_clock_edge {
                    if edge > sample_at {
                        break;
                    }
                    host.advance_to(edge)?;
                    clock_level = invert(clock_level);
                    host.force(signal, FourStateValue::splat(1, clock_level), edge)?;
                    next_clock_edge = Some(
                        edge.checked_add(half)
                            .ok_or(DigitalRunError::TickOverflow)?,
                    );
                }
            }
            host.advance_to(sample_at)?;

            observations.push(DigitalObservation {
                step: index,
                values: outputs
                    .iter()
                    .map(|port| {
                        let rendered = if port.width == 0 {
                            host.read_real(port.signal)
                                .map(render_real)
                                .unwrap_or_default()
                        } else {
                            host.read(port.signal)
                                .map(FourStateValue::spelling)
                                .unwrap_or_default()
                        };
                        (port.name.clone(), rendered)
                    })
                    .collect(),
            });
        }

        Ok(DigitalRunReport { observations })
    }
}

/// One stimulus port, resolved against the compiled design.
struct ResolvedPort {
    signal: rspice_veriloga::canonical_ir::ids::DigitalSignalId,
    /// The design's answer, not the stimulus's: zero for a real net.
    width: u32,
    name: String,
}

/// The names a stimulus may use for a design's signals.
struct DesignNames<'a> {
    aliases: &'a std::collections::BTreeMap<String, DigitalSignalId>,
    /// The root of an unlinked design: the module compiled.
    root: Option<&'a str>,
}

/// Resolve one name over the design's one store.
///
/// A name is a signal of the compiled module (`q`, `u1.q`) or, in a linked
/// design, a path from one of its roots (`a.u1.q`): each instance a design was
/// linked from is a root named by its instance path. A compiled module is its
/// own root, so `top.q` also names its `q`. Any of these may be spelled from
/// `$root` (IEEE 1800-2017 section 23.3.1): `$root.top.q`, `$root.a.u1.q`.
fn resolve_signal(
    host: &DigitalHost,
    name: &str,
    names: &DesignNames<'_>,
) -> Result<DigitalSignalId, DigitalRunError> {
    let lookup = |path: &str| {
        names
            .aliases
            .get(path)
            .copied()
            .map(Ok)
            .unwrap_or_else(|| host.signal(path))
    };
    fn below<'p>(root: Option<&str>, path: &'p str) -> Option<&'p str> {
        path.strip_prefix(root?)?.strip_prefix('.')
    }
    let found = match name.strip_prefix("$root.") {
        Some(path) if names.root.is_some() => below(names.root, path).map_or_else(
            || {
                Err(DigitalRunError::UnknownSignal {
                    name: name.to_owned(),
                })
            },
            lookup,
        ),
        Some(path) => lookup(path),
        None => lookup(name).or_else(|error| below(names.root, name).map_or(Err(error), lookup)),
    };
    found.map_err(|error| match error {
        DigitalRunError::UnknownSignal { .. } => DigitalRunError::UnknownSignal {
            name: name.to_owned(),
        },
        error => error,
    })
}

/// Resolve one stimulus port and check that the two agree about its domain.
fn resolve_port(
    host: &DigitalHost,
    port: &DigitalPort,
    names: &DesignNames<'_>,
) -> Result<ResolvedPort, DigitalRunError> {
    let signal = resolve_signal(host, &port.name, names)?;
    let real = host.is_real(signal);
    if real != (port.width == 0) {
        return Err(DigitalRunError::StimulusValueDomain {
            name: port.name.clone(),
            port_is_real: real,
        });
    }
    Ok(ResolvedPort {
        signal,
        width: port.width,
        name: port.name.clone(),
    })
}

/// Render a real for a trace column.
///
/// `{:?}` rather than `{}`, which are different for an `f64`: both round-trip,
/// and only the debug form keeps the decimal point that says the column is a
/// real. `1.0` printing as `1` would be indistinguishable from a one-bit port.
fn render_real(value: f64) -> String {
    format!("{value:?}")
}

fn invert(bit: FourStateBit) -> FourStateBit {
    match bit {
        FourStateBit::Zero => FourStateBit::One,
        FourStateBit::One => FourStateBit::Zero,
        other => other,
    }
}

/// Which order same-tick activations start in (design P2 section 2.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HdlSchedule {
    /// Source order: processes start, and a change wakes its waiters, in
    /// declaration order. What every run uses.
    #[default]
    Source,
    /// The reverse of source order, for the racy-test classifier only: a test
    /// whose outcome differs under it depends on an order IEEE 1364-2005
    /// section 11.4.2 leaves open.
    ReverseActive,
}

/// How a self-checking testbench run is configured (design P2 section 2.3).
#[derive(Debug, Clone, Default)]
pub struct HdlTestbenchOptions {
    /// Where relative HDL file paths resolve; the working directory when
    /// absent.
    pub run_directory: Option<std::path::PathBuf>,
    /// Let file writes leave the run directory (`--hdl-unrestricted-io`).
    pub unrestricted_io: bool,
    /// Write the design's files to disk. Otherwise they stay in memory: the
    /// report's `File` records carry them, and the design reads back what it
    /// wrote, but nothing on disk changes.
    pub write_files: bool,
    /// `$test$plusargs`/`$value$plusargs` sources, first match wins.
    pub plusargs: Vec<String>,
    /// `hdlseed`.
    pub seed: u32,
    /// min:typ:max selection (P2 lane L6).
    pub delay_select: crate::config::HdlDelaySelect,
    /// Honour `specify` blocks (P2 lane L6).
    pub specify: bool,
    /// `` `include `` search directories.
    pub include_dirs: Vec<std::path::PathBuf>,
    pub schedule: HdlSchedule,
    /// The run ends at the first tick past this one. `None` is the exact tick
    /// range's limit.
    pub tick_ceiling: Option<u64>,
}

/// How a testbench run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdlTestbenchEnd {
    /// `$finish` (IEEE 1364-2005 section 17.4.1).
    Finished,
    /// `$stop`.
    Stopped,
    /// `$fatal`.
    Fatal,
    /// No event was left to run.
    Quiescent,
    /// The next event lay past the tick ceiling.
    TickCeiling,
}

/// What a testbench run produced.
#[derive(Debug, Clone)]
pub struct HdlTestbenchReport {
    /// Every output record, in emission order.
    pub records: Vec<crate::config::HdlOutputRecord>,
    pub end: HdlTestbenchEnd,
    /// The level of the `$finish`, `$stop` or `$fatal` that ended the run.
    pub finish_level: Option<u8>,
    /// The tick the run ended at, and that tick in seconds.
    pub end_tick: u64,
    pub end_seconds: f64,
    /// Severity-task counts.
    pub warnings: u32,
    pub errors: u32,
    pub fatals: u32,
}

/// The modules a source declares, in order.
fn declared_modules(source: &str) -> Result<Vec<String>, DigitalRunError> {
    use rspice_veriloga::lexer::{Lexer, TokenKind};
    let mut sources = rspice_veriloga::SourceMap::new();
    let id = sources.add_source("unit", source);
    let tokens =
        Lexer::new(source, id)
            .collect_tokens()
            .map_err(|error| DigitalRunError::Compile {
                detail: error.to_string(),
            })?;
    Ok(tokens
        .windows(2)
        .filter(|pair| matches!(pair[0].kind, TokenKind::Module | TokenKind::Macromodule))
        .filter_map(|pair| pair[1].text.clone())
        .collect())
}

/// Run a self-checking HDL testbench to its end (design P2 section 2.3).
///
/// Every source is one compilation unit, read in order, so macros and
/// `timescale` carry from one file into the next (IEEE 1364-2005 sections
/// 19.3 and 19.8). Every module no other module instantiates is a root, and
/// the roots are linked into one design and run on the same host, effects and
/// interpreter the mixed coordinator drives — only the coupled trial is
/// absent — until `$finish`, `$stop`, event exhaustion, or the tick ceiling.
pub fn run_digital_testbench(
    sources: &[(std::path::PathBuf, String)],
    options: &HdlTestbenchOptions,
) -> Result<HdlTestbenchReport, DigitalRunError> {
    // Delay selection and `specify` are refused until they are implemented.
    // The token says which; the text names the feature.
    let unavailable = |capability: &'static str, option: &str, reason: &str| {
        Err(DigitalRunError::Unsupported(Box::new(
            crate::UnsupportedCapabilityError::new(
                capability,
                format!("{option} is not supported yet: {reason}"),
            ),
        )))
    };
    if options.delay_select != crate::config::HdlDelaySelect::Typ {
        return unavailable(
            "hdl.option.delay_select",
            "min:typ:max delay selection",
            "every HDL delay uses its typical value",
        );
    }
    if options.specify {
        return unavailable(
            "hdl.option.specify",
            "`specify` timing",
            "`specify` path delays and timing checks are not implemented",
        );
    }
    let mut unit = String::new();
    // Each file starts on its own line; a file's own line numbers are
    // recovered from where it starts in the unit.
    let mut starts = Vec::with_capacity(sources.len());
    for (path, text) in sources {
        starts.push((unit.lines().count() as u32, path.display().to_string()));
        unit.push_str(text);
        if !text.ends_with('\n') {
            unit.push('\n');
        }
    }
    let compiler = VerilogACompiler::new(CompilerOptions {
        include_paths: options.include_dirs.clone(),
        ..CompilerOptions::default()
    });
    let mut plans = Vec::new();
    for module in declared_modules(&unit)? {
        let artifact = compiler
            .compile_canonical_ir_module(&unit, Some(&module))
            .map_err(|error| DigitalRunError::Compile {
                detail: error.to_string(),
            })?;
        if !artifact.mir.equations.is_empty() || !artifact.digital.absdelta.is_empty() {
            return Err(DigitalRunError::MixedSignalModule {
                module,
                equations: artifact.mir.equations.len(),
            });
        }
        plans.push((module, artifact.digital));
    }
    // A module some other module instantiates is not a root.
    let instantiated: std::collections::BTreeSet<String> = plans
        .iter()
        .flat_map(|(_, plan)| plan.scopes.iter().skip(1))
        .filter(|scope| {
            scope.kind == rspice_veriloga::canonical_ir::digital::DigitalScopeKind::Module
        })
        .map(|scope| scope.module.to_string())
        .collect();
    let roots: Vec<_> = plans
        .into_iter()
        .filter(|(module, plan)| !instantiated.contains(module) && plan.has_executable_content())
        .collect();
    if roots.is_empty() {
        return Err(DigitalRunError::NoDigitalContent {
            module: sources
                .first()
                .map(|(path, _)| path.display().to_string())
                .unwrap_or_default(),
        });
    }
    let plan = if roots.len() == 1 {
        roots.into_iter().next().expect("one root").1
    } else {
        use rspice_veriloga::canonical_ir::digital_link::{
            DigitalLinkInstance, link_digital_plans,
        };
        let instances: Vec<_> = roots
            .iter()
            .map(|(module, plan)| DigitalLinkInstance {
                name: module,
                plan,
                ports: &[],
            })
            .collect();
        link_digital_plans(&instances, &[], &rspice_veriloga::NoPipelineControl)
            .map_err(|diagnostics| DigitalRunError::Compile {
                detail: diagnostics
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; "),
            })?
            .plan
    };
    let resolution = TimeResolution::new(plan.timing.precision_exponent)?;
    let mut host = DigitalHost::from_plan(Arc::new(plan), resolution, SchedulerLimits::HDL);
    let target = if options.write_files {
        effects::files::HdlFileTarget::Disk
    } else {
        effects::files::HdlFileTarget::Memory
    };
    host.configure_effects(effects::HdlEffectsConfig {
        plusargs: options.plusargs.clone(),
        seed: options.seed,
        files: effects::files::HdlFileOptions::new(
            options.run_directory.as_deref(),
            options.unrestricted_io,
            Arc::new(effects::files::HdlFileStore::new(target)),
        ),
    });
    host.set_reverse_active(options.schedule == HdlSchedule::ReverseActive);
    host.start()?;
    let ceiling = options
        .tick_ceiling
        .unwrap_or(TimeResolution::MAX_EXACT_TICKS);
    let mut end_tick = 0;
    let mut end = HdlTestbenchEnd::Quiescent;
    // Every step is accepted: its output is released, and its file writes
    // land, as it ends.
    let mut records = host.drain_hdl_output();
    while host.hdl_finish().is_none() {
        let Some(next) = host.next_tick() else {
            break;
        };
        if next > ceiling {
            end = HdlTestbenchEnd::TickCeiling;
            break;
        }
        host.advance_to(next)?;
        records.append(&mut host.drain_hdl_output());
        end_tick = next;
    }
    let finish = host.hdl_finish().cloned();
    if let Some(finish) = &finish {
        end_tick = finish.tick;
        end = match finish.kind {
            effects::HdlFinishKind::Finish => HdlTestbenchEnd::Finished,
            effects::HdlFinishKind::Stop => HdlTestbenchEnd::Stopped,
            effects::HdlFinishKind::Fatal => HdlTestbenchEnd::Fatal,
        };
    }
    let end_seconds = resolution.ticks_to_seconds(end_tick)?;
    records.append(&mut host.drain_hdl_output());
    for record in &mut records {
        record.point = crate::config::HdlPoint::Transient {
            time: record.seconds,
        };
        // The unit's lines back to each file's own.
        if record.origin.line > 0 && record.origin.file == "<input>" {
            let line = record.origin.line - 1;
            if let Some((start, file)) = starts.iter().rev().find(|(start, _)| *start <= line) {
                record.origin.file = file.clone();
                record.origin.line = line - start + 1;
            }
        }
    }
    let severities = host.hdl_severities();
    Ok(HdlTestbenchReport {
        records,
        end,
        finish_level: finish.map(|finish| finish.level),
        end_tick,
        end_seconds,
        warnings: severities.warnings,
        errors: severities.errors,
        fatals: severities.fatals,
    })
}

/// Read a `%b`-style four-state spelling, most significant bit first.
fn parse_four_state(spelling: &str) -> Option<FourStateValue> {
    if spelling.is_empty() {
        return None;
    }
    let mut bits = Vec::with_capacity(spelling.len());
    for character in spelling.chars() {
        bits.push(match character {
            '0' => FourStateBit::Zero,
            '1' => FourStateBit::One,
            'x' | 'X' => FourStateBit::Unknown,
            'z' | 'Z' | '?' => FourStateBit::HighImpedance,
            _ => return None,
        });
    }
    Some(FourStateValue::from_bits_msb_first(&bits))
}
