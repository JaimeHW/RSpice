//! Independent-phase primitives and retained modulation, including truncated mixing.
use super::*;
use crate::analysis::quasi_periodic::{QuasiPeriodicAcConfig, QuasiPeriodicLinearMethod};

#[test]
fn qpss_differential_integrals_keep_independent_common_mode_feedback() {
    let rate = 1e3;
    let f2 = rate * std::f64::consts::SQRT_2;
    let omega = std::f64::consts::TAU * rate;
    let netlist = Netlist::parse(&format!(
        "Differential torus\nVinput p1 n1 SIN(0 1 {rate})\nVsecond p2 n2 SIN(0 .4 {f2})\n\
         Bcommon n1 0 V=.1*tanh(v(out)+v(cube))\nBother n2 0 V=.15*tanh(v(out)-v(cube))\n\
         R1 p1 0 1k\nR2 n1 0 2k\nR3 p2 0 3k\nR4 n2 0 4k\n\
         BV out 0 V={omega}*sdt(.25*v(p1)-.25*v(n1)+.5*v(p2)-.5*v(n2))\nRout out 0 1k\n\
         BC cube 0 V={omega}*sdt((v(p1)-v(n1))^3)\nRc cube 0 1k\n.end\n"
    ))
    .unwrap();
    let engine = Engine::default();
    let point = engine
        .run_qpss(&netlist, QpssConfig::new(vec![rate, f2], vec![3, 1]))
        .unwrap();
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let close = |a: Complex64, b: Complex64| {
        assert!((a - b).norm() < 2e-7 * b.norm().max(1e-3), "{a} != {b}")
    };
    for (name, dc, first, third, second) in [
        (
            "out",
            0.25 + 0.2 / std::f64::consts::SQRT_2,
            -0.125,
            0.0,
            -0.1 / std::f64::consts::SQRT_2,
        ),
        ("cube", 2.0 / 3.0, -0.375, 1.0 / 24.0, 0.0),
    ] {
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        for (tuple, expected) in [
            (vec![0, 0], dc),
            (vec![1, 0], first),
            (vec![3, 0], third),
            (vec![0, 1], second),
        ] {
            close(
                point.spectra()[row][grid.index_of(&tuple).unwrap()],
                Complex64::new(expected, 0.0),
            );
        }
        let tuple = vec![-1, 1];
        let ratio = 0.13 + std::f64::consts::SQRT_2 - 1.0;
        let expected = if name == "out" { 0.25 } else { 1.5 } / Complex64::new(0.0, ratio);
        let ac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![rate * 0.13],
                    input_source: "Vinput".into(),
                    input_lattice: tuple.clone(),
                    output_node: name.into(),
                    output_ref: "0".into(),
                    output_lattice: tuple.clone(),
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: Default::default(),
                },
                &point,
            )
            .unwrap();
        close(ac.output_transfer[0], expected);
        let xf = engine
            .run_qpxf_from_qpss(
                &netlist,
                QpxfRequest {
                    frequencies_hz: vec![rate * ratio],
                    frequency_axis: QpxfFrequencyAxis::Output,
                    input_sources: QpxfSources::Named(vec!["Vinput".into()]),
                    input_lattices: QpxfInputLattices::Explicit(vec![tuple.clone()]),
                    output: QpxfOutput::Voltage {
                        positive: name.into(),
                        negative: "0".into(),
                    },
                    output_lattice: tuple,
                    linear: Default::default(),
                    group_delay: false,
                    group_delay_magnitude_floor: 0.0,
                },
                &point,
            )
            .unwrap();
        close(xf.transfers[0].values[0], expected);
    }
}

#[test]
fn qpss_filtered_integrals_preserve_network_phase_current_and_response() {
    let rate = 1e3;
    let f2 = rate * std::f64::consts::SQRT_2;
    let omega = std::f64::consts::TAU * rate;
    let netlist = Netlist::parse(&format!(
        "Filtered torus\nVinput input middle SIN(0 1 {rate})\nVsecond middle 0 SIN(0 .4 {f2})\n\
         Rfilter input junction 1k\nLfilter junction filtered {}\nCfilter filtered 0 {}\n\
         BV out 0 V={omega}*sdt(v(filtered))\nRout out 0 1k noisy=0\n\
         BI current 0 V={omega}*1k*sdt(i(Lfilter))\nRi current 0 1k noisy=0\n.end\n",
        1e3 / omega,
        1.0 / (1e3 * omega)
    ))
    .unwrap();
    let engine = Engine::default();
    let point = engine
        .run_qpss(&netlist, QpssConfig::new(vec![rate, f2], vec![1, 1]))
        .unwrap();
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let close = |a: Complex64, b: Complex64| {
        assert!((a - b).norm() < 2e-7 * b.norm().max(1e-3), "{a} != {b}")
    };
    for name in ["out", "current"] {
        let row = point
            .node_names()
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        let mut dc = 0.0;
        for (tuple, ratio, amplitude) in [
            (vec![1, 0], 1.0, 1.0),
            (vec![0, 1], std::f64::consts::SQRT_2, 0.4),
        ] {
            let network = 1.0 / Complex64::new(1.0 - ratio * ratio, ratio);
            let h = if name == "out" {
                network / Complex64::new(0.0, ratio)
            } else {
                network
            };
            let expected = h * Complex64::new(0.0, -amplitude / 2.0);
            close(
                point.spectra()[row][grid.index_of(&tuple).unwrap()],
                expected,
            );
            dc -= 2.0 * expected.re;
        }
        close(
            point.spectra()[row][grid.dc_index()],
            Complex64::new(dc, 0.0),
        );
        let tuple = vec![-1, 1];
        let ratio = 0.13 + std::f64::consts::SQRT_2 - 1.0;
        let network = 1.0 / Complex64::new(1.0 - ratio * ratio, ratio);
        let expected = if name == "out" {
            network / Complex64::new(0.0, ratio)
        } else {
            network
        };
        let ac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![rate * 0.13],
                    input_source: "Vinput".into(),
                    input_lattice: tuple.clone(),
                    output_node: name.into(),
                    output_ref: "0".into(),
                    output_lattice: tuple.clone(),
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: Default::default(),
                },
                &point,
            )
            .unwrap();
        close(ac.output_transfer[0], expected);
        let xf = engine
            .run_qpxf_from_qpss(
                &netlist,
                QpxfRequest {
                    frequencies_hz: vec![rate * ratio],
                    frequency_axis: QpxfFrequencyAxis::Output,
                    input_sources: QpxfSources::Named(vec!["Vinput".into()]),
                    input_lattices: QpxfInputLattices::Explicit(vec![tuple.clone()]),
                    output: QpxfOutput::Voltage {
                        positive: name.into(),
                        negative: "0".into(),
                    },
                    output_lattice: tuple,
                    linear: Default::default(),
                    group_delay: false,
                    group_delay_magnitude_floor: 0.0,
                },
                &point,
            )
            .unwrap();
        close(xf.transfers[0].values[0], expected);
    }
}

#[test]
fn qpss_source_driven_integrals_preserve_constants_and_small_signal_rates() {
    for (rate, method) in [
        (1e3, QuasiPeriodicLinearMethod::Direct),
        (1e9, QuasiPeriodicLinearMethod::Krylov),
    ] {
        let f2 = rate * std::f64::consts::SQRT_2;
        let omega = std::f64::consts::TAU * rate;
        let netlist = Netlist::parse(&format!(
            "Driven torus
Vnegative 0 middle SIN(0 .25 {rate})
Vinput input middle SIN(0 1.25 {rate})
Vcos cosine 0 SIN(0 1 {f2} 0 0 90)
Rnoise n 0 1k
BV out 0 V={omega}*sdt(v(input))+v(n)*(1+{omega}*sdt(v(input)))
Rout out 0 1k
BN nested 0 V={omega}*{omega}*2*sdt(sdt(v(cosine)))
Rn nested 0 1k
BI 0 sink I=.001*{omega}*sqrt(2)*sdt(v(cosine))
Ri sink 0 1k
.end
"
        ))
        .unwrap();
        let engine = Engine::default();
        let mut config = QpssConfig::new(vec![rate, f2], vec![1, 1]);
        config.solver.linear.method = method;
        let point = engine.run_qpss(&netlist, config.clone()).unwrap();
        let grid = engine
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        let close = |a: Complex64, b: Complex64| {
            assert!(
                (a - b).norm() < 2e-7 * b.norm().max(1e-3),
                "{a} != {b}, rate={rate}"
            )
        };
        for (name, tuple, dc, coefficient) in [
            ("out", vec![1, 0], 1.0, Complex64::new(-0.5, 0.0)),
            ("nested", vec![0, 1], 1.0, Complex64::new(-0.5, 0.0)),
            ("sink", vec![0, 1], 0.0, Complex64::new(0.0, -0.5)),
        ] {
            let index = point
                .node_names()
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            close(
                point.spectra()[index][grid.dc_index()],
                Complex64::new(dc, 0.0),
            );
            close(
                point.spectra()[index][grid.index_of(&tuple).unwrap()],
                coefficient,
            );
        }
        let (metadata, rows) = point.into_transfer_parts();
        let point = QpssOperatingPoint::from_transfer_parts_with_abort(
            metadata,
            rows,
            &crate::ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        let identity = point.retained_identity().to_owned();
        for (name, input, gain, order) in [
            ("out", "Vinput", omega, 1),
            ("nested", "Vcos", omega * omega * 2.0, 2),
            ("sink", "Vcos", omega * std::f64::consts::SQRT_2, 1),
        ] {
            let tuple = vec![-1, 1];
            let frequency = rate * 0.13 + f2 - rate;
            let jw = Complex64::new(0.0, std::f64::consts::TAU * frequency);
            let expected = if order == 2 {
                gain / (jw * jw)
            } else {
                gain / jw
            };
            let ac = engine
                .run_qpac_from_qpss(
                    &netlist,
                    QpacRequest {
                        offsets_hz: vec![rate * 0.13],
                        input_source: input.into(),
                        input_lattice: tuple.clone(),
                        output_node: name.into(),
                        output_ref: "0".into(),
                        output_lattice: tuple.clone(),
                        magnitude: 1.0,
                        phase_degrees: 0.0,
                        solver: QuasiPeriodicAcConfig {
                            linear: config.solver.linear.clone(),
                            ..Default::default()
                        },
                    },
                    &point,
                )
                .unwrap();
            close(ac.output_transfer[0], expected);
            let xf = engine
                .run_qpxf_from_qpss(
                    &netlist,
                    QpxfRequest {
                        frequencies_hz: vec![frequency],
                        frequency_axis: QpxfFrequencyAxis::Output,
                        input_sources: QpxfSources::Named(vec![input.into()]),
                        input_lattices: QpxfInputLattices::Explicit(vec![tuple.clone()]),
                        output: QpxfOutput::Voltage {
                            positive: name.into(),
                            negative: "0".into(),
                        },
                        output_lattice: tuple,
                        linear: config.solver.linear.clone(),
                        group_delay: false,
                        group_delay_magnitude_floor: 0.0,
                    },
                    &point,
                )
                .unwrap();
            close(xf.transfers[0].values[0], expected);
        }
        let noise = engine
            .run_qpnoise_from_qpss(
                &netlist,
                QpnoiseRequest {
                    frequencies_hz: vec![rate * 0.13],
                    frequency_axis: QpnoiseFrequencyAxis::Output,
                    outputs: vec![QpnoiseOutput {
                        observation: QpnoiseObservation::Voltage {
                            positive: "out".into(),
                            negative: "0".into(),
                        },
                        lattice: vec![0, 0],
                    }],
                    input: Some(QpnoiseInput {
                        source: "Vinput".into(),
                        lattice: vec![0, 0],
                    }),
                    input_lattices: QpnoiseLattices::AllRetained,
                    sources: QpnoiseSources::Only(vec!["RNOISE thermal".into()]),
                    integration: None,
                    contributor_ranking: false,
                    noise_figure: None,
                    linear: config.solver.linear,
                },
                &point,
            )
            .unwrap();
        let kb =
            super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect).boltzmann;
        let expected = 4.0 * kb * 300.15 * 1e3 * 4.5;
        assert!((noise.total_covariances[0].values[0].re / expected - 1.0).abs() < 2e-7);
        close(
            noise.outputs[0].input_transfer.as_ref().unwrap()[0],
            1.0 / Complex64::new(0.0, 0.13),
        );
        assert_eq!(point.retained_identity(), identity);
    }
}

#[test]
fn qpss_prescribed_integrals_anchor_nested_states_and_retained_conversion() {
    for (rate, method, mixing) in [
        (1e3, QuasiPeriodicLinearMethod::Direct, None),
        (1e9, QuasiPeriodicLinearMethod::Krylov, None),
        (1e3, QuasiPeriodicLinearMethod::Direct, Some(1)),
    ] {
        let f2 = rate * std::f64::consts::SQRT_2;
        let omega = std::f64::consts::TAU * rate;
        let deck = format!("Torus primitives
Vprobe src 0 DC 1
Rnoise src in 1k
Rbias in 0 1k
BV out 0 V=(1+{omega}*.2*sdt(sin(2*pi*{rate}*time))+{omega}*.1*sdt(sin(2*pi*{rate}*time)*cos(2*pi*{f2}*time)))*v(in)
Rout out 0 1k
BN nested 0 V={omega}*{omega}*sdt(sdt(cos(2*pi*{rate}*time)))
Rn nested 0 1k
BI 0 current I=.001*2*pi*{f2}*sdt(cos(2*pi*{f2}*time))
Ri current 0 1k
BZ zero 0 V=sdt(0)
Rz zero 0 1k
.end
");
        let netlist = Netlist::parse(&deck).unwrap();
        let engine = Engine::default();
        let mut config = QpssConfig::new(vec![rate, f2], vec![1, 1]);
        config.grid.max_mixing_order = mixing;
        config.solver.linear.method = method;
        config.solver.relative_tolerance = 1e-9;
        config.solver.current_absolute_tolerance = 1e-14;
        if rate == 1e9 {
            config.initial_state = QpssInitialState::DcOperatingPoint;
        }
        let point = engine.run_qpss(&netlist, config.clone()).unwrap();
        let grid = engine
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        let gain = |tuple: &[i32]| match tuple {
            [0, 0] => 1.1,
            [a, 0] if a.abs() == 1 => -0.1,
            [a, b] if a.abs() == 1 && b.abs() == 1 => {
                -0.1 * rate / (4.0 * (rate + (*a * *b) as Value * f2))
            }
            _ => 0.0,
        };
        let node = |name: &str| {
            point
                .node_names()
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap()
        };
        let close = |a: Complex64, b: Complex64, scale: Value| {
            assert!(
                (a - b).norm() < 2e-8 * scale,
                "{a} vs {b}, rate={rate}, mixing={mixing:?}"
            )
        };
        for (k, tuple) in grid.indices().iter().enumerate() {
            close(
                point.spectra()[node("out")][k],
                Complex64::new(0.5 * gain(tuple), 0.0),
                1.0,
            );
            let nested = match tuple.as_slice() {
                [0, 0] => 1.0,
                [a, 0] if a.abs() == 1 => -0.5,
                _ => 0.0,
            };
            close(
                point.spectra()[node("nested")][k],
                Complex64::new(nested, 0.0),
                1.0,
            );
            let current = match tuple.as_slice() {
                [0, b] if b.abs() == 1 => -0.5 * *b as Value,
                _ => 0.0,
            };
            close(
                point.spectra()[node("current")][k],
                Complex64::new(0.0, current),
                1.0,
            );
            close(point.spectra()[node("zero")][k], Complex64::ZERO, 1.0);
        }
        let integral = |name: &str| {
            &point.integral_spectra()[point
                .integral_names()
                .iter()
                .position(|n| n == name)
                .unwrap()]
        };
        close(
            integral("B:BV:sdt:0")[grid.dc_index()] * omega,
            Complex64::ONE,
            1.0,
        );
        // Excluding the mixed output modes must not erase their DC constant.
        close(
            integral("B:BV:sdt:1")[grid.dc_index()] * omega,
            -Complex64::ONE,
            1.0,
        );
        if mixing.is_none() {
            for name in [
                "B:BV:sdt:0",
                "B:BV:sdt:1",
                "B:BN:sdt:0",
                "B:BN:sdt:1",
                "B:BI:sdt:0",
                "B:BZ:sdt:0",
            ] {
                let spectrum = integral(name);
                let scale = spectrum
                    .iter()
                    .map(|v| v.norm())
                    .fold(1e-300_f64, Value::max);
                close(spectrum.iter().copied().sum(), Complex64::ZERO, scale);
            }
        }
        let (metadata, rows) = point.clone().into_transfer_parts();
        let point = QpssOperatingPoint::from_transfer_parts_with_abort(
            metadata,
            rows,
            &crate::ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        let identity = point.retained_identity().to_owned();
        let ac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![rate * 0.037],
                    input_source: "Vprobe".into(),
                    input_lattice: vec![0, 0],
                    output_node: "out".into(),
                    output_ref: "0".into(),
                    output_lattice: vec![1, 0],
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: QuasiPeriodicAcConfig {
                        linear: config.solver.linear.clone(),
                        ..Default::default()
                    },
                },
                &point,
            )
            .unwrap_or_else(|error| {
                panic!("QPAC rate={rate}, method={method:?}, mixing={mixing:?}: {error}")
            });
        close(ac.output_transfer[0], Complex64::new(-0.05, 0.0), 1.0);
        let xf = engine
            .run_qpxf_from_qpss(
                &netlist,
                QpxfRequest {
                    frequencies_hz: vec![rate * 0.037],
                    frequency_axis: QpxfFrequencyAxis::Output,
                    input_sources: QpxfSources::Named(vec!["Vprobe".into()]),
                    input_lattices: QpxfInputLattices::AllRetained,
                    output: QpxfOutput::Voltage {
                        positive: "out".into(),
                        negative: "0".into(),
                    },
                    output_lattice: vec![0, 0],
                    linear: config.solver.linear.clone(),
                    group_delay: false,
                    group_delay_magnitude_floor: 0.0,
                },
                &point,
            )
            .unwrap();
        for transfer in &xf.transfers {
            close(
                transfer.values[0],
                Complex64::new(0.5 * gain(&transfer.input_lattice), 0.0),
                1.0,
            );
        }
        let noise = engine
            .run_qpnoise_from_qpss(
                &netlist,
                QpnoiseRequest {
                    frequencies_hz: vec![rate * 0.037],
                    frequency_axis: QpnoiseFrequencyAxis::Output,
                    outputs: vec![QpnoiseOutput {
                        observation: QpnoiseObservation::Voltage {
                            positive: "out".into(),
                            negative: "0".into(),
                        },
                        lattice: vec![0, 0],
                    }],
                    input: Some(QpnoiseInput {
                        source: "Vprobe".into(),
                        lattice: vec![0, 0],
                    }),
                    input_lattices: QpnoiseLattices::AllRetained,
                    sources: QpnoiseSources::Only(vec!["RNOISE thermal".into()]),
                    integration: None,
                    contributor_ranking: false,
                    noise_figure: None,
                    linear: config.solver.linear,
                },
                &point,
            )
            .unwrap();
        let kb =
            super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect).boltzmann;
        let expected = kb
            * 300.15
            * 1e3
            * grid
                .indices()
                .iter()
                .map(|tuple| gain(tuple).powi(2))
                .sum::<Value>();
        close(
            noise.total_covariances[0].values[0],
            Complex64::new(expected, 0.0),
            expected,
        );
        close(
            noise.outputs[0].input_transfer.as_ref().unwrap()[0],
            Complex64::new(0.55, 0.0),
            1.0,
        );
        assert_eq!(point.retained_identity(), identity);
    }
}

#[test]
fn qpss_prescribed_integrals_reject_drift_including_nested_zero_origin() {
    for expression in ["sdt(1+sin(2*pi*1k*time))", "sdt(sdt(sin(2*pi*1k*time)))"] {
        let netlist = Netlist::parse(&format!(
            "Drift\nB1 out 0 V={expression}\nR1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_qpss(
                &netlist,
                QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]),
            )
            .unwrap_err()
            .to_string();
        assert!(error.contains("nonzero mean input"), "{error}");
    }
}
