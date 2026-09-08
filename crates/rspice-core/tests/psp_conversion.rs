//! Periodic power waves must describe the DUT, independently of the physical
//! source terminations, and retain conversion between signed sidebands.

use rspice_core::abort_signal::{CountingAbort, NoAbort};
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::engine::{Engine, PspAnalysisResult, SimulationError};
use rspice_core::{Complex64, Netlist};

fn sweep(sidebands: i32) -> PacConfig {
    PacConfig::new()
        .with_sweep(1e4, 1e4, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-sidebands, sidebands)
}

fn hbsp(deck: &str, sidebands: i32) -> PspAnalysisResult {
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::default();
    let point = engine
        .run_hb(
            &netlist,
            HbConfig::new(1e6)
                .with_harmonics((sidebands as usize * 2).max(8))
                .with_oversample(4),
        )
        .unwrap();
    engine
        .prepare_psp_from_hb_with_abort(
            &netlist,
            sweep(sidebands),
            &point.operating_point,
            &NoAbort,
        )
        .unwrap()
        .run_with_abort(&NoAbort)
        .unwrap()
}

#[test]
fn unequal_multiplied_periodic_ports_measure_an_ideal_through() {
    let data = hbsp(
        "* unequal physical terminations and wave references
.subckt generator a b params: number=1 reference=50
P1 a b portnum={number} z0={reference}
.ends
X1 p 0 generator M=2 number=1 reference=50
X2 q 0 generator M=0.5 number=2 reference=75
VTHRU p q 0
.end
",
        1,
    );
    assert_eq!(
        data.ports.iter().map(|p| p.z0).collect::<Vec<_>>(),
        [50.0, 75.0]
    );
    assert_eq!(data.fundamental_freq, 1e6);
    let matrix = &data.data[0];
    let transmission = 2.0 * (50.0_f64 * 75.0).sqrt() / 125.0;
    for output in 0..6 {
        for input in 0..6 {
            let expected = if output % 3 != input % 3 {
                0.0
            } else if output / 3 != input / 3 {
                transmission
            } else if output < 3 {
                0.2
            } else {
                -0.2
            };
            assert!(
                (matrix.get(output + 1, input + 1) - Complex64::new(expected, 0.0)).norm() < 1e-12
            );
        }
    }
}

#[test]
fn periodic_one_port_supports_voltage_annotations_and_differential_polarity() {
    for source in ["V", "P"] {
        for plane in ["p 0", "0 p", "p n"] {
            let data = hbsp(
                &format!(
                    "* one-port load\n{source}1 {plane} DC 0 portnum=1 z0=50\nR1 {plane} 100\nRREF n 0 1\n.end\n"
                ),
                0,
            );
            assert_eq!(data.ports.len(), 1);
            assert!(
                (data.data[0].s11() - Complex64::new(1.0 / 3.0, 0.0)).norm() < 1e-12,
                "{source}, {plane}: {}",
                data.data[0].s11()
            );
        }
    }
}

#[test]
fn periodic_chopper_converts_sidebands_and_renormalizes_the_whole_coupled_matrix() {
    let deck = |multiplicity| {
        format!(
            "* switched two-port
.subckt generator p
P1 p 0 portnum=1 z0=50
.ends
X1 in generator M={multiplicity}
P2 out 0 portnum=2 z0=50
VLO ctl 0 sin(0 1 1meg)
S1 in out ctl 0 swmod
C1 out 0 1f
.model swmod sw vt=0 ron=1 roff=1e9 smooth=1m
.end
"
        )
    };
    // Same carrier and DUT, but a different port termination in each actual
    // producer circuit. A block-by-block sideband renormalization fails here.
    let matched = hbsp(&deck(1), 8);
    let multiplied = hbsp(&deck(2), 8);
    let expected_conversion = (100.0 / 101.0) / std::f64::consts::PI;
    let bands = 17;
    let upper = matched.data[0].get(bands + 10, 9).norm();
    let lower = matched.data[0].get(bands + 8, 9).norm();
    assert!(
        (upper / expected_conversion - 1.0).abs() < 0.03,
        "upper {upper}"
    );
    assert!(
        (lower / expected_conversion - 1.0).abs() < 0.03,
        "lower {lower}"
    );
    for output in 1..=2 * bands {
        for input in 1..=2 * bands {
            let a = matched.data[0].get(output, input);
            let b = multiplied.data[0].get(output, input);
            assert!((a - b).norm() < 1e-9, "S[{output},{input}]: {a} != {b}");
        }
    }
}

#[test]
fn periodic_ports_authenticate_the_producer_and_cancel_without_publishing_partial_data() {
    let text = "* one-port\nP1 p 0 portnum=1 z0=50\nR1 p 0 100\n.end\n";
    let netlist = Netlist::parse(text).unwrap();
    let engine = Engine::default();
    let point = engine
        .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
        .unwrap();
    let changed = Netlist::parse(&text.replace("R1 p 0 100", "R1 p 0 200")).unwrap();
    let error = engine
        .prepare_psp_from_hb_with_abort(&changed, sweep(0), &point.operating_point, &NoAbort)
        .err()
        .expect("changed DUT cannot consume the old carrier");
    assert!(error.to_string().contains("identity"), "{error}");
    let count = CountingAbort::new(usize::MAX);
    engine
        .prepare_psp_from_hb_with_abort(&netlist, sweep(1), &point.operating_point, &NoAbort)
        .unwrap()
        .run_with_abort(&count)
        .unwrap();
    for threshold in 0..count.count() {
        let abort = CountingAbort::new(threshold);
        let error = engine
            .prepare_psp_from_hb_with_abort(&netlist, sweep(1), &point.operating_point, &NoAbort)
            .unwrap()
            .run_with_abort(&abort)
            .unwrap_err();
        assert!(
            matches!(error, SimulationError::Aborted),
            "poll {threshold}: {error}"
        );
        assert_eq!(abort.polls_after_abort(), 0, "poll {threshold}");
    }
}

#[test]
fn periodic_port_output_budget_counts_all_input_and_output_sidebands() {
    let mut config = rspice_core::SimulationConfig::default();
    config.resource_limits.max_result_values = 512;
    let engine = Engine::new(config);
    let netlist =
        Netlist::parse("* bounded one port\nP1 p 0 portnum=1 z0=50\nR1 p 0 100\n.end\n").unwrap();
    let point = engine
        .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(16))
        .unwrap();
    // HB and PAC node spectra fit. The port matrix is 17 x 17 complex values.
    let error = engine
        .prepare_psp_from_hb_with_abort(&netlist, sweep(8), &point.operating_point, &NoAbort)
        .err()
        .expect("the complete scattering matrix exceeds the result budget");
    let SimulationError::ResourceLimit(limit) = error else {
        panic!("unexpected error: {error}")
    };
    assert_eq!(limit.resource, rspice_core::ResourceKind::ResultValues);
    assert_eq!(limit.requested, 578);
    assert_eq!(limit.limit, 512);
}
