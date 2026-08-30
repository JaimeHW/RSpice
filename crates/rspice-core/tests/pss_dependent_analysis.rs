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
