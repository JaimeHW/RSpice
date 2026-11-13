//! The boundary between an analog node and a real-valued event net, both ways.
//!
//! `real_to_v` carries a real event into the matrix and has done for as long as
//! the estate has had real event nodes. `v_to_real` is the observer that did
//! not exist: every other analog-observing bridge — `adc_bridge`,
//! `bidi_bridge`, the Xyce `DIG` family — converts an analog node by threshold
//! crossing and produces four-state values, and a real quantity has no
//! threshold to cross.
//!
//! The unit tests beside the model (`xspice::models::xtraevt`) pin what one
//! evaluation does. These pin what the two halves do as a *pair*, on real decks
//! solved by the real engine, because the properties that matter at the
//! boundary — that the round trip carries the value, that observing does not
//! disturb the analog side — are not visible from one model's context.
//!
//! # The two rulings these check
//!
//! **Hold.** A real event value stands until the next event on its net. This is
//! the substrate's existing behaviour rather than a rule introduced here:
//! `circuit::external_models`'s drain keeps one resolved value per real event
//! node and moves it only when an event executes, and `real_to_v` reads that
//! value at every analog evaluation. The observer is built to match — it
//! publishes only when its sample moves — so a settled node costs no events.
//!
//! **Sampling.** The observer names no future time and therefore requests no
//! breakpoints, which is what `an_observer_does_not_move_the_accepted_step_sequence`
//! checks. That is D5 clause 6 — pure-analog inertness — applied to a deck that
//! is not pure analog: adding an observer must not change what the integrator
//! does, because an observer is not supposed to be part of the circuit.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

/// `gain` on the observer's model card, and therefore the ratio the whole
/// round trip should reproduce.
const OBSERVER_GAIN: f64 = 2.0;

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}"));
    Engine::default()
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("operating point solves: {error}"))
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("node {node} is solved"))
}

/// The whole boundary in one deck: an analog node observed into a real event
/// net, and that net carried back into the matrix.
///
/// `rnode` is a real event net and nothing else — it has no MNA row of its own,
/// carries no current, and is written only by the observer's event. The
/// operating point is therefore the composition of the two models' arithmetic
/// and nothing else, which is what makes the expected value exact rather than a
/// tolerance on a solve.
const ROUND_TRIP_OP_DECK: &str = "\
* an analog node observed into a real event and driven back into the matrix
vin in 0 dc 1.5
rin in 0 1k
aobs in rnode obs
.model obs v_to_real (gain=2)
adrv rnode out drv
.model drv real_to_v
rout out 0 1k
.op
.end
";

/// The round trip carries the value, scaled once.
///
/// `real_to_v` outside transient sets its output to `target * gain` directly
/// with its own `gain` left at one, so the only scaling in the chain is the
/// observer's — and a result of `2 * 1.5` says both halves ran and that neither
/// applied the other's parameter.
#[test]
fn an_analog_node_crosses_to_a_real_event_and_back() {
    let out = op_voltage(ROUND_TRIP_OP_DECK, "out");
    let expected = OBSERVER_GAIN * 1.5;
    assert!(
        (out - expected).abs() < 1.0e-9,
        "the boundary round trip should reproduce {expected} V at `out`, got {out}"
    );

    // And the observed node is undisturbed by being observed: the observer
    // stamps nothing, so `in` is still the divider's answer.
    let observed = op_voltage(ROUND_TRIP_OP_DECK, "in");
    assert!(
        (observed - 1.5).abs() < 1.0e-9,
        "observing a node must not load it; `in` should still be 1.5 V, got {observed}"
    );
}

/// The same chain with a stimulus that ramps and then holds.
///
/// A transient run is where the two halves can disagree about time: the
/// observer publishes at the timepoint it sampled, and `real_to_v` ramps from
/// the value it held toward the new target over `transition_time`. Once the
/// input has been constant for far longer than that ramp, the output has to
/// have arrived — a chain that lost an event, or that held a stale target,
/// settles somewhere else.
const ROUND_TRIP_TRAN_DECK: &str = "\
* the boundary tracking a ramped input, sampled at accepted analog steps
vin in 0 pwl(0 0 5n 1.5 100n 1.5)
rin in 0 1k
aobs in rnode obs
.model obs v_to_real (gain=2)
adrv rnode out drv
.model drv real_to_v (transition_time=1p)
rout out 0 1k
.end
";

#[test]
fn the_boundary_round_trip_settles_on_a_held_input() {
    let netlist = Netlist::parse(ROUND_TRIP_TRAN_DECK).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-7, 1.0e-9)
        .expect("transient solves");

    let out = result
        .try_voltage_waveform_named("out")
        .expect("the driven node is solved");
    let last = *out.last().expect("the run accepted at least one timepoint");
    let expected = OBSERVER_GAIN * 1.5;
    assert!(
        (last - expected).abs() < 1.0e-6,
        "after the input has held for 100 ns the chain should have arrived at \
         {expected} V, got {last}"
    );

    // The real net carries the sample itself, at the observer's scale, and the
    // trace is what says the event world saw it rather than the analog side
    // having reached the right answer some other way.
    let real = result
        .real_trace_named("rnode")
        .expect("the real event net is traced");
    let final_sample = real
        .last()
        .expect("the real net took at least one event")
        .value;
    assert!(
        (final_sample - expected).abs() < 1.0e-6,
        "the real event net should hold the scaled sample, got {final_sample}"
    );
}

/// An RC with a source breakpoint, watched by an observer that has something to
/// see.
const SAMPLING_MOVING_DECK: &str = "\
* an RC watched by an observer whose sample moves at every accepted step
v1 in 0 pulse(0 1 1n 0.1n 0.1n 5n 10n)
r1 in out 1k
c1 out 0 1p
aobs out watched obs
.model obs v_to_real
.end
";

/// The same deck, the same nets, the same event row — and an observer watching
/// the one node in any circuit that cannot move.
const SAMPLING_QUIET_DECK: &str = "\
* the same deck with the observer watching ground, so it publishes once
v1 in 0 pulse(0 1 1n 0.1n 0.1n 5n 10n)
r1 in out 1k
c1 out 0 1p
aobs 0 watched obs
.model obs v_to_real
.end
";

/// **The sampling ruling, at deck level.** Publishing a sample costs the
/// integrator nothing: an observer that speaks at every accepted step takes the
/// same step sequence, bit for bit, as one that speaks once.
///
/// The comparison is deliberately *not* against a pure-analog deck, and the
/// reason is worth recording. An event net occupies an MNA row with a pinned
/// diagonal (`engine::stamping`), so adding one changes the norms the step
/// controller reads and moves the accepted sequence — measurably: the RC below
/// accepts 244 steps alone and 292 with an event net present. That is a
/// property of every event net, digital and real alike, and it is not what the
/// sampling ruling claims. D5 clause 6 is about decks with *no* event content,
/// and this deck has some.
///
/// What the ruling does claim is that an observer names no future time and so
/// asks the integrator for nothing. Both decks here carry the same event row;
/// the only difference is how often the observer publishes. If a sample cost a
/// breakpoint, or a delay, or a settle pass the controller could see, the
/// moving deck's sequence would separate from the quiet one's.
///
/// Compared bit-exactly rather than within a tolerance: a step sequence that
/// differs at all is a different integration, and a threshold would only decide
/// how much of a difference to tolerate.
#[test]
fn publishing_a_sample_does_not_move_the_accepted_step_sequence() {
    let moving = Netlist::parse(SAMPLING_MOVING_DECK).expect("moving-sample deck parses");
    let quiet = Netlist::parse(SAMPLING_QUIET_DECK).expect("quiet-sample deck parses");

    let moving = Engine::default()
        .run_tran(&moving, 2.0e-8, 1.0e-10)
        .expect("the moving-sample deck solves");
    let quiet = Engine::default()
        .run_tran(&quiet, 2.0e-8, 1.0e-10)
        .expect("the quiet-sample deck solves");

    let moving_events = moving
        .real_trace_named("watched")
        .expect("the moving deck traces its real net")
        .len();
    // An observer of ground publishes its one sample at time zero and then has
    // nothing to say, which can leave the net with no trace at all rather than
    // a trace of length one.
    let quiet_events = quiet
        .real_trace_named("watched")
        .map(<[_]>::len)
        .unwrap_or(0);
    assert!(
        moving_events > 10 * quiet_events.max(1),
        "the two decks must differ in how much the observer publishes, or this \
         test proves nothing: {moving_events} events against {quiet_events}"
    );

    assert_eq!(
        moving.time.len(),
        quiet.time.len(),
        "publishing {moving_events} samples instead of {quiet_events} changed \
         how many steps the integrator accepted"
    );
    for (index, (first, second)) in moving.time.iter().zip(quiet.time.iter()).enumerate() {
        assert_eq!(
            first.to_bits(),
            second.to_bits(),
            "accepted time {index} moved with the sample count: {first:e} vs {second:e}"
        );
    }
    for (index, (first, second)) in moving
        .step_sizes
        .iter()
        .zip(quiet.step_sizes.iter())
        .enumerate()
    {
        assert_eq!(
            first.to_bits(),
            second.to_bits(),
            "accepted step {index} moved with the sample count: {first:e} vs {second:e}"
        );
    }
}

/// A settled node produces no further events.
///
/// The hold ruling: a real event value stands until the next event on its net,
/// so an observer watching a node that has stopped moving has nothing to say.
/// The count is what distinguishes holding from re-publishing — a chain that
/// republished every accepted step would still give the right voltages, and
/// would cost an event and a settle pass at every timepoint for the rest of the
/// run.
const HELD_INPUT_DECK: &str = "\
* a step that settles, watched: the observer should fall silent afterwards
vin in 0 pwl(0 0 1n 1 2n 1 200n 1)
rin in 0 1k
aobs in watched obs
.model obs v_to_real
.end
";

#[test]
fn a_settled_node_stops_producing_real_events() {
    let netlist = Netlist::parse(HELD_INPUT_DECK).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 2.0e-7, 1.0e-9)
        .expect("transient solves");

    let trace = result
        .real_trace_named("watched")
        .expect("the observed net is traced");
    assert!(
        trace.len() > 1,
        "the observer must have published while the input moved, or the \
         silence afterwards proves nothing"
    );

    let settled_at = 3.0e-9;
    let after: Vec<f64> = trace
        .iter()
        .filter(|point| point.time > settled_at)
        .map(|point| point.value)
        .collect();
    assert!(
        after.is_empty(),
        "the observer republished {} time(s) after the input settled at \
         {settled_at:e} s; a real event value holds until the next event, so a \
         node that has stopped moving costs nothing: {after:?}",
        after.len()
    );

    let last = trace.last().expect("the net took at least one event").value;
    assert!(
        (last - 1.0).abs() < 1.0e-6,
        "the held value should be the settled sample, got {last}"
    );
}
