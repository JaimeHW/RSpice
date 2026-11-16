//! A transient run written through `StoreSink` reads back from its segment as exactly the columns
//! the same run returns in memory: the time axis, every saved node voltage and branch current, NaN
//! and signed zero included, and the device values, event traces and impulses beside them.

use rspice_core::NoAbort;
use rspice_core::engine::{Engine, TransientResult, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_results_store::{MemoryIo, RunStatus, SegmentReader, SegmentState, StoreSink};

/// An RC step, a rectifier, a capacitor `IC=` start, an RC driven through XSPICE bridges, a clock
/// through them with thousands of events, a memristor, a transistor whose collector current is saved, and a source stepped across a
/// capacitor, whose current and a CCVS output carry impulses.
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
    (
        "memristor",
        "memristor\nvin in 0 sin(0 1 1meg)\nr1 in mem 100\n\
         .model mrm memristor level=2 ron=50 roff=1k\n\
         ymemristor mr1 mem 0 mrm ivrelation=1\n.end\n",
        2e-6,
        5e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "clock",
        "clock\nvin a 0 pulse(0 3.3 0 0.1n 0.1n 1n 2n)\na_adc [a] [d] adc\na_inv d dn inv\n\
         a_dac [dn] [drv] dac\n\
         .model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n\
         .model inv d_inverter (rise_delay=50p fall_delay=50p)\n\
         .model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\n\
         r1 drv out 1k\nc1 out 0 1p\n.end\n",
        2e-6,
        0.1e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "transistor",
        "transistor\nvcc vcc 0 5\nvin in 0 sin(0.75 0.05 1meg)\nrb in b 1k\nq1 c b 0 qmod\n\
         rc vcc c 1k\n.model qmod npn(bf=100 cje=1p cjc=1p)\n.save v(c) @q1[ic] @q1[gm]\n.end\n",
        3e-6,
        10e-9,
        TransientStartupMode::OperatingPoint,
    ),
    (
        "impulses",
        "impulses\nV1 in 0 PWL(0 0 1n 0 1n 1 2n 1 2n 0)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\n\
         R2 out 0 10\n.save v(out) i(v1) i(h1)\n.end\n",
        3e-9,
        5e-12,
        TransientStartupMode::OperatingPoint,
    ),
];

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// The saved columns of an in-memory result, in the order a run writes them to its sink: node
/// voltages in node order, then branch currents in branch order, then the step sizes and each
/// store trace. A column output retention left out is empty.
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
    let stores = result
        .store_traces
        .iter()
        .map(|trace| (trace.name.as_str(), trace.values.as_slice()));
    let devices = result
        .device_op_traces
        .iter()
        .map(|trace| (trace.parameter.as_str(), trace.values.as_slice()));
    voltages
        .chain(currents)
        .map(|(name, column)| (name.as_str(), column.as_slice()))
        .chain(std::iter::once(("step", result.step_sizes.as_slice())))
        .chain(stores)
        .chain(devices)
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
        assert!(
            streamed.step_sizes.is_empty()
                && streamed
                    .store_traces
                    .iter()
                    .all(|trace| trace.values.is_empty()),
            "{label}: the step sizes and store traces went to the sink"
        );
        assert!(
            streamed
                .digital_traces
                .iter()
                .all(|trace| trace.points.is_empty())
                && streamed
                    .real_traces
                    .iter()
                    .all(|trace| trace.points.is_empty())
                && streamed
                    .device_op_traces
                    .iter()
                    .all(|t| t.values.is_empty())
                && streamed
                    .current_impulses
                    .iter()
                    .flatten()
                    .all(|trace| trace.points.is_empty() && trace.derivatives.is_empty())
                && streamed
                    .voltage_impulses
                    .iter()
                    .flatten()
                    .all(|trace| trace.points.is_empty() && trace.derivatives.is_empty()),
            "{label}: the device values, event traces and impulses went to the sink"
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
        // Each deck records what it is here for.
        let recorded = match label {
            "xspice bridges" => memory.digital_traces.iter().any(|t| t.points.len() > 1),
            // More changes than the run gathers before it moves them to the sink, two values each.
            "clock" => {
                memory
                    .digital_traces
                    .iter()
                    .map(|trace| trace.points.len())
                    .sum::<usize>()
                    > 4096 / 2
            }
            "transistor" => memory
                .device_op_traces
                .iter()
                .any(|trace| trace.parameter.eq_ignore_ascii_case("gm")),
            "impulses" => {
                memory
                    .current_impulses
                    .iter()
                    .flatten()
                    .any(|trace| !trace.points.is_empty() && !trace.derivatives.is_empty())
                    && memory
                        .voltage_impulses
                        .iter()
                        .flatten()
                        .any(|trace| trace.points.len() > 1)
            }
            _ => true,
        };
        assert!(recorded, "{label}: nothing to compare");
        let mut events = streamed;
        read_events(&mut events, &mut reader);
        assert_eq!(
            events.digital_traces, memory.digital_traces,
            "{label}: digital"
        );
        assert_eq!(
            events.real_traces, memory.real_traces,
            "{label}: real events"
        );
        assert_eq!(
            events.current_impulses, memory.current_impulses,
            "{label}: current impulses"
        );
        assert_eq!(
            events.voltage_impulses, memory.voltage_impulses,
            "{label}: voltage impulses"
        );
        assert_eq!(
            receipt.events,
            reader
                .schema()
                .signals
                .iter()
                .enumerate()
                .filter(|(_, signal)| signal.kind.slots() == 0)
                .map(|(id, _)| reader.event_count(id).unwrap())
                .sum::<u64>(),
            "{label}: events"
        );
    }
}

/// The event signals of `reader` read back into `shell`'s traces, by name: digital and real
/// event changes, and impulses, whose derivatives keep their order as their sequence.
fn read_events(shell: &mut TransientResult, reader: &mut SegmentReader<MemoryIo>) {
    use rspice_core::VoltageImpulsePoint;
    use rspice_core::analysis::transient::{DigitalTracePoint, RealTracePoint};
    use rspice_core::xspice::DigitalValue;
    use rspice_core::{CurrentImpulseOwner, CurrentImpulsePoint, ImpulseDerivative};
    use rspice_results_store::EventValue;
    let signals = reader.schema().signals.clone();
    for (id, signal) in signals.iter().enumerate() {
        if signal.kind.slots() != 0 {
            continue;
        }
        let changes = reader.events(id, f64::NEG_INFINITY, f64::INFINITY).unwrap();
        let real = |value: &EventValue| match value {
            EventValue::Real(value) => *value,
            other => panic!("{}: {other:?}", signal.name),
        };
        let derivatives = || {
            changes
                .iter()
                .map(|change| ImpulseDerivative {
                    time: change.time,
                    order: change.sequence,
                    coefficient: real(&change.value),
                })
                .collect::<Vec<_>>()
        };
        match signal.quantity.as_str() {
            "digital" => {
                if let Some(trace) = shell
                    .digital_traces
                    .iter_mut()
                    .find(|trace| trace.node_name == signal.name)
                {
                    trace.points = changes
                        .iter()
                        .map(|change| DigitalTracePoint {
                            time: change.time,
                            value: DigitalValue::from_store_strength(change.strength).unwrap(),
                        })
                        .collect();
                }
            }
            "real_event" => {
                if let Some(trace) = shell
                    .real_traces
                    .iter_mut()
                    .find(|trace| trace.node_name == signal.name)
                {
                    trace.points = changes
                        .iter()
                        .map(|change| RealTracePoint {
                            time: change.time,
                            value: real(&change.value),
                        })
                        .collect();
                }
            }
            "current_impulse" | "current_impulse_derivative" => {
                let trace = shell
                    .current_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| match &trace.owner {
                        CurrentImpulseOwner::Branch { branch_name } => {
                            signal.path.is_empty() && *branch_name == signal.name
                        }
                        CurrentImpulseOwner::DeviceLead {
                            device_name,
                            parameter,
                        } => *device_name == signal.path && *parameter == signal.name,
                    })
                    .unwrap();
                if signal.quantity == "current_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| CurrentImpulsePoint {
                            time: change.time,
                            charge_coulombs: real(&change.value),
                        })
                        .collect();
                } else {
                    trace.derivatives = derivatives();
                }
            }
            "voltage_impulse" | "voltage_impulse_derivative" => {
                let trace = shell
                    .voltage_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| trace.node_name == signal.name)
                    .unwrap();
                if signal.quantity == "voltage_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| VoltageImpulsePoint {
                            time: change.time,
                            volt_seconds: real(&change.value),
                        })
                        .collect();
                } else {
                    trace.derivatives = derivatives();
                }
            }
            other => panic!("unexpected {other} event signal"),
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
    for (slot, signal) in signals.iter().enumerate().take(reader.slots()) {
        let values = reader.read_slot(slot, 0, rows).unwrap();
        match signal.quantity.as_str() {
            "voltage" | "instance_current" | "instance_private" => {
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
            "step_size" => shell.step_sizes = values,
            "device_op" => {
                let trace = shell
                    .device_op_traces
                    .iter_mut()
                    .find(|trace| {
                        trace.device_name == signal.path && trace.parameter == signal.name
                    })
                    .unwrap();
                trace.values = values;
            }
            "store" => {
                let trace = shell
                    .store_traces
                    .iter_mut()
                    .find(|trace| trace.name == signal.name)
                    .unwrap();
                trace.values = values;
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
    schema: Option<rspice_core::result_sink::ResultSchema>,
    refuse: usize,
    refused_at: Option<f64>,
    offered_after: usize,
    ended: Option<rspice_core::result_sink::RunStatus>,
}

impl FillsAt {
    fn new(refuse: usize) -> Self {
        Self {
            kept: rspice_core::result_sink::VecSink::new(),
            schema: None,
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
        self.schema = Some(schema.clone());
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
    // The changes the run moved to the sink, read back into its traces by name.
    let mut stopped = run.result;
    let schema = sink.schema.clone().unwrap();
    for trace in &mut stopped.digital_traces {
        let id = schema
            .signals
            .iter()
            .position(|signal| signal.quantity == "digital" && signal.name == trace.node_name)
            .unwrap() as u32;
        trace.points = sink
            .kept
            .event_changes()
            .iter()
            .filter(|change| change.signal == id)
            .map(
                |change| rspice_core::analysis::transient::DigitalTracePoint {
                    time: change.time,
                    value: rspice_core::xspice::DigitalValue::from_store_strength(change.strength)
                        .unwrap(),
                },
            )
            .collect();
    }
    assert!(
        stopped
            .digital_traces
            .iter()
            .flat_map(|trace| &trace.points)
            .all(|point| point.time <= stop)
    );
    assert_eq!(until(&stopped), until(&memory));

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
