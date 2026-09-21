//! Stateless behavioral expressions across the shared periodic MNA solver.
use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() < 1e-8 * expected.norm().max(1e-3),
        "{actual} vs {expected}"
    );
}

#[test]
fn behavioral_periodic_hb_retains_nonlinear_voltage_and_branch_controlled_current() {
    let netlist = Netlist::parse(
        "Nonlinear behavioral harmonics\n\
        v1 drive 0 sin(0 1 1k)\nbvol out 0 v=v(drive)*v(drive)\nrload out 0 1k\n\
        bcur sink 0 i=0.002*v(drive)*v(drive)+i(bvol)\nrsink sink 0 1k\n.end\n",
    )
    .unwrap();
    for krylov in [false, true] {
        let mut config = HbConfig::new(1e3).with_harmonics(3).with_tolerance(1e-9);
        config.use_krylov = krylov;
        let result = Engine::default().run_hb(&netlist, config).unwrap();
        assert!(result.converged);
        for (node, sign) in [("out", 1.0), ("sink", -1.0)] {
            let spectrum = &result
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(node))
                .unwrap()
                .coefficients;
            close(spectrum[0], Complex64::new(sign * 0.5, 0.0));
            close(spectrum[1], Complex64::ZERO);
            close(spectrum[2], Complex64::new(-sign * 0.5, 0.0));
        }
        let current = &result
            .device_currents
            .iter()
            .find(|row| row.probe.eq_ignore_ascii_case("I(bcur)"))
            .unwrap()
            .coefficients;
        close(current[0], Complex64::new(0.5e-3, 0.0));
        close(current[2], Complex64::new(-0.5e-3, 0.0));
    }
}

#[test]
fn behavioral_periodic_conversion_and_noise_use_the_modulated_expression_jacobian() {
    let netlist = Netlist::parse(
        "Behavioral mixer\n\
        vdrive ctrl 0 sin(0 1 1k)\nrnoise noise 0 1k\nitest 0 noise dc 0 ac 1\n\
        bvol out 0 v=(1+v(ctrl))*v(noise)\nrload out 0 1k noisy=0\n\
        bcur sink 0 i=i(bvol)\nrsink sink 0 1k noisy=0\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let pac = engine
        .run_pac(
            &netlist,
            PacConfig::new()
                .with_fundamental(1e3)
                .with_sweep(10.0, 10.0, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("itest")
                .with_output_node("sink"),
        )
        .unwrap();
    for (sideband, expected) in [
        (-1, Complex64::new(0.0, 500.0)),
        (0, Complex64::new(1000.0, 0.0)),
        (1, Complex64::new(0.0, -500.0)),
    ] {
        close(
            pac.result.conversion_matrix.get(0, sideband, 0).unwrap(),
            expected,
        );
    }
    let noise = engine
        .run_pnoise(&netlist, 1e3, &[10.0], "sink", None, Some("itest"), 1)
        .unwrap();
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * 1.5;
    assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-8);
}

#[test]
fn behavioral_periodic_rejects_unrepresented_clocks_and_integrals() {
    for (expression, reason) in [
        ("sin(2*pi*1.5k*time)", "not certified periodic"),
        ("sdt(v(out))", "accepted-step memory"),
        ("frequency*v(out)", "frequency-dependent equations"),
    ] {
        let netlist = Netlist::parse(&format!(
            "Behavioral state boundary\nb1 out 0 i={expression}\nr1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
            .expect_err("unrepresented time/frequency/state must not be sampled at zero")
            .to_string();
        assert!(error.contains(reason), "{error}");
    }
}

#[test]
fn behavioral_explicit_time_hb_and_retained_pss_share_modulation_and_current_outputs() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::PssConfig;
    let netlist = Netlist::parse(
        "Behavioral clocked mixer\n\
        rnoise noise 0 1k\nitest 0 noise dc 1m\n\
        bvol out 0 v=(1+sin(2*pi*1k*time))*v(noise)\nrload out 0 1k noisy=0\n\
        bcur sink 0 i=i(bvol)+0.001*cos(2*pi*3k*time)\nrsink sink 0 1k noisy=0\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(3))
        .unwrap();
    let sink = &hb
        .result
        .spectral_voltages
        .iter()
        .find(|row| row.node_name.eq_ignore_ascii_case("sink"))
        .unwrap()
        .coefficients;
    for (harmonic, expected) in [
        (0, Complex64::ONE),
        (1, Complex64::new(0.0, -1.0)),
        (3, Complex64::new(-1.0, 0.0)),
    ] {
        close(sink[harmonic], expected);
        let current = &hb
            .device_currents
            .iter()
            .find(|row| row.probe.eq_ignore_ascii_case("I(bcur)"))
            .unwrap()
            .coefficients;
        close(current[harmonic], -expected * 1e-3);
    }
    let point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1e3)
                .with_harmonics(8)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    let config = PacConfig::new()
        .with_fundamental(1e3)
        .with_sweep(10.0, 10.0, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("itest")
        .with_output_node("sink");
    let pac = engine
        .run_pac_from_pss_with_abort(&netlist, config, &point, &NoAbort)
        .unwrap();
    for (sideband, expected) in [
        (-1, Complex64::new(0.0, 500.0)),
        (0, Complex64::new(1000.0, 0.0)),
        (1, Complex64::new(0.0, -500.0)),
    ] {
        close(
            pac.result.conversion_matrix.get(0, sideband, 0).unwrap(),
            expected,
        );
    }
    for noise in [
        engine
            .run_pnoise(&netlist, 1e3, &[10.0], "sink", None, Some("itest"), 1)
            .unwrap(),
        engine
            .run_pnoise_from_pss_with_abort(
                &netlist,
                &[10.0],
                "sink",
                None,
                Some("itest"),
                1,
                &point,
                &NoAbort,
            )
            .unwrap(),
    ] {
        let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * 1.5;
        assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-8);
    }
}

#[test]
fn behavioral_explicit_time_respects_configured_grid_and_diagnoses_clock_aliasing() {
    let pulse = Netlist::parse(
        "Narrow behavioral pulse\n\
        b1 out 0 v=spice_pulse(0,1,0,1u,1u,1u,1m)\nr1 out 0 1k\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let error = engine
        .run_hb(
            &pulse,
            HbConfig::new(1e3)
                .with_harmonics(1)
                .with_collocation_points(5),
        )
        .expect_err("a five-point grid cannot resolve a one-microsecond feature")
        .to_string();
    assert!(error.contains("collocation points"), "{error}");
    let result = engine
        .run_hb(
            &pulse,
            HbConfig::new(1e3)
                .with_harmonics(1)
                .with_collocation_points(4097)
                .with_tolerance(1e-9),
        )
        .unwrap();
    // Unit-height trapezoid: one microsecond plateau plus two half ramps.
    let mean = result.result.spectral_voltages[0].coefficients[0].re;
    assert!((mean - 0.002).abs() < 1e-5, "narrow pulse mean {mean}");
    let alias = Netlist::parse(
        "Unresolved behavioral clock\n\
        b1 out 0 v=sin(2*pi*128k*time)\nr1 out 0 1k\n.end\n",
    )
    .unwrap();
    let error = engine
        .run_hb(&alias, HbConfig::new(1e3).with_harmonics(1))
        .expect_err("a clock that aliases to zero must not silently disappear")
        .to_string();
    assert!(error.contains("128 harmonics"), "{error}");
}

#[test]
fn behavioral_quasiperiodic_mixer_preserves_independent_tone_products() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::QpssConfig;
    let netlist = Netlist::parse(
        "Behavioral two-tone mixer\n\
        v1 a 0 sin(0 1 1k)\nv2 b 0 sin(0 1 1414.213562373095)\n\
        bvol out 0 v=v(a)*v(b)\nrload out 0 1k\n\
        bcur sink 0 i=i(bvol)\nrsink sink 0 1k\n.end\n",
    )
    .unwrap();
    let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![8, 8]);
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
            .unwrap();
    let result = Engine::default().run_qpss(&netlist, config).unwrap();
    for node in ["out", "sink"] {
        let row = result
            .node_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap();
        for (tuple, value) in [([1, 1], -0.25), ([1, -1], 0.25), ([0, 0], 0.0)] {
            close(
                result.spectra()[row][grid.index_of(&tuple).unwrap()],
                Complex64::new(value, 0.0),
            );
        }
    }
}
