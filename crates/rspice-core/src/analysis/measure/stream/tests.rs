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

/// The first `rows` rows of `run`.
fn probe(run: &TransientResult, rows: usize) -> TransientResult {
    let mut probe = run.clone();
    probe.time.truncate(rows);
    probe.step_sizes.truncate(rows);
    for column in probe.voltages.iter_mut().chain(&mut probe.branch_currents) {
        column.truncate(rows);
    }
    probe
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
    for (index, x) in run.time.chunks(chunk).enumerate() {
        let rows = index * chunk..index * chunk + x.len();
        let columns = stream
            .columns()
            .iter()
            .map(|&column| &full[column][rows.clone()])
            .collect::<Vec<_>>();
        stream.rows(x, &columns);
    }
    stream.finish(netlist, &probe, &NoAbort).unwrap()
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
.meas tran pm PARAM='a1*2+m1'
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
fn a_statement_the_rows_cannot_supply_is_measured_in_memory() {
    let run = run(300);
    // An expression is a waveform of its own, not a column of the rows.
    let expression = deck(".meas tran a1 AVG V(out)\n.meas tran e AVG par('V(out)*2')");
    assert!(streamed(&expression, &run, 50).is_none());
    // So is a column the caller cannot read from its rows.
    let netlist = deck(".meas tran a1 AVG V(out)\n.meas tran c AVG I(V1)");
    let probe = probe(&run, 50);
    let columns = [
        Some(probe.voltages[0].as_slice()),
        Some(&probe.voltages[1]),
        None,
    ];
    assert!(
        TranMeasureStream::plan(&netlist, &probe, &columns, &NoAbort)
            .unwrap()
            .is_none()
    );
    // And a FIND placed at an earlier result, which only the finished pass knows.
    let earlier = deck(".meas tran w WHEN V(out)=0.2\n.meas tran f FIND V(in) WHEN V(out)=w");
    assert!(streamed(&earlier, &run, 50).is_none());
}
