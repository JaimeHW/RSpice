use rspice_core::{Engine, Netlist};

#[test]
fn xyce_generic_switch_constant_control_builds_and_conducts() {
    let deck = r#"generic switch
v1 in 0 1
sw1 in out swmod control={1}
rload out 0 1k
.model swmod sw ron=10 roff=1e9 on=1 off=0
.op
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse generic switch deck");
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("generic switch deck should solve");
    let out_idx = op
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("out"))
        .expect("out node present");

    assert!(op.node_voltages[out_idx] > 0.98);
}
