//! S-parameter extraction driven end to end by the real engine.
//!
//! A resistive divider has an S-matrix anyone can write down, so these settle
//! whether the whole path -- port discovery, normalization, excitation, AC
//! solve, wave extraction -- agrees with the physics rather than merely with
//! itself. The AC solve here is the engine, not a stub, which is why they live
//! out here: an analysis that reaches for the engine to test itself would put
//! `analysis` above the layer that drives it.

use rspice_core::Complex64;
use rspice_core::analysis::s_param::{collect_ports, extract_s_matrix};
use rspice_core::engine::{Engine, SimulationConfig};
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
