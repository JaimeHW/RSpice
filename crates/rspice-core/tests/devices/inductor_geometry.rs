//! Analytical qualification for turns/geometry-based linear inductors.

use rspice_core::{Complex64, Engine, Netlist};

#[test]
fn geometry_synthesized_inductor_has_the_analytical_ac_impedance() {
    let netlist = Netlist::parse(
        "finite solenoid impedance\n\
         I1 0 out DC 0 AC 1\n\
         L1 out 0 coil NT=34 SCALE=2 M=4 TEMP=77\n\
         .MODEL coil L DIA=12.7m CSECT=1p LENGTH=1.27m NT=17 MU=1 TC1=1m TNOM=27\n\
         .AC LIN 1 1k 1k\n\
         .END\n",
    )
    .expect("geometry inductor deck parses");
    let point = Engine::default()
        .run_ac(&netlist, &[1.0e3])
        .expect("geometry inductor AC solve succeeds")
        .pop()
        .expect("one AC point");
    let output = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .map(|index| point.voltages[index])
        .expect("output node is present");

    const INDUCTANCE_ORACLE: f64 = 7.365_272_776_620_191_5e-6 * 2.1;
    let expected = Complex64::new(0.0, 2.0 * std::f64::consts::PI * 1.0e3 * INDUCTANCE_ORACLE);
    let tolerance = 128.0 * f64::EPSILON * expected.norm();
    assert!(
        (output - expected).norm() <= tolerance,
        "geometry impedance mismatch: actual={output:?}, expected={expected:?}, tolerance={tolerance:.3e}"
    );
}
