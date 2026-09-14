//! The coupled IBM2 line has simultaneously diagonalizable R, L and C.
//! Its common/differential telegrapher modes provide an independent reference
//! for the small reflected tail, where historical CPL recurrences lose accuracy.
use rspice_core::engine::{ConvergenceConfig, SimulationConfig, SpiceDialect};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Engine, Netlist};

#[test]
fn coupled_line_tail_matches_independent_telegrapher_equations() {
    let source = include_str!("../../../tests/paranoia/TransmissionLines/cpl_ibm2.sp");
    let netlist = Netlist::parse(source).unwrap();
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig::robust(),
        integration_method: IntegrationMethod::Trapezoidal,
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    });
    let result = engine.run_tran(&netlist, 20e-9, 5e-12).unwrap();
    let reference = include_str!("../../../tests/qualification/cpl_ibm2_telegrapher.tsv");
    let columns = ["v1", "v2", "v3", "v4"].map(|node| {
        result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap()
    });
    let mut compared = 0;
    for line in reference
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
    {
        let row: Vec<f64> = line
            .split_whitespace()
            .map(|field| field.parse().unwrap())
            .collect();
        assert_eq!(row.len(), 5);
        let time = row[0];
        let upper = result.time.partition_point(|&sample| sample < time);
        assert!(upper > 0 && upper < result.time.len());
        let lower = upper - 1;
        let fraction = (time - result.time[lower]) / (result.time[upper] - result.time[lower]);
        for (port, &column) in columns.iter().enumerate() {
            let values = &result.voltages[column];
            let actual = values[lower] + fraction * (values[upper] - values[lower]);
            let expected = row[port + 1];
            // 2 uV absolute accuracy on a reflected tail of 0.1--1.6 mV.
            // Final reference Fourier/window refinements move these samples
            // by less than 30 nV. No ngspice values enter this comparison.
            assert!(
                (actual - expected).abs() < 2e-6,
                "v{} at {time:.8e}: {actual:.12e} vs {expected:.12e}",
                port + 1
            );
            compared += 1;
        }
    }
    assert_eq!(
        compared, 12,
        "all four ports at all three reference times must execute"
    );
}
