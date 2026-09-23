//! Exact period-boundary events, physical storage and transient continuation.
use rspice_core::Netlist;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};

#[test]
fn pss_origin_jump_retains_charge_and_delayed_limits_through_resume() {
    for (dialect, delay, capacitor, seam) in [
        (SpiceDialect::Ngspice, 0.1875_f64, false, false),
        (SpiceDialect::Xyce, 0.1875, false, false),
        (SpiceDialect::Xyce, 0.5, false, false),
        (SpiceDialect::Xyce, 1.1875, false, false),
        (SpiceDialect::Ngspice, 2.1875, false, false),
        (SpiceDialect::Ngspice, 0.1875, true, false),
        (SpiceDialect::Xyce, 1.1875, false, true),
    ] {
        let storage = if capacitor { "C1 near 0 .01\n" } else { "" };
        let points = if seam {
            "0 1 .5 1 .5 0 1 0"
        } else {
            "0 0 0 1 .5 1 .5 0 1 0"
        };
        let deck = Netlist::parse(&format!(
            "Origin jump\nVIN in 0 PWL({points}) R=0\nRS in near 50\nT1 near 0 far 0 Z0=50 TD={delay}\nRL far 0 50\n{storage}.save all\n.end\n"
        )).unwrap();
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.resource_limits.max_result_values = 2_000_000;
        let engine = Engine::new(config);
        let (pss, state) = engine
            .run_pss_with_continuation_state(
                &deck,
                PssConfig::new(1.0)
                    .with_harmonics(1)
                    .with_points_per_period(16)
                    .with_tstab_periods(
                        if dialect == SpiceDialect::Ngspice && delay == 0.1875 && !capacitor {
                            2
                        } else {
                            0
                        },
                    ),
            )
            .unwrap_or_else(|error| panic!("{dialect:?}, TD={delay}, C={capacitor}: {error}"));
        assert!(pss.is_stable);
        let expected = |phase: f64| {
            if !capacitor {
                return if phase < 0.5 { 0.5 } else { 0.0 };
            }
            // C sees RS || Z0: tau = .01 * 25 = .25 seconds.
            let decay = (-2.0_f64).exp();
            let high = 0.5 / (1.0 + decay);
            let low = high * decay;
            if phase < 0.5 {
                0.5 + (low - 0.5) * (-phase / 0.25).exp()
            } else {
                high * (-(phase - 0.5) / 0.25).exp()
            }
        };
        let tolerance = if capacitor { 5e-4 } else { 1e-6 };
        for (name, shift) in [("near", 0.0), ("far", delay.rem_euclid(1.0))] {
            let index = pss
                .result
                .node_names
                .iter()
                .position(|node| node.eq_ignore_ascii_case(name))
                .unwrap();
            for (&time, &value) in pss
                .result
                .time
                .iter()
                .zip(&pss.result.waveforms[index].values)
            {
                let reference = expected((time.rem_euclid(1.0) - shift).rem_euclid(1.0));
                assert!(
                    (value - reference).abs() < tolerance,
                    "{dialect:?}, TD={delay}, C={capacitor}: PSS {name} at {time:.17e}: {value} vs {reference}"
                );
            }
        }
        let (first, checkpoint) = engine
            .run_tran_from_pss_state(&deck, &state, 0.7, 0.005)
            .unwrap();
        let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &checkpoint, 2.0 * delay + 1.0, 0.005)
            .unwrap();
        for transient in [&first, &resumed] {
            for (name, shift) in [("near", 0.0), ("far", delay.rem_euclid(1.0))] {
                let values = transient.try_voltage_waveform_named(name).unwrap();
                for (&time, &value) in transient.time.iter().zip(values) {
                    let reference = expected((time.rem_euclid(1.0) - shift).rem_euclid(1.0));
                    assert!(
                        (value - reference).abs() < tolerance,
                        "{dialect:?}, TD={delay}, C={capacitor}: resumed {name} at {time:.17e}: {value} vs {reference}"
                    );
                }
            }
        }
    }
}
