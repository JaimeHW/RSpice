//! The dual-representation agreement suite: every reference block modelled as
//! full analog and as an RNM must agree within a bound derived from its physics.
//!
//! The blocks, the decks, the designs and the bounds all live in
//! [`rspice_conformance::suites::verilog::rnm`], whose module documentation
//! carries the strategy, the front-end ceiling that shaped the block list, and
//! the time-alignment rule. This file is the gate over them.
//!
//! # What a failure here means
//!
//! Not "the tolerance is too tight". The two representations share no code
//! below this harness — one integrates a matrix through time, the other
//! evaluates real expressions on an event kernel — so a disagreement is a
//! disagreement about the block, and one of the two is wrong. The report says
//! which sample, what each side gave, and what the bound was made of, so the
//! reader can tell which.

#![cfg(feature = "verilog-digital")]

use rspice_conformance::suites::verilog::rnm::{self, RnmBlock};

/// Blocks this suite must contain, by name and by the mechanism each covers.
///
/// Pinned by name rather than counted, for the reason the sibling suites pin
/// theirs: a count says the list changed size, and this says which mechanism
/// stopped being covered.
const REQUIRED_BLOCKS: [(&str, &str); 4] = [
    (
        "r2r_dac",
        "bits to real: binary-weighted superposition, ladder against closed form",
    ),
    (
        "schmitt_hysteresis",
        "real to real with memory: thresholds that emerge from a feedback ratio",
    ),
    (
        "flash_quantizer",
        "real to bits and back: a reference string against arithmetic thresholds",
    ),
    (
        "ramp_integrator",
        "accumulation: charge on a capacitor against a counted step",
    ),
];

fn blocks() -> Vec<RnmBlock> {
    rnm::blocks()
}

// ===========================================================================
// The agreement gate
// ===========================================================================

#[test]
fn every_block_agrees_between_its_analog_and_rnm_representations() {
    let mut failures = String::new();
    println!(
        "\n{:<20} {:<10} {:>14} {:>14} {:>10}",
        "block", "signal", "bound(V)", "max err(V)", "margin"
    );
    for block in blocks() {
        let agreement = rnm::compare(&block)
            .unwrap_or_else(|error| panic!("`{}` could not be compared: {error}", block.name));
        for pair in &agreement.pairs {
            let bound = pair.pair.bound.total();
            println!(
                "{:<20} {:<10} {:>14.6e} {:>14.6e} {:>9.0}x",
                block.name,
                pair.pair.analog_node,
                bound,
                pair.max_error(),
                bound / pair.max_error().max(f64::MIN_POSITIVE),
            );
        }
        if !agreement.agrees() {
            failures.push_str(&agreement.failure_report());
        }
    }
    assert!(
        failures.is_empty(),
        "a block's two representations disagree beyond its declared bound. \
         The two share no code below this harness, so one of them is wrong:\n{failures}"
    );
}

/// Every declared bound must be a sum of *named* terms, and the observed error
/// must sit under it with room to spare.
///
/// The second half is what keeps a bound from being a golden. A bound that
/// hugged the observation would pass today and fail on the next machine whose
/// rounding differs; one with an order of margin is an analysis that happens to
/// be comfortably right. The margin is only asserted downward — a bound that is
/// far *looser* than the observation is not a defect here, because every bound
/// is dominated by the engine's own published accuracy promise and tightening
/// past that would be claiming an accuracy the engine does not offer.
#[test]
fn every_declared_bound_is_named_term_by_term_and_is_not_hugging_the_observation() {
    for block in blocks() {
        let agreement = rnm::compare(&block)
            .unwrap_or_else(|error| panic!("`{}` could not be compared: {error}", block.name));
        for pair in &agreement.pairs {
            assert!(
                !pair.pair.bound.terms.is_empty(),
                "`{}`/`{}` declares a bound with no terms; a bound without a derivation is a \
                 number somebody liked",
                block.name,
                pair.pair.analog_node
            );
            for (name, value) in &pair.pair.bound.terms {
                assert!(
                    value.is_finite() && *value >= 0.0,
                    "`{}`/`{}` term `{name}` is {value}",
                    block.name,
                    pair.pair.analog_node
                );
            }
            let bound = pair.pair.bound.total();
            let observed = pair.max_error();
            assert!(
                observed * 2.0 <= bound,
                "`{}`/`{}` observed {observed:.6e} V against a bound of {bound:.6e} V, which is \
                 less than a factor of two of margin — the bound is being met by luck\n{}",
                block.name,
                pair.pair.analog_node,
                pair.pair.bound.derivation()
            );
        }
    }
}

// ===========================================================================
// Integrity of the block list
// ===========================================================================

#[test]
fn every_required_block_is_present_and_says_what_it_covers() {
    let blocks = blocks();
    for (name, covers) in REQUIRED_BLOCKS {
        let block = blocks
            .iter()
            .find(|block| block.name == name)
            .unwrap_or_else(|| panic!("`{name}` is missing; it covers {covers}"));
        assert!(
            !block.models.is_empty() && !block.why_both_are_honest.is_empty(),
            "`{name}` does not say what it models or why both representations are honest"
        );
    }
    assert_eq!(
        blocks.len(),
        REQUIRED_BLOCKS.len(),
        "a block was added without a row in REQUIRED_BLOCKS"
    );
}

/// Both representations must cover the same simulated timespan.
///
/// The digital host's tick is a nanosecond and its sample instants are fixed by
/// the stimulus, so a deck written in milliseconds would be compared against an
/// RNM that had run for microseconds — and the performance leg would then be
/// dividing two unrelated numbers.
#[test]
fn both_representations_cover_the_same_timespan_on_the_same_grid() {
    for block in blocks() {
        let samples = block.samples();
        assert!(samples >= 8, "`{}` has only {samples} samples", block.name);

        let times = block.sample_times();
        assert_eq!(times.len(), samples);
        for (step, time) in times.iter().enumerate() {
            let expected = (step as u64 * rnm::SAMPLE_PERIOD_NS + rnm::SETTLE_NS) as f64 * 1e-9;
            assert!(
                (time - expected).abs() <= 1e-18,
                "`{}` sample {step} is at {time} s, not the digital host's {expected} s",
                block.name
            );
            assert!(
                *time < block.timespan(),
                "`{}` sample {step} at {time} s falls outside the transient's {} s",
                block.name,
                block.timespan()
            );
        }
    }
}

/// A block whose analog side is only ever at one value would agree with almost
/// any RNM, so every pair has to move.
#[test]
fn every_compared_signal_moves_over_its_run() {
    for block in blocks() {
        let agreement = rnm::compare(&block)
            .unwrap_or_else(|error| panic!("`{}` could not be compared: {error}", block.name));
        for pair in &agreement.pairs {
            let analog: Vec<f64> = pair.points.iter().map(|point| point.analog).collect();
            let span = analog.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                - analog.iter().copied().fold(f64::INFINITY, f64::min);
            assert!(
                span > 10.0 * pair.pair.bound.total(),
                "`{}`/`{}` moves only {span:.6e} V over its run, which is not enough above its \
                 {:.6e} V bound for agreement to mean anything",
                block.name,
                pair.pair.analog_node,
                pair.pair.bound.total()
            );
        }
    }
}

/// The one block with memory must actually use it.
///
/// `schmitt_hysteresis` is the only block here whose output depends on where its
/// input has been. If its stimulus stopped visiting the region between the two
/// thresholds with both histories, the block would degenerate into a plain
/// comparator and the suite would lose its only sequential coverage without
/// anything failing.
#[test]
fn the_hysteresis_block_reaches_the_same_input_with_both_output_states() {
    let block = rnm::schmitt_hysteresis();
    let agreement =
        rnm::compare(&block).unwrap_or_else(|error| panic!("schmitt must compare: {error}"));
    let pair = &agreement.pairs[0];

    let mut low = Vec::new();
    let mut high = Vec::new();
    for (point, vector) in pair.points.iter().zip(&block.stimulus.vectors) {
        let input: f64 = vector[0].parse().expect("a real column");
        if point.rnm > 1.0 { &mut high } else { &mut low }.push(input);
    }

    let overlap = low
        .iter()
        .any(|quiet| high.iter().any(|loud| (quiet - loud).abs() < 1e-9));
    assert!(
        overlap || {
            // Equal inputs are not required; overlapping *ranges* are what
            // memory means. The input must reach, with the output high, a level
            // it also reached with the output low.
            let low_max = low.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let high_min = high.iter().copied().fold(f64::INFINITY, f64::min);
            high_min < low_max
        },
        "the Schmitt stimulus never revisits an input level with the other output state, so \
         nothing here depends on history: low at {low:?}, high at {high:?}"
    );
}
