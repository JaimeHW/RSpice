//! Stateless behavioral expressions across the shared periodic MNA solver.
use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::{K_BOLTZMANN, TEMP_REFERENCE};
use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() < 1e-8 * expected.norm().max(1e-3),
        "{actual} vs {expected}"
    );
}

#[test]
fn behavioral_periodic_hb_retains_nonlinear_voltage_and_branch_controlled_current() {
    let netlist = Netlist::parse(
        "Nonlinear behavioral harmonics\n\
        v1 drive 0 sin(0 1 1k)\nbvol out 0 v=v(drive)*v(drive)\nrload out 0 1k\n\
        bcur sink 0 i=0.002*v(drive)*v(drive)+i(bvol)\nrsink sink 0 1k\n.end\n",
    )
    .unwrap();
    for krylov in [false, true] {
        let mut config = HbConfig::new(1e3).with_harmonics(3).with_tolerance(1e-9);
        config.use_krylov = krylov;
        let result = Engine::default().run_hb(&netlist, config).unwrap();
        assert!(result.converged);
        for (node, sign) in [("out", 1.0), ("sink", -1.0)] {
            let spectrum = &result
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(node))
                .unwrap()
                .coefficients;
            close(spectrum[0], Complex64::new(sign * 0.5, 0.0));
            close(spectrum[1], Complex64::ZERO);
            close(spectrum[2], Complex64::new(-sign * 0.5, 0.0));
        }
        let current = &result
            .device_currents
            .iter()
            .find(|row| row.probe.eq_ignore_ascii_case("I(bcur)"))
            .unwrap()
            .coefficients;
        close(current[0], Complex64::new(0.5e-3, 0.0));
        close(current[2], Complex64::new(-0.5e-3, 0.0));
    }
}

#[test]
fn behavioral_periodic_conversion_and_noise_use_the_modulated_expression_jacobian() {
    let netlist = Netlist::parse(
        "Behavioral mixer\n\
        vdrive ctrl 0 sin(0 1 1k)\nrnoise noise 0 1k\nitest 0 noise dc 0 ac 1\n\
        bvol out 0 v=(1+v(ctrl))*v(noise)\nrload out 0 1k noisy=0\n\
        bcur sink 0 i=i(bvol)\nrsink sink 0 1k noisy=0\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let pac = engine
        .run_pac(
            &netlist,
            PacConfig::new()
                .with_fundamental(1e3)
                .with_sweep(10.0, 10.0, 1)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-1, 1)
                .with_input_source("itest")
                .with_output_node("sink"),
        )
        .unwrap();
    for (sideband, expected) in [
        (-1, Complex64::new(0.0, 500.0)),
        (0, Complex64::new(1000.0, 0.0)),
        (1, Complex64::new(0.0, -500.0)),
    ] {
        close(
            pac.result.conversion_matrix.get(0, sideband, 0).unwrap(),
            expected,
        );
    }
    let noise = engine
        .run_pnoise(&netlist, 1e3, &[10.0], "sink", None, Some("itest"), 1)
        .unwrap();
    let expected = 4.0 * K_BOLTZMANN * TEMP_REFERENCE * 1e3 * 1.5;
    assert!((noise.output_noise[0] / expected - 1.0).abs() < 1e-8);
}

#[test]
fn behavioral_periodic_rejects_unrepresented_clocks_and_integrals() {
    for expression in ["sin(2*pi*1k*time)", "sdt(v(out))", "frequency*v(out)"] {
        let netlist = Netlist::parse(&format!(
            "Behavioral state boundary\nb1 out 0 i={expression}\nr1 out 0 1k\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
            .expect_err("unrepresented time/frequency/state must not be sampled at zero")
            .to_string();
        assert!(
            error.contains("explicit time/frequency forcing or accepted-step memory"),
            "{error}"
        );
    }
}

#[test]
fn behavioral_quasiperiodic_mixer_preserves_independent_tone_products() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::QpssConfig;
    let netlist = Netlist::parse(
        "Behavioral two-tone mixer\n\
        v1 a 0 sin(0 1 1k)\nv2 b 0 sin(0 1 1414.213562373095)\n\
        bvol out 0 v=v(a)*v(b)\nrload out 0 1k\n\
        bcur sink 0 i=i(bvol)\nrsink sink 0 1k\n.end\n",
    )
    .unwrap();
    let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![8, 8]);
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
            .unwrap();
    let result = Engine::default().run_qpss(&netlist, config).unwrap();
    for node in ["out", "sink"] {
        let row = result
            .node_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap();
        for (tuple, value) in [([1, 1], -0.25), ([1, -1], 0.25), ([0, 0], 0.0)] {
            close(
                result.spectra()[row][grid.index_of(&tuple).unwrap()],
                Complex64::new(value, 0.0),
            );
        }
    }
}
