//! Full lossless-line delay state in the shooting map and transient restart.
use num_complex::Complex64;
use rspice_core::Netlist;
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
use std::f64::consts::TAU;

#[test]
fn shooting_delay_preserves_ideal_repeating_pwl_steps() {
    for (dialect, delay, source) in [
        (
            SpiceDialect::Ngspice,
            0.1875,
            "VIN in 0 PWL(0 0 .375 0 .375 1 .625 1 .625 0 1 0) R=0",
        ),
        (
            SpiceDialect::Xyce,
            0.1875,
            "VIN in 0 PWL(0 0 .375 0 .375 1 .625 1 .625 0 1 0) R=0",
        ),
        (
            SpiceDialect::Xyce,
            1.1875,
            "VIN in 0 PWL(0 0 .375 0 .375 1 .625 1 .625 0 1 0) R=0",
        ),
        (
            SpiceDialect::Ngspice,
            1.1875,
            "IIN 0 in PWL(0 0 .375 0 .375 .01 .625 .01 .625 0 1 0) R=0",
        ),
        (
            SpiceDialect::Xyce,
            2.1875,
            "VIN in 0 PWL(0 0 .375 0 .375 1 .625 1 .625 0 1 0) R=0",
        ),
    ] {
        let deck = Netlist::parse(&format!("Ideal PWL delay\n{source}\nRS in near 50\nT1 near 0 far 0 Z0=50 TD={delay}\nRL far 0 50\n.save all\n.end\n")).unwrap();
        let mut simulation = SimulationConfig::default();
        simulation.spice_dialect = dialect;
        simulation.resource_limits.max_result_values = 2_000_000;
        let engine = Engine::new(simulation);
        let (pss, state) = engine
            .run_pss_with_continuation_state(
                &deck,
                PssConfig::new(1.0)
                    .with_harmonics(1)
                    .with_points_per_period(16)
                    .with_tstab_periods(0),
            )
            .unwrap();
        assert_eq!(state.time_origin(), 0.0);
        assert!(pss.is_stable);
        let far = pss
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("far"))
            .unwrap();
        let expected = |time: f64| {
            if (0.375..0.625).contains(&(time - delay).rem_euclid(1.0)) {
                0.5
            } else {
                0.0
            }
        };
        for (&time, &actual) in pss
            .result
            .time
            .iter()
            .zip(&pss.result.waveforms[far].values)
        {
            assert!(
                (actual - expected(time)).abs() < 1e-6,
                "TD={delay} t={time:.17e}: {actual} vs {}",
                expected(time)
            );
        }
        {
            let (continued, checkpoint) = engine
                .run_tran_from_pss_state(&deck, &state, 0.7, 0.001)
                .unwrap();
            let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&deck, &checkpoint, 2.0 * delay + 1.0, 0.001)
                .unwrap();
            for transient in [&continued, &resumed] {
                let far = transient
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("far"))
                    .unwrap();
                for (&time, &actual) in transient.time.iter().zip(&transient.voltages[far]) {
                    assert!(
                        (actual - expected(time)).abs() < 1e-6,
                        "continued {dialect:?} TD={delay} t={time:.17e}: {actual} vs {}",
                        expected(time)
                    );
                }
            }
        }
    }
}

#[test]
fn shooting_resolves_reflected_pulses_through_cascaded_delay_lines() {
    for cascade in [false, true] {
        let period = 1e-6;
        let delay = if cascade { 220e-9 } else { 137e-9 };
        let pulse = |time: f64| {
            let phase = time.rem_euclid(period);
            if phase < 710e-9 {
                0.0
            } else if phase < 715e-9 {
                (phase - 710e-9) / 5e-9
            } else if phase < 735e-9 {
                1.0
            } else if phase < 740e-9 {
                (740e-9 - phase) / 5e-9
            } else {
                0.0
            }
        };
        let lines = if cascade {
            "T1 near 0 middle 0 Z0=50 TD=137n\nT2 middle 0 far 0 Z0=50 TD=83n"
        } else {
            "T1 near 0 far 0 Z0=50 TD=137n"
        };
        let deck = Netlist::parse(&format!("Reflected pulse\nVIN in 0 PULSE(0 1 710n 5n 5n 20n 1u)\nRS in near 75\n{lines}\nRL far 0 100\n.save all\n.end\n")).unwrap();
        let mut simulation = SimulationConfig::default();
        simulation.resource_limits.max_result_values = 8_000_000;
        let engine = Engine::new(simulation);
        let pss = engine
            .run_pss(
                &deck,
                PssConfig::new(1e6)
                    .with_harmonics(1)
                    .with_points_per_period(16)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-8),
            )
            .unwrap();
        assert!(pss.is_stable);
        for (name, distance) in [("near", 0.0), ("middle", 137e-9), ("far", delay)] {
            if name == "middle" && !cascade {
                continue;
            }
            let index = pss
                .result
                .node_names
                .iter()
                .position(|entry| entry.eq_ignore_ascii_case(name))
                .unwrap();
            for (&time, &actual) in pss
                .result
                .time
                .iter()
                .zip(&pss.result.waveforms[index].values)
            {
                let expected: f64 = (0..12)
                    .map(|echo| {
                        let round_trip = 2.0 * echo as f64 * delay;
                        0.4 * (1.0_f64 / 15.0).powi(echo)
                            * (pulse(time - distance - round_trip)
                                + pulse(time - (2.0 * delay - distance) - round_trip) / 3.0)
                    })
                    .sum();
                assert!(
                    (actual - expected).abs() < 3e-4,
                    "cascade={cascade} {name} t={time}: {actual} vs {expected}"
                );
            }
        }
        // Even a narrow echo must have its edges represented, rather than
        // disappearing between the points used by coarse/fine qualification.
        for echo in 0..=2 {
            for corner in [710e-9, 715e-9, 735e-9, 740e-9] {
                let arrival = (corner + (2 * echo + 1) as f64 * delay).rem_euclid(period);
                assert!(
                    pss.result
                        .time
                        .iter()
                        .any(|time| (*time - arrival).abs() < 32.0 * f64::EPSILON * period),
                    "cascade={cascade}: missing echo {echo} edge {arrival}"
                );
            }
        }
    }
}

#[test]
fn shooting_delay_preserves_pulse_edges_across_the_period_boundary() {
    let period = 1e-6;
    for (delay, dialect) in [
        (137e-9_f64, SpiceDialect::Ngspice),
        (1.137e-6, SpiceDialect::Xyce),
    ] {
        let pulse = |time: f64| {
            let phase = time.rem_euclid(period);
            if phase < 0.71e-6 {
                0.0
            } else if phase < 0.76e-6 {
                (phase - 0.71e-6) / 0.05e-6
            } else if phase < 0.83e-6 {
                1.0
            } else if phase < 0.88e-6 {
                (0.88e-6 - phase) / 0.05e-6
            } else {
                0.0
            }
        };
        let deck = Netlist::parse(&format!(
            "Pulsed delay\nVIN in 0 PULSE(0 1 710n 50n 50n 70n 1u)\nRS in near 50\nT1 near 0 far 0 Z0=50 TD={delay:e}\nRL far 0 50\n.save all\n.end\n"
        )).unwrap();
        let mut simulation = SimulationConfig::default();
        simulation.spice_dialect = dialect;
        simulation.resource_limits.max_result_values = 2_000_000;
        let engine = Engine::new(simulation);
        let (pss, state) = engine
            .run_pss_with_continuation_state(
                &deck,
                PssConfig::new(1.0 / period)
                    .with_harmonics(1)
                    .with_points_per_period(16)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-8),
            )
            .unwrap();
        assert!(
            pss.is_stable,
            "a matched passive line cannot sustain a perturbation"
        );
        for corner in [0.71e-6, 0.76e-6, 0.83e-6, 0.88e-6] {
            let arrival = (corner + delay).rem_euclid(period);
            assert!(
                pss.result
                    .time
                    .iter()
                    .any(|time| (*time - arrival).abs() < 8.0 * f64::EPSILON * period),
                "missing delayed pulse corner at {arrival}"
            );
        }
        let far = pss
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("far"))
            .unwrap();
        for (&time, &actual) in pss
            .result
            .time
            .iter()
            .zip(&pss.result.waveforms[far].values)
        {
            assert!(
                (actual - 0.5 * pulse(time - delay)).abs() < 2e-4,
                "periodic t={time}: {actual} vs {}",
                0.5 * pulse(time - delay)
            );
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&deck, &state, 0.2e-6, 2e-9)
            .unwrap();
        let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 2.5e-6, 2e-9)
            .unwrap();
        for continued in [&continued, &resumed] {
            let far = continued
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("far"))
                .unwrap();
            for (&time, &actual) in continued.time.iter().zip(&continued.voltages[far]) {
                assert!(
                    (actual - 0.5 * pulse(time - delay)).abs() < 2e-4,
                    "continued t={time}: {actual} vs {}",
                    0.5 * pulse(time - delay)
                );
            }
        }
    }
}

#[test]
fn shooting_delay_lines_match_hb_and_continue_across_full_delay_windows() {
    for (delay, dialect) in [
        (137e-9, SpiceDialect::Ngspice),
        (1.375e-6, SpiceDialect::Xyce),
    ] {
        let deck = Netlist::parse(&format!(
            "Delay shooting\nVIN in 0 SIN(0.3 0.8 1meg 0 0 25)\nRS in near 33\nT1 near 0 far 0 Z0=50 TD={delay:e}\nL1 far out 1u\nRL out 0 75\n.save all\n.end\n"
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            ..Default::default()
        });
        let hb = engine
            .run_hb(&deck, HbConfig::new(1e6).with_harmonics(1))
            .unwrap();
        let (pss, state) = engine
            .run_pss_with_continuation_state(
                &deck,
                PssConfig::new(1e6)
                    .with_harmonics(1)
                    .with_points_per_period(16)
                    .with_tstab_periods(0)
                    .with_tolerance(1e-8),
            )
            .unwrap_or_else(|error| panic!("TD={delay}: {error}"));
        assert!(
            pss.monodromy.len() > 1,
            "the delay must own shooting coordinates"
        );
        assert!(
            pss.is_stable,
            "passive terminated line must retain stable Floquet modes"
        );
        let expected = |name: &str, time: f64| {
            let spectrum = hb
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            spectrum
                .coefficients
                .iter()
                .enumerate()
                .map(|(k, c)| (c * Complex64::from_polar(1.0, TAU * 1e6 * k as f64 * time)).re)
                .sum::<f64>()
        };
        for (name, waveform) in pss.result.node_names.iter().zip(&pss.result.waveforms) {
            for (&time, &actual) in pss.result.time.iter().zip(&waveform.values) {
                assert!(
                    (actual - expected(name, time)).abs() < 5e-4,
                    "TD={delay} {name} t={time}: {actual} vs {}",
                    expected(name, time)
                );
            }
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&deck, &state, 0.2e-6, 1e-9)
            .unwrap();
        let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 2.0 * delay + 0.4e-6, 1e-9)
            .unwrap();
        for transient in [&continued, &resumed] {
            for (name, values) in transient.node_names.iter().zip(&transient.voltages) {
                for (&time, &actual) in transient.time.iter().zip(values) {
                    assert!(
                        (actual - expected(name, time)).abs() < 7e-4,
                        "continued TD={delay} {name} t={time}: {actual} vs {}",
                        expected(name, time)
                    );
                }
            }
        }
    }
}
