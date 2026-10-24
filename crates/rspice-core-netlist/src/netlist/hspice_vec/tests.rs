//! The vector file format against the HSPICE manuals' own examples. Each
//! fixture under `testdata/` cites the example it holds.

use super::*;

fn no_params(_: &str) -> Option<Value> {
    None
}

fn file(text: &str) -> VectorFile {
    parse(text, &no_params).unwrap_or_else(|error| panic!("{error:?}"))
}

fn refusal(text: &str) -> VectorFileError {
    match parse(text, &no_params).and_then(|file| file.check_realizable().map(|()| file)) {
        Ok(_) => panic!("refused nothing"),
        Err(error) => error,
    }
}

fn bit<'f>(file: &'f VectorFile, name: &str) -> &'f VectorBit {
    file.bits
        .iter()
        .find(|bit| bit.name == name)
        .unwrap_or_else(|| panic!("no bit {name}"))
}

fn ns(value: Value) -> Value {
    value / 1e9
}

const DELAYS: &str = include_str!("testdata/cmdref_delays.vec");
const RADIX: &str = include_str!("testdata/cmdref_radix.vec");
const PERIOD: &str = include_str!("testdata/cmdref_period.vec");
const TSKIP: &str = include_str!("testdata/cmdref_tskip.vec");
const SIGNAL_VALUES: &str = include_str!("testdata/cmdref_signal_values.vec");
const VREF: &str = include_str!("testdata/cmdref_vref.vec");
const ENABLE: &str = include_str!("testdata/cmdref_enable.vec");
const UG_EXAMPLE: &str = include_str!("testdata/ug_example.vec");
const UG_EXPECTED: &str = include_str!("testdata/ug_expected_output.vec");
const UG_PERIODIC: &str = include_str!("testdata/ug_periodic.vec");
const UG_CBC: &str = include_str!("testdata/ug_cbc.vec");

#[test]
fn radix_declares_one_vector_per_digit_and_vname_names_their_bits() {
    let file = file(RADIX);
    let names: Vec<&str> = file.bits.iter().map(|bit| bit.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "A", "B", "C[3]", "C[2]", "C[1]", "C[0]", "I9", "I[8]", "I[7]", "I[6]", "I[5]", "I[4]",
            "I[3]", "I[2]", "I[1]", "I[0]", "O7", "O6", "O5", "O4", "O3", "O2", "O1", "O0"
        ]
    );
    let columns: Vec<usize> = file.bits.iter().map(|bit| bit.column).collect();
    assert_eq!(
        columns,
        [
            1, 2, 3, 3, 3, 3, 4, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ]
    );
    assert!(
        file.bits[..16]
            .iter()
            .all(|bit| bit.direction == Direction::Input)
    );
    assert!(
        file.bits[16..]
            .iter()
            .all(|bit| bit.direction == Direction::Output)
    );
}

#[test]
fn delays_follow_the_last_tdelay_idelay_or_odelay_that_selects_a_bit() {
    let file = file(DELAYS);
    for name in ["V1", "V8", "V15"] {
        assert_eq!(bit(&file, name).input_delay, ns(1.0), "{name}");
        assert_eq!(bit(&file, name).output_delay, ns(1.0), "{name}");
    }
    for name in ["V2", "VX[3]", "VX[0]"] {
        assert_eq!(bit(&file, name).input_delay, ns(-1.2), "{name}");
        assert_eq!(bit(&file, name).output_delay, ns(-1.2), "{name}");
    }
    for name in ["V4", "V5[1]", "V5[0]", "V6[0]", "V6[2]"] {
        assert_eq!(bit(&file, name).input_delay, ns(1.5), "{name}");
    }
    for name in ["V7[0]", "V7[3]"] {
        assert_eq!(bit(&file, name).direction, Direction::Bidirectional);
        assert_eq!(bit(&file, name).input_delay, ns(2.0), "{name}");
        assert_eq!(bit(&file, name).output_delay, ns(3.0), "{name}");
    }
    // V7 is bidirectional, which RSpice refuses at its IO line.
    let error = file.check_realizable().unwrap_err();
    assert_eq!(error.line, 7);
    assert!(
        error.message.contains("column 7 (V7[0]) is bidirectional"),
        "{}",
        error.message
    );
}

#[test]
fn period_places_rows_without_times_and_tskip_ignores_the_times_given() {
    for text in [PERIOD, TSKIP] {
        let file = file(text);
        let times: Vec<Value> = file.rows.iter().map(|row| row.time).collect();
        assert_eq!(times, [0.0, ns(10.0), ns(20.0)]);
        assert_eq!(file.rows[2].states[0], BitState::One);
        assert_eq!(file.rows[2].states[2], BitState::One);
        assert_eq!(file.rows[2].states[7], BitState::One);
        assert_eq!(file.rows[2].states[1], BitState::Zero);
    }
    let periodic = file(UG_PERIODIC);
    assert_eq!(periodic.rows.len(), 3);
    assert_eq!(periodic.rows[1].time, ns(10.0));
    assert_eq!(bit(&periodic, "H").column, 8);
}

#[test]
fn waveform_values_follow_each_manual_example() {
    let file = file(SIGNAL_VALUES);
    let at = |index: usize| &file.bits[index];
    // Bits 0..3 are vectors 1-3; 3..13 vectors 4-7; 13..21 the last eight.
    let first_seven = 0..13;
    let last_eight = 13..21;
    assert_eq!(file.bits.len(), 21);
    assert_eq!(at(0).rise, ns(0.3));
    assert_eq!(at(0).fall, ns(0.5));
    for index in 1..13 {
        assert_eq!(at(index).rise, ns(0.5), "{index}");
        assert_eq!(at(index).fall, ns(0.3), "{index}");
    }
    for index in 13..17 {
        assert_eq!(at(index).rise, ns(0.8), "{index}");
        assert_eq!(at(index).fall, ns(0.9), "{index}");
    }
    for index in 17..21 {
        assert_eq!(at(index).rise, ns(0.3), "{index}");
        assert_eq!(at(index).fall, ns(0.5), "{index}");
    }
    for index in first_seven.clone() {
        assert_eq!((at(index).high, at(index).low), (5.0, 0.0), "{index}");
        assert_eq!(at(index).voh, Some(4.5), "{index}");
        assert_eq!(at(index).vth, 2.5, "{index}");
    }
    for index in last_eight {
        assert_eq!((at(index).high, at(index).low), (3.5, 0.5), "{index}");
        assert_eq!(at(index).voh, Some(3.5), "{index}");
        assert_eq!(at(index).vol, Some(0.0), "{index}");
        assert_eq!(at(index).vth, 1.75, "{index}");
        assert_eq!(at(index).resistance, 15.1, "{index}");
        assert_eq!(at(index).tristate, 15.1e6, "{index}");
    }
    for index in 0..3 {
        assert_eq!(at(index).vol, Some(0.5), "{index}");
        assert_eq!(at(index).resistance, 150.0, "{index}");
        assert_eq!(at(index).tristate, 150e6, "{index}");
    }
    for index in 3..13 {
        assert_eq!(at(index).vol, Some(0.2), "{index}");
        assert_eq!(at(index).resistance, 50.5, "{index}");
        assert_eq!(at(index).tristate, 50.5e6, "{index}");
    }
    // Every vector is an input (no IO statement), and a nonzero OUT is
    // refused at the statement that set it.
    let error = file.check_realizable().unwrap_err();
    assert_eq!(error.line, 36);
    assert!(error.message.contains("150 ohm"), "{}", error.message);
}

#[test]
fn vref_names_the_node_an_input_is_referred_to() {
    let file = file(VREF);
    for name in ["v1", "v4", "v5[1]", "v6[0]", "v7[3]"] {
        assert_eq!(bit(&file, name).reference, "0", "{name}");
    }
    for name in ["v8", "v9", "v10"] {
        assert_eq!(bit(&file, name).reference, "vss", "{name}");
    }
}

#[test]
fn the_user_guide_example_reads_as_the_manual_states_and_is_refused_for_its_bidirectional_bus() {
    let file = file(UG_EXAMPLE);
    let names: Vec<&str> = file.bits.iter().map(|bit| bit.name.as_str()).collect();
    assert_eq!(names[..3], ["v1", "va[1]", "va[0]"]);
    assert_eq!(names[3], "vb12");
    assert_eq!(names[14], "vb1");
    assert_eq!(bit(&file, "v1").input_delay, ns(1.0));
    assert_eq!(bit(&file, "vb7").input_delay, ns(1.0));
    assert_eq!(bit(&file, "va[1]").output_delay, ns(1.5));
    assert_eq!(bit(&file, "va[0]").output_delay, ns(1.5));
    assert!(
        file.bits
            .iter()
            .all(|bit| bit.rise == ns(1.2) && bit.fall == ns(1.2))
    );
    assert_eq!(bit(&file, "v1").high, 3.3);
    assert_eq!(bit(&file, "vb3").high, 5.0);
    assert_eq!(bit(&file, "va[0]").vth, 1.75);
    assert_eq!(bit(&file, "vb12").vth, 1.75);
    assert_eq!(bit(&file, "v1").vth, DEFAULT_VTH);
    let times: Vec<Value> = file.rows.iter().map(|row| row.time).collect();
    assert_eq!(times, [ns(10.0), ns(20.0), ns(30.0)]);
    // 30.0 1 0 888: vb12, vb8 and vb4 are 1.
    let last = &file.rows[2].states;
    assert_eq!(last[3], BitState::One);
    assert_eq!(last[4], BitState::Zero);
    assert_eq!(last[7], BitState::One);
    assert_eq!(file.enables.len(), 2);
    let error = file.check_realizable().unwrap_err();
    assert_eq!(error.line, 11);
    assert!(
        error.message.contains("column 3 (vb12) is bidirectional"),
        "{}",
        error.message
    );
}

#[test]
fn enable_is_parsed_and_its_bidirectional_columns_refused() {
    let error = refusal(ENABLE);
    assert_eq!(error.line, 5);
    assert!(
        error.message.contains("column 2 (x[3])"),
        "{}",
        error.message
    );
}

#[test]
fn context_based_control_is_refused_by_name() {
    let error = parse(UG_CBC, &no_params).unwrap_err();
    assert_eq!(error.line, 7);
    assert!(error.message.starts_with("OPTION CBC"), "{}", error.message);
}

#[test]
fn expected_outputs_become_samples_and_x_is_not_checked() {
    let file = file(UG_EXPECTED);
    let reference = VectorFileReference {
        path: "dir/expected.vec".into(),
        level: AssertLevel::Error,
        origin: NetlistSourceLocation::in_file("deck.cir", 4),
    };
    let checks = file.output_checks(&reference);
    assert_eq!(checks.len(), 4);
    let samples = |index: usize| match &checks[index].check {
        AssertCheck::Sampled { samples, .. } => samples
            .iter()
            .map(|sample| (sample.origin.line, sample.high))
            .collect::<Vec<_>>(),
        other => panic!("{other:?}"),
    };
    // Rows are on lines 9..12: 1001, 1100, 1000, xx00.
    assert_eq!(samples(0), [(9, true), (10, true), (11, true)]);
    assert_eq!(samples(1), [(9, false), (10, true), (11, false)]);
    assert_eq!(
        samples(3),
        [(9, true), (10, false), (11, false), (12, false)]
    );
    assert_eq!(checks[0].name, "expected.vec");
    assert_eq!(checks[0].analyses, [AssertAnalysis::Tran]);
    // No VOH or VOL: both thresholds are the default VTH.
    let AssertCheck::Sampled { samples, .. } = &checks[0].check else {
        unreachable!()
    };
    assert_eq!(samples[0].threshold, DEFAULT_VTH);
    assert_eq!(samples[0].time, ns(11.0));
    assert_eq!(
        samples[0].origin,
        NetlistSourceLocation::in_file("dir/expected.vec", 9)
    );
}

#[test]
fn comments_start_anywhere_and_plus_continues_a_line() {
    // User Guide, "Comment Lines and Line Continuations"; the VNAME line,
    // continued, is added to name the multi-bit vectors.
    let file = file(
        "; This is a comment line\nradix 1 1 4 1234 ; This is a radix line\n\
         vname a b c[3:0] d\n+ e[1:0] f[2:0] ; comment\n+ g[3:0]\n",
    );
    assert_eq!(file.bits.len(), 1 + 1 + 4 + 1 + 2 + 3 + 4);
    assert_eq!(file.bits[15].name, "g0");
}

#[test]
fn bus_names_keep_what_surrounds_the_innermost_range() {
    // Reference Manual, VNAME, Examples 4 and 5; User Guide, "Digital Vector
    // File Example".
    let names = |entry| expand_name(entry, 1).unwrap();
    assert_eq!(names("VA[[0:1]]"), ["VA[0]", "VA[1]"]);
    assert_eq!(names("VB<[4:1]>"), ["VB<4>", "VB<3>", "VB<2>", "VB<1>"]);
    assert_eq!(names("VA[[2:2]]"), ["VA[2]"]);
    assert_eq!(names("vb[12:1]").len(), 12);
    assert_eq!(names("vb[12:1]")[0], "vb12");
    assert_eq!(names("plain"), ["plain"]);
}

#[test]
fn tunit_reads_the_legal_forms_and_refuses_the_wrong_ones() {
    // Reference Manual, TUNIT: "legal ways to write the time values" and
    // "examples of wrong syntax which will result in an error message".
    let unit = |text: &str| time_unit(&text.split_whitespace().collect::<Vec<_>>(), 1);
    assert_eq!(unit("ns").unwrap(), 1e-9);
    assert_eq!(unit("999ns").unwrap(), 999.0 * 1e-9);
    assert_eq!(unit(".99ps").unwrap(), 0.99 * 1e-12);
    assert_eq!(unit(".99e+6ps").unwrap(), 0.99e6 * 1e-12);
    assert_eq!(unit("999 ns").unwrap(), 999.0 * 1e-9);
    assert_eq!(unit(".99 ps").unwrap(), 0.99 * 1e-12);
    for wrong in [".99eps", ".99 e+6ps", ".99 eps", "s", "10 ks"] {
        assert!(unit(wrong).is_err(), "{wrong}");
    }
}

#[test]
fn parameters_stand_for_values_in_time_units_and_volts() {
    // User Guide, "Parameter Usage": `.param myperiod=10` with `tunit ns`
    // and `period myperiod` is a 10 ns period.
    let params = |name: &str| match name {
        "myperiod" => Some(10.0),
        "vhigh" => Some(1.8),
        _ => None,
    };
    let file = parse(
        "radix 1\ntunit ns\nperiod myperiod\nvih vhigh\n1\n0\n",
        &params,
    )
    .unwrap();
    assert_eq!(file.rows[1].time, ns(10.0));
    assert_eq!(file.bits[0].high, 1.8);
    let error = parse("radix 1\nvih missing\n", &params).unwrap_err();
    assert_eq!(error.line, 2);
    assert!(
        error.message.contains("neither a number nor a .PARAM"),
        "{}",
        error.message
    );
}

#[test]
fn refusals_name_their_line() {
    let cases: &[(&str, usize, &str)] = &[
        ("io i\nradix 1\n", 1, "must be its RADIX statement"),
        ("radix 15\n", 1, "RADIX digit '5'"),
        ("radix 1\nradix 1\n", 2, "one RADIX statement"),
        ("radix 1\nmask 1\n", 2, "mask is not a vector file keyword"),
        (
            "radix 1\nchk_window 1 2\n",
            2,
            "chk_window is not a vector file keyword",
        ),
        (
            "radix 11\n0 10\n1 12\n",
            3,
            "'2' in column 2 is not a digit of its 1-bit (binary)",
        ),
        (
            "radix 3\nvname a[2:0]\n0 8\n",
            3,
            "'8' in column 1 is not a digit of its 3-bit (octal)",
        ),
        ("radix 4\nvname a[3:0]\n0 G\n", 3, "'G' in column 1"),
        (
            "radix 11\n0 10\n1 101\n",
            3,
            "gives 3 value(s), but RADIX declares 2",
        ),
        ("radix 1\n0 1\n0 0\n", 3, "rows run in time order"),
        (
            "radix 1\n0 1\ntdelay 1\n",
            3,
            "every statement comes before the first row",
        ),
        (
            "radix 4\nvname a[3:0]\n0 4'b1111\n",
            3,
            "Verilog-sized value",
        ),
        (
            "radix 4\n0 1\n",
            0,
            "no VNAME, and column 1 is a 4-bit vector",
        ),
        ("radix 22\nvname a[2:0] b\n", 2, "ends inside a vector"),
        ("radix 1\nio x\n", 2, "IO 'x'"),
        ("radix 1 1\nio i\n", 2, "IO gives 1 direction(s)"),
        ("radix 1\ntdelay 1 11\n", 2, "the mask '11' has 2 digit(s)"),
        (
            "radix 2\nvname a[1:0]\ntrise 1 4\n",
            3,
            "mask digit '4' of column 1",
        ),
        ("radix 1\ntskip\n0 1\n", 2, "gives no PERIOD"),
        (
            "radix 1\nperiod 1\nperiod 2\n",
            3,
            "already given at line 2",
        ),
        ("+ radix 1\n", 1, "continues no statement"),
    ];
    for (text, line, message) in cases {
        let error = match parse(text, &no_params) {
            Ok(_) => panic!("{text:?} parsed"),
            Err(error) => error,
        };
        assert_eq!(error.line, *line, "{text:?}: {}", error.message);
        assert!(
            error.message.contains(message),
            "{text:?}: {}",
            error.message
        );
    }
}

#[test]
fn an_input_z_and_an_output_h_are_refused_naming_row_and_column() {
    let error = refusal("radix 1 1\nio i o\nvname a b\n0 0 0\n10 Z 1\n");
    assert_eq!(error.line, 5);
    assert!(
        error.message.contains("input a (column 1) is Z"),
        "{}",
        error.message
    );
    let error = refusal("radix 1 1\nio i o\nvname a b\n0 0 H\n");
    assert_eq!(error.line, 4);
    assert!(
        error.message.contains("output b (column 2) expects H"),
        "{}",
        error.message
    );
    let error = refusal("radix 1\nio o\nvoh 1\n0 1\n");
    assert_eq!(error.line, 3);
    assert!(
        error.message.contains("has VOH but no VOL"),
        "{}",
        error.message
    );
}

#[test]
fn an_input_ramps_from_its_row_time_plus_its_delay() {
    // VIH 1.8, VIL 0, TRISE 1 ns, TFALL 2 ns, IDELAY 0.5 ns: a 0 -> 1 row at
    // 10 ns ramps from 10.5 ns to 11.5 ns, and a 1 -> 0 row at 20 ns falls
    // from 20.5 ns to 22.5 ns; the unchanged row at 15 ns adds nothing.
    let file = file(
        "radix 1\nvname a\nvih 1.8\nvil 0\ntrise 1\ntfall 2\nidelay 0.5\nvref gnd2\n\
         0 0\n10 1\n15 1\n20 0\n",
    );
    let sources = file.input_sources("vvec_").unwrap();
    let [source] = sources.as_slice() else {
        panic!("{sources:?}")
    };
    // Spelled as the deck's lexer spells names, so it drives the deck's node.
    assert_eq!(source.name, "VVEC_A");
    assert_eq!(source.nodes, ["A", "GND2"]);
    let ElementKind::VoltageSource(SourceSpec::Pwl { points, .. }) = &source.kind else {
        panic!("{:?}", source.kind)
    };
    assert_eq!(
        points,
        &[
            (ns(0.5), 0.0),
            (ns(10.0) + ns(0.5), 0.0),
            (ns(10.0) + ns(0.5) + ns(1.0), 1.8),
            (ns(20.0) + ns(0.5), 1.8),
            (ns(20.0) + ns(0.5) + ns(2.0), 0.0),
        ]
    );
    // Overlapping transitions are refused at the row that starts the second.
    let overlapping = self::file("radix 1\nvname a\ntrise 5\ntfall 5\n0 0\n10 1\n12 0\n");
    let error = overlapping.input_sources("v").unwrap_err();
    assert_eq!(error.line, 7);
    assert!(
        error
            .message
            .contains("before its previous transition ends"),
        "{}",
        error.message
    );
}

#[test]
fn a_negative_delay_starts_the_waveform_at_time_zero() {
    let points = from_time_zero(vec![(-1.0, 0.0), (1.0, 2.0), (3.0, 2.0)]);
    assert_eq!(points, [(0.0, 1.0), (1.0, 2.0), (3.0, 2.0)]);
    assert_eq!(from_time_zero(vec![(-2.0, 5.0)]), [(0.0, 5.0)]);
}
