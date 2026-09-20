//! Terminal meters preserve authored hierarchy, total current, and replay identity.
use rspice_core::netlist::{SaveSignal, TerminalCurrentProbe};
use rspice_core::{Engine, Netlist, SimulationConfig};

fn probe(device: &str, terminal: usize) -> TerminalCurrentProbe {
    TerminalCurrentProbe {
        device: device.into(),
        terminal,
        source_name: "VMETER".into(),
        node_name: "meter_private".into(),
    }
}

#[test]
fn terminal_current_probes_preserve_hierarchy_replay_and_checkpoint_identity() {
    let original = Netlist::parse("Terminal meter\n.param RR=1k\nV1 in 0 PWL(0 0 1n 0 5n 1)\nX1 in out CELL\nC1 out 0 1p\n.subckt CELL a b\nR1 a b {RR}\n.ends CELL\n.options RSHUNT=10k\n.end\n").unwrap();
    let flat = rspice_core::netlist::flatten_netlist_with_models(&original).unwrap();
    let target = &flat
        .elements
        .iter()
        .find(|e| e.name.ends_with("R1"))
        .unwrap()
        .name;
    let mut metered = original.clone();
    metered.options.topology_supernode = Some(true);
    metered.add_terminal_current_probe(probe(target, 0));
    let (mut replayed, applied) =
        Engine::create_perturbed_netlist_multi(&metered, &[("RR".into(), 2000.0)]).unwrap();
    assert!(applied > 0);
    replayed.saves.signals = vec![
        SaveSignal::Voltage("in".into()),
        SaveSignal::Voltage("out".into()),
        SaveSignal::Current("V1".into()),
        SaveSignal::Current("VMETER".into()),
    ];
    let engine = Engine::new(SimulationConfig::default());
    let (result, checkpoint) = engine
        .run_tran_checkpointed(&replayed, 6e-9, 0.1e-9)
        .unwrap();
    let current = result.try_branch_current_waveform_named("VMETER").unwrap();
    let source = result.try_branch_current_waveform_named("V1").unwrap();
    let input = result.try_voltage_waveform_named("in").unwrap();
    let output = result.try_voltage_waveform_named("out").unwrap();
    assert!(current.iter().any(|i| *i > 1e-5));
    for index in 0..result.time.len() {
        let expected = (input[index] - output[index]) / 2000.0;
        assert!((current[index] - expected).abs() < 1e-10);
        assert!((current[index] + source[index] + input[index] / 10000.0).abs() < 1e-10);
    }
    engine
        .run_tran_resume(&replayed, &checkpoint, 7e-9, 0.1e-9)
        .unwrap();
    let (mut other, _) =
        Engine::create_perturbed_netlist_multi(&original, &[("RR".into(), 2000.0)]).unwrap();
    other.options.topology_supernode = Some(true);
    other.saves = replayed.saves.clone();
    other.add_terminal_current_probe(probe(target, 1));
    let error = engine
        .run_tran_resume(&other, &checkpoint, 7e-9, 0.1e-9)
        .unwrap_err();
    assert!(error.to_string().contains("different netlist"), "{error}");
}

#[test]
fn terminal_current_probes_reject_collisions_missing_pins_and_resource_overflow() {
    let original = Netlist::parse("Probe validation\nV1 n 0 1\nR1 n 0 1k\n.end\n").unwrap();
    let engine = Engine::new(SimulationConfig::default());
    for invalid in [
        probe("missing", 0),
        probe("R1", 2),
        TerminalCurrentProbe {
            source_name: "v1".into(),
            ..probe("R1", 0)
        },
        TerminalCurrentProbe {
            node_name: "n".into(),
            ..probe("R1", 0)
        },
        TerminalCurrentProbe {
            node_name: "GND".into(),
            ..probe("R1", 0)
        },
    ] {
        let mut netlist = original.clone();
        netlist.add_terminal_current_probe(invalid);
        assert!(
            engine
                .run_dc_op(&netlist)
                .unwrap_err()
                .to_string()
                .contains("Terminal current probe")
        );
    }
    let mut limited = SimulationConfig::default();
    limited.resource_limits.max_flattened_elements = 2;
    let mut netlist = original;
    netlist.add_terminal_current_probe(probe("R1", 0));
    assert!(matches!(
        Engine::new(limited).run_dc_op(&netlist),
        Err(rspice_core::engine::SimulationError::ResourceLimit(_))
    ));
}
