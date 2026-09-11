#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ModelFile(std::path::PathBuf);
impl ModelFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_unified_audit_{}_{}.va",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).unwrap();
        Self(path)
    }
    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}
impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn initial_digital_read_uses_the_solved_time_zero_voltage() {
    let model = ModelFile::new(
        "module sample(p,q); input p; electrical p; output q; reg q; initial q=(V(p)>0.999); analog I(p)<+0; endmodule",
    );
    let deck = Netlist::parse(&format!(
        "* initial sample\nV1 p 0 1\nX1 p q sample\n.va \"{}\" sample\n.end",
        model.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("q"))
        .unwrap();
    assert!(
        result.voltages[q]
            .iter()
            .all(|voltage| (*voltage - 3.3).abs() < 1e-8),
        "{:?}",
        result.voltages[q]
    );
}

#[test]
fn scheduled_digital_read_uses_analog_value_at_its_own_time() {
    let model = ModelFile::new(
        r#"
module sample_at_ten(p,q);
    input p; electrical p;
    output q; reg q;
    initial begin q=0; #10 q=(V(p)>0.999); end
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* sample a known ramp at 10 ns\nV1 p 0 PWL(0 0 20n 2)\nX1 p q sample_at_ten\n.va \"{}\" sample_at_ten\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 12e-9, 0.2e-9).unwrap();
    let p = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("p"))
        .unwrap();
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    for (index, time) in result
        .time
        .iter()
        .enumerate()
        .filter(|(_, time)| **time >= 9.5e-9 && **time <= 10.5e-9)
    {
        println!(
            "scheduled sample: t={time:e}, p={}, q={}",
            result.voltages[p][index], result.voltages[q][index]
        );
    }
    assert!(
        *result.voltages[q].last().unwrap() > 3.0,
        "V(p,10 ns)=1 V, so the sampled comparison must be true; final q={}",
        result.voltages[q].last().unwrap()
    );
}

#[test]
fn digital_variable_drives_analog_equation() {
    let model = ModelFile::new(
        r#"
module digital_to_analog(out);
    output out; electrical out;
    reg state;
    initial begin state=0; #1 state=1; end
    analog V(out)<+state;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* shared variable\nX1 out digital_to_analog\n.va \"{}\" digital_to_analog\n.end\n",
        model.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 3e-9, 0.2e-9).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        (*result.voltages[out].last().unwrap() - 1.0).abs() < 1e-9,
        "digital state=1 must produce 1 V, got {}",
        result.voltages[out].last().unwrap()
    );
}

#[test]
fn digital_state_controls_analog_conductance_and_its_jacobian() {
    let model = ModelFile::new(
        "module switched(p); inout p; electrical p; reg state; initial begin state=0; #1 state=1; end analog if(state) I(p)<+V(p)/1000; endmodule",
    );
    let deck = Netlist::parse(&format!("* state-dependent conductance\nV1 vdd 0 1\nR1 vdd p 1000\nX1 p switched\n.va \"{}\" switched\n.end", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 3e-9, 0.2e-9).unwrap();
    let p = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("p"))
        .unwrap();
    for (time, voltage) in result.time.iter().zip(&result.voltages[p]) {
        let expected = if *time < 1e-9 - 1e-18 { 1.0 } else { 0.5 };
        assert!((voltage - expected).abs() < 1e-7, "t={time}, p={voltage}");
    }
}

#[test]
fn mixed_instance_parameters_are_executable() {
    let model = ModelFile::new(
        r#"
module param_device(p,q);
    inout p; electrical p;
    output q; reg q;
    parameter real resistance=1000 from (0:inf);
    parameter real ratio=resistance/1000;
    initial q=(ratio>2.5);
    analog I(p)<+V(p)/(resistance*($param_given(resistance)?1:2));
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* parameter support\n.param r=2000\nV1 vdd 0 1\nR1 vdd p1 1000\nR2 vdd p2 1000\nX1 p1 q1 param_device resistance={{r}}\nX2 p2 q2 param_device resistance=3000\n.va \"{}\" param_device\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    for (name, expected) in [("p1", 2.0 / 3.0), ("p2", 0.75), ("q1", 0.0), ("q2", 3.3)] {
        let index = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(
            result.voltages[index]
                .iter()
                .all(|value| (value - expected).abs() < 1e-8),
            "{name} must be {expected}, got {:?}",
            result.voltages[index]
        );
    }
}

#[test]
fn pure_analog_and_spice_devices_share_the_solution() {
    let model = ModelFile::new(
        r#"
module va_resistor(p,n);
    inout p,n; electrical p,n;
    analog I(p,n)<+V(p,n)/1000;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* same MNA control\nV1 in 0 2\nR1 in out 1000\nX1 out 0 va_resistor\n.va \"{}\" va_resistor\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(result.voltages[out].iter().all(|v| (v - 1.0).abs() < 1e-9));
}

#[test]
fn high_impedance_output_releases_the_analog_net() {
    let model = ModelFile::new(
        r#"
module released_output(p,q);
    input p; electrical p;
    output q; reg q;
    initial q=1'bz;
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* pull up a released output\nV1 p 0 3.3\nRpull p q 1000\nX1 p q released_output\n.va \"{}\" released_output\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    assert!(
        (*result.voltages[q].last().unwrap() - 3.3).abs() < 1e-6,
        "released output with 1 kohm pull-up must reach 3.3 V; final q={}",
        result.voltages[q].last().unwrap()
    );
}

#[test]
fn output_release_and_unknown_drive_are_distinct_across_timesteps() {
    let model = ModelFile::new(
        "module tristate(p,q); input p; electrical p; output q; reg q; initial begin q=0; #1 q=1'bz; #1 q=1'bx; #1 q=1; end analog I(p)<+0; endmodule",
    );
    let deck = Netlist::parse(&format!("* tri-state sequence\nV1 p 0 3.3\nRpull p q 1000\nX1 p q tristate\n.va \"{}\" tristate\n.end", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 4e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("q"))
        .unwrap();
    for (time, voltage) in result.time.iter().zip(&result.voltages[q]) {
        let expected = if *time < 1e-9 - 1e-18 {
            3.3 * 20.0 / 1020.0
        } else if *time < 2e-9 - 1e-18 {
            3.3
        } else if *time < 3e-9 - 1e-18 {
            (3.3 * 20.0 + 1.65 * 1000.0) / 1020.0
        } else {
            3.3
        };
        assert!(
            (voltage - expected).abs() < 1e-7,
            "t={time}, expected={expected}, q={voltage}"
        );
    }
}

#[test]
fn crossing_does_not_execute_an_unrelated_future_timer_early() {
    let model = ModelFile::new(
        r#"
module future_timer(p,clk,q,edge_seen,delayed);
    input p; electrical p;
    input clk; wire clk;
    output q; reg q;
    output edge_seen,delayed; reg edge_seen,delayed;
    reg seen;
    initial begin q=0; #4 q=1; end
    initial begin seen=0; edge_seen=0; delayed=0; end
    always @(posedge clk) seen<=1;
    always @(posedge seen) begin #0 edge_seen=1; #1 delayed=1; end
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* A/D crossing before independent timer\nV1 p 0 1\nVclk clk 0 PWL(0 0 3.5n 0 3.6n 3.3 6n 3.3)\nX1 p clk q edge delayed future_timer\n.va \"{}\" future_timer\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 5.5e-9, 0.05e-9).unwrap();
    for (name, earliest, latest) in [
        ("edge", 3.5e-9, 3.7e-9),
        ("q", 4e-9, 4e-9),
        ("delayed", 5e-9, 5e-9),
    ] {
        let node = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        let index = result.voltages[node]
            .iter()
            .position(|value| *value > 3.0)
            .unwrap();
        assert!(
            result.time[index] >= earliest - 1e-18 && result.time[index] <= latest + 1e-18,
            "{name} first rose at {}",
            result.time[index]
        );
    }
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    let early: Vec<_> = result
        .time
        .iter()
        .zip(&result.voltages[q])
        .filter(|(t, v)| **t < 4e-9 - 1e-18 && **v > 3.0)
        .collect();
    println!("timer output before 4 ns: {early:?}");
    assert!(
        early.is_empty(),
        "an unrelated #4 timer must not drive the circuit before 4 ns"
    );
}

#[test]
fn connect_rules_survive_a_verilog_include_wrapper() {
    let mut source = String::from(
        r#"
module rules_device(p,clk,q);
    input p; electrical p;
    input clk; wire clk;
    output q; wire q;
    assign q=clk;
    analog I(p)<+0;
endmodule
"#,
    );
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str("\nconnectrules supply_rules;\nconnect a2d #(.vsup(1.0));\nconnect d2a #(.vsup(1.0));\nendconnectrules\n");
    let direct = ModelFile::new(&source);
    let wrapper = ModelFile::new(&format!("`include \"{}\"\n", direct.path()));
    let run = |model: &ModelFile| {
        let deck = Netlist::parse(&format!("* include must preserve electrical conversion\nV1 p 0 1\nVclk clk 0 1\nX1 p clk q rules_device\n.va \"{}\" rules_device\n.end\n", model.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
        let q = result
            .node_names
            .iter()
            .position(|s| s.eq_ignore_ascii_case("q"))
            .unwrap();
        *result.voltages[q].last().unwrap()
    };
    let direct_q = run(&direct);
    let wrapped_q = run(&wrapper);
    println!("connect rules: direct q={direct_q}, wrapped q={wrapped_q}");
    assert!(
        (direct_q - 1.0).abs() < 1e-9,
        "direct rules must select 1 V conversion"
    );
    assert!(
        (wrapped_q - direct_q).abs() < 1e-9,
        "an include wrapper must not silently discard the selected connect rules"
    );
}
