use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
use rspice_core::netlist::Netlist;

#[test]
fn controlled_current_drive_preserves_line_waves_and_current_observations_on_resume() {
    for (control, name) in [
        ("E1 ctrl 0 input 0 2", "e1"),
        (
            "RC input 0 1\nH1 ctrl 0 RC 2\n.options device zeroresistancetol=1",
            "h1",
        ),
    ] {
        let source=Netlist::parse(&format!("controlled matched line\nV1 input 0 SIN(0 .5 1G)\n{control}\nG1 0 near ctrl 0 1m\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16\n.save v(far) i(g1) i({name})\n.end\n")).unwrap();
        for dialect in [
            SpiceDialect::Ngspice,
            SpiceDialect::Xyce,
            SpiceDialect::BestAvailable,
        ] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let full = engine.run_tran(&source, 2.5e-9, 2e-12).unwrap();
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&source, 1.2e-9, 2e-12)
                .unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&source, &checkpoint, 2.5e-9, 2e-12)
                .unwrap();
            for result in [&full, &resumed] {
                let voltage = result.try_voltage_waveform_named("far").unwrap();
                let current = result.try_branch_current_waveform_named("g1").unwrap();
                for (index, &time) in result.time.iter().enumerate() {
                    let phase = std::f64::consts::TAU * 1e9;
                    let wave = 0.05 * (phase * (time - 1e-9).max(0.0)).sin();
                    assert!(
                        (voltage[index] - wave).abs() < 5e-6,
                        "{dialect:?}: far at {time:e}"
                    );
                    assert!((current[index] - 0.001 * (phase * time).sin()).abs() < 1e-12);
                }
                assert!(
                    result
                        .try_branch_current_waveform_named(name)
                        .unwrap()
                        .iter()
                        .all(|&current| current.abs() < 1e-16)
                );
                assert!(
                    result
                        .current_impulses
                        .as_ref()
                        .unwrap()
                        .iter()
                        .all(|trace| trace.complete)
                );
                assert_eq!(result.time.last(), Some(&2.5e-9));
            }
            assert!(
                full.time
                    .iter()
                    .any(|&time| (time - 1e-9).abs() <= 8.0 * f64::EPSILON * 1e-9)
            );
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn smooth_behavioral_drive_preserves_matched_line_delay_and_continuation() {
    let deck = Netlist::parse(
        "behavioral matched line\nBD drive 0 V={sin(2*pi*1e9*time)}\nRS drive near 50\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16\n.save v(far)\n.end\n",
    ).unwrap();
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let result = engine.run_tran(&deck, 2.5e-9, 2e-12).unwrap();
        let (_, checkpoint) = engine.run_tran_checkpointed(&deck, 1.2e-9, 2e-12).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 2.5e-9, 2e-12)
            .unwrap();
        for result in [&result, &resumed] {
            for (&time, &voltage) in result
                .time
                .iter()
                .zip(result.try_voltage_waveform_named("far").unwrap())
            {
                let expected = 0.5 * (std::f64::consts::TAU * 1e9 * (time - 1e-9).max(0.0)).sin();
                // Linear wave-history interpolation at 2 ps contributes at
                // most 1e-5 V for this tone; include a bounded solve margin.
                assert!(
                    (voltage - expected).abs() < 5e-5,
                    "{dialect:?} at {time:e}: {voltage:e} vs {expected:e}"
                );
            }
            assert_eq!(result.time.last(), Some(&2.5e-9));
            assert!(
                result
                    .current_impulses
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|trace| trace.complete)
            );
            if result.time[0] < 1e-9 {
                assert!(
                    result
                        .time
                        .iter()
                        .any(|&time| (time - 1e-9).abs() <= 8.0 * f64::EPSILON * 1e-9),
                    "{dialect:?}: missing first wavefront in {:?}; nearest {:?}",
                    result.time.first(),
                    result
                        .time
                        .iter()
                        .min_by(|a, b| (**a - 1e-9).abs().total_cmp(&(**b - 1e-9).abs()))
                );
            }
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

#[test]
fn loaded_lossless_line_preserves_dc_bias_through_startup_and_resume() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for (near_reference, far_reference, polarity) in
            [(0.0, 0.0, 1.0), (3.0, -2.0, 1.0), (3.0, -2.0, -1.0)]
        {
            let (near_port, far_port) = if polarity > 0.0 {
                ("near nr", "far fr")
            } else {
                ("nr near", "fr far")
            };
            let deck = Netlist::parse(&format!(
                "biased delay line\nVNR nr 0 {near_reference}\nVFR fr 0 {far_reference}\n\
                 VDRIVE drive nr {}\nRS drive near 25\nT1 {near_port} {far_port} Z0=50 TD=1u\n\
                 RL far fr 75\n.save all\n.end\n",
                polarity * 2.0,
            ))
            .unwrap();
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let dc = engine.run_dc_op(&deck).unwrap();
            // The existing DC fallback has a 1 mOhm series resistance. Its
            // small bias error is allowed; a Z0-sized startup droop is not.
            for (node, reference) in [("near", near_reference), ("far", far_reference)] {
                let voltage = polarity * (dc.try_voltage_named(node).unwrap() - reference);
                assert!(
                    (voltage - 1.5).abs() < 2e-5,
                    "{dialect:?}: {node} DC={voltage}"
                );
            }
            let (first, checkpoint) = engine
                .run_tran_checkpointed(&deck, 0.75e-6, 0.125e-6)
                .unwrap();
            let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&deck, &checkpoint, 3.25e-6, 0.125e-6)
                .unwrap();
            for result in [&first, &resumed] {
                for (node, reference) in [("near", near_reference), ("far", far_reference)] {
                    for (&time, &voltage) in result
                        .time
                        .iter()
                        .zip(result.try_voltage_waveform_named(node).unwrap())
                    {
                        let voltage = polarity * (voltage - reference);
                        assert!(
                            (voltage - 1.5).abs() < 2e-5,
                            "{dialect:?}, refs={near_reference}/{far_reference}, polarity={polarity}: {node} at {time:e} = {voltage}"
                        );
                    }
                }
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn native_diode_storage_has_finite_current_at_line_arrival_and_packed_restart() {
    let deck = Netlist::parse("diode line arrival\nV1 s 0 PWL(0 0 .5n 0 .5n 1 2n 1)\nRS s near 50\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\nD1 far 0 dm\n.model dm D(IS=1e-30 CJO=1p VJ=1 M=0)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(far) i(d1)\n.end\n").unwrap();
    let arrival = 0.5e-9 + 1e-9;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let (result, checkpoints) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &deck,
                1.51e-9,
                1e-12,
                rspice_core::engine::TransientStartupMode::OperatingPoint,
                &[arrival],
            )
            .unwrap();
        let index = result
            .time
            .iter()
            .position(|time| *time == arrival)
            .unwrap();
        let v = result.try_voltage_waveform_named("far").unwrap();
        let i = result.try_branch_current_waveform_named("d1").unwrap();
        assert!(
            v[index].abs() < 1e-10,
            "{dialect:?}: junction charge remains continuous"
        );
        assert!(
            (i[index] - 0.02).abs() < 1e-10,
            "{dialect:?}: finite arrival current"
        );
        for (&time, (&voltage, &current)) in result.time[index..]
            .iter()
            .zip(v[index..].iter().zip(&i[index..]))
        {
            let decay = (-(time - arrival) / 25e-12).exp();
            assert!((voltage - 0.5 * (1.0 - decay)).abs() < 3e-3);
            assert!((current - 0.02 * decay).abs() < 1.2e-4);
        }
        let packed = checkpoints[0]
            .checkpoint
            .to_bytes(rspice_core::engine::TransientCheckpointEncoding::Packed)
            .unwrap();
        let checkpoint = TransientCheckpoint::from_bytes(&packed).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 1.51e-9, 1e-12)
            .unwrap();
        assert_eq!(
            resumed.try_branch_current_waveform_named("d1").unwrap()[0].to_bits(),
            i[index].to_bits()
        );
        assert!(
            result
                .current_impulses
                .as_ref()
                .unwrap()
                .iter()
                .all(|trace| trace.complete
                    && trace
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs.abs() < 1e-24))
        );
    }
}
