//! The Verilog-AMS real-net corpus, its harness, and the reference models that
//! stand in for an oracle.
//!
//! # Why there is no oracle here
//!
//! `verilog_oracles` compares RSpice against Icarus Verilog and Verilator, two
//! simulators written by other people. That works because those cases are IEEE
//! 1364-2005 Verilog and both tools implement it. These cases are Verilog-AMS
//! LRM 2.4 section 3.7's `wreal`, which neither tool implements in a form this
//! harness could hold to an answer, so pointing them at one would produce a
//! compile failure the harness would have to learn to ignore — and an ignored
//! failure is how an oracle arm stops meaning anything.
//!
//! So the corpus lives in its own directory with its own manifest, and the
//! Icarus and Verilator arms never see it. See `tests/verilog/ams/
//! ams-manifest.tsv`.
//!
//! # What stands in for one, and what that is worth
//!
//! A reference model per case: an independent computation of the expected trace
//! in Rust, written from the clause rather than read off a run. Weaker than two
//! foreign simulators agreeing, and stated as weaker. What makes it worth
//! something is that the model and the design are written in different
//! languages against the same paragraph, and each number's arithmetic is beside
//! it — so the two agreeing by accident would take two independent mistakes
//! that happen to cancel.
//!
//! The models below take the *stimulus* as input and produce a whole trace, not
//! a list of literals. A table of literals would say what the answer is; a
//! model says why, and a change to a vector row changes both sides at once.

use rspice_conformance::suites::verilog::{AmsCorpus, AmsPortValue, AmsStimulus, ams_corpus_dir};

/// Cases the corpus must contain, by name and by what they cover.
///
/// Pinned by name rather than counted, for the reason `verilog_oracles` pins
/// its own: a count tells you the corpus changed size, and this tells you which
/// mechanism stopped being covered.
const REQUIRED_CASES: [(&str, &str); 3] = [
    (
        "wreal_forms",
        "real nets end to end: ports, arithmetic, a bit-driven ladder, a \
         comparison to bits, and the section 3.7 value-change event",
    ),
    (
        "wreal_resolution",
        "the four resolved real-net spellings, and an undriven wreal",
    ),
    (
        "real_state",
        "a real value that survives a clock edge: a module-level `real` written \
         with `<=`, a process-local `real` across a suspension, a `parameter \
         real` folded into the recurrence, an `output real` variable port, and \
         the `$realtobits`/`$bitstoreal` round trip",
    ),
];

fn corpus() -> AmsCorpus {
    AmsCorpus::load(&ams_corpus_dir()).unwrap_or_else(|error| panic!("corpus must load: {error}"))
}

// ===========================================================================
// Corpus integrity
// ===========================================================================

#[test]
fn every_required_case_is_present_and_the_manifest_agrees_with_the_directory() {
    // `AmsCorpus::load` is what checks the manifest against the directory in
    // both directions; reaching here at all is that check passing.
    let corpus = corpus();
    for (name, covers) in REQUIRED_CASES {
        let case = corpus
            .case(name)
            .unwrap_or_else(|| panic!("`{name}` is missing; it covers {covers}"));
        assert!(
            !case.note.is_empty(),
            "`{name}` has no manifest note saying what it covers"
        );
    }
    assert_eq!(
        corpus.cases.len(),
        REQUIRED_CASES.len(),
        "a case was added without a row in REQUIRED_CASES"
    );
}

/// The corpus rule the sibling corpus states and this one inherits: everything
/// observable about a case travels through its module ports.
///
/// A design that printed its own results would be testing number formatting,
/// which for a real is exactly the thing hardest to compare and easiest to get
/// away with.
#[test]
fn no_case_reports_its_own_results() {
    for case in &corpus().cases {
        let source = std::fs::read_to_string(&case.source)
            .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", case.source.display()));
        for banned in ["$display", "$monitor", "$write", "$strobe"] {
            assert!(
                !source.contains(banned),
                "`{}` uses `{banned}`; everything observable must travel through a port",
                case.name
            );
        }
    }
}

/// A real port is spelled `real`, never a width, and the parser refuses the
/// other spelling by name so the two files cannot drift into disagreeing about
/// what zero means.
#[test]
fn a_zero_width_port_is_refused_rather_than_read_as_a_real() {
    let text = "# RSPICE-VERILOG-AMS-STIMULUS 1\n\
                module m\n\
                input a 0\n\
                output b real\n\
                step 10\n\
                settle 5\n\
                vector 1\n";
    let error = rspice_conformance::suites::verilog::ams::parse_stimulus(text)
        .expect_err("zero width is not a real port");
    assert!(error.contains("declared 'real'"), "{error}");
}

// ===========================================================================
// The reference models
// ===========================================================================

/// One row of an expected trace: the output values in stimulus order.
type Row = Vec<String>;

/// Render a real the way the engine's trace does.
///
/// Shared with the engine only in the sense that both use Rust's shortest
/// round-tripping form; the model does not call the engine to find out.
fn real(value: f64) -> String {
    format!("{value:?}")
}

/// `wreal_forms`, computed from Verilog-AMS LRM 2.4 section 3.7 and the design.
///
/// Every output is a closed form of the inputs, which is why the case is
/// combinational-in-real by construction: there is no state to integrate and
/// no time constant, so a disagreement is a semantic one rather than a
/// numerical one.
fn model_wreal_forms(stimulus: &AmsStimulus) -> Vec<Row> {
    let mut rows = Vec::with_capacity(stimulus.vectors.len());
    let mut moves: u8 = 0;
    // Section 3.7 again: a `wreal` starts at zero. So a first vector of `0.0`
    // moves nothing and wakes nothing, which is the property this initial value
    // is here to state rather than a starting guess.
    let mut previous: f64 = 0.0;
    for vector in &stimulus.vectors {
        let vin: f64 = vector[0].parse().expect("a real column");
        let code = &vector[1];

        // The ladder: each bit chooses between its weight and zero. Written as
        // the sum the design writes, in the same association, so that a
        // difference would be a difference in the semantics and not in the
        // order two exactly-representable numbers were added.
        let bit = |index: usize| code.as_bytes()[index] == b'1';
        let step3 = if bit(0) { 8.0 } else { 0.0 };
        let step2 = if bit(1) { 4.0 } else { 0.0 };
        let step1 = if bit(2) { 2.0 } else { 0.0 };
        let step0 = if bit(3) { 1.0 } else { 0.0 };
        let vout = (step3 + step2) + (step1 + step0);

        let raw = vin * 0.5 + 0.25;
        let vscaled = if raw > 2.0 { 2.0 } else { raw };
        let over = if vscaled >= 1.0 { "1" } else { "0" };

        // Section 3.7's event is a change of value. The count rises once per
        // *distinct* value, and the comparison is exact — an epsilon here would
        // be inventing a rule the clause does not have.
        if previous != vin {
            moves = moves.wrapping_add(1) & 0xF;
            previous = vin;
        }

        rows.push(vec![
            real(vout),
            real(vscaled),
            over.to_string(),
            format!("{moves:04b}"),
        ]);
    }
    rows
}

/// `wreal_resolution`, computed from each keyword's own arithmetic.
fn model_wreal_resolution(stimulus: &AmsStimulus) -> Vec<Row> {
    stimulus
        .vectors
        .iter()
        .map(|vector| {
            let hi = vector[0] == "1";
            let lo = vector[1] == "1";
            let first = if hi { 3.0 } else { 1.0 };
            let second = if lo { -1.0 } else { -4.0 };
            vec![
                real(first + second),
                real((first + second) / 2.0),
                real(first.min(second)),
                real(first.max(second)),
                // Section 3.7: "If no driver is connected to a wreal net, its
                // value shall be zero (0.0)."
                real(0.0),
            ]
        })
        .collect()
}

/// `real_state`, computed from the recurrence the design writes.
///
/// The one case here with memory, so the model is a loop over the vectors with
/// its own state rather than a map over them — which is the point: a model
/// without state could not disagree with a design that had lost its.
///
/// # Why this is `==` and not a tolerance
///
/// The design and the model evaluate the *same* recurrence in the same order
/// with the same `f64` arithmetic: `state + (vin - state) * K`, one subtraction,
/// one multiplication, one addition, left to right, exactly as both are
/// written. IEEE 754 makes each of those operations correctly rounded and
/// therefore deterministic, so the two traces agree bit for bit or the
/// evaluation order differs — and an evaluation order that differs is a defect
/// this test exists to catch, not rounding to be absorbed by an epsilon.
///
/// The engine renders a real in Rust's shortest round-tripping form, so
/// comparing the rendered strings is comparing the values.
///
/// # The region rules the trace depends on
///
/// * `state <= ...` defers to the nonblocking region (IEEE 1364-2005 section
///   11), so the right-hand side reads the *previous* sample's state. A model
///   that updated in place would drift away by the second sample.
/// * `pattern = $realtobits(state)` is blocking and runs in the active region,
///   so it also reads the previous sample's state — which is why `vround`
///   trails `vout` by exactly one sample.
/// * `acc = acc + vin` is the process-local accumulation, which crosses the
///   suspension and therefore sums every edge rather than the last one.
fn model_real_state(stimulus: &AmsStimulus) -> Vec<Row> {
    // The declared default of the design's `parameter real K`. Written here as
    // the same literal, because section 12.2 fixes it at elaboration and the
    // harness overrides nothing.
    const K: f64 = 0.25;

    let mut rows = Vec::with_capacity(stimulus.vectors.len());
    // Section 3.9: a `real` variable starts at zero. So does a real net
    // (Verilog-AMS LRM 2.4 section 3.7), and so does the real *variable* port
    // `vsum` before the first edge writes it.
    let mut state = 0.0f64;
    let mut acc = 0.0f64;
    let mut vsum = 0.0f64;
    let mut vround = 0.0f64;
    // A `wire` nothing has driven is `x`; the first vector's `0` is therefore
    // an x-to-0 transition, which is a falling edge and not a rising one.
    let mut previous_clk: Option<bool> = None;

    for vector in &stimulus.vectors {
        let clk = vector[0] == "1";
        let vin: f64 = vector[1].parse().expect("a real column");

        if clk && previous_clk == Some(false) {
            // Active region: both blocking processes read the state this edge
            // has not yet changed.
            vround = state;
            acc += vin;
            vsum = acc;
            // Nonblocking region: the deferred update lands.
            state += (vin - state) * K;
        }
        previous_clk = Some(clk);

        rows.push(vec![real(state), real(vsum), real(vround)]);
    }
    rows
}

fn model(case: &str, stimulus: &AmsStimulus) -> Vec<Row> {
    match case {
        "wreal_forms" => model_wreal_forms(stimulus),
        "wreal_resolution" => model_wreal_resolution(stimulus),
        "real_state" => model_real_state(stimulus),
        other => panic!("`{other}` has no reference model; add one beside the case"),
    }
}

// ===========================================================================
// RSpice against the reference models
// ===========================================================================

#[cfg(feature = "verilog-digital")]
#[test]
fn rspice_matches_the_reference_model_on_every_case() {
    use rspice_core::xspice::verilog::{DigitalPort, DigitalStimulus, run_digital_verilog};

    for case in &corpus().cases {
        let source = std::fs::read_to_string(&case.source)
            .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", case.source.display()));
        let port = |port: &&rspice_conformance::suites::verilog::AmsPort| DigitalPort {
            name: port.name.clone(),
            width: port.value.engine_width(),
        };
        let stimulus = DigitalStimulus {
            module: Some(case.stimulus.module.clone()),
            inputs: case.stimulus.inputs().iter().map(port).collect(),
            outputs: case.stimulus.outputs().iter().map(port).collect(),
            clock: None,
            step: case.stimulus.step,
            settle: case.stimulus.settle,
            vectors: case.stimulus.vectors.clone(),
        };
        let report = run_digital_verilog(&source, &stimulus)
            .unwrap_or_else(|error| panic!("`{}` must run: {error}", case.name));

        let expected = model(&case.name, &case.stimulus);
        assert_eq!(
            report.observations.len(),
            expected.len(),
            "`{}` produced {} observation(s) for {} vector(s)",
            case.name,
            report.observations.len(),
            expected.len()
        );
        for (observation, row) in report.observations.iter().zip(&expected) {
            let actual: Vec<String> = observation
                .values
                .iter()
                .map(|(_, value)| value.clone())
                .collect();
            assert_eq!(
                &actual, row,
                "`{}` disagrees with the reference model at vector {}",
                case.name, observation.step
            );
        }
    }
}

/// At least one real port and at least one four-state port in the corpus, so
/// the domain check the engine performs is exercised in both directions rather
/// than only in whichever one the cases happen to use.
#[test]
fn the_corpus_covers_both_value_domains() {
    let corpus = corpus();
    let mut reals = 0;
    let mut four_state = 0;
    for case in &corpus.cases {
        for port in &case.stimulus.ports {
            match port.value {
                AmsPortValue::Real => reals += 1,
                AmsPortValue::FourState { .. } => four_state += 1,
            }
        }
    }
    assert!(reals >= 4, "only {reals} real port(s) in the corpus");
    assert!(
        four_state >= 2,
        "only {four_state} four-state port(s); a real-net corpus with no bits in it \
         would not cover the boundary between the two"
    );
}
