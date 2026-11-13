//! Accepted-timestep sequence invariant **I1** for the AMS engine.
//!
//! I1 states: *a deck with no digital/event content produces a bit-identical
//! accepted-timestep sequence* within a build. Each deck below exercises one
//! timestep-control regime (source breakpoints, junction LTE, oscillatory
//! order switching, a switch discontinuity, Gear). `capture_is_stable_across_runs`
//! requires two runs in one process to accept bit-identical `(time, step_size)`
//! sequences; `decks_stay_free_of_event_driven_content` keeps the decks pure
//! analog; `decks_match_closed_form_expectations` checks each waveform against
//! its closed-form answer so a degenerate run cannot pass as a stable one.

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;

/// One pure-analog transient deck chosen for a timestep-control regime.
struct TimestepDeck {
    /// Name used in failure messages.
    name: &'static str,
    /// Deck source. Line 1 is the SPICE title line, never a directive.
    deck: &'static str,
    /// `.TRAN` stop time handed to the public API.
    tstop: f64,
    /// `.TRAN` printing/ceiling step handed to the public API.
    max_step: f64,
}

/// RC step response behind a PULSE source: exercises source-breakpoint placement
/// and the post-breakpoint restart ramp on an otherwise linear circuit.
const RC_PULSE: TimestepDeck = TimestepDeck {
    name: "rc_pulse_breakpoints",
    deck: "\
* I1: RC step response driven by a PULSE source
vin in 0 pulse(0 5 1u 20n 20n 4u 10u)
r1 in out 10k
c1 out 0 1n
rleak out 0 10meg
.tran 500n 20u
.end
",
    tstop: 20.0e-6,
    max_step: 500.0e-9,
};

/// Half-wave rectifier under sinusoidal drive: exercises nonlinear Newton
/// interaction with charge-based LTE across conduction and cutoff.
const DIODE_RECTIFIER: TimestepDeck = TimestepDeck {
    name: "diode_rectifier_lte",
    deck: "\
* I1: half-wave diode rectifier under sinusoidal drive
vin in 0 sin(0 5 100k)
d1 in out dmod
rl out 0 2k
cl out 0 10n
.model dmod d is=1e-14 n=1.6 rs=1 cjo=2p vj=0.7 m=0.5 tt=5n
.tran 200n 30u
.end
",
    tstop: 30.0e-6,
    max_step: 200.0e-9,
};

/// Series RLC ring-down after a step kick: exercises oscillatory LTE control and
/// the trapezoidal/Gear order switch that lightly damped resonance provokes.
const RLC_RINGDOWN: TimestepDeck = TimestepDeck {
    name: "rlc_ringdown_oscillatory",
    deck: "\
* I1: series RLC ring-down after a step kick
vin in 0 pulse(0 1 0 1n 1n 1m 2m)
r1 in a 5
l1 a b 10u
c1 b 0 1n
.tran 100n 15u
.end
",
    tstop: 15.0e-6,
    max_step: 100.0e-9,
};

/// Switched resistive-capacitive load with sub-nanosecond control edges:
/// exercises the step collapse and recovery around a hard conductance
/// discontinuity.
const SWITCH_DISCONTINUITY: TimestepDeck = TimestepDeck {
    name: "switch_discontinuity",
    deck: "\
* I1: switched RC load with sub-nanosecond control edges
vsup sup 0 12
vctl ctl 0 pulse(0 5 200n 1n 1n 1u 2.5u)
s1 sup out ctl 0 smod
rl out 0 50
cl out 0 100p
.model smod sw (vt=2.5 vh=0.1 ron=0.5 roff=1e9)
.tran 50n 6u
.end
",
    tstop: 6.0e-6,
    max_step: 50.0e-9,
};

/// The RC deck again under `.OPTIONS METHOD=GEAR`: checks that a deck-level
/// integration-method choice still reaches the step controller.
const RC_PULSE_GEAR: TimestepDeck = TimestepDeck {
    name: "rc_pulse_gear",
    deck: "\
* I1: RC step response under .OPTIONS METHOD=GEAR
vin in 0 pulse(0 5 1u 20n 20n 4u 10u)
r1 in out 10k
c1 out 0 1n
rleak out 0 10meg
.options method=gear maxord=2
.tran 500n 20u
.end
",
    tstop: 20.0e-6,
    max_step: 500.0e-9,
};

const DECKS: [&TimestepDeck; 5] = [
    &RC_PULSE,
    &DIODE_RECTIFIER,
    &RLC_RINGDOWN,
    &SWITCH_DISCONTINUITY,
    &RC_PULSE_GEAR,
];

/// One accepted transient point: its absolute time and the integration
/// interval that produced it, as bit patterns so equality is exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AcceptedStep {
    time_bits: u64,
    step_bits: u64,
}

/// Run one deck through the public engine API.
fn run(deck: &TimestepDeck) -> TransientResult {
    let netlist =
        Netlist::parse(deck.deck).unwrap_or_else(|err| panic!("deck {} parses: {err}", deck.name));
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, deck.tstop, deck.max_step)
        .unwrap_or_else(|err| panic!("deck {} runs a transient: {err}", deck.name))
}

/// Capture the accepted step sequence, checking the two columns stay aligned.
fn capture(deck: &TimestepDeck) -> Vec<AcceptedStep> {
    let result = run(deck);
    assert_eq!(
        result.time.len(),
        result.step_sizes.len(),
        "deck {}: `time` and `step_sizes` must stay aligned",
        deck.name
    );
    assert!(
        result.time.len() >= 8,
        "deck {} accepted only {} points; a sequence that short checks nothing",
        deck.name,
        result.time.len()
    );
    result
        .time
        .iter()
        .zip(&result.step_sizes)
        .map(|(time, step)| AcceptedStep {
            time_bits: time.to_bits(),
            step_bits: step.to_bits(),
        })
        .collect()
}

/// Two runs of the same pure-analog deck in one process accept the same steps.
#[test]
fn capture_is_stable_across_runs() {
    for deck in DECKS {
        let first = capture(deck);
        let second = capture(deck);
        assert_eq!(
            first, second,
            "deck {} produced two different accepted-step sequences in one process",
            deck.name
        );
    }
}

/// `.OPTIONS METHOD=GEAR` must actually reach the step controller.
#[test]
fn gear_option_changes_the_accepted_sequence() {
    let trapezoidal = capture(&RC_PULSE);
    let gear = capture(&RC_PULSE_GEAR);
    assert_ne!(
        trapezoidal, gear,
        "the METHOD=GEAR variant produced the same accepted-step sequence as the \
         trapezoidal default; the deck option is not reaching the integrator"
    );
}

/// Peak, trough, and mean-crossing summary of one node's waveform.
fn summary(result: &TransientResult, node: &str, mean: f64) -> (f64, f64, usize) {
    let waveform = result
        .try_voltage_waveform_named(node)
        .unwrap_or_else(|| panic!("node {node} in {:?}", result.node_names));
    let min = waveform.iter().copied().fold(f64::INFINITY, f64::min);
    let max = waveform.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let crossings = waveform
        .windows(2)
        .filter(|pair| (pair[0] - mean) * (pair[1] - mean) < 0.0)
        .count();
    (min, max, crossings)
}

#[track_caller]
fn assert_within(label: &str, actual: f64, low: f64, high: f64) {
    assert!(
        actual >= low && actual <= high,
        "{label} is {actual:.6}, outside the closed-form band [{low}, {high}]. \
         The deck is no longer exercising the timestep regime it was chosen for."
    );
}

/// Every deck's waveform must still match the closed-form answer it was
/// chosen for, so a degenerate run -- an all-zero solve, a clamped diode, a
/// switch stuck open -- cannot pass as a stable step sequence.
#[test]
fn decks_match_closed_form_expectations() {
    // RC: 10k * 1n = 10 us. The 4 us pulse charges to 5(1 - e^-0.4) = 1.648 V,
    // 6 us of discharge leaves 1.648 e^-0.6 = 0.904 V, and the second pulse
    // reaches 5 - (5 - 0.904) e^-0.4 = 2.255 V.
    let rc = run(&RC_PULSE);
    let (rc_min, rc_max, _) = summary(&rc, "OUT", 1.0);
    assert_within("RC v(out) second-pulse peak", rc_max, 2.20, 2.32);
    assert_within("RC v(out) floor", rc_min, -1.0e-6, 1.0e-3);
    let (_, drive_max, _) = summary(&rc, "IN", 1.0);
    assert_within("RC v(in) pulse amplitude", drive_max, 4.999, 5.001);

    // Gear must solve the same physics as the trapezoidal default; only the
    // step sequence is allowed to differ.
    let gear = run(&RC_PULSE_GEAR);
    let (_, gear_max, _) = summary(&gear, "OUT", 1.0);
    assert_within("Gear RC v(out) second-pulse peak", gear_max, 2.20, 2.32);
    assert!(
        (gear_max - rc_max).abs() <= 0.01 * rc_max,
        "Gear peaked at {gear_max:.6} against the trapezoidal {rc_max:.6}; a \
         method change may move the steps but not the answer"
    );

    // Rectifier: half-wave, so the trough sits at ground. The peak is the 5 V
    // drive minus a junction drop of n Vt ln(I/Is) = 1.6 * 25.85m *
    // ln(2m / 1e-14) = 1.08 V, leaving 3.92 V.
    let diode = run(&DIODE_RECTIFIER);
    let (diode_min, diode_max, _) = summary(&diode, "OUT", 1.0);
    assert_within("rectified v(out) peak", diode_max, 3.80, 4.00);
    assert_within("rectified v(out) trough", diode_min, -1.0e-3, 1.0e-3);

    // RLC: zeta = (R/2) sqrt(C/L) = 2.5 * 0.01 = 0.025, so the step overshoot
    // is 1 + exp(-pi zeta / sqrt(1 - zeta^2)) = 1.925 V. It rings at
    // 1/(2 pi sqrt(LC)) = 1.59 MHz, roughly 24 cycles inside the 15 us
    // window, and every cycle crosses the 1 V asymptote twice.
    let rlc = run(&RLC_RINGDOWN);
    let (_, rlc_max, rlc_crossings) = summary(&rlc, "B", 1.0);
    assert_within("RLC v(b) first overshoot", rlc_max, 1.88, 1.97);
    assert!(
        rlc_crossings >= 30,
        "RLC v(b) crossed its 1 V asymptote only {rlc_crossings} times; the \
         ring-down is not oscillating and exercises no oscillatory LTE"
    );

    // Switch: closed is a 0.5 / 50 ohm divider off the 12 V rail, 11.881 V;
    // open is 12 * 50 / (50 + 1e9), which is microvolts.
    let switched = run(&SWITCH_DISCONTINUITY);
    let (switch_min, switch_max, _) = summary(&switched, "OUT", 6.0);
    assert_within("closed-switch v(out)", switch_max, 11.82, 11.94);
    assert_within("open-switch v(out)", switch_min, -1.0e-3, 1.0e-3);
}

/// The structural half of I1: pure-analog decks must never touch the event
/// machinery. A deck that starts constructing XSPICE instances, or a
/// transient that starts emitting digital/real traces, gets the accepted-step
/// sequence scheduled by `next_xspice_activation()` instead of by LTE alone.
#[test]
fn decks_stay_free_of_event_driven_content() {
    let engine = Engine::new(SimulationConfig::default());
    for deck in DECKS {
        let netlist = Netlist::parse(deck.deck)
            .unwrap_or_else(|err| panic!("deck {} parses: {err}", deck.name));
        let circuit = engine
            .build_circuit(&netlist)
            .unwrap_or_else(|err| panic!("deck {} builds: {err}", deck.name));
        assert!(
            !circuit.has_xspice_devices(),
            "deck {} is a pure-analog I1 deck but constructed XSPICE instances; \
             its accepted steps would now be event-scheduled",
            deck.name
        );

        let result = run(deck);
        assert!(
            result.digital_traces.is_empty(),
            "deck {} emitted {} digital trace(s); the event machinery did not stay inert",
            deck.name,
            result.digital_traces.len()
        );
        assert!(
            result.real_traces.is_empty(),
            "deck {} emitted {} real event trace(s); the event machinery did not stay inert",
            deck.name,
            result.real_traces.len()
        );
    }
}
