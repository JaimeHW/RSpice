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
