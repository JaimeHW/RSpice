//! Accepted-timestep sequence goldens: the AMS compatibility invariant **I1**.
//!
//! I1 states: *a deck with no digital/event content produces a bit-identical
//! accepted-timestep sequence*. Today that holds only structurally — for a
//! pure-analog circuit `next_xspice_event_time()` returns `None`
//! (`src/circuit/external_models.rs`) and `replace_runtime_breakpoints`
//! clears an already-empty list (`src/numerics/integration/breakpoint.rs`) —
//! and nothing pins it. `tests/determinism.rs` compares two runs of the *same*
//! build, so a uniform drift in timestep control passes it unnoticed.
//!
//! These goldens close that hole. Each deck's accepted `(time, step_size)`
//! sequence is checked in as bit patterns; any change to breakpoint
//! placement, LTE control, order selection, step growth/shrink policy, or the
//! event machinery's inertness moves the sequence and fails here.
//!
//! # What these goldens prove, and what they do not
//!
//! They prove **drift detection only**. A golden asserts that today's build
//! chooses the same steps yesterday's build chose. It says *nothing* about
//! whether those steps are physically right — a fixture can happily freeze a
//! defect. Never re-baseline to make a red test green.
//!
//! `golden_decks_match_closed_form_expectations` is the standing companion
//! that keeps the fixtures honest: every deck here was chosen because its
//! answer is available in closed form, and that test checks the waveforms
//! against the analytic values independently of the step sequence. It is what
//! made the initial baseline admissible, and it is the cheapest half of the
//! oracle evidence a re-baseline needs.
//!
//! # Re-baseline protocol
//!
//! A sequence change is a finding until proven otherwise. To re-baseline:
//!
//! 1. Identify the code change that moved the sequence. "It just moved" is
//!    not an answer; if you cannot name the cause, the run is a regression.
//! 2. Produce oracle evidence that the *waveforms* are still correct — the
//!    deck validated against ngspice/Xyce, or `.MEAS`-checked expectations.
//!    Bit-identity with the old sequence is not evidence and neither is the
//!    new sequence agreeing with itself.
//! 3. Regenerate: `RSPICE_BLESS_TIMESTEP_GOLDENS=1 cargo test -p rspice-core
//!    --test timestep_sequence_goldens`. The bless run rewrites the fixtures
//!    and then **fails on purpose** so a blessed tree cannot pass CI by
//!    accident.
//! 4. Commit the fixtures in their own commit whose message states the cause
//!    of the sequence change and cites the oracle evidence from step 2.
//!
//! # Environment preconditions
//!
//! The fixtures were captured with an env-free [`SimulationConfig::default`]
//! and no solver overrides. `RSPICE_SOLVER`, `RSPICE_PIVREL`, and
//! `RSPICE_PIVTOL` change the factorization and therefore the accepted steps,
//! so the tests below refuse to run when any of them is set rather than
//! reporting a phantom drift.
//!
//! # Fixture newlines
//!
//! Fixtures are written with `\n` and pinned to LF by a `.gitattributes`
//! entry for `crates/rspice-core/tests/testdata/timestep_goldens/**`; the
//! reader additionally tolerates CRLF (`str::lines` plus a per-line `trim`)
//! so a checkout predating that entry still compares correctly.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;

/// Setting this to `1` rewrites every fixture and fails the run.
const BLESS_ENV: &str = "RSPICE_BLESS_TIMESTEP_GOLDENS";

/// Environment variables that change solver numerics under the test's feet.
const NUMERIC_ENV_OVERRIDES: [&str; 3] = ["RSPICE_SOLVER", "RSPICE_PIVREL", "RSPICE_PIVTOL"];

/// One pure-analog transient deck whose accepted step sequence is pinned.
struct GoldenDeck {
    /// Fixture stem under `tests/testdata/timestep_goldens/`.
    name: &'static str,
    /// The timestep-control regime this deck is here to exercise.
    regime: &'static str,
    /// Deck source. Line 1 is the SPICE title line, never a directive.
    deck: &'static str,
    /// `.TRAN` stop time handed to the public API.
    tstop: f64,
    /// `.TRAN` printing/ceiling step handed to the public API.
    max_step: f64,
}

/// RC step response behind a PULSE source: pins source-breakpoint placement
/// and the post-breakpoint restart ramp on an otherwise linear circuit.
const RC_PULSE: GoldenDeck = GoldenDeck {
    name: "rc_pulse_breakpoints",
    regime: "source breakpoints on a linear RC step response",
    deck: "\
* I1 golden: RC step response driven by a PULSE source
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

/// Half-wave rectifier under sinusoidal drive: pins nonlinear Newton
/// interaction with charge-based LTE across conduction and cutoff.
const DIODE_RECTIFIER: GoldenDeck = GoldenDeck {
    name: "diode_rectifier_lte",
    regime: "nonlinear junction conduction plus charge LTE under SIN drive",
    deck: "\
* I1 golden: half-wave diode rectifier under sinusoidal drive
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

/// Series RLC ring-down after a step kick: pins oscillatory LTE control and
/// the trapezoidal/Gear order switch that lightly damped resonance provokes.
const RLC_RINGDOWN: GoldenDeck = GoldenDeck {
    name: "rlc_ringdown_oscillatory",
    regime: "oscillatory LTE control and order switching on a ringing RLC",
    deck: "\
* I1 golden: series RLC ring-down after a step kick
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
/// pins the step collapse and recovery around a hard conductance
/// discontinuity.
const SWITCH_DISCONTINUITY: GoldenDeck = GoldenDeck {
    name: "switch_discontinuity",
    regime: "hard conductance discontinuity from a switch on steep edges",
    deck: "\
* I1 golden: switched RC load with sub-nanosecond control edges
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

/// The RC deck again under `.OPTIONS METHOD=GEAR`: pins that a deck-level
/// integration-method choice still reaches the step controller, and that its
/// sequence is pinned separately from the trapezoidal default.
const RC_PULSE_GEAR: GoldenDeck = GoldenDeck {
    name: "rc_pulse_gear",
    regime: "deck-selected Gear integration over the RC breakpoint deck",
    deck: "\
* I1 golden: RC step response under .OPTIONS METHOD=GEAR
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

const GOLDEN_DECKS: [&GoldenDeck; 5] = [
    &RC_PULSE,
    &DIODE_RECTIFIER,
    &RLC_RINGDOWN,
    &SWITCH_DISCONTINUITY,
    &RC_PULSE_GEAR,
];

/// One accepted transient point: its absolute time and the integration
/// interval that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AcceptedStep {
    time_bits: u64,
    step_bits: u64,
}

impl AcceptedStep {
    fn time(self) -> f64 {
        f64::from_bits(self.time_bits)
    }

    fn step(self) -> f64 {
        f64::from_bits(self.step_bits)
    }
}

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("timestep_goldens")
}

fn golden_path(deck: &GoldenDeck) -> PathBuf {
    golden_dir().join(format!("{}.steps", deck.name))
}

fn blessing() -> bool {
    std::env::var(BLESS_ENV).as_deref() == Ok("1")
}

/// Refuse to compare against fixtures captured under different numerics.
fn assert_no_numeric_env_overrides() {
    for name in NUMERIC_ENV_OVERRIDES {
        assert!(
            std::env::var_os(name).is_none(),
            "{name} is set; the accepted-timestep goldens were captured with an \
             env-free SimulationConfig::default() and no solver overrides. Unset \
             {name} and rerun rather than re-baselining."
        );
    }
}

/// Run one deck through the public engine API.
fn run(deck: &GoldenDeck) -> TransientResult {
    let netlist =
        Netlist::parse(deck.deck).unwrap_or_else(|err| panic!("deck {} parses: {err}", deck.name));
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, deck.tstop, deck.max_step)
        .unwrap_or_else(|err| panic!("deck {} runs a transient: {err}", deck.name))
}

/// Capture the accepted step sequence, checking the two columns stay aligned.
fn capture(deck: &GoldenDeck) -> Vec<AcceptedStep> {
    let result = run(deck);
    assert_eq!(
        result.time.len(),
        result.step_sizes.len(),
        "deck {}: `time` and `step_sizes` must stay aligned",
        deck.name
    );
    assert!(
        result.time.len() >= 8,
        "deck {} accepted only {} points; a golden that short pins nothing",
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

/// Serialize a sequence: bit patterns first (the comparison key), decimals
/// beside them for humans. Header lines start with `#` and are ignored on
/// read.
fn render(deck: &GoldenDeck, steps: &[AcceptedStep]) -> String {
    let mut out = String::new();
    out.push_str("# RSpice accepted-timestep golden (AMS compatibility invariant I1)\n");
    out.push_str("#\n");
    let name = deck.name;
    let regime = deck.regime;
    let count = steps.len();
    let _ = writeln!(out, "# deck:    {name}");
    let _ = writeln!(out, "# regime:  {regime}");
    let _ = writeln!(out, "# tstop:   {:e} s", deck.tstop);
    let _ = writeln!(out, "# dtmax:   {:e} s", deck.max_step);
    let _ = writeln!(out, "# points:  {count}");
    out.push_str("#\n");
    out.push_str(
        "# THIS FIXTURE PROVES DRIFT DETECTION ONLY, NEVER CORRECTNESS. It records the\n\
         # accepted (time, step) sequence a past build chose; it cannot tell you that\n\
         # sequence was right, and it will freeze a defect just as faithfully as a fix.\n\
         #\n\
         # Re-baseline protocol -- all four steps, in order:\n\
         #   1. Name the code change that moved the sequence. \"It moved\" is a regression.\n\
         #   2. Produce oracle evidence that the deck's waveforms are still correct\n\
         #      (ngspice/Xyce comparison, or .MEAS-checked expectations). Agreement with\n\
         #      the new sequence itself is not evidence.\n\
         #   3. Regenerate with RSPICE_BLESS_TIMESTEP_GOLDENS=1 cargo test -p rspice-core\n\
         #      --test timestep_sequence_goldens (the bless run fails on purpose).\n\
         #   4. Commit the fixtures alone, with a message stating the cause of the change\n\
         #      and citing the oracle evidence from step 2.\n\
         #\n\
         # Columns: index, time bit pattern, step bit pattern, time (s), step (s).\n\
         # The two hex columns are the comparison key; the decimals are advisory.\n",
    );
    for (index, step) in steps.iter().enumerate() {
        let _ = writeln!(
            out,
            "{index:>6}  {:016x}  {:016x}  {:>24.17e}  {:>24.17e}",
            step.time_bits,
            step.step_bits,
            step.time(),
            step.step()
        );
    }
    out
}

/// Parse a fixture, tolerating either newline convention.
fn parse_golden(path: &Path, text: &str) -> Vec<AcceptedStep> {
    let mut steps = Vec::new();
    for (line_number, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let mut next = |column: &str| -> String {
            fields.next().map(str::to_string).unwrap_or_else(|| {
                panic!(
                    "{}:{}: missing {column} column in golden row {line:?}",
                    path.display(),
                    line_number + 1
                )
            })
        };
        let _index = next("index");
        let time_field = next("time bit pattern");
        let step_field = next("step bit pattern");
        let hex = |field: &str, column: &str| -> u64 {
            u64::from_str_radix(field, 16).unwrap_or_else(|err| {
                panic!(
                    "{}:{}: {column} {field:?} is not a 64-bit hex pattern: {err}",
                    path.display(),
                    line_number + 1
                )
            })
        };
        steps.push(AcceptedStep {
            time_bits: hex(&time_field, "time bit pattern"),
            step_bits: hex(&step_field, "step bit pattern"),
        });
    }
    steps
}

/// Compare a deck against its fixture, or rewrite the fixture under bless.
fn check(deck: &GoldenDeck) {
    assert_no_numeric_env_overrides();
    let path = golden_path(deck);
    let captured = capture(deck);

    if blessing() {
        std::fs::create_dir_all(golden_dir()).expect("golden directory is creatable");
        std::fs::write(&path, render(deck, &captured))
            .unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
        panic!(
            "goldens regenerated -- verify against oracle before committing ({} points written to {})",
            captured.len(),
            path.display()
        );
    }

    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "missing accepted-timestep golden {}: {err}. Regenerate with \
             {BLESS_ENV}=1 only after reading the re-baseline protocol at the top \
             of tests/timestep_sequence_goldens.rs.",
            path.display()
        )
    });
    let expected = parse_golden(&path, &text);

    let first_divergence = expected
        .iter()
        .zip(&captured)
        .position(|(want, got)| want != got);
    if let Some(index) = first_divergence {
        let want = expected[index];
        let got = captured[index];
        panic!(
            "deck {} ({}) drifted at accepted point {index}:\n  \
             golden time {:016x} ({:.17e}) step {:016x} ({:.17e})\n  \
             actual time {:016x} ({:.17e}) step {:016x} ({:.17e})\n\
             This is a timestep-control change, not a formatting one. Do not \
             re-bless without oracle evidence; see the protocol in {}.",
            deck.name,
            deck.regime,
            want.time_bits,
            want.time(),
            want.step_bits,
            want.step(),
            got.time_bits,
            got.time(),
            got.step_bits,
            got.step(),
            path.display(),
        );
    }
    assert_eq!(
        expected.len(),
        captured.len(),
        "deck {} ({}) kept its shared prefix but changed accepted-point count. \
         The timestep controller ran longer or stopped earlier; do not re-bless \
         without oracle evidence.",
        deck.name,
        deck.regime,
    );
}

#[test]
fn rc_pulse_breakpoint_sequence_matches_golden() {
    check(&RC_PULSE);
}

#[test]
fn diode_rectifier_lte_sequence_matches_golden() {
    check(&DIODE_RECTIFIER);
}

#[test]
fn rlc_ringdown_sequence_matches_golden() {
    check(&RLC_RINGDOWN);
}

#[test]
fn switch_discontinuity_sequence_matches_golden() {
    check(&SWITCH_DISCONTINUITY);
}

#[test]
fn rc_pulse_gear_sequence_matches_golden() {
    check(&RC_PULSE_GEAR);
}

/// Capture must be stable within a single process, or the fixtures above are
/// pinning noise rather than policy.
#[test]
fn capture_is_stable_across_runs() {
    assert_no_numeric_env_overrides();
    if blessing() {
        return;
    }
    for deck in GOLDEN_DECKS {
        let first = capture(deck);
        let second = capture(deck);
        assert_eq!(
            first, second,
            "deck {} produced two different accepted-step sequences in one process",
            deck.name
        );
    }
}

/// `.OPTIONS METHOD=GEAR` must actually reach the step controller. Without
/// this, `rc_pulse_gear` could silently become a duplicate of `rc_pulse` and
/// both goldens would still pass.
#[test]
fn gear_option_changes_the_accepted_sequence() {
    assert_no_numeric_env_overrides();
    let trapezoidal = capture(&RC_PULSE);
    let gear = capture(&RC_PULSE_GEAR);
    assert_ne!(
        trapezoidal, gear,
        "the METHOD=GEAR variant produced the same accepted-step sequence as the \
         trapezoidal default; the deck option is not reaching the integrator and \
         the rc_pulse_gear golden pins nothing new"
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
         The deck is no longer exercising the regime its golden claims to pin, \
         so the golden is now freezing something else."
    );
}

/// Every deck's waveform must still match the closed-form answer it was
/// chosen for. Without this, a fixture could faithfully pin a degenerate run
/// -- an all-zero solve, a clamped diode, a switch stuck open -- and stay
/// green forever.
#[test]
fn golden_decks_match_closed_form_expectations() {
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
         ring-down is not oscillating and the golden pins no oscillatory LTE"
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
/// sequence scheduled by `next_xspice_event_time()` instead of by LTE alone.
#[test]
fn golden_decks_stay_free_of_event_driven_content() {
    let engine = Engine::new(SimulationConfig::default());
    for deck in GOLDEN_DECKS {
        let netlist = Netlist::parse(deck.deck)
            .unwrap_or_else(|err| panic!("deck {} parses: {err}", deck.name));
        let circuit = engine
            .build_circuit(&netlist)
            .unwrap_or_else(|err| panic!("deck {} builds: {err}", deck.name));
        assert!(
            !circuit.has_xspice_devices(),
            "deck {} is a pure-analog I1 fixture but constructed XSPICE instances; \
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
