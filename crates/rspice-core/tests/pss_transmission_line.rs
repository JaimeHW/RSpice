//! Full lossless-line delay state in the shooting map and transient restart.
use num_complex::Complex64;
use rspice_core::Netlist;
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
use std::f64::consts::TAU;

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
