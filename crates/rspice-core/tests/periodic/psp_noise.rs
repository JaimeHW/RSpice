//! Periodic intrinsic port noise: independent circuit laws and load invariance.
use rspice_core::abort_signal::{CountingAbort, NoAbort};
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::engine::{Engine, PspAnalysisResult, SimulationError};
use rspice_core::{Complex64, Netlist};

const KT: f64 = 1.380649e-23 * 300.15;

fn sweep(bands: i32) -> PacConfig {
    PacConfig::new()
        .with_sweep(1e4, 1e4, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-bands, bands)
}

fn run(deck: &str, bands: i32) -> PspAnalysisResult {
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(
            &netlist,
            HbConfig::new(1e6)
                .with_harmonics((2 * bands as usize).max(8))
                .with_oversample(4),
        )
        .unwrap();
    engine
        .prepare_psp_from_hb_with_noise_and_abort(
            &netlist,
            sweep(bands),
            &hb.operating_point,
            true,
            &NoAbort,
        )
        .unwrap()
        .run_with_abort(&NoAbort)
        .unwrap()
}

#[test]
fn periodic_noise_waves_match_analytic_two_port_rc_and_ignore_physical_load_multiplicity() {
    for multiplicity in [0.5, 1.0, 2.0] {
        let result = run(
            &format!(
                "RC port noise
.subckt source p
P1 p 0 portnum=1 z0=50
.ends
X1 p source M={multiplicity}
P2 q 0 portnum=2 z0=75
Rd p q 100
Cq q 0 2n
.end
"
            ),
            1,
        );
        let point = &result.noise.as_ref().unwrap()[0];
        assert_eq!(point.frequency, 1e4);
        let noise = &point.wave_correlation;
        for row in 0..6 {
            for column in 0..6 {
                let expected = if row % 3 != column % 3 {
                    Complex64::ZERO
                } else {
                    let frequency = 1e4 + (row % 3) as f64 * 1e6 - 1e6;
                    let g = 0.01;
                    let a = 1.0 / 50.0 + g;
                    let d =
                        Complex64::new(1.0 / 75.0 + g, std::f64::consts::TAU * frequency * 2e-9);
                    let determinant = a * d - g * g;
                    let waves = [
                        (d - g) / determinant / 50.0_f64.sqrt(),
                        (g - a) / determinant / 75.0_f64.sqrt(),
                    ];
                    4.0 * KT * g * waves[row / 3] * waves[column / 3].conj()
                };
                assert!(
                    (noise[row][column] - expected).norm() < 2e-12 * KT,
                    "M={multiplicity}, C[{row},{column}]={} expected {expected}",
                    noise[row][column]
                );
                assert_eq!(noise[row][column], noise[column][row].conj());
            }
        }
        assert!(
            noise[1][4].im.abs() > 1e-5 * KT,
            "complex port correlation is observable"
        );
    }
}

#[test]
fn periodic_noise_excludes_external_terminations_and_supports_reversed_port_polarity() {
    for plane in ["p 0", "0 p"] {
        let resistor = run(
            &format!(
                "thermal load
P1 {plane} portnum=1 z0=50
R1 p 0 100
.end
"
            ),
            0,
        );
        let got = resistor.noise.as_ref().unwrap()[0].wave_correlation[0][0];
        assert!((got.re / KT - 8.0 / 9.0).abs() < 1e-12);
        assert_eq!(got.im, 0.0);
        let open = run(
            &format!(
                "noiseless open
P1 {plane} portnum=1 z0=50
.end
"
            ),
            0,
        );
        assert_eq!(
            open.noise.unwrap()[0].wave_correlation[0][0],
            Complex64::ZERO
        );
    }
}

#[test]
fn shooting_and_hb_noise_waves_use_the_same_authenticated_device_catalog() {
    let netlist = Netlist::parse(
        "shooting load
P1 p 0 portnum=1 z0=50
R1 p 0 100
C1 p 0 1n
.options temp=126.85
.end
",
    )
    .unwrap();
    let engine = Engine::default();
    let point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1e6)
                .with_harmonics(8)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    let data = engine
        .prepare_psp_from_pss_with_noise_and_abort(&netlist, sweep(0), &point, true, &NoAbort)
        .unwrap()
        .run_with_abort(&NoAbort)
        .unwrap();
    let got = data.noise.unwrap()[0].wave_correlation[0][0].re;
    let expected = 4.0 * 1.380649e-23 * 400.0
        / 100.0
        / (0.03_f64.powi(2) + (std::f64::consts::TAU * 1e4 * 1e-9).powi(2))
        / 50.0;
    assert!((got / expected - 1.0).abs() < 1e-11);
}

#[test]
fn periodic_switch_noise_correlations_match_independent_time_domain_quadrature() {
    for multiplicity in [1, 2] {
        let result = run(
            &format!(
                "periodic thermal load
.subckt source p
P1 p 0 portnum=1 z0=1k
.ends
X1 out source M={multiplicity}
Vlo ctl 0 SIN(0 1 1Meg)
S1 out 0 ctl 0 SWMOD
.model SWMOD SW VT=0 RON=100 ROFF=10k SMOOTH=1
.end
"
            ),
            12,
        );
        let covariance = &result.noise.as_ref().unwrap()[0].wave_correlation;
        for difference in [-2_i32, -1, 0, 1, 2] {
            let samples = 4096;
            let mut expected = Complex64::ZERO;
            for sample in 0..samples {
                let phase = std::f64::consts::TAU * (sample as f64 + 0.5) / samples as f64;
                let off = 0.5 * (1.0 - phase.sin().tanh());
                let resistance = (100.0_f64.ln() * (1.0 - off) + 10_000.0_f64.ln() * off).exp();
                let intensity = 4.0 * KT * 1000.0 * resistance / (1000.0 + resistance).powi(2);
                expected +=
                    Complex64::from_polar(intensity / samples as f64, -(difference as f64) * phase);
            }
            let got = covariance[(12 + difference) as usize][12];
            assert!(
                (got - expected).norm() < 5e-5 * KT,
                "channel {difference} correlation {got}, quadrature {expected}"
            );
        }
    }
}

#[test]
fn periodic_noise_cancellation_and_resource_budget_do_not_publish_partial_results() {
    let netlist = Netlist::parse(
        "noise cancellation
P1 p 0 portnum=1 z0=50
R1 p 0 100
.end
",
    )
    .unwrap();
    let engine = Engine::default();
    let point = engine
        .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
        .unwrap();
    let counter = CountingAbort::new(usize::MAX);
    engine
        .prepare_psp_from_hb_with_noise_and_abort(
            &netlist,
            sweep(0),
            &point.operating_point,
            true,
            &NoAbort,
        )
        .unwrap()
        .run_with_abort(&counter)
        .unwrap();
    for threshold in [0, counter.count() / 2, counter.count() - 1] {
        let abort = CountingAbort::new(threshold);
        let error = engine
            .prepare_psp_from_hb_with_noise_and_abort(
                &netlist,
                sweep(0),
                &point.operating_point,
                true,
                &NoAbort,
            )
            .unwrap()
            .run_with_abort(&abort)
            .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted), "{error}");
        assert_eq!(abort.polls_after_abort(), 0);
    }
    let mut config = rspice_core::SimulationConfig::default();
    // Enough for the carrier and S matrix, but not scattering + covariance.
    config.resource_limits.max_result_values = 70;
    let engine = Engine::new(config);
    let point = engine
        .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
        .unwrap();
    engine
        .prepare_psp_from_hb_with_abort(&netlist, sweep(2), &point.operating_point, &NoAbort)
        .unwrap();
    let error = engine
        .prepare_psp_from_hb_with_noise_and_abort(
            &netlist,
            sweep(2),
            &point.operating_point,
            true,
            &NoAbort,
        )
        .err()
        .expect("joint noise and scattering need 100 scalar result values");
    assert!(
        error.to_string().contains("result_values limit exceeded"),
        "{error}"
    );
}
