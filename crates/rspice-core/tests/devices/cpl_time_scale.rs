//! Changing only the time unit of an RLC circuit must preserve its waveform.
use rspice_core::engine::{ConvergenceConfig, SimulationConfig, SpiceDialect};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Engine, Netlist};

fn simulate(scale: f64) -> rspice_core::analysis::transient::TransientResult {
    let source = format!(
        "CPL time similarity\n\
         V1 in1 0 PULSE(0 1 {delay:.17e} {rise:.17e} {rise:.17e} {width:.17e} {period:.17e})\n\
         V2 in2 0 0\n\
         R1 in1 near1 25\nR2 in2 near2 25\n\
         P1 near1 near2 0 far1 far2 0 cp\n\
         RL1 far1 0 50\nRL2 far2 0 50\n\
         .model cp CPL\n+ R = 2.25 0 2.25\n\
         + L = {l:.17e} {lm:.17e} {l:.17e}\n\
         + G = 0 0 0\n+ C = {c:.17e} {cm:.17e} {c:.17e}\n+ length = .03\n\
         .tran {step:.17e} {stop:.17e}\n.end\n",
        delay = 2e-9 * scale,
        rise = 0.4e-9 * scale,
        width = 2e-9 * scale,
        period = 8e-9 * scale,
        l = 0.6e-6 * scale,
        lm = 0.05e-6 * scale,
        c = 1.2e-9 * scale,
        cm = -0.11e-9 * scale,
        step = 5e-12 * scale,
        stop = 6e-9 * scale,
    );
    let netlist = Netlist::parse(&source).unwrap();
    let engine = Engine::new(SimulationConfig {
        min_timestep: 1e-20 * scale,
        transient_initial_timestep: Some(5e-12 * scale),
        locked_time_grid: Some(std::sync::Arc::new(
            (0..=1200)
                .map(|sample| (6e-9 * sample as f64 / 1200.0) * scale)
                .collect(),
        )),
        convergence_config: ConvergenceConfig::robust(),
        integration_method: IntegrationMethod::Trapezoidal,
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    });
    engine
        .run_tran(&netlist, 6e-9 * scale, 5e-12 * scale)
        .unwrap()
}

#[test]
fn coupled_line_preserves_waveforms_below_one_picosecond() {
    let ordinary = simulate(1.0);
    // A power of two changes the binary time unit exactly. Scaling L and C
    // by the same factor preserves impedance, attenuation and normalized
    // delay; no reference-simulator rounding enters this identity.
    let scale = 1.0 / 1024.0;
    let compressed = simulate(scale);
    assert_eq!(ordinary.time.len(), 1201);
    assert_eq!(compressed.time.len(), ordinary.time.len());
    for (&time, &compressed_time) in ordinary.time.iter().zip(&compressed.time) {
        assert_eq!(
            compressed_time / scale,
            time,
            "the comparison requires identical normalized grids"
        );
    }
    let mut failures = Vec::new();
    for node in ["in1", "near1", "far1", "far2"] {
        let left = ordinary
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(node))
            .unwrap();
        let right = compressed
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(node))
            .unwrap();
        let mut maximum_error = 0.0_f64;
        let mut worst = (0.0, 0.0, 0.0);
        for sample in 0..=1200 {
            let time = ordinary.time[sample];
            let expected = ordinary.voltages[left][sample];
            let actual = compressed.voltages[right][sample];
            assert!(expected.is_finite() && actual.is_finite());
            if (actual - expected).abs() > maximum_error {
                maximum_error = (actual - expected).abs();
                worst = (time, expected, actual);
            }
        }
        eprintln!("{node}: error={maximum_error:.6e}, worst={worst:?}");
        if maximum_error >= 1e-8 {
            failures.push((node, maximum_error));
        }
    }
    let driven = ordinary
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("far1"))
        .unwrap();
    assert!(
        ordinary.voltages[driven]
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            > 0.5
    );
    assert!(failures.is_empty(), "time-scaling failures: {failures:?}");
}
