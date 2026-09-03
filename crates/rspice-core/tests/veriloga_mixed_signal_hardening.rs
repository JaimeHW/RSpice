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
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .expect("the deck runs")
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

    for (label, config) in [
        ("a one-iteration transient ceiling", starved()),
        ("tolerances no solve can meet", impossible()),
        ("both at once", both()),
    ] {
        let result = Engine::new(config)
            .run_tran(&netlist, 200.0e-9, 1.0e-9)
            .unwrap_or_else(|error| {
                panic!(
                    "with {label} the run must still complete or refuse with a convergence \
                     diagnostic; got: {error}"
                )
            });
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

/// **Cross-domain feedback.** A loop with no delay is diagnosed by name, not
/// simulated.
///
/// The deck is the smallest honest form of the failure: the module reads `cin`
/// across an A/D bridge, drives its inverse onto `yout` across a D/A bridge,
/// and a one-ohm feedback resistor makes `cin` follow `yout`. Every accepted
/// analog solution therefore flips the boundary, and the flipped boundary is
/// what the next solution is solved against.
///
/// Three things this must *not* be, and each was a live possibility:
///
/// * a hang — the delta-cycle kernel never sees this loop, because the flip
///   crosses a Newton solve rather than a delta cycle;
/// * `MAX_BOUNDARY_SETTLE_PASSES` — that ceiling is unreachable here, because
///   `settle_analog_bridges` is idempotent on one solution vector and the
///   engine's settle loop hands it the same one every pass;
/// * a trace — which is what it was. The run completed, with one boundary
///   transition at every accepted timepoint and no diagnostic at all.
///
/// What it is now is the participants: both nets, the circuit node each is on,
/// which way each faces, and the alternation each was holding.
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
        lowered.contains("consecutive accepted timepoints"),
        "the diagnostic must say what it counted: {error}"
    );
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
         rp p 0 1meg\n\
         ry yout 0 10k\n\
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
    host.add_adc_bridge("c", 2, 0, PROBE_THRESHOLD, PROBE_THRESHOLD)
        .expect("the A/D bridge is declarable");
    host.add_dac_bridge("y", 3, 0, 0.0, 3.3, 20.0)
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
