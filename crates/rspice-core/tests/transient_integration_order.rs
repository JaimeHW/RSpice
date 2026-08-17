//! Second-order integration has to survive contact with the device families
//! that carry their own charge truncation. The order-two trial after a
//! breakpoint is decided by a truncation walk; a family missing from that
//! walk leaves the run on backward Euler, and backward Euler announces itself
//! as a stage delay that moves with the step size.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_core::numerics::integration::IntegrationMethod;

/// Two BSIM3 NAND stages behind an ideal edge, on a uniform grid with the
/// LTE controller effectively disabled (`trtol=1e6`) so the accepted steps
/// are the ceiling and nothing else. Only the integration formula is left to
/// vary between the two runs.
fn nand_chain(max_step: &str) -> String {
    format!(
        "\
* two BSIM3 NAND stages, uniform {max_step} grid
.SUBCKT NAND in1 in2 out VDD
M1 out in2 Vdd Vdd p1 W=7.5u L=0.35u pd=13.5u ad=22.5p ps=13.5u as=22.5p
M2 net.1 in2 0 0 n1   W=3u   L=0.35u pd=9u    ad=9p    ps=9u    as=9p
M3 out in1 Vdd Vdd p1 W=7.5u L=0.35u pd=13.5u ad=22.5p ps=13.5u as=22.5p
M4 out in1 net.1 0 n1 W=3u   L=0.35u pd=9u    ad=9p    ps=9u    as=9p
.ENDS NAND
VCC 99 0 DC 3.3V
VIN1 1 0 DC 0 PULSE(0 3.3 2NS 10PS 10PS 10NS 20NS)
VIN2 2 0 DC 3.3
X1 1 2 3 99 NAND
X2 3 2 4 99 NAND
X3 4 2 5 99 NAND
.model n1 nmos level=49 version=3.3.0
.model p1 pmos level=49 version=3.3.0
.options trtol=1e6
.TRAN {max_step} 6NS 0 {max_step}
.end
"
    )
}

/// Time at which node `name` crosses half the rail, by linear interpolation
/// between the two accepted points that bracket it.
fn half_rail_crossing(result: &rspice_core::engine::TransientResult, name: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|node| node == name)
        .unwrap_or_else(|| panic!("node {name} in {:?}", result.node_names));
    let series = &result.voltages[index];
    for k in 1..result.time.len() {
        let (previous, current) = (series[k - 1], series[k]);
        if (previous - 1.65) * (current - 1.65) < 0.0 {
            let (t0, t1) = (result.time[k - 1], result.time[k]);
            return t0 + (1.65 - previous) * (t1 - t0) / (current - previous);
        }
    }
    panic!("{name} never crossed half rail");
}

fn third_stage_delay(max_step: &str) -> f64 {
    let netlist = Netlist::parse(&nand_chain(max_step)).expect("deck parses");
    let ceiling: f64 = match max_step {
        "5p" => 5.0e-12,
        "50p" => 50.0e-12,
        other => panic!("unmapped step {other}"),
    };
    let engine = Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Trapezoidal,
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    });
    let result = engine
        .run_tran(&netlist, 6.0e-9, ceiling)
        .expect("transient solves");
    half_rail_crossing(&result, "5")
}

/// Trapezoidal error is second order, so a tenfold step change moves a
/// stage delay by well under a picosecond on this chain — ngspice-46 gives
/// 3.7482 ns at 5 ps and 3.7488 ns at 50 ps. Backward Euler is first order
/// and moved it 15 ps here before the order-two trial could see BSIM3.
#[test]
fn bsim3_chain_delay_is_step_independent_at_second_order() {
    let fine = third_stage_delay("5p");
    let coarse = third_stage_delay("50p");
    let drift = (coarse - fine).abs();
    assert!(
        drift < 2.0e-12,
        "third-stage delay drifted {:.2} ps between 5 ps and 50 ps steps \
         ({fine:.4e} vs {coarse:.4e}); the run is not integrating at second order",
        drift * 1e12
    );
    // And the value itself is the one ngspice reaches, not merely a
    // self-consistent wrong one.
    assert!(
        (fine - 3.7482e-9).abs() < 3.0e-12,
        "third-stage delay {fine:.4e} is not within 3 ps of ngspice's 3.7482 ns"
    );
}
