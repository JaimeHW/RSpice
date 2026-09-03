//! Chaos coverage for the pickle-state decoders that Rust can drive.
//!
//! A pickled result is machine-written state read back from an untrusted file:
//! `pickle.load` hands `_unpickle` whatever the file contained, so every
//! `rebuild_*` helper is a decoder at a trust boundary and gets the same
//! treatment the netlist parser gets in `rspice-core`.
//!
//! Two decoders are reachable as ordinary Rust functions — they report a
//! refusal as a `String` rather than as a `PyErr`, so no interpreter has to be
//! embedded to exercise their failure paths:
//!
//! * [`rebuild_transient_event_traces`], which restores the XSPICE digital and
//!   real event histories from their versioned, label-tagged state, and
//! * [`validate_transient_state`], the structural pass a restored transient
//!   result must survive before any accessor may read it.
//!
//! The invariant is the same one the netlist and artifact decoders hold:
//! **never a panic, never a success on corrupted state that claims a different
//! value, always a typed refusal.** "Claims a different value" is checked as a
//! fixed point — anything a decoder accepted must re-serialize to exactly the
//! state it was handed, bit for bit, so a decoder that silently repaired,
//! reordered, or defaulted a field fails here.
//!
//! The remaining `_unpickle` validators build a `PyErr` on refusal, which
//! attaches to the interpreter, so they belong to the Python test suite in
//! `tests/` rather than to a Rust unit test.

use std::panic::{AssertUnwindSafe, catch_unwind};

use rspice_core::engine::{
    DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint, TransientResult,
};
use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

use super::event_state::{
    TRANSIENT_STRUCTURE_STATE_VERSION, TransientEventPersistenceState,
    rebuild_transient_event_traces, transient_event_persistence_state,
};
use super::transient::validate_transient_state;

/// Deterministic xorshift64* stream, matching the netlist and artifact
/// robustness suites so a failure reproduces from the seed in the test.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }

    /// Values that break numeric validators: the non-finite ones, the
    /// signed zeros, and the extremes.
    fn hostile_f64(&mut self) -> f64 {
        const VALUES: [f64; 10] = [
            0.0,
            -0.0,
            1.0e-9,
            -1.0e-9,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MAX,
            f64::MIN_POSITIVE,
            123.456,
        ];
        VALUES[self.below(VALUES.len())]
    }

    /// A label chosen from the accepted set or from spellings that must not
    /// be accepted by accident: case variants, padding, near misses, noise.
    fn hostile_label(&mut self, accepted: &[&str]) -> String {
        let index = self.below(accepted.len() + 6);
        match accepted.get(index) {
            Some(label) => (*label).to_owned(),
            None => match index - accepted.len() {
                0 => String::new(),
                1 => accepted[0].to_ascii_uppercase(),
                2 => format!(" {}", accepted[0]),
                3 => format!("{}_", accepted[0]),
                4 => "\u{0}".to_owned(),
                _ => format!("noise-{}", self.next()),
            },
        }
    }
}

/// Run `case` for `rounds` seeded rounds, naming the round when a decoder
/// panics instead of refusing.
fn chaos<F: FnMut(&mut Rng)>(name: &str, seed: u64, rounds: usize, mut case: F) {
    let mut rng = Rng::new(seed);
    for round in 0..rounds {
        let outcome = catch_unwind(AssertUnwindSafe(|| case(&mut rng)));
        assert!(outcome.is_ok(), "{name} panicked in round {round}");
    }
}

//=============================================================================
// Event-history pickle state
//=============================================================================

const STATE_LABELS: &[&str] = &[
    "zero",
    "one",
    "unknown",
    "zero_resistive",
    "one_resistive",
    "unknown_resistive",
    "zero_high_z",
    "one_high_z",
    "unknown_high_z",
    "high_z",
];

const STRENGTH_LABELS: &[&str] = &["undetermined", "high_z", "resistive", "strong"];

fn valid_event_state() -> TransientEventPersistenceState {
    let digital = vec![DigitalTrace {
        node_name: "clk".to_string(),
        points: vec![
            DigitalTracePoint {
                time: 0.0,
                value: DigitalValue {
                    state: DigitalState::ZeroZ,
                    strength: DigitalStrength::HighZ,
                },
            },
            DigitalTracePoint {
                time: 1.0e-9,
                value: DigitalValue {
                    state: DigitalState::One,
                    strength: DigitalStrength::Strong,
                },
            },
        ],
    }];
    let real = vec![RealTrace {
        node_name: "ctrl".to_string(),
        points: vec![RealTracePoint {
            time: 2.0e-9,
            value: -0.5,
        }],
    }];
    transient_event_persistence_state(&digital, &real)
}

/// Damage one field of a valid event state.
fn mutate_event_state(rng: &mut Rng, state: &mut TransientEventPersistenceState) {
    match rng.below(8) {
        0 => state.0 = rng.below(4),
        1 => state.1.clear(),
        2 => state.2.clear(),
        3 => {
            if let Some(trace) = state.1.first().cloned() {
                state.1.push(trace);
            }
        }
        4 => {
            if let Some(trace) = state.1.first_mut() {
                trace.0 = rng.hostile_label(&["clk", "reset"]);
            }
        }
        5 => {
            if let Some(point) = state.1.first_mut().and_then(|trace| trace.1.first_mut()) {
                point.1 = rng.hostile_label(STATE_LABELS);
            }
        }
        6 => {
            if let Some(point) = state.1.first_mut().and_then(|trace| trace.1.first_mut()) {
                point.2 = rng.hostile_label(STRENGTH_LABELS);
            }
        }
        _ => {
            if let Some(point) = state.1.first_mut().and_then(|trace| trace.1.first_mut()) {
                point.0 = rng.hostile_f64();
            }
            if let Some(point) = state.2.first_mut().and_then(|trace| trace.1.first_mut()) {
                point.1 = rng.hostile_f64();
            }
        }
    }
}

#[test]
fn event_state_decoder_survives_chaos_without_inventing_a_value() {
    chaos(
        "rebuild_transient_event_traces",
        0x0F17_0001,
        4_000,
        |rng| {
            let mut state = valid_event_state();
            mutate_event_state(rng, &mut state);
            let Ok((digital, real)) = rebuild_transient_event_traces(Some(state.clone())) else {
                return;
            };
            assert_eq!(
                state.0, TRANSIENT_STRUCTURE_STATE_VERSION,
                "a state of another version was accepted"
            );
            // Anything accepted must re-serialize to exactly the state it came
            // from, bit for bit, so a silently repaired label or time is caught.
            let reserialized = transient_event_persistence_state(&digital, &real);
            assert_event_state_bitwise_eq(&reserialized, &state);
        },
    );
}

#[test]
fn every_declared_event_label_round_trips_and_nothing_else_is_accepted() {
    for state_label in STATE_LABELS {
        for strength_label in STRENGTH_LABELS {
            let state = (
                TRANSIENT_STRUCTURE_STATE_VERSION,
                vec![(
                    "n".to_string(),
                    vec![(
                        0.0,
                        (*state_label).to_string(),
                        (*strength_label).to_string(),
                    )],
                )],
                Vec::new(),
            );
            let (digital, real) = rebuild_transient_event_traces(Some(state.clone()))
                .expect("a declared label pair must decode");
            assert_event_state_bitwise_eq(
                &transient_event_persistence_state(&digital, &real),
                &state,
            );
        }
    }

    // A near miss is a refusal, never the nearest declared state.
    for label in ["Zero", "zero ", "zero_", "", "floating"] {
        let refused = rebuild_transient_event_traces(Some((
            TRANSIENT_STRUCTURE_STATE_VERSION,
            vec![(
                "n".to_string(),
                vec![(0.0, label.to_string(), "strong".to_string())],
            )],
            Vec::new(),
        )));
        assert!(
            refused.is_err(),
            "the undeclared state label {label:?} was accepted"
        );
    }
}

fn assert_event_state_bitwise_eq(
    left: &TransientEventPersistenceState,
    right: &TransientEventPersistenceState,
) {
    assert_eq!(left.0, right.0);
    assert_eq!(left.1.len(), right.1.len());
    for (left, right) in left.1.iter().zip(&right.1) {
        assert_eq!(left.0, right.0);
        assert_eq!(left.1.len(), right.1.len());
        for (left, right) in left.1.iter().zip(&right.1) {
            assert_eq!(left.0.to_bits(), right.0.to_bits());
            assert_eq!(left.1, right.1);
            assert_eq!(left.2, right.2);
        }
    }
    assert_eq!(left.2.len(), right.2.len());
    for (left, right) in left.2.iter().zip(&right.2) {
        assert_eq!(left.0, right.0);
        assert_eq!(left.1.len(), right.1.len());
        for (left, right) in left.1.iter().zip(&right.1) {
            assert_eq!(left.0.to_bits(), right.0.to_bits());
            assert_eq!(left.1.to_bits(), right.1.to_bits());
        }
    }
}

//=============================================================================
// Restored transient structure
//=============================================================================

fn valid_transient_result() -> TransientResult {
    TransientResult {
        time: vec![0.0, 1.0e-9, 2.0e-9],
        step_sizes: vec![0.0, 1.0e-9, 1.0e-9],
        voltages: vec![vec![0.0, 0.5, 1.0], vec![0.0, 0.25, 0.5]],
        branch_currents: vec![vec![0.0, -1.0e-3, -2.0e-3]],
        num_nodes: 2,
        node_names: vec!["in".to_string(), "out".to_string()],
        branch_names: vec!["v1".to_string()],
        digital_traces: vec![DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![DigitalTracePoint {
                time: 1.0e-9,
                value: DigitalValue {
                    state: DigitalState::One,
                    strength: DigitalStrength::Strong,
                },
            }],
        }],
        real_traces: vec![RealTrace {
            node_name: "ctrl".to_string(),
            points: vec![RealTracePoint {
                time: 1.0e-9,
                value: 0.25,
            }],
        }],
        device_op_traces: Vec::new(),
        store_traces: Vec::new(),
        fft_results: Vec::new(),
    }
}

/// Damage one structural relationship of a restored transient result.
fn mutate_transient_result(rng: &mut Rng, result: &mut TransientResult) {
    match rng.below(10) {
        0 => {
            result.time.pop();
        }
        1 => {
            result.step_sizes.pop();
        }
        2 => result.time.push(rng.hostile_f64()),
        3 => {
            if let Some(time) = result.time.first_mut() {
                *time = rng.hostile_f64();
            }
        }
        4 => {
            result.voltages.pop();
        }
        5 => {
            if let Some(series) = result.voltages.first_mut() {
                series.push(rng.hostile_f64());
            }
        }
        6 => {
            result.node_names.pop();
        }
        7 => {
            result.branch_currents.pop();
        }
        8 => {
            if let Some(point) = result
                .digital_traces
                .first_mut()
                .and_then(|trace| trace.points.first_mut())
            {
                point.time = rng.hostile_f64();
            }
        }
        _ => {
            if let Some(point) = result
                .real_traces
                .first_mut()
                .and_then(|trace| trace.points.first_mut())
            {
                point.time = rng.hostile_f64();
            }
        }
    }
}

#[test]
fn restored_transient_structure_validation_survives_chaos() {
    chaos("validate_transient_state", 0x0F17_0002, 4_000, |rng| {
        let mut result = valid_transient_result();
        mutate_transient_result(rng, &mut result);
        let Ok(()) = validate_transient_state(&result) else {
            return;
        };
        // A result the structural pass accepted is aligned: every retained
        // channel has exactly one sample per time point, and the axis is
        // finite and strictly increasing. Nothing here may be inferred from
        // an unretained channel's emptiness.
        let points = result.time.len();
        assert_eq!(result.step_sizes.len(), points);
        assert!(result.time.iter().all(|time| time.is_finite()));
        assert!(result.time.windows(2).all(|window| window[1] > window[0]));
        assert_eq!(result.voltages.len(), result.num_nodes);
        assert_eq!(result.node_names.len(), result.num_nodes);
        assert_eq!(result.branch_currents.len(), result.branch_names.len());
        for series in result.voltages.iter().chain(&result.branch_currents) {
            assert!(series.is_empty() || series.len() == points);
        }
    });
}

#[test]
fn the_unmutated_fixtures_are_themselves_valid() {
    // A chaos suite whose seed is already refused proves nothing, so the two
    // fixtures are asserted valid before anything damages them.
    let state = valid_event_state();
    let (digital, real) =
        rebuild_transient_event_traces(Some(state.clone())).expect("the event fixture decodes");
    assert_event_state_bitwise_eq(&transient_event_persistence_state(&digital, &real), &state);
    validate_transient_state(&valid_transient_result())
        .expect("the transient fixture is structurally valid");
}
