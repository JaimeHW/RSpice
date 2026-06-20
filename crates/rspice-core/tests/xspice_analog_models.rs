//! Native XSPICE analog code models pinned against ngspice code-model semantics.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

#[test]
fn climit_hard_limits_to_controlled_upper_bound_like_ngspice() {
    // ngspice climit:
    //   raw = gain * (in + in_offset)
    //   upper = V(cntl_upper) - upper_delta
    //   lower = V(cntl_lower) + lower_delta
    // With limit_range=0 this is a hard clamp.
    let deck = "\
* XSPICE climit hard upper rail
vin in 0 dc 2
vhi hi 0 dc 1.6
vlo lo 0 dc 0.2
aclip in hi lo out clim
.model clim climit (gain=2 in_offset=0 lower_delta=0.05 upper_delta=0.1 limit_range=0 fraction=0)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!((out - 1.5).abs() < 1e-9, "climit upper clamp: got {out}");
}

#[test]
fn climit_negative_limit_range_uses_ngspice_threshold_math() {
    // ngspice does not constrain limit_range to non-negative values. A negative
    // range moves the lower threshold below the controlled lower limit, so this
    // point remains in the linear region instead of clamping to 0 V.
    let deck = "\
* XSPICE climit negative limit_range
vin in 0 dc -0.1
vhi hi 0 dc 1
vlo lo 0 dc 0
aclip in hi lo out clim
.model clim climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=-0.2 fraction=0)
rl out 0 1k
.op
.end
";

    let out = op_voltage(deck, "out");
    assert!(
        (out + 0.1).abs() < 1e-9,
        "climit negative limit_range should stay linear like ngspice: got {out}"
    );
}

#[test]
fn climit_lower_linear_and_fraction_smoothing_match_ngspice() {
    let deck = "\
* XSPICE climit lower/pass/fraction oracle
vlowin in_low 0 dc -1
vmid in_mid 0 dc 0.4
vsmooth in_smooth 0 dc 0.9
vhi hi 0 dc 1
vlo lo 0 dc 0
alow in_low hi lo out_low hard
amid in_mid hi lo out_mid hard
asmooth in_smooth hi lo out_smooth frac
.model hard climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=0 fraction=0)
.model frac climit (gain=1 in_offset=0 lower_delta=0 upper_delta=0 limit_range=0.2 fraction=1)
rlow out_low 0 1k
rmid out_mid 0 1k
rsmooth out_smooth 0 1k
.op
.end
";

    let out_low = op_voltage(deck, "out_low");
    let out_mid = op_voltage(deck, "out_mid");
    let out_smooth = op_voltage(deck, "out_smooth");

    assert!(
        out_low.abs() < 1e-9,
        "climit lower hard clamp should match ngspice: got {out_low}"
    );
    assert!(
        (out_mid - 0.4).abs() < 1e-9,
        "climit linear region should match ngspice: got {out_mid}"
    );
    assert!(
        (out_smooth - 0.8875).abs() < 1e-9,
        "climit fraction smoothing should match ngspice: got {out_smooth}"
    );
}
