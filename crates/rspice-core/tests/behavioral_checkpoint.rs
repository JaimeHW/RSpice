//! Accepted SDT history across public transient save/load and resume routes.
use rspice_core::engine::{Engine, TransientCheckpoint, TransientCheckpointEncoding};
use rspice_core::netlist::Netlist;

fn capacitor_fixture() -> (Engine, Netlist, TransientCheckpoint) {
    use rspice_core::config::{ExpressionDialect, SpiceDialect};
    use rspice_core::engine::SimulationConfig;
    use rspice_core::netlist::NetlistParseOptions;
    let netlist = Netlist::parse_with_options(
        "Capacitor integration checkpoint\nvin in 0 dc 1\n\
        r1 in out 1k\nc1 out 0 c={1u+1m*sdt(v(in))} ic=0\n\
        r2 in nested 1k\nc2 nested 0 c={1u+sdt(sdt(v(in)))} ic=0\n\
        cstatic in 0 1n\n.tran 2u 1m\n.end\n",
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    assert!(
        engine
            .preflight_transient_checkpoint(&netlist)
            .unwrap()
            .is_resumable()
    );
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 400e-6, 2e-6)
        .unwrap();
    assert!(checkpoint.capability().is_resumable());
    (engine, netlist, checkpoint)
}

#[test]
fn capacitor_checkpoint_restores_integrals_and_physical_charge_together() {
    let (engine, netlist, checkpoint) = capacitor_fixture();
    let (reference, reference_final) = engine
        .run_tran_resume(&netlist, &checkpoint, 1e-3, 2e-6)
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let loaded =
            TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
        assert_eq!(loaded, checkpoint);
        let (resumed, final_state) = engine
            .run_tran_resume(&netlist, &loaded, 1e-3, 2e-6)
            .unwrap();
        assert_eq!(final_state, reference_final);
        assert_eq!(resumed.time, reference.time);
        assert_eq!(resumed.voltages, reference.voltages);
    }
    // Xyce's expression capacitance integrates dQ=C(t)*dV. For R=1k,
    // C(t)=1u+1m*t and v(0)=0, C(t)*dv/dt=(1-v)/R gives
    // v(t)=1-C(0)/C(t), hence v(1ms)=0.5.
    let voltage = reference
        .try_voltage_waveform_named("out")
        .unwrap()
        .last()
        .unwrap();
    assert!((voltage - 0.5).abs() < 2e-4, "resumed voltage {voltage}");
    let uninterrupted = engine.run_tran(&netlist, 1e-3, 2e-6).unwrap();
    for name in ["out", "nested"] {
        let actual = reference
            .try_voltage_waveform_named(name)
            .unwrap()
            .last()
            .unwrap();
        let expected = uninterrupted
            .try_voltage_waveform_named(name)
            .unwrap()
            .last()
            .unwrap();
        assert!(
            (actual - expected).abs() < 2e-4,
            "{name}: {actual} vs {expected}"
        );
    }
}

#[test]
fn capacitor_checkpoint_rejects_malformed_mismatched_and_missing_integrals() {
    let (engine, netlist, checkpoint) = capacitor_fixture();
    let text = checkpoint.to_text();
    assert!(text.contains("capacitor_sdt_states 2\n"));
    let row = text
        .lines()
        .find(|line| line.starts_with("capacitor_sdt "))
        .unwrap();
    for replacement in [
        "capacitor_sdt 0 1 0.0004".to_owned(),
        "capacitor_sdt NaN 1 0.0004".to_owned(),
        format!("{row} extra"),
    ] {
        assert!(TransientCheckpoint::from_text(&text.replacen(row, &replacement, 1)).is_err());
    }
    let source = text
        .lines()
        .find(|line| line.starts_with("capacitor_sdt_state "))
        .unwrap();
    let fields: Vec<_> = source.split_whitespace().collect();
    let wrong = TransientCheckpoint::from_text(&text.replacen(
        source,
        &format!("capacitor_sdt_state missing {}", fields[2]),
        1,
    ))
    .unwrap();
    let error = engine
        .run_tran_resume(&netlist, &wrong, 1e-3, 2e-6)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("capacitor SDT checkpoint identity"),
        "{error}"
    );
    assert!(
        TransientCheckpoint::from_text(&text.replacen(
            source,
            &format!("capacitor_sdt_state {} {}", fields[1], usize::MAX),
            1
        ))
        .is_err()
    );

    let mut old = String::new();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("RSPICE-CHECKPOINT 52"));
    old.push_str("RSPICE-CHECKPOINT 50\n");
    while let Some(line) = lines.next() {
        if line.starts_with("accepted_bsim3_") {
            continue;
        }
        if line.starts_with("capacitor_sdt_state_available ") {
            continue;
        }
        if line.starts_with("capacitor_sdt_states ") {
            let count = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            for _ in 0..count {
                let count = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                for _ in 0..count {
                    lines.next().unwrap();
                }
            }
            continue;
        }
        old.push_str(line);
        old.push('\n');
    }
    let legacy = TransientCheckpoint::from_text(&old).unwrap();
    // Re-export must preserve unknown history, not invent zero-valued integrals.
    let legacy = TransientCheckpoint::from_text(&legacy.to_text()).unwrap();
    let error = engine
        .run_tran_resume(&netlist, &legacy, 1e-3, 2e-6)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("does not contain capacitor SDT history"),
        "{error}"
    );
}

fn fixture() -> (Engine, Netlist, TransientCheckpoint) {
    let netlist = Netlist::parse(
        "Behavioral integration checkpoint\nvin in 0 dc 1\n\
        bfirst first 0 v=1k*sdt(v(in))\nrfirst first 0 1k\n\
        bnested nested 0 v=1meg*sdt(sdt(v(in)))\nrnested nested 0 1k\n\
        bcurrent sink 0 i=.01*sdt(v(in))+.02*sdt(v(in))\nrsink sink 0 1k\n\
        .tran 10u 1m\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    assert!(
        engine
            .preflight_transient_checkpoint(&netlist)
            .unwrap()
            .is_resumable()
    );
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 400e-6, 10e-6)
        .unwrap();
    assert!(checkpoint.capability().is_resumable());
    (engine, netlist, checkpoint)
}

#[test]
fn behavioral_checkpoint_restores_nested_and_independent_integrals() {
    let (engine, netlist, checkpoint) = fixture();
    let (reference, reference_final) = engine
        .run_tran_resume(&netlist, &checkpoint, 1e-3, 10e-6)
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let loaded =
            TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
        assert_eq!(loaded, checkpoint);
        let (result, final_state) = engine
            .run_tran_resume(&netlist, &loaded, 1e-3, 10e-6)
            .unwrap();
        assert_eq!(final_state, reference_final);
        assert_eq!(result.time, reference.time);
        assert_eq!(result.voltages, reference.voltages);
        for (name, expected) in [("first", 1.0), ("nested", 0.5), ("sink", -0.03)] {
            let node = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            let actual = *result.voltages[node].last().unwrap();
            assert!(
                (actual - expected).abs() < 1e-8,
                "{name}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn behavioral_checkpoint_rejects_corrupt_or_mismatched_operator_history() {
    let (engine, netlist, checkpoint) = fixture();
    let text = checkpoint.to_text();
    assert!(text.contains("behavioral_states 3\n"));
    let row = text
        .lines()
        .find(|line| line.starts_with("behavioral_sdt "))
        .unwrap();
    let fields: Vec<_> = row.split_whitespace().collect();
    for replacement in [
        format!("behavioral_sdt NaN {} {}", fields[2], fields[3]),
        format!("behavioral_sdt 0 {} {}", fields[2], fields[3]),
        format!("{row} extra"),
    ] {
        assert!(TransientCheckpoint::from_text(&text.replacen(row, &replacement, 1)).is_err());
    }
    let source = text
        .lines()
        .find(|line| line.starts_with("behavioral_state V "))
        .unwrap();
    let fields: Vec<_> = source.split_whitespace().collect();
    let mismatched = text.replacen(
        source,
        &format!("behavioral_state V missing_source {}", fields[3]),
        1,
    );
    let wrong = TransientCheckpoint::from_text(&mismatched).unwrap();
    let error = engine
        .run_tran_resume(&netlist, &wrong, 1e-3, 10e-6)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("behavioral SDT checkpoint identity"),
        "{error}"
    );
    let huge = text.replacen(
        source,
        &format!("behavioral_state V {} {}", fields[2], usize::MAX),
        1,
    );
    assert!(TransientCheckpoint::from_text(&huge).is_err());
}

#[test]
fn behavioral_checkpoint_legacy_state_remains_unknown() {
    let (engine, netlist, checkpoint) = fixture();
    let text = checkpoint.to_text();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("RSPICE-CHECKPOINT 52"));
    let mut old = "RSPICE-CHECKPOINT 49\n".to_owned();
    while let Some(line) = lines.next() {
        if line.starts_with("accepted_bsim3_") {
            continue;
        }
        if line.starts_with("capacitor_sdt_state_available ") || line == "capacitor_sdt_states 0" {
            continue;
        }
        if line.starts_with("behavioral_state_available ") {
            continue;
        }
        if line.starts_with("behavioral_states ") {
            let count = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            for _ in 0..count {
                let count = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(3)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                for _ in 0..count {
                    lines.next().unwrap();
                }
            }
            continue;
        }
        old.push_str(line);
        old.push('\n');
    }
    let legacy = TransientCheckpoint::from_text(&old).unwrap();
    let error = engine
        .run_tran_resume(&netlist, &legacy, 1e-3, 10e-6)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("does not contain behavioral SDT history"),
        "{error}"
    );
}
