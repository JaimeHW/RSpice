use rspice_core::NoAbort;
use rspice_core::engine::{Engine, TransientStartupMode};
use rspice_core::netlist::Netlist;
use rspice_results_store::{MemoryIo, Signal, SignalKind, StoreSink};

use super::*;

/// The voltage, current and device columns a result saved, by name, as bits.
fn saved(result: &TransientResult) -> Vec<(String, Vec<u64>)> {
    let bits = |values: &[f64]| values.iter().map(|value| value.to_bits()).collect();
    let mut saved = Vec::new();
    for (name, values) in result.node_names.iter().zip(&result.voltages) {
        if !values.is_empty() {
            saved.push((format!("v({name})"), bits(values)));
        }
    }
    for (name, values) in result.branch_names.iter().zip(&result.branch_currents) {
        if !values.is_empty() {
            saved.push((format!("i({name})"), bits(values)));
        }
    }
    for trace in &result.device_op_traces {
        let name = format!("{}[{}]", trace.device_name, trace.parameter);
        saved.push((name, bits(&trace.values)));
    }
    for trace in &result.store_traces {
        saved.push((trace.name.clone(), bits(&trace.values)));
    }
    saved
}

#[test]
fn a_stored_transient_opened_from_its_segment_alone_is_the_run_it_returned_in_memory() {
    let decks = [
        "rc step\nv1 in 0 pulse(0 1 10n 1n 1n 200n 400n)\nr1 in out 1k\nc1 out 0 100p\n.end\n",
        "clock\nvin a 0 pulse(0 3.3 0 0.1n 0.1n 1n 2n)\na_adc [a] [d] adc\na_inv d dn inv\n\
         a_dac [dn] [drv] dac\n\
         .model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n\
         .model inv d_inverter (rise_delay=50p fall_delay=50p)\n\
         .model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\n\
         r1 drv out 1k\nc1 out 0 1p\n.end\n",
        "transistor\nvcc vcc 0 5\nvin in 0 sin(0.75 0.05 1meg)\nrb in b 1k\nq1 c b 0 qmod\n\
         rc vcc c 1k\n.model qmod npn(bf=100 cje=1p cjc=1p)\n.save v(c) @q1[ic] @q1[gm]\n.end\n",
        "memristor\nvin in 0 sin(0 1 1meg)\nr1 in mem 100\n\
         .model mrm memristor level=2 ron=50 roff=1k\n\
         ymemristor mr1 mem 0 mrm ivrelation=1\n.end\n",
        "impulses\nV1 in 0 PWL(0 0 1n 0 1n 1 2n 1 2n 0)\nC1 in 0 2p\nH1 out 0 V1 3\nC2 out 0 5p\n\
         R2 out 0 10\n.save v(out) i(v1) i(h1)\n.end\n",
    ];
    for deck in decks {
        let label = deck.lines().next().unwrap();
        let netlist = Netlist::parse(deck).unwrap();
        let (tstop, max_step) = (2e-6, 5e-9);
        let engine = Engine::default();
        let startup = TransientStartupMode::OperatingPoint;
        let memory = engine
            .run_tran_with_startup_mode_and_abort(&netlist, tstop, max_step, startup, &NoAbort)
            .unwrap();
        let mut sink = StoreSink::new(MemoryIo::new());
        engine
            .run_tran_into_sink_with_startup_mode_and_abort(
                &netlist, tstop, max_step, startup, &NoAbort, &mut sink,
            )
            .unwrap();
        let io = MemoryIo::from_bytes(sink.into_io().unwrap().into_bytes());
        let mut run = StoredTransientRun::open(io).unwrap();
        assert!(run.shell.time.is_empty(), "{label}: open reads no rows");
        run.read_all().unwrap();
        let opened = &run.shell;
        let bits = |values: &[f64]| values.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&opened.time), bits(&memory.time), "{label}");
        assert_eq!(saved(opened), saved(&memory), "{label}");
        for (name, unknown) in opened.node_names.iter().zip(&opened.instance_unknowns) {
            let index = memory.node_names.iter().position(|n| n == name).unwrap();
            assert_eq!(*unknown, memory.instance_unknowns[index], "{label}: {name}");
        }
        let debug = |value: &dyn std::fmt::Debug| format!("{value:?}");
        assert_eq!(
            debug(&opened.digital_traces),
            debug(&memory.digital_traces),
            "{label}"
        );
        assert_eq!(debug(&opened.real_traces), debug(&memory.real_traces));
        assert_eq!(
            debug(&opened.current_impulses),
            debug(&memory.current_impulses),
            "{label}"
        );
        assert_eq!(
            debug(&opened.voltage_impulses),
            debug(&memory.voltage_impulses),
            "{label}"
        );
    }
}

#[test]
fn a_verilog_a_instance_unknown_is_named_as_the_run_marked_it() {
    let node = |name: &str, unit: &str, quantity: &str| {
        let mut signal = Signal::new(name, unit, SignalKind::Real);
        signal.quantity = quantity.to_owned();
        signal
    };
    let schema = Schema {
        x: Default::default(),
        signals: vec![
            node("out", "V", "voltage"),
            node("temp", "K", "voltage"),
            node("x1:b", "A", "instance_current"),
            node("x1:__br1", "V", "instance_private"),
            node("v1", "A", "current"),
        ],
    };
    let shell = transient_shell(&schema).unwrap();
    assert_eq!(shell.node_names, ["out", "temp", "x1:b", "x1:__br1"]);
    assert_eq!(
        shell.instance_unknowns,
        [
            None,
            None,
            Some(InstanceUnknown::BranchCurrent),
            Some(InstanceUnknown::Private)
        ]
    );
    assert_eq!(
        shell.potential_units,
        [None, Some("K".to_owned()), None, None]
    );
    assert_eq!((shell.num_nodes, shell.branch_names.len()), (4, 1));
    let mut unknown = schema;
    unknown.signals.push(node("q", "C", "charge"));
    let refused = transient_shell(&unknown).unwrap_err();
    assert!(refused.contains("a 'charge' signal 'q'"), "{refused}");
}
