//! S-parameter extraction driven end to end by the real engine.
//!
//! A resistive divider has an S-matrix anyone can write down, so these settle
//! whether the whole path -- port discovery, normalization, excitation, AC
//! solve, wave extraction -- agrees with the physics rather than merely with
//! itself. The AC solve here is the engine, not a stub, which is why they live
//! out here: an analysis that reaches for the engine to test itself would put
//! `analysis` above the layer that drives it.

use rspice_core::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::s_param::{ExtractError, collect_ports, extract_s_matrix};
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

fn s_at_dc(deck: &str) -> Vec<Vec<Complex64>> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let ports = collect_ports(&netlist).expect("ports collect");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = vec![1.0];
    let s = extract_s_matrix(&netlist, &ports, &frequencies, |driven| {
        engine
            .run_ac(driven, &frequencies)
            .map_err(|error| error.to_string())
    })
    .expect("extraction succeeds");
    (0..ports.len())
        .map(|row| (0..ports.len()).map(|col| s[row][col][0]).collect())
        .collect()
}

/// `S11 = (R + Z2 - Z1)/(Z1 + R + Z2)`, `S21 = 2 sqrt(Z1 Z2)/(Z1 + R + Z2)`.
fn assert_series_resistor(s: &[Vec<Complex64>], r: f64, z1: f64, z2: f64) {
    let total = z1 + r + z2;
    let expected = [
        [(r + z2 - z1) / total, 2.0 * (z1 * z2).sqrt() / total],
        [2.0 * (z1 * z2).sqrt() / total, (r + z1 - z2) / total],
    ];
    for (row, values) in expected.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            assert!(
                (s[row][col].re - value).abs() < 1e-9 && s[row][col].im.abs() < 1e-9,
                "S[{row}][{col}] = {}, expected {value}",
                s[row][col]
            );
        }
    }
}

#[test]
fn xyce_port_elements_produce_the_closed_form_s_matrix() {
    let s = s_at_dc(
        "* series resistor between two P ports\n\
         P1 p1 0 PORT=1 Z0=75 AC 1\n\
         R1 p1 p2 50\n\
         P2 p2 0 PORT=2 Z0=50\n\
         .ac lin 1 1 1\n\
         .end\n",
    );
    assert_series_resistor(&s, 50.0, 75.0, 50.0);
}

/// The same network declared the ngspice way must measure the same, or the two
/// front-end spellings describe different circuits.
#[test]
fn annotated_sources_produce_the_same_s_matrix_as_port_elements() {
    let s = s_at_dc(
        "* series resistor between two annotated ports\n\
         V1 p1 0 DC 0 AC 1 portnum=1 z0=75\n\
         R1 p1 p2 50\n\
         V2 p2 0 DC 0 AC 0 portnum=2 z0=50\n\
         .ac lin 1 1 1\n\
         .end\n",
    );
    assert_series_resistor(&s, 50.0, 75.0, 50.0);
}

#[test]
fn missing_declared_port_node_fails_instead_of_becoming_zero_volts() {
    let netlist =
        Netlist::parse("* missing port result basis\nP1 p1 0 PORT=1 Z0=50\nR1 p1 0 50\n.end\n")
            .expect("deck parses");
    let ports = collect_ports(&netlist).expect("ports collect");
    let error = extract_s_matrix(&netlist, &ports, &[1.0], |_| {
        Ok(vec![AcResult {
            frequency: 1.0,
            node_names: vec!["different".to_string()],
            branch_names: Vec::new(),
            voltages: vec![Complex64::new(0.5, 0.0)],
            currents: Vec::new(),
        }])
    })
    .expect_err("a missing port coordinate cannot be measured as zero");

    assert!(matches!(
        error,
        ExtractError::MissingNodeVoltage { ref node, .. } if node.eq_ignore_ascii_case("p1")
    ));
}

#[test]
fn sp_preserves_resource_limits_reached_inside_the_ac_solve() {
    let netlist =
        Netlist::parse("* SP resource contract\nV1 p1 0 AC 1 portnum=1\nR1 p1 0 50\n.end\n")
            .unwrap();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_matrix_unknowns = 1;
    let error = Engine::new(config)
        .run_sp_over_grid_with_abort(&netlist, &[1.0], false, &rspice_core::NoAbort)
        .expect_err("port normalization exceeds the configured equation budget");
    assert!(
        matches!(error, SimulationError::ResourceLimit(_)),
        "{error:?}"
    );
}

#[test]
fn sp_bounds_the_full_port_cube_before_allocating_it() {
    let mut deck = String::from("* Five independently matched ports\n");
    for port in 1..=5 {
        deck.push_str(&format!(
            "P{port} p{port} 0 PORT={port} Z0=50\nR{port} p{port} 0 50\n",
        ));
    }
    deck.push_str(".end\n");
    let netlist = Netlist::parse(&deck).unwrap();
    let mut config = SimulationConfig::default();
    // Each AC solve fits (15 complex unknowns plus frequency), but the
    // assembled 5-by-5 complex S-matrix and its frequency need 51 values.
    config.resource_limits.max_result_values = 40;
    let outcome = Engine::new(config).run_sp_over_grid_with_abort(
        &netlist,
        &[1.0],
        false,
        &rspice_core::NoAbort,
    );
    let Err(SimulationError::ResourceLimit(limit)) = outcome else {
        panic!("the assembled S-matrix must respect the result budget: {outcome:?}");
    };
    assert_eq!(limit.resource, rspice_core::ResourceKind::ResultValues);
    assert_eq!(limit.requested, 51);
    assert_eq!(limit.limit, 40);
}

#[test]
fn xyce_named_gnd_port_is_an_ordinary_measured_node() {
    let netlist = Netlist::parse_with_options(
        "* GND is a real Xyce node\nP1 GND 0 PORT=1 Z0=50\nR1 GND 0 100\n.end\n",
        rspice_core::netlist::NetlistParseOptions {
            expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .unwrap();
    let result = Engine::default()
        .run_sp_over_grid_with_abort(&netlist, &[1.0], false, &rspice_core::NoAbort)
        .expect("an ordinary node named GND can be a port reference plane");
    let reflection = result.scattering.data[0].s11();
    assert!(
        (reflection - Complex64::new(1.0 / 3.0, 0.0)).norm() < 1e-12,
        "a 100-ohm load on a 50-ohm port must reflect one third, got {reflection}",
    );
}
