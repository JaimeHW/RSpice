//! IC branch identity and physical lead-current equations across periodic solves.
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::config::SpiceDialect;
use rspice_core::engine::{Engine, QpssConfig, SimulationConfig};
use rspice_core::{Complex64, Netlist, NoAbort, analysis::HbConfig};
use std::f64::consts::{SQRT_2, TAU};

fn response(frequency: f64) -> Complex64 {
    Complex64::new(1., 0.) / Complex64::new(1., TAU * frequency * 1e-4)
}

#[test]
fn fixed_capacitor_ic_branch_preserves_current_controls_and_periodic_response() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let deck = |drive| {
        Netlist::parse(&format!(
            "IC capacitor current\nVdrive in mid SIN(0 .2 1k)\nVsecond mid 0 {drive}\n\
        R1 in out 1k\nC1 out 0 100n IC=.7\nBcopy sense 0 I={{I(C1)}}\nRsense sense 0 1k\n.end\n"
        ))
        .unwrap()
    };
    let netlist = deck("0");
    let hb = engine
        .run_hb(&netlist, HbConfig::new(1e3).with_harmonics(2))
        .unwrap();
    let out = hb
        .result
        .spectral_voltages
        .iter()
        .find(|row| row.node_name.eq_ignore_ascii_case("out"))
        .unwrap();
    let sense = hb
        .result
        .spectral_voltages
        .iter()
        .find(|row| row.node_name.eq_ignore_ascii_case("sense"))
        .unwrap();
    let branch = hb
        .result
        .mna_branch_currents
        .iter()
        .find(|row| row.device_name.eq_ignore_ascii_case("C1"))
        .unwrap();
    let expected = Complex64::new(0., -0.2) * response(1e3);
    let current = Complex64::new(0., TAU * 1e3 * 1e-7) * expected;
    assert!(out.coefficients[0].norm() < 1e-10); // The IC is not a DC voltage constraint.
    assert!((out.coefficients[1] - expected).norm() < 1e-10);
    assert!((branch.coefficients[1] - current).norm() < 1e-12);
    assert!((sense.coefficients[1] + 1e3 * current).norm() < 1e-10);
    let config = PacConfig::new()
        .with_fundamental(1e3)
        .with_sweep(130., 130., 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("Vdrive")
        .with_output_node("out");
    let pac = engine
        .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
        .unwrap();
    for input in -1..=1 {
        for output in -1..=1 {
            let expected = if input == output {
                response(130. + input as f64 * 1e3)
            } else {
                Complex64::ZERO
            };
            assert!(
                (pac.result.conversion_matrix.get(0, output, input).unwrap() - expected).norm()
                    < 1e-10
            );
        }
    }
    let second = 1e3 * SQRT_2;
    let netlist = deck(&format!("SIN(0 .1 {second})"));
    let point = engine
        .run_qpss(&netlist, QpssConfig::new(vec![1e3, second], vec![1, 1]))
        .unwrap();
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let branch = point.node_names().len()
        + point
            .branch_names()
            .iter()
            .position(|name| name.eq_ignore_ascii_case("C1"))
            .unwrap();
    let sense = point
        .node_names()
        .iter()
        .position(|name| name.eq_ignore_ascii_case("sense"))
        .unwrap();
    for (tuple, amplitude, frequency) in [([1, 0], 0.2, 1e3), ([0, 1], 0.1, second)] {
        let index = grid.index_of(&tuple).unwrap();
        let voltage = Complex64::new(0., -amplitude / 2.) * response(frequency);
        let current = Complex64::new(0., TAU * frequency * 1e-7) * voltage;
        assert!((point.spectra()[branch][index] - current).norm() < 1e-12);
        assert!((point.spectra()[sense][index] + 1e3 * current).norm() < 1e-10);
    }
}
