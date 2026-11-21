use super::*;
use crate::abort_signal::NoAbort;
use crate::analysis::transient::{DigitalTrace, DigitalTracePoint, TransientDeviceOpTrace};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};

/// A run of `points` rows with uneven steps and a repeated instant: a slow sine at `out`, a
/// triangle at `in`, a current through `V1`, an operating-point history at `R1` (a stand-in: the
/// check reads it by name), and a digital trace at `q` that rises and falls with the triangle.
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
    let mut points_q = Vec::new();
    let mut high = None;
    for (&time, &input) in time.iter().zip(&input) {
        if high != Some(input > 0.5) {
            high = Some(input > 0.5);
            let state = if input > 0.5 {
                DigitalState::One
            } else {
                DigitalState::Zero
            };
            points_q.push(DigitalTracePoint {
                time,
                value: DigitalValue::new(state, DigitalStrength::Strong),
            });
        }
    }
    let current = out.iter().map(|v| -v / 1.0e3).collect::<Vec<_>>();
    let through_r1 = input
        .iter()
        .zip(&out)
        .map(|(a, b)| (a - b) / 1.0e3)
        .collect::<Vec<_>>();
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
        digital_traces: vec![DigitalTrace {
            node_name: "q".into(),
            points: points_q,
            hdl_time_precision: None,
        }],
        digital_buses: vec![],
        real_traces: vec![],
        hdl_sources: vec![],
        event_only_nodes: Vec::new(),
        device_op_traces: vec![TransientDeviceOpTrace {
            device_name: "R1".into(),
            parameter: "id".into(),
            values: through_r1,
        }],
        store_traces: vec![],
        fft_results: vec![],
        voltage_impulses: None,
        current_impulses: None,
    }
}

/// The first `rows` rows of `run`, with its whole event histories, as a store's first chunk
/// gives them.
fn probe(run: &TransientResult, rows: usize) -> TransientResult {
    let mut probe = run.clone();
    let rows = rows.min(run.time.len());
    let keep = |values: &mut Vec<Value>| values.truncate(rows);
    keep(&mut probe.time);
    keep(&mut probe.step_sizes);
    for column in probe.voltages.iter_mut().chain(&mut probe.branch_currents) {
        keep(column);
    }
    for trace in &mut probe.device_op_traces {
        keep(&mut trace.values);
    }
    probe
}

/// The run's last accepted time.
fn stop(run: &TransientResult) -> Value {
    *run.time.last().unwrap()
}

/// Every column of `result` a store could give back a row of.
fn columns(result: &TransientResult) -> Vec<&[Value]> {
    result
        .voltages
        .iter()
        .chain(&result.branch_currents)
        .map(Vec::as_slice)
        .chain(
            result
                .device_op_traces
                .iter()
                .map(|trace| trace.values.as_slice()),
        )
        .collect()
}

/// Check `run` in one pass over chunks of `chunk` rows, or `None` when the deck is not
/// streamed.
fn streamed(
    netlist: &Netlist,
    run: &TransientResult,
    chunk: usize,
) -> Option<Vec<AssertionReport>> {
    let probe = probe(run, chunk);
    let offered = columns(&probe).into_iter().map(Some).collect::<Vec<_>>();
    let mut stream =
        TranAssertStream::plan(netlist, &probe, stop(run), &offered, &NoAbort).unwrap()?;
    let full = columns(run);
    for (index, x) in run.time.chunks(chunk).enumerate() {
        let rows = index * chunk..index * chunk + x.len();
        let columns = stream
            .columns()
            .iter()
            .map(|&column| &full[column][rows.clone()])
            .collect::<Vec<_>>();
        stream.rows(x, &columns);
    }
    stream.finish()
}

fn deck(cards: &str) -> Netlist {
    Netlist::parse(&format!(
        "* streamed assertions\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n{cards}\n.tran 1m 1\n.end\n"
    ))
    .unwrap()
}

const STATEMENTS: &str = "\
.ASSERT band EXPR='V(out)' MAX=0.5 LEVEL=ERROR MESSAGE='out above half'
.ASSERT both EXPR='V(out)-V(in)' MIN=-0.6 MAX=0.4
.ASSERT window EXPR='V(out)' MIN=-0.3 FROM=0.2 TO=0.7
.ASSERT late EXPR='V(in)' MAX=0.8 FROM=0.55
.ASSERT long EXPR='V(out)' MAX=0.6 DURATION=0.05
.ASSERT short EXPR='V(in)' MAX=0.9 DURATION=0.5
.ASSERT current EXPR='I(V1)' MIN=-0.0007
.ASSERT curved EXPR='V(out)*V(out)+V(in)' MAX=1.1
.ASSERT both_ends COND='V(out) > 0.2 && V(in) < 0.9'
.ASSERT clock COND='TIME < 0.4 || V(out) < 0.3'
.ASSERT events COND='D(q)==1' TO=0.6
.ASSERT mixed COND='D(q)==0 || V(out)<0.5'
.ASSERT dev DEV=R1 PARAM=id MAX=0.0004
.ASSERT instant EXPR='V(out)' MAX=0.0 FROM=0.3 TO=0.3
.ASSERT at_stop EXPR='V(out)' MAX=-1 FROM=5
.ASSERT before EXPR='V(out)' MAX=-1 TO=-1";

fn same(streamed: &[AssertionReport], in_memory: &[AssertionReport]) {
    assert_eq!(streamed.len(), in_memory.len());
    for (streamed, in_memory) in streamed.iter().zip(in_memory) {
        assert_eq!(format!("{streamed:?}"), format!("{in_memory:?}"));
    }
}

#[test]
fn every_check_read_a_chunk_at_a_time_reports_what_the_whole_run_reports() {
    let netlist = deck(STATEMENTS);
    for points in [1, 2, 3, 40, 997] {
        let run = run(points);
        let in_memory = evaluate_tran_assertions(&netlist, &run, &NoAbort).unwrap();
        assert!(
            points < 40 || in_memory.iter().any(|report| !report.violations.is_empty()),
            "the deck finds violations to compare"
        );
        for chunk in [1, 2, 7, 64, 4096] {
            let streamed = streamed(&netlist, &run, chunk)
                .unwrap_or_else(|| panic!("{points} rows in chunks of {chunk} are streamed"));
            same(&streamed, &in_memory);
        }
    }
}

#[test]
fn a_card_the_rows_cannot_answer_leaves_the_run_to_be_checked_in_memory() {
    let run = run(50);
    // A name the run does not hold: the report the evaluation in memory words.
    assert!(streamed(&deck(".ASSERT gone EXPR='V(nowhere)' MAX=1"), &run, 8).is_none());
    // A column the store does not give back.
    let netlist = deck(".ASSERT band EXPR='V(out)' MAX=0.5");
    let probe = probe(&run, 8);
    let offered = vec![None; columns(&probe).len()];
    assert!(
        TranAssertStream::plan(&netlist, &probe, stop(&run), &offered, &NoAbort)
            .unwrap()
            .is_none()
    );
    // A time axis that runs backwards.
    let mut backwards = run.clone();
    backwards.time.swap(10, 11);
    assert!(streamed(&netlist, &backwards, 8).is_none());
}
