//! Behavioral integration coordinates in the shooting period map.
use num_complex::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, TransientCheckpoint, TransientCheckpointEncoding};
use rspice_core::netlist::Netlist;

const F0: f64 = 1000.0;
const RATE: f64 = 1000.0;

fn config() -> PssConfig {
    let mut config = PssConfig::new(F0)
        .with_points_per_period(128)
        .with_tstab_periods(0)
        .with_tolerance(1e-9);
    config.abstol = 1e-12;
    config
}

fn circuit() -> Netlist {
    Netlist::parse(
        "Integral feedback\nvin in 0 sin(0 1 1k)\n\
        bvol out 0 v=1k*sdt(v(in)-v(out))\nrout out 0 1k\n\
        bnested nested 0 v=1k*sdt(v(in)-v(nested)-1k*sdt(v(nested)))\nrnested nested 0 1k\n\
        bcur current 0 i=-sdt(v(in)-v(current))\nrcurrent current 0 1k\n.end\n",
    )
    .unwrap()
}

fn expected(name: &str, time: f64) -> f64 {
    let s = Complex64::new(0.0, std::f64::consts::TAU * F0);
    let transfer = if name == "nested" {
        RATE * s / (s * s + RATE * s + RATE * RATE)
    } else {
        RATE / (RATE + s)
    };
    (transfer * Complex64::from_polar(1.0, s.im * time)).im
}

#[test]
fn xyce_pss_frequency_context_matches_transient_and_retains_integral_history() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;
    let netlist = Netlist::parse_with_options(
        "Xyce shooting frequency\n.PARAM RUNTIME_R={2k+FREQ}\nvin in 0 sin(0 1 1k)\n\
         bv out 0 v=1k*sdt((1+FREQ/1k)*v(in)-v(out))\nrout out 0 1k\n\
         bi 0 current i=.001*(1+FREQ/1k)*v(out)\nri current 0 {RUNTIME_R}\n.end\n",
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let (analysis, state) = engine
        .run_pss_with_continuation_state(&netlist, config())
        .unwrap();
    for (name, gain) in [("out", 1.0), ("current", 2.0)] {
        let row = analysis
            .result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        for (&time, &actual) in analysis
            .result
            .time
            .iter()
            .zip(&analysis.result.waveforms[row].values)
        {
            assert!(
                (actual - gain * expected("out", time)).abs() < gain * 2e-4,
                "{name} at {time}: {actual}"
            );
        }
    }
    assert_eq!(analysis.monodromy.len(), 1);
    assert!((analysis.monodromy[0][0] - (-RATE / F0).exp()).abs() < 2e-4);
    let (continued, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 2e-4, 2e-6)
        .unwrap();
    for (name, gain) in [("out", 1.0), ("current", 2.0)] {
        let wave = continued.try_voltage_waveform_named(name).unwrap();
        for (&time, &actual) in continued.time.iter().zip(wave) {
            assert!(
                (actual - gain * expected("out", time)).abs() < gain * 3e-4,
                "continued {name} at {time}: {actual}"
            );
        }
    }
}

#[test]
fn pss_behavioral_integrals_retain_independent_nested_and_current_states() {
    let engine = Engine::default();
    let netlist = circuit();
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, config(), &NoAbort)
        .unwrap();
    assert_eq!(point.shooting_state().len(), 4);
    assert_eq!(point.shooting_state_basis().len(), 4);
    assert!(
        point
            .shooting_state_basis()
            .iter()
            .all(|name| name.starts_with("B:"))
    );
    assert_eq!(point.analysis().monodromy.len(), 4);
    let result = &point.analysis().result;
    for name in ["out", "nested", "current"] {
        let index = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[index].values) {
            let truth = expected(name, time);
            assert!(
                (actual - truth).abs() < 2e-4,
                "{name} at {time}: {actual} vs {truth}"
            );
        }
        let wave = &result.waveforms[index].values;
        assert!((wave[0] - wave.last().unwrap()).abs() < 1e-7);
    }
    // Two first-order feedback loops have multiplier exp(-RATE/F0).
    let expected = (-RATE / F0).exp();
    let diagonal: Vec<_> = (0..4)
        .map(|index| point.analysis().monodromy[index][index])
        .collect();
    assert!(
        diagonal
            .iter()
            .filter(|&&value| (value - expected).abs() < 2e-4)
            .count()
            >= 2,
        "{diagonal:?}"
    );
}

#[test]
fn pss_behavioral_integrals_continue_through_transient_checkpoint() {
    let engine = Engine::default();
    let netlist = circuit();
    let (_, state) = engine
        .run_pss_with_continuation_state(&netlist, config())
        .unwrap();
    let (continued, checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 1e-3, 2e-6)
        .unwrap();
    for name in ["out", "nested", "current"] {
        let wave = continued.try_voltage_waveform_named(name).unwrap();
        for (&time, &actual) in continued.time.iter().zip(wave) {
            let truth = expected(name, time);
            assert!(
                (actual - truth).abs() < 3e-4,
                "{name} at {time}: {actual} vs {truth}"
            );
        }
    }
    let loaded = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(loaded, checkpoint);
    let stop = checkpoint.time + 1e-4;
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &loaded, stop, 2e-6)
        .unwrap();
    let wave = resumed.try_voltage_waveform_named("out").unwrap();
    for (&time, &actual) in resumed.time.iter().zip(wave) {
        assert!((actual - expected("out", time)).abs() < 3e-4);
    }
}

#[test]
fn pss_behavioral_integrals_require_periodic_input_equations_and_state_closure() {
    for expression in ["sdt(time)", "sdt(sin(2*pi*1500*time))", "sdt(frequency)"] {
        let netlist = Netlist::parse(&format!(
            "Nonperiodic integral\nb1 out 0 v={expression}\nr1 out 0 1k\n.end"
        ))
        .unwrap();
        let error = Engine::default()
            .run_pss(&netlist, config())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("no structural certificate"),
            "{expression}: {error}"
        );
    }
    for rate in ["1", "-1e-30"] {
        let ramp = Netlist::parse(&format!(
            "No periodic integral\nb1 out 0 v=sdt({rate})\nr1 out 0 1k\n.end"
        ))
        .unwrap();
        let error = Engine::default()
            .run_pss(&ramp, config())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("B1") && error.contains("cannot be periodic"),
            "{error}"
        );
    }
}

#[test]
fn pss_behavioral_integrals_share_charge_and_winding_states_after_stabilization() {
    let netlist = Netlist::parse(
        "Mixed periodic storage\nvin in 0 sin(0 1 1k)\n\
        bvol out 0 v=1k*sdt(v(in)-v(out))\nrout out 0 1k\n\
        cload out 0 1n\nrfilter out filtered 1k\ncfilter filtered 0 1u\n\
        rl in winding 1k\nl1 winding 0 1\n.end\n",
    )
    .unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(&netlist, config().with_tstab_periods(2), &NoAbort)
        .unwrap();
    // The parallel capacitor voltage is constrained by the integral source;
    // it must not introduce a redundant, independently perturbed coordinate.
    assert_eq!(point.shooting_state().len(), 3);
    assert!(
        point
            .shooting_state_basis()
            .iter()
            .any(|name| name.starts_with("B:"))
    );
    assert!(
        point
            .shooting_state_basis()
            .iter()
            .any(|name| name.starts_with("L:"))
    );
    let result = &point.analysis().result;
    let h = RATE / Complex64::new(RATE, std::f64::consts::TAU * F0);
    for (name, transfer) in [("out", h), ("filtered", h * h)] {
        let node = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        for (&time, &actual) in result.time.iter().zip(&result.waveforms[node].values) {
            let truth =
                (transfer * Complex64::from_polar(1.0, std::f64::consts::TAU * F0 * time)).im;
            assert!(
                (actual - truth).abs() < 2e-4,
                "{name} at {time}: {actual} vs {truth}"
            );
        }
    }
}

#[test]
fn pss_behavioral_integrals_support_autonomous_period_and_retained_state() {
    let netlist = Netlist::parse(
        "Integral oscillator\nbx x 0 v=.1+1k*sdt(v(y))\n\
        by y 0 v=1k*sdt((1-v(x)*v(x))*v(y)-v(x))\n\
        rx x 0 1k\nry y 0 1k\n.end\n",
    )
    .unwrap();
    let config = PssConfig::autonomous()
        .with_period_guess(std::f64::consts::TAU / RATE)
        .with_oscillator_node("x")
        .with_tstab_periods(12)
        .with_points_per_period(128)
        .with_tolerance(1e-8);
    let engine = Engine::default();
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, config.clone(), &NoAbort)
        .unwrap();
    assert_eq!(point.shooting_state().len(), 2);
    assert!(point.analysis().is_stable);
    let result = &point.analysis().result;
    assert!(result.trivial_floquet_multiplier_index.is_some());
    assert!((6.0 / RATE..7.0 / RATE).contains(&point.analysis().period));
    for wave in &result.waveforms {
        assert!((wave.values[0] - wave.values.last().unwrap()).abs() < 1e-5);
    }
    let x = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("x"))
        .unwrap();
    let peak = result.waveforms[x]
        .values
        .iter()
        .copied()
        .fold(0.0_f64, |peak, value| peak.max(value.abs()));
    assert!((1.9..2.1).contains(&peak), "oscillation peak {peak}");
    // Both resistors are shunted by integral voltage sources whose inputs do
    // not sense branch current. Their noise therefore cannot perturb the orbit.
    let noise = engine
        .run_pnoise_oscillator_from_pss_with_abort(&netlist, config, &[1.0], &point, &NoAbort)
        .unwrap();
    assert!(
        noise.diffusion_constant < 1e-35,
        "{}",
        noise.diffusion_constant
    );
}

#[test]
fn pss_behavioral_integral_derivatives_follow_frequency_scaled_orbits() {
    let mut reference: Option<(f64, f64)> = None;
    for rate in [1e3, 1e9] {
        let netlist = Netlist::parse(&format!(
            "Scaled integral oscillator\nbx x 0 v=.1+{rate}*sdt(v(y))\n\
            by y 0 v={rate}*sdt((1-v(x)*v(x))*v(y)-v(x))\n\
            rx x 0 1k\nry y 0 1k\n.end\n",
        ))
        .unwrap();
        let mut config = PssConfig::autonomous()
            .with_period_guess(std::f64::consts::TAU / rate)
            .with_oscillator_node("x")
            .with_tstab_periods(12)
            .with_points_per_period(128)
            .with_tolerance(1e-8);
        // The stored coordinates are integrals in V*s. Keep the same
        // dimensionless absolute tolerance when changing the physical rate.
        config.abstol = 1e-10 / rate;
        let point = Engine::default()
            .run_pss_operating_point_with_abort(&netlist, config, &NoAbort)
            .unwrap_or_else(|error| panic!("rate={rate}: {error}"));
        assert!(point.analysis().is_stable, "rate={rate}");
        let result = &point.analysis().result;
        let x = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("x"))
            .unwrap();
        let peak = result.waveforms[x]
            .values
            .iter()
            .copied()
            .fold(0.0_f64, |peak, value| peak.max(value.abs()));
        let normalized_period = point.analysis().period * rate;
        if let Some((period, amplitude)) = reference {
            assert!(
                (normalized_period / period - 1.0).abs() < 1e-4,
                "rate={rate}: period {normalized_period} vs {period}"
            );
            assert!(
                (peak / amplitude - 1.0).abs() < 1e-4,
                "rate={rate}: amplitude {peak} vs {amplitude}"
            );
        } else {
            reference = Some((normalized_period, peak));
        }
    }
}
