//! PSpice `U<name> STIM(...)` digital stimulus, end to end.
//!
//! Each test is a deck. The digital assertions read the event transition
//! stream — exact times and exact ngspice state tokens — rather than sampled
//! waveforms, because a stimulus source has no numerics to be approximate
//! about: it either produced the transition at the declared instant or it did
//! not. Only the mixed-signal deck, where the stimulus has crossed a DAC bridge
//! into an RC network, compares analog magnitudes.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

fn run(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(deck).unwrap_or_else(|err| panic!("deck parses: {err}"));
    Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves")
}

/// The event transition stream for one digital net, as `(time, token)` pairs.
fn digital_tokens(result: &TransientResult, node: &str) -> Vec<(f64, String)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("digital trace {node} missing from {:?}", result.node_names))
        .iter()
        .map(|point| (point.time, point.value.to_ngspice_token()))
        .collect()
}

/// Transition times and tokens with the leading unknown-state entry dropped.
///
/// A digital net is undetermined until the stimulus first drives it, exactly as
/// `d_source` leaves its outputs before the first row. That entry is real, so
/// the tests that are about the driven waveform skip it explicitly rather than
/// pretending it does not exist; the one test that is about it asserts it.
fn driven_tokens(result: &TransientResult, node: &str) -> Vec<(f64, String)> {
    digital_tokens(result, node)
        .into_iter()
        .skip_while(|(_, token)| token == "Uu")
        .collect()
}

fn assert_transitions(actual: &[(f64, String)], expected: &[(f64, &str)]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "transition count differs\n  actual:   {actual:?}\n  expected: {expected:?}"
    );
    for (index, ((time, token), (expected_time, expected_token))) in
        actual.iter().zip(expected).enumerate()
    {
        assert!(
            (time - expected_time).abs() <= 1.0e-15,
            "transition {index} at {time:e} should be at {expected_time:e}\n  actual: {actual:?}"
        );
        assert_eq!(
            token, expected_token,
            "transition {index} token differs\n  actual: {actual:?}"
        );
    }
}

fn node_voltage(result: &SimulationResult, node: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    result.node_voltages[index]
}

fn analog_at(result: &TransientResult, node: &str, target: f64) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    let values = &result.voltages[index];
    let (best, _) = result
        .time
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| {
            (*left - target)
                .abs()
                .partial_cmp(&(*right - target).abs())
                .expect("finite transient times")
        })
        .expect("transient produced samples");
    values[best]
}

//=============================================================================
// Waveforms
//=============================================================================

#[test]
fn single_bit_stimulus_drives_its_declared_transitions() {
    let result = run(
        "\
* PSpice STIM single-bit waveform
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM
+ 0s 0
+ 10ns 1
+ 20ns 0
+ 30ns 1
.model IO_STM UIO (DRVH=50 DRVL=50)
.end
",
        4.0e-8,
        1.0e-10,
    );

    assert_transitions(
        &driven_tokens(&result, "OUT"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (2.0e-8, "0s"), (3.0e-8, "1s")],
    );
}

#[test]
fn stimulus_leaves_its_bus_undetermined_before_the_first_command() {
    let result = run(
        "\
* PSpice STIM first command after time zero
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM
+ 10ns 1
.end
",
        2.0e-8,
        1.0e-10,
    );

    let trace = digital_tokens(&result, "OUT");
    assert_eq!(
        trace.first().map(|(time, token)| (*time, token.as_str())),
        Some((0.0, "Uu")),
        "an undriven stimulus bus is undetermined, got {trace:?}"
    );
    assert_transitions(&driven_tokens(&result, "OUT"), &[(1.0e-8, "1s")]);
}

#[test]
fn incremental_times_accumulate_from_the_previous_command() {
    let result = run(
        "\
* PSpice STIM incremental times
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM
+ 0s 0
+ +10ns 1
+ +5ns 0
+ +5ns 1
.end
",
        3.0e-8,
        1.0e-10,
    );

    assert_transitions(
        &driven_tokens(&result, "OUT"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (1.5e-8, "0s"), (2.0e-8, "1s")],
    );
}

//=============================================================================
// Radices
//=============================================================================

#[test]
fn binary_format_drives_a_multi_bit_bus_most_significant_node_first() {
    let result = run(
        "\
* PSpice STIM binary bus
U1 STIM(4,1111) $G_DPWR $G_DGND B3 B2 B1 B0 IO_STM
+ 0s 0000
+ 10ns 1010
+ 20ns 0101
.end
",
        3.0e-8,
        1.0e-10,
    );

    assert_transitions(
        &driven_tokens(&result, "B3"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (2.0e-8, "0s")],
    );
    assert_transitions(
        &driven_tokens(&result, "B2"),
        &[(0.0, "0s"), (2.0e-8, "1s")],
    );
    assert_transitions(
        &driven_tokens(&result, "B1"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (2.0e-8, "0s")],
    );
    assert_transitions(
        &driven_tokens(&result, "B0"),
        &[(0.0, "0s"), (2.0e-8, "1s")],
    );
}

#[test]
fn octal_format_expands_each_character_into_three_signals() {
    let result = run(
        "\
* PSpice STIM octal bus
U1 STIM(6,33) $G_DPWR $G_DGND B5 B4 B3 B2 B1 B0 IO_STM
+ 0s 00
+ 10ns 52
.end
",
        2.0e-8,
        1.0e-10,
    );

    // Octal 5 is 101 on B5..B3; octal 2 is 010 on B2..B0.
    for (node, expected) in [
        ("B5", "1s"),
        ("B4", "0s"),
        ("B3", "1s"),
        ("B2", "0s"),
        ("B1", "1s"),
        ("B0", "0s"),
    ] {
        let trace = driven_tokens(&result, node);
        let settled = trace
            .last()
            .map(|(_, token)| token.clone())
            .unwrap_or_default();
        assert_eq!(settled, expected, "{node} settled wrong in {trace:?}");
    }
}

#[test]
fn hexadecimal_format_expands_each_character_into_four_signals() {
    let result = run(
        "\
* PSpice STIM hexadecimal bus
U1 STIM(8,44) $G_DPWR $G_DGND B7 B6 B5 B4 B3 B2 B1 B0 IO_STM
+ 0s 00
+ 10ns A5
.end
",
        2.0e-8,
        1.0e-10,
    );

    // 0xA is 1010 on B7..B4; 0x5 is 0101 on B3..B0.
    for (node, expected) in [
        ("B7", "1s"),
        ("B6", "0s"),
        ("B5", "1s"),
        ("B4", "0s"),
        ("B3", "0s"),
        ("B2", "1s"),
        ("B1", "0s"),
        ("B0", "1s"),
    ] {
        let trace = driven_tokens(&result, node);
        let settled = trace
            .last()
            .map(|(_, token)| token.clone())
            .unwrap_or_default();
        assert_eq!(settled, expected, "{node} settled wrong in {trace:?}");
    }
}

#[test]
fn mixed_radix_format_splits_the_bus_at_its_declared_boundaries() {
    let result = run(
        "\
* PSpice STIM one binary signal ahead of an octal group
U1 STIM(4,13) $G_DPWR $G_DGND MSB B2 B1 B0 IO_STM
+ 0s 00
+ 10ns 16
.end
",
        2.0e-8,
        1.0e-10,
    );

    // Binary 1 on MSB; octal 6 is 110 on B2..B0.
    for (node, expected) in [("MSB", "1s"), ("B2", "1s"), ("B1", "1s"), ("B0", "0s")] {
        let trace = driven_tokens(&result, node);
        let settled = trace
            .last()
            .map(|(_, token)| token.clone())
            .unwrap_or_default();
        assert_eq!(settled, expected, "{node} settled wrong in {trace:?}");
    }
}

//=============================================================================
// X and Z
//=============================================================================

#[test]
fn unknown_and_high_impedance_values_reach_the_event_stream() {
    let result = run(
        "\
* PSpice STIM unknown and high-impedance states
U1 STIM(2,11) $G_DPWR $G_DGND A B IO_STM
+ 0s 01
+ 10ns XZ
+ 20ns 10
.end
",
        3.0e-8,
        1.0e-10,
    );

    assert_transitions(
        &driven_tokens(&result, "A"),
        &[(0.0, "0s"), (1.0e-8, "Us"), (2.0e-8, "1s")],
    );
    // A high-impedance line has no level to report, so ngspice spells it `Uz`:
    // unknown level, high-Z strength. That is what distinguishes it from the
    // strongly-driven unknown `Us` on A.
    assert_transitions(
        &driven_tokens(&result, "B"),
        &[(0.0, "1s"), (1.0e-8, "Uz"), (2.0e-8, "0s")],
    );
}

#[test]
fn a_hexadecimal_x_marks_every_signal_of_its_group_unknown() {
    let result = run(
        "\
* PSpice STIM hexadecimal unknown digit
U1 STIM(8,44) $G_DPWR $G_DGND B7 B6 B5 B4 B3 B2 B1 B0 IO_STM
+ 0s 00
+ 10ns X0
.end
",
        2.0e-8,
        1.0e-10,
    );

    for node in ["B7", "B6", "B5", "B4"] {
        let trace = driven_tokens(&result, node);
        assert_eq!(
            trace.last().map(|(_, token)| token.as_str()),
            Some("Us"),
            "{node} should be unknown in {trace:?}"
        );
    }
    for node in ["B3", "B2", "B1", "B0"] {
        let trace = driven_tokens(&result, node);
        assert_eq!(
            trace.last().map(|(_, token)| token.as_str()),
            Some("0s"),
            "{node} should still be zero in {trace:?}"
        );
    }
}

//=============================================================================
// Loops
//=============================================================================

#[test]
fn a_counted_goto_repeats_its_body_and_then_stops() {
    let result = run(
        "\
* PSpice STIM counted GOTO
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM
+ 0s 0
+ LABEL=CYCLE
+ +10ns 1
+ +10ns 0
+ +0s GOTO CYCLE 2 TIMES
.end
",
        1.0e-7,
        1.0e-9,
    );

    // The body runs once, then the GOTO takes its two jumps: three passes.
    assert_transitions(
        &driven_tokens(&result, "OUT"),
        &[
            (0.0, "0s"),
            (1.0e-8, "1s"),
            (2.0e-8, "0s"),
            (3.0e-8, "1s"),
            (4.0e-8, "0s"),
            (5.0e-8, "1s"),
            (6.0e-8, "0s"),
        ],
    );
}

#[test]
fn a_forever_goto_runs_to_the_transient_stop_time() {
    let result = run(
        "\
* PSpice STIM forever GOTO
U1 STIM(1,1) $G_DPWR $G_DGND CLK IO_STM
+ 0s 0
+ LABEL=TICK
+ +10ns 1
+ +10ns 0
+ +0s GOTO TICK -1 TIMES
.end
",
        5.5e-8,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "CLK"),
        &[
            (0.0, "0s"),
            (1.0e-8, "1s"),
            (2.0e-8, "0s"),
            (3.0e-8, "1s"),
            (4.0e-8, "0s"),
            (5.0e-8, "1s"),
        ],
    );
}

#[test]
fn a_repeat_block_iterates_its_body() {
    let result = run(
        "\
* PSpice STIM REPEAT block
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM
+ 0s 0
+ REPEAT 2 TIMES
+ +10ns 1
+ +10ns 0
+ ENDREPEAT
.end
",
        1.0e-7,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "OUT"),
        &[
            (0.0, "0s"),
            (1.0e-8, "1s"),
            (2.0e-8, "0s"),
            (3.0e-8, "1s"),
            (4.0e-8, "0s"),
        ],
    );
}

#[test]
fn a_forever_repeat_block_runs_to_the_transient_stop_time() {
    let result = run(
        "\
* PSpice STIM REPEAT FOREVER block
U1 STIM(1,1) $G_DPWR $G_DGND CLK IO_STM
+ 0s 0
+ REPEAT FOREVER
+ +10ns 1
+ +10ns 0
+ ENDREPEAT
.end
",
        4.5e-8,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "CLK"),
        &[
            (0.0, "0s"),
            (1.0e-8, "1s"),
            (2.0e-8, "0s"),
            (3.0e-8, "1s"),
            (4.0e-8, "0s"),
        ],
    );
}

//=============================================================================
// Counting
//=============================================================================

#[test]
fn incr_and_decr_step_the_bus_as_an_unsigned_integer() {
    let result = run(
        "\
* PSpice STIM INCR/DECR counter
U1 STIM(2,11) $G_DPWR $G_DGND B1 B0 IO_STM
+ 0s 00
+ +10ns INCR BY 01
+ +10ns INCR BY 01
+ +10ns INCR BY 01
+ +10ns DECR BY 11
.end
",
        6.0e-8,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "B1"),
        &[(0.0, "0s"), (2.0e-8, "1s"), (4.0e-8, "0s")],
    );
    assert_transitions(
        &driven_tokens(&result, "B0"),
        &[
            (0.0, "0s"),
            (1.0e-8, "1s"),
            (2.0e-8, "0s"),
            (3.0e-8, "1s"),
            (4.0e-8, "0s"),
        ],
    );
}

#[test]
fn a_forever_goto_around_incr_makes_a_free_running_counter() {
    let result = run(
        "\
* PSpice STIM free-running hexadecimal counter
U1 STIM(4,4) $G_DPWR $G_DGND B3 B2 B1 B0 IO_STM
+ 0s 0
+ LABEL=COUNT
+ +10ns INCR BY 1
+ +0s GOTO COUNT -1 TIMES
.end
",
        3.5e-8,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "B0"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (2.0e-8, "0s"), (3.0e-8, "1s")],
    );
    assert_transitions(
        &driven_tokens(&result, "B1"),
        &[(0.0, "0s"), (2.0e-8, "1s")],
    );
}

//=============================================================================
// TIMESTEP
//=============================================================================

#[test]
fn timestep_scales_clock_suffixed_command_times() {
    let result = run(
        "\
* PSpice STIM clock-relative times
U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM TIMESTEP=5ns
+ 0s 0
+ +2c 1
+ +1c 0
+ 8c 1
.end
",
        5.0e-8,
        1.0e-9,
    );

    assert_transitions(
        &driven_tokens(&result, "OUT"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (1.5e-8, "0s"), (4.0e-8, "1s")],
    );
}

//=============================================================================
// Interaction with the rest of the U-device front end
//=============================================================================

#[test]
fn stimulus_feeds_a_timed_u_gate_and_the_gate_supplies_the_delay() {
    let result = run(
        "\
* PSpice STIM driving a UGATE-timed inverter
U1 STIM(1,1) $G_DPWR $G_DGND D IO_STM
+ 0s 0
+ 10ns 1
+ 20ns 0
U2 INV $G_DPWR $G_DGND D Q DLY IO_LEVEL=0
.model DLY UGATE (TPLHTY=3ns TPHLTY=3ns)
.model IO_STM UIO (DRVH=50 DRVL=50)
.end
",
        4.0e-8,
        1.0e-10,
    );

    // The stimulus lands on its declared edges: the I/O model does not delay it.
    assert_transitions(
        &driven_tokens(&result, "D"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (2.0e-8, "0s")],
    );
    // The inverter's UGATE timing model supplies the propagation delay: each
    // stimulus edge reappears on Q three nanoseconds later. The entry at t=0 is
    // the gate's own power-up output, which every d_inverter emits undelayed.
    assert_transitions(
        &driven_tokens(&result, "Q"),
        &[(0.0, "1s"), (1.3e-8, "0s"), (2.3e-8, "1s")],
    );
}

#[test]
fn stimulus_drives_an_analog_rc_through_a_dac_bridge() {
    let result = run(
        "\
* PSpice STIM through a DAC bridge into an RC
U1 STIM(1,1) $G_DPWR $G_DGND D IO_STM
+ 0s 0
+ 10ns 1
+ 60ns 0
a_dac [D] [DRV] DAC
.model DAC dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
R1 DRV OUT 1k
C1 OUT 0 10p
.end
",
        1.1e-7,
        1.0e-10,
    );

    assert_transitions(
        &driven_tokens(&result, "D"),
        &[(0.0, "0s"), (1.0e-8, "1s"), (6.0e-8, "0s")],
    );

    // One RC is 10 ns, so the analog node is a first-order step response to the
    // stimulus edges. Comparing against the closed form — not against a
    // hand-picked band — is what makes this deck prove delivery: an edge that
    // arrived at the wrong instant would miss the exponential.
    let tau = 1.0e3 * 10.0e-12;
    let charging = |elapsed: f64| 5.0 * (1.0 - (-elapsed / tau).exp());
    let discharging = |elapsed: f64| 5.0 * (-elapsed / tau).exp();

    for (time, expected) in [
        (9.0e-9, 0.0),
        (2.0e-8, charging(1.0e-8)),
        (3.5e-8, charging(2.5e-8)),
        (5.5e-8, charging(4.5e-8)),
        (7.0e-8, discharging(1.0e-8)),
        (1.05e-7, discharging(4.5e-8)),
    ] {
        let actual = analog_at(&result, "OUT", time);
        assert!(
            (actual - expected).abs() < 0.05,
            "OUT at {time:e} should be {expected:.4} V from the RC step response, got {actual:.4}"
        );
    }
}

//=============================================================================
// Diagnostics: every refusal is typed, actionable, and never a panic
//=============================================================================

fn parse_error(deck: &str) -> String {
    match Netlist::parse(deck) {
        Ok(_) => panic!("deck should not parse:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

fn stim_deck(header: &str, commands: &str) -> String {
    format!("* PSpice STIM diagnostic\nU1 {header}\n{commands}.end\n")
}

#[test]
fn a_format_that_does_not_sum_to_the_width_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(4,11) $G_DPWR $G_DGND A B C D IO_STM",
        "+ 0s 00\n",
    ));
    assert!(
        message.contains("declares width 4")
            && message.contains("describes 2 signal(s)")
            && message.contains("must sum to the width"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn an_undocumented_format_digit_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(2,2) $G_DPWR $G_DGND A B IO_STM",
        "+ 0s 0\n",
    ));
    assert!(
        message.contains("format digit '2'")
            && message.contains("1 (binary), 3 (octal), or 4 (hexadecimal)"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_value_of_the_wrong_length_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(4,1111) $G_DPWR $G_DGND A B C D IO_STM",
        "+ 0s 101\n",
    ));
    assert!(
        message.contains("value '101' with 3 character(s)") && message.contains("exactly 4"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_digit_outside_its_radix_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(3,3) $G_DPWR $G_DGND A B C IO_STM",
        "+ 0s 9\n",
    ));
    assert!(
        message.contains("character '9' is not a 3-bit digit"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn the_rising_and_falling_transition_states_are_refused_with_their_reason() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ 0s 0\n+ 10ns R\n",
    ));
    assert!(
        message.contains("transition state 'R'")
            && message.contains("0, 1, X and Z")
            && message.contains("settled level"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_goto_to_an_undefined_label_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ 0s 0\n+ +10ns GOTO NOWHERE 1 TIMES\n",
    ));
    assert!(
        message.contains("label 'NOWHERE' which is never defined")
            && message.contains("LABEL=NOWHERE"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_goto_without_a_repeat_count_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ LABEL=TOP\n+ 0s 0\n+ +10ns GOTO TOP\n",
    ));
    assert!(
        message.contains("no repeat count"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn the_conditional_goto_form_is_refused_by_name() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ LABEL=TOP\n+ 0s 0\n+ +10ns GOTO UNTIL GT 5\n",
    ));
    assert!(
        message.contains("'GOTO ... UNTIL'") && message.contains("does not implement"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn an_unbalanced_repeat_block_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ REPEAT 2 TIMES\n+ +10ns 1\n",
    ));
    assert!(
        message.contains("REPEAT block(s) with no matching ENDREPEAT"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_clock_relative_time_without_a_timestep_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ 0s 0\n+ +2c 1\n",
    ));
    assert!(
        message.contains("clock-relative time '+2c'") && message.contains("TIMESTEP="),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_stimulus_reference_to_an_absent_dot_stimulus_card_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM STIMULUS=CLOCK1",
        "",
    ));
    assert!(
        message.contains("STIMULUS=") && message.contains(".STIMULUS card"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_stimulus_with_no_commands_is_refused() {
    let message = parse_error(&stim_deck("STIM(1,1) $G_DPWR $G_DGND A IO_STM", ""));
    assert!(
        message.contains("no stimulus commands"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_bus_narrower_than_its_declared_width_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(4,1111) $G_DPWR $G_DGND A B",
        "+ 0s 0000\n",
    ));
    assert!(
        message.contains("declares 4 output node(s)") && message.contains("I/O model name"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_time_with_no_command_after_it_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ 0s 0\n+ 10ns\n",
    ));
    assert!(
        message.contains("no value, GOTO, INCR or DECR"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_step_value_carrying_an_unknown_digit_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(2,11) $G_DPWR $G_DGND A B IO_STM",
        "+ 0s 00\n+ +10ns INCR BY 0X\n",
    ));
    assert!(
        message.contains("no X or Z digits"),
        "unexpected diagnostic: {message}"
    );
}

#[test]
fn a_negative_time_increment_is_refused() {
    let message = parse_error(&stim_deck(
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM",
        "+ 0s 0\n+ +-10ns 1\n",
    ));
    assert!(
        message.contains("negative time increment"),
        "unexpected diagnostic: {message}"
    );
}

/// Every shape of malformed STIM card resolves to `Err`, never a panic.
///
/// The parser indexes a field list whose length it derives from the declared
/// width, so truncation is the failure mode to watch: each case here stops the
/// card somewhere a slice bound could have been taken on trust.
#[test]
fn no_malformed_stim_card_panics() {
    let cards = [
        "STIM",
        "STIM(",
        "STIM()",
        "STIM(1)",
        "STIM(1,)",
        "STIM(0,1)",
        "STIM(,1)",
        "STIM(1,1,1)",
        "STIM(x,1)",
        "STIM(1,1) $G_DPWR",
        "STIM(1,1) $G_DPWR $G_DGND",
        "STIM(1,1) $G_DPWR $G_DGND A",
        "STIM(1,1) $G_DPWR $G_DGND $D_NC IO_STM",
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM TIMESTEP=",
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM TIMESTEP=0",
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM TIMESTEP=-1n",
        "STIM(1,1) $G_DPWR $G_DGND A IO_STM BOGUS=1",
        "STIM(1,1) $G_DPWR $G_DGND A=B IO_STM",
        "STIM(2,11) $G_DPWR $G_DGND A IO_LEVEL=1 IO_STM",
        "STIM(65,1111111111111111111111111111111111111111111111111111111111111111\
         1) $G_DPWR $G_DGND A IO_STM",
    ];
    let bodies = [
        "",
        "+ 0s\n",
        "+ 0s 0\n",
        "+ 0s 0\n+ REPEAT\n",
        "+ 0s 0\n+ REPEAT 0 TIMES\n+ ENDREPEAT\n",
        "+ ENDREPEAT\n",
        "+ LABEL=\n",
        "+ LABEL=A\n+ LABEL=A\n+ 0s 0\n",
        "+ 0s 0\n+ GOTO\n",
        "+ 0s 0\n+ +1n GOTO A\n",
        "+ 0s 0\n+ +1n INCR\n",
        "+ 0s 0\n+ +1n INCR BY\n",
        "+ 0s 0\n+ +1n DECR BY 2\n",
        "+ 0s 0\n+ 1n R\n",
        "+ 0s 0\n+ 1c 1\n",
        "+ 0s 0\n+ {UNDEFINED} 1\n",
    ];
    for card in cards {
        for body in bodies {
            let deck = stim_deck(card, body);
            // Either outcome is acceptable; unwinding is not.
            let _ = Netlist::parse(&deck);
        }
    }
}

//=============================================================================
// Analyses that are not `.tran`
//=============================================================================

#[test]
fn a_stimulus_deck_solves_its_operating_point() {
    let deck = "\
* PSpice STIM at the operating point
U1 STIM(1,1) $G_DPWR $G_DGND D IO_STM
+ 0s 1
+ 10ns 0
a_dac [D] [DRV] DAC
.model DAC dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
R1 DRV 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    // Outside `.tran` only the commands at or before time zero apply, so the
    // stimulus holds its t=0 value and the bridge drives the rail.
    assert!(
        (node_voltage(&result, "DRV") - 5.0).abs() < 1.0e-6,
        "DRV should sit at the high rail, got {}",
        node_voltage(&result, "DRV")
    );
}

#[test]
fn a_stimulus_deck_solves_a_dc_sweep() {
    let deck = "\
* PSpice STIM across a DC sweep
U1 STIM(1,1) $G_DPWR $G_DGND D IO_STM
+ 0s 0
+ 10ns 1
a_dac [D] [DRV] DAC
.model DAC dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
R1 DRV OUT 1k
R2 OUT 0 1k
V1 BIAS 0 1
R3 BIAS OUT 1meg
.dc V1 0 2 0.5
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "V1", 0.0, 2.0, 0.5)
        .expect("DC sweep solves");
    assert_eq!(points.len(), 5, "five sweep points");
    for (sweep, result) in &points {
        // The stimulus holds its t=0 low value at every sweep point, so the
        // divider output stays pinned near ground however V1 moves.
        assert!(
            node_voltage(result, "OUT").abs() < 0.01,
            "OUT drifted at sweep value {sweep}: {}",
            node_voltage(result, "OUT")
        );
    }
}
