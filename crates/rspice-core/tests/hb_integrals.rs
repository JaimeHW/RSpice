//! Public periodic analyses of integral-defined filters, with analytic oracles.

use rspice_core::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::{Engine, PeriodicDcOperatingPointSeed, QpssConfig};
use rspice_core::netlist::{
    FreqVariation, Netlist, PeriodicSourceSelector, PeriodicSweep, PxfCard,
};

fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() < 2e-7 * expected.norm().max(1e-3),
        "{actual} vs {expected}"
    );
}

fn filter(rate: f64, options: &str) -> Netlist {
    Netlist::parse(&format!(
        "Integral filter\nitest 0 input sin(.7m .2m {})\nrin input 0 1k\n\
         bv out 0 v=.2+{rate}*sdt(v(input)-v(out))\nrout out 0 1k noisy=0\n\
         bi sink 0 i=-.001*{rate}*sdt(v(out)-v(sink))\nrsink sink 0 1k noisy=0\n\
         {options}\n.end\n",
        rate / 10.0
    ))
    .unwrap()
}

fn transfer(rate: f64, frequency: f64) -> Complex64 {
    rate / Complex64::new(rate, std::f64::consts::TAU * frequency)
}

#[test]
fn hb_chained_nonlinear_integrals_resolve_each_stage_and_retain_response() {
    let rate = 1e3;
    let omega = std::f64::consts::TAU * rate;
    let circuit = |drift: &str| {
        Netlist::parse(&format!(
        "Chained integrals\nItest 0 input SIN(0 1m {rate})\n\
         Bforcing 0 input I=.001*sin(2*pi*{rate}*time)^3\n\
         Rinput input 0 1k\nBnonlinear input 0 I=.001*v(input)^3\n\
         BA first 0 V={omega}*sdt(v(input))\nRa first 0 1k\n\
         BB second 0 V={omega}*sdt((2*v(first)-v(first)^2)*sin(2*pi*{rate}*time){drift})\nRb second 0 1k\n\
         BC out 0 V={omega}*sdt(v(second)*sin(2*pi*{rate}*time))\nRc out 0 1k\n\
         .options hbint tahb=0\n.end\n"
    )).unwrap()
    };
    let netlist = circuit("");
    let engine = Engine::default();
    let mut config = HbConfig::new(rate)
        .with_harmonics(4)
        .with_collocation_points(33);
    config.use_krylov = true;
    config.tolerance = 1e-10;
    config.abstol = 1e-14;
    let hb = engine.run_hb(&netlist, config.clone()).unwrap();
    for (name, coefficients) in [
        ("first", [1.0, -1.0, 0.0, 0.0, 0.0]),
        ("second", [2.0 / 3.0, -0.75, 0.0, 1.0 / 12.0, 0.0]),
        (
            "out",
            [15.0 / 32.0, -2.0 / 3.0, 5.0 / 24.0, 0.0, -1.0 / 96.0],
        ),
    ] {
        let row = hb
            .result
            .spectral_voltages
            .iter()
            .find(|row| row.node_name.eq_ignore_ascii_case(name))
            .unwrap();
        for (&actual, expected) in row.coefficients.iter().zip(coefficients) {
            close(actual, Complex64::new(expected, 0.0));
        }
    }
    let request = |name: &str| {
        PacConfig::new()
            .with_fundamental(rate)
            .with_sweep(rate * 0.13, rate * 0.13, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(-2, 2)
            .with_input_source("Itest")
            .with_output_node(name)
    };
    let second = engine
        .run_pac_from_hb_with_abort(&netlist, request("second"), &hb.operating_point, &NoAbort)
        .unwrap();
    let output = engine
        .run_pac_from_hb_with_abort(&netlist, request("out"), &hb.operating_point, &NoAbort)
        .unwrap();
    let expected = (Complex64::new(0.0, 0.5)
        * second.result.conversion_matrix.get(0, 1, 1).unwrap()
        - Complex64::new(0.0, 0.5) * second.result.conversion_matrix.get(0, -1, 1).unwrap())
        / Complex64::new(0.0, 0.13);
    assert!(expected.norm() > 1e-3);
    close(
        output.result.conversion_matrix.get(0, 0, 1).unwrap(),
        expected,
    );
    let error = engine
        .run_hb(&circuit("+1e-7"), config)
        .unwrap_err()
        .to_string();
    assert!(error.contains("nonzero mean"), "{error}");
}

#[test]
fn hb_nonlinear_input_integrals_preserve_configuration_and_response() {
    let rate = 1e3;
    let omega = std::f64::consts::TAU * rate;
    for (startup, krylov, exact, samples) in [
        ("", false, true, 17),
        (".options hbint tahb=0", true, true, 33),
        (".options hbint tahb=0", false, false, 17),
    ] {
        let netlist = Netlist::parse(&format!(
            "Nonlinear integral input\nItest 0 input SIN(0 1m {rate})\n\
             Bforcing 0 input I=.001*sin(2*pi*{rate}*time)^3\n\
             Rinput input 0 1k\nBnonlinear input 0 I=.001*v(input)^3\n\
             Bshaped shaped 0 V=v(input)^3\nRshaped shaped 0 1k\n\
             BV out 0 V={omega}*sdt(v(shaped))\nRout out 0 1k\n\
             BI current 0 V=-1k*{omega}*sdt(i(Bshaped))\nRi current 0 1k\n\
             {startup}\n.end\n"
        ))
        .unwrap();
        let engine = Engine::default();
        let mut config = HbConfig::new(rate)
            .with_harmonics(3)
            .with_collocation_points(samples);
        config.tolerance = 1e-10;
        config.abstol = 1e-14;
        config.damping = 0.5;
        config.min_damping = 0.125;
        config.use_krylov = krylov;
        config.gmres_restart = 16;
        config.use_exact_jacobian = exact;
        config.source_stepping = false;
        let hb = engine.run_hb(&netlist, config).unwrap_or_else(|error| {
            panic!("startup={startup}, krylov={krylov}, exact={exact}: {error}")
        });
        for name in ["out", "current"] {
            let row = hb
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            close(row.coefficients[0], Complex64::new(2.0 / 3.0, 0.0));
            close(row.coefficients[1], Complex64::new(-0.75, 0.0));
            close(row.coefficients[3], Complex64::new(1.0 / 12.0, 0.0));
        }
        hb.operating_point.validate().unwrap();
        if !startup.is_empty() {
            continue;
        }
        let request = |name: &str| {
            PacConfig::new()
                .with_fundamental(rate)
                .with_sweep(rate * 0.13, rate * 0.13, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("Itest")
                .with_output_node(name)
        };
        let shaped = engine
            .run_pac_from_hb_with_abort(&netlist, request("shaped"), &hb.operating_point, &NoAbort)
            .unwrap();
        let output = engine
            .run_pac_from_hb_with_abort(&netlist, request("out"), &hb.operating_point, &NoAbort)
            .unwrap();
        for sideband in -1..=1 {
            let input = shaped.result.conversion_matrix.get(0, sideband, 0).unwrap();
            let actual = output.result.conversion_matrix.get(0, sideband, 0).unwrap();
            close(actual, input / Complex64::new(0.0, 0.13 + sideband as f64));
        }
    }
}

#[test]
fn hb_differential_integrals_keep_nonlinear_common_mode_feedback() {
    let rate = 1e3;
    let omega = std::f64::consts::TAU * rate;
    let netlist = Netlist::parse(&format!(
        "Differential integrals\nVinput plus minus SIN(0 1 {rate})\n\
         Bcommon minus 0 V=0.1*tanh(v(out)+v(cube))\nRplus plus 0 1k\nRminus minus 0 2k\n\
         BV out 0 V={omega}*sdt(0.25*v(plus)-.25*v(minus))\nRout out 0 1k\n\
         BC cube 0 V={omega}*sdt((v(plus)-v(minus))^3)\nRc cube 0 1k\n\
         .options hbint tahb=0\n.end\n"
    ))
    .unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(rate).with_harmonics(3))
        .unwrap();
    for (name, dc, fundamental, third) in [
        ("out", 0.25, -0.25, 0.0),
        ("cube", 2.0 / 3.0, -0.75, 1.0 / 12.0),
    ] {
        let row = hb
            .result
            .spectral_voltages
            .iter()
            .find(|r| r.node_name.eq_ignore_ascii_case(name))
            .unwrap();
        close(row.coefficients[0], Complex64::new(dc, 0.0));
        close(row.coefficients[1], Complex64::new(fundamental, 0.0));
        close(row.coefficients[3], Complex64::new(third, 0.0));
        let config = PacConfig::new()
            .with_fundamental(rate)
            .with_sweep(rate * 0.13, rate * 0.13, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(-1, 1)
            .with_input_source("Vinput")
            .with_output_node(name);
        let pac = engine
            .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
            .unwrap();
        for output in -1_i32..=1 {
            for input in -1_i32..=1 {
                let gain = if input == output {
                    if name == "out" { 0.25 } else { 1.5 }
                } else if name == "cube" && (input - output).abs() == 2 {
                    -0.75
                } else {
                    0.0
                };
                close(
                    pac.result.conversion_matrix.get(0, output, input).unwrap(),
                    gain / Complex64::new(0.0, 0.13 + output as f64),
                );
            }
        }
    }
}

#[test]
fn hb_filtered_integrals_preserve_network_phase_current_and_response() {
    let rate = 1e3;
    let omega = std::f64::consts::TAU * rate;
    let netlist = Netlist::parse(&format!(
        "Filtered integrals\nVinput input 0 SIN(0 1 {rate})\nRfilter input middle 1k\n\
         Lfilter middle filtered {}\nCfilter filtered 0 {}\n\
         BV out 0 V={omega}*sdt(v(filtered))\nRout out 0 1k noisy=0\n\
         BI current 0 V={omega}*1k*sdt(i(Lfilter))\nRi current 0 1k noisy=0\n\
         .options hbint tahb=0\n.end\n",
        1e3 / omega,
        1.0 / (1e3 * omega)
    ))
    .unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(rate).with_harmonics(2))
        .unwrap();
    for (name, ac) in [
        ("out", Complex64::new(0.0, 1.0)),
        ("current", Complex64::new(-1.0, 0.0)),
    ] {
        let row = hb
            .result
            .spectral_voltages
            .iter()
            .find(|r| r.node_name.eq_ignore_ascii_case(name))
            .unwrap();
        close(row.coefficients[0], Complex64::new(-ac.re, 0.0));
        close(row.coefficients[1], ac);
        let config = PacConfig::new()
            .with_fundamental(rate)
            .with_sweep(rate * 0.13, rate * 0.13, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(-1, 1)
            .with_input_source("Vinput")
            .with_output_node(name);
        let pac = engine
            .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
            .unwrap();
        for sideband in -1..=1 {
            let ratio = 0.13 + sideband as f64;
            let network = 1.0 / Complex64::new(1.0 - ratio * ratio, ratio);
            let expected = if name == "out" {
                network / Complex64::new(0.0, ratio)
            } else {
                network
            };
            close(
                pac.result
                    .conversion_matrix
                    .get(0, sideband, sideband)
                    .unwrap(),
                expected,
            );
        }
    }
    let noise = engine
        .run_pnoise_from_hb_with_abort(
            &netlist,
            &[rate * 0.13],
            "out",
            None,
            Some("Vinput"),
            1,
            &hb.operating_point,
            &NoAbort,
        )
        .unwrap();
    let h = 1.0 / (Complex64::new(1.0 - 0.13 * 0.13, 0.13) * Complex64::new(0.0, 0.13));
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * h.norm_sqr();
    assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-7);
}

#[test]
fn hb_source_driven_integrals_preserve_constants_and_small_signal_rates() {
    for rate in [1e3, 1e9] {
        let omega = std::f64::consts::TAU * rate;
        let netlist = Netlist::parse(&format!(
            "Driven integrals
Vnegative 0 middle SIN(0 .25 {rate})
Vinput input middle SIN(0 1.25 {rate})
Vcos cosine 0 SIN(0 1 {rate} 0 0 90)
Rnoise n 0 1k
BV out 0 V={omega}*sdt(v(input))+v(n)*(1+{omega}*sdt(v(input)))
Rout out 0 1k
BN nested 0 V={omega}*{omega}*sdt(sdt(v(cosine)))
Rn nested 0 1k
BI 0 sink I=.001*{omega}*sdt(v(cosine))
Ri sink 0 1k
.options hbint tahb=0
.end
"
        ))
        .unwrap();
        let engine = Engine::default();
        let hb = engine
            .run_hb(&netlist, HbConfig::new(rate).with_harmonics(2))
            .unwrap();
        for (name, dc, ac) in [
            ("out", 1.0, Complex64::new(-1.0, 0.0)),
            ("nested", 1.0, Complex64::new(-1.0, 0.0)),
            ("sink", 0.0, Complex64::new(0.0, -1.0)),
        ] {
            let row = hb
                .result
                .spectral_voltages
                .iter()
                .find(|r| r.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            close(row.coefficients[0], Complex64::new(dc, 0.0));
            close(row.coefficients[1], ac);
        }
        hb.operating_point.validate().unwrap();
        for name in ["out", "nested", "sink"] {
            let input = if name == "out" { "Vinput" } else { "Vcos" };
            let config = PacConfig::new()
                .with_fundamental(rate)
                .with_sweep(rate * 0.13, rate * 0.13, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source(input)
                .with_output_node(name);
            let pac = engine
                .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
                .unwrap();
            for sideband in -1..=1 {
                let frequency = rate * (0.13 + sideband as f64);
                let h = omega / Complex64::new(0.0, std::f64::consts::TAU * frequency);
                let expected = if name == "nested" { h * h } else { h };
                close(
                    pac.result
                        .conversion_matrix
                        .get(0, sideband, sideband)
                        .unwrap(),
                    expected,
                );
                let other = if sideband == 0 { 1 } else { 0 };
                close(
                    pac.result
                        .conversion_matrix
                        .get(0, other, sideband)
                        .unwrap(),
                    Complex64::ZERO,
                );
            }
        }
        let noise = engine
            .run_pnoise_from_hb_with_abort(
                &netlist,
                &[rate * 0.13],
                "out",
                None,
                Some("Vinput"),
                1,
                &hb.operating_point,
                &NoAbort,
            )
            .unwrap();
        let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * 4.5;
        assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-7);
    }
    for expression in ["sdt(v(input))", "sdt(sdt(v(input)))"] {
        let voltage = if expression == "sdt(v(input))" {
            "DC 1"
        } else {
            "SIN(0 1 1k)"
        };
        let netlist = Netlist::parse(&format!(
            "Drift\nVinput input 0 {voltage}\nB1 out 0 V={expression}\nR1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
            .unwrap_err()
            .to_string();
        assert!(error.contains("nonzero mean input"), "{error}");
    }
}

fn sweep(rate: f64) -> PacConfig {
    PacConfig::new()
        .with_fundamental(rate / 10.0)
        .with_sweep(rate * 0.13, rate * 0.13, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("itest")
        .with_output_node("sink")
}

#[test]
fn hb_integral_startup_modes_retain_constants_and_physical_currents() {
    // Cover each initializer and both linear solvers without a Cartesian sweep.
    for (rate, options, krylov) in [
        (1e3, "", false),
        (1e9, ".options hbint tahb=0", true),
        (1e3, ".options hbint tahb=1\n.save v(sink)", false),
        (1e9, ".options hbint tahb=2", false),
    ] {
        let netlist = filter(rate, options);
        let mut config = HbConfig::new(rate / 10.0)
            .with_harmonics(2)
            .with_tolerance(1e-10);
        config.use_krylov = krylov;
        let hb = Engine::default()
            .run_hb(&netlist, config)
            .unwrap_or_else(|e| panic!("rate={rate}, options={options}: {e}"));
        let h = transfer(rate, rate / 10.0);
        for (name, expected) in [("out", h), ("sink", h * h)] {
            let row = hb
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            close(row.coefficients[0], Complex64::new(0.7, 0.0));
            close(row.coefficients[1], Complex64::new(0.0, -0.2) * expected);
        }
        let point = &hb.operating_point;
        point.validate().unwrap();
        assert_eq!(point.mna_branch_names(), ["BV"]);
        assert_eq!(point.integral_spectra().len(), 2);
        close(
            point.integral_spectra()[0].coefficients[0] * rate,
            Complex64::new(0.5, 0.0),
        );
        close(
            point.integral_spectra()[1].coefficients[0] * rate,
            Complex64::new(0.7, 0.0),
        );
        assert_eq!(hb.result.mna_branch_currents.len(), 1);
        let current = hb
            .device_currents
            .iter()
            .find(|row| row.probe.eq_ignore_ascii_case("I(bi)"))
            .unwrap();
        close(current.coefficients[0], Complex64::new(-0.7e-3, 0.0));
        close(current.coefficients[1], Complex64::new(0.0, 0.2e-3) * h * h);
        assert!(
            hb.device_currents
                .iter()
                .all(|row| !row.probe.contains("sdt:"))
        );
        if options.contains("tahb=2") {
            let seed = PeriodicDcOperatingPointSeed::try_new(
                point.node_names().to_vec(),
                point.mna_branch_names().to_vec(),
                point
                    .spectral_state()
                    .iter()
                    .chain(point.mna_branch_spectral_state())
                    .map(|row| row[0].re)
                    .collect(),
            )
            .unwrap();
            let seeded = Engine::default()
                .run_hb_with_dc_seed_and_abort(&netlist, point.config().clone(), &seed, &NoAbort)
                .unwrap();
            for (actual, expected) in seeded
                .operating_point
                .integral_spectra()
                .iter()
                .zip(point.integral_spectra())
            {
                for (a, b) in actual.coefficients.iter().zip(&expected.coefficients) {
                    close(*a * rate, *b * rate);
                }
            }
        }
    }
}

#[test]
fn hb_integral_forcing_enlarges_only_derived_carrier_grids() {
    let netlist = Netlist::parse(
        "Integral forcing grid\nitest 0 input 1m\nrin input 0 1k\n\
         bv sink 0 v=1k*sdt((1+.5*cos(2*pi*9k*time))*v(input)-v(sink))\n\
         rout sink 0 1k\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let error = engine
        .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
        .expect_err("an explicit two-harmonic basis cannot resolve the ninth harmonic")
        .to_string();
    assert!(error.contains("beyond the configured 2"), "{error}");
    let pac = engine
        .run_pac(
            &netlist,
            sweep(1e3).with_fundamental(1e3).with_sidebands(0, 0),
        )
        .unwrap();
    close(
        pac.result.conversion_matrix.get(0, 0, 0).unwrap(),
        1e3 * transfer(1e3, 130.0),
    );
}

#[test]
fn hb_integrals_reject_constant_drift_even_below_residual_tolerance() {
    for (kind, rate) in [("v", "1"), ("i", "-1e-30"), ("v", "1e-30")] {
        let netlist = Netlist::parse(&format!(
            "Nonperiodic integral\nbdrift out 0 {kind}=sdt({rate})\nr1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(1))
            .expect_err("a small ramp is still nonperiodic")
            .to_string();
        assert!(
            error.contains("BDRIFT")
                && error.contains("SDT 0")
                && error.contains("cannot be periodic"),
            "{error}"
        );
    }
}

#[test]
fn hb_prescribed_integrals_preserve_zero_origin_and_retained_modulation() {
    let engine = Engine::default();
    for (frequency, krylov, options) in [
        (1e3, false, ""),
        (1e9, true, ".options hbint tahb=0"),
        (1e3, false, ".options hbint tahb=1\n.save v(out)"),
    ] {
        let netlist = Netlist::parse(&format!(
            "Prescribed integrals\nitest 0 input 1m\nrin input 0 1k\n\
             bclock clock 0 v={frequency}*sdt(sin(2*pi*{frequency}*time))\n\
             bnested nested 0 v={frequency}*sdt({frequency}*sdt(cos(2*pi*{frequency}*time)))\n\
             bcurrent sink 0 i=.001*{frequency}*sdt(cos(2*pi*{frequency}*time))\nrsink sink 0 1k noisy=0\n\
             bout out 0 v=(1+v(clock))*v(input)\nrout out 0 1k noisy=0\n{options}\n.end\n"
        )).unwrap();
        let mut config = HbConfig::new(frequency)
            .with_harmonics(3)
            .with_tolerance(1e-10);
        config.use_krylov = krylov;
        let hb = engine.run_hb(&netlist, config).unwrap();
        let tau = std::f64::consts::TAU;
        for (name, dc, harmonic) in [
            ("clock", 1.0 / tau, Complex64::new(-1.0 / tau, 0.0)),
            (
                "nested",
                1.0 / (tau * tau),
                Complex64::new(-1.0 / (tau * tau), 0.0),
            ),
            ("sink", 0.0, Complex64::new(0.0, 1.0 / tau)),
            ("out", 1.0 + 1.0 / tau, Complex64::new(-1.0 / tau, 0.0)),
        ] {
            let row = hb
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            close(row.coefficients[0], Complex64::new(dc, 0.0));
            close(row.coefficients[1], harmonic);
        }
        assert_eq!(hb.operating_point.integral_spectra().len(), 4);
        hb.operating_point.validate().unwrap();
        for spectrum in hb.operating_point.integral_spectra() {
            let origin = spectrum.coefficients[0].re
                + 2.0 * spectrum.coefficients[1..].iter().map(|v| v.re).sum::<f64>();
            assert!(
                (origin * frequency).abs() < 1e-8,
                "{} origin={origin}",
                spectrum.name
            );
        }
        // One set of dependency solves covers the shared constant-Jacobian
        // constraints. Other iterations exercise startup and frequency scaling.
        if frequency == 1e3 && options.is_empty() {
            let config = PacConfig::new()
                .with_fundamental(frequency)
                .with_sweep(frequency, frequency, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("itest")
                .with_output_node("out");
            let pss = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(frequency)
                        .with_harmonics(3)
                        .with_points_per_period(256)
                        .with_tstab_periods(0),
                    &NoAbort,
                )
                .unwrap();
            for (pac, tolerance) in [
                (engine.run_pac(&netlist, config.clone()).unwrap(), 2e-7),
                (
                    engine
                        .run_pac_from_hb_with_abort(
                            &netlist,
                            config.clone(),
                            &hb.operating_point,
                            &NoAbort,
                        )
                        .unwrap(),
                    2e-7,
                ),
                (
                    engine
                        .run_pac_from_pss_with_abort(&netlist, config, &pss, &NoAbort)
                        .unwrap(),
                    2e-4,
                ),
            ] {
                for sideband in -1..=1 {
                    let expected = if sideband == 0 {
                        1e3 * (1.0 + 1.0 / tau)
                    } else {
                        -500.0 / tau
                    };
                    let actual = pac.result.conversion_matrix.get(0, sideband, 0).unwrap();
                    assert!(
                        (actual - expected).norm() < tolerance * expected.abs(),
                        "{actual} vs {expected}"
                    );
                }
            }
            let noise = engine
                .run_pnoise_from_hb_with_abort(
                    &netlist,
                    &[130.0],
                    "out",
                    None,
                    Some("itest"),
                    1,
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap();
            let expected = 4.0
                * K_BOLTZMANN
                * TEMP_REFERENCE
                * 1e3
                * ((1.0 + 1.0 / tau).powi(2) + 0.5 / (tau * tau));
            assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-7);
            let from_pss = engine
                .run_pnoise_from_pss_with_abort(
                    &netlist,
                    &[130.0],
                    "out",
                    None,
                    Some("itest"),
                    1,
                    &pss,
                    &NoAbort,
                )
                .unwrap();
            assert!((from_pss.output_noise[0] / expected - 1.0).abs() < 2e-4);
        }
    }
}

#[test]
fn hb_prescribed_integrals_reject_secular_drift_and_bound_projection_work() {
    for expression in ["sdt(1+sin(2*pi*1k*time))", "sdt(sdt(sin(2*pi*1k*time)))"] {
        let netlist = Netlist::parse(&format!(
            "Drifting primitive\nb1 out 0 v={expression}\nr1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(3))
            .expect_err("the zero-origin primitive drifts each period")
            .to_string();
        assert!(error.contains("nonzero mean input"), "{error}");
    }
    let netlist =
        Netlist::parse("Bounded primitive\nb1 out 0 v=sdt(cos(2*pi*1k*time))\nr1 out 0 1k\n.end\n")
            .unwrap();
    let mut config = rspice_core::engine::SimulationConfig::default();
    config.resource_limits.max_result_values = 120;
    let error = Engine::new(config)
        .run_hb(
            &netlist,
            HbConfig::new(1e3)
                .with_harmonics(3)
                .with_collocation_points(129),
        )
        .expect_err("primitive preparation must honor the configured memory limit")
        .to_string();
    assert!(error.contains("periodic allocation limit"), "{error}");
}

#[test]
fn hb_prescribed_zero_rates_retain_zero_integral_coordinates() {
    let deck =
        Netlist::parse("Zero primitives\nB1 out 0 V=sdt(0)+sdt(sdt(0))\nR1 out 0 1k\n.end\n")
            .unwrap();
    let result = Engine::default()
        .run_hb(&deck, HbConfig::new(1e3).with_harmonics(1))
        .unwrap();
    result.operating_point.validate().unwrap();
    assert_eq!(result.operating_point.integral_spectra().len(), 3);
    for integral in result.operating_point.integral_spectra() {
        assert!(integral.coefficients.iter().all(|v| *v == Complex64::ZERO));
    }
    assert!(
        result
            .result
            .spectral_voltages
            .iter()
            .flat_map(|row| &row.coefficients)
            .all(|v| *v == Complex64::ZERO)
    );
}

#[test]
fn hb_integral_dependents_share_transfer_and_noise_from_fresh_hb_and_pss() {
    let rate = 1e3;
    let netlist = filter(rate, "");
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(rate / 10.0).with_harmonics(3))
        .unwrap();
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(rate / 10.0)
                .with_harmonics(3)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    for pac in [
        engine.run_pac(&netlist, sweep(rate)).unwrap(),
        engine
            .run_pac_from_hb_with_abort(&netlist, sweep(rate), &hb.operating_point, &NoAbort)
            .unwrap(),
        engine
            .run_pac_from_pss_with_abort(&netlist, sweep(rate), &pss, &NoAbort)
            .unwrap(),
    ] {
        assert_eq!(pac.result.branch_names, ["BV"]);
        for output in -1..=1 {
            for input in -1..=1 {
                let h = transfer(rate, rate * (0.13 + f64::from(input) / 10.0));
                let expected = if input == output {
                    1e3 * h * h
                } else {
                    Complex64::ZERO
                };
                close(
                    pac.result.conversion_matrix.get(0, output, input).unwrap(),
                    expected,
                );
            }
        }
        let data = pac.result.get_sideband_data(0, 0).unwrap();
        assert_eq!(data.branch_currents.len(), 1);
        close(data.branch_currents[0], -transfer(rate, rate * 0.13));
    }
    let card = PxfCard {
        sweep: PeriodicSweep {
            variation: FreqVariation::Lin,
            points: 1,
            start_freq: 130.0,
            stop_freq: 130.0,
        },
        input_source: "itest".into(),
        input_sideband: 1,
        output_node: "sink".into(),
        output_ref: None,
        output_sideband: 1,
        max_sideband: 1,
        reltol: 1e-6,
        abstol: 1e-12,
        source: PeriodicSourceSelector::Preceding,
    };
    for pxf in [
        engine
            .run_pxf_card_from_hb_with_abort(&netlist, &card, &hb.operating_point, &NoAbort)
            .unwrap(),
        engine
            .run_pxf_card_from_pss_with_abort(&netlist, &card, &pss, &NoAbort)
            .unwrap(),
    ] {
        let h = transfer(rate, 230.0);
        close(pxf.points[0].transfer, 1e3 * h * h);
    }
    let h = transfer(rate, 130.0);
    let expected_noise = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * (h * h).norm_sqr();
    for noise in [
        engine
            .run_pnoise(&netlist, 100.0, &[130.0], "sink", None, Some("itest"), 1)
            .unwrap(),
        engine
            .run_pnoise_from_hb_with_abort(
                &netlist,
                &[130.0],
                "sink",
                None,
                Some("itest"),
                1,
                &hb.operating_point,
                &NoAbort,
            )
            .unwrap(),
        engine
            .run_pnoise_from_pss_with_abort(
                &netlist,
                &[130.0],
                "sink",
                None,
                Some("itest"),
                1,
                &pss,
                &NoAbort,
            )
            .unwrap(),
    ] {
        assert!((noise.output_noise[0] / expected_noise - 1.0).abs() < 1e-7);
    }
}

#[test]
fn hb_integral_port_scattering_matches_filter_transfer() {
    let netlist = Netlist::parse(
        "Integral two-port\nP1 input 0 portnum=1 z0=50\nR1 input 0 50\n\
         bv out 0 v=1k*sdt(v(input)-v(out))\nR2 out output 50 noisy=0\n\
         P2 output 0 portnum=2 z0=50\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(100.0).with_harmonics(2))
        .unwrap();
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(100.0)
                .with_harmonics(2)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    for prepared in [
        engine
            .prepare_psp_from_hb_with_abort(&netlist, sweep(1e3), &hb.operating_point, &NoAbort)
            .unwrap(),
        engine
            .prepare_psp_from_pss_with_abort(&netlist, sweep(1e3), &pss, &NoAbort)
            .unwrap(),
    ] {
        let result = prepared.run_with_abort(&NoAbort).unwrap();
        let matrix = &result.data[0];
        for sideband in 0..3 {
            let frequency = 130.0 + (sideband as f64 - 1.0) * 100.0;
            close(
                matrix.get(4 + sideband, 1 + sideband),
                0.5 * transfer(1e3, frequency),
            );
            close(matrix.get(1 + sideband, 1 + sideband), Complex64::ZERO);
            close(matrix.get(4 + sideband, 4 + sideband), Complex64::ZERO);
            close(matrix.get(1 + sideband, 4 + sideband), Complex64::ZERO);
        }
    }
    let error = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(vec![100.0, 141.4213562373095], vec![1, 1]),
        )
        .expect_err("QPSS needs its own retained integral basis")
        .to_string();
    assert!(error.contains("independent-phase state lifting"), "{error}");
}
