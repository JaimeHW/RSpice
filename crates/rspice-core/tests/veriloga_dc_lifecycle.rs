//! End-to-end DC lifecycle pins for runtime-compiled Verilog-A devices.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn write_model(name: &str, source: &str) -> PathBuf {
    let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rspice_dc_lifecycle_{name}_{}_{sequence}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn node_voltage(result: &rspice_core::solver::SimulationResult, name: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} is absent from {:?}", result.node_names));
    result.node_voltages[index]
}

#[test]
fn dc_operating_point_exposes_initial_and_final_step_together() {
    let model = write_model(
        "single_point",
        r#"
`include "disciplines.vams"
module va_dc_single_point(p, n);
    inout p, n;
    electrical p, n;
    real level;
    analog begin
        level = 0.0;
        @(initial_step("dc")) level = level + 1.0;
        @(final_step("dc")) level = level + 2.0;
        V(p, n) <+ level;
    end
endmodule
"#,
    );
    let deck = format!(
        "* one-point DC lifecycle\n\
         X1 out 0 va_dc_single_point\n\
         .va \"{}\" va_dc_single_point\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let result = Engine::default().run_dc_op(&netlist).expect("DC OP runs");
    assert!((node_voltage(&result, "out") - 3.0).abs() < 1.0e-12);

    let _ = std::fs::remove_file(model);
}

#[test]
fn initial_and_final_step_state_is_committed_at_the_public_sweep_endpoints() {
    let model = write_model(
        "sweep_initial",
        r#"
`include "disciplines.vams"
module va_dc_sweep_initial(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("dc")) count = count + 1.0;
        @(final_step("dc")) count = count + 10.0;
        V(p, n) <+ count;
    end
endmodule
"#,
    );
    let deck = format!(
        "* persistent DC sweep lifecycle\n\
         VSW sense 0 0\n\
         X1 out 0 va_dc_sweep_initial\n\
         .va \"{}\" va_dc_sweep_initial\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 2.0, 1.0)
        .expect("DC source sweep runs");
    assert_eq!(points.len(), 4);
    for (point_index, (coordinate, result)) in points.into_iter().enumerate() {
        let expected = if point_index == 3 { 11.0 } else { 1.0 };
        assert!(
            (node_voltage(&result, "out") - expected).abs() < 1.0e-12,
            "unexpected lifecycle value at sweep coordinate {coordinate}: expected {expected}"
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn above_crossing_state_is_committed_between_source_sweep_points() {
    let model = write_model(
        "sweep_above",
        r#"
`include "disciplines.vams"
module va_dc_sweep_above(sense, out);
    input sense;
    output out;
    electrical sense, out;
    real latched;
    analog begin
        @(above(V(sense))) latched = 1.0;
        V(out) <+ latched;
    end
endmodule
"#,
    );
    let deck = format!(
        "* same-time DC above lifecycle\n\
         VSW sense 0 0\n\
         X1 sense out va_dc_sweep_above\n\
         .va \"{}\" va_dc_sweep_above\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 3.0, 2.0)
        .expect("DC source sweep runs");
    let observed = points
        .iter()
        .map(|(_, result)| node_voltage(result, "out"))
        .collect::<Vec<_>>();
    assert_eq!(observed.len(), 3);
    assert!(observed[0].abs() < 1.0e-12, "{observed:?}");
    assert!((observed[1] - 1.0).abs() < 1.0e-12, "{observed:?}");
    assert!((observed[2] - 1.0).abs() < 1.0e-12, "{observed:?}");

    let _ = std::fs::remove_file(model);
}
