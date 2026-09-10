//! Contract tests for consumers of an already-authenticated shooting-PSS
//! operating point. These deliberately exercise a sideband span larger than
//! the producer's optional saved harmonic count; the retained time orbit's
//! Nyquist capacity is the governing numerical limit.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::pss::PssConfig;
use rspice_core::engine::{Engine, PssOperatingPoint, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn retained_linear_operating_point(engine: &Engine, netlist: &Netlist) -> PssOperatingPoint {
    let config = PssConfig::new(F0)
        .with_harmonics(20)
        .with_points_per_period(256)
        .with_tstab_periods(0);
    engine
        .run_pss_operating_point_with_abort(netlist, config, &NoAbort)
        .expect("linear producer yields an authenticated retained PSS state")
}

fn linear_deck() -> Netlist {
    Netlist::parse(
        "* retained PSS consumer contract\n\
         vin in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         r2 out 0 1k\n\
         c1 out 0 1p\n\
         .end\n",
    )
    .expect("deck parses")
}

#[test]
fn mockup_sideband_span_uses_orbit_nyquist_capacity_not_saved_harmonic_count() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = linear_deck();
    let operating_point = retained_linear_operating_point(&engine, &netlist);
    assert_eq!(operating_point.config().num_harmonics, 20);
    assert_eq!(operating_point.spectral_harmonic_capacity(), 128);
    let pac = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e3, 1.0e3, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-20, 20)
        .with_input_source("vin")
        .with_output_node("out");
    engine
        .run_pac_from_pss_with_abort(&netlist, pac, &operating_point, &NoAbort)
        .expect("PAC +/-20 consumes the 256-point retained orbit");

    let pnoise = engine
        .run_pnoise_from_pss_with_abort(
            &netlist,
            &[1.0e3],
            "out",
            None,
            Some("vin"),
            20,
            &operating_point,
            &NoAbort,
        )
        .expect("PNOISE +/-20 consumes the 256-point retained orbit");
    assert_eq!(pnoise.output_noise.len(), 1);
}

#[test]
fn dependent_basis_includes_the_carrier_outside_the_conversion_window() {
    let deck = Netlist::parse(
        "Carrier bandwidth\nI1 0 out SIN(0 1 10)\nR1 out 0 RM 1\nC1 out 0 1u\n.model RM R(KF=1 AF=2 EF=1)\n.end\n",
    ).unwrap();
    let engine = Engine::default();
    let point = engine
        .run_pss_operating_point_with_abort(
            &deck,
            PssConfig::new(1.0)
                .with_harmonics(2)
                .with_points_per_period(1024)
                .with_tstab(0.0),
            &NoAbort,
        )
        .unwrap();
    let omega_c = std::f64::consts::TAU * 1e-6;
    let expected_noise = 0.25 * (1.0 / 9.75 + 1.0 / 10.25)
        / (1.0 + (10.0 * omega_c).powi(2))
        / (1.0 + (0.25 * omega_c).powi(2));
    for retained in [false, true] {
        let noise = if retained {
            engine.run_pnoise_from_pss_with_abort(
                &deck,
                &[0.25],
                "out",
                None,
                None,
                0,
                &point,
                &NoAbort,
            )
        } else {
            engine.run_pnoise(&deck, 1.0, &[0.25], "out", None, None, 0)
        }
        .unwrap();
        let flicker = noise
            .contributors
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
            .unwrap()
            .1[0];
        assert!((flicker / expected_noise - 1.0).abs() < 2e-9);

        let config = PacConfig::new()
            .with_fundamental(1.0)
            .with_sweep(0.25, 0.25, 1)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(0, 0)
            .with_input_source("I1")
            .with_output_node("out");
        let pac = if retained {
            engine.run_pac_from_pss_with_abort(&deck, config, &point, &NoAbort)
        } else {
            engine.run_pac(&deck, config)
        }
        .unwrap();
        let transfer = pac.result.conversion_matrix.get(0, 0, 0).unwrap();
        let expected = num_complex::Complex64::new(1.0, 0.0)
            / num_complex::Complex64::new(1.0, 0.25 * omega_c);
        assert!((transfer - expected).norm() < 2e-12);
    }
}
