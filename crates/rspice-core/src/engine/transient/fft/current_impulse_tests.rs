use super::*;
use crate::abort_signal::NoAbort;
use crate::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};

fn fixture() -> TransientResult {
    TransientResult {
        time: (0..=8).map(|n| n as Value / 8.0).collect(),
        step_sizes: vec![0.125; 9],
        voltages: vec![vec![3.0; 9]],
        branch_currents: vec![vec![0.0; 9]],
        num_nodes: 1,
        node_names: vec!["out".into()],
        branch_names: vec!["V1".into()],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        device_op_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        current_impulses: Some(vec![CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: vec![CurrentImpulsePoint {
                time: 0.3,
                charge_coulombs: 0.002,
            }],
        }]),
    }
}

fn spectrum(result: &TransientResult, card: &str) -> Result<TransientFftResult, SimulationError> {
    let netlist = Netlist::parse(&format!(
        "* impulse FFT\nV1 out 0 0\nR1 out 0 1k\n{card}\n.end\n"
    ))
    .unwrap();
    let engine = Engine::new(Default::default());
    evaluate(&engine, &netlist, result, 1.0, &NoAbort).map(|mut results| results.remove(0))
}

fn close(actual: Value, expected: Value) {
    assert!((actual - expected).abs() < 2e-14, "{actual} != {expected}");
}

#[test]
fn current_fft_uses_exact_charge_time_without_snapping_to_the_sample_grid() {
    for points in [8, 16, 64] {
        let fft = spectrum(
            &fixture(),
            &format!(".fft I(V1) np={points} format=unorm window=rect"),
        )
        .unwrap();
        close(fft.bins[0].real, 0.002);
        for bin in &fft.bins[1..] {
            let scale = if bin.index == points / 2 {
                0.002
            } else {
                0.004
            };
            let phase = -2.0 * PI * bin.index as Value * 0.3;
            close(bin.real, scale * phase.cos());
            close(bin.imaginary, scale * phase.sin());
        }
    }
}

#[test]
fn current_fft_combines_finite_and_affine_charge_before_normalization_and_metrics() {
    let fft = spectrum(
        &fixture(),
        ".options fft fftout=1\n.fft {2*I(V1)+V(out)} np=8 format=norm window=rect",
    )
    .unwrap();
    close(fft.bins[0].real, 1.0);
    close(fft.bins[1].magnitude, 0.008 / 3.004);
    close(fft.bins[1].phase_degrees, -108.0);
    let metrics = fft.metrics.unwrap();
    close(metrics.fundamental_magnitude, 0.008 / 3.004);
    close(metrics.thd_ratio, 1.5); // bins 2, 3 and half-weight Nyquist.
}

#[test]
fn current_fft_hann_hamming_and_blackman_follow_both_window_conventions() {
    for (window, a0, a1, a2) in [
        ("hann", 0.5, 0.5, 0.0),
        ("hamming", 0.54, 0.46, 0.0),
        ("blackman", 0.42, 0.5, 0.08),
    ] {
        for mode in [0, 1] {
            let mut result = fixture();
            result.current_impulses.as_mut().unwrap()[0].points[0].time = 0.3125;
            let fft = spectrum(
                &result,
                &format!(
                    ".options fft fft_mode={mode}\n.fft I(V1) np=8 format=unorm window={window}"
                ),
            )
            .unwrap();
            let x = 2.5 / if mode == 1 { 8.0 } else { 7.0 };
            let window = a0 - a1 * (2.0 * PI * x).cos() + a2 * (4.0 * PI * x).cos();
            let gain = if mode == 1 { a0 } else { a0 - (a1 - a2) / 8.0 };
            close(fft.coherent_gain, gain);
            close(fft.bins[0].real, 0.002 * window / gain);
            close(fft.bins[1].magnitude, 0.004 * window / gain);
            close(fft.bins[1].phase_degrees, -112.5);
        }
    }
}

#[test]
fn current_fft_periodic_bartlett_has_a_continuous_peak_between_samples() {
    for fraction in [0.40625, 0.4375, 0.46875, 0.5, 0.53125] {
        let mut result = fixture();
        result.current_impulses.as_mut().unwrap()[0].points[0].time = fraction;
        let fft = spectrum(
            &result,
            ".options fft fft_mode=1\n.fft I(V1) np=8 format=unorm window=bartlett",
        )
        .unwrap();
        close(
            fft.bins[0].real,
            0.002 * (1.0 - (2.0 * fraction - 1.0).abs()) / 0.5,
        );
    }
}

#[test]
fn current_fft_symmetric_tapers_end_at_the_last_sample_and_periodic_windows_wrap() {
    let mut result = fixture();
    result.current_impulses.as_mut().unwrap()[0].points = vec![
        CurrentImpulsePoint {
            time: 0.0,
            charge_coulombs: 99.0,
        },
        CurrentImpulsePoint {
            time: 0.9,
            charge_coulombs: 0.002,
        },
        CurrentImpulsePoint {
            time: 1.0,
            charge_coulombs: 0.003,
        },
    ];
    let symmetric = spectrum(
        &result,
        ".options fft fft_mode=0\n.fft I(V1) np=8 format=unorm window=hamming",
    )
    .unwrap();
    assert!(symmetric.bins.iter().all(|bin| bin.magnitude == 0.0));
    let periodic = spectrum(
        &result,
        ".options fft fft_mode=1\n.fft I(V1) np=8 format=unorm window=hamming",
    )
    .unwrap();
    let expected = (0.002 * (0.54 - 0.46 * (1.8 * PI).cos()) + 0.003 * 0.08) / 0.54;
    close(periodic.bins[0].real, expected);
}

#[test]
fn current_fft_restart_seam_is_not_replayed_and_stop_charge_is_included() {
    let mut full = fixture();
    full.current_impulses.as_mut().unwrap()[0].points = [(0.5, 99.0), (0.75, 0.002), (1.0, 0.003)]
        .into_iter()
        .map(|(time, charge_coulombs)| CurrentImpulsePoint {
            time,
            charge_coulombs,
        })
        .collect();
    let card = ".fft I(V1) np=8 format=unorm start=.5 stop=1";
    let expected = spectrum(&full, card).unwrap();
    close(expected.bins[0].real, 0.01);
    close(expected.bins[1].real, 0.004);
    let mut resumed = full;
    resumed.time.drain(..4);
    resumed.voltages[0].drain(..4);
    resumed.branch_currents[0].drain(..4);
    resumed.current_impulses.as_mut().unwrap()[0]
        .points
        .remove(0);
    assert_eq!(spectrum(&resumed, card).unwrap(), expected);
}

#[test]
fn current_fft_lead_and_projection_aliases_share_the_impulse_owner() {
    let mut result = fixture();
    result
        .device_op_traces
        .push(crate::engine::TransientDeviceOpTrace {
            device_name: "Q1".into(),
            parameter: "ic".into(),
            values: vec![0.0; 9],
        });
    result
        .current_impulses
        .as_mut()
        .unwrap()
        .push(CurrentImpulseTrace {
            owner: CurrentImpulseOwner::DeviceLead {
                device_name: "Q1".into(),
                parameter: "ic".into(),
            },
            complete: true,
            points: vec![CurrentImpulsePoint {
                time: 0.3,
                charge_coulombs: -0.001,
            }],
        });
    for (probe, charge) in [
        ("IR(V1)", 0.002),
        ("IC(Q1)", -0.001),
        ("@Q1[ic]", -0.001),
        ("N(Q1:ic)", -0.001),
        ("{2*I(V1)+IC(Q1)}", 0.003),
    ] {
        close(
            spectrum(&result, &format!(".fft {probe} np=8 format=unorm"))
                .unwrap()
                .bins[0]
                .real,
            charge,
        );
    }
}

#[test]
fn current_fft_rejects_incomplete_coverage_and_undefined_products() {
    let card = ".fft I(V1) np=8 format=unorm";
    for kind in 0..3 {
        let mut result = fixture();
        match kind {
            0 => result.current_impulses = Some(vec![]),
            1 => result.current_impulses.as_mut().unwrap()[0].complete = false,
            _ => result.current_impulses.as_mut().unwrap()[0].points[0].time = Value::NAN,
        }
        assert!(spectrum(&result, card).is_err());
        close(
            spectrum(&result, ".fft V(out) np=8 format=unorm")
                .unwrap()
                .bins[0]
                .real,
            3.0,
        );
    }
    assert!(
        spectrum(&fixture(), ".fft {I(V1)*I(V1)} np=8 format=unorm")
            .unwrap_err()
            .to_string()
            .contains("affine")
    );
    let mut zero = fixture();
    zero.current_impulses.as_mut().unwrap()[0].points.clear();
    zero.branch_currents[0].fill(1.0);
    close(
        spectrum(&zero, ".fft {I(V1)*I(V1)} np=8 format=unorm")
            .unwrap()
            .bins[0]
            .real,
        1.0,
    );
    zero.branch_currents.clear();
    assert!(spectrum(&zero, card).is_err());
    let mut legacy = fixture();
    legacy.current_impulses = None;
    close(spectrum(&legacy, card).unwrap().bins[0].real, 0.0);
}

#[test]
fn current_fft_requires_charge_coverage_after_the_last_dft_sample() {
    let mut result = fixture();
    result.time.pop();
    result.voltages[0].pop();
    result.branch_currents[0].pop();
    result.current_impulses.as_mut().unwrap()[0].points.clear();
    assert!(
        spectrum(&result, ".fft I(V1) np=8 format=unorm")
            .unwrap_err()
            .to_string()
            .contains("entire requested")
    );
    assert!(
        spectrum(&result, ".fft V(out) np=8 format=unorm")
            .unwrap()
            .status
            .is_complete()
    );
}

#[test]
fn current_fft_subnormal_charge_is_scaled_without_an_intermediate_sample_peak() {
    let mut result = fixture();
    let duration = 1e-300;
    for time in &mut result.time {
        *time *= duration;
    }
    let point = &mut result.current_impulses.as_mut().unwrap()[0].points[0];
    point.time *= duration;
    point.charge_coulombs = Value::from_bits(1);
    let expected = point.charge_coulombs / duration;
    let fft = spectrum(&result, ".fft I(V1) np=8 stop=1e-300 format=unorm").unwrap();
    close(fft.bins[0].real / expected, 1.0);
    close(fft.bins[1].magnitude / (2.0 * expected), 1.0);
}

#[test]
fn current_fft_charge_accumulation_has_typed_cancellation() {
    use crate::analysis::measure_signals::current_observation::{
        CurrentImpulseContribution, CurrentObservationError,
    };
    let result = fixture();
    let terms = [CurrentImpulseContribution {
        trace: &result.current_impulses.as_ref().unwrap()[0],
        weight: 1.0,
    }];
    let netlist = Netlist::parse("* FFT cancellation\n.fft I(V1) np=8\n.end\n").unwrap();
    let mut bins = vec![Complex::new(0.0, 0.0); 5];
    let abort = crate::abort_signal::CountingAbort::new(1);
    assert!(matches!(
        current_impulses::add_to_bins(
            &mut bins,
            &terms,
            &netlist.fft_analyses[0],
            XyceFftMode::HspiceCompatible,
            1.0,
            1.0,
            &abort
        ),
        Err(CurrentObservationError::Aborted)
    ));
}
