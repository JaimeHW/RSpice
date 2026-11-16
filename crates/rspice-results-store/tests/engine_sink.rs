//! A transient run written through `StoreSink` reads back from its segment as exactly the columns
//! the same run returns in memory: the time axis, every saved node voltage and branch current, NaN
//! and signed zero included.

use rspice_core::NoAbort;
use rspice_core::engine::{Engine, TransientResult, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_results_store::{MemoryIo, RunStatus, SegmentReader, SegmentState, StoreSink};

/// An RC step, a rectifier, a capacitor `IC=` start, and an RC driven through XSPICE bridges.
const DECKS: &[(&str, &str, f64, f64, TransientStartupMode)] = &[
    (
        "rc",
        "rc step\nv1 in 0 pulse(0 1 10n 1n 1n 200n 400n)\nr1 in out 1k\nc1 out 0 100p\n.end\n",
        1e-6,
        5e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "rectifier",
        "rectifier\nv1 in 0 sin(0 5 1meg)\nd1 in out dmod\nr1 out 0 1k\nc1 out 0 10n\n.model dmod d(is=1e-14 cjo=2p)\n.end\n",
        3e-6,
        10e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "capacitor ic",
        "capacitor ic\nc1 a 0 1n ic=1\nr1 a b 1k\nc2 b 0 2n ic=-0.5\nr2 b 0 2k\n.end\n",
        5e-6,
        20e-9,
        TransientStartupMode::Uic,
    ),
    (
        "xspice bridges",
        "xspice bridges\nvin a 0 pwl(0 0 1n 0 1.1n 3.3)\na_adc [a] [d] adc\na_dac [d] [drv] dac\n.model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\nr1 drv out 1k\nc1 out 0 1p\n.end\n",
        5e-9,
        0.05e-9,
        TransientStartupMode::OperatingPoint,
    ),
];

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// The saved columns of an in-memory result, in the order a run writes them to its sink: node
/// voltages in node order, then branch currents in branch order. A column output retention left
/// out is empty.
fn saved_columns(result: &TransientResult) -> Vec<(&str, &[f64])> {
    let voltages = result
        .node_names
        .iter()
        .zip(&result.voltages)
        .filter(|(_, column)| !column.is_empty());
    let currents = result
        .branch_names
        .iter()
        .zip(&result.branch_currents)
        .filter(|(_, column)| !column.is_empty());
    voltages
        .chain(currents)
        .map(|(name, column)| (name.as_str(), column.as_slice()))
        .collect()
}

#[test]
fn a_transient_written_through_the_store_reads_back_as_the_in_memory_result() {
    for &(label, deck, tstop, max_step, startup) in DECKS {
        let netlist = Netlist::parse(deck).unwrap_or_else(|error| panic!("{label}: {error}"));
        let engine = Engine::default();
        let memory = engine
            .run_tran_with_startup_mode_and_abort(&netlist, tstop, max_step, startup, &NoAbort)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let mut store = StoreSink::new(MemoryIo::new());
        let run = engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                &netlist, tstop, max_step, startup, &NoAbort, &mut store,
            )
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(run.stopped, None, "{label}");
        let (streamed, receipt) = (run.result, run.receipt);
        assert!(
            streamed.time.is_empty(),
            "{label}: the rows went to the sink"
        );
        assert!(
            streamed
                .voltages
                .iter()
                .chain(&streamed.branch_currents)
                .all(Vec::is_empty),
            "{label}: the waveforms went to the sink"
        );
        assert_eq!(
            bits(&streamed.step_sizes),
            bits(&memory.step_sizes),
            "{label}: the same accepted steps"
        );
        assert_eq!(
            streamed.digital_traces, memory.digital_traces,
            "{label}: event traces"
        );

        let columns = saved_columns(&memory);
        assert_eq!(receipt.rows, memory.time.len() as u64, "{label}: rows");
        let mut reader = SegmentReader::open(store.into_io().unwrap()).unwrap();
        assert!(
            matches!(
                reader.state(),
                SegmentState::Complete {
                    status: RunStatus::Completed,
                    ..
                }
            ),
            "{label}: the segment ended"
        );
        assert_eq!(reader.slots(), columns.len(), "{label}: slots");
        let rows = memory.time.len();
        assert_eq!(
            bits(&reader.read_x(0, rows).unwrap()),
            bits(&memory.time),
            "{label}: time"
        );
        for (slot, (name, column)) in columns.iter().enumerate() {
            assert_eq!(
                reader.schema().signals[slot].name,
                *name,
                "{label}: slot {slot}"
            );
            assert_eq!(
                bits(&reader.read_slot(slot, 0, rows).unwrap()),
                bits(column),
                "{label}: {name}"
            );
        }
    }
}

/// The run read back from `reader` into `shell`, as a frontend reads it: each voltage and current
/// column into its node's or branch's place, each instance-parameter column as a device trace.
fn read_back(shell: &mut TransientResult, reader: &mut SegmentReader<MemoryIo>) {
    let rows = reader.rows() as usize;
    shell.time = reader.read_x(0, rows).unwrap();
    let (mut next_node, mut next_branch) = (0, 0);
    let signals = reader.schema().signals.clone();
    for (slot, signal) in signals.iter().enumerate() {
        let values = reader.read_slot(slot, 0, rows).unwrap();
        match signal.quantity.as_str() {
            "voltage" => {
                next_node += shell.node_names[next_node..]
                    .iter()
                    .position(|name| *name == signal.name)
                    .unwrap();
                shell.voltages[next_node] = values;
                next_node += 1;
            }
            "current" => {
                next_branch += shell.branch_names[next_branch..]
                    .iter()
                    .position(|name| *name == signal.name)
                    .unwrap();
                shell.branch_currents[next_branch] = values;
                next_branch += 1;
            }
            "instance_parameter" => {
                shell.adopt_instance_parameter(&signal.path, &signal.name, values);
            }
            other => panic!("unexpected {other} column"),
        }
    }
}

#[test]
fn a_deck_whose_outputs_read_whole_waveforms_completes_them_from_the_rows_read_back() {
    let netlist = Netlist::parse(
        "fft\nv1 in 0 sin(0 1 1meg)\nr1 in out 1k\nc1 out 0 10p\n.tran 1n 4u\n\
         .fft v(out) np=1024\n.print tran v(out) p(r1) r1:r\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let expected = engine
        .run_tran_with_startup_mode_and_abort(
            &netlist,
            4e-6,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
        )
        .unwrap();
    assert!(!expected.fft_results.is_empty());
    let traces = |result: &TransientResult| {
        result
            .device_op_traces
            .iter()
            .map(|trace| format!("{}:{}", trace.device_name, trace.parameter))
            .collect::<Vec<_>>()
    };
    assert!(
        traces(&expected).iter().any(|name| name == "R1:r") && traces(&expected).len() >= 2,
        "an instance parameter and a lead power: {:?}",
        traces(&expected)
    );

    let mut store = StoreSink::new(MemoryIo::new());
    let mut shell = engine
        .run_tran_into_sink_with_startup_mode_and_abort(
            &netlist,
            4e-6,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
            &mut store,
        )
        .unwrap()
        .result;
    assert!(shell.fft_results.is_empty() && shell.device_op_traces.is_empty());
    let mut reader = SegmentReader::open(store.into_io().unwrap()).unwrap();
    read_back(&mut shell, &mut reader);
    engine
        .finish_transient_from_rows(&netlist, &mut shell, 4e-6, &NoAbort)
        .unwrap();
    assert_eq!(traces(&shell), traces(&expected));
    for (actual, wanted) in shell
        .device_op_traces
        .iter()
        .zip(&expected.device_op_traces)
    {
        assert_eq!(
            bits(&actual.values),
            bits(&wanted.values),
            "{}",
            wanted.parameter
        );
    }
    assert_eq!(
        format!("{:?}", shell.fft_results),
        format!("{:?}", expected.fft_results)
    );
}

/// A sink that keeps rows in memory and refuses the `refuse`th as a full disk, counting any row
/// offered after it.
struct FillsAt {
    kept: rspice_core::result_sink::VecSink,
    refuse: usize,
    refused_at: Option<f64>,
    offered_after: usize,
    ended: Option<rspice_core::result_sink::RunStatus>,
}

impl FillsAt {
    fn new(refuse: usize) -> Self {
        Self {
            kept: rspice_core::result_sink::VecSink::new(),
            refuse,
            refused_at: None,
            offered_after: 0,
            ended: None,
        }
    }
}

impl rspice_core::result_sink::ResultSink for FillsAt {
    fn begin(
        &mut self,
        schema: &rspice_core::result_sink::ResultSchema,
        meta: &rspice_core::result_sink::RunMeta,
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.kept.begin(schema, meta)
    }

    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), rspice_core::result_sink::SinkStop> {
        let full = rspice_core::result_sink::SinkStop::DiskFull {
            needed: None,
            available: None,
        };
        if self.refused_at.is_some() {
            self.offered_after += 1;
            return Err(full);
        }
        if self.kept.x().len() == self.refuse {
            self.refused_at = Some(x);
            return Err(full);
        }
        self.kept.row(x, values)
    }

    fn amend_open_row(
        &mut self,
        row: u64,
        column: u32,
        value: f64,
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.kept.amend_open_row(row, column, value)
    }

    fn events(
        &mut self,
        changes: &[rspice_core::result_sink::EventChange],
    ) -> Result<(), rspice_core::result_sink::SinkStop> {
        self.kept.events(changes)
    }

    fn end(
        &mut self,
        status: rspice_core::result_sink::RunStatus,
    ) -> Result<rspice_core::result_sink::SinkReceipt, rspice_core::result_sink::SinkStop> {
        self.ended = Some(status.clone());
        self.kept.end(status)
    }
}

#[test]
fn a_sink_out_of_disk_ends_the_run_at_the_refused_point_with_no_later_step_or_event() {
    use rspice_core::result_sink::{RunStatus, SinkStop, StopReason};
    // A clock through an adc bridge, an inverter and a dac: an event at every edge.
    let netlist = Netlist::parse(
        "clocked events\n\
         vin a 0 pulse(0 3.3 0 0.1n 0.1n 1n 2n)\n\
         a_adc [a] [d] adc\n\
         a_inv d dn inv\n\
         a_dac [dn] [drv] dac\n\
         .model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n\
         .model inv d_inverter (rise_delay=50p fall_delay=50p)\n\
         .model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\n\
         r1 drv out 1k\n\
         c1 out 0 1p\n\
         .tran 0.05n 100n\n\
         .end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let memory = engine
        .run_tran_with_startup_mode_and_abort(
            &netlist,
            100e-9,
            0.05e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
        )
        .unwrap();
    let refuse = memory.time.len() / 2;
    let mut sink = FillsAt::new(refuse);
    let run = engine
        .run_tran_into_sink_with_startup_mode_and_abort(
            &netlist,
            100e-9,
            0.05e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
            &mut sink,
        )
        .unwrap();
    assert!(matches!(run.stopped, Some(SinkStop::DiskFull { .. })));
    assert_eq!(sink.ended, Some(RunStatus::Stopped(StopReason::DiskFull)));
    assert_eq!(run.receipt.rows, refuse as u64);
    // The run took no step after the refused point: no row was offered after it.
    assert_eq!(sink.offered_after, 0);
    let stop = sink.refused_at.unwrap();
    assert_eq!(stop.to_bits(), memory.time[refuse].to_bits());
    assert_eq!(bits(sink.kept.x()), bits(&memory.time[..refuse]));
    // Every event up to the refused point happened, as in the whole run, and none after it.
    assert!(
        memory
            .digital_traces
            .iter()
            .any(|trace| trace.points.iter().any(|point| point.time > stop)),
        "the whole run has events after the stop"
    );
    let until = |result: &TransientResult| {
        result
            .digital_traces
            .iter()
            .map(|trace| {
                let points: Vec<_> = trace
                    .points
                    .iter()
                    .filter(|point| point.time <= stop)
                    .map(|point| (point.time.to_bits(), format!("{:?}", point.value)))
                    .collect();
                (trace.node_name.clone(), points)
            })
            .collect::<Vec<_>>()
    };
    assert!(
        run.result
            .digital_traces
            .iter()
            .flat_map(|trace| &trace.points)
            .all(|point| point.time <= stop)
    );
    assert_eq!(until(&run.result), until(&memory));

    // Refused at its first row, the run stops there too, typed, with nothing kept.
    let mut sink = FillsAt::new(0);
    let run = engine
        .run_tran_into_sink_with_startup_mode_and_abort(
            &netlist,
            100e-9,
            0.05e-9,
            TransientStartupMode::OperatingPoint,
            &NoAbort,
            &mut sink,
        )
        .unwrap();
    assert!(matches!(run.stopped, Some(SinkStop::DiskFull { .. })));
    assert_eq!(run.receipt.rows, 0);
    assert_eq!(sink.offered_after, 0);
    assert_eq!(sink.refused_at, Some(0.0));
}
