//! A transient checkpointed inside its window resumes onto the steps the uninterrupted run takes.
//!
//! The run keeps its own window, lands on the checkpoint's time as on a breakpoint, and keeps
//! the integrator as it stood there: the step it proposed after it, the order and the
//! integration histories, as ngspice's `resume` keeps CKTdelta, CKTorder and the state vectors
//! (dctran.c, the `resume:` label). Each deck here has a source corner at the checkpoint's time,
//! so the uninterrupted run lands there too, and the run checkpointed and resumed is that run to
//! the bit.

use rspice_core::NoAbort;
use rspice_core::engine::{Engine, TransientResult, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_core::result_sink::VecSink;

/// An RC whose source has a second corner 20 ns after the one at the checkpoint, inside the
/// restart step the first one would take, and a diode rectifier and a CMOS inverter beside a
/// marker source whose corner is the checkpoint's time.
const DECKS: &[(&str, &str, f64, f64, f64)] = &[
    (
        "rc_close",
        "rc with a corner just after the checkpoint\nv1 in 0 pwl(0 0 10u 1 10.02u 0.5 20u 0)\n\
         r1 in out 1k\nc1 out 0 1n\n.tran 10n 20u 0 100n\n.end\n",
        10.0e-6,
        20.0e-6,
        100.0e-9,
    ),
    (
        "diode",
        "diode rectifier\nv1 in 0 sin(0 5 1k)\nd1 in out dmod\nr1 out 0 1k\nc1 out 0 10u\n\
         vb b 0 pwl(0 0 0.37m 1 2m 1)\nrb b 0 1k\n.model dmod d(is=1e-14 cjo=2p)\n\
         .tran 10u 2m 0 10u\n.end\n",
        0.37e-3,
        2.0e-3,
        10.0e-6,
    ),
    (
        "inverter",
        "cmos inverter\nvdd vdd 0 3.3\nvin in 0 pulse(0 3.3 0.1m 10u 10u 0.2m 0.5m)\n\
         m1 out in vdd vdd pmod w=20u l=1u\nm2 out in 0 0 nmod w=10u l=1u\ncl out 0 1p\n\
         vb m 0 pwl(0 0 0.37m 1 2m 1)\nrm m 0 1k\n\
         .model nmod nmos(level=1 vto=0.7 kp=100u)\n.model pmod pmos(level=1 vto=-0.7 kp=50u)\n\
         .tran 10u 2m 0 10u\n.end\n",
        0.37e-3,
        2.0e-3,
        10.0e-6,
    ),
];

/// The accepted times and the node voltages, to the bit.
fn bits(result: &TransientResult) -> Vec<Vec<u64>> {
    std::iter::once(&result.time)
        .chain(result.voltages.iter().filter(|column| !column.is_empty()))
        .map(|column| column.iter().map(|value| value.to_bits()).collect())
        .collect()
}

/// The first segment's rows, then the resumed segment's after its first, which repeats the
/// checkpoint's row.
fn joined(first: &TransientResult, resumed: &TransientResult) -> Vec<Vec<u64>> {
    assert_eq!(
        resumed.time.first().map(|time| time.to_bits()),
        first.time.last().map(|time| time.to_bits()),
        "the resumed run starts at the checkpoint's row"
    );
    bits(first)
        .into_iter()
        .zip(bits(resumed))
        .map(|(mut head, tail)| {
            head.extend_from_slice(&tail[1..]);
            head
        })
        .collect()
}

/// A run interrupted inside its window by its checkpoint, and resumed to its stop.
#[test]
fn a_run_checkpointed_inside_its_window_resumes_onto_the_uninterrupted_steps() {
    let engine = Engine::default();
    for &(label, deck, checkpoint_at, tstop, max_step) in DECKS {
        let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("{label}: {error}"));
        let whole = engine
            .run_tran_with_startup_mode_and_abort(
                &netlist,
                tstop,
                max_step,
                TransientStartupMode::OperatingPoint,
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let (first, end) = engine
            .run_tran_checkpointed_at_with_startup_mode_and_abort(
                &netlist,
                (tstop, max_step),
                checkpoint_at,
                TransientStartupMode::OperatingPoint,
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(
            first.time.last().copied(),
            Some(checkpoint_at),
            "{label}: the run stops at its checkpoint"
        );
        let checkpoint = end
            .into_checkpoint()
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let (resumed, _) = engine
            .run_tran_resume_with_abort(&netlist, &checkpoint, tstop, max_step, &NoAbort)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(
            joined(&first, &resumed),
            bits(&whole),
            "{label}: checkpointed and resumed, the run takes the uninterrupted steps"
        );
    }
}

/// A sink's accepted times and values, to the bit.
fn sink_bits(sink: &VecSink) -> Vec<Vec<u64>> {
    std::iter::once(sink.x())
        .chain(sink.columns().iter().map(Vec::as_slice))
        .map(|column| column.iter().map(|value| value.to_bits()).collect())
        .collect()
}

/// The first sink's rows, then the resumed sink's after its first, which repeats the checkpoint's
/// row.
fn joined_sinks(first: &VecSink, resumed: &VecSink) -> Vec<Vec<u64>> {
    assert_eq!(
        resumed.x().first().map(|time| time.to_bits()),
        first.x().last().map(|time| time.to_bits()),
        "the resumed run starts at the checkpoint's row"
    );
    sink_bits(first)
        .into_iter()
        .zip(sink_bits(resumed))
        .map(|(mut head, tail)| {
            head.extend_from_slice(&tail[1..]);
            head
        })
        .collect()
}

/// The same runs written to result sinks: interrupted inside the window and resumed to the stop,
/// the two sinks hold the uninterrupted run's rows; and a run checkpointed at its stop and resumed
/// past it takes the steps the in-memory segments take.
#[test]
fn checkpointed_and_resumed_runs_write_the_same_rows_to_a_result_sink() {
    let engine = Engine::default();
    let mode = TransientStartupMode::OperatingPoint;
    for &(label, deck, checkpoint_at, tstop, max_step) in DECKS {
        let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("{label}: {error}"));
        let fail = |error: rspice_core::SimulationError| -> ! { panic!("{label}: {error}") };
        let mut whole = VecSink::new();
        engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                &netlist, tstop, max_step, mode, &NoAbort, &mut whole,
            )
            .unwrap_or_else(|error| fail(error));
        let mut first = VecSink::new();
        let (_, end) = engine
            .run_tran_checkpointed_at_into_sink_with_startup_mode_and_abort(
                &netlist,
                (tstop, max_step),
                checkpoint_at,
                mode,
                &NoAbort,
                &mut first,
            )
            .unwrap_or_else(|error| fail(error));
        let checkpoint = end.into_checkpoint().unwrap_or_else(|error| fail(error));
        let mut resumed = VecSink::new();
        engine
            .run_tran_resume_into_sink_with_abort(
                &netlist,
                &checkpoint,
                (tstop, max_step),
                &NoAbort,
                &mut resumed,
            )
            .unwrap_or_else(|error| fail(error));
        assert_eq!(
            joined_sinks(&first, &resumed),
            sink_bits(&whole),
            "{label}: interrupted and resumed into sinks, the run writes the uninterrupted rows"
        );

        // Checkpointed at its stop, then resumed past it: the sinks hold the times memory does.
        let (in_memory, end) = engine
            .run_tran_checkpointed_with_startup_mode_and_abort(
                &netlist, tstop, max_step, mode, &NoAbort,
            )
            .unwrap_or_else(|error| fail(error));
        let checkpoint = end.into_checkpoint().unwrap_or_else(|error| fail(error));
        let (in_memory_resumed, _) = engine
            .run_tran_resume_with_abort(&netlist, &checkpoint, 1.5 * tstop, max_step, &NoAbort)
            .unwrap_or_else(|error| fail(error));
        let mut first = VecSink::new();
        let (_, end) = engine
            .run_tran_checkpointed_into_sink_with_startup_mode_and_abort(
                &netlist,
                (tstop, max_step),
                mode,
                &NoAbort,
                &mut first,
            )
            .unwrap_or_else(|error| fail(error));
        let checkpoint = end.into_checkpoint().unwrap_or_else(|error| fail(error));
        let mut resumed = VecSink::new();
        engine
            .run_tran_resume_into_sink_with_abort(
                &netlist,
                &checkpoint,
                (1.5 * tstop, max_step),
                &NoAbort,
                &mut resumed,
            )
            .unwrap_or_else(|error| fail(error));
        assert_eq!(
            joined_sinks(&first, &resumed)[0],
            joined(&in_memory, &in_memory_resumed)[0],
            "{label}: checkpointed at its stop and resumed past it, the sinks take memory's steps"
        );
    }
}
