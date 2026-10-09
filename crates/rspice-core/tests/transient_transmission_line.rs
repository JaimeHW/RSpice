use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
use rspice_core::netlist::Netlist;

#[test]
fn finite_behavioral_current_controls_drive_the_analytic_delayed_rl_wave() {
    use rspice_core::engine::{TransientCheckpointEncoding, TransientStartupMode};
    let omega = std::f64::consts::TAU * 1e9;
    let reactance = omega * 1e-6;
    let current = |time: f64| {
        let time = time.max(0.0);
        1e-3 * (1000.0 * (omega * time).sin() - reactance * (omega * time).cos()
            + reactance * (-time / 1e-9).exp())
            / (1e6 + reactance * reactance)
    };
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        // Independent refinement bounds each dialect's event-restart error
        // below the same 10 pA analytic-current gate.
        let max_step = if dialect == SpiceDialect::BestAvailable {
            0.125e-12
        } else {
            0.5e-12
        };
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for control in ["L1", "R1"] {
            let source = Netlist::parse(&format!("finite-current line drive\nVD drive 0 DC 0 SIN(0 1m 1G)\nR1 drive coil 1k\nL1 coil 0 1u\nB1 0 near I={{I({control})^1}}\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\n.options device zeroresistancetol=1k\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16\n.save v(far) i(b1)\n.end\n")).unwrap();
            let (full, checkpoints) = engine
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &source,
                    2.5e-9,
                    max_step,
                    TransientStartupMode::OperatingPoint,
                    &[1.2e-9],
                )
                .unwrap();
            let voltage = full.try_voltage_waveform_named("far").unwrap();
            let forcing = full.try_branch_current_waveform_named("b1").unwrap();
            for ((&time, &voltage), &forcing) in full.time.iter().zip(voltage).zip(forcing) {
                assert!(
                    (voltage - 50.0 * current(time - 1e-9)).abs() < 2e-9,
                    "{dialect:?}/{control}: delayed RL voltage at {time:e}"
                );
                assert!(
                    (forcing - current(time)).abs() < 1e-11,
                    "{dialect:?}/{control}: RL current at {time:e}: {forcing:e} vs {:e}",
                    current(time),
                );
            }
            let checkpoint = TransientCheckpoint::from_bytes(
                &checkpoints[0]
                    .checkpoint
                    .to_bytes(TransientCheckpointEncoding::Packed)
                    .unwrap(),
            )
            .unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&source, &checkpoint, 2.5e-9, max_step)
                .unwrap();
            let seam = full
                .time
                .iter()
                .position(|time| *time == resumed.time[0])
                .unwrap();
            assert_eq!(resumed.time, full.time[seam..]);
            for (actual, expected) in [
                (
                    resumed.try_voltage_waveform_named("far").unwrap(),
                    &voltage[seam..],
                ),
                (
                    resumed.try_branch_current_waveform_named("b1").unwrap(),
                    &forcing[seam..],
                ),
            ] {
                assert_eq!(actual.len(), expected.len());
                for (&actual, &expected) in actual.iter().zip(expected) {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn controlled_current_drive_preserves_line_waves_and_current_observations_on_resume() {
    for (control, name) in [
        ("E1 ctrl 0 input 0 2", "e1"),
        (
            "RC input 0 1\nH1 ctrl 0 RC 2\n.options device zeroresistancetol=1",
            "h1",
        ),
        ("VS input sense 0\nRC sense 0 1k\nH1 ctrl 0 VS 2k", "h1"),
        ("VS sense input 0\nRC sense 0 1k\nH1 ctrl 0 VS -2k", "h1"),
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

#[test]
fn capacitor_ic_current_tracks_a_matched_line_arrival() {
    use rspice_core::numerics::integration::IntegrationMethod;
    let deck = Netlist::parse("Capacitor IC at a line load\nV1 source 0 PWL(0 0 1n 0 1n 1 4n 1)\nRS source near 50\nT1 near 0 far 0 Z0=50 TD=1n\nRL far 0 50\nC1 far 0 10p IC=0\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    for method in [
        IntegrationMethod::Trapezoidal,
        IntegrationMethod::Gear2,
        IntegrationMethod::TrapGear,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        config.integration_method = method;
        config.convergence_config.gmin_target = 0.0;
        config.max_timestep = 0.5e-12;
        config.min_timestep = 0.5e-15;
        let engine = Engine::new(config);
        let (full, saved) = engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &deck,
                3.2e-9,
                0.5e-12,
                rspice_core::engine::TransientStartupMode::OperatingPoint,
                &[2e-9, 2.3e-9],
                &rspice_core::NoAbort,
            )
            .unwrap();
        let voltage = full.try_voltage_waveform_named("far").unwrap();
        let current = full.try_branch_current_waveform_named("c1").unwrap();
        // The incident half-volt step arrives at 2 ns. The 50-ohm line
        // and 50-ohm termination drive C through 25 ohms: tau=.25 ns.
        for (i, &time) in full.time.iter().enumerate() {
            let decay = if time < 2e-9 {
                0.0
            } else {
                (-(time - 2e-9) / 0.25e-9).exp()
            };
            let expected = if time < 2e-9 {
                0.0
            } else {
                0.5 * (1.0 - decay)
            };
            assert!(
                (voltage[i] - expected).abs() < 3e-6,
                "{method:?} at {time:e}: {} != {expected}",
                voltage[i]
            );
            assert!((current[i] - 0.02 * decay).abs() < 1.2e-7);
        }
        let trace = full.current_impulses.as_ref().unwrap().iter().find(|trace| matches!(&trace.owner, rspice_core::CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case("c1"))).unwrap();
        assert!(
            trace.complete && trace.points.is_empty() && trace.derivatives.is_empty(),
            "{trace:?}"
        );
        for saved in saved {
            let checkpoint = TransientCheckpoint::from_bytes(
                &saved
                    .checkpoint
                    .to_bytes(rspice_core::engine::TransientCheckpointEncoding::Packed)
                    .unwrap(),
            )
            .unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&deck, &checkpoint, 3.2e-9, 0.5e-12)
                .unwrap();
            let offset = full
                .time
                .iter()
                .position(|&time| time == checkpoint.time)
                .unwrap();
            assert_eq!(resumed.time, full.time[offset..]);
            for (actual, expected) in resumed
                .voltages
                .iter()
                .zip(&full.voltages)
                .chain(resumed.branch_currents.iter().zip(&full.branch_currents))
            {
                assert_eq!(actual, &expected[offset..]);
            }
        }
    }
}

#[test]
fn prescribed_capacitor_load_preserves_the_analytic_line_arrival() {
    // A matched source absorbs the reflection. After the 1 ns arrival the
    // line is a 1 V Thevenin drive with R=50 ohms. For C=C0*(1+t/T),
    // d(CV)/dt=(1-V)/R gives V=(1-(2/(1+t/T))^11)/1.1.
    let deck = Netlist::parse("Time capacitor on matched line\nV1 input 0 PWL(0 0 0 1 3n 1)\nR1 input near 50\nT1 near 0 out 0 Z0=50 TD=1n\nC1 out 0 C={2p*(1+time/1n)} IC=0\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-27\n.save all\n.end\n").unwrap();
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
    config.max_timestep = 0.25e-12;
    config.min_timestep = 1e-16;
    config.convergence_config.gmin_target = 0.0;
    let engine = Engine::new(config);
    let (result, checkpoints) = engine
        .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
            &deck,
            2.4e-9,
            0.25e-12,
            rspice_core::engine::TransientStartupMode::Uic,
            &[1e-9, 1.6e-9],
            &rspice_core::NoAbort,
        )
        .unwrap();
    let voltage = result.try_voltage_waveform_named("out").unwrap();
    let current = result.try_branch_current_waveform_named("c1").unwrap();
    for (i, &time) in result.time.iter().enumerate() {
        let arrived = time >= 1e-9;
        let expected = if arrived {
            (1.0 - (2.0 / (1.0 + time / 1e-9)).powi(11)) / 1.1
        } else {
            0.0
        };
        assert!(
            (voltage[i] - expected).abs() < 3e-6,
            "t={time:e}: {} != {expected}",
            voltage[i]
        );
        let expected_current = if arrived {
            (1.0 - voltage[i]) / 50.0
        } else {
            0.0
        };
        assert!(
            (current[i] - expected_current).abs() < 1e-10,
            "t={time:e}: {} != {expected_current}",
            current[i]
        );
    }
    for saved in checkpoints {
        let bytes = saved
            .checkpoint
            .to_bytes(rspice_core::engine::TransientCheckpointEncoding::Packed)
            .unwrap();
        let checkpoint = TransientCheckpoint::from_bytes(&bytes).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 2.4e-9, 0.25e-12)
            .unwrap();
        let offset = result
            .time
            .iter()
            .position(|&time| time == checkpoint.time)
            .unwrap();
        assert_eq!(resumed.time, result.time[offset..]);
        for (actual, full) in resumed
            .voltages
            .iter()
            .zip(&result.voltages)
            .chain(resumed.branch_currents.iter().zip(&result.branch_currents))
        {
            assert_eq!(actual, &full[offset..]);
        }
    }
}
