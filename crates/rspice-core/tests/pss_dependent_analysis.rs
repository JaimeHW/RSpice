//! Contract tests for consumers of an already-authenticated shooting-PSS
//! operating point. These deliberately exercise a sideband span larger than
//! the producer's optional saved harmonic count; the retained time orbit's
//! Nyquist capacity is the governing numerical limit.

use num_complex::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::advanced::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::advanced::pss::{PeriodicWaveform, PssConfig, PssResult};
use rspice_core::engine::{Engine, PssAnalysisResult, PssOperatingPoint, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn retained_linear_operating_point() -> PssOperatingPoint {
    let config = PssConfig::new(F0)
        .with_harmonics(20)
        .with_points_per_period(256);
    let period = 1.0 / F0;
    let time = (0..=256)
        .map(|index| period * index as f64 / 256.0)
        .collect::<Vec<_>>();
    let zeros = vec![0.0; time.len()];
    let result = PssResult {
        period,
        frequency: F0,
        iterations: 1,
        residual_norm: 0.0,
        time,
        waveforms: vec![
            PeriodicWaveform::from_values(zeros.clone()),
            PeriodicWaveform::from_values(zeros),
        ],
        node_names: vec!["in".to_owned(), "out".to_owned()],
        period_detected: false,
        floquet_multipliers: vec![Complex64::new(0.5, 0.0)],
    };
    PssOperatingPoint::try_from_parts(
        config,
        PssAnalysisResult {
            result,
            iterations: 1,
            final_residual: 0.0,
            period,
            monodromy: vec![vec![0.5]],
            floquet_multipliers: vec![Complex64::new(0.5, 0.0)],
            is_stable: true,
        },
        vec![0.0],
    )
    .expect("synthetic retained PSS state is structurally complete")
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
    let operating_point = retained_linear_operating_point();
    assert_eq!(operating_point.config().num_harmonics, 20);
    assert_eq!(operating_point.spectral_harmonic_capacity(), 128);

    let engine = Engine::new(SimulationConfig::default());
    let netlist = linear_deck();
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
