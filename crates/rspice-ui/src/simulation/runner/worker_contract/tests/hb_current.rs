use super::*;
use num_complex::Complex64;

#[test]
fn hb_current_spectra_preserve_phasors_units_and_direction_through_worker_transport() {
    let deck = "HB current outputs\nVDRIVE in 0 DC 1 SIN(1 2 1k)\nR1 in mid 100\nL1 mid out 10m\nC1 out 0 1u\nR2 out 0 1k\n.options GMIN=0\n.end\n";
    let spec = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1000.0, 3).with_source("VDRIVE")],
        reltol: 1e-9,
        abstol: 1e-12,
        max_iterations: 40,
        damping: 1.0,
        min_damping: 0.01,
        oversample: 2,
        collocation_points: None,
        max_mixing_order: 3,
        use_krylov: false,
        gmres_restart: 12,
        source_stepping: false,
        use_exact_jacobian: true,
        verbose: false,
    };
    let result = crate::simulation::runner::spec::run_spec_request(
        &crate::simulation::engine_bridge::EngineBridge::new(),
        spec,
        SpecExecutionOptions::default(),
        deck,
        None,
        &Default::default(),
        &rspice_core::abort_signal::NoAbort,
    )
    .unwrap();
    let response = WorkerResponse {
        id: 44,
        outcome: WorkerOutcome::Success(Box::new(
            WorkerSimulationResult::try_from(result).unwrap(),
        )),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let restored = transport.into_response().unwrap();
    assert_eq!(restored, response);
    let WorkerOutcome::Success(result) = restored.outcome else {
        panic!("HB success")
    };
    let result = SimulationResult::from(*result);
    let SimulationResult::HarmonicBalance {
        waveforms,
        frequencies,
        ..
    } = &result
    else {
        panic!("HB result")
    };
    assert_eq!(frequencies, &[0.0, 1000.0, 2000.0, 3000.0]);
    let wave = |name: &str| {
        waveforms
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .unwrap()
            .1
    };
    let value = |name: &str, index: usize| {
        let wave = wave(name);
        Complex64::new(wave.y_values[index], wave.y_imag.as_ref().unwrap()[index])
    };
    for name in ["I(VDRIVE)", "I(L1)", "I(C1)"] {
        assert_eq!(wave(name).y_unit, "A");
        assert_eq!(&wave(name).x_values, frequencies);
    }
    for name in ["V(in)", "V(mid)", "V(out)"] {
        assert_eq!(wave(name).y_unit, "V");
    }
    // The inductor is retained both as an MNA branch and as a reactive state.
    assert_eq!(
        waveforms
            .keys()
            .filter(|name| name.eq_ignore_ascii_case("I(L1)"))
            .count(),
        1
    );
    let omega = std::f64::consts::TAU * 1000.0;
    let load = Complex64::new(1.0, 0.0) / Complex64::new(0.001, omega * 1e-6);
    let branch = Complex64::new(0.0, -2.0) / (Complex64::new(100.0, omega * 0.01) + load);
    let output = branch * load;
    let capacitor = Complex64::new(0.0, omega * 1e-6) * output;
    for (name, expected) in [
        ("I(L1)", branch),
        ("I(VDRIVE)", -branch),
        ("I(C1)", capacitor),
        ("V(out)", output),
    ] {
        let actual = value(name, 1);
        assert!(
            (actual - expected).norm() < 1e-9,
            "{name}: {actual:?} != {expected:?}"
        );
    }
    assert!((value("I(L1)", 0).re - 1.0 / 1100.0).abs() < 1e-12);
    assert!((value("I(VDRIVE)", 0).re + 1.0 / 1100.0).abs() < 1e-12);
    assert_eq!(value("I(C1)", 0).norm(), 0.0);
    assert!((value("I(L1)", 1) - value("I(C1)", 1) - value("V(out)", 1) / 1000.0).norm() < 1e-10);
    let measured = result
        .study_measurement("bin:1:imag:I(VDRIVE)")
        .unwrap()
        .value
        .unwrap();
    assert!((measured + branch.im).abs() < 1e-9);
}
