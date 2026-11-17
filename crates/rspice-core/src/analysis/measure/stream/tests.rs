use super::*;
use crate::abort_signal::NoAbort;
use crate::analysis::measure_signals::evaluate_tran_measurements_with_abort;

/// A run of `points` rows with uneven steps, a repeated instant, a slow sine at `out`, a
/// faster triangle at `in` and a current through `V1`.
fn run(points: usize) -> TransientResult {
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut time = Vec::with_capacity(points);
    let mut t = 0.0;
    for index in 0..points {
        time.push(t);
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        // One repeated instant, as a breakpoint leaves.
        if index != points / 3 {
            t += (0.5 + (state >> 40) as Value / (1u64 << 24) as Value) / points as Value;
        }
    }
    let out = time
        .iter()
        .map(|t| (std::f64::consts::TAU * 1.3 * t).sin() * 0.8 + 0.1)
        .collect::<Vec<_>>();
    let input = time
        .iter()
        .map(|t| {
            let phase = (t * 7.0).fract();
            if phase < 0.5 {
                2.0 * phase
            } else {
                2.0 - 2.0 * phase
            }
        })
        .collect::<Vec<_>>();
    let current = out.iter().map(|v| -v / 1.0e3).collect::<Vec<_>>();
    TransientResult {
        instance_unknowns: Vec::new(),
        potential_units: Vec::new(),
        step_sizes: vec![0.0; time.len()],
        time,
        voltages: vec![out, input],
        branch_currents: vec![current],
        num_nodes: 2,
        node_names: vec!["out".into(), "in".into()],
        branch_names: vec!["V1".into()],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        hdl_sources: vec![],
        event_only_nodes: Vec::new(),
        device_op_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        voltage_impulses: None,
        current_impulses: None,
    }
}

/// The rows `rows` of `run`.
fn slice(run: &TransientResult, rows: std::ops::Range<usize>) -> TransientResult {
    let mut slice = run.clone();
    let keep = |values: &mut Vec<Value>| *values = values[rows.clone()].to_vec();
    keep(&mut slice.time);
    keep(&mut slice.step_sizes);
    for column in slice.voltages.iter_mut().chain(&mut slice.branch_currents) {
        keep(column);
    }
    slice
}

/// The first `rows` rows of `run`.
fn probe(run: &TransientResult, rows: usize) -> TransientResult {
    slice(run, 0..rows.min(run.time.len()))
}

/// The first and last rows of `run`.
fn span(run: &TransientResult) -> TransientResult {
    let mut span = run.clone();
    let last = run.time.len() - 1;
    let ends = |values: &mut Vec<Value>| *values = vec![values[0], values[last]];
    ends(&mut span.time);
    ends(&mut span.step_sizes);
    for column in span.voltages.iter_mut().chain(&mut span.branch_currents) {
        ends(column);
    }
    span
}

/// `run` with a digital trace at `q` that rises and falls on the triangle at `in`.
fn with_events(mut run: TransientResult) -> TransientResult {
    use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};
    let mut points = Vec::new();
    let mut high = None;
    for (&time, &input) in run.time.iter().zip(&run.voltages[1]) {
        if high != Some(input > 0.5) {
            high = Some(input > 0.5);
            let state = if input > 0.5 {
                DigitalState::One
            } else {
                DigitalState::Zero
            };
            points.push(crate::analysis::transient::DigitalTracePoint {
                time,
                value: DigitalValue::new(state, DigitalStrength::Strong),
            });
        }
    }
    run.digital_traces
        .push(crate::analysis::transient::DigitalTrace {
            node_name: "q".into(),
            points,
            hdl_time_precision: None,
        });
    run
}

fn deck(cards: &str) -> Netlist {
    Netlist::parse(&format!(
        "* streamed measures\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n{cards}\n.tran 1m 1\n.end\n"
    ))
    .unwrap()
}

/// Measure `run` in one pass over chunks of `chunk` rows, or `None` when the deck is not
/// streamed.
fn streamed(netlist: &Netlist, run: &TransientResult, chunk: usize) -> Option<Vec<MeasureResult>> {
    let probe = probe(run, chunk);
    let columns = probe
        .voltages
        .iter()
        .chain(&probe.branch_currents)
        .map(|column| Some(column.as_slice()))
        .collect::<Vec<_>>();
    let mut stream = TranMeasureStream::plan(netlist, &probe, &columns, &NoAbort).unwrap()?;
    let full = run
        .voltages
        .iter()
        .chain(&run.branch_currents)
        .collect::<Vec<_>>();
    for x in run.time.chunks(chunk) {
        stream.axis(x);
    }
    loop {
        for (index, x) in run.time.chunks(chunk).enumerate() {
            let rows = index * chunk..index * chunk + x.len();
            if stream.derives()
                && !stream
                    .derive(netlist, &slice(run, rows.clone()), &NoAbort)
                    .unwrap()
            {
                return None;
            }
            let columns = stream
                .columns()
                .iter()
                .map(|&column| &full[column][rows.clone()])
                .collect::<Vec<_>>();
            stream.rows(x, &columns);
        }
        if !stream.next_pass() {
            break;
        }
    }
    stream.finish(netlist, &span(run), &NoAbort).unwrap()
}

fn bits(value: Option<Value>) -> Option<u64> {
    value.map(Value::to_bits)
}

fn assert_same(streamed: &[MeasureResult], in_memory: &[MeasureResult]) {
    assert_eq!(streamed.len(), in_memory.len());
    for (streamed, in_memory) in streamed.iter().zip(in_memory) {
        let same = streamed.name == in_memory.name
            && bits(streamed.value) == bits(in_memory.value)
            && bits(streamed.raw_value) == bits(in_memory.raw_value)
            && streamed.error == in_memory.error
            && streamed.passed == in_memory.passed
            && bits(streamed.event_axis) == bits(in_memory.event_axis)
            && streamed.units == in_memory.units
            && bits(streamed.expected) == bits(in_memory.expected)
            && streamed.failure_limit_exceeded == in_memory.failure_limit_exceeded;
        assert!(same, "streamed {streamed:?}\nin memory {in_memory:?}");
    }
}

const STATEMENTS: &str = "\
.meas tran a1 AVG V(out)
.meas tran a2 AVG v(OUT) FROM=0.1 TO=0.7
.meas tran a3 AVG V(out) TD=0.25
.meas tran a4 AVG V(out) FROM=0.7 TO=0.1
.meas tran r1 RMS V(out) FROM=0.05
.meas tran r2 RMS I(V1) TO=0.33
.meas tran i1 INTEG V(out)
.meas tran i2 INTEG V(out) FROM=0.2 TO=0.6
.meas tran i3 INTEG V(out) FROM=0.6 TO=0.2
.meas tran i4 INTEG I(V1) TD=0.4
.meas tran m1 MAX V(out)
.meas tran m2 MIN V(out) FROM=0.12345 TO=0.6789
.meas tran m3 MAX V(out) TD=0.3
.meas tran m4 MAX V(in) FROM=2 TO=3
.meas tran m5 MIN V(in) FROM=0.31 TO=0.31
.meas tran p1 PP V(out) FROM=0.1 TO=0.9
.meas tran x1 MAX V(in) RISE=2
.meas tran x2 MIN V(out) FALL=LAST
.meas tran x3 PP V(out) CROSS=1 RFC_LEVEL=0.5
.meas tran x4 MAX V(in) RISE=40
.meas tran f1 FIND V(out) AT=0.4321
.meas tran f2 FIND V(out) WHEN V(in)=0.5 RISE=2
.meas tran f3 FIND V(out) WHEN V(in)=0.5 FALL=LAST
.meas tran f4 FIND V(out) AT=7
.meas tran f5 FIND I(V1) WHEN V(out)=V(in) CROSS=2
.meas tran w1 WHEN V(out)=0.2 CROSS=3
.meas tran w2 WHEN V(out)=V(in)
.meas tran w3 WHEN V(in)=0.25 FALL=2 TD=0.5
.meas tran w4 WHEN V(in)=5
.meas tran w5 WHEN V(out)=0.3 CROSS=-2
.meas tran t0 FIND V(in) WHEN time=0.5
.meas tran d1 DERIV V(out) AT=0.5
.meas tran d2 DERIV V(out) WHEN V(in)=0.5
.meas tran t1 TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.2 RISE=1
.meas tran t2 TRIG AT=0.1 TARG V(out) VAL=0.2 FALL=LAST
.meas tran t3 TRIG V(in) VAL=0.5 RISE=1 TD=0.3 TARG V(out) VAL=0.2 CROSS=2
.meas tran t4 TRIG V(in) VAL=0.5 RISE=40 TARG V(out) VAL=0.2 RISE=1
.meas tran t5 TRIG AT=0.2 TARG AT=0.9
.meas tran q1 FOUR V(out) AT=1.3
.meas tran q2 FOUR V(in) AT=7 NUMFREQ=5 GRIDSIZE=64 FROM=0.2 TO=0.8
.meas tran q3 FOUR V(out) AT=0.4
.meas tran q4 FOUR I(V1) AT=3 TD=0.5
.meas tran pm PARAM='a1*2+m1'
.meas tran fw FIND V(in) AT=w1
.meas tran dw DERIV V(out) AT=t5
";

/// The deck of the statements `dialect` reads, and how many of them it reads.
fn dialect_deck(dialect: &str) -> (Netlist, usize) {
    let cards = STATEMENTS
        .lines()
        .filter(|card| {
            // A PARAM reads the two it names.
            card.contains("PARAM")
                || Netlist::parse(&format!("* one\n{dialect}\n{card}\n.end\n")).is_ok()
        })
        .collect::<Vec<_>>();
    (
        deck(&format!("{dialect}\n{}", cards.join("\n"))),
        cards.len(),
    )
}

#[test]
fn every_streamed_statement_measures_what_the_run_in_memory_measures_to_the_bit() {
    let run = run(2_000);
    let mut read = 0;
    for dialect in [
        "",
        ".options rspice_dialect=ngspice",
        ".options rspice_dialect=xyce",
    ] {
        let (netlist, cards) = dialect_deck(dialect);
        read = read.max(cards);
        let in_memory = evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
        assert!(
            in_memory
                .iter()
                .filter(|result| result.error.is_none())
                .count()
                > cards / 2,
            "{dialect}: {in_memory:#?}"
        );
        for chunk in [1, 2, 37, 512, 2_000] {
            let streamed = streamed(&netlist, &run, chunk)
                .unwrap_or_else(|| panic!("{dialect}: every statement streams"));
            assert_same(&streamed, &in_memory);
        }
    }
    assert_eq!(read, STATEMENTS.lines().count());
}

#[test]
fn expressions_and_differential_voltages_are_derived_from_each_chunk() {
    let run = run(1_200);
    let netlist = deck(
        ".meas tran a1 AVG V(out)
.meas tran e1 AVG par('V(out)*2+V(in)')
.meas tran e2 MAX {V(in)-V(out)} FROM=0.2 TO=0.7
.meas tran e3 FIND par('V(out)*V(in)') WHEN V(in,out)=0.1 CROSS=2
.meas tran e4 TRIG V(out,in) VAL=0 RISE=1 TARG {V(in)*0.5} VAL=0.2 FALL=2
.meas tran e5 INTEG V(in,out)
.meas tran e6 RMS V(out,0)
.meas tran e7 WHEN par('abs(V(out))')=0.5 CROSS=LAST",
    );
    let in_memory = evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
    assert!(
        in_memory.iter().all(|result| result.error.is_none()),
        "{in_memory:#?}"
    );
    for chunk in [1, 7, 300, 1_200] {
        let streamed = streamed(&netlist, &run, chunk).expect("every statement streams");
        assert_same(&streamed, &in_memory);
    }
}

#[test]
fn a_statement_the_rows_cannot_supply_is_measured_in_memory() {
    let run = run(300);
    // A time integral carries its sum from row to row, which a chunk alone does not have.
    let integral = deck(".meas tran a1 AVG V(out)\n.meas tran e AVG par('sdt(V(out))')");
    assert!(streamed(&integral, &run, 50).is_none());
    // So does an expression that reads another statement's running result.
    let running = deck(".meas tran a1 AVG V(out)\n.meas tran e AVG par('V(out)*a1')");
    assert!(streamed(&running, &run, 50).is_none());
    // A column the caller cannot read from its rows is derived from each chunk instead.
    let netlist = deck(".meas tran a1 AVG V(out)\n.meas tran c AVG I(V1)");
    let probe = probe(&run, 50);
    let columns = [
        Some(probe.voltages[0].as_slice()),
        Some(&probe.voltages[1]),
        None,
    ];
    let stream = TranMeasureStream::plan(&netlist, &probe, &columns, &NoAbort)
        .unwrap()
        .expect("the column is derived");
    assert_eq!(stream.columns(), [0]);
    assert!(stream.derives());
}

#[test]
fn a_legacy_trig_targ_measures_its_target_in_a_second_pass_after_the_trigger() {
    let run = run(1_500);
    let netlist = deck(
        ".options measure use_lttm=1
.meas tran l1 TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.2 RISE=1
.meas tran l2 TRIG V(in) VAL=0.5 RISE=2 TARG V(in) VAL=0.5 FALL=LAST
.meas tran l3 TRIG AT=0.2 TARG V(out) VAL=0.3 CROSS=1 TD=0.4
.meas tran l4 TRIG V(in) VAL=0.5 RISE=LAST TARG V(out) VAL=0.2 RISE=1
.meas tran l5 TRIG V(in) VAL=7 RISE=1 TARG V(out) VAL=0.2 RISE=1
.meas tran l6 TRIG AT=0.1 TARG AT=0.5
.meas tran l7 TRIG V(in) VAL=0.5 RISE=1 TARG V(out) VAL=0.2 FALL=1 FROM=0.3 TO=0.8
.meas tran f1 FIND V(out) WHEN V(in)=0.5 RISE=3",
    );
    let in_memory = evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
    assert!(
        in_memory
            .iter()
            .filter(|result| result.error.is_none())
            .count()
            >= 4,
        "{in_memory:#?}"
    );
    for chunk in [1, 64, 1_500] {
        let streamed = streamed(&netlist, &run, chunk).expect("every statement streams");
        assert_same(&streamed, &in_memory);
    }
}

#[test]
fn a_statement_of_event_operands_alone_is_measured_on_the_run_s_span() {
    let run = with_events(run(800));
    let netlist = deck(
        ".meas tran a1 AVG V(out)
.meas tran e1 AVG D(q)
.meas tran e2 WHEN D(q)=0.5 RISE=2
.meas tran e3 TRIG D(q) VAL=0.5 RISE=1 TARG D(q) VAL=0.5 FALL=1
.meas tran e4 MAX D(q) FROM=0.2 TO=0.9",
    );
    let in_memory = evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
    assert!(
        in_memory.iter().all(|result| result.error.is_none()),
        "{in_memory:#?}"
    );
    for chunk in [1, 100, 800] {
        let streamed = streamed(&netlist, &run, chunk).expect("every statement streams");
        assert_same(&streamed, &in_memory);
    }
}

/// `run(points)` without its repeated instant, so its axis rises strictly.
fn rising(points: usize) -> TransientResult {
    let run = run(points);
    let Some(repeat) = run.time.windows(2).position(|pair| pair[0] == pair[1]) else {
        return run;
    };
    let mut rising = run.clone();
    let drop = |values: &mut Vec<Value>| {
        values.remove(repeat + 1);
    };
    drop(&mut rising.time);
    drop(&mut rising.step_sizes);
    for column in rising
        .voltages
        .iter_mut()
        .chain(&mut rising.branch_currents)
    {
        drop(column);
    }
    rising
}

/// The ERR family and a file-backed ERROR over `run`, the reference written beside the test.
fn error_deck(reference: &std::path::Path) -> Netlist {
    let file = reference.display();
    deck(&format!(
        ".meas tran e0 ERR V(out) V(in)
.meas tran e1 ERR1 V(out) V(in) FROM=0.2 TO=0.7
.meas tran e2 ERR2 V(in) V(out) MINVAL=0.3 YMIN=0.1 YMAX=0.9
.meas tran e3 ERR1 I(V1) V(out) FROM=3 TO=4
.meas tran e4 ERR V(out) V(in) YMIN=5
.meas tran e5 ERR2 V(out) I(V1) TO=0.45
.meas tran f1 ERROR V(out) FILE={file} COMP_FUNCTION=L1NORM INDEPVARCOL=0 DEPVARCOL=1
.meas tran f2 ERROR V(in) FILE={file} COMP_FUNCTION=L2NORM INDEPVARCOL=0 DEPVARCOL=2
.meas tran f3 ERROR I(V1) FILE={file} COMP_FUNCTION=INFNORM INDEPVARCOL=0 DEPVARCOL=1
.meas tran f4 ERROR V(out) FILE={file} INDEPVARCOL=1 DEPVARCOL=1"
    ))
}

#[test]
fn error_functions_and_file_errors_measure_what_the_run_in_memory_measures_to_the_bit() {
    let reference =
        std::env::temp_dir().join(format!("rspice_stream_error_{}.csv", std::process::id()));
    // Values below the first row, inside the first and last intervals, on rows, repeated, and
    // past the last row.
    let mut text = String::from("TIME,V(1),V(2)\n");
    for (index, time) in [
        0.0, 1.0e-5, 4.0e-4, 0.001, 0.25, 0.25, 0.5, 0.75, 0.9, 0.97, 0.999, 1.2, 3.0,
    ]
    .into_iter()
    .enumerate()
    {
        text.push_str(&format!(
            "{time:e},{:e},{:e}\n",
            0.1 * index as Value - 0.4,
            (index as Value * 0.7).sin()
        ));
    }
    std::fs::write(&reference, text).unwrap();
    let netlist = error_deck(&reference);
    for points in [2, 3, 4, 5, 6, 7, 8, 9, 2_000] {
        for (strict, run) in [(true, rising(points)), (false, run(points))] {
            let in_memory =
                evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
            if points == 2_000 {
                let finite = |names: &[&str]| {
                    in_memory
                        .iter()
                        .filter(|result| names.contains(&result.name.as_str()))
                        .all(|result| result.value.is_some_and(Value::is_finite))
                };
                assert!(finite(&["E0", "E1", "E2", "E5"]), "{in_memory:#?}");
                // An interpolant is defined only over an axis that rises strictly.
                assert_eq!(finite(&["F1", "F2", "F3"]), strict, "{in_memory:#?}");
            }
            for chunk in [1, 2, 3, 7, 500, 2_000] {
                let streamed = streamed(&netlist, &run, chunk).expect("every statement streams");
                assert_same(&streamed, &in_memory);
            }
        }
    }
    let _ = std::fs::remove_file(&reference);
}

/// `run` with a current impulse through `V1` at 0.5.
fn with_impulse(mut run: TransientResult) -> TransientResult {
    use crate::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};
    run.current_impulses = Some(vec![CurrentImpulseTrace {
        derivatives: Vec::new(),
        owner: CurrentImpulseOwner::Branch {
            branch_name: "V1".into(),
        },
        complete: true,
        points: vec![CurrentImpulsePoint {
            time: 0.5,
            charge_coulombs: 1.0e-6,
        }],
    }]);
    run
}

#[test]
fn a_run_with_impulses_streams_what_the_live_impulse_route_reads_no_row_of() {
    let run = with_impulse(run(2_000));
    // No voltage sees the impulse; an extremum of the current over a window holding it fails
    // before a row is read, and one over a window without it is sampled.
    let netlist = deck(
        ".meas tran a1 AVG V(out)
.meas tran m1 MAX V(in) FROM=0.1 TO=0.9
.meas tran m2 MAX I(V1) FROM=0.1 TO=0.9
.meas tran m3 MAX I(V1) FROM=0.6 TO=0.9
.meas tran f1 FIND V(out) AT=0.3
.meas tran w1 WHEN V(in)=0.5 CROSS=2
.meas tran q1 FOUR V(out) AT=1.3",
    );
    let in_memory = evaluate_tran_measurements_with_abort(&netlist, &run, &NoAbort).unwrap();
    let failed = in_memory
        .iter()
        .filter(|result| result.error.is_some())
        .map(|result| result.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(failed, ["M2"], "{in_memory:#?}");
    for chunk in [1, 37, 2_000] {
        let streamed = streamed(&netlist, &run, chunk).expect("every statement streams");
        assert_same(&streamed, &in_memory);
    }
    // The live route reads an integral of the current row by row, and a statement another
    // statement reads.
    for cards in [
        ".meas tran a1 AVG V(out)\n.meas tran i1 INTEG I(V1)",
        ".meas tran a1 AVG V(out)\n.meas tran p1 PARAM='a1*2'",
    ] {
        assert!(streamed(&deck(cards), &run, 50).is_none(), "{cards}");
    }
}
