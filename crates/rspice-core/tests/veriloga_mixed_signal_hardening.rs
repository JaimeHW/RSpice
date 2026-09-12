//! What the mixed Verilog-AMS interleave promises when something goes wrong.
//!
//! [`tests/veriloga_mixed_signal_route.rs`] pins what a deck gets when a mixed
//! module works. This file pins what it gets when one does not, and what the
//! engine may not do to a working one on the way past.
//!
//! # The three properties
//!
//! ## 1. Rollback safety at the boundary, attacked
//!
//! `circuit::mixed_signal`'s documentation states that "a `CircuitData` clone —
//! an AC sweep worker, a checkpoint — never captures speculative state, and no
//! engine path can reach a half-open module". That is a claim about paths that
//! do not exist, which is the kind a test suite is worst at and most needed
//! for. Four attacks, one per engine path that could take a copy of, restart,
//! or abandon a running module:
//!
//! * **Checkpoint and resume.** A mixed transient has no checkpoint to resume:
//!   the format carries no digital state, so a run that asks for one is refused
//!   before it solves anything, naming the state it cannot carry. Before this
//!   suite it was not refused at all — a module with no pending self-scheduled
//!   activation resumed with its `initial` blocks run again at time zero and
//!   produced a plausible, inverted trace. The refusal is pinned here at the
//!   capability, which is where it now happens.
//! * **Circuit clones.** Every analysis that hands a worker thread its own
//!   `CircuitData` refuses a mixed module first, by name, before any clone.
//! * **Swept re-runs.** A `.STEP` expansion runs one deck many times through one
//!   engine. Each run must start the module from its `initial` blocks, so the
//!   answer at a sweep point does not depend on which points ran before it.
//! * **Newton under stress.** Iteration ceilings and impossible tolerances
//!   change how much work the solver does at every timepoint and must change
//!   nothing about the module, because every Newton evaluation is a probe trial
//!   that is rolled back before the stamp returns.
//!
//! ## 2. Cross-domain feedback is diagnosed, not simulated
//!
//! A comparator whose digital inverse drives its own reference has no
//! consistent boundary value at one timepoint. It used to run to `tstop` and
//! report a trace. It now names its participants.
//!
//! ## 3. The tick grid's timing error is bounded and does not accumulate
//!
//! The digital side counts nanosecond ticks and the analog side does not land
//! on them. Flooring is the mapping between the two, so the error it introduces
//! is bounded by one tick *per conversion* — and because each conversion reads
//! only the time it is converting, the hundredth edge of a run is no worse than
//! the first. The breakpoint direction has no error at all: an event's tick
//! seconds become an accepted analog timepoint bit-exactly.
#![cfg(feature = "veriloga")]

#[path = "common/digital_trace_invariants.rs"]
mod digital_trace_invariants;

use rspice_core::analysis::PssConfig;
use rspice_core::analysis::pac::PacConfig;
use rspice_core::engine::{
    TransientCheckpointBlockerSource, TransientResult, TransientStartupMode,
};
use rspice_core::netlist::{StepCommand, StepSweep, StepTarget};
use rspice_core::xspice::event_scheduler::{SchedulerLimits, TimeResolution};
use rspice_core::xspice::verilog::{MixedSignalError, MixedSignalHost};
use rspice_core::{Engine, Netlist, SimulationConfig, SimulationError};
use rspice_veriloga::vm::IntegrationCoefficients;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// The digital time base every mixed module runs on.
///
/// `xspice::verilog`'s `TIME_UNIT_EXPONENT` is `-9`, and it is crate-private,
/// so this suite re-states it the way `tests/sync_contract.rs` re-states the
/// XSPICE tick encoding. `the_tick_grid_is_one_nanosecond` fails if the two
/// ever disagree about a time a mixed module actually produces.
const TICK_EXPONENT: i8 = -9;

/// Seconds per tick, from the resolution rather than written out.
fn tick_seconds() -> f64 {
    TimeResolution::new(TICK_EXPONENT)
        .expect("a nanosecond resolution is declarable")
        .seconds_per_tick()
}

/// The largest tick whose seconds image is at or before `seconds`.
///
/// `TimeResolution::seconds_to_floor_ticks` is crate-visible, so this mirrors
/// it — including the two corrections that make it exact, because a single
/// division is off by up to one ulp and this suite's whole subject is what the
/// last ulp does.
fn floor_ticks(seconds: f64) -> u64 {
    let scale = tick_seconds();
    let mut ticks = (seconds / scale).floor() as u64;
    while ticks > 0 && (ticks as f64) * scale > seconds {
        ticks -= 1;
    }
    while ((ticks + 1) as f64) * scale <= seconds {
        ticks += 1;
    }
    ticks
}

/// The seconds a tick names, through the public conversion.
fn tick_to_seconds(ticks: u64) -> f64 {
    TimeResolution::new(TICK_EXPONENT)
        .expect("a nanosecond resolution is declarable")
        .ticks_to_seconds(ticks)
        .expect("a tick inside the exactly-representable range")
}

/// A `.va` written to a unique path, deleted when the guard drops.
///
/// The engine's Verilog-A cache is keyed by canonical path, so a shared
/// filename would be a shared cache entry.
struct ModelFile(PathBuf);

impl ModelFile {
    fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_mixed_hardening_{name}_{}_{sequence}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create model file");
        file.write_all(source.as_bytes()).expect("write model");
        Self(path)
    }

    fn deck_path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn run(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .expect("the deck runs");
    digital_trace_invariants::assert_one_digital_value_per_instant(&result, deck);
    result
}

fn error_for(deck: &str, tstop: f64, max_step: f64) -> String {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .err()
        .map(|error| error.to_string())
        .unwrap_or_else(|| panic!("the deck was expected to be refused, and ran"))
}

/// Every recorded change of a boundary net, as `(time, spelling)`.
fn digital_points(result: &TransientResult, net: &str) -> Vec<(f64, String)> {
    result
        .digital_trace_named(net)
        .unwrap_or_else(|| panic!("net '{net}' has no digital trace"))
        .iter()
        .map(|point| (point.time, format!("{:?}", point.value.state)))
        .collect()
}

// ---------------------------------------------------------------------------
// Shared modules
// ---------------------------------------------------------------------------

/// A module that schedules its own activations, so its next event time is a
/// runtime breakpoint on every step.
const CLOCK_DIVIDER: &str = r#"
`include "disciplines.vams"
module clock_divider(p, n, qdiv);
    inout p, n;
    electrical p, n;
    output qdiv;
    reg clk, qdiv;
    initial clk = 1'b0;
    initial qdiv = 1'b0;
    always #5 clk = ~clk;
    always @(posedge clk) qdiv <= ~qdiv;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// A module with no self-scheduled activation at all: it moves only when the
/// analog side crosses its A/D threshold.
///
/// That is the shape the checkpoint hole hid behind. A module with a pending
/// `#delay` trips the missed-breakpoint guard on resume for reasons that have
/// nothing to do with the checkpoint's contents; this one has nothing pending,
/// so a resume used to succeed and answer.
const EXTERNAL_TOGGLE: &str = r#"
`include "disciplines.vams"
module external_toggle(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= ~q;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn divider_deck(model: &ModelFile, tstop_ns: u32) -> String {
    format!(
        "* a digital clock divider driving an analog RC across a d2a boundary\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         r1 qdiv out 1k\n\
         c1 out 0 10p\n\
         .va \"{}\" clock_divider\n\
         .tran 1n {tstop_ns}n\n\
         .end\n",
        model.deck_path()
    )
}

fn external_toggle_deck(model: &ModelFile) -> String {
    format!(
        "* a module that only moves when the analog side crosses its threshold\n\
         vclk clk 0 pulse(0 3.3 5n 0.1n 0.1n 10n 20n)\n\
         x1 p 0 clk qs external_toggle\n\
         rp p 0 1meg\n\
         rq qs 0 10k\n\
         .va \"{}\" external_toggle\n\
         .tran 1n 200n\n\
         .end\n",
        model.deck_path()
    )
}

//=============================================================================
// Attack 1 — checkpoint and resume
//=============================================================================

/// The refusal every checkpoint-asking entry point owes a mixed deck.
///
/// Typed, so a frontend can route it as a capability gap rather than parse a
/// sentence, and specific, so the sentence still names the state.
fn assert_mixed_checkpoint_refusal(entry: &str, error: &SimulationError) {
    let SimulationError::UnsupportedCapability(refusal) = error else {
        panic!("{entry} must be refused as a capability, got {error}");
    };
    assert_eq!(
        refusal.capability, "analysis.tran.checkpoint_capability",
        "{entry} must be refused by the checkpoint capability boundary"
    );
    let lowered = error.to_string().to_lowercase();
    assert!(
        lowered.contains("mixed verilog-ams") && lowered.contains("digital state"),
        "{entry} must name the state the checkpoint cannot carry: {error}"
    );
}

/// **Attack 1.** A mixed deck that asks for a checkpoint is refused before the
/// solver runs, and the refusal says which state cannot be carried.
///
/// The checkpoint format is a numeric store: solutions, histories, limiter
/// anchors, per-instance vectors. A mixed module's accepted state is not that.
/// It is a running digital design — an event queue, every process's resumption
/// point, every `reg`, the resolved drivers, the boundary values — and
/// `MixedSignalHost::checkpoint` captures all of it into an image that holds a
/// compiled analog device and a live scheduler, which is not a thing this
/// format writes.
///
/// So there is no such checkpoint to take, and the request for one is answered
/// at `t = 0` rather than at `tstop`: the only consumer of a checkpoint is a
/// resume, so a run that solved to the end and *then* said the image was
/// unusable would have spent the whole run to deliver the same answer. What
/// makes this a *hardening* pin rather than a statement of a limitation is what
/// it replaced: a module with nothing pending checkpointed and resumed,
/// restarted its `initial` blocks at time zero, and produced a trace inverted
/// against the baseline from the checkpoint onward, with nothing saying so.
/// This test is written against that specific deck, so a change that removes
/// the refusal has to remove this too — including the unsegmented run below,
/// which is what proves the module had live state to lose.
#[test]
fn a_mixed_checkpoint_schedule_is_refused_before_solving_by_naming_the_state_it_cannot_carry() {
    let model = ModelFile::new("checkpoint_external", EXTERNAL_TOGGLE);
    let deck = external_toggle_deck(&model);
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());

    // Unsegmented, the deck runs, and the module moves several times before
    // the time a checkpoint was being asked for. That is the state a resume
    // would have restarted, so the refusal is protecting something real.
    let baseline = run(&deck, 200.0e-9, 1.0e-9);
    let before: Vec<_> = digital_points(&baseline, "qs")
        .into_iter()
        .filter(|(time, _)| *time < 100.0e-9)
        .collect();
    assert!(
        before.len() >= 4,
        "the module must toggle several times before the checkpoint time, saw {before:?}"
    );

    let capability = engine
        .preflight_transient_checkpoint(&netlist)
        .expect("the capability preflight elaborates this deck rather than running it");
    assert!(
        !capability.is_resumable(),
        "a mixed deck has no resumable checkpoint"
    );
    let blockers = capability.blockers();
    assert_eq!(
        blockers.len(),
        1,
        "the mixed host is the only thing blocking this deck's checkpoint: {blockers:?}"
    );
    assert_eq!(
        blockers[0].source,
        TransientCheckpointBlockerSource::ExtensionState,
        "the digital half is owned by an extension runtime, not by the integrator"
    );
    assert_eq!(
        blockers[0].message,
        "mixed Verilog-AMS accepted digital state is not checkpointed"
    );

    let Err(scheduled) = engine.run_tran_checkpoint_schedule_with_startup_mode(
        &netlist,
        200.0e-9,
        1.0e-9,
        TransientStartupMode::OperatingPoint,
        &[100.0e-9],
    ) else {
        panic!("a scheduled mixed checkpoint must be refused, not produced")
    };
    assert_mixed_checkpoint_refusal("a scheduled mixed checkpoint", &scheduled);

    let Err(retained) = engine.run_tran_checkpointed(&netlist, 200.0e-9, 1.0e-9) else {
        panic!("a retained mixed checkpoint must be refused, not produced")
    };
    assert_mixed_checkpoint_refusal("a retained mixed checkpoint", &retained);
}

/// **Attack 1, self-scheduled half.** The same refusal reaches a module whose
/// event wheel is not empty, and it is still the capability's.
///
/// This deck used to be refused on resume by `MissedDigitalBreakpoint` — the
/// rebuilt module still held the activation its `initial` block placed at time
/// zero, and the first trial at the resume time stepped past it. That is a
/// guard noticing a symptom, and it fires only for modules that happen to have
/// something pending. The refusal has to be the capability's, so it arrives for
/// every mixed deck; and it now arrives before any step is taken, so the guard
/// is not even reachable on this path. Both halves are asserted: the message is
/// the capability's, and it is not the guard's.
#[test]
fn a_self_scheduling_module_is_refused_by_the_preflight_not_by_a_missed_breakpoint() {
    let model = ModelFile::new("checkpoint_divider", CLOCK_DIVIDER);
    let deck = divider_deck(&model, 200);
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let Err(error) = engine.run_tran_checkpoint_schedule_with_startup_mode(
        &netlist,
        200.0e-9,
        1.0e-9,
        TransientStartupMode::OperatingPoint,
        &[100.0e-9],
    ) else {
        panic!("a scheduled mixed checkpoint must be refused, not produced")
    };
    assert_mixed_checkpoint_refusal("a self-scheduling mixed checkpoint", &error);
    assert!(
        !error.to_string().to_lowercase().contains("stepped past"),
        "the missed-breakpoint guard must not be what refuses this; no step is taken: {error}"
    );
}

/// **Attack 1, control.** An analog-only `.VERILOGA` deck still checkpoints and
/// resumes.
///
/// Without this the refusal above could be a refusal of the whole `.VERILOGA`
/// route, which would be a regression dressed as a fix. A Verilog-A *device*
/// has serialized accepted state (`runtime_veriloga_instance_states`), and it
/// keeps working — so the blocker has to be about the mixed host's digital
/// half and nothing else.
///
/// What is asserted is that the resume happens and where it starts, not what
/// trajectory it takes. Trajectory agreement across a resume is
/// `tests/transient_checkpoint.rs`'s contract and it owns the bound; a control
/// test that restated it would become a second owner of somebody else's
/// property and would fail for reasons that have nothing to do with the
/// refusal it is controlling for.
#[test]
fn an_analog_only_veriloga_deck_still_resumes() {
    const ANALOG_ONLY: &str = r#"
`include "disciplines.vams"
module analog_only_route(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;
    let model = ModelFile::new("checkpoint_analog", ANALOG_ONLY);
    let deck = format!(
        "* an analog-only module across a checkpoint\n\
         v1 in 0 sin(0 1 20meg)\n\
         x1 in mid analog_only_route\n\
         c1 mid 0 1n\n\
         rmid mid 0 10k\n\
         .va \"{}\" analog_only_route\n\
         .tran 1n 200n\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    // The refusal above is decided by the capability preflight, so the control
    // has to clear that same preflight — otherwise the schedule below would be
    // refused for the reason this test exists to rule out.
    assert!(
        engine
            .preflight_transient_checkpoint(&netlist)
            .expect("the capability preflight elaborates this deck")
            .is_resumable(),
        "an analog-only Verilog-A deck must have a resumable checkpoint"
    );
    let (baseline, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            200.0e-9,
            1.0e-9,
            TransientStartupMode::OperatingPoint,
            &[100.0e-9],
        )
        .expect("the scheduled baseline run completes");
    let checkpoint = &scheduled[0].checkpoint;
    assert!(
        baseline
            .time
            .iter()
            .any(|time| time.to_bits() == checkpoint.time.to_bits()),
        "the checkpoint must be an accepted baseline point"
    );
    let (resumed, _) = engine
        .run_tran_resume(&netlist, checkpoint, 200.0e-9, 1.0e-9)
        .expect(
            "an analog-only Verilog-A deck must still resume; the mixed refusal is about a \
             module's digital half, not about the `.VERILOGA` route",
        );
    assert!(
        resumed
            .time
            .first()
            .is_some_and(|time| *time >= checkpoint.time),
        "the resumed run must start at the checkpoint, not at zero"
    );
    assert!(
        resumed
            .time
            .last()
            .is_some_and(|time| (*time - 200.0e-9).abs() < 1.0e-15),
        "and must run to tstop, saw {:?}",
        resumed.time.last()
    );
    assert!(
        resumed.digital_traces.is_empty(),
        "an analog-only module opens no digital trace channel, resumed or not"
    );
}

//=============================================================================
// Attack 2 — circuit clones
//=============================================================================

/// **Attack 2.** Every analysis that hands a worker its own `CircuitData`
/// refuses a mixed module before any copy is taken.
///
/// This is the structural claim in `circuit::mixed_signal`'s documentation —
/// "a `CircuitData` clone … never captures speculative state" — and the reason
/// it holds is not that a clone would be safe. It is that no clone happens: the
/// three analyses that call `circuit.clone()` to feed worker threads (AC's
/// frequency chunks, noise's frequency chunks, PSS's shooting columns) all pass
/// through `ensure_no_mixed_signal_analysis` first, and so do the seven that
/// linearize without cloning.
///
/// Named individually rather than looped over a list, so a refusal that
/// disappears from one analysis fails on that analysis's name.
#[test]
fn every_analysis_that_copies_the_circuit_refuses_a_mixed_module_first() {
    let model = ModelFile::new("clone_refusals", CLOCK_DIVIDER);
    let deck = format!(
        "* a mixed module asked for answers no interleave has\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         vac p 0 ac 1\n\
         rload qdiv 0 1k\n\
         .va \"{}\" clock_divider\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = [1.0e5, 1.0e6];

    let refusal = |analysis: &str, error: Option<String>| {
        let error = error.unwrap_or_else(|| {
            panic!("{analysis} must refuse a mixed module rather than omit or approximate it")
        });
        let lowered = error.to_lowercase();
        assert!(
            lowered.contains(&analysis.to_lowercase()) && lowered.contains("x1"),
            "the {analysis} refusal must name the analysis and the instance: {error}"
        );
        assert!(
            lowered.contains("only `.tran` runs a mixed module"),
            "the refusal must say what does run one: {error}"
        );
    };

    refusal(
        "AC analysis",
        engine
            .run_ac(&netlist, &frequencies)
            .err()
            .map(|error| error.to_string()),
    );
    refusal(
        "noise analysis",
        engine
            .run_noise(&netlist, 1, &frequencies, 300.15)
            .err()
            .map(|error| error.to_string()),
    );
    refusal(
        "distortion analysis",
        engine
            .run_distortion(&netlist, &frequencies, None)
            .err()
            .map(|error| error.to_string()),
    );
}

/// **Attack 2, periodic half.** The periodic small-signal analyses refuse a
/// mixed module too, and refuse it for the same reason and in the same words.
///
/// These are the routes the refusal was missing from. `ensure_no_mixed_signal_analysis`
/// was called by AC, DC, distortion, HB, noise, PSS, PSS-noise, sensitivity and
/// STB, but by nothing under `engine/hb/`. So a deck whose periodic operating
/// point was solved by PAC or driven pnoise itself — no retained `.PSS` or
/// `.HB` carrier in front of it — reached the harmonic solver with the mixed
/// host's equations simply not stamped, and answered. The answer was a real
/// spectrum for a circuit that is not the authored one: the module's analog
/// half contributes nothing and its digital half drives nothing, so a d2a
/// boundary node floats at whatever the rest of the deck puts there.
///
/// The two retained-carrier routes (`.PXF`, and `.PSP` in either of its
/// `prepare_psp_from_*` forms) take a `PssOperatingPoint` or an
/// `HbOperatingPoint` as an argument, and neither can be obtained for a mixed
/// deck — `run_pss*` and `run_hb*` refuse it first. They are guarded all the
/// same, in `prepare_periodic_ac`, because that is where the circuit first
/// exists; there is no way to call them here to prove it.
#[test]
fn every_periodic_small_signal_analysis_refuses_a_mixed_module_first() {
    let model = ModelFile::new("periodic_refusals", CLOCK_DIVIDER);
    let deck = format!(
        "* a mixed module asked for a periodic small-signal answer\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         vdrive p 0 SIN(0 0.1 1meg) AC 1\n\
         rload qdiv 0 1k\n\
         cload qdiv 0 1p\n\
         .va \"{}\" clock_divider\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());

    // `route` is what the caller asked for; `analysis` is the name the refusal
    // must carry, which for the oscillator route is the PSS solve it runs.
    let refusal = |route: &str, analysis: &str, outcome: Result<String, SimulationError>| {
        let error = match outcome {
            Ok(answer) => panic!(
                "{route} must refuse a mixed module rather than assemble the periodic \
                 system without it; it answered with {answer}"
            ),
            Err(error) => error.to_string(),
        };
        let lowered = error.to_lowercase();
        assert!(
            lowered.contains(&analysis.to_lowercase()),
            "the {route} refusal must name the analysis: {error}"
        );
        assert!(
            lowered.contains("x1"),
            "the {route} refusal must name the instance: {error}"
        );
        assert!(
            lowered.contains("only `.tran` runs a mixed module"),
            "the {route} refusal must say what does run one: {error}"
        );
    };

    refusal(
        "PAC",
        "PAC analysis",
        engine
            .run_pac(
                &netlist,
                PacConfig::new()
                    .with_sweep(1.0e3, 1.0e4, 2)
                    .with_sidebands(-1, 1)
                    .with_input_source("vdrive")
                    .with_output_node("qdiv")
                    .with_fundamental(1.0e6),
            )
            .map(|result| {
                format!(
                    "a PAC result at {} Hz (converged={})",
                    result.fundamental_freq, result.converged
                )
            }),
    );
    refusal(
        "driven pnoise",
        "pnoise analysis",
        engine
            .run_pnoise(&netlist, 1.0e6, &[1.0e3], "p", None, None, 1)
            .map(|_| "a driven pnoise spectrum".to_string()),
    );
    refusal(
        "oscillator pnoise",
        "PSS analysis",
        engine
            .run_pnoise_oscillator(
                &netlist,
                PssConfig::autonomous().with_period_guess(1.0e-6),
                &[1.0e3],
            )
            .map(|_| "an oscillator phase-noise spectrum".to_string()),
    );
}

/// **Attack 2, host half.** A clone of a running host shares no state it can
/// write through, and neither copy can commit the other's trial.
///
/// The engine takes no clone mid-trial — the trial is opened and closed inside
/// one call — but `MixedSignalHost` derives `Clone` because `CircuitData` does,
/// and every payload sits behind a copy-on-write cell. So the property that
/// makes the derive safe is worth pinning directly: a clone taken while a trial
/// is open is a complete, independent image of that trial, and committing on
/// one moves nothing in the other.
#[test]
fn a_clone_of_a_running_host_is_an_independent_image_of_it() {
    let mut driver = ProbeDriver::new();
    driver.land_on_activations_before(10.0e-9, &probe_voltages(false));
    assert_eq!(
        driver.host.read_digital("c").expect("c is readable"),
        "0",
        "the boundary must be settled low before the trial that moves it"
    );

    // Open a trial and move the boundary inside it, so the clone is taken over
    // state that is speculative rather than settled.
    driver.begin_and_settle(10.0e-9, &probe_voltages(true));

    let mut clone = driver.host.clone();
    assert!(
        clone.checkpoint().is_err(),
        "a clone taken mid-trial must carry the open trial, so it is not checkpointable"
    );

    // Commit on the clone. The original still holds the same open trial.
    clone
        .accept_trial()
        .expect("the clone commits its own trial");
    assert_eq!(
        clone.read_digital("c").expect("c is readable"),
        "1",
        "the clone's commit must have landed"
    );
    driver
        .host
        .reject_trial()
        .expect("the original's trial is still open and still rejectable");
    assert_eq!(
        driver.host.read_digital("c").expect("c is readable"),
        "0",
        "the clone's commit must not have reached the original"
    );
    assert_eq!(
        driver
            .host
            .next_event_time()
            .expect("the schedule is readable"),
        None,
        "and neither must the reaction the clone's commit scheduled"
    );
    driver
        .host
        .checkpoint()
        .expect("the original is idle again, and checkpointable");
}

//=============================================================================
// Attack 3 — swept re-runs
//=============================================================================

/// **Attack 3.** A `.STEP` expansion's answer at one sweep point does not
/// depend on which points ran before it.
///
/// One engine runs one deck many times over a sweep, and a mixed module is
/// built fresh on each `build_circuit` — but the compiled `.va` behind it is a
/// process-wide cache entry keyed by path, shared across every run. If any
/// running state travelled with that entry rather than with the host, a sweep's
/// third point would answer differently depending on whether the first or the
/// second preceded it.
///
/// The attack is order, not repetition: three sweep points are run forwards and
/// then backwards through the same engine, and each point's digital trace has
/// to be identical between the two orders. A cache that carried state would
/// have to carry it symmetrically to survive that, which nothing does by
/// accident.
#[test]
fn a_swept_rerun_starts_the_module_from_its_initial_blocks_every_time() {
    let model = ModelFile::new("swept_toggle", EXTERNAL_TOGGLE);
    let deck = format!(
        "* a clock period swept across three values\n\
         .param tper=20n\n\
         vclk clk 0 pulse(0 3.3 5n 0.1n 0.1n 'tper/2' 'tper')\n\
         x1 p 0 clk qs external_toggle\n\
         rp p 0 1meg\n\
         rq qs 0 10k\n\
         .va \"{}\" external_toggle\n\
         .tran 1n 400n\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let command = StepCommand {
        target: StepTarget::Param,
        name: "tper".to_string(),
        param_name: None,
        sweep: StepSweep::List(vec![20.0e-9, 40.0e-9, 80.0e-9]),
    };
    let values = command.sweep.values();
    let swept = engine
        .step_netlists_for_command(&netlist, &command, &values)
        .expect("the sweep expands");
    assert_eq!(swept.len(), 3, "the sweep must produce three netlists");

    let mut forward = Vec::new();
    for (value, stepped) in &swept {
        let result = engine
            .run_tran(stepped, 400.0e-9, 1.0e-9)
            .unwrap_or_else(|error| panic!("sweep point {value:e} runs: {error}"));
        forward.push(digital_points(&result, "qs"));
    }
    let mut backward = vec![Vec::new(); swept.len()];
    for (index, (value, stepped)) in swept.iter().enumerate().rev() {
        let result = engine
            .run_tran(stepped, 400.0e-9, 1.0e-9)
            .unwrap_or_else(|error| panic!("sweep point {value:e} re-runs: {error}"));
        backward[index] = digital_points(&result, "qs");
    }

    // Vacuity first: the three points must actually be three different
    // circuits, or an order-independence claim is about nothing.
    assert!(
        forward[0].len() > forward[1].len() && forward[1].len() > forward[2].len(),
        "a slower clock must give fewer toggles, saw {} / {} / {}",
        forward[0].len(),
        forward[1].len(),
        forward[2].len()
    );

    for (index, (first, second)) in forward.iter().zip(&backward).enumerate() {
        assert_eq!(
            first, second,
            "sweep point {index} ({:e}) answered differently depending on what ran before it",
            values[index]
        );
    }
}

//=============================================================================
// Attack 4 — Newton under stress
//=============================================================================

/// **Attack 4.** Starving Newton changes how much work the solver does at every
/// timepoint and changes nothing about the module.
///
/// `stamp_mixed_transient_trial` opens a *probe* trial for every Newton
/// evaluation and rolls it back before the stamp returns, whether or not the
/// stamp succeeded. So the number of evaluations a timepoint takes, the
/// tolerances they are judged against, and whether any of them was abandoned
/// are all invisible to the digital half.
///
/// Three configurations are run against the reference: a one-iteration
/// transient ceiling, tolerances no solve can meet, and both together. Each is
/// a different amount of Newton work — and each has to produce the same digital
/// trace, transition for transition and bit for bit, because a rejected trial
/// commits nothing.
///
/// `tests/veriloga_mixed_signal_route.rs` pins the neighbouring case, where the
/// *step controller* rejects timepoints on truncation error. The two are
/// separate paths: that one rejects an accepted-candidate trial, this one
/// abandons a Newton evaluation inside one.
#[test]
fn newton_work_never_reaches_the_module() {
    let model = ModelFile::new("newton_stress", CLOCK_DIVIDER);
    let deck = format!(
        "* the divider driving a diode-loaded rc, so Newton has real work\n\
         x1 p 0 qdiv clock_divider\n\
         rp p 0 1meg\n\
         r1 qdiv out 1k\n\
         c1 out 0 10p\n\
         d1 out 0 dmod\n\
         .model dmod d (is=1e-16 n=1 rs=0.1 cjo=2p)\n\
         .va \"{}\" clock_divider\n\
         .tran 1n 200n\n\
         .end\n",
        model.deck_path()
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let reference = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 200.0e-9, 1.0e-9)
        .expect("the reference run completes");
    let expected = digital_points(&reference, "qdiv");
    assert!(
        expected.len() >= 10,
        "the reference must exercise the boundary, saw {expected:?}"
    );

    let starved = || SimulationConfig {
        transient_nonlinear_max_iterations: Some(1),
        transient_max_iterations: 1,
        ..SimulationConfig::default()
    };
    let impossible = || {
        // Positive, so the configuration validates, and far below any residual
        // a floating-point solve can produce.
        let convergence_config = rspice_core::engine::ConvergenceConfig {
            voltage_reltol: 1.0e-300,
            voltage_abstol: 1.0e-300,
            residual_reltol: 1.0e-300,
            current_abstol: 1.0e-300,
            ..rspice_core::engine::ConvergenceConfig::default()
        };
        SimulationConfig {
            convergence_config,
            ..SimulationConfig::default()
        }
    };
    let both = || SimulationConfig {
        transient_nonlinear_max_iterations: Some(1),
        transient_max_iterations: 1,
        ..impossible()
    };

    let mut completed_constrained_runs = 0;
    for (label, config) in [
        ("a one-iteration transient ceiling", starved()),
        ("tolerances no solve can meet", impossible()),
        ("both at once", both()),
    ] {
        let result = match Engine::new(config).run_tran(&netlist, 200.0e-9, 1.0e-9) {
            Ok(result) => {
                completed_constrained_runs += 1;
                result
            }
            Err(rspice_core::SimulationError::ConvergenceFailed(_)) => continue,
            Err(error) => {
                panic!(
                    "with {label} the run must still complete or refuse with a convergence \
                     diagnostic; got: {error}"
                )
            }
        };
        let observed = digital_points(&result, "qdiv");
        assert_eq!(
            observed.len(),
            expected.len(),
            "with {label} the module took a different number of transitions"
        );
        for (index, (actual, wanted)) in observed.iter().zip(&expected).enumerate() {
            assert_eq!(
                actual.1, wanted.1,
                "with {label}, transition {index} settled to a different value"
            );
            assert_eq!(
                actual.0.to_bits(),
                wanted.0.to_bits(),
                "with {label}, transition {index} moved from {:e} to {:e}",
                wanted.0,
                actual.0
            );
        }
    }
    assert!(
        completed_constrained_runs > 0,
        "at least one constrained run must complete and verify the accepted boundary trace"
    );
}

/// **Attack 4, host half.** A stamp that fails leaves a trial that rolls back
/// to exactly what it found.
///
/// The engine's rollback is unconditional — `stamp_mixed_transient_trial`
/// rejects the trial and only then propagates the stamp's error — and this is
/// the property that makes that correct. A stamp handed a non-finite solution
/// refuses; the trial is still open, and rejecting it restores the digital
/// half, the event schedule and the boundary values to their pre-trial state.
#[test]
fn a_failed_stamp_leaves_a_trial_that_rolls_back_exactly() {
    let mut driver = ProbeDriver::new();
    driver.land_on_activations_before(10.0e-9, &probe_voltages(false));
    let settled = driver.host.read_digital("c").expect("c is readable");
    let settled_event = driver
        .host
        .next_event_time()
        .expect("the schedule is readable");

    driver.begin_and_settle(10.0e-9, &probe_voltages(true));
    assert_ne!(
        driver
            .host
            .next_event_time()
            .expect("the schedule is readable"),
        settled_event,
        "the trial must have moved the module, or the rollback proves nothing"
    );

    let mut nonfinite = probe_voltages(true);
    nonfinite[1] = f64::NAN;
    let error = driver
        .host
        .stamp(&nonfinite, |_, _, _| {}, |_, _| {})
        .expect_err("a non-finite candidate must be refused rather than stamped");
    assert!(
        matches!(error, MixedSignalError::Analog { .. }),
        "the refusal must be the analog one: {error}"
    );

    driver
        .host
        .reject_trial()
        .expect("a trial whose stamp failed is still open, and still rejectable");
    assert_eq!(
        driver.host.read_digital("c").expect("c is readable"),
        settled,
        "the rollback must restore the boundary value"
    );
    assert_eq!(
        driver
            .host
            .next_event_time()
            .expect("the schedule is readable"),
        settled_event,
        "the rollback must restore the event schedule"
    );
    driver
        .host
        .checkpoint()
        .expect("no trial is open, so the module is checkpointable again");
}

//=============================================================================
// Oscillation and zero-delay loops
//=============================================================================

/// A comparator whose digital inverse drives its own reference. No delay
/// anywhere in the loop.
const INVERTING_LOOP: &str = r#"
`include "disciplines.vams"
module inverting_loop(p, n, c, y);
    inout p, n;
    electrical p, n;
    input c;
    output y;
    wire c;
    reg y;
    initial y = 1'b0;
    always @(c) y = ~c;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// A zero-delay A/D-to-D/A loop has no consistent startup solution. Report
/// its actual boundary transitions during rejected solver probes; no accepted
/// timepoint exists from which an accepted-history diagnostic could be built.
#[test]
fn a_cross_domain_zero_delay_loop_is_refused_by_naming_its_participants() {
    let model = ModelFile::new("inverting_loop", INVERTING_LOOP);
    let deck = format!(
        "* a comparator whose digital inverse drives its own reference\n\
         x1 p 0 cin yout inverting_loop\n\
         rp p 0 1meg\n\
         rfb yout cin 1\n\
         rin cin 0 1meg\n\
         vkick kick 0 pulse(0 3.3 1n 0.1n 0.1n 5n 10n)\n\
         rkick kick cin 100k\n\
         .va \"{}\" inverting_loop\n\
         .tran 0.1n 20n\n\
         .end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 20.0e-9, 0.1e-9);
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("mixed verilog-ams instance 'x1'"),
        "the diagnostic must name the X-card the loop is inside: {error}"
    );
    assert!(
        lowered.contains("net `c` on circuit node") && lowered.contains("read by the module"),
        "the diagnostic must name the net the module reads: {error}"
    );
    assert!(
        lowered.contains("net `y` on circuit node") && lowered.contains("driven by the module"),
        "the diagnostic must name the net the module drives: {error}"
    );
    assert!(
        error.contains("1 0 1 0"),
        "the diagnostic must show the alternation, which is the evidence that the count \
         is measuring feedback rather than a fast signal: {error}"
    );
    assert!(
        lowered.contains("rejected solver probes") && lowered.contains("t=0e0s"),
        "the diagnostic must identify the rejected startup observations: {error}"
    );
}

#[test]
fn a_cross_domain_loop_enabled_after_startup_names_its_rejected_probe_activity() {
    let model = ModelFile::new(
        "enabled_loop",
        r#"
module enabled_loop(p, n, c, en, y);
    inout p, n; electrical p, n;
    input c, en; output y;
    wire c, en; reg y;
    initial y = 1'b0;
    always @(c or en) y = en ? ~c : 1'b0;
    analog I(p,n) <+ V(p,n)/1000000.0;
endmodule
"#,
    );
    let deck = format!(
        "* enable feedback after a valid operating point\n\
         x1 p 0 cin en yout enabled_loop\n\
         rp p 0 1meg\nrfb yout cin 1\nrin cin 0 1meg\n\
         ven en 0 pulse(0 3.3 1n 0 0 10n 20n)\n\
         .va \"{}\" enabled_loop\n.tran 0.1n 3n\n.end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 3e-9, 0.1e-9);
    let lowered = error.to_ascii_lowercase();
    assert!(
        lowered.contains("mixed verilog-ams instance 'x1'"),
        "{error}"
    );
    assert!(
        lowered.contains("net `c`") && lowered.contains("net `y`"),
        "{error}"
    );
    assert!(lowered.contains("rejected solver probes"), "{error}");
    assert!(
        !lowered.contains("t=0e0s"),
        "startup must have succeeded: {error}"
    );
    assert!(error.contains("1 0 1 0"), "{error}");
}

/// **Cross-domain feedback, vacuity guard.** A boundary a resolved waveform
/// drives is not mistaken for a loop, however many timepoints the run accepts.
///
/// The detector counts *consecutive* accepted timepoints that moved one net, so
/// what would break it is a legitimate deck whose boundary moves at every
/// accepted timepoint for long enough. The divider run at a twenty-picosecond
/// ceiling is the opposite extreme — ten thousand accepted timepoints and
/// twenty boundary transitions — and it is here to keep a future tightening of
/// the ceiling honest about what it would cost.
#[test]
fn a_resolved_boundary_is_not_mistaken_for_a_loop() {
    let model = ModelFile::new("resolved_boundary", CLOCK_DIVIDER);
    let result = run(&divider_deck(&model, 200), 200.0e-9, 2.0e-11);
    let transitions = digital_points(&result, "qdiv");
    assert!(
        result.time.len() > 1000,
        "the run must accept far more timepoints than the flip ceiling, saw {}",
        result.time.len()
    );
    assert_eq!(
        transitions.len(),
        21,
        "and move the boundary a bounded number of times: twenty toggles and the opening value"
    );
}

/// A module with a zero-delay loop between two of its own registers.
const SAME_TICK_LOOP: &str = r#"
`include "disciplines.vams"
module same_tick_loop(p, n, clk, y);
    inout p, n;
    electrical p, n;
    input clk;
    output y;
    wire clk;
    reg y, z;
    initial y = 1'b0;
    initial z = 1'b0;
    always @(posedge clk) z = ~z;
    always @(z) y = ~y;
    always @(y) z = ~z;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// **Same-tick loops.** The kernel's own oscillation diagnostic reaches the
/// deck with the X-card that owns it.
///
/// A loop entirely inside the discrete half is the event kernel's problem and
/// the kernel already diagnoses it: `SchedulerError::Oscillation` names the
/// tick, the ceiling and the busiest driver. What the mixed route adds is the
/// only context the kernel cannot have — which instance of which model this
/// design was inside — and this pins that the two arrive together rather than
/// the kernel's message reaching the user with no deck in it.
#[test]
fn a_same_tick_zero_delay_loop_surfaces_with_the_instance_that_owns_it() {
    let model = ModelFile::new("same_tick_loop", SAME_TICK_LOOP);
    let deck = format!(
        "* a zero-delay loop between two registers of one module\n\
         vclk clk 0 pulse(0 3.3 2n 0.1n 0.1n 5n 10n)\n\
         x1 p 0 clk yout same_tick_loop\n\
         vquiet clkquiet 0 0\n\
         x0 pquiet 0 clkquiet yquiet same_tick_loop\n\
         rp p 0 1meg\n\
         ry yout 0 10k\n\
         rpquiet pquiet 0 1meg\n\
         ryquiet yquiet 0 10k\n\
         .va \"{}\" same_tick_loop\n\
         .tran 0.2n 20n\n\
         .end\n",
        model.deck_path()
    );
    let error = error_for(&deck, 20.0e-9, 0.2e-9);
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("mixed verilog-ams instance 'x1'"),
        "the deck-level context must be the instance: {error}"
    );
    assert!(
        !lowered.contains("mixed verilog-ams instance 'x0'"),
        "the quiet instance sorts first but does not own the looping processes: {error}"
    );
    assert!(
        lowered.contains("event network did not settle at tick"),
        "the kernel's own diagnostic must survive the trip: {error}"
    );
    assert!(
        lowered.contains("delta cycles") && lowered.contains("busiest driver"),
        "including the evidence it carries: {error}"
    );
}

//=============================================================================
// Tick quantization
//=============================================================================

/// A module that reacts to a boundary change after a declared delay, so the
/// tick the change was published into is readable through `next_event_time`.
const DELAYED_REACTION: &str = r#"
`include "disciplines.vams"
module delayed_reaction(p, n, c, y);
    inout p, n;
    electrical p, n;
    input c;
    output y;
    wire c;
    reg y;
    initial y = 1'b0;
    always @(c) #3 y = ~y;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// The delay the module's process waits, in ticks.
const REACTION_DELAY_TICKS: u64 = 3;

/// The A/D threshold the probe host bridges at. Half of a 3.3 V supply, which
/// is what `add_planned_xspice_auto_bridge` gives a boundary net.
const PROBE_THRESHOLD: f64 = 1.65;

/// Circuit nodes for the probe host: `1` is the module's `p` terminal, `2` the
/// A/D sense node, `3` the D/A output node.
fn probe_host() -> MixedSignalHost {
    let mut host = MixedSignalHost::compile(
        DELAYED_REACTION,
        None,
        "xprobe",
        &[1, 0],
        SchedulerLimits::default(),
    )
    .expect("the probe module compiles and starts");
    host.add_adc_bridge("c", 0, (2, 0), PROBE_THRESHOLD, PROBE_THRESHOLD)
        .expect("the A/D bridge is declarable");
    host.add_dac_bridge("y", 0, (3, 0), 0.0, 3.3, 20.0)
        .expect("the D/A bridge is declarable");
    host
}

/// A solution vector putting the A/D sense node either side of the threshold.
fn probe_voltages(high: bool) -> Vec<f64> {
    vec![0.0, if high { 3.3 } else { 0.0 }, 0.0]
}

/// Drive one host the way the transient stepper drives it.
///
/// Two things the stepper does that a naive loop does not, and both are
/// required rather than tidy:
///
/// * it lands on every scheduled activation. `begin_trial` refuses a timepoint
///   past a pending event — that is `MixedSignalError::MissedDigitalBreakpoint`,
///   D5 clause 2 enforced from the module's side — so a driver that jumps over
///   one is refused, exactly as the engine would be if its breakpoint list ever
///   lost an entry.
/// * it hands each trial the interval it actually advanced. The interpolated
///   crossing is computed inside `[time - dt, time]`, so a `dt` larger than the
///   time is an interval starting before the run did.
struct ProbeDriver {
    host: MixedSignalHost,
    /// Last accepted analog time, or negative infinity before the first.
    accepted: f64,
    /// Time the open trial was begun at. The host publishes no reader for it.
    open: f64,
}

impl ProbeDriver {
    /// A started host with its boundary settled at time zero.
    fn new() -> Self {
        let mut driver = Self {
            host: probe_host(),
            accepted: f64::NEG_INFINITY,
            open: 0.0,
        };
        driver.begin_and_settle(0.0, &probe_voltages(false));
        driver.accept();
        driver
    }

    /// Open a trial at `time` and settle its boundary, leaving it open.
    fn begin_and_settle(&mut self, time: f64, voltages: &[f64]) {
        let first = !self.accepted.is_finite();
        let dt = if first { 0.0 } else { time - self.accepted };
        self.open = time;
        self.host
            .begin_trial(time, dt, IntegrationCoefficients::inactive(), first, false)
            .unwrap_or_else(|error| panic!("a trial at {time:e} s begins: {error}"));
        while self
            .host
            .settle_analog_bridges(voltages)
            .unwrap_or_else(|error| panic!("the bridges settle at {time:e} s: {error}"))
        {}
    }

    fn accept(&mut self) {
        let time = self.open;
        self.host
            .accept_trial()
            .unwrap_or_else(|error| panic!("a quiet trial at {time:e} s commits: {error}"));
        self.accepted = time;
    }

    fn step_to(&mut self, time: f64, voltages: &[f64]) {
        self.begin_and_settle(time, voltages);
        self.accept();
    }

    /// Land on every scheduled activation strictly before `time`.
    fn land_on_activations_before(&mut self, time: f64, voltages: &[f64]) {
        while let Some(next) = self
            .host
            .next_event_time()
            .expect("the schedule is readable")
        {
            if next >= time || next <= self.accepted {
                break;
            }
            self.step_to(next, voltages);
        }
    }

    /// The next scheduled activation, which must exist.
    fn next_event(&self) -> f64 {
        self.host
            .next_event_time()
            .expect("the schedule is readable")
            .expect("a change on the boundary schedules the module's reaction")
    }

    /// The tick the change the last trial made was published into, recovered
    /// from the schedule: the module's process waits [`REACTION_DELAY_TICKS`]
    /// after the change, so its next activation is that many ticks later.
    ///
    /// Recovered in ticks and subtracted there rather than in seconds, because
    /// `10 ns - 3 ns` is not `7 ns` in binary floating point and the whole
    /// subject here is what the last ulp does. The seconds-to-tick direction is
    /// exactly invertible over an activation's own time, which is what makes
    /// this recovery exact rather than approximate.
    fn published_ticks(&self) -> u64 {
        floor_ticks(self.next_event())
            .checked_sub(REACTION_DELAY_TICKS)
            .expect("an activation is at least the reaction delay past its cause")
    }

    /// The same instant in seconds.
    fn published_instant(&self) -> f64 {
        tick_to_seconds(self.published_ticks())
    }
}

/// The tick grid the mixed route runs on is one nanosecond.
///
/// Every bound below is stated against this number, so it is checked rather
/// than assumed — and checked through a module actually running, not only
/// through the resolution type.
#[test]
fn the_tick_grid_is_one_nanosecond() {
    assert_eq!(
        tick_seconds().to_bits(),
        1.0e-9f64.to_bits(),
        "the mixed route's declared precision is one nanosecond"
    );

    let mut driver = ProbeDriver::new();
    driver.land_on_activations_before(7.5e-9, &probe_voltages(false));
    // A crossing published inside tick 7 wakes the process 3 ticks later, at
    // 10 ns exactly. If the grid were anything but a nanosecond this is the
    // assertion that would say so.
    driver.step_to(7.5e-9, &probe_voltages(true));
    let woken = driver.next_event();
    assert_eq!(
        woken.to_bits(),
        tick_to_seconds(7 + REACTION_DELAY_TICKS).to_bits(),
        "a crossing at 7.5 ns must wake a 3-tick delay at 10 ns, got {woken:e}"
    );
}

/// **Bound 1.** A digital edge caused at accepted analog time `t` is published
/// into `[t - 1 tick, t]`, never after `t`.
///
/// The mapping is `seconds_to_floor_ticks`, whose answer is defined as the
/// largest tick `k` with `k * Δ <= t`. Two facts follow directly from that
/// definition and are what this asserts:
///
/// * `k * Δ <= t` — the digital world is never run past an instant the
///   integrator has accepted, which is the property flooring was chosen for;
/// * `(k + 1) * Δ > t`, so `t - k * Δ < Δ` — the error is under one tick.
///
/// The publication instant is read back through the schedule: the module waits
/// `REACTION_DELAY_TICKS` after the change, so its next event time is
/// `(k + 3) * Δ` and `k * Δ` is that minus three ticks. Nothing in the test
/// computes `k` from the implementation — it is derived from `t` here, and the
/// two have to agree.
///
/// The times are deliberately off-grid, spaced by an increment whose fractional
/// nanosecond part does not repeat, so the bound is exercised at many different
/// distances into a tick rather than at one.
#[test]
fn a_published_edge_lands_within_one_tick_at_or_before_the_time_that_caused_it() {
    const MEASUREMENTS: usize = 200;
    /// Off-grid, and irrational enough in nanoseconds that the fractional part
    /// walks the whole tick rather than cycling through a few values.
    const SPACING: f64 = 7.271_828_182_845_9e-9;
    const START: f64 = 0.318_309_886e-9;

    let delta = tick_seconds();
    let mut driver = ProbeDriver::new();

    let mut worst = 0.0f64;
    let mut total = 0.0f64;
    let mut deep = 0usize;
    for index in 0..MEASUREMENTS {
        let time = START + SPACING * index as f64;
        // Alternate the sense node, so every trial publishes a change, and land
        // on the reaction the previous one scheduled before stepping past it.
        let level = probe_voltages(index % 2 == 0);
        driver.land_on_activations_before(time, &probe_voltages(index % 2 == 1));
        driver.step_to(time, &level);
        let published = driver.published_instant();

        assert_eq!(
            published.to_bits(),
            tick_to_seconds(floor_ticks(time)).to_bits(),
            "edge {index}: the publication tick must be the floor of {time:e} s"
        );
        let error = time - published;
        assert!(
            error >= 0.0,
            "edge {index}: a publication at {published:e} s is after the accepted time \
             {time:e} s, which runs the digital world past the integrator"
        );
        assert!(
            error < delta,
            "edge {index}: the quantization error {error:e} s is a whole tick or more, so \
             the mapping is not a floor onto a {delta:e} s grid"
        );
        worst = worst.max(error);
        total += error;
        if error > 0.5 * delta {
            deep += 1;
        }
    }

    // Vacuity: the grid has to be doing something, or a bound on its error is
    // a bound on nothing.
    assert!(
        deep >= MEASUREMENTS / 4,
        "at least a quarter of the edges must land more than half a tick into one, or the \
         times chosen are not exercising the grid: {deep} of {MEASUREMENTS}"
    );
    assert!(
        worst > 0.9 * delta,
        "the worst error must approach a full tick, saw {worst:e} s against {delta:e} s"
    );
    // And the sum is what an accumulating mapping would have produced by the
    // end. Stating it here is what makes the per-edge bound above a
    // no-drift claim rather than a per-edge one.
    assert!(
        total > 50.0 * delta,
        "the errors must sum to many ticks, or 'they do not accumulate' says nothing: \
         {total:e} s"
    );
}

/// **Bound 2.** The error does not accumulate: the last edge of a long run is
/// bounded by one tick, not by the number of edges before it.
///
/// This is a property of the mapping's *form*. `seconds_to_floor_ticks` reads
/// only the seconds it is converting; it holds no residue, no previous tick and
/// no phase. So each conversion's error is independent, and the bound on the
/// `N`th is the bound on the first.
///
/// Asserted as a comparison rather than as a repeat of bound 1: the errors of
/// the first ten edges and of the last ten are drawn from the same interval,
/// and the run between them is long enough that a mapping carrying even a
/// single ulp of residue per edge would separate them.
#[test]
fn a_long_chain_of_crossings_accumulates_no_quantization_drift() {
    const MEASUREMENTS: usize = 400;
    const SPACING: f64 = 5.772_156_649e-9;

    let delta = tick_seconds();
    let mut driver = ProbeDriver::new();

    let mut errors = Vec::with_capacity(MEASUREMENTS);
    for index in 0..MEASUREMENTS {
        let time = 0.577e-9 + SPACING * index as f64;
        let level = probe_voltages(index % 2 == 0);
        driver.land_on_activations_before(time, &probe_voltages(index % 2 == 1));
        driver.step_to(time, &level);
        errors.push(time - driver.published_instant());
    }

    let last = errors.last().copied().expect("the run produced edges");
    assert!(
        (0.0..delta).contains(&last),
        "the {MEASUREMENTS}th edge's error {last:e} s must still be under one tick \
         ({delta:e} s), not {MEASUREMENTS} of them"
    );
    let worst_early = errors[..10].iter().copied().fold(0.0, f64::max);
    let worst_late = errors[MEASUREMENTS - 10..]
        .iter()
        .copied()
        .fold(0.0, f64::max);
    assert!(
        worst_late < delta && worst_early < delta,
        "both ends of the run must be inside one tick: {worst_early:e} then {worst_late:e}"
    );
    // The simulated span is what makes the claim worth making: an accumulating
    // mapping would be out by two microseconds here, which is a third of the
    // run.
    let span = 0.577e-9 + SPACING * (MEASUREMENTS - 1) as f64;
    assert!(
        span > 2.0e-6,
        "the run must span microseconds for a per-edge residue to have shown, saw {span:e} s"
    );
}

/// **Bound 3.** The other direction has no error: an event's tick seconds
/// become an accepted analog timepoint bit-exactly, at every event of a long
/// run.
///
/// This is D5 clause 2 — `tests/sync_contract.rs` pins it on one awkward event
/// time, through the XSPICE path — exercised here over two hundred consecutive
/// events of a mixed module instead. The chain is `next_mixed_event_time` →
/// `collect_transient_runtime_breakpoints` → `BreakpointManager::limit_step` →
/// `snap_to_breakpoint`, and the last of those is what makes `t + dt` land on
/// the stored breakpoint's own `f64` rather than a neighbour of it.
///
/// The module toggles an internal clock every five ticks, so the events are
/// `5 ns, 10 ns, 15 ns …` — and every one of those has to appear in the
/// accepted grid with the bits `ticks_to_seconds` produced, not merely within
/// tolerance of them.
#[test]
fn every_digital_activation_becomes_a_bit_exact_accepted_timepoint() {
    const TSTOP_NS: u32 = 1000;
    const ACTIVATION_TICKS: u64 = 5;

    let model = ModelFile::new("breakpoint_chain", CLOCK_DIVIDER);
    let result = run(
        &divider_deck(&model, TSTOP_NS),
        f64::from(TSTOP_NS) * 1.0e-9,
        1.0e-9,
    );

    let events = u64::from(TSTOP_NS) / ACTIVATION_TICKS;
    assert!(
        events >= 200,
        "the run must cover many events for this to be a chain, saw {events}"
    );
    let mut missing = Vec::new();
    for index in 1..=events {
        let expected = tick_to_seconds(index * ACTIVATION_TICKS);
        if !result
            .time
            .iter()
            .any(|time| time.to_bits() == expected.to_bits())
        {
            missing.push(expected);
        }
    }
    assert!(
        missing.is_empty(),
        "{} of {events} digital activations were not accepted timepoints with their own \
         bits; first few: {:?}",
        missing.len(),
        &missing[..missing.len().min(5)]
    );
}

//=============================================================================
// 4 — a digital schedule finer than the analog resolution lands, never refuses
//=============================================================================

/// A free-running clock declared in picoseconds on a femtosecond precision.
///
/// `always #1` is one *module* time unit, so this toggles every picosecond —
/// three orders of magnitude below the nanosecond cadence the rest of this
/// suite runs at, and far below any step the analog controller would pick on
/// its own. The analog half is a resistor written as a contribution so the
/// module is a mixed one; the interesting half is entirely discrete.
const PICOSECOND_CLOCK: &str = r#"
`timescale 1ps/1fs
`include "disciplines.vams"
module picosecond_clock(p, n, clk);
    inout p, n;
    electrical p, n;
    output clk;
    reg clk;
    initial clk = 1'b0;
    always #1 clk = ~clk;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// A module whose digital half chains a one-femtosecond delay onto an
/// activation the *analog* side produced.
///
/// The `posedge` comes from an A/D conversion of a deck node, so the first
/// activation lands wherever the analog crossing lands; the `#0.001` — one
/// thousandth of the declared picosecond unit, one tick of the declared
/// femtosecond precision — then asks for a second activation an interval later
/// that no analog step is allowed to resolve.
///
/// `tick_kept` is what separates the two halves of the claim. The analog point
/// the follow-up is *recorded* at moves — it has to, there is no analog time a
/// femtosecond after the crossing — but the time the module *sees* must not:
/// `$abstime` inside the resumed branch has to still be the activation's own
/// tick, a femtosecond after the one that woke it, not the ten-femtosecond
/// landing the stepper reached. Two ticks is the window, because an A/D
/// crossing's seconds are the interpolated crossing rather than its floored
/// tick; the landing is ten ticks away and cannot pass it.
const FEMTOSECOND_FOLLOW: &str = r#"
`timescale 1ps/1fs
`include "disciplines.vams"
module femtosecond_follow(p, n, sense, q, qd, tick_kept);
    inout p, n;
    electrical p, n;
    input sense;
    output q, qd, tick_kept;
    wire sense;
    reg q, qd, tick_kept;
    real woke, held;
    initial begin q = 1'b0; qd = 1'b0; tick_kept = 1'b0; woke = 0.0; held = 0.0; end
    always @(posedge sense) begin
        woke = $abstime;
        q = ~q;
        #0.001 held = $abstime;
        qd = ~qd;
        tick_kept = ((held - woke) > 0.0) && ((held - woke) < 2.0e-15);
    end
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn picosecond_clock_deck(model: &ModelFile, tstop_ns: u32) -> String {
    format!(
        "* a picosecond digital clock driving an analog RC across a d2a boundary\n\
         x1 p 0 clk picosecond_clock\n\
         rp p 0 1meg\n\
         r1 clk out 1k\n\
         c1 out 0 10p\n\
         .va \"{}\" picosecond_clock\n\
         .tran 1n {tstop_ns}n\n\
         .end\n",
        model.deck_path()
    )
}

fn femtosecond_follow_deck(model: &ModelFile) -> String {
    format!(
        "* a femtosecond follow-up chained onto an A/D crossing\n\
         vsense sense 0 pulse(0 3.3 100u 1u 1u 100u 200u)\n\
         x1 p 0 sense q qd kept femtosecond_follow\n\
         rp p 0 1meg\n\
         rq q 0 10k\n\
         rqd qd 0 10k\n\
         rkept kept 0 10k\n\
         .va \"{}\" femtosecond_follow\n\
         .tran 10u 400u\n\
         .end\n",
        model.deck_path()
    )
}

/// **Property 4, case a.** A picosecond clock is simulated, not refused, and
/// every edge of it is delivered.
///
/// Every one of these activations is a mandatory analog time: the breakpoint
/// manager holds it and the stepper must advance to it. What it must *not* do
/// is decide that a schedule finer than its own preferred cadence is an error.
/// The analog controller would happily take nanosecond steps here — the
/// requested maximum is a nanosecond — and the whole run is spent landing on
/// picosecond activations instead, which is the correct answer and the
/// expensive one.
///
/// This one does not reach the landing contract: a nanosecond maximum step
/// puts the solver hard minimum at `1e-11 * 1e-9 = 1e-20`, eight decades below
/// a picosecond, so nothing here is unresolvable. It is the other half of the
/// property — that a schedule this fine is *carried*, at full count — which is
/// why the assertion is the edge count rather than the absence of an error.
/// Ten nanoseconds is the horizon because each edge is an accepted analog
/// point: the count is exact, and the run is ten thousand of them.
#[test]
fn a_picosecond_clock_runs_to_tstop_instead_of_ending_the_run() {
    const TSTOP_NS: u32 = 10;
    const HALF_PERIOD: f64 = 1.0e-12;

    let tstop = f64::from(TSTOP_NS) * 1.0e-9;
    let model = ModelFile::new("picosecond_clock", PICOSECOND_CLOCK);
    let result = run(&picosecond_clock_deck(&model, TSTOP_NS), tstop, 1.0e-9);

    let last = result.time.last().copied().unwrap_or(0.0);
    assert!(
        last >= tstop - HALF_PERIOD,
        "the run must reach tstop {tstop:e}s, it stopped at {last:e}s"
    );
    let points = digital_points(&result, "clk");
    // One edge per half period, plus or minus whether the edge exactly at
    // tstop is inside the run and whether the initial level is recorded.
    let edges = (tstop / HALF_PERIOD).round() as usize;
    assert!(
        points.len().abs_diff(edges) <= 1,
        "a {HALF_PERIOD:e}s half period over {TSTOP_NS} ns is {edges} edges, saw {}",
        points.len()
    );
    for pair in points.windows(2).take(64) {
        let gap = pair[1].0 - pair[0].0;
        assert!(
            (gap - HALF_PERIOD).abs() <= 1.0e-15,
            "consecutive activations must be one picosecond apart, saw {gap:e}s between \
             {:e}s and {:e}s",
            pair[0].0,
            pair[1].0
        );
    }
}

/// **Property 4, case b.** A follow-up activation closer than the solver's
/// hard minimum lands on the nearest analog instant it can, and the run
/// continues.
///
/// The maximum step here is a millisecond, which puts ngspice's `delmin` — and
/// with it the solver hard minimum — at ten femtoseconds, and the breakpoint
/// tolerance at a hundred. The module's `#0.001` is *one* femtosecond after an
/// activation the analog side just landed on, so it is below both. The digital
/// scheduler keeps that exact tick; the analog side owes it only a timepoint at
/// or after it, so the two activations coalesce onto neighbouring analog points
/// rather than ending the run.
///
/// Both halves are asserted, because the analog half alone is not the claim.
/// The recorded times come from the accepted analog point, so a follow-up
/// landed ten femtoseconds late looks the same there as one landed on time;
/// `tick_kept` is the module's own reading of `$abstime` inside the resumed
/// branch, and it is what says the digital tick survived the landing.
#[test]
fn a_femtosecond_follow_up_activation_lands_instead_of_ending_the_run() {
    const TSTOP: f64 = 400.0e-6;
    const MAX_STEP: f64 = 1.0e-3;

    let model = ModelFile::new("femtosecond_follow", FEMTOSECOND_FOLLOW);
    let result = run(&femtosecond_follow_deck(&model), TSTOP, MAX_STEP);

    let last = result.time.last().copied().unwrap_or(0.0);
    assert!(
        last >= TSTOP - 1.0e-12,
        "the run must reach tstop {TSTOP:e}s, it stopped at {last:e}s"
    );
    let q = digital_points(&result, "q");
    let qd = digital_points(&result, "qd");
    assert!(
        q.len() >= 2,
        "the two rising edges of the stimulus must each activate the module, saw {q:?}"
    );
    assert_eq!(
        qd.len(),
        q.len(),
        "every activation owes exactly one follow-up: {q:?} against {qd:?}"
    );
    for (index, (&(q_time, _), &(qd_time, _))) in q.iter().zip(&qd).enumerate() {
        assert!(
            qd_time >= q_time,
            "follow-up {index} must not precede the activation it is chained to: \
             {qd_time:e}s before {q_time:e}s"
        );
        assert!(
            qd_time - q_time <= 1.0e-12,
            "follow-up {index} is one femtosecond of digital time after its activation and \
             must coalesce onto a neighbouring analog point, saw {:e}s later",
            qd_time - q_time
        );
    }

    // The module's own clock reading. Two points and no more: low from the
    // `initial` block, high at the first follow-up, and never falling back —
    // a third point would mean the second activation read a different time
    // from the first.
    let kept = digital_points(&result, "kept");
    assert_eq!(
        kept.len(),
        2,
        "every resumed branch must read its own tick, not the analog landing: {kept:?}"
    );
    assert_ne!(
        kept[1].1, kept[0].1,
        "the tick check must go high, and it stayed at {:?}",
        kept[0].1
    );
    let first_follow_up = qd.get(1).expect("a follow-up was recorded").0;
    assert_eq!(
        kept[1].0, first_follow_up,
        "the tick check is assigned in the branch it measures, so it lands on that \
         branch's analog point"
    );
}

//=============================================================================
// Crossing instants
//=============================================================================
//
// **Property 5.** A transition that crosses a domain boundary is dated by the
// instant it happened, and the three mappings that quantize such an instant
// are the three `xspice::verilog::mixed` documents — no more, and never one
// standing in for another.
//
// * An A/D crossing interior to an analog step is dated at the interpolated
//   crossing, which is both what `$abstime` reads in the process it wakes and
//   what names the tick its event lands on. The endpoint of the step that
//   discovered it is a different instant and is not either answer.
// * Two bridges that crossed at two instants are two events at two ticks,
//   published earliest first. One bank at the latest of their ticks delays the
//   earlier transition by a whole tick.
// * The ticks one trial publishes at never decrease, however far back into the
//   step a later Newton iteration's crossing interpolates. The instant stays
//   the crossing's own; the tick is clamped forward onto the trial's
//   high-water mark, because a wheel that runs backwards dates an effect
//   before its cause.
// * A wake from the other event kernel — an XSPICE code model's output — is
//   dated at the tick *at or after* it, because the reverse direction hands an
//   HDL tick to XSPICE at exactly the instant that tick names. Nearest-tick
//   would let a `#1` from the woken process elapse in under one time unit.

/// The threshold every A/D bridge below is declared at: half of the 3.3 V
/// supply, which is what the deck route gives a boundary net.
const CROSSING_THRESHOLD: f64 = 1.65;

/// Where a linear ramp from zero volts to `voltage` over `[time - dt, time]`
/// passes [`CROSSING_THRESHOLD`].
///
/// Written as the interpolation the bridge itself performs rather than as a
/// fraction of the step, so a measured crossing is compared with the
/// arithmetic that produced it and not with a restatement of it.
fn analytic_crossing(time: f64, dt: f64, voltage: f64) -> f64 {
    time - dt * (voltage - CROSSING_THRESHOLD) / voltage
}

/// The candidate voltage whose ramp crosses the threshold `fraction` of the
/// way through a step.
fn ramp_reaching_threshold_at(fraction: f64) -> f64 {
    CROSSING_THRESHOLD / fraction
}

/// A module-level `real` read back out of a four-state register.
///
/// `$realtobits` is the only exact route from the discrete half's real
/// arithmetic to something [`MixedSignalHost::read_digital`] can report, and
/// exactness is the whole point: the quantity under test is a sub-tick instant
/// and any scaling would round away the difference being measured.
fn read_real_bits(host: &MixedSignalHost, signal: &str) -> f64 {
    let spelling = host
        .read_digital(signal)
        .unwrap_or_else(|error| panic!("`{signal}` is readable: {error}"));
    let bits = u64::from_str_radix(&spelling, 2)
        .unwrap_or_else(|_| panic!("`{signal}` holds `{spelling}`, not a known 64-bit pattern"));
    f64::from_bits(bits)
}

/// A module that records the simulation time an A/D crossing woke it at, and
/// reacts a declared delay later so the tick it was published into is readable
/// through `next_event_time`.
const CROSSING_STAMP: &str = r#"
`timescale 1ns/1ns
`include "disciplines.vams"
module crossing_stamp(p, n, c, y);
    inout p, n;
    electrical p, n;
    input c;
    output y;
    wire c;
    reg y;
    reg [63:0] woke;
    initial begin y = 1'b0; woke = 64'd0; end
    always @(posedge c) begin
        woke = $realtobits($abstime);
        #3 y = ~y;
    end
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// Two independent A/D bridges on one instance, each recording its own wake
/// instant and reacting one tick later.
const TWO_CROSSINGS: &str = r#"
`timescale 1ns/1ns
`include "disciplines.vams"
module two_crossings(p, n, ca, cb, ya, yb);
    inout p, n;
    electrical p, n;
    input ca, cb;
    output ya, yb;
    wire ca, cb;
    reg ya, yb;
    reg [63:0] woke_a, woke_b;
    initial begin
        ya = 1'b0; yb = 1'b0; woke_a = 64'd0; woke_b = 64'd0;
    end
    always @(posedge ca) begin woke_a = $realtobits($abstime); #1 ya = ~ya; end
    always @(posedge cb) begin woke_b = $realtobits($abstime); #1 yb = ~yb; end
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// Two A/D bridges whose processes record the digital tick they ran on as well
/// as the physical instant that woke them.
///
/// `gain` is the discrete quantity the first crossing's process writes and the
/// analog half reads back — the feedback that makes the solver re-solve the
/// same trial and find the second bridge's crossing on a later Newton
/// iteration.
const FEEDBACK_CROSSINGS: &str = r#"
`timescale 1ns/1ns
`include "disciplines.vams"
module feedback_crossings(p, n, ca, cb, ya, yb);
    inout p, n;
    electrical p, n;
    input ca, cb;
    output ya, yb;
    wire ca, cb;
    reg ya, yb;
    reg [63:0] at_a, at_b, rt_a, rt_b;
    integer gain;
    initial begin
        ya = 1'b0; yb = 1'b0; gain = 1;
        at_a = 64'd0; at_b = 64'd0; rt_a = 64'd0; rt_b = 64'd0;
    end
    always @(posedge ca) begin
        gain = 2;
        at_a = $realtobits($abstime);
        rt_a = $realtobits($realtime);
        #1 ya = ~ya;
    end
    always @(posedge cb) begin
        at_b = $realtobits($abstime);
        rt_b = $realtobits($realtime);
        #1 yb = ~yb;
    end
    analog I(p, n) <+ gain * V(p, n) / 1000000.0;
endmodule
"#;

/// Drive one standalone host the way a transient stepper without boundary
/// refinement drives it: accept a point, open the next trial over the whole
/// interval to it, settle, accept.
///
/// Refinement is deliberately absent. `trial_boundary_refinement_time` asks
/// the engine to re-solve on an interior crossing, which drives the accepted
/// endpoint onto the crossing and hides the distinction being measured here;
/// an interval a controller chose for truncation error does not, and that is
/// the interval a crossing is genuinely interior to.
struct CrossingDriver {
    host: MixedSignalHost,
    accepted: f64,
    open: f64,
}

impl CrossingDriver {
    fn new(host: MixedSignalHost, voltages: &[f64]) -> Self {
        let mut driver = Self {
            host,
            accepted: f64::NEG_INFINITY,
            open: 0.0,
        };
        driver.begin_and_settle(0.0, voltages);
        driver.accept();
        driver
    }

    fn begin_and_settle(&mut self, time: f64, voltages: &[f64]) {
        let first = !self.accepted.is_finite();
        let dt = if first { 0.0 } else { time - self.accepted };
        self.open = time;
        self.host
            .begin_trial(time, dt, IntegrationCoefficients::inactive(), first, false)
            .unwrap_or_else(|error| panic!("a trial at {time:e} s begins: {error}"));
        self.settle(voltages);
    }

    /// Settle the open trial against one solution, to quiescence.
    ///
    /// Called a second time on the same trial with a different solution, this
    /// is the next Newton iteration: the solver re-solved the same candidate
    /// timepoint because the discrete half moved something the analog half
    /// reads, and the bridges are sampled again against the new iterate. The
    /// trial is the same trial — a crossing found here is found *after* every
    /// crossing the earlier iterations published.
    fn settle(&mut self, voltages: &[f64]) {
        let time = self.open;
        while self
            .host
            .settle_analog_bridges(voltages)
            .unwrap_or_else(|error| panic!("the bridges settle at {time:e} s: {error}"))
        {}
    }

    fn accept(&mut self) {
        let time = self.open;
        self.host
            .accept_trial()
            .unwrap_or_else(|error| panic!("a quiet trial at {time:e} s commits: {error}"));
        self.accepted = time;
    }

    fn step_to(&mut self, time: f64, voltages: &[f64]) {
        self.begin_and_settle(time, voltages);
        self.accept();
    }

    fn next_event(&self) -> f64 {
        self.host
            .next_event_time()
            .expect("the schedule is readable")
            .expect("a published boundary change schedules the module's reaction")
    }

    fn scheduled(&self) -> Option<f64> {
        self.host
            .next_event_time()
            .expect("the schedule is readable")
    }
}

/// **Property 5, case a.** `$abstime` inside a process woken by an A/D
/// crossing is the crossing, not the endpoint of the step that found it.
///
/// The ramp is linear and starts at the accepted point, so the crossing has a
/// closed form and the module's own reading of it can be compared with
/// arithmetic rather than with a recorded trace. Two fractions are measured
/// because the tick the transition is published into is the *nearest* one to
/// the crossing and the two fractions fall either side of a half tick: at 0.4
/// of a tick past the accepted point the crossing rounds back onto the trial's
/// own tick, at 0.8 it rounds forward onto the next one. `$abstime` is the
/// same quantity in both, and `$realtime` follows it rather than the endpoint.
#[test]
fn an_a_d_woken_process_reads_the_crossing_as_its_absolute_time() {
    /// The module waits this many ticks after the crossing that woke it.
    const REACTION: u64 = 3;
    const START: f64 = 10.0e-9;
    const STEP: f64 = 0.9e-9;

    for fraction in [0.4 / 0.9, 0.8 / 0.9] {
        let mut host = MixedSignalHost::compile(
            CROSSING_STAMP,
            None,
            "xstamp",
            &[1, 0],
            SchedulerLimits::default(),
        )
        .expect("the crossing-stamp module compiles");
        host.add_adc_bridge("c", 0, (2, 0), CROSSING_THRESHOLD, CROSSING_THRESHOLD)
            .expect("the A/D bridge is declarable");

        let mut driver = CrossingDriver::new(host, &[0.0, 0.0]);
        driver.step_to(START, &[0.0, 0.0]);

        let candidate = START + STEP;
        let voltage = ramp_reaching_threshold_at(fraction);
        driver.begin_and_settle(candidate, &[0.0, voltage]);

        let expected = analytic_crossing(candidate, STEP, voltage);
        let woke = read_real_bits(&driver.host, "woke");
        assert!(
            (woke - expected).abs() <= 1.0e-21,
            "a process woken by the crossing at {expected:.16e} s read $abstime \
             {woke:.16e} s (the step ran from {START:.16e} s to {candidate:.16e} s)"
        );

        // The tick the transition was published into, recovered through the
        // reaction delay: the process waits REACTION ticks after the change.
        driver.accept();
        let published = floor_ticks(driver.next_event()) - REACTION;
        let nearest = (expected / tick_seconds()).round() as u64;
        assert_eq!(
            published,
            nearest.max(floor_ticks(candidate)),
            "the crossing at {expected:.16e} s belongs to tick {nearest}, and $realtime \
             must name it rather than the endpoint's tick {}",
            floor_ticks(candidate)
        );
    }
}

/// **Property 5, case b.** Two bridges that crossed at two instants inside one
/// step publish at two ticks, earliest first.
///
/// The crossings are placed 0.4 and 0.6 of a tick past the accepted point, so
/// they round to two different ticks, and the step ends before the next tick
/// so neither is clamped forward onto the endpoint's. Each reaction is a
/// single tick, which turns the two publication ticks into two activations the
/// driver can land on separately: one process flips at the first, the other
/// only at the second.
#[test]
fn two_crossings_in_one_step_publish_at_their_own_ticks_in_order() {
    const START: f64 = 10.0e-9;
    const STEP: f64 = 0.9e-9;

    let mut host = MixedSignalHost::compile(
        TWO_CROSSINGS,
        None,
        "xpair",
        &[1, 0],
        SchedulerLimits::default(),
    )
    .expect("the two-crossing module compiles");
    host.add_adc_bridge("ca", 0, (2, 0), CROSSING_THRESHOLD, CROSSING_THRESHOLD)
        .expect("the first A/D bridge is declarable");
    host.add_adc_bridge("cb", 0, (3, 0), CROSSING_THRESHOLD, CROSSING_THRESHOLD)
        .expect("the second A/D bridge is declarable");

    let quiet = [0.0, 0.0, 0.0];
    let mut driver = CrossingDriver::new(host, &quiet);
    driver.step_to(START, &quiet);

    let candidate = START + STEP;
    let early = ramp_reaching_threshold_at(0.4 / 0.9);
    let late = ramp_reaching_threshold_at(0.6 / 0.9);
    let high = [0.0, early, late];
    driver.begin_and_settle(candidate, &high);

    let expected_early = analytic_crossing(candidate, STEP, early);
    let expected_late = analytic_crossing(candidate, STEP, late);
    for (signal, expected) in [("woke_a", expected_early), ("woke_b", expected_late)] {
        let woke = read_real_bits(&driver.host, signal);
        assert!(
            (woke - expected).abs() <= 1.0e-21,
            "`{signal}` must read its own crossing {expected:.16e} s, read {woke:.16e} s"
        );
    }
    assert!(
        expected_early < expected_late,
        "the fixture must place one crossing before the other: {expected_early:.16e} s \
         against {expected_late:.16e} s"
    );
    driver.accept();

    let first = driver.next_event();
    assert_eq!(
        first.to_bits(),
        tick_to_seconds((expected_early / tick_seconds()).round() as u64 + 1).to_bits(),
        "the earlier crossing's reaction is one tick after its own tick, saw {first:.16e} s"
    );
    driver.step_to(first, &high);
    assert_eq!(
        driver
            .host
            .read_digital("ya")
            .expect("the first reaction is readable"),
        "1",
        "the earlier crossing must have published and woken its own process first"
    );
    assert_eq!(
        driver
            .host
            .read_digital("yb")
            .expect("the second reaction is readable"),
        "0",
        "the later crossing must not have been dragged onto the earlier one's tick"
    );

    let second = driver.scheduled().unwrap_or_else(|| {
        panic!("the later crossing owes an activation of its own after {first:.16e} s")
    });
    assert_eq!(
        second.to_bits(),
        tick_to_seconds((expected_late / tick_seconds()).round() as u64 + 1).to_bits(),
        "the later crossing's reaction is one tick after *its* tick, saw {second:.16e} s"
    );
    driver.step_to(second, &high);
    assert_eq!(
        driver
            .host
            .read_digital("yb")
            .expect("the second reaction is readable"),
        "1",
        "the later crossing must still reach its own process"
    );
}

/// **Property 5, case b′.** A crossing a later Newton iteration of the same
/// trial finds is never published at a tick the trial has already left.
///
/// The interpolated instant of such a crossing is not ordered against the
/// instants of the crossings already published: the second iterate is a
/// different solution over the *whole* step, so its root can sit anywhere
/// inside the interval — including before a crossing the first iterate
/// published and the discrete half has already reacted to. Here `ca` crosses
/// 0.6 of the way through the step on the first iterate and publishes at the
/// tick after the trial's; the `posedge ca` process writes `gain`, the analog
/// half reads it, and the re-solve puts `cb`'s root 0.2 of the way through the
/// step, which rounds back onto the tick the trial started on.
///
/// The digital clock may not run backwards inside one trial. `$abstime` still
/// answers "when did this happen" and is each crossing's own instant — the two
/// are separately reported precisely so the tick can be clamped without lying
/// about the physics — but `$realtime` is where the process sits on the wheel,
/// and an effect dated a tick before a cause it followed is not a time base.
/// The second publication is therefore clamped forward onto the trial's
/// high-water mark, exactly as an interior crossing is clamped forward onto
/// the trial's own tick.
///
/// The second solution is handed to the host rather than solved for, because a
/// standalone host has no solver: the two vectors are the two iterates a
/// coupled engine would have produced.
#[test]
fn a_crossing_found_on_a_later_iteration_publishes_no_earlier_than_one_already_published() {
    const START: f64 = 10.0e-9;
    const STEP: f64 = 0.9e-9;

    let mut host = MixedSignalHost::compile(
        FEEDBACK_CROSSINGS,
        None,
        "xfeedback",
        &[1, 0],
        SchedulerLimits::default(),
    )
    .expect("the feedback-crossing module compiles");
    host.add_adc_bridge("ca", 0, (2, 0), CROSSING_THRESHOLD, CROSSING_THRESHOLD)
        .expect("the first A/D bridge is declarable");
    host.add_adc_bridge("cb", 0, (3, 0), CROSSING_THRESHOLD, CROSSING_THRESHOLD)
        .expect("the second A/D bridge is declarable");

    let quiet = [0.0, 0.0, 0.0];
    let mut driver = CrossingDriver::new(host, &quiet);
    driver.step_to(START, &quiet);

    let candidate = START + STEP;
    let late = ramp_reaching_threshold_at(0.6 / 0.9);
    let early = ramp_reaching_threshold_at(0.2 / 0.9);
    // The first iterate: only `ca` has moved, and it crossed in the upper half
    // of the trial's tick, so it publishes at the tick after it.
    driver.begin_and_settle(candidate, &[0.0, late, 0.0]);
    // The re-solve `gain` caused: `cb` now crosses too, in the lower half.
    driver.settle(&[0.0, late, early]);

    let crossing_a = analytic_crossing(candidate, STEP, late);
    let crossing_b = analytic_crossing(candidate, STEP, early);
    assert!(
        crossing_b < crossing_a,
        "the fixture must find the second crossing at an earlier instant than the \
         first: {crossing_b:.16e} s against {crossing_a:.16e} s"
    );

    for (signal, expected) in [("at_a", crossing_a), ("at_b", crossing_b)] {
        let woke = read_real_bits(&driver.host, signal);
        assert!(
            (woke - expected).abs() <= 1.0e-21,
            "`{signal}` must read its own crossing {expected:.16e} s, read {woke:.16e} s"
        );
    }

    // `$realtime` is in module units, which are ticks here.
    let first_tick = read_real_bits(&driver.host, "rt_a");
    let second_tick = read_real_bits(&driver.host, "rt_b");
    let nearest_a = (crossing_a / tick_seconds()).round();
    assert_eq!(
        first_tick.to_bits(),
        nearest_a.to_bits(),
        "the fixture needs the first publication past the trial's own tick {}: it \
         landed on {first_tick}",
        floor_ticks(candidate)
    );
    assert!(
        second_tick >= first_tick,
        "a crossing published after another one in the same trial may not be dated \
         before it: $realtime was {second_tick} after {first_tick}"
    );

    // And the wheel agrees: nothing the second publication scheduled comes due
    // before the reaction of the publication it followed.
    driver.accept();
    let due = driver.next_event();
    let earliest = tick_to_seconds(nearest_a as u64 + 1);
    assert!(
        due >= earliest,
        "the first reaction due after the trial is at {due:.16e} s, before the \
         {earliest:.16e} s one tick after the first publication"
    );
}

/// A five-nanosecond HDL clock, declared in nanoseconds.
const CEIL_CLOCK: &str = r#"
`timescale 1ns/1ps
module ceil_clock(clk); output clk; reg clk; initial clk=0; always #5 clk=~clk; endmodule
"#;

/// A divider whose unit is one tick, so its `#1` is one tick after whatever
/// tick the code model's output was dated at.
const CEIL_DIVIDER: &str = r#"
`timescale 1ps/1ps
module ceil_divider(fromx, q);
 input fromx; wire fromx;
 output q; reg q;
 initial q = 1'b0;
 always @(posedge fromx) #1 q = ~q;
endmodule
"#;

/// The picosecond grid the deck below runs on, from the resolution rather than
/// written out.
fn picosecond_tick() -> f64 {
    TimeResolution::new(-12)
        .expect("a picosecond resolution is declarable")
        .seconds_per_tick()
}

/// The least tick of that grid whose own instant is not before `seconds`.
fn ceil_picosecond_ticks(seconds: f64) -> u64 {
    let scale = picosecond_tick();
    let mut ticks = (seconds / scale).ceil() as u64;
    while ticks > 0 && ((ticks - 1) as f64) * scale >= seconds {
        ticks -= 1;
    }
    while (ticks as f64) * scale < seconds {
        ticks += 1;
    }
    ticks
}

/// **Property 5, case c.** A process woken by an XSPICE code model off the
/// tick grid is dated at the tick at or after the wake, so a `#1` from it
/// takes at least one whole time unit.
///
/// The deck is the E2 shape: an HDL clock drives a `d_inverter` whose delay is
/// 100.4 ps — four tenths of a tick, in the half that rounds *down* — and the
/// inverter's output drives an HDL divider whose reaction is a single tick.
///
/// Both directions of the boundary are asserted, because the rule is one rule
/// and the halves only mean something together. The HDL-to-XSPICE direction is
/// exact: a clock edge on tick `T` reaches the code model at precisely the
/// instant `T` names, which is why the inverter's output lands on a whole
/// clock edge plus the declared delay and not on a quantized approximation of
/// it. The XSPICE-to-HDL direction is that map's inverse, which is the least
/// tick not before the instant. Dating the wake at the *nearest* tick instead
/// puts `$realtime` 0.4 ps before the wake actually happened, and the `#1`
/// then fires 0.6 ps after it — less than the one time unit it asked for.
#[test]
fn an_off_grid_xspice_wake_is_dated_at_the_tick_at_or_after_it() {
    /// The code model's declared propagation delay.
    const DELAY: f64 = 100.4e-12;
    const PERIOD: f64 = 5.0e-9;

    let clock = ModelFile::new("ceil_clock", CEIL_CLOCK);
    let divider = ModelFile::new("ceil_divider", CEIL_DIVIDER);
    let deck = format!(
        "* an off-grid code-model wake dated onto the HDL tick grid\n\
         Xclock clk ceil_clock\n\
         Ainv clk fromx inverter\n\
         .model inverter d_inverter (rise_delay=100.4p fall_delay=100.4p)\n\
         Xdivider fromx qdiv ceil_divider\n\
         R1 qdiv out 1k\nC1 out 0 10p\n\
         .va \"{}\" ceil_clock\n.va \"{}\" ceil_divider\n.end\n",
        clock.deck_path(),
        divider.deck_path()
    );
    let result = run(&deck, 41.0e-9, 1.0e-9);

    // The HDL-to-XSPICE half: every clock edge reaches the model at the exact
    // instant its tick names, so every inverter output edge is one of those
    // instants plus the declared delay.
    let clk = digital_points(&result, "clk");
    let fromx = digital_points(&result, "fromx");
    assert!(clk.len() >= 8, "the clock must run: {clk:?}");
    for (index, (time, _)) in clk.iter().enumerate().skip(1) {
        let expected = index as f64 * PERIOD;
        assert!(
            (time - expected).abs() < 2.0e-20,
            "clock edge {index} must land on its own tick, {time:.16e} s against \
             {expected:.16e} s"
        );
    }
    for (index, (time, _)) in fromx.iter().enumerate().skip(1) {
        let expected = index as f64 * PERIOD + DELAY;
        assert!(
            (time - expected).abs() < 2.0e-20,
            "code-model edge {index} must land a whole declared delay after an exact \
             clock tick, {time:.16e} s against {expected:.16e} s"
        );
    }

    // The XSPICE-to-HDL half: each rising output edge wakes the divider, and
    // its `#1` lands one tick after the tick that wake was dated at. The
    // inverter's own initial level is one of those edges — the model reports it
    // at time zero, which is on the grid, and the divider reacts to it — so it
    // is counted here even though the timing loop above skipped it.
    let rising: Vec<f64> = fromx
        .iter()
        .filter(|(_, state)| state.as_str() == "One")
        .map(|(time, _)| *time)
        .collect();
    let reactions = digital_points(&result, "qdiv");
    assert!(
        rising.len() >= 3,
        "the fixture needs several code-model wakes: {fromx:?}"
    );
    assert_eq!(
        reactions.len(),
        rising.len() + 1,
        "one reaction per wake, plus the initial level: {reactions:?} against {rising:?}"
    );
    for (index, (wake, (reaction, _))) in rising.iter().zip(reactions.iter().skip(1)).enumerate() {
        let dated = ceil_picosecond_ticks(*wake);
        let expected = (dated + 1) as f64 * picosecond_tick();
        assert!(
            (reaction - expected).abs() < 2.0e-20,
            "reaction {index} to a wake at {wake:.16e} s must fire one tick after tick \
             {dated}, at {expected:.16e} s, saw {reaction:.16e} s"
        );
        assert!(
            reaction - wake >= picosecond_tick(),
            "reaction {index} took {:.16e} s after a wake at {wake:.16e} s, which is less \
             than the one time unit its `#1` asked for",
            reaction - wake
        );
    }
}
