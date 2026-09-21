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

#[test]
fn behavioral_quasiperiodic_clocks_feed_retained_conversion_and_noise() {
    use rspice_core::engine::*;
    let netlist = Netlist::parse(
        "Independent behavioral clocks\n\
        rnoise noise 0 1k\nitest 0 noise dc 1m\n\
        bvol out 0 v=(1+sin(2*pi*1k*time)*cos(2*pi*1414.213562373095*time))*v(noise)\n\
        rload out 0 1k noisy=0\nbcur sink 0 i=i(bvol)\nrsink sink 0 1k noisy=0\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod;
    for method in [
        QuasiPeriodicLinearMethod::Direct,
        QuasiPeriodicLinearMethod::Krylov,
    ] {
        let linear = rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearConfig {
            method,
            ..Default::default()
        };
        let point = engine
            .run_qpss(
                &netlist,
                QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]),
            )
            .unwrap();
        // Serialization preserves the authored independent clocks and their basis.
        let point: QpssOperatingPoint =
            serde_json::from_str(&serde_json::to_string(&point).unwrap()).unwrap();
        let grid = engine
            .validate_qpss_operating_point_with_abort(
                &netlist,
                &point,
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case("sink"))
            .unwrap();
        for (tuple, value) in [
            ([0, 0], Complex64::ONE),
            ([1, 1], Complex64::new(0.0, -0.25)),
            ([-1, 1], Complex64::new(0.0, 0.25)),
        ] {
            close(point.spectra()[row][grid.index_of(&tuple).unwrap()], value);
        }
        let pac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![10.0],
                    input_source: "itest".into(),
                    input_lattice: vec![0, 0],
                    output_node: "sink".into(),
                    output_ref: "0".into(),
                    output_lattice: vec![1, 1],
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: rspice_core::analysis::quasi_periodic::QuasiPeriodicAcConfig {
                        linear: linear.clone(),
                        ..Default::default()
                    },
                },
                &point,
            )
            .unwrap();
        close(pac.output_transfer[0], Complex64::new(0.0, -250.0));
        let xf = engine
            .run_qpxf_from_qpss(
                &netlist,
                QpxfRequest {
                    frequencies_hz: vec![10.0],
                    frequency_axis: QpxfFrequencyAxis::Offset,
                    input_sources: QpxfSources::Named(vec!["itest".into()]),
                    input_lattices: QpxfInputLattices::Explicit(vec![vec![0, 0]]),
                    output: QpxfOutput::Voltage {
                        positive: "sink".into(),
                        negative: "0".into(),
                    },
                    output_lattice: vec![1, 1],
                    linear: linear.clone(),
                    group_delay: false,
                    group_delay_magnitude_floor: 0.0,
                },
                &point,
            )
            .unwrap();
        close(xf.transfers[0].values[0], Complex64::new(0.0, -250.0));
        let noise = engine
            .run_qpnoise_from_qpss(
                &netlist,
                QpnoiseRequest {
                    frequencies_hz: vec![10.0],
                    frequency_axis: QpnoiseFrequencyAxis::Offset,
                    outputs: vec![QpnoiseOutput {
                        observation: QpnoiseObservation::Voltage {
                            positive: "sink".into(),
                            negative: "0".into(),
                        },
                        lattice: vec![0, 0],
                    }],
                    input: Some(QpnoiseInput {
                        source: "itest".into(),
                        lattice: vec![0, 0],
                    }),
                    input_lattices: QpnoiseLattices::AllRetained,
                    sources: QpnoiseSources::All,
                    integration: None,
                    contributor_ranking: false,
                    noise_figure: None,
                    linear: linear.clone(),
                },
                &point,
            )
            .unwrap();
        let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * 1.25;
        assert!((noise.total_covariances[0].values[0].re / expected - 1.0).abs() < 1e-8);
        close(
            noise.outputs[0].input_transfer.as_ref().unwrap()[0],
            Complex64::new(1e3, 0.0),
        );
    }
}

#[test]
fn behavioral_quasiperiodic_waveforms_and_nested_phases_match_independent_coordinates() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::QpssConfig;
    use std::f64::consts::TAU;
    let netlist = Netlist::parse(
        "Independent waveform coordinates\nvc ctrl 0 dc .7\n\
        ba a 0 v=v(ctrl)*spice_sffm(.2,.6,1k,.3,1414.213562373095)+spice_sin(.4,.2,1414.213562373095,120u,0,30)\nra a 0 1k\n\
        bp p 0 v=spice_pulse(0,v(ctrl),125u,100u,100u,400u,1m)\nrp p 0 1k\n\
        bn n 0 v=sin(2*pi*1k*time+sin(2*pi*1414.213562373095*time)+v(ctrl))+cos(2*pi*(1k-1414.213562373095)*time)\nrn n 0 1k\n.end\n",
    ).unwrap();
    let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![64, 32]);
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
            .unwrap();
    let point = Engine::default().run_qpss(&netlist, config).unwrap();
    for node in ["a", "p", "n"] {
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case(node))
            .unwrap();
        for (k, tuple) in grid.indices().iter().enumerate() {
            // Direct two-dimensional Fourier sum of independent analytic
            // waveforms, not an invented common time for incommensurate tones.
            let mut expected = Complex64::ZERO;
            for sample in 0..grid.sample_count() {
                let phase = grid.phases(sample).unwrap();
                let value = match node {
                    "a" => {
                        0.7 * (0.2 + 0.6 * (phase[0] + 0.3 * phase[1].sin()).sin())
                            + 0.4
                            + 0.2
                                * (phase[1] - TAU * 1e3 * std::f64::consts::SQRT_2 * 120e-6
                                    + TAU / 12.0)
                                    .sin()
                    }
                    "p" => {
                        let t = (phase[0] / TAU * 1e-3 - 125e-6).rem_euclid(1e-3);
                        0.7 * if t < 100e-6 {
                            t / 100e-6
                        } else if t < 500e-6 {
                            1.0
                        } else if t < 600e-6 {
                            (600e-6 - t) / 100e-6
                        } else {
                            0.0
                        }
                    }
                    _ => (phase[0] + phase[1].sin() + 0.7).sin() + (phase[0] - phase[1]).cos(),
                };
                expected += Complex64::from_polar(
                    value / grid.sample_count() as f64,
                    -(tuple[0] as f64 * phase[0] + tuple[1] as f64 * phase[1]),
                );
            }
            close(point.spectra()[row][k], expected);
        }
    }
}

#[test]
fn behavioral_quasiperiodic_rejects_unrepresented_or_nonstationary_clocks() {
    use rspice_core::engine::QpssConfig;
    for (expression, reason) in [
        (
            "sin(2*pi*128k*time)",
            "absent from the retained tone lattice",
        ),
        ("sin(2*pi*1k*time*time)", "not affine in time"),
        ("time", "nonperiodic explicit time"),
        ("spice_sin(0,1,1k,0,1)", "not stationary"),
        ("spice_pulse(0,1,0,1u,1u,1u,1m)", "collocation points"),
    ] {
        let netlist = Netlist::parse(&format!(
            "Invalid behavioral torus\nb out 0 v={expression}\nr out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_qpss(
                &netlist,
                QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]),
            )
            .expect_err("unrepresented clock must not be sampled at time zero")
            .to_string();
        assert!(error.contains(reason), "{expression}: {error}");
    }
}

#[test]
fn behavioral_quasiperiodic_signed_resets_preserve_rounding_and_independent_clocks() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::QpssConfig;
    use std::f64::consts::TAU;
    let expressions = [
        "mod(1k*time-.125,1)",
        "(-1414.213562373095*time+.125)%-1",
        "1k*time-floor(1k*time+.25)",
        "ceil(1k*time+.25)-1k*time",
        "(-1k*time+.125)-int(-1k*time+.125)",
        "2k*time-round(2k*time)",
    ];
    let mut deck = "Signed behavioral clocks\n".to_string();
    for (i, expression) in expressions.iter().enumerate() {
        deck.push_str(&format!("b{i} n{i} 0 v={expression}\nr{i} n{i} 0 1k\n"));
    }
    deck.push_str("bcur sink 0 i=1m*mod(1k*time,1)\nrsink sink 0 1k\n.end\n");
    let netlist = Netlist::parse(&deck).unwrap();
    let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![32, 24]);
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
            .unwrap();
    let point = Engine::default().run_qpss(&netlist, config).unwrap();
    for i in 0..=expressions.len() {
        let name = if i == expressions.len() {
            "sink".to_owned()
        } else {
            format!("n{i}")
        };
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case(&name))
            .unwrap();
        for (k, tuple) in grid.indices().iter().enumerate() {
            let mut expected = Complex64::ZERO;
            for sample in 0..grid.sample_count() {
                let phase = grid.phases(sample).unwrap();
                let a = phase[0] / TAU;
                let b = phase[1] / TAU;
                let value = match i {
                    0 => (a - 0.125).rem_euclid(1.0),
                    1 => -((b - 0.125).rem_euclid(1.0)),
                    2 => a - (a + 0.25).floor(),
                    3 => (a + 0.25).ceil() - a,
                    4 => -((a - 0.125).rem_euclid(1.0)),
                    5 => 2.0 * a - (2.0 * a).round_ties_even(),
                    _ => -a,
                };
                expected += Complex64::from_polar(
                    value / grid.sample_count() as f64,
                    -(tuple[0] as f64 * phase[0] + tuple[1] as f64 * phase[1]),
                );
            }
            close(point.spectra()[row][k], expected);
        }
    }
}

#[test]
fn behavioral_quasiperiodic_table_clock_retains_control_phase_derivatives() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::{QpacRequest, QpssConfig};
    use std::f64::consts::TAU;
    // Fixed lookup data, runtime PWL, and state-dependent table amplitudes
    // reach different expression representations but retain the same phase law.
    for waveform in [
        "table(mod(1k*time+v(ctrl),1),0,0,.25,1,.5,0,.75,-1,1,0)",
        "pwl(mod(1k*time+v(ctrl),1),0,0,.25,1,.5,0,.75,-1,1,0)",
        "table(mod(1k*time+v(ctrl),1),0,0,.25,v(amplitude),.5,0,.75,-v(amplitude),1,0)",
    ] {
        let netlist = Netlist::parse(&format!(
            "Table phase control\nrctrl ctrl 0 1k\nitest 0 ctrl dc 137u\nvamp amplitude 0 dc 1\n\
        bvol out 0 v={waveform}*cos(2*pi*1414.213562373095*time)\n\
        rload out 0 1k noisy=0\n.end\n"
        ))
        .unwrap();

        let engine = Engine::default();
        let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
        config.grid.sampling = QuasiPeriodicSampling::Exact(vec![64, 16]);
        let grid =
            QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
                .unwrap();
        let point = engine.run_qpss(&netlist, config).unwrap();
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        let mut expected = Complex64::ZERO;
        let mut expected_derivative = Complex64::ZERO;
        for sample in 0..grid.sample_count() {
            let phase = grid.phases(sample).unwrap();
            // The control offset avoids table corners on the collocation grid.
            let q = (phase[0] / TAU + 0.137).rem_euclid(1.0);
            let (value, slope) = if q < 0.25 {
                (4.0 * q, 4.0)
            } else if q < 0.75 {
                (2.0 - 4.0 * q, -4.0)
            } else {
                (4.0 * q - 4.0, 4.0)
            };
            let weight = Complex64::from_polar(
                phase[1].cos() / grid.sample_count() as f64,
                -phase[0] - phase[1],
            );
            expected += weight * value;
            expected_derivative += weight * slope * 1000.0;
        }
        close(
            point.spectra()[row][grid.index_of(&[1, 1]).unwrap()],
            expected,
        );
        let pac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![10.0],
                    input_source: "itest".into(),
                    input_lattice: vec![0, 0],
                    output_node: "out".into(),
                    output_ref: "0".into(),
                    output_lattice: vec![1, 1],
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: Default::default(),
                },
                &point,
            )
            .unwrap();
        close(pac.output_transfer[0], expected_derivative);
    }
}

#[test]
fn behavioral_quasiperiodic_clock_resolution_uses_authored_grid() {
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling;
    use rspice_core::engine::QpssConfig;
    let engine = Engine::default();
    let config = || QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    for (expression, reason) in [
        ("mod(128k*time,1)", "absent from the retained tone lattice"),
        ("floor(1k*time)", "nonperiodic explicit time"),
        (
            "1k*time-round(1k*time)",
            "absent from the retained tone lattice",
        ),
        ("mod(time,0)", "nonzero modulus"),
    ] {
        let netlist = Netlist::parse(&format!(
            "Clock configuration\nb out 0 v={expression}\nr out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = engine.run_qpss(&netlist, config()).unwrap_err().to_string();
        assert!(error.contains(reason), "{expression}: {error}");
    }
    let netlist = Netlist::parse("Narrow periodic table\nb out 0 v=table(time%1m,0,0,250u,0,253.90625u,1,257.8125u,0,1m,0)\nr out 0 1k\n.end\n").unwrap();
    let error = engine.run_qpss(&netlist, config()).unwrap_err().to_string();
    assert!(
        error.contains("table features") && error.contains("collocation points"),
        "{error}"
    );
    let mut fine = config();
    fine.grid.sampling = QuasiPeriodicSampling::Exact(vec![512, 8]);
    let point = engine.run_qpss(&netlist, fine).unwrap();
    let dc = point.spectra()[0].len() / 2;
    close(point.spectra()[0][dc], Complex64::new(1.0 / 256.0, 0.0));
}
