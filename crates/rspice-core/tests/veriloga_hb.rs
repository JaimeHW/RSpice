#![cfg(feature = "veriloga")]

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn write_model(name: &str, source: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_hb_{}_{}.va", name, std::process::id()));
    std::fs::write(&path, source).expect("write temporary Verilog-A model");
    path.to_string_lossy().replace('\\', "/")
}

#[test]
fn veriloga_hb_runtime_errors_are_simulation_errors_not_panics() {
    let model = write_model(
        "runtime_oob",
        r#"
`include "disciplines.vams"

module va_hb_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let deck = format!(
        "* veriloga HB runtime diagnostic\n\
         V1 in 0 DC 1 SIN(1 0 1meg)\n\
         R1 in 0 1k\n\
         C1 in 0 1p\n\
         XBAD in 0 va_hb_oob\n\
         .va \"{model}\" va_hb_oob\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| {
        Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(3))
    });

    let _ = std::fs::remove_file(model);

    let result = result.expect("Verilog-A HB runtime errors must not panic");
    let err = result.expect_err("HB runtime error must be reported to the caller");
    let text = err.to_string();
    assert!(
        text.contains("Verilog-A") && (text.contains("Array index 5") || text.contains("[1:4]")),
        "diagnostic should identify the Verilog-A HB array bounds error, got: {text}"
    );
}
